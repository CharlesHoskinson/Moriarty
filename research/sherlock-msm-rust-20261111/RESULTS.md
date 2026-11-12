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

## 4. Speed passes at 2^20 (MEASURED under the heavy lock, host load average 21 to 43, SM clock 754 MHz, throttle 0x400)

Every cell is 3 reps and equals BLST. The values are min / median in seconds. All of these are timings under load, not quiet-host numbers.

- Batch-affine passes (`MORI_MSM_AFFINE`, 0 to 6 passes, k1 32 and 128): no gain. Random raw is 0.97 to 1.00 s with 0 passes and 0.97 to 1.15 s with 1 to 6 passes. Skew raw gets worse, from 0.20 to 0.23 s up to 0.25 to 0.34 s. The grid-wide inversion and the extra passes cost more than the saved multiplications. The default stays 0. Log: `~/.cache/msm-rust/sweep-aff.log`.
- Gather prefetch in round 1 (the load of point j + 1 is issued before the addition of point j): within noise. Random raw is 0.92 to 0.94 / 0.93 to 0.95 against 0.89 to 0.97 / 0.96 to 0.98 without it. It is kept because it costs nothing. Log: `~/.cache/msm-rust/ab-pf.log`.
- One thread per key run in the final bucket pass (`run_flags`, scan, `run_compact`, `run_sum`, in place of the grid-stride `run_scatter`, which stays available with `MORI_MSM_RUN_SCATTER=1`): the stage median goes from 102 ms to 89 ms. The total is within noise (random raw 0.90 to 0.98 s). Tests to 2^18 with the sppark comparison pass. Log: `~/.cache/msm-rust/ab-rs.log`.
- New defaults from the k1/seg sweep: k1 = 64, seg = 16.

Stage profile, random 2^20, median of the new configuration: round 1 703 ms, final bucket pass 89 ms, weighted sums 91 ms, digits plus scatter 37 ms, everything else under 5 ms. Round 1 does 16.7 M mixed additions in 0.70 s, which is 24 M additions per second or about 0.24 G Fp multiplications per second. That is about 60 percent of the 0.40 to 0.43 G/s Fp multiplication peak measured on this device. Round 1 is the whole remaining gap to sppark (0.45 to 0.50 s total under load). The next levers are in round 1: occupancy (registers per thread and the 64-thread blocks), a cheaper reduction in the multiplication, and fewer additions per point (a larger c for 2^20 and above).

The benchmark gate ran later (section 6).

## 5. Round 1 fixes (commit f84a06f99)

- 32-bit even/odd Fp multiplication (ZKProphet IV-B2, the sppark `mont_t` scheme): x takes the products of the even limbs of a and p, and y takes the odd limbs one limb up. The lo and hi halves of each product land in adjacent limbs of one carry chain, and each row folds y[0] into x[0] with that carry entering the odd chain. ptxas now fuses each lo/hi pair into one `IMAD.WIDE.U32.X` (240 of them in the multiplication test kernel). The section 2 attempt put the lo and hi chains apart, and ptxas emitted 55 IMAD plus 80 IADD per row. The algorithm is checked first in a Python model (3000 random pairs plus edge cases), then on the device against BLST. Kernel feature `fp64` keeps the 6 x 64-bit CIOS.
- Point loads and stores: a plain `*p` of an `Affine` or `Xyzz` is an LLVM memcpy, which nvptx expands into a byte loop through local memory (the base gather in round 1 was 96 single-byte loads and stores). Word-wise helpers replace them.
- Out-of-line EC calls: `dbl` and `mdbl` were calls whose results merge with the hot path, so the accumulator lived in local memory (384 to 576 byte stack frames). With `add`, `add_affine`, `dbl` and `mdbl` inlined, the round kernels, `run_sum` and `sum_rows` have no stack frame and no local memory (ptxas -v), at 228 to 236 registers.
- Bucket zeroing is 1.2 to 1.4 ms of the profile, so it stays.

## 6. Benchmark gate (MEASURED 2026-11-12 03:33 to 03:52 under the heavy lock, after M7 cleared the lock)

Kernel f84a06f99 (`~/.cache/msm-rust/bin/bench-fp32`), defaults k1 = 64, seg = 16, affine passes off. Each cell is 3 reps, min / median in seconds. SM clock 754 MHz in every cell (throttle mask 0x400, max 3003 MHz). Host load average 20 to 36, so all of these are timings under load. Every one of the 20 cells equals BLST (all implementations). The recorded rows are 8 calls of the k = 20 proof (`~/.cache/msm-rust/columns-k20`, test seed). Log: `~/.cache/msm-rust/bench-gate.log`.

| input | log n | Rust raw | Rust masked | sppark raw | sppark masked | BLST | load avg |
|---|---|---|---|---|---|---|---|
| random | 16 | 0.050 / 0.052 | 0.056 / 0.056 | 0.062 / 0.064 | 0.072 / 0.090 | 0.387 / 0.429 | 23.26 |
| random | 17 | 0.071 / 0.071 | 0.092 / 0.132 | 0.107 / 0.114 | 0.121 / 0.126 | 0.818 / 0.996 | 22.75 |
| random | 18 | 0.321 / 0.327 | 0.317 / 0.357 | 0.492 / 0.542 | 0.538 / 0.540 | 1.223 / 1.244 | 22.46 |
| random | 19 | 0.246 / 0.270 | 0.222 / 0.255 | 0.307 / 0.308 | 0.547 / 0.572 | 2.449 / 2.633 | 22.77 |
| random | 20 | 0.894 / 0.902 | 0.905 / 0.914 | 0.731 / 0.734 | 0.486 / 0.706 | 3.906 / 4.262 | 25.27 |
| random | 21 | 0.736 / 0.876 | 0.760 / 1.385 | 2.074 / 2.076 | 2.137 / 2.188 | 6.679 / 7.801 | 30.91 |
| skew | 16 | 0.084 / 0.088 | 0.125 / 0.136 | 0.326 / 0.336 | 0.158 / 0.171 | 0.146 / 0.152 | 26.11 |
| skew | 17 | 0.112 / 0.120 | 0.185 / 0.187 | 0.355 / 0.376 | 0.156 / 0.212 | 0.214 / 0.268 | 27.23 |
| skew | 18 | 0.038 / 0.039 | 0.127 / 0.132 | 0.752 / 0.793 | 0.264 / 0.329 | 0.568 / 0.613 | 30.25 |
| skew | 19 | 0.045 / 0.054 | 0.399 / 0.482 | 0.810 / 0.891 | 0.345 / 0.592 | 1.217 / 1.275 | 33.87 |
| skew | 20 | 0.164 / 0.193 | 0.912 / 0.913 | 1.853 / 2.157 | 0.574 / 0.712 | 1.267 / 1.335 | 34.99 |
| skew | 21 | 0.228 / 0.229 | 1.673 / 1.685 | 4.521 / 6.643 | 1.022 / 1.364 | 2.471 / 2.551 | 36.51 |
| call-0000 | 20 | 0.891 / 0.915 | 0.922 / 0.927 | 0.968 / 1.261 | 0.538 / 0.711 | 3.092 / 3.760 | 34.06 |
| call-0010 | 20 | 0.314 / 0.314 | 0.329 / 0.329 | 0.445 / 0.459 | 0.529 / 0.554 | 3.483 / 4.828 | 34.71 |
| call-0040 | 20 | 0.018 / 0.028 | 0.333 / 0.443 | 0.076 / 0.081 | 0.518 / 0.599 | 0.338 / 0.366 | 33.69 |
| call-0080 | 20 | 0.314 / 0.381 | 0.328 / 0.328 | 0.464 / 0.466 | 0.470 / 0.619 | 3.078 / 3.175 | 33.55 |
| call-0120 | 20 | 0.336 / 0.504 | 0.336 / 0.402 | 0.840 / 0.881 | 0.458 / 0.490 | 2.651 / 2.990 | 33.14 |
| call-0160 | 20 | 0.290 / 0.290 | 0.332 / 0.338 | 99.365 / 151.118 | 0.502 / 0.726 | 2.864 / 3.200 | 31.95 |
| call-0200 | 20 | 0.053 / 0.053 | 0.418 / 0.449 | 0.589 / 0.638 | 1.270 / 1.340 | 0.649 / 0.699 | 20.77 |
| call-0230 | 20 | 0.352 / 0.461 | 0.342 / 0.402 | 0.521 / 0.683 | 0.709 / 0.751 | 3.244 / 4.095 | 23.82 |


The run has GPU contention that the lock does not cover (browser GPU processes are present, and GPU utilization is 48 percent while idle). The non-monotonic cells show it: random 2^20 takes 0.89 s against 0.74 s at 2^21, and one sppark raw cell takes 99 s. The same-binary A/B below, with identical non-EC stage times as an in-run control, is the cleaner comparison.

A/B at 2^20 random raw, run back to back (BLST skipped, MORI_MSM_PROFILE=1), median stage ms:

| kernel | total min / median s | digits | scatter | round 1 | final bucket pass | weighted sums |
|---|---|---|---|---|---|---|
| f84a06f99, 32-bit Fp | 0.314 / 0.349 | 8.0 | 29.2 | 218 | 14 | 41 |
| f84a06f99 with the `fp64` cubin | 0.516 / 0.601 | 8.0 | 29.2 | 454 | 19 | 69 |
| 3e610fe44 (committed before) | 2.382 / 2.413 | 24.3 | 88.3 | 1746 | 241 | 253 |

The fp32 and fp64 rows have equal digits and scatter times, so they saw the same GPU conditions. The 32-bit multiplication halves round 1 (454 to 218 ms). The 3e610fe44 row is slowed about 3 times in every stage, including the non-EC ones (digits 24 against 8 ms), so it ran during contention. Against its uncontended section 4 profile (round 1 703 ms, total 0.90 to 0.98 s), the load and inline fixes plus the multiplication take round 1 from 703 to 218 ms and the total from about 0.93 to 0.31 to 0.35 s.

Reading of the gate: the Rust raw MSM is at or below sppark masked (the current default) in 18 of 20 cells. The two exceptions are random 2^20 (0.89 against 0.49) and recorded call-0000 (0.89 against 0.54), and both are in the contended window; the A/B gives 0.31 to 0.35 s for random 2^20. The Rust masked MSM is slower than sppark masked in 6 cells: random 2^20, call-0000, and skew at 2^17, 2^19, 2^20 and 2^21 (2^21: 1.67 against 1.02 s). The mask turns a skewed column into a random one, and raw Rust handles skew directly: on skewed scalars, raw Rust is 3 to 20 times faster than any masked option (2^21: 0.23 s against 1.02 s for sppark masked). This strengthens the owner question below.

## Owner question: the skew mask now costs time

The mask `MSM(s + r) - MSM(r)` was added because sppark gives one thread per bucket. The Rust kernel never does, and on skewed columns it is faster without the mask: development timing at 2^20, 90 percent one value, about 0.28 s raw against about 1.8 s masked (outside the lock, host load about 28; the gate benchmark below repeats it under the lock on recorded columns). The mask is kept, as instructed. Question for the owner: may the Rust path drop the mask (`MORI_GPU_MSM_MASK=0` for `msmrs`) once the recorded-column benchmark confirms the raw kernel is skew robust? The mask is witness independent and changes no proof bytes, so the choice affects only speed.
