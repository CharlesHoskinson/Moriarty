## ADDED Requirements

Product requirements below govern objective language, proof, authorization and ledger predicates. Maintainer delivery requirements govern this repository's implementation and release evidence only. No maintainer review, named model, Foreman/Pel record or RP03 campaign approval is a prerequisite for an external developer to compile, prove or deploy a supported Moriarty program. These corrected requirements specify intended behavior; this document does not establish that the behavior is implemented.

### Requirement: Actual verifier compatibility
The adapter SHALL demonstrate that the selected Midnight acceptance path verifies the exact native proof, VK, and public-input relation.

#### Scenario: Compatible verifier
- **WHEN** a real native positive proof and its mutants reach the selected target acceptance path
- **THEN** the exact native proof, VK, and public-input relation verify there, and each invalid mutant rejects there.

#### Scenario: Missing interface
- **WHEN** the target lacks an interface that checks the required native relation
- **THEN** the adapter remains interface-blocked; host assertions and mocked verification cannot substitute.

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

### Requirement: Internal maintainer audit and provenance
For internal delivery, the package SHALL bind release evidence to exact sources, commands, environment, outputs, and the independently selected audit identities. Named reviewer requirements apply only to maintainer delivery and SHALL NOT enter program or transaction validity.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** a reviewer required by the current maintainer delivery assignment is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the internal delivery record remains pending audit; developer compilation, proving and deployment SHALL NOT reject solely for that missing review.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Extended native predicate evidence
The package SHALL implement, prove and independently verify its allocated native relation before claiming proof-dependent product behavior. Source review and campaign allocation are separate internal maintainer delivery controls.

#### Scenario: Reviewed extension executes
- **WHEN** both auditors approve the implemented relation and contract, and the allocated campaign passes
- **THEN** retained proofs and separate-process verification establish only the extended predicate and requalified dependent correspondence.

#### Scenario: Fixed proof substituted for extension
- **WHEN** the package supplies only an earlier fixed-loan proof, exceeds its campaign allocation, or lacks a required extension proof
- **THEN** package acceptance remains blocked and no earlier proof or counter can substitute.

### Requirement: Early complete verifier feasibility
For project-operated campaigns, RP02 source and component preparation SHALL run before dependent native investment. Product acceptance SHALL check the complete native final decision, canonical exported artifacts and exact ledger effects under pinned deployment versions; an unconstrained host verdict SHALL reject.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).

### Requirement: Permissionless correspondence validation
The supported correspondence and verification rules SHALL apply to developer-authored programs without a project deployment allowlist. Contract, protocol and participant commitments SHALL bind the program, specification and verifier relation; a prover-selected unrelated verifier SHALL NOT satisfy those commitments.

#### Scenario: New supported program
- **WHEN** an external developer supplies a new program within the supported semantic domain with valid required proofs and authorization
- **THEN** validation applies the same correspondence and ledger predicates without requiring a project campaign, reviewer receipt or prior registration in the demonstration catalog.

#### Scenario: Incorrect proof binding
- **WHEN** a proposed deployment or transaction substitutes a verifier or statement that does not establish the bound program and required predicates
- **THEN** validation rejects it regardless of any maintainer approval.
