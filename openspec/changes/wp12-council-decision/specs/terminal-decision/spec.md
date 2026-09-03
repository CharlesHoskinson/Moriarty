# Terminal-decision specification

## ADDED Requirements

### Requirement: immutable decision bundle

The package SHALL bind every reviewed artifact to repository commits, content
digests, versions, and prior package gate results.

#### Scenario: an artifact changes during review

- WHEN its digest differs from the frozen manifest
- THEN the review is invalid for that artifact.

### Requirement: three-provider critique

Grok, Sol, and exact Fable 5.1 SHALL review every work package. Each provider
SHALL return an independent verdict, findings, and evidence gaps.

#### Scenario: one provider fails or reports another model

- WHEN terminal provider evidence is absent or mismatched
- THEN that result is an abstention and cannot count as approval.

### Requirement: dissent preservation

The decision record SHALL preserve minority findings, unresolved assumptions,
reviewer abstentions, and rejected recommendations.

#### Scenario: the majority approves one package

- WHEN a minority identifies an unresolved stop-gate failure
- THEN the decision authority must disposition that failure explicitly.

### Requirement: terminal outcome

The record SHALL select `moriarty-language`, `audited-compact-libraries`, or
`stop`. It SHALL NOT select `continue-exploring` or an equivalent outcome.

#### Scenario: required evidence is missing

- WHEN the language gate cannot be evaluated
- THEN `moriarty-language` is ineligible.

### Requirement: implementation authority

The terminal record SHALL name approved semantic scope, SDK scope, owners,
funding boundary, release gates, rollback triggers, and the next sprint.

#### Scenario: the language option wins

- WHEN the decision is signed
- THEN implementation cannot add Core constructs outside the approved scope ledger.
