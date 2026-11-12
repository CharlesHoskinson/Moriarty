## ADDED Requirements

### Requirement: CONT-001 Tier 1 folds leaves by segment merge in any bracketing
The tier 1 scheduler SHALL keep a carry stack of proven SEGs by rank and SHALL merge two adjacent SEGs of equal rank with NODE MERGE. The fold order SHALL be a prover choice and SHALL NOT change the statement. Continuation mode 1 SHALL add no circuit cell and SHALL use only MERGE and SEAL.

#### Scenario: Out-of-order completion
- **WHEN** leaves finish out of order with randomized worker latency
- **THEN** the root SEG interval and the sealed statement are unchanged (positive `pos_tier1_stack_bound`), and the Quint invariants `OutOfOrderWorkerCompletionPreservesSegInterval` and `MaxStackDepthLeLogLPlus2` of `continuation.qnt` hold

### Requirement: CONT-002 Mode 1 stop and resume gives the same claim
A run stopped at height `h` and resumed from a validated checkpoint SHALL give the same `prog_d`, `input_d`, `outcome_d`, `n_steps` and `claim_d` as the uninterrupted run. The first suffix leaf SHALL have `h_pre = h` and `pre_d` equal to the record's `post_d`. Pause and resume SHALL execute no machine event.

#### Scenario: Allocating run stopped once
- **WHEN** the allocating GP-001 acceptance run is stopped once, at a power-of-two leaf boundary and at an odd leaf, and resumed
- **THEN** the four values and `claim_d` equal the uninterrupted run (positives `pos_mode1_stop_resume_allocating_run`, `pos_mode1_resume_nonempty_tail`, `pos_mode1_resume_nested_frames_tasks`), the Quint invariant `ResumeProducesIdenticalClaimD` holds, and O36 (CONT-L1) is open

#### Scenario: Suffix tampered
- **WHEN** fuel, input or height is changed in the suffix, a predecessor is skipped, or a record that the suffix reads is changed with restore validation off
- **THEN** the suffix cannot join the prefix (mutants `cont_refresh_fuel`, `cont_change_input`, `cont_reset_height`, `cont_skip_predecessor`), and the leaf is unsatisfiable (mutant `cont_tampered_read_cell_unvalidated`)

### Requirement: CONT-003 A checkpoint is never a run or a claim
A prefix SEG with a Running state SHALL NOT be sealed, sent to `L` as a run or consumed as a claim. Exhausted SHALL be terminal and SHALL NOT trigger a checkpoint.

#### Scenario: Checkpoint published
- **WHEN** a checkpoint SEG is passed to `L` as a run or to a GP-2 consumer
- **THEN** it is refused (mutants `cont_publish_as_run`, `cont_ckpt_as_gp2_claim`), the Quint invariant `CheckpointRejectedBySealAndGP2` holds, and O37 (CONT-L2) is open

### Requirement: CONT-004 The resume refuses a different key set
The resume SHALL refuse, before any proving, a checkpoint whose manifest differs from the key manifest or whose bundle lacks a proof. Continuation mode 1 SHALL require Profile A. Mode 2 SHALL stay out of scope (AD-15).

#### Scenario: Manifest mismatch
- **WHEN** the checkpoint manifest differs from the key manifest, or a bundle proof is missing
- **THEN** the resume refuses before proving (host refusals `cont_mismatched_key_manifest`, `cont_bundle_missing_proof`)
