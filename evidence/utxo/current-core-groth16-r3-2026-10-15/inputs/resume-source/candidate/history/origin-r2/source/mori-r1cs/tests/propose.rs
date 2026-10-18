//! Witness proposal through the actual Core reference, compared against frozen
//! [ORACLE] values. The reference is a proposer only: its refusal yields no
//! assignment, and its success is still checked by the constraint system.
mod common;

use common::*;
use mori_core::UInt;
use mori_r1cs::*;

#[test]
fn reference_proposals_equal_the_frozen_oracle_and_satisfy_the_relation() {
    for (source, case, action) in [
        (INVOICE, "invoice-open", "open"),
        (INVOICE, "invoice-partial", "partial"),
        (INVOICE, "invoice-finish", "finish"),
        (RESERVE, "reserve-allocate", "allocate"),
        (RESERVE, "reserve-rebate4", "rebate"),
        (RESERVE, "reserve-rebate2", "rebate"),
    ] {
        let rel = compile(source, options(action, CapBinding::PublicCap));
        let oracle = oracle_case(case);
        let proposed = witness::propose(&rel, &oracle.scenario, Some(oracle.cap)).expect("reference succeeds");
        assert_eq!(proposed, oracle_values(&rel, &oracle, Some(oracle.cap)), "{case}: reference vs oracle");
        assert!(accepts(&rel, proposed), "{case}");
    }
}

#[test]
fn a_reference_refusal_proposes_nothing() {
    // P4 [AUTHOR]: 21 <= 20 is false and bonus is false: `requires` refuses at step 2.
    let pay = compile(PAYROLL, options("pay", CapBinding::PublicCap));
    let mut scenario: mori_core::Scenario = serde_json::from_value(vector("P1")["scenario"].clone()).unwrap();
    scenario.inputs.insert("bonus".into(), b(false));
    scenario.inputs.insert("ceiling".into(), u(20));
    match witness::propose(&pay, &scenario, Some(UInt(45))) {
        Err(MappingError::ReferenceRefused(e)) => {
            assert_eq!(e.code, "mori::core::requires");
            assert_eq!(e.step, Some(2));
        }
        other => panic!("expected a reference refusal, got {other:?}"),
    }
    // P10: cap 44 < W = 45 refuses under the metered reference too.
    let scenario: mori_core::Scenario = serde_json::from_value(vector("P1")["scenario"].clone()).unwrap();
    match witness::propose(&pay, &scenario, Some(UInt(44))) {
        Err(MappingError::ReferenceRefused(e)) => assert_eq!(e.code, "mori::core::meter"),
        other => panic!("expected a meter refusal, got {other:?}"),
    }
}

#[test]
fn a_reference_success_is_not_trusted() {
    // Take the honest proposal and corrupt one claimed effect: the relation refuses,
    // although the corrupted claim came out of the same proposal path.
    let rel = compile(INVOICE, options("open", CapBinding::PublicCap));
    let oracle = oracle_case("invoice-open");
    let mut proposed = witness::propose(&rel, &oracle.scenario, Some(oracle.cap)).unwrap();
    proposed.effects[0] = EffectValues::Amount(UInt(103));
    assert!(!accepts(&rel, proposed));
}
