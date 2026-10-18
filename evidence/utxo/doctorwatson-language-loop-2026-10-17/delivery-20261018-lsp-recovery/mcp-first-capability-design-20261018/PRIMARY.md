# L19: first normal Rust MCP capability proposal

Read `DESIGN.md`, `INTERFACES.md`, `DEPENDENCY-REVIEW.md` and `DECISIONS.json` for the proposed result and remaining owner decisions. This packet is source-only and unadopted; it contains no implementation patch.

The exact public consumer would be `mori mcp` → official SDK stdio server → one `mori_check_source` tool → the same shared analysis used by `mori check` → actual beta parser/checker, Core/1 parser/validator or Core/2 stage loader. No standalone helper, child command or bespoke framing is proposed.

`INPUT-SCHEMA.json`, `OUTPUT-SCHEMA.json` and `examples/` specify complete request/result/type interfaces. `EXAMPLES.json` distinguishes existing pinned CLI snapshot values from source-derived expectations. No example is a performed MCP/check result.

`REPO-SOURCE-PINS.json` pins exact main74a909/treef18c23 bodies; mutable checkouts were not substituted. `PUBLIC-SOURCE-RECEIPTS.json` pins the official rmcp3.5.1 metadata/archive/source acquired with Scrapling. `TOKIO-SOURCE-PINS.json` pins existing-lock Tokio1.53.2 source from a verified cached archive. `READ-LIMITS.json` records read/authority/currentness limits. `VALIDATION.json` records only packet integrity checks.

Dependency/feature/runtime/protocol/resource choices remain pending independent design and applicable owner review. Security judgments remain with their designated owner. Maintainer review never becomes a public developer/tool-call permission. No product, language or ledger acceptance is claimed.
