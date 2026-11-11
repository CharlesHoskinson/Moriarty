---
title: "IVC node"
type: entity
tags: [recursion, node]
sources: [chip-spike-experiments, e2-e4-experiments, gp-architecture, design-2-7-chip-leaf, runlength-consensus-brief]
last_updated: 2026-11-10
---

# IVC node

The self-recursive node of the Sherlock recursion (`crates/mori-recursion/src/ivc.rs`). It verifies a leaf proof and its own previous proof in circuit; pairings are deferred to the decider ([[gp-architecture]]).

- Gate: at most 950,000 rows at k = 20 over the leaf key ([[design-2-7-chip-leaf]]).
- Over the final chip leaf key: 823,466 rows, k = 20, a `cost_model` figure on an unsafe SRS with no proof ([[chip-spike-experiments]]).
- Over the C6 digest leaf: 741,708 to 741,736 rows across leaf k = 13 to 20, k = 20 ([[e2-e4-experiments]]). This is a different leaf key from the chip figure.
- Measured node step with a stand-in leaf: 61 to 67 s alone, 90.2 s while a leaf also proves ([[e2-e4-experiments]]).
- Three proof slots do not fit k = 20 (1,078,147 measured model rows) ([[gp-architecture]]).
- The node carries the step counter `h`; SRC-004 keeps it below 2^64 ([[runlength-consensus-brief]]). See [[RunLengthCeiling]].
- The in-circuit verifier supports only rotations -1, 0 and +1 ([[chip-spike-experiments]]).

See also [[RunDecider]], [[ChipLeaf]].
