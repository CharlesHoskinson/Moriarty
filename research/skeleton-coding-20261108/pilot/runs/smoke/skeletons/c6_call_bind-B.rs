//! C6 CallBind control step (SKELETON): the legacy `k.machine.invoke.find`,
//! `k.machine.enter.bind` and `k.machine.bound.valid` rows as one per-rule
//! relation over the full C6 carriers. Conventions follow
//! `c6_legacy_control_step.rs`; shared gates come from `c6_step_kit.rs`.
//!
//! Plans (`fragments/legacy.proposed.rs:289-323`):
//! - InvokeFind: `Invoke { args }` + `FindFound { f } | FindMissing` ->
//!   `Enter { function: Some(f) | None, args }`; result cleared.
//! - EnterBind: `Enter { Some(f) }` + `BindOk { env } | BindBad` ->
//!   `Bound { binding, return_type: PROG.fn[f].return_type, body: PROG.fn[f].body }`;
//!   result cleared.
//! - BoundValid: `Bound { Ok { env }, rt, body }` -> `ScheduleNext { frame:
//!   ReturnF { rt, saved_env: env_root, origin: control_origin }, next: body, env }`;
//!   result, env root, origin and frames untouched (the push is a later rule).
//!
//! Global invariants:
//! 1. Every post IMAGE/DATA/HELPER lane is, under enable, either written by the
//!    plan (all `CONTROL_LANES` control lanes, the three LegacyComplete charge
//!    lanes, the result record when cleared) or equal to its pre lane.
//! 2. enable = 0 accepts only post = pre (`disabled_identity`); META/PROG/CTX
//!    post = pre unconditionally (`prefix`).
//! 3. The rule is a build-time constant; no witness lane selects among the three.
//! 4. All `CONTROL_LANES` post control lanes are fixed, including zero padding
//!    (Enter.function = 0 when missing, Bound.env = 0 when Bad).
//! 5. Frame slots and `L_FRAME_COUNT` are preserved; no push/pop in these rows.
//! 6. `L_ENV_ROOT` and `L_CONTROL_ORIGIN` are preserved for all three rules.
//! 7. Every enable-gated constraint is vacuous at enable = 0, so the identity
//!    pair is satisfiable for any canonical pre-state.
//! 8. META/PROG/CTX/DATA (pre, post) and HELPER (pre, post) canonicality is
//!    composed once each; the PROG function capture reuses the retained
//!    `progctx_em` serial namespace.
//! 9. No check duplicated with `select` / `compose_canonical` (minimal count);
//!    every non-statement value is an `Em` temp, there are no private inputs.
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_step_kit::{C6StepInputs, Composed, Em};

#[derive(Debug, Error)]
pub enum C6CallBindError {
    #[error("rule {0:?} is not a CallBind control rule")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 call bind emission: {0}")]
    Emit(String),
}

pub const CALL_BIND_RULES: [RuleIdC; 3] = [
    RuleIdC::KMachineInvokeFind,
    RuleIdC::KMachineEnterBind,
    RuleIdC::KMachineBoundValid,
];

/// Ensures: true exactly for `CALL_BIND_RULES`.
pub fn call_bind_supported(rule: RuleIdC) -> bool {
    todo!()
}

// ---- Statement-lane stubs (offsets inside `pre/post_image[L_CONTROL..]`,
// per `c6_phys::encode_control`; verify against c6_phys.rs:892-935). ----
const K_TAG: usize = 0;
const TAG_SCHEDULE_NEXT: u128 = 2;
const TAG_INVOKE: u128 = 3; // [2..4] args slice (start, len)
const K_INVOKE_ARGS: usize = 2;
const TAG_ENTER: u128 = 4; // [1] has_fn, [2] fn, [3..5] args
const K_ENTER_HAS_FN: usize = 1;
const K_ENTER_FN: usize = 2;
const K_ENTER_ARGS: usize = 3;
const TAG_BOUND: u128 = 5; // [1] ok, [2..4] env (present, idx), [4] rt, [5] body
const K_BOUND_OK: usize = 1;
const K_BOUND_ENV: usize = 2;
const K_BOUND_RT: usize = 4;
const K_BOUND_BODY: usize = 5;
// ScheduleNext: [1..14] frame, [14] next, [15..17] env.
const K_SCHED_FRAME: usize = 1;
const K_SCHED_NEXT: usize = 14;
const K_SCHED_ENV: usize = 15;
/// ReturnF frame row: [0]=7, [1] rt, [2..4] saved_env, [4] origin.
const FRAME_TAG_RETURN: u128 = 7;
// Result tags (`pre_helper[L_RESULT_RECORD..]`, c6_phys_helper.rs:841-850):
// res[1] = function (FindFound); res[1..3] = env opt (BindOk).
const R_FIND_FOUND: u128 = 5;
const R_FIND_MISSING: u128 = 6;
const R_BIND_OK: u128 = 7;
const R_BIND_BAD: u128 = 8;
/// FUNCTION row lanes from `emit_prog_function_capture`.
const FN_ROW_RETURN_TYPE: usize = 3;
const FN_ROW_BODY: usize = 4;

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
    /// Requires: nothing. Ensures: `Err(InvalidScope)` for empty scopes or ones
    /// containing `%` or `.c6kit.`; otherwise fresh retained `Composed`.
    /// Check: c6_legacy_control_step.rs `new` convention.
    pub fn new(scope: &str) -> Result<Self, C6CallBindError> {
        todo!()
    }

    /// Requires: `inputs` from `build_step_circuit`. Ensures: `Unsupported` for
    /// rules outside `CALL_BIND_RULES`; `check_shapes` errors surface as `Emit`;
    /// otherwise the block of `body`, temp counter advanced.
    /// Check: c6_call_bind_consumer.rs:49-56.
    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6CallBindBlock, C6CallBindError> {
        todo!()
    }

    /// Orchestration, in order: `prefix`, `validate_pre_image`, `select`, the
    /// rule's control builder, `set_control`, `preserve_image(legacy_control_lanes(),
    /// false)`, `charge_legacy`, `preserve_data`, `constrain_helper`,
    /// `compose_canonical`, `disabled_identity`.
    /// Reads: all lanes. Fixes: all post lanes (Global 1).
    /// Requires: `call_bind_supported(rule)`, shapes checked.
    /// Ensures: enable = 1 accepts exactly the owner step of `rule`; enable = 0
    /// accepts exactly the identity.
    /// Check: owner_runtime.rs `step` (P0/P1/P2, fuel, `owner_select`, plan, write).
    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        todo!()
    }

    /// Reads: pre control `[K_TAG, K_INVOKE_ARGS..+2]`, pre result `[0, 1]`.
    /// Returns the post control vector (`CONTROL_LANES` entries).
    /// Requires: `select` pinned `rule = InvokeFind` (Invoke family).
    /// Ensures (enable = 1): pre tag = `TAG_INVOKE`; res[0] in {FIND_FOUND,
    /// FIND_MISSING}; post = `[TAG_ENTER, found, found*res[1], args.start,
    /// args.len, 0..]` with `found = (res[0] == R_FIND_FOUND)`.
    /// Check: legacy.proposed.rs:290-297; owner_runtime.rs:618; owner_runtime.rs:1708.
    fn invoke_find_control(em: &mut Em, inputs: &C6StepInputs) -> Vec<Operand> {
        todo!()
    }

    /// Reads: pre control `[K_TAG, K_ENTER_HAS_FN, K_ENTER_FN]`, pre result
    /// `[0..3]`, PROG function row via `capture_function_row`.
    /// Requires: `select` pinned `rule = EnterBind` (Enter family, not Missing).
    /// Ensures (enable = 1): pre tag = `TAG_ENTER`, has_fn = 1; function found in
    /// PROG; res[0] in {BIND_OK, BIND_BAD}; post = `[TAG_BOUND, ok, ok*env.present,
    /// ok*env.idx, row.return_type, row.body, 0..]`, `ok = (res[0] == R_BIND_OK)`.
    /// Check: legacy.proposed.rs:302-312; owner_runtime.rs:623; owner_runtime.rs:1709.
    fn enter_bind_control(
        &mut self,
        em: &mut Em,
        inputs: &C6StepInputs,
    ) -> Result<Vec<Operand>, String> {
        todo!()
    }

    /// Reads: `pre_prog`, `geometry.caps`. Returns the 5-lane FUNCTION row at
    /// `function_index`, with `function_found` asserted under enable.
    /// Requires: PROG canonicality composed later in the same relation.
    /// Ensures: returned lanes are PROG's row `function_index` (`pre.function_row`).
    /// Check: legacy.proposed.rs:305.
    fn capture_function_row(
        &mut self,
        em: &mut Em,
        inputs: &C6StepInputs,
        function_index: Operand,
    ) -> Result<Vec<Operand>, String> {
        todo!()
    }

    /// Reads: pre control `[K_TAG, K_BOUND_OK, K_BOUND_ENV..+2, K_BOUND_RT,
    /// K_BOUND_BODY]`, pre `L_ENV_ROOT..+2`, pre `L_CONTROL_ORIGIN`.
    /// Requires: `select` pinned `rule = BoundValid`.
    /// Ensures (enable = 1): pre tag = `TAG_BOUND`, ok = 1; post =
    /// `[TAG_SCHEDULE_NEXT, frame = (FRAME_TAG_RETURN, rt, env_root.0,
    /// env_root.1, origin, 0..), next = body, env = bound env]`, other lanes 0.
    /// Check: legacy.proposed.rs:316-323; owner_runtime.rs:634; c6_phys.rs:3351.
    fn bound_valid_control(em: &mut Em, inputs: &C6StepInputs) -> Vec<Operand> {
        todo!()
    }
}

/// Ensures: true for InvokeFind and EnterBind (plan writes `result = None`),
/// false for BoundValid.
/// Check: legacy.proposed.rs:297, 311 (and absent at 316-323).
fn clears_result(rule: RuleIdC) -> bool {
    todo!()
}

/// Reads: `pre_data`, `post_data`. Fixes: every post DATA lane.
/// Requires: none. Ensures (enable = 1): post_data = pre_data.
/// Check: legacy.proposed.rs:289-323 (no DATA write in these rows).
fn preserve_data(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: `pre_helper`, `post_helper`. Fixes: every post HELPER lane.
/// Requires: `clear == clears_result(rule)`.
/// Ensures (enable = 1): result-record lanes `L_RESULT_RECORD..+RESULT_LANES`
/// are zero when `clear`, else preserved; every other helper lane preserved.
/// Check: legacy.proposed.rs:297, 311.
fn constrain_helper(em: &mut Em, inputs: &C6StepInputs, clear: bool) {
    todo!()
}

// 11 functions, 9 global invariants.
