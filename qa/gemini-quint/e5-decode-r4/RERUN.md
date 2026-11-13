# Lead rerun of the E5 round 4 Gemini Quint lane (2026-11-13)

The lane ran through `argon-run` with gemini-4-argon-high (unit `gq-e5-decode-r4`, under `heavy-slot`). Its workspace was `~/.cache/gemini-lanes/e5-decode-r4`, its prompt is `PROMPT.md` (copy of `~/.cache/gemini-lanes/e5-decode-r4-prompt.md`) and its stream log is `lane.log`. The lane started from the lead repair `../e5-decode-r3-lead/` (toy prime 2053, limb base 16). Its verdict in `REPORT.md`: all 7 questions SOUND. `mut_free_mult` is an equivalent mutant while the m-range check is kept, and the other 6 mutants are caught. The lane audits each range check of `decode.rs` against the toy field (REPORT section 1).

The lane sandbox refused `quint` (REPORT section 4), so the lane ran no command. Its `quint` results below are the lead's. The lane changed the model only by adding the helpers `slot_entry_m_range_unique`, `mk_attack_slot` and `free_mult_attack_wit` and by updating comments. In `catch_mut_free_mult_test` it replaced the round 3 assertion with four new ones: the round 3 witness is refused by `mut_free_mult`; it passes once `check_m_range` is also off; `check_m_range` pins each active slot to the one entry that covers it, over all 12 corpus cases; and `mut_free_mult` refuses all 64 assignments `(i6, i7)` of entry rows to slots 6 and 7 on `PW_BAD_FREE_MULT`.

## Lead control (`lead_control_test.qnt`)

An exhaustive check over 64 attack witnesses means something only if the attack generator can build an accepted witness when the m-range check is off. The lead's control evaluates `free_mult_attack_wit(i6, i7)` under the flags `free_mult: true, check_m_range: false`. It accepts 9 of the 64 pairs: (3, 3), (3, 6), (3, 7), (6, 3), (6, 6), (6, 7), (7, 3), (7, 6) and (7, 7). Under `mut_free_mult` alone, with the m-range check on, the lane test shows that all 64 pairs are refused. For (6, 6) the control gives `(true, false)`. So the attack domain is not vacuous, and the m-range check alone refuses these attacks.

## Commands and counts (MEASURED, quint 0.32.0, `lead-run.sh`, logs `lead-runs/NN.log`, summary `lead-runs/summary.txt`)

Every command went through `heavy-slot --mem 4G` with `--backend typescript` and fixed seeds. The rust backend hits its recursion limit on this model (see `../e5-decode-r3-lead/RERUN.md`).

| Command | Expected (REPORT) | Lead result |
|---|---|---|
| `quint typecheck decode.qnt`, `decode_test.qnt`, `lead_control_test.qnt` | 0 | exit 0, all three |
| `quint test --main decode_test --match '_test$' --seed 1 decode_test.qnt` | 14 passing | **14 passing, 0 failed** |
| `quint test --main control_test --match '_test$' --seed 1 lead_control_test.qnt` | (lead) | 2 passing (the control above) |
| `quint run --invariant SOUND_AND_COMPLETE --max-samples 20 --max-steps 10`, seeds 1 and 2 | `[ok]` | `[ok]` both |
| `quint run --invariant SOUND_mut_free_mult ... --seed 1` | `[ok]` (equivalent) | `[ok]`: not violated, as the lane states |
| `quint run --invariant SOUND_mut_<m> ... --seed 1`, m = `no_m_range`, `no_inactive_zero`, `no_has1`, `no_chain`, `first_cap_wrong`, `ungated_y1` | `[violation]` | `[violation]` all 6 (caught) |

The lane's table asks for 5000 samples per run. At about 30 s per sample (`../e5-decode-r3-lead/probe-10-samples.log`) that would take about 40 h per run, so the lead used 20 samples. The sampled runs add little beyond the corpus: the deterministic tests evaluate every corpus case and every mutant counterexample directly.

## Verdict

**The lead rerun confirms the round 4 lane verdict.** Every command gives the exit code that the REPORT expects. The model is SOUND and COMPLETE on its corpus and in the sampled runs. 6 of 7 mutants are caught. `mut_free_mult` is an equivalent mutant: with the m-range check, the slot-to-entry lookup and the partition of `[0, np)` by the entry intervals, each active slot is pinned to its entry whatever the multiplicities are. The fixed multiplicity of `decode.rs:736-741` is therefore defence in depth. It closes no attack that the m-range check leaves open.

Limits of this verdict:

- The equivalence rests on the lane's argument (REPORT section 3). The exhaustive check covers only slots 6 and 7 of one public input, with the other 6 slots honest. It is not a check over all witnesses.
- Each sampled run used 20 samples.
- `quint verify` was not run.
- The circuit is unchanged. Native I7 equals stock on 30 of 30 cases, and `MockProver` refuses the 3 `open_replay` cases (`r3-i9.log`).
