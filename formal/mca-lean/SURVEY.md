# Survey: Lean 4 libraries for RS codes, list decoding, proximity gaps and MCA

Date: 2026-11-11. Phase 1 of OD-19 (b).

## Result

ArkLib (Verified-zkEVM) is the only library with this material, and it fits. It has a
Johnson-range MCA theorem for RS codes on the affine line that it marks proved: the printed
[BCHKS25, Thm. 4.6] constant, derived from the sharper [DKT26] count (Dao, Kominers, Thaler,
eprint 2026/2056, Nov 2026 revision). This project uses ArkLib as a pinned dependency.

The axiom check of the cone is in "Axiom check" below. Until it is clean, "proved" is ArkLib's
claim only.

## Pins

| Package | Rev | Notes |
|---|---|---|
| ArkLib | `011391e592df184540b4b52edc2c64ee300b00ab` | main on 2026-11-12, "prove the random linear code list-size lower bound (#1471)" |
| Lean | `leanprover/lean4:v4.35.0` | ArkLib's `lean-toolchain` |
| Mathlib | `56a005855ea17e0b0958f73ec801ae140b0a1679` (`v4.35.0`) | transitive, through ArkLib |
| CompPoly | `a1ac8925bde72472724e978d7aa9b94bef3149d9` | transitive |
| VCVio | `fc54dd3d7691d6f5db7dc829e321a727f095bc2d` | transitive (`Pr`, `SampleableType`) |

`lake-manifest.json` matches ArkLib's own manifest on every package.

## What exists

| Library | Content | Status |
|---|---|---|
| Mathlib | Hamming distance, polynomials, `degreeLT`, Lagrange interpolation | no RS codes, list decoding or proximity gaps |
| CompPoly | computable polynomials | no coding theory |
| ArkLib `Data/CodingTheory` | RS codes (`ReedSolomon.code`), Berlekamp-Welch, Guruswami-Sudan, Johnson bound, Polishchuk-Spielman, BCIKS20 proximity gaps (lines, curves, affine spaces, weighted), BCHKS25 Thm 1.3 / 1.9 / Cor 1.7, generators and MCA (`IsMCA`, `mcaError`), DKT26 hidden-derivative interpolation | 2 live `sorry` in all of `ArkLib/`, both tagged external admits, neither used by our theorems (below) |
| other GitHub repos (search "coding theory", language Lean) | small or unrelated projects | not usable |

### ArkLib declarations this project uses

- `CoreDefinitions.IsMCA`, `CoreDefinitions.mcaError`
  (`ArkLib/Data/CodingTheory/ProximityGenerator/Basic.lean:139,191`). `IsMCA` is WHIR Def 4.9
  literally: a set `T` with `|T| ≥ n (1 - δ)`, the combination is in the code projected on `T`,
  and some input is not.
- `CoreDefinitions.AffineLineGenerator F = fun x => ![1, x]`, `univariatePowersGenerator F k =
  fun x i => x ^ i` (`Fin (k + 1)` outputs).
- `ReedSolomon.code domain deg`: `degreeLT F deg` mapped by evaluation (the same as WHIR Def 4.1).
- `CodingTheory.ReedSolomon.rs_mcaError_le_in_johnson_range`
  (`ProximityGap/CapacityBounds.lean:235`): for `1 < k`, `δ < 1 - √((k-1)/n)`,
  `mcaError (AffineLineGenerator F) (code domain k) δ ≤ a / |F|`, where `a` is the printed
  BCHKS25 constant at `ρ = (k-1)/n` and `m = max(⌈√ρ / (1 - √ρ - δ)⌉, 3)`.
- `CodingTheory.ReedSolomon.rs_mcaError_le_johnsonExceptionCount` (line 192): the raw DKT26
  count, which is sharper than the printed constant.
- `ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype`
  (`ProximityGap/BCIKS20/CurveBridge.lean:40`) and `mcaError_powers_le_of_exactPowerAgreement`
  (line 76): the degree `ℓ` curve case, with exceptional set at most
  `2 ℓ · johnsonExceptionCount`.

### Live `sorry` in `ArkLib/` (all of it)

- `ProximityGap/CapacityBounds.lean:341` `exists_rs_epsCa_large_near_capacity` (external admit
  [KKH26], a lower bound near capacity). It is in the same file as our theorem but is not used
  by it.
- `ListDecodability/Bounds/ReedSolomon.lean:729` (external admit [AGL24, Thm 1.1]).
- Other `sorry` matches (`ListDecodability/Bounds.lean:87`, `DG25/MainResults.lean:486,1123`,
  `SubspaceDesign.lean:469`) are comments or docstrings.

No `axiom` declarations in `ArkLib/`. No `native_decide`, `implemented_by` or `extern` in `ArkLib/Data/CodingTheory`.

### Proof route

ArkLib's Johnson-range MCA follows DKT26 (a hidden-derivative interpolant and an "ordinary
equation" exception count, characteristic-free). It does not follow the BCI+20 section 5
Hensel-lift decoder analysis that BCHKS25 section 3.2 and Thm 4.6 build on, or Hab24 Thm 3. For
the Lean proof this does not matter: the kernel checks the proof. For the paper lane it means that
the DKT26 count, not the BCHKS25 sketch, is the proof that is checked.

## Axiom check

`MCA/Axioms.lean` (classic file, so proof bodies load):

(pending the first build)

## Findings for STATEMENT.md

1. **Rate convention.** BCHKS25 Thms 1.5 and 4.x and ArkLib use `ρ = (d-1)/n` (degree bound
   over `n`, "slightly reduced rate"). `calculator.rs` plugs in `ρ = 2^-log_inv_rate = d/n`.
   The bound `a` decreases in `ρ`, so the calculator is optimistic by about
   `1.5 · log2(d/(d-1))` bits in the leading term. This is small, but the instantiation must
   close it, either from slack or from the sharper raw DKT26 count.
2. **Multiplicity convention.** Thm 1.5 (lines, CA) uses `m = ⌈√ρ/(2η)⌉`. Thms 4.2, 4.5 and 4.6
   (curves, CA for subspaces, list/mutual CA) use `m = ⌈√ρ/η⌉`, with `η = 1 - √ρ - γ`.
   `calculator.rs:761` and SWIRL Thm 4.2.4 set `η = √ρ/(2m)`, which is agreement `(7/6)√ρ` at
   `m = 3`. That is the Thm 1.5 convention, applied to Thm 4.5. Under Thm 4.6, `m = 3` needs
   agreement `(4/3)√ρ`, which is the target in the brief. The Lean statements take `γ` as a
   parameter with Thm 4.6's `m`, and specialize to `(4/3)√ρ`.
3. **Curve factor.** The brief and SWIRL (4.7) use the factor `w - 1` (BCHKS25 Thm 4.6, `M`).
   ArkLib's curve theorem gives `2 ℓ · johnsonExceptionCount` (`ℓ = w - 1`). Whether
   `2 · johnsonExceptionCount ≤ bchksA` at the frozen parameters is a phase 2 lemma. A rough
   estimate says that the raw count is several times below the printed constant, so the factor 2
   probably fits.
4. **Which theorem.** SWIRL Thm 4.2.4 batches with BCHKS25 Thm 4.5 (weighted CA for subspaces,
   not MCA). WHIR folding consumes MCA (WHIR Def 4.9) for `Gen(2; α)` on the even and odd parts
   (`MCA.fold_eq_combine`).
