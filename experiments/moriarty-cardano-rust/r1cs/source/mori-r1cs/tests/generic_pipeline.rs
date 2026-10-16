//! The falsifiable source predicate: one compiler (source -> actual Core
//! parse/validate -> typed Program -> plan -> constraint lowering), several
//! contrasting programs, every accepted witness checked against frozen expectations.
//! There are two public entry paths: `compile_source` for source text, and
//! `admit_program` + `compile` for an in-memory typed Program (review F-11). Both run
//! Core `validate` (and `require_core1`) before planning; these tests use the first.
mod common;

use common::*;
use mori_core::{Balance, Effect, Liability, Scenario, Snapshot, UInt};
use mori_r1cs::*;
use std::collections::BTreeMap;

/// (fixture, action, frozen public input count with PublicCap, frozen W).
/// Counts: [AUTHOR] shape_expectations; W: [ORACLE] expected.cost.used.
const INVOICE_CASES: &[(&str, &str, usize, u128)] =
    &[("invoice-open", "open", 70, 38), ("invoice-partial", "partial", 62, 28), ("invoice-finish", "finish", 56, 20)];
const RESERVE_CASES: &[(&str, &str, usize, u128)] =
    &[("reserve-allocate", "allocate", 89, 57), ("reserve-rebate4", "rebate", 56, 26), ("reserve-rebate2", "rebate", 56, 26)];

fn lifecycle(source: &str, cases: &[(&str, &str, usize, u128)]) {
    for &(case, action, fields, work) in cases {
        let rel = compile_ok(source, options(action, CapBinding::PublicCap));
        let oracle = oracle_case(case);
        assert_eq!(oracle.used, UInt(work), "{case}: frozen oracle work");
        assert_eq!(rel.plan().logical_work, UInt(work), "{case}: W from the IR");
        assert_eq!(rel.plan().logical_schedule, oracle.schedule, "{case}: per-statement schedule");
        assert_eq!(rel.layout().field_count, fields, "{case}: statement width");
        assert_eq!(rel.adapter().public_input_count(rel.layout()), Ok(fields));
        let values = oracle_values(&rel, &oracle, Some(oracle.cap));
        assert!(accepts(&rel, values), "{case}: oracle transition must satisfy the compiled relation");
    }
}

#[test]
fn installment_invoice_lifecycle_satisfies_the_compiled_relations() {
    lifecycle(INVOICE, INVOICE_CASES);
}

#[test]
fn conditional_reserve_lifecycle_satisfies_the_compiled_relations() {
    lifecycle(RESERVE, RESERVE_CASES);
}

#[test]
fn frontend_yields_the_frozen_typed_programs_and_their_canonical_hashes() {
    let addendum = json(ADDENDUM);
    let hashes = &addendum["additional_expectations"]["program_hashes"];
    for (source, program, key, action) in [
        (INVOICE, INVOICE_PROGRAM, "installment-invoice.mori", "open"),
        (RESERVE, RESERVE_PROGRAM, "conditional-reserve.mori", "allocate"),
    ] {
        let rel = compile_ok(source, options(action, CapBinding::NotBound));
        // [ORACLE] complete expected typed Program.
        assert_eq!(serde_json::to_value(rel.program()).unwrap(), json(program), "{key}: typed Program");
        // [AUTHOR, derived from ORACLE JSON] canonical Program hash.
        let expected = hashes[key].as_str().unwrap();
        assert_eq!(rel.identity().program_hash.hex(), expected);
        assert_eq!(rel.layout().header.program_hash.hex(), expected);
        // Raw source identity is recorded and differs from the Program identity.
        let source_hash = rel.identity().source_sha256.expect("admitted from source");
        assert_ne!(source_hash, rel.identity().program_hash);
    }
}

#[test]
fn contrasting_programs_get_distinct_bound_identities_from_the_same_compiler() {
    let a = compile_ok(INVOICE, options("open", CapBinding::PublicCap));
    let b = compile_ok(RESERVE, options("allocate", CapBinding::PublicCap));
    let c = compile_ok(PAYROLL, options("pay", CapBinding::PublicCap));
    let ids = [a.identity(), b.identity(), c.identity()];
    for (i, x) in ids.iter().enumerate() {
        for y in &ids[i + 1..] {
            assert_ne!(x.program_hash, y.program_hash);
            assert_ne!(x.shape_digest, y.shape_digest);
            assert_ne!(x.digest().unwrap(), y.digest().unwrap());
        }
        // Same profile, version and compiler build for every relation of this build.
        assert_eq!(x.profile_digest, ids[0].profile_digest);
        assert_eq!(x.compiler_build_digest, ids[0].compiler_build_digest);
    }
    // A claim for one program is not even encodable for another relation.
    let open = oracle_case("invoice-open");
    assert!(witness::from_execution(&b, &open.scenario, &open.execution, Some(open.cap)).is_err());
}

fn usd() -> mori_core::AssetId {
    mori_core::AssetId { network: "fresh-unit".into(), issuer: "payroll-issuer".into(), reference: "usd".into(), scale: 0 }
}

fn bal(account: &str, amount: u128) -> Balance {
    Balance { account: account.into(), asset: usd(), amount: UInt(amount) }
}

fn benefit(amount: u128) -> Liability {
    Liability { name: "benefit".into(), asset: usd(), debtor: "employer".into(), creditor: "worker".into(), amount: UInt(amount) }
}

/// [AUTHOR] P1 is stored in Core JSON; its scenario, post and effects map directly.
fn p1() -> (Scenario, Snapshot, Vec<Effect>) {
    let v = vector("P1");
    let scenario: Scenario = serde_json::from_value(v["scenario"].clone()).unwrap();
    let post: Snapshot = serde_json::from_value(v["expected"]["post"].clone()).unwrap();
    let effects: Vec<Effect> = serde_json::from_value(v["expected"]["effects"].clone()).unwrap();
    (scenario, post, effects)
}

#[test]
fn fresh_payroll_program_pay_and_settle_satisfy_author_expectations() {
    // pay: P1 with cap 45, W 45, 68 public inputs.
    let pay = compile_ok(PAYROLL, options("pay", CapBinding::PublicCap));
    assert_eq!(pay.plan().logical_work, UInt(45));
    let schedule: Vec<UInt> = [7, 4, 6, 2, 2, 2, 2, 4, 4, 2, 4, 6].into_iter().map(UInt).collect();
    assert_eq!(pay.plan().logical_schedule, schedule);
    assert_eq!(pay.layout().field_count, 68);
    let (scenario, post, effects) = p1();
    assert!(accepts(&pay, record_values(&pay, &scenario, &post, &effects, Some(UInt(45)))));

    // P5: same inputs except ceiling 0; Or evaluates both operands, Le false, bonus true.
    let mut p5 = scenario.clone();
    p5.inputs.insert("ceiling".into(), u(0));
    assert!(accepts(&pay, record_values(&pay, &p5, &post, &effects, Some(UInt(45)))));

    // settle from P1.post. P2 (amount 7) and P3 (amount 3), transcribed from the
    // compact frozen form into Core records.
    let settle = compile_ok(PAYROLL, options("settle", CapBinding::PublicCap));
    assert_eq!(settle.plan().logical_work, UInt(12));
    assert_eq!(settle.layout().field_count, 49);
    for (amount, employer, worker, closing, closed) in [(7u128, 71u128, 32u128, 0u128, true), (3, 75, 28, 4, false)] {
        let mut inputs = BTreeMap::new();
        inputs.insert("amount".to_string(), u(amount));
        let from = Scenario { state: post.state.clone(), inputs, balances: post.balances.clone(), liabilities: post.liabilities.clone() };
        let mut state = post.state.clone();
        state.insert("closed".into(), b(closed));
        let after = Snapshot {
            state,
            balances: vec![bal("auditor", 9), bal("employer", employer), bal("payroll", 1), bal("worker", worker)],
            liabilities: vec![benefit(closing)],
        };
        let effects = vec![
            Effect::Liability { name: "benefit".into(), asset: usd(), debtor: "employer".into(), creditor: "worker".into(), before: UInt(7), after: UInt(closing) },
            Effect::Transfer { asset: usd(), from: "employer".into(), to: "worker".into(), amount: UInt(amount) },
            Effect::State { name: "closed".into(), before: b(false), after: b(closed) },
            Effect::Emit { name: "closed".into(), value: b(closed) },
        ];
        assert!(accepts(&settle, record_values(&settle, &from, &after, &effects, Some(UInt(12)))), "settle({amount})");
    }
}

#[test]
fn compile_source_is_deterministic_and_never_reads_program_names() {
    // The same source compiled twice yields byte-identical layouts and identities; a
    // renamed agreement (different Program) yields a different program hash. This does
    // not claim `compile_source` is the only public path (see the module docs, F-11).
    let a = compile_ok(SELF_MOVE, options("move", CapBinding::NotBound));
    let again = compile_ok(SELF_MOVE, options("move", CapBinding::NotBound));
    assert_eq!(a.layout(), again.layout());
    assert_eq!(a.identity(), again.identity());
    assert_eq!(a.layout().field_count, 20); // [AUTHOR] ADDENDUM-1 without cap
    let renamed = SELF_MOVE.replace("agreement SelfMove", "agreement SelfMoveRenamed");
    let r = compile_ok(&renamed, options("move", CapBinding::NotBound));
    assert_ne!(a.identity().program_hash, r.identity().program_hash);
    assert_eq!(a.plan().effects, r.plan().effects);
}
