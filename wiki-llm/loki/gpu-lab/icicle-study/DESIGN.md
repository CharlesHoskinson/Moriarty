# Loki GPU path: clean-room design informed by open-icicle (reference only)

Date: 2026-10-24. Author: icicle-study agent. Status: design memo, nothing implemented, no GPU job run.

Inputs studied:

| What | Where | Revision |
|---|---|---|
| open-icicle (reference only, see licence note) | `/home/charles/research/open-icicle-src/repo` | commit `a1f8a74` ("Update README.md"), workspace version 4.0.0 |
| sppark (Apache-2.0, already vendored by Loki) | `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sppark-0.1.16/sppark/` | 0.1.16 (Loki `Cargo.lock`) |
| Loki | `/home/charles/repos/loki-proofs` | main `877114d` |
| W10 GPU work and A3 profile | `/home/charles/repos/loki-wt/w10-gpu-quotient` (`LOKI-A3-PROFILE.md`, `target/a3-profile/gpu-after-1792898627/phases.jsonl`) | merged source `000b3ed`, launcher `053577f` |
| Lean contracts | `/home/charles/repos/loki-lean` (`SPEC.md`, `LokiLean/*.lean`) | `a9f6247` |
| WSL2 / sm_120 notes | `.../gpu-lab/helpers/lab-haiku-2/H7.md`, `H27.md` | as captured |

---

## 0. Licence note (read first)

**Do not vendor, copy or translate any file under `icicle/backend/cuda/`.** This memo uses it as reference only. Everything
below is described in my own words or pseudocode. No icicle code was copied into this file or anywhere else.

Exact wording and locations, at commit `a1f8a74`:

- `LICENSE` (repository root, the only licence file in the tree; `find -iname 'LICENSE*' -o -iname 'NOTICE*' -o -iname 'COPYING*'` returns only this one):
  "MIT License / Copyright (c) 2026 Ingonyama / Permission is hereby granted, free of charge, ..."
- `README.md:319-326`, section "## License":
  - line 321: "ICICLE frontend is distributed under the terms of the MIT License."
  - line 324 (a `> [!NOTE]` block): "ICICLE backends, excluding the CPU backend, are distributed under a special license and are not covered by the MIT license."
  - line 326: "See [LICENSE-MIT][LMIT] for details." (the link points to `./LICENSE`).
- `icicle/backend/cuda/` has no LICENSE, NOTICE or COPYING file. Its `README.md` is two lines ("# icicle-cuda-backend / CUDA backend for icicle V3").
  No `.cu`, `.cuh` or `.h` file under it has an SPDX, Copyright or License header (grep, zero hits).
- `wrappers/rust/icicle-runtime/Cargo.toml:17-20`, a comment on the `cuda_backend` / `pull_cuda_backend` features:
  "Note: Backend repositories are private and not meant for user workflows. Users should install backends directly instead."
- The "special license" text itself is not in the repository. Its terms are unknown.

So the root MIT grant visibly does not cover the CUDA backend, and the backend's own terms are not published here.
Anything in the frontend (`icicle/include/icicle/...`, `wrappers/rust/...`) is MIT. I read three frontend files
(`include/icicle/msm.h`, `include/icicle/backend/msm_config.h`, `include/icicle/polynomials/default_backend/*`) and the Rust runtime
wrapper. Even so, this design takes only ideas from them, not code.

If the owner ever wants to vendor backend code, ask Ingonyama (support@ingonyama.com, the contact in `README.md:317`) three things:
(1) the text of the "special license"; (2) whether it covers `icicle/backend/cuda/` as published in the public `open-icicle` repository
at `a1f8a74`; (3) whether redistribution inside an Apache-2.0/MIT crate (Loki is dual Apache-2.0/MIT) is permitted.

Clean-room discipline for implementers: whoever writes the kernels in Section 3 works from this memo and from sppark (Apache-2.0),
**not** from the icicle backend sources. Reviewers may consult icicle to check an idea, but must not paste from it.

---

## 1. Baseline: where A3 time goes (gpu-after, `cuda-argument-commit`)

Source: `phases.jsonl` (one A3 k20 Sherlock parent proof, RTX 5090, CUDA 13.4, Rayon 4, BLST 1; single sample, no variance).
create_proof = **85.910 s**. Prove-plus-verify for the test = 175.256 s, so **~89.3 s lies outside create_proof** (cost modelling,
PK setup, child checks, parent verification). No package below touches that residual. It caps any create_proof speedup at about
2x on the test wall clock. For recursion latency, the residual must be measured and attacked separately (the W12 PK disk cache is
the obvious first lever).

| Phase | s | GPU calls (JSONL) | Attribution (derived below) |
|---|---:|---|---|
| quotient_evaluation | 47.171 | 22 NTT @2^22 | ~4.8 s NTT adapter + **~42.4 s CPU numerator** |
| lookup_prepare | 12.467 | 10 MSM, 10 NTT @2^20 | ~0.7 s MSM + ~0.5 s NTT + **~11 s CPU** (compress + permute) |
| witness_synthesis.phase_0 | 6.628 | none | CPU |
| kzg_multiopen | 5.928 | 2 MSM (one is length 2^20 − 1) | CPU work + 1 SRS cache miss (see problem 4) |
| compute_nu_poly | 3.940 | 17 NTT @2^22 | ~3.7 s NTT adapter |
| advice_commitments | 3.369 | 15 MSM | expected ~1.0 s at the random-scalar rate; the ~2.4 s excess is skew and/or a Lagrange SRS cache miss |
| permutation_products | 1.644 | 6 MSM, 6 NTT | |
| trash | 1.561 | 1 MSM, 1 NTT | |
| other | 0.997 | 2 MSM, 17 NTT @2^20 | |
| evaluations | 1.051 | none | CPU |
| lookup_products | 0.672 | 5 MSM, 5 NTT | |
| quotient_commitment | 0.265 | 4 MSM | **66 ms per random-scalar 2^20 MSM on a cache hit** (the reference MSM rate) |
| quotient_construct | 0.217 | 1 NTT @2^22 | ~0.23 s NTT adapter |

Derivations:

- **NTT adapter cost.** W5's bench (`loki-proofs/LOKI-GPU-NTT-BENCH.csv`) gives a 2^22 `coeff_to_extended` GPU median of 218 ms and a
  2^20 `lagrange_to_coeff` median of 46 ms. Hence 40 × 0.218 + 39 × 0.046 ≈ **10.5 s per A3 proof (12 %)**.
  Cross-checks: compute_nu_poly is 17 × 0.218 = 3.71 s, against 3.94 s measured. quotient_construct is 1 × 0.227 s, against 0.217 s measured.
  Device kernel time for a 2^22 NTT is not measured. It is expected to be a few milliseconds. The rest is host work in the adapter
  (`cuda-ntt/src/lib.rs:38-57`: two full host copies through `Vec<blst_fr>`, a per-call `dev_ptr_t` allocation, a pageable
  H2D and D2H inside sppark `NTT::Base`, `ntt/ntt.cuh:216-233`), plus the host-side ζ distribution and zero padding in `domain.rs`.
- **Transfer volume.** 40 × 128 MiB + 39 × 32 MiB = 6.2 GiB each way, matching the brief.
- **sppark skew model.** This fits W10's synthetic table. sppark sets `wbits = min(lg2(1.5 n) − 8, 18)`, floored at 10
  (`msm/pippenger.cuh:357-363`). It splits `n` into `batch = 2^(lg − wbits) >> 6` host-serial batches (`:454-459`). Then one thread
  owns each (window, bucket) and adds its points serially (`accumulate`, `:178-221`; the loop at `:201-207`). If every scalar is equal,
  each window has one bucket holding all `stride` points of the batch, so the critical path is about `n` serial mixed additions.
  - k18 has stride 65,536 over 4 batches, so 262,144 serial adds. At the measured 2,371 ms that is about 9 µs per add.
  - The 0..15 pattern has 4,096 points per bucket per batch, so about 16,384 serial adds, predicting about 150 ms. Measured: 171 ms.
  - At k20 this predicts **~9 s for a single full-width repeated-scalar MSM**, which accounts for gpu-before's 51 s lookup_prepare.
  - The all-ones control is fast because sppark routes window 0, digit 1 to a separate grid-wide `batch_addition` kernel
    (`batch_addition.cuh:137-170`, launched at `pippenger.cuh:508-512`, excluded from `accumulate` at `:193`).

---

## (a) Technique-by-technique summary (open-icicle, reference only)

Paths are relative to `icicle/backend/cuda/` unless they start with `include/icicle` (frontend, MIT).

### a1. MSM pipeline (`src/msm/cuda_msm.cuh`)

1. **Parameters** (`bucket_method_msm`, :958-1001).
   - Unsigned digits of width `c`; windows = ceil(bits / c).
   - `average_bucket_size = (n / 2^c) * precompute_factor` (:995).
   - Large-bucket threshold = `large_bucket_factor * average` (:996).
   - Default `c` = clamp(ceil(log2 n) − 4, 1, 20) (`get_optimal_c`, :45-48; the comment calls 20 "a temporary fix for the large buckets problem"). At n = 2^20 this gives c = 16 and 16 windows.
   - Default large-bucket factor is 10 (`include/icicle/backend/msm_config.h:14`). Chunk count defaults to 0, meaning automatic (:16).
   - Config extension readers are at `include/msm/cuda_msm_config.cuh:9-22`. The user-facing `MSMConfig` fields (`include/icicle/msm.h:21-53`) are stream, precompute_factor, c, bitsize, batch_size, are_points_shared_in_batch, *_on_device, *_montgomery_form and is_async.
2. **Digit split as key-value pairs** (`split_scalars_kernel`, :163-203).
   - One thread per scalar emits, for every window, a 32-bit key `(msm_index | window | digit)`.
   - The value is the point index, which is `(scalar_index * precompute_factor + precomputed_window) mod points_size` when precomputation is on.
   - A zero digit becomes key 0, which is dropped later.
3. **Sort and histogram with library primitives** (`split_and_sort_scalars`, :400-485).
   - A cub radix sort of the (key, point index) pairs (:443-452).
   - A cub run-length encode to obtain the distinct buckets and their sizes (:461-467).
   - A cub exclusive scan for bucket offsets (:475-481).
   - Every cub call is done twice (size query, then run) with `cudaMallocAsync`/`cudaFreeAsync` scratch on the stream.
4. **Sort buckets by size, descending** (`allocate_and_sort_buckets_by_size`, :560-636).
   - Two more descending radix sorts keyed by size, carrying offsets (:606-614) and bucket ids (:622-630).
   - The zero bucket is removed by reading the smallest key back to the host (:590-594).
   - The count of non-empty buckets is also read back to the host (:597-600).
5. **Large-bucket detection** (`compute_nof_large_buckets` and `find_cutoff_kernel`, :205-220 and :638-664).
   - Because sizes are sorted, the "large" set is a prefix. A small kernel finds the first index whose size drops to the threshold or below.
   - The thread count is a hard-coded `TOTAL_THREADS = 129000 // TODO: device dependent` (:651). One more host read-back follows.
6. **Large-bucket splitting and tree reduction** (`large_bucket_accumulation`, :666-765). This is the technique that addresses Loki problem 1.
   - It runs on a second stream created per call (:686-687).
   - An inclusive scan over the large-bucket sizes (:695-701) and two scalars read back to the host: the total points in large buckets and the largest bucket (:703-709).
   - `initialize_large_bucket_indices` (:56-75) gives every large bucket ceil(size / average) segment slots. Each slot packs the bucket id with a segment number.
   - `accumulate_large_buckets_kernel` (:257-289): each thread sums at most `average` points of one bucket's segment into a partial.
   - A loop of ceil(log2(largest)) passes of `sum_reduction_variable_size_kernel` (:77-94), with `normalize_kernel` resizing (:50-54, :740-750), halves every bucket's partial array in parallel.
   - `distribute_large_buckets_kernel` (:291-310) writes the reduced partial into the main bucket array, or adds it there when chunking.
   - An event joins the side stream back to the main stream (:762, :813-816).
   - In short: the time for one bucket of size S falls from S serial additions to about average + log2(S / average) steps.
7. **Small-bucket accumulation** (`accumulate_buckets_kernel`, :222-255). One thread per remaining bucket, projective += affine, skipping the identity. The buckets are size-sorted, so warps get similar work.
8. **Bucket reduction** (`bucket_reduction`, :821-956).
   - Either a per-window serial running-sum ("big triangle", :312-326; selected by an extension flag or c = 1),
   - or an **iterative window-halving reduction** (:846-943). Each round splits every c-bit window into two windows of ceil(c/2) bits, until windows are 1 bit wide. Two kernel families run on two streams (:847-850, :900) and are joined per round (:906-908). A `last_pass_kernel` (:342-366) then gathers the 1-bit windows.
   - Finally `final_accumulation_kernel` (:368-387): one thread per MSM does double-and-add over at most `bitsize` 1-bit window sums. That loop is serial in the bit length.
9. **Montgomery handling.**
   - Scalars and points may arrive in Montgomery form. They are converted to canonical form on the device by a separate kernel and a separate allocation (`from_montgomery_on_device`, :389-398; `upload_scalars` :487-518; `upload_points` :520-558; the conversion kernel is `include/cuda_mont.cuh:12-17`).
   - Digit extraction then works on canonical limbs.
   - sppark instead folds the conversion into its digit kernel (`breakdown(..., mont)`, `pippenger.cuh:73-119`) and keeps points in Montgomery form throughout. That is cheaper and is Loki's current ABI.
10. **Precomputed bases** (`precompute_points_kernel` :29-43, `precompute_msm_points_chunk` :1445-1474, `chunked_precompute` :1476-1528).
    - For a precompute factor p, each base P is stored with its shifts 2^(c·W/p·j)·P for j < p. Here W is the window count and the shift is computed at :1464-1465.
    - The MSM then needs only W/p windows and p times as many points per window. That means fewer bucket reductions, at p times the base memory.
    - `c` is capped at 22 when precomputing (`MAX_C_FOR_PRECOMPUTATION`, :27, :1407-1409, :1550-1552).
11. **Memory model and chunking** (:1130-1237).
    - Estimates: scalars + 7× (4-byte index) × n × windows for sort scratch (:1147); 4× buckets for the reduction (:1151); a budget of 0.7 × free memory (:1161-1162).
    - When data is on the host it uses 4 chunks by default, to overlap transfer and compute (:1234-1235). It uses 1 when everything is on the device.
    - `multi_chunked_msm` (:1239-1394) double-buffers the next chunk's scalars and points on an alternate stream while the current chunk accumulates into the same buckets (`init_buckets` and `return_buckets`, :1313-1314). Batched MSMs with shared points upload and convert the points once (:1249-1253, :1324-1325).
12. **Async teardown.** A detached `std::thread` synchronises and destroys the per-call side streams and events (:1103-1124, :1376-1392).

Contrast with sppark (Loki today):

- sppark uses signed (Booth) digits with half as many buckets (`pippenger.cuh:64-71`, :100-118) and xyzz bucket coordinates.
- It has a custom, non-cub radix sort (`msm/sort.cuh`) and persistent device buffers allocated once per `msm_t`.
- Its accumulate kernel is cooperative: it hands out buckets from an atomic work queue and ends with a grid sync (`pippenger.cuh:166-221`, launched with `launch_coop` at :515-518).
- Per-window integration runs on the device; the final window combination runs on the host (`collect`, :560-565, `integrate_row` :627-665).
- It has no large-bucket handling: the work queue balances across buckets but never within one.

### a2. NTT (`include/ntt/ntt.cuh`, `src/ntt/mixed_radix_ntt.cu`, `include/ntt/mixed_radix_thread_level_ntt.cuh`)

1. **Domain object**, one per device, static (`Domain`, `ntt.cuh:398-443`; `init_domain` :445-532).
   - Twiddles are generated on the device once, up to the largest log size (:480-486).
   - An optional "fast twiddles" mode costs about 4N extra memory (:488-507).
   - A host hash map from coset generator to twiddle index covers only the powers of the primitive root (:509-527).
2. **Algorithm choice** (`is_choosing_radix2_algorithm`, `ntt.cuh:585-612`): mixed-radix for log n ≥ 16, radix-2 for log n ≤ 11, and a size×batch rule between.
3. **Mixed radix.**
   - A transform of 2^k is factored into up to 5 stages of radix 2^4, 2^5 or 2^6 from a fixed table (`STAGE_SIZES_DATA`, `mixed_radix_thread_level_ntt.cuh:15-25`). For example 2^20 = 5+5+5+5 and 2^22 = 6+5+5+6, so **4 global-memory passes**.
   - Each pass is one kernel (`ntt64`, `ntt32`/`ntt32dit`, `ntt16`/`ntt16dit`, `mixed_radix_ntt.cu:159-525`) with 64 threads per block. Each thread holds 8 elements in registers and does an 8-point butterfly network; sub-transforms are exchanged through shared-memory transposes.
   - Pass order and strides are driven by the stage table (`large_ntt`, :754-904). DIT is used for natural input, DIF for natural-to-reversed.
   - A digit-reversal reorder kernel handles natural or bit-reversed orders and fuses the 1/N normalisation of the inverse (:59-127; launches :988, :991, :1005).
4. **Cosets.**
   - For a forward coset transform, an elementwise multiply by g^i (fused with the reorder) runs before the transform; for the inverse it runs after (kernel :128-157; launches :977 forward and :1010 inverse). With radix-2 the same is done by `BatchMulKernel` (`ntt.cuh:340-381`).
   - If the generator is not a power of the domain root, the n coset powers are computed **on the host, uploaded per call, then freed** (`ntt.cuh:711-726`).
5. **Batching.** `batch_size` contiguous vectors, or `columns_batch` (interleaved: element i of every column adjacent) handled by dedicated load/store paths (`mixed_radix_thread_level_ntt.cuh:211-408`). A whole batch is one launch per pass.
6. **Buffers.** The input and output may each be on the device or the host. Host buffers are staged through `cudaMallocAsync` plus `cudaMemcpyAsync` per call (`ntt.cuh:693-709`, :759-765). There is no pinned staging.
7. **Hard-coded hardware constants.** `MAX_NUM_THREADS = 512` ("hotfix") and `MAX_SHARED_MEM_ELEMENT_SIZE = 32` ("hardcoded for sm_86..sm_89"), `ntt.cuh:34-41`.

Contrast with sppark:

- sppark's `NTT::Base` (`ntt/ntt.cuh:216-245`) allocates, uploads, transforms and downloads on every call.
- `NTT_internal` (:161-214) and `Base_dev_ptr` (:344-350) work on device-resident data. Loki never calls them.
- sppark's coset type uses its own fixed group generator (`LDE_powers`, :78-98), not Midnight's ζ. So it cannot replace `distribute_powers_zeta` directly.

### a3. Device-resident polynomials and vector ops

1. **Polynomial backend** (`src/polynomials/cuda_polynomial_backend.cu:16-71`). This is only a factory that hands a CUDA stream (actually a null stream, since the per-device stream creation is commented out at :38-43) to the frontend `DefaultPolynomialContext`/`DefaultPolynomialBackend`.
   - The useful idea is in the frontend (MIT): a polynomial is one device buffer plus a state tag (coefficients, evaluations natural, evaluations reversed), and it converts lazily by an on-device NTT only when an operation needs the other form (`include/icicle/polynomials/default_backend/default_poly_context.h:29-33, 193-290`).
2. **Vector ops** (`src/field/cuda_vec_ops.cu`).
   - One thread per element for add, sub, mul, scalar-vector ops, inverse, division, reductions, bit reverse, slice and polynomial evaluation/division (:100-1102; registrations at :1106-1158).
   - Each op uploads and downloads unless the `is_*_on_device` flags are set (:20-56).
3. **Expression VM** (`execute_program_kernel` :996-1010, `program_execution` :1016-1085, interpreter `include/cuda_vec_ops.cuh:6-54`).
   - A host-built program of 32-bit instructions (8-bit opcode plus operand slots indexing a pointer table of inputs, constants and intermediates).
   - One thread per row runs the whole program.
   - Weaknesses: **every intermediate is a full-length device vector** (:1045-1047), and **there is no rotation (row offset) operand**. Both rule it out as a model for the PLONK numerator; Section (c) WP2 fixes both.

### a4. Device API, streams and memory (`src/cuda_device_api.cu`, Rust `wrappers/rust/icicle-runtime/src/{memory,stream}.rs`)

- A thin C++ shim: malloc and free (sync and stream-ordered), memset, copy and copy_async, stream create and destroy (:28-111).
- `get_device_properties` reports `supports_pinned_memory = false` (:117).
- There is no explicit memory pool: it relies on the default `cudaMallocAsync` pool.
- The Rust side has `HostSlice`, `DeviceSlice` and `DeviceVec` behind a `HostOrDeviceSlice` trait (`memory.rs:12-63`). Device memory is held as a `ManuallyDrop<Box<[T]>>` over a device pointer (`:13`, `:395-432`), freed synchronously on drop (`:439-457`).
- Async copies borrow the host slice with no lifetime tied to stream completion (`:322-349`).
- `IcicleStream`'s `Drop` only prints a warning and leaks the stream (`stream.rs:56-66`).
- These are patterns **not** to copy (Section e).

---

## (b) Loki's problems: the technique that solves each, the gain, and how it maps

All gains are Amdahl estimates against the gpu-after A3 profile (create_proof 85.91 s) unless stated. They are estimates, not
measurements. Each package in Section (c) names the measurement that decides it.

### Problem 1: bucket skew on repeated scalars

- **Technique.** Balance work within a bucket, not only across buckets. Two complementary routes:
  - (a) **Segmented accumulation.** Cut every bucket's sorted point list into segments of at most L points (L ≈ the average bucket
    size, or a fixed 64–256). Each work item sums one segment into a partial. A segmented tree reduction (log2(S/L) passes) folds
    each bucket's partials. This is icicle's large-bucket idea (a1.6), but applied uniformly to all buckets with no threshold
    heuristic, and expressed as virtual work items in sppark's atomic work queue.
  - (b) **Group by scalar** for low-cardinality inputs. Sort (scalar, point index) by scalar, then run-length encode. If the number
    of distinct non-zero scalars D is ≪ n, sum each group's points with a segmented reduction (pure additions, perfectly balanced),
    then run an ordinary MSM over the D pairs (distinct scalar, group sum). Lookup columns with ≤ 2^16 table values become n
    additions plus a tiny MSM.
  - (c) A **bitsize bound**: a device max-reduction of scalar bit length chooses the window count. Boolean or small columns then
    skip empty high windows (icicle exposes this as `MSMConfig.bitsize`, `msm.h:32-34`).
- **Gain on A3 now.** Small, because W10's `cuda-argument-commit` already moved the 16 argument commits to coefficient-basis
  (random-looking) scalars. The residual is in advice_commitments: 15 MSMs in 3.37 s against about 1.0 s at the 66 ms random rate,
  so **≤ 2.4 s (2.8 %)**. Part of that may be a cache miss (problem 4); WP3's prepare timer separates the two.
- **Gain in the worst case.** The gpu-before profile shows the class of regression this prevents: lookup_prepare plus
  permutation_products ran 75.3 s on GPU against 32.1 s on CPU: **43 s slower than CPU, and 61 s slower than the coefficient-basis
  GPU path** ((51.43 − 12.47) + (23.83 − 1.64) = 61.1 s) on one proof.
  - The argument-commit fix applies only when `params.max_k() == domain.k()`. Moriarty's parent and decider enforce exact SRS size
    (`Moriarty-loki/crates/mori-recursion/src/parent.rs:804-812`, :900, :986, :1100). I did not find the leaf's guard. **WP4 must
    first check that every chain prover takes the coefficient path.**
  - Any Lagrange-basis commitment of a structured column (advice, any future argument) stays exposed.
- **Mapping: a new kernel, inside a new crate `cuda-msm/` (clean-room).** It keeps sppark's field and curve arithmetic
  (`ff/bls12-381.hpp`, `ec/xyzz_t.hpp`, Apache-2.0) and sppark's Booth digits. Patching sppark's `accumulate` in place is
  possible, but it would fork a cooperative kernel with a grid-wide sync. A separate crate keeps sppark untouched as the
  fallback and the oracle.

### Problem 2: transfers, and nothing resident across phases

- **Technique.** Device-resident polynomial store (a2.6, a3.1 idea) plus an NTT engine that consumes and produces device buffers
  (sppark `NTT_internal` / `Base_dev_ptr`). ζ distribution and zero padding move onto the device (3 constants, `i mod 3`). A
  small pinned staging ring carries the unavoidable host traffic (pinned memory is limited on WSL2, so never pin whole columns).
  The extended (2^22) columns are produced on the device and consumed there by WP2, so they **never cross PCIe**.
- **Gain.**
  - Standalone (WP1 only, quotient still on CPU), each extended column must still come back to the host (5 GiB D2H). Only the
    2^22 upload shrinks to a 2^20 coefficient upload, and the two host copies and the per-call allocation disappear. Estimate
    218 → ~70 ms per 2^22 call and 46 → ~25 ms per 2^20 call: 40 × 0.15 + 39 × 0.02 ≈ **5–7 s (6–8 %)**.
  - With WP2, the whole extended chain is device-only: about 0.3 GiB of host traffic instead of 6.2 GiB each way. The NTT part of
    the 51.3 s quotient chain drops to roughly 0.3–0.5 s of kernel time.
- **Mapping: a new crate `cuda-poly/`.** It reuses sppark NTT kernels through device-pointer entry points (`NTT_internal`),
  adds a ζ distribute/pad kernel, and extends `cuda-runtime/` with streams, events, a device arena and the pinned ring. A
  mixed-radix NTT of our own is optional later work (Section e: do not start there).

### Problem 3: the CPU-bound quotient (47.2 s, the largest phase)

- **Technique.** Evaluate the numerator on the device. One thread evaluates all passes for its row, in exactly the CPU order.
  - Custom gates run through a compiled bytecode interpreter, with **row-offset (rotation) operands** and **per-thread register
    intermediates** (the two things icicle's VM lacks, a3.3).
  - Permutation, lookup and trash terms are fixed kernels mirroring `evaluation.rs:378-580` (877114d).
  - Inputs come from WP1's resident extended columns. The output goes straight into the device `extended_to_coeff` that
    quotient_construct needs.
  - Division by X^n − 1 on the extended coset takes only 2^(K−k) = 4 distinct values, so it is an elementwise multiply by a
    4-periodic constant.
- **Gain.** The chain quotient_evaluation + compute_nu_poly + quotient_construct = 51.3 s (60 % of create_proof). The device
  estimate is about 1.5–3.5 s:
  - ~0.3 s for coefficient uploads of about 1.5–2.5 GiB;
  - ~0.3–0.5 s for about 40–80 device NTTs at 2^22;
  - 0.5–2 s for the numerator kernel, whose gate cost is unknown;
  - ~0.05 s for the final D2H.
  - **Savings ≈ 48 s, about 56 % of create_proof**: 85.9 → ~38 s (2.25x), and prove-plus-verify 175.3 → ~127 s (1.38x).
  - Incremental over WP1 alone: ~42 s.
- **Mapping: a new crate `cuda-quotient/`** (VM kernel plus argument kernels) on top of `cuda-poly/`. A host compiler from Loki's
  `GraphEvaluator` (`evaluation.rs:580-820`) to bytecode lives in the proofs crate behind the feature flag.

### Problem 4: small resident SRS cache

- **What actually happens.** The cache (`src/poly/kzg/msm/cuda/backend.rs:66-127`) keys on base address + exact length + a full
  coordinate compare.
  - A3 makes 44 MSMs of length 2^20 and one of 2^20 − 1 (kzg_multiopen). The prefix request misses even though sppark accepts any
    `npoints ≤` the resident count (`pippenger.cuh:452`).
  - The miss evicts the least recently used of two entries, then re-validates 2^20 points serially, on curve (:88) and in the
    subgroup (:93). It also normalises, keeps a full projective copy (`source: bases.to_vec()`, :111) and re-uploads.
  - In a warm multi-proof process this cascades into about 3 misses per proof. In the recursion chain, k19 and k20 parameter sets
    are separate objects (exact-k guard), so 4 SRS arrays compete for 2 slots.
  - Every hit also pays an O(n) coordinate compare of two 144 MiB arrays (:57-62, :72-78).
- **Technique.** Key by (address, capacity) and serve any prefix n ≤ capacity. Compare only the requested prefix on a hit. Validate
  once per upload, in parallel (validation is a pure per-point check). Size the slot count by a VRAM budget (for example 4–8
  entries, about 100 MiB each at 2^20). Optionally add icicle-style precomputed shifted bases (a1.10) later, as a separate
  resident form.
- **Gain.** It depends on the miss cost, which is unmeasured. My estimate is 1–4 s per miss (a serial 2^20 subgroup check at tens
  of µs per point dominates), plus 30–60 ms per hit compare × 44. That gives **~2–8 s per proof (2–9 %)**, the lower end on a cold
  single proof, the upper end in a warm chain process.
- **Mapping: extend Loki's existing backend** (`backend.rs`, `cuda/msm.cu` unchanged). No new kernel is needed for v1.

### Problem 5: recursion latency (owner priority)

- **Facts.** Every chain proof is k19–k20 and they are proven sequentially. A3 is the only measured k20 proof. Chain latency =
  Σ (create_proof + residual), and the residual is ~89 s on A3, as large as create_proof.
- **Technique.** Per-proof acceleration (WP1–WP4), plus moving lookup_prepare's CPU work (~11 s, the largest phase once the
  quotient is on GPU) onto the device by reusing WP2's VM for expression compression and a device sort for the permutation (WP5).
  A process-wide resident SRS registry (WP3) also stops k19/k20 thrash between chain steps.
- **Gain (extrapolation, not measured).** Suppose the leaf ≈ 0.5 × A3 create_proof and the decider ≈ 1 × A3 (unverified). Then chain
  create_proof ≈ 2.5 × 85.9 ≈ 215 s today. After WP1–WP3 (≈ 85.9 → ~33 s per k20 proof) it would be ≈ 2.5 × 33 ≈ **85 s (~2.5x on
  proving)**. WP5 takes it to ≈ 2.5 × ~25 ≈ 63 s. The residual outside create_proof is untouched and must be profiled per chain step.
- **Mapping.** Covered by the packages above. In addition, a chain-level profile (create_proof vs residual per step) is a
  prerequisite measurement, not a package.

Summary of Amdahl stacking on A3 create_proof (estimates; each step assumes the previous):

| Step | Removes | create_proof |
|---|---|---:|
| today (gpu-after) | none | 85.9 s |
| WP1 resident store and NTT engine | NTT adapter overhead outside the quotient chain, plus 2^22 upload volume | ~79–81 s |
| WP2 GPU quotient (with WP1) | rest of the 51.3 s quotient chain → ~2.5 s | ~37–39 s |
| WP3 SRS registry | avoidable misses and hit compares | ~31–36 s |
| WP4 skew-robust MSM | advice-commit skew residual (≤ 2.4 s); protects against the 43–61 s class | ~30–35 s |
| WP5 GPU lookup_prepare | ~11 s CPU → ~1–2 s | ~21–26 s |

---

## (c) Clean-room work packages (ordered by value; WP1 is the prerequisite of WP2 and is listed first for that reason)

Common gates (every package; "the gate" below means all of these unless a package adds more):

- **G-pointwise.** Bit-exact equality of every output element (field elements, or affine points after normalisation) against the
  CPU path, on real-distribution inputs.
  - Inputs: A3 parent columns dumped from a profiling run (public/witness-derived values stay local), W10's randomized circuit
    generator (`w10-gpu-quotient/src/plonk/evaluation/argument_commit_gpu_tests.rs`, the `CpuExtended` oracle at :24), and the
    synthetic families: random, all-zero, all-one, boolean (zero/one mix), alternating, repeating 0..15, a single repeated
    full-width value, top-window values (r − 1, 2^254..r), and a column where one value fills 99 % of rows.
  - Sizes 2^10..2^22. Every run asserts the actual GPU completion counters (no silent CPU fallback).
- **G-W2.** `cargo run --offline --release --features <pkg-flag> --example loki-w2-gpu` (k10/k14/k17, cross-verify, determinism,
  negative controls).
- **G-stock.** A k14 proof (and the A3 proof when the runner allows) accepted by the unmodified registry `midnight-proofs 0.8.3`
  verifier (`tests/stock-profile-verifier`), consuming every byte.
- **G-fallback.** An injected device error (`LOKI_*_FORCE_ERROR`) returns the CPU result, with the inputs untouched before the
  error (the `fftW9_needs_preservation` lesson).
- **G-off.** `--no-default-features` and feature-off builds are byte-identical in behaviour (feature-off proofs equal main's).

### WP1: resident device store and NTT engine (`cuda-resident`)

- **Value.** About 5–7 s alone (problem 2). It is the substrate without which WP2's ~42 s is unreachable.
- **Files.**
  - New `cuda-poly/` crate: `src/lib.rs` (safe API), `src/store.rs`, `cuda/poly.cu` (ζ-distribute + pad, elementwise kernels;
    NTT through sppark `NTT_internal` on a caller stream).
  - Extend `cuda-runtime/src/lib.rs` (streams, events, a device arena, the pinned ring; keep the existing process lock as the
    outer guard).
  - Proofs crate: `src/poly/domain.rs` (a dispatch arm for the resident path), new `src/poly/gpu_resident.rs` (cfg).
- **Interface (Rust, sketch).**

  ```text
  struct DevPoly { buf: DevBuf<Fr>, len_log2: u32, basis: Coeff | Lagrange | Extended }   // owned by a Device arena
  impl Device {
      fn upload(&self, host: &[Fq], s: &Stream) -> Result<DevPoly>           // via pinned ring, chunked
      fn download(&self, p: &DevPoly, host: &mut [Fq], s: &Stream) -> Result<()>   // writes host only after s completes
      fn ntt(&self, p: &mut DevPoly, inverse: bool, s: &Stream) -> Result<()>       // natural order, 1/n on inverse
      fn coeff_to_extended(&self, p: &DevPoly, ext_k: u32, s: &Stream) -> Result<DevPoly> // pad + zeta^(i mod 3) + NTT
      fn extended_to_coeff(&self, p: DevPoly, s: &Stream) -> Result<DevPoly>         // NTT^-1 + zeta^-(i mod 3)
  }
  // Pending<'h>: a guard that keeps &'h mut [Fq] borrowed until its event completes (no use-after-free of host buffers)
  ```

  The host `Polynomial<F, B>` API stays unchanged. Residency is opt-in inside the prover phases that WP2 rewrites. Standalone,
  the five `domain.rs` conversions call the engine and download.
- **Implementation notes.**
  - The arena is preallocated once per process. Its size is bounded by `cudaMemGetInfo` and a budget variable, and it is
    sub-allocated, so there is no per-call `cudaMallocAsync`.
  - Two compute streams and one copy stream. The pinned ring is 2–4 × 32 MiB (`cudaHostAlloc` with every return code checked);
    on failure it falls back to pageable copies.
  - Batched launches: all columns of one phase transform in one launch where sppark allows, otherwise back-to-back on one stream.
  - The Montgomery ABI is unchanged (reduced residues, R = 2^256).
- **Lean contract.**
  - `Ntt.IsNaturalNtt` (`Ntt.lean:696`), `coset_via_natural` (:783), `icoset_via_inverse` (:788).
  - `distribute_into` / `distribute_out` (:458, :464) for the ζ kernel; `coeffToExtended` / `extendedToCoeff` (:487, :489).
  - The fallback discipline is `ifftW9_eq_idft` and `fftW9_needs_preservation` (SPEC W9 table).
  - New, small: none. The resident path computes the same functions; residency is not visible to the spec.
- **Correctness gate.** The common gates, plus all five `domain.rs` conversions at logs 10..23 and extension factors 1/2/4 (the
  W5 matrix, reused), with resident chains of 2–3 conversions compared against CPU after each step.
- **Deciding measurement.** The A3 phase table with `cuda-resident` against gpu-after, plus per-call event timing (H27 method:
  events on a quiet stream plus wall clock) for 2^20 and 2^22. **Ship if** the NTT adapter total falls from ~10.5 s to ≤ 5 s
  standalone. Also record pinned-ring hit and fallback counts and the pinned-allocation probe result (H7 §4).

### WP2: GPU quotient numerator (`cuda-quotient`, implies `cuda-resident`)

- **Value.** About 42 s on A3 beyond WP1 (create_proof 85.9 → ~38 s with WP1). It is the largest single item and applies to every
  chain proof.
- **Files.**
  - New `cuda-quotient/` crate: `cuda/vm.cu` (bytecode interpreter), `cuda/args.cu` (permutation / lookup / trash term kernels),
    `src/lib.rs`.
  - Proofs crate: new `src/plonk/evaluation/gpu.rs` (graph → bytecode compiler, launch, CPU fallback); `src/plonk/prover.rs`
    (`compute_nu_poly`, :667-720, and quotient_construct dispatch under cfg).
- **Design.**
  1. **Layout: full-extended resident by default.** On A3: 17 advice/instance columns, 22 argument columns, the fixed and
     permutation columns and 3 Lagrange selectors at 128 MiB each. That is ≈ 6–11 GiB, which fits the 5090's 32 GB. Keep a
     streaming variant for memory pressure that extends lookups one at a time, as the CPU does (`evaluation.rs:467-476`).
     - Fixed and permutation key columns are uploaded in **coefficient** form (32 MiB each) and extended on the device, instead of
       uploading the PK's 128 MiB extended cosets.
     - Advice coefficients come from the host trace (in the later residency stage, from WP1 buffers left by `lagrange_to_coeff`).
     - A per-sub-coset layout (4 cosets of 2^20) would cut memory 4x and turn rotations into in-coset shifts. It needs a new Lean
       decimation lemma (only the two-way `Ntt.dft_split`, :159, exists), so it is not the default.
  2. **Bytecode.**
     - The host compiles `GraphEvaluator` (constants, rotations, calculations: Add, Sub, Mul, Square, Double, Negate, Horner,
       Store; `evaluation.rs:115-131`) into a linear program.
     - Operands are: a register (per-thread intermediate), a constant (constant memory), a column read `(column id, rotation id)`,
       a challenge, or y/β/γ/θ/trash.
     - The compiler does liveness analysis and register allocation, so the register file is the peak live set, not the
       intermediate count. Programs and column pointer tables live in constant memory.
     - Rotation index: `(idx + rot · 2^(K−k)) mod N` with Euclidean remainder, exactly `get_rotation_idx`.
  3. **Kernel shape.**
     - Grid-stride over rows; each thread handles 1–2 rows (more rows give ILP but more registers).
     - Pass order inside the thread exactly matches the CPU's nesting: for each circuit, custom gates, then the permutation
       constraints in set order, then each lookup, then each trashcan. Every term is accumulated as `value = value * y + term`
       (Horner in y).
     - β·ζ·ω_e^idx (`beta_term`, `current_delta`) comes from a precomputed powers table, or one `pow` per thread with incremental
       multiplies.
     - Fixed term kernels for permutation and lookup are fine as separate launches only if they consume and rewrite `values` in
       the same pass order. Prefer one fused launch per circuit.
  4. **Output.** `values` stays resident and feeds `divide_by_vanishing_poly` (a 4-periodic constant multiply) and the device
     `extended_to_coeff`. The quotient pieces are downloaded for commitment. Later, commit them on the device straight from the
     resident buffer through the MSM, with no host hop.
  5. **Fallback.** Any device error recomputes on the CPU from host inputs (they are never modified).
- **Interface.**
  `fn evaluate_numerator_gpu(&self, domain, cs, advice, instance, fixed_coeffs, challenges, y, beta, gamma, theta, trash, lookups,
  trashcans, permutations, pk_perm_coeffs) -> Option<DevPoly>` mirrors `Evaluator::evaluate_numerator` (`evaluation.rs:305`).
  The caller falls back on `None`.
- **Lean contract.**
  - `Plonk.numeratorAt` (`Plonk.lean:1158`), `numeratorAt_evaluates` (:1164), `EvaluatesNumerator` (:1122), `hornerY_swap` (:221)
    (fixed term order).
  - `Fusion.fused_numerator_eq` (`Fusion.lean:137`), `sequential_eq_fused` (:106), `fusedChunked_eq` (:116) (any row chunking,
    which covers any grid shape), and `fused_order_matters` (:168) (the warning).
  - `betaTerm_eq` (:179); `compile_spec` (:610) and `sharedGraph_eval_eq` (:750) for the compiled graph (O15.1);
    `Ntt.rotate_extended` (`Ntt.lean:643`) for rotations.
  - New obligation: **the bytecode compiler and its register allocation preserve `Graph` evaluation**. State it as an extension of
    `compile_spec`: the allocated program evaluates to `val g` for every root. The per-sub-coset variant would add the decimation
    lemma.
- **Correctness gate.** The common gates, plus:
  - The full 4n-entry numerator vector on the GPU equals `evaluate_numerator::<CpuExtended>` pointwise. Use W10's generator (15
    schedules, multi-wrap positive and negative rotations, multiple instances) and the 5 challenge families of
    `argument_commit_gpu_tests.rs:302-330` (all-zero, all-one, all-minus-one, random, mixed).
  - The same on the real A3 circuit's dumped inputs.
  - h(X) coefficients equal CPU.
  - A3 accepted by Moriarty's verifier and k14 by the stock verifier.
- **Deciding measurement.** A3 `quotient_evaluation + compute_nu_poly + quotient_construct`: 51.3 s → target ≤ 4 s, with zero
  fallbacks. Record kernel event times for the VM, arguments and NTTs separately, peak device memory, and register and spill
  counts from `-Xptxas -v` on sm_120.

### WP3: SRS registry (`cuda-srs-registry`, implies `cuda`)

- **Value.** About 2–8 s per proof (problem 4). Cheapest package (days, not weeks). Larger in warm multi-proof processes and in
  the chain.
- **Files.** `src/poly/kzg/msm/cuda/backend.rs` (cache → registry), `src/poly/kzg/msm/cuda/tests.rs`. `cuda/msm.cu` is unchanged:
  `loki_gpu_msm` already passes `n` and sppark accepts n ≤ the resident count.
- **Design.**
  - Each entry holds (address, capacity, validated host copy of the bases, resident handle).
  - Hit rule: same address, request n ≤ capacity, and the first n coordinates equal (exact compare, parallel `memcmp` on raw
    limbs, only the prefix).
  - On a miss: validate in parallel (on curve, subgroup), then upload. Eviction is LRU under a byte budget (default: 4 entries
    or 1 GiB).
  - Store the validated copy in affine wire form (96 B per point) instead of projective, which halves host memory and compare
    bandwidth.
  - Add profile counters `msm.prepare` (calls, ns), `msm.hit` and `msm.miss`.
- **Lean contract.**
  - `GpuSystem.residentCache_sound` (`GpuSystem.lean:276`) and `stockHit_sound` (:325), restated for a prefix hit: the hit
    predicate compares only the first n entries. New small lemma: `stockPrefixHit_sound`. The `Inv`/`InvV` invariant is preserved
    under `take`.
  - `msmSpecific_take_min` (:367) (an MSM over the first n bases is `msmSpecific`).
  - `runV_sound` (:530), `msmW9_sound` (:575), `dispatchSeqW9_eq` (:606), `KernelOkOnValid` (:568): validation once per upload is
    still the precondition.
  - **Do not** switch to digest keying: `srs-identity-immutable` (SPEC obligations table, `digestHit_sound` :349) is open.
- **Correctness gate.** The common gates, plus:
  - Request sequences mixing lengths 2^20, 2^20 − 1, 2^19 and arbitrary prefixes, interleaved coefficient/Lagrange and k19/k20,
    each equal to CPU `msm_specific`.
  - The existing `gpu_rejects_corrupted_resident_base` (`backend.rs:181-203`, 877114d), plus a new test where the same address holds
    mutated contents (the hit must fail).
- **Deciding measurement.** A3 gpu-after against registry, using the new counters: misses per proof (expect 2 cold, 0 warm) and
  `kzg_multiopen` and `advice_commitments` deltas. Also a two-proof warm run in one process.

### WP4: skew-robust MSM (`cuda-msm-balanced`, implies `cuda`)

- **Value.** ≤ 2.4 s on A3 today. It removes the 43–61 s regression class on every Lagrange-basis or structured-scalar path, and
  makes `cuda-argument-commit` an optimisation rather than a necessity.
- **Files.** New `cuda-msm/` crate (`cuda/balanced.cu`, `src/lib.rs`), using sppark headers for field and curve types only.
  `src/poly/kzg/msm/cuda/backend.rs` gets a strategy dispatch. sppark's `msm_t` stays as the fallback and the differential oracle.
- **Algorithm (our formulation).**
  1. **Digits.** One thread per scalar: Booth signed digits of width w from Montgomery input, converted in-kernel (sppark's
     approach). Emit `(key = window << (w−1) | |digit|, val = point index | sign << 31)` and drop zeros. Window count = ceil(B / w)
     with B = the device max bit length (one reduction kernel). Choose w from n and B.
  2. **Sort.** cub radix sort on just the bits needed (log2 windows + w − 1), then run-length encode and scan into bucket offsets.
     No host read-backs: sizes stay on the device and launch extents use an upper bound (n × windows / L + buckets).
  3. **Balanced accumulation.** A virtual work item is (bucket, segment of ≤ L points). Each item sums its segment with xyzz mixed
     additions into a partial. A segmented reduction over the partials, keyed by bucket and log2(max segments) passes or one
     warp-level tree per bucket, yields the buckets. L is tuned on sm_120 (start at 128).
  4. **Low-cardinality mode** (selected when the run-length encode over (scalar) gives D ≤ n / 64; one extra sort of 256-bit keys
     as 8 × 32-bit or 4 × 64-bit LSD passes). Segment-sum the points of each distinct scalar (balanced, additions only), then run
     steps 1–3 on the D pairs. Zero scalars are dropped first (`msm_remove_zero_scalar`).
  5. **Window reduction.** Each window's 2^(w−1) buckets are split across a block. Each thread computes a local running sum and a
     weighted sum. Combine with one small scalar multiple per thread of the segment sum by its offset, then do one Horner
     double-and-add over windows on the device. One result point goes back.
  6. **Batch API.** Several scalar vectors over one resident base array in one call (the 15 advice commits, the 10 + 6 argument
     commits). Scalars upload through the WP1 pinned ring while the previous MSM runs. Results are emitted in program order.
- **Interface.** `fn msm_batch(bases: &ResidentBases, scalars: &[&[Fq]]) -> Result<Vec<G1Projective>>`. The single MSM is a
  batch of 1. Strategy is chosen by `LOKI_MSM_STRATEGY = sppark | balanced | auto` (auto: balanced when a cheap device probe —
  max bucket size of window 0 > 8 × average — or the low-cardinality test fires).
- **Lean contract.**
  - `Msm.pippengerS_eq_msm` (`Msm.lean:443`) (signed digits); `bucket_append` (:283) (a bucket = the sum of its segments);
    `chunkedMsm_eq` (:888) and `chunkedMsm_perm` (:900) (any chunking and reduction order).
  - `msmSpecific_perm` (:545) (sorting is a permutation); `msm_filter_nonzero` (:93) and `msm_remove_zero_scalar` (:65)
    (dropping zeros); `msmBatch` (:118).
  - `GpuSystem.asyncBatch_meets` (:723) and `async_emit_eq` (:743) for the batch and emission order.
  - New, small: **group-by-scalar**: `msmSpecific cs bs = msmSpecific (distinct cs) (groupSums cs bs)`. It follows from
    `msmSpecific_perm`, `msmSpecific_append` (:552) and `msm_add_points` (:113), but needs its own statement.
- **Correctness gate.** The common gates, with emphasis on the scalar families above. Add the existing fuzz `msm_diff` (w7-fuzz)
  run with `LOKI_MSM_STRATEGY=balanced`, plus ≥ 10^4 random small cases comparing balanced, sppark and CPU at n ≤ 2^12
  (degenerate buckets: all in one bucket, one point per bucket, negated duplicates P and −P with equal digits).
- **Deciding measurement.** W10's `gpu_structured_scalars` benchmark at k16/k18/k20. **Ship if** every pattern ≤ 1.2 × the
  random-scalar time and random ≤ 1.1 × sppark. Then the A3 advice_commitments delta, and a Lagrange-path A3 (argument-commit off)
  showing lookup_prepare ≤ the CPU value.

### WP5: lookup preparation on the device (`cuda-lookup-prepare`, implies `cuda-quotient`)

- **Value.** About 8–10 s on A3: lookup_prepare minus its GPU MSMs and NTTs is ~11 s of CPU at Rayon 4. Once WP2 lands it is the
  largest remaining phase.
- **Files.** `src/plonk/lookup/prover.rs` (`commit_permuted`, the permute step at :391-431 per SPEC), `cuda-quotient/` (reuse the
  VM in the Lagrange basis), new `cuda/permute.cu`.
- **Design.**
  1. **Compress** the input and table expressions with θ on n rows using the WP2 VM. It is the same compiled graph machinery,
     with a Lagrange basis and rotation stride 1.
  2. **Permute.** This must produce **exactly** Loki main's deterministic output (sorted leftover keys,
     `loki_lookup_order_free`), not merely a valid permutation, so W2's determinism byte-compare still passes.
     - Sort the compressed input (stable radix sort on 256-bit canonical values).
     - Mark the first occurrence of each run; per run, take the table value at the run start.
     - Collect leftover table entries by multiset difference (sort the table, count-match with a segmented scan).
     - Fill the non-first slots from the sorted leftover keys in order, exactly the Rust `pop()` order.
  3. Download a′ and s′ only if the host needs them. They go straight into the coefficient commit (argument-commit path) and stay
     resident for the quotient.
- **Lean contract.** `Refinement.loki_lookup_valid`, `loki_lookup_order_free`, `lookupPermuteWith_ord_unique`,
  `sorted_perm_eq_insertionSort`, `sortOrd_sorted_rust` (SPEC first table). `Pipeline.lookupColsBatch_eq` (O16.1) for batching the
  lookups. `Fusion.compile_spec` for compression.
- **Correctness gate.** The common gates, plus: a′ and s′ byte-equal to the CPU on A3 dumps and on W10's random lookups (tables
  with repeats, inputs with all-equal values, inputs absent from the blinding rows). The W2 `determinism` test must be
  byte-identical.
- **Deciding measurement.** First add sub-phase timers (compress / permute / commit) to the CPU path. If permute is < 3 s, ship
  only the compression part. A3 lookup_prepare target is ≤ 3 s.

Not packaged, but recommended alongside: a chain-level profile (per step: create_proof, PK setup, child checks, verification),
because the ~89 s A3 residual may dominate recursion latency after WP1–WP3.

---

## (d) Risks

**sm_120 (RTX 5090, consumer Blackwell)**

- icicle's tuning constants are explicitly for sm_86–89 (`ntt.cuh:34-41`; `cuda_msm.cuh:651` "TODO: device dependent"). Nothing
  from them transfers. Tune segment length L, window width, block sizes and NTT passes on sm_120 with events.
- sppark's `accumulate` is a cooperative launch with a grid-wide sync (`pippenger.cuh:219`, `launch_coop`). Any kernel that
  shares its work queue must stay co-resident at the chosen occupancy. Register growth in a modified kernel can make the
  cooperative launch fail (`cudaErrorCooperativeLaunchTooLarge`). Keep WP4's kernels non-cooperative.
- Field arithmetic is integer multiply-add chains (sppark PTX `mad.lo.cc`/`madc`). Build native `sm_120` SASS plus a
  `compute_80` PTX fallback, as `cuda-ntt` already does (`LOKI-GPU-NTT.md`). Check `cuobjdump -lelf` for `sm_120` in every new
  crate, since a PTX-only build JIT-compiles at first launch and adds seconds.
- The WP2 VM is register-hungry. Spills to local memory on sm_120 can erase the gain. Measure with `-Xptxas -v` and cap rows per
  thread. The NVRTC/JIT alternative (one generated straight-line kernel per circuit) is faster but adds a runtime toolkit
  dependency and compile latency. Defer it.
- The WSL guide does not mention sm_120 (H27 §1). Confirm compute capability 12.0 by runtime query in every new crate's probe.

**WSL2 / WDDM**

- Pinned host memory is limited and Windows-managed, likely below native (H7 §1, staff post). Keep the pinned ring small
  (2–4 × 32 MiB), check every `cudaHostAlloc`, and fall back to pageable copies on failure.
- **Never `cudaHostRegister` a Rust `Vec`.** Its support on WSL2 is unconfirmed (H7 §2, H27 §1), and registration may not even
  page-lock (H27, `pageableMemoryAccessUsesHostPageTables`).
- Pageable `cudaMemcpyAsync` may synchronise and stage (H27 §2). Do not count on overlap from pageable memory. Attribute timings
  with events plus wall clock, per H27 §3.
- WDDM adds launch overhead (H7 §3, a community report). Prefer few, batched launches (all columns of a phase per launch) over
  icicle-style many small kernels, and avoid host read-backs mid-MSM (icicle does about 5 per MSM: `cuda_msm.cuh:591-598`,
  :659-660, :704-709).
- Windows GPU timeout detection (TDR) may apply to long kernels under WDDM. This is unverified on this host. The gpu-before A3
  completed with skewed kernels estimated at seconds, which suggests no kill at that scale, but kernel durations were never
  traced. WP2 and WP4 kernels should stay well under 1 s each.

**Correctness and spec**

- Pass order in the numerator matters (`Fusion.fused_order_matters`). Any reordering of the permutation, lookup or trash terms,
  or of sets and lookups, changes h(X) and makes proofs non-byte-identical. With fixed challenges they become invalid.
- Rotation arithmetic must use the Euclidean remainder (`rotIdxI128_eq`/`rotIdxI32_eq`). Negative multi-wrap rotations are where
  GPU index math usually breaks.
- Lookup determinism (WP5) is stronger than validity. A valid but differently ordered permutation passes the verifier but fails W2
  determinism and changes proof bytes.
- Resident buffers create a new failure mode: a device error after host state has advanced. Keep the rule that host inputs are
  read-only until the device result is complete, and keep the sticky disable flag per backend.
- Device memory: full-extended residency on A3 is estimated at 6–11 GiB. Larger circuits, or a future k21, double it. The
  streaming variant must exist before k21.

**Measurement**

- A3 is a single shared-host sample with no variance. Every package's decision needs ≥ 3 samples per mode (the matched-triple
  launcher), and the chain extrapolation in (b) needs real leaf and decider profiles.

---

## (e) What not to do

1. Do not vendor, copy or translate icicle backend code, or port its kernels line by line (licence, Section 0). Implementers
   should not have icicle backend files open while writing kernels.
2. Do not replace sppark wholesale. Its field and curve arithmetic, Booth digits, xyzz buckets and NTT kernels are good and
   Apache-2.0. Add balanced accumulation beside it and keep it as the oracle and fallback.
3. Do not write a new mixed-radix NTT first. The A3 NTT cost is host overhead (≈ 218 ms per call against a few ms of kernel time),
   not butterflies. Residency and batching come first, and a custom NTT only if kernel events show sppark's NTT above ~10 % of
   the remaining phase.
4. Do not use icicle's arbitrary-coset path or its idea of uploading coset tables. ζ is not in the 2-adic subgroup, so that path
   (`ntt.cuh:711-726`) would upload a 128 MiB table per transform. Use the 3-constant `i mod 3` kernel.
5. Do not use an expression VM whose intermediates are full-length device vectors (`cuda_vec_ops.cu:1045-1047`). At 2^22 rows and
   hundreds of intermediates that is tens of GiB and pure memory traffic. Use per-thread registers.
6. Do not key the SRS cache by digest or by address alone (`srs-identity-immutable` is open). Do not drop validation on upload
   (`KernelOkOnValid` is the precondition).
7. Do not copy icicle's Rust memory model: device pointers in `Box<[T]>`, async host copies without a lifetime tied to completion,
   and streams that warn and leak on drop (`memory.rs:13`, :322-349; `stream.rs:56-66`). Use a guard (`Pending<'h>`) that holds
   the host borrow until the event completes, and destroy streams in `Drop`.
8. Do not spawn detached threads for stream cleanup (`cuda_msm.cuh:1123`, :1391). Under the process device lock and sticky
   failure flag this hides errors and races with shutdown.
9. Do not pin whole polynomials or call `cudaHostRegister` on Rust allocations under WSL2.
10. Do not add per-call `cudaStreamCreate`, `cudaMallocAsync` or `cudaFreeAsync` in hot paths (icicle does this throughout, and
    sppark's `NTT::Base` allocates per call). Use one arena and long-lived streams.
11. Do not claim speedups from overlapping or unattributed runs (the W10 rule). Do not merge a package without its deciding
    measurement and the three common gates.
12. Do not enable any package by default until the stock-verifier gate passes on a real chain proof, not only on k14.
