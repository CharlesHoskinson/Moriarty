import Init

set_option exponentiation.threshold 300

/-!
Encodings between BabyBear and BLS12-381 Fr, and the deferred field log of the Sherlock bridge
candidate (`research/sherlock-stark-bls-bridge-20261111/CANDIDATE.md`, sections 2, 3 and 11).

Everything is stated over `Nat` and `Int` (no mathlib). The moduli are fixed literals, so every
bound below is checked by `decide` or `omega` on the real numbers.

Part 1 (arithmetic, lemmas 1 to 9 of section 11.5):

* limb encoding of Fr values (`dec`, `Canon`) and the lexicographic canonical check;
* injective packing of at most 8 BabyBear elements into one Fr element, with the length and range
  checks it needs (`pack_injective`, `pack_length_check_needed`, `pack_range_check_needed`);
* the bounded-quotient encoding of small-field multiplication into Fr
  (`bounded_quotient_sound`, `fmul_sound`, `unbounded_quotient_unsound`), the signed lazy
  reduction relation (`reduce_sound`) and the expression compiler (`Expr`, `arith_refines`);
* the base-`p` digits of a squeezed Fr value (`squeeze_digits_sound`, `squeeze_alias`).

Part 2 (the log): entries, their encoding as public values of `L`, the onion digest over
Poseidon2 on BabyBear, the deferred-or-emulated split and the discharge shard.
-/

set_option linter.unusedSectionVars false

namespace Clean.Sherlock.Bridge

/-! ## Constants -/

/-- The BabyBear prime `2^31 - 2^27 + 1`. -/
def pBB : Nat := 2013265921
/-- The BLS12-381 scalar field order `r` (about `2^254.86`). -/
def rFr : Nat := 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
/-- Bits per log limb. -/
def limbW : Nat := 30
/-- Limbs per logged Fr value. -/
def limbN : Nat := 9
/-- Packing base `2^31` (OpenVM `multi_field_packing.rs`, base `2^F::bits()`). -/
def packBase : Nat := 2 ^ 31
/-- Quotient range of the bounded-quotient gadget. -/
def qMax : Nat := 2 ^ 223

theorem pBB_eq : pBB = 2 ^ 31 - 2 ^ 27 + 1 := by decide
theorem pBB_pos : 0 < pBB := by decide
theorem limb_fits : 2 ^ limbW ≤ pBB := by decide
theorem limbs_cover : rFr ≤ 2 ^ (limbW * limbN) := by decide
theorem pack_fits : packBase ^ 8 < rFr := by decide
theorem packBase_gt : pBB < packBase := by decide
theorem qMax_fits : pBB + qMax * pBB ≤ rFr := by decide
theorem r_big : 2 ^ 254 < rFr := by decide

/-! ## Horner evaluation and digits -/

/-- Little-endian Horner evaluation in base `b`. -/
def horner (b : Nat) : List Nat → Nat
  | [] => 0
  | a :: l => a + b * horner b l

/-- Horner evaluation as the wrap computes it: every step is a reduction mod `r` (Fr cells). -/
def hornerR (b : Nat) : List Nat → Nat
  | [] => 0
  | a :: l => (a + b * hornerR b l) % rFr

/-- The first `n` little-endian digits of `x` in base `b`. -/
def digitsOf (b : Nat) : Nat → Nat → List Nat
  | 0, _ => []
  | n + 1, x => x % b :: digitsOf b n (x / b)

theorem add_mul_mod_congr (a b h n : Nat) : (a + b * (h % n)) % n = (a + b * h) % n := by
  have hm : (b * (h % n)) % n = (b * h) % n := by
    rw [Nat.mul_mod, Nat.mod_mod, ← Nat.mul_mod]
  rw [Nat.add_mod, hm, ← Nat.add_mod]

theorem hornerR_eq (b : Nat) : ∀ l, hornerR b l = horner b l % rFr
  | [] => rfl
  | a :: l => by simp only [hornerR, horner, hornerR_eq b l, add_mul_mod_congr]

theorem horner_lt {b : Nat} : ∀ {x : List Nat}, (∀ a ∈ x, a < b) → horner b x < b ^ x.length
  | [], _ => by simp [horner]
  | a :: x, h => by
    have ha : a < b := h a (by simp)
    have ih : horner b x < b ^ x.length := horner_lt (fun c hc => h c (by simp [hc]))
    have h1 : b * (horner b x + 1) ≤ b * b ^ x.length := Nat.mul_le_mul_left b ih
    simp only [horner, List.length_cons, Nat.pow_succ]
    rw [Nat.mul_comm (b ^ x.length) b]
    rw [Nat.mul_add] at h1
    omega

/-- Lemma 1 (`dec_inj` in general form): equal-length digit vectors with equal Horner value are
equal. -/
theorem horner_inj {b : Nat} : ∀ {x y : List Nat}, x.length = y.length → (∀ a ∈ x, a < b) →
    (∀ a ∈ y, a < b) → horner b x = horner b y → x = y
  | [], [], _, _, _, _ => rfl
  | [], _ :: _, hl, _, _, _ => by simp at hl
  | _ :: _, [], hl, _, _, _ => by simp at hl
  | a :: x, c :: y, hl, hx, hy, h => by
    simp only [horner] at h
    have ha : a < b := hx a (by simp)
    have hc : c < b := hy c (by simp)
    have hac : a = c := by
      have h' := congrArg (· % b) h
      simp only [Nat.add_mul_mod_self_left, Nat.mod_eq_of_lt ha, Nat.mod_eq_of_lt hc] at h'
      exact h'
    subst hac
    have hb : 0 < b := by omega
    have h2 : horner b x = horner b y :=
      Nat.eq_of_mul_eq_mul_left hb (by omega)
    rw [horner_inj (by simpa using hl) (fun c hc => hx c (by simp [hc]))
      (fun c hc => hy c (by simp [hc])) h2]

/-- Digits are unique: a digit vector and a high part that recompose `x` are the digits of `x`. -/
theorem digits_unique {b : Nat} (hb : 0 < b) : ∀ (n : Nat) (ds : List Nat) (h x : Nat),
    ds.length = n → (∀ d ∈ ds, d < b) → horner b ds + b ^ n * h = x → ds = digitsOf b n x
  | 0, [], _, _, _, _, _ => rfl
  | 0, _ :: _, _, _, hl, _, _ => by simp at hl
  | _ + 1, [], _, _, hl, _, _ => by simp at hl
  | n + 1, d :: ds, h, x, hl, hd, hx => by
    have hd0 : d < b := hd d (by simp)
    have hx' : x = d + b * (horner b ds + b ^ n * h) := by
      rw [← hx]; simp only [horner, Nat.pow_succ]
      rw [Nat.mul_add, Nat.mul_comm (b ^ n) b, Nat.mul_assoc]; omega
    have hmod : x % b = d := by
      rw [hx', Nat.add_mul_mod_self_left, Nat.mod_eq_of_lt hd0]
    have hdiv : x / b = horner b ds + b ^ n * h := by
      rw [hx', Nat.add_mul_div_left _ _ hb, Nat.div_eq_of_lt hd0, Nat.zero_add]
    simp only [digitsOf, hmod]
    rw [digits_unique hb n ds h (x / b) (by simpa using hl) (fun c hc => hd c (by simp [hc]))
      hdiv.symm]

/-! ## Limb encoding of Fr values in the log -/

/-- The integer value of a limb vector (base `2^30`, little-endian). -/
def dec (x : List Nat) : Nat := horner (2 ^ limbW) x
/-- The value of a limb vector as the wrap recomposes it in Fr. -/
def decR (x : List Nat) : Nat := hornerR (2 ^ limbW) x

/-- A canonical logged value: 9 limbs, each below `2^30`, and value below `r`. -/
def Canon (x : List Nat) : Prop := x.length = limbN ∧ (∀ a ∈ x, a < 2 ^ limbW) ∧ dec x < rFr

/-- Lemma 1: canonical limb vectors with equal value are equal. -/
theorem dec_inj {x y : List Nat} (hx : Canon x) (hy : Canon y) (h : dec x = dec y) : x = y :=
  horner_inj (hx.1.trans hy.1.symm) hx.2.1 hy.2.1 h

/-- Lemma 2: canonical limb vectors equal mod `r` are equal. -/
theorem canon_mod_inj {x y : List Nat} (hx : Canon x) (hy : Canon y)
    (h : dec x % rFr = dec y % rFr) : x = y := by
  rw [Nat.mod_eq_of_lt hx.2.2, Nat.mod_eq_of_lt hy.2.2] at h
  exact dec_inj hx hy h

/-- Lemma 3: a log limb embeds injectively in BabyBear. -/
theorem limb_embed_inj {a b : Nat} (ha : a < 2 ^ limbW) (hb : b < 2 ^ limbW)
    (h : a % pBB = b % pBB) : a = b := by
  have := limb_fits
  rwa [Nat.mod_eq_of_lt (show a < pBB by omega), Nat.mod_eq_of_lt (show b < pBB by omega)] at h

/-- On a canonical value the wrap's Fr recomposition is the integer value. -/
theorem decR_canon {x : List Nat} (hx : Canon x) : decR x = dec x := by
  simp only [decR, dec, hornerR_eq]
  exact Nat.mod_eq_of_lt hx.2.2

/-- Lexicographic strict order on little-endian digit vectors (the most significant limb last). -/
def ltLE : List Nat → List Nat → Prop
  | a :: x, c :: y => ltLE x y ∨ (x = y ∧ a < c)
  | _, _ => False

theorem ltLE_sound {b : Nat} : ∀ {x y : List Nat}, x.length = y.length → (∀ a ∈ x, a < b) →
    (∀ a ∈ y, a < b) → ltLE x y → horner b x < horner b y
  | [], _, _, _, _, h => by simp [ltLE] at h
  | _ :: _, [], _, _, _, h => by simp [ltLE] at h
  | a :: x, c :: y, hl, hx, hy, h => by
    simp only [ltLE] at h
    simp only [horner]
    rcases h with h | ⟨rfl, hac⟩
    · have ha : a < b := hx a (by simp)
      have ih := ltLE_sound (b := b) (by simpa using hl) (fun d hd => hx d (by simp [hd]))
        (fun d hd => hy d (by simp [hd])) h
      have h1 : b * (horner b x + 1) ≤ b * horner b y := Nat.mul_le_mul_left b ih
      rw [Nat.mul_add] at h1
      omega
    · omega

/-- The limbs of `r`, against which the wrap compares every logged value. -/
def rLimbs : List Nat := digitsOf (2 ^ limbW) limbN rFr

theorem rLimbs_props : rLimbs.length = limbN ∧ (∀ a ∈ rLimbs, a < 2 ^ limbW) ∧
    dec rLimbs = rFr := by decide

/-- The canonical-check gadget of the wrap: 9 range-checked limbs, lexicographically below the
limbs of `r`. -/
def CanonGadget (x : List Nat) : Prop :=
  x.length = limbN ∧ (∀ a ∈ x, a < 2 ^ limbW) ∧ ltLE x rLimbs

/-- Lemma 14: the wrap's range and comparison gadget implies `Canon`. -/
theorem canon_check_sound {x : List Nat} (h : CanonGadget x) : Canon x := by
  obtain ⟨hl, hr, hlt⟩ := h
  obtain ⟨rl, rr, rd⟩ := rLimbs_props
  refine ⟨hl, hr, ?_⟩
  have := ltLE_sound (hl.trans rl.symm) hr rr hlt
  simp only [dec] at rd ⊢
  omega

/-- Without the canonical check the wrap cannot tell `v` from `v + r`: the limb vectors of `5`
and of `5 + r` recompose to the same Fr element. -/
theorem alias_without_canon :
    decR (digitsOf (2 ^ limbW) limbN (5 + rFr)) = decR (digitsOf (2 ^ limbW) limbN 5) ∧
    digitsOf (2 ^ limbW) limbN (5 + rFr) ≠ digitsOf (2 ^ limbW) limbN 5 := by decide

/-! ## Packing BabyBear elements into Fr -/

/-- Packing of at most 8 BabyBear elements into one Fr element: Horner in base `2^31`, as
`multi_field_packing.rs:28-35`, reduced into Fr. -/
def pack (v : List Nat) : Nat := horner packBase v % rFr

/-- Lemma 4: on vectors of the same length at most 8 with entries below `2^31`, `pack` is
injective. -/
theorem pack_injective {v w : List Nat} (hl : v.length = w.length) (h8 : v.length ≤ 8)
    (hv : ∀ a ∈ v, a < packBase) (hw : ∀ a ∈ w, a < packBase) (h : pack v = pack w) : v = w := by
  have hb : packBase ^ v.length ≤ packBase ^ 8 := Nat.pow_le_pow_right (by decide) h8
  have h1 := horner_lt hv
  have h2 := horner_lt hw
  rw [← hl] at h2
  have := pack_fits
  simp only [pack] at h
  rw [Nat.mod_eq_of_lt (show horner packBase v < rFr by omega),
    Nat.mod_eq_of_lt (show horner packBase w < rFr by omega)] at h
  exact horner_inj hl hv hw h

/-- The length check is needed: `[1]` and `[1, 0]` pack to the same Fr element. -/
theorem pack_length_check_needed : pack [1] = pack [1, 0] ∧ ([1] : List Nat) ≠ [1, 0] := by decide

/-- The range check is needed: `[2^31]` and `[0, 1]` pack to the same Fr element. -/
theorem pack_range_check_needed : pack [2 ^ 31] = pack [0, 1] ∧ ([2 ^ 31] : List Nat) ≠ [0, 1] :=
  by decide

/-- The wrap's packing gadget (Fr arithmetic) computes `pack`. -/
theorem pack_gadget_refines (v : List Nat) : hornerR packBase v = pack v := by
  simp [pack, hornerR_eq]

/-- Split into chunks of 8 (the last one may be shorter). -/
def chunks8 (v : List Nat) : List (List Nat) :=
  if _h : v = [] then [] else v.take 8 :: chunks8 (v.drop 8)
termination_by v.length
decreasing_by
  cases v with
  | nil => contradiction
  | cons a l => simp <;> omega

theorem chunks8_nil : chunks8 [] = [] := by rw [chunks8]; simp

theorem chunks8_cons {v : List Nat} (h : v ≠ []) : chunks8 v = v.take 8 :: chunks8 (v.drop 8) := by
  rw [chunks8, dif_neg h]

/-- Pack each chunk: the absorb format of the Fr transcript of `L`. -/
def packChunks (v : List Nat) : List Nat := (chunks8 v).map pack

/-- The same in the wrap: each chunk packed with Fr arithmetic. -/
def packChunksR (v : List Nat) : List Nat := (chunks8 v).map (hornerR packBase)

theorem packChunksR_eq (v : List Nat) : packChunksR v = packChunks v := by
  simp [packChunksR, packChunks, pack_gadget_refines]

theorem packChunks_inj : ∀ (n : Nat) {v w : List Nat}, v.length ≤ n → v.length = w.length →
    (∀ a ∈ v, a < packBase) → (∀ a ∈ w, a < packBase) → packChunks v = packChunks w → v = w
  | 0, v, w, hn, hl, _, _, _ => by
    have hv : v = [] := List.eq_nil_of_length_eq_zero (by omega)
    have hw : w = [] := List.eq_nil_of_length_eq_zero (by omega)
    rw [hv, hw]
  | n + 1, v, w, hn, hl, hv, hw, h => by
    by_cases hv0 : v = []
    · have hw0 : w = [] := List.eq_nil_of_length_eq_zero (by rw [← hl, hv0]; rfl)
      rw [hv0, hw0]
    · have hw0 : w ≠ [] := by
        intro hw0; apply hv0; apply List.eq_nil_of_length_eq_zero; rw [hl, hw0]; rfl
      simp only [packChunks, chunks8_cons hv0, chunks8_cons hw0, List.map_cons] at h
      obtain ⟨h1, h2⟩ := List.cons.inj h
      have ht := pack_injective (v := v.take 8) (w := w.take 8) (by simp [hl])
        (by rw [List.length_take]; exact Nat.min_le_left _ _) (fun a ha => hv a (List.mem_of_mem_take ha))
        (fun a ha => hw a (List.mem_of_mem_take ha)) h1
      have hd := packChunks_inj n (v := v.drop 8) (w := w.drop 8)
        (by
          have : v.length ≠ 0 := fun h0 => hv0 (List.eq_nil_of_length_eq_zero h0)
          simp; omega)
        (by simp [hl]) (fun a ha => hv a (List.mem_of_mem_drop ha))
        (fun a ha => hw a (List.mem_of_mem_drop ha)) h2
      rw [← List.take_append_drop 8 v, ← List.take_append_drop 8 w, ht, hd]

/-- Every packed chunk is a canonical Fr element. -/
theorem packChunks_lt (v : List Nat) : ∀ x ∈ packChunks v, x < rFr := by
  intro x hx
  simp only [packChunks, List.mem_map] at hx
  obtain ⟨c, _, rfl⟩ := hx
  exact Nat.mod_lt _ (by decide)

/-! ## Bounded-quotient multiplication and lazy reduction -/

/-- The core relation. If the integer `x` and the witnessed `c + q p` are both below `r`, then
equality in Fr is equality of integers, so `c` is the BabyBear residue of `x`. -/
theorem bounded_quotient_sound {m n x c q : Nat} (hx : x < n) (hc : c < m) (hq : c + q * m < n)
    (h : x % n = (c + q * m) % n) : c = x % m := by
  rw [Nat.mod_eq_of_lt hx, Nat.mod_eq_of_lt hq] at h
  rw [h, Nat.add_mul_mod_self_right, Nat.mod_eq_of_lt hc]

/-- BabyBear multiplication in Fr: `a * b = c + q * p` in Fr with `c < p` and `q < 2^223` gives
`c = a * b mod p`. -/
theorem fmul_sound {a b c q : Nat} (ha : a < pBB) (hb : b < pBB) (hc : c < pBB) (hq : q < qMax)
    (h : (a * b) % rFr = (c + q * pBB) % rFr) : c = (a * b) % pBB := by
  have hab : a * b < rFr := by
    have : a * b ≤ (pBB - 1) * (pBB - 1) := Nat.mul_le_mul (by omega) (by omega)
    have : (pBB - 1) * (pBB - 1) < rFr := by decide
    omega
  have hq' : c + q * pBB < rFr := by
    have : q * pBB ≤ (qMax - 1) * pBB := Nat.mul_le_mul_right _ (by omega)
    have : pBB + (qMax - 1) * pBB < rFr := by decide
    omega
  exact bounded_quotient_sound hab hc hq' h

/-- Without the quotient bound the relation is unsound: with `q = p^(-1) mod r`, the claim
`1 * 1 = 0 + q * p` holds in Fr although `1 * 1 mod p = 1`. -/
theorem unbounded_quotient_unsound : ∃ a b c q : Nat, a < pBB ∧ b < pBB ∧ c < pBB ∧
    (a * b) % rFr = (c + q * pBB) % rFr ∧ c ≠ (a * b) % pBB :=
  ⟨1, 1, 0, 51629295354089899775704133734215549056150545422920697519896189442578700531036,
    by decide, by decide, by decide, by decide, by decide⟩

/-- Lemma 5 (`lift_eq`, difference form): integers equal mod `r` whose difference is below `r` in
absolute value are equal. -/
theorem lift_eq {x y : Int} (hd1 : -(rFr : Int) < x - y) (hd2 : x - y < rFr)
    (h : x % (rFr : Int) = y % (rFr : Int)) : x = y := by
  simp only [rFr] at *
  omega

/-- The signed lazy-reduction relation of the wrap, with explicit bounds: `x = q p + t` in Fr,
`0 ≤ t < 2^31`, `|q| < Q`, `|x| < X`, and the static condition `X + Q p + 2^31 ≤ r`. -/
def ReduceRel (x q t : Int) (X Q : Nat) : Prop :=
  x % (rFr : Int) = (q * pBB + t) % (rFr : Int) ∧ 0 ≤ t ∧ t < 2 ^ 31 ∧ -(Q : Int) < q ∧
    q < Q ∧ -(X : Int) < x ∧ x < X ∧ X + Q * pBB + 2 ^ 31 ≤ rFr

/-- Lemma 6: the lazy-reduction relation gives the BabyBear residue. -/
theorem reduce_sound {x q t : Int} {X Q : Nat} (h : ReduceRel x q t X Q) :
    x % (pBB : Int) = t % (pBB : Int) := by
  obtain ⟨he, ht0, ht1, hq1, hq2, hx1, hx2, hs⟩ := h
  have hd1 : -(rFr : Int) < x - (q * pBB + t) := by simp only [pBB, rFr] at *; omega
  have hd2 : x - (q * pBB + t) < rFr := by simp only [pBB, rFr] at *; omega
  have := lift_eq hd1 hd2 he
  rw [this, Int.add_comm, Int.add_mul_emod_self_right]

/-- The bit sizes of the plan: `|x| < 2^b`, `|q| < 2^(b - 30)` and `b ≤ 252` meet the static
condition. (The plan's `2^248 < r` would not: the condition needs `r > 2^254`.) -/
theorem reduce_bits_ok {b : Nat} (hb : b ≤ 252) :
    2 ^ b + 2 ^ (b - 30) * pBB + 2 ^ 31 ≤ rFr := by
  have h1 : 2 ^ b ≤ 2 ^ 252 := Nat.pow_le_pow_right (by decide) hb
  have h2 : 2 ^ (b - 30) ≤ 2 ^ 222 := Nat.pow_le_pow_right (by decide) (by omega)
  have h3 : 2 ^ (b - 30) * pBB ≤ 2 ^ 222 * pBB := Nat.mul_le_mul_right _ h2
  have : 2 ^ 252 + 2 ^ 222 * pBB + 2 ^ 31 ≤ rFr := by decide
  omega

/-! ## Expressions: the constraint DAG of `L` compiled into Fr -/

/-- BabyBear expressions over the proof values (lemma 12's DAG, as a tree). -/
inductive Expr where
  | const (c : Nat)
  | var (i : Nat)
  | add (a b : Expr)
  | mul (a b : Expr)
  deriving DecidableEq

namespace Expr

/-- Exact integer value. -/
def eval (env : List Nat) : Expr → Nat
  | const c => c
  | var i => env.getD i 0
  | add a b => a.eval env + b.eval env
  | mul a b => a.eval env * b.eval env

/-- Reference semantics: BabyBear arithmetic, every operation reduced mod `p`. -/
def evalP (env : List Nat) : Expr → Nat
  | const c => c % pBB
  | var i => env.getD i 0 % pBB
  | add a b => (a.evalP env + b.evalP env) % pBB
  | mul a b => (a.evalP env * b.evalP env) % pBB

/-- Wrap semantics: Fr arithmetic, every operation reduced mod `r` (no BabyBear reduction). -/
def evalR (env : List Nat) : Expr → Nat
  | const c => c % rFr
  | var i => env.getD i 0 % rFr
  | add a b => (a.evalR env + b.evalR env) % rFr
  | mul a b => (a.evalR env * b.evalR env) % rFr

/-- The tracked bound of the lazy rule (lemma 7): variables are canonical BabyBear elements. -/
def bound : Expr → Nat
  | const c => c
  | var _ => pBB - 1
  | add a b => a.bound + b.bound
  | mul a b => a.bound * b.bound

/-- Variables read by the expression. -/
def vars : Expr → List Nat
  | const _ => []
  | var i => [i]
  | add a b => a.vars ++ b.vars
  | mul a b => a.vars ++ b.vars

theorem evalP_eq (env : List Nat) : ∀ e : Expr, e.evalP env = e.eval env % pBB
  | const _ => rfl
  | var _ => rfl
  | add a b => by simp only [evalP, eval, evalP_eq env a, evalP_eq env b, ← Nat.add_mod]
  | mul a b => by simp only [evalP, eval, evalP_eq env a, evalP_eq env b, ← Nat.mul_mod]

theorem evalR_eq (env : List Nat) : ∀ e : Expr, e.evalR env = e.eval env % rFr
  | const _ => rfl
  | var _ => rfl
  | add a b => by simp only [evalR, eval, evalR_eq env a, evalR_eq env b, ← Nat.add_mod]
  | mul a b => by simp only [evalR, eval, evalR_eq env a, evalR_eq env b, ← Nat.mul_mod]

/-- Lemma 7 (`add_bound`, `mul_bound`): on canonical variables the value is within the bound. -/
theorem eval_le_bound (env : List Nat) : ∀ e : Expr, (∀ i ∈ e.vars, env.getD i 0 < pBB) →
    e.eval env ≤ e.bound
  | const _, _ => Nat.le_refl _
  | var i, h => by
    have := h i (by simp [vars])
    simp only [eval, bound]; omega
  | add a b, h => by
    simp only [eval, bound]
    exact Nat.add_le_add (eval_le_bound env a (fun i hi => h i (by simp [vars, hi])))
      (eval_le_bound env b (fun i hi => h i (by simp [vars, hi])))
  | mul a b, h => by
    simp only [eval, bound]
    exact Nat.mul_le_mul (eval_le_bound env a (fun i hi => h i (by simp [vars, hi])))
      (eval_le_bound env b (fun i hi => h i (by simp [vars, hi])))

end Expr

/-- The wrap's gadget for one equality check `l = r` of `L`: range checks on every variable,
lazy Fr evaluation of both sides, and one bounded-quotient reduction per side to a common
residue `c`. -/
def ArithGadget (env : List Nat) (l r : Expr) : Prop :=
  (∀ i ∈ l.vars ++ r.vars, env.getD i 0 < pBB) ∧
    ∃ c q₁ q₂, c < pBB ∧ q₁ < qMax ∧ q₂ < qMax ∧
      l.evalR env = (c + q₁ * pBB) % rFr ∧ r.evalR env = (c + q₂ * pBB) % rFr

/-- The static condition on a check of the key: both tracked bounds stay below `r`. -/
abbrev ArithOK (l r : Expr) : Prop := l.bound < rFr ∧ r.bound < rFr

theorem reduce_side {env : List Nat} {e : Expr} {c q : Nat} (hv : ∀ i ∈ e.vars, env.getD i 0 < pBB)
    (hb : e.bound < rFr) (hc : c < pBB) (hq : q < qMax) (h : e.evalR env = (c + q * pBB) % rFr) :
    c = e.evalP env := by
  have hle := e.eval_le_bound env hv
  rw [Expr.evalR_eq] at h
  rw [Expr.evalP_eq]
  have hq' : c + q * pBB < rFr := by
    have : q * pBB ≤ (qMax - 1) * pBB := Nat.mul_le_mul_right _ (by omega)
    have : pBB + (qMax - 1) * pBB < rFr := by decide
    omega
  exact bounded_quotient_sound (by omega) hc hq' h

/-- Lemma 12 (`constraint_dag_refines`): an accepted wrap gadget for `l = r` implies the
reference BabyBear equality, provided the key's static bound check holds. -/
theorem arith_refines {env : List Nat} {l r : Expr} (hok : ArithOK l r)
    (h : ArithGadget env l r) :
    (∀ i ∈ l.vars ++ r.vars, env.getD i 0 < pBB) ∧ l.evalP env = r.evalP env := by
  obtain ⟨hv, c, q₁, q₂, hc, hq₁, hq₂, h₁, h₂⟩ := h
  refine ⟨hv, ?_⟩
  have e₁ := reduce_side (fun i hi => hv i (List.mem_append_left _ hi)) hok.1 hc hq₁ h₁
  have e₂ := reduce_side (fun i hi => hv i (List.mem_append_right _ hi)) hok.2 hc hq₂ h₂
  rw [← e₁, ← e₂]

/-! ### Instances of the compiler: extension multiplication, Horner, sumcheck rounds -/

/-- A check `e = y` with `y` a single variable: the variable holds the BabyBear residue. -/
theorem arith_var_sound {env : List Nat} {e : Expr} {j : Nat} (hok : ArithOK e (.var j))
    (h : ArithGadget env e (.var j)) : env.getD j 0 = e.eval env % pBB := by
  obtain ⟨hv, he⟩ := arith_refines hok h
  have hj := hv j (by simp [Expr.vars])
  rw [Expr.evalP_eq, Expr.evalP_eq] at he
  simp only [Expr.eval] at he
  rw [Nat.mod_eq_of_lt hj] at he
  exact he.symm

/-- `BabyBear^4 = BabyBear[X]/(X^4 - 11)`: the four unreduced product sums (16 products), with
`a` in variables `0..3`, `b` in `4..7`. -/
def efRaw0 : Expr := .add (.mul (.var 0) (.var 4)) (.mul (.const 11)
  (.add (.mul (.var 1) (.var 7)) (.add (.mul (.var 2) (.var 6)) (.mul (.var 3) (.var 5)))))
def efRaw1 : Expr := .add (.add (.mul (.var 0) (.var 5)) (.mul (.var 1) (.var 4))) (.mul (.const 11)
  (.add (.mul (.var 2) (.var 7)) (.mul (.var 3) (.var 6))))
def efRaw2 : Expr := .add (.add (.mul (.var 0) (.var 6)) (.add (.mul (.var 1) (.var 5))
  (.mul (.var 2) (.var 4)))) (.mul (.const 11) (.mul (.var 3) (.var 7)))
def efRaw3 : Expr := .add (.add (.mul (.var 0) (.var 7)) (.mul (.var 1) (.var 6)))
  (.add (.mul (.var 2) (.var 5)) (.mul (.var 3) (.var 4)))

/-- The reference product in `BabyBear^4` with `X^4 = 11` (coefficients little-endian). -/
def efMul (a b : Nat × Nat × Nat × Nat) : Nat × Nat × Nat × Nat :=
  let (a0, a1, a2, a3) := a
  let (b0, b1, b2, b3) := b
  ((a0 * b0 + 11 * (a1 * b3 + a2 * b2 + a3 * b1)) % pBB,
   (a0 * b1 + a1 * b0 + 11 * (a2 * b3 + a3 * b2)) % pBB,
   (a0 * b2 + a1 * b1 + a2 * b0 + 11 * (a3 * b3)) % pBB,
   (a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0) % pBB)

theorem efRaw_ok : ArithOK efRaw0 (.var 8) ∧ ArithOK efRaw1 (.var 9) ∧ ArithOK efRaw2 (.var 10) ∧
    ArithOK efRaw3 (.var 11) := by decide

/-- Lemma 8 (`ef_mul_sound`): the 16-product, 4-reduction gadget computes `efMul`. The
environment holds `a`, `b`, then the four output cells. -/
theorem ef_mul_sound {a0 a1 a2 a3 b0 b1 b2 b3 o0 o1 o2 o3 : Nat}
    (h0 : ArithGadget [a0, a1, a2, a3, b0, b1, b2, b3, o0, o1, o2, o3] efRaw0 (.var 8))
    (h1 : ArithGadget [a0, a1, a2, a3, b0, b1, b2, b3, o0, o1, o2, o3] efRaw1 (.var 9))
    (h2 : ArithGadget [a0, a1, a2, a3, b0, b1, b2, b3, o0, o1, o2, o3] efRaw2 (.var 10))
    (h3 : ArithGadget [a0, a1, a2, a3, b0, b1, b2, b3, o0, o1, o2, o3] efRaw3 (.var 11)) :
    (o0, o1, o2, o3) = efMul (a0, a1, a2, a3) (b0, b1, b2, b3) := by
  have e0 := arith_var_sound efRaw_ok.1 h0
  have e1 := arith_var_sound efRaw_ok.2.1 h1
  have e2 := arith_var_sound efRaw_ok.2.2.1 h2
  have e3 := arith_var_sound efRaw_ok.2.2.2 h3
  simp only [efRaw0, efRaw1, efRaw2, efRaw3, Expr.eval, List.getD_cons_succ,
    List.getD_cons_zero] at e0 e1 e2 e3
  simp only [efMul, Prod.mk.injEq]
  simp only [pBB] at *
  refine ⟨?_, ?_, ?_, ?_⟩ <;> omega

/-- Horner evaluation of a polynomial with coefficient variables `cs` at the variable `x`
(the final polynomial of WHIR, folding formulas). -/
def hornerExpr (x : Nat) : List Nat → Expr
  | [] => .const 0
  | c :: cs => .add (.var c) (.mul (.var x) (hornerExpr x cs))

theorem hornerExpr_eval (env : List Nat) (x : Nat) : ∀ cs : List Nat,
    (hornerExpr x cs).eval env = horner (env.getD x 0) (cs.map (env.getD · 0))
  | [] => rfl
  | c :: cs => by simp [hornerExpr, Expr.eval, horner, hornerExpr_eval env x cs]

/-- Lemma 11 (`horner_refines`): a wrap check `poly(x) = y` implies the reference evaluation. -/
theorem horner_refines {env : List Nat} {x y : Nat} {cs : List Nat}
    (hok : ArithOK (hornerExpr x cs) (.var y)) (h : ArithGadget env (hornerExpr x cs) (.var y)) :
    horner (env.getD x 0) (cs.map (env.getD · 0)) % pBB = env.getD y 0 % pBB := by
  have := (arith_refines hok h).2
  simpa [Expr.evalP_eq, Expr.eval, hornerExpr_eval] using this

/-- The sum of variables. -/
def sumExpr : List Nat → Expr
  | [] => .const 0
  | c :: cs => .add (.var c) (sumExpr cs)

theorem sumExpr_eval (env : List Nat) : ∀ cs : List Nat,
    (sumExpr cs).eval env = (cs.map (env.getD · 0)).sum
  | [] => rfl
  | c :: cs => by simp [sumExpr, Expr.eval, sumExpr_eval env cs]

/-- `g(0) + g(1)` for `g = sum_j c_j X^j`: `c_0 + sum_j c_j`. -/
def roundSumExpr : List Nat → Expr
  | [] => .const 0
  | c :: cs => .add (.var c) (sumExpr (c :: cs))

theorem roundSumExpr_eval (env : List Nat) (cs : List Nat) :
    (roundSumExpr cs).eval env = horner 0 (cs.map (env.getD · 0)) +
      horner 1 (cs.map (env.getD · 0)) := by
  have h0 : ∀ l : List Nat, horner 0 l = l.headD 0 := by
    intro l; cases l <;> simp [horner]
  have h1 : ∀ l : List Nat, horner 1 l = l.sum := by
    intro l; induction l with
    | nil => rfl
    | cons a l ih => simp [horner, ih]
  cases cs with
  | nil => simp [roundSumExpr, Expr.eval, horner]
  | cons c cs => simp [roundSumExpr, Expr.eval, sumExpr_eval, h0, h1]

/-- One sumcheck round of `L` (WHIR runs `k_whir` of them per round): the coefficient variables
of the round polynomial `g`, the challenge variable `alpha`, the incoming claim and the outgoing
claim. -/
structure SumcheckRound where
  cs : List Nat
  alpha : Nat
  claim : Nat
  next : Nat

/-- Lemma 11 (`sumcheck_round_refines`): the two wrap checks of a round give the reference
relations `g(0) + g(1) = claim` and `g(alpha) = next` in BabyBear. -/
theorem sumcheck_round_refines {env : List Nat} {r : SumcheckRound}
    (hok₁ : ArithOK (roundSumExpr r.cs) (.var r.claim))
    (h₁ : ArithGadget env (roundSumExpr r.cs) (.var r.claim))
    (hok₂ : ArithOK (hornerExpr r.alpha r.cs) (.var r.next))
    (h₂ : ArithGadget env (hornerExpr r.alpha r.cs) (.var r.next)) :
    (horner 0 (r.cs.map (env.getD · 0)) + horner 1 (r.cs.map (env.getD · 0))) % pBB =
      env.getD r.claim 0 % pBB ∧
    horner (env.getD r.alpha 0) (r.cs.map (env.getD · 0)) % pBB = env.getD r.next 0 % pBB := by
  refine ⟨?_, horner_refines hok₂ h₂⟩
  have := (arith_refines hok₁ h₁).2
  simpa [Expr.evalP_eq, Expr.eval, roundSumExpr_eval] using this

/-- Lemma 11 (`whir_round_refines`): a WHIR round is a chain of sumcheck rounds, each round's
outgoing claim the next round's incoming claim. If the wrap accepts every round's two checks,
the reference sumcheck relations hold along the whole chain. -/
theorem whir_round_refines {env : List Nat} : ∀ (rs : List SumcheckRound),
    (∀ r ∈ rs, ArithOK (roundSumExpr r.cs) (.var r.claim) ∧
      ArithGadget env (roundSumExpr r.cs) (.var r.claim) ∧
      ArithOK (hornerExpr r.alpha r.cs) (.var r.next) ∧
      ArithGadget env (hornerExpr r.alpha r.cs) (.var r.next)) →
    ∀ r ∈ rs, (horner 0 (r.cs.map (env.getD · 0)) + horner 1 (r.cs.map (env.getD · 0))) % pBB =
        env.getD r.claim 0 % pBB ∧
      horner (env.getD r.alpha 0) (r.cs.map (env.getD · 0)) % pBB = env.getD r.next 0 % pBB := by
  intro rs h r hr
  obtain ⟨a, b, c, d⟩ := h r hr
  exact sumcheck_round_refines a b c d

/-- Chained rounds: the claim of round `j + 1` is `g_j(alpha_j)`. -/
def Chained : List SumcheckRound → Prop
  | r :: s :: rest => r.next = s.claim ∧ Chained (s :: rest)
  | _ => True

/-- Along a chained WHIR round the reference claims propagate: each round's `g(0) + g(1)` is the
previous round's `g(alpha)` (mod `p`). -/
theorem whir_round_chain {env : List Nat} : ∀ (r s : SumcheckRound) (rest : List SumcheckRound),
    Chained (r :: s :: rest) →
    (∀ t ∈ r :: s :: rest, ArithOK (roundSumExpr t.cs) (.var t.claim) ∧
      ArithGadget env (roundSumExpr t.cs) (.var t.claim) ∧
      ArithOK (hornerExpr t.alpha t.cs) (.var t.next) ∧
      ArithGadget env (hornerExpr t.alpha t.cs) (.var t.next)) →
    horner (env.getD r.alpha 0) (r.cs.map (env.getD · 0)) % pBB =
      (horner 0 (s.cs.map (env.getD · 0)) + horner 1 (s.cs.map (env.getD · 0))) % pBB := by
  intro r s rest hc h
  have hr := whir_round_refines _ h r (by simp)
  have hs := whir_round_refines _ h s (by simp)
  rw [hr.2, hs.1, hc.1]

/-- One binary fold at the coset `{x, -x}` (WHIR folds `2^k_whir` to 1 as `k_whir` binary
folds): `2 x y = x (f(x) + f(-x)) + beta (f(x) - f(-x))`, the division-free form of
`y = (f(x) + f(-x)) / 2 + beta (f(x) - f(-x)) / (2 x)`. Subtraction is `+ (p - 1) *`. -/
def foldLhs (x y : Nat) : Expr := .mul (.const 2) (.mul (.var x) (.var y))
def foldRhs (x fx fnx beta : Nat) : Expr :=
  .add (.mul (.var x) (.add (.var fx) (.var fnx)))
    (.mul (.var beta) (.add (.var fx) (.mul (.const (pBB - 1)) (.var fnx))))

/-- Lemma 11 (`coset_fold_refines`): the wrap's fold check implies the reference fold relation
in BabyBear. -/
theorem coset_fold_refines {env : List Nat} {x y fx fnx beta : Nat}
    (hok : ArithOK (foldLhs x y) (foldRhs x fx fnx beta))
    (h : ArithGadget env (foldLhs x y) (foldRhs x fx fnx beta)) :
    (2 * (env.getD x 0 * env.getD y 0)) % pBB =
      (env.getD x 0 * (env.getD fx 0 + env.getD fnx 0) +
        env.getD beta 0 * (env.getD fx 0 + (pBB - 1) * env.getD fnx 0)) % pBB := by
  have := (arith_refines hok h).2
  simpa [Expr.evalP_eq, Expr.eval, foldLhs, foldRhs] using this

/-- The honest fold passes: for `f(t) = a + b t`, `y = a + beta b` satisfies the fold relation
(as an identity over the integers, so also mod `p`). -/
theorem fold_complete (a b x beta : Int) :
    2 * (x * (a + beta * b)) = x * ((a + b * x) + (a - b * x)) + beta * ((a + b * x) - (a - b * x)) :=
  by grind

/-! ### The `eq` binding of `pv` -/

/-- `eq(rho, bits)` as an expression: factor `rho_j` for a bit 1, `1 - rho_j` (written
`1 + (p - 1) rho_j`) for a bit 0. -/
def eqExpr : List Nat → List Bool → Expr
  | r :: rs, b :: bs =>
      .mul (if b then .var r else .add (.const 1) (.mul (.const (pBB - 1)) (.var r))) (eqExpr rs bs)
  | _, _ => .const 1

/-- The reference value of `eq(rho, bits)` (unreduced). -/
def eqVal : List Nat → List Bool → Nat
  | r :: rs, b :: bs => (if b then r else 1 + (pBB - 1) * r) * eqVal rs bs
  | _, _ => 1

theorem eqExpr_eval (env : List Nat) : ∀ (rs : List Nat) (bs : List Bool),
    (eqExpr rs bs).eval env = eqVal (rs.map (env.getD · 0)) bs
  | [], _ => rfl
  | _ :: _, [] => rfl
  | r :: rs, b :: bs => by
    cases b <;> simp [eqExpr, eqVal, Expr.eval, eqExpr_eval env rs bs]

/-- `sum_i eq(rho, bits_i) pv_i` over a table of (index bits, `pv` variable). -/
def eqSumExpr (rho : List Nat) : List (List Bool × Nat) → Expr
  | [] => .const 0
  | (bs, v) :: t => .add (.mul (eqExpr rho bs) (.var v)) (eqSumExpr rho t)

/-- Its reference value. -/
def eqSumVal (rho : List Nat) (env : List Nat) : List (List Bool × Nat) → Nat
  | [] => 0
  | (bs, v) :: t => eqVal rho bs * env.getD v 0 + eqSumVal rho env t

theorem eqSumExpr_eval (env : List Nat) (rho : List Nat) : ∀ tbl : List (List Bool × Nat),
    (eqSumExpr rho tbl).eval env = eqSumVal (rho.map (env.getD · 0)) env tbl
  | [] => rfl
  | (bs, v) :: t => by simp [eqSumExpr, eqSumVal, Expr.eval, eqExpr_eval, eqSumExpr_eval env rho t]

/-- Lemma 13 (`pv_eq_refines`): the wrap's check `sum_i eq(rho, i) pv_i = claim` at the
batch-constraint point `rho` implies the reference evaluation of the multilinear extension of
`pv`. -/
theorem pv_eq_refines {env : List Nat} {rho : List Nat} {tbl : List (List Bool × Nat)} {claim : Nat}
    (hok : ArithOK (eqSumExpr rho tbl) (.var claim))
    (h : ArithGadget env (eqSumExpr rho tbl) (.var claim)) :
    eqSumVal (rho.map (env.getD · 0)) env tbl % pBB = env.getD claim 0 % pBB := by
  have := (arith_refines hok h).2
  simpa [Expr.evalP_eq, Expr.eval, eqSumExpr_eval] using this

/-- On Boolean points `eq` is the indicator: `eq(rho, bits) = 1` if `rho` is `bits`, else `0`
(mod `p`). So `sum_i eq(rho, i) pv_i` is the multilinear extension of the table `pv`. -/
theorem eqVal_bool : ∀ (rho : List Nat) (bs : List Bool), rho.length = bs.length →
    (∀ r ∈ rho, r ≤ 1) →
    eqVal rho bs % pBB = if rho = bs.map (fun b => if b then 1 else 0) then 1 else 0
  | [], [], _, _ => by decide
  | [], _ :: _, hl, _ => by simp at hl
  | _ :: _, [], hl, _ => by simp at hl
  | r :: rs, b :: bs, hl, hr => by
    have ih := eqVal_bool rs bs (by simpa using hl) (fun x hx => hr x (by simp [hx]))
    have hr0 : r ≤ 1 := hr r (by simp)
    simp only [eqVal]
    rw [Nat.mul_mod, ih]
    have hf : (if b then r else 1 + (pBB - 1) * r) % pBB = if r = (if b then 1 else 0) then 1 else 0 := by
      rcases Nat.le_one_iff_eq_zero_or_eq_one.mp hr0 with rfl | rfl <;> cases b <;> decide
    rw [hf]
    simp only [List.map_cons, List.cons.injEq]
    by_cases h1 : r = (if b then 1 else 0) <;>
      by_cases h2 : rs = bs.map (fun b => if b then 1 else 0) <;> simp [h1, h2] <;> decide

/-! ## Squeezing challenges from Fr -/

/-- The wrap's digit gadget for a squeeze: 5 range-checked digits and a high part recomposing
the Fr cell `x`, with the canonical check that the recomposition is below `r`. -/
def SqueezeGadget (x : Nat) (ds : List Nat) : Prop :=
  ds.length = 5 ∧ (∀ d ∈ ds, d < pBB) ∧ ∃ h, horner pBB ds + pBB ^ 5 * h < rFr ∧
    (horner pBB ds + pBB ^ 5 * h) % rFr = x % rFr

/-- Lemma 9: the 5 base-`p` digits of a squeezed Fr value are unique given the canonical check. -/
theorem squeeze_digits_sound {x : Nat} {ds : List Nat} (hx : x < rFr) (h : SqueezeGadget x ds) :
    ds = digitsOf pBB 5 x := by
  obtain ⟨hl, hd, h, hlt, he⟩ := h
  rw [Nat.mod_eq_of_lt hlt, Nat.mod_eq_of_lt hx] at he
  exact digits_unique pBB_pos 5 ds h x hl hd he

/-- Without the canonical check the digits of `r` also recompose `0` in Fr, and they are not the
digits of `0`. -/
theorem squeeze_alias :
    (horner pBB (digitsOf pBB 5 rFr) + pBB ^ 5 * (rFr / pBB ^ 5)) % rFr = 0 % rFr ∧
    digitsOf pBB 5 rFr ≠ digitsOf pBB 5 0 := by decide


/-! ## Part 2: the deferred field log -/

/-- The primitives as data (not assumptions). `p2` is Poseidon2 over BabyBear with a digest of
8 elements; `frSponge` is Midnight's Poseidon sponge over Fr; `frPerm` its permutation. -/
structure Prims where
  frPerm : Nat × Nat × Nat → Nat × Nat × Nat
  frSponge : List Nat → Nat
  jubjubMul : Nat → Nat × Nat → Nat × Nat
  jubjubGen : Nat × Nat
  p2 : List Nat → List Nat
  p2_width : ∀ a, (p2 a).length = 8
  frSponge_lt : ∀ a, frSponge a < rFr

/-- Midnight's two-input hash (Merkle compression, `H2`). -/
def Prims.frH2 (X : Prims) (a b : Nat) : Nat := X.frSponge [a, b]

/-- Merkle root from a leaf, the path bits (leaf to root) and the siblings. -/
def merkleRoot (X : Prims) (leaf : Nat) : List Bool → List Nat → Nat
  | b :: bs, s :: ss => merkleRoot X (if b then X.frH2 s leaf else X.frH2 leaf s) bs ss
  | _, _ => leaf

/-- The frozen, versioned operation list (CANDIDATE.md section 3.1). -/
inductive Op where
  | perm
  | h2
  | sponge (n : Nat)
  | path (d : Nat)
  | ecmul
  | ecmulg
  | pub (i : Nat)
  | digest
  | nop
  deriving DecidableEq

/-- The number of logged values of each operation (inputs and outputs). The operation fixes it. -/
def Op.arity : Op → Nat
  | perm => 6
  | h2 => 3
  | sponge n => n + 1
  | path _ => 3
  | ecmul => 5
  | ecmulg => 3
  | pub _ => 1
  | digest => 9
  | nop => 0

/-- Operation tag and parameter, as two BabyBear words. -/
def Op.code : Op → List Nat
  | perm => [1, 0]
  | h2 => [2, 0]
  | sponge n => [3, n]
  | path d => [4, d]
  | ecmul => [5, 0]
  | ecmulg => [6, 0]
  | pub i => [7, i]
  | digest => [8, 0]
  | nop => [0, 0]

theorem Op.code_length (o : Op) : o.code.length = 2 := by cases o <;> rfl

theorem Op.code_inj {a b : Op} (h : a.code = b.code) : a = b := by
  cases a <;> cases b <;> simp_all [Op.code]

/-- A log entry: the operation and its values (inputs, then claimed outputs) as limb vectors. -/
structure Entry where
  op : Op
  vals : List (List Nat)
  deriving DecidableEq

/-- Well-formed entry: the parameter is a BabyBear word, the operation fixes the number of
values, and every value is canonical. -/
def Entry.WF (e : Entry) : Prop :=
  (∀ x ∈ e.op.code, x < pBB) ∧ e.vals.length = e.op.arity ∧ ∀ v ∈ e.vals, Canon v

/-- The public-value words of one entry. -/
def Entry.enc (e : Entry) : List Nat := e.op.code ++ e.vals.flatten

/-- The padding entry. -/
def nopE : Entry := ⟨.nop, []⟩

theorem nopE_wf : nopE.WF := by
  refine ⟨?_, rfl, by simp [nopE]⟩
  intro x hx; simp [nopE, Op.code] at hx; subst hx; decide

/-- The public-value vector `pv` of `L` for a table of entries. -/
def encode (l : List Entry) : List Nat := (l.map Entry.enc).flatten

/-- The table padded with `NOP` to the fixed capacity `B`. -/
def pad (B : Nat) (l : List Entry) : List Entry := l ++ List.replicate (B - l.length) nopE

theorem pad_length {B : Nat} {l : List Entry} (h : l.length ≤ B) : (pad B l).length = B := by
  simp [pad]; omega

theorem flatten_append_inj {n : Nat} : ∀ {xs ys : List (List Nat)} {s t : List Nat},
    xs.length = ys.length → (∀ v ∈ xs, v.length = n) → (∀ v ∈ ys, v.length = n) →
    xs.flatten ++ s = ys.flatten ++ t → xs = ys ∧ s = t
  | [], [], _, _, _, _, _, h => ⟨rfl, by simpa using h⟩
  | [], _ :: _, _, _, hl, _, _, _ => by simp at hl
  | _ :: _, [], _, _, hl, _, _, _ => by simp at hl
  | x :: xs, y :: ys, s, t, hl, hx, hy, h => by
    simp only [List.flatten_cons, List.append_assoc] at h
    have hxy := List.append_inj h ((hx x (by simp)).trans (hy y (by simp)).symm)
    have ih := flatten_append_inj (by simpa using hl) (fun v hv => hx v (by simp [hv]))
      (fun v hv => hy v (by simp [hv])) hxy.2
    exact ⟨by rw [hxy.1, ih.1], ih.2⟩

theorem enc_append_inj {e f : Entry} {s t : List Nat} (he : e.WF) (hf : f.WF)
    (h : e.enc ++ s = f.enc ++ t) : e = f ∧ s = t := by
  simp only [Entry.enc, List.append_assoc] at h
  have h1 := List.append_inj h (by rw [Op.code_length, Op.code_length])
  have hop := Op.code_inj h1.1
  have h2 := flatten_append_inj (n := limbN) (by rw [he.2.1, hf.2.1, hop])
    (fun v hv => (he.2.2 v hv).1) (fun v hv => (hf.2.2 v hv).1) h1.2
  refine ⟨?_, h2.2⟩
  cases e
  cases f
  simp only [Entry.mk.injEq]
  exact ⟨hop, h2.1⟩

/-- Lemma 17 (`encode_inj`): on well-formed tables of the same length the public-value vector
determines the table. -/
theorem encode_inj : ∀ {a b : List Entry}, a.length = b.length → (∀ e ∈ a, e.WF) →
    (∀ e ∈ b, e.WF) → encode a = encode b → a = b
  | [], [], _, _, _, _ => rfl
  | [], _ :: _, hl, _, _, _ => by simp at hl
  | _ :: _, [], hl, _, _, _ => by simp at hl
  | e :: a, f :: b, hl, ha, hb, h => by
    simp only [encode, List.map_cons, List.flatten_cons] at h
    have h1 := enc_append_inj (ha e (by simp)) (hb f (by simp)) h
    have ih := encode_inj (by simpa using hl) (fun x hx => ha x (by simp [hx]))
      (fun x hx => hb x (by simp [hx])) h1.2
    rw [h1.1, ih]

/-- `encode` is prefix-free over tables with the same number of entries. -/
theorem encode_append_inj : ∀ {a b : List Entry} {s t : List Nat}, a.length = b.length →
    (∀ e ∈ a, e.WF) → (∀ e ∈ b, e.WF) → encode a ++ s = encode b ++ t → a = b ∧ s = t
  | [], [], _, _, _, _, _, h => ⟨rfl, by simpa [encode] using h⟩
  | [], _ :: _, _, _, hl, _, _, _ => by simp at hl
  | _ :: _, [], _, _, hl, _, _, _ => by simp at hl
  | e :: a, f :: b, s, t, hl, ha, hb, h => by
    simp only [encode, List.map_cons, List.flatten_cons, List.append_assoc] at h
    have h1 := enc_append_inj (ha e (by simp)) (hb f (by simp)) h
    have ih := encode_append_inj (s := s) (t := t) (by simpa using hl)
      (fun x hx => ha x (by simp [hx])) (fun x hx => hb x (by simp [hx])) h1.2
    exact ⟨by rw [h1.1, ih.1], ih.2⟩

/-- The public-value vector of `L` with the fixed word length `W` of the key: the encoding of
the table, zero-padded to `W` (lead amendment 4). -/
def pvOf (W : Nat) (l : List Entry) : List Nat :=
  encode l ++ List.replicate (W - (encode l).length) 0

theorem pvOf_length {W : Nat} {l : List Entry} (h : (encode l).length ≤ W) :
    (pvOf W l).length = W := by
  simp [pvOf]; omega

/-- `pvOf` is injective on well-formed tables with the same number of entries. -/
theorem pvOf_inj {W : Nat} {a b : List Entry} (hl : a.length = b.length) (ha : ∀ e ∈ a, e.WF)
    (hb : ∀ e ∈ b, e.WF) (h : pvOf W a = pvOf W b) : a = b :=
  (encode_append_inj hl ha hb h).1

theorem pad_wf {B : Nat} {l : List Entry} (h : ∀ e ∈ l, e.WF) : ∀ e ∈ pad B l, e.WF := by
  intro e he
  simp only [pad, List.mem_append] at he
  rcases he with he | he
  · exact h e he
  · rw [List.eq_of_mem_replicate he]; exact nopE_wf

/-- The value of a Midnight sponge entry: inputs, then one output. -/
def spongeOK (X : Prims) (n : Nat) (vs : List Nat) : Prop :=
  vs.drop n = [X.frSponge (vs.take n)]

/-- Operation semantics over Fr on decoded values, relative to the statement `s` (for `PUB`).
`PATH` asserts membership, with private siblings. `DIGEST` maps an internal `sherlock-value/2`
digest (8 BabyBear words) to its public form `PoseidonFr(tag, pack d2)`. -/
def ValidOn (X : Prims) (s : List Nat) : Op → List Nat → Prop
  | .perm, [a, b, c, x, y, z] => X.frPerm (a, b, c) = (x, y, z)
  | .h2, [a, b, o] => o = X.frH2 a b
  | .sponge n, vs => spongeOK X n vs
  | .path d, [leaf, idx, root] => ∃ bits : List Bool, ∃ sibs : List Nat, bits.length = d ∧
      sibs.length = d ∧ horner 2 (bits.map (fun b => if b then 1 else 0)) = idx ∧
      merkleRoot X leaf bits sibs = root
  | .ecmul, [k, px, py, ox, oy] => X.jubjubMul k (px, py) = (ox, oy)
  | .ecmulg, [k, ox, oy] => X.jubjubMul k X.jubjubGen = (ox, oy)
  | .pub i, [v] => s[i]? = some v
  | .digest, vs => vs.length = 9 ∧ (∀ x ∈ vs.take 8, x < pBB) ∧
      vs.drop 8 = [X.frSponge [digestTag, pack (vs.take 8)]]
  | .nop, [] => True
  | _, _ => False
where
  /-- Domain tag of public digests. -/
  digestTag : Nat := 0x5348

/-- `Valid e`: the entry is a true statement about Fr (the reference semantics). -/
def Valid (X : Prims) (s : List Nat) (e : Entry) : Prop := ValidOn X s e.op (e.vals.map dec)

/-- The wrap's native replay: the same relation on the Fr recompositions of the limbs. -/
def ReplayGadget (X : Prims) (s : List Nat) (e : Entry) : Prop :=
  ValidOn X s e.op (e.vals.map decR)

/-- Lemma 15 (`replay_sound`): on canonical values native replay implies `Valid`. -/
theorem replay_sound {X : Prims} {s : List Nat} {e : Entry} (hc : ∀ v ∈ e.vals, Canon v)
    (h : ReplayGadget X s e) : Valid X s e := by
  have : e.vals.map decR = e.vals.map dec := List.map_congr_left (fun v hv => decR_canon (hc v hv))
  simpa [ReplayGadget, Valid, this] using h

/-- Lemma 19 (`nop_pad_valid`): padding entries are valid, so every entry of a padded table is
an entry of the table or valid. -/
theorem nop_pad_valid (X : Prims) (s : List Nat) : Valid X s nopE := by
  simp [Valid, nopE, ValidOn]

/-! ### The log tree and its onion digest (Poseidon2 over BabyBear) -/

/-- Domain tags of the onion. -/
def tagEmpty : Nat := 1
def tagStep : Nat := 2
def tagNode : Nat := 3
def tagEnt : Nat := 4

/-- The shape of the log: leaf onions and `TAG_NODE` merges. -/
inductive LogTree where
  | seg (l : List Entry)
  | node (a b : LogTree)

def LogTree.flatten : LogTree → List Entry
  | seg l => l
  | node a b => a.flatten ++ b.flatten

/-- The onion over a log given newest entry first. The empty onion is `p2 [TAG_EMPTY]`, a hash
output, not a raw constant (design finding: with a raw `TAG_EMPTY` the injectivity of the
digest would need a preimage assumption on `p2`). -/
def onionRev (X : Prims) : List Entry → List Nat
  | [] => X.p2 [tagEmpty]
  | e :: es => X.p2 (tagStep :: (onionRev X es ++ X.p2 (tagEnt :: e.enc)))

/-- The incremental onion, oldest entry first. -/
def onion (X : Prims) (l : List Entry) : List Nat := onionRev X l.reverse

/-- The onion is incremental: appending one entry is one step from the previous digest. -/
theorem onion_snoc (X : Prims) (l : List Entry) (e : Entry) :
    onion X (l ++ [e]) = X.p2 (tagStep :: (onion X l ++ X.p2 (tagEnt :: e.enc))) := by
  simp [onion, onionRev]

/-- The empty onion is a hash output, `P2(TAG_EMPTY)` (lead amendment 1). -/
theorem onion_empty (X : Prims) : onion X [] = X.p2 [tagEmpty] := rfl

/-- Digest of a log tree. -/
def LogTree.digest (X : Prims) : LogTree → List Nat
  | seg l => onion X l
  | node a b => X.p2 (tagNode :: (a.digest X ++ b.digest X))

/-- Every input of `p2` met while computing an onion (newest first). -/
def OnionIn (X : Prims) (Dom : List Nat → Prop) : List Entry → Prop
  | [] => Dom [tagEmpty]
  | e :: es => Dom (tagStep :: (onionRev X es ++ X.p2 (tagEnt :: e.enc))) ∧
      Dom (tagEnt :: e.enc) ∧ OnionIn X Dom es

/-- Every input of `p2` met while computing the digest of a tree. -/
def LogTree.DigestIn (X : Prims) (Dom : List Nat → Prop) : LogTree → Prop
  | seg l => OnionIn X Dom l.reverse
  | node a b => Dom (tagNode :: (a.digest X ++ b.digest X)) ∧ a.DigestIn X Dom ∧ b.DigestIn X Dom

/-- Carrier trees: at most `B` entries in total (the fixed capacity), every entry well-formed,
and every `p2` input in the carrier domain `Dom` of the small-field hash premise. -/
def LogTree.Carrier (X : Prims) (Dom : List Nat → Prop) (B : Nat) (t : LogTree) : Prop :=
  t.flatten.length ≤ B ∧ (∀ e ∈ t.flatten, e.WF) ∧ t.DigestIn X Dom

/-- Carrier-guarded injectivity of Poseidon2 over BabyBear (the form of premise A5). -/
def P2Inj (X : Prims) (Dom : List Nat → Prop) : Prop :=
  ∀ a b, Dom a → Dom b → X.p2 a = X.p2 b → a = b

theorem onionRev_length (X : Prims) : ∀ l : List Entry, (onionRev X l).length = 8
  | [] => X.p2_width _
  | _ :: _ => X.p2_width _

theorem onionRev_inj {X : Prims} {Dom : List Nat → Prop} (hinj : P2Inj X Dom) :
    ∀ {a b : List Entry}, (∀ e ∈ a, e.WF) → (∀ e ∈ b, e.WF) → OnionIn X Dom a →
      OnionIn X Dom b → onionRev X a = onionRev X b → a = b
  | [], [], _, _, _, _, _ => rfl
  | [], _ :: _, _, _, ha, hb, h => by
    have := hinj _ _ ha hb.1 h
    simp [tagEmpty, tagStep] at this
  | _ :: _, [], _, _, ha, hb, h => by
    have := hinj _ _ ha.1 hb h
    simp [tagEmpty, tagStep] at this
  | e :: a, f :: b, hwa, hwb, ha, hb, h => by
    have h1 := hinj _ _ ha.1 hb.1 h
    simp only [List.cons.injEq, true_and] at h1
    have h2 := List.append_inj h1 (by rw [onionRev_length, onionRev_length])
    have h3 := hinj _ _ ha.2.1 hb.2.1 h2.2
    simp only [List.cons.injEq, true_and] at h3
    have h4 := enc_append_inj (s := []) (t := []) (hwa e (by simp)) (hwb f (by simp))
      (by simpa using h3)
    have ih := onionRev_inj hinj (fun x hx => hwa x (by simp [hx]))
      (fun x hx => hwb x (by simp [hx])) ha.2.2 hb.2.2 h2.1
    rw [h4.1, ih]

theorem onionRev_cases (X : Prims) (l : List Entry) :
    (l = [] ∧ onionRev X l = X.p2 [tagEmpty]) ∨
    ∃ e es, l = e :: es ∧ onionRev X l = X.p2 (tagStep :: (onionRev X es ++ X.p2 (tagEnt :: e.enc))) := by
  cases l with
  | nil => exact Or.inl ⟨rfl, rfl⟩
  | cons e es => exact Or.inr ⟨e, es, rfl, rfl⟩

theorem seg_ne_node {X : Prims} {Dom : List Nat → Prop} (hinj : P2Inj X Dom) {l : List Entry}
    {u : List Nat} (hl : OnionIn X Dom l.reverse) (hu : Dom (tagNode :: u))
    (h : onion X l = X.p2 (tagNode :: u)) : False := by
  simp only [onion] at h
  rcases onionRev_cases X l.reverse with ⟨he, h'⟩ | ⟨e, es, he, h'⟩
  · rw [he] at hl h; have := hinj _ _ hl hu h; simp [tagEmpty, tagNode] at this
  · rw [he] at hl h; have := hinj _ _ hl.1 hu h; simp [tagStep, tagNode] at this

theorem digest_length (X : Prims) : ∀ t : LogTree, (t.digest X).length = 8
  | .seg l => onionRev_length X l.reverse
  | .node _ _ => X.p2_width _

/-- Lemma 18 (`onion_inj`, `digest_inj`): carrier trees with the same digest are equal. A theorem
from the small-field hash premise and the domain tags, not a premise. -/
theorem digest_inj {X : Prims} {Dom : List Nat → Prop} (hinj : P2Inj X Dom) :
    ∀ {a b : LogTree}, (∀ e ∈ a.flatten, e.WF) → (∀ e ∈ b.flatten, e.WF) → a.DigestIn X Dom →
      b.DigestIn X Dom → a.digest X = b.digest X → a = b
  | .seg l, .seg m, hwa, hwb, ha, hb, h => by
    have := onionRev_inj hinj (by simpa [LogTree.flatten] using hwa)
      (by simpa [LogTree.flatten] using hwb) ha hb h
    rw [List.reverse_inj.mp this]
  | .seg l, .node c d, _, _, ha, hb, h => (seg_ne_node hinj ha hb.1 h).elim
  | .node c d, .seg l, _, _, ha, hb, h => (seg_ne_node hinj hb ha.1 h.symm).elim
  | .node c d, .node c' d', hwa, hwb, ha, hb, h => by
    have h1 := hinj _ _ ha.1 hb.1 h
    simp only [List.cons.injEq, true_and] at h1
    have h2 := List.append_inj h1 (by rw [digest_length, digest_length])
    have hc := digest_inj hinj (fun e he => hwa e (by simp [LogTree.flatten, he]))
      (fun e he => hwb e (by simp [LogTree.flatten, he])) ha.2.1 hb.2.1 h2.1
    have hd := digest_inj hinj (fun e he => hwa e (by simp [LogTree.flatten, he]))
      (fun e he => hwb e (by simp [LogTree.flatten, he])) ha.2.2 hb.2.2 h2.2
    rw [hc, hd]

/-! ### Deferred or emulated: the split is a prover choice -/

/-- What leaf and node shards prove relative to a log. `RunOps ops s` is the leaves' relation over
the run's whole sequence of Fr operations, each read as an oracle answer. Each operation is either
deferred (`true`: it enters the log, tier D) or emulated inline in its leaf shard by tier E chips
(`false`: the shard proves `Valid`). -/
def RunWith (X : Prims) (RunOps : List Entry → List Nat → Prop) (log : List Entry)
    (s : List Nat) : Prop :=
  ∃ tr : List (Entry × Bool), RunOps (tr.map Prod.fst) s ∧
    (tr.filter (·.2)).map Prod.fst = log ∧ ∀ x ∈ tr, x.2 = false → Valid X s x.1

theorem filter_split (ops : List Entry) (defer : Entry → Bool) :
    ((ops.map (fun e => (e, defer e))).filter (·.2)).map Prod.fst = ops.filter defer := by
  induction ops with
  | nil => rfl
  | cons e es ih =>
    simp only [List.map_cons, List.filter_cons]
    cases defer e <;> simp [ih]

/-- Lemma 21 (`spill_equiv`): any split of a valid run gives `RunWith` of its deferred part. The
split does not enter the statement. -/
theorem spill_equiv {X : Prims} {RunOps : List Entry → List Nat → Prop} {s : List Nat}
    (ops : List Entry) (hrun : RunOps ops s) (hv : ∀ e ∈ ops, Valid X s e) (defer : Entry → Bool) :
    RunWith X RunOps (ops.filter defer) s := by
  have hm : (ops.map (fun e => (e, defer e))).map Prod.fst = ops := by
    simp [Function.comp_def]
  refine ⟨ops.map (fun e => (e, defer e)), by rw [hm]; exact hrun, ?_, ?_⟩
  · exact filter_split ops defer
  · intro x hx _
    obtain ⟨e, he, rfl⟩ := List.mem_map.mp hx
    exact hv e he

/-- The converse direction: a `RunWith` whose log is valid gives a run all of whose operations
are valid. -/
theorem runWith_all_valid {X : Prims} {RunOps : List Entry → List Nat → Prop} {log : List Entry}
    {s : List Nat} (h : RunWith X RunOps log s) (hlog : ∀ e ∈ log, Valid X s e) :
    ∃ ops, RunOps ops s ∧ ∀ e ∈ ops, Valid X s e := by
  obtain ⟨tr, hrun, hl, hemu⟩ := h
  refine ⟨_, hrun, ?_⟩
  intro e he
  obtain ⟨x, hx, rfl⟩ := List.mem_map.mp he
  cases hb : x.2 with
  | false => exact hemu x hx hb
  | true =>
    apply hlog
    rw [← hl]
    exact List.mem_map.mpr ⟨x, List.mem_filter.mpr ⟨hx, hb⟩, rfl⟩

/-- Lemma 20 (`discharge_sound`): a discharge shard that proves every entry of a log segment with
tier E turns `RunWith log s` into `RunWith [] s`. -/
theorem discharge_sound {X : Prims} {RunOps : List Entry → List Nat → Prop} {log : List Entry}
    {s : List Nat} (h : RunWith X RunOps log s) (hlog : ∀ e ∈ log, Valid X s e) :
    RunWith X RunOps [] s := by
  obtain ⟨ops, hrun, hv⟩ := runWith_all_valid h hlog
  have h0 : ops.filter (fun _ => false) = [] := by
    induction ops with
    | nil => rfl
    | cons _ _ ih => simp
  have := spill_equiv ops hrun hv (fun _ => false)
  rwa [h0] at this

end Clean.Sherlock.Bridge
