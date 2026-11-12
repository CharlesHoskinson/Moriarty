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

/// Host load and GPU clock at this moment (the benchmark gate records both per cell).
fn host() -> String {
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let smi = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=clocks.sm,utilization.gpu,clocks_throttle_reasons.active",
            "--format=csv,noheader",
        ])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    format!("\"loadavg\":\"{}\",\"gpu\":\"{}\"", load.trim(), smi)
}

fn emit(input: &str, lg: u32, name: &str, first_s: Option<f64>, v: Vec<f64>, eq: bool) {
    let reps = v.len();
    let (mn, md) = stats(v);
    let first = first_s
        .map(|f| format!("\"first_s\":{f:.5},"))
        .unwrap_or_default();
    println!(
        "{{\"input\":\"{input}\",\"log_n\":{lg},\"impl\":\"{name}\",{first}\"min_s\":{mn:.5},\"median_s\":{md:.5},\"reps\":{reps},\"equal_blst\":{eq},{}}}",
        host()
    );
}

/// One cell: BLST (unless skipped), mori-msm raw and masked, sppark raw and masked.
fn cell(input: &str, lg: u32, s: &[Fq], b: &[G1Projective], reps: usize, skip_blst: bool) {
    let want = if skip_blst {
        None
    } else {
        let (w, _, v) = time(reps, || G1Projective::multi_exp(&b[..s.len()], s));
        emit(input, lg, "blst", None, v, true);
        Some(w.to_affine())
    };
    for (name, masked) in [("rust_raw", false), ("rust_masked", true)] {
        let (x, first_s, v) = time(reps, || {
            if masked {
                mori_msm::mask::msm_g1(s, b).unwrap()
            } else {
                mori_msm::msm::msm_raw(s, b).unwrap()
            }
        });
        if let Some(w) = want {
            assert_eq!(x.to_affine(), w, "{name} {input} 2^{lg}");
        }
        emit(input, lg, name, Some(first_s), v, want.is_some());
    }
    #[cfg(feature = "sppark")]
    for (name, mask) in [("sppark_raw", "0"), ("sppark_masked", "1")] {
        // SAFETY: the benchmark is single threaded; mori-gpu reads the variable per call.
        unsafe { std::env::set_var("MORI_GPU_MSM_MASK", mask) };
        let (x, first_s, v) = time(reps, || mori_gpu::msm_g1(s, b).unwrap());
        if let Some(w) = want {
            assert_eq!(x.to_affine(), w, "{name} {input} 2^{lg}");
        }
        emit(input, lg, name, Some(first_s), v, want.is_some());
    }
}

/// Recorded columns from the hook (`MORI_MSM_RECORD`): `call-NNNN-n<N>-b<len>.bin` holds
/// the scalars (blst_fr words), `bases-<len>.bin` the affine bases of that table.
fn recorded(dir: &str) -> Vec<(String, Vec<Fq>, usize)> {
    let mut v: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().into_string().unwrap())
        .filter(|f| f.starts_with("call-"))
        .collect();
    v.sort();
    let want: Option<Vec<String>> = std::env::var("BENCH_CALLS")
        .ok()
        .map(|s| s.split(',').map(String::from).collect());
    v.into_iter()
        .filter(|f| {
            want.as_ref()
                .is_none_or(|w| w.iter().any(|c| f.contains(c.as_str())))
        })
        .map(|f| {
            let bytes = std::fs::read(format!("{dir}/{f}")).unwrap();
            let n = bytes.len() / 32;
            let mut sc = vec![Fq::ZERO; n];
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), sc.as_mut_ptr() as *mut u8, n * 32)
            };
            let blen: usize = f
                .split("-b")
                .nth(1)
                .unwrap()
                .trim_end_matches(".bin")
                .parse()
                .unwrap();
            (f, sc, blen)
        })
        .collect()
}

fn load_bases(dir: &str, len: usize) -> Vec<G1Projective> {
    let bytes = std::fs::read(format!("{dir}/bases-{len}.bin")).unwrap();
    assert_eq!(bytes.len(), len * 96);
    let mut a = vec![midnight_curves::G1Affine::default(); len];
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), a.as_mut_ptr() as *mut u8, len * 96) };
    a.iter().map(|p| G1Projective::from(*p)).collect()
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
    println!("{{\"start\":true,{}}}", host());
    let mut r = ChaCha20Rng::seed_from_u64(42);
    let mut bases: Vec<G1Projective> = vec![];
    for input in &inputs {
        if let Some(dir) = input.strip_prefix("rec:") {
            let mut cur: (usize, Vec<G1Projective>) = (0, vec![]);
            for (name, sc, blen) in recorded(dir) {
                if cur.0 != blen {
                    cur = (blen, load_bases(dir, blen));
                }
                let lg = usize::BITS - sc.len().leading_zeros() - 1;
                if lg < lo || lg > hi {
                    continue;
                }
                cell(&format!("rec:{name}"), lg, &sc, &cur.1, reps, skip_blst);
            }
            continue;
        }
        let nmax = 1usize << hi;
        if bases.len() < nmax {
            let g = G1Projective::random(&mut r);
            let step = G1Projective::random(&mut r);
            let mut acc = g;
            bases.clear();
            for _ in 0..nmax {
                bases.push(acc);
                acc += step;
            }
        }
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
            cell(input, lg, &scal[..n], &bases[..n], reps, skip_blst);
        }
    }
    println!("{{\"end\":true,{}}}", host());
}
