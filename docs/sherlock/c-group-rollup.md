# C group rollup (C1-C6)

Status snapshot: 2026-10-25, branch sherlock-s3 at 0699194f9 (factory-u). Source of truth is
`qa/sherlock/predicates.yaml`; this file is the human map and must be updated as U items land.

## Mechanism

Every C item uses `scripts/qa/check-c.sh`, which has no check of its own and exits 0. A C item
passes only when every U predicate in its `depends` list has passed (the runner enforces order).
So proving C2-C6 means proving exactly the U items below; there is no separate C gate to write.
Re-run `scripts/doctorwatson check --set C` after the U items pass.

Gate kinds: `scripts/qa/check-uNNN.sh` is a repo script; `qa/holdout/run.sh uireq-NNN` is an
evaluator-owned sealed holdout (never write qa/holdout, never set DW_SKIP_T0).

## Roll-up table

Status column is the last recorded result in this worktree's `qa/sherlock/status.json`
(updated 2026-10-25T11:32Z, stale: U021 and U026 landed in commits 0c74e4dbf and 0699194f9
but have not been re-checked into status.json yet).

| C | Requirements | Depends on (U) | Gate per U item | Status / owner |
|---|---|---|---|---|
| C1 | UIREQ-001..012 | U001-U012 | mixed | passes |
| C2 | UIREQ-013..016, 032 | U013, U014, U015, U016, U032 | U013 `check-u013.sh`; U014 holdout uireq-014; U015 holdout uireq-015; U016 `check-u016.sh`; U032 `check-u032.sh` | U013, U016, U032 pass; U014, U015 open (sherlock-u2, quint-1, lean-3) |
| C3 | UIREQ-022, 027, 031, 034, 035 | U022, U027, U031, U034, U035 | U022 holdout uireq-022; U027 holdout uireq-027; U031 holdout uireq-031; U034 `check-u034.sh`; U035 `check-u035.sh` | U022, U027, U034, U035 pass; U031 open (sherlock-c4) |
| C4 | UIREQ-023, 024, 028..030, 033 | U023, U024, U028, U029, U030, U033 | U023/U024/U028/U029/U033 holdouts uireq-NNN; U030 `check-u030.sh` | U028, U029, U030 pass; U023, U024, U033 open (sherlock-c4, quint-3, lean-3) |
| C5 | UIREQ-017..020 | U017, U018, U019, U020 | all four holdouts uireq-017..020 | all open (sherlock-u2, quint-2, lean-2) |
| C6 | UIREQ-021, 025, 026 | U021, U025, U026 | U021 `check-u021.sh`; U025 holdout uireq-025; U026 holdout uireq-026 | U025 passes; U021 and U026 implemented at 0699194f9 (model_package.py, contracts U021.json/U026.json), need a fresh check run (lean-1) |

## Remaining U items that gate C (per C)

- C2: U014, U015
- C3: U031
- C4: U023, U024, U033
- C5: U017, U018, U019, U020
- C6: U021, U026 (re-check), nothing else

Total open U items blocking C: U014 U015 U017 U018 U019 U020 U021 U023 U024 U026 U031 U033
(12; matches U 23/35 in status.json). C1 is already green; C2-C6 flip as soon as their list is clear.
U014-U016 are coupled (checkpoint, resume, chunk transparency), so C2 cannot flip before U014/U015 land.

## Update log

- 2026-10-25: initial map by sherlock-s3.
- 2026-10-26: sherlock-eval reports uireq-014..020 PASS on factory-u (C2/C5 U-side clear there; flip after the consolidation tree re-checks those ids). U031 export plans module landed on sherlock-s3 `b705bfacb` (CLI wiring: sherlock-s5); C3 then needs only a U031 re-check. B9 waits on sherlock-b READY-MERGE.
- 2026-10-26 (sherlock-s3 on consolidation bbc464a08): re-checked gating U + C. U014 PASS 20261026T035130Z-U014, U015 PASS 20261026T035133Z-U015, U017 PASS 20261026T035133Z-U017, U018/U019 PASS 20261026T035134Z-U018/U019, U020 PASS 20261026T035134Z-U020 -> C2 PASS + C5 PASS (20261026T035141Z-C2 / -C5). U023 FAIL 20261026T035134Z-U023 (spec-gate inventory: no native consumer for many transitions, recursion adapter not wired, u023 stagenet qualification record not committed; owners c4/s6) -> C4 BLOCKED-BY U023. U031 FAIL 20261026T035134Z-U031 on this base (export-plans module is on sherlock-s3-u031/s9 line, not in bbc464a08; s9 holdout PASS at 93f61a41c, awaiting its READY-MERGE) -> C3 BLOCKED-BY U031. Counts now C 4/6, U 33/35.
- 2026-10-26 (consolidation line sherlock-s10, supersedes the s3-line runs above): C2 PASS run `20261026T034453Z-C2` and C5 PASS run `20261026T034453Z-C5` on sherlock-s10 `c742e0136`. At fable's 03:49Z re-task, C3 was still blocked on the U031 union + HMERGE E=`7f0942c3d`; that has since landed: eval HMERGE `9e5617e6d` (holdout = eval `7f0942c3d`) into sherlock-s10, s9 `461e76d8c` merged (`82cbf4f4e`), eval confirmed uireq-031 PASS at E=`7f0942c3d` on the s9 binary, then U031 PASS run `20261026T035253Z-U031` and C3 PASS run `20261026T035312Z-C3` on sherlock-s10 `369281b01`. C4 remains blocked on U023: fold-path defect (append_helper step 35, eval_schedule step 0, Match steps 60-62; lead: `progctx_em.emit` walk; owners c4/s6, s2/s4/s7 WIP). C6 PASS (run `20261025T234955Z-C6`, carried). Consolidation counts at `369281b01`: C 5/6, U 34/35, only U023 (hence C4) and F4 open.
- 2026-10-26 04:02Z (fable, consolidation sherlock-s10): F4 REAL PASS run `20261026T040123Z`; U024 closed `035738Z` from c4's committed receipt+trace. Board 71/73: OPEN only U023 + C4 (fold-path defect; s6 root-causing, hypothesis: program-context canonical emitter walk vs owner PROG encoding on tree_fold). s3 standby: no duplicate checks vs s10; final docs pass + C4 flip on sherlock-final once U023 lands.
- 2026-10-26 04:32Z (fable): U023 fold-path fix landed on sherlock-s6 `d570c75cc` (stale-hold PAT width in program-context walk; duplicated in validate walk). s2/s7/s4 merging and re-running their fragments; C4 flip (this line, after U023 passes on the owning line): `scripts/doctorwatson check U023 U024 U028 U029 U030 U033 C4`.
