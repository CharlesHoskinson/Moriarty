# astra-1-10: adversarial numeric review of astra-1/PROPOSAL.md

## Question
Audit only: source benchmark attribution, memory geometry and lifetimes, Amdahl arithmetic, Blackwell integer claims. Identify concrete errors with replacement text. No builds.

## Answer
Amdahl arithmetic and the receipt numbers check out. Two substantive problems and three minor ones.

1. **Memory table, N=8n column uses the wrong permutation count (substantive).** The 72-buffer figure uses S=6, which comes from d=5. N=8n needs n(d-1) > 4n, so d >= 6, and the permutation set count S = ceil(18/(d-2)) then changes: d=6 gives S=5, d=9 gives S=3. Peak buffers at N=8n are therefore 66+5=71 (d=6) or 66+3=69 (d=9), not 72.
   - Replacement row: "N=8n: 69-71 extended buffers = 17.25-17.75 GiB; total proxy 27.34-27.84 GiB."
   - Verified arithmetic: 71x256 MiB = 17.75 GiB; 69x256 MiB = 17.25 GiB; 17.75+2.8125+1.28125+0.75+5.25 = 27.84; 17.25+... = 27.34.

2. **N=4n does not pin d=5 (substantive, same root cause).** N = smallest power of two >= n(d-1) gives N=4n for both d=4 (3n -> 4n) and d=5 (4n). d=4 gives S=ceil(18/2)=9, so peak = 66+9 = 75 buffers = 9.375 GiB; total proxy 19.47 GiB (vs 19.09). The draft should state the d range.
   - Replacement sentence: "At N=4n the degree is d in {4,5}: d=5 gives 72 buffers (9.00 GiB, total 19.09 GiB); d=4 gives 75 buffers (9.375 GiB, total 19.47 GiB)."

3. **ICICLE-Halo2 scope (RETRACTED: no change needed).** My first pass said the phrase "expressly excluding witness optimization" was not in the capture. That was a false negative. `astra-1/raw/icicle-halo2-v2.md:31` says: "In this work we didn't focus on optimizing the witness generation." The draft's wording is correct. Replacement is optional: "ICICLE-Halo2 v2 reports up to 25x (capture line 19) on RTX 4080 / i9-13900K. Its line 31 states it did not focus on witness generation."

4. **Line citation (minor).** The 2^19 BLST threshold in registry proofs 0.8.2 is at `midnight-proofs-0.8.2/src/poly/kzg/msm.rs:178` (`coeffs.len() <= (2 << 18)`). Line 176 is the comment. Verified locally. Registry 0.8.3 at lines 176-178 has the always-BLST comment, consistent with the draft.

5. **Blackwell wording (minor, supporting).** Verified in the official table (`helpers/haiku-03/raw/h3b-bp-raw.html`, Table 5) for CC 12.0/12.1: add.s32 128 (same as 8.6/8.9); mad.lo.s32 64 (footnote 6: extended-precision variant 32); extended-precision add 64; shl.b32 64; xor.b32 64; add.s64 32->64; min.s32 64->128. Replacement: "For CC 12.0 the official table keeps add.s32 and mad.lo.s32 unchanged from 8.6/8.9 (128 and 64 per SM per clock). The only doublings are add.s64 and min.s32." The current draft's phrasing is directionally right but does not give the numbers.

6. **cuSnark witness numbers (verified, add source).** The 363.28 s is GPU time: Generate Advice 306.44 + Generate Lookups 56.84 = 363.28 (`astra-1/raw/snarkify-cusnark.md`, wide k20 table). Ceiling 1762.75/363.28 = 4.85x checks. Suggest citing the two rows explicitly, and noting advice was only 393.58->306.44 (1.28x).

## Verified, no change needed
- Receipt: parent-proof.json `phases.prove.duration_seconds` 882.589555, `setup_pk.duration_seconds` 62.328385; sum 944.918.
- Nocy 2.80x (12.3/4.4, 24.6/8.8) matches node04's capture.
- cuSnark hardware (4x RTX 3090, EPYC 7702) is on the page, so the draft's attribution is correct.
- Snarkify 1.13/1.23/28.40 ms match my capture; the draft's benchmark-code caveats (warm-up, timed events, identical generator bases) match `MicroBenchmarks.cu`.
- Amdahl: 10x at s=20, h=0.01 needs f <= 0.04/0.95 = 0.0421053. 10% tail: 1/0.11 = 9.09; 20% tail: 1/0.21 = 4.76; finite s=20 gives 6.45 and 4.00.
- Scenario: R = 0.025+0.039+0.02+0.01 = 0.094 -> 10.638x. Seconds at 882.59: 22.065 + 34.421 + 17.652 + 8.826 = 82.964 (draft 82.963, rounding).
- Witness-only 2x: 5.917x; 5x: 9.174x; threshold 6.452x; bulk and witness both 8x: 6.557x. All reproduce.
- Sequential product: 1.07062 x 3.71747 x 2.86170 = 11.389 (draft 11.3896, rounding).
- Historical chain: 1617.292 s; node 10x gives 1.933x; infinite node 2.156x; serial 2.226 batches/h; 10x interval 161.73 s. Reproduces.

## Raw numbers table
| Item | Value | Source |
|---|---|---|
| parent prove duration | 882.589555 s | `Moriarty-factory/docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json` |
| parent setup_pk duration | 62.328385 s | same |
| cuSnark wide k20 Generate Advice GPU | 306.44 s (CPU 393.58) | `astra-1/raw/snarkify-cusnark.md` |
| cuSnark wide k20 Generate Lookups GPU | 56.84 s (CPU 59.63) | same |
| cuSnark Eval_h | 1115.43 -> 36.19 s (30.82x) | same |
| Total | 1762.75 -> 463.13 s (3.81x) | same |
| 0.8.2 BLST threshold | coeffs.len() <= (2<<18), i.e. <= 2^19 | `midnight-proofs-0.8.2/src/poly/kzg/msm.rs:178` |
| 0.8.3 MSM | always BLST for BLS12-381 G1 | `midnight-proofs-0.8.3/src/poly/kzg/msm.rs:176-178` |
| CC 12.0 add.s32 / mad.lo.s32 | 128 / 64 per SM per clock | `helpers/haiku-03/raw/h3b-bp-raw.html` |

## Caveats
- The d range at N=4n and the d=6 vs d=9 choice at N=8n are assumptions. Fixing d from the Sherlock circuit would resolve items 1 and 2.
- "Registry 0.8.3 always BLST" was checked at a local line only. I did not re-read the full call path.
- Correction to my own caveat: the draft's memory proxy DOES include 5.25 GiB of MSM/NTT scratch and runtime allowances (1.25+2+2). What it omits is bucket storage and per-kernel workspace beyond those allowances.
