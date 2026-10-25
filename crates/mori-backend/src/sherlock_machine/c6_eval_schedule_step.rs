//! C6 Eval schedule step: the legacy `k.machine.eval.*` rows that factorial and
//! tree fold exercise, as one per-rule relation over the full C6 carriers.
//!
//! Each circuit fixes its rule at build time and:
//! - selects it with the composed dispatch owner (`c6_dispatch_step`), so the
//!   pre state must make Core select exactly this rule;
//! - reads the Eval expression row (and, for CallArguments, the first
//!   argument child) from canonical PROG;
//! - writes the post control exactly as `fragments/legacy.proposed.rs` plans it
//!   (Return, or ScheduleNext with the rule's frame, next and env root);
//! - clears the helper result where the legacy plan does (Literal and
//!   Variable rules) and preserves every other helper lane;
//! - charges LegacyComplete (work 1, reference cursor 1) and preserves all
//!   other image, frame and DATA lanes;
//! - composes META/PROG/CTX/DATA/HELPER canonicality; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{CONTROL_LANES, L_CONTROL, L_ENV_ROOT};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_RESULT_RECORD, RESULT_LANES};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_program_context_canonical::C6ProgramContextDispatchCaptureInputs;
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_legacy, check_shapes, compose_canonical, disabled_identity,
    legacy_control_lanes, prefix, preserve_image, select, set_control, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6EvalScheduleError {
    #[error("rule {0:?} is not an Eval schedule rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 Eval schedule emission: {0}")]
    Emit(String),
}

/// The Eval rules this fragment constrains (those factorial and fold fire).
pub const EVAL_SCHEDULE_RULES: [RuleIdC; 7] = [
    RuleIdC::KMachineEvalLiteralValid,
    RuleIdC::KMachineEvalVariablePresent,
    RuleIdC::KMachineEvalBinary,
    RuleIdC::KMachineEvalUnary,
    RuleIdC::KMachineEvalIf,
    RuleIdC::KMachineEvalMatch,
    RuleIdC::KMachineEvalCallArguments,
];

pub fn eval_schedule_supported(rule: RuleIdC) -> bool {
    EVAL_SCHEDULE_RULES.contains(&rule)
}

/// PROG expression row tag the rule's Eval site requires.
fn row_tag(rule: RuleIdC) -> u128 {
    match rule {
        RuleIdC::KMachineEvalLiteralValid => 0,
        RuleIdC::KMachineEvalVariablePresent => 1,
        RuleIdC::KMachineEvalBinary => 3,
        RuleIdC::KMachineEvalUnary => 4,
        RuleIdC::KMachineEvalIf => 5,
        RuleIdC::KMachineEvalCallArguments => 6,
        RuleIdC::KMachineEvalMatch => 7,
        _ => unreachable!("checked by eval_schedule_supported"),
    }
}

#[derive(Clone, Debug)]
pub struct C6EvalScheduleBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6EvalScheduleEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

impl C6EvalScheduleEmitter {
    pub fn new(scope: &str) -> Result<Self, C6EvalScheduleError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6EvalScheduleError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6EvalScheduleError::Emit)?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6EvalScheduleBlock, C6EvalScheduleError> {
        if !eval_schedule_supported(rule) {
            return Err(C6EvalScheduleError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6EvalScheduleError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6EvalScheduleError::Emit)?;
        Ok(C6EvalScheduleBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;
        let k = &pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES];
        // Eval { expr }: tag 0, expr at lane 1; the expression is the origin.
        em.geq(en.clone(), k[0].clone(), Em::imm(0));
        let origin = k[1].clone();
        let cap = self
            .composed
            .progctx_em
            .emit_prog_dispatch_capture(&C6ProgramContextDispatchCaptureInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps,
                first_index: origin.clone(),
                second_index: origin.clone(),
                child_index: Em::imm(0),
            })
            .map_err(|e| e.to_string())?;
        em.ins.extend(cap.instructions);
        em.gassert(en.clone(), cap.first_found.clone());
        let row = cap.first_row;
        em.geq(en.clone(), row[0].clone(), Em::imm(row_tag(rule)));

        let env_p = pre_i[L_ENV_ROOT].clone();
        let env_i = pre_i[L_ENV_ROOT + 1].clone();
        let res = &inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES];
        let z = || Em::imm(0);
        let mut ctl: Vec<Operand> = (0..CONTROL_LANES).map(|_| z()).collect();
        // ScheduleNext: tag 2, frame lanes 1..14, next 14, env 15..17.
        let schedule = |frame: &[Operand], next: Operand, ctl: &mut Vec<Operand>| {
            ctl[0] = Em::imm(2);
            for (i, f) in frame.iter().enumerate() {
                ctl[1 + i] = f.clone();
            }
            ctl[14] = next;
            ctl[15] = env_p.clone();
            ctl[16] = env_i.clone();
        };
        let mut clears_result = false;
        match rule {
            RuleIdC::KMachineEvalLiteralValid => {
                // Return { Const(constant) }.
                ctl[0] = Em::imm(1);
                ctl[2] = row[1].clone();
                clears_result = true;
            }
            RuleIdC::KMachineEvalVariablePresent => {
                // Return { value } from the EnvFound result (tag 2, ref at 1..3).
                em.geq(en.clone(), res[0].clone(), Em::imm(2));
                ctl[0] = Em::imm(1);
                ctl[1] = res[1].clone();
                ctl[2] = res[2].clone();
                clears_result = true;
            }
            RuleIdC::KMachineEvalBinary => {
                // LeftF { op, right, saved_env, origin }, next = left.
                let f = [
                    Em::imm(1),
                    row[1].clone(),
                    row[3].clone(),
                    env_p.clone(),
                    env_i.clone(),
                    origin.clone(),
                ];
                schedule(&f, row[2].clone(), &mut ctl);
            }
            RuleIdC::KMachineEvalUnary => {
                // UnaryF { op, origin }, next = operand.
                let f = [Em::imm(3), row[1].clone(), origin.clone()];
                schedule(&f, row[2].clone(), &mut ctl);
            }
            RuleIdC::KMachineEvalIf => {
                // IfF { then, else, saved_env, origin }, next = cond.
                let f = [
                    Em::imm(4),
                    row[2].clone(),
                    row[3].clone(),
                    env_p.clone(),
                    env_i.clone(),
                    origin.clone(),
                ];
                schedule(&f, row[1].clone(), &mut ctl);
            }
            RuleIdC::KMachineEvalMatch => {
                // MatchF { arms, saved_env, origin }, next = scrutinee.
                let f = [
                    Em::imm(5),
                    row[2].clone(),
                    row[3].clone(),
                    env_p.clone(),
                    env_i.clone(),
                    origin.clone(),
                ];
                schedule(&f, row[1].clone(), &mut ctl);
            }
            RuleIdC::KMachineEvalCallArguments => {
                // ArgsF { fname, completed: empty, rest: tail(args), saved_env,
                // origin }, next = child(args.start); args.len > 0.
                let empty = em.teq(row[3].clone(), Em::imm(0));
                let nonempty = em.not(empty);
                em.gassert(en.clone(), nonempty);
                // Route the PROG-capture output through a kit temp: the
                // capture refuses operands in its own reserved namespace.
                let child_index = em.add(row[2].clone(), Em::imm(0));
                let child = self
                    .composed
                    .progctx_em
                    .emit_prog_dispatch_capture(&C6ProgramContextDispatchCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        first_index: origin.clone(),
                        second_index: origin.clone(),
                        child_index,
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(child.instructions);
                em.gassert(en.clone(), child.child_found.clone());
                // rest.len + 1 = args.len; rest.start = len ? start + 1 : 0.
                let rest_start = post_i[L_CONTROL + 5].clone();
                let rest_len = post_i[L_CONTROL + 6].clone();
                let len_plus = em.add(rest_len.clone(), Em::imm(1));
                em.geq(en.clone(), len_plus, row[3].clone());
                let rest_empty = em.teq(rest_len.clone(), Em::imm(0));
                let start_plus = em.add(row[2].clone(), Em::imm(1));
                let want_start = em.sel(rest_empty, Em::imm(0), start_plus);
                em.geq(en.clone(), rest_start.clone(), want_start);
                let f = [
                    Em::imm(6),
                    row[1].clone(),
                    z(),
                    z(),
                    rest_start,
                    rest_len,
                    env_p.clone(),
                    env_i.clone(),
                    origin.clone(),
                ];
                schedule(&f, child.child_value.clone(), &mut ctl);
            }
            _ => unreachable!("checked by eval_schedule_supported"),
        }
        set_control(em, inputs, &ctl);
        preserve_image(em, inputs, &legacy_control_lanes(), false);
        charge_legacy(em, inputs);
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.geq(en.clone(), b.clone(), a.clone());
        }
        for (lane, (a, b)) in inputs
            .pre_helper
            .iter()
            .zip(inputs.post_helper.iter())
            .enumerate()
        {
            let in_result = (L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES).contains(&lane);
            if clears_result && in_result {
                em.gzero(en.clone(), b.clone());
            } else {
                em.geq(en.clone(), b.clone(), a.clone());
            }
        }
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }
}

/// One-shot emission of the Eval schedule relation for `rule`.
pub fn emit_c6_eval_schedule_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6EvalScheduleBlock, C6EvalScheduleError> {
    C6EvalScheduleEmitter::new(scope)?.emit(rule, inputs)
}
