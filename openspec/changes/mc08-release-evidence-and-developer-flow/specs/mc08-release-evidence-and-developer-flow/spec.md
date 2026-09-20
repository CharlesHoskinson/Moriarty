## ADDED Requirements

Product requirements below govern objective language, proof, authorization and ledger predicates. Maintainer delivery requirements govern this repository's implementation and release evidence only. No maintainer review, named model, Foreman/Pel record or RP03 campaign approval is a prerequisite for an external developer to compile, prove or deploy a supported Moriarty program. These corrected requirements specify intended behavior; this document does not establish that the behavior is implemented.

### Requirement: Internal evidence-based release completion
Maintainer release completion SHALL require applicable product evidence, resolved blocking findings, and the independent audits selected by the current delivery assignment. Internal audit or campaign requirements SHALL NOT become developer deployment prerequisites.

#### Scenario: Complete supported language
- **WHEN** a fresh checkout runs the complete program verification and both audits accept the exact candidate
- **THEN** the dossier records reproduced parser, semantic, proof, compiler, conformance, acceptance, recovery, and composition predicates.

#### Scenario: False completion
- **WHEN** closure relies on checkboxes, missing reports, stale digests, abstention, exhausted resources, or mock evidence
- **THEN** the program remains incomplete.

### Requirement: Usable developer flow
Developers SHALL author, simulate, inspect, sign, prove, submit, and diagnose their own supported programs. Loan and swap operations are qualification examples, not an exhaustive deployment catalog.

#### Scenario: End-to-end workflow
- **WHEN** a developer authors, simulates, inspects, signs, proves, and submits the representative loan and swap
- **THEN** the interface shows exact signed authority, verified claim predicates, finalized effects, and durable receipt status.

#### Scenario: Misleading proof status
- **WHEN** a profile is unsupported, required witness data is unavailable, or mandatory evidence rejects
- **THEN** the interface reports unavailable or rejected and cannot show proved or settled success.

### Requirement: Scope and release claims
The final dossier SHALL reconcile all seven requested workstreams and the legacy G01-G24 requirements.

#### Scenario: Explicit scope
- **WHEN** the final dossier reconciles the requested checklist with legacy G01-G24 requirements
- **THEN** each additional pilot, baseline, license, leakage, and assurance requirement has evidence or an explicit remaining release blocker.

#### Scenario: Scope laundering
- **WHEN** an unperformed legacy release requirement is marked complete solely because the seven-item checklist passed
- **THEN** the scope reconciliation gate rejects that unsupported release claim.

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

### Requirement: Developer-visible semantics and assumptions
The developer flow SHALL distinguish signed intent, selected plan, simulation, proof verification, finalized effects and outstanding obligations. It SHALL disclose assumed oracle/custody/finality facts and unresolved availability, without implying finite evaluation proves liveness.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).

#### Scenario: Revoked verifier or inactive specification
- **WHEN** evidence verifies cryptographically but its key/specification is revoked or outside the activation window bound by the deployed contract, protocol rules and applicable participant authorization
- **THEN** acceptance rejects it and migration cannot restore a consumed predecessor or reset lifecycle authority.

#### Scenario: Oversized verification input
- **WHEN** claim count, dependency bounds, encoded evidence/sidecar bytes or declared total verification work exceed the registered budget
- **THEN** bounded admission rejects before expensive verification or unbounded allocation, without applying effects.

### Requirement: Independent developer workflow
The public toolchain SHALL support developer-owned compilation, proving and deployment without a Moriarty administrator, project reviewer or project-operated service. Actual correctness proofs, authenticated participant authority, finite bounds and target ledger validity remain required.

#### Scenario: Clean external installation
- **WHEN** a developer uses a clean installation without `.moriarty-dev`, Foreman/Pel state, campaign records or reviewer receipts to deploy a newly authored supported program
- **THEN** the workflow permits objective proof and ledger validation using the developer's own resources and credentials without project approval.

#### Scenario: Missing mandatory proof
- **WHEN** that developer submits a transaction without valid required correctness evidence
- **THEN** the ledger acceptance path rejects it; permissionless access does not waive proof obligations.
