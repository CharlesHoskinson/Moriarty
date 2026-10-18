//! Normal-language expectations frozen independently before implementation.
//! Oracle SHA256: 76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640.
//! These are local reference tests, not authenticated settlement evidence.
use mori_core::stage::{self, Continuation, StageEvent, StageOutcome};
use mori_core::{Scenario, UInt, Value as CoreValue};
use serde_json::{Value, json};

const ESCROW: &str = include_str!("../../../examples/core/continuations/escrow.mori");
const RESERVE: &str = include_str!("../../../examples/core/continuations/reserve.mori");

fn oracle() -> Value {
    serde_json::from_str(include_str!("fixtures/continuations/normal-oracle.json")).unwrap()
}
fn scenario(value: &Value) -> Scenario {
    serde_json::from_value(value.clone()).unwrap()
}
fn event(value: &Value) -> StageEvent {
    serde_json::from_value(value.clone()).unwrap()
}
fn waiting(outcome: StageOutcome, expected: &Value) -> Continuation {
    let StageOutcome::Waiting {
        advanced,
        segment,
        continuation,
        attempted,
        residual,
    } = outcome
    else {
        panic!("expected a waiting computation")
    };
    assert_eq!(advanced, expected["advanced"].as_bool().unwrap());
    assert_eq!(
        attempted,
        UInt(expected["attempted"].as_u64().unwrap().into())
    );
    assert_eq!(
        continuation.logical.committed,
        UInt(expected["committed"].as_u64().unwrap().into())
    );
    assert_eq!(
        serde_json::to_value(&continuation.captures).unwrap(),
        expected["captures"]
    );
    assert_eq!(
        serde_json::to_value(&residual).unwrap(),
        expected["residual"]
    );
    if advanced {
        compare_segment(&segment.expect("advanced prefix"), &expected["segment"]);
    } else {
        assert!(segment.is_none());
        assert_eq!(
            serde_json::to_value(&continuation.boundary).unwrap(),
            expected["post"]
        );
    }
    continuation
}
fn compare_segment(actual: &stage::StageSegment, expected: &Value) {
    let execution = &actual.execution;
    assert_eq!(execution.semantics, "moriarty-core/2");
    assert_eq!(
        serde_json::to_value(&execution.pre).unwrap(),
        expected["pre"]
    );
    assert_eq!(
        serde_json::to_value(&execution.post).unwrap(),
        expected["post"]
    );
    assert_eq!(
        serde_json::to_value(&execution.effects).unwrap(),
        expected["effects"]
    );
    assert_eq!(
        execution.steps.len(),
        expected["steps"].as_array().unwrap().len()
    );
    for (step, expected) in execution
        .steps
        .iter()
        .zip(expected["steps"].as_array().unwrap())
    {
        assert_eq!(u64::from(step.index), expected["index"].as_u64().unwrap());
        assert_eq!(
            serde_json::to_value(&step.value).unwrap(),
            expected["value"]
        );
        assert_eq!(
            serde_json::to_value(&step.snapshot).unwrap(),
            expected["snapshot"]
        );
        assert_eq!(
            serde_json::to_value(&step.effects).unwrap(),
            expected["effects"]
        );
    }
}
fn complete(outcome: StageOutcome, expected: &Value) {
    let StageOutcome::Complete {
        segment,
        committed,
        attempted,
        residual,
    } = outcome
    else {
        panic!("expected complete computation")
    };
    assert_eq!(
        committed,
        UInt(expected["committed"].as_u64().unwrap().into())
    );
    assert_eq!(
        attempted,
        UInt(expected["attempted"].as_u64().unwrap().into())
    );
    assert_eq!(segment.frame_post.site, 0);
    assert!(segment.frame_post.captures.is_empty());
    assert_eq!(
        serde_json::to_value(residual).unwrap(),
        expected["residual"]
    );
    compare_segment(&segment, &expected["segment"]);
}

#[test]
fn escrow_prefix_false_observation_and_true_remainder_match_literal_oracle() {
    let expected = &oracle()["escrow"];
    let program = stage::load(ESCROW).unwrap();
    let plan = &program.plan().actions[0];
    assert_eq!(plan.segment_cost, vec![UInt(5), UInt(6)]);
    assert_eq!(plan.committed_before, vec![UInt(0), UInt(5)]);
    assert_eq!(plan.plan_total, UInt(11));
    let prefix = waiting(
        stage::start(
            &program,
            "deliver",
            scenario(&expected["opening"]),
            Some(UInt(11)),
        )
        .unwrap(),
        &expected["prefix_waiting"],
    );
    let before = serde_json::to_value(&prefix).unwrap();
    let same = waiting(
        stage::resume(
            &program,
            prefix,
            event(&expected["false_resume"]["event"]),
            None,
            None,
        )
        .unwrap(),
        &expected["false_resume"],
    );
    assert_eq!(serde_json::to_value(&same).unwrap(), before);
    complete(
        stage::resume(
            &program,
            same,
            event(&expected["true_resume_complete"]["event"]),
            None,
            None,
        )
        .unwrap(),
        &expected["true_resume_complete"],
    );
}

#[test]
fn reserve_two_awaits_preserve_captured_held_and_residual_five() {
    let expected = &oracle()["reserve"];
    let program = stage::load(RESERVE).unwrap();
    let plan = &program.plan().actions[0];
    assert_eq!(plan.segment_cost, vec![UInt(11), UInt(15), UInt(10)]);
    assert_eq!(plan.committed_before, vec![UInt(0), UInt(11), UInt(26)]);
    assert_eq!(plan.plan_total, UInt(36));
    let first = waiting(
        stage::start(
            &program,
            "allocate",
            scenario(&expected["opening"]),
            Some(UInt(36)),
        )
        .unwrap(),
        &expected["prefix_waiting"],
    );
    let first_value = serde_json::to_value(&first).unwrap();
    let first = waiting(
        stage::resume(
            &program,
            first,
            event(&expected["first_false_resume"]["event"]),
            None,
            None,
        )
        .unwrap(),
        &expected["first_false_resume"],
    );
    assert_eq!(serde_json::to_value(&first).unwrap(), first_value);
    let second = waiting(
        stage::resume(
            &program,
            first,
            event(&expected["first_true_second_boundary"]["event"]),
            None,
            None,
        )
        .unwrap(),
        &expected["first_true_second_boundary"],
    );
    assert_eq!(
        second
            .captures
            .iter()
            .find(|v| v.name == "held")
            .unwrap()
            .value,
        CoreValue::UInt(UInt(42))
    );
    assert_eq!(second.boundary.state["basis"], CoreValue::UInt(UInt(78)));
    let second_value = serde_json::to_value(&second).unwrap();
    let second = waiting(
        stage::resume(
            &program,
            second,
            event(&expected["second_false_resume"]["event"]),
            None,
            None,
        )
        .unwrap(),
        &expected["second_false_resume"],
    );
    assert_eq!(serde_json::to_value(&second).unwrap(), second_value);
    complete(
        stage::resume(
            &program,
            second,
            event(&expected["second_true_complete"]["event"]),
            None,
            None,
        )
        .unwrap(),
        &expected["second_true_complete"],
    );
}

#[test]
fn normal_business_guard_rejects_only_the_final_attempted_segment() {
    let expected = &oracle()["reserve"];
    let refusal = &expected["normal_business_suffix_rejection"];
    let program = stage::load(RESERVE).unwrap();
    let StageOutcome::Waiting {
        continuation: first,
        ..
    } = stage::start(
        &program,
        "allocate",
        scenario(&expected["opening"]),
        Some(UInt(36)),
    )
    .unwrap()
    else {
        panic!("first boundary")
    };
    let StageOutcome::Waiting {
        continuation: second,
        ..
    } = stage::resume(
        &program,
        first,
        event(&refusal["first_accepted_event"]),
        None,
        None,
    )
    .unwrap()
    else {
        panic!("second boundary")
    };
    let before = serde_json::to_value(&second).unwrap();
    let error = stage::resume(
        &program,
        second.clone(),
        event(&refusal["resume_event"]),
        None,
        None,
    )
    .unwrap_err();
    assert_eq!(error.error.code, refusal["error_code"].as_str().unwrap());
    assert_eq!(
        u64::from(error.error.step.unwrap()),
        refusal["error_step"].as_u64().unwrap()
    );
    assert_eq!(error.attempted, UInt(6));
    assert_eq!(error.committed, UInt(26));
    let rejection = serde_json::to_value(&error).unwrap();
    assert!(rejection.get("segment").is_none());
    assert!(rejection.get("effects").is_none());
    assert!(rejection.get("continuation").is_none());
    assert_eq!(serde_json::to_value(&second).unwrap(), before);
}

#[test]
fn consecutive_awaits_capture_earlier_events_without_reexecuting_prefix() {
    let program = stage::load(
        r#"language "moriarty/2"; agreement Consecutive {
      state count: UInt = 0;
      action run(x: UInt) {
        let captured = x + 1;
        next count = captured;
        await first(y: UInt) when y == captured;
        await second(ready: Bool) when ready;
        next count = count + y;
      }
    }"#,
    )
    .unwrap();
    let opening = Scenario {
        inputs: [("x".into(), CoreValue::UInt(UInt(4)))].into(),
        ..Scenario::default()
    };
    let StageOutcome::Waiting {
        continuation: first,
        attempted,
        ..
    } = stage::start(&program, "run", opening, None).unwrap()
    else {
        panic!("first")
    };
    assert_eq!(attempted, UInt(7));
    assert_eq!(first.boundary.state["count"], CoreValue::UInt(UInt(5)));
    let e = |label, values| {
        event(
            &json!({"format":"moriarty-stage-event/1","action":"run","label":label,"values":values}),
        )
    };
    let StageOutcome::Waiting {
        continuation: second,
        segment: Some(segment),
        attempted,
        ..
    } = stage::resume(
        &program,
        first,
        e(
            "first",
            json!([{"name":"y","value":{"type":"UInt","value":"5"}}]),
        ),
        None,
        None,
    )
    .unwrap()
    else {
        panic!("second")
    };
    assert_eq!(attempted, UInt(5));
    assert_eq!(
        segment
            .execution
            .steps
            .iter()
            .map(|s| s.index)
            .collect::<Vec<_>>(),
        vec![2]
    );
    assert_eq!(
        second
            .captures
            .iter()
            .map(|v| v.name.as_str())
            .collect::<Vec<_>>(),
        vec!["x", "captured", "y"]
    );
    let StageOutcome::Complete {
        segment,
        attempted,
        committed,
        ..
    } = stage::resume(
        &program,
        second,
        e(
            "second",
            json!([{"name":"ready","value":{"type":"Bool","value":true}}]),
        ),
        None,
        None,
    )
    .unwrap()
    else {
        panic!("complete")
    };
    assert_eq!(attempted, UInt(6));
    assert_eq!(committed, UInt(18));
    assert_eq!(
        segment.execution.post.state["count"],
        CoreValue::UInt(UInt(10))
    );
    assert_eq!(
        segment
            .execution
            .steps
            .iter()
            .map(|s| s.index)
            .collect::<Vec<_>>(),
        vec![3, 4]
    );
}

#[test]
fn ordinary_zero_await_action_completes_with_empty_frames() {
    let program = stage::load(
        r#"language "moriarty/2"; agreement Ordinary {
      state count: UInt = 0;
      action run(x: UInt) { next count = count + x; }
    }"#,
    )
    .unwrap();
    let opening = Scenario {
        inputs: [("x".into(), CoreValue::UInt(UInt(3)))].into(),
        ..Scenario::default()
    };
    let StageOutcome::Complete {
        segment,
        committed,
        attempted,
        residual,
    } = stage::start(&program, "run", opening, None).unwrap()
    else {
        panic!("ordinary completion")
    };
    assert_eq!(program.plan().actions[0].segment_cost, vec![UInt(4)]);
    assert_eq!(program.plan().actions[0].committed_before, vec![UInt(0)]);
    assert_eq!(committed, UInt(4));
    assert_eq!(attempted, UInt(4));
    assert!(residual.is_empty());
    assert_eq!(segment.frame_pre.site, 0);
    assert!(segment.frame_pre.captures.is_empty());
    assert_eq!(segment.frame_post.site, 0);
    assert!(segment.frame_post.captures.is_empty());
    assert_eq!(
        segment.execution.post.state["count"],
        CoreValue::UInt(UInt(3))
    );
}
