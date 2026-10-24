//! C6 `BHelperValidateField` (`RuleIdC::BHelperValidateField`):
//! active `HelperControlC::Validate { common, root, status: TriStatus::Pending }`
//! (`pre_helper[L_HELPER_RECORD] == 7` and `pre_helper[L_HELPER_RECORD + 7] == 0`),
//! non-empty helper task stack (`pre.task_live() > 0`, i.e., `pre_helper[3] > 0`),
//! top task at slot `pre_helper[3] - 1` equal to
//! `HelperTaskC::ValidateTask { r, visit: TaskVisitC::Fields, cursor }`
//! (`top_task[0] == 0` and `top_task[3] == 1`), `OutcomeC::Running` (`[0, 0, 0]`),
//! and positive remaining fuel (`pre.remaining_work > 0`).
//!
//! Core selection and effect (`structural::plan_validate_nonentry` and
//! `owner_runtime::owner_step`):
//! * In `helper_group(HelperKind::Validate)`, the preceding rules are:
//!   - `BBookkeepingTaskRelease` (`status == Negative && task_live > 0`),
//!   - `BHelperValidateFailure` (`status == Negative && task_live == 0`),
//!   - `BHelperValidateExit` (`status == Positive`),
//!   - `BHelperValidateEmpty` (`status == Pending && task_live == 0`),
//!   - `BHelperValidateInvalid` (`status == Pending && top_task == ValidateTask { r, visit: Unvisited, .. } && kind_of(r) == Marker`), and
//!   - `BHelperValidateOccurrence` (`status == Pending && top_task == ValidateTask { r, visit: Unvisited, .. }`).
//!   Enforcing `status == TriStatus::Pending` (`0`), `task_live > 0`,
//!   `top_task.tag == ValidateTask` (`0`), and `top_task.visit == TaskVisitC::Fields` (`1`)
//!   uniquely selects `BHelperValidateField`.
//! * `selected_for(pre, BHelperValidateField)` invokes `owner_site(pre)`,
//!   which requires `pre.control` not to be `ControlC::Error` (tag `9`) and,
//!   when `pre.control` is `ControlC::Return` (tag `1`), requires
//!   `pre.depth() > 0` (`pre_image[L_FRAME_COUNT] > 0`). Both conditions are
//!   enforced here under `enable == 1`, along with full `decode_image_section`
//!   canonicality of `pre_image` across all 11 `ControlC` tags (`0..11`) and
//!   all 10 `FrameRowC` tags (`0..10`), plus `pre.control_origin < max_expressions`.
//! * PS child lookup `pre.field_at(&r, &cursor)`:
//!   - Top-task reference `r = (top_task[1], top_task[2])` is bound in-circuit
//!     through `C6CheckedValueReadEmitter` over `pre_prog` and `pre_data`
//!     (`reader.selected_found == 1` and `reader.is_marker == 0`).
//!   - `cursor < reader.slice_len` is enforced from the actual raw value slice
//!     of `r` (`const_fields(row)` or `data_fields(row)`), never from
//!     `checked_arity`.
//!   - `field_pos = reader.slice_start + cursor` is computed in 129 bits.
//!   - When `r` is `RefC::Const` (`top_ref_is_data == 0`), `field_pos` is
//!     bound against `prog.constant_fields` via an in-circuit 22-table
//!     `PROG` grammar walk (`PH_CONSTFIELD_COUNT` / `PH_CONSTFIELD_ROW`):
//!     `field_pos < const_field_count` and `const_field_found == 1`, yielding
//!     `child = RefC::Const(const_field_value)` (`child_ref_is_data == 0`,
//!     `child_ref_index == const_field_value`).
//!   - When `r` is `RefC::Data` (`top_ref_is_data == 1`), `field_pos` is
//!     bound against `img.data_fields` in `pre_data`:
//!     `field_pos < pre_data[4]` and `data_field_found == 1`, yielding
//!     `child = (pre_data[base], pre_data[base + 1])` preserving the stored
//!     `Const`/`Data` discriminator and source order.
//!   - Plan validation (`owner_validate_plan` / `SC-05` `check_written_code_ids`)
//!     requires every written `RefC::Const(id)` in `child_task` to satisfy
//!     `id < prog.constants.len()` (`const_count`) in PS before P3 and P6.
//!   - `child` may be a `RefC::Const` pointing to `LogicalKind::Marker`
//!     (`ConstantSnapshot::U128OutOfRange`): `BHelperValidateField` schedules
//!     the `Unvisited` child task without inspecting the child row or setting
//!     `TriStatus::Negative`. Later `BHelperValidateInvalid` is a separate rule.
//! * Checked arity (`pre.checked_arity(&pre.kind_of(&r)?)`) and guard
//!   `GuardId::GValidateFieldNontail` (`cursor.checked_add(1) < checked_arity`):
//!   - Dedicated kinds use `pure_leaves::arity_of`: `UInt`/`Bool`/`Nil` -> `0`,
//!     `Leaf` -> `1`, `Cons`/`Branch` -> `2`.
//!   - Generic `LogicalKind::Constructor { type_id, ctor_id }` (`kind_code == 6`)
//!     is bound in-circuit against the 22-table `PROG` declarations via
//!     `C6ProgramContextCanonicalEmitter::emit_prog_checked_sort_capture` and
//!     checked in exact `adt_checked_ctor_arity(view, type_id, ctor_id)` order
//!     (`nominal_valid == 1` under `enable == 1 && reader.is_constructor == 1`),
//!     setting `checked_arity` to the declared `ctor_row.field_len`
//!     (`nominal_capture.field_len`), NEVER `reader.slice_len`.
//!   - `GValidateFieldNontail` (`is_nontail`) holds iff `cursor + 1 < checked_arity`.
//! * Driver outcomes in exact Core `owner_step` order after PS selection,
//!   plan (`plan_validate_nonentry`), plan validation (`owner_validate_plan`),
//!   and physical check (`owner_physical_check`):
//!   - When `is_nontail == 1`, the plan rewrites the top task to
//!     `ValidateTask { r, visit: Fields, cursor: cursor + 1 }` and pushes
//!     `ValidateTask { r: child, visit: Unvisited, cursor: 0 }` (`task_live`
//!     delta `+1`), so `owner_cap_refusal` returns `Some(CapKind::HelperTasks)`
//!     iff `pre.task_live() + 1 > admission.bounds.helper_tasks`.
//!   - When `is_nontail == 0` (tail field), the plan replaces the top task in
//!     place with `ValidateTask { r: child, visit: Unvisited, cursor: 0 }`
//!     (`task_live` delta `0`), so `owner_cap_refusal` is `None` even when
//!     `pre.task_live() == admission.bounds.helper_tasks`.
//!   1. **P3 (`StepDisposition::RefusedHelperWorkUncharged`)**:
//!      `pre.helper_work == admission.bounds.max_helper_work` (from the
//!      separately retained original `source_params.admission_bounds()`,
//!      never layout bounds). P3 takes precedence over would-be P6.
//!      `post.outcome` becomes `OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }`
//!      (`[3, 5, 0]`), and all other `IMAGE`, `DATA`, and `HELPER` lanes
//!      (`remaining_work`, `consumed_work`, `helper_work`, `helper_visits`,
//!      `reference_cursor`, `control`, `control_origin`, `env_root`, frames,
//!      `helper`, `result`, and `tasks`) are preserved unchanged without any
//!      counter charge.
//!   2. **P6 (`StepDisposition::RefusedCapCharged` with `ExhaustReason::HelperTasks`)**:
//!      `pre.helper_work != admission.bounds.max_helper_work`, `is_nontail == 1`,
//!      and `pre.task_live() + 1 > admission.bounds.helper_tasks` (against the
//!      separately retained original `source_params.admission_bounds()`,
//!      never layout capacities).
//!      - P4 charges `remaining_work - 1`, `consumed_work + 1`, and
//!        `helper_work + 1` with checked 128-bit no-wrap assertions.
//!      - `helper_visits` and `reference_cursor` are UNCHANGED.
//!      - `post.outcome` becomes `OutcomeC::Exhausted { reason: ExhaustReason::HelperTasks }`
//!        (`[3, 4, 0]`).
//!      - `post_helper` (including `task_live`, all task slots, `helper`, and
//!        `result`) is preserved completely unchanged (no parent rewrite and
//!        no child push).
//!   3. **P7 (`StepDisposition::Applied`)**:
//!      `pre.helper_work != admission.bounds.max_helper_work` and not P6.
//!      - P4 charges `remaining_work - 1`, `consumed_work + 1`, and
//!        `helper_work + 1` with checked 128-bit no-wrap assertions.
//!      - P7 charges `helper_visits + 1` (`visits_of(BHelperValidateField) == 1`)
//!        with a checked 128-bit no-wrap assertion.
//!      - `reference_cursor` is unchanged (`EventKind::HelperVisit`).
//!      - `post.outcome` remains `OutcomeC::Running` (`[0, 0, 0]`).
//!      - In `post_helper`:
//!        * When `is_nontail == 1`: `post_helper[3] == pre_helper[3] + 1`,
//!          slot `pre_helper[3] - 1` is rewritten to
//!          `ValidateTask { r, visit: TaskVisitC::Fields, cursor: cursor + 1 }`
//!          (`[0, top_r_is_data, top_r_index, 1, cursor + 1, 0, 0]`), and
//!          slot `pre_helper[3]` is pushed with
//!          `ValidateTask { r: child, visit: TaskVisitC::Unvisited, cursor: 0 }`
//!          (`[0, child_is_data, child_index, 0, 0, 0, 0]`), preserving all
//!          lower task slots `< pre_helper[3] - 1` and keeping all higher
//!          inactive slots `> pre_helper[3]` at zero.
//!        * When `is_nontail == 0` (tail): `post_helper[3] == pre_helper[3]`
//!          and slot `pre_helper[3] - 1` is replaced in place with
//!          `ValidateTask { r: child, visit: TaskVisitC::Unvisited, cursor: 0 }`
//!          (`[0, child_is_data, child_index, 0, 0, 0, 0]`), preserving all
//!          other task slots unchanged.
//!
//! Statement disposition and derived receipt gating contract:
//! * `C6ValidateFieldStepInputs` carries two boolean statement disposition
//!   lanes: `disposition_is_refused_helper_work` (constrained under
//!   `enable == 1` to `is_p3`) and `disposition_is_refused_helper_tasks`
//!   (constrained under `enable == 1` to `is_p6`, which already embeds
//!   `!is_p3`). When `enable == 0`, both disposition lanes are only
//!   boolean-constrained and the disabled identity relation (`post == pre`
//!   across `META`, `PROG`, `CTX`, `image`, `DATA`, and `helper`) is enforced.
//! * Honest receipt gating classification on `C6ValidateFieldStepBlock`:
//!   - **Gated by `enable` (exact `0` when `enable == 0`)**:
//!     * Every field of `reader: C6CheckedValueReadBlock`.
//!     * `top_task_found`, `top_task_tag`, `top_ref_is_data`, `top_ref_index`,
//!       `top_task_visit`, `top_task_cursor`.
//!     * `field_pos`, `field_found`, `child_ref_is_data`, `child_ref_index`.
//!     * `nominal_valid` (gated by `enable && reader.is_constructor`).
//!     * `checked_arity`, `next_cursor`, and `is_nontail`.
//!   - **UNGATED derived receipts (must NEVER be treated as constrained
//!     post-state truth when `enable == 0`, `is_p3 == 1`, or `is_p6 == 1`)**:
//!     * Every field of `nominal_capture: C6ProgramContextCheckedSortCaptureBlock`.
//!     * `const_field_count`, `const_field_found`, `const_field_value`,
//!       `data_field_count`, `data_field_found`, `data_field_is_data`, and
//!       `data_field_index`.
//!     * `is_helper_work_max` (`is_p3`), `is_helper_tasks_refused` (`is_p6`),
//!       `is_applied` (`!is_p3 && !is_p6`), `post_task_count`,
//!       `post_top_task_tag`, `post_top_ref_is_data`, `post_top_ref_index`,
//!       `post_top_task_visit`, `post_top_task_cursor`, and
//!       `post_rewritten_parent_cursor`. Only the constrained `post_image`
//!       and `post_helper` carrier lanes in `instructions` represent the
//!       step relation's post-state.
//!
//! Retained caller premises NOT discharged by this single-row emitter:
//! * `source_params.admission_bounds()` equaling the admitted program's
//!   `Admitted.body.bounds` (and P1 metadata issuer alignment) is a caller /
//!   composition obligation.
//! * `PreImage::bind` / `snap` event-building lookups (`pre.occurrence(&control_origin)`
//!   and `pre.occurrence_path(&occurrence.path)`) are host `StepEventC`
//!   construction premises outside the C6 carrier state transition.
//! * `RefC::Data` parent and child references are accepted by bare
//!   `BHelperValidateField` when valid in `pre_data`, whereas genuine
//!   post-`BHelperValidateEntry` literal validation walks in admitted runs
//!   reach only `RefC::Const` tasks; local `RawData` acceptance does not
//!   claim genuine `SI21` reachability.
//! * Cross-row invariants `SI07`, `SI08`, `SI17`, `SI21`, `SI22`, `V6`
//!   constant DAG well-formedness (which equates raw constant slice length with
//!   checked arity on admitted constants), full `StepInv`, downstream
//!   `BHelperValidateEmpty` / `Invalid` / `Exit` / `Failure` and
//!   `KMachineEvalLiteralValid` / `Invalid` rules, program admission, and
//!   native proof issuance remain open obligations.

use midnight_transient_crypto::curve::{FR_BITS, Fr};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, C6PhysCaps, CONTROL_LANES, FRAME_LANES, L_ACTIVE, L_ALLOC,
    L_CONSUMED_WORK, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_FRAME_COUNT, L_HELPER_VISITS,
    L_HELPER_WORK, L_MAGIC, L_OUTCOME, L_REFERENCE_CURSOR, L_REMAINING_WORK, L_SEG_ACTIVE,
    L_SEG_ALLOC, L_STACK_LIMIT, L_VERSION,
};
use mori_core::sherlock::helper_machine::c6_phys_data::REF_LANES;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_FIXED_LANES, L_HELPER_RECORD, L_RESULT_RECORD, L_TASKS, TASK_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{C6_HOST_SCHEMA, CTX_LANES};
use mori_core::sherlock::helper_machine::c6_phys_program::PROGRAM_HEADER_LANES;
use thiserror::Error;

use super::c6_checked_value_read::{
    C6CheckedValueReadBlock, C6CheckedValueReadEmitter, C6CheckedValueReadInputs,
};
use super::c6_data_canonical::c6_data_geometry;
use super::c6_helper_canonical::{C6HelperCanonicalEmitter, C6HelperCanonicalInputs};
use super::c6_meta_canonical::{C6MetaBoundCanonicalInputs, C6MetaCanonicalEmitter};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextCanonicalInputs,
    C6ProgramContextCheckedSortCaptureBlock, C6ProgramContextCheckedSortCaptureInputs,
    C6ProgramContextCtxBindingInputs,
};
use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushStepError,
};

const LANE_BITS: u32 = 128;
const END_BITS: u32 = 129;
const CONTROL_VARIANT_COUNT: usize = 11;
const FRAME_VARIANT_COUNT: usize = 10;

const PH_MAGIC: u128 = 0;
const PH_VERSION: u128 = 1;
const PH_TAG: u128 = 2;
const PH_ENTRY: u128 = 3;
const PH_EXPR_COUNT: u128 = 4;
const PH_EXPR_ROW: u128 = 5;
const PH_OCC_COUNT: u128 = 6;
const PH_OCC_ROW: u128 = 7;
const PH_OCCPATH_COUNT: u128 = 8;
const PH_OCCPATH_ROW: u128 = 9;
const PH_CHILD_COUNT: u128 = 10;
const PH_CHILD_ROW: u128 = 11;
const PH_FUNC_COUNT: u128 = 12;
const PH_FUNC_ROW: u128 = 13;
const PH_PARAM_COUNT: u128 = 14;
const PH_PARAM_ROW: u128 = 15;
const PH_ARM_COUNT: u128 = 16;
const PH_ARM_ROW: u128 = 17;
const PH_PAT_COUNT: u128 = 18;
const PH_PAT_ROW: u128 = 19;
const PH_PATCHILD_COUNT: u128 = 20;
const PH_PATCHILD_ROW: u128 = 21;
const PH_TYPE_COUNT: u128 = 22;
const PH_TYPE_ROW: u128 = 23;
const PH_TYPEARG_COUNT: u128 = 24;
const PH_TYPEARG_ROW: u128 = 25;
const PH_CTOR_COUNT: u128 = 26;
const PH_CTOR_ROW: u128 = 27;
const PH_CTORFIELD_COUNT: u128 = 28;
const PH_CTORFIELD_ROW: u128 = 29;
const PH_DECPARAMCNT_COUNT: u128 = 30;
const PH_DECPARAMCNT_ROW: u128 = 31;
const PH_DECCTORCNT_COUNT: u128 = 32;
const PH_DECCTORCNT_ROW: u128 = 33;
const PH_DECL_COUNT: u128 = 34;
const PH_DECL_NAME: u128 = 35;
const PH_DECL_PARAMCNT: u128 = 36;
const PH_DECL_PARAM: u128 = 37;
const PH_DECL_CTORCNT: u128 = 38;
const PH_DECL_CTORNAME: u128 = 39;
const PH_DECL_FIELDCNT: u128 = 40;
const PH_DECL_FIELDNAME: u128 = 41;
const PH_TYPEEXPR: u128 = 42;
const PH_PLAN_COUNT: u128 = 43;
const PH_PLAN_ROW: u128 = 44;
const PH_PLANCASE_COUNT: u128 = 45;
const PH_PLANCASE_ROW: u128 = 46;
const PH_FIELDPATH_COUNT: u128 = 47;
const PH_FIELDPATH_ROW: u128 = 48;
const PH_CONST_COUNT: u128 = 49;
const PH_CONST_ROW: u128 = 50;
const PH_CONSTFIELD_COUNT: u128 = 51;
const PH_CONSTFIELD_ROW: u128 = 52;
const PH_NAMES_COUNT: u128 = 53;
const PH_NAMES_STR: u128 = 54;
const PH_END: u128 = 55;

#[derive(Debug, Error)]
pub enum C6ValidateFieldStepError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error("invalid scope identifier: scope must not contain '%' or a reserved composed segment")]
    InvalidScope,
    #[error("operand uses reserved parent or child temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow ({section})")]
    GeometryOverflow { section: &'static str },
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch {
        section: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ValidateFieldStepError {
    C6ValidateFieldStepError::Backend(e.to_string())
}

const COMPOSED_RESERVED_SEGMENTS: [&str; 10] = [
    ".local.temp.",
    ".c6data.temp.",
    ".c6helper.temp.",
    ".c6meta.temp.",
    ".c6progctx.temp.",
    ".fldread.local.temp.",
    ".fldread.c6progctx.temp.",
    ".fldread.c6data.temp.",
    ".fldnom.c6progctx.temp.",
    ".fldcf.c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ValidateFieldStepError> {
    if scope.trim().is_empty() {
        return Err(C6ValidateFieldStepError::EmptyScope);
    }
    if scope.contains('%')
        || scope.contains(".fldread")
        || scope.contains(".fldnom")
        || scope.contains(".fldcf")
        || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s))
    {
        return Err(C6ValidateFieldStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ValidateFieldStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ValidateFieldStepError::ReservedOperand {
                    identifier: id.0.clone(),
                });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct C6ValidateFieldStepInputs {
    pub enable: Operand,
    /// Statement lane constrained in-circuit when `enable == 1` to P3
    /// (`pre.helper_work == retained admission max_helper_work`,
    /// `StepDisposition::RefusedHelperWorkUncharged`).
    pub disposition_is_refused_helper_work: Operand,
    /// Statement lane constrained in-circuit when `enable == 1` to P6
    /// (`pre.helper_work != retained admission max_helper_work` AND
    /// `GValidateFieldNontail` (`cursor + 1 < checked_arity`) AND
    /// `pre.task_live + 1 > retained admission helper_tasks`,
    /// `StepDisposition::RefusedCapCharged` with `ExhaustReason::HelperTasks`).
    pub disposition_is_refused_helper_tasks: Operand,
    pub host_schema: Operand,
    pub carrier_version: Operand,
    pub geometry: C6PaddedCarrierGeometry,
    pub meta_bounds_frames_offset: usize,
    pub source_params: C6PushFamilyParams,
    pub pre_meta: Vec<Operand>,
    pub pre_prog: Vec<Operand>,
    pub pre_ctx: [Operand; CTX_LANES],
    pub pre_image: Vec<Operand>,
    pub pre_data: Vec<Operand>,
    pub pre_helper: Vec<Operand>,
    pub post_meta: Vec<Operand>,
    pub post_prog: Vec<Operand>,
    pub post_ctx: [Operand; CTX_LANES],
    pub post_image: Vec<Operand>,
    pub post_data: Vec<Operand>,
    pub post_helper: Vec<Operand>,
}

/// Denotation receipt for `BHelperValidateField` consumers.
///
/// Receipt gating contract:
/// * `reader`: every field is gated by `enable` (`0` when `enable == 0`) inside
///   `C6CheckedValueReadEmitter::emit`.
/// * `nominal_capture`: every field (`type_count`, `type_found`, `ctor_found`,
///   `owning_type`, `ctor_start`, `ctor_len`, `field_start`, `field_len`,
///   `ctor_field_count`) is an UNGATED output of
///   `C6ProgramContextCanonicalEmitter::emit_prog_checked_sort_capture`.
/// * `const_field_count`, `const_field_found`, `const_field_value`,
///   `data_field_count`, `data_field_found`, `data_field_is_data`, and
///   `data_field_index` are UNGATED table-capture wires over `pre_prog` and
///   `pre_data`.
/// * `top_task_found`, `top_task_tag`, `top_ref_is_data`, `top_ref_index`,
///   `top_task_visit`, `top_task_cursor`, `field_pos`, `field_found`,
///   `child_ref_is_data`, `child_ref_index`, `nominal_valid`, `checked_arity`,
///   `next_cursor`, and `is_nontail` are explicitly gated by `enable`
///   (`0` when `enable == 0`; `nominal_valid` is additionally gated by
///   `reader.is_constructor`).
/// * `is_helper_work_max` (`is_p3`), `is_helper_tasks_refused` (`is_p6`),
///   `is_applied` (`!is_p3 && !is_p6`), `post_task_count`,
///   `post_top_task_tag`, `post_top_ref_is_data`, `post_top_ref_index`,
///   `post_top_task_visit`, `post_top_task_cursor`, and
///   `post_rewritten_parent_cursor` are UNGATED derived candidate/driver wires.
///   When `enable == 0`, `is_p3 == 1`, or `is_p6 == 1`, `post_helper` is
///   constrained equal to `pre_helper` in `instructions` even though candidate
///   `post_top_*` wires reflect the unapplied child/rewrite values. Consumers
///   must rely only on the constrained `post_image` and `post_helper` carrier
///   lanes in `instructions`.
#[derive(Clone, Debug)]
pub struct C6ValidateFieldStepBlock {
    pub instructions: Vec<Instruction>,
    pub reader: C6CheckedValueReadBlock,
    pub nominal_capture: C6ProgramContextCheckedSortCaptureBlock,
    pub const_field_count: Operand,
    pub const_field_found: Operand,
    pub const_field_value: Operand,
    pub data_field_count: Operand,
    pub data_field_found: Operand,
    pub data_field_is_data: Operand,
    pub data_field_index: Operand,
    pub top_task_found: Operand,
    pub top_task_tag: Operand,
    pub top_ref_is_data: Operand,
    pub top_ref_index: Operand,
    pub top_task_visit: Operand,
    pub top_task_cursor: Operand,
    pub field_pos: Operand,
    pub field_found: Operand,
    pub child_ref_is_data: Operand,
    pub child_ref_index: Operand,
    pub nominal_valid: Operand,
    pub checked_arity: Operand,
    pub next_cursor: Operand,
    pub is_nontail: Operand,
    pub is_helper_work_max: Operand,
    pub is_helper_tasks_refused: Operand,
    pub is_applied: Operand,
    pub post_task_count: Operand,
    pub post_top_task_tag: Operand,
    pub post_top_ref_is_data: Operand,
    pub post_top_ref_index: Operand,
    pub post_top_task_visit: Operand,
    pub post_top_task_cursor: Operand,
    pub post_rewritten_parent_cursor: Operand,
}

#[derive(Clone)]
struct CfRegs {
    phase: Operand,
    sub: Operand,
    hold: Operand,
    rem: Operand,
    cnt: Operand,
    cnt2: Operand,
    cnt3: Operand,
    te_need: Operand,
    srem: Operand,
    const_count: Operand,
    const_field_count: Operand,
    const_field_row_index: Operand,
    const_field_found: Operand,
    const_field_value: Operand,
}

struct CfWalk<'a> {
    scope: &'a str,
    next_temp: usize,
    instructions: Vec<Instruction>,
}

impl<'a> CfWalk<'a> {
    fn new(scope: &'a str, next_temp: usize) -> Self {
        Self {
            scope,
            next_temp,
            instructions: Vec::new(),
        }
    }
    fn temp(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.c6progctx.temp.{}", self.scope, self.next_temp));
        self.next_temp += 1;
        id
    }
    fn imm(v: u128) -> Operand {
        Operand::Immediate(Fr::from(v))
    }
    fn ceq(&mut self, a: Operand, b: Operand) {
        self.instructions.push(Instruction::ConstrainEq { a, b });
    }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::TestEq {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn eqv(&mut self, a: Operand, v: u128) -> Operand {
        self.teq(a, Self::imm(v))
    }
    fn lt128(&mut self, a: Operand, v: u128) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::LessThan {
            a,
            b: Self::imm(v),
            bits: LANE_BITS,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::Add {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn add_i(&mut self, a: Operand, d: i128) -> Operand {
        self.add(a, Operand::Immediate(Fr::from(d)))
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::Mul {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::Not {
            a,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn and(&mut self, a: Operand, b: Operand) -> Operand {
        self.mul(a, b)
    }
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a);
        let nb = self.not(b);
        let both = self.mul(na, nb);
        self.not(both)
    }
    fn gated_eq(&mut self, gate: Operand, a: Operand, b: Operand) {
        let s = self.sel(gate, a, b.clone());
        self.ceq(s, b);
    }
    fn count_step(
        &mut self,
        lane: &Operand,
        org: &CfRegs,
        nxt: &mut CfRegs,
        cur: u128,
        row: u128,
        zero_next: u128,
    ) {
        let cond = self.eqv(org.phase.clone(), cur);
        let is_zero = self.eqv(lane.clone(), 0);
        let chosen = self.sel(is_zero, Self::imm(zero_next), Self::imm(row));
        nxt.phase = self.sel(cond.clone(), chosen, nxt.phase.clone());
        nxt.rem = self.sel(cond.clone(), lane.clone(), nxt.rem.clone());
        nxt.sub = self.sel(cond, Self::imm(0), nxt.sub.clone());
    }
    fn fixed_step(
        &mut self,
        org: &CfRegs,
        nxt: &mut CfRegs,
        cur: u128,
        width: u128,
        next_table: u128,
    ) {
        let cond = self.eqv(org.phase.clone(), cur);
        let wm1 = width - 1;
        let is_last = self.eqv(org.sub.clone(), wm1);
        let done = self.and(cond.clone(), is_last.clone());
        let dec = self.add_i(org.rem.clone(), -1);
        let dec_zero = self.eqv(dec.clone(), 0);
        let chosen = self.sel(dec_zero, Self::imm(next_table), Self::imm(cur));
        nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
        let inc = self.add_i(org.sub.clone(), 1);
        let sub_row = self.sel(is_last, Self::imm(0), inc);
        nxt.sub = self.sel(cond, sub_row, nxt.sub.clone());
        nxt.rem = self.sel(done, dec, nxt.rem.clone());
    }
    fn var_step(
        &mut self,
        org: &CfRegs,
        nxt: &mut CfRegs,
        cur: u128,
        used: Operand,
        next_table: u128,
    ) {
        let cond = self.eqv(org.phase.clone(), cur);
        let used_m1 = self.add_i(used, -1);
        let is_last = self.teq(org.sub.clone(), used_m1);
        let done = self.and(cond.clone(), is_last.clone());
        let dec = self.add_i(org.rem.clone(), -1);
        let dec_zero = self.eqv(dec.clone(), 0);
        let chosen = self.sel(dec_zero, Self::imm(next_table), Self::imm(cur));
        nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
        let inc = self.add_i(org.sub.clone(), 1);
        let sub_row = self.sel(is_last, Self::imm(0), inc);
        nxt.sub = self.sel(cond, sub_row, nxt.sub.clone());
        nxt.rem = self.sel(done, dec, nxt.rem.clone());
    }
    fn used_from_hold(&mut self, org: &CfRegs, table: &[u128]) -> Operand {
        let mut used = Self::imm(table[0]);
        for (tag, width) in table.iter().enumerate().skip(1) {
            let is_tag = self.eqv(org.hold.clone(), tag as u128);
            used = self.sel(is_tag, Self::imm(*width), used);
        }
        used
    }
    fn string_step(&mut self, lane: Operand, org: &CfRegs, nxt: &mut CfRegs, cur: u128) -> Operand {
        let cond = self.eqv(org.phase.clone(), cur);
        let is_len = self.eqv(org.sub.clone(), 0);
        let is_byte = self.eqv(org.sub.clone(), 1);
        let cond_len = self.and(cond.clone(), is_len);
        let cond_byte = self.and(cond.clone(), is_byte);
        nxt.srem = self.sel(cond_len.clone(), lane.clone(), nxt.srem.clone());
        let lane_zero = self.eqv(lane, 0);
        let len_done = self.and(cond_len.clone(), lane_zero.clone());
        let not_zero = self.not(lane_zero);
        let go_bytes = self.and(cond_len, not_zero);
        let mut next_sub = nxt.sub.clone();
        next_sub = self.sel(go_bytes, Self::imm(1), next_sub);
        next_sub = self.sel(len_done.clone(), Self::imm(0), next_sub);
        let is_last = self.eqv(org.srem.clone(), 1);
        let byte_done = self.and(cond_byte.clone(), is_last);
        let dec = self.add_i(org.srem.clone(), -1);
        nxt.srem = self.sel(cond_byte, dec, nxt.srem.clone());
        next_sub = self.sel(byte_done.clone(), Self::imm(0), next_sub);
        nxt.sub = self.sel(cond, next_sub, nxt.sub.clone());
        self.or(len_done, byte_done)
    }
    fn after_field_phase(&mut self, org: &CfRegs) -> Operand {
        let d3 = self.add_i(org.cnt3.clone(), -1);
        let z3 = self.eqv(d3, 0);
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2, 0);
        let dr = self.add_i(org.rem.clone(), -1);
        let zr = self.eqv(dr, 0);
        let after_decl = self.sel(zr, Self::imm(PH_PLAN_COUNT), Self::imm(PH_DECL_NAME));
        let after_ctor = self.sel(z2, after_decl, Self::imm(PH_DECL_CTORNAME));
        self.sel(z3, after_ctor, Self::imm(PH_DECL_FIELDNAME))
    }
    fn apply_after_field(&mut self, org: &CfRegs, nxt: &mut CfRegs, event: Operand) {
        let p = self.after_field_phase(org);
        nxt.phase = self.sel(event.clone(), p, nxt.phase.clone());
        let d3 = self.add_i(org.cnt3.clone(), -1);
        let z3 = self.eqv(d3.clone(), 0);
        nxt.cnt3 = self.sel(event.clone(), d3, nxt.cnt3.clone());
        let ev2 = self.and(event.clone(), z3);
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2.clone(), 0);
        nxt.cnt2 = self.sel(ev2.clone(), d2, nxt.cnt2.clone());
        let evr = self.and(ev2, z2);
        let dr = self.add_i(org.rem.clone(), -1);
        nxt.rem = self.sel(evr, dr, nxt.rem.clone());
    }
    fn after_ctor_phase(&mut self, org: &CfRegs) -> Operand {
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2, 0);
        let dr = self.add_i(org.rem.clone(), -1);
        let zr = self.eqv(dr, 0);
        let after_decl = self.sel(zr, Self::imm(PH_PLAN_COUNT), Self::imm(PH_DECL_NAME));
        self.sel(z2, after_decl, Self::imm(PH_DECL_CTORNAME))
    }
    fn apply_after_ctor(&mut self, org: &CfRegs, nxt: &mut CfRegs, event: Operand) {
        let p = self.after_ctor_phase(org);
        nxt.phase = self.sel(event.clone(), p, nxt.phase.clone());
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2.clone(), 0);
        nxt.cnt2 = self.sel(event.clone(), d2, nxt.cnt2.clone());
        let evr = self.and(event, z2);
        let dr = self.add_i(org.rem.clone(), -1);
        nxt.rem = self.sel(evr, dr, nxt.rem.clone());
    }
    fn after_decl_phase(&mut self, org: &CfRegs) -> Operand {
        let dr = self.add_i(org.rem.clone(), -1);
        let zr = self.eqv(dr, 0);
        self.sel(zr, Self::imm(PH_PLAN_COUNT), Self::imm(PH_DECL_NAME))
    }
    fn apply_after_decl(&mut self, org: &CfRegs, nxt: &mut CfRegs, event: Operand) {
        let p = self.after_decl_phase(org);
        nxt.phase = self.sel(event.clone(), p, nxt.phase.clone());
        let dr = self.add_i(org.rem.clone(), -1);
        nxt.rem = self.sel(event, dr, nxt.rem.clone());
    }
    fn step(
        &mut self,
        lane: Operand,
        org: CfRegs,
        caps: &C6PhysCaps,
        selected_field_pos: &Operand,
    ) -> CfRegs {
        let mut nxt = org.clone();
        for cur in [PH_MAGIC, PH_VERSION, PH_TAG, PH_ENTRY] {
            let cond = self.eqv(org.phase.clone(), cur);
            nxt.phase = self.sel(cond, Self::imm(cur + 1), nxt.phase.clone());
        }
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_EXPR_COUNT,
            PH_EXPR_ROW,
            PH_OCC_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_OCC_COUNT,
            PH_OCC_ROW,
            PH_OCCPATH_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_OCCPATH_COUNT,
            PH_OCCPATH_ROW,
            PH_CHILD_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_CHILD_COUNT,
            PH_CHILD_ROW,
            PH_FUNC_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_FUNC_COUNT,
            PH_FUNC_ROW,
            PH_PARAM_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_PARAM_COUNT,
            PH_PARAM_ROW,
            PH_ARM_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_ARM_COUNT,
            PH_ARM_ROW,
            PH_PAT_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_PAT_COUNT,
            PH_PAT_ROW,
            PH_PATCHILD_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_PATCHILD_COUNT,
            PH_PATCHILD_ROW,
            PH_TYPE_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_TYPE_COUNT,
            PH_TYPE_ROW,
            PH_TYPEARG_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_TYPEARG_COUNT,
            PH_TYPEARG_ROW,
            PH_CTOR_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_CTOR_COUNT,
            PH_CTOR_ROW,
            PH_CTORFIELD_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_CTORFIELD_COUNT,
            PH_CTORFIELD_ROW,
            PH_DECPARAMCNT_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_DECPARAMCNT_COUNT,
            PH_DECPARAMCNT_ROW,
            PH_DECCTORCNT_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_DECCTORCNT_COUNT,
            PH_DECCTORCNT_ROW,
            PH_DECL_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_DECL_COUNT,
            PH_DECL_NAME,
            PH_PLAN_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_PLAN_COUNT,
            PH_PLAN_ROW,
            PH_PLANCASE_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_PLANCASE_COUNT,
            PH_PLANCASE_ROW,
            PH_FIELDPATH_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_FIELDPATH_COUNT,
            PH_FIELDPATH_ROW,
            PH_CONST_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_CONST_COUNT,
            PH_CONST_ROW,
            PH_CONSTFIELD_COUNT,
        );
        self.count_step(
            &lane,
            &org,
            &mut nxt,
            PH_CONSTFIELD_COUNT,
            PH_CONSTFIELD_ROW,
            PH_NAMES_COUNT,
        );
        self.count_step(&lane, &org, &mut nxt, PH_NAMES_COUNT, PH_NAMES_STR, PH_END);

        for cur in [PH_EXPR_ROW, PH_PAT_ROW, PH_PLAN_ROW, PH_CONST_ROW] {
            let row = self.eqv(org.phase.clone(), cur);
            let s0 = self.eqv(org.sub.clone(), 0);
            let cond = self.and(row, s0);
            nxt.hold = self.sel(cond, lane.clone(), nxt.hold.clone());
        }
        let used = self.used_from_hold(&org, &[2, 2, 4, 4, 3, 4, 4, 6, 7]);
        self.var_step(&org, &mut nxt, PH_EXPR_ROW, used, PH_OCC_COUNT);
        let used = self.used_from_hold(&org, &[2, 1, 3, 2, 3, 1, 5]);
        self.var_step(&org, &mut nxt, PH_PAT_ROW, used, PH_PATCHILD_COUNT);
        let used = self.used_from_hold(&org, &[8, 6, 3, 2]);
        self.var_step(&org, &mut nxt, PH_PLAN_ROW, used, PH_PLANCASE_COUNT);
        let used = self.used_from_hold(&org, &[3, 3, 2, 4, 4, 4, 6, 2]);
        self.var_step(&org, &mut nxt, PH_CONST_ROW, used, PH_CONSTFIELD_COUNT);

        self.fixed_step(&org, &mut nxt, PH_OCC_ROW, 4, PH_OCCPATH_COUNT);
        self.fixed_step(&org, &mut nxt, PH_OCCPATH_ROW, 1, PH_CHILD_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CHILD_ROW, 1, PH_FUNC_COUNT);
        self.fixed_step(&org, &mut nxt, PH_FUNC_ROW, 5, PH_PARAM_COUNT);
        self.fixed_step(&org, &mut nxt, PH_PARAM_ROW, 2, PH_ARM_COUNT);
        self.fixed_step(&org, &mut nxt, PH_ARM_ROW, 2, PH_PAT_COUNT);
        self.fixed_step(&org, &mut nxt, PH_PATCHILD_ROW, 1, PH_TYPE_COUNT);
        self.fixed_step(&org, &mut nxt, PH_TYPE_ROW, 7, PH_TYPEARG_COUNT);
        self.fixed_step(&org, &mut nxt, PH_TYPEARG_ROW, 1, PH_CTOR_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CTOR_ROW, 5, PH_CTORFIELD_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CTORFIELD_ROW, 2, PH_DECPARAMCNT_COUNT);
        self.fixed_step(&org, &mut nxt, PH_DECPARAMCNT_ROW, 1, PH_DECCTORCNT_COUNT);
        self.fixed_step(&org, &mut nxt, PH_DECCTORCNT_ROW, 1, PH_DECL_COUNT);
        self.fixed_step(&org, &mut nxt, PH_PLANCASE_ROW, 2, PH_FIELDPATH_COUNT);
        self.fixed_step(&org, &mut nxt, PH_FIELDPATH_ROW, 1, PH_CONST_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CONSTFIELD_ROW, 1, PH_NAMES_COUNT);

        let done = self.string_step(lane.clone(), &org, &mut nxt, PH_DECL_NAME);
        nxt.phase = self.sel(done, Self::imm(PH_DECL_PARAMCNT), nxt.phase.clone());
        {
            let cond = self.eqv(org.phase.clone(), PH_DECL_PARAMCNT);
            nxt.cnt = self.sel(cond.clone(), lane.clone(), nxt.cnt.clone());
            let z = self.eqv(lane.clone(), 0);
            let chosen = self.sel(z, Self::imm(PH_DECL_CTORCNT), Self::imm(PH_DECL_PARAM));
            nxt.phase = self.sel(cond, chosen, nxt.phase.clone());
        }
        let done = self.string_step(lane.clone(), &org, &mut nxt, PH_DECL_PARAM);
        {
            let dec = self.add_i(org.cnt.clone(), -1);
            let z = self.eqv(dec.clone(), 0);
            let chosen = self.sel(z, Self::imm(PH_DECL_CTORCNT), Self::imm(PH_DECL_PARAM));
            nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
            nxt.cnt = self.sel(done, dec, nxt.cnt.clone());
        }
        {
            let cond = self.eqv(org.phase.clone(), PH_DECL_CTORCNT);
            nxt.cnt2 = self.sel(cond.clone(), lane.clone(), nxt.cnt2.clone());
            let z = self.eqv(lane.clone(), 0);
            let nz = self.not(z);
            let go = self.and(cond.clone(), nz);
            nxt.phase = self.sel(go, Self::imm(PH_DECL_CTORNAME), nxt.phase.clone());
            let z2 = self.eqv(lane.clone(), 0);
            let ev = self.and(cond, z2);
            self.apply_after_decl(&org, &mut nxt, ev);
        }
        let done = self.string_step(lane.clone(), &org, &mut nxt, PH_DECL_CTORNAME);
        nxt.phase = self.sel(done, Self::imm(PH_DECL_FIELDCNT), nxt.phase.clone());
        {
            let cond = self.eqv(org.phase.clone(), PH_DECL_FIELDCNT);
            nxt.cnt3 = self.sel(cond.clone(), lane.clone(), nxt.cnt3.clone());
            let z = self.eqv(lane.clone(), 0);
            let nz = self.not(z);
            let go = self.and(cond.clone(), nz);
            nxt.phase = self.sel(go, Self::imm(PH_DECL_FIELDNAME), nxt.phase.clone());
            let z2 = self.eqv(lane.clone(), 0);
            let ev = self.and(cond, z2);
            self.apply_after_ctor(&org, &mut nxt, ev);
        }
        let done = self.string_step(lane.clone(), &org, &mut nxt, PH_DECL_FIELDNAME);
        nxt.phase = self.sel(done.clone(), Self::imm(PH_TYPEEXPR), nxt.phase.clone());
        nxt.te_need = self.sel(done.clone(), Self::imm(1), nxt.te_need.clone());
        nxt.sub = self.sel(done, Self::imm(0), nxt.sub.clone());
        {
            let cond = self.eqv(org.phase.clone(), PH_TYPEEXPR);
            let s0 = self.eqv(org.sub.clone(), 0);
            let tag_cond = self.and(cond.clone(), s0);
            nxt.hold = self.sel(tag_cond.clone(), lane.clone(), nxt.hold.clone());
            let is_leaf = self.lt128(lane.clone(), 4);
            let t4 = self.eqv(lane.clone(), 4);
            let t5 = self.eqv(lane.clone(), 5);
            let named = self.or(t4, t5);
            let apply = self.eqv(lane.clone(), 6);
            let need_dec = self.add_i(org.te_need.clone(), -1);
            let leaf_zero = self.eqv(need_dec.clone(), 0);
            let leaf_mid = self.and(is_leaf.clone(), leaf_zero);
            let leaf_event = self.and(tag_cond.clone(), leaf_mid);
            let go_str = self.or(named, apply);
            let go_cond = self.and(tag_cond.clone(), go_str);
            nxt.sub = self.sel(go_cond, Self::imm(1), nxt.sub.clone());
            let leaf_cond = self.and(tag_cond, is_leaf);
            nxt.te_need = self.sel(leaf_cond, need_dec, nxt.te_need.clone());
            self.apply_after_field(&org, &mut nxt, leaf_event);

            let es1 = self.eqv(org.sub.clone(), 1);
            let len_cond = self.and(cond.clone(), es1);
            nxt.srem = self.sel(len_cond.clone(), lane.clone(), nxt.srem.clone());
            let lz = self.eqv(lane.clone(), 0);
            let lnz = self.not(lz.clone());
            let go_bytes = self.and(len_cond.clone(), lnz);
            nxt.sub = self.sel(go_bytes, Self::imm(2), nxt.sub.clone());
            let str_done_now = self.and(len_cond, lz);

            let es2 = self.eqv(org.sub.clone(), 2);
            let byte_cond = self.and(cond.clone(), es2);
            let lastb = self.eqv(org.srem.clone(), 1);
            let byte_done = self.and(byte_cond.clone(), lastb);
            let sdec = self.add_i(org.srem.clone(), -1);
            nxt.srem = self.sel(byte_cond, sdec, nxt.srem.clone());
            let any_done = self.or(str_done_now, byte_done);
            let hold_apply = self.eqv(org.hold.clone(), 6);
            let to_arity = self.and(any_done.clone(), hold_apply.clone());
            nxt.sub = self.sel(to_arity, Self::imm(3), nxt.sub.clone());
            let not_apply = self.not(hold_apply);
            let named_done = self.and(any_done, not_apply);
            let nd = self.add_i(org.te_need.clone(), -1);
            let nz2 = self.eqv(nd.clone(), 0);
            let named_event = self.and(named_done.clone(), nz2);
            nxt.te_need = self.sel(named_done.clone(), nd, nxt.te_need.clone());
            nxt.sub = self.sel(named_done, Self::imm(0), nxt.sub.clone());
            self.apply_after_field(&org, &mut nxt, named_event);

            let es3 = self.eqv(org.sub.clone(), 3);
            let arity_cond = self.and(cond, es3);
            let bound = (caps.max_type_expr_nodes as u128) + 1;
            let within = self.lt128(lane.clone(), bound);
            let capped = self.sel(within, lane.clone(), Self::imm(bound));
            let base = self.add_i(org.te_need.clone(), -1);
            let raw_need = self.add(base, capped);
            let te_zero = self.eqv(org.te_need.clone(), 0);
            let safe_raw = self.sel(te_zero, Self::imm(bound), raw_need.clone());
            let rwithin = self.lt128(safe_raw, bound);
            let new_need = self.sel(rwithin, raw_need, Self::imm(bound));
            let az = self.eqv(new_need.clone(), 0);
            let arity_event = self.and(arity_cond.clone(), az);
            nxt.te_need = self.sel(arity_cond.clone(), new_need, nxt.te_need.clone());
            nxt.sub = self.sel(arity_cond, Self::imm(0), nxt.sub.clone());
            self.apply_after_field(&org, &mut nxt, arity_event);
        }
        let done = self.string_step(lane.clone(), &org, &mut nxt, PH_NAMES_STR);
        {
            let dec = self.add_i(org.rem.clone(), -1);
            let z = self.eqv(dec.clone(), 0);
            let chosen = self.sel(z, Self::imm(PH_END), Self::imm(PH_NAMES_STR));
            nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
            nxt.rem = self.sel(done, dec, nxt.rem.clone());
        }

        // Capture `constants.len()` (`PH_CONST_COUNT = 49`), `constant_fields.len()`
        // (`PH_CONSTFIELD_COUNT = 51`), and `constant_fields[selected_field_pos]`
        // (`PH_CONSTFIELD_ROW = 52`).
        let is_ccount = self.eqv(org.phase.clone(), PH_CONST_COUNT);
        nxt.const_count = self.sel(is_ccount, lane.clone(), nxt.const_count.clone());
        let is_cf_count = self.eqv(org.phase.clone(), PH_CONSTFIELD_COUNT);
        nxt.const_field_count = self.sel(is_cf_count, lane.clone(), nxt.const_field_count.clone());
        let is_cf_row = self.eqv(org.phase.clone(), PH_CONSTFIELD_ROW);
        let idx_eq = self.teq(
            org.const_field_row_index.clone(),
            selected_field_pos.clone(),
        );
        let cf_hit = self.and(is_cf_row.clone(), idx_eq);
        nxt.const_field_found =
            self.sel(cf_hit.clone(), Self::imm(1), nxt.const_field_found.clone());
        nxt.const_field_value = self.sel(cf_hit, lane, nxt.const_field_value.clone());
        let next_cf_idx = self.add_i(org.const_field_row_index.clone(), 1);
        nxt.const_field_row_index =
            self.sel(is_cf_row, next_cf_idx, nxt.const_field_row_index.clone());
        nxt
    }
}

struct ConstFieldCaptureOutput {
    instructions: Vec<Instruction>,
    const_count: Operand,
    const_field_count: Operand,
    const_field_found: Operand,
    const_field_value: Operand,
}

struct ConstFieldCaptureEmitter {
    scope: String,
    next_temp: usize,
}

impl ConstFieldCaptureEmitter {
    fn new(scope: &str) -> Self {
        Self {
            scope: scope.to_string(),
            next_temp: 0,
        }
    }
    fn emit(
        &mut self,
        gate: &Operand,
        pre_prog: &[Operand],
        caps: &C6PhysCaps,
        selected_field_pos: &Operand,
    ) -> ConstFieldCaptureOutput {
        let mut w = CfWalk::new(&self.scope, self.next_temp);
        let mut regs = CfRegs {
            phase: CfWalk::imm(0),
            sub: CfWalk::imm(0),
            hold: CfWalk::imm(0),
            rem: CfWalk::imm(0),
            cnt: CfWalk::imm(0),
            cnt2: CfWalk::imm(0),
            cnt3: CfWalk::imm(0),
            te_need: CfWalk::imm(0),
            srem: CfWalk::imm(0),
            const_count: CfWalk::imm(0),
            const_field_count: CfWalk::imm(0),
            const_field_row_index: CfWalk::imm(0),
            const_field_found: CfWalk::imm(0),
            const_field_value: CfWalk::imm(0),
        };
        for lane in pre_prog {
            regs = w.step(lane.clone(), regs, caps, selected_field_pos);
        }
        w.gated_eq(gate.clone(), regs.phase, CfWalk::imm(PH_END));
        self.next_temp = w.next_temp;
        ConstFieldCaptureOutput {
            instructions: w.instructions,
            const_count: regs.const_count,
            const_field_count: regs.const_field_count,
            const_field_found: regs.const_field_found,
            const_field_value: regs.const_field_value,
        }
    }
}

struct Em<'a> {
    scope: &'a str,
    next: usize,
    ins: Vec<Instruction>,
}

impl<'a> Em<'a> {
    fn imm(v: u128) -> Operand {
        Operand::Immediate(Fr::from(v))
    }
    fn t(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.next));
        self.next += 1;
        id
    }
    fn cbool(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainToBoolean { val: v });
    }
    fn cbits(&mut self, v: Operand) {
        self.ins.push(Instruction::ConstrainBits {
            val: v,
            bits: LANE_BITS,
        });
    }
    fn ceq(&mut self, a: Operand, b: Operand) {
        self.ins.push(Instruction::ConstrainEq { a, b });
    }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::TestEq {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn lt(&mut self, a: Operand, b: Operand, bits: u32) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::LessThan {
            a,
            b,
            bits,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn lt128(&mut self, a: Operand, b: Operand) -> Operand {
        self.lt(a, b, LANE_BITS)
    }
    fn lt129(&mut self, a: Operand, b: Operand) -> Operand {
        self.lt(a, b, END_BITS)
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Add {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Mul {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::Not {
            a,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.t();
        self.ins.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn assert_t(&mut self, c: Operand) {
        self.ins.push(Instruction::Assert { cond: c });
    }
    fn geq(&mut self, g: Operand, a: Operand, b: Operand) {
        let s = self.sel(g, a, b.clone());
        self.ceq(s, b);
    }
    fn gassert(&mut self, g: Operand, c: Operand) {
        let e = self.sel(g, c, Self::imm(1));
        self.assert_t(e);
    }
    fn gzero(&mut self, g: Operand, v: Operand) {
        self.geq(g, v, Self::imm(0));
    }
    fn gbool(&mut self, g: Operand, v: Operand) {
        let c = self.sel(g, v, Self::imm(0));
        self.cbool(c);
    }
    fn glt_index(&mut self, g: Operand, v: Operand, cap: usize) {
        let lt = self.lt128(v, Self::imm(cap as u128));
        self.gassert(g, lt);
    }
    fn glt_count(&mut self, g: Operand, v: Operand, cap: usize) {
        let lt = self.lt128(v, Self::imm((cap as u128) + 1));
        self.gassert(g, lt);
    }
    fn gusize(&mut self, g: Operand, v: Operand) {
        let lt = self.lt128(v, Self::imm((usize::MAX as u128) + 1));
        self.gassert(g, lt);
    }
    fn glt_end(&mut self, g: Operand, end: Operand, table_cap: usize) {
        let lt = self.lt129(end, Self::imm((table_cap as u128) + 1));
        self.gassert(g, lt);
    }
    fn and(&mut self, a: Operand, b: Operand) -> Operand {
        self.mul(a, b)
    }
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a);
        let nb = self.not(b);
        let both = self.mul(na, nb);
        self.not(both)
    }
    fn tag_bits(&mut self, g: &Operand, lane: Operand, n: usize) -> Vec<Operand> {
        let mut bits = Vec::with_capacity(n);
        let mut sum = Self::imm(0);
        for t in 0..n {
            let b = self.teq(lane.clone(), Self::imm(t as u128));
            sum = self.add(sum, b.clone());
            bits.push(b);
        }
        self.geq(g.clone(), sum, Self::imm(1));
        bits
    }
    fn gvalidate_env(&mut self, g: Operand, present: Operand, index: Operand, env_cap: usize) {
        self.gbool(g.clone(), present.clone());
        let is_absent = self.teq(present.clone(), Self::imm(0));
        let absent_gate = self.mul(g.clone(), is_absent);
        self.gzero(absent_gate, index.clone());
        let is_present = self.teq(present, Self::imm(1));
        let present_gate = self.mul(g, is_present);
        self.glt_index(present_gate, index, env_cap);
    }
    fn gvalidate_ref(&mut self, g: Operand, lanes: &[Operand], at: usize, caps: &C6PhysCaps) {
        self.gbool(g.clone(), lanes[at].clone());
        let is_data = self.teq(lanes[at].clone(), Self::imm(1));
        let cap = self.sel(
            is_data,
            Self::imm(caps.max_data_nodes as u128),
            Self::imm(caps.max_constants as u128),
        );
        let lt = self.lt128(lanes[at + 1].clone(), cap);
        self.gassert(g, lt);
    }
    fn gvalidate_slice(
        &mut self,
        g: Operand,
        lanes: &[Operand],
        at: usize,
        table_cap: usize,
        max_len: Option<usize>,
    ) {
        if let Some(m) = max_len {
            self.glt_count(g.clone(), lanes[at + 1].clone(), m);
        }
        let end = self.add(lanes[at].clone(), lanes[at + 1].clone());
        self.glt_end(g, end, table_cap);
    }
    fn gvalidate_frame(&mut self, g: Operand, frame: &[Operand], caps: &C6PhysCaps) {
        debug_assert_eq!(frame.len(), FRAME_LANES);
        let builtin_types = caps.max_types.max(4);
        let tag_bits = self.tag_bits(&g, frame[0].clone(), FRAME_VARIANT_COUNT);
        // 0 LetF
        let gt = self.mul(g.clone(), tag_bits[0].clone());
        self.glt_index(gt.clone(), frame[1].clone(), caps.max_names);
        self.glt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gvalidate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[5].clone(), caps.max_expressions);
        // 1 LeftF
        let gt = self.mul(g.clone(), tag_bits[1].clone());
        self.glt_index(gt.clone(), frame[1].clone(), 7);
        self.glt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gvalidate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[5].clone(), caps.max_expressions);
        // 2 RightF
        let gt = self.mul(g.clone(), tag_bits[2].clone());
        self.glt_index(gt.clone(), frame[1].clone(), 7);
        self.gvalidate_ref(gt.clone(), frame, 2, caps);
        self.glt_index(gt, frame[4].clone(), caps.max_expressions);
        // 3 UnaryF
        let gt = self.mul(g.clone(), tag_bits[3].clone());
        self.glt_index(gt.clone(), frame[1].clone(), 2);
        self.glt_index(gt, frame[2].clone(), caps.max_expressions);
        // 4 IfF
        let gt = self.mul(g.clone(), tag_bits[4].clone());
        self.glt_index(gt.clone(), frame[1].clone(), caps.max_expressions);
        self.glt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gvalidate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[5].clone(), caps.max_expressions);
        // 5 MatchF
        let gt = self.mul(g.clone(), tag_bits[5].clone());
        self.gvalidate_slice(gt.clone(), frame, 1, caps.max_arms, None);
        self.gvalidate_env(
            gt.clone(),
            frame[3].clone(),
            frame[4].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[5].clone(), caps.max_expressions);
        // 6 ArgsF
        let gt = self.mul(g.clone(), tag_bits[6].clone());
        self.glt_index(gt.clone(), frame[1].clone(), caps.max_names);
        self.gvalidate_slice(
            gt.clone(),
            frame,
            2,
            caps.max_value_slots,
            Some(caps.max_arity),
        );
        self.gvalidate_slice(
            gt.clone(),
            frame,
            4,
            caps.max_children,
            Some(caps.max_arity),
        );
        self.gvalidate_env(
            gt.clone(),
            frame[6].clone(),
            frame[7].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[8].clone(), caps.max_expressions);
        // 7 ReturnF (builtin TypeId cap)
        let gt = self.mul(g.clone(), tag_bits[7].clone());
        self.glt_index(gt.clone(), frame[1].clone(), builtin_types);
        self.gvalidate_env(
            gt.clone(),
            frame[2].clone(),
            frame[3].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[4].clone(), caps.max_expressions);
        // 8 ScopeF
        let gt = self.mul(g.clone(), tag_bits[8].clone());
        self.gvalidate_env(
            gt.clone(),
            frame[1].clone(),
            frame[2].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[3].clone(), caps.max_expressions);
        // 9 ConstructorFieldsF (builtin TypeId caps)
        let gt = self.mul(g.clone(), tag_bits[9].clone());
        self.glt_index(gt.clone(), frame[1].clone(), builtin_types);
        self.glt_index(gt.clone(), frame[2].clone(), caps.max_constructors);
        self.glt_index(gt.clone(), frame[3].clone(), caps.max_arity);
        self.glt_index(gt.clone(), frame[4].clone(), builtin_types);
        self.gbool(gt.clone(), frame[5].clone());
        self.gvalidate_slice(
            gt.clone(),
            frame,
            6,
            caps.max_value_slots,
            Some(caps.max_arity),
        );
        self.gvalidate_slice(
            gt.clone(),
            frame,
            8,
            caps.max_children,
            Some(caps.max_arity),
        );
        self.gvalidate_env(
            gt.clone(),
            frame[10].clone(),
            frame[11].clone(),
            caps.max_env_cells,
        );
        self.glt_index(gt, frame[12].clone(), caps.max_expressions);

        const USED: [usize; FRAME_VARIANT_COUNT] = [6, 6, 5, 3, 6, 6, 9, 5, 4, 13];
        for lane in 1..FRAME_LANES {
            let mut must_zero = Self::imm(0);
            let mut any = false;
            for (tag, used) in USED.iter().enumerate() {
                if lane >= *used {
                    must_zero = self.add(must_zero, tag_bits[tag].clone());
                    any = true;
                }
            }
            if any {
                let pad_gate = self.mul(g.clone(), must_zero);
                self.gzero(pad_gate, frame[lane].clone());
            }
        }
    }
    fn gvalidate_control(&mut self, g: Operand, k: &[Operand], caps: &C6PhysCaps) {
        debug_assert_eq!(k.len(), CONTROL_LANES);
        let builtin_types = caps.max_types.max(4);
        let bits = self.tag_bits(&g, k[0].clone(), CONTROL_VARIANT_COUNT);
        // 0 Eval { expr }
        {
            let gt = self.mul(g.clone(), bits[0].clone());
            self.glt_index(gt, k[1].clone(), caps.max_expressions);
        }
        // 1 Return { value }
        {
            let gt = self.mul(g.clone(), bits[1].clone());
            self.gvalidate_ref(gt, k, 1, caps);
        }
        // 2 ScheduleNext { frame, next, env }
        {
            let gt = self.mul(g.clone(), bits[2].clone());
            let fr: Vec<Operand> = k[1..14].to_vec();
            self.gvalidate_frame(gt.clone(), &fr, caps);
            self.glt_index(gt.clone(), k[14].clone(), caps.max_expressions);
            self.gvalidate_env(gt, k[15].clone(), k[16].clone(), caps.max_env_cells);
        }
        // 3 Invoke { function_name, args }
        {
            let gt = self.mul(g.clone(), bits[3].clone());
            self.glt_index(gt.clone(), k[1].clone(), caps.max_names);
            self.gvalidate_slice(gt, k, 2, caps.max_value_slots, Some(caps.max_arity));
        }
        // 4 Enter { function, args }
        {
            let gt = self.mul(g.clone(), bits[4].clone());
            self.gbool(gt.clone(), k[1].clone());
            let a = self.teq(k[1].clone(), Self::imm(0));
            let ag = self.mul(gt.clone(), a);
            self.gzero(ag, k[2].clone());
            let p = self.teq(k[1].clone(), Self::imm(1));
            let pg = self.mul(gt.clone(), p);
            self.glt_index(pg, k[2].clone(), caps.max_functions);
            self.gvalidate_slice(gt, k, 3, caps.max_value_slots, Some(caps.max_arity));
        }
        // 5 Bound { binding, return_type, body }
        {
            let gt = self.mul(g.clone(), bits[5].clone());
            self.gbool(gt.clone(), k[1].clone());
            let a = self.teq(k[1].clone(), Self::imm(0));
            let ag = self.mul(gt.clone(), a);
            self.gzero(ag.clone(), k[2].clone());
            self.gzero(ag, k[3].clone());
            self.gvalidate_env(gt.clone(), k[2].clone(), k[3].clone(), caps.max_env_cells);
            self.glt_index(gt.clone(), k[4].clone(), builtin_types);
            self.glt_index(gt, k[5].clone(), caps.max_expressions);
        }
        // 6 Cases { scrutinee, arms, saved_env }
        {
            let gt = self.mul(g.clone(), bits[6].clone());
            self.gvalidate_ref(gt.clone(), k, 1, caps);
            self.gvalidate_slice(gt.clone(), k, 3, caps.max_arms, None);
            self.gvalidate_env(gt, k[5].clone(), k[6].clone(), caps.max_env_cells);
        }
        // 7 CaseResult { matching, scrutinee, body, rest, saved_env }
        {
            let gt = self.mul(g.clone(), bits[7].clone());
            self.gbool(gt.clone(), k[1].clone());
            let a = self.teq(k[1].clone(), Self::imm(0));
            let ag = self.mul(gt.clone(), a);
            self.gzero(ag.clone(), k[2].clone());
            self.gzero(ag, k[3].clone());
            self.gvalidate_env(gt.clone(), k[2].clone(), k[3].clone(), caps.max_env_cells);
            self.gvalidate_ref(gt.clone(), k, 4, caps);
            self.glt_index(gt.clone(), k[6].clone(), caps.max_expressions);
            self.gvalidate_slice(gt.clone(), k, 7, caps.max_arms, None);
            self.gvalidate_env(gt, k[9].clone(), k[10].clone(), caps.max_env_cells);
        }
        // 8 PrimResult { result }
        {
            let gt = self.mul(g.clone(), bits[8].clone());
            self.gbool(gt.clone(), k[1].clone());
            let e = self.teq(k[1].clone(), Self::imm(1));
            let eg = self.mul(gt.clone(), e.clone());
            self.glt_index(eg.clone(), k[2].clone(), 256);
            self.gzero(eg, k[3].clone());
            let v = self.not(e);
            let vg = self.mul(gt, v);
            self.gvalidate_ref(vg, k, 2, caps);
        }
        // 9 Error { code }
        {
            let gt = self.mul(g.clone(), bits[9].clone());
            self.glt_index(gt, k[1].clone(), 256);
        }
        // 10 Materialize { type_id, ctor_id, fields }
        {
            let gt = self.mul(g.clone(), bits[10].clone());
            self.glt_index(gt.clone(), k[1].clone(), builtin_types);
            self.glt_index(gt.clone(), k[2].clone(), caps.max_constructors);
            self.gvalidate_slice(gt, k, 3, caps.max_value_slots, Some(caps.max_arity));
        }

        const USED: [usize; CONTROL_VARIANT_COUNT] = [2, 3, 17, 4, 5, 6, 7, 11, 4, 2, 5];
        for lane in 1..CONTROL_LANES {
            let mut must_zero = Self::imm(0);
            let mut any = false;
            for (tag, used) in USED.iter().enumerate() {
                if lane >= *used {
                    must_zero = self.add(must_zero, bits[tag].clone());
                    any = true;
                }
            }
            if any {
                let pad_gate = self.mul(g.clone(), must_zero);
                self.gzero(pad_gate, k[lane].clone());
            }
        }
    }
}

/// Retained-serial composition API; see module docs. The free function
/// `emit_c6_validate_field_step_block` is single-shot per scope per circuit.
pub struct C6ValidateFieldStepEmitter {
    scope: String,
    next_temp: usize,
    reader_em: C6CheckedValueReadEmitter,
    nominal_em: C6ProgramContextCanonicalEmitter,
    const_field_em: ConstFieldCaptureEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ValidateFieldStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ValidateFieldStepError> {
        validate_parent_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            reader_em: C6CheckedValueReadEmitter::new(&format!("{scope}.fldread")).map_err(be)?,
            nominal_em: C6ProgramContextCanonicalEmitter::new(&format!("{scope}.fldnom"))
                .map_err(be)?,
            const_field_em: ConstFieldCaptureEmitter::new(&format!("{scope}.fldcf")),
            meta_em: C6MetaCanonicalEmitter::new(scope).map_err(be)?,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(be)?,
            helper_em: C6HelperCanonicalEmitter::new(scope).map_err(be)?,
        })
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn next_temp(&self) -> usize {
        self.next_temp
    }

    pub fn emit(
        &mut self,
        inputs: &C6ValidateFieldStepInputs,
    ) -> Result<C6ValidateFieldStepBlock, C6ValidateFieldStepError> {
        let scope: &str = &self.scope;
        validate_parent_scope(scope)?;
        if (LANE_BITS as usize) >= FR_BITS || (END_BITS as usize) >= FR_BITS {
            return Err(C6ValidateFieldStepError::InvalidBitWidth { bits: END_BITS });
        }
        for op in [
            &inputs.enable,
            &inputs.disposition_is_refused_helper_work,
            &inputs.disposition_is_refused_helper_tasks,
            &inputs.host_schema,
            &inputs.carrier_version,
        ] {
            reject_reserved_operand(scope, op)?;
        }
        for op in inputs
            .pre_meta
            .iter()
            .chain(inputs.post_meta.iter())
            .chain(inputs.pre_prog.iter())
            .chain(inputs.post_prog.iter())
            .chain(inputs.pre_ctx.iter())
            .chain(inputs.post_ctx.iter())
            .chain(inputs.pre_image.iter())
            .chain(inputs.post_image.iter())
            .chain(inputs.pre_data.iter())
            .chain(inputs.post_data.iter())
            .chain(inputs.pre_helper.iter())
            .chain(inputs.post_helper.iter())
        {
            reject_reserved_operand(scope, op)?;
        }
        inputs.source_params.validate_geometry(inputs.geometry)?;
        let g = &inputs.geometry;
        let caps = &g.caps;
        let p = &caps.phys;
        for cap in [
            p.max_frames,
            p.max_expressions,
            p.max_names,
            p.max_arms,
            p.max_children,
            p.max_value_slots,
            p.max_constants,
            p.max_data_nodes,
            p.max_env_cells,
            p.max_functions,
            p.max_arity,
            p.max_types,
            p.max_occurrences,
            p.max_occurrence_paths,
            p.max_parameters,
            p.max_patterns,
            p.max_pattern_children,
            p.max_constructors,
            p.max_ctor_fields,
            p.max_decl_templates,
            p.max_plan_nodes,
            p.max_plan_cases,
            p.max_field_paths,
            p.max_constant_fields,
            p.max_name_bytes,
            p.max_type_expr_nodes,
            p.max_template_bytes,
            caps.max_data_field_slots,
            caps.max_helper_tasks,
        ] {
            if cap == usize::MAX {
                return Err(C6ValidateFieldStepError::GeometryOverflow { section: "caps" });
            }
        }
        let data_geom = c6_data_geometry(caps).map_err(be)?;
        for (s, a, e) in [
            ("image", inputs.pre_image.len(), g.image_total_lanes),
            ("image", inputs.post_image.len(), g.image_total_lanes),
            ("data", inputs.pre_data.len(), g.data_total_lanes),
            ("data", inputs.post_data.len(), g.data_total_lanes),
            ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
            ("helper", inputs.post_helper.len(), g.helper_total_lanes),
        ] {
            if a != e {
                return Err(C6ValidateFieldStepError::LengthMismatch {
                    section: s,
                    expected: e,
                    actual: a,
                });
            }
        }
        if inputs.pre_meta.len() < 3
            || inputs.pre_meta.len() != inputs.post_meta.len()
            || inputs.pre_prog.len() < PROGRAM_HEADER_LANES
            || inputs.pre_prog.len() != inputs.post_prog.len()
            || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len()
        {
            return Err(C6ValidateFieldStepError::Backend(
                "host prefix length/offset".into(),
            ));
        }

        let adm = inputs.source_params.admission_bounds();
        let max_hw = adm.max_helper_work;
        let max_ht = adm.helper_tasks;
        let rec = L_HELPER_RECORD;
        let _ = L_RESULT_RECORD;

        let mut em = Em {
            scope,
            next: self.next_temp,
            ins: Vec::new(),
        };
        let en = inputs.enable.clone();
        let dp_hw = inputs.disposition_is_refused_helper_work.clone();
        let dp_ht = inputs.disposition_is_refused_helper_tasks.clone();
        em.cbool(en.clone());
        em.cbool(dp_hw.clone());
        em.cbool(dp_ht.clone());
        let disabled = em.not(en.clone());

        em.ceq(inputs.host_schema.clone(), Em::imm(C6_HOST_SCHEMA));
        em.ceq(
            inputs.carrier_version.clone(),
            Em::imm(C6_PADDED_CARRIER_VERSION),
        );
        for (a, b) in inputs
            .pre_meta
            .iter()
            .zip(inputs.post_meta.iter())
            .chain(inputs.pre_prog.iter().zip(inputs.post_prog.iter()))
        {
            em.cbits(a.clone());
            em.cbits(b.clone());
            em.ceq(b.clone(), a.clone());
        }
        for (a, b) in inputs.pre_ctx.iter().zip(inputs.post_ctx.iter()) {
            em.cbits(a.clone());
            em.cbits(b.clone());
            em.ceq(b.clone(), a.clone());
        }
        em.geq(
            en.clone(),
            inputs.pre_meta[0].clone(),
            Em::imm(C6_PHYS_MAGIC),
        );
        em.geq(
            en.clone(),
            inputs.pre_meta[1].clone(),
            Em::imm(C6_PHYS_VERSION),
        );
        em.geq(
            en.clone(),
            inputs.pre_ctx[0].clone(),
            Em::imm(C6_PHYS_MAGIC),
        );
        em.geq(
            en.clone(),
            inputs.pre_ctx[1].clone(),
            Em::imm(C6_PHYS_VERSION),
        );
        for l in inputs
            .pre_image
            .iter()
            .chain(inputs.post_image.iter())
            .chain(inputs.pre_data.iter())
            .chain(inputs.post_data.iter())
            .chain(inputs.pre_helper.iter())
            .chain(inputs.post_helper.iter())
        {
            em.cbits(l.clone());
        }

        let pre_i = &inputs.pre_image;
        let post_i = &inputs.post_image;
        let pre_h = &inputs.pre_helper;
        let post_h = &inputs.post_helper;

        // ---- 1. Pre-image canonicality and owner_site well-definedness (PS) ----
        em.geq(en.clone(), pre_i[L_MAGIC].clone(), Em::imm(C6_PHYS_MAGIC));
        em.geq(
            en.clone(),
            pre_i[L_VERSION].clone(),
            Em::imm(C6_PHYS_VERSION),
        );
        for off in 0..3 {
            em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone()); // OutcomeC::Running
        }
        {
            let hf = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone());
            em.gassert(en.clone(), hf);
        }
        em.gvalidate_control(en.clone(), &pre_i[L_CONTROL..L_CONTROL + CONTROL_LANES], p);
        // `selected_for` calls `owner_site(pre)`, which rejects `ControlC::Error` (tag 9)
        // and rejects `ControlC::Return` (tag 1) when `pre.depth() == 0`.
        {
            let ctrl_is_err = em.teq(pre_i[L_CONTROL].clone(), Em::imm(9));
            em.gzero(en.clone(), ctrl_is_err);
            let ctrl_is_ret = em.teq(pre_i[L_CONTROL].clone(), Em::imm(1));
            let ret_gate = em.and(en.clone(), ctrl_is_ret);
            let depth_pos = em.lt128(Em::imm(0), pre_i[L_FRAME_COUNT].clone());
            em.gassert(ret_gate, depth_pos);
        }
        em.glt_index(
            en.clone(),
            pre_i[L_CONTROL_ORIGIN].clone(),
            p.max_expressions,
        );
        em.gvalidate_env(
            en.clone(),
            pre_i[L_ENV_ROOT].clone(),
            pre_i[L_ENV_ROOT + 1].clone(),
            p.max_env_cells,
        );
        em.glt_count(en.clone(), pre_i[L_STACK_LIMIT].clone(), p.max_frames);
        em.glt_count(en.clone(), pre_i[L_ALLOC].clone(), p.max_data_nodes);
        em.gusize(en.clone(), pre_i[L_ALLOC + 1].clone());
        em.glt_count(en.clone(), pre_i[L_ALLOC + 2].clone(), p.max_env_cells);
        em.glt_count(en.clone(), pre_i[L_ACTIVE].clone(), p.max_data_nodes);
        em.gusize(en.clone(), pre_i[L_ACTIVE + 1].clone());
        em.glt_count(en.clone(), pre_i[L_ACTIVE + 2].clone(), p.max_env_cells);
        em.glt_count(en.clone(), pre_i[L_ACTIVE + 3].clone(), p.max_value_slots);
        em.glt_count(en.clone(), pre_i[L_SEG_ACTIVE].clone(), p.max_data_nodes);
        em.gusize(en.clone(), pre_i[L_SEG_ACTIVE + 1].clone());
        em.glt_count(en.clone(), pre_i[L_SEG_ACTIVE + 2].clone(), p.max_env_cells);
        em.glt_count(
            en.clone(),
            pre_i[L_SEG_ACTIVE + 3].clone(),
            p.max_value_slots,
        );
        em.glt_count(en.clone(), pre_i[L_SEG_ALLOC].clone(), p.max_data_nodes);
        em.gusize(en.clone(), pre_i[L_SEG_ALLOC + 1].clone());
        em.glt_count(en.clone(), pre_i[L_SEG_ALLOC + 2].clone(), p.max_env_cells);
        let depth = pre_i[L_FRAME_COUNT].clone();
        em.glt_count(en.clone(), depth.clone(), p.max_frames);
        for slot in 0..p.max_frames {
            let slot_imm = Em::imm(slot as u128);
            let base = g.frame_slot_offset(slot);
            let slot_active = em.lt128(slot_imm, depth.clone());
            let slot_inactive = em.not(slot_active.clone());
            let active_gate = em.and(en.clone(), slot_active);
            let inactive_gate = em.and(en.clone(), slot_inactive);
            em.gvalidate_frame(active_gate, &pre_i[base..base + FRAME_LANES], p);
            for off in 0..FRAME_LANES {
                em.gzero(inactive_gate.clone(), pre_i[base + off].clone());
            }
        }

        // ---- 2. Helper-group selection and dynamic top-task extraction (PS) ----
        // `HelperControlC::Validate` has codec tag 7 at `L_HELPER_RECORD` (`rec`),
        // with `status` at `rec + 7` (`0 = TriStatus::Pending`).
        em.geq(en.clone(), pre_h[rec].clone(), Em::imm(7));
        em.gzero(en.clone(), pre_h[rec + 7].clone());
        // `pre.task_live() > 0` (`pre_h[3] > 0`).
        {
            let tasks_pos = em.lt128(Em::imm(0), pre_h[3].clone());
            em.gassert(en.clone(), tasks_pos);
        }

        // Extract top task at `slot == pre_h[3] - 1` (`slot + 1 == pre_h[3]`).
        let mut top_found_raw = Em::imm(0);
        let mut top_task_raw: [Operand; TASK_LANES] = std::array::from_fn(|_| Em::imm(0));
        for slot in 0..caps.max_helper_tasks {
            let base = L_TASKS + slot * TASK_LANES;
            let is_top = em.teq(Em::imm((slot as u128) + 1), pre_h[3].clone());
            top_found_raw = em.or(top_found_raw, is_top.clone());
            for k in 0..TASK_LANES {
                top_task_raw[k] = em.sel(
                    is_top.clone(),
                    pre_h[base + k].clone(),
                    top_task_raw[k].clone(),
                );
            }
        }
        em.geq(en.clone(), top_found_raw.clone(), Em::imm(1));
        // `HelperTaskC::ValidateTask` has codec tag 0 (`top_task[0] == 0`),
        // and `TaskVisitC::Fields` has codec tag 1 (`top_task[3] == 1`).
        em.gzero(en.clone(), top_task_raw[0].clone());
        em.geq(en.clone(), top_task_raw[3].clone(), Em::imm(1));

        // Gate the top-task operands by `enable`.
        let top_task_found = em.sel(en.clone(), top_found_raw, Em::imm(0));
        let top_task_tag = em.sel(en.clone(), top_task_raw[0].clone(), Em::imm(0));
        let top_ref_is_data = em.sel(en.clone(), top_task_raw[1].clone(), Em::imm(0));
        let top_ref_index = em.sel(en.clone(), top_task_raw[2].clone(), Em::imm(0));
        let top_task_visit = em.sel(en.clone(), top_task_raw[3].clone(), Em::imm(0));
        let top_task_cursor = em.sel(en.clone(), top_task_raw[4].clone(), Em::imm(0));

        // ---- 3. PS read 1: checked value read of `top_task.r` ----
        // `C6CheckedValueReadEmitter::emit` composes both
        // `C6ProgramContextCanonicalEmitter::emit_prog_const_capture` and
        // `C6DataCanonicalEmitter::emit` over `pre_prog` and `pre_data` under `en`.
        let reader = self
            .reader_em
            .emit(&C6CheckedValueReadInputs {
                gate: en.clone(),
                ref_kind_is_data: top_ref_is_data.clone(),
                ref_index: top_ref_index.clone(),
                pre_prog: inputs.pre_prog.clone(),
                pre_data: inputs.pre_data.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(reader.instructions.clone());

        // `top_task.r` must exist in its active table (`selected_found == 1`)
        // and not be a `Marker` (`reader.is_marker == 0`).
        em.geq(en.clone(), reader.selected_found.clone(), Em::imm(1));
        em.gzero(en.clone(), reader.is_marker.clone());

        // ---- 4. PS read 2: `field_at(&r, &cursor)` child lookup & SC-05 check ----
        // Raw value slice bound: `cursor < reader.slice_len`.
        let cursor_in_slice = em.lt128(top_task_cursor.clone(), reader.slice_len.clone());
        em.gassert(en.clone(), cursor_in_slice);

        // `pos = start.checked_add(cursor)` in 129 bits (`reader.slice_start + top_task_cursor`).
        let field_pos_raw = em.add(reader.slice_start.clone(), top_task_cursor.clone());
        let field_pos = em.sel(en.clone(), field_pos_raw.clone(), Em::imm(0));

        // In-circuit `PROG` `constant_fields[field_pos]` and `constants.len()` capture.
        let cf_capture = self
            .const_field_em
            .emit(&en, &inputs.pre_prog, p, &field_pos);
        em.ins.extend(cf_capture.instructions);

        // In-circuit `DATA` `data_fields[field_pos]` capture over `pre_data`.
        let data_field_count = inputs.pre_data[4].clone();
        let mut data_field_found_raw = Em::imm(0);
        let mut data_field_is_data_raw = Em::imm(0);
        let mut data_field_index_raw = Em::imm(0);
        for slot in 0..caps.max_data_field_slots {
            let base = data_geom.fields_base + slot * REF_LANES;
            let hit = em.teq(field_pos.clone(), Em::imm(slot as u128));
            data_field_found_raw = em.or(data_field_found_raw, hit.clone());
            data_field_is_data_raw = em.sel(
                hit.clone(),
                inputs.pre_data[base].clone(),
                data_field_is_data_raw,
            );
            data_field_index_raw =
                em.sel(hit, inputs.pre_data[base + 1].clone(), data_field_index_raw);
        }

        // Enforce active field-table membership by parent reference space:
        // * Const parent (`top_ref_is_data == 0`): `field_pos < const_field_count`
        //   and `const_field_found == 1`.
        // * Data parent (`top_ref_is_data == 1`): `field_pos < data_field_count`
        //   and `data_field_found == 1`.
        let is_data_parent = top_ref_is_data.clone();
        let is_const_parent = em.not(is_data_parent.clone());
        let const_parent_gate = em.and(en.clone(), is_const_parent);
        let data_parent_gate = em.and(en.clone(), is_data_parent.clone());

        let pos_lt_cf = em.lt129(field_pos_raw.clone(), cf_capture.const_field_count.clone());
        em.gassert(const_parent_gate.clone(), pos_lt_cf);
        em.geq(
            const_parent_gate,
            cf_capture.const_field_found.clone(),
            Em::imm(1),
        );

        let pos_lt_df = em.lt129(field_pos_raw, data_field_count.clone());
        em.gassert(data_parent_gate.clone(), pos_lt_df);
        em.geq(data_parent_gate, data_field_found_raw.clone(), Em::imm(1));

        let field_found_raw = em.sel(
            is_data_parent.clone(),
            data_field_found_raw.clone(),
            cf_capture.const_field_found.clone(),
        );
        let child_ref_is_data_raw = em.sel(
            is_data_parent.clone(),
            data_field_is_data_raw.clone(),
            Em::imm(0),
        );
        let child_ref_index_raw = em.sel(
            is_data_parent,
            data_field_index_raw.clone(),
            cf_capture.const_field_value.clone(),
        );
        let field_found = em.sel(en.clone(), field_found_raw, Em::imm(0));
        let child_ref_is_data = em.sel(en.clone(), child_ref_is_data_raw.clone(), Em::imm(0));
        let child_ref_index = em.sel(en.clone(), child_ref_index_raw.clone(), Em::imm(0));

        // SC-05 (`check_written_code_ids` in `owner_validate_plan`, evaluated in
        // PS before P3 and P6): every written `RefC::Const(id)` in `child_task`
        // must satisfy `id < prog.constants.len()` (`cf_capture.const_count`).
        {
            let child_is_const = em.not(child_ref_is_data_raw.clone());
            let child_const_gate = em.and(en.clone(), child_is_const);
            let child_const_in_bounds =
                em.lt128(child_ref_index_raw.clone(), cf_capture.const_count.clone());
            em.gassert(child_const_gate, child_const_in_bounds);
        }

        // ---- 5. PS read 3: bounded program declaration lookup (`adt_checked_ctor_arity`) ----
        let nominal_capture = self
            .nominal_em
            .emit_prog_checked_sort_capture(&C6ProgramContextCheckedSortCaptureInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps: *caps,
                selected_type_id: reader.type_id.clone(),
                selected_ctor_id: reader.ctor_id.clone(),
            })
            .map_err(be)?;
        em.ins.extend(nominal_capture.instructions.clone());

        // Core `adt_checked_ctor_arity(view, type_id, ctor_id)` conditions:
        // 1. `type_id.0 >= 4` (built-in type IDs `0..4` rejected)
        // 2. `type_id.0 < view.types.len()` and active type row captured (`type_found == 1`)
        // 3. `ctor_id.0 < view.constructors.len()` (`ctor_found == 1`)
        // 4. `c_row.owning_type == type_id`
        // 5. `ctor_id.0 >= t_row.ctor_start.0` and `ctor_id.0 < t_row.ctor_start.0 + t_row.ctor_len`
        // 6. `c_row.field_start + c_row.field_len <= view.ctor_fields.len()`
        let nominal_valid_raw = {
            let lt4 = em.lt128(reader.type_id.clone(), Em::imm(4));
            let ge4 = em.not(lt4);
            let type_in_bounds =
                em.lt128(reader.type_id.clone(), nominal_capture.type_count.clone());
            let type_ok = em.and(type_in_bounds, nominal_capture.type_found.clone());
            let owning = em.teq(nominal_capture.owning_type.clone(), reader.type_id.clone());
            let below_start = em.lt129(reader.ctor_id.clone(), nominal_capture.ctor_start.clone());
            let at_least_start = em.not(below_start);
            let ctor_end = em.add(
                nominal_capture.ctor_start.clone(),
                nominal_capture.ctor_len.clone(),
            );
            let below_end = em.lt129(reader.ctor_id.clone(), ctor_end);
            let fend = em.add(
                nominal_capture.field_start.clone(),
                nominal_capture.field_len.clone(),
            );
            let count_below = em.lt129(nominal_capture.ctor_field_count.clone(), fend);
            let field_ok = em.not(count_below);

            let v = em.and(ge4, type_ok);
            let v = em.and(v, nominal_capture.ctor_found.clone());
            let v = em.and(v, owning);
            let v = em.and(v, at_least_start);
            let v = em.and(v, below_end);
            em.and(v, field_ok)
        };
        let ctor_gate = em.and(en.clone(), reader.is_constructor.clone());
        em.gassert(ctor_gate.clone(), nominal_valid_raw.clone());
        let nominal_valid = em.sel(ctor_gate, nominal_valid_raw, Em::imm(0));

        // ---- 6. `checked_arity` and `GValidateFieldNontail` (`cursor + 1 < checked_arity`) ----
        // - UInt / Bool / Nil -> 0
        // - Leaf -> 1
        // - Cons / Branch -> 2
        // - Constructor -> `nominal_capture.field_len` (declared arity, never `reader.slice_len`)
        let cons_or_branch = em.or(reader.is_cons.clone(), reader.is_branch.clone());
        let leaf_or_zero = em.sel(reader.is_leaf.clone(), Em::imm(1), Em::imm(0));
        let fixed_arity = em.sel(cons_or_branch, Em::imm(2), leaf_or_zero);
        let checked_arity_raw = em.sel(
            reader.is_constructor.clone(),
            nominal_capture.field_len.clone(),
            fixed_arity,
        );
        let checked_arity = em.sel(en.clone(), checked_arity_raw, Em::imm(0));

        let next_cursor_raw = em.add(top_task_cursor.clone(), Em::imm(1));
        let next_cursor = em.sel(en.clone(), next_cursor_raw.clone(), Em::imm(0));
        let is_nontail_raw = em.lt129(next_cursor_raw.clone(), checked_arity.clone());
        let is_nontail = em.sel(en.clone(), is_nontail_raw.clone(), Em::imm(0));

        // ---- 7. Three-outcome driver: P3 vs P6 vs P7 (`Applied`) ----
        // * P3 (`RefusedHelperWorkUncharged`): `pre.helper_work == adm.max_helper_work`.
        //   Precedes P4 and P6 even when `would_p6` holds.
        // * P6 (`RefusedCapCharged` / `HelperTasks`): `!is_p3 && is_nontail && pre.task_live + 1 > adm.helper_tasks`.
        // * P7 (`Applied`): `!is_p3 && !is_p6`.
        let is_p3 = em.teq(pre_i[L_HELPER_WORK].clone(), Em::imm(max_hw));
        em.geq(en.clone(), dp_hw, is_p3.clone());
        let not_p3 = em.not(is_p3.clone());

        let next_task_live = em.add(pre_h[3].clone(), Em::imm(1));
        let exceeds_adm_tasks = em.lt129(Em::imm(max_ht as u128), next_task_live.clone());
        let would_p6 = em.and(is_nontail_raw.clone(), exceeds_adm_tasks);
        let is_p6 = em.and(not_p3.clone(), would_p6);
        em.geq(en.clone(), dp_ht, is_p6.clone());

        let not_p6 = em.not(is_p6.clone());
        let is_applied = em.and(not_p3.clone(), not_p6);
        let is_refused = em.or(is_p3.clone(), is_p6.clone());

        let p3_gate = em.and(en.clone(), is_p3.clone());
        let charged_gate = em.and(en.clone(), not_p3);
        let applied_gate = em.and(en.clone(), is_applied.clone());

        // Post outcome:
        // * P3 (`is_p3 == 1`): `OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }` -> `[3, 5, 0]`
        // * P6 (`is_p6 == 1`): `OutcomeC::Exhausted { reason: ExhaustReason::HelperTasks }` -> `[3, 4, 0]`
        // * P7 (`is_applied == 1`): `OutcomeC::Running` -> `[0, 0, 0]`
        let exp_o0 = em.sel(is_refused, Em::imm(3), Em::imm(0));
        em.geq(en.clone(), post_i[L_OUTCOME].clone(), exp_o0);
        let exp_o1_p6 = em.sel(is_p6.clone(), Em::imm(4), Em::imm(0));
        let exp_o1 = em.sel(is_p3.clone(), Em::imm(5), exp_o1_p6);
        em.geq(en.clone(), post_i[L_OUTCOME + 1].clone(), exp_o1);
        em.gzero(en.clone(), post_i[L_OUTCOME + 2].clone());

        // P4 counters (`remaining_work`, `consumed_work`, `helper_work`):
        // Charged under `charged_gate` (both P6 and Applied); uncharged under `p3_gate`.
        {
            let s = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
            em.geq(charged_gate.clone(), s, pre_i[L_REMAINING_WORK].clone());
            em.geq(
                p3_gate,
                post_i[L_REMAINING_WORK].clone(),
                pre_i[L_REMAINING_WORK].clone(),
            );
        }
        for lane in [L_CONSUMED_WORK, L_HELPER_WORK] {
            let noov = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX));
            em.gassert(charged_gate.clone(), noov);
            let exp_charged = em.add(pre_i[lane].clone(), Em::imm(1));
            let exp = em.sel(is_p3.clone(), pre_i[lane].clone(), exp_charged);
            em.geq(en.clone(), post_i[lane].clone(), exp);
        }

        // P7 counter (`helper_visits`):
        // Charged ONLY under `applied_gate`; uncharged on both P3 and P6.
        {
            let noov_visits = em.lt128(pre_i[L_HELPER_VISITS].clone(), Em::imm(u128::MAX));
            em.gassert(applied_gate, noov_visits);
            let exp_visits_applied = em.add(pre_i[L_HELPER_VISITS].clone(), Em::imm(1));
            let exp_visits = em.sel(
                is_applied.clone(),
                exp_visits_applied,
                pre_i[L_HELPER_VISITS].clone(),
            );
            em.geq(en.clone(), post_i[L_HELPER_VISITS].clone(), exp_visits);
        }

        // Every other `IMAGE` lane (`reference_cursor`, `control`, `control_origin`,
        // `env_root`, `stack_limit`, `alloc`, `active`, `seg_active`, `seg_alloc`,
        // `frame_count`, and all frame slots) is preserved unchanged.
        let mutable = [
            L_OUTCOME,
            L_OUTCOME + 1,
            L_OUTCOME + 2,
            L_REMAINING_WORK,
            L_CONSUMED_WORK,
            L_HELPER_WORK,
            L_HELPER_VISITS,
        ];
        for lane in 0..g.image_total_lanes {
            if !mutable.contains(&lane) {
                em.geq(en.clone(), post_i[lane].clone(), pre_i[lane].clone());
            }
        }

        // `DATA` is unconditionally preserved unchanged (`pre_data` canonicality is
        // already emitted inside `self.reader_em.emit`).
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.ceq(b.clone(), a.clone());
        }

        // ---- 8. `HELPER` post-state ----
        // Candidate task-stack values when `is_applied == 1`:
        // * When `is_nontail_raw == 1`:
        //   `post_task_count = pre_h[3] + 1`, slot `pre_h[3] - 1` is rewritten to
        //   `[0, top_task_raw[1], top_task_raw[2], 1, next_cursor_raw, 0, 0]`,
        //   and slot `pre_h[3]` is pushed with
        //   `[0, child_ref_is_data_raw, child_ref_index_raw, 0, 0, 0, 0]`.
        // * When `is_nontail_raw == 0` (tail):
        //   `post_task_count = pre_h[3]` and slot `pre_h[3] - 1` is replaced in
        //   place with `[0, child_ref_is_data_raw, child_ref_index_raw, 0, 0, 0, 0]`.
        // * When `is_applied == 0` (P3 or P6):
        //   `post_h` is preserved completely unchanged (`post_h == pre_h`).
        let cand_task_count = em.sel(is_nontail_raw.clone(), next_task_live, pre_h[3].clone());
        let post_task_count = em.sel(is_applied.clone(), cand_task_count, pre_h[3].clone());

        let post_top_task_tag = Em::imm(0);
        let post_top_ref_is_data = child_ref_is_data_raw.clone();
        let post_top_ref_index = child_ref_index_raw.clone();
        let post_top_task_visit = Em::imm(0);
        let post_top_task_cursor = Em::imm(0);
        let post_rewritten_parent_cursor = next_cursor_raw.clone();

        let child_slot: [Operand; TASK_LANES] = [
            post_top_task_tag.clone(),
            post_top_ref_is_data.clone(),
            post_top_ref_index.clone(),
            post_top_task_visit.clone(),
            post_top_task_cursor.clone(),
            Em::imm(0),
            Em::imm(0),
        ];
        let rewritten_parent_slot: [Operand; TASK_LANES] = [
            Em::imm(0),
            top_task_raw[1].clone(),
            top_task_raw[2].clone(),
            Em::imm(1),
            post_rewritten_parent_cursor.clone(),
            Em::imm(0),
            Em::imm(0),
        ];

        em.geq(en.clone(), post_h[3].clone(), post_task_count.clone());
        for lane in 0..HELPER_FIXED_LANES {
            if lane != 3 {
                em.geq(en.clone(), post_h[lane].clone(), pre_h[lane].clone());
            }
        }
        for slot in 0..caps.max_helper_tasks {
            let base = L_TASKS + slot * TASK_LANES;
            let is_top = em.teq(Em::imm((slot as u128) + 1), pre_h[3].clone());
            let is_push = em.teq(Em::imm(slot as u128), pre_h[3].clone());
            for k in 0..TASK_LANES {
                let top_applied_k = em.sel(
                    is_nontail_raw.clone(),
                    rewritten_parent_slot[k].clone(),
                    child_slot[k].clone(),
                );
                let push_applied_k = em.sel(
                    is_nontail_raw.clone(),
                    child_slot[k].clone(),
                    pre_h[base + k].clone(),
                );
                let after_push_k = em.sel(is_push.clone(), push_applied_k, pre_h[base + k].clone());
                let applied_lane = em.sel(is_top.clone(), top_applied_k, after_push_k);
                let exp_lane = em.sel(is_applied.clone(), applied_lane, pre_h[base + k].clone());
                em.geq(en.clone(), post_h[base + k].clone(), exp_lane);
            }
        }

        // ---- 9. Composed canonical blocks (retained serials) ----
        let mb_block = self
            .meta_em
            .emit_bound(&C6MetaBoundCanonicalInputs {
                gate: en.clone(),
                pre_meta: inputs.pre_meta.clone(),
                post_meta: inputs.post_meta.clone(),
                limits: inputs.source_params.limits(),
                meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
                binding: inputs.source_params.meta_binding(),
            })
            .map_err(be)?;
        em.ins.extend(mb_block.instructions);

        let pc = self
            .progctx_em
            .emit(&C6ProgramContextCanonicalInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                pre_ctx: inputs.pre_ctx.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(pc.instructions);

        let cx = self
            .progctx_em
            .emit_ctx_binding(&C6ProgramContextCtxBindingInputs {
                gate: en.clone(),
                pre_ctx: inputs.pre_ctx.clone(),
                expected_ctx: inputs.source_params.expected_ctx(),
            })
            .map_err(be)?;
        em.ins.extend(cx.instructions);

        let hb1 = self
            .helper_em
            .emit(&C6HelperCanonicalInputs {
                gate: en.clone(),
                helper: inputs.pre_helper.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(hb1.instructions);

        let hb2 = self
            .helper_em
            .emit(&C6HelperCanonicalInputs {
                gate: en.clone(),
                helper: inputs.post_helper.clone(),
                caps: *caps,
            })
            .map_err(be)?;
        em.ins.extend(hb2.instructions);

        // Disabled (`enable == 0`): full equality in image and helper
        // (DATA/META/PROG/CTX equality above is already unconditional).
        for (a, b) in inputs
            .pre_image
            .iter()
            .zip(inputs.post_image.iter())
            .chain(inputs.pre_helper.iter().zip(inputs.post_helper.iter()))
        {
            em.geq(disabled.clone(), b.clone(), a.clone());
        }

        self.next_temp = em.next;
        Ok(C6ValidateFieldStepBlock {
            instructions: em.ins,
            reader,
            nominal_capture,
            const_field_count: cf_capture.const_field_count,
            const_field_found: cf_capture.const_field_found,
            const_field_value: cf_capture.const_field_value,
            data_field_count,
            data_field_found: data_field_found_raw,
            data_field_is_data: data_field_is_data_raw,
            data_field_index: data_field_index_raw,
            top_task_found,
            top_task_tag,
            top_ref_is_data,
            top_ref_index,
            top_task_visit,
            top_task_cursor,
            field_pos,
            field_found,
            child_ref_is_data,
            child_ref_index,
            nominal_valid,
            checked_arity,
            next_cursor,
            is_nontail,
            is_helper_work_max: is_p3,
            is_helper_tasks_refused: is_p6,
            is_applied,
            post_task_count,
            post_top_task_tag,
            post_top_ref_is_data,
            post_top_ref_index,
            post_top_task_visit,
            post_top_task_cursor,
            post_rewritten_parent_cursor,
        })
    }
}

/// Single-shot block at parent serial zero; see
/// `C6ValidateFieldStepEmitter` for the composition contract.
pub fn emit_c6_validate_field_step_block(
    inputs: &C6ValidateFieldStepInputs,
    scope: &str,
) -> Result<C6ValidateFieldStepBlock, C6ValidateFieldStepError> {
    C6ValidateFieldStepEmitter::new(scope)?.emit(inputs)
}
