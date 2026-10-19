use mori_core::*;
use serde_json::Value as Json;
fn oracle() -> Json {
    serde_json::from_str(include_str!("fixtures/bound-replay-oracle.json")).unwrap()
}
fn execution() -> Execution {
    serde_json::from_value(oracle()["known_payment"]["expected_execution"].clone()).unwrap()
}
fn mutated(actual: &Execution, edit: impl FnOnce(&mut Json)) -> Execution {
    let mut v = serde_json::to_value(actual).unwrap();
    edit(&mut v);
    serde_json::from_value(v).unwrap()
}
fn difference(actual: &Execution, candidate: &Execution, step: Option<u16>, field: &str) {
    let d = compare_execution(actual, candidate).unwrap_err();
    assert_eq!(d.step, step);
    assert_eq!(d.field, field);
}
#[test]
fn detailed_replay_checks_independent_parser_and_complete_financial_oracle() {
    let c = &oracle()["contrasting"];
    let program = parse(c["source"].as_str().unwrap()).unwrap();
    assert_eq!(
        serde_json::to_value(&program).unwrap(),
        c["hand_lowered_program"]
    );
    let scenario: Scenario = serde_json::from_value(c["scenario"].clone()).unwrap();
    let expected: Execution = serde_json::from_value(c["expected_execution"].clone()).unwrap();
    assert_eq!(simulate(&program, "collect", &scenario).unwrap(), expected);
    replay_detailed(&program, "collect", &scenario, &expected).unwrap();
    replay(&program, "collect", &scenario, &expected).unwrap();
    let mut bad = scenario;
    bad.inputs.insert("amount".into(), Value::UInt(UInt(0)));
    assert!(matches!(
        replay_detailed(&program, "collect", &bad, &expected),
        Err(DetailedReplayError::Execution(_))
    ));
}
#[test]
fn first_step_and_explicit_field_order_survive_later_mutations() {
    let actual = execution();
    let cases = [
        ("index", "/steps/1/index"),
        ("statement", "/steps/1/statement/amount"),
        ("value", "/steps/1/value/value"),
        ("snapshot", "/steps/1/snapshot/balances/0/amount"),
        ("effects", "/steps/1/effects/0/amount"),
    ];
    for (field, path) in cases {
        let bad = mutated(&actual, |v| {
            v["steps"][5]["index"] = 99.into();
            match field {
                "index" => v["steps"][1][field] = 9.into(),
                "statement" => v["steps"][1][field]["amount"] = 0.into(),
                "value" => v["steps"][1][field]["value"] = "69".into(),
                "snapshot" => v["steps"][1][field]["balances"][0]["amount"] = "29".into(),
                "effects" => v["steps"][1][field][0]["amount"] = "69".into(),
                _ => unreachable!(),
            }
        });
        difference(&actual, &bad, Some(1), path);
    }
    let bad = mutated(&actual, |v| {
        v["steps"][1]["index"] = 99.into();
        v["steps"][1]["effects"][0]["amount"] = "1".into();
    });
    difference(&actual, &bad, Some(1), "/steps/1/index");
}
#[test]
fn missing_extra_steps_and_terminal_frames_have_deterministic_paths() {
    let actual = execution();
    let mut bad = actual.clone();
    bad.steps.pop();
    difference(&actual, &bad, Some(5), "/steps/5");
    let mut bad = actual.clone();
    bad.steps.push(actual.steps[5].clone());
    difference(&actual, &bad, Some(6), "/steps/6");
    let mut bad = actual.clone();
    bad.effects.swap(0, 1);
    difference(&actual, &bad, None, "/effects/0/amount");
    let mut bad = actual.clone();
    bad.effects.pop();
    difference(&actual, &bad, None, "/effects");
    let mut bad = actual.clone();
    bad.post.liabilities.pop();
    difference(&actual, &bad, None, "/post/liabilities");
    let bad = mutated(&actual, |v| {
        v["post"]["liabilities"][1]["amount"] = "6".into()
    });
    difference(&actual, &bad, None, "/post/liabilities/1/amount");
    let bad = mutated(&actual, |v| v["pre"]["balances"][0]["amount"] = "99".into());
    difference(&actual, &bad, None, "/pre/balances/0/amount");
}
#[test]
fn pointer_escaping_lexical_keys_and_legacy_option_decode_are_preserved() {
    let mut actual = execution();
    actual.post.state.insert("a~/b".into(), Value::Bool(true));
    let mut bad = actual.clone();
    bad.post.state.insert("a~/b".into(), Value::Bool(false));
    bad.post.state.insert("z".into(), Value::Bool(true));
    difference(&actual, &bad, None, "/post/state/a~0~1b/value");
    let mut v = serde_json::to_value(&actual.steps[0]).unwrap();
    v.as_object_mut().unwrap().remove("value");
    assert!(serde_json::from_value::<Step>(v).unwrap().value.is_none());
    assert_eq!(compare_execution(&actual, &actual), Ok(()));
}
