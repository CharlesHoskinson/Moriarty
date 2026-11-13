# Soundness Review Complete (`x14-chain-id`)

Completed the independent soundness and completeness review of the Sherlock X14 `ChainAir` and `PointsIn` `id = row index` condition rows (`when_first_row: id = 0`, `when_transition: next.id = id + 1`, and `id`-prefixed `PermutationCheckBus` messages).

## Files Written
1. `qa/gemini-quint/x14-chain-id/chain_id.qnt` — Quint 0.32.0 specification over $\mathbb{F}_{13}$ (`chain_id` sound module first, followed by mutant modules `m_no_first_row`, `m_no_transition`, `m_step_two`, `m_id_not_in_msg`), with invariants `invRow`, `invNoSwapById`, `invComplete`, and `invNonVacuous`.
2. `qa/gemini-quint/x14-chain-id/chain_id_test.qnt` — Self-contained Quint 0.32.0 test suite (`chain_id_test`) covering honest chains (including unequal cross-AIR heights and middle padding), all four required mutants, adversarial tampering, and $\mathbb{G}_1$ doubling/exceptional inputs.
3. `qa/gemini-quint/x14-chain-id/REPORT.md` — Full soundness report, mutant table with exact `quint` commands and expected exit codes, and analysis of padding, field wrap, cross-AIR heights, and exceptional cases.
4. `qa/gemini-quint/x14-chain-id/DONE.md` — Completion summary.

## Verdict
**SOUND and COMPLETE** for non-exceptional affine $\mathbb{G}_1$ addition ($x_1 \not\equiv x_2 \pmod{p_{381}}$) and doubling ($y \not\equiv 0 \pmod{p_{381}}$) at heights $H \le 2^{17} < p_{\text{BabyBear}}$, provided the MSM schedule avoids affine exceptional cases ($\mathcal{O}$, $x_1 = x_2$, $y = 0$), input points are validated, and the consumer AIR binds the output message `id`.
