# Rust MSM (K2, BLS12-381 G1): results

## 0. Feasibility spike (MEASURED, 2026-11-11 22:40)

Question: can a Rust nvptx kernel with 384-bit Montgomery multiplication build, load and run on this host (RTX Spark N1X, sm_121, driver 615.78.08)?

Answer: yes.

- Device: `crates/mori-msm/kernel`, `#![no_std]`, `extern "ptx-kernel"`, `core::arch::nvptx` intrinsics. Built with the pinned toolchain Rust 1.99.0 (LLVM 23.1.1) for target `nvptx64-nvidia-cuda` (rustup target plus the `llvm-bitcode-linker` and `llvm-tools` components). `abi_ptx` and `stdarch_nvptx` are unstable features; the build sets `RUSTC_BOOTSTRAP=1` for the kernel crate only. The root workspace stays on stable.
- PTX: `.version 7.0`, `.target sm_70`. The driver JIT compiles it for sm_121. `ptxas -arch=sm_121` (CUDA 13.4.92) accepts it: 46 registers, 48 byte stack frame, no spills.
- Host: `crates/mori-msm`, cudarc 0.19.10 (features std, driver, nvrtc, dynamic-loading, cuda-version-from-build-system), PTX embedded with `include_str!` and loaded with `CudaContext::load_module`.
- Test `tests/spike.rs`: 2^16 random Fp pairs, 3 chained Montgomery multiplications (CIOS, 6 x u64 limbs, u128 products, branch-free final subtraction) on the GPU, compared limb by limb with `blst_fp_mul`: all equal, test time 0.75 s.
