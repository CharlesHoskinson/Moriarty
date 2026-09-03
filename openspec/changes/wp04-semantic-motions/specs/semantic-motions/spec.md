# Semantic-motion specification

## ADDED Requirements

### Requirement: complete motion set

The package SHALL decide numeric domains, partial payment, atomic action
collection, Attest, authority, mandate, split or merge, and state bounds.

#### Scenario: one motion remains ambiguous

- WHEN no single semantics satisfies the gate
- THEN the feature is deferred or moved outside the Core
- AND the Core freeze does not include it.

### Requirement: operational precision

Each accepted motion SHALL define preconditions, transition results, warnings,
errors, timeout behavior, ordering, and quiescence effects.

#### Scenario: two actions compete

- WHEN both are valid in the same interval
- THEN the semantics determines ordering or rejects the ambiguous collection.

### Requirement: boundedness

Every accepted motion SHALL preserve a computable lifetime, state, transition,
and witness bound before deployment.

#### Scenario: bounded iteration expands beyond the deployment limit

- WHEN the calculated envelope exceeds the backend budget
- THEN elaboration fails before artifact generation.

### Requirement: authorization and value safety

Authority, mandate, Attest, and split or merge motions SHALL state exact
credentials, assets, quantities, replay domains, and conservation equations.

#### Scenario: an attestation is replayed

- WHEN its domain, contract, state, or validity interval does not match
- THEN the transition fails without moving value.

### Requirement: decision evidence

Each disposition SHALL cite demand, semantics, proof effort, backend cost,
migration impact, and security evidence.

#### Scenario: feature demand is only speculative

- WHEN no protocol or pilot requires the feature
- THEN demand cannot justify Core inclusion.

### Requirement: semantic scope ledger

The package SHALL version the active Core after each accepted motion. Each
version SHALL list Core, surface, backend, external, deferred, and rejected scope.

#### Scenario: one motion changes the Core

- WHEN a motion is accepted or revised
- THEN the journal records the previous and new semantic scope
- AND it records new proof, migration, privacy, and resource obligations.
