# mca-lean: a Lean proof of the Reed-Solomon MCA bound that the Sherlock calculator uses

Owner decision OD-19 option (b), 2026-11-11.

## Target

Prove in Lean 4 the mutual correlated agreement (MCA) bound for Reed-Solomon codes beyond the
unique decoding radius that the soundness calculator uses
(`repos/stark/openvm-org_stark-backend/crates/stark-backend/src/soundness/calculator.rs:534`,
`whir_proximity_gap_security` and `log2_a_bound_bchks25`). With this proof, the internal and `L`
layers of the Sherlock design meet "proven 100 bits, no conjecture".

The calculator bound is the BCHKS25 linear-in-`n` bound:

- error `d * ceil(a) / q`, with `a = 2 D_X D_Y^2 D_Z + (gamma n + 1) D_Y`;
- `D_X = (m + 1/2) n sqrt(rho)`, `D_Y = (m + 1/2) / sqrt(rho)`, `D_Z = (m + 1/2)^2 / (3 rho)`;
- `m = 3`, agreement `(4/3) sqrt(rho)` (that is, `gamma = 1 - (4/3) sqrt(rho)`);
- `d = 1` for folding (the generator `Gen(2; alpha) = (1, alpha)`);
- `d = w - 1` for batching (the generator `Gen(w; mu) = (1, mu, ..., mu^(w-1))`, a degree `w - 1`
  curve).

The MCA event is WHIR (eprint 2024/1586) Definition 4.9. The paper proof is in
`research/sherlock-mca-proof-20261111/` (STATEMENT.md, AUDIT.md, PROOF.md). The Lean statements
follow STATEMENT.md. Where this project and STATEMENT.md differ, STATEMENT.md wins and this project
changes.

## Definition of done

1. 0 `sorry` in the project.
2. `#print axioms` on each main theorem shows only `propext`, `Classical.choice` and `Quot.sound`.
3. The hard lemma (the Johnson-range MCA exception bound) is never a hypothesis, a structure field
   or a premise of a main theorem. It is proved.
4. An instantiation theorem at the frozen `L` and internal parameters proves the bit bound that the
   calculator uses.

Note (coordinator, 2026-11-11): OD-19 now runs unique decoding in production. This proof is for
bringing list decoding back later (k = 19). The definition of done is unchanged.

## Status (phase 2, route (a))

Route (a) is the proof. **PROOF.md Theorem 7.1 is not formalized. ArkLib's proved Johnson curve
count replaces it.** ArkLib's
`ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype` is sorry-free and uses only the
standard axioms. It bounds the exceptional set of a degree `M` power curve by
`2 M · johnsonExceptionCount`. At each frozen row, this count (plus the subcode term `M · L` for
Corollary 8.2) is at most PROOF.md's `a*_M`. A closed integer check (`rowOk`, `codeOk`,
`decide +kernel`) proves this. So the Lean theorems prove the PROOF.md bits through ArkLib's count.
(H1) to (H6) (Lemmas 9.0 and 9.0b) are still proved as integer facts, but no theorem consumes them.

The "proved in Lean" column is the bit level that `row_bits` / `row_subcode_bits` prove: the bound
is `a*_M(g)`, with `a*_M · 2^bits ≤ p^4`. The "ArkLib route count" column is
`log2(p^4 / (2M·count + M·L))`: the bit level that ArkLib's count alone would give, if the theorem
were stated with the count instead of `a*_M`. Rows with `M = 1` are folds. The two others are the
`L` and internal mu rows.

| mu | beta | M | n | g | PROOF.md a*_M bits (proved in Lean) | ArkLib route count bits |
|---|---|---|---|---|---|---|
| 19 | 4 | 1 | 8388608 | 5592405 | 88 | 89.98 |
| 18 | 4 | 1 | 4194304 | 2796202 | 89 | 90.98 |
| 17 | 4 | 1 | 2097152 | 1398101 | 90 | 91.98 |
| 16 | 4 | 1 | 1048576 | 699050 | 91 | 92.98 |
| 15 | 7 | 1 | 4194304 | 3700000 | 87 | 88.13 |
| 14 | 7 | 1 | 2097152 | 1850000 | 88 | 89.13 |
| 13 | 7 | 1 | 1048576 | 925000 | 89 | 90.13 |
| 12 | 7 | 1 | 524288 | 462500 | 90 | 91.13 |
| 11 | 10 | 1 | 2097152 | 2009770 | 84 | 86.17 |
| 10 | 10 | 1 | 1048576 | 1004885 | 85 | 87.17 |
| 9 | 10 | 1 | 524288 | 502442 | 86 | 88.17 |
| 8 | 10 | 1 | 262144 | 251221 | 87 | 89.17 |
| 18 | 3 | 1 | 2097152 | 1108545 | 92 | 93.09 |
| 17 | 3 | 1 | 1048576 | 554272 | 93 | 94.09 |
| 16 | 3 | 1 | 524288 | 277136 | 94 | 95.09 |
| 15 | 3 | 1 | 262144 | 138568 | 95 | 96.09 |
| 14 | 6 | 1 | 1048576 | 873813 | 89 | 91.07 |
| 13 | 6 | 1 | 524288 | 436906 | 90 | 92.07 |
| 12 | 6 | 1 | 262144 | 218453 | 91 | 93.08 |
| 11 | 6 | 1 | 131072 | 109226 | 92 | 94.08 |
| 10 | 9 | 1 | 524288 | 493394 | 87 | 89.16 |
| 9 | 9 | 1 | 262144 | 246697 | 88 | 90.16 |
| 8 | 9 | 1 | 131072 | 123348 | 89 | 91.16 |
| 7 | 9 | 1 | 65536 | 61674 | 90 | 92.16 |
| 20 | 4 | 47 | 16777216 | 11184810 | 82 | 83.43 |
| 19 | 3 | 511 | 4194304 | 2217090 | 82 | 83.09 |

## Layout

- `MCA/Defs.lean`: definitions written from the papers over Mathlib only (RS code on an evaluation
  domain, agreement, MCA as WHIR Def 4.9, the fold and power-curve generators, `mcaErr`, `HasMCA`).
- `MCA/Bridge.lean`: the definitions are ArkLib's (`ReedSolomon.code`, `univariatePowersGenerator`,
  `AffineLineGenerator`, `IsMCA`, `mcaError`).
- `MCA/Johnson.lean`: ArkLib's `johnsonExceptionCount` at an integer threshold `A`, as the natural
  number `countNum / (A - k)`, under the integer conditions `JOk`.
- `MCA/Route.lean`: `curve_card_le` (full code, any curve degree `M`; `M = 1` is the fold),
  `vandermonde_mem`, and `cor_8_2` (affine subcodes, PROOF.md Corollary 8.2).
- `MCA/Proof.lean`: `aStar`, (H1) to (H6) (`Hyps`), Lemma 9.0 (`lemma_9_0`, 26 rows), Lemma 9.0b
  (`lemma_9_0b`, 30 codes, all `g ≤ g_end`), Definition 9.1b (`errStar`) and Corollary 9.2
  (`cor_9_2`, fold MCA with error `errStar` at every radius), with the row and code data.
- `MCA/Instantiation.lean`: `row_bits`, `row_mcaError_bits` (ArkLib `mcaError` form) and
  `row_subcode_bits` at the 26 frozen rows over `|F| = p^4`.
- `MCA/Axioms.lean`: `#print axioms` checks. It is a classic file (no `module` header), because a
  module importer does not load proof bodies.
- `SURVEY.md`: the libraries that exist, with pinned commits, versions, axioms and sorry status.

The phase 1 file `MCA/Statements.lean` (the printed BCHKS25 Thm 4.6 form with factor `w - 1`) has
been deleted. ArkLib's curve bound has the factor `2 M`, which does not prove that form. The frozen
rows do not need it.

## Build

Builds run in `heavy.slice`, under the shared lock, with logs in `~/.cache/mca-lean/`:

```
systemd-run --user --unit=mca-lean-build-N --collect \
  -p WorkingDirectory=$PWD \
  flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run lake build
```

Each Lean command gets at most 25 minutes, except `lake exe cache get`. Lake resumes an
interrupted build, so a timed-out build continues with the next unit number.
