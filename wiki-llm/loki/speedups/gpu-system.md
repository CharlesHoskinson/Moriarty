---
title: "Loki speed-up scout: whole-prover GPU residency and scheduling (RTX 5090, WSL2)"
date: 2026-10-24
sources: "Loki source (loki-proofs: LOKI-GPU-MSM.md, LOKI-GPU-NTT.md, LOKI-CPU.md, LOKI-PROFILE.md, cuda/msm.cu, src/poly/kzg/msm.rs, src/poly/kzg/msm/cuda/backend.rs, src/poly/kzg/msm/cuda/tests.rs, cuda-ntt/); loki-lean SPEC.md; Moriarty 10x/LOKI-10X.md; sppark 0.1.16 (crates.io source); Nocy GPU proof server (github.com/Nocy-io/nocy-gpu-proof-server, forum.midnight.network topic 972); midnight-zk PR #157; Pipelonk PPoPP 2026; ZKProphet arXiv 2509.22684; cuZK TCHES 2023 / ePrint 2022/1321; GZKP ASPLOS 2023; BatchZK ePrint 2024/1862; Elastic MSM TCHES 2024 / ePrint 2024/057; EZKL/Metal MSM notes (hackmd @guard); Snarkify ZK Hub event page; NVIDIA developer forums (WSL2 pinned memory); blst 0.3.17 source"
---

# Whole-prover GPU residency and scheduling

Component: GPU residency, streams, host transfers, multi-proof scheduling, the sppark Blackwell
state, and the CPU baseline. Target: the Sherlock k20 recursive parent on an RTX 5090 (32 GB,
sm_120) under WSL2. No code was edited and nothing was committed.

## Prerequisite, before the table

**H0: confirm the 0% k20 run really used the GPU.** Check the log of that run for the one-time
`tracing::warn!` that the CUDA backend emits after its first error. After an error the backend
falls back to CPU silently, so a 0% result is what a fallback run would print. Also confirm that
the prover binary had the `cuda` feature and the default `LOKI_GPU_MSM_MIN` (16384). If H0 holds,
the table below is secondary. If H0 is ruled out, the thread count of the 759 ms benchmark decides
between the two Amdahl readings in section G.

## Ranked table (top 5 techniques, by expected k20 gain divided by effort)

| Rank | Technique | Track | Loki hook | Effort | Expected k20 gain | Evidence and source | Lean / correctness risk |
|---:|---|---|---|---|---|---|---|
| 1 | **Fix the CPU thread budget and print it in every benchmark.** Sweep `RAYON_NUM_THREADS` over 4, 8, 12, 16 and 20, with affinity. Keep blst `no-threads`, which is already set. | A | `Cargo.toml:126-128`; `src/poly/kzg/msm.rs:201-215` | Hours | Unmeasured. Depends on the thread count behind the 759 ms number (section G) | **None published.** Local hypothesis. The stock blst path runs its own pool (`blst-0.3.17/src/lib.rs:35-54`), which is the only public-source point, and it is not measured | Medium: byte equality needs a fixed thread count (`loki-lean/SPEC.md` line 26) |
| 2 | **Pinned, reusable staging and an identity-based SRS cache key.** Allocate scalar staging once with `cudaHostAlloc`. Replace the O(n) coordinate compare on every warm hit with an immutable SRS identity. | A | `cuda/msm.cu` `loki_gpu_msm`; `backend.rs` `same_coordinates` and `Cache::run` | 2-4 days | Estimated 6-17% of GPU time per transfer direction, plus the removed O(n) scan. Unmeasured on WSL2 | Two NVIDIA forum threads: `cudaHostRegister` unsupported on WSL2, `cudaHostAlloc` works, pin limits set on the Windows side ([1](https://forums.developer.nvidia.com/t/cudahostregister-not-supported-on-wsl2/279429), [2](https://forums.developer.nvidia.com/t/change-limit-of-50-for-cudahostalloc-pinned-memory-on-windows-10-11/228235/8)). Pinning cost is Loki-independent | Low. Keep the mutation-at-stable-address test, which exists |
| 3 | **Fixed-base SRS precomputation.** Precompute window tables for the SRS prefixes once, keep them resident, and make the MSM read them. | A (no new keys: it depends only on SRS points) | `backend.rs` resident store; `cuda/msm.cu` | 1-2 weeks; memory cost to be sized (roughly 1.5 GB per 2^20 bases at a 16-bit window, to be checked) | Unmeasured for Loki. `LOKI-GPU-MSM.md` lists it as future work | **Published source:** Elastic MSM (TCHES 2024, [ePrint 2024/057](https://eprint.iacr.org/2024/057)). Reported speed-ups of up to about 192x and 223x are against two preprocessing Pippenger implementations on its own hardware. They do not transfer to sppark on this host | Low: same points, same MSM result. Test against the existing differential suite |
| 4 | **Async MSM handoff, so CPU phases overlap GPU MSM.** Launch on sppark's non-blocking streams, return a handle, and read results in transcript order. | A | `cuda/msm.cu` (`invoke` synchronises before returning); call sites in `src/plonk/prover.rs` | 1-2 weeks | Bounded by independent CPU work. The k16 profile puts MSM-bearing phases at at most 43% of wall time, mostly in serial chains | Pipelonk's pipeline executor overlaps transfer with compute ([PPoPP 2026](https://ppopp26.sigplan.org/details/PPoPP-2026-papers/38/Pipelonk-Accelerating-End-to-End-Zero-Knowledge-Proof-Generation-on-GPUs-for-PLONK-B)). BatchZK overlaps host-device copies with multi-stream execution ([ePrint 2024/1862](https://eprint.iacr.org/2024/1862)), though for batches. EZKL's Metal MSM (a roughly 2x MSM gain for about 9% end to end) is the Amdahl warning ([hackmd](https://hackmd.io/@guard/BJDmuEODke)) | Medium: `SameOrder` and `scheduler.qnt` require the absorption order to be unchanged |
| 5 | **Device-resident quotient and multiopen polynomial work.** Keep vectors on device across NTT, coset, quotient and the multiopen polynomial steps. | A (new arithmetic must match `evaluation.rs`) | `src/plonk/evaluation.rs`; `src/poly/kzg/multiopen`; `cuda-ntt/src/lib.rs` | 3-6 weeks | Quotient evaluation plus construction is 21.8% of the k16 profile, so the ceiling from that alone is 1.28x. Larger only combined with rank 4 | Pipelonk claims all PLONK operations on GPU, average 10.7x and maximum 19.4x over its baseline. Hardware, sizes and baseline are not in the abstract. ZKProphet finds that NTT becomes the bottleneck once MSM is fast, at up to 90% of GPU latency. Both are published evidence for the direction, not for Loki's numbers. High Lean risk: `Refines.quotientEval` in the Lean contract |

**Bottom line.** Published evidence says whole-prover residency is the only route to large gains,
and that overlap of transfers with compute is what makes it pay. Loki today offloads MSM and NTT
only, with synchronous calls, pageable buffers and a CPU quotient. Under those conditions the GPU
saves only the MSM and NTT share. Section G shows that this is at most about 5% if the 759 ms
benchmark ran at 4 threads, and about 20% or more if it ran at 20 threads. The brief's 0% is
consistent with the first reading and points to a broken path under the second. H0 separates them.

## Details

### A. Measured or claimed on-device fraction for end-to-end GPU provers

| System | What runs on GPU | Measured on-device share | Reported numbers | Status |
|---|---|---|---|---|
| **Loki (today)** | MSM (sppark, resident bases, sync per call) and NTT (sppark, transfer per call) | Not measured for any full proof. On the k16 synthetic profile, MSM-bearing phases (`advice_commitments` 87.9 ms, `quotient_commitment` 190.2 ms, `kzg_multiopen` 211.5 ms) are at most 489.6 of 1127.3 ms, or 43% | MSM warm 13.2x at 2^20 (57.7 ms vs 759 ms CPU); NTT k20 2.6x (46.6 vs 122.5 ms); k20 parent net 0% | Local, measured. Sources: `LOKI-GPU-MSM.md`, `LOKI-GPU-NTT.md`, `LOKI-PROFILE.md` |
| **ICICLE-halo2** (Ingonyama) | Claimed MSM and NTT only | **No figure found.** No `icicle-halo2` repository exists under `ingonyama-zk` in GitHub search, only the `aleksandar409/icicle_halo2` fork (pushed 2025-12-01). Core `ingonyama-zk/icicle` last pushed 2025-11-10 | None verified | Unverified. The ICICLE site's "around 97% of ZK protocol runtime is parallel" is marketing, not a halo2 measurement |
| **Snarkify** | Halo2 zkEVM proving on GPU, in production for Scroll and SDK chains | Not stated | "1.25x performance boost over the nearest competitor in GPU proving efficiency". No method, hardware or fraction | Unverified marketing claim ([ZK Hub event](https://lu.ma/fz3x308c)) |
| **Nocy GPU proof server** (RTX 5090, Midnight) | Native CUDA PLONK prover in `native-plonk/cuda`. `mn_prove` takes serialised params, prover key, preprocessed polynomials and RNG seed, caches parsed keys, and runs `plonk_prove` on device | Not stated. The README gives no stage split | k17: 4.4 s GPU vs 12.3 s CPU (EPYC 7642). k18: 8.8 s vs 24.6 s. 2.8x, 10-run average (forum) | Public single-page claim. The CUDA FFT path copies every transform to and from the host (`fft_host.h:102,129`). The MSM kernel has a single-thread naive launch (`msm.cu:6-9`, `blocks=1, threads=1`), so this is not an MSM-optimised reference. Which kernel the prover calls is not verified |
| **midnight-zk PR #157** | Proof of concept, ICICLE v4.0.0 CUDA backend, MSM dispatch with a threshold and BLST fallback | None | None. The PR body says "significant performance improvements" with no numbers | **Closed 2025-12-08 without merge.** Title "[DO NOT MERGE] POC - Add GPU acceleration support". The author moved the work to a ledger `gpu-integration` branch. No evidence to use |
| **Pipelonk** (PPoPP 2026, DOI 10.1145/3774934.3786448) | All PLONK operations, including those not trivially parallel, through a segmentable operator library and a pipeline executor that overlaps transfer with compute | Stated as all operations, with no CPU stages in the abstract | Average 10.7x, maximum 19.4x over the state-of-the-art baseline. Runs on 8-80 GB devices. Hardware, circuit sizes and baseline not in the abstract | Most relevant published design for residency plus overlap. Full paper not read |
| **ZKProphet** (arXiv 2509.22684) | Study of GPU ZK proving | Not a prover. After MSM speed-ups, NTT becomes the bottleneck, at up to 90% of GPU proof latency | Field multiplication is about 94% of NTT time and about 80% of MSM time. NTT kernels under-use the GPU and lack async copies | Characterisation only. GPU model not in the abstract |
| **cuZK** (TCHES 2023, ePrint 2022/1321) | Groth16 MSM, NTT and matrix ops on GPU | Not stated as a fraction | MSM 2.08-2.94x over the prior GPU implementation. End to end 2.65x (up to 4.86x) on standard benchmarks, 2.18x on Filecoin | Groth16, not PLONK |
| **GZKP** (ASPLOS 2023) | MSM and polynomial kernels on GPU | Not stated | Abstract gives 48.1x and 17.6x peak speed-ups without saying whether these are kernel or end-to-end | Unverified scope |
| **EZKL on Metal** (hackmd @guard) | MSM only | Not stated, but the result is the useful cautionary case | MSM at 2^20 from about 10 s down to 0.59 s, vs about 1.1 s CPU. End-to-end prove-time gain about 9% | Strongest published Amdahl evidence: a roughly 2x MSM gain produced about 9% end to end |
| **BatchZK** (ePrint 2024/1862) | Batch proving, sum-check and Merkle, with multi-stream host-device overlap | Overlap is the design point | Claims more than 259.5x throughput over prior GPU systems | Throughput for batches. Not single-proof latency and not MSM. Do not transfer the number |
| **Elastic MSM** (TCHES 2024, ePrint 2024/057) | MSM preprocessing with a storage trade-off | Not stated | Up to about 192x and 223x over two preprocessing Pippenger implementations | MSM kernel only |

**Read-across.** Only Pipelonk and the Nocy full-prover path cover the whole prover on device.
Neither publishes a stage split. So "what fraction stays on device" has no published answer for a
PLONK-style prover on a single GPU. The Amdahl data point (EZKL: a roughly 2x MSM gain gave about
9%) is the best guide to rank 4 alone.

### B. Host-to-device transfer costs and pinned memory under WSL2

What Loki does now:

- `loki_gpu_prepare` uploads SRS bases from a pageable `Vec` on the zero stream, then synchronises
  (`cuda/msm.cu`, the comment near line 33). Bases stay resident, with at most two SRS slots
  (`backend.rs`, `Cache`).
- Each `loki_gpu_msm` call uploads scalars from a pageable wire array, runs kernels on flip-flop
  streams and returns after synchronisation (`cuda/msm.cu` lines 51-52).
- NTT (`cuda-ntt/src/lib.rs`): each transform copies input to the device and copies the result back.
  The Loki NTT notes say device residency was deferred.

Facts gathered:

- `cudaHostRegister` is not supported on WSL2, but `cudaHostAlloc` works ([forum](https://forums.developer.nvidia.com/t/cudahostregister-not-supported-on-wsl2/279429)).
- NVIDIA staff say the pinning limit is set on the Windows side, and that WSL2 pin limits are
  likely lower than native Windows ([forum](https://forums.developer.nvidia.com/t/change-limit-of-50-for-cudahostalloc-pinned-memory-on-windows-10-11/228235/8)).
- Pinning is an expensive allocation. Allocate once and reuse. One user saw about 20% slower
  overall after switching every `malloc` to `cudaHostAlloc`.
- nvidia-smi reports PCIe gen 5 x16 for the 5090, with 32607 MiB total.

Estimates (not measured on this host):

- 2^20 scalars are 32 MiB. At about 8 GB/s pageable this is about 4 ms per direction. At about
  25 GB/s pinned it is about 1.3 ms.
- The k20 `coeff_to_lagrange` GPU time is 46.6 ms. The round trip of 32 MiB each way is about 6-17%
  of that, depending on pinned or pageable.
- A WSL2 bandwidth check is needed before any of this is relied on: the CUDA `bandwidthTest`
  sample in pinned and pageable modes, with sizes of 32 MiB and 256 MiB.

### C. CUDA streams that overlap CPU synthesis with GPU arithmetic

- sppark `stream_t` creates non-blocking streams. It has a zero stream and three flip-flop streams
  (`sppark-0.1.16/sppark/util/gpu_t.cuh` lines 57-65 and 180-181), with async copies and frees.
- Loki's MSM uses those streams, but the host call is synchronous, so no CPU work runs during a
  GPU MSM. The mutex in `backend.rs` serialises all MSM calls.
- The NTT adapter has its own lock (`cuda-ntt/src/lib.rs` lines 11 and 37). sppark shares its
  streams, so concurrent MSM and NTT calls are not safe until one lock or context policy exists
  (`LOKI-GPU-NTT.md`, around line 220). The sequential default never overlaps the two.
- Overlap design: return a handle from `loki_gpu_msm`, run the next independent CPU phase, then
  wait. Dependencies limit how much can overlap. The quotient must exist before its commitment,
  and the multiopen MSM is at the end of its chain. The realistic overlap is within a phase, for
  example commitment chunks of the next column while the previous one runs on the GPU.
- Expected ceiling is min(GPU time, independent CPU time). This cannot be sized until the k20
  per-phase profile exists.

### D. Multi-proof throughput scheduling

- VRAM is the binding constraint. `LOKI-10X.md` estimates that two heavy proofs peak near 30.7 GB
  of the 32 GB card. So only one heavy GPU job at a time, with the next job's CPU witness
  overlapping the current GPU phase.
- Host RAM is 92 GB in `.wslconfig` (`memory=92GB`), with a 32 GB swap file. Each k20 proof is about
  16 GB RSS (brief), so CPU-side concurrency is limited to about four to five jobs before swap.
- LOKI-10X's throughput figure (7.4-9.7x at measured core counts) is an arithmetic model, not a
  measurement. Treat it as a hypothesis.
- Pipelonk's segmentation and executor is the published pattern (segments sized by memory). BatchZK's
  multi-stream overlap is the batch analogue, for a different workload.
- Byte equality needs a fixed worker count per process (`loki-lean/SPEC.md` line 26), so the
  scheduler must pin the thread count per job class. `scheduler.qnt` already models ordering.
- This does not reduce k20 latency. It is a throughput lever and should follow ranks 1-2.

### E. sppark's state on Blackwell (sm_120)

- Pinned source is sppark 0.1.16, the latest tag. Apache-2.0. Last push 2026-10-20 (GitHub API).
- Its `src/build.rs` has an explicit sm_120 branch: `-gencode arch=compute_120,code=sm_120`, gated
  on nvcc support (lines 56-58). sm_100 is built as PTX only (lines 52-55). The upstream README says
  "Volta+ GPU hardware" and lists no architectures.
- Loki does not use sppark's `build.rs` for its MSM. Its own `build.rs` compiles `cuda/msm.cu` with
  `-gencode arch=compute_120,code=[sm_120,compute_120]`. That embeds PTX as well as SASS, so it can
  run on later architectures. `LOKI-GPU-MSM.md` records `msm.sm_120.cubin` on the 5090 with CUDA 13.4.
- A GitHub issue search for sm_120 or Blackwell in `supranational/sppark` returned nothing. This is
  not proof of absence, because the search is shallow.
- Known sppark pitfalls are already handled: BLST ADX symbol selection (`build.rs`), the destructor
  `noexcept` shadow patch, and the static CUDA runtime link.
- Not tested: multi-GPU, other architectures, and sustained VRAM pressure near 32 GB.

### F. CPU baseline: Rayon tuning, NUMA and the BLST threads issue

Host facts:

- Intel Core Ultra 9 285K. WSL is set to `processors=20` (`.wslconfig`), and `lscpu` shows 20 CPUs,
  one thread per core, and one NUMA node. NUMA tuning does nothing on this host. Affinity
  (`taskset`) is the only pinning lever from inside WSL.
- `/sys/devices/system/cpu/cpu*/cpu_capacity` reads 1024 for cpu0 and cpu8, so the performance and
  efficiency core split is not visible inside WSL. Check it from Windows before choosing a pin set.

Rayon:

- Loki's MSM chunks on the active Rayon pool with `chunk = max(1024, n / current_num_threads)`
  (`src/poly/kzg/msm.rs` lines 201-215). Nested commitments share the same pool.
- Byte reproducibility depends on the thread count (`loki-lean/SPEC.md` line 26), so every benchmark
  must print `RAYON_NUM_THREADS` and affinity. The `gpu_benchmark` log does not record either.

BLST threads issue:

- Stock `midnight-curves` 0.3.1 depends on blst with default features (its `Cargo.toml` lines 84-86).
  blst's default thread pool (`da_pool`, `blst-0.3.17/src/lib.rs` lines 35-54, used around line 181)
  runs outside Rayon. Stock can therefore have two pools competing for 20 CPUs. Whether stock's G1
  path reaches `da_pool` has not been traced.
- Loki sets blst `no-threads` (`Cargo.toml` lines 126-128) and runs serial Pippenger chunks on the
  Rayon pool. This already removes the second pool in the fork. Any stock-versus-Loki comparison
  must say so.
- The `LOKI-CPU.md` benchmark used `RAYON_NUM_THREADS=4` with `taskset -c 0-3`. The `gpu_benchmark`
  function (`src/poly/kzg/msm/cuda/tests.rs` lines 108-135) builds no pool and sets no thread count,
  so its CPU reference runs on the global Rayon default, which is 20 threads on this host unless
  `RAYON_NUM_THREADS` was exported. The log (`target/gpu-runs/20261024T175244-w4-msm-benchmark.log`
  in `loki-wt/w4-gpu-msm`) does not record it.

### G. Why k20 is 0% net: the arithmetic and the hypotheses

Facts that bound the explanation:

1. Warm GPU MSM at 2^20 is 57.7 ms, and cold is 247 ms. Both are faster than the 759 ms CPU figure at
   the same size. The 307 ms cold figure at 2^16 includes CUDA context start-up and is not
   representative.
2. The warm path includes an O(n) coordinate comparison on every hit (`backend.rs`
   `same_coordinates`), plus host-side scalar conversion and output validation on the calling thread.
3. Each MSM blocks its caller, and the mutex serialises all calls. CPU phases never overlap GPU MSM.
4. The cache has two slots keyed by `(pointer, length)`. A proof issues MSMs of several lengths against
   the same SRS pointer, so misses are likely. Their size and count are not logged.

Arithmetic, assuming the k20 parent resembles the k16 profile at the larger size: 14 MSMs at 2^20
and 17 FFTs. The MSM count is from the k16 profile, not from k20, so this is a proxy.

| Thread count of the 759 ms CPU reference | CPU MSM share of 220 s (14 calls) | GPU saving if warm (58 ms each) | GPU saving if cold (247 ms each) | Predicted net |
|---|---|---|---|---|
| 4 threads (759 ms is a 4-thread figure) | 14 x 759 ms = 10.6 s, about 4.8% | about 10 s, about 4.5% | about 7.2 s, about 3.3% | 3-5%, within noise of a 220 s run |
| 20 threads (default; the likely case for `gpu_benchmark`) | 759 ms x 20/4 at 70-100% efficiency = 3.0-3.8 s per MSM, so 42-53 s, about 19-24% | about 41-52 s, about 19-24% | about 38-49 s, about 17-22% | about 17-24% |

NTT, for either row: 17 transforms at about 122-142 ms CPU (base domain, k20) and 46-50 ms GPU give
at most about 1-2 s, under 1% of 220 s. The k20 NTT CPU cost is higher on extended domains
(245 ms for `extended_to_lagrange`), so this is a floor on the saving, not a ceiling.

Reading the table. If the 759 ms was a 4-thread number, the 0% result is inside noise of the
expected ceiling, about 5%, and Amdahl explains it. If it was a 20-thread number, the same arithmetic
predicts about 20% and the 0% result cannot come from Amdahl. Under that reading, something in the
run prevented the GPU work, or it was paid back in host overhead. Only the log can distinguish the
two. The k20 parent may also issue more or wider MSMs than the k16 synthetic, which the rank-1
profile would confirm.

Hypotheses, in order to test:

- H0 (silent fallback, section prerequisite). The backend disabled itself after one error, the
  `cuda` feature or env was absent, or the run used a build without the change. Check first.
- H1 (Amdahl, 4-thread reading). The MSM and NTT share is about 5% of k20 wall time. Nothing more is
  needed beyond ranks 1-2.
- H2 (host overhead, 20-thread reading). Per-call host work (compare, conversion, validation) eats
  most of the MSM saving at the sizes k20 uses.
- H3 (cache thrash). Misses and re-uploads dominate many calls. Cold cost alone still predicts about
  17% saving, so H3 must combine with H2 to explain 0%.

Test, in this order:

1. Read the k20 run log for the one-time GPU warning, and check the feature and env (H0).
2. Run the CPU reference at the proof's thread count, and print the thread count in
   `gpu_benchmark` (fixes the ambiguity in section F).
3. Run `LOKI_PROFILE` (feature `loki-profile`) on the k20 parent, with `LOKI_GPU_MSM_MIN` at 0 and at
   an effectively infinite value. The two runs give the MSM share directly.
4. Add per-call counters in `backend.rs`: size, hit or miss, host ms, GPU ms, mutex-wait ms.
5. Run `nsys profile` on one k20 run to separate kernel time from upload and host time.

**Inconsistent baseline to resolve before anything else.** The brief says 220 s to prove on 4 threads.
`LOKI-10X.md` records 882.6 s proving and 62.3 s proving-key setup for the same k20 parent. These
cannot both be the same measurement. The conclusions above depend on which one applies, so confirm
the source of each figure first.

## Limits and what would have to be true

- The ranked list is a judgement from published evidence and local code. None of the five has a
  measured Loki gain yet.
- Rank 1 gains only if the default Rayon count is wrong for this hybrid host, and it may be neutral.
- Rank 2 gains a few percent per MSM. It matters more if H2 or H3 is confirmed.
- Rank 3 pays off if MSM bases dominate the proof's MSM time and the memory cost fits alongside the
  two-slot cache. Its gain is unmeasured for Loki.
- Rank 4 pays off only if CPU work independent of the MSM is at least comparable to the GPU MSM time.
  The k16 profile suggests this is limited.
- Rank 5 pays off only if the quotient and multiopen polynomial work become a larger share at k20
  than at k16. The k16 profile says quotient evaluation plus construction is 21.8% of wall time.
- Throughput scheduling (section D) does not change k20 latency. It needs the VRAM admission check
  and the fixed-thread rule.
- Nothing here changes the proof format or needs new verifying keys. All recommendations are Track A.
- Measurements in this note are Loki's own, with the caveat that k16 is synthetic. Everything marked
  as an estimate or as a claim is unverified on this host.

## Sources

- Loki (`/home/charles/repos/loki-proofs`): `LOKI-GPU-MSM.md`, `LOKI-GPU-NTT.md`, `LOKI-CPU.md`,
  `LOKI-PROFILE.md`, `cuda/msm.cu`, `build.rs`, `Cargo.toml`, `src/poly/kzg/msm.rs`,
  `src/poly/kzg/msm/cuda/backend.rs`, `src/poly/kzg/msm/cuda/tests.rs`, `cuda-ntt/src/lib.rs`,
  `cuda-ntt/cuda/ntt.cu`.
- Lean contract: `/home/charles/repos/loki-lean/SPEC.md` (lines 3, 16-40).
- Moriarty: `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/LOKI-10X.md`.
- sppark 0.1.16 source: `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sppark-0.1.16/`
  (`src/build.rs`, `sppark/util/gpu_t.cuh`).
- blst 0.3.17 source: `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blst-0.3.17/src/lib.rs`.
- Raw captures: `/home/charles/repos/Moriarty-factory/wiki-llm/loki/speedups/raw/gpu-system/`
  (Nocy README and `native-plonk` sources).
- Nocy GPU proof server: https://github.com/Nocy-io/nocy-gpu-proof-server ;
  https://forum.midnight.network/t/open-source-drop-cuda-accelerated-midnight-proof-server-nocy-gpu-proof-server/972
- midnight-zk PR #157: https://github.com/midnightntwrk/midnight-zk/pull/157
- Pipelonk (PPoPP 2026): https://ppopp26.sigplan.org/details/PPoPP-2026-papers/38/Pipelonk-Accelerating-End-to-End-Zero-Knowledge-Proof-Generation-on-GPUs-for-PLONK-B ;
  https://doi.org/10.1145/3774934.3786448
- ZKProphet: https://arxiv.org/abs/2509.22684
- cuZK: https://tches.iacr.org/index.php/TCHES/article/view/10961 ; https://eprint.iacr.org/2022/1321
- GZKP (ASPLOS 2023): https://elmi.hbku.edu.qa/en/publications/gzkp-a-gpu-accelerated-zero-knowledge-proof-system/
- BatchZK: https://eprint.iacr.org/2024/1862
- Elastic MSM: https://eprint.iacr.org/2024/057 ; https://tches.iacr.org/index.php/TCHES/article/view/11791
- EZKL Metal MSM: https://hackmd.io/@guard/BJDmuEODke
- Snarkify: https://lu.ma/fz3x308c
- ICICLE: https://github.com/ingonyama-zk/icicle (no halo2 repository found)
- sppark: https://github.com/supranational/sppark
- NVIDIA forums (WSL2 pinned memory): https://forums.developer.nvidia.com/t/cudahostregister-not-supported-on-wsl2/279429 ;
  https://forums.developer.nvidia.com/t/change-limit-of-50-for-cudahostalloc-pinned-memory-on-windows-10-11/228235/8
