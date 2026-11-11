## ADDED Requirements

### Requirement: DCD-001 The run decider publishes only `[prog_d, input_d, outcome_d, n_steps]`
The run decider SHALL verify the root proof under pinned node keys and SHALL publish exactly `[prog_d, input_d, outcome_d, n_steps]` with `n_steps` in `[1, 2^64)` (SRC-006). It SHALL refuse Running. It SHALL NOT publish `init_d`, `fin_d`, `pre_d`, `post_d`, a memory root, a tail digest, `len`, an address or `result_be`. Kind 7 under `/1` SHALL follow OD-16.

#### Scenario: Every outcome
- **WHEN** a sealed Halted, Rejected or Exhausted run is decided, with every admitted rejection code and exhaustion kind
- **THEN** the public inputs are exactly the four values and no state digest (positives `pos_decider_all_outcomes`, `pos_decider_halted_scalar_and_struct`, `pos_decider_genesis_terminal_opening`), the Quint invariant `NoStateDigestOnChain` of `deciderOutcome.qnt` holds, and O34 (DEC-L1, `OutcomeStatement.lean`) is open

#### Scenario: Running or forged statement
- **WHEN** the root is Running, the terminal payload is unbound, `h_lo` is not zero, `n_steps` enters the claim, a state digest is public, or `n_steps = 2^64`
- **THEN** the decider is unsatisfiable (mutants `decider_running_as_terminal`, `decider_unbound_terminal_payload`, `decider_nonzero_h_lo`, `decider_steps_in_claim`, `decider_public_state_digest`, `decider_nsteps_2p64`)

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
The collection decider SHALL publish `[reg_root, set_root, shape_d]` with a schema separate from the run entry. No claim count SHALL be public, range-checked or used for soundness (OD-2, OWNER to confirm). Settlement SHALL have no per-claim side effect keyed by `set_root` or `shape_d`.

#### Scenario: Three runs joined
- **WHEN** three sealed runs are joined and decided
- **THEN** each claim opens from `set_root` (positive `pos_decider_collection_three_runs`, run at group 6 integration), and the Quint invariant `EveryJoinedClaimOpens` holds

#### Scenario: Diamond-shared claim
- **WHEN** one claim has two paths in `set_root`
- **THEN** it settles at most once (Quint invariant `DiamondAncestorOpenedAtMostOnce`), a stale or foreign registry root is refused (Quint invariant `StaleOrForeignRegRootRefused`), and O35 (DEC-L2, `ClaimDag.lean`) is open

### Requirement: DCD-005 The decider adds no range-table tag and stays under its row gate
The decider SHALL use 8-bit byte decomposition only and SHALL add no range-table tag. It SHALL stay at or below 1,000,000 rows with its table rows and usable domain recorded with headroom. A closing proof SHALL be added only if the measured decider exceeds the gate (N-5).

#### Scenario: Cost model on both deciders
- **WHEN** `cost_model` runs on the run decider and the collection decider
- **THEN** rows are at or below 1,000,000 and table rows do not grow (positive `pos_decider_table_rows`), and a new range tag is refused by the cost model (`decider_new_range_tag`)
