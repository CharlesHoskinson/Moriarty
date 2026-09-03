# High-risk-composition specification

## ADDED Requirements

### Requirement: conditional-token conservation

Split SHALL lock one backing quantity and create equal complete-set claims.
Merge SHALL consume one complete set and release the matching backing quantity.

#### Scenario: an incomplete set is merged

- WHEN any outcome claim is missing or has the wrong quantity
- THEN merge fails without releasing backing.

### Requirement: attestation binding

Resolution SHALL bind source identity, event identifier, outcome domain,
contract identifier, state sequence, validity interval, and replay domain.

#### Scenario: an attestation is replayed across contracts

- WHEN the contract identifier differs
- THEN resolution fails without consuming or creating value.

### Requirement: atomic exchange

The conditional-token exchange SHALL transfer both legs or neither leg. Partial
submission SHALL not create a unilateral settlement path.

#### Scenario: one party does not complete

- WHEN the timeout becomes eligible
- THEN the contract follows its declared refund path.

### Requirement: artifact integrity

The client SHALL verify Core, Compact, ZKIR, token-policy, oracle-capability,
disclosure, and compiler digests before signing.

#### Scenario: the oracle capability changes

- WHEN a plan substitutes another capability identifier
- THEN client verification rejects the plan.

### Requirement: Core inclusion gate

The package SHALL reject Core inclusion after any reproducible conservation,
authorization, replay, nondeterminism, or unbounded-resource failure.

#### Scenario: one invariant fails

- WHEN a minimized failing trace reproduces
- THEN the motion returns to WP04 with a reject or revise disposition.
