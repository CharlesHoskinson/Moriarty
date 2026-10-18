use mori_core::{Scenario, parse, simulate};
use serde_json::Value;
use std::{env, fs::OpenOptions, io::Write};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/utxo-pattern-exchange/",
            $name
        ))
    };
}

#[test]
fn prefunded_exchange_matches_complete_authored_normal_outcome() {
    let program = parse(fixture!("program.mori")).expect("Core/1 example must parse");
    let scenario: Scenario = serde_json::from_str(fixture!("normal.scenario.json"))
        .expect("complete normal scenario must decode");
    let original_scenario = scenario.clone();
    let execution = simulate(&program, "exchange", &scenario)
        .expect("normal prefunded exchange must succeed locally");
    let actual = serde_json::to_value(&execution).expect("Execution must serialize");

    // Root can retain the complete observation before any expectation comparison.
    if let Some(path) = env::var_os("MORI_UTXO_PATTERN_EXECUTION_OUTPUT") {
        let mut bytes = serde_json::to_vec_pretty(&actual).expect("Execution JSON must encode");
        bytes.push(b'\n');
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("explicit fresh Execution output must open");
        file.write_all(&bytes)
            .expect("complete Execution must write");
        file.sync_all().expect("Execution output must persist");
    }

    let independent: Value =
        serde_json::from_str(fixture!("independent/COMPLETE-PREFIXES.expected.json"))
            .expect("independent complete semantic expectation must decode");
    assert_eq!(
        fixture!("program.mori"),
        fixture!("independent/inputs/program.mori")
    );
    assert_eq!(
        fixture!("normal.scenario.json"),
        fixture!("independent/inputs/normal.scenario.json")
    );
    assert_eq!(
        program.language,
        independent["language"]
            .as_str()
            .expect("independent language")
    );
    assert_eq!(
        program.name,
        independent["agreement"]
            .as_str()
            .expect("independent agreement")
    );
    assert_eq!(actual["semantics"], independent["semantics"]);
    assert_eq!(actual["action"], independent["action"]);
    assert_eq!(
        actual["pre"], independent["pre"],
        "independent complete prestate"
    );
    assert_eq!(
        actual["post"], independent["post"],
        "independent complete poststate"
    );

    let actual_steps = actual["steps"].as_array().expect("execution steps");
    let independent_prefixes = independent["prefixes"]
        .as_array()
        .expect("independent complete prefixes");
    assert_eq!(actual_steps.len(), 11);
    assert_eq!(independent_prefixes.len(), 11);
    let mut cumulative_effects = Vec::new();
    for (index, (step, prefix)) in actual_steps.iter().zip(independent_prefixes).enumerate() {
        assert_eq!(
            step["index"], prefix["index"],
            "independent prefix ordinal {index}"
        );
        assert_eq!(
            step["snapshot"], prefix["snapshot"],
            "independent full snapshot at prefix {index}"
        );
        assert_eq!(
            step["effects"], prefix["stepEffects"],
            "independent ordered delta effects at prefix {index}"
        );
        cumulative_effects.extend(
            step["effects"]
                .as_array()
                .expect("ordered per-step effects")
                .iter()
                .cloned(),
        );
        assert_eq!(
            Value::Array(cumulative_effects.clone()),
            prefix["cumulativeEffects"],
            "independent ordered cumulative effects at prefix {index}"
        );
    }
    assert_eq!(actual["effects"].as_array().expect("all effects").len(), 6);
    assert_eq!(
        actual["effects"], independent["effects"],
        "independent complete effects"
    );
    assert_eq!(Value::Array(cumulative_effects), independent["effects"]);

    // The financial oracle does not specify expression IDs, Statement IR or Step.value.
    // Those complete wire fields remain checked against the distinct authored fixture.
    let authored: Value = serde_json::from_str(fixture!("authored.expected.json"))
        .expect("authored complete expectation must decode");
    let expected = &authored["execution"];
    assert_eq!(program.language, "moriarty/1");
    assert_eq!(program.semantics, "moriarty-core/1");
    assert_eq!(program.name, "PrefundedBilateralExchange");
    assert_eq!(program.actions.len(), 1);
    assert_eq!(program.actions[0].name, "exchange");
    assert_eq!(actual["semantics"], expected["semantics"]);
    assert_eq!(actual["action"], expected["action"]);
    assert_eq!(
        actual["pre"], expected["pre"],
        "complete normalized prestate"
    );
    assert_eq!(
        actual["post"], expected["post"],
        "complete normal poststate"
    );

    let expected_steps = expected["steps"].as_array().expect("authored steps");
    assert_eq!(expected_steps.len(), 11);
    for (index, (step, expected_step)) in actual_steps.iter().zip(expected_steps).enumerate() {
        assert_eq!(
            step, expected_step,
            "complete authored wire prefix at statement {index}"
        );
    }
    let actual_effects = actual["effects"].as_array().expect("execution effects");
    let expected_effects = expected["effects"].as_array().expect("authored effects");
    assert_eq!(actual_effects.len(), 6);
    assert_eq!(expected_effects.len(), 6);
    for (index, (effect, expected_effect)) in
        actual_effects.iter().zip(expected_effects).enumerate()
    {
        assert_eq!(
            effect, expected_effect,
            "complete authored ordered effect {index}"
        );
    }
    assert_eq!(
        &actual, expected,
        "all authored public Execution wire fields"
    );
    assert_eq!(
        scenario, original_scenario,
        "borrowed input remains unchanged"
    );
}
