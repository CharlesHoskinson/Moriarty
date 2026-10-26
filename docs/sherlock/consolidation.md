# Consolidation inputs for sherlock-final (2026-10-26)

`sherlock-final` does not exist yet (per sherlock-eval). It should be built by a
consolidator (fable/s10/s1 line) as a true merge of the inputs below; holdout-carrying
merges are committed only by sherlock-eval via READY-HMERGE (coordinator ruling).
Base line today: sherlock-s10 at `369281b01` (A 10/10, B 9/9, C 5/6, U 34/35, F 12/13;
open: U023 -> C4, and F4).

## Already merged into sherlock-s10 (verify, do not re-merge blindly)

| Input | Sha | Carries | Merged as |
|---|---|---|---|
| sherlock-s1 | `d72a12b47` | A1/A2 real-run ledger | `248724655` |
| sherlock-s8 | `232307247` | U023 C6 digest leaf binding-lane fix, M2 proves | `ded4a5a19` |
| sherlock-s9 | `461e76d8c` (+ trace copy `b035b567d`) | U031 passing check (s3 export_plans union, s5 flag plumbing) | `82cbf4f4e` |
| eval HMERGE | E=`7f0942c3d` | Strengthened uireq-031 holdout | `9e5617e6d` |
| sherlock-s10 own | `c742e0136`, then `369281b01` | C2/C5 PASS (`20261026T034453Z`), U031/C3 PASS (`20261026T035253Z-U031`, `20261026T035312Z-C3`) | tip |

## READY-MERGE inputs not yet in the consolidation line

| Input | Sha | Carries | Status |
|---|---|---|---|
| sherlock-s5 | `3fabcca1c` | U031 simulate `--export-plan` wiring to core export_plans | Caveat: its core exporter blob is pre-repair; s9 union keeps s3 blob `ce73a4524`. Do not take s5's exporter. Ledger entry is s9/s10's, not s5's. |
| sherlock-s3 (-c) | `ae138b0ba` on `origin/sherlock-s3-c` | C2/C5 PASS ledger on bbc464a08 base + this docs reconciliation | READY-MERGE sent; ledger superseded by s10 runs above, docs are the live part |
| c4 U024 trace copy | via sherlock-s10 | U024 PASS `20261026T004810Z-U024` already in s10 status.json | Verify trace copy lands with final |
| sherlock-gpr | `47673b635` | M2/M3/M4 measurements, GP stand-in bundle; S-steps-per-leaf spec for s8 | Pending fable routing |
| U023 WIP (s2/s4/s6/s7) | s2 `dd286467f`, s4 `36456d510`, s7 `5d7663275`, s6 (uncommitted root-cause) | Match/helper fragments for the tree_fold fold-path defect (append_helper s35, eval_schedule s0, Match s60-62; lead: `progctx_em.emit` walk) | Not ready; U023 is the last U blocker, gates C4 |

## Remaining to finish

- U023 fix + `doctorwatson check U023 C4` on the final line (only C item open).
- F4: `scripts/doctorwatson check F4` in a heavy.lock-quiet window (fable runs it).
- Re-verify all groups on `sherlock-final`, update SCOREBOARD/design docs, chip
  animation (`visualization/`) and Pages per COMMON.md.
