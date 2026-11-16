import Clean.Sherlock.PartitionIndependence

/-!
# MEM window and MEMORY BACKEND contract (obligations O14, O15, O16)

Tasks 1.3 and 2.4 of `openspec/changes/sherlock-modular-leaf-20261111` (kits MEM window and
MEMORY BACKEND dense; `FD/ARCHITECTURE.md` section 3, Lean obligations).

* **O14** (`FitsWindow W`): partition independence for the circuit rule of a window of `W` flush
  units, under the state-dependent capacity `r_pre + appends <= 8 W + 7` (MEM-006). The circuit
  closes a leaf by `leafFlushW W`: it moves up to `W` complete segments from the tail and the
  appends into the tree. `leafFlushW_eq` shows that this equals the per-cell rule inside the
  capacity, and `partition_independent_window` lifts it to every cut. `fitsLeaf_window1` shows
  that the C3 premise (at most 8 appends per leaf) is the special case `W = 1`.
* **O15** (single-step fit): when every rule appends at most `8 W` cells in one step, the cut
  into single steps fits the window at every `r_pre` (`ones_fit_window`), so a valid cut exists
  for every run.
* **O16** (cold-read soundness): a walk of `d` levels whose bits are the bits of `a` and that
  reaches the root of the tree `f` starts at the leaf `f a`, for any siblings, if the height-tagged
  node hash is injective (`walk_sound`). With an injective record hash the opened record is the
  stored record (`cold_read_sound`). Collision resistance is a hypothesis of the theorems, not
  an axiom.

Core Lean only. `MemContractAxioms.lean` prints the axioms of every theorem.
-/

namespace Clean.Sherlock.MemContract

open Clean.Sherlock.Partition

variable {C F T R : Type}

/-! ## O14: the window close rule with `W` flush units -/

/-- Split up to `n` complete segments of 8 cells off the front of `l`. -/
def splitSegs : Nat → List C → List (List C) × List C
  | 0, l => ([], l)
  | n + 1, l =>
    if 8 ≤ l.length then ((l.take 8) :: (splitSegs n (l.drop 8)).1, (splitSegs n (l.drop 8)).2)
    else ([], l)

theorem splitSegs_cells : ∀ (n : Nat) (l : List C),
    (splitSegs n l).1.flatten ++ (splitSegs n l).2 = l
  | 0, l => by simp [splitSegs]
  | n + 1, l => by
    unfold splitSegs
    split
    · simp only [List.flatten_cons, List.append_assoc]
      rw [splitSegs_cells n (l.drop 8), List.take_append_drop]
    · simp

theorem splitSegs_seg8 : ∀ (n : Nat) (l : List C), ∀ g ∈ (splitSegs n l).1, g.length = 8
  | 0, l => by simp [splitSegs]
  | n + 1, l => by
    unfold splitSegs
    split
    · intro g hg
      simp only [List.mem_cons] at hg
      rcases hg with rfl | hg
      · simp; omega
      · exact splitSegs_seg8 n _ g hg
    · simp

theorem splitSegs_tail : ∀ (n : Nat) (l : List C), l.length < 8 * (n + 1) →
    (splitSegs n l).2.length < 8
  | 0, l, h => by simp [splitSegs]; omega
  | n + 1, l, h => by
    unfold splitSegs
    split
    · exact splitSegs_tail n _ (by simp; omega)
    · simp; omega

/-- The circuit's close rule with `W` flush units (`chip/dense.rs`, `heap.rs` `SegAir` and
`FlushAir`): the tail and the appends, up to `W` complete segments flushed. -/
def leafFlushW (W : Nat) (m : Mem C) (app : List C) : Mem C :=
  ⟨m.segs ++ (splitSegs W (m.tail ++ app)).1, (splitSegs W (m.tail ++ app)).2⟩

/-- Inside the window's capacity the `W`-unit rule is the per-cell rule. -/
theorem leafFlushW_eq {W : Nat} {m : Mem C} (h : m.WF) (app : List C)
    (hcap : m.tail.length + app.length ≤ 8 * W + 7) : leafFlushW W m app = appendCells m app := by
  apply Mem.wf_unique
  · refine ⟨?_, ?_⟩
    · intro g hg
      simp only [leafFlushW, List.mem_append] at hg
      rcases hg with hg | hg
      · exact h.1 g hg
      · exact splitSegs_seg8 W _ g hg
    · exact splitSegs_tail W _ (by simp; omega)
  · exact appendCells_wf h app
  · rw [appendCells_cells]
    simp only [leafFlushW, Mem.cells, List.flatten_append, List.append_assoc]
    rw [splitSegs_cells]

/-- Close by the `W`-unit rule. -/
def closeFlushW (W : Nat) (w : Win C (Mem C) F T R) : PState (Mem C) F T R :=
  ⟨leafFlushW W w.mem w.buf, w.frames, w.tasks, w.regs⟩

/-- The window's capacity at every leaf of the cut: `r_pre + appends <= 8 W + 7`. -/
def FitsWindow (W : Nat) (eff : LState C F T R → Effect C F T R) : List Nat → LState C F T R → Prop
  | [], _ => True
  | k :: ks, s =>
    (canon s.mem).tail.length + (appsOf eff k s).length ≤ 8 * W + 7 ∧
      FitsWindow W eff ks (iter (LState.next eff) k s)

theorem leaf_window_commutes (W : Nat) (eff : LState C F T R → Effect C F T R) (k : Nat)
    (s : LState C F T R) (h : (canon s.mem).tail.length + (appsOf eff k s).length ≤ 8 * W + 7) :
    leaf (closeFlushW W) eff k s (abs s) = abs (iter (LState.next eff) k s) := by
  rw [← leaf_tail_commutes, leaf, leaf, runWin_eq eff k s _ rfl rfl rfl]
  simp only [closeFlushW, closeTail, abs, openW, List.nil_append]
  rw [leafFlushW_eq (canon_wf _) _ h]

/-- **O14: partition independence at the window's capacity.** For every cut whose leaves fit the
window of `W` flush units, the physical state after the leaves is the canonical state after
`ks.sum` steps. -/
theorem partition_independent_window (W : Nat) (eff : LState C F T R → Effect C F T R) :
    ∀ (ks : List Nat) (s : LState C F T R), FitsWindow W eff ks s →
    runPart (closeFlushW W) eff ks s (abs s) = abs (iter (LState.next eff) ks.sum s)
  | [], _, _ => rfl
  | k :: ks, s, ⟨hk, hks⟩ => by
    rw [runPart, leaf_window_commutes W eff k s hk, partition_independent_window W eff ks _ hks,
      List.sum_cons, iter_add]

/-- Two cuts that fit the window give the same physical state. -/
theorem cuts_agree_window (W : Nat) (eff : LState C F T R → Effect C F T R) (s : LState C F T R)
    (ks₁ ks₂ : List Nat) (hsum : ks₁.sum = ks₂.sum)
    (h₁ : FitsWindow W eff ks₁ s) (h₂ : FitsWindow W eff ks₂ s) :
    runPart (closeFlushW W) eff ks₁ s (abs s) = runPart (closeFlushW W) eff ks₂ s (abs s) := by
  rw [partition_independent_window W eff ks₁ s h₁, partition_independent_window W eff ks₂ s h₂, hsum]

theorem canon_tail_lt (l : List C) : (canon l).tail.length < 8 := (canon_wf l).2

/-- The C3 premise is the case `W = 1`: a leaf of at most 8 appends fits a one-unit window. -/
theorem fitsLeaf_window1 (eff : LState C F T R → Effect C F T R) :
    ∀ (ks : List Nat) (s : LState C F T R), FitsLeaf eff ks s → FitsWindow 1 eff ks s
  | [], _, _ => trivial
  | k :: ks, s, ⟨hk, hks⟩ => by
    refine ⟨?_, fitsLeaf_window1 eff ks _ hks⟩
    have := canon_tail_lt s.mem
    omega

/-- At `W = 1` the `W`-unit rule and the one-flush rule of C3 agree inside the capacity. -/
theorem leafFlushW_one {m : Mem C} (h : m.WF) (app : List C) (happ : app.length ≤ 8) :
    leafFlushW 1 m app = leafFlush m app := by
  rw [leafFlush_eq_of_wf h app happ, leafFlushW_eq h app (by have := h.2; omega)]

/-! ## O15: single-step fit -/

/-- **O15.** If every rule appends at most `8 W` cells in one step, the cut into single steps
fits the window at every `r_pre`, so a cut that the window accepts exists for every run. -/
theorem ones_fit_window (W : Nat) (eff : LState C F T R → Effect C F T R)
    (h : ∀ s, (eff s).app.length ≤ 8 * W) : ∀ (N : Nat) (s : LState C F T R),
    FitsWindow W eff (List.replicate N 1) s
  | 0, _ => trivial
  | N + 1, s => by
    refine ⟨?_, ones_fit_window W eff h N _⟩
    rw [appsOf_one]
    have := canon_tail_lt s.mem
    have := h s
    omega

/-- A step that appends more than `8 W + 7 - r_pre` cells has no fitting leaf: the per-rule
bound `8 W` is the largest bound that fits at every `r_pre` (`r_pre = 7`). -/
theorem step_bound_tight (W : Nat) (eff : LState C F T R → Effect C F T R) (s : LState C F T R)
    (ht : (canon s.mem).tail.length = 7) (hs : 8 * W < (eff s).app.length) :
    ¬ FitsWindow W eff [1] s := by
  intro h
  have := h.1
  rw [appsOf_one] at this
  omega

/-! ## O16: cold-read soundness -/

variable {H : Type}

/-- The root of the complete tree of depth `d` over the leaves `f 0 .. f (2^d - 1)`; the node of
height `h` is `node h x y` (heights 1 to `d`; the record leaf is height 0). -/
def rootOf (node : Nat → H → H → H) : Nat → (Nat → H) → H
  | 0, f => f 0
  | d + 1, f => node (d + 1) (rootOf node d f) (rootOf node d (fun i => f (2 ^ d + i)))

/-- The low `d` bits of `a`, least significant first (the walk's bit of level `l` is bit `l`). -/
def bitsOf : Nat → Nat → List Bool
  | 0, _ => []
  | d + 1, a => bitsOf d (a % 2 ^ d) ++ [decide (2 ^ d ≤ a)]

theorem bitsOf_length : ∀ (d a : Nat), (bitsOf d a).length = d
  | 0, _ => rfl
  | d + 1, a => by simp [bitsOf, bitsOf_length d]

/-- One walk level of height `h`: `(cur, sib)` ordered by the bit. -/
def stepUp (node : Nat → H → H → H) (h : Nat) (b : Bool) (sib cur : H) : H :=
  if b then node h sib cur else node h cur sib

/-- The walk from `cur` at height `h - 1` up the levels of `bits` with the siblings `sibs`. -/
def walk (node : Nat → H → H → H) : Nat → H → List Bool → List H → H
  | _, cur, [], _ => cur
  | _, cur, _, [] => cur
  | h, cur, b :: bs, s :: ss => walk node (h + 1) (stepUp node h b s cur) bs ss

theorem walk_snoc (node : Nat → H → H → H) : ∀ (bs : List Bool) (ss : List H) (h : Nat) (cur : H)
    (b : Bool) (s : H), bs.length = ss.length →
    walk node h cur (bs ++ [b]) (ss ++ [s]) = stepUp node (h + bs.length) b s (walk node h cur bs ss)
  | [], [], h, cur, b, s, _ => by simp [walk]
  | [], _ :: _, _, _, _, _, hl => by simp at hl
  | _ :: _, [], _, _, _, _, hl => by simp at hl
  | b' :: bs, s' :: ss, h, cur, b, s, hl => by
    simp only [List.cons_append, walk, List.length_cons]
    rw [walk_snoc node bs ss (h + 1) _ b s (by simpa using hl)]
    congr 1; omega

/-- The height-tagged node hash is injective (collision resistance of `H_NODE_h` across heights). -/
def NodeInj (node : Nat → H → H → H) : Prop :=
  ∀ h h' x y x' y', node h x y = node h' x' y' → h = h' ∧ x = x' ∧ y = y'

/-- **O16, the walk.** A walk of `d` levels from `v` with the bits of `a < 2^d` that reaches the
root of `f` starts at the leaf `f a`, whatever the siblings. -/
theorem walk_sound (node : Nat → H → H → H) (hinj : NodeInj node) :
    ∀ (d : Nat) (f : Nat → H) (a : Nat) (v : H) (sibs : List H), a < 2 ^ d → sibs.length = d →
    walk node 1 v (bitsOf d a) sibs = rootOf node d f → v = f a
  | 0, f, a, v, sibs, ha, _, hw => by
    have : a = 0 := by simp at ha; omega
    subst this
    simpa [bitsOf, walk, rootOf] using hw
  | d + 1, f, a, v, sibs, ha, hl, hw => by
    obtain ⟨ss, s, rfl⟩ : ∃ ss s, sibs = ss ++ [s] := by
      rcases List.eq_nil_or_concat sibs with rfl | ⟨ss, s, rfl⟩
      · simp at hl
      · exact ⟨ss, s, List.concat_eq_append ..⟩
    have hss : ss.length = d := by simp at hl; omega
    simp only [bitsOf] at hw
    rw [walk_snoc node _ ss 1 v _ s (by rw [bitsOf_length, hss]), bitsOf_length, rootOf] at hw
    have hlo : a % 2 ^ d < 2 ^ d := Nat.mod_lt _ (Nat.two_pow_pos d)
    by_cases hb : 2 ^ d ≤ a
    · simp only [stepUp, hb, decide_true, ite_true] at hw
      have hh := hinj _ _ _ _ _ _ hw
      have hr := walk_sound node hinj d (fun i => f (2 ^ d + i)) (a % 2 ^ d) v ss hlo hss hh.2.2
      have : 2 ^ d + a % 2 ^ d = a := by
        have : a < 2 * 2 ^ d := by rw [Nat.pow_succ] at ha; omega
        rw [Nat.mod_eq_sub_mod hb, Nat.mod_eq_of_lt (by omega)]; omega
      simpa [this] using hr
    · simp only [stepUp, hb, decide_false] at hw
      have hh := hinj _ _ _ _ _ _ hw
      have hr := walk_sound node hinj d f (a % 2 ^ d) v ss hlo hss hh.2.1
      rwa [Nat.mod_eq_of_lt (by omega)] at hr

/-- **O16, the cold read.** If the record hash and the node hash are injective, a cold read that
the walk opens against the root of the stored memory (`rec_hash (stored a)` at leaf `a`) with the
bits of `a < 2^d` returns the stored record: value and kind (the kind is a lane of the record). -/
theorem cold_read_sound {Rec : Type} (recHash : Rec → H) (hrec : ∀ r r', recHash r = recHash r' → r = r')
    (node : Nat → H → H → H) (hinj : NodeInj node) (d : Nat) (stored : Nat → Rec) (a : Nat)
    (r : Rec) (sibs : List H) (ha : a < 2 ^ d) (hl : sibs.length = d)
    (hw : walk node 1 (recHash r) (bitsOf d a) sibs = rootOf node d (fun i => recHash (stored i))) :
    r = stored a :=
  hrec _ _ (walk_sound node hinj d _ a _ sibs ha hl hw)

/-- The read is against `pre.root`: if the walk reaches the root of `f` and of `g`, the leaf is
the same in both, so opening against a root after a flush (a different tree) proves nothing about
the pre state unless the two trees agree at `a`. -/
theorem walk_root_determines (node : Nat → H → H → H) (hinj : NodeInj node) (d : Nat)
    (f g : Nat → H) (a : Nat) (v : H) (sibs : List H) (ha : a < 2 ^ d) (hl : sibs.length = d)
    (hf : walk node 1 v (bitsOf d a) sibs = rootOf node d f)
    (hg : walk node 1 v (bitsOf d a) sibs = rootOf node d g) : f a = g a := by
  rw [← walk_sound node hinj d f a v sibs ha hl hf, ← walk_sound node hinj d g a v sibs ha hl hg]

end Clean.Sherlock.MemContract
