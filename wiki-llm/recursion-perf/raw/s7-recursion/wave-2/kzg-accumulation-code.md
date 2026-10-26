# Axiom KZG accumulation source, mutable main
> Source: https://raw.githubusercontent.com/axiom-crypto/snark-verifier/main/snark-verifier/src/pcs/kzg/accumulation.rs
> Collected: 2026-10-26
> Published: Unknown

use crate::{
loader::{native::NativeLoader, LoadedScalar, Loader},
pcs::{kzg::KzgAccumulator, AccumulationScheme, AccumulationSchemeProver},
util::{
arithmetic::{Curve, CurveAffine, Field, MultiMillerLoop},
msm::Msm,
transcript::{TranscriptRead, TranscriptWrite},
},
Error,
};
use rand::Rng;
use std::{fmt::Debug, marker::PhantomData};
/// KZG accumulation scheme. The second generic `MOS` stands for different kind
/// of multi-open scheme.
#[derive(Clone, Debug)]
pub struct KzgAs(PhantomData<(M, MOS)>);
impl AccumulationScheme for KzgAs
where
M: MultiMillerLoop,
M::G1Affine: CurveAffine,
L: Loader,
MOS: Clone + Debug,
{
type Accumulator = KzgAccumulator;
type VerifyingKey = KzgAsVerifyingKey;
type Proof = KzgAsProof;
fn read\_proof(
vk: &Self::VerifyingKey,
instances: &[Self::Accumulator],
transcript: &mut T,
) -> Result
where
T: TranscriptRead,
{
KzgAsProof::read(vk, instances, transcript)
}
fn verify(
\_: &Self::VerifyingKey,
instances: &[Self::Accumulator],
proof: &Self::Proof,
) -> Result {
let (lhs, rhs) = instances
.iter()
.map(|accumulator| (&accumulator.lhs, &accumulator.rhs))
.chain(proof.blind.as\_ref().map(|(lhs, rhs)| (lhs, rhs)))
.unzip::<\_, \_, Vec<\_>, Vec<\_>>();
let powers\_of\_r = proof.r.powers(lhs.len());
let [lhs, rhs] = [lhs, rhs].map(|bases| {
bases
.into\_iter()
.zip(powers\_of\_r.iter())
.map(|(base, r)| Msm::::base(base) \* r)
.sum::>()
.evaluate(None)
});
Ok(KzgAccumulator::new(lhs, rhs))
}
}
/// KZG accumulation scheme proving key.
#[derive(Clone, Copy, Debug, Default)]
pub struct KzgAsProvingKey(pub Option<(C, C)>);
impl KzgAsProvingKey {
/// Initialize a [`KzgAsProvingKey`].
pub fn new(g: Option<(C, C)>) -> Self {
Self(g)
}
/// Returns if it supports zero-knowledge or not.
pub fn zk(&self) -> bool {
self.0.is\_some()
}
/// Returns [`KzgAsVerifyingKey`].
pub fn vk(&self) -> KzgAsVerifyingKey {
KzgAsVerifyingKey(self.zk())
}
}
/// KZG accumulation scheme verifying key.
#[derive(Clone, Copy, Debug, Default)]
pub struct KzgAsVerifyingKey(bool);
impl KzgAsVerifyingKey {
/// Returns if it supports zero-knowledge or not.
pub fn zk(&self) -> bool {
self.0
}
}
/// KZG accumulation scheme proof.
#[derive(Clone, Debug)]
pub struct KzgAsProof
where
C: CurveAffine,
L: Loader,
{
blind: Option<(L::LoadedEcPoint, L::LoadedEcPoint)>,
r: L::LoadedScalar,
}
impl KzgAsProof
where
C: CurveAffine,
L: Loader,
{
fn read(
vk: &KzgAsVerifyingKey,
instances: &[KzgAccumulator],
transcript: &mut T,
) -> Result
where
T: TranscriptRead,
{
assert!(!instances.is\_empty());
for accumulator in instances {
transcript.common\_ec\_point(&accumulator.lhs)?;
transcript.common\_ec\_point(&accumulator.rhs)?;
}
let blind = vk
.zk()
.then(|| Ok((transcript.read\_ec\_point()?, transcript.read\_ec\_point()?)))
.transpose()?;
let r = transcript.squeeze\_challenge();
Ok(Self { blind, r })
}
}
impl AccumulationSchemeProver for KzgAs
where
M: MultiMillerLoop,
M::G1Affine: CurveAffine,
MOS: Clone + Debug,
{
type ProvingKey = KzgAsProvingKey;
fn create\_proof(
pk: &Self::ProvingKey,
instances: &[KzgAccumulator],
transcript: &mut T,
rng: R,
) -> Result, Error>
where
T: TranscriptWrite,
R: Rng,
{
assert!(!instances.is\_empty());
for accumulator in instances {
transcript.common\_ec\_point(&accumulator.lhs)?;
transcript.common\_ec\_point(&accumulator.rhs)?;
}
let blind = pk
.zk()
.then(|| {
let s = M::Fr::random(rng);
let (g, s\_g) = pk.0.unwrap();
let lhs = (s\_g \* s).to\_affine();
let rhs = (g \* s).to\_affine();
transcript.write\_ec\_point(lhs)?;
transcript.write\_ec\_point(rhs)?;
Ok((lhs, rhs))
})
.transpose()?;
let r = transcript.squeeze\_challenge();
let (lhs, rhs) = instances
.iter()
.cloned()
.map(|accumulator| (accumulator.lhs, accumulator.rhs))
.chain(blind)
.unzip::<\_, \_, Vec<\_>, Vec<\_>>();
let powers\_of\_r = r.powers(lhs.len());
let [lhs, rhs] = [lhs, rhs].map(|msms| {
msms.iter()
.zip(powers\_of\_r.iter())
.map(|(msm, power\_of\_r)| Msm::::base(msm) \* power\_of\_r)
.sum::>()
.evaluate(None)
});
Ok(KzgAccumulator::new(lhs, rhs))
}
}