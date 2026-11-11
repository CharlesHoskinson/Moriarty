## ADDED Requirements

### Requirement: DCD-001 The run decider publishes only `[prog_d, input_d, outcome_d, n_steps]`
The run decider SHALL verify the root proof under pinned node keys and SHALL publish exactly `[prog_d, input_d, outcome_d, n_steps]` with `n_steps` in `[1, 2^64)` (SRC-006). It SHALL refuse Running. It SHALL NOT publish `init_d`, `fin_d`, `pre_d`, `post_d`, a memory root, a tail digest, `len`, an address or `result_be`. Kind 7 under `/1` SHALL follow OD-16.

#### Scenario: Every outcome
- **WHEN** a sealed Halted, Rejected or Exhausted run is decided, with every admitted rejection code and exhaustion kind
- **THEN** the public inputs are exactly the four values and no state digest (positives `pos_decider_all_outcomes`, `pos_decider_halted_scalar_and_struct`, `pos_decider_set_head_leaf_and_dep`), the Quint invariant `NoStateDigestOnChain` of `deciderOutcome.qnt` holds, and O34 (DEC-L1, `OutcomeStatement.lean`) is open

#### Scenario: Forged statement
- **WHEN** `n_steps` enters the claim, a state digest is public, or `n_steps = 2^64`
- **THEN** the decider is unsatisfiable (mutants `decider_steps_in_claim`, `decider_public_state_digest`, `decider_nsteps_2p64`)

### Requirement: DCD-002 Every carried obligation reaches the pairing
The decider SHALL pin the root key, SHALL fold and pair every carried accumulator in circuit, and SHALL bind every public value to the exact ledger call transcript.

#### Scenario: Carry dropped or pairing skipped
- **WHEN** the root carry is dropped, the pairing is skipped, the root key is unpinned, or a call value is unbound
- **THEN** the decider is unsatisfiable (mutants `decider_drop_root_carry`, `decider_skip_pairing`, `decider_unpinned_root_key`, `decider_unbound_call_value`), and at integration each flipped public value is refused by `validate_call` (SRC-020)

### Requirement: DCD-003 Replay is refused by `claim_d` only
The replay identity SHALL be `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`, without `n_steps` and without any physical field (SRC-007, SRC-023).

#### Scenario: Second partition
- **WHEN** the same run is proved under a second leaf partition and submitted
- **THEN** the second call is refused (integration test, SRC-023), and the Quint invariant `PartitionReplayRefusedByClaimD` holds

### Requirement: DCD-004 The collection decider publishes `[reg_root, set_root, shape_d]` and no claim count
The collection decider SHALL publish `[reg_root, set_root, shape_d]` with a schema separate from the run entry. No claim count SHALL be public, range-checked or used for soundness (OD-2, OWNER to confirm). The collection entry SHALL publish no per-claim value. Settlement and claim opening SHALL be owned by the LEDGER CONTRACT (LDG-003 to LDG-005).

#### Scenario: Three runs joined
- **WHEN** three sealed runs are joined and decided
- **THEN** the decider publishes exactly `[reg_root, set_root, shape_d]` (positive `pos_decider_collection_three_runs`, run at group 6 integration), and each claim opens from `set_root` through the LEDGER CONTRACT (LDG-004)

#### Scenario: Stale registry root
- **WHEN** the transcript carries a registry root other than the contract's
- **THEN** the call is refused (Quint invariant `StaleOrForeignRegRootRefused`)

### Requirement: DCD-005 The decider adds no range-table tag and stays under its row gate
The decider SHALL use 8-bit byte decomposition only and SHALL add no range-table tag. It SHALL stay at or below 1,000,000 rows with its table rows and usable domain recorded with headroom. A closing proof SHALL be added only if the measured decider exceeds the gate (N-5).

#### Scenario: Cost model on both deciders
- **WHEN** `cost_model` runs on the run decider and the collection decider
- **THEN** rows are at or below 1,000,000 and table rows do not grow (positive `pos_decider_table_rows`), and a new range tag is refused by the cost model (`decider_new_range_tag`)

### Requirement: DCD-006 The run entry opens a sealed SET head and checks no seal predicate
The run root SHALL be a SET root sealed by NODE. The decider SHALL recompute `claim_d` from the four values and SHALL open the root with one hash as `LEAF(claim_d, n_steps)` or `DEP(claim_d, n_steps, cover_root, in_link)`. It SHALL refuse every other root. It SHALL NOT open a state header. On the M4 linear path one SEAL node SHALL run before the decider.

#### Scenario: Unsealed root or forged head
- **WHEN** a SEG root (Running or terminal) is offered to the run entry, the SET head preimage differs from the published values, or a root of another SET form is opened as a run head
- **THEN** the decider is unsatisfiable (mutants `decider_seg_root_as_run`, `decider_set_head_forged`), and the seal predicates are refused in NODE (mutants `node_seal_running`, `node_seal_nonzero_hlo`, `node_seal_header_word_forged`, `node_seal_nonempty_deps`)

### Requirement: DCD-007 Under mid-run consumption the run entry binds the registry root
If OD-20 selects option (b), the run entry SHALL either refuse a `DEP` head whose cover holds a FOREIGN member, or SHALL read `reg_root` into the run-entry transcript from contract state and check it there. The four published run values SHALL NOT change. Under option (a) this requirement SHALL have no effect.

#### Scenario: Foreign member under another root
- **WHEN** OD-20 option (b) is taken and a `DEP` head offered to the run entry has a FOREIGN member lifted under a root other than the contract's
- **THEN** the decider or the contract refuses it (mutant `decider_dep_foreign_other_reg_root`)
