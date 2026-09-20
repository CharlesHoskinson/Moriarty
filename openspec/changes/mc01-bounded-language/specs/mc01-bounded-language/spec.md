## ADDED Requirements

Product predicates govern objective semantics, proofs, participant authorization and ledger validity. Package promotion, reviewer identities and RP01/RP02/RP03 dispatch conditions govern internal maintainer delivery and project-operated resources only. They SHALL NOT be required to author, compile, prove or deploy an independently authored supported program. The [product contract](../../../../../docs/MORIARTY-PRODUCT-CONTRACT.md) and [roadmap reconciliation](../../../../../docs/ROADMAP-RECONCILIATION-2026-09-19.md) control scope; this correction establishes no new implementation evidence.

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

### Requirement: Target-driven successor profile
Before a successor language profile is frozen, RP01 SHALL trace the selected financial challenges through source, intent, authority, liabilities, effects, bounds and acceptance; unsupported constructs SHALL remain explicit versioned extension obligations.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).

### Requirement: Complete successor specification and executable K semantics
The successor SHALL define separate lexical rules, ISO/IEC 14977 EBNF, static judgments and executable Moriarty Core semantics in K. Required metatheorems and correspondence domains SHALL have explicit proof status.

#### Scenario: Supported successor agreement
- **WHEN** a supported `.mori` agreement is parsed, checked and elaborated
- **THEN** its K execution and evaluator produce the independently expected complete observation, including residual duties and work.

#### Scenario: Missing semantics or substituted proof
- **WHEN** a required production, constructor or proof obligation is missing, or archived ZKIR evidence is substituted
- **THEN** MC01 successor acceptance remains incomplete.
