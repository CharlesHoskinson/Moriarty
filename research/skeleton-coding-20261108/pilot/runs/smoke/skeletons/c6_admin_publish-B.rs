//! C6 admin publish step: the `k.admin.publish.value` row every factorial and
//! tree fold run fires last, as one per-rule relation over the full C6
//! carriers. SKELETON: bodies are `todo!()`.
//!
//! Reference (`owner_runtime.rs` `step`, P1 at ~3709-3733): with outcome
//! Running, helper and result absent, control `Return { value }` and no frames,
//! the owner sets `outcome = Halted { value }` and returns a Publish event
//! (`publish_event`: charge 0, no alloc deltas). P1 runs before the P2
//! `remaining_work == 0` refusal, so a zero remaining-work pre is accepted.
//!
//! Statement lanes (`C6StepInputs`):
//! - enable, host_schema, carrier_version: `fix_prefix`.
//! - pre/post META, PROG, CTX: `fix_prefix` (unconditional equality + ranges).
//! - pre IMAGE: `validate_pre`, `select_rule` (outcome, control, depth),
//!   `write_outcome` (control value ref 3..5), `preserve_image`.
//! - pre DATA / pre HELPER: `select_rule` (helper/result absent).
//! - post IMAGE 23..26 (outcome tag + ref): `write_outcome` only.
//! - post IMAGE every other lane (header incl. control, fuel/work/cursor
//!   lanes 26..32, alloc/active/seg lanes, frame count, all frame slots):
//!   `preserve_image`.
//! - post DATA: `preserve_data`. post HELPER: `preserve_helper`.
//!
//! Global invariants:
//! 1. Every post IMAGE/DATA/HELPER lane is fixed under enable = 1 by exactly
//!    one of `write_outcome` (lanes 23..26) or a preserve function (all
//!    others); no lane is left free and none is constrained twice.
//! 2. enable = 0 accepts exactly the identity pair: `disabled_identity`
//!    fixes IMAGE/DATA/HELPER, `fix_prefix` fixes META/PROG/CTX (ungated),
//!    and every other gate is multiplied by enable so it is vacuous.
//! 3. enable is boolean and every statement lane is range-checked to 128 bits
//!    before any comparison relies on it (`fix_prefix`).
//! 4. Charge is 0: no work, helper, visit, cursor or alloc lane moves, and no
//!    gate requires remaining_work > 0 (P1 precedes P2).
//! 5. The pre carriers decode to an owner state (composed canonicality +
//!    `validate_pre`), and post canonicality is implied by pre canonicality
//!    plus the write (the copied ref is the pre control's validated ref).
//! 6. The relation is per-rule: `emit` rejects every rule not in
//!    `ADMIN_PUBLISH_RULES`, and the dispatch owner guarantees that no other
//!    owner rule fires from an accepted pre (P0/P1 priority is composed).
//! 7. Temporaries stay in the emitter's scope namespace; the retained
//!    `Composed` serials make repeated `emit` calls collision free.
use midnight_zkir_v3::Instruction;
use mori_core::sherlock::helper_machine::c6_phys::{L_CONTROL, L_OUTCOME};
use mori_core::sherlock::helper_machine::shared::RuleIdC;
use thiserror::Error;

use super::c6_step_kit::{C6StepInputs, Composed, Em};

#[derive(Debug, Error)]
pub enum C6AdminPublishError {
    #[error("rule {0:?} is not an admin publish rule of this fragment")]
    Unsupported(RuleIdC),
    #[error("invalid scope")]
    InvalidScope,
    #[error("C6 admin publish emission: {0}")]
    Emit(String),
}

pub const ADMIN_PUBLISH_RULES: [RuleIdC; 1] = [RuleIdC::KAdminPublishValue];

pub fn admin_publish_supported(rule: RuleIdC) -> bool {
    todo!()
}

/// Outcome tag of `Halted` (`c6_phys::put_outcome`).
const OUTCOME_HALTED: u128 = 1;
/// Post image lanes written: outcome tag, ref kind, ref index (23, 24, 25).
const OUTCOME_LANES: [usize; 3] = [L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2];
/// Pre control `Return { value }` ref lanes (`put_control`: o[1], o[2]).
const CONTROL_VALUE_REF: [usize; 2] = [L_CONTROL + 1, L_CONTROL + 2];

#[derive(Clone, Debug)]
pub struct C6AdminPublishBlock {
    pub instructions: Vec<Instruction>,
}

pub struct C6AdminPublishEmitter {
    scope: String,
    next_temp: usize,
    composed: Composed,
}

impl C6AdminPublishEmitter {
    /// Requires: nothing. Ensures: `scope` is non-empty, has no `%` and no
    /// `.c6kit.`; the composed emitters are created in that scope.
    /// Check: sibling convention (`c6_call_bind_step::new`).
    pub fn new(scope: &str) -> Result<Self, C6AdminPublishError> {
        todo!()
    }

    /// Requires: `inputs` allocated by `build_step_circuit`.
    /// Ensures: `rule` is supported and shapes check (else Err, no
    /// instructions); the returned block is the whole relation (`body`).
    /// Check: `owner_runtime.rs:3730` names the only rule this emits.
    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
        todo!()
    }

    /// Requires: `rule` supported, shapes checked.
    /// Ensures: calls every function below once, in order: fix_prefix,
    /// validate_pre, select_rule, write_outcome, preserve_image,
    /// preserve_data, preserve_helper, compose_canonical, disabled_identity.
    /// Check: `owner_runtime.rs` `step` P0 -> P1 order (3705-3733).
    fn body(
        &mut self,
        em: &mut Em<'_>,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<(), String> {
        todo!()
    }
}

/// Reads: enable, host_schema, carrier_version, pre/post META/PROG/CTX, all
/// IMAGE/DATA/HELPER lanes (ranges). Fixes: post META/PROG/CTX.
/// Requires: shapes checked.
/// Ensures: enable ∈ {0,1}; schema/carrier constants; every lane < 2^128;
/// post META/PROG/CTX = pre (ungated); META/CTX magic+version under enable.
/// Check: kit `prefix`; the owner never writes META/PROG/CTX in `step`.
fn fix_prefix(em: &mut Em<'_>, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre IMAGE header and frame slots, pre DATA header 3..7.
/// Requires: `fix_prefix` emitted. Ensures under enable: pre image root and
/// every active frame are representation-valid, inactive slots are zero.
/// Open: confirm the pre Running payload lanes 24..26 are zero and the
/// control value ref is in range (`c6_phys.rs:593` `cap_ref`); add a gate
/// here only if neither the dispatch owner nor the kit already checks it.
/// Check: `owner_runtime.rs:3700` (`PreImage::bind` admits the pre).
fn validate_pre(em: &mut Em<'_>, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre IMAGE (outcome tag, control tag, frame count), pre HELPER
/// (helper/result records), pre PROG/DATA via the dispatch owner.
/// Requires: `rule == KAdminPublishValue`.
/// Ensures under enable: outcome Running, helper and result absent, control
/// tag Return (1), depth 0, i.e. the owner selects exactly this rule.
/// Check: `owner_runtime.rs:3705` (P0), `:3709-3730` (P1 Return arm);
/// composed as `c6_dispatch_step` `path` `p1_value`.
fn select_rule(
    composed: &mut Composed,
    em: &mut Em<'_>,
    rule: RuleIdC,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    todo!()
}

/// Reads: pre IMAGE `CONTROL_VALUE_REF`. Fixes: post IMAGE `OUTCOME_LANES`.
/// Requires: `select_rule` emitted (control is `Return { value }`).
/// Ensures under enable: post[23] = OUTCOME_HALTED, post[24] = pre[3],
/// post[25] = pre[4] (`put_ref` kind, index).
/// Check: `owner_runtime.rs:3728` `img.outcome = OutcomeC::Halted { value }`;
/// encoding `c6_phys.rs:1297` `put_outcome`.
fn write_outcome(em: &mut Em<'_>, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre IMAGE. Fixes: post IMAGE every lane except `OUTCOME_LANES`
/// (control 2..19 incl. the Return value, origin, env, stack limit, fuel and
/// work lanes 26..32, alloc/active/seg lanes, frame count, all frame slots).
/// Requires: nothing beyond `fix_prefix`.
/// Ensures under enable: post = pre on all those lanes; charge is 0.
/// Check: `owner_runtime.rs:3726` `publish_event` (charge/alloc deltas 0);
/// P1 writes only `img.outcome`.
fn preserve_image(em: &mut Em<'_>, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre DATA. Fixes: post DATA (all lanes).
/// Ensures under enable: post DATA = pre DATA.
/// Check: `owner_runtime.rs:3709-3733` P1 performs no DATA write.
fn preserve_data(em: &mut Em<'_>, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre HELPER. Fixes: post HELPER (all lanes).
/// Ensures under enable: post HELPER = pre HELPER (absent helper/result
/// stay absent).
/// Check: `owner_runtime.rs:3710` (helper and result absent, untouched).
fn preserve_helper(em: &mut Em<'_>, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre/post META (bound), pre PROG/CTX, pre/post DATA, pre/post HELPER.
/// Requires: the retained `Composed`. Ensures under enable: every carrier is
/// canonical. Keep only the parts not already implied by preservation if
/// the instruction count allows dropping post DATA/HELPER (they equal pre).
/// Check: `owner_runtime.rs:3700` `PreImage::bind` (AB admission).
fn compose_canonical(
    composed: &mut Composed,
    em: &mut Em<'_>,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    todo!()
}

/// Reads: enable, pre/post IMAGE/DATA/HELPER. Fixes: all of them when off.
/// Ensures under enable = 0: post = pre on every IMAGE/DATA/HELPER lane.
/// Check: task statement (enable = 0 is identity); kit `disabled_identity`.
fn disabled_identity(em: &mut Em<'_>, inputs: &C6StepInputs) {
    todo!()
}

// 13 functions, 7 global invariants.
