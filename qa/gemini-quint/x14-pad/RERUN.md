# Lead rerun of the x14-pad Gemini Quint lane (2026-11-13)

Lane: `argon-run`, gemini-4-argon-high. Lane verdict in `REPORT.md`: SOUND and COMPLETE with two interface conditions (the OUT-bus consumer constrains its receive multiplicity to be boolean and binds `id`; padding chain rows hold a dummy satisfying carry witness with `is_valid = 0`). The lane sandbox refused `quint`, so the lane ran none of its commands. The lead ran them with quint 0.32.0 (default rust backend), each command through `heavy-slot --mem 4G` (runner `lead-run.sh`, logs `lead-runs/NN.log`, summary `lead-runs/summary.txt`). The lead made no change to the model. The lane's seeds `0x14padNN` are not valid hex, so the lead used the fixed seeds `0x14a000` to `0x14a009`.

**Result: the lead rerun confirms the lane verdict.** Every command gives the exit code that the lane's REPORT table expects.

## Commands and counts (MEASURED)

| Command | Expected (REPORT) | Lead result |
|---|---|---|
| `quint typecheck x14_pad.qnt` | 0 | exit 0 |
| `quint typecheck x14_pad_test.qnt` | 0 | exit 0 |
| `quint test --main x14_pad_test --match 'Test$' --seed 0x14a000 x14_pad_test.qnt` | 11 passing | 11 passing, 0 failed |
| `quint test --main x14_pad --match 'Test$' --seed 0x14a000 x14_pad.qnt` (the copies in the model) | not listed | 10 passing, 0 failed |
| `quint run --main x14_pad --invariant invRow` | `[ok]` | `[ok]` |
| `quint run --main x14_pad --invariant invRefinement` | not listed | `[ok]` |
| `quint run --main x14_pad --invariant invNonVacuousPadded` | `[violation]` (witness) | `[violation]`: an accepted partly filled unit is reached |
| `quint run --main m_no_prefix --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_no_prefix --invariant invPerIdSound` | `[ok]` | `[ok]`: the hole breaks the prefix property only |
| `quint run --main m_v_not_bool --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_send_mult_one --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_chain_valid_free --invariant invRow` | `[violation]` | `[violation]` (caught) |
| `quint run --main m_range_mult_v --invariant invRow` | `[violation]` | `[violation]` (caught by the limb range part of `invRow`) |
| `quint run --main m_range_mult_v --invariant invRefinement` | `[ok]` | `[ok]`: an allowed variant for operational soundness |

Every `quint run` uses `--max-samples 1000 --max-steps 5`. Counts: 2 typechecks, 21 tests passing, 3 sound-instance runs as expected, 5 of 5 mutants caught by `invRow`, 2 mutant control runs `[ok]` as the lane states.

## What this confirms and what it does not

The confirmed verdict is the model's: SOUND and COMPLETE under the two interface conditions of the REPORT. The conditions stay open items for the production consumer of the OUT bus (the harness `SinkAir` neither asserts a boolean multiplicity nor binds `id`). `quint verify` was not run.
