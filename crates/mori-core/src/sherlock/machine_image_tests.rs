//! Source-first normal representation expectations. Snapshot literals and the
//! hand-written acyclic row fixture are independent of either codec and of
//! Machine::step. These tests make no trace-reachability or security claim.

use super::*;

#[path = "extra_oracle_programs.rs"]
mod extra_oracle_programs;

fn limits() -> HostImageLimits {
    // Ordinary specimen allowances only, never selected native capacities.
    HostImageLimits { rows: 2048, cells: 2048, name_bytes: 4096 }
}

fn uint(n: u128) -> ExprRef {
    Arc::new(Expr::Literal(Value::UInt(n)))
}

fn var(name: &str) -> ExprRef {
    Arc::new(Expr::Var(name.into()))
}

fn environment(n: u128) -> Environment {
    Environment::from([("0".into(), Value::UInt(n))])
}

fn branch() -> Value {
    let shared = Arc::new(Tree::Leaf(7));
    Value::Tree(Tree::Branch(shared.clone(), shared))
}

fn normal_arms() -> Vec<Arm> {
    vec![
        Arm { pattern: Pattern::Nil, body: uint(0) },
        Arm { pattern: Pattern::Cons("h".into(), "t".into()), body: var("h") },
        Arm { pattern: Pattern::Leaf("n".into()), body: var("n") },
        Arm { pattern: Pattern::Branch("l".into(), "r".into()), body: var("l") },
        Arm { pattern: Pattern::Var("v".into()), body: var("v") },
    ]
}

fn structural_snapshot(control: Option<Control>, outcome: Outcome) -> Snapshot {
    Snapshot {
        program: Arc::new(Program {
            entry: Arc::new(Expr::Let("0".into(), uint(7), Arc::new(Expr::Binary(
                BinaryOp::Add,
                Arc::new(Expr::Let("0".into(), uint(4), var("0"))),
                var("0"),
            )))),
            // Deliberately retain original duplicate-name function table order.
            functions: vec![
                Function {
                    name: "f".into(),
                    parameters: vec![Parameter { name: "0".into(), ty: Type::UInt }],
                    return_type: Type::UInt,
                    body: var("0"),
                },
                Function {
                    name: "f".into(), parameters: vec![], return_type: Type::Bool,
                    body: Arc::new(Expr::Literal(Value::Bool(true))),
                },
            ],
        }),
        control,
        environment: environment(4),
        frames: vec![
            Frame::Let("0".into(), var("0"), environment(7)),
            Frame::Left(BinaryOp::Add, var("0"), environment(7)),
            Frame::Right(BinaryOp::Add, Value::UInt(4)),
            Frame::Unary(UnaryOp::Not),
            Frame::If(uint(9), uint(4), environment(4)),
            Frame::Match(normal_arms(), environment(7)),
            Frame::Args("f".into(), vec![Value::List(vec![4, 9]), branch()],
                vec![Arc::new(Expr::Call("f".into(), vec![uint(3)])), var("0")], environment(4)),
            Frame::Return(Type::UInt, environment(7)),
            Frame::Scope(environment(4)),
        ],
        stack_limit: 16,
        remaining_fuel: 5,
        initial_fuel: 37,
        outcome,
    }
}

#[test]
fn frozen_n08_boundary75_preserves_complete_caller_frames() {
    // Direct independent N08 boundary75: no evaluation derives this expectation.
    let expected = Snapshot {
        program: Arc::new(extra_oracle_programs::extra_program("N08").unwrap()),
        control: Some(Control::Return(Value::UInt(1))),
        environment: environment(0),
        frames: vec![
            Frame::Return(Type::UInt, environment(1)),
            Frame::Right(BinaryOp::Mul, Value::UInt(1)),
            Frame::Return(Type::UInt, environment(2)),
            Frame::Right(BinaryOp::Mul, Value::UInt(2)),
            Frame::Return(Type::UInt, Environment::new()),
        ],
        stack_limit: 8, remaining_fuel: 7, initial_fuel: 82, outcome: Outcome::Pending,
    };
    let image = encode_snapshot(&expected, limits()).expect("normal specimen fits");
    assert_eq!(image.frame_ids.len(), 5);
    let FrameRow::Return(Type::UInt, caller) = image.frames[image.frame_ids[0].0] else {
        panic!("top frame must retain the n=1 caller");
    };
    let saved = image.environments[caller.0].bindings;
    assert_eq!(saved.len, 1);
    let binding = &image.bindings[saved.start];
    assert_eq!(binding.name, "0");
    assert_eq!(image.values[binding.value.0].payload, ValuePayload::UInt(1));
    assert_eq!(image.remaining_fuel, 7);
    assert_eq!(image.initial_fuel, 82);
    assert_eq!(decode_snapshot(&image), expected);
}

#[test]
fn all_typed_controls_frames_and_outcomes_have_flat_structural_payloads() {
    let function = Function {
        name: "detached".into(), parameters: vec![Parameter { name: "x".into(), ty: Type::Tree }],
        return_type: Type::Tree, body: var("x"),
    };
    let controls = vec![
        None,
        Some(Control::Eval(Arc::new(Expr::If(
            Arc::new(Expr::Unary(UnaryOp::Not, Arc::new(Expr::Literal(Value::Bool(false))))),
            Arc::new(Expr::Match(Arc::new(Expr::Literal(Value::List(vec![4]))), normal_arms())),
            Arc::new(Expr::Literal(branch())),
        )))),
        Some(Control::Return(branch())),
        Some(Control::Push(Frame::Scope(environment(7)), Box::new(Control::Push(
            Frame::Return(Type::Tree, environment(4)), Box::new(Control::Return(branch())),
            environment(3),
        )), environment(2))),
        Some(Control::Invoke("f".into(), vec![Value::UInt(4), Value::List(vec![])])),
        Some(Control::Enter(Some(function), vec![branch()])),
        Some(Control::Enter(None, vec![])),
        Some(Control::Bound(Some(environment(3)), Type::List, var("x"))),
        Some(Control::Bound(None, Type::Bool, uint(0))),
        Some(Control::Cases(branch(), normal_arms(), environment(7))),
        Some(Control::CaseResult(Some(environment(4)), Value::List(vec![4, 9]),
            var("h"), normal_arms(), environment(7))),
        Some(Control::CaseResult(None, Value::Bool(false), uint(1), vec![], environment(4))),
        Some(Control::PrimitiveResult(Ok(branch()))),
        Some(Control::PrimitiveResult(Err(10))),
        Some(Control::Error(3)),
    ];
    for control in controls {
        let expected = structural_snapshot(control, Outcome::Pending);
        let image = encode_snapshot(&expected, limits()).expect("finite typed fixture fits");
        assert_eq!(image.frame_ids.len(), 9);
        assert_eq!(image.program.functions.len, 2);
        assert_eq!(image.functions[0].name, "f");
        assert_eq!(image.functions[0].return_type, Type::UInt);
        assert_eq!(image.functions[1].name, "f");
        assert_eq!(image.functions[1].return_type, Type::Bool);
        assert_eq!(decode_snapshot(&image), expected);
    }
    for outcome in [Outcome::Pending, Outcome::Done(branch()), Outcome::Rejected(6), Outcome::Exhausted] {
        let expected = structural_snapshot(None, outcome);
        let image = encode_snapshot(&expected, limits()).expect("finite typed fixture fits");
        assert_eq!(decode_snapshot(&image), expected);
    }
}

#[test]
fn independent_normal_row_fixture_decodes_lists_and_repeated_tree_occurrences() {
    // Hand-written normal acyclic rows, not produced by encode_snapshot. Sharing
    // here is structural; the expected branch contains two separate leaf terms.
    let image = MachineImageDraft {
        limits: limits(),
        usage: HostImageUsage { rows: 12, cells: 4, name_bytes: 3 },
        program: ProgramRoots { entry: ExprId(0), functions: Slice { start: 0, len: 0 } },
        expressions: vec![ExprRow { payload: ExprPayload::Literal(ValueId(6)), children: Slice { start: 0, len: 0 } }],
        expr_children: vec![], arms: vec![], functions: vec![], parameters: vec![],
        values: vec![
            ValueRow { payload: ValuePayload::UInt(4), fields: Slice { start: 0, len: 0 } },
            ValueRow { payload: ValuePayload::UInt(9), fields: Slice { start: 0, len: 0 } },
            ValueRow { payload: ValuePayload::List, fields: Slice { start: 0, len: 2 } },
            ValueRow { payload: ValuePayload::TreeLeaf(7), fields: Slice { start: 0, len: 0 } },
            ValueRow { payload: ValuePayload::TreeLeaf(7), fields: Slice { start: 0, len: 0 } },
            ValueRow { payload: ValuePayload::TreeBranch, fields: Slice { start: 2, len: 2 } },
            ValueRow { payload: ValuePayload::Bool(true), fields: Slice { start: 0, len: 0 } },
        ],
        value_ids: vec![ValueId(0), ValueId(1), ValueId(3), ValueId(4)],
        environments: vec![EnvironmentRow { bindings: Slice { start: 0, len: 2 } }],
        bindings: vec![BindingRow { name: "xs".into(), value: ValueId(2) }, BindingRow { name: "t".into(), value: ValueId(5) }],
        frames: vec![], frame_ids: vec![],
        controls: vec![ControlRow::Return(ValueId(5))],
        control: Some(ControlId(0)), environment: EnvironmentId(0),
        outcome: OutcomeRow::Pending, stack_limit: 3, remaining_fuel: 7, initial_fuel: 19,
    };
    let tree = Value::Tree(Tree::Branch(Arc::new(Tree::Leaf(7)), Arc::new(Tree::Leaf(7))));
    let expected = Snapshot {
        program: Arc::new(Program { functions: vec![], entry: Arc::new(Expr::Literal(Value::Bool(true))) }),
        control: Some(Control::Return(tree.clone())),
        environment: Environment::from([("xs".into(), Value::List(vec![4, 9])), ("t".into(), tree)]),
        frames: vec![], stack_limit: 3, remaining_fuel: 7, initial_fuel: 19, outcome: Outcome::Pending,
    };
    assert_eq!(decode_snapshot(&image), expected);
}

#[test]
fn independent_encoder_rows_preserve_each_shared_tree_occurrence_and_field_order() {
    let expected = Snapshot {
        program: Arc::new(Program {
            functions: vec![], entry: Arc::new(Expr::Literal(branch())),
        }),
        control: Some(Control::Return(Value::List(vec![4, 9]))),
        environment: Environment::new(), frames: vec![], stack_limit: 8,
        remaining_fuel: 0, initial_fuel: 12, outcome: Outcome::Done(branch()),
    };
    // Independent counts: two 3-row trees + one 3-row list, one expression,
    // one empty environment and one control row; three ordered pairs of fields.
    let budget = HostImageLimits { rows: 12, cells: 6, name_bytes: 0 };
    let image = encode_snapshot(&expected, budget).expect("literal exact allowance");
    assert_eq!(image.limits(), budget);
    assert_eq!(image.usage(), HostImageUsage { rows: 12, cells: 6, name_bytes: 0 });
    assert_eq!(image.values.iter().map(|row| row.payload).collect::<Vec<_>>(), vec![
        ValuePayload::TreeBranch, ValuePayload::TreeLeaf(7), ValuePayload::TreeLeaf(7),
        ValuePayload::List, ValuePayload::UInt(4), ValuePayload::UInt(9),
        ValuePayload::TreeBranch, ValuePayload::TreeLeaf(7), ValuePayload::TreeLeaf(7),
    ]);
    assert_eq!(image.value_ids, vec![ValueId(1), ValueId(2), ValueId(4), ValueId(5), ValueId(7), ValueId(8)]);
    assert_eq!(image.expressions, vec![ExprRow {
        payload: ExprPayload::Literal(ValueId(0)), children: Slice { start: 0, len: 0 },
    }]);
    assert_eq!(image.controls, vec![ControlRow::Return(ValueId(3))]);
    assert_eq!(image.outcome, OutcomeRow::Done(ValueId(6)));
    assert_eq!(decode_snapshot(&image), expected);
}

#[test]
fn normal_encoding_refusal_reports_caller_limits_outside_machine_outcome() {
    let expected = structural_snapshot(Some(Control::Return(Value::UInt(4))), Outcome::Pending);
    for (budget, kind) in [
        (HostImageLimits { rows: 0, ..limits() }, HostLimitKind::Rows),
        (HostImageLimits { cells: 0, ..limits() }, HostLimitKind::Cells),
        (HostImageLimits { name_bytes: 0, ..limits() }, HostLimitKind::NameBytes),
    ] {
        let refusal = encode_snapshot(&expected, budget).err().expect("fixture exceeds selected allowance");
        assert_eq!(refusal.kind, kind);
        assert_eq!(expected.outcome, Outcome::Pending);
        assert_eq!(expected.remaining_fuel, 5);
        assert_eq!(expected.initial_fuel, 37);
    }
}
