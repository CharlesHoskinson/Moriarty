/-
OD-19 (b): bridges between `MCA/Defs.lean` and ArkLib (Verified-zkEVM/ArkLib at
011391e592df184540b4b52edc2c64ee300b00ab). Phase 1 leaves the proofs as `sorry`.

ArkLib proves (sorry-free, see SURVEY.md) a Johnson-range MCA bound for the affine line
(`CodingTheory.rs_mcaError_le_in_johnson_range`, the printed [BCHKS25, Thm. 4.6]
constant) and an exceptional-set bound for polynomial curves
(`ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype`, factor `2 ℓ`). Phase 2
proves targets (i) and (ii) from them through these bridges.
-/
import MCA.Defs
import ArkLib.Data.CodingTheory.ProximityGap.CapacityBounds
import ArkLib.Data.CodingTheory.ProximityGap.BCIKS20.CurveBridge

namespace MCA

variable {F : Type} [Field F] [Fintype F] [DecidableEq F]
variable {ι : Type} [Fintype ι] [DecidableEq ι]

/-- Our RS code is ArkLib's `ReedSolomon.code` (same construction: `degreeLT` mapped by
evaluation). -/
theorem rsCode_eq_arklib (dom : ι ↪ F) (d : ℕ) : rsCode dom d = ReedSolomon.code dom d := by
  sorry

/-- `powGen F (k + 1)` is ArkLib's `univariatePowersGenerator F k`. -/
theorem powGen_eq_univariatePowers (k : ℕ) :
    (powGen F (k + 1) : F → Fin (k + 1) → F) = CoreDefinitions.univariatePowersGenerator F k := by
  sorry

/-- `foldGen F` is ArkLib's `AffineLineGenerator F`. -/
theorem foldGen_eq_affineLine :
    (foldGen F : F → Fin 2 → F) = CoreDefinitions.AffineLineGenerator F := by
  sorry

/-- The bad event of [WHIR, Def. 4.9] (`McaBad`) is ArkLib's `IsMCA`. -/
theorem mcaBad_iff_isMCA {R : Type} [Nonempty R] [Fintype R] {ℓ : ℕ}
    (C : Submodule F (ι → F)) (G : Generator R ℓ F) (f : Fin ℓ → ι → F) (δ : ℝ) (r : R) :
    McaBad (C : Set (ι → F)) G f δ r ↔ CoreDefinitions.IsMCA G C r f δ := by
  sorry

/-- [WHIR, Def. 4.9] with error `err` is ArkLib's `mcaError ≤ err` for a uniform seed. -/
theorem hasMCA_iff_mcaError_le {R : Type} [Nonempty R] [Fintype R] [SampleableType R] {ℓ : ℕ}
    (C : Submodule F (ι → F)) (G : Generator R ℓ F) (δ err : ℝ) (herr : 0 ≤ err) :
    HasMCA (C : Set (ι → F)) G δ err ↔
      CoreDefinitions.mcaError G C δ ≤ ENNReal.ofReal err := by
  sorry

end MCA
