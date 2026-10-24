//! We implement the multi-open technique developed in Halo 2. It is designed to
//! efficiently open multiple polynomials at multiple points while minimizing
//! proof size and verification time. In a nutshell, multiple opening queries
//! are batched into a single query by combining the target
//! polynomials/commitments and evaluation points using verifier-chosen
//! random scalars.
//!
//! For a more detailed explanation, see the [Halo 2 Book](https://zcash.github.io/halo2/design/proving-system/multipoint-opening.html) on Multipoint Openings.
use std::{
collections::HashMap,
io::{self, Read},
marker::PhantomData,
};
use midnight\_curves::pairing::Engine;
use rayon::iter::{
IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
/// KZG commitment type
pub mod commitment;
/// Multiscalar multiplication engines
pub mod msm;
/// KZG commitment scheme
pub mod params;
mod utils;
use std::{fmt::Debug, hash::Hash};
use commitment::{KZGCommitment, KZGMultiCommitment};
use ff::Field;
use group::Group;
use midnight\_curves::{pairing::MultiMillerLoop, serde::SerdeObject};
use rand\_core::OsRng;
use crate::{
poly::{
Coeff, Error, Polynomial, PolynomialRepresentation, ProverQuery,
commitment::{InterleavedCommit, PolynomialCommitmentScheme},
kzg::{
commitment::NB\_POLYS\_PREFIX\_BYTES,
msm::{DualMSM, msm\_specific},
params::{ParamsKZG, ParamsVerifierKZG},
utils::construct\_intermediate\_sets,
},
query::{PolynomialLabel, VerifierQuery},
},
transcript::{Hashable, Sampleable, Transcript},
utils::{
arithmetic::{
CurveAffine, CurveExt, divide\_by\_roots, eval\_interpolant, eval\_polynomial,
evals\_inner\_product, inner\_product, kate\_division, parallelize, powers, truncate,
truncated\_powers,
},
helpers::{ProcessedSerdeObject, SerdeFormat},
},
};
/// The multi-open collapses the combined commitment of a point set (but the
/// first) into one point before scaling it by its `x4` power only if it has at
/// least this many variable-base terms. A set with fewer keeps its terms,
/// scaled by `x4` one by one.
pub const COLLAPSE\_THRESHOLD: usize = 4;
#[derive(Clone, Debug)]
/// KZG verifier
pub struct KZGCommitmentScheme {
\_marker: PhantomData,
}
impl PolynomialCommitmentScheme for KZGCommitmentScheme
where
E::G1: Default + CurveExt + ProcessedSerdeObject,
E::G1Affine: Default + CurveAffine + SerdeObject,
E::G2: ProcessedSerdeObject,
{
type Parameters = ParamsKZG;
type VerifierParameters = ParamsVerifierKZG;
type Commitment = KZGMultiCommitment;
type VerificationGuard = DualMSM;
fn gen\_params(k: u32) -> Self::Parameters {
ParamsKZG::unsafe\_setup(k, OsRng)
}
fn load\_params(
reader: &mut R,
format: SerdeFormat,
k: u32,
) -> io::Result {
let mut params = ParamsKZG::read\_custom(reader, format)?;
if params.max\_k() < k {
return Err(io::Error::new(
io::ErrorKind::InvalidData,
format!("SRS of size 2^{} is smaller than 2^{k}", params.max\_k()),
));
}
params.downsize(k);
Ok(params)
}
fn max\_k(params: &Self::Parameters) -> u32 {
params.max\_k()
}
fn get\_verifier\_params(params: &Self::Parameters) -> Self::VerifierParameters {
params.verifier\_params()
}
fn commit\_many(
params: &Self::Parameters,
polynomials: &[&Polynomial],
labels: &[PolynomialLabel],
) -> Self::Commitment {
assert\_eq!(
polynomials.len(),
labels.len(),
"polynomials and labels must have the same length"
);
PolynomialLabel::assert\_distinct(labels);
let bases = params.bases::**();
KZGMultiCommitment(
(polynomials.iter().zip(labels))
.map(|(polynomial, label)| {
let size = polynomial.values.len();
assert!(bases.len() >= size);
KZGCommitment::Simple(
msm\_specific::(&polynomial.values, &bases[..size]),
label.clone(),
)
})
.collect(),
)
}
fn read\_commitment(
transcript: &mut T,
labels: &[PolynomialLabel],
) -> io::Result
where
Self::Commitment: Hashable,
{
// An empty group is never written to the transcript.
if labels.is\_empty() {
return Ok(KZGMultiCommitment(vec![]));
}
PolynomialLabel::assert\_distinct(labels);
let commitment: KZGMultiCommitment = transcript.read()?;
// How many polynomials the group holds is fixed by the verifying key,
// so any other number is a malformed proof. The prover declares the
// count in the proof itself, and that count is not part of the hashed
// transcript: without this check the `zip` below would silently
// truncate to the shorter of the two, letting a prover collapse a whole
// group onto fewer points than it has labels.
if commitment.0.len() != labels.len() {
return Err(io::Error::new(
io::ErrorKind::InvalidData,
format!(
"commitment to {} polynomials, expected {}",
commitment.0.len(),
labels.len()
),
));
}
let commitment = KZGMultiCommitment(
commitment
.0
.iter()
.zip(labels.iter())
.map(|(com, label)| match com {
KZGCommitment::Simple(point, PolynomialLabel::NoLabel) => {
KZGCommitment::::Simple(\*point, label.clone())
}
\_ => unreachable!(),
})
.collect(),
);
Ok(commitment)
}
fn write\_commitment(
transcript: &mut T,
commitment: &Self::Commitment,
) -> io::Result<()>
where
Self::Commitment: Hashable,
{
// An empty group is neither written nor absorbed, so the transcript
// carries no prefix for it and `read\_commitment` reads nothing back.
if commitment.0.is\_empty() {
return Ok(());
}
transcript.write(commitment)
}
fn commitment\_labels(commitment: &Self::Commitment) -> Vec {
(commitment.0.iter())
.flat\_map(|c| match c {
KZGCommitment::Simple(\_, label) => vec![label.clone()],
KZGCommitment::Linear(\_, \_, labels) => labels.clone(),
})
.collect()
}
fn commitment\_byte\_length(n: usize) -> usize {
// A non-empty group of `n` polynomials travels through the transcript
// as one length-prefixed message: the prefix, then one point per
// polynomial. (See `Hashable::to\_bytes` for `KZGMultiCommitment`.) An
// empty group is not written at all.
if n == 0 {
return 0;
}
NB\_POLYS\_PREFIX\_BYTES + n \* Self::Commitment::default().byte\_length(SerdeFormat::Processed)
}
fn deserialize\_commitment(
reader: &mut R,
format: SerdeFormat,
labels: &[PolynomialLabel],
) -> io::Result {
PolynomialLabel::assert\_distinct(labels);
let inners = labels
.iter()
.map(|label| {
let point = E::G1::read(reader, format)?;
Ok(KZGCommitment::Simple(point, label.clone()))
})
.collect::>>()?;
Ok(KZGMultiCommitment(inners))
}
fn multi\_open(
params: &Self::Parameters,
queries: &[ProverQuery],
transcript: &mut T,
) -> Result<(), Error>
where
E::Fr: Sampleable + Hash + Ord + Hashable,
KZGMultiCommitment: Hashable,
{
/// Like [`inner\_product`] but for coefficient-form polynomials that may
/// have different lengths (zero-extending the shorter operands).
///
/// Fused parallel implementation: a single pass accumulates all
/// scaled contributions directly into the output buffer, avoiding
/// M intermediate allocations and the sequential reduce chain.
fn poly\_inner\_product(
polys: &[&Polynomial],
scalars: impl IntoIterator,
) -> Polynomial {
let scalars: Vec = scalars.into\_iter().take(polys.len()).collect();
let max\_len = polys.iter().map(|p| p.len()).max().unwrap\_or(0);
let mut values = vec![F::ZERO; max\_len];
parallelize(&mut values, |chunk, start| {
for (poly, scalar) in polys.iter().zip(scalars.iter()) {
let pv: &[F] = poly;
let end = (start + chunk.len()).min(pv.len());
if start < pv.len() {
for (out, coeff) in chunk[..end - start].iter\_mut().zip(&pv[start..end]) {
\*out += \*coeff \* scalar;
}
}
}
});
Polynomial {
values,
\_marker: PhantomData,
}
}
// Refer to the halo2 book for docs:
// https://zcash.github.io/halo2/design/proving-system/multipoint-opening.html
let x1: E::Fr = transcript.squeeze\_challenge();
let x2: E::Fr = transcript.squeeze\_challenge();
// Map each label to the polynomial it identifies, so the per-set
// grouping (keyed by label) can recover the actual polynomials.
let label\_to\_poly: HashMap> =
queries.iter().map(|q| (q.label.clone(), q.poly())).collect();
// `construct\_intermediate\_sets` is shared with the verifier, and the
// evaluations (the third component of each query) are only read by the
// verifier, so the prover passes zeros.
// TODO: address this better.
let kzg\_queries = (queries.iter())
.map(|query| (query.label.clone(), query.point, E::Fr::ZERO))
.collect::>();
let (poly\_map, point\_sets) = construct\_intermediate\_sets(&kzg\_queries)?;
let mut q\_polys = vec![vec![]; point\_sets.len()];
for com\_data in poly\_map.iter() {
q\_polys[com\_data.set\_index].push(label\_to\_poly[&com\_data.label]);
}
let q\_polys: Vec<\_> = q\_polys
.par\_iter()
.map(|polys| {
let x1 = truncated\_powers(x1);
poly\_inner\_product(polys, x1)
})
.collect();
// Sort point sets by ascending cardinality, as the verifier does. The (len, i)
// key provides a deterministic total order even when two sets share the same
// cardinality.
let (q\_polys, point\_sets) = {
let mut order: Vec = (0..point\_sets.len()).collect();
order.sort\_by\_key(|&i| (point\_sets[i].len(), i));
let q\_polys: Vec<\_> = order.iter().map(|&i| &q\_polys[i]).collect();
let point\_sets: Vec<\_> = order.iter().map(|&i| point\_sets[i].clone()).collect();
(q\_polys, point\_sets)
};
let f\_poly = {
let f\_polys: Vec<\_> = point\_sets
.into\_par\_iter()
.zip(q\_polys.clone().into\_par\_iter())
.map(|(points, q\_poly)| Polynomial {
values: divide\_by\_roots(&q\_poly.values, &points),
\_marker: PhantomData,
})
.collect();
poly\_inner\_product(&f\_polys.iter().collect::>(), powers(x2))
};
let f\_com = Self::commit(
params,
&f\_poly,
PolynomialLabel::Custom("multi\_open\_batch".into()),
);
transcript.write(&f\_com).map\_err(|\_| Error::OpeningError)?;
let x3: E::Fr = transcript.squeeze\_challenge();
let x3 = truncate(x3);
// Evaluate all q\_polys at x3 in parallel, then write sequentially.
let q\_evals: Vec =
q\_polys.par\_iter().map(|q\_poly| eval\_polynomial(&q\_poly.values, x3)).collect();
for eval in &q\_evals {
transcript.write(eval).map\_err(|\_| Error::OpeningError)?;
}
let x4: E::Fr = transcript.squeeze\_challenge();
let final\_poly = {
let mut polys = q\_polys;
polys.push(&f\_poly);
let powers = truncated\_powers(x4);
poly\_inner\_product(&polys, powers)
};
let v = eval\_polynomial(&final\_poly, x3);
let pi = {
let pi\_poly = Polynomial::<\_, Coeff> {
values: kate\_division(&(&final\_poly - v).values, x3),
\_marker: PhantomData,
};
Self::commit(
params,
&pi\_poly,
PolynomialLabel::Custom("kzg\_proof".into()),
)
};
transcript.write(&pi).map\_err(|\_| Error::OpeningError)
}
fn multi\_prepare<'com, T: Transcript>(
queries: &[VerifierQuery<'com, E::Fr, KZGCommitmentScheme>],
transcript: &mut T,
) -> Result, Error>
where
E::Fr: Sampleable + Ord + Hash + Hashable,
E::G1: CurveExt,
KZGMultiCommitment: Hashable + 'com,
{
// Refer to the halo2 book for docs:
// https://zcash.github.io/halo2/design/proving-system/multipoint-opening.html
let x1: E::Fr = transcript.squeeze\_challenge();
let x2: E::Fr = transcript.squeeze\_challenge();
// Peel each query's multi-commitment down to the single inner
// `KZGCommitment` it targets, keyed by the query label. The rest of the
// routine then operates on individual `KZGCommitment`s as before.
//
// The `Linear` quotient commitment aggregates many polynomials and
// carries no single label of its own, so it is taken as it is. Every
// `Simple` has to name the queried polynomial, whether it stands alone
// or in a batch: matching on the label rather than on the position keeps
// a malformed group from aliasing every query onto the same point.
// A query naming a polynomial its group lacks (e.g. a mislabelled
// committed instance) is malformed input: reject it, never panic.
let label\_to\_commitment: HashMap> = queries
.iter()
.map(|q| {
let inners = &q.commitment.0;
let inner = match inners.as\_slice() {
[single @ KZGCommitment::Linear(..)] => single,
\_ => inners
.iter()
.find(|c| matches!(c, KZGCommitment::Simple(\_, label) if \*label == q.label))
.ok\_or(Error::OpeningError)?,
};
Ok((q.label.clone(), inner))
})
.collect::>()?;
let kzg\_queries = queries
.iter()
.map(|query| (query.label.clone(), query.point, query.eval))
.collect::>();
let (commitment\_map, point\_sets) = construct\_intermediate\_sets(&kzg\_queries)?;
let mut q\_coms: Vec<\_> = vec![vec![]; point\_sets.len()];
let mut q\_eval\_sets = vec![vec![]; point\_sets.len()];
for com\_data in commitment\_map.into\_iter() {
q\_coms[com\_data.set\_index].push(label\_to\_commitment[&com\_data.label].clone());
q\_eval\_sets[com\_data.set\_index].push(com\_data.evals);
}
let nb\_x1\_powers = q\_coms.iter().map(Vec::len).max().unwrap\_or(0);
assert!(nb\_x1\_powers >= q\_eval\_sets.iter().map(Vec::len).max().unwrap\_or(0));
let powers\_x1 = truncated\_powers(x1).take(nb\_x1\_powers).collect::>();
let q\_coms = q\_coms
.into\_iter()
.map(|coms| inner\_product(&coms, powers\_x1.clone().into\_iter()))
.collect::>();
let q\_eval\_sets = q\_eval\_sets
.iter()
.map(|evals| evals\_inner\_product(evals, &powers\_x1))
.collect::>();
// Sort point sets by ascending cardinality. The (len, i) key provides a
// deterministic total order even when two sets share the same cardinality.
let (q\_coms, q\_eval\_sets, point\_sets) = {
let mut order: Vec = (0..point\_sets.len()).collect();
order.sort\_by\_key(|&i| (point\_sets[i].len(), i));
let q\_coms: Vec<\_> = order.iter().map(|&i| q\_coms[i].clone()).collect();
let q\_eval\_sets: Vec<\_> = order.iter().map(|&i| q\_eval\_sets[i].clone()).collect();
let point\_sets: Vec<\_> = order.iter().map(|&i| point\_sets[i].clone()).collect();
(q\_coms, q\_eval\_sets, point\_sets)
};
let f\_com = Self::read\_commitment(
transcript,
&[PolynomialLabel::Custom("kzg\_batch".to\_string())],
)
.map\_err(|\_| Error::SamplingError)?
.into\_single();
// Sample a challenge x\_3 for checking that f(X) was committed to
// correctly.
let x3: E::Fr = transcript.squeeze\_challenge();
let x3 = truncate(x3);
let mut q\_evals\_on\_x3 = Vec::::with\_capacity(q\_eval\_sets.len());
for \_ in 0..q\_eval\_sets.len() {
q\_evals\_on\_x3.push(transcript.read().map\_err(|\_| Error::SamplingError)?);
}
// We can compute the expected msm\_eval at x\_3 using the u provided
// by the prover and from x\_2
let f\_eval =
point\_sets.iter().zip(q\_eval\_sets.iter()).zip(q\_evals\_on\_x3.iter()).rev().fold(
E::Fr::ZERO,
|acc\_eval, ((points, evals), proof\_eval)| {
// eval = (proof\_eval - r\_eval) / prod\_i (x3 - point\_i)
let (sum, vanishing\_inv) = eval\_interpolant(points, evals, x3);
let eval = \*proof\_eval \* vanishing\_inv - sum;
acc\_eval \* &(x2) + &eval
},
);
let x4: E::Fr = transcript.squeeze\_challenge();
let final\_com = {
let size = q\_coms.len() + 1;
let mut coms = q\_coms;
// Collapse the MSMs of at least `COLLAPSE\_THRESHOLD` variable-base
// terms before combining with x4 powers, to match the in-circuit
// verifier. Skip the first one since its x4 power is 1.
(coms.iter\_mut().skip(1))
.filter(|c| {
matches!(c, KZGCommitment::Linear(\_, \_, labels)
if labels.iter().filter(|l| !l.is\_fixed()).count() >= COLLAPSE\_THRESHOLD)
})
.for\_each(|c| c.collapse(PolynomialLabel::NoLabel));
coms.push(f\_com);
let powers = truncated\_powers(x4);
inner\_product(&coms, powers.take(size))
};
let v = {
let mut evals = q\_evals\_on\_x3;
evals.push(f\_eval);
let powers = truncated\_powers(x4);
inner\_product(&evals, powers)
};
let pi = Self::read\_commitment(transcript, &[PolynomialLabel::Custom("π".to\_string())])
.map\_err(|\_| Error::SamplingError)?
.into\_single();
// (π, C − vG + zπ), assembled with the scheme's own linear operations:
// the terms stay a lazy `Linear` combination until the pairing check
// collapses them into one MSM.
let minus\_g =
KZGCommitment::Simple(-E::G1::generator(), PolynomialLabel::Custom("-G".into()));
let rhs = final\_com + minus\_g \* v + pi.clone() \* x3;
Ok(DualMSM::new(pi.into(), rhs.into()))
}
}
impl InterleavedCommit for KZGCommitmentScheme
where
E::G1: Default + CurveExt + ProcessedSerdeObject,
E::G1Affine: Default + CurveAffine + SerdeObject,
E::G2: ProcessedSerdeObject,
{
fn commit\_interleaved(
params: &Self::Parameters,
chunks: &[&[&Polynomial]],
labels: &[PolynomialLabel],
) -> Self::Commitment {
assert\_eq!(
chunks.len(),
labels.len(),
"chunks and labels must have the same length"
);
PolynomialLabel::assert\_distinct(labels);
KZGMultiCommitment(
(chunks.iter().zip(labels))
.map(|(chunk, label)| {
KZGCommitment::Simple(commit\_chunk(params, chunk), label.clone())
})
.collect(),
)
}
}
/// Commitment to `Σ\_i X^i f\_i(X^t)`, for `chunk` the polynomials `f\_i` and
/// `t` the next power of two of their number.
fn commit\_chunk(
params: &ParamsKZG,
chunk: &[&Polynomial],
) -> E::G1
where
E::G1Affine: CurveAffine,
{
let t = chunk.len().next\_power\_of\_two();
let n = chunk[0].len();
assert!(
chunk.iter().all(|poly| poly.len() == n),
"the polynomials of a chunk have different lengths"
);
(chunk.iter().enumerate())
.map(|(i, poly)| msm\_specific(&poly.values, ¶ms.residue\_bases::**(t, n, i)))
.sum()
}
#[cfg(test)]
mod tests {
use std::{hash::Hash, slice};
use blake2b\_simd::State as Blake2bState;
use ff::WithSmallOrderMulGroup;
use midnight\_curves::{CurveAffine, CurveExt, pairing::MultiMillerLoop, serde::SerdeObject};
use rand\_core::OsRng;
use crate::{
poly::{
Coeff, EvaluationDomain, PolynomialLabel,
commitment::{Guard, PolynomialCommitmentScheme},
kzg::{
KZGCommitmentScheme,
commitment::KZGMultiCommitment,
params::{ParamsKZG, ParamsVerifierKZG},
},
query::{ProverQuery, VerifierQuery},
},
transcript::{CircuitTranscript, Hashable, Sampleable, Transcript},
utils::{arithmetic::eval\_polynomial, helpers::ProcessedSerdeObject},
};
#[test]
fn test\_roundtrip\_gwc() {
use midnight\_curves::Bls12;
const K: u32 = 4;
let params: ParamsKZG = ParamsKZG::unsafe\_setup(K, OsRng);
let proof = create\_proof::<\_, CircuitTranscript>(¶ms);
let verifier\_params = params.verifier\_params();
verify::>(&verifier\_params, &proof[..], false);
verify::>(&verifier\_params, &proof[..], true);
}
#[test]
fn test\_empty\_commitment\_group() {
use midnight\_curves::{Bls12, Fq};
type CS = KZGCommitmentScheme;
type T = CircuitTranscript;
let params: ParamsKZG = ParamsKZG::unsafe\_setup(4, OsRng);
let commitment = CS::commit\_many::(¶ms, &[], &[]);
assert!(commitment.0.is\_empty());
let fresh\_challenge: Fq = T::init().squeeze\_challenge();
// Writing an empty group neither writes nor absorbs anything.
let mut transcript = T::init();
CS::write\_commitment(&mut transcript, &commitment).unwrap();
let prover\_challenge: Fq = transcript.squeeze\_challenge();
let proof = transcript.finalize();
assert!(proof.is\_empty());
assert\_eq!(prover\_challenge, fresh\_challenge);
// Reading an empty group neither reads nor absorbs anything.
let mut transcript = T::init\_from\_bytes(&proof);
let read = CS::read\_commitment(&mut transcript, &[]).unwrap();
assert!(read.0.is\_empty());
let verifier\_challenge: Fq = transcript.squeeze\_challenge();
assert\_eq!(verifier\_challenge, fresh\_challenge);
transcript.assert\_empty().unwrap();
}
#[test]
fn test\_query\_on\_empty\_commitment\_group() {
use ff::Field;
use midnight\_curves::{Bls12, Fq};
let empty = KZGMultiCommitment::(vec![]);
let query = VerifierQuery::new(
Fq::ONE,
&empty,
PolynomialLabel::Custom("a".into()),
Fq::ONE,
);
// A query naming a polynomial its commitment group lacks is malformed
// input: the verifier rejects it rather than panicking.
let mut transcript = CircuitTranscript::::init();
assert!(matches!(
KZGCommitmentScheme::::multi\_prepare(&[query], &mut transcript),
Err(crate::poly::Error::OpeningError)
));
}
fn verify(verifier\_params: &ParamsVerifierKZG, proof: &[u8], should\_fail: bool)
where
E: MultiMillerLoop,
T: Transcript,
E::Fr: Hashable + Sampleable + Ord + Hash,
E::G1: Hashable + CurveExt,
E::G1Affine: CurveAffine + SerdeObject,
E::G2: ProcessedSerdeObject,
KZGMultiCommitment: Hashable,
{
let mut transcript = T::init\_from\_bytes(proof);
let a = KZGCommitmentScheme::::read\_commitment(
&mut transcript,
&[PolynomialLabel::Custom("a".into())],
)
.unwrap();
let b = KZGCommitmentScheme::::read\_commitment(
&mut transcript,
&[PolynomialLabel::Custom("b".into())],
)
.unwrap();
let c = KZGCommitmentScheme::::read\_commitment(
&mut transcript,
&[PolynomialLabel::Custom("c".into())],
)
.unwrap();
let x: E::Fr = transcript.squeeze\_challenge();
let y: E::Fr = transcript.squeeze\_challenge();
let avx: E::Fr = transcript.read().unwrap();
let bvx: E::Fr = transcript.read().unwrap();
let cvy: E::Fr = transcript.read().unwrap();
let valid\_queries = std::iter::empty()
.chain(Some(VerifierQuery::new(
x,
&a,
PolynomialLabel::Custom("a".into()),
avx,
)))
.chain(Some(VerifierQuery::new(
x,
&b,
PolynomialLabel::Custom("b".into()),
bvx,
)))
.chain(Some(VerifierQuery::new(
y,
&c,
PolynomialLabel::Custom("c".into()),
cvy,
)));
let invalid\_queries = std::iter::empty()
.chain(Some(VerifierQuery::new(
x,
&a,
PolynomialLabel::Custom("a".into()),
avx,
)))
.chain(Some(VerifierQuery::new(
x,
&b,
PolynomialLabel::Custom("b".into()),
avx,
)))
.chain(Some(VerifierQuery::new(
y,
&c,
PolynomialLabel::Custom("c".into()),
cvy,
)));
let queries = if should\_fail {
invalid\_queries
} else {
valid\_queries
};
let result =
KZGCommitmentScheme::multi\_prepare(&queries.collect::>(), &mut transcript)
.unwrap();
if should\_fail {
assert!(result.verify(verifier\_params).is\_err());
} else {
assert!(result.verify(verifier\_params).is\_ok());
}
}
fn create\_proof(kzg\_params: &ParamsKZG) -> Vec
where
E: MultiMillerLoop,
T: Transcript,
E::Fr: WithSmallOrderMulGroup<3> + Hashable + Hash + Sampleable + Ord,
E::G1: Hashable + CurveExt,
E::G1Affine: SerdeObject + CurveAffine,
E::G2: ProcessedSerdeObject,
{
let k = (kzg\_params.g.len() - 1).ilog2() + 1;
let domain = EvaluationDomain::new(1, k);
let mut ax = domain.empty\_coeff();
for (i, a) in ax.iter\_mut().enumerate() {
\*a = ::from(10 + i as u64);
}
let mut bx = domain.empty\_coeff();
for (i, a) in bx.iter\_mut().enumerate() {
\*a = ::from(100 + i as u64);
}
let mut cx = domain.empty\_coeff();
for (i, a) in cx.iter\_mut().enumerate() {
\*a = ::from(100 + i as u64);
}
let mut transcript = T::init();
let a = KZGCommitmentScheme::commit(kzg\_params, &ax, PolynomialLabel::Custom("a".into()));
let b = KZGCommitmentScheme::commit(kzg\_params, &bx, PolynomialLabel::Custom("b".into()));
let c = KZGCommitmentScheme::commit(kzg\_params, &cx, PolynomialLabel::Custom("c".into()));
transcript.write(&a).unwrap();
transcript.write(&b).unwrap();
transcript.write(&c).unwrap();
let x: E::Fr = transcript.squeeze\_challenge();
let y = transcript.squeeze\_challenge();
let avx = eval\_polynomial(&ax, x);
let bvx = eval\_polynomial(&bx, x);
let cvy = eval\_polynomial(&cx, y);
transcript.write(&avx).unwrap();
transcript.write(&bvx).unwrap();
transcript.write(&cvy).unwrap();
let labels = |name: &str| [PolynomialLabel::Custom(name.into())];
let (la, lb, lc) = (labels("a"), labels("b"), labels("c"));
let queries = [
ProverQuery::new(&la, slice::from\_ref(&ax), x, la[0].clone()),
ProverQuery::new(&lb, slice::from\_ref(&bx), x, lb[0].clone()),
ProverQuery::new(&lc, slice::from\_ref(&cx), y, lc[0].clone()),
]
.into\_iter();
KZGCommitmentScheme::multi\_open(kzg\_params, &queries.collect::>(), &mut transcript)
.unwrap();
transcript.finalize()
}
}****