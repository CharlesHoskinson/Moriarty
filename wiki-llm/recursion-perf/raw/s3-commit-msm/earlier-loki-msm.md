# Earlier Loki GPU MSM study
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-02-gpu-msm.md
> Collected: 2026-10-26
> Published: 2026-10-24

---
title: "GPU MSM for Midnight: resident BLS12-381 commitments on RTX 5090"
project: loki
node: loki-02-gpu-msm
author: gpt-6.1-sol
date: 2026-10-24
tags: [gpu, msm, bls12-381, kzg, cuda, blackwell, recursion]
sources:
  - https://www.zprize.io/blog/announcing-the-2023-zprize-winners
  - https://github.com/matter-labs/z-prize-msm-gpu-combined
  - https://github.com/z-prize/2023-entries/tree/main/prize-1-fpga-gpu-proof/prize-1a-msm/gpu/Marco/ZPrize-23-Prize1A
  - https://github.com/supranational/sppark
  - https://dev.ingonyama.com/3.0.0/icicle/primitives/msm
  - https://github.com/ingonyama-zk/icicle/releases/tag/v4.0.0
  - https://github.com/ingonyama-zk/icicle/blob/v4.0.0/wrappers/rust/icicle-core/src/msm/mod.rs
  - https://dev.ingonyama.com/start/architecture/install_gpu_backend
  - https://dev.ingonyama.com/1.10.1/icicle/golang-bindings/msm-pre-computation
  - https://github.com/matter-labs/era-bellman-cuda
  - https://github.com/matter-labs/era-bellman-cuda/blob/main/src/ff_config.cuh
  - https://eprint.iacr.org/2022/1321
  - https://github.com/Tao-Lu-123/cuZK
  - https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/
  - https://developer.nvidia.com/cuda-gpus
  - https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html
  - https://docs.nvidia.com/cuda/wsl-user-guide/index.html
---

## Question

**PROPOSAL:** Accelerate Sherlock's existing KZG commitments with GPU BLS12-381 G1 MSM, retaining the deployed ledger's proof system, transcript, SRS, VK, and serialization. Evaluate reusable kernels before writing new field arithmetic. This node specifies a design and experiments; it reports no new builds, benchmarks, or stagenet acceptance.

## Summary

- **CODE:** The installed prover's hooks are `KZGCommitmentScheme::commit` and `commit_lagrange`, both calling `msm_specific`; searching for the older name `best_multiexp` misses this implementation. [Installed KZG implementation:74](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74).
- **PROPOSAL:** Keep monomial and Lagrange SRS bases resident, precompute shifted bases, and batch independent columns within each transcript phase. Return ordinary Midnight G1 commitments.
- **SOURCE:** ICICLE documents BLS12-381 $2^{20}$ MSM at 25.5 ms without precomputation and 18.1 ms with factor four on RTX 3090 Ti; these are vendor measurements with incompletely specified timing boundaries. [Benchmark](https://dev.ingonyama.com/1.10.1/icicle/golang-bindings/msm-pre-computation), [raw/02/icicle-bls381-precompute-bench.md](../raw/02/icicle-bls381-precompute-bench.md).
- **ESTIMATE:** A conditional RTX 5090 target is approximately 11–21 ms per warm $2^{20}$ commitment, from $1.5+(18.1/\alpha+0.67)$ ms with assumed $\alpha\in[1,2]$. This is a planning model, not measured throughput.
- **PROPOSAL:** Use sppark as the first open-source candidate and ICICLE v4 as a comparison backend, subject to its GPU licensing and Blackwell support. Require exact commitment equality and unchanged-verifier acceptance before any performance claim.

## Findings

### Actual call graph and G2 scope

**CODE:** Moriarty pins proofs 0.8.3, circuits 7.2.4, and stdlib 2.3.5 in [mori-backend/Cargo.toml:14](/home/charles/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14), and curves 0.3.1 in [workspace Cargo.toml:39](/home/charles/repos/Moriarty-factory/Cargo.toml:39). Stdlib's `prove` delegates to `BlstPLONK`; its key types embed `KZGCommitmentScheme<Bls12>`. [Installed stdlib/lib.rs:1551](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1551), [lib.rs:1984](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1984).

**CODE:** The installed chain is `commit` → monomial `params.g`, or `commit_lagrange` → `params.g_lagrange` → `msm_specific` → `G1Projective::multi_exp` → blst Pippenger. The wrapper filters zero scalars and copies the remaining projective bases; blst's wrapper converts points to affine and serializes scalars on each call. [proofs/kzg/mod.rs:74](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74), [proofs/kzg/msm.rs:163](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:163), [curves/g1.rs:628](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/g1.rs:628).

**CODE:** The checkout at `31435e9` differs materially: `commit_many` accepts labeled polynomials and affine bases, but currently maps them to individual MSMs. Its `msm_specific` switches away from blst above `2 << 18`, which equals $2^{19}$, despite its comment mentioning $2^{18}$. The installed copy instead always selects blst for BLS12-381 G1. [Checkout kzg/mod.rs:82](/home/charles/repos/midnight-zk/proofs/src/poly/kzg/mod.rs:82), [kzg/msm.rs:174](/home/charles/repos/midnight-zk/proofs/src/poly/kzg/msm.rs:174). The comparison Halo2 checkout calls `msm_best` after per-call normalization. [halo2/kzg/mod.rs:68](/home/charles/repos/halo2/src/poly/kzg/mod.rs:68).

**CODE:** `MSMKZG::eval` also calls `msm_specific`, but its bases are accumulated proof commitments rather than the large SRS arrays. `DualMSM::check` finishes with two G1 results paired against prepared G2 parameters. This observed verification path has no large G2 MSM. [Installed kzg/msm.rs:140](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:140), [msm.rs:285](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:285).

**CODE:** Recursive circuit MSMs are separately represented by `AssignedMsm`; they are constraints, not interchangeable host MSM calls. GPU commitment arithmetic does not remove those constraints. [Installed circuits/verifier/msm.rs:40](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/msm.rs:40). The checkout aggregation module likewise describes deferred accumulators. [aggregation/mod.rs:15](/home/charles/repos/midnight-zk/aggregation/src/multi_circuit_aggregator/mod.rs:15).

### Primary implementations and benchmark interpretation

| Classification | Evidence and implication |
|---|---|
| **SOURCE** | **ZPrize 2023:** official GPU winner Marco Zhou/StorSwift scored 430 ms; Yrrid/Snarkify received special mention at 367 ms for a late submission. The announcement alone does not establish BLS12-381 performance. [Results](https://www.zprize.io/blog/announcing-the-2023-zprize-winners), [raw/02/zprize-2023.md](../raw/02/zprize-2023.md). |
| **SOURCE** | **Marco's submitted README:** targets four $2^{24}$ MSMs on A40 and roughly 46 GiB, with performance/optimization sections unfinished. Treat the score as a contest result, not a deployable 32 GB configuration. [Entry](https://github.com/z-prize/2023-entries/tree/main/prize-1-fpga-gpu-proof/prize-1a-msm/gpu/Marco/ZPrize-23-Prize1A), [raw/02/zprize-marco.md](../raw/02/zprize-marco.md). |
| **SOURCE** | **Yrrid/Matter Labs combined:** reports 2.2–2.3 s for four $2^{26}$ MSMs, A40, approximately 46 GiB. Uses 23-bit signed windows, scalar/bucket sorting by workload, shifted-base precomputation, XYZZ accumulators, optimized Montgomery arithmetic, and copy/compute overlap. Its 253-bit recoding is not a BLS12-381 scalar implementation. [Repository](https://github.com/matter-labs/z-prize-msm-gpu-combined), [raw/02/yrrid-matterlabs-combined.md](../raw/02/yrrid-matterlabs-combined.md). |
| **SOURCE** | **sppark/Supranational:** CUDA/C++ templates explicitly include BLS12-381 MSM, Rust integration examples, and Apache-2.0 licensing. Its README cautions that proof-of-concept FFI warnings need resolving for production. [Repository](https://github.com/supranational/sppark), [raw/02/sppark.md](../raw/02/sppark.md). |
| **SOURCE** | **cuZK:** paper reports 2.08–2.94× against an earlier GPU MSM baseline and overlapping transfers with computation. The authors' repository reports BLS12-381 V100 times of 190 ms at $2^{20}$, 580 ms at $2^{22}$; this is historical evidence, not a 5090 forecast. [Paper](https://eprint.iacr.org/2022/1321), [raw/02/cuzk-paper.md](../raw/02/cuzk-paper.md); [code/results](https://github.com/Tao-Lu-123/cuZK), [raw/02/cuzk-code.md](../raw/02/cuzk-code.md). |
| **SOURCE** | **era-bellman-cuda:** useful GPU prover architecture, defaulting to compute architecture 8.0. Its field configuration contains eight 32-bit limbs and the BN254 moduli, so importing its arithmetic would require a BLS12-381 port. [Repository](https://github.com/matter-labs/era-bellman-cuda), [raw/02/era-bellman-cuda.md](../raw/02/era-bellman-cuda.md); [field definitions](https://github.com/matter-labs/era-bellman-cuda/blob/main/src/ff_config.cuh), [raw/02/era-ff-config.md](../raw/02/era-ff-config.md). |

**SOURCE:** ICICLE v3 exposes shared-base batches, precomputation, asynchronous streams, scalar bit length, window size, and Montgomery flags. Its CUDA algorithm sorts bucket work, splits unusually large buckets, and parallelizes weighted reduction. [v3 MSM documentation](https://dev.ingonyama.com/3.0.0/icicle/primitives/msm), [raw/02/icicle-v3-msm.md](../raw/02/icicle-v3-msm.md).

**SOURCE:** ICICLE v4 announces a Rust/API redesign; its tagged Rust implementation retains `MSMConfig`, `msm`, and `precompute_bases`, deriving batch size and device placement from slices. The release notes establish no BLS12-381 speedup over v3. [v4 release](https://github.com/ingonyama-zk/icicle/releases/tag/v4.0.0), [raw/02/icicle-v4-release.md](../raw/02/icicle-v4-release.md); [v4 Rust source](https://github.com/ingonyama-zk/icicle/blob/v4.0.0/wrappers/rust/icicle-core/src/msm/mod.rs), [raw/02/icicle-v4-msm-api.md](../raw/02/icicle-v4-msm-api.md).

**SOURCE:** The version-4 GPU-backend documentation describes closed-source CUDA/Metal components, an R&D license using a license server, and separate production arrangements. Its wording still refers to v3; verify terms for the exact artifact. The release's open-source ML-KEM claim does not establish open-source MSM CUDA kernels. [Backend documentation](https://dev.ingonyama.com/start/architecture/install_gpu_backend), [raw/02/icicle-gpu-backend.md](../raw/02/icicle-gpu-backend.md).

### Windows, precomputation, and transfer arithmetic

**PROPOSAL:** Use Pippenger buckets for $Q=\sum_i s_iP_i$: split each 255-bit scalar into digits, group points by digit, sum each bucket, then compute weighted bucket sums and combine windows. Sort by bucket occupancy; split large buckets into parallel partial sums. Compare unsigned and signed recoding with explicit final carries. Do not transplant the contest's 253-bit negation shortcut.

**SOURCE:** ICICLE recommends starting near $c=\log_2N-4$ without precomputation and testing larger windows with precomputation. It identifies nonuniform scalars and incomplete top windows as large-bucket causes. [Tuning guidance](https://dev.ingonyama.com/3.0.0/icicle/primitives/msm), [raw/02/icicle-v3-msm.md](../raw/02/icicle-v3-msm.md).

**PROPOSAL:** At k20, test $c\in\{14,16,18,19\}$, precomputation factors $p\in\{1,2,4,8\}$, and batches $b\in\{1,4,8\}$; begin with $c=16,p=4$. Fixed SRS vectors permit shifted copies $2^{\ell j}P_i$, not an assumption that every scalar multiplies one generator. Select using real advice/fixed/quotient distributions, including sparse columns.

**PROPOSAL:** Make the recoding identity explicit during review. With W original c-bit windows and m = ceil(W/p), prepare Q(i,t) = [2^(cmt)]P(i). Original digit j = r + mt contributes to bucket module r using Q(i,t); the final module weight is 2^(cr), reproducing its original weight 2^(cj). Thus precomputation reduces the number of bucket modules from W to m while increasing base storage. It does not eliminate scalar digit extraction or all point additions. Backend adapters must use their library's actual precomputation layout, with explicit zero padding for the partial top window. For weighted reduction, validate a parallel prefix/reduction implementation against the descending running-sum identity before tuning it.

**ESTIMATE:** For $N=2^{20}$, assume packed scalar/affine/Jacobian sizes 32/96/144 bytes. Two SRS representations with $p=4$ require $2Np96=768$ MiB. With $q=\lceil\lceil255/c\rceil/p\rceil=4$, a conservative per-MSM scratch allowance is $32N+24qN+144q2^c=164$ MiB. Eight concurrent jobs plus both tables total $768+8(164)=2080$ MiB, excluding allocator/backend overhead and other prover buffers. The index coefficient follows [ICICLE's memory model](https://dev.ingonyama.com/3.0.0/icicle/primitives/msm), [raw/02/icicle-v3-msm.md](../raw/02/icicle-v3-msm.md); shared bases are counted once.

**ESTIMATE:** If the selected kernel stores XYZZ buckets at 192 bytes instead, bucket storage rises by $4(2^{16})(192-144)=12$ MiB per job; the same eight-job total becomes $2080+8(12)=2176$ MiB. These allowances deliberately retain indices and buckets simultaneously; actual peak allocation and padded backend layouts must be measured.

**ESTIMATE:** Aggressive $p=14,c=19$ reduces bucket modules to one, but two k20 tables alone cost $2N14(96)=2.625$ GiB; at k24 they cost 42 GiB and cannot fit a 32 GB card. More precomputation is not automatically better. Break-even reuse is $m>T_{prepare}/(t_{unprepared}-t_{prepared})$, with all three times measured independently.

**SOURCE:** NVIDIA specifies 32 GB GDDR7 and PCIe Gen 5 for RTX 5090, and lists compute capability 12.0. [GPU specifications](https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/), [raw/02/nvidia-5090.md](../raw/02/nvidia-5090.md); [compute capabilities](https://developer.nvidia.com/cuda-gpus), [raw/02/cuda-gpus.md](../raw/02/cuda-gpus.md).

**ESTIMATE:** Assuming a negotiated Gen-5 x16 link at 32 GT/s and 128/130 encoding, the pre-packet upper bound is $32\cdot16\cdot128/130/8=63.0$ GB/s per direction. At an assumed effective 50 GB/s, one k20 scalar vector transfers in $33{,}554{,}432/(50\cdot10^9)=0.67$ ms; one affine SRS table takes $100{,}663{,}296/(50\cdot10^9)=2.01$ ms. Two cold tables take 4.03 ms of transfer alone. Normalization, conversion, precomputation, and synchronization add time; negotiated bandwidth and WSL performance remain unmeasured.

## Proposed design

**PROPOSAL:** Compare three approaches: (1) a synchronous replacement inside `msm_specific`, easiest but with weak SRS identity and batching information; (2) a commitment-aware resident cache plus explicit phase batches, recommended; (3) a custom Blackwell Pippenger implementation, highest tuning freedom but largest arithmetic-validation burden. Start with sppark and retain an ICICLE adapter for comparisons; choose the production backend from measured results and deployability.

**PROPOSAL:** Add an optional prover-only backend with these boundaries:

| Classification | Module | API/responsibility |
|---|---|---|
| **PROPOSAL** | `gpu_msm::encoding` | Convert Midnight scalars and affine G1 into explicit backend wire types; encode infinity; convert results back through checked decoding. |
| **PROPOSAL** | `gpu_msm::srs_cache` | `prepare(BaseKey, bases, policy) -> Result<BaseHandle>`; cache normalized/uploaded bases and shifted copies. |
| **PROPOSAL** | `gpu_msm::backend` | `msm_many(handle, scalar_columns, stream) -> Result<Pending<Vec<G1Projective>>>`; implement CPU, sppark, and optional ICICLE adapters. |
| **PROPOSAL** | `gpu_msm::scheduler` | Bound memory, own streams/pinned buffers/events, apply CPU crossover, and return results in submission order. |

**PROPOSAL:** `BaseKey` binds curve, authenticated SRS digest, polynomial representation, domain k, vector length, backend version, and precomputation configuration. Compute digests at parameter loading, not per column. Monomial prefixes can share a table; downsized Lagrange bases need a distinct entry. Keep a typed sidecar handle outside serialized parameters and keys. Scope device work to a proof session to prevent overlapping proof processes from exhausting VRAM.

**CODE:** Downsizing actually truncates monomial bases and recomputes Lagrange bases. [Installed params.rs:40](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/params.rs:40). Midnight's scalar type is confusingly named `Fq` and stored as little-endian Montgomery `u64` limbs; its blst MSM uses canonical little-endian bytes with 255 bits. [curves/fq.rs:22](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fq.rs:22), [g1.rs:639](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/g1.rs:639).

**PROPOSAL:** Initially transfer canonical 32-byte scalars and explicit affine coordinates. Do not transmute Midnight objects into ICICLE/arkworks/sppark objects. Later permit direct Montgomery transfers only after proving limb order, modulus, radix, and representation equality. Preserve original SRS indices when skipping zeros; compacting scalars alone silently commits to a different polynomial.

**CODE:** The local CUDA Rust project provides host contexts, streams, buffers, and events, but describes itself as early-stage research; its SIMT compiler uses a pinned nightly. [nvidia-cuda-rust/README.md:23](/home/charles/repos/nvidia-cuda-rust/README.md:23), [README.md:33](/home/charles/repos/nvidia-cuda-rust/README.md:33), [README.md:45](/home/charles/repos/nvidia-cuda-rust/README.md:45).

**PROPOSAL:** Initially isolate existing CUDA/C++ kernels behind a narrow C ABI with opaque context/table handles, explicit lengths, owned buffers, and status codes. Keep the Rust 1.99 host build independent of a new device compiler; evaluate native Rust kernels only after the reference adapter establishes correctness and useful speedup.

**PROPOSAL:** Hook installed `commit`/`commit_lagrange` before zero filtering. Route sufficiently large BLS12-381 jobs to the resident handle, preserving the original CPU path for unsupported curves and small inputs. Since the current trait returns a point rather than `Result`, recoverable GPU errors require an exact CPU recomputation; never return identity on failure. Reject invalid inputs, and abandon a poisoned device context before fallback. [Trait signatures:44](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/commitment.rs:44).

**CODE:** Advice columns are committed together, written to the transcript in vector order, and only then followed by phase challenges. Fixed-key commitments also form a natural batch. [Installed prover.rs:568](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:568), [keygen.rs:283](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/keygen.rs:283).

**PROPOSAL:** In a later patch to 0.8.3, add default CPU-loop batch methods to `PolynomialCommitmentScheme`; override for KZG, replacing these loops without changing result or transcript order. Batch lookup/permutation/quotient commitments only within their existing challenge dependencies. The `31435e9` `commit_many` API is a separate adaptation target, not justification to upgrade the proof protocol. Warm bases → scalar staging → GPU bucket work → ordered points → existing transcript is the complete data flow.

**PROPOSAL:** Start with one compute stream and double-buffered scalar transfers; limit GPU scratch to 8 GiB until the combined MSM/NTT memory model is measured. Keep small G1 verifier MSMs and G2 pairings on CPU. CUDA remains an optional acceleration feature; public compilation, proving, and deployment retain a CPU route.

## Expected speedup

**ESTIMATE:** Use the vendor's 18.1 ms k20 BLS12-381 result as a scale anchor, assuming comparable work, resident bases, factor-four preparation already paid, 5090 kernel improvement $\alpha=1\ldots2$, a fixed 1.5 ms service allowance, and the assumed 0.67 ms k20 scalar transfer scaled with input size. Model $T(k)=1.5+(18.1/\alpha+0.67)2^{k-20}$ ms. Linear scaling is an explicit approximation; these extrapolations are not vendor measurements or guaranteed bounds.

| Classification | Points | Modeled warm latency | Modeled throughput |
|---|---:|---:|---:|
| **ESTIMATE** | $2^{16}$ | 2.11–2.67 ms | 24.5–31.1 Mpoints/s |
| **ESTIMATE** | $2^{18}$ | 3.93–6.19 ms | 42.3–66.7 Mpoints/s |
| **ESTIMATE** | $2^{20}$ | 11.22–20.27 ms | 51.7–93.5 Mpoints/s |
| **ESTIMATE** | $2^{22}$ | 40.38–76.58 ms | 54.8–103.9 Mpoints/s |
| **ESTIMATE** | $2^{24}$ | 157.02–301.82 ms | 55.6–106.8 Mpoints/s |

**ESTIMATE:** Throughput is $2^k/(T(k)/1000)$; k24 additionally requires a memory-feasible configuration. The fixed allowance excludes any additional input-dependent conversion cost; kernel scaling, allocator behavior, and the original benchmark timing boundary still need measurement. Batching may amortize launches, but must not be counted as another multiplicative speedup without measurements.

**ESTIMATE:** If one reference k20 MSM takes an assumed 0.5 s, 11.22–20.27 ms implies $500/20.27\ldots500/11.22=24.7\ldots44.6\times$ for that operation. No CPU MSM timing was measured here. For an illustrative 500 s proof with MSM fraction $f=0.6$, kernel factor $s=30$, and 2 s cold preparation, $T'=500[(1-0.6)+0.6/30]+2=212$ s, or 2.36× overall. At $f=0.3$, the same calculation gives 357 s, or 1.40×. The reported long parent/VK attempts do not identify either fraction.

## Risks and soundness concerns

**PROPOSAL:** Preserve exactly $\sum_i s_iP_i$, including scalar reduction, signed carries, infinity, duplicate bases, inverse points, exceptional additions, and subgroup membership. Check equality after normalization, not equality of projective coordinate bytes. Secret-dependent bucket access has side-channel implications; do not claim constant-time behavior for this design.

**SOURCE:** NVIDIA's compatibility guide requires compatible cubins or forward-compatible PTX; an Ampere-only binary is insufficient. WSL documentation limits pinned memory and full managed-memory functionality. [Blackwell guide](https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html), [raw/02/cuda-blackwell.md](../raw/02/cuda-blackwell.md); [WSL guide](https://docs.nvidia.com/cuda/wsl-user-guide/index.html), [raw/02/cuda-wsl.md](../raw/02/cuda-wsl.md).

**PROPOSAL:** Build the selected backend for `sm_120` with PTX retained, verify its support independently, and use bounded explicit device/pinned allocations. Do not assume the 90 GB host RAM extends the 32 GB device budget. Protect witness buffers, enforce stream lifetimes, and clear/release sensitive staging allocations.

**CODE:** Sherlock's parent uses the Poseidon transcript type `Pos` and checks the child accumulator before proving; the terminal decider uses `Blake`. Ledger verification delegates to stdlib under its verifier parameters. [recl9/sherlock_link.rs:1024](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:1024), [recl9/main.rs:603](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:603), [ledger/proofs.rs:545](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:545).

**PROPOSAL:** GPU MSM changes no proof format and requires no SRS/VK rotation when arithmetic is equivalent. Replacing KZG/BLS12-381 with BN254, Groth16, a folding proof, or another transcript changes the verifier contract: it requires an existing-compatible outer wrapper or new verifier/circuit/key deployment and acceptance evidence. Do not silently adopt a different checkout's proof API or substitute `unsafe_setup` parameters.

## Validation plan

1. **PROPOSAL:** Freeze crate/backend source and binary digests, SRS digest, VK bytes, transcript, k, device, driver, and negotiated PCIe link. Record calls by representation, length, nonzero count, and phase; establish reference CPU time before assigning an MSM fraction.
2. **PROPOSAL:** Differentially test against the current blst path and an independent small naive sum: empty/all-zero inputs, one term, sparse positions, powers of two, $r-1$, top-bit/carry boundaries, equal and opposite points, and randomized vectors through k20. Require exact normalized G1 equality, precomputed/unprepared equality, and single/batch equality. Explicitly test wrong cache keys and Lagrange downsizing.
3. **PROPOSAL:** Run identical circuit, witness, SRS, VK, transcript, and deterministic test RNG through CPU/GPU proving. Require byte-identical commitments and, when RNG consumption and ordering match, byte-identical proofs. With fresh blinding, require acceptance by the unchanged CPU verifier instead of expecting identical proof bytes.
4. **PROPOSAL:** Carry GPU-produced child proofs through the existing parent and terminal decider. Verify under the pinned ledger-9 implementation with real proof verification enabled; mutation tests must reject altered statements, wrong VK/SRS, and corrupted proof bytes. Local acceptance is followed by a separately authorized stagenet transaction and receipt, not substituted for it.
5. **PROPOSAL:** Publish predicates `MSM_EQ`, `BATCH_EQ`, `TRANSCRIPT_EQ`, `REFERENCE_VERIFY`, `LEDGER9_ACCEPT`, and `MEMORY_BOUNDED`. Measure cold/warm wall time, conversion, transfers, kernel phases, preparation amortization, VRAM, host RSS, and whole-proof latency separately. Any later heavy experiments run through `scripts/doctorwatson heavy`; none were run for this research node.

## Open questions

- **OPEN QUESTION:** How many full-length commitments occur in the actual k20 parent, and what fraction of its wall time is blst MSM versus synthesis, FFTs, or field operations?
- **OPEN QUESTION:** Which sppark/ICICLE artifacts support CUDA 13.4 and `sm_120`, and what production redistribution/offline terms apply to the selected ICICLE backend?
- **OPEN QUESTION:** What are actual WSL pinned-memory limits, PCIe bandwidth, SRS preparation cost, sparse-column crossover, and memory available alongside NTT buffers?
- **OPEN QUESTION:** Do the inspected ledger/node checkouts match the deployed ledger-9/stagenet verifier pins? This node establishes no live-chain compatibility result.

## Links

- **PROPOSAL:** Obtain operation counts and MSM fractions from [[loki-01-prover-profile]]; share resident buffers and allocation budgets with [[loki-03-gpu-ntt]].
- **PROPOSAL:** Compare integration precedents in [[loki-04-halo2-gpu-precedents]] and acceptance predicates in [[loki-10-integration-verification]].
- **PROPOSAL:** Keep host acceleration distinct from recursive constraints in [[loki-05-recursion-accumulation]] and [[loki-06-recursive-circuit-cost]]; coordinate batches with [[loki-08-recursion-tree-scheduling]] and conversion costs with [[loki-09-cpu-witness-field]].
- **PROPOSAL:** Route proof-system replacements to [[loki-07-folding-alternatives]], preserving this node's unchanged-verifier boundary.
