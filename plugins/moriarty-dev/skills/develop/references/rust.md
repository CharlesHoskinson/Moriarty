# Rust toolchain changes

Apply this to any change under checkout `crates/`, `Cargo.toml`, `Cargo.lock`
or `rust-toolchain.toml`. Checkout `RUST_GUIDELINES.md` is the authority: read
it before writing Rust, and read it again before review. This list only names
the rules agents have broken most often. The guidelines are outside the
installed package, so resolve the path against the active checkout root.

## Before writing

- A deliberate difference from the guidelines, or from the TypeScript
  reference frontend, needs the maintainer's agreement before it is made.
  Record an agreed difference in the guidelines.
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

- Never run the `mori` binary from a test and never snapshot `--help`. Test the
  library crates; keep the CLI a thin layer over them.
- Nothing under `crates/` reads outside it. Shared programs and fixtures live in
  `crates/fixtures/`.
- Snapshot tests use insta and show the `.mori` source, not the Rust
  expression. Every phase that consumes source has a proptest never-panic
  property.

## Before handing off

Run these from the checkout root. All three must pass; CI runs the same:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Fix a lint rather than silence it; an `#[allow(clippy::...)]` needs a comment
saying why. Report any check that was not run.

Reviewers check the change against `RUST_GUIDELINES.md` as well as for
correctness. A guideline deviation without recorded agreement is a finding.
