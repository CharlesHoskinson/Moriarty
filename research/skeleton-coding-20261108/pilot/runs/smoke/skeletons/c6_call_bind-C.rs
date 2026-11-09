//! C6 CallBind control step (SKELETON): the legacy `k.machine.invoke.find`,
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
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_step_kit::{C6StepInputs, Composed, Em};

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
    todo!()
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
        todo!()
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
        todo!()
    }

    /// Orchestration: prefix, pre-image validation, `select`, the rule's
    /// control plan, `write_control`, `charge`, `preserve_data`,
    /// `write_helper`, `canonical`, `disabled`. Owns no constraint itself.
    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        todo!()
    }

    /// Reads: enable, host_schema, carrier_version, all pre/post lanes; pre
    /// image header and frames.
    /// Ensures: enable boolean; schema/carrier fixed; post META/PROG/CTX = pre
    /// unconditionally; every lane is range-checked; under enable pre META/CTX/
    /// IMAGE magic+version and every active pre frame is valid.
    /// Check: `owner_runtime.rs` `step` (P0 publication preconditions).
    fn prefix_and_validate(em: &mut Em, inputs: &C6StepInputs) {
        todo!()
    }

    /// Reads: enable, pre PROG/IMAGE/DATA/HELPER.
    /// Ensures: under enable, the composed dispatch owner selects exactly
    /// `rule` from the pre state (P0/P1/P2 not taken, fuel available,
    /// `owner_select` picks `rule`, incl. its guard: InvokeFind needs result
    /// FindFound|FindMissing, EnterBind BindOk|BindBad, BoundValid control
    /// Bound{Ok}).
    /// Check: `owner_runtime.rs:618-640`, `owner_runtime.rs:1708-1710`.
    fn select(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        todo!()
    }

    /// Reads: enable, pre control `k[0], k[2], k[3]`, pre result `res[0], res[1]`.
    /// Ensures: under enable `k[0]` = Invoke; plan = Enter { found =
    /// [res[0] = FindFound], function = found * res[1], args = (k[2], k[3]) },
    /// remaining control lanes 0; clears_result = true.
    /// Check: `legacy.proposed.rs:289-299`.
    fn invoke_find_control(em: &mut Em, inputs: &C6StepInputs) -> ControlPlan {
        todo!()
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
        todo!()
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
        todo!()
    }

    /// Writes: post image `L_CONTROL..+CONTROL_LANES`; preserves every other
    /// image header lane outside `legacy_control_lanes()` and every frame lane.
    /// Requires: `plan.ctl.len() == CONTROL_LANES`, built from pre lanes only.
    /// Ensures: under enable post control = plan exactly; no frame push/pop,
    /// env_root, origin, depth unchanged.
    /// Check: `legacy.proposed.rs` (only `writes.control`/`writes.result` set).
    fn write_control(em: &mut Em, inputs: &C6StepInputs, plan: &ControlPlan) {
        todo!()
    }

    /// Writes: post image `L_REMAINING_WORK`, `L_CONSUMED_WORK`,
    /// `L_REFERENCE_CURSOR`.
    /// Ensures: under enable remaining - 1, consumed + 1, cursor + 1 with no
    /// 128-bit overflow (fuel availability from `select`).
    /// Check: `owner_runtime.rs:1373-1377` (LegacyComplete charge).
    fn charge(em: &mut Em, inputs: &C6StepInputs) {
        todo!()
    }

    /// Writes: every post DATA lane.
    /// Ensures: under enable post DATA = pre DATA.
    /// Check: `legacy.proposed.rs:289-324` (no data writes).
    fn preserve_data(em: &mut Em, inputs: &C6StepInputs) {
        todo!()
    }

    /// Writes: every post HELPER lane.
    /// Ensures: under enable, the result record `L_RESULT_RECORD..+RESULT_LANES`
    /// is zero (None) if `clears_result`, else preserved; all other helper
    /// lanes preserved.
    /// Check: `legacy.proposed.rs:297`, `legacy.proposed.rs:311`
    /// (`writes.result = Write::Set(None)`).
    fn write_helper(em: &mut Em, inputs: &C6StepInputs, clears_result: bool) {
        todo!()
    }

    /// Reads: enable, pre/post META, pre PROG, pre CTX, pre/post DATA,
    /// pre/post HELPER.
    /// Ensures: under enable META bound canonical, PROG/CTX canonical and bound
    /// to the expected ctx, pre and post DATA and HELPER canonical (retained
    /// serial composition, shared with `enter_bind_control`'s PROG capture).
    /// Check: `owner_runtime.rs` `step` (carriers are owner snapshots).
    fn canonical(&mut self, em: &mut Em, inputs: &C6StepInputs) -> Result<(), String> {
        todo!()
    }

    /// Writes: all post IMAGE, DATA, HELPER lanes under `1 - enable`.
    /// Ensures: enable = 0 => post = pre on those lanes (META/PROG/CTX by
    /// `prefix_and_validate`).
    /// Check: task statement (disabled step is identity).
    fn disabled(em: &mut Em, inputs: &C6StepInputs) {
        todo!()
    }
}

/// One-shot emission of the CallBind control relation for `rule`.
/// Ensures: same as `C6CallBindEmitter::new(scope)?.emit(rule, inputs)`.
pub fn emit_c6_call_bind_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6CallBindBlock, C6CallBindError> {
    todo!()
}

// 16 functions, 6 global invariants.
