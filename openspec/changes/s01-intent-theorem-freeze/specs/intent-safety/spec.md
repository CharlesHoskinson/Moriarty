# Intent-safety specification

## ADDED Requirements

The following ten acceptance predicates are requirements, not passed results.
The S01 package remains specified-only and in-progress until all evidence and
validation tasks pass.

### Requirement: unambiguous terminology

Every required term SHALL have one definition and no unresolved alias.

#### Scenario: a required term is inspected

- WHEN the validator reads the terminology registry
- THEN it SHALL find one definition and no unresolved alias
- AND ambiguity SHALL produce `block-s01-on-ambiguity`.

### Requirement: bounded lifecycle authority

Every lifecycle object SHALL have one primary category and one authority
boundary.

#### Scenario: a lifecycle object is inspected

- WHEN the validator reads a lifecycle object
- THEN it SHALL find exactly one primary category and one authority boundary.

### Requirement: bound hard predicates

Every hard predicate SHALL have a signed binding or an explicit external
premise.

#### Scenario: a hard predicate enters authorization

- WHEN a hard predicate contributes to `authorizedAt`
- THEN its record SHALL identify a signed binding or explicit external premise.

### Requirement: separate optimization

No optimization preference SHALL enter `authorizedAt`.

#### Scenario: a plan receives an optimization rank

- WHEN an optimization preference ranks a plan
- THEN the preference SHALL NOT change any authorization result.

### Requirement: complete theorem statement

The theorem SHALL contain all required variables, premises, bindings, and
claims.

#### Scenario: the theorem artifact is inspected

- WHEN the validator reads the candidate theorem
- THEN it SHALL find every required variable, premise, binding, and claim.

### Requirement: explicit assumptions

Every assumption SHALL have a scope, evidence rule, and failure result.

#### Scenario: an assumption is unavailable

- WHEN required assumption evidence is missing, stale, revoked, contradictory, or unsupported
- THEN the declared rule SHALL return unavailable or invalid.

### Requirement: controlled observations

Every observation field SHALL have visibility and declassification rules.

#### Scenario: an observation field is inspected

- WHEN the validator reads an observation field
- THEN it SHALL find visibility and declassification rules.

### Requirement: constrained atomic-swap baseline

The eventual atomic-swap checker SHALL require the baseline to pass within its
declared scope.

#### Scenario: the SignAfterResolve baseline is checked

- WHEN the future checker evaluates the `SignAfterResolve` baseline
- THEN it SHALL use exact transfer comparison only
- AND it SHALL NOT claim authenticated effect completeness
- AND it SHALL NOT authorize signing.

### Requirement: rejected extra-effect mutant

The eventual checker SHALL reject the extra-effect mutant without widening the
signed intent.

#### Scenario: the mutant adds an unauthorized transfer

- WHEN the mutant preserves baseline transfers and adds one third-party transfer
- THEN the checker SHALL fail with `UNAUTHORIZED_EXTRA_EFFECT`
- AND the failure outcome SHALL be `reject-mutant-without-widening-intent`.

### Requirement: stable semantic scope

The semantic-scope version and digest SHALL remain unchanged.

#### Scenario: the S01 transition is inspected

- WHEN the validator checks semantic scope `0.0.0-e00.2`
- THEN it SHALL find the approved digest unchanged
- AND it SHALL move an architecture-specific operation to S02.

## Open gaps

Complete effect projection remains open. Its failure result is
`reject-incomplete-effect-projection`. The `SignBeforeResolve` boundaries remain
open. An architecture-specific Core operation produces
`move-architecture-operation-to-s02`.

