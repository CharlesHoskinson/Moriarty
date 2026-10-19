//! Ordinary bounded-input observation consumer, not a new stage implementation.
//! Link the pinned existing normal Core rlib. Expected data is never an input.
use mori_core::sherlock_financial_projection::{resume_observed, start_observed};
use mori_core::{Scenario, UInt, stage};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{error::Error, fs, io::Write};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Selection {
    case_id: String,
    action: String,
    start_cap: Option<UInt>,
    resume_cli_action: Option<String>,
    resume_cli_cap: Option<UInt>,
}

fn input_record(path: &str, text: &str) -> Value {
    json!({"path": path, "utf8": text,
        "sha256": mori_core::intent::hash_bytes(text.as_bytes())})
}

fn main() -> Result<(), Box<dyn Error>> {
    // Root supplies exactly these four frozen input paths under operation caps.
    // Existing Core parsers/admission retain their own policies and diagnostics.
    let args: Vec<String> = std::env::args().collect();
    let source = fs::read_to_string(&args[1])?;
    let scenario_text = fs::read_to_string(&args[2])?;
    let event_text = fs::read_to_string(&args[3])?;
    let selection_text = fs::read_to_string(&args[4])?;
    let scenario: Scenario = serde_json::from_value(mori_core::json::parse(&scenario_text)?)?;
    let event = stage::decode::event(&event_text)?;
    let selection: Selection = serde_json::from_value(mori_core::json::parse(&selection_text)?)?;
    let program = stage::load(&source)?;

    let start = start_observed(&program, selection.action.clone(), scenario, selection.start_cap);
    let start_publication = start.publication()?;
    // This is sum elimination over the actual returned call record. No caller-
    // supplied or expected continuation can replace the returned continuation.
    let resume_publication = match start.raw_result() {
        Ok(stage::StageOutcome::Waiting { continuation, .. }) => {
            let resume = resume_observed(&program, continuation.clone(), event,
                selection.resume_cli_action, selection.resume_cli_cap);
            Some(resume.publication()?)
        }
        _ => None,
    };
    // A non-Waiting start remains fully published; no synthetic resume result,
    // application rejection, rollback continuation or model premise is invented.
    let capture = json!({
        "format": "sherlock-core2-owning-call-capture/1",
        "caseId": selection.case_id,
        "sourceInput": input_record(&args[1], &source),
        "scenarioInput": input_record(&args[2], &scenario_text),
        "eventInput": input_record(&args[3], &event_text),
        "selectionInput": input_record(&args[4], &selection_text),
        "startPublication": start_publication,
        "resumePublication": resume_publication,
    });
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer(&mut output, &capture)?;
    output.write_all(b"\n")?;
    Ok(())
}
