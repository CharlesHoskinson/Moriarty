//! Independent oracle f63ba247 / matrix bd480352, frozen before producer.
//! SOURCE ONLY, UNCOMPILED/UNRUN. No expected state calls owner/encoder/emitter.
//! The real owner is the subject in the admitted-record test; raw tests use the
//! complete frozen carriers directly and recompute each current commitment.
use super::*;
use super::super::c6_construct_nullary_step::{
    C6ConstructNullaryStepEmitter, C6ConstructNullaryStepInputs, C6ConstructNullaryStepError,
};
use super::super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextCanonicalError,
    C6ProgramContextExpressionCaptureInputs, C6ProgramContextCheckedSortCaptureInputs,
    C6ProgramContextConstCaptureInputs,
};
use mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets;
use mori_core::sherlock::helper_machine::complete_observation::{
    record_step, CaptureStepError, ObservedStep, ProgramSnapshot, AdmissionSnapshot,
    LayoutSnapshot, ConstFieldsSnapshot, ConstantSnapshot,
};
use mori_core::sherlock::helper_machine::shared::*;
use mori_core::sherlock::helper_machine::shared::adt::{TypeIdC, CtorIdC, TypeRowC};
use mori_core::sherlock::helper_machine::shared::adt_storage::{CtorRowExt, CtorFieldRowExt};
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::{ExtProgram, ExtDomain};
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::ExtExpr;
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    TypeExprSrc, TypeDeclSrc, CtorDeclSrc, ExtTypeBuildError,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::host_implied_prefix::SegBase;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashSet;

const SECTIONS: [(&str, usize); 6] = [
    ("META", 88), ("PROG", 156), ("CTX", 22), ("IMAGE", 74), ("DATA", 47), ("HELPER", 45),
];
fn fixture() -> Value {
    let bytes = include_bytes!("testdata/c6_construct_nullary_oracle01.json");
    assert_eq!(Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
        "f63ba2472123eacbc8aba569f3690935455198a0e2ab67cab403febc73cf7fc1");
    serde_json::from_slice(bytes).expect("frozen manual oracle JSON")
}
fn matrix() -> Value {
    let bytes = include_bytes!("testdata/c6_construct_nullary_matrix01.json");
    assert_eq!(Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
        "bd480352389f5dc53cc6e6e76dd4aeffc8649f6ff0034a6e2405794571753a82");
    serde_json::from_slice(bytes).expect("frozen manual mutation matrix JSON")
}
// Every positive carrier lane in this independently chosen fixture fits u64.
// Full-width NEGATIVE values are handled exactly in patch_after, never via f64.
fn lanes(control: &Value, side: &str, section: &str) -> Vec<u128> {
    control[side][section].as_array().expect("complete carrier")
        .iter().map(|n| u128::from(n.as_u64().expect("frozen positive lane"))).collect()
}
fn raw_statement(control: &Value) -> Vec<Fr> {
    let mut out = vec![Fr::from(control["enabled"].as_u64().expect("enable")),
        Fr::from(1u128), Fr::from(1u128)];
    for side in ["fullPreCarriers", "fullExpectedPostCarriers"] {
        for (section, len) in SECTIONS {
            let v = lanes(control, side, section);
            assert_eq!(v.len(), len, "{side}/{section}");
            out.extend(v.into_iter().map(Fr::from));
        }
    }
    assert_eq!(out.len(), 867);
    out
}
fn statement_offset(side: &str, section: &str, lane: usize) -> usize {
    let mut offset = match side { "pre" => 3, "post" => 435, _ => panic!("frozen patch side") };
    for (name, len) in SECTIONS {
        if name == section { assert!(lane < len); return offset + lane; }
        offset += len;
    }
    panic!("frozen patch section")
}
fn bounds() -> BoundsC {
    BoundsC { code_rows: 8, function_rows: 0, code_child_slots: 8, parameter_slots: 0,
        pattern_rows: 0, match_plan_rows: 0, match_arm_rows: 0, type_rows: 5,
        constructor_rows: 5, constructor_field_slots: 5, max_arity: 2,
        constant_nodes: 2, constant_field_slots: 2, data_nodes: 2, data_field_slots: 2,
        environment_cells: 2, frames: 2, helper_tasks: 2, serialization_bytes: 4096,
        max_reference_work: 16, max_helper_work: 16, step_cap: 16 }
}
fn capacities() -> LayoutCapacitiesC {
    LayoutCapacitiesC { name_rows: 3, name_bytes: 9, occurrence_path_slots: 2, value_slots: 8 }
}
fn layout() -> LayoutSchemaC {
    LayoutSchemaC { layout_version: ("lv".into(), "1".into()), field_id: None, widths: None,
        scalar_layout_id: None, bounds: bounds(), capacities: capacities(), ext_capacities: None,
        boundary_capacities_present: false }
}
fn config() -> C6HostConfig {
    C6HostConfig::from_retained_layout(&layout(), C6CodecBudgets { type_expr_nodes: 16, template_bytes: 64 })
}
fn limits() -> C6HostLimits {
    C6HostLimits { max_total_lanes: 2048, max_state_lanes: 1024,
        max_metadata_bytes: 256, max_string_bytes: 32, max_width_entries: 0 }
}
fn params() -> C6PushFamilyParams {
    C6PushFamilyParams::new(config(), bounds(), limits()).expect("manual retained family")
}
fn declaration() -> TypeDeclSrc {
    TypeDeclSrc { name: "Unit".into(), parameters: vec![], ctors: vec![
        CtorDeclSrc { name: "Done".into(), fields: vec![] }] }
}
fn identity() -> IdentityC {
    IdentityC { profile_id: 2, grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
        finance: FinanceInput::Absent, layout_version: ("lv".into(), "1".into()),
        layout_digest: DigestC::Unselected, bounds_digest: DigestC::Unselected,
        field_id: None, program_membership_digest: DigestC::Unselected }
}
fn run_id(fuel: u128) -> RunIdC {
    assert!(fuel == 7 || fuel == 3);
    RunIdC { identity: identity(), fuel_input: RunIntC::Nat(fuel), stack_limit_input: RunIntC::Nat(2) }
}
// Complete expected admission is a literal transcription of the independent
// fullAdmissionSnapshot, NOT the result of prepare/admit/capture/encode.
fn expected_admission() -> AdmissionSnapshot {
    let empty = SliceC { start: 0, len: 0 };
    let t = |name_id, decl, ctor_start, ctor_len| TypeRowC {
        name_id, decl, args: empty, ctor_start: CtorIdC(ctor_start), ctor_len };
    let ctor = |owning_type, source_order, field_start, field_len| CtorRowExt {
        owning_type: TypeIdC(owning_type), name_id: 0, source_order, field_start, field_len };
    AdmissionSnapshot { identity: identity(), bounds: bounds(),
        layout: LayoutSnapshot { layout_version: ("lv".into(), "1".into()), field_id: None,
            widths: None, scalar_layout_id: None, bounds: bounds(), capacities: capacities(),
            boundary_capacities_present: false },
        view: ProgramSnapshot { entry: 0,
            expressions: vec![ExpressionSnapshot::Let { binder: 2, value: 1, body: 2 },
                ExpressionSnapshot::Construct { type_id: TypeIdC(4), ctor_id: CtorIdC(4),
                    fields: empty, constant: Some(0) }, ExpressionSnapshot::Var { binder: 2 }],
            occurrences: vec![
                OccurrenceRowC { owner_is_function: false, function: 0, path: empty },
                OccurrenceRowC { owner_is_function: false, function: 0, path: SliceC { start: 0, len: 1 } },
                OccurrenceRowC { owner_is_function: false, function: 0, path: SliceC { start: 1, len: 1 } }],
            occurrence_paths: vec![0, 1], children: vec![], functions: vec![], parameters: vec![],
            arms: vec![], patterns: vec![], pattern_children: vec![],
            types: vec![t(0, None, 0, 0), t(0, None, 0, 0), t(0, None, 0, 2),
                t(0, None, 2, 2), t(1, Some(0), 4, 1)], type_args: vec![],
            constructors: vec![ctor(2, 0, 0, 0), ctor(2, 1, 0, 2), ctor(3, 0, 2, 1),
                ctor(3, 1, 3, 2), ctor(4, 0, 5, 0)],
            ctor_fields: [0, 2, 0, 3, 3].into_iter().map(|type_id|
                CtorFieldRowExt { name_id: 0, type_id: TypeIdC(type_id) }).collect(),
            decl_param_counts: vec![0], decl_ctor_counts: vec![1], decl_templates: vec![declaration()],
            plan_nodes: vec![], plan_cases: vec![], field_paths: vec![],
            constants: vec![ConstantSnapshot::Constructor { marker_free: true,
                type_id: TypeIdC(4), ctor_id: CtorIdC(4), fields: ConstFieldsSnapshot { start: 0, len: 0 } }],
            constant_fields: vec![], names: vec!["Done".into(), "Unit".into(), "x".into()] } }
}
fn expected_image(post: bool, fuel: u128) -> ImageSnapshot {
    let remaining_work = match (post, fuel) { (false, 7) => 5, (true, 7) => 4,
        (false, 3) => 1, (true, 3) => 0, _ => panic!("only independently frozen runs") };
    let active = ActiveLengths { data: 0, data_fields: 0, env: 0, value_slots: 0 };
    let alloc = AllocCounters { data_nodes: 0, data_fields: 0, environment_cells: 0 };
    ImageSnapshot { run_id: run_id(fuel),
        control: if post { ControlC::Return { value: RefC::Const(0) } } else { ControlC::Eval { expr: 1 } },
        control_origin: 1, env_root: None, stack_limit: 2, outcome: OutcomeC::Running,
        initial_fuel: fuel, remaining_work, consumed_work: if post { 3 } else { 2 },
        helper_work: 0, helper_visits: 0, reference_cursor: if post { 3 } else { 2 },
        alloc, active, helper: None, result: None, seg_base: SegBase::from_parts(active, alloc, 0),
        data: vec![], data_fields: vec![], env: vec![],
        frames: vec![FrameRowC::LetF { binder: 2, body: 2, saved_env: None, origin: 0 }],
        tasks: vec![], value_slots: vec![] }
}
fn expected_event(fuel: u128) -> StepEventC {
    StepEventC { step_id: ExpandedStepIdC { run_id: Arc::new(run_id(fuel)), ordinal: 2 },
        rule_id: RuleIdC::XAdtEvalConstructNullary, event_kind: EventKind::LegacyComplete,
        site: Some(SiteTag::EvalConstruct), helper_kind: None, disposition: StepDisposition::Applied,
        work_delta: 1, helper_work_delta: 0, helper_visits_delta: 0, reference_cursor_delta: 1,
        alloc_deltas: AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 },
        value_slot_writes: 0, logical_depth_before: 1, logical_depth_after: 1,
        task_cursor_before: 0, task_cursor_after: 0, helper_phase_before: None, helper_phase_after: None,
        helper_cursor_before: None, helper_cursor_after: None, result_tag: None,
        origin_reference_cursor: None, source_expr_id: 1,
        source_occurrence: LocatorC::Occurrence { owner_is_function: false, function: 0, ast_path: vec![0] },
        exhausted_kind: None, outcome_tag: OutcomeTagC::Running, outcome_payload: OutcomeC::Running,
        boundary: BoundaryKind::None }
}
fn admission_error(e: ExtTypeBuildError) -> String {
    match e {
        ExtTypeBuildError::CoreTooling(t) => format!("CoreTooling {t:?}"),
        ExtTypeBuildError::ExtCapacityNotAdmitted { kind, limit, locator, .. } =>
            format!("ExtCapacity {kind:?} limit={limit} locator={locator:?}"),
        ExtTypeBuildError::ExtAdmissionRejected { reason, detail, locator } =>
            format!("ExtAdmission {reason:?}: {detail}, {locator:?}"),
        ExtTypeBuildError::ExtUnsupportedFeature { feature, locator } =>
            format!("ExtFeature {feature:?}, {locator:?}"),
        ExtTypeBuildError::Host(v) => format!("Host {:?}, {:?}", v.kind, v.locator),
    }
}
fn capture_error(e: CaptureStepError) -> String {
    let v = match e { CaptureStepError::BeforeObservation(v) => v,
        CaptureStepError::Step { violation, .. } => violation,
        CaptureStepError::AfterObservation { violation, .. } => violation };
    format!("{:?}, {:?}", v.kind, v.locator)
}
fn genuine_record(fuel: u128) -> CompleteStepRecord {
    let program = ExtProgram { types: vec![declaration()], assets: vec![], states: vec![],
        inputs: vec![], functions: vec![], entry: ExtExpr::Let { name: "x".into(),
            value: Box::new(ExtExpr::Construct { ty: TypeExprSrc::Named { name: "Unit".into() },
                ctor: "Done".into(), fields: vec![] }), body: Box::new(ExtExpr::Var { name: "x".into() }) } };
    let (admitted, _) = admit_ext_program(program, ExtDomain::GENERIC_ABSENT, &bounds(), &layout(), None)
        .unwrap_or_else(|e| panic!("frozen Unit/Done admission: {}", admission_error(e)));
    // Both independently frozen fuels (7 and3) fit value_slots8; stack2 fits frames2.
    let mut image = host_owner_engine().admit_run(&admitted, &RunIntC::Nat(fuel),
        &RunIntC::Nat(2), &FinanceInput::Absent).unwrap_or_else(|e| panic!("admit run: {e:?}"));
    for rule in [RuleIdC::KMachineEvalLet, RuleIdC::KMachinePushAccept] {
        match record_step(&admitted, &mut image).unwrap_or_else(|e| panic!("prelude {rule:?}: {}", capture_error(e))) {
            ObservedStep::Event(r) => { assert_eq!(r.event.rule_id, rule); assert_eq!(r.after.image.outcome, OutcomeC::Running); },
            ObservedStep::Terminal { .. } => panic!("terminal before frozen nullary predecessor"),
        }
    }
    match record_step(&admitted, &mut image).unwrap_or_else(|e| panic!("nullary step: {}", capture_error(e))) {
        ObservedStep::Event(r) => r,
        ObservedStep::Terminal { .. } => panic!("terminal instead of nullary event"),
    }
}
fn circuit(mode: C6PushStatementBindingMode, scope: &str) -> C6ConstructNullaryStepCircuit {
    build_c6_construct_nullary_step_circuit(params(), 88, 156, 31, mode, scope)
        .expect("normal construction must reach expression, nominal and constant captures")
}
fn sdk_rejects(c: &C6ConstructNullaryStepCircuit, statement: &[Fr], id: &str) {
    match check_c6_construct_nullary_statement(c, statement) {
        Err(C6ConstructNullaryConsumerError::Backend(s)) =>
            assert!(s.starts_with("SDK ProofPreimage::check failed:"), "{id}: unexpected host error {s}"),
        Err(_) => panic!("{id}: host shape failure is not an SDK refusal"),
        Ok(_) => panic!("{id}: frozen negative was accepted"),
    }
}
#[test]
fn independent_full_admitted_record_and_carriers_match_both_producers() {
    let o = fixture();
    for (fuel, name) in [(7, "enabledPositive"), (3, "lastFuelPositive")] {
        let r = genuine_record(fuel);
        assert_eq!(r.before, HostSnapshot { admission: expected_admission(), image: expected_image(false, fuel) });
        assert_eq!(r.after, HostSnapshot { admission: expected_admission(), image: expected_image(true, fuel) });
        assert_eq!(r.event, expected_event(fuel), "complete28-field independent host event");
        let want = raw_statement(&o[name]);
        let owner = witness_from_owner_record(&r, &config(), &limits()).expect("owner record producer");
        let pair = witness_from_host_pair(true, &r.before, &r.after, &config(), &limits()).expect("independent-post producer");
        assert_eq!(owner.to_statement_frs(), want, "full867 carriers from owner record");
        assert_eq!(pair.to_statement_frs(), want, "full867 carriers from host pair");
    }
}
#[test]
fn manual_enabled_lastfuel_disabled_pass_both_binding_modes() {
    let o = fixture();
    for (mode, scope) in [(C6PushStatementBindingMode::LocalPublic, "nullary_positive_local"),
        (C6PushStatementBindingMode::LedgerCall, "nullary_positive_ledger")] {
        let c = circuit(mode, scope);
        assert_eq!(c.statement_lane_count(), 867);
        assert_eq!(c.geometry().state_total_lanes, 166);
        for name in ["enabledPositive", "lastFuelPositive", "disabledPositive"] {
            check_c6_construct_nullary_statement(&c, &raw_statement(&o[name])).expect(name);
        }
    }
}
fn patch_after(id: &str, patch: &Value) -> Fr {
    let side = patch["side"].as_str().expect("side");
    let section = patch["section"].as_str().expect("section");
    let lane = patch["lane"].as_u64().expect("lane") as usize;
    // The exact frozen JSON byte hash above is verified FIRST. serde_json::Value
    // may round >u64 integers; these three declared full-width values therefore
    // use exact frozen Rust constants and never a floating-point conversion.
    if id == "nonUInt128" {
        assert_eq!((side, section, lane), ("post", "DATA", 7));
        return Fr::from(u128::MAX) + Fr::from(1u128); // exactly2^128
    }
    if side == "pre" && matches!(id, "consumed-overflow" | "reference-overflow") {
        assert_eq!(section, "IMAGE");
        assert_eq!(lane, if id == "consumed-overflow" { 28 } else { 31 });
        return Fr::from(u128::MAX);
    }
    Fr::from(u128::from(patch["after"].as_u64().expect("other frozen patch is exact u64")))
}
#[test]
fn all465_independent_raw_mutants_reject_both_binding_modes() {
    let o = fixture(); let m = matrix();
    let post = m["postLaneMatrix"].as_array().expect("432 lane cases");
    let named = m["semanticAndFrameCases"].as_array().expect("33 semantic cases");
    assert_eq!((post.len(), named.len()), (432, 33));
    for (mode, scope) in [(C6PushStatementBindingMode::LocalPublic, "nullary_mutant_local"),
        (C6PushStatementBindingMode::LedgerCall, "nullary_mutant_ledger")] {
        let c = circuit(mode, scope); let base = raw_statement(&o["enabledPositive"]);
        check_c6_construct_nullary_statement(&c, &base).expect("positive before every raw negative batch");
        let mut ids = HashSet::new();
        for case in post.iter().chain(named.iter()) {
            let id = case["id"].as_str().expect("case ID"); assert!(ids.insert(id));
            assert_eq!(case["expectedRelation"], Value::Bool(false));
            let mut statement = base.clone();
            statement[0] = Fr::from(case["enabled"].as_u64().expect("raw enable"));
            for patch in case["patches"].as_array().expect("raw patches") {
                let index = statement_offset(patch["side"].as_str().unwrap(),
                    patch["section"].as_str().unwrap(), patch["lane"].as_u64().unwrap() as usize);
                statement[index] = patch_after(id, patch);
            }
            sdk_rejects(&c, &statement, id);
        }
        assert_eq!(ids.len(), 465);
    }
}
#[test]
fn disabled_identity_rejects_each_post_carrier_lane_change() {
    let o = fixture(); let control = &o["disabledPositive"];
    for (mode, scope) in [(C6PushStatementBindingMode::LocalPublic, "nullary_disabled_local"),
        (C6PushStatementBindingMode::LedgerCall, "nullary_disabled_ledger")] {
        let c = circuit(mode, scope); let base = raw_statement(control);
        check_c6_construct_nullary_statement(&c, &base).expect("independent disabled identity control");
        let mut checked = 0;
        for (section, len) in SECTIONS {
            let expected = lanes(control, "fullExpectedPostCarriers", section);
            for lane in 0..len {
                let mut statement = base.clone();
                statement[statement_offset("post", section, lane)] = Fr::from(expected[lane] + 1);
                sdk_rejects(&c, &statement, &format!("disabled-post-{section}-{lane}")); checked += 1;
            }
        }
        assert_eq!(checked, 432);
    }
}
fn fragment_inputs() -> C6ConstructNullaryStepInputs {
    let o = fixture(); let control = &o["enabledPositive"]; let p = params();
    let ops = |side, section| lanes(control, side, section).into_iter()
        .map(|v| Operand::Immediate(Fr::from(v))).collect::<Vec<_>>();
    let pre_ctx = ops("fullPreCarriers", "CTX"); let post_ctx = ops("fullExpectedPostCarriers", "CTX");
    C6ConstructNullaryStepInputs { enable: Operand::Immediate(Fr::from(1u128)),
        host_schema: Operand::Immediate(Fr::from(1u128)), carrier_version: Operand::Immediate(Fr::from(1u128)),
        geometry: p.geometry(), meta_bounds_frames_offset: 31, source_params: p,
        pre_meta: ops("fullPreCarriers", "META"), pre_prog: ops("fullPreCarriers", "PROG"),
        pre_ctx: std::array::from_fn(|i| pre_ctx[i].clone()), pre_image: ops("fullPreCarriers", "IMAGE"),
        pre_data: ops("fullPreCarriers", "DATA"), pre_helper: ops("fullPreCarriers", "HELPER"),
        post_meta: ops("fullExpectedPostCarriers", "META"), post_prog: ops("fullExpectedPostCarriers", "PROG"),
        post_ctx: std::array::from_fn(|i| post_ctx[i].clone()), post_image: ops("fullExpectedPostCarriers", "IMAGE"),
        post_data: ops("fullExpectedPostCarriers", "DATA"), post_helper: ops("fullExpectedPostCarriers", "HELPER") }
}
#[test]
fn retained_subscope_composition_constructs_and_never_rebinds_outputs() {
    let inputs = fragment_inputs(); let mut e = C6ConstructNullaryStepEmitter::new("nullary_composed").unwrap();
    // Candidate01 fails here before nominal instructions: this is a construction
    // regression, not credit for refusing a semantic negative or an SDK pass.
    let a = e.emit(&inputs).expect("dependent-reader construction1");
    let b = e.emit(&inputs).expect("same retained emitter construction2");
    let mut outputs = HashSet::new(); let mut count = 0;
    for instruction in a.instructions.iter().chain(b.instructions.iter()) {
        let output = match instruction {
            Instruction::Add { output, .. } | Instruction::Mul { output, .. }
            | Instruction::Not { output, .. } | Instruction::CondSelect { output, .. }
            | Instruction::TestEq { output, .. } | Instruction::LessThan { output, .. } => Some(output),
            _ => None,
        };
        if let Some(output) = output { assert!(outputs.insert(output.0.clone()), "rebound {}", output.0); count += 1; }
    }
    assert!(count > 0);
    for suffix in [".nullaryexpr.c6progctx.temp.", ".nullarynom.c6progctx.temp.", ".nullaryconst.c6progctx.temp."] {
        assert!(outputs.iter().any(|id| id.starts_with(&format!("%nullary_composed{suffix}"))), "reader not reached: {suffix}");
    }
}
#[test]
fn every_derived_caller_namespace_and_reader_own_namespace_stays_reserved() {
    let scope = "nullary_reserved"; let inputs = fragment_inputs();
    for segment in [".local.temp.", ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
        ".nullaryexpr.c6progctx.temp.", ".nullarynom.c6progctx.temp.", ".nullaryconst.c6progctx.temp."] {
        let id = format!("%{scope}{segment}0");
        for location in 0..15 {
            let mut bad = inputs.clone(); let operand = Operand::Variable(Identifier(id.clone()));
            match location { 0 => bad.enable = operand, 1 => bad.host_schema = operand, 2 => bad.carrier_version = operand,
                3 => bad.pre_meta[0] = operand, 4 => bad.post_meta[0] = operand, 5 => bad.pre_prog[0] = operand,
                6 => bad.post_prog[0] = operand, 7 => bad.pre_ctx[0] = operand, 8 => bad.post_ctx[0] = operand,
                9 => bad.pre_image[0] = operand, 10 => bad.post_image[0] = operand, 11 => bad.pre_data[0] = operand,
                12 => bad.post_data[0] = operand, 13 => bad.pre_helper[0] = operand, 14 => bad.post_helper[0] = operand,
                _ => unreachable!() }
            let mut e = C6ConstructNullaryStepEmitter::new(scope).unwrap();
            match e.emit(&bad) { Err(C6ConstructNullaryStepError::ReservedOperand { identifier }) => assert_eq!(identifier, id),
                _ => panic!("parent must reject caller namespace at {location}/{segment}") }
        }
    }
    // Do not repair composition by weakening ANY existing standalone guard.
    let own = Operand::Variable(Identifier("%strict.c6progctx.temp.0".into()));
    let gate = Operand::Immediate(Fr::from(1u128)); let caps = params().caps();
    let mut e = C6ProgramContextCanonicalEmitter::new("strict").unwrap();
    assert!(matches!(e.emit_prog_expression_capture(&C6ProgramContextExpressionCaptureInputs {
        gate: gate.clone(), pre_prog: inputs.pre_prog.clone(), caps, selected_index: own.clone(),
    }), Err(C6ProgramContextCanonicalError::ReservedOperand { .. })));
    for (selected_type_id, selected_ctor_id) in [(own.clone(), gate.clone()), (gate.clone(), own.clone())] {
        assert!(matches!(e.emit_prog_checked_sort_capture(&C6ProgramContextCheckedSortCaptureInputs {
            gate: gate.clone(), pre_prog: inputs.pre_prog.clone(), caps, selected_type_id, selected_ctor_id,
        }), Err(C6ProgramContextCanonicalError::ReservedOperand { .. })));
    }
    assert!(matches!(e.emit_prog_const_capture(&C6ProgramContextConstCaptureInputs {
        gate, pre_prog: inputs.pre_prog.clone(), caps, selected_index: own,
    }), Err(C6ProgramContextCanonicalError::ReservedOperand { .. })));
}
