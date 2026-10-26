# Current upstream ledger proof verification source
> Source: https://raw.githubusercontent.com/midnightntwrk/midnight-ledger/main/transient-crypto/src/proofs.rs
> Collected: 2026-10-26
> Published: Unknown

// This file is part of midnight-ledger.
// Copyright (C) 2025 Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! This module provides access to creating, and verifying zero-knowledge
//! proofs. It assumes that keys and IR are generated externally, which is the
//! focus of [Compact](https://github.com/input-output-hk/compactc).
use crate::curve::{Fr, outer};
use base\_crypto::hash::{HashOutput, persistent\_hash};
use lazy\_static::lazy\_static;
use lru::LruCache;
use midnight\_curves::Bls12;
use midnight\_proofs::{
poly::kzg::params::{ParamsKZG, ParamsVerifierKZG},
utils::SerdeFormat,
};
use midnight\_zk\_stdlib::{MidnightCircuit, MidnightPK, MidnightVK, Relation};
#[cfg(feature = "proptest")]
use proptest::arbitrary::Arbitrary;
#[cfg(feature = "proptest")]
use proptest\_derive::Arbitrary;
use rand::distributions::{Distribution, Standard};
use rand::{CryptoRng, Rng};
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::Error as SerError};
use serialize::{
Deserializable, Serializable, Tagged, VecExt, tag\_enforcement\_test, tagged\_deserialize,
};
#[cfg(feature = "proptest")]
use serialize::{NoStrategy, simple\_arbitrary};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::io::{self, Read};
#[cfg(feature = "proptest")]
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::{any::Any, cmp::Ordering};
use std::{borrow::Cow, num::NonZeroUsize};
use storage\_core::Storable;
use storage\_core::arena::ArenaKey;
use storage\_core::db::DB;
use storage\_core::storable::Loader;
use zeroize::{Zeroize, ZeroizeOnDrop};
/// A provider of prover parameters.
pub trait ParamsProverProvider {
// Allowed because we don't care about auto traits here.
#[allow(async\_fn\_in\_trait)]
/// Retrieve the parameters for a given `k` value
async fn get\_params(&self, k: u8) -> io::Result;
}
/// The hash used during proof transcript processing
pub type TranscriptHash = blake2b\_simd::State;
impl ParamsProverProvider for base\_crypto::data\_provider::MidnightDataProvider {
async fn get\_params(&self, k: u8) -> io::Result {
let name = Self::name\_k(k);
let reader = self
.get\_file(
&name,
&format!("public parameters for k={k} not found in cache"),
)
.await?;
ParamsProver::read(reader)
}
}
/// A specific instance of the prover parameters.
#[derive(Clone)]
pub struct ParamsProver(pub Arc>);
impl AsRef> for ParamsProver {
fn as\_ref(&self) -> &ParamsKZG {
&self.0
}
}
impl ParamsProver {
/// Reads the prover parameters from a data stream
pub fn read(mut reader: R) -> io::Result {
Ok(ParamsProver(Arc::new(ParamsKZG::read\_custom(
&mut reader,
SerdeFormat::RawBytesUnchecked,
)?)))
}
pub(crate) fn as\_verifier(&self) -> ParamsVerifier {
ParamsVerifier(Arc::new(self.0.verifier\_params()))
}
}
/\*\*
\* The maximum degree supported by the standard verifier key.
\* This limits the number of public inputs usable.
\*/
pub const VERIFIER\_MAX\_DEGREE: u8 = 14;
/// Parameters used for verifying with the `KZG` commitment scheme
#[derive(Clone)]
pub struct ParamsVerifier(Arc>);
impl ParamsVerifier {
/// Reads in verifier parameters
pub fn read(reader: R) -> io::Result {
Ok(ParamsProver::read(reader)?.as\_verifier())
}
}
const PARAMS\_VERIFIER\_RAW: &[u8] = include\_bytes!("../static/bls\_midnight\_2p14");
lazy\_static! {
/// The midnight verifier parameters, up to [`VERIFIER\_MAX\_DEGREE`].
///
/// Note that using this \*will\* embed these into the binary at compile time, if that's not what
/// you want, please use `ParamsVerifier::read` instead.
pub static ref PARAMS\_VERIFIER: ParamsVerifier = ParamsVerifier::read(PARAMS\_VERIFIER\_RAW).expect("Static verifier parameters should be valid.");
}
/// A zero-knowledge proof.
#[cfg\_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serializable, Storable)]
#[storable(base)]
#[tag = "proof[v5]"]
pub struct Proof(pub Vec);
tag\_enforcement\_test!(Proof);
/// A prover key, used for creating proofs.
#[derive(Debug, Clone)]
pub struct ProverKey(Arc>>);
impl From> for ProverKey {
fn from(pk: MidnightPK) -> Self {
ProverKey(Arc::new(Mutex::new(InnerProverKey::Initialized(Arc::new(
pk,
)))))
}
}
/// An intermediate representation for Midnight's circuits.
#[allow(async\_fn\_in\_trait)]
pub trait Zkir: Relation + Tagged + Deserializable + Any + Send + Sync + Debug {
/// Check that a proof preimage satisfies the circuit
///
/// Returns which outputs were skipped in the proof preimage, and how many
/// zero element to buffer them with. Specifically, because our circuits
/// compile to JavaScript, and there do not evaluate untaken branches, this
/// leads to the output of the JavaScript circuit targets omitting public
/// inputs that occurred in an untaken branch. This information still needs
/// to be included in the statement vectors, where it is padded with zero
/// elements.
///
/// Currently, we handle this by grouping the statement vector into 'blocks'
/// of public inputs, with each block corresponding to exactly one VM
/// instruction, and running `check` to figure out which blocks were
/// omitted due to untaken branches, and how many zeros to pad them with.
///
/// Long-term, we probably want to move to make this obsolete, by having the
/// computer target gather information about untaken branches at run-time.
fn check(&self, preimage: &ProofPreimage) -> Result>, ProvingError>;
/// Proves a circuit.
/// Returns the proof, the statement vector, and the skips from `check`.
async fn prove(
&self,
rng: impl Rng + CryptoRng,
params: &impl ParamsProverProvider,
pk: ProverKey,
preimage: &ProofPreimage,
) -> Result<(Proof, Vec, Vec>), ProvingError>;
/// Returns the k value for this circuit
fn k(&self) -> u8 {
MidnightCircuit::from\_relation(self).min\_k() as u8
}
/// Performs key generation on this circuit, outputting the verifier key
async fn keygen\_vk(
&self,
params: &impl ParamsProverProvider,
) -> Result {
use midnight\_zk\_stdlib::setup\_vk;
let vk = VerifierKey::from(setup\_vk(params.get\_params(self.k()).await?.as\_ref(), self));
Ok(vk)
}
/// Performs key generation on this circuit, outputting the prover/verifier
/// key pair
async fn keygen(
&self,
params: &impl ParamsProverProvider,
) -> Result<(ProverKey, VerifierKey), anyhow::Error> {
use midnight\_zk\_stdlib::{setup\_pk, setup\_vk};
let vk = setup\_vk(params.get\_params(self.k()).await?.as\_ref(), self);
let pk = setup\_pk(self, &vk);
Ok((ProverKey::from(pk), VerifierKey::from(vk)))
}
}
impl PartialEq for ProverKey {
fn eq(&self, other: &Self) -> bool {
let mut self\_ser = Vec::new();
let mut other\_ser = Vec::new();
Serializable::serialize(self, &mut self\_ser).expect("In-memory serialization must succeed");
Serializable::serialize(other, &mut other\_ser)
.expect("In-memory serialization must succeed");
self\_ser == other\_ser
}
}
impl Eq for ProverKey {}
impl Distribution> for Standard {
fn sample(&self, rng: &mut R) -> ProverKey {
let size: u8 = rng.gen\_range(0..32);
let mut bytes = Vec::with\_bounded\_capacity(size as usize);
rng.fill\_bytes(&mut bytes);
ProverKey(Arc::new(Mutex::new(InnerProverKey::Uninitialized(bytes))))
}
}
#[derive(Debug, Clone)]
pub(crate) enum InnerProverKey {
Uninitialized(Vec),
Invalid(Vec),
Initialized(Arc>),
}
impl Tagged for ProverKey {
fn tag() -> Cow<'static, str> {
Cow::Owned(format!("prover-key[v7]({})", T::tag()))
}
fn tag\_unique\_factor() -> String {
format!("prover-key[v7]({})", T::tag())
}
}
const PK\_COMPRESSION\_LEVEL: u32 = 6;
const PK\_CACHE\_SIZE: usize = 5;
lazy\_static! {
// forall Arc>
static ref PK\_CACHE: Mutex>> =
Mutex::new(LruCache::new(NonZeroUsize::new(PK\_CACHE\_SIZE).unwrap()));
}
impl InnerProverKey {
fn try\_cache(&mut self) {
let hash = match self {
InnerProverKey::Uninitialized(data) => persistent\_hash(&data[..]),
\_ => return,
};
if let Some(pk) = PK\_CACHE
.lock()
.ok()
.and\_then(|mut c| c.get(&hash).cloned())
.and\_then(|ptr| ptr.downcast().ok())
{
\*self = InnerProverKey::Initialized(pk);
}
}
}
impl ProverKey {
/// Initializes the lazy prover key
pub fn init(&self) -> Result>, ProvingError> {
let mut mutex = self.0.lock().expect("mutex is not poisoned");
mutex.try\_cache();
let data = match &\*mutex {
InnerProverKey::Initialized(key) => {
return Ok(key.clone());
}
InnerProverKey::Invalid(\_) => {
return Err(anyhow::anyhow!("known invalid verifier key"));
}
InnerProverKey::Uninitialized(data) => data.clone(),
};
let inner\_reader = &mut &data[..];
let mut reader = flate2::read::GzDecoder::new(inner\_reader);
let read\_inner = |reader| {
let pk = MidnightPK::::read(reader, SerdeFormat::RawBytesUnchecked)?;
Ok(pk)
};
let res: Result<\_, ProvingError> = read\_inner(&mut reader);
match res {
Ok(pk) => {
let key = Arc::new(pk);
PK\_CACHE
.lock()
.ok()
.and\_then(|mut c| c.put(persistent\_hash(&data), key.clone()));
\*mutex = InnerProverKey::Initialized(key.clone());
Ok(key)
}
Err(e) => {
\*mutex = InnerProverKey::Invalid(data);
Err(e)
}
}
}
fn inner\_serialize(&self, mut writer: W) -> std::io::Result<()> {
match &\*self.0.lock().expect("mutex is not poisoned") {
InnerProverKey::Uninitialized(data) | InnerProverKey::Invalid(data) => {
writer.write\_all(data)?;
Ok(())
}
InnerProverKey::Initialized(key) => {
let mut writer = flate2::write::GzEncoder::new(
writer,
flate2::Compression::new(PK\_COMPRESSION\_LEVEL),
);
key.write(&mut writer, SerdeFormat::RawBytesUnchecked)
}
}
}
}
struct Count(usize);
impl std::io::Write for Count {
fn write(&mut self, buf: &[u8]) -> std::io::Result {
self.0 += buf.len();
Ok(buf.len())
}
fn flush(&mut self) -> std::io::Result<()> {
Ok(())
}
}
impl Serializable for ProverKey {
fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()> {
let mut count = Count(0);
self.inner\_serialize(&mut count).ok();
Serializable::serialize(&(count.0 as u64), writer)?;
self.inner\_serialize(writer)
}
fn serialized\_size(&self) -> usize {
let mut writer = Count(0);
self.inner\_serialize(&mut writer).ok();
(writer.0 as u64).serialized\_size() + writer.0
}
}
impl Deserializable for ProverKey {
fn deserialize(reader: &mut impl Read, recursion\_depth: u32) -> Result {
let buf =  as Deserializable>::deserialize(reader, recursion\_depth)?;
let mut pk = InnerProverKey::Uninitialized(buf);
pk.try\_cache();
Ok(Self(Arc::new(Mutex::new(pk))))
}
}
/// A verifier key, used for checking proofs.
#[derive(Debug, Storable)]
#[storable(base)]
pub struct VerifierKey(Arc>);
#[cfg(feature = "proptest")]
simple\_arbitrary!(VerifierKey);
impl Tagged for VerifierKey {
fn tag() -> Cow<'static, str> {
Cow::Borrowed("verifier-key[v6]")
}
fn tag\_unique\_factor() -> String {
"verifier-key[v6]".into()
}
}
tag\_enforcement\_test!(VerifierKey);
impl Distribution for Standard {
fn sample(&self, rng: &mut R) -> VerifierKey {
let size: u8 = rng.r#gen();
let mut bytes = Vec::with\_bounded\_capacity(size as usize);
rng.fill\_bytes(&mut bytes);
VerifierKey(Arc::new(Mutex::new(InnerVerifierKey::Uninitialized(bytes))))
}
}
impl From for VerifierKey {
fn from(vk: MidnightVK) -> Self {
VerifierKey(Arc::new(Mutex::new(InnerVerifierKey::Initialized(vk))))
}
}
#[derive(Debug, Clone)]
#[allow(dead\_code)] // Some features don't try to initialize
#[allow(clippy::large\_enum\_variant)]
pub(crate) enum InnerVerifierKey {
Uninitialized(Vec),
Invalid(Vec),
Initialized(MidnightVK),
}
impl Clone for VerifierKey {
fn clone(&self) -> Self {
Self(self.0.clone())
}
}
impl Deserializable for VerifierKey {
fn deserialize(
reader: &mut impl std::io::Read,
recursion\_depth: u32,
) -> Result {
const MAX\_EXPECTED\_SIZE: usize = 50\_000;
let buf =  as Deserializable>::deserialize(reader, recursion\_depth)?;
if buf.len() > MAX\_EXPECTED\_SIZE {
return Err(std::io::Error::new(
std::io::ErrorKind::InvalidData,
format!(
"Declared vk size {} exceeded permitted limit of {MAX\_EXPECTED\_SIZE}",
buf.len()
),
));
}
Ok(Self(Arc::new(Mutex::new(InnerVerifierKey::Uninitialized(
buf,
)))))
}
}
#[derive(Clone)]
struct DummyRelation;
// TODO: This is a temporary workaround for verifier key deserialization.
// Longer-term, we'll need to store information about the circuit architecture
// in the verifier key, and use that for deserializing, but those API endpoints
// do not currently exist in midnight-circuits.
impl Relation for DummyRelation {
type Instance = Vec;
type Witness = ();
fn format\_instance(
instance: &Self::Instance,
) -> Result, midnight\_proofs::plonk::Error> {
Ok(instance.clone())
}
fn circuit(
&self,
\_std\_lib: &midnight\_zk\_stdlib::ZkStdLib,
\_layouter: &mut impl midnight\_proofs::circuit::Layouter,
\_instance: midnight\_proofs::circuit::Value,
\_witness: midnight\_proofs::circuit::Value,
) -> Result<(), midnight\_proofs::plonk::Error> {
unimplemented!("should not attempt to execute dummy relation")
}
fn read\_relation(\_reader: &mut R) -> io::Result {
unimplemented!("should not attempt to read dummy relation")
}
fn write\_relation(&self, \_writer: &mut W) -> io::Result<()> {
unimplemented!("should not attempt to write dummy relation")
}
}
impl Serialize for VerifierKey {
fn serialize(&self, ser: S) -> Result {
let mut vec = Vec::new();
::serialize(self, &mut vec).map\_err(S::Error::custom)?;
ser.serialize\_bytes(&vec)
}
}
impl<'de> Deserialize<'de> for VerifierKey {
fn deserialize>(deserializer: D) -> Result {
let bytes = serde\_bytes::ByteBuf::deserialize(deserializer)?;
::deserialize(&mut &bytes[..], 0)
.map\_err(serde::de::Error::custom)
}
}
#[allow(clippy::derived\_hash\_with\_manual\_eq)]
impl Hash for VerifierKey {
fn hash(&self, state: &mut H) {
let mut data = Vec::new();
Serializable::serialize(&self, &mut data).ok();
state.write(&data);
}
}
impl Serializable for VerifierKey {
fn serialize(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
let mut count = Count(0);
self.inner\_serialize(&mut count).ok();
Serializable::serialize(&(count.0 as u64), writer)?;
self.inner\_serialize(writer)
}
fn serialized\_size(&self) -> usize {
let mut writer = Count(0);
self.inner\_serialize(&mut writer).ok();
(writer.0 as u64).serialized\_size() + writer.0
}
}
impl VerifierKey {
/// Initializes the lazy verifier key
pub fn init(&self) -> Result<(), VerifyingError> {
self.force\_init()?;
Ok(())
}
// warning! This grabs the lock! Make sure to drop the result before re-running!
#[allow(dead\_code)] // Some features don't try to initialize
pub(crate) fn force\_init(&self) -> Result {
let mut mutex = self.0.lock().expect("mutex is not poisoned");
let data = match &\*mutex {
InnerVerifierKey::Initialized(key) => {
return Ok(key.clone());
}
InnerVerifierKey::Invalid(\_) => {
return Err(anyhow::anyhow!("known invalid verifier key"));
}
InnerVerifierKey::Uninitialized(data) => data.clone(),
};
let reader = &mut &data[..];
let vk = MidnightVK::read(reader, SerdeFormat::Processed)
.map\_err(|\_| anyhow::anyhow!("problem reading the verifier key"))?;
\*mutex = InnerVerifierKey::Initialized(vk.clone());
Ok(vk)
}
fn inner\_serialize(&self, mut writer: W) -> std::io::Result<()> {
match &\*self.0.lock().expect("mutex is not poisoned") {
InnerVerifierKey::Uninitialized(data) | InnerVerifierKey::Invalid(data) => {
writer.write\_all(data)
}
InnerVerifierKey::Initialized(key) => key.write(&mut writer, SerdeFormat::Processed),
}
}
/// Checks a proof against a statement.
pub fn verify>(
&self,
params: &ParamsVerifier,
proof: &Proof,
statement: F,
) -> Result<(), VerifyingError> {
let vk = self.force\_init()?;
let pi = statement.map(|f| f.0).collect::>();
trace!(statement = ?pi, "verifying proof against statement");
midnight\_zk\_stdlib::verify::(
¶ms.0, &vk, &pi, None, &proof.0,
)
.map\_err(|\_| anyhow::anyhow!("Invalid proof"))
}
/// Mocks the checking of a proof against a statement
///
/// We do this by running a number of CPU burn cycles calculated to be approximately
/// equivalent in time-taken to real proof verification
#[cfg(feature = "mock-verify")]
pub fn mock\_verify>(&self, statement: F) -> Result<(), VerifyingError> {
let pi\_len = statement.count();
crate::mock\_verify::mock\_verify\_for(pi\_len)
}
/// Checks a sequence of proofs against their corresponding statements and verifier keys
pub fn batch\_verify<
'a,
F: Iterator,
V: Iterator,
>(
params: &ParamsVerifier,
parts: V,
) -> Result<(), VerifyingError> {
use midnight\_zk\_stdlib::batch\_verify;
let mut vks = vec![];
let mut pis = vec![];
let mut proofs = vec![];
for (vk, proof, stmt) in parts.into\_iter() {
let pi = stmt.map(|f| f.0).collect::>();
let vk = vk.force\_init()?;
vks.push(vk);
pis.push(pi);
proofs.push(proof.0.clone());
}
batch\_verify::(¶ms.0, &vks, &pis, &proofs)
.map\_err(|\_| anyhow::anyhow!("Invalid proof"))
}
/// Mocks the checking of a sequence of proofs against a statement
///
/// This is simulated by sequentially mocking each individual verification,
/// it doesn't currently benefit from any performance benefits one should associate
/// with batching
#[cfg(feature = "mock-verify")]
pub fn mock\_batch\_verify<
'a,
F: Iterator,
V: Iterator,
>(
parts: V,
) -> Result<(), VerifyingError> {
for (vk, \_proof, stmt) in parts {
vk.mock\_verify(stmt)?;
}
Ok(())
}
}
/// A hint on where keys for a circuit can be found.
///
/// Circuit keys are associated with a string name, and are resolved at proving
/// time against a hash table of provided keys.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serializable)]
#[cfg\_attr(feature = "proptest", derive(Arbitrary))]
pub struct KeyLocation(pub Cow<'static, str>);
impl Zeroize for KeyLocation {
fn zeroize(&mut self) {
if let Cow::Owned(s) = &mut self.0 {
s.zeroize();
}
self.0 = Cow::Borrowed("");
}
}
impl Tagged for KeyLocation {
fn tag() -> Cow<'static, str> {
Cow::Borrowed("string")
}
fn tag\_unique\_factor() -> String {
"string".into()
}
}
#[derive(Serializable)]
#[tag = "wrapped-ir"]
/// A container for just the IR part of [`ProofData`].
pub struct WrappedIr(pub Vec);
tag\_enforcement\_test!(WrappedIr);
#[derive(Clone, Serializable)]
#[tag = "proving-data"]
/// A container for the parts required for proving
pub struct ProvingKeyMaterial {
/// The prover key
pub prover\_key: Vec,
/// The verifier key
pub verifier\_key: Vec,
/// The IR source
pub ir\_source: Vec,
}
tag\_enforcement\_test!(ProvingKeyMaterial);
/// A mechanism to retrieve / resolve zero-knowledge key material from a short location string.
pub trait Resolver {
/// Resolves the given key to the key material it represents, if available.
// Allowed as we do not need auto traits here
#[allow(async\_fn\_in\_trait)]
async fn resolve\_key(&self, key: KeyLocation) -> io::Result>;
}
/// A tool that provides proving against opaque/serialized proof preimages
/// It is assumed (though not strictly required) that this also implements
/// `Resolver` to resolve keys.
#[allow(async\_fn\_in\_trait)]
pub trait ProvingProvider {
/// Check the proof preimage is valid, and if so returns the pi skip sequence
async fn check(&self, preimage: &ProofPreimage) -> Result>, anyhow::Error>;
/// Produces the proof, optionally modifying the binding input in the proof preimage first.
async fn prove(
self,
preimage: &ProofPreimage,
overwrite\_binding\_input: Option,
) -> Result;
/// Creates a copy of this provider. As providers often include an RNG, this
/// may mutate the provider itself.
fn split(&mut self) -> Self;
}
/// Everything necessary to produce a proof.
#[derive(
Clone,
Debug,
PartialEq,
Eq,
PartialOrd,
Ord,
Serializable,
Hash,
Storable,
Zeroize,
ZeroizeOnDrop,
)]
#[storable(base)]
#[tag = "proof-preimage"]
#[cfg\_attr(feature = "proptest", derive(Arbitrary))]
pub struct ProofPreimage {
/// The inputs to be directly handed to the IR.
pub inputs: Vec,
/// A private witness vector consumed by active witness calls in the IR.
pub private\_transcript: Vec,
/// A public statement vector encoding statement call information in the IR.
pub public\_transcript\_inputs: Vec,
/// A public statement vector encoding statement call results in the IR.
pub public\_transcript\_outputs: Vec,
/// An arbitrary input to be bound to in the proof.
pub binding\_input: Fr,
/// The communications commitment that will be checked, and its randomness.
/// May be [None], in which case inputs and outputs are not committed to.
pub communications\_commitment: Option<(Fr, Fr)>,
/// Where the keys for carrying out the proving can be found.
pub key\_location: KeyLocation,
}
tag\_enforcement\_test!(ProofPreimage);
impl ProofPreimage {
/// Runs witness generation and checks for correctness without generating a
/// proof
#[allow(unused\_variables)]
pub fn check(&self, ir: &impl Zkir) -> Result>, ProvingError> {
ir.check(self)
}
/// Carries out the actual proving of the proof preimage.
#[allow(unreachable\_code, unused\_variables)]
pub async fn prove(
&self,
rng: impl Rng + CryptoRng,
params: &impl ParamsProverProvider,
resolver: &impl Resolver,
) -> Result<(Proof, Vec>), ProvingError> {
let proof\_data = resolver
.resolve\_key(self.key\_location.clone())
.await?
.ok\_or(anyhow::Error::msg(format!(
"failed to find proving key for '{}'",
&self.key\_location.0
)))?;
let ir = tagged\_deserialize::(&mut &proof\_data.ir\_source[..])?;
let verifier\_key = tagged\_deserialize::(&mut &proof\_data.verifier\_key[..])?;
let prover\_key = tagged\_deserialize::>(&mut &proof\_data.prover\_key[..])?;
let (proof, pis, pi\_skips) = ir.prove(rng, params, prover\_key, self).await?;
debug!("proof created; verifying to make sure");
let k = verifier\_key.force\_init()?.k();
if let Err(e) = verifier\_key.verify(
¶ms.get\_params(k).await?.as\_verifier(),
&proof,
pis.iter().copied(),
) {
error!(error = ?e, ?pis, ?ir, "self-verification failed! This may be a bug, check that your keys match!");
return Err(e);
}
debug!("proof ok");
Ok((proof, pi\_skips))
}
}
impl PartialEq for VerifierKey {
fn eq(&self, other: &Self) -> bool {
let mut self\_ser = Vec::new();
let mut other\_ser = Vec::new();
Serializable::serialize(self, &mut self\_ser).expect("In-memory serialization must succeed");
Serializable::serialize(other, &mut other\_ser)
.expect("In-memory serialization must succeed");
self\_ser == other\_ser
}
}
impl Eq for VerifierKey {}
impl PartialOrd for VerifierKey {
fn partial\_cmp(&self, other: &Self) -> Option {
Some(self.cmp(other))
}
}
impl Ord for VerifierKey {
fn cmp(&self, other: &Self) -> Ordering {
let mut self\_ser = Vec::new();
let mut other\_ser = Vec::new();
Serializable::serialize(self, &mut self\_ser).expect("In-memory serialization must succeed");
Serializable::serialize(other, &mut other\_ser)
.expect("In-memory serialization must succeed");
self\_ser.cmp(&other\_ser)
}
}
/// An error during proving. The type of this should not be considered part of
/// the public API, although it may be assumed to be [`Debug`]` +
/// `[`Display`](std::fmt::Display).
pub type ProvingError = anyhow::Error;
/// An error during verifying. The type of this should not be considered part of
/// the public API, although it may be assumed to be [`Debug`]` +
/// `[`Display`](std::fmt::Display).
pub type VerifyingError = anyhow::Error;
