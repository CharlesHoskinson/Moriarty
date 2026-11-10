# Capability: sherlock-recursion (leaf, node, decider, contract)

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1). Refines UIREQ-023, UISEC-006/009/010/011, ZR04/05/08/10, SRPOC-008; adopts SND-001..005 and fable-2's SRC text. Amended 2026-11-10 by the design review (`research/sherlock-recursion-design-review-20261110/review.md` §5; SRC-001, SRC-010, SRC-011, SRC-013, SRC-014, SRC-017 edited; SRC-019..021 added) and by the owner definition `../../general-purpose.md`; not council text. Design: `../../design.md` §3.

## ADDED Requirements

### Requirement: SRC-001 Leaf statement schema
Leaf, node and decider SHALL consume one versioned statement `sherlock-leaf-stmt/1 = [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`; any other lane count or order SHALL be refused before proving. (Amended 2026-11-10.) `rules_d` SHALL be `Poseidon(dom("SHK_RULES"), rid_0, ..., rid_{S-1})` over exactly S slot rule lanes in slot order with `rid_j = sum_r sel_j[r] * lane(r)`, so a disabled slot carries the reserved id 0, which SHALL NOT be the lane of any rule in `R`; no special case for S=1 or one enabled slot. The enabled slots SHALL form a prefix; a publishing step SHALL be the last enabled slot; the outcome tag and result inside `pre_d` and `post_d` SHALL be derived from the constrained carriers; `prog_d` SHALL be the identity of SVM-005.

#### Scenario: Disabled slot encoded as a rule
- **WHEN** a raw witness encodes a disabled slot of `rules_d` with the lane of an admitted rule, or leaves a gap in the enable prefix, or enables a slot after a publishing step.
- **THEN** the keyed leaf refuses it; the Quint slot model (F24) catches the matching mutant.

#### Scenario: Outcome supplied as a constant
- **WHEN** a raw witness proves a step with an outcome tag or result inside `post_d` that differs from the post carriers.
- **THEN** the keyed leaf refuses it (today the C6 bridge takes the tag and value as constants, `sherlock_c6_child.rs:560-561,571-572`).

#### Scenario: Six-lane export
- **WHEN** the current C6 digest export (six lanes, `sherlock_c6_child.rs:817-834`) is fed to the node harness.
- **THEN** it is refused with a shape error; after the bridge, one leaf per rule of `entry 7;` verifies under the node with all seven lanes constrained.

### Requirement: SRC-002 Program identity
`prog_d` SHALL be the digest defined in SVM-005.

#### Scenario: As SVM-005
- **WHEN** SVM-005's scenario runs. **THEN** it holds for the leaf, node and decider lanes.

### Requirement: SRC-003 In-leaf constraint of rules and steps
The leaf SHALL constrain `rules_d` to the digest of its selector outputs, `steps` to its count of enabled owner-step events (free publication included, disabled padding excluded) and SHALL chain `post_i == pre_{i+1}` internally.

#### Scenario: Label lanes become facts
- **WHEN** a fresh malicious witness proves one internal transition but claims a different `rules_d` or `steps = 2`.
- **THEN** the keyed leaf refuses (today `rule`/`h` are only `PublicInput` + `Impact`, `sherlock_c6_child.rs:598-608`).

### Requirement: SRC-004 Bounded height arithmetic
If `prev_h`, `steps` or `h = prev_h + steps` is not below 2^64, or `steps == 0`, or `steps > S`, the node relation SHALL be unsatisfiable.

#### Scenario: Field wrap
- **WHEN** raw witnesses set `steps = p - 1` or `prev_h = 2^64 - 1` from a valid baseline.
- **THEN** the witness reaches the node's constraint/verifier boundary and is refused there; a synthesis error, decoder refusal or panic is recorded as "boundary untested" (SAS-003), not as a pass.

### Requirement: SRC-005 Authenticated genesis
If `prev_h == 0`, the node SHALL require `prev_d == init_d` and SHALL introduce no unauthenticated deferred obligation, by realization (a) constraining the supplied carry to the designated neutral encoding, or (b) constraining a zero/ignore construction (carry scaled by `not_genesis`) with a recorded justification that an ignored carry cannot authorize an invalid leaf or predecessor. Retaining an unscaled supplied carry requires a separately recorded security argument.

#### Scenario: Realization (a), neutral encoding
- **WHEN** a genesis node witness supplies a non-neutral carry, from a valid baseline.
- **THEN** the node constraint boundary refuses it; the neutral carry passes.

#### Scenario: Realization (b), ignored carry
- **WHEN** two genesis node proofs differ only in a well-formed ignored carry.
- **THEN** both pass the run decider; identical folded bytes are not required.

#### Scenario: Invalid leaf at genesis
- **WHEN** a genesis node proof carries an invalid leaf accumulator and any chosen carry.
- **THEN** the run decider's final pairing refuses it under either realization; a malformed carry encoding is refused at its decoder boundary and counts only as decoder evidence. This review did not establish a practical attack; the clause is hardening.

### Requirement: SRC-006 Decider pins and discharges
The run decider SHALL verify the last node proof under constant node keys, pin `self_vk_repr`, fold and pair the accumulator in circuit, open `fin_d` as a halted state with a range-checked u128 result derived from the proved state, and expose exactly `[prog_d, init_d, fin_d, n_steps, result_be]`.

#### Scenario: Outer-only success
- **WHEN** the node chain verifies but the decider is skipped.
- **THEN** the result is not reported final-valid (UISEC-010).

### Requirement: SRC-007 Ledger binding and replay
`acceptRun` SHALL read sealed `program_d` and `init_d` into the transcript, refuse a `stmt` already in `runs`, and write the result as canonical big-endian `Bytes<16>`; readback SHALL decode the ledger's trimmed representation to the exact u128 over the whole domain.

#### Scenario: Replay on the target
- **WHEN** the same `stmt` is submitted twice on Preview.
- **THEN** the second call is refused and the indexer shows one accepted call bound to the manifest.

#### Scenario: Result domain
- **WHEN** runs halt with results 0, 1, 256, 2^64 and u128::MAX on a local 9.1.0.1 ledger.
- **THEN** deploy/call/readback decodes each exactly; no nonzero-low-byte restriction applies (`sherlock-run.compact:40-47` has none).

### Requirement: SRC-008 Verifier manifest
When a key is generated or installed, the toolchain SHALL write the manifest fields listed in `requirements.md`; deploy or call SHALL refuse when fresh keygen differs.

#### Scenario: Drifted zkir binary
- **WHEN** the manifest's zkir binary version differs from the installed one.
- **THEN** `deploy-run.mjs` refuses before any transaction.

### Requirement: SRC-009 Re-key closure
When a ledger, circuits, zk-stdlib or zkir pin changes, the toolchain SHALL regenerate and hash-compare every relation and regenerate the dependent closure in the order leaf -> node -> decider -> contract template.

#### Scenario: Node VK changed, leaf unchanged
- **WHEN** the regression at 9.1.0.1 reports leaf `unchanged`, node `changed`.
- **THEN** node, decider and template are regenerated; the leaf key is reused; `validate_call` accepts the new bundle and refuses the five mutations.

### Requirement: SRC-010 Independent verification of the exact bundle
(Amended 2026-11-10.) Before any network call, a verifier process that is not the prover SHALL re-verify every leaf, node and decider proof of the versioned GP bundle (SRC-019), reconstruct the accumulator and check every link, and the record SHALL state the shared `midnight-proofs` implementation. The legacy `examples/verify_chain.rs` (historical A102/A80 pair) and a check of the final node proof alone SHALL NOT satisfy it.

#### Scenario: One flipped byte
- **WHEN** one byte of any leaf, node or decider proof in the bundle is flipped.
- **THEN** the independent verifier refuses and the E6 record shows the refusal (row A31).

### Requirement: SRC-011 Throughput target (Mandatory since 2026-11-08 (3))
A committed program of at most 320 owner steps SHALL prove its leaves, fold them through the node chain or the tree, prove the run decider, pass the independent verifier (SRC-010) and the local `validate_call` (SRC-020) in under 2 h wall, under 48 GB peak RSS per process and under 100 GB aggregate peak RSS across the concurrently running heavy processes of the run (owner decision 2026-11-08: "48 GB per process, 100 GB aggregate"), on the 20-core 120 GB host. The receipt SHALL record `RAYON_NUM_THREADS`, the concurrency P and both peaks. The C0 phase profile SHALL be committed before any S, concurrency, GPU or tree adoption, and `qa/sherlock/decisions/speed-levers.json` SHALL cite the C0 metric behind each adopted lever. A shortfall is a failing row. (Amended 2026-11-10.) A projection SHALL use total wall = setup + max(node service, leaf service) under measured mixed contention + decider + independent verification + call validation; a lever not built and measured SHALL get no credit in G-C.

#### Scenario: Shortfall is a failure
- **WHEN** fact(3) at the chosen S takes more than 2 h, or any process exceeds 48 GB, or the aggregate exceeds 100 GB.
- **THEN** row A19 fails with the shortfall and its C0 phase cause recorded; gate G-C is not passed and OP-PV-DEPLOY does not run.

#### Scenario: Lever without a C0 metric
- **WHEN** S, a thread count, a GPU backend or the tree is adopted and `speed-levers.json` names no C0 metric for it.
- **THEN** the adoption is refused by the gate record review (G-A); the lever is not keyed.

### Requirement: SRC-012 Resource caps are hard limits
If a run exceeds its `heavy` memory or wall cap, it SHALL stop and preserve its result; no counter reset, no k increase, no substituted profile.

#### Scenario: Wall cap
- **WHEN** a leaf batch exceeds its budget.
- **THEN** `runs.jsonl` records exit 4 with a trace and the next attempt carries `--hypothesis`.

### Requirement: SRC-013 Leaf shape decision
(Amended 2026-11-10; was "U if `k <= 17 + log2 S`, otherwise F".) When milestone M4 probes and keys the universal gated leaf over the frozen inventory (tasks 4.1a, 4.2b, 4.1, 4.4), the toolchain SHALL use option U and SHALL record rows, k, RSS and prove time of the leaf at the chosen S, and the `cost_model` rows of the node and decider keyed over a leaf of its arch. Option F (finite family, in-circuit key membership) is defined and not planned: it fixes S=1, which fails SRC-011 at the measured node step; it MAY be used as a diagnostic.

#### Scenario: Diagnostic probe first
- **WHEN** task 4.1a runs `IrSource::model()` over every existing per-rule C6 circuit and the dispatch block alone.
- **THEN** rows, table rows, arch and minimum k are written to `speed-levers.json` `G-A-prime.probe`, labelled a projection with reduced coverage (SAS-014), and no S is chosen from it alone.

#### Scenario: Decider headroom
- **WHEN** the node keyed over a leaf of the universal leaf's arch pushes the decider above 2^20 rows.
- **THEN** the record shows the decider k next to the leaf k, and S is chosen from the full envelope (leaf, node and decider), not from `k_U + log2 S` alone (experiment E2, task 4.12).

### Requirement: SRC-014 S-slot leaf with a keyed S (amended 2026-11-08 (3); 2026-11-10)
S SHALL be a keyed constant of the leaf relation, chosen once at gate G-A' and fixed before the leaf is keyed; S=1 is the one-enabled-slot case of the same relation. S SHALL be the largest power of two, at most `S_MAX` = 16, for which the measured leaf, node and decider fit their k and memory caps; `k_U + log2 S <= K_LEAF_MAX` is a first sizing estimate (`K_LEAF_MAX` = [PENDING experiment E4, task 4.11]); S >= 8 is the planning target; S=4 SHALL be chosen only with a measured leaf time; S <= 2 is a redesign trigger; if no S >= 4 fits, the leaf design SHALL be revisited before any key. `S_MAX` SHALL be a serialized field of the node relation. When a leaf proves fewer than S enabled steps, remaining slots SHALL be disabled with identity lanes and zero `steps` contribution; the node SHALL accept `steps` in [1, S] and refuse `steps == 0` and `steps > S`. The slot contract is specified by the Quint slot model (F24); Lean proves boundary composition only until a Lean refinement from S-slot execution to the boundary `Leaf` exists.

#### Scenario: Terminal publication at slot 3 of 8
- **WHEN** `KAdminPublishValue` fires at slot 3.
- **THEN** slots 4-7 are disabled, `steps == 4`, and the decider's `n_steps` equals the final `h`.

#### Scenario: Disabled slot that writes
- **WHEN** a raw witness sets `enable == 0` on slot 5 and `post_5 != pre_5`, or counts the slot in `steps`.
- **THEN** the keyed leaf refuses it (row A20).

#### Scenario: S from the measured envelope
- **WHEN** the measured leaf at S=8 fits 16 GB and `K_LEAF_MAX`, and the node and decider keyed over its arch fit their caps.
- **THEN** G-A' records S=8 with each input labelled measured or forecast, and 4.4 keys the leaf once with `S_MAX` serialized equal to 8.

#### Scenario: S_MAX changed
- **WHEN** two node relations differ only in `S_MAX`.
- **THEN** their `write_relation` bytes differ and the key cascade runs.

### Requirement: SRC-015 Staged throughput gates
After the node fixes and the k=19 cut, one node step SHALL prove in at most 60 s and 16 GB at the qualification thread count, or the shortfall and C0 cause SHALL be recorded and S raised at G-A (A16); the `entry 7;` chain SHALL complete end to end in under 20 min (A18, gate G-B); fact(3) and list123 SHALL each complete in under 2 h (A19, A19b, gate G-C); every row at 48 GB per process and 100 GB aggregate; caps are seeded requirement caps, never measured-then-doubled.

#### Scenario: Seeded cap
- **WHEN** `chain-entry7` has `cap.wallSeconds = 1200`, `fixed: true` and `measured.wallSeconds = null`, and the run takes 1,500 s.
- **THEN** `heavy` stops the run at the cap with exit 4; the cap is not rewritten to twice the measurement; A18 fails.

#### Scenario: G-B shortfall
- **WHEN** A18 fails and the C0 profile of the real `entry 7;` chain (task 5.3) names the phase.
- **THEN** S or the GPU is re-decided at G-B with that cause while re-keying is still local; no Preview deploy has happened.

### Requirement: SRC-016 Memory safety (E11 required)
Every `heavy` process SHALL run under a cgroup v2 `MemoryMax` equal to its cap (default 48 GiB, never above 48 GiB) with `MemorySwapMax=0`; a declared concurrent run SHALL sit in one parent scope with `MemoryMax=100G`; preflight SHALL refuse when `MemAvailable` < cap + 20 GiB or swap in use > 4 GiB; the sum of admitted caps SHALL not exceed 100 GiB; `cargo test` under `heavy` SHALL set `RUST_TEST_THREADS <= 4` for the prover crates, and heavy tests SHALL be `#[ignore]` in plain `cargo test`.

#### Scenario: Runaway test binary
- **WHEN** a `mori_backend` test binary under `heavy` grows past 48 GiB.
- **THEN** the kernel kills it (exit 137), the run records `capExceeded: "rss"`, the host never swaps, and the next attempt needs `--hypothesis`.

#### Scenario: Fourth concurrent node
- **WHEN** three 26 GB node provers are admitted under `--concurrent 3 --aggregate-cap 100G` and a fourth asks for 26 GB.
- **THEN** `acquire_turn` refuses the fourth until one finishes (78 + 26 > 100).

### Requirement: SRC-017 General-purpose recursion: fixed keys and formal gates
(Amended 2026-11-10, owner: "It needs to be general purpose recursion. Make sure we understand what that means. And universal".) The target SHALL be GP-1 to GP-4 as defined in `../../general-purpose.md` (GP-001..GP-004); the design of this change is the first milestone toward GP-1, and its open gaps are listed in `../../design.md` §3.6. The text below states what the first milestone SHALL meet.

One key set (leaf over `R` at the keyed S, one self-recursive node, one decider, one acceptRun template) SHALL serve every committed program, step count, fold schedule and tree shape; per-program, per-depth or per-level keys are refused. The aggregation protocol SHALL have a Quint model (`models/sherlock/quint/aggregation.qnt`, constants `TREE` and S) whose invariants pass with every seeded mutant caught, and Lean theorems (`clean/fork/Clean/Sherlock/Aggregation.lean`, library `SherlockSpec`) that build with zero `sorry` and declared premises only: segment soundness for every program, associativity, padding neutrality, exactly-once coverage, height arithmetic, decider pinning, independence from shape, depth and S. F20, F21 and the council review SHALL pass before 2.3, 4.2 and 5.7 are implemented.

#### Scenario: Program-dependent constant
- **WHEN** a node or decider relation's `write_relation` bytes change between two committed programs.
- **THEN** the key is refused as not general purpose; the defect is fixed before keygen.

#### Scenario: Mutant caught
- **WHEN** the Quint model lets a padded slot count one step, or lets two fold schedules reach different roots.
- **THEN** `quint test` reports the invariant violation for that mutant; F20 passes only when every mutant is caught.

#### Scenario: Theorem with an undeclared axiom
- **WHEN** `#print axioms` of `Aggregation.segment_sound` lists an axiom outside `AggregationAxioms.lean`.
- **THEN** `scripts/qa/check-lean-aggregation.sh` fails F21.

### Requirement: SRC-018 GPU backend bit-identity and fallback
If the GPU path is enabled (the `cuda` feature of `crates/mori-gpu` built and its hooks registered by `heavy --gpu`), its MSM (BLS12-381 G1) result SHALL equal the CPU path's (`msm_specific` over BLST `multi_exp`) in the group, checked on `to_affine()` and the compressed encoding that reaches the transcript, and its NTT (scalar field) result SHALL be bit-exact per element against `best_fft`, on every input (zero, one and r-1 scalars, the identity point as base and as result, duplicate bases, projective bases with Z != 1, the signed-digit exceptions, host- and device-resident inputs), so proof bytes, keys and format are unchanged and stock Midnight verifies the proofs. The backend SHALL be an open-source Rust crate owned in this repository with a small CUDA C++ kernel core compiled by nvcc from `build.rs` for sm_121 using 32-bit limbs, derived only from permissively licensed sources with headers preserved (sppark Apache-2.0, Yrrid ZPrize-2023 MIT/Apache, ICICLE v2.8.0 MIT, ec-gpu MIT/Apache; nothing from ICICLE v3+/open-icicle, gnark-cuda, Nocy, Snarkify ZPrize-1B or the WebGPU entries); neither sppark nor ICICLE SHALL be a crate dependency. The `midnight-proofs 0.8.3` hooks patch SHALL live in the separate workspace `tools/gpu-prover/` (registry crate stays the default in the main workspace, T0 isolation extended to the vendored copy). The backend SHALL fall back to the CPU path at compile time, init time (no device, driver or toolchain) and call time (small n or kernel error) with identical bytes and the fallback recorded, SHALL run under the same cgroup caps (device allocations are process RSS on this host), and SHALL accept any `n <= SRS size` and `log_n <= 32` without knowledge of the program or circuit. Design source: `research/sherlock-speed-replan-20261108/gpu.md`.

#### Scenario: One differing point or element
- **WHEN** the GPU MSM differs in the group from `multi_exp` on one of 2^20 random inputs or on an identity-point, duplicate-base or signed-digit edge input, or the GPU NTT differs in one element from `best_fft`.
- **THEN** A25 or A26 fails; no key, transcript or decider change may hide it; the kernel is fixed or the phase is closed.

#### Scenario: Same point, different projective coordinates
- **WHEN** the GPU MSM returns (X, Y, Z) different from BLST's for the same group element.
- **THEN** `to_affine()` and the compressed bytes are equal and the differential test passes; the proof bytes are unchanged because only compressed points reach the transcript.

#### Scenario: No device
- **WHEN** `tools/gpu-prover` is built with `cuda` and runs on a host without a CUDA device, driver or toolchain, or an MSM has fewer than `2^MORI_GPU_MIN_K` points.
- **THEN** it proves on the CPU path with identical bytes and the receipt records the fallback (`gpu: false` when the path was never enabled; otherwise `gpu.fallbacks` counts the calls).

#### Scenario: Device memory under the cap
- **WHEN** a `heavy --gpu` process allocates device memory with `cudaMalloc` or `cudaMallocManaged` under a cgroup `MemoryMax`.
- **THEN** the allocation appears in the process RSS and counts toward the cap; the receipt records `gpu.device_bytes_peak`.

#### Scenario: Track opened or closed by C0 and the owner
- **WHEN** the C0 profile records MSM share m and NTT share q of the node step and gate G-A is written.
- **THEN** the track proceeds past the toolchain smoke (G0) only if `m + q >= 0.4` and the owner funds the phases; below 0.25 it is closed with the shares recorded; each later phase G1-G6 passes or is killed by its own gate in `gpu.md` §6.

### Requirement: SRC-019 Versioned GP bundle
(Added 2026-11-10, task 5.1a.) A proving run SHALL write a versioned bundle that keeps every leaf, node and decider proof, every key manifest, the fold order and the run statement, bound by one manifest hash; no proof SHALL be discarded before SRC-010 and SRC-020 have run.

#### Scenario: Missing node proof
- **WHEN** the bundle of `entry 7;` lacks one intermediate node proof or one artifact hash differs from its file.
- **THEN** the bundle reader refuses it (row A30); today the GP harness keeps only the last node proof (`tests/sherlock_gp.rs:289-291`).

### Requirement: SRC-020 Full local call validation
(Added 2026-11-10, task 5.1c.) Before any network call, the decider proof SHALL pass full local `validate_call` of the acceptRun call with the template recaptured under Compact 0.36.0 on a local proof-verifying ledger 9.1.0.1 with mainnet parameters, and the five mutations SHALL be refused.

#### Scenario: Stale template
- **WHEN** the template predates the re-pin or its compiler version is unobserved.
- **THEN** the validation does not count (row A32 fails) until the template is recaptured.

### Requirement: SRC-021 Canonical initialization
(Added 2026-11-10.) If a run's initial carriers are not the canonical initial state for the program image, configuration, fuel and stack limit that `prog_d` commits, the run SHALL NOT be accepted as a canonical run; equality with a supplied `init_d` alone SHALL NOT count. Mechanism: [PENDING owner question 1 of `../../dissent.md` "Owner questions from the 2026-11-10 design review"], option G1 (an in-circuit or otherwise authenticated first-leaf check selected by `h_pre == 0`; cost [PENDING experiment E3, task 4.13]) or option G2 (off-chain recomputation of `init_d` by the independent verifier, with O1 restated).

#### Scenario: Forged initial carriers
- **WHEN** a run starts from carriers that open to the sealed `init_d` but are not the canonical initial state of the program.
- **THEN** the chosen mechanism refuses it at its named boundary (row A13), and the record names the circuit or verifier that performed the check.
