//! C6 legacy control step: the remaining legacy `k.machine.*` rows factorial
//! and fold fire, as one per-rule relation over the full C6 carriers:
//! `return.unary`, `return.if.true`, `return.if.false`, `return.match`,
//! `return.arguments.final` (frame pops), `primitive.value`, `cases.try`,
//! `cases.miss` and `cases.hit` (control rewrites).
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner and writes the post control, control origin, env root,
//! frame stack and helper result exactly as `fragments/legacy.proposed.rs`
//! plans them. A pop decrements the depth and zeroes the popped slot.
//! `cases.try` reads the first arm's body from canonical PROG. Charge is
//! LegacyComplete (work 1, reference cursor 1). Every unnamed lane is
//! preserved; canonicality is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_FRAME_COUNT,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_RESULT_RECORD, RESULT_LANES};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_program_context_canonical::C6ProgramContextArmCaptureInputs;
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_legacy, check_shapes, compose_canonical, disabled_identity,
    legacy_control_lanes, pop_frame, prefix, preserve_image, select, set_control,
    validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6LegacyControlError {
    #[error("rule {0:?} is not a legacy control rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 legacy control emission: {0}")]
    Emit(String),
}

pub const LEGACY_CONTROL_RULES: [RuleIdC; 9] = [
    RuleIdC::KMachineReturnUnary,
    RuleIdC::KMachineReturnIfTrue,
    RuleIdC::KMachineReturnIfFalse,
    RuleIdC::KMachineReturnMatch,
    RuleIdC::KMachineReturnArgumentsFinal,
    RuleIdC::KMachinePrimitiveValue,
    RuleIdC::KMachineCasesTry,
    RuleIdC::KMachineCasesMiss,
    RuleIdC::KMachineCasesHit,
];

pub fn legacy_control_supported(rule: RuleIdC) -> bool {
    LEGACY_CONTROL_RULES.contains(&rule)
}

// Result tags (c6_phys_helper::encode_result_record).
const R_APPEND_DONE: u128 = 9;
const R_SCALAR_VALUE: u128 = 10;
const R_SCALAR_ERROR: u128 = 11;
const R_EQUAL_DONE: u128 = 14;
const R_MATCH_MATCHED: u128 = 15;
const R_MATCH_NO_MATCH: u128 = 16;

#[derive(Clone, Debug)]
pub struct C6LegacyControlBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6LegacyControlEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

/// Register writes besides control: (origin, env root) when the rule sets them.
struct Regs {
    origin: Option<Operand>,
    env: Option<(Operand, Operand)>,
    pop: bool,
    clears_result: bool,
}

impl C6LegacyControlEmitter {
    pub fn new(scope: &str) -> Result<Self, C6LegacyControlError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6LegacyControlError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6LegacyControlError::Emit)?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6LegacyControlBlock, C6LegacyControlError> {
        if !legacy_control_supported(rule) {
            return Err(C6LegacyControlError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6LegacyControlError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6LegacyControlError::Emit)?;
        Ok(C6LegacyControlBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        prefix(em, inputs);
        let top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;
        let k: Vec<Operand> = pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES].to_vec();
        let res: Vec<Operand> =
            inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES].to_vec();
        let mut ctl: Vec<Operand> = (0..CONTROL_LANES).map(|_| Em::imm(0)).collect();
        let tag = |em: &mut Em, v: &Operand, t: u128| em.geq(en.clone(), v.clone(), Em::imm(t));
        let one_of = |em: &mut Em, v: &Operand, tags: &[u128]| {
            let mut n = Em::imm(0);
            for t in tags {
                let h = em.teq(v.clone(), Em::imm(*t));
                n = em.add(n, h);
            }
            em.geq(en.clone(), n, Em::imm(1));
        };
        let regs = match rule {
            RuleIdC::KMachineReturnUnary => {
                // UnaryF { op, origin } -> PrimResult { prim result }.
                tag(em, &top[0], 3);
                one_of(em, &res[0], &[R_SCALAR_VALUE, R_EQUAL_DONE, R_SCALAR_ERROR]);
                let is_err = em.teq(res[0].clone(), Em::imm(R_SCALAR_ERROR));
                ctl[0] = Em::imm(8);
                ctl[1] = is_err.clone();
                ctl[2] = res[1].clone();
                ctl[3] = em.sel(is_err, Em::imm(0), res[2].clone());
                Regs {
                    origin: Some(top[2].clone()),
                    env: None,
                    pop: true,
                    clears_result: true,
                }
            }
            RuleIdC::KMachineReturnIfTrue | RuleIdC::KMachineReturnIfFalse => {
                // IfF { then, else, saved_env, origin } -> Eval { branch }.
                tag(em, &top[0], 4);
                let next = if rule == RuleIdC::KMachineReturnIfTrue {
                    top[1].clone()
                } else {
                    top[2].clone()
                };
                ctl[1] = next.clone();
                Regs {
                    origin: Some(next),
                    env: Some((top[3].clone(), top[4].clone())),
                    pop: true,
                    clears_result: true,
                }
            }
            RuleIdC::KMachineReturnMatch => {
                // MatchF { arms, saved_env, origin } + Return { value } ->
                // Cases { value, arms, saved_env }.
                tag(em, &top[0], 5);
                tag(em, &k[0], 1);
                ctl[0] = Em::imm(6);
                ctl[1] = k[1].clone();
                ctl[2] = k[2].clone();
                ctl[3..7].clone_from_slice(&top[1..5]);
                Regs {
                    origin: Some(top[5].clone()),
                    env: Some((top[3].clone(), top[4].clone())),
                    pop: true,
                    clears_result: false,
                }
            }
            RuleIdC::KMachineReturnArgumentsFinal => {
                // ArgsF { fname, .., saved_env, origin } + AppendDone { seq }
                // -> Invoke { fname, seq }.
                tag(em, &top[0], 6);
                tag(em, &res[0], R_APPEND_DONE);
                ctl[0] = Em::imm(3);
                ctl[1] = top[1].clone();
                ctl[2] = res[1].clone();
                ctl[3] = res[2].clone();
                Regs {
                    origin: Some(top[8].clone()),
                    env: Some((top[6].clone(), top[7].clone())),
                    pop: true,
                    clears_result: true,
                }
            }
            RuleIdC::KMachinePrimitiveValue => {
                // PrimResult { Value { value } } -> Return { value }.
                tag(em, &k[0], 8);
                tag(em, &k[1], 0);
                ctl[0] = Em::imm(1);
                ctl[1] = k[2].clone();
                ctl[2] = k[3].clone();
                Regs {
                    origin: None,
                    env: None,
                    pop: false,
                    clears_result: false,
                }
            }
            RuleIdC::KMachineCasesTry => {
                // Cases { value, arms, saved_env } + Match result ->
                // CaseResult { matching, value, arm(arms.start).body,
                // tail(arms), saved_env }.
                tag(em, &k[0], 6);
                one_of(em, &res[0], &[R_MATCH_MATCHED, R_MATCH_NO_MATCH]);
                let empty = em.teq(k[4].clone(), Em::imm(0));
                let nonempty = em.not(empty);
                em.gassert(en.clone(), nonempty);
                let arm = self
                    .composed
                    .progctx_em
                    .emit_prog_arm_capture(&C6ProgramContextArmCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps: inputs.geometry.caps,
                        arm_index: k[3].clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(arm.instructions);
                em.gassert(en.clone(), arm.arm_found.clone());
                let m = em.teq(res[0].clone(), Em::imm(R_MATCH_MATCHED));
                // rest.len + 1 = arms.len; rest.start = len ? start + 1 : 0.
                let rest_start = post_i[L_CONTROL + 7].clone();
                let rest_len = post_i[L_CONTROL + 8].clone();
                let len_plus = em.add(rest_len.clone(), Em::imm(1));
                em.geq(en.clone(), len_plus, k[4].clone());
                let rest_empty = em.teq(rest_len.clone(), Em::imm(0));
                let start_plus = em.add(k[3].clone(), Em::imm(1));
                let want_start = em.sel(rest_empty, Em::imm(0), start_plus);
                em.geq(en.clone(), rest_start.clone(), want_start);
                ctl[0] = Em::imm(7);
                ctl[1] = m.clone();
                ctl[2] = em.mul(m.clone(), res[1].clone());
                ctl[3] = em.mul(m, res[2].clone());
                ctl[4] = k[1].clone();
                ctl[5] = k[2].clone();
                ctl[6] = arm.arm_row[1].clone();
                ctl[7] = rest_start;
                ctl[8] = rest_len;
                ctl[9] = k[5].clone();
                ctl[10] = k[6].clone();
                Regs {
                    origin: None,
                    env: None,
                    pop: false,
                    clears_result: true,
                }
            }
            RuleIdC::KMachineCasesMiss => {
                // CaseResult { .., value, rest, saved_env } -> Cases { value,
                // rest, saved_env }.
                tag(em, &k[0], 7);
                ctl[0] = Em::imm(6);
                ctl[1] = k[4].clone();
                ctl[2] = k[5].clone();
                ctl[3..7].clone_from_slice(&k[7..11]);
                Regs {
                    origin: None,
                    env: None,
                    pop: false,
                    clears_result: false,
                }
            }
            RuleIdC::KMachineCasesHit => {
                // CaseResult { Matched { env }, body, saved_env } ->
                // ScheduleNext { ScopeF { saved_env, origin }, body, env }.
                tag(em, &k[0], 7);
                tag(em, &k[1], 1);
                ctl[0] = Em::imm(2);
                ctl[1] = Em::imm(8);
                ctl[2] = k[9].clone();
                ctl[3] = k[10].clone();
                ctl[4] = pre_i[L_CONTROL_ORIGIN].clone();
                ctl[14] = k[6].clone();
                ctl[15] = k[2].clone();
                ctl[16] = k[3].clone();
                Regs {
                    origin: None,
                    env: None,
                    pop: false,
                    clears_result: false,
                }
            }
            _ => unreachable!("checked by legacy_control_supported"),
        };
        set_control(em, inputs, &ctl);
        let mut mutable = legacy_control_lanes();
        if let Some(o) = &regs.origin {
            em.geq(en.clone(), post_i[L_CONTROL_ORIGIN].clone(), o.clone());
            mutable.push(L_CONTROL_ORIGIN);
        }
        if let Some((p, i)) = &regs.env {
            em.geq(en.clone(), post_i[L_ENV_ROOT].clone(), p.clone());
            em.geq(en.clone(), post_i[L_ENV_ROOT + 1].clone(), i.clone());
            mutable.extend([L_ENV_ROOT, L_ENV_ROOT + 1]);
        }
        if regs.pop {
            pop_frame(em, inputs);
            mutable.push(L_FRAME_COUNT);
        }
        preserve_image(em, inputs, &mutable, regs.pop);
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
            if regs.clears_result && in_result {
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

/// One-shot emission of the legacy control relation for `rule`.
pub fn emit_c6_legacy_control_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6LegacyControlBlock, C6LegacyControlError> {
    C6LegacyControlEmitter::new(scope)?.emit(rule, inputs)
}
