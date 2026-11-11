---
title: "Workstream status"
type: synthesis
tags: [status]
sources: [c6-outcome-ledger9-review, design-2-7-chip-leaf, e2-e4-experiments, quint-chipleaf-final, chip-spike-experiments, runlength-consensus-brief, lookup-cache-brief, runlength-consensus, runlength-consensus-audit, final-design-decisions, land-3x-merge, astra-area1-processor-draft, astra-area2-memory-draft, astra-area3-recursion-draft]
last_updated: 2026-11-10
---

# Workstream status

State as of 2026-11-10 23:24 local. `origin/main` is at `e39bc7af8` (the land-3x merge). Decisions marked "lead" were taken overnight while the owner is asleep and are pending owner review ([[final-design-decisions]]).

| Workstream | State | Next step | Pages |
|---|---|---|---|
| C6 outcome-only publication | Adopted into the spec, on `main` and pushed (`333cbe05a`, follow-up `d9c4e6edd`) | Contract, template and key cascade; value digest experiment; refinement model | [[C6OutcomeOnlyPublication]], [[c6-outcome-ledger9-review]] |
| Chip leaf | Proposed, not adopted. Final spike key recorded in design 2.7.11; Quint lanes done. Lead R-3 replaces D3 memory in the design direction | SI-12 under C = 8; PAGE, PROG, TRAP, Init; real rules; a proof; experiment C1 | [[ChipLeaf]], [[design-2-7-chip-leaf]], [[DenseTailMemory]] |
| Chip page animation | On `main` and pushed (`67e6e6cab`) | none recorded | [[ChipLeaf]] |
| README Sherlock section | Updated for outcome-only publication and the chip leaf, on `main` and pushed (`150810932`) | none recorded | [[C6OutcomeOnlyPublication]] |
| E4 leaf cap k=20 | On `main` and pushed (`f8208f582`; runs recorded in `3bf2a740e`). `C6_LEAF_MAX_K = 20` for the C6 digest leaf path only; `MAX_K` stays 19 elsewhere | Task 4.13 genesis prove time | [[e2-e4-experiments]] |
| 3.x dispatch partition | Merged into `main` (`e39bc7af8`, true merge of `land-3x`), on `origin/main`. U023c receipt `dispatch.json` from run `20261111T013913Z-land3x-u023c`, 6 passed, 0 failed. No task ticked | Per-class-group T2 runs; U023a to U023c predicates still `unimplemented.sh`; task 3.4 receipts and inventory flips | [[land-3x-merge]] |
| Run-length ceiling study | Done: consensus (5 lanes) and Fable 5.2 audit (adopt with changes). Lead decisions R-1 to R-10 | Owner review of R-4 (TOP >= 63) and R-5 (claim counter); experiment C1 for rows and arenas (R-6, decide in C4) | [[runlength-consensus]], [[runlength-consensus-audit]], [[RunLengthCeiling]] |
| GP-1 and GP-001 wording | Lead R-2: use the audit's corrected text, "applied in night-plan step D3" (a night-plan step, not the D3 memory constraint). Applied to `general-purpose.md`, GP-001 and the spec mirror in the commit with this wiki update; openspec validate and F14 pass | Owner review | [[GeneralPurposeRecursion]], [[final-design-decisions]] |
| Final design (run `sherlock-final-design-20261111`) | Three Astra area drafts written before the run-length consensus existed; drafts only, for a later merge | Architecture merge (B1); reconcile with R-3 to R-7 | [[astra-area1-processor-draft]], [[astra-area2-memory-draft]], [[astra-area3-recursion-draft]] |
| Lookup cache study | Pending; no consensus or audit ingested | Ingest `CONSENSUS.md` and the audit when they exist | [[lookup-cache-brief]] |

## 3.x dispatch partition

- The merge carries the dispatch partition, the split dispatch tests, the k_U probe, the receipt and the `tasks.md` notes for 3.1 to 3.4 and 4.1 ([[land-3x-merge]]).
- The U023c result recorded by the lead earlier is now in repository files: the 3.3 note and `testdata/receipts/dispatch.json`. The earlier log notes that it was not found are settled.
- Why no task is ticked: 3.1 and 3.2 predicates are still `unimplemented.sh`; 3.2 has 291 unrealized classes and 5 rules with no case; 3.3 is one run, not one budget key per class group, and covers 72 of 108 rules; 3.4 lacks per-rule receipts and inventory flips ([[land-3x-merge]], [[final-design-decisions]] O-1).

## Open questions

See [[OpenOwnerQuestions]]: the D3 run ceiling (answered, pending owner), R-4 tree depth, R-5 claim counter, R-6 arenas, D-1, D-2, D-6, exhaust kind 7 and the Init mechanism.
