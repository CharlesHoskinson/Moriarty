## ADDED Requirements

### Requirement: Bounded native certificates
Certificate relations SHALL be Poseidon zk-stdlib relations verified through `VerifyProof` and `InnerProof` with guard constant 1, with each accumulator pairing checked by ledger `well_formed` on `ledger-10`. Contract-call proofs SHALL NOT be used as inner proofs. The outer relation SHALL constrain `vk_repr` and the state decider. Certificate campaigns SHALL NOT be admitted before a reviewed resource amendment, because the measured outer circuits need k = 18 for one level and k = 19 for two levels, above the Charter's k ≤ 17 ceiling.

#### Scenario: Accepted certificate
- **WHEN** a certificate-bearing call runs on a pinned `ledger-10` devnet under an approved resource amendment
- **THEN** the ledger accepts the call and its accumulator pairing checks.

#### Scenario: Unsound certificate
- **WHEN** a guard is free or mismatched, `vk_repr` is substituted, an inner instance is unbound, or an inner proof is tampered or belongs to another key
- **THEN** the ledger rejects the call or the Moriarty lint rejects the build, and an outer proof over an invalid inner proof is not evidence.

### Requirement: Off-ledger segment certificate
Experiment E5 SHALL produce segment certificates over the Moriarty step at 1, 10 and 100 steps, with segment length at most 16 until benchmarked. Retained bytes SHALL be verified in a fresh process and imported through a certificate entry point.

#### Scenario: Verified segment
- **WHEN** a 10-step segment certificate is retained
- **THEN** a fresh process verifies it and the certificate entry point imports its final state.

#### Scenario: Substituted segment evidence
- **WHEN** evidence offers a host hash chain, a MockProver run, a nonrecursive re-proof or an altered segment
- **THEN** it is not segment evidence and the requirement stays open.

### Requirement: Recursion dependency tracking and stop
Certificate campaigns SHALL NOT dispatch until each tracker item they need holds and is pinned: pull request 738 merged and `ledger-10` released, accumulator fee accounting, k ≥ 18 parameters served, released formats pinned and a reviewed resource amendment. The tracker SHALL also carry ledger 9 on Preview for `mandatory` and `composition`. If recursion slips, certificate stages SHALL record `blocked`, the core requirements and the Preview gate SHALL stand, and no scope SHALL be dropped without new user direction.

#### Scenario: Tracker satisfied
- **WHEN** every tracker item for a certificate stage holds and is pinned
- **THEN** the stage may seek campaign admission.

#### Scenario: Unsatisfied tracker item
- **WHEN** a dispatch is attempted while a required tracker item is open
- **THEN** dispatch is refused and the stage records `blocked`.
