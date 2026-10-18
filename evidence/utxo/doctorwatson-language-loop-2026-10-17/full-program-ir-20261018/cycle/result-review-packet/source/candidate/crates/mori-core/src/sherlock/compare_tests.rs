//! Ordinary observation-equality unit coverage, not trace-validity or proof checks.
//! Literal N01 states follow the committed normal-prefix oracle: rule 102 costs
//! one step; rule 80 publishes UInt(7) for free. No evaluator computes expectations.

use super::Machine;
use super::compare::*;
use super::reference_types::*;
use std::sync::Arc;

const IDENTITY: TraceIdentity<'static> = TraceIdentity {
    profile: "sherlock-k-a/1",
    source_sha256: "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41",
};

struct LiteralTrace {
    initial: Snapshot,
    records: Vec<StepRecord>,
}

impl LiteralTrace {
    fn view(&self) -> TraceView<'_> {
        TraceView {
            identity: IDENTITY,
            initial: &self.initial,
            records: &self.records,
        }
    }
}

fn literal_program(value: u128) -> Arc<Program> {
    Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(value))),
    })
}

fn literal_seven() -> LiteralTrace {
    let program = literal_program(7);
    let initial = Snapshot {
        program: program.clone(),
        control: Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 1,
        initial_fuel: 1,
        outcome: Outcome::Pending,
    };
    let returned = Snapshot {
        program: program.clone(),
        control: Some(Control::Return(Value::UInt(7))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 1,
        outcome: Outcome::Pending,
    };
    let published = Snapshot {
        program,
        control: None,
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 1,
        outcome: Outcome::Done(Value::UInt(7)),
    };
    LiteralTrace {
        initial: initial.clone(),
        records: vec![
            StepRecord {
                rule_start_line: 102,
                charge: 1,
                before: initial,
                after: returned.clone(),
            },
            StepRecord {
                rule_start_line: 80,
                charge: 0,
                before: returned,
                after: published,
            },
        ],
    }
}

fn assert_difference(left: TraceView<'_>, right: TraceView<'_>, expected: TraceDifference) {
    assert_eq!(first_difference(left, right), Some(expected));
    // The position is stable under exchanging the two original observations.
    // Length contains side-specific counts, so those cases assert both explicitly.
    if !matches!(expected, TraceDifference::Length { .. }) {
        assert_eq!(first_difference(right, left), Some(expected));
    }
}

#[test]
fn equal_literal_trace_uses_structural_equality_across_distinct_arcs() {
    let left = literal_seven();
    let right = literal_seven();
    assert!(!Arc::ptr_eq(&left.initial.program, &right.initial.program));
    assert!(!Arc::ptr_eq(
        &left.initial.program.entry,
        &right.initial.program.entry,
    ));
    assert_eq!(first_difference(left.view(), right.view()), None);
    assert_eq!(first_difference(left.view(), left.view()), None);
}

#[test]
fn actual_machine_observations_match_literal_two_step_oracle() {
    let expected = literal_seven();
    let mut machine = Machine::new(literal_program(7), 1, 8);
    let initial = machine.snapshot();
    let records = vec![
        machine.step().expect("literal evaluation"),
        machine.step().expect("free terminal publication"),
    ];
    assert!(machine.step().is_none());
    let actual = TraceView {
        identity: TraceIdentity {
            profile: PROFILE,
            source_sha256: K_SOURCE_SHA256,
        },
        initial: &initial,
        records: &records,
    };
    assert_eq!(first_difference(expected.view(), actual), None);
}

#[test]
fn profile_identity_is_the_first_comparison() {
    let left = literal_seven();
    let mut right = literal_seven();
    right.initial.program = literal_program(8);
    let mut right_view = right.view();
    right_view.identity = TraceIdentity {
        profile: "different-declared-clock",
        source_sha256: "different-source",
    };
    assert_difference(
        left.view(),
        right_view,
        TraceDifference::Identity(IdentityField::Profile),
    );
}

#[test]
fn source_identity_precedes_state_differences() {
    let left = literal_seven();
    let mut right = literal_seven();
    right.initial.program = literal_program(8);
    let mut right_view = right.view();
    right_view.identity.source_sha256 = "different-source";
    assert_difference(
        left.view(),
        right_view,
        TraceDifference::Identity(IdentityField::SourceSha256),
    );
}

// These changes exercise ordinary field equality. They do not ask the comparator
// to accept or reject a transition, prove validity, or authenticate metadata.
const SNAPSHOT_FIELDS: [SnapshotField; 8] = [
    SnapshotField::Program,
    SnapshotField::Control,
    SnapshotField::Environment,
    SnapshotField::Frames,
    SnapshotField::StackLimit,
    SnapshotField::RemainingFuel,
    SnapshotField::InitialFuel,
    SnapshotField::Outcome,
];

fn change_field(snapshot: &mut Snapshot, field: SnapshotField) {
    match field {
        SnapshotField::Program => snapshot.program = literal_program(8),
        SnapshotField::Control => snapshot.control = Some(Control::Return(Value::UInt(8))),
        SnapshotField::Environment => {
            snapshot.environment.insert("0".into(), Value::Bool(true));
        }
        SnapshotField::Frames => {
            snapshot.frames.push(Frame::Scope(Environment::from([(
                "1".into(),
                Value::List(vec![4, 5]),
            )])));
        }
        SnapshotField::StackLimit => snapshot.stack_limit = 9,
        SnapshotField::RemainingFuel => snapshot.remaining_fuel = 3,
        SnapshotField::InitialFuel => snapshot.initial_fuel = 4,
        SnapshotField::Outcome => snapshot.outcome = Outcome::Done(Value::UInt(8)),
    }
}

#[test]
fn every_initial_snapshot_field_has_an_exact_position() {
    let left = literal_seven();
    for field in SNAPSHOT_FIELDS {
        let mut right = literal_seven();
        change_field(&mut right.initial, field);
        assert_difference(left.view(), right.view(), TraceDifference::Initial(field));
    }
}

#[test]
fn every_record_before_and_after_field_has_an_exact_position() {
    let left = literal_seven();
    for index in 0..2 {
        for field in SNAPSHOT_FIELDS {
            let mut right = literal_seven();
            change_field(&mut right.records[index].before, field);
            assert_difference(
                left.view(),
                right.view(),
                TraceDifference::Record {
                    index,
                    field: RecordField::Before(field),
                },
            );
            let mut right = literal_seven();
            change_field(&mut right.records[index].after, field);
            assert_difference(
                left.view(),
                right.view(),
                TraceDifference::Record {
                    index,
                    field: RecordField::After(field),
                },
            );
        }
    }
}

#[test]
fn each_record_rule_and_charge_has_an_exact_position() {
    let left = literal_seven();
    for index in 0..2 {
        let mut right = literal_seven();
        right.records[index].rule_start_line = 108;
        assert_difference(
            left.view(),
            right.view(),
            TraceDifference::Record {
                index,
                field: RecordField::RuleStartLine,
            },
        );
        let mut right = literal_seven();
        right.records[index].charge = 2;
        assert_difference(
            left.view(),
            right.view(),
            TraceDifference::Record {
                index,
                field: RecordField::Charge,
            },
        );
    }
}

#[test]
fn initial_field_order_is_stable_when_multiple_fields_differ() {
    let left = literal_seven();
    for (index, first) in SNAPSHOT_FIELDS.into_iter().enumerate() {
        let mut right = literal_seven();
        for field in &SNAPSHOT_FIELDS[index..] {
            change_field(&mut right.initial, *field);
        }
        assert_difference(left.view(), right.view(), TraceDifference::Initial(first));
    }
}

#[test]
fn earliest_record_precedes_later_metadata_differences() {
    let left = literal_seven();
    let mut right = literal_seven();
    right.records[0].after.outcome = Outcome::Done(Value::UInt(8));
    right.records[1].rule_start_line = 108;
    assert_difference(
        left.view(),
        right.view(),
        TraceDifference::Record {
            index: 0,
            field: RecordField::After(SnapshotField::Outcome),
        },
    );
}

#[test]
fn record_order_is_rule_then_charge_then_before_then_after() {
    let left = literal_seven();
    let mut right = literal_seven();
    right.records[0].rule_start_line = 108;
    right.records[0].charge = 2;
    right.records[0].before.outcome = Outcome::Done(Value::UInt(8));
    right.records[0].after.program = literal_program(8);
    assert_difference(
        left.view(),
        right.view(),
        TraceDifference::Record {
            index: 0,
            field: RecordField::RuleStartLine,
        },
    );
    right.records[0].rule_start_line = 102;
    assert_difference(
        left.view(),
        right.view(),
        TraceDifference::Record {
            index: 0,
            field: RecordField::Charge,
        },
    );
    right.records[0].charge = 1;
    assert_difference(
        left.view(),
        right.view(),
        TraceDifference::Record {
            index: 0,
            field: RecordField::Before(SnapshotField::Outcome),
        },
    );
}

#[test]
fn initial_difference_precedes_record_difference() {
    let left = literal_seven();
    let mut right = literal_seven();
    right.initial.outcome = Outcome::Done(Value::UInt(8));
    right.records[0].rule_start_line = 108;
    assert_difference(
        left.view(),
        right.view(),
        TraceDifference::Initial(SnapshotField::Outcome),
    );
}

#[test]
fn prefix_length_difference_is_not_a_match() {
    let left = literal_seven();
    let right = literal_seven();
    let prefix = TraceView {
        records: &right.records[..1],
        ..right.view()
    };
    assert_difference(
        left.view(),
        prefix,
        TraceDifference::Length {
            common_records: 1,
            left_records: 2,
            right_records: 1,
        },
    );
    assert_difference(
        prefix,
        left.view(),
        TraceDifference::Length {
            common_records: 1,
            left_records: 1,
            right_records: 2,
        },
    );
    let empty = TraceView {
        records: &[],
        ..right.view()
    };
    assert_difference(
        left.view(),
        empty,
        TraceDifference::Length {
            common_records: 0,
            left_records: 2,
            right_records: 0,
        },
    );
    assert_eq!(first_difference(empty, empty), None);
}

#[test]
fn common_prefix_difference_precedes_length_difference() {
    let left = literal_seven();
    let mut right = literal_seven();
    right.records.truncate(1);
    right.records[0].after.remaining_fuel = 3;
    assert_difference(
        left.view(),
        right.view(),
        TraceDifference::Record {
            index: 0,
            field: RecordField::After(SnapshotField::RemainingFuel),
        },
    );
}

#[test]
fn legitimate_distinct_programs_and_fuel_are_observable() {
    let left = literal_seven();
    // A separately constructed normal literal-eight initial state.
    let right = Snapshot {
        program: literal_program(8),
        control: Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(8))))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 1,
        initial_fuel: 1,
        outcome: Outcome::Pending,
    };
    let right_view = TraceView {
        identity: IDENTITY,
        initial: &right,
        records: &[],
    };
    assert_difference(
        left.view(),
        right_view,
        TraceDifference::Initial(SnapshotField::Program),
    );
    // Same normal input with fuel two, an independent legal startup observation.
    let fueled = Snapshot {
        program: literal_program(7),
        control: Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 2,
        initial_fuel: 2,
        outcome: Outcome::Pending,
    };
    let fueled_view = TraceView {
        identity: IDENTITY,
        initial: &fueled,
        records: &[],
    };
    assert_difference(
        left.view(),
        fueled_view,
        TraceDifference::Initial(SnapshotField::RemainingFuel),
    );
}

#[test]
fn returned_position_keeps_original_typed_values_accessible() {
    let left = literal_seven();
    let mut right = literal_seven();
    right.records[1].after.outcome = Outcome::Done(Value::UInt(8));
    let position = first_difference(left.view(), right.view()).expect("different outcomes");
    let TraceDifference::Record {
        index,
        field: RecordField::After(SnapshotField::Outcome),
    } = position
    else {
        panic!("expected the terminal outcome position, got {position:?}");
    };
    assert_eq!(
        left.records[index].after.outcome,
        Outcome::Done(Value::UInt(7))
    );
    assert_eq!(
        right.records[index].after.outcome,
        Outcome::Done(Value::UInt(8))
    );
}

#[test]
fn program_function_order_is_structural_observation() {
    let function = |name: &str| Function {
        name: name.into(),
        parameters: vec![],
        return_type: Type::UInt,
        body: Arc::new(Expr::Literal(Value::UInt(7))),
    };
    let mut left = literal_seven();
    left.initial.program = Arc::new(Program {
        functions: vec![function("10"), function("11")],
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    let mut right = literal_seven();
    right.initial.program = Arc::new(Program {
        functions: vec![function("11"), function("10")],
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    assert_difference(
        left.view(),
        right.view(),
        TraceDifference::Initial(SnapshotField::Program),
    );
}
