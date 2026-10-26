# WLC BLS12-381 benchmark driver
> Source: https://raw.githubusercontent.com/dunkirkturbo/wlc_msm/master/381_xyzz_bal/benches/msm.rs
> Collected: 2026-10-26
> Published: Unknown

// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use criterion::{criterion\_group, criterion\_main, Criterion};
use ark\_bls12\_381::G1Affine;
use ark\_ff::BigInteger256;
use std::str::FromStr;
use blst\_msm::\*;
fn criterion\_benchmark(c: &mut Criterion) {
let bench\_npow = std::env::var("BENCH\_NPOW").unwrap\_or("20".to\_string());
let npoints\_npow = i32::from\_str(&bench\_npow).unwrap();
let batches = 16;
let (points, scalars) =
util::generate\_points\_scalars::(1usize << npoints\_npow, batches);
let mut context = multi\_scalar\_mult\_init(points.as\_slice());
let mut group = c.benchmark\_group("CUDA");
group.sample\_size(10);
let name = format!("2\*\*{}x{}", npoints\_npow, batches);
group.bench\_function(name, |b| {
b.iter(|| {
let \_ = multi\_scalar\_mult(&mut context, &points.as\_slice(), unsafe {
std::mem::transmute::<&[\_], &[BigInteger256]>(
scalars.as\_slice(),
)
});
})
});
group.finish();
}
criterion\_group!(benches, criterion\_benchmark);
criterion\_main!(benches);