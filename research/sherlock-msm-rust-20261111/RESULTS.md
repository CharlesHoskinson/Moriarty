# Rust MSM (K2, BLS12-381 G1): results

## 0. Feasibility spike (MEASURED, 2026-11-11 22:40)

Question: can a Rust nvptx kernel with 384-bit Montgomery multiplication build, load and run on this host (RTX Spark N1X, sm_121, driver 615.78.08)?

Answer: yes.

- Device: `crates/mori-msm/kernel`, `#![no_std]`, `extern "ptx-kernel"`, `core::arch::nvptx` intrinsics. Built with the pinned toolchain Rust 1.99.0 (LLVM 23.1.1) for target `nvptx64-nvidia-cuda` (rustup target plus the `llvm-bitcode-linker` and `llvm-tools` components). `abi_ptx` and `stdarch_nvptx` are unstable features; the build sets `RUSTC_BOOTSTRAP=1` for the kernel crate only. The root workspace stays on stable.
- PTX: `.version 7.0`, `.target sm_70`. The driver JIT compiles it for sm_121. `ptxas -arch=sm_121` (CUDA 13.4.92) accepts it: 46 registers, 48 byte stack frame, no spills.
- Host: `crates/mori-msm`, cudarc 0.19.10 (features std, driver, nvrtc, dynamic-loading, cuda-version-from-build-system), PTX embedded with `include_str!` and loaded with `CudaContext::load_module`.
- Test `tests/spike.rs`: 2^16 random Fp pairs, 3 chained Montgomery multiplications (CIOS, 6 x u64 limbs, u128 products, branch-free final subtraction) on the GPU, compared limb by limb with `blst_fp_mul`: all equal, test time 0.75 s.

## 1. Skew-robust MSM with XYZZ accumulation (step 3)

Code: `crates/mori-msm` (host `src/msm.rs`, `src/mask.rs`; device `kernel/src/{fp,fr,ec,lib}.rs`).

- Device pipeline: canonical scalars and signed c-bit digits (`msm_digits`, histogram with atomics), exclusive scan, counting sort by key `(window, |digit| - 1)`, then load-balanced segmented reduction rounds: each thread takes `k` consecutive sorted entries whatever bucket they belong to (k1 = 32 in round 1 with affine bases, k2 = 16 later) and emits one partial per key run; the chunk grid shifts by k / 2 on alternate rounds. The host knows the largest bucket from the scan (`max`), so it runs rounds until no run can exceed 8 entries, then one run-aligned pass writes the dense bucket array. A hot bucket costs about log_k of its size in rounds; no thread ever owns a whole bucket.
- Window sums: running sums in segments of 16 buckets (`wsum_level`, no scalar multiplication), recursion on the segment totals for the `len * sum s R_s` term, plain tree sums (`sum_rows`), and the last few points per window plus the window combination on the host with BLST. Serial work on one GPU thread is avoided: one Fp multiplication has about 9 us latency on one thread here (see below).
- Field: 6 x 64-bit limbs, CIOS with the row loop unrolled by a macro (no local memory in `fp_mul`), branch-free final subtraction, add, sub and negation (enhancement 4 from the start).
- Device buffers are kept across calls (allocation of hundreds of MB costs tens of ms per call on this unified-memory device).
- Mask: `MSM(s + r) - MSM(r)` exactly as in `mori-gpu` (same splitmix64 vector, MSM(r) cached per table and length); `MORI_GPU_MSM_MASK=0` runs the raw MSM.

Correctness, MEASURED (`cargo test -p mori-msm --features cuda --release`): field ops (add, sub, inverse, mul) equal BLST on 4,096 random pairs; point ops (mixed add, add, double, 5P, with P = Q, P = -Q and infinity cases) equal BLST; the MSM raw and masked equal BLST `multi_exp` for n = 1, 2, 3, 17, 255, 1000, all zero, all 5, all p - 1, one nonzero, and random and skewed (90 percent one value, rest below 2^16) vectors at 2^10 to 2^16. Larger sizes and recorded witness columns follow in the gate runs.

Device calibration, MEASURED 2026-11-11 23:2x (host load average about 28, not under the heavy lock, so not a benchmark): the GPU runs at 754 MHz SM clock (max 3,003 MHz) under this load, throttle reason mask 0x400. 48 SMs. An independent u32 multiply-add chain kernel reaches 560 G IMAD/s. `fp_mul` (64-bit limbs) compiles to about 1,100 SASS instructions (562 IADD, 401 IMAD) and runs at 0.40 to 0.43 G Fp mul/s. So the device integer pipe, not memory, bounds the MSM on this host today, and fewer instructions per multiplication and fewer multiplications per point addition are the levers (enhancements 2 and 3).

Development timing (same conditions, NOT a benchmark, one or two runs): 2^20 random scalars about 1.0 s raw (round 1 about 680 ms of 16.8 M mixed additions, final bucket pass about 150 ms, window sums about 120 ms); 2^20 skewed scalars about 0.28 s raw with 5 rounds for a bucket of 942,881 entries. The raw kernel is faster on the skewed column than on random scalars, which shows the skew robustness; the mask turns the skewed column into a random one and costs the random-scalar time.

PTX JIT, MEASURED: loading the module with an empty driver cache (`CUDA_CACHE_DISABLE=1`) takes 126 s (the spike test binary, whole process); with the cache warm it takes 1.4 s. The fix is to compile the PTX to an sm_121 cubin with `ptxas` at build time and load the cubin (the source stays Rust); until then the first process on a host pays the JIT once.
