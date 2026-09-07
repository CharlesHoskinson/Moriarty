## ADDED Requirements

### Requirement: Mandatory acceptance
The real acceptance path SHALL require valid evidence for all four claim types under a fixed trusted policy.

#### Scenario: Certified action
- **WHEN** a loan or swap action carries all mandatory evidence under the fixed trusted policy
- **THEN** actual acceptance checks contract, intent, transition, history, and ledger-consumption predicates before applying effects.

#### Scenario: Downgrade attack
- **WHEN** an action omits claims, strips mandatory roots, selects arbitrary verifiers, or supplies stale certificates, false guards, or unsatisfied dependencies
- **THEN** actual acceptance rejects it before applying effects.

### Requirement: Intent refinement and complete effects
The proof relation SHALL connect the concrete plan and complete effects to signed bounded authority and net goals.

#### Scenario: Authorized route choice
- **WHEN** different permitted plans refine the same signed outcome intent
- **THEN** each accepted route satisfies the original gross authority limits, net goals, and complete-effect relation.

#### Scenario: Ledger-valid intent-invalid action
- **WHEN** a ledger-valid action has wrong recipients, refunds hiding excess gross debit, fees violating net goals, or undeclared approvals
- **THEN** the mandatory refinement relation rejects it in actual acceptance.

### Requirement: No circular or simulated evidence
The certificate and proof construction SHALL use the non-circular commitment order and actual checked evidence.

#### Scenario: Bound claims
- **WHEN** the system builds and consumes the non-circular signed claim envelope
- **THEN** program, policy, predecessors, observations, resulting state, and effects bind identically across signature, proof, and ledger.

#### Scenario: Fake compliance
- **WHEN** an action supplies hash-linked receipts, mock proofs, signatures alone, or unchecked certificate labels
- **THEN** actual acceptance rejects the missing correctness evidence.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Opus or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
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
