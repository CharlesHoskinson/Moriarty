# Backend stop-test specification

## ADDED Requirements

### Requirement: finite source fragment

The experiment SHALL contain accounts, deposits, bounded choices, a timeout,
and an atomic two-token swap. Every collection and transition count SHALL have
a static upper bound.

#### Scenario: inspect the generated contract

- WHEN the generator emits Compact
- THEN every collection has a declared maximum size
- AND every witness input has a declared visibility and purpose.

### Requirement: deterministic correspondence

The reference interpreter and generated backend SHALL agree on at least 1,000
seeded traces. The result SHALL include final state, transfers, warnings,
errors, and disclosure decisions.

#### Scenario: one trace diverges

- WHEN any compared field differs
- THEN the package gate fails
- AND the report preserves the smallest failing trace.

### Requirement: artifact identity

The package SHALL produce a manifest and translation certificate with hashes
for source, Core, Compact, ZKIR, compiler tuple, and test corpus.

#### Scenario: one artifact changes

- WHEN a client validates an artifact with a mismatched digest
- THEN certificate validation fails before signing.

### Requirement: disclosure fail-closed behavior

The Compact compiler SHALL reject a witness disclosure that the manifest does
not declare.

#### Scenario: compile the negative fixture

- WHEN the undeclared-decision fixture is compiled
- THEN compilation exits nonzero
- AND no successful artifact replaces the preserved failure result.

### Requirement: bounded claim scope

The package SHALL label its result S4 at most. It SHALL NOT claim a real proof,
ledger feasibility, production cost, or independent audit.

#### Scenario: publish the result

- WHEN the wiki and matrix are updated
- THEN each claim states its assumptions and reproduction status.
