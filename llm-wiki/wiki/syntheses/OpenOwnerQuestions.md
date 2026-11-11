---
title: "Open owner questions"
type: synthesis
tags: [open-questions]
sources: [dissent-c6-section, c6-outcome-ledger9-review, design-2-7-chip-leaf, general-purpose-definition, runlength-consensus-brief, lookup-cache-brief, gp-architecture]
last_updated: 2026-11-10
---

# Open owner questions

Questions the owner must settle. Keep each until a source settles it. See [[WorkstreamStatus]].

| # | Question | State | Pages |
|---|---|---|---|
| 1 | D3 run ceiling against GP-1 | Open; one running study is on the bound | [[RunLengthCeiling]], [[dissent-c6-section]] |
| 2 | D-1 SI-12 to SI-14 under C = 8 | Open; no form measured | [[dissent-c6-section]], [[ChipLeaf]] |
| 3 | D-2 type identity in a constructor digest | Open | [[ValueDigest]], [[dissent-c6-section]] |
| 4 | D-6 leaf partition in the GP-4 shape commitment | Open, against GPR-016 | [[dissent-c6-section]], [[gp-architecture]] |
| 5 | Exhaust kind 7 against SVM-009 | Open conflict | [[ValueDigest]], [[dissent-c6-section]] |
| 6 | Init mechanism for an input under the new digest | Open | [[ValueDigest]], [[c6-outcome-ledger9-review]] |

## 1. The D3 run ceiling against GP-1

- Fact: under D3 a run has fewer than 2^17 leaves and 2^20 cells. GP-1 says "The run length is bounded only by the height field (below 2^64)" ([[dissent-c6-section]], [[general-purpose-definition]]).
- The 2^64 is the step-counter width in SRC-004, not a circuit size: SRC-004 makes the node unsatisfiable if `prev_h`, `steps` or `h = prev_h + steps` is not below 2^64. "No circuit has 2^64 rows." ([[runlength-consensus-brief]]).
- Owner, 2026-11-11: "2^64 is physically impossible" and "I'm not understanding what 2^64 is, is it a limit or a something we calculate? We cannot calculate a 2^64 circuit" ([[runlength-consensus-brief]]).
- Question recorded in the dissent file: "which change to the memory design removes the ceiling, so that one key set proves runs up to 2^64 steps?" Task 4.19 records the candidates; the lead makes no recommendation ([[dissent-c6-section]]).
- GP-1 is not narrowed. "A narrowing needs an explicit owner decision" ([[general-purpose-definition]], [[OwnerRules]]).
- Pending: [[runlength-consensus-brief]] studies the bound (Q1 asks for replacement GP-1 and GP-001 text that does not narrow GP-1). [[lookup-cache-brief]] studies hashing cost and must say where its design touches the run-length study. No result is ingested.

## 2. D-1 SI-12 to SI-14

Dense logical identifiers (Astra), addresses as references (Fable), or linked slices (Carbon). The candidate to measure first is Carbon's linked-slice form, Astra's mapping the alternative (task 4.18). SI-12 blocks D3 adoption for the C6 machine, not the publication decision ([[dissent-c6-section]]).

## 3. D-2 type identity

The verdict takes the program-independent `ty_d`; Astra's dissent (program-scoped identity) stays. The normal form of `decl_schema_d` must be fixed before any key is frozen ([[dissent-c6-section]]).

## 4. D-6 leaf partition in the shape commitment

Carbon: bind only the inter-claim DAG publicly. Astra: the shape may differ between proofs of the same claims. The GP study binds segment bracketing into `shape_d`. It is a decision on GPR-016. Also open: whether the "local statement" in `shape_d` may contain `pre_d` or `post_d` ([[dissent-c6-section]], [[gp-architecture]]).

## 5. Exhaust kind 7

SRC-022 lists `Exhausted` kind 7 = `LogicalCap`; SVM-009 refuses exhaust reason 7 in profile `/1`. The dissent record does not reconcile the two ([[dissent-c6-section]]).

## 6. Init mechanism

How `val_d` of an input relates to the initial memory (lazy unfold or load steps) is open. It must not scan the input in the Init check ([[c6-outcome-ledger9-review]], [[design-2-7-chip-leaf]]).
