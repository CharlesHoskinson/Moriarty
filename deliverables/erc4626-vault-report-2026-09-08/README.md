# ERC-4626 collateral-vault report intake

Added the supplied **ERC-4626 and Collateral Vaults: A Research Taxonomy and Technical Atlas** as SRC-0104. The complete Markdown report is retained unchanged; its taxonomy is graphed and compared with Moriarty's language design and the four supplied DeFi papers.

- [Original report](../../.raw/captured/21f40f3a7faa56406001dd783c4b9d33ae1c25751ffba875a198d75c716993ba.md) and [source/hash record](source.json).
- [Taxonomy extraction and critical analysis](ANALYSIS.md): A–H architectures, T01–T11 facets, R001–R014 reported results, mechanisms, open questions and evidence limitations.
- [Moriarty design comparison](DESIGN-IMPLICATIONS.md) and [crosswalk](crosswalk.csv): six proposed vault-specific tests supplement the earlier twelve.
- [Interactive graph](graph.html), [JSON](graph.json), [full extraction](extraction.json) and [graph report](GRAPH_REPORT.md).
- [Four-paper library and graph](../defi-taxonomy-papers-2026-09-08/README.md).

The main addition is a separation between accounting value, redemption value, market value and stressed liquidation value. Interface conformance or local accounting cannot establish safe downstream borrowing without a specified valuation, liquidity and loss-allocation policy.

This is **secondary research input**, not verified primary evidence for every external assertion. All 383 source lines were read locally. The report's opaque citation markers and `sandbox:` links do not provide the originating source map or attachments. Its claimed 80-source atlas, 15-project catalog, CSVs, JSON, BibTeX and ZIP were not supplied or found at the corresponding Desktop filenames. This intake does not recreate them or count them as acquired sources.

The retained local ERC-4626/ERC-7540 snapshots support a limited method/lifecycle comparison. Other standard-status, product-version, incident and theorem claims remain attributed to the report. No new network request, incident replay, financial implementation, K proof or Midnight transaction occurred. Capture operation: `moriarty-vault-report-capture-20260908`; synthesis operation: `moriarty-vault-report-ingest-20260908`. Proposed language requirements remain S2 and preserve all existing sprint/acceptance gates.
