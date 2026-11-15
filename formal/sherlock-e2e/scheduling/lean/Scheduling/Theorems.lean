import Scheduling.Spec

/-!
# Theorems of Scheduling (each names its requirement)
-/

namespace Scheduling

/-! ## STM-017: the pick is the least element of a strict total order on the picks -/

theorem better_irrefl (x : Cand) : ¬ Better x x := by
  unfold Better; omega

theorem better_trans {x y z : Cand} (h1 : Better x y) (h2 : Better y z) : Better x z := by
  unfold Better at *; omega

/-- Two picks of different shape or device are ordered one way or the other. -/
theorem better_total {x y : Cand} (h : x.shape ≠ y.shape ∨ x.dev ≠ y.dev) :
    Better x y ∨ Better y x := by
  unfold Better; omega

theorem pickList_none (l : List Cand) : pickList l = none ↔ l = [] := by
  cases l with
  | nil => simp [pickList]
  | cons x xs =>
    simp only [pickList, reduceCtorEq, iff_false]
    split <;> (try split) <;> simp

theorem pickList_mem {l : List Cand} {c : Cand} (h : pickList l = some c) : c ∈ l := by
  induction l generalizing c with
  | nil => simp [pickList] at h
  | cons x xs ih =>
    simp only [pickList] at h
    split at h
    · simp_all
    · rename_i y hy
      split at h
      · simp_all
      · simp only [Option.some.injEq] at h; subst h; exact List.mem_cons_of_mem _ (ih hy)

/-- STM-017: no candidate is better than the pick. -/
theorem pickList_least {l : List Cand} {c : Cand} (h : pickList l = some c) :
    ∀ x ∈ l, ¬ Better x c := by
  induction l generalizing c with
  | nil => simp [pickList] at h
  | cons x xs ih =>
    simp only [pickList] at h
    split at h
    · rename_i hn
      simp only [Option.some.injEq] at h; subst h
      rw [pickList_none] at hn; subst hn
      simp [better_irrefl]
    · rename_i y hy
      have hy' := ih hy
      split at h
      · rename_i hb
        simp only [Option.some.injEq] at h; subst h
        intro z hz
        rcases List.mem_cons.mp hz with rfl | hz
        · exact better_irrefl _
        · exact fun hzx => hy' z hz (better_trans hzx hb)
      · rename_i hb
        simp only [Option.some.injEq] at h; subst h
        intro z hz
        rcases List.mem_cons.mp hz with rfl | hz
        · exact hb
        · exact hy' z hz

/-- STM-017: the least estimated wall time wins. -/
theorem pickList_est_le {l : List Cand} {c : Cand} (h : pickList l = some c) :
    ∀ x ∈ l, c.est ≤ x.est := by
  intro x hx
  have := pickList_least h x hx
  unfold Better at this; omega

/-- STM-017 (scenario "A tie with reversed entries"): the pick does not depend on the order of
    the candidates, when no two candidates share a shape and a device. -/
theorem pick_order_independent {l l' : List Cand} (hm : ∀ x, x ∈ l ↔ x ∈ l')
    (hd : ∀ x ∈ l, ∀ y ∈ l, x.shape = y.shape → x.dev = y.dev → x = y) :
    pickList l = pickList l' := by
  cases h : pickList l with
  | none =>
    rw [pickList_none] at h; subst h
    symm; rw [pickList_none]
    cases l' with
    | nil => rfl
    | cons z zs => exact absurd ((hm z).mpr (List.mem_cons_self ..)) (by simp)
  | some c =>
    cases h' : pickList l' with
    | none =>
      rw [pickList_none] at h'; subst h'
      exact absurd ((hm c).mp (pickList_mem h)) (by simp)
    | some c' =>
      have hc := pickList_mem h
      have hc' := (hm c').mpr (pickList_mem h')
      have n1 := pickList_least h c' hc'
      have n2 := pickList_least h' c ((hm c).mp hc)
      by_cases hs : c.shape = c'.shape ∧ c.dev = c'.dev
      · rw [hd c hc c' hc' hs.1 hs.2]
      · have := better_total (x := c) (y := c') (by omega)
        unfold Better at n1 n2 this; omega

/-! ## STM-018, ELA-003: a lower budget never gives a faster pick -/

theorem ceilDiv_anti (m c c' : Nat) (h0 : 0 < c') (h : c' ≤ c) : ceilDiv m c ≤ ceilDiv m c' := by
  unfold ceilDiv
  have hc : 0 < c := by omega
  have h1 := Nat.div_add_mod (m + c' - 1) c'
  have h2 := Nat.mod_lt (m + c' - 1) h0
  have h3 : c' * ((m + c' - 1) / c') ≤ c * ((m + c' - 1) / c') := Nat.mul_le_mul_right _ h
  apply Nat.le_of_lt_succ
  rw [Nat.div_lt_iff_lt_mul hc, Nat.succ_mul, Nat.mul_comm ((m + c' - 1) / c') c]
  generalize (m + c' - 1) / c' = q at *
  generalize c' * q = a at *
  generalize c * q = b at *
  omega

theorem divAtLeast1_mono {a b : Nat} (d : Nat) (h : a ≤ b) : divAtLeast1 a d ≤ divAtLeast1 b d :=
  Nat.div_le_div_right h

/-- STM-011: the concurrency a device allows does not rise when the host budget falls. -/
theorem concOf_mono (s : St) (e : Nat) {h' h : Nat} (hh : h' ≤ h) : concOf s e h' ≤ concOf s e h := by
  unfold concOf
  simp only
  split
  · have := divAtLeast1_mono (s.calHost.getD e 0) hh; omega
  · split
    · exact Nat.le_refl _
    · split
      · split
        · exact divAtLeast1_mono _ (by omega)
        · exact divAtLeast1_mono _ hh
      · have := divAtLeast1_mono (s.calHost.getD e 0) hh; omega

theorem cand_mono (s : St) (e : Nat) {h' h : Nat} (hh : h' ≤ h) {c' : Cand}
    (hc : cand s h' e = some c') : ∃ c, cand s h e = some c ∧ c.est ≤ c'.est := by
  unfold cand at hc ⊢
  simp only at hc ⊢
  have hm := concOf_mono s e hh
  split at hc
  · rename_i hok
    simp only [Option.some.injEq] at hc; subst hc
    have hpos : 0 < min (concOf s e h) (leavesOf s e) := by omega
    rw [ite_eq_left ⟨hok.1, hpos⟩]
    refine ⟨_, rfl, ?_⟩
    simp only
    exact Nat.mul_le_mul_right _ (ceilDiv_anti _ _ _ hok.2 (by omega))
  · simp at hc

/-- STM-018: when the budget is lowered, a pick at the lower budget is never faster than the
    pick at the higher budget (and the higher budget has a pick). -/
theorem pickAt_mono (s : St) {h' h : Nat} (hh : h' ≤ h) {c' : Cand} (hc : pickAt s h' = some c') :
    ∃ c, pickAt s h = some c ∧ c.est ≤ c'.est := by
  have hmem := pickList_mem hc
  unfold candsAt at hmem
  obtain ⟨e, _, he⟩ := List.mem_filterMap.mp hmem
  obtain ⟨c0, h0, hle⟩ := cand_mono s e hh he
  have hc0 : c0 ∈ candsAt s h := List.mem_filterMap.mpr ⟨e, by simp_all [entries], h0⟩
  cases hp : pickAt s h with
  | none =>
    unfold pickAt at hp; rw [pickList_none] at hp; rw [hp] at hc0; simp at hc0
  | some c =>
    exact ⟨c, rfl, Nat.le_trans (pickList_est_le hp c0 hc0) hle⟩

/-- STM-012, ELA-003: a budget that fits no shape stays so when lowered further. -/
theorem pickAt_none_mono (s : St) {h' h : Nat} (hh : h' ≤ h) (hn : pickAt s h = none) :
    pickAt s h' = none := by
  cases hp : pickAt s h' with
  | none => rfl
  | some c' =>
    obtain ⟨c, hc, _⟩ := pickAt_mono s hh hp
    rw [hn] at hc; simp at hc

/-- STM-010: a limit only lowers the profile budget and the GPU count. -/
theorem limits_only_lower (s : St) : hostBudget s ≤ s.memBudget ∧ gpusUsable s ≤ s.gpuCount := by
  unfold hostBudget gpusUsable
  constructor <;> split <;> omega

/-- STM-003: the budget is the lower of `MemAvailable` and the cgroup headroom. -/
theorem budgetOf_le (a c : Nat) : budgetOf a c ≤ a ∧ (c ≠ 0 → budgetOf a c ≤ c) := by
  unfold budgetOf; constructor <;> split <;> omega

/-- STM-011: the CPU concurrency is at most max(1, cores / 4), and no pick has more leaves at once
    than leaves. -/
theorem cpu_conc_cap (s : St) (e h : Nat) (he : e % 2 = 1) :
    concOf s e h ≤ max 1 (s.profCores / cpuThreadsPerLeaf) := by
  unfold concOf; simp only [he, ite_true]; omega

theorem cand_conc_le_leaves (s : St) (h e : Nat) (c : Cand) (hc : cand s h e = some c) :
    c.conc ≤ c.m ∧ 0 < c.conc := by
  unfold cand at hc; simp only at hc
  split at hc
  · rename_i hok; simp only [Option.some.injEq] at hc; subst hc; simp only; omega
  · simp at hc

/-- ELA-004: with no usable GPU every pick is on the CPU. -/
theorem no_gpu_cpu_pick (s : St) (h : Nat) (hg : gpusUsable s = 0) (c : Cand)
    (hc : pickAt s h = some c) : c.dev = 1 := by
  have hmem := pickList_mem hc
  unfold candsAt at hmem
  obtain ⟨e, _, he⟩ := List.mem_filterMap.mp hmem
  unfold cand at he; simp only at he
  split at he
  · rename_i hok
    simp only [Option.some.injEq] at he; subst he; simp only
    have : e % 2 = 1 := by
      refine Classical.byContradiction fun hne => ?_
      have h0 : concOf s e h = 0 := by unfold concOf; simp [hg]; omega
      omega
    exact this
  · simp at he

/-! ## STM-009, STM-013, STM-014: the scheduler's own cut passes `check_free` -/

theorem cutAux_props (cap : Nat) (hcap : 0 < cap) :
    ∀ k left, k ≤ left → left ≤ k * cap →
      (cutAux cap left k).sum = left ∧ (cutAux cap left k).length = k ∧
      ∀ x ∈ cutAux cap left k, 1 ≤ x ∧ x ≤ cap := by
  intro k
  induction k with
  | zero => intro left h1 h2; simp [cutAux]; omega
  | succ k ih =>
    intro left h1 h2
    simp only [cutAux]
    rw [Nat.succ_mul] at h2
    have hk : k ≤ k * cap := Nat.le_mul_of_pos_right k hcap
    have := ih (left - min cap (left - k)) (by omega) (by
      generalize k * cap = kc at *; omega)
    obtain ⟨hs, hl, hx⟩ := this
    refine ⟨?_, by simp [hl], ?_⟩
    · simp only [List.sum_cons, hs]; omega
    · intro x hxm
      rcases List.mem_cons.mp hxm with rfl | hxm
      · omega
      · exact hx x hxm

theorem ceilDiv_bounds (n cap : Nat) (hn : 0 < n) (hc : 0 < cap) :
    0 < ceilDiv n cap ∧ ceilDiv n cap ≤ n ∧ n ≤ ceilDiv n cap * cap := by
  unfold ceilDiv
  have h1 := Nat.div_add_mod (n + cap - 1) cap
  have h2 := Nat.mod_lt (n + cap - 1) hc
  have hq : 0 < (n + cap - 1) / cap := Nat.div_pos (by omega) hc
  have h3 : (n + cap - 1) / cap < n + 1 := by
    rw [Nat.div_lt_iff_lt_mul hc, Nat.succ_mul]
    have : n * cap ≥ n := Nat.le_mul_of_pos_right n hc
    omega
  rw [Nat.mul_comm ((n + cap - 1) / cap) cap]
  generalize (n + cap - 1) / cap = q at *
  generalize cap * q = a at *
  refine ⟨hq, by omega, by omega⟩

/-- STM-009, STM-014: the greedy cut of `ceil(n / S)` leaves passes `check_free`, with no list of
    admitted shard counts and no menu of schedules. -/
theorem schedule_cut_free (caps : List Nat) (n shape : Nat) (hs : shape < shapes)
    (hc : 0 < caps.getD shape 0) (hn : 0 < n) :
    checkFree caps n shape (cutAux (caps.getD shape 0) n (ceilDiv n (caps.getD shape 0))) = [] := by
  obtain ⟨h0, h1, h2⟩ := ceilDiv_bounds n _ hn hc
  obtain ⟨hs', hl, hx⟩ := cutAux_props _ hc _ n h1 h2
  have hne : cutAux (caps.getD shape 0) n (ceilDiv n (caps.getD shape 0)) ≠ [] := by
    intro he; rw [he] at hl; simp only [List.length_nil] at hl; omega
  have hall : (cutAux (caps.getD shape 0) n (ceilDiv n (caps.getD shape 0))).all
      (fun c => decide (1 ≤ c ∧ c ≤ caps.getD shape 0)) = true :=
    List.all_eq_true.mpr fun x hx' => decide_eq_true (hx x hx')
  unfold checkFree
  simp only []
  rw [ite_eq_left ⟨hs, hc⟩, ite_eq_right hne, ite_eq_left hall, ite_eq_left hs']
  rfl

theorem checkFree_nil {caps : List Nat} {n shape : Nat} {cut : List Nat}
    (h : checkFree caps n shape cut = []) :
    (shape < shapes ∧ 0 < caps.getD shape 0) ∧ cut ≠ [] ∧
    (∀ c ∈ cut, 1 ≤ c ∧ c ≤ caps.getD shape 0) ∧ cut.sum = n := by
  unfold checkFree at h
  simp only [] at h
  split at h <;> split at h <;> split at h <;> split at h <;> simp_all

/-- STM-013: a proved schedule met every property of `check_free`. -/
theorem prove_guard (s : St) (h : stepRefusals s (6 * 16) = []) :
    s.scheduled = true ∧ s.schShape < shapes ∧ 0 < s.caps.getD s.schShape 0 ∧ s.cut ≠ [] ∧
    (∀ c ∈ s.cut, 1 ≤ c ∧ c ≤ s.caps.getD s.schShape 0) ∧ s.cut.sum = s.n := by
  have hd : decodeOp (6 * 16) = .prove := rfl
  unfold stepRefusals at h
  rw [hd] at h
  split at h
  · simp at h
  · simp only [] at h
    split at h
    · rename_i hsch
      obtain ⟨⟨k1, k2⟩, k3, k4, k5⟩ := checkFree_nil h
      exact ⟨hsch, k1, k2, k3, k4, k5⟩
    · simp at h

/-! ## ELA-001, ELA-002, ELA-005, SPL-011: one `claim_pub` and one key set for every schedule -/

theorem decodeOp_newRun (op : Nat) (h : decodeOp op = .newRun) : op / 16 = 7 := by
  unfold decodeOp at h
  generalize op / 16 = q at h ⊢
  generalize op % 16 = a at h
  rcases q with _ | _ | _ | _ | _ | _ | _ | _ | q <;> simp at h <;>
    (try (split at h <;> simp at h)) <;> (try rfl) <;>
    (rcases q with _ | _ | _ | q <;> simp at h <;> (try (split at h <;> simp at h)))

theorem decodeOp_moveTo (op b : Nat) (h : decodeOp op = .moveTo b) : b < boxes := by
  unfold decodeOp at h
  generalize op / 16 = q at h
  generalize op % 16 = a at h
  rcases q with _ | _ | _ | _ | _ | _ | _ | _ | _ | q <;> simp at h <;>
    (try (split at h <;> simp at h)) <;> (try omega) <;>
    (rcases q with _ | _ | q <;> simp at h <;> (try (split at h <;> simp at h)))

/-- The prepared run: program, input, `r_P`, salt. -/
def prepared (s : St) : Nat × Nat × Nat × Nat := (s.prog, s.input, s.rP, s.salt)

/-- ELA-001, SPL-011: every operation except `newRun` keeps the prepared run; the schedule,
    the box, the limits and the profile do not reach it. -/
theorem apply_keeps_prepared (s : St) (op : Nat) (h : op / 16 ≠ 7) :
    prepared (apply s op) = prepared s := by
  unfold apply
  split
  · unfold step prepared
    cases hop : decodeOp op with
    | newRun => exact absurd (decodeOp_newRun op hop) h
    | startRun => simp only; split <;> rfl
    | setup r => simp only; split <;> rfl
    | _ => rfl
  · rfl

theorem run_keeps_prepared (s : St) (ops : List Nat) (h : ∀ op ∈ ops, op / 16 ≠ 7) :
    prepared (run s ops) = prepared s := by
  induction ops generalizing s with
  | nil => rfl
  | cons op ops ih =>
    simp only [run]
    rw [ih _ (fun o ho => h o (List.mem_cons_of_mem _ ho)),
      apply_keeps_prepared s op (h op (List.mem_cons_self ..))]

/-- ELA-001, ELA-005: the claim of a proof is a function of the prepared run only. -/
theorem prove_claim (s : St) : (step s (6 * 16)).recClaim = claimOf s.prog s.input s.rP s.salt := by
  simp [step, decodeOp]

/-- ELA-001 (scenario "Five leaves against one"), ELA-005 (three boxes): two runs that differ
    only in schedule, box, limits or profile, from one prepared run, give one `claim_pub`. -/
theorem one_claim_every_schedule (s t : St) (h : prepared s = prepared t) :
    (step s (6 * 16)).recClaim = (step t (6 * 16)).recClaim := by
  simp only [prepared, Prod.mk.injEq] at h
  rw [prove_claim, prove_claim, h.1, h.2.1, h.2.2.1, h.2.2.2]

/-- ELA-001 (scenario "Two fresh runs"): a new preparation draws another salt and another claim. -/
theorem fresh_run_other_claim (s : St) :
    claimOf s.prog s.input s.rP (s.salt + 1) ≠ claimOf s.prog s.input s.rP s.salt := by
  unfold claimOf; omega

/-- ELA-002: no operation changes the prover's key set, and every proof records it. -/
theorem apply_keeps_keys (s : St) (op : Nat) : (apply s op).proverKeyD = s.proverKeyD := by
  unfold apply
  split
  · unfold step
    cases decodeOp op <;> simp only <;> (try split) <;> rfl
  · rfl

theorem prove_records_keys (s : St) : (step s (6 * 16)).recKeys = s.proverKeyD := by
  simp [step, decodeOp]

/-! ## Setup and the profile -/

/-- STM-005: a refused calibration proof stops setup and writes no profile. -/
theorem setup_refused_writes_nothing (s : St) :
    (step s 1).profWritten = s.profWritten ∧ (step s 1).profKeyD = s.profKeyD ∧
    (step s 1).memBudget = s.memBudget ∧ (step s 1).calMask = s.calMask := by
  simp [step, decodeOp]

/-- STM-006, STM-008: setup writes a profile of the `sherlock-profile/1` format with every field and
    the prover's key set; a run from it is calibrated. -/
theorem setup_writes_profile (s : St) :
    (step s 0).profWritten = true ∧ (step s 0).profFormatOk = true ∧
    (step s 0).profFieldsOk = true ∧ (step s 0).profKeyD = s.proverKeyD := by
  simp [step, decodeOp, setupOk]

/-- STM-008: a run records whether the profile is of this key set. -/
theorem startRun_calibrated (s : St) (c : Cand) (h : pick s = some c) :
    (step s 16).calibrated = decide (s.profKeyD = s.proverKeyD) := by
  simp [step, decodeOp, h, schedule]

/-- STM-007, STM-012, STM-016: a run starts only with a written, well-formed profile and a pick. -/
theorem startRun_guard (s : St) (h : stepRefusals s 16 = []) :
    s.profWritten = true ∧ profileOk s = true ∧ pick s ≠ none := by
  have hd : decodeOp 16 = .startRun := rfl
  unfold stepRefusals at h
  rw [hd] at h
  split at h
  · simp at h
  · simp only [] at h
    split at h
    · simp at h
    · split at h
      · simp at h
      · split at h
        · simp at h
        · rename_i h1 h2 h3; simp_all

/-- STM-019: the early wrap starts only with the STARK extra, the wrap tail and the margin free. -/
theorem earlyWrap_guard (s : St) (h : stepRefusals s (10 * 16) = []) :
    s.starkExtra + wrapTail s + s.margin ≤ s.wrapFree := by
  have hd : decodeOp (10 * 16) = .earlyWrap := rfl
  unfold stepRefusals at h
  rw [hd] at h
  split at h
  · simp at h
  · simp only [] at h
    split at h
    · simp at h
    · split at h
      · simp at h
      · omega

/-- ELA-006: a leaf that the stock verifier refuses stops the join before N1. -/
theorem join_refuses_bad_leaf (s : St) (hv : validState s = true) :
    (stepRefusals s (9 * 16 + 1)).map Refusal.code = ["leaf-not-verified"] := by
  simp [stepRefusals, hv, decodeOp, Refusal.code]

/-! ## The state domain is an invariant -/

theorem apply_valid (s : St) (op : Nat) (h : Valid s) : Valid (apply s op) := by
  unfold apply
  split
  · rename_i hr
    have hv : validState s = true := by simp [validState, h]
    simp only [stepRefusals, hv, Bool.true_eq_false, ite_false] at hr
    obtain ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩ := h
    unfold step
    cases hop : decodeOp op <;> simp only [hop] at hr ⊢
    case setup r =>
      split
      · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
      · refine ⟨h1, h2, h3, h4, ?_, h6, ?_, fun _ => rfl⟩ <;> simp only [setupOk] <;> split <;> omega
    case startRun =>
      split
      · exact ⟨h1, h2, h3, h4, h5, h6, h7, fun _ => by
          simp only [schedule]; split at hr <;> simp_all⟩
      · exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    case memLimit a => exact ⟨h1, h2, h3, h4, h5, h6, h7, fun hc => by simp at hc⟩
    case gpus g => exact ⟨h1, h2, h3, h4, h5, h6, h7, fun hc => by simp at hc⟩
    case gpuMemLimit a => exact ⟨h1, h2, h3, h4, h5, h6, h7, fun hc => by simp at hc⟩
    case forceCut k => exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    case prove => exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    case newRun => exact ⟨h1, h2, h3, h4, h5, h6, h7, fun hc => by simp at hc⟩
    case moveTo b =>
      have hb : b < boxes := decodeOp_moveTo op b hop
      exact ⟨h1, h2, h3, h4, h5, hb, h7, fun hc => by simp at hc⟩
    case join v => exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    case earlyWrap => exact ⟨h1, h2, h3, h4, h5, h6, h7, h8⟩
    case bad => simp at hr
  · exact h

theorem run_valid (s : St) (ops : List Nat) (h : Valid s) : Valid (run s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons op ops ih => exact ih _ (apply_valid s op h)

end Scheduling
