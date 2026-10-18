//! Normal host tests for bounded collection of the existing A-profile events.
//!
//! N01/N02 expectations below are typed transcriptions of the committed independent
//! `models/sherlock/oracles/normal-prefixes.json` literals, frozen before run_steps
//! implementation. Corpus SHA256: dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d.
//! Input corpus SHA256: ef87f94f7e5d22a75b1c2e5e61af016b08175beee9a3599e3e69ddf92945d32f.
//! Expected records never call Machine, its step implementation, or another evaluator.
//! The final mixed-call test separately labels actual one-step internal agreement.

use super::*;
use std::sync::Arc;

fn uint(value: u128) -> ExprRef {
    Arc::new(Expr::Literal(Value::UInt(value)))
}

fn program(case: &str) -> Arc<Program> {
    Arc::new(Program {
        functions: vec![],
        entry: match case {
            "N01" => uint(7),
            "N02" => Arc::new(Expr::Binary(BinaryOp::Add, uint(2), uint(3))),
            _ => panic!("unknown frozen normal case {case}"),
        },
    })
}

// (initial fuel, last charged boundary, remaining fuel, published outcome, rule).
// Every ordinary fuel prefix for these two inputs, copied from the frozen corpus.
const N01_PREFIXES: &[(u128, usize, u128, Outcome, usize)] = &[
    (0, 0, 0, Outcome::Exhausted, 84),
    (1, 1, 0, Outcome::Done(Value::UInt(7)), 80),
    (2, 1, 1, Outcome::Done(Value::UInt(7)), 80),
];
const N02_PREFIXES: &[(u128, usize, u128, Outcome, usize)] = &[
    (0, 0, 0, Outcome::Exhausted, 84),
    (1, 1, 0, Outcome::Exhausted, 84),
    (2, 2, 0, Outcome::Exhausted, 84),
    (3, 3, 0, Outcome::Exhausted, 84),
    (4, 4, 0, Outcome::Exhausted, 84),
    (5, 5, 0, Outcome::Exhausted, 84),
    (6, 6, 0, Outcome::Exhausted, 84),
    (7, 7, 0, Outcome::Done(Value::UInt(5)), 80),
    (8, 7, 1, Outcome::Done(Value::UInt(5)), 80),
];

fn prefixes(case: &str) -> &'static [(u128, usize, u128, Outcome, usize)] {
    match case {
        "N01" => N01_PREFIXES,
        "N02" => N02_PREFIXES,
        _ => panic!("unknown frozen normal case {case}"),
    }
}

fn literal_boundaries(case: &str) -> Vec<(Control, Vec<Frame>, Option<usize>)> {
    match case {
        "N01" => vec![
            (Control::Eval(uint(7)), vec![], None),
            (Control::Return(Value::UInt(7)), vec![], Some(102)),
        ],
        "N02" => {
            let left = Frame::Left(BinaryOp::Add, uint(3), Environment::new());
            let right = Frame::Right(BinaryOp::Add, Value::UInt(2));
            vec![
                (Control::Eval(program("N02").entry.clone()), vec![], None),
                (
                    Control::Push(
                        left.clone(),
                        Box::new(Control::Eval(uint(2))),
                        Environment::new(),
                    ),
                    vec![],
                    Some(117),
                ),
                (Control::Eval(uint(2)), vec![left.clone()], Some(135)),
                (Control::Return(Value::UInt(2)), vec![left], Some(102)),
                (Control::Eval(uint(3)), vec![right.clone()], Some(149)),
                (Control::Return(Value::UInt(3)), vec![right], Some(102)),
                (
                    Control::PrimitiveResult(Ok(Value::UInt(5))),
                    vec![],
                    Some(153),
                ),
                (Control::Return(Value::UInt(5)), vec![], Some(223)),
            ]
        }
        _ => panic!("unknown frozen normal case {case}"),
    }
}

// Apply only the frozen corpus's documented ordinary-prefix reconstruction.
// No rule is executed to construct an expected observation.
fn expected_records(case: &str, fuel: u128) -> Vec<StepRecord> {
    let (_, last, remaining, outcome, publication_rule) = prefixes(case)
        .iter()
        .find(|row| row.0 == fuel)
        .expect("explicit frozen fuel prefix");
    let program = program(case);
    let boundaries = literal_boundaries(case);
    let states: Vec<Snapshot> = boundaries
        .iter()
        .take(last + 1)
        .enumerate()
        .map(|(index, (control, frames, _))| Snapshot {
            program: program.clone(),
            control: Some(control.clone()),
            environment: Environment::new(),
            frames: frames.clone(),
            stack_limit: 8,
            remaining_fuel: fuel - index as u128,
            initial_fuel: fuel,
            outcome: Outcome::Pending,
        })
        .collect();
    let mut records: Vec<StepRecord> = states
        .windows(2)
        .enumerate()
        .map(|(index, pair)| StepRecord {
            rule_start_line: boundaries[index + 1].2.expect("charged edge"),
            charge: 1,
            before: pair[0].clone(),
            after: pair[1].clone(),
        })
        .collect();
    let before = states.last().expect("initial boundary").clone();
    assert_eq!(before.remaining_fuel, *remaining, "literal prefix overlay");
    let mut after = before.clone();
    after.control = None;
    after.outcome = outcome.clone();
    records.push(StepRecord {
        rule_start_line: *publication_rule,
        charge: 0,
        before,
        after,
    });
    records
}

#[test]
fn zero_events_observes_initial_and_partial_machine_without_advancing() {
    let mut machine = Machine::new(program("N02"), 7, 8);
    let start = machine.snapshot();
    assert!(machine.run_steps(0).is_empty());
    assert_eq!(machine.snapshot(), start);

    let expected = expected_records("N02", 7);
    assert_eq!(machine.run_steps(3), expected[..3]);
    let partial = machine.snapshot();
    assert_eq!(partial, expected[2].after);
    assert!(machine.run_steps(0).is_empty());
    assert_eq!(machine.snapshot(), partial);
    assert_eq!(machine.run_steps(2), expected[3..5]);
}

#[test]
fn partial_calls_resume_the_actual_current_machine() {
    let expected = expected_records("N02", 7);
    let mut machine = Machine::new(program("N02"), 7, 8);
    assert_eq!(machine.run_steps(2), expected[..2]);
    assert_eq!(machine.snapshot(), expected[1].after);
    assert_eq!(machine.run_steps(3), expected[2..5]);
    assert_eq!(machine.snapshot(), expected[4].after);
    assert_eq!(machine.run_steps(8), expected[5..]);
    assert_eq!(machine.snapshot(), expected[7].after);
}

#[test]
fn free_publication_is_a_separate_retained_event_at_zero_fuel() {
    let expected = expected_records("N01", 1);
    let mut machine = Machine::new(program("N01"), 1, 8);
    assert_eq!(machine.run_steps(1), expected[..1]);
    assert_eq!(machine.snapshot().remaining_fuel, 0);
    assert_eq!(machine.snapshot().outcome, Outcome::Pending);
    assert!(machine.run_steps(0).is_empty());
    assert_eq!(machine.run_steps(1), expected[1..]);
    assert_eq!(machine.snapshot().outcome, Outcome::Done(Value::UInt(7)));
}

#[test]
fn excess_event_budget_stops_at_first_terminal_publication() {
    for case in ["N01", "N02"] {
        let fuel = if case == "N01" { 1 } else { 7 };
        let expected = expected_records(case, fuel);
        let mut machine = Machine::new(program(case), fuel, 8);
        assert_eq!(machine.run_steps(32), expected);
        assert_eq!(machine.snapshot(), expected.last().unwrap().after);
    }
}

#[test]
fn already_published_normal_outcomes_return_no_further_events() {
    for case in ["N01", "N02"] {
        for (fuel, _, _, _, _) in prefixes(case) {
            let expected = expected_records(case, *fuel);
            let mut machine = Machine::new(program(case), *fuel, 8);
            assert_eq!(machine.run_steps(32), expected);
            let terminal = machine.snapshot();
            for width in [0, 1, 2, 4, 8] {
                assert!(machine.run_steps(width).is_empty());
                assert_eq!(machine.snapshot(), terminal);
            }
        }
    }
}

#[test]
fn widths_one_two_four_eight_retain_every_independent_normal_prefix() {
    for case in ["N01", "N02"] {
        for (fuel, _, _, _, _) in prefixes(case) {
            let expected = expected_records(case, *fuel);
            for width in [1, 2, 4, 8] {
                let mut machine = Machine::new(program(case), *fuel, 8);
                assert_eq!(machine.snapshot(), expected[0].before);
                let mut offset = 0;
                while offset < expected.len() {
                    let end = (offset + width).min(expected.len());
                    let records = machine.run_steps(width);
                    // StepRecord equality covers every field of both full snapshots,
                    // the exact source-rule line and the raw per-event charge.
                    assert_eq!(records, expected[offset..end], "{case}, fuel={fuel}, width={width}, offset={offset}");
                    assert_eq!(machine.snapshot(), expected[end - 1].after);
                    offset = end;
                }
                assert!(machine.run_steps(width).is_empty());
            }
        }
    }
}

#[test]
fn mixed_step_and_group_calls_match_actual_one_step_baseline() {
    // Supplemental internal agreement, not an independent semantic oracle.
    let mut baseline = Machine::new(program("N02"), 8, 8);
    let mut actual = Machine::new(program("N02"), 8, 8);
    let first = baseline.step().expect("first baseline event");
    assert_eq!(actual.step(), Some(first));
    for width in [2, 0, 1, 4, 8] {
        let mut expected = Vec::new();
        for _ in 0..width {
            let Some(record) = baseline.step() else { break };
            expected.push(record);
        }
        assert_eq!(actual.run_steps(width), expected);
        assert_eq!(actual.snapshot(), baseline.snapshot());
    }
}
