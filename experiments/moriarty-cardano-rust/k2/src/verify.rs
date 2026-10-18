//! Verification only. Nothing here takes an RNG, generates or modifies a key, or sets up a
//! relation. Inputs are the exact public setup-record bytes and proof-export bytes, plus,
//! optionally, a relation recompiled from source by the caller.
//!
//! What each kind of check establishes is listed in the report: cryptographic checks rest on
//! setup honesty and Groth16 knowledge soundness; recomputed checks are byte equalities this
//! process derived itself; declared items are provenance that no verifier can check.
use crate::error::DriverError;
use crate::export::{self, CheckedExport};
use crate::hexutil;
use crate::relation::{self, SelectedRelation};
use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey, prepare_verifying_key};
use serde::Serialize;

/// The Groth16 pairing check with the two SettlementHashV2 public inputs. Both inputs are
/// below 2^128, hence below r, so `Fr::from` is exact (no modular alias).
pub fn pairing_verify(vk: &VerifyingKey<Bls12_381>, proof: &Proof<Bls12_381>, x: [u128; 2]) -> Result<bool, DriverError> {
    let pvk = prepare_verifying_key(vk);
    let inputs = [Fr::from(x[0]), Fr::from(x[1])];
    Groth16::<Bls12_381>::verify_proof(&pvk, proof, &inputs).map_err(|e| DriverError::Backend(format!("Groth16 verify: {e}")))
}

pub const REPORT_FORMAT: &str = "moriarty-k2-verify-report/1";

/// Returned only when every check passed. A refusal is a `DriverError`, never a report.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyReport {
    pub format: &'static str,
    pub accepted: bool,
    pub setup_record_sha256: String,
    pub predicate_digest: String,
    pub vk_digest_blake2b256: String,
    pub statement_digest_blake2b256: String,
    pub relation_recompiled: bool,
    pub cryptographic: Vec<&'static str>,
    pub recomputed: Vec<&'static str>,
    pub declared_only: Vec<&'static str>,
}

/// Verifies a proof export against a setup record. With `recompiled`, also requires the
/// record's relation descriptor to equal the descriptor of that actual compilation.
pub fn verify_export(record_bytes: &[u8], export_bytes: &[u8], recompiled: Option<&SelectedRelation>) -> Result<VerifyReport, DriverError> {
    let setup = export::parse_setup_record(record_bytes)?;
    let e = export::parse_proof_export(export_bytes)?;
    let CheckedExport { proof, public_inputs, statement_digest, .. } = export::check_export(&setup, &e)?;
    if let Some(selected) = recompiled {
        relation::require_same(&setup.record.relation, selected.descriptor())?;
    }
    if !pairing_verify(&setup.vk, &proof, public_inputs)? {
        return Err(DriverError::ProofRejected("Groth16 pairing check failed for this VK and statement digest".into()));
    }
    let mut recomputed = vec![
        "strict JSON (duplicate keys, unknown fields, trailing data, bounds) for both documents",
        "VK: 488 bytes, IC prefix 3, canonical points, on curve, prime-order subgroup, no identity point",
        "vkDigestBlake2b256 = BLAKE2b-256(VK bytes)",
        "proof: 192 bytes, canonical points, on curve, prime-order subgroup, no identity point",
        "setupRecordSha256 = SHA-256(exact setup-record bytes)",
        "statement: tag, family, predicate digest, ABI v2 shape digest, VK digest and body length equal the record",
        "context fields equal the statement's context bytes",
        "statement digest = BLAKE2b-256(statement); x1, x2 = its big-endian halves",
        "Cardano EnvV2 fields, catalogue action entry, intent target hash and anchor recomputed",
    ];
    if recompiled.is_some() {
        recomputed.push("relation descriptor equals a fresh compilation of the supplied source by the actual compiler");
    }
    Ok(VerifyReport {
        format: REPORT_FORMAT,
        accepted: true,
        setup_record_sha256: setup.record_sha256.hex(),
        predicate_digest: setup.record.relation.predicate_digest.clone(),
        vk_digest_blake2b256: hexutil::encode(&setup.vk_digest),
        statement_digest_blake2b256: hexutil::encode(&statement_digest),
        relation_recompiled: recompiled.is_some(),
        cryptographic: vec![
            "Groth16 pairing equation holds for this VK and the two public inputs (under setup honesty and knowledge soundness: a satisfying witness of the VK's relation exists for this statement digest)",
        ],
        recomputed,
        declared_only: vec![
            "that this VK was generated from the matrices named by r1csSha256 (setup provenance)",
            "setup trust profile single-party-test-setup/1 and destruction of setup randomness",
            "sourceSha256 and the compiler build digest as byte identities of sources, not of the executing binary",
            "context premises (claimant, network, settle hash, thread, outref, seq, catalogue and intents digests)",
            "proving-key custody link (provingKeySha256)",
        ],
    })
}
