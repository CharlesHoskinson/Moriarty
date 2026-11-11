//! Kernel micro-benchmarks against the CPU functions the prover calls
//! (BLST `multi_exp`, `best_fft`). Prints one JSON line per measurement.
//! Usage: bench [msm|ntt|all]   (run under the heavy lock on this host)
use std::time::Instant;

use ff::Field;
use group::{Curve, Group};
use midnight_curves::{Fq, G1Projective, fft::best_fft};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let what = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    let info = mori_gpu::info().expect("GPU");
    eprintln!("{info:?} threads={}", rayon::current_num_threads());
    let mut r = ChaCha20Rng::seed_from_u64(42);
    if what == "msm" || what == "all" {
        let nmax = 1 << 21;
        let g = G1Projective::random(&mut r);
        let step = G1Projective::random(&mut r);
        let mut bases = Vec::with_capacity(nmax);
        let mut acc = g;
        for _ in 0..nmax {
            bases.push(acc);
            acc += step;
        }
        let scal: Vec<Fq> = (0..nmax).map(|_| Fq::random(&mut r)).collect();
        for lg in [16u32, 19, 20, 21] {
            let n = 1usize << lg;
            let b = &bases[..n];
            let t = Instant::now();
            let first = mori_gpu::msm_g1(&scal[..n], b).unwrap();
            let first_s = t.elapsed().as_secs_f64();
            let mut gs = vec![];
            for _ in 0..5 {
                let t = Instant::now();
                let x = mori_gpu::msm_g1(&scal[..n], b).unwrap();
                gs.push(t.elapsed().as_secs_f64());
                assert_eq!(x.to_affine(), first.to_affine());
            }
            let mut cs = vec![];
            let mut want = G1Projective::identity();
            for _ in 0..3 {
                let t = Instant::now();
                want = G1Projective::multi_exp(b, &scal[..n]);
                cs.push(t.elapsed().as_secs_f64());
            }
            assert_eq!(want.to_affine(), first.to_affine());
            println!(
                "{{\"kind\":\"msm\",\"log_n\":{lg},\"gpu_first_s\":{first_s:.4},\"gpu_s\":{:.4},\"cpu_blst_s\":{:.4},\"equal\":true}}",
                med(gs),
                med(cs)
            );
        }
    }
    if what == "ntt" || what == "all" {
        for lg in [19u32, 20, 21, 22, 23, 24] {
            let n = 1usize << lg;
            let v: Vec<Fq> = (0..n).map(|_| Fq::random(&mut r)).collect();
            let (w, _) = mori_gpu::std_root(lg);
            let mut cpu = v.clone();
            let mut cs = vec![];
            for i in 0..3 {
                let mut x = v.clone();
                let t = Instant::now();
                best_fft(&mut x, w, lg);
                cs.push(t.elapsed().as_secs_f64());
                if i == 0 {
                    cpu = x;
                }
            }
            let mut gs = vec![];
            for _ in 0..5 {
                let mut x = v.clone();
                let t = Instant::now();
                assert!(mori_gpu::fft(&mut x, w, lg));
                gs.push(t.elapsed().as_secs_f64());
                assert!(x == cpu);
            }
            println!(
                "{{\"kind\":\"ntt\",\"log_n\":{lg},\"gpu_s\":{:.4},\"cpu_best_fft_s\":{:.4},\"equal\":true}}",
                med(gs),
                med(cs)
            );
        }
    }
}
