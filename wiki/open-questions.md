---
id: research.open-questions
type: question
title: Open questions
status: active
updated_at: 2026-09-03T14:43:01Z
sources:
  - SRC-0003
  - SRC-0005
  - SRC-0007
  - SRC-0019
  - SRC-0036
  - SRC-0038
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

## ZKIR semantics in K (added 2026-09-03)

- Which ZKIR v3 surface is the deployed one: the 34-instruction `92e8bdd3` surface that arc-zkir mechanizes, or the 42-instruction standalone `2ffe2d1` surface? Resolution route: inspect the Midnight node and ledger release that mainnet runs and the ZKIR version tag inside shipped `.zkir` artifacts.
- Can the K definition be checked against the arc-zkir Agda model rather than only against the Rust crate, for example by generating the same witness and constraint traces for the precompile programs and comparing them with the Agda `synth` and preprocess functions? This needs the Agda toolchain (`nix run .#agda`) which has not been run locally. Status 2026-09-05: the whole v3 development type-checks with its Nix toolchain (`evidence/arc-zkir-agda-typecheck-2026-09-05.txt`), but it is parametric over an `Assumptions` record (field, curve and hash primitives) with no concrete instance, so it cannot execute programs; a concrete run needs an Agda implementation of the trust base, which does not exist in the repository.
- Does Midnight's `k-rust` accept a definition that uses `MInt`, `Bytes`, and hooked `Int` operations, so that the ZKIR semantics can run inside the Midnight TypeScript tooling without canonical K? Not yet tested; `k-rust` is S4. Answered in part 2026-09-05: k-rust 0.4.0 parses the definition (`krust kast` on `ZKIR-SYNTAX` terms works) and runs the K tutorial lesson, but `krust kcompile` of the full ZKIR definition did not finish within 25 minutes (`evidence/k-rust-compatibility-2026-09-05.txt`); whether the in-process backend supports the `Bytes` and `^%Int` hooks remains untested because compilation never completed.
- What does `compactc --feature-zkir-v3` emit for the Moriarty escrow and swap circuits, and do those programs stay inside the 34-instruction surface? The E00 experiment produced ZKIR 3 artifacts that can be re-read for this. Partly answered 2026-09-05: the seven escrow and swap artifacts in `experiments/` are version 3.0 and use only nine of the 34 instructions (`assert`, `bytes32_into_low_high`, `cond_select`, `constrain_bits`, `constrain_to_boolean`, `impact`, `persistent_hash`, `private_input`, `public_input`, `test_eq`); they parse and type-check in the base definition and agree with the Rust crate on every preimage tried, but no run reaches a successful witness with generated inputs because their assertions need a consistent transaction context (`evidence/zkir-k-differential-92e8bdd3-2026-09-05.txt`).
