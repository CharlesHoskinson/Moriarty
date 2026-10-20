//! Genuine-owner positive tests for the three-mode `BHelperScalarConstruct`.
//!
//! UNCOMPILED/UNRUN. Every fixture is a fresh actual public Core or Ext
//! program admitted through the real public owner, then searched with
//! real `record_step` to the actual selected PRE:
//! `(First, Leaf)` reading operand A, or `(Second, Cons|Branch)` reading
//! operand B with the valid SI22 A-domain (Cons: A UInt; Branch: A
//! Leaf|Branch). A wrong-kind A for Cons/Branch terminates at ScalarTag
//! and is never fabricated into a Construct PRE. Expected staging is
//! derived from the literal Core predicate below, never from emitter
//! `post_*`/`is_helper_work_max` receipts or from POST. Active DATA
//! operands come from earlier real Add/Lt/Cons/Leaf/Branch ScalarExit
//! allocations inside the same genuine program.
//!
//! Marker source fixtures below are genuine-owner only in the
//! not-held and validation-boundary senses: an admitted Marker row
//! may exist in the constant table while no held reference denotes
//! it, and a held literal root is routed through Validate, whose
//! root/task refs are the exact SI17 positional exemptions. Neither
//! case is a ScalarConstruct Marker positive. RAW held-Marker
//! Construct specimens live only in the consumer raw child module.
//!
//! REQUIRED_OPEN (not delivered, not claimed): full in-circuit
//! SI17 held-reference traversal and transitive `marker_free`/V6
//! correctness, full raw Fr/namespace/serial suite (separately
//! required), invalid-SI22 fabricated PRE,
//! SI08/SI17/SI21/SI22/StepInv as wholes, program membership,
//! P1/native/full39, proof/synthesis. A nominal `Constructor` kind
//! is read only from the actual snapshot variant, never inferred
//! from a raw type id.

use super::{
    build_c6_scalar_construct_step_circuit, check_c6_scalar_construct_step_circuit,
    record_scalar_construct_step, witness_scalar_construct_from_host_pair,
    witness_scalar_construct_from_owner_record, C6ScalarConstructConsumerError,
};
use super::c6_push_step::C6PushStatementBindingMode;
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    capture, record_step as owner_record, CaptureStepError, CompleteStepRecord,
    ConstantSnapshot, DataSnapshot, HostSnapshot, ObservedStep,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_admit::ExtCapacitiesC;
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr, ExtValue};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

const MAX: u128 = u128::MAX;

fn violation_label(v: &HostViolation) -> String {
    format!("kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind, v.clause, v.rule, v.locator.as_deref().unwrap_or("<none>"))
}
fn capture_label(e: &CaptureStepError) -> String {
    match e {
        CaptureStepError::BeforeObservation(v) =>
            format!("before observation: {}", violation_label(v)),
        CaptureStepError::Step { violation, after, .. } => match after {
            Ok(_) => format!("owner step: {}; after snapshot captured", violation_label(violation)),
            Err(av) => format!("owner step: {}; after observation: {}",
                violation_label(violation), violation_label(av)),
        },
        CaptureStepError::AfterObservation { violation, event, .. } => format!(
            "after observation: {}; event_present={}", violation_label(violation), event.is_some()),
    }
}
fn tooling_label(e: &Tooling) -> String {
    match e {
        Tooling::AdmissionRejected { reason, detail, locator } => format!(
            "admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        Tooling::CapacityNotAdmitted { kind, needed, limit, locator } => format!(
            "capacity not admitted: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"),
        Tooling::UnsupportedFeature { feature, locator } => format!(
            "unsupported feature: feature={feature:?}, locator={locator:?}"),
    }
}
fn ext_label(e: &ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(t) => tooling_label(t),
        ExtTypeBuildError::ExtCapacityNotAdmitted { kind, needed, limit, locator } => format!(
            "ext capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"),
        ExtTypeBuildError::ExtAdmissionRejected { reason, detail, locator } => format!(
            "ext admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } => format!(
            "ext unsupported feature: feature={feature:?}, locator={locator:?}"),
        ExtTypeBuildError::Host(v) => format!("ext host violation: {}", violation_label(v)),
    }
}

fn bounds(max_helper_work: u128) -> BoundsC {
    BoundsC {
        code_rows: 64, function_rows: 8, code_child_slots: 64, parameter_slots: 16,
        pattern_rows: 16, match_plan_rows: 16, match_arm_rows: 16, type_rows: 16,
        constructor_rows: 16, constructor_field_slots: 16, max_arity: 4,
        constant_nodes: 64, constant_field_slots: 64, data_nodes: 16,
        data_field_slots: 16, environment_cells: 16, frames: 16, helper_tasks: 4,
        serialization_bytes: 1_000_000, max_reference_work: 1_000_000,
        max_helper_work, step_cap: 1_000_000,
    }
}
fn generous() -> BoundsC { bounds(100) }
fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC { name_rows: 32, name_bytes: 512, occurrence_path_slots: 64, value_slots: 512 }
}
fn layout(b: BoundsC, ext: Option<ExtCapacitiesC>) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()), field_id: None, widths: None,
        scalar_layout_id: None, bounds: b, capacities: capacities(),
        ext_capacities: ext, boundary_capacities_present: false,
    }
}
fn cfg(b: BoundsC, ext: Option<ExtCapacitiesC>) -> C6HostConfig {
    C6HostConfig::from_retained_layout(&layout(b, ext),
        C6CodecBudgets { type_expr_nodes: 256, template_bytes: 4096 })
}
fn core_cfg(b: BoundsC) -> C6HostConfig { cfg(b, None) }
fn ext_cfg(b: BoundsC) -> C6HostConfig { cfg(b, Some(ExtCapacitiesC::all_zero())) }
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Space { Const, Data }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KindWant { UInt(u128), Bool(bool), Nil, Cons, Leaf, Branch, Ctor }
#[derive(Clone, Copy, Debug)]
struct Spec {
    mode: ScalarMode, a: KindWant, a_space: Space,
    b: Option<KindWant>, b_space: Option<Space>, staged: ScalarStaged,
}
fn leaf_spec(a: KindWant, a_space: Space, staged: ScalarStaged) -> Spec {
    Spec { mode: ScalarMode::Leaf, a, a_space, b: None, b_space: None, staged }
}
fn cons_spec(a: KindWant, a_space: Space, b: KindWant, b_space: Space, staged: ScalarStaged) -> Spec {
    Spec { mode: ScalarMode::Cons, a, a_space, b: Some(b), b_space: Some(b_space), staged }
}
fn branch_spec(a: KindWant, a_space: Space, b: KindWant, b_space: Space, staged: ScalarStaged) -> Spec {
    Spec { mode: ScalarMode::Branch, a, a_space, b: Some(b), b_space: Some(b_space), staged }
}
const ERR11: ScalarStaged = ScalarStaged::PrimError { code: 11 };
fn pend(tag: NodeKind) -> ScalarStaged { ScalarStaged::PendingNode { tag } }

fn kind_matches(actual: KindWant, want: KindWant) -> bool {
    // Nominal Constructor: exact snapshot variant only; ids are not a kind oracle.
    if want == KindWant::Ctor { return actual == KindWant::Ctor; }
    actual == want
}
fn operand_kind(s: &HostSnapshot, r: RefC) -> KindWant {
    match r {
        RefC::Const(id) => match s.admission.view.constants.get(id) {
            Some(ConstantSnapshot::U128 { n, .. }) => KindWant::UInt(*n),
            Some(ConstantSnapshot::Bool { b, .. }) => KindWant::Bool(*b),
            Some(ConstantSnapshot::Nil { .. }) => KindWant::Nil,
            Some(ConstantSnapshot::Cons { .. }) => KindWant::Cons,
            Some(ConstantSnapshot::Leaf { .. }) => KindWant::Leaf,
            Some(ConstantSnapshot::Branch { .. }) => KindWant::Branch,
            Some(ConstantSnapshot::Constructor { .. }) => KindWant::Ctor,
            Some(ConstantSnapshot::U128OutOfRange { .. }) =>
                panic!("raw Marker is REQUIRED_OPEN, not a genuine positive"),
            None => panic!("selected Const operand must exist"),
        },
        RefC::Data(id) => match s.image.data.get(id) {
            Some(DataSnapshot::U128 { n }) => KindWant::UInt(*n),
            Some(DataSnapshot::Bool { b }) => KindWant::Bool(*b),
            Some(DataSnapshot::Nil) => KindWant::Nil,
            Some(DataSnapshot::Cons { .. }) => KindWant::Cons,
            Some(DataSnapshot::Leaf { .. }) => KindWant::Leaf,
            Some(DataSnapshot::Branch { .. }) => KindWant::Branch,
            Some(DataSnapshot::Constructor { .. }) => KindWant::Ctor,
            None => panic!("selected Data operand must exist"),
        },
    }
}
fn space_of(r: RefC) -> Space { match r { RefC::Const(_) => Space::Const, RefC::Data(_) => Space::Data } }

// --- Marker/SI17 snapshot projection (host test code only) ---
// TYPED ACCESS OBSTRUCTION: global `check_step_inv` needs a
// `PreImage`; supplied source05 exposes no backend-test-public
// constructor for it for `host_owner_engine()`. This is therefore
// an exact snapshot-holder projection of the Core SI17 holder
// definition, not the engine checker and not a proof.
fn const_marker_free(s: &HostSnapshot, id: usize) -> bool {
    match s.admission.view.constants.get(id) {
        Some(ConstantSnapshot::U128 { marker_free, .. })
        | Some(ConstantSnapshot::Bool { marker_free, .. })
        | Some(ConstantSnapshot::Nil { marker_free })
        | Some(ConstantSnapshot::Cons { marker_free, .. })
        | Some(ConstantSnapshot::Leaf { marker_free, .. })
        | Some(ConstantSnapshot::Branch { marker_free, .. })
        | Some(ConstantSnapshot::Constructor { marker_free, .. })
        | Some(ConstantSnapshot::U128OutOfRange { marker_free }) => *marker_free,
        None => panic!("Const id must exist for marker_free projection"),
    }
}
fn const_kind_name(s: &HostSnapshot, id: usize) -> &'static str {
    match s.admission.view.constants.get(id) {
        Some(ConstantSnapshot::U128 { .. }) => "UInt",
        Some(ConstantSnapshot::Bool { .. }) => "Bool",
        Some(ConstantSnapshot::Nil { .. }) => "Nil",
        Some(ConstantSnapshot::Cons { .. }) => "Cons",
        Some(ConstantSnapshot::Leaf { .. }) => "Leaf",
        Some(ConstantSnapshot::Branch { .. }) => "Branch",
        Some(ConstantSnapshot::Constructor { .. }) => "Constructor",
        Some(ConstantSnapshot::U128OutOfRange { .. }) => "Marker",
        None => panic!("Const id must exist for kind projection"),
    }
}
fn marker_const_ids(s: &HostSnapshot) -> Vec<usize> {
    s.admission.view.constants.iter().enumerate().filter_map(|(i, c)| match c {
        ConstantSnapshot::U128OutOfRange { marker_free } => {
            assert!(!*marker_free, "admitted Marker row must carry marker_free=false");
            Some(i)
        }
        _ => None,
    }).collect()
}
fn transitive_false_composite_ids(s: &HostSnapshot) -> Vec<usize> {
    s.admission.view.constants.iter().enumerate().filter_map(|(i, c)| match c {
        ConstantSnapshot::Cons { marker_free: false, .. }
        | ConstantSnapshot::Leaf { marker_free: false, .. }
        | ConstantSnapshot::Branch { marker_free: false, .. }
        | ConstantSnapshot::Constructor { marker_free: false, .. } => Some(i),
        _ => None,
    }).collect()
}
fn collect_si17_refs(s: &HostSnapshot, held: &mut Vec<RefC>, exempt: &mut Vec<RefC>) {
    match &s.image.control {
        ControlC::Return { value } | ControlC::Cases { value, .. }
        | ControlC::CaseResult { value, .. } => held.push(*value),
        ControlC::PrimResult { result: PrimOutcome::Value { value } } => held.push(*value),
        ControlC::ScheduleNext { frame, .. } => {
            if let FrameRowC::RightF { left, .. } = frame { held.push(*left); }
        }
        _ => {}
    }
    for f in &s.image.frames {
        if let FrameRowC::RightF { left, .. } = f { held.push(*left); }
    }
    if let Some(h) = &s.image.helper {
        match h {
            HelperControlC::Scalar { operand_a, operand_b, .. } => {
                held.push(*operand_a);
                if let Some(b) = operand_b { held.push(*b); }
            }
            HelperControlC::EnvGet { found, .. } => {
                if let Some(r) = found { held.push(*r); }
            }
            HelperControlC::EnvPut { value, .. } => held.push(*value),
            HelperControlC::Append { value, staged, .. } => {
                held.push(*value);
                if let Some(r) = staged { held.push(*r); }
            }
            HelperControlC::Build { staged, node, .. } => {
                if let Some(r) = staged { held.push(*r); }
                if let Some(r) = node { held.push(*r); }
            }
            HelperControlC::Validate { root, .. } => exempt.push(*root),
            HelperControlC::Equal { left, right, .. } => { held.push(*left); held.push(*right); }
            HelperControlC::Match { scrutinee, staged, .. } => {
                held.push(*scrutinee);
                if let Some(r) = staged { held.push(*r); }
            }
            HelperControlC::Plan { scrutinee, cur, .. } => { held.push(*scrutinee); held.push(*cur); }
            HelperControlC::Find { .. } | HelperControlC::Bind { .. } => {}
        }
    }
    if let Some(r) = &s.image.result {
        match r {
            HelperResultC::EnvFound { value } | HelperResultC::ScalarValue { value }
            | HelperResultC::EqualDone { value } | HelperResultC::BuildDone { value } => held.push(*value),
            _ => {}
        }
    }
    for c in &s.image.env { held.push(c.value); }
    held.extend(s.image.value_slots.iter().copied());
    for t in &s.image.tasks {
        match t {
            HelperTaskC::ValidateTask { r, .. } => exempt.push(*r),
            HelperTaskC::EqualTask { left, right, .. } => { held.push(*left); held.push(*right); }
            HelperTaskC::PairTask { arg, .. } => held.push(*arg),
        }
    }
    held.extend(s.image.data_fields.iter().copied());
}
fn snapshot_si17_projection(s: &HostSnapshot) -> (Vec<RefC>, Vec<RefC>, bool) {
    let (mut held, mut exempt) = (Vec::new(), Vec::new());
    collect_si17_refs(s, &mut held, &mut exempt);
    let holds = held.iter().all(|r| match r {
        RefC::Const(id) => const_marker_free(s, *id),
        RefC::Data(_) => true,
    });
    (held, exempt, holds)
}
fn selected_helper(s: &HostSnapshot) -> HelperControlC {
    match s.image.helper { Some(h @ HelperControlC::Scalar { .. }) => h,
        _ => panic!("snapshot must carry a Scalar helper") }
}
fn is_selected(s: &HostSnapshot, w: &Spec) -> bool {
    let helper = match s.image.helper {
        Some(h @ HelperControlC::Scalar { .. }) => h, _ => return false,
    };
    let HelperControlC::Scalar { common, mode, operand_a, operand_b, expected,
        phase, reg_a, staged } = helper else { return false };
    if mode != w.mode || reg_a != 0 || staged != ScalarStaged::None || expected.is_some()
        || common.parent_resume.is_some() || s.image.result.is_some() || !s.image.tasks.is_empty()
        || s.image.outcome != OutcomeC::Running
        || common.origin_reference_cursor != s.image.reference_cursor { return false; }
    let shape = match w.mode {
        ScalarMode::Leaf => phase == ScalarPhase::First && common.site == SiteTag::RetUnary
            && operand_b.is_none(),
        ScalarMode::Cons => phase == ScalarPhase::Second && common.site == SiteTag::RetRight
            && operand_b.is_some()
            && matches!(operand_kind(s, operand_a), KindWant::UInt(_)),
        ScalarMode::Branch => phase == ScalarPhase::Second && common.site == SiteTag::RetRight
            && operand_b.is_some()
            && matches!(operand_kind(s, operand_a), KindWant::Leaf | KindWant::Branch),
        _ => return false,
    };
    if !shape { return false; }
    if space_of(operand_a) != w.a_space || !kind_matches(operand_kind(s, operand_a), w.a) { return false; }
    match (w.b, w.b_space, operand_b) {
        (None, None, None) => true,
        (Some(k), Some(sp), Some(rb)) =>
            space_of(rb) == sp && kind_matches(operand_kind(s, rb), k),
        _ => false,
    }
}

fn search(admitted: Admitted, mut image: MachineImage, w: &Spec, label: &str)
    -> (Admitted, MachineImage, HostSnapshot) {
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search: {}", capture_label(&e)));
        let after = match &step { ObservedStep::Event(r) => &r.after,
            ObservedStep::Terminal { after, .. } =>
                panic!("{label}: terminal before selected PRE: {:?}", after.image.outcome) };
        if is_selected(after, w) {
            let sel = capture(&admitted, &image)
                .unwrap_or_else(|e| panic!("{label}: capture PRE: {}", violation_label(&e)));
            assert_eq!(&sel, after, "{label}: capture equals searched PRE");
            return (admitted, image, sel);
        }
    }
    panic!("{label}: genuine owner never reached selected Construct PRE")
}
fn reach_core_fuel(b: BoundsC, fuel: u128, w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram)
    -> (Admitted, MachineImage, HostSnapshot) {
    let engine = host_owner_engine();
    let admitted = engine.admit(&make(), &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let image = engine.admit_run(&admitted, &RunIntC::Nat(fuel), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}
fn reach_core(b: BoundsC, w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram)
    -> (Admitted, MachineImage, HostSnapshot) { reach_core_fuel(b, 64, w, label, make) }
fn reach_ext(b: BoundsC, w: &Spec, label: &str, make: &dyn Fn() -> ExtProgram)
    -> (Admitted, MachineImage, HostSnapshot) {
    let (admitted, _report) = admit_ext_program(make(), ExtDomain::GENERIC_ABSENT, &b,
        &layout(b, Some(ExtCapacitiesC::all_zero())), None)
        .unwrap_or_else(|e| panic!("{label}: admit_ext_program: {}", ext_label(&e)));
    let engine = host_owner_engine();
    let image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("{label}: ext genesis: {}", tooling_label(&e)));
    search(admitted, image, w, label)
}

fn assert_preserved(r: &CompleteStepRecord, label: &str) {
    let (pre, post) = (&r.before.image, &r.after.image);
    assert_eq!(r.after.admission, r.before.admission, "{label}: admission");
    assert_eq!(post.run_id, pre.run_id, "{label}: run_id");
    assert_eq!(post.control, pre.control, "{label}: control");
    assert_eq!(post.control_origin, pre.control_origin, "{label}: origin");
    assert_eq!(post.env_root, pre.env_root, "{label}: env root");
    assert_eq!(post.stack_limit, pre.stack_limit, "{label}: stack");
    assert_eq!(post.initial_fuel, pre.initial_fuel, "{label}: fuel");
    assert_eq!(post.seg_base, pre.seg_base, "{label}: seg_base");
    assert_eq!(post.frames, pre.frames, "{label}: frames");
    assert_eq!(post.env, pre.env, "{label}: env");
    assert_eq!(post.value_slots, pre.value_slots, "{label}: slots");
    assert_eq!(post.tasks, pre.tasks, "{label}: tasks");
    assert_eq!(post.data, pre.data, "{label}: DATA");
    assert_eq!(post.data_fields, pre.data_fields, "{label}: DATA fields");
    assert_eq!(post.alloc, pre.alloc, "{label}: alloc");
    assert_eq!(post.active, pre.active, "{label}: active");
    assert_eq!(post.reference_cursor, pre.reference_cursor, "{label}: ref cursor");
}
fn assert_applied(r: &CompleteStepRecord, w: &Spec, label: &str) {
    assert!(is_selected(&r.before, w), "{label}: selected PRE by shape/kinds/spaces, not POST");
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert_preserved(r, label);
    assert_eq!(e.rule_id, RuleIdC::BHelperScalarConstruct, "{label}: rule");
    assert_eq!(e.event_kind, EventKind::HelperVisit, "{label}: event kind");
    assert_eq!(e.helper_kind, Some(HelperKind::Scalar), "{label}: helper kind");
    assert_eq!(e.disposition, StepDisposition::Applied, "{label}: disposition");
    assert_eq!((e.work_delta, e.helper_work_delta, e.helper_visits_delta), (1, 1, 1), "{label}: deltas");
    assert_eq!(e.reference_cursor_delta, 0, "{label}: cursor delta");
    assert_eq!(e.alloc_deltas, AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }, "{label}: no alloc");
    assert_eq!(e.value_slot_writes, 0, "{label}: slot writes");
    assert!(e.result_tag.is_none() && e.exhausted_kind.is_none(), "{label}: no result/exhaust");
    assert_eq!(e.outcome_tag, OutcomeTagC::Running, "{label}: outcome tag");
    assert_eq!(e.outcome_payload, OutcomeC::Running, "{label}: outcome");
    assert_eq!(e.boundary, BoundaryKind::None, "{label}: boundary");
    assert_eq!(e.step_id.ordinal, pre.consumed_work, "{label}: ordinal");
    assert_eq!(*e.step_id.run_id, pre.run_id, "{label}: step run id");
    let (site, cur_before, phase_before) = match w.mode {
        ScalarMode::Leaf => (SiteTag::RetUnary, 0, ScalarPhase::First),
        _ => (SiteTag::RetRight, 1, ScalarPhase::Second),
    };
    assert_eq!(e.site, Some(site), "{label}: site");
    assert_eq!(e.origin_reference_cursor, Some(pre.reference_cursor), "{label}: origin cursor");
    assert_eq!(e.source_expr_id, pre.control_origin, "{label}: source expr");
    {
        let occ = &r.before.admission.view.occurrences[pre.control_origin];
        let path = r.before.admission.view.occurrence_paths[occ.path.start..occ.path.start + occ.path.len].to_vec();
        assert_eq!(e.source_occurrence, LocatorC::Occurrence {
            owner_is_function: occ.owner_is_function, function: occ.function, ast_path: path },
            "{label}: source occurrence");
    }
    assert_eq!(e.logical_depth_before, pre.frames.len(), "{label}: depth before");
    assert_eq!(e.logical_depth_after, post.frames.len(), "{label}: depth after");
    assert_eq!(e.task_cursor_before, pre.tasks.len(), "{label}: task before");
    assert_eq!(e.task_cursor_after, post.tasks.len(), "{label}: task after");
    assert_eq!(e.helper_phase_before, Some(HelperPhaseSummaryC::Scalar(phase_before)), "{label}: phase before");
    assert_eq!(e.helper_cursor_before, Some(cur_before), "{label}: cursor before");
    assert_eq!(e.helper_phase_after, Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done)), "{label}: phase after");
    assert_eq!(e.helper_cursor_after, Some(2), "{label}: cursor after");
    assert_eq!(post.remaining_work, pre.remaining_work - 1, "{label}: remaining-1");
    assert_eq!(post.consumed_work, pre.consumed_work + 1, "{label}: consumed+1");
    assert_eq!(post.helper_work, pre.helper_work + 1, "{label}: helper_work+1");
    assert_eq!(post.helper_visits, pre.helper_visits + 1, "{label}: visits+1");
    assert!(pre.result.is_none() && post.result.is_none(), "{label}: result absent, helper not cleared");
    let mut want = selected_helper(&r.before);
    if let HelperControlC::Scalar { phase, staged, .. } = &mut want {
        *phase = ScalarPhase::Done; *staged = w.staged;
    }
    // Only phase/staged change: full common/mode/operands/expected/regA preserved.
    assert_eq!(post.helper, Some(want), "{label}: complete post helper");
}
fn assert_both_modes(witness: &super::C6ScalarConstructStepWitness, label: &str) {
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let circuit = build_c6_scalar_construct_step_circuit(witness.source_params().clone(),
            witness.host_prefix().meta.len(), witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset, mode, "scalar_construct_genuine")
            .unwrap_or_else(|e| panic!("{label}: build {mode:?}: {e}"));
        // ProofPreimage::check preprocessing only; never ungated receipts.
        check_c6_scalar_construct_step_circuit(&circuit, witness)
            .unwrap_or_else(|e| panic!("{label}: check {mode:?}: {e}"));
    }
}
struct Applied { record: CompleteStepRecord }
fn finish_applied(admitted: Admitted, mut image: MachineImage, pre: HostSnapshot,
    replay: CompleteStepRecord, config: &C6HostConfig, w: &Spec, label: &str) -> Applied {
    assert_eq!(replay.before, pre, "{label}: replay PRE equals original PRE");
    assert_applied(&replay, w, label);
    let witness = record_scalar_construct_step(&admitted, &mut image, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: wrapper selected step once: {e}"));
    let original_post = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture original POST: {}", violation_label(&e)));
    assert_eq!(original_post, replay.after, "{label}: original POST == independently replayed POST");
    let rec_wit = witness_scalar_construct_from_owner_record(&replay, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: owner-record witness: {e}"));
    let hp_wit = witness_scalar_construct_from_host_pair(true, &replay.before, &replay.after, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: host-pair witness: {e}"));
    assert_eq!(witness.to_statement_frs(), rec_wit.to_statement_frs(), "{label}: wrapper == record witness");
    assert_eq!(witness.to_statement_frs(), hp_wit.to_statement_frs(), "{label}: wrapper == host-pair witness");
    assert!(witness.enable() && !witness.disposition_is_refused_helper_work(), "{label}: applied flags");
    assert_both_modes(&witness, label);
    Applied { record: replay }
}
fn run_core(w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram) -> Applied {
    let b = generous();
    let (admitted, image, pre) = reach_core(b, w, label, make);
    let (radm, mut rimg, _) = reach_core(b, w, label, make);
    let replay = match owner_record(&radm, &mut rimg) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: replay selected step terminal"),
        Err(e) => panic!("{label}: replay selected step: {}", capture_label(&e)),
    };
    finish_applied(admitted, image, pre, replay, &core_cfg(b), w, label)
}
fn run_ext(w: &Spec, label: &str, make: &dyn Fn() -> ExtProgram) -> Applied {
    let b = generous();
    let (admitted, image, pre) = reach_ext(b, w, label, make);
    let (radm, mut rimg, _) = reach_ext(b, w, label, make);
    let replay = match owner_record(&radm, &mut rimg) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: replay selected ext step terminal"),
        Err(e) => panic!("{label}: replay selected ext step: {}", capture_label(&e)),
    };
    finish_applied(admitted, image, pre, replay, &ext_cfg(b), w, label)
}

fn lit_u(n: u128) -> CoreExpr { CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::InRange(n) } } }
fn lit_b(v: bool) -> CoreExpr { CoreExpr::Literal { value: CoreValue::Bool { b: v } } }
fn lit_list(v: &[u128]) -> CoreExpr { CoreExpr::Literal { value: CoreValue::List {
    elements: v.iter().map(|n| i128_class::KIntC::InRange(*n)).collect() } } }
fn tree_leaf(n: u128) -> CoreTree { CoreTree::Leaf { n: i128_class::KIntC::InRange(n) } }
fn lit_tree(t: CoreTree) -> CoreExpr { CoreExpr::Literal { value: CoreValue::Tree { tree: t } } }
fn lit_branch() -> CoreExpr { lit_tree(CoreTree::Branch {
    left: Box::new(tree_leaf(1)), right: Box::new(tree_leaf(2)) }) }
fn bin(op: BinaryOp, l: CoreExpr, r: CoreExpr) -> CoreExpr {
    CoreExpr::Binary { op, left: Box::new(l), right: Box::new(r) } }
fn unary(op: UnaryOp, x: CoreExpr) -> CoreExpr { CoreExpr::Unary { op, operand: Box::new(x) } }
fn prog(entry: CoreExpr) -> CoreProgram { CoreProgram { functions: Vec::new(), entry } }
fn lit_marker_neg() -> CoreExpr {
    CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::Negative } }
}
fn lit_marker_above() -> CoreExpr {
    CoreExpr::Literal { value: CoreValue::UInt { n: i128_class::KIntC::AboveU128 } }
}
fn lit_list_marker() -> CoreExpr {
    CoreExpr::Literal { value: CoreValue::List { elements: vec![
        i128_class::KIntC::InRange(1), i128_class::KIntC::Negative] } }
}
fn lit_tree_marker_branch() -> CoreExpr {
    lit_tree(CoreTree::Branch {
        left: Box::new(CoreTree::Leaf { n: i128_class::KIntC::AboveU128 }),
        right: Box::new(tree_leaf(2)) })
}
fn unused_marker_fn(name: &str, return_type: SortTag, body: CoreExpr) -> CoreFunction {
    CoreFunction { name: name.into(), parameters: Vec::new(), return_type, body }
}
fn marker_unused_prog(entry: CoreExpr) -> CoreProgram {
    CoreProgram { functions: vec![
        unused_marker_fn("unused_marker_scalar", SortTag::UInt, lit_marker_neg()),
        unused_marker_fn("unused_marker_list", SortTag::List, lit_list_marker()),
        unused_marker_fn("unused_marker_tree", SortTag::Tree, lit_tree_marker_branch()),
    ], entry }
}
fn leaf_of(x: CoreExpr) -> CoreProgram { prog(unary(UnaryOp::Leaf, x)) }
fn cons_of(a: CoreExpr, b: CoreExpr) -> CoreProgram { prog(bin(BinaryOp::Cons, a, b)) }
fn branch_of(a: CoreExpr, b: CoreExpr) -> CoreProgram { prog(bin(BinaryOp::Branch, a, b)) }

#[test]
fn leaf_const_success_and_mismatches_both_modes() {
    run_core(&leaf_spec(KindWant::UInt(0), Space::Const, pend(NodeKind::Leaf)), "Leaf Const UInt0",
        &|| leaf_of(lit_u(0)));
    run_core(&leaf_spec(KindWant::UInt(MAX), Space::Const, pend(NodeKind::Leaf)), "Leaf Const UIntMAX",
        &|| leaf_of(lit_u(MAX)));
    run_core(&leaf_spec(KindWant::Bool(true), Space::Const, ERR11), "Leaf Const Bool11",
        &|| leaf_of(lit_b(true)));
    run_core(&leaf_spec(KindWant::Nil, Space::Const, ERR11), "Leaf Const Nil11",
        &|| leaf_of(lit_list(&[])));
    run_core(&leaf_spec(KindWant::Cons, Space::Const, ERR11), "Leaf Const Cons11",
        &|| leaf_of(lit_list(&[9])));
    run_core(&leaf_spec(KindWant::Leaf, Space::Const, ERR11), "Leaf Const Leaf11",
        &|| leaf_of(lit_tree(tree_leaf(3))));
    run_core(&leaf_spec(KindWant::Branch, Space::Const, ERR11), "Leaf Const Branch11",
        &|| leaf_of(lit_branch()));
}
#[test]
fn leaf_data_success_and_mismatch_both_modes() {
    // A is the Data UInt allocated by the inner Add ScalarExit.
    run_core(&leaf_spec(KindWant::UInt(5), Space::Data, pend(NodeKind::Leaf)), "Leaf Data UInt5",
        &|| leaf_of(bin(BinaryOp::Add, lit_u(2), lit_u(3))));
    // A is the Data Bool allocated by the inner Lt ScalarExit.
    run_core(&leaf_spec(KindWant::Bool(true), Space::Data, ERR11), "Leaf Data Bool11",
        &|| leaf_of(bin(BinaryOp::Lt, lit_u(1), lit_u(2))));
}
#[test]
fn cons_const_success_and_mismatches_both_modes() {
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Nil, Space::Const, pend(NodeKind::Cons)),
        "Cons Const Nil", &|| cons_of(lit_u(7), lit_list(&[])));
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Cons, Space::Const, pend(NodeKind::Cons)),
        "Cons Const ConsB", &|| cons_of(lit_u(7), lit_list(&[9])));
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::UInt(1), Space::Const, ERR11),
        "Cons Const UIntB11", &|| cons_of(lit_u(7), lit_u(1)));
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Bool(false), Space::Const, ERR11),
        "Cons Const BoolB11", &|| cons_of(lit_u(7), lit_b(false)));
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Leaf, Space::Const, ERR11),
        "Cons Const LeafB11", &|| cons_of(lit_u(7), lit_tree(tree_leaf(1))));
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Branch, Space::Const, ERR11),
        "Cons Const BranchB11", &|| cons_of(lit_u(7), lit_branch()));
}
#[test]
fn cons_data_spaces_success_and_mismatch_both_modes() {
    run_core(&cons_spec(KindWant::UInt(5), Space::Data, KindWant::Nil, Space::Const, pend(NodeKind::Cons)),
        "Cons DataA UInt NilB", &|| cons_of(bin(BinaryOp::Add, lit_u(2), lit_u(3)), lit_list(&[])));
    // B is the Data Cons allocated by the inner Cons ScalarExit.
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Cons, Space::Data, pend(NodeKind::Cons)),
        "Cons ConstA DataConsB", &|| cons_of(lit_u(7), bin(BinaryOp::Cons, lit_u(8), lit_list(&[]))));
    // B is the Data Leaf allocated by the inner Leaf ScalarExit.
    run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Leaf, Space::Data, ERR11),
        "Cons ConstA DataLeafB11", &|| cons_of(lit_u(7), unary(UnaryOp::Leaf, lit_u(3))));
}
#[test]
fn branch_const_success_and_mismatches_both_modes() {
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Leaf, Space::Const, pend(NodeKind::Branch)),
        "Branch Const LeafB", &|| branch_of(lit_tree(tree_leaf(1)), lit_tree(tree_leaf(2))));
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Branch, Space::Const, pend(NodeKind::Branch)),
        "Branch Const BranchB", &|| branch_of(lit_tree(tree_leaf(1)), lit_branch()));
    run_core(&branch_spec(KindWant::Branch, Space::Const, KindWant::Leaf, Space::Const, pend(NodeKind::Branch)),
        "Branch ConstA Branch", &|| branch_of(lit_branch(), lit_tree(tree_leaf(2))));
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Nil, Space::Const, ERR11),
        "Branch Const NilB11", &|| branch_of(lit_tree(tree_leaf(1)), lit_list(&[])));
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::UInt(1), Space::Const, ERR11),
        "Branch Const UIntB11", &|| branch_of(lit_tree(tree_leaf(1)), lit_u(1)));
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Bool(true), Space::Const, ERR11),
        "Branch Const BoolB11", &|| branch_of(lit_tree(tree_leaf(1)), lit_b(true)));
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Cons, Space::Const, ERR11),
        "Branch Const ConsB11", &|| branch_of(lit_tree(tree_leaf(1)), lit_list(&[4])));
}
#[test]
fn branch_data_spaces_success_and_mismatch_both_modes() {
    // A is the Data Leaf from the inner Leaf ScalarExit.
    run_core(&branch_spec(KindWant::Leaf, Space::Data, KindWant::Branch, Space::Const, pend(NodeKind::Branch)),
        "Branch DataA Leaf", &|| branch_of(unary(UnaryOp::Leaf, lit_u(3)), lit_branch()));
    // B is the Data Branch from the inner Branch ScalarExit.
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Branch, Space::Data, pend(NodeKind::Branch)),
        "Branch ConstA DataBranchB",
        &|| branch_of(lit_tree(tree_leaf(1)), bin(BinaryOp::Branch, lit_tree(tree_leaf(2)), lit_tree(tree_leaf(3)))));
    // B is the Data Cons from the inner Cons ScalarExit: sort mismatch.
    run_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Cons, Space::Data, ERR11),
        "Branch ConstA DataConsB11",
        &|| branch_of(lit_tree(tree_leaf(1)), bin(BinaryOp::Cons, lit_u(8), lit_list(&[]))));
}

fn nominal_types() -> Vec<TypeDeclSrc> {
    vec![
        TypeDeclSrc { name: "Alpha".into(), parameters: Vec::new(),
            ctors: vec![CtorDeclSrc { name: "MkAlpha".into(), fields: Vec::new() }] },
        TypeDeclSrc { name: "Beta".into(), parameters: Vec::new(),
            ctors: vec![CtorDeclSrc { name: "MkBeta".into(), fields: Vec::new() }] },
    ]
}
fn ext_ctor() -> ExtExpr {
    ExtExpr::Literal { value: ExtValue::Constructor {
        ty: TypeExprSrc::Named { name: "Alpha".into() }, ctor: "MkAlpha".into(), fields: Vec::new() } }
}
fn ext_u(n: u128) -> ExtExpr {
    ExtExpr::Literal { value: ExtValue::Core { value: CoreValue::UInt { n: i128_class::KIntC::InRange(n) } } }
}
fn ext_leaf_tree() -> ExtExpr {
    ExtExpr::Literal { value: ExtValue::Core { value: CoreValue::Tree { tree: tree_leaf(1) } } }
}
fn ext_prog(entry: ExtExpr) -> ExtProgram {
    ExtProgram { types: nominal_types(), assets: Vec::new(), states: Vec::new(),
        inputs: Vec::new(), functions: Vec::new(), entry }
}
#[test]
fn nominal_constructor_mismatches_ext_both_modes() {
    run_ext(&leaf_spec(KindWant::Ctor, Space::Const, ERR11), "Leaf Ext CtorA11",
        &|| ext_prog(ExtExpr::Unary { op: UnaryOp::Leaf, operand: Box::new(ext_ctor()) }));
    run_ext(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Ctor, Space::Const, ERR11),
        "Cons Ext CtorB11",
        &|| ext_prog(ExtExpr::Binary { op: BinaryOp::Cons,
            left: Box::new(ext_u(7)), right: Box::new(ext_ctor()) }));
    run_ext(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Ctor, Space::Const, ERR11),
        "Branch Ext CtorB11",
        &|| ext_prog(ExtExpr::Binary { op: BinaryOp::Branch,
            left: Box::new(ext_leaf_tree()), right: Box::new(ext_ctor()) }));
}

fn assert_p3(r: &CompleteStepRecord, w: &Spec, label: &str) {
    assert!(is_selected(&r.before, w), "{label}: P3 selected PRE");
    assert_preserved(r, label);
    let (pre, post, e) = (&r.before.image, &r.after.image, &r.event);
    assert_eq!(pre.helper_work, r.before.admission.bounds.max_helper_work,
        "{label}: PRE helper_work at exact original admission bound");
    assert_eq!(e.rule_id, RuleIdC::BHelperScalarConstruct, "{label}: P3 rule");
    assert_eq!(e.disposition, StepDisposition::RefusedHelperWorkUncharged, "{label}: P3 disposition");
    assert_eq!((e.work_delta, e.helper_work_delta, e.helper_visits_delta), (0, 0, 0), "{label}: P3 zero charges");
    assert_eq!(e.reference_cursor_delta, 0, "{label}: P3 cursor");
    assert_eq!(e.alloc_deltas, AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }, "{label}: P3 alloc");
    assert_eq!(e.exhausted_kind, Some(ExhaustReason::HelperWork), "{label}: P3 exhausted kind");
    assert_eq!(e.outcome_payload, OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }, "{label}: P3 outcome");
    assert_eq!(e.boundary, BoundaryKind::Terminal, "{label}: P3 boundary");
    let (phase, cursor) = match w.mode { ScalarMode::Leaf => (ScalarPhase::First, 0), _ => (ScalarPhase::Second, 1) };
    assert_eq!(e.helper_phase_before, Some(HelperPhaseSummaryC::Scalar(phase)), "{label}: P3 phase before");
    assert_eq!(e.helper_phase_after, Some(HelperPhaseSummaryC::Scalar(phase)), "{label}: P3 phase preserved");
    assert_eq!(e.helper_cursor_before, Some(cursor), "{label}: P3 cursor before");
    assert_eq!(e.helper_cursor_after, Some(cursor), "{label}: P3 cursor preserved");
    assert_eq!(post.remaining_work, pre.remaining_work, "{label}: P3 remaining");
    assert_eq!(post.consumed_work, pre.consumed_work, "{label}: P3 consumed");
    assert_eq!(post.helper_work, pre.helper_work, "{label}: P3 helper work");
    assert_eq!(post.helper_visits, pre.helper_visits, "{label}: P3 visits");
    assert_eq!(post.helper, pre.helper, "{label}: P3 installed helper preserved");
    assert_eq!(post.outcome, OutcomeC::Exhausted { reason: ExhaustReason::HelperWork }, "{label}: P3 post outcome");
}
fn run_p3_core(w: &Spec, label: &str, make: &dyn Fn() -> CoreProgram) {
    let (_, _, probe) = reach_core(generous(), w, label, make);
    let b = bounds(probe.image.helper_work);
    let (admitted, mut image, pre) = reach_core(b, w, label, make);
    let (radm, mut rimg, _) = reach_core(b, w, label, make);
    let replay = match owner_record(&radm, &mut rimg) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("{label}: P3 replay terminal"),
        Err(e) => panic!("{label}: P3 replay: {}", capture_label(&e)),
    };
    assert_eq!(replay.before, pre, "{label}: P3 replay PRE");
    assert_p3(&replay, w, label);
    let witness = record_scalar_construct_step(&admitted, &mut image, &core_cfg(b), &limits())
        .unwrap_or_else(|e| panic!("{label}: P3 wrapper: {e}"));
    let post = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: P3 capture POST: {}", violation_label(&e)));
    assert_eq!(post, replay.after, "{label}: P3 original POST == replay POST");
    assert!(witness.disposition_is_refused_helper_work(), "{label}: P3 witness disposition");
    assert_both_modes(&witness, label);
    // Terminal next call: owner P0, wrapper refusal, image unchanged by that call.
    match record_scalar_construct_step(&admitted, &mut image, &core_cfg(b), &limits()) {
        Err(C6ScalarConstructConsumerError::UnexpectedOwnerStep) => {}
        other => panic!("{label}: terminal wrapper must be UnexpectedOwnerStep, ok={}", other.is_ok()),
    }
    let after_terminal = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: terminal capture: {}", violation_label(&e)));
    assert_eq!(after_terminal, post, "{label}: terminal call does not step");
}
#[test]
fn p3_exact_original_admission_all_modes_both_modes() {
    run_p3_core(&leaf_spec(KindWant::UInt(0), Space::Const, pend(NodeKind::Leaf)), "P3 Leaf First0",
        &|| leaf_of(lit_u(0)));
    run_p3_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Nil, Space::Const, pend(NodeKind::Cons)),
        "P3 Cons Second1", &|| cons_of(lit_u(7), lit_list(&[])));
    run_p3_core(&branch_spec(KindWant::Leaf, Space::Const, KindWant::Leaf, Space::Const, pend(NodeKind::Branch)),
        "P3 Branch Second1", &|| branch_of(lit_tree(tree_leaf(1)), lit_tree(tree_leaf(2))));
}

#[test]
fn p2_fuel_zero_separate_no_construct() {
    let w = cons_spec(KindWant::UInt(7), Space::Const, KindWant::Nil, Space::Const, pend(NodeKind::Cons));
    let make = &|| cons_of(lit_u(7), lit_list(&[]));
    let (_, _, probe) = reach_core(generous(), &w, "P2 probe", make);
    let fuel = probe.image.consumed_work;
    let (admitted, mut image, pre) = reach_core_fuel(generous(), fuel, &w, "P2 Cons", make);
    assert_eq!(pre.image.remaining_work, 0, "P2: PRE fuel exactly zero");
    assert!(is_selected(&pre, &w), "P2: shape is Construct PRE before driver P2");
    let (radm, mut rimg, _) = reach_core_fuel(generous(), fuel, &w, "P2 Cons replay", make);
    let replay = match owner_record(&radm, &mut rimg) {
        Ok(ObservedStep::Event(r)) => r,
        Ok(ObservedStep::Terminal { .. }) => panic!("P2: replay must publish Exhausted event"),
        Err(e) => panic!("P2: replay: {}", capture_label(&e)),
    };
    assert_eq!(replay.event.rule_id, RuleIdC::KAdminPublishExhausted, "P2: publish rule, not Construct/P6");
    assert_eq!(replay.event.disposition, StepDisposition::ExhaustedFuelUncharged, "P2: disposition");
    assert_eq!(replay.event.exhausted_kind, Some(ExhaustReason::Fuel), "P2: fuel");
    assert_eq!(replay.after.image.helper, replay.before.image.helper, "P2: helper preserved");
    assert_eq!(replay.after.image.consumed_work, replay.before.image.consumed_work, "P2: no charge");
    assert_eq!(replay.after.image.outcome, OutcomeC::Exhausted { reason: ExhaustReason::Fuel }, "P2: outcome");
    // Wrapper on an independently replayed P2 image: exact host-consistency refusal; step already happened.
    match record_scalar_construct_step(&admitted, &mut image, &core_cfg(generous()), &limits()) {
        Err(C6ScalarConstructConsumerError::EventMismatch { field: "disposition" }) => {}
        other => panic!("P2: wrapper must refuse disposition, ok={}", other.is_ok()),
    }
    let post = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("P2: capture POST: {}", violation_label(&e)));
    assert_eq!(post, replay.after, "P2: wrapper image equals raw P2 POST (no rollback)");
}

#[test]
fn wrapper_wrong_rule_refusal_is_nonrollback() {
    let b = generous();
    let engine = host_owner_engine();
    let admitted = engine.admit(&prog(lit_u(1)), &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("wrong-rule admit: {}", tooling_label(&e)));
    let mut image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("wrong-rule genesis: {}", tooling_label(&e)));
    let before = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("wrong-rule capture: {}", violation_label(&e)));
    match record_scalar_construct_step(&admitted, &mut image, &core_cfg(b), &limits()) {
        Err(C6ScalarConstructConsumerError::UnexpectedOwnerStep) => {}
        other => panic!("wrong-rule: expected UnexpectedOwnerStep, ok={}", other.is_ok()),
    }
    let after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("wrong-rule capture POST: {}", violation_label(&e)));
    assert_ne!(after, before, "wrong-rule: Err does not mean no step happened");
    assert_eq!(after.image.consumed_work, before.image.consumed_work + 1,
        "wrong-rule: the one owner step's charge persists (no rollback)");
}

#[test]
fn host_pair_canonical_wrong_post_reaches_checker_both_modes() {
    let a = run_core(&leaf_spec(KindWant::UInt(0), Space::Const, pend(NodeKind::Leaf)),
        "wrong-post base Leaf", &|| leaf_of(lit_u(0)));
    let mut wrong_after = a.record.after.clone();
    if let Some(HelperControlC::Scalar { staged, .. }) = &mut wrong_after.image.helper {
        *staged = ERR11; // canonical encodable row, semantically wrong for UInt Leaf
    }
    // No semantic producer filter: encodable wrong post must build a witness.
    let witness = witness_scalar_construct_from_host_pair(
            true, &a.record.before, &wrong_after, &core_cfg(generous()), &limits())
        .unwrap_or_else(|e| panic!("wrong-post: host-pair must not semantically filter: {e}"));
    for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
        let circuit = build_c6_scalar_construct_step_circuit(witness.source_params().clone(),
            witness.host_prefix().meta.len(), witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset, mode, "scalar_construct_wrong_post")
            .unwrap_or_else(|e| panic!("wrong-post build {mode:?}: {e}"));
        // Checker recomputes the LedgerCall commitment from this current
        // statement+post; rejection is therefore the relation, not stale CC.
        match check_c6_scalar_construct_step_circuit(&circuit, &witness) {
            Err(C6ScalarConstructConsumerError::Backend(m)) => assert!(
                m.contains("SDK ProofPreimage::check failed"),
                "wrong-post {mode:?}: SDK preprocessing category required, got: {m}"),
            other => panic!("wrong-post {mode:?}: expected checker rejection, ok={}", other.is_ok()),
        }
    }
}

#[test]
fn owner_record_mutations_reject_exact_host_consistency() {
    let leaf = run_core(&leaf_spec(KindWant::UInt(0), Space::Const, pend(NodeKind::Leaf)),
        "record-mutation Leaf", &|| leaf_of(lit_u(0)));
    let mut bad_event = leaf.record.clone();
    bad_event.event.helper_visits_delta = 0;
    match witness_scalar_construct_from_owner_record(&bad_event, &core_cfg(generous()), &limits()) {
        Err(C6ScalarConstructConsumerError::EventMismatch { field: "helper_visits_delta" }) => {}
        other => panic!("mutated visits: expected EventMismatch, ok={}", other.is_ok()),
    }
    let mut bad_after = leaf.record.clone();
    if let Some(HelperControlC::Scalar { staged, .. }) = &mut bad_after.after.image.helper {
        *staged = ERR11;
    }
    match witness_scalar_construct_from_owner_record(&bad_after, &core_cfg(generous()), &limits()) {
        Err(C6ScalarConstructConsumerError::EventMismatch { field: "snapshot helper after" }) => {}
        other => panic!("mutated after: expected EventMismatch, ok={}", other.is_ok()),
    }
    let cons = run_core(&cons_spec(KindWant::UInt(7), Space::Const, KindWant::Nil, Space::Const, pend(NodeKind::Cons)),
        "record-mutation Cons", &|| cons_of(lit_u(7), lit_list(&[])));
    let mut crossed = leaf.record.clone();
    crossed.after = cons.record.after.clone();
    // Internal consistency only; this is not authentication of provenance.
    match witness_scalar_construct_from_owner_record(&crossed, &core_cfg(generous()), &limits()) {
        Err(C6ScalarConstructConsumerError::EventMismatch { .. }) => {}
        other => panic!("crossed record: expected EventMismatch, ok={}", other.is_ok()),
    }
}
#[test]
fn marker_rows_admitted_not_held_genuine_baseline_both_modes() {
    // (a) Marker and transitive composites are admitted in the
    // constant table of unused functions. The selected genuine
    // Construct PRE holds only the UInt entry operand.
    let w = leaf_spec(KindWant::UInt(0), Space::Const, pend(NodeKind::Leaf));
    let make = &|| marker_unused_prog(unary(UnaryOp::Leaf, lit_u(0)));
    let (_, _, pre) = reach_core(generous(), &w, "Marker not-held probe", make);
    let markers = marker_const_ids(&pre);
    assert!(!markers.is_empty(), "genuine prepare must create U128OutOfRange rows");
    let composites = transitive_false_composite_ids(&pre);
    assert!(composites.iter().any(|i| const_kind_name(&pre, *i) == "Cons"),
        "list containing a Marker must be transitively marker_free=false");
    assert!(composites.iter().any(|i| const_kind_name(&pre, *i) == "Branch"),
        "tree containing a Marker must be transitively marker_free=false");
    assert!(composites.iter().all(|i| !const_marker_free(&pre, *i)));
    let (held, exempt, holds) = snapshot_si17_projection(&pre);
    assert!(holds, "snapshot SI17 projection must hold at genuine Construct PRE");
    for id in markers.iter().chain(composites.iter()) {
        assert!(!held.contains(&RefC::Const(*id)),
            "unused Marker/composite Const {id} must not be held at Construct PRE");
        assert!(!exempt.contains(&RefC::Const(*id)),
            "unused Marker/composite Const {id} is not a Validate exemption either");
    }
    // Full normal baseline: fresh factories, selected step exactly
    // once in the wrapper, independent replay, both modes. This is
    // run_core/assert_applied/assert_preserved unchanged.
    run_core(&w, "Marker not-held Leaf baseline", make);
}

fn collect_marker_boundary(make: &dyn Fn() -> CoreProgram, label: &str) -> Vec<CompleteStepRecord> {
    let b = generous();
    let engine = host_owner_engine();
    let admitted = engine.admit(&make(), &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let mut image = engine.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    let mut records = Vec::new();
    for _ in 0..64 {
        match owner_record(&admitted, &mut image) {
            Ok(ObservedStep::Event(r)) => {
                let done = r.event.rule_id == RuleIdC::KMachineEvalLiteralInvalid;
                records.push(r);
                if done { return records; }
            }
            Ok(ObservedStep::Terminal { after, .. }) =>
                panic!("{label}: terminal before LiteralInvalid: {:?}", after.image.outcome),
            Err(e) => panic!("{label}: boundary step: {}", capture_label(&e)),
        }
    }
    panic!("{label}: genuine owner never reached LiteralInvalid")
}
fn assert_marker_boundary(records: &[CompleteStepRecord], root_kind: &str, label: &str) {
    assert!(!records.is_empty(), "{label}: records");
    for r in records {
        assert_ne!(r.event.rule_id, RuleIdC::BHelperScalarConstruct,
            "{label}: invalid literal must never reach ScalarConstruct");
        assert!(!matches!(r.after.image.helper, Some(HelperControlC::Scalar { .. })),
            "{label}: no Scalar helper may be installed on the literal path");
        for snap in [&r.before, &r.after] {
            let (_, _, holds) = snapshot_si17_projection(snap);
            assert!(holds, "{label}: SI17 snapshot projection holds at every boundary snapshot");
        }
        assert_eq!(r.after.admission, r.before.admission, "{label}: admission preserved");
        assert_eq!(r.after.image.data, r.before.image.data, "{label}: DATA preserved");
        assert_eq!(r.after.image.data_fields, r.before.image.data_fields, "{label}: DATA fields preserved");
        assert_eq!(r.after.image.env, r.before.image.env, "{label}: env preserved");
        assert_eq!(r.after.image.value_slots, r.before.image.value_slots, "{label}: slots preserved");
    }
    let entry = records.iter().find(|r| r.event.rule_id == RuleIdC::BHelperValidateEntry)
        .unwrap_or_else(|| panic!("{label}: ValidateEntry required"));
    assert_eq!(entry.event.event_kind, EventKind::HelperEnter, "{label}: entry kind");
    assert_eq!(entry.event.disposition, StepDisposition::Applied, "{label}: entry disposition");
    let root = match entry.after.image.helper {
        Some(HelperControlC::Validate { root, status: TriStatus::Pending, .. }) => root,
        _ => panic!("{label}: ValidateEntry must install Validate/Pending"),
    };
    let RefC::Const(root_id) = root else { panic!("{label}: literal root must be Const") };
    assert_eq!(const_kind_name(&entry.after, root_id), root_kind, "{label}: literal root kind");
    assert!(!const_marker_free(&entry.after, root_id), "{label}: literal root marker_free=false");
    assert_eq!(entry.after.image.tasks,
        vec![HelperTaskC::ValidateTask { r: root, visit: TaskVisitC::Unvisited, cursor: 0 }],
        "{label}: exactly the root ValidateTask after entry");
    let (held, exempt, holds) = snapshot_si17_projection(&entry.after);
    assert!(holds && !held.contains(&root), "{label}: Validate root is exempt, not held");
    assert_eq!(exempt, vec![root, root],
        "{label}: only Validate.root and ValidateTask.r positions are exempt");
    assert!(records.iter().any(|r| r.event.rule_id == RuleIdC::BHelperValidateInvalid),
        "{label}: a Marker occurrence must be judged invalid");
    let failure = &records[records.len() - 2];
    assert_eq!(failure.event.rule_id, RuleIdC::BHelperValidateFailure, "{label}: ValidateFailure precedes LiteralInvalid");
    assert_eq!(failure.after.image.result, Some(HelperResultC::Validated { ok: false }),
        "{label}: primitive validation result is false, not PrimError11");
    assert!(failure.after.image.helper.is_none() && failure.after.image.tasks.is_empty(),
        "{label}: failure clears helper/tasks");
    let last = &records[records.len() - 1];
    let (pre, post, e) = (&last.before.image, &last.after.image, &last.event);
    assert_eq!(e.rule_id, RuleIdC::KMachineEvalLiteralInvalid, "{label}: final rule");
    assert_eq!(e.event_kind, EventKind::LegacyComplete, "{label}: final kind");
    assert_eq!(e.site, Some(SiteTag::EvalLiteral), "{label}: final site");
    assert_eq!(e.disposition, StepDisposition::Applied, "{label}: final disposition");
    assert_eq!((e.work_delta, e.helper_work_delta, e.helper_visits_delta, e.reference_cursor_delta),
        (1, 0, 0, 1), "{label}: legacy deltas");
    assert_eq!(e.step_id.ordinal, pre.consumed_work, "{label}: ordinal");
    assert_eq!(*e.step_id.run_id, pre.run_id, "{label}: step run id");
    assert!(matches!(pre.control, ControlC::Eval { .. }), "{label}: pre control Eval");
    assert_eq!(pre.result, Some(HelperResultC::Validated { ok: false }), "{label}: pre result");
    assert_eq!(post.control, ControlC::Error { code: 2 },
        "{label}: KMachineEvalLiteralInvalid/ControlError2, distinct from PrimError11");
    assert!(post.result.is_none() && post.helper.is_none() && post.tasks.is_empty(),
        "{label}: post helper/result/tasks absent");
    assert_eq!(post.outcome, OutcomeC::Running, "{label}: publication is a later step");
    assert_eq!(post.remaining_work, pre.remaining_work - 1, "{label}: remaining-1");
    assert_eq!(post.consumed_work, pre.consumed_work + 1, "{label}: consumed+1");
    assert_eq!(post.helper_work, pre.helper_work, "{label}: helper_work unchanged");
    assert_eq!(post.helper_visits, pre.helper_visits, "{label}: visits unchanged");
    assert_eq!(post.reference_cursor, pre.reference_cursor + 1, "{label}: legacy cursor+1");
    assert_eq!(post.frames, pre.frames, "{label}: frames preserved");
    assert_eq!(post.alloc, pre.alloc, "{label}: alloc preserved");
    assert_eq!(post.active, pre.active, "{label}: active preserved");
}
fn run_marker_boundary(make: &dyn Fn() -> CoreProgram, root_kind: &str, label: &str) {
    // Independent fresh replay of the whole genuine boundary trace.
    let records = collect_marker_boundary(make, label);
    let replay = collect_marker_boundary(make, label);
    assert_eq!(records, replay, "{label}: fresh replay equals original trace exactly");
    assert_marker_boundary(&records, root_kind, label);
}
#[test]
fn marker_literal_direct_validate_boundary_no_scalar_construct() {
    run_marker_boundary(&|| prog(lit_marker_neg()), "Marker", "direct Negative marker");
    run_marker_boundary(&|| prog(lit_marker_above()), "Marker", "direct AboveU128 marker");
}
#[test]
fn marker_containing_literal_validate_boundary_no_scalar_construct() {
    run_marker_boundary(&|| prog(lit_list_marker()), "Cons", "containing list marker");
    run_marker_boundary(&|| prog(lit_tree_marker_branch()), "Branch", "containing tree marker");
}
