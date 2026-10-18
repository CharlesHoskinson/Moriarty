//! P3S1-R1 `SettlementHashV2` generality tests (review RR-05 / RP-4). Source only: never
//! compiled or run; every assertion is a prediction.
//!
//! Expectation sources, by test:
//! - `v2r1_*_matches_fixtures`: `rust/expectations/RUST-ABI2-R1-EXPECTATIONS.json`, the
//!   R1 author fixtures (AUTH). Their Core effects are the frozen non-author Sol `/1`
//!   vectors (P01 open, P02 partial, P03 finish, P04 allocate, P05 rebate); bodies,
//!   statements and public inputs are the R1 author's derivation. Equality is
//!   cross-language agreement, not an independent oracle.
//! - `v2r1_heldout_h1_settle_*`: the frozen non-author held-out vector H1-P1
//!   (`rust/fixtures/heldout/HELDOUT-VECTORS.json`, sha256 b72cec5a...2f1f, reviewer
//!   claude-opus-5-5 of the compiler lineage, frozen before reading the compiler). Its
//!   pre/post/effects and accounting are used unchanged; the V2 context is synthetic.
//! - `v2r1_wide_carry_*`: `rust/EXPECTED-PACKET-R1/WIDE-CARRY.json`, an R1-author (AUTH)
//!   specimen for a gross debit of exactly 2^128.
mod common;

use ark_bls12_381::Fr;
use ark_relations::r1cs::ConstraintSystem;
use common::{INVOICE, RESERVE, build};
use mori_core::{Effect, Scenario, Snapshot, UInt, Value};
use mori_r1cs::*;
use serde_json::Value as Json;
use std::collections::BTreeMap;

const R1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../expectations/RUST-ABI2-R1-EXPECTATIONS.json"));
const HELDOUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/heldout/HELDOUT-VECTORS.json"));
const H1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/heldout/H1-settle.mori"));
const WIDE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../EXPECTED-PACKET-R1/wide-carry.mori"));
const WIDE_EXPECT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../EXPECTED-PACKET-R1/WIDE-CARRY.json"));

fn json(text: &str) -> Json {
    serde_json::from_str(text).expect("frozen JSON parses")
}

fn unhex(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "even hex");
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex")).collect()
}

fn arr<const N: usize>(s: &str) -> [u8; N] {
    unhex(s).try_into().expect("length")
}

fn v2_options(action: &str) -> CompileOptions {
    CompileOptions {
        action: action.into(),
        cap: CapBinding::NotBound,
        frame: FrameCells::TouchedOnly,
        accounting: AccountingBinding::GrossDebitAndFees,
        compiler: build(),
    }
}

fn compile_v2(source: &str, action: &str) -> CompiledRelation {
    compile_source::<Fr>(source, v2_options(action), StatementAdapter::SettlementHashV2).unwrap_or_else(|e| panic!("v2 compile refused: {e}"))
}

fn context(c: &Json) -> ContextV2 {
    ContextV2 {
        vk_digest: arr(c["vkDigestSynthetic"].as_str().unwrap()),
        claimant_vkh: arr(c["claimantVkh"].as_str().unwrap()),
        network_id: c["networkId"].as_u64().unwrap() as u8,
        settle_hash: arr(c["settleHash"].as_str().unwrap()),
        thread_id: arr(c["threadId"].as_str().unwrap()),
        consumed_txid: arr(c["consumedTxid"].as_str().unwrap()),
        consumed_ix: c["consumedIx"].as_u64().unwrap() as u16,
        seq: c["seq"].as_u64().unwrap(),
        catalogue_digest: arr(c["catalogueDigest"].as_str().unwrap()),
        intents_digest: arr(c["intentsDigest"].as_str().unwrap()),
    }
}

/// A synthetic V2 context (labelled; held-out and wide-carry tests only).
fn synthetic_context() -> ContextV2 {
    ContextV2 {
        vk_digest: [0x11; 32],
        claimant_vkh: [0x55; 28],
        network_id: 0,
        settle_hash: [0xa2; 28],
        thread_id: [0x33; 32],
        consumed_txid: [0x44; 32],
        consumed_ix: 0,
        seq: 0,
        catalogue_digest: [0x66; 32],
        intents_digest: [0x77; 32],
    }
}

fn satisfied(rel: &CompiledRelation, a: Assignment<Fr>, ctx: ContextV2) -> bool {
    let cs = ConstraintSystem::<Fr>::new_ref();
    rel.circuit_v2(Some(a), Some(ctx), Tamper::none()).expect("v2 circuit").synthesize_into(cs.clone()).expect("synthesis");
    cs.is_satisfied().expect("prove-mode system")
}

/// Byte offset of a body slot (header slots omitted, ABI-V2.md section 3).
fn body_offset(rel: &CompiledRelation, kind: SlotKind) -> usize {
    let mut at = 0;
    for s in &rel.layout().slots {
        if s.kind.is_header() {
            continue;
        }
        if s.kind == kind {
            return at;
        }
        at += match s.encoding {
            SlotEncoding::UInt => 16,
            SlotEncoding::Bool => 1,
            SlotEncoding::Wide => 18,
            other => panic!("unexpected body encoding {other:?}"),
        };
    }
    panic!("no body slot {kind:?}")
}

fn source_of(program: &str) -> &'static str {
    match program {
        "invoice" => INVOICE,
        "reserve" => RESERVE,
        other => panic!("no program {other}"),
    }
}

/// Body, statement, public inputs, accounting and constraint satisfaction of one R1
/// fixture case; returns the relation, its values and its body for case-specific checks.
fn check_fixture_case(id: &str) -> (CompiledRelation, TransitionValues, Vec<u8>) {
    let e = json(R1);
    let c = &e["cases"][id];
    let rel = compile_v2(source_of(c["program"].as_str().unwrap()), c["action"].as_str().unwrap());
    let consts = rel.abi2().expect("v2 constants").clone();
    assert_eq!(consts.shape_bytes, unhex(c["shapeBytesHex"].as_str().unwrap()), "{id} shape bytes");
    assert_eq!(consts.shape_digest.to_vec(), unhex(c["shapeDigest"].as_str().unwrap()), "{id} shape digest");
    let scenario: Scenario = serde_json::from_value(c["bridgeScenario"].clone()).expect("bridge scenario is a Core Scenario");
    let values = witness::propose(&rel, &scenario, None).expect("Core reference proposes the bridge transition");
    let body = abi2::body_bytes(rel.layout(), rel.plan(), &values).expect("body");
    assert_eq!(body, unhex(c["bodyHex"].as_str().unwrap()), "{id} body");
    assert_eq!(consts.body_len as usize, body.len(), "{id} body length");
    let summary = statement::accounting_summary(rel.plan(), &values);
    let frozen = c["accounting"].as_array().unwrap();
    assert_eq!(summary.len(), frozen.len(), "{id} debit cells");
    for ((cell, gross, fees), f) in summary.iter().zip(frozen) {
        assert_eq!(*cell as u64, f["cell"].as_u64().unwrap(), "{id} debit cell index");
        assert_eq!(*gross, Wide::from_u128(f["gross"].as_str().unwrap().parse().unwrap()), "{id} gross");
        assert_eq!(*fees, Wide::from_u128(f["fees"].as_str().unwrap().parse().unwrap()), "{id} fees");
    }
    let ctx = context(&c["context"]);
    let fixture_consts = AbiV2Constants {
        predicate_digest: arr(c["context"]["predicateDigestSynthetic"].as_str().unwrap()),
        shape_bytes: consts.shape_bytes.clone(),
        shape_digest: consts.shape_digest,
        body_len: consts.body_len,
    };
    let stmt = abi2::statement_bytes(&fixture_consts, &ctx, &body).expect("statement");
    assert_eq!(stmt, unhex(c["statementHex"].as_str().unwrap()), "{id} statement");
    let x = abi2::public_inputs::<Fr>(&stmt);
    let x1: u128 = c["publicInputs"]["x1"].as_str().unwrap().parse().unwrap();
    let x2: u128 = c["publicInputs"]["x2"].as_str().unwrap().parse().unwrap();
    assert_eq!(x, vec![Fr::from(x1), Fr::from(x2)], "{id} public inputs");
    // The compiled relation hashes its own predicate digest; honest satisfaction is checked
    // in that relation's own statement (the fixture uses a synthetic predicate digest).
    let a = rel.assignment_v2::<Fr>(values.clone(), &ctx).expect("assignment");
    assert!(satisfied(&rel, a, ctx.clone()), "{id} honest assignment satisfies");
    let mut bad = values.clone();
    bad.post.cells[0] = UInt(bad.post.cells[0].0 + 1);
    let a = rel.assignment_v2::<Fr>(bad, &ctx).expect("a dishonest claim still encodes");
    assert!(!satisfied(&rel, a, ctx), "{id} dishonest post cell unsatisfied");
    (rel, values, body)
}

// ## R1 fixtures (Sol /1 effects; R1 author statements)

#[test]
fn v2r1_p01_open_new_duties_matches_fixtures() {
    // New duties: invoice and reserve absent before, present after; invoice accrues twice.
    let (rel, _, body) = check_fixture_case("P01");
    let plan = rel.plan();
    let invoice = plan.duties.iter().position(|d| d.name == "invoice").expect("invoice duty");
    let reserve = plan.duties.iter().position(|d| d.name == "reserve").expect("reserve duty");
    for j in [invoice, reserve] {
        assert_eq!(body[body_offset(&rel, SlotKind::PreDutyPresent { index: j })], 0, "absent before");
        assert_eq!(body[body_offset(&rel, SlotKind::PostDutyPresent { index: j })], 1, "present after");
    }
    let accruals = plan.effects.iter().filter(|e| matches!(e, EffectShape::Liability { duty, .. } if *duty == invoice)).count();
    assert_eq!(accruals, 2, "two accrual statements on one duty");
}

#[test]
fn v2r1_p02_partial_matches_fixtures() {
    check_fixture_case("P02");
}

#[test]
fn v2r1_p03_finish_empty_inputs_matches_fixtures() {
    let (rel, values, _) = check_fixture_case("P03");
    assert!(rel.plan().inputs.is_empty());
    assert!(values.inputs.is_empty());
}

#[test]
fn v2r1_p04_allocate_bool_multi_asset_two_debits_matches_fixtures() {
    let (rel, _, body) = check_fixture_case("P04");
    let plan = rel.plan();
    assert_eq!(plan.inputs[0].name, "approved");
    assert_eq!(body[body_offset(&rel, SlotKind::Input { index: 0 })], 1, "Bool input approved = true is one byte 0x01");
    let assets: std::collections::BTreeSet<_> = plan.debit_cells.iter().map(|&c| plan.cells[c].asset_name.clone()).collect();
    assert_eq!(plan.debit_cells.len(), 2, "two debit cells");
    assert_eq!(assets.len(), 2, "two assets");
}

#[test]
fn v2r1_p05_rebate_matches_fixtures() {
    check_fixture_case("P05");
}

// ## Non-author held-out program (H1-settle, vector H1-P1)

fn heldout(id: &str) -> Json {
    json(HELDOUT)["vectors"].as_array().expect("vectors").iter().find(|v| v["id"] == id).cloned().unwrap_or_else(|| panic!("no vector {id}"))
}

fn h1_values(rel: &CompiledRelation) -> TransitionValues {
    let v = heldout("H1-P1");
    let inputs: BTreeMap<String, Value> = serde_json::from_value(v["scenario"]["inputs"].clone()).expect("inputs");
    let pre: Snapshot = serde_json::from_value(v["expected"]["pre"].clone()).expect("pre");
    let post: Snapshot = serde_json::from_value(v["expected"]["post"].clone()).expect("post");
    let effects: Vec<Effect> = serde_json::from_value(v["expected"]["effects"].clone()).expect("effects");
    witness::from_records(rel.plan(), &inputs, &pre, &post, &effects, None).expect("frozen held-out record maps onto the V2 relation")
}

#[test]
fn v2r1_heldout_h1_settle_accounting_matches_frozen_vector() {
    let rel = compile_v2(H1, "settle");
    let values = h1_values(&rel);
    let v = heldout("H1-P1");
    let frozen = v["expected"]["accounting"]["balances"].as_array().unwrap();
    let plan = rel.plan();
    for (cell, gross, fees) in statement::accounting_summary(plan, &values) {
        let c = &plan.cells[cell];
        let row = frozen
            .iter()
            .find(|r| r["account"] == c.account.as_str() && r["asset"]["reference"] == c.asset.reference.as_str())
            .unwrap_or_else(|| panic!("no frozen accounting row for {} {}", c.asset_name, c.account));
        assert_eq!(gross, Wide::from_u128(row["gross_debit"].as_str().unwrap().parse().unwrap()), "gross {}", c.account);
        assert_eq!(fees, Wide::from_u128(row["fees_paid"].as_str().unwrap().parse().unwrap()), "fees {}", c.account);
    }
    // Two assets among the debit cells: buyer USDX (transfer + fee) and seller PTS (self-transfer).
    let assets: std::collections::BTreeSet<_> = plan.debit_cells.iter().map(|&c| plan.cells[c].asset_name.clone()).collect();
    assert_eq!(assets.len(), 2);
}

#[test]
fn v2r1_heldout_h1_settle_body_bool_input_and_new_present_zero_duty() {
    let rel = compile_v2(H1, "settle");
    let values = h1_values(&rel);
    let body = abi2::body_bytes(rel.layout(), rel.plan(), &values).expect("body");
    assert_eq!(body.len(), rel.abi2().unwrap().body_len as usize);
    let plan = rel.plan();
    let last = plan.inputs.iter().position(|i| i.name == "last").expect("input last");
    assert_eq!(body[body_offset(&rel, SlotKind::Input { index: last })], 0, "Bool input last = false is one byte 0x00");
    let gross = plan.inputs.iter().position(|i| i.name == "gross").unwrap();
    let at = body_offset(&rel, SlotKind::Input { index: gross });
    assert_eq!(&body[at..at + 16], &250_000u128.to_be_bytes(), "UInt input gross");
    let w = plan.duties.iter().position(|d| d.name == "warranty").expect("warranty duty");
    assert_eq!(body[body_offset(&rel, SlotKind::PreDutyPresent { index: w })], 0, "warranty absent before");
    assert_eq!(body[body_offset(&rel, SlotKind::PostDutyPresent { index: w })], 1, "warranty present after");
    let amt = body_offset(&rel, SlotKind::PostDutyAmount { index: w });
    assert_eq!(&body[amt..amt + 16], &[0u8; 16], "present-zero amount");
}

#[test]
fn v2r1_heldout_h1_settle_honest_satisfies_and_dishonest_refused() {
    let rel = compile_v2(H1, "settle");
    let values = h1_values(&rel);
    let ctx = synthetic_context();
    let a = rel.assignment_v2::<Fr>(values.clone(), &ctx).expect("assignment");
    assert!(satisfied(&rel, a, ctx.clone()));
    let mut bad = values;
    let seller_usdx = rel.plan().cells.iter().position(|c| c.account == "seller" && c.asset_name == "USDX").expect("cell");
    bad.post.cells[seller_usdx] = UInt(bad.post.cells[seller_usdx].0 + 1);
    let a = rel.assignment_v2::<Fr>(bad, &ctx).expect("encodes");
    assert!(!satisfied(&rel, a, ctx));
}

// ## Wide carry (AUTH specimen): gross debit of exactly 2^128 across two amounts

fn wide_case() -> (CompiledRelation, TransitionValues, Json) {
    let e = json(WIDE_EXPECT);
    let rel = compile_v2(WIDE, "spin");
    let scenario: Scenario = serde_json::from_value(e["scenario"].clone()).expect("scenario");
    let values = witness::propose(&rel, &scenario, None).expect("Core accepts the self-transfer and the transfer");
    (rel, values, e)
}

#[test]
fn v2r1_wide_carry_post_and_accounting() {
    let (rel, values, e) = wide_case();
    let plan = rel.plan();
    for (i, c) in plan.cells.iter().enumerate() {
        let expected: u128 = e["post"][c.account.as_str()].as_str().unwrap().parse().unwrap();
        assert_eq!(values.post.cells[i].0, expected, "post {}", c.account);
    }
    let summary = statement::accounting_summary(plan, &values);
    assert_eq!(summary.len(), 1);
    assert_eq!(summary[0].1, Wide { lo: 0, top: 1 }, "gross = 2^128");
    assert_eq!(summary[0].2, Wide::ZERO, "fees = 0");
    let body = abi2::body_bytes(rel.layout(), plan, &values).expect("body");
    let tail = &body[body.len() - 36..];
    assert_eq!(hex(tail), e["accountingTailHex"].as_str().unwrap());
}

#[test]
fn v2r1_wide_carry_honest_satisfies_and_dropped_carry_refused() {
    let (rel, values, _) = wide_case();
    let ctx = synthetic_context();
    let a = rel.assignment_v2::<Fr>(values.clone(), &ctx).expect("assignment");
    assert!(satisfied(&rel, a, ctx.clone()));
    // Hostile claimant: public inputs of a statement whose gross sum lost its carry limb.
    let mut stmt = rel.statement_v2(&values, &ctx).expect("statement");
    let at = stmt.len() - 36;
    stmt[at] = 0;
    stmt[at + 1] = 0;
    let mut a = rel.assignment_v2::<Fr>(values, &ctx).expect("assignment");
    a.public = abi2::public_inputs::<Fr>(&stmt);
    assert!(!satisfied(&rel, a, ctx));
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
