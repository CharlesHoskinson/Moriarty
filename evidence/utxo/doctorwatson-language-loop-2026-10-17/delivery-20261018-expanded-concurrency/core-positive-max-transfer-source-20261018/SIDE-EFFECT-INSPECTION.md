# Source-only consumer and side-effect inspection

The full pinned paths and hashes are in SOURCE-MANIFEST.json; originals are copied under basis/.

- `crates/mori/src/commands/check.rs:66`: beta route calls mori_parser and mori_checker, then local CheckReport serialization. The distinct `language` branch handles Core. No backend/ledger call occurs in the beta check branch.
- `crates/mori/src/commands/expand.rs:22`: explicit transfer flag routes to the feature-gated Source6 subcommand; default expansion calls Core simulation and is a different route.
- `crates/mori/src/commands/expand/source6.rs:155`: reads source and local scenario, selects and checks the transfer, expands it, writes its actual text to stdout, and emits only a shell status line (quiet suppresses it). Source/cell inputs are local claims. No SDK preprocessing, proving, ledger call, wallet or network operation occurs in this path.
- `crates/mori-source6/src/transfer.rs:83`: selection uses actual Rust parser/checker; extract requires LocalS0 transfer. `max`, Qty arithmetic and atoms have already been constant-evaluated by the checker before the emitter reads their nominal values. Repay remains explicitly unsupported here.
- `crates/mori-source6/Cargo.toml`: dependencies are AST/checker/diagnostics/lexer/parser/span/serde_json; no native backend dependency. `crates/mori/Cargo.toml` makes `source6-expansion` independent of default `native`. A future scoped build should use `--no-default-features --features source6-expansion`; this author did not build it.
- `experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts:355`: parses and lowers actual Source6 input. `mil4-s0-source-v6.ts:28` calls the existing provisional local prepare API. Both are imported directly by the candidate. Their local TS results do not supply native proofs or backend Artifacts.
- The new test itself uses only the explicit absolute Rust binary and a fresh absolute observation directory. It spawns local `check` and `expand` with no shell, timeout30s and maxBuffer1MiB; it writes capture files and fixture copies into that fresh directory. It does not invoke the Node beta emitter, compile/compile-call, a provider, SDK/native tools or a network service.

This is a control-flow/dependency observation, not a security judgment. Actual build, runtime outcome and resource receipt are unobserved by this source task.
