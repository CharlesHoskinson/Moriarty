//! A positive Core/1 payment against independent financial prefix literals.
use mori_core::{Execution, Scenario, parse, simulate, validate};
use serde_json::Value;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/core-positive-transfer/",
            $name
        ))
    };
}

fn financial_projection(execution: &Execution) -> Value {
    let mut actual = serde_json::to_value(execution).unwrap();
    // The financial oracle excludes raw statement/ExprId serialization only.
    // Keep all other actual fields, every row and their original array order.
    for step in actual["steps"].as_array_mut().unwrap() {
        assert!(step.as_object_mut().unwrap().remove("statement").is_some());
    }
    actual
}

#[test]
fn print_run_matches_every_independent_financial_prefix() {
    let program = parse(fixture!("print-run.mori")).expect("positive Core source must parse");
    validate(&program).expect("positive Core program must validate");
    assert_eq!(program.name, "PrintRun");
    let scenario: Scenario = serde_json::from_str(fixture!("print-run.scenario.json")).unwrap();
    let original = scenario.clone();
    let execution = simulate(&program, "pay", &scenario).expect("positive payment must succeed");
    let expected: Value =
        serde_json::from_str(fixture!("financial-execution-projection.expected.json")).unwrap();
    assert_eq!(financial_projection(&execution), expected);
    assert_eq!(scenario, original);
}
