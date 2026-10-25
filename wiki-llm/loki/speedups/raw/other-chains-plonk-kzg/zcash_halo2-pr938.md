# Speed up verifier and prover instance commitments
https://github.com/zcash/halo2/pull/938
merged: 2026-10-09T21:35:46Z

Cuts the cost of committing to INSTANCE columns, on both the verifier and the
prover side, upstreamed from https://github.com/zakura-core/libraries.

Closes #968.

> **Stacked on #936**, whose branch is the base. It is a SIBLING of #937, not
> stacked on it: the two share `arithmetic.rs` but were verified to merge
> cleanly, see "Relationship to #937" below.

Five of the seven commits are ports and keep their original author, date and
message, with an upstreaming block appended carrying the `format-patch` command
that reproduces each one and its `Source-Commit:` trailer. Two are written
here: a manifest entry and the CHANGELOG.

## What it does

- `verify_proof` computes instance commitments directly from their supplied
  Lagrange coefficients instead of zero-padding to the evaluation domain, and
  batch-normalizes them when there are at least four, so one inversion serves
  many.
- An invariant is added so the normalization cannot silently regress.
- `create_proof` likewise commits directly to the supplied instance values.
- `best_multiexp` uses serial window accumulation when only one worker is
  available, because with a single worker the parallel algorithm only adds
  doublings.

## What a reviewer should look at

**1. The normalization invariant.** "Enforce instance normalization invariant"
is the commit that turns an optimisation into a contract. Worth checking that
every construction path satisfies it, not just the ones the tests drive.

**2. The multiexp predicate is a behaviour change, not a refactor.**
`should_parallelize_multiexp(num_coeffs, num_threads)` is
`num_threads > 1 && num_coeffs > num_threads`, where the code previously read
`coeffs.len() > num_threads`. The single-worker case now takes the serial
path. It has its own unit test, `test_multiexp_algorithm_selection`.

**3. One manifest change is hand-written, not ported.** The standard exclusions
drop `Cargo.toml`, so the `[[bench]]` entry registering
`benches/plonk_verifier.rs` is reapplied by hand. Without it cargo
auto-discovers the file and builds it against the default libtest harness,
which does not compile, because the benchmark is written for criterion. It is
written in this file's own style, name plus `harness = false` with the path
inferred, where the source commit spelled out an explicit path.

## Relationship to #937

#937 (signed Booth recoding) also edits `arithmetic.rs`, and its patches
expected `should_parallelize_multiexp` in their context. Rather than pull this
commit into that pull request, the predicate was left alone there and the
change kept here where it belongs, with its own changelog entry.

The two were verified to combine correctly rather than assumed to: building
this branch and merging #937's into it gives a clean merge whose tree builds,
passes its tests and is clippy clean, and whose `best_multiexp` prologue is
byte-identical to the source repository's own. They can merge in either order.

## Provenance

All five ports were checked against the `format-patch` output of their source
commits and are VERBATIM: four byte-identical, and `e95fe3b` differing only in
hunk-header offsets. No hunk was re-sited.

## Verification

- `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and
  `cargo fmt --check` for `halo2_proofs`: clean.
- `cargo test --workspace --release`: 10 suites, 91 tests, 0 failures. This
  matters here because `halo2_gadgets` proves and verifies real circuits
  through both changed paths.
- The two jobs most likely to catch a manifest mistake were run directly:
  `cargo build --benches --examples --all-features` and
  `cargo doc --all --all-features --document-private-items`, both clean.

