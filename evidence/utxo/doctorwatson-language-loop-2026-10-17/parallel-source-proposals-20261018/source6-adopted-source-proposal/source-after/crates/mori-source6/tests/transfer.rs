#![cfg(feature = "source6-transfer-slice")]

use mori_source6::{
    Allowance, Balance, LocalTransferScenario, Replay, expand_transfer,
};

fn invoice_scenario() -> LocalTransferScenario<'static> {
    LocalTransferScenario {
        domain: "Midnight",
        asset: "A",
        head: "h0",
        predecessor: "genesis",
        round: 1,
        balances: [
            Balance { account: "Owner", amount: 10_000 },
            Balance { account: "Recipient", amount: 0 },
            Balance { account: "Fee", amount: 0 },
        ],
        allowance: Allowance { owner: "Owner", remaining: 10_000, spent: 0 },
        replay: Replay::Unused,
        work_remaining: 10,
        work_spent: 0,
        post_head: "h1",
    }
}

#[test]
fn literal_decimal_transfer_has_complete_source6_presentation() {
    let source = include_str!("fixtures/invoice.mori");
    let expansion = expand_transfer(source, "pay", &invoice_scenario()).unwrap();
    assert_eq!(expansion.qualification, "local-stipulation-only");
    assert_eq!(expansion.source6, include_str!("fixtures/invoice.source6"));
    insta::with_settings!({
        description => format!("Code:\n\n{source}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!("invoice_source6", expansion.source6);
    });
}

#[test]
fn referenced_quantity_sum_and_fee_use_exact_atomic_values() {
    let source = include_str!("fixtures/workshop.mori");
    let scenario = LocalTransferScenario {
        head: "s0",
        predecessor: "earlier",
        round: 3,
        balances: [
            Balance { account: "Owner", amount: 1000 },
            Balance { account: "Recipient", amount: 5 },
            Balance { account: "Fee", amount: 2 },
        ],
        allowance: Allowance { owner: "Owner", remaining: 900, spent: 100 },
        work_remaining: 9,
        work_spent: 4,
        post_head: "s1",
        ..invoice_scenario()
    };
    let expansion = expand_transfer(source, "pay", &scenario).unwrap();
    assert_eq!(expansion.qualification, "local-stipulation-only");
    assert_eq!(expansion.source6, include_str!("fixtures/workshop.source6"));
    insta::with_settings!({
        description => format!("Code:\n\n{source}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!("workshop_source6", expansion.source6);
    });
}
