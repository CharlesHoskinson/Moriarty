// SPDX-License-Identifier: Apache-2.0
//! Registers mori-gpu in the hook-patched midnight-proofs (`midnight_proofs::hooks`).
//! Phases are switched at run time, so one process can prove on the CPU path and on
//! the GPU path with the same key: `set_phases(0)` is the stock CPU prover.
#![allow(unsafe_code)]

use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

use midnight_curves::{Fq, G1Projective};
use midnight_proofs::hooks::{self, DevBuf, GpuHooks, QuotientJob};

pub const MSM: u32 = 1;
pub const NTT: u32 = 2;
pub const COSET: u32 = 4;
pub const QUOTIENT: u32 = 8;
pub const ALL: u32 = MSM | NTT | COSET | QUOTIENT;
/// MSM on the Rust kernel (`mori-msm`) instead of sppark. `MORI_MSM=rust` also routes
/// the `msm` phase there.
pub const MSM_RUST: u32 = 16;

static PHASES: AtomicU32 = AtomicU32::new(0);

pub fn set_phases(mask: u32) {
    PHASES.store(mask, Relaxed);
    hooks::QUOTIENT_ON.store(mask & QUOTIENT != 0, Relaxed);
}

pub fn phases() -> u32 {
    PHASES.load(Relaxed)
}

fn on(bit: u32) -> bool {
    PHASES.load(Relaxed) & bit != 0
}

fn rust_msm_selected() -> bool {
    static R: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *R.get_or_init(|| std::env::var("MORI_MSM").as_deref() == Ok("rust"))
}

fn msm(s: &[Fq], b: &[G1Projective]) -> Option<G1Projective> {
    // Record the proof's commitments on the CPU config only (phases 0), not the small
    // verifier MSMs.
    if phases() == 0 && s.len() >= 1usize << mori_msm::min_k() {
        record::call(s, b);
    }
    if on(MSM_RUST) || (on(MSM) && rust_msm_selected()) {
        let n = s.len();
        if n == 0 || n > b.len() || n < (1usize << mori_msm::min_k()) {
            return None;
        }
        let t = std::time::Instant::now();
        let r = mori_msm::msm_g1(s, b);
        mori_gpu::record_msm(r.is_some(), t.elapsed().as_nanos() as u64);
        r
    } else if on(MSM) {
        mori_gpu::msm_g1(s, b)
    } else {
        None
    }
}

/// Test-only recording of the MSM inputs of a proof for the K2 benchmarks
/// (`MORI_MSM_RECORD=<dir>`): `call-NNNN-n<N>-b<len>.bin` holds the scalars as blst_fr
/// words, `bases-<len>.bin` the affine bases of each table once. Only calls of the
/// stock CPU config (phases 0) at or above 2^min_k are recorded. The scalars are
/// witness data; record only seeded test runs, never production witnesses.
mod record {
    use std::sync::{
        Mutex,
        atomic::{AtomicU32, Ordering::Relaxed},
    };

    use group::Curve;
    use midnight_curves::{Fq, G1Affine, G1Projective};

    static N: AtomicU32 = AtomicU32::new(0);
    static SEEN: Mutex<Vec<usize>> = Mutex::new(Vec::new());

    pub fn call(s: &[Fq], b: &[G1Projective]) {
        let Some(dir) = std::env::var_os("MORI_MSM_RECORD") else {
            return;
        };
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).unwrap();
        let i = N.fetch_add(1, Relaxed);
        let raw = unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len() * 32) };
        std::fs::write(
            dir.join(format!("call-{i:04}-n{}-b{}.bin", s.len(), b.len())),
            raw,
        )
        .unwrap();
        let mut seen = SEEN.lock().unwrap();
        if !seen.contains(&b.len()) {
            seen.push(b.len());
            let mut aff = vec![G1Affine::default(); b.len()];
            G1Projective::batch_normalize(b, &mut aff);
            let raw =
                unsafe { std::slice::from_raw_parts(aff.as_ptr() as *const u8, aff.len() * 96) };
            std::fs::write(dir.join(format!("bases-{}.bin", b.len())), raw).unwrap();
        }
    }
}
fn fft(a: &mut [Fq], w: Fq, lg: u32) -> bool {
    on(NTT) && mori_gpu::fft(a, w, lg)
}
fn ifft(a: &mut [Fq], w: Fq, lg: u32, d: Fq) -> bool {
    on(NTT) && mori_gpu::ifft(a, w, lg, d)
}
fn coset(c: &[Fq], out: &mut [Fq], k: u32, ek: u32, z: [Fq; 2]) -> bool {
    on(COSET) && mori_gpu::coeff_to_extended(c, out, k, ek, z)
}
fn dev_coset(c: &[Fq], k: u32, ek: u32, z: [Fq; 2]) -> Option<DevBuf> {
    if !on(QUOTIENT) {
        return None;
    }
    let d = mori_gpu::quotient::DevPoly::coeff_to_extended(c, k, ek, z)?;
    Some(DevBuf {
        ptr: d.as_ptr(),
        len: d.len(),
        owner: Box::new(d),
    })
}
unsafe fn quotient(job: *mut QuotientJob, nslots: u32, out: &mut [Fq]) -> bool {
    const _: () = assert!(
        std::mem::size_of::<QuotientJob>()
            == std::mem::size_of::<mori_gpu::quotient::QuotientJob>()
    );
    const _: () = assert!(hooks::QUOTIENT_ABI == mori_gpu::quotient::QUOTIENT_ABI);
    on(QUOTIENT)
        && unsafe {
            mori_gpu::quotient::run(job as *mut mori_gpu::quotient::QuotientJob, nslots, out)
        }
}

/// Registers the hooks (once) and enables `mask`. Returns the device, if any.
pub fn install(mask: u32) -> Option<&'static mori_gpu::GpuInfo> {
    let info = mori_gpu::info()?;
    hooks::install(GpuHooks {
        msm_g1: Some(msm),
        fft: Some(fft),
        ifft: Some(ifft),
        coeff_to_extended: Some(coset),
        dev_coeff_to_extended: Some(dev_coset),
        quotient: Some(quotient),
    });
    set_phases(mask);
    Some(info)
}

/// Parses `MORI_GPU_PHASES` style lists: "all", "cpu", or "msm,ntt,coset,quotient".
pub mod proxy;

pub fn parse_phases(s: &str) -> u32 {
    match s {
        "all" | "gpu" => ALL,
        "cpu" | "none" | "" => 0,
        _ => s.split(',').fold(0, |m, p| {
            m | match p.trim() {
                "msm" => MSM,
                "ntt" => NTT,
                "coset" => COSET,
                "quotient" => QUOTIENT,
                "msmrs" => MSM_RUST,
                o => panic!("unknown phase {o}"),
            }
        }),
    }
}
