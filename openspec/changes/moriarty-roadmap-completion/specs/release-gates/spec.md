# XML release-gate acceptance

## ADDED Requirements

Every gate remains open in this specification handoff.
The exact source criterion below remains mandatory.

### Requirement: G01 release criterion

EARS pattern: event-driven.

WHEN G01 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> No unresolved ambiguity exists in normative syntax, typing, dynamics, observations, or canonical encoding.

#### Scenario: G01 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G01 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G02 release criterion

EARS pattern: event-driven.

WHEN G02 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Every deployable program has machine-checked finite lifetime and resource bounds.

#### Scenario: G02 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G02 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G03 release criterion

EARS pattern: event-driven.

WHEN G03 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Every accepted semantic motion has a proof delta, migration result, backend result, and negative test.

#### Scenario: G03 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G03 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G04 release criterion

EARS pattern: event-driven.

WHEN G04 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> At least two independent semantics implementations pass every conformance vector, including every mandatory ACTUS vector.

#### Scenario: G04 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G04 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G05 release criterion

EARS pattern: event-driven.

WHEN G05 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Reference and generated backend agree on every required trace and effect projection, including every present ordered ACTUS result field.

#### Scenario: G05 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G05 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G06 release criterion

EARS pattern: event-driven.

WHEN G06 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Every adjacent lifecycle artifact binds its predecessor digest, domain, state, resolver and contract identity, upgrade state, assumptions, and version. The intent verifier rejects substitution, omission, equivocation, staleness, or any unauthorized objective, quote, authorization, resolution, plan, fee, disclosure, fill, fulfillment, and recovery mutation before its authority boundary.

#### Scenario: G06 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G06 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G07 release criterion

EARS pattern: event-driven.

WHEN G07 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Replay fails across network, contract, version, sequence, nonce, validity, and cancellation domains.

#### Scenario: G07 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G07 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G08 release criterion

EARS pattern: event-driven.

WHEN G08 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> All public effects and private disclosures match the signed intent and approved manifests. Disclosure acceptance is distinct from effect authorization and cannot expand it.

#### Scenario: G08 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G08 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G09 release criterion

EARS pattern: event-driven.

WHEN G09 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Real proofs verify under pinned, approved, and digest-bound parameters.

#### Scenario: G09 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G09 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G10 release criterion

EARS pattern: event-driven.

WHEN G10 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Proof and ledger costs meet frozen application-specific budgets on named hardware and network versions.

#### Scenario: G10 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G10 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G11 release criterion

EARS pattern: event-driven.

WHEN G11 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Every security-critical compiler or verifier mutant is detected, or the survivor blocks release.

#### Scenario: G11 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G11 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G12 release criterion

EARS pattern: event-driven.

WHEN G12 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Build artifacts reproduce byte-for-byte on two independent clean builders.

#### Scenario: G12 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G12 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G13 release criterion

EARS pattern: event-driven.

WHEN G13 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> No critical or high audit finding remains unresolved.

#### Scenario: G13 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G13 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G14 release criterion

EARS pattern: event-driven.

WHEN G14 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> The SDK has stable errors, typed verification certificates, idempotency, version negotiation, asynchronous cancellation, partial-fill residuals, durable delivery deduplication, crash consistency, fill and payout separation, settlement levels, typed reversals, compensation, refund, and unavailable outcomes.

#### Scenario: G14 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G14 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G15 release criterion

EARS pattern: event-driven.

WHEN G15 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> A compromised solver, Runtime, indexer, registry, or remote prover cannot create a signing request for extra effects.

#### Scenario: G15 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G15 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G16 release criterion

EARS pattern: event-driven.

WHEN G16 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> The proof claim names every trusted compiler, circuit, verifier, parameter, ledger, wallet, oracle, relay, validator, treasury, custodian, and bridge assumption. Confidential profiles have a separate leakage theorem and trusted-computing-base statement.

#### Scenario: G16 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G16 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G17 release criterion

EARS pattern: event-driven.

WHEN G17 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> At least two non-toy pilots, including the complete ACTUS benchmark, prefer Moriarty's assurance workflow to the audited Compact-library baseline.

#### Scenario: G17 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G17 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G18 release criterion

EARS pattern: event-driven.

WHEN G18 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> If G05, G06, G09, G15, G17, or any ACTUS gate G19 through G24 fails, reduce scope or stop the language path.

#### Scenario: G18 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G18 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G19 release criterion

EARS pattern: event-driven.

WHEN G19 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> The ACTUS source lock reproduces all public site and repository dispositions, the 32-row taxonomy, the 18 executable contract types, 276 contract fixtures, one analysis-date fixture, all licenses, and every known access limitation.

#### Scenario: G19 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G19 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G20 release criterion

EARS pattern: event-driven.

WHEN G20 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Discovery, lossless parsing, and execution cover all 277 mandatory ACTUS vectors. No vector is skipped, excluded, quarantined, marked as an expected failure, or hidden by an allowlist.

#### Scenario: G20 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G20 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G21 release criterion

EARS pattern: event-driven.

WHEN G21 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Two independent semantics implementations match every present ordered result field for every ACTUS vector under the frozen decimal, rounding, date, calendar, event-order, observation, and comparison rules.

#### Scenario: G21 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G21 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G22 release criterion

EARS pattern: event-driven.

WHEN G22 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Every ACTUS vector elaborates through the shared typed surface and canonical Core, compiles through the general Compact backend, matches the reference trace and effect projection, and stays within declared finite resource bounds. No fixture-specific or contract-specific compiler bypass exists.

#### Scenario: G22 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G22 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G23 release criterion

EARS pattern: event-driven.

WHEN G23 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> Every one of the 32 ACTUS taxonomy rows has exactly one evidence-backed disposition. Only the 18 executable types with passing vectors count as implemented-and-vector-tested.

#### Scenario: G23 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G23 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.

### Requirement: G24 release criterion

EARS pattern: event-driven.

WHEN G24 acceptance is requested, the release validator SHALL independently establish the complete source criterion below.

Source criterion:

> The release dossier uses ACTUS reference-vector compatibility, preserves attribution and license duties, identifies the private Java core as optional and unavailable unless authorized, states limitations, and makes no certification or endorsement claim.

#### Scenario: G24 evidence-backed acceptance

- **WHEN** the required evidence satisfies the complete source criterion
- **THEN** the validator SHALL record the exact artifacts and independent result.

#### Scenario: G24 missing evidence

- **WHEN** required evidence is missing, stale, inconclusive, or contradicted
- **THEN** the validator SHALL refuse gate acceptance.
- **AND** the report SHALL distinguish missing evidence from a demonstrated failure.
