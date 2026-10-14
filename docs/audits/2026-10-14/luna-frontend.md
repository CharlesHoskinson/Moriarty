# Luna frontend, semantics and authoring map

Base commit: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa`

This is a read-only source map of the CLI, parser/checker, Core execution path, TypeScript authoring package and their boundaries. It records inspected code, not exercised behavior. No product tests or native operations were run.

## Main finding

The repository has distinct authoring and execution paths. Rust `mori check` routes a file beginning with `language` to Core/5 and otherwise parses/checks Moriarty Beta. Rust compile, expand and simulate load Core/5 directly. The Node package has a separate TypeScript Beta frontend and CLI; its supported transfer/repay bridge generates Source/6 for unqualified local preparation. The beta family examples are checked but specified-only. Rust `mori lsp` and `mori mcp` are todo stubs; the Node package has separate stdio servers. The roadmap explicitly leaves broad source-to-ledger correspondence, mandatory proof-carrying acceptance and full conformance open.

## Source map

- **Consolidated delivery roadmap** (`roadmap`): `ROADMAP.md`
- **Product language and profile guide** (`product_readme`): `README.md`
- **Rust `mori` CLI dispatcher** (`rust_cli`): `crates/mori/src/cli.rs`
- **Rust `mori check` router** (`rust_check`): `crates/mori/src/commands/check.rs`
- **Rust Moriarty Beta parser** (`rust_parser`): `crates/mori-parser/src/lib.rs`
- **Struct-of-arrays Beta AST** (`rust_ast`): `crates/mori-ast/src/lib.rs`
- **Rust Beta declaration/type checker** (`rust_checker`): `crates/mori-checker/src/lib.rs`
- **Rust constant/value evaluator** (`rust_const_eval`): `crates/mori-checker/src/eval.rs`
- **Rust action support classifier** (`rust_support`): `crates/mori-checker/src/intents.rs`
- **Core/5 parser, validated IR, deterministic evaluator** (`core5`): `crates/mori-core/src/lib.rs`
- **Core/5 source parser** (`core5_parser`): `crates/mori-core/src/parser.rs`
- **Core/5 validation, simulation and replay** (`core5_runtime`): `crates/mori-core/src/runtime.rs`
- **ZKIRv3 backend compiler** (`backend`): `crates/mori-backend/src/lib.rs`
- **Rust `mori compile` consumer** (`rust_compile`): `crates/mori/src/commands/compile.rs`
- **Rust `mori expand` consumer** (`rust_expand`): `crates/mori/src/commands/expand.rs`
- **Rust `mori simulate` consumer** (`rust_simulate`): `crates/mori/src/commands/simulate.rs`
- **Rust LSP command** (`rust_lsp`): `crates/mori/src/commands/lsp.rs`
- **Rust MCP command** (`rust_mcp`): `crates/mori/src/commands/mcp.rs`
- **Published Node beta package/CLI entry** (`beta_pkg`): `packages/moriarty-beta/package.json`
- **TypeScript beta parser/analyzer** (`beta_frontend`): `packages/moriarty-beta/src/frontend.ts`
- **TypeScript public authoring API** (`beta_api`): `packages/moriarty-beta/src/index.ts`
- **Beta-to-Source/6 local preparation bridge** (`beta_bridge`): `packages/moriarty-beta/src/bridge.ts`
- **Successor Source/6 S0 parser/lowerer** (`source6_frontend`): `experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts`
- **Source/6 local S0 unqualified preparation** (`source6_core`): `experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts`
- **Node `mori` command implementation** (`beta_cli`): `packages/moriarty-beta/src/cli.ts`
- **Node LSP/MCP stdio server** (`beta_servers`): `packages/moriarty-beta/src/servers.ts`
- **Editor assets and integration notes** (`beta_editor`): `packages/moriarty-beta/editor/README.md`
- **VS Code extension integration notes** (`beta_vscode`): `packages/moriarty-beta/editor/vscode/README.md`
- **Beta family example qualification** (`beta_examples`): `packages/moriarty-beta/examples/README.md`
- **Core/5 intent and effect checks** (`intent_core`): `crates/mori-core/src/intent.rs`

## Audit questions

1. Which supported source profile is the intended public authoring source for the roadmap successor: Rust moriarty-beta/1, Core moriarty/1, TypeScript beta syntax, or Source/6? The CLI/package split and duplicated frontends create multiple apparent entry points.
2. What is the intended end-to-end production path from current authoring source through typed IR, backend, native verifier and ledger effect readback? In particular, Rust `mori check` can accept Beta grammar while compile/expand/simulate use Core parser.
3. Are operation support registries intentionally duplicated across Rust checker and TypeScript frontend (`call name contains dot` versus `HORIZON` membership)? How is drift and unknown bare operations prevented?
4. Which local preparation predicates are actually enforced by Core/5, generated Source/6 and source-level bridge, and which are only stipulated scenario inputs? Preserve exact provenance of state, identities, effects, duties and replay bindings.
5. Are Rust LSP/MCP intended as supported entry points, or should users be routed to Node beta services? Rust command bodies are stubs while Node package provides real stdio servers.
6. Which roadmap U0/U2 source, Core and target profiles are versioned and embedded together? What proves semantic contract and target pin agreement rather than parser acceptance alone?
7. What author-facing behavior is intentionally excluded for family operations and how will partial/continuing lifecycle obligations be represented before those operations become runnable?
8. What regression separates `PreparedUnqualified`, reference execution, genuine native proof verification, and accepted Preview ledger settlement at each public output/API boundary?

## Coverage and authority

- Scope is the frontend/semantics/authoring subgraph; it does not claim full repository graph coverage.
- Files were inspected, but product commands, tests, editor activation, and runtime behavior were not exercised.
- Native authority was zero: no native SDK tests, decode/query/check/prove/verify operations, or ledger actions.
- The single executed command was the required guarded plugin status query. It reports loan-swap repair blocked by stale binding/candidate inputs and missing accounting/resource live state; this is operational status, not product acceptance.
