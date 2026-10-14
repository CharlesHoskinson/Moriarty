//! Real source evaluation; documents, recipient tags and time are local premises.
//! These regressions do not authenticate a ledger predecessor or prove finality.
use mori_core::*;
use serde_json::Value as Json;

const SOURCE: &str = include_str!("../../../examples/core/escrow-recovery/escrow.mori");
const MANIFEST: &str = include_str!("../../../examples/core/escrow-recovery/mori-tests.json");
fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
fn fixture(action: &str) -> Scenario {
    let text = match action {
        "fund" => include_str!("../../../examples/core/escrow-recovery/fund.json"),
        "partial" => include_str!("../../../examples/core/escrow-recovery/partial.json"),
        "complete" => include_str!("../../../examples/core/escrow-recovery/complete.json"),
        "recover" => include_str!("../../../examples/core/escrow-recovery/recover.json"),
        _ => panic!("unknown fixture"),
    };
    serde_json::from_value(json::parse(text).unwrap()).unwrap()
}
fn continuation(post: &Snapshot, action: &str) -> Scenario {
    Scenario {
        state: post.state.clone(),
        balances: post.balances.clone(),
        liabilities: post.liabilities.clone(),
        inputs: fixture(action).inputs,
    }
}
fn debt(snapshot: &Snapshot, name: &str) -> u128 {
    snapshot
        .liabilities
        .iter()
        .find(|l| l.name == name)
        .unwrap()
        .amount
        .0
}
fn balance(snapshot: &Snapshot, account: &str, reference: &str) -> u128 {
    snapshot
        .balances
        .iter()
        .find(|b| b.account == account && b.asset.reference == reference)
        .unwrap()
        .amount
        .0
}
fn refusal(program: &Program, action: &str, scenario: &Scenario, code: &str) {
    let before = scenario.clone();
    let error = match simulate(program, action, scenario) {
        Err(error) => error,
        Ok(_) => panic!("{action} accepted a scenario expected to refuse with {code}"),
    };
    assert_eq!(error.code, code, "{action}: {error}");
    assert_eq!(*scenario, before, "refusal changed caller snapshot");
}
fn exact_manifest_case(action: &str, execution: &Execution) {
    let manifest: Json = json::parse(MANIFEST).unwrap();
    let expected = &manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["action"] == action)
        .unwrap()["expected"];
    let post: Snapshot = serde_json::from_value(expected["post"].clone()).unwrap();
    let effects: Vec<Effect> = serde_json::from_value(expected["effects"].clone()).unwrap();
    assert_eq!(execution.post, post);
    assert_eq!(execution.effects, effects);
}

#[test]
fn existing_source_consumer_matches_independent_complete_fixture_expectations() {
    let program = parse(SOURCE).unwrap();
    assert_eq!(program.actions.len(), 4);
    for action in ["fund", "partial", "complete", "recover"] {
        let scenario = fixture(action);
        let before = scenario.clone();
        let execution = simulate(&program, action, &scenario).unwrap();
        exact_manifest_case(action, &execution);
        replay(&program, action, &scenario, &execution).unwrap();
        assert_eq!(scenario, before);
        for reference in ["PAY", "GOODS"] {
            let total = |rows: &[Balance]| {
                rows.iter()
                    .filter(|b| b.asset.reference == reference)
                    .map(|b| b.amount.0)
                    .sum::<u128>()
            };
            assert_eq!(
                total(&execution.pre.balances),
                total(&execution.post.balances)
            );
        }
    }
}

#[test]
fn funded_partial_continuation_keeps_both_residual_duties_and_exact_economics() {
    let program = parse(SOURCE).unwrap();
    let funded = simulate(&program, "fund", &fixture("fund")).unwrap();
    let mut partial = continuation(&funded.post, "partial");
    partial.inputs.insert("units".into(), uint(2));
    let first = simulate(&program, "partial", &partial).unwrap();
    assert_eq!(debt(&first.post, "delivery"), 8);
    assert_eq!(debt(&first.post, "refund"), 16);
    let mut second = continuation(&first.post, "partial");
    second.inputs.insert("units".into(), uint(1));
    let second = simulate(&program, "partial", &second).unwrap();
    exact_manifest_case(
        "partial",
        &simulate(&program, "partial", &fixture("partial")).unwrap(),
    );
    assert_eq!(
        second.post,
        simulate(&program, "partial", &fixture("partial"))
            .unwrap()
            .post
    );
    let complete = simulate(
        &program,
        "complete",
        &continuation(&second.post, "complete"),
    )
    .unwrap();
    assert_eq!(complete.post.state["phase"], uint(3));
    assert_eq!(complete.post.state["terminal"], Value::Bool(true));
    assert_eq!(debt(&complete.post, "delivery"), 0);
    assert_eq!(debt(&complete.post, "refund"), 0);
    assert_eq!(balance(&complete.post, "buyer", "GOODS"), 10);
    assert_eq!(balance(&complete.post, "seller", "PAY"), 20);
    assert_eq!(balance(&complete.post, "treasury", "PAY"), 1);
}

#[test]
fn recovery_retains_unfulfilled_delivery_and_gross_debit_despite_refund() {
    let program = parse(SOURCE).unwrap();
    for prior_partial in [false, true] {
        let funded = simulate(&program, "fund", &fixture("fund")).unwrap();
        let opening = if prior_partial {
            simulate(&program, "partial", &continuation(&funded.post, "partial"))
                .unwrap()
                .post
        } else {
            funded.post
        };
        let recovered = simulate(&program, "recover", &continuation(&opening, "recover")).unwrap();
        let residual = if prior_partial { 7 } else { 10 };
        assert_eq!(debt(&recovered.post, "delivery"), residual);
        assert_eq!(debt(&recovered.post, "refund"), 0);
        assert_eq!(recovered.post.state["grossPayment"], uint(21));
        assert_eq!(recovered.post.state["refunded"], uint(residual * 2));
        assert_eq!(
            balance(&recovered.post, "buyer", "PAY"),
            if prior_partial { 93 } else { 99 }
        );
        assert_eq!(balance(&recovered.post, "seller", "GOODS"), residual);
        assert_eq!(recovered.post.state["terminal"], Value::Bool(true));
        assert_eq!(recovered.post.state["phase"], uint(4));
    }
}

#[test]
fn completion_recovery_race_in_both_orders_and_double_terminal_refuse() {
    let program = parse(SOURCE).unwrap();
    for first in ["complete", "recover"] {
        let terminal = simulate(&program, first, &fixture(first)).unwrap().post;
        for second in ["partial", "complete", "recover", "fund"] {
            // Even an old success receipt and pre-deadline time cannot revive a
            // locally committed recovery tombstone. Authentic ordering is open.
            let scenario = continuation(&terminal, second);
            refusal(&program, second, &scenario, "mori::core::requires");
        }
    }
}

#[test]
fn supplied_receipt_recipient_clock_phase_and_amount_controls_refuse_atomically() {
    let program = parse(SOURCE).unwrap();
    for action in ["partial", "complete", "recover"] {
        let original = fixture(action);
        let document = if action == "recover" {
            "unusedDocument"
        } else {
            "successDocument"
        };
        let mut bad = original.clone();
        bad.inputs.insert(document.into(), Value::Bool(false));
        refusal(&program, action, &bad, "mori::core::requires");
        let mut bad = original.clone();
        bad.inputs.insert("recipient".into(), uint(2));
        refusal(&program, action, &bad, "mori::core::requires");
        let mut bad = original.clone();
        bad.inputs.insert(
            "now".into(),
            uint(if action == "recover" { 100 } else { 101 }),
        );
        refusal(&program, action, &bad, "mori::core::requires");
        let mut bad = original.clone();
        bad.state.insert("phase".into(), uint(0));
        refusal(&program, action, &bad, "mori::core::requires");
        let mut bad = original.clone();
        bad.state.insert("remaining".into(), uint(6));
        refusal(&program, action, &bad, "mori::core::requires");
    }
    for units in [0, 10, 11, u128::MAX] {
        let mut bad = fixture("partial");
        bad.inputs.insert("units".into(), uint(units));
        refusal(&program, "partial", &bad, "mori::core::requires");
    }
    let mut before_funding = fixture("complete");
    before_funding.inputs.insert("now".into(), uint(9));
    refusal(
        &program,
        "complete",
        &before_funding,
        "mori::core::requires",
    );
}

#[test]
fn erased_underfunded_and_identity_substituted_duties_refuse_without_effects() {
    let program = parse(SOURCE).unwrap();
    let mut identity_failures = Vec::new();
    for action in ["partial", "complete", "recover"] {
        let original = fixture(action);
        let mut missing = original.clone();
        missing.liabilities.retain(|l| l.name != "delivery");
        refusal(&program, action, &missing, "mori::core::liability");
        for identity_member in ["asset", "debtor", "creditor"] {
            let mut changed = original.clone();
            let delivery = changed
                .liabilities
                .iter_mut()
                .find(|l| l.name == "delivery")
                .unwrap();
            match identity_member {
                "asset" => delivery.asset = program.assets[0].identity.clone(),
                "debtor" => delivery.debtor = "other-seller".into(),
                "creditor" => delivery.creditor = "other-buyer".into(),
                _ => unreachable!(),
            }
            let before = changed.clone();
            match simulate(&program, action, &changed) {
                Err(error) if error.code == "mori::core::liability" => {}
                Err(error) => {
                    identity_failures.push(format!("{action}/{identity_member}: {}", error.code))
                }
                Ok(_) => {
                    identity_failures.push(format!("{action}/{identity_member}: wrongly accepted"))
                }
            }
            assert_eq!(changed, before);
        }
    }
    for action in ["partial", "complete"] {
        let mut bad = fixture(action);
        bad.liabilities
            .iter_mut()
            .find(|l| l.name == "delivery")
            .unwrap()
            .amount = UInt(1);
        refusal(&program, action, &bad, "mori::core::liability");
    }
    assert!(
        identity_failures.is_empty(),
        "{}",
        identity_failures.join("; ")
    );
}

#[test]
fn held_out_funding_and_partial_amounts_use_the_same_source_consumers() {
    let program = parse(SOURCE).unwrap();
    for (units, price) in [(2, 1), (17, 7), (1000, 11)] {
        let principal = units * price;
        let mut fund = fixture("fund");
        fund.inputs.insert("units".into(), uint(units));
        fund.inputs.insert("unitPrice".into(), uint(price));
        fund.inputs.insert("grossCap".into(), uint(principal + 1));
        for row in &mut fund.balances {
            if row.account == "buyer" && row.asset.reference == "PAY" {
                row.amount = UInt(principal + 9);
            }
            if row.account == "seller" && row.asset.reference == "GOODS" {
                row.amount = UInt(units);
            }
        }
        let funded = simulate(&program, "fund", &fund).unwrap();
        assert_eq!(debt(&funded.post, "delivery"), units);
        assert_eq!(debt(&funded.post, "refund"), principal);
        let mut partial = continuation(&funded.post, "partial");
        partial.inputs.insert("units".into(), uint(1));
        let progress = simulate(&program, "partial", &partial).unwrap();
        assert_eq!(debt(&progress.post, "delivery"), units - 1);
        assert_eq!(debt(&progress.post, "refund"), (units - 1) * price);
        assert_eq!(progress.post.state["paid"], uint(price));
        let completed = simulate(
            &program,
            "complete",
            &continuation(&progress.post, "complete"),
        )
        .unwrap();
        assert_eq!(debt(&completed.post, "delivery"), 0);
        assert_eq!(completed.post.state["paid"], uint(principal));
    }
}

#[test]
fn funding_caps_checked_arithmetic_and_late_financial_failures_preserve_inputs() {
    let program = parse(SOURCE).unwrap();
    let mut capped = fixture("fund");
    capped.inputs.insert("grossCap".into(), uint(20));
    refusal(&program, "fund", &capped, "mori::core::requires");
    let mut overflow = fixture("fund");
    overflow.inputs.insert("unitPrice".into(), uint(u128::MAX));
    refusal(&program, "fund", &overflow, "mori::core::overflow");
    let mut insufficient = fixture("fund");
    insufficient
        .balances
        .iter_mut()
        .find(|b| b.account == "seller" && b.asset.reference == "GOODS")
        .unwrap()
        .amount = UInt(9);
    refusal(&program, "fund", &insufficient, "mori::core::balance");
    let mut refund_overflow = fixture("recover");
    refund_overflow
        .balances
        .iter_mut()
        .find(|b| b.account == "buyer" && b.asset.reference == "PAY")
        .unwrap()
        .amount = UInt(u128::MAX);
    refusal(
        &program,
        "recover",
        &refund_overflow,
        "mori::core::overflow",
    );
}

#[test]
fn excess_duty_is_retained_and_exposes_the_current_source_correspondence_gap() {
    // Core has no liability-amount expression. This test records the actual
    // limitation rather than treating supplied state as authenticated evidence.
    let program = parse(SOURCE).unwrap();
    let mut scenario = fixture("complete");
    scenario
        .liabilities
        .iter_mut()
        .find(|l| l.name == "delivery")
        .unwrap()
        .amount = UInt(8);
    let execution = simulate(&program, "complete", &scenario).unwrap();
    assert_eq!(execution.post.state["remaining"], uint(0));
    assert_eq!(debt(&execution.post, "delivery"), 1);
}
