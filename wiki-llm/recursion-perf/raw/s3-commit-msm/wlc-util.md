# WLC BLS12-381 input generator
> Source: https://raw.githubusercontent.com/dunkirkturbo/wlc_msm/master/381_xyzz_bal/src/util.rs
> Collected: 2026-10-26
> Published: Unknown

// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use rand::SeedableRng;
use rand\_chacha::ChaCha20Rng;
use ark\_ec::{AffineCurve, ProjectiveCurve};
use ark\_std::UniformRand;
use ark\_ff::prelude::\*;
use ark\_std::vec::Vec;
pub fn generate\_points\_scalars(
len: usize,
batch\_size: usize
) -> (Vec, Vec) {
let rand\_gen: usize = 1 << 11;
let mut rng = ChaCha20Rng::from\_entropy();
let mut points =
::batch\_normalization\_into\_affine(
&(0..rand\_gen)
.map(|\_| G::Projective::rand(&mut rng))
.collect::>(),
);
// Sprinkle in some infinity points
// points[3] = G::zero();
while points.len() < len {
points.append(&mut points.clone());
}
let points = &points[0..len];
let scalars = (0..len \* batch\_size)
.map(|\_| G::ScalarField::rand(&mut rng))
.collect::>();
(points.to\_vec(), scalars)
}
#[cfg(feature = "parallel")]
use rayon::prelude::\*;
pub struct VariableBaseMSM2;
impl VariableBaseMSM2 {
pub fn multi\_scalar\_mul(
bases: &[G],
scalars: &[::BigInt],
) -> G::Projective {
let size = ark\_std::cmp::min(bases.len(), scalars.len());
let scalars = &scalars[..size];
let bases = &bases[..size];
let scalars\_and\_bases\_iter = scalars.iter().zip(bases).filter(|(s, \_)| !s.is\_zero());
// let c = if size < 32 {
// 3
// } else {
// super::ln\_without\_floats(size) + 2
// };
let c = 21;
let num\_bits = ::Params::MODULUS\_BITS as usize;
let fr\_one = G::ScalarField::one().into\_repr();
let zero = G::Projective::zero();
let window\_starts: Vec<\_> = (0..num\_bits).step\_by(c).collect();
// Each window is of size `c`.
// We divide up the bits 0..num\_bits into windows of size `c`, and
// in parallel process each such window.
let window\_sums: Vec<\_> = ark\_std::cfg\_into\_iter!(window\_starts)
.map(|w\_start| {
let mut res = zero;
// let mut count = 0;
// We don't need the "zero" bucket, so we only have 2^c - 1 buckets.
let mut buckets = vec![zero; (1 << c) - 1];
// This clone is cheap, because the iterator contains just a
// pointer and an index into the original vectors.
scalars\_and\_bases\_iter.clone().for\_each(|(&scalar, base)| {
if scalar == fr\_one {
// We only process unit scalars once in the first window.
if w\_start == 0 {
res.add\_assign\_mixed(base);
}
} else {
let mut scalar = scalar;
// We right-shift by w\_start, thus getting rid of the
// lower bits.
scalar.divn(w\_start as u32);
// We mod the remaining bits by 2^{window size}, thus taking `c` bits.
let scalar = scalar.as\_ref()[0] % (1 << c);
// If the scalar is non-zero, we update the corresponding
// bucket.
// (Recall that `buckets` doesn't have a zero bucket.)
// if w\_start != 252{
if scalar != 0 {
buckets[(scalar - 1) as usize].add\_assign\_mixed(base);
}
// }
}
});
// Compute sum\_{i in 0..num\_buckets} (sum\_{j in i..num\_buckets} bucket[j])
// This is computed below for b buckets, using 2b curve additions.
//
// We could first normalize `buckets` and then use mixed-addition
// here, but that's slower for the kinds of groups we care about
// (Short Weierstrass curves and Twisted Edwards curves).
// In the case of Short Weierstrass curves,
// mixed addition saves ~4 field multiplications per addition.
// However normalization (with the inversion batched) takes ~6
// field multiplications per element,
// hence batch normalization is a slowdown.
// `running\_sum` = sum\_{j in i..num\_buckets} bucket[j],
// where we iterate backward from i = num\_buckets to 0.
let mut running\_sum = G::Projective::zero();
buckets.into\_iter().rev().for\_each(|b| {
running\_sum += &b;
res += &running\_sum;
});
res
})
.collect();
// We store the sum for the lowest window.
let lowest = \*window\_sums.first().unwrap();
// let mm = window\_sums[1..]
// .iter()
// .rev()
// .fold(zero, |mut total, sum\_i| {
// total += sum\_i;
// // for \_ in 0..c {
// // total.double\_in\_place();
// // }
// total
// });
// We're traversing windows from high to low.
lowest
+ &window\_sums[1..]
.iter()
.rev()
.fold(zero, |mut total, sum\_i| {
total += sum\_i;
for \_ in 0..c {
total.double\_in\_place();
}
total
})
// if mm == zero{
// panic!("zero!");
// }
// mm
}
}