# Prior study: loki05-prior
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-05-recursion-accumulation.md
> Collected: 2026-10-26
> Published: 2026-10-24

---
title: "Sherlock recursion: defer KZG decisions, preserve the ledger boundary"
project: loki
node: 05
author: gpt-6.1-sol
date: 2026-10-24
tags: [recursion, accumulation, kzg, midnight, sherlock, gpu]
sources:
  - https://github.com/midnightntwrk/midnight-zk
  - https://github.com/midnightntwrk/midnight-zk/pull/480
  - https://github.com/midnightntwrk/midnight-zk/pull/450
  - https://eprint.iacr.org/2020/499
  - https://eprint.iacr.org/2019/1021
  - https://eprint.iacr.org/2025/695
  - https://github.com/axiom-crypto/snark-verifier
  - https://github.com/axiom-crypto/snark-verifier/blob/main/snark-verifier/src/pcs/kzg.rs
  - https://github.com/axiom-crypto/snark-verifier/blob/main/snark-verifier/src/pcs/kzg/accumulation.rs
  - https://github.com/axiom-crypto/snark-verifier/blob/main/snark-verifier/src/pcs/kzg/decider.rs
  - https://github.com/midnightntwrk/midnight-trusted-setup
  - https://github.com/ingonyama-zk/icicle
  - https://github.com/supranational/sppark
---

## Question

**PROPOSAL:** Find the cheapest execution of Sherlock's child → parent → terminal-decider recursion that preserves the installed ledger-9 verifier, including its proof system, transcript, SRS and VK. Treat a different circuit/VK as a separate deployment design, even if the ledger's verification software could accept that proof after registering the new key.

## Summary

- **CODE:** Pairing deferral already exists: `VerifierGadget::prepare` returns an accumulator; it does not establish full child-proof validity. The terminal must discharge that accumulator. `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:705`.
- **CODE:** Fenrir's Sherlock parent prepares two differently keyed children, combines their MSM obligations and exposes 16 statement fields plus 12 accumulator fields. Its decider verifies the parent, folds the carried obligation, subgroup-checks the final points and checks one pairing product. `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:3`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:148`.
- **ESTIMATE:** The retained same-key arity sweep gives approximately 192,000 additional advice rows per child, or 239,524 rows per child after sharing the four-child node overhead: 958,095 / 4. These are workload-specific row costs, not universal constraint counts. Evidence: `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:122`.
- **CODE:** BLS12-381/Jubjub provides an embedded application curve, not a recursion cycle: BLS G1 coordinates remain foreign-field values in the BLS scalar circuit. `~/repos/midnight-zk/curves/src/jubjub/curve.rs:49`; `~/repos/midnight-zk/curves/src/bls12_381/fp.rs:553`; `~/repos/midnight-zk/curves/src/jubjub/fr.rs:101`.
- **PROPOSAL:** Keep the existing recursion shape and keys; cache setup and accelerate its prover MSMs/NTTs. For a future rekeyed family, prefer verifier-free fused leaves and bounded fan-in, initially four where measured tables already force k20. Do not promise an additional “defer pairings” speedup to a pipeline that already does so.

## Findings

### Released code and PR #480 are different interfaces

**SOURCE:** Midnight describes its system as PLONK with KZG commitments. PR #480, whose head branch was `incircuit-pcs`, merged as `31435e9` on July 21, 2026. Its incorporated PR #450 adds multi-polynomial commitments, positional labels and label-based query grouping. This is a concrete PCS-interface refactor, not evidence of a GPU prover. [Repository](https://github.com/midnightntwrk/midnight-zk), capture [midnight-zk.md](../raw/05/midnight-zk.md); [PR #480](https://github.com/midnightntwrk/midnight-zk/pull/480), capture [midnight-pr480.md](../raw/05/midnight-pr480.md); [PR #450](https://github.com/midnightntwrk/midnight-zk/pull/450), capture [midnight-pr450.md](../raw/05/midnight-pr450.md).

**CODE:** At that checkout, `InCircuitPCS<S>` supplies commitment assignment/transcript operations and `multi_prepare -> AssignedAccumulator<S>`. `VerifierGadget::prepare<PCS>` parses the proof and checks the algebraic relations; the PCS opening check still ends in an accumulator. `~/repos/midnight-zk/circuits/src/verifier/pcs.rs:89`; `~/repos/midnight-zk/circuits/src/verifier/verifier_gadget.rs:845`; `~/repos/midnight-zk/circuits/src/verifier/verifier_gadget.rs:865`.

**CODE:** Moriarty's lockfile selects proofs 0.8.3, circuits 7.2.4, curves 0.3.1 and stdlib 2.3.5. The released circuit API instead accepts `AssignedVk<S>` and raw assigned committed-instance points. Recl9 pins proofs 0.8.2 and patches circuits to its hardened copy. Therefore the checkout is an architectural reference, not a drop-in replacement for either dependency graph. `~/repos/Moriarty-factory/Cargo.lock:1803`; `~/repos/Moriarty-factory/Cargo.lock:2030`; `~/repos/Moriarty-factory/Cargo.lock:2271`; `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:713`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:31`.

### What accumulation removes—and what remains

**CODE:** Midnight's convention is `e(A, [tau]₂) = e(B, [1]₂)`. Each side initially contains a lazy MSM, including named fixed-base terms. Accumulation hashes the encodings of all input accumulators into a Poseidon challenge r, then combines corresponding sides with powers of r. The code explicitly qualifies the converse preservation implication as computational. `~/repos/midnight-zk/circuits/src/verifier/accumulator.rs:14`; `~/repos/midnight-zk/circuits/src/verifier/accumulator.rs:179`.

**SOURCE:** Accumulation schemes support recursive composition by separating accumulation from the final decision. Axiom's `snark-verifier` implements that split as `KzgAs` and `KzgDecidingKey`: accumulation combines two point channels, while the native decider performs a pairing product. Its lhs/rhs convention differs from Midnight's, so its equations must not be copied by field name. [PCD paper](https://eprint.iacr.org/2020/499), capture [accumulation-pcd.md](../raw/05/accumulation-pcd.md); [KZG API](https://github.com/axiom-crypto/snark-verifier/blob/main/snark-verifier/src/pcs/kzg.rs), capture [snark-kzg.md](../raw/05/snark-kzg.md); [accumulation implementation](https://github.com/axiom-crypto/snark-verifier/blob/main/snark-verifier/src/pcs/kzg/accumulation.rs), capture [snark-accumulation.md](../raw/05/snark-accumulation.md); [decider](https://github.com/axiom-crypto/snark-verifier/blob/main/snark-verifier/src/pcs/kzg/decider.rs), capture [snark-decider.md](../raw/05/snark-decider.md).

**CODE:** The inner circuit still performs transcript hashing, challenge generation, instance evaluation, permutation/lookup/algebraic checks and KZG multi-opening reduction. Collapsing the result executes foreign-curve MSMs. The accumulator code calls collapse a likely recursive-circuit bottleneck; omitting pairings does not omit these MSMs. `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:690`; `~/repos/midnight-zk/circuits/src/verifier/verifier_gadget.rs:513`; `~/repos/midnight-zk/circuits/src/verifier/accumulator.rs:313`.

**CODE:** Midnight's `aggregation/` supplies both generic IVC and a multi-circuit aggregator. An IVC step verifies the previous step and carries an accumulator; the multi-circuit transition also verifies a new application proof, maintains a claims hash and collapses its running inner accumulator. Thus an aggregation step contains both the chain verifier and application-proof verifier, rather than eliminating one. Its exported verifier pins the self-VK, invokes the state decider and checks the accumulated pairing off circuit. `~/repos/midnight-zk/aggregation/src/ivc/circuit.rs:190`; `~/repos/midnight-zk/aggregation/src/multi_circuit_aggregator/circuit.rs:328`; `~/repos/midnight-zk/aggregation/src/ivc/verifier.rs:49`.

**PROPOSAL:** Use this IVC implementation as a reference for recursive-state binding. Do not substitute its off-circuit decider for Sherlock's terminal relation: the ledger's ordinary outer-proof check must also attest that the carried inner obligation was decided.

### Curves and the foreign-field cost

**CODE:** Midnight calls the BLS12-381 scalar field `Fq` and its 381-bit base field `Fp`. The circuit uses the 255-bit scalar field; `BlstrsEmulation` pairs that field with G1 and a `ForeignWeierstrassEccChip`. Jubjub's base equals that BLS scalar field, but Jubjub has a separate 252-bit scalar field. From these definitions, BLS12-381 and Jubjub do not satisfy the two reciprocal field equalities required for a curve cycle. `~/repos/midnight-zk/curves/src/bls12_381/fq.rs:375`; `~/repos/midnight-zk/curves/src/bls12_381/fp.rs:553`; `~/repos/midnight-zk/curves/src/jubjub/curve.rs:49`; `~/repos/midnight-zk/curves/src/jubjub/fr.rs:101`; `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/types.rs:140`.

**SOURCE:** Halo demonstrates recursion using ordinary curve cycles, while the Midnight authors' foreign-field paper reports approximately one order of magnitude elliptic-curve emulation overhead against native arithmetic in its studied settings. Neither statement establishes a whole-Sherlock timing ratio or makes Jubjub a replacement KZG curve. [Halo](https://eprint.iacr.org/2019/1021), capture [halo.md](../raw/05/halo.md); [foreign-field paper](https://eprint.iacr.org/2025/695), capture [foreign-field.md](../raw/05/foreign-field.md).

**CODE:** The checkout also exposes k256, p256 and curve25519, with bn256 behind a development/testing feature. Their presence supplies no implemented native recursion cycle for the BLS KZG verifier. `~/repos/midnight-zk/curves/src/lib.rs:26`.

### Fenrir's working shape and the ledger boundary

**CODE:** Recl9's synthetic leaf directly proves up to four transactions with Poseidon/Jubjub and contains no recursive verifier. The node verifies two to four leaf proofs under one fixed VK, composes their segments, resolves shared fixed bases once and collapses once. This implementation is a leaf aggregator, not a general node-of-nodes tree. `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/leaf.rs:1`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/leaf.rs:38`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:223`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/node.rs:128`.

**CODE:** Sherlock's specialized parent instead links A102 and A80 under separate VKs, shares program/post-to-pre cells, merges the fixed-base maps and emits 28 public fields. The collapsed accumulator is 12 fields: two five-field encoded points and two unit scalars. These are SDK public-input encodings, not serialized affine coordinates. `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:3`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:395`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:32`; `~/repos/Moriarty-factory/docs/sherlock/reference/recursive-child-parent-terminal.md:68`.

**CODE:** The decider verifies that parent with Poseidon, incorporates its carried accumulator, checks final point subgroups and unit scalars, then enforces `e(A,[tau]₂)e(B,-[1]₂)=1`. Its residue method uses a constrained Fp12/Fp6 witness and Miller loop instead of a conventional final exponentiation. The terminal proof itself uses Blake2b in the retained recl9 run. `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:148`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/pairing.rs:1`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/out/decider-b1/batch-1.stats.json:37`.

**CODE:** Ledger verification calls stdlib verification with the registered VK, ordered public vector and ordinary proof bytes. The pinned ledger-9 source uses Blake2b and embedded `bls_midnight_2p14` verifier parameters; this does not imply every proved circuit must have k≤14. Recl9 records a k20 terminal accepted by the ledger-9 Rust verifier, explicitly without chain submission in that report. `~/repos/projectfenrir/recursion/stagenet-strawman/chain/rc5check/ledger-rc5/transient-crypto/src/proofs.rs:65`; `~/repos/projectfenrir/recursion/stagenet-strawman/chain/rc5check/ledger-rc5/transient-crypto/src/proofs.rs:121`; `~/repos/projectfenrir/recursion/stagenet-strawman/chain/rc5check/ledger-rc5/transient-crypto/src/proofs.rs:563`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:3`.

**CODE:** StrawAnchor's `acceptValidity` inserts the statement; settlement recomputes it against current state. SherlockAnchor additionally checks key/program/context pins, predecessor root, height, ordered trace and replay state. Prover correctness and successful settlement are separate predicates. The node checkout explicitly selects separate ledger-9 dependencies, so its generic ledger alias is insufficient to identify this acceptance path. `~/repos/projectfenrir/recursion/stagenet-strawman/chain/anchor/straw_anchor.compact:54`; `~/repos/projectfenrir/recursion/stagenet-strawman/chain/anchor/sherlock/sherlock_anchor.compact:85`; `~/repos/midnight-node/Cargo.toml:96`.

### Cost per inner proof

**CODE:** The retained recl9 sizing results below describe the same-key synthetic workload, not Sherlock's heterogeneous children. Evidence: `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:102`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:117`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/keys/decider/keygen.stats.json:2`.

| Evidence | Circuit | Advice rows | Table rows | k |
|---|---|---:|---:|---:|
| CODE | Four-transaction leaf | 112,098 | 98,563 | 17 |
| CODE | Two-child node, interim shift fix | 574,040 | 1,032,465 | 20 |
| CODE | Three-child node, interim shift fix | 765,907 | 1,032,465 | 20 |
| CODE | Four-child node, interim shift fix | 957,724 | 1,032,465 | 20 |
| CODE | Four-child node, hardened | 958,095 | 1,032,465 | 20 |
| CODE | Terminal, including one parent verifier and pairing decision | 841,682 | 1,032,209 | 20 |

**ESTIMATE:** Marginal child cost is `(957,724 − 574,040)/2 = 191,842` advice rows. A local interpolation including the four-child hardening increment is `R_node(b) ≈ 190,727 + 191,842b`. It gives 239,524 amortized rows/child at b=4. Extrapolating b=5 gives 1,149,937 rows, exceeding k20's 1,048,569 usable rows; actual b=5 synthesis is unmeasured. This model must not be applied to a recursive node child with a different architecture/VK.

**ESTIMATE:** Including the terminal, the four-child pipeline spends `(958,095 + 841,682)/4 = 449,944.25` recursion advice rows per original leaf proof, amortized. This sums work across separate circuits; it is not one circuit's row count, gate count or domain size. The measured terminal alone amortizes to `841,682/4 = 210,420.5` rows per original leaf proof. Isolating its pairing-only cost requires instrumentation.

**CODE:** Advice reduction alone need not reduce k: the table already occupies 1,032,465 rows at all measured arities. The full decider cost cannot be called “the pairing cost,” because it includes verification, collapse, subgroup checks and statement binding. `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:129`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:148`.

## Proposed design

**PROPOSAL:** Choose the first of these three designs under the current fixed-VK requirement:

| Status | Shape | Compatibility and tradeoff |
|---|---|---|
| PROPOSAL: immediate | Existing children → existing linked parent → existing decider | Preserve all circuits, proof formats and keys; accelerate prover execution and reuse setup. |
| PROPOSAL: future family | Fused verifier-free leaves → four-child bounded aggregators → terminal | Amortize fixed verifier/table cost; needs new parent and terminal VKs, plus anchor key registration/deployment. |
| PROPOSAL: research comparison | One terminal directly verifies the children and decides their accumulator | Removes the parent-proof layer, but combines all child verification and pairing constraints; unknown k/memory and a different terminal VK. |

**PROPOSAL:** Do not select the latter alternatives for an already registered key. Keeping PLONK/KZG and proof serialization while changing the circuit still changes the VK. Switching to IPA, a different curve, folding or another opening protocol changes inner proof material and requires a checked adapter plus a newly keyed terminal; sending that proof directly to the present ledger verifier is incompatible.

**PROPOSAL:** Introduce three off-chain modules in a future implementation:

1. **PROPOSAL:** `RecursionPlan` pins child/parent/terminal VK digests, relation bytes, public layouts, transcript types, SRS identity and feature closure. `prepare_parent(children)` preserves existing witness order and returns the existing 28-field statement and proof.
2. **PROPOSAL:** `ProverCache::get_or_create(plan_id)` retains validated VK/PK material, coefficient/Lagrange SRS bases and immutable domain metadata. Cache keys include exact circuit and implementation fingerprints; cache hits never authorize a different relation.
3. **PROPOSAL:** `PcsAccelerator` supplies `msm_g1(srs_id, basis, scalars)` and `ntt_fq(domain, direction, values)`, with CPU fallback and explicit device-buffer lifetimes. Preserve RNG consumption, transcript writes, root ordering, Montgomery conversions and normalization.

**PROPOSAL:** The data flow is pinned child bytes and instances → existing parent witness construction → accelerated parent polynomial prover with Poseidon → existing terminal witness containing parent proof and accumulator → accelerated terminal polynomial prover with Blake2b → unchanged ledger-9 verification → anchor settlement. The terminal's pairing predicate remains constrained inside its circuit throughout.

**CODE:** Concrete released hooks are `KZGCommitmentScheme::{commit,commit_lagrange}`, which call `poly::kzg::msm::msm_specific`, and `EvaluationDomain::{lagrange_to_coeff,coeff_to_lagrange,coeff_to_extended,extended_to_coeff}`. Stdlib `setup_pk`/`prove` provide the cache/prover boundary. The checkout's newer `PolynomialCommitmentScheme::commit_many` is a distinct future adapter point. `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74`; `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:163`; `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:224`; `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1967`; `~/repos/midnight-zk/proofs/src/poly/kzg/mod.rs:82`.

**PROPOSAL:** Keep circuit-side `VerifierGadget::prepare`, `AssignedAccumulator::accumulate`, fixed-base resolution and collapse unchanged for the installed VK. Accelerating the host evaluation of an MSM does not remove its in-circuit constraints. For a new multi-level aggregator, explicitly fold every child's carried accumulator as well as its own proof obligation; the current leaf-only helper is insufficient.

**SOURCE:** ICICLE lists BLS12-381 and a Rust GPU backend interface; sppark supplies BLS12-381 MSM and NTT primitives. These support backend candidates, not a measured Sherlock/RTX-5090 speedup. [ICICLE](https://github.com/ingonyama-zk/icicle), capture [icicle.md](../raw/05/icicle.md); [sppark](https://github.com/supranational/sppark), capture [sppark.md](../raw/05/sppark.md).

**CODE:** The local NVIDIA Rust runtime offers streams, buffers and events but describes itself as early-stage research. The local `halo2` ancestor normalizes bases and calls `msm_best` in its commitment implementation, whereas the used 0.8.3 path above calls `msm_specific`. Backend integration and profiling must target the actual selected implementation. `~/repos/nvidia-cuda-rust/README.md:23`; `~/repos/nvidia-cuda-rust/README.md:33`; `~/repos/halo2/src/poly/kzg/mod.rs:68`.

**SOURCE:** Midnight's ceremony repository identifies its BLS12-381 powers-of-tau SRS. **PROPOSAL:** Retain the exact deployed setup and verify its verifier-parameter relationship; never use a fresh benchmark setup for production proofs. [Trusted setup](https://github.com/midnightntwrk/midnight-trusted-setup), capture [midnight-srs.md](../raw/05/midnight-srs.md).

## Expected speedup

**CODE:** A later Sherlock receipt records k20, 6,432 proof bytes, PK setup 62.328 s and a prove interval of 882.590 s including explicit PK destruction. Cache state is unqualified; its mixed service memory counter is not isolated proof RSS. These observations supersede neither the question's censored >500-second attempt nor a controlled benchmark. `~/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/README.md:5`; `~/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json:62`.

**ESTIMATE:** For accelerated fraction f, kernel speedup s and transfer/synchronization overhead h measured as a fraction of original runtime, `S = 1 / ((1−f)+f/s+h)`. Assuming f=0.80, s=10, h=0.03 gives `1/0.31 = 3.23×`. Assuming f=0.90, s=20, h=0.03 gives `1/0.175 = 5.71×`. Neither f nor s has been measured here; witness generation and foreign-field assignment may dominate instead.

**ESTIMATE:** On an illustrative completed 500-second baseline, those assumptions give 155 s and 87.5 s. They are scenario arithmetic, not predictions from the unfinished >500-second run. Reusing the later receipt's PK alone would remove at most `62.328/(62.328+882.590)=6.60%` of that two-phase interval, before cache costs and changed destruction timing.

**ESTIMATE:** For four fixed leaves, one arity-four node uses one k20 parent job; a binary tree would require three parent jobs and two levels, assuming compatible recursive node circuits existed and each still required k20. That suggests approximately 3× less parent-job work, not 3× whole-pipeline speedup. Existing recl9 already uses the four-child shape; the incremental shape speedup there is 1×.

## Risks and soundness concerns

**CODE:** The hardened Fenrir MSM shift uses subgroup-checked assignment and enforces nonidentity. The terminal independently binds subgroup-checked A/B encodings and unit scalars before pairing. Preserve both protections; do not interpret unchecked proof-point assignment as permission to weaken the shift or terminal checks. `~/repos/projectfenrir/recursion/stagenet-strawman/verifier-patch/midnight-circuits/src/ecc/foreign/weierstrass_chip.rs:1758`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:163`.

**CODE:** Accumulation challenges depend on the exact encoded accumulator shape. The upstream aggregator explicitly collapses before resolving fixed bases to match its host transition; recl9's same-key helper instead accumulates own obligations before resolving/collapsing. Reordering across these implementations can change Fiat–Shamir inputs. `~/repos/midnight-zk/aggregation/src/multi_circuit_aggregator/circuit.rs:343`; `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:199`.

**PROPOSAL:** Audit canonical encodings, subgroup assumptions, challenge domains, duplicate/reordered children, VK namespaces and setup consistency as one protocol. Host checks help diagnose failures but must not substitute for terminal constraints. Preserve the full-width challenge configuration; an optimization changing challenge sampling is a protocol change.

**CODE:** Sherlock's backend source explicitly leaves broader machine obligations open, and the parent receipt disclaims full-goal completion/target acceptance. A faster proof establishes only the keyed relation, not omitted machine semantics. `~/repos/Moriarty-factory/crates/mori-backend/src/sherlock_machine/mod.rs:14`; `~/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json:118`.

## Validation plan

**PROPOSAL:** Execute these predicates in a later authorized implementation campaign; this research ran no builds or benchmarks:

1. **PROPOSAL — P0, identity:** exact SRS/VK/relation/layout/feature digests match the reference. Confirm proofs 0.8.3 versus recl9 0.8.2 interoperability with actual bytes; do not infer it from version numbers.
2. **PROPOSAL — P1, primitives:** differential CPU/GPU MSM and forward/inverse/coset NTT tests over zero, identity, sparse, cancellation, modulus-boundary and non-power-of-two MSM cases. Compare canonical field values and affine points, independent of projective representation.
3. **PROPOSAL — P2, deterministic proof:** under a test-only identical RNG stream, compare commitments, transcript challenges, evaluations, accumulator encodings and final proof bytes. Byte identity is a strong debugging target; separately establish ordinary verifier acceptance when legitimate randomization differs.
4. **PROPOSAL — P3, recursion:** verify child bytes against their exact instances; compare host/in-circuit accumulators; verify parent normally and with its carried obligation; run the terminal pairing predicate. Include well-formed false statements whose preparation succeeds but deferred decision fails.
5. **PROPOSAL — P4, adversarial:** mutate child/VK/SRS/public lanes, reorder or duplicate children, alter accumulator coordinates/scalars, inject low-order shift witnesses, corrupt residue hints, truncate/append proof bytes and cross transcript types. Require rejection at the appropriate complete verification boundary.
6. **PROPOSAL — P5, ledger:** feed GPU terminal bytes to the unchanged ledger-9 verifier under the already registered VK and captured call template. Require honest acceptance and public-slot tamper rejection; then separately check stagenet transaction inclusion, replay rejection and exact anchor state effects.
7. **PROPOSAL — P6, performance:** measure isolated setup, witness assignment, MSM, NTT, quotient, transfers, prove, PK drop and final memory counters; repeat controlled CPU/GPU runs. Admit speedup claims only after P0–P5 pass.

**CODE:** Recl9's retained negative tests include wrong child statements/proof swaps that can pass the prepare-only node and fail the decider. Such node acceptance is expected deferral behavior, not a sufficient success condition. `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:178`.

## Open questions

- **OPEN QUESTION:** What are Sherlock's exact per-child prepare/MSM/Poseidon row counts and lookup-table decomposition? The same-key 192k marginal estimate cannot answer this for its two VKs.
- **OPEN QUESTION:** Which installed terminal VK, captured call template and source fingerprints define the current stagenet acceptance target? Historical ledger verification and current deployment identity must be reconciled.
- **OPEN QUESTION:** How much of the slow interval is witness/foreign-field assignment versus host MSM/NTT and PK destruction? This determines whether the GPU scenarios are attainable.
- **OPEN QUESTION:** Can a future rekeyed range-table configuration reduce k without increasing columns or verifier cost enough to lose the gain? Does a direct two-child terminal beat the present parent-plus-decider at that configuration?
- **OPEN QUESTION:** Is there a reviewed compositional soundness argument for the exact unchecked-point, hardened-shift, accumulator and residue-decider combination across the selected dependency versions?

## Links

**PROPOSAL:** Coordinate primitive acceleration with [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]], profiling with [[loki-01-prover-profile]], precedents with [[loki-04-halo2-gpu-precedents]], and precise circuit accounting with [[loki-06-recursive-circuit-cost]]. Compare format-changing alternatives in [[loki-07-folding-alternatives]], scheduling in [[loki-08-recursion-tree-scheduling]], witness work in [[loki-09-cpu-witness-field]], and final acceptance in [[loki-10-integration-verification]].
