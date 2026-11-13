# Lead rerun of the E5 round 3 Gemini Quint lane (2026-11-13)

Lane: `argon-run`, gemini-4-argon-high, prompt `PROMPT.md`. Lane verdict in `REPORT.md`: SOUND and COMPLETE on all 7 questions; the only caveat is the deliberate `pw[0] == sh.pre_hash` pin. The lane wrote `decode.qnt` and `decode_test.qnt`. The lead ran them with quint 0.32.0, each command under `heavy-run --mem 4G` (runner `lead-run.sh`, logs `typecheck.log`, `lead-runs.log`, `lead-runs-rust.log`). The lead made no change to the model.

**Result: the lead rerun does not confirm the lane verdict.** One counterexample of the model passes the model's own constraints. The cause is a model artifact, not a circuit defect (see below). The model needs a repair before its runs count as evidence.

## Commands and counts (MEASURED)

| Command | Result |
|---|---|
| `quint typecheck decode.qnt` | exit 0 |
| `quint typecheck decode_test.qnt` | exit 0 |
| `quint test --main decode_test --match '_test$' --seed 1 decode_test.qnt` (default rust backend) | the evaluator stops with `recursion limit exceeded` on the tests that use the corpus, and on every `quint run` (`lead-runs-rust.log`). All later runs use `--backend typescript` |
| `quint test --backend typescript ... --seed 1` | 12 passing, 2 failed: `corpus_sound_and_complete_test` and `catch_mut_no_m_range_test` |
| `quint run --backend typescript --invariant SOUND_AND_COMPLETE --max-samples 5000 --max-steps 10 --seed 1` and `--seed 2` | `[violation]` both, at State 0 (an initial corpus state) |
| `quint run ... --invariant SOUND_mut_<m> --seed 1` for the 7 mutants `free_mult`, `no_m_range`, `no_inactive_zero`, `no_has1`, `no_chain`, `first_cap_wrong`, `ungated_y1` | `[violation]` all 7 |

## Diagnosis (MEASURED with `lead_diag_test.qnt`, typescript backend)

- Per corpus case, `is_complete_on` holds for all 12 cases, and `is_sound_on` holds for 11. It fails for case 6 only: `PW_BAD_NO_M_RANGE` with `WIT_BAD_NO_M_RANGE`, the counterexample written for the mutant `mut_no_m_range`.
- For that case: `ref_decode(...).ok` is false, `constraints_ok(...)` is **true**, and `mut_no_m_range(...)` is true. So the sound model accepts a witness that stock refuses. This is the failed assertion of `catch_mut_no_m_range_test`.
- Cause: the model works over the toy field `FR_MOD = 359` (`decode.qnt:14`), and `in_bound(x, lim)` is `x >= 0 and x < FR_MOD and x < lim` with `SLOT_FIELD_LIM = 512` (`decode.qnt:74, 203`). Since 359 < 512, the 2^9 range checks on `t - poff` and `perms - 1 - (t - poff)` accept every field element. The counterexample swaps two slots, so `rem = -1 = 358`, which passes the model's range check. Over BLS12-381 Fr, -1 is `r - 1`, far above 2^9, and the circuit's `range(d2, 2^9)` refuses it (the lane's own Question 1 argument).
- The `SOUND_AND_COMPLETE` violations start at State 0, whose state is a corpus entry, and case 6 is the only corpus entry that fails. So they are the same artifact. For the same reason, the 7 sampled mutant violations do not discriminate: the sound instance is violated on the same corpus.

## What the model needs

A toy modulus above every range bound that the model checks (at least 2^10 here, so that `t - poff` and `perms - 1 - m` cannot wrap below 2^9), or range checks that model the wrap of a negative value at the real field size. Then rerun this table. Until then, the lane verdict SOUND and COMPLETE rests on the lane's written argument only. The circuit itself is unchanged by this finding: native I7 equals stock on 30 of 30 cases, and `MockProver` refuses the 3 `open_replay` cases (`r3-i9.log`).
