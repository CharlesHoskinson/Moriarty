# Lead rerun of the x14-identity Gemini Quint lane (2026-11-13)

Lane: `argon-run`, gemini-4-argon-high. Lane verdict in `REPORT.md`: SOUND and COMPLETE with conditions (the producer binds both the flags and the limbs to the transcript encoding of O; an active `add` row needs `x1 != x2`). The lane sandbox refused `quint`, so the lane ran none of its commands. The lead ran them with quint 0.32.0 (default rust backend), each command through `heavy-slot --mem 4G` (runner `lead-run.sh`, logs `lead-runs/NN.log`, summary `lead-runs/summary.txt`). The lead made no change to the model. The lane gave no `--max-steps`; the lead used 5 and the fixed seeds `0x14b000` to `0x14b007`.

**Result: the lead rerun confirms the lane verdict.** Every command gives the exit code that the lane's REPORT table expects.

## Commands and counts (MEASURED)

| Command | Expected (REPORT) | Lead result |
|---|---|---|
| `quint typecheck x14_identity.qnt` | 0 | exit 0 |
| `quint typecheck x14_identity_test.qnt` | 0 | exit 0 |
| `quint test --main x14_identity_test --match 'Test$' --seed 0x14b000 x14_identity_test.qnt` | 11 passing | 11 passing, 0 failed |
| `quint run --main x14_identity --invariant invRow` | `[ok]` | `[ok]` |
| `quint run --main x14_identity --invariant invNonVacuous` | `[violation]` (witness) | `[violation]`: the accepted honest `add` trace with O + P, P + O, O + O and P1 + P2 is reached |
| `quint run --main m_no_zero_limbs --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_e_ignores_flag --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_sel_wrong --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_out_flag_wrong --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_out_mult_v --invariant invRow` | `[violation]` | `[violation]` (caught) |

Every `quint run` uses `--max-samples 1000 --max-steps 5`. Counts: 2 typechecks, 11 tests passing, the sound invariant `[ok]`, the non-vacuity witness found, 5 of 5 mutants caught.

## What this confirms and what it does not

The confirmed verdict is the model's: SOUND and COMPLETE under the REPORT conditions. The producer that binds the flags and the limbs to the transcript encoding of O is not built yet; it stays an open item of the X14 gate. `quint verify` was not run.
