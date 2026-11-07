# Capability: sherlock-recursion (leaf, node, decider, contract)

Status: DRAFT v1, specified-only. Refines UIREQ-023, UISEC-006/009/010/011, ZR04/05/08/10, SRPOC-008; adopts SND-001..005 and fable-2's SRC text. Design: `../../design.md` §3.

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
- **WHEN** raw witnesses set `steps = p - 1` or `prev_h = 2^64 - 1`.
- **THEN** synthesis or verification fails at the node key; outcomes are classified per SAS-003.

### Requirement: SRC-005 Authenticated genesis
If `prev_h == 0`, the node SHALL require `prev_d == init_d` and SHALL introduce no unauthenticated deferred obligation, by realization (a) scaling the carried accumulator by `not_genesis` (irrelevance) or (b) keeping the fold with the recorded binding argument of the pinned accumulator.

#### Scenario: Realization (a), irrelevance
- **WHEN** two genesis node proofs differ only in the supplied carry.
- **THEN** both pass the run decider; the carry has no effect on acceptance.

#### Scenario: Invalid leaf at genesis
- **WHEN** a genesis node proof carries an invalid leaf accumulator and any chosen carry.
- **THEN** the run decider's final pairing refuses it under either realization; a malformed carry encoding is refused by point decoding. This review did not establish a practical attack; the clause is hardening.

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

### Requirement: SRC-011 Throughput target (Optional)
Where the owner enables the target, a program of at most 320 owner steps SHALL prove, fold, decide and verify in under 2 h and 48 GB, or the shortfall SHALL be recorded with its C0 phase cause; a shortfall is not a pass.

#### Scenario: Shortfall recorded
- **WHEN** fact(3) at S=1 takes 7.7 h.
- **THEN** the measurement task closes with the shortfall and phase shares; the throughput target stays open.

### Requirement: SRC-012 Resource caps are hard limits
If a run exceeds its `heavy` memory or wall cap, it SHALL stop and preserve its result; no counter reset, no k increase, no substituted profile.

#### Scenario: Wall cap
- **WHEN** a leaf batch exceeds its budget.
- **THEN** `runs.jsonl` records exit 4 with a trace and the next attempt carries `--hypothesis`.

### Requirement: SRC-013 Leaf shape decision
When M2 measures the universal gated leaf at S=1, option U SHALL be selected if k <= 17 and option F otherwise; both branches SHALL carry the same statement and obligations.

#### Scenario: k above threshold
- **WHEN** the union leaf keys at k=19.
- **THEN** option F is recorded, the family key table is committed before P3 is written, and the node checks membership in circuit.

### Requirement: SRC-014 Short final chunk
While a leaf proves fewer than S enabled steps, remaining slots SHALL be disabled with identity lanes and zero `steps` contribution; the node SHALL accept `steps` in [1, S].

#### Scenario: Terminal publication at slot 3 of 8
- **WHEN** `KAdminPublishValue` fires at slot 3.
- **THEN** slots 4-7 are disabled, `steps == 4`, and the decider's `n_steps` equals the final `h`.
