# SDK planning-and-verification specification

## ADDED Requirements

### Requirement: typed user intent

The SDK SHALL represent contract identity, current state, desired transition,
value movement, fees, proofs, disclosures, capabilities, validity interval, and
signers in a typed intent.

#### Scenario: a planner adds a payment

- WHEN the payment is absent from approved intent
- THEN local verification rejects the plan before signing.

### Requirement: untrusted transaction planning

Planners MAY select inputs, collateral, change, fees, and providers. Clients
SHALL independently verify all semantic and monetary effects.

#### Scenario: a planner substitutes a script

- WHEN a validator, policy, or verifier identity differs from the manifest
- THEN the client rejects the plan.

### Requirement: untrusted coin selection

Coin selection SHALL be separate from semantic approval. The local verifier
SHALL recompute inputs, outputs, change, fees, collateral, network, and signers.

#### Scenario: coin selection diverts change

- WHEN selected inputs are sufficient but change uses an unapproved address
- THEN the verifier rejects the plan before signing.

### Requirement: generated backend validation

The backend validator SHALL compare Core, generated Compact, compiler metadata,
visibility, capabilities, resource bounds, and certificate digests.

#### Scenario: generated metadata adds one public effect

- WHEN the added effect is absent from the approved manifests
- THEN backend validation rejects the artifact before proof generation.

### Requirement: state and sequence verification

The verifier SHALL bind the plan to the expected contract, state hash, sequence,
continuation, ledger network, and rollback point.

#### Scenario: an old valid state is replayed

- WHEN its sequence or chain point is stale
- THEN the verifier rejects the plan as a state mismatch.

### Requirement: disclosure and capability verification

The verifier SHALL compare every public output and private disclosure against
the approved visibility and capability manifests.

#### Scenario: a private amount becomes public

- WHEN the manifest lacks that declassification
- THEN verification rejects the plan before proof generation or signing.

### Requirement: partial signing and idempotency

The SDK SHALL bind partial signatures to immutable intent and plan digests. A
retry SHALL not change either digest.

#### Scenario: a coordinator changes the fee or output

- WHEN it reuses earlier partial signatures
- THEN signature assembly fails because the plan digest differs.

### Requirement: closed verification result

Verification SHALL return `verified`, `rejected`, or `unavailable` with stable
reason codes. Only `verified` SHALL enable a signing request.

#### Scenario: a provider times out

- WHEN required state evidence is unavailable
- THEN the result is `unavailable`, not `verified` or a semantic rejection.
