//! Independent normal consumer proposal, SOURCE_ONLY/UNRUN.
//! Only public declarations read; full A/B-prefix/clock projection remains OPEN.
use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::shared::*;
use mori_core::sherlock::helper_machine::{complete_observation as co, witness_plan as wp};
use serde_json::{Value, json};
use std::mem::discriminant;
const EXPECTED: &str =
    include_str!("fixtures/witness-baseline-independent/NORMAL-DECODING-EXPECTATIONS.json");
const CONFIG: &str = include_str!("fixtures/witness-baseline-independent/CONFIG.json");
const FIELDS: [&str; 33] = [
    "constants",
    "constant_fields",
    "data",
    "data_fields",
    "environment_cells",
    "current_environment",
    "frames",
    "control",
    "helper",
    "tasks",
    "result",
    "value_slots",
    "outcome",
    "names",
    "expressions",
    "parameters",
    "arms",
    "patterns",
    "occurrences",
    "occurrence_paths",
    "children",
    "functions",
    "pattern_children",
    "types",
    "type_args",
    "constructors",
    "ctor_fields",
    "decl_param_counts",
    "decl_ctor_counts",
    "decl_templates",
    "plan_nodes",
    "plan_cases",
    "field_paths",
];
fn field<'a>(fields: &'a [wp::NamedValue], name: &str) -> &'a wp::SlotValue {
    &fields
        .iter()
        .find(|f| f.field == name)
        .expect("declared required field")
        .value
}
fn sequence(v: &wp::SlotValue) -> &[wp::SlotValue] {
    match v {
        wp::SlotValue::Sequence { items, .. } => items,
        _ => panic!("expected sequence"),
    }
}
fn payload(v: &wp::SlotValue) -> &wp::DecodedPayload {
    match v {
        wp::SlotValue::Payload(p) => p,
        _ => panic!("expected payload"),
    }
}
fn flat(v: &wp::SlotValue) -> Value {
    let v = match v {
        wp::SlotValue::Value(v) => v,
        _ => panic!("expected resolved value"),
    };
    let nodes: Vec<Value> = v
        .nodes
        .iter()
        .map(|n| match n {
            wp::ValueNode::UInt(n) => json!(["UInt", n.to_string()]),
            wp::ValueNode::Bool(b) => json!(["Bool", b]),
            wp::ValueNode::Nil => json!(["Nil"]),
            wp::ValueNode::U128OutOfRange => json!(["U128OutOfRange"]),
            wp::ValueNode::Cons { head, tail } => json!(["Cons", head, tail]),
            wp::ValueNode::Leaf { value } => json!(["Leaf", value]),
            wp::ValueNode::Branch { left, right } => json!(["Branch", left, right]),
            wp::ValueNode::Constructor {
                type_id,
                ctor_id,
                fields,
            } => json!(["Constructor", type_id.0, ctor_id.0, fields]),
        })
        .collect();
    json!({"nodes":nodes,"root":v.root})
}
fn same_type(a: &wp::ValueType, b: &wp::ValueType) -> bool {
    match (a, b) {
        (wp::ValueType::Source(a), wp::ValueType::Source(b)) => discriminant(a) == discriminant(b),
        (wp::ValueType::Payload(a), wp::ValueType::Payload(b)) => {
            discriminant(a) == discriminant(b)
        }
        (wp::ValueType::Sequence(a), wp::ValueType::Sequence(b)) => {
            discriminant(a) == discriminant(b)
        }
        (wp::ValueType::Value, wp::ValueType::Value)
        | (wp::ValueType::Environment, wp::ValueType::Environment)
        | (wp::ValueType::Snapshot, wp::ValueType::Snapshot) => true,
        _ => false,
    }
}
fn path(p: &wp::InputPath) -> (&'static str, Option<usize>) {
    use wp::InputPath::*;
    match p {
        Metadata => ("metadata", None),
        Constant(i) => ("constant", Some(*i)),
        ConstantField(i) => ("constant_field", Some(*i)),
        Data(i) => ("data", Some(*i)),
        DataField(i) => ("data_field", Some(*i)),
        Environment(i) => ("environment", Some(*i)),
        Name(i) => ("name", Some(*i)),
        Expression(i) => ("expression", Some(*i)),
        Function(i) => ("function", Some(*i)),
        Parameter(i) => ("parameter", Some(*i)),
        Arm(i) => ("arm", Some(*i)),
        Pattern(i) => ("pattern", Some(*i)),
        Occurrence(i) => ("occurrence", Some(*i)),
        OccurrencePath(i) => ("occurrence_path", Some(*i)),
        Child(i) => ("child", Some(*i)),
        Frame(i) => ("frame", Some(*i)),
        ScheduledFrame => ("scheduled_frame", None),
        Control => ("control", None),
        Helper => ("helper", None),
        Task(i) => ("task", Some(*i)),
        Result => ("result", None),
        Outcome => ("outcome", None),
        ValueSlot(i) => ("value_slot", Some(*i)),
        PatternChild(i) => ("pattern_child", Some(*i)),
        Type(i) => ("type", Some(*i)),
        TypeArg(i) => ("type_arg", Some(*i)),
        Constructor(i) => ("constructor", Some(*i)),
        ConstructorField(i) => ("constructor_field", Some(*i)),
        DeclParamCount(i) => ("decl_param_count", Some(*i)),
        DeclCtorCount(i) => ("decl_ctor_count", Some(*i)),
        DeclTemplate(i) => ("decl_template", Some(*i)),
        PlanNode(i) => ("plan_node", Some(*i)),
        PlanCase(i) => ("plan_case", Some(*i)),
        FieldPath(i) => ("field_path", Some(*i)),
    }
}
fn check_plan(p: &wp::WitnessPlan) {
    // Structural dataflow obligations; no generated DAG is used as semantic truth.
    let n = p.nodes.len();
    assert!(n > 0);
    assert_eq!(p.stable_topological_order.len(), n);
    let mut positions = vec![usize::MAX; n];
    for (position, id) in p.stable_topological_order.iter().enumerate() {
        assert!(id.0 < n);
        assert_eq!(positions[id.0], usize::MAX);
        positions[id.0] = position;
    }
    let mut node_ids = vec![usize::MAX; n];
    let mut consumers = vec![Vec::new(); n];
    for (index, node) in p.nodes.iter().enumerate() {
        assert!(node.id.0 < n);
        assert_eq!(node_ids[node.id.0], usize::MAX);
        node_ids[node.id.0] = index;
    }
    let mut binding_ids = vec![usize::MAX; p.inputs.len()];
    for (index, b) in p.inputs.iter().enumerate() {
        assert!(b.id.0 < p.inputs.len());
        assert_eq!(binding_ids[b.id.0], usize::MAX);
        binding_ids[b.id.0] = index;
    }
    for id in &p.stable_topological_order {
        let node = &p.nodes[node_ids[id.0]];
        for dep in &node.dependencies {
            assert!(dep.node.0 < n);
            assert!(positions[dep.node.0] < positions[node.id.0]);
            let source = &p.nodes[node_ids[dep.node.0]];
            assert!(same_type(&dep.value_type, &source.output_type));
            consumers[dep.node.0].push(node.id.0); // duplicates and dependency order matter
        }
        if let wp::Operation::Read(binding) = &node.operation {
            assert!(binding.0 < binding_ids.len());
            let input = &p.inputs[binding_ids[binding.0]];
            match &node.output_type {
                wp::ValueType::Source(t) => {
                    assert_eq!(discriminant(t), discriminant(&input.value_type))
                }
                _ => panic!("read source type"),
            }
        }
    }
    assert_eq!(p.definitions.len(), n);
    let mut defined = vec![false; n];
    for d in &p.definitions {
        assert!(d.value.0 < n && d.producer.0 < n);
        assert!(!defined[d.value.0]);
        defined[d.value.0] = true;
        assert_eq!(
            d.consumers.iter().map(|id| id.0).collect::<Vec<_>>(),
            consumers[d.value.0]
        );
        assert_eq!(discriminant(&d.lifetime), discriminant(&p.lifetime));
        assert_eq!(d.live_out, p.live_outs.iter().any(|id| id.0 == d.value.0));
        let last = if d.live_out {
            n
        } else {
            d.consumers
                .last()
                .map(|id| positions[id.0])
                .unwrap_or(positions[d.producer.0])
        };
        assert_eq!(d.last_use_order, last);
    }
    assert!(p.output.0 < n);
    assert!(p.live_outs.iter().any(|id| id.0 == p.output.0));
    let mut ids = p.inputs.iter().map(|b| b.id.0).collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), p.inputs.len());
}
fn check_snapshot(m: &wp::MaterializedSnapshot, expected_lifetime: wp::LifetimeUnit) {
    check_plan(&m.plan);
    assert_eq!(
        discriminant(&m.plan.lifetime),
        discriminant(&expected_lifetime)
    );
    let d = &m.evaluation.decoded;
    let raw = &m.raw;
    assert_eq!(d.fields.iter().map(|f| f.field).collect::<Vec<_>>(), FIELDS);
    // Current dedicated inputs have eleven explicit empty ADT/declaration/plan tables.
    assert_eq!(
        raw.admission.identity.grammar_domain,
        GrammarDomain::DedicatedSorts
    );
    assert!(
        raw.admission.view.pattern_children.is_empty(),
        "dedicated input pattern_children"
    );
    assert!(raw.admission.view.types.is_empty(), "dedicated input types");
    assert!(
        raw.admission.view.type_args.is_empty(),
        "dedicated input type_args"
    );
    assert!(
        raw.admission.view.constructors.is_empty(),
        "dedicated input constructors"
    );
    assert!(
        raw.admission.view.ctor_fields.is_empty(),
        "dedicated input ctor_fields"
    );
    assert!(
        raw.admission.view.decl_param_counts.is_empty(),
        "dedicated input decl_param_counts"
    );
    assert!(
        raw.admission.view.decl_ctor_counts.is_empty(),
        "dedicated input decl_ctor_counts"
    );
    assert!(
        raw.admission.view.decl_templates.is_empty(),
        "dedicated input decl_templates"
    );
    assert!(
        raw.admission.view.plan_nodes.is_empty(),
        "dedicated input plan_nodes"
    );
    assert!(
        raw.admission.view.plan_cases.is_empty(),
        "dedicated input plan_cases"
    );
    assert!(
        raw.admission.view.field_paths.is_empty(),
        "dedicated input field_paths"
    );
    let functions = match field(&d.fields, "functions") {
        wp::SlotValue::Sequence {
            kind: wp::SequenceKind::Functions,
            items,
        } => items,
        _ => panic!("typed current functions output"),
    };
    assert_eq!(functions.len(), raw.admission.view.functions.len());
    for (decoded, original) in functions.iter().zip(&raw.admission.view.functions) {
        match decoded {
            wp::SlotValue::Source(wp::InputValue::Function(row)) => assert_eq!(row, original),
            _ => panic!("complete ordered FunctionRowC output"),
        }
    }
    for (name, expected_kind) in [
        ("pattern_children", wp::SequenceKind::Indices),
        ("types", wp::SequenceKind::Types),
        ("type_args", wp::SequenceKind::TypeIds),
        ("constructors", wp::SequenceKind::Constructors),
        ("ctor_fields", wp::SequenceKind::ConstructorFields),
        ("decl_param_counts", wp::SequenceKind::Indices),
        ("decl_ctor_counts", wp::SequenceKind::Indices),
        ("decl_templates", wp::SequenceKind::DeclTemplates),
        ("plan_nodes", wp::SequenceKind::PlanNodes),
        ("plan_cases", wp::SequenceKind::PlanCases),
        ("field_paths", wp::SequenceKind::Indices),
    ] {
        match field(&d.fields, name) {
            wp::SlotValue::Sequence { kind, items } => {
                assert_eq!(*kind, expected_kind, "typed {name} output");
                assert!(items.is_empty(), "dedicated decoded {name}");
            }
            _ => panic!("present typed {name} output"),
        }
    }
    assert_eq!(d.metadata.admission, raw.admission);
    assert_eq!(d.metadata.run_id, raw.image.run_id);
    assert_eq!(d.metadata.control_origin, raw.image.control_origin);
    assert_eq!(d.metadata.env_root, raw.image.env_root);
    assert_eq!(d.metadata.stack_limit, raw.image.stack_limit);
    assert_eq!(d.metadata.initial_fuel, raw.image.initial_fuel);
    assert_eq!(d.metadata.remaining_work, raw.image.remaining_work);
    assert_eq!(d.metadata.consumed_work, raw.image.consumed_work);
    assert_eq!(d.metadata.helper_work, raw.image.helper_work);
    assert_eq!(d.metadata.helper_visits, raw.image.helper_visits);
    assert_eq!(d.metadata.reference_cursor, raw.image.reference_cursor);
    assert_eq!(d.metadata.alloc, raw.image.alloc);
    assert_eq!(d.metadata.active, raw.image.active);
    assert_eq!(d.metadata.seg_base, raw.image.seg_base);
    assert_eq!(
        d.metadata.table_lengths,
        [
            raw.image.data.len(),
            raw.image.data_fields.len(),
            raw.image.env.len(),
            raw.image.frames.len(),
            raw.image.tasks.len(),
            raw.image.value_slots.len()
        ]
    );
    let frames = sequence(field(&d.fields, "frames"));
    assert_eq!(frames.len(), raw.image.frames.len());
    for (decoded, original) in frames.iter().zip(raw.image.frames.iter().rev()) {
        match &payload(decoded).raw {
            wp::InputValue::Frame(frame) => assert_eq!(frame, original),
            _ => panic!("exact raw frame"),
        }
    }
    for (name, original) in [
        ("control", wp::InputValue::Control(raw.image.control)),
        ("helper", wp::InputValue::Helper(raw.image.helper)),
        ("result", wp::InputValue::Result(raw.image.result)),
        ("outcome", wp::InputValue::Outcome(raw.image.outcome)),
    ] {
        match (&payload(field(&d.fields, name)).raw, original) {
            (wp::InputValue::Control(a), wp::InputValue::Control(b)) => assert_eq!(*a, b),
            (wp::InputValue::Helper(a), wp::InputValue::Helper(b)) => assert_eq!(*a, b),
            (wp::InputValue::Result(a), wp::InputValue::Result(b)) => assert_eq!(*a, b),
            (wp::InputValue::Outcome(a), wp::InputValue::Outcome(b)) => assert_eq!(*a, b),
            _ => panic!("exact raw payload"),
        }
    }
    // These Options cannot be relabelled as physical bytes/time or A semantic work.
    let _separate_diagnostics = (
        &m.evaluation.diagnostics.measured_host_bytes,
        &m.evaluation.diagnostics.measured_host_time_ns,
    );
}
fn var(n: &str) -> CoreExpr {
    CoreExpr::Var { name: n.into() }
}
fn uint(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: KIntC::InRange(n),
        },
    }
}
fn bin(op: BinaryOp, left: CoreExpr, right: CoreExpr) -> CoreExpr {
    CoreExpr::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}
fn call(n: &str, arg: CoreExpr) -> CoreExpr {
    CoreExpr::Call {
        name: n.into(),
        args: vec![arg],
    }
}
fn program(case: &str) -> CoreProgram {
    // Literal AST construction only; no alternate evaluator/reference interpreter.
    let (name, sort, body, arg) = match case {
        "factorial3" => (
            "18",
            SortTag::UInt,
            CoreExpr::If {
                cond: Box::new(bin(BinaryOp::Eq, var("0"), uint(0))),
                then_expr: Box::new(uint(1)),
                else_expr: Box::new(bin(
                    BinaryOp::Mul,
                    var("0"),
                    call("18", bin(BinaryOp::Sub, var("0"), uint(1))),
                )),
            },
            uint(3),
        ),
        "recursive_list" => (
            "3",
            SortTag::List,
            CoreExpr::Match {
                scrutinee: Box::new(var("0")),
                arms: vec![
                    CoreArm {
                        pattern: CorePattern::PNil,
                        body: uint(0),
                    },
                    CoreArm {
                        pattern: CorePattern::PCons {
                            x: "1".into(),
                            y: "2".into(),
                        },
                        body: bin(BinaryOp::Add, var("1"), call("3", var("2"))),
                    },
                ],
            },
            CoreExpr::Literal {
                value: CoreValue::List {
                    elements: vec![KIntC::InRange(2), KIntC::InRange(5)],
                },
            },
        ),
        "recursive_tree" => (
            "4",
            SortTag::Tree,
            CoreExpr::Match {
                scrutinee: Box::new(var("0")),
                arms: vec![
                    CoreArm {
                        pattern: CorePattern::PLeaf { x: "1".into() },
                        body: var("1"),
                    },
                    CoreArm {
                        pattern: CorePattern::PBranch {
                            x: "1".into(),
                            y: "2".into(),
                        },
                        body: bin(BinaryOp::Add, call("4", var("1")), call("4", var("2"))),
                    },
                ],
            },
            CoreExpr::Literal {
                value: CoreValue::Tree {
                    tree: CoreTree::Branch {
                        left: Box::new(CoreTree::Leaf {
                            n: KIntC::InRange(4),
                        }),
                        right: Box::new(CoreTree::Branch {
                            left: Box::new(CoreTree::Leaf {
                                n: KIntC::InRange(1),
                            }),
                            right: Box::new(CoreTree::Leaf {
                                n: KIntC::InRange(3),
                            }),
                        }),
                    },
                },
            },
        ),
        _ => panic!("fixed ordinary fixture"),
    };
    CoreProgram {
        functions: vec![CoreFunction {
            name: name.into(),
            parameters: vec![CoreParameter {
                name: "0".into(),
                sort,
            }],
            return_type: SortTag::UInt,
            body,
        }],
        entry: call(name, arg),
    }
}
fn config() -> (BoundsC, LayoutSchemaC) {
    let c: Value = serde_json::from_str(CONFIG).unwrap();
    let b = &c["bounds"];
    let n = |name: &str| b[name].as_str().unwrap().parse::<usize>().unwrap();
    let bounds = BoundsC {
        code_rows: n("code_rows"),
        function_rows: n("function_rows"),
        code_child_slots: n("code_child_slots"),
        parameter_slots: n("parameter_slots"),
        pattern_rows: n("pattern_rows"),
        match_plan_rows: n("match_plan_rows"),
        match_arm_rows: n("match_arm_rows"),
        type_rows: n("type_rows"),
        constructor_rows: n("constructor_rows"),
        constructor_field_slots: n("constructor_field_slots"),
        max_arity: n("max_arity"),
        constant_nodes: n("constant_nodes"),
        constant_field_slots: n("constant_field_slots"),
        data_nodes: n("data_nodes"),
        data_field_slots: n("data_field_slots"),
        environment_cells: n("environment_cells"),
        frames: n("frames"),
        helper_tasks: n("helper_tasks"),
        serialization_bytes: n("serialization_bytes"),
        max_reference_work: n("max_reference_work") as u128,
        max_helper_work: n("max_helper_work") as u128,
        step_cap: n("step_cap") as u128,
    };
    let cap = &c["layout"]["capacities"];
    let k = |name: &str| cap[name].as_str().unwrap().parse::<usize>().unwrap();
    let layout = LayoutSchemaC {
        layout_version: (
            c["layout"]["layout_version"][0].as_str().unwrap().into(),
            c["layout"]["layout_version"][1].as_str().unwrap().into(),
        ),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds,
        capacities: LayoutCapacitiesC {
            name_rows: k("name_rows"),
            name_bytes: k("name_bytes"),
            occurrence_path_slots: k("occurrence_path_slots"),
            value_slots: k("value_slots"),
        },
        ext_capacities: None,
        boundary_capacities_present: false,
    };
    (bounds, layout)
}
fn pair(p: &CoreProgram) -> (Admitted, MachineImage) {
    let (b, l) = config();
    let owner = host_owner_engine();
    let a = owner
        .admit(p, &b, &l)
        .unwrap_or_else(|_| panic!("fixed positive admission"));
    let image = owner
        .admit_run(
            &a,
            &RunIntC::Nat(4096),
            &RunIntC::Nat(64),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|_| panic!("fixed positive run admission"));
    (a, image)
}
fn expected<'a>(e: &'a Value, id: &str) -> &'a Value {
    &e["postorder_values"][id]
}
fn input_value<'a>(s: &'a wp::DecodedSnapshot, raw: &co::HostSnapshot) -> &'a wp::SlotValue {
    let entry = match raw.admission.view.expressions[raw.admission.view.entry] {
        co::ExpressionSnapshot::Call { args, .. } => args,
        _ => panic!("literal fixture call"),
    };
    let expr = raw.admission.view.children[entry.start];
    let constant = match raw.admission.view.expressions[expr] {
        co::ExpressionSnapshot::Literal { constant } => constant,
        _ => panic!("literal fixture input"),
    };
    &sequence(field(&s.fields, "constants"))[constant]
}
#[test]
fn actual_owner_records_materialize_all_four_normal_domains() {
    let e: Value = serde_json::from_str(EXPECTED).unwrap();
    let mut saw_pending = false;
    for (case, input, result) in [
        ("factorial3", "u3", "u6"),
        ("recursive_list", "l25", "u7"),
        ("recursive_tree", "tr", "u8"),
    ] {
        let p = program(case);
        let (a, mut image) = pair(&p);
        let initial = co::capture(&a, &image).unwrap_or_else(|_| panic!("real capture"));
        let first = wp::materialize(&initial)
            .unwrap_or_else(|_| panic!("materialize real initial snapshot"));
        check_snapshot(&first, wp::LifetimeUnit::Snapshot);
        assert_eq!(first.raw, initial);
        assert_eq!(
            flat(input_value(&first.evaluation.decoded, &initial)),
            *expected(&e, input)
        );
        loop {
            let step = co::record_step_with_plan(&a, &mut image)
                .unwrap_or_else(|_| panic!("actual owner/planned record"));
            let record = match &step.observed {
                co::ObservedStep::Event(r) => r,
                co::ObservedStep::Terminal { .. } => panic!("positive terminal before publication"),
            };
            assert_eq!(step.before.raw, record.before);
            assert_eq!(step.after.raw, record.after);
            check_snapshot(&step.before, wp::LifetimeUnit::RecordBefore);
            check_snapshot(&step.after, wp::LifetimeUnit::RecordAfter);
            let d = &step.before.evaluation.decoded;
            let frames = sequence(field(&d.fields, "frames"));
            if case == "factorial3" && frames.len() == 2 {
                let f0 = payload(&frames[0]);
                let f1 = payload(&frames[1]);
                if let (
                    wp::InputValue::Frame(FrameRowC::RightF {
                        op: BinaryOp::Mul, ..
                    }),
                    wp::InputValue::Frame(FrameRowC::ReturnF {
                        saved_env: None, ..
                    }),
                ) = (&f0.raw, &f1.raw)
                {
                    if flat(field(&f0.fields, "left")) == *expected(&e, "u3") {
                        if let ControlC::Eval { expr } = record.before.image.control {
                            if matches!(
                                record.before.admission.view.expressions[expr],
                                co::ExpressionSnapshot::Call { .. }
                            ) && record.before.image.helper.is_none()
                            {
                                let env = match field(&d.fields, "current_environment") {
                                    wp::SlotValue::Environment(env) => env,
                                    _ => panic!("current environment"),
                                };
                                assert_eq!(env.len(), 1);
                                assert_eq!(env[0].name, "0");
                                // Compare a public decoded value, not a guessed B cell index.
                                let value = wp::SlotValue::Value(wp::DecodedValue {
                                    nodes: env[0]
                                        .value
                                        .nodes
                                        .iter()
                                        .map(|n| match n {
                                            wp::ValueNode::UInt(n) => wp::ValueNode::UInt(*n),
                                            _ => panic!("n is UInt"),
                                        })
                                        .collect(),
                                    root: env[0].value.root,
                                });
                                assert_eq!(flat(&value), *expected(&e, "u3"));
                                saw_pending = true;
                            }
                        }
                    }
                }
            }
            match record.after.image.outcome {
                OutcomeC::Running => {}
                OutcomeC::Halted { .. } => {
                    let out = payload(field(&step.after.evaluation.decoded.fields, "outcome"));
                    assert_eq!(flat(field(&out.fields, "value")), *expected(&e, result));
                    break;
                }
                _ => panic!("fixed positive source failed"),
            }
        }
    }
    assert!(
        saw_pending,
        "actual pending caller3 multiplication survives decoding"
    );
    // No A prefix index/reference-work total is compared with a B step or clock.
}
#[test]
fn ordinary_shared_value_and_shadowed_environment_decode_exactly() {
    let e: Value = serde_json::from_str(EXPECTED).unwrap();
    let p = CoreProgram {
        functions: vec![],
        entry: uint(3),
    };
    let (a, image) = pair(&p);
    let mut raw = co::capture(&a, &image).unwrap_or_else(|_| panic!("real base capture"));
    // Declared diagnostic decoder inputs, not claimed authentic owner transitions.
    // This modified raw snapshot is never passed to the owner or stepped.
    raw.admission.view.names = vec!["n".into(), "x".into()];
    raw.admission.view.constants = vec![
        co::ConstantSnapshot::U128 {
            marker_free: true,
            n: 3,
        },
        co::ConstantSnapshot::U128 {
            marker_free: true,
            n: 1,
        },
        co::ConstantSnapshot::U128 {
            marker_free: true,
            n: 4,
        },
        co::ConstantSnapshot::Leaf {
            marker_free: true,
            fields: co::ConstFieldsSnapshot { start: 0, len: 1 },
        },
        co::ConstantSnapshot::Branch {
            marker_free: true,
            fields: co::ConstFieldsSnapshot { start: 1, len: 2 },
        },
    ];
    raw.admission.view.constant_fields = vec![2, 3, 3];
    raw.image.data = vec![co::DataSnapshot::U128 { n: 2 }];
    raw.image.env = vec![
        EnvCellC {
            binder: 0,
            value: RefC::Const(0),
            previous: None,
        },
        EnvCellC {
            binder: 1,
            value: RefC::Data(0),
            previous: Some(0),
        },
        EnvCellC {
            binder: 0,
            value: RefC::Const(1),
            previous: Some(1),
        },
    ];
    raw.image.env_root = Some(2);
    raw.image.active.data = 1;
    raw.image.active.env = 3;
    raw.image.alloc.data_nodes = 1;
    raw.image.alloc.environment_cells = 3;
    let plan = wp::construct_plan_for(&raw, wp::LifetimeUnit::Snapshot)
        .unwrap_or_else(|_| panic!("normal diagnostic plan"));
    check_plan(&plan);
    let evaluated = wp::evaluate_uncached(&plan, &raw)
        .unwrap_or_else(|_| panic!("normal diagnostic evaluation"));
    let materialized = wp::materialize_for(&raw, wp::LifetimeUnit::Snapshot)
        .unwrap_or_else(|_| panic!("normal diagnostic materialization"));
    check_snapshot(&materialized, wp::LifetimeUnit::Snapshot);
    assert_eq!(materialized.raw, raw);
    assert_eq!(
        plan.stable_topological_order
            .iter()
            .map(|id| id.0)
            .collect::<Vec<_>>(),
        materialized
            .plan
            .stable_topological_order
            .iter()
            .map(|id| id.0)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        plan.inputs
            .iter()
            .map(|b| (b.id.0, path(&b.path)))
            .collect::<Vec<_>>(),
        materialized
            .plan
            .inputs
            .iter()
            .map(|b| (b.id.0, path(&b.path)))
            .collect::<Vec<_>>()
    );
    for d in [&evaluated.decoded, &materialized.evaluation.decoded] {
        assert_eq!(
            flat(&sequence(field(&d.fields, "constants"))[4]),
            e["repeated_shared_leaf"]["expected"]
        );
        let env = match field(&d.fields, "current_environment") {
            wp::SlotValue::Environment(env) => env,
            _ => panic!("environment"),
        };
        assert_eq!(env.len(), 3);
        for (entry, expected) in env.iter().zip(
            e["ordered_shadowed_environment_slice"]["expected_newest_first_entries"]
                .as_array()
                .unwrap(),
        ) {
            assert_eq!(entry.binder, expected["binder"].as_u64().unwrap() as usize);
            assert_eq!(entry.name, expected["name"].as_str().unwrap());
            let n = match entry.value.nodes.as_slice() {
                [wp::ValueNode::UInt(n)] => *n,
                _ => panic!("normal scalar binding"),
            };
            assert_eq!(entry.value.root, 0);
            assert_eq!(
                json!({"nodes":[["UInt",n.to_string()]],"root":0}),
                *self::expected(&e, expected["value"].as_str().unwrap())
            );
        }
    }
}
