//! Owner aux/2 normal specimens through actual program-bound A state images.
use super::arithmetic_fuel::{NumericAuxA, materialize_numeric_aux_a};
use super::prepare::prepare_a;
use super::rows::{Capacities, ConstantNode, PreparedA, ValueRef};
use super::state::{AImage, StateBlock, StateCapacities};
use super::state_encode::encode_observed_a;
use crate::sherlock::{
    BinaryOp, Control, Environment, Expr, Frame, Outcome, PROFILE, Program, Snapshot, Value,
};
use std::sync::Arc;

fn capacities() -> StateCapacities {
    // Ordinary fixture dimensions only; no resource defaults or adequacy claim.
    StateCapacities {
        data: 16,
        data_fields: 16,
        env_cells: 4,
        value_slots: 4,
        frame_slots: 4,
        control_rows: 4,
        stack_limit: 4,
    }
}

fn binary(op: BinaryOp, a: Value, b: Value, fuel: u128) -> (PreparedA, StateBlock) {
    let program = Arc::new(Program {
        entry: Arc::new(Expr::Binary(
            op,
            Arc::new(Expr::Literal(a.clone())),
            Arc::new(Expr::Literal(b.clone())),
        )),
        functions: vec![],
    });
    let p = prepare_a(
        program,
        Capacities {
            tables: [64; 15],
            data: 16,
        },
    )
    .unwrap();
    let before = Snapshot {
        program: p.full_program.clone(),
        control: Some(Control::Return(b)),
        environment: Environment::new(),
        frames: vec![Frame::Right(op, a)],
        stack_limit: 4,
        remaining_fuel: fuel,
        initial_fuel: fuel,
        outcome: Outcome::Pending,
    };
    let image = encode_observed_a(&p, &capacities(), &before).unwrap();
    (p, image)
}

fn materialize(p: &PreparedA, state: &StateBlock) -> NumericAuxA {
    materialize_numeric_aux_a(
        p,
        &capacities(),
        &AImage {
            profile: PROFILE,
            full_program: &p.full_program,
            program: &p.block,
            state,
        },
    )
    .unwrap()
}

#[test]
fn owner_p1_uses_constant_left_and_data_right_from_bound_rows() {
    let (p, mut state) = binary(BinaryOp::Add, Value::UInt(7), Value::UInt(5), 3);
    assert_eq!(p.block.constants.rows[0].uint_bytes, 7u128.to_le_bytes());
    state.data[2] = state.data[0].clone();
    state.root.active_data = 3;
    state.frame_slots[0].value = ValueRef::constant(0);
    state.control_rows[0].value = ValueRef {
        space: true,
        index: 2,
    };
    let w = materialize(&p, &state);
    assert_eq!(w.header.op_l, ValueRef::constant(0));
    assert_eq!(
        w.header.op_r,
        ValueRef {
            space: true,
            index: 2
        }
    );
    assert_eq!(w.header.n_l, p.block.constants.rows[0]);
    assert_eq!(w.header.n_r, state.data[2]);
    assert_eq!(w.arithmetic.add.r, 12u128.to_le_bytes());
    assert!(w.arithmetic.sel_add && w.arithmetic.uu && !w.arithmetic.ovf);
    assert!(w.fuel.charged);
    assert_eq!(w.fuel.post, 2u128.to_le_bytes());
}

#[test]
fn owner_a3_s3_l4_m6_select_their_actual_right_frame_operands() {
    let cases = [
        (BinaryOp::Add, 255, 1),
        (BinaryOp::Sub, 256, 1),
        (BinaryOp::Lt, 1u128 << 120, 255),
        (BinaryOp::Mul, u128::MAX, u128::MAX),
    ];
    for (op, a, b) in cases {
        let (p, state) = binary(op, Value::UInt(a), Value::UInt(b), 1);
        let w = materialize(&p, &state);
        assert_eq!(w.header.op_l, state.frame_slots[0].value);
        assert_eq!(w.header.op_r, state.control_rows[0].value);
        assert_eq!(w.header.n_l.uint_bytes, a.to_le_bytes());
        assert_eq!(w.header.n_r.uint_bytes, b.to_le_bytes());
        assert!(w.arithmetic.uu && w.fuel.charged);
        assert_eq!(w.fuel.post, [0; 16]);
        match op {
            BinaryOp::Add => {
                assert_eq!(w.arithmetic.add.r, 256u128.to_le_bytes());
                assert!(w.arithmetic.add.c[1] && !w.arithmetic.add.c[16]);
            }
            BinaryOp::Sub => {
                assert_eq!(w.arithmetic.sub.r, 255u128.to_le_bytes());
                assert!(w.arithmetic.sub.bo[1] && !w.arithmetic.sub.bo[16]);
            }
            BinaryOp::Lt => {
                let mut expected = [255; 16];
                expected[0] = 1;
                expected[15] = 0;
                assert_eq!(w.arithmetic.sub.r, expected);
                assert!(!w.arithmetic.lt_out && !w.arithmetic.ovf);
            }
            BinaryOp::Mul => {
                let mut high = [255; 16];
                high[0] = 254;
                assert_eq!(w.arithmetic.mul.r, 1u128.to_le_bytes());
                assert_eq!(w.arithmetic.mul.h, high);
                assert_eq!(w.arithmetic.mul.c[16], 4079);
                assert_eq!(w.arithmetic.mul.c[32], 0);
                assert_eq!(w.arithmetic.mul.h_sum, 4079);
                assert!(w.arithmetic.ovf);
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn owner_t1_t2_type_headers_and_t3_t4_dispatch_stay_separate() {
    for (op, a, b) in [
        (BinaryOp::Add, Value::UInt(1), Value::Bool(true)),
        (BinaryOp::Lt, Value::List(vec![]), Value::UInt(3)),
    ] {
        let (p, mut state) = binary(op, a, b, 1);
        if op == BinaryOp::Lt {
            // Owner T-2 specifically places Nil in Constants.
            state.frame_slots[0].value = ValueRef::constant(0);
        }
        let w = materialize(&p, &state);
        assert!(w.arithmetic.sel_add || w.arithmetic.sel_lt);
        assert!(!w.arithmetic.uu && !w.arithmetic.ovf && !w.arithmetic.lt_out);
        assert_eq!(w.arithmetic.add.r, [0; 16]);
        assert_eq!(w.arithmetic.sub.r, [0; 16]);
        assert_eq!(w.arithmetic.mul.c, [0; 33]);
        assert!(!w.arithmetic.mul.h_zero);
    }
    for (op, a, b) in [
        (BinaryOp::Eq, Value::UInt(1), Value::Bool(true)),
        (BinaryOp::Cons, Value::UInt(1), Value::List(vec![])),
    ] {
        let (p, state) = binary(op, a, b, 1);
        let w = materialize(&p, &state);
        assert!(
            !w.arithmetic.sel_add
                && !w.arithmetic.sel_sub
                && !w.arithmetic.sel_mul
                && !w.arithmetic.sel_lt
                && !w.arithmetic.uu
        );
        assert_eq!(w.header.op_l, ValueRef::default());
        assert_eq!(w.header.op_r, ValueRef::default());
        assert_eq!(w.header.n_l, ConstantNode::default());
        assert_eq!(w.header.n_r, ConstantNode::default());
        assert!(w.fuel.charged);
    }
}

#[test]
fn owner_fuel_values_use_real_control_precedence_and_root_bytes() {
    let (p, base) = binary(BinaryOp::Add, Value::UInt(7), Value::UInt(5), u128::MAX);
    // Owner F1-F4/F8 numeric fuel inputs on ordinary charged Eval controls.
    // This consumer derives charge; it does not select or claim a rule line.
    for (pre, post) in [
        (1, 0),
        (65536, 65535),
        (1u128 << 120, (1u128 << 120) - 1),
        (u128::MAX, u128::MAX - 1),
        (3, 2),
    ] {
        let mut state = base.clone();
        state.root.remaining_fuel = pre.to_le_bytes();
        state.control_rows[0] = super::state::ControlRow {
            tag: 0,
            expr_id: p.block.header.entry_expr_id,
            ..Default::default()
        };
        let w = materialize(&p, &state);
        assert_eq!(w.fuel.post, post.to_le_bytes());
        assert!(w.fuel.charged);
        assert!(!w.arithmetic.uu);
    }
    for (tag, fuel) in [(1, 0u128), (0, 0), (9, 5), (9, 0)] {
        let mut state = base.clone();
        state.root.remaining_fuel = fuel.to_le_bytes();
        state.root.depth = 0;
        state.frame_slots.fill(Default::default());
        state.control_rows[0] = match tag {
            1 => super::state::ControlRow {
                tag: 1,
                value: base.control_rows[0].value,
                ..Default::default()
            },
            0 => super::state::ControlRow {
                tag: 0,
                expr_id: p.block.header.entry_expr_id,
                ..Default::default()
            },
            _ => super::state::ControlRow {
                tag: 9,
                error_code: 4,
                ..Default::default()
            },
        };
        let w = materialize(&p, &state);
        assert!(!w.fuel.charged && !w.arithmetic.uu);
        assert_eq!(w.fuel.post, fuel.to_le_bytes());
        assert_eq!(w.fuel.bw, [false; 17]);
        assert_eq!(w.fuel.zp[16], fuel == 0);
    }
}
