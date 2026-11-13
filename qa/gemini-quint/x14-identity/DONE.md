Completed the X14 identity-point flag path soundness and completeness review.

Files written under `qa/gemini-quint/x14-identity/`:
- `x14_identity.qnt`: Quint 0.32.0 formal model (`x14_defs`, sound module `x14_identity` with `invRow` and `invNonVacuous`, and mutant modules `m_no_zero_limbs`, `m_e_ignores_flag`, `m_sel_wrong`, `m_out_flag_wrong`, `m_out_mult_v`).
- `x14_identity_test.qnt`: Deterministic Quint test suite (`curveGroupLawTest`, `honestAddVerifyTest`, `honestDblVerifyTest`, `negFlagOnRealPointFailsOnlyPointsInTest`, `negConsumerTakesChainSumFailsLogUpTest`, and all five mutant detection tests).
- `REPORT.md`: Full review report with verdicts, boundary conditions, mutant table, answers to questions (1)-(3), and measured negative test analysis.
- `DONE.md`: Completion record.

Verdict: **SOUND and COMPLETE** subject to the documented upstream producer binding conditions (both flags and limbs bound to the transcript encoding) and active `add`-row distinct-$x$ condition ($x_1 \neq x_2$).
