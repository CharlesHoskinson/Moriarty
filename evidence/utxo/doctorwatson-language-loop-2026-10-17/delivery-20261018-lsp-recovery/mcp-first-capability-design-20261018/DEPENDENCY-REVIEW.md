# Dependency and protocol review proposal

No manifest, lockfile, feature default or resource policy has been adopted or changed. This is a choice packet grounded in retained public package source.

## Candidate A: direct official SDK handler, recommended for review

Proposed root workspace declaration: `rmcp = { version = "=3.5.1", default-features = false, features = ["server", "transport-io"] }`. The owning crate would reference it through `workspace = true` and an independently reviewed optional `mcp` feature. Proposed Tokio root declaration reuses exact pinned lock version `=1.53.2`; necessary features must cover `rt`, `sync`, `time`, and stdio, with `rmcp` enabling its own declared `macros`/I/O requirements on Tokio. Avoid a second version declaration in the child manifest. These lines are proposed configuration examples, not an unapplied patch or resolved feature graph.

The SDK's `server` feature itself enables `transport-async-rw`, `schemars`, `pastey` and `uuid`. `transport-io` adds `tokio/io-std` and async read/write support. Turning off the SDK's optional `macros` feature does not remove every schema or procedural-macro dependency. Its nonoptional dependencies include Tokio/futures/serde/serde_json/thiserror/indexmap/tracing/tokio-util/pin-project-lite and target-specific chrono. Existing Moriarty declarations already cover some names; exact versions and resolved transitive feature unification remain unestablished until the separate reviewed resolver/check step. No HTTP, OAuth, client-spawn or streamable HTTP feature is requested by this candidate.

`rmcp` metadata declares Rust 1.88 while Moriarty declares Rust 1.99 and edition2024. This is a manifest compatibility observation, not a successful build. The pinned Moriarty `Cargo.lock` has Tokio1.53.2 but no `rmcp` or `schemars` package. A new lockfile resolution is necessary; no old-lock success is claimed. All source/license/feature and resource review requirements stay open.

The direct SDK API supports one-tool `ServerHandler` integration without the optional `#[tool]` and `#[tool_handler]` macros. `Tool::new` takes a JSON object schema, `with_raw_output_schema` sets the complete response schema, and the structured-result constructors duplicate payload JSON into text content. `ServerConfig` is an alias of `InitializeResult`; use its builder methods, not struct literals that assume exhaustive public types. `CallToolResponse` may represent multiple result variants in this SDK; a `CallToolResult` must use its public SDK conversion, not a newly declared response enum.

## Candidate B: SDK macro/router route

Enable the official `macros` feature and use its tool/router helpers. It can reduce manual handler code, but introduces `rmcp-macros` and generated-schema contracts to inspect. This packet did not fetch or inspect the separate macro crate and therefore cannot approve this alternative. Candidate A avoids depending on that uninspected API for the first tool.

## Feature boundary choice

Propose an optional, native-independent `mcp` feature with an explicit availability result when absent, replacing the current panic stub. A public developer could build a native-free binary with MCP enabled after adoption. Alternative: include SDK/server dependencies in every normal CLI build and keep `mori mcp` always available. The first changes feature exposure; the second expands all builds' dependency closure. Independent design/owner review must select one. No new project admission, campaign or reviewer record enters either public execution path.

## Runtime/resource choice

The existing CLI is synchronous. A Tokio current-thread runtime through `runtime::Builder::new_current_thread`, `build`, and `Runtime::block_on` is a concrete candidate for hosting the SDK service without rewriting `main`. These APIs exist in the exact Tokio1.53.2 archive whose SHA matches pinned Moriarty Cargo.lock. Runtime choice and one simultaneous analysis remain proposals. No throughput, latency, worker-count or memory result exists.

The public source reader's 1MiB bound and lexer65,536-byte/8,192-token bounds are existing contracts. They do not settle transport ingress-frame size, pending requests or checker cancellation. The SDK source has a line-buffer read path and a codec default max length of `usize::MAX`; an owner must review the actual supported ingress/configuration path before claiming such a bound. This is a source fact, not a security finding. No custom protocol/framing layer is authorized or proposed to fill the gap. Record a precise remaining predicate if the required bound is unavailable from an adopted standard SDK configuration.

## Protocol compatibility choice

The inspected SDK source explicitly distinguishes 2026-07-28 discovery/per-request metadata from versions through2025-11-25 that use initialize. It defaults to all known versions. Candidate A can use its default lifecycle/configuration; alternatively, the documented `supported_protocol_versions` and `ProtocolVersion::known_up_to` permit a reviewed smaller version set. The latter cannot be silently selected to match an obsolete tutorial. The packet illustrates both lifecycle families without adopting a supported-client matrix. Client interoperability and exact SDK output stripping/defaults remain unrun.

## Evidence

- Official metadata: https://crates.io/api/v1/crates/rmcp, retained with retrieval status/time and body hash.
- Exact package: https://static.crates.io/crates/rmcp/rmcp-3.5.1.crate; SHA matches metadata. Retained source `.cargo_vcs_info.json` identifies commit79437f291b2c44053d00dcd5db969fd0cca7c887, path `crates/rmcp` in the official repository https://github.com/modelcontextprotocol/rust-sdk/.
- Consulted SDK files: `Cargo.toml`, `Cargo.toml.orig`, `README.md`, `.cargo_vcs_info.json`, `src/handler/server.rs`, `src/model.rs`, `src/model/tool.rs`, `src/model/meta.rs`, `src/model/capabilities.rs`, `src/model/mrtr.rs`, `src/service.rs`, `src/transport/io.rs`, `src/transport/async_rw.rs`, and bounded test examples.
- Tokio1.53.2 came from an existing cached public archive and was verified against exact74a909 Cargo.lock, then selected files were extracted. It was not freshly fetched or executed. The unpacked cache lacked `.cargo-checksum.json` and was not used as verified source.
- `PUBLIC-SOURCE-RECEIPTS.json`, `TOKIO-SOURCE-PINS.json` and `REPO-SOURCE-PINS.json` separate package-source provenance from design and runtime acceptance.

Public package metadata/source acquisition used Scrapling only; no package manager, SDK runtime, global install, build, provider or Git write was invoked. Existing generic raw/API site patterns already covered this acquisition; no new cookie or site-pattern state was saved.

## Additional package-build source fact for owner disposition

The normalized package manifest declares `build = "build.rs"` and license `Apache-2.0`. The retained `build.rs` reads its manifest directory, computes an ancestor, and conditionally invokes `git config core.hooksPath .githooks` when that ancestor has both `.githooks` and `.git`. It was read, not executed. This packet does not assert the condition holds for any Moriarty build or supply a security judgment. The dependency/build and designated security owners must disposition this exact source behavior before adopting/building the dependency. The crate README points to a repository-level license; the crate archive did not supply a standalone LICENSE file. Metadata is not an independent license audit.
