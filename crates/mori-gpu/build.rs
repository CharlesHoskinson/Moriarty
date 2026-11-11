// SPDX-License-Identifier: Apache-2.0
//! With feature `cuda`: compiles `cuda/mori_gpu.cu` (sppark templates from
//! `third_party/sppark`, blst host headers from the `blst` crate) with nvcc into a
//! static library and links cudart. Without it: nothing, the crate is the CPU path.
//! Environment: `CUDA_PATH` (default `/opt/cuda`), `MORI_GPU_ARCH` (default `121`).
use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=cuda");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=MORI_GPU_ARCH");
    if env::var_os("CARGO_FEATURE_CUDA").is_none() {
        return;
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cuda = env::var("CUDA_PATH").unwrap_or_else(|_| "/opt/cuda".into());
    let arch = env::var("MORI_GPU_ARCH").unwrap_or_else(|_| "121".into());
    let nvcc = format!("{cuda}/bin/nvcc");
    let blst_src = env::var("DEP_BLST_C_SRC").expect("DEP_BLST_C_SRC from the blst crate");
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sppark = manifest.join("third_party/sppark");

    let v = Command::new(&nvcc).arg("--version").output().expect("nvcc");
    let v = String::from_utf8_lossy(&v.stdout);
    let release = v
        .lines()
        .find(|l| l.contains("release"))
        .unwrap_or("unknown")
        .trim();
    println!("cargo:rustc-env=MORI_GPU_NVCC={release}");
    println!("cargo:rustc-env=MORI_GPU_ARCH=sm_{arch}");

    let obj = out.join("mori_gpu.o");
    let st = Command::new(&nvcc)
        .args([
            "-O3",
            "-std=c++17",
            "-Xcompiler",
            "-fPIC",
            "-Xcompiler",
            "-Wno-unused-function",
        ])
        .args([
            "-DFEATURE_BLS12_381",
            "-DTAKE_RESPONSIBILITY_FOR_ERROR_MESSAGE",
        ])
        .arg("-gencode")
        .arg(format!("arch=compute_{arch},code=sm_{arch}"))
        .arg(format!("-I{}", sppark.display()))
        .arg(format!("-I{blst_src}"))
        .args(["-c", "cuda/mori_gpu.cu", "-o"])
        .arg(&obj)
        .status()
        .expect("nvcc");
    assert!(st.success(), "nvcc failed");
    let st = Command::new("ar")
        .arg("crs")
        .arg(out.join("libmorigpu.a"))
        .arg(&obj)
        .status()
        .expect("ar");
    assert!(st.success());
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-search=native={cuda}/lib64");
    println!("cargo:rustc-link-search=native={cuda}/targets/sbsa-linux/lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{cuda}/lib64");
    println!("cargo:rustc-link-lib=static=morigpu");
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
