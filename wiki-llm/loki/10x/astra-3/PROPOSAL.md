---
title: "Loki 10x: throughput from bounded concurrency, latency from less recursive work"
project: loki-10x
author: gpt-6-astra
date: 2026-10-24
sources:
  - ../../LOKI-DESIGN.md
  - ../../nodes/
  - /home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/
  - /home/charles/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/
  - /home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/
  - /home/charles/repos/midnight-zk/
  - /home/charles/repos/midnight-ledger/
  - https://docs.nvidia.com/cuda/wsl-user-guide/index.html
  - https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html
  - https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html
  - https://developer.nvidia.com/cuda/gpus
  - https://dev.ingonyama.com/3.0.0/icicle/primitives/msm
  - https://github.com/Nocy-io/nocy-gpu-proof-server
  - https://github.com/midnightntwrk/midnight-zk/pull/480
  - https://github.com/midnightntwrk/midnight-zk/pull/157
---

# Target definition

**PROPOSAL —** Target **(c), sustained completed 16-transaction batches/hour**, on the supplied 20-core/90-GB/RTX-5090 host. Count a batch only after its terminal proof passes independent, real ledger-9 verification. Measure steady intervals with a bounded backlog; separately report first-batch latency, queueing, cold initialization, and stagenet inclusion/readback. The denominator must be the best measured CPU schedule under the same resource limit and semantics, not an artificially serial implementation. Demonstrate stagenet acceptance of the exact final proof family; offline verifier throughput alone is not on-chain settlement throughput.

**PROPOSAL —** Secondary target **(b)** is witness-input-to-verified-terminal latency for the actual A102→A80 Sherlock chain. A replacement relation under new keys may achieve this target if it proves the same intended transition and ledger statement. Removing the k20 parent is not a 10× improvement in **(a), that parent's own latency**. Report those outcomes separately.

**CODE —** Sherlock's heterogeneous two-child relation is distinct from Fenrir's four four-transaction leaves→four-child node→decider. Fenrir's leaf holds every spender's secret key and explicitly limits that arrangement to the synthetic strawman. Its timings cannot establish production distributed-wallet or full Sherlock-machine throughput. See [Sherlock relation](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:3) and [leaf trust boundary](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/leaf.rs:7).

**CODE —** For compact source references below, `R` is `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f`; `P=R/midnight-proofs-0.8.3`, `C=R/midnight-curves-0.3.1`, `Z=R/midnight-zk-stdlib-2.3.5`; `F=/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9`, `M=/home/charles/repos/Moriarty-factory`, `L=/home/charles/repos/midnight-ledger`. CODE includes observations retained in local artifacts; ESTIMATE means arithmetic or a model, never a new prover measurement.

# Verdict

**ESTIMATE —** **10× throughput is technically plausible, but unproved; confidence is low for the present workload and medium that the proposed experiment can decide it.** Four CPU witness workers feeding one GPU can beat a two-stage overlap ceiling when an individual witness job leaves cores idle. A reproducible queue model reaches **10.45×** against a historical serial reference while isolated serial service improves only **2.99×**. That narrow result disappears with modest contention or a better CPU denominator. A stronger eight-worker/20×-bulk scenario gives **10.47× against a hypothetical CPU baseline twice as fast as serial**, with only 3.36× isolated service improvement. Neither scenario establishes its assumptions; these are research targets, not performance forecasts.

**ESTIMATE —** **Accounted setup amortization alone does not reach 10×.** The recorded Sherlock parent has 62.328385318 s PK setup and 882.589555363 s proving **including explicit PK destruction**. Removing the separately timed setup gives `(62.328+882.590)/882.590=1.07062×`; additional destruction savings are unmeasured. The brief's censored >500 s run cannot supply a completed baseline. If its >500 s excludes a recurring 120 s setup, caching gives <1.24×; if it includes that setup, the conditional bound is <1.316×. Neither identifies what the 120 s timer actually covered. [Receipt explanation](/home/charles/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/README.md:5).

**ESTIMATE —** For **(b)**, scheduling different batches cannot shorten the parent→decider dependency. In the historical Fenrir chain, making only its arity-four node 10× faster improves total proving by **1.933×**; even a zero-time node has a **2.156×** ceiling with other stages unchanged. A broader prover rewrite has no established 2–4× mathematical ceiling. With an unchanged 25% CPU remainder and 1% added overhead, however, even infinite GPU speed caps isolated service at `1/.26=3.846×`. New keys that remove unnecessary recursive work are the stronger latency route.

# The plan

## 1. Establish a baseline that can support the claim

**CODE —** Historical Fenrir artifacts give leaf proving sum **163.630576714 s**, node **867.279750364 s**, decider **586.381689547 s**: **1,617.292016625 s** total proving. PK setup adds **157.751410713 s**, separately from VK generation, parameter loading and verification. These are sums of stage observations, not a measured warm steady-state run. Sources: `F/out/leaves-b1/leaves.stats.json`, `F/out/node-b1/w.stats.json`, `F/out/decider-b1/batch-1.stats.json`; sizing and acceptance are in [REPORT](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:102).

**CODE —** Historical node invocation CPU time is `2534.17+81.31=2615.48` seconds over 973.73 wall seconds; decider is `2486.95+59.80=2546.75` over 634.85: approximately **2.69 and 4.01 effective cores**. These include PK setup and other work, not isolated synthesis. Historical CPU allocation is not established by today's 20-core host. They motivate concurrency measurement, not a guaranteed scaling factor. [Node log](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/logs/node-prove.log:5), [decider log](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/logs/decider-prove.log:5).

**CODE —** Version reconciliation is mandatory. Moriarty pins proofs 0.8.3; recl9 declares 0.8.2 and patched circuits. The inspected 0.8.2 dispatch uses generic MSM above `2<<18` **nonzero scalars**, whereas 0.8.3 always uses BLST for BLS G1 (`P/src/poly/kzg/msm.rs:176`; corresponding 0.8.2 file:178). Historical timings therefore cannot forecast the actual pinned prover. The parent receipt also needs binary/dependency provenance before serving as a matched denominator. Upstream PR #480 merged the in-circuit PCS refactor at `31435e9`; its `commit_many` still maps individual MSMs. It supplies an API precedent, not a compatible upgrade. [PR #480](https://github.com/midnightntwrk/midnight-zk/pull/480), [Scrapling capture](raw/midnight-pr480.md).

**PROPOSAL —** First freeze binary/source/features, exact relation, keys, SRS, transcript, row/table/column inventory, input corpus and thread policy. Instrument exclusive phase spans: cost-model probing, PK creation/read/drop, real witness synthesis, rational conversion, lookup sort/scan, MSM, NTT, quotient, multiopen, transfers, verification and queue waits. Record per-phase wall and core-seconds; synchronize GPU timing at phase boundaries and collect allocator high-water independently of NVML. Never sum nested timers. Compare cold, decoded-resident CPU, and decoded-resident GPU runs; sweep CPU concurrency before selecting the baseline.

## 2. Retain prepared keys, not just serialized files

**PROPOSAL —** Introduce a persistent `ProverContext` with single-flight loading, immutable shared keys, per-domain SRS bases, and byte-budgeted eviction. Identity includes relation/configuration bytes, child VKs, source/features, architecture, domain, transcript and authenticated SRS digest. Cache the exact-degree validation result only after enrollment against the expected relation and VK; keep per-request statement checks.

| Label | Existing hook | Concrete change | New keys? |
|---|---|---|---|
| CODE / PROPOSAL | `F/src/sherlock_link.rs:1029,1041,1044` | Cache degree validation and resident PK; remove repeated preparation/destruction from the job lifetime. | No |
| CODE / PROPOSAL | `M/crates/mori-backend/src/sherlock_child.rs:388`; `src/sherlock_link.rs:736` | Reuse prepared child contexts. | No |
| CODE / PROPOSAL | `F/src/main.rs:349,462,598` | Preserve existing within-batch leaf reuse; extend node/decider reuse across batches. | No |
| CODE / PROPOSAL | `P/src/plonk/mod.rs:416` | Avoid repeated decoded-key reconstruction; serialized reads rebuild coefficient/coset transforms. | No |
| CODE / PROPOSAL | `P/src/poly/kzg/params.rs:40` | Cache each downsized Lagrange basis separately; it is recomputed, not a prefix. | No |

**CODE —** The ledger proof server already has a five-entry decoded-PK LRU (`L/transient-crypto/src/proofs.rs:250`). It already uses `spawn_blocking` and defaults to two workers (`L/proof-server/src/worker_pool.rs:236`, `src/main.rs:44`). Replace count-only cache/admission with measured byte budgets where needed. Do not claim HTTP, WASM or moving work off the async executor as savings for Sherlock's native CLI without a trace showing that path.

## 3. Several host jobs, one CUDA context, bounded GPU residency

**PROPOSAL —** Build one native scheduler inside one factory `heavy` invocation. It owns one CUDA context, a bounded ready queue, shared key leases and separate CPU/GPU/control executors. Start with one heavy job; expand to four CPU witness workers only after service measurements. Keep chip state and `MidnightCircuit` on their creating worker: they contain `Rc<RefCell<…>>` (`Z/src/lib.rs:1529`; circuits `src/field/native/native_chip.rs:117`). Send owned field buffers and job identifiers across the GPU boundary, not live layouters.

**CODE / PROPOSAL —** Compile-check shared key `Send+Sync` bounds; a relation embedded in `MidnightPK<R>` may require separation from the immutable polynomial key. The existing `compute_trace`→`finalise_proof` split already mixes CPU work, NTTs and commitments (`P/src/plonk/prover.rs:371,515,568`); it is not the model's clean CPU→GPU boundary. Implement bounded submissions at arithmetic boundaries and measure the resulting repeated handoffs.

**PROPOSAL —** Reserve CPU work against the complete 20-core allowance, including Rayon, BLST and verification. Four workers are not four unrestricted global pools: BLST has its own pool (`R/blst-0.3.17/src/lib.rs:41`). GPU completion must not depend on a Rayon worker blocked awaiting that completion. Treat the assumed two effective cores per witness worker in the model as a quantity to measure, not an implemented thread cap.

**PROPOSAL —** Within a proof, batch independent commitments in `P/src/plonk/prover.rs:568` and preserve column/transcript order; route coefficient and Lagrange commitments through `P/src/poly/kzg/mod.rs:74,83`, including multiopen's internal commitments. Then retain polynomial data through `P/src/poly/domain.rs:251`, `plonk/prover.rs:595`, `plonk/evaluation.rs:285` and vanishing construction. Sorting, scans and witness work remain explicit costs until accelerated. The `create_proof` circuits slice shares one transcript and is not a drop-in batching API for independent ledger proofs.

**SOURCE / PROPOSAL —** ICICLE documents shared-base MSM batches and precomputation, supporting this execution pattern. Batch compatible arithmetic across jobs without merging proof transcripts. Count preparation, launch amortization and avoided transfers once. Break-even reuse is `m > T_prepare/(t_plain−t_prepared)`; no measured preparation time exists here. [MSM API](https://dev.ingonyama.com/3.0.0/icicle/primitives/msm), [capture](raw/icicle-msm.md).

**SOURCE —** Nocy reports 12.3→4.4 s at k17 and 24.6→8.8 s at k18 on RTX 5090 versus EPYC 7642, without sufficient timing-boundary detail for this comparison. Its inspected KZG code caches bases but synchronizes each commitment; its MSM computes per-point windowed scalar multiplications followed by reduction. Thus 2.8× is neither a Sherlock result nor a hardware ceiling. [README](https://github.com/Nocy-io/nocy-gpu-proof-server), [capture](raw/nocy.md); [KZG code](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/native-plonk/cuda/src/kzg.h), [capture](raw/nocy-kzg.md); [MSM code](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/native-plonk/cuda/src/msm.cuh), [capture](raw/nocy-msm.md); [reduction at inspected commit](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/366d3c72e814691cd16bbd72e77b5169d07f729c/native-plonk/cuda/src/msm.cu), [capture](raw/nocy-msm-kernels.md).

**PROPOSAL —** Prioritize ready terminal/parent jobs while aging leaves. Parents wait for their own required, independently verified child artifacts. Overlap only independent batches or valid speculative successors whose deterministic state is already known; preserve settlement order and invalidate descendants after a predecessor mismatch. Checkpoint completed verified proofs atomically. Restart interrupted proofs with fresh randomness; do not persist unfinished transcripts or private witnesses as public evidence.

## 4. Make memory and WSL limits part of admission

**ESTIMATE —** At k20 a field vector occupies 32 MiB; a k22 extended vector occupies 128 MiB. With historical 27 fixed plus 18 permutation columns and assumed extension factor four, retained PK payload is `32 MiB×[(27+18)×6+12]=8.8125 GiB` per heavy shape. The reported 45 already includes permutation columns (`P/src/dev/cost_model.rs:244`); both polynomial families retain base, coefficient and coset forms (`plonk/mod.rs:338`, `plonk/permutation.rs:139`). Two full decoded keys alone consume **17.625 GiB** before witness workspaces. Sharing keys helps greatly, but persistent caching can increase the live memory footprint across different shapes.

**PROPOSAL —** The initial unrestricted-host experiment permits at most 64 GiB, reduced by actual available memory and safety margin, and device allocations at most `min(24 GiB, .8×free_device_bytes)`. Admit one heavy GPU workspace; initially stage at most two 64-MiB pinned buffers. Four host jobs do not imply four GPU-resident proofs. Record incremental workspace size, shared cache bytes and queued buffer bytes separately before enabling the five-host-slot model.

**CODE / PROPOSAL —** Existing factory **B7 requires peak below decimal 16 GB and swap zero** (`M/qa/sherlock/predicates.yaml:154`). The 64-GiB experiment cannot pass B7. A later memory-saving backend could retain only base/coefficient key polynomials on host and reconstruct/upload cosets by phase; that changes representation, not VKs. Its transfer cost and joint memory peaks must be measured. Do not add two independent 12-GiB proof estimates or assume host RAM extends VRAM.

**SOURCE / PROPOSAL —** NVIDIA documents limited pinned memory and incomplete managed-memory support under WSL; the concurrent-access restriction concerns managed memory, not separate CPU work overlapping explicitly allocated GPU buffers. Use bounded explicit allocations, events and separate streams. Build/validate `sm_120` plus appropriate PTX; do not infer field-arithmetic performance from Tensor/FP4 specifications. [WSL guide](https://docs.nvidia.com/cuda/wsl-user-guide/index.html), [capture](raw/nvidia-wsl.md); [CUDA practices](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html), [capture](raw/cuda-best-practices.md); [Blackwell compatibility](https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html), [capture](raw/nvidia-blackwell.md); [GPU capabilities](https://developer.nvidia.com/cuda/gpus), [capture](raw/nvidia-capabilities.md).

## 5. Price a new-key route against continued optimization

**CODE / PROPOSAL —** Astra-2 found the bounded Literal→Return→StepDone children at k13 and k12 in `M/docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json:1`, not the full Sherlock machine. For this same-witness fixed pair, build one direct relation enforcing both transitions, program/context pins, state linkage, charges, ordered trace and terminal statement/communication binding. Reuse emitters `M/crates/mori-backend/src/sherlock_link.rs:390,444` and `src/sherlock_machine/literal_bridge.rs:69,132`, plus terminal binding in `F/src/decider.rs:193`. Host-only checks at `M/crates/mori-backend/src/sherlock_link.rs:984` must become constraints. This removes proof-verification work rather than accelerating a k20 parent.

**ESTIMATE —** A new terminal taking **≤88.259 s**, including witness processing and independent verification, would be at most one tenth of the recorded old parent interval alone. It implies a bounded-chain 10× result only if it replaces every old terminal obligation under matched workload, setup policy and lineage. This is a target, not a measurement. A k14–15 fit is unproved. For synthetic 16 transactions, four native slot groups plus the same composition/hash structure could similarly become a direct Blake2b terminal. A hypothetical k19 circuit reduces total domain rows from `4×2^17+2×2^20` to `2^19`, a **5× unweighted domain-row proxy**, not a time guarantee: leaves have nine advice columns versus fifteen in node/decider, and degree/table work differs. Table tags, columns, degree and terminal binding can defeat the fit.

**CODE —** Existing [mono cost evidence](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/mono/logs/cost.log:10) strengthens that geometry hypothesis: sixteen synthetic slots use **449,169 advice rows, 360,707 table rows, k19, nine advice columns**. Its 35.195-second honest check is MockProver, not proving time. The [relation](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/mono/src/mono.rs:273) leaves `CommunicationCommitment` unconstrained beyond public assignment, so it is not already a qualified Sherlock terminal. No completed sixteen-slot proof was located.

**CODE —** New contract keys do not authorize a new outer proof protocol. The stock reader reconstructs `ZkStdLib::configure(architecture,k−1)` (`Z/src/lib.rs:1593`), and the ledger uses Blake2b/stdlib verification (`L/transient-crypto/src/proofs.rs:64,545`). Retain that outer protocol and SRS. An arbitrary custom gate, LogUp or foreign PCS requires a compatible outer wrapper; merely installing a VK does not add those capabilities.

**ESTIMATE / PROPOSAL —** Budget roughly 2–5 engineering days for profiling/resident-context work, several weeks for a validated GPU polynomial backend and scheduler, and 2–4 engineer-weeks for a bounded direct-terminal experiment plus equivalence/adversarial qualification. These are planning allowances, not quotes. Stagenet key deployment is cheap by the brief; its exact fee is unmeasured. Charge new relation/key generation, independent verification, deployment/readback, client pin updates and state/replay migration once. Amortize actual cost `K_new/B` over B batches. General distributed-witness fusion has additional privacy/protocol work and no credible schedule here. No production secret-key centralization is proposed.

# Speedup budget

**ESTIMATE —** Let T be serial warm service and C, G, Q be CPU core-seconds, exclusive GPU seconds and serial control service per batch. Any interval satisfies `I≥max(C/available_cores,G,Q,transfer_service)` plus dependency/admission effects. If the CPU lane is already saturated, merely overlapping CPU and GPU improves service by at most `(CPU+GPU)/max(CPU,GPU)≤2`. Recovering idle CPU cores is a different mechanism. Never multiply a GPU kernel ratio by a guessed worker count.

| Label | Successive factor / scenario | Arithmetic and scope |
|---|---|---|
| CODE / ESTIMATE | Historical serial scale | T=1,617.292 s → 2.22594 batches/hour; 10× requires ≤161.729 s/output. Older cohort only. |
| ESTIMATE | Warm caching factor **1.000×** | Both sides resident. Setup-inclusive alternative is 1,775.043/1,617.292=1.09754×; exclude it from the warm product. |
| ESTIMATE | Whole arithmetic service **2.98507×** | Assume CPU fraction .25, remaining .75 accelerated 10×, added overhead .01: T′=.335T=541.793 s. Fractions are unmeasured. |
| ESTIMATE | Bounded pipeline **3.50211×** beyond T′ | Four CPU workers, five host slots, one GPU lane, six eligible batches: simulated I=154.705 s; 541.793/154.705. |
| ESTIMATE | Combined warm **10.45407×** | **1×2.98507×3.50211**; 23.2702 batches/hour. Not an observed speedup. |
| ESTIMATE | Sensitivity | Three CPU workers: 8.82×; bulk acceleration 8×: 8.94×; bulk3×: 3.81×; CPU remainder .35: 9.31×. |
| ESTIMATE | Dependence limit | One eligible batch: 508.297 s, 3.18×; two: 282.001 s, 5.74×. |
| ESTIMATE | Stronger, fair-baseline target | Eight CPU workers/nine host slots/eight eligible batches; bulk20×: I=77.221 s. If CPU baseline R=2, gain=20.94364/2=**10.47182×**. |

**ESTIMATE —** [Queue model](queue_model.py) and [results](queue_model_results.json) simulate the actual six-job dependency shape with assumed CPU→GPU→control phases. They impose slot limits and GPU serialization, but omit real multi-phase interleaving, bandwidth contention and measured workspace sizes. Its 10.45× margin tolerates only approximately **4.54%** slowdown; an improved CPU baseline can erase it. The model demonstrates feasibility under assumptions, not their truth. With these .25/.75/10 assumptions, GPU service alone imposes a **13.33×** throughput ceiling; the simulated scheduler realizes less.

**ESTIMATE —** Make the denominator challenge explicit: if tuned CPU concurrency improves the historical serial reference by R, this model's gain becomes **10.454/R**, with an ideal GPU-service ceiling **13.333/R**. At R=2, those are **5.227× and 6.667×**, respectively; 10× is impossible under the assumed 10× GPU arithmetic. Even infinite GPU speed leaves four CPU workers a ceiling `4/.25/R=8×` at R=2. Reaching 10× then needs fewer CPU seconds, more qualified CPU workers, or less work per batch—not more buffering. These are conditional ceilings; a current CPU baseline is the decisive missing number.

**ESTIMATE / PROPOSAL —** The stronger scenario shows a possible next target, not a recommendation to launch eight proofs now. Isolated serial service improves only `1/(.25+.75/20+.01)=3.361×`, yet modeled throughput beats a hypothetical doubled CPU baseline by 10.472×. Eight workers at the assumed two effective cores consume sixteen cores, leaving four for control/verification; actual core demand and nine-slot memory must qualify first. Six workers give only 9.387×, bulk16× gives 9.048×, and CPU remainder .35 gives 9.960× against that baseline. Broad 20× arithmetic acceleration remains wholly unmeasured.

**ESTIMATE —** Pure scheduling of unchanged historical node/decider invocations requires at least `(2615.48+2546.75)/20=258.11 s` host service before leaves. Against the approximate 1,775.043 s setup-inclusive sum, this gives a loose conditional upper bound of **6.88×**, assuming CPU work is unchanged and 20 cores are available. Including the unknown leaf CPU time only lowers that ceiling. This is not a warm/current-prover bound. On-chain throughput has the additional ceiling imposed by ledger inclusion/settlement capacity, which remains unmeasured.

# Risks and soundness

**PROPOSAL —** Preserve canonical field/point encodings, scalar bounds, subgroup checks, exact roots/cosets, rotations, lookup/permutation/trash identities, blinding and transcript order. Treat GPU failures transactionally. Independent verifier acceptance establishes arithmetic compatibility; it does not establish zero knowledge if randomness leaks or repeats, nor repair an underconstrained relation.

**CODE / PROPOSAL —** Parent preparation may accept a changed child statement with a bad deferred accumulator; the terminal must reject it (`F/REPORT.md:191`). Preserve terminal subgroup/pairing checks whenever recursive obligations remain. Direct fusion may eliminate that obligation only by constraining the underlying computation fully. Fixed RNG alone does not guarantee identical proof bytes: lookup leftovers iterate a randomized HashMap (`P/src/plonk/lookup/prover.rs:399,430`), vanishing randomness depends on Rayon width, and the foreign ECC chip independently samples `OsRng` (`R/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:994`). Control all three only for deterministic fixtures; production keeps fresh randomness. Otherwise require identical statements/VKs and independent acceptance.

# Validation: factory predicates, id/check/tier

**CODE —** Read-only `scripts/sherlock-next --json` named **A2** first failing in this session. A2–A6 and B3/B7 currently use unimplemented checks. This proposal does not turn historical receipts into passing factory results. No repository implementation, prover benchmark, key generation or deployment was performed; only source acquisition and the lightweight planning model ran.

**PROPOSAL —** Add the following records only during implementation. Each initially has `check: scripts/qa/unimplemented.sh <ID>`; the table gives its proposed replacement runner. Preserve existing A2–A10/B3–B9 obligations and evaluator-owned holdouts.

**PROPOSAL —** Rejection families cover malformed proofs, changed public statements, invalid deferred accumulators, mismatched keys/SRS, and contract-level replay/state binding. The GPU backend remains optional; factory instrumentation must not become a public compile/prove/deploy prerequisite.

| Proposed ID | Replacement check | Tier | Required result |
|---|---|---|---|
| F301 | `scripts/qa/check-loki-identity.sh` | T0 | CPU-only build works; exact key/SRS/version and statement identities. |
| F302 | `scripts/qa/check-loki-arithmetic.sh` | T1 | CPU/GPU field, MSM, NTT, quotient and ordering differentials; malformed/edge inputs. |
| F303 | `scripts/qa/check-loki-residency.sh` | T2, heavy | Resident-cache correctness, bounded resources, cancellation/recovery, no witness leakage. |
| F304 | `scripts/qa/check-loki-baseline.sh` | T2, heavy | Complete current CPU chain; exclusive phase/core-second measurements; concurrency sweep. |
| F305 | `scripts/qa/check-loki-throughput.sh` | T2, heavy | Paired CPU/GPU backlog runs, fill/drain separated, at least 20 completions/run and three repetitions; failures retained. |
| F306 | `scripts/qa/check-loki-ledger9.sh` | T2, heavy | Exact terminal accepted by real ledger-9, five required rejection families, independent verifier. |
| F307 | `scripts/qa/check-loki-stagenet.sh` | T3 | Exact old/new key family deployed, finalized positive/negative evidence and effects readback. |

**PROPOSAL —** Use separate measured budgets for current-key latency, host-throughput and B7-compliant experiments. Run concurrency inside one `heavy` child: its exclusive lock intentionally prevents separate heavy invocations from overlapping (`M/models/doctorwatson/factory/heavy.py:225`). Add VRAM telemetry; RSS accounting does not cover it. Read each failure trace, retry with a one-line `--hypothesis`, shrink after two same-kind failures, and apply the stop rule. T3 retains the factory's one-hour nonblocking cold review. Claim 10× only when repeated completed-work measurements support it against the qualified baseline; publish variability and resource costs.

# What the other leads got right/wrong

**PROPOSAL —** Astra-1 correctly rejects a fixed-key 2–4× law: broad witness/field acceleration could exceed it. Their HashMap-order correction improves the validation design. Their ≤4.21% irreducible-fraction condition for a 20× bulk/1% overhead latency model is useful, but cannot be transferred to throughput without CPU service accounting. I initially proposed a universal two-lane scheduling ceiling; our exchange corrected its hidden saturated-CPU assumption.

**PROPOSAL —** Astra-2 correctly identifies fixed-pair fusion as a stronger bounded latency experiment and flags synthetic witness custody. Their k13/k12 finding is source-backed; k14–15 fusion and ECIP savings remain hypotheses. A domain reduction cannot be multiplied blindly by GPU gains, and neither that tiny fixture nor an unmeasured 16-slot layout establishes the full financial-rollup target. Both leads correctly insisted on separate workload identities and the unchanged outer ledger protocol.

# Helper contributions used

**CODE / PROPOSAL —** **haiku-08** supplied cache lifecycle, PK decode and batching-API analysis (tasks `astra-3-1`, `astra-3-4`), queue-model and proposal reviews (`astra-3-7`, `astra-3-10`). **haiku-09** located historical CPU logs and DAG evidence (`astra-3-2`), composition/16-GB constraints (`astra-3-5`), baseline protocol (`astra-3-8`) and proposal review (`astra-3-11`). **haiku-10** supplied NVIDIA captures, Nocy source inspection, memory and proposal reviews (`astra-3-3`, `astra-3-6`, `astra-3-9`, `astra-3-12`). Reports reside under `../helpers/haiku-NN/`. I cross-checked adopted claims; unsupported linear table scaling, double-counted fixed columns, CPU-phase fractions inferred from whole-process averages, and a mandatory recursive wrapper for a new direct terminal were rejected. Their reports are research inputs, not acceptance evidence.
