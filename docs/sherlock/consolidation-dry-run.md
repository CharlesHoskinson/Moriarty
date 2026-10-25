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
