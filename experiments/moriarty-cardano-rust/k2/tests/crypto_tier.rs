//! Tier C: fresh Groth16 setup, proving and verification. OPT-IN ONLY.
//!
//! Compiled only with `--features crypto-authority`, and every test is `#[ignore]`, so it
//! runs only with `cargo test --features crypto-authority --test crypto_tier -- --ignored`
//! under separate crypto/native authority (`EXECUTION-PROPOSAL.json`, step E-K2-3). Keys are
//! fresh, test-only, drawn from OsRng, kept in memory and dropped at process exit; no file is
//! written. No existing key, wallet, SRS or cache is read. Source only: never compiled or run.
//!
//! Four setups are shared through `OnceLock` (invoice A, invoice B, reserve, held-out H1).
//! Run with `--test-threads=1` to bound memory.
#![cfg(feature = "crypto-authority")]

mod common;

use ark_bls12_381::Bls12_381;
use ark_groth16::{Groth16, Proof, ProvingKey};
use common::*;
use mori_k2_driver::error::DriverError;
use mori_k2_driver::export::{self, ParsedSetup};
use mori_k2_driver::input::{ProveRequest, REQUEST_FORMAT};
use mori_k2_driver::relation::SelectedRelation;
use mori_k2_driver::{claim, compile_selected, encoding, prove, setup, verify};
use rand_core::OsRng;
use std::sync::OnceLock;

const WHY: &str = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)";

struct Fresh {
    sel: SelectedRelation,
    record_bytes: Vec<u8>,
    setup: ParsedSetup,
    pk_bytes: Vec<u8>,
    pk: ProvingKey<Bls12_381>,
}

fn fresh(source: &str, action: &str) -> Fresh {
    let sel = compile_selected(source, &selector(action)).expect("compile");
    let out = setup::setup(&sel, &mut OsRng).expect("setup");
    let setup = export::parse_setup_record(&out.record_bytes).expect("record parses");
    let pk = prove::load_proving_key(&out.proving_key_bytes, &setup).expect("proving key loads");
    Fresh { sel, record_bytes: out.record_bytes, setup, pk_bytes: out.proving_key_bytes, pk }
}

fn invoice_a() -> &'static Fresh {
    static F: OnceLock<Fresh> = OnceLock::new();
    F.get_or_init(|| fresh(INVOICE, "partial"))
}

fn invoice_b() -> &'static Fresh {
    static F: OnceLock<Fresh> = OnceLock::new();
    F.get_or_init(|| fresh(INVOICE, "partial"))
}

fn reserve() -> &'static Fresh {
    static F: OnceLock<Fresh> = OnceLock::new();
    F.get_or_init(|| fresh(RESERVE, "allocate"))
}

fn heldout_h1() -> &'static Fresh {
    static F: OnceLock<Fresh> = OnceLock::new();
    F.get_or_init(|| fresh(H1, "settle"))
}

fn prove_request(f: &Fresh, req: &ProveRequest) -> Result<prove::ProveOutput, DriverError> {
    prove::prove(&f.sel, &f.setup, &f.pk, req, &mut OsRng)
}

fn proved(f: &Fresh, text: &str) -> prove::ProveOutput {
    prove_request(f, &request(text)).expect("prove")
}

fn checked(f: &Fresh, out: &prove::ProveOutput) -> export::CheckedExport {
    export::check_export(&f.setup, &export::parse_proof_export(&out.export_bytes).unwrap()).unwrap()
}

/// Re-exports `proof` for a different (consistently re-derived) claim.
fn reexport(f: &Fresh, claim: &claim::PreparedClaim, proof: &Proof<Bls12_381>) -> Vec<u8> {
    export::document_bytes(&export::build_export(&f.setup, claim, proof).unwrap()).unwrap()
}

fn assert_rejected(r: Result<verify::VerifyReport, DriverError>, what: &str) {
    match r {
        Err(DriverError::ProofRejected(_)) => {}
        other => panic!("{what}: expected ProofRejected, got {:?}", other.map(|r| r.statement_digest_blake2b256)),
    }
}

// ## Positive paths (two contrasting programs plus the non-author held-out program)

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c01_invoice_partial_end_to_end() {
    let _ = WHY;
    let f = invoice_a();
    let out = proved(f, REQ_INVOICE);
    let r = verify::verify_export(&f.record_bytes, &out.export_bytes, None).expect("export-only verification");
    assert!(r.accepted && !r.relation_recompiled);
    let r = verify::verify_export(&f.record_bytes, &out.export_bytes, Some(&f.sel)).expect("recompiled verification");
    assert!(r.relation_recompiled);
    // [R1-AUTH] the proved statement is the fixture statement up to predicate and VK digest.
    let case = r1_case("P02");
    let mut stmt = unhex(&out.export.statement_hex);
    stmt[48..80].copy_from_slice(&arr32(case["context"]["predicateDigestSynthetic"].as_str().unwrap()));
    stmt[112..144].copy_from_slice(&arr32(case["context"]["vkDigestSynthetic"].as_str().unwrap()));
    assert_eq!(hex(&stmt), case["statementHex"].as_str().unwrap());
    // Cardano-facing values agree with the record.
    let vk = unhex(&f.setup.record.verifying_key_hex);
    assert_eq!(vk.len(), CONSUMER_VK_LEN);
    assert_eq!(hex(&vk[336..344]), CONSUMER_VK_IC_COUNT_HEX);
    assert_eq!(hex(&mori_r1cs::blake2b::blake2b256(&vk)), out.export.cardano.env_v2.vk_digest);
    assert_eq!(out.export.cardano.env_v2.statement_digest, out.export.statement_digest_blake2b256);
    assert_eq!(out.export.cardano.catalogue_action_entry.predicate_digest, f.sel.descriptor().predicate_digest);
    assert_eq!(unhex(&out.export.proof_hex).len(), CONSUMER_PROOF_LEN);
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c02_reserve_allocate_end_to_end() {
    let f = reserve();
    let out = proved(f, REQ_RESERVE);
    assert!(verify::verify_export(&f.record_bytes, &out.export_bytes, Some(&f.sel)).is_ok());
    let case = r1_case("P04");
    let mut stmt = unhex(&out.export.statement_hex);
    stmt[48..80].copy_from_slice(&arr32(case["context"]["predicateDigestSynthetic"].as_str().unwrap()));
    stmt[112..144].copy_from_slice(&arr32(case["context"]["vkDigestSynthetic"].as_str().unwrap()));
    assert_eq!(hex(&stmt), case["statementHex"].as_str().unwrap());
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c03_heldout_h1_records_end_to_end_and_dishonest_refused() {
    let f = heldout_h1();
    let honest = ProveRequest { format: REQUEST_FORMAT.into(), selector: selector("settle"), transition: h1_records(), context: synthetic_context() };
    let out = prove_request(f, &honest).expect("held-out records prove");
    assert!(verify::verify_export(&f.record_bytes, &out.export_bytes, Some(&f.sel)).is_ok());
    let dishonest = ProveRequest { transition: h1_records_dishonest(), ..honest };
    assert!(matches!(prove_request(f, &dishonest), Err(DriverError::Unsatisfied(_))));
}

// ## Identity and key mismatches

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c04_cross_relation_record_refused() {
    let (inv, res) = (invoice_a(), reserve());
    let out = proved(inv, REQ_INVOICE);
    assert!(matches!(verify::verify_export(&res.record_bytes, &out.export_bytes, None), Err(DriverError::IdentityMismatch { .. })));
    let c = checked(inv, &out);
    assert!(!verify::pairing_verify(&res.setup.vk, &c.proof, c.public_inputs).unwrap());
    // Proving one relation's request with another relation's record is refused before key use.
    assert!(matches!(prove::prove(&res.sel, &inv.setup, &inv.pk, &request(REQ_RESERVE), &mut OsRng), Err(DriverError::IdentityMismatch { .. })));
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c05_second_setup_of_the_same_relation_is_a_different_vk() {
    let (a, b) = (invoice_a(), invoice_b());
    assert_eq!(a.setup.record.relation, b.setup.record.relation, "same compiled relation");
    assert_ne!(a.setup.vk_digest, b.setup.vk_digest, "fresh randomness");
    let out = proved(a, REQ_INVOICE);
    assert!(verify::verify_export(&b.record_bytes, &out.export_bytes, None).is_err());
    let c = checked(a, &out);
    assert!(!verify::pairing_verify(&b.setup.vk, &c.proof, c.public_inputs).unwrap());
    // A proving key from setup A never proves under record B.
    assert!(prove::load_proving_key(&a.pk_bytes, &b.setup).is_err());
    assert!(matches!(prove_request_with(b, &a.pk), Err(DriverError::IdentityMismatch { .. })));
}

fn prove_request_with(f: &Fresh, pk: &ProvingKey<Bls12_381>) -> Result<prove::ProveOutput, DriverError> {
    prove::prove(&f.sel, &f.setup, pk, &request(REQ_INVOICE), &mut OsRng)
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c06_vk_tamper_with_consistent_digest_refused() {
    let f = invoice_a();
    let out = proved(f, REQ_INVOICE);
    let mut swapped = f.setup.vk.clone();
    std::mem::swap(&mut swapped.gamma_g2, &mut swapped.delta_g2);
    let bytes = encoding::encode_vk(&swapped).unwrap();
    let mut record = f.setup.record.clone();
    record.verifying_key_hex = hex(&bytes);
    record.vk_digest_blake2b256 = hex(&mori_r1cs::blake2b::blake2b256(&bytes));
    let record_bytes = export::document_bytes(&record).unwrap();
    assert!(export::parse_setup_record(&record_bytes).is_ok(), "the tampered record is well formed");
    assert!(matches!(verify::verify_export(&record_bytes, &out.export_bytes, None), Err(DriverError::IdentityMismatch { .. })));
    let c = checked(f, &out);
    assert!(!verify::pairing_verify(&swapped, &c.proof, c.public_inputs).unwrap());
}

// ## Statement, context, public-input and proof tampering

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c07_every_context_field_is_bound_by_the_proof() {
    let f = invoice_a();
    let out = proved(f, REQ_INVOICE);
    let proof = checked(f, &out).proof;
    let edits: [(&str, fn(&mut mori_k2_driver::ContextInput)); 9] = [
        ("claimant", |c| c.claimant_vkh = "56".repeat(28)),
        ("network", |c| c.network_id = 1),
        ("settle hash", |c| c.settle_hash = "a3".repeat(28)),
        ("thread", |c| c.thread_id = "7b".repeat(32)),
        ("consumed txid", |c| c.consumed_txid = "23".repeat(32)),
        ("consumed ix", |c| c.consumed_ix = 1),
        ("seq", |c| c.seq = 2),
        ("catalogue digest", |c| c.catalogue_digest = "00".repeat(32)),
        ("intents digest", |c| c.intents_digest = "00".repeat(32)),
    ];
    for (what, edit) in edits {
        let mut req = request(REQ_INVOICE);
        edit(&mut req.context);
        let other = claim::prepare_claim(&f.sel, &req.transition, &req.context, f.setup.vk_digest).unwrap();
        assert_rejected(verify::verify_export(&f.record_bytes, &reexport(f, &other, &proof), None), what);
    }
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c08_another_valid_transition_is_not_covered_by_the_proof() {
    let f = invoice_a();
    let out = proved(f, REQ_INVOICE);
    let proof = checked(f, &out).proof;
    let mut req = request(REQ_INVOICE);
    let mori_k2_driver::TransitionInput::CoreReference { scenario } = &mut req.transition else { panic!() };
    scenario.inputs.insert("amount".into(), mori_core::Value::UInt(mori_core::UInt(34)));
    let other = claim::prepare_claim(&f.sel, &req.transition, &req.context, f.setup.vk_digest).expect("Core accepts amount 34");
    assert_rejected(verify::verify_export(&f.record_bytes, &reexport(f, &other, &proof), None), "body");
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c09_public_input_tamper_rejected() {
    let f = invoice_a();
    let out = proved(f, REQ_INVOICE);
    let c = checked(f, &out);
    let [x1, x2] = c.public_inputs;
    assert!(verify::pairing_verify(&f.setup.vk, &c.proof, [x1, x2]).unwrap());
    assert!(!verify::pairing_verify(&f.setup.vk, &c.proof, [x1 ^ 1, x2]).unwrap());
    assert!(!verify::pairing_verify(&f.setup.vk, &c.proof, [x2, x1]).unwrap());
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c10_proof_tamper_rejected() {
    let f = invoice_a();
    let out = proved(f, REQ_INVOICE);
    let c = checked(f, &out);
    let mut req = request(REQ_INVOICE);
    req.context.vk_digest = None;
    let claim = claim::prepare_claim(&f.sel, &req.transition, &req.context, f.setup.vk_digest).unwrap();
    let negated = Proof { a: -c.proof.a, ..c.proof.clone() };
    assert_rejected(verify::verify_export(&f.record_bytes, &reexport(f, &claim, &negated), None), "negated A");
    let swapped = Proof { a: c.proof.c, c: c.proof.a, b: c.proof.b };
    assert_rejected(verify::verify_export(&f.record_bytes, &reexport(f, &claim, &swapped), None), "A and C swapped");
    // A flipped byte is refused either as an encoding error or by the pairing check.
    let mut e = export::parse_proof_export(&out.export_bytes).unwrap();
    let mut raw = unhex(&e.proof_hex);
    raw[100] ^= 0x01;
    e.proof_hex = hex(&raw);
    assert!(verify::verify_export(&f.record_bytes, &export::document_bytes(&e).unwrap(), None).is_err());
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c11_rerandomized_proof_also_verifies_malleability_is_documented() {
    let f = invoice_a();
    let out = proved(f, REQ_INVOICE);
    let c = checked(f, &out);
    let other = Groth16::<Bls12_381>::rerandomize_proof(&f.setup.vk, &c.proof, &mut OsRng);
    assert_ne!(encoding::encode_proof(&other).unwrap(), encoding::encode_proof(&c.proof).unwrap());
    assert!(verify::pairing_verify(&f.setup.vk, &other, c.public_inputs).unwrap(), "same statement, new proof bytes");
}

#[test]
#[ignore = "crypto authority: fresh test-only Groth16 setup/prove/verify (EXECUTION-PROPOSAL E-K2-3)"]
fn c12_setup_record_and_key_custody_link() {
    let f = invoice_a();
    assert_eq!(f.setup.record.proving_key_bytes, f.pk_bytes.len() as u64);
    assert_eq!(f.setup.record.proving_key_sha256, mori_r1cs::identity::digest_bytes(&f.pk_bytes).hex());
    assert_eq!(f.setup.record.relation, *f.sel.descriptor());
    assert_eq!(f.setup.record.trust_profile, "single-party-test-setup/1");
    let mut truncated = f.pk_bytes.clone();
    truncated.pop();
    assert!(prove::load_proving_key(&truncated, &f.setup).is_err());
}
