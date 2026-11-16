## ADDED Requirements

### Requirement: FQL-001 D3 on every changed set
FQL-001: When a parameter set changes, the D3 calculator SHALL print every term at 100 bits or more and every grindable term at 101 bits or more at the measured, vk and bounds shapes.

Source: FD-70 (A-17). Hard rule: 100-bit security, unique decoding. Status: ARCHITECTURE 1.9 "not qualified".

#### Scenario: Term under the bar
- **WHEN** D3 prints a grindable term at 100.9 bits
- **THEN** the set is not admitted

### Requirement: FQL-002 G1 to G5 on the final keys
FQL-002: When the final keys are cut, the qualification runner SHALL rerun G1 to G4 and the theorem map G5 on them.

Source: FD-70.

#### Scenario: Final keys
- **WHEN** the final rekey completes
- **THEN** G1 to G5 have records on the final key digests

### Requirement: FQL-003 Composition claim
FQL-003: The qualification record SHALL state the recursive-composition claim with its adversarial budget and no invented depth penalty.

Source: FD-71 (C-21 (a), 6 of 6).

#### Scenario: Record content
- **WHEN** the qualification record is read
- **THEN** it names the composition claim and its budget

### Requirement: FQL-004 Exact parameter certificate
FQL-004: The parameter certificate SHALL use exact arithmetic or certified bounds and no field order held in floating point.

Source: FD-72 (C-21).

#### Scenario: Floating-point field order
- **WHEN** a certificate computes with the field order in floating point
- **THEN** it is not accepted as the certificate

### Requirement: FQL-005 Collection relation accounting
FQL-005: The collection relation SHALL have its own quantified soundness accounting with a Quint model and a Carbon lane and counterexamples on real code.

Source: FD-73 (A-28, C-21).

#### Scenario: Forgery set
- **WHEN** the counterexamples run on real code
- **THEN** each of wrong root with a valid SET, dropped member, member at two slots, run offered as foreign, sub-root passed as a leaf, list different from `deps_d`, wrong collector `prog_d`, root altered at lane 29 or 43 and wrong-entry reuse has a verdict in COUNTEREXAMPLES.md

### Requirement: FQL-006 No production keys before complete relations
FQL-006: If the interpreter kits or the GP-3 verifier or the collection relation is incomplete, then the rekey runner SHALL refuse to cut a production key set.

Source: FD-74 (C-22 (c), 6 of 6). Prototype rekeys are allowed.

#### Scenario: Prototype rekey
- **WHEN** a rekey is run for E-PCS step 2 before the kits are complete
- **THEN** its keys are marked prototype and never enter a production manifest
