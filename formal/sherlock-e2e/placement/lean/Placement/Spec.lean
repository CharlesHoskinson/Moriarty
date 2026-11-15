/-!
# Placement oracle (bounded context Placement, slice C)

Executable reference semantics of `sherlock-e2e-helper-machines` (HLP-001 to HLP-008)
and of the adopted placement rules ZK-001, ZK-009 (processor-final as fixed on
spec-audit), PRD-002 and PRD-003 (processor-final). Written from the EARS sentences.

One proving job of one call on a box goes through these phases:

| phase | meaning |
|---|---|
| 0 | idle: no job |
| 1 | dispatched to an owned box (case A) with the requested call vector `jobCv` |
| 2 | result received on muninn (call vector `resCv`, sha match `shaOk`) |
| 3 | kept: bound to its job and verified by the ledger 9.1.0.1 check |
| 4 | dropped (`dropCode` says why) |
| 5 | submitted (one call carries the proof) |

The box returns the 4 public words and the wrap output only (HLP-003, HLP-004): the
`receive` operation has no field for a private part, so a received result carries
`resWords = 4` and `resPrivate = 0` by construction.

LEAD PICKs: LP-1 a drop of HLP-008 (binding) and of HLP-002 (verifier) both end the
job; `submit` in phase 4 records the refusal and submits nothing. LP-2 the off-box
send of a private artefact to an owned host is allowed (case A, OD-9) and counted.
-/

namespace Placement

def opBase : Int := 1000000000

/-! ## Codes recorded in `lastRefusal` and `dropCode` -/

def rNotOwned : Int := 1        -- HLP-005, ZK-001, ZK-018, PRD-003 (a host the user does not own)
def dBinding : Int := 3         -- HLP-008 (sha or call vector differs)
def dVerifier : Int := 4        -- HLP-001, HLP-002 (the ledger check refused the proof)
def rDropped : Int := 5         -- HLP-002 (no call carries a dropped proof)
def rNoWallet : Int := 6        -- HLP-007
def rHelperSlot : Int := 7      -- PRD-002, ZK-009 (a helper-made slot)
def rNoSlot : Int := 8          -- PRD-002 (each wrap_prove consumes one fresh slot)
def rSlotExport : Int := 9      -- PRD-002 (a slot is never serialized off the machine)

structure St where
  phase : Int
  jobCv : Int
  boxOwned : Bool
  resCv : Int
  shaOk : Bool
  resWords : Int
  resPrivate : Int
  verifyRan : Bool
  verifyOk : Bool
  dropCode : Int
  kept : Int
  submitted : Int
  lastRefusal : Int
  signed : Int
  signedNoWallet : Int
  slotsFree : Int
  slotsUsed : Int
  helperSlots : Int
  wrapProofs : Int
  sentOwned : Int
  sentNotOwned : Int
  deriving Repr, DecidableEq

def init : St :=
  { phase := 0, jobCv := 0, boxOwned := false, resCv := 0, shaOk := false, resWords := 0,
    resPrivate := 0, verifyRan := false, verifyOk := false, dropCode := 0, kept := 0,
    submitted := 0, lastRefusal := 0, signed := 0, signedNoWallet := 0, slotsFree := 0,
    slotsUsed := 0, helperSlots := 0, wrapProofs := 0, sentOwned := 0, sentNotOwned := 0 }

inductive Op where
  | dispatch (owned : Bool) (cv : Int)     -- 1: arg = owned + 10 * cv
  | receive (cv : Int) (shaOk : Bool)      -- 2: arg = shaOk + 10 * cv
  | check (verifierOk : Bool)              -- 3: arg 0 | 1
  | submit                                 -- 4
  | newJob                                 -- 5
  | sign (hasWallet : Bool)                -- 6: arg 0 | 1
  | makeSlot (byHelper : Bool)             -- 7: arg 0 | 1
  | wrapProve                              -- 8
  | exportSlot                             -- 9
  | sendOffBox (kind : Int) (owned : Bool) -- 10: arg = owned + 10 * kind, kind 1..4
  deriving Repr, DecidableEq

def bit (a : Int) : Option Bool := if a = 0 then some false else if a = 1 then some true else none

def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then (bit (a % 10)).map (fun b => .dispatch b (a / 10))
  else if c = 2 then (bit (a % 10)).map (fun b => .receive (a / 10) b)
  else if c = 3 then (bit a).map .check
  else if c = 4 then (if a = 0 then some .submit else none)
  else if c = 5 then (if a = 0 then some .newJob else none)
  else if c = 6 then (bit a).map .sign
  else if c = 7 then (bit a).map .makeSlot
  else if c = 8 then (if a = 0 then some .wrapProve else none)
  else if c = 9 then (if a = 0 then some .exportSlot else none)
  else if c = 10 then
    (if 1 ≤ a / 10 ∧ a / 10 ≤ 4 then (bit (a % 10)).map (fun b => .sendOffBox (a / 10) b) else none)
  else none

def enabled (s : St) : Op → Bool
  | .dispatch _ _ => s.phase = 0
  | .receive _ _ => s.phase = 1
  | .check _ => s.phase = 2
  | .submit => s.phase = 3 ∨ s.phase = 4
  | .newJob => s.phase = 3 ∨ s.phase = 4 ∨ s.phase = 5
  | _ => true

def apply (s : St) (op : Op) : St :=
  if !enabled s op then s else
  match op with
  | .dispatch owned cv =>
    -- HLP-005, ZK-001, ZK-018, PRD-003: case A only
    if owned then { s with phase := 1, jobCv := cv, boxOwned := true, lastRefusal := 0 }
    else { s with lastRefusal := rNotOwned }
  | .receive cv ok =>
    -- HLP-003, HLP-004: the box returns the 4 public words and the wrap output only
    { s with phase := 2, resCv := cv, shaOk := ok, resWords := 4, resPrivate := 0 }
  | .check vok =>
    -- HLP-008: the binding check comes before the proof is verified
    if s.shaOk = false ∨ s.resCv ≠ s.jobCv then { s with phase := 4, dropCode := dBinding }
    -- HLP-001: the ledger 9.1.0.1 check verifies the proof before the result is kept
    else if vok then { s with phase := 3, verifyRan := true, verifyOk := true, kept := s.kept + 1 }
    -- HLP-002: a refused proof is dropped
    else { s with phase := 4, verifyRan := true, verifyOk := false, dropCode := dVerifier }
  | .submit =>
    -- HLP-002 (LP-1): no call carries a dropped proof
    if s.phase = 4 then { s with lastRefusal := rDropped }
    else { s with phase := 5, submitted := s.submitted + 1, lastRefusal := 0 }
  | .newJob =>
    { s with phase := 0, jobCv := 0, boxOwned := false, resCv := 0, shaOk := false, resWords := 0,
             resPrivate := 0, verifyRan := false, verifyOk := false, dropCode := 0 }
  | .sign w =>
    -- HLP-007: a box with no wallet directory refuses to sign or submit
    if w then { s with signed := s.signed + 1, lastRefusal := 0 } else { s with lastRefusal := rNoWallet }
  | .makeSlot helper =>
    -- PRD-002, ZK-009: a helper-made slot is refused
    if helper then { s with lastRefusal := rHelperSlot }
    else { s with slotsFree := s.slotsFree + 1, lastRefusal := 0 }
  | .wrapProve =>
    -- PRD-002: each slot is consumed by exactly one wrap_prove; ZK-009: no slot reuse
    if s.slotsFree = 0 then { s with lastRefusal := rNoSlot }
    else { s with slotsFree := s.slotsFree - 1, slotsUsed := s.slotsUsed + 1,
                  wrapProofs := s.wrapProofs + 1, lastRefusal := 0 }
  | .exportSlot =>
    -- PRD-002: a slot is not serialized off the machine
    { s with lastRefusal := rSlotExport }
  | .sendOffBox _ owned =>
    -- HLP-005, PRD-003, ZK-001: no witness, STARK proof, checkpoint or shard plan to a
    -- host the user does not own (LP-2: an owned host is the user's machine)
    if owned then { s with sentOwned := s.sentOwned + 1, lastRefusal := 0 }
    else { s with lastRefusal := rNotOwned }

inductive Refusal where
  | notAnOperation
  | opNotEnabled
  | outsideStateDomain
  deriving Repr, DecidableEq

def Refusal.code : Refusal → String
  | .notAnOperation => "not-an-operation"
  | .opNotEnabled => "op-not-enabled"
  | .outsideStateDomain => "outside-state-domain"

def validState (s : St) : Bool :=
  0 ≤ s.phase && s.phase ≤ 5 && 0 ≤ s.jobCv && 0 ≤ s.resCv && 0 ≤ s.resWords && 0 ≤ s.resPrivate
    && 0 ≤ s.dropCode && s.dropCode ≤ 4 && 0 ≤ s.kept && 0 ≤ s.submitted && 0 ≤ s.lastRefusal
    && s.lastRefusal ≤ 9 && 0 ≤ s.signed && 0 ≤ s.signedNoWallet && 0 ≤ s.slotsFree
    && 0 ≤ s.slotsUsed && 0 ≤ s.helperSlots && 0 ≤ s.wrapProofs && 0 ≤ s.sentOwned
    && 0 ≤ s.sentNotOwned

def stepRefusals (s : St) (n : Int) : List Refusal :=
  match Op.ofInt n with
  | none => [.notAnOperation] ++ (if validState s then [] else [.outsideStateDomain])
  | some op => (if enabled s op then [] else [.opNotEnabled])
      ++ (if validState s then [] else [.outsideStateDomain])

def stepOp (s : St) (n : Int) : St :=
  match Op.ofInt n with
  | none => s
  | some op => apply s op

def runOps (s : St) : List Int → St
  | [] => s
  | n :: ns => runOps (stepOp s n) ns

def refusals (ops : List Int) : List Refusal :=
  if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation]

def run (ops : List Int) : St := runOps init ops

/-! ## Invariants -/

structure Inv (s : St) : Prop where
  /-- HLP-005, ZK-001, ZK-018, PRD-003: a job exists only on an owned box (case A). -/
  owned : s.phase ≠ 0 → s.boxOwned = true
  /-- HLP-001, HLP-008: a kept or submitted result is bound to its job and verified. -/
  keptChecked : (s.phase = 3 ∨ s.phase = 5) →
    s.shaOk = true ∧ s.resCv = s.jobCv ∧ s.verifyRan = true ∧ s.verifyOk = true
  /-- HLP-008: a binding drop happens before the proof is verified. -/
  bindingBeforeVerify : s.phase = 4 → s.dropCode = dBinding → s.verifyRan = false
  /-- HLP-002: a verifier drop is a refused proof. -/
  verifierDrop : s.phase = 4 → s.dropCode = dVerifier → s.verifyRan = true ∧ s.verifyOk = false
  dropCodes : s.phase = 4 → s.dropCode = dBinding ∨ s.dropCode = dVerifier
  /-- HLP-002, ZK-009: every submitted call carries a kept proof, once. -/
  submittedKept : s.submitted ≤ s.kept
  phase5 : s.phase = 5 → 1 ≤ s.submitted
  phase3 : s.phase = 3 → s.submitted < s.kept
  /-- HLP-003, HLP-004: a received result has 4 public words and no private part. -/
  result : 2 ≤ s.phase → s.resWords = 4 ∧ s.resPrivate = 0
  /-- HLP-007: no box without a wallet signs. -/
  noWalletSign : s.signedNoWallet = 0
  /-- PRD-002, ZK-009: one slot for one wrap_prove, no helper slot. -/
  slots : s.slotsUsed = s.wrapProofs ∧ 0 ≤ s.slotsFree ∧ s.helperSlots = 0
  /-- HLP-005, PRD-003: nothing private is sent to a host the user does not own. -/
  noHelper : s.sentNotOwned = 0
  phaseRange : 0 ≤ s.phase ∧ s.phase ≤ 5
  preVerify : s.phase ≤ 2 → s.verifyRan = false
  counts : 0 ≤ s.kept ∧ 0 ≤ s.submitted

theorem inv_init : Inv init := by
  constructor <;> simp [init, dBinding, dVerifier]

macro "ptac" : tactic => `(tactic| (
  cases ‹Inv _›
  constructor <;> simp_all [dBinding, dVerifier] <;> omega))

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases hen : enabled s op
  · simp [apply, hen]; exact h
  cases op with
  | dispatch o cv => simp [enabled] at hen; cases o <;> simp [apply, enabled, hen] <;> ptac
  | receive cv b => simp [apply, hen]; simp [enabled] at hen; ptac
  | check v =>
    simp [enabled] at hen
    by_cases hb : s.shaOk = false ∨ s.resCv ≠ s.jobCv
    · simp [apply, enabled, hen, hb]; ptac
    · cases v <;> simp [apply, enabled, hen, hb] <;> ptac
  | submit =>
    simp [enabled] at hen
    by_cases h4 : s.phase = 4
    · simp [apply, enabled, h4]; ptac
    · have h3 : s.phase = 3 := by omega
      simp [apply, enabled, h3]; ptac
  | newJob => simp [enabled] at hen; simp [apply, enabled, hen]; ptac
  | sign w => cases w <;> simp [apply, enabled] <;> ptac
  | makeSlot b => cases b <;> simp [apply, enabled] <;> ptac
  | wrapProve => by_cases hz : s.slotsFree = 0 <;> simp [apply, enabled, hz] <;> ptac
  | exportSlot => simp [apply, enabled]; ptac
  | sendOffBox k o => cases o <;> simp [apply, enabled] <;> ptac

theorem inv_stepOp (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp; split
  · exact h
  · exact inv_apply s _ h

theorem inv_runOps (s : St) (ns : List Int) (h : Inv s) : Inv (runOps s ns) := by
  induction ns generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_stepOp s n h)

theorem inv_run (ns : List Int) : Inv (run ns) := inv_runOps _ _ inv_init

/-! ## Requirement-level corollaries over every run -/

theorem HLP_001_kept_after_ledger_check (ns : List Int) :
    (run ns).phase = 3 → (run ns).verifyRan = true ∧ (run ns).verifyOk = true :=
  fun h => let ⟨_, _, a, b⟩ := (inv_run ns).keptChecked (Or.inl h); ⟨a, b⟩

theorem HLP_002_no_submit_of_refused (ns : List Int) : (run ns).submitted ≤ (run ns).kept :=
  (inv_run ns).submittedKept

theorem HLP_003_four_public_words (ns : List Int) :
    2 ≤ (run ns).phase → (run ns).resWords = 4 := fun h => ((inv_run ns).result h).1

theorem HLP_004_no_private_part (ns : List Int) :
    2 ≤ (run ns).phase → (run ns).resPrivate = 0 := fun h => ((inv_run ns).result h).2

theorem HLP_005_case_A_only (ns : List Int) :
    (run ns).sentNotOwned = 0 ∧ ((run ns).phase ≠ 0 → (run ns).boxOwned = true) :=
  ⟨(inv_run ns).noHelper, (inv_run ns).owned⟩

theorem HLP_007_no_sign_without_wallet (ns : List Int) : (run ns).signedNoWallet = 0 :=
  (inv_run ns).noWalletSign

theorem HLP_008_binding_before_verify (ns : List Int) :
    (run ns).phase = 4 → (run ns).dropCode = 3 → (run ns).verifyRan = false :=
  (inv_run ns).bindingBeforeVerify

theorem PRD_002_one_slot_per_wrap (ns : List Int) :
    (run ns).slotsUsed = (run ns).wrapProofs ∧ (run ns).helperSlots = 0 :=
  ⟨(inv_run ns).slots.1, (inv_run ns).slots.2.2⟩

end Placement
