## ADDED Requirements

### Requirement: ZML-001 Profile B is benchmark-only
The ZmLogUp backend SHALL be built only behind a feature, and not before milestone M4 (L-5, OD-7). It SHALL NOT be described as supporting stop and resume of a run. It SHALL be listed as changing the leaf statement, the node, the seal and the continuation modules, and as replacing the `digest_inj` premise. It SHALL be offered only with the Horner boundary.

#### Scenario: Checkpoint resets the memory sum
- **WHEN** a checkpoint resets the memory sum of a run
- **THEN** the seal refuses (mutant `checkpoint_resets_debt`)

#### Scenario: Not part of completion
- **WHEN** the completion of this change is assessed
- **THEN** the Profile B task "benchmark after M4" is not counted, and O21, O22 and O30 are open and not scheduled

### Requirement: ZML-002 Every term is bound to the committed instance
Every write, read and multiplicity cell SHALL equal its cell in the committed instance column. The challenges SHALL be scalar public inputs equal to the hash of `d_root`. A commitment point SHALL use the subgroup-checked assign.

#### Scenario: Term not committed
- **WHEN** a multiplicity, an activity bit or a tuple is not committed
- **THEN** the test refuses the witness (mutants `multiplicity_uncommitted`, `activity_uncommitted`, `tuple_uncommitted`)

#### Scenario: Challenge or commitment forged
- **WHEN** the challenges are not derived from `d_root`, a commitment is substituted in the node, or a commitment is not subgroup checked
- **THEN** the seal refuses (mutants `challenge_not_derived`, `commitment_substituted_in_node`, `commitment_not_subgroup_checked`), and O21 (`WriteOnceLogUp.lean`) is open

### Requirement: ZML-003 Rational terms are well formed
An active entry SHALL have a non-zero denominator with `(alpha - E) inv = 1`. An inactive entry SHALL contribute zero. A read entry SHALL be charged.

#### Scenario: Free inverse
- **WHEN** the inverse is free, an inactive term is non-zero, an active denominator is zero, or a read entry is not charged
- **THEN** MockProver refuses the witness (mutants `inverse_unconstrained`, `inactive_term_nonzero`, `zero_denominator_active`, `read_entry_not_charged`)

### Requirement: ZML-004 Causality and single writes
Every read SHALL keep `addr < len_cur`, and every address SHALL be written once. The seal SHALL check that the run sum is zero.

#### Scenario: Read balanced by a later write
- **WHEN** a read is balanced by a later write, an address is written twice, or the seal does not check the sum
- **THEN** MockProver refuses the witness (mutants `read_before_write_balanced_later`, `address_written_twice`, `seal_sum_unchecked`), and the Quint model `writeOnceLogup.qnt` keeps the precommit phase apart from the challenge phase

### Requirement: ZML-005 Imported tail rows are debits
Every imported window row and every old-record provider row SHALL be a read entry of the balance, also when no request uses it. Backend capabilities SHALL be fixed at configuration time.

#### Scenario: Forged imported tail
- **WHEN** the second of two leaves imports a tail record that the first did not write, with every commitment binding intact
- **THEN** the seal refuses (mutant `zm_imported_tail_forged`), and the control without the debit accepts it

#### Scenario: Unused imported row left out
- **WHEN** an imported row that no request uses is left out of the sum
- **THEN** the committed-entry binding refuses it (mutant `zm_imported_row_not_debited_when_unused`), and the positives `ok_logup_two_and_three_leaf_cancel`, `ok_logup_unused_write_m0`, `ok_logup_repeated_cold_reads` and `ok_logup_input_records` pass
