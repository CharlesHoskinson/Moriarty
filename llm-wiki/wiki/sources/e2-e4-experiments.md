---
title: "Experiments E2 to E4: node headroom, genesis cost, k=20 leaf"
type: source
tags: [experiments, recursion, measured]
sources: [e2-e4-experiments]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md
source_url: local
---

# Experiments E2 to E4: node headroom, genesis cost, k=20 leaf

## Summary

Experiments of the recursion design review, run through `scripts/doctorwatson heavy` on 2026-11-10. E4 proves a k=20 C6 digest leaf stand-in and measures time and RSS. E2 sweeps node and decider rows against leaf k. E3 measures the in-circuit genesis check. These numbers are over the C6 digest leaf stand-in, not the chip leaf.

## Key Claims

E4, run `20261110T160436Z-e24-e4-solo`, exit 0 (real proving run):

| Item | Value |
|---|---|
| Rows (model) / k | 742,327 / 20 |
| Prove | 66.2 s (20 rayon threads) |
| SRS load and VK | 87.0 s |
| Proving key setup | 23.9 s |
| Verify | 0.015 s |
| Proof | 3,824 B |
| Peak RSS | 11.1 GB (wall 464 s) |

- A k=20 leaf fits the 16 GB leaf cap with 4.9 GB to spare. `K_LEAF_MAX` = 20. The wrapper cap `MAX_K = 19` is raised in a separate code commit (on main as `f8208f582`: `C6_LEAF_MAX_K = 20` for the C6 digest leaf path only).
- Concurrent run `20261110T180006Z-e24-e4-concurrent`: aggregate peak RSS 17.6 GB; node step 90.2 s (61 to 67 s alone); leaf 69.1 s (66.2 s alone). The slowdown of a full overlap is not measured.

E2, run `20261110T064428Z-e24-e2` (row figures (unlabeled) in the source; the filled-leaf sweep hit the 1 h cap after leaf k=18):
- Node over the C6 digest leaf: 741,708 rows at leaf k = 13, growing 4 rows per extra leaf k, to 741,736 at leaf k = 20. Node k = 20 throughout.
- Decider: 929,166 rows, k = 20, unchanged across leaf k.
- Headroom under 2^20: 306,840 rows (node) and 119,410 rows (decider). The leaf's own prover cap limits it, not the node.
- These node figures are over the C6 digest leaf; the chip-key node figure (823,466) is a different key ([[chip-spike-experiments]]).

E3, run `20261110T161809Z-e24-e3`, exit 0:
- The genesis check adds 27,736 rows on the C6 digest leaf (2,637,481 to 2,665,217, k=22 in both).
- Genesis prefix lengths differ by program, so carriers are not padded to `Bounds` (a GP-1 gap, unchanged).
- Not measured: the prove time of the check. Task 4.13 stays open. The chip `Init` of design §3.2(5) is not yet a circuit.

## Key Quotes

- "The leaf's size does not limit the node; the leaf's own prover cap does."

## Connections

- [[IvcNode]], [[RunDecider]], [[ChipLeaf]].
- The running study [[runlength-consensus-brief]] cites E4 as the practical throughput basis.

## Contradictions

None found.
