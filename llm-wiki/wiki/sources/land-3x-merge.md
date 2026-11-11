---
title: "land-3x merge: dispatch partition and U023c receipt"
type: source
tags: [dispatch, receipt, tasks, land-3x]
sources: [land-3x-merge, final-design-decisions]
last_updated: 2026-11-10
date: 2026-11-11
source_file: ~/Projects/Moriarty/openspec/changes/sherlock-vm-consensus-20261107/tasks.md; ~/Projects/Moriarty/crates/mori-backend/src/sherlock_machine/testdata/receipts/dispatch.json
source_url: local
---

# land-3x merge: dispatch partition and U023c receipt

## Summary

Main commit `e39bc7af8` (2026-11-10 23:08 local) merges branch `land-3x`: the dispatch partition, the dispatch receipt `crates/mori-backend/src/sherlock_machine/testdata/receipts/dispatch.json` and status notes for tasks 3.1 to 3.4 and 4.1 in `tasks.md`. The merge is on `origin/main`. The source for this page is the `tasks.md` notes on main and the receipt. **No task is ticked.** This settles the earlier log note that "U023c 6 of 6" was not found in any repository file: it is now in the 3.3 note and in the receipt (`"passed": 6`, `"failed": 0`).

## Key Claims

- 3.1 (U023a, SAS-014 diagnostic): run `20261111T055301Z-land3x-u023a` (exit 0): build 23.13 s, `ProofPreimage::check` 3.01 s (accepts), 108 rules between 1,273,895 and 1,274,018 IR instructions, selector 1,279,926, statement 2,129 lanes (unlabeled). Reduced coverage, not an acceptance receipt. Not ticked: `U023a` in `predicates.yaml` is still `unimplemented.sh`.
- 3.2 (U023b, partition): `partition.json` reblessed on main `c57351dd4` (only `source_sha256` changed, still 3,153 classes); exclusivity, drift and directed-case selection pass in run `20261111T013913Z-land3x-u023c`. Not ticked: 291 classes stay unrealized and 5 rules (`XAdtEvalConstructNullary`, `XAdtEvalConstructFields`, `XAdtCasesPlanHit`, `XAdtCasesPlanMiss`, `BHelperValidateInvalid`) have no reached or directed case.
- 3.3 (U023c, split tests and receipt): T2 run `20261111T013913Z-land3x-u023c` (exit 0, commit `2947727d6`, 6 passed, 9,224 s, 22.8 GB peak RSS, run figures unlabeled): fired-rule SDK acceptance on every step (849 checks, 72 circuits), directed SDK cases (72 rule representatives, 583 checks), partition, concrete selection, mutations. Receipt `dispatch.json` (label heuristic), trace copy `qa/sherlock/receipt-traces/20261111T013913Z-land3x-u023c.log`; `RECEIPTED` lists `dispatch`; `verify_receipt` accepts it, and the U023 holdout (run `20261111T055226Z-land3x-u023-holdout`) no longer reports the dispatch-owner gate. Not ticked: one run, not one budget key per class group, and the SDK exclusivity check covers 72 of 108 supported rules (one representative each), not every class.
- 3.4 (U023d, fragments): the fragment suites pass under the SDK check: `c6_admin_publish` (4 passed), `c6_validate_helper` (4), `c6_equal_helper` (4), `c6_match_helper` (5), `c6_scalar_construct` (4) (test counts, unlabeled), all at commit `2947727d6`. Open: per-rule receipts, trace copies, `RECEIPTED` entries and the `native_inventory.rs` row flips.
- 4.1 (M1' measurement, `k_U` probe): note of 2026-11-10 (runs `20261110T051216Z-t3x-m1` and `20261110T055103Z-t3x-ku1`, `cost_model` only): the selector-only S=1 leaf is k=22 (2,747,489 rows) before any fragment, so `k_U` > 17 at S=1. Not ticked; the amended task runs after 4.2b over the frozen `R`.
- Lead note O-1 ([[final-design-decisions]]): a true merge keeps the receipt's bound commit `2947727d6` valid; U023a, U023b and U023c are still `unimplemented.sh`.

## Key Quotes

- 3.3: "Not ticked: one run, not one budget key per class group, and the SDK exclusivity check covers 72 of 108 supported rules (one representative case each), not every class."

## Connections

- [[WorkstreamStatus]] (3.x dispatch partition row). Lead record: [[final-design-decisions]] O-1.

## Contradictions

- Settles the log notes of 2026-11-10 21:31 and 21:36, which said the U023c 6 of 6 result was not in any repository file, and the earlier WorkstreamStatus text, which said `dispatch.json` did not exist on the branch. Both are now in the main checkout.
