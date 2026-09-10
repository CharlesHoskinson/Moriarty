# TypeScript, Elm and Unison language-design research

Status: **S2 provisional research**, after independent PL convergence. Thirteen complete academic documents and fourteen substantive official pages inform twenty recommended feature positions. This does not implement language features or establish developer usability, proof correctness or financial settlement.

Start with the [report](REPORT.md) and [feature checklist](FEATURE-CHECKLIST.md). The [full matrix](feature-matrix.csv) and [JSON](features.json) include rationale, finite-resource tradeoffs, current Moriarty gaps, syntax sketches, minimum proposed tests and confidence. Every test is specified-only.

The [paper atlas](PAPER-ATLAS.md) gives methods, results, locators and limitations. [Source inventory](SOURCE-INDEX.md), [source manifest](source-manifest.json), [repository pins](repository-pins.json) and immutable [raw captures](../../raw/sources/language-design-2026-09-09/) identify the evidence. The [readable graph](GRAPH.md) and [typed graph](research-graph.json) connect sources to claims and proposed features.

The recommended initial path is a standalone `.mori` language with TypeScript-style syntax. Inert tagged-template integration is a possible later adapter. Unrestricted TypeScript execution is excluded. Current Elm commands/subscriptions are distinguished from historical FRP; Unison identity and effects are adapted under explicit bounds and authority.

The branch starts at `9157a29` and contains only dated research outputs before integration of reviewed main. Raw acquisitions are immutable; corrected extractions use new files. Failed access attempts remain visible. No package installs, language/proof/network-financial runs or canonical wiki mutations occurred in this stream. PL review is expert critique, not an empirical developer study.

[PL convergence](CONVERGENCE.md) preserves votes, amendments and dissent. The [complete proposed payment fixture](PAYMENT-FIXTURE.md) shows authority, custody, identified duties, complete effects and residual work; it is not admitted grammar or an execution result.
