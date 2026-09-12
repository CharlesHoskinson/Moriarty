## ADDED Requirements

### Requirement: Mandatory acceptance
The real acceptance path SHALL require valid evidence for all four claim types under a fixed trusted policy.

#### Scenario: Certified action
- **WHEN** a loan or swap action carries all mandatory evidence under the fixed trusted policy
- **THEN** actual acceptance checks contract, intent, transition, history, and ledger-consumption predicates before applying effects.

#### Scenario: Downgrade attack
- **WHEN** an action omits claims, strips mandatory roots, selects arbitrary verifiers, or supplies stale certificates, false guards, or unsatisfied dependencies
- **THEN** actual acceptance rejects it before applying effects.
- **Amended by** `pcd-ledger-anchored-acceptance` requirements "Immutable authority deployment audit" and "Claim discharge map" and "Forward-declared migration replaces in-place revocation": claims are compiled into immutable operation keys, so a stripped claim or arbitrary verifier needs a different key and fails the deploy audit.

### Requirement: Intent refinement and complete effects
The proof relation SHALL connect the concrete plan and complete effects to signed bounded authority and net goals.

#### Scenario: Authorized route choice
- **WHEN** different permitted plans refine the same signed outcome intent
- **THEN** each accepted route satisfies the original gross authority limits, net goals, and complete-effect relation.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Intent digest v2 and program digest": route choice uses outcome mode, which binds a program-digest allowlist and a consumed nonce.

#### Scenario: Ledger-valid intent-invalid action
- **WHEN** a ledger-valid action has wrong recipients, refunds hiding excess gross debit, fees violating net goals, or undeclared approvals
- **THEN** the mandatory refinement relation rejects it in actual acceptance.

### Requirement: No circular or simulated evidence
The certificate and proof construction SHALL use the non-circular commitment order and actual checked evidence.

#### Scenario: Bound outcome claims
- **WHEN** a participant signs an outcome intent before a concrete execution is selected
- **THEN** the signature binds the canonical constraints, program/profile/policy and required claims; the proof statement binds that authorization digest, concrete predecessors, observations, resulting state and complete effects, proves refinement, and matches the actual ledger projection.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Intent digest v2 and program digest": the signature binds digest v2 fields, and the head read replaces predecessor lists.

#### Scenario: Bound exact-plan claims
- **WHEN** a participant signs an exact plan
- **THEN** acceptance additionally checks the signed canonical execution-body commitment against the concrete execution; the commitment excludes self-referential proof/signature bytes.

#### Scenario: Fake compliance
- **WHEN** an action supplies hash-linked receipts, mock proofs, signatures alone, or unchecked certificate labels
- **THEN** actual acceptance rejects the missing correctness evidence.

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

### Requirement: Authority covers liabilities and all protected effects
Intent refinement SHALL constrain newly created or modified nominal liabilities separately from token debit budgets. The versioned authoring and signing model SHALL distinguish outcome intent from concrete plan and bind hard requirements without treating soft ranking as global optimality.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).

#### Scenario: Revoked verifier or inactive specification
- **WHEN** evidence verifies cryptographically but its key/specification is revoked or outside the policy activation window
- **THEN** acceptance rejects it and migration cannot restore a consumed predecessor or reset lifecycle authority.
- **Amended by** `pcd-ledger-anchored-acceptance` requirement "Forward-declared migration replaces in-place revocation": keys cannot be revoked in place; a defective program is halted by `Pause` and replaced by migration, which still cannot restore consumed state or reset lifecycle authority.

#### Scenario: Oversized verification input
- **WHEN** claim count, dependency bounds, encoded evidence/sidecar bytes or declared total verification work exceed the registered budget
- **THEN** bounded admission rejects before expensive verification or unbounded allocation, without applying effects.
- **Amended by** `pcd-ledger-anchored-acceptance` requirements "Intent digest v2 and program digest" and "Measured bounds frozen into the program digest": the registered budget is the Π_P bound set, checked before proving; on-chain sidecar bytes are zero.
