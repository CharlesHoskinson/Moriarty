---
title: "Draft: area 3 recursion and system architecture (Astra)"
type: source
tags: [draft, final-design, recursion, gp-2, gp-3, gp-4]
sources: [astra-area3-recursion-draft]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/drafts/astra-area3.md
source_url: local
---

# Draft: area 3 recursion and system architecture (Astra)

## Summary

Draft, not adopted. Astra's area 3 draft for run `sherlock-final-design-20261111` proposes the modules NODE INTEGRATION, DECIDER, CONTINUATION, CLAIM CONSUMPTION, FOREIGN ADMISSION and GRAPH COMPOSITION, a whole-leaf differential harness, a build order and a module kit template, for a later merge. Interfaces are proposed; nothing built or run. It was written before [[runlength-consensus]] existed and cites the lane files. The first integrated backend is dense Merkle memory with Poseidon boundaries; the C6 statement `[prog_d, input_d, outcome_d, n_steps]` and `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)` stay unchanged.

## Key Claims

- Segment and claim separation: a `Segment` proves execution on `[h_lo, h_hi)`; a `RunReceipt` proves a terminal run from height 0. `SHK_CONT_V1`, `SHK_RUN` and foreign-claim domains are disjoint.
- NODE: one or two child slots (`Off`, `Node`, `RunLift`, `ForeignLift`), adjacent segments equal in program, input, dependencies, schema, backend context and boundary; `left.h_hi == right.h_lo`; `0 <= h_lo < h_hi < 2^64`. Measured anchor: the linear spike-key node at 823,466 rows; gate 950,000.
- DECIDER: run and collection entry points with separate schemas; `Running` cannot settle. The measured decider is 929,166 ordinary rows with 1,032,467 table rows, so the ordinary-row gate does not establish domain fit. ESTIMATE 1,000 to 5,000 extra rows.
- CONTINUATION, in this draft's terms: Tier 1 is a `PhysicalCheckpoint` (an authenticated prefix with the boundary, pause and resume without executing an event, fuel and `original_input_d` unchanged); Tier 2 is a `LogicalCheckpoint` with a `Relocation` proof that old and new representations decode to the same logical state. Fuel exhaustion is terminal and must not trigger a checkpoint. A Horner fingerprint is not a portable checkpoint digest. See [[Continuations]] for how these tiers map to the consensus terms.
- CLAIM CONSUMPTION: only sealed claims enter; a `Rejected` or `Exhausted` claim is consumed as a claim, never relabelled as a value.
- FOREIGN ADMISSION: a registry entry binds the full verification interpretation (proof system, curve, SRS, key, transcript, input schema, adapter). Native adapters do not cover full GP-3; a general verifier path is required.
- GRAPH COMPOSITION: two predecessors plus inline owner work in one node is required for GP-4; "Failure of that experiment leaves GP-4 incomplete. It does not authorize removing 'plus new work.'" Claim count: "Do not carry an authoritative additive occurrence count merely to support membership."
- 15 spec conflicts listed, among them `n_steps` in the GP study's `out_d`, the undefined "local statement" in `shape_d`, M3 leaf-close allocation, the arena dissent, dependency admission against the empty `deps_d` at Init, and exhaust kind 7.

## Key Quotes

- "The full target remains GP-1 through GP-4."
- "Two nodes do not satisfy the literal one-node requirement."

## Connections

- [[Continuations]], [[GeneralPurposeRecursion]], [[IvcNode]], [[RunDecider]], [[C6OutcomeOnlyPublication]], [[gp-architecture]].
- Sister drafts: [[astra-area1-processor-draft]], [[astra-area2-memory-draft]].

## Contradictions

- "Tier" means different things: here Tier 1 is the physical checkpoint and Tier 2 logical relocation; in [[runlength-consensus]] Tier 1 is intra-run segment recursion and Tier 2 cross-session epochs with modes 1 and 2.
- Against [[gp-architecture]]: `n_steps` in `out_d` (line 99), the "local statement" in `shape_d` (line 101) and the occurrence count (line 100). Its claim-count advice agrees with audit blocker 2 and lead R-5.
- Keeps D-6 open: recommends binding execution intervals and bracketing in `shape_d`, with disclosure an open decision ([[dissent-c6-section]], [[OpenOwnerQuestions]]).
