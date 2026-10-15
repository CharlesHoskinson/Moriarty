//! Optional Groth16 verification glue over BLS12-381 (feature `groth16`, arkworks 0.4).
//!
//! Never exercised by any test. This module contains no setup and no proving path:
//! a relation is circuit-specific, so every (program, action, options) needs its own
//! reviewed setup, and no VK, PK or proof from the X7 v0 runs or attempt-2 is a key for
//! any relation compiled here (orphan Class 0 VKs cannot be a real genesis).
//!
//! The direct field adapter is a test statement, not a claim ABI (see `adapter.rs`).
use crate::circuit::CompiledRelation;
use crate::error::MappingError;
use crate::statement::TransitionValues;
use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_relations::r1cs::SynthesisError;
use ark_snark::SNARK;

#[derive(Debug)]
pub enum VerifyError {
    Mapping(MappingError),
    /// The VK's input count differs from the relation's public statement.
    InputCount { vk_inputs: usize, statement: usize },
    Backend(SynthesisError),
}

/// Verifies `proof` for the claimed transition. The VK must come from a reviewed setup
/// over exactly this relation's matrices (`ProbeReport::r1cs_sha256`); this function
/// cannot check that provenance (X7 review §7, F2).
pub fn verify(relation: &CompiledRelation, vk: &VerifyingKey<Bls12_381>, values: &TransitionValues, proof: &Proof<Bls12_381>) -> Result<bool, VerifyError> {
    let public = relation.public_inputs::<Fr>(values).map_err(VerifyError::Mapping)?;
    if vk.gamma_abc_g1.len() != public.len() + 1 {
        return Err(VerifyError::InputCount { vk_inputs: vk.gamma_abc_g1.len().saturating_sub(1), statement: public.len() });
    }
    Groth16::<Bls12_381>::verify(vk, &public, proof).map_err(VerifyError::Backend)
}
