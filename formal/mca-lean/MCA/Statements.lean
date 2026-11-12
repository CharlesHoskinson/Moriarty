/-
OD-19 (b): target statements. Phase 1 leaves the proofs as `sorry`.

The statements follow [BCHKS25, Thm. 4.6] with the MCA event of [WHIR, Def. 4.9]. They are
pending a check against `research/sherlock-mca-proof-20261111/STATEMENT.md`. When STATEMENT.md
pins a statement, the statement here changes to match it.

No statement takes the Johnson-range exception bound as a hypothesis.
-/
import MCA.Defs

namespace MCA

open Finset

variable {F : Type} [Field F] [Fintype F] [DecidableEq F]
variable {ι : Type} [Fintype ι] [DecidableEq ι]

/-- `calcA` (the calculator form, eq. (13) with the Lemma 3.1 degrees) equals `bchksA` (the
printed form, eq. (1)). This is algebra. -/
theorem calcA_eq_bchksA (n : ℕ) (ρ γ : ℝ) (m : ℕ) (hρ : 0 < ρ) :
    calcA n ρ γ m = bchksA n ρ γ m := by
  sorry

/-- At agreement `1 - γ = (4/3) √ρ` the multiplicity of [BCHKS25, Thm. 4.6] is `m = 3`. -/
theorem bchksM_four_thirds (ρ : ℝ) (hρ : 0 < ρ) :
    bchksM ρ (1 - 4 / 3 * Real.sqrt ρ) = 3 := by
  sorry

/-- **Target (ii)**, the degree `w - 1` curve ([BCHKS25, Thm. 4.6] with `M = w - 1`; the
mu-power batching of [SWIRL, Thm. 4.2.4]). For `rsCode dom d` with `2 ≤ d < n`, reduced rate
`ρ = (d - 1) / n`, any radius `0 ≤ γ < 1 - √ρ` and any words `f_0 .. f_(w-1)`, the set of bad
challenges of `Gen(w; μ)` (`w ≥ 1`) has at most `(w - 1) · a` elements, with `a = bchksA n ρ γ m` and
`m = bchksM ρ γ`. -/
theorem mca_curve_johnson (dom : ι ↪ F) (d w : ℕ) (hw : 1 ≤ w) (hd : 2 ≤ d) (hdn : d < Fintype.card ι)
    (γ : ℝ) (hγ0 : 0 ≤ γ)
    (hγ : γ < 1 - Real.sqrt (reducedRate (Fintype.card ι) d))
    (f : Fin w → ι → F) :
    ((mcaBadSet (rsCode dom d : Set (ι → F)) (powGen F w) f γ).card : ℝ) ≤
      ((w : ℝ) - 1) *
        bchksA (Fintype.card ι) (reducedRate (Fintype.card ι) d) γ
          (bchksM (reducedRate (Fintype.card ι) d) γ) := by
  sorry

/-- **Target (i)**, the fold generator `Gen(2; α)` ([BCHKS25, Thm. 4.6] with `M = 1`). -/
theorem mca_fold_johnson (dom : ι ↪ F) (d : ℕ) (hd : 2 ≤ d) (hdn : d < Fintype.card ι)
    (γ : ℝ) (hγ0 : 0 ≤ γ)
    (hγ : γ < 1 - Real.sqrt (reducedRate (Fintype.card ι) d))
    (f : Fin 2 → ι → F) :
    ((mcaBadSet (rsCode dom d : Set (ι → F)) (foldGen F) f γ).card : ℝ) ≤
      bchksA (Fintype.card ι) (reducedRate (Fintype.card ι) d) γ
        (bchksM (reducedRate (Fintype.card ι) d) γ) := by
  have h := mca_curve_johnson dom d 2 (by norm_num) hd hdn γ hγ0 hγ f
  norm_num at h
  simpa [foldGen] using h

/-- Target (i) and (ii) at the design agreement `(4/3) √ρ` (so `m = 3`), as an MCA error bound
in the form of [WHIR, Def. 4.9]: error `d · a / q` with `d = w - 1` (`d = 1` for the fold). -/
theorem hasMCA_curve_four_thirds (dom : ι ↪ F) (d w : ℕ) (hw : 1 ≤ w) (hd : 2 ≤ d)
    (hdn : d < Fintype.card ι)
    (h43 : 4 / 3 * Real.sqrt (reducedRate (Fintype.card ι) d) ≤ 1) :
    HasMCA (rsCode dom d : Set (ι → F)) (powGen F w)
      (1 - 4 / 3 * Real.sqrt (reducedRate (Fintype.card ι) d))
      (((w : ℝ) - 1) *
        bchksA (Fintype.card ι) (reducedRate (Fintype.card ι) d)
          (1 - 4 / 3 * Real.sqrt (reducedRate (Fintype.card ι) d)) 3 /
        Fintype.card F) := by
  sorry

end MCA
