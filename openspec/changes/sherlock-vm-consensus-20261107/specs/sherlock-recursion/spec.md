# Capability: sherlock-recursion (leaf, node, decider, contract)

Status: CONSENSUS v2.1 (council vote 6/6 ACCEPT at 2026-11-07 21:29Z; text unchanged from DRAFT v2.1). Refines UIREQ-023, UISEC-006/009/010/011, ZR04/05/08/10, SRPOC-008; adopts SND-001..005 and fable-2's SRC text. Design: `../../design.md` §3.

## ADDED Requirements

### Requirement: SRC-001 Leaf statement schema
Leaf, node and decider SHALL consume one versioned statement `sherlock-leaf-stmt/1 = [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`; any other lane count or order SHALL be refused before proving.

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
Before any network call, a verifier process that is not the prover SHALL re-verify the exact bytes to be submitted, reconstruct the accumulator and check every link, and the record SHALL state the shared `midnight-proofs` implementation.

#### Scenario: One flipped byte
- **WHEN** one byte of the node proof in the bundle is flipped.
- **THEN** `verify_chain` refuses and the E6 record shows the refusal.

### Requirement: SRC-011 Throughput target (Mandatory since 2026-11-08 (3))
A committed program of at most 320 owner steps SHALL prove its leaves, fold them through the node chain or the tree, prove the run decider, pass `verify_chain` and the local `validate_call` in under 2 h wall, under 48 GB peak RSS per process and under 100 GB aggregate peak RSS across the concurrently running heavy processes of the run (owner decision 2026-11-08: "48 GB per process, 100 GB aggregate"), on the 20-core 120 GB host. The receipt SHALL record `RAYON_NUM_THREADS`, the concurrency P and both peaks. The C0 phase profile SHALL be committed before any S, concurrency, GPU or tree adoption, and `qa/sherlock/decisions/speed-levers.json` SHALL cite the C0 metric behind each adopted lever. A shortfall is a failing row.

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
When milestone M4 probes the rows of the universal gated leaf over the frozen inventory at S=1 (task 4.1, modeling only) and keys it at the S chosen at gate G-A (task 4.4), option U SHALL be selected if `k_U <= 17` at S=1 and the keyed leaf has `k <= 17 + log2(S)`, and option F otherwise; both branches SHALL carry the same statement and obligations. Under F a leaf proves one rule, so F fixes S=1 (amended 2026-11-08 (3)).

#### Scenario: k above threshold
- **WHEN** the universal leaf probes at `k_U = 19` at S=1.
- **THEN** option F is recorded with S=1, the family key table is committed before P3 is written, the node checks membership in circuit, and the throughput target rests on k=19, concurrency, the GPU track and the tree.

#### Scenario: Non-member key under F
- **WHEN** a valid leaf proof is produced under a key outside the committed family table.
- **THEN** the node refuses it.

#### Scenario: S from k_U
- **WHEN** 4.1 records `k_U = 16` and G-A is open.
- **THEN** G-A records S=8 (`16 + 3 <= 20`) and 4.2 builds the leaf with 8 slots; 4.4 keys it once.

### Requirement: SRC-014 S-slot leaf with a keyed S (amended 2026-11-08 (3))
S SHALL be a keyed constant of the leaf relation, chosen once at gate G-A by `k_U + log2(S) <= 20` and fixed before the leaf is keyed; S=1 is the one-enabled-slot case of the same relation. When a leaf proves fewer than S enabled steps, remaining slots SHALL be disabled with identity lanes and zero `steps` contribution; the node SHALL accept `steps` in [1, S] and refuse `steps == 0` and `steps > S`. The slot semantics SHALL equal the `Leaf` of the Lean composition theorems (SRC-017).

#### Scenario: Terminal publication at slot 3 of 8
- **WHEN** `KAdminPublishValue` fires at slot 3.
- **THEN** slots 4-7 are disabled, `steps == 4`, and the decider's `n_steps` equals the final `h`.

#### Scenario: Disabled slot that writes
- **WHEN** a raw witness sets `enable == 0` on slot 5 and `post_5 != pre_5`, or counts the slot in `steps`.
- **THEN** the keyed leaf refuses it (row A20).

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
If a GPU MSM (BLS12-381 G1) or NTT (Fr) backend is enabled, its results SHALL be bit-identical to the CPU path on every input (zero, one, r-1 scalars, the identity point included), so proof bytes, keys and format are unchanged and stock Midnight verifies the proofs; the backend SHALL be open-source Rust behind a default-off `gpu` feature of a patched `midnight-proofs 0.8.3` (registry crate stays the default, T0 isolation), SHALL fall back to the CPU path without a device, and SHALL count device allocations against the process cap. ICICLE and sppark are not used.

#### Scenario: One differing limb
- **WHEN** the GPU MSM differs from `multi_exp` on one of 2^20 random inputs, or on an identity-point input.
- **THEN** A25 fails; no key, transcript or decider change may hide it; the kernel is fixed or the track is closed.

#### Scenario: No device
- **WHEN** the prover is built with `gpu` and runs on a host without a CUDA device.
- **THEN** it proves on the CPU path with identical bytes and the receipt records `gpu: false`.

#### Scenario: Track opened or closed by C0
- **WHEN** the C0 profile records MSM share m and NTT share q of the node step.
- **THEN** the track proceeds past the toolchain smoke only if `m + q >= 0.4`; below 0.25 it is closed with the shares recorded.
