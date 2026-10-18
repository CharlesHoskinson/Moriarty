# Rust toolchain changes

Apply this to any change under checkout `crates/`, `Cargo.toml`, `Cargo.lock`
or `rust-toolchain.toml`. Read checkout `RUST_GUIDELINES.md` before writing
Rust and before review. Current checkout `AGENTS.md`, the product contract and
explicit user steering control profile scope, routing and execution. This list only names
the rules agents have broken most often. The guidelines are outside the
installed package, so resolve the path against the active checkout root.

## Before writing

- Record deliberate guideline or profile differences in the guidelines under
  the applicable user authorization. Routine authorized repairs need no
  additional approval gate.
- Put code in the crate that owns the concern and reuse what exists. Do not
  copy a module into a second crate or add a parallel frontend, checker or
  ZKIR builder.
- Build and test every file through Cargo. No direct `rustc` harness and no
  `#[path]` that remounts a file from elsewhere or loads it twice.
- Unadopted prototypes go in `experiments/` or behind a feature, not in `src/`
  gated by `#[cfg(test)]`.

## While writing

- Source diagnostics are one `MoriDiagnostic`, built in the crate's
  `diagnostics.rs`, with a `mori::<phase>::<kind>` code. Use `thiserror` only
  for errors that never point at source. No `anyhow` in library crates.
- Declare every dependency once in the root `[workspace.dependencies]`.
- No absolute paths; locate files from `env!("CARGO_MANIFEST_DIR")`.
- Never derive an ABI, deployment, key or build identity from source text.
- Comments explain the code. Review status, authorship, model names, audit IDs
  and "not yet run" notes belong in commits, pull requests or `docs/audits/`.

## Tests

- Test semantics in the library crates; keep the CLI a thin layer over them.
  The CLI feature-boundary work permits narrowly scoped real binary tests of
  dispatch, availability, complete frozen output, process exits and I/O as
  recorded in `RUST_GUIDELINES.md`. Never snapshot `--help`; use clap schema
  assertions. Existing fixture paths and test-support mounts remain separate
  documented guideline deviations, not permission to add new mounts.
- Nothing under `crates/` reads outside it. Shared programs and fixtures live in
  `crates/fixtures/`.
- Snapshot tests use insta and show the `.mori` source, not the Rust
  expression. Every phase that consumes source has a proptest never-panic
  property.

## Before handing off

Inspect these local checks from the checkout root before selecting one. Run
only within the current scope and execution authority; broad Cargo commands
can invoke blocked SDK/compiler/native operations:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Fix a lint rather than silence it; an `#[allow(clippy::...)]` needs a comment
saying why. Source-only handoffs report every unrun check and acceptance gap.
The optional `rust` workflow is manual only; do not dispatch hosted CI during
ordinary work. No check here grants native authority.

Reviewers check the change against `RUST_GUIDELINES.md` as well as for
correctness. Unrecorded guideline differences are findings; existing user authorization
remains effective.
