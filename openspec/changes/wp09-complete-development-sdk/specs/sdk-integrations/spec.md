# SDK integration specification

## ADDED Requirements

### Requirement: replaceable adapters

The SDK SHALL define replaceable adapters for wallets, custody, Runtime,
oracles, identity, registries, continuations, chains, indexers, events, payouts,
and explorers.

#### Scenario: two index providers disagree

- WHEN their contract state or chain point differs
- THEN the SDK exposes the disagreement and blocks signing.

### Requirement: wallet and custody isolation

Wallet adapters SHALL receive only a locally verified immutable signing
request. They SHALL NOT decide contract semantics or disclosure policy.

#### Scenario: a hardware wallet cannot display one effect

- WHEN the user cannot review the effect through the approved channel
- THEN the signing flow stops or uses an approved external display binding.

### Requirement: oracle and identity evidence

Oracle and identity adapters SHALL return signed, versioned evidence with
source, domain, freshness, replay, revocation, and confidence metadata.

#### Scenario: oracle evidence is stale

- WHEN its freshness policy fails
- THEN the adapter result cannot satisfy the transition capability.

### Requirement: Runtime trust minimization

Runtime adapters SHALL separate discovery, indexing, continuation retrieval,
planning, estimation, and submission. Clients SHALL verify decision-bearing
outputs locally.

#### Scenario: Runtime returns a valid but unintended transaction

- WHEN its effects differ from user intent
- THEN local verification rejects it before signing.

### Requirement: rollback and event semantics

Chain and index adapters SHALL expose chain points, rollbacks, finality policy,
event ordering, deduplication keys, and recovery cursors.

#### Scenario: a reported transition is rolled back

- WHEN the adapter receives the rollback
- THEN derived state and events revert deterministically to the named chain point.

### Requirement: browser and server parity

Browser and Node.js adapters SHALL use identical canonical data and verification
rules. Environment-specific transport SHALL not change semantics.

#### Scenario: both environments verify one plan

- WHEN inputs and versions match
- THEN both return the same verification result and reason code.
