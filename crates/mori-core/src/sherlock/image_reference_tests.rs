//! Source-first normal tests for the proposed image consumer. Expected counts,
//! values and saved frames come from EXTRA-NORMAL-DERIVATION.md and its frozen
//! N07/N08 literals, independently of either Rust transition implementation.
//! These tests are specified-only until separately authorized execution.

use super::super::program_image::prepare_program_image;
use super::*;

use super::super::extra_oracle_programs;

fn machine(case: &str, fuel: u128) -> ImageMachine {
    let program = extra_oracle_programs::extra_program(case).expect("normal source case");
    ImageMachine::new(Arc::new(prepare_program_image(&program)), fuel, 8)
}

fn environment(n: u128) -> Environment {
    Environment::from([("0".into(), Value::UInt(n))])
}

fn advance(machine: &mut ImageMachine, count: usize) {
    for _ in 0..count {
        let record = machine.step().expect("pending normal transition");
        assert_eq!(record.charge, 1);
        assert_eq!(record.after.outcome, Outcome::Pending);
    }
}

#[test]
fn n07_ordered_arguments_keep_return_check_and_free_publication() {
    let mut machine = machine("N07", 39);
    advance(&mut machine, 37);
    let pending = machine.snapshot();
    assert_eq!(pending.control, Some(Control::Return(Value::UInt(321))));
    assert_eq!(pending.remaining_fuel, 2);
    assert_eq!(
        pending.frames,
        vec![Frame::Return(Type::UInt, Environment::new())]
    );
    assert_eq!(
        pending.environment,
        Environment::from([
            ("0".into(), Value::UInt(1)),
            ("1".into(), Value::UInt(2)),
            ("2".into(), Value::UInt(3)),
        ])
    );

    let restored = machine.step().expect("charged caller return");
    assert_eq!((restored.rule_start_line, restored.charge), (185, 1));
    assert_eq!(restored.after.remaining_fuel, 1);
    assert!(restored.after.environment.is_empty());
    assert!(restored.after.frames.is_empty());
    let published = machine.step().expect("free done publication");
    assert_eq!((published.rule_start_line, published.charge), (80, 0));
    assert_eq!(published.after.outcome, Outcome::Done(Value::UInt(321)));
    assert_eq!(published.after.remaining_fuel, 1);
    assert_eq!(published.after.charged_steps(), 38);
    assert!(machine.step().is_none());
}

#[test]
fn n08_named_recursion_preserves_all_pending_returns_and_multiplications() {
    let mut machine = machine("N08", 82);
    advance(&mut machine, 75);
    let base = machine.snapshot();
    assert_eq!(base.control, Some(Control::Return(Value::UInt(1))));
    assert_eq!(base.environment, environment(0));
    assert_eq!(base.remaining_fuel, 7);
    assert_eq!(
        base.frames,
        vec![
            Frame::Return(Type::UInt, environment(1)),
            Frame::Right(BinaryOp::Mul, Value::UInt(1)),
            Frame::Return(Type::UInt, environment(2)),
            Frame::Right(BinaryOp::Mul, Value::UInt(2)),
            Frame::Return(Type::UInt, Environment::new()),
        ]
    );
    let first_return = machine.step().expect("base caller return");
    assert_eq!(first_return.rule_start_line, 185);
    assert_eq!(first_return.after.environment, environment(1));
    assert_eq!(first_return.after.frames, base.frames[1..].to_vec());
    advance(&mut machine, 2);
    let second_return = machine.step().expect("recursive caller return");
    assert_eq!(second_return.rule_start_line, 185);
    assert_eq!(second_return.after.environment, environment(2));
    assert_eq!(second_return.after.frames, base.frames[3..].to_vec());
    advance(&mut machine, 2);
    assert_eq!(
        machine.snapshot().frames,
        vec![Frame::Return(Type::UInt, Environment::new())]
    );
    let outer_return = machine.step().expect("outer caller return");
    assert_eq!(outer_return.rule_start_line, 185);
    assert_eq!(outer_return.after.remaining_fuel, 0);
    assert!(outer_return.after.environment.is_empty());
    assert!(outer_return.after.frames.is_empty());
    let publication = machine.step().expect("done at zero fuel");
    assert_eq!((publication.rule_start_line, publication.charge), (80, 0));
    assert_eq!(publication.after.outcome, Outcome::Done(Value::UInt(2)));
    assert_eq!(publication.after.charged_steps(), 82);
    assert_eq!(publication.after.remaining_fuel, 0);
}

#[test]
fn n08_fuel_prefix_keeps_last_pending_return_observable() {
    let mut machine = machine("N08", 81);
    let final_state = machine.run();
    assert_eq!(final_state.outcome, Outcome::Exhausted);
    assert_eq!(final_state.remaining_fuel, 0);
    assert_eq!(final_state.environment, environment(2));
    assert_eq!(
        final_state.frames,
        vec![Frame::Return(Type::UInt, Environment::new())]
    );
    assert_eq!(final_state.charged_steps(), 81);
}
