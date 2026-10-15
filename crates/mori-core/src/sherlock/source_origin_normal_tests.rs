//! Source-first ordinary causal-metadata examples.
//! Paths/cursors below are literal derivations from the written programs and
//! frozen K-A rule boundaries, never obtained from a machine or helper output.
//! Wire as a cfg(test) child of sherlock; existing modules/tests stay unchanged.
use super::helper_call_adapter::CallContext;
use super::image_reference::ImageMachine;
use super::program_image::{prepare_program_image, SourceLocator, SourceOwner};
use super::reference_types::*;
use std::sync::Arc;

fn expr(expression: Expr) -> ExprRef { Arc::new(expression) }
fn uint(value: u128) -> ExprRef { expr(Expr::Literal(Value::UInt(value))) }
fn var(name: &str) -> ExprRef { expr(Expr::Var(name.into())) }
fn entry(path: &[usize]) -> SourceLocator {
    SourceLocator { owner: SourceOwner::Entry, ast_path: path.to_vec() }
}
fn function(path: &[usize]) -> SourceLocator {
    SourceLocator { owner: SourceOwner::Function(0), ast_path: path.to_vec() }
}
fn machine(program: Program, fuel: u128) -> ImageMachine {
    ImageMachine::new(Arc::new(prepare_program_image(&program)), fuel, 16)
}
fn advance(machine: &mut ImageMachine, count: usize) {
    for _ in 0..count {
        let record = machine.step().expect("fixed ordinary charged prefix");
        assert_eq!(record.charge, 1);
        assert_eq!(record.after.outcome, Outcome::Pending);
    }
}
fn metadata(machine: &ImageMachine, cursor: u128, origin: SourceLocator,
    owners: &[SourceLocator])
{
    let image = machine.program_image();
    assert_eq!(machine.snapshot().charged_steps(), cursor);
    assert_eq!(image.source_map[machine.control_origin().0], origin);
    let actual: Vec<_> = machine.frame_origins().iter()
        .map(|id| image.source_map[id.0].clone()).collect();
    assert_eq!(actual, owners);
    assert_eq!(machine.frame_origins().len(), machine.snapshot().frames.len());
}
fn capture_call(machine: &ImageMachine, expected: SourceLocator) {
    let mut context = CallContext::from_image_machine(machine).expect("actual Invoke capture");
    let id = machine.control_origin();
    assert_eq!(context.resolve_origin(0).unwrap(), &(id, expected.clone()));
    assert_eq!(context.saved_frame_origins(), Some(machine.frame_origins()));
    assert_eq!(context.work().reference_cursor, machine.snapshot().charged_steps());
    assert_eq!(context.snapshot().unwrap(), machine.snapshot());
    assert_eq!(context.resume_snapshot().unwrap(), machine.snapshot());
    // Every real helper prefix keeps the captured call and saved frame owners.
    // Existing independent frozen tests remain the helper work-count authority.
    while context.step().unwrap().is_some() {
        assert_eq!(context.resolve_origin(0).unwrap(), &(id, expected.clone()));
        assert_eq!(context.saved_frame_origins(), Some(machine.frame_origins()));
    }
}

#[test]
fn two_same_name_calls_keep_distinct_paths_through_pending_returns() {
    let call = || expr(Expr::Call("f".into(), vec![]));
    let mut machine = machine(Program {
        functions: vec![Function { name: "f".into(), parameters: vec![],
            return_type: Type::UInt, body: uint(7) }],
        entry: expr(Expr::Binary(BinaryOp::Add, call(), call())),
    }, 19);
    metadata(&machine, 0, entry(&[]), &[]);
    advance(&mut machine, 1); // binary schedules a pending push, still owned by root
    metadata(&machine, 1, entry(&[]), &[]);
    assert!(matches!(machine.snapshot().control, Some(Control::Push(..))));
    advance(&mut machine, 2);
    metadata(&machine, 3, entry(&[0]), &[entry(&[])]);
    assert_eq!(machine.snapshot().control, Some(Control::Invoke("f".into(), vec![])));
    let left_call = machine.control_origin();
    capture_call(&machine, entry(&[0]));
    advance(&mut machine, 5);
    metadata(&machine, 8, function(&[]), &[entry(&[0]), entry(&[])]);
    assert_eq!(machine.snapshot().control, Some(Control::Return(Value::UInt(7))));
    assert_eq!(machine.snapshot().frames, vec![
        Frame::Return(Type::UInt, Environment::new()),
        Frame::Left(BinaryOp::Add, call(), Environment::new()),
    ]);
    advance(&mut machine, 1);
    metadata(&machine, 9, entry(&[0]), &[entry(&[])]);
    advance(&mut machine, 2);
    metadata(&machine, 11, entry(&[1]), &[entry(&[])]);
    assert_ne!(machine.control_origin(), left_call);
    assert_eq!(machine.snapshot().frames, vec![Frame::Right(BinaryOp::Add, Value::UInt(7))]);
    capture_call(&machine, entry(&[1]));
    advance(&mut machine, 5);
    metadata(&machine, 16, function(&[]), &[entry(&[1]), entry(&[])]);
    advance(&mut machine, 1);
    metadata(&machine, 17, entry(&[1]), &[entry(&[])]);
    advance(&mut machine, 2);
    metadata(&machine, 19, entry(&[]), &[]);
    let published = machine.step().unwrap();
    assert_eq!((published.rule_start_line, published.charge), (80, 0));
    assert_eq!(published.after.outcome, Outcome::Done(Value::UInt(14)));
    assert_eq!(published.after.remaining_fuel, 0);
    metadata(&machine, 19, entry(&[]), &[]);
    assert!(machine.step().is_none());
}

#[test]
fn recursive_occurrence_repeats_with_distinct_cursors_and_saved_call_owners() {
    // f(n) = if n == 0 then 9 else f(n - 1), entry f(2).
    // Entry Invoke is at4; the one recursive source Call at [2] executes at28,52.
    // Each nonzero level costs24 after its Invoke. Base returns at67, then the
    // three saved Return frames consume steps68,69,70 before free publication.
    let body = expr(Expr::If(
        expr(Expr::Binary(BinaryOp::Eq, var("n"), uint(0))), uint(9),
        expr(Expr::Call("f".into(), vec![
            expr(Expr::Binary(BinaryOp::Sub, var("n"), uint(1)))]))));
    let mut machine = machine(Program {
        functions: vec![Function { name: "f".into(), parameters: vec![
            Parameter { name: "n".into(), ty: Type::UInt }],
            return_type: Type::UInt, body }],
        entry: expr(Expr::Call("f".into(), vec![uint(2)])),
    }, 70);
    advance(&mut machine, 4);
    metadata(&machine, 4, entry(&[]), &[]);
    capture_call(&machine, entry(&[]));
    advance(&mut machine, 24);
    metadata(&machine, 28, function(&[2]), &[entry(&[])]);
    assert_eq!(machine.snapshot().control,
        Some(Control::Invoke("f".into(), vec![Value::UInt(1)])));
    let repeated = machine.control_origin();
    capture_call(&machine, function(&[2]));
    advance(&mut machine, 24);
    metadata(&machine, 52, function(&[2]), &[function(&[2]), entry(&[])]);
    assert_eq!(machine.control_origin(), repeated);
    assert_eq!(machine.snapshot().control,
        Some(Control::Invoke("f".into(), vec![Value::UInt(0)])));
    capture_call(&machine, function(&[2]));
    advance(&mut machine, 15);
    metadata(&machine, 67, function(&[1]),
        &[function(&[2]), function(&[2]), entry(&[])]);
    assert_eq!(machine.snapshot().control, Some(Control::Return(Value::UInt(9))));
    assert_eq!(machine.snapshot().frames, vec![
        Frame::Return(Type::UInt, Environment::from([("n".into(), Value::UInt(1))])),
        Frame::Return(Type::UInt, Environment::from([("n".into(), Value::UInt(2))])),
        Frame::Return(Type::UInt, Environment::new()),
    ]);
    advance(&mut machine, 1);
    metadata(&machine, 68, function(&[2]), &[function(&[2]), entry(&[])]);
    advance(&mut machine, 1);
    metadata(&machine, 69, function(&[2]), &[entry(&[])]);
    advance(&mut machine, 1);
    metadata(&machine, 70, entry(&[]), &[]);
    let published = machine.step().unwrap();
    assert_eq!((published.rule_start_line, published.charge), (80, 0));
    assert_eq!(published.after.outcome, Outcome::Done(Value::UInt(9)));
    metadata(&machine, 70, entry(&[]), &[]);
}

#[test]
fn ordinary_underflow_retains_binary_owner_and_pending_let_at_free_error() {
    let mut machine = machine(Program { functions: vec![],
        entry: expr(Expr::Let("x".into(),
            expr(Expr::Binary(BinaryOp::Sub, uint(1), uint(2))), uint(0))),
    }, 9);
    advance(&mut machine, 7);
    metadata(&machine, 7, entry(&[0, 1]), &[entry(&[0]), entry(&[])]);
    advance(&mut machine, 1);
    metadata(&machine, 8, entry(&[0]), &[entry(&[])]);
    assert_eq!(machine.snapshot().control, Some(Control::PrimitiveResult(Err(10))));
    advance(&mut machine, 1);
    metadata(&machine, 9, entry(&[0]), &[entry(&[])]);
    assert_eq!(machine.snapshot().control, Some(Control::Error(10)));
    let published = machine.step().unwrap();
    assert_eq!((published.rule_start_line, published.charge), (82, 0));
    assert_eq!(published.after.outcome, Outcome::Rejected(10));
    assert_eq!(published.after.remaining_fuel, 0);
    assert_eq!(published.after.frames,
        vec![Frame::Let("x".into(), uint(0), Environment::new())]);
    metadata(&machine, 9, entry(&[0]), &[entry(&[])]);
}
