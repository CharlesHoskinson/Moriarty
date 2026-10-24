//! C6 `BHelperScalarExit` consumer: typed host-pair / owner-record
//! witnesses, the selected-step wrapper, and the SDK statement
//! builder/checker for the corrected `C6ScalarExitStepEmitter`.
//!
//! Statement order mirrors ScalarFailure: enable,
//! disposition_is_refused_helper_work, host_schema, carrier_version,
//! pre(META, PROG, CTX, image, data, helper), post(same). The P6 operand
//! is derived in-circuit by the emitter from alloc counters, staged
//! deltas and the ORIGINAL SEPARATE admission bounds retained in
//! `C6PushFamilyParams`; it is not a caller flag in this consumer.
//!
//! Host-stage honesty: `witness_from_host_pair` accepts an INDEPENDENT
//! post snapshot. Only host-prefix (META/PROG/CTX) mismatch and padded
//! state encoder/shape refusals are producer/host-stage errors here;
//! semantic wrong-post values that encode reach
//! `check_c6_scalar_exit_step_circuit` and are rejected there by SDK
//! preprocessing. This module never filters semantic wrong posts on the
//! producer route and claims no circuit acceptance from host prechecks.
//! `CompleteStepRecord` is publicly constructible: the owner-record
//! route validates only internal event/snapshot consistency and does
//! not authenticate provenance. Actual owner provenance exists only
//! through `record_scalar_exit_step`, which calls the fixed owner
//! `record_step` exactly once. Full StepInv, SI08/SI21/SI22 denotation,
//! program membership and native P1 issuance remain REQUIRED_OPEN.

use std::sync::Arc;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, Zkir};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6_HOST_SCHEMA, C6HostConfig, C6HostLimits, CTX_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, DataFieldsSnapshot, DataSnapshot, HostSnapshot,
    ImageSnapshot, ObservedStep, record_step,
};
use mori_core::sherlock::helper_machine::shared::{
    Admitted, AllocDeltasC, BoundaryKind, EventKind, ExhaustReason, HelperControlC, HelperKind,
    HelperPhaseSummaryC, HelperResultC, HelperResultTagC, MachineImage, NodeKind, OutcomeC,
    OutcomeTagC, RefC, RuleIdC, ScalarMode, ScalarPhase, ScalarStaged, SiteTag, StepDisposition,
};
use serde_json::json;
use thiserror::Error;

use super::c6_push_step::{
    C6_PADDED_CARRIER_VERSION, C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStatementBindingMode, C6PushStepError,
};
use super::c6_scalar_exit_step::{
    C6ScalarExitStepError, C6ScalarExitStepInputs, emit_c6_scalar_exit_step_block,
};

const CHECK_KEY_LOCATION: &str = "sherlock_c6_scalar_exit_step_check";

#[derive(Debug, Error)]
pub enum C6ScalarExitConsumerError {
    #[error("host prefix section {section} mismatch between pre and post state")]
    HostPrefixMismatch { section: &'static str },
    #[error("owner step is not the selected Scalar exit step (terminal or wrong rule)")]
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
    #[error("C6 ScalarExit emitter error: {0}")]
    ScalarExit(#[from] C6ScalarExitStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ScalarExitConsumerError {
    C6ScalarExitConsumerError::Backend(e.to_string())
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

fn capture_step_error_label(e: &CaptureStepError) -> String {
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
pub struct C6ScalarExitStepWitness {
    enable: bool,
    disposition_is_refused_helper_work: bool,
    source_params: C6PushFamilyParams,
    host_prefix: C6PushHostPrefixLanes,
    pre_image: Vec<u128>,
    pre_data: Vec<u128>,
    pre_helper: Vec<u128>,
    post_image: Vec<u128>,
    post_data: Vec<u128>,
    post_helper: Vec<u128>,
}

impl C6ScalarExitStepWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams {
        &self.source_params
    }
    pub fn enable(&self) -> bool {
        self.enable
    }
    pub fn disposition_is_refused_helper_work(&self) -> bool {
        self.disposition_is_refused_helper_work
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

    fn post_flat(&self) -> Vec<u128> {
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
) -> Result<(Vec<u128>, Vec<u128>, Vec<u128>), C6ScalarExitConsumerError> {
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

/// Independent-post route: no recorder runs; a wrong post that encodes
/// reaches the circuit checker. Prefix mismatch and encoder/shape
/// refusals are host-stage errors, not circuit rejections.
pub fn witness_from_host_pair(
    enable: bool,
    pre: &HostSnapshot,
    post: &HostSnapshot,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ScalarExitStepWitness, C6ScalarExitConsumerError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry();
    let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pp.meta != qp.meta {
        return Err(C6ScalarExitConsumerError::HostPrefixMismatch { section: "meta" });
    }
    if pp.prog != qp.prog {
        return Err(C6ScalarExitConsumerError::HostPrefixMismatch { section: "prog" });
    }
    if pp.ctx != qp.ctx {
        return Err(C6ScalarExitConsumerError::HostPrefixMismatch { section: "ctx" });
    }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    Ok(C6ScalarExitStepWitness {
        enable,
        disposition_is_refused_helper_work: pre.image.helper_work
            == params.admission_bounds().max_helper_work,
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

type EM = C6ScalarExitConsumerError;
fn em(field: &'static str) -> EM {
    EM::EventMismatch { field }
}

/// Staged branch deltas from the pinned Core contract: index the new row
/// by `pre.active.data`, fields start at `pre.active.data_fields`, and
/// the logical P6 cap uses the ALLOCATION counters against the ORIGINAL
/// SEPARATE admission bounds (never physical layout caps, never a flag).
fn staged_delta(
    mode: ScalarMode,
    has_operand_b: bool,
    has_expected: bool,
    staged: &ScalarStaged,
) -> Result<(usize, usize), EM> {
    let binary = matches!(
        mode,
        ScalarMode::Add
            | ScalarMode::Sub
            | ScalarMode::Mul
            | ScalarMode::Lt
            | ScalarMode::Cons
            | ScalarMode::Branch
    );
    if has_operand_b != binary {
        return Err(em("snapshot operand_b presence"));
    }
    if has_expected != (mode == ScalarMode::TestType) {
        return Err(em("snapshot expected presence"));
    }
    Ok(match (staged, mode) {
        (ScalarStaged::U128Value { .. }, ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul) => {
            (1, 0)
        }
        (ScalarStaged::BoolValue { .. }, ScalarMode::Lt | ScalarMode::Not) => (1, 0),
        (
            ScalarStaged::PendingNode {
                tag: NodeKind::Leaf,
            },
            ScalarMode::Leaf,
        ) => (1, 1),
        (
            ScalarStaged::PendingNode {
                tag: NodeKind::Cons,
            },
            ScalarMode::Cons,
        ) => (1, 2),
        (
            ScalarStaged::PendingNode {
                tag: NodeKind::Branch,
            },
            ScalarMode::Branch,
        ) => (1, 2),
        (ScalarStaged::Selector { .. }, ScalarMode::TestBool) => (0, 0),
        (ScalarStaged::TypeTest { .. }, ScalarMode::TestType) => (0, 0),
        _ => return Err(em("snapshot staged/mode coherence")),
    })
}

fn validate_owner_record(r: &CompleteStepRecord) -> Result<(), EM> {
    let e = &r.event;
    let before_host = &r.before;
    let after_host = &r.after;
    let before = &before_host.image;
    let after = &after_host.image;
    let bounds = before_host.admission.bounds;

    let applied = e.disposition == StepDisposition::Applied;
    let p3 = e.disposition == StepDisposition::RefusedHelperWorkUncharged;
    let p6 = e.disposition == StepDisposition::RefusedCapCharged;
    if !applied && !p3 && !p6 {
        return Err(em("disposition"));
    }
    if e.rule_id != RuleIdC::BHelperScalarExit {
        return Err(EM::UnexpectedOwnerStep);
    }
    if e.event_kind != EventKind::HelperExit {
        return Err(em("event_kind"));
    }
    if e.helper_kind != Some(HelperKind::Scalar) {
        return Err(em("helper_kind"));
    }
    if e.helper_visits_delta != 0 {
        return Err(em("helper_visits_delta"));
    }
    if e.reference_cursor_delta != 0 {
        return Err(em("reference_cursor_delta"));
    }
    if e.value_slot_writes != 0 {
        return Err(em("value_slot_writes"));
    }
    if e.logical_depth_before != e.logical_depth_after
        || e.task_cursor_before != e.task_cursor_after
    {
        return Err(em("depth/task cursors"));
    }
    if e.helper_phase_before != Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done))
        || e.helper_cursor_before != Some(2)
    {
        return Err(em("helper phase/cursor before"));
    }

    // Selected pre-state: Scalar, Done, valid staged, exact mode payload.
    let (site, origin_cursor, operand_a, operand_b, staged) = match &before.helper {
        Some(HelperControlC::Scalar {
            common,
            mode,
            operand_a,
            operand_b,
            expected,
            phase: ScalarPhase::Done,
            staged,
            ..
        }) if common.parent_resume.is_none() => {
            let _ = staged_delta(*mode, operand_b.is_some(), expected.is_some(), staged)?;
            (
                common.site,
                common.origin_reference_cursor,
                *operand_a,
                *operand_b,
                *staged,
            )
        }
        _ => return Err(em("snapshot helper before")),
    };
    let (dn, df) = staged_delta(
        match &before.helper {
            Some(HelperControlC::Scalar { mode, .. }) => *mode,
            _ => return Err(em("snapshot helper before")),
        },
        operand_b.is_some(),
        matches!(
            &before.helper,
            Some(HelperControlC::Scalar {
                expected: Some(_),
                ..
            })
        ),
        &staged,
    )?;
    if !matches!(
        site,
        SiteTag::RetRight
            | SiteTag::RetUnary
            | SiteTag::RetIf
            | SiteTag::RetCall
            | SiteTag::RetCtorFields
    ) {
        return Err(em("snapshot site"));
    }
    if before.result.is_some() {
        return Err(em("snapshot result before"));
    }
    if !before.tasks.is_empty() || !after.tasks.is_empty() {
        return Err(em("snapshot tasks"));
    }
    if before.active.data != before.data.len()
        || before.active.data_fields != before.data_fields.len()
    {
        return Err(em("snapshot active lengths"));
    }

    // Logical P6 cap, node before fields, checked arithmetic.
    let node_over = match before.alloc.data_nodes.checked_add(dn) {
        Some(s) => s > bounds.data_nodes,
        None => true,
    };
    let field_over = match before.alloc.data_fields.checked_add(df) {
        Some(s) => s > bounds.data_field_slots,
        None => true,
    };
    let cap_reason = if node_over {
        Some(ExhaustReason::DataNodes)
    } else if field_over {
        Some(ExhaustReason::DataFields)
    } else {
        None
    };
    let at_max = before.helper_work == bounds.max_helper_work;

    if applied {
        if e.work_delta != 1 || e.helper_work_delta != 1 {
            return Err(em("applied deltas"));
        }
        let want_tag = match staged {
            ScalarStaged::U128Value { .. }
            | ScalarStaged::BoolValue { .. }
            | ScalarStaged::PendingNode { .. } => HelperResultTagC::ScalarValue,
            ScalarStaged::Selector { .. } => HelperResultTagC::ScalarSelector,
            ScalarStaged::TypeTest { .. } => HelperResultTagC::ScalarTypeTest,
            _ => return Err(em("applied result_tag")),
        };
        if e.result_tag != Some(want_tag) {
            return Err(em("applied result_tag"));
        }
        if e.alloc_deltas
            != (AllocDeltasC {
                data_nodes: dn as i64,
                data_fields: df as i64,
                environment_cells: 0,
            })
        {
            return Err(em("applied alloc deltas"));
        }
        if e.exhausted_kind.is_some()
            || e.outcome_tag != OutcomeTagC::Running
            || e.outcome_payload != OutcomeC::Running
            || e.boundary != BoundaryKind::None
        {
            return Err(em("applied outcome"));
        }
        if e.helper_phase_after.is_some() || e.helper_cursor_after.is_some() {
            return Err(em("applied helper after"));
        }
    } else {
        if e.result_tag.is_some() {
            return Err(em("refused result_tag"));
        }
        if e.alloc_deltas
            != (AllocDeltasC {
                data_nodes: 0,
                data_fields: 0,
                environment_cells: 0,
            })
        {
            return Err(em("refused alloc deltas"));
        }
        if e.helper_phase_after != Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done))
            || e.helper_cursor_after != Some(2)
        {
            return Err(em("refused helper after"));
        }
        if p3 {
            if e.work_delta != 0 || e.helper_work_delta != 0 {
                return Err(em("p3 deltas"));
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
            let reason = cap_reason.ok_or(em("p6 exhausted_kind"))?;
            if e.exhausted_kind != Some(reason) {
                return Err(em("p6 exhausted_kind"));
            }
            if e.outcome_tag != OutcomeTagC::Exhausted
                || e.outcome_payload != (OutcomeC::Exhausted { reason })
                || e.boundary != BoundaryKind::Terminal
            {
                return Err(em("p6 outcome"));
            }
        }
    }

    // Snapshot binding (checked arithmetic only; no panics).
    if before.remaining_work == 0 {
        return Err(em("snapshot remaining_work"));
    }
    if p3 != at_max {
        return Err(em("snapshot disposition p3"));
    }
    if applied && (at_max || cap_reason.is_some()) {
        return Err(em("snapshot disposition applied"));
    }
    if p6 && (at_max || cap_reason.is_none()) {
        return Err(em("snapshot disposition p6"));
    }
    if e.step_id.ordinal != before.consumed_work || *e.step_id.run_id != before.run_id {
        return Err(em("snapshot step_id"));
    }
    if e.logical_depth_before != before.frames.len() || e.logical_depth_after != after.frames.len()
    {
        return Err(em("snapshot depth"));
    }
    if e.task_cursor_before != before.tasks.len() || e.task_cursor_after != after.tasks.len() {
        return Err(em("snapshot task cursor"));
    }
    if after.helper_visits != before.helper_visits {
        return Err(em("snapshot helper_visits"));
    }
    if after.reference_cursor != before.reference_cursor {
        return Err(em("snapshot reference_cursor"));
    }
    if e.site != Some(site) {
        return Err(em("snapshot site"));
    }
    if e.origin_reference_cursor != Some(origin_cursor) {
        return Err(em("snapshot origin_reference_cursor"));
    }
    if e.source_expr_id != before.control_origin {
        return Err(em("snapshot source_expr_id"));
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
        || after.tasks != before.tasks
    {
        return Err(em("snapshot preservation"));
    }

    let charged = applied || p6;
    if charged {
        let want = before
            .remaining_work
            .checked_sub(1)
            .ok_or(em("snapshot remaining_work"))?;
        if after.remaining_work != want {
            return Err(em("snapshot remaining_work"));
        }
        let want = before
            .consumed_work
            .checked_add(1)
            .ok_or(em("snapshot consumed_work"))?;
        if after.consumed_work != want {
            return Err(em("snapshot consumed_work"));
        }
        let want = before
            .helper_work
            .checked_add(1)
            .ok_or(em("snapshot helper_work"))?;
        if after.helper_work != want {
            return Err(em("snapshot helper_work"));
        }
    } else {
        if after.remaining_work != before.remaining_work
            || after.consumed_work != before.consumed_work
            || after.helper_work != before.helper_work
        {
            return Err(em("snapshot p3 counters"));
        }
    }

    if applied {
        let want = before
            .alloc
            .data_nodes
            .checked_add(dn)
            .ok_or(em("snapshot alloc"))?;
        if after.alloc.data_nodes != want {
            return Err(em("snapshot alloc nodes"));
        }
        let want = before
            .alloc
            .data_fields
            .checked_add(df)
            .ok_or(em("snapshot alloc"))?;
        if after.alloc.data_fields != want {
            return Err(em("snapshot alloc fields"));
        }
        if after.alloc.environment_cells != before.alloc.environment_cells {
            return Err(em("snapshot alloc env"));
        }
        let want = before
            .active
            .data
            .checked_add(dn)
            .ok_or(em("snapshot active"))?;
        if after.active.data != want {
            return Err(em("snapshot active data"));
        }
        let want = before
            .active
            .data_fields
            .checked_add(df)
            .ok_or(em("snapshot active"))?;
        if after.active.data_fields != want {
            return Err(em("snapshot active fields"));
        }
        if after.active.env != before.active.env
            || after.active.value_slots != before.active.value_slots
        {
            return Err(em("snapshot active others"));
        }
        if after.helper.is_some() {
            return Err(em("snapshot helper after"));
        }
        if after.outcome != OutcomeC::Running || e.outcome_payload != after.outcome {
            return Err(em("snapshot outcome"));
        }
        let want_result = match staged {
            ScalarStaged::U128Value { .. }
            | ScalarStaged::BoolValue { .. }
            | ScalarStaged::PendingNode { .. } => HelperResultC::ScalarValue {
                value: RefC::Data(before.active.data),
            },
            ScalarStaged::Selector { sel } => HelperResultC::ScalarSelector { sel },
            ScalarStaged::TypeTest { ok } => HelperResultC::ScalarTypeTest { ok },
            _ => return Err(em("snapshot result")),
        };
        if after.result != Some(want_result) {
            return Err(em("snapshot result"));
        }
        if after.data.len() != before.data.len() + dn
            || after.data[..before.data.len()] != before.data[..]
        {
            return Err(em("snapshot data prefix"));
        }
        if after.data_fields.len() != before.data_fields.len() + df
            || after.data_fields[..before.data_fields.len()] != before.data_fields[..]
        {
            return Err(em("snapshot data_fields prefix"));
        }
        if dn == 1 {
            let start = before.active.data_fields;
            let want_row = match staged {
                ScalarStaged::U128Value { n } => DataSnapshot::U128 { n },
                ScalarStaged::BoolValue { b } => DataSnapshot::Bool { b },
                ScalarStaged::PendingNode {
                    tag: NodeKind::Leaf,
                } => DataSnapshot::Leaf {
                    fields: DataFieldsSnapshot { start, len: 1 },
                },
                ScalarStaged::PendingNode {
                    tag: NodeKind::Cons,
                } => DataSnapshot::Cons {
                    fields: DataFieldsSnapshot { start, len: 2 },
                },
                ScalarStaged::PendingNode {
                    tag: NodeKind::Branch,
                } => DataSnapshot::Branch {
                    fields: DataFieldsSnapshot { start, len: 2 },
                },
                _ => return Err(em("snapshot new row")),
            };
            if after.data[before.active.data] != want_row {
                return Err(em("snapshot new row"));
            }
            // Fields appended BEFORE the node, operand_a then operand_b.
            if df >= 1 && after.data_fields[start] != operand_a {
                return Err(em("snapshot field operand_a"));
            }
            if df == 2 {
                let ob = operand_b.ok_or(em("snapshot operand_b"))?;
                if after.data_fields[start + 1] != ob {
                    return Err(em("snapshot field operand_b"));
                }
            }
        }
    } else {
        if after.alloc != before.alloc || after.active != before.active {
            return Err(em("snapshot refused alloc/active"));
        }
        if after.helper != before.helper {
            return Err(em("snapshot refused helper"));
        }
        if after.result != before.result {
            return Err(em("snapshot refused result"));
        }
        if after.data != before.data || after.data_fields != before.data_fields {
            return Err(em("snapshot refused data"));
        }
        let reason = if p3 {
            ExhaustReason::HelperWork
        } else {
            cap_reason.ok_or(em("snapshot refused outcome"))?
        };
        if after.outcome != (OutcomeC::Exhausted { reason }) || e.outcome_payload != after.outcome {
            return Err(em("snapshot refused outcome"));
        }
    }
    Ok(())
}

/// Build a witness from one provided complete record. No step is taken.
/// This validates only internal consistency of the provided event
/// against the selected ScalarExit relation; it does not authenticate
/// that a real owner produced the record.
pub fn witness_from_owner_record(
    record: &CompleteStepRecord,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ScalarExitStepWitness, C6ScalarExitConsumerError> {
    validate_owner_record(record)?;
    witness_from_host_pair(true, &record.before, &record.after, cfg, limits)
}

/// Wrapper: calls fixed owner `record_step` exactly once on this image.
/// Refusal means the stepped step was not the selected Scalar exit.
///
/// Non-rollback contract: the one owner step is taken before this
/// wrapper can refuse. An `Err` return can therefore leave `image`
/// already advanced and mutated by that one step. No rollback is
/// performed or guaranteed, and callers must not interpret `Err` as
/// "no step happened". Only success returns a witness for the
/// selected step.
pub fn record_scalar_exit_step(
    admitted: &Admitted,
    image: &mut MachineImage,
    cfg: &C6HostConfig,
    limits: &C6HostLimits,
) -> Result<C6ScalarExitStepWitness, C6ScalarExitConsumerError> {
    match record_step(admitted, image) {
        Ok(ObservedStep::Event(record)) => witness_from_owner_record(&record, cfg, limits),
        Ok(ObservedStep::Terminal { .. }) => Err(C6ScalarExitConsumerError::UnexpectedOwnerStep),
        Err(e) => Err(C6ScalarExitConsumerError::Backend(format!(
            "owner recorder failed: {}",
            capture_step_error_label(&e)
        ))),
    }
}

#[derive(Clone, Debug)]
pub struct C6ScalarExitStepCircuit {
    mode: C6PushStatementBindingMode,
    geometry: C6PaddedCarrierGeometry,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    source_params: C6PushFamilyParams,
    statement_lane_count: usize,
    ir: IrSource,
    serialized_zkir: String,
}

impl C6ScalarExitStepCircuit {
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
}

fn make_typed(id: Identifier) -> Result<TypedIdentifier, C6ScalarExitConsumerError> {
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
) -> Result<Operand, C6ScalarExitConsumerError> {
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

pub fn build_c6_scalar_exit_step_circuit(
    source_params: C6PushFamilyParams,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    mode: C6PushStatementBindingMode,
    scope: &str,
) -> Result<C6ScalarExitStepCircuit, C6ScalarExitConsumerError> {
    if scope.trim().is_empty() {
        return Err(C6ScalarExitStepError::EmptyScope.into());
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
     -> Result<Operand, C6ScalarExitConsumerError> {
        alloc_lane(mode, scope, label, idx, ci, ii, tt)
    };
    let enable = alloc("enable", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let disp = alloc(
        "disposition_is_refused",
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
     -> Result<Vec<Operand>, C6ScalarExitConsumerError> {
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
        .map_err(|_| C6ScalarExitConsumerError::Backend("pre_ctx".into()))?;
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
        .map_err(|_| C6ScalarExitConsumerError::Backend("post_ctx".into()))?;
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
    let block = emit_c6_scalar_exit_step_block(
        &C6ScalarExitStepInputs {
            enable,
            disposition_is_refused_helper_work: disp,
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
    let _ = (block.is_helper_work_max, block.is_cap_refused);
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
    // No instruction-count guard is emitted here. An instruction count
    // is not the circuit degree `k`, and no size/`k`/performance
    // qualification is made by this module.
    Ok(C6ScalarExitStepCircuit {
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

/// `ProofPreimage::check` preprocessing only; not constraint synthesis,
/// proving, MockProver, or native execution. Returns the actual SDK
/// public-input skips. Structural host prechecks in the witness routes
/// above are not circuit acceptance; rejection here is the SDK
/// preprocessing category.
pub fn check_c6_scalar_exit_step_circuit(
    circuit: &C6ScalarExitStepCircuit,
    witness: &C6ScalarExitStepWitness,
) -> Result<Vec<Option<usize>>, C6ScalarExitConsumerError> {
    if witness.source_params.geometry() != circuit.geometry {
        return Err(C6ScalarExitConsumerError::Backend(
            "witness geometry does not match circuit".into(),
        ));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6ScalarExitConsumerError::LengthMismatch {
            section: "meta",
            expected: circuit.meta_len,
            actual: witness.host_prefix.meta.len(),
        });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6ScalarExitConsumerError::LengthMismatch {
            section: "prog",
            expected: circuit.prog_len,
            actual: witness.host_prefix.prog.len(),
        });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6ScalarExitConsumerError::Backend(
            "meta bounds frames offset mismatch".into(),
        ));
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6ScalarExitConsumerError::LengthMismatch {
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
        C6ScalarExitConsumerError::Backend(format!("SDK ProofPreimage::check failed: {e}"))
    })
}

#[cfg(test)]
#[path = "c6_scalar_exit_consumer_owner_tests.rs"]
mod c6_scalar_exit_consumer_owner_tests;
#[cfg(test)]
#[path = "c6_scalar_exit_consumer_raw_tests.rs"]
mod c6_scalar_exit_consumer_raw_tests;
