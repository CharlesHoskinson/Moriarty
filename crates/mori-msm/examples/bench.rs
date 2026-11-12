// SPDX-License-Identifier: Apache-2.0
//! MSM benchmark: mori-msm (raw and masked), BLST and, with feature `sppark`, mori-gpu.
//! Usage: bench <min_lg> <max_lg> <reps> [random|skew|file:<dir>]...
//! Prints one JSON line per (input, size, implementation).
use std::time::Instant;

use ff::Field;
use group::{Curve, Group};
use midnight_curves::{Fq, G1Projective};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn stats(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (v[0], v[v.len() / 2])
}

fn time<T>(reps: usize, mut f: impl FnMut() -> T) -> (T, f64, Vec<f64>) {
    let t = Instant::now();
    let first = f();
    let first_s = t.elapsed().as_secs_f64();
    let mut v = vec![];
    for _ in 0..reps {
        let t = Instant::now();
        let _ = f();
        v.push(t.elapsed().as_secs_f64());
    }
    (first, first_s, v)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let lo: u32 = a[1].parse().unwrap();
    let hi: u32 = a[2].parse().unwrap();
    let reps: usize = a[3].parse().unwrap();
    let inputs: Vec<String> = if a.len() > 4 {
        a[4..].to_vec()
    } else {
        vec!["random".into()]
    };
    let skip_blst = std::env::var_os("BENCH_NO_BLST").is_some();
    let nmax = 1usize << hi;
    let mut r = ChaCha20Rng::seed_from_u64(42);
    let g = G1Projective::random(&mut r);
    let step = G1Projective::random(&mut r);
    let mut bases = Vec::with_capacity(nmax);
    let mut acc = g;
    for _ in 0..nmax {
        bases.push(acc);
        acc += step;
    }
    for input in &inputs {
        let scal: Vec<Fq> = match input.as_str() {
            "random" => (0..nmax).map(|_| Fq::random(&mut r)).collect(),
            "skew" => (0..nmax)
                .map(|_| {
                    if r.gen_bool(0.9) {
                        Fq::from(3u64)
                    } else {
                        Fq::from(r.gen_range(0..1u64 << 16))
                    }
                })
                .collect(),
            _ => panic!("input {input}"),
        };
        for lg in lo..=hi {
            let n = 1usize << lg;
            let b = &bases[..n];
            let s = &scal[..n];
            let want = if skip_blst {
                None
            } else {
                let (w, _, v) = time(reps, || G1Projective::multi_exp(b, s));
                let (mn, md) = stats(v);
                println!(
                    "{{\"input\":\"{input}\",\"log_n\":{lg},\"impl\":\"blst\",\"min_s\":{mn:.5},\"median_s\":{md:.5},\"reps\":{reps}}}"
                );
                Some(w.to_affine())
            };
            for (name, masked) in [("rust_raw", false), ("rust_masked", true)] {
                let (x, first_s, v) = time(reps, || {
                    if masked {
                        mori_msm::msm_g1(s, b).unwrap()
                    } else {
                        mori_msm::msm::msm_raw(s, b).unwrap()
                    }
                });
                if let Some(w) = want {
                    assert_eq!(x.to_affine(), w, "{name} {input} 2^{lg}");
                }
                let (mn, md) = stats(v);
                println!(
                    "{{\"input\":\"{input}\",\"log_n\":{lg},\"impl\":\"{name}\",\"first_s\":{first_s:.5},\"min_s\":{mn:.5},\"median_s\":{md:.5},\"reps\":{reps},\"equal_blst\":{}}}",
                    want.is_some()
                );
            }
        }
    }
}
