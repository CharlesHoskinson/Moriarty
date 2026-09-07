## ADDED Requirements

### Requirement: Complete ACTUS evidence
Coverage SHALL include all 18 executable types and 277 fixtures with every present result field.

#### Scenario: Full comparison
- **WHEN** the conformance harness evaluates all 277 pinned ACTUS fixtures across all 18 executable types
- **THEN** all nine present result-field kinds match explicit per-field rules for units, event ordering, calendars, and numbers.

#### Scenario: Coverage shortcut
- **WHEN** coverage skips fixtures or fields, uses a Float32 oracle shortcut, applies an unjustified global tolerance, or lacks results
- **THEN** the conformance gate fails without reducing its required denominator.

### Requirement: Complete DeFi evidence
Each of the 72 DeFi rows SHALL have row-specific behavior, refinement, feasible positives, and meaningful negative evidence.

#### Scenario: Target implementation
- **WHEN** the harness tests all 72 DeFi rows against their source-defined behaviors
- **THEN** each row has implemented refinement, feasible positives, meaningful negatives, certificates, and acceptance evidence.

#### Scenario: Taxonomy substitution
- **WHEN** a row provides only a family classification, keyword certificate, or aggregate passing count
- **THEN** the individual row remains incomplete and the full conformance gate fails.

### Requirement: Held-out semantic coverage
The package SHALL implement NAM19 capitalization, accepted refinance, and pending redemption with carried obligations.

#### Scenario: Three held-outs
- **WHEN** the harness executes NAM19 IPCI, accepted refinance, and pending redemption cases
- **THEN** IPCI capitalizes interest with zero payoff, refinance binds accepted terms and debt identity, and redemption separates request from claim.

#### Scenario: Lost obligations
- **WHEN** a held-out action makes premature payoff, unilaterally refinances, forgets unfilled redemption, or loses residual authority
- **THEN** semantic and acceptance checks reject the action.

### Requirement: Source gaps remain explicit
All 32 ACTUS taxonomy dispositions and DS-01 through DS-07 SHALL remain visible until justified resolution.

#### Scenario: Independent resolution
- **WHEN** a source decision addresses ANN initialization, CLM schedule, absent CSMP fixtures, or FXOUT event identity
- **THEN** primary-source evidence and executable tests support it while all 32 taxonomy dispositions and DS-01 through DS-07 remain visible.

#### Scenario: Fabricated conformance
- **WHEN** a source-gap disposition is submitted as a passing fixture or permission to reduce coverage
- **THEN** the coverage gate rejects the substitution and preserves the unresolved requirement.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Extended native predicate evidence
The package SHALL implement, review, prove, and independently verify its allocated native relation before proof-dependent acceptance.

#### Scenario: Reviewed extension executes
- **WHEN** both auditors approve the implemented relation and contract, and the allocated campaign passes
- **THEN** retained proofs and separate-process verification establish only the extended predicate and requalified dependent correspondence.

#### Scenario: Fixed proof substituted for extension
- **WHEN** the package supplies only an earlier fixed-loan proof, exceeds its campaign allocation, or lacks a required extension proof
- **THEN** package acceptance remains blocked and no earlier proof or counter can substitute.
