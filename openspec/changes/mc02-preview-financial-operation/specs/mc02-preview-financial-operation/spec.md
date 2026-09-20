## ADDED Requirements

Product predicates govern objective semantics, proofs, participant authorization and ledger validity. Package promotion, reviewer identities and RP01/RP02/RP03 dispatch conditions govern internal maintainer delivery and project-operated resources only. They SHALL NOT be required to author, compile, prove or deploy an independently authored supported program. The [product contract](../../../../../docs/MORIARTY-PRODUCT-CONTRACT.md) and [roadmap reconciliation](../../../../../docs/ROADMAP-RECONCILIATION-2026-09-19.md) control scope; this correction establishes no new implementation evidence.

### Requirement: Explicit financial meaning
The experiment SHALL distinguish contractual denomination, test-token identity, dues, and actual transferred value.

#### Scenario: Feasible funded episode
- **WHEN** the dedicated wallet executes the frozen small loan fixture with its explicit asset mapping
- **THEN** real supported ledger assets move, contractual dues update correctly, and independent expected values match.

#### Scenario: Fictitious settlement
- **WHEN** a result supplies only a message update, synthetic balance field, or undisclosed currency conversion
- **THEN** the financial-settlement gate rejects it as transfer evidence.

### Requirement: Independent complete comparison
The checker SHALL compare complete local financial effects with finalized Preview effects and contract state.

#### Scenario: Two target families
- **WHEN** the generated loan and pool-swap programs settle on Preview
- **THEN** complete effects and state match local expectations, with indexed success, canonical finality, and exact readback.

#### Scenario: Partial comparison
- **WHEN** observed effects contain a wrong recipient, wrong domain, excess fee, missing debit, extra approval, or undeclared write
- **THEN** the complete-effect comparator rejects the result.

### Requirement: Integration scope
This package SHALL label its transactions uncertified until MC05 enforces all mandatory claims.

#### Scenario: Honest integration result
- **WHEN** MC02 produces a valid financial integration receipt before MC05 acceptance
- **THEN** the receipt names the financial predicate and marks mandatory proof claims unavailable.

#### Scenario: Promotion shortcut
- **WHEN** network success is offered as evidence of history compliance or compiler correspondence
- **THEN** the package gate rejects the unsupported proof claim.

### Requirement: Internal maintainer audit and provenance
For internal delivery, the package SHALL bind release evidence to exact sources, commands, environment, outputs, and the independent audit identities selected by the current task. Reviewer records SHALL NOT enter public program or transaction validity.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** an independently selected internal reviewer is unavailable or lacks a fresh substantive identity-bound verdict
- **THEN** the internal package remains pending audit; public validation does not reject a supported program solely because that maintainer audit is absent.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Scoped financial integration
For project-operated integration campaigns, RP01 and RP03 govern resource dispatch; finalized financial effects SHALL be compared independently while the integration contract remains explicitly uncertified until MC05.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).
