use mori_core::{Scenario, parse, simulate};
use serde_json::Value;

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
    assert_eq!(program.language, "moriarty/1");
    assert_eq!(program.semantics, "moriarty-core/1");
    assert_eq!(program.name, "PrefundedBilateralExchange");
    assert_eq!(program.actions.len(), 1);
    assert_eq!(program.actions[0].name, "exchange");

    let scenario: Scenario = serde_json::from_str(fixture!("normal.scenario.json"))
        .expect("complete normal scenario must decode");
    let original_scenario = scenario.clone();
    let authored: Value = serde_json::from_str(fixture!("authored.expected.json"))
        .expect("authored complete expectation must decode");
    let expected = &authored["execution"];

    let execution = simulate(&program, "exchange", &scenario)
        .expect("normal prefunded exchange must succeed locally");
    let actual = serde_json::to_value(&execution).expect("Execution must serialize");
    assert_eq!(actual["semantics"], expected["semantics"]);
    assert_eq!(actual["action"], expected["action"]);
    assert_eq!(actual["pre"], expected["pre"], "complete normalized prestate");
    assert_eq!(actual["post"], expected["post"], "complete normal poststate");

    let actual_steps = actual["steps"].as_array().expect("execution steps");
    let expected_steps = expected["steps"].as_array().expect("authored steps");
    assert_eq!(actual_steps.len(), 11);
    assert_eq!(expected_steps.len(), 11);
    for (index, (step, expected_step)) in
        actual_steps.iter().zip(expected_steps).enumerate()
    {
        // Every prefix includes the statement, result, all state, all balances,
        // both residual duties and exactly that statement's ordered effects.
        assert_eq!(step, expected_step, "complete prefix at statement {index}");
    }

    let actual_effects = actual["effects"].as_array().expect("execution effects");
    let expected_effects = expected["effects"].as_array().expect("authored effects");
    assert_eq!(actual_effects.len(), 6);
    assert_eq!(expected_effects.len(), 6);
    for (index, (effect, expected_effect)) in
        actual_effects.iter().zip(expected_effects).enumerate()
    {
        assert_eq!(effect, expected_effect, "complete ordered effect {index}");
    }

    assert_eq!(&actual, expected, "all public Execution fields");
    assert_eq!(scenario, original_scenario, "borrowed input remains unchanged");
}
