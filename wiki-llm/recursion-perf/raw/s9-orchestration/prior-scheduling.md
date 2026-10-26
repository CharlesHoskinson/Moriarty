# Earlier scheduling study
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-08-recursion-tree-scheduling.md
> Collected: 2026-10-26
> Published: 2026-10-24

---
title: "Scheduling Midnight recursion on one CPU and RTX 5090"
project: loki
node: 08
author: gpt-6.1-sol
date: 2026-10-24
tags: [recursion, scheduling, gpu, memory, key-cache, doctorwatson]
sources:
  - https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html
  - https://docs.nvidia.com/cuda/wsl-user-guide/index.html
  - https://developer.nvidia.com/cuda/gpus
  - https://dev.ingonyama.com/3.0.0/icicle/primitives/msm
  - https://github.com/ingonyama-zk/icicle
  - https://github.com/supranational/sppark
  - https://eprint.iacr.org/2022/1321
  - https://arxiv.org/abs/2509.22684
  - https://www.zprize.io/prizes/accelerating-msm-operations-on-gpu-fpga
  - https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover
---

## Question

**PROPOSAL —** Schedule an already defined recursion DAG on the supplied 20-core, 90-GB WSL2 host and 32-GB RTX 5090, preserving the existing proof system, transcript, SRS, VK and ledger acceptance. Optimize completed, verified batches per second alongside single-batch latency.

## Summary

- **PROPOSAL —** Use one persistent Rust scheduler inside one DoctorWatson `heavy` invocation, sharing keys and SRS across jobs and owning one CUDA context.
- **PROPOSAL —** Overlap CPU synthesis, GPU arithmetic and verification; initially admit one heavy GPU proof plus one small proof. Enable two heavy proofs only after measuring their combined live allocations.
- **CODE —** Cache both VK and PK: the Sherlock parent currently calls `setup_pk` and drops it on each proof ([~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:1041](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:1041)).
- **CODE —** The available 16-transaction Fenrir topology is four four-transaction leaves → one four-child node → decider; its parent waits for all leaves ([~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/leaf.rs:41](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/leaf.rs:41), [~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:128](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:128)).
- **ESTIMATE —** Conditional warm planning cases give 370–640 seconds per isolated 16-transaction batch. Initial one-heavy-proof scheduling targets 280–640 seconds between completed batches; two-heavy-proof admission could improve that to 260–520 seconds. These are unmeasured assumptions and arithmetic below.

## Findings

### Acceptance and topology come first

**CODE —** Ledger verification calls `midnight_zk_stdlib::verify` with Blake2b and embedded KZG verifier parameters; the node's ledger-9 adapter forwards transaction validation to `Bridge` ([~/repos/midnight-ledger/transient-crypto/src/proofs.rs:64](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:64), [~/repos/midnight-ledger/transient-crypto/src/proofs.rs:120](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:120), [~/repos/midnight-ledger/transient-crypto/src/proofs.rs:554](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:554); [~/repos/midnight-node/ledger/src/host_api/ledger_9.rs:173](/home/charles/repos/midnight-node/ledger/src/host_api/ledger_9.rs:173)). The ledger checks the operation's installed VK and public statement ([~/repos/midnight-ledger/ledger/src/structure.rs:444](/home/charles/repos/midnight-ledger/ledger/src/structure.rs:444), [~/repos/midnight-ledger/ledger/src/structure.rs:468](/home/charles/repos/midnight-ledger/ledger/src/structure.rs:468)).

**CODE —** Sherlock's parent verifies two *different transition relations*, A102 then A80, under separately pinned keys, with a deferred pairing checked by the decider ([~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:3](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:3)). It is not an interchangeable binary transaction aggregator. Fenrir's `NodeRelation` accepts two through four leaf children and explicitly verifies them ([~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:45](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:45), [~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:67](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:67), [~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:130](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:130)).

**PROPOSAL —** Preserve the selected relation graph exactly. For the four-leaf Fenrir layout, overlap its parent with leaves of a later batch; a parent cannot start proving while one of its own required proofs is unfinished. Across dependent batches, prepare against the deterministically computed predecessor state, retain ledger submission order, and invalidate speculative descendants on a state mismatch. No scheduling change establishes that a Sherlock entrypoint accepts a 16-transaction Fenrir statement.

### Versions and reusable setup

**CODE —** Moriarty pins proofs `0.8.3`, circuits `7.2.4`, and stdlib `2.3.5` ([~/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14](/home/charles/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14)). Fenrir instead declares proofs `0.8.2` and a patched circuits copy ([~/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:25](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:25), [~/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:35](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:35)). The `31435e9` checkout already uses `commit_many` and labelled multicommitments ([~/repos/midnight-zk/proofs/src/poly/kzg/mod.rs:82](/home/charles/repos/midnight-zk/proofs/src/poly/kzg/mod.rs:82)); registry `0.8.3` uses `commit`/`commit_lagrange` ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74)).

**CODE —** Fenrir already constructs one leaf PK outside its sequential leaf loop, whereas each node invocation reconstructs a PK ([~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:349](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:349), [~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:354](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:354), [~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:462](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:462)). Moriarty's return-child prover likewise constructs its PK per call ([~/repos/Moriarty-factory/crates/mori-backend/src/sherlock_link.rs:736](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_link.rs:736)). Thus “cache keygen” must distinguish VK construction, PK construction, deserialization and residency.

**CODE —** `MidnightPK::write/read` already serialize the relation and underlying PK ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1635](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1635), [~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1651](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1651)). Reading reconstructs fixed polynomial transforms and an evaluator, so a disk PK cache is not free setup ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/mod.rs:416](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/mod.rs:416)). SRS downsizing recomputes the Lagrange basis rather than taking its prefix ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/params.rs:40](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/params.rs:40)).

### Primary-source constraints on GPU scheduling

**SOURCE —** NVIDIA documents overlap through pinned buffers and separate non-default streams, conditional on device capabilities; multiple CUDA contexts add memory and switching overhead. [CUDA guide](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html), [capture: raw/08/cuda-best-practices.md](../raw/08/cuda-best-practices.md).

**SOURCE —** WSL has limited pinned-memory availability and restrictions on managed memory and concurrent CPU/GPU access. [WSL guide](https://docs.nvidia.com/cuda/wsl-user-guide/index.html), [capture: raw/08/cuda-wsl.md](../raw/08/cuda-wsl.md). NVIDIA lists RTX 5090 at compute capability 12.0. [GPU table](https://developer.nvidia.com/cuda/gpus), [capture: raw/08/nvidia-compute-capability.md](../raw/08/nvidia-compute-capability.md).

**SOURCE —** ICICLE's versioned v3 MSM API provides streams, asynchronous execution, shared-base batching and precomputation; its memory model includes scalar indices and buckets as well as points. Treat this as a versioned API example. [MSM documentation](https://dev.ingonyama.com/3.0.0/icicle/primitives/msm), [capture: raw/08/icicle-msm.md](../raw/08/icicle-msm.md). Its repository documents explicit backend loading and device selection. [ICICLE](https://github.com/ingonyama-zk/icicle), [capture: raw/08/icicle-repository.md](../raw/08/icicle-repository.md).

**SOURCE —** sppark supplies CUDA/C++ MSM and NTT templates, including BLS12-381, with Rust integration considerations. [sppark](https://github.com/supranational/sppark), [capture: raw/08/sppark.md](../raw/08/sppark.md). cuZK explicitly reduces redundant transfers and overlaps transfers with device computation. [Paper](https://eprint.iacr.org/2022/1321), [capture: raw/08/cuzk.md](../raw/08/cuzk.md).

**SOURCE —** ZKProphet reports studied GPU configurations where NTT reaches 90% of proof latency after MSM optimization. [Paper](https://arxiv.org/abs/2509.22684), [capture: raw/08/zkprophet.md](../raw/08/zkprophet.md). Snarkify documents Halo2 GPU modules and benchmarks on a 64-core EPYC with four RTX 3090s. [Halo2 GPU prover](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover), [capture: raw/08/snarkify-halo2.md](../raw/08/snarkify-halo2.md).

**SOURCE —** ZPrize's cited category targets fastest MSM, rather than complete recursion-tree latency. [Prize specification page](https://www.zprize.io/prizes/accelerating-msm-operations-on-gpu-fpga), [capture: raw/08/zprize-msm.md](../raw/08/zprize-msm.md).

**PROPOSAL —** Use these as implementation precedents; establish Midnight/5090 service times independently. Prefer explicit allocations and a small reusable pinned ring over relying on managed-memory oversubscription.

## Proposed design

### Execution boundary and APIs

**PROPOSAL —** Choose persistent, typed DAG execution over either process-per-proof execution, which loses resident caches, or an unrestricted GPU proof swarm, which cannot enforce the memory envelope. Keep the public CPU prover available. Proposed internal modules:

| Module | Interface and responsibility |
|---|---|
| **PROPOSAL —** `tree_manifest` | `validate(manifest) -> Dag`; bind ordered children, transaction coverage, public instances, relation/VK/SRS identities, transcript and network entrypoint. |
| **PROPOSAL —** `key_store` | `acquire(KeyId) -> KeyLease<R>`; single-flight construction, disk persistence, resident `Arc` keys and device basis leases. |
| **PROPOSAL —** `witness_stage` | `prepare(JobId, inputs) -> WitnessHandle`; bounded input replay and synthesis preparation. |
| **PROPOSAL —** `admission` | `reserve(ResourceRequest) -> Permit`; atomically reserve CPU, host RAM, VRAM and queue capacity before allocation. |
| **PROPOSAL —** `gpu_executor` | `submit(KernelRequest, BufferLease, dependencies) -> Completion`; one CUDA owner, event-controlled lifetimes, bounded arithmetic queue. |
| **PROPOSAL —** `proof_worker` | `prove<R,H>(KeyLease<R>, witness, instance) -> ProofArtifact`; preserve the reference transcript writer and RNG semantics. |
| **PROPOSAL —** `artifact_store` | `publish_verified(artifact) -> ArtifactId`; atomic durable publication followed by DAG release. |

**PROPOSAL —** Data flow is `validated manifest → key lease → prepared witness → synthesis/arithmetic phases → CPU verification → durable proof → ready parents → decider → ledger verification`. Scheduling metadata stays outside proof bytes. Mutable circuits, witnesses and RNGs belong to individual jobs; share only structures whose Rust `Send + Sync` bounds are established.

### Concrete prover hooks

**CODE —** The current wrapper enters `BlstPLONK::prove` through stdlib `prove<R,H>` ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1984](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1984)). Registry `create_proof` separates private trace computation and finalization ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:348](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:348), [~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:371](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:371)). These are instrumentation boundaries, not existing resumable APIs.

**PROPOSAL —** First retain `KZGCommitmentScheme<Bls12>` and dispatch its `PolynomialCommitmentScheme::{commit, commit_lagrange}` implementations to the GPU executor, preserving CPU fallback ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74), [~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:83](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:83)). This avoids replacing the concrete PCS type throughout stdlib. Keep `multi_open` algebra and serialization unchanged, ensuring its commitments reach the same backend.

**PROPOSAL —** Batch independent advice-column commitments where `parse_advices` currently collects them; reorder completed GPU results back into the original column order before `transcript.write` ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:568](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:568)). Pool arithmetic across jobs without combining their cryptographic transcripts or instances.

**PROPOSAL —** Add NTT dispatch at `EvaluationDomain::{lagrange_to_coeff, coeff_to_lagrange, coeff_to_extended, extended_to_coeff}`; preserve roots, inverse normalization and coset factors ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:224](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:224), [~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:251](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:251)). A later device-resident polynomial layer can eliminate repeated transfers; initially account for every copy. Coordinate kernel details with [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]].

**CODE —** Local CUDA Rust exports streams, events, device buffers and pinned host buffers; stream `wait` provides cross-stream event ordering ([~/repos/nvidia-cuda-rust/cuda-core/src/lib.rs:34](/home/charles/repos/nvidia-cuda-rust/cuda-core/src/lib.rs:34), [~/repos/nvidia-cuda-rust/cuda-core/src/simt/stream.rs:192](/home/charles/repos/nvidia-cuda-rust/cuda-core/src/simt/stream.rs:192)). The local halo2 checkout has its own `create_proof` signature ([~/repos/halo2/src/plonk/prover.rs:54](/home/charles/repos/halo2/src/plonk/prover.rs:54)). Neither that checkout nor Midnight's newer `commit_many` API should be silently substituted into the pinned integration.

### Cache lifecycle

**PROPOSAL —** Define `KeyId = H(serialized relation, circuit/configuration version, k, architecture, child VK hashes, fixed constants, transcript/challenge mode, SRS hash, dependency versions/features)`. Use full equality of canonical identity material behind the digest. Distinct A102/A80 relations, arities and templates receive distinct keys. A filename or caller-supplied hash cannot establish a trusted VK.

**PROPOSAL —** On trusted cache enrollment, reconstruct and compare the expected VK, then build/persist the PK once. Warm jobs reuse resident keys; restarts read checked PK bytes, compare their VK to the enrolled pin, and reconstruct required transforms. Pin active leases; evict unleased LRU entries under a byte cap. Retain both coefficient and Lagrange bases per domain; never prefix a larger Lagrange array. Cache setup failure once per identity and require an explicit retry hypothesis.

**CODE —** Current child verification independently rebuilds the statement and key ([~/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:429](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:429)). **PROPOSAL —** Preserve that trust boundary through independently validated cache enrollment and per-request identity checks; do not replace it with trusting an artifact's own VK. Whether changing programs share a key remains a relation-specific question.

### Queues, CPU split and back-pressure

**PROPOSAL —** Start with 12 shared Rayon prover threads, four witness-preparation threads, two verification threads, and two control/CUDA/I/O threads: `12 + 4 + 2 + 2 = 20`. Reclaim idle preparation capacity only through explicit admission. Avoid a separate 20-thread Rayon pool per proof. Keep GPU completion handling outside blocked Rayon work so queued completions cannot deadlock behind waiting tasks.

**PROPOSAL —** Bound prepared witnesses to four and GPU-resident proof contexts to two. Other ready jobs remain small descriptors or durable artifacts. Schedule by oldest-batch remaining critical path, preferring a ready decider/parent, while aging leaves to prevent starvation. Batch compatible commitments by SRS/domain/basis, initially at most four vectors and at most 2 ms collection delay; dispatch immediately if no compatible work is ready. Tune these proposed values using traces.

**PROPOSAL —** Acquire resources atomically before synthesis; include the next phase's peak, not just its input buffer. A waiting job holds no partial reservation. At memory pressure, stop witness production, evict inactive caches, then reduce concurrency. Never make forward progress depend on admission of a third proof. Yield GPU ownership only at safe kernel/event boundaries; stream priority cannot promise preemption of an already running kernel.

### Memory envelope

**SOURCE —** The supplied brief reports a k20 parent attempt exceeding 500 seconds at approximately 12 GiB host RSS and a roughly 120-second VK build. These are owner observations, not new measurements or GPU-memory measurements.

**ESTIMATE —** Midnight's scalar uses four 64-bit Montgomery limbs ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fq.rs:25](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fq.rs:25)). Thus a k20 polynomial is `2^20 × 32 = 32 MiB`; an assumed eightfold extended domain is 256 MiB. Keeping two base-sized representations plus one extended representation for 24 columns costs `24 × 32 × (2+8) / 1024 = 7.5 GiB`; 40 columns cost 12.5 GiB, before key polynomials and MSM scratch. The extension factor and live columns must come from the actual VK/trace.

**ESTIMATE —** Assuming a packed 96-byte affine device point, both unexpanded k20 SRS bases require `2 × 2^20 × 96 / 2^20 = 192 MiB`; factor-eight tables require approximately 1.5 GiB. Precomputation does not include MSM buckets/indices. A 12-GiB host peak cannot justify two GPU contexts fitting in 24 GiB.

| Resource | Initial admission policy |
|---|---|
| **PROPOSAL —** Host | `min(64 GiB, initial MemAvailable − 20 GiB)`; for a full 64-GiB allowance: 16 keys/SRS + 30 proof workspaces + 8 witnesses + 2 I/O/pinned + 8 margin. |
| **ESTIMATE —** Parent host lease | `12 GiB × 1.25 = 15 GiB`, provisional; measured larger shapes override it. |
| **PROPOSAL —** Device | `min(28 GiB, free VRAM after context creation − 2 GiB)`; query actual bytes rather than equating advertised GB with GiB. |
| **PROPOSAL —** GPU contexts | Initial envelope: 4 GiB shared cache + 16 GiB heavy workspace + 6 GiB small workspace + 2 GiB allocator/staging = 28 GiB. Admit fewer jobs if the measured cap is smaller. |
| **PROPOSAL —** Two heavy proofs | Require measured per-job peaks ≤10 GiB: `4 + 2×10 + 2 = 26 GiB`; otherwise serialize heavy GPU phases. |
| **PROPOSAL —** Pinned ring | Start with two 64-MiB buffers, included in host I/O allowance; shrink or disable transfer overlap if allocation fails. |

**PROPOSAL —** Workspace reservations include resident PK polynomials, extended vectors, MSM scratch, conversion and staging buffers; shared-cache accounting includes only genuinely shared allocations. Admit unprofiled deciders alone. Multiple resident proofs are an overlap opportunity, not an assumption that two saturated kernels run twice as fast.

### Streaming and recovery

**CODE —** `parse_advices` allocates complete advice and rational columns, synthesizes a phase, batch-inverts, blinds, commits and then advances challenges ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:492](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:492), [~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:517](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:517), [~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:540](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:540)). **PROPOSAL —** Stream input replay and leaf witness preparation first; do not describe this as constant-memory streaming of the existing prover. A later column/phase sink needs a separate proof of layout and challenge equivalence. NTT still needs a complete domain or an explicit external-memory algorithm.

**PROPOSAL —** Checkpoint completed, independently verified leaf/node proofs. Store proof bytes, ordered public instance, child artifact hashes, key/SRS/transcript identities and batch state digest in a content-addressed directory; fsync temporary files and publish the manifest atomically. Resume by validating dependencies and re-verifying artifacts. Private witnesses stay outside public evidence.

**PROPOSAL —** Restart an interrupted proof with fresh randomness; reuse completed descendants. Do not checkpoint mid-transcript initially: safely resuming would require sensitive witness state, exact RNG position, transcript state and phase buffers, with a larger audit surface.

### DoctorWatson integration

**CODE —** `heavy` holds an exclusive lock and launches its child in a process group; capped execution samples group RSS and kills that group on overrun ([~/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:74](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:74), [~/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:119](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:119), [~/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:211](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:211)). The uncapped path primarily reports `wait4` peak RSS ([~/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:106](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:106), [~/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:143](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/heavy.py:143)). GPU allocations are absent from its RSS accounting.

**PROPOSAL —** A future command is `scripts/doctorwatson heavy --id LOKI08 --budget loki-tree16-warm -- target/release/sherlock-tree-worker run --manifest batch16.json`. The ID, budget and executable are proposed. Keep all concurrency inside this child, using a separate warm-tree budget rather than the existing single-parent budget. The scheduler enforces VRAM and host totals even on the initial uncapped measurement and emits queue depths, phase timings, cache hits, CPU tokens, peak allocations and artifact hashes to the existing trace. Keep children in the process group; never nest `heavy`. Follow `--hypothesis` retries and shrink after two equivalent failures ([~/repos/Moriarty-factory/AGENTS.md:17](/home/charles/repos/Moriarty-factory/AGENTS.md:17)).

## Expected speedup

**ESTIMATE —** For a *planning proxy*, take reference parent time `P=500 s`, although the observed attempt only establishes `P>500 s`. Assume 20% CPU remainder, 55% MSM at 15× acceleration, 25% NTT at 8×, plus six seconds transfer/verification overhead: `P_gpu = 100 + 275/15 + 125/8 + 6 ≈ 140 s`, approximately `500/140 = 3.57×`. This decomposition is unprofiled; [[loki-01-prover-profile]] must replace it.

**ESTIMATE —** Model four four-transaction leaves, one node and one decider. Assume each leaf takes 60 s reference or 25 s accelerated; decider takes 360 s reference or 130 s accelerated. Neither leaf nor decider time is observed here. Introduce unknown multipliers `aP,aD` for the actual four-child node and matching decider: `T_ref=4×60+500aP+360aD`; conservative serial accelerated latency `T_gpu=4×25+140aP+130aD`, excluding setup and ledger submission.

| Planning case | Warm reference | Warm accelerated | Batch transaction throughput |
|---|---:|---:|---:|
| **ESTIMATE —** `aP=aD=1` | 1,100 s | 370 s | `16/370 = 0.0432 tx/s = 2.59 tx/min`; speedup `1100/370 = 2.97×`. |
| **ESTIMATE —** `aP=aD=2` | 1,960 s | 640 s | `16/640 = 0.025 tx/s = 1.50 tx/min`; speedup `1960/640 = 3.06×`. |

**OPEN QUESTION —** Does the four-child relation fit k20? Its cost may exceed even the second planning case. Neither row is a confidence interval or an accepted stagenet result.

**ESTIMATE —** For pipeline capacity, partition the assumed accelerated leaf into 10 CPU + 12 GPU + 3 other seconds; parent into 100 CPU + 34 GPU + 6 other; decider into 80 CPU + 45 GPU + 5 other. Treat all CPU/other work conservatively as one shared service lane at the stated CPU allocation. At `aP=aD=1`, CPU demand is `4×13+106+85=243 s/batch`; GPU demand is `4×12+34+45=127 s/batch`. Initial admission also serializes heavy jobs for `140+130=270 s/batch`. Thus the interval is at least `max(243,127,270)=270 s`; a provisional 280–320 s target gives `960/320` through `960/280 = 3.00–3.43 tx/min`.

**ESTIMATE —** If both parent and decider stage demands double, CPU demand becomes `52+212+170=434 s`, GPU demand `48+68+90=206 s`, and serialized heavy residency costs `280+260=540 s`. A provisional 560–640 s interval gives `960/640` through `960/560 = 1.50–1.71 tx/min`. If measured workspaces permit two heavy contexts, the residency restriction can relax: targets become 260–300 s in the first case (`3.20–3.69 tx/min`) or 450–520 s in the doubled case (`1.85–2.13 tx/min`). These capacity bounds are necessary, not sufficient: dependencies, memory and CPU-pool contention can prevent the target. They require independent or valid speculative next-batch work; the first batch still pays pipeline fill.

**ESTIMATE —** Reusing one shape's 120-s VK across ten repeated parent jobs saves `10×120−120=1,080 s` against rebuilding it each time. This is a counterfactual, not a claim that current Fenrir rebuilds a VK per leaf. Warm resident PK reuse saves a separately measured setup cost. Cold latency adds distinct leaf/node/decider setup, PK reconstruction, SRS loading and device upload; their sum is currently unknown.

## Risks and soundness concerns

**PROPOSAL —** Preserve commitment encodings, field conversions, transcript order/challenges, blinding and SRS exactly. Backend replacement needs no proof-format or VK change when it computes the same algebra. Reject stale or foreign cache identities before using keys.

**CODE —** Accumulation is explicitly implemented in the verifier gadget ([~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:211](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:211)); Sherlock defers the pairing to its terminal decider ([~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:13](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:13)). **PROPOSAL —** Scheduling must preserve subgroup checks, pairing checks, ordered coverage and state continuity. GPU completion alone never releases a parent.

**PROPOSAL —** Changing fan-out, leaf transaction count or recursion relation generally changes circuit keys and the terminal decider VK even if the proof wire format stays unchanged; an installed verifier would require the corresponding contract/key update and validation. Switching to a folding scheme or different PCS changes proof semantics/format and needs a compatible terminal wrapper or verifier change. It is outside this scheduler's acceptance-preserving scope.

**CODE —** Midnight's newer aggregator requires inner circuits to share a constraint system, domain and SRS ([~/repos/midnight-zk/aggregation/src/multi_circuit_aggregator/mod.rs:7](/home/charles/repos/midnight-zk/aggregation/src/multi_circuit_aggregator/mod.rs:7)). **PROPOSAL —** Do not treat it as a ready replacement for Sherlock's heterogeneous children. GPU memory races, asynchronous buffer reuse and incorrect Montgomery conversion remain correctness risks even when the mathematical protocol is unchanged.

## Validation plan

1. **PROPOSAL —** Establish phase traces and exact identities on the reference prover first; record VK/PK setup, witness/synthesis, commitments, NTT, quotient/opening, verification, live host/device memory and cold/warm states. Run future heavy measurements through DoctorWatson; none are performed for this research note.
2. **PROPOSAL —** Differentially test BLS12-381 MSM and every domain transform against registry `0.8.3`, including zero/one scalars, identity points, incomplete batches, inverse normalization and cosets. Test simultaneous jobs with intentionally varied completion order and buffer-lifetime checks.
3. **PROPOSAL —** With identical test-only RNG streams, compare intermediate commitments/challenges and aim for byte-identical proofs. With production randomness, require acceptance by the unchanged reference verifier under identical instances, SRS and VK; byte inequality alone is not failure. Test reference/GPU children in mixed combinations through parent and decider.
4. **PROPOSAL —** Require named predicates for `all_16_transactions_once_in_order`, `child_state_links`, `cache_identity_match`, `terminal_pairing`, `reference_verifier_accepts`, `resource_caps_hold`, and `restart_reuses_only_verified_artifacts`. Mutate a child proof, instance, order, key, SRS identity and accumulator; each affected chain must reject.
5. **PROPOSAL —** Inject cancellation and crashes at artifact publication and CUDA boundaries. Verify atomic recovery, no partial-proof reuse, bounded queues and allocation permits. Then compare identical 16-transaction workloads for isolated latency and steady throughput, including completed ledger verification and final stagenet acceptance under the installed key. Keep a performance result distinct from the acceptance predicate; coordinate with [[loki-10-integration-verification]].

## Open questions

- **OPEN QUESTION —** What are the actual arity-four node/decider k values, column counts, PK sizes and service times?
- **OPEN QUESTION —** Which selected ICICLE/sppark version passes exact field, encoding and SM 12.0 checks with CUDA 13.4 on this WSL installation?
- **OPEN QUESTION —** Which Sherlock fixed data varies between transactions, preventing key reuse across apparently similar jobs?
- **OPEN QUESTION —** Can the ledger workflow supply stable next-batch witnesses early enough to hide parent/decider time without excessive invalidation?
- **OPEN QUESTION —** Is synthesis, polynomial traffic or GPU integer arithmetic the limiting service after acceleration?

## Links

**PROPOSAL —** Use [[loki-01-prover-profile]] for measured stage demands; [[loki-02-gpu-msm]], [[loki-03-gpu-ntt]] and [[loki-04-halo2-gpu-precedents]] for arithmetic/backend integration; [[loki-05-recursion-accumulation]], [[loki-06-recursive-circuit-cost]] and [[loki-07-folding-alternatives]] for relation costs and protocol boundaries; [[loki-09-cpu-witness-field]] for synthesis; and [[loki-10-integration-verification]] for ledger acceptance. The ten primary-source captures linked above are stored exclusively under `raw/08/`.
