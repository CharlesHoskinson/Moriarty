---
title: "Audit of the run-length consensus (Fable 5.2)"
type: source
tags: [run-length, gp-1, memory, continuations, audit]
sources: [runlength-consensus-audit, runlength-consensus]
last_updated: 2026-11-10
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-runlength-consensus-20261111/AUDIT.md
source_url: local
---

# Audit of the run-length consensus (Fable 5.2)

## Summary

Fable 5.2 audit of [[runlength-consensus]], read-only at `9ffc2fe76` (no build, test, proof or model run). Verdict: **adopt with changes**, with 3 blockers, 10 major and 7 minor findings. The answer on 2^64 is correct and the memory direction is sound, but the report cannot be adopted as written: its GP-001 text narrows GP-1 and GP-4, it chooses no single memory design, its continuation mode 2 is not specified well enough, and several "5/5 unanimous" statements do not match the lane files. Section 5 gives corrected GP-1 and GP-001 text that keeps every current sentence and only appends. The lead adopted the audit's corrections in [[final-design-decisions]] (R-1 to R-10).

## Key Claims

Blockers:
1. GP-001 narrows "every program, every input" to "every admitted program ... every valid input" (Lane E only). Restore the original words.
2. The claim counter bound narrows GP-4: `n_claims` counts occurrences, so a DAG where each level feeds two consumers doubles the count per level and reaches 2^64 at depth 64 with about 128 proofs. Remove the counter from GP-1 and GP-001; record an open item against `architecture.md:100` (count distinct claims, or no count).
3. The memory counter bound removes "bounded only by the height field". With `TOP = 61` the space is `2^64 - 1` cells and a step can append up to 3 cells, so a run of fewer than 2^64 steps can exhaust it. Keep line 14 verbatim; size each address counter so the height field binds first: TOP >= 63 (2^66 cells; ESTIMATE +280 rows per dual walk at 70 rows per permutation), plus the input cells. Alternative: an explicit owner decision that accepts a cell bound.

Major (selected):
- 4. No single memory design is chosen. State "tail-buffered dense memory at fixed depth is the direction; the arena count is open". Decide by a mechanised contiguity invariant and measured leaf rows; "until (a) passes, d2 is the default". List the Init fields added and removed.
- 5. Continuation mode 2 is not ready (slices of m > 8 cells, addresses differ after resume, no proof carries the checkpoint work, no mode field in `cont_claim`, `quantum_steps` bound nowhere), and "via GP-002" contradicts GP-002. For mode 1 the audit found no replay attack. Adopt tier 1 and mode 1 only; move mode 2 to open research.
- 6. No per-step maximum of reads, appends and flushes; add a per-rule table from the 108 rules and require it to fit the chip heights at S = 1.
- 7. "5/5" overstates the lane files; the GP text is the orchestrator's composite; D-Q1.1 is 2 (C, E) against 3 (A, B, D).
- 8. The flush design is sound by a hand check, on conditions the report does not state: canonical `tail_d`; tail opened or passed through; both lengths decomposed as `8q + r`; kind-tag checks on reads in d1; the W_flush = 1 argument is per leaf; domain tags on every hash; the stack hash chains are not D7 (D7 links frames by an address in memory). New chip models are a condition of adoption.
- 9. Slot-0 zeroing does not replace write-before-read; the obligation is "no rule reads `v0..v2` before it writes them in the same step".
- 10. D3 does not falsify SRC-023 or the Lean theorem; it makes the `det` premise impossible to discharge at digest level.
- 11. "+4..+12 node rows" is the E2 fit for leaf k alone; the node also grows with columns and lookups. The decider gate is 1,000,000, not 1,048,570.
- 12. Throughput figures assume 16 steps per leaf and are lower bounds; mark them ESTIMATE; keep time caps out of GP-001.
- 13. D-1 is closed without the required measurement; the report's design is a third form (contiguous slices in dense memory). The lead or the owner changes D-1 status, not the report.

Minor: 14 to 20 (a nonexistent local path for `verifier_gadget.rs`, which says `num_challenges = 1`; an unsourced 73-row figure; line references; "`2^64 - 1`" cells; nonexistent rows A33b and A34c and the `6,432 B` constant; the CVE-2026-62333 date is 2026-10-30 per OSV; `k_leaf = 16..17` against the gate `k_leaf <= 16`).

Section 5, corrected text: line 14 of `general-purpose.md` keeps "The run length is bounded only by the height field (below 2^64)" and appends an explanation (the 64-bit step counter that SRC-004 range-checks; not a circuit size; no leaf count or memory address counter sets a lower limit; a step that allocates no memory consumes no address; proving can stop and resume under the same key set; tested lengths are evidence under SAS-001). The acceptance line and GP-001 get appended items only. Step thresholds, time caps and gate names stay out of GP-1 and GP-001.

## Key Quotes

- "The rule used: keep every current sentence, and add. Nothing that the current text grants is removed, so nothing is narrowed."
- "2^64 is the range of the step counter that SRC-004 checks ... it is not a circuit size, and no circuit grows with the run length."
- "Until (a) passes, d2 is the default, because it needs no global invariant and no address refinement map." (Finding 4; not taken into [[final-design-decisions]], which keeps the arena count open.)

## Connections

- Target: [[runlength-consensus]]. Lead decisions: [[final-design-decisions]].
- Concepts: [[RunLengthCeiling]], [[GeneralPurposeRecursion]], [[DenseTailMemory]], [[Continuations]], [[OpenOwnerQuestions]].
- D-1 and D3: [[dissent-c6-section]], [[ChipLeaf]]. Claim counter: [[gp-architecture]].

## Contradictions

- Contradicts [[runlength-consensus]] on the GP-001 wording, the claim and memory counter bounds, mode 2 continuations, slot-0 zeroing, D-1 and the node and decider row claims. The lead adopted the audit's correction on these ([[final-design-decisions]] R-2, R-4 to R-10).
- Also contradicts the report on the "5/5" counts (finding 7) and on D3 "falsifying" SRC-023 (finding 10); no separate lead decision is recorded for these.
- Against [[gp-architecture]]: the occurrence-count `n_claims` at `architecture.md:100` must not be range-checked or carry soundness.
- Against [[final-design-decisions]] on one point: the audit makes four arenas (d2) the default until the contiguity invariant passes; R-6 keeps the arena count open and decides it in C4.
