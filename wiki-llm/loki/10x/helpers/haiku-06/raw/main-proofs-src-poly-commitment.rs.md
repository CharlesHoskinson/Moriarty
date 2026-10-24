//! Trait for a commitment scheme
use core::ops::{Add, Mul};
use std::{
fmt::Debug,
hash::Hash,
io::{self, Read},
};
use ff::PrimeField;
use crate::{
poly::{
Error, Polynomial, PolynomialRepresentation, ProverQuery, VerifierQuery,
query::PolynomialLabel,
},
transcript::{Hashable, Sampleable, Transcript},
utils::helpers::{ProcessedSerdeObject, SerdeFormat},
};
/// Public interface for a additively homomorphic Polynomial Commitment Scheme
/// (PCS)
pub trait PolynomialCommitmentScheme: Clone + Debug {
/// Parameters needed to generate a proof in the PCS
type Parameters: Send + Sync;
/// Parameters needed to verify a proof in the PCS
type VerifierParameters;
/// Type of a committed polynomial
type Commitment: Clone
+ Debug
+ Default
+ PartialEq
+ ProcessedSerdeObject
+ Send
+ Sync
+ Add
+ Mul;
/// Verification guard. Allows for batch verification
type VerificationGuard: Guard;
/// Generates the parameters of the polynomial commitment scheme, for
/// committing to polynomials of degree strictly less than `2^k`.
fn gen\_params(k: u32) -> Self::Parameters;
/// Reads the parameters of the polynomial commitment scheme from `reader`,
/// for committing to polynomials of degree strictly less than `2^k`.
fn load\_params(
reader: &mut R,
format: SerdeFormat,
k: u32,
) -> io::Result;
/// Returns the largest `k` such that `params` can commit to polynomials of
/// degree strictly less than `2^k`.
fn max\_k(params: &Self::Parameters) -> u32;
/// Extract the `VerifierParameters` from `Parameters`
fn get\_verifier\_params(params: &Self::Parameters) -> Self::VerifierParameters;
/// Commit to several polynomials, tagging the result with the
/// corresponding labels for identification during multi-open accumulation.
///
/// The polynomials are committed to in the order given, which must be the
/// order in which [`read\_commitment`](Self::read\_commitment) and
/// [`deserialize\_commitment`](Self::deserialize\_commitment) receive their
/// labels.
///
/// Committing to no polynomials does not fail: it returns an empty
/// commitment, which [`write\_commitment`](Self::write\_commitment) does not
/// write to the transcript. An empty commitment holds no polynomial, so it
/// cannot be queried: the multi-open rejects any query against it.
///
/// # Panics
///
/// Panics if `polynomials` and `labels` have different lengths, or if a
/// label is repeated.
fn commit\_many(
params: &Self::Parameters,
polynomials: &[&Polynomial],
labels: &[PolynomialLabel],
) -> Self::Commitment;
/// Commit to a single polynomial in coefficient form, tagging the result
/// with `label`. Convenience wrapper around
/// [`commit\_many`](Self::commit\_many).
fn commit(
params: &Self::Parameters,
polynomial: &Polynomial,
label: PolynomialLabel,
) -> Self::Commitment {
Self::commit\_many(params, &[polynomial], &[label])
}
/// Read a commitment to `labels.len()` polynomials from the proof
/// transcript, absorbing it into the transcript state and tagging each
/// polynomial with its label.
///
/// The labels are matched to the points read in the order given, that of
/// [`commit\_many`](Self::commit\_many).
///
/// Use [`deserialize\_commitment`](Self::deserialize\_commitment) instead for
/// commitments that are not part of the proof.
///
/// # Panics
///
/// Panics if a label is repeated. Labels name the polynomials the
/// verifying key expects, so a repeat is a caller bug, not a malformed
/// proof.
fn read\_commitment(
transcript: &mut T,
labels: &[PolynomialLabel],
) -> io::Result
where
Self::Commitment: Hashable;
/// Write a commitment produced by [`commit\_many`](Self::commit\_many) to the
/// proof transcript, absorbing it in exactly the granularity in which
/// [`read\_commitment`](Self::read\_commitment) reads it back.
///
/// A commitment to no polynomials writes and absorbs nothing, and
/// [`read\_commitment`](Self::read\_commitment) with no labels reads nothing.
fn write\_commitment(
transcript: &mut T,
commitment: &Self::Commitment,
) -> io::Result<()>
where
Self::Commitment: Hashable,
{
transcript.write(commitment)
}
/// Deserialize a commitment to `labels.len()` polynomials from `reader`,
/// tagging each polynomial with its label.
///
/// Unlike [`read\_commitment`](Self::read\_commitment), the bytes come from a
/// plain reader, typically a serialized verifying key, and nothing is
/// absorbed into a transcript.
///
/// # Panics
///
/// Panics if a label is repeated.
fn deserialize\_commitment(
reader: &mut R,
format: SerdeFormat,
labels: &[PolynomialLabel],
) -> io::Result;
/// Squeeze the evaluation point used by the protocol to open committed
/// polynomials. The default implementation simply squeezes a challenge,
/// but specific PCS may require squeezing challenges satisfying certain
/// properties, for example fflonk requires the evaluation point to be a
/// `t`-th power in the field.
///
/// The protocol must squeeze evaluation points through this method.
fn squeeze\_evaluation\_point(transcript: &mut T) -> F
where
F: Sampleable,
{
transcript.squeeze\_challenge()
}
/// Create a multi-opening proof at a set of [ProverQuery]'s.
///
/// The evaluations of the queries are already in the transcript.
fn multi\_open(
params: &Self::Parameters,
prover\_query: &[ProverQuery],
transcript: &mut T,
) -> Result<(), Error>
where
F: Sampleable + Hash + Ord + Hashable,
Self::Commitment: Hashable;
/// The labels `commitment` tags its polynomials with.
fn commitment\_labels(commitment: &Self::Commitment) -> Vec;
/// Total byte length when committing to `n` polynomials, which is 0 when
/// `n` is 0.
///
/// For schemes that commit each polynomial independently (e.g. KZG), this
/// equals `n` times the per-commitment size. Override for schemes that fold
/// `n` polynomials into a single proof element (e.g. fflonk).
fn commitment\_byte\_length(n: usize) -> usize {
n \* Self::Commitment::default().byte\_length(SerdeFormat::Processed)
}
/// Verify an multi-opening proof for a given set of [VerifierQuery]'s.
/// The function fails if the transcript has trailing bytes.
fn multi\_prepare<'com, T: Transcript>(
verifier\_query: &[VerifierQuery<'com, F, Self>],
transcript: &mut T,
) -> Result
where
F: Sampleable + Hash + Ord + Hashable,
Self::Commitment: Hashable + 'com;
}
/// A [`PolynomialCommitmentScheme`] that commits to polynomials interleaved
/// in chunks.
pub trait InterleavedCommit: PolynomialCommitmentScheme {
/// Commit to `g(X) = Σ\_i X^i f\_i(X^t)` for every chunk `f\_0, ..., f\_{k-1}`
/// of `chunks`, with `t` the next power of two of `k`, tagging the `g` of
/// each chunk with the corresponding label.
///
/// # Panics
///
/// Panics if `chunks` and `labels` have different lengths, if a label is
/// repeated, or if the polynomials of a chunk have different lengths.
fn commit\_interleaved(
params: &Self::Parameters,
chunks: &[&[&Polynomial]],
labels: &[PolynomialLabel],
) -> Self::Commitment;
}
/// Interface for verifier finalizer, given the verifier parameters `VP` of
/// its PCS
pub trait Guard: Sized {
/// Finalize the verification guard
fn verify(self, params: &VP) -> Result<(), Error>;
/// Finalize a batch of verification guards
fn batch\_verify<'a, I, J>(guards: I, params: J) -> Result<(), Error>
where
I: ExactSizeIterator,
J: ExactSizeIterator,
VP: 'a,
{
assert\_eq!(guards.len(), params.len());
guards
.into\_iter()
.zip(params)
.try\_for\_each(|(guard, params)| guard.verify(params))
}
}