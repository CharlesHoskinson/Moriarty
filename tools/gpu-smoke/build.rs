// SPDX-License-Identifier: Apache-2.0
//! Compiles `cuda/kernels.cu` with nvcc into a static library in OUT_DIR and links it
//! with cudart. This is the build pattern for a later `crates/mori-gpu` (gpu.md F1).
//! Environment: `CUDA_PATH` (default `/opt/cuda`), `MORI_GPU_ARCH` (default `121`).
use std::{env, path::PathBuf, process::Command};

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let cuda = env::var("CUDA_PATH").unwrap_or_else(|_| "/opt/cuda".into());
    let arch = env::var("MORI_GPU_ARCH").unwrap_or_else(|_| "121".into());
    let nvcc = format!("{cuda}/bin/nvcc");

    let version = Command::new(&nvcc)
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("cannot run {nvcc}: {e}"));
    assert!(version.status.success(), "{nvcc} --version failed");
    let version = String::from_utf8_lossy(&version.stdout);
    let release = version
        .lines()
        .find(|l| l.contains("release"))
        .unwrap_or("unknown")
        .trim()
        .to_string();
    println!("cargo:rustc-env=GPU_SMOKE_NVCC={release}");
    println!("cargo:rustc-env=GPU_SMOKE_ARCH=sm_{arch}");

    let obj = out.join("kernels.o");
    let st = Command::new(&nvcc)
        .args(["-O3", "-std=c++17", "-Xcompiler", "-fPIC", "-gencode"])
        .arg(format!("arch=compute_{arch},code=sm_{arch}"))
        .args(["-c", "cuda/kernels.cu", "-o"])
        .arg(&obj)
        .status()
        .expect("nvcc");
    assert!(st.success(), "nvcc failed");
    let st = Command::new("ar")
        .arg("crs")
        .arg(out.join("libmorismoke.a"))
        .arg(&obj)
        .status()
        .expect("ar");
    assert!(st.success(), "ar failed");

    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-search=native={cuda}/lib64");
    println!("cargo:rustc-link-search=native={cuda}/lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{cuda}/lib64");
    println!("cargo:rustc-link-lib=static=morismoke");
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rerun-if-changed=cuda/kernels.cu");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=MORI_GPU_ARCH");
}
