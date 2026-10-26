# Midnight upstream IVC verifier
> Source: https://raw.githubusercontent.com/midnightntwrk/midnight-zk/main/aggregation/src/ivc/verifier.rs
> Collected: 2026-10-26
> Published: Unknown

//! IVC verifier.
//!
//! Given an [`IvcInstance`] and the corresponding proof bytes, the
//! [`IvcVerifier`] checks that a valid chain of transitions from genesis
//! to the claimed state exists. Verification is constant-time regardless
//! of how many steps the prover has performed.
use midnight\_circuits::{hash::poseidon::PoseidonState, verifier::Accumulator};
use midnight\_proofs::{
plonk::{self},
poly::kzg::params::ParamsVerifierKZG,
transcript::{CircuitTranscript, Transcript},
};
use midnight\_zk\_stdlib::{DefaultPCS, MidnightVK, Relation};
use super::{E, F, Ivc, IvcCircuit, IvcError, IvcInstance, S, circuit::IvcExtendedInstance};
/// Lightweight IVC verifier carrying:
/// - the application context (for the decider check),
/// - the self-verifying key,
/// - the SRS verifier parameters (for the pairing check).
///
/// Returned by [`super::setup()`].
#[derive(Clone, Debug)]
pub struct IvcVerifier {
pub(crate) ctx: T::Context,
pub(crate) vk: MidnightVK,
pub(crate) params\_verifier: ParamsVerifierKZG,
}
impl IvcVerifier {
/// Verifies an IVC proof against the given instance.
///
/// Checks that the proof is valid with respect to the given instance by:
/// 1. Preparing the proof to obtain a proof accumulator.
/// 2. Accumulating it with the instance's accumulator.
/// 3. Checking the pairing invariant on the result.
/// 4. Running the application-level [`decider`](super::IvcState::decider)
/// on the instance state.
///
/// The public inputs of the verifying key are not part of `instance`: they
/// are derived from the canonical key held by this verifier (obtained via
/// [`setup`](super::setup())).
pub fn verify(&self, instance: &IvcInstance, proof: &[u8]) -> Result<(), IvcError> {
if !T::decider(&self.ctx, &instance.state) {
return Err(IvcError::DeciderFailed);
}
let fixed\_bases = midnight\_circuits::verifier::fixed\_bases::~~(self.vk.vk());
let pi = IvcCircuit::::format\_instance(&IvcExtendedInstance::new(
self.vk.vk(),
instance.clone(),
))
.map\_err(|\_| IvcError::InvalidInstance)?;
let mut transcript = CircuitTranscript::>::init\_from\_bytes(proof);
let dual\_msm = plonk::prepare::>>(
self.vk.vk(),
&[],
&[&[], &pi],
&mut transcript,
)
.map\_err(|\_| IvcError::InvalidProof)?;
transcript.assert\_empty().map\_err(|\_| IvcError::TranscriptNotEmpty)?;
let proof\_acc = Accumulator::from\_dual\_msm(dual\_msm, &fixed\_bases);
// Verify that both `proof\_acc` and `instance.acc` satisfy the pairing
// invariant, with a single pairing, by accumulating them first.
let final\_acc = Accumulator::~~::accumulate(&[proof\_acc, instance.acc.clone()]);
if !final\_acc.check(&self.params\_verifier, &fixed\_bases) {
return Err(IvcError::InvalidProof);
};
Ok(())
}
}~~~~
