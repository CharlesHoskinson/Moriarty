---
title: "Recalibration round brief: the Sherlock chip at k = 21"
type: source
tags: [brief, k21, lookups, zero-knowledge, optimization]
sources: [k21-recalibration-brief]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-k21-recalibration-20261111/BRIEF.md
source_url: local
---

# Recalibration round brief: the Sherlock chip at k = 21

## Summary

The owner moved the node and the decider to k = 21 and asked for the design and every optimization, lookups included, to be recalibrated, then for six named optimizations to be examined. The brief also makes zero knowledge end to end a hard requirement ([[ZeroKnowledge]]).

## Key Claims

- Node and decider are k = 21 circuits: 2,097,146 usable rows (6 blinding rows). Leaf k, slice length S, node arity, leaves per node proof and leaf cores are derived, not kept.
- Outside constraints: the ledger 9 verifier and the Midnight ceremony SRS (k = 1 to 25); we build our own proof server.
- Part 1 recalibrates gates (usable rows minus tables minus named margins), leaf shape, aggregation, every lookup (including those rejected for node cost), the decider, the rule inventory, time and memory.
- Part 2 examines profile-guided superinstructions, straight-line mode, fixed-base tables, SHA-256 spread tables shared with RANGE, compute once and consume as a GP-2 claim, and a pre-admitted Compact standard library.
- Part 3 designs zero knowledge end to end: hiding public statement, the `n_steps` rule, zero-knowledge proofs, elastic proving with no plain witness on helper hosts, fresh blinders in production, parity with Compact disclosure, and a simulator argument.
- Area A estimates 241,000 to 254,000 node rows per added column core (estimated) and prefers row cores.
- A measurement lane runs node and decider at k = 20 and k = 21 and verifies a k = 21 proof through the ledger 9 verifier path.

## Key Quotes

- Owner: "Alright expand to K=21 and recalibrate our design and all optimizations under it including the lookups and then examine these".
- Owner: "we need to make this zero knowledge or else its useless".

## Connections

[[AnimationV2]], [[ZeroKnowledge]], [[ChipLeaf]], [[IvcNode]], [[RunDecider]], [[BlockCache]].

## Contradictions

The gates node at most 950,000 rows and decider at most 1,000,000 rows (sherlock-vm-consensus `design.md:105`) were set for k = 20 without a derivation; this round replaces them.
