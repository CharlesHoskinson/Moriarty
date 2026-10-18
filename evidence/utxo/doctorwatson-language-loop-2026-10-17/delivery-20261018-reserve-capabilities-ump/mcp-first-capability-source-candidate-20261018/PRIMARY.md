# Unapplied first MCP source candidate

Status: complete physical ordinary source/test proposal; UNAPPLIED, UNADOPTED, UNBUILT and UNRUN. The base is exact main `e4d7a2b5709caa18a94f5256d03a0302364b8235`, tree `b44043fba64fa2bd396d18412adb3472434c70ac`. No repository/worktree source, Git state, installed package, native resource or shared runtime was changed.

## Candidate behavior and real consumer

The existing `mori mcp` public command would stop panicking. With the proposed optional `mcp` feature enabled, it constructs the official SDK stdio service and exposes one tool with the previously proposed wire name `mori_check_source`. Without that feature, it returns the explicit `mori::command::mcp-unavailable` command error. `mcp` is independent of `native`; the proposed native-free selection is `--no-default-features --features mcp`.

The physical callable path is `main` → `commands::run(Command::Mcp)` → `commands/mcp.rs::run` → `MoriServer.serve(rmcp::transport::stdio())` → `ServerHandler::call_tool` → `diagnostic_payload` → **the same** `check_source::analyze_source` consumed by `check.rs::run`. This shared function invokes the current beta parser/checker, Core/1 parse-and-validate or Core/2 stage loader using the exact original dispatch. There is no subprocess, temporary source file, custom JSON-RPC dispatcher/framing or independent checker implementation.

The result is a diagnostic report, not compilation/proof/settlement. It preserves all beta codes/severity/message/help/labels, original UTF-8 byte locations, Core error optional offset/step and the exact three staged qualification labels. `checked_profile` identifies the consumer selected by current CLI routing, including a rejected unsupported header; it does not approve the source's declared profile. Display labels are not read as paths. The source hash identifies the supplied source bytes only.

## Full physical files

`before/` retains exact current Git blobs and unchanged consumer/type context. `after/` contains complete Rust bodies, complete root/crate Cargo manifests and complete normal fixtures. It is a source overlay, not a runnable checkout. `SOURCE-CHANGES.json` binds every after path to exact before/after hashes or an explicit new-file disposition. `candidate.patch` is a genuine unapplied unified patch generated from those same bytes. No partial function extract is substituted for a changed physical source file.

| Changed concern | Candidate paths |
| --- | --- |
| Shared existing dispatch and full CLI reports | `crates/mori/src/commands/check_source.rs`, its `tests.rs`, and `commands/check.rs` |
| SDK stdio public entrypoint and feature-off result | `crates/mori/src/commands/mcp.rs` |
| Standard SDK handler and normal session tests | `commands/mcp/server.rs` and `server/tests.rs` |
| Shared module registration | full `commands/mod.rs` |
| New optional feature/dependency proposals | full root and `crates/mori/Cargo.toml` |
| Independent expected public data | `crates/fixtures/mcp/` schemas, eight check cases and complete JSON-RPC request/response examples |

Beta CLI JSON still serializes its original `CheckReport` struct directly to retain field order; converting it through a generic JSON map would change that public text. Core JSON retains the existing shape, including absent source hash on rejection. Human diagnostics and parsed/finished status content remain on the CLI path. The shared analysis now constructs an owned beta report for human mode too, and emits the Parsed status after analysis completes; memory/timing effects are unmeasured, even though field content and ordering are intended to stay equivalent.

## Source-first normal expectations and tests

`TEST-FIRST-SOURCE-RECORD.json` records test/fixture preparation before implementation. RED and GREEN are **UNRUN** under the explicit source-only boundary; this is not a claim of an observed TDD cycle.

Eight frozen cases cover accepted/rejected/unparsable beta source; accepted/rejected Core/1 and Core/2; and a normal UTF-8 comment prefix over the retained multi-diagnostic beta source. Beta oracles come from exact current source snapshots. Core full CLI expectations are manually derived from the pinned source's action/validation rules. The UTF-8 relation independently shifts every frozen span by the prefix's byte count; no evaluator/checker generated it.

The shared-dispatch test compares full CLI objects and original beta pretty field order. The SDK test uses the official SDK over an in-memory duplex transport, lists the exact full tool definition, compares complete diagnostic payloads/text for all eight cases, then makes another valid request with the default label after the reported source errors. A separate test covers public command parsing; feature-off test source checks the explicit unavailable result. No security/hostile tests were authored.

These are genuine executable test sources **after** review/integration/resolution. They were not compiled or executed here. In-memory service tests do not establish actual public stdio process behavior, actual exit/I/O compatibility, either lifecycle's interoperability, native-free build success, or timing/resource bounds. All remain later evidence predicates.

## Dependency/resource decisions are still missing

Two substantive agreeing ordinary design votes and the designated dependency/build-script/resource judgments are missing. This candidate supplies concrete reviewable bytes, not their adoption. `ADOPTION-BOUNDARY.json` records those remaining gates.

The proposed SDK is exact `rmcp=3.5.1`, defaults off, `server` and `transport-io`; proposed Tokio is exact `1.53.2` from the existing base lock. The direct handler uses actual inspected SDK definitions; there is no generated macro API invented for the candidate. The SDK client's `client` feature is a dev-dependency proposal for the normal in-memory session test. Consequently, test dependency resolution can include the SDK even when the application `mcp` feature is disabled; normal binary exposure remains optional. This consequence needs the same independent dependency/build review.

`Cargo.lock` is retained as exact base context only and is not fabricated or updated. New dependency resolution, transitive feature/version compatibility, build-script behavior and compiler compatibility are unestablished. A locked build cannot use this proposal until an approved lock candidate exists. Do not run resolution/build merely because the source packet is complete.

Source facts retained for designated owners: the SDK package declares a build script that conditionally invokes `git config core.hooksPath`; its standard async stdio receive path and default codec do not establish a proposed ingress-frame/concurrency/cancellation bound. The existing 1MiB source-input bound after decoding is not a transport-frame bound. These facts carry no new security judgment in this ordinary candidate. The current-thread runtime, SDK default supported versions/cache hints, source cap and dependency feature choices are candidate bytes only, not approved resource policy. No custom framing is added to resolve their open review.

## Evidence and limitations

`BASE-SOURCE-PINS.json` pins the exact current source. `dependency-source/` retains the previously acquired, checksum-bound official SDK archive/full definitions and selected exact-lock Tokio source, with original provenance receipts. Public source acquisition in the prior design used Scrapling; this candidate made no new network/provider call and ran no package code or build script.

`SOURCE-SYNTAX-CHECK.json` records only parsing text with an existing tree-sitter Rust grammar; it is not a Rust type check or build. `VALIDATION.json` records source/hash/manifest/schema/fixture checks only. Cargo resolution, rustfmt, clippy, builds, ordinary tests, public CLI/MCP/SDK execution and all native/proof/wallet/ledger operations are UNRUN. No source review or packet integrity result closes a product exit or supplies missing owner approval.
