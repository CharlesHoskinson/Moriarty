# SDK assurance-tools specification

## ADDED Requirements

### Requirement: normative simulation

The SDK SHALL provide a deterministic reference interpreter, simulator,
time-travel debugger, and visual trace format based on normative Core semantics.

#### Scenario: a user replays a saved trace

- WHEN inputs, versions, and seed match
- THEN the simulator returns identical states, payments, warnings, and errors.

### Requirement: static and symbolic analysis

The analyzer SHALL report reachability, timeout paths, partial payments,
unmet obligations, resource envelopes, external dependencies, and counterexamples.

#### Scenario: an action is unreachable

- WHEN symbolic exploration proves no valid path reaches it
- THEN the diagnostic identifies the source span and supporting path condition.

### Requirement: test interfaces

The SDK SHALL expose property, differential, fuzz, mutation, golden,
compatibility, on-chain, and cost-regression interfaces.

#### Scenario: two semantics implementations diverge

- WHEN a generated correlated vector produces different results
- THEN the runner preserves and minimizes the vector.

### Requirement: semantic comparison

The SDK SHALL compare Core artifacts for trace equivalence or report bounded
counterexamples. It SHALL distinguish source similarity from semantic equivalence.

#### Scenario: migration changes timeout behavior

- WHEN traces differ at a deadline boundary
- THEN the comparison reports the earliest divergent transition.

### Requirement: evidence export

Every assurance tool SHALL export a versioned machine-readable result with
input hashes, commands, environment, limits, and reproduction status.

#### Scenario: a result lacks its input digest

- WHEN evidence export is requested
- THEN the tool fails instead of emitting a reproducibility claim.
