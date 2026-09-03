# Normative-assurance specification

## ADDED Requirements

### Requirement: complete strategy comparison

The comparison SHALL evaluate Isabelle, Agda, Lean, a compact reference
interpreter, and multiple differential implementations.

#### Scenario: a strategy reuses existing proofs

- WHEN proof reuse depends on missing implementation correspondence
- THEN the score records that gap instead of awarding full reuse credit.

### Requirement: exact theorem inventory

The package SHALL define termination, lifetime, conservation, account
positivity, closure, quiescence, determinism, authorization, serialization, and
resource theorems with assumptions.

#### Scenario: a property depends on participant action

- WHEN liveness needs a submitter, key, continuation, or ledger capacity
- THEN that dependency appears in the theorem statement.

### Requirement: executable prototype

Each viable strategy SHALL execute at least one shared transition vector. The
two strongest viable strategies SHALL prototype all three obligations frozen in
`assurance-scorecard.json`.

#### Scenario: a prototype uses an axiom or admitted result

- WHEN an unproved assumption remains
- THEN the output lists it and cannot count it as a completed proof.

### Requirement: normative authority

The final decision SHALL name the normative artifact, derived artifacts,
conformance artifacts, owners, versioning policy, and drift-detection process.

#### Scenario: two maintained implementations disagree

- WHEN a conformance vector diverges
- THEN release stops until the normative disposition is applied.

### Requirement: maintainability stop gate

The package SHALL select library-only when no strategy has two credible
maintainers or an approved succession plan.

#### Scenario: one specialist is the only maintainer

- WHEN no second maintainer or funded succession exists
- THEN the language path fails the maintainability gate.

### Requirement: precommitted selection

Scores, weights, uncertainty penalties, viability thresholds, maintainer
criteria, and tie rules SHALL be frozen before prototype results exist.

#### Scenario: a preferred strategy misses one minimum criterion

- WHEN its weighted score passes but one frozen minimum fails
- THEN the strategy is not viable.
