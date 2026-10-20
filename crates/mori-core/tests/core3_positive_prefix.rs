use mori_core::stage::{self, Continuation, StageEvent, StageOutcome, StageSegment, StagedProgram};
use mori_core::{Expr, Program, Scenario, Snapshot, Statement, UInt, Value};
use serde::Serialize;
use serde_json::{Value as Json, json};

const EXPECTED: &str =
    include_str!("../../fixtures/core3-positive-prefix/EXPECTED-FINANCIAL-AND-WORK.json");
const INPUTS: &str = include_str!("../../fixtures/core3-positive-prefix/SOURCE-INPUTS.json");
const P1: &str = include_str!("../../fixtures/core3-positive-prefix/P1-RECONSTRUCTED-INPUT.mori");
const P2: &str = include_str!("../../fixtures/core3-positive-prefix/P2-RECONSTRUCTED-INPUT.mori");

fn json_of(value: &impl Serialize) -> Json {
    serde_json::to_value(value).expect("serialize observation")
}

fn number(value: &Json) -> u128 {
    value
        .as_str()
        .expect("decimal string")
        .parse()
        .expect("u128")
}

fn project(value: &Json, fields: &[&str]) -> Json {
    let mut result = serde_json::Map::new();
    for field in fields {
        result.insert((*field).into(), value.get(*field).expect(field).clone());
    }
    Json::Object(result)
}

// Resolve actual references by meaning; numeric arena allocation is not an oracle.
fn source_statement(program: &Program, statement: &Statement) -> &'static str {
    match statement {
        Statement::Discharge { name, amount } => {
            assert_eq!(name, "loan");
            assert_eq!(
                program.expressions[usize::from(*amount)],
                Expr::Literal {
                    value: Value::UInt(UInt(3)),
                }
            );
            "discharge(\"loan\", 3);"
        }
        Statement::Await {
            label,
            binders,
            guard,
        } => {
            assert_eq!(label, "ready");
            assert_eq!(json_of(binders), json!([{"name":"go","ty":"Bool"}]));
            assert_eq!(
                program.expressions[usize::from(*guard)],
                Expr::Name { name: "go".into() }
            );
            "await ready(go: Bool) when go;"
        }
        other => panic!("Unexpected source statement: {other:?}"),
    }
}

fn check_program(sp: &StagedProgram, case: &Json, input: &Json) {
    let p = sp.program();
    assert_eq!(json!(p.language), case["source_profile"]);
    assert_eq!(json!(p.semantics), case["proposed_program_semantics"]);
    assert_eq!(json!(p.name), input["agreement"]);
    assert!(p.states.is_empty());
    assert_eq!(json_of(&p.assets), json!([input["asset"].clone()]));
    assert_eq!(json_of(&p.duties), json!([input["source_duty"].clone()]));
    assert_eq!(
        json_of(&p.assets),
        case["financial_summary"]["exact_declared_assets"]
    );
    assert_eq!(
        json_of(&p.duties[0]),
        case["financial_summary"]["source_duty_declaration"]
    );
    assert_eq!(p.actions.len(), 1);
    let a = &p.actions[0];
    assert_eq!(json!(a.name), case["action"]);
    assert!(a.inputs.is_empty());
    let source_input = input["programs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| {
            entry["id"]
                == if case["source_input"] == "P1-RECONSTRUCTED-INPUT.mori" {
                    "P1"
                } else {
                    "P2"
                }
        })
        .unwrap();
    let statements: Vec<_> = a
        .statements
        .iter()
        .map(|s| source_statement(p, s))
        .collect();
    assert_eq!(json_of(&statements), source_input["statements"]);
    // Each selected root is a distinct one-node literal/name expression.
    assert_eq!(p.expressions.len(), a.statements.len());
    let plan = sp.plan();
    assert_eq!(plan.actions.len(), 1);
    let action_plan = &plan.actions[0];
    assert_eq!(action_plan.action, a.name);
    let expected_plan = &case["plan_observation_under_preserved_staged_schedule"];
    assert_eq!(
        json!({
            "statement_count": action_plan.statement_count,
            "await_sites": action_plan.sites,
            "segment_cost": action_plan.segment_cost,
            "committed_before": action_plan.committed_before,
            "plan_total": action_plan.plan_total,
        }),
        project(
            expected_plan,
            &[
                "statement_count",
                "await_sites",
                "segment_cost",
                "committed_before",
                "plan_total",
            ]
        )
    );
}

const STEP_FIELDS: &[&str] = &[
    "action_statement_ordinal",
    "source_statement",
    "typed_result",
    "pre_statement_frame",
    "complete_post_statement_snapshot",
    "new_effect_delta",
    "post_statement_effect_prefix",
];

fn expected_steps(steps: &Json) -> Json {
    Json::Array(
        steps
            .as_array()
            .unwrap()
            .iter()
            .map(|s| project(s, STEP_FIELDS))
            .collect(),
    )
}

fn segment_steps(
    sp: &StagedProgram,
    segment: &StageSegment,
    expected: &Json,
    event: &Json,
) -> Vec<Json> {
    let e = &segment.execution;
    let wanted = expected.as_array().unwrap();
    assert_eq!(e.steps.len(), wanted.len());
    let mut before = json_of(&e.pre);
    let mut effects = Vec::<Json>::new();
    let mut result = Vec::new();
    for (step, wanted) in e.steps.iter().zip(wanted) {
        let ordinal = usize::from(step.index);
        // Additional internal attachment check; the independent comparison is semantic.
        assert_eq!(step.statement, sp.program().actions[0].statements[ordinal]);
        effects.extend(step.effects.iter().map(json_of));
        let observed = json!({
            "action_statement_ordinal": step.index,
            "source_statement": source_statement(sp.program(), &step.statement),
            "typed_result": step.value,
            "pre_statement_frame": before,
            "complete_post_statement_snapshot": step.snapshot,
            "new_effect_delta": step.effects,
            "post_statement_effect_prefix": effects,
        });
        assert_eq!(observed, project(wanted, STEP_FIELDS));
        if let Some(accepted) = wanted.get("accepted_event") {
            assert_eq!(event, accepted);
            assert_eq!(step.value, Some(Value::Bool(true)));
            assert_eq!(json_of(&step.snapshot), before);
            assert!(step.effects.is_empty());
            assert_eq!(
                json!(step.snapshot.state.contains_key("go")),
                wanted["event_binding_is_state"]
            );
        }
        // Individual work components are not emitted by this runtime API.
        before = json_of(&step.snapshot);
        result.push(observed);
    }
    assert_eq!(json_of(&e.effects), Json::Array(effects));
    assert_eq!(json_of(&e.post), before);
    result
}

fn positive_rows(snapshot: &Snapshot) -> Json {
    json_of(
        &snapshot
            .liabilities
            .iter()
            .filter(|row| row.amount.0 > 0)
            .collect::<Vec<_>>(),
    )
}

// All numeric summary fields come from actual complete frames and ordered deltas.
// Narrative and external-cost qualifications are retained in the fixture/source note.
fn check_financial(pre: &Snapshot, post: &Snapshot, effects: &[Json], summary: &Json) {
    assert_eq!(json_of(pre), summary["opening_frame"]);
    assert_eq!(json_of(post), summary["closing_frame"]);
    assert_eq!(
        Json::Array(effects.to_vec()),
        summary["ordered_economic_effects"]
    );
    assert_eq!(positive_rows(pre), summary["positive_residual_before"]);
    assert_eq!(positive_rows(post), summary["positive_residual_after"]);
    assert_eq!(
        json!(pre.balances.len()),
        summary["balance_row_count_before"]
    );
    assert_eq!(
        json!(post.balances.len()),
        summary["balance_row_count_after"]
    );
    assert_eq!(
        json_of(&pre.state.keys().collect::<Vec<_>>()),
        summary["state_keys_before"]
    );
    assert_eq!(
        json_of(&post.state.keys().collect::<Vec<_>>()),
        summary["state_keys_after"]
    );
    assert_eq!(
        json!(pre.liabilities.iter().any(|r| r.name == "loan")),
        summary["loan_presence_before"]
    );
    assert_eq!(
        json!(post.liabilities.iter().any(|r| r.name == "loan")),
        summary["loan_presence_after"]
    );
    let opening = pre
        .liabilities
        .iter()
        .find(|r| r.name == "loan")
        .unwrap()
        .amount
        .0;
    let closing = post
        .liabilities
        .iter()
        .find(|r| r.name == "loan")
        .unwrap()
        .amount
        .0;
    assert_eq!(json!(closing.to_string()), summary["remaining_loan_amount"]);
    let mut created = 0_u128;
    let mut discharged = 0_u128;
    let mut gross = 0_u128;
    let mut fee = 0_u128;
    for effect in effects {
        match effect["kind"].as_str().unwrap() {
            "Liability" => {
                let before = number(&effect["before"]);
                let after = number(&effect["after"]);
                if after >= before {
                    created += after - before;
                } else {
                    discharged += before - after;
                }
            }
            "Transfer" => gross += number(&effect["amount"]),
            "Fee" => {
                let amount = number(&effect["amount"]);
                gross += amount;
                fee += amount;
            }
            other => panic!("Unexpected financial effect: {other}"),
        }
    }
    assert_eq!(
        json!(created.to_string()),
        summary["liability_creation_amount"]
    );
    assert_eq!(
        json!(discharged.to_string()),
        summary["liability_discharge_amount"]
    );
    assert_eq!(opening + created - discharged, closing);
    assert_eq!(
        json!((closing as i128 - opening as i128).to_string()),
        summary["alice_liability_change"]
    );
    assert_eq!(
        json!((closing as i128 - opening as i128).to_string()),
        summary["bob_receivable_change"]
    );
    for key in ["cash_gross_debit", "cash_gross_credit"] {
        assert_eq!(json!(gross.to_string()), summary[key]);
    }
    assert_eq!(json!(fee.to_string()), summary["explicit_fee"]);
    // These cases have no balance rows or cash/supply effect of any kind.
    assert!(pre.balances.is_empty() && post.balances.is_empty());
    assert_eq!(gross, 0);
    for key in [
        "alice_cash_net",
        "bob_cash_net",
        "cash_net_receipts",
        "signed_net_cash_flow",
        "token_supply_change",
    ] {
        assert_eq!(summary[key], "0");
    }
    assert_eq!(summary["payment_implied_by_discharge"], false);
}

fn event_from(operation: &Json) -> StageEvent {
    let event = &operation["event"];
    StageEvent {
        format: stage::STAGE_EVENT_FORMAT.into(),
        action: event["action"].as_str().unwrap().into(),
        label: event["label"].as_str().unwrap().into(),
        values: serde_json::from_value(event["values"].clone()).unwrap(),
    }
}

fn check_continuation(c: &Continuation, operation: &Json) {
    assert_eq!(
        json!({
            "action": c.action, "site": c.site, "captures": c.captures,
            "stored_cap": c.logical.cap, "committed": c.logical.committed,
            "boundary": c.boundary,
        }),
        operation["continuation_semantic_projection"]
    );
    assert_eq!(
        json!(c.logical.profile),
        operation["logical"]["profile_under_preservation_premise"]
    );
}

fn run_case(index: usize) {
    let oracle: Json = serde_json::from_str(EXPECTED).unwrap();
    let inputs: Json = serde_json::from_str(INPUTS).unwrap();
    assert_eq!(oracle["case_count"], 3);
    assert_eq!(oracle["source_program_count"], 2);
    let case = &oracle["cases"][index];
    let source = match case["source_input"].as_str().unwrap() {
        "P1-RECONSTRUCTED-INPUT.mori" => P1,
        "P2-RECONSTRUCTED-INPUT.mori" => P2,
        other => panic!("Unknown reconstructed source: {other}"),
    };
    let sp = stage::load_v3(source).expect("load selected positive Core/3 source");
    check_program(&sp, case, &inputs);
    let original_program = sp.program().clone();
    let original_plan = sp.plan().clone();
    let original_hashes = (sp.program_hash(), sp.plan_hash());
    let scenario: Scenario = serde_json::from_value(case["initial_scenario"].clone()).unwrap();
    let original_scenario = scenario.clone();
    let initial: Snapshot =
        serde_json::from_value(case["financial_summary"]["opening_frame"].clone()).unwrap();
    let mut boundary = initial.clone();
    let mut continuation: Option<Continuation> = None;
    let mut committed = UInt(0);
    let mut attempted_total = 0_u128;
    let mut history_effects = Vec::<Json>::new();
    let mut history_steps = Vec::<Json>::new();
    // One initial frame, then each invocation's retained or advanced boundary.
    let mut history_frames = vec![json_of(&initial)];
    for (ordinal, operation) in case["operation_history"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        assert_eq!(json!(ordinal), operation["operation_ordinal"]);
        assert_eq!(
            json_of(&committed),
            operation["logical"]["committed_before"]
        );
        assert_eq!(
            Json::Array(history_effects.clone()),
            operation["history_effects_before"]
        );
        let before_continuation = continuation.clone();
        let before_boundary = boundary.clone();
        let outcome = if operation["operation"] == "start" {
            assert_eq!(ordinal, 0);
            assert!(operation["event"].is_null());
            let cap: Option<UInt> =
                serde_json::from_value(case["start_logical_cap"].clone()).unwrap();
            stage::start(&sp, case["action"].as_str().unwrap(), scenario.clone(), cap)
        } else {
            assert_eq!(operation["operation"], "resume");
            let cap: Option<UInt> =
                serde_json::from_value(case["resume_logical_caps"][ordinal - 1].clone()).unwrap();
            stage::resume(
                &sp,
                continuation.clone().unwrap(),
                event_from(operation),
                Some("pay"),
                cap,
            )
        }
        .expect("selected ordinary operation succeeds");
        let report = stage::outcome_report(Ok(outcome.clone()));
        assert_eq!(report["status"], operation["semantic_outcome"]);
        for (key, value) in operation["qualification_projection_under_preserved_stage_contract"]
            .as_object()
            .unwrap()
        {
            assert_eq!(&report[key], value);
        }
        let (segment, attempted, residual) = match &outcome {
            StageOutcome::Waiting {
                advanced,
                segment,
                continuation: c,
                attempted,
                residual,
            } => {
                assert_eq!(json!(advanced), operation["advanced"]);
                check_continuation(c, operation);
                boundary = c.boundary.clone();
                committed = c.logical.committed;
                continuation = Some(c.clone());
                if operation
                    .get("continuation_unchanged_from_previous_operation")
                    .is_some()
                {
                    assert_eq!(c, before_continuation.as_ref().unwrap());
                    assert_eq!(
                        json_of(&before_boundary),
                        operation["retained_boundary_before"]
                    );
                    assert_eq!(json_of(&boundary), operation["retained_boundary_after"]);
                    assert!(segment.is_none());
                    assert_eq!(operation["guard_public_step_emitted"], false);
                    assert_eq!(operation["public_effects_emitted"], false);
                    assert_eq!(
                        operation["guard_semantic_result"],
                        operation["event"]["values"][0]["value"]["value"]
                    );
                    assert!(report["segment"].is_null());
                }
                (segment.as_ref(), *attempted, residual)
            }
            StageOutcome::Complete {
                segment,
                committed: work,
                attempted,
                residual,
            } => {
                assert_eq!(operation["advanced_member"], "absent");
                assert_eq!(operation["continuation_member"], "absent");
                assert!(report.get("advanced").is_none());
                assert!(report.get("continuation").is_none());
                boundary = segment.execution.post.clone();
                committed = *work;
                continuation = None;
                (Some(segment), *attempted, residual)
            }
        };
        assert_eq!(json!(segment.is_some()), operation["segment_exists"]);
        assert_eq!(json_of(&attempted), operation["logical"]["attempted"]);
        attempted_total += attempted.0;
        assert_eq!(
            json!(attempted_total.to_string()),
            operation["logical"]["cumulative_attempted"]
        );
        assert_eq!(json_of(&committed), operation["logical"]["committed_after"]);
        assert_eq!(report["committed"], json_of(&committed));
        assert_eq!(report["attempted"], json_of(&attempted));
        assert_eq!(json_of(residual), operation["positive_residual"]);
        assert_eq!(positive_rows(&boundary), operation["positive_residual"]);
        if let Some(segment) = segment {
            assert_eq!(json!(segment.action), case["action"]);
            assert_eq!(json!(segment.execution.action), case["action"]);
            assert_eq!(
                json!(segment.execution.semantics),
                case["proposed_program_semantics"]
            );
            assert_eq!(json_of(&segment.frame_pre), operation["frame_pre"]);
            assert_eq!(json_of(&segment.frame_post), operation["frame_post"]);
            assert_eq!(json_of(&segment.execution.pre), operation["segment_pre"]);
            assert_eq!(json_of(&segment.execution.post), operation["segment_post"]);
            assert_eq!(
                json_of(&segment.execution.effects),
                operation["segment_effects"]
            );
            assert_eq!(segment.execution.pre, before_boundary);
            history_steps.extend(segment_steps(
                &sp,
                segment,
                &operation["step_observations"],
                &operation["event"],
            ));
            history_effects.extend(segment.execution.effects.iter().map(json_of));
            if let Some(entry) = operation.get("await_entry") {
                assert_eq!(entry["public_step_emitted"], false);
                assert_eq!(entry["guard_evaluated"], false);
                assert!(segment.execution.steps.is_empty());
                assert_eq!(json_of(&attempted), entry["entry_charge"]);
            }
        } else {
            assert!(operation["public_segment"].is_null());
            assert_eq!(operation["step_observations"], json!([]));
            assert_eq!(
                json!({"site":before_continuation.as_ref().unwrap().site,"captures":before_continuation.as_ref().unwrap().captures}),
                operation["frame_pre"]
            );
            assert_eq!(
                json!({"site":continuation.as_ref().unwrap().site,"captures":continuation.as_ref().unwrap().captures}),
                operation["frame_post"]
            );
        }
        history_frames.push(json_of(&boundary));
        assert_eq!(
            Json::Array(history_frames.clone()),
            operation["history_financial_prefixes"]
        );
        assert_eq!(
            Json::Array(history_steps.clone()),
            expected_steps(&operation["history_steps_after"])
        );
        assert_eq!(
            Json::Array(history_effects.clone()),
            operation["history_effects_after"]
        );
        if let Some(summary) = operation.get("financial_summary_for_operation") {
            let effects: Vec<Json> = segment
                .map(|s| s.execution.effects.iter().map(json_of).collect())
                .unwrap_or_default();
            check_financial(&before_boundary, &boundary, &effects, summary);
        }
        assert_eq!(scenario, original_scenario);
        assert_eq!(sp.program(), &original_program);
        assert_eq!(sp.plan(), &original_plan);
        assert_eq!((sp.program_hash(), sp.plan_hash()), original_hashes);
    }
    check_financial(
        &initial,
        &boundary,
        &history_effects,
        &case["financial_summary"],
    );
    assert_eq!(committed, sp.plan().actions[0].plan_total);
    assert!(continuation.is_none());
    if let Some(ordinals) = case.get("cross_operation_step_ordinals") {
        assert_eq!(
            json_of(
                &history_steps
                    .iter()
                    .map(|s| &s["action_statement_ordinal"])
                    .collect::<Vec<_>>()
            ),
            *ordinals
        );
        assert_eq!(case["retained_false_event_binding"], false);
        assert!(!boundary.state.contains_key("go"));
    }
}

#[test]
fn declared_duty_discharge_five_to_two_complete_prefix() {
    run_case(0);
}

#[test]
fn declared_duty_discharge_three_to_present_zero_complete_prefix() {
    run_case(1);
}

#[test]
fn declared_duty_await_false_then_true_complete_prefixes_and_work() {
    run_case(2);
}
