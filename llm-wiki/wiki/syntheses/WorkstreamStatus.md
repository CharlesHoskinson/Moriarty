---
title: "Workstream status"
type: synthesis
tags: [status]
sources: [c6-outcome-ledger9-review, design-2-7-chip-leaf, e2-e4-experiments, quint-chipleaf-final, chip-spike-experiments, runlength-consensus-brief, lookup-cache-brief]
last_updated: 2026-11-10
---

# Workstream status

State as of 2026-11-10. The commits marked "on `main` and pushed" are contained in `origin/main` at `c57351dd4`.

| Workstream | State | Next step | Pages |
|---|---|---|---|
| C6 outcome-only publication | Adopted into the spec, on `main` and pushed (`333cbe05a`, follow-up `d9c4e6edd`) | Contract, template and key cascade; value digest experiment; refinement model | [[C6OutcomeOnlyPublication]], [[c6-outcome-ledger9-review]] |
| Chip leaf | Proposed, not adopted. Final spike key recorded in design 2.7.11; Quint lanes done | SI-12 under C = 8; PAGE, PROG, TRAP, Init; real rules; a proof | [[ChipLeaf]], [[design-2-7-chip-leaf]], [[quint-chipleaf-final]] |
| Chip page animation | On `main` and pushed (`67e6e6cab`) | none recorded | [[ChipLeaf]] |
| README Sherlock section | Updated for outcome-only publication and the chip leaf, on `main` and pushed (`150810932`) | none recorded | [[C6OutcomeOnlyPublication]] |
| E4 leaf cap k=20 | On `main` and pushed (`f8208f582`; runs recorded in `3bf2a740e`). `C6_LEAF_MAX_K = 20` for the C6 digest leaf path only; `MAX_K` stays 19 elsewhere | Task 4.13 genesis prove time | [[e2-e4-experiments]] |
| 3.x dispatch partition | Landing: branch `land-3x` (head `2947727d6`), not yet on `main` | Merge to `main` | see below |
| Run-length ceiling study | Pending, running | Ingest `CONSENSUS.md` and the audit when they exist | [[runlength-consensus-brief]], [[RunLengthCeiling]] |
| Lookup cache study | Pending, running | Ingest `CONSENSUS.md` and the audit when they exist | [[lookup-cache-brief]] |

## 3.x dispatch partition

- Branch `land-3x` carries the dispatch partition, the split dispatch tests and the k_U probe (tasks 3.1 to 3.3 and 4.1; commits `506bd78af` to `2947727d6`).
- The lead reported that its U023c receipt passed 6 of 6. That result was not found in commits on `land-3x` or in the main checkout. The `tasks.md` 3.3 note on `land-3x` still lists the T2 runs and the receipt `testdata/receipts/dispatch.json` as open, and that receipt file does not exist on the branch. Confirm on the next sync.

## Open questions

See [[OpenOwnerQuestions]]: the D3 run ceiling against GP-1, D-1, D-2, D-6, exhaust kind 7 and the Init mechanism.
