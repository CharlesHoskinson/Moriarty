//! C6 admin publish step (SKELETON): the `k.admin.publish.value` row every
//! factorial and tree fold run fires last, as one per-rule relation over
//! the full C6 carriers.
//!
//! The rule is fixed at build time and selected through the composed
//! dispatch owner (P1 publication: outcome Running, helper and result
//! absent, control `Return { value }`, depth 0). The only write is the
//! outcome `Halted { value }` (image lanes 23-25 = `L_OUTCOME..+3`), with
//! the value reference copied from the preserved Return control
//! (`L_CONTROL+1`, `L_CONTROL+2`). Charge is 0: publication fires before
//! the P2 fuel refusal, so `remaining_work = 0` is accepted.
//!
//! Global invariants:
//! - G1 (frame): every post IMAGE, DATA and HELPER lane is either written
//!   by `write_halted_outcome` (only `L_OUTCOME..+3`) or constrained equal
//!   to its pre lane; post META is bound by `canonical`; no post lane the
//!   rule fixes is left free.
//! - G2 (disabled): with enable = 0, post = pre on every IMAGE, DATA and
//!   HELPER lane, and every enable-gated check is vacuous.
//! - G3 (enable boolean, header): enable ∈ {0,1}, host schema and carrier
//!   version are the fixed constants, regardless of enable.
//! - G4 (no fuel): no work/visit counter is charged; `L_REMAINING_WORK` and
//!   `L_CONSUMED_WORK` are preserved, and no lower bound on remaining work
//!   is imposed (P1 precedes P2).
//! - G5 (exact rule): with enable = 1 the accepted pairs are exactly the
//!   owner's `KAdminPublishValue` steps; selection plus G1 leave no other
//!   rule's post-state satisfiable.
//! - G6 (canonicality): the pre carrier, and post META/DATA/HELPER, are
//!   canonical encodings (composed canonical emitters; post IMAGE follows
//!   from preservation of a validated pre image), so a lane equality is a
//!   state equality.
//! - G7 (scoping): all temporaries are fresh names under `scope`; the temp
//!   counter is carried across `emit` calls on one emitter.
use midnight_zkir_v3::Instruction;
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

/// Ensures: true exactly for the members of `ADMIN_PUBLISH_RULES`.
pub fn admin_publish_supported(rule: RuleIdC) -> bool {
    todo!()
}

/// Outcome tag of `Halted { value }` in the image outcome lane.
const OUTCOME_HALTED: u128 = 1;

// ---- Statement-lane stubs (indices into `C6StepInputs` vectors) ----
// Written post lanes:  post_image[L_OUTCOME], [L_OUTCOME+1], [L_OUTCOME+2]
// Read pre lanes:      pre_image[L_CONTROL+1], pre_image[L_CONTROL+2]
// Selection reads:     pre_image outcome/helper/result/control/frame count
//                      (via composed dispatch owner)
// Preserved:           all other post_image lanes; all post_data; all
//                      post_helper. post_meta is bound by `canonical`;
//                      post_prog/ctx are owned by the harness statement.

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
    /// Requires: nothing.
    /// Ensures: `Err(InvalidScope)` if `scope` is blank, contains `%`, or
    /// contains `.c6kit.`; otherwise an emitter with temp counter 0 and a
    /// freshly built `Composed` for `scope` (its error mapped to `Emit`).
    /// Check: G7.
    pub fn new(scope: &str) -> Result<Self, C6AdminPublishError> {
        todo!()
    }

    /// Requires: `inputs` is the statement allocated by
    /// `c6_step_harness::build_step_circuit`.
    /// Ensures: `Err(Unsupported)` unless `admin_publish_supported(rule)`;
    /// `Err(Emit)` if `check_shapes` rejects `inputs`; otherwise the
    /// instructions of `body`, and `next_temp` advanced past every temp used.
    /// Check: owner_runtime.rs:3730 (the only rule this relation fires).
    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
        todo!()
    }

    /// Requires: `rule` supported; shapes checked.
    /// Ensures: the conjunction of `statement_prefix`, `select_publish`,
    /// `write_halted_outcome`, `preserve_unwritten_image`, `preserve_data`,
    /// `preserve_helper`, `canonical` and `identity_when_disabled`, in that
    /// order. Satisfies G1–G6 jointly.
    /// Check: owner_runtime.rs:3695-3735 (owner_step through P1 publish).
    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        todo!()
    }
}

/// Reads: enable, host_schema, carrier_version, pre_image (phys header).
/// Requires: shapes checked.
/// Ensures: enable ∈ {0,1}; host schema and carrier version equal the
/// fixed constants; when enabled, the pre image passes the kit's physical
/// validation (magic, bounds) — `prefix` then `validate_pre_image`.
/// Check: owner_runtime.rs:3701 (`PreImage::bind`, AB before any read).
fn statement_prefix(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre_image outcome, helper, result, control, frame-count lanes.
/// Requires: `rule == KAdminPublishValue`; `statement_prefix` emitted.
/// Ensures: when enabled, the composed dispatch owner selects exactly this
/// rule: pre outcome Running, helper absent, result absent, control tag
/// `Return`, frame depth 0 (via the kit `select`).
/// Check: owner_runtime.rs:3705 (P0), :3710 (helper/result absent),
/// :3724 (`Return { value }` with empty frames).
fn select_publish(
    composed: &mut Composed,
    em: &mut Em,
    rule: RuleIdC,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    todo!()
}

/// Reads: pre_image[L_CONTROL+1], pre_image[L_CONTROL+2].
/// Fixes: post_image[L_OUTCOME] = OUTCOME_HALTED,
/// post_image[L_OUTCOME+1] = pre_image[L_CONTROL+1],
/// post_image[L_OUTCOME+2] = pre_image[L_CONTROL+2] (all enable-gated).
/// Requires: selection emitted (pre control is `Return { value }`).
/// Ensures: when enabled, post outcome is `Halted { value }` with `value`
/// the pre Return value reference. Control lanes are not written here.
/// Check: owner_runtime.rs:3725 (`img.outcome = OutcomeC::Halted { value }`).
fn write_halted_outcome(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: every pre_image lane except L_OUTCOME..L_OUTCOME+3.
/// Fixes: the corresponding post_image lanes (frames immutable).
/// Requires: nothing beyond shapes.
/// Ensures: when enabled, post_image[i] = pre_image[i] for every i outside
/// the outcome triple, including control, remaining/consumed work and
/// frames (charge 0, G4). Uses `preserve_image` with mutable =
/// `[L_OUTCOME, L_OUTCOME+1, L_OUTCOME+2]`, `frames_mutable = false`.
/// Check: owner_runtime.rs:3724-3734 (no write other than outcome before
/// `publish_event`; return precedes the P2 fuel branch at :3739).
fn preserve_unwritten_image(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre_data. Fixes: post_data.
/// Requires: pre_data and post_data have equal length (check_shapes).
/// Ensures: when enabled, post_data[i] = pre_data[i] for every i.
/// Check: owner_runtime.rs:3725 (publication touches no DATA lane).
fn preserve_data(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre_helper. Fixes: post_helper.
/// Requires: pre_helper and post_helper have equal length (check_shapes).
/// Ensures: when enabled, post_helper[i] = pre_helper[i] for every i.
/// Check: owner_runtime.rs:3710 (helper absent; P1 does not install one).
fn preserve_helper(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: pre/post META, pre PROG, pre CTX, pre/post DATA, pre/post HELPER.
/// Requires: `select_publish` used the same `composed`.
/// Ensures (enable-gated, via the kit `compose_canonical`): pre/post META
/// canonical and bound to the source params; pre PROG+CTX canonical and CTX
/// equal to the expected context; pre and post DATA canonical; pre and
/// post HELPER canonical (G6). Post PROG/CTX are not read here.
/// Check: owner_runtime.rs:3708 (pre snapshot is a well-formed image).
fn canonical(composed: &mut Composed, em: &mut Em, inputs: &C6StepInputs) -> Result<(), String> {
    todo!()
}

/// Reads: enable, every pre lane. Fixes: every post lane when disabled.
/// Requires: nothing.
/// Ensures: enable = 0 implies post = pre on every IMAGE, DATA and HELPER
/// lane (G2), via the kit `disabled_identity`.
/// Check: task spec (enable = 0 is identity); no owner line.
fn identity_when_disabled(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// One-shot emission of the admin publish relation for `rule`.
/// Ensures: identical to `C6AdminPublishEmitter::new(scope)?.emit(rule, inputs)`.
pub fn emit_c6_admin_publish_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
    todo!()
}

// 13 functions, 7 global invariants.
