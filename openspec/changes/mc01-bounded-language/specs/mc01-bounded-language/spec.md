## ADDED Requirements

### Requirement: Defined authoring and canonical representation
The frontend SHALL define grammar, units, asset domains, typing, diagnostics, canonical bytes, and versioned elaboration.

#### Scenario: Valid loan and swap programs
- **WHEN** the frontend receives valid loan and swap source programs
- **THEN** both parse, typecheck, elaborate, roundtrip canonically, and match independent expected evaluator traces.

#### Scenario: Invalid authoring
- **WHEN** source contains recursion, ambiguous assets, unbounded collections, malformed encodings, or unsupported operations
- **THEN** the frontend rejects it before signing and reports the source location.

### Requirement: Finite semantic work
Every execution SHALL have explicit arithmetic, allocation, schedule, horizon, nesting, fold, transaction, proof-size, and predecessor bounds.

#### Scenario: Finite closure
- **WHEN** an execution follows ordinary work, pending progress, closure, or bounded epoch migration
- **THEN** each transition decreases the declared lifecycle measure and respects every declared local bound.

#### Scenario: Bound evasion
- **WHEN** a transition overflows, exhausts work, leaves intermediate arithmetic unchecked, or resets a continuation budget
- **THEN** the evaluator and proof relation reject that transition.

### Requirement: Target-driven semantic freeze
Every Core operation SHALL trace to an ACTUS behavior, DeFi behavior, or explicit developer requirement.

#### Scenario: Supported profile
- **WHEN** loan and swap source programs elaborate into Core
- **THEN** dues and settlement remain distinct, and gross debit and net delivery checks match independent expected effects.

#### Scenario: Taxonomy shortcut
- **WHEN** a proposed Core operation cites only a financial family label or convenient numeric tolerance
- **THEN** the semantic-freeze gate rejects that unsupported operation.

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
