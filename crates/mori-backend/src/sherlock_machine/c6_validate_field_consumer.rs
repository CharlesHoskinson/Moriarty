//! C6 `BHelperValidateField` consumer: typed host-pair / owner-record
//! witnesses, the single-step owner wrapper, and the SDK statement
//! builder/checker for `C6ValidateFieldStepEmitter`.
//!
//! Statement order (`SCALAR_PREFIX_LANES = 5`):
//! 1. `enable` (`1` lane)
//! 2. `disposition_is_refused_helper_work` (`1` lane)
//! 3. `disposition_is_refused_helper_tasks` (`1` lane)
//! 4. `host_schema` (`1` lane)
//! 5. `carrier_version` (`1` lane)
//! 6. `pre(META, PROG, CTX, image, data, helper)`
//! 7. `post(META, PROG, CTX, image, data, helper)`
//!
//! Relation and driver contract:
//! * `BHelperValidateField` has `task_live` delta `+1` when
//!   `GValidateFieldNontail` holds (`cursor + 1 < checked_arity`) and `0`
//!   when `GValidateFieldNontail` is false (`cursor + 1 >= checked_arity`),
//!   with zero allocation deltas (`data_nodes = 0`, `data_fields = 0`,
//!   `environment_cells = 0`, `depth = 0`, `active_value_slots = 0`).
//!   Consequently `owner_cap_refusal` returns `Some(CapKind::HelperTasks)`
//!   iff `GValidateFieldNontail` holds and
//!   `pre.task_live() + 1 > admission.bounds.helper_tasks`.
//! * `disposition_is_refused_helper_work` (P3) and
//!   `disposition_is_refused_helper_tasks` (P6) are derived strictly from the
//!   PRE snapshot against the ORIGINAL SEPARATE admission bounds carried in
//!   `C6PushFamilyParams::admission_bounds()` (never layout bounds or any
//!   caller/post flag), and are constrained in-circuit by
//!   `C6ValidateFieldStepEmitter` when `enable == 1`. P3 (`pre.helper_work == adm.max_helper_work`)
//!   takes precedence over P6 (`!p3 && is_nontail && pre.task_live + 1 > adm.helper_tasks`).
//! * Selection and PS checks (`owner_select`, `plan_validate_nonentry`,
//!   `field_at(&r, &cursor)`, `eval_guard(GValidateFieldNontail)`,
//!   `owner_validate_plan`, and `owner_physical_check`) precede P3 and P6
//!   both in Core `owner_step` and in `validate_owner_record` /
//!   `C6ValidateFieldStepEmitter`:
//!   - `pre.outcome == OutcomeC::Running`, `pre.remaining_work > 0`, and
//!     `owner_site(pre)` well-defined (`!ControlC::Error`, and non-empty
//!     `frames` when `ControlC::Return`).
//!   - `pre.helper == Some(HelperControlC::Validate { common, root, status: TriStatus::Pending })`
//!     with `common.parent_resume == None` and `!pre.tasks.is_empty()`.
//!   - `pre.tasks.last() == Some(HelperTaskC::ValidateTask { r: top_r, visit: TaskVisitC::Fields, cursor: top_cursor })`.
//!   - `field_at(&top_r, &top_cursor)` resolves `top_r` in its active table
//!     (`RefC::Const` in `pre.admission.view.constants` or `RefC::Data` in
//!     `pre.image.data`), checks `top_cursor < raw_slice_len(top_r)`, and
//!     reads `child_r` at `pos = raw_slice_start(top_r) + top_cursor` from
//!     `pre.admission.view.constant_fields` (yielding `RefC::Const(cid)`) or
//!     `pre.image.data_fields` (yielding stored `RefC::Const` or `RefC::Data`).
//!     Plan validation (`SC-05` `check_written_code_ids`) requires any
//!     `RefC::Const(cid)` written in `child_task` to satisfy
//!     `cid < pre.admission.view.constants.len()`. `child_r` may reference a
//!     `ConstantSnapshot::U128OutOfRange` (`LogicalKind::Marker`) row;
//!     `BHelperValidateField` schedules the `Unvisited` child task without
//!     inspecting the child row or setting `TriStatus::Negative`.
//!   - When `top_r` is `Constructor { type_id, ctor_id, .. }`, full Core
//!     `adt_checked_ctor_arity(&pre.admission.view, type_id, ctor_id)` must
//!     succeed before P3 and P6, and `checked_arity` is the declared
//!     `c_row.field_len` (NEVER `fields.len` / `reader.slice_len`).
//!   - `GValidateFieldNontail` (`is_nontail`) holds iff
//!     `top_cursor.checked_add(1) < checked_arity`:
//!     * Under `StepDisposition::Applied` (`!p3 && !p6`), P4/P7 charge
//!       `remaining_work - 1`, `consumed_work + 1`, `helper_work + 1`, and
//!       `helper_visits + 1` while `reference_cursor` is unchanged; if
//!       `is_nontail`, the top task is rewritten to
//!       `ValidateTask { r: top_r, visit: TaskVisitC::Fields, cursor: top_cursor + 1 }`
//!       and `ValidateTask { r: child_r, visit: TaskVisitC::Unvisited, cursor: 0 }`
//!       is pushed (`tasks.len() + 1`); if `!is_nontail` (tail), the top task
//!       is replaced in place with
//!       `ValidateTask { r: child_r, visit: TaskVisitC::Unvisited, cursor: 0 }`
//!       (`tasks.len()` unchanged).
//!     * Under `StepDisposition::RefusedHelperWorkUncharged` (`p3`),
//!       `post.outcome == OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }`
//!       and all counters, `helper`, `result`, and `tasks` are preserved
//!       unchanged without charge.
//!     * Under `StepDisposition::RefusedCapCharged` (`p6`), P4 charges
//!       `remaining_work - 1`, `consumed_work + 1`, and `helper_work + 1`,
//!       `helper_visits` is unchanged,
//!       `post.outcome == OutcomeC::Exhausted { reason: ExhaustReason::HelperTasks }`,
//!       and `helper`, `result`, and `tasks` are preserved completely
//!       unchanged (no parent rewrite and no child push).
//!
//! Receipt gating and host-stage honesty:
//! * On `C6ValidateFieldStepBlock`, `reader`, `top_task_found`, `top_task_tag`,
//!   `top_ref_is_data`, `top_ref_index`, `top_task_visit`, `top_task_cursor`,
//!   `field_pos`, `field_found`, `child_ref_is_data`, `child_ref_index`,
//!   `nominal_valid` (gated by `enable && reader.is_constructor`),
//!   `checked_arity`, `next_cursor`, and `is_nontail` are gated by `enable`
//!   (`0` when `enable == 0`), whereas `nominal_capture`, `const_field_count`,
//!   `const_field_found`, `const_field_value`, `data_field_count`,
//!   `data_field_found`, `data_field_is_data`, `data_field_index`,
//!   `is_helper_work_max`, `is_helper_tasks_refused`, `is_applied`,
//!   `post_task_count`, `post_top_task_tag`, `post_top_ref_is_data`,
//!   `post_top_ref_index`, `post_top_task_visit`, `post_top_task_cursor`, and
//!   `post_rewritten_parent_cursor` are UNGATED derived wires and are never
//!   bound as post-state truth. `LedgerCall` output binds only the
//!   constrained `post_image`, `post_data`, and `post_helper` operands.
//! * `witness_from_host_pair` accepts an INDEPENDENT `post` snapshot. Only
//!   host-prefix (`META`/`PROG`/`CTX`) mismatch and padded state
//!   encoder/shape refusals are host-stage errors here; semantically wrong
//!   `post` snapshots that encode reach `check_c6_validate_field_step_circuit`
//!   and are rejected there by SDK `ProofPreimage::check` preprocessing.
//! * `CompleteStepRecord` is publicly constructible: `witness_from_owner_record`
//!   validates only internal event/snapshot consistency and does not
//!   authenticate owner provenance. Actual owner provenance exists only
//!   through `record_validate_field_step`, which calls the fixed owner
//!   `record_step` at most once (without rollback on `Err`).
//! * `ProofPreimage::check` is SDK preprocessing only (not constraint
//!   synthesis, proving, MockProver, or native execution). `CHECK_KEY_LOCATION`
//!   is diagnostic only. Current native ELF does not contain this module;
//!   `SI07`/`SI08`/`SI17`/`SI21`/`SI22`, `V6`, full `StepInv`, downstream
//!   `Validate` / `EvalLiteral` rules, program admission, and native P1
//!   proof issuance remain open.

use std::sync::Arc;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits, CTX_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, ConstantSnapshot, DataSnapshot, ExpressionSnapshot,
    HostSnapshot, ImageSnapshot, ObservedStep, record_step,
};
use mori_core::sherlock::helper_machine::shared::{
    Admitted, AllocDeltasC, BoundaryKind, ControlC, EventKind, ExhaustReason, FrameRowC,
    HelperControlC, HelperKind, HelperPhaseSummaryC, HelperTaskC, LocatorC, MachineImage, OutcomeC,
    OutcomeTagC, RefC, RuleIdC, SiteTag, StepDisposition, TaskVisitC, TriStatus,
};
use serde_json::json;
use thiserror::Error;

use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStatementBindingMode, C6PushStepError,
};
use super::c6_validate_field_step::{
    C6ValidateFieldStepError, C6ValidateFieldStepInputs, emit_c6_validate_field_step_block,
};

/// Diagnostic `ProofPreimage::check` key location only; not a native verifier
/// issuer, circuit authentication, or ledger deployment identifier.
pub(super) const CHECK_KEY_LOCATION: &str = "sherlock_c6_validate_field_step_check";

#[derive(Debug, Error)]
pub enum C6ValidateFieldConsumerError {
    #[error("host prefix section {section} mismatch between pre and post state")]
    HostPrefixMismatch { section: &'static str },
    #[error("owner step is not the selected ValidateField step (terminal or wrong rule)")]
    UnexpectedOwnerStep,
    #[error("owner event mismatch in {field}")]
    EventMismatch { field: &'static str },
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch {
        section: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("C6 ValidateField emitter error: {0}")]
    ValidateField(#[from] C6ValidateFieldStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ValidateFieldConsumerError {
    C6ValidateFieldConsumerError::Backend(e.to_string())
}

fn violation_label(v: &mori_core::sherlock::helper_machine::shared::HostViolation) -> String {
    format!(
        "kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind,
        v.clause,
        v.rule,
        v.locator.as_deref().unwrap_or("<none>")
    )
}

pub(super) fn capture_step_error_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) => {
            format!("before observation: {}", violation_label(v))
        }
        CaptureStepError::Step {
            violation, after, ..
        } => match after {
            Ok(_) => format!(
                "owner step: {}; after snapshot captured",
                violation_label(violation)
            ),
            Err(after_violation) => format!(
                "owner step: {}; after observation: {}",
                violation_label(violation),
                violation_label(after_violation)
            ),
        },
        CaptureStepError::AfterObservation {
            violation, event, ..
        } => format!(
            "after observation: {}; event_present={}",
            violation_label(violation),
            event.is_some()
        ),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct C6ValidateFieldStepWitness {
    pub(super) enable: bool,
    pub(super) disposition_is_refused_helper_work: bool,
    pub(super) disposition_is_refused_helper_tasks: bool,
    pub(super) source_params: C6PushFamilyParams,
    pub(super) host_prefix: C6PushHostPrefixLanes,
    pub(super) pre_image: Vec<u128>,
    pub(super) pre_data: Vec<u128>,
    pub(super) pre_helper: Vec<u128>,
    pub(super) post_image: Vec<u128>,
    pub(super) post_data: Vec<u128>,
    pub(super) post_helper: Vec<u128>,
}

impl C6ValidateFieldStepWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn enable(&self) -> bool {
        self.enable
    }
    pub fn disposition_is_refused_helper_work(&self) -> bool {
        self.disposition_is_refused_helper_work
    }
    pub fn disposition_is_refused_helper_tasks(&self) -> bool {
        self.disposition_is_refused_helper_tasks
    }
    pub fn host_prefix(&self) -> &C6PushHostPrefixLanes {
        &self.host_prefix
    }
    pub fn pre_image(&self) -> &[u128] {
        &self.pre_image
    }
    pub fn pre_data(&self) -> &[u128] {
        &self.pre_data
    }
    pub fn pre_helper(&self) -> &[u128] {
        &self.pre_helper
    }
    pub fn post_image(&self) -> &[u128] {
        &self.post_image
    }
    pub fn post_data(&self) -> &[u128] {
        &self.post_data
    }
    pub fn post_helper(&self) -> &[u128] {
        &self.post_helper
    }

    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = Vec::new();
        out.push(Fr::from(self.enable));
        out.push(Fr::from(self.disposition_is_refused_helper_work));
        out.push(Fr::from(self.disposition_is_refused_helper_tasks));
        out.push(Fr::from(C6_HOST_SCHEMA));
        out.push(Fr::from(C6_PADDED_CARRIER_VERSION));
        for l in self
            .host_prefix
            .meta
            .iter()
            .chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.pre_image)
            .chain(&self.pre_data)
            .chain(&self.pre_helper)
        {
            out.push(Fr::from(*l));
        }
        for l in self
            .host_prefix
            .meta
            .iter()
            .chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.post_image)
            .chain(&self.post_data)
            .chain(&self.post_helper)
        {
            out.push(Fr::from(*l));
        }
        out
    }

    pub(super) fn post_flat(&self) -> Vec<u128> {
        let mut v = self.post_image.clone();
        v.extend(&self.post_data);
        v.extend(&self.post_helper);
        v
    }
}

fn split_state(
    state: &ImageSnapshot,
    caps: C6StateCaps,
    limits: C6HostLimits,
    geom: C6PaddedCarrierGeometry,
) -> Result<(Vec<u128>, Vec<u128>, Vec<u128>), C6ValidateFieldConsumerError> {
    let padded = super::c6_push_step::encode_owned_state_to_padded(
        &C6OwnedState::from_image(state),
        caps,
        C6StateLimits {
            max_total_lanes: limits.max_state_lanes,
        },
    )?;
    let flat = padded.to_flat_lanes();
    let i = geom.image_total_lanes;
    let d = i + geom.data_total_lanes;
    Ok((flat[..i].to_vec(), flat[i..d].to_vec(), flat[d..].to_vec()))
}

type EM = C6ValidateFieldConsumerError;
fn em(field: &'static str) -> EM {
    EM::EventMismatch { field }
}

/// Exact Core `owner_site(pre)` derivation (`selected_for` calls `owner_site`
/// during PS selection before P3 and P6).
fn derived_owner_site(before_host: &HostSnapshot) -> Result<SiteTag, EM> {
    let before = &before_host.image;
    Ok(match before.control {
        ControlC::Eval { expr } => match before_host.admission.view.expressions.get(expr) {
            Some(ExpressionSnapshot::Literal { .. }) => SiteTag::EvalLiteral,
            Some(ExpressionSnapshot::Var { .. }) => SiteTag::EvalVar,
            Some(ExpressionSnapshot::Let { .. }) => SiteTag::EvalLet,
            Some(ExpressionSnapshot::Binary { .. }) => SiteTag::EvalBinary,
            Some(ExpressionSnapshot::Unary { .. }) => SiteTag::EvalUnary,
            Some(ExpressionSnapshot::If { .. }) => SiteTag::EvalIf,
            Some(ExpressionSnapshot::Call { .. }) => SiteTag::EvalCall,
            Some(ExpressionSnapshot::Match { .. }) => SiteTag::EvalMatch,
            Some(ExpressionSnapshot::Construct { .. }) => SiteTag::EvalConstruct,
            None => return Err(em("snapshot control expr")),
        },
        ControlC::ScheduleNext { .. } => SiteTag::Push,
        ControlC::Return { .. } => {
            let top = before
                .frames
                .last()
                .ok_or(em("snapshot return top frame"))?;
            match top {
                FrameRowC::LetF { .. } => SiteTag::RetLet,
                FrameRowC::LeftF { .. } => SiteTag::RetLeft,
                FrameRowC::RightF { .. } => SiteTag::RetRight,
                FrameRowC::UnaryF { .. } => SiteTag::RetUnary,
                FrameRowC::IfF { .. } => SiteTag::RetIf,
                FrameRowC::MatchF { .. } => SiteTag::RetMatch,
                FrameRowC::ArgsF { .. } => SiteTag::RetArgs,
                FrameRowC::ReturnF { .. } => SiteTag::RetCall,
                FrameRowC::ScopeF { .. } => SiteTag::RetScope,
                FrameRowC::ConstructorFieldsF { .. } => SiteTag::RetCtorFields,
            }
        }
        ControlC::Invoke { .. } => SiteTag::Invoke,
        ControlC::Enter { .. } => SiteTag::Enter,
        ControlC::Bound { .. } => SiteTag::Bound,
        ControlC::Cases { .. } => SiteTag::Cases,
        ControlC::CaseResult { .. } => SiteTag::CaseResult,
        ControlC::PrimResult { .. } => SiteTag::Prim,
        ControlC::Materialize { .. } => SiteTag::Materialize,
        ControlC::Error { .. } => return Err(em("snapshot control before")),
    })
}

/// Exact Core nominal constructor declaration check (`adt_checked_ctor_arity`):
/// returns the declared `crow.field_len` and never reads `fields.len` from the
/// constant or data value row.
fn nominal_ctor_checked_arity(
    before: &HostSnapshot,
    type_id: usize,
    ctor_id: usize,
) -> Result<usize, EM> {
    if type_id < 4 {
        return Err(em("snapshot nominal type_id"));
    }
    let view = &before.admission.view;
    let trow = view
        .types
        .get(type_id)
        .ok_or(em("snapshot nominal type row"))?;
    let crow = view
        .constructors
        .get(ctor_id)
        .ok_or(em("snapshot nominal ctor row"))?;
    if crow.owning_type.0 != type_id {
        return Err(em("snapshot nominal owning_type"));
    }
    if ctor_id < trow.ctor_start.0 || ctor_id - trow.ctor_start.0 >= trow.ctor_len {
        return Err(em("snapshot nominal ctor range"));
    }
    let end = crow
        .field_start
        .checked_add(crow.field_len)
        .ok_or(em("snapshot nominal field end"))?;
    if end > view.ctor_fields.len() {
        return Err(em("snapshot nominal field end"));
    }
    Ok(crow.field_len)
}

/// PS `checked_arity(kind_of(top_r))`, evaluated before P3 and P6 in Core `owner_step`.
fn checked_arity_of_ref(before: &HostSnapshot, r: RefC) -> Result<usize, EM> {
    match r {
        RefC::Const(id) => match before.admission.view.constants.get(id) {
            Some(ConstantSnapshot::U128 { .. })
            | Some(ConstantSnapshot::Bool { .. })
            | Some(ConstantSnapshot::Nil { .. }) => Ok(0),
            Some(ConstantSnapshot::Leaf { .. }) => Ok(1),
            Some(ConstantSnapshot::Cons { .. }) | Some(ConstantSnapshot::Branch { .. }) => Ok(2),
            Some(ConstantSnapshot::Constructor {
                type_id, ctor_id, ..
            }) => nominal_ctor_checked_arity(before, type_id.0, ctor_id.0),
            Some(ConstantSnapshot::U128OutOfRange { .. }) => Err(em("snapshot marker top task")),
            None => Err(em("snapshot top task ref")),
        },
        RefC::Data(id) => match before.image.data.get(id) {
            Some(DataSnapshot::U128 { .. })
            | Some(DataSnapshot::Bool { .. })
            | Some(DataSnapshot::Nil) => Ok(0),
            Some(DataSnapshot::Leaf { .. }) => Ok(1),
            Some(DataSnapshot::Cons { .. }) | Some(DataSnapshot::Branch { .. }) => Ok(2),
            Some(DataSnapshot::Constructor {
                type_id, ctor_id, ..
            }) => nominal_ctor_checked_arity(before, type_id.0, ctor_id.0),
            None => Err(em("snapshot top task ref")),
        },
    }
}

/// PS `field_at(&top_r, &top_cursor)` plus `SC-05` (`check_written_code_ids`)
/// constant-table bound on the resulting `child_r`, evaluated before P3 and P6
/// in Core `owner_step`.
fn field_at_of_ref(before: &HostSnapshot, r: RefC, i: usize) -> Result<RefC, EM> {
    let child_r = match r {
        RefC::Const(id) => {
            let row = before
                .admission
                .view
                .constants
                .get(id)
                .ok_or(em("snapshot top task ref"))?;
            let (start, len) = match row {
                ConstantSnapshot::Cons { fields, .. }
                | ConstantSnapshot::Leaf { fields, .. }
                | ConstantSnapshot::Branch { fields, .. }
                | ConstantSnapshot::Constructor { fields, .. } => (fields.start, fields.len),
                ConstantSnapshot::U128 { .. }
                | ConstantSnapshot::Bool { .. }
                | ConstantSnapshot::Nil { .. }
                | ConstantSnapshot::U128OutOfRange { .. } => (0, 0),
            };
            if i >= len {
                return Err(em("snapshot const field index"));
            }
            let pos = start.checked_add(i).ok_or(em("snapshot const field pos"))?;
            let child_cid = *before
                .admission
                .view
                .constant_fields
                .get(pos)
                .ok_or(em("snapshot constant_fields"))?;
            RefC::Const(child_cid)
        }
        RefC::Data(id) => {
            let row = before
                .image
                .data
                .get(id)
                .ok_or(em("snapshot top task ref"))?;
            let (start, len) = match row {
                DataSnapshot::Cons { fields }
                | DataSnapshot::Leaf { fields }
                | DataSnapshot::Branch { fields }
                | DataSnapshot::Constructor { fields, .. } => (fields.start, fields.len),
                DataSnapshot::U128 { .. } | DataSnapshot::Bool { .. } | DataSnapshot::Nil => (0, 0),
            };
            if i >= len {
                return Err(em("snapshot data field index"));
            }
            let pos = start.checked_add(i).ok_or(em("snapshot data field pos"))?;
            *before
                .image
                .data_fields
                .get(pos)
                .ok_or(em("snapshot data_fields"))?
        }
    };
    if let RefC::Const(cid) = child_r {
        if cid >= before.admission.view.constants.len() {
            return Err(em("snapshot child const id"));
        }
    }
    Ok(child_r)
}

/// Independent-post route: no recorder runs; a semantically wrong post
/// that encodes reaches the circuit checker. The P3 (`RefusedHelperWorkUncharged`)
/// and P6 (`RefusedCapCharged` / `HelperTasks`) disposition lanes derive
/// strictly from `pre` against the ORIGINAL separately retained admission
/// bounds carried by `params.admission_bounds()`, never from layout bounds,
/// post labels, or any host trusted flag.
pub fn witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ValidateFieldStepWitness, C6ValidateFieldConsumerError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry();
    let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pp.meta != qp.meta {
        return Err(C6ValidateFieldConsumerError::HostPrefixMismatch { section: "meta" });
    }
    if pp.prog != qp.prog {
        return Err(C6ValidateFieldConsumerError::HostPrefixMismatch { section: "prog" });
    }
    if pp.ctx != qp.ctx {
        return Err(C6ValidateFieldConsumerError::HostPrefixMismatch { section: "ctx" });
    }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    let adm = params.admission_bounds();
    let p3 = pre.image.helper_work == adm.max_helper_work;
    let would_p6 = match pre.image.tasks.last() {
        Some(HelperTaskC::ValidateTask {
            r,
            visit: TaskVisitC::Fields,
            cursor,
        }) => {
            checked_arity_of_ref(pre, *r)
                .ok()
                .and_then(|arity| cursor.checked_add(1).map(|next| next < arity))
                .unwrap_or(false)
                && pre
                    .image
                    .tasks
                    .len()
                    .checked_add(1)
                    .map_or(true, |n| n > adm.helper_tasks)
        }
        _ => false,
    };
    let p6 = !p3 && would_p6;
    Ok(C6ValidateFieldStepWitness {
        enable,
        disposition_is_refused_helper_work: p3,
        disposition_is_refused_helper_tasks: p6,
        source_params: params,
        host_prefix: pp,
        pre_image,
        pre_data,
        pre_helper,
        post_image,
        post_data,
        post_helper,
    })
}

fn validate_owner_record(r: &CompleteStepRecord) -> Result<(), EM> {
    let e = &r.event;
    let before_host = &r.before;
    let after_host = &r.after;
    let before = &before_host.image;
    let after = &after_host.image;
    let bounds = before_host.admission.bounds;

    if e.rule_id != RuleIdC::BHelperValidateField {
        return Err(EM::UnexpectedOwnerStep);
    }
    let applied = e.disposition == StepDisposition::Applied;
    let p3 = e.disposition == StepDisposition::RefusedHelperWorkUncharged;
    let p6 = e.disposition == StepDisposition::RefusedCapCharged;
    if !applied && !p3 && !p6 {
        return Err(em("disposition"));
    }
    if e.event_kind != EventKind::HelperVisit {
        return Err(em("event_kind"));
    }
    if e.helper_kind != Some(HelperKind::Validate) {
        return Err(em("helper_kind"));
    }
    if e.reference_cursor_delta != 0 {
        return Err(em("reference_cursor_delta"));
    }
    if e.alloc_deltas
        != (AllocDeltasC {
            data_nodes: 0,
            data_fields: 0,
            environment_cells: 0,
        })
    {
        return Err(em("alloc deltas"));
    }
    if e.value_slot_writes != 0 {
        return Err(em("value_slot_writes"));
    }
    if e.result_tag.is_some() {
        return Err(em("result_tag"));
    }

    // Selected pre-state & PS checks (evaluated before P3 and P6 in Core `owner_step`).
    if before.outcome != OutcomeC::Running {
        return Err(em("snapshot outcome before"));
    }
    if before.remaining_work == 0 {
        return Err(em("snapshot remaining_work"));
    }
    let derived_site = derived_owner_site(before_host)?;
    let common = match &before.helper {
        Some(HelperControlC::Validate {
            common,
            status: TriStatus::Pending,
            ..
        }) if common.parent_resume.is_none() => *common,
        _ => return Err(em("snapshot helper before")),
    };
    if before.tasks.is_empty() {
        return Err(em("snapshot tasks before"));
    }
    let (top_r, top_cursor) = match before.tasks.last() {
        Some(HelperTaskC::ValidateTask {
            r,
            visit: TaskVisitC::Fields,
            cursor,
        }) => (*r, *cursor),
        _ => return Err(em("snapshot top task before")),
    };
    let child_r = field_at_of_ref(before_host, top_r, top_cursor)?;
    let checked_arity = checked_arity_of_ref(before_host, top_r)?;
    let next_cursor = top_cursor
        .checked_add(1)
        .ok_or(em("snapshot validate cursor"))?;
    let is_nontail = next_cursor < checked_arity;

    // P3 vs P6 vs P7 precedence against original admission bounds.
    let at_hw_max = before.helper_work == bounds.max_helper_work;
    let would_p6 = is_nontail
        && before
            .tasks
            .len()
            .checked_add(1)
            .map_or(true, |n| n > bounds.helper_tasks);
    let want_p3 = at_hw_max;
    let want_p6 = !at_hw_max && would_p6;
    let want_applied = !want_p3 && !want_p6;
    if p3 != want_p3 {
        return Err(em("snapshot disposition p3"));
    }
    if p6 != want_p6 {
        return Err(em("snapshot disposition p6"));
    }
    if applied != want_applied {
        return Err(em("snapshot disposition applied"));
    }

    let want_after_tasks_len = if applied && is_nontail {
        before.tasks.len() + 1
    } else {
        before.tasks.len()
    };

    if e.helper_phase_before != Some(HelperPhaseSummaryC::Tri(TriStatus::Pending))
        || e.helper_cursor_before != Some(before.tasks.len())
    {
        return Err(em("event helper before"));
    }
    if e.helper_phase_after != Some(HelperPhaseSummaryC::Tri(TriStatus::Pending))
        || e.helper_cursor_after != Some(want_after_tasks_len)
    {
        return Err(em("event helper after"));
    }

    if applied {
        if e.work_delta != 1 || e.helper_work_delta != 1 {
            return Err(em("applied deltas"));
        }
        if e.helper_visits_delta != 1 {
            return Err(em("helper_visits_delta"));
        }
        if e.exhausted_kind.is_some()
            || e.outcome_tag != OutcomeTagC::Running
            || e.outcome_payload != OutcomeC::Running
            || e.boundary != BoundaryKind::None
        {
            return Err(em("applied outcome"));
        }
    } else if p3 {
        if e.work_delta != 0 || e.helper_work_delta != 0 {
            return Err(em("p3 deltas"));
        }
        if e.helper_visits_delta != 0 {
            return Err(em("helper_visits_delta"));
        }
        if e.exhausted_kind != Some(ExhaustReason::HelperWork) {
            return Err(em("p3 exhausted_kind"));
        }
        if e.outcome_tag != OutcomeTagC::Exhausted
            || e.outcome_payload
                != (OutcomeC::Exhausted {
                    reason: ExhaustReason::HelperWork,
                })
            || e.boundary != BoundaryKind::Terminal
        {
            return Err(em("p3 outcome"));
        }
    } else {
        if e.work_delta != 1 || e.helper_work_delta != 1 {
            return Err(em("p6 deltas"));
        }
        if e.helper_visits_delta != 0 {
            return Err(em("helper_visits_delta"));
        }
        if e.exhausted_kind != Some(ExhaustReason::HelperTasks) {
            return Err(em("p6 exhausted_kind"));
        }
        if e.outcome_tag != OutcomeTagC::Exhausted
            || e.outcome_payload
                != (OutcomeC::Exhausted {
                    reason: ExhaustReason::HelperTasks,
                })
            || e.boundary != BoundaryKind::Terminal
        {
            return Err(em("p6 outcome"));
        }
    }

    // Event-to-snapshot binding (host-record consistency only).
    if e.step_id.ordinal != before.consumed_work || *e.step_id.run_id != before.run_id {
        return Err(em("snapshot step_id"));
    }
    if e.site != Some(derived_site) {
        return Err(em("event site"));
    }
    if e.origin_reference_cursor != Some(common.origin_reference_cursor) {
        return Err(em("event origin_reference_cursor"));
    }
    if e.source_expr_id != before.control_origin {
        return Err(em("event source_expr_id"));
    }
    {
        let occ = before_host
            .admission
            .view
            .occurrences
            .get(before.control_origin)
            .ok_or(em("snapshot source occurrence"))?;
        let ast_path = if occ.path.len == 0 {
            if occ.path.start != 0 {
                return Err(em("snapshot source occurrence"));
            }
            Vec::new()
        } else {
            let end = occ
                .path
                .start
                .checked_add(occ.path.len)
                .ok_or(em("snapshot source occurrence"))?;
            before_host
                .admission
                .view
                .occurrence_paths
                .get(occ.path.start..end)
                .ok_or(em("snapshot source occurrence"))?
                .to_vec()
        };
        if e.source_occurrence
            != (LocatorC::Occurrence {
                owner_is_function: occ.owner_is_function,
                function: occ.function,
                ast_path,
            })
        {
            return Err(em("event source_occurrence"));
        }
    }
    if e.logical_depth_before != before.frames.len() || e.logical_depth_after != after.frames.len()
    {
        return Err(em("snapshot depth"));
    }
    if e.task_cursor_before != before.tasks.len() || e.task_cursor_after != after.tasks.len() {
        return Err(em("snapshot task cursor"));
    }
    if after.reference_cursor != before.reference_cursor {
        return Err(em("snapshot reference_cursor"));
    }
    if after.helper != before.helper {
        return Err(em("snapshot helper after"));
    }
    if after.result != before.result {
        return Err(em("snapshot result"));
    }
    if after_host.admission != before_host.admission
        || after.run_id != before.run_id
        || after.control != before.control
        || after.control_origin != before.control_origin
        || after.env_root != before.env_root
        || after.stack_limit != before.stack_limit
        || after.initial_fuel != before.initial_fuel
        || after.seg_base != before.seg_base
        || after.frames != before.frames
        || after.env != before.env
        || after.value_slots != before.value_slots
        || after.data != before.data
        || after.data_fields != before.data_fields
        || after.alloc != before.alloc
        || after.active != before.active
    {
        return Err(em("snapshot preservation"));
    }

    if applied {
        let want_remaining = before
            .remaining_work
            .checked_sub(1)
            .ok_or(em("snapshot remaining_work"))?;
        if after.remaining_work != want_remaining {
            return Err(em("snapshot remaining_work"));
        }
        let want_consumed = before
            .consumed_work
            .checked_add(1)
            .ok_or(em("snapshot consumed_work"))?;
        if after.consumed_work != want_consumed {
            return Err(em("snapshot consumed_work"));
        }
        let want_hw = before
            .helper_work
            .checked_add(1)
            .ok_or(em("snapshot helper_work"))?;
        if after.helper_work != want_hw {
            return Err(em("snapshot helper_work"));
        }
        let want_visits = before
            .helper_visits
            .checked_add(1)
            .ok_or(em("snapshot helper_visits"))?;
        if after.helper_visits != want_visits {
            return Err(em("snapshot helper_visits"));
        }
        if after.outcome != OutcomeC::Running {
            return Err(em("snapshot outcome"));
        }
        let prefix_len = before.tasks.len() - 1;
        let want_child_task = HelperTaskC::ValidateTask {
            r: child_r,
            visit: TaskVisitC::Unvisited,
            cursor: 0,
        };
        if is_nontail {
            let want_parent_task = HelperTaskC::ValidateTask {
                r: top_r,
                visit: TaskVisitC::Fields,
                cursor: next_cursor,
            };
            if after.tasks.len() != before.tasks.len() + 1
                || after.tasks[..prefix_len] != before.tasks[..prefix_len]
                || after.tasks.get(prefix_len) != Some(&want_parent_task)
                || after.tasks.last() != Some(&want_child_task)
            {
                return Err(em("snapshot tasks after"));
            }
        } else if after.tasks.len() != before.tasks.len()
            || after.tasks[..prefix_len] != before.tasks[..prefix_len]
            || after.tasks.last() != Some(&want_child_task)
        {
            return Err(em("snapshot tasks after"));
        }
    } else if p3 {
        if after.remaining_work != before.remaining_work
            || after.consumed_work != before.consumed_work
            || after.helper_work != before.helper_work
            || after.helper_visits != before.helper_visits
        {
            return Err(em("snapshot p3 counters"));
        }
        if after.tasks != before.tasks {
            return Err(em("snapshot p3 tasks"));
        }
        if after.outcome
            != (OutcomeC::Exhausted {
                reason: ExhaustReason::HelperWork,
            })
        {
            return Err(em("snapshot p3 outcome"));
        }
    } else {
        let want_remaining = before
            .remaining_work
            .checked_sub(1)
            .ok_or(em("snapshot p6 remaining_work"))?;
        let want_consumed = before
            .consumed_work
            .checked_add(1)
            .ok_or(em("snapshot p6 consumed_work"))?;
        let want_hw = before
            .helper_work
            .checked_add(1)
            .ok_or(em("snapshot p6 helper_work"))?;
        if after.remaining_work != want_remaining
            || after.consumed_work != want_consumed
            || after.helper_work != want_hw
            || after.helper_visits != before.helper_visits
        {
            return Err(em("snapshot p6 counters"));
        }
        if after.tasks != before.tasks {
            return Err(em("snapshot p6 tasks"));
        }
        if after.outcome
            != (OutcomeC::Exhausted {
                reason: ExhaustReason::HelperTasks,
            })
        {
            return Err(em("snapshot p6 outcome"));
        }
    }
    Ok(())
}

/// Build a witness from one provided complete record. No step is taken.
/// This validates only internal consistency of the provided event
/// against the selected `BHelperValidateField` relation; it does not
/// authenticate that a real owner produced the record.
pub fn witness_from_owner_record(
    record: &CompleteStepRecord,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ValidateFieldStepWitness, C6ValidateFieldConsumerError> {
    validate_owner_record(record)?;
    witness_from_host_pair(true, &record.before, &record.after, cfg, limits)
}

/// Wrapper around `complete_observation::record_step`, called at most
/// once; no pre-witness or double step.
///
/// Non-rollback contract: the one owner step is taken before this
/// wrapper can refuse. An `Err` return can therefore leave `image`
/// already advanced and mutated by that one step. No rollback is
/// performed or guaranteed, and callers must not interpret `Err` as
/// "no step happened". Only success returns a witness for the selected
/// step.
pub fn record_validate_field_step(
    admitted: &Admitted,
    image: &mut MachineImage,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ValidateFieldStepWitness, C6ValidateFieldConsumerError> {
    match record_step(admitted, image) {
        Ok(ObservedStep::Event(record)) => witness_from_owner_record(&record, cfg, limits),
        Ok(ObservedStep::Terminal { .. }) => Err(C6ValidateFieldConsumerError::UnexpectedOwnerStep),
        Err(e) => Err(C6ValidateFieldConsumerError::Backend(format!(
            "owner recorder failed: {}",
            capture_step_error_label(&e)
        ))),
    }
}

#[derive(Clone, Debug)]
pub struct C6ValidateFieldStepCircuit {
    pub(super) mode: C6PushStatementBindingMode,
    pub(super) geometry: C6PaddedCarrierGeometry,
    pub(super) meta_len: usize,
    pub(super) prog_len: usize,
    pub(super) meta_bounds_frames_offset: usize,
    pub(super) source_params: C6PushFamilyParams,
    pub(super) statement_lane_count: usize,
    pub(super) ir: IrSource,
    pub(super) serialized_zkir: String,
}

impl C6ValidateFieldStepCircuit {
    pub fn mode(&self) -> C6PushStatementBindingMode {
        self.mode
    }
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn geometry(&self) -> C6PaddedCarrierGeometry {
        self.geometry
    }
    pub fn meta_len(&self) -> usize {
        self.meta_len
    }
    pub fn prog_len(&self) -> usize {
        self.prog_len
    }
    pub fn meta_bounds_frames_offset(&self) -> usize {
        self.meta_bounds_frames_offset
    }
    pub fn statement_lane_count(&self) -> usize {
        self.statement_lane_count
    }
    pub fn ir(&self) -> &IrSource {
        &self.ir
    }
    pub fn serialized_zkir(&self) -> &str {
        &self.serialized_zkir
    }
}

fn make_typed(id: Identifier) -> Result<TypedIdentifier, C6ValidateFieldConsumerError> {
    serde_json::from_value(json!({ "name": id, "type": IrType::Native })).map_err(be)
}

#[allow(clippy::too_many_arguments)]
fn alloc_lane(
    mode: C6PushStatementBindingMode,
    scope: &str,
    label: &str,
    idx: usize,
    inputs: &mut Vec<TypedIdentifier>,
    ins: &mut Vec<Instruction>,
    tr: &mut Vec<Operand>,
) -> Result<Operand, C6ValidateFieldConsumerError> {
    let prefix = match mode {
        C6PushStatementBindingMode::LocalPublic => "public",
        C6PushStatementBindingMode::LedgerCall => "input",
    };
    let id = Identifier(format!("%{scope}.{prefix}.{label}.{idx}"));
    match mode {
        C6PushStatementBindingMode::LocalPublic => ins.push(Instruction::PublicInput {
            guard: None,
            val_t: IrType::Native,
            output: id.clone(),
        }),
        C6PushStatementBindingMode::LedgerCall => inputs.push(make_typed(id.clone())?),
    }
    let op = Operand::Variable(id);
    tr.push(op.clone());
    Ok(op)
}

pub fn build_c6_validate_field_step_circuit(
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    mode: C6PushStatementBindingMode,
    scope: &str,
) -> Result<C6ValidateFieldStepCircuit, C6ValidateFieldConsumerError> {
    if scope.trim().is_empty() {
        return Err(C6ValidateFieldStepError::EmptyScope.into());
    }
    let geometry = source_params.geometry();
    let mut call_inputs = Vec::new();
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut alloc = |label: &str,
                     idx: usize,
                     ci: &mut Vec<TypedIdentifier>,
                     ii: &mut Vec<Instruction>,
                     tt: &mut Vec<Operand>|
     -> Result<Operand, C6ValidateFieldConsumerError> {
        alloc_lane(mode, scope, label, idx, ci, ii, tt)
    };
    let enable = alloc("enable", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let disp_hw = alloc(
        "disposition_is_refused_helper_work",
        0,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let disp_ht = alloc(
        "disposition_is_refused_helper_tasks",
        0,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let schema = alloc("host_schema", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let carrier = alloc("carrier_version", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let mut vec_of = |label: &str,
                      n: usize,
                      ci: &mut Vec<TypedIdentifier>,
                      ii: &mut Vec<Instruction>,
                      tt: &mut Vec<Operand>|
     -> Result<Vec<Operand>, C6ValidateFieldConsumerError> {
        let mut o = Vec::with_capacity(n);
        for i in 0..n {
            o.push(alloc(label, i, ci, ii, tt)?);
        }
        Ok(o)
    };
    let pre_meta = vec_of("pre_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_prog = vec_of("pre_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx_v = vec_of("pre_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx: [Operand; CTX_LANES] = pre_ctx_v
        .try_into()
        .map_err(|_| C6ValidateFieldConsumerError::Backend("pre_ctx".into()))?;
    let pre_image = vec_of(
        "pre_image",
        geometry.image_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let pre_data = vec_of(
        "pre_data",
        geometry.data_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let pre_helper = vec_of(
        "pre_helper",
        geometry.helper_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_meta = vec_of("post_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_prog = vec_of("post_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx_v = vec_of("post_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx: [Operand; CTX_LANES] = post_ctx_v
        .try_into()
        .map_err(|_| C6ValidateFieldConsumerError::Backend("post_ctx".into()))?;
    let post_image = vec_of(
        "post_image",
        geometry.image_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_data = vec_of(
        "post_data",
        geometry.data_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let post_helper = vec_of(
        "post_helper",
        geometry.helper_total_lanes,
        &mut call_inputs,
        &mut ins,
        &mut tr,
    )?;
    let block = emit_c6_validate_field_step_block(
        &C6ValidateFieldStepInputs {
            enable,
            disposition_is_refused_helper_work: disp_hw,
            disposition_is_refused_helper_tasks: disp_ht,
            host_schema: schema,
            carrier_version: carrier,
            geometry,
            meta_bounds_frames_offset,
            source_params: source_params.clone(),
            pre_meta,
            pre_prog,
            pre_ctx,
            pre_image,
            pre_data,
            pre_helper,
            post_meta,
            post_prog,
            post_ctx,
            post_image: post_image.clone(),
            post_data: post_data.clone(),
            post_helper: post_helper.clone(),
        },
        scope,
    )?;
    // Receipt contract: `block.reader`, `block.top_*`, `block.field_*`,
    // `block.child_*`, `block.nominal_valid`, `block.checked_arity`,
    // `block.next_cursor`, and `block.is_nontail` are gated by `enable`
    // (`0` when `enable == 0`), whereas `block.nominal_capture`,
    // `block.const_field_*`, `block.data_field_*`, `block.is_helper_work_max`,
    // `block.is_helper_tasks_refused`, `block.is_applied`, and `block.post_*`
    // are UNGATED derived candidate/capture wires and are NEVER bound to
    // post-state, output, disposition, or enable truth here. The `LedgerCall`
    // output below binds only the constrained `post_image`, `post_data`, and
    // `post_helper` operands.
    let _ = (
        block.reader.clone(),
        block.nominal_capture.clone(),
        block.const_field_count.clone(),
        block.const_field_found.clone(),
        block.const_field_value.clone(),
        block.data_field_count.clone(),
        block.data_field_found.clone(),
        block.data_field_is_data.clone(),
        block.data_field_index.clone(),
        block.top_task_found.clone(),
        block.top_task_tag.clone(),
        block.top_ref_is_data.clone(),
        block.top_ref_index.clone(),
        block.top_task_visit.clone(),
        block.top_task_cursor.clone(),
        block.field_pos.clone(),
        block.field_found.clone(),
        block.child_ref_is_data.clone(),
        block.child_ref_index.clone(),
        block.nominal_valid.clone(),
        block.checked_arity.clone(),
        block.next_cursor.clone(),
        block.is_nontail.clone(),
        block.is_helper_work_max.clone(),
        block.is_helper_tasks_refused.clone(),
        block.is_applied.clone(),
        block.post_task_count.clone(),
        block.post_top_task_tag.clone(),
        block.post_top_ref_is_data.clone(),
        block.post_top_ref_index.clone(),
        block.post_top_task_visit.clone(),
        block.post_top_task_cursor.clone(),
        block.post_rewritten_parent_cursor.clone(),
    );
    ins.extend(block.instructions);
    let statement_lane_count = tr.len();
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)),
        inputs: tr,
    });
    let outputs = match mode {
        C6PushStatementBindingMode::LocalPublic => Vec::new(),
        C6PushStatementBindingMode::LedgerCall => {
            let mut vals = Vec::with_capacity(geometry.state_total_lanes);
            vals.extend(post_image);
            vals.extend(post_data);
            vals.extend(post_helper);
            ins.push(Instruction::Output { vals });
            vec![IrType::Native; geometry.state_total_lanes]
        }
    };
    let ir0 = IrSource {
        version: IrMinorVersion::V0,
        inputs: call_inputs,
        outputs,
        do_communications_commitment: matches!(mode, C6PushStatementBindingMode::LedgerCall),
        instructions: Arc::new(ins),
    };
    let mut disk = serde_json::to_value(&ir0).map_err(be)?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized_zkir = serde_json::to_string(&disk).map_err(be)?;
    let ir = IrSource::load(serialized_zkir.as_bytes()).map_err(be)?;
    // Instruction count is not degree k / performance qualification.
    Ok(C6ValidateFieldStepCircuit {
        mode,
        geometry,
        meta_len,
        prog_len,
        meta_bounds_frames_offset,
        source_params,
        statement_lane_count,
        ir,
        serialized_zkir,
    })
}

/// `ProofPreimage::check` preprocessing only; not synthesis, proving,
/// MockProver, or native execution. Semantic rejection here is the
/// SDK preprocessing category. `key_location` is diagnostic only:
/// not a native verifier issuer, circuit authentication, or ledger
/// deployment.
pub fn check_c6_validate_field_step_circuit(
    circuit: &C6ValidateFieldStepCircuit,
    witness: &C6ValidateFieldStepWitness,
) -> Result<Vec<Option<usize>>, C6ValidateFieldConsumerError> {
    if witness.source_params.geometry() != circuit.geometry {
        return Err(C6ValidateFieldConsumerError::Backend(
            "witness geometry does not match circuit".into(),
        ));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6ValidateFieldConsumerError::LengthMismatch {
            section: "meta",
            expected: circuit.meta_len,
            actual: witness.host_prefix.meta.len(),
        });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6ValidateFieldConsumerError::LengthMismatch {
            section: "prog",
            expected: circuit.prog_len,
            actual: witness.host_prefix.prog.len(),
        });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6ValidateFieldConsumerError::Backend(
            "meta bounds frames offset mismatch".into(),
        ));
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6ValidateFieldConsumerError::LengthMismatch {
            section: "statement",
            expected: circuit.statement_lane_count,
            actual: statement.len(),
        });
    }
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(),
            private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            // Recomputed from the CURRENT statement and post flat lanes
            // on every call, so a mutated statement in a raw test is
            // checked against its own recomputed commitment, never a
            // stale commitment carried from an earlier witness.
            let randomness = Fr::from(0u64);
            let mut comm = statement.clone();
            comm.extend(witness.post_flat().iter().map(|&l| Fr::from(l)));
            ProofPreimage {
                inputs: statement.clone(),
                private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&circuit.ir).map_err(|e| {
        C6ValidateFieldConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}
