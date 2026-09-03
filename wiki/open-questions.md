---
id: research.open-questions
type: question
title: Open questions
status: active
updated_at: 2026-09-03T05:39:34Z
sources:
  - SRC-0003
  - SRC-0005
  - SRC-0007
  - SRC-0019
---

<!-- markdownlint-disable MD025 -->

# Open questions

The controlling assignment defines the initial research backlog. Each question
will move to a topic page when investigation starts and will remain linked here
until resolved or explicitly deferred.

- Can the Moriarty escrow and atomic swap generate proving and verifier keys?
  Can both run a full proof with pinned `MIDNIGHT_PP` assets?
- What are proof time, verifier cost, transaction size, state size, and fees for
  the 15-contract suite on the current public Midnight network?
- Does a universal bounded Core interpreter have acceptable key size and proving
  latency compared with ahead-of-time specialization?
- Which formal environment has at least two committed maintainers for the Core
  proof program: Isabelle, Agda, Lean, or K plus an independent prover?
- What exact Midnight credential binds a circuit witness to a wallet/user, and
  what recovery and rotation semantics should Moriarty expose?
- What public and private audit evidence must be generated so that a hidden
  financial agreement remains reviewable by counterparties and regulators?
- Can fixed-depth Merkle continuations obtain operational availability without
  introducing a trusted Runtime operator?
- Which two pilot teams will commit to non-toy value-bearing use cases and share
  integration/time/cost evidence?
- Which V1 contracts are important enough to justify a trace-equivalent
  Marlowe-to-Moriarty translator instead of a one-way design migration tool?
- Which source-level visibility types prevent a public Moriarty choice from
  arriving as a private Compact argument that needs an inferred disclosure?
