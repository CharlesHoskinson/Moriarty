import Governance.Spec

/-!
# Invariants of the Governance oracle

`Inv` holds in `init` and after every operation (`inv_run`). It is the status layer:
slot ids in order and never reused, one pending and one current era, the readback gates
before use, the rekey gate. The order layer (start(i) < start(j) and end(i) <= start(j)
for i < j, which gives ERT-005 at every block time) is not proved here; ORACLE.md lists it
as open obligation OB-1, checked by the pins and the Quint replay.

`InvC` reads only the `Core` projection of the state, so an operation that keeps the
core keeps it (`invA_of_core`).
-/

set_option linter.unusedSimpArgs false

namespace Governance

/-- The cells the arithmetic invariant reads. -/
structure Core where
  tl : Int
  grid : Int
  q : Int
  rtl : Int
  now : Int
  nextId : Int
  pending : Int
  current : Int
  e1 : Era
  e2 : Era
  e3 : Era
  l1 : Int
  l2 : Int
  l3 : Int
  p1 : Int
  p2 : Int
  p3 : Int
  deriving DecidableEq

def core (s : St) : Core :=
  { tl := s.tl, grid := s.grid, q := s.q, rtl := s.rtl, now := s.now, nextId := s.nextId,
    pending := s.pending, current := s.current, e1 := s.e1, e2 := s.e2, e3 := s.e3,
    l1 := s.l1, l2 := s.l2, l3 := s.l3, p1 := s.prem1, p2 := s.prem2, p3 := s.prem3 }

/-- Status facts of one era slot `i`. -/
def SlotS (c : Core) (e : Era) (i : Int) : Prop :=
  (0 ≤ e.st ∧ e.st ≤ 5)
  ∧ (e.st ≠ 0 ↔ i < c.nextId)          -- GOV-007: ids in order, never reused
  ∧ (e.st = 1 ↔ c.pending = i)         -- GOV-002: at most one pending era
  ∧ (e.st = 2 ↔ c.current = i)         -- GOV-003: one current era
  ∧ ((e.st = 1 ∨ e.st = 2) → e.fin = 0) -- ERT-007: end is written once, at supersession
  ∧ (e.st ≠ 0 → 1 ≤ e.keyset)

/-- The scalar facts. -/
def Glob (c : Core) : Prop :=
  1 ≤ c.tl ∧ 1 ≤ c.grid ∧ 1 ≤ c.q ∧ 0 ≤ c.rtl ∧ 0 ≤ c.now
  ∧ 1 ≤ c.nextId ∧ c.nextId ≤ 4 ∧ 0 ≤ c.pending ∧ c.pending ≤ 3 ∧ 0 ≤ c.current ∧ c.current ≤ 3
  ∧ (c.pending ≠ 0 → c.nextId = c.pending + 1)
  ∧ (0 ≤ c.l1 ∧ c.l1 ≤ 2 ∧ 0 ≤ c.l2 ∧ c.l2 ≤ 2 ∧ 0 ≤ c.l3 ∧ c.l3 ≤ 2)
  ∧ (0 ≤ c.p1 ∧ c.p1 ≤ 2 ∧ 0 ≤ c.p2 ∧ c.p2 ≤ 2 ∧ 0 ≤ c.p3 ∧ c.p3 ≤ 2)

def InvC (c : Core) : Prop := Glob c ∧ SlotS c c.e1 1 ∧ SlotS c c.e2 2 ∧ SlotS c c.e3 3

/-- The arithmetic invariant. -/
def InvA (s : St) : Prop := InvC (core s)

/-- The boolean invariant. -/
def InvB (s : St) : Prop :=
  (1 < s.nextId → s.tableSealed = true)                                        -- ERT-002
  ∧ (s.tableSealed = true → s.tableDeployed = true)                            -- ERT-001
  ∧ (s.rekeyStarted = true → s.eraRow = true ∨ s.prem2 = 2 ∨ s.prem3 = 2)      -- EGV-005

def Inv (s : St) : Prop := InvA s ∧ InvB s

theorem inv_init (tl rtl q n grid : Int) (h : paramsOk tl rtl q n grid = true) :
    Inv (init tl rtl q n grid) := by
  simp [paramsOk] at h
  simp [Inv, InvA, InvB, InvC, Glob, core, SlotS, init, Era.empty]
  omega

/-- Rewrite an `if` with a known condition, whatever its `Decidable` instance. -/
theorem ite_t {α : Type} {c : Prop} {_ : Decidable c} {a b : α} (h : c) : (if c then a else b) = a :=
  ite_eq_left_of_eq_true a b (eq_true h)
theorem ite_f {α : Type} {c : Prop} {_ : Decidable c} {a b : α} (h : ¬c) : (if c then a else b) = b :=
  ite_eq_right_of_eq_false a b (eq_false h)

theorem invA_of_core {s s' : St} (hc : core s' = core s) (h : InvA s) : InvA s' := by
  unfold InvA; rw [hc]; exact h

/-- An operation that keeps the core, branch by branch. -/
macro "frame" : tactic => `(tactic| (
  apply invA_of_core _ ‹InvA _›
  cases hen : enabled _ _
  · rw [apply_dis hen]
  · rw [apply_en hen]; simp only [applyE, refuse, setClaims]
    (repeat' split) <;> simp [core]))

section
variable (s : St) (h : InvA s)
include h

theorem invA_refuse (c : Int) : InvA (refuse s c) :=
  invA_of_core (by simp [refuse, core]) h

theorem invA_rotate (g r : Int) : InvA (apply s (.rotateRoster g r)) := by frame
theorem invA_cancelRotation (g : Int) : InvA (apply s (.cancelRotation g)) := by frame
theorem invA_attest (g r : Int) : InvA (apply s (.attest g r)) := by frame
theorem invA_vkChange : InvA (apply s .vkChange) := by frame
theorem invA_report (b : Bool) : InvA (apply s (.reportLedger b)) := by frame
theorem invA_addEraRow : InvA (apply s .addEraRow) := by frame
theorem invA_rekeyStart : InvA (apply s .rekeyStart) := by frame
theorem invA_useAttested : InvA (apply s .useAttested) := by frame
theorem invA_useClaim (e : Int) : InvA (apply s (.useClaim e)) := by frame
theorem invA_acceptCall (e : Int) : InvA (apply s (.acceptCall e)) := by frame

end


/-! ## Operations that change the core

Each proof splits the slot index into its three values, computes the new state, and
closes the four parts of `InvC` one at a time with only the facts of that part, so each
`omega` call stays small. -/

/-- One part: unfold the slot and scalar facts and close with `omega`. -/
macro "stac" : tactic => `(tactic| (
  simp only [SlotS, Glob, core, Int.reduceEq, Int.reduceAdd, ↓reduceIte, ne_eq, implies_true,
    and_true, true_and, not_false_eq_true, not_true_eq_false, Bool.false_eq_true, false_implies,
    true_implies, iff_false, iff_true, false_iff, true_iff, false_and, and_false, or_false,
    false_or, true_or, or_true, Classical.not_not] at *
  omega))

set_option hygiene false in
/-- Split `InvA s` into its parts `g s1 s2 s3`. -/
macro "opens" : tactic => `(tactic| (
  obtain ⟨g, s1, s2, s3⟩ := ‹InvA _›))

set_option hygiene false in
/-- Close the four parts of the new `InvA` from `g s1 s2 s3`. -/
macro "fin4" : tactic => `(tactic| (
  refine ⟨?_, ?_, ?_, ?_⟩
  focus (clear s1 s2 s3; stac)
  focus (clear s2 s3; stac)
  focus (clear s1 s3; stac)
  focus (clear s1 s2; stac)))

theorem setEra_eq (s : St) (m : Int) (E : Era) (h1 : 1 ≤ m) (h3 : m ≤ 3) :
    setEra s m E = { s with
      e1 := (if m = 1 then E else s.e1)
      e2 := (if m = 2 then E else s.e2)
      e3 := (if m = 3 then E else s.e3) } := by
  unfold setEra
  by_cases a : m = 1
  · subst a; simp
  · by_cases b : m = 2
    · subst b; simp
    · have c : m = 3 := by omega
      subst c; simp

theorem era_eq (s : St) (i : Int) :
    era s i = if i = 1 then s.e1 else if i = 2 then s.e2 else s.e3 := rfl

section
variable (s : St) (h : InvA s)
include h

theorem invA_tick (dt : Int) : InvA (apply s (.tick dt)) := by
  cases hen : enabled s (.tick dt)
  · rw [apply_dis hen]; exact h
  · rw [apply_en hen]; simp only [enabled, decide_eq_true_eq] at hen
    opens; simp only [applyE]; split <;> fin4

theorem invA_deploy (t c th ctr : Int) (vk : Bool) (l : Int) :
    InvA (apply s (.deploy t c th ctr vk l)) := by
  cases hen : enabled s (.deploy t c th ctr vk l)
  · rw [apply_dis hen]; exact h
  · rw [apply_en hen]; opens; simp only [applyE, setLedger]
    (repeat' split) <;> fin4

theorem invA_premise (i : Int) (f : Bool) : InvA (apply s (.premise i f)) := by
  cases hen : enabled s (.premise i f)
  · rw [apply_dis hen]; exact h
  · rw [apply_en hen]; opens; simp only [applyE, setPrem]
    (repeat' split) <;> fin4

theorem invA_announce (sg k l t : Int) : InvA (apply s (.announce sg k l t)) := by
  cases hen : enabled s (.announce sg k l t)
  · rw [apply_dis hen]; exact h
  · rw [apply_en hen]
    simp only [enabled, isSlot, decide_eq_true_eq] at hen
    simp only [applyE]
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    have hn1 : 1 ≤ s.nextId := h.1.2.2.2.2.2.1
    rw [setEra_eq _ _ _ hn1 hen.1]
    opens
    have hm : s.nextId = 1 ∨ s.nextId = 2 ∨ s.nextId = 3 := by omega
    rcases hm with hm | hm | hm <;> simp only [hm, ↓reduceIte] <;> fin4

theorem invA_doActivate (m : Int) (hm : s.pending = m) (h1 : 1 ≤ m) (h3 : m ≤ 3) :
    InvA (doActivate s m) := by
  opens
  have hc : 0 ≤ s.current ∧ s.current ≤ 3 := by simp only [Glob, core] at g; omega
  unfold doActivate
  by_cases c0 : s.current = 0
  · simp only [c0, ne_eq, not_true_eq_false, ↓reduceIte]
    rw [setEra_eq _ _ _ h1 h3]
    have hm' : m = 1 ∨ m = 2 ∨ m = 3 := by omega
    rcases hm' with rfl | rfl | rfl <;> simp only [era_eq, ↓reduceIte] <;> fin4
  · simp only [c0, ne_eq, not_false_eq_true, ↓reduceIte]
    rw [setEra_eq _ _ _ (by omega) hc.2, setEra_eq _ _ _ h1 h3]
    have hn' : s.current = 1 ∨ s.current = 2 ∨ s.current = 3 := by omega
    have hm' : m = 1 ∨ m = 2 ∨ m = 3 := by omega
    rcases hn' with hn | hn | hn <;> rcases hm' with rfl | rfl | rfl <;>
      simp only [hn, era_eq, ↓reduceIte] <;> fin4

theorem invA_activate (m : Int) : InvA (apply s (.activate m)) := by
  cases hen : enabled s (.activate m)
  · rw [apply_dis hen]; exact h
  · rw [apply_en hen]
    simp only [enabled, isSlot, decide_eq_true_eq] at hen
    simp only [applyE]
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    exact invA_doActivate s h m (by simp_all) hen.1 hen.2

theorem invA_cancel (sg m : Int) : InvA (apply s (.cancel sg m)) := by
  cases hen : enabled s (.cancel sg m)
  · rw [apply_dis hen]; exact h
  · rw [apply_en hen]
    simp only [enabled, isSlot, decide_eq_true_eq] at hen
    simp only [applyE]
    split; · exact invA_refuse s h _
    split; · exact invA_refuse s h _
    rw [setEra_eq _ _ _ hen.1 hen.2]
    opens
    have hm' : m = 1 ∨ m = 2 ∨ m = 3 := by omega
    rcases hm' with rfl | rfl | rfl <;> simp only [era_eq, ↓reduceIte] at * <;> fin4

end

/-- GOV-006: the end part of a soundness end, from any state with a current era. -/
theorem invA_endPart (t1 : St) (h : InvA t1) (hc : t1.current ≠ 0) (x nc : Int) :
    InvA (let s2 := setEra t1 t1.current { era t1 t1.current with fin := x, st := 5 }
          let s3 := if s2.pending ≠ 0 then setEra s2 s2.pending { era s2 s2.pending with st := 4 } else s2
          { s3 with current := 0, pending := 0, nonce := nc, lastRefusal := 0 }) := by
  opens
  have hb : 1 ≤ t1.current ∧ t1.current ≤ 3 ∧ 0 ≤ t1.pending ∧ t1.pending ≤ 3 := by
    simp only [Glob, core] at g; omega
  have hn' : t1.current = 1 ∨ t1.current = 2 ∨ t1.current = 3 := by omega
  have hp' : t1.pending = 0 ∨ t1.pending = 1 ∨ t1.pending = 2 ∨ t1.pending = 3 := by omega
  rcases hn' with hn | hn | hn <;> rcases hp' with hp | hp | hp | hp <;>
    simp only [hn, hp, setEra, era_eq, ne_eq, ↓reduceIte, not_true_eq_false, not_false_eq_true,
      Int.reduceEq] <;> fin4

theorem invA_soundnessEnd (s : St) (h : InvA s) (sg : Int) : InvA (apply s (.soundnessEnd sg)) := by
  rw [apply_en (by rfl)]
  simp only [applyE]
  split; · exact invA_refuse s h _
  have h1 : InvA (if s.pending ≠ 0 ∧ (era s s.pending).start ≤ s.now then doActivate s s.pending else s) := by
    split
    · rename_i hd
      have hp : 0 ≤ s.pending ∧ s.pending ≤ 3 := by
        have g := h.1; simp only [Glob, core] at g; omega
      exact invA_doActivate s h _ rfl (by omega) hp.2
    · exact h
  generalize hs1 : (if s.pending ≠ 0 ∧ (era s s.pending).start ≤ s.now then doActivate s s.pending else s) = s1 at h1 ⊢
  split
  · exact invA_refuse s h _
  · rename_i hne
    exact invA_endPart s1 h1 (by simp_all) s.now _

/-! ## The boolean invariant -/

/-- The cells the boolean invariant reads. -/
structure BCore where
  nextId : Int
  tableSealed : Bool
  tableDeployed : Bool
  rekeyStarted : Bool
  eraRow : Bool
  p2 : Int
  p3 : Int

def bcore (s : St) : BCore :=
  { nextId := s.nextId, tableSealed := s.tableSealed, tableDeployed := s.tableDeployed,
    rekeyStarted := s.rekeyStarted, eraRow := s.eraRow, p2 := s.prem2, p3 := s.prem3 }

theorem invB_of_bcore {s s' : St} (hc : bcore s' = bcore s) (h : InvB s) : InvB s' := by
  simp only [bcore, BCore.mk.injEq] at hc
  obtain ⟨a, b, c, d, e, f, g⟩ := hc
  unfold InvB; rw [a, b, c, d, e, f, g]; exact h

/-- An operation that keeps the boolean core, branch by branch. -/
macro "frameB" : tactic => `(tactic| (
  apply invB_of_bcore _ ‹InvB _›
  cases hen : enabled _ _
  · rw [apply_dis hen]
  · rw [apply_en hen]; simp only [applyE, refuse, setClaims, setEra, doActivate, setLedger]
    (repeat' split) <;> simp [bcore]))

theorem bcore_setEra (s : St) (i : Int) (E : Era) : bcore (setEra s i E) = bcore s := by
  unfold setEra; split
  · rfl
  · split <;> rfl

theorem bcore_doActivate (s : St) (m : Int) : bcore (doActivate s m) = bcore s := by
  have h1 : bcore (if s.current ≠ 0 then
      setEra s s.current { era s s.current with fin := (era s m).start, st := 3 } else s) = bcore s := by
    split
    · exact bcore_setEra _ _ _
    · rfl
  have h2 := bcore_setEra (if s.current ≠ 0 then
      setEra s s.current { era s s.current with fin := (era s m).start, st := 3 } else s) m
    { era (if s.current ≠ 0 then
      setEra s s.current { era s s.current with fin := (era s m).start, st := 3 } else s) m with st := 2 }
  rw [h1] at h2
  simp only [bcore, BCore.mk.injEq] at h2 ⊢
  unfold doActivate
  exact h2

/-- GOV-006: a soundness end changes no cell of the boolean invariant. -/
theorem invB_soundnessEnd (s : St) (hb : InvB s) (sg : Int) : InvB (apply s (.soundnessEnd sg)) := by
  rw [apply_en (by rfl)]
  simp only [applyE]
  split
  · exact hb
  have hb1 : InvB (if s.pending ≠ 0 ∧ (era s s.pending).start ≤ s.now then doActivate s s.pending else s) := by
    split
    · exact invB_of_bcore (bcore_doActivate _ _) hb
    · exact hb
  generalize (if s.pending ≠ 0 ∧ (era s s.pending).start ≤ s.now then doActivate s s.pending else s) = s1 at hb1 ⊢
  split
  · exact hb
  · apply invB_of_bcore _ hb1
    have h2 := bcore_setEra s1 s1.current { era s1 s1.current with fin := s.now, st := 5 }
    generalize setEra s1 s1.current { era s1 s1.current with fin := s.now, st := 5 } = s2 at h2 ⊢
    split
    · have h3 := bcore_setEra s2 s2.pending { era s2 s2.pending with st := 4 }
      generalize setEra s2 s2.pending { era s2 s2.pending with st := 4 } = s3 at h3 ⊢
      rw [← h2, ← h3]; rfl
    · rw [← h2]; rfl

section
variable (s : St) (hb : InvB s)
include hb

theorem invB_tick (dt : Int) : InvB (apply s (.tick dt)) := by frameB
theorem invB_activate (m : Int) : InvB (apply s (.activate m)) := by frameB
theorem invB_cancel (g m : Int) : InvB (apply s (.cancel g m)) := by frameB
theorem invB_rotate (g r : Int) : InvB (apply s (.rotateRoster g r)) := by frameB
theorem invB_cancelRotation (g : Int) : InvB (apply s (.cancelRotation g)) := by frameB
theorem invB_attest (g r : Int) : InvB (apply s (.attest g r)) := by frameB
theorem invB_vkChange : InvB (apply s .vkChange) := by frameB
theorem invB_report (b : Bool) : InvB (apply s (.reportLedger b)) := by frameB
theorem invB_acceptCall (e : Int) : InvB (apply s (.acceptCall e)) := by frameB
theorem invB_useClaim (e : Int) : InvB (apply s (.useClaim e)) := by frameB
theorem invB_useAttested : InvB (apply s .useAttested) := by frameB

/-- ERT-002: an announcement needs the sealed table. -/
theorem invB_announce (g k l t : Int) : InvB (apply s (.announce g k l t)) := by
  cases hen : enabled s (.announce g k l t)
  · rw [apply_dis hen]; exact hb
  · rw [apply_en hen]; simp only [applyE]
    obtain ⟨b1, b2, b3⟩ := hb
    (repeat' split) <;> (try exact ⟨b1, b2, b3⟩)
    rename_i hs _ _ _ _ _ _
    unfold setEra
    (repeat' split) <;> exact ⟨fun _ => by simpa using hs, by simpa using b2, by simpa using b3⟩

/-- ERT-001: the table is deployed once; it is sealed only by its own deploy. -/
theorem invB_deploy (t c th ctr : Int) (vk : Bool) (l : Int) :
    InvB (apply s (.deploy t c th ctr vk l)) := by
  cases hen : enabled s (.deploy t c th ctr vk l)
  · rw [apply_dis hen]; exact hb
  · rw [apply_en hen]
    obtain ⟨b1, b2, b3⟩ := hb
    simp only [enabled, isSlot, decide_eq_true_eq, Bool.or_eq_true, Bool.and_eq_true] at hen
    simp only [applyE, setLedger]
    (repeat' split) <;> refine ⟨?_, ?_, ?_⟩
    all_goals (try simp_all)
    all_goals (intro; omega)

/-- EGV-009 (LP-6): a premise result is written once, so a failed A2 or A3 stays. -/
theorem invB_premise (i : Int) (f : Bool) : InvB (apply s (.premise i f)) := by
  cases hen : enabled s (.premise i f)
  · rw [apply_dis hen]; exact hb
  · rw [apply_en hen]
    obtain ⟨b1, b2, b3⟩ := hb
    simp only [enabled, isSlot, decide_eq_true_eq, Bool.and_eq_true] at hen
    simp only [applyE, setPrem]
    (repeat' split) <;> refine ⟨b1, b2, ?_⟩ <;> simp_all

theorem invB_addEraRow : InvB (apply s .addEraRow) := by
  cases hen : enabled s .addEraRow
  · rw [apply_dis hen]; exact hb
  · rw [apply_en hen]; obtain ⟨b1, b2, b3⟩ := hb
    exact ⟨b1, b2, fun _ => Or.inl rfl⟩

/-- EGV-005: the rekey runner starts only with an era row or a failed A2 or A3. -/
theorem invB_rekeyStart : InvB (apply s .rekeyStart) := by
  cases hen : enabled s .rekeyStart
  · rw [apply_dis hen]; exact hb
  · rw [apply_en hen]; obtain ⟨b1, b2, b3⟩ := hb
    simp only [applyE]
    split
    · exact ⟨b1, b2, b3⟩
    · rename_i hc
      refine ⟨b1, b2, fun _ => ?_⟩
      by_cases he : s.eraRow = true
      · exact Or.inl he
      · right; simp only [fallback] at hc; simp only []; simp [he] at hc; omega
end

theorem invB_apply (s : St) (op : Op) (hb : InvB s) : InvB (apply s op) := by
  cases op with
  | tick dt => exact invB_tick s hb dt
  | announce g k l t => exact invB_announce s hb g k l t
  | activate m => exact invB_activate s hb m
  | cancel g m => exact invB_cancel s hb g m
  | soundnessEnd g => exact invB_soundnessEnd s hb g
  | rotateRoster g r => exact invB_rotate s hb g r
  | cancelRotation g => exact invB_cancelRotation s hb g
  | attest g r => exact invB_attest s hb g r
  | deploy t c th ctr vk l => exact invB_deploy s hb t c th ctr vk l
  | vkChange => exact invB_vkChange s hb
  | reportLedger b => exact invB_report s hb b
  | premise i f => exact invB_premise s hb i f
  | addEraRow => exact invB_addEraRow s hb
  | rekeyStart => exact invB_rekeyStart s hb
  | acceptCall e => exact invB_acceptCall s hb e
  | useClaim e => exact invB_useClaim s hb e
  | useAttested => exact invB_useAttested s hb

theorem invA_apply (s : St) (op : Op) (h : InvA s) : InvA (apply s op) := by
  cases op with
  | tick dt => exact invA_tick s h dt
  | announce sg k l t => exact invA_announce s h sg k l t
  | activate m => exact invA_activate s h m
  | cancel sg m => exact invA_cancel s h sg m
  | soundnessEnd sg => exact invA_soundnessEnd s h sg
  | rotateRoster g r => exact invA_rotate s h g r
  | cancelRotation g => exact invA_cancelRotation s h g
  | attest g r => exact invA_attest s h g r
  | deploy t c th ctr vk l => exact invA_deploy s h t c th ctr vk l
  | vkChange => exact invA_vkChange s h
  | reportLedger b => exact invA_report s h b
  | premise i f => exact invA_premise s h i f
  | addEraRow => exact invA_addEraRow s h
  | rekeyStart => exact invA_rekeyStart s h
  | acceptCall e => exact invA_acceptCall s h e
  | useClaim e => exact invA_useClaim s h e
  | useAttested => exact invA_useAttested s h

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) :=
  ⟨invA_apply s op h.1, invB_apply s op h.2⟩

theorem inv_step (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp; split
  · exact h
  · exact inv_apply s _ h

theorem inv_runOps (s : St) (ops : List Int) (h : Inv s) : Inv (runOps s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_step s n h)

/-- `Inv` holds after every run from `init`. -/
theorem inv_run (tl rtl q n grid : Int) (h : paramsOk tl rtl q n grid = true) (ops : List Int) :
    Inv (run tl rtl q n grid ops) :=
  inv_runOps _ ops (inv_init tl rtl q n grid h)

end Governance
