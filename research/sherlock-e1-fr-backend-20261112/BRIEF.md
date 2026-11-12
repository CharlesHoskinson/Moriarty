# E1 brief: stark-backend with a BLS12-381 Fr base field

Owner ruling 2026-11-12: OD-24 is option E. E1 is the first step. Read first:
`research/sherlock-m7-wrap-20261112/RESULTS.md` sections 2b to 2d (the E1 row, the 48-site list at lines 172 to 173) and `PLAN.md`, plus `lfr/` (the M7b `L` proof over Fr, which already adds a Midnight Poseidon duplex over Fr for a BabyBear base).

## Goal (the E1 gate)

stark-backend and cpu-backend accept `F = Bls12381Fr` as the base field with `EF = F` (degree 1). A small AIR proves and verifies end to end at the Fr base with negative controls (a flipped proof byte and a changed public value are refused). The D3 security terms at the configured Fr parameters equal RESULTS.md section 2b.2.

## Work

1. Source: `~/.cache/m7-wrap/src/openvm-org_stark-backend` (main a6b4761 plus the uncommitted M7b diff, saved as `lfr/stark-backend.diff`). Create a branch `e1-fr-base` in that clone. Commit the M7b diff as its first commit, then work there. Field crate: `~/.cache/m7-wrap/src/p3-bls12-381-fr`.
2. Relax `PrimeField64` to `PrimeField` (or a narrow new trait) at the 48 sites. Keep BabyBear behavior bit-identical: the existing BabyBear tests must pass unchanged, and an existing BabyBear proof must stay byte-identical.
3. Write sampling (`sample_bits`) and grinding for a 255-bit field, with an argument for uniformity in RESULTS.md.
4. Use a native Fr duplex transcript and Midnight Poseidon Merkle trees (u = 1, 2, 3), following the section 3.6 hash-domain rules.
5. Add the small-AIR end-to-end test, the negative controls and the Fr D3 check.

## Rules

- Builds and tests run only via `flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run --mem <size> --name e1-<stage> -- ...`. Any job over 30 minutes runs as a `systemd-run --user` unit. Logs go in `~/.cache/e1-fr/`. Use `CARGO_TARGET_DIR=~/.cache/e1-fr/target`.
- No installs of any kind (no new toolchains, no crates from outside the existing lock files without stopping to report). If one seems needed, stop and report it.
- Never use Ollama or vault. Never set API keys.
- Git: commit only on `e1-fr-base` (backend clone) and on `e1-fr-backend` (worktree `~/Projects/Moriarty-wt/e1-fr`). Stage explicit paths. Never push, never touch main, never use --no-verify. Docs commits in Moriarty use `DW_DOCS_COMMIT=1` and the pre-commit hook takes 3 to 5 minutes. Run it in the background. No AI attribution or Co-Authored-By lines in any commit.
- Label every number MEASURED, DERIVED or ESTIMATE.
- Write `research/sherlock-e1-fr-backend-20261112/RESULTS.md` in the worktree (STE register, no process narration). Save the backend diff as `e1.diff` beside it.
- E1 is estimated at 3 to 6 days. Work in self-contained, tested increments and commit each one. If you must stop, leave RESULTS.md stating exactly what is done, what is tested and what is next.

## Hand-back

At most 5 lines plus the path to RESULTS.md: gate status, commits, the next step.
