---
title: "Workstream status"
type: synthesis
tags: [status]
sources: [animation-v2-notes, k21-recalibration-brief, c6-outcome-ledger9-review, design-2-7-chip-leaf, e2-e4-experiments, quint-chipleaf-final, chip-spike-experiments, runlength-consensus-brief, lookup-cache-brief, runlength-consensus, runlength-consensus-audit, final-design-decisions, land-3x-merge, astra-area1-processor-draft, astra-area2-memory-draft, astra-area3-recursion-draft, c1-dense-memory-experiment, lookup-cache-consensus, lookup-cache-audit, rule-maxima-experiment, sherlock-architecture, arch-audit-astra, arch-audit-response, gemini-area1-processor-draft, gemini-area2-memory-draft, gemini-area3-recursion-draft]
last_updated: 2026-11-11
---

# Workstream status

State as of 2026-11-11 04:20 local. The last recorded `origin/main` push is `e39bc7af8` (the land-3x merge); later commits are recorded in [[log]]. Decisions marked "lead" were taken overnight while the owner is asleep and are pending owner review ([[final-design-decisions]]).

| Workstream | State | Next step | Pages |
|---|---|---|---|
| C6 outcome-only publication | Adopted into the spec, on `main` and pushed (`333cbe05a`, follow-up `d9c4e6edd`) | Contract, template and key cascade; value digest experiment; refinement model | [[C6OutcomeOnlyPublication]], [[c6-outcome-ledger9-review]] |
| Chip leaf | Proposed, not adopted. Final spike key recorded in design 2.7.11; Quint lanes done. Lead R-3 replaces D3 memory in the design direction | SI-12 under C = 8; PAGE, PROG, TRAP, Init; real rules; a proof; C1 dense backend built and measured as a spike chip ([[c1-dense-memory-experiment]]), LogUp backend and leaf wiring open; lead L-1 to L-10 add a block cache ([[BlockCache]]), keep the C1 interface, pick one arena (L-9); per-rule maximum table and non-interleaving check done ([[rule-maxima-experiment]], M-1 to M-4); block-line method, leaf wiring and ROM maxima (X15) not done | [[ChipLeaf]], [[design-2-7-chip-leaf]], [[DenseTailMemory]] |
| Chip page animation | On `main` and pushed (`67e6e6cab`) | none recorded | [[ChipLeaf]] |
| README Sherlock section | Updated for outcome-only publication and the chip leaf, on `main` and pushed (`150810932`) | none recorded | [[C6OutcomeOnlyPublication]] |
| E4 leaf cap k=20 | On `main` and pushed (`f8208f582`; runs recorded in `3bf2a740e`). `C6_LEAF_MAX_K = 20` for the C6 digest leaf path only; `MAX_K` stays 19 elsewhere | Task 4.13 genesis prove time | [[e2-e4-experiments]] |
| 3.x dispatch partition | Merged into `main` (`e39bc7af8`, true merge of `land-3x`), on `origin/main`. U023c receipt `dispatch.json` from run `20261111T013913Z-land3x-u023c`, 6 passed, 0 failed. No task ticked | Per-class-group T2 runs; U023a to U023c predicates still `unimplemented.sh`; task 3.4 receipts and inventory flips | [[land-3x-merge]] |
| Run-length ceiling study | Done: consensus (5 lanes) and Fable 5.2 audit (adopt with changes). Lead decisions R-1 to R-10 | Owner review of R-4 (TOP >= 63) and R-5 (claim counter); experiment C1 for rows and arenas (R-6, decide in C4) | [[runlength-consensus]], [[runlength-consensus-audit]], [[RunLengthCeiling]] |
| GP-1 and GP-001 wording | Lead R-2: use the audit's corrected text, "applied in night-plan step D3" (a night-plan step, not the D3 memory constraint). Applied to `general-purpose.md`, GP-001 and the spec mirror in the commit with this wiki update; openspec validate and F14 pass | Owner review | [[GeneralPurposeRecursion]], [[final-design-decisions]] |
| Final design (run `sherlock-final-design-20261111`) | Six drafts (three Astra, three Gemini) merged into `ARCHITECTURE.md` (B1); GPT-6 Astra audit: adopt with changes, 4 blocker, 15 major, 1 minor; revision B2 fixes all 20 as design text: 23 kits, OD-1 to OD-22, X1 to X15 (X1, X2 done), Lean 14 proved and 53 open. Lead decisions A-1 to A-6, pending owner review. Nothing built for B2 | Owner review of OD-1, OD-2, OD-17, OD-18, OD-20, OD-21; OpenSpec change `sherlock-modular-leaf-20261111` (one spec per kit); experiments X10 to X15; group F interface freeze | [[sherlock-architecture]], [[arch-audit-astra]], [[arch-audit-response]], [[ModularArchitecture]], [[gemini-area1-processor-draft]], [[gemini-area2-memory-draft]], [[gemini-area3-recursion-draft]], [[astra-area1-processor-draft]], [[astra-area2-memory-draft]], [[astra-area3-recursion-draft]] |
| Lookup cache study | Done: consensus (5 lanes, Profile A dense Merkle plus 8-record PAGE and ROM lines, Profile B ZmLogUp + Horner behind the interface) and Fable 5.2 audit (adopt with changes; 3 blocker, 11 major, 9 minor). Lead decisions L-1 to L-10, pending owner review | Per-rule maximum table and one-arena non-interleaving check done ([[rule-maxima-experiment]]: non-interleaving holds, `K_page >= 4` as a code-reading bound, `K_rom` blocks per table open); owner review of L-2 and L-9; X4 line prototype, X15 ROM rows per step; measurements of the block cache | [[lookup-cache-consensus]], [[lookup-cache-audit]], [[rule-maxima-experiment]], [[BlockCache]], [[final-design-decisions]] |

## 3.x dispatch partition

- The merge carries the dispatch partition, the split dispatch tests, the k_U probe, the receipt and the `tasks.md` notes for 3.1 to 3.4 and 4.1 ([[land-3x-merge]]).
- The U023c result recorded by the lead earlier is now in repository files: the 3.3 note and `testdata/receipts/dispatch.json`. The earlier log notes that it was not found are settled.
- Why no task is ticked: 3.1 and 3.2 predicates are still `unimplemented.sh`; 3.2 has 291 unrealized classes and 5 rules with no case; 3.3 is one run, not one budget key per class group, and covers 72 of 108 rules; 3.4 lacks per-rule receipts and inventory flips ([[land-3x-merge]], [[final-design-decisions]] O-1).

## Open questions

See [[OpenOwnerQuestions]]: the D3 run ceiling (answered, pending owner), R-4 / OD-1 tree depth (TOP = 63 backed by M-2), R-5 / OD-2 claim counter (no count), R-6 and L-9 arenas (one arena, check done), L-2 cache-line counts, OD-17 input admission (the Init mechanism), OD-18 one-arena SRC-021 text, OD-20 GP-2 scope, OD-21 semantic reference, D-1, D-2, D-6 and exhaust kind 7.
| k = 21 recalibration and zero knowledge | Owner decisions 2026-11-11: node and decider at k = 21; zero knowledge end to end. Brief written, measurement lane running | Design lanes after M, S1, N1b and the elastic chip proposals; merge, verify, apply | [[k21-recalibration-brief]], [[ZeroKnowledge]] |
| Pipeline animation version 1 and version 2 | Version 1: scenes 1 to 8 built on branch `pipeline-anim`, integration running. Version 2: notes and running record started | Version 1: integration, location amendment, Terra QA, fix round, merge. Version 2: add each landing design round | [[AnimationV2]], [[animation-v2-notes]] |
