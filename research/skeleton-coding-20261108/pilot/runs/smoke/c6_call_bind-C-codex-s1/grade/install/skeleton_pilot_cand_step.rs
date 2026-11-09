//! C6 CallBind control step: the legacy `k.machine.invoke.find`,
//! `k.machine.enter.bind` and `k.machine.bound.valid` rows as one per-rule
//! relation over the full C6 carriers (`C6StepInputs`, all lanes public).
//!
//! Spec: `owner_runtime.rs:618-640` (enabledness), `legacy.proposed.rs:289-325`
//! (plans), `owner_runtime.rs:1373-1377` (LegacyComplete charge),
//! `c6_phys.rs:3351` (BoundValid ReturnF frame encoding).
//!
//! Lane map (`c6_phys`, `c6_phys_helper`): `k = pre_image[L_CONTROL..+CONTROL_LANES]`,
//! `res = pre_helper[L_RESULT_RECORD..+RESULT_LANES]`; tags: control Invoke=3,
//! Enter=4, Bound=5, ScheduleNext=2; frame ReturnF=7; result FindFound=5, BindOk=7.
//!
//! Global invariants:
//! 1. Every post lane is either written by exactly one function below or
//!    preserved: post control <- `*_control` + `write_control`; work/consumed/
//!    cursor <- `charge`; all other image header and frame lanes, all DATA lanes,
//!    and every HELPER lane outside the result record <- preserved; helper
//!    result record <- cleared (InvokeFind, EnterBind) or preserved (BoundValid);
//!    post META/PROG/CTX <- unconditional equality in `prefix`.
//! 2. enable = 1 accepts only the owner step: selection fixes the rule's guard,
//!    each rule constraint uses only pre lanes and canonical PROG, so the post
//!    state is a function of the pre state.
//! 3. enable = 0 accepts exactly the identity (post = pre on IMAGE/DATA/HELPER
//!    via `disabled`, META/PROG/CTX via `prefix`); every enabled gate is
//!    multiplied by `enable`, so it is vacuous when disabled.
//! 4. Every constraint that reads a pre lane as an encoding (tags, payloads,
//!    result records, PROG rows) is backed by composed canonicality of
//!    META/PROG/CTX/DATA/HELPER (pre and post DATA/HELPER).
//! 5. All lanes are range-checked bit strings; `enable` is boolean.
//! 6. The rule is fixed at build time; no lane selects among the three rules.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_ACTIVE, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_OUTCOME,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_RESULT_RECORD, RESULT_LANES};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_program_context_canonical::{
    C6ProgramContextDispatchCaptureInputs, C6ProgramContextFunctionCaptureInputs,
};
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

/// Ensures: true exactly for the members of `CALL_BIND_RULES`.
/// Check: `owner_runtime.rs:1708-1710` (family rows Invoke/Enter/Bound).
pub fn call_bind_supported(rule: RuleIdC) -> bool {
    CALL_BIND_RULES.contains(&rule)
}

#[derive(Clone, Debug)]
pub struct C6CallBindBlock {
    pub instructions: Vec<Instruction>,
}

/// One planned post control payload (`CONTROL_LANES` operands) and whether the
/// plan clears the helper result record.
struct ControlPlan {
    ctl: Vec<Operand>,
    clears_result: bool,
}

pub struct C6CallBindEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

impl C6CallBindEmitter {
    /// Requires: nothing.
    /// Ensures: Err(InvalidScope) for an empty/blank scope or one containing
    /// `%` or `.c6kit.`; otherwise fresh temp counter and composed emitters.
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

    /// Requires: `inputs` allocated by `c6_step_harness::build_step_circuit`.
    /// Ensures: Err(Unsupported) unless `call_bind_supported(rule)`; shapes
    /// checked; returned instructions encode `body`; temp counter advanced so
    /// repeated emissions do not collide.
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
        let result = self.body(&mut em, rule, inputs);
        self.next_temp = em.next;
        result.map_err(C6CallBindError::Emit)?;
        Ok(C6CallBindBlock {
            instructions: em.ins,
        })
    }

    /// Orchestration: prefix, pre-image validation, `select`, the rule's
    /// control plan, written-ID validation, `write_control`, `charge`, `preserve_data`,
    /// `write_helper`, `canonical`, `disabled`. Owns no constraint itself.
    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        Self::prefix_and_validate(em, inputs);
        self.select(em, rule, inputs)?;
        let plan = match rule {
            RuleIdC::KMachineInvokeFind => Self::invoke_find_control(em, inputs),
            RuleIdC::KMachineEnterBind => self.enter_bind_control(em, inputs)?,
            RuleIdC::KMachineBoundValid => Self::bound_valid_control(em, inputs),
            _ => unreachable!("checked by call_bind_supported"),
        };
        self.validate_written_ids(em, rule, inputs, &plan)?;
        Self::write_control(em, inputs, &plan);
        Self::charge(em, inputs);
        Self::preserve_data(em, inputs);
        Self::write_helper(em, inputs, plan.clears_result);
        self.canonical(em, inputs)?;
        Self::disabled(em, inputs);
        Ok(())
    }

    /// Reads: enable, host_schema, carrier_version, all pre/post lanes; pre
    /// image header and frames.
    /// Ensures: enable boolean; schema/carrier fixed; post META/PROG/CTX = pre
    /// unconditionally; every lane is range-checked; under enable pre META/CTX/
    /// IMAGE magic+version and every active pre frame is valid.
    /// Check: `owner_runtime.rs` `step` (P0 publication preconditions).
    fn prefix_and_validate(em: &mut Em, inputs: &C6StepInputs) {
        prefix(em, inputs);
        validate_pre_image(em, inputs);
        // Dispatch fixes Running's tag. Its two inactive payload lanes must
        // also encode Running, rather than arbitrary preserved field values.
        for lane in L_OUTCOME + 1..L_OUTCOME + 3 {
            em.gzero(inputs.enable.clone(), inputs.pre_image[lane].clone());
        }
    }

    /// Reads: enable, pre PROG/IMAGE/DATA/HELPER.
    /// Ensures: under enable, the composed dispatch owner selects exactly
    /// `rule` from the pre state (P0/P1/P2 not taken, fuel available,
    /// `owner_select` picks `rule`, incl. its guard: InvokeFind needs result
    /// FindFound|FindMissing, EnterBind BindOk|BindBad, BoundValid control
    /// Bound{Ok}).
    /// Check: `owner_runtime.rs:618-640`, `owner_runtime.rs:1708-1710`.
    fn select(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        select(&mut self.composed, em, rule, inputs)
    }

    /// Reads: enable, pre control `k[0], k[2], k[3]`, pre result `res[0], res[1]`.
    /// Ensures: under enable `k[0]` = Invoke; plan = Enter { found =
    /// [res[0] = FindFound], function = found * res[1], args = (k[2], k[3]) },
    /// remaining control lanes 0; clears_result = true.
    /// Check: `legacy.proposed.rs:289-299`.
    fn invoke_find_control(em: &mut Em, inputs: &C6StepInputs) -> ControlPlan {
        let en = inputs.enable.clone();
        let k = &inputs.pre_image[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let res = &inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES];
        let caps = &inputs.geometry.caps.phys;
        em.geq(en.clone(), k[0].clone(), Em::imm(3));
        em.gated_lt_index(en.clone(), k[1].clone(), caps.max_names);
        em.gated_validate_slice(en.clone(), k, 2, caps.max_value_slots, Some(caps.max_arity));
        Self::control_padding(em, inputs, 4);

        // The output Enter.args is a written slice: owner_validate_plan
        // requires normalization and membership in the active slot prefix.
        let empty = em.teq(k[3].clone(), Em::imm(0));
        let empty_gate = em.and(en.clone(), empty);
        em.gzero(empty_gate, k[2].clone());
        let end = em.add(k[2].clone(), k[3].clone());
        let active_plus_one = em.add(inputs.pre_image[L_ACTIVE + 3].clone(), Em::imm(1));
        let fits = em.less_than_129(end, active_plus_one);
        em.gassert(en, fits);

        let found = em.teq(res[0].clone(), Em::imm(5));
        let mut ctl = vec![Em::imm(0); CONTROL_LANES];
        ctl[0] = Em::imm(4);
        ctl[1] = found.clone();
        ctl[2] = em.mul(found, res[1].clone());
        ctl[3] = k[2].clone();
        ctl[4] = k[3].clone();
        ControlPlan {
            ctl,
            clears_result: true,
        }
    }

    /// Reads: enable, pre control `k[0..3]`, pre result `res[0..3]`, pre PROG
    /// (function row capture at index `k[2]`).
    /// Ensures: under enable `k[0]` = Enter, `k[1]` = 1 (Some); the function
    /// row at `k[2]` exists in canonical PROG; plan = Bound { ok = [res[0] =
    /// BindOk], env = ok * (res[1], res[2]), return_type = row[3], body =
    /// row[4] }, remaining lanes 0; clears_result = true.
    /// Check: `legacy.proposed.rs:301-313` (`pre.function_row`).
    fn enter_bind_control(
        &mut self,
        em: &mut Em,
        inputs: &C6StepInputs,
    ) -> Result<ControlPlan, String> {
        let en = inputs.enable.clone();
        let k = &inputs.pre_image[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let res = &inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES];
        let caps = &inputs.geometry.caps.phys;
        em.geq(en.clone(), k[0].clone(), Em::imm(4));
        em.geq(en.clone(), k[1].clone(), Em::imm(1));
        em.gated_validate_slice(en.clone(), k, 3, caps.max_value_slots, Some(caps.max_arity));
        Self::control_padding(em, inputs, 5);
        let function = self
            .composed
            .progctx_em
            .emit_prog_function_capture(&C6ProgramContextFunctionCaptureInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps: inputs.geometry.caps,
                function_index: k[2].clone(),
            })
            .map_err(|e| e.to_string())?;
        em.ins.extend(function.instructions);
        em.gassert(en.clone(), function.function_found);
        // The PROG parser checks row structure; Bound's physical TypeId
        // field also uses the builtin-inclusive representation bound.
        em.gated_lt_index(en, function.function_row[3].clone(), caps.max_types.max(4));
        let ok = em.teq(res[0].clone(), Em::imm(7));
        let mut ctl = vec![Em::imm(0); CONTROL_LANES];
        ctl[0] = Em::imm(5);
        ctl[1] = ok.clone();
        ctl[2] = em.mul(ok.clone(), res[1].clone());
        ctl[3] = em.mul(ok, res[2].clone());
        ctl[4] = function.function_row[3].clone();
        ctl[5] = function.function_row[4].clone();
        Ok(ControlPlan {
            ctl,
            clears_result: true,
        })
    }

    /// Reads: enable, pre control `k[0..6]`, pre image `L_ENV_ROOT..+2`,
    /// `L_CONTROL_ORIGIN`.
    /// Ensures: under enable `k[0]` = Bound, `k[1]` = 1 (Ok); plan =
    /// ScheduleNext { frame ReturnF { return_type k[4], saved_env pre env_root,
    /// origin pre control_origin } at ctl[1..6], next = body k[5] at ctl[14],
    /// env = (k[2], k[3]) at ctl[15..17] }, remaining lanes 0;
    /// clears_result = false (result untouched).
    /// Check: `legacy.proposed.rs:315-324`, `c6_phys.rs:3351`, `c6_phys.rs:2170`.
    fn bound_valid_control(em: &mut Em, inputs: &C6StepInputs) -> ControlPlan {
        let en = inputs.enable.clone();
        let k = &inputs.pre_image[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let caps = &inputs.geometry.caps.phys;
        em.geq(en.clone(), k[0].clone(), Em::imm(5));
        em.geq(en.clone(), k[1].clone(), Em::imm(1));
        em.gated_validate_env(en.clone(), k[2].clone(), k[3].clone(), caps.max_env_cells);
        em.gated_lt_index(en.clone(), k[4].clone(), caps.max_types.max(4));
        em.gated_lt_index(en, k[5].clone(), caps.max_expressions);
        Self::control_padding(em, inputs, 6);
        let mut ctl = vec![Em::imm(0); CONTROL_LANES];
        ctl[0] = Em::imm(2);
        ctl[1] = Em::imm(7);
        ctl[2] = k[4].clone();
        ctl[3] = inputs.pre_image[L_ENV_ROOT].clone();
        ctl[4] = inputs.pre_image[L_ENV_ROOT + 1].clone();
        ctl[5] = inputs.pre_image[L_CONTROL_ORIGIN].clone();
        ctl[14] = k[5].clone();
        ctl[15] = k[2].clone();
        ctl[16] = k[3].clone();
        ControlPlan {
            ctl,
            clears_result: false,
        }
    }

    /// Writes: post image `L_CONTROL..+CONTROL_LANES`; preserves every other
    /// image header lane outside `legacy_control_lanes()` and every frame lane.
    /// Requires: `plan.ctl.len() == CONTROL_LANES`, built from pre lanes only.
    /// Ensures: under enable post control = plan exactly; no frame push/pop,
    /// env_root, origin, depth unchanged.
    /// Check: `legacy.proposed.rs` (only `writes.control`/`writes.result` set).
    fn write_control(em: &mut Em, inputs: &C6StepInputs, plan: &ControlPlan) {
        set_control(em, inputs, &plan.ctl);
        preserve_image(em, inputs, &legacy_control_lanes(), false);
    }

    /// Writes: post image `L_REMAINING_WORK`, `L_CONSUMED_WORK`,
    /// `L_REFERENCE_CURSOR`.
    /// Ensures: under enable remaining - 1, consumed + 1, cursor + 1 with no
    /// 128-bit overflow (fuel availability from `select`).
    /// Check: `owner_runtime.rs:1373-1377` (LegacyComplete charge).
    fn charge(em: &mut Em, inputs: &C6StepInputs) {
        charge_legacy(em, inputs);
    }

    /// Writes: every post DATA lane.
    /// Ensures: under enable post DATA = pre DATA.
    /// Check: `legacy.proposed.rs:289-324` (no data writes).
    fn preserve_data(em: &mut Em, inputs: &C6StepInputs) {
        for (pre, post) in inputs.pre_data.iter().zip(&inputs.post_data) {
            em.geq(inputs.enable.clone(), post.clone(), pre.clone());
        }
    }

    /// Writes: every post HELPER lane.
    /// Ensures: under enable, the result record `L_RESULT_RECORD..+RESULT_LANES`
    /// is zero (None) if `clears_result`, else preserved; all other helper
    /// lanes preserved.
    /// Check: `legacy.proposed.rs:297`, `legacy.proposed.rs:311`
    /// (`writes.result = Write::Set(None)`).
    fn write_helper(em: &mut Em, inputs: &C6StepInputs, clears_result: bool) {
        for (lane, (pre, post)) in inputs
            .pre_helper
            .iter()
            .zip(&inputs.post_helper)
            .enumerate()
        {
            if clears_result && (L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES).contains(&lane) {
                em.gzero(inputs.enable.clone(), post.clone());
            } else {
                em.geq(inputs.enable.clone(), post.clone(), pre.clone());
            }
        }
    }

    /// Reads: enable, pre/post META, pre PROG, pre CTX, pre/post DATA,
    /// pre/post HELPER.
    /// Ensures: under enable META bound canonical, PROG/CTX canonical and bound
    /// to the expected ctx, pre and post DATA and HELPER canonical (retained
    /// serial composition, shared with `enter_bind_control`'s PROG capture).
    /// Check: `owner_runtime.rs` `step` (carriers are owner snapshots).
    fn canonical(&mut self, em: &mut Em, inputs: &C6StepInputs) -> Result<(), String> {
        compose_canonical(&mut self.composed, em, inputs)
    }

    /// Writes: all post IMAGE, DATA, HELPER lanes under `1 - enable`.
    /// Ensures: enable = 0 => post = pre on those lanes (META/PROG/CTX by
    /// `prefix_and_validate`).
    /// Check: task statement (disabled step is identity).
    fn disabled(em: &mut Em, inputs: &C6StepInputs) {
        disabled_identity(em, inputs);
    }

    fn control_padding(em: &mut Em, inputs: &C6StepInputs, used: usize) {
        for lane in L_CONTROL + used..L_CONTROL + CONTROL_LANES {
            em.gzero(inputs.enable.clone(), inputs.pre_image[lane].clone());
        }
    }

    /// SC-05 written code membership, SC-07 copied environment membership,
    /// and the source occurrence's expression index used by owner snapshots.
    fn validate_written_ids(
        &mut self,
        em: &mut Em,
        rule: RuleIdC,
        inputs: &C6StepInputs,
        plan: &ControlPlan,
    ) -> Result<(), String> {
        let en = inputs.enable.clone();
        let origin = inputs.pre_image[L_CONTROL_ORIGIN].clone();
        let body = match rule {
            RuleIdC::KMachineInvokeFind => origin.clone(),
            // Re-entering the retained PROG emitter must use a parent-kit
            // operand, not an operand in its reserved temporary namespace.
            RuleIdC::KMachineEnterBind => em.add(plan.ctl[5].clone(), Em::imm(0)),
            RuleIdC::KMachineBoundValid => plan.ctl[14].clone(),
            _ => unreachable!("checked by call_bind_supported"),
        };
        let program = self
            .composed
            .progctx_em
            .emit_prog_dispatch_capture(&C6ProgramContextDispatchCaptureInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps: inputs.geometry.caps,
                first_index: body,
                second_index: origin,
                child_index: Em::imm(0),
            })
            .map_err(|e| e.to_string())?;
        em.ins.extend(program.instructions);
        em.gassert(en.clone(), program.first_found);
        em.gassert(en.clone(), program.second_found);
        match rule {
            RuleIdC::KMachineInvokeFind => {
                let found_gate = em.and(en, plan.ctl[1].clone());
                let exists = em.lt128(plan.ctl[2].clone(), program.functions_count);
                em.gassert(found_gate, exists);
            }
            RuleIdC::KMachineEnterBind => {
                Self::active_env(em, inputs, &plan.ctl[2], &plan.ctl[3]);
            }
            RuleIdC::KMachineBoundValid => {
                Self::active_env(em, inputs, &plan.ctl[3], &plan.ctl[4]);
                Self::active_env(em, inputs, &plan.ctl[15], &plan.ctl[16]);
            }
            _ => unreachable!("checked by call_bind_supported"),
        }
        Ok(())
    }

    fn active_env(em: &mut Em, inputs: &C6StepInputs, present: &Operand, index: &Operand) {
        let gate = em.and(inputs.enable.clone(), present.clone());
        let exists = em.lt128(index.clone(), inputs.pre_image[L_ACTIVE + 2].clone());
        em.gassert(gate, exists);
    }
}

/// One-shot emission of the CallBind control relation for `rule`.
/// Ensures: same as `C6CallBindEmitter::new(scope)?.emit(rule, inputs)`.
pub fn emit_c6_call_bind_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6CallBindBlock, C6CallBindError> {
    C6CallBindEmitter::new(scope)?.emit(rule, inputs)
}

// 16 functions, 6 global invariants.
