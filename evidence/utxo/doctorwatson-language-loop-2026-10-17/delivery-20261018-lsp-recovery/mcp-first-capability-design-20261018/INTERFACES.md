# Proposed application interface and SDK integration seams

These are design types and method contracts, not implemented Rust. Protocol envelopes/framing belong to `rmcp`; the payload schemas are the authoritative complete field definitions in this packet. All examples are unrun.

## Tool arguments

```typescript
type CheckSourceArgs = {
  source: string;       // required, exact source text
  file?: string;       // display label, default "mcp-input.mori"
};
```

No other keys are accepted. Missing source, a non-string field, explicit null, or an unknown key is an invalid-arguments request. An empty source is a valid tool invocation and is checked by the real parser; it is not a transport error. `file` is not a path permission or a file lookup. There is no profile override, credential, reviewer/campaign record, target or backend option.

## Complete successful tool payload types

```typescript
type Span = { start: number; end: number }; // u32 UTF-8 byte offsets

type BetaDiagnostic = {
  kind: "beta";
  code: string | null;
  severity: "error" | "warning" | "advice";
  message: string;
  help: string | null;
  labels: Array<{
    span: Span;
    text: string | null;
    primary: boolean;
  }>;
};

type CoreDiagnostic = {
  kind: "core";
  code: string;
  message: string;
  offset: number | null; // existing Option<u32>; no invented end offset
  step: number | null;   // existing Option<u16>, often null for source errors
};

type StageQualification = {
  qualification: "local-reference-stage";
  authentication: "none";
  consumption: "untracked";
};

type CheckResult = {
  format: "mori-mcp-check/1";
  kind: "check";
  scope: "source-check-only";
  file: string;
  source_hash: string; // exactly 64 lowercase hex SHA-256 digits
  checked_profile: "moriarty-beta/1" | "moriarty/1" | "moriarty/2";
  status: "accepted" | "rejected";
  diagnostics: Array<BetaDiagnostic | CoreDiagnostic>;
  stage_qualification: StageQualification | null;
};

type SourceTooLarge = {
  format: "mori-mcp-check/1";
  kind: "tool_error";
  code: "source_too_large";
  message: string;
  limit_bytes: 1048576;
};

type ToolPayload = CheckResult | SourceTooLarge;
```

`checked_profile` is the profile of the actual invoked consumer, even when the input declared an unsupported header. It is not a successful recognition claim about that input. A Core/2 route always has its three qualification fields and only Core diagnostics; beta has beta diagnostics; Core/1 has Core diagnostics. A successful source check has no diagnostics; rejected reports preserve every original diagnostic. Source hash identifies the checked source text only, never an ABI, deployment or key.

### Existing-to-proposed projection

| Existing consumer value | Proposed result field | Required preservation |
| --- | --- | --- |
| `CheckReport::status` / Core `Result::is_ok` | status | identical decision; no acceptance inferred from protocol success |
| `CheckReport::source_hash` / `intent::hash_bytes(source.as_bytes())` | source_hash | exact original bytes including final newline |
| `CheckReport::diagnostics` | beta diagnostics plus `kind=beta` | codes, severity, message, nullable help, all labels in original order |
| `mori_core::Error` | one Core diagnostic plus `kind=core` | code, message, optional offset and step unchanged |
| `stage::STAGE_LABELS` | stage_qualification | all three exact values on both stage acceptance and rejection |
| current route branch | checked_profile | same leading-token and supported `/2` selection as CLI |

The beta JSON snapshot with duplicate `price` and unknown `pirce` is the complete multi-diagnostic example. Its two diagnostics and three labels retain their exact byte spans. The Core/1 and Core/2 empty-agreement rejection examples derive from the source's nonempty-action rule, not a performed check. Their location fields are null because validator errors do not add a parser token position. The `accepted` beta fixture retains no diagnostic and does not claim its `specified_only` action executes.

## Internal shared analysis contract, proposed

```text
check::analyze_source(file_label: &str, source: &str) -> CheckAnalysis
CheckAnalysis::is_accepted(&self) -> bool
CheckAnalysis::public_cli_report(&self) -> existing full CLI JSON representation
CheckAnalysis::mcp_diagnostics(&self, exact_source: &str) -> CheckResult
CheckAnalysis::render_human(...) -> existing human diagnostic behavior
```

Names are design seams for the implementation author, not newly exported library APIs or code to compile. `CheckAnalysis` must retain owned beta report plus original human diagnostics or the owned Core result. It may not retain `Checked` beyond its borrowed AST. The consuming CLI still owns file-read errors and existing timing/status behavior. Both CLI and MCP must call this one analysis path; consuming a different parser for MCP invalidates the design predicate.

## Inspected SDK interfaces, no replacement protocol

| Responsibility | `rmcp` 3.5.1 source API |
| --- | --- |
| startup/configuration | `ServerHandler::get_info() -> ServerConfig`; `ServerConfig::new(ServerCapabilities::builder().enable_tools().build())` |
| implementation identity | explicit `Implementation::new("mori", <mori package version>)`; avoid accidentally using the SDK crate's build identity |
| tool listing | `ServerHandler::list_tools(Option<PaginatedRequestParams>, RequestContext<RoleServer>) -> Future<Result<ListToolsResult, ErrorData>>`; `ListToolsResult::with_all_items(...)` plus reviewed cache metadata |
| argument routing | `ServerHandler::call_tool(CallToolRequestParams, RequestContext<RoleServer>) -> Future<Result<CallToolResponse, ErrorData>>`; SDK non-exhaustive structs must use supported constructors/builders |
| tool definition/schema | `Tool::new`, `with_raw_output_schema`, `with_annotations`; return the same definition from `get_tool` for SDK validation |
| argument validation | serde closed input type plus SDK model/schema seam; same definition for listing and `get_tool` |
| ordinary result | `CallToolResult::structured(payload)`; return via `CallToolResponse`'s SDK-supported conversion |
| source-size tool error | `CallToolResult::structured_error(payload)` |
| unknown tool / malformed arguments | SDK `ErrorData::method_not_found` / `invalid_params`, preserving its request ID and error envelope |
| transport and lifecycle | `ServiceExt::serve(rmcp::transport::stdio())`, then `RunningService::waiting`; SDK default initialize/discover/metadata negotiation |

Proposed `mcp::run` can host the SDK future inside a reviewed Tokio runtime without changing the entire CLI to an asynchronous `main`. No transport loop, lines codec, handwritten initialize state machine, protocol version parser or host JSON-RPC client is proposed. stdout must remain exclusively the SDK transport; the existing `Shell` is not a report channel inside a tool handler. Startup errors before service entry and process exit handling retain the CLI error boundary, with no false check report.

## Example index

- `discover.*`: 2026-07-28 discovery with required per-request metadata; compatible proposed non-cacheable complete response.
- `legacy-initialize.*` and `legacy-initialized.notification.json`: 2025-11-25 handshake alternative. After it, legacy tool calls omit per-request current-version metadata, and SDK responses omit newer result-type/cache fields as applicable. No2026 initialize response is fabricated.
- `tools-list.*`: exactly one complete tool definition and both payload schemas.
- `accepted.*`, `rejected.*`, `unparsable.*`: exact-source pinned beta snapshots projected to the proposed tool shape.
- `core1-accepted.*`, `core2-accepted.*`: proposed positive expectations from the inspected requires/Bool path; not observed acceptance.
- `core1-rejected.*`, `core2-rejected.*`: source-derived normal rejections, preserving error type and Core/2 qualification.
- `invalid-arguments.*`: proposed ordinary handler argument error, not a checker rejection.
- `source-too-large.response.json`: distinct caller-visible tool error; generating a megabyte request is unnecessary in this design packet.

Every response includes the full proposed JSON value. The SDK's structured-result helper intentionally provides both text JSON and structured content; tests should parse the text and compare it to the structured value. These examples specify expected values and cannot establish stdio interoperability or successful public invocation.
