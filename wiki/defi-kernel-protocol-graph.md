---
id: moriarty.defi.protocol-graph
type: comparison
title: NEAR, Hyperliquid and Tron protocol graph for the DeFi kernel
status: active
updated_at: 2026-09-12T01:20:00Z
sources:
  - src-423b5d0e3d19a89746de
  - src-98daf7062af590c4e1f0
  - src-10b1eb20e779608749b2
created: 2026-09-12
updated: 2026-09-12
tags:
  - moriarty
  - defi
  - multichain
---

# NEAR, Hyperliquid and Tron protocol graph

Source material for the Moriarty DeFi kernel interface. Thirteen repositories were cloned at pinned commits, scoped to five topics, and built into one knowledge graph. Structural extraction covered code; semantic extraction over documentation used grok 4.6.

## What was built

| Measure | Value |
|---|---|
| Repositories | 13, pinned commits recorded |
| Scoped corpus | 695 files, 5.5 MB |
| Structural graph | 6,676 nodes, 18,499 edges |
| Semantic graph | 1,731 nodes, 2,413 edges, 51 group relationships |
| Combined graph | 8,389 nodes, 17,212 edges, 501 communities |

The corpus is scoped, not complete: files were selected by topic evidence, so absence from the graph is not absence from the protocol.

## What the graph shows

The three systems converge on staking and validator discipline, and diverge everywhere else. After removing single-letter and generic type-name collisions, only nine edges join nodes from different protocols, and every one of them is an inferred semantic similarity rather than a structural link. They pair Tron permission thresholds with NEAR key authorisation, Hyperliquid staking and unbonding with NEAR stake accounting, and Hyperliquid validator jailing with NEAR validator kickout.

The absence is the finding. These protocols share no intent format, no fee representation and no settlement interface. Anything Moriarty builds across them is its own construction, not an adoption of a common standard.

## Topic hubs

Highest-degree nodes per topic came out as account identity and token identity for intents, energy cost accounting for fees, delegation and reward stores for tokenomics, attestation and foreign-chain request types for signing, and precompiles and bridge contracts for multichain coordination.

## Honest limits

- 2,350 edges reference endpoints that never materialised as nodes, because semantic extraction named code symbols under identifiers the structural pass did not produce. Those edges are dropped from the built graph.
- 1,462 edges collapsed onto endpoint pairs already present.
- Community labels are hand-written for the fourteen largest communities and derived mechanically for the remaining 487.

## Where the work lives

The graph, its report and the interactive view are outside the repository at `~/protocol-graph/graphify-out/`. The scoped corpus is at `~/protocol-graph/corpus/`, and the clones at `~/protocol-graph/repos/`.
