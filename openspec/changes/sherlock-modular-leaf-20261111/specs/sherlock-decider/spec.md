## ADDED Requirements

### Requirement: DCD-001 The run statement is published by kit L AND WRAP
DCD-001 SHALL be met by LW-007 of `sherlock-processor-final-20261111`. A general run SHALL publish `[prog_d, input_d, outcome_d, n_steps]` as effect words of the call vector. Kind 7 under `/1` SHALL follow AD-16. No halo2 run decider SHALL be built (superseded 2026-11-11, old text at commit `0ab8cd79d`).

#### Scenario: Every outcome
- **WHEN** a sealed Halted, Rejected or Exhausted run is wrapped
- **THEN** the effect words carry exactly the four values (positives `pos_decider_all_outcomes`, `pos_decider_halted_scalar_and_struct`, `pos_decider_set_head_leaf_and_dep` in kit L AND WRAP), the Quint invariant `NoStateDigestOnChain` of `deciderOutcome.qnt` holds, and O34 (DEC-L1) is open

### Requirement: DCD-002 The wrap verifies `L` in place of the pairing of a carried accumulator
DCD-002 SHALL be met by LW-002 to LW-006 of `sherlock-processor-final-20261111`. No accumulator SHALL be carried to the ledger proof. The wrap SHALL verify `L` under the frozen `L` key and SHALL bind every public word to the call vector.

#### Scenario: Unbound call value
- **WHEN** one public word of a real wrap proof is flipped
- **THEN** `validate_call` refuses the call (SRC-020), and the wrap mutants of LW-004 to LW-006 are refused

### Requirement: DCD-003 Replay is refused by `claim_d` only
DCD-003 SHALL be met by LW-010 of `sherlock-processor-final-20261111`. The replay identity SHALL be `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`, without `n_steps` and without any physical field (SRC-007, SRC-023).

#### Scenario: Second partition
- **WHEN** the same run is proved under a second leaf partition and submitted
- **THEN** the second call is refused, and the Quint invariant `PartitionReplayRefusedByClaimD` holds

### Requirement: DCD-004 The collection statement carries no claim count
DCD-004 SHALL be met by LW-009 of `sherlock-processor-final-20261111`. The collection statement SHALL be `[reg_root, set_root, shape_d]` with no claim count (AD-2, OWNER to confirm). Settlement and claim opening SHALL be owned by the LEDGER CONTRACT (LDG-003 to LDG-005).

#### Scenario: Three runs joined
- **WHEN** three sealed runs are joined and wrapped
- **THEN** the statement is exactly `[reg_root, set_root, shape_d]` (positive `pos_decider_collection_three_runs` in kit L AND WRAP), and each claim opens through the LEDGER CONTRACT (LDG-004)

### Requirement: DCD-005 The derived gates replace the decider row gate
DCD-005 SHALL be met by DG-001 and DG-002 of `sherlock-processor-final-20261111`. The decider gate of 1,000,000 rows SHALL NOT apply. The wrap SHALL fit 85 percent of `2^k - 7` usable rows at the smallest k of 19, 20 and 21, with one fixed 2^16-row range table.

#### Scenario: Wrap rows measured
- **WHEN** M7 measures the G4 wrap rows
- **THEN** the k of the wrap key is the smallest of 19, 20 and 21 at which the rows fit the gate, and no circuit is above k = 21

### Requirement: DCD-006 The run statement comes from a sealed SET head
DCD-006 SHALL be met by LW-008 of `sherlock-processor-final-20261111`. The run root SHALL be a SET root sealed by NODE and opened with one hash as `LEAF(claim_d, n_steps)` or `DEP(claim_d, n_steps, cover_root, in_link)`. No seal predicate SHALL be checked again after NODE.

#### Scenario: Unsealed root or forged head
- **WHEN** a SEG root is offered as a run, or the SET head preimage differs from the run statement
- **THEN** the proof is refused (mutants `decider_seg_root_as_run`, `decider_set_head_forged` in kit L AND WRAP), and the seal predicates are refused in NODE (mutants `node_seal_running`, `node_seal_nonzero_hlo`, `node_seal_header_word_forged`, `node_seal_nonempty_deps`)

### Requirement: DCD-007 Under mid-run consumption the run statement binds the registry root
DCD-007 SHALL be met by LW-011 of `sherlock-processor-final-20261111`. If AD-20 selects option (b), a `DEP` head with a FOREIGN member SHALL be refused or SHALL be checked against the contract's `reg_root`. Under option (a) this requirement SHALL have no effect.

#### Scenario: Foreign member under another root
- **WHEN** AD-20 option (b) is taken and a `DEP` head has a FOREIGN member lifted under another registry root
- **THEN** the wrap or the contract refuses it (mutant `decider_dep_foreign_other_reg_root`)
