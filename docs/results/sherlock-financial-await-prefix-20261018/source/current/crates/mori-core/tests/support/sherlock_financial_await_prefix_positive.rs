//! New explicit external ordinary QA consumer; SOURCE ONLY / UNRUN.
//! Complete independent publication/prefix fixtures were frozen before execution.
//! Exactly one real start and two real resumes; no diagnostic projector supplies
//! substitute results. Core/2 local projection is not universal018 completion.
use mori_core::sherlock_financial_projection::{
    self as projection, AwaitPrefixObservation, InvocationInput, ProducedInvocation,
    SegmentObservation,
};
use mori_core::stage::{self, Continuation, StageEvent, StageOutcome, StageSegment};
use mori_core::{Scenario, UInt};
use serde_json::Value;

const SOURCE: &str = include_str!("fixtures/financial-await-projection/retained-payment.mori");
const SCENARIO: &str = include_str!("fixtures/financial-await-projection/scenario.json");
const INPUTS: &str = include_str!("fixtures/financial-await-projection/invocations.json");
struct Golden {
    publication: &'static str,
    prefix: &'static str,
}
const START: Golden = Golden {
    publication: include_str!("fixtures/financial-await-projection/EXPECTED-START-PUBLICATION.json"),
    prefix: include_str!("fixtures/financial-await-projection/EXPECTED-START-AWAIT-PREFIX.json"),
};
const FALSE: Golden = Golden {
    publication: include_str!("fixtures/financial-await-projection/EXPECTED-FALSE-AWAIT-PUBLICATION.json"),
    prefix: include_str!("fixtures/financial-await-projection/EXPECTED-FALSE-AWAIT-PREFIX.json"),
};
const TRUE: Golden = Golden {
    publication: include_str!("fixtures/financial-await-projection/EXPECTED-TRUE-RESUME-PUBLICATION.json"),
    prefix: include_str!("fixtures/financial-await-projection/EXPECTED-TRUE-RESUME-AWAIT-PREFIX.json"),
};
fn frozen(text: &str) -> Value { serde_json::from_str(text).expect("complete independent JSON") }
fn whole(actual: Value, expected: &Value, call: &str, layer: &str) {
    // Value equality checks complete keys, types, scalar values and ordered
    // arrays in memory. Failure does not dump huge objects into bounded streams.
    assert!(actual == *expected, "{call}: entire {layer} differs from independent fixture");
}
fn waiting<'a>(produced: &'a ProducedInvocation<'_>) -> &'a Continuation {
    match produced.raw_result() {
        Ok(StageOutcome::Waiting { continuation, .. }) => continuation,
        _ => panic!("positive call must return actual Waiting continuation"),
    }
}
fn same_segment(view: SegmentObservation<'_>, raw: &StageSegment) {
    assert!(std::ptr::eq(view.action, raw.action.as_str()));
    assert!(std::ptr::eq(view.frame_pre, &raw.frame_pre));
    assert!(std::ptr::eq(view.frame_post, &raw.frame_post));
    assert!(std::ptr::eq(view.execution.semantics, raw.execution.semantics.as_str()));
    assert!(std::ptr::eq(view.execution.action, raw.execution.action.as_str()));
    assert!(std::ptr::eq(view.execution.pre, &raw.execution.pre));
    assert!(std::ptr::eq(view.execution.post, &raw.execution.post));
    assert!(std::ptr::eq(view.execution.steps, raw.execution.steps.as_slice()));
    assert!(std::ptr::eq(view.execution.effects, raw.execution.effects.as_slice()));
}
fn compare_call(produced: &ProducedInvocation<'_>, program: &stage::StagedProgram,
    golden: &Golden, label: &str)
{
    let expected_publication = frozen(golden.publication);
    let expected_prefix = frozen(golden.prefix);
    let view = produced.observation();
    assert!(std::ptr::eq(view.program, program));
    assert!(std::ptr::eq(view.result, produced.raw_result()));
    // All original017 program/plan/hash/input/raw-result/accounting/steps/effects
    // fields are checked directly against full standalone literal expectations.
    whole(serde_json::to_value(produced.raw_result()).unwrap(),
        &expected_publication["financial_observation"]["result"], label, "raw result");
    whole(serde_json::to_value(&view).unwrap(),
        &expected_publication["financial_observation"], label, "financial observation");
    let prefix = view.await_prefix();
    // Independent prefix literal, not comparison to publication's own new key.
    whole(serde_json::to_value(prefix).unwrap(), &expected_prefix, label, "typed await prefix");
    // The complete real publisher called by simulate, without key removal,
    // array sorting, subset matching, expected-from-actual construction or rerun.
    let publication = produced.publication().expect("complete public JSON");
    whole(publication, &expected_publication, label, "publication");
    // Prove these ordinary views borrow the SAME returned raw fields; this does
    // not assert authenticated origin, consumed events or financial refinement.
    match (prefix, produced.raw_result()) {
        (AwaitPrefixObservation::Waiting { advanced, action, site, captures,
            logical, committed_boundary, residual, published_segment, attempted },
            Ok(StageOutcome::Waiting { advanced: raw_advanced, segment,
                continuation, attempted: raw_attempted, residual: raw_residual })) => {
            assert_eq!(advanced, *raw_advanced);
            assert!(std::ptr::eq(action, continuation.action.as_str()));
            assert_eq!(site, continuation.site);
            assert!(std::ptr::eq(captures, continuation.captures.as_slice()));
            assert!(std::ptr::eq(logical, &continuation.logical));
            assert!(std::ptr::eq(committed_boundary, &continuation.boundary));
            assert!(std::ptr::eq(residual, raw_residual.as_slice()));
            assert_eq!(attempted, *raw_attempted);
            match (published_segment, segment.as_ref()) {
                (Some(projected), Some(raw)) => same_segment(projected, raw),
                (None, None) => {},
                _ => panic!("returned segment absence is not an empty successful segment"),
            }
        },
        (AwaitPrefixObservation::Complete, Ok(StageOutcome::Complete { .. })) => {},
        _ => panic!("positive typed prefix must correspond to actual returned outcome"),
    }
}

#[test]
fn real_core2_three_calls_preserve_complete_financial_await_publications() {
    let program = stage::load(SOURCE).expect("independent positive payment source");
    let scenario: Scenario = serde_json::from_str(SCENARIO).unwrap();
    let inputs = frozen(INPUTS);
    let action = inputs["action"].as_str().unwrap().to_owned();
    let cap: Option<UInt> = serde_json::from_value(inputs["start_cap"].clone()).unwrap();
    let cli_action: Option<String> = serde_json::from_value(inputs["resume_cli_action"].clone()).unwrap();
    let cli_cap: Option<UInt> = serde_json::from_value(inputs["resume_cli_cap"].clone()).unwrap();
    let events: Vec<StageEvent> = serde_json::from_value(inputs["events"].clone()).unwrap();
    assert_eq!(events.len(), 2);
    // Exactly three genuine producer calls, retaining each owning raw result.
    let start = projection::start_observed(&program, action.clone(), scenario.clone(), cap);
    compare_call(&start, &program, &START, "START");
    match start.observation().input {
        InvocationInput::Start { action: captured_action, scenario: captured_scenario, cap: captured_cap } => {
            assert_eq!(captured_action, action);
            assert_eq!(captured_scenario, &scenario);
            assert_eq!(captured_cap, cap);
        },
        _ => panic!("actual captured start input"),
    }
    let first_returned = waiting(&start).clone();
    let suspended = projection::resume_observed(&program, first_returned.clone(),
        events[0].clone(), cli_action.clone(), cli_cap);
    compare_call(&suspended, &program, &FALSE, "FALSE");
    match suspended.observation().input {
        InvocationInput::Resume { continuation, event, cli_action: captured_action, cli_cap: captured_cap } => {
            assert_eq!(continuation, &first_returned);
            assert_eq!(event, &events[0]);
            assert_eq!(captured_action, cli_action.as_deref());
            assert_eq!(captured_cap, cli_cap);
        },
        _ => panic!("actual captured false resume input"),
    }
    assert_eq!(waiting(&suspended), &first_returned,
        "false guard preserves the complete actual returned continuation");
    let second_returned = waiting(&suspended).clone();
    let complete = projection::resume_observed(&program, second_returned.clone(),
        events[1].clone(), cli_action.clone(), cli_cap);
    compare_call(&complete, &program, &TRUE, "TRUE");
    match complete.observation().input {
        InvocationInput::Resume { continuation, event, cli_action: captured_action, cli_cap: captured_cap } => {
            assert_eq!(continuation, &second_returned);
            assert_eq!(event, &events[1]);
            assert_eq!(captured_action, cli_action.as_deref());
            assert_eq!(captured_cap, cli_cap);
        },
        _ => panic!("actual captured true resume input"),
    }
    assert!(matches!(complete.raw_result(), Ok(StageOutcome::Complete { .. })));
    // All three real owning records remain live; every full financial snapshot,
    // ordered step/effect, committed boundary and residual was compared above.
    // No output file, CLI, second producer pass or universal-machine run is used.
}
