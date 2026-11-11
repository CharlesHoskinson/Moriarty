---
title: "Quint verdict: layout B pair rows and M3 aligned segments"
type: source
tags: [quint, chip-leaf, m3, c6, qa]
sources: [quint-chipleaf-pairrow-m3]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/qa/gemini-quint/chipleaf-pairrow-m3/verdict.md
source_url: local
---

# Quint verdict: layout B pair rows and M3 aligned segments

## Summary

Lead verdict on the Gemini Quint lane of 2026-11-10 for layout B pair rows and the M3 aligned-segment memory root (C = 8). Driver `argon-run`, `gemini-4-argon-high`. The lane could not run `quint`; every result is from the lead's re-run (33 commands, 20 exit 1 and 13 exit 0, each as expected).

| Part | Verdict |
|---|---|
| 1. Layout B pair rows | SOUND with C1 (the spike `rfcore.rs` already meets C1) |
| 2. M3 aligned segment, C = 8 | SOUND only with C2 to C6; not sound as written against `rf.rs` (`NIL = 0`, genesis `len = 1`) and the then D3 text |

## Key Claims

- C1 undigested scratch registers are write-only.
- C2 no honest record is the zero tuple: a non-zero tag word in every written record. A completeness failure, not a soundness one, but M3 cannot be adopted without it.
- C3 aligned base as a circuit constraint; genesis at an aligned `len`.
- C4 the D3 text must state the active prefix, zero padding, a segment hash over all C rows, and `post_d.len := base + C` at leaf close.
- C5 `len` advances by exactly 1 per append; a read refuses the zero tuple; the root update authenticates the empty slot.
- C6 partition-dependent addresses: under M3 every leaf allocates C addresses, a zero-append leaf too, so `len`, `fp`, every stored link and `mem_root` depend on where the prover closed the leaves. M3 is sound only if no program-visible value and no published statement depends on raw addresses, or if the claim names the partition. The model's reference uses the same per-leaf allocation as the circuit, so it does not catch this.
- Address space: at C = 8 and depth 20 there are 2^17 segments, one per leaf.

## Key Quotes

- "The Sherlock VM must state which option it takes." (C6)
- "It does not check that M3's addressing is the right one." (Limits, part 2 reference)

## Connections

- C6 led to [[C6OutcomeOnlyPublication]] (the "opaque addresses" option). The per-leaf allocation is the source of [[RunLengthCeiling]].
- [[c6-outcome-ledger9-review]] condition 5 asks for a refinement model whose reference does not allocate per leaf, citing this verdict's line 41.
- Later lane: [[quint-chipleaf-final]] closes C1 to C5 in the built code.
- [[ChipLeaf]], [[MoriChip]].

## Contradictions

- "Not sound as written" here against "C1 to C5 closed" in [[quint-chipleaf-final]]: the later verdict is over code changed in `experiments-4.md` ([[chip-spike-experiments]]). C6 stays open in both.
