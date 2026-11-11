---
title: "Design 2.7: leaf as a chip (proposed, not adopted)"
type: source
tags: [chip-leaf, design, c6]
sources: [design-2-7-chip-leaf]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/design.md
source_url: local
---

# Design 2.7: leaf as a chip (proposed, not adopted)

## Summary

Section 2.7 of the Sherlock VM design proposes the universal leaf as one hand-written circuit: a CPU core plus the chips DEC, PAGE, MEM, ALU with RANGE, and HASH, reached through one lookup bus. It is headed "proposed 2026-11-10; not adopted"; until adoption, sections 2.2 to 2.4 stand. The section also records the C6 outcome-only decision of 2026-11-10, which adopts the publication rule only, not the chip leaf or D3. See [[ChipLeaf]].

## Key Claims

- Owner direction (2026-11-10): "really lean into the chip design and let's get it modular and concise".
- 2.7.2 Bus: every chip is reached through bus tuples `(chip, key, a, b, c)` tied by one `lookup_any`. No multiset, shuffle, LogUp or timestamp argument is needed.
- 2.7.3 Memory is write-once (append-only tables).
- 2.7.8 Soundness constraints D1 to D8, each with a Quint counterexample when missing. D3 is aligned segment memory (M3, C = 8): `base = C * seg` with `seg` given by 17 boolean bits; genesis `len` is C. D4 pins the chip tag on each provider row by a gate. D7 is the cached frame protocol, decided 2026-11-10.
- 2.7.9 Gates: node at most 950,000 rows and decider at most 1,000,000 over the leaf key; `k_leaf <= 16` at S = 16.
- 2.7.11 Final spike key: layout B pair rows, Poseidon-only HASH chip, RANGE on one limb lookup, M3 wired, packed selectors. 12 advice, 13 equality and 20 fixed columns, 4 lookups, degree 6.

Figures of 2.7.11. The design heads the column "Measured". The experiment files define that as `cost_model` rows over keys made on an unsafe SRS, with no proof made ([[chip-spike-experiments]]):

| Item | Figure (cost model, unsafe SRS, no proof) | Gate |
|---|---|---|
| Leaf, S = 16 | 6,511 rows, k = 13 | `k_leaf <= 16` |
| Node over the leaf key | 823,466 rows, k = 20 (headroom 126,534) | <= 950,000 |
| Decider over that node | 929,166 rows, k = 20 | <= 1,000,000 |
| Rotations of the leaf key | -1, 0, +1 | the node verifier's limit |

- Node cost fit, labelled ESTIMATE: 570,957 + 7,298 per advice, 8,189 per equality and 6,004 per fixed column; about 22,600 per lookup. Wide rows (50 advice columns) put the node at 1,373,001 rows (k = 21).
- Per leaf at S = 16, M3 is 87 Poseidon permutations (6,090 rows).
- C6 decided 2026-11-10: outcome-only publication, with the review's conditions of adoption (SRC-006, SRC-007, SRC-021, SRC-022, SRC-023, SVM-010 to SVM-012, GP-002). See [[C6OutcomeOnlyPublication]].
- Scope: the decision does not adopt the chip leaf or D3. SI-12 under C = 8 blocks D3 adoption for the C6 machine.
- The run ceiling of D3 (fewer than 2^17 leaves and 2^20 cells per run) conflicts with the GP-1 bound of 2^64 steps. It is an open owner question, and GP-1 is not narrowed ([[RunLengthCeiling]]).
- Still stand-ins: the three spike rules, no PAGE, PROG, TRAP or Init region, test siblings, a leaf statement that does not cover `prog_root`.

## Key Quotes

- "Until adoption, 2.2 to 2.4 stand."
- "This decision adopts the publication rule only. It does not adopt the chip leaf or D3"
- "It is an open owner question, and GP-1 is not narrowed."

## Connections

- [[ChipLeaf]], [[C6OutcomeOnlyPublication]], [[RunLengthCeiling]].
- Evidence: [[chip-spike-experiments]], [[quint-chipleaf-pairrow-m3]], [[quint-chipleaf-hash]], [[quint-chipleaf-final]].
- Decision source: [[c6-outcome-ledger9-review]].
- Entities: [[MoriChip]], [[IvcNode]], [[RunDecider]].

## Contradictions

- 2.7.11 says "[OPEN: a Quint model and Gemini lane over this exact circuit with packed selectors.]" [[quint-chipleaf-final]] is that lane (packed selectors SOUND WITH CONDITIONS, running-sum RANGE SOUND, M3 as built SOUND WITH CONDITIONS). The design text was not updated to say so.
- The design labels the 2.7.10 and 2.7.11 figures "Measured"; [[c6-outcome-ledger9-review]] labels the decider figure "cost model over the chip-leaf node key, unsafe SRS". Both describe the same `cost_model` numbers; no proof exists.
