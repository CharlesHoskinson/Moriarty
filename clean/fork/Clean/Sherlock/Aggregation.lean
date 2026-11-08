import Init

/-!
General aggregation model of the Sherlock recursion (task 2.12; SRC-017; design §3.6;
`tree.md` §4.2; `prover.md` §6b L-A..L-E).

Proofs are abstract certificates. `NodeCert` is the certificate tree of the uniform three-slot
node of `tree.md` F8: a left self slot, a leaf slot and a right self slot, each gated by a bit.
`off` is a disabled slot. `NodeCert.stmt S` is the node checker: it links the present slots in
slot order and refuses an empty node, a leaf with `steps` outside `[1, S]` and any boundary
mismatch. The linear chain is the tree whose right slot is always disabled (`chainOf`); its
theorems are corollaries of the tree theorems, not separate proofs.

Premises are the fields of `Premises` (the reference machine's step relation, digest injectivity
on the carrier domain, in-circuit leaf soundness) and the algebra laws of `AccAlgebra`. No
`axiom` is declared. The cryptographic step (a node proof under the self key yields the slot
statements it verified, i.e. the certificate tree) and the accumulator algebra stay SRC-005/006
test obligations and are the modelling premise of this file, as `tree.md` §4 states.
-/

set_option linter.unusedSectionVars false

namespace Clean.Sherlock.Aggregation

/-- A segment claim `(prog_d, pre_d, post_d, h_lo, h_hi)` over digests `D`. Heights are naturals;
`field_eq_exact` (L5) justifies that reading of the range-checked field lanes. -/
structure Seg (D : Type) where
  prog : D
  pre : D
  post : D
  lo : Nat
  hi : Nat
  deriving DecidableEq, Repr

/-- The leaf statement `(prog_d, pre_d, post_d, h_pre, steps)` of the S-slot leaf (SRC-014). -/
structure Leaf (D : Type) where
  prog : D
  pre : D
  post : D
  hpre : Nat
  steps : Nat
  deriving DecidableEq, Repr

variable {D : Type} [DecidableEq D]

/-- The identity segment `(p, x, x, h, h)`: what a disabled slot contributes. -/
def ident (p x : D) (h : Nat) : Seg D := ⟨p, x, x, h, h⟩

/-- `link a b` is defined iff the boundaries agree (design §3.6). -/
def link (a b : Seg D) : Option (Seg D) :=
  if a.prog = b.prog ∧ a.post = b.pre ∧ a.hi = b.lo then
    some ⟨a.prog, a.pre, b.post, a.lo, b.hi⟩
  else none

/-- `link` lifted to optional operands; an undefined operand stays undefined. -/
def linkOpt : Option (Seg D) → Option (Seg D) → Option (Seg D)
  | some a, some b => link a b
  | _, _ => none

/-- Left-to-right linkage of a non-empty segment sequence starting from `s`. -/
def linkFrom (s : Seg D) (l : List (Seg D)) : Option (Seg D) :=
  l.foldl (fun acc t ↦ linkOpt acc (some t)) (some s)

/-- Left-to-right linkage of a segment sequence; the empty sequence has no statement. -/
def linkAll : List (Seg D) → Option (Seg D)
  | [] => none
  | s :: rest => linkFrom s rest

/-- The segment a leaf certifies: `h_hi = h_pre + steps`. -/
def Leaf.seg (f : Leaf D) : Seg D := ⟨f.prog, f.pre, f.post, f.hpre, f.hpre + f.steps⟩

/-- The certificate tree of the uniform three-slot node (`tree.md` F8). `off` is a disabled
slot; it is not a certificate (`stmt` refuses it) and only appears as a child. -/
inductive NodeCert (D : Type) where
  | off
  | node (left : NodeCert D) (leaf : Option (Leaf D)) (right : NodeCert D)
  deriving DecidableEq, Repr

namespace NodeCert

def isOff : NodeCert D → Bool
  | off => true
  | node _ _ _ => false

/-- The leaf slot's contribution: nothing when disabled, the leaf segment when `1 ≤ steps ≤ S`,
refused otherwise. -/
def leafSlot (S : Nat) : Option (Leaf D) → Option (List (Seg D))
  | none => some []
  | some f => if 1 ≤ f.steps ∧ f.steps ≤ S then some [f.seg] else none

/-- The node checker. A disabled self slot contributes nothing, a present one its own
statement (refused if it has none); the present contributions are linked in slot order
(left, leaf, right); an empty node has no statement. -/
def stmt (S : Nat) : NodeCert D → Option (Seg D)
  | off => none
  | node L f R =>
    match (if L.isOff then some [] else (L.stmt S).map ([·])), leafSlot S f,
        (if R.isOff then some [] else (R.stmt S).map ([·])) with
    | some ls, some fs, some rs => linkAll (ls ++ fs ++ rs)
    | _, _, _ => none

/-- A self slot's contribution to its parent. -/
def slot (S : Nat) (c : NodeCert D) : Option (List (Seg D)) :=
  if c.isOff then some [] else (c.stmt S).map ([·])

theorem stmt_node (S : Nat) (L : NodeCert D) (f : Option (Leaf D)) (R : NodeCert D) :
    stmt S (node L f R) = match slot S L, leafSlot S f, slot S R with
      | some ls, some fs, some rs => linkAll (ls ++ fs ++ rs)
      | _, _, _ => none := rfl

/-- The leaves below a certificate, in slot order. -/
def leaves : NodeCert D → List (Leaf D)
  | off => []
  | node L f R => L.leaves ++ f.toList ++ R.leaves

/-- Depth of the certificate tree. No theorem below mentions it (L7). -/
def depth : NodeCert D → Nat
  | off => 0
  | node L _ R => max L.depth R.depth + 1

end NodeCert

/-- The linear chain: every node has its right slot disabled; the carried node is the left slot
and the new leaf the leaf slot (`ivc.rs`: `link(carried, leaf)`). -/
def chainOf : List (Leaf D) → NodeCert D
  | [] => .off
  | f :: rest => rest.foldl (fun c g ↦ .node c (some g) .off) (.node .off (some f) .off)

/-- A certificate `c` certifies `s` when the checker accepts it and every leaf below it was
accepted by the in-circuit leaf verifier. -/
def Certified (S : Nat) (leafValid : Leaf D → Prop) (c : NodeCert D) (s : Seg D) : Prop :=
  c.stmt S = some s ∧ ∀ f ∈ c.leaves, leafValid f

/-! ## The reference machine -/

/-- `Run step n s t`: exactly `n` steps of `step` from `s` to `t`. -/
inductive Run {State : Type} (step : State → State → Prop) : Nat → State → State → Prop where
  | refl (s : State) : Run step 0 s s
  | step {s m t : State} {n : Nat} : step s m → Run step n m t → Run step (n + 1) s t

theorem Run.trans {State : Type} {step : State → State → Prop} {n m : Nat} {a b c : State}
    (h₁ : Run step n a b) (h₂ : Run step m b c) : Run step (n + m) a c := by
  induction h₁ with
  | refl => simpa using h₂
  | step hs _ ih => rw [Nat.succ_add]; exact .step hs (ih h₂)

/-- A deterministic machine has unique runs of a given length. -/
theorem Run.unique {State : Type} {step : State → State → Prop}
    (det : ∀ s t u, step s t → step s u → t = u) {n : Nat} {a b c : State}
    (h₁ : Run step n a b) (h₂ : Run step n a c) : b = c := by
  induction h₁ generalizing c with
  | refl => cases h₂; rfl
  | step hs _ ih =>
    cases h₂ with
    | step hs' h₂' => exact ih (det _ _ _ hs hs' ▸ h₂')

/-- A deterministic machine whose halted states are terminal halts once: two runs from the same
state into halted states have the same length and the same end. -/
theorem Run.halt_unique {State : Type} {step : State → State → Prop} {halted : State → Prop}
    (det : ∀ s t u, step s t → step s u → t = u) (stuck : ∀ t u, halted t → ¬ step t u)
    {n m : Nat} {a b c : State} (h₁ : Run step n a b) (hb : halted b) (h₂ : Run step m a c)
    (hc : halted c) : n = m ∧ b = c := by
  induction h₁ generalizing m c with
  | refl =>
    cases h₂ with
    | refl => exact ⟨rfl, rfl⟩
    | step hs _ => exact absurd hs (stuck _ _ hb)
  | step hs _ ih =>
    cases h₂ with
    | refl => exact absurd hs (stuck _ _ hc)
    | step hs' h₂' =>
      obtain ⟨hn, hbc⟩ := ih hb (det _ _ _ hs hs' ▸ h₂') hc
      exact ⟨by omega, hbc⟩

/-- The premises of the composition theorems. Each field is a stated premise, not an axiom. -/
structure Premises (D : Type) where
  /-- Machine states of the general reference machine. -/
  State : Type
  /-- Committed programs. -/
  Prog : Type
  /-- Results read back by `acceptRun`. -/
  Result : Type
  /-- The reference machine's step relation, the program as parameter (SRC-017: one machine,
  every program). -/
  Step : Prog → State → State → Prop
  /-- Halted states. -/
  halted : State → Prop
  /-- The result of a halted state. -/
  result : State → Result
  /-- The state digest (Poseidon over the canonical carrier encoding). -/
  digest : State → D
  /-- The carrier domain: states with a canonical encoding, on which `digest` is injective. -/
  Carrier : State → Prop
  /-- The program digest `prog_d`. -/
  progDigest : Prog → D
  /-- Committed programs, on which `progDigest` is injective. -/
  Committed : Prog → Prop
  /-- The in-circuit leaf verifier accepts this leaf statement under the fixed leaf key. -/
  leafValid : Leaf D → Prop
  /-- Premise (collision resistance of the state digest on the carrier domain). -/
  digest_inj : ∀ s t, Carrier s → Carrier t → digest s = digest t → s = t
  /-- Premise (collision resistance of the program digest on committed programs). -/
  progDigest_inj : ∀ p q, Committed p → Committed q → progDigest p = progDigest q → p = q
  /-- Premise (leaf soundness, SAS O2 for the S-slot leaf): an accepted leaf statement has a
  witness run of exactly `steps` machine steps between carrier states with those digests, for the
  committed program the leaf names. -/
  leaf_sound : ∀ f, leafValid f → ∃ p s t, Committed p ∧ progDigest p = f.prog ∧
    Carrier s ∧ Carrier t ∧ digest s = f.pre ∧ digest t = f.post ∧ Run (Step p) f.steps s t

/-- Segment soundness: for every committed program with digest `s.prog`, a run of exactly
`s.hi - s.lo` steps between carrier states with digests `s.pre` and `s.post`. -/
def Sound (P : Premises D) (s : Seg D) : Prop :=
  s.lo ≤ s.hi ∧ ∀ p, P.Committed p → P.progDigest p = s.prog →
    ∃ a b, P.Carrier a ∧ P.Carrier b ∧ P.digest a = s.pre ∧ P.digest b = s.post ∧
      Run (P.Step p) (s.hi - s.lo) a b

/-! ## L2: linkage is associative with the identity segment as unit -/

theorem link_ident_left (a : Seg D) : link (ident a.prog a.pre a.lo) a = some a := by
  simp [link, ident]

theorem link_ident_right (a : Seg D) : link a (ident a.prog a.post a.hi) = some a := by
  simp [link, ident]

/-- A disabled slot is a unit only at the matching boundary: linking the identity at another
boundary is refused. -/
theorem link_ident_left_iff (p x : D) (h : Nat) (a : Seg D) :
    link (ident p x h) a = some a ↔ a.prog = p ∧ a.pre = x ∧ a.lo = h := by
  unfold link ident
  constructor
  · intro hl
    split at hl
    · rename_i hc
      exact ⟨hc.1.symm, hc.2.1.symm, hc.2.2.symm⟩
    · exact absurd hl (by simp)
  · rintro ⟨h1, h2, h3⟩
    subst h1 h2 h3
    simp

theorem link_eq_some_iff {a b c : Seg D} :
    link a b = some c ↔
      (a.prog = b.prog ∧ a.post = b.pre ∧ a.hi = b.lo) ∧ c = ⟨a.prog, a.pre, b.post, a.lo, b.hi⟩ := by
  unfold link
  split
  · rename_i hc
    constructor
    · intro h; cases h; exact ⟨hc, rfl⟩
    · rintro ⟨-, rfl⟩; rfl
  · rename_i hc
    constructor
    · intro h; cases h
    · rintro ⟨h, -⟩; exact absurd h hc

theorem link_eq_none_iff {a b : Seg D} :
    link a b = none ↔ ¬ (a.prog = b.prog ∧ a.post = b.pre ∧ a.hi = b.lo) := by
  unfold link
  split
  · rename_i hc
    constructor
    · intro h; cases h
    · intro h; exact absurd hc h
  · rename_i hc
    exact ⟨fun _ ↦ hc, fun _ ↦ rfl⟩

theorem link_assoc (a b c : Seg D) : linkOpt (link a b) (some c) = linkOpt (some a) (link b c) := by
  cases hab : link a b with
  | none =>
    rw [link_eq_none_iff] at hab
    cases hbc : link b c with
    | none => rfl
    | some u =>
      obtain ⟨-, rfl⟩ := link_eq_some_iff.mp hbc
      simp only [linkOpt]
      exact ((link_eq_none_iff (b := ⟨b.prog, b.pre, c.post, b.lo, c.hi⟩)).mpr hab).symm
  | some u =>
    obtain ⟨⟨h1, h2, h3⟩, rfl⟩ := link_eq_some_iff.mp hab
    cases hbc : link b c with
    | none =>
      rw [link_eq_none_iff] at hbc
      simp only [linkOpt]
      exact link_eq_none_iff.mpr fun ⟨x, y, z⟩ ↦ hbc ⟨h1 ▸ x, y, z⟩
    | some v =>
      obtain ⟨⟨h4, h5, h6⟩, rfl⟩ := link_eq_some_iff.mp hbc
      simp only [linkOpt]
      have e₁ : link (⟨a.prog, a.pre, b.post, a.lo, b.hi⟩ : Seg D) c =
          some ⟨a.prog, a.pre, c.post, a.lo, c.hi⟩ :=
        link_eq_some_iff.mpr ⟨⟨h1.trans h4, h5, h6⟩, rfl⟩
      have e₂ : link a (⟨b.prog, b.pre, c.post, b.lo, c.hi⟩ : Seg D) =
          some ⟨a.prog, a.pre, c.post, a.lo, c.hi⟩ :=
        link_eq_some_iff.mpr ⟨⟨h1, h2, h3⟩, rfl⟩
      rw [e₁, e₂]

theorem linkOpt_assoc : ∀ a b c : Option (Seg D), linkOpt (linkOpt a b) c = linkOpt a (linkOpt b c)
  | none, _, _ => rfl
  | some _, none, _ => rfl
  | some a, some b, none => by
    show linkOpt (link a b) none = none
    cases link a b <;> rfl
  | some a, some b, some c => link_assoc a b c

/-- Linked lengths add (L2, the additive height of Mina's statement). -/
theorem link_len {a b c : Seg D} (h : link a b = some c) (ha : a.lo ≤ a.hi) (hb : b.lo ≤ b.hi) :
    c.hi - c.lo = (a.hi - a.lo) + (b.hi - b.lo) ∧ c.lo ≤ c.hi := by
  unfold link at h
  split at h
  · rename_i hc
    cases h
    obtain ⟨-, -, h3⟩ := hc
    simp only
    omega
  · exact absurd h (by simp)

theorem link_fields {a b c : Seg D} (h : link a b = some c) :
    a.prog = b.prog ∧ a.post = b.pre ∧ a.hi = b.lo ∧
      c = ⟨a.prog, a.pre, b.post, a.lo, b.hi⟩ := by
  unfold link at h
  split at h
  · rename_i hc
    cases h
    exact ⟨hc.1, hc.2.1, hc.2.2, rfl⟩
  · exact absurd h (by simp)

theorem foldl_linkOpt (a b : Option (Seg D)) (l : List (Seg D)) :
    l.foldl (fun acc t ↦ linkOpt acc (some t)) (linkOpt a b) =
      linkOpt a (l.foldl (fun acc t ↦ linkOpt acc (some t)) b) := by
  induction l generalizing b with
  | nil => rfl
  | cons t rest ih => simp only [List.foldl_cons, linkOpt_assoc]; exact ih _

theorem linkAll_single (s : Seg D) : linkAll [s] = some s := rfl

theorem linkAll_cons (s : Seg D) (l : List (Seg D)) :
    linkAll (s :: l) = linkOpt (some s) (linkAll l) ∨ l = [] := by
  cases l with
  | nil => exact Or.inr rfl
  | cons t rest =>
    left
    simp only [linkAll, linkFrom, List.foldl_cons]
    have := foldl_linkOpt (some s) (some t) rest
    simpa [linkOpt] using this

/-- Linkage of a concatenation is the linkage of the two parts (both non-empty). -/
theorem linkAll_append {l₁ l₂ : List (Seg D)} (h₁ : l₁ ≠ []) (h₂ : l₂ ≠ []) :
    linkAll (l₁ ++ l₂) = linkOpt (linkAll l₁) (linkAll l₂) := by
  cases l₁ with
  | nil => exact absurd rfl h₁
  | cons s r₁ =>
    cases l₂ with
    | nil => exact absurd rfl h₂
    | cons t r₂ =>
      simp only [List.cons_append, linkAll, linkFrom, List.foldl_append, List.foldl_cons]
      exact foldl_linkOpt _ (some t) r₂

/-- Replacing a certified prefix by its statement does not change the linkage (left slot). -/
theorem linkAll_replace_left {l₁ : List (Seg D)} {s : Seg D} (h : linkAll l₁ = some s)
    (l₂ : List (Seg D)) : linkAll (l₁ ++ l₂) = linkAll (s :: l₂) := by
  have hne : l₁ ≠ [] := by rintro rfl; simp [linkAll] at h
  cases l₂ with
  | nil => simpa [linkAll, linkFrom] using h
  | cons t r =>
    rw [linkAll_append hne (by simp), h]
    rcases linkAll_cons s (t :: r) with h' | h'
    · exact h'.symm
    · cases h'

/-- Replacing a certified suffix by its statement does not change the linkage (right slot). -/
theorem linkAll_replace_right (l₁ : List (Seg D)) {l₂ : List (Seg D)} {s : Seg D}
    (h : linkAll l₂ = some s) : linkAll (l₁ ++ l₂) = linkAll (l₁ ++ [s]) := by
  have hne : l₂ ≠ [] := by rintro rfl; simp [linkAll] at h
  cases l₁ with
  | nil => simpa [linkAll, linkFrom] using h
  | cons t r =>
    rw [linkAll_append (by simp) hne, linkAll_append (by simp) (by simp), h, linkAll_single]

/-- A bracketing of a segment sequence: the fold order of a tree. -/
inductive Bracket (D : Type) where
  | one (s : Seg D)
  | both (l r : Bracket D)

def Bracket.fold : Bracket D → Option (Seg D)
  | one s => some s
  | both l r => linkOpt l.fold r.fold

def Bracket.segs : Bracket D → List (Seg D)
  | one s => [s]
  | both l r => l.segs ++ r.segs

theorem Bracket.segs_ne_nil (b : Bracket D) : b.segs ≠ [] := by
  cases b with
  | one s => simp [segs]
  | both l r => simp [segs, List.append_eq_nil_iff, segs_ne_nil l]

theorem Bracket.fold_eq_linkAll (b : Bracket D) : b.fold = linkAll b.segs := by
  induction b with
  | one s => rfl
  | both l r ihl ihr =>
    simp only [fold, segs, ihl, ihr]
    exact (linkAll_append l.segs_ne_nil r.segs_ne_nil).symm

/-- Schedule independence: two bracketings of the same segment sequence fold to the same
statement (the Lean form of Quint I5). -/
theorem bracket_independent (b₁ b₂ : Bracket D) (h : b₁.segs = b₂.segs) : b₁.fold = b₂.fold := by
  rw [Bracket.fold_eq_linkAll, Bracket.fold_eq_linkAll, h]

/-! ## L3: padding neutrality -/

namespace NodeCert

theorem slot_off (S : Nat) : slot S (off : NodeCert D) = some [] := rfl

theorem slot_node (S : Nat) (L : NodeCert D) (f : Option (Leaf D)) (R : NodeCert D) :
    slot S (node L f R) = (stmt S (node L f R)).map ([·]) := rfl

/-- A disabled self slot contributes nothing; a present one contributes exactly its statement. -/
theorem slot_cases (S : Nat) (c : NodeCert D) :
    (c = off ∧ slot S c = some []) ∨ (c ≠ off ∧ slot S c = (c.stmt S).map ([·])) := by
  cases c with
  | off => exact Or.inl ⟨rfl, rfl⟩
  | node L f R => exact Or.inr ⟨by simp, rfl⟩

/-- An all-disabled node is refused (F8: at least one of `has_L`, `has_leaf`, `has_R`). -/
theorem empty_node_refused (S : Nat) : stmt S (node (off : NodeCert D) none off) = none := rfl

/-- A unary node (only the left slot present) certifies exactly its child's statement. -/
theorem unary_left (S : Nat) (c : NodeCert D) : stmt S (node c none off) = stmt S c := by
  cases c with
  | off => rfl
  | node L f R =>
    rw [stmt_node, slot_node, slot_off]
    cases h : stmt S (node L f R) <;> simp [leafSlot, linkAll, linkFrom]

/-- A unary node (only the right slot present) certifies exactly its child's statement. -/
theorem unary_right (S : Nat) (c : NodeCert D) : stmt S (node off none c) = stmt S c := by
  cases c with
  | off => rfl
  | node L f R =>
    rw [stmt_node, slot_node, slot_off]
    cases h : stmt S (node L f R) <;> simp [leafSlot, linkAll, linkFrom]

/-- A leaf-only node certifies exactly the leaf segment when `1 ≤ steps ≤ S`. -/
theorem leaf_only (S : Nat) (f : Leaf D) (h1 : 1 ≤ f.steps) (h2 : f.steps ≤ S) :
    stmt S (node off (some f) off) = some f.seg := by
  rw [stmt_node, slot_off]
  simp [leafSlot, h1, h2, linkAll, linkFrom]

/-- A leaf with `steps = 0` or `steps > S` is refused wherever it sits. -/
theorem leaf_out_of_range (S : Nat) (L : NodeCert D) (f : Leaf D) (R : NodeCert D)
    (h : ¬ (1 ≤ f.steps ∧ f.steps ≤ S)) : stmt S (node L (some f) R) = none := by
  rw [stmt_node]
  simp only [leafSlot, h, if_false]
  cases slot S L <;> cases slot S R <;> rfl

end NodeCert

@[simp] theorem ident_prog (p x : D) (h : Nat) : (ident p x h).prog = p := rfl
@[simp] theorem ident_pre (p x : D) (h : Nat) : (ident p x h).pre = x := rfl
@[simp] theorem ident_post (p x : D) (h : Nat) : (ident p x h).post = x := rfl
@[simp] theorem ident_lo (p x : D) (h : Nat) : (ident p x h).lo = h := rfl
@[simp] theorem ident_hi (p x : D) (h : Nat) : (ident p x h).hi = h := rfl

/-- Linking the identity on the left succeeds exactly at the matching boundary and is neutral. -/
theorem link_ident_left_eq {p x : D} {h : Nat} {a u : Seg D} :
    link (ident p x h) a = some u ↔ (a.prog = p ∧ a.pre = x ∧ a.lo = h) ∧ u = a := by
  rw [link_eq_some_iff]
  constructor
  · rintro ⟨⟨h1, h2, h3⟩, rfl⟩
    simp only [ident_prog] at h1 h2 h3
    refine ⟨⟨h1.symm, h2.symm, h3.symm⟩, ?_⟩
    cases a; simp only [ident_prog, ident_pre, ident_lo] at h1 h2 h3 ⊢; subst h1 h2 h3; rfl
  · rintro ⟨⟨h1, h2, h3⟩, rfl⟩
    refine ⟨⟨h1.symm, h2.symm, h3.symm⟩, ?_⟩
    simp only [ident_prog, ident_pre, ident_lo]
    rw [← h1, ← h2, ← h3]

/-- Linking the identity on the right succeeds exactly at the matching boundary and is neutral. -/
theorem link_ident_right_eq {p x : D} {h : Nat} {a u : Seg D} :
    link a (ident p x h) = some u ↔ (a.prog = p ∧ a.post = x ∧ a.hi = h) ∧ u = a := by
  rw [link_eq_some_iff]
  constructor
  · rintro ⟨⟨h1, h2, h3⟩, rfl⟩
    simp only [ident_prog, ident_pre, ident_hi, ident_post, ident_lo] at h1 h2 h3 ⊢
    refine ⟨⟨h1, h2, h3⟩, ?_⟩
    cases a; simp only at h1 h2 h3 ⊢; subst h2 h3; rfl
  · rintro ⟨⟨h1, h2, h3⟩, rfl⟩
    refine ⟨⟨h1, h2, h3⟩, ?_⟩
    simp only [ident_post, ident_hi]
    rw [← h2, ← h3]

theorem linkOpt_some_eq_some {o : Option (Seg D)} {z s : Seg D} :
    linkOpt o (some z) = some s ↔ ∃ u, o = some u ∧ link u z = some s := by
  cases o with
  | none => simp [linkOpt]
  | some u => simp [linkOpt]

/-- The F8 gated reading of one node: each disabled slot is the identity at its incoming
boundary, and the node links `link(link(L, leaf), R)`. -/
def gated (prog pre : D) (lo : Nat) (L f R : Option (Seg D)) : Option (Seg D) :=
  let a := L.getD (ident prog pre lo)
  let b := f.getD (ident prog a.post a.hi)
  let c := R.getD (ident prog b.post b.hi)
  linkOpt (link a b) (some c)

/-- Without the non-empty constraint the gated node would certify any identity segment. -/
theorem gated_empty (prog pre : D) (lo : Nat) :
    gated prog pre lo none none none = some (ident prog pre lo) := by
  simp only [gated, Option.getD, ident_post, ident_hi]
  rw [linkOpt_some_eq_some]
  exact ⟨_, link_ident_left_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩,
    link_ident_left_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩⟩

/-- The gated reading and the present-slot linkage agree on a non-empty node: a disabled slot
is exactly the unit of L2 (padding neutrality). -/
theorem gated_eq_linkAll (L f R : Option (Seg D)) (hne : L.isSome ∨ f.isSome ∨ R.isSome)
    (s : Seg D) :
    gated s.prog s.pre s.lo L f R = some s ↔ linkAll (L.toList ++ f.toList ++ R.toList) = some s := by
  cases L with
  | none =>
    cases f with
    | none =>
      cases R with
      | none => simp at hne
      | some r =>
        -- only R
        simp only [gated, Option.getD, ident_post, ident_hi, Option.toList, List.nil_append,
          linkAll, linkFrom, List.foldl_nil, Option.some.injEq]
        rw [linkOpt_some_eq_some]
        constructor
        · rintro ⟨u, hu, hr⟩
          obtain ⟨-, rfl⟩ := link_ident_left_eq.mp hu
          obtain ⟨-, rfl⟩ := link_ident_left_eq.mp hr
          rfl
        · rintro rfl
          exact ⟨_, link_ident_left_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩,
            link_ident_left_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩⟩
    | some b =>
      cases R with
      | none =>
        -- only leaf
        simp only [gated, Option.getD, Option.toList, List.nil_append,
          List.append_nil, linkAll, linkFrom, List.foldl_nil, Option.some.injEq]
        rw [linkOpt_some_eq_some]
        constructor
        · rintro ⟨u, hu, hr⟩
          obtain ⟨-, rfl⟩ := link_ident_left_eq.mp hu
          obtain ⟨-, rfl⟩ := link_ident_right_eq.mp hr
          rfl
        · rintro rfl
          exact ⟨_, link_ident_left_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩,
            link_ident_right_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩⟩
      | some r =>
        -- leaf and R
        simp only [gated, Option.getD, Option.toList, List.nil_append,
          List.singleton_append, linkAll, linkFrom, List.foldl_cons, List.foldl_nil]
        rw [linkOpt_some_eq_some]
        simp only [linkOpt]
        constructor
        · rintro ⟨u, hu, hr⟩
          obtain ⟨-, rfl⟩ := link_ident_left_eq.mp hu
          exact hr
        · intro h
          obtain ⟨-, hs⟩ := link_eq_some_iff.mp h
          subst hs
          exact ⟨_, link_ident_left_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩, h⟩
  | some a =>
    cases f with
    | none =>
      cases R with
      | none =>
        -- only L
        simp only [gated, Option.getD, ident_post, ident_hi, Option.toList, List.append_nil,
          linkAll, linkFrom, List.foldl_nil, Option.some.injEq]
        rw [linkOpt_some_eq_some]
        constructor
        · rintro ⟨u, hu, hr⟩
          obtain ⟨-, rfl⟩ := link_ident_right_eq.mp hu
          obtain ⟨-, rfl⟩ := link_ident_right_eq.mp hr
          rfl
        · rintro rfl
          exact ⟨_, link_ident_right_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩,
            link_ident_right_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩⟩
      | some r =>
        -- L and R
        simp only [gated, Option.getD, Option.toList, List.append_nil,
          List.singleton_append, linkAll, linkFrom, List.foldl_cons, List.foldl_nil]
        rw [linkOpt_some_eq_some]
        simp only [linkOpt]
        constructor
        · rintro ⟨u, hu, hr⟩
          obtain ⟨-, rfl⟩ := link_ident_right_eq.mp hu
          exact hr
        · intro h
          obtain ⟨-, hs⟩ := link_eq_some_iff.mp h
          subst hs
          exact ⟨_, link_ident_right_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩, h⟩
    | some b =>
      cases R with
      | none =>
        -- L and leaf
        simp only [gated, Option.getD, Option.toList, List.append_nil,
          List.singleton_append, linkAll, linkFrom, List.foldl_cons, List.foldl_nil]
        rw [linkOpt_some_eq_some]
        simp only [linkOpt]
        constructor
        · rintro ⟨u, hu, hr⟩
          obtain ⟨-, rfl⟩ := link_ident_right_eq.mp hr
          exact hu
        · intro h
          obtain ⟨-, hs⟩ := link_eq_some_iff.mp h
          subst hs
          exact ⟨_, h, link_ident_right_eq.mpr ⟨⟨rfl, rfl, rfl⟩, rfl⟩⟩
      | some r =>
        -- all three
        simp only [gated, Option.getD, Option.toList, List.cons_append,
          List.nil_append, linkAll, linkFrom, List.foldl_cons, List.foldl_nil]
        exact Iff.rfl

/-! ## L7: the statement depends on the leaf sequence only, not on shape, depth or S -/

namespace NodeCert

theorem leaves_off : leaves (off : NodeCert D) = [] := rfl

/-- The statement of any certificate tree is the linkage of its leaf sequence. Shape and depth
do not appear. -/
theorem stmt_eq_linkAll_leaves (S : Nat) : ∀ (c : NodeCert D) (s : Seg D),
    c.stmt S = some s → linkAll (c.leaves.map Leaf.seg) = some s := by
  intro c
  induction c with
  | off => intro s h; cases h
  | node L f R ihL ihR =>
    intro s h
    rw [stmt_node] at h
    have hL : ∀ ls, slot S L = some ls → ∀ M : List (Seg D),
        linkAll (ls ++ M) = linkAll (L.leaves.map Leaf.seg ++ M) := by
      intro ls hls M
      rcases slot_cases S L with ⟨rfl, h'⟩ | ⟨-, h'⟩
      · rw [h'] at hls; cases hls; rfl
      · rw [h'] at hls
        cases hc : stmt S L with
        | none => rw [hc] at hls; cases hls
        | some u =>
          rw [hc] at hls; cases hls
          exact (linkAll_replace_left (ihL _ hc) M).symm
    have hR : ∀ rs, slot S R = some rs → ∀ M : List (Seg D),
        linkAll (M ++ rs) = linkAll (M ++ R.leaves.map Leaf.seg) := by
      intro rs hrs M
      rcases slot_cases S R with ⟨rfl, h'⟩ | ⟨-, h'⟩
      · rw [h'] at hrs; cases hrs; rfl
      · rw [h'] at hrs
        cases hc : stmt S R with
        | none => rw [hc] at hrs; cases hrs
        | some u =>
          rw [hc] at hrs; cases hrs
          exact (linkAll_replace_right M (ihR _ hc)).symm
    have hf : ∀ fs, leafSlot S f = some fs → fs = f.toList.map Leaf.seg := by
      intro fs hfs
      cases f with
      | none => cases hfs; rfl
      | some g =>
        simp only [leafSlot] at hfs
        split at hfs
        · cases hfs; rfl
        · cases hfs
    revert h
    cases hls : slot S L with
    | none => intro h; cases h
    | some ls =>
      cases hfs : leafSlot S f with
      | none => intro h; cases h
      | some fs =>
        cases hrs : slot S R with
        | none => intro h; cases h
        | some rs =>
          intro h
          simp only at h
          rw [leaves, List.map_append, List.map_append, List.append_assoc]
          rw [hf fs hfs, hR rs hrs, List.append_assoc, hL ls hls] at h
          exact h

/-- Two certificate trees with the same leaf sequence certify the same statement, whatever
their shapes and depths. -/
theorem same_leaves_same_stmt (S : Nat) (c₁ c₂ : NodeCert D) (h : c₁.leaves = c₂.leaves)
    {s₁ s₂ : Seg D} (h₁ : c₁.stmt S = some s₁) (h₂ : c₂.stmt S = some s₂) : s₁ = s₂ := by
  have e₁ := stmt_eq_linkAll_leaves S c₁ s₁ h₁
  have e₂ := stmt_eq_linkAll_leaves S c₂ s₂ h₂
  rw [h, e₂] at e₁
  cases e₁
  rfl

/-- The certified statement does not depend on S: the same tree accepted at two values of S
certifies the same segment. -/
theorem stmt_independent_of_S (S S' : Nat) (c : NodeCert D) {s s' : Seg D}
    (h : c.stmt S = some s) (h' : c.stmt S' = some s') : s = s' :=
  same_leaves_same_stmt S c c rfl h (by
    have := stmt_eq_linkAll_leaves S' c s' h'
    have e := stmt_eq_linkAll_leaves S c s h
    rw [e] at this
    cases this
    exact h)

/-- Raising S never refuses a certificate (the leaf bound is the only place S appears). -/
theorem stmt_mono_S {S S' : Nat} (hS : S ≤ S') : ∀ (c : NodeCert D) (s : Seg D),
    c.stmt S = some s → c.stmt S' = some s := by
  intro c
  induction c with
  | off => intro s h; cases h
  | node L f R ihL ihR =>
    intro s h
    rw [stmt_node] at h ⊢
    have hslot : ∀ (c : NodeCert D), (∀ s, c.stmt S = some s → c.stmt S' = some s) →
        ∀ ls, slot S c = some ls → slot S' c = some ls := by
      intro c ih ls hls
      cases c with
      | off => exact hls
      | node a b d =>
        rw [slot_node] at hls ⊢
        cases hc : stmt S (node a b d) with
        | none => rw [hc] at hls; cases hls
        | some u => rw [ih u hc]; rw [hc] at hls; exact hls
    have hleaf : ∀ fs, leafSlot S f = some fs → leafSlot S' f = some fs := by
      intro fs hfs
      cases f with
      | none => exact hfs
      | some g =>
        simp only [leafSlot] at hfs ⊢
        split at hfs
        · rename_i hg; cases hfs; rw [if_pos ⟨hg.1, Nat.le_trans hg.2 hS⟩]
        · cases hfs
    revert h
    cases hls : slot S L with
    | none => intro h; cases h
    | some ls =>
      cases hfs : leafSlot S f with
      | none => intro h; cases h
      | some fs =>
        cases hrs : slot S R with
        | none => intro h; cases h
        | some rs =>
          intro h
          rw [hslot L ihL ls hls, hleaf fs hfs, hslot R ihR rs hrs]
          exact h

end NodeCert

/-- The linear chain is a tree: `chainOf` builds only nodes with the right slot disabled. -/
theorem chainOf_leaves (f : Leaf D) (rest : List (Leaf D)) :
    (chainOf (f :: rest)).leaves = f :: rest := by
  suffices h : ∀ (c : NodeCert D) (l : List (Leaf D)),
      (l.foldl (fun c g ↦ NodeCert.node c (some g) .off) c).leaves = c.leaves ++ l by
    simpa [chainOf, NodeCert.leaves] using h (.node .off (some f) .off) rest
  intro c l
  induction l generalizing c with
  | nil => simp
  | cons g rest ih => simp [ih, NodeCert.leaves, List.append_assoc]

/-! ## L4: exactly-once coverage -/

/-- `Covers prog h x l hi post`: the leaves `l` tile `[h, hi)` contiguously from digest `x` to
digest `post`, each leaf starting where the previous one ended, every leaf non-empty, all for
program `prog`. -/
inductive Covers (prog : D) : Nat → D → List (Leaf D) → Nat → D → Prop where
  | nil (h : Nat) (x : D) : Covers prog h x [] h x
  | cons {h : Nat} {x : D} {hi : Nat} {post : D} {rest : List (Leaf D)} (f : Leaf D) :
      f.prog = prog → f.pre = x → f.hpre = h → 1 ≤ f.steps →
      Covers prog (h + f.steps) f.post rest hi post → Covers prog h x (f :: rest) hi post

theorem Covers.le {prog : D} {h : Nat} {x : D} {l : List (Leaf D)} {hi : Nat} {post : D}
    (c : Covers prog h x l hi post) : h ≤ hi := by
  induction c with
  | nil => exact Nat.le_refl _
  | cons f _ _ _ _ _ ih => omega

/-- Every leaf's height lies in `[h, hi)` and the leaf ends within `hi`. -/
theorem Covers.bounds {prog : D} {h : Nat} {x : D} {l : List (Leaf D)} {hi : Nat} {post : D}
    (c : Covers prog h x l hi post) : ∀ f ∈ l, h ≤ f.hpre ∧ f.hpre + f.steps ≤ hi := by
  induction c with
  | nil => intro f hf; cases hf
  | cons g _ _ hg _ c' ih =>
    intro f hf
    have hle := c'.le
    cases hf with
    | head => omega
    | tail _ hf' =>
      obtain ⟨h1, h2⟩ := ih f hf'
      omega

/-- The heights of the covering leaves are strictly increasing: a leaf is consumed at most once,
and only at its own height. -/
theorem Covers.pairwise {prog : D} {h : Nat} {x : D} {l : List (Leaf D)} {hi : Nat} {post : D}
    (c : Covers prog h x l hi post) : l.Pairwise (fun f g ↦ f.hpre + f.steps ≤ g.hpre) := by
  induction c with
  | nil => exact List.Pairwise.nil
  | cons g _ _ hg _ c' ih =>
    refine List.Pairwise.cons ?_ ih
    intro f hf
    exact (hg ▸ (c'.bounds f hf).1)

theorem Covers.hpre_lt {prog : D} {h : Nat} {x : D} {l : List (Leaf D)} {hi : Nat} {post : D}
    (c : Covers prog h x l hi post) : l.Pairwise (fun f g ↦ f.hpre < g.hpre) := by
  induction c with
  | nil => exact List.Pairwise.nil
  | cons g _ _ hg hs c' ih =>
    refine List.Pairwise.cons ?_ ih
    intro f hf
    have := (c'.bounds f hf).1
    omega

/-- No leaf certificate is used at two positions. -/
theorem Covers.nodup {prog : D} {h : Nat} {x : D} {l : List (Leaf D)} {hi : Nat} {post : D}
    (c : Covers prog h x l hi post) : l.Nodup :=
  List.Pairwise.imp (fun hlt heq ↦ by subst heq; exact Nat.lt_irrefl _ hlt) c.hpre_lt

/-- The covered heights partition `[h, hi)`: the leaf steps sum to the segment length. -/
theorem Covers.sum {prog : D} {h : Nat} {x : D} {l : List (Leaf D)} {hi : Nat} {post : D}
    (c : Covers prog h x l hi post) : h + (l.map Leaf.steps).sum = hi := by
  induction c with
  | nil => simp
  | cons g _ _ _ _ _ ih => simp only [List.map_cons, List.sum_cons]; omega

/-- Every leaf of a covering names the program. -/
theorem Covers.prog_fixed {prog : D} {h : Nat} {x : D} {l : List (Leaf D)} {hi : Nat}
    {post : D} (c : Covers prog h x l hi post) : ∀ f ∈ l, f.prog = prog := by
  induction c with
  | nil => intro f hf; cases hf
  | cons g hg _ _ _ _ ih =>
    intro f hf
    cases hf with
    | head => exact hg
    | tail _ hf' => exact ih f hf'

/-- The linkage of a leaf sequence covers its segment. -/
theorem linkAll_covers : ∀ (l : List (Leaf D)) (s : Seg D), (∀ f ∈ l, 1 ≤ f.steps) →
    linkAll (l.map Leaf.seg) = some s → Covers s.prog s.lo s.pre l s.hi s.post := by
  intro l
  induction l with
  | nil => intro s _ h; cases h
  | cons f rest ih =>
    intro s hs h
    cases rest with
    | nil =>
      simp only [List.map_cons, List.map_nil, linkAll_single, Option.some.injEq] at h
      subst h
      exact .cons f rfl rfl rfl (hs f (by simp)) (.nil _ _)
    | cons g rest' =>
      rcases linkAll_cons f.seg ((g :: rest').map Leaf.seg) with h' | h'
      · rw [List.map_cons] at h
        rw [h'] at h
        cases hr : linkAll ((g :: rest').map Leaf.seg) with
        | none => rw [hr] at h; cases h
        | some u =>
          rw [hr] at h
          simp only [linkOpt] at h
          obtain ⟨h1, h2, h3, h4⟩ := link_fields h
          have hc := ih u (fun x hx ↦ hs x (List.mem_cons_of_mem _ hx)) hr
          subst h4
          simp only [Leaf.seg] at h1 h2 h3 hc ⊢
          refine .cons f rfl rfl rfl (hs f (by simp)) ?_
          rw [h2, h3, h1]
          exact hc
      · cases h'

namespace NodeCert

/-- Exactly-once coverage (L4): the leaves below a certified segment tile `[lo, hi)` contiguously
from `pre` to `post`, every leaf once, at its own height, for the one program. -/
theorem cert_covers (S : Nat) (c : NodeCert D) (s : Seg D) (h : c.stmt S = some s) :
    Covers s.prog s.lo s.pre c.leaves s.hi s.post := by
  apply linkAll_covers c.leaves s _ (stmt_eq_linkAll_leaves S c s h)
  -- every leaf below a certified node passed the `1 ≤ steps` check
  suffices hgen : ∀ (c : NodeCert D) (s : Seg D), c.stmt S = some s → ∀ f ∈ c.leaves, 1 ≤ f.steps from
    hgen c s h
  intro c
  induction c with
  | off => intro s h; cases h
  | node L f R ihL ihR =>
    intro s h g hg
    rw [stmt_node] at h
    revert h
    cases hls : slot S L with
    | none => intro h; cases h
    | some ls =>
      cases hfs : leafSlot S f with
      | none => intro h; cases h
      | some fs =>
        cases hrs : slot S R with
        | none => intro h; cases h
        | some rs =>
          intro _
          simp only [leaves, List.mem_append] at hg
          rcases hg with (hg | hg) | hg
          · rcases slot_cases S L with ⟨rfl, _⟩ | ⟨_, h'⟩
            · cases hg
            · rw [h'] at hls
              cases hc : stmt S L with
              | none => rw [hc] at hls; cases hls
              | some u => exact ihL u hc g hg
          · cases f with
            | none => cases hg
            | some f' =>
              simp only [Option.toList, List.mem_singleton] at hg
              subst hg
              simp only [leafSlot] at hfs
              split at hfs
              · rename_i hr; exact hr.1
              · cases hfs
          · rcases slot_cases S R with ⟨rfl, _⟩ | ⟨_, h'⟩
            · cases hg
            · rw [h'] at hrs
              cases hc : stmt S R with
              | none => rw [hc] at hrs; cases hrs
              | some u => exact ihR u hc g hg

/-- A certified segment is non-empty: every certificate tree has a leaf below it. -/
theorem cert_len_pos (S : Nat) (c : NodeCert D) (s : Seg D) (h : c.stmt S = some s) :
    s.lo < s.hi := by
  have hc := cert_covers S c s h
  cases hl : c.leaves with
  | nil =>
    rw [hl] at hc
    have := stmt_eq_linkAll_leaves S c s h
    rw [hl] at this
    cases this
  | cons f rest =>
    rw [hl] at hc
    cases hc with
    | cons _ _ _ hf hs c' => have := c'.le; omega

end NodeCert

/-! ## L1: segment soundness for every program -/

theorem sound_link {P : Premises D} {a b c : Seg D} (ha : Sound P a) (hb : Sound P b)
    (h : link a b = some c) : Sound P c := by
  obtain ⟨h1, h2, h3, rfl⟩ := link_fields h
  obtain ⟨hlen, hle⟩ := link_len h ha.1 hb.1
  refine ⟨hle, fun p hp hprog ↦ ?_⟩
  obtain ⟨x, y, hx, hy, hdx, hdy, run₁⟩ := ha.2 p hp hprog
  obtain ⟨y', z, hy', hz, hdy', hdz, run₂⟩ := hb.2 p hp (h1 ▸ hprog)
  have hyy : y = y' := P.digest_inj y y' hy hy' (by rw [hdy, hdy', h2])
  subst hyy
  refine ⟨x, z, hx, hz, hdx, hdz, ?_⟩
  simp only at hlen
  rw [hlen]
  exact run₁.trans run₂

theorem sound_linkAll {P : Premises D} : ∀ (l : List (Seg D)) (c : Seg D),
    (∀ s ∈ l, Sound P s) → linkAll l = some c → Sound P c := by
  intro l
  induction l with
  | nil => intro c _ h; cases h
  | cons s rest ih =>
    intro c hs h
    cases rest with
    | nil => rw [linkAll_single] at h; cases h; exact hs s (by simp)
    | cons t rest' =>
      rcases linkAll_cons s (t :: rest') with h' | h'
      · rw [h'] at h
        cases hr : linkAll (t :: rest') with
        | none => rw [hr] at h; cases h
        | some u =>
          rw [hr] at h
          exact sound_link (hs s (by simp)) (ih u (fun x hx ↦ hs x (List.mem_cons_of_mem _ hx)) hr) h
      · cases h'

/-- An accepted leaf statement with `1 ≤ steps` is sound. -/
theorem sound_leaf {P : Premises D} {f : Leaf D} (hv : P.leafValid f) : Sound P f.seg := by
  refine ⟨Nat.le_add_right _ _, fun p hp hprog ↦ ?_⟩
  obtain ⟨q, s, t, hq, hqd, hs, ht, hds, hdt, run⟩ := P.leaf_sound f hv
  have : q = p := P.progDigest_inj q p hq hp (by rw [hqd]; exact hprog.symm)
  subst this
  refine ⟨s, t, hs, ht, hds, hdt, ?_⟩
  simp only [Leaf.seg, Nat.add_sub_cancel_left]
  exact run

/-- **L1, segment soundness for every program.** The statement of a certified tree of any shape
and depth implies, for every committed program with digest `prog_d`, a run of the general
machine of exactly `h_hi - h_lo` steps between carrier states opening to `pre_d` and `post_d`.
Structural induction on `NodeCert`. -/
theorem segment_sound (P : Premises D) (S : Nat) : ∀ (c : NodeCert D) (s : Seg D),
    Certified S P.leafValid c s → Sound P s := by
  intro c
  induction c with
  | off => intro s h; cases h.1
  | node L f R ihL ihR =>
    intro s ⟨h, hv⟩
    rw [NodeCert.stmt_node] at h
    have hvL : ∀ g ∈ L.leaves, P.leafValid g := fun g hg ↦ hv g (by simp [NodeCert.leaves, hg])
    have hvR : ∀ g ∈ R.leaves, P.leafValid g := fun g hg ↦ hv g (by simp [NodeCert.leaves, hg])
    have hslot : ∀ (c : NodeCert D), (∀ s, Certified S P.leafValid c s → Sound P s) →
        (∀ g ∈ c.leaves, P.leafValid g) → ∀ ls, NodeCert.slot S c = some ls → ∀ x ∈ ls, Sound P x := by
      intro c ih hvc ls hls x hx
      rcases NodeCert.slot_cases S c with ⟨rfl, h'⟩ | ⟨_, h'⟩
      · rw [h'] at hls; cases hls; cases hx
      · rw [h'] at hls
        cases hc : c.stmt S with
        | none => rw [hc] at hls; cases hls
        | some u =>
          rw [hc] at hls; cases hls
          simp only [List.mem_singleton] at hx
          subst hx
          exact ih _ ⟨hc, hvc⟩
    have hleaf : ∀ fs, NodeCert.leafSlot S f = some fs → ∀ x ∈ fs, Sound P x := by
      intro fs hfs x hx
      cases f with
      | none => cases hfs; cases hx
      | some g =>
        simp only [NodeCert.leafSlot] at hfs
        split at hfs
        · cases hfs
          simp only [List.mem_singleton] at hx
          subst hx
          exact sound_leaf (hv g (by simp [NodeCert.leaves]))
        · cases hfs
    revert h
    cases hls : NodeCert.slot S L with
    | none => intro h; cases h
    | some ls =>
      cases hfs : NodeCert.leafSlot S f with
      | none => intro h; cases h
      | some fs =>
        cases hrs : NodeCert.slot S R with
        | none => intro h; cases h
        | some rs =>
          intro h
          refine sound_linkAll _ s ?_ h
          intro x hx
          simp only [List.mem_append] at hx
          rcases hx with (hx | hx) | hx
          · exact hslot L ihL hvL ls hls x hx
          · exact hleaf fs hfs x hx
          · exact hslot R ihR hvR rs hrs x hx

/-- The linear chain is a corollary of L1: a chain certificate is a tree certificate. -/
theorem chain_sound (P : Premises D) (S : Nat) (fs : List (Leaf D)) (s : Seg D)
    (h : Certified S P.leafValid (chainOf fs) s) : Sound P s :=
  segment_sound P S (chainOf fs) s h

/-! ## L6: decider pinning -/

/-- The decider's statement: a certified root that starts at genesis and whose final digest opens
as a halted carrier state with result `r`. -/
structure Decided (P : Premises D) (S : Nat) (root : Seg D) (r : P.Result) : Prop where
  cert : ∃ c, Certified S P.leafValid c root
  genesis : root.lo = 0
  halt : ∃ t, P.Carrier t ∧ P.digest t = root.post ∧ P.halted t ∧ P.result t = r

/-- `acceptRun`'s reads: the root is pinned to the sealed program and initial state, and its
`(h_hi, post_d)` are the published `(n_steps, fin_d)`. -/
def AcceptRun (programD initD : D) (root : Seg D) (nSteps : Nat) (finD : D) : Prop :=
  root.prog = programD ∧ root.pre = initD ∧ root.hi = nSteps ∧ root.post = finD

/-- A root for another program is refused by `acceptRun`. -/
theorem other_program_refused {programD initD : D} {root : Seg D} {nSteps : Nat} {finD : D}
    (h : root.prog ≠ programD) : ¬ AcceptRun programD initD root nSteps finD :=
  fun ha ↦ h ha.1

/-- A root for another initial state is refused by `acceptRun`. -/
theorem other_init_refused {programD initD : D} {root : Seg D} {nSteps : Nat} {finD : D}
    (h : root.pre ≠ initD) : ¬ AcceptRun programD initD root nSteps finD :=
  fun ha ↦ h ha.2.1

/-- A root that does not start at genesis is never decided. -/
theorem non_genesis_refused (P : Premises D) (S : Nat) (root : Seg D) (r : P.Result)
    (h : root.lo ≠ 0) : ¬ Decided P S root r :=
  fun hd ↦ h hd.genesis

/-- **L6, decider pinning.** A decided root accepted by `acceptRun` for the sealed program `p`
and the canonical initial state `s₀` is a run of exactly `n_steps` steps of the general machine
for `p` from `s₀` to a halted state whose digest is `fin_d` and whose result is `r`. -/
theorem decider_pinning (P : Premises D) (S : Nat) {root : Seg D} {r : P.Result}
    (hd : Decided P S root r) {programD initD : D} {nSteps : Nat} {finD : D}
    (ha : AcceptRun programD initD root nSteps finD) {p : P.Prog} (hp : P.Committed p)
    (hpd : P.progDigest p = programD) {s₀ : P.State} (hs₀ : P.Carrier s₀)
    (hd₀ : P.digest s₀ = initD) :
    ∃ t, Run (P.Step p) nSteps s₀ t ∧ P.halted t ∧ P.result t = r ∧ P.digest t = finD := by
  obtain ⟨c, hc⟩ := hd.cert
  obtain ⟨hprog, hpre, hn, hfin⟩ := ha
  obtain ⟨-, hsound⟩ := segment_sound P S c root hc
  obtain ⟨a, b, ha, hb, hda, hdb, run⟩ := hsound p hp (hpd.trans hprog.symm)
  obtain ⟨t, ht, hdt, hhalt, hr⟩ := hd.halt
  have hab : a = s₀ := P.digest_inj a s₀ ha hs₀ (by rw [hda, hd₀, hpre])
  have hbt : b = t := P.digest_inj b t hb ht (by rw [hdb, hdt])
  subst hab hbt
  refine ⟨b, ?_, hhalt, hr, hfin ▸ hdb⟩
  rw [hd.genesis, Nat.sub_zero, hn] at run
  exact run

/-- The accepted result is the halted value of the unique trace: for a deterministic machine with
terminal halted states, two decided roots for the same program and initial state publish the
same `(n_steps, fin_d, result)` (replay and L-C). -/
theorem result_unique (P : Premises D) (S₁ S₂ : Nat)
    (det : ∀ p s t u, P.Step p s t → P.Step p s u → t = u)
    (stuck : ∀ p t u, P.halted t → ¬ P.Step p t u)
    {root₁ root₂ : Seg D} {r₁ r₂ : P.Result}
    (hd₁ : Decided P S₁ root₁ r₁) (hd₂ : Decided P S₂ root₂ r₂)
    {programD initD : D} {n₁ n₂ : Nat} {fin₁ fin₂ : D}
    (ha₁ : AcceptRun programD initD root₁ n₁ fin₁) (ha₂ : AcceptRun programD initD root₂ n₂ fin₂)
    {p : P.Prog} (hp : P.Committed p) (hpd : P.progDigest p = programD)
    {s₀ : P.State} (hs₀ : P.Carrier s₀) (hd₀ : P.digest s₀ = initD) :
    n₁ = n₂ ∧ fin₁ = fin₂ ∧ r₁ = r₂ := by
  obtain ⟨t₁, run₁, halt₁, hr₁, hf₁⟩ := decider_pinning P S₁ hd₁ ha₁ hp hpd hs₀ hd₀
  obtain ⟨t₂, run₂, halt₂, hr₂, hf₂⟩ := decider_pinning P S₂ hd₂ ha₂ hp hpd hs₀ hd₀
  obtain ⟨hn, ht⟩ := Run.halt_unique (det p) (stuck p) run₁ halt₁ run₂ halt₂
  subst ht
  exact ⟨hn, hf₁ ▸ hf₂ ▸ rfl, hr₁ ▸ hr₂ ▸ rfl⟩

/-- **L-C, statement independence from S.** The decider statement `(n_steps, fin_d, result)` of
a program and initial state is the same at every S (here S₁ and S₂, e.g. 1 and 8). -/
theorem statement_independent_of_S (P : Premises D) (S₁ S₂ : Nat)
    (det : ∀ p s t u, P.Step p s t → P.Step p s u → t = u)
    (stuck : ∀ p t u, P.halted t → ¬ P.Step p t u)
    {root₁ root₂ : Seg D} {r₁ r₂ : P.Result}
    (hd₁ : Decided P S₁ root₁ r₁) (hd₂ : Decided P S₂ root₂ r₂)
    (hprog : root₁.prog = root₂.prog) (hpre : root₁.pre = root₂.pre)
    {p : P.Prog} (hp : P.Committed p) (hpd : P.progDigest p = root₁.prog)
    {s₀ : P.State} (hs₀ : P.Carrier s₀) (hd₀ : P.digest s₀ = root₁.pre) :
    root₁.hi = root₂.hi ∧ root₁.post = root₂.post ∧ r₁ = r₂ :=
  result_unique P S₁ S₂ det stuck hd₁ hd₂ ⟨rfl, rfl, rfl, rfl⟩ ⟨hprog.symm, hpre.symm, rfl, rfl⟩
    hp hpd hs₀ hd₀

/-! ## L5: height arithmetic without wrap -/

/-- The SRC-004 check pair `prev_h < 2^64 - S`, `1 ≤ steps ≤ S` keeps the next height below
`2^64` (L-A, sufficiency; the lower bound `1 ≤ steps` is not needed here, it serves L4). -/
theorem height_no_wrap (S prevH steps : Nat) (hS : S ≤ 2 ^ 64) (h₁ : prevH < 2 ^ 64 - S)
    (h₂ : steps ≤ S) : prevH + steps < 2 ^ 64 := by
  omega

/-- The weaker pair `prev_h < 2^64`, `steps ≤ S` without the `h` check is insufficient: it admits
a next height of `2^64`, outside the u64 lane domain (L-A, insufficiency; `tree.md` F8 therefore
range-checks `h1`, `h2`, `h_hi` directly). -/
theorem weaker_pair_insufficient (S : Nat) (hS : 1 ≤ S) :
    ∃ prevH steps, prevH < 2 ^ 64 ∧ steps ≤ S ∧ ¬ prevH + steps < 2 ^ 64 :=
  ⟨2 ^ 64 - 1, 1, by omega, hS, by omega⟩

/-- Field addition of two range-checked lanes does not wrap when `2^65 ≤ p` (BLS12-381's `r` has
255 bits), so the field sum is the natural sum. -/
theorem field_sum_exact (p a b : Nat) (hp : 2 ^ 65 ≤ p) (ha : a < 2 ^ 64) (hb : b < 2 ^ 64) :
    (a + b) % p = a + b :=
  Nat.mod_eq_of_lt (by omega)

/-- The in-circuit equation `h₂ = h₁ + steps` over the field, with every lane range-checked
below `2^64`, is the natural-number equation used by L1-L4. -/
theorem field_eq_exact (p h₁ steps h₂ : Nat) (hp : 2 ^ 65 ≤ p) (h₁r : h₁ < 2 ^ 64)
    (hsr : steps < 2 ^ 64) (heq : (h₁ + steps) % p = h₂) : h₂ = h₁ + steps := by
  rw [field_sum_exact p h₁ steps hp h₁r hsr] at heq
  exact heq.symm

/-! ## L-D: the genesis scale-by-bit lemma -/

/-- The accumulator algebra as premises: a fold with a neutral element, scaling by a bit, and a
collapse that preserves the pairing-check value. -/
structure AccAlgebra (A V : Type) where
  zero : A
  add : A → A → A
  zero_add : ∀ a, add zero a = a
  scale : Bool → A → A
  scale_false : ∀ a, scale false a = zero
  scale_true : ∀ a, scale true a = a
  collapse : A → A
  pairing : A → V
  pairing_collapse : ∀ a, pairing (collapse a) = pairing a

/-- `accumulate` folds the inputs in order. -/
def AccAlgebra.accumulate {A V : Type} (alg : AccAlgebra A V) : List A → A
  | [] => alg.zero
  | a :: rest => alg.add a (alg.accumulate rest)

/-- At genesis (`not_genesis = 0`, `ivc.rs` step 3) the previous-proof accumulator is scaled away
and the fold equals the fold of the carried and leaf accumulators alone; the collapsed pairing
value agrees. -/
theorem genesis_scale_by_bit {A V : Type} (alg : AccAlgebra A V) (proofAcc carried leaf : A) :
    alg.accumulate [alg.scale false proofAcc, carried, leaf] = alg.accumulate [carried, leaf] ∧
    alg.pairing (alg.collapse (alg.accumulate [alg.scale false proofAcc, carried, leaf])) =
      alg.pairing (alg.accumulate [carried, leaf]) := by
  have h : alg.accumulate [alg.scale false proofAcc, carried, leaf] =
      alg.accumulate [carried, leaf] := by
    simp only [AccAlgebra.accumulate, alg.scale_false, alg.zero_add]
  exact ⟨h, by rw [h, alg.pairing_collapse]⟩

/-- Off genesis (`not_genesis = 1`) the previous proof is folded unchanged. -/
theorem non_genesis_keeps_proof {A V : Type} (alg : AccAlgebra A V) (proofAcc carried leaf : A) :
    alg.accumulate [alg.scale true proofAcc, carried, leaf] =
      alg.accumulate [proofAcc, carried, leaf] := by
  simp only [AccAlgebra.accumulate, alg.scale_true]

/-! ## L-E: `assert_lower_than_fixed` for a non-power-of-two bound -/

/-- `assert_lower_than_fixed` (`native_gadget.rs:275-306`) for `2^k ≤ bound < 2^(k+1)`: with
`diff = bound - 2^k`, the prover-chosen bit selects `x` or `x - diff` (field subtraction) and the
selected value is range-checked below `2^k`. For a field element `x < p` the check accepts
exactly `x < bound` (no lower bound on `p` beyond `x < p` is needed: when `x < diff` the first
disjunct already holds, so the wrapped `x - diff` is never the deciding branch). -/
theorem lower_than_fixed_correct (p k bound x : Nat)
    (hk : 2 ^ k ≤ bound) (hk' : bound < 2 ^ (k + 1)) (hx : x < p) :
    (x < 2 ^ k ∨ (x + (p - (bound - 2 ^ k))) % p < 2 ^ k) ↔ x < bound := by
  have h2 : 2 ^ (k + 1) = 2 ^ k * 2 := Nat.pow_succ ..
  rw [h2] at hk'
  have hd : bound - 2 ^ k < 2 ^ k := by omega
  rcases Nat.lt_or_ge x (bound - 2 ^ k) with hlt | hge
  · constructor
    · intro _; omega
    · intro _; left; omega
  · have hrew : x + (p - (bound - 2 ^ k)) = (x - (bound - 2 ^ k)) + p := by omega
    rw [hrew, Nat.add_mod_right, Nat.mod_eq_of_lt (by omega)]
    constructor
    · rintro (h | h) <;> omega
    · intro h; right; omega

end Clean.Sherlock.Aggregation
