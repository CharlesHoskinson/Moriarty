# E4: `L_Fr` prover speed on the GPU (plan)

Loop item E4 of `research/sherlock-e-path-20261112/LOOP.md`; row E4 of the E1 to E6 table in `research/sherlock-m7-wrap-20261112/RESULTS.md` (worktree `~/Projects/Moriarty-wt/m7-wrap`). Results go in `RESULTS.md` beside this file.

## Goal and gate

Gate: a MEASURED `L_Fr` prove time and memory on the GPU path against the call-time budget, with every GPU increment giving proof bytes identical to the CPU path.

Budget (PROPOSAL section 6.3, `research/sherlock-processor-final-20261111/PROPOSAL.md`): the prove stages of one warm call on the full GPU path are 6.31 / 8.90 / 16.20 s (lo / mid / hi) and the whole warm call is 11.1 / 19.0 / 45.0 s, ESTIMATE. Those figures predate option E and have no `L_Fr` stage. So `L_Fr` has no budget of its own; it adds to the call. E4 reports the `L_Fr` prove time as an addition to these figures and as a fraction of the 19.0 s mid warm call.

## Inputs

- Prover: stark-backend Fr config (`~/.cache/m7-wrap/src/openvm-org_stark-backend`, branch `e2-lfr`, head 5f698b5) and `sherlock-lfr` (`~/.cache/m7-wrap/src/openvm-org_openvm`, branch `e2-lfr`, I10 commit c4bd312).
- CPU baseline from E2 I10, on a loaded host (load 16 to 37): proof 2,516,530 bytes, prove 728.8 s, keygen 253.5 s.
- GPU code: `crates/mori-gpu` (sppark NTT for BLS12-381 Fr, `fr_t` 8 x 32-bit Montgomery limbs = kernel K0).

## Worktrees and branches (all `e4-gpu`)

- Moriarty: `~/Projects/Moriarty-wt/e4-gpu` (from `e2-lfr`), docs and `crates/mori-gpu`.
- stark-backend: `~/.cache/m7-wrap/src/stark-backend-e4` (from `e2-lfr` 5f698b5).
- openvm: `~/.cache/m7-wrap/src/openvm-e4` (from `e2-lfr` c4bd312); its `[patch]` points at `../stark-backend-e4`.

Runner: `~/.cache/e4-gpu/run.sh <stage> <mem> <dir> cmd...` (heavy lock, `heavy-run --name e4-<stage>`, `CARGO_TARGET_DIR=~/.cache/e4-gpu/target`, logs `~/.cache/e4-gpu/logs/`).

## Steps

1. Profile harness: `crates/sherlock-lfr/tests/e4_profile.rs` (ignored test). One honest prove at `lfr_params()` on the real `L` proof, backend tracing spans on, proof bytes to `$E4_PROOF_OUT`, peak RSS, proving key cached in `$E4_PK_CACHE`.
2. CPU baseline on the quietest host available: prove time, stage profile, peak RSS, proof SHA-256 (the reference for byte identity).
3. Move the dominant stages to the GPU in tested increments, largest first. Expected order (to confirm by the profile): Merkle row hashing and digest layers (Midnight Poseidon over Fr, width 3), then the LDE (iDFT plus coset DFT) with the sppark Fr NTT, then the WHIR round DFT and Merkle, then sumcheck work if it dominates after that.
   - Hook: `MerkleHasher` gets default batch methods that return `None`; the Fr hasher in stark-sdk overrides them under a `gpu` feature that calls `mori-gpu`. The CPU path is unchanged when the feature is off or the GPU call fails.
   - Each kernel: a unit test against the CPU function on random input, then the full `L_Fr` prove with the proof SHA-256 equal to the CPU reference.
4. Measure the GPU path: prove time, peak host RSS, peak device memory, host load and GPU state at the time.

## Rules

No installs, no new crates outside existing lock versions, no nightly. Zero knowledge stays end to end: the GPU code runs on the user's machine only. Every number is labelled MEASURED, DERIVED or ESTIMATE, with the host load.
