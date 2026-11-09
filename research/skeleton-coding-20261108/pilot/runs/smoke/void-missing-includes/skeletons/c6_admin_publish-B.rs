//! C6 admin publish step (SKELETON): the `k.admin.publish.value` row every
//! factorial and tree fold run fires last, as one per-rule relation over the
//! full C6 carriers (LocalPublic pre+post statement, no private inputs).
//!
//! The circuit fixes its rule at build time, selects it with the composed
//! dispatch owner (P1 publication: outcome Running, helper and result absent,
//! control `Return { value }`, depth 0), writes the outcome
//! `Halted { value }` (image lanes 23-25) with the value reference copied
//! from the preserved Return control, charges nothing and preserves every
//! other image, frame, DATA and helper lane. Canonicality of
//! META/PROG/CTX/DATA/HELPER is composed; enable = 0 is identity.
//!
//! Global invariants:
//! 1. Lane closure: under enable = 1 every post IMAGE/DATA/HELPER lane is
//!    either written by `write_halted_outcome` (exactly lanes 23..26) or
//!    fixed equal to its pre lane by `preserve_image_except_outcome` /
//!    `preserve_data_and_helper`; META/PROG/CTX post = pre unconditionally
//!    (`prefix_and_pre_validation`). No post lane is left free.
//! 2. Disabled identity: with enable = 0 every enabled-gated constraint is
//!    vacuous and `disabled_identity` forces post = pre on every lane; the
//!    ungated constraints (enable boolean, schema, carrier version, 128-bit
//!    ranges, META/PROG/CTX equality) hold for the honest identity pair.
//! 3. Rule exclusivity: with enable = 1 the accepted pairs are exactly the
//!    owner `step` results that publish `KAdminPublishValue` (owner_runtime.rs
//!    P1 branch, `Return { value } if img.frames.is_empty()`); no other rule,
//!    and no P2 fuel refusal, can be accepted by this circuit.
//! 4. Charge is zero: no fuel, work, helper-work, visit or cursor lane moves,
//!    and `remaining_work == 0` in the pre-state is NOT rejected (P1 precedes
//!    the P2 fuel check).
//! 5. Completeness: pre and post are encoded independently from the owner's
//!    snapshots, so every constraint compares encoded lanes, never a value
//!    re-derived from a different encoding path.
//! 6. Size: no constraint is emitted twice across functions (e.g. the
//!    selection already pins the control tag / depth / outcome tag; the writer
//!    must not re-check them), and the relation stays minimal.
//! 7. Temporaries stay in the emitter's scope namespace; `next_temp` is
//!    threaded across `emit` calls so repeated emission never collides.
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

// ---- Statement-lane stubs -------------------------------------------------
/// `encode_control`: `Return { value }` has tag 1 at control lane 0.
const CONTROL_RETURN_TAG: u128 = 1;
/// Pre control lanes holding the Return value reference (`put_ref` at 1).
const RETURN_VALUE_REF: std::ops::Range<usize> = L_CONTROL + 1..L_CONTROL + 3;
/// `put_outcome`: `Halted { value }` has tag 1, ref at `at + 1..at + 3`.
const OUTCOME_HALTED_TAG: u128 = 1;
/// The only post image lanes this rule writes (outcome tag + value ref).
const OUTCOME_LANES: std::ops::Range<usize> = L_OUTCOME..L_OUTCOME + 3;

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
    /// Requires: nothing. Ensures: `scope` is non-blank, contains no `%` and
    /// no `.c6kit.` (else `InvalidScope`); the composed canonical/dispatch
    /// emitters are built in `scope`. Check: c6_call_bind_step conventions.
    pub fn new(scope: &str) -> Result<Self, C6AdminPublishError> {
        todo!()
    }

    /// Requires: `inputs` is the statement allocated by
    /// `c6_step_harness::build_step_circuit`.
    /// Ensures: `rule` is in `ADMIN_PUBLISH_RULES` (else `Unsupported`),
    /// `check_shapes` passed, and the returned instructions encode exactly
    /// the relation of the module docs (all Global invariants); `next_temp`
    /// is advanced past every temporary used.
    /// Check: owner_runtime.rs:3730 (`RuleIdC::KAdminPublishValue` publish).
    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
        todo!()
    }

    /// Orchestration: calls every function below once, in order (prefix,
    /// selection, write, preservation, canonical, disabled identity).
    /// Requires: `rule` supported, shapes checked. Ensures: the conjunction
    /// of every callee's Ensures. Check: owner_runtime.rs `step` order
    /// (P0, P1 publish, return before P2).
    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        todo!()
    }

    /// Selection of exactly `KAdminPublishValue` on the pre carriers via the
    /// composed dispatch owner (`c6_step_kit::select`).
    /// Reads: pre_prog, pre_image (outcome 23..26, control 2..19, frame count
    /// 47, ...), pre_data, pre_helper (helper/result presence).
    /// Fixes: no post lane.
    /// Requires: enable boolean (from `prefix_and_pre_validation`).
    /// Ensures: enable = 1 implies pre outcome = Running (tag 0, lanes 24-25
    /// zero), helper and result absent, pre control tag =
    /// `CONTROL_RETURN_TAG`, frame count = 0, i.e. the owner fires this rule
    /// and no other. Does NOT require remaining_work > 0.
    /// Check: owner_runtime.rs:1784 / 1907 / 2046 (dispatch mapping) and
    /// owner_runtime.rs:3730 (P1 `Return { value } if frames.is_empty()`).
    fn select_publish_value(
        &mut self,
        em: &mut Em,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<(), String> {
        todo!()
    }
}

/// Ungated prefix (enable boolean, host schema, carrier version, 128-bit
/// lane ranges, META/PROG/CTX post = pre) and gated pre-image representation
/// validation (`c6_step_kit::prefix`, `validate_pre_image`).
/// Reads: enable, host_schema, carrier_version, all pre/post lanes.
/// Fixes: post META/PROG/CTX (= pre, unconditionally).
/// Requires: `check_shapes(inputs)` succeeded.
/// Ensures: enable in {0,1}; every lane is a 128-bit value; under enable the
/// pre image decodes as an owner root with valid frames (needed so the
/// Return value reference copied into the outcome is a valid reference).
/// Check: owner_runtime.rs `step` (`PreImage::bind` before any read).
fn prefix_and_pre_validation(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// The only write: post outcome = `Halted { value }`.
/// Reads: pre_image[RETURN_VALUE_REF].
/// Fixes: post_image[OUTCOME_LANES] = [OUTCOME_HALTED_TAG, ref0, ref1].
/// Requires: selection constrains the pre control to `Return { value }`.
/// Ensures: enable = 1 implies the post outcome lanes equal the encoding of
/// `OutcomeC::Halted { value }` with `value` from the pre control.
/// Check: owner_runtime.rs:3730 (`img.outcome = OutcomeC::Halted { value }`)
/// and owner_runtime.rs:1345 (EventKind::Publish).
fn write_halted_outcome(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Preservation of every image header lane outside `OUTCOME_LANES`
/// (including control 2..19, remaining/consumed/helper work, visits,
/// reference cursor, allocation and segment lanes, frame count) and of every
/// frame slot lane (`c6_step_kit::preserve_image(.., &OUTCOME_LANES, false)`).
/// Reads: pre_image. Fixes: post_image minus OUTCOME_LANES.
/// Requires: none beyond enable boolean.
/// Ensures: enable = 1 implies post_image[l] = pre_image[l] for every
/// l not in OUTCOME_LANES; in particular the charge is 0.
/// Check: owner_runtime.rs:3730 (publish_event writes only `img.outcome`;
/// reference_projection.rs:895 `KAdminPublishValue => (80, 0)`).
fn preserve_image_except_outcome(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Preservation of every DATA and HELPER lane.
/// Reads: pre_data, pre_helper. Fixes: post_data, post_helper.
/// Requires: none beyond enable boolean.
/// Ensures: enable = 1 implies post_data = pre_data, post_helper = pre_helper.
/// Check: owner_runtime.rs:3730 (P1 publication performs no DATA/helper write).
fn preserve_data_and_helper(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Composed META(bound)/PROG/CTX/DATA/HELPER canonicality on pre and post
/// (`c6_step_kit::compose_canonical`).
/// Reads: pre/post META, pre PROG/CTX, pre/post DATA and HELPER.
/// Fixes: no lane (canonical-form constraints only).
/// Requires: emitters in `composed` retained across calls.
/// Ensures: enable = 1 implies each carrier is the unique canonical encoding
/// of an owner snapshot, so (pre, post) determine owner states.
/// Check: owner_runtime.rs `step` (`PreImage::bind`), native_inventory.rs:121.
fn compose_canonicality(
    composed: &mut Composed,
    em: &mut Em,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    todo!()
}

/// Disabled identity (`c6_step_kit::disabled_identity`).
/// Reads: pre image/data/helper. Fixes: post image/data/helper under !enable.
/// Requires: enable boolean.
/// Ensures: enable = 0 implies post = pre on every IMAGE/DATA/HELPER lane
/// (META/PROG/CTX already equal by the prefix).
/// Check: task contract (enable = 0 accepts only the identity).
fn disabled_identity_pair(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

// 11 functions, 7 global invariants.
