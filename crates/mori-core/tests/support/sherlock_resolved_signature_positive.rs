//! Explicit external ordinary QA consumer; not an autodiscovered Cargo test.
//! Four frozen builtin inputs run the real admitted host owner. Nominal IDs are
//! exercised only as public carrier storage, never passed to admission/runtime.
//! Absolute B clocks are independently UNBOUND; K charges are not B expectations.
use mori_core::sherlock::helper_machine::complete_observation::{
    capture, record_step, ConstantSnapshot, ExpressionSnapshot, HostSnapshot,
    ObservedStep, ProgramSnapshot,
};
use mori_core::sherlock::helper_machine::shared::*;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use serde_json::Value;

const NORMAL: &str = include_str!("fixtures/resolved_signature/NORMAL-RUNTIME-PRESERVATION.json");
const STORAGE: &str = include_str!("fixtures/resolved_signature/NOMINAL-STORAGE-ONLY.json");
const IDENTITIES: &str = include_str!("fixtures/resolved_signature/BUILTIN-TYPE-IDENTITIES.json");

fn tooling<T>(result: Result<T, Tooling>) -> T {
    match result {
        Ok(v) => v,
        Err(Tooling::AdmissionRejected { reason, detail, locator }) =>
            panic!("ordinary fixture admission: {reason:?} {detail} {locator:?}"),
        Err(Tooling::CapacityNotAdmitted { kind, needed, limit, locator }) =>
            panic!("ordinary fixture capacity: {kind:?} {needed:?} {limit} {locator:?}"),
        Err(Tooling::UnsupportedFeature { feature, locator }) =>
            panic!("ordinary fixture unsupported: {feature:?} {locator:?}"),
    }
}

// Concrete host-only fixture uses the existing checked V4 fixture layout API.
// These are case capacities, not changes to product/security resource policy.
fn fixture_layout() -> (BoundsC, LayoutSchemaC) {
    let b = BoundsC {
        code_rows: 64, function_rows: 4, code_child_slots: 16, parameter_slots: 8,
        pattern_rows: 8, match_plan_rows: 8, match_arm_rows: 8, type_rows: 8,
        constructor_rows: 8, constructor_field_slots: 16, max_arity: 8,
        constant_nodes: 16, constant_field_slots: 16, data_nodes: 64,
        data_field_slots: 64, environment_cells: 64, frames: 64, helper_tasks: 64,
        serialization_bytes: 65536, max_reference_work: 4096,
        max_helper_work: 4096, step_cap: 4096,
    };
    let l = LayoutSchemaC {
        layout_version: ("moriarty".into(), "sherlock-helper-microstep-hostonly/1".into()),
        field_id: None, widths: None, scalar_layout_id: None, bounds: b,
        capacities: LayoutCapacitiesC {
            name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 4096,
        },
        ext_capacities: None, boundary_capacities_present: false,
    };
    (b, l)
}

fn source_sort(s: &str) -> SortTag {
    match s {
        "UInt" => SortTag::UInt, "Bool" => SortTag::Bool,
        "List" => SortTag::List, "Tree" => SortTag::Tree,
        _ => panic!("outside four frozen builtin input constructors"),
    }
}
fn kint(v: &Value) -> i128_class::KIntC {
    i128_class::KIntC::InRange(v.as_str().unwrap().parse().unwrap())
}
// Input materialization only: no AST evaluation or replacement type predicate.
fn source_tree(v: &Value) -> CoreTree {
    if let Some(n) = v.get("Leaf") { CoreTree::Leaf { n: kint(n) } }
    else {
        let children = v["Branch"].as_array().unwrap();
        assert_eq!(children.len(), 2);
        CoreTree::Branch {
            left: Box::new(source_tree(&children[0])),
            right: Box::new(source_tree(&children[1])),
        }
    }
}
fn source_value(v: &Value) -> CoreValue {
    if let Some(n) = v.get("UInt") { CoreValue::UInt { n: kint(n) } }
    else if let Some(b) = v.get("Bool") { CoreValue::Bool { b: b.as_bool().unwrap() } }
    else if let Some(xs) = v.get("List") {
        CoreValue::List { elements: xs.as_array().unwrap().iter().map(kint).collect() }
    } else { CoreValue::Tree { tree: source_tree(&v["Tree"]) } }
}

// Direct checks of the actual immutable literal graph, including every field.
// These are read-only assertions, not another language evaluator or value engine.
fn uint_constant(p: &ProgramSnapshot, i: usize, n: &Value) {
    assert_eq!(p.constants[i], ConstantSnapshot::U128 {
        marker_free: true, n: n.as_str().unwrap().parse().unwrap(),
    });
}
fn node_fields<'a>(p: &'a ProgramSnapshot, i: usize, kind: &str) -> &'a [usize] {
    let f = match (&p.constants[i], kind) {
        (ConstantSnapshot::Cons { marker_free: true, fields }, "Cons") => fields,
        (ConstantSnapshot::Leaf { marker_free: true, fields }, "Leaf") => fields,
        (ConstantSnapshot::Branch { marker_free: true, fields }, "Branch") => fields,
        _ => panic!("literal node {i} is not expected {kind}: {:?}", p.constants[i]),
    };
    &p.constant_fields[f.start..f.start + f.len]
}
fn tree_constant(p: &ProgramSnapshot, i: usize, expected: &Value) {
    if let Some(n) = expected.get("Leaf") {
        let f = node_fields(p, i, "Leaf"); assert_eq!(f.len(), 1);
        uint_constant(p, f[0], n);
    } else {
        let f = node_fields(p, i, "Branch"); assert_eq!(f.len(), 2);
        tree_constant(p, f[0], &expected["Branch"][0]);
        tree_constant(p, f[1], &expected["Branch"][1]);
    }
}
fn literal_constant(p: &ProgramSnapshot, i: usize, expected: &Value) {
    if let Some(n) = expected.get("UInt") { uint_constant(p, i, n); }
    else if let Some(b) = expected.get("Bool") {
        assert_eq!(p.constants[i], ConstantSnapshot::Bool { marker_free: true, b: b.as_bool().unwrap() });
    } else if let Some(xs) = expected.get("List") {
        let mut cursor = i;
        for n in xs.as_array().unwrap() {
            let f = node_fields(p, cursor, "Cons"); assert_eq!(f.len(), 2);
            uint_constant(p, f[0], n); cursor = f[1];
        }
        assert_eq!(p.constants[cursor], ConstantSnapshot::Nil { marker_free: true });
    } else { tree_constant(p, i, &expected["Tree"]); }
}
fn bound_value(s: &HostSnapshot, root: EnvRootC, binder: usize, argument: RefC) {
    let cell = s.image.env[root.expect("actual parameter environment")];
    assert_eq!(cell.binder, binder);
    assert_eq!(cell.value, argument);
    assert_eq!(cell.previous, None); // exactly the one frozen parameter
}


// Inspect every observed boundary, not just creation of a carrier or its final value.
fn live_carriers(s: &HostSnapshot, f: FunctionRowC, p: ParameterRowC, argument: RefC, origin: usize) {
    for task in &s.image.tasks {
        if let HelperTaskC::PairTask { binder, sort, arg } = *task {
            assert_eq!((binder, sort, arg), (p.binder, p.sort, argument));
        }
    }
    for frame in &s.image.frames {
        if let FrameRowC::ReturnF { return_type, saved_env, origin: actual_origin } = *frame {
            assert_eq!((return_type, saved_env, actual_origin), (f.return_type, None, origin));
        }
    }
    match s.image.control {
        ControlC::Bound { binding: BoundBinding::Ok { env }, return_type, body } => {
            assert_eq!((return_type, body), (f.return_type, f.body));
            bound_value(s, env, p.binder, argument);
        },
        ControlC::ScheduleNext { frame: FrameRowC::ReturnF { return_type, saved_env, origin: actual_origin }, next, env } => {
            assert_eq!((return_type, saved_env, actual_origin, next), (f.return_type, None, origin, f.body));
            bound_value(s, env, p.binder, argument);
        },
        _ => {},
    }
    if let Some(helper) = s.image.helper {
        match helper {
            HelperControlC::Bind { function, params, p_cursor, a_cursor, partial, .. } => {
                assert_eq!(function, 0); assert_eq!(params, f.params);
                assert_eq!(p_cursor, a_cursor); assert!(p_cursor <= 1);
                if partial.is_some() { bound_value(s, partial, p.binder, argument); }
            },
            HelperControlC::Scalar { common, mode, operand_a, operand_b, expected, .. } => {
                assert_eq!(common.site, SiteTag::RetCall); assert_eq!(mode, ScalarMode::TestType);
                assert_eq!((operand_a, operand_b, expected), (argument, None, Some(f.return_type)));
            },
            _ => {},
        }
    }
    match s.image.result {
        Some(HelperResultC::BindOk { env }) => bound_value(s, env, p.binder, argument),
        Some(HelperResultC::EnvFound { value }) => assert_eq!(value, argument),
        Some(HelperResultC::ScalarTypeTest { ok }) => assert!(ok),
        _ => {},
    }
    if s.image.env_root.is_some() { bound_value(s, s.image.env_root, p.binder, argument); }
}

fn run_identity(case_id: &str) {
    let all: Value = serde_json::from_str(NORMAL).unwrap();
    let case = all["cases"].as_array().unwrap().iter()
        .find(|c| c["id"] == case_id).unwrap();
    assert_eq!(case["kind"], "identity");
    let source = &case["source_program_descriptor"];
    assert_eq!(source["functions"].as_array().unwrap().len(), 1);
    let function = &source["functions"][0];
    assert_eq!(function["parameters"].as_array().unwrap().len(), 1);
    let parameter = &function["parameters"][0];
    let name = function["name"].as_str().unwrap();
    let binder_name = parameter["name"].as_str().unwrap();
    assert_eq!(function["body"]["Var"], binder_name);
    assert_eq!(source["entry"]["Call"]["name"], name);
    let arguments = source["entry"]["Call"]["arguments"].as_array().unwrap();
    assert_eq!(arguments.len(), 1);
    let literal = &arguments[0]["Literal"];
    assert_eq!(*literal, case["B_expected_preservation"]["logical_outcome"]["Done"]);
    assert_eq!(*literal, case["bound_logical_result"]["environment"][binder_name]);
    assert_eq!(*literal, case["return_test_logical_expectation"]["actual_return_value"]);
    let program = CoreProgram {
        functions: vec![CoreFunction {
            name: name.into(),
            parameters: vec![CoreParameter { name: binder_name.into(),
                sort: source_sort(parameter["source_sort"].as_str().unwrap()) }],
            return_type: source_sort(function["return_source_sort"].as_str().unwrap()),
            body: CoreExpr::Var { name: binder_name.into() },
        }],
        entry: CoreExpr::Call { name: name.into(), args: vec![CoreExpr::Literal {
            value: source_value(literal),
        }] },
    };
    let expected = TypeIdC(case["prepared_signature_expectations"]["return_canonical_type_id"].as_u64().unwrap() as usize);
    assert_eq!(case["prepared_signature_expectations"]["parameter_canonical_type_id"].as_u64().unwrap() as usize, expected.0);
    let ids: Value = serde_json::from_str(IDENTITIES).unwrap();
    let identity = ids["builtin_mapping"].as_array().unwrap().iter()
        .find(|r| r["source_sort"] == function["return_source_sort"]).unwrap();
    assert_eq!(identity["canonical_type_id"].as_u64().unwrap() as usize, expected.0);
    let (bounds, layout) = fixture_layout();
    let owner = host_owner_engine();
    let prepared = tooling(owner.prepare_core(&program, &bounds, &layout));
    owner.check_program_view(&prepared, &bounds, &layout).unwrap_or_else(|v|
        panic!("normal prepared view: {:?} {} {:?} {}", v.check, v.table, v.row, v.detail));
    assert_eq!(prepared.functions.len(), 1);
    assert_eq!(prepared.parameters.len(), 1);
    let f = prepared.functions[0];
    let p = prepared.parameters[0];
    let _: TypeIdC = f.return_type; // cannot compile against old SortTag slots
    let _: TypeIdC = p.sort;
    assert_eq!(f.return_type, expected);
    assert_eq!(p.sort, expected);
    assert_eq!(f.params, SliceC { start: 0, len: 1 });
    assert_eq!(prepared.names[f.function_name], name);
    assert_eq!(prepared.names[p.binder], binder_name);
    assert!(matches!(prepared.expressions[f.body], ExpressionRowC::Var { binder } if binder == p.binder));
    let args = match prepared.expressions[prepared.entry] {
        ExpressionRowC::Call { function_name, args } => {
            assert_eq!(function_name, f.function_name); args
        }, _ => panic!("prepared entry must be actual call"),
    };
    assert_eq!(args.len, 1);
    let argument_expr = prepared.children[args.start];
    let constant = match prepared.expressions[argument_expr] {
        ExpressionRowC::Literal { constant } => constant,
        _ => panic!("prepared actual argument must be literal"),
    };
    let argument = RefC::Const(constant);
    let admitted = tooling(owner.admit(&program, &bounds, &layout));
    let fuel = case["fixed_normal_run_inputs"]["initial_fuel"].as_u64().unwrap() as u128;
    let stack = case["fixed_normal_run_inputs"]["stack_limit"].as_u64().unwrap() as u128;
    let mut image = tooling(owner.admit_run(&admitted, &RunIntC::Nat(fuel), &RunIntC::Nat(stack), &FinanceInput::Absent));
    let initial = capture(&admitted, &image).unwrap_or_else(|_| panic!("initial bound observation"));
    assert_eq!(initial.admission.view.functions, prepared.functions);
    assert_eq!(initial.admission.view.parameters, prepared.parameters);
    assert_eq!(initial.admission.view.names, prepared.names);
    assert_eq!(initial.admission.view.children, prepared.children);
    assert_eq!(initial.admission.view.entry, prepared.entry);
    assert_eq!(initial.admission.view.expressions[argument_expr], ExpressionSnapshot::Literal { constant });
    literal_constant(&initial.admission.view, constant, literal);
    assert_eq!(initial.image.initial_fuel, fuel);
    assert_eq!(initial.image.stack_limit as u128, stack);
    assert_eq!(initial.image.env_root, None);
    let mut previous = initial.clone();
    let mut stages = Vec::new();
    let mut actual_bound_env: Option<EnvRootC> = None;
    let mut events = 0usize;
    let mut terminal = false;
    // Existing case step_cap bounds this observation loop; no numeric B-clock oracle.
    for _ in 0..=bounds.step_cap {
        match record_step(&admitted, &mut image).unwrap_or_else(|_| panic!("actual owner step/observation")) {
            ObservedStep::Terminal { before, after } => {
                assert_eq!(before, previous); assert_eq!(after, before);
                assert_eq!(after.image.outcome, OutcomeC::Halted { value: argument });
                assert_eq!(after.image.frames, vec![]);
                assert_eq!(after.image.env_root, None);
                assert!(after.image.helper.is_none());
                assert!(after.image.tasks.is_empty());
                literal_constant(&after.admission.view, constant, &case["B_expected_preservation"]["logical_outcome"]["Done"]);
                terminal = true; break;
            },
            ObservedStep::Event(r) => {
                events += 1;
                assert_eq!(r.before, previous);
                assert_eq!(r.after.admission, initial.admission);
                live_carriers(&r.before, f, p, argument, prepared.entry);
                live_carriers(&r.after, f, p, argument, prepared.entry);
                let before = &r.before.image; let after = &r.after.image;
                assert_eq!(after.consumed_work, before.consumed_work + u128::from(r.event.work_delta));
                assert_eq!(after.remaining_work + u128::from(r.event.work_delta), before.remaining_work);
                assert_eq!(after.helper_work, before.helper_work + u128::from(r.event.helper_work_delta));
                assert_eq!(after.helper_visits, before.helper_visits + u128::from(r.event.helper_visits_delta));
                assert_eq!(after.reference_cursor, before.reference_cursor + u128::from(r.event.reference_cursor_delta));
                assert_eq!(after.consumed_work + after.remaining_work, fuel);
                assert_eq!(r.event.logical_depth_before, before.frames.len());
                assert_eq!(r.event.logical_depth_after, after.frames.len());
                assert_eq!(r.event.outcome_payload, after.outcome);
                match r.event.rule_id {
                    RuleIdC::BHelperBindPair => {
                        let HelperControlC::Bind { function, params, .. } = before.helper.unwrap() else { panic!("real Bind producer") };
                        assert_eq!(function, 0); assert_eq!(params, f.params);
                        let HelperTaskC::PairTask { binder, sort, arg } = *after.tasks.last().unwrap() else { panic!("real PairTask producer") };
                        assert_eq!((binder, sort, arg), (p.binder, p.sort, argument));
                        stages.push("pair");
                    },
                    RuleIdC::KMachineEnterBind => {
                        assert!(matches!(before.control, ControlC::Enter { function: Some(0), .. }));
                        let ControlC::Bound { binding: BoundBinding::Ok { env }, return_type, body } = after.control else { panic!("real Bound producer") };
                        assert_eq!(return_type, f.return_type); assert_eq!(body, f.body);
                        bound_value(&r.after, env, p.binder, argument);
                        actual_bound_env = Some(env); stages.push("bound");
                    },
                    RuleIdC::KMachineBoundValid => {
                        let ControlC::ScheduleNext { frame: FrameRowC::ReturnF { return_type, saved_env, origin }, next, env } = after.control else { panic!("real ReturnF schedule") };
                        assert_eq!(return_type, f.return_type); assert_eq!(saved_env, None);
                        assert_eq!(origin, prepared.entry); assert_eq!(next, f.body);
                        assert_eq!(Some(env), actual_bound_env); bound_value(&r.after, env, p.binder, argument);
                        stages.push("schedule_return");
                    },
                    RuleIdC::KMachinePushAccept => {
                        if let ControlC::ScheduleNext { frame: FrameRowC::ReturnF { .. }, .. } = before.control {
                            let FrameRowC::ReturnF { return_type, saved_env, .. } = *after.frames.last().unwrap() else { panic!("real pushed ReturnF") };
                            assert_eq!(return_type, f.return_type); assert_eq!(saved_env, None);
                            assert_eq!(after.control, ControlC::Eval { expr: f.body });
                            assert_eq!(Some(after.env_root), actual_bound_env);
                            bound_value(&r.after, after.env_root, p.binder, argument); stages.push("return_frame");
                        }
                    },
                    RuleIdC::KMachineEvalVariablePresent => {
                        assert_eq!(before.control, ControlC::Eval { expr: f.body });
                        assert_eq!(before.result, Some(HelperResultC::EnvFound { value: argument }));
                        assert_eq!(after.control, ControlC::Return { value: argument }); stages.push("body_value");
                    },
                    RuleIdC::BHelperScalarEntry => {
                        let FrameRowC::ReturnF { return_type, .. } = *before.frames.last().unwrap() else { panic!("scalar's actual continuation") };
                        let HelperControlC::Scalar { common, mode, operand_a, operand_b, expected: scalar_expected, .. } = after.helper.unwrap() else { panic!("real Scalar producer") };
                        assert_eq!(before.control, ControlC::Return { value: argument });
                        assert_eq!(common.site, SiteTag::RetCall); assert_eq!(mode, ScalarMode::TestType);
                        assert_eq!(operand_a, argument); assert_eq!(operand_b, None);
                        assert_eq!(return_type, f.return_type); assert_eq!(scalar_expected, Some(return_type));
                        stages.push("scalar_expected");
                    },
                    RuleIdC::BHelperScalarTag => {
                        let HelperControlC::Scalar { expected: scalar_expected, staged, .. } = after.helper.unwrap() else { panic!("real scalar type test") };
                        assert_eq!(scalar_expected, Some(f.return_type));
                        assert_eq!(staged, ScalarStaged::TypeTest { ok: true }); stages.push("type_true");
                    },
                    RuleIdC::BHelperScalarExit => {
                        assert_eq!(after.result, Some(HelperResultC::ScalarTypeTest { ok: true })); stages.push("type_result");
                    },
                    RuleIdC::KMachineReturnCallValid => {
                        assert_eq!(before.result, Some(HelperResultC::ScalarTypeTest { ok: true }));
                        assert_eq!(after.control, ControlC::Return { value: argument });
                        assert!(after.frames.is_empty()); assert_eq!(after.env_root, None); stages.push("returned");
                    },
                    RuleIdC::KAdminPublishValue => {
                        assert_eq!(after.outcome, OutcomeC::Halted { value: argument }); stages.push("published");
                    },
                    _ => {},
                }
                previous = r.after;
            },
        }
    }
    assert!(terminal, "frozen positive fixture must reach actual terminal within case step_cap");
    assert_eq!(stages, ["pair", "bound", "schedule_return", "return_frame", "body_value", "scalar_expected", "type_true", "type_result", "returned", "published"]);
    println!("{case_id}: owner_events={events}, consumed={}, remaining={}, helper_work={}, helper_visits={}, reference_cursor={}; absolute independent B clocks UNBOUND", previous.image.consumed_work, previous.image.remaining_work, previous.image.helper_work, previous.image.helper_visits, previous.image.reference_cursor);
}

#[test]
fn uint_builtin_identity_uses_real_canonical_carriers() { run_identity("uint_identity"); }
#[test]
fn bool_builtin_identity_uses_real_canonical_carriers() { run_identity("bool_identity"); }
#[test]
fn list_builtin_identity_uses_real_canonical_carriers() { run_identity("list_identity"); }
#[test]
fn tree_builtin_identity_uses_real_canonical_carriers() { run_identity("tree_identity"); }

#[test]
fn nominal_ids_roundtrip_in_all_six_real_storage_roles_only() {
    let oracle: Value = serde_json::from_str(STORAGE).unwrap();
    let ids: Vec<usize> = oracle["nominal_input_ids"].as_array().unwrap().iter()
        .map(|v| v.as_u64().unwrap() as usize).collect();
    assert_eq!(ids, [4, 5, 17]);
    let mut observed = vec![Vec::new(); 6];
    for &n in &ids {
        let id = TypeIdC(n);
        // Actual public carrier types. These illustrative payloads are never
        // admitted, put into a MachineImage, or claimed as nominal runtime state.
        let f = FunctionRowC { function_name: 0, params: SliceC::empty(), return_type: id, body: 0 };
        let p = ParameterRowC { binder: 0, sort: id };
        let t = HelperTaskC::PairTask { binder: 0, sort: id, arg: RefC::Const(0) };
        let c = ControlC::Bound { binding: BoundBinding::Bad, return_type: id, body: 0 };
        let r = FrameRowC::ReturnF { return_type: id, saved_env: None, origin: 0 };
        let h = HelperControlC::Scalar {
            common: HelperCommon { site: SiteTag::RetCall, origin_reference_cursor: 0, parent_resume: None },
            mode: ScalarMode::TestType, operand_a: RefC::Const(0), operand_b: None,
            expected: Some(id), phase: ScalarPhase::First, reg_a: 0, staged: ScalarStaged::None,
        };
        observed[0].push(f.return_type.0); observed[1].push(p.sort.0);
        if let HelperTaskC::PairTask { sort, .. } = t { observed[2].push(sort.0); } else { unreachable!() }
        if let ControlC::Bound { return_type, .. } = c { observed[3].push(return_type.0); } else { unreachable!() }
        if let FrameRowC::ReturnF { return_type, .. } = r { observed[4].push(return_type.0); } else { unreachable!() }
        if let HelperControlC::Scalar { expected: Some(e), .. } = h { observed[5].push(e.0); } else { unreachable!() }
    }
    let roles = oracle["required_existing_carrier_slots"].as_array().unwrap();
    assert_eq!(roles.len(), 6);
    for (i, role) in roles.iter().enumerate() {
        let wanted: Vec<usize> = oracle["per_slot_expected_roundtrip"][role.as_str().unwrap()]
            .as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect();
        assert_eq!(observed[i], wanted);
        for pair in oracle["distinctness_pairs"].as_array().unwrap() {
            let a = pair[0].as_u64().unwrap() as usize; let b = pair[1].as_u64().unwrap() as usize;
            assert_ne!(observed[i][ids.iter().position(|v| *v == a).unwrap()], observed[i][ids.iter().position(|v| *v == b).unwrap()]);
        }
    }
}
