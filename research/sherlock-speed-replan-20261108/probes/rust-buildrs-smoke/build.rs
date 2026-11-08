// Minimal pattern for crates/mori-gpu: nvcc compiles .cu to a static lib in OUT_DIR, Rust links it plus cudart.
use std::{env, path::PathBuf, process::Command};
fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cuda = env::var("CUDA_PATH").unwrap_or_else(|_| "/opt/cuda".into());
    let arch = env::var("MORI_GPU_ARCH").unwrap_or_else(|_| "sm_121".into());
    let obj = out.join("kernels.o");
    let st = Command::new(format!("{cuda}/bin/nvcc"))
        .args(["-O3", "-std=c++17", "-Xcompiler", "-fPIC", &format!("-arch={arch}"), "-c", "cuda/kernels.cu", "-o"])
        .arg(&obj).status().expect("nvcc");
    assert!(st.success(), "nvcc failed");
    let st = Command::new("ar").args(["crs"]).arg(out.join("libmorikernels.a")).arg(&obj).status().unwrap();
    assert!(st.success());
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-search=native={cuda}/lib64");
    println!("cargo:rustc-link-search=native={cuda}/lib");
    println!("cargo:rustc-link-lib=static=morikernels");
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rerun-if-changed=cuda/kernels.cu");
}
