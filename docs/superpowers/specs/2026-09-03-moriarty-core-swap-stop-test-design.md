# Moriarty Core atomic-swap stop-test design

Date: 2026-09-03 UTC  
Status: approved by the armed 90-day research sprint  
Experiment: E00

## Decision purpose

Test whether a small finite Moriarty Core can generate reviewable Compact without
hidden unbounded behavior. Test whether an independent backend model preserves
the Core observations for at least 1,000 deterministic traces.

Continue language work only when all E00 acceptance predicates pass. Choose the
audited Compact-library fallback if the lowering needs an unbounded container or
an unconstrained witness.

## Scope

The experiment implements one two-party atomic token swap. Alice deposits token
A. Bob deposits token B. Bob then submits a bounded public choice in the range
`0..1`. Choice `1` transfers both assets atomically. Choice `0` refunds both
assets. One absolute ledger deadline refunds all deposited assets.

The Core subset contains these constructs:

- `Close`
- `Pay`
- `If`
- `When`
- `Deposit`
- bounded `Choice`
- constant values
- choice equality

The experiment excludes modules, packages, loops, mandates, oracles, shielded
asset flows, external contract calls, and direct ZKIR generation.

## Architecture

`moriarty/core.py` defines immutable Core syntax, typed identifiers, state, and
transaction results. Its interpreter reduces internal constructs to quiescence
before it accepts one external input.

`moriarty/swap.py` constructs the canonical atomic-swap Core term from an
immutable parameter record.

`moriarty/compact.py` accepts only the exact canonical swap shape. It rejects any
other Core tree. It emits fixed-ledger-state Compact and a transition manifest.
The generated contract uses no collection and no loop.

`moriarty/backend.py` executes the transition manifest without importing the Core
interpreter. This separation detects disagreements between the Core semantics
and the generated state machine.

`moriarty/certificate.py` generates deterministic traces. It compares accepted
or rejected transitions, phase, accounts, payments, choices, and warnings. It
writes hashes, bounds, coverage, and disagreement counts to a certificate.

## Core semantics

An account key consists of one party and one token. Every account quantity is a
non-negative integer. A deposit must match the expected party, account, token,
and quantity exactly.

`Pay` transfers the smaller of the requested quantity and the source balance.
It emits a typed partial-payment warning when the balance is insufficient.
`Close` refunds all remaining accounts in deterministic key order.

`When` accepts a matching action only before the deadline. At or after the
deadline, reduction selects the timeout continuation before it examines the
input. The interpreter rejects an input that does not match a quiescent case.

One transaction accepts at most one external input. It then reduces all internal
`If`, `Pay`, and `Close` work to quiescence. Therefore both successful swap
payments occur in one transaction result.

## Static bounds

The analyzer computes these structural maxima from the Core tree:

- maximum accepted external inputs on one path
- maximum internal reductions on one path
- maximum payments on one path
- maximum live accounts
- maximum absolute timeout
- syntax-node count

The generated manifest records the measured values. The compiler does not use a
run-time list, map, recursion, or dynamic iteration to realize these bounds.

## Compact lowering

The generated Compact contract has fixed sealed parameters and one finite phase
enum. It exports entry points for Alice's deposit, Bob's deposit, Bob's decision,
and deadline expiry.

Authority witnesses are private. Each witness hash must equal its corresponding
sealed authority hash before any state or value effect. The public decision is a
bounded `Uint<0..2>` value.

The compiler emits explicit `receiveUnshielded` and `sendUnshielded` effects.
The settlement entry point performs both sends before it enters the terminal
phase. The expiry entry point refunds only the assets held in the current phase.

The artifact manifest records every witness, disclosure, public parameter,
entry-point effect, toolchain version, source hash, Compact hash, and expected
ZKIR artifact.

## Differential traces

The trace generator uses a fixed seed and a documented input alphabet. It covers
valid paths, each phase timeout, deadline boundaries, wrong parties, wrong
tokens, wrong quantities, out-of-range choices, early expiry, and calls after a
terminal phase.

A trace is materially distinct when its ordered input payload sequence differs.
The certificate reports semantic coverage separately. Semantic coverage includes
every accepted transition, every rejection class, every terminal phase, and each
deadline boundary.

The initial stop test requires at least 1,000 distinct traces and zero observable
divergence. The release gate remains 10,000 traces per canonical application.

## Security and assurance claims

The experiment can establish finite reference behavior, generated-source
boundedness, toolchain acceptance, and differential agreement with the manifest
machine. It cannot establish compiler correctness, ZKIR correctness, proof
soundness, network deployment, privacy against traffic analysis, or economic
safety.

The certificate is translation-validation evidence at status S3. It is not a
mechanized proof. Any failing trace blocks expansion of the Core until the cause
is resolved.

## Acceptance predicates

1. The Core analyzer returns finite bounds for the canonical swap.
2. All account balances remain non-negative.
3. Deposits equal payments plus final account balances for each token.
4. Successful settlement emits both payments in one transaction result.
5. At least 1,000 materially distinct traces produce zero divergence.
6. Generated Compact contains no collection, loop, recursion, or
   cross-contract call.
7. Every witness is constrained before its first state or value effect.
8. The pinned Compact compiler emits TypeScript, metadata, and ZKIR 3 artifacts.
9. The ZKIR mock compiler accepts each generated proving circuit.
10. The manifest and certificate contain canonical SHA-256 artifact hashes.

## Failure disposition

Choose the Compact-library fallback when predicates 6 or 7 fail by necessity.
Stop the vertical slice when predicates 2, 3, or 4 fail. Repair the experimental
compiler when predicates 1, 5, 8, 9, or 10 fail for an implementation defect.
