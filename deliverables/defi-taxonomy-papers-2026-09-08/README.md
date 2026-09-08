# Four DeFi papers: library, taxonomy graph and language implications

Four complete PDFs (80 pages) have been added as immutable, hash-verified library captures. This dossier extracts their distinct taxonomies, preserves source inconsistencies, and maps their implications to Moriarty's existing design. All findings are source-version observations or S2 research recommendations; no survey result, financial implementation, theorem or network milestone is reproduced by this intake.

Start with [the language comparison and twelve proposed tests](DESIGN-IMPLICATIONS.md), [the taxonomy-to-design crosswalk](taxonomy-design-crosswalk.csv), or [the interactive graph](graph.html). The [graph report](GRAPH_REPORT.md) states its coverage and limitations; [graph JSON](graph.json) and [full extraction](extraction.json) retain relationships and provenance. Download/open the HTML locally if GitHub only displays its source.

## Paper library

| Source | Paper and detailed analysis | Supplied version/date | PDF pages | Original bytes | Related existing source |
| --- | --- | --- | --- | --- | --- |
| SRC-0100 | [SoK: Decentralized Finance (DeFi) - Fundamentals, Taxonomy and Risks](paper-1-analysis.md) | 2023-06 | 29 | [PDF](../../.raw/captured/1c7211969624807b9a9aaf2308c4a80374816fb9c119d785b0d13437093cc192.pdf) | SRC-0085 |
| SRC-0101 | [SoK: DeFi Lending and Yield Aggregation Protocol Taxonomy, Empirical Measurements, and Security Challenges](paper-2-analysis.md) | Undated supplied PDF; creation metadata April 2026 | 16 | [PDF](../../.raw/captured/7165f6cbe7b86280dd5ed94dd5585d3b55dda34d77776f2d9db87321683475c8.pdf) | SRC-0087 |
| SRC-0102 | [SoK: Decentralized Finance (DeFi) Attacks](paper-3-analysis.md) | 2023 | 18 | [PDF](../../.raw/captured/863c0ee080bac271b9ff1e3a0f1e036759cf71baf9c4b1d3dd38b963cbde4979.pdf) | New work in this library |
| SRC-0103 | [SoK: Decentralized Finance (DeFi)](paper-4-analysis.md) | 2022-09-15 | 17 | [PDF](../../.raw/captured/9079375c121a8033457a952b58b5dc2fb6f2ad9d94ba9ba247b1994b127c5b2c.pdf) | SRC-0095 |

The [source register](sources.json) contains authors, original basenames, hashes, page counts and bibliographic caveats. Each analysis gives 1-based PDF page citations; printed page numbers can differ. The [source inventory](../../evidence/source-inventory.csv) and portable ledgers retain both new capture IDs and old capsule IDs.

## Version and evidence cautions

- SRC-0100 is the June 2023 Gogol draft, with a placeholder DOI. SRC-0085 describes a later 2024 version; classifications and sample percentages must not be silently merged.
- SRC-0101 exactly matches the original PDF payload hash recorded in SRC-0087. It adds full retained bytes and reading coverage, not an independent study.
- SRC-0102 prints different DOI strings in its side banner and footer. Both are preserved; the supplied PDF hash defines this intake.
- SRC-0103 is Werner arXiv v6, the same work/version as SRC-0095's HTML capsule. New rendition does not mean independent corroboration. Zhou also explicitly cites Werner; shared concepts do not establish independent validation.
- All figures and empirical results describe the papers' historical samples. No current protocol parameters, loss prevalence, APY, TVL or safety claims were verified online.

## Principal language findings

Keep economic family, mechanism, asset/claim, time/funding, collateral/loss, settlement, authority and threat as separate dimensions. A loan can be both overcollateralized and free of early liquidation. A flash loan is an atomic liquidity capability, not inherently a vulnerability. A token can represent a changing redemption entitlement rather than a fixed unit of underlying value.

The security model needs more than code-bug labels: an adversary's knowledge and capabilities, the vulnerable layer, the action order and the violated property belong in each test. ABI compatibility alone does not establish semantic compatibility. Formal accounting and authorization claims still depend on explicit oracle, custody, liquidity, sequencing and finality assumptions.

This extends the existing [family/facet taxonomy](../../wiki/defiformal-taxonomy.md), [security boundaries](../../wiki/security.md) and [surface/semantics proposal](../defi-language-design-2026-09-07/LANGUAGE-DESIGN.md). Full EBNF, executable Moriarty K semantics and proof/ledger correspondence remain owned by the [existing sprints](../../openspec/sprints/README.md).

## Method and reproducibility

The four user-supplied PDFs were processed locally with Poppler `pdftotext -layout`; PDF page boundaries came from form-feed separators. Three bounded source readers covered all 80 pages, including references, and inspected the figures/tables listed in each analysis. Root review checked source/version relationships and the design mapping. Detailed full-text intermediates and rendered page images are local scratch; the durable PDFs permit re-extraction.

Capture operation: `moriarty-defi-pdf-capture-20260908`. Coupled wiki ingest: `moriarty-defi-taxonomy-ingest-20260908`. Network requests: zero. Token use is unavailable from the native worker interface, not measured as zero. The graph uses the installed graphify library with directed relations; it is a scoped graph of these papers and design connections, not a rebuild of the whole repository. Semantic judgments and confidence scores are researcher annotations, not calibrated probabilities.
