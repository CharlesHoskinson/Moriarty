## ADDED Requirements

### Requirement: GTR-001 Leaf node and L on the GPU
GTR-001: The resident service SHALL prove the leaf layer and the node layers and `L` on the GPU.

Source: FD-43 (A-07). Label: removes about 82 s, ESTIMATE; merged stack 41 to 45 s warm, DERIVED from MEASURED parts (25.67 s chain and 12.89 to 15.3 s wrap, two processes, older keys).

#### Scenario: Warm call bar
- **WHEN** E-S1 times 10 warm resident R_SL calls on huginn on the T25 keys
- **THEN** the median is at or under 50 s (MEASURED bar; 45 s once E-GAP names the `L_Fr` 21.8 against 17.55 s cause)

### Requirement: GTR-002 Order of the later GPU ports
GTR-002: When the leaf layer and the node layers and `L` run on the GPU, the GPU port plan SHALL port query grind first and then parallel trace fill and then `L_Fr` sumcheck and GKR preparation.

Source: FD-43 (A-07). Label: 24 to 30 s warm after these ports, ESTIMATE, not a requirement.

#### Scenario: Port order
- **WHEN** the next GPU kernel is chosen
- **THEN** it is the first unported item of that order

### Requirement: GTR-003 Wrap CPU phases after E-WRAPSPLIT
GTR-003: Where E-WRAPSPLIT has passed, the GPU port plan SHALL port the movable CPU phases of the wrap.

Source: FD-43. Pass bar: 6 s or more of movable CPU work; under 4 s drops the item. Label: 10.9 s of the 12.89 s wrap is unattributed, MEASURED; the 18 s ladder row is ESTIMATE and conditional.

#### Scenario: Under 4 s movable
- **WHEN** E-WRAPSPLIT finds under 4 s of movable CPU work
- **THEN** no wrap CPU phase is ported

### Requirement: GTR-004 GPU proofs checked by the stock verifier
GTR-004: If the stock CPU verifier refuses a GPU proof, then the resident service SHALL discard that proof and pass nothing to the next layer.

Source: FD-44 (A-07).

#### Scenario: Bad GPU proof
- **WHEN** a GPU proof fails the stock CPU verifier
- **THEN** the service does not pass it to the next layer

### Requirement: GTR-005 G3 transcript equality
GTR-005: The G3 test SHALL show equal transcript vectors on the CPU prover and the GPU prover and the verifiers with the grind compared on a fixed witness.

Source: FD-44. Gate: E-G3 at the new `L_Fr` set.

#### Scenario: Grind on a fixed witness
- **WHEN** E-G3 runs at the new `L_Fr` set
- **THEN** all legs are equal

### Requirement: GTR-006 Starting layout for a collection
GTR-006: While a multi-card box proves a collection, the resident service fleet SHALL use one tail card for `L` and `L_Fr` and the wrap with the other cards on app and leaf and collector and node proofs.

Source: FD-46 (C-16 (a), chair decision, upheld; a starting schedule, not an invariant). Label: one pipeline peaks at 24.75 GiB device memory, DERIVED. Dissent: astra-1, astra-2, astra-3: no layout has a measurement.

#### Scenario: Checkpoint seal
- **WHEN** the tail card is free and the interval has passed
- **THEN** the fleet seals a checkpoint

### Requirement: GTR-007 Layout replaced by measurement
GTR-007: If E-S5 or E-S6 shows a mixed layout winning by 10 percent or the tail card idle more than half of a checkpoint at N = 1,000, then the resident service fleet SHALL use the measured layout in place of GTR-006.

Source: FD-46, E-S6 pass bar (ESTIMATE bars set by the debate).

#### Scenario: Mixed layout wins
- **WHEN** E-S6 shows a mixed layout 10 percent faster
- **THEN** the mixed layout becomes the default

### Requirement: GTR-008 Lone calls on every card
GTR-008: While a multi-card box proves lone calls, the resident service fleet SHALL run a full pipeline on every card.

Source: FD-46. Bar (C-13 (a), 6 of 6): four cards fail under 3.2 times one card on the stated host.

#### Scenario: Four-card scaling
- **WHEN** E-S6 runs lone calls on the 4x RTX PRO 6000 box
- **THEN** the rate is at least 3.2 times one card
