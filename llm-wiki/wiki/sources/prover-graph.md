---
title: "Knowledge graph of the Midnight prover stack"
type: source
tags: [graph, graphify, ledger9, prover]
sources: [prover-graph]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/prover-graph-20261111/graphify-out/GRAPH_REPORT.md
source_url: local
---

# Knowledge graph of the Midnight prover stack

## Summary

A graphify knowledge graph over `repos/midnightntwrk` (the graph root): `midnight-ledger` on branch `ledger-9` with the proof server, `midnight-zk` (proofs, circuits, curves, aggregation) and `midnight-zkir`. Built 2026-11-11 for the elastic chip round, whose area C reads the ledger 9 proof server through it ([[elastic-chip-brief]]). Only the summary of the report was read. The folder has no `manifest.json` and no `cost.json`.

## Key Claims

- Scope: 893 files (report corpus check).
- Size: 21,437 nodes, 57,968 edges, 684 communities (483 shown, 201 thin omitted), 9 hyperedges.
- Extraction: 94 percent extracted, 6 percent inferred (3,211 inferred edges, average confidence 0.71), 0 percent ambiguous.
- Cost: 444,000 input tokens, 0 output (report).
- `graph.json` records `built_at_commit` `4b5697f7d`, a Moriarty commit, not a commit of the graphed repositories.
- Health warnings: the corpus check reads "893 files · ~0 words"; the report lists 20 import cycles, all one-file cycles (in `midnight-ledger` and `midnight-zk`).
- God nodes: `Layouter` (1,131 edges), `F` (549), `AssignedNative` (397), `ConstraintSystem` (261), `AssignedBit` (254), `ZkStdLib` (213).
- Hyperedges include versioned proving and verification dispatch, the fee pricing pipeline, the ZKIR proving flow and the `ZkStdLib` prove and verify flow.

## Key Quotes

- "893 files · ~0 words. Verdict: corpus is large enough that graph structure adds value."

## Connections

[[elastic-chip-brief]], [[compact-graph]], [[MidnightLedger9]], [[elastic-chip-gpu-fable]].

## Contradictions

- The word count of 0 sits beside 21,437 nodes; the source does not explain it.
