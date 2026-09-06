# PCD report: graph and revised Moriarty plan

The report has been preserved, read in full and graphed. Its useful claim/binding
and witness-handoff recommendations now revise the actual semantic, interface
and implementation plans. Mandatory PCD, finite bounds and ACTUS/DeFi targets
remain requirements. The user's Midnight clarification is supported by pinned
native recursion/IVC source; application/ledger compatibility remains to test.

- [Interactive report graph](graphify-out/graph.html): 73 concepts, 110 edges,
  8 communities and 3 extracted hyperedges. Search nodes, select a community,
  or click a node to inspect its source locator. The HTML loads the pinned
  vis-network library from unpkg; its interactive view needs internet access.
- [Graph JSON](graphify-out/graph.json), [plain-language graph report](graphify-out/GRAPH_REPORT.md)
  and [health check](graphify-out/graph-health.json). No missing/dangling endpoints,
  self-loops, duplicate edges or collapsed relations were found.
- [Decisions and revised R0–R6 plan](../../docs/research/2026-09-06-pcd-report-integration.md).
- [Midnight native recursion findings](../../docs/research/2026-09-06-midnight-native-recursion.md).
- [Runnable developer mock](../../experiments/moriarty-developer-mock/README.md).
- [Original report](../../raw/reports/pcd-2026-09-06/PCD.md) and
  [receipt](../../raw/reports/pcd-2026-09-06/receipt.json).

## What changes next

1. Completed: first local mock with visible required claim types (all real evidence unavailable).
2. Implement the shared loan/swap semantic slice, canonical ClaimSpec → manifest
   → intent → BoundClaim construction, and independent intent/effect checking.
3. Test native Midnight IVC, then its actual ledger proof/key boundary separately.
4. Test private witness handoff and bounded split/join history compliance.
5. Connect checked contract properties and expand ACTUS/DeFi coverage.
6. Evaluate optional acceleration only if it preserves every mandatory check.

R3/R4 successes are scoped cryptographic/ledger compatibility experiments.
Full Moriarty acceptance remains unavailable until all mandatory properties and
proofs are enforced. Neither a native IVC example nor a browser mock closes that gate.

## Reading the graph correctly

This is a graph of one secondary report, not an independently verified literature
graph. EXTRACTED means the relationship appears in the report; INFERRED means an
analyst inferred it. Original citation tokens are unresolved. Primary follow-up
and the Midnight correction are in the linked synthesis, not silently rewritten
into the raw report. The graph's Compact restriction must not be read as absence
of Midnight backend recursion.

From this directory, using the existing Graphify interpreter:

```sh
/home/charl/Moriarty/.venv/bin/graphify query 'Cross-party witness handoff' --budget 1000
/home/charl/Moriarty/.venv/bin/graphify query 'Signed mandatory-claim root' --budget 1000
```

[Saved handoff query](graphify-out/query-witness-handoff.txt) and
[mandatory-claim query](graphify-out/query-mandatory-claims.txt) demonstrate retrieval.
The report contains 10,174 detected words. Extraction used one semantic agent;
actual token usage is unmeasured. Graphify's
[corpus benchmark](graphify-out/benchmark-corpus.txt) estimates text reduction
using generic software questions, not answer quality or actual compute savings.
The initial default benchmark estimated corpus size from node count; it is
retained as [unqualified initial output](graphify-out/benchmark.txt), not used as
the measured report length. Installed Graphify 0.9.53 reports a 0.9.48 skill
version mismatch; extraction, integrity, query and browser checks succeeded
without changing shared tooling.

## Review disposition

Independent review caught a potential self-referential intent/claim commitment.
The revised design separates ClaimSpec from BoundClaim and defines the hash
order explicitly. It also distinguishes proof-feasibility tests from full
acceptance before contract certificates are connected. Both were corrected.
No native proof campaign, new consensus rule or foreign prover integration ran.
