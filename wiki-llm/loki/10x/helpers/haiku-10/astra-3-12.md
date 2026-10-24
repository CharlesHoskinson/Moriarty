# astra-3-12: Review of astra-3/PROPOSAL.md (memory, CUDA, WSL, Nocy claims and capture paths)

## Question
Review `loki/10x/astra-3/PROPOSAL.md` for memory, CUDA, WSL and Nocy claims and for its source-capture paths. Establish correct device-vs-CPU concurrency limits and flag any capacity claim without evidence. Lead file not edited.

## Answer
- **Overall:** the arithmetic and most citations check out. The verdict is too strong. "10x throughput is technically plausible" rests on an assumed 25% CPU / 75% GPU-accelerated-10x split with no source support. The 10.45x result is a conditional model output, not a capacity proof. Recommend restating the verdict as "conditional on measured CPU remainder and bulk speedup; no measured GPU result supports it."
- **Verified (exact match or recomputed):**
  - Historical stage sums: leaves 34.779+35.296+44.186+49.369 = 163.631 s; node 867.280 s; decider 586.382 s; total 1617.292 s (`recl9/out/*/stats.json`).
  - PK setup 157.751 s = leaves keygen 6.396 + node 104.463 + decider 46.892 (`keygen_pk_s` in each stats file).
  - Historical CPU use: node 2534.17+81.31 user+sys over 16:13.73 (973.73 s), 268% CPU = 2.68 cores; decider 2486.95+59.80 over 10:34.85 (634.85 s), 401% = 4.01 cores (`recl9/logs/node-prove.log`, `decider-prove.log`).
  - Ledger proof server: 2 default workers (`proof-server/src/main.rs:44`, env `MIDNIGHT_PROOF_SERVER_NUM_WORKERS`) and `spawn_blocking` (`worker_pool.rs:236`).
  - Decoded PK LRU of five (`transient-crypto/src/proofs.rs:250`).
  - Lookup HashMap iteration (`plonk/lookup/prover.rs:399,430`) and vanishing RNG chunking by Rayon thread count (`plonk/vanishing/prover.rs:45-55`).
  - PR #480 merge commit `31435e9` exists in `~/repos/midnight-zk`.
  - `RUN-REPORT.json` children: A102 `circuit_k` 13, A80 `circuit_k` 12, `proof_bytes` 3440 each, `instance_lanes` 253.
  - Queue model (`queue_model_results.json`): 4-worker case 154.705 s, 10.454x; 3 workers 8.819x; bulk 8x 8.940x; CPU 0.35 with bulk 10x 9.313x; 1 active batch 508.297 s (3.182x); 2 active 282.001 s (5.735x). All match the PROPOSAL.
  - Arithmetic: 1617.292/10 = 161.73 s; (62.328+882.590)/882.590 = 1.0706; 1775.043/1617.292 = 1.0975; 2.98507 x 3.50211 = 10.454; isolated ceiling with f=0.25, overhead 0.01 = 1/0.26 = 3.846; GPU-only ceiling 1/0.075 = 13.33; (2615.48+2546.75)/20 = 258.11 s; 1775.043/258.11 = 6.88; k19 domain rows 4x2^17 + 2x2^20 = 5x2^19 (5x proxy).
  - B7 text `qa/sherlock/predicates.yaml:154-158` (peak under decimal 16 GB, swap 0). The 64 GiB experiment cannot pass B7.
  - WSL: the "concurrent CPU/GPU access is not supported" sentence sits in the managed-memory row of the WSL guide (`raw/wsl-guide.md`). The PROPOSAL's reading is correct.
  - Nocy: README 12.3/4.4 s (k17) and 24.6/8.8 s (k18); per-commit `cudaStreamSynchronize` and one commitment per call in `kzg.h` (astra's `raw/nocy-kzg.md` contains the commit path); per-point windowed scalar multiplication then reduction in `msm.cuh` (`g1_scalar_mul` at lines 104-131 of the repo-HEAD file; astra's `raw/nocy-msm.md` captures only the loop body, ~lines 105-107).
- **Corrections required:**
  1. **RUN-REPORT scope.** The report's `qualification` field reads: "scoped A102 Eval(Literal)->Return + A80 Return->StepDone producer/consumer run; not parent recursion, terminal validity, ledger acceptance or full-39 coverage." The k13/k12 children are a scoped fixture, not the full A102/A80 Sherlock chain. The PROPOSAL's "actual bounded Sherlock children at k13 and k12" must say so.
  2. **Fixed-pair budget baseline.** The "<=88.259 s" threshold is valid only as a lower-bound argument: old full chain >= old parent alone (882.590 s), so a new terminal <=88.259 s gives >=10x only if both measure the same statement, witness scope and setup/key inclusion. The new route's own key creation and independent verification must be inside the 88.259 s budget or reported separately. The k15 capacity arithmetic (`12274 + 8192 = 20466 < 32761`) is a design budget, not a row measurement, as the PROPOSAL states for the k20 analogue; keep that caveat.
  3. **Verdict and bulk assumption.** Add: a 10x isolated service result requires the CPU remainder to be at most about 9% of serial service (ceiling 1/(f+0.01) >= 10 needs f <= 0.09), before any pipelining. The 25% remainder is an assumption; no measured CPU fraction exists. Historical CPU use of 2.7-4 effective cores of 20 means the serial denominator probably leaves cores idle, so its service time depends on thread policy, which the PROPOSAL already flags.
  4. **Nocy wording.** "Nocy 2.8x is neither a Sherlock result nor a hardware ceiling" is correct. Also say that Nocy's MSM is per-point double-and-add (not Pippenger), so an op-count comparison suggests MSM kernel headroom. This is an estimate, not a measured ratio; do not cite a 15-20x count as runtime.
  5. **Capture paths.** `raw/nocy-msm.md` is a partial capture (139 lines; loop body only). It lacks the `g1_scalar_mul` header and the reduction kernel, which lives in `msm.cu` and is not captured. Capture `msm.cu` as well, or cite the repo HEAD commit (`366d3c72e814691cd16bbd72e77b5169d07f729c`). The `raw.githubusercontent.com` URLs the PROPOSAL uses are the right form. GitHub blob pages captured via Scrapling are page chrome, not code (confirmed for `msm.cuh` and `mn_prover.h` in helper-10's own captures).
  6. **Not re-verified here:** `BLST has its own pool (R/blst-0.3.17/src/lib.rs:41)` and `F/REPORT.md:191`.
- **Device vs CPU concurrency (what the evidence supports):**
  - **Device:** one CUDA context per process per device. Kernels and copies on different non-default streams can overlap; overlapping kernels needs SM headroom, and truly asynchronous copies need pinned host memory. WSL2: pinned memory is limited with no numeric cap; managed-memory oversubscription and concurrent CPU/GPU access to managed memory are unsupported. Explicit `cudaMalloc` with bounded pinned staging is the supported path.
  - **Pageable copies block.** Nocy's `cudaMemcpyAsync` of scalars from a `std::vector` (pageable memory) is staged through the host, so the calling thread can block on it. A pinned ring is required for copy/compute overlap.
  - **CPU:** 20 cores shared across witness workers, Rayon, BLST's own pool, verification and the GPU submit thread. Concurrency is one budget, not separate pools each assuming 20 cores.
  - **Device-CPU overlap:** host work proceeds while a GPU stream runs, provided the host thread does not block on a synchronize or a pageable copy. GPU completion must not depend on a Rayon worker blocked waiting for it (PROPOSAL section 3 already says so).
  - **Multiple processes on one GPU** time-slice unless MPS is used. The PROPOSAL does not address this; state it as a constraint. No MPS evidence was gathered here.
- **Memory capacity (cross-check with astra-3-9):** per-key PK payload 8.8125 GiB (coset-only 6.00 GiB); two full decoded keys 17.625 GiB; per-proof workspace, listed components only (4.28 GiB; incomplete: excludes MSM scratch, rational representations and overlapping temporaries); B7 bound 14.90 GiB; device 24 GiB budget fits one heavy job plus leaves, and two resident key sets only with per-proof peak <=4.8 GiB. The PROPOSAL's figures agree. The host currently shows 8 GB of swap in use (`raw/host-snapshot.txt`), relevant to any swap-0 claim.

## Raw numbers
| Item | Value | Source |
|---|---|---|
| Historical serial prove sum | 1617.292 s | leaves, node, decider stats |
| PK setup sum | 157.751 s | `keygen_pk_s`, three stats files |
| Node / decider effective cores | 2.68 / 4.01 | logs (`%CPU` 268%, 401%) |
| 4-worker model interval | 154.705 s (10.454x) | `queue_model_results.json` |
| A102 / A80 circuit_k | 13 / 12 (scoped fixture) | `RUN-REPORT.json` |
| B7 | < 16 GB decimal, swap 0 | `predicates.yaml:154-158` |
| Host swap in use | 8 GB of 32 GB | `raw/host-snapshot.txt` |

## Caveats
- The queue model is a deterministic simulation with assumed CPU fractions and perfect bulk speedups; it omits contention and measured memory. Its outputs are ESTIMATE-class.
- Historical CPU-core figures include PK setup and come from unknown host and cgroup configurations.
- No builds, runs or benchmarks were done; this is source reading, arithmetic, and reruns of the model file's stored outputs.

## Sources
- `loki/10x/astra-3/PROPOSAL.md`; `astra-3/queue_model_results.json`; `astra-3/raw/nocy-kzg.md`, `nocy-msm.md`, `nvidia-wsl.md`
- `recl9/out/*/*.stats.json`, `recl9/logs/*-prove.log` (historical)
- `docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`
- `qa/sherlock/predicates.yaml:154-158`
- `helpers/haiku-10/raw/wsl-guide.md`, `nocy-native-plonk_cuda_src_kzg.h.txt`, `nocy-native-plonk_cuda_src_msm.cuh.txt`, `host-snapshot.txt`
- midnight-ledger `transient-crypto/src/proofs.rs:250`; `proof-server/src/main.rs:44`, `worker_pool.rs:236`; midnight-proofs 0.8.3 `plonk/lookup/prover.rs:399,430`, `plonk/vanishing/prover.rs:45-55`
