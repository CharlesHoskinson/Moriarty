---
title: "GP architecture study (sherlock-gp-architecture-20261110)"
type: source
tags: [gp, architecture, pcd]
sources: [gp-architecture]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/research/sherlock-gp-architecture-20261110/architecture.md
source_url: local
---

# GP architecture study (sherlock-gp-architecture-20261110)

## Summary

Brief ingest of section 1 only (the recommended architecture); sections 2 to 6 (options, dissent, migration, experiments, proposed text) are not summarised here. Editor Fable, second designer and critic Astra. Nothing was built, run or proved; every row figure for the new relations is an ESTIMATE until its experiments run. The recommendation: a binary proof-carrying-data node over the existing KZG accumulation, with typed statements, a claim tree, a key registry, and committed memory in the VM.

## Key Claims

- Fact 1: three proof slots do not fit k=20. Measured model rows: self plus stand-in leaf 652,437; self plus self 730,243; self plus self plus leaf 1,078,147; self plus C6 digest leaf 741,708; decider 929,166.
- Fact 2: the in-circuit verifier is specialised to one constraint system and one k.
- Fact 3: ledger proofs use a Blake2b transcript; the gadget verifies Poseidon transcripts only.
- Layers: L0 base proofs (U universal leaf, X foreign circuits, A adapters), L1 lifts (R run lift, F foreign lift), L2 node N with two slots (SEG and SET rules), L3 closing proof C and decider D, L4 contract `SherlockClaims`.
- The decider's public values are `[reg_root, set_root, n_claims, shape_d]`.
- The RUN claim is `c = H(CLAIM, kind, id, in_d, out_d)` with `out_d = H(outcome_kind, value_d, n_steps)` (`architecture.md:98`).
- `shape_d = H(SHAPE, relation tag, slot bits, child shape_d values, local statement)` binds the proof graph, including segment bracketing.
- GP-1: height is range-checked below 2^64.
- Not met as written (§1.4): arity 2 plus new work in one k=20 node; foreign halo2 circuits outside the stdlib configuration; proofs already accepted on Midnight (ledger bytes); and "Capacity does not depend on the key" with fixed Merkle depths. Astra holds that the last is still a narrowing.

## Key Quotes

- "No evidence shows that a requirement is impossible on this stack, and none needs a different proof system on present evidence."

## Connections

- [[GeneralPurposeRecursion]], [[IvcNode]], [[RunDecider]].
- [[c6-outcome-ledger9-review]] reads lines 96 to 101 and 349 to 367 for D-3 and D-6.

## Contradictions

- `out_d` includes `n_steps` (`architecture.md:98`); D-3 of [[dissent-c6-section]] takes `n_steps` out of the claim and says this line needs the matching edit (task 4.20).
- `shape_d` binds segment bracketing and a "local statement"; D-6 is open on whether bracketing is public, and on whether the local statement may contain `pre_d` or `post_d` ([[c6-outcome-ledger9-review]]).
