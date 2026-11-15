import Governance.Props

/-!
# Requirement-level corollaries

One theorem per requirement sentence, named `<ID>_<slug>`. The invariant corollaries
read `Inv` (`inv_run`); the step theorems read one `apply`.
-/

set_option linter.unusedSimpArgs false

namespace Governance

theorem setEra_current (s : St) (i : Int) (E : Era) : (setEra s i E).current = s.current := by
  unfold setEra; split
  · rfl
  · split <;> rfl

theorem era_setEra (s : St) (i : Int) (E : Era) : era (setEra s i E) i = E := by
  unfold era setEra
  by_cases a : i = 1
  · simp [a]
  · by_cases b : i = 2
    · simp [b]
    · simp [a, b]

/-! ## From the invariant -/

/-- GOV-007: an era id is used iff it is below `nextId`; `nextId` only grows, so a
cancelled id is never used again. -/
theorem GOV_007_ids_in_order (s : St) (h : Inv s) (i : Int) (h1 : 1 ≤ i) (h3 : i ≤ 3) :
    (era s i).st ≠ 0 ↔ i < s.nextId := by
  obtain ⟨⟨_, s1, s2, s3⟩, _⟩ := h
  unfold era
  by_cases a : i = 1
  · subst a; simpa [core] using s1.2.1
  · by_cases b : i = 2
    · subst b; simpa [core] using s2.2.1
    · have c : i = 3 := by omega
      subst c; simpa [core] using s3.2.1

/-- GOV-003: at most one era is current. -/
theorem GOV_003_one_current (s : St) (h : Inv s) :
    ¬(s.e1.st = 2 ∧ s.e2.st = 2) ∧ ¬(s.e1.st = 2 ∧ s.e3.st = 2) ∧ ¬(s.e2.st = 2 ∧ s.e3.st = 2) := by
  obtain ⟨⟨_, s1, s2, s3⟩, _⟩ := h
  simp only [SlotS, core] at s1 s2 s3
  omega

/-- GOV-002: at most one era is pending (announced). -/
theorem GOV_002_one_pending (s : St) (h : Inv s) :
    ¬(s.e1.st = 1 ∧ s.e2.st = 1) ∧ ¬(s.e1.st = 1 ∧ s.e3.st = 1) ∧ ¬(s.e2.st = 1 ∧ s.e3.st = 1) := by
  obtain ⟨⟨_, s1, s2, s3⟩, _⟩ := h
  simp only [SlotS, core] at s1 s2 s3
  omega

/-- ERT-002: no era is announced before the era table passed its readback gate. -/
theorem ERT_002_table_gate_first (s : St) (h : Inv s) (hn : 1 < s.nextId) : s.tableSealed = true :=
  h.2.1 hn

/-- EGV-005: the rekey runner started only with an era row or a failed A2 or A3. -/
theorem EGV_005_rekey_gate (s : St) (h : Inv s) (hr : s.rekeyStarted = true) :
    s.eraRow = true ∨ s.prem2 = 2 ∨ s.prem3 = 2 :=
  h.2.2.2 hr

/-! ## One step -/

/-- GOV-001: `announce` below the quorum changes nothing but `lastRefusal`. -/
theorem GOV_001_quorum_announce (s : St) (g k l t : Int) (hq : g < s.q) :
    apply s (.announce g k l t) = s ∨ apply s (.announce g k l t) = refuse s rNoQuorum := by
  unfold apply; split
  · right; simp [applyE, hq]
  · left; rfl

/-- GOV-001: `cancel` below the quorum. -/
theorem GOV_001_quorum_cancel (s : St) (g m : Int) (hq : g < s.q) :
    apply s (.cancel g m) = s ∨ apply s (.cancel g m) = refuse s rNoQuorum := by
  unfold apply; split
  · right; simp [applyE, hq]
  · left; rfl

/-- GOV-001: `soundness_end` below the quorum. -/
theorem GOV_001_quorum_soundnessEnd (s : St) (g : Int) (hq : g < s.q) :
    apply s (.soundnessEnd g) = refuse s rNoQuorum := by
  rw [apply_en rfl]; simp [applyE, hq]

/-- GOV-001: `rotate_roster` below the quorum. -/
theorem GOV_001_quorum_rotate (s : St) (g r : Int) (hq : g < s.q) :
    apply s (.rotateRoster g r) = s ∨ apply s (.rotateRoster g r) = refuse s rNoQuorum := by
  unfold apply; split
  · right; simp [applyE, hq]
  · left; rfl

/-- GOV-002: an accepted announcement adds era `nextId` as announced with `start = T`
and sets `pending`. -/
theorem GOV_002_announce (s : St) (g k l t : Int) (hen : enabled s (.announce g k l t) = true)
    (hq : s.q ≤ g) (hs : s.tableSealed = true) (hp : s.pending = 0) (ht : s.now + s.tl ≤ t)
    (hg : t % s.grid = 0) (hk : keysetUsed s k = false) (hl : ledgerUsed s l = false)
    (hlg : ledgerState s l = 1) :
    (apply s (.announce g k l t)).pending = s.nextId ∧
    era (apply s (.announce g k l t)) s.nextId =
      { st := 1, start := t, fin := 0, keyset := k, ledger := l, annAt := s.now } := by
  rw [apply_en hen]
  simp only [applyE, show ¬g < s.q by omega, hs, hp, show ¬t < s.now + s.tl by omega, hg, hk, hl,
    hlg, ↓reduceIte, ne_eq, not_true_eq_false, Bool.false_eq_true, not_false_eq_true]
  exact ⟨rfl, era_setEra _ _ _⟩

/-- GOV-002, GOV-004: `T < now + TL` is refused. -/
theorem GOV_004_too_early (s : St) (g k l t : Int) (hen : enabled s (.announce g k l t) = true)
    (hq : s.q ≤ g) (hs : s.tableSealed = true) (hp : s.pending = 0) (ht : t < s.now + s.tl) :
    apply s (.announce g k l t) = refuse s rTooEarly := by
  rw [apply_en hen]
  simp [applyE, show ¬g < s.q by omega, hs, hp, ht]

/-- ERT-009: a start off the grid is refused. -/
theorem ERT_009_grid (s : St) (g k l t : Int) (hen : enabled s (.announce g k l t) = true)
    (hq : s.q ≤ g) (hs : s.tableSealed = true) (hp : s.pending = 0) (ht : s.now + s.tl ≤ t)
    (hg : t % s.grid ≠ 0) :
    apply s (.announce g k l t) = refuse s rOffGrid := by
  rw [apply_en hen]
  simp [applyE, show ¬g < s.q by omega, hs, hp, show ¬t < s.now + s.tl by omega, hg]

/-- ERT-003, EGV-004: a key set that an earlier era holds is refused. -/
theorem ERT_003_keyset (s : St) (g k l t : Int) (hen : enabled s (.announce g k l t) = true)
    (hq : s.q ≤ g) (hs : s.tableSealed = true) (hp : s.pending = 0) (ht : s.now + s.tl ≤ t)
    (hg : t % s.grid = 0) (hk : keysetUsed s k = true) :
    apply s (.announce g k l t) = refuse s rKeysetReused := by
  rw [apply_en hen]
  simp [applyE, show ¬g < s.q by omega, hs, hp, show ¬t < s.now + s.tl by omega, hg, hk]

/-- ERT-003: a ledger that an earlier era holds is refused. -/
theorem ERT_003_ledger (s : St) (g k l t : Int) (hen : enabled s (.announce g k l t) = true)
    (hq : s.q ≤ g) (hs : s.tableSealed = true) (hp : s.pending = 0) (ht : s.now + s.tl ≤ t)
    (hg : t % s.grid = 0) (hk : keysetUsed s k = false) (hl : ledgerUsed s l = true) :
    apply s (.announce g k l t) = refuse s rLedgerReused := by
  rw [apply_en hen]
  simp [applyE, show ¬g < s.q by omega, hs, hp, show ¬t < s.now + s.tl by omega, hg, hk, hl]

/-- ELD-002, EGV-002: a ledger that has not passed its readback gate is refused. -/
theorem ELD_002_ledger_gate (s : St) (g k l t : Int) (hen : enabled s (.announce g k l t) = true)
    (hq : s.q ≤ g) (hs : s.tableSealed = true) (hp : s.pending = 0) (ht : s.now + s.tl ≤ t)
    (hg : t % s.grid = 0) (hk : keysetUsed s k = false) (hl : ledgerUsed s l = false)
    (hlg : ledgerState s l ≠ 1) :
    apply s (.announce g k l t) = refuse s rLedgerNotGated := by
  rw [apply_en hen]
  simp [applyE, show ¬g < s.q by omega, hs, hp, show ¬t < s.now + s.tl by omega, hg, hk, hl, hlg]

/-- GOV-003: `current` changes by `activate` only for the pending era at or after its start. -/
theorem GOV_003_activate_at_start (s : St) (m : Int)
    (hc : (apply s (.activate m)).current ≠ s.current) :
    s.pending = m ∧ (era s m).start ≤ s.now := by
  unfold apply at hc
  split at hc
  · simp only [applyE] at hc
    split at hc
    · simp [refuse] at hc
    · split at hc
      · simp [refuse] at hc
      · rename_i h1 h2
        exact ⟨by simpa using h1, by omega⟩
  · exact absurd rfl hc

/-- GOV-007: an accepted cancel marks the pending era `cancelled` and clears `pending`. -/
theorem GOV_007_cancel (s : St) (g m : Int) (hen : enabled s (.cancel g m) = true)
    (hq : s.q ≤ g) (hp : s.pending = m) (ht : s.now < (era s m).start) :
    (era (apply s (.cancel g m)) m).st = 4 ∧ (apply s (.cancel g m)).pending = 0 := by
  rw [apply_en hen]
  simp only [applyE]
  rw [ite_f (by omega), ite_f (by simp only [hp, ne_eq, not_true_eq_false, false_or]; omega)]
  exact ⟨congrArg Era.st (era_setEra s m { era s m with st := 4 }), rfl⟩

/-- GOV-007: a cancel at or after `start(M)` is refused. -/
theorem GOV_007_cancel_late (s : St) (g m : Int) (hen : enabled s (.cancel g m) = true)
    (hq : s.q ≤ g) (ht : (era s m).start ≤ s.now) :
    apply s (.cancel g m) = refuse s rCancelLate := by
  rw [apply_en hen]
  simp [applyE, show ¬g < s.q by omega, ht]

/-- GOV-005: a rotation changes no era record, ledger, claim or nullifier, and the roster
in force only from `now + RTL`. -/
theorem GOV_005_rotation_frame (s : St) (g r : Int) :
    let s' := apply s (.rotateRoster g r)
    s'.e1 = s.e1 ∧ s'.e2 = s.e2 ∧ s'.e3 = s.e3 ∧ s'.l1 = s.l1 ∧ s'.l2 = s.l2 ∧ s'.l3 = s.l3
      ∧ s'.c1 = s.c1 ∧ s'.c2 = s.c2 ∧ s'.c3 = s.c3 ∧ s'.uses = s.uses ∧ s'.roster = s.roster := by
  unfold apply; split
  · simp only [applyE]; split <;> simp [refuse]
  · simp

/-- GOV-005: a pending rotation takes effect once block time reaches its time. -/
theorem GOV_005_rotation_effect (s : St) (dt : Int) (hd : 0 ≤ dt) (hp : s.rosterPending ≠ 0)
    (ht : s.rosterAt ≤ s.now + dt) : (apply s (.tick dt)).roster = s.rosterPending := by
  rw [apply_en (by simp [enabled, hd])]
  simp [applyE, hp, ht]

/-- GOV-011: an accepted cancel of a pending rotation drops it. -/
theorem GOV_011_cancel_rotation (s : St) (g : Int) (hq : s.q ≤ g) (hp : s.rosterPending ≠ 0) :
    (apply s (.cancelRotation g)).rosterPending = 0 := by
  rw [apply_en rfl]; simp [applyE, show ¬g < s.q by omega, hp]

/-- GOV-008: an attested root is used only from `TL` after it was recorded. -/
theorem GOV_008_attest_delay (s : St) (hu : (apply s .useAttested).attestUses ≠ s.attestUses) :
    s.attestRoot ≠ 0 ∧ s.attestAt + s.tl ≤ s.now := by
  rw [apply_en rfl] at hu
  simp only [applyE] at hu
  split at hu
  · simp [refuse] at hu
  · rename_i h; omega

/-- GOV-010: an accepted signed rotation, attestation or rotation cancel increments the nonce. -/
theorem GOV_010_nonce (s : St) (g r : Int) (hq : s.q ≤ g) (hr : 1 ≤ r) :
    (apply s (.rotateRoster g r)).nonce = s.nonce + 1 ∧ (apply s (.attest g r)).nonce = s.nonce + 1 := by
  rw [apply_en (by simp [enabled, hr]), apply_en (by simp [enabled, hr])]
  simp [applyE, show ¬g < s.q by omega]

/-- EGV-007, GOV-009: a verifier-key change is refused and changes nothing else. -/
theorem EGV_007_vk_change (s : St) : apply s .vkChange = refuse s rVkChange := by
  rw [apply_en rfl]; rfl

/-- EGV-003: an unsealed ledger's receipts are counted as transaction-path evidence. -/
theorem EGV_003_path_evidence (s : St) :
    (apply s (.reportLedger false)).pathEvidence = s.pathEvidence + 1 := by
  rw [apply_en rfl]; simp [applyE]

/-- EGV-002, ERT-002 (LP-2): a table readback other than `{[], 1, 0}` leaves the table unsealed. -/
theorem EGV_002_table_readback (s : St) (c th ctr : Int) (vk : Bool)
    (hen : enabled s (.deploy 0 c th ctr vk 0) = true) (hbad : ¬(c = 0 ∧ th = 1 ∧ ctr = 0 ∧ vk = true)) :
    (apply s (.deploy 0 c th ctr vk 0)).tableSealed = false ∧
    (apply s (.deploy 0 c th ctr vk 0)).unusable = s.unusable + 1 := by
  rw [apply_en hen]
  simp only [applyE, ↓reduceIte]
  simp_all

/-- EGV-002, ELD-002 (LP-2): an era ledger readback other than `{[], 1, 0}` marks it unusable. -/
theorem EGV_002_ledger_readback (s : St) (c th ctr : Int) (vk : Bool) (l : Int)
    (hen : enabled s (.deploy 1 c th ctr vk l) = true) (hbad : ¬(c = 0 ∧ th = 1 ∧ ctr = 0 ∧ vk = true)) :
    ledgerState (apply s (.deploy 1 c th ctr vk l)) l = 2 := by
  rw [apply_en hen]
  simp only [applyE, ledgerState, setLedger]
  by_cases a : l = 1
  · simp_all
  · by_cases b : l = 2 <;> simp_all

/-- ELD-004, ELD-003: on the primary path a call for an inactive era is refused whole. -/
theorem ELD_004_refuse_call (s : St) (e : Int) (hen : enabled s (.acceptCall e) = true)
    (hf : fallback s = false) (ha : active s e = false) :
    apply s (.acceptCall e) = refuse s rEraInactive := by
  rw [apply_en hen]; simp [applyE, hf, ha]

/-- ELD-007: under the fallback the era ledger accepts without the era check. -/
theorem ELD_007_fallback_no_check (s : St) (e : Int) (hen : enabled s (.acceptCall e) = true)
    (hf : fallback s = true) : (apply s (.acceptCall e)).lastRefusal = 0 := by
  rw [apply_en hen]; simp only [applyE, hf]; simp

/-- CRR-004: a claim of a `soundness_ended` era is refused. -/
theorem CRR_004_unsafe_era (s : St) (e : Int) (hen : enabled s (.useClaim e) = true)
    (hs : (era s e).st = 5) : apply s (.useClaim e) = refuse s rSoundnessEnded := by
  rw [apply_en hen]; simp [applyE, hs]

/-- CRR-006, EGV-006: under the fallback the consumer refuses a use of an inactive era. -/
theorem CRR_006_check_at_use (s : St) (e : Int) (hen : enabled s (.useClaim e) = true)
    (hs : (era s e).st ≠ 5) (hf : fallback s = true) (ha : active s e = false) :
    apply s (.useClaim e) = refuse s rEraInactive := by
  rw [apply_en hen]; simp [applyE, hs, hf, ha]

/-- CRR-007: a use writes only the consumer's own nullifier count. -/
theorem CRR_007_use_writes_own (s : St) (e : Int) :
    apply s (.useClaim e) = s ∨ (∃ c, apply s (.useClaim e) = refuse s c)
      ∨ apply s (.useClaim e) = { s with uses := s.uses + 1, lastRefusal := 0 } := by
  unfold apply; split
  · simp only [applyE]
    split
    · exact Or.inr (Or.inl ⟨_, rfl⟩)
    · split
      · exact Or.inr (Or.inl ⟨_, rfl⟩)
      · exact Or.inr (Or.inr rfl)
  · exact Or.inl rfl

/-- EGV-009 (LP-6): each premise gets one result; a second result is not enabled. -/
theorem EGV_009_premise_once (s : St) (f : Bool) (h : s.prem2 ≠ 0) :
    enabled s (.premise 2 f) = false := by
  simp [enabled, isSlot, h]

end Governance
