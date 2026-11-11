---
title: "Knowledge graph of the Compact repository"
type: source
tags: [graph, graphify, compact]
sources: [compact-graph]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/compact-graph-20261111/graphify-out/manifest.json; ~/Projects/Moriarty/research/compact-graph-20261111/graphify-out/GRAPH_REPORT.md
source_url: local
---

# Knowledge graph of the Compact repository

## Summary

A graphify knowledge graph of the Compact compiler repository (`repos/LFDT-Minokawa/compact`, the graph root recorded in `.graphify_root`), built 2026-11-11 for the Compact feasibility and cost rounds ([[cost-round-brief]] names it as a routing source). Only the manifest and the summary of the report were read for this page.

## Key Claims

- Scope: 1,026 files, about 438,191 words (report corpus check); the manifest lists 909 file entries with mtime and AST and semantic hashes.
- Size: 6,907 nodes, 13,202 edges, 356 communities (268 shown, 88 thin omitted), 27 hyperedges.
- Extraction: 87 percent extracted, 13 percent inferred (1,773 inferred edges, average confidence 0.66), 0 percent ambiguous.
- Cost: 1,147,000 input tokens, 0 output; "semantic pass ran as 9 Opus subagents" (`cost.json`).
- `graph.json` records `built_at_commit` `4b5697f7d`, a Moriarty commit, not a commit of the Compact repository.
- Health warnings: the corpus check warns "Large corpus ... Semantic extraction will be expensive"; six import cycles are reported (two one-file Rust cycles in `tools/compact`, one two-file Rust cycle, three TypeScript cycles in `runtime/src`).
- God nodes: `defineCompileTest()` (352 edges), `defineRuntimeTest()` (176), `createTestContract()` (155), `run_command()` (150), "Compact compiler notes" (111).
- Hyperedges relevant to Sherlock: boundedness enforced by loop unrolling, inlining and recursion rejection; ZKIR v3 backend rollout; cross-contract call safety checks.

## Key Quotes

- "Large corpus: 1026 files · ~438,191 words. Semantic extraction will be expensive (many Claude tokens). Consider running on a subfolder."

## Connections

[[cost-round-brief]], [[prover-graph]], [[cost-lane-astra]].

## Contradictions

- The manifest holds 909 entries against 1,026 files in the corpus check; the source does not explain the difference.
