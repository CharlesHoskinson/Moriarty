## ADDED Requirements

### Requirement: SQG-001 Proven 100-bit security is a gate
Every layer on the chain of a call SHALL have at least 100 bits of round-by-round soundness proven from theorems as stated, with no unproved conjecture. The layers SHALL be the leaf, the OpenVM leaf layer, the internal layers, `L` and the wrap. The design SHALL claim this security only after gate checks G1 to G5 all pass. Until then every bit count SHALL be reported as a calculator figure.

#### Scenario: Status today
- **WHEN** the security status is reported
- **THEN** it reads "not qualified", D3 has passed the calculator part of G2 only, and G1, the rest of G2, G3, G4 and G5 are open

### Requirement: SQG-002 One parameter set and one set of shape bounds per layer
Each layer SHALL have one parameter set and one set of shape bounds, as `FD/ARCHITECTURE.md` section 1.9 lists them (OD-1 to OD-4, LEAD). The sets SHALL freeze with the universal keys. Keygen SHALL refuse a key outside the bounds of its layer and a key whose per-round query vector differs from the frozen one.

#### Scenario: Key outside its bounds
- **WHEN** keygen receives an internal key of height `2^20`, or an app key whose query vector differs from 193/88/81/81
- **THEN** keygen refuses the key

### Requirement: SQG-003 The parameters of `L`
`L` SHALL use BabyBear with `EF` of degree 4, Midnight Poseidon over Fr for its Merkle tree and transcript, 8 BabyBear elements per Fr element, WHIR with `k_whir` = 4, 3 rounds and a final polynomial of `2^8`. It SHALL take two OOD samples per round (OD-1) and the per-fold PoW vector 15, 14, 13, 12, then 19, 18, 17, 16, then 22, 21, 20, 19 (OD-2). Its `mu` PoW SHALL be 22, its query PoW 20 and its LogUp PoW 21 (OD-3).

#### Scenario: One OOD sample
- **WHEN** `L` takes one OOD sample per round
- **THEN** D3 prints the OOD term at 98.0129 bits, and the set fails the margin rule

### Requirement: SQG-004 Margin rule
Every term of every layer SHALL be at least 100 bits. Every grindable term (folding, `mu`, query, LogUp) SHALL be at least 101 bits through its PoW constant. The app, leaf-aggregation and internal sets SHALL use query PoW 21. The internal set SHALL use LogUp PoW 20 (OD-4).

#### Scenario: Internal LogUp PoW 19
- **WHEN** the internal set takes the code rule's LogUp PoW 19
- **THEN** D3 prints the LogUp term at 100.4129 bits, and the set fails the margin rule

### Requirement: SQG-005 The stacked width of `L`
The `L` key SHALL have the shape bound `w_stack` at most 48 with `mu` PoW 22 (OD-3, LEAD, owner to confirm). D10, the trace heights of `L`, MAY tighten the bound. A tighter bound SHALL rerun D3 before the `L` key freezes. The internal set SHALL keep its own bound 512.

#### Scenario: Width at the bound
- **WHEN** D3 runs `L` at `w_stack` 48
- **THEN** the weakest term is 101.1727 bits with `mu` PoW 22, and 99.6198 bits with `mu` PoW 20 (MEASURED, D3 section 8, commit `81abb8f7a`)

#### Scenario: `mu` PoW 21
- **WHEN** a key takes `mu` PoW 21
- **THEN** both gates hold only to `w_stack` 37 (DERIVED), so `mu` PoW 21 does not meet the margin rule at the bound 48

#### Scenario: Width above the bound
- **WHEN** an `L` trace needs `w_stack` above 48
- **THEN** keygen refuses the key

### Requirement: SQG-006 Gate check G1, parameters frozen
G1 SHALL pass only when the owner confirms OD-1 to OD-4, OD-17 and OD-18, and every set and bound is fixed in the key-generation code with the explicit per-round query vectors.

#### Scenario: Unconfirmed decision
- **WHEN** one of OD-1 to OD-4, OD-17 or OD-18 is not confirmed by the owner
- **THEN** G1 stays open

### Requirement: SQG-007 Gate check G2, the calculator on the frozen sets
G2 SHALL pass only when the stock calculator, after build step 4, prints every term of every layer at its shape bounds at 100 bits or more, and every grindable term at 101 or more, on the frozen query vectors, with `L` at its key bound `w_stack` 48 or at a tighter bound from D10.

#### Scenario: Calculator part only
- **WHEN** D3 has run on the recommended sets with internal LogUp PoW 20 and `L` at `w_stack` 16, 18, 24, 32, 36 and 48
- **THEN** the calculator part of G2 is passed (weakest `L` term 101.1727 bits at 48 with `mu` PoW 22, MEASURED) and G2 stays open until the frozen sets exist

### Requirement: SQG-008 Gate check G3, two-OOD argument and transcript
G3 SHALL pass only when the prover, the stark-backend verifier, the calculator and the wrap's verifier of `L` follow the two-OOD order and the transcript of HDM-004 to HDM-006, the T-FS vectors are equal on all four implementations, and the Lean proof of the two-OOD bound builds.

#### Scenario: T-FS pending
- **WHEN** the T-FS vectors have not run
- **THEN** G3 is open

### Requirement: SQG-009 Gate check G4, Lean
G4 SHALL pass only when the restated theorem is committed with a build log of 0 sorry and the standard axioms only, and the concrete `L` key passes `ProvenRegime` by `decide`.

#### Scenario: Older regime
- **WHEN** the build at `bdb92973a` is offered for G4
- **THEN** G4 stays open, because its `ProvenRegime` requires 21-bit folding and `mu` PoW and does not check the recommended set

### Requirement: SQG-010 Gate check G5, theorem-to-configuration mapping
G5 SHALL pass only when WHIR (ACFY24, ePrint 2024/1586) and BCGM26 are fetched and read in full, and a written mapping shows that each hypothesis of WHIR Theorem 5.2 and of BCGM26 Theorem 9.2 holds for each frozen set, with the two-OOD order and the `gamma` batching of `q_i + 2` claims.

#### Scenario: Hypothesis not met
- **WHEN** a hypothesis does not hold for a frozen set
- **THEN** it is closed by the Lean two-OOD proof or by a parameter change, never by a conjecture, and the capacity regime of the WHIR reference code is excluded

### Requirement: SQG-011 No universal key freezes before the gate
No universal key SHALL freeze before G1 to G5 all pass. The frozen items of SPL-009 SHALL freeze only after the gate passes.

#### Scenario: Freeze request before the gate
- **WHEN** a freeze of the wrap key is requested while one gate check is open
- **THEN** the freeze is refused

### Requirement: SQG-012 Lean obligations of the security gate
The gate SHALL count Lean as passed only when BR-L2 is proved, with the per-fold PoW vector in `ProvenRegime`, separate radius and bound multiplicities, two OOD samples and the `w_stack` bound. Premise A2 (`lastLayerSound`) SHALL stay a named hypothesis.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** the two-OOD bound and the per-fold PoW in `ProvenRegime` are open, `s4_mult_three`, `s4_pow15_not_proven`, `remark_m1_not_proven` and `lKey_ok` are built at `bdb92973a` for the older example key, and BR-L1 is built with premises A1 to A7
