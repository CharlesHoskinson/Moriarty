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

## 2. Gate plumbing and build (2026-11-12)

- Drop-in parity: `mori_msm::msm_g1` declines (returns `None`) below `2^min_k` with the same variables and default as `mori_gpu` (`MORIARTY_GPU_MIN_K`, `MORI_GPU_MIN_K`, 14), when `n > bases.len()`, and without a device. The hook counts each Rust MSM in `mori_gpu::stats()` through the new `mori_gpu::record_msm`, so the harness receipts (`msm_calls`, `msm_s`, `fallbacks`, `gpu_path_valid`) read the same counters for both kernels.
- Hook (`tools/gpu-prover/src/lib.rs`): phase token `msmrs` (bit 16) runs the MSM phase on `mori-msm`; `MORI_MSM=rust` routes the `msm` phase there too. One process can prove `cpu;msm;msmrs` with one key, so byte identity is checked within one run.
- Recording: `MORI_MSM_RECORD=<dir>` writes the scalars of every MSM call (`call-NNNN-n<N>-b<len>.bin`, blst_fr words) and each base table once (`bases-<len>.bin`, affine). Witness data: seeded test runs only, stored under `~/.cache/msm-rust/`, never in the repository.
- Cubin: `build.rs` runs `ptxas -O3 -arch=sm_121` (override `MORI_MSM_ARCH`) and the module loads the cubin; PTX stays as the fallback. MEASURED: the spike test with an empty driver cache now takes 1.1 s for the whole process (was 126 s).
- Tests compare with sppark too (`--features sppark`): raw, masked and `mori_gpu::msm_g1` all equal BLST for every vector at or above sppark's `min_k`.

32-bit limbs with PTX carry chains (enhancement 3 candidate), MEASURED under load (load average about 30, outside the lock, development only): `mad.lo.cc` / `madc.hi.cc` chains compile on sm_121 to IMAD plus separate IADD carry instructions (per limb row 55 IMAD and 80 IADD), and run at 0.40 to 0.42 G Fp mul/s, the same as the 6 x 64-bit version with IMAD.WIDE. No gain on this GPU; the 64-bit version stays. The device issues about 0.5 T integer instructions per second at 754 MHz in this state, so fewer instructions per point addition (batch-affine) and fewer additions are the remaining arithmetic levers.

## 3. Correctness gates (MEASURED, 2026-11-12, under the heavy lock)

Synthetic vectors, 2^10 to 2^21: `tests/msm.rs` with `MORI_MSM_TEST_MAX_LG=21`, `MORI_GPU_MIN_K=0`, `--features sppark` (binary `~/.cache/msm-rust/bin/msm-test`, committed kernel 79dbce1dd, affine passes off). `drop_in_gate`, `small_and_edge` and `sizes_random_and_skewed` pass: mori-msm raw and masked equal BLST and sppark for random and 90 percent skewed scalars at every size. 57 s, load average 12.6 at start.

k = 20 wrap proof on the stdlib run decider (the gpu-wrap `wrap_prove` path, bundle `~/.cache/k21-measure/out-k21-20261111T193356Z`, `GW_DET=1`, seed 20261111), one process, CPU reference first. Run `~/.cache/msm-rust/runs/k20-20261112T001954/`.

| config | proof sha256 (prefix) | byte identical | library verify | ledger 9 verify | negative control rejected | gpu_path_valid | MSM calls | fallbacks | MSM s | prove s | load avg (1 min) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cpu | 1cb0edf55b343f00 | reference | true | true | true | true | 0 | 0 | 0 | 316.4 | 24.7 |
| msmrs | 1cb0edf55b343f00 | true | true | true | true | true | 45 | 0 | 50.2 | 232.1 | 21.0 |
| msm (sppark, masked) | 1cb0edf55b343f00 | true | true | true | true | true | 45 | 0 | 27.2 | 253.3 | 26.3 |
| msmrs, ntt, coset, quotient | 1cb0edf55b343f00 | true | true | true | true | true | 45 | 0 | 46.6 | 132.5 | 12.6 |

The correctness gate passes. All 45 MSM calls of the proof go to the Rust kernel with no fallback, and the proof bytes equal the CPU proof, so the kernel is exact on the real witness columns too. The CPU config records those columns to `~/.cache/msm-rust/columns-k20/` (237 calls, 7.3 GB, test seed only). Timing under load (SM clock 754 MHz, throttle 0x400): in the proof the Rust MSM (masked, committed defaults k1 = 32, seg = 64) takes 50.2 s against 27.2 s for sppark masked. The Rust kernel is 1.8 times slower than sppark in the proof.

## Owner question: the skew mask now costs time

The mask `MSM(s + r) - MSM(r)` was added because sppark gives one thread per bucket. The Rust kernel never does, and on skewed columns it is faster without the mask: development timing at 2^20, 90 percent one value, about 0.28 s raw against about 1.8 s masked (outside the lock, host load about 28; the gate benchmark below repeats it under the lock on recorded columns). The mask is kept, as instructed. Question for the owner: may the Rust path drop the mask (`MORI_GPU_MSM_MASK=0` for `msmrs`) once the recorded-column benchmark confirms the raw kernel is skew robust? The mask is witness independent and changes no proof bytes, so the choice affects only speed.
