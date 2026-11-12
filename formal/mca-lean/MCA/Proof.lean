/-
OD-19 (b): the statements of `research/sherlock-mca-proof-20261111/PROOF.md`, the Lean target.

* Theorem 7.1: `|E| ≤ a*_M` for every linear subcode `C' ⊆ F[X]_{≤k}` and curve degree `M ≥ 1`,
  under the integer hypotheses (H1) to (H6).
* Corollary 8.2: the affine-subcode (constrained code) form used by mu batching.
* Lemma 9.0: (H1) to (H6) at the 26 frozen rows (24 folds with `M = 1`, the `L` and internal mu
  rows), by integer computation.
* Lemma 9.0b, Definition 9.1b, Corollary 9.2: the error function over the whole radius interval
  for the 30 WHIR codes (Astra review fix).

Phase 1 leaves the proofs as `sorry`. Theorem 7.1 is the hard lemma. It is never a hypothesis.
PROOF.md degree convention: `k` is the degree bound (`deg ≤ k`), so the code is `rsCode dom (k + 1)`.
-/
import MCA.Defs

namespace MCA

open Finset

variable {F : Type} [Field F] [Fintype F] [DecidableEq F]
variable {ι : Type} [Fintype ι] [DecidableEq ι]

/-- The bad set `E` of PROOF.md section 1 for the curve `u_0 + Z u_1 + ... + Z^M u_M`, with the
size condition `|S| ≥ n - g`. -/
noncomputable def badSetG (C : Set (ι → F)) (M : ℕ) (u : Fin (M + 1) → ι → F) (g : ℕ) :
    Finset F := by
  classical
  exact Finset.univ.filter fun z => ∃ S : Finset ι, Fintype.card ι - g ≤ S.card ∧
    AgreesOn C (combine (powGen F (M + 1)) z u) S ∧ ∃ j, ¬ AgreesOn C (u j) S

/-- The bound `a*_M := (a_Z - 1)(1 + 4 N (a_Y - 1)) + M (a_Y - 1)(g + 1)`, `N = a_X - 1`. -/
def aStar (M aX aY aZ g : ℕ) : ℕ := (aZ - 1) * (1 + 4 * (aX - 1) * (aY - 1)) + M * (aY - 1) * (g + 1)

/-- The hypotheses (H1) to (H6) of PROOF.md section 1, over the integers. `p` is the
characteristic and `q = |F|`. -/
def Hyps (n k M m aX aY aZ g p q : ℕ) : Prop :=
  let N : ℤ := (aX : ℤ) - 1
  let A0 : ℤ := (n : ℤ) - g
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
  (A0 > k) ∧
  -- (H6)
  ((2 * (k : ℤ) - 1 + M) * ((n : ℤ) - k) ≤ (2 * N - 1) * (A0 - k))

/-- **Theorem 7.1** (PROOF.md). For every linear subcode `C'` of `RS_k[F, D]` (degree `≤ k`),
curve degree `M ≥ 1` and words `u_0 .. u_M`, under (H1) to (H6), `|E| ≤ a*_M`. -/
theorem thm_7_1 (dom : ι ↪ F) (k M m aX aY aZ g : ℕ)
    (hk : 1 ≤ k) (hkn : k < Fintype.card ι) (hg : g < Fintype.card ι) (hM : 1 ≤ M) (hm : 1 ≤ m)
    (haX : 1 ≤ aX) (haY : 1 ≤ aY) (haZ : 1 ≤ aZ)
    (C' : Submodule F (ι → F)) (hC' : C' ≤ rsCode dom (k + 1))
    (H : Hyps (Fintype.card ι) k M m aX aY aZ g (ringChar F) (Fintype.card F))
    (u : Fin (M + 1) → ι → F) :
    (badSetG (C' : Set (ι → F)) M u g).card ≤ aStar M aX aY aZ g := by
  sorry

/-- The coset `C''_v := {f ∈ C'' : λ f = v}` of PROOF.md Corollary 8.2. -/
def cosetCode {s : ℕ} (C'' : Submodule F (ι → F)) (lam : (ι → F) →ₗ[F] (Fin s → F))
    (v : Fin s → F) : Set (ι → F) :=
  {f | f ∈ C'' ∧ lam f = v}

open Classical in
/-- **Corollary 8.2** (PROOF.md, affine subcodes; SWIRL Protocol 3.7.1 constrained code). If no
large `S` has every `u_j` in `C''_{v_j}` on `S`, then at most `a*_M` challenges `z` put the
batched word `u_z` in `C''_{σ(z)}` on some large `S`, where `σ(z) = ∑_j z^j v_j`. -/
theorem cor_8_2 (dom : ι ↪ F) (k M m aX aY aZ g s : ℕ)
    (hk : 1 ≤ k) (hkn : k < Fintype.card ι) (hg : g < Fintype.card ι) (hM : 1 ≤ M) (hm : 1 ≤ m)
    (haX : 1 ≤ aX) (haY : 1 ≤ aY) (haZ : 1 ≤ aZ)
    (C'' : Submodule F (ι → F)) (hC'' : C'' ≤ rsCode dom (k + 1))
    (lam : (ι → F) →ₗ[F] (Fin s → F)) (v : Fin (M + 1) → Fin s → F)
    (H : Hyps (Fintype.card ι) k M m aX aY aZ g (ringChar F) (Fintype.card F))
    (u : Fin (M + 1) → ι → F)
    (hfar : ¬ ∃ S : Finset ι, Fintype.card ι - g ≤ S.card ∧
      ∀ j, AgreesOn (cosetCode C'' lam (v j)) (u j) S) :
    ((Finset.univ.filter fun z : F => ∃ S : Finset ι, Fintype.card ι - g ≤ S.card ∧
        AgreesOn (cosetCode C'' lam (∑ j : Fin (M + 1), z ^ (j : ℕ) • v j))
          (combine (powGen F (M + 1)) z u) S).card) ≤ aStar M aX aY aZ g := by
  sorry

/-- One frozen row: `(n, k, M, a_X, a_Y, a_Z, g)` with `m = 3`. -/
structure Row where
  n : ℕ
  k : ℕ
  M : ℕ
  aX : ℕ
  aY : ℕ
  aZ : ℕ
  g : ℕ

/-- BabyBear, `p = 2^31 - 2^27 + 1`; the challenge field is `BabyBear^4`, `q = p^4`. -/
def babyBearP : ℕ := 2013265921

/-- The 26 frozen rows of STATEMENT.md section 7, computed by `bits.py` (`ints`): 12 `L` folds,
12 internal folds (`M = 1`), the `L` mu row (`M = 47`) and the internal mu row (`M = 511`). -/
def frozenRows : List Row := [
    ⟨8388608, 524287, 1, 7340025, 15, 66, 5592405⟩,  -- mu=19 beta=4
    ⟨4194304, 262143, 1, 3670009, 15, 66, 2796202⟩,  -- mu=18 beta=4
    ⟨2097152, 131071, 1, 1835001, 15, 66, 1398101⟩,  -- mu=17 beta=4
    ⟨1048576, 65535, 1, 917497, 15, 66, 699050⟩,  -- mu=16 beta=4
    ⟨4194304, 32767, 1, 1297527, 40, 523, 3700000⟩,  -- mu=15 beta=7
    ⟨2097152, 16383, 1, 648754, 40, 523, 1850000⟩,  -- mu=14 beta=7
    ⟨1048576, 8191, 1, 324367, 40, 523, 925000⟩,  -- mu=13 beta=7
    ⟨524288, 4095, 1, 162174, 40, 523, 462500⟩,  -- mu=12 beta=7
    ⟨2097152, 2047, 1, 229320, 113, 4184, 2009770⟩,  -- mu=11 beta=10
    ⟨1048576, 1023, 1, 114632, 113, 4186, 1004885⟩,  -- mu=10 beta=10
    ⟨524288, 511, 1, 57288, 113, 4190, 502442⟩,  -- mu=9 beta=10
    ⟨262144, 255, 1, 28616, 113, 4198, 251221⟩,  -- mu=8 beta=10
    ⟨2097152, 262143, 1, 2595089, 10, 33, 1108545⟩,  -- mu=18 beta=3
    ⟨1048576, 131071, 1, 1297542, 10, 33, 554272⟩,  -- mu=17 beta=3
    ⟨524288, 65535, 1, 648769, 10, 33, 277136⟩,  -- mu=16 beta=3
    ⟨262144, 32767, 1, 324382, 10, 33, 138568⟩,  -- mu=15 beta=3
    ⟨1048576, 16383, 1, 458738, 29, 262, 873813⟩,  -- mu=14 beta=6
    ⟨524288, 8191, 1, 229362, 29, 262, 436906⟩,  -- mu=13 beta=6
    ⟨262144, 4095, 1, 114674, 29, 262, 218453⟩,  -- mu=12 beta=6
    ⟨131072, 2047, 1, 57330, 29, 262, 109226⟩,  -- mu=11 beta=6
    ⟨524288, 1023, 1, 81058, 80, 2093, 493394⟩,  -- mu=10 beta=9
    ⟨262144, 511, 1, 40509, 80, 2095, 246697⟩,  -- mu=9 beta=9
    ⟨131072, 255, 1, 20235, 80, 2099, 123348⟩,  -- mu=8 beta=9
    ⟨65536, 127, 1, 10098, 80, 2108, 61674⟩,  -- mu=7 beta=9
    ⟨16777216, 1048575, 47, 14680057, 15, 3071, 11184810⟩,  -- mu=20 beta=4
    ⟨4194304, 524287, 511, 5190182, 10, 16693, 2217090⟩   -- mu=19 beta=3
  ]

/-- One WHIR code `C^(i,s) = RS_k[F, D]` of PROOF.md Lemma 9.0b at `M = 1`:
`(n, k, a_X, a_Y, a_Z, g_end)` with `g_end = n - ⌈√(49 k n / 36)⌉ = ⌊(1 - (7/6)√(k/n)) n⌋`. -/
structure WhirCode where
  n : ℕ
  k : ℕ
  aX : ℕ
  aY : ℕ
  aZ : ℕ
  gEnd : ℕ

/-- The 30 codes of PROOF.md Lemma 9.0b (`0 ≤ s ≤ 4` in each of the three WHIR rounds of `L` and
of the internal layer), computed with the exact integer arithmetic of `bits.py`. -/
def whirCodes : List WhirCode := [
    ⟨16777216, 1048575, 14680057, 15, 66, 11883863⟩,  -- mu=20 beta=4
    ⟨8388608, 524287, 7340025, 15, 66, 5941933⟩,  -- mu=19 beta=4
    ⟨4194304, 262143, 3670009, 15, 66, 2970967⟩,  -- mu=18 beta=4
    ⟨2097152, 131071, 1835001, 15, 66, 1485485⟩,  -- mu=17 beta=4
    ⟨1048576, 65535, 917497, 15, 66, 742743⟩,  -- mu=16 beta=4
    ⟨8388608, 65535, 2595074, 40, 523, 7523583⟩,  -- mu=16 beta=7
    ⟨4194304, 32767, 1297527, 40, 523, 3761795⟩,  -- mu=15 beta=7
    ⟨2097152, 16383, 648754, 40, 523, 1880900⟩,  -- mu=14 beta=7
    ⟨1048576, 8191, 324367, 40, 523, 940453⟩,  -- mu=13 beta=7
    ⟨524288, 4095, 162174, 40, 523, 470230⟩,  -- mu=12 beta=7
    ⟨4194304, 4095, 458696, 113, 4183, 4041405⟩,  -- mu=12 beta=10
    ⟨2097152, 2047, 229320, 113, 4184, 2020712⟩,  -- mu=11 beta=10
    ⟨1048576, 1023, 114632, 113, 4186, 1010365⟩,  -- mu=10 beta=10
    ⟨524288, 511, 57288, 113, 4190, 505192⟩,  -- mu=9 beta=10
    ⟨262144, 255, 28616, 113, 4198, 252605⟩,  -- mu=8 beta=10
    ⟨4194304, 524287, 5190182, 10, 33, 2464243⟩,  -- mu=19 beta=3
    ⟨2097152, 262143, 2595089, 10, 33, 1232122⟩,  -- mu=18 beta=3
    ⟨1048576, 131071, 1297542, 10, 33, 616062⟩,  -- mu=17 beta=3
    ⟨524288, 65535, 648769, 10, 33, 308031⟩,  -- mu=16 beta=3
    ⟨262144, 32767, 324382, 10, 33, 154016⟩,  -- mu=15 beta=3
    ⟨2097152, 32767, 917490, 29, 262, 1791322⟩,  -- mu=15 beta=6
    ⟨1048576, 16383, 458738, 29, 262, 895663⟩,  -- mu=14 beta=6
    ⟨524288, 8191, 229362, 29, 262, 447834⟩,  -- mu=13 beta=6
    ⟨262144, 4095, 114674, 29, 262, 223919⟩,  -- mu=12 beta=6
    ⟨131072, 2047, 57330, 29, 262, 111962⟩,  -- mu=11 beta=6
    ⟨1048576, 2047, 162154, 80, 2092, 994524⟩,  -- mu=11 beta=9
    ⟨524288, 1023, 81058, 80, 2093, 497268⟩,  -- mu=10 beta=9
    ⟨262144, 511, 40509, 80, 2095, 248641⟩,  -- mu=9 beta=9
    ⟨131072, 255, 20235, 80, 2099, 124327⟩,  -- mu=8 beta=9
    ⟨65536, 127, 10098, 80, 2108, 62170⟩   -- mu=7 beta=9
  ]

/-- **Lemma 9.0b** (PROOF.md). For every code of `whirCodes` and every `g ≤ g_end`, (H1) to (H6)
hold with `M = 1`, `m = 3`, `p = babyBearP`, `q = p^4`. -/
theorem lemma_9_0b : ∀ c ∈ whirCodes, ∀ g ≤ c.gEnd,
    Hyps c.n c.k 1 3 c.aX c.aY c.aZ g babyBearP (babyBearP ^ 4) := by
  sorry

/-- **Definition 9.1b** (PROOF.md). The WHIR (H2) error function of code `c` on the whole radius
interval: `a*_1(⌊δ n⌋) / q` if `⌊δ n⌋ ≤ g_end`, and `1` otherwise. -/
noncomputable def errStar (c : WhirCode) (q : ℕ) (δ : ℝ) : ℝ :=
  if ⌊δ * c.n⌋₊ ≤ c.gEnd then (aStar 1 c.aX c.aY c.aZ ⌊δ * c.n⌋₊ : ℝ) / q else 1

/-- **Corollary 9.2** (PROOF.md, statement (i)). For every code `c` of `whirCodes` over
`F = BabyBear^4` and every `δ ∈ [0, 1)`, the fold generator `Gen(2; α)` has MCA for
`RS_k[F, D]` with error `errStar c |F| δ` (WHIR Theorem 5.2 hypothesis H2). -/
theorem cor_9_2 (c : WhirCode) (hc : c ∈ whirCodes) (dom : ι ↪ F)
    (hn : Fintype.card ι = c.n) (hp : ringChar F = babyBearP)
    (hq : Fintype.card F = babyBearP ^ 4) (δ : ℝ) (hδ0 : 0 ≤ δ) (hδ1 : δ < 1) :
    HasMCA (rsCode dom (c.k + 1) : Set (ι → F)) (foldGen F) δ
      (errStar c (Fintype.card F) δ) := by
  sorry

/-- **Lemma 9.0** (PROOF.md). (H1) to (H6) hold at every frozen row, with `m = 3`,
`p = babyBearP` and `q = p^4`. Integer computation. -/
theorem lemma_9_0 : ∀ r ∈ frozenRows,
    Hyps r.n r.k r.M 3 r.aX r.aY r.aZ r.g babyBearP (babyBearP ^ 4) := by
  sorry

end MCA
