## ADDED Requirements

### Requirement: FREG-001 A foreign proof enters only through a registry entry that binds its full verification interpretation
A foreign proof SHALL enter only through an entry `entry_d` of the version `sherlock-reg-entry/1` that binds proof system, curve, commitment scheme, SRS id, verifying-key digest, transcript id, public-input schema and count, adapter digest and policy, opened under `reg_root` at the fixed schema depth. `pi_count` SHALL be below `2^32`.

#### Scenario: Sherlock and Compact in one proof
- **WHEN** a Sherlock claim and a Compact claim are aggregated against `reg_root`
- **THEN** both open from the collection (positive `pos_gp3_sherlock_plus_compact`, a GP-003 acceptance line), and O39 (GP3-L1, `ClaimDag.lean`) is open

#### Scenario: Partial binding
- **WHEN** only the key digest matches, the transcript or the SRS differs, the input count is wrong, or the registry root is wrong
- **THEN** the lift is unsatisfiable (mutants `foreign_key_digest_only`, `foreign_swap_transcript`, `foreign_swap_srs`, `foreign_wrong_input_count`, `foreign_wrong_registry_root`), and the Quint invariant `StaleOrForeignRegRootRefused` of `foreignRegistry.qnt` holds

### Requirement: FREG-002 Both adapter key and inner key are registered
The foreign lift SHALL prove membership of both the adapter key and the inner foreign key in `reg_root`, SHALL require the adapter verification and the inner verification to complete in circuit (FVR-003) with no accumulator folded, and SHALL emit `fclaim_d = H(SHK_CLAIM, KIND_FOREIGN, vk_id_inner, adapter_id, out_d)` with a domain disjoint from `SHK_RUN`.

#### Scenario: Inner key absent
- **WHEN** the inner key or the adapter is absent from `reg_root`, or the inner obligation is dropped
- **THEN** the lift is unsatisfiable (mutants `foreign_unregistered_inner_vk`, `foreign_unregistered_adapter`, `foreign_drop_inner_obligation`), and the Quint invariant `ForeignClaimRequiresDualRegistryMembership` holds

### Requirement: FREG-003 A foreign field becomes a value digest only through the adapter relation
The adapter SHALL consume the verified public inputs as `pi_d` and SHALL derive `out_d` from them. A foreign field SHALL become a value digest only through the authenticated adapter relation. Proof parsing SHALL refuse trailing or missing data.

#### Scenario: Relabelled field or ignored input
- **WHEN** an adapter relabels a foreign field as `vd`, ignores its input, leaves a committed instance unbound, or accepts trailing proof bytes
- **THEN** the lift is unsatisfiable (mutants `foreign_value_relabel`, `foreign_adapter_ignores_input`, `foreign_unbound_committed_instance`, `foreign_trailing_proof_bytes`)

#### Scenario: Several keys and adapters
- **WHEN** two keys of one adapter class, or two distinct adapters, are used
- **THEN** the stock verifier accepts a real proof of each (positives `pos_gp3_two_keys_one_class`, `pos_gp3_distinct_adapters`)

### Requirement: FREG-004 The native adapter path is a milestone and never ticks GP-3
The native adapter path SHALL be a milestone task, scoped by X8 (N-10). No foreign proof SHALL be folded into an accumulator, under any SRS (FVR-003). GP-3 SHALL be reported complete only with the general verifier evidence of `sherlock-foreign-verifier`. An adapter-only GP-3 SHALL NOT be an acceptance path.

#### Scenario: Native path passes alone
- **WHEN** `pos_gp3_sherlock_plus_compact` passes and `pos_fv_plonk_proof_same_srs` or `pos_fv_plonk_proof_other_srs` has not passed
- **THEN** GP-003 stays open and the native path is reported as a milestone

### Requirement: FREG-005 The GP-3 hook AIRs meet the STARK kit gate
The foreign-lift AIRs of the node SHALL meet the STARK kit gate SG-1 to SG-6 of `design.md` section 9 inside the internal set (height at most `2^19`, at most 1,000 constraints and 200 interactions per AIR, 64 AIRs, 3,000 columns), with every AIR required and a fixed height in its menu shape.

#### Scenario: AIR omitted from the proof
- **WHEN** a proof omits one of the foreign-lift AIRs of the node
- **THEN** N1, N3 and `L` refuse it (mutant `foreign_air_omitted`), and the kit does not pass SG-1 before row T3 passes

#### Scenario: Positive and negative proofs on the stock verifier
- **WHEN** the positive `pos_gp3_sherlock_plus_compact` and the mutant `foreign_swap_srs` are proved with the stark-backend prover
- **THEN** the stock verifier accepts the first, refuses the second and accepts the second under its weakened control AIR; a prover panic or tracegen error earns no rejection credit

#### Scenario: Shape, budget and D3
- **WHEN** the foreign-lift AIRs of the node are keyed in a menu shape
- **THEN** each AIR has the fixed height of the shape, keygen refuses a shape outside the bounds above, D3 at the measured shape prints every term at 100 bits or more and every grindable term at 101 or more (SG-5), and the receipt records seconds per proved step (SG-6)
