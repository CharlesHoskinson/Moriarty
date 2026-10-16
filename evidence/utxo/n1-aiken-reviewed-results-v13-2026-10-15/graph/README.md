# V13 source navigation

V13 appends Luna revision 3 to the exact frozen V12 graph.
No old source corpus was detected, reread or re-extracted.
No semantic extraction, provider call, network fetch, installation or wiki mutation occurred.
Graphify `0.9.91` ran local build, cluster, analyze, report and export functions.

| Artifact | Meaning |
|---|---|
| [graph.json](graph.json) | Canonical archival graph. Exact parent payloads plus 76 new nodes and 39 corrected edges. |
| [graph-raw.json](graph-raw.json) | Identical full-fidelity archival copy. |
| [graph-analysis.json](graph-analysis.json) | Derived undirected Graphify analysis export. It is not archival evidence. |
| [graph.html](graph.html) | Bounded community view, with 353 community nodes and 958 cross-community links. |
| [SOURCE-INDEX.html](SOURCE-INDEX.html) | Local clickable source index for all 115 new node and edge citations. |
| [SOURCE-PATH-MAP.json](SOURCE-PATH-MAP.json) | Exact mapping from original source paths to 40 portable source copies. |
| [CITATION-INDEX.json](CITATION-INDEX.json) | Original SHA-256 and line or JSON-pointer location for each new citation. |
| [GRAPH_REPORT.md](GRAPH_REPORT.md) | Graphify analysis, raw cohesion scores, navigation questions and scope qualifications. |
| [GRAPH-HEALTH.json](GRAPH-HEALTH.json) | Diagnostics on the complete raw input. |
| [ANALYSIS-NORMALIZATION.json](ANALYSIS-NORMALIZATION.json) | All 45 historical alias IDs that Graphify omits from analysis. |
| [COMMUNITY-CHANGES.json](COMMUNITY-CHANGES.json) | Derived community assignments. These do not replace archival node metadata. |
| [BUILD-RECEIPT.json](BUILD-RECEIPT.json) | Predicted and measured counts, usage unknowns and pipeline scope. |

The archival graph has 6,177 nodes, 16,856 raw links and one hyperedge.
Every one of the 6,101 parent nodes remains unchanged.
Every one of the 16,817 parent links remains unchanged and retains list order.
The parent hyperedge and original community metadata remain unchanged.
All 20 duplicate raw link objects and five self-loops remain present.
There are 781 repeated undirected endpoint pairs beyond their first occurrence.

The derived analysis has 6,132 nodes, 16,025 edges and 353 communities.
Graphify normalizes legacy aliases, merges parallel endpoint pairs and can omit import self-loops.
All 76 new Luna nodes remain in analysis.
The analysis also retains diagnostics for missing source fields and confidence fields.
The report and HTML use this derived view, not a lossless replacement for raw evidence.

The graph exceeds 5,000 nodes. HTML therefore uses Graphify's bounded community view.
The HTML requires vis-network `9.1.6` from `unpkg.com` when opened.
The CDN script is not bundled, fetched or tested by this publication.
Browser interaction remains UNRUN.
The separate source index requires no JavaScript or CDN.

## Source fidelity and historical failures

The [unchanged R3 delta](frozen/revision-3/semantic-v13-luna.json) contains 76 nodes and 39 authoritative `edges`.
Its legacy `links` alias contains only 11 entries and one stale location.
The stale location belongs to `luna_v13_rust_review_blocks_execution`.
It says `$/decision; $/blockers` where corrected `edges` says `$/eligible_now; $/blocking_conditions`.
Integration uses the complete corrected `edges` array only.
The [root validation](frozen/R3-ROOT-VALIDATION.json) covered nodes and edges, not that legacy alias.
The raw R3 source file remains unchanged, including the defective alias.

Revision 1 partial-read coverage and revision 2 invalid-citation failures remain historical.
They have not been silently promoted or integrated as corrected evidence.
R3's [report](frozen/revision-3/REPORT.md) and [coverage](frozen/revision-3/SOURCE-COVERAGE-v13.json) retain the author's read claims and limits.
The author claims 40 FULL reads: 28 structural JSON reads and 12 text reads.
Publication verified 115 source bindings, 75 line spans and 92 JSON-pointer resolutions.
Pointer resolution and line bounds do not establish semantic support.
Fresh Meta audit must check source fidelity and the changed publication bytes.

Requested semantic model and effort were Luna/high.
Actual provider, engine, effort, token use and cost remain UNKNOWN.
The later [registry receipt](frozen/GRAPHIFY-REGISTRY-RECEIPT.json) reports `0.9.91` as latest at 23:14:50Z.
That bounded October 15 observation does not claim future currentness.

Only V13's 40 cited source copies are supplied as its navigable source corpus.
Historical parent `source_file` paths remain archival references and were not followed.
Old Source5/6 rows never become current Rust passes.
All 20 family records remain `pending-full-family-evidence`.
All Opus judgments remain attributed to their named reports.
No graph edge, community label or suggested question grants proof or ledger qualification.

## Rebuild the derived graph views

The local interpreter used Python 3.13 with Graphify `0.9.91`.
Graphify and its NetworkX and clustering dependencies are not vendored here.
This source snapshot supplies no package installer or automatic update command.
Use an existing compatible Graphify installation for a separately selected local graph rebuild.

```bash
python tools/graphify_pipeline.py
```

Run that command from the leaf root, one directory above this README.
The helper reads only frozen inputs and writes graph views within this leaf.
It does not detect the parent corpus or call an extraction backend.
A rebuild changes report dates or derived bytes and requires refreshed publication hashes and audit.
