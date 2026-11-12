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

/// Drop-in for `mori_gpu::msm_g1`: `sum scalars[i] * bases[i]` with the skew mask
/// (bases may be longer; the prefix is used). `None` means "not taken": run BLST.
pub fn msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective> {
    #[cfg(feature = "cuda")]
    {
        mask::msm_g1(scalars, bases)
    }
    #[cfg(not(feature = "cuda"))]
    {
        let _ = (scalars, bases);
        None
    }
}
