## ADDED Requirements

### Requirement: DG-001 No BLS12-381 circuit above k = 21
No BLS12-381 circuit of the co-processor SHALL be keyed above k = 21 (CPR-001).

#### Scenario: Keygen at k = 22
- **WHEN** a BLS12-381 key at k = 22 is requested
- **THEN** keygen refuses it

### Requirement: DG-002 The wrap row gate
The wrap SHALL fit 85 percent of the `2^k - 7` usable rows at the smallest k of 19, 20 and 21 at which the measured G4 rows fit. Every gate and lookup SHALL have degree at most 5. The wrap SHALL use one fixed 2^16-row range table. The k of the key SHALL be fixed by the measured rows of M7 (D15).

#### Scenario: Rows over the gate at k = 21
- **WHEN** M7 measures wrap rows above 85 percent of 2,097,145
- **THEN** the wrap key is not frozen, and a fallback trigger of LW-014 is recorded

#### Scenario: Rows before M7
- **WHEN** a wrap row figure is reported before M7
- **THEN** it is ESTIMATE, such as 434,692 rows mid for G4 on the keyed `L` structure (`FD/ARCHITECTURE.md` kit L AND WRAP, wrap-row table; `PR` section 3.5)

### Requirement: DG-003 The wrap verifier key fits the ledger
The wrap verifier key SHALL be at most 50,000 bytes (MID-003).

#### Scenario: Key bytes measured
- **WHEN** the wrap key is generated at M7
- **THEN** its byte length is recorded against 50,000, and until then it is PENDING

### Requirement: DG-004 STARK shards have fixed shapes inside the bounds of their layer
Every STARK shard SHALL have a fixed shape from the plan menu. Keygen SHALL refuse a key outside the shape bounds of its layer (SQG-002).

#### Scenario: Leaf over the app bounds
- **WHEN** a leaf AIR has more than 5,000 constraints or 1,000 interactions
- **THEN** keygen refuses the app key

### Requirement: DG-005 The height and stacked width of `L`
`L` SHALL have a height of at most `2^20`. Its stacked width SHALL be at most 48 with `mu` PoW 22 (OD-3). A tighter bound from D10 SHALL rerun D3 before the `L` key freezes (SQG-005).

#### Scenario: Height above the bound
- **WHEN** `L` tracegen gives a height of `2^21`
- **THEN** the `L` key is refused

### Requirement: DG-006 STARK cost is seconds per proved step
STARK leaves and nodes SHALL have no fixed row gate. Their cost SHALL be recorded in seconds per proved step on the GPU, with the run record.

#### Scenario: Leaf cost record
- **WHEN** a leaf shape is measured
- **THEN** the record gives seconds per proved step and the host load, such as the W1 leaf at `2^20` in 5.829 s on the GPU (MEASURED, `PR` section 2)

### Requirement: DG-007 The halo2 row gates are history
The halo2 row gates `k_leaf <= 16` at S = 16, node at most 950,000 rows and decider at most 1,000,000 rows SHALL NOT be gates of the co-processor. Row figures of the halo2 prototypes in `crates/mori-chip` and `crates/mori-recursion` SHALL be read as prototype figures only.

#### Scenario: Prototype rows over the old gate
- **WHEN** a halo2 prototype leaf measures more than `2^16` rows
- **THEN** no gate fails, and the figure is recorded as a prototype figure

### Requirement: DG-008 Constraint tests name their checker
A constraint test of a STARK AIR SHALL run the stark-backend debug constraint checker. A constraint test of the wrap circuit or of a halo2 prototype SHALL run MockProver. Every named mutant SHALL be refused at the constraint, lookup or permutation boundary. Every named mutant SHALL have a weakened control that accepts the same witness.

#### Scenario: Mutant refused by a panic
- **WHEN** a mutant witness makes the prover panic before the checker runs
- **THEN** the mutant earns no rejection credit

### Requirement: DG-009 Node height edge and real-proof checks
At the constraint boundary the node SHALL accept a height of `2^64 - 1` and refuse `2^64`. Real proofs SHALL pass the independent verifier of SRC-010 and `validate_call` with the five mutations of SRC-020 refused.

#### Scenario: Height edge
- **WHEN** a node child has `h = 2^64`
- **THEN** the constraint checker refuses the node (mutant `node_height_wrap`), and `pos_node_height_2p64_minus_1` passes

### Requirement: DG-010 A failed gate never narrows a GP target
A failed derived gate SHALL leave its GP target open. It SHALL go to the owner. It SHALL NOT narrow GP-001 to GP-004 or GP-ZK (CPR-005).

#### Scenario: Wrap over every k
- **WHEN** the wrap fits no k of 19, 20 and 21
- **THEN** the targets that need the wrap stay open, and the owner decides

### Requirement: DG-011 Lean obligations of the derived gates
The design lists no Lean obligation for the derived gates. Each gate SHALL be met by a measurement or a keygen check with its record.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** it reads "no Lean obligation in the design", and each gate names its measurement
