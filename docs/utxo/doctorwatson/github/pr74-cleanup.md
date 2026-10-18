# PR74 source cleanup candidate

Blocked source-only draft. No commit, push, PR edit or main merge.

- Isolated worktree: `/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017`
- Branch: `doctorwatson-utxo/pr74-integration-20261017`
- Pinned main: `1721457439dbd6d988d00c6c86d1cf648caf2a0c`
- PR head: `0861c75254f33b92194db0542f224215d47269a8`
- Merge base: `f7ff06ed05bcef5fc3540bdde70ddbf6402f5547` (GitHub recorded base; ancestor of main)
- Exact staged candidate tree: `8d4804002b720cdae1c07046ddd81f1f94e5b535`
- Candidate: 114 files changed, 7993 insertions(+), 2591 deletions(-)

## Resolutions

- `crates/mori-core/src/lib.rs`: Keep main sherlock_bounded_source public API/comment; retain PR declaration ordering with one replay, runtime and stage declaration.
- `crates/mori-core/src/sherlock/reference_oracles_tests.rs`: Keep main normal_oracle_inputs imports; retain PR formatting in remaining file. Do not reintroduce duplicate typed normal_program implementations.
- `crates/mori-core/src/sherlock/normal_oracle_inputs.rs`: Import sibling extra_oracle_programs now declared once by PR in sherlock/mod.rs; preserve all normal typed input definitions and main consumers.
- `.github/workflows/rust.yml`: Retain optional workflow_dispatch only; remove push/pull_request triggers. No workflow dispatched.
- `AGENTS.md; RUST_GUIDELINES.md; plugin README/develop SKILL/rust checklist/test_guidance`: Preserve current AGENTS Opus4.8/5.5 high, Gemini4 Argon high nonsecurity reviewer, Sol author, noForeman/localCI/native0 rules. Keep useful Rust checklist; replace extra maintainer agreement and unconditional pre-handoff Cargo execution with current user authorization, inspected local scope, source-only unrun reporting. Cargo commands documented with locked flags.
- `crates/mori-core/tests/fixtures/continuations/normal-oracle.json; examples/core/continuations/normal-oracle.json`: Restore exact pinned-main bytes; incoming PR path-only cleanup would rewrite historical oracle provenance. No oracle regenerated.

## Evidence and remaining work

Inspected stdlib-only plugin guidance tests:10 passed (exit0). Git whitespace
checks pass, no unmerged index entries or unstaged changes remain. These are
source/guidance checks only. Rust, compiler, SDK, model, circuit, native, wallet
and target checks were not run. Hosted CI was not dispatched. No keys or proofs
were regenerated. Full GitHub PR body, files, commits, empty comments/reviews,
per-path candidate hashes and remaining gates are retained in the JSON report.

PR author's macOS890-test report belongs to its earlier tree; it is not a pass
for this integration. Broad formatting changes also invalidate source-hash-bound
results and manifests; no historical evidence was rewritten to appear current.
The four incoming public static-binding hash updates remain unvalidated source
fixtures. The two frozen continuation oracle copies and generated Sherlock
normal/extra-normal literal files match pinned main exactly.

A concrete source blocker remains: PR's source closure misses29 tracked Rust
files already added by main. The coverage test's inspected walker includes their
roots; passing closure coverage cannot be claimed for this candidate. Opus owns
the identity/closure disposition and other security judgments. Missing paths:

- `crates/mori-core/src/sherlock/arithmetic_witness.rs`
- `crates/mori-core/src/sherlock/arithmetic_witness_normal_tests.rs`
- `crates/mori-core/src/sherlock/bounded_scalar_source.rs`
- `crates/mori-core/src/sherlock/bounded_scalar_source_tests.rs`
- `crates/mori-core/src/sherlock/layout_a/arithmetic_fuel.rs`
- `crates/mori-core/src/sherlock/layout_a/arithmetic_fuel_normal_tests.rs`
- `crates/mori-core/src/sherlock/layout_a/aux_normal_tests.rs`
- `crates/mori-core/src/sherlock/layout_a/auxiliary.rs`
- `crates/mori-core/src/sherlock/layout_a/constants.rs`
- `crates/mori-core/src/sherlock/layout_a/encoding.rs`
- `crates/mori-core/src/sherlock/layout_a/limits.rs`
- `crates/mori-core/src/sherlock/layout_a/mod.rs`
- `crates/mori-core/src/sherlock/layout_a/normal_tests.rs`
- `crates/mori-core/src/sherlock/layout_a/observation.rs`
- `crates/mori-core/src/sherlock/layout_a/observation_normal_tests.rs`
- `crates/mori-core/src/sherlock/layout_a/prepare.rs`
- `crates/mori-core/src/sherlock/layout_a/rows.rs`
- `crates/mori-core/src/sherlock/layout_a/state.rs`
- `crates/mori-core/src/sherlock/layout_a/state_decode.rs`
- `crates/mori-core/src/sherlock/layout_a/state_decode_normal_tests.rs`
- `crates/mori-core/src/sherlock/layout_a/state_encode.rs`
- `crates/mori-core/src/sherlock/layout_a/state_encode_normal_tests.rs`
- `crates/mori-core/src/sherlock/layout_a/state_encoding.rs`
- `crates/mori-core/src/sherlock/layout_a/state_encoding_normal_tests.rs`
- `crates/mori-core/src/sherlock/normal_oracle_inputs.rs`
- `crates/mori-core/src/sherlock/reference_bounded_extra_tests.rs`
- `crates/mori-core/src/sherlock/reference_bounded_tests.rs`
- `crates/mori-core/src/sherlock/reference_stream.rs`
- `crates/mori-core/src/sherlock/reference_stream_tests.rs`

## Substantive scope

The PR contains formatting plus dependency changes, boxed payloads, FENS/SRS
sharing, proving refusal recovery, source-closure changes, CLI publication order
and changed test expectations. These need full-candidate review; this merge
preparation makes no cryptographic or security correctness judgment. See the
JSON substantive list and manifest. No global formatting-only classification
is asserted.

Fresh non-author Gemini4 Argon high review and applicable Opus5.5/4.8 high
security review remain required under current AGENTS. Any later source edits
change the tree pin and stale affected results. Rust validation remains unrun
within the current zero-native boundary, and all formal/circuit/native/stagenet
and parent roadmap exits remain open. The broader Preprod goal stays paused.

The candidate is ready for source review and blocked from an acceptance or
merge claim. Do not regenerate deployment/key/proof artifacts or run the optional
manual hosted workflow as part of this cleanup.
