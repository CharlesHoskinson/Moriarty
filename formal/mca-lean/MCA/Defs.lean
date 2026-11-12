/-
OD-19 (b): definitions for the Reed-Solomon mutual correlated agreement (MCA) bound.

The definitions use Mathlib only. They follow the papers literally, so that an auditor can check
each one against the source without reading ArkLib. `MCA/Bridge.lean` relates them to ArkLib.

Sources (read in full for the parts below):
* [WHIR] Arnon, Chiesa, Fenzi, Yogev. WHIR. eprint 2024/1586. Definitions 4.1, 4.9, 4.14.
* [BCHKS25] Ben-Sasson, Carmon, Haboeck, Kopparty, Saraf. On proximity gaps for Reed-Solomon
  codes. ECCC TR25-169. Theorem 1.5, Lemma 3.1, equation (13), Theorems 4.2, 4.5, 4.6.
* [SWIRL] OpenVM SWIRL. Theorem 4.2.4 (the mu-power batching bound, equation (4.7)).
-/
import Mathlib

namespace MCA

open Polynomial Finset

variable {F : Type} [Field F]
variable {ι : Type} [Fintype ι]

/-! ## Reed-Solomon codes -/

/-- Evaluation of a polynomial on an evaluation domain `dom : ι ↪ F`, as an `F`-linear map. -/
noncomputable def evalOn (dom : ι ↪ F) : F[X] →ₗ[F] (ι → F) where
  toFun p := fun i => p.eval (dom i)
  map_add' p q := by funext i; simp
  map_smul' c p := by funext i; simp

/-- [WHIR, Def. 4.1] The Reed-Solomon code `RS[F, L, d]`: the evaluations on `L` of the
polynomials of degree strictly less than `d`. The rate is `d / |L|`.

[BCHKS25] writes `RS[F_q, D, k]` for polynomials of degree at most `k`, that is `d = k + 1`, and
its `ρ = k / n` is the rate minus `1 / n` ("the slightly reduced rate"). See `reducedRate`. -/
noncomputable def rsCode (dom : ι ↪ F) (d : ℕ) : Submodule F (ι → F) :=
  (Polynomial.degreeLT F d).map (evalOn dom)

/-- The rate `d / n` of `rsCode dom d` ([WHIR, Def. 4.1]). This is the `ρ` of the calculator
(`2^-log_inv_rate`). -/
noncomputable def rate (n d : ℕ) : ℝ := (d : ℝ) / n

/-- The reduced rate `(d - 1) / n` of `rsCode dom d`. This is the `ρ` of [BCHKS25] Theorems 1.5,
4.2, 4.5 and 4.6 (their `k / n` with `k = d - 1` the degree bound). -/
noncomputable def reducedRate (n d : ℕ) : ℝ := ((d - 1 : ℕ) : ℝ) / n

/-! ## Distance and agreement -/

variable [DecidableEq F]

/-- Relative Hamming distance `Δ(f, g) = |{x : f x ≠ g x}| / n`. -/
noncomputable def relDist (f g : ι → F) : ℝ := (hammingDist f g : ℝ) / Fintype.card ι

/-- Relative distance from a word to a code: the minimum over codewords. -/
noncomputable def relDistToCode (f : ι → F) (C : Set (ι → F)) : ℝ :=
  ⨅ u : C, relDist f u

/-- `f` agrees with some codeword of `C` on every point of `S`
(in [WHIR, Def. 4.9]: `∃ u ∈ C, u(S) = f(S)`). -/
def AgreesOn (C : Set (ι → F)) (f : ι → F) (S : Finset ι) : Prop :=
  ∃ u ∈ C, ∀ x ∈ S, u x = f x

/-- The size condition `|S| ≥ (1 - δ) n` of [WHIR, Defs. 4.7 and 4.9]. -/
def LargeSet (δ : ℝ) (S : Finset ι) : Prop :=
  ((1 - δ) * Fintype.card ι : ℝ) ≤ S.card

/-- Correlated agreement ([WHIR, Def. 4.7] conclusion; [BCHKS25] `Δ([u_0..u_M], C^{M+1}) ≤ γ`):
one set `S` with `|S| ≥ (1 - δ) n` on which every `f_i` agrees with a codeword. -/
def CorrelatedAgreement {ℓ : Type} (C : Set (ι → F)) (f : ℓ → ι → F) (δ : ℝ) : Prop :=
  ∃ S : Finset ι, LargeSet δ S ∧ ∀ i, AgreesOn C (f i) S

/-! ## Generators -/

/-- A generator with seed space `R` and `ℓ` outputs ([WHIR, Def. 4.7]: `(r_1..r_ℓ) ← Gen(ℓ)`). -/
abbrev Generator (R : Type) (ℓ : ℕ) (F : Type) := R → Fin ℓ → F

/-- [WHIR, Thm. 4.8] `Gen(ℓ; α) = (1, α, ..., α^(ℓ-1))`. For `ℓ = w` this is the degree `w - 1`
curve of mu-power batching ([SWIRL, Thm. 4.2.4]); it combines `f_0 + α f_1 + ... + α^(w-1) f_(w-1)`.
-/
def powGen (F : Type) [Field F] (ℓ : ℕ) : Generator F ℓ F := fun α j => α ^ (j : ℕ)

/-- The fold generator `Gen(2; α) = (1, α)`: the affine line `f_0 + α f_1`. -/
def foldGen (F : Type) [Field F] : Generator F 2 F := powGen F 2

/-- The generated word `∑_j r_j • f_j` ([WHIR, Def. 4.9]). -/
def combine {R : Type} {ℓ : ℕ} (G : Generator R ℓ F) (r : R) (f : Fin ℓ → ι → F) : ι → F :=
  fun x => ∑ j, G r j * f j x

/-! ## Mutual correlated agreement ([WHIR, Def. 4.9]) -/

/-- The MCA bad event of [WHIR, Def. 4.9] at seed `r`: there is `S` with `|S| ≥ (1 - δ) n` such
that the generated word agrees with a codeword on `S`, but some `f_i` agrees with no codeword
on `S`. -/
def McaBad {R : Type} {ℓ : ℕ} (C : Set (ι → F)) (G : Generator R ℓ F) (f : Fin ℓ → ι → F)
    (δ : ℝ) (r : R) : Prop :=
  ∃ S : Finset ι, LargeSet δ S ∧ AgreesOn C (combine G r f) S ∧ ∃ i, ¬ AgreesOn C (f i) S

open Classical in
/-- The set of bad seeds (the exceptional set `E` of [BCHKS25, Thm. 4.6]). -/
noncomputable def mcaBadSet {R : Type} [Fintype R] {ℓ : ℕ} (C : Set (ι → F))
    (G : Generator R ℓ F) (f : Fin ℓ → ι → F) (δ : ℝ) : Finset R :=
  Finset.univ.filter (McaBad C G f δ)

/-- The MCA error for one tuple `f`: the probability of the bad event over a uniform seed. With a
uniform seed this is `|E| / |R|`, so [WHIR, Def. 4.9] holds with error `err⋆` if and only if
`mcaErr C G f δ ≤ err⋆` for every `f`. -/
noncomputable def mcaErr {R : Type} [Fintype R] {ℓ : ℕ} (C : Set (ι → F))
    (G : Generator R ℓ F) (f : Fin ℓ → ι → F) (δ : ℝ) : ℝ :=
  ((mcaBadSet C G f δ).card : ℝ) / Fintype.card R

/-- [WHIR, Def. 4.9]: `G` has mutual correlated agreement for `C` with error `err` at radius `δ`. -/
def HasMCA {R : Type} [Fintype R] {ℓ : ℕ} (C : Set (ι → F)) (G : Generator R ℓ F)
    (δ err : ℝ) : Prop :=
  ∀ f : Fin ℓ → ι → F, mcaErr C G f δ ≤ err

/-! ## Folding ([WHIR, Def. 4.14]) -/

/-- The even part `(f x + f (-x)) / 2` of `f` at `x`. -/
noncomputable def foldEven (f : F → F) (x : F) : F := (f x + f (-x)) / 2

/-- The odd part `(f x - f (-x)) / (2 x)` of `f` at `x`. -/
noncomputable def foldOdd (f : F → F) (x : F) : F := (f x - f (-x)) / (2 * x)

/-- [WHIR, Def. 4.14] `Fold(f, α)(x^2) := (f x + f (-x)) / 2 + α (f x - f (-x)) / (2 x)`, given
pointwise at `x` (the value at `x^2`). -/
noncomputable def fold (f : F → F) (α x : F) : F :=
  (f x + f (-x)) / 2 + α * ((f x - f (-x)) / (2 * x))

/-- A fold is the fold generator `Gen(2; α)` applied to the even and odd parts. So the fold step
of WHIR consumes MCA for `foldGen` on the pair `(foldEven f, foldOdd f)`. -/
theorem fold_eq_combine (f : F → F) (α x : F) :
    fold f α x = foldEven f x + α * foldOdd f x := rfl

/-! ## The BCHKS25 bound -/

/-- The multiplicity of [BCHKS25, Thms. 4.2, 4.5, 4.6]: `m = max(⌈√ρ / (1 - √ρ - γ)⌉, 3)`.
At agreement `1 - γ = (4/3) √ρ` this is `3` (`bchksM_four_thirds`). -/
noncomputable def bchksM (ρ γ : ℝ) : ℕ := max ⌈Real.sqrt ρ / (1 - Real.sqrt ρ - γ)⌉₊ 3

/-- The printed bound of [BCHKS25, eq. (1) and Thm. 4.6] for one line:
`(2 (m+1/2)^5 + 3 (m+1/2) γ ρ) / (3 ρ^{3/2}) · n + (m+1/2) / √ρ`. -/
noncomputable def bchksA (n : ℕ) (ρ γ : ℝ) (m : ℕ) : ℝ :=
  let t : ℝ := m + 1 / 2
  (2 * t ^ 5 + 3 * t * γ * ρ) / (3 * Real.sqrt ρ ^ 3) * n + t / Real.sqrt ρ

/-- The degrees of [BCHKS25, Lemma 3.1, eqs. (7)-(9)] with `k / n = ρ`:
`D_X = (m+1/2) n √ρ`, `D_Y = (m+1/2) / √ρ`, `D_Z = (m+1/2)^2 / (3 ρ)`. -/
noncomputable def degX (n : ℕ) (ρ : ℝ) (m : ℕ) : ℝ := (m + 1 / 2) * n * Real.sqrt ρ
noncomputable def degY (ρ : ℝ) (m : ℕ) : ℝ := (m + 1 / 2) / Real.sqrt ρ
noncomputable def degZ (ρ : ℝ) (m : ℕ) : ℝ := (m + 1 / 2) ^ 2 / (3 * ρ)

/-- The calculator form `a = 2 D_X D_Y^2 D_Z + (γ n + 1) D_Y` ([BCHKS25, eq. (13)];
`calculator.rs` `bchks25_log2_a_from_log2_degrees`). -/
noncomputable def calcA (n : ℕ) (ρ γ : ℝ) (m : ℕ) : ℝ :=
  2 * degX n ρ m * degY ρ m ^ 2 * degZ ρ m + (γ * n + 1) * degY ρ m

end MCA
