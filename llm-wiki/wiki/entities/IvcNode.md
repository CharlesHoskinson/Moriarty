---
title: "IVC node"
type: entity
tags: [recursion, node]
sources: [chip-spike-experiments, e2-e4-experiments, gp-architecture, design-2-7-chip-leaf, runlength-consensus-brief, council-findings, council-revisions, cost-lane-fable, cost-lane-astra, cost-lane-gemini, elastic-chip-gpu-fable]
last_updated: 2026-11-11
---

# IVC node

The self-recursive node of the Sherlock recursion (`crates/mori-recursion/src/ivc.rs`). It verifies a leaf proof and its own previous proof in circuit; pairings are deferred to the decider ([[gp-architecture]]).

- Gate: at most 950,000 rows at k = 20 over the leaf key ([[design-2-7-chip-leaf]]).
- Over the final chip leaf key: 823,466 rows, k = 20, a `cost_model` figure on an unsafe SRS with no proof ([[chip-spike-experiments]]).
- Over the C6 digest leaf: 741,708 to 741,736 rows across leaf k = 13 to 20, k = 20, (unlabeled) in [[e2-e4-experiments]]; [[gp-architecture]] calls the 741,708 figure "Measured model rows". This is a different leaf key from the chip figure.
- Measured node step with a stand-in leaf: 61 to 67 s alone, 90.2 s while a leaf also proves ([[e2-e4-experiments]]).
- Three proof slots do not fit k = 20 (1,078,147 measured model rows) ([[gp-architecture]]).
- The node carries the step counter `h`; SRC-004 keeps it below 2^64 ([[runlength-consensus-brief]]). See [[RunLengthCeiling]].
- The in-circuit verifier supports only rotations -1, 0 and +1 ([[chip-spike-experiments]]).

## Council revisions (branch `pipeline-anim`)

- C5 (blocker): `reg_root` was not tied across child slots. P-1: `reg_root` equal in every enabled NODE or FOREIGN child and the parent, mutant `node_reg_root_mixed`, under 10 rows (ESTIMATE) ([[council-findings]], [[council-revisions]]).
- New administrative modes ADMIT-WORK (P-5, program admission) and VIEW-WORK (P-6, certified input view); the inline kernel is built from the leaf kit code (P-4); FV shape case and payload fixed (P-2) ([[council-revisions]]).

## Cost round (proposals, not merged)

- 78 percent of the measured stand-in node is two non-native MSMs, `collapse_self` 239,577 and `collapse_final` 267,514 of 652,437 rows (measured) ([[cost-lane-fable]], [[cost-lane-astra]]).
- The node term is more than 90 percent of the rows per step at S = 16 (ESTIMATE) ([[cost-lane-fable]]); 98.42 percent at the brief's figures ([[cost-lane-astra]]).
- Rule from [[cost-lane-fable]]: every column and lookup added to the leaf is paid again in the node (7,298 node rows per advice column, about 22,600 per lookup, from the design fit).
- [[cost-lane-gemini]] proposes single-collapse accumulation (k = 20 to k = 19, ESTIMATE); [[cost-lane-fable]] cites the consensus design that k = 19 is "not reachable by trims". Not reconciled.
- GPU: one k = 20 node proof spends 0.57 to 0.71 of its wall in quotient and NTT and 0.16 to 0.26 in MSM (measured); with the quotient on the GPU about 3.5 times faster (ESTIMATE) ([[elastic-chip-gpu-fable]]).

See also [[RunDecider]], [[ChipLeaf]].
