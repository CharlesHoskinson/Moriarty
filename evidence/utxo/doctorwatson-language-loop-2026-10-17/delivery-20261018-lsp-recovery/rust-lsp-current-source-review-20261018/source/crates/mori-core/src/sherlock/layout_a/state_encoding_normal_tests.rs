//! Independent full-byte normal expectations, authored first. ALL UNRUN.
//! Literal vectors copied from the Root-owned expected.json; no candidate oracle.
use std::sync::Arc;
use crate::sherlock::{Control, Environment, Expr, Outcome, Program, Snapshot, Value};
use super::{prepare::prepare_a, rows::{Capacities, PreparedA}, state::StateCapacities};
use super::state_encoding::observed_state_bytes_a;

fn prepared(data: u128) -> PreparedA {
    let program = Program { functions: vec![], entry: Arc::new(Expr::Literal(Value::UInt(0))) };
    prepare_a(Arc::new(program), Capacities {
        tables: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0], data,
    }).unwrap()
}

#[test]
fn pending_eval_root_zero() {
    let p = prepared(0);
    let caps = StateCapacities {
        data: 0, data_fields: 0, env_cells: 0, value_slots: 0, frame_slots: 0, control_rows: 1, stack_limit: 0,
    };
    let snapshot = Snapshot {
        program: p.full_program.clone(),
        control: Some(Control::Eval(p.full_program.entry.clone())), environment: Environment::new(), frames: vec![],
        stack_limit: 0, remaining_fuel: 7,
        initial_fuel: 9, outcome: Outcome::Pending,
    };
    let expected: Vec<u8> = vec![
        0, 0, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,
    ];
    let actual = observed_state_bytes_a(&p, &caps, &snapshot).unwrap();
    assert_eq!(actual.len(), 81);
    assert_eq!(actual, expected);
}

#[test]
fn done_uint_little_endian() {
    let p = prepared(1);
    let caps = StateCapacities {
        data: 1, data_fields: 0, env_cells: 0, value_slots: 0, frame_slots: 0, control_rows: 0, stack_limit: 0,
    };
    let snapshot = Snapshot {
        program: p.full_program.clone(),
        control: None, environment: Environment::new(), frames: vec![],
        stack_limit: 0, remaining_fuel: 1,
        initial_fuel: 2, outcome: Outcome::Done(Value::UInt(0x0102030405060708090a0b0c0d0e0f10)),
    };
    let expected: Vec<u8> = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        0, 0, 0,
    ];
    let actual = observed_state_bytes_a(&p, &caps, &snapshot).unwrap();
    assert_eq!(actual.len(), 67);
    assert_eq!(actual, expected);
}

#[test]
fn stack_limit_256_full_fuel() {
    let p = prepared(0);
    let caps = StateCapacities {
        data: 0, data_fields: 0, env_cells: 0, value_slots: 0, frame_slots: 0, control_rows: 1, stack_limit: 256,
    };
    let snapshot = Snapshot {
        program: p.full_program.clone(),
        control: Some(Control::Eval(p.full_program.entry.clone())), environment: Environment::new(), frames: vec![],
        stack_limit: 256, remaining_fuel: 340282366920938463463374607431768211455,
        initial_fuel: 340282366920938463463374607431768211455, outcome: Outcome::Pending,
    };
    let expected: Vec<u8> = vec![
        0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let actual = observed_state_bytes_a(&p, &caps, &snapshot).unwrap();
    assert_eq!(actual.len(), 82);
    assert_eq!(actual, expected);
}

// Root supplied this fourth independent vector after serializer source was written.
// Its expectation is hand-derived; this test and the initial three remain UNRUN.
#[test]
fn push_saved_current_and_installed_frame() {
    use crate::sherlock::{BinaryOp, Frame};
    let literal = Arc::new(Expr::Literal(Value::UInt(0)));
    let variable = Arc::new(Expr::Var("x".into()));
    let program = Program { functions: vec![], entry: Arc::new(
        Expr::Let("x".into(), literal.clone(), variable.clone())) };
    let p = prepare_a(Arc::new(program), Capacities {
        tables: [1, 1, 3, 2, 3, 2, 0, 0, 0, 0, 4, 4, 5, 1, 0], data: 3,
    }).unwrap();
    let caps = StateCapacities {
        data: 3, data_fields: 0, env_cells: 2, value_slots: 0,
        frame_slots: 1, control_rows: 2, stack_limit: 1,
    };
    let saved = [("x".into(), Value::Bool(true))].into_iter().collect();
    let current = [("x".into(), Value::UInt(9))].into_iter().collect();
    let snapshot = Snapshot {
        program: p.full_program.clone(),
        control: Some(Control::Push(Frame::Left(BinaryOp::Sub, variable, saved),
            Box::new(Control::Eval(literal)), Environment::new())),
        environment: current, frames: vec![Frame::Right(BinaryOp::Mul, Value::UInt(7))],
        stack_limit: 1, remaining_fuel: 10, initial_fuel: 12, outcome: Outcome::Pending,
    };
    let expected: Vec<u8> = vec![
        3, 0, 2, 0, 1, 2, 1, 1, 1, 10, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 1, 1, 0, 0, 2, 0, 2, 0, 0, 0, 0,
        0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0,
        1, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let actual = observed_state_bytes_a(&p, &caps, &snapshot).unwrap();
    assert_eq!(actual.len(), 210);
    assert_eq!(actual, expected);
}
