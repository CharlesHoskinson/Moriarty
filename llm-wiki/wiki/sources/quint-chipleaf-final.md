---
title: "Quint verdict: final chip leaf (packed selectors, RANGE, M3)"
type: source
tags: [quint, chip-leaf, qa]
sources: [quint-chipleaf-final]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/qa/gemini-quint/chipleaf-final/verdict.md
source_url: local
---

# Quint verdict: final chip leaf (packed selectors, RANGE, M3)

## Summary

Lead verdict on three Gemini Quint lanes of 2026-11-10 over `crates/mori-chip/src/` at main `f59ef318f`. Driver `argon-run` with `gemini-4-argon-high`, one lane per question, each in `heavy.slice`. The lanes' sandbox refused `quint`, so every result comes from the lead re-run (quint 0.32.0, sampled `quint run` with 4000 samples and 3 steps, plus pure tests).

| Question | Verdict | Gap in the Rust code |
|---|---|---|
| Q1 packed selectors | SOUND WITH CONDITIONS (P1 to P4) | none |
| Q2 running-sum RANGE | SOUND | none |
| Q3 M3 as built | SOUND WITH CONDITIONS (C1 to C5 closed; C6 still open, out of scope) | none |

## Key Claims

- No mutant that breaks a required constraint survives. No real gap was found, so there is no failing MockProver test to add.
- P1 one code per row per column (holds on every layout today; nothing asserts it); P2 distinct codes; P3 degree bound 6; P4 lookup selectors not packed.
- RANGE: `value < 2^128`, far below p, so no field wrap. The ALU operand bound below 2^128 is a premise from the requester, not checked here.
- M3: the 17-bit index forces `base / 8 < 2^17`. C1 to C5 of [[quint-chipleaf-pairrow-m3]] are closed by the current code. C6 (partition-dependent `len` and physical addresses) stays open; it is VM and publication policy, out of scope.
- New finding: `mem first` is redundant in the full leaf but load-bearing at chip level; keep it.
- Re-run: packsel 16 passing, 9 of 9 mutants caught; range 15 passing, 8 of 8 caught; m3 20 passing and 2 failing (the 2 failing tests expect M9 to be caught, an expectation the lead judges wrong), 14 of 15 mutants caught.
- Optional defensive test for P1: a negative MockProver test and a `debug_assert!` in `VSel::enable`. Not a gap.

## Key Quotes

- "No mutant that breaks a required constraint survives."
- "C6 (partition-dependent `len` and physical addresses) stays open; it is VM and publication policy, out of scope here."

## Connections

- [[ChipLeaf]], [[MoriChip]], [[quint-chipleaf-pairrow-m3]], [[quint-chipleaf-hash]].
- C6 is handled by [[C6OutcomeOnlyPublication]].

## Contradictions

- [[design-2-7-chip-leaf]] (2.7.11) and `experiments-4.md` §5 ([[chip-spike-experiments]]) say there is no Quint model or Gemini lane over the packed-selector M3 circuit. This verdict is that lane; those texts predate it.
- [[quint-chipleaf-pairrow-m3]] found M3 "SOUND only with C2 to C6" and not sound as written. This verdict finds C1 to C5 closed by the current code, so the two agree once the code changes of `experiments-4.md` are counted.
