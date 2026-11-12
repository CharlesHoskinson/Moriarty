// SPDX-License-Identifier: Apache-2.0
//! MSM against BLST `multi_exp` (and the sppark path of mori-gpu when built with it):
//! random scalars, skewed and edge vectors, 2^10 up to 2^MORI_MSM_TEST_MAX_LG (default 16).
#![cfg(feature = "cuda")]
use ff::Field;
use group::{Curve, Group};
use midnight_curves::{Fq, G1Projective};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn points(n: usize, seed: u64) -> Vec<G1Projective> {
    let mut r = ChaCha20Rng::seed_from_u64(seed);
    let g = G1Projective::random(&mut r);
    let step = G1Projective::random(&mut r);
    let mut v = Vec::with_capacity(n);
    let mut acc = g;
    for _ in 0..n {
        v.push(acc);
        acc += step;
    }
    v
}

fn check(s: &[Fq], b: &[G1Projective], what: &str) {
    let want = G1Projective::multi_exp(&b[..s.len()], s);
    let raw = mori_msm::msm::msm_raw(s, b).unwrap_or_else(|| panic!("{what}: not taken"));
    assert_eq!(raw.to_affine(), want.to_affine(), "{what}: raw");
    let masked = mori_msm::msm_g1(s, b).unwrap_or_else(|| panic!("{what}: masked not taken"));
    assert_eq!(masked.to_affine(), want.to_affine(), "{what}: masked");
}

fn max_lg() -> u32 {
    std::env::var("MORI_MSM_TEST_MAX_LG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(16)
}

#[test]
fn small_and_edge() {
    let b = points(1 << 10, 1);
    let mut r = ChaCha20Rng::seed_from_u64(7);
    for n in [1usize, 2, 3, 17, 255, 1000] {
        let s: Vec<Fq> = (0..n).map(|_| Fq::random(&mut r)).collect();
        check(&s, &b, &format!("random n={n}"));
    }
    let n = 1 << 10;
    check(&vec![Fq::ZERO; n], &b, "all zero");
    check(&vec![Fq::from(5u64); n], &b, "all five");
    check(&vec![-Fq::ONE; n], &b, "all p-1");
    let mut one = vec![Fq::ZERO; n];
    one[n / 2] = Fq::random(&mut r);
    check(&one, &b, "single nonzero");
}

#[test]
fn sizes_random_and_skewed() {
    let top = max_lg();
    let b = points(1 << top, 2);
    let mut r = ChaCha20Rng::seed_from_u64(9);
    for lg in 10..=top {
        let n = 1usize << lg;
        let s: Vec<Fq> = (0..n).map(|_| Fq::random(&mut r)).collect();
        check(&s, &b, &format!("random 2^{lg}"));
        // Range-check-like column: small values, one hot value at 90 percent.
        let s: Vec<Fq> = (0..n)
            .map(|_| {
                if r.gen_bool(0.9) {
                    Fq::from(3u64)
                } else {
                    Fq::from(r.gen_range(0..1u64 << 16))
                }
            })
            .collect();
        check(&s, &b, &format!("skewed 2^{lg}"));
    }
}
