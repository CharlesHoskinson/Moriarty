//! In-circuit gross debit and fees per debit cell (AccountingBinding::GrossDebitAndFees,
//! UNADOPTED). Rows G1-G7 and G-T1 are frozen in ADDENDUM-1; G1-G6 values are
//! transcribed from the [ORACLE] prose, G7 from the [AUTHOR] P1 derivation.
mod common;

use common::*;
use mori_core::{Effect, Scenario, Snapshot, UInt};
use mori_r1cs::statement::accounting_summary;
use mori_r1cs::*;

fn with_accounting(source: &str, action: &str) -> CompiledRelation {
    let mut opts = options(action, CapBinding::PublicCap);
    opts.accounting = AccountingBinding::GrossDebitAndFees;
    compile_ok(source, opts)
}

fn check(rel: &CompiledRelation, values: TransitionValues, cells: &[(&str, &str)], gross: &[u128], fees: &[u128]) {
    let plan = rel.plan();
    let debit: Vec<(String, String)> = plan.debit_cells.iter().map(|&c| (plan.cells[c].asset_name.clone(), plan.cells[c].account.clone())).collect();
    let expected: Vec<(String, String)> = cells.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
    assert_eq!(debit, expected);
    let summary = accounting_summary(plan, &values);
    for (i, (_, g, f)) in summary.iter().enumerate() {
        assert_eq!(*g, Wide::from_u128(gross[i]), "gross {:?}", expected[i]);
        assert_eq!(*f, Wide::from_u128(fees[i]), "fees {:?}", expected[i]);
    }
    assert!(accepts(rel, values));
}

#[test]
fn oracle_lifecycles_expose_exact_gross_debit_and_fees() {
    let rows: &[(&str, &str, &str, &[(&str, &str)], &[u128], &[u128])] = &[
        (INVOICE, "open", "invoice-open", &[("CASH", "buyer")], &[107], &[3]),
        (INVOICE, "partial", "invoice-partial", &[("CASH", "escrow")], &[37], &[2]),
        (INVOICE, "finish", "invoice-finish", &[("CASH", "escrow")], &[55], &[0]),
        (RESERVE, "allocate", "reserve-allocate", &[("A", "vault"), ("B", "vault")], &[23, 4], &[2, 0]),
        (RESERVE, "rebate", "reserve-rebate4", &[("B", "vault")], &[4], &[0]),
        (RESERVE, "rebate", "reserve-rebate2", &[("B", "vault")], &[2], &[0]),
    ];
    for &(source, action, case, cells, gross, fees) in rows {
        let rel = with_accounting(source, action);
        let oracle = oracle_case(case);
        check(&rel, oracle_values(&rel, &oracle, Some(oracle.cap)), cells, gross, fees);
    }
}

#[test]
fn self_transfer_counts_toward_gross_debit() {
    // G7: employer 22 (21 + fee 1), worker 21 (self-transfer).
    let rel = with_accounting(PAYROLL, "pay");
    let v = vector("P1");
    let scenario: Scenario = serde_json::from_value(v["scenario"].clone()).unwrap();
    let post: Snapshot = serde_json::from_value(v["expected"]["post"].clone()).unwrap();
    let effects: Vec<Effect> = serde_json::from_value(v["expected"]["effects"].clone()).unwrap();
    let values = record_values(&rel, &scenario, &post, &effects, Some(UInt(45)));
    check(&rel, values, &[("USD", "employer"), ("USD", "worker")], &[22, 21], &[1, 0]);
}

#[test]
fn summary_limbs_are_bound_and_canonical() {
    // G-T1 on G1.
    let rel = with_accounting(INVOICE, "open");
    let oracle = oracle_case("invoice-open");
    let values = oracle_values(&rel, &oracle, Some(oracle.cap));
    let gross = offset(&rel, "accounting.CASH.buyer.gross_debit");
    let fees = offset(&rel, "accounting.CASH.buyer.fees_paid");
    assert!(accepts(&rel, values.clone()));
    assert!(!accepts_public(&rel, values.clone(), |p| p[gross] += fr(1)));
    assert!(!accepts_public(&rel, values.clone(), |p| p[fees] += fr(1)));
    assert!(!accepts_public(&rel, values, |p| p[gross + 2] = fr(1 << 16)));
}
