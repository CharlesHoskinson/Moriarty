//! Proposed ordinary Core-only consumer; execution remains UNRUN.
//! Independent O bytes were frozen before translation and before subject runs.
//! These two normal cases cover null CLI flags and no rejection expectation.
use crate::*;
use crate::stage::{self, *};
use crate::sherlock_financial_projection::{
    ExecutionObservation, FinancialObservation, ResumeFinancialObservation,
    SegmentObservation, financial_observation_matches, project_resume_result,
    project_stage_result, resume_financial_observation_matches,
};
use std::collections::BTreeMap;

struct ExpectedCase {
    id: &'static str,
    source: &'static str,
    action: &'static str,
    start_cap: UInt,
    scenario: Scenario,
    program: Program,
    plan: StagePlan,
    program_hash: &'static str,
    plan_hash: &'static str,
    invocations: Vec<ExpectedInvocation>,
}

enum ExpectedInvocation {
    Start { id: &'static str, outcome: StageOutcome },
    Resume {
        id: &'static str,
        continuation_from: &'static str,
        input: Continuation,
        event: StageEvent,
        event_input: StageEvent,
        cli_action: Option<&'static str>,
        cli_cap: Option<UInt>,
        cli_action_input: Option<&'static str>,
        cli_cap_input: Option<UInt>,
        outcome: StageOutcome,
    },
}

include!("financial_core2_literals.rs");

// Expected borrowed views are formed directly from independent typed storage.
// They never call the production projection functions.
fn expected_segment(segment: &StageSegment) -> SegmentObservation<'_> {
    SegmentObservation {
        action: &segment.action,
        frame_pre: &segment.frame_pre,
        frame_post: &segment.frame_post,
        execution: ExecutionObservation {
            semantics: &segment.execution.semantics,
            action: &segment.execution.action,
            pre: &segment.execution.pre,
            post: &segment.execution.post,
            steps: &segment.execution.steps,
            effects: &segment.execution.effects,
        },
    }
}

fn expected_view(outcome: &StageOutcome) -> FinancialObservation<'_> {
    match outcome {
        StageOutcome::Waiting { advanced, segment, continuation, attempted, residual } =>
            FinancialObservation::Waiting {
                advanced: *advanced,
                segment: segment.as_ref().map(expected_segment),
                continuation,
                attempted: *attempted,
                residual,
            },
        StageOutcome::Complete { segment, committed, attempted, residual } =>
            FinancialObservation::Complete {
                segment: expected_segment(segment),
                committed: *committed,
                attempted: *attempted,
                residual,
            },
    }
}

fn hash_text(bytes: [u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn compare_case(case: ExpectedCase) {
    // Actual preparation uses the same existing producer as Core/2 CLI stages:
    // stage::load -> parser::parse_v2 -> profile/validation/plan/cost/digests.
    let program = stage::load(case.source).expect("fixed positive source admits");
    println!("{}", serde_json::json!({
        "case": case.id, "operation": "load", "actual_program": program.program(),
        "actual_plan": program.plan(), "actual_program_hash": hash_text(program.program_hash()),
        "actual_plan_hash": hash_text(program.plan_hash()), "labels": stage::STAGE_LABELS,
    }));
    assert_eq!(program.program(), &case.program, "{} complete Program", case.id);
    assert_eq!(program.plan(), &case.plan, "{} complete StagePlan", case.id);
    assert_eq!(hash_text(program.program_hash()), case.program_hash);
    assert_eq!(hash_text(program.plan_hash()), case.plan_hash);
    let mut published: BTreeMap<&str, Continuation> = BTreeMap::new();
    for invocation in &case.invocations {
        let (id, actual, outcome) = match invocation {
            ExpectedInvocation::Start { id, outcome } => {
                let actual = stage::start(&program, case.action, case.scenario.clone(), Some(case.start_cap));
                println!("{}", serde_json::json!({
                    "case": case.id, "id": id, "operation": "start", "action": case.action,
                    "scenario": case.scenario, "cap": case.start_cap, "actual_result": actual,
                }));
                (*id, actual, outcome)
            }
            ExpectedInvocation::Resume { id, continuation_from, input, event, event_input, cli_action, cli_cap, cli_action_input, cli_cap_input, outcome } => {
                // Resume the actual preceding published continuation, not O's input.
                let actual_input = published.get(continuation_from).expect("named actual waiting prefix").clone();
                // Capture all call arguments before passing owned clones to Core/2.
                let actual_event = event_input.clone();
                let actual_cli_action = *cli_action_input;
                let actual_cli_cap = *cli_cap_input;
                let actual = stage::resume(&program, actual_input.clone(), actual_event.clone(), actual_cli_action, actual_cli_cap);
                let actual_view = project_resume_result(&actual_input, &actual_event, actual_cli_action, actual_cli_cap, &actual);
                let expected_resume = ResumeFinancialObservation {
                    input, event, cli_action: *cli_action, cli_cap: *cli_cap,
                    outcome: expected_view(outcome),
                };
                println!("{}", serde_json::json!({
                    "case": case.id, "id": id, "operation": "resume", "continuation_from": continuation_from,
                    "actual_input": actual_input, "actual_event": actual_event,
                    "actual_cli_action": actual_cli_action, "actual_cli_cap": actual_cli_cap,
                    "actual_result": actual,
                    "resume_projection_equal": actual_view == expected_resume,
                    "actual_resume_projection_debug": format!("{actual_view:?}"),
                }));
                assert_eq!(actual_view, expected_resume, "{} {} full resume view", case.id, id);
                assert!(resume_financial_observation_matches(&actual_view, &expected_resume));
                (*id, actual, outcome)
            }
        };
        let expected = expected_view(outcome);
        println!("{}", serde_json::json!({
            "case": case.id, "id": id,
            "actual_stage_projection_debug": format!("{:?}", project_stage_result(&actual)),
        }));
        assert_eq!(project_stage_result(&actual), expected, "{} {} full financial view", case.id, id);
        assert!(financial_observation_matches(&actual, &expected));
        assert_eq!(&actual, &Ok(outcome.clone()), "{} {} full stage result", case.id, id);
        if let Ok(StageOutcome::Waiting { continuation, .. }) = actual {
            published.insert(id, continuation);
        }
    }
}

#[test]
fn independent_core2_financial_two_programs_match_complete_frozen_views() {
    compare_case(frozen_f01());
    compare_case(frozen_f02());
}
