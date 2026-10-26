# Earlier CPU witness study
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-09-cpu-witness-field.md
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Local source snapshot; SHA-256 `cf4f2a4c9e315a4a7f422b91118eaf5e953377fb3a595e7c783b0ba97fdc9f44`. Earlier study claims remain historical, not new measurements.

```text
---
title: "CPU witness, field arithmetic, and prover service optimizations"
project: loki
node: 09
author: gpt-6.1-sol
date: 2026-10-24
tags: [loki, cpu, witness, field-arithmetic, blst, rayon, allocation, wasm]
sources:
  - https://github.com/midnightntwrk/midnight-zk
  - https://github.com/supranational/blst
  - https://raw.githubusercontent.com/supranational/blst/master/bindings/blst.h
  - https://raw.githubusercontent.com/zkcrypto/ff/main/src/batch.rs
  - https://github.com/rayon-rs/rayon
  - https://docs.rs/rayon-core/latest/rayon_core/struct.ThreadPoolBuilder.html
  - https://github.com/RReverser/wasm-bindgen-rayon
  - https://github.com/mitschabaude/montgomery/blob/main/doc/zprize22.md
  - https://github.com/SimonLangowski/VROOM
  - https://eprint.iacr.org/2025/1068
  - https://eprint.iacr.org/2022/1321
  - https://github.com/ingonyama-zk/icicle
  - https://github.com/supranational/sppark
---

## Question

PROPOSAL — Identify CPU improvements that shorten Sherlock leaf/child → k20 parent → terminal decider proving, also improving GPU feeding, while preserving the existing ledger verifier, transcript, SRS, VK, and public statement. This is a research/design result; validation below is future work.

## Summary

- **CODE — BLST is already present.** Midnight calls BLST for both BLS scalar and base arithmetic; its scalar is named `Fq`, while its base field is `Fp`. (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/mod.rs:31`, `src/bls12_381/fq.rs:333`, `src/bls12_381/fp.rs:501` under that same crate.)
- **CODE — Witness handling has concrete costs:** all advice columns are allocated for each phase, region closures execute twice, and stdlib synthesis clones instance/witness values. (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:520`, `src/circuit/floor_planner/single_pass.rs:89`; `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1880`.)
- **PROPOSAL — First optimize lazy inversion, sparse denominator handling, reusable buffers, and persistent keys.** Preserve row placement and all constraints; defer broader layout changes.
- **CODE — Rayon and BLST have separate pools.** Wrapping already parallel BLST MSMs in more Rayon tasks requires an explicit CPU budget. (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/utils/multicore.rs:1`; `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blst-0.3.17/src/lib.rs:35`.)
- **ESTIMATE — A conditional CPU target is 1.11–1.40× steady-state improvement**, from the Amdahl calculations below. The reported >500 s attempt is censored, not a measured completed-proof baseline.

## Findings

### Evidence and compatibility boundary

CODE — Citation abbreviations below expand exactly to these local roots: `P` = `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3`; `C` = the same registry directory plus `/midnight-curves-0.3.1`; `G` = `/midnight-circuits-7.2.4` there; `Z` = `/midnight-zk-stdlib-2.3.5` there; `B` = `/blst-0.3.17` there. `F` = `/home/charles/repos/projectfenrir/recursion/stagenet-strawman`; `L` = `/home/charles/repos/midnight-ledger`; `M` = `/home/charles/repos/Moriarty-factory`; `K` = `/home/charles/repos/midnight-zk`. Citations use `root/relative-path:line`.

CODE — The checkout at the supplied `31435e9` source cut declares proofs 0.8.0, circuits 7.1.0, and stdlib 2.2.0; Moriarty's lock resolves the requested 0.8.3/7.2.4/0.3.1/2.3.5 stack. Separately, recl9 explicitly pins proofs **0.8.2** and a patched circuits directory. These are distinct source surfaces, not interchangeable build artifacts. (`K/proofs/Cargo.toml:3`, `K/circuits/Cargo.toml:3`, `K/zk_stdlib/Cargo.toml:3`; `M/Cargo.lock:1805`, `:1873`, `:2031`, `:2272`; `F/recl9/Cargo.toml:25`, `:35`.)

CODE — Parent proving uses Poseidon, whereas the terminal decider uses Blake2b. The ledger selects the operation VK and verifies with its own parameters; transient crypto fixes its transcript hash to Blake2b. StrawAnchor's source describes replacing the compiled `acceptValidity` key with the handwritten decider key. (`F/recl9/src/sherlock_link.rs:1042`; `F/recl9/src/main.rs:603`; `L/transient-crypto/src/proofs.rs:64`; `L/ledger/src/structure.rs:444`, `:468`; `F/chain/anchor/straw_anchor.compact:1`, `:54`.)

PROPOSAL — Preserve each layer's existing protocol separately. Identical mathematical field results and assignment locations can retain the VK and proof format; changing circuit shape requires new keys and, for the terminal circuit, replacing the installed operation key. Changing transcript, opening protocol, or proof system requires compatible recursive verification and potentially ledger changes; that is outside this node's quick-win scope.

### Field arithmetic: assembly exists; AVX-512 is unavailable here

CODE — `Fq` wraps four Montgomery `u64` limbs in `blst_fr`; `Fp` wraps six in `blst_fp`. Multiplication and inversion call `blst_fr_mul`, `blst_fr_eucl_inverse`, `blst_fp_mul`, and `blst_fp_eucl_inverse`. This is not a pure Rust scalar-arithmetic baseline. (`C/src/bls12_381/fq.rs:22`, `:333`, `:409`; `C/src/bls12_381/fp.rs:93`, `:501`, `:599`.)

SOURCE — BLST provides C/assembly implementations and ADX/non-ADX variants; its public API exposes both Fr and Fp arithmetic. ([BLST](https://github.com/supranational/blst), capture [blst.md](../raw/09/blst.md); [API header](https://raw.githubusercontent.com/supranational/blst/master/bindings/blst.h), capture [blst-api.md](../raw/09/blst-api.md).)

CODE — BLST's build script selects assembly on x86-64, detects host ADX when no `target-cpu` is supplied, honors target features when it is supplied, and distinguishes `portable` from `force-adx`. Portable mode disables ISA extensions; it does **not** imply that all assembly disappears. `ci_build` enables Midnight's portable feature. (`B/build.rs:148`, `:160`, `:181`, `:203`; `P/Cargo.toml:176`; `G/Cargo.toml:137`; `Z/Cargo.toml:220`.)

CODE — The inspected WSL guest reports Core Ultra 9 285K, 20 cores, ADX/BMI2/AVX2, and no AVX-512 flags. BLST also requests `-mno-avx` for its compiled C path. (`/proc/cpuinfo:5`, `:13`, `:20`; `B/build.rs:219`.)

SOURCE — VROOM's production CPU implementation requires AVX-512 IFMA. The related RNS paper's approximately 4× CPU result concerns **1024+-bit primes**. ([VROOM](https://github.com/SimonLangowski/VROOM), capture [vroom.md](../raw/09/vroom.md); [RNS paper](https://eprint.iacr.org/2025/1068), capture [vector-rns.md](../raw/09/vector-rns.md).)

PROPOSAL — Audit the actual binary's build log/features and disassembly before proposing flags. Compare an optimized native build with current settings later; a BLST deployment already using ADX gets no “enable ADX” dividend. Do not transfer the AVX-512/large-prime results to this host. Prototype batched FFI/AVX2 only after profiling demonstrates scalar-call overhead; preserve Montgomery representation and zero semantics.

### Synthesis and inversions

CODE — `parse_advices` loops phases and circuits serially, allocating `empty_lagrange_rational()` for **every** advice column in each phase, then discarding inactive columns before conversion. Inactive assignments return before calling their value closure. Polynomial output storage is also allocated before synthesis. (`P/src/plonk/prover.rs:492`, `:501`, `:515`, `:520`, `:540`, `:858`.)

CODE — `SingleChipLayouter::assign_region` invokes its closure for region shape and again for assignment. However, shape collection ignores the actual `to` closure. Code inference: moving expensive calculations into that closure avoids evaluating them during shape collection and inactive phases, while retaining the region's structural work. (`P/src/circuit/floor_planner/single_pass.rs:89`, `:109`; `P/src/circuit/layouter.rs:222`; `P/src/circuit/mod.rs:288`.)

CODE — Concrete eager inversions occur in `NativeChip::assign_with_shifted_inverse`, `assert_not_equal`, and equality auxiliary witnesses. The first two calculate inside the twice-invoked region closure; equality calculates before it. The hardened recl9 copy retains these sites. `AssignedNative<F>` is specifically `AssignedCell<F,F>`, so universally substituting rational cells would change APIs. (`G/src/field/native/native_chip.rs:367`, `:377`, `:774`, `:1253`; `G/src/field/mod.rs:23`; `F/verifier-patch/midnight-circuits/src/field/native/native_chip.rs:377`, `:1253`.)

PROPOSAL — First make existing F-valued inverse suppliers lazy. Next, change **discarded auxiliary assignments**, such as `assert_not_equal`'s `r`, to `Value<Rational<F>>`: encode inverse as `Rational(ONE, delta)` and let the backend batch it. For equality's auxiliary, preserve the existing value ONE when delta is zero; plain `1/0` would instead evaluate to zero. Leave returned inverse cells F-valued until their consumers have a separate design.

CODE — Backend rational conversion already batch-inverts nontrivial denominators, but first materializes dense `Vec<Option<F>>` arrays and then multiplies every numerator, including trivial/zero entries, by an inverse or ONE. Lookup and permutation products also batch-invert serially between parallel loops. (`P/src/poly/mod.rs:287`, `:319`; `P/src/utils/rational.rs:289`; `P/src/plonk/lookup/prover.rs:184`, `:196`; `P/src/plonk/permutation/prover.rs:94`, `:106`.)

SOURCE — `ff` supplies a zero-preserving nonallocating `BatchInverter` with external scratch; its iterator version allocates temporary storage. ([Batch inversion implementation](https://raw.githubusercontent.com/zkcrypto/ff/main/src/batch.rs), capture [ff-batch.md](../raw/09/ff-batch.md).)

PROPOSAL — Compact rational denominators with their destination indices, reuse prefix scratch, copy trivial values, and emit zero directly. Parallelize sufficiently large independent chunks; each chunk pays one inversion. Preserve `x/0 → 0` and unknown-witness errors. For future GPU work this produces contiguous field buffers without requiring witness synthesis on the GPU.

### Cloning, allocation, and parallelism

CODE — Stdlib synthesis clones `instance` and `witness`; recl9 clones child-proof byte vectors into verifier inputs and decider witnesses. Chip clones can instead share `Rc<RefCell<...>>` state, and field-valued `AssignedCell` clones copy a value/cell record. Thus clone counts alone do not measure allocation cost. (`Z/src/lib.rs:1876`; `F/recl9/src/node.rs:125`, `:129`; `F/recl9/src/decider.rs:159`; `G/src/field/native/native_chip.rs:117`; `P/src/circuit/mod.rs:103`.)

PROPOSAL — Share immutable proof bytes and large witness payloads at relation boundaries; keep mutable assignment state local. Count bytes cloned, allocation size, and live capacity. Replace repeated large allocations with bounded worker-owned scratch before considering a global allocator change. Cache measured region shapes only if their exact rows, columns, constants, and copies are checked on replay.

CODE — Existing parallelism includes FFT calls, polynomial arithmetic through `parallelize`, and Rayon exports. Advice commitments nevertheless iterate serially. BLST MSM internally takes a distinct global `threadpool::ThreadPool`, so `RAYON_NUM_THREADS` cannot bound all prover threads. (`P/src/poly/domain.rs:241`, `:369`; `P/src/utils/arithmetic.rs:142`; `P/src/plonk/prover.rs:568`; `B/src/lib.rs:41`; `B/src/pippenger.rs:141`.)

SOURCE — Rayon supports custom pools and defaults according to its configured thread count/environment/available CPUs. ([Rayon](https://github.com/rayon-rs/rayon), capture [rayon.md](../raw/09/rayon.md); [pool API](https://docs.rs/rayon-core/latest/rayon_core/struct.ThreadPoolBuilder.html), capture [rayon-pool.md](../raw/09/rayon-pool.md).)

CODE — The inspected 0.8.3 MSM dispatch already always selects BLST for BLS G1; the checkout still has a size cutoff. Its curve wrapper repeatedly converts projective bases to affine and allocates canonical scalar bytes. Recommending removal of that cutoff as new 0.8.3 work would duplicate the inspected behavior. (`P/src/poly/kzg/msm.rs:176`; `K/proofs/src/poly/kzg/msm.rs:187`; `C/src/bls12_381/g1.rs:628`.)

PROPOSAL — Coordinate jobs, Rayon, and BLST instead of blindly parallelizing columns. Cache affine SRS bases by exact SRS digest/domain/order, before scalar-dependent filtering, and reuse scalar-byte storage. This reduces CPU preparation for both CPU and GPU commitments; [[loki-02-gpu-msm]] owns the MSM kernel decision.

### Proof server and WASM

CODE — `/prove` deserializes requests, clones optional key material, constructs a current-thread Tokio runtime per job, and calls versioned IR proving. Version detection parses IR before `ProofPreimage::prove` parses it again. Work already runs under `spawn_blocking`; moving it there is not a missing optimization. The server defaults to two workers. (`L/proof-server/src/endpoints.rs:251`, `:262`, `:265`; `L/proof-server/src/versioned_ir.rs:89`; `L/transient-crypto/src/proofs.rs:747`; `L/proof-server/src/worker_pool.rs:236`; `L/proof-server/src/main.rs:44`.)

CODE — There is already a five-entry, hash-keyed parsed-PK cache. Cache hits require hashing serialized bytes; misses clone compressed bytes, gzip-decompress, and read the key. The Sherlock parent helper instead calls `setup_pk` and drops it per invocation; Moriarty's literal helper calls keygen per invocation. These require different caching interventions. (`L/transient-crypto/src/proofs.rs:249`, `:259`, `:287`; `F/recl9/src/sherlock_link.rs:1041`; `M/crates/mori-backend/src/sherlock_machine/literal_bridge.rs:428`.)

CODE — WASM proving copies `Uint8Array` input into a Rust vector and serializes/copies output back. The ledger's wasm profile optimizes for size (`opt-level="z"`); its multithread demo explicitly exports and initializes `initThreadPool`. (`L/zkir-wasm/src/lib.rs:145`, `:155`; `L/Cargo.toml:59`; `L/wasm-proving-demos/zkir-mt/src/lib.rs:14`; `L/wasm-proving-demos/webpage/src/workerMt.js:20`.)

SOURCE — Default Rayon WebAssembly execution falls back to sequential work; wasm-bindgen-rayon requires worker/shared-memory setup, cross-origin isolation, and pool initialization. A ZPrize BLS12-381 WASM MSM submission reports 5–8× over its contest reference using specialized arithmetic/layout, not Midnight's complete prover. ([Rayon](https://github.com/rayon-rs/rayon), [capture](../raw/09/rayon.md); [adapter](https://github.com/RReverser/wasm-bindgen-rayon), [capture](../raw/09/wasm-rayon.md); [submission](https://github.com/mitschabaude/montgomery/blob/main/doc/zprize22.md), [capture](../raw/09/zprize-wasm.md).)

PROPOSAL — Prefer native proving for the large recursion workload. For browser deployments, compare a throughput-oriented profile and verified worker initialization separately from copy costs. Do not attribute a 12 GiB native workload's time to HTTP or WASM without showing it traversed that path.

## Proposed design

PROPOSAL — Implement in this order, behind independently selectable features:

1. **Persistent context:** `ProverContext::load(ArtifactIdentity) -> Arc<ProverContext>` owns pinned params, VK, PK, parsed relation, and immutable affine SRS. Identity includes source/dependency digests, protocol features, transcript, k, SRS digest, relation/configuration, and VK digest. `prove(context, instance, witness, rng)` retains current statement checks. Replace per-call `setup_pk` in `prove_link_parent`; for proof-server requests cache typed parsed material after its existing validation, with byte-based eviction and single-flight loading.
2. **Witness workspace:** `WitnessWorkspace<F>` owns phase-column slots, denominator/index buffers, prefix scratch, and final field buffers. Hook `parse_advices`/`WitnessCollection::assign_advice`; represent inactive columns as absent storage while preserving their global indices and the early phase return. Keep `Circuit`, `FloorPlanner`, `Layouter`, and circuit configuration unchanged.
3. **Conversion module:** `evaluate_rationals_into(input, output, scratch, policy)` replaces `batch_invert_rational` internally. Partition by public chunk boundaries, preserve zero denominators, and use reusable `ff::BatchInverter` scratch. Apply measured chunking separately to lookup/permutation inversion; do not reorder their recurrence or transcript operations.
4. **Circuit suppliers:** lazily evaluate current inversions first; then defer only auxiliaries whose rational values need not be exposed as `AssignedNative<F>`. Store large immutable witness arrays/proofs behind shared ownership. Preserve region closures' assignment sequence and fixed/selector/copy effects.
5. **Scheduling/telemetry:** a bounded native executor owns admission, workspace capacity, and an explicit Rayon pool. Initially allow one k20 job; tune concurrency only after measuring BLST's separate pool. Add spans for key lookup/hash/decompression, configure/shape/assignment, rational conversion, scalar encoding, commitments, FFT, quotient, opening, and local verification.

CODE — The relevant entrypoint is `plonk::create_proof`, which calls `compute_trace` and `finalise_proof`; commitment acceleration fits `PolynomialCommitmentScheme::{commit,commit_lagrange}`. These existing seams avoid changing transcript serialization. (`P/src/plonk/prover.rs:348`, `:371`; `P/src/poly/commitment.rs:44`, `:48`.)

PROPOSAL — Data flow: request → validated cached context → circuit synthesis into phase workspace → compact/deferred inversion → contiguous `Fq` columns → existing commitments/FFT/opening → unchanged ordered transcript → reference verification → ledger-bound proof envelope. Device implementations can later consume the same completed columns; CPU mode remains the differential oracle.

SOURCE — ICICLE documents a built-in CPU backend and BLS12-381 support; sppark describes CUDA/C++ proof-generation primitives. cuZK explicitly addresses redundant host/device transfer and overlap. ([ICICLE](https://github.com/ingonyama-zk/icicle), [capture](../raw/09/icicle.md); [sppark](https://github.com/supranational/sppark), [capture](../raw/09/sppark.md); [cuZK](https://eprint.iacr.org/2022/1321), [capture](../raw/09/cuzk.md).)

CODE — NVIDIA CUDA Rust separates host runtime/buffers/streams from device kernels; its README calls the project early-stage. (`/home/charles/repos/nvidia-cuda-rust/README.md:8`, `:23`, `:33`.)

PROPOSAL — Keep optional accelerator integrations outside CPU witness APIs. The cited accelerator sources motivate ownership/backend seams; they do not establish drop-in Midnight compatibility or this workload's speedup.

## Expected speedup

ESTIMATE — All fractions below are hypotheses requiring profiling. For a hypothetical **completed** 500 s steady-state proof, a conservative split of 15% synthesis improved 1.5×, 10% conversion/allocation improved 2×, and 75% unchanged gives `500 × (0.15/1.5 + 0.10/2 + 0.75) = 450 s`, or **1.11×**. An upside split of 30% synthesis at 2×, 20% conversion/allocation at 3×, and 50% unchanged gives `358.3 s`, or **1.40×**. These are alternatives, not additive claims.

ESTIMATE — For 4,096 independent inverses, assuming inversion costs 80 multiplications, individual inversion costs `4096×80=327,680 M`. Montgomery batching costs approximately `3×4096+80=12,368 M`, a **26.5× arithmetic-only** reduction; adding a generic numerator multiplication per output gives `16,464 M`, or **19.9×**. Allocation, scanning, and dependent consumers lower realized gains; the actual inversion/multiplication ratio is unknown.

ESTIMATE — At k20, `n=2^20` and each scalar column occupies `32n=32 MiB`. Assuming an eight-byte enum tag, `Rational<Fq>` occupies approximately 72 bytes and `Option<Fq>` 40 bytes; these layouts need measurement. With 40 total/20 active columns, avoiding inactive rational allocations removes `20×72n=1,440 MiB = 1.406 GiB` from that allocation stage. If only 10% of active cells have denominators, replacing dense optional-denominator storage could remove approximately `0.9×20×40n=720 MiB` before index/scratch overhead. Neither saving is a promised reduction of total 12 GiB peak RSS.

ESTIMATE — If 120 s key generation genuinely recurs for the same key, ten hypothetical 500 s proofs cost `10×(120+500)=6,200 s`; one setup plus ten proofs costs `120+10×500=5,120 s`, **1.21×** across that batch. Existing loaded-VK paths cannot claim this saving, and PK construction must be timed independently.

ESTIMATE — GPU gains remain CPU-limited: if 40% of work stays on CPU and 60% accelerates 10×, total speedup is `1/(0.4+0.6/10)=2.17×`. Halving that CPU component gives `1/(0.2+0.06)=3.85×` against the original baseline. This illustrates why CPU work benefits GPU integration; it is not a measured RTX 5090 forecast.

## Risks and soundness concerns

- **PROPOSAL — Require identical VK bytes and circuit layout** for the quick-win track. A changed floor planner or lookup-table shape leaves the deployed-key constraint and needs explicit rekeying/deployment analysis.
- **PROPOSAL — Preserve unknown values, zero denominators, equality auxiliary defaults, inactive-phase behavior, and blinding rows.** Sparse storage must distinguish unwritten storage from valid field zero.
- **PROPOSAL — Keep RNG sampling and transcript writes ordered.** Parallel arithmetic may finish out of order; serialize commitments in original column order. Never reuse a deterministic benchmark seed in production.
- **PROPOSAL — Retain parsing/subgroup checks and local verification.** Cache only material authenticated under its original rules. Do not interpret internal Montgomery limbs as canonical wire scalars or assume cross-library representations coincide.
- **PROPOSAL — Bound cache/scratch memory and clear private job state before reuse.** Avoid new witness-dependent access patterns where the existing threat model requires constant-time behavior; constant-time and throughput variants need explicit review.
- **OPEN QUESTION — Which backend/feature combination produced the historical timeout?** Registry sources, checkout sources, and recl9 pins visibly differ; performance attribution needs the exact executable/source hashes.

## Validation plan

PROPOSAL — Future execution should proceed through these predicates; no builds, benchmarks, proofs, or stagenet actions were performed for this research node.

1. **Artifact predicate:** record binary/source digests, dependency/features, actual ADX selection, CPU flags, params/VK digests, transcript, k, and worker configuration. Pin separate Moriarty 0.8.3 and recl9 0.8.2 comparisons. Existing source documentation explicitly identifies the byte boundary between those crate graphs (`M/docs/sherlock/reference/recursive-child-parent-terminal.md:27`).
2. **Arithmetic predicate:** differential tests cover 0, 1, −1, modulus boundaries, random operands, carries, canonical serialization, Montgomery conversion, inversion, and aliasing. For batched inversion, compare every output to individual inversion, including zero/all-zero/empty inputs and chunk edges.
3. **Assignment predicate:** compare phase/column/row/value maps, region starts/shapes, fixed/selectors, copies/permutation, public instances, and challenges against reference. Require processed VK equality. Exercise unknown witnesses and equal-input/noninvertible cases; verify that rejected witnesses still reject.
4. **Proof predicate:** with fixed test-only RNG streams and preserved ordering, first require byte-identical proofs. If only unconstrained auxiliary values legitimately differ, require identical VK/statements and successful verification by the untouched reference verifier; investigate every byte divergence. Cover leaf, linked parent, and terminal separately.
5. **Acceptance predicate:** terminal proofs must pass the ledger-9 verifier and the actual contract operation using its installed key. Mutating proof bytes, statement, binding input, communications commitment, context, or child/VK identity must reject at the appropriate layer. Require actual stagenet acceptance before declaring the integration complete; native verifier success alone is insufficient.
6. **Performance predicate:** measure cold setup, warm proving, and local verification separately with wall time, cumulative CPU, RSS, allocation bytes/count, active workers, and queue delay. Begin with small fixtures, then the same pinned k20 workload. Accept each optimization only with correctness predicates preserved and a repeatable stage-level improvement.

## Open questions

- **OPEN QUESTION — What fraction of the k20 parent is synthesis, PK construction, rational conversion, BLST preparation/MSM, FFT, and quotient evaluation?** Existing elapsed/RSS observations do not answer this.
- **OPEN QUESTION — How many advice phases/columns and nontrivial denominators occur in the actual Sherlock relation?** These determine whether phase-only/sparse storage deserves first priority.
- **OPEN QUESTION — Does the deployed binary already enable ADX, and does `ci_build` activate portability?** Build configuration is not proven by CPU capability.
- **OPEN QUESTION — Can large relation witnesses adopt shared immutable storage without cloning mutable chip state or changing public APIs?** Which eager inverse results escape their local region?
- **OPEN QUESTION — What byte-budgeted PK/SRS cache and job admission limit fit the real simultaneous child/parent/decider workload?** The five-entry service cache is not a memory budget.
- **OPEN QUESTION — Is browser-side recursion required, or is WASM only transaction construction around a native proving service?** Optimize the path actually used.

## Links

PROPOSAL — Coordinate measurements with [[loki-01-prover-profile]], CPU/GPU commitment preparation with [[loki-02-gpu-msm]], and field buffers with [[loki-03-gpu-ntt]]. Compare integration patterns in [[loki-04-halo2-gpu-precedents]], accumulation semantics in [[loki-05-recursion-accumulation]], and unchanged circuit costs in [[loki-06-recursive-circuit-cost]].

PROPOSAL — Keep proof-system alternatives in [[loki-07-folding-alternatives]], cross-job budgeting in [[loki-08-recursion-tree-scheduling]], and ledger acceptance predicates in [[loki-10-integration-verification]].

SOURCE — Midnight describes its system as Plonk/KZG and notes divergence from its upstream libraries; use its exact interfaces rather than assuming upstream Halo2 behavior. ([Midnight ZK](https://github.com/midnightntwrk/midnight-zk), capture [zk.md](../raw/09/zk.md).)

```
