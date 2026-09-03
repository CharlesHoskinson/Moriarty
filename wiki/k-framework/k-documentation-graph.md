---
id: k.framework.documentation-graph
type: source
title: K Framework documentation graph
status: active
updated_at: 2026-09-03T14:43:01Z
sources:
  - SRC-0023
---

# K Framework documentation graph

## What was graphed

The corpus is a copy of the documentation in the pinned K Framework repository (SRC-0023, commit `4a46d1231473b599c699160132fd6e76a5c46406`, v7.1.337): `docs/` (user manual, cheat sheet, ktools, sort inference), the complete `k-distribution/k-tutorial/` tree (sections 1, 2 and 3 with their example `.k` files), the builtin modules under `k-distribution/include/kframework/builtin/`, `INSTALL.md`, `README.md`, `CLAUDE.md`, and the pyk docs (CLM-0300; SRC-0023 graphs/k-framework/corpus; repository observation; reproduced; high; S6). The corpus holds 72 files and about 93,000 words: 68 documents and 4 code files (CLM-0301; evidence/k-framework-docs-graph-2026-09-03.json; experiment observation; reproduced; high; S6).

Semantic extraction ran through the Antigravity CLI (`agy` 1.1.24) with model `gemini-3.8-flash-high` and structured JSON output, in 11 chunks, following the graphify 0.9.53 extraction specification; code files went through graphify's AST extractor (CLM-0302; scripts/run_graphify_k_framework_agy.py; experiment observation; reproduced; high; S6). Community labels were also produced by the same model from each community's node labels (CLM-0302). The run consumed 613,606 input and 282,677 output tokens (CLM-0303; graphs/k-framework/graphify-out/cost.json; experiment observation; reproduced; high; S6).

## Result

| Measure | Value |
|---|---|
| Nodes | 315 |
| Edges | 313 (329 raw edges before merging duplicates) |
| Communities | 45 |
| Largest community | 28 nodes |
| Health | 2 directed and 16 undirected collapsed edges; no dangling, missing or self-loop edges |

(CLM-0304; graphs/k-framework/graphify-out/health.txt and graph.json; experiment observation; reproduced; high; S6)

The collapsed-edge warning means that the extractor emitted 16 pairs of edges between the same two nodes with different relations, which the undirected build merged into one edge; the graph is usable, but relation multiplicity between those pairs is lost (CLM-0305; graphs/k-framework/graphify-out/health.txt; experiment observation; reproduced; high; S6).

## Communities

The ten largest communities and their labels are: Configuration and Execution Semantics (28 nodes), Builtin Domains and Types (23), Haskell Backend Proving (23), Syntax Definition and Parsing (22), Advanced K Concepts (22), Intermediate K Concepts (21), K Compilation Pipeline (19), K Distribution and Packaging (18), Rule Attributes (18), and K Language Fundamentals (15) (CLM-0306; graphs/k-framework/graphify-out/labels.json; experiment observation; reproduced; high; S6). The remaining 35 communities are small topic clusters such as Sort Inference and Subtyping, GDB and LLDB Debugging, Evaluation Contexts and Strictness, Matching Logic Foundations, Symbol and Overload Attributes, and one cluster per intermediate lesson stub (CLM-0306).

Inference: the community structure follows the documentation's own file boundaries more than its concepts, because the intermediate lessons 2.2 to 2.17 are nine-line stubs that redirect to the user manual, so they cluster alone (CLM-0307; SRC-0023 k-distribution/k-tutorial/2_intermediate; inference; reproduced; medium; S6).

## God nodes

| Node | Degree | Source |
|---|---|---|
| pyk Skipped Test Triage | 11 | pyk-docs/regression-triage.md |
| Section 2: Intermediate K Concepts | 10 | k-tutorial/2_intermediate/README.md |
| DOMAINS Module | 8 | builtins/domains.md |
| Bytes Sort | 8 | builtins/domains.md |
| Section 3: Advanced K Concepts | 8 | k-tutorial/3_advanced/README.md |
| K Framework | 7 | CLAUDE.md |
| Lesson 1.19: Debugging with GDB or LLDB | 7 | k-tutorial/1_basic/19_debugging |
| Lesson 1.21: Unification and Symbolic Execution | 7 | k-tutorial/1_basic/21_symbolic_execution |
| String Sort | 6 | builtins/domains.md |
| K Attributes Reference | 6 | docs/user_manual.md |

(CLM-0308; graphs/k-framework/graphify-out/GRAPH_REPORT.md; experiment observation; reproduced; high; S6)

Inference: the hubs a reader must know are the DOMAINS module, the Bytes and String sorts, and the attributes reference in the user manual; the tutorial section indexes and the pyk triage page are hubs only because they link to many pages, not because they carry semantics (CLM-0309; inference; medium; S6).

## Surprising connections and suggested questions

The report's cross-community connections are all inferred edges from install and packaging pages (kup, Docker, Homebrew) to the K Framework hub, plus one semantic-similarity edge between the advanced lesson on scripting K and the pyk pipeline document (CLM-0310; graphs/k-framework/graphify-out/GRAPH_REPORT.md; experiment observation; reproduced; high; S6). The graph flags 104 weakly connected nodes, chiefly command scripts and test scripts in the tutorial directories, and asks whether the Configuration and Execution Semantics community (cohesion 0.08) should be split (CLM-0311; graphs/k-framework/graphify-out/GRAPH_REPORT.md; experiment observation; reproduced; high; S6).

## Outputs

The interactive graph, raw graph, report, labels and health check are at `graphs/k-framework/graphify-out/graph.html`, `graph.json`, `GRAPH_REPORT.md`, `labels.json` and `health.txt`; `graphs/` is gitignored, so these are regenerated with `scripts/run_graphify_k_framework_agy.py` after staging the corpus as described in that script (CLM-0312; scripts/run_graphify_k_framework_agy.py; repository observation; reproduced; high; S6). The summary numbers are preserved in `evidence/k-framework-docs-graph-2026-09-03.json`.

## Limitations

The graph covers documentation only, not the Java, Scala, Haskell or C++ source of K, and not the backends' own docs (CLM-0313; graphs/k-framework/corpus; repository observation; reproduced; high; S6). INFERRED edges are model inferences and carry the confidence score the extractor assigned; they are not source facts (CLM-0313). Query it for navigation across the K docs, for example with `graphify query` run inside `graphs/k-framework`, not for normative statements, which belong to [k-user-manual.md](k-user-manual.md), [k-builtins.md](k-builtins.md), [k-tutorial-basic.md](k-tutorial-basic.md) and [k-tutorial-intermediate.md](k-tutorial-intermediate.md).
