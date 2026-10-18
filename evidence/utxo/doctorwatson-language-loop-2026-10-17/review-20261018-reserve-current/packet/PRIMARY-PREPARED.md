# ReserveQuint/Rust four-path source and actual-result review — PREPARE ONLY

No provider is dispatched. Root requested this fresh ordinary lane and must confirm final decoder author-freeze/readiness before any request. Desired eventual reviewer AGY Gemini4ArgonHigh HIGH; actual canonical identity remains unknown unless provider-envelope evidence attests it. Root alone integrates.

Exact source f15214ab7a885a4dc3194002cbacbb56841fdcbf/tree0d0a663592893e239ddea82bab5c88eb494e95e3, parent actualmaine4d7a2b5709caa18a94f5256d03a0302364b8235. Entire4changedfiles,2fullimportedmodels,193verified actual source pins/current public Rust library callchain/helpers/static-context/input/scope sources are retained physically. All4changedfiles and whole runtime decoder/first+corrected comparison scripts are reproduced below. Full importedmodels and normative sources are material physical supplements; reviewer must actually read them and report coverage honestly.

Root actualchecks:1newRust +5helperPASS; Quinttypecheck +1deterministic TypeScript-backend testPASS;5rawITFstates. Root subsequent CAPTURED-COMPARISON records MATCHED: entire actualRust initial/prefix+5observations equals independent projection byteSHA519d2972fc58d74c824a8f514b8a0176c3849b9848bbc0b99729b4bd9e92c8e8; actual5decodedQuintObservations structurally equal actualRust andoracle. Decoder source freeze5b4d046c735b2d1b6b387127f38e9e68a7b72b330e034f31ccc47411af1525e0 and full source/encoding basis are physical. No decoder/product/comparison execution occurred in this transport lane.

Preserve chronology: initial root compare_actual.py ran decoder successfully, then stdout extraction failed on the Rust harness prefix. ROOT-COMPARISON-A1-FAILURE.json is retained; corrected compare_actual_a2.py reuses existing actual outputs and requires the exact harness prefix/single marker. No Core/Quint/decoder rerun. Original earlier RESULT/CAPTURE-SUMMARY still say PENDING and are unchanged; later captured comparison supplies the observed match. No failure is hidden.

Independent original35memberfreeze7dc1b5c503a7cd2ab8e298fa638d42d7afd365df5e6253ce280e99af2178e7c3 is unchanged and verified; actual CHECK-PLAN bound it before execution. Its codec is author-supplied conditional normalization, with prior exposure and all omissions preserved. Decoder author froze runtime source before actualtrace exposure and reports no runtime changes afterward; ACTUAL-EXPOSURE is disclosed. Do not turn this finite match into exhaustive model proof, native/SDK/proof/ledger/target or parent/composition closure. Quint initial/prefix frames are pure assertions, not raw trace variables.

Distinct from old LSP/Q1/remittance lanes: no unrelated fixture, sourcehash or reviewer vote imported. This is a prepared whole-source/result packet, not a review verdict.


## WHOLE FILE candidate/crates/mori-core/tests/continuations.rs

```
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
        u64::from(continuation.site),
        expected["site"].as_u64().unwrap()
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
        let segment = segment.expect("advanced prefix");
        assert_eq!(segment.frame_post.site, continuation.site);
        assert_eq!(segment.frame_post.captures, continuation.captures);
        assert_eq!(segment.execution.post, continuation.boundary);
        compare_segment(&segment, &expected["segment"]);
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

```

## WHOLE FILE candidate/crates/mori-core/tests/fixtures/continuations/reserve-quint-projection.json

```
{
  "initial": {
    "specimen": "Reserve",
    "inputValue": 2,
    "balances": {
      "observer": 42,
      "reserve": 100,
      "target": 0,
      "treasury": 0
    },
    "programState": {
      "allocated": 0,
      "basis": 40,
      "marker": 77
    },
    "dutyPresent": true,
    "dutyAmount": 5,
    "pendingSite": 0,
    "captureValues": [],
    "logicalAttempted": 0,
    "logicalCommitted": 0,
    "effects": []
  },
  "prefix": {
    "specimen": "Reserve",
    "inputValue": 2,
    "balances": {
      "observer": 42,
      "reserve": 100,
      "target": 0,
      "treasury": 0
    },
    "programState": {
      "allocated": 0,
      "basis": 35,
      "marker": 77
    },
    "dutyPresent": true,
    "dutyAmount": 47,
    "pendingSite": 1,
    "captureValues": [
      2,
      42
    ],
    "logicalAttempted": 11,
    "logicalCommitted": 11,
    "effects": [
      {
        "tag": "StateChange",
        "before": 40,
        "after": 35
      },
      {
        "tag": "LiabilityChange",
        "before": 5,
        "after": 47
      }
    ]
  },
  "observations": [
    {
      "c": {
        "specimen": "Reserve",
        "balances": {
          "observer": 42,
          "reserve": 100,
          "target": 0,
          "treasury": 0
        },
        "programState": {
          "allocated": 0,
          "basis": 35,
          "marker": 77
        },
        "dutyPresent": true,
        "dutyAmount": 47,
        "site": 1,
        "captures": [
          2,
          42
        ],
        "committed": 11
      },
      "last": {
        "status": "NotInvoked",
        "attempted": 0,
        "advanced": false,
        "hasSegment": false,
        "newContinuation": false,
        "effects": [],
        "sourceSteps": [],
        "errorSteps": []
      },
      "attemptedTotal": 11
    },
    {
      "c": {
        "specimen": "Reserve",
        "balances": {
          "observer": 42,
          "reserve": 100,
          "target": 0,
          "treasury": 0
        },
        "programState": {
          "allocated": 0,
          "basis": 35,
          "marker": 77
        },
        "dutyPresent": true,
        "dutyAmount": 47,
        "site": 1,
        "captures": [
          2,
          42
        ],
        "committed": 11
      },
      "last": {
        "status": "Waiting",
        "attempted": 4,
        "advanced": false,
        "hasSegment": false,
        "newContinuation": false,
        "effects": [],
        "sourceSteps": [],
        "errorSteps": []
      },
      "attemptedTotal": 15
    },
    {
      "c": {
        "specimen": "Reserve",
        "balances": {
          "observer": 42,
          "reserve": 58,
          "target": 42,
          "treasury": 0
        },
        "programState": {
          "allocated": 42,
          "basis": 78,
          "marker": 77
        },
        "dutyPresent": true,
        "dutyAmount": 5,
        "site": 2,
        "captures": [
          2,
          42,
          43
        ],
        "committed": 26
      },
      "last": {
        "status": "Waiting",
        "attempted": 15,
        "advanced": true,
        "hasSegment": true,
        "newContinuation": true,
        "effects": [
          {
            "tag": "StateValue",
            "name": "allocated",
            "before": 0,
            "after": 42
          },
          {
            "tag": "TransferAmount",
            "amount": 42
          },
          {
            "tag": "DutyValue",
            "before": 47,
            "after": 5
          },
          {
            "tag": "StateValue",
            "name": "basis",
            "before": 35,
            "after": 78
          }
        ],
        "sourceSteps": [
          3,
          4,
          5,
          6,
          7
        ],
        "errorSteps": []
      },
      "attemptedTotal": 30
    },
    {
      "c": {
        "specimen": "Reserve",
        "balances": {
          "observer": 42,
          "reserve": 58,
          "target": 42,
          "treasury": 0
        },
        "programState": {
          "allocated": 42,
          "basis": 78,
          "marker": 77
        },
        "dutyPresent": true,
        "dutyAmount": 5,
        "site": 2,
        "captures": [
          2,
          42,
          43
        ],
        "committed": 26
      },
      "last": {
        "status": "Waiting",
        "attempted": 2,
        "advanced": false,
        "hasSegment": false,
        "newContinuation": false,
        "effects": [],
        "sourceSteps": [],
        "errorSteps": []
      },
      "attemptedTotal": 32
    },
    {
      "c": {
        "specimen": "Reserve",
        "balances": {
          "observer": 42,
          "reserve": 56,
          "target": 42,
          "treasury": 2
        },
        "programState": {
          "allocated": 42,
          "basis": 78,
          "marker": 77
        },
        "dutyPresent": true,
        "dutyAmount": 5,
        "site": 0,
        "captures": [],
        "committed": 36
      },
      "last": {
        "status": "Complete",
        "attempted": 10,
        "advanced": true,
        "hasSegment": true,
        "newContinuation": false,
        "effects": [
          {
            "tag": "FeeAmount",
            "amount": 2
          },
          {
            "tag": "EmitQuote",
            "quote": 43
          }
        ],
        "sourceSteps": [
          8,
          9,
          10,
          11
        ],
        "errorSteps": []
      },
      "attemptedTotal": 42
    }
  ]
}

```

## WHOLE FILE candidate/crates/mori-core/tests/reserve_quint_projection.rs

```
use mori_core::stage::{self, Continuation, NamedValue, StageEvent, StageOutcome, StageSegment};
use mori_core::{AssetId, Effect, Scenario, Snapshot, UInt, Value};
use serde_json::{Value as Json, json};
use std::collections::BTreeMap;

const SOURCE: &str = include_str!("../../../examples/core/continuations/reserve.mori");
const OPENING: &str = include_str!("../../../examples/core/continuations/reserve-scenario.json");
const EXPECTED: &str = include_str!("fixtures/continuations/reserve-quint-projection.json");

// This adapter is intentionally restricted to the existing normal reserve trace.
// Small JSON integers name the model's mathematical ints; they are not a UInt wire.
fn number(value: UInt) -> u64 {
    let value = u64::try_from(value.0).expect("normal reserve value fits u64");
    assert!(value <= 142, "outside the disclosed normal adapter domain");
    value
}

fn uint(value: &Value) -> u64 {
    match value {
        Value::UInt(value) => number(*value),
        Value::Bool(_) => panic!("the selected boundary/captures are UInt-only"),
    }
}

fn reserve_asset(asset: &AssetId) {
    assert_eq!(
        serde_json::to_value(asset).unwrap(),
        json!({
            "network":"local", "issuer":"continuation-reserve-oracle",
            "reference":"UNIT", "scale":0
        })
    );
}

fn allocation(name: &str, asset: &AssetId, debtor: &str, creditor: &str) {
    assert_eq!(name, "allocation");
    reserve_asset(asset);
    assert_eq!((debtor, creditor), ("reserve", "target"));
}

fn boundary(snapshot: &Snapshot) -> Json {
    let mut balances = BTreeMap::new();
    for balance in &snapshot.balances {
        reserve_asset(&balance.asset);
        assert!(
            balances
                .insert(&balance.account, number(balance.amount))
                .is_none()
        );
    }
    let program_state: BTreeMap<_, _> = snapshot
        .state
        .iter()
        .map(|(name, value)| (name, uint(value)))
        .collect();
    assert!(snapshot.liabilities.len() <= 1, "no unprojected duties");
    let duty_amount = match snapshot.liabilities.first() {
        Some(duty) => {
            allocation(&duty.name, &duty.asset, &duty.debtor, &duty.creditor);
            number(duty.amount)
        }
        None => 0,
    };
    json!({"balances":balances, "programState":program_state,
        "dutyPresent":!snapshot.liabilities.is_empty(), "dutyAmount":duty_amount})
}

fn capture_values(site: u16, captures: &[NamedValue]) -> Vec<u64> {
    let names: &[&str] = match site {
        0 => &[],
        1 => &["extra", "held"],
        2 => &["extra", "held", "quote"],
        _ => panic!("outside the two-await reserve trace"),
    };
    assert_eq!(
        captures
            .iter()
            .map(|value| value.name.as_str())
            .collect::<Vec<_>>()
            .as_slice(),
        names
    );
    captures.iter().map(|value| uint(&value.value)).collect()
}

fn control(snapshot: &Snapshot, site: u16, captures: &[NamedValue], committed: UInt) -> Json {
    let mut result = boundary(snapshot);
    let fields = result.as_object_mut().unwrap();
    fields.insert("specimen".into(), json!("Reserve"));
    fields.insert("site".into(), json!(site));
    fields.insert("captures".into(), json!(capture_values(site, captures)));
    fields.insert("committed".into(), json!(number(committed)));
    result
}

fn resume_effects(effects: &[Effect]) -> Vec<Json> {
    effects
        .iter()
        .map(|effect| match effect {
            Effect::State {
                name,
                before,
                after,
            } => json!({
                "tag":"StateValue", "name":name, "before":uint(before), "after":uint(after)
            }),
            Effect::Transfer {
                asset,
                from,
                to,
                amount,
            } => {
                reserve_asset(asset);
                assert_eq!((from.as_str(), to.as_str()), ("reserve", "target"));
                json!({"tag":"TransferAmount", "amount":number(*amount)})
            }
            Effect::Liability {
                name,
                asset,
                debtor,
                creditor,
                before,
                after,
            } => {
                allocation(name, asset, debtor, creditor);
                json!({"tag":"DutyValue", "before":number(*before), "after":number(*after)})
            }
            Effect::Fee {
                asset,
                from,
                to,
                amount,
            } => {
                reserve_asset(asset);
                assert_eq!((from.as_str(), to.as_str()), ("reserve", "treasury"));
                json!({"tag":"FeeAmount", "amount":number(*amount)})
            }
            Effect::Emit { name, value } => {
                assert_eq!(name, "settled_quote");
                json!({"tag":"EmitQuote", "quote":uint(value)})
            }
        })
        .collect()
}

fn financial_frame(
    snapshot: &Snapshot,
    input: UInt,
    site: u16,
    captures: &[NamedValue],
    attempted: UInt,
    committed: UInt,
    effects: Vec<Json>,
) -> Json {
    let data = boundary(snapshot);
    json!({"specimen":"Reserve", "inputValue":number(input),
        "balances":data["balances"], "programState":data["programState"],
        "dutyPresent":data["dutyPresent"], "dutyAmount":data["dutyAmount"],
        "pendingSite":site, "captureValues":capture_values(site, captures),
        "logicalAttempted":number(attempted), "logicalCommitted":number(committed),
        "effects":effects})
}

fn segment_entry(segment: &StageSegment, previous: Option<&Continuation>) {
    assert_eq!(segment.action, "allocate");
    assert_eq!(segment.execution.action, "allocate");
    assert_eq!(segment.execution.semantics, "moriarty-core/2");
    if let Some(previous) = previous {
        assert_eq!(segment.frame_pre.site, previous.site);
        assert_eq!(segment.frame_pre.captures, previous.captures);
        assert_eq!(segment.execution.pre, previous.boundary);
    } else {
        assert_eq!(segment.frame_pre.site, 0);
        assert!(segment.frame_pre.captures.is_empty());
    }
}

fn invocation(
    status: &str,
    attempted: UInt,
    advanced: bool,
    segment: Option<&StageSegment>,
    new_continuation: bool,
) -> Json {
    let effects = segment.map_or_else(Vec::new, |segment| {
        resume_effects(&segment.execution.effects)
    });
    let steps: Vec<_> = segment
        .into_iter()
        .flat_map(|segment| &segment.execution.steps)
        .map(|step| step.index)
        .collect();
    json!({"status":status, "attempted":number(attempted), "advanced":advanced,
        "hasSegment":segment.is_some(), "newContinuation":new_continuation,
        "effects":effects, "sourceSteps":steps, "errorSteps":[]})
}

fn observe(
    outcome: StageOutcome,
    previous: &Continuation,
    total: &mut u64,
) -> (Json, Option<Continuation>) {
    let (c, last, attempted, pending) = match outcome {
        StageOutcome::Waiting {
            advanced,
            segment,
            continuation,
            attempted,
            residual,
        } => {
            assert_eq!(residual, continuation.boundary.liabilities);
            if advanced {
                let segment = segment.as_ref().expect("advanced invocation has a segment");
                segment_entry(segment, Some(previous));
                assert_eq!(segment.frame_post.site, continuation.site);
                assert_eq!(segment.frame_post.captures, continuation.captures);
                assert_eq!(segment.execution.post, continuation.boundary);
            } else {
                assert!(segment.is_none());
                assert_eq!(&continuation, previous);
            }
            let c = control(
                &continuation.boundary,
                continuation.site,
                &continuation.captures,
                continuation.logical.committed,
            );
            let last = invocation("Waiting", attempted, advanced, segment.as_ref(), advanced);
            (c, last, attempted, Some(continuation))
        }
        StageOutcome::Complete {
            segment,
            committed,
            attempted,
            residual,
        } => {
            segment_entry(&segment, Some(previous));
            assert_eq!(segment.frame_post.site, 0);
            assert!(segment.frame_post.captures.is_empty());
            assert_eq!(residual, segment.execution.post.liabilities);
            // A completed abstract model c is a post-state, never a Rust Continuation.
            let c = control(&segment.execution.post, 0, &[], committed);
            let last = invocation("Complete", attempted, true, Some(&segment), false);
            (c, last, attempted, None)
        }
    };
    *total += number(attempted);
    (
        json!({"c":c, "last":last, "attemptedTotal":*total}),
        pending,
    )
}

fn event(label: &str, name: &str, value: Value) -> StageEvent {
    StageEvent {
        format: stage::STAGE_EVENT_FORMAT.into(),
        action: "allocate".into(),
        label: label.into(),
        values: vec![NamedValue {
            name: name.into(),
            value,
        }],
    }
}

#[test]
fn reserve_existing_trace_matches_complete_shared_quint_projection() {
    let expected: Json = serde_json::from_str(EXPECTED).unwrap();
    let scenario: Scenario = serde_json::from_str(OPENING).unwrap();
    let input = match scenario.inputs.get("extra").unwrap() {
        Value::UInt(input) => *input,
        Value::Bool(_) => panic!("extra is UInt"),
    };
    let opening = Snapshot {
        state: scenario.state.clone(),
        balances: scenario.balances.clone(),
        liabilities: scenario.liabilities.clone(),
    };
    let initial = financial_frame(&opening, input, 0, &[], UInt(0), UInt(0), vec![]);
    let program = stage::load(SOURCE).unwrap();
    let StageOutcome::Waiting {
        advanced,
        segment: Some(segment),
        continuation: first,
        attempted,
        residual,
    } = stage::start(&program, "allocate", scenario, Some(UInt(36))).unwrap()
    else {
        panic!("reserve starts at its first await")
    };
    assert!(advanced);
    segment_entry(&segment, None);
    assert_eq!(segment.execution.pre, opening);
    assert_eq!(segment.frame_post.site, first.site);
    assert_eq!(segment.frame_post.captures, first.captures);
    assert_eq!(segment.execution.post, first.boundary);
    assert_eq!(residual, first.boundary.liabilities);
    assert_eq!(
        segment
            .execution
            .steps
            .iter()
            .map(|step| step.index)
            .collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
    let prefix_effects: Vec<_> = segment
        .execution
        .effects
        .iter()
        .map(|effect| match effect {
            Effect::State {
                name,
                before,
                after,
            } => {
                assert_eq!(name, "basis");
                json!({"tag":"StateChange", "before":uint(before), "after":uint(after)})
            }
            Effect::Liability {
                name,
                asset,
                debtor,
                creditor,
                before,
                after,
            } => {
                allocation(name, asset, debtor, creditor);
                json!({"tag":"LiabilityChange", "before":number(*before), "after":number(*after)})
            }
            _ => panic!("reserve prefix has only basis and duty effects"),
        })
        .collect();
    let prefix = financial_frame(
        &first.boundary,
        input,
        first.site,
        &first.captures,
        attempted,
        first.logical.committed,
        prefix_effects,
    );
    // Lifecycle init is a pure projection of the prefix, with no resume invoked.
    let mut total = number(attempted);
    let mut observations = vec![json!({
        "c":control(&first.boundary, first.site, &first.captures, first.logical.committed),
        "last":invocation("NotInvoked", UInt(0), false, None, false), "attemptedTotal":total
    })];
    let mut pending = Some(first);
    for event in [
        event("approval", "quote", Value::UInt(UInt(41))),
        event("approval", "quote", Value::UInt(UInt(43))),
        event("settlement", "ready", Value::Bool(false)),
        event("settlement", "ready", Value::Bool(true)),
    ] {
        let previous = pending
            .take()
            .expect("the first three invocations leave a continuation");
        let actual = stage::resume(&program, previous.clone(), event, None, None).unwrap();
        let (observation, next) = observe(actual, &previous, &mut total);
        observations.push(observation);
        pending = next;
    }
    assert!(pending.is_none());
    let actual = json!({"initial":initial, "prefix":prefix, "observations":observations});
    assert_eq!(actual, expected);
    println!(
        "RESERVE_QUINT_PROJECTION {}",
        serde_json::to_string(&actual).unwrap()
    );
}

```

## WHOLE FILE candidate/models/core-stage-correspondence/reserve_shared_projection_test.qnt

```
// One existing normal reserve trajectory; no model actions or domains change.
module reserve_shared_projection_test {
  import continuation_lifecycle.* from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_lifecycle"
  import continuation_prefix.Reserve from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.FinancialFrame from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.initialFrame from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.applyStart from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.StateChange from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.LiabilityChange from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"

  pure val expectedInitial: FinancialFrame = {
    specimen: Reserve,
    inputValue: 2,
    balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
    programState: Map("allocated" -> 0, "basis" -> 40, "marker" -> 77),
    dutyPresent: true,
    dutyAmount: 5,
    pendingSite: 0,
    captureValues: List(),
    logicalAttempted: 0,
    logicalCommitted: 0,
    effects: List(),
  }

  pure val expectedPrefix: FinancialFrame = {
    specimen: Reserve,
    inputValue: 2,
    balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
    programState: Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
    dutyPresent: true,
    dutyAmount: 47,
    pendingSite: 1,
    captureValues: List(2, 42),
    logicalAttempted: 11,
    logicalCommitted: 11,
    effects: List(StateChange({ before: 40, after: 35 }), LiabilityChange({ before: 5, after: 47 })),
  }

  pure val expected0: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
      programState: Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 47,
      site: 1,
      captures: List(2, 42),
      committed: 11,
    },
    last: {
      status: NotInvoked,
      attempted: 0,
      advanced: false,
      hasSegment: false,
      newContinuation: false,
      effects: List(),
      sourceSteps: List(),
      errorSteps: List(),
    },
    attemptedTotal: 11,
  }

  pure val expected1: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
      programState: Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 47,
      site: 1,
      captures: List(2, 42),
      committed: 11,
    },
    last: {
      status: Waiting,
      attempted: 4,
      advanced: false,
      hasSegment: false,
      newContinuation: false,
      effects: List(),
      sourceSteps: List(),
      errorSteps: List(),
    },
    attemptedTotal: 15,
  }

  pure val expected2: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 58, "target" -> 42, "treasury" -> 0),
      programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 5,
      site: 2,
      captures: List(2, 42, 43),
      committed: 26,
    },
    last: {
      status: Waiting,
      attempted: 15,
      advanced: true,
      hasSegment: true,
      newContinuation: true,
      effects: List(StateValue({ name: "allocated", before: 0, after: 42 }), TransferAmount(42), DutyValue({ before: 47, after: 5 }), StateValue({ name: "basis", before: 35, after: 78 })),
      sourceSteps: List(3, 4, 5, 6, 7),
      errorSteps: List(),
    },
    attemptedTotal: 30,
  }

  pure val expected3: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 58, "target" -> 42, "treasury" -> 0),
      programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 5,
      site: 2,
      captures: List(2, 42, 43),
      committed: 26,
    },
    last: {
      status: Waiting,
      attempted: 2,
      advanced: false,
      hasSegment: false,
      newContinuation: false,
      effects: List(),
      sourceSteps: List(),
      errorSteps: List(),
    },
    attemptedTotal: 32,
  }

  pure val expected4: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 56, "target" -> 42, "treasury" -> 2),
      programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 5,
      site: 0,
      captures: List(),
      committed: 36,
    },
    last: {
      status: Complete,
      attempted: 10,
      advanced: true,
      hasSegment: true,
      newContinuation: false,
      effects: List(FeeAmount(2), EmitQuote(43)),
      sourceSteps: List(8, 9, 10, 11),
      errorSteps: List(),
    },
    attemptedTotal: 42,
  }

  run reserveSharedProjectionTraceTest =
    initCase(Reserve)
      .expect(all {
        initialFrame(Reserve) == expectedInitial,
        applyStart(initialFrame(Reserve)) == expectedPrefix,
        observation == expected0,
      })
      .then(resume(Approval(41)))
      .expect(observation == expected1)
      .then(resume(Approval(43)))
      .expect(observation == expected2)
      .then(resume(Settlement(false)))
      .expect(observation == expected3)
      .then(resume(Settlement(true)))
      .expect(observation == expected4)
}

```

## WHOLE FILE actual-check-a1/decode_reserve_itf.py

```
"""Decode only the five reserve lifecycle Observation states in Quint 0.32.0 ITF.

This module does not execute a model, generate trace states, or authenticate input
provenance. The caller supplies a captured file and an independent expectation.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

STATE_COUNT = 5
NORMAL_MAX = 142
PENDING_FIELDS = {
    "specimen", "balances", "programState", "dutyPresent", "dutyAmount",
    "site", "captures", "committed",
}
INVOCATION_FIELDS = {
    "status", "attempted", "advanced", "hasSegment", "newContinuation",
    "effects", "sourceSteps", "errorSteps",
}
STATUSES = {"NotInvoked", "Waiting", "Complete", "BusinessRejected"}


class DecodeError(ValueError):
    """The input does not have the explicitly supported normal ITF shape."""


def fail(path: str, message: str) -> None:
    raise DecodeError(f"{path}: {message}")


def record(value: Any, fields: set[str], path: str) -> dict[str, Any]:
    if type(value) is not dict:
        fail(path, "expected an object")
    if set(value) != fields:
        fail(path, f"expected fields {sorted(fields)}, found {sorted(value)}")
    return value


def array(value: Any, path: str) -> list[Any]:
    if type(value) is not list:
        fail(path, "expected an array")
    return value


def string(value: Any, path: str) -> str:
    if type(value) is not str:
        fail(path, "expected a string")
    return value


def boolean(value: Any, path: str) -> bool:
    if type(value) is not bool:
        fail(path, "expected a boolean")
    return value


def integer(value: Any, path: str) -> int:
    wrapped = record(value, {"#bigint"}, path)
    decimal = string(wrapped["#bigint"], path + "/#bigint")
    if re.fullmatch(r"0|[1-9][0-9]*", decimal) is None:
        fail(path, "expected a canonical nonnegative normal-example integer")
    decoded = int(decimal)
    if decoded > NORMAL_MAX:
        fail(path, "outside the disclosed common adapter's normal 0..142 domain")
    return decoded


def integers(value: Any, path: str) -> list[int]:
    return [integer(item, f"{path}/{i}") for i, item in enumerate(array(value, path))]


def unit_variant(value: Any, tags: set[str], path: str) -> str:
    variant = record(value, {"tag", "value"}, path)
    tag = string(variant["tag"], path + "/tag")
    if tag not in tags:
        fail(path, f"expected a declared unit constructor in {sorted(tags)}")
    # ToIrListener.unitValue is Tup([]); itf.toItf maps it to {"#tup":[]}.
    payload = record(variant["value"], {"#tup"}, path + "/value")
    if array(payload["#tup"], path + "/value/#tup"):
        fail(path, "a nullary constructor must retain its empty-tuple payload")
    return tag


def integer_map(value: Any, path: str) -> dict[str, int]:
    entries = array(record(value, {"#map"}, path)["#map"], path + "/#map")
    result: dict[str, int] = {}
    for i, item in enumerate(entries):
        pair = array(item, f"{path}/#map/{i}")
        if len(pair) != 2:
            fail(f"{path}/#map/{i}", "expected one key/value pair")
        key = string(pair[0], f"{path}/#map/{i}/0")
        if key in result:
            fail(path, f"duplicate map key {key!r}")
        result[key] = integer(pair[1], f"{path}/#map/{i}/1")
    return result


def effect(value: Any, path: str) -> dict[str, Any]:
    variant = record(value, {"tag", "value"}, path)
    tag = string(variant["tag"], path + "/tag")
    payload = variant["value"]
    if tag == "StateValue":
        data = record(payload, {"name", "before", "after"}, path + "/value")
        return {"tag": tag, "name": string(data["name"], path + "/value/name"),
                "before": integer(data["before"], path + "/value/before"),
                "after": integer(data["after"], path + "/value/after")}
    if tag == "DutyValue":
        data = record(payload, {"before", "after"}, path + "/value")
        return {"tag": tag,
                "before": integer(data["before"], path + "/value/before"),
                "after": integer(data["after"], path + "/value/after")}
    if tag in {"TransferAmount", "FeeAmount"}:
        return {"tag": tag, "amount": integer(payload, path + "/value")}
    if tag == "EmitQuote":
        return {"tag": tag, "quote": integer(payload, path + "/value")}
    fail(path, f"unsupported ResumeEffect constructor {tag!r}")


def pending(value: Any, path: str) -> dict[str, Any]:
    data = record(value, PENDING_FIELDS, path)
    return {
        "specimen": unit_variant(data["specimen"], {"Reserve"}, path + "/specimen"),
        "balances": integer_map(data["balances"], path + "/balances"),
        "programState": integer_map(data["programState"], path + "/programState"),
        "dutyPresent": boolean(data["dutyPresent"], path + "/dutyPresent"),
        "dutyAmount": integer(data["dutyAmount"], path + "/dutyAmount"),
        "site": integer(data["site"], path + "/site"),
        "captures": integers(data["captures"], path + "/captures"),
        "committed": integer(data["committed"], path + "/committed"),
    }


def invocation(value: Any, path: str) -> dict[str, Any]:
    data = record(value, INVOCATION_FIELDS, path)
    return {
        "status": unit_variant(data["status"], STATUSES, path + "/status"),
        "attempted": integer(data["attempted"], path + "/attempted"),
        "advanced": boolean(data["advanced"], path + "/advanced"),
        "hasSegment": boolean(data["hasSegment"], path + "/hasSegment"),
        "newContinuation": boolean(data["newContinuation"], path + "/newContinuation"),
        "effects": [effect(item, f"{path}/effects/{i}")
                    for i, item in enumerate(array(data["effects"], path + "/effects"))],
        "sourceSteps": integers(data["sourceSteps"], path + "/sourceSteps"),
        "errorSteps": integers(data["errorSteps"], path + "/errorSteps"),
    }


def observation(value: Any, path: str) -> dict[str, Any]:
    data = record(value, {"c", "last", "attemptedTotal"}, path)
    return {"c": pending(data["c"], path + "/c"),
            "last": invocation(data["last"], path + "/last"),
            "attemptedTotal": integer(data["attemptedTotal"], path + "/attemptedTotal")}


def decode_trace(document: Any, variable: str) -> list[dict[str, Any]]:
    """Return exactly the captured Observation sequence; invent no other frames."""
    data = record(document, {"#meta", "vars", "states"}, "ITF")
    meta = data["#meta"]
    if type(meta) is not dict or meta.get("format") != "ITF" or meta.get("status") != "passed":
        fail("ITF/#meta", "expected CLI ITF metadata with status=passed")
    if array(data["vars"], "ITF/vars") != [variable]:
        fail("ITF/vars", "expected exactly the explicitly named observation variable")
    states = array(data["states"], "ITF/states")
    if len(states) != STATE_COUNT:
        fail("ITF/states", "expected exactly five captured lifecycle states")
    decoded = []
    for i, state in enumerate(states):
        state = record(state, {"#meta", variable}, f"ITF/states/{i}")
        state_meta = record(state["#meta"], {"index"}, f"ITF/states/{i}/#meta")
        if type(state_meta["index"]) is not int or state_meta["index"] != i:
            fail(f"ITF/states/{i}/#meta/index", "expected the original ordered state index")
        decoded.append(observation(state[variable], f"ITF/states/{i}/{variable}"))
    return decoded


def expected_observations(value: Any) -> list[Any]:
    """Select only the expected observations, explicitly excluding pure frames."""
    if type(value) is dict:
        if not set(value).issubset({"initial", "prefix", "observations"}) or "observations" not in value:
            fail("expected", "expected the documented common projection wrapper")
        value = value["observations"]
    result = array(value, "expected/observations")
    if len(result) != STATE_COUNT:
        fail("expected/observations", "expected exactly five frozen observations")
    return result


def pointer(path: str, key: str | int) -> str:
    return path + "/" + str(key).replace("~", "~0").replace("/", "~1")


def first_difference(actual: Any, expected: Any, path: str = "") -> dict[str, Any] | None:
    # Python considers True == 1; shared JSON comparison must retain type equality.
    if type(actual) is not type(expected):
        return {"path": path, "reason": "JSON types differ",
                "actual_type": type(actual).__name__, "expected_type": type(expected).__name__}
    if type(actual) is dict:
        if set(actual) != set(expected):
            return {"path": path, "reason": "record/map keys differ",
                    "actual_keys": sorted(actual), "expected_keys": sorted(expected)}
        for key in sorted(actual):
            mismatch = first_difference(actual[key], expected[key], pointer(path, key))
            if mismatch is not None:
                return mismatch
    elif type(actual) is list:
        if len(actual) != len(expected):
            return {"path": path, "reason": "array lengths differ",
                    "actual_length": len(actual), "expected_length": len(expected)}
        for i, (left, right) in enumerate(zip(actual, expected)):
            mismatch = first_difference(left, right, pointer(path, i))
            if mismatch is not None:
                return mismatch
    elif actual != expected:
        return {"path": path, "reason": "values differ", "actual": actual, "expected": expected}
    return None


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise DecodeError(f"duplicate JSON object key {key!r}")
        result[key] = value
    return result


def invalid_constant(value: str) -> None:
    raise DecodeError(f"non-JSON numeric constant {value}")


def read_document(path: Path) -> tuple[Any, str]:
    raw = path.read_bytes()
    return (json.loads(raw.decode("utf-8"), object_pairs_hook=unique_object,
                       parse_constant=invalid_constant), hashlib.sha256(raw).hexdigest())


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True, help="actual captured ITF file")
    parser.add_argument("--variable", required=True, help="exact declared ITF variable name")
    parser.add_argument("--expected", type=Path, required=True,
                        help="separately frozen common projection or five-observation array")
    parser.add_argument("--output", type=Path, required=True,
                        help="new file for decoded input observations; never initial/prefix")
    args = parser.parse_args(argv)
    try:
        document, input_hash = read_document(args.input)
        reference, expected_hash = read_document(args.expected)
        decoded = decode_trace(document, args.variable)
        expected = expected_observations(reference)
        mismatch = first_difference(decoded, expected, "/observations")
        output = (json.dumps({"observations": decoded}, indent=2, sort_keys=True) + "\n").encode()
        with args.output.open("xb") as stream:
            stream.write(output)
        receipt = {"status": "matched" if mismatch is None else "mismatch",
                   "scope": "five captured lifecycle Observation records only",
                   "variable": args.variable, "state_count": len(decoded),
                   "input_sha256": input_hash, "expected_sha256": expected_hash,
                   "output_sha256": hashlib.sha256(output).hexdigest(),
                   "first_difference": mismatch}
        print(json.dumps(receipt, sort_keys=True))
        return 0 if mismatch is None else 1
    except (DecodeError, OSError, UnicodeError, json.JSONDecodeError, RecursionError) as error:
        print(json.dumps({"status": "error", "error": str(error)}, sort_keys=True), file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())

```

## WHOLE FILE compare_actual.py

```
from pathlib import Path
import json,hashlib,subprocess,datetime,importlib.util
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');C=R/'core2-quint-shared-projection-check-cycle/actual-check-a1';D=R/'reserve-itf-decoder-source-20261018';O=R/'reserve-quint-independent-oracle-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
for line in (O/'FREEZE.sha256').read_text().splitlines():
 h,n=line.split(maxsplit=1);assert sha(O/n.lstrip('*'))==h
code=(D/'decode_reserve_itf.py').read_bytes();decoder=C/'decode_reserve_itf.py';decoder.write_bytes(code)
write(C/'DECODER-CUSTODY.json',{'source':str(D/'decode_reserve_itf.py'),'sha256':sha(decoder),'rootReadWholeSource':True,'sourceGrounding':'Pinned Quint0.32.0 ITF serializer + original lifecycle/prefix type declarations. Author wrote decoder before actualtrace exposure, reported no subsequentcodechange.','runtime':'Python stdlib only, file parsing/comparison, no model/native/SDK execution.'})
argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','10s','/usr/bin/python3',str(decoder),'--input',str(C/'reserve_reserveSharedProjectionTraceTest_0.itf.json'),'--variable','observation','--expected',str(O/'reserve-common-projection.expected.json'),'--output',str(C/'quint-decoded-observations.json')]
env={'PATH':'/usr/bin:/bin','LANG':'C.UTF-8','PYTHONDONTWRITEBYTECODE':'1'}
write(C/'DECODER-COMMAND.json',{'argv':argv,'environment':env,'cwd':str(C)})
p=subprocess.run(argv,cwd=C,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE);(C/'DECODER-STDOUT.json').write_bytes(p.stdout);(C/'DECODER-STDERR.log').write_bytes(p.stderr);assert p.returncode==0,(p.returncode,p.stderr.decode());assert not p.stderr
spec=importlib.util.spec_from_file_location('reserve_decoder',decoder);mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
lines=(C/'rust-shared-projection/STDOUT.log').read_text().splitlines();prefix='RESERVE_QUINT_PROJECTION ';selected=[l[len(prefix):] for l in lines if l.startswith(prefix)];assert len(selected)==1
rust=json.loads(selected[0]);write(C/'rust-actual-common-projection.json',rust)
expected=json.loads((O/'reserve-common-projection.expected.json').read_text());quint=json.loads((C/'quint-decoded-observations.json').read_text())['observations']
comparisons={'rustEntireProjectionVsIndependentOracle':mod.first_difference(rust,expected),'quintCapturedFiveObservationsVsIndependentOracle':mod.first_difference(quint,expected['observations']),'quintCapturedFiveObservationsVsActualRust':mod.first_difference(quint,rust['observations'])}
assert all(v is None for v in comparisons.values()),comparisons
result={'recordedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'MATCHED','comparisons':comparisons,'independentFreezeSha256':sha(O/'FREEZE.sha256'),'independentExpectedSha256':sha(O/'reserve-common-projection.expected.json'),'rustStdoutSha256':sha(C/'rust-shared-projection/STDOUT.log'),'rustProjectionSha256':sha(C/'rust-actual-common-projection.json'),'quintRawCaptureSha256':sha(C/'reserve_reserveSharedProjectionTraceTest_0.itf.json'),'quintDecodedSha256':sha(C/'quint-decoded-observations.json'),'decoderSha256':sha(decoder),'capturedModelObservations':len(quint),'committedLogicalWork':[s['c']['committed'] for s in quint],'cumulativeAttemptedLogicalWork':[s['attemptedTotal'] for s in quint],'residualDutyFinal':quint[-1]['c']['dutyAmount'],'limitations':['One existing finite normal reserve trajectory; adapter domain0..142 and declared effect/record/map shapes.','Quint initial/prefix FinancialFrames are pure assertions, not raw emitted trace variables; Rust frames separately equal independent expected.','No exhaustive invariant proof, K/Lean/circuit/native/proof/ledger/target correspondence acceptance.','Independent custodian related prior exposure is disclosed; no global blindness claim.']}
write(C/'CAPTURED-COMPARISON.json',result);print(json.dumps(result))

```

## WHOLE FILE compare_actual_a2.py

```
from pathlib import Path
import json,hashlib,subprocess,datetime,importlib.util
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');C=R/'core2-quint-shared-projection-check-cycle/actual-check-a1';D=R/'reserve-itf-decoder-source-20261018';O=R/'reserve-quint-independent-oracle-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
for line in (O/'FREEZE.sha256').read_text().splitlines():
 h,n=line.split(maxsplit=1);assert sha(O/n.lstrip('*'))==h
code=(D/'decode_reserve_itf.py').read_bytes();decoder=C/'decode_reserve_itf.py';decoder.write_bytes(code)
write(C/'DECODER-CUSTODY.json',{'source':str(D/'decode_reserve_itf.py'),'sha256':sha(decoder),'rootReadWholeSource':True,'sourceGrounding':'Pinned Quint0.32.0 ITF serializer + original lifecycle/prefix type declarations. Author wrote decoder before actualtrace exposure, reported no subsequentcodechange.','runtime':'Python stdlib only, file parsing/comparison, no model/native/SDK execution.'})
assert json.loads((C/'DECODER-STDOUT.json').read_text())['status']=='matched'
spec=importlib.util.spec_from_file_location('reserve_decoder',decoder);mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
lines=(C/'rust-shared-projection/STDOUT.log').read_text().splitlines();prefix='RESERVE_QUINT_PROJECTION ';selected=[l.partition(prefix) for l in lines if prefix in l];assert len(selected)==1;assert selected[0][0]=='test reserve_existing_trace_matches_complete_shared_quint_projection ... ';selected=[selected[0][2]]
rust=json.loads(selected[0]);write(C/'rust-actual-common-projection.json',rust)
expected=json.loads((O/'reserve-common-projection.expected.json').read_text());quint=json.loads((C/'quint-decoded-observations.json').read_text())['observations']
comparisons={'rustEntireProjectionVsIndependentOracle':mod.first_difference(rust,expected),'quintCapturedFiveObservationsVsIndependentOracle':mod.first_difference(quint,expected['observations']),'quintCapturedFiveObservationsVsActualRust':mod.first_difference(quint,rust['observations'])}
assert all(v is None for v in comparisons.values()),comparisons
result={'recordedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'MATCHED','comparisons':comparisons,'independentFreezeSha256':sha(O/'FREEZE.sha256'),'independentExpectedSha256':sha(O/'reserve-common-projection.expected.json'),'rustStdoutSha256':sha(C/'rust-shared-projection/STDOUT.log'),'rustProjectionSha256':sha(C/'rust-actual-common-projection.json'),'quintRawCaptureSha256':sha(C/'reserve_reserveSharedProjectionTraceTest_0.itf.json'),'quintDecodedSha256':sha(C/'quint-decoded-observations.json'),'decoderSha256':sha(decoder),'capturedModelObservations':len(quint),'committedLogicalWork':[s['c']['committed'] for s in quint],'cumulativeAttemptedLogicalWork':[s['attemptedTotal'] for s in quint],'residualDutyFinal':quint[-1]['c']['dutyAmount'],'limitations':['One existing finite normal reserve trajectory; adapter domain0..142 and declared effect/record/map shapes.','Quint initial/prefix FinancialFrames are pure assertions, not raw emitted trace variables; Rust frames separately equal independent expected.','No exhaustive invariant proof, K/Lean/circuit/native/proof/ledger/target correspondence acceptance.','Independent custodian related prior exposure is disclosed; no global blindness claim.']}
write(C/'CAPTURED-COMPARISON.json',result);print(json.dumps(result))

```

## WHOLE FILE independent-oracle/FREEZE.json

```
{
  "all14_original_gaps_retained": true,
  "all_parent_and_composition_exits_unchanged": true,
  "basis_commit": "e4d7a2b5709caa18a94f5256d03a0302364b8235",
  "codec_sha256": "c48480f528ba45243d810328c5c628bbcb9bad9046e1946aa0773d53251b5ab8",
  "complete_selected_objects": {
    "Observation": 5,
    "full_source_financial_prefixes": 14,
    "initial_FinancialFrame": 1,
    "prefix_FinancialFrame": 1
  },
  "exposure": "Prior related DTO/multi/oracle exposure disclosed; current original normative model constants read after source arithmetic; no new candidate/test/expected/trace body read",
  "input_manifest_sha256": "7c75c89553e0338a4ae2935534edc51591b3e79b81d7ab424b0dbe870c2b932b",
  "inventory_members": 33,
  "inventory_sha256": "d2681353988bfffc41e118dd34827095b2f58769e23da525318a676e303ea08d",
  "model_attempted_totals": [
    11,
    15,
    30,
    32,
    42
  ],
  "opening_created_discharged_closing_duty": [
    "5 present",
    "42",
    "42",
    "5 present"
  ],
  "remaining_bridge": "Actual ITF capture/decoder-to-common-normalization and actual static-context/Rust/model comparisons unobserved",
  "representation_premise": "AUTHOR-SUPPLIED proposed common JSON codec; NOT actual Quint ITF or independently established compiler wire",
  "source_committed": [
    "11",
    "11",
    "26",
    "26",
    "36"
  ],
  "source_invocation_attempted": [
    "11",
    "4",
    "15",
    "2",
    "10"
  ],
  "static_checks": "Input pins, typed field sets/maps/lists, split/aggregate equality, prefix conservation/effects/work, unchanged false pending state, prior freeze hashes",
  "status": "FROZEN before future actual consumer; model/Rust/product execution UNRUN",
  "typed_values": "MODEL-TYPED-VALUES.qnt.literal; documentary values, not runnable test/observed trace",
  "whole_projection_sha256": "519d2972fc58d74c824a8f514b8a0176c3849b9848bbc0b99729b4bd9e92c8e8"
}

```

## WHOLE FILE independent-oracle/PREMISES-AND-OMISSIONS.json

```
{
  "actual_product_or_model_execution": "UNRUN",
  "all14_original_staged_gaps": "OPEN, unchanged",
  "all_original_roadmap_and_composition_exits": "unchanged; no parent closure",
  "basis_commit": "e4d7a2b5709caa18a94f5256d03a0302364b8235 (pinned by input-only custodian)",
  "codec": {
    "path": "COMMON-PROJECTION-CODEC.json",
    "sha256": "c48480f528ba45243d810328c5c628bbcb9bad9046e1946aa0773d53251b5ab8",
    "status": "AUTHOR-SUPPLIED proposed common normalization, conditional; NOT actual Quint ITF or independent compiler-wire proof"
  },
  "explicit_unknown_bridge": [
    "Actual Quint ITF encoding/decoder and documented conversion into proposed common codec",
    "Actual Rust/model execution/result correspondence",
    "Actual static-context adapter checks; no authentication or history certification"
  ],
  "initialization_note": "FinancialFrame initial/prefix separate; Observation0 is after prefix with NotInvoked, attempted0 and attemptedTotal11; completed c is abstract post-state not emitted Rust Continuation",
  "input_only_manifest_sha256": "7c75c89553e0338a4ae2935534edc51591b3e79b81d7ab424b0dbe870c2b932b",
  "inputs": {
    "action": "allocate",
    "events": [
      "Approval(41)",
      "Approval(43)",
      "Settlement(false)",
      "Settlement(true)"
    ],
    "resume_overrides": {
      "expected_action": [
        null,
        null,
        null,
        null
      ],
      "logical_cap": [
        null,
        null,
        null,
        null
      ]
    },
    "scenario": "reserve-scenario.json",
    "source": "reserve.mori",
    "start_cap": "36"
  },
  "presence_note": "dutyPresent separately models absence; selected reserve starts/ends present5 and never reaches zero/absence; no extra zero/absence coverage claimed",
  "prior_exposure": [
    "Earlier R3/Core1/Core2/Source6/financial/composition/multi-condition oracle custody",
    "Public DTO reads and earlier incidental residual/InputDecl conversion helper snippets",
    "Current original Quint specification constants, read after independently deriving source financial/work arithmetic",
    "Current DTO extraction incidentally displayed duplicate historical nested Step/Execution declarations, not implementation bodies"
  ],
  "security_authority_judgment": "None supplied",
  "shared_projection_omissions": [
    "Raw Statement/ExprId",
    "outer Step.value",
    "all raw per-Step snapshots/deltas",
    "language/semantics/work-profile fields",
    "cap and cap-refusal policy",
    "Program/Plan local descriptors",
    "Continuation format/action/capture names/types",
    "residual full DTO identity",
    "Error code/message/offset/step",
    "public report qualifications",
    "stdout/stderr/exit bytes",
    "native/circuit/ledger economics"
  ],
  "static_context_required_before_Rust_projection": {
    "account_set": [
      "observer",
      "reserve",
      "target",
      "treasury"
    ],
    "action": "allocate",
    "agreement": "ReserveContinuationOracle",
    "alias": "CREDIT",
    "asset": {
      "issuer": "continuation-reserve-oracle",
      "network": "local",
      "reference": "UNIT",
      "scale": 0
    },
    "duty": {
      "asset": "exact same CREDIT identity",
      "creditor": "target",
      "debtor": "reserve",
      "name": "allocation"
    },
    "input_extra_UInt": "2",
    "site1": {
      "capture_names_types": [
        "extra:UInt",
        "held:UInt"
      ],
      "label": "approval"
    },
    "site2": {
      "capture_names_types": [
        "extra:UInt",
        "held:UInt",
        "quote:UInt"
      ],
      "label": "settlement"
    },
    "specimen": "Reserve",
    "state_fields_all_UInt": [
      "allocated",
      "basis",
      "marker"
    ]
  }
}

```

## WHOLE FILE actual-check-a1/RESULT.json

```
{
  "recordedAt": "2026-10-18T12:20:13.090966+00:00",
  "commit": "f15214ab7a885a4dc3194002cbacbb56841fdcbf",
  "tree": "0d0a663592893e239ddea82bab5c88eb494e95e3",
  "steps": [
    {
      "finishedAt": "2026-10-18T12:20:10.650848+00:00",
      "name": "rust-shared-projection",
      "exitCode": 0,
      "wallSeconds": 28.535909085003368,
      "stdoutSha256": "9584a4747896184cf0e996008756892f34b276d6c9fca27d9a8b10346191da82",
      "stderrSha256": "69bd2489876e80d4f3381d8104611302d5ef0a15bec5e73cd6d5bc6bf412937f",
      "commitAfter": "f15214ab7a885a4dc3194002cbacbb56841fdcbf",
      "treeAfter": "0d0a663592893e239ddea82bab5c88eb494e95e3",
      "statusAfter": ""
    },
    {
      "finishedAt": "2026-10-18T12:20:11.162869+00:00",
      "name": "rust-continuation-helper",
      "exitCode": 0,
      "wallSeconds": 0.4036930590009433,
      "stdoutSha256": "6835e9f3d8e52257d69f78c59a466084726e33325624d6e91b51f6f0dad3e6c2",
      "stderrSha256": "a4a04d178396c8a6ec93a5be9629725cede047fdf62e03c153ad09dfc8049029",
      "commitAfter": "f15214ab7a885a4dc3194002cbacbb56841fdcbf",
      "treeAfter": "0d0a663592893e239ddea82bab5c88eb494e95e3",
      "statusAfter": ""
    },
    {
      "finishedAt": "2026-10-18T12:20:12.055003+00:00",
      "name": "quint-typecheck",
      "exitCode": 0,
      "wallSeconds": 0.7910851380001986,
      "stdoutSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "commitAfter": "f15214ab7a885a4dc3194002cbacbb56841fdcbf",
      "treeAfter": "0d0a663592893e239ddea82bab5c88eb494e95e3",
      "statusAfter": ""
    },
    {
      "finishedAt": "2026-10-18T12:20:13.017503+00:00",
      "name": "quint-deterministic-trace",
      "exitCode": 0,
      "wallSeconds": 0.8572865780006396,
      "stdoutSha256": "de3a45af85b1591c1c0b54115151377de7dd4370c7f3e5de83f7b13fe6d3bda6",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "commitAfter": "f15214ab7a885a4dc3194002cbacbb56841fdcbf",
      "treeAfter": "0d0a663592893e239ddea82bab5c88eb494e95e3",
      "statusAfter": ""
    }
  ],
  "allStepsCompleted": true,
  "allExitZero": true,
  "temporaryRootExistsAfter": false,
  "semanticCapturedStreamComparison": "PENDING; exit0 and literal assertions alone are not full stream equality."
}

```

## WHOLE FILE actual-check-a1/CAPTURED-COMPARISON.json

```
{
  "recordedAt": "2026-10-18T12:21:55.711516+00:00",
  "status": "MATCHED",
  "comparisons": {
    "rustEntireProjectionVsIndependentOracle": null,
    "quintCapturedFiveObservationsVsIndependentOracle": null,
    "quintCapturedFiveObservationsVsActualRust": null
  },
  "independentFreezeSha256": "7dc1b5c503a7cd2ab8e298fa638d42d7afd365df5e6253ce280e99af2178e7c3",
  "independentExpectedSha256": "519d2972fc58d74c824a8f514b8a0176c3849b9848bbc0b99729b4bd9e92c8e8",
  "rustStdoutSha256": "9584a4747896184cf0e996008756892f34b276d6c9fca27d9a8b10346191da82",
  "rustProjectionSha256": "519d2972fc58d74c824a8f514b8a0176c3849b9848bbc0b99729b4bd9e92c8e8",
  "quintRawCaptureSha256": "77c8d1896b29408d1b4da48edb5b99bae202989686291d9e466c9923d5fd748d",
  "quintDecodedSha256": "6b4cfb1503f0d08a5cecade74e3c57fe522c4778571176b148e5e9c2110862de",
  "decoderSha256": "5b4d046c735b2d1b6b387127f38e9e68a7b72b330e034f31ccc47411af1525e0",
  "capturedModelObservations": 5,
  "committedLogicalWork": [
    11,
    11,
    26,
    26,
    36
  ],
  "cumulativeAttemptedLogicalWork": [
    11,
    15,
    30,
    32,
    42
  ],
  "residualDutyFinal": 5,
  "limitations": [
    "One existing finite normal reserve trajectory; adapter domain0..142 and declared effect/record/map shapes.",
    "Quint initial/prefix FinancialFrames are pure assertions, not raw emitted trace variables; Rust frames separately equal independent expected.",
    "No exhaustive invariant proof, K/Lean/circuit/native/proof/ledger/target correspondence acceptance.",
    "Independent custodian related prior exposure is disclosed; no global blindness claim."
  ]
}

```

## WHOLE FILE actual-check-a1/ROOT-COMPARISON-A1-FAILURE.json

```
{
  "status": "ROOT_EXPORT_EXTRACTION_FAILED_AFTER_DECODER_MATCH",
  "exitCode": 1,
  "failure": "AssertionError selecting stdout lines starting with marker",
  "actualSourceFact": "Rust harness prefixes test name on same line before RESERVE_QUINT_PROJECTION marker",
  "noProductFailure": true,
  "decoderActualResult": "exit0 matched five captured states, complete raw receipt retained",
  "correction": "Reuse existing decoder actual outputs; parse exactly one marker with exact Rust harness prefix. No Core/Quint/decoder rerun."
}

```

## WHOLE FILE actual-check-a1/DECODER-CUSTODY.json

```
{
  "source": "/home/charl/moriarty-c2-opus55-restored-20261017/reserve-itf-decoder-source-20261018/decode_reserve_itf.py",
  "sha256": "5b4d046c735b2d1b6b387127f38e9e68a7b72b330e034f31ccc47411af1525e0",
  "rootReadWholeSource": true,
  "sourceGrounding": "Pinned Quint0.32.0 ITF serializer + original lifecycle/prefix type declarations. Author wrote decoder before actualtrace exposure, reported no subsequentcodechange.",
  "runtime": "Python stdlib only, file parsing/comparison, no model/native/SDK execution."
}

```

## WHOLE FILE actual-check-a1/DECODER-COMMAND.json

```
{
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "10s",
    "/usr/bin/python3",
    "/home/charl/moriarty-c2-opus55-restored-20261017/core2-quint-shared-projection-check-cycle/actual-check-a1/decode_reserve_itf.py",
    "--input",
    "/home/charl/moriarty-c2-opus55-restored-20261017/core2-quint-shared-projection-check-cycle/actual-check-a1/reserve_reserveSharedProjectionTraceTest_0.itf.json",
    "--variable",
    "observation",
    "--expected",
    "/home/charl/moriarty-c2-opus55-restored-20261017/reserve-quint-independent-oracle-20261018/reserve-common-projection.expected.json",
    "--output",
    "/home/charl/moriarty-c2-opus55-restored-20261017/core2-quint-shared-projection-check-cycle/actual-check-a1/quint-decoded-observations.json"
  ],
  "environment": {
    "PATH": "/usr/bin:/bin",
    "LANG": "C.UTF-8",
    "PYTHONDONTWRITEBYTECODE": "1"
  },
  "cwd": "/home/charl/moriarty-c2-opus55-restored-20261017/core2-quint-shared-projection-check-cycle/actual-check-a1"
}

```

## WHOLE FILE actual-check-a1/DECODER-STDOUT.json

```
{"expected_sha256": "519d2972fc58d74c824a8f514b8a0176c3849b9848bbc0b99729b4bd9e92c8e8", "first_difference": null, "input_sha256": "77c8d1896b29408d1b4da48edb5b99bae202989686291d9e466c9923d5fd748d", "output_sha256": "6b4cfb1503f0d08a5cecade74e3c57fe522c4778571176b148e5e9c2110862de", "scope": "five captured lifecycle Observation records only", "state_count": 5, "status": "matched", "variable": "observation"}

```

## WHOLE FILE actual-check-a1/DECODER-STDERR.log

```

```

## WHOLE FILE actual-check-a1/rust-shared-projection/STDOUT.log

```

running 1 test
test reserve_existing_trace_matches_complete_shared_quint_projection ... RESERVE_QUINT_PROJECTION {"initial":{"balances":{"observer":42,"reserve":100,"target":0,"treasury":0},"captureValues":[],"dutyAmount":5,"dutyPresent":true,"effects":[],"inputValue":2,"logicalAttempted":0,"logicalCommitted":0,"pendingSite":0,"programState":{"allocated":0,"basis":40,"marker":77},"specimen":"Reserve"},"observations":[{"attemptedTotal":11,"c":{"balances":{"observer":42,"reserve":100,"target":0,"treasury":0},"captures":[2,42],"committed":11,"dutyAmount":47,"dutyPresent":true,"programState":{"allocated":0,"basis":35,"marker":77},"site":1,"specimen":"Reserve"},"last":{"advanced":false,"attempted":0,"effects":[],"errorSteps":[],"hasSegment":false,"newContinuation":false,"sourceSteps":[],"status":"NotInvoked"}},{"attemptedTotal":15,"c":{"balances":{"observer":42,"reserve":100,"target":0,"treasury":0},"captures":[2,42],"committed":11,"dutyAmount":47,"dutyPresent":true,"programState":{"allocated":0,"basis":35,"marker":77},"site":1,"specimen":"Reserve"},"last":{"advanced":false,"attempted":4,"effects":[],"errorSteps":[],"hasSegment":false,"newContinuation":false,"sourceSteps":[],"status":"Waiting"}},{"attemptedTotal":30,"c":{"balances":{"observer":42,"reserve":58,"target":42,"treasury":0},"captures":[2,42,43],"committed":26,"dutyAmount":5,"dutyPresent":true,"programState":{"allocated":42,"basis":78,"marker":77},"site":2,"specimen":"Reserve"},"last":{"advanced":true,"attempted":15,"effects":[{"after":42,"before":0,"name":"allocated","tag":"StateValue"},{"amount":42,"tag":"TransferAmount"},{"after":5,"before":47,"tag":"DutyValue"},{"after":78,"before":35,"name":"basis","tag":"StateValue"}],"errorSteps":[],"hasSegment":true,"newContinuation":true,"sourceSteps":[3,4,5,6,7],"status":"Waiting"}},{"attemptedTotal":32,"c":{"balances":{"observer":42,"reserve":58,"target":42,"treasury":0},"captures":[2,42,43],"committed":26,"dutyAmount":5,"dutyPresent":true,"programState":{"allocated":42,"basis":78,"marker":77},"site":2,"specimen":"Reserve"},"last":{"advanced":false,"attempted":2,"effects":[],"errorSteps":[],"hasSegment":false,"newContinuation":false,"sourceSteps":[],"status":"Waiting"}},{"attemptedTotal":42,"c":{"balances":{"observer":42,"reserve":56,"target":42,"treasury":2},"captures":[],"committed":36,"dutyAmount":5,"dutyPresent":true,"programState":{"allocated":42,"basis":78,"marker":77},"site":0,"specimen":"Reserve"},"last":{"advanced":true,"attempted":10,"effects":[{"amount":2,"tag":"FeeAmount"},{"quote":43,"tag":"EmitQuote"}],"errorSteps":[],"hasSegment":true,"newContinuation":false,"sourceSteps":[8,9,10,11],"status":"Complete"}}],"prefix":{"balances":{"observer":42,"reserve":100,"target":0,"treasury":0},"captureValues":[2,42],"dutyAmount":47,"dutyPresent":true,"effects":[{"after":35,"before":40,"tag":"StateChange"},{"after":47,"before":5,"tag":"LiabilityChange"}],"inputValue":2,"logicalAttempted":11,"logicalCommitted":11,"pendingSite":1,"programState":{"allocated":0,"basis":35,"marker":77},"specimen":"Reserve"}}
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```

## WHOLE FILE actual-check-a1/rust-shared-projection/STDERR.log

```
   Compiling cfg-if v1.0.5
   Compiling libc v0.2.190
   Compiling typenum v1.20.1
   Compiling zeroize v1.9.1
   Compiling version_check v0.9.5
   Compiling generic-array v0.14.9
   Compiling const-oid v0.9.6
   Compiling subtle v2.6.1
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling getrandom v0.2.17
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling der v0.7.10
   Compiling getrandom v0.3.4
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling syn v3.0.6
   Compiling hybrid-array v0.4.15
   Compiling serde_core v1.0.229
   Compiling base16ct v0.2.0
   Compiling once_cell v1.21.4
   Compiling rustix v1.1.5
   Compiling getrandom v0.4.3
   Compiling unicode-width v0.1.14
   Compiling zerocopy v0.8.61
   Compiling miette v7.6.0
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling rand_core v0.9.5
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling syn v2.0.119
   Compiling zmij v1.0.23
   Compiling linux-raw-sys v0.12.1
   Compiling bitflags v2.13.2
   Compiling autocfg v1.5.1
   Compiling num-traits v0.2.19
   Compiling soa-rs-derive v1.0.1
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-span)
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling signature v2.2.0
   Compiling serde v1.0.229
   Compiling const-oid v0.10.2
   Compiling serde_json v1.0.151
   Compiling thiserror v2.0.21
   Compiling fastrand v2.5.0
   Compiling cpufeatures v0.2.17
   Compiling sha2 v0.10.9
   Compiling tempfile v3.27.0
   Compiling digest v0.11.3
   Compiling ecdsa v0.16.9
   Compiling ppv-lite86 v0.2.21
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling wait-timeout v0.2.1
   Compiling fnv v1.0.7
   Compiling cpufeatures v0.3.1
   Compiling itoa v1.0.18
   Compiling memchr v2.8.3
   Compiling bit-vec v0.8.0
   Compiling quick-error v1.2.3
   Compiling rusty-fork v0.3.1
   Compiling bit-set v0.8.0
   Compiling sha2 v0.11.0
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-lexer)
   Compiling rand_chacha v0.9.0
   Compiling k256 v0.13.4
   Compiling rand_xorshift v0.4.0
   Compiling rand v0.9.5
   Compiling unarray v0.1.4
   Compiling regex-syntax v0.8.11
   Compiling proptest v1.11.0
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 28.50s
     Running tests/reserve_quint_projection.rs (/tmp/doctorwatson-reserve-quint-sy0qlqvu/target/debug/deps/reserve_quint_projection-8da97f048f577763)

```

## WHOLE FILE actual-check-a1/rust-continuation-helper/STDOUT.log

```

running 5 tests
test consecutive_awaits_capture_earlier_events_without_reexecuting_prefix ... ok
test escrow_prefix_false_observation_and_true_remainder_match_literal_oracle ... ok
test normal_business_guard_rejects_only_the_final_attempted_segment ... ok
test ordinary_zero_await_action_completes_with_empty_frames ... ok
test reserve_two_awaits_preserve_captured_held_and_residual_five ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


```

## WHOLE FILE actual-check-a1/rust-continuation-helper/STDERR.log

```
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running tests/continuations.rs (/tmp/doctorwatson-reserve-quint-sy0qlqvu/target/debug/deps/continuations-a63c2fb5271d9807)

```

## WHOLE FILE actual-check-a1/quint-typecheck/STDOUT.log

```

```

## WHOLE FILE actual-check-a1/quint-typecheck/STDERR.log

```

```

## WHOLE FILE actual-check-a1/quint-deterministic-trace/STDOUT.log

```

  reserve_shared_projection_test
    ok reserveSharedProjectionTraceTest passed 1 test(s)

  1 passing (31ms)

```

## WHOLE FILE actual-check-a1/quint-deterministic-trace/STDERR.log

```

```

## WHOLE FILE EXACT-SOURCE-GUARD.json

```
{
  "allowedBaseExactly": "e4d7a2b5709caa18a94f5256d03a0302364b8235",
  "candidateCommitExactly": "f15214ab7a885a4dc3194002cbacbb56841fdcbf",
  "candidateTreeExactly": "0d0a663592893e239ddea82bab5c88eb494e95e3",
  "candidateChangedPathsExactly": [
    "crates/mori-core/tests/continuations.rs",
    "crates/mori-core/tests/fixtures/continuations/reserve-quint-projection.json",
    "crates/mori-core/tests/reserve_quint_projection.rs",
    "models/core-stage-correspondence/reserve_shared_projection_test.qnt"
  ],
  "changedPathCount": 4,
  "rootAloneIntegrates": true,
  "sourceMaySwitchCheckoutBranch": "All physical sources extracted by full immutable commit; mutable HEAD is not a source pin.",
  "resultAcceptance": "Prepared source/result evidence only; final decoder/comparison and fresh review remain pending."
}

```
