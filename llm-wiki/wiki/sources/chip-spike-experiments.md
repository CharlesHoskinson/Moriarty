---
title: "Chip spike experiments E0 to E8"
type: source
tags: [chip-leaf, experiments, cost-model]
sources: [chip-spike-experiments]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/research/sherlock-processor-analogy-20261110/experiments.md; ~/Projects/Moriarty/research/sherlock-processor-analogy-20261110/experiments-2.md; ~/Projects/Moriarty/research/sherlock-processor-analogy-20261110/experiments-3.md; ~/Projects/Moriarty/research/sherlock-processor-analogy-20261110/experiments-4.md
source_url: local
---

# Chip spike experiments E0 to E8

## Summary

Four sessions of run `sherlock-processor-analogy-20261110` (2026-11-10) that sized the proposed chip leaf in `crates/mori-chip`. Labels in all four files: MEASURED is a `cost_model`, key or count from a named run; ESTIMATE is arithmetic on measured numbers. All node and decider rows are `cost_model` rows over keys made on an unsafe SRS (rows do not depend on the SRS). No proof was made. See [[ChipLeaf]].

## Key Claims

`experiments.md` (E0, E1, E3, MEASURED unless marked):
- Spike leaf (CORE, DEC, MEM, ALU, RANGE, 3 stand-in rules): 6 rows per step, `k_leaf` 13 at S = 16 (set by the 4,096-row RANGE table).
- Node over the spike leaf key: 785,128 rows at S = 16, 768,872 at S = 1, k = 20. Decider over that node: 929,166 rows, k = 20.
- The in-circuit verifier evaluates queries only at rotations -1, 0 and +1 (`verifier_gadget.rs:609`), so every leaf gate sees at most three consecutive rows.
- E1: one shared PROG walk cuts a per-rule circuit from 2,674,552 rows (k = 22) to 731,523 (k = 20), 3.66x; parity 186 of 186.
- E0 census over fact(3) (293 steps), fold (196) and list123 (303): appends average 0.20 to 0.22 per step, at most 3; no step overwrites a non-zero table row; at most 6 distinct control-word PROG rows per 16 steps.
- ESTIMATE: 12 to 23 rows per step without HASH; a leaf at S = 16 of 10,006 to 20,782 rows (k = 14 to 15).

`experiments-2.md` (E5 register file, E6 HASH):
- Layout B (pair rows), register file only: node 891,770 rows, k = 20, PASS. Wide, A-inline and staggered layouts: 1,373,001 / 1,061,231 / 1,058,485 rows, k = 21, FAIL.
- B with the stdlib HASH chip: node 999,488 (FAIL); B with a Poseidon-only HASH key shape: 953,150 (FAIL by 3,150).
- The node pays for leaf columns, not gate size. M3 recommended: one aligned segment subtree of C = 8 cells per leaf, root appended to a depth-17 top tree.

`experiments-3.md` (E7):
- Real Poseidon-only HASH chip plus RANGE on one limb lookup: node 888,466, k = 20, PASS (61,534 headroom); decider 929,166, k = 20. M3 was not yet wired (sponges absorbed free inputs).

`experiments-4.md` (E8, the final key `narrow,1,pos_m3+r1+a1+pc`):
- Node 823,466 rows, k = 20, PASS, 126,534 headroom; decider 929,166, k = 20; `k_leaf` 13 (6,511 rows).
- HASH work per leaf: 87 permutations, 6,090 rows.
- The walker forces `base / 8 < 2^17`, so `base` is a multiple of 8 and below 2^20.
- §4 C6 finding: programs see no raw addresses, but `fin_d`, the chain digests and SI-12 to SI-14 depend on raw addresses under M3. Not resolved there [OPEN]. It led to [[C6OutcomeOnlyPublication]].
- §5 stand-ins: spike rules, record tag is the active flag, no PAGE, PROG, TRAP or task stack, stand-in siblings, leaf statement `[0, 0, pre_d, post_d, 0, 0, steps]`, no core HASH requester.

## Key Quotes

- "All node and decider rows are `cost_model` rows over keys made on an unsafe SRS (rows do not depend on the SRS). No proof was made."
- "So: programs see no addresses, but the published `fin_d`, the chain digests, and the dense-allocation invariants SI-12 to SI-14 do depend on raw addresses."

## Connections

- [[design-2-7-chip-leaf]] records the final key; [[quint-chipleaf-final]], [[quint-chipleaf-hash]] and [[quint-chipleaf-pairrow-m3]] check the circuits.
- [[MoriChip]], [[IvcNode]], [[RunDecider]], [[RunLengthCeiling]].

## Contradictions

- `experiments-4.md` §5 says no Quint model or Gemini lane was run on the packed-selector M3 circuit; [[quint-chipleaf-final]] later did that.
- `experiments-2.md` and `experiments-3.md` say the HASH chip, the pair-row core and M3 need a Gemini Quint lane (not run); [[quint-chipleaf-hash]], [[quint-chipleaf-pairrow-m3]] and [[quint-chipleaf-final]] are those lanes.
