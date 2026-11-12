// SPDX-License-Identifier: Apache-2.0
//! Builds kernel/ (Rust, no_std) to PTX for nvptx64-nvidia-cuda when feature `cuda` is on.
//! The pinned toolchain is stable; RUSTC_BOOTSTRAP=1 unlocks `abi_ptx` and `stdarch_nvptx`
//! for the kernel crate only.
use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=kernel/src");
    println!("cargo:rerun-if-changed=kernel/Cargo.toml");
    println!("cargo:rerun-if-env-changed=MORI_MSM_KFEAT");
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
        .args(
            env::var("MORI_MSM_KFEAT")
                .ok()
                .map(|f| format!("--features={f}")),
        )
        .status()
        .expect("cargo for kernel");
    assert!(st.success(), "kernel build failed");
    let ptx = tdir.join("nvptx64-nvidia-cuda/release/mori_msm_kernel.ptx");
    std::fs::copy(&ptx, out.join("mori_msm_kernel.ptx")).expect("copy ptx");
    // Ahead-of-time SASS for this host's GPU, so a process never pays the driver's PTX
    // JIT (126 s with an empty cache). MORI_MSM_ARCH overrides; no ptxas means PTX only.
    println!("cargo:rerun-if-env-changed=MORI_MSM_ARCH");
    println!("cargo:rustc-check-cfg=cfg(mori_msm_cubin)");
    let arch = env::var("MORI_MSM_ARCH").unwrap_or_else(|_| "sm_121".into());
    let ptxas = ["ptxas", "/opt/cuda/bin/ptxas"].into_iter().find(|p| {
        Command::new(p)
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success())
    });
    let cubin = out.join("mori_msm_kernel.cubin");
    if let Some(ptxas) = ptxas {
        let st = Command::new(ptxas)
            .args(["-O3", &format!("-arch={arch}")])
            .arg(&ptx)
            .arg("-o")
            .arg(&cubin)
            .status()
            .expect("ptxas");
        assert!(st.success(), "ptxas failed");
        println!("cargo:rustc-cfg=mori_msm_cubin");
        println!("cargo:rustc-env=MORI_MSM_ARCH={arch}");
    }
}
