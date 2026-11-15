# Scheduling: findings of the Lean oracle (Formal-Lean-B, 2026-11-15)

No finding. Every invariant of STM and ELA that the oracle states is proved for every state (no
counterexample), and the recorded T16 values satisfy their bounds.

| ID | Kind | Evidence | Proposed verdict |
|---|---|---|---|

Checked and holding (not findings):
- STM-017 with STM-018: the tie order is a strict total order on picks of distinct shape and device
  (`better_total`, `pick_order_independent`), and a lower budget never gives a faster pick for any
  profile (`pickAt_mono`), including a pick that moves to another shape (`pin_stm018`). GUARD HOLDS.
- STM-013 with STM-009 and STM-014: the scheduler's own greedy cut passes `check_free` for every run
  length (`schedule_cut_free`). GUARD HOLDS.
- ELA-001, ELA-005, SPL-011: no schedule, limit, box or profile operation reaches the prepared run
  (`run_keeps_prepared`). GUARD HOLDS (in the model; the T25 rerun is the spec's open item).
- The recorded T16 picks h-gpu, h-nogpu, h-cap, o-gpu, o-nogpu and the DERIVED section 3.2 estimates
  are reproduced from the MEASURED calibration table (`RESULTS-CDE.md:45` to `:50`).

Status notes (not findings; the specs already state them): STM-014 code/result-file disagreement
(GAP), STM-016 path not read by code (GAP), STM-004 multi-GPU calibration (OPEN), ELA-005 muninn row
(PARTIAL).
