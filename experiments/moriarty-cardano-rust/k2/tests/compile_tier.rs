//! Tier B: compilation through the actual compiler and prove-mode constraint synthesis. No
//! key, no proof, no pairing. Source only: never compiled or run; every assertion is a
//! prediction.
//!
//! Resource note: every SettlementHashV2 compile runs the compiler's setup-mode probe and every
//! `check_claim` a prove-mode synthesis, each including an in-circuit BLAKE2b of a 6-8 block
//! statement. With the frozen R1 `blake2b.rs` (E-01) each is predicted to exceed 8 GiB; this
//! tier is only runnable on the reviewed bounded-XOR compiler.
mod common;

use common::*;
use mori_k2_driver::claim;
use mori_k2_driver::error::DriverError;
use mori_k2_driver::prove;
use mori_k2_driver::relation::{self, SelectedRelation};
use mori_k2_driver::{TransitionInput, compile_selected};
use mori_r1cs::{AdapterError, CompileError, MappingError};

fn compiled(source: &str, action: &str) -> SelectedRelation {
    compile_selected(source, &selector(action)).unwrap_or_else(|e| panic!("compile {action}: {e}"))
}

fn blake(b: &[u8]) -> [u8; 32] {
    mori_r1cs::blake2b::blake2b256(b)
}

#[test]
fn b01_two_contrasting_programs_through_the_same_driver() {
    let invoice = compiled(INVOICE, "partial");
    let reserve = compiled(RESERVE, "allocate");
    let (a, b) = (invoice.descriptor(), reserve.descriptor());
    assert_ne!(a.program_hash, b.program_hash);
    assert_ne!(a.predicate_digest, b.predicate_digest);
    assert_ne!(a.abi2_shape_digest, b.abi2_shape_digest);
    assert_ne!(a.relation_shape_digest, b.relation_shape_digest);
    // One compiler build and one semantic profile for both relations.
    assert_eq!(a.compiler_build_digest, b.compiler_build_digest);
    assert_eq!(a.profile_digest, b.profile_digest);
    for d in [a, b] {
        assert_eq!(d.adapter, "SettlementHashV2");
        assert_eq!(d.public_input_count, 2);
        assert_eq!(d.instance_variables, 3);
        assert!(d.constraints > 0);
        assert!(d.source_sha256.is_some());
        assert!(d.check_profile().is_ok());
    }
}

#[test]
fn b02_abi2_shape_bytes_and_body_length_match_r1_fixtures() {
    for (source, action, id) in [(INVOICE, "partial", "P02"), (RESERVE, "allocate", "P04")] {
        let d = compiled(source, action).descriptor().clone();
        let case = r1_case(id);
        assert_eq!(d.abi2_shape_bytes_hex, case["shapeBytesHex"].as_str().unwrap(), "{id} shape bytes");
        assert_eq!(d.abi2_shape_digest, case["shapeDigest"].as_str().unwrap(), "{id} shape digest");
        assert_eq!(d.body_len as usize, case["bodyHex"].as_str().unwrap().len() / 2, "{id} body length");
    }
}

#[test]
fn b03_program_hashes_match_the_addendum() {
    let hashes = &json(ADDENDUM)["additional_expectations"]["program_hashes"];
    assert_eq!(compiled(INVOICE, "partial").descriptor().program_hash, hashes["installment-invoice.mori"].as_str().unwrap());
    assert_eq!(compiled(RESERVE, "allocate").descriptor().program_hash, hashes["conditional-reserve.mori"].as_str().unwrap());
}

#[test]
fn b04_statement_body_and_spliced_statement_match_r1_fixtures() {
    for (source, text, id) in [(INVOICE, REQ_INVOICE, "P02"), (RESERVE, REQ_RESERVE, "P04")] {
        let case = r1_case(id);
        let c = &case["context"];
        let req = request(text);
        let sel = compiled(source, &req.selector.action);
        let vk = arr32(c["vkDigestSynthetic"].as_str().unwrap());
        let claim = claim::prepare_claim(&sel, &req.transition, &req.context, vk).expect("claim");
        assert_eq!(hex(&claim.statement[343..]), case["bodyHex"].as_str().unwrap(), "{id} body");
        assert_eq!(hex(&claim.statement[48..80]), sel.descriptor().predicate_digest, "{id} real predicate in the statement");
        // Replace only the predicate with the fixture's synthetic one: the rest is the fixture.
        let mut spliced = claim.statement.clone();
        spliced[48..80].copy_from_slice(&arr32(c["predicateDigestSynthetic"].as_str().unwrap()));
        assert_eq!(hex(&spliced), case["statementHex"].as_str().unwrap(), "{id} statement");
        assert_eq!(hex(&blake(&spliced)), case["statementDigest"].as_str().unwrap(), "{id} digest");
        assert_eq!(claim.statement_digest, blake(&claim.statement));
        assert_eq!(claim.public_inputs, claim::digest_halves(&claim.statement_digest));
    }
}

#[test]
fn b05_honest_claims_satisfy_and_prove_matrices_equal_the_probe() {
    for (source, text, id) in [(INVOICE, REQ_INVOICE, "P02"), (RESERVE, REQ_RESERVE, "P04")] {
        let req = request(text);
        let sel = compiled(source, &req.selector.action);
        let vk = arr32(r1_case(id)["context"]["vkDigestSynthetic"].as_str().unwrap());
        let claim = claim::prepare_claim(&sel, &req.transition, &req.context, vk).expect("claim");
        let sat = prove::check_claim(&sel, &claim).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(relation::matrix_digest(&sat.matrices).hex(), sel.descriptor().r1cs_sha256, "{id}");
        assert_eq!(sat.num_inputs, 3);
        assert_eq!(sat.num_constraints as u64, sel.descriptor().constraints);
        assert_eq!(sat.full_assignment.len() as u64, 3 + sel.descriptor().witness_variables);
    }
}

#[test]
fn b06_heldout_h1_records_satisfy() {
    let sel = compiled(H1, "settle");
    let claim = claim::prepare_claim(&sel, &h1_records(), &synthetic_context(), [0x11; 32]).expect("held-out records map");
    assert_eq!(claim.statement.len(), 343 + sel.descriptor().body_len as usize);
    assert!(prove::check_claim(&sel, &claim).is_ok());
}

#[test]
fn b07_dishonest_records_are_unsatisfied_not_proved() {
    let sel = compiled(H1, "settle");
    let claim = claim::prepare_claim(&sel, &h1_records_dishonest(), &synthetic_context(), [0x11; 32]).expect("a dishonest claim still encodes");
    assert!(matches!(prove::check_claim(&sel, &claim), Err(DriverError::Unsatisfied(_))));
}

#[test]
fn b08_core_refusal_proposes_no_claim() {
    let mut req = request(REQ_INVOICE);
    let TransitionInput::CoreReference { scenario } = &mut req.transition else { panic!("CoreReference") };
    scenario.inputs.insert("amount".into(), mori_core::Value::UInt(mori_core::UInt(0)));
    let sel = compiled(INVOICE, "partial");
    let r = claim::prepare_claim(&sel, &req.transition, &req.context, [0x11; 32]);
    assert!(matches!(r, Err(DriverError::Proposal(MappingError::ReferenceRefused(_)))));
}

#[test]
fn b09_staged_v2_is_refused_at_lowering() {
    let r = compile_selected(DELIVERY2, &selector("deliver"));
    assert!(matches!(r, Err(DriverError::Compile(CompileError::StagedNotLowered(_)))));
}

#[test]
fn b10_whole_program_emit_is_refused() {
    let r = compile_selected(PAYROLL, &selector("settle"));
    assert!(matches!(r, Err(DriverError::Compile(CompileError::Adapter(AdapterError::TargetRefusal(_))))));
}

#[test]
fn b11_unknown_action_is_refused() {
    let r = compile_selected(INVOICE, &selector("refund"));
    assert!(matches!(r, Err(DriverError::Compile(CompileError::UnknownAction(_)))));
}

#[test]
fn b12_identity_pins() {
    let hash = json(ADDENDUM)["additional_expectations"]["program_hashes"]["installment-invoice.mori"].as_str().unwrap().to_string();
    let mut s = selector("partial");
    s.expected_program_hash = Some(hash);
    assert!(compile_selected(INVOICE, &s).is_ok());
    s.expected_program_hash = Some("11".repeat(32));
    assert!(matches!(compile_selected(INVOICE, &s), Err(DriverError::IdentityMismatch { .. })));
    let mut t = selector("partial");
    t.expected_source_sha256 = Some("00".repeat(32));
    assert!(matches!(compile_selected(INVOICE, &t), Err(DriverError::IdentityMismatch { .. })));
    t.expected_source_sha256 = Some("XYZ".into());
    assert!(matches!(compile_selected(INVOICE, &t), Err(DriverError::Input(_))));
}

#[test]
fn b13_restated_vk_digest_must_match_the_vk_used() {
    let mut req = request(REQ_INVOICE);
    req.context.vk_digest = Some("12".repeat(32));
    let sel = compiled(INVOICE, "partial");
    let r = claim::prepare_claim(&sel, &req.transition, &req.context, [0x11; 32]);
    assert!(matches!(r, Err(DriverError::IdentityMismatch { .. })));
}

#[test]
fn b14_a_context_change_moves_only_its_bytes_and_the_digest() {
    let req = request(REQ_INVOICE);
    let sel = compiled(INVOICE, "partial");
    let a = claim::prepare_claim(&sel, &req.transition, &req.context, [0x11; 32]).unwrap();
    let mut other = req.context.clone();
    other.consumed_ix = 1;
    let b = claim::prepare_claim(&sel, &req.transition, &other, [0x11; 32]).unwrap();
    assert_eq!(a.statement.len(), b.statement.len());
    let differing: Vec<usize> = (0..a.statement.len()).filter(|&i| a.statement[i] != b.statement[i]).collect();
    assert_eq!(differing, vec![266], "consumed_ix is a big-endian u16 at 265..267");
    assert_ne!(a.statement_digest, b.statement_digest);
    assert_ne!(a.public_inputs, b.public_inputs);
}

#[test]
fn b15_a_record_for_another_relation_is_detected_before_key_use() {
    let invoice = compiled(INVOICE, "partial");
    let reserve = compiled(RESERVE, "allocate");
    assert!(matches!(relation::require_same(invoice.descriptor(), reserve.descriptor()), Err(DriverError::IdentityMismatch { .. })));
    let open = compiled(INVOICE, "open");
    assert!(relation::require_same(invoice.descriptor(), open.descriptor()).is_err(), "same program, other action");
}

#[test]
fn b16_compilation_is_deterministic_and_reproducible_from_a_descriptor() {
    let a = compiled(INVOICE, "partial");
    let b = compiled(INVOICE, "partial");
    assert_eq!(a.descriptor(), b.descriptor());
    let again = compile_selected(INVOICE, &relation::selector_for(a.descriptor())).expect("recompile from descriptor");
    assert_eq!(again.descriptor(), a.descriptor());
}
