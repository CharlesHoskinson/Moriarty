//! Ordinary positive Boolean expression coverage through both Core profiles.
use mori_core::{Scenario, Value, parse, simulate, stage};

const CORE1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/positive-grammar-edges/core1.mori"
));
const CORE2: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/positive-grammar-edges/core2.mori"
));

#[test]
fn core1_positive_less_inequality_and_disjunction() {
    let program = parse(CORE1).expect("positive Core/1 source");
    let result = simulate(&program, "run", &Scenario::default()).expect("normal result");
    assert_eq!(result.semantics, "moriarty-core/1");
    assert_eq!(
        result.post.state,
        [("result".into(), Value::Bool(true))].into()
    );
}

#[test]
fn core2_positive_boolean_values_survive_await_and_ensure() {
    let program = stage::load(CORE2).expect("positive Core/2 source");
    let outcome = stage::start(&program, "run", Scenario::default(), None)
        .expect("normal waiting result");
    let stage::StageOutcome::Waiting { continuation, .. } = outcome else {
        panic!("expected waiting at proceed");
    };
    assert_eq!(
        continuation.boundary.state,
        [("result".into(), Value::Bool(false))].into()
    );
    let event = stage::StageEvent {
        format: stage::STAGE_EVENT_FORMAT.into(),
        action: "run".into(),
        label: "proceed".into(),
        values: vec![stage::NamedValue {
            name: "ok".into(),
            value: Value::Bool(true),
        }],
    };
    let outcome = stage::resume(&program, continuation, event, Some("run"), None)
        .expect("normal completion");
    let stage::StageOutcome::Complete { segment, .. } = outcome else {
        panic!("expected completion");
    };
    assert_eq!(segment.execution.semantics, "moriarty-core/2");
    assert_eq!(
        segment.execution.post.state,
        [("result".into(), Value::Bool(true))].into()
    );
}
