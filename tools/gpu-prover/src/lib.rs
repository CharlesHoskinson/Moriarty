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

fn msm(s: &[Fq], b: &[G1Projective]) -> Option<G1Projective> {
    if on(MSM) {
        mori_gpu::msm_g1(s, b)
    } else {
        None
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
                o => panic!("unknown phase {o}"),
            }
        }),
    }
}
