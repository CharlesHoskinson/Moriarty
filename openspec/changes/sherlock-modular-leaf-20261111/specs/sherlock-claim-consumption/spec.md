## ADDED Requirements

### Requirement: CLM-001 The SET grammar has typed forms with distinct domains
A `set_root` SHALL be the pinned-sponge hash of exactly one of `LEAF(claim_d, n_steps)`, `DEP(claim_d, n_steps, cover_root, in_link)`, `JOIN(l, r)`, `FOREIGN(fclaim_d)`, `ADM(adm_d)` and `VIEW(vw_d)`, with distinct domain words that differ from `SHK_RUN`, `SHK_DEPS` and every state and memory domain. This protocol SHALL NOT freeze before X12 passes (N-3).

#### Scenario: Wrong domain opening
- **WHEN** a `LEAF` hash is opened as a `JOIN`, a JOIN root is offered as if it had a head claim, or `n_steps` is forged in a SET leaf
- **THEN** the node is unsatisfiable (mutants `claim_descend_wrong_domain`, `claim_head_of_join`, `claim_nsteps_forged_in_set_leaf`)

### Requirement: CLM-002 Only sealed claims are consumed
Only the head claim of a verified SET SHALL be discharged. A SEG statement or a state digest SHALL NOT be consumed as a claim. The claim identity SHALL hold no address and no step count.

#### Scenario: B consumes A's Halted claim
- **WHEN** program B consumes A's Halted claim from a verified SET
- **THEN** B seals with its dependencies discharged (positive `pos_gp2_b_consumes_a_halted`), the Quint invariant `DepsDischargedIffVerifiedSet` of `claimConsumption.qnt` holds, and O38 (GP2-L1, `ClaimDag.lean`) is open

#### Scenario: Segment offered as a claim
- **WHEN** a SEG or a state digest is offered as a claim, the SET root is wrong, the outcome is swapped, or an address or step count enters the identity
- **THEN** the node is unsatisfiable (mutants `claim_consume_unsealed_segment`, `claim_wrong_set_root`, `claim_swap_outcome`, `claim_address_in_identity`, `claim_steps_in_identity`), and the Quint invariant `SegmentNeverConsumed` holds

### Requirement: CLM-003 A claim is discharged from any verified SET that contains it
DESCEND SHALL derive `SUB(R, x)` from a verified SET `R` by one-hash openings of `JOIN` or `DEP` nodes. A head opening SHALL give the head claim and its `n_steps`. The discharge SHALL NOT depend on the batching shape of the producer.

#### Scenario: Claim inside a JOIN or a DEP cover
- **WHEN** the producer's claim is offered as its own sealed proof, inside a JOIN of either bracketing, or inside a DEP cover
- **THEN** the consumer seals (positives `pos_gp2_claim_from_join_left_and_right_bracketing`, `pos_gp2_claim_from_dep_cover`), the Quint invariant `DischargeIndependentOfProducerShape` holds, and O51 (GP2-L2) is open

#### Scenario: Unrelated sub-root
- **WHEN** a sub-root not reached from the verified root is offered
- **THEN** the node is unsatisfiable (mutant `claim_sub_root_unrelated`)

### Requirement: CLM-004 A consumer cannot seal with an undischarged dependency
`deps_d` SHALL start at `EMPTY_DEPS` and each consume event SHALL set `deps_d' = H(SHK_DEPS, deps_d, claim_d')`. SEAL-WITH-DEPS SHALL require `DL.chain_d` equal to the opened `deps_d`, with order and repetition. Plain SEAL SHALL refuse a non-empty chain.

#### Scenario: Dropped, reordered or deduplicated dependency
- **WHEN** the opened `deps_d` differs from the chain of the discharge list in content, order or repetition, or a plain seal has a non-empty chain
- **THEN** the node is unsatisfiable (mutants `claim_seal_undischarged_deps`, `claim_drop_dependency`, `claim_chain_reordered`, `claim_chain_dedup_one_step_for_two`, `claim_plain_seal_with_nonempty_chain`), the Quint invariants `ChainOrderAndRepetitionExact` and `EmptyDepsOnlyBySeal` hold, and O52 (GP2-L3) is open

#### Scenario: Repeated and empty dependencies
- **WHEN** one claim is consumed twice, one producer feeds two consumers, or a run has no dependency
- **THEN** MockProver accepts each (positives `pos_gp2_repeated_consumption`, `pos_gp2_one_producer_two_consumers`, `pos_gp2_empty_deps_plain_seal`)

### Requirement: CLM-005 The input link binds the consumer's input to the producer's Halted value
If `DL.in_link = c_A` is set, SEAL-WITH-DEPS SHALL re-hash the consumer's `input_d` and `c_A` and SHALL require `outcome_d' == H(O, 1, 0, val_d)`. A Rejected or Exhausted claim SHALL be consumable as a chain element only, never as an input link or a value.

#### Scenario: Rejected producer or value mismatch
- **WHEN** the producer is Rejected, the value digests differ, the value opening is skipped, or a Rejected claim is used as a value
- **THEN** the node is unsatisfiable (mutants `claim_input_link_rejected_producer`, `claim_input_link_value_mismatch`, `claim_skip_value_opening`, `claim_rejected_as_value`), and the Quint invariant `InputLinkHaltedOnly` holds

#### Scenario: Rejected claim as metadata
- **WHEN** a Rejected claim is consumed as a chain element, or a structured value passes across programs
- **THEN** MockProver accepts it (positives `pos_gp2_rejected_as_metadata`, `pos_gp2_struct_value_across_programs`, `pos_gp2_three_runs_collection`)

### Requirement: CLM-006 The scope of GP-2 is gated on OD-20
The built path SHALL be input consumption at the seal (OD-20 option a), with the circuit side of a mid-run consume rule specified so that it needs no new node relation. If the owner reads GP-2 as requiring a mid-run consume rule or a genesis-bound dependency, that work SHALL be required by this change and option (a) SHALL be only a milestone.

#### Scenario: Owner decision pending
- **WHEN** the GP-2 hook is offered as complete for GP-002 while OD-20 is open
- **THEN** `pos_gp2_b_consumes_a_halted` and `pos_gp2_three_runs_collection` count as milestone evidence only, and the GP-002 target stays open until OD-20 is answered

### Requirement: CLM-007 A seal premise enters a discharge list only as the premise element
`DL` SHALL be `(chain_d, cover_root, in_link, prem_root)`. The premise element SHALL be the first element of a DL that a seal uses. It SHALL set `prem_root` and SHALL leave `chain_d`, `cover_root` and `in_link` unchanged. A chain element or an input-link element SHALL take only a `LEAF`, `DEP` or `FOREIGN` head.

#### Scenario: Premise in the chain
- **WHEN** an `ADM` or `VIEW` head is appended to `chain_d` or used as an input link
- **THEN** the node is unsatisfiable (mutant `claim_premise_in_chain`), and a run with no dependency still seals to a `LEAF` (positive `pos_gp2_empty_deps_plain_seal`)
