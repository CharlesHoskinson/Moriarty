---
title: "Continuations (stop and resume of one run)"
type: concept
tags: [continuations, gp-2, gp-4, c6, lead-decision]
sources: [runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area2-memory-draft, astra-area3-recursion-draft, sherlock-architecture, arch-audit-astra, arch-audit-response]
last_updated: 2026-11-11
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

## Architecture revision B2: CONTINUATION and HOST CHECKPOINT STORE

Proposed design, not built ([[sherlock-architecture]]):

- CONTINUATION is the scheduler: tier 1 carry stack of proven SEGs merged by NODE MERGE, one SEAL at the end; mode 1 resume has the first suffix leaf at `h_pre = h` with `pre_d` equal to the record's `post_d`. 0 added circuit rows in mode 1 (ESTIMATE). Mode 2 stays open research with the six gaps of the run-length audit. Profile B is not supported.
- The B1 text said the store was "the memory store as written by `MemBackend::persist`"; the audit found that `persist` writes nothing durable and no `StoreHandle` exists (finding 13) ([[arch-audit-astra]]).
- New kit HOST CHECKPOINT STORE: a `RunStore` (append log of every record, both stacks as full lists, register state, program image, input preimage) and `CheckpointV1` in the versioned codec `sherlock-ckpt/1` (header with manifest hash, identity, state, memory log, stacks, program and input, proof carry stack, record digest under `SHK_CKPT`). Restore validates in six steps (codec and manifest; prefix proofs verify and cover `[0, h)`; the log rebuilds `(len, root, tail_d)`; stacks rebuild their heads; `body_d` and `state_d` equal `post_d`; program and input hash to `prog_d` and `input_d`) before any proving ([[arch-audit-response]]).
- Two failure kinds are kept apart: a constraint failure (no proof exists) and a host integrity failure (refused at restore). "Soundness never depends on the store: a corrupted store can only stop the prover." A changed record that no later leaf reads is caught by restore validation, not by a constraint (`ckpt_tampered_unused_cell`).
- Whether the checkpoint tag is also needed in circuit is open (N-4).

## Contradictions

- [[runlength-consensus]] presents mode 2 as agreed by 5 of 5 and tier 2 as GP-2 composition; [[runlength-consensus-audit]] contradicts both. The lead adopted the audit's correction (R-7).

See also [[ModularArchitecture]], [[DenseTailMemory]], [[RunLengthCeiling]], [[GeneralPurposeRecursion]], [[RunDecider]].
