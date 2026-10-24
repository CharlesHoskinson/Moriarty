# astra-3-6: Nocy server caching, batching, and break-even (GPU prover)

## Question
Does the public Nocy GPU proof server (and upstream midnight-zk PR #157) show real batching, key/SRS caching, or persistent-context timing? Is the whole proof on GPU, is native witness work on CPU, can MSMs share SRS, what is the prep-factor break-even, does batching fill an underused GPU versus double-counting quotient gains, and is WSL CUDA-graph support relevant? No speedup promises.

## Answer
- **Upstream PR #157 (midnight-zk, ICICLE v4 POC) gives no numbers and is closed.** Title "[DO NOT MERGE] POC - Add GPU acceleration support", K>=14 threshold, CPU BLST below / GPU ICICLE above a threshold, "batch processing support" claimed in the description. Closed 8 Dec 2025 by the author, who moved work to a midnight-ledger `gpu-integration` branch. No benchmark table in the thread. [capture raw/pr157.md](https://github.com/midnightntwrk/midnight-zk/pull/157)
- **Nocy is a separate fork of midnight-ledger's proof server (not upstream).** Build pins "Variant of 6.1.0-alpha.6" of the ledger, not Moriarty's proofs 0.8.3. Its benchmark (k17 4.4 s GPU vs 12.3 s EPYC 7642; k18 8.8 s vs 24.6 s; 10-run average; 2.80x) is whole-server, but the circuit, curve path and whether it is BLS12-381 are not stated. [capture raw/nocy-repo.md](https://github.com/Nocy-io/nocy-gpu-proof-server); [raw/nocy-forum.md](https://forum.midnight.network/t/open-source-drop-cuda-accelerated-midnight-proof-server-nocy-gpu-proof-server/972)
- **Nocy batching claim: none in code I read.** The C ABI (`cuda/include/mn_prover.h`; Rust binding `mn_prove_input` at `native-plonk/native-plonk/src/lib.rs:35`) takes one proof request. The GPU MSM path commits one polynomial per call (`kzg_commit_impl`, `native-plonk/cuda/src/kzg.h:102-162`), synchronises the stream after each commit (`kzg.h:155`), and uses the default stream (`nullptr`, `kzg.h:112`). No cross-column batching. [raw/nocy-native-plonk_cuda_src_kzg.h.txt]
- **Nocy key/SRS caching: partial and process-lifetime.**
  - SRS points are uploaded once per host pointer and reused (`ensure_kzg_points_cache`, `kzg.h:64-80`; keyed on `host_ptr` and capacity). This is a real resident-base cache, separate per monomial and Lagrange array.
  - Scalars are copied per commit from pageable host vectors (`cudaMemcpyAsync`, `kzg.h:121`), so no pinned overlap.
  - Rust side: `PackedPolysCache` holds one entry (`lib.rs:60-63`). Params and prover key are re-packed on every call unless already in packed format (`pack_params` `lib.rs:155`, `pack_prover_key` `lib.rs:165`). No persistent CUDA context object across requests; the GPU graph cache is a static mutex-guarded global (`plonk_prover.cu:234-235`) gated by env var `MIDNIGHT_CUDA_GRAPH_GPU` (line 121). "Graph" here means device-side gate-evaluation tables, not the CUDA Graphs API. No `cudaGraph*` calls in the files I fetched.
- **Whole proof on GPU? Mostly, but not witness preprocessing.** `mn_prove` consumes `preproc` bytes (`mn_prove_input`, `cuda/include/mn_prover.h:9-17`). Preprocessing runs on CPU in `witness/src/lib.rs:13 preprocess_ir` and is packed by `preprocess_and_pack` (called at `lib.rs:438`). The GPU does FFT, quotient/gate evaluation, MSM and the transcript/hash path (`blake2b.h` present). Whether the native transcript stays on host was not confirmed.
- **Nocy's MSM is not Pippenger.** `g1_scalar_mul` (`native-plonk/cuda/src/msm.cuh:104-131`) does one 4-bit fixed-window scalar multiplication per point (table of 16, 64 windows, 4 doublings each), then a pairwise tree reduction (`msm_reduce_launch`). The scalar field is BLS12-381 Fr, modulus 0x73eda753... (`field.cuh:51`), which matches Midnight's `Fq`. ESTIMATE from operation counts, not a measurement: per point about 256 doublings + ~60 adds + 14 table adds (about 330 group ops), against roughly 16 mixed adds per point plus bucket reduction for Pippenger at c=16. At N=2^20 that is roughly 3.5e8 versus 1.9e7, about 15–20x more group operations. Implication: the 2.8x whole-proof speedup is not evidence of MSM-optimal GPU performance; the MSM is the place where a proper bucket-sum kernel could gain most. Do not claim this as measured.
- **Can compatible MSMs share SRS?** Yes. Commitments over the same base array can share one resident base table (what Nocy already does for a single base array, and what ICICLE's shared-base batch does). Only the scalars and outputs differ. Batching does not combine transcripts: each commitment stays a separate output in transcript order. Merging distinct proofs' MSMs into one commitment would be invalid. Node-02 phase ordering (advice, then permutation/lookup, then quotient) still bounds which commitments can be batched.
- **Break-even for precomputation (formula from node 02, checked):** precompute factor p means p points stored per base (ICICLE: "total number of points to precompute for each base point", `raw/icicle-precomp.md`). Break-even reuse m > T_prepare / (t_plain - t_prepared). Using the only measured pair (ICICLE, RTX 3090 Ti, 2^20 BLS12-381): t_plain = 25.5 ms, t_prepared(p=4) = 18.1 ms, saving 7.4 ms per MSM. T_prepare is not published by ICICLE, so no break-even number exists in the sources. Illustration only: if T_prepare = 50 ms, m > 6.8 commitments per base table; a p=4 k20 table is 4 x 96 MiB = 384 MiB per base array. Memory is the binding cost, not time.
- **Batching fills underused GPU? Partly, and it is already measured for small MSMs.** ICICLE 2^10 batch of 2^12 MSMs: 136.7 ms (p=1) to 116.0 ms (p=8), i.e. about 32.6 ns per point at p=1, versus 24.3 ns per point for one 2^20 MSM at p=1. Batching small MSMs does not hide per-point cost. Its benefit is launch amortisation and filling SMs for small sizes. For Nocy's serial one-commit-per-call loop with default-stream sync after each call, the GPU idles during host work between commits. That idle gap is what batching could fill. Not measured.
- **Double counting quotient and MSM gains:** the two residency gains both remove host-device transfer of the same polynomials (node 02 SRS and scalar transfers; node 03 extended columns). Do not add their speedups multiplicatively. Model per phase on one shared timeline: T = sum over phases of max(compute, transfer) when transfers overlap, else sum. Get phase timings first (node 01 / L0).
- **WSL CUDA graphs:** the WSL guide (captured) lists WSL limitations for managed memory, concurrent CPU/GPU access, pinned memory, NVML queries and OpenGL interop. It does not list CUDA graphs as unsupported, and it does not explicitly support them either. Relevance is only for launch-latency reduction across the many small per-column kernels, which is not established. Test with a micro-benchmark on this host before using it. No CUDA-graph claim is made here.

## Raw numbers
| Item | Value | Source |
|---|---|---|
| Nocy k17, GPU RTX 5090 / CPU EPYC 7642 | 4.4 s / 12.3 s (2.80x) | README, 10-run average |
| Nocy k18, GPU / CPU | 8.8 s / 24.6 s (2.80x) | README |
| ICICLE 2^20 BLS12-381 MSM, 3090 Ti, p=1/2/4/8 | 25.5 / 20.3 / 18.1 / 17.2 ms | ICICLE 1.10.1 doc |
| ICICLE 2^10 batch of 2^12, p=1/2/4/8 | 136.7 / 123.8 / 117.8 / 116.0 ms | ICICLE 1.10.1 doc |
| p=4 table size per base array at k20 | 4 x 2^20 x 96 B = 384 MiB | arithmetic (96 B affine, ICICLE stores p points per base) |
| Break-even reuse (illustrative) | m > T_prep / 7.4 ms | arithmetic; T_prep not published |
| Nocy k20 commit group ops (estimate) | ~3.5e8 (naive windowed) vs ~1.9e7 (Pippenger c=16) | op-count estimate, not measured |
| PR #157 benchmark numbers | none | closed POC |

## Caveats
- No measured Nocy batching, caching or persistent-context timing exists in the public materials. Only the single k17/k18 table.
- The GPU MSM op-count comparison ignores doubling-vs-addition cost differences and any Nocy-specific kernel tuning. Treat as a direction, not a result.
- Nocy's CPU reference and proof circuits are unspecified. The 2.8x may combine MSM slowness with GPU gains elsewhere.
- ICICLE timings are from RTX 3090 Ti and their timing boundary is unstated. Nothing transfers directly to the 5090.
- I did not inspect the midnight-ledger `gpu-integration` branch referenced by PR #157; it may have the benchmarks.
- Witness preprocessing being CPU is established for Nocy's code. Whether Midnight's own proof path matches is not checked.

## Source-capture note (added on review)
- Nocy source was read at repo HEAD `366d3c72e814691cd16bbd72e77b5169d07f729c` (raw/nocy-head-sha.txt), via the GitHub contents API (`gh api`), saved as raw/nocy-*.txt. Scrapling captures of the GitHub blob pages (raw/scrap-nocy-kzg.md, scrap-nocy-msm.md, scrap-nocy-lib.md, scrap-nocy-mnprover-h.md) are page chrome; the msm and header pages contain no code body, so do not cite them as code evidence. Only kzg.md shows the `cudaStreamSynchronize` line.
- PR #157 state confirmed via API: closed 2025-12-08T18:12:02Z.
- The per-point scalar-mul versus Pippenger comparison is an operation-count estimate, not a runtime measurement.

## Sources (captures under helpers/haiku-10/raw/)
- raw/pr157.md, raw/nocy-repo.md, raw/nocy-forum.md, raw/nocy-tree.md, raw/nocy-files.txt
- raw/nocy-cuda_src_mn_prover.cu.txt, raw/nocy-cuda_include_mn_prover.h.txt
- raw/nocy-native-plonk_cuda_src_kzg.h.txt, raw/nocy-native-plonk_cuda_src_msm.cuh.txt, raw/nocy-native-plonk_cuda_src_msm.cu.txt, raw/nocy-native-plonk_cuda_src_field.cuh.txt, raw/nocy-native-plonk_cuda_src_plonk_prover.cu.txt
- raw/nocy-native-plonk_native-plonk_src_lib.rs.txt, raw/nocy-witness-lib.rs.txt
- raw/icicle-precomp.md, raw/wsl-guide.md, raw/zkprophet-abs.md
