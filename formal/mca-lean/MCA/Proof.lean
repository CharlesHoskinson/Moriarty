/-
OD-19 (b), route (a): the PROOF.md integer lemmas and the WHIR (H2) error function.

* Lemma 9.0: (H1) to (H6) at the 26 frozen rows, by kernel computation (`decide +kernel`).
* Lemma 9.0b: (H1) to (H6) for every `g ≤ g_end` at the 30 WHIR codes (computation at `g_end`
  and monotonicity in `g`).
* Corollary 9.2: the fold generator has MCA with error `errStar` on the whole radius interval,
  for each of the 30 codes. The proof is route (a): ArkLib's proved Johnson curve count
  (`MCA/Route.lean`, `curve_card_le` with `M = 1`) is at most `a*_1(0) ≤ a*_1(⌊δ n⌋)`.

PROOF.md Theorem 7.1 (the `a*_M` bound from (H1) to (H6)) is not formalized: route (a) replaces
it with ArkLib's count, which is at most `a*_M` at every row (`rowOk`). (H1) to (H6) stay as
checked integer facts. Degree convention: `k` is the degree bound, the code is
`rsCode dom (k + 1)`.
-/
import MCA.Route

namespace MCA

open Finset

variable {F : Type} [Field F] [Fintype F] [DecidableEq F]
variable {ι : Type} [Fintype ι] [DecidableEq ι]

/-- The bound `a*_M := (a_Z - 1)(1 + 4 N (a_Y - 1)) + M (a_Y - 1)(g + 1)`, `N = a_X - 1`. -/
def aStar (M aX aY aZ g : ℕ) : ℕ := (aZ - 1) * (1 + 4 * (aX - 1) * (aY - 1)) + M * (aY - 1) * (g + 1)

theorem aStar_mono {M aX aY aZ g g' : ℕ} (h : g ≤ g') : aStar M aX aY aZ g ≤ aStar M aX aY aZ g' := by
  unfold aStar
  have : M * (aY - 1) * (g + 1) ≤ M * (aY - 1) * (g' + 1) := Nat.mul_le_mul_left _ (by omega)
  omega

/-- The hypotheses (H1) to (H6) of PROOF.md section 1, over the integers, with `N = a_X - 1` and
`A_0 = n - g` written out. `p` is the characteristic and `q = |F|`. -/
abbrev Hyps (n k M m aX aY aZ g p q : ℕ) : Prop :=
  -- (H1)
  ((m : ℤ) * ((n : ℤ) - g) ≥ aX) ∧
  -- (H2)
  ((aY : ℤ) - 1 < p) ∧
  -- (H3)
  ((q : ℤ) > 2 * ((aX : ℤ) - 1) * ((aY : ℤ) - 1) ∧ 2 ≤ aY) ∧
  -- (H4)
  ((∀ b ∈ Finset.range aY, (aX : ℤ) - k * b ≥ 1) ∧
    (∑ b ∈ Finset.range aY, ((aX : ℤ) - k * b) * max 0 ((aZ : ℤ) - M * b)) >
      (n : ℤ) * ∑ t ∈ Finset.range m, ((m : ℤ) - t) * max 0 ((aZ : ℤ) - M * t)) ∧
  -- (H5)
  ((n : ℤ) - g > k) ∧
  -- (H6)
  ((2 * (k : ℤ) - 1 + M) * ((n : ℤ) - k) ≤ (2 * ((aX : ℤ) - 1) - 1) * (((n : ℤ) - g) - k))

/-- (H1) to (H6) at `g` give them at every `g' ≤ g` (for `a_X ≥ 2`): only (H1), (H5) and (H6)
mention `g`, and each gets weaker as `g` grows. -/
theorem Hyps.mono {n k M m aX aY aZ g g' p q : ℕ} (h : Hyps n k M m aX aY aZ g p q)
    (hg : g' ≤ g) (haX : 2 ≤ aX) : Hyps n k M m aX aY aZ g' p q := by
  obtain ⟨h1, h2, h3, h4, h5, h6⟩ := h
  have hg' : (g' : ℤ) ≤ g := by exact_mod_cast hg
  have haX' : (2 : ℤ) ≤ aX := by exact_mod_cast haX
  refine ⟨?_, h2, h3, h4, by omega, ?_⟩
  · nlinarith [mul_nonneg (Int.natCast_nonneg m) (sub_nonneg.mpr hg')]
  · nlinarith [mul_nonneg (by linarith : (0 : ℤ) ≤ 2 * ((aX : ℤ) - 1) - 1) (sub_nonneg.mpr hg')]

/-- One WHIR code `C^(i,s) = RS_k[F, D]` of PROOF.md Lemma 9.0b at `M = 1`:
`(n, k, a_X, a_Y, a_Z, g_end, μ0, h0)` with `g_end = n - ⌈√(49 k n / 36)⌉`, and `μ0`, `h0` the
Johnson parameters of `JOk` at threshold `A = n - g_end`. -/
structure WhirCode where
  n : ℕ
  k : ℕ
  aX : ℕ
  aY : ℕ
  aZ : ℕ
  gEnd : ℕ
  mu0 : ℕ
  h0 : ℕ

/-- The 30 codes of PROOF.md Lemma 9.0b (`0 ≤ s ≤ 4` in each of the three WHIR rounds of `L` and
of the internal layer), computed with the exact integer arithmetic of `bits.py`. -/
def whirCodes : List WhirCode := [
    ⟨16777216, 1048575, 14680057, 15, 66, 11883863, 14, 65⟩,  -- mu=20 beta=4
    ⟨8388608, 524287, 7340025, 15, 66, 5941933, 14, 65⟩,  -- mu=19 beta=4
    ⟨4194304, 262143, 3670009, 15, 66, 2970967, 14, 65⟩,  -- mu=18 beta=4
    ⟨2097152, 131071, 1835001, 15, 66, 1485485, 14, 65⟩,  -- mu=17 beta=4
    ⟨1048576, 65535, 917497, 15, 66, 742743, 14, 65⟩,  -- mu=16 beta=4
    ⟨8388608, 65535, 2595074, 40, 523, 7523583, 39, 522⟩,  -- mu=16 beta=7
    ⟨4194304, 32767, 1297527, 40, 523, 3761795, 39, 522⟩,  -- mu=15 beta=7
    ⟨2097152, 16383, 648754, 40, 523, 1880900, 39, 522⟩,  -- mu=14 beta=7
    ⟨1048576, 8191, 324367, 40, 523, 940453, 39, 522⟩,  -- mu=13 beta=7
    ⟨524288, 4095, 162174, 40, 523, 470230, 39, 522⟩,  -- mu=12 beta=7
    ⟨4194304, 4095, 458696, 113, 4183, 4041405, 112, 4182⟩,  -- mu=12 beta=10
    ⟨2097152, 2047, 229320, 113, 4184, 2020712, 112, 4183⟩,  -- mu=11 beta=10
    ⟨1048576, 1023, 114632, 113, 4186, 1010365, 112, 4185⟩,  -- mu=10 beta=10
    ⟨524288, 511, 57288, 113, 4190, 505192, 112, 4189⟩,  -- mu=9 beta=10
    ⟨262144, 255, 28616, 113, 4198, 252605, 112, 4197⟩,  -- mu=8 beta=10
    ⟨4194304, 524287, 5190182, 10, 33, 2464243, 9, 32⟩,  -- mu=19 beta=3
    ⟨2097152, 262143, 2595089, 10, 33, 1232122, 9, 32⟩,  -- mu=18 beta=3
    ⟨1048576, 131071, 1297542, 10, 33, 616062, 9, 32⟩,  -- mu=17 beta=3
    ⟨524288, 65535, 648769, 10, 33, 308031, 9, 32⟩,  -- mu=16 beta=3
    ⟨262144, 32767, 324382, 10, 33, 154016, 9, 32⟩,  -- mu=15 beta=3
    ⟨2097152, 32767, 917490, 29, 262, 1791322, 28, 261⟩,  -- mu=15 beta=6
    ⟨1048576, 16383, 458738, 29, 262, 895663, 28, 261⟩,  -- mu=14 beta=6
    ⟨524288, 8191, 229362, 29, 262, 447834, 28, 261⟩,  -- mu=13 beta=6
    ⟨262144, 4095, 114674, 29, 262, 223919, 28, 261⟩,  -- mu=12 beta=6
    ⟨131072, 2047, 57330, 29, 262, 111962, 28, 261⟩,  -- mu=11 beta=6
    ⟨1048576, 2047, 162154, 80, 2092, 994524, 79, 2091⟩,  -- mu=11 beta=9
    ⟨524288, 1023, 81058, 80, 2093, 497268, 79, 2092⟩,  -- mu=10 beta=9
    ⟨262144, 511, 40509, 80, 2095, 248641, 79, 2094⟩,  -- mu=9 beta=9
    ⟨131072, 255, 20235, 80, 2099, 124327, 79, 2098⟩,  -- mu=8 beta=9
    ⟨65536, 127, 10098, 80, 2108, 62170, 79, 2107⟩   -- mu=7 beta=9
  ]

/-- BabyBear, `p = 2^31 - 2^27 + 1`; the challenge field is `BabyBear^4`, `q = p^4`. -/
def babyBearP : ℕ := 2013265921

/-- The closed check for one code: the Johnson conditions at `A = n - g_end`, ArkLib's fold
count `2 · countNum / (A - k)` at most `a*_1(0)`, `a_X ≥ 2` and `g_end ≤ n`. -/
def codeOk (c : WhirCode) : Bool :=
  jok c.n c.k (c.n - c.gEnd) c.mu0 c.h0 &&
  decide (2 * 1 * countNum c.n c.k (c.n - c.gEnd) c.mu0 c.h0 / (c.n - c.gEnd - c.k) ≤
    aStar 1 c.aX c.aY c.aZ 0) &&
  decide (2 ≤ c.aX) && decide (c.gEnd ≤ c.n)

theorem codes_ok : ∀ c ∈ whirCodes, codeOk c = true := by decide +kernel

/-- (H1) to (H6) at `g_end` for the 30 codes. -/
theorem codes_hyps : ∀ c ∈ whirCodes,
    Hyps c.n c.k 1 3 c.aX c.aY c.aZ c.gEnd babyBearP (babyBearP ^ 4) := by
  decide +kernel

/-- **Lemma 9.0b** (PROOF.md). For every code of `whirCodes` and every `g ≤ g_end`, (H1) to (H6)
hold with `M = 1`, `m = 3`, `p = babyBearP`, `q = p^4`. -/
theorem lemma_9_0b : ∀ c ∈ whirCodes, ∀ g ≤ c.gEnd,
    Hyps c.n c.k 1 3 c.aX c.aY c.aZ g babyBearP (babyBearP ^ 4) := by
  intro c hc g hg
  have hok := codes_ok c hc
  simp only [codeOk, Bool.and_eq_true, decide_eq_true_eq] at hok
  exact (codes_hyps c hc).mono hg hok.1.2

/-- **Definition 9.1b** (PROOF.md). The WHIR (H2) error function of code `c` on the whole radius
interval: `a*_1(⌊δ n⌋) / q` if `⌊δ n⌋ ≤ g_end`, and `1` otherwise. -/
noncomputable def errStar (c : WhirCode) (q : ℕ) (δ : ℝ) : ℝ :=
  if ⌊δ * c.n⌋₊ ≤ c.gEnd then (aStar 1 c.aX c.aY c.aZ ⌊δ * c.n⌋₊ : ℝ) / q else 1

/-- **Corollary 9.2** (PROOF.md, statement (i)). For every code `c` of `whirCodes`, any field `F`,
any domain `D` of size `n` and every radius `δ`, the fold generator `Gen(2; α)` has MCA for
`RS_k[F, D]` with error `errStar c |F| δ` (WHIR Theorem 5.2 hypothesis H2). -/
theorem cor_9_2 (c : WhirCode) (hc : c ∈ whirCodes) (dom : ι ↪ F)
    (hn : Fintype.card ι = c.n) (δ : ℝ) :
    HasMCA (rsCode dom (c.k + 1) : Set (ι → F)) (foldGen F) δ
      (errStar c (Fintype.card F) δ) := by
  have hok := codes_ok c hc
  simp only [codeOk, Bool.and_eq_true, decide_eq_true_eq] at hok
  obtain ⟨⟨⟨hj, hb⟩, -⟩, hge⟩ := hok
  have hJ := JOk.of_jok hj
  rw [← hn] at hJ
  intro f
  unfold errStar mcaErr
  split_ifs with hg
  · apply div_le_div_of_nonneg_right _ (Nat.cast_nonneg _)
    have h1 : (mcaBadSet (rsCode dom (c.k + 1) : Set (ι → F)) (foldGen F) f δ).card ≤
        2 * 1 * countNum (Fintype.card ι) c.k (Fintype.card ι - c.gEnd) c.mu0 c.h0 /
          (Fintype.card ι - c.gEnd - c.k) :=
      curve_card_le dom (M := 1) hJ f δ (by rw [hn]; omega)
    rw [hn] at h1
    exact_mod_cast (h1.trans hb).trans (aStar_mono (Nat.zero_le _))
  · rw [div_le_one (by exact_mod_cast Fintype.card_pos)]
    exact_mod_cast Finset.card_le_univ _

/-- One frozen row `(n, k, M, a_X, a_Y, a_Z, g, μ0, h0, bits)` with `m = 3`: `μ0`, `h0` are the
Johnson parameters at threshold `A = n - g`, and `bits` is the claimed security level. -/
structure Row where
  n : ℕ
  k : ℕ
  M : ℕ
  aX : ℕ
  aY : ℕ
  aZ : ℕ
  g : ℕ
  mu0 : ℕ
  h0 : ℕ
  bits : ℕ

/-- The 26 frozen rows of STATEMENT.md section 7, computed by `bits.py` (`ints`): 12 `L` folds,
12 internal folds (`M = 1`), the `L` mu row (`M = 47`) and the internal mu row (`M = 511`). -/
def frozenRows : List Row := [
    ⟨8388608, 524287, 1, 7340025, 15, 66, 5592405, 14, 65, 88⟩,  -- mu=19 beta=4
    ⟨4194304, 262143, 1, 3670009, 15, 66, 2796202, 14, 65, 89⟩,  -- mu=18 beta=4
    ⟨2097152, 131071, 1, 1835001, 15, 66, 1398101, 14, 65, 90⟩,  -- mu=17 beta=4
    ⟨1048576, 65535, 1, 917497, 15, 66, 699050, 14, 65, 91⟩,  -- mu=16 beta=4
    ⟨4194304, 32767, 1, 1297527, 40, 523, 3700000, 39, 522, 87⟩,  -- mu=15 beta=7
    ⟨2097152, 16383, 1, 648754, 40, 523, 1850000, 39, 522, 88⟩,  -- mu=14 beta=7
    ⟨1048576, 8191, 1, 324367, 40, 523, 925000, 39, 522, 89⟩,  -- mu=13 beta=7
    ⟨524288, 4095, 1, 162174, 40, 523, 462500, 39, 522, 90⟩,  -- mu=12 beta=7
    ⟨2097152, 2047, 1, 229320, 113, 4184, 2009770, 112, 4183, 84⟩,  -- mu=11 beta=10
    ⟨1048576, 1023, 1, 114632, 113, 4186, 1004885, 112, 4185, 85⟩,  -- mu=10 beta=10
    ⟨524288, 511, 1, 57288, 113, 4190, 502442, 112, 4189, 86⟩,  -- mu=9 beta=10
    ⟨262144, 255, 1, 28616, 113, 4198, 251221, 112, 4197, 87⟩,  -- mu=8 beta=10
    ⟨2097152, 262143, 1, 2595089, 10, 33, 1108545, 9, 32, 92⟩,  -- mu=18 beta=3
    ⟨1048576, 131071, 1, 1297542, 10, 33, 554272, 9, 32, 93⟩,  -- mu=17 beta=3
    ⟨524288, 65535, 1, 648769, 10, 33, 277136, 9, 32, 94⟩,  -- mu=16 beta=3
    ⟨262144, 32767, 1, 324382, 10, 33, 138568, 9, 32, 95⟩,  -- mu=15 beta=3
    ⟨1048576, 16383, 1, 458738, 29, 262, 873813, 28, 261, 89⟩,  -- mu=14 beta=6
    ⟨524288, 8191, 1, 229362, 29, 262, 436906, 28, 261, 90⟩,  -- mu=13 beta=6
    ⟨262144, 4095, 1, 114674, 29, 262, 218453, 28, 261, 91⟩,  -- mu=12 beta=6
    ⟨131072, 2047, 1, 57330, 29, 262, 109226, 28, 261, 92⟩,  -- mu=11 beta=6
    ⟨524288, 1023, 1, 81058, 80, 2093, 493394, 79, 2092, 87⟩,  -- mu=10 beta=9
    ⟨262144, 511, 1, 40509, 80, 2095, 246697, 79, 2094, 88⟩,  -- mu=9 beta=9
    ⟨131072, 255, 1, 20235, 80, 2099, 123348, 79, 2098, 89⟩,  -- mu=8 beta=9
    ⟨65536, 127, 1, 10098, 80, 2108, 61674, 79, 2107, 90⟩,  -- mu=7 beta=9
    ⟨16777216, 1048575, 47, 14680057, 15, 3071, 11184810, 14, 65, 82⟩,  -- mu=20 beta=4
    ⟨4194304, 524287, 511, 5190182, 10, 16693, 2217090, 9, 32, 82⟩   -- mu=19 beta=3
  ]

/-- The closed check for one row: the Johnson conditions at `A = n - g`; ArkLib's count plus the
subcode term, `2 M · countNum / (A - k) + M · ⌊n (A - k) / (A² - n k)⌋`, at most `a*_M(g)`; and
`a*_M(g) · 2^bits ≤ p^4`. -/
def rowOk (r : Row) : Bool :=
  jok r.n r.k (r.n - r.g) r.mu0 r.h0 &&
  decide (2 * r.M * countNum r.n r.k (r.n - r.g) r.mu0 r.h0 / (r.n - r.g - r.k) +
      r.M * (r.n * (r.n - r.g - r.k) / ((r.n - r.g) * (r.n - r.g) - r.n * r.k)) ≤
    aStar r.M r.aX r.aY r.aZ r.g) &&
  decide (aStar r.M r.aX r.aY r.aZ r.g * 2 ^ r.bits ≤ babyBearP ^ 4) &&
  decide (r.g ≤ r.n)

theorem rows_ok : ∀ r ∈ frozenRows, rowOk r = true := by decide +kernel

/-- **Lemma 9.0** (PROOF.md). (H1) to (H6) hold at every frozen row, with `m = 3`,
`p = babyBearP` and `q = p^4`. Integer computation. -/
theorem lemma_9_0 : ∀ r ∈ frozenRows,
    Hyps r.n r.k r.M 3 r.aX r.aY r.aZ r.g babyBearP (babyBearP ^ 4) := by
  decide +kernel

end MCA
