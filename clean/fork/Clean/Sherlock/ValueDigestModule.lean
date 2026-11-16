import Clean.Sherlock.ValueDigest
import Clean.Sherlock.ModularComposition

/-!
O27: VALUE DIGEST as a `Module` of `ModularComposition.lean` (task 2.5,
`openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-value-digest`, VDG-004 and VDG-006).

The bus tuples are the three VALUE DIGEST uses: a RANGE tuple `(x, b)` means `x < 2^b`, a HASH
sponge tuple `(ws, out)` means `out = H ws`, and a `vd_req` tuple (port 6) `(n, out)` means that
the node `n` is ranged and `out = H (pre n)`. RANGE and HASH tuples are on level 0, `vd_req`
tuples on level 1.

A VALUE DIGEST row holds the node it hashes (the op, the scalar columns `S` and the child digest
columns), the eight 16-bit limbs of a `UInt`, the sponge output `VDH` and the digest `VD`. Its
relation is `VD = VDH` and the shape of the scalar columns (`UInt` is its limbs, `Fr` has nine
limbs). It requests one sponge over `pre n` and one RANGE tuple per scalar limb, and it provides
`(n, VD)` on `vd_req`.

* `vd_contract`: the VALUE DIGEST module meets its bus contract.
* `vd_leaf_sound`: in a leaf where every module meets its contract, if CORE requests the `vd_req`
  tuple of every record (and the fold steps and the completion of every constructor), every `vd`
  lane is `vd` of the value its record represents. So O25 (`vd_heap_eq`) holds of an accepted
  leaf witness, and with O24 equal lanes mean equal values.
* `vd_needs_sponge`: without the sponge bus, the row relation admits a forged digest
  (mutant `host_supplied_vd`).
No `sorry`, no `axiom`.
-/

namespace Clean.Sherlock.ValueDigestModule

open Clean.Sherlock.ValueDigest
open Clean.Sherlock.Modular

section
variable {D : Type}

/-- The bus tuples of VALUE DIGEST. -/
inductive Tup (D : Type) where
  | range (x b : Nat)
  | sponge (ws : List (W D)) (out : D)
  | vd (n : Node D) (out : D)

variable (H : List (W D) → D)

/-- The meaning of a tuple. -/
def Valid : Tup D → Prop
  | .range x b => x < 2 ^ b
  | .sponge ws out => out = H ws
  | .vd n out => Ranged n ∧ out = H (pre n)

/-- RANGE and HASH below VALUE DIGEST below CORE. -/
def level : Tup D → Nat
  | .range _ _ => 0
  | .sponge _ _ => 0
  | .vd _ _ => 1

/-- A VALUE DIGEST row. -/
structure Row (D : Type) where
  node : Node D
  limbs : List Nat
  hout : D
  out : D

/-- The `u128` that eight little-endian 16-bit limbs compose. -/
def u128 : List Nat → Nat
  | [] => 0
  | l :: ls => l + 65536 * u128 ls

/-- The shape of the scalar columns. -/
def Shape : Node D → List Nat → Prop
  | .uint n, ls => ls.length = 8 ∧ n = u128 ls
  | .fr x, _ => x.length = 9
  | _, _ => True

def RowRel (r : Row D) : Prop := r.out = r.hout ∧ Shape r.node r.limbs

def rangeReqs : Node D → List Nat → List (Tup D)
  | .fr x, _ => x.map (fun k ↦ .range k 30)
  | .uint _, ls => ls.map (fun k ↦ .range k 16)
  | _, _ => []

def rowReqs (r : Row D) : List (Tup D) := .sponge (pre r.node) r.hout :: rangeReqs r.node r.limbs

/-- VALUE DIGEST over a leaf witness `Wt`, whose rows are `rows w`. -/
def vdModule {Wt : Type} (rows : Wt → List (Row D)) : Module Wt (Tup D) where
  rel w := ∀ r ∈ rows w, RowRel r
  requests w := (rows w).flatMap rowReqs
  provides w := (rows w).map (fun r ↦ .vd r.node r.out)

theorem u128_lt (ls : List Nat) (hl : ls.length = 8) (hr : ∀ l ∈ ls, l < 2 ^ 16) :
    u128 ls < 2 ^ 128 := by
  match ls, hl with
  | [a, b, c, d, e, f, g, h], _ =>
    simp only [List.mem_cons, List.not_mem_nil, or_false, forall_eq_or_imp, forall_eq] at hr
    simp only [u128, Nat.reducePow] at hr ⊢
    omega

theorem row_ranged (r : Row D) (hs : Shape r.node r.limbs)
    (hv : ∀ t ∈ rangeReqs r.node r.limbs, Valid H t) : Ranged r.node := by
  rcases r with ⟨n, ls, ho, o⟩
  cases n with
  | fr x =>
    refine ⟨hs, fun k hk ↦ ?_⟩
    exact hv (.range k 30) (List.mem_map.2 ⟨k, hk, rfl⟩)
  | uint n =>
    dsimp only at hs hv ⊢
    obtain ⟨hl, he⟩ := hs
    show n < 2 ^ 128
    rw [he]
    exact u128_lt ls hl fun l hl' ↦ hv (.range l 16) (List.mem_map.2 ⟨l, hl', rfl⟩)
  | _ => trivial

/-- O27: VALUE DIGEST meets its bus contract. -/
theorem vd_contract {Wt : Type} (rows : Wt → List (Row D)) :
    Contract (Valid H) level (vdModule rows) := by
  refine ⟨fun w _ q hq t ht ↦ ?_, fun w hrel hreq t ht ↦ ?_⟩
  · obtain ⟨r, -, rfl⟩ := List.mem_map.1 ht
    obtain ⟨r', -, hq⟩ := List.mem_flatMap.1 hq
    simp only [rowReqs, List.mem_cons] at hq
    rcases hq with rfl | hq
    · simp [level]
    · rcases r' with ⟨n, ls, ho, o⟩
      cases n <;> simp only [rangeReqs, List.mem_map, List.not_mem_nil] at hq <;>
        first | (obtain ⟨_, -, rfl⟩ := hq; simp [level]) | exact hq.elim
  · obtain ⟨r, hr, rfl⟩ := List.mem_map.1 ht
    have req : ∀ q ∈ rowReqs r, Valid H q :=
      fun q hq ↦ hreq q (List.mem_flatMap.2 ⟨r, hr, hq⟩)
    obtain ⟨ho, hs⟩ := hrel r hr
    have hsp : r.hout = H (pre r.node) := req _ (List.mem_cons_self ..)
    exact ⟨row_ranged H r hs fun q hq ↦ req q (List.mem_cons_of_mem _ hq), ho.trans hsp⟩

/-! ## The leaf: CORE's records get their lanes from `vd_req` -/

/-- The fold: valid step tuples chained from `a` end at `accD a fs`. -/
theorem fold_chain (a : D) (fs : List D) (accs : Nat → D) (h0 : accs 0 = a)
    (hs : ∀ k (hk : k < fs.length), Valid H (.vd (.step (accs k) fs[k]) (accs (k + 1)))) :
    accs fs.length = accD H a fs := by
  induction fs generalizing a accs with
  | nil => simp [accD, h0]
  | cons f fs ih =>
    have h1 : accs 1 = H (pre (.step a f)) := by
      have := (hs 0 (by simp)).2; simpa [h0] using this
    have := ih (H (pre (.step a f))) (fun k ↦ accs (k + 1)) h1 fun k hk ↦ by
      have h := hs (k + 1) (by simp; omega)
      simp only [List.getElem_cons_succ] at h
      exact h
    simpa [accD] using this

/-- CORE requests the `vd_req` tuple of every record: the node of a non-constructor record, and
the fold steps from the zero digest `z` plus the completion of a constructor. -/
def Covers {Wt : Type} (core : Module Wt (Tup D)) (w : Wt) (z : D) (R : Nat → Rec D)
    (lane : Nat → D) : Prop :=
  ∀ i, match R i with
    | .ctor ty c js => ∃ accs : Nat → D, accs 0 = z ∧
        (∀ k (hk : k < js.length),
          Tup.vd (.step (accs k) (lane js[k])) (accs (k + 1)) ∈ core.requests w) ∧
        Tup.vd (.done js.length ty c (accs js.length)) (lane i) ∈ core.requests w
    | r => Tup.vd (nodeOf H z lane r) (lane i) ∈ core.requests w

/-- In an accepted leaf whose modules meet their contracts, the `vd` lanes CORE covers satisfy
`Kit`: each lane is VALUE DIGEST's hash of its record's form and its children's lanes. -/
theorem kit_of_leaf {Wt : Type} {mods : List (Module Wt (Tup D))}
    (hc : ∀ m ∈ mods, Contract (Valid H) level m) {w : Wt} (hl : LeafRel mods w)
    {core : Module Wt (Tup D)} (hcore : core ∈ mods) {z : D} {R : Nat → Rec D}
    {lane : Nat → D} (hcov : Covers H core w z R lane) : Kit H z R lane := by
  have hv : ∀ t ∈ core.requests w, Valid H t :=
    fun t ht ↦ requested_valid hc hl t (List.mem_flatMap.2 ⟨core, hcore, ht⟩)
  intro i
  have hi := hcov i
  generalize hR : R i = rec at hi
  cases rec with
  | ctor ty c js =>
    obtain ⟨accs, h0, hs, hd⟩ := hi
    have hf := fold_chain H z (js.map lane) accs h0 fun k hk ↦ by
      simpa using hv _ (hs k (by simpa using hk))
    simp only [List.length_map] at hf
    simpa [nodeOf, hf] using (hv _ hd).2
  | _ => exact (hv _ hi).2

/-- O27 with O25: in an accepted leaf, every covered `vd` lane is the digest of the value its
record represents, for any address layout. -/
theorem vd_leaf_sound {Wt : Type} {mods : List (Module Wt (Tup D))}
    (hc : ∀ m ∈ mods, Contract (Valid H) level m) {w : Wt} (hl : LeafRel mods w)
    {core : Module Wt (Tup D)} (hcore : core ∈ mods) {z : D} {R : Nat → Rec D}
    {lane : Nat → D} (hcov : Covers H core w z R lane) {i : Nat} {v : Value D}
    (hr : Repr R i v) : lane i = vd H z v :=
  vd_heap_eq H z (kit_of_leaf H hc hl hcore hcov) hr

/-- O27 with O24: two covered lanes of an accepted leaf that represent ranged values are equal
only if the values are equal (under `Inj H`). -/
theorem vd_leaf_injective {Wt : Type} {mods : List (Module Wt (Tup D))}
    (hc : ∀ m ∈ mods, Contract (Valid H) level m) {w : Wt} (hl : LeafRel mods w)
    {core : Module Wt (Tup D)} (hcore : core ∈ mods) {z : D} {R : Nat → Rec D}
    {lane : Nat → D} (hcov : Covers H core w z R lane) (hH : Inj H) {i j : Nat} {u v : Value D}
    (hu : Repr R i u) (hv : Repr R j v) (ru : RV u) (rv : RV v) (e : lane i = lane j) : u = v :=
  vd_injective H z hH u v ru rv (by
    rw [← vd_leaf_sound H hc hl hcore hcov hu, ← vd_leaf_sound H hc hl hcore hcov hv]; exact e)

end

/-- Without the sponge bus, the VALUE DIGEST row relation admits a digest that is not the hash of
the node (mutant `host_supplied_vd`). -/
theorem vd_needs_sponge : ∃ (H : List (W Bool) → Bool) (r : Row Bool),
    (vdModule (fun _ : Unit ↦ [r])).rel () ∧ ¬ Valid H (.vd r.node r.out) :=
  ⟨fun _ ↦ true, ⟨.nil, [], false, false⟩, by
    refine ⟨fun r hr ↦ ?_, ?_⟩
    · simp only [List.mem_singleton] at hr; subst hr; exact ⟨rfl, trivial⟩
    · simp [Valid]⟩

end Clean.Sherlock.ValueDigestModule
