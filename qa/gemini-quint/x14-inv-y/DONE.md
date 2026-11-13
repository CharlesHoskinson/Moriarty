Completed the independent soundness and completeness review of the X14 BLS12-381 G1 doubling condition row `INV_y`.

Files written under `qa/gemini-quint/x14-inv-y/`:
- `qa/gemini-quint/x14-inv-y/inv_y.qnt`: Quint 0.32.0 formal model over F_13 with the sound reference module (`inv_y`, invariants `invRow` and `invNonVacuous`) and all five required mutant modules (`m_no_inv_row`, `m_no_recv`, `m_wrong_den`, `m_no_id`, `m_inv_valid_free`).
- `qa/gemini-quint/x14-inv-y/inv_y_test.qnt`: Quint unit and property test suite (`inv_y_test`) covering honest completeness, unchecked `PointsIn` reachability of `(0, 0)`, single-cell fault rejection, bus drop/duplication rejection, and mutant detection.
- `qa/gemini-quint/x14-inv-y/REPORT.md`: Full review report with verdict (`SOUND/COMPLETE`), conditions, architecture analysis, mutant table with exact `quint` commands and expected exit codes, and answers to all review questions.
- `qa/gemini-quint/x14-inv-y/DONE.md`: This completion summary.

Verdict: **SOUND/COMPLETE** (**SOUND and COMPLETE**, subject to the stated input limb range-check, active consumer binding, trace height bound, and `add`/`dbl` bus offset separation conditions).
