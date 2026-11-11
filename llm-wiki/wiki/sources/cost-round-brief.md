---
title: "Cost round brief: making Sherlock competitive with native proving"
type: source
tags: [brief, cost, compact, gpu, multicore]
sources: [cost-round-brief]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/BRIEF.md
source_url: local
---

# Cost round brief: making Sherlock competitive with native proving

## Summary

The owner rejected an unmeasured "1,000 to 10,000 times cheaper" claim and asked how to optimize the design to reduce the gap. Three independent lanes answer: Claude Fable 5.2 ([[cost-lane-fable]]), GPT-6 Astra high ([[cost-lane-astra]]) and Gemini 4 Carbon high ([[cost-lane-gemini]]). The lead runs the measurements N1 ([[n1-native-circuits]]) and S1 and merges the lanes. A sixth question asks how Sherlock scales as a multicore chip on the host GPU.

## Key Claims

- The claim under test came from the Compact feasibility lane, which used the 742,327-row stand-in leaf and guessed 10 to 20 machine steps per Compact IR instruction from `fact(3)` = 293 steps. Nothing measured that guess.
- Cost model: proving cost per Compact operation = (machine steps per IR operation) x (leaf rows / S + node rows / S, plus the decider once per run) / (native rows per operation). At the design figures the node term (about 59,000 rows per step) is about 60 times the leaf term (about 950 rows per step); lanes must check this.
- Measurements: N1 native circuit sizes and proving time; S1 Sherlock machine steps for `tiny` `set` and `election` `vote$commit` by `mori simulate`.
- Questions: decompose the ratio for `tiny` `set`, `election` `vote$commit` and `zerocash` `spend`; levers per factor (straight-line mode, larger S, a cheaper node by folding, tree proving, chip hashes, native proof through GP-3, batching under one decider, per-entry hybrids); designs within 10x and 2x of native for `spend`; experiments; papers; and Sherlock as a multicore chip on the 6,144-core GPU with a wall-time model `T(cores)`.
- Constraints: GP-1 to GP-4 never narrowed; one key set for every program stays the goal, and a lever that needs a per-contract key is a finding, not a design; ledger 9 only, never ledger 8.

## Key Quotes

- Owner, 2026-11-11: "the 1000 to 10,000 times cheaper is not a number you just say. We need to understand how to optimize the design so we dramatically reduce this."
- Owner, 2026-11-11: "Can't sherlock be a multicore chip? We want the chip to scale to the GPU processing cores."

## Connections

[[cost-lane-fable]], [[cost-lane-astra]], [[cost-lane-gemini]], [[n1-native-circuits]], [[elastic-chip-brief]], [[IvcNode]], [[RunDecider]], [[WorkstreamStatus]].

## Contradictions

- The brief cites the pre-council design at line numbers that had drifted; [[cost-lane-fable]] reads OD-6 at line 2682 and the gates at 3229, not 2712 and 3259.
- The brief points lanes at the design on `main`, before the council revisions of [[council-revisions]] (one seal path, one ledger contract).
