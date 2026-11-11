---
title: "Dissent record: C6 outcome-only review section"
type: source
tags: [c6, dissent, open-questions]
sources: [dissent-c6-section]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/dissent.md
source_url: local
---

# Dissent record: C6 outcome-only review section

## Summary

The section "C6 outcome-only review (2026-11-10)" of `dissent.md` records the dissent and open items of [[c6-outcome-ledger9-review]]. It states that the decision adopts the publication rule only, not the chip leaf or D3. It keeps six dissent items (D-1 to D-6), a recorded-as-wrong SRS dissent, an open conflict on exhaust kind 7, and the owner question on the D3 run ceiling against GP-1.

## Key Claims

- D-1 SI-12 to SI-14 (open): Astra keeps dense logical identifiers with an authenticated mapping; Fable keeps the address as the reference; Carbon keeps addresses and uses cons-linked slice cells `(TAG_SLICE, elem_ref, prev_link)`. No form is measured under C = 8. The candidate to measure first is Carbon's linked-slice form (task 4.18). SI-12 blocks D3 adoption for the C6 machine, not the publication decision.
- D-2 type identity in a constructor digest (open): Astra hashes the producer's program identity; Fable and Carbon hash a program-independent `ty_d`. The verdict takes the program-independent form; the normal form of `decl_schema_d` must be fixed before any key is frozen.
- D-3 step count in the claim (decided): the claim is `(version, prog_d, input_d, outcome_d)`; `n_steps` is published beside it. `architecture.md:98` needs the matching edit (task 4.20).
- D-4 cost of the cached digest (measurement task 4.16). No member measured either mechanism.
- D-5 shape of the constructor digest (resolved): Carbon's fold.
- D-6 leaf partition in the GP-4 shape commitment (open, against GPR-016 of [[gp-architecture]]).
- Carbon's Q2 SRS ceiling at k = 20 is recorded as wrong: the ledger data catalogue lists ceremony files through `bls_midnight_2p25`. The project's row gates and prover cost exclude k = 21, not the SRS.
- Exhaust kind 7 (`LogicalCap`) against SVM-009: SRC-022 lists kind 7; SVM-009 refuses exhaust reason 7 in profile `/1`. Not reconciled.
- Owner question: under D3 a run has fewer than 2^17 leaves and 2^20 cells; GP-1 states 2^64 steps. Open conflict with GP-1. Task 4.19 records the candidates. The lead makes no recommendation.

## Key Quotes

- "GP-1 to GP-4 are not narrowed. The owner ruled that they must never be narrowed. No requirement of this change states the ceiling as a bound."
- "Question to the owner: which change to the memory design removes the ceiling, so that one key set proves runs up to 2^64 steps?"
- "This record does not reconcile the two." (exhaust kind 7)

## Connections

- [[OpenOwnerQuestions]] carries D-1, D-2, D-6, exhaust kind 7, the Init mechanism and the run ceiling.
- [[RunLengthCeiling]], [[C6OutcomeOnlyPublication]], [[ValueDigest]], [[GeneralPurposeRecursion]].

## Contradictions

- Exhaust kind 7 (`LogicalCap`, SRC-022 encoding) against SVM-009 (refuses reason 7 in profile `/1`). Also flagged on [[ValueDigest]].
- D-3 against `architecture.md:98` ([[gp-architecture]]), which puts `n_steps` in `out_d`.
- The D3 ceiling against GP-1 ([[general-purpose-definition]]).
