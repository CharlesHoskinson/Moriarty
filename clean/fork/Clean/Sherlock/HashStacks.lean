/-
O11 (kit STACKS, `sherlock-chip-stacks` STK-001 to STK-003): the cached-top protocol of a
hash-chain stack, with the compound step (an optional rewrite of the top, then none, push or pop,
as one owner event), refines the reference list function.

The physical stack is `(count, top, below)`. The list is top first: `x :: l` is the stack whose
top is `x` above the suffix `l`. The depth-indexed chain is `chainD [] = E` and
`chainD (x :: l) = hc (l.length + 1) (chainD l) x`, so `below` is the chain of the suffix under
the cached top, with the depth word.

`Accepts` is the relation of one STACKS row (`crates/sherlock-leaf/src/stacks.rs`, `StacksAir`,
branch `kit-stacks`), with the count adder and the no-wrap check read over `Nat`. The only
premise is that `hc` is injective (collision resistance). `E` outside the range of `hc` is not
needed for this direction.

Theorem `step_refines`: if the pre cells abstract the list `L` and the row is accepted, then the
post cells abstract `refStep L q`. Corollaries: `count = 1 -> below = E` holds on every abstracted
state (`abs_count_one`), and a restore returns the record and the predecessor spilled at that
depth (`restore_returns_spilled`).
-/

namespace Clean.Sherlock.HashStacks

variable {X H : Type}

/-- The request of CORE: rewrite (`rw`, `rwt`), then `op` (`new` is the pushed child). -/
inductive Op where
  | none
  | push
  | pop
  deriving DecidableEq

structure Req (X : Type) where
  rw : Bool
  rwt : X
  op : Op
  new : X

/-- The rewrite of the top (`last_mut()`: a rewrite of an empty list changes nothing). -/
def rewr (q : Req X) : List X → List X
  | [] => []
  | x :: l => (if q.rw = true then q.rwt else x) :: l

/-- The reference list function (owner runtime: the rewrite, then `push`/`pop`; a pop of an empty
list changes nothing). -/
def refStep (q : Req X) (L : List X) : List X :=
  match q.op with
  | .none => rewr q L
  | .push => q.new :: rewr q L
  | .pop => (rewr q L).tail

/-- Depth-indexed chain of a suffix. -/
def chainD (hc : Nat → H → X → H) (E : H) : List X → H
  | [] => E
  | x :: l => hc (l.length + 1) (chainD hc E l) x

structure Phys (X H : Type) where
  c : Nat
  top : X
  below : H

/-- The cached-top abstraction. -/
def Abs (hc : Nat → H → X → H) (E : H) (zero : X) (p : Phys X H) : List X → Prop
  | [] => p.c = 0 ∧ p.top = zero ∧ p.below = E
  | x :: l => p.c = l.length + 1 ∧ p.top = x ∧ p.below = chainD hc E l

/-- The witness of one row: the hash enable, the hashed preimage `(d, b, x)` and the digest. -/
structure Wit (X H : Type) where
  hs : Bool
  d : Nat
  b : H
  x : X
  hi : H

/-- One accepted STACKS row (the constraints of `StacksAir::eval`). -/
structure Accepts (hc : Nat → H → X → H) (E : H) (zero : X)
    (pre : Phys X H) (q : Req X) (post : Phys X H) (w : Wit X H) : Prop where
  -- STK-002 shape of the pre cells
  shape0 : pre.c = 0 → pre.top = zero ∧ pre.below = E
  shape1 : pre.c = 1 → pre.below = E
  -- hash enable
  hs_def : w.hs = true ↔ ((q.op = .push ∧ 1 ≤ pre.c) ∨ (q.op = .pop ∧ 2 ≤ pre.c))
  -- the count adder with no wrap: post = pre + push - pop * (pre >= 1)
  adder : post.c + (if q.op = .pop ∧ 1 ≤ pre.c then 1 else 0) = pre.c + (if q.op = .push then 1 else 0)
  -- the new top
  top_none : q.op = .none → post.top = (if q.rw = true ∧ 1 ≤ pre.c then q.rwt else pre.top)
  top_push : q.op = .push → post.top = q.new
  top_pop_le1 : q.op = .pop → pre.c ≤ 1 → post.top = zero
  -- no hash: below kept
  keep : w.hs = false → post.below = pre.below
  -- push binds the preimage to (count, below, top*) and the digest to the new below
  push_lo : q.op = .push →
    w.d = pre.c ∧ w.b = pre.below ∧ w.x = (if q.rw = true ∧ 1 ≤ pre.c then q.rwt else pre.top) ∧ w.hi = post.below
  -- pop binds the preimage to the post cells and the digest to the old below
  pop_lo : q.op = .pop → w.d = post.c ∧ w.b = post.below ∧ w.x = post.top ∧ w.hi = pre.below
  -- the sponge call
  hash : w.hs = true → hc w.d w.b w.x = w.hi

theorem abs_count_one {hc : Nat → H → X → H} {E : H} {zero : X} {p : Phys X H} {L : List X}
    (ha : Abs hc E zero p L) (h1 : p.c = 1) : p.below = E := by
  cases L with
  | nil => exact ha.2.2
  | cons x l =>
    obtain ⟨hc1, _, hb⟩ := ha
    have : l.length = 0 := by omega
    rw [List.length_eq_zero_iff] at this
    subst this
    exact hb

/-- O11: the compound step on the cached top refines the reference list function. -/
theorem step_refines {hc : Nat → H → X → H} {E : H} {zero : X}
    (hinj : ∀ d b x d' b' x', hc d b x = hc d' b' x' → d = d' ∧ b = b' ∧ x = x')
    {pre post : Phys X H} {q : Req X} {w : Wit X H} {L : List X}
    (ha : Abs hc E zero pre L) (hacc : Accepts hc E zero pre q post w) :
    Abs hc E zero post (refStep q L) := by
  obtain ⟨_, _, hs_def, adder, top_none, top_push, top_pop_le1, keep, push_lo, pop_lo, hash⟩ := hacc
  cases L with
  | nil =>
    obtain ⟨h0, ht, hb⟩ := ha
    have hs : w.hs = false := by
      cases hw : w.hs
      · rfl
      · have := hs_def.mp hw; omega
    have hkeep := keep hs
    cases hop : q.op with
    | none =>
      have htop := top_none hop
      simp only [hop, h0, reduceCtorEq, false_and, if_false, Nat.add_zero] at adder
      simp only [refStep, hop, rewr, Abs]
      refine ⟨by omega, ?_, by rw [hkeep, hb]⟩
      rw [htop]; simp [h0, ht]
    | push =>
      have htop := top_push hop
      simp only [hop, h0, reduceCtorEq, false_and, if_false, if_true, Nat.add_zero] at adder
      simp only [refStep, hop, rewr, Abs, chainD, List.length_nil]
      exact ⟨by omega, htop, by rw [hkeep, hb]⟩
    | pop =>
      have htop := top_pop_le1 hop (by omega)
      simp only [hop, h0, reduceCtorEq, if_false, Nat.add_zero] at adder
      simp only [refStep, hop, rewr, Abs, List.tail_nil]
      refine ⟨by simp at adder; omega, htop, by rw [hkeep, hb]⟩
  | cons x l =>
    obtain ⟨hcnt, ht, hb⟩ := ha
    have hge : 1 ≤ pre.c := by omega
    have heff : (if q.rw = true ∧ 1 ≤ pre.c then q.rwt else pre.top) = (if q.rw = true then q.rwt else x) := by
      cases q.rw <;> simp [ht, hge]
    cases hop : q.op with
    | none =>
      have hs : w.hs = false := by
        cases hw : w.hs
        · rfl
        · have := hs_def.mp hw; simp [hop] at this
      have hkeep := keep hs
      have htop := top_none hop
      simp only [hop, reduceCtorEq, false_and, if_false, Nat.add_zero] at adder
      simp only [refStep, hop, rewr, Abs]
      exact ⟨by omega, by rw [htop, heff], by rw [hkeep, hb]⟩
    | push =>
      have hs : w.hs = true := hs_def.mpr (Or.inl ⟨hop, hge⟩)
      obtain ⟨hd, hbb, hx, hhi⟩ := push_lo hop
      have hh := hash hs
      have htop := top_push hop
      simp only [hop, reduceCtorEq, false_and, if_false, if_true, Nat.add_zero] at adder
      simp only [refStep, hop, rewr, Abs, chainD, List.length_cons]
      refine ⟨by omega, htop, ?_⟩
      rw [← hhi, ← hh, hd, hbb, hx, heff, hcnt, hb]
    | pop =>
      by_cases h2 : 2 ≤ pre.c
      · have hs : w.hs = true := hs_def.mpr (Or.inr ⟨hop, h2⟩)
        obtain ⟨hd, hbb, hx, hhi⟩ := pop_lo hop
        have hh := hash hs
        cases l with
        | nil => simp at hcnt; omega
        | cons y l' =>
          rw [hhi, hb] at hh
          simp only [chainD] at hh
          obtain ⟨e1, e2, e3⟩ := hinj _ _ _ _ _ _ hh
          simp only [refStep, hop, rewr, Abs, List.tail_cons]
          exact ⟨by rw [← hd, e1], by rw [← hx, e3], by rw [← hbb, e2]⟩
      · have hs : w.hs = false := by
          cases hw : w.hs
          · rfl
          · have := hs_def.mp hw; simp [hop] at this; omega
        have hkeep := keep hs
        have htop := top_pop_le1 hop (by omega)
        cases l with
        | cons y l' => simp at hcnt; omega
        | nil =>
          simp only [hop, hge, and_self, if_true, reduceCtorEq, if_false, Nat.add_zero] at adder
          simp only [refStep, hop, rewr, Abs, List.tail_cons]
          refine ⟨by simp at hcnt; omega, htop, ?_⟩
          rw [hkeep, hb]; rfl

/-- STK-003: a restore (a pop at count >= 2) returns the record and the predecessor spilled at
that depth: the post cells are the head and the chain of the rest of the suffix. -/
theorem restore_returns_spilled {hc : Nat → H → X → H} {E : H} {zero : X}
    (hinj : ∀ d b x d' b' x', hc d b x = hc d' b' x' → d = d' ∧ b = b' ∧ x = x')
    {pre post : Phys X H} {q : Req X} {w : Wit X H} {x y : X} {l' : List X}
    (ha : Abs hc E zero pre (x :: y :: l')) (hacc : Accepts hc E zero pre q post w)
    (hop : q.op = .pop) :
    post.c = l'.length + 1 ∧ post.top = y ∧ post.below = chainD hc E l' := by
  have h := step_refines hinj ha hacc
  simp only [refStep, hop, rewr, List.tail_cons, Abs] at h
  exact h

end Clean.Sherlock.HashStacks
