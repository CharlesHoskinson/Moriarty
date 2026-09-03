# Marlowe semantic-delta specification

## ADDED Requirements

### Requirement: complete construct inventory

The matrix SHALL cover contracts, actions, inputs, values, observations,
parties, payees, tokens, accounts, choices, timeouts, state, warnings, errors,
payments, and results.

#### Scenario: a construct has several implementations

- WHEN Isabelle, Haskell, Plutus, Agda, or TypeScript defines the construct
- THEN the row lists every maintained definition and its pinned locator.

### Requirement: exact inheritance evidence

Every inherited guarantee SHALL cite an exact theorem or reproduced
correspondence test. Otherwise, the matrix SHALL mark new proof work.

#### Scenario: a documentation page makes a broad safety claim

- WHEN no exact theorem matches its assumptions and conclusion
- THEN the claim is qualified and excluded from inherited guarantees.

### Requirement: realization separation

The matrix SHALL distinguish semantic behavior, validator acceptance, ledger
feasibility, transaction planning, continuation availability, participant
cooperation, and future protocol support.

#### Scenario: Close terminates abstractly

- WHEN ledger limits prevent one closing transaction
- THEN semantic termination remains separate from operational executability.

### Requirement: serialization and identity

The delta SHALL compare JSON, Plutus data, continuation hashes, source maps,
language versions, and validator versions.

#### Scenario: two encodings represent the same source intent

- WHEN their canonical bytes or hashes differ
- THEN the compatibility report records the difference and migration impact.

### Requirement: contradiction preservation

The package SHALL record unresolved disagreements in `wiki/contradictions.md`.
It SHALL NOT silently prefer the newest-looking source.

#### Scenario: Isabelle and Haskell differ

- WHEN behavior differs on a reproduced example
- THEN the matrix records both results and blocks unconditional inheritance.
