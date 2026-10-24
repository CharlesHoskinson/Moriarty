//! Finite event-vector observation through existing owning Core2 APIs.
//! Inputs and raw per-call publications are retained; no expected input exists.
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
    // Root supplies SOURCE SCENARIO SELECTION EVENT... under ordinary caps.
    // Production decoders retain their own policies. All files are read once.
    let args: Vec<String> = std::env::args().collect();
    let source = fs::read_to_string(&args[1])?;
    let scenario_text = fs::read_to_string(&args[2])?;
    let selection_text = fs::read_to_string(&args[3])?;
    let event_texts: Vec<String> = args[4..].iter()
        .map(fs::read_to_string).collect::<Result<_, _>>()?;
    let scenario: Scenario = serde_json::from_value(mori_core::json::parse(&scenario_text)?)?;
    let selection: Selection = serde_json::from_value(mori_core::json::parse(&selection_text)?)?;
    let events: Vec<stage::StageEvent> = event_texts.iter()
        .map(|text| stage::decode::event(text)).collect::<Result<_, _>>()?;
    let program = stage::load(&source)?;

    let mut current = start_observed(&program, selection.action.clone(), scenario,
        selection.start_cap);
    let mut publications = vec![current.publication()?];
    let mut consumed_event_count = 0_usize;
    for event in events {
        // Only the immediately preceding real call can supply continuation.
        let continuation = match current.raw_result() {
            Ok(stage::StageOutcome::Waiting { continuation, .. }) => continuation.clone(),
            _ => break,
        };
        current = resume_observed(&program, continuation, event,
            selection.resume_cli_action.clone(), selection.resume_cli_cap);
        publications.push(current.publication()?);
        consumed_event_count += 1;
    }
    // An early Complete/Rejected stays published. All requested input files,
    // including any unused events, remain recorded; no replacement is invented.
    let event_inputs: Vec<Value> = args[4..].iter().zip(&event_texts)
        .map(|(path, text)| input_record(path, text)).collect();
    let capture = json!({
        "format": "sherlock-core2-owning-event-sequence-capture/1",
        "caseId": selection.case_id,
        "sourceInput": input_record(&args[1], &source),
        "scenarioInput": input_record(&args[2], &scenario_text),
        "selectionInput": input_record(&args[3], &selection_text),
        "eventInputs": event_inputs,
        "publications": publications,
        "consumedEventCount": consumed_event_count,
    });
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer(&mut output, &capture)?;
    output.write_all(b"\n")?;
    Ok(())
}
