//! Direct consumer of the complete actual selected simulate handler.
//! SOURCE_ONLY / UNRUN. R324 expectations froze before these assertions.
//! The verification mode reads persisted JSON only; it never repeats a run.
#[path = "../../src/cli_args.rs"]
mod cli_args;
#[path = "../../src/display.rs"]
mod display;
#[path = "../../src/commands/source_io.rs"]
mod source_io;
#[path = "selected_simulate_adapter.rs"]
mod selected_simulate;

use clap::{Args as ClapArgs, FromArgMatches};
use serde_json::{Value, json};
use std::{collections::BTreeSet, ffi::OsString, path::Path, process::ExitCode};
type Result<T> = std::result::Result<T, mori_diagnostics::Report>;
const EXPECTED: &str = include_str!("../fixtures/real-simulate-handler/EXPECTED-OUTPUT-CONTRACT.json");

fn number(value: &Value) -> u128 {
    let text = value.as_str().expect("counter must be a string");
    assert!(text == "0" || (!text.is_empty() && !text.starts_with('0')
        && text.bytes().all(|b| b.is_ascii_digit())), "noncanonical counter {text:?}");
    text.parse().expect("counter exceeds u128")
}
fn index(value: &Value) -> usize {
    usize::try_from(number(value)).expect("reference index exceeds usize")
}
fn object_keys(value: &Value) -> BTreeSet<&str> {
    value.as_object().expect("required JSON object").keys().map(String::as_str).collect()
}
fn at<'a>(value: &'a Value, pointer: &str) -> &'a Value {
    if pointer == "/" { value } else { value.pointer(pointer).expect(pointer) }
}

fn verify_report(path: &Path, case: &str, expected: &Value) -> Result<()> {
    // Genuine production bounded source reader, then the genuine bounded strict
    // JSON parser. It rejects duplicate keys, extra JSON and clipped output.
    let text = source_io::read_source(path)?;
    let actual = mori_core::json::parse(&text).map_err(source_io::operation)?;
    assert!(actual.is_object());
    // These are literal fixture comparisons, not a predicate language or evaluator.
    for (pointer, keys) in expected["exactObjectKeySets"].as_object().unwrap() {
        let required: BTreeSet<_> = keys.as_array().unwrap().iter()
            .map(|key| key.as_str().unwrap()).collect();
        assert_eq!(object_keys(at(&actual, pointer)), required, "{case}: keys {pointer}");
    }
    for (pointer, value) in expected["commonExactJsonPointerValues"].as_object().unwrap() {
        assert_eq!(at(&actual, pointer), value, "{case}: {pointer}");
    }
    for (pointer, value) in expected["caseExactJsonPointerValues"][case].as_object().unwrap() {
        assert_eq!(at(&actual, pointer), value, "{case}: {pointer}");
    }
    let preparation = &actual["preparation"];
    let execution = &actual["execution"];
    let image = &execution["image"];
    let summary = &execution["summary"];
    let resource = &execution["resource_report"];
    let observed = &resource["observed"];
    let resource_summary = &resource["run_summary"];
    assert!(actual.get("events").is_none() && execution.get("events").is_none());
    assert!(resource["profile"]["profile_id"].as_u64().is_some_and(|n| n <= 255));
    for plan in ["witness", "constraint", "export"] {
        assert!(preparation["plans"][plan]["reason"].as_str().is_some_and(|s| !s.is_empty()));
    }
    for group in resource["unmeasured"].as_object().unwrap().values() {
        assert_eq!(group["status"], "unmeasured");
        assert!(group["reason"].as_str().is_some_and(|s| !s.is_empty()));
        assert!(group["metrics"].is_array());
    }
    for field in ["events", "consumed_work", "helper_work", "helper_visits", "reference_cursor"] {
        number(&summary[field]);
        number(&resource_summary[field]);
    }
    for field in ["initial_fuel", "remaining_work", "consumed_work", "helper_work", "helper_visits", "reference_cursor"] {
        number(&image[field]);
        number(&observed[field]);
    }
    let forwarded = number(&execution["events_forwarded"]);
    assert!(forwarded > 0);
    assert_eq!(forwarded, number(&summary["events"]));
    assert_eq!(execution["events_forwarded"], resource_summary["events"]);

    // R324 independent final-value predicate: resolve the actual reference,
    // never assume an allocation position or execute a second computation.
    let outcome = image["outcome"].as_object().expect("tagged outcome");
    assert_eq!(outcome.len(), 1);
    let halted = outcome.get("Halted").expect("expected actual Halted outcome");
    assert_eq!(object_keys(halted), BTreeSet::from(["value"]));
    let reference = halted["value"].as_object().expect("tagged reference");
    assert_eq!(reference.len(), 1);
    let (kind, reference_value) = reference.iter().next().unwrap();
    let reference_index = index(reference_value);
    let table = match kind.as_str() {
        "Const" => preparation["admission"]["program_view"]["constants"].as_array().unwrap(),
        "Data" => image["data"].as_array().unwrap(),
        other => panic!("unsupported halted reference {other}"),
    };
    let cell = table.get(reference_index).expect("halted reference out of bounds");
    assert_eq!(object_keys(cell), BTreeSet::from(["U128"]));
    assert_eq!(object_keys(&cell["U128"]), BTreeSet::from(["n"]));
    assert_eq!(cell["U128"]["n"], expected["cases"][case]["independentReturnedUInt"]);
    number(&cell["U128"]["n"]);
    assert_eq!(image["control"], json!({"Return":{"value":halted["value"]}}));
    assert_eq!(summary["outcome"], image["outcome"]);
    assert_ne!(image["outcome"], preparation["genesis"]["outcome"]);
    let resource_outcome = json!({"status":"halted","value":{
        "kind":if kind == "Const" { "const" } else { "data" }, "index":reference_index}});
    assert_eq!(observed["outcome"], resource_outcome);
    assert_eq!(resource_summary["outcome"], resource_outcome);

    for field in ["consumed_work", "helper_work", "helper_visits", "reference_cursor"] {
        assert_eq!(summary[field], image[field], "{case}: image {field}");
        assert_eq!(summary[field], observed[field], "{case}: observed {field}");
        assert_eq!(summary[field], resource_summary[field], "{case}: summary {field}");
    }
    let used = number(&image["consumed_work"]);
    let remaining = number(&image["remaining_work"]);
    assert!(used > 0);
    assert_eq!(used.checked_add(remaining), Some(4096));
    assert_eq!(number(&image["initial_fuel"]), 4096);
    assert!(number(&image["helper_work"]) <= used);
    assert_eq!(observed["initial_fuel"], image["initial_fuel"]);
    assert_eq!(observed["remaining_work"], image["remaining_work"]);

    // Resource wire conventions deliberately differ from owner decimal strings:
    // usize extents are JSON numbers; work counters remain decimal strings.
    let config = &preparation["config"];
    for destination in ["admission_bounds", "layout_bounds"] {
        let supplied = &resource["supplied"][destination];
        assert_eq!(object_keys(supplied), object_keys(&config["bounds"]));
        for (name, value) in config["bounds"].as_object().unwrap() {
            if ["max_reference_work", "max_helper_work", "step_cap"].contains(&name.as_str()) {
                assert_eq!(&supplied[name], value);
                number(&supplied[name]);
            } else {
                let expected_extent = u64::try_from(number(value)).expect("extent outside u64");
                assert_eq!(supplied[name].as_u64(), Some(expected_extent));
            }
        }
    }
    let capacities = &config["layout"]["capacities"];
    let supplied_capacities = &resource["supplied"]["layout_capacities"];
    assert_eq!(object_keys(supplied_capacities), object_keys(capacities));
    for (name, value) in capacities.as_object().unwrap() {
        assert_eq!(supplied_capacities[name].as_u64(),
            Some(u64::try_from(number(value)).expect("capacity outside u64")));
    }
    assert_eq!(resource["supplied"]["boundary_capacities_present"], config["layout"]["boundary_capacities_present"]);
    Ok(())
}

fn verify_files(paths: &[OsString]) -> Result<()> {
    assert_eq!(paths.len(), 2, "exactly Chain and Branch persisted stdout files required");
    let expected: Value = serde_json::from_str(EXPECTED).map_err(source_io::operation)?;
    for (case, path) in ["chain_total", "branch_total"].into_iter().zip(paths) {
        verify_report(Path::new(path), case, &expected)?;
    }
    println!("verified complete persisted ChainTotal/BranchTotal JSON against frozen R324 predicates; no trace or exact B-clock claim");
    Ok(())
}

fn main() -> ExitCode {
    let argv: Vec<OsString> = std::env::args_os().collect();
    // This explicit mode is harness-only, not a new public mori subcommand.
    if argv.get(1).is_some_and(|arg| arg == "--verify-report-files") {
        return match verify_files(&argv[2..]) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => { eprintln!("{error}"); ExitCode::FAILURE }
        };
    }
    let command = cli_args::simulate::Args::augment_args(clap::Command::new("simulate"));
    let matches = match command.try_get_matches_from(argv) {
        Ok(matches) => matches,
        Err(error) => error.exit(),
    };
    let args = cli_args::simulate::Args::from_arg_matches(&matches).expect("actual production Args conversion");
    match selected_simulate::run(&args) {
        Ok(exit) => exit,
        Err(error) => { eprintln!("{error}"); ExitCode::FAILURE }
    }
}
