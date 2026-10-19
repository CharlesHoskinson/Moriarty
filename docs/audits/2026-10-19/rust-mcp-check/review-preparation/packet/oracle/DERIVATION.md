# Independent normal MCP check expectations

Subject baseline: Moriarty main `a2d0f968720eda21f5b4d87b184716bead028b45`, tree `e1d993e9edb53382a437284156eb379799f1745e`. This packet was authored before the new Rust MCP protocol implementation. No file in the candidate author's `rust-devtool-next-implementation-20261019` directory, its expectations, or consumer outputs was read. No Moriarty parser, checker, evaluator, compiler, SDK, Node program, provider or test was executed to obtain these expectations.

The three `.mori` programs are new independent inputs. Their expected typing follows the existing Core/1 language contract. Serialization details follow the existing baseline IR and shared check report API, not the new MCP implementation. This is independence from the new protocol consumer; it is not an independent proof of the baseline Core checker or its representation.

## Normative protocol derivation

Whole official 2025-06-18 Markdown pages and retrieval receipts are in `sources/` and `*-RETRIEVAL.json`. They were retrieved directly from modelcontextprotocol.io through Scrapling 0.4.15, with no authentication or cookies supplied. The current website serves versioned documents; each exact retrieved body is pinned, rather than assuming an immutable upstream website revision.

- `basic.raw`, Messages: request IDs are non-null strings or integers, unique per session; responses preserve IDs and contain either result or error. Notifications carry no ID and receive no response.
- `lifecycle.raw`, Initialization/Version Negotiation: initialize is first; after its result the client sends notifications/initialized. A supported requested version must be echoed; otherwise the server supplies a version it supports. A client that cannot support that returned version should disconnect. Shutdown uses transport closure, not a shutdown RPC.
- `transports.raw`, stdio: UTF-8, one JSON-RPC message per physical line, embedded source newlines escaped inside strings; stdout contains only MCP messages. Stderr logging is permitted and is not assigned invented exact bytes here.
- `ping.raw`, Behavior Requirements: ping receives an empty result object.
- `tools.raw`, Tools/Structured Content/Errors: tools/list advertises tools, tools/call invokes them; structuredContent is an object and its serialized text copy supports older clients. A rejected source supplied to an otherwise valid check request is a tool result with isError=true, not a JSON-RPC protocol failure. The latter mapping is also the agreed Moriarty API contract.

Exact server identity, version choice, tool description, schemas, annotations, wrapper names and display-label behavior are project API decisions supplied in public coordination messages BEFORE implementation. They are not requirements imposed by MCP itself. The chosen 2025-06-18 protocol is deliberately pinned rather than inferred as the latest MCP version. Tool hints are metadata, not a security authorization claim.

## Core/1 typing and report derivation

Whole pinned sources are under `sources/repository/`, with Git blob IDs, hashes and original URLs in `REPOSITORY-SOURCE-MANIFEST.json`.

`crates/mori-core/README.md` defines the explicit Core/1 header, UInt/Bool states and inputs, ordered let/requires/next/ensures/emit statements, arithmetic/comparison/Boolean typing and unsupported control flow. `src/ir.rs` defines the existing serialized Action/InputDecl/Statement/Error shapes. `src/parser.rs` defines the existing flat expression numbering; `src/runtime.rs`'s validate/type_cached establishes the baseline type-error category and message. These baseline representation sources were exposed to this oracle author and are disclosed. The expected accepted/rejected semantics were derived manually; no baseline function was called.

`commands/check_source.rs` is the existing source-profile dispatcher and report API: explicit language header routes Core; accepted Core report fields are status,file,source_hash,language,agreement,actions,diagnostics. Rejected Core report has status,file,diagnostics. The current stub `commands/mcp.rs` is only a todo at the pinned baseline. The new MCP implementation is not a source for this oracle.

`intent.rs::hash_bytes` specifies SHA-256 of the exact UTF-8 source bytes. Python hashlib was used only to calculate that source-identity field and packet custody hashes. No contract evaluation, expected financial result or SDK cryptography was computed.

### UInt accumulator

The state total and input delta have type UInt. `total + delta` is UInt and binds updated. `updated >= total` and `total >= delta` are Bool, satisfying requires/ensures. Assigning updated to total preserves UInt. Emitting total is a supported UInt event value. Checking does not run addition or establish that a requires clause holds for any supplied input.

Manually enumerated expression nodes in source order: 0=total, 1=delta, 2=Add(0,1); 3=updated, 4=total, 5=Ge(3,4); 6=updated; 7=total, 8=delta, 9=Ge(7,8); 10=total. The full expected action report uses Let2, Requires5, Next6, Ensures9, Emit10. There is no invented runtime state/result in this packet.

### Bool gate

State ready and input flag have type Bool. `not ready or flag` is Bool, assigning flag to ready preserves Bool, and `ready == flag` is Bool. The event value is Bool. Expressions: 0=ready, 1=Not(0), 2=flag, 3=Or(1,2); 4=flag; 5=ready, 6=flag, 7=Eq(5,6); 8=ready. Thus Requires3, Next4, Ensures7, Emit8. This contrasts Boolean unary/logic/equality with arithmetic/let/UInt ordering.

### Ordinary type error

OracleWrongState declares count:UInt but `next count = true` supplies Bool. This is ordinary source invalidity, not a hostile/protocol/security specimen. Existing validate's Next branch returns code `mori::core::type`, message `State update has wrong type.`, and Error::new supplies null offset/step. Expected status is rejected, one diagnostic, no accepted actions/source_hash/language report keys. The MCP sourceProfile still identifies the recognized header; coreProfile is null by the proposed consumer API's refusal contract.

## Transcript use

`inputs/normal-session.ndjson` contains initialize, initialized notification, ping, tools/list, and three check calls. A real client should send initialize, wait for its response, send initialized, then send each request and wait for its response; the file records messages and does not authorize a new runtime. There are seven input lines, six response lines, and no reply to initialized. IDs intentionally mix strings and integers. The Bool request omits displayName and expects the default `<mcp>`; the other calls preserve explicit display labels.

`inputs/version-fallback-session.ndjson` is a SEPARATE connection whose client supports only 2024-11-05. The server offers its sole supported 2025-06-18; this client then closes stdin/connection without sending initialized or tools requests. This is normal negotiation, not an expectation that the server implements 2024-11-05. No server error is expected for this version proposal.

Expected NDJSON files are complete illustrative serializations of JSON values, NOT a demand for JSON object key ordering. Compare parsed objects structurally; preserve array order, ID type, strings, numbers and nulls. For each check result, parse content[0].text as compact JSON and require equality to structuredContent; object key order inside that string is also irrelevant. Exactly one text content block is required by the agreed API. No unexpected protocol messages/replies or non-JSON stdout may be silently discarded. No exact process exit status or stderr diagnostic has been invented from the MCP standard; those terminal expectations remain for root's execution contract.

## Qualifications and undecided scope

The agreed API metadata is frozen in `PUBLIC-API-CONTRACT.json`. JSON Schema maxLength measures characters, while the agreed implementation limits are UTF-8 bytes (65536/256); these ASCII examples are below both limits and establish no Unicode-boundary conformance. Frame/response bounds and server-defined -32002 lifecycle refusals are recorded as project decisions but are not exercised by these normal transcripts. Ping-before-initialize behavior was corrected by the author before freeze; this oracle uses ping after initialization and does not assert the old restriction.

This selected oracle covers Core/1 only. It does not establish Beta/Core2 dispatch correctness, malformed protocol handling, concurrency, SDK/native/evaluator/proof/ledger behavior, general MCP conformance or the full language EARS. The baseline shared dispatcher contains Beta and Core2 paths; preserving those remains an implementation/review obligation. No new requirement IDs, milestone counts or acceptance states are created.

The graph was consulted first for discovery, as recorded in GRAPH-DISCOVERY.json. Its dated ba21 source snapshot is not relabeled current; all baseline bodies above came from the exact requested a2d0 commit. Startup W moved independently to 107e0079 while preparing this packet; that does not rebase the oracle. An untracked owned/node_modules pathname was observed in git status; no contents were read.
