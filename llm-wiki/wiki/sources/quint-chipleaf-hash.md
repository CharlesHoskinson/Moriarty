---
title: "Quint verdict: Poseidon-only HASH chip"
type: source
tags: [quint, chip-leaf, hash, qa]
sources: [quint-chipleaf-hash]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/qa/gemini-quint/chipleaf-hash/verdict.md
source_url: local
---

# Quint verdict: Poseidon-only HASH chip

## Summary

Lead verdict on the Gemini Quint lane of 2026-11-10 for the Poseidon-only HASH chip (`crates/mori-chip/src/poseidon.rs` and its bus use) and constraints D1, D4 and D8 of design 2.7.8. Driver `argon-run`, `gemini-4-argon-high`. The lane could not run `quint`; every result is from the lead's re-run. Verdict: **SOUND**, with conditions C1 to C6. The current `poseidon.rs` meets C1 to C4; C5 and C6 apply to code that was not built yet.

## Key Claims

- The model abstracts the permutation as a bijection of Z_p^3 (p = 1000003), with 2 round rows per permutation.
- C1 the layout asserts `n >= 2` and `!provide || n == 2` are load-bearing. C2 the capacity is fixed. C3 absorb and round gates. C4 the provider row pins `chip = HASH` and `key = 2`.
- C5 (D4 and D8): the chip tag is gate-pinned advice. Sound only while every row with `q_prov = 1` has a gate that pins its own nonzero chip id. Either D4's text changes to "gate-pinned advice on every `q_prov` row", or the tag moves to a fixed column. (Design 2.7.8 D4 now reads as gate-pinned; see [[design-2-7-chip-leaf]].)
- C6 (not built yet): the core has no HASH requester; in-region sponge inputs must be `HIn::Cell` copies and the output copy-constrained.
- Re-run: 22 commands, 17 exit 1 each with a `[violation]`, 5 exit 0; 22 tests passing; all 14 mutants H1 to H14 caught.
- Lead repair 1 changed the round function (integer overflow fixed by working in Z_p with S-box `x^5 mod p`); the lead asks the caller to judge it.

## Key Quotes

- "In the model, no accepted witness gives a bus HASH request an output other than the sponge of exactly its inputs."
- "Nothing here proves soundness for every witness."

## Connections

- [[ChipLeaf]], [[MoriChip]], [[quint-chipleaf-final]], [[quint-chipleaf-pairrow-m3]].
- HASH work dominates the leaf: [[chip-spike-experiments]].

## Contradictions

None found. The C5 alternative ("D4's text changes") matches the D4 text now in [[design-2-7-chip-leaf]].
