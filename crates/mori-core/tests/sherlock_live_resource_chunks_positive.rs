//! Proposed fixed live A collection check; SOURCE_ONLY / UNCOMPILED / UNRUN.
//! Typed expected states/slices are unchanged literal R130/R73/R131 data;
//! report goldens are the same complete four independently frozen R131 files.
//! Two Machine instances collect real records: one advance_m(2), one 1 then 1.
//! No expected state, charge or report is derived from either actual machine.
//! This new exact positive case needs fresh classification before execution.
use std::sync::Arc;
use mori_core::sherlock::{
    Control, Environment, Expr, Machine, Outcome, Program, Snapshot, StepRecord, Value,
};
use mori_core::sherlock::reference_slice::{
    advance_m, cumulative_resource_report, ReferenceSlice, SliceBoundary, SliceBoundaryKind, SliceStop,
};

fn states() -> (Snapshot, Snapshot, Snapshot) {
    let program = Arc::new(Program {
        functions: vec![], entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    let initial = Snapshot {
        program: program.clone(),
        control: Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))),
        environment: Environment::new(), frames: vec![], stack_limit: 8,
        remaining_fuel: 1, initial_fuel: 1, outcome: Outcome::Pending,
    };
    let pending = Snapshot {
        program: program.clone(), control: Some(Control::Return(Value::UInt(7))),
        environment: Environment::new(), frames: vec![], stack_limit: 8,
        remaining_fuel: 0, initial_fuel: 1, outcome: Outcome::Pending,
    };
    let done = Snapshot {
        program, control: None, environment: Environment::new(), frames: vec![],
        stack_limit: 8, remaining_fuel: 0, initial_fuel: 1,
        outcome: Outcome::Done(Value::UInt(7)),
    };
    (initial, pending, done)
}

fn slices() -> (ReferenceSlice, ReferenceSlice, ReferenceSlice) {
    let (initial, pending, done) = states();
    let literal = StepRecord {
        rule_start_line: 102, charge: 1, before: initial.clone(), after: pending.clone(),
    };
    let publication = StepRecord {
        rule_start_line: 80, charge: 0, before: pending.clone(), after: done.clone(),
    };
    let whole = ReferenceSlice {
        profile_id: "sherlock-k-a/1", start: initial.clone(),
        records: vec![literal.clone(), publication.clone()], prefix_work: vec![0, 1, 1],
        boundaries: vec![SliceBoundary { position: 2, kind: SliceBoundaryKind::Terminal }],
        end: done.clone(), stop: SliceStop::Outcome(Outcome::Done(Value::UInt(7))),
    };
    let first = ReferenceSlice {
        profile_id: "sherlock-k-a/1", start: initial, records: vec![literal],
        prefix_work: vec![0, 1],
        boundaries: vec![SliceBoundary { position: 1, kind: SliceBoundaryKind::TechnicalCut }],
        end: pending.clone(), stop: SliceStop::TechnicalCut,
    };
    let second = ReferenceSlice {
        profile_id: "sherlock-k-a/1", start: pending, records: vec![publication],
        prefix_work: vec![0, 0],
        boundaries: vec![SliceBoundary { position: 1, kind: SliceBoundaryKind::Terminal }],
        end: done, stop: SliceStop::Outcome(Outcome::Done(Value::UInt(7))),
    };
    (whole, first, second)
}

#[test]
fn live_n01_whole_trace_and_two_chunks_keep_complete_slices_and_reports() {
    let expected_whole: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-WHOLE-REPORT.json")).unwrap();
    let expected_first: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-FIRST-REPORT.json")).unwrap();
    let expected_second: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-SECOND-REPORT.json")).unwrap();
    let expected_cumulative: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-CUMULATIVE-REPORT.json")).unwrap();
    // Entire expected Snapshot/StepRecord/ReferenceSlice values are independent
    // frozen literals above, not a projection or reconstruction of actual output.
    let (wanted_whole, wanted_first, wanted_second) = slices();
    let program = Arc::new(Program {
        functions: vec![], entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    let mut whole_machine = Machine::new(program.clone(), 1, 8);
    let mut chunk_machine = Machine::new(program, 1, 8);
    assert_eq!(whole_machine.snapshot(), wanted_whole.start);
    assert_eq!(chunk_machine.snapshot(), wanted_first.start);

    let whole = advance_m(&mut whole_machine, 2);
    assert_eq!(whole, wanted_whole, "complete live whole slice");
    assert_eq!(whole_machine.snapshot(), wanted_whole.end);

    let first = advance_m(&mut chunk_machine, 1);
    assert_eq!(first, wanted_first, "complete live first technical chunk");
    assert_eq!(chunk_machine.snapshot(), wanted_first.end);
    // Resume the same second machine, retaining pending Return at zero fuel.
    // The existing next free publication is included in the second chunk.
    let second = advance_m(&mut chunk_machine, 1);
    assert_eq!(second, wanted_second, "complete live second terminal chunk");
    assert_eq!(chunk_machine.snapshot(), wanted_second.end);
    assert_eq!(first.end, second.start, "full live inter-chunk boundary");

    let before_reporting = (whole.clone(), first.clone(), second.clone());
    let whole_state_before_reporting = whole_machine.snapshot();
    let chunk_state_before_reporting = chunk_machine.snapshot();
    let whole_report = whole.resource_report().unwrap();
    let first_report = first.resource_report().unwrap();
    let second_report = second.resource_report().unwrap();
    let cumulative = cumulative_resource_report(&[&first, &second]).unwrap();
    assert_eq!(serde_json::to_value(&whole_report).unwrap(), expected_whole);
    assert_eq!(serde_json::to_value(first_report).unwrap(), expected_first);
    assert_eq!(serde_json::to_value(second_report).unwrap(), expected_second);
    assert_eq!(serde_json::to_value(&cumulative).unwrap(), expected_cumulative);
    assert_eq!(whole_report, cumulative, "complete whole/two-chunk cumulative report");
    assert_eq!((whole, first, second), before_reporting,
        "reporting preserves all collected slices and input fields");
    assert_eq!(whole_machine.snapshot(), whole_state_before_reporting);
    assert_eq!(chunk_machine.snapshot(), chunk_state_before_reporting);
}
