//! SOURCE ONLY, UNCOMPILED/UNRUN. Actual-call adapter regressions.
//! Expected full snapshots/event/420-lane HOST values are independent oracle
//! f63ba247 (mp2_actual_review), frozen before this adapter. Literal builders
//! transcribe that oracle; admitted owner and codec calls are subjects only.
//! The private after-failure fixture is supplied codec data, not an owner step.
use super::*;
use crate::sherlock::helper_machine::c6_phys_host::{C6HostError, decode_host};
use crate::sherlock::helper_machine::complete_observation::{
    AdmissionSnapshot, CompleteStepRecord, ConstFieldsSnapshot, ConstantSnapshot, DataSnapshot,
    ExpressionSnapshot, ImageSnapshot, LayoutSnapshot, ProgramSnapshot, capture,
};
use crate::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC, TypeRowC};
use crate::sherlock::helper_machine::shared::adt_storage::{CtorFieldRowExt, CtorRowExt};
use crate::sherlock::helper_machine::shared::host_implied_prefix::SegBase;
use crate::sherlock::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::ExtExpr;
use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, TypeDeclSrc, TypeExprSrc,
};
use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use crate::sherlock::helper_machine::shared::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::sync::Arc;

fn oracle() -> Value {
    let bytes = include_bytes!("testdata/c6_complete_step_capture_oracle01.json");
    assert_eq!(
        Sha256::digest(bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        "f63ba2472123eacbc8aba569f3690935455198a0e2ab67cab403febc73cf7fc1"
    );
    serde_json::from_slice(bytes).expect("independent oracle JSON")
}
fn expected_host(post: bool, fuel: u128) -> HostSnapshot {
    HostSnapshot {
        admission: expected_admission(),
        image: expected_image(post, fuel),
    }
}
fn compact_host(post: bool) -> Vec<u128> {
    let o = oracle();
    o["enabledPositive"][if post {
        "expectedPostCompactHOST"
    } else {
        "preCompactHOST"
    }]
    .as_array()
    .expect("independent complete420lanes")
    .iter()
    .map(|x| u128::from(x.as_u64().expect("manual lane fits u64")))
    .collect()
}
fn budgets() -> C6CodecBudgets {
    C6CodecBudgets {
        type_expr_nodes: 16,
        template_bytes: 64,
    }
}
fn bounds() -> BoundsC {
    BoundsC {
        code_rows: 8,
        function_rows: 0,
        code_child_slots: 8,
        parameter_slots: 0,
        pattern_rows: 0,
        match_plan_rows: 0,
        match_arm_rows: 0,
        type_rows: 5,
        constructor_rows: 5,
        constructor_field_slots: 5,
        max_arity: 2,
        constant_nodes: 2,
        constant_field_slots: 2,
        data_nodes: 2,
        data_field_slots: 2,
        environment_cells: 2,
        frames: 2,
        helper_tasks: 2,
        serialization_bytes: 4096,
        max_reference_work: 16,
        max_helper_work: 16,
        step_cap: 16,
    }
}
fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC {
        name_rows: 3,
        name_bytes: 9,
        occurrence_path_slots: 2,
        value_slots: 8,
    }
}
fn layout() -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("lv".into(), "1".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: bounds(),
        capacities: capacities(),
        ext_capacities: None,
        boundary_capacities_present: false,
    }
}
fn config() -> C6HostConfig {
    C6HostConfig::from_retained_layout(
        &layout(),
        C6CodecBudgets {
            type_expr_nodes: 16,
            template_bytes: 64,
        },
    )
}
fn limits() -> C6HostLimits {
    C6HostLimits {
        max_total_lanes: 2048,
        max_state_lanes: 1024,
        max_metadata_bytes: 256,
        max_string_bytes: 32,
        max_width_entries: 0,
    }
}
fn declaration() -> TypeDeclSrc {
    TypeDeclSrc {
        name: "Unit".into(),
        parameters: vec![],
        ctors: vec![CtorDeclSrc {
            name: "Done".into(),
            fields: vec![],
        }],
    }
}
fn identity() -> IdentityC {
    IdentityC {
        profile_id: 2,
        grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
        finance: FinanceInput::Absent,
        layout_version: ("lv".into(), "1".into()),
        layout_digest: DigestC::Unselected,
        bounds_digest: DigestC::Unselected,
        field_id: None,
        program_membership_digest: DigestC::Unselected,
    }
}
fn run_id(fuel: u128) -> RunIdC {
    assert!(fuel == 7 || fuel == 3);
    RunIdC {
        identity: identity(),
        fuel_input: RunIntC::Nat(fuel),
        stack_limit_input: RunIntC::Nat(2),
    }
}
// Complete expected admission is a literal transcription of the independent
// fullAdmissionSnapshot, NOT the result of prepare/admit/capture/encode.
fn expected_admission() -> AdmissionSnapshot {
    let empty = SliceC { start: 0, len: 0 };
    let t = |name_id, decl, ctor_start, ctor_len| TypeRowC {
        name_id,
        decl,
        args: empty,
        ctor_start: CtorIdC(ctor_start),
        ctor_len,
    };
    let ctor = |owning_type, source_order, field_start, field_len| CtorRowExt {
        owning_type: TypeIdC(owning_type),
        name_id: 0,
        source_order,
        field_start,
        field_len,
    };
    AdmissionSnapshot {
        identity: identity(),
        bounds: bounds(),
        layout: LayoutSnapshot {
            layout_version: ("lv".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds: bounds(),
            capacities: capacities(),
            boundary_capacities_present: false,
        },
        view: ProgramSnapshot {
            entry: 0,
            expressions: vec![
                ExpressionSnapshot::Let {
                    binder: 2,
                    value: 1,
                    body: 2,
                },
                ExpressionSnapshot::Construct {
                    type_id: TypeIdC(4),
                    ctor_id: CtorIdC(4),
                    fields: empty,
                    constant: Some(0),
                },
                ExpressionSnapshot::Var { binder: 2 },
            ],
            occurrences: vec![
                OccurrenceRowC {
                    owner_is_function: false,
                    function: 0,
                    path: empty,
                },
                OccurrenceRowC {
                    owner_is_function: false,
                    function: 0,
                    path: SliceC { start: 0, len: 1 },
                },
                OccurrenceRowC {
                    owner_is_function: false,
                    function: 0,
                    path: SliceC { start: 1, len: 1 },
                },
            ],
            occurrence_paths: vec![0, 1],
            children: vec![],
            functions: vec![],
            parameters: vec![],
            arms: vec![],
            patterns: vec![],
            pattern_children: vec![],
            types: vec![
                t(0, None, 0, 0),
                t(0, None, 0, 0),
                t(0, None, 0, 2),
                t(0, None, 2, 2),
                t(1, Some(0), 4, 1),
            ],
            type_args: vec![],
            constructors: vec![
                ctor(2, 0, 0, 0),
                ctor(2, 1, 0, 2),
                ctor(3, 0, 2, 1),
                ctor(3, 1, 3, 2),
                ctor(4, 0, 5, 0),
            ],
            ctor_fields: [0, 2, 0, 3, 3]
                .into_iter()
                .map(|type_id| CtorFieldRowExt {
                    name_id: 0,
                    type_id: TypeIdC(type_id),
                })
                .collect(),
            decl_param_counts: vec![0],
            decl_ctor_counts: vec![1],
            decl_templates: vec![declaration()],
            plan_nodes: vec![],
            plan_cases: vec![],
            field_paths: vec![],
            constants: vec![ConstantSnapshot::Constructor {
                marker_free: true,
                type_id: TypeIdC(4),
                ctor_id: CtorIdC(4),
                fields: ConstFieldsSnapshot { start: 0, len: 0 },
            }],
            constant_fields: vec![],
            names: vec!["Done".into(), "Unit".into(), "x".into()],
        },
    }
}
fn expected_image(post: bool, fuel: u128) -> ImageSnapshot {
    let remaining_work = match (post, fuel) {
        (false, 7) => 5,
        (true, 7) => 4,
        (false, 3) => 1,
        (true, 3) => 0,
        _ => panic!("only independently frozen runs"),
    };
    let active = ActiveLengths {
        data: 0,
        data_fields: 0,
        env: 0,
        value_slots: 0,
    };
    let alloc = AllocCounters {
        data_nodes: 0,
        data_fields: 0,
        environment_cells: 0,
    };
    ImageSnapshot {
        run_id: run_id(fuel),
        control: if post {
            ControlC::Return {
                value: RefC::Const(0),
            }
        } else {
            ControlC::Eval { expr: 1 }
        },
        control_origin: 1,
        env_root: None,
        stack_limit: 2,
        outcome: OutcomeC::Running,
        initial_fuel: fuel,
        remaining_work,
        consumed_work: if post { 3 } else { 2 },
        helper_work: 0,
        helper_visits: 0,
        reference_cursor: if post { 3 } else { 2 },
        alloc,
        active,
        helper: None,
        result: None,
        seg_base: SegBase::from_parts(active, alloc, 0),
        data: vec![],
        data_fields: vec![],
        env: vec![],
        frames: vec![FrameRowC::LetF {
            binder: 2,
            body: 2,
            saved_env: None,
            origin: 0,
        }],
        tasks: vec![],
        value_slots: vec![],
    }
}
fn expected_event(fuel: u128) -> StepEventC {
    StepEventC {
        step_id: ExpandedStepIdC {
            run_id: Arc::new(run_id(fuel)),
            ordinal: 2,
        },
        rule_id: RuleIdC::XAdtEvalConstructNullary,
        event_kind: EventKind::LegacyComplete,
        site: Some(SiteTag::EvalConstruct),
        helper_kind: None,
        disposition: StepDisposition::Applied,
        work_delta: 1,
        helper_work_delta: 0,
        helper_visits_delta: 0,
        reference_cursor_delta: 1,
        alloc_deltas: AllocDeltasC {
            data_nodes: 0,
            data_fields: 0,
            environment_cells: 0,
        },
        value_slot_writes: 0,
        logical_depth_before: 1,
        logical_depth_after: 1,
        task_cursor_before: 0,
        task_cursor_after: 0,
        helper_phase_before: None,
        helper_phase_after: None,
        helper_cursor_before: None,
        helper_cursor_after: None,
        result_tag: None,
        origin_reference_cursor: None,
        source_expr_id: 1,
        source_occurrence: LocatorC::Occurrence {
            owner_is_function: false,
            function: 0,
            ast_path: vec![0],
        },
        exhausted_kind: None,
        outcome_tag: OutcomeTagC::Running,
        outcome_payload: OutcomeC::Running,
        boundary: BoundaryKind::None,
    }
}

fn pair_at_nullary(fuel: u128) -> (Admitted, MachineImage) {
    let program = ExtProgram {
        types: vec![declaration()],
        assets: vec![],
        states: vec![],
        inputs: vec![],
        functions: vec![],
        entry: ExtExpr::Let {
            name: "x".into(),
            value: Box::new(ExtExpr::Construct {
                ty: TypeExprSrc::Named {
                    name: "Unit".into(),
                },
                ctor: "Done".into(),
                fields: vec![],
            }),
            body: Box::new(ExtExpr::Var { name: "x".into() }),
        },
    };
    let (admitted, _) = admit_ext_program(
        program,
        ExtDomain::GENERIC_ABSENT,
        &bounds(),
        &layout(),
        None,
    )
    .unwrap_or_else(|_| panic!("independent Unit/Done admission"));
    let mut image = host_owner_engine()
        .admit_run(
            &admitted,
            &RunIntC::Nat(fuel),
            &RunIntC::Nat(2),
            &FinanceInput::Absent,
        )
        .expect("actual genesis");
    for rule in [RuleIdC::KMachineEvalLet, RuleIdC::KMachinePushAccept] {
        match record_step(&admitted, &mut image).unwrap_or_else(|_| panic!("actual fixed prelude"))
        {
            ObservedStep::Event(r) => assert_eq!(r.event.rule_id, rule),
            ObservedStep::Terminal { .. } => panic!("terminal before nullary predecessor"),
        }
    }
    assert_eq!(
        capture(&admitted, &image).expect("actual prestate"),
        expected_host(false, fuel)
    );
    (admitted, image)
}
fn assert_independent_nullary(observed: &ObservedStep, fuel: u128) {
    match observed {
        ObservedStep::Event(r) => {
            assert_eq!(r.before, expected_host(false, fuel));
            assert_eq!(r.after, expected_host(true, fuel));
            assert_eq!(
                r.event,
                expected_event(fuel),
                "all28 independent raw event fields"
            );
        }
        ObservedStep::Terminal { .. } => panic!("nullary must be actual event"),
    }
}

#[test]
fn actual_nullary_event_matches_independent_complete_host_and420lanes() {
    let (admitted, mut image) = pair_at_nullary(7);
    let captured = record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
        .expect("actual one-step compact capture");
    assert_independent_nullary(captured.observed(), 7);
    assert_eq!(captured.before_lanes(), compact_host(false));
    assert_eq!(captured.after_lanes(), compact_host(true));
    assert_eq!(captured.config(), &config());
    // An accidental second step would advance another rule/work/cursor.
    assert_eq!(
        capture(&admitted, &image).expect("actual current"),
        expected_host(true, 7)
    );
    let (observed, retained, before, after) = captured.into_parts();
    assert_independent_nullary(&observed, 7);
    assert_eq!(retained, config());
    assert_eq!(before, compact_host(false));
    assert_eq!(after, compact_host(true));
}

#[test]
fn before_codec_failure_preserves_genuine_advanced_step() {
    let (admitted, mut image) = pair_at_nullary(7);
    let mut tight = limits();
    tight.max_total_lanes = 419;
    match record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &tight) {
        Err(C6CompleteStepCaptureError::Materialization {
            side: C6CaptureSide::Before,
            failure:
                C6BoundaryCaptureFailure::Codec(C6HostError::LimitExceeded {
                    scope: "host.total_lanes",
                    needed: 420,
                    limit: 419,
                }),
            observed,
            config: retained,
            completed_before: None,
        }) => {
            assert_independent_nullary(&observed, 7);
            assert_eq!(retained, config());
        }
        _ => panic!("typed before failure must retain genuine advanced boundaries"),
    }
    assert_eq!(
        capture(&admitted, &image).expect("advanced pair"),
        expected_host(true, 7)
    );
}

#[test]
fn wrong_retained_layout_preserves_genuine_step() {
    let (admitted, mut image) = pair_at_nullary(7);
    let mut wrong = layout();
    wrong.layout_version.1.push('x');
    match record_c6_complete_step(&admitted, &mut image, &wrong, budgets(), &limits()) {
        Err(C6CompleteStepCaptureError::Materialization {
            side: C6CaptureSide::Before,
            failure:
                C6BoundaryCaptureFailure::RetainedLayout(C6StateError::RetainedLayoutMismatch {
                    field: "layout_version",
                }),
            observed,
            completed_before: None,
            ..
        }) => assert_independent_nullary(&observed, 7),
        _ => panic!("wrong original layout must fail without erasing actual step"),
    }
    assert_eq!(
        capture(&admitted, &image).expect("advanced pair"),
        expected_host(true, 7)
    );
}

#[test]
fn after_codec_failure_keeps_encoded_before_and_complete_raw() {
    // Private materializer branch only: supplied complete post has a new data
    // row. This is NOT claimed to be a genuine nullary transition or a proof.
    let before = expected_host(false, 7);
    let mut after = expected_host(true, 7);
    after.image.data.push(DataSnapshot::U128 { n: 7 });
    after.image.active.data = 1;
    after.image.alloc.data_nodes = 1;
    let supplied = ObservedStep::Event(CompleteStepRecord {
        before,
        event: expected_event(7),
        after,
    });
    let mut tight = limits();
    tight.max_state_lanes = 140;
    match encode_observed(supplied.clone(), &layout(), budgets(), &tight) {
        Err(C6CompleteStepCaptureError::Materialization {
            side: C6CaptureSide::After,
            failure:
                C6BoundaryCaptureFailure::Codec(C6HostError::State {
                    error:
                        C6StateError::LimitExceeded {
                            needed: 145,
                            limit: 140,
                        },
                    ..
                }),
            observed,
            completed_before: Some(encoded_before),
            ..
        }) => {
            assert_eq!(*observed, supplied);
            assert_eq!(encoded_before, compact_host(false));
        }
        _ => panic!("after failure must retain raw post and completed independent pre420"),
    }
}

#[test]
fn terminal_is_retained_without_invented_event() {
    let (admitted, mut image) = pair_at_nullary(3);
    let nullary = record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
        .expect("last-fuel nullary");
    assert_independent_nullary(nullary.observed(), 3);
    let publication =
        record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
            .expect("actual uncharged fuel publication");
    match publication.observed() {
        ObservedStep::Event(r) => {
            assert_eq!(r.event.rule_id, RuleIdC::KAdminPublishExhausted);
            assert_eq!(r.event.work_delta, 0);
            assert_eq!(r.after.image.consumed_work, 3);
        }
        _ => panic!("publication precedes Terminal"),
    }
    let terminal = record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
        .expect("actual terminal capture");
    let mut expected = expected_host(true, 3);
    expected.image.outcome = OutcomeC::Exhausted {
        reason: ExhaustReason::Fuel,
    };
    match terminal.observed() {
        ObservedStep::Terminal { before, after } => {
            assert_eq!(before, &expected);
            assert_eq!(after, &expected);
        }
        ObservedStep::Event(_) => panic!("Terminal must not fabricate an event"),
    }
    assert_eq!(terminal.before_lanes(), terminal.after_lanes());
    assert_eq!(
        decode_host(terminal.before_lanes(), terminal.config(), &limits())
            .expect("complete compact terminal")
            .host(),
        &expected
    );
    assert_eq!(
        capture(&admitted, &image).expect("unchanged current"),
        expected
    );
}

#[test]
fn AB_mismatch_preserves_existing_owner_refusal_without_step() {
    let (admitted_a, _) = pair_at_nullary(7);
    let (admitted_b, mut image_b) = pair_at_nullary(7);
    match record_c6_complete_step(&admitted_a, &mut image_b, &layout(), budgets(), &limits()) {
        Err(C6CompleteStepCaptureError::Owner(CaptureStepError::BeforeObservation(v))) => {
            assert_eq!(v.kind, HostViolationKind::StepInvViolated);
            assert_eq!(v.clause, Some(StepInvClauseId::SI01));
            assert_eq!(v.locator.as_deref(), Some("admission binding"));
        }
        _ => panic!("AB-first owner refusal must pass through unchanged"),
    }
    assert_eq!(
        capture(&admitted_b, &image_b).expect("original bound image"),
        expected_host(false, 7)
    );
}

// Source-only streaming tests. Same real owner pair; no proxy full16 fixture.
use super::super::c6_range_stream::{
    C6RangeStream, OwnerAdvance, RangeCaptureError, StreamBinding, StreamBounds, StreamFailure,
};
use super::super::c6_range_stream_read::C6RangeReader;
use super::super::c6_stream_event::{decode_event, encode_event};
fn stream_binding() -> StreamBinding {
    StreamBinding {
        source_sha256: [0; 32],
        input_sha256: [0; 32],
        expected_result_sha256: [0; 32],
    }
}
fn stream_bounds() -> StreamBounds {
    StreamBounds {
        max_record_bytes: 1_000_000,
        max_total_bytes: 4_000_000,
        max_records: 100,
    }
}
#[test]
fn stream_actual_nullary_matches_frozen_lossless_prefix_and_inverse() {
    let (a, mut image) = pair_at_nullary(7);
    let initial = capture(&a, &image).unwrap();
    let mut stream = C6RangeStream::begin(
        Vec::new(),
        &initial,
        config(),
        &limits(),
        stream_bounds(),
        stream_binding(),
    )
    .unwrap_or_else(|_| panic!("begin"));
    let mut custody = C6StepCustody::default();
    let observed = stream
        .record_next(
            &a,
            &mut image,
            &layout(),
            budgets(),
            &limits(),
            &mut custody,
        )
        .expect("same actual owner once");
    assert_independent_nullary(&observed, 7);
    assert!(
        custody.held().is_none(),
        "published step leaves no in-flight custody"
    );
    let (bytes, p) = stream.into_partial();
    assert_eq!(p.records, 1);
    assert_eq!(p.completed_intents, 0);
    assert!(!p.terminal && !p.complete);
    let independent = include_bytes!("testdata/c6_stream_nullary_prefix01.bin");
    assert_eq!(bytes.as_slice(), independent);
    assert_eq!(
        Sha256::digest(&bytes)
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect::<String>(),
        "1c5f57033617e25be3b8a6bb94aad18a600756d2d355593a924a4b2183526f55"
    );
    let mut reader = C6RangeReader::begin(
        std::io::Cursor::new(&bytes),
        stream_binding(),
        config(),
        limits(),
        stream_bounds(),
    )
    .unwrap();
    let decoded = reader.next().unwrap().unwrap();
    assert_eq!(decoded.sequence, 0);
    assert_eq!(decoded.range, 0);
    assert_eq!(decoded.observed, observed);
    assert!(reader.next().is_err(), "prefix has no complete16 footer");
    assert!(reader.next().is_err(), "read error poisons continuation");
    let mut bad = bytes.clone();
    bad[6834..6842].copy_from_slice(&1u64.to_le_bytes());
    let mut reader = C6RangeReader::begin(
        std::io::Cursor::new(bad),
        stream_binding(),
        config(),
        limits(),
        stream_bounds(),
    )
    .unwrap();
    assert!(reader.next().is_err(), "record sequence gap");
    let mut bad = bytes;
    bad[8] ^= 1;
    assert!(
        C6RangeReader::begin(
            std::io::Cursor::new(bad),
            stream_binding(),
            config(),
            limits(),
            stream_bounds()
        )
        .is_err(),
        "source identity mismatch"
    );
    assert_eq!(capture(&a, &image).unwrap(), expected_host(true, 7));
}
#[test]
fn stream_event_codec_preserves_complete_raw_fields_and_refuses_corruption() {
    let mut event = expected_event(7);
    event.step_id.ordinal = u128::MAX;
    event.alloc_deltas = AllocDeltasC {
        data_nodes: -9,
        data_fields: i64::MIN,
        environment_cells: i64::MAX,
    };
    event.helper_phase_before = Some(HelperPhaseSummaryC::Bind(BindPhase::Finished));
    event.helper_phase_after = Some(HelperPhaseSummaryC::Build(BuildPhase::Done));
    event.helper_cursor_before = Some(0);
    event.helper_cursor_after = Some(usize::MAX);
    event.result_tag = Some(HelperResultTagC::PlanMatched);
    event.origin_reference_cursor = Some(u128::MAX);
    event.source_occurrence = LocatorC::Table {
        table: "complete-Δ".into(),
        row: usize::MAX,
    };
    event.exhausted_kind = Some(ExhaustReason::HelperWork);
    event.outcome_tag = OutcomeTagC::Halted;
    event.outcome_payload = OutcomeC::Halted {
        value: RefC::Data(7),
    };
    event.boundary = BoundaryKind::TechnicalCut;
    // Supplied event only tests the codec, never claims an actual owner transition.
    let encoded = encode_event(&event, &run_id(7), 512).unwrap();
    assert_eq!(decode_event(&encoded, &run_id(7)).unwrap(), event);
    let mut bad = encoded.clone();
    bad[1] = u128::MAX;
    assert!(decode_event(&bad, &run_id(7)).is_err());
    let mut bad = encoded.clone();
    bad.push(0);
    assert!(decode_event(&bad, &run_id(7)).is_err());
    assert!(encode_event(&event, &run_id(3), 512).is_err());
    assert!(encode_event(&event, &run_id(7), encoded.len() - 1).is_err());
}
struct FailStream {
    bytes: Vec<u8>,
    remaining: usize,
}
impl std::io::Write for FailStream {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if self.remaining == 0 {
            return Err(std::io::Error::other("injected owned IO failure"));
        }
        let n = b.len().min(self.remaining);
        self.bytes.extend_from_slice(&b[..n]);
        self.remaining -= n;
        Ok(n)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
fn stream_partial_io_and_byte_bound_preserve_actual_advanced_capture() {
    for partial_io in [false, true] {
        let (a, mut image) = pair_at_nullary(7);
        let initial = capture(&a, &image).unwrap();
        let mut bound = stream_bounds();
        if !partial_io {
            bound.max_total_bytes = 6843
        }
        let output = FailStream {
            bytes: Vec::new(),
            remaining: if partial_io { 6843 } else { usize::MAX },
        };
        let mut stream = C6RangeStream::begin(
            output,
            &initial,
            config(),
            &limits(),
            bound,
            stream_binding(),
        )
        .unwrap_or_else(|_| panic!("initial bound"));
        let mut custody = C6StepCustody::default();
        match stream.record_next(
            &a,
            &mut image,
            &layout(),
            budgets(),
            &limits(),
            &mut custody,
        ) {
            Err(RangeCaptureError::Stream {
                capture: Some(c),
                progress,
                ..
            }) => {
                assert_independent_nullary(c.observed(), 7);
                assert_eq!(c.before_lanes(), compact_host(false));
                assert_eq!(c.after_lanes(), compact_host(true));
                assert!(progress.poisoned && !progress.complete);
                assert_eq!(progress.records, 0);
            }
            _ => panic!("failed publication retains genuine complete capture"),
        }
        assert!(
            custody.held().is_none(),
            "complete capture moved into the error, not duplicated"
        );
        assert_eq!(capture(&a, &image).unwrap(), expected_host(true, 7));
        assert!(stream.progress().poisoned);
        assert_eq!(stream.progress().records, 0);
        assert!(
            stream
                .record_next(
                    &a,
                    &mut image,
                    &layout(),
                    budgets(),
                    &limits(),
                    &mut custody
                )
                .is_err()
        );
        assert!(custody.held().is_none());
        assert_eq!(capture(&a, &image).unwrap(), expected_host(true, 7));
        let (output, p) = stream.into_partial();
        assert_eq!(p.bytes, if partial_io { 6843 } else { 6832 });
        assert_eq!(output.bytes.len() as u64, p.bytes);
    }
}
#[test]
fn stream_actual_terminal_has_no_event_and_cannot_finish_missing16() {
    let (a, mut image) = pair_at_nullary(3);
    let initial = capture(&a, &image).unwrap();
    let mut stream = C6RangeStream::begin(
        Vec::new(),
        &initial,
        config(),
        &limits(),
        stream_bounds(),
        stream_binding(),
    )
    .unwrap_or_else(|_| panic!("begin"));
    let mut custody = C6StepCustody::default();
    for _ in 0..3 {
        stream
            .record_next(
                &a,
                &mut image,
                &layout(),
                budgets(),
                &limits(),
                &mut custody,
            )
            .expect("actual lastfuel/publication/Terminal");
        assert!(custody.held().is_none());
    }
    assert!(stream.progress().terminal);
    assert_eq!(stream.progress().completed_intents, 0);
    assert!(
        stream
            .record_next(
                &a,
                &mut image,
                &layout(),
                budgets(),
                &limits(),
                &mut custody
            )
            .is_err()
    );
    let stream = match stream.finish() {
        Err(error) => error.stream,
        Ok(_) => panic!("unit capture is not full16"),
    };
    let (bytes, _) = stream.into_partial();
    let mut reader = C6RangeReader::begin(
        std::io::Cursor::new(bytes),
        stream_binding(),
        config(),
        limits(),
        stream_bounds(),
    )
    .unwrap();
    assert!(matches!(
        reader.next().unwrap().unwrap().observed,
        ObservedStep::Event(_)
    ));
    assert!(matches!(
        reader.next().unwrap().unwrap().observed,
        ObservedStep::Event(_)
    ));
    match reader.next().unwrap().unwrap().observed {
        ObservedStep::Terminal { before, after } => assert_eq!(before, after),
        _ => panic!("Terminal invented event"),
    };
    assert!(reader.next().is_err());
}
struct PanicStream {
    bytes: Vec<u8>,
    panic_at: usize,
}
impl std::io::Write for PanicStream {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if self.bytes.len() >= self.panic_at {
            panic!("injected writer panic")
        }
        let n = b.len().min(self.panic_at - self.bytes.len());
        self.bytes.extend_from_slice(&b[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
// Same real owner pair. The writer panics mid-frame after the 6832-byte header
// and the 2-byte frame tag/range: a torn publish of the actual nullary step.
#[test]
fn stream_writer_panic_holds_actual_capture_poisons_and_never_steps_again() {
    let (a, mut image) = pair_at_nullary(7);
    let initial = capture(&a, &image).unwrap();
    let output = PanicStream {
        bytes: Vec::new(),
        panic_at: 6834,
    };
    let mut stream = C6RangeStream::begin(
        output,
        &initial,
        config(),
        &limits(),
        stream_bounds(),
        stream_binding(),
    )
    .unwrap_or_else(|_| panic!("begin"));
    let mut custody = C6StepCustody::default();
    let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stream.record_next(
            &a,
            &mut image,
            &layout(),
            budgets(),
            &limits(),
            &mut custody,
        )
    }));
    let payload = match unwound {
        Err(p) => p,
        Ok(_) => panic!("writer panic must unwind out of record_next"),
    };
    assert_eq!(
        payload.downcast_ref::<&str>(),
        Some(&"injected writer panic"),
        "unwind is the injected writer, not another fault"
    );
    let held = custody
        .held()
        .expect("unwind leaves the actual step in custody");
    assert_independent_nullary(held.observed(), 7);
    assert_eq!(held.config(), &config());
    assert_eq!(held.before_lanes(), Some(compact_host(false).as_slice()));
    assert_eq!(held.after_lanes(), Some(compact_host(true).as_slice()));
    let p = stream.progress();
    assert!(
        p.poisoned && !p.complete && !p.terminal,
        "unwound publish leaves the stream poisoned"
    );
    assert_eq!((p.records, p.bytes, p.completed_intents), (0, 6834, 0));
    let independent = include_bytes!("testdata/c6_stream_nullary_prefix01.bin");
    let torn: [u8; 32] = Sha256::digest(&independent[..6834]).into();
    assert_eq!(p.sha256, torn);
    assert_eq!(
        capture(&a, &image).unwrap(),
        expected_host(true, 7),
        "exactly one owner step"
    );
    let (observed, retained, before, after) =
        custody.take().expect("custody extraction").into_parts();
    assert!(custody.held().is_none());
    assert_independent_nullary(&observed, 7);
    assert_eq!(retained, config());
    assert_eq!(before, Some(compact_host(false)));
    assert_eq!(after, Some(compact_host(true)));
    let e = match stream.record_next(
        &a,
        &mut image,
        &layout(),
        budgets(),
        &limits(),
        &mut custody,
    ) {
        Err(e) => e,
        Ok(_) => panic!("poisoned stream must refuse"),
    };
    assert_eq!(e.owner_advance(), OwnerAdvance::NotStepped);
    let RangeCaptureError::Stream {
        failure: StreamFailure::Poisoned,
        capture: None,
        progress,
    } = e
    else {
        panic!("refusal must be Poisoned before any owner step")
    };
    assert!(progress.poisoned && !progress.complete);
    assert_eq!((progress.records, progress.bytes), (0, 6834));
    assert!(custody.held().is_none());
    assert_eq!(
        capture(&a, &image).unwrap(),
        expected_host(true, 7),
        "no second owner step after catch/unwind/extraction"
    );
    let (output, _) = stream.into_partial();
    assert_eq!(output.bytes.as_slice(), &independent[..6834]);
}

// Source-only saved-counter getter checks. Literal tuples are the frozen plan
// expectations (initial,remaining,consumed,helper,reference), never getter
// output. Every carrier comes from the real owner capture route or fixture.
#[test]
fn counter_observation_capture_fuel7_event_reads_saved_before_after_owner_advance() {
    let (admitted, mut image) = pair_at_nullary(7);
    let captured = record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
        .expect("actual fuel7 nullary capture");
    assert_independent_nullary(captured.observed(), 7);
    assert_eq!(
        capture(&admitted, &image).expect("advanced pair"),
        expected_host(true, 7)
    );
    let c = captured.before_counter_observation();
    assert_eq!(
        (
            c.initial_fuel(),
            c.remaining_work(),
            c.consumed_work(),
            c.helper_work(),
            c.reference_cursor()
        ),
        (7, 5, 2, 0, 2),
        "saved BEFORE, not AFTER (7,4,3,0,3)"
    );
    assert_eq!(c.remaining_checked_add_consumed(), Some(7));
    assert_eq!(c.consumed_checked_sub_helper(), Some(2));
    assert!(c.fuel_sum_matches_initial());
    assert!(c.cursor_difference_matches_reference());
    assert!(c.remaining_work_is_positive());
    assert_independent_nullary(captured.observed(), 7);
    assert_eq!(
        capture(&admitted, &image).expect("getter never steps owner"),
        expected_host(true, 7)
    );
}

#[test]
fn counter_observation_capture_lastfuel3_event_reads_positive_saved_before() {
    let (admitted, mut image) = pair_at_nullary(3);
    let captured = record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
        .expect("actual last-fuel nullary capture");
    assert_independent_nullary(captured.observed(), 3);
    assert_eq!(
        capture(&admitted, &image).expect("advanced pair"),
        expected_host(true, 3)
    );
    let c = captured.before_counter_observation();
    assert_eq!(
        (
            c.initial_fuel(),
            c.remaining_work(),
            c.consumed_work(),
            c.helper_work(),
            c.reference_cursor()
        ),
        (3, 1, 2, 0, 2),
        "saved BEFORE, not AFTER (3,0,3,0,3)"
    );
    assert_eq!(c.remaining_checked_add_consumed(), Some(3));
    assert_eq!(c.consumed_checked_sub_helper(), Some(2));
    assert!(c.fuel_sum_matches_initial());
    assert!(c.cursor_difference_matches_reference());
    assert!(
        c.remaining_work_is_positive(),
        "BEFORE fuel1, not post-charge fuel0"
    );
    assert_independent_nullary(captured.observed(), 3);
    assert_eq!(
        capture(&admitted, &image).expect("getter never steps owner"),
        expected_host(true, 3)
    );
}

#[test]
fn counter_observation_capture_fuel3_terminal_reads_equal_saved_boundary() {
    let (admitted, mut image) = pair_at_nullary(3);
    let nullary = record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
        .expect("last-fuel nullary");
    assert_independent_nullary(nullary.observed(), 3);
    let publication =
        record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
            .expect("actual uncharged fuel publication");
    match publication.observed() {
        ObservedStep::Event(r) => {
            assert_eq!(r.event.rule_id, RuleIdC::KAdminPublishExhausted);
            assert_eq!(r.event.work_delta, 0);
            assert_eq!(r.after.image.consumed_work, 3);
        }
        _ => panic!("publication precedes Terminal"),
    }
    let terminal = record_c6_complete_step(&admitted, &mut image, &layout(), budgets(), &limits())
        .expect("actual terminal capture");
    let mut expected = expected_host(true, 3);
    expected.image.outcome = OutcomeC::Exhausted {
        reason: ExhaustReason::Fuel,
    };
    assert!(
        matches!(terminal.observed(), ObservedStep::Terminal { before, after }
        if before == &expected && after == &expected),
        "actual Terminal with equal saved boundaries"
    );
    assert_eq!(capture(&admitted, &image).expect("terminal pair"), expected);
    let c = terminal.before_counter_observation();
    assert_eq!(
        (
            c.initial_fuel(),
            c.remaining_work(),
            c.consumed_work(),
            c.helper_work(),
            c.reference_cursor()
        ),
        (3, 0, 3, 0, 3)
    );
    assert_eq!(c.remaining_checked_add_consumed(), Some(3));
    assert_eq!(c.consumed_checked_sub_helper(), Some(3));
    assert!(c.fuel_sum_matches_initial());
    assert!(c.cursor_difference_matches_reference());
    assert!(
        !c.remaining_work_is_positive(),
        "Terminal accounting holds with no positive fuel"
    );
    assert!(
        matches!(terminal.observed(), ObservedStep::Terminal { before, after }
        if before == &expected && after == &expected),
        "getter leaves Terminal unchanged"
    );
    assert_eq!(
        capture(&admitted, &image).expect("getter never steps owner"),
        expected
    );
}

#[test]
fn counter_observation_inflight_writer_unwind_reads_saved_before() {
    let (a, mut image) = pair_at_nullary(7);
    let initial = capture(&a, &image).unwrap();
    let output = PanicStream {
        bytes: Vec::new(),
        panic_at: 6834,
    };
    let mut stream = C6RangeStream::begin(
        output,
        &initial,
        config(),
        &limits(),
        stream_bounds(),
        stream_binding(),
    )
    .unwrap_or_else(|_| panic!("begin"));
    let mut custody = C6StepCustody::default();
    let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stream.record_next(
            &a,
            &mut image,
            &layout(),
            budgets(),
            &limits(),
            &mut custody,
        )
    }));
    let payload = match unwound {
        Err(p) => p,
        Ok(_) => panic!("writer panic must unwind out of record_next"),
    };
    assert_eq!(
        payload.downcast_ref::<&str>(),
        Some(&"injected writer panic"),
        "unwind is the injected writer, not another fault"
    );
    let held = custody
        .held()
        .expect("unwind leaves the actual step in custody");
    assert_independent_nullary(held.observed(), 7);
    assert_eq!(held.before_lanes(), Some(compact_host(false).as_slice()));
    assert_eq!(held.after_lanes(), Some(compact_host(true).as_slice()));
    let c = held.before_counter_observation();
    assert_eq!(
        (
            c.initial_fuel(),
            c.remaining_work(),
            c.consumed_work(),
            c.helper_work(),
            c.reference_cursor()
        ),
        (7, 5, 2, 0, 2),
        "saved BEFORE, not AFTER (7,4,3,0,3)"
    );
    assert_eq!(c.remaining_checked_add_consumed(), Some(7));
    assert_eq!(c.consumed_checked_sub_helper(), Some(2));
    assert!(c.fuel_sum_matches_initial());
    assert!(c.cursor_difference_matches_reference());
    assert!(c.remaining_work_is_positive());
    assert_independent_nullary(held.observed(), 7);
    let p = stream.progress();
    assert!(
        p.poisoned && !p.complete && !p.terminal,
        "unwound publish leaves the stream poisoned"
    );
    assert_eq!((p.records, p.bytes, p.completed_intents), (0, 6834, 0));
    assert_eq!(
        capture(&a, &image).unwrap(),
        expected_host(true, 7),
        "exactly one owner step"
    );
    let inflight = custody.take().expect("custody extraction");
    assert!(custody.held().is_none());
    assert_independent_nullary(inflight.observed(), 7);
    let e = match stream.record_next(
        &a,
        &mut image,
        &layout(),
        budgets(),
        &limits(),
        &mut custody,
    ) {
        Err(e) => e,
        Ok(_) => panic!("poisoned stream must refuse"),
    };
    assert_eq!(e.owner_advance(), OwnerAdvance::NotStepped);
    let RangeCaptureError::Stream {
        failure: StreamFailure::Poisoned,
        capture: None,
        progress,
    } = e
    else {
        panic!("refusal must be Poisoned before any owner step")
    };
    assert!(progress.poisoned && !progress.complete);
    assert_eq!((progress.records, progress.bytes), (0, 6834));
    assert!(custody.held().is_none());
    assert_eq!(
        capture(&a, &image).unwrap(),
        expected_host(true, 7),
        "no second owner step after catch/unwind/extraction"
    );
    let independent = include_bytes!("testdata/c6_stream_nullary_prefix01.bin");
    let (output, _) = stream.into_partial();
    assert_eq!(output.bytes.as_slice(), &independent[..6834]);
}
