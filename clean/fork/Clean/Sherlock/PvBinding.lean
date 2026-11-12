import Init

/-!
Lemma 23 of the bridge plan (`CANDIDATE.md` section 11.5): the agreement count of two distinct
multilinear tables (Thaler, Lemma 1, in table form). It documents the `c m / |K|` term of the
`eq` binding of `pv` inside premise A2 and is off the trust path of `bridge_sound`.

Over `K = Z/q` with `q` prime: if two tables `T`, `U` on `{0,1}^m` differ somewhere, their
multilinear extensions agree on at most `m q^(m-1)` of the `q^m` points of `K^m`, a fraction of
at most `m / q`. The only field fact used is Euclid's property of `q` (`EuclidMod`), proved here
from primality (`euclid_of_prime`). The instance at BabyBear is `pv_binding_count_babybear`, with
primality of `p` checked by the kernel (`pBB_prime`). For `L` the binding challenge lies in the
extension field `EF`; the same induction applies over any field, and this file proves the prime
field case.
-/

set_option linter.unusedSectionVars false

namespace Clean.Sherlock.Bridge.PvBinding

/-- Multilinear extension mod `q` of a table `T` (indexed by bit lists, first variable first) at
a point with coordinates in `0..q-1`. -/
def mle (q : Nat) (T : List Bool → Int) : List Nat → Int
  | [] => T [] % q
  | x :: xs => (mle q (fun bs => T (false :: bs)) xs * (1 - (x : Int)) +
      mle q (fun bs => T (true :: bs)) xs * x) % q

/-- All points of `(Z/q)^m`. -/
def points (q : Nat) : Nat → List (List Nat)
  | 0 => [[]]
  | m + 1 => (points q m).flatMap (fun xs => (List.range q).map (· :: xs))

/-- The two extensions agree at the point. -/
def agree (q : Nat) (T U : List Bool → Int) (xs : List Nat) : Bool := mle q T xs == mle q U xs

/-- Euclid's property of `q`: the property of a prime that the count uses. -/
def EuclidMod (q : Nat) : Prop := ∀ u v : Int, (q : Int) ∣ u * v → (q : Int) ∣ u ∨ (q : Int) ∣ v

/-- Primality on `Nat`. -/
def IsPrimeNat (q : Nat) : Prop := 2 ≤ q ∧ ∀ d, d ∣ q → d = 1 ∨ d = q

theorem euclid_of_prime {q : Nat} (hq : IsPrimeNat q) : EuclidMod q := by
  intro u v h
  have h' : q ∣ u.natAbs * v.natAbs := by
    rw [← Int.natAbs_mul]
    have := Int.natAbs_dvd_natAbs.mpr h
    simpa using this
  rcases hq.2 (Nat.gcd q u.natAbs) (Nat.gcd_dvd_left _ _) with hg | hg
  · right
    have : q ∣ v.natAbs := Nat.Coprime.dvd_of_dvd_mul_left hg h'
    exact Int.natAbs_dvd_natAbs.mp (by simpa using this)
  · left
    have : q ∣ u.natAbs := hg ▸ Nat.gcd_dvd_right q u.natAbs
    exact Int.natAbs_dvd_natAbs.mp (by simpa using this)

theorem mle_mod (q : Nat) (T : List Bool → Int) : ∀ xs, mle q T xs % q = mle q T xs
  | [] => Int.emod_emod _ _
  | _ :: _ => Int.emod_emod _ _

theorem points_length (q : Nat) : ∀ m, (points q m).length = q ^ m
  | 0 => rfl
  | m + 1 => by
    have h : ∀ l : List (List Nat), (l.flatMap (fun xs => (List.range q).map (· :: xs))).length =
        q * l.length := by
      intro l
      induction l with
      | nil => simp
      | cons a l ih => simp [List.flatMap_cons, ih, Nat.mul_succ]; omega
    rw [points, h, points_length q m, Nat.pow_succ, Nat.mul_comm]

theorem countP_le_one {α : Type} {P : α → Bool} : ∀ {l : List α}, l.Nodup →
    (∀ x ∈ l, ∀ y ∈ l, P x = true → P y = true → x = y) → l.countP P ≤ 1
  | [], _, _ => by simp
  | a :: l, hn, h => by
    rw [List.countP_cons]
    have hn' := List.nodup_cons.mp hn
    by_cases ha : P a = true
    · have : l.countP P = 0 := by
        rw [List.countP_eq_zero]
        intro y hy hpy
        have := h a (by simp) y (by simp [hy]) ha hpy
        exact hn'.1 (this ▸ hy)
      simp [ha, this]
    · have := countP_le_one hn'.2 (fun x hx y hy => h x (by simp [hx]) y (by simp [hy]))
      simp [ha]; omega

/-- A nonzero affine function `t ↦ a (1 - t) + b t` mod a prime has at most one root in `0..q-1`. -/
theorem line_roots {q : Nat} (he : EuclidMod q) (a b c d : Int)
    (hne : ¬ (a % q = c % q ∧ b % q = d % q)) :
    (List.range q).countP (fun x : Nat => (a * (1 - (x : Int)) + b * x) % q ==
      (c * (1 - (x : Int)) + d * x) % q) ≤ 1 := by
  apply countP_le_one List.nodup_range
  intro x hx y hy hpx hpy
  simp only [beq_iff_eq] at hpx hpy
  have hxl := List.mem_range.mp hx
  have hyl := List.mem_range.mp hy
  -- D(t) = (a - c) + t ((b - d) - (a - c))
  have dx : (q : Int) ∣ (a - c) + x * ((b - d) - (a - c)) := by
    have := Int.dvd_of_emod_eq_zero (Int.emod_eq_emod_iff_emod_sub_eq_zero.mp hpx)
    have e : a * (1 - (x : Int)) + b * x - (c * (1 - (x : Int)) + d * x) =
        (a - c) + x * ((b - d) - (a - c)) := by grind
    rwa [e] at this
  have dy : (q : Int) ∣ (a - c) + y * ((b - d) - (a - c)) := by
    have := Int.dvd_of_emod_eq_zero (Int.emod_eq_emod_iff_emod_sub_eq_zero.mp hpy)
    have e : a * (1 - (y : Int)) + b * y - (c * (1 - (y : Int)) + d * y) =
        (a - c) + y * ((b - d) - (a - c)) := by grind
    rwa [e] at this
  have dxy : (q : Int) ∣ ((x : Int) - y) * ((b - d) - (a - c)) := by
    have := Int.dvd_sub dx dy
    have e : (a - c) + x * ((b - d) - (a - c)) - ((a - c) + y * ((b - d) - (a - c))) =
        ((x : Int) - y) * ((b - d) - (a - c)) := by grind
    rwa [e] at this
  rcases he _ _ dxy with h | h
  · -- q divides x - y with x, y < q, so x = y
    rcases Nat.le_total x y with hle | hle
    · have h2 : (q : Int) ∣ ((y - x : Nat) : Int) := by
        have : ((y - x : Nat) : Int) = -((x : Int) - y) := by omega
        rw [this]; exact Int.dvd_neg.mpr h
      have := Nat.eq_zero_of_dvd_of_lt (Int.natCast_dvd_natCast.mp h2) (by omega)
      omega
    · have h2 : (q : Int) ∣ ((x - y : Nat) : Int) := by
        have : ((x - y : Nat) : Int) = (x : Int) - y := by omega
        rw [this]; exact h
      have := Nat.eq_zero_of_dvd_of_lt (Int.natCast_dvd_natCast.mp h2) (by omega)
      omega
  · -- the slope vanishes, then the constant term vanishes: both sides agree everywhere
    exfalso
    apply hne
    have hA : (q : Int) ∣ a - c := by
      have := Int.dvd_sub dx (Int.dvd_mul_of_dvd_right (b := (x : Int)) h)
      have e : (a - c) + x * ((b - d) - (a - c)) - x * ((b - d) - (a - c)) = a - c := by omega
      rwa [e] at this
    have hB : (q : Int) ∣ b - d := by
      have := Int.dvd_add h hA
      have e : (b - d) - (a - c) + (a - c) = b - d := by omega
      rwa [e] at this
    exact ⟨Int.emod_eq_emod_iff_emod_sub_eq_zero.mpr (Int.emod_eq_zero_of_dvd hA),
      Int.emod_eq_emod_iff_emod_sub_eq_zero.mpr (Int.emod_eq_zero_of_dvd hB)⟩

theorem flat_count {P : List Nat → Bool} {A : List Nat → Bool} {q : Nat}
    (f : List Nat → List (List Nat)) : ∀ (l : List (List Nat)),
    (∀ xs ∈ l, (f xs).countP P ≤ if A xs then q else 1) →
    (l.flatMap f).countP P ≤ q * l.countP A + l.length
  | [], _ => by simp
  | a :: l, h => by
    have ih := flat_count f l (fun xs hxs => h xs (by simp [hxs]))
    have ha := h a (by simp)
    rw [List.flatMap_cons, List.countP_append, List.countP_cons, List.length_cons]
    by_cases hA : A a = true
    · simp only [hA, if_true] at ha ⊢
      rw [Nat.mul_add, Nat.mul_one]
      omega
    · simp only [hA, Bool.false_eq_true, if_false, Nat.add_zero] at ha ⊢
      omega

theorem mul_pow_pred_le (q m : Nat) : q * (m * q ^ (m - 1)) ≤ m * q ^ m := by
  cases m with
  | zero => simp
  | succ k =>
    simp only [Nat.add_sub_cancel, Nat.pow_succ]
    rw [Nat.mul_left_comm, Nat.mul_comm (q ^ k) q]
    exact Nat.le_refl _

/-- Lemma 23 (`pv_binding_count`): two tables that differ at some point of `{0,1}^m` (mod `q`)
have multilinear extensions that agree on at most `m q^(m-1)` points of `(Z/q)^m`. -/
theorem pv_binding_count {q : Nat} (he : EuclidMod q) : ∀ (m : Nat) (T U : List Bool → Int),
    (∃ bs : List Bool, bs.length = m ∧ T bs % q ≠ U bs % q) →
    (points q m).countP (agree q T U) ≤ m * q ^ (m - 1)
  | 0, T, U, ⟨bs, hl, hne⟩ => by
    have : bs = [] := List.eq_nil_of_length_eq_zero hl
    subst this
    simp [points, agree, mle, hne]
  | m + 1, T, U, ⟨bs, hl, hne⟩ => by
    obtain ⟨b, bs', rfl⟩ : ∃ b bs', bs = b :: bs' := by
      cases bs with
      | nil => simp at hl
      | cons b bs' => exact ⟨b, bs', rfl⟩
    let T0 : List Bool → Int := fun l => T (false :: l)
    let T1 : List Bool → Int := fun l => T (true :: l)
    let U0 : List Bool → Int := fun l => U (false :: l)
    let U1 : List Bool → Int := fun l => U (true :: l)
    let A : List Nat → Bool := fun xs => agree q T0 U0 xs && agree q T1 U1 xs
    have hstep : ∀ xs ∈ points q m,
        ((List.range q).map (· :: xs)).countP (agree q T U) ≤ if A xs then q else 1 := by
      intro xs _
      rw [List.countP_map]
      by_cases hA : A xs = true
      · simp only [hA, if_true]
        exact Nat.le_trans List.countP_le_length (by simp)
      · simp only [hA, Bool.false_eq_true, if_false]
        have hne' : ¬ (mle q T0 xs % q = mle q U0 xs % q ∧ mle q T1 xs % q = mle q U1 xs % q) := by
          rw [mle_mod, mle_mod, mle_mod, mle_mod]
          intro ⟨h0, h1⟩
          apply hA
          simp [A, agree, h0, h1]
        have := line_roots he _ _ _ _ hne'
        refine Nat.le_trans (Nat.le_of_eq ?_) this
        congr 1
    have hflat := flat_count (P := agree q T U) (A := A) (q := q)
      (fun xs => (List.range q).map (· :: xs)) (points q m) hstep
    have hsub : (points q m).countP A ≤ m * q ^ (m - 1) := by
      cases b with
      | false =>
        refine Nat.le_trans (List.countP_mono_left ?_) (pv_binding_count he m T0 U0
          ⟨bs', by simpa using hl, hne⟩)
        intro xs _ h; simp only [A, Bool.and_eq_true] at h; exact h.1
      | true =>
        refine Nat.le_trans (List.countP_mono_left ?_) (pv_binding_count he m T1 U1
          ⟨bs', by simpa using hl, hne⟩)
        intro xs _ h; simp only [A, Bool.and_eq_true] at h; exact h.2
    have hlen := points_length q m
    have h1 : q * (points q m).countP A ≤ q * (m * q ^ (m - 1)) := Nat.mul_le_mul_left q hsub
    have h2 := mul_pow_pred_le q m
    simp only [points, Nat.add_sub_cancel]
    rw [Nat.succ_mul]
    omega

/-! ## The BabyBear instance -/

/-- No divisor of `q` in `d, d + 1, ..., d + n - 1`. -/
def noDivFrom (q : Nat) : Nat → Nat → Bool
  | _, 0 => true
  | d, n + 1 => q % d != 0 && noDivFrom q (d + 1) n

theorem noDivFrom_sound {q : Nat} : ∀ {d n : Nat}, noDivFrom q d n = true →
    ∀ k, d ≤ k → k < d + n → q % k ≠ 0
  | _, 0, _, k, h1, h2 => by omega
  | d, n + 1, h, k, h1, h2 => by
    simp only [noDivFrom, Bool.and_eq_true, bne_iff_ne, ne_eq] at h
    by_cases hk : k = d
    · subst hk; exact h.1
    · exact noDivFrom_sound h.2 k (by omega) (by omega)

/-- Trial division to `s` with `q < s^2` proves primality. -/
theorem prime_of_noDiv {q s : Nat} (hq : 2 ≤ q) (hs : q < s * s) (h : noDivFrom q 2 (s - 2) = true) :
    IsPrimeNat q := by
  refine ⟨hq, fun d hd => ?_⟩
  obtain ⟨e, he⟩ := hd
  have hd0 : d ≠ 0 := by rintro rfl; simp at he; omega
  have he0 : e ≠ 0 := by rintro rfl; simp at he; omega
  by_cases hd1 : d = 1
  · exact Or.inl hd1
  by_cases he1 : e = 1
  · right; subst he1; simp at he; exact he.symm
  exfalso
  have hsound := noDivFrom_sound h
  by_cases hds : d < s
  · exact hsound d (by omega) (by omega) (by rw [he, Nat.mul_mod_right])
  · by_cases hes : e < s
    · exact hsound e (by omega) (by omega) (by rw [he, Nat.mul_mod_left])
    · have : s * s ≤ d * e := Nat.mul_le_mul (by omega) (by omega)
      omega

/-- BabyBear `p = 2013265921` is prime (trial division up to 44,871, checked by the kernel). -/
theorem pBB_prime : IsPrimeNat 2013265921 :=
  prime_of_noDiv (s := 44871) (by decide) (by decide) (by decide +kernel)

/-- Lemma 23 at BabyBear: distinct tables on `{0,1}^m` agree on at most `m p^(m-1)` points. -/
theorem pv_binding_count_babybear (m : Nat) (T U : List Bool → Int)
    (h : ∃ bs : List Bool, bs.length = m ∧ T bs % 2013265921 ≠ U bs % 2013265921) :
    (points 2013265921 m).countP (agree 2013265921 T U) ≤ m * 2013265921 ^ (m - 1) :=
  pv_binding_count (euclid_of_prime pBB_prime) m T U h

end Clean.Sherlock.Bridge.PvBinding
