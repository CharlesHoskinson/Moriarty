/-
OD-19 (b): bridges between `MCA/Defs.lean` and ArkLib (Verified-zkEVM/ArkLib at
011391e592df184540b4b52edc2c64ee300b00ab).

The definitions of `MCA/Defs.lean` follow the papers literally. These theorems show that they are
ArkLib's: the Reed-Solomon code, the power and line generators, the MCA bad event `IsMCA` and the
MCA error `mcaError` for a uniform seed. So the bounds of `MCA/Route.lean` are bounds on ArkLib's
`mcaError` too (`mcaError_le_of_card_le`).
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
  ext f
  rw [ReedSolomon.mem_code_iff_eval]
  simp only [rsCode, Submodule.mem_map, Polynomial.mem_degreeLT, funext_iff, evalOn_apply]

/-- `powGen F (k + 1)` is ArkLib's `univariatePowersGenerator F k`. -/
theorem powGen_eq_univariatePowers (k : ℕ) :
    (powGen F (k + 1) : F → Fin (k + 1) → F) = CoreDefinitions.univariatePowersGenerator F k :=
  rfl

/-- `foldGen F` is ArkLib's `AffineLineGenerator F`. -/
theorem foldGen_eq_affineLine :
    (foldGen F : F → Fin 2 → F) = CoreDefinitions.AffineLineGenerator F :=
  CoreDefinitions.univariatePowersGenerator_one_eq_affineLineGenerator

/-- Membership of a projected word in ArkLib's projected code is `AgreesOn`. -/
theorem proj_mem_iff (C : Submodule F (ι → F)) (w : ι → F) (T : Finset ι) :
    LinearCode.projectedWord w T ∈ LinearCode.projectedCodeSubmod C T ↔
      AgreesOn (C : Set (ι → F)) w T := by
  rw [LinearCode.mem_projectedCodeSubmod_iff]
  constructor
  · rintro ⟨c, hc, heq⟩
    exact ⟨c, hc, fun x hx => (congrFun heq ⟨x, hx⟩).symm⟩
  · rintro ⟨c, hc, hcw⟩
    exact ⟨c, hc, funext fun i => (hcw i i.2).symm⟩

/-- The bad event of [WHIR, Def. 4.9] (`McaBad`) is ArkLib's `IsMCA`. -/
theorem mcaBad_iff_isMCA {R : Type} [Nonempty R] [Fintype R] {ℓ : ℕ}
    (C : Submodule F (ι → F)) (G : Generator R ℓ F) (f : Fin ℓ → ι → F) (δ : ℝ) (r : R) :
    McaBad (C : Set (ι → F)) G f δ r ↔ CoreDefinitions.IsMCA G C r f δ := by
  dsimp only [McaBad, CoreDefinitions.IsMCA, LargeSet]
  constructor
  · rintro ⟨S, hS, hagree, i, hi⟩
    exact ⟨S, by linarith, (proj_mem_iff C (combine G r f) S).mpr hagree, i,
      fun hm => hi ((proj_mem_iff C _ S).mp hm)⟩
  · rintro ⟨S, hS, hm, i, hi⟩
    exact ⟨S, by linarith, (proj_mem_iff C (combine G r f) S).mp hm, i,
      fun ha => hi ((proj_mem_iff C _ S).mpr ha)⟩

/-- [WHIR, Def. 4.9] with error `err` is ArkLib's `mcaError ≤ err` for a uniform seed. -/
theorem hasMCA_iff_mcaError_le {R : Type} [Nonempty R] [Fintype R] [SampleableType R] {ℓ : ℕ}
    (C : Submodule F (ι → F)) (G : Generator R ℓ F) (δ err : ℝ) (herr : 0 ≤ err) :
    HasMCA (C : Set (ι → F)) G δ err ↔
      CoreDefinitions.mcaError G C δ ≤ ENNReal.ofReal err := by
  classical
  unfold HasMCA CoreDefinitions.mcaError
  rw [iSup_le_iff]
  refine forall_congr' fun f => ?_
  rw [SampleableType.prEvent_uniformSample_eq_ofReal, ENNReal.ofReal_le_ofReal_iff herr]
  have hcard : (mcaBadSet (C : Set (ι → F)) G f δ).card =
      (Finset.univ.filter fun x => CoreDefinitions.IsMCA G C x f δ).card := by
    apply Finset.card_bij (fun r _ => r)
    · intro r hr
      simp only [mcaBadSet, Finset.mem_filter, Finset.mem_univ, true_and] at hr ⊢
      exact (mcaBad_iff_isMCA C G f δ r).mp hr
    · intro _ _ _ _ h
      exact h
    · intro r hr
      refine ⟨r, ?_, rfl⟩
      simp only [mcaBadSet, Finset.mem_filter, Finset.mem_univ, true_and] at hr ⊢
      exact (mcaBad_iff_isMCA C G f δ r).mpr hr
  unfold mcaErr
  rw [hcard]

end MCA
