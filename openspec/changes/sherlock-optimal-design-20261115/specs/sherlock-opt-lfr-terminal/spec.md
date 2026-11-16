## ADDED Requirements

### Requirement: LFT-001 Unique decoding in every STARK layer
LFT-001: The stark-backend parameter code SHALL use WHIR in unique decoding in every STARK layer.

Source: FD-10 (A-02, 6 of 6). Hard rule: 100-bit security in unique-decoding form. List decoding is OQ-1 (`OPEN (owner)`, design.md).

#### Scenario: List-decoding set offered
- **WHEN** a parameter set with a list-decoding proximity strategy is loaded
- **THEN** keygen refuses it

### Requirement: LFT-002 Lower parameter sets unchanged
LFT-002: The stark-backend parameter code SHALL keep the APP, LEAF_AGG, INTERNAL and `L` parameter sets as in `sherlock-lfr/src/frozen.rs`.

Source: FD-10. Label: the values of FD section 1.2 are MEASURED reads of `frozen.rs` (APP 1/4/20/2,048, `k_whir` 4, queries 193/88/81/81; LEAF_AGG 2/4/17/2,048; INTERNAL and MERGE-WORK 3/2/17/512; `L` 3/2/18/48).

#### Scenario: Set digest
- **WHEN** the final keys are cut
- **THEN** the parameter digests of APP, LEAF_AGG, INTERNAL and `L` equal those of the T25 prototype keys

### Requirement: LFT-003 L_Fr folding and stacking
LFT-003: The stark-backend parameter code SHALL set `L_Fr` to `k_whir` 3 (4 WHIR rounds) and `n_stack` 19 with query PoW 20.

Source: FD-11 (A-03, 6 of 6; query PoW 24 rejected).

#### Scenario: L_Fr parameters
- **WHEN** the `L_Fr` key of the final rekey is read
- **THEN** it records `k_whir` 3, `n_stack` 19 and query PoW 20

### Requirement: LFT-004 k_whir is frozen
LFT-004: The `FrozenSet` type SHALL carry `k_whir` as an explicit frozen parameter.

Source: FD-11.

#### Scenario: Mismatched k_whir
- **WHEN** a key is loaded whose `k_whir` differs from its `FrozenSet`
- **THEN** the load is refused

### Requirement: LFT-005 L_Fr blowup 3 by default
LFT-005: The stark-backend parameter code SHALL set the `L_Fr` log blowup to 3.

Source: FD-12 (C-07 (b), 4 of 6). Label: control vector 98/85/82/82 at 101.0794 bits, MEASURED (D3, T10 fixture). Dissent: fable-1 and fable-2 hold blowup 2 on the MEASURED `L_Fr` 37.9 to 25.4 s and key 3.07 to 1.70 GB (older stack).

#### Scenario: E-PCS not passed
- **WHEN** E-PCS has no pass for blowup 2
- **THEN** the final `L_Fr` key has log blowup 3

### Requirement: LFT-006 L_Fr blowup 2 after E-PCS
LFT-006: Where E-PCS has passed for blowup 2, the stark-backend parameter code SHALL set the `L_Fr` log blowup to 2.

Source: FD-12, C-07. Pass bar (E-PCS): wrap at least 100,000 rows under the gate after the 60,000-row reserve (ESTIMATE reserve); every D3 term at 100 bits or more and grindable terms at 101 or more at the measured, vk and bounds shapes; 2 s or more saved on the full resident path; memory no worse.

#### Scenario: One manifest
- **WHEN** blowup 2 is adopted
- **THEN** the manifest holds one `L_Fr` set and no hardware-selected parameter

### Requirement: LFT-007 D3-minimum query vector
LFT-007: The stark-backend parameter code SHALL use for `L_Fr` the D3 search-minimum query vector with every term at 100 bits or more and every grindable term at 101 bits or more at the measured, vk and bounds shapes.

Source: FD-13 (C-08 (a), 6 of 6). Label: at blowup 2 the local minimum is 120/89/83/82, 374 queries, DERIVED (K-1); it is not a D3 run.

#### Scenario: Extra reserve
- **WHEN** a vector above the D3 minimum is proposed without a quantified deficit
- **THEN** the D3 minimum is kept

### Requirement: LFT-008 L_Fr w_stack from the measured layout
LFT-008: The stark-backend parameter code SHALL set the `L_Fr` `w_stack` bound from the measured stacked layout of the complete `L_Fr` with a named consumer for every spare column.

Source: FD-14 (C-09 (b), 5 of 6). Label: at least 24 columns, DERIVED (K-2: 17 main + 7 preprocessed at stacked height 2^21). Dissent: fable-1 holds 32.

#### Scenario: Complete layout
- **WHEN** the collection terminal change and the GP-3 rows are in `L_Fr`
- **THEN** the bound equals the measured width plus the named reserve and D3 runs at that bound

#### Scenario: Unnamed spare column
- **WHEN** a spare column has no named consumer
- **THEN** it is removed from the bound
