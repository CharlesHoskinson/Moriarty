//! InvoiceReceipt continues through a successful receipt using its real boundary.
use mori_core::stage::{self, StageOutcome};
use mori_core::{Effect, Execution, Scenario, UInt, json};
use serde::Serialize;
use serde_json::{Value, json as object};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/invoice-receipt-resume/",
            $name
        ))
    };
}

const SOURCE: &str = fixture!("inputs/program.mori");

fn expected(text: &str) -> Value {
    serde_json::from_str(text).expect("valid independent oracle JSON")
}

// Save genuine results before semantic assertions when a caller supplies an
// existing output directory. Expected fixtures are never written or refreshed.
fn capture(name: &str, value: &impl Serialize) {
    if let Some(directory) = std::env::var_os("MORI_INVOICE_RECEIPT_CAPTURE_DIR") {
        let path = std::path::PathBuf::from(directory).join(format!("{name}.json"));
        std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    }
}

// This projection copies observed records; it does not evaluate source rules.
// A cumulative prefix is explicitly derived from observed per-step deltas.
fn prefixes(
    execution: &Execution,
    segment: &str,
    cumulative_key: &str,
    accumulated: &mut Vec<Effect>,
) -> Vec<Value> {
    execution
        .steps
        .iter()
        .map(|step| {
            accumulated.extend(step.effects.iter().cloned());
            let statement = serde_json::to_value(&step.statement).unwrap();
            let mut observation = object!({
                "sourceOrdinal": step.index,
                "segment": segment,
                "statementKind": statement["kind"],
                "typedValue": step.value,
                "snapshot": step.snapshot,
                "deltaEffects": step.effects
            });
            observation.as_object_mut().unwrap().insert(
                cumulative_key.into(),
                serde_json::to_value(&*accumulated).unwrap(),
            );
            observation
        })
        .collect()
}

#[test]
fn reopened_invoice_receipt_completes_with_all_financial_prefixes_preserved() {
    let program = stage::load(SOURCE).expect("ordinary InvoiceReceipt source");
    let scenario: Scenario = serde_json::from_str(fixture!("inputs/scenario.json")).unwrap();
    let event = stage::decode::event(fixture!("inputs/receipt-true.event.json")).unwrap();
    let started = stage::start(&program, "pay", scenario, Some(UInt(23)));
    capture("start-result", &started);
    let StageOutcome::Waiting {
        advanced,
        segment: Some(start_segment),
        continuation,
        attempted: start_attempted,
        residual: start_residual,
    } = started.expect("normal start reaches the receipt boundary")
    else {
        panic!("InvoiceReceipt must wait with an actual prefix segment");
    };

    let encoded = json::export(&continuation).unwrap();
    let reopened = stage::decode::continuation(&encoded).unwrap();
    capture("reopened-continuation", &reopened);
    let completed = stage::resume(&program, reopened.clone(), event.clone(), Some("pay"), None);
    capture("resume-result", &completed);
    let StageOutcome::Complete {
        segment,
        committed,
        attempted,
        residual,
    } = completed.expect("the supplied receipt satisfies the ordinary condition")
    else {
        panic!("accepted receipt must complete the existing pending action");
    };

    let resume_oracle = expected(fixture!("EXPECTED-RESUME.json"));
    let all_oracle = expected(fixture!("EXPECTED-ALL-PREFIXES.json"));
    let work_oracle = expected(fixture!("EXPECTED-LOGICAL-WORK.json"));
    let mut resume_effects = Vec::new();
    let resume_steps = prefixes(
        &segment.execution,
        "resume",
        "derivedSegmentEffectPrefix",
        &mut resume_effects,
    );
    let actual_resume = object!({
        "action": segment.action,
        "semantics": segment.execution.semantics,
        "status": "complete",
        "pre": segment.execution.pre,
        "steps": resume_steps,
        "post": segment.execution.post,
        "effects": segment.execution.effects,
        "frame_pre": segment.frame_pre,
        "frame_post": segment.frame_post,
        "residual": residual,
        "attempted": attempted,
        "committed": committed,
        "selectedEvent": event
    });
    let resume_keys = [
        "action",
        "semantics",
        "status",
        "pre",
        "steps",
        "post",
        "effects",
        "frame_pre",
        "frame_post",
        "residual",
        "attempted",
        "committed",
        "selectedEvent",
    ];
    let expected_resume: serde_json::Map<String, Value> = resume_keys
        .into_iter()
        .map(|key| (key.to_owned(), resume_oracle[key].clone()))
        .collect();

    let mut across = Vec::new();
    let mut all_steps = prefixes(
        &start_segment.execution,
        "start",
        "derivedCrossSegmentEffectPrefix",
        &mut across,
    );
    all_steps.extend(prefixes(
        &segment.execution,
        "resume",
        "derivedCrossSegmentEffectPrefix",
        &mut across,
    ));
    let combined_effects: Vec<_> = start_segment
        .execution
        .effects
        .iter()
        .chain(&segment.execution.effects)
        .cloned()
        .collect();
    let actual_journey = object!({
        "pre": start_segment.execution.pre,
        "steps": all_steps,
        "post": segment.execution.post,
        "orderedCrossSegmentEffects": combined_effects
    });
    let expected_journey = object!({
        "pre": all_oracle["pre"],
        "steps": all_oracle["steps"],
        "post": all_oracle["post"],
        "orderedCrossSegmentEffects": all_oracle["orderedCrossSegmentEffects"]
    });
    capture("resume-observations", &actual_resume);
    capture("cross-segment-observations", &actual_journey);

    assert!(advanced, "source:\n{SOURCE}");
    assert_eq!(reopened, continuation, "source:\n{SOURCE}");
    assert_eq!(
        serde_json::to_value(&continuation.boundary).unwrap(),
        resume_oracle["pre"],
        "source:\n{SOURCE}"
    );
    assert_eq!(
        serde_json::to_value(&start_segment.execution.post).unwrap(),
        resume_oracle["pre"],
        "source:\n{SOURCE}"
    );
    assert_eq!(
        serde_json::to_value(continuation.logical.cap).unwrap(),
        work_oracle["startCap"]
    );
    assert_eq!(
        serde_json::to_value(continuation.logical.committed).unwrap(),
        work_oracle["startCommitted"]
    );
    assert_eq!(
        serde_json::to_value(start_attempted).unwrap(),
        work_oracle["startAttempted"]
    );
    assert_eq!(
        serde_json::to_value(start_residual).unwrap(),
        resume_oracle["residual"]
    );
    assert_eq!(
        actual_resume,
        Value::Object(expected_resume),
        "source:\n{SOURCE}"
    );
    assert_eq!(actual_journey, expected_journey, "source:\n{SOURCE}");
}
