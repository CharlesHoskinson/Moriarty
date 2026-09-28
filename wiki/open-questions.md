---
id: research.open-questions
type: question
title: Open questions
status: active
updated_at: 2026-09-28T15:38:14Z
sources:
  - SRC-0111
  - SRC-0112
  - SRC-0113
  - SRC-0003
  - SRC-0005
  - SRC-0007
  - SRC-0019
  - SRC-0036
  - SRC-0038
created: 2026-09-02
updated: 2026-09-28
tags:
  - moriarty
  - research
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

## Midnight-native PCD (added 2026-09-11)

From the [[wiki/decisions/pcd-midnight-native-architecture|PCD decision]]; experiments E1–E5 are in the [PCD roadmap](attachments/historical-evidence/openspec/PCD-ROADMAP-2026-09-11.md).

- Does head read-then-write give exactly-once consumption on Preview, including fallible sections and pool reordering (E1)?
- Does the fused step relation fit k ≤ 17, 600 s and 8 GiB on the proof server (E2)?
- Will pull request 738's formats and hazards (fees, guards, collapsed decider) be fixed before `ledger-10` release candidate (E3)?
- Do the published k ≥ 18 prover parameters share the embedded verifier parameters' setup, and are they served to provers?
- Does the cross-contract `Reclaim` rule keep an unclaimed `Release` recoverable exactly once (E4)?
- Is an off-ledger IVC segment certificate worth about 20 s per step and a 411 MB proving key (E5)?
- Can join summaries express every Moriarty merge rule without full branch history?

## U0 unified proposal (added 2026-09-28)

From the [[wiki/sessions/u0-unified-proposal-2026-09-28|nine-reviewer U0 study]]. The first six are owner decisions that block Phase B of its work plan.

- Do the five remainder classes (`none`, `conserved-split`, `charged-increment`, `sub-unit-residual`, `protocol-reserve`) replace D2's single reserve beneficiary, and is liability evolution fixed as opening + accrual − discharge = closing with non-negative components?
- Does a charged-minus-credited rounding spread count against `signedIntent.feeCap` and `grossDebitCap`? Must be decided before U3, where "at most 1 A fee" could otherwise become tight.
- Is signed-intent authentication in-circuit (canonical intent digest as a public input), with the circuit-commitment-plus-ledger-signature route admitted only on a passing negative control? This is trust premise TP07.
- Is S0 approved as the named first slice: program A a funded repay under AccrualFirst with exact conversion, program B a transfer with a literal fee line?
- Should the T3 tuple harness be run on historical profile H now, as harness validation carrying no target claim, to close CLM-0958 ("proved, not verified")?
- Is the Aeon refinement package committed under re-scoped text, or left uncommitted?
- Does any released or installable compiler, ledger and proof server emit and accept ZKIR v3 end to end? Until one exists, U0-T stays blocked. Resolution route: install from upstream release artifacts and read the IR header from the emitted bytes.
- Was the only Moriarty prove at k=14 and never verified, and does the pinned `accrue` key never produce a transaction proof? Single-reviewer finding; verify before acting.
- Can one `Conversion.rounding` field serve origination (floor) and repayment (ceil) at the same call site, and what does the profile record when it cannot? Single-reviewer finding.
- What UInt128 bound must a K/TypeScript correspondence state, given that `lcRepay` uses unbounded `Int`? Single-reviewer finding.
- Does the ledger enforce any stage-relation leaf intrinsically? The enforcement search roots exclude the ledger itself, so ledger-intrinsic loci are an unexamined candidate.

## Intent language and programmable escrow (added 2026-09-28)

The owner placed conditional settlement with programmable escrow in the intent language ([[wiki/moriarty-architecture|architecture decision]]). That placement settles the carrier and opens these:

- Which milestone owns the intent condition grammar? S6 in the U0 plan writes a signed-intent abstract syntax during the freeze phase, but no milestone is assigned the condition language itself.
- Is the intent condition language the same small predicate language proposed for executable judgment clauses, or a second one? Recommendation: the same, to avoid two predicate surfaces, two evaluators and two correspondence arguments.
- What is the canonical, bounded encoding of a condition tree, given that the in-circuit intent digest must cover it and U1 must measure the cost?
- How are escrowed funds, their release condition and their refund path represented in the stage relation, which today carries only effects and a failure policy?
- Can a condition reference an observed value at all before the canonical stage statement binds an observation's value and observed-at time? On current evidence, no — which puts the observation gap on the escrow critical path.
- Does a signed condition bind the recipient, who does not sign the stage? The design requires consent from a party made liable and says recording a request imposes no duty on an unconsenting recipient.
