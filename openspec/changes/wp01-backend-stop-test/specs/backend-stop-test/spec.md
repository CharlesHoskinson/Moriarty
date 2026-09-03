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

### Requirement: third-party disclosure negative control

The Compact compiler SHALL reject a witness disclosure that the manifest does
not declare.

#### Scenario: compile the negative fixture

- WHEN the undeclared-decision fixture is compiled
- THEN compilation exits nonzero
- AND no successful artifact replaces the preserved failure result.

### Requirement: Moriarty disclosure correspondence

Moriarty SHALL compare its visibility manifest with generated Compact and
compiler metadata. The check SHALL parse lexical whitespace and SHALL ignore
comments and string literals. This check SHALL fail on missing or additional
disclosure. It SHALL reject a disclosure expression that is not one declared
identifier.

#### Scenario: generated code discloses an undeclared value

- WHEN generated Compact or compiler metadata exposes an undeclared value
- THEN backend validation fails before proof generation or signing.

#### Scenario: disclosure syntax contains whitespace

- WHEN generated Compact contains `disclose ( value )`
- THEN the validator treats it as an executable disclosure
- AND a missing or additional value still fails the gate.

#### Scenario: compiler metadata differs from the manifest

- WHEN a compiler reports another version, circuit argument, witness, or ledger
  field
- THEN backend validation fails before proof generation or signing.
- AND the ledger comparison includes index, export status, storage kind, and
  complete type metadata.

### Requirement: separate reproduction identities

The package SHALL keep the clean pinned-commit reproduction separate from
current-checkout validation. Current-checkout evidence without a commit identity
SHALL bind every decision-bearing source by SHA-256.

#### Scenario: publish remediated validation

- WHEN the current checkout adds a validator that is absent from the clean
  reproduced commit
- THEN a separate receipt identifies the current source hashes and commands
- AND the clean receipt continues to identify only the pinned commit artifacts.

### Requirement: bounded claim scope

The package SHALL label its current result S3. It SHALL NOT claim a real proof,
ledger feasibility, production cost, or independent audit.

#### Scenario: publish the result

- WHEN the wiki and matrix are updated
- THEN each claim states its assumptions and reproduction status.
