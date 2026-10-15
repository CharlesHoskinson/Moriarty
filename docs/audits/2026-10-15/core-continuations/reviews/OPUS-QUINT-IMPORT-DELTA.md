# Opus delta classification — lifecycle test import fix

**Verdict: APPROVE_ORDINARY** for exactly the two commands in `QUINT-LIFECYCLE-TEST-FIX-CHECKS.json` (sha256 `2c64e3c4…c91c`). They run sequentially, stop at the first failure, use the 30s TERM/5s KILL wrapper, and run once each.

## What was checked (read-only, no execution)
- `diff` of the archived old test against the current test shows only two added lines, at 4–5. Both are `import continuation_prefix.{Escrow,Reserve} from "./continuation_prefix"`. The `.patch` (sha `ffa00d1a…efb`) has the same content.
- Current hashes match the delta pins:
  - test `dc39c999…6d6e`, archive `9a923041…7fc9`.
  - `continuation_lifecycle.qnt` `f8690924…7d`, `continuation_prefix.qnt` `f3036766…7f`, `continuation_prefix_test.qnt`.
  - INPUTS/LIFECYCLE-INPUTS/PROPOSED-CHECKS/PROPOSED-LIFECYCLE-CHECKS, plus the three md files.
  - the preserved stderr of failure 03, `8785adbc…0ab`.
- The argv of both commands matches prior commands 3 and 4 in `PROPOSED-LIFECYCLE-CHECKS.json` element for element. That covers the typecheck, the `--backend=typescript`, the nine-name anchored `--match`, `--max-samples=1` and `--seed=20261016`.
- The test file has 15 constructor uses (6 in `pure val` records and 9 in `initCase(...)`), which matches the QNT404 count. This supports the diagnosis: the names were not visible through `continuation_lifecycle.*`. `continuation_lifecycle.qnt:6-7` already imports the same two names the same way, and that form passed typecheck and run in 01/02.

## Rationale
`Escrow` and `Reserve` are nullary sum-type constructors of `Specimen`, which is already reachable through the lifecycle model's types. Importing them adds no new definitions and no `def`, action, `run` or external effect. The evaluation path is unchanged: the same pure lifecycle functions and the same nine runs. Model, prefix, oracle, literals and limits are unchanged. In my judgment the attack surface does not change.

## Preconditions — all prior ones carry over unchanged
1. Immediately before and after each command, root checks full `quint/dist/src`, `node_modules` and the Node binary against the same `QUINT-EXECUTION-PINS` hashes.
2. Per-model-directory listing and the Quint cache are unchanged relative to the current state. The only expected new artifacts are capture files under `R/checks/`.
3. `NODE_OPTIONS`, `NODE_PATH` and `QUINT*` are cleared. Log only that they were cleared, not their values. `HOME`/`CODEX_HOME` stay unchanged.
4. cwd is exactly `R/quint-continuations`. No network/install/verify, Rust backend, Apalache, Cargo/SDK/native/VM/proof/keygen/ledger/key/SRS/cache actions.
5. **One pin refresh is permitted:** the input pin for `continuation_lifecycle_test.qnt` changes to `dc39c999…6d6e`, sourced from `QUINT-LIFECYCLE-TEST-IMPORT-FIX-PINS.json`. Every other input pin keeps its prior value. `LIFECYCLE-INPUTS.json` and the 01–03 captures stay immutable. Before dispatch, re-hash the test file and abort on any mismatch.
6. New captures go to fresh directories (for example `quint-lifecycle-03b`/`04`) and never overwrite `quint-lifecycle-03`.

## Residuals (unchanged, not resolved by this verdict)
- This is not an assessment of model correctness, Rust/compiler correspondence or security acceptance. Exit 0 means only that nine literal expectations held under the TS evaluator with one sample each.
- The 01/02 results stand. They are not rerun or re-attested here.
- Trust in the Quint 0.32.0 toolchain rests on the existing closure. I did not re-derive it.
- The frozen feature candidate `e9a2b80b` is unaffected. I made no change to the worktree and read no private paths.
- If 03 fails again, stop. Any further source change needs a new delta review (**NEEDS_SOURCE**), not an in-place retry.

I wrote only this file. Nothing was executed.
