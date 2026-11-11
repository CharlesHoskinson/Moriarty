---
title: "Cost round, Astra lane: Sherlock cost optimization"
type: source
tags: [lane-report, cost, compact, gpu, multicore]
sources: [cost-lane-astra]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/proposals/astra.md
source_url: local
---

# Cost round, Astra lane

## Summary

The GPT-6 Astra answer to [[cost-round-brief]], at repository snapshot `4b5697f7d`. No experiment was run. The lane finds no measured Compact-on-Sherlock time ratio: native sizes are measured ([[n1-native-circuits]]), step counts, integrated leaf costs and native proving times are not. Its priority: reduce the number and cost of recursion nodes, then the decider. No measured universal design reaches 10x or 2x today; the lane gives explicit row budgets (TARGET) for both. A proposal, not merged.

## Key Claims

- Labels used: MEASURED, DESIGN, ESTIMATE, TARGET (a proposed acceptance budget, not a prediction), UNREAD.
- At the brief's figures the node is 98.42 percent of leaf-plus-node rows (94.83 percent with a 51,800-row leaf); node over leaf is 62.39. Scenario percentages, not measured shares.
- Conditional row ratios at leaf 15,226, node 950,000, decider 1,000,000 and the feasibility lane's steps (ESTIMATE): `set` 6,465 to 14,412 times; `vote$commit` 11,732 to 23,338; `spend` 15,108 to 30,174. Time ratios stay unidentified (node-plus-decider projection 22,945 to 50,320 s for `spend`, over an unmeasured native time).
- Replacing the 742,327-row stand-in leaf with 15,226 rows is a 48.75 times leaf reduction but only 1.75 times on leaf plus node; removing all leaf rows would gain only 1.016 times.
- The node already accumulates and defers the pairing, so "accumulation instead of a pairing every step" is not a new saving. Removing both measured collapses gives only about 4.49 times on the old node; a 95,000-row node is a TARGET, not a folding result.
- Larger S: 3.91 times at S = 64 and 7.65 times at S = 128 (ESTIMATE), only when leaves fill; the append window holds `8W+7-r_pre` new records, so at W = 5 a trace with three appends per step can close after about 13 to 15 steps.
- Native proof through GP-3: with a 550,000 to 750,000-row lift and one 950,000-row join per call, `spend` stays at 63.43 to 71.76 times native (ESTIMATE).
- Batching: decider rows per call fall to 2.60 times native `spend` at B = 16 and 0.65 at B = 64, but batch throughput is not per-caller latency.
- Single-call 10x TARGET: one `spend` leaf at most 60,000 rows, all recursion and sealing at most 60,000, decider at most 100,000: at most 220,000 rows = 9.16 times. 2x TARGET: 30,000 + 8,000 + 10,000 = 48,000 rows = 1.998 times, about two orders below today's gates. Universal 2x is a research target, not an engineering estimate.
- Multicore leaf: at three appends per step a full C = 4, s = 128 leaf needs 1,536 append slots and W >= 192, and `R_persist(W) = 12,417 + 10,441(W-1)` exceeds two million rows there (arithmetic on the measured model). Choose one production C; C per program would create several keyed shapes.
- GPU: sppark and ICICLE v2.8.0 are named; whole-proof gain 2.60 times at 5x on the offloaded fraction, 4.35 times at the limit (ESTIMATE). A GPU gain shared with native proofs may leave the ratio unchanged.
- A one-day 10^9-step run needs 11,574 useful steps per second, about 22.6 leaf-equivalent aggregation units per second at 512 steps per leaf.
- Ranked: straight-line mode with bounded builtins; cheaper node and decider; larger effective occupancy; native proof through GP-3; shared decider; GPU kernels; binary tree; fixed multicore leaf. Hybrid native keys are an application option.
- Papers: seven listed, all UNREAD.

## Key Quotes

- "reduce the number and cost of recursion nodes, then address the final decider."
- "Batch throughput and single-call response latency are different metrics. Do not report batch time divided by B as the latency experienced by each caller."

## Connections

[[cost-round-brief]], [[n1-native-circuits]], [[cost-lane-fable]], [[cost-lane-gemini]], [[IvcNode]], [[RunDecider]], [[DenseTailMemory]].

## Contradictions

- Native proving time is unknown here; the lead measured it later: 0.552 s, 1.022 s and 1.755 s for the three entries ([[n1b-native-proving-time]]).
- Ranks straight-line mode first and larger S third; [[cost-lane-fable]] ranks the large leaf first; [[cost-lane-gemini]] ranks chip hashes first.
- Native proof through GP-3 is the first composition route to measure here; [[cost-lane-gemini]] rejects it for single calls.
- Reachability: here no measured design reaches 10x and 2x is research; [[cost-lane-gemini]] claims designs at 7.65 to 8.74 times (10x) and 1.51 to 1.78 times (2x); [[cost-lane-fable]] gives 4 to 8 times for 10x and 2 to 4 times as the best universal estimate.
