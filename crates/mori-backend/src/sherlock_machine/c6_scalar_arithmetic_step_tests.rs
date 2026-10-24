//! Genuine-owner positive tests for `c6_scalar_arithmetic_consumer`.
//!
//! Source-only and UNRUN. Every fixture is admitted through the real
//! public owner (`host_owner_engine`) and progressed with the real
//! `record_step` until the actual selected ScalarArithmetic pre-state:
//! `(First, Not)` with `reg_a == 0`, or `(Second, Add|Sub|Mul|Lt)` with
//! `reg_a` equal to the actual UInt operand-A payload. The selected step
//! is taken once by `record_scalar_arithmetic_step` on the original
//! image and once by raw `record_step` on an independently replayed
//! image. Complete captures and the event are compared. Both
//! `LocalPublic` and `LedgerCall` are checked with the actual
//! `build_c6_scalar_arithmetic_step_circuit` /
//! `check_c6_scalar_arithmetic_step_circuit` (`ProofPreimage::check`
//! preprocessing only) in every selected logical case.
//!
//! Retained premises are NOT discharged here: SI08/SI21 entry/top-frame
//! coherence, SI17 marker placement, full SI21/SI22/StepInv, program
//! membership, P1 issuer and SourceSpec authentication remain caller
//! premises. No fake admitted state or synthetic helper is constructed.
//! Expected staged values below are independently stated mathematical
//! results (checked `u128` arithmetic, codes 10/11 as in Core), not
//! copied from emitter/post receipts.
//!
//! DELIVERED_SOURCE (this file, bounded delivery ONE of TWO):
//! five modes; Add/Sub/Mul/Lt/Not branch cases incl. overflow and
//! underflow; wrong-kind 11; Const and DATA operand A and B including
//! genuine nested arithmetic producing DATA; applied P4/P7 counters;
//! genuine P3 for Not and a numeric mode; owner-record cross-field
//! negatives; one independent wrong-post host-pair checker negative;
//! wrapper non-rollback on a wrong-rule step.
//! REQUIRED_OPEN (separate mandatory raw delivery, NOT dropped):
//! raw Fr/gate/Ref/padding/index/namespace/overflow suite and full
//! wrong-post isolation, both modes. This leaf is not done.

use super::c6_push_step::C6PushStatementBindingMode;
use super::{
    C6ScalarArithmeticConsumerError, C6ScalarArithmeticStepWitness,
    build_c6_scalar_arithmetic_step_circuit, check_c6_scalar_arithmetic_step_circuit,
    record_scalar_arithmetic_step, witness_scalar_arithmetic_from_host_pair,
    witness_scalar_arithmetic_from_owner_record,
};
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits};
use mori_core::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, ConstantSnapshot, DataSnapshot, HostSnapshot,
    ObservedStep, capture, record_step as owner_record,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;

// ---------- error labels: no Debug bound on CaptureStepError/Tooling --

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

// ---------- retained host configuration ----------

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

fn layout(b: BoundsC) -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: capacities(),
        ext_capacities: None,
        boundary_capacities_present: false,
    }
}

fn cfg(b: BoundsC) -> C6HostConfig {
    C6HostConfig::from_retained_layout(
        &layout(b),
        C6CodecBudgets {
            type_expr_nodes: 256,
            template_bytes: 4096,
        },
    )
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

// ---------- selected-state description ----------

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
}

#[derive(Clone, Copy, Debug)]
struct Want {
    mode: ScalarMode,
    a_space: Space,
    a_kind: KindWant,
    b_space: Option<Space>,
    b_kind: Option<KindWant>,
}

fn numeric(mode: ScalarMode, a_space: Space, a: u128, b_space: Space, b_kind: KindWant) -> Want {
    Want {
        mode,
        a_space,
        a_kind: KindWant::UInt(a),
        b_space: Some(b_space),
        b_kind: Some(b_kind),
    }
}

fn not_want(a_space: Space, a_kind: KindWant) -> Want {
    Want {
        mode: ScalarMode::Not,
        a_space,
        a_kind,
        b_space: None,
        b_kind: None,
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
            Some(ConstantSnapshot::Constructor { .. }) => {
                panic!("Constructor operand is outside this builtin delivery")
            }
            Some(ConstantSnapshot::U128OutOfRange { .. }) => {
                panic!("raw Marker operand belongs to the separate raw delivery")
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
            Some(DataSnapshot::Constructor { .. }) => {
                panic!("Constructor operand is outside this builtin delivery")
            }
            None => panic!("selected Data operand index must exist"),
        },
    }
}

fn space_of(reference: RefC) -> Space {
    match reference {
        RefC::Const(_) => Space::Const,
        RefC::Data(_) => Space::Data,
    }
}

fn selected_helper(snapshot: &HostSnapshot) -> HelperControlC {
    match snapshot.image.helper {
        Some(h @ HelperControlC::Scalar { .. }) => h,
        _ => panic!("selected snapshot must carry a Scalar helper"),
    }
}

fn is_selected(snapshot: &HostSnapshot, wanted: &Want) -> bool {
    let helper = match snapshot.image.helper {
        Some(h) => h,
        None => return false,
    };
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
        return false;
    };
    if mode != wanted.mode || staged != ScalarStaged::None || expected.is_some() {
        return false;
    }
    if common.parent_resume.is_some()
        || common.origin_reference_cursor != snapshot.image.reference_cursor
    {
        return false;
    }
    let is_not = wanted.mode == ScalarMode::Not;
    let (want_phase, want_cursor_site) = if is_not {
        (ScalarPhase::First, SiteTag::RetUnary)
    } else {
        (ScalarPhase::Second, SiteTag::RetRight)
    };
    if phase != want_phase || common.site != want_cursor_site {
        return false;
    }
    if is_not {
        if reg_a != 0 || operand_b.is_some() {
            return false;
        }
    } else {
        // Valid SI22 numeric domain: reg_a equals actual operand-A UInt.
        let KindWant::UInt(a) = wanted.a_kind else {
            return false;
        };
        if reg_a != a || operand_b.is_none() {
            return false;
        }
    }
    if space_of(operand_a) != wanted.a_space || operand_kind(snapshot, operand_a) != wanted.a_kind {
        return false;
    }
    match (wanted.b_space, wanted.b_kind, operand_b) {
        (None, None, None) => true,
        (Some(space), Some(kind), Some(b_ref)) => {
            space_of(b_ref) == space && operand_kind(snapshot, b_ref) == kind
        }
        _ => false,
    }
}

// ---------- genuine owner reach / replay ----------

fn search(
    admitted: Admitted,
    mut image: MachineImage,
    wanted: &Want,
    label: &str,
) -> (Admitted, MachineImage, HostSnapshot) {
    for _ in 0..512 {
        let step = owner_record(&admitted, &mut image)
            .unwrap_or_else(|e| panic!("{label}: search step: {}", capture_label(&e)));
        let after = match &step {
            ObservedStep::Event(record) => &record.after,
            ObservedStep::Terminal { after, .. } => panic!(
                "{label}: terminal before selected state: {:?}",
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
                "{label}: capture equals searched pre-state"
            );
            return (admitted, image, selected);
        }
    }
    panic!("{label}: genuine owner did not reach the selected ScalarArithmetic state")
}

fn reach(
    b: BoundsC,
    wanted: &Want,
    label: &str,
    make: &dyn Fn() -> CoreProgram,
) -> (Admitted, MachineImage, HostSnapshot) {
    let program = make();
    let engine = host_owner_engine();
    let admitted = engine
        .admit(&program, &b, &layout(b))
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

fn replay(
    b: BoundsC,
    wanted: &Want,
    label: &str,
    make: &dyn Fn() -> CoreProgram,
) -> CompleteStepRecord {
    let (admitted, mut image, _) = reach(b, wanted, label, make);
    match owner_record(&admitted, &mut image) {
        Ok(ObservedStep::Event(record)) => record,
        Ok(ObservedStep::Terminal { .. }) => {
            panic!("{label}: selected raw step must be an event")
        }
        Err(e) => panic!("{label}: selected raw step: {}", capture_label(&e)),
    }
}

// ---------- complete record assertions ----------

fn assert_selected_pre(record: &CompleteStepRecord, wanted: &Want, label: &str) {
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
    assert_eq!(
        phase,
        if wanted.mode == ScalarMode::Not {
            ScalarPhase::First
        } else {
            ScalarPhase::Second
        },
        "{label}: phase First0Not vs Second1numeric"
    );
    assert_eq!(
        reg_a,
        if wanted.mode == ScalarMode::Not {
            0
        } else {
            match wanted.a_kind {
                KindWant::UInt(n) => n,
                _ => panic!("{label}: numeric Want requires UInt A"),
            }
        },
        "{label}: reg_a (0 for Not, operand-A payload for numeric)"
    );
    assert_eq!(staged, ScalarStaged::None, "{label}: pre staged");
    assert!(common.parent_resume.is_none(), "{label}: parent_resume");
    assert_eq!(
        common.origin_reference_cursor, pre.reference_cursor,
        "{label}: origin cursor"
    );
    assert_eq!(
        operand_b.is_some(),
        wanted.mode != ScalarMode::Not,
        "{label}: operand_b presence iff numeric"
    );
    assert!(expected.is_none(), "{label}: expected absent");
    assert_eq!(
        operand_kind(&record.before, operand_a),
        wanted.a_kind,
        "{label}: operand A kind/value"
    );
    assert_eq!(
        space_of(operand_a),
        wanted.a_space,
        "{label}: operand A Ref space"
    );
    if let (Some(b_ref), Some(kind), Some(space)) = (operand_b, wanted.b_kind, wanted.b_space) {
        assert_eq!(
            operand_kind(&record.before, b_ref),
            kind,
            "{label}: operand B kind/value"
        );
        assert_eq!(space_of(b_ref), space, "{label}: operand B Ref space");
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
    assert_eq!(
        after.admission, before.admission,
        "{label}: admission/source params"
    );
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
        "{label}: reference cursor unchanged (checked P4/P7 lane)"
    );
}

fn assert_applied(record: &CompleteStepRecord, wanted: &Want, staged: ScalarStaged, label: &str) {
    assert_selected_pre(record, wanted, label);
    assert_preserved(record, label);
    let pre = &record.before.image;
    let post = &record.after.image;
    let event = &record.event;
    let is_not = wanted.mode == ScalarMode::Not;
    assert_eq!(
        event.rule_id,
        RuleIdC::BHelperScalarArithmetic,
        "{label}: rule"
    );
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
    assert_eq!(
        event.site,
        Some(if is_not {
            SiteTag::RetUnary
        } else {
            SiteTag::RetRight
        }),
        "{label}: event site"
    );
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
        Some(HelperPhaseSummaryC::Scalar(if is_not {
            ScalarPhase::First
        } else {
            ScalarPhase::Second
        })),
        "{label}: helper phase before"
    );
    assert_eq!(
        event.helper_cursor_before,
        Some(if is_not { 0 } else { 1 }),
        "{label}: helper cursor before"
    );
    assert_eq!(
        post.remaining_work,
        pre.remaining_work - 1,
        "{label}: remaining -1"
    );
    assert_eq!(
        post.consumed_work,
        pre.consumed_work + 1,
        "{label}: consumed +1"
    );
    assert_eq!(
        post.helper_work,
        pre.helper_work + 1,
        "{label}: helper work +1"
    );
    assert_eq!(
        post.helper_visits,
        pre.helper_visits + 1,
        "{label}: helper visits +1"
    );
    assert_eq!(post.outcome, OutcomeC::Running, "{label}: post outcome");

    // Complete expected staged result, independently stated by caller.
    let reg_a = if is_not {
        0
    } else {
        match wanted.a_kind {
            KindWant::UInt(n) => n,
            _ => unreachable!(),
        }
    };
    let mut expected_helper = selected_helper(&record.before);
    if let HelperControlC::Scalar {
        phase: p,
        reg_a: ra,
        staged: s,
        ..
    } = &mut expected_helper
    {
        *p = ScalarPhase::Done;
        *ra = reg_a;
        *s = staged;
    }
    assert_eq!(
        post.helper,
        Some(expected_helper),
        "{label}: complete post helper (helpers retained, Done)"
    );
    assert_eq!(
        event.helper_phase_after,
        Some(HelperPhaseSummaryC::Scalar(ScalarPhase::Done)),
        "{label}: appliedDone2 phase after"
    );
    assert_eq!(
        event.helper_cursor_after,
        Some(2),
        "{label}: appliedDone2 cursor after"
    );
}

fn assert_checks(witness: &C6ScalarArithmeticStepWitness, label: &str) {
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let circuit = build_c6_scalar_arithmetic_step_circuit(
            witness.source_params().clone(),
            witness.host_prefix().meta.len(),
            witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset,
            mode,
            "scalar_arith_owner_test",
        )
        .unwrap_or_else(|e| panic!("{label}: circuit builds in {mode:?}: {e}"));
        // Geometric derivation comes from the actual circuit, never a
        // hard-coded fixture offset.
        assert_eq!(
            circuit.geometry(),
            witness.source_params().geometry(),
            "{label}: circuit geometry matches retained params in {mode:?}"
        );
        assert_eq!(
            circuit.meta_len(),
            witness.host_prefix().meta.len(),
            "{label}: meta len in {mode:?}"
        );
        assert_eq!(
            circuit.prog_len(),
            witness.host_prefix().prog.len(),
            "{label}: prog len in {mode:?}"
        );
        check_c6_scalar_arithmetic_step_circuit(&circuit, witness).unwrap_or_else(|e| {
            panic!("{label}: SDK ProofPreimage::check must pass in {mode:?}: {e}")
        });
    }
}

fn finish_applied(
    record: CompleteStepRecord,
    wanted: &Want,
    staged: ScalarStaged,
    label: &str,
    admitted: &Admitted,
    image: &mut MachineImage,
    config: &C6HostConfig,
) -> C6ScalarArithmeticStepWitness {
    let witness = record_scalar_arithmetic_step(admitted, image, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: selected wrapper step: {e}"));
    let original_after = capture(admitted, image).unwrap_or_else(|e| {
        panic!(
            "{label}: capture original wrapper post: {}",
            violation_label(&e)
        )
    });
    assert_eq!(
        original_after, record.after,
        "{label}: wrapper/post == replay/post (complete capture)"
    );
    assert_eq!(
        original_after.image.helper, record.after.image.helper,
        "{label}: wrapper helper == replay helper"
    );
    assert_applied(&record, wanted, staged, label);
    assert!(witness.enable(), "{label}: witness enable");
    assert!(
        !witness.disposition_is_refused_helper_work(),
        "{label}: applied disposition flag"
    );
    assert_eq!(
        witness.source_params().admission_bounds(),
        record.before.admission.bounds,
        "{label}: original source-params admission bounds retained"
    );
    let host_pair = witness_scalar_arithmetic_from_host_pair(
        true,
        &record.before,
        &record.after,
        config,
        &limits(),
    )
    .unwrap_or_else(|e| panic!("{label}: host-pair witness: {e}"));
    assert_eq!(
        host_pair, witness,
        "{label}: host-pair equals wrapper witness"
    );
    let owner_witness = witness_scalar_arithmetic_from_owner_record(&record, config, &limits())
        .unwrap_or_else(|e| panic!("{label}: owner-record witness: {e}"));
    assert_eq!(
        owner_witness, witness,
        "{label}: owner-record equals wrapper witness"
    );
    assert_eq!(
        host_pair.pre_image(),
        witness.pre_image(),
        "{label}: pre image lanes"
    );
    assert_eq!(
        host_pair.post_helper(),
        witness.post_helper(),
        "{label}: post helper lanes"
    );
    witness
}

fn run_case(
    wanted: &Want,
    staged: ScalarStaged,
    label: &str,
    make: &dyn Fn() -> CoreProgram,
) -> C6ScalarArithmeticStepWitness {
    let b = generous();
    let (admitted, mut image, pre) = reach(b, wanted, label, make);
    let record = replay(b, wanted, label, make);
    assert_eq!(
        record.before, pre,
        "{label}: replay reaches identical selected pre-state"
    );
    let witness = finish_applied(
        record,
        wanted,
        staged,
        label,
        &admitted,
        &mut image,
        &cfg(b),
    );
    assert_checks(&witness, label);
    witness
}

// ---------- Core program builders ----------

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

fn not_expr(operand: CoreExpr) -> CoreExpr {
    CoreExpr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(operand),
    }
}

fn program(entry: CoreExpr) -> CoreProgram {
    CoreProgram {
        functions: Vec::new(),
        entry,
    }
}

fn u(n: u128) -> ScalarStaged {
    ScalarStaged::U128Value { n }
}

fn bv(b: bool) -> ScalarStaged {
    ScalarStaged::BoolValue { b }
}

fn err(code: u8) -> ScalarStaged {
    ScalarStaged::PrimError { code }
}

type Case = (
    &'static str,
    Want,
    ScalarStaged,
    Box<dyn Fn() -> CoreProgram>,
);

fn run_cases(cases: Vec<Case>) {
    for (label, wanted, staged, make) in &cases {
        run_case(wanted, *staged, label, make.as_ref());
    }
}

// ---------- five-mode branch coverage ----------

#[test]
fn add_ordinary_zero_max_and_overflow() {
    run_cases(vec![
        (
            "Add ordinary Const/Const 7+3",
            numeric(
                ScalarMode::Add,
                Space::Const,
                7,
                Space::Const,
                KindWant::UInt(3),
            ),
            u(10),
            Box::new(|| program(bin(BinaryOp::Add, lit_u(7), lit_u(3)))),
        ),
        (
            "Add zero 0+0",
            numeric(
                ScalarMode::Add,
                Space::Const,
                0,
                Space::Const,
                KindWant::UInt(0),
            ),
            u(0),
            Box::new(|| program(bin(BinaryOp::Add, lit_u(0), lit_u(0)))),
        ),
        (
            "Add max success max+0",
            numeric(
                ScalarMode::Add,
                Space::Const,
                u128::MAX,
                Space::Const,
                KindWant::UInt(0),
            ),
            u(u128::MAX),
            Box::new(|| program(bin(BinaryOp::Add, lit_u(u128::MAX), lit_u(0)))),
        ),
        (
            "Add overflow max+1 error10",
            numeric(
                ScalarMode::Add,
                Space::Const,
                u128::MAX,
                Space::Const,
                KindWant::UInt(1),
            ),
            err(10),
            Box::new(|| program(bin(BinaryOp::Add, lit_u(u128::MAX), lit_u(1)))),
        ),
        (
            "Add Data A (40+2)+1",
            numeric(
                ScalarMode::Add,
                Space::Data,
                42,
                Space::Const,
                KindWant::UInt(1),
            ),
            u(43),
            Box::new(|| {
                program(bin(
                    BinaryOp::Add,
                    bin(BinaryOp::Add, lit_u(40), lit_u(2)),
                    lit_u(1),
                ))
            }),
        ),
        (
            "Add Data B 7+(2+1)",
            numeric(
                ScalarMode::Add,
                Space::Const,
                7,
                Space::Data,
                KindWant::UInt(3),
            ),
            u(10),
            Box::new(|| {
                program(bin(
                    BinaryOp::Add,
                    lit_u(7),
                    bin(BinaryOp::Add, lit_u(2), lit_u(1)),
                ))
            }),
        ),
    ]);
}

#[test]
fn sub_ordinary_equal_zero_and_underflow() {
    run_cases(vec![
        (
            "Sub ordinary 7-3",
            numeric(
                ScalarMode::Sub,
                Space::Const,
                7,
                Space::Const,
                KindWant::UInt(3),
            ),
            u(4),
            Box::new(|| program(bin(BinaryOp::Sub, lit_u(7), lit_u(3)))),
        ),
        (
            "Sub equal max-max=0",
            numeric(
                ScalarMode::Sub,
                Space::Const,
                u128::MAX,
                Space::Const,
                KindWant::UInt(u128::MAX),
            ),
            u(0),
            Box::new(|| program(bin(BinaryOp::Sub, lit_u(u128::MAX), lit_u(u128::MAX)))),
        ),
        (
            "Sub zero 0-0",
            numeric(
                ScalarMode::Sub,
                Space::Const,
                0,
                Space::Const,
                KindWant::UInt(0),
            ),
            u(0),
            Box::new(|| program(bin(BinaryOp::Sub, lit_u(0), lit_u(0)))),
        ),
        (
            "Sub underflow 0-1 error10",
            numeric(
                ScalarMode::Sub,
                Space::Const,
                0,
                Space::Const,
                KindWant::UInt(1),
            ),
            err(10),
            Box::new(|| program(bin(BinaryOp::Sub, lit_u(0), lit_u(1)))),
        ),
        (
            "Sub Data A (50-8)-1",
            numeric(
                ScalarMode::Sub,
                Space::Data,
                42,
                Space::Const,
                KindWant::UInt(1),
            ),
            u(41),
            Box::new(|| {
                program(bin(
                    BinaryOp::Sub,
                    bin(BinaryOp::Sub, lit_u(50), lit_u(8)),
                    lit_u(1),
                ))
            }),
        ),
        (
            "Sub Data B 50-(5+3)",
            numeric(
                ScalarMode::Sub,
                Space::Const,
                50,
                Space::Data,
                KindWant::UInt(8),
            ),
            u(42),
            Box::new(|| {
                program(bin(
                    BinaryOp::Sub,
                    lit_u(50),
                    bin(BinaryOp::Add, lit_u(5), lit_u(3)),
                ))
            }),
        ),
    ]);
}

#[test]
fn mul_ordinary_zero_max_and_high_half_overflow() {
    run_cases(vec![
        (
            "Mul ordinary 7*3",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                7,
                Space::Const,
                KindWant::UInt(3),
            ),
            u(21),
            Box::new(|| program(bin(BinaryOp::Mul, lit_u(7), lit_u(3)))),
        ),
        (
            "Mul zero max*0",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                u128::MAX,
                Space::Const,
                KindWant::UInt(0),
            ),
            u(0),
            Box::new(|| program(bin(BinaryOp::Mul, lit_u(u128::MAX), lit_u(0)))),
        ),
        (
            "Mul max*1 success",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                u128::MAX,
                Space::Const,
                KindWant::UInt(1),
            ),
            u(u128::MAX),
            Box::new(|| program(bin(BinaryOp::Mul, lit_u(u128::MAX), lit_u(1)))),
        ),
        (
            "Mul 2^64*2^64 high-half error10",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                1u128 << 64,
                Space::Const,
                KindWant::UInt(1u128 << 64),
            ),
            err(10),
            Box::new(|| program(bin(BinaryOp::Mul, lit_u(1u128 << 64), lit_u(1u128 << 64)))),
        ),
        (
            "Mul max*max high-half error10",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                u128::MAX,
                Space::Const,
                KindWant::UInt(u128::MAX),
            ),
            err(10),
            Box::new(|| program(bin(BinaryOp::Mul, lit_u(u128::MAX), lit_u(u128::MAX)))),
        ),
        (
            "Mul Data A (6*7)*2",
            numeric(
                ScalarMode::Mul,
                Space::Data,
                42,
                Space::Const,
                KindWant::UInt(2),
            ),
            u(84),
            Box::new(|| {
                program(bin(
                    BinaryOp::Mul,
                    bin(BinaryOp::Mul, lit_u(6), lit_u(7)),
                    lit_u(2),
                ))
            }),
        ),
        (
            "Mul Data B 6*(3+4)",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                6,
                Space::Data,
                KindWant::UInt(7),
            ),
            u(42),
            Box::new(|| {
                program(bin(
                    BinaryOp::Mul,
                    lit_u(6),
                    bin(BinaryOp::Add, lit_u(3), lit_u(4)),
                ))
            }),
        ),
    ]);
}

#[test]
fn lt_less_equal_greater() {
    run_cases(vec![
        (
            "Lt less 0<max",
            numeric(
                ScalarMode::Lt,
                Space::Const,
                0,
                Space::Const,
                KindWant::UInt(u128::MAX),
            ),
            bv(true),
            Box::new(|| program(bin(BinaryOp::Lt, lit_u(0), lit_u(u128::MAX)))),
        ),
        (
            "Lt equal 7<7 false",
            numeric(
                ScalarMode::Lt,
                Space::Const,
                7,
                Space::Const,
                KindWant::UInt(7),
            ),
            bv(false),
            Box::new(|| program(bin(BinaryOp::Lt, lit_u(7), lit_u(7)))),
        ),
        (
            "Lt greater max<0 false",
            numeric(
                ScalarMode::Lt,
                Space::Const,
                u128::MAX,
                Space::Const,
                KindWant::UInt(0),
            ),
            bv(false),
            Box::new(|| program(bin(BinaryOp::Lt, lit_u(u128::MAX), lit_u(0)))),
        ),
        (
            "Lt Data A (40+2)<100",
            numeric(
                ScalarMode::Lt,
                Space::Data,
                42,
                Space::Const,
                KindWant::UInt(100),
            ),
            bv(true),
            Box::new(|| {
                program(bin(
                    BinaryOp::Lt,
                    bin(BinaryOp::Add, lit_u(40), lit_u(2)),
                    lit_u(100),
                ))
            }),
        ),
        (
            "Lt Data B 7<(2+3) false",
            numeric(
                ScalarMode::Lt,
                Space::Const,
                7,
                Space::Data,
                KindWant::UInt(5),
            ),
            bv(false),
            Box::new(|| {
                program(bin(
                    BinaryOp::Lt,
                    lit_u(7),
                    bin(BinaryOp::Add, lit_u(2), lit_u(3)),
                ))
            }),
        ),
    ]);
}

#[test]
fn not_true_false_const_and_data() {
    run_cases(vec![
        (
            "Not true Const",
            not_want(Space::Const, KindWant::Bool(true)),
            bv(false),
            Box::new(|| program(not_expr(lit_b(true)))),
        ),
        (
            "Not false Const",
            not_want(Space::Const, KindWant::Bool(false)),
            bv(true),
            Box::new(|| program(not_expr(lit_b(false)))),
        ),
        (
            "Not Data Bool from Lt (2<3)=true",
            not_want(Space::Data, KindWant::Bool(true)),
            bv(false),
            Box::new(|| program(not_expr(bin(BinaryOp::Lt, lit_u(2), lit_u(3))))),
        ),
        (
            "Not Data Bool from Lt (3<2)=false",
            not_want(Space::Data, KindWant::Bool(false)),
            bv(true),
            Box::new(|| program(not_expr(bin(BinaryOp::Lt, lit_u(3), lit_u(2))))),
        ),
    ]);
}

#[test]
fn wrong_kind_error11_every_mode() {
    run_cases(vec![
        // Numeric modes: operand B is the checked wrong kind; A is UInt.
        (
            "Add wrong-kind B Bool error11",
            numeric(
                ScalarMode::Add,
                Space::Const,
                7,
                Space::Const,
                KindWant::Bool(true),
            ),
            err(11),
            Box::new(|| program(bin(BinaryOp::Add, lit_u(7), lit_b(true)))),
        ),
        (
            "Sub wrong-kind B List/Nil error11",
            numeric(
                ScalarMode::Sub,
                Space::Const,
                7,
                Space::Const,
                KindWant::Nil,
            ),
            err(11),
            Box::new(|| program(bin(BinaryOp::Sub, lit_u(7), lit_list(&[])))),
        ),
        (
            "Sub wrong-kind B Cons error11",
            numeric(
                ScalarMode::Sub,
                Space::Const,
                7,
                Space::Const,
                KindWant::Cons,
            ),
            err(11),
            Box::new(|| program(bin(BinaryOp::Sub, lit_u(7), lit_list(&[1])))),
        ),
        (
            "Mul wrong-kind B Leaf/Tree error11",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                7,
                Space::Const,
                KindWant::Leaf,
            ),
            err(11),
            Box::new(|| program(bin(BinaryOp::Mul, lit_u(7), lit_tree(tree_leaf(1))))),
        ),
        (
            "Mul wrong-kind B Branch/Tree error11",
            numeric(
                ScalarMode::Mul,
                Space::Const,
                7,
                Space::Const,
                KindWant::Branch,
            ),
            err(11),
            Box::new(|| {
                program(bin(
                    BinaryOp::Mul,
                    lit_u(7),
                    lit_tree(CoreTree::Branch {
                        left: Box::new(tree_leaf(1)),
                        right: Box::new(tree_leaf(2)),
                    }),
                ))
            }),
        ),
        (
            "Lt wrong-kind B Bool error11",
            numeric(
                ScalarMode::Lt,
                Space::Const,
                7,
                Space::Const,
                KindWant::Bool(false),
            ),
            err(11),
            Box::new(|| program(bin(BinaryOp::Lt, lit_u(7), lit_b(false)))),
        ),
        (
            "Lt wrong-kind B Data Bool error11",
            numeric(
                ScalarMode::Lt,
                Space::Const,
                7,
                Space::Data,
                KindWant::Bool(true),
            ),
            err(11),
            Box::new(|| {
                program(bin(
                    BinaryOp::Lt,
                    lit_u(7),
                    bin(BinaryOp::Lt, lit_u(1), lit_u(2)),
                ))
            }),
        ),
        // Not: operand A is the checked wrong kind.
        (
            "Not wrong-kind A UInt error11",
            not_want(Space::Const, KindWant::UInt(7)),
            err(11),
            Box::new(|| program(not_expr(lit_u(7)))),
        ),
        (
            "Not wrong-kind A List/Nil error11",
            not_want(Space::Const, KindWant::Nil),
            err(11),
            Box::new(|| program(not_expr(lit_list(&[])))),
        ),
        (
            "Not wrong-kind A Cons error11",
            not_want(Space::Const, KindWant::Cons),
            err(11),
            Box::new(|| program(not_expr(lit_list(&[1])))),
        ),
        (
            "Not wrong-kind A Leaf/Tree error11",
            not_want(Space::Const, KindWant::Leaf),
            err(11),
            Box::new(|| program(not_expr(lit_tree(tree_leaf(1))))),
        ),
    ]);
}

// ---------- genuine P3, separately re-admitted bound ----------

fn assert_p3(record: &CompleteStepRecord, wanted: &Want, label: &str) {
    assert_selected_pre(record, wanted, label);
    assert_preserved(record, label);
    let pre = &record.before.image;
    let post = &record.after.image;
    let event = &record.event;
    let is_not = wanted.mode == ScalarMode::Not;
    assert_eq!(
        pre.helper_work, record.before.admission.bounds.max_helper_work,
        "{label}: P3 at exact retained bound"
    );
    assert_eq!(
        event.rule_id,
        RuleIdC::BHelperScalarArithmetic,
        "{label}: rule"
    );
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
    assert_eq!(
        post.helper, pre.helper,
        "{label}: helper preserved (no Done receipt misuse)"
    );
    assert_eq!(
        post.outcome,
        OutcomeC::Exhausted {
            reason: ExhaustReason::HelperWork
        },
        "{label}: post outcome"
    );
    // P3 preserves actual pre phase/cursor: First/0 Not, Second/1 numeric.
    assert_eq!(
        event.helper_phase_after,
        Some(HelperPhaseSummaryC::Scalar(if is_not {
            ScalarPhase::First
        } else {
            ScalarPhase::Second
        })),
        "{label}: P3 phase preserved"
    );
    assert_eq!(
        event.helper_cursor_after,
        Some(if is_not { 0 } else { 1 }),
        "{label}: P3 cursor preserved"
    );
}

fn run_p3(wanted: &Want, label: &str, make: &dyn Fn() -> CoreProgram) {
    // Choose the bound from a real selected state, then separately
    // re-admit the original program at exactly that bound. No admitted
    // witness/engine is mutated to force the case.
    let (_, _, generous_pre) = reach(generous(), wanted, label, make);
    let helper_work = generous_pre.image.helper_work;
    assert!(
        helper_work > 0,
        "{label}: selected helper work must be positive"
    );
    let exact = bounds(helper_work);
    let (admitted, mut image, pre) = reach(exact, wanted, label, make);
    assert_eq!(pre.image.helper_work, exact.max_helper_work);
    let record = replay(exact, wanted, label, make);
    assert_eq!(
        record.before, pre,
        "{label}: replay reaches identical P3 pre-state"
    );
    assert_p3(&record, wanted, label);
    let config = cfg(exact);
    let witness = record_scalar_arithmetic_step(&admitted, &mut image, &config, &limits())
        .unwrap_or_else(|e| panic!("{label}: selected P3 wrapper step: {e}"));
    let original_after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture original P3 post: {}", violation_label(&e)));
    assert_eq!(
        original_after, record.after,
        "{label}: wrapper post equals replay post"
    );
    assert!(
        witness.disposition_is_refused_helper_work(),
        "{label}: witness P3 disposition (constrained helper lane, not ungated receipt)"
    );
    assert_eq!(
        witness.source_params().admission_bounds(),
        record.before.admission.bounds,
        "{label}: witness retains original admission bounds"
    );
    assert_checks(&witness, label);
}

#[test]
fn p3_exact_fresh_bound_not_and_numeric() {
    run_p3(
        &not_want(Space::Const, KindWant::Bool(true)),
        "P3 Not Const true",
        &|| program(not_expr(lit_b(true))),
    );
    run_p3(
        &numeric(
            ScalarMode::Add,
            Space::Const,
            40,
            Space::Const,
            KindWant::UInt(2),
        ),
        "P3 Add Const UInt",
        &|| program(bin(BinaryOp::Add, lit_u(40), lit_u(2))),
    );
}

// ---------- owner-record negatives (host stage, not IR rejection) ---

#[test]
fn owner_record_cross_field_negatives_refuse_at_host_stage() {
    let wanted = numeric(
        ScalarMode::Add,
        Space::Const,
        7,
        Space::Const,
        KindWant::UInt(3),
    );
    let label = "owner-record negative base";
    let make = || program(bin(BinaryOp::Add, lit_u(7), lit_u(3)));
    let record = replay(generous(), &wanted, label, &make);
    let config = cfg(generous());
    // Positive control: the unmutated real record is accepted.
    witness_scalar_arithmetic_from_owner_record(&record, &config, &limits())
        .unwrap_or_else(|e| panic!("{label}: genuine record must pass: {e}"));

    // Each mutation crosses one real event/snapshot field. Refusal must
    // be the owner-record consumer stage (EventMismatch /
    // UnexpectedOwnerStep), never an SDK/IR checker result.
    let mut wrong_rule = record.clone();
    wrong_rule.event.rule_id = RuleIdC::KMachinePushAccept;
    assert!(
        matches!(
            witness_scalar_arithmetic_from_owner_record(&wrong_rule, &config, &limits()),
            Err(C6ScalarArithmeticConsumerError::UnexpectedOwnerStep)
        ),
        "wrong rule must refuse as UnexpectedOwnerStep at owner-record stage"
    );

    let mut mutations: Vec<(&str, Box<dyn Fn(&mut CompleteStepRecord)>)> = vec![
        (
            "disposition",
            Box::new(|r| {
                r.event.disposition = StepDisposition::Published;
            }),
        ),
        (
            "phase after",
            Box::new(|r| {
                r.event.helper_phase_after = Some(HelperPhaseSummaryC::Scalar(ScalarPhase::First));
            }),
        ),
        (
            "cursor after",
            Box::new(|r| {
                r.event.helper_cursor_after = Some(0);
            }),
        ),
        (
            "helper visits delta",
            Box::new(|r| {
                r.event.helper_visits_delta = 0;
            }),
        ),
        (
            "checked charges",
            Box::new(|r| {
                r.event.work_delta = 0;
            }),
        ),
        (
            "occurrence",
            Box::new(|r| {
                r.event.source_occurrence = LocatorC::Table {
                    table: "bogus".into(),
                    row: 0,
                };
            }),
        ),
        (
            "record swap post helper staged",
            Box::new(|r| {
                if let Some(HelperControlC::Scalar { staged, .. }) = &mut r.after.image.helper {
                    *staged = ScalarStaged::U128Value { n: 999 };
                }
            }),
        ),
        (
            "snapshot counts",
            Box::new(|r| {
                r.after.image.helper_visits += 1;
            }),
        ),
        (
            "outcome",
            Box::new(|r| {
                r.event.outcome_tag = OutcomeTagC::Exhausted;
            }),
        ),
    ];
    for (field, mutate) in &mut mutations {
        let mut bad = record.clone();
        mutate(&mut bad);
        assert!(
            matches!(
                witness_scalar_arithmetic_from_owner_record(&bad, &config, &limits()),
                Err(C6ScalarArithmeticConsumerError::EventMismatch { .. })
                    | Err(C6ScalarArithmeticConsumerError::UnexpectedOwnerStep)
            ),
            "{field}: mutated real record must refuse at owner-record stage"
        );
    }
}

// ---------- independent wrong post: host-pair witness, IR rejection --

#[test]
fn host_pair_semantically_wrong_post_reaches_checker_both_modes() {
    let wanted = numeric(
        ScalarMode::Add,
        Space::Const,
        7,
        Space::Const,
        KindWant::UInt(3),
    );
    let label = "host-pair wrong post";
    let make = || program(bin(BinaryOp::Add, lit_u(7), lit_u(3)));
    let record = replay(generous(), &wanted, label, &make);
    let config = cfg(generous());
    // Mutate only the post staged payload to a semantically wrong value
    // that still encodes. The host-pair route must produce a witness
    // (no host semantic filter, no regeneration from pre); rejection
    // must come from the actual checker in BOTH modes. LedgerCall
    // commitment is recomputed by the checker from the CURRENT mutated
    // statement and CURRENT post flat lanes.
    let mut wrong_after = record.after.clone();
    if let Some(HelperControlC::Scalar { staged, .. }) = &mut wrong_after.image.helper {
        *staged = ScalarStaged::U128Value { n: 999 };
    } else {
        panic!("{label}: post helper must be Scalar");
    }
    let witness = witness_scalar_arithmetic_from_host_pair(
        true,
        &record.before,
        &wrong_after,
        &config,
        &limits(),
    )
    .unwrap_or_else(|e| panic!("{label}: encodable wrong post must yield a witness: {e}"));
    for mode in [
        C6PushStatementBindingMode::LocalPublic,
        C6PushStatementBindingMode::LedgerCall,
    ] {
        let circuit = build_c6_scalar_arithmetic_step_circuit(
            witness.source_params().clone(),
            witness.host_prefix().meta.len(),
            witness.host_prefix().prog.len(),
            witness.host_prefix().meta_bounds_frames_offset,
            mode,
            "scalar_arith_owner_test",
        )
        .unwrap_or_else(|e| panic!("{label}: circuit builds in {mode:?}: {e}"));
        assert!(
            check_c6_scalar_arithmetic_step_circuit(&circuit, &witness).is_err(),
            "{label}: wrong post must reach and fail the checker in {mode:?} \
             (circuit category, not producer precheck)"
        );
    }
}

// ---------- wrapper non-rollback on a wrong-rule step ----------

#[test]
fn wrapper_wrong_rule_step_is_non_rollback() {
    // At genesis the next real step is not the selected arithmetic
    // step. The wrapper takes exactly that one owner step, then refuses
    // with UnexpectedOwnerStep. Category: producer/wrapper refusal
    // AFTER one real step; explicitly NOT a circuit failure and NOT
    // "no step happened" — the image is already advanced.
    let label = "wrapper non-rollback";
    let wanted = numeric(
        ScalarMode::Add,
        Space::Const,
        7,
        Space::Const,
        KindWant::UInt(3),
    );
    let make = || program(bin(BinaryOp::Add, lit_u(7), lit_u(3)));
    let program = make();
    let b = generous();
    let engine = host_owner_engine();
    let admitted = engine
        .admit(&program, &b, &layout(b))
        .unwrap_or_else(|e| panic!("{label}: admit: {}", tooling_label(&e)));
    let mut image = engine
        .admit_run(
            &admitted,
            &RunIntC::Nat(64),
            &RunIntC::Nat(8),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|e| panic!("{label}: genesis: {}", tooling_label(&e)));
    let _ = &wanted;
    let before = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture before: {}", violation_label(&e)));
    let result = record_scalar_arithmetic_step(&admitted, &mut image, &cfg(b), &limits());
    assert!(
        matches!(
            result,
            Err(C6ScalarArithmeticConsumerError::UnexpectedOwnerStep)
        ),
        "{label}: wrong-rule step must refuse as UnexpectedOwnerStep"
    );
    let after = capture(&admitted, &image)
        .unwrap_or_else(|e| panic!("{label}: capture after: {}", violation_label(&e)));
    assert_ne!(
        after, before,
        "{label}: image already advanced; no rollback"
    );
    assert_eq!(
        after.image.consumed_work,
        before.image.consumed_work + 1,
        "{label}: exactly one owner step was charged"
    );
}
