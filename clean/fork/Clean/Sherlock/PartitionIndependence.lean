import Clean.Sherlock.Aggregation

/-!
Partition independence for dense tail-buffered memory (experiment C3, SRC-023;
`research/sherlock-runlength-consensus-20261111/CONSENSUS.md` Q2(d), `AUDIT.md` findings 8 and 10).

The logical machine state `LState` has an append-only memory (a list of cells), the `FRAMES`
and `TASKS` stacks (hash-chain stacks, abstracted as lists) and the remaining registers. The
reference step function `LState.next eff` applies the step's `Effect`: real appends, one stack
operation per stack, new registers. `len` is the memory length, so it advances only by real
appends.

The physical state `PState (Mem C)` holds the memory as flushed full 8-cell segments (the content
of the fixed-depth tree under `mem_root`) and the uncommitted tail (`tail_d`). A leaf opens the
pre-state into a window, buffers the appends of its steps and closes:

* `closeTail` flushes by the per-cell rule: a segment is flushed exactly when `floor(len / 8)`
  grows (`pushCell_flush_iff`).
* `closeFlush` is the circuit rule: one flush per leaf at most, iff `r_pre + used >= 8`.
  `leafFlush_eq` shows it equals the per-cell rule when `r_pre + used < 16`, which holds when the
  leaf has at most 8 appends (`r_pre < 8` by `Mem.WF`).
* `closePad` is the old D3 layout (`crates/mori-chip/src/m3.rs`, gate `m3 walk first`): every leaf
  writes one fresh aligned segment and sets `len_post = base + 8`.

Results:

* `leaf_tail_commutes`, `leaf_flush_commutes`: a leaf maps the canonical physical state of `s`
  to the canonical physical state of `next^[k] s`.
* `partition_independent`, `partition_independent_flush`: the physical state after a cut of the
  N steps into leaves is `abs (next^[N] s)` for every cut (for the circuit rule: every cut whose
  leaves hold at most 8 appends each). `cuts_agree` is the two-partition form.
* `pstep_det`, `leaf_run`, `premises_det`, `result_unique_tail`: the connection to
  `Aggregation.lean`. The physical step relation `PStep` is deterministic because the logical step
  is a function and `abs` is injective (`abs_inj`); a k-step leaf is a `Run` of exactly k physical
  steps. So the `det` premise of `result_unique` / `statement_independent_of_S` is discharged for
  any `Premises` whose states represent `PState` injectively.
* `d3_partition_dependent`, `d3_no_det_run`: under D3 padding the cuts `[2]` and `[1, 1]` of a
  two-append trace give different physical states, and no deterministic step relation makes every
  padded leaf from a reachable state a k-step `Run` (`tail_det_run` is the positive twin). This is audit finding 10's reading: D3 does not falsify
  `statement_independent_of_S`; it makes its `det` premise impossible to discharge at the physical
  level.

Still assumed (not proved here): `digest_inj` (collision resistance of the state digest, so that
equal physical states give equal digests and back), `stuck` (halted states are terminal), the
leaf circuit's soundness (`leaf_sound`, the modular-composition theorem of
`ModularComposition.lean` plus the module contracts), that the leaf has at most 8 appends (the
`FitsLeaf` side condition of the circuit rule), and the address bound `len < 2^64` (audit finding
3; the lists here are unbounded). `tail_d` canonicity (finding 8) is built into the list model:
`Mem.wf_unique` shows the `(segs, tail)` split is a function of the cells.
-/

namespace Clean.Sherlock.Partition

open Clean.Sherlock.Aggregation (Run Run.trans Run.unique Premises Decided AcceptRun result_unique)

/-- `iter f n a` applies `f` n times. -/
def iter {α : Type} (f : α → α) : Nat → α → α
  | 0, a => a
  | n + 1, a => iter f n (f a)

theorem iter_add {α : Type} (f : α → α) : ∀ (m n : Nat) (a : α),
    iter f (m + n) a = iter f n (iter f m a)
  | 0, n, a => by simp [iter]
  | m + 1, n, a => by rw [Nat.succ_add]; exact iter_add f m n (f a)

/-! ## Dense tail-buffered memory -/

/-- Memory as flushed full segments (under `mem_root`) and the uncommitted tail (`tail_d`). -/
structure Mem (C : Type) where
  segs : List (List C)
  tail : List C
  deriving DecidableEq, Repr

variable {C : Type}

/-- The memory length `len = 8 q + r`. -/
def Mem.len (m : Mem C) : Nat := 8 * m.segs.length + m.tail.length

/-- The cells in address order. -/
def Mem.cells (m : Mem C) : List C := m.segs.flatten ++ m.tail

/-- Well formed: every flushed segment is full, and the tail holds `r < 8` cells. -/
def Mem.WF (m : Mem C) : Prop := (∀ g ∈ m.segs, g.length = 8) ∧ m.tail.length < 8

/-- Append one cell; flush the tail as a full segment when it reaches 8 cells. -/
def pushCell (m : Mem C) (c : C) : Mem C :=
  if m.tail.length + 1 = 8 then ⟨m.segs ++ [m.tail ++ [c]], []⟩ else ⟨m.segs, m.tail ++ [c]⟩

/-- Append cells one by one (the per-cell flush rule). -/
def appendCells (m : Mem C) (l : List C) : Mem C := l.foldl pushCell m

/-- The canonical memory of a cell list. -/
def canon (l : List C) : Mem C := appendCells ⟨[], []⟩ l

theorem pushCell_cells (m : Mem C) (c : C) : (pushCell m c).cells = m.cells ++ [c] := by
  unfold pushCell Mem.cells; split <;> simp

theorem pushCell_wf {m : Mem C} (h : m.WF) (c : C) : (pushCell m c).WF := by
  obtain ⟨hs, ht⟩ := h
  unfold pushCell; split
  · refine ⟨fun g hg ↦ ?_, by simp⟩
    simp only [List.mem_append, List.mem_singleton] at hg
    rcases hg with hg | rfl
    · exact hs g hg
    · simp; omega
  · exact ⟨hs, by simp; omega⟩

theorem pushCell_len {m : Mem C} (h : m.WF) (c : C) : (pushCell m c).len = m.len + 1 := by
  obtain ⟨-, ht⟩ := h
  unfold pushCell Mem.len; split <;> simp <;> omega

/-- A segment is flushed exactly when `floor(len / 8)` grows. -/
theorem pushCell_flush_iff {m : Mem C} (h : m.WF) (c : C) :
    (pushCell m c).segs.length = m.segs.length + 1 ↔ m.len / 8 < (m.len + 1) / 8 := by
  obtain ⟨-, ht⟩ := h
  unfold pushCell Mem.len; split <;> simp <;> omega

theorem appendCells_cells (m : Mem C) (l : List C) : (appendCells m l).cells = m.cells ++ l := by
  induction l generalizing m with
  | nil => simp [appendCells]
  | cons c l ih =>
    show (appendCells (pushCell m c) l).cells = _
    rw [ih, pushCell_cells]; simp

theorem appendCells_wf {m : Mem C} (h : m.WF) (l : List C) : (appendCells m l).WF := by
  induction l generalizing m with
  | nil => exact h
  | cons c l ih => exact ih (pushCell_wf h c)

/-- `len` advances only by real appends. -/
theorem appendCells_len {m : Mem C} (h : m.WF) (l : List C) :
    (appendCells m l).len = m.len + l.length := by
  induction l generalizing m with
  | nil => simp [appendCells]
  | cons c l ih =>
    show (appendCells (pushCell m c) l).len = _
    rw [ih (pushCell_wf h c), pushCell_len h]; simp; omega

theorem appendCells_append (m : Mem C) (l l' : List C) :
    appendCells m (l ++ l') = appendCells (appendCells m l) l' := by
  simp [appendCells, List.foldl_append]

theorem canon_append (l l' : List C) : canon (l ++ l') = appendCells (canon l) l' :=
  appendCells_append _ l l'

theorem canon_wf (l : List C) : (canon l).WF :=
  appendCells_wf ⟨by simp, by simp⟩ l

theorem canon_cells (l : List C) : (canon l).cells = l := by
  have := appendCells_cells (⟨[], []⟩ : Mem C) l
  simpa [Mem.cells, canon] using this

theorem canon_len (l : List C) : (canon l).len = l.length := by
  rw [canon, appendCells_len ⟨by simp, by simp⟩]; simp [Mem.len]

theorem canon_inj {l l' : List C} (h : canon l = canon l') : l = l' := by
  rw [← canon_cells l, ← canon_cells l', h]

/-- Canonicity of the split (audit finding 8): a well-formed memory is determined by its cells. -/
theorem Mem.wf_unique : ∀ {m m' : Mem C}, m.WF → m'.WF → m.cells = m'.cells → m = m'
  | ⟨[], t⟩, ⟨[], t'⟩, _, _, h => by simp [Mem.cells] at h; rw [h]
  | ⟨[], t⟩, ⟨g' :: gs', t'⟩, hw, hw', h => by
    exfalso
    have h1 := congrArg List.length h
    have := hw'.1 g' (by simp)
    simp [Mem.cells] at h1; have := hw.2; simp at this; omega
  | ⟨g :: gs, t⟩, ⟨[], t'⟩, hw, hw', h => by
    exfalso
    have h1 := congrArg List.length h
    have := hw.1 g (by simp)
    simp [Mem.cells] at h1; have := hw'.2; simp at this; omega
  | ⟨g :: gs, t⟩, ⟨g' :: gs', t'⟩, hw, hw', h => by
    have hg : g.length = g'.length := by rw [hw.1 g (by simp), hw'.1 g' (by simp)]
    simp only [Mem.cells, List.flatten_cons, List.append_assoc] at h
    obtain ⟨rfl, hrest⟩ := List.append_inj h hg
    have ih := @Mem.wf_unique ⟨gs, t⟩ ⟨gs', t'⟩
      ⟨fun x hx ↦ hw.1 x (by simp [hx]), hw.2⟩ ⟨fun x hx ↦ hw'.1 x (by simp [hx]), hw'.2⟩
      (by simpa [Mem.cells] using hrest)
    simp only [Mem.mk.injEq] at ih
    rw [ih.1, ih.2]

/-- The circuit's close rule: at most one flush per leaf, iff `r_pre + used >= 8`. -/
def leafFlush (m : Mem C) (app : List C) : Mem C :=
  if 8 ≤ (m.tail ++ app).length then ⟨m.segs ++ [(m.tail ++ app).take 8], (m.tail ++ app).drop 8⟩
  else ⟨m.segs, m.tail ++ app⟩

theorem appendCells_noflush (s : List (List C)) : ∀ (t app : List C),
    t.length + app.length < 8 → appendCells ⟨s, t⟩ app = ⟨s, t ++ app⟩
  | t, [], _ => by simp [appendCells]
  | t, c :: app, h => by
    show appendCells (pushCell ⟨s, t⟩ c) app = _
    have : ¬ t.length + 1 = 8 := by simp at h; omega
    simp only [pushCell, this, ite_false]
    rw [appendCells_noflush s (t ++ [c]) app (by simp at h ⊢; omega)]; simp

/-- The one-flush circuit rule equals the per-cell rule while `r_pre + used < 16` (the audit's
per-leaf argument: `r_pre <= 7`, `used <= 8`). -/
theorem leafFlush_eq (s : List (List C)) : ∀ (t app : List C), t.length < 8 →
    t.length + app.length < 16 → leafFlush ⟨s, t⟩ app = appendCells ⟨s, t⟩ app
  | t, [], ht, _ => by simp [leafFlush, appendCells]; omega
  | t, c :: app, ht, h => by
    show _ = appendCells (pushCell ⟨s, t⟩ c) app
    by_cases h8 : t.length + 1 = 8
    · simp only [pushCell, h8, ite_true]
      rw [appendCells_noflush _ [] app (by simp at h ⊢; omega)]
      have ht' : (t ++ [c]).length = 8 := by simp; omega
      have hb : t ++ c :: app = (t ++ [c]) ++ app := by simp
      simp only [leafFlush, hb, List.take_left' ht', List.drop_left' ht']
      simp; omega
    · simp only [pushCell, h8, ite_false]
      rw [← leafFlush_eq s (t ++ [c]) app (by simp; omega) (by simp at h ⊢; omega)]
      simp [leafFlush]

theorem leafFlush_eq_of_wf {m : Mem C} (h : m.WF) (app : List C) (happ : app.length ≤ 8) :
    leafFlush m app = appendCells m app := by
  obtain ⟨s, t⟩ := m
  exact leafFlush_eq s t app h.2 (by have := h.2; simp at this ⊢; omega)

/-- The flush rule of the circuit in `floor` form: a leaf flushes iff `floor(len / 8)` grows. -/
theorem leafFlush_iff {m : Mem C} (h : m.WF) (app : List C) (happ : app.length ≤ 8) :
    (leafFlush m app).segs.length = m.segs.length + 1 ↔ m.len / 8 < (m.len + app.length) / 8 := by
  have ht := h.2
  unfold leafFlush Mem.len; split <;> simp_all <;> omega

/-! ## The reference machine -/

/-- One operation on a LIFO stack (`FRAMES`, `TASKS`). -/
inductive StackOp (A : Type) where
  | keep
  | push (a : A)
  | pop
  | rewrite (a : A)
  deriving DecidableEq, Repr

def StackOp.apply {A : Type} : StackOp A → List A → List A
  | keep, l => l
  | push a, l => a :: l
  | pop, l => l.tail
  | rewrite _, [] => []
  | rewrite a, _ :: l => a :: l

/-- The hash chain of a stack (`fp_hash`, `tp_hash`): `EMPTY` for the empty stack, else
`H(top, chain of the rest)`. -/
def chainHash {A H : Type} (h : A → H → H) (empty : H) : List A → H
  | [] => empty
  | x :: l => h x (chainHash h empty l)

/-- The list abstraction of a hash-chain stack is faithful under collision resistance of `h`
(injective) and `EMPTY` outside its range. Both are collision premises (audit finding 8: the
direction `fp_hash == EMPTY -> count == 0` is not enforceable otherwise). -/
theorem chainHash_inj {A H : Type} {h : A → H → H} {empty : H}
    (hinj : ∀ x y a b, h x a = h y b → x = y ∧ a = b) (hne : ∀ x a, h x a ≠ empty) :
    ∀ l l' : List A, chainHash h empty l = chainHash h empty l' → l = l'
  | [], [], _ => rfl
  | [], _ :: _, he => absurd he.symm (hne _ _)
  | _ :: _, [], he => absurd he (hne _ _)
  | x :: l, y :: l', he => by
    obtain ⟨rfl, hr⟩ := hinj _ _ _ _ he
    rw [chainHash_inj hinj hne l l' hr]

/-- The effect of one step: real appends, one operation per stack, the new registers. -/
structure Effect (C F T R : Type) where
  app : List C
  fop : StackOp F
  top : StackOp T
  regs : R

/-- The logical machine state. -/
structure LState (C F T R : Type) where
  mem : List C
  frames : List F
  tasks : List T
  regs : R
  deriving DecidableEq, Repr

variable {F T R : Type}

/-- The reference step function: the step's effect is a function of the logical state. -/
def LState.next (eff : LState C F T R → Effect C F T R) (s : LState C F T R) : LState C F T R :=
  ⟨s.mem ++ (eff s).app, (eff s).fop.apply s.frames, (eff s).top.apply s.tasks, (eff s).regs⟩

/-- The cells appended by `k` steps from `s`. -/
def appsOf (eff : LState C F T R → Effect C F T R) : Nat → LState C F T R → List C
  | 0, _ => []
  | k + 1, s => (eff s).app ++ appsOf eff k (s.next eff)

theorem iter_mem (eff : LState C F T R → Effect C F T R) : ∀ (k : Nat) (s : LState C F T R),
    (iter (LState.next eff) k s).mem = s.mem ++ appsOf eff k s
  | 0, s => by simp [iter, appsOf]
  | k + 1, s => by
    show (iter (LState.next eff) k (s.next eff)).mem = _
    rw [iter_mem eff k]; simp [LState.next, appsOf]

/-- Logical partition independence: running the leaves of any cut runs `ks.sum` steps. -/
def runLogical (eff : LState C F T R → Effect C F T R) :
    List Nat → LState C F T R → LState C F T R
  | [], s => s
  | k :: ks, s => runLogical eff ks (iter (LState.next eff) k s)

theorem runLogical_eq (eff : LState C F T R → Effect C F T R) : ∀ (ks : List Nat)
    (s : LState C F T R), runLogical eff ks s = iter (LState.next eff) ks.sum s
  | [], s => rfl
  | k :: ks, s => by rw [runLogical, runLogical_eq eff ks, List.sum_cons, iter_add]

/-! ## The physical state and the leaf -/

/-- The physical state over a memory representation `M`. -/
structure PState (M F T R : Type) where
  mem : M
  frames : List F
  tasks : List T
  regs : R
  deriving DecidableEq, Repr

/-- The canonical physical state of a logical state (tail-buffered memory). -/
def abs (s : LState C F T R) : PState (Mem C) F T R := ⟨canon s.mem, s.frames, s.tasks, s.regs⟩

theorem abs_inj {s s' : LState C F T R} (h : abs s = abs s') : s = s' := by
  obtain ⟨m, f, t, r⟩ := s
  obtain ⟨m', f', t', r'⟩ := s'
  simp only [abs, PState.mk.injEq] at h
  obtain ⟨hm, rfl, rfl, rfl⟩ := h
  rw [canon_inj hm]

/-- The leaf window: the opened pre-state memory, the buffered appends, the stacks and registers. -/
structure Win (C M F T R : Type) where
  mem : M
  buf : List C
  frames : List F
  tasks : List T
  regs : R

def Win.step {M : Type} (e : Effect C F T R) (w : Win C M F T R) : Win C M F T R :=
  ⟨w.mem, w.buf ++ e.app, e.fop.apply w.frames, e.top.apply w.tasks, e.regs⟩

/-- The leaf runs `k` reference steps of the witness trace from `s` on the window. -/
def runWin {M : Type} (eff : LState C F T R → Effect C F T R) :
    Nat → LState C F T R → Win C M F T R → Win C M F T R
  | 0, _, w => w
  | k + 1, s, w => runWin eff k (s.next eff) (w.step (eff s))

def openW {M : Type} (P : PState M F T R) : Win C M F T R :=
  ⟨P.mem, [], P.frames, P.tasks, P.regs⟩

theorem runWin_eq {M : Type} (eff : LState C F T R → Effect C F T R) : ∀ (k : Nat)
    (s : LState C F T R) (w : Win C M F T R), w.frames = s.frames → w.tasks = s.tasks →
    w.regs = s.regs → runWin eff k s w = ⟨w.mem, w.buf ++ appsOf eff k s,
      (iter (LState.next eff) k s).frames, (iter (LState.next eff) k s).tasks,
      (iter (LState.next eff) k s).regs⟩
  | 0, s, ⟨m, b, f, t, r⟩, hf, ht, hr => by simp_all [runWin, iter, appsOf]
  | k + 1, s, w, hf, ht, hr => by
    rw [runWin, runWin_eq eff k (s.next eff) (w.step (eff s))
      (by simp [Win.step, LState.next, hf]) (by simp [Win.step, LState.next, ht]) rfl]
    simp [Win.step, iter, appsOf]

/-- Close by the per-cell flush rule. -/
def closeTail (w : Win C (Mem C) F T R) : PState (Mem C) F T R :=
  ⟨appendCells w.mem w.buf, w.frames, w.tasks, w.regs⟩

/-- Close by the circuit's one-flush rule. -/
def closeFlush (w : Win C (Mem C) F T R) : PState (Mem C) F T R :=
  ⟨leafFlush w.mem w.buf, w.frames, w.tasks, w.regs⟩

/-- A leaf of `k` steps with witness trace from `s` and close rule `close`. -/
def leaf {M : Type} (close : Win C M F T R → PState M F T R)
    (eff : LState C F T R → Effect C F T R) (k : Nat) (s : LState C F T R) (P : PState M F T R) :
    PState M F T R :=
  close (runWin eff k s (openW P))

/-- A leaf maps the canonical physical state of `s` to that of `next^[k] s` (per-cell rule). -/
theorem leaf_tail_commutes (eff : LState C F T R → Effect C F T R) (k : Nat)
    (s : LState C F T R) : leaf closeTail eff k s (abs s) = abs (iter (LState.next eff) k s) := by
  rw [leaf, runWin_eq eff k s _ rfl rfl rfl]
  simp only [closeTail, abs, openW, List.nil_append, iter_mem, canon_append]

/-- The same for the circuit rule, when the leaf has at most 8 appends. -/
theorem leaf_flush_commutes (eff : LState C F T R → Effect C F T R) (k : Nat)
    (s : LState C F T R) (h : (appsOf eff k s).length ≤ 8) :
    leaf closeFlush eff k s (abs s) = abs (iter (LState.next eff) k s) := by
  rw [← leaf_tail_commutes, leaf, leaf, runWin_eq eff k s _ rfl rfl rfl]
  simp only [closeFlush, closeTail, abs, openW, List.nil_append]
  rw [leafFlush_eq_of_wf (canon_wf _) _ h]

/-! ## Partitions -/

/-- Run the leaves of the cut `ks` from logical witness `s` and physical state `P`. -/
def runPart {M : Type} (close : Win C M F T R → PState M F T R)
    (eff : LState C F T R → Effect C F T R) : List Nat → LState C F T R → PState M F T R →
    PState M F T R
  | [], _, P => P
  | k :: ks, s, P => runPart close eff ks (iter (LState.next eff) k s) (leaf close eff k s P)

/-- A cut into leaves of `1..S` steps. -/
def ValidCut (S : Nat) (ks : List Nat) : Prop := ∀ k ∈ ks, 1 ≤ k ∧ k ≤ S

/-- Every leaf of the cut holds at most 8 appends (side condition of the circuit rule). -/
def FitsLeaf (eff : LState C F T R → Effect C F T R) : List Nat → LState C F T R → Prop
  | [], _ => True
  | k :: ks, s => (appsOf eff k s).length ≤ 8 ∧ FitsLeaf eff ks (iter (LState.next eff) k s)

/-- **Partition independence (SRC-023), per-cell rule.** For every cut `ks` of the steps into
leaves, the physical state after the leaves is the canonical state after `ks.sum` steps. No side
condition: it holds for every cut, in particular every cut into leaves of `1..S` steps. -/
theorem partition_independent (eff : LState C F T R → Effect C F T R) : ∀ (ks : List Nat)
    (s : LState C F T R),
    runPart closeTail eff ks s (abs s) = abs (iter (LState.next eff) ks.sum s)
  | [], s => rfl
  | k :: ks, s => by
    rw [runPart, leaf_tail_commutes, partition_independent eff ks, List.sum_cons, iter_add]

/-- **Partition independence, circuit rule.** The same for every cut whose leaves hold at most
8 appends each. -/
theorem partition_independent_flush (eff : LState C F T R → Effect C F T R) : ∀ (ks : List Nat)
    (s : LState C F T R), FitsLeaf eff ks s →
    runPart closeFlush eff ks s (abs s) = abs (iter (LState.next eff) ks.sum s)
  | [], s, _ => rfl
  | k :: ks, s, ⟨hk, hks⟩ => by
    rw [runPart, leaf_flush_commutes eff k s hk, partition_independent_flush eff ks _ hks,
      List.sum_cons, iter_add]

/-- Two valid cuts of the same N steps give the same physical state, hence the same `post_d`
under any digest. -/
theorem cuts_agree {D : Type} (digest : PState (Mem C) F T R → D) (S : Nat)
    (eff : LState C F T R → Effect C F T R) (s : LState C F T R) (ks₁ ks₂ : List Nat)
    (_ : ValidCut S ks₁) (_ : ValidCut S ks₂) (hsum : ks₁.sum = ks₂.sum)
    (h₁ : FitsLeaf eff ks₁ s) (h₂ : FitsLeaf eff ks₂ s) :
    runPart closeFlush eff ks₁ s (abs s) = runPart closeFlush eff ks₂ s (abs s) ∧
    digest (runPart closeFlush eff ks₁ s (abs s)) = digest (runPart closeFlush eff ks₂ s (abs s)) := by
  rw [partition_independent_flush eff ks₁ s h₁, partition_independent_flush eff ks₂ s h₂, hsum]
  exact ⟨rfl, rfl⟩

theorem appsOf_one (eff : LState C F T R → Effect C F T R) (s : LState C F T R) :
    appsOf eff 1 s = (eff s).app := by simp [appsOf]

/-- The cut into single steps fits the circuit rule when every step appends at most 8 cells, so
a cut satisfying `FitsLeaf` exists for every N. -/
theorem ones_fit (eff : LState C F T R → Effect C F T R)
    (h : ∀ s, (eff s).app.length ≤ 8) : ∀ (N : Nat) (s : LState C F T R),
    FitsLeaf eff (List.replicate N 1) s
  | 0, _ => trivial
  | N + 1, s => ⟨by rw [appsOf_one]; exact h s, ones_fit eff h N _⟩

/-! ## Connection to `Premises` and `det` -/

/-- The physical step relation: one reference step between canonical physical states. -/
def PStep (eff : LState C F T R → Effect C F T R) (a b : PState (Mem C) F T R) : Prop :=
  ∃ s, a = abs s ∧ b = abs (s.next eff)

/-- `det` holds for the physical step: the logical step is a function and `abs` is injective. -/
theorem pstep_det (eff : LState C F T R → Effect C F T R) (a b c : PState (Mem C) F T R)
    (h₁ : PStep eff a b) (h₂ : PStep eff a c) : b = c := by
  obtain ⟨s, rfl, rfl⟩ := h₁
  obtain ⟨s', hs, rfl⟩ := h₂
  rw [abs_inj hs]

theorem run_iter (eff : LState C F T R → Effect C F T R) : ∀ (k : Nat) (s : LState C F T R),
    Run (PStep eff) k (abs s) (abs (iter (LState.next eff) k s))
  | 0, _ => .refl _
  | k + 1, s => .step ⟨s, rfl, rfl⟩ (run_iter eff k (s.next eff))

/-- A k-step leaf (circuit rule, at most 8 appends) is a `Run` of exactly k physical steps: the
shape of the `Run (Step p) f.steps` conclusion of `Premises.leaf_sound`. -/
theorem leaf_run (eff : LState C F T R → Effect C F T R) (k : Nat) (s : LState C F T R)
    (h : (appsOf eff k s).length ≤ 8) :
    Run (PStep eff) k (abs s) (leaf closeFlush eff k s (abs s)) := by
  rw [leaf_flush_commutes eff k s h]; exact run_iter eff k s

/-- The `det` premise of `result_unique` / `statement_independent_of_S`, discharged for any
`Premises` whose states represent the tail-buffered physical state injectively and whose step is
the physical step of the program's effect function. -/
theorem premises_det {D : Type} [DecidableEq D] (P : Premises D)
    (effOf : P.Prog → LState C F T R → Effect C F T R) (rep : P.State → PState (Mem C) F T R)
    (rep_inj : ∀ a b, rep a = rep b → a = b)
    (hstep : ∀ p a b, P.Step p a b ↔ PStep (effOf p) (rep a) (rep b)) :
    ∀ p s t u, P.Step p s t → P.Step p s u → t = u := fun p s t u h₁ h₂ ↦
  rep_inj t u (pstep_det (effOf p) (rep s) (rep t) (rep u) ((hstep p s t).1 h₁) ((hstep p s u).1 h₂))

/-- `result_unique` with `det` discharged by the tail-buffered memory model; `stuck` (halted
states are terminal) stays a hypothesis. -/
theorem result_unique_tail {D : Type} [DecidableEq D] (P : Premises D) (S₁ S₂ : Nat)
    (effOf : P.Prog → LState C F T R → Effect C F T R) (rep : P.State → PState (Mem C) F T R)
    (rep_inj : ∀ a b, rep a = rep b → a = b)
    (hstep : ∀ p a b, P.Step p a b ↔ PStep (effOf p) (rep a) (rep b))
    (stuck : ∀ p t u, P.halted t → ¬ P.Step p t u)
    {root₁ root₂ : Aggregation.Seg D} {r₁ r₂ : P.Result}
    (hd₁ : Decided P S₁ root₁ r₁) (hd₂ : Decided P S₂ root₂ r₂)
    {programD initD : D} {n₁ n₂ : Nat} {fin₁ fin₂ : D}
    (ha₁ : AcceptRun programD initD root₁ n₁ fin₁) (ha₂ : AcceptRun programD initD root₂ n₂ fin₂)
    {p : P.Prog} (hp : P.Committed p) (hpd : P.progDigest p = programD)
    {s₀ : P.State} (hs₀ : P.Carrier s₀) (hd₀ : P.digest s₀ = initD) :
    n₁ = n₂ ∧ fin₁ = fin₂ ∧ r₁ = r₂ :=
  result_unique P S₁ S₂ (premises_det P effOf rep rep_inj hstep) stuck hd₁ hd₂ ha₁ ha₂ hp hpd
    hs₀ hd₀

/-! ## The old D3 padded layout is partition-dependent -/

/-- D3 close (`m3.rs`, `m3 walk first`: `len_post = base + 8`): the leaf writes one fresh aligned
segment, its appends padded with empty records (`none`, `act = 0`) to 8 cells. -/
def closePad (w : Win C (List (Option C)) F T R) : PState (List (Option C)) F T R :=
  ⟨w.mem ++ (w.buf.map some ++ List.replicate (8 - w.buf.length) none), w.frames, w.tasks, w.regs⟩

/-- A trace whose every step appends one cell. -/
def effOne : LState Unit Unit Unit Unit → Effect Unit Unit Unit Unit :=
  fun _ ↦ ⟨[()], .keep, .keep, ()⟩

def s₀ : LState Unit Unit Unit Unit := ⟨[], [], [], ()⟩
def P₀ : PState (List (Option Unit)) Unit Unit Unit := ⟨[], [], [], ()⟩

/-- Under D3 the cuts `[2]` and `[1, 1]` of the same two-append trace give different physical
memories (`len` 8 against 16). -/
theorem d3_partition_dependent :
    runPart closePad effOne [2] s₀ P₀ ≠ runPart closePad effOne [1, 1] s₀ P₀ ∧
    (runPart closePad effOne [2] s₀ P₀).mem.length = 8 ∧
    (runPart closePad effOne [1, 1] s₀ P₀).mem.length = 16 := by
  decide

/-- The tail-buffered layout gives one state for both cuts (contrast). -/
theorem tail_same_example :
    runPart closeFlush effOne [2] s₀ (abs s₀) = runPart closeFlush effOne [1, 1] s₀ (abs s₀) := by
  decide

theorem runPart_append {M : Type} (close : Win C M F T R → PState M F T R)
    (eff : LState C F T R → Effect C F T R) : ∀ (ks ks' : List Nat) (s : LState C F T R)
    (P : PState M F T R), runPart close eff (ks ++ ks') s P =
      runPart close eff ks' (iter (LState.next eff) ks.sum s) (runPart close eff ks s P)
  | [], ks', s, P => rfl
  | k :: ks, ks', s, P => by
    rw [List.cons_append, runPart, runPart, runPart_append close eff ks ks', List.sum_cons,
      iter_add]

/-- **D3 has no deterministic physical step on reachable states.** No deterministic step relation
on the padded physical state makes every padded leaf of 1 or 2 steps, taken from a state reached
by honest padded leaves from genesis with its consistent witness, a `Run` of exactly its number
of steps. So `det` together with the `Run` form of `leaf_sound` cannot be discharged at the
physical level under D3 (audit finding 10). `tail_det_run` is the positive twin for the
tail-buffered layout. -/
theorem d3_no_det_run : ¬ ∃ step : PState (List (Option Unit)) Unit Unit Unit →
    PState (List (Option Unit)) Unit Unit Unit → Prop,
    (∀ a b c, step a b → step a c → b = c) ∧
    ∀ (ks : List Nat) (k : Nat), 1 ≤ k → k ≤ 2 →
      Run step k (runPart closePad effOne ks s₀ P₀) (runPart closePad effOne (ks ++ [k]) s₀ P₀) := by
  rintro ⟨step, hdet, hrun⟩
  have h2 := hrun [] 2 (by decide) (by decide)
  have h1 := hrun [] 1 (by decide) (by decide)
  have h1' := hrun [1] 1 (by decide) (by decide)
  exact (d3_partition_dependent).1 (Run.unique hdet h2 (Run.trans h1 h1'))

/-- **The tail-buffered layout has one.** For every effect function, `PStep` is deterministic and
every leaf of the circuit rule from a reachable state (any cut `ks` from `abs s`, each leaf with at
most 8 appends) is a `Run` of exactly its number of steps. -/
theorem tail_det_run (eff : LState C F T R → Effect C F T R) :
    ∃ step : PState (Mem C) F T R → PState (Mem C) F T R → Prop,
    (∀ a b c, step a b → step a c → b = c) ∧
    ∀ (s : LState C F T R) (ks : List Nat) (k : Nat), FitsLeaf eff ks s →
      (appsOf eff k (iter (LState.next eff) ks.sum s)).length ≤ 8 →
      Run step k (runPart closeFlush eff ks s (abs s)) (runPart closeFlush eff (ks ++ [k]) s (abs s)) := by
  refine ⟨PStep eff, pstep_det eff, fun s ks k hks hk ↦ ?_⟩
  rw [runPart_append, partition_independent_flush eff ks s hks, runPart, runPart]
  exact leaf_run eff k _ hk

end Clean.Sherlock.Partition
