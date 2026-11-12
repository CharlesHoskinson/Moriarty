# gpu-prover: the GPU path of the Sherlock wrap prover

Separate workspace (own `Cargo.lock`) so that the hook-patched midnight-proofs never
reaches the root workspace (design 3.7, gpu.md F6, RPP-007).

- `vendor/midnight-proofs-0.8.3-gpu/` = registry `midnight-proofs 0.8.3` (archive
  SHA-256 `19b2e4b1acb581ad741b0cb7c0e9dbdf51142f4e9518886f25fcff40a6702dcc`) plus
  `patches/gpu-hooks.diff` (`src/hooks.rs` registry; hooks in `msm_specific`, the
  `EvaluationDomain` transforms, `compute_nu_poly`; the quotient job builder in
  `plonk/evaluation.rs`; a test-only fixed lookup leftover order). With no hook
  registered the crate is the registry crate. The patch also adds per-phase timers
  (`hooks::PHASE_SECONDS`).
- `vendor/midnight-circuits-7.3.0-seed/` = registry `midnight-circuits 7.3.0`
  (archive SHA-256 `79ba23314560e105bb1da33551199ae9abaa25188589d8308caddbcd8d1f3448`,
  Apache-2.0) plus `patches/chip-seed.diff`: when `GW_CHIP_SEED` is set, the foreign
  ECC chip draws its incomplete-addition point from a seeded RNG instead of `OsRng`.
  That point is a witness cell, so without the seed two proofs of one witness differ.
  Test-only; unset, the crate behaves as the registry crate.
- `src/lib.rs` registers `crates/mori-gpu` in the hooks; phases switch at run time.
- `src/bin/wrap_prove.rs` proves the run decider (or the k = 19 proxy) on the CPU
  path and on GPU paths with one key, verifies each proof with the library verifier
  and the ledger 9 `VerifierKey::verify`, and compares proof bytes under a test seed.

Build: `CARGO_TARGET_DIR=~/.cache/gpu-wrap/target-gp cargo build --release --bin wrap_prove`
(needs CUDA 13.4 at `/opt/cuda`). Results: `research/sherlock-gpu-wrap-20261111/RESULTS.md`.
