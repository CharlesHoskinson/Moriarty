/-
OD-19 (b), route (a): the bit bounds at the frozen parameters (STATEMENT.md section 7).

For each of the 26 frozen rows (24 folds with `M = 1`, the `L` mu row `M = 47` and the internal
mu row `M = 511`), over any field with `|F| = p^4` (`p` BabyBear) and any domain of size `n`:

* `row_bits`: the MCA error of `Gen(M + 1; z)` for `RS_k[F, D]` is at most `2^-bits` at every
  radius `δ` with `⌊δ n⌋ ≤ g`;
* `row_mcaError_bits`: the same bound on ArkLib's `mcaError` (uniform seed);
* `row_subcode_bits`: the affine subcode form (Corollary 8.2, the constrained code of mu batching)
  at size `n - g`.

`bits` is the field in `frozenRows`. The smallest is 84 for the folds and 82 for both mu rows,
before proof of work.
-/
import MCA.Proof
import MCA.Bridge

namespace MCA

open Finset

variable {F : Type} [Field F] [Fintype F] [DecidableEq F]
variable {ι : Type} [Fintype ι] [DecidableEq ι]

/-- `N ≤ a` and `a · 2^b ≤ q` give `N / q ≤ 2^-b`. -/
theorem div_le_two_zpow {N a b q : ℕ} (hN : N ≤ a) (hq : 0 < q) (hab : a * 2 ^ b ≤ q) :
    (N : ℝ) / q ≤ (2 : ℝ) ^ (-(b : ℤ)) := by
  have hq' : (0 : ℝ) < q := by exact_mod_cast hq
  rw [zpow_neg, zpow_natCast, div_le_iff₀ hq', ← div_eq_inv_mul, le_div_iff₀ (by positivity)]
  have : N * 2 ^ b ≤ q := (Nat.mul_le_mul_right _ hN).trans hab
  exact_mod_cast this

theorem rowOk_spec {r : Row} (hr : r ∈ frozenRows) :
    JOk r.n r.k (r.n - r.g) r.mu0 r.h0 ∧
    2 * r.M * countNum r.n r.k (r.n - r.g) r.mu0 r.h0 / (r.n - r.g - r.k) +
      r.M * (r.n * (r.n - r.g - r.k) / ((r.n - r.g) * (r.n - r.g) - r.n * r.k)) ≤
      aStar r.M r.aX r.aY r.aZ r.g ∧
    aStar r.M r.aX r.aY r.aZ r.g * 2 ^ r.bits ≤ babyBearP ^ 4 ∧ r.g ≤ r.n := by
  have hok := rows_ok r hr
  simp only [rowOk, Bool.and_eq_true, decide_eq_true_eq] at hok
  obtain ⟨⟨⟨hj, hb⟩, hbits⟩, hg⟩ := hok
  exact ⟨JOk.of_jok hj, hb, hbits, hg⟩

/-- **Bit bound, full code.** At every frozen row, the MCA error of `Gen(M + 1; z)` for
`RS_k[F, D]` is at most `2^-bits` at every radius `δ` with `⌊δ n⌋ ≤ g`. -/
theorem row_bits (r : Row) (hr : r ∈ frozenRows) (dom : ι ↪ F) (hn : Fintype.card ι = r.n)
    (hq : Fintype.card F = babyBearP ^ 4) (δ : ℝ) (hδ : ⌊δ * r.n⌋₊ ≤ r.g)
    (u : Fin (r.M + 1) → ι → F) :
    mcaErr (rsCode dom (r.k + 1) : Set (ι → F)) (powGen F (r.M + 1)) u δ ≤
      (2 : ℝ) ^ (-(r.bits : ℤ)) := by
  obtain ⟨hJ, hb, hbits, hg⟩ := rowOk_spec hr
  rw [← hn] at hJ
  have h1 := curve_card_le dom hJ u δ (by rw [hn]; omega)
  rw [hn] at h1
  unfold mcaErr
  rw [hq]
  exact div_le_two_zpow (h1.trans ((Nat.le_add_right _ _).trans hb))
    (by unfold babyBearP; norm_num) hbits

/-- **Bit bound, ArkLib form.** The same bound on ArkLib's `mcaError` of
`univariatePowersGenerator F M` for `ReedSolomon.code dom (k + 1)`, for a uniform seed. -/
theorem row_mcaError_bits [SampleableType F] (r : Row) (hr : r ∈ frozenRows) (dom : ι ↪ F)
    (hn : Fintype.card ι = r.n) (hq : Fintype.card F = babyBearP ^ 4) (δ : ℝ)
    (hδ : ⌊δ * r.n⌋₊ ≤ r.g) :
    CoreDefinitions.mcaError (CoreDefinitions.univariatePowersGenerator F r.M)
        (ReedSolomon.code dom (r.k + 1)) δ ≤
      ENNReal.ofReal ((2 : ℝ) ^ (-(r.bits : ℤ))) := by
  have h := (hasMCA_iff_mcaError_le (rsCode dom (r.k + 1)) (powGen F (r.M + 1)) δ _
    (by positivity)).mp fun u => row_bits r hr dom hn hq δ hδ u
  rw [rsCode_eq_arklib] at h
  exact h

open Classical in
/-- **Bit bound, affine subcode** (Corollary 8.2 at the frozen rows; the constrained code of mu
batching). For a subspace `C'' ⊆ RS_k[F, D]`, a linear map `λ` and targets `v_j`, if no set of
`n - g` points has every `u_j` in `C''_{v_j}` on it, the fraction of challenges `z` that put
`u_z` in `C''_{σ(z)}` on some set of `n - g` points is at most `2^-bits`. -/
theorem row_subcode_bits (r : Row) (hr : r ∈ frozenRows) (dom : ι ↪ F)
    (hn : Fintype.card ι = r.n) (hq : Fintype.card F = babyBearP ^ 4) {s : ℕ}
    (C'' : Submodule F (ι → F)) (hC'' : C'' ≤ rsCode dom (r.k + 1))
    (lam : (ι → F) →ₗ[F] (Fin s → F)) (v : Fin (r.M + 1) → Fin s → F)
    (u : Fin (r.M + 1) → ι → F)
    (hfar : ¬ ∃ S : Finset ι, Fintype.card ι - r.g ≤ S.card ∧
      ∀ j, AgreesOn (cosetCode C'' lam (v j)) (u j) S) :
    (((Finset.univ.filter fun z : F => ∃ S : Finset ι, Fintype.card ι - r.g ≤ S.card ∧
        AgreesOn (cosetCode C'' lam (∑ j : Fin (r.M + 1), z ^ (j : ℕ) • v j))
          (combine (powGen F (r.M + 1)) z u) S).card : ℝ) / Fintype.card F) ≤
      (2 : ℝ) ^ (-(r.bits : ℤ)) := by
  obtain ⟨hJ, hb, hbits, hg⟩ := rowOk_spec hr
  rw [← hn] at hJ
  have h1 := cor_8_2 dom hJ (by rw [hn]; omega) C'' hC'' lam v u hfar
  rw [← hn] at hb
  rw [hq]
  exact div_le_two_zpow (h1.trans hb) (by unfold babyBearP; norm_num) hbits

end MCA
