// SPDX-License-Identifier: Apache-2.0
//! Builds kernel/ (Rust, no_std) to PTX for nvptx64-nvidia-cuda when feature `cuda` is on.
//! The pinned toolchain is stable; RUSTC_BOOTSTRAP=1 unlocks `abi_ptx` and `stdarch_nvptx`
//! for the kernel crate only.
use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=kernel/src");
    println!("cargo:rerun-if-changed=kernel/Cargo.toml");
    if env::var_os("CARGO_FEATURE_CUDA").is_none() {
        return;
    }
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("kernel");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let tdir = dir.join("target");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let st = Command::new(cargo)
        .current_dir(&dir)
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_BUILD_TARGET")
        .env("RUSTC_BOOTSTRAP", "1")
        .env("CARGO_TARGET_DIR", &tdir)
        .args([
            "build",
            "--release",
            "--target",
            "nvptx64-nvidia-cuda",
            "--message-format",
            "short",
        ])
        .status()
        .expect("cargo for kernel");
    assert!(st.success(), "kernel build failed");
    let ptx = tdir.join("nvptx64-nvidia-cuda/release/mori_msm_kernel.ptx");
    std::fs::copy(&ptx, out.join("mori_msm_kernel.ptx")).expect("copy ptx");
}
