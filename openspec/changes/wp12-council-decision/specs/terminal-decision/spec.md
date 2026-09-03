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
The package SHALL verify model identities through the terminal evidence source
specified in `decision-scorecard.json`. Reviewer self-labels are not terminal
model evidence. Auxiliary model calls SHALL remain disclosed.

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

### Requirement: deterministic selection

The decision SHALL apply the frozen eligibility, safety-veto, selection-order,
tie, and abstention rules in `decision-scorecard.json`.

#### Scenario: language and libraries are within the tie margin

- WHEN both alternatives are eligible and differ by less than 0.5
- THEN the record selects `audited-compact-libraries`.

### Requirement: implementation authority

The terminal record SHALL name approved semantic scope, SDK scope, owners,
funding boundary, release gates, rollback triggers, and the next sprint.

#### Scenario: the language option wins

- WHEN the decision is signed
- THEN implementation cannot add Core constructs outside the approved scope ledger.

### Requirement: signed authority

The decision authority SHALL sign canonical decision bytes with Ed25519 through
minisign. The record SHALL name the public key and authority.

#### Scenario: the decision file changes

- WHEN the signed bytes differ from the reviewed record
- THEN signature verification fails and no implementation authority exists.
