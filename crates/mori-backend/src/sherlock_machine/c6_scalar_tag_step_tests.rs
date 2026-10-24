//! Genuine-owner positive tests for `c6_scalar_tag_consumer`.
//!
//! Source-only and UNRUN. Every fixture is admitted through the real
//! public owner and searched with the real `record_step` until the
//! actual selected Scalar `First` state. The selected step is taken
//! once by `record_scalar_tag_step` on the original image and once by
//! raw `record_step` on an independently replayed image.
//!
//! DELIVERED_SOURCE: all eight Tag modes; Const and Data operand
//! spaces where the genuine programs below reach them; wrong-kind
//! Done/PrimError{11}; TestBool selectors; builtin and nominal
//! TestType; applied P4/P7 effects; P3 at an exact fresh bound;
//! LocalPublic and LedgerCall ProofPreimage::check preprocessing.
//!
//! REQUIRED_OPEN: raw Const Marker, invalid nominal tuples, raw
//! Fr/ref-space/gate domain negatives, wrong-independent-post
//! circuit negatives, SI08/SI17/SI21/SI22, StepInv, program
//! membership, nominal bridge theorem, and native issuance.

use super::c6_push_step::C6PushStatementBindingMode;
use super::{
    C6ScalarTagConsumerError, C6ScalarTagStepWitness, build_c6_scalar_tag_step_circuit,
    check_c6_scalar_tag_step_circuit, record_scalar_tag_step, witness_scalar_tag_from_host_pair,
    witness_scalar_tag_from_owner_record,
};
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, ConstantSnapshot, DataSnapshot, HostSnapshot,
    LayoutSnapshot, ObservedStep, capture, record_step as owner_record,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_admit::ExtCapacitiesC;
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{
    ExtDomain, ExtProgram,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtExpr, ExtFunction, ExtValue,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, ExtTypeBuildError, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

fn violation_label(v: &HostViolation) -> String {
    format!(
        "kind={:?}, clause={:?}, rule={:?}, locator={}",
        v.kind,
        v.clause,
        v.rule,
        v.locator.as_deref().unwrap_or("<none>")
    )
}

fn capture_label(e: &CaptureStepError) -> String {
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

fn tooling_label(e: &Tooling) -> String {
    match e {
        Tooling::AdmissionRejected {
            reason,
            detail,
            locator,
        } => format!("admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"),
        Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        } => format!(
            "capacity not admitted: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        Tooling::UnsupportedFeature { feature, locator } => {
            format!("unsupported feature: feature={feature:?}, locator={locator:?}")
        }
    }
}

fn ext_label(e: &ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(t) => tooling_label(t),
        ExtTypeBuildError::ExtCapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        } => format!(
            "ext capacity: kind={kind:?}, needed={needed:?}, limit={limit}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtAdmissionRejected {
            reason,
            detail,
            locator,
        } => format!(
            "ext admission rejected: reason={reason:?}, detail={detail}, locator={locator:?}"
        ),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } => {
            format!("ext unsupported feature: feature={feature:?}, locator={locator:?}")
        }
        ExtTypeBuildError::Host(v) => format!("ext host violation: {}", violation_label(v)),
    }
}

fn bounds(max_helper_work: u128) -> BoundsC {
    BoundsC {
        code_rows: 64,
        function_rows: 8,
        code_child_slots: 64,
        parameter_slots: 16,
        pattern_rows: 16,
        match_plan_rows: 16,
        match_arm_rows: 16,
        type_rows: 16,
        constructor_rows: 16,
        constructor_field_slots: 16,
        max_arity: 4,
        constant_nodes: 64,
        constant_field_slots: 64,
        data_nodes: 16,
        data_field_slots: 16,
        environment_cells: 16,
        frames: 16,
        helper_tasks: 4,
        serialization_bytes: 1_000_000,
        max_reference_work: 1_000_000,
        max_helper_work,
        step_cap: 1_000_000,
    }
}

fn generous() -> BoundsC {
    bounds(100)
}

fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC {
        name_rows: 32,
        name_bytes: 512,
        occurrence_path_slots: 64,
        value_slots: 512,
    }
}

fn layout(b: BoundsC, ext: Option<ExtCapacitiesC>) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: capacities(),
        ext_capacities: ext,
        boundary_capacities_present: false,
    }
}

fn cfg(b: BoundsC, ext: Option<ExtCapacitiesC>) -> C6HostConfig {
    C6HostConfig::from_retained_layout(
        &layout(b, ext),
        C6CodecBudgets {
            type_expr_nodes: 256,
            template_bytes: 4096,
        },
    )
}

fn core_cfg(b: BoundsC) -> C6HostConfig {
    cfg(b, None)
}

fn ext_cfg(b: BoundsC) -> C6HostConfig {
    cfg(b, Some(ExtCapacitiesC::all_zero()))
}

fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 1 << 20,
        max_state_lanes: 1 << 20,
        max_metadata_bytes: 8192,
        max_string_bytes: 512,
        max_width_entries: 64,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Space {
    Const,
    Data,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KindWant {
    UInt(u128),
    Bool(bool),
    Nil,
    Cons,
    Leaf,
    Branch,
    Ctor { type_id: usize, ctor_id: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PostWant {
    SecondRegA,
    SecondRegZero,
    DoneError11,
    DoneSelector(BoolSelector),
    DoneTypeTest(bool),
}

#[derive(Clone, Copy, Debug)]
struct Spec {
    mode: ScalarMode,
    site: SiteTag,
    space: Space,
    kind: KindWant,
    post: PostWant,
}

fn spec(mode: ScalarMode, site: SiteTag, space: Space, kind: KindWant, post: PostWant) -> Spec {
    Spec {
        mode,
        site,
        space,
        kind,
        post,
    }
}

fn operand_kind(snapshot: &HostSnapshot, reference: RefC) -> KindWant {
    match reference {
        RefC::Const(id) => match snapshot.admission.view.constants.get(id) {
            Some(ConstantSnapshot::U128 { n, .. }) => KindWant::UInt(*n),
            Some(ConstantSnapshot::Bool { b, .. }) => KindWant::Bool(*b),
            Some(ConstantSnapshot::Nil { .. }) => KindWant::Nil,
            Some(ConstantSnapshot::Cons { .. }) => KindWant::Cons,
            Some(ConstantSnapshot::Leaf { .. }) => KindWant::Leaf,
            Some(ConstantSnapshot::Branch { .. }) => KindWant::Branch,
            Some(ConstantSnapshot::Constructor {
                type_id, ctor_id, ..
            }) => KindWant::Ctor {
                type_id: type_id.0,
                ctor_id: ctor_id.0,
            },
            Some(ConstantSnapshot::U128OutOfRange { .. }) => {
                panic!("raw Marker operand is REQUIRED_OPEN, not a delivered positive")
            }
            None => panic!("selected Const operand index must exist"),
        },
        RefC::Data(id) => match snapshot.image.data.get(id) {
            Some(DataSnapshot::U128 { n }) => KindWant::UInt(*n),
            Some(DataSnapshot::Bool { b }) => KindWant::Bool(*b),
            Some(DataSnapshot::Nil) => KindWant::Nil,
            Some(DataSnapshot::Cons { .. }) => KindWant::Cons,
            Some(DataSnapshot::Leaf { .. }) => KindWant::Leaf,
            Some(DataSnapshot::Branch { .. }) => KindWant::Branch,
            Some(DataSnapshot::Constructor {
                type_id, ctor_id, ..
            }) => KindWant::Ctor {
                type_id: type_id.0,
                ctor_id: ctor_id.0,
            },
            None => panic!("selected Data operand index must exist"),
        },
    }
}

fn selected_helper(snapshot: &HostSnapshot) -> HelperControlC {
    match snapshot.image.helper {
        Some(h @ HelperControlC::Scalar { .. }) => h,
        _ => panic!("selected snapshot must carry a Scalar helper"),
    }
}

fn is_selected(snapshot: &HostSnapshot, wanted: &Spec) -> bool {
    let helper = match snapshot.image.helper {
        Some(h) => h,
        None => return false,
    };
    let HelperControlC::Scalar {
        common,
        mode,
        operand_a,
        phase,
        reg_a,
        staged,
        ..
    } = helper
    else {
        return false;
    };
    if phase != ScalarPhase::First || mode != wanted.mode {
        return false;
    }
    if common.site != wanted.site || reg_a != 0 || staged != ScalarStaged::None {
        return false;
    }
    let actual_space = match operand_a {
        RefC::Const(_) => Space::Const,
        RefC::Data(_) => Space::Data,
    };
    actual_space == wanted.space && operand_kind(snapshot, operand_a) == wanted.kind
}

fn search(
    admitted: Admitted,
    mut image: MachineImage,
    wanted: &Spec,
    label: &str,
) -> (Admitted, MachineImage, HostSnapshot) {
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search step: {}", capture_label(&e)));
        let after = match &step {
            ObservedStep::Event(record) => &record.after,
            ObservedStep::Terminal { after, .. } => panic!(
                "{label}: terminal before selected Scalar First state: {:?}",
                after.image.outcome
            ),
        };
        if is_selected(after, wanted) {
            let selected = capture(&admitted, &image).unwrap_or_else(|e| {
                panic!(
                    "{label}: capture selected pre-state: {}",
                    violation_label(&e)
                )
            });
            assert_eq!(
                &selected, after,
                "{label}: capture must equal searched pre-state"
            );
            return (admitted, image, selected);
        }
    }
    panic!("{label}: genuine owner did not reach the selected Scalar First state")
}

fn reach_core(
    b: BoundsC,
    wanted: &Spec,
    label: &str,
    make: &dyn Fn() -> CoreProgram,
) -> (Admitted, MachineImage, HostSnapshot) {
    let program = make();
    let engine = host_owner_engine();
    let admitted = engine
        .admit(&program, &b, &layout(b, None))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    search(admitted, image, wanted, label)
}

fn reach_ext(
    b: BoundsC,
    wanted: &Spec,
    label: &str,
    make: &dyn Fn() -> ExtProgram,
) -> (Admitted, MachineImage, HostSnapshot) {
    let program = make();
    let (admitted, _report) = admit_ext_program(
        program,
        ExtDomain::GENERIC_ABSENT,
        &b,
        &layout(b, Some(ExtCapacitiesC::all_zero())),
        None,
    )
    .unwrap_or_else(|e| panic!("{label}: admit_ext_program: {}", ext_label(&e)));
    let engine = host_owner_engine();
    let image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: ext genesis: {}", tooling_label(&e)));
    search(admitted, image, wanted, label)
}

fn replay_core(
    b: BoundsC,
    wanted: &Spec,
    label: &str,
    make: &dyn Fn() -> CoreProgram,
) -> CompleteStepRecord {
    let (admitted, mut image, _) = reach_core(b, wanted, label, make);
    match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(record)) => record,
        Ok(ObservedStep::Terminal { .. }) => {
            panic!("{label}: selected raw step must be an event")
        }
        Err(e) => panic!("{label}: selected raw step: {}", capture_label(&e)),
    }
}

fn replay_ext(
    b: BoundsC,
    wanted: &Spec,
    label: &str,
    make: &dyn Fn() -> ExtProgram,
) -> CompleteStepRecord {
    let (admitted, mut image, _) = reach_ext(b, wanted, label, make);
    match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(record)) => record,
        Ok(ObservedStep::Terminal { .. }) => {
            panic!("{label}: selected raw ext step must be an event")
        }
        Err(e) => panic!("{label}: selected raw ext step: {}", capture_label(&e)),
    }
}

fn post_fields(wanted: &Spec) -> (ScalarPhase, u128, ScalarStaged) {
    match wanted.post {
        PostWant::SecondRegA => {
            let KindWant::UInt(n) = wanted.kind else {
                panic!("SecondRegA requires an exact UInt operand")
            };
            (ScalarPhase::Second, n, ScalarStaged::None)
        }
        PostWant::SecondRegZero => (ScalarPhase::Second, 0, ScalarStaged::None),
        PostWant::DoneError11 => (ScalarPhase::Done, 0, ScalarStaged::PrimError { code: 11 }),
        PostWant::DoneSelector(selector) => (
            ScalarPhase::Done,
            0,
            ScalarStaged::Selector { sel: selector },
        ),
        PostWant::DoneTypeTest(ok) => (ScalarPhase::Done, 0, ScalarStaged::TypeTest { ok }),
    }
}

fn assert_selected_pre(record: &CompleteStepRecord, wanted: &Spec, label: &str) {
    let pre = &record.before.image;
    let helper = selected_helper(&record.before);
    let HelperControlC::Scalar {
        common,
        mode,
        operand_a,
        operand_b,
        expected,
        phase,
        reg_a,
        staged,
    } = helper
    else {
        unreachable!()
    };
    assert_eq!(mode, wanted.mode, "{label}: mode");
    assert_eq!(common.site, wanted.site, "{label}: site");
    assert_eq!(phase, ScalarPhase::First, "{label}: phase");
    assert_eq!(reg_a, 0, "{label}: pre reg_a");
    assert_eq!(staged, ScalarStaged::None, "{label}: pre staged");
    assert!(common.parent_resume.is_none(), "{label}: parent_resume");
    assert_eq!(
        common.origin_reference_cursor, pre.reference_cursor,
        "{label}: origin cursor equals reference cursor"
    );
    let binary = matches!(
        mode,
        ScalarMode::Add
            | ScalarMode::Sub
            | ScalarMode::Mul
            | ScalarMode::Lt
            | ScalarMode::Cons
            | ScalarMode::Branch
    );
    assert_eq!(operand_b.is_some(), binary, "{label}: operand_b presence");
    assert_eq!(
        expected.is_some(),
        mode == ScalarMode::TestType,
        "{label}: expected presence"
    );
    assert_eq!(
        operand_kind(&record.before, operand_a),
        wanted.kind,
        "{label}: exact operand kind and payload"
    );
    match (wanted.space, operand_a) {
        (Space::Const, RefC::Const(_)) | (Space::Data, RefC::Data(_)) => {}
        _ => panic!("{label}: exact operand RefC space"),
    }
    assert!(pre.result.is_none(), "{label}: pre result");
    assert!(pre.tasks.is_empty(), "{label}: pre tasks");
    assert_eq!(pre.outcome, OutcomeC::Running, "{label}: pre outcome");
    assert!(pre.remaining_work > 0, "{label}: pre remaining work");
}

fn assert_preserved(record: &CompleteStepRecord, label: &str) {
    let before = &record.before;
    let after = &record.after;
    let pre = &before.image;
    let post = &after.image;
    assert_eq!(after.admission, before.admission, "{label}: admission");
    assert_eq!(post.run_id, pre.run_id, "{label}: run_id");
    assert_eq!(post.control, pre.control, "{label}: control");
    assert_eq!(
        post.control_origin, pre.control_origin,
        "{label}: control origin"
    );
    assert_eq!(post.env_root, pre.env_root, "{label}: env root");
    assert_eq!(post.stack_limit, pre.stack_limit, "{label}: stack limit");
    assert_eq!(post.initial_fuel, pre.initial_fuel, "{label}: initial fuel");
    assert_eq!(post.seg_base, pre.seg_base, "{label}: segment base");
    assert_eq!(post.frames, pre.frames, "{label}: frames");
    assert_eq!(post.env, pre.env, "{label}: env");
    assert_eq!(post.value_slots, pre.value_slots, "{label}: value slots");
    assert_eq!(post.tasks, pre.tasks, "{label}: tasks");
    assert_eq!(post.data, pre.data, "{label}: DATA rows");
    assert_eq!(post.data_fields, pre.data_fields, "{label}: DATA fields");
    assert_eq!(post.alloc, pre.alloc, "{label}: alloc counters");
    assert_eq!(post.active, pre.active, "{label}: active lengths");
    assert_eq!(
        post.reference_cursor, pre.reference_cursor,
        "{label}: reference cursor unchanged"
    );
}

fn assert_applied(record: &CompleteStepRecord, wanted: &Spec, label: &str) {
    assert_selected_pre(record, wanted, label);
    assert_preserved(record, label);
    let pre = &record.before.image;
    let post = &record.after.image;
    let event = &record.event;
    assert_eq!(event.rule_id, RuleIdC::BHelperScalarTag, "{label}: rule");
    assert_eq!(
        event.event_kind,
        EventKind::HelperVisit,
        "{label}: event kind"
    );
    assert_eq!(
        event.helper_kind,
        Some(HelperKind::Scalar),
        "{label}: helper kind"
    );
    assert_eq!(
        event.disposition,
        StepDisposition::Applied,
        "{label}: disposition"
    );
    assert_eq!(event.work_delta, 1, "{label}: work delta");
    assert_eq!(event.helper_work_delta, 1, "{label}: helper work delta");
    assert_eq!(event.helper_visits_delta, 1, "{label}: helper visits delta");
    assert_eq!(event.reference_cursor_delta, 0, "{label}: cursor delta");
    assert_eq!(
        event.alloc_deltas,
        AllocDeltasC {
            data_nodes: 0,
            data_fields: 0,
            environment_cells: 0
        },
        "{label}: alloc deltas"
    );
    assert_eq!(event.value_slot_writes, 0, "{label}: value slot writes");
    assert!(event.result_tag.is_none(), "{label}: result tag");
    assert!(event.exhausted_kind.is_none(), "{label}: exhausted kind");
    assert_eq!(
        event.outcome_tag,
        OutcomeTagC::Running,
        "{label}: outcome tag"
    );
    assert_eq!(
        event.outcome_payload,
        OutcomeC::Running,
        "{label}: outcome payload"
    );
    assert_eq!(event.boundary, BoundaryKind::None, "{label}: boundary");
    assert_eq!(
        event.step_id.ordinal, pre.consumed_work,
        "{label}: step ordinal"
    );
    assert_eq!(*event.step_id.run_id, pre.run_id, "{label}: event run_id");
    assert_eq!(event.site, Some(wanted.site), "{label}: event site");
    assert_eq!(
        event.origin_reference_cursor,
        Some(pre.reference_cursor),
        "{label}: event origin cursor"
    );
    assert_eq!(
        event.source_expr_id, pre.control_origin,
        "{label}: source expr"
    );
    assert_eq!(
        event.logical_depth_before,
        pre.frames.len(),
        "{label}: depth before"
    );
    assert_eq!(
        event.logical_depth_after,
        post.frames.len(),
        "{label}: depth after"
    );
    assert_eq!(event.task_cursor_before, 0, "{label}: task cursor before");
    assert_eq!(event.task_cursor_after, 0, "{label}: task cursor after");
    assert_eq!(
        event.helper_phase_before,
        Some(HelperPhaseSummaryC::Scalar(ScalarPhase::First)),
        "{label}: helper phase before"
    );
    assert_eq!(
        event.helper_cursor_before,
        Some(0),
        "{label}: helper cursor before"
    );
    assert_eq!(
        post.remaining_work,
        pre.remaining_work - 1,
        "{label}: remaining"
    );
    assert_eq!(
        post.consumed_work,
        pre.consumed_work + 1,
        "{label}: consumed"
    );
    assert_eq!(
        post.helper_work,
        pre.helper_work + 1,
        "{label}: helper work"
    );
    assert_eq!(
        post.helper_visits,
        pre.helper_visits + 1,
        "{label}: helper visits"
    );
    assert_eq!(post.outcome, OutcomeC::Running, "{label}: post outcome");

    let (phase, reg_a, staged) = post_fields(wanted);
    let mut expected_helper = selected_helper(&record.before);
    if let HelperControlC::Scalar {
        phase: actual_phase,
        reg_a: actual_reg_a,
        staged: actual_staged,
        ..
    } = &mut expected_helper
    {
        *actual_phase = phase;
        *actual_reg_a = reg_a;
        *actual_staged = staged;
    }
    assert_eq!(
        post.helper,
        Some(expected_helper),
        "{label}: complete post helper"
    );
    assert_eq!(
        event.helper_phase_after,
        Some(HelperPhaseSummaryC::Scalar(phase)),
        "{label}: helper phase after"
    );
    let expected_cursor = match phase {
        ScalarPhase::Second => 1,
        ScalarPhase::Done => 2,
        ScalarPhase::First => 0,
    };
    assert_eq!(
        event.helper_cursor_after,
        Some(expected_cursor),
        "{label}: helper cursor after"
    );
}

fn assert_checks(witness: &C6ScalarTagStepWitness, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let circuit = build_c6_scalar_tag_step_circuit(
            witness.source_params().clone(),
            witness.host_prefix().meta.len(),
            witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset,
            mode,
            "scalar_tag_owner_test",
        )
        .unwrap_or_else(|e| panic!("{label}: circuit builds in {mode:?}: {e}"));
        check_c6_scalar_tag_step_circuit(&circuit, witness).unwrap_or_else(|e| {
            panic!("{label}: SDK ProofPreimage::check must pass in {mode:?}: {e}")
        });
    }
}

struct Applied {
    record: CompleteStepRecord,
    witness: C6ScalarTagStepWitness,
}

fn finish_applied(
    record: CompleteStepRecord,
    wanted: &Spec,
    label: &str,
    admitted: &Admitted,
    image: &mut MachineImage,
    config: &C6HostConfig,
) -> Applied {
    let witness = record_scalar_tag_step(admitted, image, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: selected wrapper step: {e}"));
    let original_after = capture(admitted, image).unwrap_or_else(|e| {
        panic!(
            "{label}: capture original wrapper post: {}",
            violation_label(&e)
        )
    });
    assert_eq!(
        original_after, record.after,
        "{label}: original wrapper post must equal replayed raw post"
    );
    assert_applied(&record, wanted, label);
    assert!(!witness.enable() == false, "{label}: witness enable");
    assert!(
        !witness.disposition_is_refused_helper_work(),
        "{label}: applied disposition flag"
    );
    let host_pair =
        witness_scalar_tag_from_host_pair(true, &record.before, &record.after, config, &limits())
            .unwrap_or_else(|e| panic!("{label}: host-pair witness: {e}"));
    assert_eq!(
        host_pair, witness,
        "{label}: host-pair equals owner-record witness"
    );
    let owner_record_witness = witness_scalar_tag_from_owner_record(&record, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: owner-record witness: {e}"));
    assert_eq!(
        owner_record_witness, witness,
        "{label}: record witness equality"
    );
    Applied { record, witness }
}

fn run_core(wanted: &Spec, label: &str, make: &dyn Fn() -> CoreProgram) -> Applied {
    let b = generous();
    let (admitted, mut image, pre) = reach_core(b, wanted, label, make);
    let record = replay_core(b, wanted, label, make);
    assert_eq!(
        record.before, pre,
        "{label}: replay reaches identical selected pre-state"
    );
    finish_applied(record, wanted, label, &admitted, &mut image, &core_cfg(b))
}

fn run_ext(wanted: &Spec, label: &str, make: &dyn Fn() -> ExtProgram) -> Applied {
    let b = generous();
    let (admitted, mut image, pre) = reach_ext(b, wanted, label, make);
    let record = replay_ext(b, wanted, label, make);
    assert_eq!(
        record.before, pre,
        "{label}: replay reaches identical selected pre-state"
    );
    finish_applied(record, wanted, label, &admitted, &mut image, &ext_cfg(b))
}

fn lit_u(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: i128_class::KIntC::InRange(n),
        },
    }
}

fn lit_b(value: bool) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::Bool { b: value },
    }
}

fn lit_list(values: &[u128]) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::List {
            elements: values
                .iter()
                .map(|n| i128_class::KIntC::InRange(*n))
                .collect(),
        },
    }
}

fn tree_leaf(n: u128) -> CoreTree {
    CoreTree::Leaf {
        n: i128_class::KIntC::InRange(n),
    }
}

fn lit_tree(tree: CoreTree) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::Tree { tree },
    }
}

fn bin(op: BinaryOp, left: CoreExpr, right: CoreExpr) -> CoreExpr {
    CoreExpr::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn program(entry: CoreExpr) -> CoreProgram {
    CoreProgram {
        functions: Vec::new(),
        entry,
    }
}

fn call_program(return_type: SortTag, body: CoreExpr) -> CoreProgram {
    CoreProgram {
        functions: vec![CoreFunction {
            name: "f".into(),
            parameters: Vec::new(),
            return_type,
            body,
        }],
        entry: CoreExpr::Call {
            name: "f".into(),
            args: Vec::new(),
        },
    }
}

type CoreCase = (&'static str, Spec, Box<dyn Fn() -> CoreProgram>);

fn run_core_cases(cases: Vec<CoreCase>) {
    for (label, wanted, make) in &cases {
        let applied = run_core(wanted, label, make.as_ref());
        assert_checks(&applied.witness, label);
    }
}

#[test]
fn numeric_modes_const_and_data_uint_to_second() {
    run_core_cases(vec![
        (
            "Add Const UInt",
            spec(
                ScalarMode::Add,
                SiteTag::RetRight,
                Space::Const,
                KindWant::UInt(40),
                PostWant::SecondRegA,
            ),
            Box::new(|| program(bin(BinaryOp::Add, lit_u(40), lit_u(2)))),
        ),
        (
            "Add Data UInt",
            spec(
                ScalarMode::Add,
                SiteTag::RetRight,
                Space::Data,
                KindWant::UInt(42),
                PostWant::SecondRegA,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Add,
                    bin(BinaryOp::Add, lit_u(40), lit_u(2)),
                    lit_u(1),
                ))
            }),
        ),
        (
            "Sub Const UInt",
            spec(
                ScalarMode::Sub,
                SiteTag::RetRight,
                Space::Const,
                KindWant::UInt(50),
                PostWant::SecondRegA,
            ),
            Box::new(|| program(bin(BinaryOp::Sub, lit_u(50), lit_u(8)))),
        ),
        (
            "Sub Data UInt",
            spec(
                ScalarMode::Sub,
                SiteTag::RetRight,
                Space::Data,
                KindWant::UInt(42),
                PostWant::SecondRegA,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Sub,
                    bin(BinaryOp::Sub, lit_u(50), lit_u(8)),
                    lit_u(1),
                ))
            }),
        ),
        (
            "Mul Const UInt",
            spec(
                ScalarMode::Mul,
                SiteTag::RetRight,
                Space::Const,
                KindWant::UInt(6),
                PostWant::SecondRegA,
            ),
            Box::new(|| program(bin(BinaryOp::Mul, lit_u(6), lit_u(7)))),
        ),
        (
            "Mul Data UInt",
            spec(
                ScalarMode::Mul,
                SiteTag::RetRight,
                Space::Data,
                KindWant::UInt(42),
                PostWant::SecondRegA,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Mul,
                    bin(BinaryOp::Mul, lit_u(6), lit_u(7)),
                    lit_u(2),
                ))
            }),
        ),
        (
            "Lt Const UInt",
            spec(
                ScalarMode::Lt,
                SiteTag::RetRight,
                Space::Const,
                KindWant::UInt(2),
                PostWant::SecondRegA,
            ),
            Box::new(|| program(bin(BinaryOp::Lt, lit_u(2), lit_u(3)))),
        ),
        (
            "Lt Data UInt",
            spec(
                ScalarMode::Lt,
                SiteTag::RetRight,
                Space::Data,
                KindWant::UInt(42),
                PostWant::SecondRegA,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Lt,
                    bin(BinaryOp::Add, lit_u(40), lit_u(2)),
                    lit_u(100),
                ))
            }),
        ),
    ]);
}

#[test]
fn cons_and_branch_modes_const_and_data_to_second() {
    run_core_cases(vec![
        (
            "Cons Const UInt",
            spec(
                ScalarMode::Cons,
                SiteTag::RetRight,
                Space::Const,
                KindWant::UInt(7),
                PostWant::SecondRegZero,
            ),
            Box::new(|| program(bin(BinaryOp::Cons, lit_u(7), lit_list(&[3])))),
        ),
        (
            "Cons Data UInt",
            spec(
                ScalarMode::Cons,
                SiteTag::RetRight,
                Space::Data,
                KindWant::UInt(42),
                PostWant::SecondRegZero,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Cons,
                    bin(BinaryOp::Add, lit_u(40), lit_u(2)),
                    lit_list(&[3]),
                ))
            }),
        ),
        (
            "Branch Const Leaf",
            spec(
                ScalarMode::Branch,
                SiteTag::RetRight,
                Space::Const,
                KindWant::Leaf,
                PostWant::SecondRegZero,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Branch,
                    lit_tree(tree_leaf(1)),
                    lit_tree(CoreTree::Branch {
                        left: Box::new(tree_leaf(2)),
                        right: Box::new(tree_leaf(3)),
                    }),
                ))
            }),
        ),
        (
            "Branch Const Branch",
            spec(
                ScalarMode::Branch,
                SiteTag::RetRight,
                Space::Const,
                KindWant::Branch,
                PostWant::SecondRegZero,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Branch,
                    lit_tree(CoreTree::Branch {
                        left: Box::new(tree_leaf(1)),
                        right: Box::new(tree_leaf(2)),
                    }),
                    lit_tree(tree_leaf(3)),
                ))
            }),
        ),
        (
            "Branch Data Branch",
            spec(
                ScalarMode::Branch,
                SiteTag::RetRight,
                Space::Data,
                KindWant::Branch,
                PostWant::SecondRegZero,
            ),
            Box::new(|| {
                program(bin(
                    BinaryOp::Branch,
                    bin(
                        BinaryOp::Branch,
                        lit_tree(tree_leaf(1)),
                        lit_tree(tree_leaf(2)),
                    ),
                    lit_tree(tree_leaf(3)),
                ))
            }),
        ),
    ]);
}

#[test]
fn binary_wrong_kind_modes_done_prim_error_11() {
    run_core_cases(vec![
        (
            "Add Bool wrong kind",
            spec(
                ScalarMode::Add,
                SiteTag::RetRight,
                Space::Const,
                KindWant::Bool(true),
                PostWant::DoneError11,
            ),
            Box::new(|| program(bin(BinaryOp::Add, lit_b(true), lit_u(1)))),
        ),
        (
            "Sub Nil wrong kind",
            spec(
                ScalarMode::Sub,
                SiteTag::RetRight,
                Space::Const,
                KindWant::Nil,
                PostWant::DoneError11,
            ),
            Box::new(|| program(bin(BinaryOp::Sub, lit_list(&[]), lit_u(1)))),
        ),
        (
            "Mul Cons wrong kind",
            spec(
                ScalarMode::Mul,
                SiteTag::RetRight,
                Space::Const,
                KindWant::Cons,
                PostWant::DoneError11,
            ),
            Box::new(|| program(bin(BinaryOp::Mul, lit_list(&[1]), lit_u(1)))),
        ),
        (
            "Lt Leaf wrong kind",
            spec(
                ScalarMode::Lt,
                SiteTag::RetRight,
                Space::Const,
                KindWant::Leaf,
                PostWant::DoneError11,
            ),
            Box::new(|| program(bin(BinaryOp::Lt, lit_tree(tree_leaf(1)), lit_u(1)))),
        ),
        (
            "Cons Bool wrong kind",
            spec(
                ScalarMode::Cons,
                SiteTag::RetRight,
                Space::Const,
                KindWant::Bool(false),
                PostWant::DoneError11,
            ),
            Box::new(|| program(bin(BinaryOp::Cons, lit_b(false), lit_list(&[])))),
        ),
        (
            "Branch UInt wrong kind",
            spec(
                ScalarMode::Branch,
                SiteTag::RetRight,
                Space::Const,
                KindWant::UInt(9),
                PostWant::DoneError11,
            ),
            Box::new(|| program(bin(BinaryOp::Branch, lit_u(9), lit_tree(tree_leaf(1))))),
        ),
    ]);
}

#[test]
fn test_bool_true_false_data_and_non_bool() {
    run_core_cases(vec![
        (
            "TestBool Const true",
            spec(
                ScalarMode::TestBool,
                SiteTag::RetIf,
                Space::Const,
                KindWant::Bool(true),
                PostWant::DoneSelector(BoolSelector::True),
            ),
            Box::new(|| {
                program(CoreExpr::If {
                    cond: Box::new(lit_b(true)),
                    then_expr: Box::new(lit_u(1)),
                    else_expr: Box::new(lit_u(0)),
                })
            }),
        ),
        (
            "TestBool Const false",
            spec(
                ScalarMode::TestBool,
                SiteTag::RetIf,
                Space::Const,
                KindWant::Bool(false),
                PostWant::DoneSelector(BoolSelector::False),
            ),
            Box::new(|| {
                program(CoreExpr::If {
                    cond: Box::new(lit_b(false)),
                    then_expr: Box::new(lit_u(1)),
                    else_expr: Box::new(lit_u(0)),
                })
            }),
        ),
        (
            "TestBool Data true",
            spec(
                ScalarMode::TestBool,
                SiteTag::RetIf,
                Space::Data,
                KindWant::Bool(true),
                PostWant::DoneSelector(BoolSelector::True),
            ),
            Box::new(|| {
                program(CoreExpr::If {
                    cond: Box::new(bin(BinaryOp::Lt, lit_u(2), lit_u(3))),
                    then_expr: Box::new(lit_u(1)),
                    else_expr: Box::new(lit_u(0)),
                })
            }),
        ),
        (
            "TestBool Const non-Bool",
            spec(
                ScalarMode::TestBool,
                SiteTag::RetIf,
                Space::Const,
                KindWant::UInt(7),
                PostWant::DoneSelector(BoolSelector::NotBool),
            ),
            Box::new(|| {
                program(CoreExpr::If {
                    cond: Box::new(lit_u(7)),
                    then_expr: Box::new(lit_u(1)),
                    else_expr: Box::new(lit_u(0)),
                })
            }),
        ),
    ]);
}

#[test]
fn test_type_builtin_true_and_false() {
    let cases: Vec<CoreCase> = vec![
        (
            "TestType UInt true",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::UInt(42),
                PostWant::DoneTypeTest(true),
            ),
            Box::new(|| call_program(SortTag::UInt, lit_u(42))),
        ),
        (
            "TestType UInt false",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::Bool(true),
                PostWant::DoneTypeTest(false),
            ),
            Box::new(|| call_program(SortTag::UInt, lit_b(true))),
        ),
        (
            "TestType Bool true",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::Bool(true),
                PostWant::DoneTypeTest(true),
            ),
            Box::new(|| call_program(SortTag::Bool, lit_b(true))),
        ),
        (
            "TestType Bool false",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::UInt(1),
                PostWant::DoneTypeTest(false),
            ),
            Box::new(|| call_program(SortTag::Bool, lit_u(1))),
        ),
        (
            "TestType List true",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::Cons,
                PostWant::DoneTypeTest(true),
            ),
            Box::new(|| call_program(SortTag::List, lit_list(&[1, 2]))),
        ),
        (
            "TestType List false",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::UInt(1),
                PostWant::DoneTypeTest(false),
            ),
            Box::new(|| call_program(SortTag::List, lit_u(1))),
        ),
        (
            "TestType Tree true",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::Leaf,
                PostWant::DoneTypeTest(true),
            ),
            Box::new(|| call_program(SortTag::Tree, lit_tree(tree_leaf(1)))),
        ),
        (
            "TestType Tree false",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Const,
                KindWant::Bool(false),
                PostWant::DoneTypeTest(false),
            ),
            Box::new(|| call_program(SortTag::Tree, lit_b(false))),
        ),
        (
            "TestType Data UInt true",
            spec(
                ScalarMode::TestType,
                SiteTag::RetCall,
                Space::Data,
                KindWant::UInt(42),
                PostWant::DoneTypeTest(true),
            ),
            Box::new(|| call_program(SortTag::UInt, bin(BinaryOp::Add, lit_u(40), lit_u(2)))),
        ),
    ];
    for (label, wanted, make) in &cases {
        let applied = run_core(wanted, label, make.as_ref());
        assert!(
            applied.record.before.admission.view.types.is_empty(),
            "{label}: dedicated Core admission has no user type rows; builtin sort domain is distinct"
        );
        assert_checks(&applied.witness, label);
    }
}

fn nominal_program(return_ctor_is_alpha: bool) -> ExtProgram {
    let (body_ctor, body_type) = if return_ctor_is_alpha {
        ("MkAlpha", "Alpha")
    } else {
        ("MkBeta", "Beta")
    };
    ExtProgram {
        types: vec![
            TypeDeclSrc {
                name: "Alpha".into(),
                parameters: Vec::new(),
                ctors: vec![CtorDeclSrc {
                    name: "MkAlpha".into(),
                    fields: Vec::new(),
                }],
            },
            TypeDeclSrc {
                name: "Beta".into(),
                parameters: Vec::new(),
                ctors: vec![CtorDeclSrc {
                    name: "MkBeta".into(),
                    fields: Vec::new(),
                }],
            },
        ],
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: vec![ExtFunction {
            name: "f".into(),
            parameters: Vec::new(),
            return_type: TypeExprSrc::Named {
                name: "Alpha".into(),
            },
            body: ExtExpr::Literal {
                value: ExtValue::Constructor {
                    ty: TypeExprSrc::Named {
                        name: body_type.into(),
                    },
                    ctor: body_ctor.into(),
                    fields: Vec::new(),
                },
            },
        }],
        entry: ExtExpr::Call {
            name: "f".into(),
            args: Vec::new(),
        },
    }
}

#[test]
fn test_type_nominal_constructor_true_and_false() {
    let true_wanted = spec(
        ScalarMode::TestType,
        SiteTag::RetCall,
        Space::Const,
        KindWant::Ctor {
            type_id: 4,
            ctor_id: 4,
        },
        PostWant::DoneTypeTest(true),
    );
    let applied = run_ext(&true_wanted, "TestType nominal Alpha true", &|| {
        nominal_program(true)
    });
    assert_eq!(applied.record.before.admission.view.types.len(), 6);
    assert_eq!(applied.record.before.admission.view.constructors.len(), 6);
    let helper = selected_helper(&applied.record.before);
    let HelperControlC::Scalar { expected, .. } = helper else {
        unreachable!()
    };
    assert_eq!(expected, Some(TypeIdC(4)), "nominal expected TypeId");
    assert_checks(&applied.witness, "TestType nominal Alpha true");

    let false_wanted = spec(
        ScalarMode::TestType,
        SiteTag::RetCall,
        Space::Const,
        KindWant::Ctor {
            type_id: 5,
            ctor_id: 5,
        },
        PostWant::DoneTypeTest(false),
    );
    let applied = run_ext(&false_wanted, "TestType nominal Beta false", &|| {
        nominal_program(false)
    });
    let helper = selected_helper(&applied.record.before);
    let HelperControlC::Scalar { expected, .. } = helper else {
        unreachable!()
    };
    assert_eq!(expected, Some(TypeIdC(4)), "nominal expected remains Alpha");
    assert_checks(&applied.witness, "TestType nominal Beta false");
}

fn assert_p3(record: &CompleteStepRecord, wanted: &Spec, label: &str) {
    assert_selected_pre(record, wanted, label);
    assert_preserved(record, label);
    let pre = &record.before.image;
    let post = &record.after.image;
    let event = &record.event;
    assert_eq!(
        pre.helper_work,
        record.before.admission.bounds.max_helper_work
    );
    assert_eq!(event.rule_id, RuleIdC::BHelperScalarTag, "{label}: rule");
    assert_eq!(
        event.event_kind,
        EventKind::HelperVisit,
        "{label}: event kind"
    );
    assert_eq!(
        event.helper_kind,
        Some(HelperKind::Scalar),
        "{label}: helper kind"
    );
    assert_eq!(
        event.disposition,
        StepDisposition::RefusedHelperWorkUncharged,
        "{label}: disposition"
    );
    assert_eq!(event.work_delta, 0, "{label}: work delta");
    assert_eq!(event.helper_work_delta, 0, "{label}: helper work delta");
    assert_eq!(event.helper_visits_delta, 0, "{label}: visits delta");
    assert_eq!(event.reference_cursor_delta, 0, "{label}: cursor delta");
    assert_eq!(
        event.exhausted_kind,
        Some(ExhaustReason::HelperWork),
        "{label}: exhausted kind"
    );
    assert_eq!(
        event.outcome_tag,
        OutcomeTagC::Exhausted,
        "{label}: outcome tag"
    );
    assert_eq!(
        event.outcome_payload,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperWork
        },
        "{label}: outcome payload"
    );
    assert_eq!(event.boundary, BoundaryKind::Terminal, "{label}: boundary");
    assert_eq!(
        post.remaining_work, pre.remaining_work,
        "{label}: remaining preserved"
    );
    assert_eq!(
        post.consumed_work, pre.consumed_work,
        "{label}: consumed preserved"
    );
    assert_eq!(
        post.helper_work, pre.helper_work,
        "{label}: helper work preserved"
    );
    assert_eq!(
        post.helper_visits, pre.helper_visits,
        "{label}: visits preserved"
    );
    assert_eq!(post.helper, pre.helper, "{label}: helper preserved");
    assert_eq!(
        post.outcome,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperWork
        },
        "{label}: post outcome"
    );
    assert_eq!(
        event.helper_phase_after,
        Some(HelperPhaseSummaryC::Scalar(ScalarPhase::First)),
        "{label}: helper phase after"
    );
    assert_eq!(
        event.helper_cursor_after,
        Some(0),
        "{label}: helper cursor after"
    );
}

#[test]
fn p3_exact_fresh_helper_work_bound_uncharged() {
    let label = "P3 Add Const UInt";
    let wanted = spec(
        ScalarMode::Add,
        SiteTag::RetRight,
        Space::Const,
        KindWant::UInt(40),
        PostWant::SecondRegA,
    );
    let make = || program(bin(BinaryOp::Add, lit_u(40), lit_u(2)));
    let (_, _, generous_pre) = reach_core(generous(), &wanted, label, &make);
    let helper_work = generous_pre.image.helper_work;
    assert!(
        helper_work > 0,
        "{label}: selected helper work must be positive"
    );
    let exact = bounds(helper_work);
    let (admitted, mut image, pre) = reach_core(exact, &wanted, label, &make);
    assert_eq!(pre.image.helper_work, exact.max_helper_work);
    let record = replay_core(exact, &wanted, label, &make);
    assert_eq!(
        record.before, pre,
        "{label}: replay reaches identical P3 pre-state"
    );
    assert_p3(&record, &wanted, label);
    let config = core_cfg(exact);
    let witness = record_scalar_tag_step(&admitted, &mut image, &config, &limits())
        .unwrap_or_else(|e| panic!("{label}: selected P3 wrapper step: {e}"));
    let original_after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture original P3 post: {}", violation_label(&e)));
    assert_eq!(
        original_after, record.after,
        "{label}: wrapper post equals replay post"
    );
    assert!(
        witness.disposition_is_refused_helper_work(),
        "{label}: witness P3 disposition"
    );
    assert_eq!(
        witness.source_params().admission_bounds(),
        record.before.admission.bounds,
        "{label}: witness retains original admission bounds"
    );
    assert_checks(&witness, label);
    let _: C6ScalarTagConsumerError = C6ScalarTagConsumerError::UnexpectedOwnerStep;
}
