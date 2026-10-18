use mori_core::{Effect, Scenario, Snapshot, UInt, Value, parse, simulate, simulate_metered};
use serde::Deserialize;

const SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/guarded-quantity-fee-positive/quantity.mori"
));
const CASE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/guarded-quantity-fee-positive/case.json"
));

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expected {
    pre: Snapshot,
    post: Snapshot,
    effects: Vec<Effect>,
    step_values: Vec<Value>,
    logical_used: String,
    logical_cap: String,
}
#[derive(Deserialize)]
struct FrozenCase {
    id: String,
    action: String,
    scenario: Scenario,
    expected: Expected,
}

#[test]
fn guarded_quantity_arithmetic_keeps_fee_effects_and_residual_duties() {
    let case: FrozenCase = serde_json::from_str(CASE).unwrap();
    assert_eq!(case.id, "varied-quantity-price-fee");
    let original = case.scenario.clone();
    let program = parse(SOURCE).unwrap();
    assert_eq!(program.language, "moriarty/1");
    assert_eq!(program.semantics, "moriarty-core/1");
    let run = simulate(&program, &case.action, &case.scenario).unwrap();
    assert_eq!(run.action, "first_ge");
    assert_eq!(run.semantics, "moriarty-core/1");
    assert_eq!(run.pre, case.expected.pre);
    assert_eq!(run.post, case.expected.post);
    assert_eq!(run.effects, case.expected.effects);
    assert_eq!(run.steps.len(), case.expected.step_values.len());
    for (ordinal, (step, expected_value)) in
        run.steps.iter().zip(&case.expected.step_values).enumerate()
    {
        assert_eq!(usize::from(step.index), ordinal);
        assert_eq!(step.value.as_ref(), Some(expected_value));
        assert_eq!(step.snapshot.state["marker"], Value::UInt(UInt(91)));
        assert_eq!(step.snapshot.state["active"], Value::Bool(true));
        assert_eq!(
            step.snapshot.liabilities[1],
            case.expected.pre.liabilities[1]
        );
        assert_eq!(
            step.snapshot.liabilities[2],
            case.expected.pre.liabilities[2]
        );
        assert_eq!(step.snapshot.balances[4..], case.expected.pre.balances[4..]);
    }
    assert_eq!(case.scenario, original);

    let cap = UInt(case.expected.logical_cap.parse().unwrap());
    let metered = simulate_metered(&program, &case.action, &case.scenario, cap).unwrap();
    assert_eq!(metered.cost.profile, "moriarty-reference-logical-cost/1");
    assert_eq!(
        metered.cost.used,
        UInt(case.expected.logical_used.parse().unwrap())
    );
    assert_eq!(metered.execution, run);
}
