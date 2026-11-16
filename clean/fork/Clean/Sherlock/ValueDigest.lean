/-!
VALUE DIGEST (task 2.5, `openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-value-digest`;
`research/sherlock-final-design-20261111/ARCHITECTURE.md` section 1.11, FP-4 row `vd`, the
`sherlock-value/2` forms; obligations O23 to O26 of the consolidated table).

The digest `vd` of a value is one hash `H` of a word list: the head `[2, form, arity (2 words)]`
then the fields (scalar limbs, child digests, `ty_d`, the ordinal, the fold accumulator). The
constructor of arity `m` folds its fields: `acc_0` is the zero digest, `acc_(k+1) = H(7, 2,
acc_k, vd(f_k))`, and the completion is `H(6, m, ty_d, c, acc_m)`. The tag word and the length
word of `P2` are functions of the head, so they are left out of the word list here (they add no
distinction beyond the head).

* O23 `pre_injective`: two nodes with the same preimage have the same form, scalars and child
  digests (structural; `UInt n` with `n < 2^128`, `Fr` limbs below `2^30`, ordinals and arities
  below `2^32`).
* O24 `vd_injective`: under `Inj H` (the collision-resistance premise, as `P2Inj`), equal digests
  imply equal values.
* O25 `vd_heap_eq`: the digest a heap representation gives (each record's digest from its form
  and the digests of the records it points to) equals the digest of the decoded tree, for every
  address layout; so relocated, shared and constant representations of one value have one `vd`.
* O26 `sponge_split`: a sponge started from the state after a constant prefix equals the full
  sponge over the prefix and the rest (an abstract absorb step). VALUE DIGEST does not use a
  constant-start mode (X.9 has no result, HSH-004), so O26 is the equivalence a later mode needs.
* O27 (VALUE DIGEST as a `Module` of `ModularComposition.lean`) is in `ValueDigestModule.lean`.
No `sorry`, no `axiom`.
-/

namespace Clean.Sherlock.ValueDigest

/-- A word of a preimage: a field word below the field size, or a digest. -/
inductive W (D : Type) where
  | n (k : Nat)
  | d (x : D)
  deriving DecidableEq

/-- `sherlock-value/2` values. -/
inductive Value (D : Type) where
  | fr (x : List Nat)
  | uint (n : Nat)
  | bool (b : Bool)
  | nil
  | cons (h t : Value D)
  | leaf (x : Value D)
  | branch (l r : Value D)
  | ctor (ty : D) (c : Nat) (fields : List (Value D))

/-- 16-bit little-endian limbs of `n` (8 limbs: `u128`). -/
def limbs8 (n : Nat) : List Nat :=
  [n % 65536, n / 65536 % 65536, n / 65536 ^ 2 % 65536, n / 65536 ^ 3 % 65536,
   n / 65536 ^ 4 % 65536, n / 65536 ^ 5 % 65536, n / 65536 ^ 6 % 65536, n / 65536 ^ 7 % 65536]

theorem limbs8_inj {a b : Nat} (ha : a < 2 ^ 128) (hb : b < 2 ^ 128) (h : limbs8 a = limbs8 b) :
    a = b := by
  simp only [limbs8, List.cons.injEq] at h
  obtain ⟨h0, h1, h2, h3, h4, h5, h6, h7, -⟩ := h
  have ka : a = a % 65536 + 65536 * (a / 65536 % 65536) + 65536 ^ 2 * (a / 65536 ^ 2 % 65536)
      + 65536 ^ 3 * (a / 65536 ^ 3 % 65536) + 65536 ^ 4 * (a / 65536 ^ 4 % 65536)
      + 65536 ^ 5 * (a / 65536 ^ 5 % 65536) + 65536 ^ 6 * (a / 65536 ^ 6 % 65536)
      + 65536 ^ 7 * (a / 65536 ^ 7 % 65536) := by omega
  have kb : b = b % 65536 + 65536 * (b / 65536 % 65536) + 65536 ^ 2 * (b / 65536 ^ 2 % 65536)
      + 65536 ^ 3 * (b / 65536 ^ 3 % 65536) + 65536 ^ 4 * (b / 65536 ^ 4 % 65536)
      + 65536 ^ 5 * (b / 65536 ^ 5 % 65536) + 65536 ^ 6 * (b / 65536 ^ 6 % 65536)
      + 65536 ^ 7 * (b / 65536 ^ 7 % 65536) := by omega
  rw [ka, kb, h0, h1, h2, h3, h4, h5, h6, h7]

/-- `u32` as two 16-bit limbs. -/
def u32w (x : Nat) : List Nat := [x % 65536, x / 65536]

theorem u32w_inj {a b : Nat} (h : u32w a = u32w b) : a = b := by
  simp only [u32w, List.cons.injEq] at h
  omega

section
variable {D : Type}

def head (form arity : Nat) : List (W D) := [.n 2, .n form] ++ (u32w arity).map .n

/-- A node of the digest table: the form and the words after the head. -/
inductive Node (D : Type) where
  | fr (x : List Nat)
  | uint (n : Nat)
  | bool (b : Bool)
  | nil
  | cons (h t : D)
  | leaf (x : D)
  | branch (l r : D)
  | step (acc f : D)
  | done (m : Nat) (ty : D) (c : Nat) (acc : D)

/-- The preimage word list of a node (FP-4 row `vd`). -/
def pre : Node D → List (W D)
  | .fr x => head 8 0 ++ x.map .n
  | .uint n => head 0 0 ++ (limbs8 n).map .n
  | .bool b => head 1 0 ++ [.n (if b then 1 else 0)]
  | .nil => head 2 0
  | .cons h t => head 3 2 ++ [.d h, .d t]
  | .leaf x => head 4 1 ++ [.d x]
  | .branch l r => head 5 2 ++ [.d l, .d r]
  | .step a f => head 7 2 ++ [.d a, .d f]
  | .done m ty c a => head 6 m ++ [.d ty] ++ (u32w c).map .n ++ [.d a]

/-- The ranges of the typed lanes (FP-1): `u128`, `fr` limbs of 30 bits (9 of them), `u32`. -/
def Ranged : Node D → Prop
  | .fr x => x.length = 9 ∧ ∀ k ∈ x, k < 2 ^ 30
  | .uint n => n < 2 ^ 128
  | _ => True

/-- The form and arity are the second and third head words, so the head fixes the constructor. -/
theorem head_eq {f g a b : Nat} (h : (head f a : List (W D)) = head g b) : f = g ∧ a = b := by
  simp only [head, u32w, List.map, List.cons_append, List.nil_append, List.cons.injEq,
    W.n.injEq] at h
  omega

private theorem map_n_inj {x y : List Nat} (h : (x.map W.n : List (W D)) = y.map W.n) : x = y := by
  induction x generalizing y with
  | nil => cases y <;> simp_all
  | cons a x ih =>
    cases y with
    | nil => simp at h
    | cons b y =>
      simp only [List.map, List.cons.injEq, W.n.injEq] at h
      rw [h.1, ih h.2]

/-- O23: encoding injectivity. Ranged nodes with equal preimages are equal. -/
theorem pre_injective (u v : Node D) (hu : Ranged u) (hv : Ranged v) (h : pre u = pre v) : u = v := by
  cases u <;> cases v <;>
    simp_all [pre, head, u32w, Ranged, List.cons_append] <;>
    first
    | (exact map_n_inj h)
    | (exact limbs8_inj hu hv (map_n_inj h))
    | (cases ‹Bool› <;> cases ‹Bool› <;> simp_all)
    | omega

/-! The digest of a value (FP-4): `H` is `P2` with its tag, `z` the zero digest `acc_0`. -/
section digest
variable (H : List (W D) → D) (z : D)

/-- The arity of a constructor value: its field count. -/
def fieldCount {D : Type} : List (Value D) → Nat
  | [] => 0
  | _ :: fs => fieldCount fs + 1

mutual
def vd : Value D → D
  | .fr x => H (pre (.fr x))
  | .uint n => H (pre (.uint n))
  | .bool b => H (pre (.bool b))
  | .nil => H (pre .nil)
  | .cons h t => H (pre (.cons (vd h) (vd t)))
  | .leaf x => H (pre (.leaf (vd x)))
  | .branch l r => H (pre (.branch (vd l) (vd r)))
  | .ctor ty c fs => H (pre (.done (fieldCount fs) ty c (accFrom z fs)))
/-- The fold accumulator after the fields, from `a`. -/
def accFrom : D → List (Value D) → D
  | a, [] => a
  | a, f :: fs => accFrom (H (pre (.step a (vd f)))) fs
end

/-- The fold over digests (what VALUE DIGEST computes from the field `vd` lanes). -/
def accD (a : D) : List D → D
  | [] => a
  | f :: fs => accD (H (pre (.step a f))) fs

theorem accFrom_eq (fs : List (Value D)) (a : D) : accFrom H z a fs = accD H a (fs.map (vd H z)) := by
  induction fs generalizing a with
  | nil => simp [accFrom, accD]
  | cons f fs ih => simp [accFrom, accD, ih]

theorem fieldCount_eq (fs : List (Value D)) : fieldCount fs = fs.length := by
  induction fs with
  | nil => simp [fieldCount]
  | cons f fs ih => simp [fieldCount, ih]

/-- O26: a sponge started from the state after a constant prefix `p` equals the full sponge over
`p ++ r` (absorb is any step function; the permutation is abstract). -/
theorem sponge_split {S A : Type} (absorb : S → A → S) (init : S) (p r : List A) :
    (p ++ r).foldl absorb init = r.foldl absorb (p.foldl absorb init) := List.foldl_append

/-! O25: layout independence. A memory is any map from addresses to records whose children are
addresses; `Repr R i v` says the record at `i` represents `v` (relocated, shared or a constant
arena alike). `Kit R lane` says every `vd` lane is what VALUE DIGEST computes from the record's
form and the lanes of its children. Then every lane is `vd` of the value it represents. -/

inductive Rec (D : Type) where
  | fr (x : List Nat)
  | uint (n : Nat)
  | bool (b : Bool)
  | nil
  | cons (h t : Nat)
  | leaf (x : Nat)
  | branch (l r : Nat)
  | ctor (ty : D) (c : Nat) (fs : List Nat)

inductive Repr (R : Nat → Rec D) : Nat → Value D → Prop
  | fr {i x} : R i = .fr x → Repr R i (.fr x)
  | uint {i n} : R i = .uint n → Repr R i (.uint n)
  | bool {i b} : R i = .bool b → Repr R i (.bool b)
  | nil {i} : R i = .nil → Repr R i .nil
  | cons {i a b h t} : R i = .cons a b → Repr R a h → Repr R b t → Repr R i (.cons h t)
  | leaf {i a x} : R i = .leaf a → Repr R a x → Repr R i (.leaf x)
  | branch {i a b l r} : R i = .branch a b → Repr R a l → Repr R b r → Repr R i (.branch l r)
  | ctor {i ty c js} {fs : List (Value D)} : R i = .ctor ty c js → (hl : js.length = fs.length) →
      (∀ k (hk : k < js.length), Repr R (js[k]) (fs[k]'(by omega))) → Repr R i (.ctor ty c fs)

/-- The node VALUE DIGEST hashes for the record at `i`, from the child lanes. -/
def nodeOf (lane : Nat → D) : Rec D → Node D
  | .fr x => .fr x
  | .uint n => .uint n
  | .bool b => .bool b
  | .nil => .nil
  | .cons a b => .cons (lane a) (lane b)
  | .leaf a => .leaf (lane a)
  | .branch a b => .branch (lane a) (lane b)
  | .ctor ty c js => .done js.length ty c (accD H z (js.map lane))

def Kit (R : Nat → Rec D) (lane : Nat → D) : Prop := ∀ i, lane i = H (pre (nodeOf H z lane (R i)))

private theorem map_eq_of_idx {α β : Type} (f : α → D) (g : β → D) (js : List α) (fs : List β)
    (hl : js.length = fs.length) (h : ∀ k (hk : k < js.length), f js[k] = g (fs[k]'(by omega))) :
    js.map f = fs.map g := by
  apply List.ext_getElem (by simp [hl])
  intro k h1 h2
  simp only [List.getElem_map]
  exact h k (by simpa using h1)

/-- O25: every lane of a kit-consistent memory is the digest of the value it represents. -/
theorem vd_heap_eq {R : Nat → Rec D} {lane : Nat → D} (hk : Kit H z R lane) {i : Nat} {v : Value D}
    (hr : Repr R i v) : lane i = vd H z v := by
  induction hr with
  | fr e => rw [hk, e]; simp [nodeOf, vd]
  | uint e => rw [hk, e]; simp [nodeOf, vd]
  | bool e => rw [hk, e]; simp [nodeOf, vd]
  | nil e => rw [hk, e]; simp [nodeOf, vd]
  | cons e _ _ iha ihb => rw [hk, e]; simp [nodeOf, vd, iha, ihb]
  | leaf e _ iha => rw [hk, e]; simp [nodeOf, vd, iha]
  | branch e _ _ iha ihb => rw [hk, e]; simp [nodeOf, vd, iha, ihb]
  | ctor e hl _ ih =>
    rw [hk, e]
    simp only [nodeOf, vd, accFrom_eq, fieldCount_eq]
    rw [map_eq_of_idx lane (vd H z) _ _ hl ih, hl]

/-- O25, layout independence: two representations of one value, in any two memories (relocated,
shared, or the constants arena against the data arena), have equal lanes. -/
theorem vd_layout_independent {R R' : Nat → Rec D} {lane lane' : Nat → D} (hk : Kit H z R lane)
    (hk' : Kit H z R' lane') {i j : Nat} {v : Value D} (hr : Repr R i v) (hr' : Repr R' j v) :
    lane i = lane' j := by
  rw [vd_heap_eq H z hk hr, vd_heap_eq H z hk' hr']

/-! O24: equal digests imply equal values, under the injectivity premise of `H` (`P2Inj`). -/

/-- The premise: `H` is injective (collision resistance, premise A5 `P2Inj`). -/
def Inj : Prop := ∀ x y, H x = H y → x = y

mutual
/-- The typed ranges of a value (FP-1): `u128`, nine 30-bit `fr` limbs. -/
def RV : Value D → Prop
  | .fr x => x.length = 9 ∧ ∀ k ∈ x, k < 2 ^ 30
  | .uint n => n < 2 ^ 128
  | .bool _ => True
  | .nil => True
  | .cons h t => RV h ∧ RV t
  | .leaf x => RV x
  | .branch l r => RV l ∧ RV r
  | .ctor _ _ fs => RVs fs
def RVs : List (Value D) → Prop
  | [] => True
  | f :: fs => RV f ∧ RVs fs
end

/-- The node VALUE DIGEST hashes for a value (child digests in place of children). -/
def nodeV : Value D → Node D
  | .fr x => .fr x
  | .uint n => .uint n
  | .bool b => .bool b
  | .nil => .nil
  | .cons h t => .cons (vd H z h) (vd H z t)
  | .leaf x => .leaf (vd H z x)
  | .branch l r => .branch (vd H z l) (vd H z r)
  | .ctor ty c fs => .done (fieldCount fs) ty c (accFrom H z z fs)

theorem vd_node (v : Value D) : vd H z v = H (pre (nodeV H z v)) := by
  cases v <;> simp [vd, nodeV]

theorem nodeV_ranged (v : Value D) (h : RV v) : Ranged (nodeV H z v) := by
  cases v <;> simp_all [nodeV, Ranged, RV]

theorem node_eq (hH : Inj H) {u v : Value D} (hu : RV u) (hv : RV v) (e : vd H z u = vd H z v) :
    nodeV H z u = nodeV H z v :=
  pre_injective _ _ (nodeV_ranged H z u hu) (nodeV_ranged H z v hv)
    (hH _ _ (by rw [← vd_node H z u, ← vd_node H z v]; exact e))

/-- O24: under `Inj H`, ranged values with equal `vd` are equal. -/
theorem vd_injective (hH : Inj H) (u : Value D) :
    ∀ v, RV u → RV v → vd H z u = vd H z v → u = v := by
  refine Value.rec
    (motive_1 := fun u => ∀ v, RV u → RV v → vd H z u = vd H z v → u = v)
    (motive_2 := fun fs => ∀ (gs : List (Value D)) (a b : D), RVs fs → RVs gs →
      fs.length = gs.length → accFrom H z a fs = accFrom H z b gs → a = b ∧ fs = gs)
    ?fr ?uint ?bool ?nil ?cons ?leaf ?branch ?ctor ?lnil ?lcons u
  case fr =>
    intro x v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp_all [nodeV]
  case uint =>
    intro x v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp_all [nodeV]
  case bool =>
    intro x v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp_all [nodeV]
  case nil =>
    intro v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp_all [nodeV]
  case cons =>
    intro h t ih1 ih2 v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp only [nodeV, reduceCtorEq] at hn
    case cons h' t' =>
      simp only [Node.cons.injEq] at hn
      simp only [RV] at hu hv
      rw [ih1 h' hu.1 hv.1 hn.1, ih2 t' hu.2 hv.2 hn.2]
  case leaf =>
    intro x ih v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp only [nodeV, reduceCtorEq] at hn
    case leaf x' =>
      simp only [Node.leaf.injEq] at hn
      simp only [RV] at hu hv
      rw [ih x' hu hv hn]
  case branch =>
    intro l r ih1 ih2 v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp only [nodeV, reduceCtorEq] at hn
    case branch l' r' =>
      simp only [Node.branch.injEq] at hn
      simp only [RV] at hu hv
      rw [ih1 l' hu.1 hv.1 hn.1, ih2 r' hu.2 hv.2 hn.2]
  case ctor =>
    intro ty c fs ih v hu hv e; have hn := node_eq H z hH hu hv e
    cases v <;> simp only [nodeV, reduceCtorEq] at hn
    case ctor ty' c' gs =>
      simp only [Node.done.injEq] at hn
      obtain ⟨hm, hty, hc, ha⟩ := hn
      simp only [RV] at hu hv
      have hl : fs.length = gs.length := by rw [← fieldCount_eq, ← fieldCount_eq, hm]
      rw [hty, hc, (ih gs z z hu hv hl ha).2]
  case lnil =>
    intro gs a b _ _ hl e
    cases gs with
    | nil => simp_all [accFrom]
    | cons _ _ => simp at hl
  case lcons =>
    intro f fs ihf ihfs gs a b hu hv hl e
    cases gs with
    | nil => simp at hl
    | cons g gs =>
      simp only [accFrom] at e
      simp only [RVs] at hu hv
      obtain ⟨h1, h2⟩ := ihfs gs _ _ hu.2 hv.2 (by simpa using hl) e
      have hs := hH _ _ h1
      have hn := pre_injective (D := D) (.step a (vd H z f)) (.step b (vd H z g)) trivial trivial hs
      simp only [Node.step.injEq] at hn
      exact ⟨hn.1, by rw [ihf g hu.1 hv.1 hn.2, h2]⟩

end digest
end

end Clean.Sherlock.ValueDigest
