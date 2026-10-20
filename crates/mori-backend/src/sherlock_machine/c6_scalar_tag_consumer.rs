//! C6 `BHelperScalarTag` consumer: typed host-pair / owner-record
//! witnesses, selected-step wrapper, SDK builder/checker for the
//! corrected C2 `C6ScalarTagStepEmitter`. Statement order: enable,
//! disposition_is_refused_helper_work, host_schema, carrier_version,
//! pre(META,PROG,CTX,image,data,helper), post(same), both
//! `LocalPublic`/`LedgerCall`; identical relation predicate; LedgerCall
//! output is the post state lanes, commitment via actual
//! `transient_commit` from `completehash.rs` re-export. Host-pair is
//! the independent-post route: only prefix/config/encoder refusals are
//! producer errors; semantic wrong posts reach the checker.
//! `CompleteStepRecord` is publicly constructible: owner-record route
//! validates internal event/snapshot consistency only, no authenticated
//! provenance. `run_id` exists in host metadata but the physical root
//! encoding omits it: no crypto binding / provenance is claimed.
//! Full SI08/SI17/SI21/SI22/StepInv/program-membership/P1/native
//! remain REQUIRED_OPEN.

use std::sync::Arc;

use midnight_transient_crypto::curve::Fr;
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, Zkir};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6HostConfig, C6HostLimits, CTX_LANES, C6_HOST_SCHEMA,
};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    record_step, CaptureStepError, CompleteStepRecord, ConstantSnapshot, DataSnapshot,
    HostSnapshot, ImageSnapshot, ObservedStep,
};
use mori_core::sherlock::helper_machine::shared::{
    Admitted, AllocDeltasC, BoundaryKind, BoolSelector, EventKind, ExhaustReason,
    HelperControlC, HelperKind, HelperPhaseSummaryC, LocatorC, MachineImage, OutcomeC,
    OutcomeTagC, RefC, RuleIdC, ScalarMode, ScalarPhase, ScalarStaged, SiteTag,
    StepDisposition,
};
use serde_json::json;
use thiserror::Error;

use super::c6_push_step::{
    C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStatementBindingMode, C6PushStepError, C6_PADDED_CARRIER_VERSION,
};
use super::c6_scalar_tag_step::{
    emit_c6_scalar_tag_step_block, C6ScalarTagStepError, C6ScalarTagStepInputs,
};

const CHECK_KEY_LOCATION: &str = "sherlock_c6_scalar_tag_step_check";

#[derive(Debug, Error)]
pub enum C6ScalarTagConsumerError {
    #[error("host prefix section {section} mismatch between pre and post state")]
    HostPrefixMismatch { section: &'static str },
    #[error("owner step is not the selected Scalar tag step (terminal or wrong rule)")]
    UnexpectedOwnerStep,
    #[error("owner event mismatch in {field}")]
    EventMismatch { field: &'static str },
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch { section: &'static str, expected: usize, actual: usize },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("C6 ScalarTag emitter error: {0}")]
    ScalarTag(#[from] C6ScalarTagStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ScalarTagConsumerError {
    C6ScalarTagConsumerError::Backend(e.to_string())
}

fn violation_label(v: &mori_core::sherlock::helper_machine::shared::HostViolation) -> String {
    format!(
        "kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind, v.clause, v.rule, v.locator.as_deref().unwrap_or("<none>")
    )
}

fn capture_step_error_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) => {
            format!("before observation: {}", violation_label(v))
        }
        CaptureStepError::Step { violation, after, .. } => match after {
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
        CaptureStepError::AfterObservation { violation, event, .. } => format!(
            "after observation: {}; event_present={}",
            violation_label(violation),
            event.is_some()
        ),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct C6ScalarTagStepWitness {
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

impl C6ScalarTagStepWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams { &self.source_params }
    pub fn enable(&self) -> bool { self.enable }
    pub fn disposition_is_refused_helper_work(&self) -> bool {
        self.disposition_is_refused_helper_work
    }
    pub fn host_prefix(&self) -> &C6PushHostPrefixLanes { &self.host_prefix }
    pub fn pre_image(&self) -> &[u128] { &self.pre_image }
    pub fn pre_data(&self) -> &[u128] { &self.pre_data }
    pub fn pre_helper(&self) -> &[u128] { &self.pre_helper }
    pub fn post_image(&self) -> &[u128] { &self.post_image }
    pub fn post_data(&self) -> &[u128] { &self.post_data }
    pub fn post_helper(&self) -> &[u128] { &self.post_helper }

    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = Vec::new();
        out.push(Fr::from(self.enable));
        out.push(Fr::from(self.disposition_is_refused_helper_work));
        out.push(Fr::from(C6_HOST_SCHEMA));
        out.push(Fr::from(C6_PADDED_CARRIER_VERSION));
        for l in self.host_prefix.meta.iter().chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.pre_image).chain(&self.pre_data).chain(&self.pre_helper) {
            out.push(Fr::from(*l));
        }
        for l in self.host_prefix.meta.iter().chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.post_image).chain(&self.post_data).chain(&self.post_helper) {
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
) -> Result<(Vec<u128>, Vec<u128>, Vec<u128>), C6ScalarTagConsumerError> {
    let padded = super::c6_push_step::encode_owned_state_to_padded(
        &C6OwnedState::from_image(state), caps,
        C6StateLimits { max_total_lanes: limits.max_state_lanes })?;
    let flat = padded.to_flat_lanes();
    let i = geom.image_total_lanes;
    let d = i + geom.data_total_lanes;
    Ok((flat[..i].to_vec(), flat[i..d].to_vec(), flat[d..].to_vec()))
}

/// Independent-post route: no recorder runs; a wrong post that encodes
/// reaches the circuit checker. `disposition` derives from the PRE
/// image helper_work against the ORIGINAL separately retained
/// admission bounds carried by the retained params, never from post
/// labels or any host trusted flag.
pub fn witness_from_host_pair(
    enable: bool, pre: &HostSnapshot, post: &HostSnapshot,
    cfg: &C6HostConfig, limits: &C6HostLimits,
) -> Result<C6ScalarTagStepWitness, C6ScalarTagConsumerError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry();
    let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pp.meta != qp.meta {
        return Err(C6ScalarTagConsumerError::HostPrefixMismatch { section: "meta" });
    }
    if pp.prog != qp.prog {
        return Err(C6ScalarTagConsumerError::HostPrefixMismatch { section: "prog" });
    }
    if pp.ctx != qp.ctx {
        return Err(C6ScalarTagConsumerError::HostPrefixMismatch { section: "ctx" });
    }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    Ok(C6ScalarTagStepWitness {
        enable,
        disposition_is_refused_helper_work:
            pre.image.helper_work == params.admission_bounds().max_helper_work,
        source_params: params,
        host_prefix: pp,
        pre_image, pre_data, pre_helper,
        post_image, post_data, post_helper,
    })
}

type EM = C6ScalarTagConsumerError;
fn em(field: &'static str) -> EM { EM::EventMismatch { field } }

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    UInt(u128),
    Bool(bool),
    Nil,
    Cons,
    Leaf,
    Branch,
    Constructor { type_id: usize, ctor_id: usize },
    Marker,
}

fn kind_of_ref(before: &HostSnapshot, r: RefC) -> Result<Kind, EM> {
    Ok(match r {
        RefC::Const(id) => match before.admission.view.constants.get(id) {
            Some(ConstantSnapshot::U128 { n, .. }) => Kind::UInt(*n),
            Some(ConstantSnapshot::Bool { b, .. }) => Kind::Bool(*b),
            Some(ConstantSnapshot::Nil { .. }) => Kind::Nil,
            Some(ConstantSnapshot::Cons { .. }) => Kind::Cons,
            Some(ConstantSnapshot::Leaf { .. }) => Kind::Leaf,
            Some(ConstantSnapshot::Branch { .. }) => Kind::Branch,
            Some(ConstantSnapshot::Constructor { type_id, ctor_id, .. }) =>
                Kind::Constructor { type_id: type_id.0, ctor_id: ctor_id.0 },
            Some(ConstantSnapshot::U128OutOfRange { .. }) => Kind::Marker,
            None => return Err(em("snapshot operand_a")),
        },
        RefC::Data(id) => match before.image.data.get(id) {
            Some(DataSnapshot::U128 { n }) => Kind::UInt(*n),
            Some(DataSnapshot::Bool { b }) => Kind::Bool(*b),
            Some(DataSnapshot::Nil) => Kind::Nil,
            Some(DataSnapshot::Cons { .. }) => Kind::Cons,
            Some(DataSnapshot::Leaf { .. }) => Kind::Leaf,
            Some(DataSnapshot::Branch { .. }) => Kind::Branch,
            Some(DataSnapshot::Constructor { type_id, ctor_id, .. }) =>
                Kind::Constructor { type_id: type_id.0, ctor_id: ctor_id.0 },
            None => return Err(em("snapshot operand_a")),
        },
    })
}

/// Exact Core nominal check (`adt_checked_ctor_arity`): a failure is a
/// Core-error rejection of the record, never `ok=false`.
fn nominal_ctor_ok(before: &HostSnapshot, type_id: usize, ctor_id: usize) -> Result<(), EM> {
    if type_id < 4 { return Err(em("snapshot nominal type_id")); }
    let view = &before.admission.view;
    let trow = view.types.get(type_id).ok_or(em("snapshot nominal type row"))?;
    let crow = view.constructors.get(ctor_id).ok_or(em("snapshot nominal ctor row"))?;
    if crow.owning_type.0 != type_id { return Err(em("snapshot nominal owning_type")); }
    if ctor_id < trow.ctor_start.0
        || ctor_id - trow.ctor_start.0 >= trow.ctor_len {
        return Err(em("snapshot nominal ctor range"));
    }
    let end = crow.field_start.checked_add(crow.field_len)
        .ok_or(em("snapshot nominal field end"))?;
    if end > view.ctor_fields.len() {
        return Err(em("snapshot nominal field end"));
    }
    Ok(())
}

fn expected_applied_post(
    before: &HostSnapshot,
    mode: ScalarMode,
    operand_a: RefC,
    expected: Option<mori_core::sherlock::helper_machine::shared::adt::TypeIdC>,
    reg_a: u128,
) -> Result<(ScalarPhase, u128, ScalarStaged), EM> {
    let kind = kind_of_ref(before, operand_a)?;
    let none = ScalarStaged::None;
    let err11 = ScalarStaged::PrimError { code: 11 };
    Ok(match mode {
        ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul | ScalarMode::Lt =>
            match kind {
                Kind::UInt(n) => (ScalarPhase::Second, n, none),
                _ => (ScalarPhase::Done, reg_a, err11),
            },
        ScalarMode::Cons => match kind {
            Kind::UInt(_) => (ScalarPhase::Second, reg_a, none),
            _ => (ScalarPhase::Done, reg_a, err11),
        },
        ScalarMode::Branch => match kind {
            Kind::Leaf | Kind::Branch => (ScalarPhase::Second, reg_a, none),
            _ => (ScalarPhase::Done, reg_a, err11),
        },
        ScalarMode::TestBool => {
            let sel = match kind {
                Kind::Bool(true) => BoolSelector::True,
                Kind::Bool(false) => BoolSelector::False,
                _ => BoolSelector::NotBool,
            };
            (ScalarPhase::Done, reg_a, ScalarStaged::Selector { sel })
        }
        ScalarMode::TestType => {
            let exp = expected.ok_or(em("snapshot expected"))?;
            let ok = match kind {
                Kind::Marker => false,
                Kind::UInt(_) => exp.0 == 0,
                Kind::Bool(_) => exp.0 == 1,
                Kind::Nil | Kind::Cons => exp.0 == 2,
                Kind::Leaf | Kind::Branch => exp.0 == 3,
                Kind::Constructor { type_id, ctor_id } => {
                    nominal_ctor_ok(before, type_id, ctor_id)?;
                    type_id == exp.0
                }
            };
            (ScalarPhase::Done, reg_a, ScalarStaged::TypeTest { ok })
        }
        ScalarMode::Leaf | ScalarMode::Not => return Err(em("snapshot mode")),
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
    if !applied && !p3 { return Err(em("disposition")); }
    if e.rule_id != RuleIdC::BHelperScalarTag { return Err(EM::UnexpectedOwnerStep); }
    if e.event_kind != EventKind::HelperVisit { return Err(em("event_kind")); }
    if e.helper_kind != Some(HelperKind::Scalar) { return Err(em("helper_kind")); }
    if e.reference_cursor_delta != 0 { return Err(em("reference_cursor_delta")); }
    if e.alloc_deltas != (AllocDeltasC {
        data_nodes: 0, data_fields: 0, environment_cells: 0 }) {
        return Err(em("alloc deltas"));
    }
    if e.value_slot_writes != 0 { return Err(em("value_slot_writes")); }
    if e.result_tag.is_some() { return Err(em("result_tag")); }
    if applied {
        if e.helper_visits_delta != 1 { return Err(em("helper_visits_delta")); }
        if e.work_delta != 1 || e.helper_work_delta != 1 {
            return Err(em("applied deltas"));
        }
        if e.exhausted_kind.is_some() || e.outcome_tag != OutcomeTagC::Running
            || e.outcome_payload != OutcomeC::Running || e.boundary != BoundaryKind::None {
            return Err(em("applied outcome"));
        }
    } else {
        if e.helper_visits_delta != 0 { return Err(em("helper_visits_delta")); }
        if e.work_delta != 0 || e.helper_work_delta != 0 {
            return Err(em("p3 deltas"));
        }
        if e.exhausted_kind != Some(ExhaustReason::HelperWork) {
            return Err(em("p3 exhausted_kind"));
        }
        if e.outcome_tag != OutcomeTagC::Exhausted
            || e.outcome_payload != (OutcomeC::Exhausted {
                reason: ExhaustReason::HelperWork })
            || e.boundary != BoundaryKind::Terminal {
            return Err(em("p3 outcome"));
        }
    }

    // Selected pre-state, exact actual helper fields.
    let (pre_helper, site, origin_cursor, mode, operand_a, operand_b, expected) =
        match &before.helper {
            Some(h @ HelperControlC::Scalar { common, mode, operand_a, operand_b,
                expected, phase: ScalarPhase::First, reg_a: 0,
                staged: ScalarStaged::None })
                if common.parent_resume.is_none() =>
            {
                (*h, common.site, common.origin_reference_cursor, *mode,
                 *operand_a, *operand_b, *expected)
            }
            _ => return Err(em("snapshot helper before")),
        };
    if !matches!(site, SiteTag::RetRight | SiteTag::RetUnary | SiteTag::RetIf
        | SiteTag::RetCall | SiteTag::RetCtorFields) {
        return Err(em("snapshot site"));
    }
    let binary = matches!(mode,
        ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul | ScalarMode::Lt
        | ScalarMode::Cons | ScalarMode::Branch);
    if matches!(mode, ScalarMode::Leaf | ScalarMode::Not) {
        return Err(em("snapshot mode"));
    }
    if operand_b.is_some() != binary { return Err(em("snapshot operand_b presence")); }
    if expected.is_some() != (mode == ScalarMode::TestType) {
        return Err(em("snapshot expected presence"));
    }
    if before.result.is_some() || after.result.is_some() {
        return Err(em("snapshot result"));
    }
    if !before.tasks.is_empty() || !after.tasks.is_empty() {
        return Err(em("snapshot tasks"));
    }
    if before.outcome != OutcomeC::Running { return Err(em("snapshot outcome before")); }
    if before.remaining_work == 0 { return Err(em("snapshot remaining_work")); }
    if origin_cursor != before.reference_cursor {
        return Err(em("snapshot origin_reference_cursor"));
    }

    // Full event-to-snapshot binding (host-record consistency only;
    // this is not an encoded PI run_id / authentication proof).
    let at_max = before.helper_work == bounds.max_helper_work;
    if p3 != at_max { return Err(em("snapshot disposition p3")); }
    if applied && at_max { return Err(em("snapshot disposition applied")); }
    if e.step_id.ordinal != before.consumed_work
        || *e.step_id.run_id != before.run_id {
        return Err(em("snapshot step_id"));
    }
    if e.site != Some(site) { return Err(em("event site")); }
    if e.origin_reference_cursor != Some(origin_cursor) {
        return Err(em("event origin_reference_cursor"));
    }
    if e.source_expr_id != before.control_origin {
        return Err(em("event source_expr_id"));
    }
    {
        let occ = before_host.admission.view.occurrences
            .get(before.control_origin).ok_or(em("snapshot source occurrence"))?;
        let end = occ.path.start.checked_add(occ.path.len)
            .ok_or(em("snapshot source occurrence"))?;
        let ast_path = before_host.admission.view.occurrence_paths
            .get(occ.path.start..end).ok_or(em("snapshot source occurrence"))?
            .to_vec();
        if e.source_occurrence != (LocatorC::Occurrence {
            owner_is_function: occ.owner_is_function,
            function: occ.function,
            ast_path,
        }) {
            return Err(em("event source_occurrence"));
        }
    }
    if e.logical_depth_before != before.frames.len()
        || e.logical_depth_after != after.frames.len() {
        return Err(em("snapshot depth"));
    }
    if e.task_cursor_before != before.tasks.len()
        || e.task_cursor_after != after.tasks.len() {
        return Err(em("snapshot task cursor"));
    }
    if e.helper_phase_before != Some(HelperPhaseSummaryC::Scalar(ScalarPhase::First))
        || e.helper_cursor_before != Some(0) {
        return Err(em("event helper before"));
    }
    if after.reference_cursor != before.reference_cursor {
        return Err(em("snapshot reference_cursor"));
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
        || after.data != before.data
        || after.data_fields != before.data_fields
        || after.alloc != before.alloc
        || after.active != before.active {
        return Err(em("snapshot preservation"));
    }

    if applied {
        let want_remaining = before.remaining_work.checked_sub(1)
            .ok_or(em("snapshot remaining_work"))?;
        if after.remaining_work != want_remaining {
            return Err(em("snapshot remaining_work"));
        }
        let want_consumed = before.consumed_work.checked_add(1)
            .ok_or(em("snapshot consumed_work"))?;
        if after.consumed_work != want_consumed {
            return Err(em("snapshot consumed_work"));
        }
        let want_hw = before.helper_work.checked_add(1)
            .ok_or(em("snapshot helper_work"))?;
        if after.helper_work != want_hw { return Err(em("snapshot helper_work")); }
        let want_visits = before.helper_visits.checked_add(1)
            .ok_or(em("snapshot helper_visits"))?;
        if after.helper_visits != want_visits {
            return Err(em("snapshot helper_visits"));
        }
        if after.outcome != OutcomeC::Running { return Err(em("snapshot outcome")); }
        // Complete post helper from the actual operand-A row and mode.
        let (phase, reg_a, staged) =
            expected_applied_post(before_host, mode, operand_a, expected, 0)?;
        let mut want_helper = pre_helper;
        if let HelperControlC::Scalar { phase: p, reg_a: ra, staged: s, .. } =
            &mut want_helper
        {
            *p = phase;
            *ra = reg_a;
            *s = staged;
        }
        if after.helper != Some(want_helper) {
            return Err(em("snapshot helper after"));
        }
        let want_cursor = match phase {
            ScalarPhase::Second => 1,
            ScalarPhase::Done => 2,
            ScalarPhase::First => 0,
        };
        if e.helper_phase_after != Some(HelperPhaseSummaryC::Scalar(phase))
            || e.helper_cursor_after != Some(want_cursor) {
            return Err(em("event helper after"));
        }
    } else {
        if after.remaining_work != before.remaining_work
            || after.consumed_work != before.consumed_work
            || after.helper_work != before.helper_work
            || after.helper_visits != before.helper_visits {
            return Err(em("snapshot p3 counters"));
        }
        if after.helper != before.helper { return Err(em("snapshot p3 helper")); }
        if after.outcome != (OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }) {
            return Err(em("snapshot p3 outcome"));
        }
        if e.helper_phase_after != Some(HelperPhaseSummaryC::Scalar(ScalarPhase::First))
            || e.helper_cursor_after != Some(0) {
            return Err(em("event helper after"));
        }
    }
    Ok(())
}

/// Build a witness from one provided complete record. No step is taken.
/// This validates only internal consistency of the provided event
/// against the selected ScalarTag relation; it does not authenticate
/// that a real owner produced the record.
pub fn witness_from_owner_record(
    record: &CompleteStepRecord, cfg: &C6HostConfig, limits: &C6HostLimits,
) -> Result<C6ScalarTagStepWitness, C6ScalarTagConsumerError> {
    validate_owner_record(record)?;
    witness_from_host_pair(true, &record.before, &record.after, cfg, limits)
}

/// Wrapper around `complete_observation::record_step`, called exactly
/// once; no pre-witness or double step.
///
/// Non-rollback contract: the one owner step is taken before this
/// wrapper can refuse. An `Err` return can therefore leave `image`
/// already advanced and mutated by that one step. No rollback is
/// performed or guaranteed, and callers must not interpret `Err` as
/// "no step happened". Only success returns a witness for the
/// selected step.
pub fn record_scalar_tag_step(
    admitted: &Admitted, image: &mut MachineImage,
    cfg: &C6HostConfig, limits: &C6HostLimits,
) -> Result<C6ScalarTagStepWitness, C6ScalarTagConsumerError> {
    match record_step(admitted, image) {
        Ok(ObservedStep::Event(record)) => witness_from_owner_record(&record, cfg, limits),
        Ok(ObservedStep::Terminal { .. }) =>
            Err(C6ScalarTagConsumerError::UnexpectedOwnerStep),
        Err(e) => Err(C6ScalarTagConsumerError::Backend(format!(
            "owner recorder failed: {}",
            capture_step_error_label(&e)
        ))),
    }
}

#[derive(Clone, Debug)]
pub struct C6ScalarTagStepCircuit {
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

impl C6ScalarTagStepCircuit {
    pub fn source_params(&self) -> &C6PushFamilyParams { &self.source_params }
    pub fn geometry(&self) -> C6PaddedCarrierGeometry { self.geometry }
    pub fn meta_len(&self) -> usize { self.meta_len }
    pub fn prog_len(&self) -> usize { self.prog_len }
    pub fn meta_bounds_frames_offset(&self) -> usize { self.meta_bounds_frames_offset }
    pub fn statement_lane_count(&self) -> usize { self.statement_lane_count }
}

fn make_typed(id: Identifier) -> Result<TypedIdentifier, C6ScalarTagConsumerError> {
    serde_json::from_value(json!({ "name": id, "type": IrType::Native })).map_err(be)
}

#[allow(clippy::too_many_arguments)]
fn alloc_lane(
    mode: C6PushStatementBindingMode, scope: &str, label: &str, idx: usize,
    inputs: &mut Vec<TypedIdentifier>, ins: &mut Vec<Instruction>, tr: &mut Vec<Operand>,
) -> Result<Operand, C6ScalarTagConsumerError> {
    let prefix = match mode {
        C6PushStatementBindingMode::LocalPublic => "public",
        C6PushStatementBindingMode::LedgerCall => "input",
    };
    let id = Identifier(format!("%{scope}.{prefix}.{label}.{idx}"));
    match mode {
        C6PushStatementBindingMode::LocalPublic =>
            ins.push(Instruction::PublicInput {
                guard: None, val_t: IrType::Native, output: id.clone() }),
        C6PushStatementBindingMode::LedgerCall => inputs.push(make_typed(id.clone())?),
    }
    let op = Operand::Variable(id);
    tr.push(op.clone());
    Ok(op)
}

pub fn build_c6_scalar_tag_step_circuit(
    source_params: C6PushFamilyParams, meta_len: usize, prog_len: usize,
    meta_bounds_frames_offset: usize, mode: C6PushStatementBindingMode, scope: &str,
) -> Result<C6ScalarTagStepCircuit, C6ScalarTagConsumerError> {
    if scope.trim().is_empty() {
        return Err(C6ScalarTagStepError::EmptyScope.into());
    }
    let geometry = source_params.geometry();
    let mut call_inputs = Vec::new();
    let mut ins = Vec::new();
    let mut tr = Vec::new();
    let mut alloc = |label: &str, idx: usize, ci: &mut Vec<TypedIdentifier>,
        ii: &mut Vec<Instruction>, tt: &mut Vec<Operand>|
     -> Result<Operand, C6ScalarTagConsumerError> {
        alloc_lane(mode, scope, label, idx, ci, ii, tt)
    };
    let enable = alloc("enable", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let disp = alloc("disposition_is_refused", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let schema = alloc("host_schema", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let carrier = alloc("carrier_version", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let mut vec_of = |label: &str, n: usize, ci: &mut Vec<TypedIdentifier>,
        ii: &mut Vec<Instruction>, tt: &mut Vec<Operand>|
     -> Result<Vec<Operand>, C6ScalarTagConsumerError> {
        let mut o = Vec::with_capacity(n);
        for i in 0..n { o.push(alloc(label, i, ci, ii, tt)?); }
        Ok(o)
    };
    let pre_meta = vec_of("pre_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_prog = vec_of("pre_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx_v = vec_of("pre_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx: [Operand; CTX_LANES] = pre_ctx_v.try_into()
        .map_err(|_| C6ScalarTagConsumerError::Backend("pre_ctx".into()))?;
    let pre_image = vec_of("pre_image", geometry.image_total_lanes,
        &mut call_inputs, &mut ins, &mut tr)?;
    let pre_data = vec_of("pre_data", geometry.data_total_lanes,
        &mut call_inputs, &mut ins, &mut tr)?;
    let pre_helper = vec_of("pre_helper", geometry.helper_total_lanes,
        &mut call_inputs, &mut ins, &mut tr)?;
    let post_meta = vec_of("post_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_prog = vec_of("post_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx_v = vec_of("post_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx: [Operand; CTX_LANES] = post_ctx_v.try_into()
        .map_err(|_| C6ScalarTagConsumerError::Backend("post_ctx".into()))?;
    let post_image = vec_of("post_image", geometry.image_total_lanes,
        &mut call_inputs, &mut ins, &mut tr)?;
    let post_data = vec_of("post_data", geometry.data_total_lanes,
        &mut call_inputs, &mut ins, &mut tr)?;
    let post_helper = vec_of("post_helper", geometry.helper_total_lanes,
        &mut call_inputs, &mut ins, &mut tr)?;
    let block = emit_c6_scalar_tag_step_block(&C6ScalarTagStepInputs {
        enable, disposition_is_refused_helper_work: disp, host_schema: schema,
        carrier_version: carrier, geometry, meta_bounds_frames_offset,
        source_params: source_params.clone(),
        pre_meta, pre_prog, pre_ctx, pre_image, pre_data, pre_helper,
        post_meta, post_prog, post_ctx,
        post_image: post_image.clone(), post_data: post_data.clone(),
        post_helper: post_helper.clone(),
    }, scope)?;
    // Receipt fields (nominal_valid only asserted in-circuit under
    // TestType/Constructor in C2) are not re-asserted here.
    let _ = (block.nominal_valid.clone(), block.checked_sort_ok.clone(),
        block.is_helper_work_max.clone());
    ins.extend(block.instructions);
    let statement_lane_count = tr.len();
    ins.push(Instruction::Impact {
        guard: Operand::Immediate(Fr::from(1u64)), inputs: tr });
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
    let ir0 = IrSource { version: IrMinorVersion::V0, inputs: call_inputs, outputs,
        do_communications_commitment: matches!(mode, C6PushStatementBindingMode::LedgerCall),
        instructions: Arc::new(ins) };
    let mut disk = serde_json::to_value(&ir0).map_err(be)?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized_zkir = serde_json::to_string(&disk).map_err(be)?;
    let ir = IrSource::load(serialized_zkir.as_bytes()).map_err(be)?;
    // Instruction count is not degree k / performance qualification.
    Ok(C6ScalarTagStepCircuit { mode, geometry, meta_len, prog_len,
        meta_bounds_frames_offset, source_params, statement_lane_count,
        ir, serialized_zkir })
}

/// `ProofPreimage::check` preprocessing only; not synthesis, proving,
/// MockProver, or native execution. Semantic rejection here is the
/// SDK preprocessing category.
pub fn check_c6_scalar_tag_step_circuit(
    circuit: &C6ScalarTagStepCircuit, witness: &C6ScalarTagStepWitness,
) -> Result<Vec<Option<usize>>, C6ScalarTagConsumerError> {
    if witness.source_params.geometry() != circuit.geometry {
        return Err(C6ScalarTagConsumerError::Backend(
            "witness geometry does not match circuit".into()));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6ScalarTagConsumerError::LengthMismatch {
            section: "meta", expected: circuit.meta_len,
            actual: witness.host_prefix.meta.len() });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6ScalarTagConsumerError::LengthMismatch {
            section: "prog", expected: circuit.prog_len,
            actual: witness.host_prefix.prog.len() });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6ScalarTagConsumerError::Backend(
            "meta bounds frames offset mismatch".into()));
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6ScalarTagConsumerError::LengthMismatch {
            section: "statement", expected: circuit.statement_lane_count,
            actual: statement.len() });
    }
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(),
            public_transcript_outputs: statement,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()) },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm = statement.clone();
            comm.extend(witness.post_flat().iter().map(|&l| Fr::from(l)));
            ProofPreimage {
                inputs: statement.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: statement,
                public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()) }
        }
    };
    preimage.check(&circuit.ir)
        .map_err(|e| C6ScalarTagConsumerError::Backend(
            format!("SDK ProofPreimage::check failed: {e}")))
}

#[cfg(test)]
#[path = "c6_scalar_tag_consumer_raw_tests.rs"]
mod c6_scalar_tag_consumer_raw_tests;
