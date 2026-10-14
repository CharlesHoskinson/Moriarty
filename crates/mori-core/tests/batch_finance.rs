//! Local runtime expectations for exact base-per-quote batch and swap.
//! Ordinary `simulate` / `replay` only. This file does not certify native execution.
use mori_core::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
fn program() -> Program {
    parse(include_str!(
        "../../../examples/core/batch-finance/batch.mori"
    ))
    .expect("batch source")
}
fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/core/batch-finance")
        .join(name);
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {name}: {e}"))
}
fn asset<'a>(program: &'a Program, name: &str) -> &'a AssetId {
    &program
        .assets
        .iter()
        .find(|item| item.name == name)
        .unwrap()
        .identity
}
fn balance(account: &str, asset: &AssetId, amount: u128) -> Balance {
    Balance {
        account: account.into(),
        asset: asset.clone(),
        amount: UInt(amount),
    }
}
fn sum_of(balances: &[Balance], id: &AssetId) -> u128 {
    balances
        .iter()
        .filter(|item| &item.asset == id)
        .map(|item| item.amount.0)
        .sum()
}
fn amount_at(balances: &[Balance], id: &AssetId, account: &str) -> u128 {
    balances
        .iter()
        .find(|item| &item.asset == id && item.account == account)
        .unwrap()
        .amount
        .0
}
fn state_at(snapshot: &Snapshot, name: &str) -> u128 {
    match &snapshot.state[name] {
        Value::UInt(n) => n.0,
        Value::Bool(_) => panic!("{name} is Bool"),
    }
}
fn input_at(scenario: &Scenario, name: &str) -> u128 {
    match &scenario.inputs[name] {
        Value::UInt(n) => n.0,
        Value::Bool(_) => panic!("{name} is Bool"),
    }
}
fn refuse(program: &Program, action: &str, scenario: &Scenario) -> Error {
    let original = scenario.clone();
    let err = simulate(program, action, scenario).expect_err(action);
    assert_eq!(scenario, &original, "{}", err.code);
    err
}

#[derive(Clone)]
struct Clear {
    alice_quote: u128,
    bob_quote: u128,
    alice_base: u128,
    bob_base: u128,
    price: u128,
    alice_price: u128,
    bob_price: u128,
    oracle_price: u128,
    observed_at: u128,
    now: u128,
    alice_fee: u128,
    bob_fee: u128,
    alice_rebate: u128,
    bob_rebate: u128,
    solver_cap: u128,
    gross_cap: u128,
    min_net: u128,
    settled_duty: u128,
    same_domain: bool,
    oracle_authorized: bool,
    solver_authorized: bool,
    base_maker: u128,
    base_alice: u128,
    base_bob: u128,
    quote_alice: u128,
    quote_bob: u128,
    quote_maker: u128,
    quote_pool: u128,
    gross: u128,
    fees: u128,
    net: u128,
    duty: u128,
    prior_duty: Option<u128>,
}
impl Default for Clear {
    fn default() -> Self {
        Self {
            alice_quote: 3,
            bob_quote: 5,
            alice_base: 12,
            bob_base: 20,
            price: 4,
            alice_price: 4,
            bob_price: 4,
            oracle_price: 4,
            observed_at: 40,
            now: 48,
            alice_fee: 1,
            bob_fee: 2,
            alice_rebate: 2,
            bob_rebate: 1,
            solver_cap: 32,
            gross_cap: 11,
            min_net: 5,
            settled_duty: 1,
            same_domain: true,
            oracle_authorized: true,
            solver_authorized: true,
            base_maker: 40,
            base_alice: 0,
            base_bob: 0,
            quote_alice: 10,
            quote_bob: 20,
            quote_maker: 0,
            quote_pool: 0,
            gross: 0,
            fees: 0,
            net: 0,
            duty: 0,
            prior_duty: None,
        }
    }
}
impl Clear {
    fn scenario(&self, program: &Program) -> Scenario {
        let base = asset(program, "BASE");
        let quote = asset(program, "QUOTE");
        let inputs = BTreeMap::from([
            ("aliceQuote".into(), uint(self.alice_quote)),
            ("bobQuote".into(), uint(self.bob_quote)),
            ("aliceBase".into(), uint(self.alice_base)),
            ("bobBase".into(), uint(self.bob_base)),
            ("price".into(), uint(self.price)),
            ("alicePrice".into(), uint(self.alice_price)),
            ("bobPrice".into(), uint(self.bob_price)),
            ("oraclePrice".into(), uint(self.oracle_price)),
            ("observedAt".into(), uint(self.observed_at)),
            ("now".into(), uint(self.now)),
            ("aliceFee".into(), uint(self.alice_fee)),
            ("bobFee".into(), uint(self.bob_fee)),
            ("aliceRebate".into(), uint(self.alice_rebate)),
            ("bobRebate".into(), uint(self.bob_rebate)),
            ("solverCap".into(), uint(self.solver_cap)),
            ("grossCap".into(), uint(self.gross_cap)),
            ("minNet".into(), uint(self.min_net)),
            ("settledDuty".into(), uint(self.settled_duty)),
            ("sameDomain".into(), Value::Bool(self.same_domain)),
            (
                "oracleAuthorized".into(),
                Value::Bool(self.oracle_authorized),
            ),
            (
                "solverAuthorized".into(),
                Value::Bool(self.solver_authorized),
            ),
        ]);
        let state = if self.gross == 0 && self.fees == 0 && self.net == 0 && self.duty == 0 {
            BTreeMap::new()
        } else {
            BTreeMap::from([
                ("gross".into(), uint(self.gross)),
                ("fees".into(), uint(self.fees)),
                ("net".into(), uint(self.net)),
                ("duty".into(), uint(self.duty)),
            ])
        };
        let liabilities = self
            .prior_duty
            .map(|amount| Liability {
                name: "clearDuty".into(),
                asset: quote.clone(),
                debtor: "maker".into(),
                creditor: "reserve".into(),
                amount: UInt(amount),
            })
            .into_iter()
            .collect();
        Scenario {
            state,
            inputs,
            balances: vec![
                balance("maker", base, self.base_maker),
                balance("alice", base, self.base_alice),
                balance("bob", base, self.base_bob),
                balance("alice", quote, self.quote_alice),
                balance("bob", quote, self.quote_bob),
                balance("maker", quote, self.quote_maker),
                balance("pool", quote, self.quote_pool),
            ],
            liabilities,
        }
    }
}

#[derive(Clone)]
struct Swap {
    quote_qty: u128,
    base_qty: u128,
    price: u128,
    left_price: u128,
    right_price: u128,
    oracle_price: u128,
    observed_at: u128,
    now: u128,
    left_fee: u128,
    settled_duty: u128,
    solver_cap: u128,
    gross_cap: u128,
    min_net: u128,
    same_domain: bool,
    oracle_authorized: bool,
    solver_authorized: bool,
    base_left: u128,
    base_right: u128,
    quote_left: u128,
    quote_right: u128,
    quote_pool: u128,
}
impl Default for Swap {
    fn default() -> Self {
        Self {
            quote_qty: 6,
            base_qty: 42,
            price: 7,
            left_price: 7,
            right_price: 7,
            oracle_price: 7,
            observed_at: 100,
            now: 100,
            left_fee: 2,
            settled_duty: 2,
            solver_cap: 42,
            gross_cap: 8,
            min_net: 4,
            same_domain: true,
            oracle_authorized: true,
            solver_authorized: true,
            base_left: 1,
            base_right: 50,
            quote_left: 15,
            quote_right: 4,
            quote_pool: 9,
        }
    }
}
impl Swap {
    fn scenario(&self, program: &Program) -> Scenario {
        let base = asset(program, "BASE");
        let quote = asset(program, "QUOTE");
        Scenario {
            inputs: BTreeMap::from([
                ("quoteQty".into(), uint(self.quote_qty)),
                ("baseQty".into(), uint(self.base_qty)),
                ("price".into(), uint(self.price)),
                ("leftPrice".into(), uint(self.left_price)),
                ("rightPrice".into(), uint(self.right_price)),
                ("oraclePrice".into(), uint(self.oracle_price)),
                ("observedAt".into(), uint(self.observed_at)),
                ("now".into(), uint(self.now)),
                ("leftFee".into(), uint(self.left_fee)),
                ("settledDuty".into(), uint(self.settled_duty)),
                ("solverCap".into(), uint(self.solver_cap)),
                ("grossCap".into(), uint(self.gross_cap)),
                ("minNet".into(), uint(self.min_net)),
                ("sameDomain".into(), Value::Bool(self.same_domain)),
                (
                    "oracleAuthorized".into(),
                    Value::Bool(self.oracle_authorized),
                ),
                (
                    "solverAuthorized".into(),
                    Value::Bool(self.solver_authorized),
                ),
            ]),
            balances: vec![
                balance("left", base, self.base_left),
                balance("right", base, self.base_right),
                balance("left", quote, self.quote_left),
                balance("right", quote, self.quote_right),
                balance("pool", quote, self.quote_pool),
            ],
            ..Default::default()
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    profile: String,
    cases: Vec<Case>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    name: String,
    source: String,
    scenario: String,
    action: String,
    expected: Expected,
}
#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum Expected {
    Accepted {
        post: Snapshot,
        effects: Vec<Effect>,
    },
    Rejected {
        code: String,
    },
}

#[test]
fn declared_assets_share_one_domain_and_differ_by_issuer_and_reference() {
    let program = program();
    assert_eq!(program.name, "ExactBatchSwap");
    assert_eq!(
        program
            .actions
            .iter()
            .map(|action| action.name.as_str())
            .collect::<Vec<_>>(),
        ["clear", "swap"]
    );
    let base = asset(&program, "BASE").clone();
    let quote = asset(&program, "QUOTE").clone();
    assert_eq!(base.network, "local");
    assert_eq!(quote.network, "local");
    assert_eq!(base.issuer, "base-desk");
    assert_eq!(quote.issuer, "quote-desk");
    assert_eq!(base.reference, "BASE");
    assert_eq!(quote.reference, "QUOTE");
    assert_eq!(base.scale, 0);
    assert_eq!(quote.scale, 0);
    assert_ne!(base.issuer, quote.issuer);
    assert_ne!(base.reference, quote.reference);
}

#[test]
fn manifest_cases_match_the_runtime_and_rejections_leave_inputs() {
    let program = program();
    let manifest: Manifest = serde_json::from_str(&fixture("mori-tests.json")).unwrap();
    assert_eq!(manifest.profile, "moriarty-tests/1");
    assert_eq!(manifest.cases.len(), 4);
    for case in manifest.cases {
        assert_eq!(case.source, "batch.mori");
        let scenario: Scenario = serde_json::from_str(&fixture(&case.scenario)).unwrap();
        let original = scenario.clone();
        let result = simulate(&program, &case.action, &scenario);
        match case.expected {
            Expected::Accepted { post, effects } => {
                let execution = result.expect(&case.name);
                assert_eq!(execution.post, post, "{}", case.name);
                assert_eq!(execution.effects, effects, "{}", case.name);
                assert_eq!(&scenario, &original);
                replay(&program, &case.action, &scenario, &execution).unwrap();
                let base = asset(&program, "BASE");
                let quote = asset(&program, "QUOTE");
                assert_eq!(
                    sum_of(&execution.pre.balances, base),
                    sum_of(&post.balances, base)
                );
                assert_eq!(
                    sum_of(&execution.pre.balances, quote),
                    sum_of(&post.balances, quote)
                );
                if case.action == "clear" {
                    assert_eq!(
                        input_at(&scenario, "aliceBase"),
                        input_at(&scenario, "aliceQuote") * input_at(&scenario, "price")
                    );
                    assert_ne!(
                        input_at(&scenario, "aliceQuote"),
                        input_at(&scenario, "aliceBase") * input_at(&scenario, "price")
                    );
                    assert_eq!(
                        input_at(&scenario, "bobBase"),
                        input_at(&scenario, "bobQuote") * input_at(&scenario, "price")
                    );
                    assert_eq!(state_at(&post, "gross"), 11);
                    assert_eq!(state_at(&post, "fees"), 3);
                    assert_eq!(state_at(&post, "net"), 5);
                    assert_eq!(state_at(&post, "duty"), 2);
                    assert_eq!(post.liabilities[0].amount, UInt(2));
                } else {
                    assert_eq!(
                        input_at(&scenario, "baseQty"),
                        input_at(&scenario, "quoteQty") * input_at(&scenario, "price")
                    );
                    assert_ne!(
                        input_at(&scenario, "quoteQty"),
                        input_at(&scenario, "baseQty") * input_at(&scenario, "price")
                    );
                    assert_eq!(state_at(&post, "gross"), 8);
                    assert_eq!(state_at(&post, "net"), 4);
                    assert_eq!(post.liabilities[0].amount, UInt(0));
                    assert_eq!(post.liabilities[0].name, "swapDuty");
                }
            }
            Expected::Rejected { code } => {
                let err = result.expect_err(&case.name);
                assert_eq!(err.code, code, "{}", case.name);
                assert_eq!(scenario, original, "{}", case.name);
            }
        }
    }
}

#[test]
fn quote_per_base_amounts_and_unequal_prices_refuse() {
    let program = program();
    let mut swapped = Clear::default();
    swapped.alice_base = 3;
    swapped.alice_quote = 12;
    swapped.bob_base = 5;
    swapped.bob_quote = 20;
    swapped.quote_alice = 20;
    swapped.quote_bob = 30;
    swapped.base_maker = 20;
    assert_eq!(swapped.alice_quote, swapped.alice_base * swapped.price);
    assert_ne!(swapped.alice_base, swapped.alice_quote * swapped.price);
    let err = refuse(&program, "clear", &swapped.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(14));

    let mut unequal = Clear::default();
    unequal.bob_price = 6;
    let err = refuse(&program, "clear", &unequal.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(9));

    let mut swap = Swap::default();
    swap.right_price = 8;
    let err = refuse(&program, "swap", &swap.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(9));
}

#[test]
fn solver_cap_stale_future_and_local_premises_refuse() {
    let program = program();
    let mut over = Clear::default();
    over.solver_cap = 31;
    let err = refuse(&program, "clear", &over.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(17));

    let mut zero_cap = Clear::default();
    zero_cap.solver_cap = 0;
    assert_eq!(
        refuse(&program, "clear", &zero_cap.scenario(&program)).step,
        Some(17)
    );

    let stale: Scenario = serde_json::from_str(&fixture("reject-stale.json")).unwrap();
    let err = refuse(&program, "clear", &stale);
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(5));

    let mut future = Clear::default();
    future.now = 39;
    let err = refuse(&program, "clear", &future.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(3));

    for (field, step) in [("domain", 0u16), ("oracle", 1u16), ("solver", 2u16)] {
        let mut case = Clear::default();
        match field {
            "domain" => case.same_domain = false,
            "oracle" => case.oracle_authorized = false,
            "solver" => case.solver_authorized = false,
            _ => unreachable!(),
        }
        let err = refuse(&program, "clear", &case.scenario(&program));
        assert_eq!(err.code, "mori::core::requires");
        assert_eq!(err.step, Some(step), "{field}");
    }
}

#[test]
fn gross_rebates_cannot_satisfy_the_separate_gross_cap() {
    let program = program();
    let mut bypass = Clear::default();
    bypass.gross_cap = 10;
    let err = refuse(&program, "clear", &bypass.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(22));

    let accepted = simulate(&program, "clear", &Clear::default().scenario(&program)).unwrap();
    assert_eq!(state_at(&accepted.post, "gross"), 11);
    assert_eq!(state_at(&accepted.post, "net"), 5);
    assert_eq!(
        amount_at(&accepted.post.balances, asset(&program, "QUOTE"), "maker"),
        5
    );
    assert_eq!(
        amount_at(&accepted.post.balances, asset(&program, "QUOTE"), "pool"),
        3
    );
}

#[test]
fn fees_reduce_the_net_goal_and_a_short_balance_rolls_back() {
    let program = program();
    let mut short_net = Clear::default();
    short_net.min_net = 6;
    let err = refuse(&program, "clear", &short_net.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(20));

    let mut tight = Clear::default();
    tight.quote_alice = 3;
    let err = refuse(&program, "clear", &tight.scenario(&program));
    assert_eq!(err.code, "mori::core::balance");
    assert_eq!(err.step, Some(32));
}

#[test]
fn asset_identity_rejects_same_symbol_with_other_issuer_reference_scale_or_domain() {
    let program = program();
    let base = asset(&program, "BASE").clone();
    let quote = asset(&program, "QUOTE").clone();
    let mutations: [(&str, AssetId); 4] = [
        (
            "issuer",
            AssetId {
                issuer: "other-desk".into(),
                ..base.clone()
            },
        ),
        (
            "reference",
            AssetId {
                reference: "BASE-OTHER".into(),
                ..base.clone()
            },
        ),
        (
            "scale",
            AssetId {
                scale: 2,
                ..quote.clone()
            },
        ),
        (
            "domain",
            AssetId {
                network: "other".into(),
                ..base.clone()
            },
        ),
    ];
    for (label, identity) in mutations {
        let mut scenario = Clear::default().scenario(&program);
        scenario.balances.push(balance("lookalike", &identity, 7));
        let err = refuse(&program, "clear", &scenario);
        assert_eq!(err.code, "mori::core::asset", "{label}");
        assert_eq!(err.step, None, "{label}");
    }
    let cross: Scenario = serde_json::from_str(&fixture("reject-cross-domain.json")).unwrap();
    assert_eq!(cross.balances[0].asset.reference, "BASE");
    assert_eq!(cross.balances[0].asset.network, "other");
    let err = refuse(&program, "clear", &cross);
    assert_eq!(err.code, "mori::core::asset");
    assert_eq!(err.step, None);
}

#[test]
fn held_out_amounts_accumulate_and_boundary_equality_holds() {
    let program = program();
    let base = asset(&program, "BASE").clone();
    let quote = asset(&program, "QUOTE").clone();
    let mut held = Clear::default();
    held.price = 9;
    held.alice_price = 9;
    held.bob_price = 9;
    held.oracle_price = 9;
    held.alice_quote = 2;
    held.bob_quote = 4;
    held.alice_base = 18;
    held.bob_base = 36;
    held.alice_fee = 1;
    held.bob_fee = 1;
    held.alice_rebate = 0;
    held.bob_rebate = 0;
    held.settled_duty = 0;
    held.min_net = 4;
    held.solver_cap = 54;
    held.gross_cap = 13;
    held.observed_at = 7;
    held.now = 7;
    held.base_maker = 54;
    held.quote_alice = 3;
    held.quote_bob = 5;
    held.gross = 5;
    held.fees = 2;
    held.net = 1;
    held.duty = 4;
    held.prior_duty = Some(4);
    let scenario = held.scenario(&program);
    let before_base = sum_of(&scenario.balances, &base);
    let before_quote = sum_of(&scenario.balances, &quote);
    let execution = simulate(&program, "clear", &scenario).unwrap();
    replay(&program, "clear", &scenario, &execution).unwrap();
    assert_eq!(sum_of(&execution.post.balances, &base), before_base);
    assert_eq!(sum_of(&execution.post.balances, &quote), before_quote);
    assert_eq!(amount_at(&execution.post.balances, &base, "alice"), 18);
    assert_eq!(amount_at(&execution.post.balances, &base, "bob"), 36);
    assert_eq!(amount_at(&execution.post.balances, &base, "maker"), 0);
    assert_eq!(amount_at(&execution.post.balances, &quote, "alice"), 0);
    assert_eq!(amount_at(&execution.post.balances, &quote, "bob"), 0);
    assert_eq!(amount_at(&execution.post.balances, &quote, "maker"), 6);
    assert_eq!(amount_at(&execution.post.balances, &quote, "pool"), 2);
    assert_eq!(state_at(&execution.post, "gross"), 13);
    assert_eq!(state_at(&execution.post, "fees"), 4);
    assert_eq!(state_at(&execution.post, "net"), 5);
    assert_eq!(state_at(&execution.post, "duty"), 6);
    assert_eq!(execution.post.liabilities[0].amount, UInt(6));
    assert_eq!(execution.post.liabilities[0].name, "clearDuty");

    let mut off = Clear::default();
    off.alice_base = 13;
    let err = refuse(&program, "clear", &off.scenario(&program));
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(14));
}

#[test]
fn limb_product_accepts_and_overflow_boundaries_refuse() {
    let program = program();
    const LIMB: u128 = u64::MAX as u128;
    const ALICE_BASE: u128 = LIMB * LIMB;
    const BASE_FILLED: u128 = ALICE_BASE + LIMB;
    const NOTIONAL: u128 = LIMB + 1;
    let mut limb = Clear {
        alice_quote: LIMB,
        bob_quote: 1,
        alice_base: ALICE_BASE,
        bob_base: LIMB,
        price: LIMB,
        alice_price: LIMB,
        bob_price: LIMB,
        oracle_price: LIMB,
        observed_at: 0,
        now: 0,
        alice_fee: 0,
        bob_fee: 0,
        alice_rebate: 0,
        bob_rebate: 0,
        solver_cap: BASE_FILLED,
        gross_cap: NOTIONAL,
        min_net: NOTIONAL,
        settled_duty: 0,
        base_maker: BASE_FILLED,
        base_alice: 0,
        base_bob: 0,
        quote_alice: LIMB,
        quote_bob: 1,
        quote_maker: 0,
        quote_pool: 0,
        ..Clear::default()
    };
    let scenario = limb.scenario(&program);
    let execution = simulate(&program, "clear", &scenario).unwrap();
    replay(&program, "clear", &scenario, &execution).unwrap();
    let base = asset(&program, "BASE");
    let quote = asset(&program, "QUOTE");
    assert_eq!(
        amount_at(&execution.post.balances, base, "alice"),
        ALICE_BASE
    );
    assert_eq!(amount_at(&execution.post.balances, base, "bob"), LIMB);
    assert_eq!(amount_at(&execution.post.balances, base, "maker"), 0);
    assert_eq!(
        amount_at(&execution.post.balances, quote, "maker"),
        NOTIONAL
    );
    assert_eq!(amount_at(&execution.post.balances, quote, "alice"), 0);
    assert_eq!(state_at(&execution.post, "gross"), NOTIONAL);
    assert_eq!(state_at(&execution.post, "fees"), 0);
    assert_eq!(state_at(&execution.post, "net"), NOTIONAL);
    assert_eq!(state_at(&execution.post, "duty"), 0);
    assert_eq!(execution.post.liabilities.len(), 1);
    assert_eq!(execution.post.liabilities[0].amount, UInt(0));
    assert_eq!(
        sum_of(&execution.pre.balances, base),
        sum_of(&execution.post.balances, base)
    );
    assert_eq!(
        sum_of(&execution.pre.balances, quote),
        sum_of(&execution.post.balances, quote)
    );

    limb.alice_quote = 2;
    limb.price = 1u128 << 127;
    limb.alice_price = limb.price;
    limb.bob_price = limb.price;
    limb.oracle_price = limb.price;
    limb.alice_base = 0;
    let err = refuse(&program, "clear", &limb.scenario(&program));
    assert_eq!(err.code, "mori::core::overflow");
    assert_eq!(err.step, Some(12));

    let mut add = Clear::default();
    add.price = 1;
    add.alice_price = 1;
    add.bob_price = 1;
    add.oracle_price = 1;
    add.alice_quote = u128::MAX;
    add.bob_quote = 1;
    add.alice_base = u128::MAX;
    add.bob_base = 1;
    add.alice_fee = 0;
    add.bob_fee = 0;
    add.alice_rebate = 0;
    add.bob_rebate = 0;
    add.settled_duty = 0;
    add.min_net = 0;
    add.solver_cap = u128::MAX;
    add.gross_cap = u128::MAX;
    add.quote_alice = u128::MAX;
    add.base_maker = u128::MAX;
    let err = refuse(&program, "clear", &add.scenario(&program));
    assert_eq!(err.code, "mori::core::overflow");
    assert_eq!(err.step, Some(16));

    let mut fee = Clear::default();
    fee.price = 1;
    fee.alice_price = 1;
    fee.bob_price = 1;
    fee.oracle_price = 1;
    fee.alice_quote = 1;
    fee.bob_quote = 1;
    fee.alice_base = 1;
    fee.bob_base = 1;
    fee.alice_fee = u128::MAX;
    fee.bob_fee = 1;
    fee.alice_rebate = 0;
    fee.bob_rebate = 0;
    fee.settled_duty = 0;
    fee.min_net = 0;
    fee.solver_cap = 2;
    fee.gross_cap = u128::MAX;
    fee.quote_alice = 1;
    fee.quote_bob = 1;
    fee.base_maker = 2;
    let err = refuse(&program, "clear", &fee.scenario(&program));
    assert_eq!(err.code, "mori::core::overflow");
    assert_eq!(err.step, Some(19));

    let mut gross = Clear::default();
    gross.gross = u128::MAX - 1;
    gross.fees = 0;
    gross.net = 0;
    gross.duty = 0;
    let err = refuse(&program, "clear", &gross.scenario(&program));
    assert_eq!(err.code, "mori::core::overflow");
    assert_eq!(err.step, Some(22));
}

#[test]
fn clear_then_swap_keeps_residual_duty_and_cumulative_gross() {
    let program = program();
    let base = asset(&program, "BASE").clone();
    let quote = asset(&program, "QUOTE").clone();
    let clear_run = simulate(&program, "clear", &Clear::default().scenario(&program)).unwrap();
    let mut swap = Swap::default();
    swap.quote_pool = 0;
    swap.gross_cap = 19;
    let mut next = Scenario {
        state: clear_run.post.state.clone(),
        balances: clear_run.post.balances.clone(),
        liabilities: clear_run.post.liabilities.clone(),
        inputs: swap.scenario(&program).inputs,
    };
    next.balances.push(balance("left", &base, 1));
    next.balances.push(balance("right", &base, 50));
    next.balances.push(balance("left", &quote, 15));
    next.balances.push(balance("right", &quote, 4));
    let before_base = sum_of(&next.balances, &base);
    let before_quote = sum_of(&next.balances, &quote);
    let execution = simulate(&program, "swap", &next).unwrap();
    replay(&program, "swap", &next, &execution).unwrap();
    assert_eq!(sum_of(&execution.post.balances, &base), before_base);
    assert_eq!(sum_of(&execution.post.balances, &quote), before_quote);
    assert_eq!(before_base, 91);
    assert_eq!(before_quote, 49);
    assert_eq!(state_at(&execution.post, "gross"), 19);
    assert_eq!(state_at(&execution.post, "fees"), 5);
    assert_eq!(state_at(&execution.post, "net"), 9);
    assert_eq!(state_at(&execution.post, "duty"), 2);
    assert_eq!(amount_at(&execution.post.balances, &base, "left"), 43);
    assert_eq!(amount_at(&execution.post.balances, &base, "right"), 8);
    assert_eq!(amount_at(&execution.post.balances, &base, "alice"), 12);
    assert_eq!(amount_at(&execution.post.balances, &quote, "pool"), 5);
    assert_eq!(amount_at(&execution.post.balances, &quote, "left"), 7);
    let clear_duty = execution
        .post
        .liabilities
        .iter()
        .find(|item| item.name == "clearDuty")
        .unwrap();
    let swap_duty = execution
        .post
        .liabilities
        .iter()
        .find(|item| item.name == "swapDuty")
        .unwrap();
    assert_eq!(clear_duty.amount, UInt(2));
    assert_eq!(swap_duty.amount, UInt(0));
    assert_eq!(clear_duty.debtor, "maker");
    assert_eq!(swap_duty.debtor, "left");

    next.inputs.insert("grossCap".into(), uint(18));
    let original = next.clone();
    let err = simulate(&program, "swap", &next).unwrap_err();
    assert_eq!(err.code, "mori::core::requires");
    assert_eq!(err.step, Some(16));
    assert_eq!(next, original);
}

#[test]
fn replay_rejects_a_dropped_effect() {
    let program = program();
    let scenario = Clear::default().scenario(&program);
    let mut execution = simulate(&program, "clear", &scenario).unwrap();
    execution.effects.pop();
    assert_eq!(
        replay(&program, "clear", &scenario, &execution)
            .unwrap_err()
            .code,
        "mori::core::replay"
    );
}
