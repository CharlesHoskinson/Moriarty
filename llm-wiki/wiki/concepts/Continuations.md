---
title: "Continuations (stop and resume of one run)"
type: concept
tags: [continuations, gp-2, gp-4, c6, lead-decision]
sources: [runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area2-memory-draft, astra-area3-recursion-draft]
last_updated: 2026-11-10
---

# Continuations (stop and resume of one run)

How a long run is proved in pieces, and across proving sessions, under one key set and with one public statement. Status: **lead decision R-7, pending owner review**: tier 1 (segment tree) plus the retained-memory checkpoint mode only ([[final-design-decisions]]). Nothing is built.

## Terms

"Tier" means different things in two sources:

- In [[runlength-consensus]]: Tier 1 is intra-run segment recursion (leaf, lift, node, decider); Tier 2 is cross-session epochs under a tag `SHK_CONT_V1`, with checkpoint mode 1 (retained-arena `canon_state_d(h)`) and mode 2 (address-free `sherlock-ckpt/1` with lazy `UNFOLD`).
- In [[astra-area3-recursion-draft]]: Tier 1 is a `PhysicalCheckpoint` (an authenticated prefix plus boundary, with memory retained); Tier 2 is a `LogicalCheckpoint` with a relocation proof. The draft's Tier 1 corresponds to the consensus mode 1; its Tier 2 corresponds to the address-free mode 2.

## What the lead adopted

- Tier 1 segment recursion and mode 1 only. The audit found no replay attack on mode 1: an epoch proof binds the key, `prog_d`, the unchanged `input_d`, both heights and both state digests; a chain seals only from `h_lo = 0`, where the first leaf proves Init; the only replay key is `claim_d`, as for an uninterrupted run ([[runlength-consensus-audit]] finding 5).
- The audit asks whether mode 1 needs a new tag at all; a stored SEG proof may be enough.
- Continuations compose under GP-4 and the checkpoint domain tag, not "via GP-002": GP-002 says a segment statement or a state digest SHALL NOT be accepted as a claim of another run ([[final-design-decisions]] R-7, [[runlength-consensus-audit]]).
- The cut never uses `fuel`; fuel exhaustion stays a terminal outcome ([[runlength-consensus]], [[astra-area3-recursion-draft]]). The audit adds that `quantum_steps` is bound nowhere and must be a prover choice, like a leaf boundary; a keyed constant would put a run-length parameter into the key.
- The sealed output is one RUN proof with `[prog_d, input_d, outcome_d, n_steps]`, no state digests or epoch counts on chain ([[C6OutcomeOnlyPublication]], [[runlength-consensus]]).

## Open research

- Mode 2 (address-free checkpoint with lazy `UNFOLD`) is not defined well enough: large slices do not fit the 15-row window, addresses differ after resume, no proof carries the checkpoint work, `cont_claim` has no mode field. The consensus claim that mode 2 "closes SRC-021 [OPEN]" is removed ([[runlength-consensus-audit]] finding 5). So the Init mechanism question stays open ([[OpenOwnerQuestions]]).
- With a run-wide LogUp backend, a prefix keeps an unresolved memory obligation; an epoch bridge is an open construction; a Horner fingerprint is not a portable checkpoint digest ([[astra-area2-memory-draft]], [[astra-area3-recursion-draft]]).

## Contradictions

- [[runlength-consensus]] presents mode 2 as agreed by 5 of 5 and tier 2 as GP-2 composition; [[runlength-consensus-audit]] contradicts both. The lead adopted the audit's correction (R-7).

See also [[DenseTailMemory]], [[RunLengthCeiling]], [[GeneralPurposeRecursion]], [[RunDecider]].
