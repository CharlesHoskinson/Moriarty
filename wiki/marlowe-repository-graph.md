---
id: marlowe.repository.graph
type: component
title: Marlowe repository and documentation graph
status: active
updated_at: 2026-09-02T23:18:37Z
sources:
  - SRC-0009
  - SRC-0011
  - SRC-0012
  - SRC-0013
  - SRC-0014
  - SRC-0015
created: 2026-09-02
updated: 2026-09-07
tags:
  - moriarty
  - research
---

# Marlowe repository and documentation graph

## Reproduced scope

**CLM-0100.** The 2026-09-02 Scrapling organization snapshot enumerates 36
`marlowe-lang` repositories. Each repository in
`evidence/repository-locks-2026-09-02.tsv` was fetched with fast-forward-only
and pinned to its full default-branch commit. The exact tracked-HEAD staging
tree contains 5,912 files. The source trees have no tracked modifications;
17 worktrees contain untracked research-generated Graphify output/cache files,
which are not part of the pins. Source: SRC-0012 and the lock TSV; observed 2026-09-02;
authority primary repository metadata/code; scope repository snapshot;
evidence repository observation; reproduced; confidence high; status S6 for
the snapshot process, not for every repository's product lifecycle.

**CLM-0101.** The live documentation crawl acquired 100 of 100 sitemap paths
with Scrapling. The sitemap named the dead host `play.marlowe.iohk.io`; the
same paths were captured from `docs.marlowe-lang.org`. Source: SRC-0011,
manifest fields `sitemap_url_count`, `successful`, `failed`, and
`host_rewrite`; observed 2026-09-02; authority primary descriptive; scope
current documentation; source fact and experiment observation; reproduced;
confidence high; status S6.

## Graph construction

The combined graph joins six independently preserved extraction layers:

| Layer | Input coverage | Nodes | Edges | Hyperedges |
|---|---:|---:|---:|---:|
| Tracked repository AST | 5,912 files staged; supported source parsed | 6,432 | 11,006 | 0 |
| Repository prose | 639 documents | 1,232 | 1,039 | 87 |
| Image path index | 783 images | 783 | 0 | 0 |
| Live documentation | 100 pages | 300 | 302 | 21 |
| Official online entry points | 8 documents | 70 | 79 | 9 |
| User taxonomy report | 25-page PDF text | 75 | 71 | 3 |

**CLM-0102.** After stable-ID coalescing and edge validation, the graph contains
8,864 nodes, 10,888 undirected edges, 120 hyperedges, 1,840 detected
communities, and explicit membership for all 36 repositories. Source:
`graphs/marlowe-org-full/graphify-out/.graphify_analysis.json`, derived from
SRC-0009 and SRC-0011 through SRC-0015; build date 2026-09-02; authority
experimental derived artifact; scope research graph; experiment observation;
reproduced; confidence high for counts and medium for semantic relationships;
status S3.

The interactive graph is aggregated to community level because the node count
exceeds 5,000. The full node-level graph remains in `graph.json`. The token
benchmark estimates a 443,200-word corpus as approximately 590,933 naive
tokens and an average graph query as approximately 7,059 tokens, an 83.7-fold
reduction. This is a retrieval-cost estimate, not an accuracy metric.

## Health and limitations

**CLM-0103.** Diagnostics found zero missing-endpoint edges and zero self-loops,
but rejected 1,510 dangling-endpoint edges and collapsed 265 same-endpoint
relations when producing an undirected simple graph. These are material graph
health warnings: the current artifact must not be used to claim complete call
or import coverage. Source: graph `DIAGNOSTICS.md`; build date 2026-09-02;
authority experimental; scope extraction pipeline; reproduced; confidence
high; status S3.

Three R source files were unsupported; two TSX inputs produced partial syntax
errors; one `marlowe-doc/tsconfig.json` was invalid. Twenty-four demonstration
fixture paths containing keys or addresses were excluded from semantic prose
extraction. The 783 images are path-indexed only and have not received visual
interpretation. `marlowe-runner/e2e/artifacts/**` was excluded as generated
test output. Graphify package 0.9.53 also warns that the installed skill text is
version 0.9.48; the package API and CLI behavior used for this build are 0.9.53.

Semantic `similar_to` and cross-repository connections are inferred discovery
leads unless a pinned source explicitly states them. They are not proof of
architecture, runtime correspondence, or formal equivalence.

The compact hash-addressed build record is
`evidence/marlowe-org-full-graph-2026-09-02.json`.
