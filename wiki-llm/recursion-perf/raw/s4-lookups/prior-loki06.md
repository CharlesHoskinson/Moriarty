# Earlier recursive circuit cost study
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-06-recursive-circuit-cost.md
> Collected: 2026-10-26
> Published: Unknown

---
title: "Recursive circuit cost: the route from k20 toward k18"
project: loki
node: 06
author: gpt-6.1-sol
date: 2026-10-24
tags: [recursion, circuit-cost, poseidon, foreign-field, lookups, ledger-compatibility]
sources:
  - https://github.com/midnightntwrk/midnight-zk
  - https://eprint.iacr.org/2019/458
  - https://eprint.iacr.org/2023/323
  - https://www.rfc-editor.org/rfc/rfc7693
  - https://zcash.github.io/halo2/design/proving-system/lookup.html
  - https://zcash.github.io/halo2/design/implementation/proofs.html
  - https://github.com/privacy-scaling-explorations/halo2wrong
  - https://eprint.iacr.org/2020/081
  - https://github.com/ingonyama-zk/halo2
  - https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2
---

## Question

**PROPOSAL —** Reduce recursive verification toward k18 by attacking lookup-table height and foreign-curve MSM cost together, while preserving the terminal statement and ledger verification semantics. Treat **the existing pinned VK** as a hard boundary: a smaller-domain circuit cannot use that same VK. Circuit redesign is therefore a conditional future key migration; GPU arithmetic acceleration remains the immediate option under immutable keys.

## Summary

- **CODE —** Children already use Poseidon; the terminal proof uses Blake2b. Switching the parent away from Blake2b offers no saving on this existing path. See [Moriarty child:47](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:47), [Fenrir inner:82](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:82), and [terminal record:154](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:154).
- **CODE —** Fenrir's recorded two-child node has 574,040 advice rows and 1,032,465 table rows. Removing the table bottleneck alone leaves advice above k19 capacity. These are historical homogeneous-node measurements, not a measurement of the current heterogeneous Sherlock parent. [REPORT:117](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:117)
- **CODE —** The BLS base field is emulated with seven 56-bit limbs; the scalar field and Poseidon are native. Fixed VK bases are currently moved into the variable-base MSM before collapse. [Foreign parameters:277](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/foreign/params.rs:277), [MSM:481](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/msm.rs:481)
- **PROPOSAL —** First inventory the real Sherlock parent; then evaluate smaller lookup limbs, preservation of fixed-base information, and scalar-bound-aware MSM scheduling using existing gates. Preserve the existing aggregate-before-collapse design.
- **ESTIMATE —** k20→k18 gives 1,048,576/262,144 = 4× fewer domain rows, but a plausible k18 budget requires roughly 75–80% savings in an assumed dominant EC component. No such reduction has been demonstrated here; widening columns can consume much of the gain.

## Findings

### Provenance and acceptance boundary

**CODE —** The requested upstream checkout is at `31435e989f224ccd0bbfabbaaffebeea5e17f15b`; its circuits manifest declares 7.1.0 and proofs 0.8.0, whereas Moriarty's lockfile contains proofs 0.8.3/circuits 7.2.4/stdlib 2.3.5. Fenrir recl9 instead pins proofs 0.8.2 and patches circuits to its hardened copy. These graphs must remain separate in comparisons. [Upstream manifest:1](/home/charles/repos/midnight-zk/circuits/Cargo.toml:1), [Moriarty lock:2030](/home/charles/repos/Moriarty-factory/Cargo.lock:2030), [Fenrir manifest:25](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:25)

**SOURCE —** Midnight's repository describes PLONK with KZG, BLS12-381/Jubjub, circuits, and a standard-library layer. This establishes architectural context, not compatibility between releases. [Repository](https://github.com/midnightntwrk/midnight-zk); capture [raw/06/midnight-zk.md](../raw/06/midnight-zk.md).

**CODE —** `MidnightVK` serializes architecture, k, public-input count, and the underlying key; reading reconstructs the constraint system through `ZkStdLib::configure(architecture, k-1)`. Arbitrary new custom gates are not described by these bytes. Changing k, fixed tables, or selectors changes the VK; introducing gates outside the stock architecture also requires a compatible reader/constraint-system integration. [Stdlib:1547](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1547), [reader:1581](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1581)

**CODE —** The local ledger fixes `TranscriptHash = blake2b_simd::State` and calls stdlib verification. The node delegates transaction validation to its ledger bridge. Fenrir's historical ledger-9 report records verification of its terminal proof and rejection of public-input mutations; that does not establish acceptance of a redesigned Sherlock circuit today. [Ledger proofs:64](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:64), [verify:544](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:544), [node:576](/home/charles/repos/midnight-node/pallets/midnight/src/lib.rs:576), [ledger-9 record:160](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:160)

### Transcript gap: deliberately different layers

**CODE —** Moriarty's `ChildTranscript` is `PoseidonState<outer::Scalar>`; `ChildKey::prove` and `verify` use it explicitly. Fenrir's off-circuit `plonk::prepare` and its `BlstrsEmulation` verifier use the corresponding Poseidon transcript. The child producer itself flags 0.8.3↔0.8.2 compatibility and truncated-challenge feature parity as unestablished. [Child:48](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:48), [prove:384](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:384), [obligations:54](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:54), [emulation:140](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/types.rs:140)

**CODE —** A Poseidon point absorption uses its field-element public-input encoding, although proof serialization still uses compressed curve bytes. Blake2b instead absorbs compressed bytes and samples scalars from its digest. Its transcript also has a keyed domain separator and common/challenge prefixes. Replacing only the hash call cannot preserve challenges or proof validity. [Poseidon encoding:207](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/hash/poseidon/poseidon_cpu.rs:207), [Blake transcript:13](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/transcript/implementors.rs:13), [encodings:138](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/transcript/implementors.rs:138)

**SOURCE —** Poseidon targets arithmetic over prime fields; this motivates its placement inside recursion. Poseidon2 changes the linear layers and is a distinct hash, rather than an implementation substitution for existing Poseidon transcripts. [Poseidon](https://eprint.iacr.org/2019/458), capture [poseidon.md](../raw/06/poseidon.md); [Poseidon2](https://eprint.iacr.org/2023/323), capture [poseidon2.md](../raw/06/poseidon2.md).

**CODE —** Existing Poseidon already packs six partial rounds per row: `NB_SKIPS_CIRCUIT=5`, eight full rounds, sixty partial rounds, rate two. Its saved cost-model fixture reports **20 rows per permutation**, eight advice columns, degree five. This is not 68 generic arithmetic rows. [Chip:45](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/hash/poseidon/poseidon_chip.rs:45), [constants:20](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/hash/poseidon/constants/mod.rs:20), [fixture:38](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/goldenfiles/cost-model.json:38)

**SOURCE —** BLAKE2b compression has twelve rounds, eight G mixes per round; a G mix contains six word additions and four word XORs. [RFC 7693 §3](https://www.rfc-editor.org/rfc/rfc7693), capture [blake2-rfc.md](../raw/06/blake2-rfc.md).

**ESTIMATE —** A deliberately naive bit circuit therefore needs 12×8×(6+4)×64 = **61,440 bit-operation slots per compression**, excluding input constraints/finalization. At four such slots per row that is 15,360 rows. This is an illustrative implementation assumption, not the Midnight Blake gadget's measured cost or a lower bound; lookup word gadgets can differ substantially. For the actual parent, avoided Blake-transcript rows from another hash switch = **zero**, because it already verifies Poseidon children.

**PROPOSAL —** Keep Poseidon internally and Blake2b for the terminal proof. Poseidon2 or an all-Poseidon terminal proof changes Fiat–Shamir semantics and requires corresponding verifier support; neither satisfies the existing transcript constraint. Outer Blake2b runs in the host prover/verifier and does not itself add Blake2b gates to the decider circuit.

### Rows: two independent barriers

**CODE —** Saved Fenrir sizing gives the following historical baselines. These are repository-reported observations; no synthesis/build/benchmark was run for this note. [REPORT:102](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:102)

| CODE — historical relation | Advice rows | Table rows | k |
|---|---:|---:|---:|
| Two homogeneous children | 574,040 | 1,032,465 | 20 |
| Four homogeneous children, hardened | 958,095 | 1,032,465 | 20 |
| Terminal decider | 841,682 | 1,032,209 | 20 |

**CODE —** Table height matters because `Pow2RangeChip` loads `(tag,value)` entries for each used tag n and all values below 2^n. It already skips unused tags. Stdlib couples maximum lookup limb length to k−1; decomposition chooses row-efficient limbs under that limit. Merely removing unused tags repeats an existing optimization. [Range table:43](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/decomposition/pow2range.rs:43), [load:214](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/decomposition/pow2range.rs:214), [planner:121](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/decomposition/chip.rs:121), [k coupling:1873](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1873)

**ESTIMATE —** All tags 0…15 require Σ2^n = **65,535** entries; 0…16 require 131,071; 0…17 require 262,143, leaving no blinding margin at k18 if all occur. A cap of fifteen is a useful candidate, not an assertion that the observed million rows all come from this one table. Splitting a 255-bit value needs ceil(255/19)=14 versus ceil(255/15)=17 chunks, a **21.4%** increase before packing/reconstruction effects.

**CODE —** Domain sizing takes the maximum of advice extent, table extent, public inputs, and minimum circuit rows, with unusable rows included. Tables and advice are not simply added. [Cost model:330](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/dev/cost_model.rs:330)

### Foreign EC and the MSM that remains

**CODE —** Midnight names the 255-bit BLS scalar field `Fq` and the 381-bit base field `Fp`. Scalar verifier arithmetic is native; G1 coordinates require foreign arithmetic. Seven 56-bit limbs, two auxiliary moduli, and 15-bit range limbs support the latter. The ECC column formula gives 7+max(7,2+2)+1 = **15 advice columns**. [Scalar bits:366](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fq.rs:366), [base bits:553](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fp.rs:553), [parameters:277](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/foreign/params.rs:277), [column formula:97](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:97)

**CODE —** Stock fixtures report complete addition 342 rows, doubling 89, three full-width scalar MSM 39,935. The bounded fixture is **not three short scalars**: its widths are 4, 12, and 255 bits. Fenrir's hardening raises those MSM fixtures to 55,630 and 46,318 rows respectively. These include setup/assignment and cannot be multiplied per scalar to predict a parent. [Stock fixture:2029](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/goldenfiles/cost-model.json:2029), [bounded inputs:617](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/instructions/ecc.rs:617), [hardening counts:83](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/verifier-patch/PATCH-REPORT.md:83)

**CODE —** The MSM already uses width-four windows, GLV splitting when appropriate, 1-bit scalar preprocessing, and shared randomness. Shorter scalar sequences are padded to the longest; each variable-base table constructs fifteen additions. Fixed-base resolution discards the named/fixed distinction before this algorithm. These are specific remaining optimization sites, whereas “add GLV” duplicates existing work. [MSM preprocessing:810](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:810), [padding:1788](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:1788), [table loop:1840](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:1840)

**CODE —** Sherlock's heterogeneous A102/A80 parent already assigns merged fixed bases, prepares both children, accumulates once, and collapses once. The terminal decider additionally folds the carried accumulator, checks subgroup membership, and executes a pairing-product circuit. Parent savings cannot be attributed to elimination of a pairing it never performed. [Sherlock:472](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:472), [decider:148](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:148), [pairing:186](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:186)

### Lookups, gates, columns, and opening points

**SOURCE —** Halo2's lookup argument introduces permuted input/table columns and a product column. The proof-encoding documentation shows three commitments and five evaluations per lookup. Its final IPA encoding is not Midnight's KZG encoding. [Lookup design](https://zcash.github.io/halo2/design/proving-system/lookup.html), capture [halo2-lookups.md](../raw/06/halo2-lookups.md); [proof encoding](https://zcash.github.io/halo2/design/implementation/proofs.html), capture [halo2-proof-encoding.md](../raw/06/halo2-proof-encoding.md).

**CODE —** Midnight's deployed gadget independently confirms three read points and five read scalars per lookup, including x, next, and previous rotations. Adding lookup lanes can shorten advice while making the next recursive verifier larger. Sherlock already selects four range-check lanes; stock configuration permits fewer than five. Advice columns are shared across chips by taking their maximum, rather than summing chip widths. [Lookup gadget:56](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/lookup.rs:56), [architecture:52](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:52), [lane bound:181](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/decomposition/pow2range.rs:181), [column sharing:492](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:492)

**CODE —** Foreign arithmetic already has slope, tangent, and lambda-squared custom gates; Poseidon already batches partial rounds. Optimizing these requires retaining integer no-wrap bounds and exceptional-point checks. [Slope equations:51](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/gates/weierstrass/slope.rs:51), [doubling checks:1195](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:1195)

**SOURCE —** Halo2wrong provides established non-native arithmetic and ECC gadget examples, useful as design comparisons rather than drop-in Midnight replacements. The multipolynomial commitment paper studies batching evaluations at different point sets, motivating attention to query structure. [Halo2wrong](https://github.com/privacy-scaling-explorations/halo2wrong), capture [halo2wrong.md](../raw/06/halo2wrong.md); [Boneh–Drake–Fisch–Gabizon](https://eprint.iacr.org/2020/081), capture [shplonk.md](../raw/06/shplonk.md).

**CODE —** Despite its “GWC” file comment, deployed `multi_prepare` groups commitments by point sets, reads an auxiliary commitment and one evaluation per set, then a final opening point. The prover writes `f_com` and `pi`; “one proof point per rotation” is not its wire model. Fewer point sets can save interpolation/evaluations without removing existing queries. [Gadget:338](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/kzg.rs:338), [prover:163](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:163), [final opening:191](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:191)

**ESTIMATE —** Removing one genuinely redundant lookup would remove three points and five evaluations. With five field lanes per G1 point, that is 3×5+5 = 20 fewer absorbed fields, approximately ten rate-two permutations or **200 Poseidon rows**, before phase padding. The more significant saving can be three EC terms in the final MSM. Eliminating one point-set evaluation alone saves roughly half a permutation, about ten amortized hash rows, plus interpolation work; it does not necessarily remove an EC proof point. Neither operation is valid unless the underlying circuit no longer requires those checks. [Point encoding size:42](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:42)

**PROPOSAL —** Preserve lookup protocol and opening algorithm. Reduce redundant native expressions, equality-enabled columns, and avoidable advice rotations when rebuilding circuits. Never omit required lookup/permutation openings. Replacing this argument with another SHPLONK variant or LogUp changes the protocol, transcript, and verifier; it is outside immutable-verifier acceleration.

### Moriarty contributes child cost, not a measured recursion decomposition

**CODE —** `sherlock_machine` emits SDK IR; it is not the recursive verifier. `fetch` constrains all cells, creates a selector per capacity row, and conditionally selects each column across capacity. The arithmetic step already reuses a comparison for subtraction/less-than and implements UInt128 multiplication through 64-bit limbs. [Module:1](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_machine/mod.rs:1), [fetch:82](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_machine/fetch.rs:82), [arithmetic:627](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_machine/c6_scalar_arithmetic_step.rs:627)

**ESTIMATE —** For capacity C and width W, fetch emits C×W conditional selections plus O(C) selectors/sums and O(CW) type checks. C=64,W=8 gives **512 selections**, before their SDK row expansion. This is an IR count, not 512 proven Halo2 rows. Reducing repeated checks can shrink children, but lower child k alone does not remove the parent's commitment/lookup inventory.

## Proposed design

**PROPOSAL —** Use three compatibility classes: **A**, identical VK/transcript/SRS with accelerated arithmetic; **B**, new VK/layout using stock verifier gates and unchanged outer Blake2b/KZG; **C**, changed gates, transcript, PCS, or curve requiring additional verifier integration. Under the owner's strict existing-VK condition, only A is presently admissible. B and C are research results, with explicit migration costs.

1. **PROPOSAL — `recursive_cost` inventory.** Add a future read-only `inventory(vk, instance_len) -> CostInventory` covering advice/fixed/equality columns, gate degree, lookup count, point sets, used range tags, scalar bounds, and named/variable MSM terms. Add region counters around `VerifierGadget::prepare`, transcript squeezing, accumulation, fixed-base resolution, and collapse. Separate table extent from advice extent. Use `stdlib::cost_model(rel, Some(k))` only in a bounded later experiment; automatic optimal-k search repeatedly synthesizes. No profiling build is part of this research delivery.
2. **PROPOSAL — `lookup_policy`.** Evaluate `LookupLimbPolicy { max_limb_bits: 15 }` in decomposition planning while retaining stock `configure(arch,k-1)` identities and foreign-gate bound coefficients. All larger bounds must decompose exactly, including remainders. A global change to configure's max-bit parameter would change those coefficients and fail stock reconstruction. Compare four versus fewer range lanes; four already exist. Table/selector changes require new VKs even if gate expressions remain stock. Hook `P2RDecompositionChip::new`, `compute_optimal_limb_sizes`, and `Pow2RangeChip::load_table`.
3. **PROPOSAL — `partitioned_msm`.** Preserve named fixed terms through `AssignedAccumulator` collapse; expose `collapse_partitioned(variable_terms, named_fixed_terms, scalar_bounds)`. Precompute known VK-base tables, constrain every table entry as a constant, then use existing dynamic multi-select gates where possible. Use checked nonzero subgroup shifts and constrained corrections: the existing selector forbids identity entries, so a digit-zero identity table is invalid. Current `load_multi_select_table` accepts assigned points, making this a concrete reuse candidate. Keep variable-base MSM hardened; group differing proven bit bounds only when saved padded additions exceed extra ladders and group-combination cost. Reuse existing GLV. Hook `AssignedMsm::resolve_fixed_bases/collapse`, `SelfEmulation::msm`, and `ForeignWeierstrassEccChip::windowed_msm`. This changes layouts/VKs; cache-only acceleration of identical assignments is class A but saves no rows. [Multi-select:1405](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:1405)
4. **PROPOSAL — `stock_gate_optimizer`.** First reduce normalization/copy duplication using existing native and foreign instructions. Evaluate wider fused slope/normalization gates only as class C experiments, recording degree, rotations, commitments, and downstream recursion cost. Serialize custom parent constraint-system metadata separately if needed; stock `MidnightVK` cannot reconstruct arbitrary gates. Even an internal custom parent changes the decider that pins it, requiring a new terminal VK and contract acceptance path.

**PROPOSAL —** Data flow remains child bytes plus complete instances → existing Poseidon `prepare` → challenge-bound accumulation → optimized EC evaluation → same mathematical accumulator → subgroup checks and pairing in the decider → Blake2b terminal proof. Removing the carried accumulator or pairing would change the claimed relation, not optimize it.

**CODE —** For class A GPU integration, relevant proof-system hooks are `plonk::create_proof`, `PolynomialCommitmentScheme::{commit,commit_lagrange,multi_open}`, and the KZG MSM/NTT implementation beneath them. The newer upstream checkout also has `InCircuitPCS`, but deployed circuits 7.2.4 use the concrete KZG gadget; porting that trait is a separate migration. [Prover:348](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:348), [PCS:16](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/commitment.rs:16), [upstream abstraction:14](/home/charles/repos/midnight-zk/circuits/src/verifier/pcs.rs:14)

**SOURCE —** ICICLE's Halo2 fork accelerates an EZKL-derived prover; its published benchmarks use lookup/simple/PLONK circuits on RTX 4080 hardware. Its v2 account includes gate, permutation, lookup, MSM, and NTT acceleration. Those results establish precedent, not a Sherlock/5090 speed forecast. [Repository](https://github.com/ingonyama-zk/halo2), capture [icicle-halo2.md](../raw/06/icicle-halo2.md); [technical report](https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2), capture [icicle-v2.md](../raw/06/icicle-v2.md).

**CODE —** NVIDIA CUDA Rust offers host/device tooling and describes itself as an early research project; it supplies no Midnight recursion gadget in its overview. Circuit-row reduction and GPU execution are separate work streams. [Local README:8](/home/charles/repos/nvidia-cuda-rust/README.md:8)

## Expected speedup

**ESTIMATE —** Using the historical two-child proxy, k18 with seven unusable rows requires R≤262,137: reduction ≥1−262,137/574,040 = **54.3%**. The four-child hardened proxy needs **72.6%**. The actual Sherlock relation has different child keys/instances, so these are sizing hurdles rather than predictions.

**ESTIMATE —** A sensitivity model makes the missing evidence explicit. Assume 80% of 574,040 rows is EC, the remainder grows 20% under smaller range limbs, and specialization adds 5,000 rows. With 60% EC savings: R=574,040×(0.8×0.4+0.2×1.2)+5,000 = **326,462**, still k19. With 80% EC savings: R=574,040×(0.8×0.2+0.2×1.2)+5,000 = **234,616**, compatible with a k18 advice budget if every table also fits. Applying the latter assumptions to 958,095 rows gives **388,238**, still k19. The EC fraction and savings are hypotheses requiring measurement.

**ESTIMATE —** Under the immutable-VK constraint, circuit-domain reduction is **1×**: none. The following ratios describe conditional redesigned circuits, after an authorized key migration.

**ESTIMATE —** Fixed-width polynomial work scales approximately with domain size: MSM length ratio 4; FFT work ratio (2^20×20)/(2^18×18)=**4.44**. If only 80% of proving time follows that FFT ratio, total speedup is 1/(0.2+0.8/4.44)=**2.63×**. At 60%, it is **1.87×**. These apply only after k actually drops; advice reduction within k20 does not reduce the polynomial domain.

**ESTIMATE —** Doubling advice width from 15 to 30 while quartering rows leaves (30×2^18)/(15×2^20)=**0.5×** advice elements, rather than 0.25×. The corresponding advice FFT ratio is only **2.22×**. Fixed columns, lookup products, quotient degree, and the unchanged terminal stage further limit whole-pipeline gains. No numerical GPU speedup or terminal k18 prediction is supported by this study.

## Risks and soundness concerns

- **CODE —** Fenrir's MSM patch checks the shift subgroup and guards incomplete additions; reverting it for the 15,000-row fixture saving would remove constraints. Its report explicitly requires new keys for affected circuits. [Patch:25](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/verifier-patch/PATCH-REPORT.md:25)
- **PROPOSAL —** Preserve point identity handling, integer reconstruction/no-wrap bounds, canonical encodings, table tags, scalar-bound proofs, and challenge ordering. Never shorten a scalar because its witness happens to be small.
- **CODE —** The in-circuit transcript tolerates unread suffix bytes; dummy-point/scalar parsing is used for synthesis. Off-circuit ingestion must retain strict decoding and transcript-end checks. [Transcript:78](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/transcript_gadget.rs:78), [Moriarty verify:368](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:368)
- **PROPOSAL —** Treat Poseidon2, another curve/cycle, truncated challenges, LogUp, and a different PCS as protocol research. They change transcript/relations or proof format, require fresh soundness analysis and verifier/key integration, and cannot produce proofs under the existing pinned acceptance configuration.

## Validation plan

1. **PROPOSAL — Provenance predicate.** Record exact source/lockfile/feature hashes, SRS and verifier-parameter pins, processed VK bytes, public-input order, and transcript type for every stage. Establish 0.8.3 producer versus 0.8.2 consumer compatibility on real child artifacts before assigning their timings to one pipeline.
2. **PROPOSAL — Arithmetic predicate.** Differentially compare foreign-field operations, GLV decompositions, table selections, and MSM outputs with CPU curve arithmetic. Include zero/max scalars, identity, opposite/equal points, subgroup/torsion cases, malformed limbs, table-tag substitution, and understated scalar bounds. Require constraint rejection, not merely host refusal, where the witness is adversarial.
3. **PROPOSAL — Accumulator predicate.** Compare reference and optimized prepared accumulators, challenge sequences, named scalar maps, and collapsed affine points for identical children. Compare final group values when implementations intentionally partition MSM terms. Mutate each child instance/proof/order; require terminal rejection. Prepare-only parents can output an invalid accumulator without rejecting locally, so parent MockProver acceptance alone is insufficient. [Recorded example:191](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/REPORT.md:191)
4. **PROPOSAL — Capacity predicate.** After bounded synthesis, require advice and every table extent plus unusable rows below 2^18, then confirm actual floor planning and proof generation. Record new commitment/query inventories and any decider growth. Reject a claimed k18 result based only on advice counts or on a small standalone gadget.
5. **PROPOSAL — Acceptance predicate.** For class A, use the exact existing VK/SRS/transcript: fixed-randomness differential runs should seek byte identity, while independent-randomness proofs need only cross-verifier acceptance. Verify GPU proofs with the unmodified CPU verifier and ledger-9 path, then the existing stagenet contract. For class B, explicitly record that existing-VK acceptance is impossible; prove stock-reader compatibility and require a separately authorized deployment/key migration. No implementation or deployment occurs in this research task.

## Open questions

- **OPEN QUESTION —** What are advice/table extents and MSM-term inventories for the exact current Sherlock A102/A80 parent, rather than the homogeneous-node proxy?
- **OPEN QUESTION —** Which used range tags and table regions account for the million-row extent, and can fifteen-bit planning retain every stock-configured foreign-gate bound?
- **OPEN QUESTION —** How much fixed-base work survives GLV/preprocessing, and do constrained precomputations save more rows than their assignments and lookup loading cost?
- **OPEN QUESTION —** Does reducing parent rows increase terminal verifier work through extra columns, lookups, point sets, or degree?
- **OPEN QUESTION —** Is a future terminal VK migration permitted at all? Without it, k reduction remains inadmissible and GPU/CPU acceleration of the existing circuit is the actionable route.

## Links

**PROPOSAL —** Coordinate profiling with [[loki-01-prover-profile]], GPU kernels with [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]], and precedent analysis with [[loki-04-halo2-gpu-precedents]].

**PROPOSAL —** Keep accumulation soundness in [[loki-05-recursion-accumulation]], protocol replacements in [[loki-07-folding-alternatives]], tree scheduling in [[loki-08-recursion-tree-scheduling]], witness/field improvements in [[loki-09-cpu-witness-field]], and deployment acceptance in [[loki-10-integration-verification]].
