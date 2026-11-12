// SPDX-License-Identifier: Apache-2.0
//! mori-msm: BLS12-381 G1 MSM for the Sherlock wrap prover, in Rust end to end.
//! Device code is `kernel/` (no_std Rust compiled to PTX); host code loads it through
//! the CUDA driver (cudarc). Feature `cuda` off: every call returns `None`.
#![allow(unsafe_code)]

use midnight_curves::{Fq, G1Projective};

#[cfg(feature = "cuda")]
pub mod dev;
#[cfg(feature = "cuda")]
pub mod mask;
#[cfg(feature = "cuda")]
pub mod msm;

/// Smallest log size sent to the GPU: `MORIARTY_GPU_MIN_K` or `MORI_GPU_MIN_K`,
/// default 14, the same rule as `mori_gpu::min_k`.
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

/// Drop-in for `mori_gpu::msm_g1`: `sum scalars[i] * bases[i]` with the skew mask
/// (bases may be longer; the prefix is used). `None` means "not taken": run BLST.
/// Not taken below 2^min_k, when n > bases.len(), or without a device.
pub fn msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective> {
    #[cfg(feature = "cuda")]
    {
        let n = scalars.len();
        if n == 0 || n > bases.len() || n < (1usize << min_k()) {
            return None;
        }
        mask::msm_g1(scalars, bases)
    }
    #[cfg(not(feature = "cuda"))]
    {
        let _ = (scalars, bases);
        None
    }
}
