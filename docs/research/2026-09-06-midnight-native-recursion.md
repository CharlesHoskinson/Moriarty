# Midnight recursion review

## Decision

Midnight has a concrete native recursive-proof implementation in its Rust ZK
stack. The pinned `midnight-zk` main branch at
`695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` contains in-circuit PLONK
verification, proof accumulation, incremental verifiable computation (IVC), and
a constant-size final verifier. This changes the Moriarty plan: use this as the
first native PCD candidate and test its deployment boundary before designing a
foreign recursion layer.

This does **not** mean a Compact contract can directly verify a prior proof, or
that the deployed ledger accepts an IVC proof today. The named aggregation API was not found by lexical search in the pinned public
application stack; this does not exclude indirect or differently named integration. The evidence supports a
native Rust feasibility candidate, not an application-ready recursive verifier.

## Three distinct meanings of recursion

1. **Midnight-native Halo2/PLONK recursion — implemented, source-inspected, not
   reproduced here.** `midnight-zk/circuits/README.md` explicitly lists
   “in-circuit verification of PLONK proofs (a.k.a. recursion).” The
   `midnight-aggregation` crate (`aggregation/Cargo.toml`, version `0.1.2`) exports
   `ivc` and `multi_circuit_aggregator`. In `aggregation/src/ivc/mod.rs`, the
   public traits `IvcState`, `IvcIO`, and `IvcTransition` define an application
   state, its binding public-input representation, genesis, an application
   `decider`, and matching native/in-circuit transitions. `IvcProver::prove_step`
   folds another transition into the running proof. `IvcVerifier::verify` checks
   the canonical VK representation, the application decider, the proof
   accumulator plus instance accumulator, and the final KZG pairing invariant.
   The module documents proof size and verification time as constant in the
   step count. `aggregation/examples/ivc.rs` demonstrates setup, three proving
   steps, and verification after each step. These are repository source facts
   at the pinned commit, not results reproduced in this review.

2. **Compact language recursion — deliberately rejected, a separate issue.**
   The pinned Compact reference and compiler at
   `11e7ec5abeecb99297c4faa74d30ef9adc7b51f3` state that Compact circuits and
   structure types cannot recurse and that cyclic cross-contract call graphs
   are undefined/rejected. The compiler has a named
   `reject-recursive-circuits` pass. These restrictions ensure finite circuit
   elaboration. They do not imply that a generated circuit or a lower Rust
   circuit cannot contain a bounded in-circuit verifier for another proof.

3. **Application-facing recursive verifier — no pinned public integration
   found.** At the inspected commits, lexical search for `midnight-aggregation`,
   `IvcProver`, `IvcVerifier`, and `multi_circuit_aggregator` across
   `midnight-ledger`, `midnight-zkir`, `midnight-js`, `midnight-sdk`, and Compact
   produced zero matching files. The ledger's
   `transient-crypto/src/proofs.rs` exposes ordinary `VerifierKey::verify` and
   batch verification for supplied proofs/statements; it depends on
   `midnight-proofs` and `midnight-circuits`, not `midnight-aggregation`.
   Midnight JS exposes proof providers that turn an unproven transaction into a
   proven transaction, but the inspected public APIs do not expose IVC state,
   an aggregation verifier, accumulator, decider, or a method to submit an
   aggregated proof as a contract-call proof. This is a bounded repository
   observation, not a universal claim about private or future systems.

## Lifecycle and compatibility qualifications

- The separate pinned `midnightntwrk/halo2` fork is commit
  `92d9356722f4e7943bec2dd5d2dcc5ebbedd4ff1` on `dev`. It provides verifier and
  accumulator primitives, but the current `midnight-zk` README says its proof
  implementation has diverged from upstream Halo2 into a standalone Midnight
  implementation. The relevant integration target is therefore
  `midnight-zk`, not the mere existence of the older Halo2 fork.
- The `midnight-zk` aggregation changelog places IVC and multi-circuit
  aggregation under **Unreleased**. The inspected main commit is not contained
  by a local release tag. `aggregation/Cargo.toml` uses newer
  `midnight-proofs 0.8.0` and `midnight-circuits 7.0.0`, while the pinned ledger
  uses `midnight-proofs ^0.7.0` and `midnight-circuits ^6.2.0`. That version gap
  is direct evidence that drop-in ledger compatibility must be tested.
- The generic IVC framework requires Moriarty to implement both the native
  transition and its in-circuit counterpart, a computationally binding state
  encoding, constrained genesis, and an application decider. Its existence does
  not establish Moriarty semantic correctness, compiler correspondence,
  authorization binding, multi-parent PCD composition, or ledger
  double-consumption protection.

## Smallest feasibility test now required

Specify one tiny Moriarty transition, such as one bounded ACTUS due update or AMM reserve update with a domain,
program digest, predecessor commitment, effect commitment, and step counter in
the IVC state. Implement it directly against the pinned `midnight-aggregation`
Rust traits. Generate genesis plus two steps; verify the final proof; then show
rejection for (a) altered final public state, (b) wrong program/domain digest,
(c) wrong VK, and (d) a broken predecessor link. Measure proof size and verifier
time only for this case.

Then test the actual boundary separately: determine whether that final proof and
VK can be represented by the pinned ledger's accepted proof/statement types and
submitted through the current transaction path. If not, record the smallest
adapter or ledger change needed. A direct Rust IVC success is enough to select
the cryptographic candidate; it is not enough to claim Compact or deployed
Midnight integration.

No proof/build run was performed in this bounded review.

## Exact source locations

- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/mod.rs`
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/prover.rs`
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/verifier.rs`
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/src/ivc/setup.rs`
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/examples/ivc.rs`
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/aggregation/examples/multi_circuit_aggregation.rs`
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-zk/circuits/src/verifier/`
- `/home/charl/Moriarty/repos/midnightntwrk/midnight-ledger/transient-crypto/src/proofs.rs`
- `/home/charl/Moriarty/repos/LFDT-Minokawa/compact/compiler/analysis-passes/reject-recursive-circuits.ss`
- `/home/charl/Moriarty/repos/LFDT-Minokawa/compact/compiler/compact-reference-proto.mdx`
- Receipt: `raw/pcd-midnight-recursion-2026-09-06/repository-inspection.receipt.json`
