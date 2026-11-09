//! Per-rule C6 CallBind relations over the full public pre/post carriers.
//! The dispatch owner selects the rule; the legacy plan rewrites control and
//! clears the result for InvokeFind/EnterBind. BoundValid only schedules its
//! ReturnF: the frame stack, caller environment and control origin stay put.
//! LegacyComplete charges work and reference cursor by one. All other lanes
//! are preserved, and a disabled step is identity.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_ACTIVE, L_CONSUMED_WORK, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_RESULT_RECORD, RESULT_LANES};
use mori_core::sherlock::helper_machine::c6_phys_program::PROGRAM_HEADER_LANES;
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_program_context_canonical::C6ProgramContextFunctionCaptureInputs;
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_legacy, check_shapes, compose_canonical,
    legacy_control_lanes, prefix, select, validate_pre_image,
};

pub const CALL_BIND_RULES: [RuleIdC; 3] = [
    RuleIdC::KMachineInvokeFind,
    RuleIdC::KMachineEnterBind,
    RuleIdC::KMachineBoundValid,
];

pub fn call_bind_supported(rule: RuleIdC) -> bool {
    CALL_BIND_RULES.contains(&rule)
}

#[derive(Debug, Error)]
pub enum C6CallBindError {
    #[error("rule {0:?} is not a CallBind control rule")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 CallBind emission: {0}")]
    Emit(String),
}

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
        if inputs.pre_prog.len() <= PROGRAM_HEADER_LANES {
            return Err(C6CallBindError::Emit(
                "missing PROG expression count".into(),
            ));
        }
        let scope = self.scope.clone();
        let mut em = Em {
            scope: &scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let result = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        result.map_err(C6CallBindError::Emit)?;
        Ok(C6CallBindBlock {
            instructions: em.ins,
        })
    }

    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        let en = inputs.enable.clone();
        let caps = inputs.geometry.caps;
        let p = &caps.phys;
        prefix(em, inputs);
        let _top = validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;

        let pre = &inputs.pre_image;
        let k = &pre[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let res = &inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES];
        // Canonical PROG starts with four header lanes and expressions.count.
        let expressions = inputs.pre_prog[PROGRAM_HEADER_LANES].clone();
        // Dispatch fixes Running; its two unused outcome lanes must also be zero.
        em.gzero(en.clone(), pre[L_OUTCOME + 1].clone());
        em.gzero(en.clone(), pre[L_OUTCOME + 2].clone());

        let mut control: Vec<Operand> = vec![Em::imm(0); CONTROL_LANES];
        let used = match rule {
            RuleIdC::KMachineInvokeFind => {
                // Invoke { name, args } + FindFound/FindMissing -> Enter.
                // Dispatch fixes the control tag and result tag (5 or 6).
                em.gated_lt_index(en.clone(), k[1].clone(), p.max_names);
                em.gated_validate_slice(en.clone(), k, 2, p.max_value_slots, Some(p.max_arity));
                // validate_plan checks the written args against active slots,
                // and requires a zero start for an empty slice.
                let empty = em.teq(k[3].clone(), Em::imm(0));
                let empty_gate = em.and(en.clone(), empty);
                em.gzero(empty_gate, k[2].clone());
                let end = em.add(k[2].clone(), k[3].clone());
                let active_end = em.add(pre[L_ACTIVE + 3].clone(), Em::imm(1));
                let fits = em.less_than_129(end, active_end);
                em.gassert(en.clone(), fits);

                let found = em.teq(res[0].clone(), Em::imm(5));
                let found_gate = em.and(en.clone(), found.clone());
                // SC-05 checks a written FunctionRowId against the actual
                // function table, not just the configured physical capacity.
                let f = self
                    .composed
                    .progctx_em
                    .emit_prog_function_capture(&C6ProgramContextFunctionCaptureInputs {
                        gate: found_gate.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        function_index: res[1].clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(f.instructions);
                em.gassert(found_gate, f.function_found);
                control[0] = Em::imm(4);
                control[1] = found;
                // Result canonicality makes this zero for FindMissing.
                control[2] = res[1].clone();
                control[3] = k[2].clone();
                control[4] = k[3].clone();
                4
            }
            RuleIdC::KMachineEnterBind => {
                // Enter { Some(function), args } + BindOk/BindBad -> Bound.
                em.geq(en.clone(), k[1].clone(), Em::imm(1));
                em.gated_validate_slice(en.clone(), k, 3, p.max_value_slots, Some(p.max_arity));
                let f = self
                    .composed
                    .progctx_em
                    .emit_prog_function_capture(&C6ProgramContextFunctionCaptureInputs {
                        gate: en.clone(),
                        pre_prog: inputs.pre_prog.clone(),
                        caps,
                        function_index: k[2].clone(),
                    })
                    .map_err(|e| e.to_string())?;
                em.ins.extend(f.instructions);
                em.gassert(en.clone(), f.function_found);
                em.gated_lt_index(en.clone(), f.function_row[3].clone(), p.max_types.max(4));
                let body_live = em.lt128(f.function_row[4].clone(), expressions);
                em.gassert(en.clone(), body_live);
                // SC-07 requires a copied environment to be below active.env.
                // BindBad and BindOk(None) have zero env lanes canonically.
                let env_gate = em.and(en.clone(), res[1].clone());
                let env_live = em.lt128(res[2].clone(), pre[L_ACTIVE + 2].clone());
                em.gassert(env_gate, env_live);
                control[0] = Em::imm(5);
                control[1] = em.teq(res[0].clone(), Em::imm(7));
                control[2] = res[1].clone();
                control[3] = res[2].clone();
                control[4] = f.function_row[3].clone();
                control[5] = f.function_row[4].clone();
                5
            }
            RuleIdC::KMachineBoundValid => {
                // Bound { Ok(env), type, body } -> ScheduleNext(ReturnF).
                // Dispatch fixes Bound and the positive binding tag.
                em.gated_validate_env(en.clone(), k[2].clone(), k[3].clone(), p.max_env_cells);
                em.gated_lt_index(en.clone(), k[4].clone(), p.max_types.max(4));
                let body_live = em.lt128(k[5].clone(), expressions.clone());
                em.gassert(en.clone(), body_live);
                let origin_live = em.lt128(pre[L_CONTROL_ORIGIN].clone(), expressions);
                em.gassert(en.clone(), origin_live);
                for (present, index) in [
                    (k[2].clone(), k[3].clone()),
                    (pre[L_ENV_ROOT].clone(), pre[L_ENV_ROOT + 1].clone()),
                ] {
                    let env_gate = em.and(en.clone(), present);
                    let env_live = em.lt128(index, pre[L_ACTIVE + 2].clone());
                    em.gassert(env_gate, env_live);
                }
                control[0] = Em::imm(2);
                control[1] = Em::imm(7); // ReturnF
                control[2] = k[4].clone();
                control[3] = pre[L_ENV_ROOT].clone();
                control[4] = pre[L_ENV_ROOT + 1].clone();
                control[5] = pre[L_CONTROL_ORIGIN].clone();
                control[14] = k[5].clone();
                control[15] = k[2].clone();
                control[16] = k[3].clone();
                6
            }
            _ => unreachable!("checked by call_bind_supported"),
        };
        // The IMAGE control union has no separate composed canonicalizer.
        for lane in &k[used..] {
            em.gzero(en.clone(), lane.clone());
        }

        // Select identity directly for mutable payloads. Fixed lanes use one
        // unconditional equality, valid for both enabled and disabled steps.
        for (offset, want) in control.into_iter().enumerate() {
            let expected = em.sel(en.clone(), want, k[offset].clone());
            em.ceq(inputs.post_image[L_CONTROL + offset].clone(), expected);
        }
        charge_legacy(em, inputs);
        let disabled = em.not(en.clone());
        for lane in [L_REMAINING_WORK, L_CONSUMED_WORK, L_REFERENCE_CURSOR] {
            em.geq(
                disabled.clone(),
                inputs.post_image[lane].clone(),
                pre[lane].clone(),
            );
        }
        let mutable = legacy_control_lanes();
        for (lane, (a, b)) in pre.iter().zip(&inputs.post_image).enumerate() {
            if !mutable.contains(&lane) {
                em.ceq(b.clone(), a.clone());
            }
        }
        for (a, b) in inputs.pre_data.iter().zip(&inputs.post_data) {
            em.ceq(b.clone(), a.clone());
        }
        let clears_result = rule != RuleIdC::KMachineBoundValid;
        for (lane, (a, b)) in inputs
            .pre_helper
            .iter()
            .zip(&inputs.post_helper)
            .enumerate()
        {
            let in_result = (L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES).contains(&lane);
            if clears_result && in_result {
                let expected = em.sel(en.clone(), Em::imm(0), a.clone());
                em.ceq(b.clone(), expected);
            } else {
                em.ceq(b.clone(), a.clone());
            }
        }
        compose_canonical(&mut self.composed, em, inputs)?;
        Ok(())
    }
}
