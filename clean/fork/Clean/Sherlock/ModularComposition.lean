import Clean.Sherlock.Aggregation

/-!
Modular composition of the leaf (experiment C3; `research/sherlock-final-design-20261111/`
`MODULARITY.md` and `ARCH-BRIEF.md` addendum, Lean obligation 3).

The leaf is a list of modules (CORE, MEM, ALU, RANGE, HASH, ...) over one leaf witness `W`. Each
module has a local relation `rel` (its gates over the witness), the bus tuples it requests and the
bus tuples it provides. The leaf relation `LeafRel` is every module's relation plus lookup
soundness of the bus: every requested tuple is among the provided ones. `BusBalanced` (the
multiset balance that a LogUp argument establishes, up to its Schwartz-Zippel error, which is a
proof-system premise and is not modelled) implies it (`balanced_lookup`).

A module's bus contract (`Contract`) has two parts:
* `guarantee`: if its relation holds and every tuple it requests is valid, every tuple it provides
  is valid (`Valid` is the meaning of a tuple, for example `c = a + b` on the ALU add chip);
* `ordered`: every tuple it requests has a strictly lower level than every tuple it provides
  (a module may not justify a tuple by itself; for example RANGE below ALU below CORE).

`provided_valid`: if every module meets its contract, every provided tuple and every requested
tuple of an accepted leaf witness is valid. `leaf_refines`: if in addition the spec module (CORE)
guarantees its goal from valid requests, the leaf relation implies the goal. `leaf_refines_step`
is the goal "the decoded rows follow the reference step function", concluded as an
`Aggregation.Run` of exactly the leaf's step count, the form `Premises.leaf_sound` uses.

Each hypothesis is a per-module fact, so each module kit discharges its own contract without
reading the other modules' relations. The theorems take module lists and are not tied to the
Sherlock module set; `toy_refines` instantiates them with CORE plus an ALU add table, and
`toy_needs_bus` shows the bus hypothesis is needed.
-/

namespace Clean.Sherlock.Modular

open Clean.Sherlock.Aggregation (Run)

/-- A module over the leaf witness `W` and bus tuples `Tup`. -/
structure Module (W Tup : Type) where
  rel : W → Prop
  requests : W → List Tup
  provides : W → List Tup

variable {W Tup : Type}

def allReqs (mods : List (Module W Tup)) (w : W) : List Tup := mods.flatMap (·.requests w)

def allProvs (mods : List (Module W Tup)) (w : W) : List Tup := mods.flatMap (·.provides w)

/-- Multiset balance of the bus (requests against provided tuples with their multiplicities). -/
def BusBalanced (mods : List (Module W Tup)) (w : W) : Prop := (allReqs mods w).Perm (allProvs mods w)

/-- Lookup soundness of the bus: every requested tuple is provided. -/
def LookupSound (mods : List (Module W Tup)) (w : W) : Prop :=
  ∀ t ∈ allReqs mods w, t ∈ allProvs mods w

theorem balanced_lookup {mods : List (Module W Tup)} {w : W} (h : BusBalanced mods w) :
    LookupSound mods w := fun _ ht ↦ (List.Perm.mem_iff h).1 ht

/-- The leaf relation: every module's local relation and a sound bus. -/
def LeafRel (mods : List (Module W Tup)) (w : W) : Prop :=
  (∀ m ∈ mods, m.rel w) ∧ LookupSound mods w

/-- A module's bus contract for tuple meaning `Valid` and tuple level `level`. -/
structure Contract (Valid : Tup → Prop) (level : Tup → Nat) (m : Module W Tup) : Prop where
  ordered : ∀ w, m.rel w → ∀ r ∈ m.requests w, ∀ t ∈ m.provides w, level r < level t
  guarantee : ∀ w, m.rel w → (∀ r ∈ m.requests w, Valid r) → ∀ t ∈ m.provides w, Valid t

/-- Every provided tuple of an accepted leaf witness is valid. -/
theorem provided_valid {Valid : Tup → Prop} {level : Tup → Nat} {mods : List (Module W Tup)}
    (hc : ∀ m ∈ mods, Contract Valid level m) {w : W} (hl : LeafRel mods w) :
    ∀ t ∈ allProvs mods w, Valid t := by
  have key : ∀ n t, level t < n → t ∈ allProvs mods w → Valid t := by
    intro n
    induction n with
    | zero => intro t h; omega
    | succ n ih =>
      intro t hlt ht
      obtain ⟨m, hm, htm⟩ := List.mem_flatMap.1 ht
      refine (hc m hm).guarantee w (hl.1 m hm) (fun r hr ↦ ?_) t htm
      have hord := (hc m hm).ordered w (hl.1 m hm) r hr t htm
      exact ih r (by omega) (hl.2 r (List.mem_flatMap.2 ⟨m, hm, hr⟩))
  exact fun t ht ↦ key _ t (Nat.lt_succ_self _) ht

/-- Every requested tuple of an accepted leaf witness is valid. -/
theorem requested_valid {Valid : Tup → Prop} {level : Tup → Nat} {mods : List (Module W Tup)}
    (hc : ∀ m ∈ mods, Contract Valid level m) {w : W} (hl : LeafRel mods w) :
    ∀ t ∈ allReqs mods w, Valid t :=
  fun t ht ↦ provided_valid hc hl t (hl.2 t ht)

/-- **Modular composition.** If every module meets its bus contract and the spec module `core`
guarantees `Goal` from its relation and valid requests, the leaf relation implies `Goal`. -/
theorem leaf_refines {Valid : Tup → Prop} {level : Tup → Nat} {mods : List (Module W Tup)}
    (hc : ∀ m ∈ mods, Contract Valid level m) {core : Module W Tup} (hcore : core ∈ mods)
    {Goal : W → Prop} (hgoal : ∀ w, core.rel w → (∀ r ∈ core.requests w, Valid r) → Goal w)
    {w : W} (hl : LeafRel mods w) : Goal w :=
  hgoal w (hl.1 core hcore) fun r hr ↦ requested_valid hc hl r (List.mem_flatMap.2 ⟨core, hcore, hr⟩)

/-- The leaf with a balanced bus (the LogUp form) refines the goal. -/
theorem leaf_refines_balanced {Valid : Tup → Prop} {level : Tup → Nat}
    {mods : List (Module W Tup)} (hc : ∀ m ∈ mods, Contract Valid level m) {core : Module W Tup}
    (hcore : core ∈ mods) {Goal : W → Prop}
    (hgoal : ∀ w, core.rel w → (∀ r ∈ core.requests w, Valid r) → Goal w) {w : W}
    (hrel : ∀ m ∈ mods, m.rel w) (hbal : BusBalanced mods w) : Goal w :=
  leaf_refines hc hcore hgoal ⟨hrel, balanced_lookup hbal⟩

/-! ## The step-function goal -/

/-- Consecutive rows follow the step function. -/
def Follows {St : Type} (step : St → St) : St → List St → Prop
  | _, [] => True
  | a, b :: l => b = step a ∧ Follows step b l

/-- Rows that follow `step` are a `Run` of exactly their number of steps of the functional step
relation, ending at the last row. -/
theorem follows_run {St : Type} (step : St → St) : ∀ (a : St) (l : List St),
    Follows step a l → Run (fun x y ↦ y = step x) l.length a (l.getLastD a)
  | a, [], _ => .refl a
  | a, b :: l, ⟨hb, hl⟩ => by
    have := follows_run step b l hl
    rw [List.getLastD_cons]
    exact .step hb this

/-- The functional step relation is deterministic (the `det` premise of `Aggregation.lean`). -/
theorem fun_det {St : Type} (step : St → St) :
    ∀ s t u, (fun x y ↦ y = step x) s t → (fun x y ↦ y = step x) s u → t = u :=
  fun _ _ _ h₁ h₂ ↦ h₁.trans h₂.symm

/-- **Modular composition, step form.** If every module meets its contract and CORE's contract
says its decoded rows `pre :: rows` follow the reference step function whenever its requests are
valid, an accepted leaf is a `Run` of exactly `rows.length` reference steps from the decoded
pre-state to the decoded post-state. -/
theorem leaf_refines_step {Valid : Tup → Prop} {level : Tup → Nat} {mods : List (Module W Tup)}
    (hc : ∀ m ∈ mods, Contract Valid level m) {core : Module W Tup} (hcore : core ∈ mods)
    {St : Type} (step : St → St) (pre : W → St) (rows : W → List St)
    (hcoreSpec : ∀ w, core.rel w → (∀ r ∈ core.requests w, Valid r) → Follows step (pre w) (rows w))
    {w : W} (hl : LeafRel mods w) :
    Run (fun x y ↦ y = step x) (rows w).length (pre w) ((rows w).getLastD (pre w)) :=
  follows_run step _ _ (leaf_refines hc hcore hcoreSpec hl)

/-! ## Toy instance: CORE requests an ALU add tuple -/

/-- Leaf witness: CORE's pre and post cells and the ALU add table. -/
structure ToyW where
  pre : Nat
  post : Nat
  table : List (Nat × Nat × Nat)

/-- An add tuple `(a, b, c)` means `c = a + b`. -/
def toyValid (t : Nat × Nat × Nat) : Prop := t.2.2 = t.1 + t.2.1

/-- CORE: one step `x ↦ x + 3`, computed by an ALU request `(pre, 3, post)`. -/
def toyCore : Module ToyW (Nat × Nat × Nat) := ⟨fun _ ↦ True, fun w ↦ [(w.pre, 3, w.post)], fun _ ↦ []⟩

/-- ALU: every table row satisfies the add gate; the table is provided on the bus. -/
def toyAlu : Module ToyW (Nat × Nat × Nat) :=
  ⟨fun w ↦ ∀ r ∈ w.table, toyValid r, fun _ ↦ [], fun w ↦ w.table⟩

theorem toy_contracts : ∀ m ∈ [toyCore, toyAlu], Contract toyValid (fun _ ↦ 0) m := by
  intro m hm
  simp only [List.mem_cons, List.not_mem_nil, or_false] at hm
  rcases hm with rfl | rfl
  · exact ⟨fun _ _ _ _ t ht ↦ by simp [toyCore] at ht, fun _ _ _ t ht ↦ by simp [toyCore] at ht⟩
  · exact ⟨fun _ _ r hr ↦ by simp [toyAlu] at hr, fun _ hrel _ t ht ↦ hrel t ht⟩

/-- The toy leaf refines the step `x ↦ x + 3`. -/
theorem toy_refines (w : ToyW) (hl : LeafRel [toyCore, toyAlu] w) :
    Run (fun x y ↦ y = x + 3) 1 w.pre w.post := by
  have := leaf_refines_step toy_contracts (core := toyCore) (by simp) (fun x ↦ x + 3)
    (fun w ↦ w.pre) (fun w ↦ [w.post])
    (fun w _ hv ↦ ⟨by
      have := hv (w.pre, 3, w.post) (by simp [toyCore])
      simp only [toyValid] at this; simp only [this], trivial⟩) hl
  simpa using this

/-- Without the bus, the module relations alone admit a wrong step (post = pre + 4). -/
theorem toy_needs_bus : ∃ w : ToyW, (∀ m ∈ [toyCore, toyAlu], m.rel w) ∧ w.post ≠ w.pre + 3 :=
  ⟨⟨0, 4, []⟩, by
    refine ⟨fun m hm ↦ ?_, by decide⟩
    simp only [List.mem_cons, List.not_mem_nil, or_false] at hm
    rcases hm with rfl | rfl
    · trivial
    · intro r hr; simp at hr⟩

end Clean.Sherlock.Modular
