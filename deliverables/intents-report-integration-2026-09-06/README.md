# Intents report: graph and adopted plan changes

The complete user report is preserved under
[raw/reports](../../raw/reports/intents-2026-09-06/intents.md), with its
[receipt](../../raw/reports/intents-2026-09-06/receipt.json).
Read the [reconciliation and revised gates](../../docs/research/2026-09-06-intents-report-integration.md)
for what Moriarty adopts, adapts or leaves out.

- [Interactive graph](graphify-out/graph.html): 78 nodes, 134 edges,
  3 group relationships and 7 communities.
- [Graph JSON](graphify-out/graph.json), [report](graphify-out/GRAPH_REPORT.md),
  [health](graphify-out/HEALTH.txt) and [provenance](graphify-out/provenance.json).
- [Saved paths](graphify-out/query-paths.json) connect gross debit to bounded
  authority, canonical IntentIR and proof-carrying plans; exact-plan signing
  connects to canonical encoding and signed authority.
- [Browser image](graphify-out/graph-browser.png) and
  [browser checks](graphify-out/browser-check.json).

The most connected concepts are bounded authority, canonical outcome IntentIR,
plan acceptance, typed adapters and solver plans. The useful bridge is residual
authority and persistent obligations: unfinished work must carry both forward,
while a pending receipt must not claim a terminal goal. That bridge is labeled
inferred; source-explicit relationships retain line locators in the extraction.

Graph health found no missing/dangling endpoints, self-loops or collapsed edges.
The interactive page needs Internet access for its SRI-pinned vis-network
9.1.6 dependency from unpkg. The image and JSON are available offline.
Tokens were not measured. Graphify's generic token-reduction benchmark is a
heuristic estimate, not a retrieval-quality or billing measurement; the query
also exceeded its requested token budget, as recorded in its receipt.

The graph describes the report, not proved financial semantics. The report's
embedded ZIP/prototype results were not supplied or reproduced. Its optional
proof and other-backend-first suggestions do not replace the user's mandatory
PCD or Midnight target. The [earlier PCD graph](../pcd-report-integration-2026-09-06/README.md)
remains separate evidence. Moriarty's adopted connection is in the reconciliation:
PCD must carry intent refinement, consumed authority and residual obligations,
alongside valid contract transitions.
