# DeFi taxonomy graph

The [interactive graph](graph.html) connects four supplied papers to proposed Moriarty requirements. It contains **152 nodes, 253 directed edges and nine communities**. The [JSON graph](graph.json) preserves node/edge provenance; [extraction JSON](extraction.json) also retains seven grouped relationships (hyperedges). These groups are annotations, not additional pairwise edges. The interactive viewer loads its visualization library from the pinned public unpkg URL on opening; JSON and this report remain available without it.

## Coverage and interpretation

The 140 source nodes select significant concepts from all 80 PDF pages; they include cited-work nodes whose sources were not separately acquired. Twelve additional nodes describe S2 Moriarty requirements. Detailed taxonomy leaves and risk matrices remain in the four [paper analyses](README.md#paper-library) and node rationale fields, so the graph is a navigation aid rather than an exhaustive formal ontology.

Of the 253 edges, 186 are EXTRACTED, 66 INFERRED and one AMBIGUOUS. EXTRACTED means a relationship appears explicitly in a paper; it does not endorse the paper's truth, establish current protocol behavior or independently verify a cited work. Hierarchical `references` edges record how a paper includes/describes a child. They do not universally classify all products. INFERRED includes cross-paper similarity and all connections to Moriarty's proposed design. Confidence values are annotations, not statistical probabilities.

Each node/edge has a vault-relative source file and locator. PDF paths resolve to immutable hash-verified captures. The Zhou→Werner connection identifies the cited work; Zhou's 2021 citation form does not establish that the authors cited the supplied v6 rendition. Separate source nodes for similar concepts preserve differing definitions and source independence.

## Communities

| Community | Label | Nodes |
| --- | --- | --- |
| 0 | Assets, mechanisms and claims | 32 |
| 1 | Credit and liquidation | 23 |
| 2 | Attack powers and atomicity | 21 |
| 3 | Composition and economic security | 18 |
| 4 | Observations and intended properties | 13 |
| 5 | Interest rates and clocks | 12 |
| 6 | Services and strategies | 12 |
| 7 | Deployment, ordering and settlement | 11 |
| 8 | Dependencies and authority | 10 |

Communities are graphify's computed grouping with names assigned after inspecting members. Their concentration around individual papers partly reflects the extraction structure. It is not evidence that these are disjoint financial categories or the optimal language decomposition. Exact cohesion values and confidence counts are in [graph statistics](graph-stats.json).

## Connections worth examining

- **Flash loans → attacker capability → atomic obligation.** Kotzer's repayment rule and Zhou's adversary power meet in Moriarty's proposed rollback/fee test. Legitimate functionality and an attacker's use of it remain distinct.
- **Receipt tokens → shares → residual duties.** Gogol's token designs and Kotzer's vaults motivate accounting beyond wallet balances, including changing redemption value and unfulfilled claims.
- **Oracle manipulation → estimated truth → proof assumptions.** Zhou's dependency layer and Werner's ground-truth distinction show why a valid signature or a proved transition cannot establish market truth.
- **Rate clocks → utilization → liquidation.** Interest policy can change debt and trigger eligibility without a new token transfer. Time units and accrual boundaries therefore belong in financial semantics.
- **ABI compatibility → callbacks → composition.** Similar interfaces can hide different effects and break another protocol's assumptions. The proposed behavior summaries and complete-effect comparison need pinned implementations.
- **Topology → ordering → settlement.** Independent multichain deployments, cross-chain obligations and one-ledger atomic execution carry different assumptions; none implies cross-chain rollback.

Use these connections to select the next concrete financial fixture in [the design comparison](DESIGN-IMPLICATIONS.md), not as a proof that Moriarty already implements the behavior.

## Integrity checks and reproducibility

[Graph health](graph-health.json) reports zero missing or dangling endpoints, self-loops, exact duplicate edges and directed edge collapses. The extracted and built graph both contain 152 nodes and 253 pairwise edges. All graph source files resolve in the completed repository tree. Semantic extraction used three bounded readers and the installed graphify schema; build and clustering used its directed Python API. No graph server, database or whole-repository re-index was added.

Native agent token use was unavailable and is recorded as null in statistics. The extraction schema's numeric zero token fields are placeholders and explicitly annotated as unavailable. No empirical benchmark of graph retrieval quality or token savings was performed. Regeneration needs the retained extraction plus the installed graphify library; re-extraction is a new semantic judgment and is not promised to be byte-deterministic.
