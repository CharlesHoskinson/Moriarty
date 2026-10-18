# First normal Rust MCP capability — source-only design proposal

Status: proposed for independent design/dependency/resource review; not adopted, implemented, built, run or product-qualified. Source basis is Moriarty main commit `74a909d69639df8823a527a4d608a7516ccd69a2`, tree `f18c2346ff435f9ac95a7f6f56457c2edde3f71c`. Exact blobs are preserved under `repo-source/` and hashed in `REPO-SOURCE-PINS.json`. Mutable main and the dirty LSP worktree are not substituted.

## Useful capability and falsifiable predicate

Expose exactly one normal MCP tool, `mori_check_source`, through the existing public `mori mcp` command. A client supplies a source string and optional display label; it receives the same check acceptance decision and complete profile-specific diagnostics that the current `mori check --json` consumer produces. Source rejection is a successfully delivered check result. The result says `source-check-only`; it makes no compilation, simulation, proof, authentication, ledger or settlement claim.

The acceptance predicate is: for each frozen source and file label, the MCP result selects the same profile consumer as `mori check`, has the identical accepted/rejected decision and diagnostics, retains the byte-identical source digest and Core/2 qualification, and is reachable by an ordinary MCP client through `mori mcp`. A helper called only by tests cannot satisfy it.

## Inspected current path

At the pinned commit, `main.rs::main` parses `cli::Cli`, constructs `Shell`, and calls `commands::run`. `cli.rs::Command::Mcp` is advertised as a read-only stdio MCP server. `commands/mod.rs::run` calls `mcp::run`, whose complete body is `todo!("mori mcp")`. There is no working Rust MCP capability in that body. The independently changing LSP candidate is outside this design.

The actual public check path is `mori check FILE [--json]` → `commands::check::run` → `core_io::read_source`. Its routes are:

| Route in exact source | Actual callable consumer | Existing diagnostic representation |
| --- | --- | --- |
| first token is not `language`, including lexical/profile errors | `mori_parser::parse`; on success `mori_checker::check` | `json::CheckReport::new`, with all diagnostic codes, severity, message, help, labels and UTF-8 byte spans |
| first token is `language` and `mori_core::language` does not identify `/2` | `mori_core::parse`, which validates Core/1 | `mori_core::Error { code, message, offset, step }`; unavailable locations remain null |
| first token is `language` and supported language is `moriarty/2` | `mori_core::stage::load` | the same Core error type, plus exact `STAGE_LABELS`: local-reference-stage / none / untracked |

`language()` is routing, not admission. Unknown `language` profiles follow the existing Core/1 parser error path. An MCP caller cannot override the source header. `checked_profile` names the selected consumer profile, not a claim that an invalid declaration was supported. Beta and Rust Core reports are distinct from Node Source/6 and experimental Core/5; this tool does not invoke those consumers or lower source into them.

## Proposed connection to the public consumer

Retain the existing command entry point. Replace its stub only after the separate decisions below are adopted. The proposed call path is:

`main → commands::run(Command::Mcp) → mcp::run → official rmcp stdio service → ServerHandler::call_tool("mori_check_source") → shared check::analyze_source → the exact parser/checker/Core route above → typed diagnostic projection → rmcp::model::CallToolResult`.

Extract the existing branch selection and analysis into an internal `check::analyze_source(file_label, source)` operation, consumed by **both** `check::run` and the MCP handler. This operation does not read a file, print, create a process, simulate, compile, prove or access a ledger. The CLI remains responsible for its current bounded file read, human diagnostics, timing, output and exit status. The MCP handler supplies the source string directly and serializes a diagnostic projection. Keep existing `CheckReport::new` and `MoriDiagnostic` rendering as the beta source of truth. Do not copy the checker into the MCP module, spawn a second `mori` process, write temporary source files, or capture the CLI's stdout to manufacture a result.

The internal analysis must retain sufficient owned data for the existing human CLI path, existing full JSON report and MCP projection. Beta `Checked` borrows its AST, so build the owned `CheckReport` while the AST is alive; retain original `MoriDiagnostic` values for human rendering. Core/2 admission must keep calling `stage::load`, rather than its syntax-only parser. The implementation author must demonstrate CLI before/after report equality, including both successful and failed staged labels. This is a proposed shared interface, not a claim that `analyze_source` exists today.

## Request and response contract

`INPUT-SCHEMA.json` and `OUTPUT-SCHEMA.json` are the complete proposed tool payload schemas. `INTERFACES.md` defines types and exact field semantics. `examples/` contains complete JSON request/response values, not transport implementations.

Inputs are `source: string` and optional `file: string`, default `mcp-input.mori`. The file is an uninterpreted display label; no filesystem read follows it. Preserve exact UTF-8 bytes, comments, newlines and whitespace. Do not normalize before hashing or checking. The existing public CLI reads at most 1 MiB; the lexer independently limits source to 65,536 bytes and 8,192 tokens. Propose preserving those bounds for source input rather than introducing a larger language scope. A post-decode source string above 1 MiB yields a caller-visible `source_too_large` tool error. At or below that cap, use the real consumer so its lexical bounds and errors remain observable. JSON Schema string length is not a UTF-8 byte bound; the implementation must not substitute it.

A normal check result contains `format`, `kind=check`, `scope=source-check-only`, file label, lowercase SHA-256 of exact source bytes, selected `checked_profile`, status, **all** diagnostics in their original order, and nullable staged qualification. Beta diagnostics retain every label and primary flag; spans stay UTF-8 byte ranges, not UTF-16 positions. Core errors retain null offsets/steps; invent no range, severity or source location. Do not truncate additional diagnostics or rewrite codes to fit beta syntax. `stage_qualification` is the exact three Core/2 labels as one object, including on rejection; it is null on the other routes.

`CallToolResult::structured` supplies both machine `structuredContent` and SDK-generated JSON text content. Both accepted and rejected check results use `isError=false`: the tool delivered the requested diagnostic report. The explicit source-input cap uses `CallToolResult::structured_error` with `kind=tool_error` and `isError=true`. Wrong argument types, missing source, unknown argument keys or unknown tool names use the SDK's ordinary JSON-RPC error types; they do not become fabricated compiler diagnostics. Exact human error prose in the invalid-argument example is a proposed handler message, not a captured SDK message. There are no resources, prompts, subscriptions, sampling, elicitation or transaction tools in the first capability.

## Standard protocol and dependency proposal

Recommend the official Rust SDK over a handwritten loop. The inspected public crate is `rmcp` 3.5.1, source VCS `79437f291b2c44053d00dcd5db969fd0cca7c887`, package SHA-256 `ab5023804cbe0304cbe06197ac16e645a0a0d0ffa2953bf735c15c2fa0213350`. It was fetched with Scrapling and matched to crates.io metadata; no SDK code was imported or run. `DEPENDENCY-REVIEW.md` identifies exact APIs and feature consequences. The recommendation is not dependency adoption.

Use the SDK's `ServerHandler` methods, `ServerConfig`, `Tool`, `ListToolsResult`, `CallToolRequestParams`, `CallToolResult`, `ServiceExt::serve`, `RunningService::waiting`, and `transport::stdio`. For one tool, manual implementation of the SDK handler and explicit serde input types avoids requiring its optional tool macros. This is normal SDK integration; do not write JSON-RPC dispatch, line framing, request-ID correlation, initialization, discovery, version negotiation, cancellation or transport parsing in Moriarty.

The inspected SDK knows protocol versions through 2026-07-28. That version uses `server/discover` and per-request `_meta`; it does **not** use an initialize handshake. The same SDK supports the 2025-11-25 initialize/initialized lifecycle. Both are illustrated; version configuration must be independently reviewed. Do not advertise the 2026 version while hand-writing a 2025 initialization exchange. SDK `ListToolsResult` requires the version-appropriate result/cache fields; use its constructors and review the exact client-facing values. The proposed examples use non-cacheable discovery/list metadata; these are examples pending review, not an adoption of broader cache semantics.

## Alternatives and unresolved adoption decisions

| Approach | What it reuses | Tradeoff / disposition |
| --- | --- | --- |
| Official `rmcp`, direct `ServerHandler`, shared existing check consumer | Standard lifecycle/stdio/models plus existing analysis | Recommended candidate; introduces SDK/runtime/schema dependencies needing independent review |
| Official `rmcp` with `#[tool]`/router macros | Same protocol plus generated tool dispatch/schema | Feasible alternative; more macro API/source and dependency review for a single tool |
| External wrapper that shells out to `mori check --json` | Existing binary behavior | Avoids analysis extraction but adds process/tempfile/output capture interfaces; not the proposed first implementation |

A custom protocol/framing implementation is excluded by the task. No dependency, feature, runtime thread count, transport cap, parallelism limit or resource allowance is adopted here. `DECISIONS.json` records proposed choices and the specific review that would resolve each. Prefer a proposed optional `mcp` feature independent of `native`, with a native-free public developer build, over making a checker endpoint require Midnight dependencies. This remains a reviewable feature choice; no manifest or lockfile was edited.

Source fact requiring review: the inspected SDK async stdio receive path calls `read_until` into its line buffer; its separate codec constructor defaults `max_length` to `usize::MAX`. A source-string bound after decoding does not establish an ingress-frame bound. This report makes no security verdict about that fact. The designated resource/dependency and security owners must disposition any framing/resource requirement through an actual supported SDK configuration or upstream change; do not silently add a bespoke framing layer or claim a complete bound from source-size checks. Likewise, request concurrency, cancellation during a synchronous checker call and process termination are unmeasured. A current-thread runtime and one simultaneous check are candidate resource choices only, without an asserted throughput, latency or memory guarantee.

## Independent implementation acceptance, all unrun

1. Freeze the exact candidate, dependency resolution and selected SDK features after the required independent decisions. A changed source basis needs refreshed source qualification; a matching old stub does not qualify new code.
2. Through the real public `mori mcp` process and SDK client, discover/list exactly one tool and run contrasting valid and invalid beta sources. Compare all existing CLI diagnostic fields and source digests to independently frozen expectations. Retain the beta `specified_only` distinction where full CLI reports expose it; source acceptance cannot imply execution support.
3. Exercise Core/1 and Core/2 successful and failed checks, unknown profiles and lexer failures after a `language` prefix. Check dispatch parity and both staged error/success labels. Preserve unknown/no-location errors exactly.
4. Check every diagnostic from a multi-error beta source, all labels/primary flags, comments, Unicode byte offsets and exact source hash. Include valid empty-diagnostic reports; a tool that only emits errors is incomplete.
5. Through the standard SDK lifecycle, exercise current per-request metadata and a supported legacy initialize flow, unknown tool, malformed arguments, rejected source followed by a later successful request, EOF and clean stop. No unrelated stdout may corrupt protocol output. JSON values alone are not interoperability evidence.
6. Preserve CLI human output, full `--json` objects and exit 0/1 behavior for the same fixtures. Verify native-free MCP build/runtime and absence of native consumer calls within the actual selected implementation; this design has not established them.
7. Obtain current ordinary result review for exact implementation/results and owner decisions for dependency/resource/security matters. Public tool use must never require reviewer identity, project campaign state, RP03, registry membership or a hosted Moriarty service.

No check, server, build, package resolver, SDK runtime, provider, native operation, wallet or ledger was invoked in this task. Source pinning, package-source retrieval and artifact shape/hash checks are the only observations. These documents cannot close U0/U2/U7, SP02/MC08 or any broader product exit.
