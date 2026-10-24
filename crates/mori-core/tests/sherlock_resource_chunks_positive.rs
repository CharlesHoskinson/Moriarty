//! Ordinary supplied-data reporting checks using frozen R73 N01 records.
//! All four complete report expectations were independently frozen in R131.
//! The original source passed once in R150; see the actual result and fresh
//! review at docs/results/sherlock-resource-chunks-20261017/. This promotion
//! changes only comments; the executed body below is byte-identical.
//! No Machine is constructed and no evaluator, step/run or SDK is called.
use mori_core::sherlock::reference_slice::{
    ReferenceSlice, SliceBoundary, SliceBoundaryKind, SliceStop, cumulative_resource_report,
};
use mori_core::sherlock::{
    Control, Environment, Expr, Outcome, Program, Snapshot, StepRecord, Value,
};
use std::sync::Arc;

fn states() -> (Snapshot, Snapshot, Snapshot) {
    let program = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
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
    let pending = Snapshot {
        program: program.clone(),
        control: Some(Control::Return(Value::UInt(7))),
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 1,
        outcome: Outcome::Pending,
    };
    let done = Snapshot {
        program,
        control: None,
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 8,
        remaining_fuel: 0,
        initial_fuel: 1,
        outcome: Outcome::Done(Value::UInt(7)),
    };
    (initial, pending, done)
}

fn slices() -> (ReferenceSlice, ReferenceSlice, ReferenceSlice) {
    let (initial, pending, done) = states();
    let literal = StepRecord {
        rule_start_line: 102,
        charge: 1,
        before: initial.clone(),
        after: pending.clone(),
    };
    let publication = StepRecord {
        rule_start_line: 80,
        charge: 0,
        before: pending.clone(),
        after: done.clone(),
    };
    let whole = ReferenceSlice {
        profile_id: "sherlock-k-a/1",
        start: initial.clone(),
        records: vec![literal.clone(), publication.clone()],
        prefix_work: vec![0, 1, 1],
        boundaries: vec![SliceBoundary {
            position: 2,
            kind: SliceBoundaryKind::Terminal,
        }],
        end: done.clone(),
        stop: SliceStop::Outcome(Outcome::Done(Value::UInt(7))),
    };
    let first = ReferenceSlice {
        profile_id: "sherlock-k-a/1",
        start: initial,
        records: vec![literal],
        prefix_work: vec![0, 1],
        boundaries: vec![SliceBoundary {
            position: 1,
            kind: SliceBoundaryKind::TechnicalCut,
        }],
        end: pending.clone(),
        stop: SliceStop::TechnicalCut,
    };
    let second = ReferenceSlice {
        profile_id: "sherlock-k-a/1",
        start: pending,
        records: vec![publication],
        prefix_work: vec![0, 0],
        boundaries: vec![SliceBoundary {
            position: 1,
            kind: SliceBoundaryKind::Terminal,
        }],
        end: done,
        stop: SliceStop::Outcome(Outcome::Done(Value::UInt(7))),
    };
    (whole, first, second)
}

#[test]
fn one_trace_and_two_chunks_keep_the_independent_complete_resource_report() {
    let expected_whole: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-WHOLE-REPORT.json"
    ))
    .unwrap();
    let expected_cumulative: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-CUMULATIVE-REPORT.json"
    ))
    .unwrap();
    let expected_first: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-FIRST-REPORT.json"
    ))
    .unwrap();
    let expected_second: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/resource-chunks-n01/EXPECTED-SECOND-REPORT.json"
    ))
    .unwrap();
    let (whole, first, second) = slices();
    let original = (whole.clone(), first.clone(), second.clone());
    assert_eq!(
        serde_json::to_value(whole.resource_report().unwrap()).unwrap(),
        expected_whole
    );
    assert_eq!(
        serde_json::to_value(first.resource_report().unwrap()).unwrap(),
        expected_first
    );
    assert_eq!(
        serde_json::to_value(second.resource_report().unwrap()).unwrap(),
        expected_second
    );
    let cumulative = cumulative_resource_report(&[&first, &second]).unwrap();
    assert_eq!(
        serde_json::to_value(&cumulative).unwrap(),
        expected_cumulative
    );
    assert_eq!(whole.resource_report().unwrap(), cumulative);
    assert_eq!(
        (whole, first, second),
        original,
        "reporting preserves every supplied slice field"
    );
}
