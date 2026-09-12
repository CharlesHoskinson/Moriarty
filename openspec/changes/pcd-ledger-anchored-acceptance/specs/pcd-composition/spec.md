## ADDED Requirements

### Requirement: Ledger-atomic split and join
`Split` and `Join` within one contract SHALL be ledger-atomic. Child head identities SHALL be computed in the circuit. Budgets SHALL be conserved, obligations partitioned and authority attenuated. The lifetime invariant SHALL hold per head: `revision + remaining = allocatedLifetime(head)`, fixed when the head is created. `Join` SHALL require distinct heads and compatible policies. Fan-in and fan-out SHALL be at most 2 until the bounds freeze sets them.

#### Scenario: Split then join
- **WHEN** head A splits into B and C and they later join into D
- **THEN** budgets, residual obligations and authority are conserved and each head's lifetime invariant holds.

#### Scenario: Composition attack
- **WHEN** a join repeats a head, mixes policies, restores budget, drops an obligation, or exceeds fan-in or fan-out
- **THEN** the call rejects.

### Requirement: Cross-contract release with reclaim
Cross-contract `Release`, `JoinFrom` and `Reclaim` SHALL use claimed calls from the Compact 0.33 toolchain on ledger 9. An unclaimed `Release` applies alone and SHALL leave the head `Releasing`. `Reclaim` SHALL return a `Releasing` head to `Live` only when it claims, in the same intent, a target-side call that reads the release commitment as absent from the target's imported set and marks it dead there. Experiment E4 SHALL confirm this rule.

#### Scenario: Recovered unclaimed release
- **WHEN** a `Release` lands without its `JoinFrom`
- **THEN** the head stays `Releasing` until a `Reclaim` with the claimed target-side absence call restores it.

#### Scenario: Unsafe release or reclaim
- **WHEN** a commitment is imported twice, a `Reclaim` lacks the claimed absence call, a `Reclaim` follows an import, or a `JoinFrom` claims a missing `Release`
- **THEN** the call rejects.

### Requirement: Recipient-keyed successor handoff
A successor SHALL prove from the head commitment and a recipient-encrypted opening, with no predecessor proof or witness. Per-party sub-state commitments SHALL be used where parties differ. Isolation evidence SHALL use distinct OS users or containers.

#### Scenario: Independent successor
- **WHEN** Bob receives only the opening addressed to him
- **THEN** Bob proves the next step and the audit shows Bob never held Alice's secret fields.

#### Scenario: Tampered opening
- **WHEN** an opening is altered or addressed to another recipient
- **THEN** proving fails and no step applies.

### Requirement: Composition operators under head discipline
Sequential composition SHALL map to `Step`, disjoint parallel composition to `Split`, and atomic synchronization to same-contract `Join` or a claimed cross-contract call. Shared-state interleaving, asynchronous messaging and Pending SHALL receive a reviewed rule under head discipline before any SP10 campaign advertises them.

#### Scenario: Mapped operators
- **WHEN** a composition uses sequential, disjoint parallel or atomic synchronization
- **THEN** it runs through the mapped entry points under the same acceptance lineage.

#### Scenario: Operator without a rule
- **WHEN** an operator has no accepted rule
- **THEN** it is unavailable, is not advertised, and MC06 stays open.
