## ADDED Requirements

### Requirement: Actual verifier compatibility
The adapter SHALL demonstrate that the selected Midnight acceptance path verifies the exact native proof, VK, and public-input relation.

#### Scenario: Compatible verifier
- **WHEN** a real native positive proof and its mutants reach the selected target acceptance path
- **THEN** the exact native proof, VK, and public-input relation verify there, and each invalid mutant rejects there.

#### Scenario: Missing interface
- **WHEN** the target lacks an interface that checks the required native relation
- **THEN** the adapter remains interface-blocked; host assertions and mocked verification cannot substitute.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Declared ledger verification seam": the core interface is ledger `well_formed` against the operation key in contract state, so the core is not interface-blocked; host assertions and mocked verification still cannot substitute.

### Requirement: Named compiler correspondence
The package SHALL state and mechanically check source-to-Core-to-proof-to-ledger correspondence for its supported finite domain.

#### Scenario: Positive realization
- **WHEN** loan and swap programs compile within the supported finite domain
- **THEN** mechanically checked correspondence and positive realized traces cover the complete effect projection.

#### Scenario: Unsound projection
- **WHEN** an execution adds transfers, approvals, hidden calls, changed rounding, reordered events, or omitted writes
- **THEN** the correspondence checker rejects it and the package cannot close.

### Requirement: Durable one-time consumption
The acceptance path SHALL enforce authorization and predecessor consumption across restart, concurrency, and competing branches.

#### Scenario: Crash recovery
- **WHEN** a client restarts after finalized acceptance or crashes at each consumption boundary
- **THEN** durable ledger identities prevent repeated effects and recovery reconciles finality without issuing duplicate authority.

#### Scenario: Duplicate authority
- **WHEN** proposals share a consumed nonce or predecessor, including changed-intent-hash and concurrent valid-branch cases
- **THEN** at most one conflicting proposal settles; restart does not restore consumed authority.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Head read-then-write discipline": unique consumption is enforced by reading each head before writing it, checked over generated ZKIR and by E1.

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

### Requirement: Early complete verifier feasibility
RP02 source and component preparation SHALL run before dependent native investment. Acceptance SHALL check the complete native final decision, canonical exported artifacts and exact ledger effects under pinned deployment versions; an unconstrained host verdict SHALL reject.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).
- **Amended by** `pcd-ledger-anchored-acceptance` requirements "Declared ledger verification seam" and "Bounded native certificates": early feasibility is the Stage 0 seam for the core and E3 for certificates; no in-circuit pairing is required.
