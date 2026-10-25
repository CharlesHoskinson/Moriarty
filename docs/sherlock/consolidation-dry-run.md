# Consolidation dry run (sherlock-s1)

Branch `sherlock-s1`, started at factory-u `0699194f9`. Merged with `--no-ff`, one branch at a time.

| Source | Tip | Result |
| --- | --- | --- |
| factory-a (branch name `factory-a2`) | `f4e88b8d6` | 2 conflicts, resolved (below) |
| factory-b (Moriarty-rollup line) | `c742fecf1` | 1 conflict, resolved (below); already contained factory-a2, so the 34 new commits are B-chain work |
| factory-u | `0699194f9` | already up to date |
| factory-u-kc | `0699194f9` | already up to date (no unmerged commits at dry-run time) |
| factory-u-c4 | `0699194f9` | already up to date (no unmerged commits at dry-run time) |

Merge result: 96 files, +11352 / -116 against factory-u. No source (`.rs`, Compact, Lean, Quint) conflicts; every conflict is in generated ledger files under `qa/sherlock/`. T0 gates passed on both merge commits (docs commit with `DW_DOCS_COMMIT=1`).

## Conflicts and resolutions

### factory-a2

- `qa/sherlock/runs.jsonl` (append-only run log): both sides appended. Resolved as the union, factory-u lines first, then the F4/F5/F8/F9/F12/F13 runs from factory-a2. File is not time-sorted after the union; consumers read it by run id. Every line parses as JSON.
- `qa/sherlock/status.json`: three hunks (`updated`/`head`, `lastFlip`, `iterationsSinceFlip`), each a scalar that differs between the branches. Resolved by taking the factory-u (HEAD) values, since they are newer (U035 flip, 2026-10-25T09:51Z). The per-predicate `passes` entries and `counts` merged automatically.

### factory-b

- `qa/sherlock/status.json` only (runs.jsonl, budgets.json, predicates.yaml merged cleanly). Same three scalar hunks plus a predicate tail hunk (U035 / C1 / C2 / F13 entries). Resolved by taking the factory-u (HEAD) side, which has U035 passing and C1 passing; factory-b's side had U035 failing from an older run. Verified afterwards that the predicate key sets equal factory-b's plus factory-u's (nothing dropped), and that the only `passes` differences against factory-b are U013, U016, U029, U032, U035 (all true on the merged line, as on factory-u).

## Recommendation for the real consolidation

- `status.json` and `runs.jsonl` are the only conflict sources. Prefer merging factory-b first (it already contains factory-a2), then factory-u; take `ours`/factory-u for scalar fields and re-derive `counts`.
- A conflict-free tool-assisted merge of these two files (union of runs, newest-run-wins per predicate, recomputed counts) would remove the manual step.
- Re-merge factory-u-kc and factory-u-c4 once they have commits beyond `0699194f9`.

## Counts after merge (`scripts/sherlock-next`, `scripts/doctorwatson stop-rule`)

A 10/10, B 5/9, C 1/6, U 23/35, F 12/13. First failing: B4 (`scripts/qa/check-b4.sh`, SRPOC-004). Stop-rule: STOP, escalate to owner, 9 iterations since the last flip (limit 3), last failing trace `qa/sherlock/traces/20261025T090541Z-F4.log`.

B counts: the B predicates present in status.json are B1, B2, B3, B5, B6, B7 (5 passing); B4, B8, B9 have predicates but no recorded run yet. COMMON.md says B 7/9; the recorded line says 5/9, so B5 and B9 are not the only ones missing from the status record.

## Part 2: verification of the merged line (sherlock-s1)

`scripts/doctorwatson check --set X`, one set at a time, 2026-10-25 23:47-23:50Z.

| Set | Result on sherlock-s1 | Source line | Regression? |
| --- | --- | --- | --- |
| F | F1-F3, F5-F13 pass (12/13); F4 fails exit 3 (heavy lock held) | factory-a2/b: 12/13, F4 held by heavy lock | no |
| U | 23/35. Failing: U014 U015 U017 U018 U019 U020 U023 U024 U031 U033 (all open slices), plus U-set passes include U013 U016 U029 U032 U035 | factory-u: 23/35 on its own status; factory-b: 18/35 | no (strictly better than b; same as u) |
| C | C1 and C6 pass (2/6); C2-C5 blocked by U014/U031/U023/U017 | factory-u C 1/6 | no (C6 newly passes) |
| A | A1 passes; A2 fails exit 3 "host unfit: another process holds the heavy lock", A3-A10 blocked by A2 | factory-a2/b: 10/10 recorded | not a regression: lock-blocked, not run |
| B | B1, B2 pass; B3 blocked by A5 (dependency on the lock-blocked A chain); B4 B6 B7 B8 B9 blocked transitively | rollup: B1-B4, B6-B8 pass (7/9) | not a regression: see below |

Heavy lock holder: the sherlock-b B5 stagenet job (`flock -w 3600 heavy.lock ... timeout 14400 memcap 48G`, started 16:18, running 1h30m at check time). A and B heavy predicates cannot be re-run on this line until it exits.

### B4 root cause (not a merge regression)

`scripts/qa/check-b4.sh` is byte-identical in `Moriarty-rollup` and `sherlock-s1`. It consumes untracked evidence under `target/sherlock-b4/stagenet-*/` (attempt.json, deploy/acceptValidity/settleValidity receipts, ready.json), and ready.json stores absolute paths into `/home/charles/repos/Moriarty-rollup/target/sherlock-b3/...` and the SRS. `target/` is gitignored, so B3, B4, B6, B7, B8 (all read `target/sherlock-b*`) cannot pass in any worktree other than Moriarty-rollup. The "B5/9 on sherlock-s1" count was just the carried-over ledger, and rollup's 7/9 comes from the same evidence being present there.

Consequence for the real consolidation: run the B set from the integration worktree that holds the evidence (Moriarty-rollup), or copy `target/sherlock-b*` and `target/sherlock-srs` there and rewrite the absolute paths in ready.json.

### Static evidence for A and B code

The sherlock-u side adds no changes to mori-recursion, the Compact contracts, tools/sherlock-sdk or the Loki runner. Diff of the merge against factory-b outside qa/, docs/ and models/ is only crates/mori-core/src/sherlock, crates/mori-backend/src/sherlock_machine, crates/mori simulate commands, and scripts/qa/check-u0{13,16,21,32,35}.sh.
