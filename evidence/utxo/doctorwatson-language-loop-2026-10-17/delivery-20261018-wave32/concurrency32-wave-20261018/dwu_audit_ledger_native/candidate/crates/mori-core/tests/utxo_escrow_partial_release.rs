use mori_core::{Scenario, parse, simulate};
use serde_json::Value;
use std::{env, fs::OpenOptions, io::Write};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/utxo-escrow-partial-release/",
            $name
        ))
    };
}

#[test]
fn partial_escrow_release_preserves_residual_duty_and_observers() {
    let program = parse(fixture!("program.mori")).expect("partial-release source must parse");
    let scenario: Scenario = serde_json::from_str(fixture!("normal.scenario.json"))
        .expect("complete partial-release Scenario must decode");
    let original = scenario.clone();
    let execution = simulate(&program, "release", &scenario)
        .expect("normal partial release must succeed locally");
    let actual = serde_json::to_value(&execution).expect("Execution must serialize");

    if let Some(path) = env::var_os("MORI_UTXO_PATTERN_EXECUTION_OUTPUT") {
        let mut bytes = serde_json::to_vec_pretty(&actual).expect("Execution must encode");
        bytes.push(b'\n');
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("fresh root-selected output must open");
        file.write_all(&bytes).expect("complete Execution must write");
        file.sync_all().expect("Execution output must persist");
    }

    let expected: Value =
        serde_json::from_str(fixture!("independent/COMPLETE-PREFIXES.expected.json"))
            .expect("frozen independent semantic expectation must decode");
    assert_eq!(fixture!("program.mori"), fixture!("independent/inputs/program.mori"));
    assert_eq!(
        fixture!("normal.scenario.json"),
        fixture!("independent/inputs/normal.scenario.json")
    );
    assert_eq!(
        program.language,
        expected["language"].as_str().expect("expected language")
    );
    assert_eq!(
        program.name,
        expected["agreement"].as_str().expect("expected agreement")
    );
    assert_eq!(actual["semantics"], expected["semantics"]);
    assert_eq!(actual["action"], expected["action"]);
    assert_eq!(actual["pre"], expected["pre"], "complete independent prestate");
    assert_eq!(actual["post"], expected["post"], "complete independent poststate");

    let steps = actual["steps"].as_array().expect("actual steps");
    let prefixes = expected["prefixes"].as_array().expect("independent prefixes");
    assert_eq!(steps.len(), 11);
    assert_eq!(prefixes.len(), 11);
    let mut cumulative = Vec::new();
    for (index, (step, prefix)) in steps.iter().zip(prefixes).enumerate() {
        assert_eq!(step["index"], prefix["index"], "prefix ordinal {index}");
        // Complete snapshots retain unrelated balances, duties and ordinary state.
        assert_eq!(step["snapshot"], prefix["snapshot"], "complete snapshot {index}");
        assert_eq!(step["effects"], prefix["stepEffects"], "ordered delta effects {index}");
        cumulative.extend(
            step["effects"]
                .as_array()
                .expect("actual step effects")
                .iter()
                .cloned(),
        );
        assert_eq!(
            Value::Array(cumulative.clone()), prefix["cumulativeEffects"],
            "ordered cumulative effects {index}"
        );
    }
    assert_eq!(actual["effects"].as_array().expect("actual effects").len(), 6);
    assert_eq!(actual["effects"], expected["effects"], "complete ordered effects");
    assert_eq!(Value::Array(cumulative), expected["effects"]);
    assert_eq!(scenario, original, "borrowed Scenario remains unchanged");
}
