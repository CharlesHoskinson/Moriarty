//! Two accepted fee-bearing fills retain complete state and residual duties.
use mori_core::{Scenario, stage};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/two-stage-partial-fill-example")
}

fn fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn pending(outcome: &stage::StageOutcome) -> stage::Continuation {
    match outcome {
        stage::StageOutcome::Waiting { continuation, .. } => continuation.clone(),
        stage::StageOutcome::Complete { .. } => panic!("expected a pending fill"),
    }
}

#[test]
fn partial_fills_keep_cumulative_fees_and_remaining_duties() {
    let source = fs::read_to_string(fixtures().join("two-stage-partial-fill.mori")).unwrap();
    let program = stage::load(&source).expect("source must be admitted");
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        fixture("program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        fixture("plan.expected.json")
    );
    let scenario: Scenario = serde_json::from_value(fixture("opening.scenario.json")).unwrap();
    let first = stage::start(&program, "fill", scenario, None).expect("first fill");
    let first_report = stage::outcome_report(Ok(first.clone()));
    assert_eq!(first_report, fixture("start.report.expected.json"));
    let first_pending = pending(&first);

    let false_event: stage::StageEvent =
        serde_json::from_value(fixture("finish-false.event.json")).unwrap();
    let observed = stage::resume(&program, first_pending.clone(), false_event, None, None)
        .expect("ordinary false observation");
    let observed_report = stage::outcome_report(Ok(observed.clone()));
    assert_eq!(observed_report, fixture("finish-false.report.expected.json"));
    let actual_retained = pending(&observed);
    assert_eq!(actual_retained, first_pending);

    let true_event: stage::StageEvent =
        serde_json::from_value(fixture("finish-true.event.json")).unwrap();
    let final_fill = stage::resume(&program, actual_retained, true_event, None, None)
        .expect("second fill");
    assert!(matches!(&final_fill, stage::StageOutcome::Complete { .. }));
    let final_report = stage::outcome_report(Ok(final_fill));
    assert_eq!(final_report, fixture("finish-true.report.expected.json"));
    assert_eq!(
        first_report["continuation"]["boundary"],
        final_report["segment"]["execution"]["pre"]
    );

    let mut history = Vec::new();
    for report in [&first_report, &final_report] {
        history.extend(
            report["segment"]["execution"]["effects"]
                .as_array()
                .unwrap()
                .iter()
                .cloned(),
        );
    }
    assert_eq!(Value::Array(history), fixture("ordered-history.expected.json"));
}
