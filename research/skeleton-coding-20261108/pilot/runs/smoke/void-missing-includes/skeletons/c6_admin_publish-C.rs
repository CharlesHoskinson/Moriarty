//! C6 admin publish step: the `k.admin.publish.value` row every
//! factorial and tree fold run fires last, as one per-rule relation
//! over the full C6 carriers.
//!
//! The circuit fixes its rule at build time and selects it with the
//! composed dispatch owner (P1 publication: outcome Running, helper
//! and result absent, control `Return { value }`, depth 0). The only
//! write is the outcome: `Halted { value }` (image lanes 23-25), with
//! the value reference copied from the preserved Return control.
//! Charge is 0 (publication fires before the P2 fuel refusal, so a
//! zero remaining-work pre-state is accepted); every other image,
//! DATA and helper lane is preserved exactly. Canonicality is
//! composed; enable = 0 is identity.
//!
//! SKELETON: bodies are `todo!()`.
//!
//! Global invariants:
//! - G1 (lane coverage): under enable = 1 every post IMAGE lane is either
//!   written by `write_halted_outcome` (lanes `L_OUTCOME..L_OUTCOME+3`) or
//!   equated to its pre lane by `preserve_unwritten`; every post DATA and
//!   HELPER lane is equated to its pre lane; post META/PROG/CTX are equated
//!   to pre unconditionally by `prefix_and_validate`. No post lane is free.
//! - G2 (disabled identity): with enable = 0 the only satisfying
//!   assignments have post = pre on every lane; every enabled-only
//!   constraint is gated by `enable` so it imposes nothing then.
//! - G3 (enable is boolean): `enable` is constrained to {0, 1} before any
//!   gate uses it, so gating by `enable` / `1 - enable` is exhaustive.
//! - G4 (single rule): the rule is a build-time constant; `emit` rejects
//!   every rule outside `ADMIN_PUBLISH_RULES` and emits no instruction.
//! - G5 (no charge): no fuel/work/cursor lane (27-31, 46) changes; the
//!   relation adds no fuel check, so remaining work 0 is accepted.
//! - G6 (lane ranges): every statement lane, pre and post, is range-checked
//!   to field bits so equalities are equalities of machine words.
//! - G7 (serial hygiene): all temporaries come from one `Em` whose counter
//!   is carried across `emit` calls; composed emitters keep their own
//!   retained serial namespaces, so repeated emission never collides.
//! - G8 (size): no constraint is duplicated beyond what the composed
//!   helpers already own (no re-check of dispatch-owned lanes).
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

/// Check: native_inventory.rs:121 / raw.rs:579 / shared.rs:510.
pub const ADMIN_PUBLISH_RULES: [RuleIdC; 1] = [RuleIdC::KAdminPublishValue];

/// Ensures: true exactly for the members of `ADMIN_PUBLISH_RULES`.
pub fn admin_publish_supported(rule: RuleIdC) -> bool {
    todo!()
}

// ---- Statement-lane stubs -------------------------------------------------
/// Post image lanes this relation writes: outcome tag, value ref (2 lanes).
/// Pre lanes read: none (written from constants / pre control).
const WRITE_OUTCOME_LANES: [usize; 3] = [L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2];
/// Pre image lanes the Halted value is copied from: `Return { value }`
/// payload in control lanes `L_CONTROL+1`, `L_CONTROL+2`.
const READ_RETURN_VALUE_LANES: [usize; 2] = [L_CONTROL + 1, L_CONTROL + 2];
/// `OutcomeC::Halted` tag in the outcome tag lane.
const OUTCOME_HALTED: u128 = 1;

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
    /// Ensures: `Err(InvalidScope)` for an empty/blank scope or one
    /// containing `%` or `.c6kit.`; otherwise a fresh emitter with temp
    /// counter 0 and a `Composed` built for `scope` (its error -> `Emit`).
    pub fn new(scope: &str) -> Result<Self, C6AdminPublishError> {
        todo!()
    }

    /// Requires: `inputs` comes from `c6_step_harness::build_step_circuit`.
    /// Ensures: `Err(Unsupported)` unless `admin_publish_supported(rule)`;
    /// `Err(Emit)` if `check_shapes` fails; otherwise the instructions of
    /// `body`, with the temp counter advanced and retained (G7).
    /// Check: owner_runtime.rs:3730 (the only publishing row fired).
    pub fn emit(
        &mut self,
        rule: RuleIdC,
        inputs: &C6StepInputs,
    ) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
        todo!()
    }

    /// Requires: `rule` supported, shapes checked.
    /// Ensures: emits, in order, `prefix_and_validate`, `select_publish`,
    /// `write_halted_outcome`, `preserve_unwritten`, `compose_canonicality`,
    /// `disabled_identity_guard`; the conjunction gives G1-G6.
    /// Check: owner_runtime.rs `step` (P1 publication before P2 fuel).
    fn body(&mut self, em: &mut Em, rule: RuleIdC, inputs: &C6StepInputs) -> Result<(), String> {
        todo!()
    }
}

/// Reads: enable, host_schema, carrier_version, all pre/post lanes;
/// pre_image header + frame slots, pre_data[3..7].
/// Requires: shapes checked.
/// Ensures: enable boolean; schema = C6_HOST_SCHEMA and carrier version =
/// C6_PADDED_CARRIER_VERSION; post META/PROG/CTX = pre (unconditional);
/// every lane is in range; under enable the pre image is well formed
/// (magic/version, root refs, counts below caps, ACTIVE matches DATA
/// header, depth <= stack limit, inactive frame slots zero).
/// Check: kit `prefix` + `validate_pre_image` (shared step preconditions).
fn prefix_and_validate(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: enable, pre_prog, pre_image, pre_data, pre_helper.
/// Requires: `prefix_and_validate` emitted (enable boolean).
/// Ensures: under enable, the composed dispatch owner proves the owner
/// machine selects exactly `rule` from the pre state: outcome Running,
/// helper and result absent, control `Return { value }`, frame count 0.
/// This relation does not re-check those lanes (G8).
/// Check: owner_runtime.rs:3724-3730 (`ControlC::Return { value } if
/// img.frames.is_empty()`), owner_runtime.rs:1784 (no family plan),
/// owner_runtime.rs:1345 (EventKind::Publish).
fn select_publish(
    composed: &mut Composed,
    em: &mut Em,
    rule: RuleIdC,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    todo!()
}

/// Reads: enable, pre_image[READ_RETURN_VALUE_LANES].
/// Fixes: post_image[WRITE_OUTCOME_LANES].
/// Requires: `select_publish` emitted (pre control is `Return { value }`).
/// Ensures: under enable, post outcome tag = OUTCOME_HALTED and the
/// post outcome value ref = pre control value ref, lane for lane.
/// Check: owner_runtime.rs:3725 (`img.outcome = OutcomeC::Halted { value }`).
fn write_halted_outcome(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: enable, every pre image lane except WRITE_OUTCOME_LANES, all
/// pre_data, all pre_helper.
/// Fixes: the corresponding post lanes (header, all frame slots, DATA,
/// HELPER).
/// Requires: enable boolean.
/// Ensures: under enable, post = pre on every image lane outside
/// WRITE_OUTCOME_LANES (incl. control, fuel/work/cursor lanes: charge 0,
/// G5) and on every DATA and HELPER lane.
/// Check: owner_runtime.rs:3730 (`publish_event` writes only the outcome).
fn preserve_unwritten(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// Reads: enable, pre/post META, pre PROG, pre CTX, pre/post DATA,
/// pre/post HELPER, geometry caps, source params.
/// Requires: enable boolean.
/// Ensures: under enable, META bounds, PROG/CTX and the CTX binding to
/// the source params, both DATA carriers and both HELPER carriers are
/// canonical (each emitted via its retained composed emitter).
/// Check: kit `compose_canonical` (owner snapshot encoding is canonical).
fn compose_canonicality(
    composed: &mut Composed,
    em: &mut Em,
    inputs: &C6StepInputs,
) -> Result<(), String> {
    todo!()
}

/// Reads: enable, all pre/post IMAGE, DATA, HELPER lanes.
/// Fixes: every post IMAGE/DATA/HELPER lane when enable = 0.
/// Requires: enable boolean.
/// Ensures: with enable = 0, post = pre on every IMAGE, DATA and HELPER
/// lane (META/PROG/CTX already equal by `prefix_and_validate`) (G2).
/// Check: task spec, "with enable = 0 it must accept only the identity".
fn disabled_identity_guard(em: &mut Em, inputs: &C6StepInputs) {
    todo!()
}

/// One-shot emission of the admin publish relation for `rule`.
/// Ensures: same as `C6AdminPublishEmitter::new(scope)?.emit(rule, inputs)`.
pub fn emit_c6_admin_publish_block(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<C6AdminPublishBlock, C6AdminPublishError> {
    todo!()
}

// Functions: 11 (4 public + body + 6 private helpers); global invariants: 8.
