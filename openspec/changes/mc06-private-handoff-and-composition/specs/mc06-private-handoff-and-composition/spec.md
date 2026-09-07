## ADDED Requirements

### Requirement: Independent successor witness
A successor SHALL prove using only its permitted private inputs and the declared handoff package.

#### Scenario: Alice to Bob
- **WHEN** Bob receives only his permitted private inputs and the declared handoff artifacts
- **THEN** isolated processes and secret stores produce a valid successor proof without access to Alice secrets.

#### Scenario: Unavailable witness
- **WHEN** a required handoff artifact is missing
- **THEN** the successor reports unavailable and cannot fabricate evidence or disclose Alice secrets to force acceptance.

### Requirement: Bounded history composition
Split and join SHALL preserve obligations, authority limits, output uniqueness, and the original global lifecycle measure.

#### Scenario: Compatible composition
- **WHEN** two independently proved compatible branches undergo the declared split and join
- **THEN** their new proofs discharge dependencies and preserve obligations, residual authority, uniqueness, and global lifecycle bounds.

#### Scenario: Composition attack
- **WHEN** composition duplicates predecessors, reuses outputs, mixes incompatible policies, exceeds fan-in, or resets bounds
- **THEN** proof and acceptance gates reject the invalid composition.

### Requirement: Ledger and confidentiality boundaries
The evidence SHALL distinguish history compliance, ledger uniqueness, oracle truth, and confidentiality assumptions.

#### Scenario: Competing valid branches
- **WHEN** two otherwise valid branches conflict on ledger consumption
- **THEN** both may have valid proofs, but only permitted unique consumption settles.

#### Scenario: False privacy claim
- **WHEN** a demonstration relies on a public proof alone or shared access to both parties secret directories
- **THEN** the private-handoff evidence gate rejects the demonstration.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable 5.1 or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Extended native predicate evidence
The package SHALL implement, review, prove, and independently verify its allocated native relation before proof-dependent acceptance.

#### Scenario: Reviewed extension executes
- **WHEN** both auditors approve the implemented relation and contract, and the allocated campaign passes
- **THEN** retained proofs and separate-process verification establish only the extended predicate and requalified dependent correspondence.

#### Scenario: Fixed proof substituted for extension
- **WHEN** the package supplies only an earlier fixed-loan proof, exceeds its campaign allocation, or lacks a required extension proof
- **THEN** package acceptance remains blocked and no earlier proof or counter can substitute.

### Requirement: Explicit extension and acceptance lineage
The package SHALL extend owned upstream predicates and requalify them against the single versioned acceptance lineage.

#### Scenario: Changed semantic domain
- **WHEN** a new profile changes language, proof, correspondence, or acceptance behavior
- **THEN** the package extends and reproves affected domains, reruns their checks, and obtains both updated audits.

#### Scenario: Unrelated validator or stale theorem
- **WHEN** evidence uses an independent bypass contract or an older theorem outside its supported domain
- **THEN** acceptance fails and affected packages remain pending requalification.

### Requirement: Composition operators and witness ownership
Composition SHALL state whether it is sequential, disjoint, shared-state atomic or asynchronous. Private handoff SHALL enumerate required backend artifacts and preserve liabilities, residual authority and a conserved global work budget under genuine predecessor proof verification.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).
