# Moriarty repository graph and audit

Produced with Grok 4.6 (returned identity `grok-4.6-build`), with independent root validation. Repository snapshot: main at `89b0c7b9809b501c091223c78f6dd2df8581936c` plus existing uncommitted work.

- [Full repository file map](repository-file-map.html): 58,729 in-scope files; 69,049 file/directory nodes and 71,653 containment/literal-reference relationships. Search and lazy directory expansion work offline. [Coverage and exclusions](FILE_MAP.md), [JSON](repository-file-map.json).
- [Code and evidence graph](graph.html): 15,085 nodes and 29,827 edges, displayed by community. Fresh deterministic AST covers 749 product source files; new Grok semantic extraction covers 703 documents, with 346 historical semantic-cache files reused. [Graph report](GRAPH_REPORT.md), [JSON](graph.json).
- [Repository audit](AUDIT.md): demonstrated capabilities, acceptance gaps, evidence freshness and limits. [Independent checks](ROOT_VALIDATION.md).
- [Hook repair](HOOK_FIX.md): both protocol failures repaired; 176 plugin tests pass and actual host events complete without the prior errors.

The file map is repository-wide navigation, not semantic review of every file. Secrets, build caches and duplicate worktrees are excluded. Compact/K and other unsupported source families remain navigable at file level. The semantic/AST graph dropped 3,378 unresolved raw relationships during construction; that loss remains recorded rather than inferred away. Historical semantic cache entries may come from other models; all new semantic extraction used Grok 4.6.

The language suite passed 682 tests and typechecking. Retained Preview loan/swap financial results are scoped. Mandatory PCD/history acceptance, full ACTUS/DeFi conformance, campaign accounting/binding repair and release gates remain open. No new financial transaction was sent for this audit.

Terminal usage receipts: [main graph](grok-measured-usage.json), [file map](file-map-measured-usage.json). These are reported invocation counters, not a billing invoice.
