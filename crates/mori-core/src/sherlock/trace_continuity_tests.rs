//! Ordinary complete-boundary/accounting checks, never transition evaluation.
//! N01 manually transcribes frozen EXPECTED-N01-FUEL1.json: Eval UInt7 ->
//! pending Return UInt7 (line102, charge1) -> Done UInt7 (line80, charge0).
//! The isolated harness imports only reference_types, compare and this module.

use super::super::reference_types::*;
use super::*;
use std::sync::Arc;

const IDENTITY: TraceIdentity<'static> = TraceIdentity {
    profile: "sherlock-k-a/1",
    source_sha256: "b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41",
};

struct LiteralTrace {
    initial: Snapshot,
    records: Vec<StepRecord>,
    returned: Snapshot,
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
fn program(value: u128) -> Arc<Program> {
    Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(value))),
    })
}
fn n01() -> LiteralTrace {
    let initial = Snapshot {
        program: program(7),
        control: Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 1,
        initial_fuel: 1,
        outcome: Outcome::Pending,
    };
    let pending = Snapshot {
        program: program(7),
        control: Some(Control::Return(Value::UInt(7))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 1,
        outcome: Outcome::Pending,
    };
    let returned = Snapshot {
        program: program(7),
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
        returned: returned.clone(),
        records: vec![
            StepRecord {
                rule_start_line: 102,
                charge: 1,
                before: initial,
                after: pending.clone(),
            },
            StepRecord {
                rule_start_line: 80,
                charge: 0,
                before: pending,
                after: returned,
            },
        ],
    }
}
fn changed(snapshot: &mut Snapshot, field: SnapshotField) {
    match field {
        SnapshotField::Program => snapshot.program = program(8),
        SnapshotField::Control => snapshot.control = Some(Control::Error(9)),
        SnapshotField::Environment => {
            snapshot.environment.insert("0".into(), Value::Bool(true));
        }
        SnapshotField::Frames => snapshot
            .frames
            .push(Frame::Right(BinaryOp::Add, Value::UInt(8))),
        SnapshotField::StackLimit => snapshot.stack_limit = 9,
        SnapshotField::RemainingFuel => snapshot.remaining_fuel += 1,
        SnapshotField::InitialFuel => snapshot.initial_fuel = 2,
        SnapshotField::Outcome => snapshot.outcome = Outcome::Rejected(9),
    }
}
fn initial_field(field: SnapshotField) {
    let mut trace = n01();
    changed(&mut trace.records[0].before, field);
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::InitialToFirst,
            field
        })
    );
}
fn between_field(field: SnapshotField) {
    let mut trace = n01();
    changed(&mut trace.records[1].before, field);
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::BetweenRecords {
                previous: 0,
                next: 1
            },
            field,
        })
    );
}
fn returned_field(field: SnapshotField) {
    let mut trace = n01();
    changed(&mut trace.returned, field);
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::LastToReturned { last: 1 },
            field,
        })
    );
}
fn empty_field(field: SnapshotField) {
    let mut trace = n01();
    trace.records.clear();
    trace.returned = trace.initial.clone();
    changed(&mut trace.returned, field);
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::EmptyToReturned,
            field
        })
    );
}
macro_rules! field_edges {
    ($field:ident, $initial:ident, $between:ident, $returned:ident, $empty:ident) => {
        #[test]
        fn $initial() {
            initial_field(SnapshotField::$field);
        }
        #[test]
        fn $between() {
            between_field(SnapshotField::$field);
        }
        #[test]
        fn $returned() {
            returned_field(SnapshotField::$field);
        }
        #[test]
        fn $empty() {
            empty_field(SnapshotField::$field);
        }
    };
}
field_edges!(
    Program,
    initial_program,
    between_program,
    returned_program,
    empty_program
);
field_edges!(
    Control,
    initial_control,
    between_control,
    returned_control,
    empty_control
);
field_edges!(
    Environment,
    initial_environment,
    between_environment,
    returned_environment,
    empty_environment
);
field_edges!(
    Frames,
    initial_frames,
    between_frames,
    returned_frames,
    empty_frames
);
field_edges!(
    StackLimit,
    initial_stack_limit,
    between_stack_limit,
    returned_stack_limit,
    empty_stack_limit
);
field_edges!(
    RemainingFuel,
    initial_remaining_fuel,
    between_remaining_fuel,
    returned_remaining_fuel,
    empty_remaining_fuel
);
field_edges!(
    InitialFuel,
    initial_initial_fuel,
    between_initial_fuel,
    returned_initial_fuel,
    empty_initial_fuel
);
field_edges!(
    Outcome,
    initial_outcome,
    between_outcome,
    returned_outcome,
    empty_outcome
);

#[test]
fn complete_literal_charged_and_free_publication_prefix_matches() {
    let trace = n01();
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        None
    );
    assert!(!Arc::ptr_eq(
        &trace.initial.program,
        &trace.records[0].after.program
    ));
}
#[test]
fn present_empty_running_sequence_matches_without_terminal_claim() {
    let trace = n01();
    let view = TraceView {
        records: &[],
        ..trace.view()
    };
    assert_eq!(first_continuity_difference(view, &trace.initial), None);
    assert_eq!(trace.initial.outcome, Outcome::Pending);
}
#[test]
fn present_empty_terminal_sequence_matches() {
    let trace = n01();
    let view = TraceView {
        initial: &trace.returned,
        records: &[],
        ..trace.view()
    };
    assert_eq!(first_continuity_difference(view, &trace.returned), None);
}
#[test]
fn matching_pending_prefix_needs_no_publication() {
    let trace = n01();
    let view = TraceView {
        records: &trace.records[..1],
        ..trace.view()
    };
    assert_eq!(
        first_continuity_difference(view, &trace.records[0].after),
        None
    );
    assert_eq!(trace.records[0].after.outcome, Outcome::Pending);
}
#[test]
fn one_record_wrong_returned_state_names_last_index_zero() {
    let trace = n01();
    let view = TraceView {
        records: &trace.records[..1],
        ..trace.view()
    };
    assert_eq!(
        first_continuity_difference(view, &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::LastToReturned { last: 0 },
            field: SnapshotField::Control,
        })
    );
}
#[test]
fn missing_literal_charge_is_an_exact_sum_mismatch() {
    let mut trace = n01();
    trace.records[0].charge = 0;
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::ChargeSumMismatch {
            recorded: 0,
            endpoint_delta: 1
        })
    );
}
#[test]
fn charging_the_free_publication_is_an_exact_sum_mismatch() {
    let mut trace = n01();
    trace.records[1].charge = 1;
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::ChargeSumMismatch {
            recorded: 2,
            endpoint_delta: 1
        })
    );
}
#[test]
fn all_u8_charge_values_are_summed_without_clamping() {
    let mut trace = n01();
    trace.records[0].charge = u8::MAX;
    trace.records[1].charge = u8::MAX;
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::ChargeSumMismatch {
            recorded: 510,
            endpoint_delta: 1
        })
    );
}
#[test]
fn endpoint_fuel_increase_is_explicitly_undefined() {
    let mut trace = n01();
    trace.records[0].after.remaining_fuel = 2;
    trace.records[1].before.remaining_fuel = 2;
    trace.records[1].after.remaining_fuel = 2;
    trace.returned.remaining_fuel = 2;
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::UndefinedFuelDelta)
    );
}
#[test]
fn complete_fields_use_existing_stable_priority() {
    let mut trace = n01();
    changed(&mut trace.records[0].before, SnapshotField::Outcome);
    changed(&mut trace.records[0].before, SnapshotField::Control);
    changed(&mut trace.records[0].before, SnapshotField::Program);
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::InitialToFirst,
            field: SnapshotField::Program,
        })
    );
}
#[test]
fn initial_edge_precedes_earlier_field_at_later_edge() {
    let mut trace = n01();
    changed(&mut trace.records[0].before, SnapshotField::Outcome);
    changed(&mut trace.records[1].before, SnapshotField::Program);
    changed(&mut trace.returned, SnapshotField::Program);
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::InitialToFirst,
            field: SnapshotField::Outcome,
        })
    );
}
#[test]
fn adjacent_edges_precede_returned_and_accounting_mismatches() {
    let mut trace = n01();
    changed(&mut trace.records[1].before, SnapshotField::Outcome);
    changed(&mut trace.returned, SnapshotField::Program);
    trace.records[0].charge = 0;
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::BetweenRecords {
                previous: 0,
                next: 1
            },
            field: SnapshotField::Outcome,
        })
    );
}
#[test]
fn ascending_adjacent_index_is_stable() {
    let mut trace = n01();
    trace.records.push(StepRecord {
        rule_start_line: 999,
        charge: 0,
        before: trace.returned.clone(),
        after: trace.returned.clone(),
    });
    changed(&mut trace.records[1].before, SnapshotField::Outcome);
    changed(&mut trace.records[2].before, SnapshotField::Program);
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::BetweenRecords {
                previous: 0,
                next: 1
            },
            field: SnapshotField::Outcome,
        })
    );
}
#[test]
fn returned_edge_precedes_undefined_fuel_delta() {
    let mut trace = n01();
    trace.returned.remaining_fuel = 2;
    assert_eq!(
        first_continuity_difference(trace.view(), &trace.returned),
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::LastToReturned { last: 1 },
            field: SnapshotField::RemainingFuel,
        })
    );
}
#[test]
fn matched_edges_and_total_do_not_validate_rules_or_authenticate_identity() {
    let mut trace = n01();
    trace.records[0].rule_start_line = 999;
    trace.records[0].charge = 0;
    trace.records[1].charge = 1;
    let view = TraceView {
        identity: TraceIdentity {
            profile: "caller-declaration",
            source_sha256: "unverified",
        },
        ..trace.view()
    };
    assert_eq!(first_continuity_difference(view, &trace.returned), None);
}
#[test]
fn diagnostics_retain_the_callers_raw_states() {
    let mut trace = n01();
    changed(&mut trace.records[1].before, SnapshotField::Environment);
    let saved_initial = trace.initial.clone();
    let saved_records = trace.records.clone();
    let saved_returned = trace.returned.clone();
    let difference = first_continuity_difference(trace.view(), &trace.returned);
    assert_eq!(
        difference,
        Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::BetweenRecords {
                previous: 0,
                next: 1
            },
            field: SnapshotField::Environment,
        })
    );
    assert_eq!(trace.initial, saved_initial);
    assert_eq!(trace.records, saved_records);
    assert_eq!(trace.returned, saved_returned);
    assert_ne!(
        trace.records[0].after.environment,
        trace.records[1].before.environment
    );
}
