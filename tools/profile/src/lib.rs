//! Caller-side observation using the existing recursion/backend APIs.
//!
//! No workload is run on import. Call `command` around an invocation to flush
//! even a returned-error trace. Sampling and qualified sidecars are separate.

use std::{io, path::Path};

use midnight_curves::{Bls12, Fq, G1Affine, G1Projective};
use midnight_proofs::{
    poly::kzg::params::{ParamsKZG, ParamsVerifierKZG},
    profile, profile_result, profile_span,
    transcript::{Hashable, Sampleable, TranscriptHash},
};
use midnight_zk_stdlib::{MidnightPK, MidnightVK, Relation};
use mori_backend::sherlock_child::{ChildKey, SherlockChildProof};
use mori_recursion::parent::{self, SherlockLinkRelation};
use rand_core::{CryptoRng, RngCore};

/// Retain the caller's exact result independently of observer flush failures.
/// The second result reports I/O failure without changing proof behavior.
pub fn command<T, E>(operation: impl FnOnce() -> Result<T, E>) -> (Result<T, E>, io::Result<()>) {
    let result = profile_result!(Command, Diagnostic, operation());
    (result, profile::flush())
}

/// Load a parent key using the existing digest and relation checks.
pub fn load_parent_key(
    directory: &Path,
    vk_sha256: &str,
    relation: &SherlockLinkRelation,
) -> io::Result<MidnightVK> {
    profile_result!(
        KeyLoad,
        Exclusive,
        parent::load_link_parent_key(directory, vk_sha256, relation)
    )
}

/// Decode a processed key through the recursion API.
pub fn load_key(bytes: &[u8]) -> io::Result<MidnightVK> {
    profile_result!(
        KeyLoad,
        Exclusive,
        mori_recursion::util::vk_from_bytes(bytes)
    )
}

/// Prepare once and retain the returned key for a resident-PK second proof.
pub fn setup_pk<R: Relation>(relation: &R, vk: &MidnightVK) -> MidnightPK<R> {
    profile_span!(
        SetupPk,
        Exclusive,
        midnight_zk_stdlib::setup_pk(relation, vk)
    )
}

/// Call the stock prover with the caller's exact relation, transcript and RNG.
/// This root is diagnostic: unobserved proof time remains unexplained.
pub fn prove<R: Relation, H: TranscriptHash>(
    params: &ParamsKZG<Bls12>,
    pk: &MidnightPK<R>,
    relation: &R,
    instance: &R::Instance,
    witness: R::Witness,
    rng: impl RngCore + CryptoRng,
) -> Result<Vec<u8>, R::Error>
where
    G1Projective: Hashable<H>,
    Fq: Hashable<H> + Sampleable<H>,
{
    profile_result!(
        Prove,
        Diagnostic,
        midnight_zk_stdlib::prove::<R, H>(params, pk, relation, instance, witness, rng)
    )
}

/// Call stock verification without selecting or changing the transcript.
pub fn verify<R: Relation, H: TranscriptHash>(
    params: &ParamsVerifierKZG<Bls12>,
    vk: &MidnightVK,
    instance: &R::Instance,
    committed_instance: Option<G1Affine>,
    proof: &[u8],
) -> Result<(), R::Error>
where
    G1Projective: Hashable<H>,
    Fq: Hashable<H> + Sampleable<H>,
{
    profile_result!(
        Verify,
        Exclusive,
        midnight_zk_stdlib::verify::<R, H>(params, vk, instance, committed_instance, proof)
    )
}

/// Use the backend's existing A102 statement, RNG and self-verification checks.
/// Its internal setup_pk is observed by the vendored keygen entry point.
pub fn prove_child(key: &ChildKey) -> Result<SherlockChildProof, mori_backend::Error> {
    profile_result!(Prove, Diagnostic, key.prove())
}

/// Verify through the same backend API used by the child producer.
pub fn verify_child(key: &ChildKey, proof: &SherlockChildProof) -> Result<(), mori_backend::Error> {
    profile_result!(Verify, Exclusive, key.verify(&proof.instance, &proof.proof))
}
