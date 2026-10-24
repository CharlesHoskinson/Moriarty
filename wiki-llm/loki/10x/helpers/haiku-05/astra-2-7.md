# astra-2-7: audit of actual Garaga ECIP code, and corrections to astra-2-4

> SUPERSEDED IN PART by `astra-2-10.md`. The verifier loops over basis points, not digits: two divisions per point, witness `4n+10`. The "~107 divisions per scalar" count and the "~124-row break-even per division" below are withdrawn. The Fiat-Shamir, hash-to-curve, and source-path findings still stand.

## Question

Identify, from Garaga's primary source, what the implemented ECIP certificate computes: the hash-to-curve and coefficient derivation, the remainder/verification checks, and any BLS12-381 field-operation counts. Correct astra-2-4 accordingly. Read-only; no repo modifications.

## Answer

- **Correction to astra-2 path.** `hydra/garaga/precompiled_circuits/ecip.py` does not exist in the repo tree. The actual files are:
  - Python verifier and hint: `hydra/garaga/hints/ecip.py` (captured as `raw/garaga-hydra_garaga_hints_ecip.py.txt`);
  - Rust hint generator: `tools/garaga_rs/src/ecip/core.rs` (`raw/garaga-tools_garaga_rs_src_ecip_core.rs.txt`), with `ff.rs` alongside;
  - Cairo: `src/src/ec/ec_ops.cairo` (MSM only, no ECIP verification; `raw/garaga-src_src_ec_ec_ops.cairo.txt`) and `src/src/universal_ecip.cairo`, which only dispatches to `msm_g1` (`raw/garaga-src_src_universal_ecip.cairo.txt`).
  - Tree source: `raw/garaga-tree.txt` (GitHub API, `main`, recursive).
- **The certificate is a rational-function divisor, not a Bulletproofs argument.** Prover (`core.rs`, `run_ecip`, `ecip_functions`, `row_function`, `construct_function`): each scalar is expanded into base −3 digits in {−1, 0, 1} (`neg_3_base_le`); one divisor per digit row is built as a balanced product tree of line functions (`construct_function`, `line`, `mul_line_line_p_p_neg`); each divisor's logarithmic derivative is taken (`dlog`); and `sum_dlog = Σ_i (−3)^i · dlog(D_i)`. The output is `Q` (2 coordinates) and four coefficient lists `a_num, a_den, b_num, b_den` (`prepare_result`).
- **The verification equation is per-term, not constant-size.** `verify_ecip` (`hints/ecip.py` ~147-212): pick `A0`, set `A2 = −2·A0`, compute `coeff0, coeff2` from the tangent/line at `A0`, then check
  `coeff0·sum_dlog(A0) − coeff2·sum_dlog(A2) = eval_point_challenge(−Q) + Σ_P [ep·(xA0−xP)/(yP − mA0·xP − bA0) + en·(xA0−xP')/(yP' − …)]`
  with `P'=−P`. Each term costs one field division (`eval_point_challenge`, `ep`/`en` the positive/negative digit multiplicities from `positive_negative_multiplicities(neg_3_base_le(s))`). So the verifier cost is one division per nonzero base −3 digit per scalar, not a constant number of multiplications.
- **Number of verifier terms (derivation, not run).** A 255-bit scalar has about 255/log2(3) ≈ 161 base −3 digit positions. For random digits in {−1,0,1}, about 2/3 are nonzero, so ≈ 107 division terms per full-width scalar. This is arithmetic from the code's digit rule, not a measurement.
- **Hash-to-curve (`derive_ec_point_from_X`, `hints/ecip.py:38-102`).** Try-and-increment with Starknet Poseidon (`hades_permutation`, over the Stark field): `x ← poseidon(x, attempt)` until `x³+ax+b` is a quadratic residue; the square roots of `g·rhs` are stored so Cairo can verify existence. The docstring says the Cairo version is in `ec_ops.cairo`, but the captured `ec_ops.cairo` contains no `derive_ec_point_from_X` or ECIP code (grep found only `hades_permutation` in the helper at lines 72-75). The Cairo derive is **not located**.
- **Fiat-Shamir binding is not implemented in the captured Python verifier.** The comment at `hints/ecip.py:175-176` says to hash `(Bs, scalars)` and `(Q, sum_dlog)` into `A0`, but the code path at 177-178 samples `A0 = gen_random_point()` when `A0` is `None`. So the default Python verify is interactive-style random, not Fiat-Shamir. The Cairo binding of `A0` to the transcript was not found. This is the main unreviewed soundness gap.
- **Remainder and degree checks.** The verifier calls `assert sum_dlog.validate_degrees(len(Bs))` (line 167) and `assert LHS == RHS` (line ~206). The Python also computes `assert Q == msm(Bs, scalars)` but marks it "Sanity check. Not part of the verification" (~line 208-210), so the equation alone must enforce `Q`. The degree-validation code was not read in full, and no Cairo counterpart was found.
- **Mapping to Midnight.** The challenge Poseidon is over the Stark field (Starknet), so for Midnight it would be a native Poseidon over the BLS scalar field `Fq`, and the hash output must then be mapped into the curve base field `Fp` (foreign). That mapping is not in Garaga and would need its own soundness argument (bias/reduction).
- **BLS12-381 operation counts.** None found in the captured source or README. The README's MSM benchmark table covers SECP256K1/R1 only (`raw/ecip-garaga-readme.md` lines 457-466); BLS MSM is absent. The BLS entries are pairings/Groth16 in Cairo steps (lines 366, 473-501), not PLONK rows, and are not convertible.
- **Threshold, derived from stock fixtures (estimate).** Stock full-width term cost is ≤ 13.3k rows per term (`msm_3` 39,935 / 3, an upper bound that includes setup). With ≈107 divisions per full-width scalar, the certificate beats the stock per-term cost only if one foreign-field division costs less than about 13.3k / 107 ≈ 124 rows, including its share of the `Q` and coefficient-list handling. For reference only, the stock `field_chip_k256_base/div` is 28 rows (`cost-model.json`), but that is a 256-bit field, not BLS `Fp` (381 bits), so it is a lower-order reference, not a bound. The BLS `Fp` division row cost is not in the fixtures and must be measured.

## Corrections to astra-2-4

Astra-2-4 (`helpers/haiku-05/astra-2-4.md`) is superseded in these respects:

1. **Remove the 765-element committed-witness estimate.** It conflated the base-field size (381 bits) with the scalar size (255 bits) and the polynomial degree, which depends on the algorithm and MSM size. Garaga's coefficient-list length was not measured (it scales with the number of nonzero digits and points per row). Do not use it.
2. **Remove the "7 multiplicative constraints" as a count for the Garaga certificate.** The 7-constraint figure applies only to the Bulletproofs/Parker instantiation in zkSecurity Construction 5 (Fig. 5). Garaga's verifier cost is one division per nonzero base −3 digit (≈107 per full-width scalar, derived).
3. **Do not claim the Bulletproofs route is required.** Astra-2's point is right: the Garaga verification is non-interactive application code, so it does not inherently need Bulletproofs. The challenge can be Poseidon-derived (as Garaga does) and could be hashed inside a stock Poseidon circuit. But that Fiat-Shamir binding is unimplemented in the captured code and is unreviewed, so "not impossible" is the honest statement, not "feasible".
4. **The 75% reduction remains unevaluated.** The per-term threshold above (about 124 rows per division) is the number to measure, along with the Fiat-Shamir and hash-to-curve cost in Midnight's Poseidon and the cost of the degree and subgroup checks.

## Raw numbers

| Quantity | Value | Source |
|---|---:|---|
| Base −3 digit positions, 255-bit scalar | ≈161 (estimate) | `neg_3_base_le`; arithmetic |
| Nonzero digits, 255-bit scalar (random) | ≈107 (estimate, 2/3 of digits) | derived from digit rule |
| Verifier divisions per full-width term | ≈107 (one per nonzero digit) | `hints/ecip.py:147-212`, `eval_point_challenge` |
| Garaga Python ECIP verify default A0 | random (`gen_random_point`) | `hints/ecip.py:177-178` |
| Hash-to-curve | Poseidon252 try-and-increment | `hints/ecip.py:38-102` |
| Stock msm_3 per term (upper bound) | ≈13.3k rows | `cost-model.json`; `ecc.rs:555-600` |
| Stock k256 base div | 28 rows (reference only; not BLS) | `cost-model.json` |
| Break-even division cost | ≈124 rows (estimate) | 13.3k / 107 |
| BLS MSM counts in Garaga source/README | none found | `raw/ecip-garaga-readme.md` |

## Caveats

- No Garaga code was built or run; all counts are from reading source and arithmetic.
- The Cairo-side `derive_ec_point_from_X` and the ECIP verifier were not located in the captured Cairo files; a fuller tree walk could find them, but this capture is limited to `src/src/ec/ec_ops.cairo` and `src/src/universal_ecip.cairo`.
- The Garaga README is a cached snapshot and is partial.
- The 107-division figure assumes uniformly random scalar digits; a real scalar distribution changes it.

## Files

- Captures: `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/helpers/haiku-05/raw/garaga-hydra_garaga_hints_ecip.py.txt`, `garaga-tools_garaga_rs_src_ecip_core.rs.txt`, `garaga-tools_garaga_rs_src_ecip_ff.rs.txt`, `garaga-src_src_ec_ec_ops.cairo.txt`, `garaga-src_src_universal_ecip.cairo.txt`, `garaga-tree.txt`
- Superseded: `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/helpers/haiku-05/astra-2-4.md`
