//! SOURCE_ONLY / UNRUN: actual owning Core/2 producer and shared publisher.
//! Whole independent expectations were frozen before this implementation.
//! This positive test selects no negative/adversarial/security case.
use mori_core::sherlock_financial_projection::{
    self as projection, InvocationInput, ProducedInvocation,
};
use mori_core::stage::{self, Continuation, StageEvent, StageOutcome};
use mori_core::{Scenario, UInt};
use serde_json::{Value, json};

const SOURCE: &str = include_str!("fixtures/financial-producer-integration/inputs/retained-payment.mori");
const SCENARIO: &str = include_str!("fixtures/financial-producer-integration/inputs/scenario.json");
const INPUTS: &str = include_str!("fixtures/financial-producer-integration/expected/INPUTS.json");
const PROGRAM: &str = include_str!("fixtures/financial-producer-integration/expected/PROGRAM.json");
const PLAN: &str = include_str!("fixtures/financial-producer-integration/expected/PLAN.json");

struct Golden {
    result: &'static str,
    accounting: &'static str,
    publication: &'static str,
}
const START: Golden = Golden {
    result: include_str!("fixtures/financial-producer-integration/expected/START-RESULT.json"),
    accounting: include_str!("fixtures/financial-producer-integration/expected/START-ACCOUNTING.json"),
    publication: include_str!("fixtures/financial-producer-integration/expected/START-PUBLICATION.json"),
};
const FALSE: Golden = Golden {
    result: include_str!("fixtures/financial-producer-integration/expected/FALSE-RESULT.json"),
    accounting: include_str!("fixtures/financial-producer-integration/expected/FALSE-ACCOUNTING.json"),
    publication: include_str!("fixtures/financial-producer-integration/expected/FALSE-PUBLICATION.json"),
};
const TRUE: Golden = Golden {
    result: include_str!("fixtures/financial-producer-integration/expected/TRUE-RESULT.json"),
    accounting: include_str!("fixtures/financial-producer-integration/expected/TRUE-ACCOUNTING.json"),
    publication: include_str!("fixtures/financial-producer-integration/expected/TRUE-PUBLICATION.json"),
};
fn frozen(text: &str) -> Value { serde_json::from_str(text).expect("frozen complete JSON") }
fn waiting<'a>(produced: &'a ProducedInvocation<'_>) -> &'a Continuation {
    match produced.raw_result() {
        Ok(StageOutcome::Waiting { continuation, .. }) => continuation,
        _ => panic!("fixed positive invocation must be waiting"),
    }
}
fn compare(
    produced: &ProducedInvocation<'_>, program: &stage::StagedProgram,
    expected_input: Value, golden: &Golden,
) {
    let raw = frozen(golden.result);
    assert_eq!(serde_json::to_value(produced.raw_result()).unwrap(), raw,
        "complete independently frozen raw result including all state/steps/frames");
    let view = produced.observation();
    assert!(std::ptr::eq(view.program, program));
    assert!(std::ptr::eq(view.result, produced.raw_result()));
    // The frozen plan records the program digest; each raw waiting result
    // records the independently derived plan digest. The final call reuses it.
    let expected_program_hash = frozen(PLAN)["program_hash"].clone();
    let expected_plan_hash = frozen(START.result)["Ok"]["Waiting"]["continuation"]["plan_hash"].clone();
    let expected = json!({
        "program": {"program": frozen(PROGRAM), "plan": frozen(PLAN),
            "program_hash": expected_program_hash, "plan_hash": expected_plan_hash},
        "input": expected_input, "result": raw,
        "outcome": raw["Ok"].clone(),
        "accounting": frozen(golden.accounting),
        "tentative_publication": frozen(golden.publication),
    });
    assert_eq!(serde_json::to_value(&view).unwrap(), expected,
        "every serialized observation field, complete ordered values");
    // Use the exact public publisher called by the real Core/2 simulate branch.
    // Expected legacy keys come only from complete frozen raw-result fields.
    let mut report = match &raw["Ok"] {
        Value::Object(outcome) if outcome.contains_key("Waiting") => {
            let mut value = outcome["Waiting"].as_object().unwrap().clone();
            value.insert("status".into(), json!("waiting"));
            value.insert("committed".into(), value["continuation"]["logical"]["committed"].clone());
            Value::Object(value)
        }
        Value::Object(outcome) if outcome.contains_key("Complete") => {
            let mut value = outcome["Complete"].as_object().unwrap().clone();
            value.insert("status".into(), json!("complete"));
            Value::Object(value)
        }
        _ => panic!("positive frozen publication must be waiting or complete"),
    };
    let object = report.as_object_mut().unwrap();
    object.insert("financial_observation".into(), expected);
    object.insert("qualification".into(), json!("local-reference-stage"));
    object.insert("authentication".into(), json!("none"));
    object.insert("consumption".into(), json!("untracked"));
    assert_eq!(produced.publication().unwrap(), report,
        "complete mandatory publication and preserved legacy keys/labels");
}

#[test]
fn owning_core2_producer_publishes_complete_frozen_financial_sequence() {
    let program = stage::load(SOURCE).expect("retained positive financial source");
    let scenario: Scenario = serde_json::from_str(SCENARIO).unwrap();
    let input = frozen(INPUTS);
    let action = input["action"].as_str().unwrap().to_owned();
    let start_cap: Option<UInt> = serde_json::from_value(input["start_cap"].clone()).unwrap();
    let cli_action: Option<String> = serde_json::from_value(input["resume_cli_action"].clone()).unwrap();
    let cli_cap: Option<UInt> = serde_json::from_value(input["resume_cli_cap"].clone()).unwrap();
    let events: Vec<StageEvent> = serde_json::from_value(input["events"].clone()).unwrap();
    assert_eq!(events.len(), 2);
    let start = projection::start_observed(&program, action.clone(), scenario.clone(), start_cap);
    compare(&start, &program, json!({"Start": {
        "action": action, "scenario": frozen(SCENARIO), "cap": input["start_cap"]
    }}), &START);
    match start.observation().input {
        InvocationInput::Start { action: actual_action, scenario: actual_scenario, cap } => {
            assert_eq!(actual_action, action);
            assert_eq!(actual_scenario, &scenario);
            assert_eq!(cap, start_cap);
        }
        _ => panic!("actual start capture"),
    }
    let false_input = waiting(&start).clone();
    let suspended = projection::resume_observed(&program, false_input.clone(), events[0].clone(), cli_action.clone(), cli_cap);
    compare(&suspended, &program, json!({"Resume": {
        "continuation": frozen(START.result)["Ok"]["Waiting"]["continuation"],
        "event": input["events"][0], "cli_action": input["resume_cli_action"],
        "cli_cap": input["resume_cli_cap"]
    }}), &FALSE);
    assert_eq!(waiting(&suspended), &false_input);
    let final_input = waiting(&suspended).clone();
    let complete = projection::resume_observed(&program, final_input.clone(), events[1].clone(), cli_action, cli_cap);
    compare(&complete, &program, json!({"Resume": {
        "continuation": frozen(FALSE.result)["Ok"]["Waiting"]["continuation"],
        "event": input["events"][1], "cli_action": input["resume_cli_action"],
        "cli_cap": input["resume_cli_cap"]
    }}), &TRUE);
    match complete.observation().input {
        InvocationInput::Resume { continuation, event, cli_action, cli_cap: captured_cap } => {
            assert_eq!(continuation, &final_input);
            assert_eq!(event, &events[1]);
            assert_eq!(cli_action, None);
            assert_eq!(captured_cap, cli_cap);
        }
        _ => panic!("actual resume capture"),
    }
    assert!(matches!(complete.raw_result(), Ok(StageOutcome::Complete { .. })));
}
