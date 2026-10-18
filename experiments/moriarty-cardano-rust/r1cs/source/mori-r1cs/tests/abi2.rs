//! P3S1 `SettlementHashV2` tests. Source only: never compiled or run.
//!
//! Expectations come from `rust/expectations/RUST-ABI2-EXPECTATIONS.json`, a byte copy
//! of the P3S1 file (FREEZE-1B, written before these bodies; sha256 0b6ba408...73bd).
//! BLAKE2b-256 vectors are Python `hashlib` values (independent of this crate's BLAKE2b);
//! the P02 shape, body and statement bytes are the P3S1 AUTHOR ORACLE's (domain /1 intents
//! digest; the relation hashes context bytes without interpreting them), so equality here
//! is cross-language agreement on one specimen, not proof.
//!
//! P3S1-R1 (review RR-17): every include of this crate resolves inside `rust/`, so copying
//! `candidate/rust/` to `<root>/r1cs/` keeps all relative paths. This one was
//! `/../../../expectations/` (outside `rust/`), which the requested layout did not provide.
mod common;

use ark_bls12_381::Fr;
use ark_relations::r1cs::ConstraintSystem;
use common::{INVOICE, PAYROLL, build};
use mori_core::Scenario;
use mori_r1cs::*;
use serde_json::Value as Json;

const EXPECT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../expectations/RUST-ABI2-EXPECTATIONS.json"));

fn expect() -> Json {
    serde_json::from_str(EXPECT).expect("frozen expectations parse")
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

fn compile_v2(source: &str, options: CompileOptions) -> Result<CompiledRelation, CompileError> {
    compile_source::<Fr>(source, options, StatementAdapter::SettlementHashV2)
}

fn oracle_context(e: &Json) -> ContextV2 {
    let c = &e["p02"]["context"];
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

fn p02_values(rel: &CompiledRelation, e: &Json) -> TransitionValues {
    let scenario: Scenario = serde_json::from_value(e["p02"]["bridgeScenario"].clone()).expect("bridge scenario is a Core Scenario");
    witness::propose(rel, &scenario, None).expect("Core reference proposes the bridge transition")
}

fn satisfied(rel: &CompiledRelation, a: Assignment<Fr>, ctx: ContextV2, tamper: Tamper<Fr>) -> (bool, usize) {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let report = rel.circuit_v2(Some(a), Some(ctx), tamper).expect("v2 circuit").synthesize_into(cs.clone()).expect("synthesis");
    (cs.is_satisfied().expect("prove-mode system"), report.overrides_used)
}

// ## BLAKE2b

#[test]
fn blake2b_host_matches_hashlib_vectors() {
    for v in expect()["blake2b"].as_array().unwrap() {
        let msg = unhex(v["messageHex"].as_str().unwrap());
        assert_eq!(blake2b::blake2b256(&msg).to_vec(), unhex(v["blake2b256"].as_str().unwrap()), "vector {}", v["name"]);
    }
}

#[test]
fn blake2b_gadget_matches_hashlib_vectors() {
    // P3S1-R1G1 (review E-01) [AUTH, STRUCTURAL]: no stored row grows with the number of
    // blocks, and the stored weight grows linearly with it. The widest equation is a sum of
    // three words modulo 2^64: 3 * 64 input bits, 66 result bits and the constant. Checked
    // first, on 1- and 3-block messages, so a regression fails here before the 6-block
    // statement vector below is synthesized.
    let widest = 3 * 64 + 66 + 1;
    let (row1, weight1) = blake2b::gadget_weight::<Fr>(&[0xa5; 100]).expect("1-block setup synthesis");
    assert!(row1 <= widest, "1 block: a stored row has {row1} nonzero entries");
    let (row3, weight3) = blake2b::gadget_weight::<Fr>(&[0xa5; 300]).expect("3-block setup synthesis");
    assert!(row3 <= widest, "3 blocks: a stored row has {row3} nonzero entries");
    assert!(weight3 <= 4 * weight1, "3-block weight {weight3} is not linear in the 1-block weight {weight1}");
    for v in expect()["blake2b"].as_array().unwrap() {
        let msg = unhex(v["messageHex"].as_str().unwrap());
        let (ok, digest, constraints) = blake2b::gadget_check::<Fr>(&msg).expect("gadget synthesis");
        assert!(ok, "gadget unsatisfied for {}", v["name"]);
        assert!(constraints > 0);
        assert_eq!(digest.to_vec(), unhex(v["blake2b256"].as_str().unwrap()), "vector {}", v["name"]);
    }
}

// ## Compile-time admission

#[test]
fn v2_compiles_invoice_partial_with_two_public_inputs() {
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    assert_eq!(rel.adapter().public_input_count(rel.layout()).unwrap(), 2);
    assert_eq!(rel.identity().adapter, StatementAdapter::SettlementHashV2);
}

#[test]
fn v2_shape_bytes_match_oracle() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let consts = rel.abi2().expect("v2 constants");
    assert_eq!(consts.shape_bytes, unhex(e["p02"]["shapeBytesHex"].as_str().unwrap()));
    assert_eq!(consts.shape_digest.to_vec(), unhex(e["p02"]["shapeDigest"].as_str().unwrap()));
    assert_eq!(consts.body_len as usize, unhex(e["p02"]["bodyHex"].as_str().unwrap()).len());
    assert_eq!(consts.predicate_digest, rel.identity().digest().unwrap().0);
}

#[test]
fn v2_refuses_public_cap() {
    let mut o = v2_options("partial");
    o.cap = CapBinding::PublicCap;
    assert!(matches!(compile_v2(INVOICE, o), Err(CompileError::Option(_))));
}

#[test]
fn v2_refuses_unbound_accounting() {
    let mut o = v2_options("partial");
    o.accounting = AccountingBinding::NotBound;
    assert!(matches!(compile_v2(INVOICE, o), Err(CompileError::Option(_))));
}

#[test]
fn v2_refuses_frame_cells() {
    let mut o = v2_options("partial");
    o.frame = FrameCells::Include(vec![CellRef { asset: "CASH".into(), account: "spectator".into() }]);
    assert!(matches!(compile_v2(INVOICE, o), Err(CompileError::Option(_))));
}

#[test]
fn v2_refuses_whole_program_emit() {
    assert!(matches!(compile_v2(PAYROLL, v2_options("settle")), Err(CompileError::Adapter(AdapterError::TargetRefusal(_)))));
}

#[test]
fn direct_field_adapter_unchanged_for_emit() {
    let mut o = v2_options("settle");
    o.accounting = AccountingBinding::NotBound;
    assert!(compile_source::<Fr>(PAYROLL, o, StatementAdapter::DirectFieldV0).is_ok());
}

// ## Statement agreement with the oracle

#[test]
fn v2_body_statement_and_public_inputs_match_oracle() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let values = p02_values(&rel, &e);
    let body = abi2::body_bytes(rel.layout(), rel.plan(), &values).expect("body");
    assert_eq!(body, unhex(e["p02"]["bodyHex"].as_str().unwrap()));
    let consts = rel.abi2().unwrap();
    let oracle_consts = AbiV2Constants {
        predicate_digest: arr(e["p02"]["context"]["predicateDigestSynthetic"].as_str().unwrap()),
        shape_bytes: consts.shape_bytes.clone(),
        shape_digest: consts.shape_digest,
        body_len: consts.body_len,
    };
    let stmt = abi2::statement_bytes(&oracle_consts, &oracle_context(&e), &body).expect("statement");
    assert_eq!(stmt, unhex(e["p02"]["statementHex"].as_str().unwrap()));
    let x = abi2::public_inputs::<Fr>(&stmt);
    let x1: u128 = e["p02"]["publicInputs"]["x1"].as_str().unwrap().parse().unwrap();
    let x2: u128 = e["p02"]["publicInputs"]["x2"].as_str().unwrap().parse().unwrap();
    assert_eq!(x, vec![Fr::from(x1), Fr::from(x2)]);
}

// ## Constraint-level behaviour

#[test]
fn v2_honest_assignment_satisfies() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let ctx = oracle_context(&e);
    let a = rel.assignment_v2::<Fr>(p02_values(&rel, &e), &ctx).expect("assignment");
    assert_eq!(a.public.len(), 2);
    assert!(satisfied(&rel, a, ctx, Tamper::none()).0);
}

#[test]
fn v2_public_input_change_unsatisfied() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let ctx = oracle_context(&e);
    let mut a = rel.assignment_v2::<Fr>(p02_values(&rel, &e), &ctx).expect("assignment");
    a.public[1] += Fr::from(1u64);
    assert!(!satisfied(&rel, a, ctx, Tamper::none()).0);
}

#[test]
fn v2_context_swap_unsatisfied() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let ctx = oracle_context(&e);
    let a = rel.assignment_v2::<Fr>(p02_values(&rel, &e), &ctx).expect("assignment");
    let mut other = ctx.clone();
    other.claimant_vkh = [0x66; 28];
    assert!(!satisfied(&rel, a, other, Tamper::none()).0);
}

#[test]
fn v2_claimant_and_outref_bind_public_inputs() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let values = p02_values(&rel, &e);
    let ctx = oracle_context(&e);
    let base = rel.public_inputs_v2::<Fr>(&values, &ctx).unwrap();
    let mut c1 = ctx.clone();
    c1.claimant_vkh = [0x66; 28];
    let mut c2 = ctx.clone();
    c2.consumed_ix = 1;
    let mut c3 = ctx;
    c3.intents_digest = [0; 32];
    for c in [c1, c2, c3] {
        assert_ne!(rel.public_inputs_v2::<Fr>(&values, &c).unwrap(), base);
    }
}

#[test]
fn v2_dishonest_post_cell_unsatisfied() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let ctx = oracle_context(&e);
    let mut values = p02_values(&rel, &e);
    values.post.cells[0] = mori_core::UInt(values.post.cells[0].0 + 1);
    let a = rel.assignment_v2::<Fr>(values, &ctx).expect("a dishonest claim still encodes");
    assert!(!satisfied(&rel, a, ctx, Tamper::none()).0);
}

#[test]
fn v2_tampered_body_bit_unsatisfied() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let ctx = oracle_context(&e);
    let a = rel.assignment_v2::<Fr>(p02_values(&rel, &e), &ctx).expect("assignment");
    // First body bit: most significant byte of the `amount` input (35), bit 0, honest 0.
    let (ok, used) = satisfied(&rel, a, ctx, Tamper::none().set("abi2/abi2.body#0", Fr::from(1u64)));
    assert_eq!(used, 1, "the tamper reached its witness");
    assert!(!ok);
}

#[test]
fn v2_context_free_apis_refuse() {
    let e = expect();
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    let values = p02_values(&rel, &e);
    assert!(rel.public_inputs::<Fr>(&values).is_err());
    let a = rel.assignment_v2::<Fr>(values, &oracle_context(&e)).unwrap();
    assert!(matches!(rel.circuit(Some(a), Tamper::none()), Err(CompileError::Option(_))));
}

#[test]
fn v2_setup_mode_needs_no_context() {
    let rel = compile_v2(INVOICE, v2_options("partial")).expect("v2 compile");
    assert!(rel.circuit_v2::<Fr>(None, None, Tamper::none()).is_ok());
    assert!(rel.probe().constraints > 0);
}
