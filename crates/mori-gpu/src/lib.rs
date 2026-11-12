// SPDX-License-Identifier: Apache-2.0
//! mori-gpu: GPU backend for the Sherlock wrap prover (design 3.7, gpu.md F1).
//!
//! BLS12-381 G1 MSM and scalar-field NTT through sppark templates, a coset transform and
//! the quotient numerator kernel, all behind safe Rust functions that return `None` /
//! `false` when the GPU path is not taken, so the caller runs the stock CPU code. The
//! GPU is never a correctness dependency. Fallback levels: compile time (feature `cuda`
//! off), init time (no device, or `MORI_GPU=off`), call time (`n < 2^MORI_GPU_MIN_K`,
//! a non-standard root of unity, or a kernel error; counted in `stats().fallbacks`).
//!
//! Field and point layouts are the host's: `Fq` is blst_fr (4 x u64 Montgomery,
//! R = 2^256), `G1Projective` is blst_p1 (Jacobian), `G1Affine` is blst_p1_affine. The
//! kernels read the same bytes as 32-bit limbs; there is no conversion pass.
#![allow(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

use ff::{Field, PrimeField};
use midnight_curves::{Fq, G1Projective};

pub mod quotient;

#[cfg(feature = "cuda")]
mod ffi;

/// Counters for receipts.
#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub msm_calls: u64,
    pub msm_ns: u64,
    pub ntt_calls: u64,
    pub ntt_ns: u64,
    pub coset_calls: u64,
    pub quotient_calls: u64,
    pub quotient_ns: u64,
    pub fallbacks: u64,
}

static MSM_CALLS: AtomicU64 = AtomicU64::new(0);
static MSM_NS: AtomicU64 = AtomicU64::new(0);
static NTT_CALLS: AtomicU64 = AtomicU64::new(0);
static NTT_NS: AtomicU64 = AtomicU64::new(0);
static COSET_CALLS: AtomicU64 = AtomicU64::new(0);
static Q_CALLS: AtomicU64 = AtomicU64::new(0);
static Q_NS: AtomicU64 = AtomicU64::new(0);
static FALLBACKS: AtomicU64 = AtomicU64::new(0);

pub fn stats() -> Stats {
    Stats {
        msm_calls: MSM_CALLS.load(Relaxed),
        msm_ns: MSM_NS.load(Relaxed),
        ntt_calls: NTT_CALLS.load(Relaxed),
        ntt_ns: NTT_NS.load(Relaxed),
        coset_calls: COSET_CALLS.load(Relaxed),
        quotient_calls: Q_CALLS.load(Relaxed),
        quotient_ns: Q_NS.load(Relaxed),
        fallbacks: FALLBACKS.load(Relaxed),
    }
}

pub fn reset_stats() {
    for c in [
        &MSM_CALLS,
        &MSM_NS,
        &NTT_CALLS,
        &NTT_NS,
        &COSET_CALLS,
        &Q_CALLS,
        &Q_NS,
        &FALLBACKS,
    ] {
        c.store(0, Relaxed);
    }
}

static TABLE_BUILDS: AtomicU64 = AtomicU64::new(0);
static HEAVY_SPLITS: AtomicU64 = AtomicU64::new(0);

pub(crate) fn note_heavy_split() {
    HEAVY_SPLITS.fetch_add(1, Relaxed);
}

/// MSM calls split by the bucket skew guard since process start.
pub fn heavy_splits() -> u64 {
    HEAVY_SPLITS.load(Relaxed)
}

pub(crate) fn note_table_build() {
    TABLE_BUILDS.fetch_add(1, Relaxed);
}

/// MSM base tables built (affine conversion and upload) since process start.
pub fn table_builds() -> u64 {
    TABLE_BUILDS.load(Relaxed)
}

/// Counts an MSM taken by another backend (the Rust MSM in `mori-msm`) in these stats,
/// so receipts read one set of counters whichever MSM ran. `ok = false` is a fallback.
pub fn record_msm(ok: bool, ns: u64) {
    if ok {
        MSM_CALLS.fetch_add(1, Relaxed);
        MSM_NS.fetch_add(ns, Relaxed);
    } else {
        note_fallback();
    }
}

pub(crate) fn note_fallback() {
    FALLBACKS.fetch_add(1, Relaxed);
}

/// Device facts recorded in receipts.
#[derive(Clone, Debug)]
pub struct GpuInfo {
    pub name: String,
    pub sm_count: i32,
    pub cc: (i32, i32),
    pub arch: &'static str,
    pub nvcc: &'static str,
}

/// `Some` when the CUDA core is compiled in, a device answers and `MORI_GPU != off`.
pub fn info() -> Option<&'static GpuInfo> {
    #[cfg(feature = "cuda")]
    {
        ffi::info()
    }
    #[cfg(not(feature = "cuda"))]
    {
        None
    }
}

/// Smallest log size sent to the GPU (`MORI_GPU_MIN_K`, default 14).
pub fn min_k() -> u32 {
    static K: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *K.get_or_init(|| {
        std::env::var("MORIARTY_GPU_MIN_K")
            .or_else(|_| std::env::var("MORI_GPU_MIN_K"))
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(14)
    })
}

/// The standard 2^log_n root of unity (as `EvaluationDomain` derives it) and its inverse.
pub fn std_root(log_n: u32) -> (Fq, Fq) {
    let mut w = Fq::ROOT_OF_UNITY;
    for _ in log_n..Fq::S {
        w = w.square();
    }
    (w, w.invert().unwrap())
}

/// G1 MSM `sum scalars[i] * bases[i]` (bases may be longer; the prefix is used).
/// `None` means "not taken": the caller runs BLST.
pub fn msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective> {
    #[cfg(feature = "cuda")]
    {
        let n = scalars.len();
        if n == 0 || n > bases.len() || n < (1usize << min_k()) || info().is_none() {
            return None;
        }
        let t = std::time::Instant::now();
        let r = ffi::msm_g1(scalars, bases);
        if r.is_some() {
            MSM_CALLS.fetch_add(1, Relaxed);
            MSM_NS.fetch_add(t.elapsed().as_nanos() as u64, Relaxed);
        } else {
            note_fallback();
        }
        r
    }
    #[cfg(not(feature = "cuda"))]
    {
        let _ = (scalars, bases);
        None
    }
}

/// In-place radix-2 DFT matching `midnight_curves::fft::best_fft(a, omega, log_n)`.
/// Taken only for the standard roots (forward or inverse). Returns `false` if not taken.
pub fn fft(a: &mut [Fq], omega: Fq, log_n: u32) -> bool {
    transform(a, omega, log_n, None)
}

/// `best_fft(a, omega_inv, log_n)` followed by `a[i] *= divisor`
/// (`EvaluationDomain::ifft`). Returns `false` if not taken.
pub fn ifft(a: &mut [Fq], omega_inv: Fq, log_n: u32, divisor: Fq) -> bool {
    transform(a, omega_inv, log_n, Some(divisor))
}

#[allow(unused_variables)]
fn transform(a: &mut [Fq], omega: Fq, log_n: u32, divisor: Option<Fq>) -> bool {
    #[cfg(feature = "cuda")]
    {
        if a.len() != 1usize << log_n || log_n < min_k() || log_n > 28 || info().is_none() {
            return false;
        }
        let (w, wi) = std_root(log_n);
        let t = std::time::Instant::now();
        let ok = if omega == w && divisor.is_none() {
            ffi::ntt(a, log_n, false, None)
        } else if omega == wi {
            // sppark's inverse includes the 1/n scaling; fix up any other divisor.
            let n_f = Fq::from(1u64 << log_n);
            let n_inv = n_f.invert().unwrap();
            let post = match divisor {
                Some(d) if d == n_inv => None,
                Some(d) => Some(d * n_f),
                None => Some(n_f),
            };
            ffi::ntt(a, log_n, true, post)
        } else {
            false
        };
        if ok {
            NTT_CALLS.fetch_add(1, Relaxed);
            NTT_NS.fetch_add(t.elapsed().as_nanos() as u64, Relaxed);
        } else {
            note_fallback();
        }
        ok
    }
    #[cfg(not(feature = "cuda"))]
    false
}

/// `coeff_to_extended`: distribute powers of zeta (into the coset) over the n
/// coefficients, zero-extend to 2^ext_k and transform at the extended root.
/// Writes into `out` (length 2^ext_k). Returns `false` if not taken.
#[allow(unused_variables)]
pub fn coeff_to_extended(coeffs: &[Fq], out: &mut [Fq], k: u32, ext_k: u32, zeta: [Fq; 2]) -> bool {
    #[cfg(feature = "cuda")]
    {
        if coeffs.len() != 1usize << k
            || out.len() != 1usize << ext_k
            || ext_k < min_k()
            || ext_k > 28
            || info().is_none()
        {
            return false;
        }
        let t = std::time::Instant::now();
        let ok = ffi::coeff_to_extended(coeffs, out, k, ext_k, zeta);
        if ok {
            COSET_CALLS.fetch_add(1, Relaxed);
            NTT_CALLS.fetch_add(1, Relaxed);
            NTT_NS.fetch_add(t.elapsed().as_nanos() as u64, Relaxed);
        } else {
            note_fallback();
        }
        ok
    }
    #[cfg(not(feature = "cuda"))]
    false
}

pub(crate) fn add_quotient_time(ns: u64) {
    Q_CALLS.fetch_add(1, Relaxed);
    Q_NS.fetch_add(ns, Relaxed);
}

pub(crate) fn note_coset(ns: u64) {
    COSET_CALLS.fetch_add(1, Relaxed);
    NTT_CALLS.fetch_add(1, Relaxed);
    NTT_NS.fetch_add(ns, Relaxed);
}

/// Returns the transform scratch buffer to the system (call between proofs).
pub fn release_scratch() {
    #[cfg(feature = "cuda")]
    ffi::release_scratch();
}

/// Free and total device memory in bytes (on this host: the shared system memory).
pub fn mem_info() -> Option<(usize, usize)> {
    #[cfg(feature = "cuda")]
    {
        info()?;
        let (mut f, mut t) = (0usize, 0usize);
        let _g = ffi::GPU.lock().unwrap();
        (unsafe { ffi::mori_gpu_mem_info(&mut f, &mut t) } == 0).then_some((f, t))
    }
    #[cfg(not(feature = "cuda"))]
    None
}

static DEV_CUR: AtomicU64 = AtomicU64::new(0);
static DEV_PEAK: AtomicU64 = AtomicU64::new(0);

/// Tracks device allocations made by this crate (MSM tables are counted as their
/// point bytes; sppark's per-call MSM and NTT scratch is not counted).
pub(crate) fn dev_add(bytes: u64) {
    let c = DEV_CUR.fetch_add(bytes, Relaxed) + bytes;
    DEV_PEAK.fetch_max(c, Relaxed);
}

pub(crate) fn dev_sub(bytes: u64) {
    DEV_CUR.fetch_sub(bytes, Relaxed);
}

/// Peak bytes of this crate's device allocations since the last reset.
pub fn device_bytes_peak() -> u64 {
    DEV_PEAK.load(Relaxed)
}

pub fn reset_device_bytes_peak() {
    DEV_PEAK.store(DEV_CUR.load(Relaxed), Relaxed);
}
