//! Proposed ordinary positive reference checks, not run.
//! Expectations are byte copies of the independent frozen oracle packet.
use mori_cardano::{StagedReferenceContext, prepare_staged_reference};
use mori_core::{UInt, stage::{Continuation, StageOutcome}};
use serde_json::Value;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("../../staged-fixtures/", $name))
    };
}

fn expected(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

fn metadata(source: &str, all: &str, program: &str, plan: &str) -> StagedReferenceContext {
    let context = prepare_staged_reference(source).unwrap();
    let actual = serde_json::to_value(context.metadata()).unwrap();
    // Program/Execution Deserialize is /1-only. Compare complete /2 values as
    // JSON; never decode the oracle Program into the legacy Program interface.
    assert_eq!(actual, expected(all));
    assert_eq!(actual["program"], expected(program));
    assert_eq!(actual["plan"], expected(plan));
    context
}

fn assert_outcome(actual: &StageOutcome, golden: &str) {
    // Exact field membership and all arrays/values, not selected summary fields.
    assert_eq!(serde_json::to_value(actual).unwrap(), expected(golden));
}

fn continuation(actual: StageOutcome, golden: &str) -> Continuation {
    match actual {
        StageOutcome::Waiting { continuation, .. } => {
            assert_eq!(serde_json::to_value(&continuation).unwrap(), expected(golden));
            continuation
        }
        StageOutcome::Complete { .. } => panic!("Expected independent pending boundary"),
    }
}

#[test]
fn delivery_complete_metadata_and_three_chained_positive_outcomes() {
    let context = metadata(
        fixture!("delivery.mori"),
        fixture!("delivery.expected-metadata.json"),
        fixture!("delivery.expected-program.json"),
        fixture!("delivery.expected-plan.json"),
    );
    let start = context.start_reference(
        "deliver",
        serde_json::from_str(fixture!("delivery-scenario.json")).unwrap(),
        Some(UInt(11)),
    ).unwrap();
    assert_outcome(&start, fixture!("delivery.start.expected.json"));
    let pending = continuation(start, fixture!("delivery.continuation-site1.json"));
    let not_advanced = context.resume_reference(
        pending,
        serde_json::from_str(fixture!("delivery-false-resume.json")).unwrap(),
        None,
        None,
    ).unwrap();
    assert_outcome(&not_advanced, fixture!("delivery.false.expected.json"));
    let pending = continuation(not_advanced, fixture!("delivery.continuation-site1.json"));
    let complete = context.resume_reference(
        pending,
        serde_json::from_str(fixture!("delivery-true-resume-complete.json")).unwrap(),
        None,
        None,
    ).unwrap();
    assert_outcome(&complete, fixture!("delivery.true.expected.json"));
}

#[test]
fn reserve_complete_metadata_and_five_chained_positive_outcomes() {
    let context = metadata(
        fixture!("reserve.mori"),
        fixture!("reserve.expected-metadata.json"),
        fixture!("reserve.expected-program.json"),
        fixture!("reserve.expected-plan.json"),
    );
    let start = context.start_reference(
        "allocate",
        serde_json::from_str(fixture!("reserve-scenario.json")).unwrap(),
        Some(UInt(36)),
    ).unwrap();
    assert_outcome(&start, fixture!("reserve.start.expected.json"));
    let pending = continuation(start, fixture!("reserve.continuation-site1.json"));
    let first_false = context.resume_reference(
        pending,
        serde_json::from_str(fixture!("reserve-first-false-resume.json")).unwrap(),
        None,
        None,
    ).unwrap();
    assert_outcome(&first_false, fixture!("reserve.first-false.expected.json"));
    let pending = continuation(first_false, fixture!("reserve.continuation-site1.json"));
    let first_true = context.resume_reference(
        pending,
        serde_json::from_str(fixture!("reserve-first-true-second-boundary.json")).unwrap(),
        None,
        None,
    ).unwrap();
    assert_outcome(&first_true, fixture!("reserve.first-true.expected.json"));
    let pending = continuation(first_true, fixture!("reserve.continuation-site2.json"));
    let second_false = context.resume_reference(
        pending,
        serde_json::from_str(fixture!("reserve-second-false-resume.json")).unwrap(),
        None,
        None,
    ).unwrap();
    assert_outcome(&second_false, fixture!("reserve.second-false.expected.json"));
    let pending = continuation(second_false, fixture!("reserve.continuation-site2.json"));
    let complete = context.resume_reference(
        pending,
        serde_json::from_str(fixture!("reserve-second-true-complete.json")).unwrap(),
        None,
        None,
    ).unwrap();
    assert_outcome(&complete, fixture!("reserve.second-true.expected.json"));
}
