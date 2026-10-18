//! Proving: the compiled relation decides, then Groth16 proves, then the proof is verified
//! before anything is exported.
//!
//! ark-groth16 0.4 checks satisfaction only with `debug_assert!` (`prover.rs:197`), so a
//! release build would turn an unsatisfied witness into an invalid proof without any error.
//! [`check_claim`] therefore synthesizes the relation's own `circuit_v2` once, in prove mode
//! with matrices, and refuses unless (1) the system is satisfied, (2) no tamper override was
//! used, (3) the instance assignment is exactly `[1, x1, x2]` and (4) the matrix digest equals
//! the compile probe's `r1cs_sha256`. The same synthesis then feeds
//! `create_proof_with_reduction_and_matrices` (`prover.rs:26-51`).
use crate::claim::{self, PreparedClaim};
use crate::encoding;
use crate::error::DriverError;
use crate::export::{self, ParsedSetup, ProofExport};
use crate::hexutil;
use crate::input::ProveRequest;
use crate::relation::{self, SelectedRelation};
use crate::verify;
use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::{One, UniformRand};
use ark_groth16::{Groth16, ProvingKey};
use ark_relations::r1cs::{ConstraintMatrices, ConstraintSystem, OptimizationGoal, SynthesisMode};
use mori_r1cs::Tamper;
use rand_core::{CryptoRng, RngCore};

/// A claim the compiled relation accepted, with the synthesis that proves it.
#[derive(Debug, Clone)]
pub struct SatisfiedClaim {
    pub matrices: ConstraintMatrices<Fr>,
    /// `[1, x1, x2, witness...]`.
    pub full_assignment: Vec<Fr>,
    pub num_inputs: usize,
    pub num_constraints: usize,
}

/// The constraint-system decision on a prepared claim. Needs no key.
pub fn check_claim(selected: &SelectedRelation, claim: &PreparedClaim) -> Result<SatisfiedClaim, DriverError> {
    let relation = selected.relation();
    let assignment = relation.assignment_v2::<Fr>(claim.values.clone(), &claim.context).map_err(DriverError::Proposal)?;
    let expected_public = vec![Fr::from(claim.public_inputs[0]), Fr::from(claim.public_inputs[1])];
    if assignment.public != expected_public {
        return Err(DriverError::Backend("assignment public inputs differ from the statement digest halves".into()));
    }
    let circuit = relation
        .circuit_v2::<Fr>(Some(assignment), Some(claim.context.clone()), Tamper::none())
        .map_err(DriverError::Compile)?;
    let cs = ConstraintSystem::<Fr>::new_ref();
    cs.set_optimization_goal(OptimizationGoal::Constraints);
    cs.set_mode(SynthesisMode::Prove { construct_matrices: true });
    let report = circuit.synthesize_into(cs.clone()).map_err(|e| DriverError::Backend(format!("synthesis: {e}")))?;
    if report.overrides_used != 0 {
        return Err(DriverError::Backend("a tamper override reached a witness in a production synthesis".into()));
    }
    match cs.is_satisfied() {
        Ok(true) => {}
        Ok(false) => {
            return Err(DriverError::Unsatisfied(
                "the compiled relation rejects this claimed transition in this context".into(),
            ));
        }
        Err(e) => return Err(DriverError::Backend(format!("satisfaction check: {e}"))),
    }
    cs.finalize();
    let matrices = cs.to_matrices().ok_or_else(|| DriverError::Backend("prove-mode matrices were not constructed".into()))?;
    let digest = relation::matrix_digest(&matrices);
    let probe = relation.probe().r1cs_sha256;
    if digest != probe {
        return Err(DriverError::mismatch("r1cs matrices (prove synthesis vs compile probe)", probe.hex(), digest.hex()));
    }
    let (full_assignment, num_inputs) = {
        let inner = cs.borrow().ok_or_else(|| DriverError::Backend("constraint system missing".into()))?;
        let full: Vec<Fr> = inner.instance_assignment.iter().chain(inner.witness_assignment.iter()).copied().collect();
        (full, inner.num_instance_variables)
    };
    let expected_instance = [Fr::one(), expected_public[0], expected_public[1]];
    if num_inputs != 3 || full_assignment.len() < 3 || full_assignment[..3] != expected_instance[..] {
        return Err(DriverError::Backend("instance assignment is not [1, x1, x2]".into()));
    }
    let num_constraints = matrices.num_constraints;
    Ok(SatisfiedClaim { matrices, full_assignment, num_inputs, num_constraints })
}

/// Decodes a proving-key file after checking its declared custody link and that its embedded
/// VK is the record's VK.
pub fn load_proving_key(bytes: &[u8], setup: &ParsedSetup) -> Result<ProvingKey<Bls12_381>, DriverError> {
    if bytes.len() as u64 != setup.record.proving_key_bytes {
        return Err(DriverError::mismatch("proving key size", setup.record.proving_key_bytes.to_string(), bytes.len().to_string()));
    }
    let sha = mori_r1cs::identity::digest_bytes(bytes).hex();
    if sha != setup.record.proving_key_sha256 {
        return Err(DriverError::mismatch("proving key sha256", setup.record.proving_key_sha256.clone(), sha));
    }
    let pk = encoding::decode_proving_key(bytes)?;
    if pk.vk != setup.vk {
        return Err(DriverError::mismatch("proving key embedded VK", setup.record.vk_digest_blake2b256.clone(), "a different VK"));
    }
    Ok(pk)
}

/// Public proof export and its exact bytes.
#[derive(Debug, Clone)]
pub struct ProveOutput {
    pub export: ProofExport,
    pub export_bytes: Vec<u8>,
}

/// Proves one request. Refuses before using the key unless the record names exactly the
/// relation just compiled and the request names exactly its selector.
pub fn prove<R: RngCore + CryptoRng>(
    selected: &SelectedRelation,
    setup: &ParsedSetup,
    pk: &ProvingKey<Bls12_381>,
    request: &ProveRequest,
    rng: &mut R,
) -> Result<ProveOutput, DriverError> {
    if request.selector != *selected.selector() {
        return Err(DriverError::mismatch("request selector", "the selector the relation was compiled with", "a different selector"));
    }
    relation::require_same(&setup.record.relation, selected.descriptor())?;
    if pk.vk != setup.vk {
        return Err(DriverError::mismatch("proving key embedded VK", setup.record.vk_digest_blake2b256.clone(), "a different VK"));
    }
    let claim = claim::prepare_claim(selected, &request.transition, &request.context, setup.vk_digest)?;
    let satisfied = check_claim(selected, &claim)?;

    let r = Fr::rand(rng);
    let s = Fr::rand(rng);
    let proof = Groth16::<Bls12_381>::create_proof_with_reduction_and_matrices(
        pk,
        r,
        s,
        &satisfied.matrices,
        satisfied.num_inputs,
        satisfied.num_constraints,
        &satisfied.full_assignment,
    )
    .map_err(|e| DriverError::Backend(format!("Groth16 prove: {e}")))?;
    drop(satisfied);

    if !verify::pairing_verify(&setup.vk, &proof, claim.public_inputs)? {
        return Err(DriverError::ProofRejected("self-verification of the new proof failed; nothing exported".into()));
    }
    let export = export::build_export(setup, &claim, &proof)?;
    let export_bytes = export::document_bytes(&export)?;
    // The bytes must pass the verifier's own non-cryptographic checks.
    let reparsed = export::parse_proof_export(&export_bytes)?;
    let checked = export::check_export(setup, &reparsed)?;
    if checked.statement != claim.statement || hexutil::encode(&encoding::encode_proof(&checked.proof)?) != export.proof_hex {
        return Err(DriverError::Backend("proof export does not round-trip".into()));
    }
    Ok(ProveOutput { export, export_bytes })
}
