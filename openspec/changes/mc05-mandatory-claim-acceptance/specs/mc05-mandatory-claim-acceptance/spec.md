## ADDED Requirements

Product requirements below govern objective language, proof, authorization and ledger predicates. Maintainer delivery requirements govern this repository's implementation and release evidence only. No maintainer review, named model, Foreman/Pel record or RP03 campaign approval is a prerequisite for an external developer to compile, prove or deploy a supported Moriarty program. These corrected requirements specify intended behavior; this document does not establish that the behavior is implemented.

### Requirement: Mandatory acceptance
The real acceptance path SHALL require valid evidence for all four claim types under the policy bound by the deployed contract, protocol rules and applicable participant authorization. The policy SHALL NOT require project approval of the developer or program.

#### Scenario: Certified action
- **WHEN** an action of any supported program carries all mandatory evidence under that bound policy
- **THEN** actual acceptance checks contract, intent, transition, history, and ledger-consumption predicates before applying effects.

#### Scenario: Downgrade attack
- **WHEN** an action omits claims, strips mandatory roots, selects arbitrary verifiers, or supplies stale certificates, false guards, or unsatisfied dependencies
- **THEN** actual acceptance rejects it before applying effects.

### Requirement: Intent refinement and complete effects
The proof relation SHALL connect the concrete plan and complete effects to signed bounded authority. Successful fulfillment SHALL satisfy signed net goals. Every accepted partial or failed outcome SHALL satisfy the separately signed failure policy, including phase-specific authority, nonce consumption, retained effects, liabilities and fee bounds; receipts SHALL NOT label such an outcome successful fulfillment.

#### Scenario: Authorized route choice
- **WHEN** different permitted plans refine the same signed outcome intent
- **THEN** each successfully fulfilled route satisfies the original gross authority limits, net goals, and complete-effect relation; any accepted partial or failed outcome satisfies its signed failure policy and all applicable authority and complete-effect bounds.

#### Scenario: Ledger-valid intent-invalid action
- **WHEN** a ledger-valid action has wrong recipients, refunds hiding excess gross debit, fees violating the applicable signed success or failure bounds, or undeclared approvals
- **THEN** the mandatory refinement relation rejects it in actual acceptance.

### Requirement: No circular or simulated evidence
The certificate and proof construction SHALL use the non-circular commitment order and actual checked evidence.

#### Scenario: Bound outcome claims
- **WHEN** a participant signs an outcome intent before a concrete execution is selected
- **THEN** the signature binds the canonical constraints, program/profile/policy and required claims; the proof statement binds that authorization digest, concrete predecessors, observations, resulting state and complete effects, proves refinement, and matches the actual ledger projection.

#### Scenario: Bound exact-plan claims
- **WHEN** a participant signs an exact plan
- **THEN** acceptance additionally checks the signed canonical execution-body commitment against the concrete execution; the commitment excludes self-referential proof/signature bytes.

#### Scenario: Fake compliance
- **WHEN** an action supplies hash-linked receipts, mock proofs, signatures alone, or unchecked certificate labels
- **THEN** actual acceptance rejects the missing correctness evidence.

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
- **WHEN** evidence verifies cryptographically but its key/specification is revoked or outside the activation window bound by the deployed contract, protocol rules and applicable participant authorization
- **THEN** acceptance rejects it and migration cannot restore a consumed predecessor or reset lifecycle authority.

#### Scenario: Oversized verification input
- **WHEN** claim count, dependency bounds, encoded evidence/sidecar bytes or declared total verification work exceed the registered budget
- **THEN** bounded admission rejects before expensive verification or unbounded allocation, without applying effects.

### Requirement: Administrative metadata is not correctness evidence
Program and transaction validation SHALL neither require internal maintainer workflow records nor accept them as substitutes for mandatory correctness evidence.

#### Scenario: No project workflow records
- **WHEN** a supported program and its transaction satisfy all required proof, authorization, financial and ledger predicates without Foreman/Pel records, RP03 admission or reviewer receipts
- **THEN** validation does not reject solely because those internal records are absent.

#### Scenario: Approved but unproved transaction
- **WHEN** a transaction has internal maintainer approval but lacks a valid mandatory contract, intent, transition or history proof
- **THEN** actual acceptance rejects it before applying effects.
