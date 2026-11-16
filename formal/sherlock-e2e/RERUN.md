# Lead rerun of the sherlock-e2e Quint models (2026-11-16)

The lead reran every `quint` command of each context's `quint/REPORT.md` (594 commands, 17 contexts) on muninn (aarch64), quint 0.32.0, in `heavy.slice`, 6 contexts in parallel. The slice B contexts were also rerun on odin (quint 0.33.0) on 2026-11-15 (`replay/STATUS.tsv`).

Method (`scripts/rerun_parse.py`, `scripts/rerun_module.py`, `scripts/rerun_strict.py`):
- The parser takes each `quint typecheck|test|run` command and its expected exit code from the REPORT. Each command runs from `<context>/quint/`. The `--out-itf` path points to a scratch directory, so the stored traces stay as they are.
- A row agrees only if the exit code matches and the output matches it:
  - an expected-1 run printed `[violation]` / `Invariant violated`;
  - an expected-0 run printed `[ok]`;
  - a test printed a passing count above zero, equal to the REPORT, with no failing test.
- No REPORT has a `quint verify` (Apalache) command, so none ran.

## Result: every model passes the rerun

| context | commands | typecheck | quint test (passing) | honest runs `[ok]` | witness and mutant runs violated | agree | replay | traces committed |
|---|---:|---|---|---|---|---|---|---|
| aggregation | 45 | 2/2 | 17, 2 | 4/4 | 37/37 | AGREE | agrees (odin, STATUS.tsv) | no (90 MB, skipped) |
| assurance | 38 | 2/2 | 8 | 4/4 | 31/31 | AGREE | agrees (odin, STATUS.tsv) | no (11 MB, skipped) |
| box-fleet | 43 | 2/2 | 10 | 4/4 | 36/36 | AGREE | agrees (odin, STATUS.tsv) | no (20 MB, skipped) |
| bridge | 48 | 2/2 | 10, 1, 1 | 4/4 | 39/39 | AGREE | agrees (odin, STATUS.tsv) | no (36 MB, skipped) |
| call-flow | 25 | 2/2 | 4 | 4/4 | 18/18 | AGREE | not run | yes (0.6 MB) |
| compact-compilation | 28 | 2/2 | 5 | 4/4 | 21/21 | AGREE | not run | yes (0.8 MB) |
| conformance | 26 | 2/2 | 4 | 4/4 | 19/19 | AGREE | not run | yes (0.6 MB) |
| governance | 26 | 2/2 | 5 | 4/4 | 19/19 | AGREE | not run | yes (1.1 MB) |
| key-management | 29 | 2/2 | 16 | 4/4 | 22/22 | AGREE | agrees (odin, STATUS.tsv) | no (12 MB, skipped) |
| leaf-proving | 43 | 2/2 | 18, 1, 1 | 5/5 | 33/33 | AGREE (test count 18 vs REPORT 17: R-leaf-1 regression added after the REPORT) | agrees (odin, STATUS.tsv) | no (21 MB, skipped) |
| ledger-verification | 24 | 2/2 | 6 | 4/4 | 17/17 | AGREE | not run | yes (0.8 MB) |
| network-operations | 45 | 2/2 | 5 | 4/4 | 38/38 | AGREE | not run | yes (0.9 MB) |
| placement | 23 | 2/2 | 5 | 4/4 | 16/16 | AGREE | not run | yes (0.9 MB) |
| scheduling | 36 | 2/2 | 20 | 4/4 | 29/29 | AGREE | agrees (odin, STATUS.tsv) | no (38 MB, skipped) |
| settlement | 38 | 2/2 | 6 | 4/4 | 31/31 | AGREE | not run | yes (0.6 MB) |
| sherlock-execution | 39 | 2/2 | 5 | 4/4 | 32/32 | AGREE | not run | yes (1.0 MB) |
| wrap | 38 | 2/2 | 11 | 4/4 | 31/31 | AGREE | projected only (1,040 chains), oracle replay not run | no (14 MB, skipped) |

All 17 models AGREE, so every model is committed as passing its own Quint check. A model whose MODELED rows need replay stays `pending` in COVERAGE-FORMAL-AC.md until the replay runs.

Notes:
- leaf-proving: the main test module passes 18 tests, and the REPORT says 17. The 18th is `lp_R_leaf_1_N1WaitsForLeafProofsReadyTest`, the R-leaf-1 regression that was added after the REPORT (2759f4d61).
- Trace determinism: for the nine A/C contexts other than wrap, the 20 regenerated traces have the same `states` as the stored `traces/*_main/t_*.itf.json`. For wrap, the stored `wrap_main` has 100 traces from a run with other sampling parameters, and the 20 regenerated traces differ from them. The wrap traces are not committed (see below).
- Replay: only the seven slice B contexts have an oracle replay (odin, STATUS.tsv). For the ten A/C contexts the replay was NOT run. The Formal-Quint lane stopped at the machine shutdown. Wrap has a projection only (`wrap/traces/projected/SUMMARY.json`, 1,040 chains, `replay/project_wrap.py`) and no `--step` run.

## Traces

The 5 MB limit applies to each context's `traces/` directory. Committed: the 20-trace `*_main` sets of call-flow, compact-compilation, conformance, governance, ledger-verification, network-operations, placement, settlement and sherlock-execution (each 1.1 MB or less). Not committed (over 5 MB; kept on disk in this worktree):

| directory | size | files |
|---|---:|---:|
| aggregation/traces | 87 MB | 843 |
| scheduling/traces (`scheduling_main`, `projected`) | 38 MB | 1,659 |
| bridge/traces | 37 MB | 828 |
| box-fleet/traces | 23 MB | 1,530 |
| leaf-proving/traces (`leaf_proving_main`, `projected`) | 22 MB | 487 |
| wrap/traces (`wrap_main`, `projected`) | 17 MB | 1,142 |
| assurance/traces | 12 MB | 708 |
| key-management/traces | 12 MB | 356 |

The regression traces already in git (`leaf-proving/traces/regressions/`, `scheduling/traces/regressions/`) do not change.

## Still needs a lane (not started here)

- Oracle replay (a `replay/project_<context>.py` projection plus the Lean oracle `--step` run, as for slice B) for the ten A/C contexts: call-flow, compact-compilation, conformance, governance, ledger-verification, network-operations, placement, settlement, sherlock-execution, wrap (finish the projection run).
- Carbon Quint QA (T15 Carbon QA driver) for the A/C findings that stay PENDING-CX in `tests/quint-regressions/COUNTEREXAMPLES.md`.
