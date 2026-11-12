// SPDX-License-Identifier: Apache-2.0
//! mori-msm: BLS12-381 G1 MSM for the Sherlock wrap prover, in Rust end to end.
//! Device code is `kernel/` (no_std Rust compiled to PTX); host code loads it through
//! the CUDA driver (cudarc). Feature `cuda` off: every call returns `None`.
#![allow(unsafe_code)]

#[cfg(feature = "cuda")]
pub mod dev;
