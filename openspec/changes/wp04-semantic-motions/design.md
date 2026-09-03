# Design: WP04 semantic motions

## Context

The candidate Core preserves finite Marlowe-style contracts. Proposed DeFi
extensions can weaken determinism, conservation, authorization, or boundedness.

## Inputs

- WP03 semantic and proof-premise matrix.
- DeFiFormal demand and residue evidence.
- E00 backend restrictions and disclosure manifest.

## Outputs

- One decision record per semantic motion.
- Operational rules and typed failure results.
- Resource-bound equations and privacy disclosures.
- A disposition of accept, revise, defer, outside-core, or reject.

## Decisions

Prefer elaboration and libraries over Core growth. Accept a Core construct only
when it has demand evidence, finite semantics, and a credible proof path.

## Failure Handling

An unresolved motion blocks the Core freeze. If two semantics remain viable,
defer the feature or keep it outside the Core.

## Verification

Use transition tables, counterexamples, and finite model tests. Record race,
front-running, timeout, authorization, and resource consequences.
