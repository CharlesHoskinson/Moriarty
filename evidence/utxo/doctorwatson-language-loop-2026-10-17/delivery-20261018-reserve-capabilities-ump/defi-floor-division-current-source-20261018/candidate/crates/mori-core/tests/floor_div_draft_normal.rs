#![cfg(feature = "core-floor-div-draft")]

use mori_core::floor_div_draft as draft;
use mori_core::{AssetId, Balance, Effect, Liability, Scenario, Snapshot, UInt, Value};
use std::collections::BTreeMap;
const SWAP: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
    "/../fixtures/defi-floor-div-draft/constant-product.mori"));
fn u(n: u128) -> Value { Value::UInt(UInt(n)) }
fn b(value: bool) -> Value { Value::Bool(value) }
fn asset(reference: &str) -> AssetId {
    AssetId { network: "specimen".into(), issuer: "example-issuer".into(),
        reference: reference.into(), scale: 0 }
}
fn snapshot(a: u128, bb: u128, revision: u128, trader_a: u128, trader_b: u128) -> Snapshot {
    let mut balances = Vec::new();
    for (id, rows) in [
        (asset("A"), [("pool", a), ("spectator", 41), ("trader", trader_a), ("treasury", 11)]),
        (asset("B"), [("pool", bb), ("spectator", 43), ("trader", trader_b), ("treasury", 17)]),
    ] {
        for (account, amount) in rows {
            balances.push(Balance { account: account.into(), asset: id.clone(), amount: UInt(amount) });
        }
    }
    Snapshot {
        state: BTreeMap::from([("reserveA".into(), u(a)), ("reserveB".into(), u(bb)),
            ("revision".into(), u(revision)), ("marker".into(), u(7)), ("paused".into(), b(false))]),
        balances,
        liabilities: vec![
            Liability { name: "reserveDuty".into(), asset: asset("A"), debtor: "pool".into(),
                creditor: "saver".into(), amount: UInt(14) },
            Liability { name: "settledDuty".into(), asset: asset("B"), debtor: "trader".into(),
                creditor: "saver".into(), amount: UInt(0) },
        ],
    }
}
fn scenario(pre: &Snapshot, inputs: [(&str, u128); 2]) -> Scenario {
    Scenario { state: pre.state.clone(),
        inputs: inputs.into_iter().map(|(name, value)| (name.into(), u(value))).collect(),
        balances: pre.balances.clone(), liabilities: pre.liabilities.clone() }
}
fn effects(old: (u128,u128,u128), new: (u128,u128), output: u128, event: (&str,u128)) -> Vec<Effect> {
    vec![
        Effect::Transfer { asset: asset("A"), from: "trader".into(), to: "pool".into(), amount: UInt(10_000) },
        Effect::Transfer { asset: asset("B"), from: "pool".into(), to: "trader".into(), amount: UInt(output) },
        Effect::State { name: "reserveA".into(), before: u(old.0), after: u(new.0) },
        Effect::State { name: "reserveB".into(), before: u(old.1), after: u(new.1) },
        Effect::State { name: "revision".into(), before: u(old.2), after: u(old.2 + 1) },
        Effect::Emit { name: event.0.into(), value: u(event.1) },
    ]
}
#[test]
fn generic_floor_division_is_source_computed() {
    let source = format!(r#"language "{}"; agreement Quotient {{ state q: UInt = 0;
        action divide(n: UInt, d: UInt) {{ next q = floor_div(n, d); }} }}"#, draft::LANGUAGE);
    let program = draft::parse(&source).unwrap();
    draft::validate(&program).unwrap();
    for (n, d, q) in [(7, 3, 2), (0, 3, 0), (u128::MAX, 1, u128::MAX)] {
        let mut input = Scenario::default();
        input.inputs = BTreeMap::from([("n".into(), u(n)), ("d".into(), u(d))]);
        let execution = draft::simulate(&program, "divide", &input).unwrap();
        assert_eq!(execution.post.state["q"], u(q));
        assert_eq!(execution.effects, vec![Effect::State { name: "q".into(), before: u(0), after: u(q) }]);
    }
}
#[test]
fn exact_input_preserves_complete_normal_state_values_and_ordered_effects() {
    let program = draft::parse(SWAP).unwrap();
    let pre = snapshot(1_000_000, 2_000_000, 4, 30_000, 0);
    let input = scenario(&pre, [("amountIn", 10_000), ("minimumOut", 19_743)]);
    let saved = input.clone();
    let execution = draft::simulate(&program, "exactInput", &input).unwrap();
    let post = snapshot(1_010_000, 1_980_257, 5, 20_000, 19_743);
    let deltas = effects((1_000_000,2_000_000,4), (1_010_000,1_980_257), 19_743, ("amountOut",19_743));
    assert_eq!(execution.semantics, draft::SEMANTICS);
    assert_eq!(execution.pre, pre);
    assert_eq!(execution.post, post);
    assert_eq!(execution.effects, deltas);
    assert_eq!(input, saved);
    let values = vec![b(true),b(true),b(true),b(true),u(1_000_000),u(2_000_000),u(9_970_000),
        u(19_940_000_000_000),u(1_009_970_000),u(19_743),b(true),b(true),u(1_010_000),
        u(1_980_257),b(true),u(10_000),u(19_743),u(1_010_000),u(1_980_257),u(5),u(19_743),b(true),b(true)];
    assert_eq!(execution.steps.len(), values.len());
    for (ordinal, (step, value)) in execution.steps.iter().zip(values).enumerate() {
        assert_eq!(usize::from(step.index), ordinal);
        assert_eq!(step.value, Some(value));
        let mut expected = pre.clone();
        if ordinal >= 15 {
            expected.balances[0].amount = UInt(1_010_000);
            expected.balances[2].amount = UInt(20_000);
        }
        if ordinal >= 16 { expected.balances = post.balances.clone(); }
        if ordinal >= 17 { expected.state.insert("reserveA".into(),u(1_010_000)); }
        if ordinal >= 18 { expected.state.insert("reserveB".into(),u(1_980_257)); }
        if ordinal >= 19 { expected.state.insert("revision".into(),u(5)); }
        assert_eq!(step.snapshot, expected);
        let delta = match ordinal { 15..=20 => vec![deltas[ordinal-15].clone()], _ => Vec::new() };
        assert_eq!(step.effects, delta);
    }
    let metered = draft::simulate_metered(&program,"exactInput",&input,UInt(100)).unwrap();
    assert_eq!(metered.execution,execution);
    assert_eq!(metered.cost.profile,draft::LOGICAL_COST_PROFILE);
    assert_eq!(metered.cost.used,UInt(83));
    draft::replay_detailed(&program,"exactInput",&input,&execution).unwrap();
}
#[test]
fn successor_trade_uses_current_reserves_and_keeps_duties() {
    let program = draft::parse(SWAP).unwrap();
    let pre = snapshot(1_010_000,1_980_257,5,20_000,19_743);
    let input = scenario(&pre,[("amountIn",10_000),("minimumOut",19_356)]);
    let execution = draft::simulate(&program,"exactInput",&input).unwrap();
    assert_eq!(execution.pre,pre);
    assert_eq!(execution.post,snapshot(1_020_000,1_960_901,6,10_000,39_099));
    assert_eq!(execution.effects,effects((1_010_000,1_980_257,5),(1_020_000,1_960_901),19_356,("amountOut",19_356)));
    assert_eq!(execution.steps[9].value,Some(u(19_356)));
    let metered = draft::simulate_metered(&program,"exactInput",&input,UInt(100)).unwrap();
    assert_eq!(metered.cost.used,UInt(83));
    assert_eq!(metered.execution,execution);
}
#[test]
fn exact_output_computes_ceiling_from_the_general_primitive() {
    let program = draft::parse(SWAP).unwrap();
    let pre = snapshot(1_000_000,2_000_000,4,30_000,0);
    let input = scenario(&pre,[("desiredOut",19_743),("maximumIn",10_000)]);
    let execution = draft::simulate(&program,"exactOutput",&input).unwrap();
    assert_eq!(execution.pre,pre);
    assert_eq!(execution.post,snapshot(1_010_000,1_980_257,5,20_000,19_743));
    assert_eq!(execution.effects,effects((1_000_000,2_000_000,4),(1_010_000,1_980_257),19_743,("amountIn",10_000)));
    assert_eq!(execution.steps.len(),22);
    assert_eq!(execution.steps[9].value,Some(u(10_000)));
    let metered = draft::simulate_metered(&program,"exactOutput",&input,UInt(100)).unwrap();
    assert_eq!(metered.cost.used,UInt(85));
    assert_eq!(metered.execution,execution);
}
#[test]
fn existing_core1_counter_keeps_the_existing_public_consumer() {
    let source = r#"language "moriarty/1"; agreement Counter { state count: UInt = 0;
        action increment(amount: UInt) { let total = count + amount; requires total <= 100;
            next count = total; ensures count == total; emit "count"(count); } }"#;
    let program = mori_core::parse(source).unwrap();
    let mut input = Scenario::default(); input.inputs.insert("amount".into(),u(7));
    let metered = mori_core::simulate_metered(&program,"increment",&input,UInt(16)).unwrap();
    assert_eq!(metered.execution.post.state["count"],u(7));
    assert_eq!(metered.cost.used,UInt(16));
    assert_eq!(metered.cost.profile,mori_core::LOGICAL_COST_PROFILE);
}
