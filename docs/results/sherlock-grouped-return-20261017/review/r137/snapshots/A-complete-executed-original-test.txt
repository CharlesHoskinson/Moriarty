//! External positive UIREQ-032 grouped pending-callee discriminator.
//! Expected tail is transcribed from immutable N08 oracle boundaries75..82
//! plus its explicit free-publication override, never from Machine output.
//! Source only: not compiled or run. Public mori_core API only.
use mori_core::sherlock::{
    BinaryOp, Control, Environment, Expr, ExprRef, Frame, Function, Machine,
    Outcome, Parameter, Program, Snapshot, StepRecord, Type, Value, PROFILE,
};
use mori_core::sherlock::reference_slice::{
    advance_m, ReferenceSlice, SliceBoundary, SliceBoundaryKind, SliceStop,
};
use std::sync::Arc;

fn expr(e: Expr) -> ExprRef { Arc::new(e) }
fn uint(n: u128) -> ExprRef { expr(Expr::Literal(Value::UInt(n))) }
fn var() -> ExprRef { expr(Expr::Var("0".into())) }
fn binary(op: BinaryOp, a: ExprRef, b: ExprRef) -> ExprRef { expr(Expr::Binary(op, a, b)) }
fn call(arg: ExprRef) -> ExprRef { expr(Expr::Call("18".into(), vec![arg])) }

// Source-only typed transcription of extra-normal-programs.json/N08 and the
// existing extra_oracle_programs.rs literal builder, using public imports.
fn program() -> Arc<Program> {
    Arc::new(Program {
        functions: vec![Function {
            name: "18".into(),
            parameters: vec![Parameter { name: "0".into(), ty: Type::UInt }],
            return_type: Type::UInt,
            body: expr(Expr::If(
                binary(BinaryOp::Eq, var(), uint(0)),
                uint(1),
                binary(BinaryOp::Mul, var(), call(binary(BinaryOp::Sub, var(), uint(1)))),
            )),
        }],
        entry: call(uint(2)),
    })
}
fn env(n: u128) -> Environment {
    Environment::from([("0".into(), Value::UInt(n))])
}
fn empty() -> Environment { Environment::new() }
fn ret(saved: Environment) -> Frame { Frame::Return(Type::UInt, saved) }
fn mul(n: u128) -> Frame { Frame::Right(BinaryOp::Mul, Value::UInt(n)) }
fn returning(n: u128) -> Option<Control> { Some(Control::Return(Value::UInt(n))) }
fn primitive(n: u128) -> Option<Control> { Some(Control::PrimitiveResult(Ok(Value::UInt(n)))) }

// Every Snapshot field is literal or bound to the immutable source Program.
// This helper performs no evaluator/subject call and copies no observed state.
fn expected_snapshot(
    program: &Arc<Program>, fuel: u128, environment: Environment,
    frames: Vec<Frame>, control: Option<Control>, outcome: Outcome,
) -> Snapshot {
    Snapshot {
        program: program.clone(), control, environment, frames,
        stack_limit: 8, remaining_fuel: fuel, initial_fuel: 82, outcome,
    }
}
fn expected_tail(program: &Arc<Program>) -> Vec<Snapshot> {
    vec![
        // State75: base case returned1; caller multiplications1 and2 pending.
        expected_snapshot(program, 7, env(0), vec![ret(env(1)), mul(1), ret(env(2)), mul(2), ret(empty())], returning(1), Outcome::Pending),
        // State76: caller n=1 restored, its pending multiplication retained.
        expected_snapshot(program, 6, env(1), vec![mul(1), ret(env(2)), mul(2), ret(empty())], returning(1), Outcome::Pending),
        expected_snapshot(program, 5, env(1), vec![ret(env(2)), mul(2), ret(empty())], primitive(1), Outcome::Pending),
        expected_snapshot(program, 4, env(1), vec![ret(env(2)), mul(2), ret(empty())], returning(1), Outcome::Pending),
        // State79: caller n=2 restored, outer multiplication2 still pending.
        expected_snapshot(program, 3, env(2), vec![mul(2), ret(empty())], returning(1), Outcome::Pending),
        expected_snapshot(program, 2, env(2), vec![ret(empty())], primitive(2), Outcome::Pending),
        expected_snapshot(program, 1, env(2), vec![ret(empty())], returning(2), Outcome::Pending),
        // State82 remains Pending Return at zero fuel, before free publication.
        expected_snapshot(program, 0, empty(), vec![], returning(2), Outcome::Pending),
        // Oracle publication rule80 changes only control/outcome at zero fuel.
        expected_snapshot(program, 0, empty(), vec![], None, Outcome::Done(Value::UInt(2))),
    ]
}
fn expected_records(states: &[Snapshot]) -> Vec<StepRecord> {
    let rules = [185, 153, 223, 185, 153, 223, 185, 80];
    let charges = [1, 1, 1, 1, 1, 1, 1, 0];
    rules.into_iter().zip(charges).enumerate().map(|(i, (rule_start_line, charge))| {
        StepRecord {
            rule_start_line, charge,
            before: states[i].clone(), after: states[i + 1].clone(),
        }
    }).collect()
}
fn assert_group(
    actual: &ReferenceSlice, states: &[Snapshot], records: &[StepRecord],
    first: usize, past: usize,
) {
    assert_eq!(actual.profile_id, PROFILE);
    assert_eq!(actual.start, states[first]);
    // Whole typed record equality covers every before/after field and order,
    // not merely the final result, line list or cross-group self-consistency.
    assert_eq!(actual.records, records[first..past]);
    assert_eq!(actual.end, states[past]);
    let mut prefix = vec![0u128];
    for expected in &records[first..past] {
        prefix.push(prefix.last().copied().unwrap() + u128::from(expected.charge));
    }
    assert_eq!(actual.prefix_work, prefix);
    let terminal = past == records.len();
    let (stop, kind) = if terminal {
        (SliceStop::Outcome(Outcome::Done(Value::UInt(2))), SliceBoundaryKind::Terminal)
    } else {
        (SliceStop::TechnicalCut, SliceBoundaryKind::TechnicalCut)
    };
    assert_eq!(actual.stop, stop);
    assert_eq!(actual.boundaries, vec![SliceBoundary { position: past - first, kind }]);
    for event in &actual.records {
        assert!(event.before.depth() <= event.before.stack_limit);
        assert!(event.after.depth() <= event.after.stack_limit);
    }
}

#[test]
fn grouped_recursive_callee_return_preserves_complete_records_for_widths_1_2_4_8() {
    // All starts are independently frozen boundaries around the same callee
    // return: before base return, just after base return, and before next return.
    // The latter starts make width8 include fewer than8 events before first stop.
    for start_boundary in [75usize, 76, 78] {
        for width in [1usize, 2, 4, 8] {
            let source = program();
            let states = expected_tail(&source);
            let expected = expected_records(&states);
            let mut machine = Machine::new(source, 82, 8);
            // Warm-up observations are discarded, not used as expectations.
            // Full independently frozen start equality below checks the cut.
            for _ in 0..start_boundary { machine.step().expect("pending warm-up event"); }
            let mut at = start_boundary - 75;
            assert_eq!(machine.snapshot(), states[at]);
            while at < expected.len() {
                let past = (at + width).min(expected.len());
                let group = advance_m(&mut machine, width);
                assert_group(&group, &states, &expected, at, past);
                assert_eq!(machine.snapshot(), states[past]);
                at = past;
            }
            let terminal = states.last().unwrap();
            let empty_group = advance_m(&mut machine, width);
            assert_eq!(empty_group.start, *terminal);
            assert_eq!(empty_group.end, *terminal);
            assert!(empty_group.records.is_empty());
            assert_eq!(empty_group.prefix_work, vec![0]);
            assert_eq!(empty_group.stop, SliceStop::Outcome(Outcome::Done(Value::UInt(2))));
            assert_eq!(empty_group.boundaries, vec![SliceBoundary {
                position: 0, kind: SliceBoundaryKind::Terminal,
            }]);
            assert!(machine.step().is_none(), "no event after the first published outcome");
            assert_eq!(machine.snapshot(), *terminal);
        }
    }
}
