# Lead repair and rerun of the E5 round 3 decoder model (2026-11-13)

Input: the round 3 lane model `../e5-decode-r3/decode.qnt` and its tests, and the round 3 lead rerun `../e5-decode-r3/RERUN.md`. That rerun found that the toy prime `FR_MOD = 359` is below the 2^9 slot range bound, so the model accepts `rem = -1`. Over BLS12-381 Fr the circuit refuses that value.

## The repair (this folder)

`decode.qnt` is the round 3 model with these changes only:

- `FR_MOD = 2053`. This is a prime above twice every range bound of the model; the largest bound is `SLOT_FIELD_LIM = 512`. A difference `t - poff` or `perms - 1 - m` of two in-range values lies in [-1023, 1022]. A negative value now wraps to at least 1030, which fails the 2^9 check, as `r - k` fails `range(d, 2^9)` over Fr.
- `LIMB_BASE = 16`. 3 limbs now cover 16^3 = 4096 > FR_MOD, so the limb canonicity check against `r - 1` still bites. Also 16 < BB_P = 17. This matches the circuit, where 9 limbs of 30 bits cover 2^270 > r and 2^30 is below the BabyBear prime (`decode.rs:47-48, 216-254`).
- `R1_LIMBS = [4, 0, 8]`, the base-16 limbs of 2052 = FR_MOD - 1.
- `pow_mod` uses 12 exponent bits (exponents up to 2051).
- Comments that named 358 now name 2052.

`decode_test.qnt` is unchanged. `lead_diag_test.qnt` holds the round 3 lead diagnostics and one new diagnostic, `d_freemult_test`.

## Commands and counts (MEASURED, quint 0.32.0, `lead-run.sh`, logs `lead-runs/NN.log`, summary `lead-runs/summary.txt`)

Every command went through `heavy-slot --mem 4G`, with fixed seeds.

| Command | Result |
|---|---|
| `quint typecheck` of `decode.qnt`, `decode_test.qnt`, `lead_diag_test.qnt` | exit 0, all three |
| `quint test --main decode_test --match '_test$' --seed 1 decode_test.qnt` (default rust backend) | `recursion limit exceeded`, all 14 fail as before; every later command uses `--backend typescript` |
| `quint test --backend typescript --main decode_test ... --seed 1` | **13 passing, 1 failed**: `catch_mut_free_mult_test` |
| `quint test --backend typescript --main diag_test ... --seed 1 lead_diag_test.qnt` | 3 passing |
| `quint run --backend typescript --invariant SOUND_AND_COMPLETE --max-samples 20 --max-steps 10`, seeds 1 and 2 | `[ok]` both |
| `quint run ... --invariant SOUND_mut_free_mult --max-samples 20 --max-steps 10 --seed 1` | `[ok]`: **not caught** |
| the same for `SOUND_mut_<m>`, m = `no_m_range`, `no_inactive_zero`, `no_has1`, `no_chain`, `first_cap_wrong`, `ungated_y1` | `[violation]` all 6 (caught) |

Sample count: the typescript backend takes about 30 s per 10-step sample on this model. `probe-10-samples.log` records 10 samples in at most 552 s, including the wait for a slot. At that rate 5000 samples would take about 40 h, so the lead uses 20 samples per run. A first runner at 5000 samples was stopped after about 22 minutes, inside its first `SOUND_AND_COMPLETE` run.

## Findings

1. **The repair removes the round 3 artifact.** Diagnostic `d_sound_test`: `is_sound_on` and `is_complete_on` hold for all 12 corpus cases. Case 6 (`PW_BAD_NO_M_RANGE`) failed in round 3 and holds now. `d_mrange_test` gives `ref_ok = false`, `constraints_ok = false` and `mut_no_m_range = true`: the sound constraints refuse the swapped-slot witness and the mutant accepts it. `SOUND_AND_COMPLETE` holds in the sampled runs.
2. **The round 3 counterexample for `mut_free_mult` was the same artifact.** `WIT_BAD_FREE_MULT` puts the PATH row (poff 7) at slot 6, so `t - poff = -1`. Diagnostic `d_freemult_test` gives `ref_ok = false`, `constraints_ok = false` and `mut_free_mult = false`. With the flags `free_mult: true, check_m_range: false` together it gives true. The m-range check alone refuses this witness, so the round 3 claim "7 mutants caught" is not confirmed: `catch_mut_free_mult_test` fails and `SOUND_mut_free_mult` is not violated.
3. **`mut_free_mult` may be an equivalent mutant (DERIVED, not measured).** With the m-range check on, a slot t can hold only a tuple whose `(poff, perms)` satisfies `poff <= t < poff + perms`. The slot-to-entry lookup forces the slot tuple to equal some entry row. The entry intervals `[poff_i, poff_i + perms_i)` partition `[0, np)`. So each active slot's tuple is forced to the row of the one entry that covers it, whatever the multiplicities are. The lookup balance then forces the multiplicities to the slot counts. If this holds, the fixed multiplicity of `decode.rs:736-739` adds defence in depth and closes no attack of its own. Round 4 is asked to settle it with a counterexample or a tested argument.

## Status

The repair changes what the model asserts: one of the 7 mutant counterexamples is invalid. A Gemini Quint lane, round 4, re-checks the repaired model (`argon-run`, `~/.cache/gemini-lanes/e5-decode-r4`, prompt `~/.cache/gemini-lanes/e5-decode-r4-prompt.md`). Until the lead reruns that lane, the E5 decoder verdict rests on this rerun: SOUND and COMPLETE on the sampled and corpus checks, 6 of 7 mutants caught, and `mut_free_mult` not caught. The circuit is unchanged.
