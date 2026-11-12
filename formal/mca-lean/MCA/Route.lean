/-
OD-19 (b), route (a): MCA bounds for Reed-Solomon codes from ArkLib's proved curve theorem.

* `exists_exceptional`: ArkLib's Johnson curve theorem at integer threshold `A`, with the
  exceptional-set size as the closed natural number `2 M · countNum / (A - k)`.
* `curve_card_le`: the bad set of [WHIR, Def. 4.9] for the power curve `Gen(M + 1; z)` over
  `RS_k[F, D]` (degree `≤ k`) is inside that exceptional set. `M = 1` is the fold.
* `vandermonde_mem`: `M + 1` distinct challenges recover each coefficient of a power curve.
* `cor_8_2`: the affine subcode (constrained code) form of PROOF.md Corollary 8.2, with the extra
  term `M · L`, `L = ⌊n (A - k) / (A² - n k)⌋` (ArkLib's pairwise-intersection Johnson count).

No statement takes an exception bound as a hypothesis. The only hypotheses are the code, the
words, and the closed integer conditions `JOk`.
-/
import MCA.Johnson
import ArkLib.Data.Finset.PairwiseIntersection

namespace MCA

open Polynomial Finset ReedSolomon

variable {F : Type} [Field F] [Fintype F] [DecidableEq F]
variable {ι : Type} [Fintype ι] [DecidableEq ι]

/-- ArkLib's Johnson curve theorem
(`ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype`) at integer threshold `A`:
an exceptional set of at most `2 M · countNum / (A - k)` challenges outside which every
candidate of degree `≤ k` with `A` agreements has exact power agreement. -/
theorem exists_exceptional (dom : ι ↪ F) {k A μ0 h0 M : ℕ}
    (h : JOk (Fintype.card ι) k A μ0 h0) (u : Fin (M + 1) → ι → F) :
    ∃ Ex : Finset F, Ex.card ≤ 2 * M * countNum (Fintype.card ι) k A μ0 h0 / (A - k) ∧
      ∀ z ∉ Ex, ∀ P : F[X], P.degree < k + 1 →
        A ≤ (polynomialAgreementSet dom (powerBatchedWord u z) P).card →
        HasExactPowerAgreement dom u (RingHom.id F) (k + 1) z P := by
  obtain ⟨Ex, hcard, hgood⟩ :=
    ProximityGap.exists_johnson_curve_exactPowerAgreement_of_fintype
      (D := k) (A := A) (eta := jEta (Fintype.card ι) k A) dom u h.hk
      (by have := h.hkn; omega) h.eta_pos (le_of_eq h.agree_mul)
  refine ⟨Ex, ?_, hgood⟩
  rw [Nat.le_div_iff_mul_le (by have := h.hkA; omega)]
  have h1 := h.count_mul
  have h2 : (Ex.card : ℝ) * ((A - k : ℕ) : ℝ) ≤
      2 * (M : ℝ) * (countNum (Fintype.card ι) k A μ0 h0 : ℝ) := by
    rw [← h1]
    have := mul_le_mul_of_nonneg_right hcard (Nat.cast_nonneg (α := ℝ) (A - k))
    linarith
  exact_mod_cast h2

/-- Outside the exceptional set, a codeword `c` of `RS_k` that agrees with the batched word on a
set `S` of at least `A` points splits as `c = ∑ z^j P_j` with every `P_j` agreeing with `u_j` on
the whole common agreement set, which contains `S`. -/
theorem exact_of_agree (dom : ι ↪ F) {k A M : ℕ} {u : Fin (M + 1) → ι → F} {Ex : Finset F}
    (hgood : ∀ z ∉ Ex, ∀ P : F[X], P.degree < k + 1 →
        A ≤ (polynomialAgreementSet dom (powerBatchedWord u z) P).card →
        HasExactPowerAgreement dom u (RingHom.id F) (k + 1) z P)
    {z : F} (hz : z ∉ Ex) {S : Finset ι} (hS : A ≤ S.card) {c : ι → F}
    (hc : c ∈ rsCode dom (k + 1)) (hcS : ∀ x ∈ S, c x = combine (powGen F (M + 1)) z u x) :
    ∃ P : Fin (M + 1) → F[X], (∀ j, P j ∈ Polynomial.degreeLT F (k + 1)) ∧
      c = (∑ j : Fin (M + 1), z ^ (j : ℕ) • evalOn dom (P j)) ∧
      S ⊆ commonCurveAgreementSet dom u P ∧ A ≤ (commonCurveAgreementSet dom u P).card := by
  have hc' : c ∈ (Polynomial.degreeLT F (k + 1)).map (evalOn dom) := hc
  obtain ⟨Q, hQ, rfl⟩ := Submodule.mem_map.mp hc'
  have hQdeg : Q.degree < k + 1 := by
    have := Polynomial.mem_degreeLT.mp hQ
    exact_mod_cast this
  have hsub : S ⊆ polynomialAgreementSet dom (powerBatchedWord u z) Q := by
    intro x hx
    rw [mem_polynomialAgreementSet]
    have := hcS x hx
    simpa only [evalOn_apply, combine, powGen, powerBatchedWord] using this
  have hA : A ≤ (polynomialAgreementSet dom (powerBatchedWord u z) Q).card :=
    hS.trans (Finset.card_le_card hsub)
  obtain ⟨P, hP, hQeq, hset⟩ :=
    (hasExactPowerAgreement_id_iff dom u (k + 1) z Q).mp (hgood z hz Q hQdeg hA)
  refine ⟨P, fun j => Polynomial.mem_degreeLT.mpr (hP j), ?_, ?_, ?_⟩
  · funext x
    simp only [evalOn_apply, Finset.sum_apply, Pi.smul_apply, smul_eq_mul]
    rw [hQeq, Polynomial.powerBatch_eval]
  · rw [← hset]
    exact hsub
  · rw [← hset]
    exact hA

/-- A large set at radius `δ` with `⌊δ n⌋ + A ≤ n` has at least `A` points. -/
theorem largeSet_card {δ : ℝ} {S : Finset ι} {A : ℕ}
    (hδ : ⌊δ * Fintype.card ι⌋₊ + A ≤ Fintype.card ι) (hS : LargeSet δ S) : A ≤ S.card := by
  have hle := S.card_le_univ
  have h1 : ((Fintype.card ι - S.card : ℕ) : ℝ) ≤ δ * Fintype.card ι := by
    rw [Nat.cast_sub hle]
    unfold LargeSet at hS
    linarith
  have h2 := Nat.le_floor h1
  omega

/-- **Curve MCA, full code.** For `RS_k[F, D]` (`rsCode dom (k + 1)`), the power curve
`Gen(M + 1; z)` and any words `u_0 .. u_M`, at every radius `δ` with `⌊δ n⌋ + A ≤ n`, the bad
set of [WHIR, Def. 4.9] has at most `2 M · countNum / (A - k)` elements. -/
theorem curve_card_le (dom : ι ↪ F) {k A μ0 h0 M : ℕ} (h : JOk (Fintype.card ι) k A μ0 h0)
    (u : Fin (M + 1) → ι → F) (δ : ℝ) (hδ : ⌊δ * Fintype.card ι⌋₊ + A ≤ Fintype.card ι) :
    (mcaBadSet (rsCode dom (k + 1) : Set (ι → F)) (powGen F (M + 1)) u δ).card ≤
      2 * M * countNum (Fintype.card ι) k A μ0 h0 / (A - k) := by
  classical
  obtain ⟨Ex, hEx, hgood⟩ := exists_exceptional dom h u
  refine le_trans (Finset.card_le_card ?_) hEx
  intro z hz
  simp only [mcaBadSet, Finset.mem_filter, Finset.mem_univ, true_and] at hz
  by_contra hzE
  obtain ⟨S, hS, ⟨c, hc, hcS⟩, i, hi⟩ := hz
  obtain ⟨P, hP, -, hsub, -⟩ :=
    exact_of_agree dom hgood hzE (largeSet_card hδ hS) hc hcS
  apply hi
  refine ⟨evalOn dom (P i), Submodule.mem_map_of_mem (hP i), fun x hx => ?_⟩
  have := (mem_commonCurveAgreementSet dom u P x).mp (hsub hx) i
  simpa only [evalOn_apply] using this

/-- **Vandermonde step.** If `∑_j z^j • x_j ∈ W` for `M + 1` distinct challenges `z`, then every
`x_j ∈ W`. -/
theorem vandermonde_mem {V : Type*} [AddCommGroup V] [Module F V] (W : Submodule F V) {M : ℕ}
    (x : Fin (M + 1) → V) (Z : Finset F) (hZ : Z.card = M + 1)
    (hW : ∀ z ∈ Z, ∑ j : Fin (M + 1), z ^ (j : ℕ) • x j ∈ W) (j : Fin (M + 1)) : x j ∈ W := by
  classical
  let e : Fin (M + 1) ≃ Z := (Fintype.equivFinOfCardEq (by rw [Fintype.card_coe]; exact hZ)).symm
  let a : Fin (M + 1) → F := fun i => (e i : F)
  have ha : Function.Injective a := fun i i' hii => e.injective (Subtype.ext hii)
  let Vm := Matrix.vandermonde a
  have hdet : IsUnit Vm.det := isUnit_iff_ne_zero.mpr (Matrix.det_vandermonde_ne_zero_iff.mpr ha)
  have hrow : ∀ i, ∑ l, Vm i l • x l ∈ W := fun i => hW (e i) (e i).2
  have key : x j = ∑ i, Vm⁻¹ j i • ∑ l, Vm i l • x l := by
    simp only [Finset.smul_sum, smul_smul]
    rw [Finset.sum_comm]
    simp only [← Finset.sum_smul]
    have h1 : ∀ l, ∑ i, Vm⁻¹ j i * Vm i l = (Vm⁻¹ * Vm) j l := fun l => (Matrix.mul_apply).symm
    simp only [h1, Matrix.nonsing_inv_mul _ hdet, Matrix.one_apply, ite_smul, one_smul, zero_smul,
      Finset.sum_ite_eq, Finset.mem_univ, ite_true]
  rw [key]
  exact W.sum_mem fun i _ => W.smul_mem _ (hrow i)

/-- The coset `C''_v := {f ∈ C'' : λ f = v}` of PROOF.md Corollary 8.2. -/
def cosetCode {s : ℕ} (C'' : Submodule F (ι → F)) (lam : (ι → F) →ₗ[F] (Fin s → F))
    (v : Fin s → F) : Set (ι → F) :=
  {f | f ∈ C'' ∧ lam f = v}

/-- Two distinct polynomials of degree `≤ k` agree with the same words on at most `k` common
points: distinct tuples have common agreement sets meeting in at most `k` points. -/
theorem inter_card_le (dom : ι ↪ F) {k M : ℕ} (u : Fin (M + 1) → ι → F)
    (t t' : Fin (M + 1) → F[X]) (ht : ∀ j, t j ∈ Polynomial.degreeLT F (k + 1))
    (ht' : ∀ j, t' j ∈ Polynomial.degreeLT F (k + 1)) (hne : t ≠ t') :
    (commonCurveAgreementSet dom u t ∩ commonCurveAgreementSet dom u t').card ≤ k := by
  obtain ⟨j, hj⟩ := Function.ne_iff.mp hne
  by_contra hc
  push_neg at hc
  apply hj
  set X := commonCurveAgreementSet dom u t ∩ commonCurveAgreementSet dom u t'
  have hX : ((k + 1 : ℕ) : WithBot ℕ) ≤ (X.card : WithBot ℕ) := by exact_mod_cast hc
  refine Polynomial.eq_of_degrees_lt_of_eval_index_eq X (dom.injective.injOn)
    ((Polynomial.mem_degreeLT.mp (ht j)).trans_le hX)
    ((Polynomial.mem_degreeLT.mp (ht' j)).trans_le hX) fun i hi => ?_
  rw [Finset.mem_inter, mem_commonCurveAgreementSet, mem_commonCurveAgreementSet] at hi
  rw [hi.1 j, hi.2 j]

open Classical in
/-- **Corollary 8.2, route (a)** (affine subcodes; SWIRL Protocol 3.7.1 constrained code). Let
`C'' ⊆ RS_k[F, D]` be a subspace, `λ` a linear map and `v_0 .. v_M` targets. If no set of at least
`n - g` points has every `u_j` in `C''_{v_j}` on it, then at most
`2 M · countNum / (A - k) + M · ⌊n (A - k) / (A² - n k)⌋` challenges `z` put the batched word
`u_z = ∑ z^j u_j` in `C''_{σ(z)}`, `σ(z) = ∑ z^j v_j`, on some set of at least `n - g` points. -/
theorem cor_8_2 (dom : ι ↪ F) {k A μ0 h0 M g s : ℕ} (h : JOk (Fintype.card ι) k A μ0 h0)
    (hgA : g + A ≤ Fintype.card ι)
    (C'' : Submodule F (ι → F)) (hC'' : C'' ≤ rsCode dom (k + 1))
    (lam : (ι → F) →ₗ[F] (Fin s → F)) (v : Fin (M + 1) → Fin s → F)
    (u : Fin (M + 1) → ι → F)
    (hfar : ¬ ∃ S : Finset ι, Fintype.card ι - g ≤ S.card ∧
      ∀ j, AgreesOn (cosetCode C'' lam (v j)) (u j) S) :
    ((Finset.univ.filter fun z : F => ∃ S : Finset ι, Fintype.card ι - g ≤ S.card ∧
        AgreesOn (cosetCode C'' lam (∑ j : Fin (M + 1), z ^ (j : ℕ) • v j))
          (combine (powGen F (M + 1)) z u) S).card) ≤
      2 * M * countNum (Fintype.card ι) k A μ0 h0 / (A - k) +
        M * (Fintype.card ι * (A - k) / (A * A - Fintype.card ι * k)) := by
  classical
  obtain ⟨Ex, hEx, hgood⟩ := exists_exceptional dom h u
  set B := Finset.univ.filter fun z : F => ∃ S : Finset ι, Fintype.card ι - g ≤ S.card ∧
      AgreesOn (cosetCode C'' lam (∑ j : Fin (M + 1), z ^ (j : ℕ) • v j))
        (combine (powGen F (M + 1)) z u) S with hB
  set B' := B.filter fun z => z ∉ Ex with hB'
  have hsplit : B ⊆ Ex ∪ B' := by
    intro z hz
    by_cases hzE : z ∈ Ex
    · exact Finset.mem_union_left _ hzE
    · exact Finset.mem_union_right _ (Finset.mem_filter.mpr ⟨hz, hzE⟩)
  refine (Finset.card_le_card hsplit).trans ((Finset.card_union_le _ _).trans
    (add_le_add hEx ?_))
  have key : ∀ z ∈ B', ∃ P : Fin (M + 1) → F[X], (∀ j, P j ∈ Polynomial.degreeLT F (k + 1)) ∧
      (∑ j : Fin (M + 1), z ^ (j : ℕ) • evalOn dom (P j)) ∈ C'' ∧
      lam (∑ j : Fin (M + 1), z ^ (j : ℕ) • evalOn dom (P j)) = ∑ j : Fin (M + 1), z ^ (j : ℕ) • v j ∧
      (∃ S : Finset ι, Fintype.card ι - g ≤ S.card ∧ S ⊆ commonCurveAgreementSet dom u P) ∧
      A ≤ (commonCurveAgreementSet dom u P).card := by
    intro z hz
    simp only [hB', hB, Finset.mem_filter, Finset.mem_univ, true_and] at hz
    obtain ⟨⟨S, hS, c, ⟨hcC, hclam⟩, hcS⟩, hzE⟩ := hz
    obtain ⟨P, hP, hcP, hsub, hA⟩ :=
      exact_of_agree dom hgood hzE (by omega) (hC'' hcC) hcS
    rw [hcP] at hcC hclam
    exact ⟨P, hP, hcC, hclam, ⟨S, hS, hsub⟩, hA⟩
  choose! tup htup using key
  -- Each tuple serves at most `M` challenges (Vandermonde and `hfar`).
  have hfib : ∀ t ∈ B'.image tup, (B'.filter fun z => tup z = t).card ≤ M := by
    intro t _
    by_contra hlt
    push_neg at hlt
    obtain ⟨Z, hZsub, hZcard⟩ := Finset.exists_subset_card_eq (show M + 1 ≤ _ from hlt)
    have hZ : ∀ z ∈ Z, z ∈ B' ∧ tup z = t := fun z hz => Finset.mem_filter.mp (hZsub hz)
    have hmem : ∀ j, evalOn dom (t j) ∈ C'' := by
      refine vandermonde_mem C'' (fun j => evalOn dom (t j)) Z hZcard fun z hz => ?_
      obtain ⟨hzB, hzt⟩ := hZ z hz
      have := (htup z hzB).2.1
      rw [hzt] at this
      exact this
    have hlam : ∀ j, lam (evalOn dom (t j)) = v j := by
      have hv := vandermonde_mem (⊥ : Submodule F (Fin s → F))
        (fun j => lam (evalOn dom (t j)) - v j) Z hZcard fun z hz => by
          obtain ⟨hzB, hzt⟩ := hZ z hz
          have h2 := (htup z hzB).2.2.1
          rw [hzt] at h2
          rw [Submodule.mem_bot]
          simp only [smul_sub, Finset.sum_sub_distrib]
          rw [map_sum] at h2
          simp only [map_smul] at h2
          rw [h2, sub_self]
      intro j
      exact sub_eq_zero.mp ((Submodule.mem_bot F).mp (hv j))
    obtain ⟨z0, hz0⟩ : Z.Nonempty := by
      rw [← Finset.card_pos, hZcard]
      omega
    obtain ⟨hz0B, hz0t⟩ := hZ z0 hz0
    obtain ⟨S, hS, hsub⟩ := (htup z0 hz0B).2.2.2.1
    rw [hz0t] at hsub
    refine hfar ⟨S, hS, fun j => ⟨evalOn dom (t j), ⟨hmem j, hlam j⟩, fun x hx => ?_⟩⟩
    have := (mem_commonCurveAgreementSet dom u t x).mp (hsub hx) j
    simpa only [evalOn_apply] using this
  -- The tuples form a family of large sets with small pairwise intersections.
  have hN : (B'.image tup).card ≤
      Fintype.card ι * (A - k) / (A * A - Fintype.card ι * k) := by
    have hpos : 0 < A * A - Fintype.card ι * k := by have := h.nk_lt; omega
    rw [Nat.le_div_iff_mul_le hpos]
    refine Finset.card_mul_sq_sub_card_mul_le_of_inter_card_le (B'.image tup)
      (fun t => commonCurveAgreementSet dom u t) A k ?_ ?_
    · intro t ht
      obtain ⟨z, hz, rfl⟩ := Finset.mem_image.mp ht
      exact (htup z hz).2.2.2.2
    · intro t ht t' ht' hne
      obtain ⟨z, hz, rfl⟩ := Finset.mem_image.mp ht
      obtain ⟨z', hz', rfl⟩ := Finset.mem_image.mp ht'
      exact inter_card_le dom u _ _ (htup z hz).1 (htup z' hz').1 hne
  calc B'.card ≤ M * (B'.image tup).card := Finset.card_le_mul_card_image B' M hfib
    _ ≤ M * (Fintype.card ι * (A - k) / (A * A - Fintype.card ι * k)) :=
      Nat.mul_le_mul_left M hN

end MCA
