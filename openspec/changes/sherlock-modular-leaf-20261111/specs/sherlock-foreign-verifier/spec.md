## ADDED Requirements

### Requirement: FVR-001 General foreign verification runs in bounded, authenticated work units
The general verifier SHALL verify a proof of any registered PLONK or halo2 circuit on the same curve and commitment scheme in FV-WORK units. Each unit SHALL continue `fv_state_d = H(SHK_FV, entry_d, phase, cursor, transcript_d, acc_d, pi_d, consumed_d, work_root)` of its child and SHALL do at most the unit budget of X14. All units SHALL consume one proof stream in order.

#### Scenario: Forked state or skipped element
- **WHEN** a unit continues a state other than its child's, skips a proof element, ignores a trailing element, or swaps the registry entry mid-run
- **THEN** the unit is unsatisfiable (mutants `fv_state_forked`, `fv_skip_proof_element`, `fv_trailing_element_ignored`, `fv_entry_swapped_mid_run`), and the Quint invariants `UnitsContinueCommittedState` and `ProofStreamConsumedExactlyOnce` of `foreignRegistry.qnt` hold

#### Scenario: Unit boundaries
- **WHEN** a proof is verified with a unit boundary in every phase
- **THEN** the result equals a host verifier of the same registry entry (positive `pos_fv_unit_boundaries_every_phase`)

### Requirement: FVR-002 A claim is emitted only at completion
FV-SCHED SHALL emit the foreign SET in the `FOREIGN` form only at `phase = DONE`, after every phase. The adapter SHALL be applied to `pi_d` only. Challenges SHALL come from the registered transcript and points SHALL pass their encoding checks.

#### Scenario: Claim from an unfinished phase
- **WHEN** a claim is taken from an unfinished phase, the adapter runs on unverified inputs, a challenge is not from the transcript, or a point is not on the curve
- **THEN** the unit is unsatisfiable (mutants `fv_claim_at_unfinished_phase`, `fv_adapter_on_unverified_inputs`, `fv_challenge_not_from_transcript`, `fv_point_not_on_curve`), the Quint invariant `NoClaimBeforeDone` holds, and O53 (GP3-L2, `ForeignVerify.lean`) is open

### Requirement: FVR-003 A proof under another SRS is fully checked in circuit
FV-FINAL SHALL check every foreign proof, under the core SRS or under another SRS, fully in circuit in work units with the verifier parameters of its registry entry, with nothing deferred to `L` or the wrap (AD-22 option a). The STARK node folds no accumulator (NODE-004, NODE-007), so no foreign obligation SHALL be folded into an accumulator. A second deferred accumulator per SRS SHALL be used only by owner decision.

#### Scenario: Foreign obligation folded or deferred
- **WHEN** a foreign proof under any SRS has its obligation folded into an accumulator or deferred to `L` or the wrap
- **THEN** the unit is unsatisfiable (mutant `fv_other_srs_folded_into_core_acc`), and the positives `pos_fv_plonk_proof_same_srs` and `pos_fv_plonk_proof_other_srs` pass against a host verifier

### Requirement: FVR-004 The subkits wait for X14 and the general path is never dropped
FV-PARSE, FV-TRANSCRIPT, FV-ARITH, FV-FINAL and FV-SCHED SHALL NOT start before X14 gives the operation inventory and the unit budget. One unit plus the verification of one `FV` child SHALL fit the node shape bounds of the internal set, with at most 3,300 columns and the 17-bit range checker for FV work units only (rulings O1 and O2). If no unit size fits, GP-3 SHALL stay incomplete and the scope SHALL NOT be reduced to adapter classes.

#### Scenario: No unit size fits
- **WHEN** X14 finds no unit size that fits the node gate
- **THEN** GP-003 stays open, the general path stays in the change, and the result is reported to the owner

### Requirement: FVR-005 A later phase reads only the values absorbed earlier
`work_root` SHALL be an indexed store of typed records `(index, kind, value)`, empty at the first unit. FV-PARSE SHALL write each element at index `cursor`. FV-TRANSCRIPT SHALL write each challenge at its schema index. FV-ARITH and FV-FINAL SHALL read elements and challenges only by openings against `work_root`. FV-SCHED SHALL own the index schema per `transcript_id` and SHALL allow `DONE` only with every scheduled record written.

#### Scenario: Substituted operand or swapped challenge
- **WHEN** an MSM term uses a commitment other than the one absorbed, or a challenge is read from another schema index
- **THEN** the unit is unsatisfiable (mutants `fv_operand_substituted`, `fv_challenge_index_swapped`), and the Quint invariant `LaterPhaseReadsAbsorbedValue` of `foreignRegistry.qnt` holds

### Requirement: FVR-006 FV work units meet the STARK kit gate with their own column bound
The FV work-unit AIRs SHALL meet SG-1 to SG-6 of `design.md` section 9 inside the internal set, except that a unit MAY use at most 3,300 columns and the 17-bit range checker (owner rulings O1 and O2 of 2026-11-13). No other kit SHALL use these two exceptions.

#### Scenario: Unit over its column bound
- **WHEN** an FV unit is keyed with more than 3,300 columns, or a non-FV kit is keyed with the 17-bit checker
- **THEN** keygen refuses the key (refusals `fv_unit_over_3300_columns`, `non_fv_17bit_checker`)

#### Scenario: Positive and negative units on the stock verifier
- **WHEN** `pos_fv_unit_boundaries_every_phase` and the mutant `fv_state_forked` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR, a proof that omits a unit AIR is refused (mutant `fv_air_omitted`), and D3 at the measured unit shape prints every term at 100 bits or more
