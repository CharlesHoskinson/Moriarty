import Clean.Sherlock.PartitionIndependence

/-!
O46 (kit STACKS, `sherlock-chip-stacks` STK-001): partition independence for the compound stack
effect. This file extends `PartitionIndependence.lean` (C3), whose `Effect` carries exactly one
`StackOp` per stack. The reference machine applies, per stack and step, an optional rewrite of the
top and then one operation (owner runtime: "Task rewrite of the pre-image top, before any
push/pop"; `BHelperValidateField` and `BHelperEqualField` rewrite the parent and push the child in
one step). That is not one `StackOp`, so P7 does not cover it.

`CStackOp` is the compound op: `rw : Option A` (the rewrite of the top, a no-op on the empty
stack), then `op : StackOp A`. `CEffect` carries one compound op per stack. The results are the
same as C3, for the compound machine: `leaf_tail_commutesC`, `leaf_flush_commutesC`,
`partition_independentC`, `partition_independent_flushC` and `cuts_agreeC`. The stack part needs no
side condition: the leaf window applies the same compound ops to the same lists. The single-op
machine embeds with `rw = none` (`CEffect.ofEffect`, `next_ofEffect`).
-/

namespace Clean.Sherlock.Partition

variable {C F T R : Type}

/-- One compound owner event on a stack: an optional rewrite of the top, then one operation. -/
structure CStackOp (A : Type) where
  rw : Option A
  op : StackOp A

/-- The rewrite of the top (`last_mut()`: no-op on the empty stack), then the operation. -/
def CStackOp.apply {A : Type} (c : CStackOp A) (l : List A) : List A :=
  c.op.apply (match c.rw with
    | none => l
    | some a => (StackOp.rewrite a).apply l)

/-- The effect of one step with compound stack ops. -/
structure CEffect (C F T R : Type) where
  app : List C
  fop : CStackOp F
  top : CStackOp T
  regs : R

/-- The single-op effect as a compound effect with no rewrite. -/
def CEffect.ofEffect (e : Effect C F T R) : CEffect C F T R :=
  ⟨e.app, ⟨none, e.fop⟩, ⟨none, e.top⟩, e.regs⟩

def LState.nextC (eff : LState C F T R → CEffect C F T R) (s : LState C F T R) : LState C F T R :=
  ⟨s.mem ++ (eff s).app, (eff s).fop.apply s.frames, (eff s).top.apply s.tasks, (eff s).regs⟩

theorem next_ofEffect (eff : LState C F T R → Effect C F T R) (s : LState C F T R) :
    s.nextC (fun s ↦ CEffect.ofEffect (eff s)) = s.next eff := rfl

def appsOfC (eff : LState C F T R → CEffect C F T R) : Nat → LState C F T R → List C
  | 0, _ => []
  | k + 1, s => (eff s).app ++ appsOfC eff k (s.nextC eff)

theorem iter_memC (eff : LState C F T R → CEffect C F T R) : ∀ (k : Nat) (s : LState C F T R),
    (iter (LState.nextC eff) k s).mem = s.mem ++ appsOfC eff k s
  | 0, s => by simp [iter, appsOfC]
  | k + 1, s => by
    show (iter (LState.nextC eff) k (s.nextC eff)).mem = _
    rw [iter_memC eff k]; simp [LState.nextC, appsOfC]

def Win.stepC {M : Type} (e : CEffect C F T R) (w : Win C M F T R) : Win C M F T R :=
  ⟨w.mem, w.buf ++ e.app, e.fop.apply w.frames, e.top.apply w.tasks, e.regs⟩

def runWinC {M : Type} (eff : LState C F T R → CEffect C F T R) :
    Nat → LState C F T R → Win C M F T R → Win C M F T R
  | 0, _, w => w
  | k + 1, s, w => runWinC eff k (s.nextC eff) (w.stepC (eff s))

theorem runWinC_eq {M : Type} (eff : LState C F T R → CEffect C F T R) : ∀ (k : Nat)
    (s : LState C F T R) (w : Win C M F T R), w.frames = s.frames → w.tasks = s.tasks →
    w.regs = s.regs → runWinC eff k s w = ⟨w.mem, w.buf ++ appsOfC eff k s,
      (iter (LState.nextC eff) k s).frames, (iter (LState.nextC eff) k s).tasks,
      (iter (LState.nextC eff) k s).regs⟩
  | 0, s, ⟨m, b, f, t, r⟩, hf, ht, hr => by simp_all [runWinC, iter, appsOfC]
  | k + 1, s, w, hf, ht, hr => by
    rw [runWinC, runWinC_eq eff k (s.nextC eff) (w.stepC (eff s))
      (by simp [Win.stepC, LState.nextC, hf]) (by simp [Win.stepC, LState.nextC, ht]) rfl]
    simp [Win.stepC, iter, appsOfC]

def leafC {M : Type} (close : Win C M F T R → PState M F T R)
    (eff : LState C F T R → CEffect C F T R) (k : Nat) (s : LState C F T R) (P : PState M F T R) :
    PState M F T R :=
  close (runWinC eff k s (openW P))

/-- A leaf of compound steps maps `abs s` to `abs (nextC^[k] s)` (per-cell rule). -/
theorem leaf_tail_commutesC (eff : LState C F T R → CEffect C F T R) (k : Nat)
    (s : LState C F T R) : leafC closeTail eff k s (abs s) = abs (iter (LState.nextC eff) k s) := by
  rw [leafC, runWinC_eq eff k s _ rfl rfl rfl]
  simp only [closeTail, abs, openW, List.nil_append, iter_memC, canon_append]

/-- The same for the circuit rule, when the leaf has at most 8 appends. -/
theorem leaf_flush_commutesC (eff : LState C F T R → CEffect C F T R) (k : Nat)
    (s : LState C F T R) (h : (appsOfC eff k s).length ≤ 8) :
    leafC closeFlush eff k s (abs s) = abs (iter (LState.nextC eff) k s) := by
  rw [← leaf_tail_commutesC, leafC, leafC, runWinC_eq eff k s _ rfl rfl rfl]
  simp only [closeFlush, closeTail, abs, openW, List.nil_append]
  rw [leafFlush_eq_of_wf (canon_wf _) _ h]

def runPartC {M : Type} (close : Win C M F T R → PState M F T R)
    (eff : LState C F T R → CEffect C F T R) : List Nat → LState C F T R → PState M F T R →
    PState M F T R
  | [], _, P => P
  | k :: ks, s, P => runPartC close eff ks (iter (LState.nextC eff) k s) (leafC close eff k s P)

def FitsLeafC (eff : LState C F T R → CEffect C F T R) : List Nat → LState C F T R → Prop
  | [], _ => True
  | k :: ks, s => (appsOfC eff k s).length ≤ 8 ∧ FitsLeafC eff ks (iter (LState.nextC eff) k s)

/-- **O46, per-cell rule.** Partition independence for the compound stack effect: every cut of
the steps into leaves gives the canonical state after `ks.sum` compound steps. -/
theorem partition_independentC (eff : LState C F T R → CEffect C F T R) : ∀ (ks : List Nat)
    (s : LState C F T R),
    runPartC closeTail eff ks s (abs s) = abs (iter (LState.nextC eff) ks.sum s)
  | [], s => rfl
  | k :: ks, s => by
    rw [runPartC, leaf_tail_commutesC, partition_independentC eff ks, List.sum_cons, iter_add]

/-- **O46, circuit rule.** The same for every cut whose leaves hold at most 8 appends each. -/
theorem partition_independent_flushC (eff : LState C F T R → CEffect C F T R) : ∀ (ks : List Nat)
    (s : LState C F T R), FitsLeafC eff ks s →
    runPartC closeFlush eff ks s (abs s) = abs (iter (LState.nextC eff) ks.sum s)
  | [], s, _ => rfl
  | k :: ks, s, ⟨hk, hks⟩ => by
    rw [runPartC, leaf_flush_commutesC eff k s hk, partition_independent_flushC eff ks _ hks,
      List.sum_cons, iter_add]

/-- Two cuts of the same N compound steps give the same physical state, hence the same digest. -/
theorem cuts_agreeC {D : Type} (digest : PState (Mem C) F T R → D)
    (eff : LState C F T R → CEffect C F T R) (s : LState C F T R) (ks₁ ks₂ : List Nat)
    (hsum : ks₁.sum = ks₂.sum) (h₁ : FitsLeafC eff ks₁ s) (h₂ : FitsLeafC eff ks₂ s) :
    digest (runPartC closeFlush eff ks₁ s (abs s)) = digest (runPartC closeFlush eff ks₂ s (abs s)) := by
  rw [partition_independent_flushC eff ks₁ s h₁, partition_independent_flushC eff ks₂ s h₂, hsum]

/-- The compound op is the reference list function: rewrite-then-push spills the rewritten parent
and installs the child; a rewrite before a pop is discarded with the popped top. -/
theorem compound_rw_push {A : Type} (a b x : A) (l : List A) :
    (CStackOp.mk (some a) (.push b)).apply (x :: l) = b :: a :: l := rfl

theorem compound_rw_pop {A : Type} (a x : A) (l : List A) :
    (CStackOp.mk (some a) .pop).apply (x :: l) = l := rfl

theorem compound_empty {A : Type} (a : A) :
    (CStackOp.mk (some a) .pop).apply ([] : List A) = [] ∧
    (CStackOp.mk (some a) .keep).apply ([] : List A) = [] := ⟨rfl, rfl⟩

end Clean.Sherlock.Partition
