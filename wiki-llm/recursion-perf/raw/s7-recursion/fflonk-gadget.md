# Midnight in-circuit fflonk at PR 575 merge
> Source: https://raw.githubusercontent.com/midnightntwrk/midnight-zk/1e39c12/circuits/src/verifier/fflonk.rs
> Collected: 2026-10-26
> Published: Unknown

// This file is part of MIDNIGHT-ZK.
// Copyright (C) Midnight Foundation
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
//! In-circuit fflonk over a generic in-circuit PCS.
//!
//! In-circuit analog of [`midnight\_proofs::poly::fflonk`]: the polynomials of
//! a group are committed in chunks of at most `T\_MAX`, each chunk under the
//! label `Collection(chunk)`, and opening a chunk at `x` amounts to opening it
//! with the inner PCS at the `t`-th roots of `x`.
use std::{
collections::{BTreeMap, HashMap, hash\_map::Entry},
marker::PhantomData,
};
use midnight\_proofs::{
circuit::{Layouter, Value},
plonk::Error,
poly::{
PolynomialLabel::{self, Collection},
commitment::InterleavedCommit,
fflonk::Fflonk,
},
};
use crate::{
field::AssignedNative,
instructions::ArithInstructions,
verifier::{
AssignedAccumulator, SelfEmulation,
pcs::{InCircuitPCS, VerifierQuery},
transcript\_gadget::TranscriptGadget,
utils::{RootsCache, cached\_roots, eval\_polynomial},
},
};
/// fflonk over the inner in-circuit `PCS`, combining at most `T\_MAX =
/// 2^LOG2\_T\_MAX` polynomials per chunk. Verifies the proofs of
/// [`Fflonk`](Fflonk).
#[derive(Clone, Copy, Debug)]
pub struct InCircuitFflonk(PhantomData);
impl InCircuitFflonk {
/// The maximum number of polynomials combined into one.
const T\_MAX: usize = 1 << LOG2\_T\_MAX;
/// The labels of the inner commitments to the group `labels`.
fn chunk\_labels(labels: &[PolynomialLabel]) -> Vec {
PolynomialLabel::assert\_distinct(labels);
labels.chunks(Self::T\_MAX).map(|l| Collection(l.to\_vec())).collect()
}
}
impl InCircuitPCS ~~for InCircuitFflonk
where
S: SelfEmulation,
PCS: InCircuitPCS>,
{
type OffCircuitPCS = Fflonk;
type AssignedCommitment = PCS::AssignedCommitment;
fn fixed\_commitment(labels: &[PolynomialLabel]) -> Self::AssignedCommitment {
PCS::fixed\_commitment(&Self::chunk\_labels(labels))
}
fn fixed\_base\_labels(labels: &[PolynomialLabel]) -> Vec {
PCS::fixed\_base\_labels(&Self::chunk\_labels(labels))
}
fn assign\_commitment(
layouter: &mut impl Layouter,
curve\_chip: &S::CurveChip,
value: Value,
label: PolynomialLabel,
) -> Result {
PCS::assign\_commitment(layouter, curve\_chip, value, Collection(vec![label]))
}
fn read\_commitment(
transcript: &mut TranscriptGadget~~,
layouter: &mut impl Layouter,
labels: &[PolynomialLabel],
) -> Result {
PCS::read\_commitment(transcript, layouter, &Self::chunk\_labels(labels))
}
fn common\_commitment(
transcript: &mut TranscriptGadget~~,
layouter: &mut impl Layouter,
commitment: &Self::AssignedCommitment,
) -> Result<(), Error> {
PCS::common\_commitment(transcript, layouter, commitment)
}
fn squeeze\_evaluation\_point(
layouter: &mut impl Layouter,
scalar\_chip: &S::ScalarChip,
transcript: &mut TranscriptGadget~~,
) -> Result, Error> {
let x = PCS::squeeze\_evaluation\_point(layouter, scalar\_chip, transcript)?;
scalar\_chip.pow(layouter, &x, Self::T\_MAX as u64)
}
fn commitment\_labels(commitment: &Self::AssignedCommitment) -> Vec {
PCS::commitment\_labels(commitment)
.into\_iter()
.flat\_map(|label| match label {
Collection(labels) => labels,
label => panic!("fflonk commitment tagged with {label}, not a collection"),
})
.collect()
}
fn multi\_prepare(
layouter: &mut impl Layouter,
curve\_chip: &S::CurveChip,
scalar\_chip: &S::ScalarChip,
queries: &[VerifierQuery<'\_, S, Self>],
transcript: &mut TranscriptGadget~~,
) -> Result, Error> {
// Maps the labels of every queried chunk to its commitment and the points
// its polynomials are queried at, in query order: `labels -> (commitment,
// points)`.
let chunks\_info = queries.iter().fold(BTreeMap::new(), |mut chunks\_info, q| {
let labels = (Self::commitment\_labels(q.commitment).chunks(Self::T\_MAX))
.find(|labels| labels.contains(&q.label))
.map\_or\_else(|| vec![q.label.clone()], <[\_]>::to\_vec);
let (\_, points) = chunks\_info.entry(labels).or\_insert((q.commitment, Vec::new()));
if !points.contains(&q.point) {
points.push(q.point.clone());
}
chunks\_info
});
// The evaluations of the chunks at all the points, both the explicit (taken
// from the queries) and the implicit (read from the transcript here) ones.
let mut evals: HashMap<\_, \_> = queries
.iter()
.map(|q| ((q.label.clone(), q.point.clone()), q.eval.clone()))
.collect();
for (labels, (\_, points)) in &chunks\_info {
for point in points {
for label in labels {
if let Entry::Vacant(entry) = evals.entry((label.clone(), point.clone())) {
entry.insert(transcript.read\_scalar(layouter)?);
}
}
}
}
// Opening a chunk at `x` amounts to opening its `g` polynomial at
// the `t`-th roots of `x`, where computing `g(r) = Σ\_i r^i f\_i(x)` for
// such a root `r` is equivalent to evaluating at `r` the polynomial with
// coefficients `f\_i(x)`.
let mut inner\_queries = Vec::new();
let mut roots\_cache = RootsCache::new();
for (labels, (commitment, points)) in &chunks\_info {
let t = labels.len().next\_power\_of\_two();
for x in points {
let f\_evals: Vec<\_> =
labels.iter().map(|l| &evals[&(l.clone(), x.clone())]).collect();
for root in cached\_roots(layouter, scalar\_chip, &mut roots\_cache, x, t)? {
let eval = eval\_polynomial(layouter, scalar\_chip, &f\_evals, root)?;
let label = Collection(labels.clone());
let query = VerifierQuery::::new(root, \*commitment, label, &eval);
inner\_queries.push(query.with\_root\_of(x, t));
}
}
}
PCS::multi\_prepare(
layouter,
curve\_chip,
scalar\_chip,
&inner\_queries,
transcript,
)
}
}~~~~~~~~~~
