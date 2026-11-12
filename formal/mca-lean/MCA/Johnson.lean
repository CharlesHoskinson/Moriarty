/-
OD-19 (b), route (a): the ArkLib Johnson exception count at integer parameters.

ArkLib's curve theorem (`ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype`,
sorry-free, standard axioms) bounds the exceptional set of a degree `ℓ` power curve by
`2 ℓ · johnsonExceptionCount n D A η`. This file evaluates that count exactly when the agreement
threshold is an integer `A` with `7 √(k / n) ≤ 6 A / n` (multiplicity `m = 3`).

We choose the slack `η := A / n - √(k / n)`, so that the agreement fraction `√(k / n) + η` is
exactly `A / n`. Then, under the integer side conditions `JOk n k A μ0 h0`:

* `johnsonM n k η = 3`,
* `johnsonMu n k η = μ0` and `johnsonH n k η = h0`,
* `johnsonExceptionCount n k A η · (A - k) = countNum n k A μ0 h0`, a natural number.

So every per-row check reduces to a closed natural-number computation.
-/
import MCA.Defs
import ArkLib.Data.CodingTheory.ProximityGap.BCIKS20.CurveBridge

namespace MCA

open ReedSolomon.HiddenDerivative

/-- The integer side conditions of the Johnson count at threshold `A`, as a decidable Boolean.
`k` is the degree bound (`deg ≤ k`), `n` the domain size. -/
def jok (n k A μ0 h0 : ℕ) : Bool :=
  decide (1 ≤ k) && decide (k + 2 ≤ n) && decide (A ≤ n) && decide (k < A) &&
  decide (49 * k * n ≤ 36 * A * A) && decide (4 * k * μ0 * μ0 < 49 * n) &&
  decide (49 * n ≤ 4 * k * (μ0 + 1) * (μ0 + 1)) && decide (12 * k * h0 < 49 * n) &&
  decide (49 * n ≤ 12 * k * (h0 + 1))

/-- The integer side conditions of the Johnson count at threshold `A`:
`1 ≤ k ≤ n - 2`, `k < A ≤ n`, `49 k n ≤ 36 A²` (so `m = 3`), and `μ0`, `h0` are the integer
parts `⌈(7/2) √(n/k)⌉ - 1` and `⌈49 n / (12 k)⌉ - 1`. -/
structure JOk (n k A μ0 h0 : ℕ) : Prop where
  hk : 1 ≤ k
  hkn : k + 2 ≤ n
  hAn : A ≤ n
  hkA : k < A
  h49 : 49 * k * n ≤ 36 * A * A
  hmu1 : 4 * k * μ0 * μ0 < 49 * n
  hmu2 : 49 * n ≤ 4 * k * (μ0 + 1) * (μ0 + 1)
  hh1 : 12 * k * h0 < 49 * n
  hh2 : 49 * n ≤ 12 * k * (h0 + 1)

theorem JOk.of_jok {n k A μ0 h0 : ℕ} (h : jok n k A μ0 h0 = true) : JOk n k A μ0 h0 := by
  simp only [jok, Bool.and_eq_true, decide_eq_true_eq] at h
  obtain ⟨⟨⟨⟨⟨⟨⟨⟨h1, h2⟩, h3⟩, h4⟩, h5⟩, h6⟩, h7⟩, h8⟩, h9⟩ := h
  exact ⟨h1, h2, h3, h4, h5, h6, h7, h8, h9⟩

/-- The numerator of the exception count: `johnsonExceptionCount · (A - k)`
(`JOk.count_mul`). -/
def countNum (n k A μ0 h0 : ℕ) : ℕ :=
  (2 * μ0 - 1) * h0 * (A - k) + (n - k) * (h0 + μ0 + 4 * k * μ0 * h0) +
    (n - k - 1) * μ0 * (A - k)

/-- The slack `η = A / n - √(k / n)`: the agreement fraction `√(k/n) + η` is `A / n`. -/
noncomputable def jEta (n k A : ℕ) : ℝ := (A : ℝ) / n - Real.sqrt (johnsonRhoMinus n k)

theorem lt_of_sq_lt' {a b : ℝ} (hb : 0 ≤ b) (h : a ^ 2 < b ^ 2) : a < b := by
  by_contra hc
  push_neg at hc
  have : b * b ≤ a * a := mul_self_le_mul_self hb hc
  nlinarith

theorem le_of_sq_le' {a b : ℝ} (hb : 0 ≤ b) (h : a ^ 2 ≤ b ^ 2) : a ≤ b := by
  by_contra hc
  push_neg at hc
  have : b * b < a * a := mul_self_lt_mul_self hb hc
  nlinarith

namespace JOk

variable {n k A μ0 h0 : ℕ} (h : JOk n k A μ0 h0)
include h

theorem n_pos : (0 : ℝ) < n := by
  have := h.hkn
  exact_mod_cast (by omega : 0 < n)

theorem k_pos : (0 : ℝ) < k := by
  have := h.hk
  exact_mod_cast (by omega : 0 < k)

theorem nk_lt : n * k < A * A := by
  have h1 := h.h49
  have h2 : 0 < k * n := Nat.mul_pos (by have := h.hk; omega) (by have := h.hkn; omega)
  nlinarith

theorem rho_pos : 0 < johnsonRhoMinus n k := by
  unfold johnsonRhoMinus
  exact div_pos h.k_pos h.n_pos

theorem sqrt_pos : 0 < Real.sqrt (johnsonRhoMinus n k) := Real.sqrt_pos.mpr h.rho_pos

theorem sqrt_sq : Real.sqrt (johnsonRhoMinus n k) ^ 2 = (k : ℝ) / n := by
  rw [Real.sq_sqrt h.rho_pos.le]
  rfl

/-- `7 √(k/n) ≤ 6 A / n`. -/
theorem seven_le : 7 * Real.sqrt (johnsonRhoMinus n k) ≤ 6 * ((A : ℝ) / n) := by
  have hn := h.n_pos
  have h49 : (49 : ℝ) * k * n ≤ 36 * A * A := by exact_mod_cast h.h49
  apply le_of_sq_le' (by positivity)
  rw [mul_pow, h.sqrt_sq]
  have e : (6 * ((A : ℝ) / n)) ^ 2 - 7 ^ 2 * ((k : ℝ) / n) = (36 * A * A - 49 * k * n) / (n : ℝ) ^ 2 := by
    field_simp
    ring
  have : 0 ≤ (36 * (A : ℝ) * A - 49 * k * n) / (n : ℝ) ^ 2 := div_nonneg (by linarith) (by positivity)
  linarith

theorem sqrt_lt : Real.sqrt (johnsonRhoMinus n k) < (A : ℝ) / n := by
  have h1 := h.seven_le
  have h2 := h.sqrt_pos
  linarith

theorem eta_pos : 0 < jEta n k A := by
  unfold jEta
  linarith [h.sqrt_lt]

theorem agree_mul : johnsonAgreement n k (jEta n k A) * n = A := by
  have hn := h.n_pos
  unfold johnsonAgreement jEta
  have : Real.sqrt (johnsonRhoMinus n k) + ((A : ℝ) / n - Real.sqrt (johnsonRhoMinus n k)) =
      (A : ℝ) / n := by ring
  rw [this]
  field_simp

theorem m_eq : johnsonM n k (jEta n k A) = 3 := by
  unfold johnsonM
  apply max_eq_right
  rw [Nat.ceil_le]
  have he := h.eta_pos
  have h7 := h.seven_le
  rw [div_le_iff₀ (by linarith)]
  unfold jEta at he ⊢
  push_cast
  linarith

theorem t_eq : johnsonT n k (jEta n k A) = 7 / 2 := by
  unfold johnsonT
  rw [h.m_eq]
  norm_num

theorem mu_eq : johnsonMu n k (jEta n k A) = μ0 := by
  have hn := h.n_pos
  have hr := h.sqrt_pos
  have hr2 := h.sqrt_sq
  set r := Real.sqrt (johnsonRhoMinus n k) with hr_def
  have h1 : (4 : ℝ) * k * μ0 * μ0 < 49 * n := by exact_mod_cast h.hmu1
  have h2 : (49 : ℝ) * n ≤ 4 * k * (μ0 + 1) * (μ0 + 1) := by exact_mod_cast h.hmu2
  have hlow : (μ0 : ℝ) * r < 7 / 2 := by
    apply lt_of_sq_lt' (by norm_num)
    rw [mul_pow, hr2]
    have e : ((7 : ℝ) / 2) ^ 2 - (μ0 : ℝ) ^ 2 * ((k : ℝ) / n) =
        (49 * n - 4 * k * μ0 * μ0) / (4 * n) := by
      field_simp
      ring
    have : 0 < (49 * (n : ℝ) - 4 * k * μ0 * μ0) / (4 * n) := div_pos (by linarith) (by positivity)
    linarith
  have hup : (7 : ℝ) / 2 ≤ ((μ0 : ℝ) + 1) * r := by
    apply le_of_sq_le' (by positivity)
    rw [mul_pow, hr2]
    have e : ((μ0 : ℝ) + 1) ^ 2 * ((k : ℝ) / n) - ((7 : ℝ) / 2) ^ 2 =
        (4 * k * (μ0 + 1) * (μ0 + 1) - 49 * n) / (4 * n) := by
      field_simp
      ring
    have : 0 ≤ (4 * (k : ℝ) * (μ0 + 1) * (μ0 + 1) - 49 * n) / (4 * n) :=
      div_nonneg (by linarith) (by positivity)
    linarith
  unfold johnsonMu
  rw [h.t_eq]
  show ⌈(7 / 2 : ℝ) / r⌉₊ - 1 = μ0
  have hc : ⌈(7 / 2 : ℝ) / r⌉₊ = μ0 + 1 := by
    rw [Nat.ceil_eq_iff (by omega)]
    constructor
    · rw [Nat.add_sub_cancel, lt_div_iff₀ hr]
      exact hlow
    · rw [div_le_iff₀ hr]
      push_cast
      exact hup
  rw [hc]
  omega

theorem h_eq : johnsonH n k (jEta n k A) = h0 := by
  have hn := h.n_pos
  have hk := h.k_pos
  have h1 : (12 : ℝ) * k * h0 < 49 * n := by exact_mod_cast h.hh1
  have h2 : (49 : ℝ) * n ≤ 12 * k * (h0 + 1) := by exact_mod_cast h.hh2
  unfold johnsonH
  rw [h.t_eq]
  have hρ : johnsonRhoMinus n k = (k : ℝ) / n := rfl
  rw [hρ]
  have hval : ((7 : ℝ) / 2) ^ 2 / (3 * ((k : ℝ) / n)) = 49 * n / (12 * k) := by
    field_simp
    ring
  rw [hval]
  have hc : ⌈(49 * (n : ℝ)) / (12 * k)⌉₊ = h0 + 1 := by
    rw [Nat.ceil_eq_iff (by omega)]
    constructor
    · rw [Nat.add_sub_cancel, lt_div_iff₀ (by positivity)]
      linarith
    · rw [div_le_iff₀ (by positivity)]
      push_cast
      linarith
  rw [hc]
  omega

/-- The exception count at threshold `A` times `A - k` is the natural number `countNum`. -/
theorem count_mul :
    johnsonExceptionCount n k A (jEta n k A) * ((A - k : ℕ) : ℝ) =
      (countNum n k A μ0 h0 : ℝ) := by
  have hA : ((A - k : ℕ) : ℝ) ≠ 0 := by
    have := h.hkA
    exact_mod_cast (by omega : A - k ≠ 0)
  simp only [johnsonExceptionCount, johnsonTheta, h.mu_eq, h.h_eq, countNum]
  push_cast
  field_simp

end JOk

end MCA
