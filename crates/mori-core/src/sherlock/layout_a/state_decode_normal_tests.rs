//! Normal typed decoder examples, frozen before decoder implementation. UNRUN.
//! Images are supplied by hand; expected Snapshots do not use the encoder.
use super::{prepare::prepare_a, rows::*, state::*, state_decode::decode_observed_a};
use crate::sherlock::{
    Arm, BinaryOp, Control, Environment, Expr, Frame, Function, Outcome, PROFILE, Pattern, Program,
    Type, UnaryOp, Value,
};
use std::sync::Arc;

fn fixture() -> (PreparedA, StateCapacities, StateBlock) {
    let seven = Arc::new(Expr::Literal(Value::UInt(7)));
    let nine = Arc::new(Expr::Literal(Value::UInt(9)));
    let body = Arc::new(Expr::Match(
        seven.clone(),
        vec![
            Arm {
                pattern: Pattern::Var("x".into()),
                body: nine.clone(),
            },
            Arm {
                pattern: Pattern::Nil,
                body: seven.clone(),
            },
        ],
    ));
    let p = Arc::new(Program {
        entry: Arc::new(Expr::Call("f".into(), vec![seven, nine])),
        functions: vec![Function {
            name: "f".into(),
            parameters: vec![],
            return_type: Type::UInt,
            body,
        }],
    });
    let prepared = prepare_a(
        p,
        Capacities {
            tables: [64; 15],
            data: 16,
        },
    )
    .unwrap();
    let cap = StateCapacities {
        data: 16,
        data_fields: 32,
        env_cells: 16,
        value_slots: 16,
        frame_slots: 9,
        control_rows: 2,
        stack_limit: 12,
    };
    let mut state = StateBlock {
        root: StateRoot {
            control_length: 1,
            stack_limit: 12,
            remaining_fuel: 5u128.to_le_bytes(),
            initial_fuel: 10u128.to_le_bytes(),
            ..StateRoot::default()
        },
        data: vec![DataNode::default(); 16],
        data_fields: vec![ValueRef::default(); 32],
        env_cells: vec![EnvCell::default(); 16],
        value_slots: vec![ValueRef::default(); 16],
        frame_slots: vec![FrameRow::default(); 9],
        control_rows: vec![ControlRow::default(); 2],
    };
    state.control_rows[0] = ControlRow {
        tag: 0,
        expr_id: 0,
        ..ControlRow::default()
    };
    (prepared, cap, state)
}
fn decode(
    p: &PreparedA,
    c: &StateCapacities,
    s: &StateBlock,
    mode: DecodeMode,
) -> crate::sherlock::Snapshot {
    decode_observed_a(
        p,
        c,
        &AImage {
            profile: PROFILE,
            full_program: p.full_program.as_ref(),
            program: &p.block,
            state: s,
        },
        mode,
    )
    .unwrap()
}
fn data(i: usize) -> ValueRef {
    ValueRef {
        space: true,
        index: i,
    }
}
fn values(s: &mut StateBlock) {
    s.root.active_data = 3;
    s.data[0] = DataNode {
        kind: 0,
        type_id: 0,
        uint_bytes: 7u128.to_le_bytes(),
        ..DataNode::default()
    };
    s.data[1] = DataNode {
        kind: 1,
        type_id: 1,
        bool_value: false,
        ..DataNode::default()
    };
    s.data[2] = DataNode {
        kind: 2,
        type_id: 2,
        ctor_id: 0,
        ..DataNode::default()
    };
}

#[test]
fn supplied_initial_state_keeps_all_eight_fields_in_both_modes() {
    let (p, c, s) = fixture();
    for mode in [DecodeMode::Step, DecodeMode::Canonical] {
        let got = decode(&p, &c, &s, mode);
        assert_eq!(got.program.as_ref(), p.full_program.as_ref());
        assert_eq!(
            got.control,
            Some(Control::Eval(p.full_program.entry.clone()))
        );
        assert!(got.environment.is_empty());
        assert!(got.frames.is_empty());
        assert_eq!(got.stack_limit, 12);
        assert_eq!(got.remaining_fuel, 5);
        assert_eq!(got.initial_fuel, 10);
        assert_eq!(got.outcome, Outcome::Pending);
    }
}

#[test]
fn all_nine_frame_payloads_and_saved_shadowed_environment_survive() {
    let (p, c, mut s) = fixture();
    values(&mut s);
    let x = p.names.iter().position(|n| n == "x").unwrap();
    s.root.active_env_cells = 2;
    s.env_cells[0] = EnvCell {
        name_id: x,
        value: data(0),
        parent: EnvRoot::default(),
    };
    s.env_cells[1] = EnvCell {
        name_id: x,
        value: data(1),
        parent: EnvRoot {
            present: true,
            index: 0,
        },
    };
    s.root.environment = EnvRoot {
        present: true,
        index: 1,
    };
    let saved = EnvRoot {
        present: true,
        index: 0,
    };
    let body = p.block.functions.rows[0].body_expr_id;
    let arms = p.block.expressions.rows[body].arms;
    let args = p.block.expressions.rows[0].children;
    s.root.active_value_slots = 2;
    s.value_slots[0] = data(1);
    s.value_slots[1] = data(2);
    s.root.depth = 9;
    s.frame_slots = vec![
        FrameRow {
            tag: 0,
            name_id: x,
            expr0: 1,
            saved_env: saved,
            ..FrameRow::default()
        },
        FrameRow {
            tag: 1,
            binary_op: 2,
            expr0: 2,
            saved_env: saved,
            ..FrameRow::default()
        },
        FrameRow {
            tag: 2,
            binary_op: 0,
            value: data(0),
            ..FrameRow::default()
        },
        FrameRow {
            tag: 3,
            unary_op: 1,
            ..FrameRow::default()
        },
        FrameRow {
            tag: 4,
            expr0: 1,
            expr1: 2,
            saved_env: saved,
            ..FrameRow::default()
        },
        FrameRow {
            tag: 5,
            arms,
            saved_env: saved,
            ..FrameRow::default()
        },
        FrameRow {
            tag: 6,
            name_id: 0,
            completed: Slice { start: 0, len: 2 },
            remaining_exprs: args,
            saved_env: saved,
            ..FrameRow::default()
        },
        FrameRow {
            tag: 7,
            type_id: 3,
            saved_env: saved,
            ..FrameRow::default()
        },
        FrameRow {
            tag: 8,
            saved_env: saved,
            ..FrameRow::default()
        },
    ];
    let got = decode(&p, &c, &s, DecodeMode::Step);
    let env = Environment::from([("x".into(), Value::UInt(7))]);
    let Expr::Call(_, children) = p.full_program.entry.as_ref() else {
        panic!("fixture")
    };
    let Expr::Match(_, expected_arms) = p.full_program.functions[0].body.as_ref() else {
        panic!("fixture")
    };
    assert_eq!(
        got.environment,
        Environment::from([("x".into(), Value::Bool(false))])
    );
    assert_eq!(
        got.frames,
        vec![
            Frame::Let("x".into(), children[0].clone(), env.clone()),
            Frame::Left(BinaryOp::Mul, children[1].clone(), env.clone()),
            Frame::Right(BinaryOp::Add, Value::UInt(7)),
            Frame::Unary(UnaryOp::Not),
            Frame::If(children[0].clone(), children[1].clone(), env.clone()),
            Frame::Match(expected_arms.clone(), env.clone()),
            Frame::Args(
                "f".into(),
                vec![Value::Bool(false), Value::List(vec![])],
                children.clone(),
                env.clone()
            ),
            Frame::Return(Type::Tree, env.clone()),
            Frame::Scope(env)
        ]
    );
}

#[test]
fn ten_controls_keep_option_zero_row_and_empty_binding_distinctions() {
    let (p, c, mut s) = fixture();
    values(&mut s);
    let empty = Environment::new();
    let expr = p.full_program.entry.clone();
    let mut cases = vec![
        (
            ControlRow {
                tag: 0,
                expr_id: 0,
                ..ControlRow::default()
            },
            Control::Eval(expr.clone()),
        ),
        (
            ControlRow {
                tag: 1,
                value: data(1),
                ..ControlRow::default()
            },
            Control::Return(Value::Bool(false)),
        ),
        (
            ControlRow {
                tag: 3,
                name_id: 0,
                ..ControlRow::default()
            },
            Control::Invoke("f".into(), vec![]),
        ),
        (
            ControlRow {
                tag: 4,
                function_present: true,
                function_id: 0,
                ..ControlRow::default()
            },
            Control::Enter(Some(p.full_program.functions[0].clone()), vec![]),
        ),
        (
            ControlRow {
                tag: 4,
                ..ControlRow::default()
            },
            Control::Enter(None, vec![]),
        ),
        (
            ControlRow {
                tag: 5,
                bindings_present: true,
                expr_id: 0,
                type_id: 0,
                ..ControlRow::default()
            },
            Control::Bound(Some(empty.clone()), Type::UInt, expr.clone()),
        ),
        (
            ControlRow {
                tag: 5,
                expr_id: 0,
                type_id: 0,
                ..ControlRow::default()
            },
            Control::Bound(None, Type::UInt, expr.clone()),
        ),
        (
            ControlRow {
                tag: 6,
                value: data(2),
                ..ControlRow::default()
            },
            Control::Cases(Value::List(vec![]), vec![], empty.clone()),
        ),
        (
            ControlRow {
                tag: 7,
                bindings_present: true,
                value: data(0),
                expr_id: 0,
                ..ControlRow::default()
            },
            Control::CaseResult(
                Some(empty.clone()),
                Value::UInt(7),
                expr.clone(),
                vec![],
                empty.clone(),
            ),
        ),
        (
            ControlRow {
                tag: 7,
                value: data(0),
                expr_id: 0,
                ..ControlRow::default()
            },
            Control::CaseResult(None, Value::UInt(7), expr.clone(), vec![], empty.clone()),
        ),
        (
            ControlRow {
                tag: 8,
                primitive_is_ok: true,
                value: data(0),
                ..ControlRow::default()
            },
            Control::PrimitiveResult(Ok(Value::UInt(7))),
        ),
        (
            ControlRow {
                tag: 8,
                error_code: 255,
                ..ControlRow::default()
            },
            Control::PrimitiveResult(Err(255)),
        ),
        (
            ControlRow {
                tag: 9,
                error_code: 0,
                ..ControlRow::default()
            },
            Control::Error(0),
        ),
    ];
    for (row, want) in cases.drain(..) {
        s.control_rows[0] = row;
        assert_eq!(decode(&p, &c, &s, DecodeMode::Step).control, Some(want));
    }
    s.root.control_length = 2;
    s.control_rows[0] = ControlRow {
        tag: 2,
        pending_frame: FrameRow {
            tag: 8,
            ..FrameRow::default()
        },
        ..ControlRow::default()
    };
    s.control_rows[1] = ControlRow {
        tag: 0,
        ..ControlRow::default()
    };
    assert_eq!(
        decode(&p, &c, &s, DecodeMode::Step).control,
        Some(Control::Push(
            Frame::Scope(empty.clone()),
            Box::new(Control::Eval(expr)),
            empty
        ))
    );
}

#[test]
fn constant_space_values_and_all_error_payloads_are_exact_normal_data() {
    let (p, c, mut s) = fixture();
    let constant = p.block.expressions.rows[1].value;
    s.control_rows[0] = ControlRow {
        tag: 1,
        value: constant,
        ..ControlRow::default()
    };
    assert_eq!(
        decode(&p, &c, &s, DecodeMode::Step).control,
        Some(Control::Return(Value::UInt(7)))
    );
    for code in 0..=255u8 {
        s.control_rows[0] = ControlRow {
            tag: 9,
            error_code: code,
            ..ControlRow::default()
        };
        assert_eq!(
            decode(&p, &c, &s, DecodeMode::Step).control,
            Some(Control::Error(code))
        );
        s.control_rows[0] = ControlRow {
            tag: 8,
            error_code: code,
            ..ControlRow::default()
        };
        assert_eq!(
            decode(&p, &c, &s, DecodeMode::Step).control,
            Some(Control::PrimitiveResult(Err(code)))
        );
        s.root.control_length = 0;
        s.control_rows[0] = ControlRow::default();
        s.root.outcome_tag = 2;
        s.root.outcome_error = code;
        assert_eq!(
            decode(&p, &c, &s, DecodeMode::Step).outcome,
            Outcome::Rejected(code)
        );
        s.root.control_length = 1;
        s.root.outcome_tag = 0;
        s.root.outcome_error = 0;
    }
    s.root.control_length = 0;
    s.control_rows[0] = ControlRow::default();
    s.root.outcome_tag = 3;
    assert_eq!(
        decode(&p, &c, &s, DecodeMode::Step).outcome,
        Outcome::Exhausted
    );
    s.root.outcome_tag = 1;
    s.root.outcome_value = constant;
    assert_eq!(
        decode(&p, &c, &s, DecodeMode::Step).outcome,
        Outcome::Done(Value::UInt(7))
    );
}

#[test]
fn shared_tree_dag_and_ordered_list_decode_without_canonical_unfolding() {
    let (p, c, mut s) = fixture();
    s.root.active_data = 8;
    s.root.active_data_fields = 9;
    s.root.active_value_slots = 2;
    s.data[0] = DataNode {
        uint_bytes: 7u128.to_le_bytes(),
        ..DataNode::default()
    };
    s.data[1] = DataNode {
        uint_bytes: 9u128.to_le_bytes(),
        ..DataNode::default()
    };
    s.data[2] = DataNode {
        kind: 2,
        type_id: 2,
        ..DataNode::default()
    };
    s.data[3] = DataNode {
        kind: 2,
        type_id: 2,
        ctor_id: 1,
        fields: Slice { start: 0, len: 2 },
        ..DataNode::default()
    };
    s.data[4] = DataNode {
        kind: 2,
        type_id: 2,
        ctor_id: 1,
        fields: Slice { start: 2, len: 2 },
        ..DataNode::default()
    };
    s.data[5] = DataNode {
        kind: 2,
        type_id: 3,
        ctor_id: 2,
        fields: Slice { start: 4, len: 1 },
        ..DataNode::default()
    };
    s.data[6] = DataNode {
        kind: 2,
        type_id: 3,
        ctor_id: 3,
        fields: Slice { start: 5, len: 2 },
        ..DataNode::default()
    };
    s.data[7] = DataNode {
        kind: 2,
        type_id: 3,
        ctor_id: 3,
        fields: Slice { start: 7, len: 2 },
        ..DataNode::default()
    };
    for (i, r) in [
        data(1),
        data(2),
        data(0),
        data(3),
        data(0),
        data(5),
        data(5),
        data(6),
        data(6),
    ]
    .into_iter()
    .enumerate()
    {
        s.data_fields[i] = r;
    }
    s.value_slots[0] = data(4);
    s.value_slots[1] = data(7);
    s.control_rows[0] = ControlRow {
        tag: 3,
        name_id: 0,
        values: Slice { start: 0, len: 2 },
        ..ControlRow::default()
    };
    let leaf = Arc::new(crate::sherlock::Tree::Leaf(7));
    let pair = Arc::new(crate::sherlock::Tree::Branch(leaf.clone(), leaf));
    let tree = crate::sherlock::Tree::Branch(pair.clone(), pair);
    assert_eq!(
        decode(&p, &c, &s, DecodeMode::Step).control,
        Some(Control::Invoke(
            "f".into(),
            vec![Value::List(vec![7, 9]), Value::Tree(tree)]
        ))
    );
    // Distinct typed zero values remain data, not optional absence.
    values(&mut s);
    s.data[0].uint_bytes = 0u128.to_le_bytes();
    s.root.active_data_fields = 0;
    for f in &mut s.data_fields {
        *f = ValueRef::default();
    }
    for n in s.data.iter_mut().skip(3) {
        *n = DataNode::default();
    }
    s.root.active_value_slots = 3;
    s.value_slots[0] = data(0);
    s.value_slots[1] = data(1);
    s.value_slots[2] = data(2);
    s.control_rows[0].values = Slice { start: 0, len: 3 };
    assert_eq!(
        decode(&p, &c, &s, DecodeMode::Step).control,
        Some(Control::Invoke(
            "f".into(),
            vec![Value::UInt(0), Value::Bool(false), Value::List(vec![])]
        ))
    );
}
