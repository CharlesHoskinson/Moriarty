# astra-1-8: Nocy native-plonk CUDA inspection (code only, no builds)

Source tree: shallow clone of github.com/Nocy-io/nocy-gpu-proof-server at commit 366d3c72e814691cd16bbd72e77b5169d07f729c (2026-01-19), at helpers/haiku-01/nocy-src (read-only inspection; nothing built or run). Captures: raw/haiku01-nocy-repo.md, raw/haiku01-nocy-forum.md.

## Question
Which CPU stages and host round trips explain the published 2.795x on an RTX 5090? Is witness generation and key setup included in the timing? Is there evidence that more GPU residency could exceed the published result, and what is the serial ceiling?

## Answer
- **Curve is BLS12-381 by code, not just by name.** The bench imports `midnight_curves::Bls12` (native-plonk/native-plonk/tests/bench_native_prove.rs:3) and builds `ParamsKZG::<Bls12>::unsafe_setup` (:107). The params directory is named `bls_filecoin_2p10..2p17` (params/). The `midnight-curves` crate is BLS12-381 (~/repos/midnight-zk/curves/Cargo.toml:3). The earlier "curve unstated" caveat is now resolved at code level, though the README does not state it.
- **The GPU timed region includes `pack_polys`, which does CPU witness assignment and per-column GPU commits.** `api.prove` is timed at bench_native_prove.rs:170-171. Inside it, `NativePlonkApi::prove` (native-plonk/native-plonk/src/lib.rs:133) calls `pack_polys` at :187 whenever the cache misses. `pack_polys` (native-plonk/polys-packer/src/lib.rs:523) unpacks preproc, runs halo2-style witness assignment on the CPU (`Value::known(preproc.clone())`, advice loop with random blinding per cell), and calls `commit_lagrange` per instance and advice column (GPU KZG, via gpu_kzg.rs). The cache key includes `rng_seed` (native-plonk/src/lib.rs:78-85). The bench derives a distinct seed per iteration (`build_seeds`, bench :47-66; `rng.fill_bytes` per iteration), so the cache misses on every timed GPU iteration.
- **The CPU timed region is `prove(...)`** (bench :139-145). The CPU baseline gets `preproc.clone()` as the witness, and `prove` calls halo2 `create_proof` with `Value::known(witness)` (third_party/midnight-zk/circuits/src/compact_std_lib.rs:1565-1580). Both sides therefore include advice assignment inside the timer. Both exclude `preprocess_ir` (ZKIR evaluation, bench :~105-110 via `midnight_cuda_witness::preprocess_ir`, and proof-server lib.rs:425). I did not trace halo2 `create_proof` line by line, so "both include synthesis" is an inference from the API.
- **Server path (proof-server/src/lib.rs:425-448)** matches the bench: `resolve_key`, IR deserialize, `ir.preprocess(preimage)` (425), params fetch, packed key cache, `pack_preprocessed` (441), then `OsRng` seed (444), then `api.prove` timed from 446. Per-request random seed means `pack_polys` runs every request.
- **The pure CPU prover path is excluded by env toggles only when set.** `MIDNIGHT_PROOFS_CUDA=1` and `MIDNIGHT_CUDA_USE_KERNELS=1` are set by the server (lib.rs:84-93). Debug and sanity paths are gated off by default: `sanity_enabled()` (plonk_prover.cu:35, checks at :673, :702, :762, :1929, :2038), `h_debug_enabled()` (:30), `graph_gpu_enabled()` defaults to ON (:118-124, disabled only with `MIDNIGHT_CUDA_GRAPH_GPU=0`). Timing logs are off unless `MIDNIGHT_CUDA_TIMING=1` (plonk_prover.cu:40-45, log at :54-60; mn_prover.cu stage labels at :119, :153, :212, :221-222). So the default run does not pay sanity-check cost.
- **Host round trips in the GPU proving path** (native-plonk/cuda/src):
  1. Every KZG commit: scalars H2D copy (`cudaMemcpyAsync(kzg_work_cache.d_scalars, ...)`, kzg.h:121), result D2H (kzg.h:149), and `cudaStreamSynchronize(stream)` (kzg.h:155). The points are cached on device (`ensure_kzg_points_cache`, kzg.h:114, keyed on host pointer) so bases are not recopied per commit. Commits are called from plonk_prover.cu:2309, 2315, 2389, 2531, 2612, 2690, 3358 (each is one sync point, and some are per-column).
  2. Device-wide synchronizations on the proof path, unconditional: plonk_prover.cu:1534, 1599, 1656, 1708, 1727, 1747, 1765, 3175, 3239 (9 sites; the `cudaGetLastError() || cudaDeviceSynchronize()` pattern at 1708-1765 is not behind a debug flag). Each drains the pipeline.
  3. Graph / constant upload: `build_device_graph` uploads constants, rotations, calculations and parts with `cudaMemcpy` (plonk_prover.cu:1068-1158). This is cached in `GraphGpuCache` after init (`init_graph_gpu_cache`, :1204) and is not per-proof for a fixed key, but is cold on first call.
  4. Polynomial subset upload per stage: `copy_polys_subset_to_device` (plonk_prover.cu:1166-1196) and `cudaMemcpy polys failed` path. Count of `cudaMemcpy` in the native library is 89 and `cudaMemcpyAsync` is 7 (grep over native-plonk/cuda/src/*); most are setup, not all are per-proof.
  5. Host-side vector construction of Lagrange and blinding helpers (`std::vector<Fr> l0_vals(n)`, `l_last_vals`, `l_blind_vals`, loops at plonk_prover.cu:712-800) runs on CPU before upload.
  6. `pack_polys` CPU work (above) runs every call in both bench and server, and is the largest identifiable host stage in the GPU timer. It is not measured in the repo.
- **Residency question.** Nothing in the repo measures the breakdown: `MIDNIGHT_CUDA_TIMING=1` exists, but no timing output is committed and I did not build or run it. Two facts constrain the answer. (a) Arithmetic from the README: GPU/CPU = 4.4/12.3 = 0.358 at k17; 8.8/24.6 = 0.358 at k18. So the GPU-run wall time is 35.8% of the CPU baseline, and the remainder is host work plus unaccelerated device time, whose split is unknown. (b) The sync points and per-commit scalar copies listed above are the concrete candidates for lost residency. Whether removing them could exceed 2.8x cannot be decided from code alone. The serial ceiling is therefore unknown; a bound of "more than 2.8x is possible" needs a timing run with `MIDNIGHT_CUDA_TIMING=1` plus the bench with `MIDNIGHT_CUDA_BENCH=1`.
- **The README benchmark is not produced by any script in the repo.** `k17` and `k18` appear only in README.md (lines 56-57). The bench defaults are `MIDNIGHT_BENCH_ITERS=3` (bench :189) and `MIDNIGHT_BENCH_OPS=8192` (:190), not 10 runs and not k17/k18. The bench prints `k` via `min_k()` (:196), so the README's k values can only be checked by running it. This is an important discrepancy to flag.

## Raw numbers
| Item | Value | Source |
|---|---|---|
| README k17: GPU vs CPU | 4.4 s vs 12.3 s (2.795x) | README.md:56 |
| README k18: GPU vs CPU | 8.8 s vs 24.6 s (2.795x) | README.md:57 |
| README averaging | 10 runs | README.md:52-53 |
| Bench default iterations | 3 (+1 warmup seed) | bench_native_prove.rs:189, :182-193 |
| Bench default ops | 8192 | bench_native_prove.rs:190 |
| Bench curve | midnight_curves::Bls12 (BLS12-381) | bench :3, :107 |
| Bench timer, CPU | `prove` (includes halo2 create_proof) | bench :139-145 |
| Bench timer, GPU | `api.prove` (includes pack_polys on cache miss) | bench :170-171; lib.rs:187 |
| Native timing gate | MIDNIGHT_CUDA_TIMING=1 (default off) | plonk_prover.cu:40-45 |
| Sanity gate | MIDNIGHT_CUDA_SANITY=1 (default off) | plonk_prover.cu:35 |
| Device syncs on proof path | 9 unconditional sites | plonk_prover.cu:1534-3239 |
| KZG per-commit sync | 1 stream sync per commit | native-plonk/cuda/src/kzg.h:155 |
| Top-level cuda/ mn_prove | stub, returns error | cuda/src/mn_prover.cu:193-203 |
| Real prover | native-plonk/cuda/src/mn_prover.cu:74 | |

## Caveats
- **No build or run.** All conclusions are from reading code. No measured stage breakdown exists in the repo, and I did not reproduce 2.795x.
- **Halo2 internals not traced.** The claim that CPU `create_proof` includes witness synthesis is inferred from its API, not from line-level reading.
- **Witness generation definition.** `preprocess_ir` (witness/src/lib.rs:13) is excluded on both sides. Advice assignment (`pack_polys` on GPU path; halo2 synthesis on CPU path) is inside both timers. Under this definition the comparison is symmetric; under a stricter "include ZKIR evaluation" definition the GPU path would be penalized less than the CPU path, and the ratio would change. Not quantified.
- **Vendored midnight-ledger proof server** was located (sources/midnight-ledger/proof-server) but I did not inspect its `cuda-prover-backend/` crate, which may contain another route.
- **Which k the README used is unknown.** No script links k17/k18 to a circuit.
- **Forum claim** (raw/haiku01-nocy-forum.md): "averaged over 10 runs" matches the README but not the bench defaults. Treat as the author's own run configuration, which is not in the repo.
