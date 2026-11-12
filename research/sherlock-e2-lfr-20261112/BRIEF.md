# E2 brief: the Fr recursion AIR set `L_Fr` that verifies one `L` proof

Owner ruling 2026-11-12: OD-24 is option E. E1 passed. E2 is the long pole of the path (6 to 12 weeks, ESTIMATE).

Read first:
- `research/sherlock-e1-fr-backend-20261112/RESULTS.md` (the Fr base-field config `bls12381_fr_poseidon.rs`, its tests, and the open tag point).
- `research/sherlock-m7-wrap-20261112/RESULTS.md` sections 2b to 2d (the E2 row, the `L_Fr` model, and the wrap rows that `L_Fr` must make possible), and `PLAN.md`.
- `research/sherlock-m7-wrap-20261112/lfr/` (the real 430,073-byte `L` proof setup: `l_fr.rs`, `run.sh`).
- `research/sherlock-final-design-20261111/ARCHITECTURE.md` for the `L` interface (12,164 public words) and the hash domains (section 3.6 rules of PROPOSAL.md).

## Goal (the E2 gate)

`L_Fr` proves over the real `L` proof and the stock verifier accepts it. Negative controls refuse a changed `L` proof byte and a changed public word. Heights are MEASURED, then D3 runs at the measured shape.

## Work

1. Write `PLAN.md` beside this brief first. Choose one route: a port of OpenVM's recursion crate (the structural template, about 35,297 lines), or a dedicated `L` verifier. Base the choice on what `L` actually uses, and give the reasons in one table. Break E2 into increments, each with its own test gate. Example increments: BabyBear and EF arithmetic emulated in Fr with range-checked reduction mod p; a Midnight Poseidon sub-AIR; proof shape; GKR; zerocheck; stacked reduction; WHIR.
2. Implement the increments in order. Commit each increment only after its tests pass.
3. Give `L_Fr`'s Fr transcript its own hash tag, distinct from `L`'s tags for u = 1 and u = 3. This follows the E6 plan (OD-18). The codes are config fields.

## Source locations

- Backend: `~/.cache/m7-wrap/src/openvm-org_stark-backend`, branch `e2-lfr`. It starts at `e1-fr-base`.
- OpenVM: `~/.cache/m7-wrap/src/openvm-org_openvm`. It is on `main` with uncommitted bench changes. Create a branch `e2-lfr` there. Commit those existing changes as its first commit, with a message that names them as the M7 bench crate.
- Moriarty worktree: `~/Projects/Moriarty-wt/e2-lfr`, branch `e2-lfr`. It starts at `e1-fr-backend`. Results go in `research/sherlock-e2-lfr-20261112/`.

## Rules

- Builds and tests run only via `flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run --mem <size> --name e2-<stage> -- ...`. A job over 30 minutes runs as a `systemd-run --user` unit. Logs go in `~/.cache/e2-lfr/`. Use `CARGO_TARGET_DIR=~/.cache/e2-lfr/target`.
- Install nothing. Do not add crates outside the versions already in the openvm or backend lock files. If something new seems needed, stop and report it.
- Never use Ollama or vault. Never set API keys.
- Git: commit only on the three `e2-lfr` branches. Stage explicit paths. Never push. Never touch main. Never use --no-verify. Moriarty docs commits use `DW_DOCS_COMMIT=1`, and the pre-commit hook takes 3 to 5 minutes. A retry is fine when only factory timing or lease tests fail. No AI attribution or Co-Authored-By lines.
- Label every number MEASURED, DERIVED or ESTIMATE.
- Keep `RESULTS.md` current after each increment (STE register, no process narration). It states what is done and tested, the measured heights, and the next increment.
- This is weeks of work. Go as far as you can with tested increments. Then stop with RESULTS.md exact enough that a fresh agent can continue from it.

## Hand-back

At most 5 lines plus the RESULTS.md path: the route chosen, the increments passed, the commits, and the next increment.
