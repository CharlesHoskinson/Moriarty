## ADDED Requirements

### Requirement: SQG-001 Proven 100-bit security is a gate
Every layer on the chain of a call SHALL have at least 100 bits of round-by-round soundness proven from theorems as stated, with no unproved conjecture. The layers SHALL be the leaf, the OpenVM leaf layer, the internal layers, `L` and the wrap. Every STARK layer SHALL run WHIR in unique decoding (OD-19 for the internal and `L` layers). The design SHALL claim this security only after gate checks G1 to G4 pass with G5. Until then every bit count SHALL be reported as a calculator figure.

#### Scenario: Status today
- **WHEN** the security status is reported
- **THEN** it reads "not qualified", D3 has passed the calculator part of G2 only, G5 is discharged at the paper level, and G1, the rest of G2, G3 and G4 are open

### Requirement: SQG-002 One parameter set and one set of shape bounds per layer
Each layer SHALL have one parameter set and one set of shape bounds, as `FD/ARCHITECTURE.md` section 1.9 lists them (app and leaf-aggregation sets OD-4, LEAD; internal and `L` sets OD-19). Every layer SHALL have the shape bound that the total interaction count of one proof is below `p`. The sets SHALL freeze with the universal keys. Keygen SHALL refuse a key outside the bounds of its layer and a key whose per-round query vector differs from the frozen one.

#### Scenario: Key outside its bounds
- **WHEN** keygen receives an internal key of height `2^20`, an app key whose query vector differs from 193/88/81/81, or an internal key whose query vector differs from 97/82/81
- **THEN** keygen refuses the key

### Requirement: SQG-003 The parameters of `L`
`L` SHALL use BabyBear with `EF` of degree 4, Midnight Poseidon over Fr for its Merkle tree and transcript, 8 BabyBear elements per Fr element, WHIR with `k_whir` = 4, 3 rounds, a final polynomial of `2^8` and `WhirProximityStrategy::UniqueDecoding` in every round. It SHALL use log blowup 3, queries 98/83/82, folding PoW 0, `mu` PoW 6, query PoW 20, LogUp PoW 18 and one OOD sample per round in the stock WHIR order (OD-19).

#### Scenario: OOD term with one sample
- **WHEN** D3 runs the `L` set at `w_stack` 48
- **THEN** the OOD terms are 108.6276 bits (round 0) and 112.6276 bits (round 1) with list size 1, MEASURED (`UDS:347-348`, where `UDS` is the D3 calculator log `~/.cache/ud-sizing/ud-sweep.log` of `research/sherlock-design-paper-review-20261111/UD-SIZING.md`)

### Requirement: SQG-004 Margin rule
Every term of every layer SHALL be at least 100 bits. Every grindable term (folding, `mu`, query, LogUp) SHALL be at least 101 bits through its PoW constant. The app, leaf-aggregation and internal sets SHALL use query PoW 21 (OD-4, OD-19).

#### Scenario: App query PoW 20
- **WHEN** the app and leaf-aggregation sets take query PoW 20
- **THEN** their query terms are 100.09 and 100.01 bits, and the sets fail the margin rule

### Requirement: SQG-005 The stacked width of `L`
The `L` key SHALL have the shape bound `w_stack` at most 48 with `mu` PoW 6 (LEAD under OD-19, owner to confirm). D10, the trace heights of `L`, MAY tighten the bound. A tighter bound SHALL rerun D3 before the `L` key freezes. The internal set SHALL keep its own bound 512.

#### Scenario: Width at the bound
- **WHEN** D3 runs `L` at `w_stack` 48 with `mu` PoW 6
- **THEN** the weakest term is 101.0730 bits (`mu`), MEASURED (`UDS:351`)

#### Scenario: Width above the bound
- **WHEN** an `L` trace needs `w_stack` above 48
- **THEN** keygen refuses the key

### Requirement: SQG-006 Gate check G1, parameters frozen
G1 SHALL pass only when the owner confirms OD-4, OD-17 and OD-18, the internal and `L` sets of OD-19 are fixed, and every set and bound is fixed in the key-generation code with the explicit per-round query vectors and the global interaction bound.

#### Scenario: Unconfirmed decision
- **WHEN** one of OD-4, OD-17 or OD-18 is not confirmed by the owner
- **THEN** G1 stays open

### Requirement: SQG-007 Gate check G2, the calculator on the frozen sets
G2 SHALL pass only when the stock calculator, after build step 4, prints every term of every layer at its shape bounds at 100 bits or more, and every grindable term at 101 or more, on the frozen query vectors, with `L` at the `w_stack` that D10 measures.

#### Scenario: Calculator part only
- **WHEN** D3 has run the internal set, `L` at `w_stack` 36 and 48 with `mu` PoW 6 and at 16 with `mu` PoW 5, and the app and leaf-aggregation sets
- **THEN** the calculator part of G2 is passed (weakest `L` term 101.0730 bits, weakest internal term 101.1643 bits, MEASURED) and G2 stays open until the frozen sets exist and D10 has run

### Requirement: SQG-008 Gate check G3, transcript
G3 SHALL pass only when the prover, the stark-backend verifier, the calculator and the wrap's verifier of `L` follow the stock one-sample WHIR order of HDM-006 and the transcript of HDM-004 and HDM-005, and the T-FS vectors are equal on all four implementations.

#### Scenario: T-FS pending
- **WHEN** the T-FS vectors have not run
- **THEN** G3 is open

### Requirement: SQG-009 Gate check G4, Lean
G4 SHALL pass only when the restated theorem is committed with a build log of 0 sorry and the standard axioms only, and the concrete `L` key passes `ProvenRegime` by `decide`.

#### Scenario: Older regime
- **WHEN** the build at `bdb92973a` is offered for G4
- **THEN** G4 stays open, because its `ProvenRegime` takes the list decoding regime and does not check the unique decoding sets

### Requirement: SQG-010 Gate check G5, theorem-to-configuration mapping
G5 SHALL pass only when a written mapping shows that each hypothesis of WHIR Theorem 5.2 holds for each frozen set in unique decoding, by WHIR Lemma 4.10, Corollary 4.11, the unique decoding branch of Theorem 4.8 and Lemma 4.25, and the unique decoding branch of SWIRL Theorem 4.2.4, with one OOD sample and the `gamma` batching of `q_i + 1` claims.

#### Scenario: Mapping written
- **WHEN** the status of G5 is reported
- **THEN** it is discharged at the paper level for every layer (`research/sherlock-design-paper-review-20261111/G5-PAPER-MAP.md` sections 1, 2 and 6), and the capacity regime of the WHIR reference code is excluded

### Requirement: SQG-011 No universal key freezes before the gate
No universal key SHALL freeze before G1 to G4 pass with G5 and FP-8 closes (FPR-019). The frozen items of SPL-009 SHALL freeze only after the gate passes.

#### Scenario: Freeze request before the gate
- **WHEN** a freeze of the wrap key is requested while one gate check is open
- **THEN** the freeze is refused

### Requirement: SQG-012 Lean obligations of the security gate
The gate SHALL count Lean as passed only when BR-L2 is proved, with `ProvenRegime` in the unique decoding regime, the frozen per-round query vector, the PoW constants, one OOD sample per round and the `w_stack` bound. Premise A2 (`lastLayerSound`) SHALL stay a named hypothesis.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** the unique decoding `ProvenRegime` is open, and BR-L1 is built at `bdb92973a` with premises A1 to A7 against the list decoding example key
