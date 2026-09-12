## ADDED Requirements

### Requirement: Declared ledger verification seam
Moriarty acceptance SHALL count a contract-call proof as verified only when ledger `well_formed` checks it against the operation key read from `ContractState.operations`. A toolchain manifest SHALL pin Compact, ZKIR, ledger and proof-server versions for each network generation: ledger 8, ledger 9 and `ledger-10`.

#### Scenario: Ledger-verified call
- **WHEN** a Moriarty call is accepted on a pinned network generation
- **THEN** its evidence records the operation key read from contract state, the toolchain manifest entry and the ledger acceptance result.

#### Scenario: Substituted verification
- **WHEN** evidence offers a host verdict, a mock verifier, an in-circuit pairing claim or a proof checked against a key not read from contract state
- **THEN** the evidence is rejected and the requirement stays open.

### Requirement: Fused step relation per entry point
Each Moriarty entry point SHALL compile to one circuit that proves authorization, transition validity, effect correspondence, intent refinement and the per-step invariant together. The native effects SHALL equal the projection of the Moriarty effects. If experiment E2 shows the funded repayment step cannot fit k ≤ 17 in any of its three variants, work SHALL stop and revisit relation decomposition without relaxing acceptance requirements.

#### Scenario: Fused proof verifies
- **WHEN** the proof server proves a funded repayment step
- **THEN** one proof verifies against the compiled key and binds every effect, recipient, asset and cap.

#### Scenario: Unbound field or separate claim proofs
- **WHEN** an effect, recipient, asset or cap is mutated, or a design proves each claim of one action in a separate proof
- **THEN** proving or verification fails for the mutation, and review rejects the separate-proof design.

#### Scenario: Fit failure
- **WHEN** no E2 variant fits k ≤ 17
- **THEN** the step relation is not promoted and the acceptance requirements stay unchanged.

### Requirement: Head read-then-write discipline
Every head write SHALL follow a read of that head in the same transcript section, with no checkpoint between them. Head creation SHALL follow an absence read. A checker over generated ZKIR SHALL enforce both rules. Experiment E1 on Preview SHALL show that at most one of two individually valid conflicting calls from one head applies.

#### Scenario: Conflicting calls
- **WHEN** two individually valid calls consume the same head on Preview
- **THEN** at most one applies, and the other fails its read at application.

#### Scenario: Blind write
- **WHEN** generated ZKIR writes a head without the preceding read, or creates a head without an absence read
- **THEN** the checker rejects the build.

#### Scenario: E1 fails
- **WHEN** E1 shows both conflicting calls applied
- **THEN** Stages 2–6 stop until a reviewed explicit consumption-set design replaces head discipline.

### Requirement: Immutable authority deployment audit
Every deployment used for acceptance evidence SHALL pass a deploy audit. The audit SHALL recompute the contract address and check the exact operation set, every key against a reproducible build, the initial state and the maintenance authority: committee `[]`, threshold ≥ 1, counter 0. The initial state SHALL be `Uninit(Π_P, netTag)` for a genesis deployment, or `Uninit(Π_P, netTag, A_pred)` for a migration successor, where `A_pred` equals the predecessor named in the signed migration digest. Deployments SHALL use a custom deploy path, because midnight-js installs a one-key committee by default. Tooling that requires a one-key committee SHALL NOT gate these deployments.

#### Scenario: Audited deployment
- **WHEN** a Moriarty contract is deployed to Preview through the custom path
- **THEN** the audit reproduces its address and keys and records an empty committee with threshold at least 1.

#### Scenario: Weak or altered deployment
- **WHEN** a deployment has an extra operation, a key mismatch, a non-`Uninit` initial state, a successor whose recorded predecessor differs from the migration digest, threshold 0 or a non-empty committee
- **THEN** the audit fails and no acceptance is claimed for that deployment.

### Requirement: Measured bounds frozen into the program digest
Final bounds SHALL come from the report's section 14.4 microbenchmarks on one pinned machine plus browser WASM measurements. The frozen set SHALL cover join fan-in, branch fan-out, certificates per call, segment length, effects, assets and obligations per step, public inputs per call, core step k, proving time and memory, and verifier work. Frozen bounds SHALL enter `bounds.json`, owned by MC01, and Π_P as a new program-digest version.

#### Scenario: Frozen profile
- **WHEN** every bound has a retained benchmark
- **THEN** the profile's Π_P binds all bounds and campaigns may rely on them.

#### Scenario: Unmeasured bound
- **WHEN** a bound lacks a retained benchmark or Π_P omits it
- **THEN** the bound is not frozen and no campaign may rely on it.
