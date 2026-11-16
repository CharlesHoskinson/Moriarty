import Clean.Sherlock.MemContract
import Clean.Sherlock.ModularComposition

/-!
# MEM obligations O12, O13, O43, O47 (task 1.3)

`openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-chip-mem` (MEM-001 to MEM-007);
`research/sherlock-final-design-20261111/ARCHITECTURE.md`, Lean obligations.

* **O12** (address bound, extends `PartitionIndependence.lean`): if every step appends at most
  `a e` physical cells, a run of at most `2^64` steps from genesis length `len_0` ends at a length
  of at most `len_0 + a e 2^64`, so with `len_0 + a e 2^64 <= 2^(TOP+3) - 1` every address fits the
  `TOP + 3` bits of the MEM frontier (`addr_bound`). `top63_bound` checks the working values
  `a = 3`, `e = 1`, TOP = 63 for every `len_0 < 2^64`.
* **O43** (`MemHist`): the physical memory after the leaves is the canonical state of the log of
  every appended record, in address order. INIT gives it at genesis (`memHist_genesis`), each leaf
  keeps it inside the window capacity (`memHist_leaf`), and so does every chain of leaves
  (`memHist_leaves`). The tail a leaf imports is the log at addresses `8 q .. len - 1`
  (`memHist_tail`).
* **O13** (MEM window as a `Module`): the window requests the backend's tail tuple of `len_pre`
  and provides one MEM tuple per imported and appended row. `mem_contract`: if the tail digest is
  injective, the window meets its bus contract, so in an accepted leaf every MEM tuple CORE reads is
  the log's record at its address (`mem_leaf_sound`). `backend_tail_valid`: a backend whose state
  satisfies `MemHist` provides a valid tail tuple.
* **O47** (every accepted leaf fits the window): the window has `8 W + 8` rows, its active rows are a
  prefix and its last row is inactive, so `r_pre + appends <= 8 W + 7` (`window_active_lt`). Every
  cut of accepted leaves satisfies `FitsWindow W` (`accepted_fits`), and SRC-023 holds for it at the
  window capacity (`src023_accepted`, with O14).

Core Lean only. `MemModuleAxioms.lean` prints the axioms of every theorem.
-/

namespace Clean.Sherlock.MemModule

open Clean.Sherlock.Partition
open Clean.Sherlock.Modular
open Clean.Sherlock.MemContract

variable {C F T R : Type}

/-! ## The log -/

/-- The log's records at addresses `a .. a + k - 1`. -/
def logFrom (log : Nat → C) (a : Nat) : Nat → List C
  | 0 => []
  | k + 1 => log a :: logFrom log (a + 1) k

theorem logFrom_length (log : Nat → C) : ∀ (a k : Nat), (logFrom log a k).length = k
  | _, 0 => rfl
  | a, k + 1 => by simp [logFrom, logFrom_length log (a + 1) k]

theorem logFrom_add (log : Nat → C) : ∀ (a n k : Nat),
    logFrom log a (n + k) = logFrom log a n ++ logFrom log (a + n) k
  | a, 0, k => by simp [logFrom]
  | a, n + 1, k => by
    rw [Nat.add_right_comm, logFrom, logFrom, logFrom_add log (a + 1) n k, Nat.add_right_comm a 1 n,
      Nat.add_assoc a n 1, Nat.add_comm n 1, ← Nat.add_assoc]
    rfl

theorem logFrom_drop (log : Nat → C) : ∀ (a n j : Nat), j ≤ n →
    (logFrom log a n).drop j = logFrom log (a + j) (n - j)
  | a, n, 0, _ => by simp
  | a, 0, j + 1, h => by omega
  | a, n + 1, j + 1, h => by
    simp only [logFrom, List.drop_succ_cons]
    rw [logFrom_drop log (a + 1) n j (by omega), Nat.add_assoc, Nat.add_comm 1 j,
      Nat.add_sub_add_right]

/-! ## O43: the memory history -/

/-- `MemHist`: the physical memory `m` is the canonical state of the first `n` log records. -/
def MemHist (log : Nat → C) (n : Nat) (m : Mem C) : Prop := m = canon (logFrom log 0 n)

theorem memHist_genesis (log : Nat → C) : MemHist log 0 ⟨[], []⟩ := by
  simp [MemHist, logFrom, canon, appendCells]

theorem memHist_len {log : Nat → C} {n : Nat} {m : Mem C} (h : MemHist log n m) : m.len = n := by
  rw [h, canon_len, logFrom_length]

theorem flatten_len8 : ∀ (s : List (List C)), (∀ g ∈ s, g.length = 8) → s.flatten.length = 8 * s.length
  | [], _ => rfl
  | g :: s, h => by
    simp only [List.flatten_cons, List.length_append, List.length_cons]
    rw [h g (by simp), flatten_len8 s (fun x hx ↦ h x (by simp [hx]))]
    omega

/-- The tail is the log at addresses `8 q .. n - 1`, in address order. -/
theorem memHist_tail {log : Nat → C} {n : Nat} {m : Mem C} (h : MemHist log n m) :
    m.tail = logFrom log (8 * (n / 8)) (n % 8) := by
  have hw : m.WF := by rw [h]; exact canon_wf _
  have hc : m.cells = logFrom log 0 n := by rw [h]; exact canon_cells _
  have hl := memHist_len h
  have hf := flatten_len8 m.segs hw.1
  have ht := hw.2
  simp only [Mem.len] at hl
  have hq : n / 8 = m.segs.length := by omega
  have hr : n % 8 = m.tail.length := by omega
  have hd : m.cells.drop (m.segs.flatten.length) = m.tail := by simp [Mem.cells]
  rw [← hd, hc, hf, logFrom_drop log 0 n _ (by omega), hq]
  congr 1
  · omega
  · omega

/-- The records one leaf appends: the log at `n .. n + k - 1`. -/
def leafApps (log : Nat → C) (n k : Nat) : List C := logFrom log n k

/-- A leaf of `k` appends inside the window capacity keeps `MemHist`. -/
theorem memHist_leaf (W : Nat) (log : Nat → C) {n k : Nat} {m : Mem C} (h : MemHist log n m)
    (hcap : m.tail.length + k ≤ 8 * W + 7) :
    MemHist log (n + k) (leafFlushW W m (leafApps log n k)) := by
  have hcap' : m.tail.length + (leafApps log n k).length ≤ 8 * W + 7 := by
    rw [leafApps, logFrom_length]; exact hcap
  have hw : m.WF := by rw [h]; exact canon_wf _
  rw [leafFlushW_eq hw _ hcap']
  unfold MemHist at h ⊢
  rw [h, ← canon_append, logFrom_add, Nat.zero_add]
  rfl

/-- The leaves of a cut `ks` from length `n`. -/
def runLeaves (W : Nat) (log : Nat → C) : Nat → List Nat → Mem C → Mem C
  | _, [], m => m
  | n, k :: ks, m => runLeaves W log (n + k) ks (leafFlushW W m (leafApps log n k))

/-- Every leaf of the cut fits the window. -/
def FitsLog (W : Nat) (log : Nat → C) : Nat → List Nat → Prop
  | _, [] => True
  | n, k :: ks => n % 8 + k ≤ 8 * W + 7 ∧ FitsLog W log (n + k) ks

/-- **O43.** `MemHist` holds after every chain of leaves that fit the window. -/
theorem memHist_leaves (W : Nat) (log : Nat → C) : ∀ (ks : List Nat) (n : Nat) (m : Mem C),
    MemHist log n m → FitsLog W log n ks → MemHist log (n + ks.sum) (runLeaves W log n ks m)
  | [], n, m, h, _ => by simpa [runLeaves] using h
  | k :: ks, n, m, h, ⟨hk, hks⟩ => by
    have hr : m.tail.length = n % 8 := by rw [memHist_tail h, logFrom_length]
    have h1 := memHist_leaf W log h (by rw [hr]; exact hk)
    have h2 := memHist_leaves W log ks (n + k) _ h1 hks
    simp only [runLeaves, List.sum_cons]
    rwa [← Nat.add_assoc]

/-! ## O13: the MEM window as a `Module` -/

/-- The bus tuples of the MEM window: the backend's tail tuple and the MEM tuples to CORE. -/
inductive MTup (C D : Type) where
  | tail (n : Nat) (d : D)
  | cell (a : Nat) (c : C)

/-- The meaning of a tuple against the log (`tdig` is the canonical tail digest). -/
def Valid {D : Type} (log : Nat → C) (tdig : List C → D) : MTup C D → Prop
  | .tail n d => d = tdig (logFrom log (8 * (n / 8)) (n % 8))
  | .cell a c => log a = c

/-- The backend below MEM below CORE. -/
def level {D : Type} : MTup C D → Nat
  | .tail _ _ => 0
  | .cell _ _ => 1

/-- One MEM tuple per row, at consecutive addresses from `a`. -/
def cellsFrom {D : Type} : Nat → List C → List (MTup C D)
  | _, [] => []
  | a, c :: cs => .cell a c :: cellsFrom (a + 1) cs

/-- A window witness: `len_pre`, the imported tail rows, the appended rows, and the tail digest
that the backend opens. -/
structure WinW (C D : Type) where
  n : Nat
  imported : List C
  appended : List C
  d : D

/-- The window relation: the imported rows open the pre tail (the backend's tail tuple), and the
appended rows are the records CORE appends at `len_pre ..`, which define the log. -/
def WinRel {D : Type} (log : Nat → C) (tdig : List C → D) (w : WinW C D) : Prop :=
  tdig w.imported = w.d ∧ w.appended = logFrom log w.n w.appended.length

def memModule {D : Type} (log : Nat → C) (tdig : List C → D) : Module (WinW C D) (MTup C D) where
  rel := WinRel log tdig
  requests w := [.tail w.n w.d]
  provides w := cellsFrom (8 * (w.n / 8)) w.imported ++ cellsFrom w.n w.appended

theorem cellsFrom_valid {D : Type} (log : Nat → C) (tdig : List C → D) : ∀ (a k : Nat),
    ∀ t ∈ (cellsFrom a (logFrom log a k) : List (MTup C D)), Valid log tdig t
  | _, 0 => by simp [logFrom, cellsFrom]
  | a, k + 1 => by
    intro t ht
    simp only [logFrom, cellsFrom, List.mem_cons] at ht
    rcases ht with rfl | ht
    · rfl
    · exact cellsFrom_valid log tdig (a + 1) k t ht

/-- **O13.** With an injective tail digest, the MEM window meets its bus contract. -/
theorem mem_contract {D : Type} (log : Nat → C) (tdig : List C → D)
    (hinj : ∀ x y, tdig x = tdig y → x = y) :
    Contract (Valid log tdig) level (memModule log tdig) where
  ordered w _ r hr t ht := by
    simp only [memModule, List.mem_singleton] at hr
    subst hr
    simp only [memModule, List.mem_append] at ht
    have : ∀ a (l : List C), ∀ t ∈ (cellsFrom a l : List (MTup C D)), level t = 1 := by
      intro a l
      induction l generalizing a with
      | nil => simp [cellsFrom]
      | cons c cs ih =>
        intro t ht
        simp only [cellsFrom, List.mem_cons] at ht
        rcases ht with rfl | ht
        · rfl
        · exact ih _ t ht
    rcases ht with ht | ht <;> rw [this _ _ t ht] <;> simp [level]
  guarantee w hrel hreq t ht := by
    have hr : WinRel log tdig w := hrel
    have htail : Valid log tdig (.tail w.n w.d) := hreq (.tail w.n w.d) (by simp [memModule])
    simp only [Valid] at htail
    have himp : w.imported = logFrom log (8 * (w.n / 8)) (w.n % 8) :=
      hinj _ _ (hr.1.trans htail)
    simp only [memModule, List.mem_append] at ht
    rcases ht with ht | ht
    · rw [himp] at ht; exact cellsFrom_valid log tdig _ _ t ht
    · rw [hr.2] at ht; exact cellsFrom_valid log tdig _ _ t ht

/-- A backend whose state satisfies `MemHist` provides a valid tail tuple. -/
theorem backend_tail_valid {D : Type} (log : Nat → C) (tdig : List C → D) {n : Nat} {m : Mem C}
    (h : MemHist log n m) : Valid log tdig (.tail n (tdig m.tail)) := by
  simp only [Valid]; rw [memHist_tail h]

/-- In an accepted leaf whose modules meet their contracts, every MEM tuple CORE reads is the log's
record at its address. -/
theorem mem_leaf_sound {D : Type} {log : Nat → C} {tdig : List C → D}
    {mods : List (Module (WinW C D) (MTup C D))}
    (hc : ∀ m ∈ mods, Contract (Valid log tdig) level m) {w : WinW C D} (hl : LeafRel mods w)
    {a : Nat} {c : C} (hreq : MTup.cell a c ∈ allReqs mods w) : log a = c :=
  requested_valid hc hl _ hreq

/-! ## O47: every accepted leaf fits the window -/

/-- The window's act column: `8 W + 8` rows, a prefix of `a` active rows, the last row inactive. -/
structure WinShape (W : Nat) where
  act : List Bool
  a : Nat
  hlen : act.length = 8 * W + 8
  hpre : act = List.replicate a true ++ List.replicate (act.length - a) false
  hlast : act[act.length - 1]? = some false

/-- The active rows of the window number at most `8 W + 7`. -/
theorem window_active_lt {W : Nat} (sh : WinShape W) : sh.a ≤ 8 * W + 7 := by
  have hl := sh.hlen
  have hlast := sh.hlast
  have hp := sh.hpre
  by_cases hc : sh.a ≤ 8 * W + 7
  · exact hc
  exfalso
  have hb : sh.act.length - sh.a = 0 := by omega
  have hall : sh.act = List.replicate sh.a true := by
    have h2 := sh.hpre
    rw [hb, List.replicate_zero, List.append_nil] at h2
    exact h2
  have ha : sh.a = 8 * W + 8 := by
    have h3 := congrArg List.length hall
    simp only [List.length_replicate] at h3
    omega
  rw [hall] at hlast
  simp [List.getElem?_replicate] at hlast
  all_goals omega

/-- A cut of leaves that the window accepts: each leaf has a window whose active rows are the
`r_pre` imported rows and its appends (`len_pre = 8 b0 + r_pre`, MEM bus count `act - imp`). -/
def AcceptedCut (W : Nat) (eff : LState C F T R → Effect C F T R) :
    List Nat → LState C F T R → Prop
  | [], _ => True
  | k :: ks, s =>
    (∃ sh : WinShape W, sh.a = (canon s.mem).tail.length + (appsOf eff k s).length) ∧
      AcceptedCut W eff ks (iter (LState.next eff) k s)

/-- **O47.** Every cut of accepted leaves satisfies `FitsWindow W`. -/
theorem accepted_fits (W : Nat) (eff : LState C F T R → Effect C F T R) :
    ∀ (ks : List Nat) (s : LState C F T R), AcceptedCut W eff ks s → FitsWindow W eff ks s
  | [], _, _ => trivial
  | _ :: ks, _, ⟨⟨sh, hsh⟩, hks⟩ => ⟨hsh ▸ window_active_lt sh, accepted_fits W eff ks _ hks⟩

/-- SRC-023 at the window capacity: for every cut of accepted leaves, the physical state after the
leaves is the canonical state after `ks.sum` steps (O14 with O47). -/
theorem src023_accepted (W : Nat) (eff : LState C F T R → Effect C F T R) (ks : List Nat)
    (s : LState C F T R) (h : AcceptedCut W eff ks s) :
    runPart (closeFlushW W) eff ks s (abs s) = abs (iter (LState.next eff) ks.sum s) :=
  partition_independent_window W eff ks s (accepted_fits W eff ks s h)

/-! ## O12: the address bound -/

theorem appsOf_length_le (eff : LState C F T R → Effect C F T R) (b : Nat)
    (h : ∀ s, (eff s).app.length ≤ b) : ∀ (k : Nat) (s : LState C F T R),
    (appsOf eff k s).length ≤ b * k
  | 0, _ => by simp [appsOf]
  | k + 1, s => by
    simp only [appsOf, List.length_append]
    have := h s
    have := appsOf_length_le eff b h k (s.next eff)
    rw [Nat.mul_succ]
    omega

/-- After at most `2^64` steps of at most `a e` cells each, the length is at most
`len_0 + a e 2^64`. -/
theorem len_bound (eff : LState C F T R → Effect C F T R) (a e : Nat)
    (h : ∀ s, (eff s).app.length ≤ a * e) (N : Nat) (hN : N ≤ 2 ^ 64) (s : LState C F T R) :
    (iter (LState.next eff) N s).mem.length ≤ s.mem.length + a * e * 2 ^ 64 := by
  rw [iter_mem, List.length_append]
  have h1 := appsOf_length_le eff (a * e) h N s
  have h2 : a * e * N ≤ a * e * 2 ^ 64 := Nat.mul_le_mul_left _ hN
  omega

/-- **O12.** With `len_0 + a e 2^64 <= 2^(TOP+3) - 1`, the MEM length (and every address below it)
stays below `2^(TOP+3)` for every run of at most `2^64` steps, so the `TOP + 3` frontier bits never
wrap. -/
theorem addr_bound (eff : LState C F T R → Effect C F T R) (a e TOP : Nat)
    (h : ∀ s, (eff s).app.length ≤ a * e) (s : LState C F T R)
    (hgen : s.mem.length + a * e * 2 ^ 64 ≤ 2 ^ (TOP + 3) - 1) (N : Nat) (hN : N ≤ 2 ^ 64) :
    (canon (iter (LState.next eff) N s).mem).len < 2 ^ (TOP + 3) := by
  rw [canon_len]
  have := len_bound eff a e h N hN s
  have : 0 < 2 ^ (TOP + 3) := Nat.two_pow_pos _
  omega

/-- The working values: `a = 3`, `e = 1`, TOP = 63 satisfy the bound for every `len_0 < 2^64`. -/
theorem top63_bound (len0 : Nat) (h : len0 < 2 ^ 64) : len0 + 3 * 1 * 2 ^ 64 ≤ 2 ^ (63 + 3) - 1 := by
  have e : (2 : Nat) ^ (63 + 3) = 4 * 2 ^ 64 := by
    rw [show 63 + 3 = 2 + 64 from rfl, Nat.pow_add]
  omega

end Clean.Sherlock.MemModule
