## ADDED Requirements

### Requirement: Claim discharge map
Acceptance SHALL discharge all four mandatory claim families at declared loci. ContractInvariant SHALL be discharged by the deploy-time property certificate and the per-step invariant. IntentRefinement and TransitionValidity SHALL be discharged by the fused step relation. HistoryCompliance SHALL be discharged by ledger applicability plus provenance: induction on ledger, or a certificate off ledger. One canonical claim vocabulary SHALL be used, with historical aliases recorded.

#### Scenario: Induction conditions hold
- **WHEN** a deployment has immutable keys, constrained genesis and a passing head-discipline check
- **THEN** HistoryCompliance for its on-ledger heads is discharged by induction over ledger acceptance.

#### Scenario: Induction condition missing
- **WHEN** immutable keys, constrained genesis or head discipline is not established for a deployment
- **THEN** HistoryCompliance is not discharged by induction and acceptance is not claimed for that deployment.

### Requirement: Constrained genesis and termination
Deployment SHALL write `Uninit(Π_P, netTag)`. `Initialize` SHALL prove the genesis predicate with all-principal signatures, succeed at most once and derive `instanceId = H(A ‖ H(G))`. `Terminate` SHALL move a `Live` head to `Terminated` only under the contract rules' terminal condition or the principal threshold declared in Π_P, and SHALL record a disposition for every residual obligation.

#### Scenario: Genesis and terminal step
- **WHEN** all principals sign a valid genesis body and later a permitted terminal condition holds
- **THEN** `Initialize` creates the instance once and `Terminate` records every residual obligation's disposition.

#### Scenario: Invalid lifecycle action
- **WHEN** a second `Initialize` runs, a principal signature is missing, the deployment is not in `Uninit`, any action targets a `Terminated` head, or `Terminate` leaves an obligation without a disposition
- **THEN** the call rejects.

### Requirement: Intent digest v2 and program digest
Canonical encodings and test vectors SHALL exist for Π_P, intent digest v2 and genesis body v2. Exact-head mode SHALL bind netTag, contract, entry point, instance, head, revision, state commitment, Π_P, action and arguments, caps, observation policy, certificate requirements and validity window; its nonce is informational. Outcome mode SHALL bind the constraint set, a program-digest allowlist, the validity window and a nonce recorded in a per-instance consumed-nonce set. Π_P bounds SHALL be checked before proving.

#### Scenario: Canonical vectors
- **WHEN** an implementation encodes the published test vectors
- **THEN** it reproduces every digest byte for byte.

#### Scenario: Altered or replayed intent
- **WHEN** a bound field is altered, an outcome nonce is reused, an exact-head intent is replayed after the revision increments, or an input exceeds a Π_P bound
- **THEN** acceptance rejects before applying effects, and a bound violation rejects before expensive proving.

### Requirement: Observation freshness at application
Observations SHALL be signed by keys in `σ.oracleKeys` and verified in the step relation. The feed allowlist and `maxObservationAge` SHALL be part of the observation policy bound in the intent digest. Freshness SHALL use block-time reads in the transcript that the step relation proves and the ledger re-executes at application.

#### Scenario: Fresh observation
- **WHEN** a signed observation from an allowed feed is within its maximum age at application
- **THEN** the call applies.

#### Scenario: Stale observation
- **WHEN** an observation is older than its maximum age at application on a local ledger node or on Preview
- **THEN** application rejects, and an offline proof verification is not evidence of freshness.

### Requirement: Forward-declared migration replaces in-place revocation
Operation keys SHALL be immutable. Replacement SHALL happen only through `Migrate` to a successor with a strictly greater version, under one of two branches. In the declared branch, Π_new SHALL be in the successor allowlist compiled into Π_old, which holds program digests only, and the migration threshold declared in Π_old SHALL sign netTag, A_old, A_new, Π_new, revision and state. In the unanimous branch, all principals SHALL sign `MORIARTY-MIGRATE-v2` over the same fields. The successor SHALL record its predecessor address in its deploy-time state `Uninit(Π_new, netTag, A_old)`. `ImportFrom` SHALL accept only a claimed `Migrate` call from that address whose commitment is absent from its imported set. An optional principal-threshold `Pause` entry point MAY halt new steps without changing keys. Cross-contract evidence requires ledger 9, and its Preview qualification is a tracked dependency.

#### Scenario: Declared migration
- **WHEN** principals audit A_new, sign a declared migration and submit `Migrate` with the claimed `ImportFrom`
- **THEN** the old head enters `Releasing`, the new head keeps obligations and budgets, and consumed state stays consumed.

#### Scenario: Unauthorized or unsafe migration
- **WHEN** a maintenance update, key substitution, downgrade, successor neither declared nor unanimously signed, import from a caller other than the recorded predecessor, second import of one commitment, or revival of consumed authority is attempted
- **THEN** the ledger or the step relation rejects it.

### Requirement: Governed deploy-time property certificate
The deploy audit SHALL check the universal ContractProperty certificate and act as the governed registration mechanism. The certificate hash SHALL be bound in Π_P. A ledger-checked `Initialize` certificate MAY replace the audit check once `ledger-10` certificates are accepted.

#### Scenario: Checked certificate
- **WHEN** the deploy audit verifies the property certificate against Π_P
- **THEN** the deployment may count ContractInvariant as discharged at deploy time.

#### Scenario: Hash without a check
- **WHEN** Π_P carries a certificate hash but the certificate is absent or fails its check
- **THEN** the audit fails and ContractInvariant is not discharged.
