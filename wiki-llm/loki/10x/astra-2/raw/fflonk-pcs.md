//! fflonk over a generic inner polynomial commitment scheme.
//!
//! Let `t` be a power of two. fflonk commits to a group of polynomials
//! `f\_0, ..., f\_{t-1}` of degree < `n` by committing to a meta polynomial:
//!
//! ```text
//! g(X) = Σ\_i X^i f\_i(X^t)
//! ```
//!
//! with the inner PCS commitment scheme. Note that `g` has degree < `t · n`.
//!
//! In order to "open" the committed polynomials `f\_i` at evaluation point `x`,
//! we open `g` at the `t`-th roots of `x` instead. From the evaluations of `g`
//! at such roots, one can recompute (univocally) the evaluations of `f\_i` at
//! `x`. Note that for every root `r`:
//!
//! ```text
//! g(r) = Σ\_i r^i f\_i(x)
//! ```
//!
//! which gives a Vandermonde system of `t` equations. Conversely, the verifier,
//! given the evaluations of the `f\_i` at `x`, computes those of `g` at the
//! roots and checks them with the inner PCS.
//!
//! # Notes
//!
//! - The verifier needs every `f\_i(x)`, so the evaluations at `x` of the `f\_i`
//! not queried at `x` are written to the proof.
//!
//! - A group of `k` polynomials, for `k` not a power of two, is padded with
//! dummy polynomials up to the next power of two `t`.
//!
//! - The degree of `g` grows with `t`, so `t` is bounded by `T\_MAX`: the
//! polynomials of a `commit\_many` call are split into chunks of at most
//! `T\_MAX` polynomials.
mod utils;
use std::{
collections::{BTreeMap, HashMap, HashSet, hash\_map::Entry},
hash::Hash,
io::{self, Read},
marker::PhantomData,
slice,
};
use ff::WithSmallOrderMulGroup;
pub use utils::roots;
use utils::{cached\_roots, compute\_g};
use crate::{
poly::{
Error, Polynomial, PolynomialLabel,
PolynomialLabel::Collection,
PolynomialRepresentation, ProverQuery, VerifierQuery,
commitment::{InterleavedCommit, PolynomialCommitmentScheme},
},
transcript::{Hashable, Sampleable, Transcript},
utils::{
arithmetic::{eval\_polynomial, eval\_polynomial\_seq},
helpers::SerdeFormat,
},
};
/// fflonk over the polynomial commitment scheme `PCS`.
///
/// The polynomials of a `commit\_many` call are combined in chunks of at most
/// `T\_MAX = 2^LOG2\_T\_MAX`.
#[derive(Clone, Debug)]
pub struct Fflonk(PhantomData);
impl Fflonk {
/// The maximum number of polynomials combined into one.
const T\_MAX: usize = 1 << LOG2\_T\_MAX;
}
impl PolynomialCommitmentScheme for Fflonk
where
F: WithSmallOrderMulGroup<3>,
PCS: InterleavedCommit,
{
type Parameters = PCS::Parameters;
type VerifierParameters = PCS::VerifierParameters;
type Commitment = PCS::Commitment;
type VerificationGuard = PCS::VerificationGuard;
fn gen\_params(k: u32) -> Self::Parameters {
PCS::gen\_params(k + LOG2\_T\_MAX)
}
fn load\_params(
reader: &mut R,
format: SerdeFormat,
k: u32,
) -> io::Result {
PCS::load\_params(reader, format, k + LOG2\_T\_MAX)
}
fn max\_k(params: &Self::Parameters) -> u32 {
PCS::max\_k(params).saturating\_sub(LOG2\_T\_MAX)
}
fn get\_verifier\_params(params: &Self::Parameters) -> Self::VerifierParameters {
PCS::get\_verifier\_params(params)
}
fn commit\_many(
params: &Self::Parameters,
polynomials: &[&Polynomial],
labels: &[PolynomialLabel],
) -> Self::Commitment {
PolynomialLabel::assert\_distinct(labels);
let g\_labels: Vec<\_> = labels.chunks(Self::T\_MAX).map(|l| Collection(l.to\_vec())).collect();
let chunks: Vec<\_> = polynomials.chunks(Self::T\_MAX).collect();
PCS::commit\_interleaved(params, &chunks, &g\_labels)
}
fn read\_commitment(
transcript: &mut T,
labels: &[PolynomialLabel],
) -> io::Result
where
Self::Commitment: Hashable,
{
PolynomialLabel::assert\_distinct(labels);
let g\_labels: Vec<\_> = labels.chunks(Self::T\_MAX).map(|l| Collection(l.to\_vec())).collect();
PCS::read\_commitment(transcript, &g\_labels)
}
fn write\_commitment(
transcript: &mut T,
commitment: &Self::Commitment,
) -> io::Result<()>
where
Self::Commitment: Hashable,
{
PCS::write\_commitment(transcript, commitment)
}
fn deserialize\_commitment(
reader: &mut R,
format: SerdeFormat,
labels: &[PolynomialLabel],
) -> io::Result {
PolynomialLabel::assert\_distinct(labels);
let g\_labels: Vec<\_> = labels.chunks(Self::T\_MAX).map(|l| Collection(l.to\_vec())).collect();
PCS::deserialize\_commitment(reader, format, &g\_labels)
}
fn squeeze\_evaluation\_point(transcript: &mut T) -> F
where
F: Sampleable,
{
PCS::squeeze\_evaluation\_point(transcript).pow\_vartime([Self::T\_MAX as u64])
}
fn multi\_open(
params: &Self::Parameters,
queries: &[ProverQuery],
transcript: &mut T,
) -> Result<(), Error>
where
F: Sampleable + Hash + Ord + Hashable,
Self::Commitment: Hashable,
{
// Maps the labels of every queried chunk to its polynomials and the
// points they are queried at, in query order: `labels -> (polys, points)`.
let chunks\_info = queries.iter().fold(BTreeMap::new(), |mut chunks\_info, q| {
let (labels, polys) =
(q.group\_labels.chunks(Self::T\_MAX).zip(q.group\_polys.chunks(Self::T\_MAX)))
.find(|(labels, \_)| labels.contains(&q.label))
.expect("the queried group has no polynomial under the query label");
let (\_, points) = (chunks\_info.entry(labels))
.or\_insert\_with(|| (polys.iter().collect::>(), Vec::new()));
if !points.contains(&q.point) {
points.push(q.point);
}
chunks\_info
});
// fflonk requires that every polynomial of a chunk be queried at every point of
// the chunk. We compute the missing evaluations (those that were not queried)
// and write them to the transcript. Note that the evaluations of explicit
// queries are expected to be in the transcript before the call to `multi\_open`.
let queried: HashSet<\_> = queries.iter().map(|q| (q.label.clone(), q.point)).collect();
for (labels, (polys, points)) in &chunks\_info {
for point in points {
for (label, poly) in labels.iter().zip(polys.iter()) {
if !queried.contains(&(label.clone(), \*point)) {
let eval = eval\_polynomial(poly, \*point);
transcript.write(&eval).map\_err(|\_| Error::OpeningError)?;
}
}
}
}
// Compute g for every chunk as in `commit\_many`. This step needs to be
// performed again, since the PCS is stateless, but it is cheap here.
let g\_labels: Vec<\_> = chunks\_info.keys().map(|l| [Collection(l.to\_vec())]).collect();
let gs: Vec<\_> = chunks\_info.values().map(|(polys, \_)| compute\_g(polys)).collect();
// Opening a chunk at `x` amounts to opening its `g` at the `t`-th roots of `x`.
let mut inner\_queries = Vec::new();
let mut roots\_cache = HashMap::new();
for (((polys, points), g\_label), g) in chunks\_info.values().zip(&g\_labels).zip(&gs) {
let t = polys.len().next\_power\_of\_two();
for x in points {
for root in cached\_roots(&mut roots\_cache, \*x, t).ok\_or(Error::OpeningError)? {
let g\_poly = slice::from\_ref(g);
inner\_queries.push(ProverQuery::new(
g\_label,
g\_poly,
\*root,
g\_label[0].clone(),
));
}
}
}
PCS::multi\_open(params, &inner\_queries, transcript)
}
fn commitment\_labels(commitment: &Self::Commitment) -> Vec {
PCS::commitment\_labels(commitment)
.into\_iter()
.flat\_map(|label| match label {
Collection(labels) => labels,
label => panic!("fflonk commitment tagged with {label}, not a collection"),
})
.collect()
}
fn commitment\_byte\_length(n: usize) -> usize {
PCS::commitment\_byte\_length(n.div\_ceil(Self::T\_MAX))
}
fn multi\_prepare<'com, T: Transcript>(
queries: &[VerifierQuery<'com, F, Self>],
transcript: &mut T,
) -> Result
where
F: Sampleable + Hash + Ord + Hashable,
Self::Commitment: Hashable + 'com,
{
// Maps the labels of every queried chunk to its commitment and the points
// its polynomials are queried at, in query order: `labels -> (commitment,
// points)`.
let chunks\_info = queries.iter().fold(BTreeMap::new(), |mut chunks\_info, q| {
let labels = (Self::commitment\_labels(q.commitment).chunks(Self::T\_MAX))
.find(|labels| labels.contains(&q.label))
.map\_or\_else(|| vec![q.label.clone()], <[\_]>::to\_vec);
let (\_, points) = chunks\_info.entry(labels).or\_insert((q.commitment, Vec::new()));
if !points.contains(&q.point) {
points.push(q.point);
}
chunks\_info
});
// The evaluations of the chunks at all the points, both the explicit (taken
// from the queries) and the implicit (read from the transcript here) ones.
let mut evals: HashMap<(PolynomialLabel, F), F> =
queries.iter().map(|q| ((q.label.clone(), q.point), q.eval)).collect();
for (labels, (\_, points)) in &chunks\_info {
for point in points {
for label in labels {
if let Entry::Vacant(entry) = evals.entry((label.clone(), \*point)) {
entry.insert(transcript.read().map\_err(|\_| Error::SamplingError)?);
}
}
}
}
// Opening a chunk at `x` amounts to opening its `g` polynomial at
// the `t`-th roots of `x`, where computing `g(r) = Σ\_i r^i f\_i(x)` for
// such a root `r` is equivalent to evaluating at `r` the polynomial with
// coefficients `f\_i(x)`.
let mut inner\_queries = Vec::new();
let mut roots\_cache = HashMap::new();
for (labels, (commitment, points)) in &chunks\_info {
let t = labels.len().next\_power\_of\_two();
for x in points {
let f\_evals: Vec = labels.iter().map(|l| evals[&(l.clone(), \*x)]).collect();
for root in cached\_roots(&mut roots\_cache, \*x, t).ok\_or(Error::OpeningError)? {
let eval = eval\_polynomial\_seq(&f\_evals, \*root);
let label = Collection(labels.clone());
inner\_queries.push(VerifierQuery::new(\*root, \*commitment, label, eval));
}
}
}
PCS::multi\_prepare(&inner\_queries, transcript)
}
}
#[cfg(test)]
mod tests {
use std::io;
use blake2b\_simd::State as Blake2bState;
use ff::Field;
use midnight\_curves::{Bls12, Fq};
use rand\_core::{OsRng, RngCore};
use super::Fflonk;
use crate::{
poly::{
EvaluationDomain, PolynomialLabel, ProverQuery, VerifierQuery,
commitment::{Guard, PolynomialCommitmentScheme},
kzg::{KZGCommitmentScheme, params::ParamsKZG},
},
transcript::{CircuitTranscript, Transcript},
utils::{arithmetic::eval\_polynomial, helpers::SerdeFormat},
};
type Kzg = KZGCommitmentScheme;
/// A chunk `[a0, a1]` queried as `a0(p)` and `a1(q)` has the implicit
/// evaluations `a1(p)` and `a0(q)`. They are read in query order, `p`
/// before `q`, whichever of `p` and `q` is the larger field element.
#[test]
fn test\_implicit\_evals\_in\_query\_order() {
type CS = Fflonk;
type T = CircuitTranscript;
let k = 3;
let params = CS::gen\_params(k);
let domain = EvaluationDomain::::new(1, k);
let labels = [PolynomialLabel::Advice(0), PolynomialLabel::Advice(1)];
let polys: Vec<\_> = (0..2)
.map(|\_| domain.coeff\_from\_vec((0..1 << k).map(|\_| Fq::random(OsRng)).collect()))
.collect();
let commitment = CS::commit\_many(¶ms, &polys.iter().collect::>(), &labels);
let (mut seen\_lt, mut seen\_gt) = (false, false);
while !(seen\_lt && seen\_gt) {
// Opening points of a `T\_MAX = 2` chunk must be squares.
let [p, q] = [(); 2].map(|\_| Fq::random(OsRng).square());
seen\_lt |= p < q;
seen\_gt |= p > q;
let explicit = [eval\_polynomial(&polys[0], p), eval\_polynomial(&polys[1], q)];
let mut transcript = T::init();
explicit.iter().for\_each(|e| transcript.write(e).unwrap());
let queries = [
ProverQuery::new(&labels, &polys, p, labels[0].clone()),
ProverQuery::new(&labels, &polys, q, labels[1].clone()),
];
CS::multi\_open(¶ms, &queries, &mut transcript).unwrap();
let proof = transcript.finalize();
let mut transcript = T::init\_from\_bytes(&proof);
let read: [Fq; 4] = [(); 4].map(|\_| transcript.read().unwrap());
let implicit = [eval\_polynomial(&polys[1], p), eval\_polynomial(&polys[0], q)];
assert\_eq!(read, [explicit, implicit].concat()[..]);
let mut transcript = T::init\_from\_bytes(&proof);
let evals: [Fq; 2] = [(); 2].map(|\_| transcript.read().unwrap());
let queries = [
VerifierQuery::new(p, &commitment, labels[0].clone(), evals[0]),
VerifierQuery::new(q, &commitment, labels[1].clone(), evals[1]),
];
let guard = CS::multi\_prepare(&queries, &mut transcript).unwrap();
transcript.assert\_empty().unwrap();
guard.verify(¶ms.verifier\_params()).unwrap();
}
}
/// Commits to `nb\_polys` sparse polynomials of `2^k` rows, given in
/// Lagrange form, in its delta forms and in coefficient form, and checks
/// that the commitments agree.
fn assert\_lagrange\_matches\_coeff(params: &ParamsKZG, k: u32, nb\_polys: usize) {
type CS = Fflonk;
let domain = EvaluationDomain::::new(1, k);
let lagrange: Vec<\_> = (0..nb\_polys)
.map(|\_| {
let values = (0..1 << k)
.map(|\_| {
if OsRng.next\_u32().is\_multiple\_of(8) {
Fq::random(OsRng)
} else {
Fq::ZERO
}
})
.collect();
domain.lagrange\_from\_vec(values)
})
.collect();
let coeff: Vec<\_> = lagrange.iter().map(|p| domain.lagrange\_to\_coeff(p.clone())).collect();
let delta: Vec<\_> = lagrange.iter().map(|p| p.to\_delta()).collect();
let double\_delta: Vec<\_> = lagrange.iter().map(|p| p.to\_double\_delta()).collect();
let labels: Vec<\_> = (0..nb\_polys).map(PolynomialLabel::Advice).collect();
let expected = CS::commit\_many(params, &coeff.iter().collect::>(), &labels);
assert\_eq!(
CS::commit\_many(params, &lagrange.iter().collect::>(), &labels),
expected
);
assert\_eq!(
CS::commit\_many(params, &delta.iter().collect::>(), &labels),
expected
);
assert\_eq!(
CS::commit\_many(params, &double\_delta.iter().collect::>(), &labels),
expected
);
}
#[test]
fn test\_lagrange\_commitments() {
// Full and partial chunks, of every chunk size up to `T\_MAX = 4`.
let params =  as PolynomialCommitmentScheme>::gen\_params(4);
for nb\_polys in 1..=9 {
assert\_lagrange\_matches\_coeff(¶ms, 4, nb\_polys);
}
}
#[test]
fn test\_load\_params() {
let mut srs = Vec::new();
let format = SerdeFormat::RawBytesUnchecked;
ParamsKZG::::unsafe\_setup(6, OsRng)
.write\_custom(&mut srs, format)
.unwrap();
let load\_kzg =
|k| >::load\_params(&mut &srs[..], format, k);
let load\_fflonk = |k| {
 as PolynomialCommitmentScheme>::load\_params(
&mut &srs[..],
format,
k,
)
};
let params = load\_kzg(5).unwrap();
assert\_eq!(>::max\_k(¶ms), 5);
let params = load\_fflonk(4).unwrap();
assert\_eq!(params.max\_k(), 6);
assert\_eq!(
 as PolynomialCommitmentScheme>::max\_k(¶ms),
4
);
assert\_eq!(load\_kzg(7).unwrap\_err().kind(), io::ErrorKind::InvalidData);
assert\_eq!(
load\_fflonk(5).unwrap\_err().kind(),
io::ErrorKind::InvalidData
);
}
}