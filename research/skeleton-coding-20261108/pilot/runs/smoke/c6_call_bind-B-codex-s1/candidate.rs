//! C6 CallBind control step: the legacy `k.machine.invoke.find`,
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
use mori_core::sherlock::helper_machine::c6_phys::{
    CONTROL_LANES, L_ACTIVE, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_OUTCOME,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_RESULT_RECORD, RESULT_LANES};
use mori_core::sherlock::helper_machine::c6_phys_program::PROGRAM_HEADER_LANES;
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_program_context_canonical::C6ProgramContextFunctionCaptureInputs;
use super::c6_step_kit::{
    C6StepInputs, Composed, Em, charge_legacy, check_shapes, compose_canonical, disabled_identity,
    legacy_control_lanes, prefix, preserve_image, select, set_control, validate_pre_image,
};

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
    CALL_BIND_RULES.contains(&rule)
}

// Control offsets inside `pre/post_image[L_CONTROL..]`, per `encode_control`.
// Dispatch already pins the pre control tag and the result tag alternatives.
const K_TAG: usize = 0;
const TAG_SCHEDULE_NEXT: u128 = 2;
// Invoke: [2..4] args slice (start, len).
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
const R_BIND_OK: u128 = 7;
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
        if scope.trim().is_empty() || scope.contains('%') || scope.contains(".c6kit.") {
            return Err(C6CallBindError::InvalidScope);
        }
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            composed: Composed::new(scope).map_err(C6CallBindError::Emit)?,
        })
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
        prefix(em, inputs);
        validate_pre_image(em, inputs);
        select(&mut self.composed, em, rule, inputs)?;
        // Dispatch requires Running; its two unused outcome lanes are padding.
        for lane in L_OUTCOME + 1..L_OUTCOME + 3 {
            em.gzero(inputs.enable.clone(), inputs.pre_image[lane].clone());
        }
        let control = match rule {
            RuleIdC::KMachineInvokeFind => {
                let control = Self::invoke_find_control(em, inputs);
                // SC-05: a written Some(function) must name a live PROG row,
                // not merely an index below the function-table capacity.
                let mut lookup_inputs = inputs.clone();
                lookup_inputs.enable =
                    em.and(inputs.enable.clone(), control[K_ENTER_HAS_FN].clone());
                self.capture_function_row(em, &lookup_inputs, control[K_ENTER_FN].clone())?;
                control
            }
            RuleIdC::KMachineEnterBind => self.enter_bind_control(em, inputs)?,
            RuleIdC::KMachineBoundValid => Self::bound_valid_control(em, inputs),
            _ => unreachable!("checked by call_bind_supported"),
        };
        set_control(em, inputs, &control);
        preserve_image(em, inputs, &legacy_control_lanes(), false);
        charge_legacy(em, inputs);
        preserve_data(em, inputs);
        constrain_helper(em, inputs, clears_result(rule));
        compose_canonical(&mut self.composed, em, inputs)?;
        disabled_identity(em, inputs);
        Ok(())
    }

    /// Reads: pre control `[K_TAG, K_INVOKE_ARGS..+2]`, pre result `[0, 1]`.
    /// Returns the post control vector (`CONTROL_LANES` entries).
    /// Requires: `select` pinned `rule = InvokeFind` (Invoke family).
    /// Ensures (enable = 1): pre tag = Invoke (3); res[0] in {FIND_FOUND,
    /// FIND_MISSING}; post = `[TAG_ENTER, found, found*res[1], args.start,
    /// args.len, 0..]` with `found = (res[0] == R_FIND_FOUND)`.
    /// Check: legacy.proposed.rs:290-297; owner_runtime.rs:618; owner_runtime.rs:1708.
    fn invoke_find_control(em: &mut Em, inputs: &C6StepInputs) -> Vec<Operand> {
        let en = inputs.enable.clone();
        let caps = &inputs.geometry.caps.phys;
        let k = &inputs.pre_image[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let res = &inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES];
        // Complete the pre control representation checks not made by dispatch.
        em.gated_lt_index(en.clone(), k[1].clone(), caps.max_names);
        em.gated_lt_count(en.clone(), k[K_INVOKE_ARGS + 1].clone(), caps.max_arity);
        // The copied args slice is also checked by owner_validate_plan:
        // empty slices start at zero; nonempty slices end within live slots.
        let empty = em.teq(k[K_INVOKE_ARGS + 1].clone(), Em::imm(0));
        let empty_gate = em.and(en.clone(), empty);
        em.gzero(empty_gate, k[K_INVOKE_ARGS].clone());
        let end = em.add(k[K_INVOKE_ARGS].clone(), k[K_INVOKE_ARGS + 1].clone());
        let live_end = em.add(inputs.pre_image[L_ACTIVE + 3].clone(), Em::imm(1));
        let fits = em.less_than_129(end, live_end);
        em.gassert(en.clone(), fits);
        for lane in &k[K_INVOKE_ARGS + 2..] {
            em.gzero(en.clone(), lane.clone());
        }
        let found = em.teq(res[0].clone(), Em::imm(R_FIND_FOUND));
        let mut control = vec![Em::imm(0); CONTROL_LANES];
        control[K_TAG] = Em::imm(TAG_ENTER);
        control[K_ENTER_HAS_FN] = found.clone();
        control[K_ENTER_FN] = em.mul(found, res[1].clone());
        control[K_ENTER_ARGS..K_ENTER_ARGS + 2]
            .clone_from_slice(&k[K_INVOKE_ARGS..K_INVOKE_ARGS + 2]);
        control
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
        let en = inputs.enable.clone();
        let caps = &inputs.geometry.caps.phys;
        let k = &inputs.pre_image[L_CONTROL..L_CONTROL + CONTROL_LANES];
        let res = &inputs.pre_helper[L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES];
        // A staged Bind result selects this row even for Enter(None); the
        // owner's plan additionally requires the resolved function payload.
        em.geq(en.clone(), k[K_ENTER_HAS_FN].clone(), Em::imm(1));
        em.gated_validate_slice(
            en.clone(),
            k,
            K_ENTER_ARGS,
            caps.max_value_slots,
            Some(caps.max_arity),
        );
        for lane in &k[K_ENTER_ARGS + 2..] {
            em.gzero(en.clone(), lane.clone());
        }
        let row = self.capture_function_row(em, inputs, k[K_ENTER_FN].clone())?;
        let ok = em.teq(res[0].clone(), Em::imm(R_BIND_OK));
        let mut control = vec![Em::imm(0); CONTROL_LANES];
        control[K_TAG] = Em::imm(TAG_BOUND);
        control[K_BOUND_OK] = ok.clone();
        control[K_BOUND_ENV] = em.mul(ok.clone(), res[1].clone());
        control[K_BOUND_ENV + 1] = em.mul(ok, res[2].clone());
        control[K_BOUND_RT] = row[FN_ROW_RETURN_TYPE].clone();
        control[K_BOUND_BODY] = row[FN_ROW_BODY].clone();
        // SC-05 checks the written body against the actual expression count.
        let body_live = em.lt128(
            control[K_BOUND_BODY].clone(),
            inputs.pre_prog[PROGRAM_HEADER_LANES].clone(),
        );
        em.gassert(en.clone(), body_live);
        em.gated_lt_index(en, control[K_BOUND_RT].clone(), caps.max_types.max(4));
        constrain_live_env(em, inputs, &control[K_BOUND_ENV..K_BOUND_ENV + 2]);
        Ok(control)
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
        let block = self
            .composed
            .progctx_em
            .emit_prog_function_capture(&C6ProgramContextFunctionCaptureInputs {
                gate: inputs.enable.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps: inputs.geometry.caps,
                function_index,
            })
            .map_err(|e| e.to_string())?;
        em.ins.extend(block.instructions);
        em.gassert(inputs.enable.clone(), block.function_found);
        Ok(block.function_row.to_vec())
    }

    /// Reads: pre control `[K_TAG, K_BOUND_OK, K_BOUND_ENV..+2, K_BOUND_RT,
    /// K_BOUND_BODY]`, pre `L_ENV_ROOT..+2`, pre `L_CONTROL_ORIGIN`.
    /// Requires: `select` pinned `rule = BoundValid`.
    /// Ensures (enable = 1): pre tag = `TAG_BOUND`, ok = 1; post =
    /// `[TAG_SCHEDULE_NEXT, frame = (FRAME_TAG_RETURN, rt, env_root.0,
    /// env_root.1, origin, 0..), next = body, env = bound env]`, other lanes 0.
    /// Check: legacy.proposed.rs:316-323; owner_runtime.rs:634; c6_phys.rs:3351.
    fn bound_valid_control(em: &mut Em, inputs: &C6StepInputs) -> Vec<Operand> {
        let en = inputs.enable.clone();
        let caps = &inputs.geometry.caps.phys;
        let pre = &inputs.pre_image;
        let k = &pre[L_CONTROL..L_CONTROL + CONTROL_LANES];
        // The positive binding tag is already required by dispatch.
        em.gated_validate_env(
            en.clone(),
            k[K_BOUND_ENV].clone(),
            k[K_BOUND_ENV + 1].clone(),
            caps.max_env_cells,
        );
        em.gated_lt_index(en.clone(), k[K_BOUND_RT].clone(), caps.max_types.max(4));
        for expr in [&k[K_BOUND_BODY], &pre[L_CONTROL_ORIGIN]] {
            let live = em.lt128(expr.clone(), inputs.pre_prog[PROGRAM_HEADER_LANES].clone());
            em.gassert(en.clone(), live);
        }
        constrain_live_env(em, inputs, &k[K_BOUND_ENV..K_BOUND_ENV + 2]);
        constrain_live_env(em, inputs, &pre[L_ENV_ROOT..L_ENV_ROOT + 2]);
        for lane in &k[K_BOUND_BODY + 1..] {
            em.gzero(en.clone(), lane.clone());
        }
        let mut control = vec![Em::imm(0); CONTROL_LANES];
        control[K_TAG] = Em::imm(TAG_SCHEDULE_NEXT);
        control[K_SCHED_FRAME] = Em::imm(FRAME_TAG_RETURN);
        control[K_SCHED_FRAME + 1] = k[K_BOUND_RT].clone();
        control[K_SCHED_FRAME + 2..K_SCHED_FRAME + 4]
            .clone_from_slice(&pre[L_ENV_ROOT..L_ENV_ROOT + 2]);
        control[K_SCHED_FRAME + 4] = pre[L_CONTROL_ORIGIN].clone();
        control[K_SCHED_NEXT] = k[K_BOUND_BODY].clone();
        control[K_SCHED_ENV..K_SCHED_ENV + 2].clone_from_slice(&k[K_BOUND_ENV..K_BOUND_ENV + 2]);
        control
    }
}

/// SC-07: an environment copied into the new control is below active.env.
/// Optional-environment representation is checked by the caller or HELPER.
fn constrain_live_env(em: &mut Em, inputs: &C6StepInputs, env: &[Operand]) {
    let gate = em.and(inputs.enable.clone(), env[0].clone());
    let live = em.lt128(env[1].clone(), inputs.pre_image[L_ACTIVE + 2].clone());
    em.gassert(gate, live);
}

/// Ensures: true for InvokeFind and EnterBind (plan writes `result = None`),
/// false for BoundValid.
/// Check: legacy.proposed.rs:297, 311 (and absent at 316-323).
fn clears_result(rule: RuleIdC) -> bool {
    matches!(
        rule,
        RuleIdC::KMachineInvokeFind | RuleIdC::KMachineEnterBind
    )
}

/// Reads: `pre_data`, `post_data`. Fixes: every post DATA lane.
/// Requires: none. Ensures (enable = 1): post_data = pre_data.
/// Check: legacy.proposed.rs:289-323 (no DATA write in these rows).
fn preserve_data(em: &mut Em, inputs: &C6StepInputs) {
    for (pre, post) in inputs.pre_data.iter().zip(&inputs.post_data) {
        em.geq(inputs.enable.clone(), post.clone(), pre.clone());
    }
}

/// Reads: `pre_helper`, `post_helper`. Fixes: every post HELPER lane.
/// Requires: `clear == clears_result(rule)`.
/// Ensures (enable = 1): result-record lanes `L_RESULT_RECORD..+RESULT_LANES`
/// are zero when `clear`, else preserved; every other helper lane preserved.
/// Check: legacy.proposed.rs:297, 311.
fn constrain_helper(em: &mut Em, inputs: &C6StepInputs, clear: bool) {
    for (lane, (pre, post)) in inputs
        .pre_helper
        .iter()
        .zip(&inputs.post_helper)
        .enumerate()
    {
        if clear && (L_RESULT_RECORD..L_RESULT_RECORD + RESULT_LANES).contains(&lane) {
            em.gzero(inputs.enable.clone(), post.clone());
        } else {
            em.geq(inputs.enable.clone(), post.clone(), pre.clone());
        }
    }
}

// The control builders also enforce the owner's written-id, environment and
// slice validation; no predecessor's reachability is assumed for these writes.
