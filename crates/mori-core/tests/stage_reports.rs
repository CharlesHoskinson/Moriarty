//! Complete Core/2 outcome reports against independent literal envelopes.
use mori_core::stage::{self, Continuation, StageOutcome};
use mori_core::{Scenario, UInt};
use serde_json::Value;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/core-stage-reports/",
            $name
        ))
    };
}

fn case(name: &str) -> Value {
    let manifest: Value = serde_json::from_str(fixture!("mori-tests.json")).unwrap();
    assert_eq!(manifest["profile"], "moriarty-tests/2");
    manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == name)
        .unwrap()
        .clone()
}

fn checked_report(outcome: StageOutcome, expected_text: &str) -> Option<Continuation> {
    let pending = match &outcome {
        StageOutcome::Waiting { continuation, .. } => Some(continuation.clone()),
        StageOutcome::Complete { .. } => None,
    };
    let actual = stage::outcome_report(Ok(outcome));
    let expected: Value = serde_json::from_str(expected_text).unwrap();
    assert_eq!(actual, expected);

    // Compare the complete new canonical-serialization regression directly to
    // unchanged fixture bytes; expected data is never serialized from actuals.
    let mut bytes = serde_json::to_vec_pretty(&actual).unwrap();
    bytes.push(b'\n');
    assert_eq!(bytes, expected_text.as_bytes());
    pending
}

#[test]
fn two_deliveries_full_reports_and_bytes_follow_actual_continuations() {
    let case = case("two-deliveries");
    let action = case["action"].as_str().unwrap();
    let cap: Option<UInt> = serde_json::from_value(case["logical_cap"].clone()).unwrap();
    let program = stage::load(fixture!("two-deliveries.mori")).unwrap();
    let scenario: Scenario =
        serde_json::from_str(fixture!("two-deliveries-scenario.json")).unwrap();
    let started = stage::start(&program, action, scenario, cap).unwrap();
    let first = checked_report(started, fixture!("two-deliveries-start.json"))
        .expect("start must yield its actual waiting continuation");

    let before_observation = first.clone();
    let event = serde_json::from_str(fixture!("approval-false.json")).unwrap();
    let observed = stage::resume(&program, first, event, Some(action), None).unwrap();
    let observed = checked_report(observed, fixture!("two-deliveries-observed.json"))
        .expect("false observation must retain an actual waiting continuation");
    assert_eq!(observed, before_observation);

    let event = serde_json::from_str(fixture!("approval-true.json")).unwrap();
    let approved = stage::resume(&program, observed, event, Some(action), None).unwrap();
    let approved = checked_report(approved, fixture!("two-deliveries-approved.json"))
        .expect("approval must yield the next actual waiting continuation");

    let event = serde_json::from_str(fixture!("receipt-true.json")).unwrap();
    let complete = stage::resume(&program, approved, event, Some(action), None).unwrap();
    assert!(checked_report(complete, fixture!("two-deliveries-complete.json")).is_none());
}

#[test]
fn quoted_fee_full_reports_and_bytes_preserve_actual_captured_amount() {
    let case = case("quoted-fee");
    let action = case["action"].as_str().unwrap();
    let cap: Option<UInt> = serde_json::from_value(case["logical_cap"].clone()).unwrap();
    let program = stage::load(fixture!("quoted-fee.mori")).unwrap();
    let scenario: Scenario = serde_json::from_str(fixture!("quoted-fee-scenario.json")).unwrap();
    let started = stage::start(&program, action, scenario, cap).unwrap();
    let pending = checked_report(started, fixture!("quoted-fee-start.json"))
        .expect("start must yield its actual waiting continuation");

    let event = serde_json::from_str(fixture!("quote-true.json")).unwrap();
    let complete = stage::resume(&program, pending, event, Some(action), None).unwrap();
    assert!(checked_report(complete, fixture!("quoted-fee-complete.json")).is_none());
}
