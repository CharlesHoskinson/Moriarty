//! Fixed ordinary positive Core/2 integration.
//! Actual stage calls produce every observed result. R152 supplies all expected
//! result/accounting/program/plan schema; R175 independently revises its input
//! identities. The test author read the expectation schema only.
//! Borrow checks supplement independent JSON equality; they are not an oracle.
use mori_core::{Execution, Scenario, UInt};
use mori_core::stage::{self, Continuation, StageEvent, StageOutcome, StageRejection, StageSegment, StagedProgram};
use mori_core::sherlock_financial_projection::{
    self as projection, AccountingAvailability, ExecutionObservation, FinancialObservation,
    InvocationInput, InvocationObservation, SegmentObservation,
};
use serde_json::Value;

const SOURCE: &str = include_str!("fixtures/financial-observation/retained-payment.mori");
const SCENARIO: &str = include_str!("fixtures/financial-observation/scenario.json");
const INVOCATIONS: &str = include_str!("fixtures/financial-observation/invocations.json");
const EXPECTED_CONTEXT: &str = include_str!("fixtures/financial-observation/EXPECTED.json");
const EXPECTED_PROGRAM: &str = include_str!("fixtures/financial-observation/EXPECTED-PROGRAM.json");
const EXPECTED_PLAN: &str = include_str!("fixtures/financial-observation/EXPECTED-PLAN.json");

struct Golden {
    raw_result: &'static str,
    accounting: &'static str,
    tentative_publication: &'static str,
}
const START: Golden = Golden {
    raw_result: include_str!("fixtures/financial-observation/EXPECTED-START-RESULT.json"),
    accounting: include_str!("fixtures/financial-observation/EXPECTED-START-ACCOUNTING.json"),
    tentative_publication: include_str!("fixtures/financial-observation/EXPECTED-START-TENTATIVE-PUBLICATION.json"),
};
const FALSE_AWAIT: Golden = Golden {
    raw_result: include_str!("fixtures/financial-observation/EXPECTED-FALSE-AWAIT-RESULT.json"),
    accounting: include_str!("fixtures/financial-observation/EXPECTED-FALSE-AWAIT-ACCOUNTING.json"),
    tentative_publication: include_str!("fixtures/financial-observation/EXPECTED-FALSE-AWAIT-TENTATIVE-PUBLICATION.json"),
};
const TRUE_RESUME: Golden = Golden {
    raw_result: include_str!("fixtures/financial-observation/EXPECTED-TRUE-RESUME-RESULT.json"),
    accounting: include_str!("fixtures/financial-observation/EXPECTED-TRUE-RESUME-ACCOUNTING.json"),
    tentative_publication: include_str!("fixtures/financial-observation/EXPECTED-TRUE-RESUME-TENTATIVE-PUBLICATION.json"),
};
fn json(text: &str) -> Value { serde_json::from_str(text).expect("frozen JSON parses") }
fn hash_text(bytes: [u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

// The following checks compare borrowed views to their original raw arguments.
// They do not construct any golden or replace independent raw-result comparison.
fn assert_execution(view: &ExecutionObservation<'_>, execution: &Execution) {
    let ExecutionObservation { semantics, action, pre, post, steps, effects } = view;
    let Execution { semantics: raw_semantics, action: raw_action, pre: raw_pre,
        post: raw_post, steps: raw_steps, effects: raw_effects } = execution;
    assert!(std::ptr::eq(*semantics, raw_semantics.as_str()));
    assert!(std::ptr::eq(*action, raw_action.as_str()));
    assert!(std::ptr::eq(*pre, raw_pre));
    assert!(std::ptr::eq(*post, raw_post));
    assert!(std::ptr::eq(*steps, raw_steps.as_slice()));
    assert!(std::ptr::eq(*effects, raw_effects.as_slice()));
}
fn assert_segment(view: &SegmentObservation<'_>, segment: &StageSegment) {
    let SegmentObservation { action, frame_pre, frame_post, execution } = view;
    let StageSegment { action: raw_action, frame_pre: raw_pre, frame_post: raw_post,
        execution: raw_execution } = segment;
    assert!(std::ptr::eq(*action, raw_action.as_str()));
    assert!(std::ptr::eq(*frame_pre, raw_pre));
    assert!(std::ptr::eq(*frame_post, raw_post));
    assert_execution(execution, raw_execution);
}
fn assert_input(actual: InvocationInput<'_>, captured: InvocationInput<'_>) {
    match (actual, captured) {
        (InvocationInput::Start { action, scenario, cap },
         InvocationInput::Start { action: a, scenario: s, cap: c }) => {
            assert!(std::ptr::eq(action, a));
            assert!(std::ptr::eq(scenario, s));
            assert_eq!(cap, c);
        }
        (InvocationInput::Resume { continuation, event, cli_action, cli_cap },
         InvocationInput::Resume { continuation: c, event: e, cli_action: a, cli_cap: cap }) => {
            assert!(std::ptr::eq(continuation, c));
            assert!(std::ptr::eq(event, e));
            match (cli_action, a) {
                (Some(actual), Some(captured)) => assert!(std::ptr::eq(actual, captured)),
                (None, None) => {},
                _ => panic!("CLI action option changed"),
            }
            assert_eq!(cli_cap, cap);
        }
        _ => panic!("invocation input variant changed"),
    }
}
fn assert_outcome(view: &FinancialObservation<'_>, result: &Result<StageOutcome, StageRejection>) {
    match (view, result) {
        (FinancialObservation::Waiting { advanced, segment, continuation, attempted, residual },
         Ok(StageOutcome::Waiting { advanced: a, segment: s, continuation: c, attempted: w, residual: r })) => {
            assert_eq!(advanced, a);
            match (segment, s) {
                (Some(view), Some(raw)) => assert_segment(view, raw),
                (None, None) => {},
                _ => panic!("published segment option changed"),
            }
            assert!(std::ptr::eq(*continuation, c));
            assert_eq!(attempted, w);
            assert!(std::ptr::eq(*residual, r.as_slice()));
            assert!(std::ptr::eq(view.committed_boundary().unwrap(), &c.boundary));
            assert_eq!(view.committed_logical_work(), c.logical.committed);
        }
        (FinancialObservation::Complete { segment, committed, attempted, residual },
         Ok(StageOutcome::Complete { segment: s, committed: c, attempted: a, residual: r })) => {
            assert_segment(segment, s);
            assert_eq!(committed, c);
            assert_eq!(attempted, a);
            assert!(std::ptr::eq(*residual, r.as_slice()));
            assert!(std::ptr::eq(view.committed_boundary().unwrap(), &s.execution.post));
            assert_eq!(view.committed_logical_work(), *c);
        }
        // No rejected result is constructed or selected by this positive test.
        // Any unexpected rejection first fails complete independent JSON equality.
        _ => panic!("positive stage outcome shape differs from its projection"),
    }
}
fn compare_invocation(
    key: &str,
    program: &StagedProgram,
    input: InvocationInput<'_>,
    result: &Result<StageOutcome, StageRejection>,
    golden: &Golden,
) {
    assert_eq!(serde_json::to_value(result).unwrap(), json(golden.raw_result), "{key}: complete raw Result");
    let view = projection::project_invocation(program, input, result);
    let InvocationObservation { program: observed_program, input: observed_input,
        result: observed_result, outcome, accounting, tentative_publication } = &view;
    assert!(std::ptr::eq(*observed_program, program));
    assert!(std::ptr::eq(*observed_result, result));
    assert_input(*observed_input, input);
    assert_outcome(outcome, result);
    // Complete independent accounting JSON checks every ordered effect/group/
    // bucket/status; no selected account, field stripping or numeric coercion.
    assert_eq!(serde_json::to_value(accounting).unwrap(), json(golden.accounting), "{key}: full accounting");
    assert_eq!(serde_json::to_value(tentative_publication).unwrap(), json(golden.tentative_publication), "{key}: publication");
    match (accounting, result) {
        (AccountingAvailability::PublishedSegment(report), Ok(StageOutcome::Waiting { segment: Some(segment), .. })) |
        (AccountingAvailability::PublishedSegment(report), Ok(StageOutcome::Complete { segment, .. })) => {
            assert!(std::ptr::eq(report.effects, segment.execution.effects.as_slice()));
        }
        (AccountingAvailability::NoPublishedSegment, Ok(StageOutcome::Waiting { segment: None, .. })) => {},
        _ => panic!("accounting must preserve actual published-segment availability"),
    }
    // The original Program/Plan/hash context is available through the identical
    // StagedProgram argument; compare complete public values to independent cells.
    assert_eq!(serde_json::to_value(observed_program.program()).unwrap(), json(EXPECTED_PROGRAM));
    assert_eq!(serde_json::to_value(observed_program.plan()).unwrap(), json(EXPECTED_PLAN));
    let context = json(EXPECTED_CONTEXT);
    assert_eq!(Value::String(hash_text(observed_program.program_hash())),
        context.pointer("/program_context/program_hash").expect("frozen program hash").clone());
    assert_eq!(Value::String(hash_text(observed_program.plan_hash())),
        context.pointer("/program_context/plan_hash").expect("frozen plan hash").clone());
}
fn waiting(result: &Result<StageOutcome, StageRejection>) -> &Continuation {
    match result {
        Ok(StageOutcome::Waiting { continuation, .. }) => continuation,
        _ => panic!("fixed positive invocation must return Waiting"),
    }
}

#[test]
fn actual_core2_positive_financial_observations_match_complete_independent_r152() {
    let program = stage::load(SOURCE).expect("fixed ordinary source admits");
    let scenario: Scenario = serde_json::from_str(SCENARIO).expect("frozen input scenario");
    let inputs = json(INVOCATIONS);
    let action = inputs["action"].as_str().expect("selected action");
    let start_cap: Option<UInt> = serde_json::from_value(inputs.get("start_cap").expect("frozen start_cap input").clone()).unwrap();
    let cli_action: Option<String> = serde_json::from_value(inputs.get("resume_cli_action").expect("frozen resume_cli_action input").clone()).unwrap();
    let cli_cap: Option<UInt> = serde_json::from_value(inputs.get("resume_cli_cap").expect("frozen resume_cli_cap input").clone()).unwrap();
    let events: Vec<StageEvent> = serde_json::from_value(inputs.get("events").expect("frozen events input").clone()).unwrap();
    assert_eq!(events.len(), 2, "frozen positive input sequence");

    let start_result = stage::start(&program, action, scenario.clone(), start_cap);
    compare_invocation("start", &program,
        InvocationInput::Start { action, scenario: &scenario, cap: start_cap }, &start_result, &START);
    // Initial full snapshot is observed from the actual successful segment,
    // never synthesized from scenario or substituted for missing actual fields.
    let initial = match &start_result {
        Ok(StageOutcome::Waiting { segment: Some(segment), .. }) => &segment.execution.pre,
        _ => panic!("fixed prefix publishes a successful suspended segment"),
    };
    assert_eq!(serde_json::to_value(initial).unwrap(),
        json(START.raw_result).pointer("/Ok/Waiting/segment/execution/pre")
            .expect("independent full initial snapshot").clone());

    let false_input = waiting(&start_result).clone();
    let false_result = stage::resume(&program, false_input.clone(), events[0].clone(), cli_action.as_deref(), cli_cap);
    compare_invocation("false_await", &program,
        InvocationInput::Resume { continuation: &false_input, event: &events[0],
            cli_action: cli_action.as_deref(), cli_cap }, &false_result, &FALSE_AWAIT);
    assert_eq!(waiting(&false_result), &false_input, "actual false guard retains full input continuation");

    let true_input = waiting(&false_result).clone();
    let true_result = stage::resume(&program, true_input.clone(), events[1].clone(), cli_action.as_deref(), cli_cap);
    compare_invocation("true_resume", &program,
        InvocationInput::Resume { continuation: &true_input, event: &events[1],
            cli_action: cli_action.as_deref(), cli_cap }, &true_result, &TRUE_RESUME);
    assert!(matches!(&true_result, Ok(StageOutcome::Complete { .. })));
}
