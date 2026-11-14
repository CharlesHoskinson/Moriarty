## ADDED Requirements

### Requirement: SQG-001 Proven 100-bit security is a gate
Every layer on the chain of a call SHALL have at least 100 bits of round-by-round soundness proven from theorems as stated, with no unproved conjecture. The layers SHALL be the leaf, the OpenVM leaf layer, the internal layers, `L`, `L_Fr` and the wrap. Every STARK layer SHALL run WHIR in unique decoding (OD-19 for the internal and `L` layers). The design SHALL claim this security only after gate checks G1 to G4 pass with G5. Until then every bit count SHALL be reported as a calculator figure.

#### Scenario: Status today
- **WHEN** the security status is reported
- **THEN** it reads "not qualified", D3 has passed the calculator part of G2 only, G4 has passed for the unique decoding `L`, internal, app and leaf-aggregation sets at `6661325d4`, G5 is discharged at the paper level, and G1, the rest of G2, G3 and the rest of G4 are open

### Requirement: SQG-002 One parameter set and one set of shape bounds per layer
Each layer SHALL have one parameter set and one set of shape bounds, as `FD/ARCHITECTURE.md` section 1.9 lists them (app and leaf sets OD-4; internal and `L` sets OD-19; `L_Fr` OD-24). Every layer SHALL have the shape bound that the total interaction count of one proof is below `p`. The sets SHALL freeze with the universal keys. Keygen SHALL refuse a key outside the bounds of its layer and a key whose per-round query vector differs from the frozen one.

#### Scenario: Key outside its bounds
- **WHEN** keygen receives an internal key of height `2^20`, an app key whose query vector differs from 193/88/81/81, or an internal key whose query vector differs from 97/82/81
- **THEN** keygen refuses the key

### Requirement: SQG-003 The parameters of `L`
`L` SHALL use BabyBear with `EF` of degree 4, Midnight Poseidon over Fr for its Merkle tree and transcript, 8 BabyBear elements per Fr element, WHIR with `k_whir` = 4, 3 rounds, a final polynomial of `2^8` and `WhirProximityStrategy::UniqueDecoding` in every round. It SHALL use log blowup 3, queries 98/83/82, folding PoW 0, `mu` PoW 6, query PoW 20, LogUp PoW 18 and one OOD sample per round in the stock WHIR order (OD-19).

#### Scenario: OOD term with one sample
- **WHEN** D3 runs the `L` set at `w_stack` 48
- **THEN** the OOD terms are 108.6276 bits (round 0) and 112.6276 bits (round 1) with list size 1, MEASURED (`UDS:347-348`, where `UDS` is the D3 calculator log `~/.cache/ud-sizing/ud-sweep.log` of `research/sherlock-design-paper-review-20261111/UD-SIZING.md`)

### Requirement: SQG-004 Margin rule
Every term of every layer SHALL be at least 100 bits. Every grindable term (folding, `mu`, query, LogUp) SHALL be at least 101 bits through its PoW constant. The app, leaf-aggregation and internal sets SHALL use query PoW 21 (OD-4, OD-19). The leaf-aggregation set SHALL use LogUp PoW 17.

#### Scenario: App query PoW 20
- **WHEN** the app and leaf-aggregation sets take query PoW 20
- **THEN** their query terms are 100.09 and 100.01 bits, and the sets fail the margin rule

#### Scenario: Leaf-aggregation LogUp PoW
- **WHEN** D3 runs the leaf-aggregation set (log blowup 2, `l_skip` 4, `n_stack` 17, `w_stack` 2,048, height 2^21) at LogUp PoW 0, 16 and 17
- **THEN** the LogUp term is 84.7207, 100.7206 and 101.7206 bits, so 17 is the smallest LogUp PoW that meets the margin rule, and the weakest term stays the query term at 101.0110 bits, MEASURED (`~/.cache/ud-sizing/leaf-logup.log`, test `crates/stark-backend/tests/leaf_logup.rs`)

### Requirement: SQG-005 The stacked width of `L`
The `L` key SHALL have the shape bound `w_stack` at most 48 with `mu` PoW 6 (OD-19; frozen by owner decision 2026-11-12). D10 measured `w_stack` 18 on `L_cached` and 26 without the cache (branch `d10-heights`, commit `3b979bd5c`). The internal set SHALL keep its own bound 512.

#### Scenario: Width at the bound
- **WHEN** D3 runs `L` at `w_stack` 48 with `mu` PoW 6
- **THEN** the weakest term is 101.0730 bits (`mu`), MEASURED (`UDS:351`)

#### Scenario: Width above the bound
- **WHEN** an `L` trace needs `w_stack` above 48
- **THEN** keygen refuses the key

### Requirement: SQG-006 Gate check G1, parameters frozen
G1 SHALL pass only when the sets of OD-4, OD-17, OD-18 (owner decisions 2026-11-12), the internal and `L` sets of OD-19 and the `L_Fr` set of OD-24 option E are fixed in the key-generation code with the explicit per-round query vectors, the global interaction bound, query PoW 21 for the app, leaf-aggregation and internal sets and LogUp PoW 17 for the leaf-aggregation set.

#### Scenario: Set not in the code
- **WHEN** one set or bound is not fixed in the key-generation code
- **THEN** G1 stays open

### Requirement: SQG-007 Gate check G2, the calculator on the frozen sets
G2 SHALL pass only when the stock calculator, after build step 4, prints every term of every layer at its shape bounds at 100 bits or more, and every grindable term at 101 or more, on the frozen query vectors, with `L` at the `w_stack` that D10 measures.

#### Scenario: Calculator part only
- **WHEN** D3 has run the internal set, `L` at `w_stack` 36 and 48 with `mu` PoW 6 and at 16 with `mu` PoW 5, the app and leaf-aggregation sets (the latter at LogUp PoW 17), and `L_Fr` at its measured key of gate E2
- **THEN** the calculator part of G2 is passed (weakest `L` term 101.0730 bits, weakest internal term 101.1643 bits, weakest `L_Fr` term 101.1435 bits, MEASURED) and G2 stays open until the frozen sets exist and the `L_Fr` key of the full interface (E3) has run

### Requirement: SQG-008 Gate check G3, transcript
G3 SHALL pass only when the prover, the stark-backend verifier, the calculator and the verifier of `L` inside `L_Fr` follow the stock one-sample WHIR order of HDM-006 and the transcript of HDM-004 and HDM-005, the T-FS vectors are equal on all four implementations, and the same holds for the `L_Fr` transcript (u = 7) and the wrap's verifier of `L_Fr`.

#### Scenario: T-FS pending
- **WHEN** the T-FS vectors have not run
- **THEN** G3 is open

### Requirement: SQG-009 Gate check G4, Lean
G4 SHALL pass only when the restated theorem is committed with a build log of 0 sorry and the standard axioms only, the concrete `L` key passes `ProvenRegime` by `decide`, and the `L_Fr` set passes a parameter gate under premise A2'.

#### Scenario: Older regime
- **WHEN** the build at `bdb92973a` is offered for G4
- **THEN** G4 stays open, because its `ProvenRegime` takes the list decoding regime and does not check the unique decoding sets

#### Scenario: Unique decoding build
- **WHEN** the build at `6661325d4` of branch `bridge-lean` is offered for G4
- **THEN** the unique decoding part passes (146 theorems, 0 sorry, standard axioms only; the OD-19 example `L` key passes `lKey_ok`; the gates of the `L`, internal, app and leaf-aggregation sets pass), and G4 stays open for the leaf-aggregation entry at LogUp PoW 17 (the build checks it at LogUp PoW 0), the `L_Fr` parameter gate and `decide` on the key that G1 freezes

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
The gate SHALL count Lean as passed only when BR-L2 is proved, with `ProvenRegime` in the unique decoding regime, the frozen per-round query vector, the PoW constants, one OOD sample per round and the `w_stack` bound. Premises A2 (`lastLayerSound`) and A2' (`lfrSound`) SHALL stay named hypotheses.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** the unique decoding `ProvenRegime` and BR-L1 with premises A1 to A7, A2', `vkL_ok` and `vkLFr_ok` build at `6661325d4`, and the leaf-aggregation entry at LogUp PoW 17 and the `L_Fr` parameter gate are open

### Requirement: SQG-013 The MERGE-WORK parameter set
The MERGE-WORK nodes SHALL have their own set `merge_work`. It SHALL have the internal parameters and query vector 97/82/81, and the internal bounds with 5,400 columns (owner ruling N-6 "own bounds", 2026-11-14; the column bound is a T8 AFK pick for owner confirmation at T15).

#### Scenario: D3 of the MERGE-WORK set
- **WHEN** D3 runs `merge_work` at its bounds
- **THEN** every term is at least 100 and every grindable term at least 101 (T8: weakest `shift` 101.1643, MEASURED)
