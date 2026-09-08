# Collateral-vault report graph

The [interactive graph](graph.html) and [JSON](graph.json) contain **53 nodes and 95 directed edges**, grouped into seven computed communities. [Full extraction](extraction.json) retains one additional hyperedge annotation. The graph has 43 report-concept nodes, four paper-context nodes imported from the [four-paper graph](../defi-taxonomy-papers-2026-09-08/graph.json), and six proposed Moriarty requirement nodes. Shared paper identifiers allow the graphs to be joined without treating the papers as new captures.

## Evidence interpretation

Seventy edges are EXTRACTED from the secondary report. This means the report explicitly states the relationship, not that its standard-status, implementation, incident or theorem assertion was verified. Twenty-five edges are INFERRED: one source-analysis grouping plus cross-paper/design connections. All connections to proposed Moriarty requirements are inferred. No edge proves that an ERC implementation, an economic model or Moriarty is safe.

All A–H architectures, T01–T11 facets, seven reported standards and nine attack mechanisms are nodes. The R001–R014 result ledger, twelve unnumbered research gaps and ecosystem details are retained in the [analysis](ANALYSIS.md) and node rationale attributes. Source paths and line locators point to the unchanged captured report; the four context nodes point to the immutable PDFs. The report's unprovided 80-source atlas and relationship table were not imported or reconstructed.

## Communities

| Group | Name | Nodes |
| --- | --- | --- |
| 0 | Credit architecture and research | 10 |
| 1 | Donation, rounding and execution | 9 |
| 2 | Cross-paper recovery obligations | 7 |
| 3 | Nested strategies and authority | 7 |
| 4 | Asynchronous claims and exits | 7 |
| 5 | Reported standards and accounting | 7 |
| 6 | Valuation and borrowing power | 6 |

Names were assigned after inspecting the computed memberships. Communities depend on extraction choices and are not proved economic categories. The central cross-paper connection is the need to state separate assumptions between accounting, valuation, borrowing, liquidation and recovery. The [design comparison](DESIGN-IMPLICATIONS.md) turns those distinctions into six proposed tests owned by existing sprints.

## Checks and limits

[Graph health](graph-health.json) reports zero missing/dangling endpoints, self-loops, duplicate edges or directed edge collapse. Built and extracted pairwise counts match. [Statistics](graph-stats.json) preserve counts and labels. Token use is unavailable; zero-valued extraction fields are explicitly annotated schema placeholders.

The HTML uses graphify's pinned external visualization library from unpkg when opened. It has been checked statically; no browser interaction or graph-retrieval benchmark is claimed. JSON and this report are usable without that dependency. Use the analysis/extraction for precise locators and rationale beyond the viewer's compact node panel.
