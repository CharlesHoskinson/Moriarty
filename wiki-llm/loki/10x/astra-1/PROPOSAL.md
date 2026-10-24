---
title: "Same-key Sherlock at 10×: a resident GPU prover, including the witness"
project: loki-10x
author: gpt-6-astra
date: 2026-10-24
sources:
  local: ["../../LOKI-DESIGN.md", "../../nodes/loki-01-prover-profile.md", "../../nodes/loki-02-gpu-msm.md", "../../nodes/", "/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3", "/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4", "/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1", "/home/charles/repos/midnight-zk@31435e9", "/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9", "/home/charles/repos/Moriarty-factory/crates/mori-backend", "/home/charles/repos/midnight-ledger", "/home/charles/repos/projectfenrir/recursion/stagenet-strawman/chain/rc5check/ledger-rc5"]
  web:
    - {url: "https://ppopp26.sigplan.org/details/PPoPP-2026-papers/38/Pipelonk-Accelerating-End-to-End-Zero-Knowledge-Proof-Generation-on-GPUs-for-PLONK-B", capture: "raw/pipelonk-conference.md"}
    - {url: "https://github.com/zlyber/pipelonk/tree/ae", capture: "raw/pipelonk-repo.md"}
    - {url: "https://github.com/zlyber/pipelonk", capture: "raw/pipelonk-main.md"}
    - {url: "https://github.com/zlyber/pipelonk/blob/ae/ntt.cu", capture: "raw/pipelonk-ntt-page.md"}
    - {url: "https://github.com/Nocy-io/nocy-gpu-proof-server", capture: "raw/haiku-01-haiku01-nocy-repo.md"}
    - {url: "https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/366d3c72e814691cd16bbd72e77b5169d07f729c/native-plonk/cuda/src/kzg.h", capture: "raw/nocy-kzg.md"}
    - {url: "https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/366d3c72e814691cd16bbd72e77b5169d07f729c/native-plonk/cuda/src/msm.cuh", capture: "raw/nocy-msm.md"}
    - {url: "https://github.com/snarkify/zprize-2023-prize1b", capture: "raw/snarkify-zprize.md"}
    - {url: "https://raw.githubusercontent.com/snarkify/zprize-2023-prize1b/main/GPU-Micro-Benchmarks/MicroBenchmarks.cu", capture: "raw/snarkify-microbench-code.md"}
    - {url: "https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover", capture: "raw/snarkify-cusnark.md"}
    - {url: "https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2", capture: "raw/icicle-halo2-v2.md"}
    - {url: "https://arxiv.org/html/2509.22684v1", capture: "raw/zkprophet-full.md"}
    - {url: "https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html", capture: "raw/cuda134-best-practices.md"}
    - {url: "https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/", capture: "raw/haiku-03-h3-nvidia-5090.md"}
    - {url: "https://github.com/supranational/sppark", capture: "raw/haiku-03-h3-sppark.md"}
    - {url: "https://dev.ingonyama.com/start/architecture/install_gpu_backend", capture: "raw/haiku-03-h3-icicle-gpu-backend.md"}
---

## Target definition

**PROPOSAL —** Primary target **(a)** is tenfold lower **warm-service** latency for one unchanged k20 Sherlock parent: same linked children, public inputs, circuit, SRS, VK, transcript, proof encoding and acceptance conditions. Measure from witness availability through proof serialization, including synthesis, device transfers and synchronization. Report persistent-key warm service and cold process/key initialization separately; compare both against equally cached, release-mode CPU runs on the same host. Kernel speedups and batching are not single-proof speedups. The target host is the brief’s 20-core/90-GB WSL2 machine, RTX 5090 32 GB, CUDA 13.4.

**CODE —** The best completed local reference is [parent-proof.json](/home/charles/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json): **882.5896 s prove**, **62.3284 s setup_pk**, one fixed A102→A80 observation, cache state unknown, no controlled repetitions or postdeployment acceptance. Its prove interval includes PK destruction (`M/docs/results/sherlock-native-source44-20261022/source/sherlock_link.rs:1230`). The supplied “>500 s, killed” is censored, not a completed 500-second baseline. Neither the roughly 120-second keygen anecdote nor the nested 951.6/954.1-second intervals may be added indiscriminately.

**PROPOSAL —** Secondary targets are **(b)** witness-to-ledger-acceptable terminal decider latency and **(c)** sustained accepted 16-transaction batches/hour. Count all leaves, parent, decider, verification and serialization; report queueing separately. A fixed two-instruction Sherlock example is not a 16-transaction rollup. The final proof must pass ledger 9, including the terminal pairing, even if intermediate proofs use deferred verification.

## Verdict

**ESTIMATE —** **10× on (a) is technically credible but unproven; confidence low-to-medium.** The existing 2–4× estimate is a reasonable partial-offload expectation, not a same-VK protocol ceiling. Under the budget’s assumed shares, crossing 10× requires optimized witness construction as well as resident polynomial arithmetic. Do not promise it from an MSM adapter or the RTX 5090 name.

**OPEN QUESTION —** No available trace separates actual Sherlock synthesis, polynomial work and irreducible orchestration. Consequently an unconditional hardware ceiling cannot honestly be calculated. The exact conditional ceilings below identify what would make 10× impossible. My GPU query was denied by this execution environment; this report contains no new GPU benchmark or proof run.

**ESTIMATE —** Parent-only acceleration cannot deliver 10× on (b). Historical recl9 proof times sum to four leaves **163.631 s**, node **867.280 s**, decider **586.382 s**: **1,617.292 s**. A 10× node gives only **1.933×** chain speedup; an infinitely fast node gives **2.156×**. Even free node **and decider** leave a **9.884×** ceiling if the leaves remain unchanged. These are arithmetic bounds for that recorded cohort, not predictions for the current stack. Its serial rate is 2.226 batches/hour; tenfold means an interval below 161.729 s. Throughput requires a separately measured CPU scheduling baseline.

**CODE —** Those times come from `recl9/out/leaves-b1/leaves.stats.json`, `out/node-b1/w.stats.json`, and `out/decider-b1/batch-1.stats.json`. recl9 pins proofs 0.8.2; `poly/kzg/msm.rs:176` switches away from BLST above 2^19 nonzero scalars. Registry 0.8.3 instead always uses BLST for BLS G1. The Source44 parent receipt identifies a qualified Fenrir binary, not a proven current-workspace 0.8.3 build (`M/docs/results/sherlock-native-source44-20261022/README.md:7`). Its exact SDK closure also needs recovery. Mixing these lineages into a measured GPU speedup would be invalid.

## Evidence against a 4× hardware ceiling

| Label | Published evidence | What it establishes and excludes |
|---|---|---|
| SOURCE | [Nocy](https://github.com/Nocy-io/nocy-gpu-proof-server), RTX 5090: k17 12.3→4.4 s; k18 24.6→8.8 s, **2.80×**. [Capture](raw/haiku-01-haiku01-nocy-repo.md). | Closest hardware/protocol precedent; different workload and prover revision, no Sherlock phase breakdown. |
| SOURCE | [Snarkify BLS12-381 PLONK submission](https://github.com/snarkify/zprize-2023-prize1b): four proofs in **2.16 s**; reported ~900× versus its Rust baseline. [Capture](raw/snarkify-zprize.md). | Specialized witness and proving-round rewrite; removed unused lookups. **540 ms is amortized throughput, not measured one-proof latency**, and this is not a same-Sherlock-key comparison. |
| SOURCE | [cuSnark](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover): wide k20 circuit **1,762.75→463.13 s**, 3.81×; quotient **1,115.43→36.19 s**, 30.82×, on four RTX 3090s. [Capture](raw/snarkify-cusnark.md). | Witness/advice plus lookup preparation still consume 363.28 s after acceleration: holding those fixed gives a **4.85×** ceiling even with everything else free. Curve is not established by this page. |
| SOURCE | [ICICLE-Halo2 v2](https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2) reports up to **25×**, RTX 4080/i9-13900K examples, expressly excluding witness optimization. [Capture](raw/icicle-halo2-v2.md). | Proves broad offload can exceed 4× on some circuits; not a BLS12-381 Sherlock benchmark. |

**SOURCE —** [Pipelonk, PPoPP 2026](https://ppopp26.sigplan.org/details/PPoPP-2026-papers/38/Pipelonk-Accelerating-End-to-End-Zero-Knowledge-Proof-Generation-on-GPUs-for-PLONK-B) reports 10.7× average/19.4× maximum over an unspecified-in-abstract baseline across 8–80 GB GPUs. [Capture](raw/pipelonk-conference.md). Its [released artifact](https://github.com/zlyber/pipelonk/tree/ae) omits end-to-end instances; [NTT code](https://github.com/zlyber/pipelonk/blob/ae/ntt.cu) uses BLS12-381 scalar-field arithmetic. [Captures](raw/pipelonk-repo.md), [code capture](raw/pipelonk-ntt-page.md). The [current README](https://github.com/zlyber/pipelonk) names H100/4090 and says its BLST CPU operator baseline is 25–55% faster than the paper’s arkworks baseline. [Capture](raw/pipelonk-main.md). Borrow segmented grand-product/division operators; do not import the headline ratio or presume witness acceleration.

**CODE —** Nocy is not an optimized-residency ceiling: its [KZG implementation](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/366d3c72e814691cd16bbd72e77b5169d07f729c/native-plonk/cuda/src/kzg.h) copies scalars and synchronizes each commitment; its [MSM kernel](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/366d3c72e814691cd16bbd72e77b5169d07f729c/native-plonk/cuda/src/msm.cuh) performs independent windowed scalar multiplications followed by reduction, rather than a bucket MSM. [Captures](raw/nocy-kzg.md), [MSM capture](raw/nocy-msm.md). Removing these costs is a research opportunity, not an inferred numeric speedup.

**SOURCE —** Snarkify's BLS12-381 microtable reports 2^22 NTT **1.13 ms**, inverse **1.23 ms**, MSM **28.40 ms**. However, [benchmark code](https://raw.githubusercontent.com/snarkify/zprize-2023-prize1b/main/GPU-Micro-Benchmarks/MicroBenchmarks.cu) initializes before timing, warms up, times CUDA events, and repeats the generator as every MSM base. These are warm kernels, not transferable proof latencies. [Capture](raw/snarkify-microbench-code.md). [ZKProphet](https://arxiv.org/html/2509.22684v1) reports strong GPU gains but studies Groth16 and different CPU baselines; it also finds NTT can dominate after MSM acceleration. [Capture](raw/zkprophet-full.md).

**SOURCE —** The [5090 specification](https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/) provides 32 GB VRAM. [Capture](raw/haiku-03-h3-nvidia-5090.md). CUDA 13.4's [instruction-throughput table](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html) does not give Blackwell a blanket doubled Montgomery-arithmetic rate: integer multiply-add/carry throughput differs from ordinary addition, and extended-precision operations have further qualifications. [Capture](raw/cuda134-best-practices.md). Tensor/FP32 marketing throughput is not a BLS field-arithmetic roofline.

## The plan

**CODE —** Path shorthand below: `P` = registry `midnight-proofs-0.8.3/src`; `C` = `midnight-circuits-7.2.4/src`; `G` = `midnight-curves-0.3.1/src`; `Z` = `midnight-zk-stdlib-2.3.5/src`, all under `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`. `R` = `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9`; `M` = `/home/charles/repos/Moriarty-factory`.

1. **PROPOSAL — Profile and persist keys, 3–5 engineer-days.** Instrument `P/plonk/prover.rs:57,242,460`, argument helpers and PK destruction with exclusive wall/CPU scopes and device events. Record actual columns, degree, phases, MSM sizes/nonzeros, transfers and allocation high-water. Retain decoded PK/SRS across requests at `R/src/sherlock_link.rs:1041`; separate `:1029` cost-model work from synthesis. Identify caches by SRS/VK/circuit/domain/backend, not pointer identity. PK reads also reconstruct coefficient/coset representations (`P/plonk/mod.rs:416`); caching serialized bytes alone is insufficient. This changes no keys. Cache alone gives only `(882.590+62.328)/882.590 = 1.07062×` for those two intervals.

2. **PROPOSAL — Build a resident polynomial backend, 4–8 engineer-weeks.** Introduce a fallible `DeviceProofSession` around `compute_trace`/`finalise_proof`; preserve the CPU entry point and wire every commitment, including committed instances. `P/poly/kzg/mod.rs:74,83` calls `msm_specific`; replacing only an accumulator MSM trait misses these commitments. Batch within each transcript round, keep both SRS bases and precomputation resident, and return commitments in the original order. Avoid repeated zero-filter copying, affine conversion and scalar encoding at `P/poly/kzg/msm.rs:163` and `G/bls12_381/g1.rs:628`. Evaluate [sppark](https://github.com/supranational/sppark) first; independently price ICICLE's separately licensed GPU backend. [Captures](raw/haiku-03-h3-sppark.md), [license documentation](https://dev.ingonyama.com/start/architecture/install_gpu_backend), [capture](raw/haiku-03-h3-icicle-gpu-backend.md).

3. **PROPOSAL — Move all polynomial stages, not just FFT/MSM.** At `P/plonk/lookup/prover.rs:396`, canonicalize field sort keys once, radix-sort and compute multiplicities on device. Implement zero-preserving batch inversion and prefix scans for lookup/permutation products (`P/plonk/permutation/prover.rs:106,143`). Carry each permutation chunk's terminal product correctly. Compile the exact quotient expression graph (`P/plonk/evaluation.rs:285`) with liveness-based buffer reuse. Preserve y-Horner order, masks and rotations. `P/poly/domain.rs:37,81,224` defines extended size, roots and order-three `ZETA`; rotations use the extended stride. `P/plonk/vanishing/prover.rs:98,108,112,138` truncates and splits into **n−1 coefficient limbs**, then blinds them. A generic n-sized quotient splitter is wrong here.

4. **PROPOSAL — Complete resident multiopen.** Preserve query grouping and ordering at `P/poly/kzg/mod.rs:94,129`; use GPU reductions for evaluations and linear combinations. Replace serial `kate_division` (`P/utils/arithmetic.rs:95`) by an associative affine-map scan: the recurrence is `q_i = a_(i+1) + b*q_(i+1)`. Composition parallelizes it without changing the PCS, including b=0. Keep dense polynomial data resident through the final commitment; move only small scalar/point messages across transcript boundaries.

5. **PROPOSAL — Compile the witness if profiling requires it, another 8–16 engineer-weeks.** `P/plonk/prover.rs:501,533,540` synthesizes by phase and resolves rational values. `P/circuit/floor_planner/single_pass.rs:89,109` executes shape and assignment closures; `Z/lib.rs:1880` clones witness/instance values. Native-chip eager inversions appear at `C/field/native/native_chip.rs:367,774,1253`. First remove redundant allocation and batch safe inversions. Then build a specialized, key-bound witness recipe for Sherlock: preserve every cell, region offset, constant, copy constraint and challenge dependency, but schedule independent field/foreign-field operations in GPU batches. Arbitrary Rust `Value` closures do not automatically become GPU programs. Validate static layout before replay; retain fallback for unsupported paths. Circuit rows and VK remain unchanged.

**CODE —** Concrete witness costs include foreign-EC slope inversions (`C/ecc/foreign/weierstrass_chip.rs:1182,1266`), BigUint limb conversions (`C/field/foreign/field_chip.rs:430`) and GLV decomposition (`C/utils/util.rs:110`). The MSM loop at `C/ecc/foreign/weierstrass_chip.rs:1875` chains four doublings and selected-point additions per window. A million circuit rows does not imply a million sequential witness operations.

**PROPOSAL —** Record layout with an `Assignment`, but obtain arithmetic recipes from instrumented chip operations or a specialized generator: opaque Rust closures do not expose an operation graph. For each MSM window, compute **all original accumulator prefixes** by an associative group scan, batch-normalize them, then batch-invert the original slope denominators. Four doublings still connect successive windows. Preserve original adjacent-operand exceptional-case checks, shift/subgroup witnesses, table tags and GLV/limb conventions; a final-sum-only replacement is insufficient. Generate constrained Poseidon intermediate states in fused kernels, retaining their sequential sponge dependency. First implement the recipe on CPU and compare every assignment under injected test randomness; port only after equivalence. This could remove both arithmetic and interpreter overhead without changing constraints, but its speed remains unmeasured.

**CODE —** The outer prover has **10+C challenge squeezes**, grouped into **eight dependency barriers plus challenge-bearing advice phases** (`P/plonk/prover.rs:108,138,141,175,205,282,581`; `P/poly/kzg/mod.rs:105,106,166,176`). This is not eight kernel launches. Hash absorption, canonical point encoding, challenge ordering, request validation and output framing remain on CPU in this design. Scans, sorting, inversion, FFT bit reversal, polynomial division and RNG expansion are implementation work that can move; they are not intrinsic CPU floors.

**OPEN QUESTION —** The true serial cost is the measured host work **plus device dependency span and synchronization**, not the number of transcript calls. Recursive EC ladders and Poseidon witness chains retain dependencies even on GPU. Count operations, dependency depth, launch overhead and CPU fallbacks. The 5.687-second cost-model phase skips advice value closures (`P/dev/cost_model.rs:526`); it does not bound real synthesis. No recorded trace supplies the necessary fraction. The receipt’s 9.061 ms stdlib verification replays the outer transcript (`Z/utils/plonk_api.rs:130`), suggesting hashing itself is small; it does not bound prover synchronization or witness work.

**PROPOSAL —** Use a host-seeded, validated device RNG schedule for blinding only after differential testing; retain fresh production entropy and the same distribution. Keep transcript absorption on CPU. Build device kernels in coarse dependency groups, not one launch per field operation. None of steps 1–5 requires a new VK or ledger contract provided layout and VK-byte equality checks pass.

### Memory admission

**ESTIMATE —** For a geometry proxy, historical node4 has A=15, 27 actual fixed columns plus 18 sigma columns, five lookups; assume two instances. Actual Sherlock geometry is unrecorded. At k20 a scalar polynomial is 32 MiB; extended N=4n/8n costs 128/256 MiB. Keep 66 common extended buffers plus `max(S,3)` argument buffers, with `S=ceil(18/(d−2))`. For N=4n, d=4–5 gives 72–75 buffers; N=8n, d=6–9 gives 69–71. Do not sum mutually dead intermediates. `P/plonk/evaluation.rs:363,443` supports this lifetime separation.

| ESTIMATE allocation | N=4n GiB | N=8n GiB |
|---|---:|---:|
| Extended buffers, degree-dependent | 9.00–9.375 | 17.25–17.75 |
| Fixed/sigma ordinary+coefficient forms | 2.8125 | 2.8125 |
| Allowance for up to 64 live base-domain buffers | 2.00 | 2.00 |
| Two affine SRS tables, precomputation factor four | 0.75 | 0.75 |
| MSM scratch / NTT scratch / runtime allowances | 1.25+2+2 | 1.25+2+2 |
| Total proxy | **19.81–20.19** | **28.06–28.56** |

**PROPOSAL —** Admit one heavy proof with peak below `min(24 GiB, 0.8 × free VRAM)`; if extended degree forces N=8n, reduce retained caches or stream quotient argument groups before claiming it fits. Measure workspace and registers on this GPU; validate explicit CUDA allocations and bounded pinned staging under WSL2 rather than assuming native-Linux transfer behavior. Host RSS is not VRAM; unified-memory oversubscription does not establish residency. Two simultaneous heavy proofs are not budgeted.

## Speedup budget

**ESTIMATE —** For unchanged fraction f, accelerated remainder at s, and added overhead h relative to original time, `S = 1/[f+(1−f)/s+h]`. At s=20 and h=.01, 10× requires **f≤.0421053**, or ≤37.162 s at the historical scale, including every unchanged stage. An unchanged 10% tail has infinite-compute ceiling **9.09×**; a 20% tail has ceiling **4.76×**. With finite s=20 those become **6.45×** and **4.00×**. These are true conditional ceilings, not measured Sherlock fractions.

**ESTIMATE —** A falsifiable same-key success scenario splits original proof time into witness W=.20, other bulk B=.78, host U=.02; witness accelerates 8×, bulk 20×, added overhead=.01. Then `R=.20/8+.78/20+.02+.01=.094`, giving **10.638×**. At the historical 882.590-second scale this is **82.963 s**: witness 22.065 + bulk 34.421 + host 17.652 + overhead 8.826. Every phase share and acceleration in this scenario is an assumption; the receipt supplies only the scale. Recomputed arithmetic is in [budget.json](budget.json).

| Label | Sequential budget step | Remaining seconds | Incremental factor |
|---|---|---:|---:|
| CODE | Recorded setup_pk + prove intervals | 944.918 | 1 |
| ESTIMATE | Eliminate repeat PK construction, keep recorded prove | 882.590 | 1.07062 |
| ESTIMATE | Bulk20×, witness unchanged, U=.02, overhead=.01: R=.269 | 237.417 | 3.71747 |
| ESTIMATE | Also witness8×: R=.094 | 82.963 | 2.86170 |
| ESTIMATE | Product versus recorded setup+prove | 82.963 | **1.07062×3.71747×2.86170 = 11.3896×** |

**ESTIMATE —** This scenario has only **6.38% runtime headroom** before losing 10×. With witness8× it requires bulk≥17.33×, not merely a fast MSM kernel. Cold setup+proof, leaving PK rebuild unchanged, gives only `944.918/(62.328+82.963)=6.504×`; cold 10× would additionally need PK initialization≤11.528 s, or ≥5.407× faster. Persistent caching is a warm-service claim.

**ESTIMATE —** Primary warm-proof gain excludes caching: **3.71747×2.86170=10.638×**. Witness2× gives only **5.917×**; witness5× gives **9.174×**. This scenario requires witness≥6.452×. If bulk and witness each reach only8×, total is **6.557×**. Kernel factors are never multiplied as though every stage received every optimization.

**PROPOSAL —** Approve implementation incrementally: profile first; accept partial GPU gains if useful; pursue the expensive witness compiler only when its measured required speedup is plausible. Planning cost is **12–24 engineer-weeks plus profiling** for polynomial backend and specialized witness work, excluding deployment review and general-purpose circuit support. Re-estimate after the first profile.

## Risks and soundness; keeping keys versus new keys

**CODE —** Upstream `/home/charles/repos/midnight-zk@31435e9`, including PR #480's in-circuit PCS, uses interfaces/protocol work beyond registry 0.8.3. Porting upstream wholesale, switching lookup arguments, changing gates or reducing k is not a same-key optimization. The stock VK reader reconstructs configured architecture (`Z/lib.rs:1581,1593`); arbitrary new gates are not conveyed by simply deploying new key bytes.

**CODE —** The historical recursive keys use Fenrir-hardened circuits: `R/REPORT.md:64` records extra subgroup constraints on the private MSM shift and a small-constant multiplication guard. Freeze that exact audited relation; stock 7.2.4 source is an implementation map, not permission to remove hardening or substitute its keys. Soundness repairs that alter constraints require rekeying.

**CODE —** Fixed prover RNG/thread count alone does not guarantee identical proof bytes: `P/plonk/lookup/prover.rs:399,430` iterates a randomized HashMap when filling leftover table entries. Additionally, `C/ecc/foreign/weierstrass_chip.rs:994` samples independent `OsRng` witness randomness. GPU canonical ordering may produce a different valid lookup permutation. Require unchanged VK bytes, assignment equivalence, multiset/adjacency identities and independent verification; freeze circuit randomness and lookup permutations as well as prover RNG/threads for byte-exact fixtures. Test zero denominators, identity points, canonical encodings, reductions, races, rotations and quotient limb boundaries. Never weaken subgroup checks or challenge entropy.

**CODE —** `R/src/decider.rs:163,186,201` binds reassigned accumulator points, completes the pairing and binds communication state. Prepare-only intermediate acceptance is insufficient; `R/REPORT.md:191` documents that a swapped proof can pass an intermediate check while failing the decider. Its ledger9 rc.6 verification was local, not a submitted transaction. The inspected reader is `.../chain/rc5check/ledger-rc5/transient-crypto/src/proofs.rs:563`; I could not bind that source snapshot to the recorded rc.6 verifier binary. Recover the tested SDK closure and artifact hashes; neither directory names nor generic `midnight-ledger` package labels establish interoperability.

**PROPOSAL —** Price new keys as a competing product route. Astra-2's direct stock-gate fusion could remove the recursive parent for the fixed A102/A80 example: estimated 20,466 rows fits k15 capacity 32,761, subject to actual table/layout measurement. That targets **(b), not faster production of the existing k20 parent**. Its planning estimate of 2–4 engineer-weeks for this fixture is potentially cheaper than a full GPU witness compiler; broader bounded-chunk semantics are estimated 6–12+ weeks, not demonstrated. Stagenet deployment may be cheap, but statement equivalence, keygen, reader compatibility, negative tests and contract registration are real work; transaction fees remain unpriced.

**CODE —** Rekeying a retained parent propagates to its decider, which embeds the parent VK (`R/src/decider.rs:238`). Reusing existing suitable official SRS powers needs no new ceremony; the ledger's embedded 2^14 verifier parameters do not impose a k14 circuit limit. Preserve the ledger-compatible tau. Fusion must retain custody and financial semantics: the historical synthetic leaf holds all spender secrets (`R/src/leaf.rs:7`), which is not a license to change production custody.

## Validation: factory predicates, id/check/tier

**CODE —** Read-only `scripts/sherlock-next` reports **A2 first failing**, with A2–A6 currently unimplemented checks. Factory sets are A/B/C/U/F, not the design's proposed L set. No implementation, repository edit, heavy run or deployment was performed for this research. The read-only result is retained in [factory-next.txt](factory-next.txt).

| PROPOSAL ID | Tier; future check | Heavy budget key | Acceptance condition; dependencies |
|---|---|---|---|
| F201 | T2; `scripts/qa/check-f201.sh` | `f201-phase-profile` | Exclusive CPU/GPU phase profile and geometry; A2 |
| F202 | T1; `scripts/qa/check-f202.sh` | `f202-gpu-differential` | Small arithmetic/scan/witness differentials; larger cases promoted to T2 |
| F203 | T2; `scripts/qa/check-f203.sh` | `f203-argument-equivalence` | Complete randomized GPU proofs and mixed CPU/GPU recursion; A2,F202 |
| F204 | T2; `scripts/qa/check-f204.sh` | `f204-k20-10x` | Paired k20 latency ratio≥10; memory/transfer/tail report; F201,F203,F8 |
| F205 | T2; `scripts/qa/check-f205.sh` | `f205-ledger9` | Terminal GPU chain accepted by pinned ledger9 reader; A4,A5,A6,F203 |
| F206 | T3; `scripts/qa/check-f206.sh` | `f206-stagenet` | Stagenet acceptance, rejection cases and effect readback; F205,A7–A10 |

**PROPOSAL —** These free F201–F206 IDs and runners are proposals, not installed predicates. Until implemented, use `scripts/qa/unimplemented.sh ID`. Set `heavy: true` for these GPU/proof checks; keep F202 small enough for T1’s 20-minute limit. F201 awaits genuine linked children under A2. Give heavy checks measured budgets and run through `scripts/doctorwatson check ID`/`heavy`; retry with a one-line hypothesis, shrink after two like failures, obey stop-rule. Keep evaluator-owned holdouts intact. CUDA and factory tooling remain optional for public developers compiling, proving or deploying. Run required Rust fmt/clippy/workspace tests after implementation. T3 receives the prescribed nonblocking cold review.

**PROPOSAL —** Compare frozen input/VK/SRS hashes, release flags, CPU/BLST thread budgets and identical cache conditions. Use at least five completed paired runs for the median and at least twenty for a preliminary tail estimate; report uncertainty and require the speedup confidence bound to clear 10×; isolate PK creation/destruction and cold start. Independently verify every output; F204 alone grants no ledger claim. Extend B3/B4 for 16-transaction throughput and B7 for its resource requirement: B7's <16 GB RSS/no-swap target is stricter than merely fitting this 90 GB host. CPU MockProver mutation tests check constraints, not GPU arithmetic; run real cryptographic corruption/rejection tests too.

## What the other leads got right/wrong

**ESTIMATE —** Astra-2 correctly prices new-key fusion against expensive same-key engineering. The correction is scope: fixed-fixture row arithmetic establishes neither 39-instruction completeness nor a 16-transaction workload, and eliminating the parent cannot be advertised as axis(a). I requested and incorporated explicit engineering costs and stock-reader constraints.

**ESTIMATE —** Astra-3 correctly separates latency from queue throughput. Its simulation with four CPU witness workers and one GPU reaches a 154.705-second interval under assumed 25% witness/75% bulk and 10× bulk speedup, but single-batch latency remains 508.297 seconds. The 10.45× throughput ratio is against a naive historical serial baseline; a 5% stronger CPU baseline already removes 10×. If tuned CPU throughput is twice that baseline, the scenario yields only 5.23×; even infinitely fast GPU work with its four witness workers has an 8× ceiling. Its later eight-worker/20×-bulk scenario reaches a conditional 10.47× even against that doubled CPU baseline, but assumes nine host slots and adequate memory. Its earlier two-lane ceiling did not cover multiple witness builders and was corrected. Conversely, multiple workers require measured CPU service time, memory and contention; throughput gains cannot simply multiply single-proof gains. Neither scheduler result is a hardware measurement.

## Helper contributions used

**CODE —** [haiku-01](../helpers/haiku-01/astra-1-8.md) audited Nocy's actual backend, timer boundaries and per-commit transfers; the [Pipelonk follow-up](../helpers/haiku-01/astra-1-13.md) separated scalar operators from full proving. Initial broad search omissions were corrected against primary sources. [haiku-02](../helpers/haiku-02/astra-1-2.md) traced phase barriers, synthesis and the critical 0.8.2/0.8.3 MSM difference. [haiku-03](../helpers/haiku-03/astra-1-3.md) and [follow-up](../helpers/haiku-03/astra-1-5.md) supplied kernel evidence and memory calculations, corrected for 27 fixed + 18 sigma columns. [haiku-04](../helpers/haiku-04/astra-1-6.md) checked factory records, ledger/decider obligations and randomized lookup ordering. [Witness follow-up](../helpers/haiku-02/astra-1-2-followup-witness.md) identified foreign-EC inversions, BigUint conversion and the cost-model limitation. [Scan review](../helpers/haiku-02/astra-1-12.md) checked preservation of ordered MSM intermediates and exceptional cases. Their claims were treated as leads and checked against source; unsupported version-interoperability and benchmark extrapolations were excluded.
