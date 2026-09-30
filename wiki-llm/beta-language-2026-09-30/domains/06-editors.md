---
title: Real editor tooling for the Moriarty authoring beta
status: research-complete-recommendation-unreviewed
created: 2026-09-30
updated: 2026-09-30
scope: authoring-beta-design
---

# Real editor tooling for the Moriarty authoring beta

**Recommendation:** ship a `.mori` language extension and a bounded stdio language server around the same TypeScript analysis service used by the CLI. Begin with lexical highlighting, snippets, comments and brackets, then add real diagnostics, completion, hover, declaration navigation, document symbols and formatting as their services become callable. Advertise each implemented capability from its registered handler. Keep Langium as a measured workbench alternative, consistent with [domain 08](08-meta-tools.md); defer independent proprietary analyzers and a second authoritative grammar.

This is a research recommendation for the five-reviewer candidate. No extension, LSP server, framework migration, production implementation or IDE activation was performed here. An editor feature cannot establish financial validity, native proof qualification or Midnight acceptance.

## Evidence boundary and current repository

**Repository observations:** inspected checkout `870f998b36ecda04622fa4274132e74902942d0b`; loaded `moriarty-dev:develop`, read `AGENTS.md`, queried `wiki/index.md` and inspected guarded `status --json`. No pending transactions. Existing campaign status reports stale binding/candidate inputs, missing accounting/live-resource evidence and unresolved operational history. Those gaps do not block this research and are not repaired by editor work. The [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md) makes tools permissionless for external developers; maintainer campaign records must not become extension activation requirements.

The [mockup requirements](../../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md) require eight financial families, precise local/specified/open labels, complete effects and explicit missing evidence. [DECISION-CANDIDATE.md](../DECISION-CANDIDATE.md) proposes one analysis service and an inspectable signing display. Broad readable examples must retain their proposed status when executable services cover only a narrower profile.

The [language package](../../../experiments/moriarty-language/package.json) is private experimental ESM. Its `build` invokes TypeScript checking; it is not presently an installable server package. The [Source/6 frontend](../../../experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts) has bounded text/token handling and UTF-8 byte errors, but its AST does not carry per-node spans. The [diagnostic layer](../../../experiments/moriarty-language/src/diagnostics.ts) has stable codes, primary/related byte spans and judgment ordering. [format.ts](../../../experiments/moriarty-language/src/successor/format.ts) exposes earlier versioned formatters; this does not supply a beta formatter. No editor package or grammar was found in the inspected experiment/docs paths.

**Source facts:** exactly two new full official captures, both HTTP 200, using Scrapling 0.4.15 and the supplied create-only capture script. [editor-sources.json](../editor-sources.json) records exact times, hashes, files, inspected locators and limits. The comprehensive [LSP 3.17 specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/) is normative for the protocol. The [VS Code overview](https://code.visualstudio.com/api/language-extensions/overview) distinguishes declarative editing contributions from programmatic analysis. It reports a September 16, 2026 page date; the receipt leaves publication date unknown because the displayed date's meaning was not independently established. Its `main` extract repeats the article; repetition is not independent evidence.

Other linked official sources below are **search discovery/snippet only**, with no retained complete page or runtime result. Search excerpts can omit version and edition qualifications. Documentation availability is not Moriarty compatibility. No browser rendering, authenticated fetch, endpoint/token workaround, private source upload or cookie retention was used. Existing static-document patterns suffice; no new cookie or site-pattern class needed preservation. Dynamic extension registries, current license terms, platform versions and client feature coverage remain unverified beyond these excerpts.

## Highlighting is several different services

**Source fact:** VS Code's declarative contributions cover TextMate syntax coloring, snippets, comment toggling, brackets, indentation and marker folding. Programmatic features include analysis-driven completion, hover, definitions, errors and formatting. LSP lets a language service communicate with different editor clients. [Overview](https://code.visualstudio.com/api/language-extensions/overview).

**Inference and recommendation:** a TextMate grammar classifies lexical patterns: keywords, numeric literals, comments, strings and punctuation. It must not color an asset reference as financially authenticated because its spelling matches a pattern. Test unterminated strings/comments and keyword-like text inside strings. Use theme scopes rather than prescribing colors. Bundle profile-specific snippets whose comments distinguish executable and proposed examples; expansion is source editing, never validation.

**Discovery source fact:** [VS Code semantic highlighting](https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide) adds classifications based on resolved symbols and can overlay lexical styles. **Recommendation:** add semantic tokens after the shared resolver can distinguish declarations, constants, nominal assets, types and uses. Prefer standard token types/modifiers with fallback scopes. Theme visibility varies; warnings and financial status also need explicit text. Do not advertise semantic tokens because a lexical lexer exists.

**Repository-linked inference:** domain 08's captured Tree-sitter introduction concerns an incremental concrete syntax tree useful on malformed input. A Tree-sitter CST is an editor representation; a compiler AST carries typed meanings, bindings and provenance. Neither makes recovered syntax financially executable. A future Tree-sitter grammar needs corpus parity with the beta grammar; it must not become a separate financial checker or silently accept constructs rejected by the compiler.

## Platform comparison and delivery order

The table is a **recommendation based on official discovery evidence**, not a compatibility certification. Each client/version needs a recorded install and interaction result.

| Platform | Plausible integration | Beta decision and public limit |
| --- | --- | --- |
| VS Code desktop | TextMate/config/snippets plus a thin `vscode-languageclient` adapter to stdio server | Primary packaged client. Build an owned VSIX and test activation, Problems and navigation in the actual extension host before claiming support. |
| VSCodium | Same extension API family; owned VSIX or Open VSX distribution | Secondary smoke target. [Official extension documentation](https://github.com/VSCodium/vscodium/blob/master/docs/extensions.md) describes Open VSX as its preset gallery and Microsoft marketplace restrictions. Do not depend on repackaged marketplace downloads. |
| Cursor | VS Code extension model; distribution can differ | Reuse owned VSIX if a tested version permits it. [Official extensions guidance](https://prod.cursor.com/help/customization/extensions) reports a marketplace proxy and possible publisher differences. Cursor APIs and AI features are optional; do not promise marketplace identity or automatic imports. |
| JetBrains IDEs | Small LSP launcher/file association plugin, or much larger PSI language plugin | Defer full plugin. [SDK LSP guidance](https://plugins.jetbrains.com/docs/intellij/language-server-protocol.html) describes an integration tradeoff and product/module requirements. Pin exact IDE/build/edition; do not announce universal Community support. |
| Neovim | Built-in LSP plus `.mori` filetype/config; simple lexical syntax initially | First available real client check. [Official docs](https://neovim.io/doc/user/lsp) expose LSP configuration/start facilities. This host has Neovim 0.11.6; version-specific commands need testing. |
| Emacs | Small major mode for syntax plus Eglot stdio command | Provide documented configuration later. [Official Eglot manual](https://www.gnu.org/software/emacs/manual/html_node/eglot/) requires a language server and major mode. No Emacs runtime found here. |
| Zed | Native extension declaring language/server; possible Tree-sitter queries | Later integration. [Official extension discovery](https://zed.dev/docs/extensions/languages) describes Tree-sitter and LSP; excerpts also mention a semantic-token-only route. Exact current grammar requirements are unresolved; inspect the full current guide before implementing. |
| Browser Monaco | Browser providers over shared analysis in a worker; optional LSP bridge | Separate web adapter. [Official FAQ](https://github.com/microsoft/monaco-editor/blob/main/README.md) says a VS Code extension does not directly run in Monaco. Node stdio cannot run in a browser; worker imports/bounds require a separate build. |

**Contradiction disposition:** JetBrains' SDK search excerpt describes commercial IDE integration, while its [September 2025 announcement](https://blog.jetbrains.com/platform/2025/09/the-lsp-api-is-now-available-to-all-intellij-idea-users-and-plugin-developers/) describes wider access and retained LSP features after subscription expiry. This is potentially a product/version distinction. The report makes no edition-wide support claim; precise supported builds remain an implementation prerequisite. Zed's excerpts likewise do not settle whether every current extension requires a grammar. Neither uncertainty blocks a VSIX plus portable server.

## Architecture choice

The following is a **decision proposal**, informed by [domain 08](08-meta-tools.md), rather than a framework benchmark. None of these alternatives was installed or implemented in this research.

| Approach | Fit to the beta | Decision and test that could change it |
| --- | --- | --- |
| Lightweight LSP over existing TS frontend | Retains current exact bounds, strict judgments and local financial boundary; requires real spans, symbols and recovery work | Select for first beta. Measure edit latency and reference/completion complexity over bounded realistic files; keep transport out of financial semantics. |
| Langium workbench | Grammar-generated AST and language services can reduce infrastructure work, but migration can change accepted syntax, precedence and error order | Reserve a bounded trial if manual services become costly. Demonstrate parity on the same independently checked syntax/error/Core corpus before replacing the authoritative parser. |
| Separate proprietary plugin/analyzer for every IDE | Deep native UX may benefit a mature language; duplicates parsers, financial checks, packaging and release obligations | Defer. Use thin IDE launchers for the shared server; implement specialized UI only for a named user benefit that the protocol cannot carry. |

The beta needs predictable editing on modest bounded files before incremental parsing. Benchmark full analysis rather than assuming an incremental workbench is necessary. Browser reuse is a separate dependency/build test: shared TS code may import Node-only services, and a worker should receive inert text/artifacts rather than wallet keys or automatic provider access.

## Shared analysis and truthful LSP capabilities

**Recommendation:** expose an inert analysis API taking `(source, profile, sourceIdentity)` and returning syntax/semantic diagnostics, source nodes, declarations/references and available strict artifacts. CLI and IDE consumers call it directly. Separate editor partial syntax from strict compile eligibility: an incomplete edit can produce keywords and known symbols, but never a financial Core candidate with omitted bounds, fees or duties. Successful tolerant parsing must not replace strict parsing/checking. Preserve the strict first-failure judgment while permitting useful additional editing diagnostics.

| Service | Minimal useful beta result | Capability advertised only when implemented |
| --- | --- | --- |
| Diagnostics | Exact stable code, readable message, byte-derived range, related location when supported | Push `publishDiagnostics`; no invented pull diagnostic provider |
| Completion | Reserved words and declarations valid in context; distinguish keyword from symbol items | `completionProvider`; no resolve handler unless implemented |
| Hover | Declared/inferred type, identity/unit orientation, supported-profile status | `hoverProvider` |
| Definition | Resolved binding to its actual source span; null for unknown symbol | `definitionProvider` |
| Outline | Named agreement/action/type/state declarations and ranges | `documentSymbolProvider` |
| Formatting | Deterministic profile-specific formatting preserving comments and semantic projection | `documentFormattingProvider`; empty/no edits on unsafe incomplete input |
| Semantic tokens | Resolver-derived classifications with negotiated legend | Deferred until real resolver service exists |

A beta registry should derive capabilities from registered service handlers and their actual options, with contract tests tying each flag to a callable behavior. Do not advertise rename, workspace symbols, cross-file linking, code actions, incremental sync, formatting, semantic tokens or execute-command merely because the protocol defines them. Initially scope symbols/references to one open document and document this limit. Unknown methods need proper JSON-RPC errors; unknown notifications may be ignored as the protocol allows.

**Source facts:** LSP messages use JSON-RPC with ASCII headers and required byte-counted `Content-Length`, UTF-8 content, initialization/capability exchange and shutdown/exit lifecycle. `Full` sync sends full document text. Positions default to UTF-16; 3.17 permits negotiation. Diagnostic arrays replace prior arrays, and empty arrays clear them. [LSP 3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/).

**Recommendations:** use a pinned maintained protocol implementation where feasible; a hand-rolled framing parser adds avoidable risk. Start stdio, full sync, UTF-16, balanced open/close ownership and a bounded document store. Keep stdout exclusively protocol and stderr redacted logs. Enforce header/message/document/open-document/queue limits before allocating or analyzing untrusted text; distinguish document bytes from larger escaped JSON transport bytes. Preserve declared language bounds rather than weakening them to accommodate IDE editing. Report an oversize document clearly and suppress strict artifacts.

Serialize state updates per document URI; do not read disk over an open editor buffer. Attach document version and analysis generation to every job, discard superseded results server-side, and include diagnostic version where supported. Do not depend on clients supporting version filtering. Clear obsolete diagnostics after a successful change and on close for this single-document server. Handle cancellation without publishing partial success; release buffers and exit after the agreed lifecycle. Define behavior for malformed envelopes, duplicate opens, unknown documents and decreasing versions.

## Source locations, signing and result display

**Recommendations grounded in repository span contracts:** build one indexed converter between half-open UTF-8 byte intervals and LSP zero-based UTF-16 line/character coordinates. Never copy byte offsets into `character`. Index Unicode scalar boundaries and LF/CRLF/CR line starts once per document version; reject invalid surrogates according to the language policy, and map EOF/zero-width diagnostics deliberately. Test escaped Unicode versus literal Unicode, astral characters before an error, combining marks, comments and each newline form. Convert definitions, outlines, formatting edits and related ranges through the same map. Retain original source identity and byte spans on elaborated Core nodes.

Formatting changes source bytes and invalidates source commitments/prepared results where those bind original text. Applying formatting or completion must clear cached signing/evidence state. Signing is an explicit operation outside ordinary language requests: no hover, save, completion or formatting action may submit a transaction. A future IDE command must show exact program/profile/domain, asset identities/scales, participants/recipients, gross debit, fee recipients/caps, minimum net outcome, complete read/write footprint, liability change and residual duties, with source-to-Core links. Distinguish authored limits, solver completion and provider evidence. Never present caller-supplied or unresolved identities as authenticated.

Use independent visible status fields: syntax/type check; local preparation/simulation; proof qualification; authenticated evidence; ledger submission/finality. The Source/6 S0 result remains `PreparedUnqualified`; absent signatures, selected-code authentication, predecessor evidence or proofs must stay named missing evidence. Broad profiles show unsupported/specified/open results. An empty Problems list means no reported analysis diagnostics, not settlement or proof validity. The signing pane must expose recovery assumptions, fee inclusion and the controlling price unit orientation.

## Install, experiments and release claims

**Recommendation:** provide reproducible installation without the maintainer vault: pinned dependencies/lockfile, supported Node engines, emitted JS, an executable `moriarty-lsp --stdio`, profile docs and CLI check/format commands. Prefer a server bundled with the VSIX for the first desktop beta; do not require globally installed TypeScript, source-tree execution or `npx` network downloads during activation. Keep a separately installable server for other clients. Bundle only required artifacts and licenses, and record package/VSIX hashes. The [official packaging discovery](https://code.visualstudio.com/api/working-with-extensions/publishing-extension) describes `vsce package`, VSIX and API engine constraints. Marketplace publication is a later distribution action, independent of local package validation.

**Specified-only experiment plan:** freeze a small corpus with independently written expected diagnostics, symbols and Core projections. Exercise valid transfer/repayment, unsupported family constructs, incomplete editing, duplicate/unresolved names, wrong asset/unit, overflow, malformed Unicode, comments and fees/duties omissions. Compare CLI and LSP strict results on identical bytes; no shared snapshots alone establish financial correctness. Verify formatter semantic preservation/idempotence and comment retention; disable it until that behavior exists.

Run protocol subprocess tests for split/coalesced frames, byte lengths, initialization, capability truthfulness, limits, lifecycle, stale-version races, clears and each service. Then run an actual headless Neovim client with the installed server to observe attach, buffer change, diagnostics, completion, hover, definitions and outline. Retain client/server versions, commands and result artifacts. Node 24.21.0/npm 11.19.0 and Neovim 0.11.6 were observed here; no server existed to attach, so only environment discovery was performed.

Finally install the VSIX into a clean VS Code extension host and demonstrate `.mori` recognition, token scopes, snippet expansion, bracket/comment behavior, live Problems updates, navigation and formatting. Inspect reload/deactivation and package contents. Repeat a smaller smoke in VSCodium/Cursor only when available; mark JetBrains, Emacs, Zed and browser adapters untested until their own checks. Protocol tests cannot establish IDE activation or marketplace availability. Public compatibility statements must name exact tested client/version and features; beta source checks cannot imply native proof or ledger support.
