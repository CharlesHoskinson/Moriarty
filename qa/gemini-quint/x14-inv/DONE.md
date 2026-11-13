# X14 `INV` Soundness & Completeness Review — Completed Deliverables

All deliverables for the independent soundness and completeness review of the X14 BLS12-381 $\mathbb{G}_1$ addition-chain condition row `INV` (addition chain base `o = 1100`, condition bus `o+10 = 1110`) have been written under `qa/gemini-quint/x14-inv/`:

1. `qa/gemini-quint/x14-inv/REPORT.md` — Independent soundness and completeness review report answering Q0–Q3, detailing the algebraic proof, completeness conditions on distinct non-opposite inputs and padding rows, and the mutant verification matrix.
2. `qa/gemini-quint/x14-inv/x14_inv.qnt` (and `qa/gemini-quint/x14-inv/inv.qnt`) — Quint formal model of the 5-AIR addition chain (`PointsIn`, `L_add`, `X3`, `Y3`, `INV`) and `sink` over $\mathbb{F}_{13}$ with the sound module (`x14_inv` / `inv`), refinement invariant `invRow`, non-vacuity witness `nonVacuous`, and all five required mutant modules (`m_no_inv_row`, `m_no_recv`, `m_wrong_den`, `m_no_id`, `m_inv_valid_free`).
3. `qa/gemini-quint/x14-inv/x14_inv_test.qnt` (and `qa/gemini-quint/x14-inv/inv_test.qnt`) — Ten deterministic Quint unit tests covering honest completeness, padding completeness, same-point and opposite-point rejection, bus message drop/duplication rejection, and each of the five mutants.

**Verdict:** `SOUND` and `COMPLETE` (for distinct non-opposite affine points $x_1 \not\equiv x_2 \pmod p$ on active rows `is_valid = 1`, and subject to supplying a dummy witness with $x_2 - x_1 \not\equiv 0 \pmod p$ on inactive padding rows `is_valid = 0` if `FieldExpr` constraints are un-gated by `is_valid`).
