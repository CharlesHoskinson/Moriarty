//! C6 CallBind control step: the legacy `k.machine.invoke.find`,
//! `k.machine.enter.bind` and `k.machine.bound.valid` rows as one per-rule
//! relation over the full C6 carriers (the rows factorial and fold fire).
//!
//! Each circuit fixes its rule at build time, selects it with the composed
//! dispatch owner, writes the post control exactly as
//! `fragments/legacy.proposed.rs` plans it, clears the helper result where
//! the plan does (InvokeFind, EnterBind), charges LegacyComplete and preserves
//! every other image, frame, DATA and helper lane. EnterBind reads the entered
//! function's row (return type, body) from canonical PROG. Canonicality of
//! META/PROG/CTX/DATA/HELPER is composed; enable = 0 is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_RESULT_RECORD, RESULT_LANES};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_program_context_canonical::C6ProgramContextFunctionCaptureInputs;
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_legacy, check_shapes, compose_canonical, disabled_identity,
    legacy_control_lanes, prefix, preserve_image, select, set_control, validate_pre_image,
};

#[derive(Debug, Error)]
pub enum C6CallBindError {
    #[error("rule {0:?} is not a CallBind control rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 CallBind emission: {0}")]
    Emit(String),
}

pub const CALL_BIND_RULES: [RuleIdC; 3] = [
    RuleIdC::KMachineInvokeFind,
    RuleIdC::KMachineEnterBind,
    RuleIdC::KMachineBoundValid,
];

pub fn call_bind_supported(rule: RuleIdC) -> bool {
    CALL_BIND_RULES.contains(&rule)
}

// Helper result tags (c6_phys_helper::encode_result_record).
const R_FIND_FOUND: u128 = 5;
const R_BIND_OK: u128 = 7;

#[derive(Clone, Debug)]
pub struct C6CallBindBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6CallBindEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

impl C6CallBindEmitter {
    pub fn new(scope: &str) -> Result<Self, C6CallBindError> {
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6CallBindError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6CallBindError::Emit)?,
        })
    }

    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6CallBindBlock, C6CallBindError> {
        if !call_bind_supported(rule) {
            return Err(C6CallBindError::Unsupported(rule));
        }
        check_shapes(inputs).map_err(C6CallBindError::Emit)?;
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let r = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        r.map_err(C6CallBindError::Emit)?;
        Ok(C6CallBindBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre_i = &inputs.pre_image;
        let k = &pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let res = &inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES];
        let mut ctl: Vec<Operand> = (0..CONTROL_LANES).map(|_| Em::imm(0)).collect();
        let clears_result = match rule {
            RuleIdC::KMachineInvokeFind => {
                // Invoke { fname, args } + Find result -> Enter { function?, args }.
                em.geq(en.clone(), k[0].clone(), Em::imm(3));
                let found = em.teq(res[0].clone(), Em::imm(R_FIND_FOUND));
                ctl[0] = Em::imm(4);
                ctl[1] = found.clone();
                ctl[2] = em.mul(found, res[1].clone());
                ctl[3] = k[2].clone();
                ctl[4] = k[3].clone();
                true
            }
            RuleIdC::KMachineEnterBind => {
                // Enter { Some(f), args } + Bind result -> Bound { binding,
                // f.return_type, f.body }.
                em.geq(en.clone(), k[0].clone(), Em::imm(4));
                em.geq(en.clone(), k[1].clone(), Em::imm(1));
                let f = self
                    .composed
                    .progctx_em
                    .emit_prog_function_capture(&C6ProgramContextFunctionCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps: inputs.geometry.caps,
                        function_index: k[2].clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(f.instructions);
                em.gassert(en.clone(), f.function_found.clone());
                let ok = em.teq(res[0].clone(), Em::imm(R_BIND_OK));
                ctl[0] = Em::imm(5);
                ctl[1] = ok.clone();
                ctl[2] = em.mul(ok.clone(), res[1].clone());
                ctl[3] = em.mul(ok, res[2].clone());
                ctl[4] = f.function_row[3].clone();
                ctl[5] = f.function_row[4].clone();
                true
            }
            RuleIdC::KMachineBoundValid => {
                // Bound { Ok { env }, return_type, body } -> ScheduleNext {
                // ReturnF { return_type, saved_env: env_root, origin:
                // control_origin }, next: body, env }.
                em.geq(en.clone(), k[0].clone(), Em::imm(5));
                em.geq(en.clone(), k[1].clone(), Em::imm(1));
                ctl[0] = Em::imm(2);
                ctl[1] = Em::imm(7);
                ctl[2] = k[4].clone();
                ctl[3] = pre_i[L_ENV_ROOT].clone();
                ctl[4] = pre_i[L_ENV_ROOT + 1].clone();
                ctl[5] = pre_i[L_CONTROL_ORIGIN].clone();
                ctl[14] = k[5].clone();
                ctl[15] = k[2].clone();
                ctl[16] = k[3].clone();
                false
            }
            _ => unreachable!("checked by call_bind_supported"),
        };
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

/// One-shot emission of the CallBind control relation for `rule`.
pub fn emit_c6_call_bind_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6CallBindBlock, C6CallBindError> {
    C6CallBindEmitter::new(scope)?.emit(rule, inputs)
}
