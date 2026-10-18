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
    assert_eq!(serde_json::to_value(asset).unwrap(), json!({
        "network":"local", "issuer":"continuation-reserve-oracle",
        "reference":"UNIT", "scale":0
    }));
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
        assert!(balances.insert(&balance.account, number(balance.amount)).is_none());
    }
    let program_state: BTreeMap<_, _> = snapshot.state.iter()
        .map(|(name, value)| (name, uint(value))).collect();
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
    assert_eq!(captures.iter().map(|value| value.name.as_str()).collect::<Vec<_>>().as_slice(), names);
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
    effects.iter().map(|effect| match effect {
        Effect::State { name, before, after } => json!({
            "tag":"StateValue", "name":name, "before":uint(before), "after":uint(after)
        }),
        Effect::Transfer { asset, from, to, amount } => {
            reserve_asset(asset);
            assert_eq!((from.as_str(), to.as_str()), ("reserve", "target"));
            json!({"tag":"TransferAmount", "amount":number(*amount)})
        }
        Effect::Liability { name, asset, debtor, creditor, before, after } => {
            allocation(name, asset, debtor, creditor);
            json!({"tag":"DutyValue", "before":number(*before), "after":number(*after)})
        }
        Effect::Fee { asset, from, to, amount } => {
            reserve_asset(asset);
            assert_eq!((from.as_str(), to.as_str()), ("reserve", "treasury"));
            json!({"tag":"FeeAmount", "amount":number(*amount)})
        }
        Effect::Emit { name, value } => {
            assert_eq!(name, "settled_quote");
            json!({"tag":"EmitQuote", "quote":uint(value)})
        }
    }).collect()
}

fn financial_frame(snapshot: &Snapshot, input: UInt, site: u16, captures: &[NamedValue],
    attempted: UInt, committed: UInt, effects: Vec<Json>) -> Json {
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

fn invocation(status: &str, attempted: UInt, advanced: bool, segment: Option<&StageSegment>,
    new_continuation: bool) -> Json {
    let effects = segment.map_or_else(Vec::new, |segment| resume_effects(&segment.execution.effects));
    let steps: Vec<_> = segment.into_iter().flat_map(|segment| &segment.execution.steps)
        .map(|step| step.index).collect();
    json!({"status":status, "attempted":number(attempted), "advanced":advanced,
        "hasSegment":segment.is_some(), "newContinuation":new_continuation,
        "effects":effects, "sourceSteps":steps, "errorSteps":[]})
}

fn observe(outcome: StageOutcome, previous: &Continuation, total: &mut u64)
    -> (Json, Option<Continuation>) {
    let (c, last, attempted, pending) = match outcome {
        StageOutcome::Waiting { advanced, segment, continuation, attempted, residual } => {
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
            let c = control(&continuation.boundary, continuation.site,
                &continuation.captures, continuation.logical.committed);
            let last = invocation("Waiting", attempted, advanced, segment.as_ref(), advanced);
            (c, last, attempted, Some(continuation))
        }
        StageOutcome::Complete { segment, committed, attempted, residual } => {
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
    (json!({"c":c, "last":last, "attemptedTotal":*total}), pending)
}

fn event(label: &str, name: &str, value: Value) -> StageEvent {
    StageEvent { format:stage::STAGE_EVENT_FORMAT.into(), action:"allocate".into(),
        label:label.into(), values:vec![NamedValue { name:name.into(), value }] }
}

#[test]
fn reserve_existing_trace_matches_complete_shared_quint_projection() {
    let expected: Json = serde_json::from_str(EXPECTED).unwrap();
    let scenario: Scenario = serde_json::from_str(OPENING).unwrap();
    let input = match scenario.inputs.get("extra").unwrap() {
        Value::UInt(input) => *input,
        Value::Bool(_) => panic!("extra is UInt"),
    };
    let opening = Snapshot { state:scenario.state.clone(), balances:scenario.balances.clone(),
        liabilities:scenario.liabilities.clone() };
    let initial = financial_frame(&opening, input, 0, &[], UInt(0), UInt(0), vec![]);
    let program = stage::load(SOURCE).unwrap();
    let StageOutcome::Waiting { advanced, segment:Some(segment), continuation:first,
        attempted, residual } = stage::start(&program, "allocate", scenario, Some(UInt(36))).unwrap()
    else { panic!("reserve starts at its first await") };
    assert!(advanced);
    segment_entry(&segment, None);
    assert_eq!(segment.execution.pre, opening);
    assert_eq!(segment.frame_post.site, first.site);
    assert_eq!(segment.frame_post.captures, first.captures);
    assert_eq!(segment.execution.post, first.boundary);
    assert_eq!(residual, first.boundary.liabilities);
    assert_eq!(segment.execution.steps.iter().map(|step| step.index).collect::<Vec<_>>(), vec![0, 1, 2]);
    let prefix_effects: Vec<_> = segment.execution.effects.iter().map(|effect| match effect {
        Effect::State { name, before, after } => {
            assert_eq!(name, "basis");
            json!({"tag":"StateChange", "before":uint(before), "after":uint(after)})
        }
        Effect::Liability { name, asset, debtor, creditor, before, after } => {
            allocation(name, asset, debtor, creditor);
            json!({"tag":"LiabilityChange", "before":number(*before), "after":number(*after)})
        }
        _ => panic!("reserve prefix has only basis and duty effects"),
    }).collect();
    let prefix = financial_frame(&first.boundary, input, first.site, &first.captures,
        attempted, first.logical.committed, prefix_effects);
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
        let previous = pending.take().expect("the first three invocations leave a continuation");
        let actual = stage::resume(&program, previous.clone(), event, None, None).unwrap();
        let (observation, next) = observe(actual, &previous, &mut total);
        observations.push(observation);
        pending = next;
    }
    assert!(pending.is_none());
    let actual = json!({"initial":initial, "prefix":prefix, "observations":observations});
    assert_eq!(actual, expected);
    println!("RESERVE_QUINT_PROJECTION {}", serde_json::to_string(&actual).unwrap());
}
