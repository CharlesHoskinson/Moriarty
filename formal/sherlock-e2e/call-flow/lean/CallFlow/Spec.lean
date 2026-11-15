/-!
# Call Flow oracle (bounded context Call Flow, slice A)

Executable reference semantics of `sherlock-e2e-user-flow` (EUF-001, EUF-002, EUF-005 to
EUF-014; EUF-003 and EUF-004 are withdrawn) and of the Mycroft requirements it cites
(MYC-KEY-002, MYC-SEAL-015). Call Flow owns no state of its own in the product: the cells
below are the facts each stage publishes to the next. Spec text only.

Stages (EUF-001): 1 compile, 2 setup, 3 plan, 4 deploy, 5 tape, 6 execute, 7 prove,
8 wrap, 9 ledger check, 10 submit, 11 read-back. `stage` is the last stage done.
A second call of a deployed program starts after stage 4 (EUF-002).
Pins (EUF-012), bit i of a pin mask, named by `failedPin = i + 1`: 0 compactc, 1 ZKIR,
2 machine version, 3 universal key-set identity, 4 ledger version.
-/

namespace CallFlow

def opBase : Int := 1000000000

def sCompile : Int := 1
def sSetup : Int := 2
def sDeploy : Int := 4
def sExecute : Int := 6
def sProve : Int := 7
def sCheck : Int := 9
def sSubmit : Int := 10
def sReadBack : Int := 11

/-- EUF-012: the key-set pin. -/
def pinKeyset : Int := 4

/-- The lowest set bit of a pin mask, as a pin number 1..5 (0 for an empty mask). -/
def lowestPin (m : Int) : Int :=
  if m % 2 = 1 then 1 else if (m / 2) % 2 = 1 then 2 else if (m / 4) % 2 = 1 then 3
  else if (m / 8) % 2 = 1 then 4 else if (m / 16) % 2 = 1 then 5 else 0

theorem lowestPin_range (m : Int) : 0 ≤ lowestPin m ∧ lowestPin m ≤ 5 := by
  unfold lowestPin; repeat' split
  all_goals decide

structure St where
  stage : Int
  stopped : Bool
  failedStage : Int
  failedPin : Int
  nativeRejected : Bool
  universalVk : Int
  programs : Int
  deploys : Int
  lastDeployVk : Int
  vkRefused : Int
  installedKeys : Int
  profileKeys : Int
  provedOk : Bool
  claim : Int
  claimA : Int
  claimB : Int
  txs : Int
  txTotal : Int
  unknown : Bool
  resendRefused : Int
  outcome : Int
  eligible : Int
  joined : Int
  firstKeys : Int
  mixedKeys : Bool
  complete : Bool
  completeBlocked : Int
  deriving Repr, DecidableEq

/-- `vk` is the one universal wrap key, made with no program as input (MYC-SEAL-015). The
installed key set is 1; no box has a profile yet. The first call has identity 1. -/
def init (vk : Int) : St :=
  { stage := 0, stopped := false, failedStage := 0, failedPin := 0, nativeRejected := false,
    universalVk := vk, programs := 0, deploys := 0, lastDeployVk := 0, vkRefused := 0,
    installedKeys := 1, profileKeys := 0, provedOk := false, claim := 1, claimA := 0, claimB := 0,
    txs := 0, txTotal := 0, unknown := false, resendRefused := 0, outcome := 0, eligible := 0,
    joined := 0, firstKeys := 0, mixedKeys := false, complete := false, completeBlocked := 0 }

inductive Op where
  | step (n : Int) (ok : Bool)    -- 1: run stage n (n = stage + 1) with its result
  | setupBox (keys : Int)         -- 2: a box writes a profile for key set `keys`
  | rekey (keys : Int)            -- 3: a new universal key set is installed
  | newCall (id : Int)            -- 4: the next call (identity = its claim)
  | checkPins (mask : Int)        -- 5: compare the artifact pins with the run pins
  | proveOnBox (box : Int)        -- 6: box 1 or 2 proves the call
  | deploySecond (vk : Int)       -- 7: deploy a further program with key `vk`
  | lostResponse                  -- 8: the transport fails after the node took the tx
  | reconcile (r : Int)           -- 9: by tx hash: 0 not found, 1 equal, 2 differs
  | reportComplete                -- 10: report the prototype complete
  deriving Repr, DecidableEq

/-- Decode `code * 10^9 + arg`. step: arg = n + 100 ok. -/
def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then
    (if 1 ≤ a % 100 ∧ a % 100 ≤ 11 ∧ a / 100 ≤ 1 then some (.step (a % 100) (a / 100 = 1)) else none)
  else if c = 2 then (if 1 ≤ a then some (.setupBox a) else none)
  else if c = 3 then (if 1 ≤ a then some (.rekey a) else none)
  else if c = 4 then (if 1 ≤ a then some (.newCall a) else none)
  else if c = 5 then (if a < 32 then some (.checkPins a) else none)
  else if c = 6 then (if a = 1 ∨ a = 2 then some (.proveOnBox a) else none)
  else if c = 7 then (if 1 ≤ a then some (.deploySecond a) else none)
  else if c = 8 then (if a = 0 then some .lostResponse else none)
  else if c = 9 then (if a ≤ 2 then some (.reconcile a) else none)
  else if c = 10 then (if a = 0 then some .reportComplete else none)
  else none

def enabled (s : St) : Op → Bool
  | .step n _ => !s.stopped ∧ n = s.stage + 1 ∧ s.stage ≤ 10 ∧ !s.unknown
  | .setupBox k => 1 ≤ k
  | .rekey k => 1 ≤ k
  | .newCall id => (s.stage = sReadBack ∨ s.stopped) ∧ 1 ≤ id
  | .checkPins m => !s.stopped ∧ s.stage < sSubmit ∧ 0 ≤ m
  | .proveOnBox b => !s.stopped ∧ sProve ≤ s.stage ∧ (b = 1 ∨ b = 2)
  | .deploySecond _ => 1 ≤ s.programs
  | .lostResponse => !s.stopped ∧ s.stage = sSubmit ∧ !s.unknown
  | .reconcile r => s.stage = sSubmit ∧ s.unknown ∧ 0 ≤ r ∧ r ≤ 2
  | .reportComplete => true

def stopAt (s : St) (n : Int) : St := { s with stopped := true, failedStage := n }

/-- EUF-013: a read-back with an equal outcome joins the call; the key set of every
joined call is recorded. -/
def join (s : St) : St :=
  { s with joined := s.joined + 1, complete := false,
           firstKeys := if s.joined = 0 then s.installedKeys else s.firstKeys,
           mixedKeys := s.mixedKeys || (s.joined ≠ 0 && s.installedKeys ≠ s.firstKeys) }

/-! One helper per stage outcome, so each proof goal stays small. -/

/-- EUF-006: a native refusal at execute starts no prove and no submit. -/
def rejectAt (s : St) : St := { stopAt s sExecute with nativeRejected := true }
/-- EUF-007, EUF-012: no proof on a box whose profile is not for the installed key set. -/
def profileStop (s : St) : St := { stopAt s sProve with failedPin := pinKeyset }
def proveAt (s : St) : St := { s with stage := sProve, provedOk := true }
/-- EUF-007 scenario: setup writes a profile of the installed key set. -/
def setupAt (s : St) : St := { s with stage := sSetup, profileKeys := s.installedKeys }
/-- EUF-009, MYC-KEY-002, MYC-SEAL-015: the deployment stores the one universal key. -/
def deployAt (s : St) : St :=
  { s with stage := sDeploy, programs := s.programs + 1, deploys := s.deploys + 1,
           lastDeployVk := s.universalVk }
/-- EUF-013: a call that passed execute is eligible. -/
def executeAt (s : St) : St :=
  { s with stage := sExecute, eligible := s.eligible + 1, complete := false }
/-- EUF-014: one transaction for one claim_pub; `ok = false` is a lost response. -/
def submitAt (s : St) (ok : Bool) : St :=
  { s with stage := sSubmit, txs := s.txs + 1, txTotal := s.txTotal + 1, unknown := !ok }
/-- EUF-010: the read-back compares runs[claim_pub] and the native outcome. -/
def readBackAt (s : St) (ok : Bool) : St :=
  if ok then join { s with stage := sReadBack, outcome := 1 } else { s with stage := sReadBack, outcome := 2 }
def plainAt (s : St) (n : Int) : St := { s with stage := n }

/-- EUF-001: stage `n` with its result. -/
def stageStep (s : St) (n : Int) (ok : Bool) : St :=
  if n = sExecute ∧ ok = false then rejectAt s
  -- EUF-005: a failed stage before submit sends no transaction and records the stage
  else if n ≤ sCheck ∧ ok = false then stopAt s n
  else if n = sProve ∧ s.profileKeys ≠ s.installedKeys then profileStop s
  else if n = sProve then proveAt s
  else if n = sSetup then setupAt s
  else if n = sDeploy then deployAt s
  else if n = sExecute then executeAt s
  else if n = sSubmit then submitAt s ok
  else if n = sReadBack then readBackAt s ok
  else plainAt s n

/-- The response of an enabled operation. -/
def applyCore (s : St) : Op → St
  | .step n ok => stageStep s n ok
  | .setupBox k => { s with profileKeys := k }
  | .rekey k => { s with installedKeys := k }
  | .newCall id =>
    -- EUF-014: no second transaction for the same claim_pub
    if id = s.claim ∧ 0 < s.txs then { s with resendRefused := s.resendRefused + 1 }
    -- EUF-002: a second call of a deployed program starts at the tape stage
    else { s with stage := if 1 ≤ s.programs then sDeploy else 0, stopped := false,
                  failedStage := 0, failedPin := 0, nativeRejected := false, provedOk := false,
                  claim := id, claimA := 0, claimB := 0, txs := 0, unknown := false, outcome := 0 }
  | .checkPins m =>
    -- EUF-012: a differing pin stops the call and names the pin
    if m = 0 then s else { stopAt s s.stage with failedPin := lowestPin m }
  | .proveOnBox b =>
    -- EUF-008: every owned box gives the claim_pub of the call
    if b = 1 then { s with claimA := s.claim } else { s with claimB := s.claim }
  | .deploySecond vk =>
    -- EUF-009: a second program uses the first program's wrap key, byte for byte
    if vk = s.universalVk then
      { s with programs := s.programs + 1, deploys := s.deploys + 1, lastDeployVk := vk }
    else { s with vkRefused := s.vkRefused + 1 }
  | .lostResponse => { s with unknown := true }
  | .reconcile r =>
    -- EUF-014: reconciled by tx hash; the receipt completes from the chain, no new tx
    if r = 0 then s
    else if r = 1 then join { s with stage := sReadBack, unknown := false, outcome := 1 }
    else { s with stage := sReadBack, unknown := false, outcome := 2 }
  | .reportComplete =>
    -- EUF-013: complete only with every eligible call joined, under one key set
    if 1 ≤ s.eligible ∧ s.joined = s.eligible ∧ s.mixedKeys = false then { s with complete := true }
    else { s with completeBlocked := s.completeBlocked + 1, complete := false }

def apply (s : St) (op : Op) : St := if !enabled s op then s else applyCore s op

theorem apply_en (s : St) (op : Op) (hen : enabled s op = true) : apply s op = applyCore s op := by
  simp [apply, hen]

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
  0 ≤ s.stage && s.stage ≤ 11 && 0 ≤ s.failedStage && s.failedStage ≤ 11 && 0 ≤ s.failedPin
    && s.failedPin ≤ 5 && 1 ≤ s.universalVk && 0 ≤ s.programs && 0 ≤ s.txs && 0 ≤ s.outcome
    && s.outcome ≤ 2 && 0 ≤ s.eligible && 0 ≤ s.joined

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

def refusals (vk : Int) (ops : List Int) : List Refusal :=
  (if 1 ≤ vk then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (vk : Int) (ops : List Int) : St := runOps (init vk) ops

/-! ## Invariants -/

structure Inv (s : St) : Prop where
  stageRange : 0 ≤ s.stage ∧ s.stage ≤ 11
  /-- EUF-005, EUF-012: a stopped call stopped before submit and sent no transaction. -/
  stoppedNoTx : s.stopped = true → s.txs = 0 ∧ s.stage < sSubmit
  /-- EUF-005: no transaction before the submit stage. -/
  txAfterSubmit : 0 < s.txs → sSubmit ≤ s.stage
  /-- EUF-014: at most one transaction for the claim of the call. -/
  oneTx : 0 ≤ s.txs ∧ s.txs ≤ 1 ∧ (sSubmit ≤ s.stage → s.txs = 1)
  /-- EUF-006: a native refusal stops at execute, before prove and submit. -/
  rejected : s.nativeRejected = true → s.stopped = true ∧ s.failedStage = sExecute ∧ s.stage < sProve
  /-- EUF-007: a call past the prove stage was proved on a box with a profile of the
  installed key set. -/
  proved : sProve ≤ s.stage → s.provedOk = true
  /-- EUF-009, MYC-KEY-002, MYC-SEAL-015: every deployment holds the one universal key, and
  each program is deployed once (EUF-002: a second call sends no deployment). -/
  oneKey : s.deploys = s.programs ∧ 0 ≤ s.programs ∧ (1 ≤ s.programs → s.lastDeployVk = s.universalVk)
  /-- EUF-002: the program is deployed before any per-call stage runs. -/
  deployedFirst : sDeploy ≤ s.stage → 1 ≤ s.programs
  /-- EUF-008: two boxes that prove one call give byte-equal claim_pub values. -/
  oneClaim : (s.claimA = 0 ∨ s.claimA = s.claim) ∧ (s.claimB = 0 ∨ s.claimB = s.claim)
  /-- EUF-010: an outcome exists only after the read-back. -/
  outcomeOk : 0 ≤ s.outcome ∧ s.outcome ≤ 2 ∧ (s.outcome ≠ 0 → s.stage = sReadBack)
  /-- EUF-012: a named pin always belongs to a stopped call. -/
  pinNamed : 0 ≤ s.failedPin ∧ s.failedPin ≤ 5 ∧ (s.failedPin ≠ 0 → s.stopped = true)
  /-- EUF-013: a complete report means every eligible call joined, under one key set. -/
  completeOk : s.complete = true → s.joined = s.eligible ∧ s.mixedKeys = false
  /-- An unknown submission status exists only at the submit stage. -/
  unknownAtSubmit : s.unknown = true → s.stage = sSubmit ∧ s.stopped = false
  nonneg : 0 ≤ s.joined ∧ 0 ≤ s.eligible ∧ 0 ≤ s.txTotal

theorem inv_init (vk : Int) : Inv (init vk) := by
  constructor <;> simp [init, sSubmit, sProve, sDeploy, sExecute, sReadBack]

set_option maxHeartbeats 2000000
set_option linter.unusedSimpArgs false

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  cases h
  constructor <;> simp_all [stopAt, join, sCompile, sSetup, sDeploy, sExecute, sProve, sCheck,
    sSubmit, sReadBack, pinKeyset] <;> omega))

theorem inv_stepAt (n : Int) (ok : Bool)
    (he : s.stopped = false ∧ n = s.stage + 1 ∧ s.stage ≤ 10 ∧ s.unknown = false) :
    Inv (stageStep s n ok) := by
  obtain ⟨h1, h2, h3, h4⟩ := he
  have ht : s.txs = 0 ∨ 10 ≤ s.stage := by
    have := h.txAfterSubmit; have := h.oneTx; simp only [sSubmit] at *; omega
  unfold stageStep
  by_cases c1 : n = sExecute ∧ ok = false
  · rw [ite_eq_left c1]; obtain ⟨c1a, _⟩ := c1; subst c1a; simp only [rejectAt]; inv_tac
  rw [ite_eq_right c1]
  by_cases c2 : n ≤ sCheck ∧ ok = false
  · rw [ite_eq_left c2]; obtain ⟨c2a, _⟩ := c2; simp only [sCheck] at c2a; simp only [stopAt]; inv_tac
  rw [ite_eq_right c2]
  by_cases c3 : n = sProve ∧ s.profileKeys ≠ s.installedKeys
  · rw [ite_eq_left c3]; obtain ⟨c3a, _⟩ := c3; subst c3a; simp only [profileStop]; inv_tac
  rw [ite_eq_right c3]
  by_cases c4 : n = sProve
  · rw [ite_eq_left c4]; subst c4; simp only [proveAt]; inv_tac
  rw [ite_eq_right c4]
  by_cases c5 : n = sSetup
  · rw [ite_eq_left c5]; subst c5; simp only [setupAt]; inv_tac
  rw [ite_eq_right c5]
  by_cases c6 : n = sDeploy
  · rw [ite_eq_left c6]; subst c6; simp only [deployAt]; inv_tac
  rw [ite_eq_right c6]
  by_cases c7 : n = sExecute
  · rw [ite_eq_left c7]; subst c7; simp only [executeAt]; inv_tac
  rw [ite_eq_right c7]
  by_cases c8 : n = sSubmit
  · rw [ite_eq_left c8]
    have hs : s.stage = 9 := by simp only [sSubmit] at c8; omega
    clear h2 c8 c1 c2 c3 c4 c5 c6 c7; simp only [submitAt]; inv_tac
  rw [ite_eq_right c8]
  by_cases c9 : n = sReadBack
  · rw [ite_eq_left c9]
    have hs : s.stage = 10 := by simp only [sReadBack] at c9; omega
    clear h2 c9 c1 c2 c3 c4 c5 c6 c7 c8; simp only [readBackAt]
    cases ok
    · simp only [Bool.false_eq_true, ite_false]; inv_tac
    · simp only [ite_true]; inv_tac
  rw [ite_eq_right c9]; simp only [plainAt]
  have hs := h.stageRange
  have hn : n = 1 ∨ n = 3 ∨ n = 5 ∨ n = 8 ∨ n = 9 := by
    simp only [sExecute, sCheck, sProve, sSetup, sDeploy, sSubmit, sReadBack] at *; omega
  have hp := h.proved
  rcases hn with rfl | rfl | rfl | rfl | rfl
  · have hs : s.stage = 0 := by omega
    clear h2 c1 c2 c3 c4 c5 c6 c7 c8 c9; inv_tac
  · have hs : s.stage = 2 := by omega
    clear h2 c1 c2 c3 c4 c5 c6 c7 c8 c9; inv_tac
  · have hs : s.stage = 4 := by omega
    clear h2 c1 c2 c3 c4 c5 c6 c7 c8 c9; inv_tac
  · have hs : s.stage = 7 := by omega
    have : s.provedOk = true := hp (by simp only [sProve]; omega)
    clear h2 c1 c2 c3 c4 c5 c6 c7 c8 c9; inv_tac
  · have hs : s.stage = 8 := by omega
    have : s.provedOk = true := hp (by simp only [sProve]; omega)
    clear h2 c1 c2 c3 c4 c5 c6 c7 c8 c9; inv_tac

theorem inv_step (n : Int) (ok : Bool)
    (he : s.stopped = false ∧ n = s.stage + 1 ∧ s.stage ≤ 10 ∧ s.unknown = false) :
    Inv (apply s (.step n ok)) := by
  rw [apply_en s _ (by simp [enabled, he])]; exact inv_stepAt s h n ok he
theorem inv_setupBox (k : Int) (hk : 1 ≤ k) : Inv (apply s (.setupBox k)) := by
  rw [apply_en s _ (by simp [enabled, hk])]; simp only [applyCore]; inv_tac
theorem inv_rekey (k : Int) (hk : 1 ≤ k) : Inv (apply s (.rekey k)) := by
  rw [apply_en s _ (by simp [enabled, hk])]; simp only [applyCore]; inv_tac
theorem inv_newCall (id : Int) (he : (s.stage = sReadBack ∨ s.stopped = true) ∧ 1 ≤ id) :
    Inv (apply s (.newCall id)) := by
  rw [apply_en s _ (by simp [enabled, he])]; simp only [applyCore]
  by_cases c1 : id = s.claim ∧ 0 < s.txs
  · rw [ite_eq_left c1]; inv_tac
  · rw [ite_eq_right c1]
    by_cases c2 : 1 ≤ s.programs
    · simp only [c2, ite_true]; inv_tac
    · simp only [c2, ite_false]; inv_tac
theorem inv_checkPins (m : Int) (he : s.stopped = false ∧ s.stage < sSubmit ∧ 0 ≤ m) :
    Inv (apply s (.checkPins m)) := by
  rw [apply_en s _ (by simp [enabled, he])]; simp only [applyCore]
  have hr := lowestPin_range m
  have ht : s.txs = 0 := by have := h.txAfterSubmit; have := h.oneTx; simp only [sSubmit] at *; omega
  have hu : s.unknown = false := by
    cases hc : s.unknown
    · rfl
    · have := (h.unknownAtSubmit hc).1; simp only [sSubmit] at *; omega
  have hn : s.nativeRejected = false := by
    cases hc : s.nativeRejected
    · rfl
    · have := (h.rejected hc).1; simp_all
  by_cases c : m = 0
  · rw [ite_eq_left c]; exact h
  · rw [ite_eq_right c]; simp only [stopAt]; inv_tac
theorem inv_prove (b : Int) (he : s.stopped = false ∧ sProve ≤ s.stage ∧ (b = 1 ∨ b = 2)) :
    Inv (apply s (.proveOnBox b)) := by
  rw [apply_en s _ (by simp [enabled, he])]; simp only [applyCore]
  by_cases c : b = 1
  · rw [ite_eq_left c]; inv_tac
  · rw [ite_eq_right c]; inv_tac
theorem inv_deploySecond (vk : Int) (he : 1 ≤ s.programs) : Inv (apply s (.deploySecond vk)) := by
  rw [apply_en s _ (by simp [enabled, he])]; simp only [applyCore]
  by_cases c : vk = s.universalVk
  · rw [ite_eq_left c]; inv_tac
  · rw [ite_eq_right c]; inv_tac
theorem inv_lost (he : s.stopped = false ∧ s.stage = sSubmit ∧ s.unknown = false) :
    Inv (apply s .lostResponse) := by
  rw [apply_en s _ (by simp [enabled, he])]; simp only [applyCore]; inv_tac
theorem inv_reconcile (r : Int) (he : s.stage = sSubmit ∧ s.unknown = true ∧ 0 ≤ r ∧ r ≤ 2) :
    Inv (apply s (.reconcile r)) := by
  rw [apply_en s _ (by simp [enabled, he])]; simp only [applyCore]
  have ht : s.txs = 1 := by have := h.oneTx; simp only [sSubmit] at *; omega
  by_cases c0 : r = 0
  · rw [ite_eq_left c0]; exact h
  rw [ite_eq_right c0]
  by_cases c1 : r = 1
  · rw [ite_eq_left c1]; inv_tac
  · rw [ite_eq_right c1]; inv_tac
theorem inv_report : Inv (apply s .reportComplete) := by
  rw [apply_en s _ (by simp [enabled])]; simp only [applyCore]
  by_cases c : 1 ≤ s.eligible ∧ s.joined = s.eligible ∧ s.mixedKeys = false
  · rw [ite_eq_left c]; inv_tac
  · rw [ite_eq_right c]; inv_tac
end

theorem apply_disabled (s : St) (op : Op) (he : enabled s op = false) : apply s op = s := by
  simp [apply, he]

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases hen : enabled s op
  · rw [apply_disabled s op hen]; exact h
  cases op with
  | step n ok => exact inv_step s h n ok (by simpa [enabled] using hen)
  | setupBox k => exact inv_setupBox s h k (by simpa [enabled] using hen)
  | rekey k => exact inv_rekey s h k (by simpa [enabled] using hen)
  | newCall id => exact inv_newCall s h id (by simpa [enabled] using hen)
  | checkPins m => exact inv_checkPins s h m (by simpa [enabled] using hen)
  | proveOnBox b => exact inv_prove s h b (by simpa [enabled] using hen)
  | deploySecond vk => exact inv_deploySecond s h vk (by simpa [enabled] using hen)
  | lostResponse => exact inv_lost s h (by simpa [enabled] using hen)
  | reconcile r => exact inv_reconcile s h r (by simpa [enabled] using hen)
  | reportComplete => exact inv_report s h

theorem inv_stepOp (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp
  split
  · exact h
  · rename_i op _
    exact inv_apply s op h

theorem inv_runOps (s : St) (ns : List Int) (h : Inv s) : Inv (runOps s ns) := by
  induction ns generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_stepOp s n h)

theorem inv_run (vk : Int) (ns : List Int) : Inv (run vk ns) := inv_runOps _ _ (inv_init _)

/-! ## Requirement-level corollaries over every run -/

theorem EUF_005_no_tx_after_stop (vk : Int) (ns : List Int) :
    (run vk ns).stopped = true → (run vk ns).txs = 0 := fun h => ((inv_run vk ns).stoppedNoTx h).1

theorem EUF_006_native_refusal_no_prove (vk : Int) (ns : List Int) :
    (run vk ns).nativeRejected = true → (run vk ns).stage < 7 ∧ (run vk ns).txs = 0 := by
  intro h
  have r := (inv_run vk ns).rejected h
  exact ⟨r.2.2, ((inv_run vk ns).stoppedNoTx r.1).1⟩

theorem EUF_007_prove_needs_profile (vk : Int) (ns : List Int) :
    7 ≤ (run vk ns).stage → (run vk ns).provedOk = true := (inv_run vk ns).proved

theorem EUF_008_one_claim (vk : Int) (ns : List Int) :
    (run vk ns).claimA ≠ 0 → (run vk ns).claimB ≠ 0 → (run vk ns).claimA = (run vk ns).claimB := by
  have := (inv_run vk ns).oneClaim; omega

theorem EUF_009_one_wrap_key (vk : Int) (ns : List Int) :
    1 ≤ (run vk ns).programs → (run vk ns).lastDeployVk = (run vk ns).universalVk :=
  (inv_run vk ns).oneKey.2.2

theorem EUF_002_one_deploy_per_program (vk : Int) (ns : List Int) :
    (run vk ns).deploys = (run vk ns).programs := (inv_run vk ns).oneKey.1

theorem EUF_014_one_tx_per_claim (vk : Int) (ns : List Int) : (run vk ns).txs ≤ 1 :=
  (inv_run vk ns).oneTx.2.1

theorem EUF_013_complete_joined (vk : Int) (ns : List Int) :
    (run vk ns).complete = true → (run vk ns).joined = (run vk ns).eligible ∧ (run vk ns).mixedKeys = false :=
  (inv_run vk ns).completeOk

theorem EUF_012_pin_stops (vk : Int) (ns : List Int) :
    (run vk ns).failedPin ≠ 0 → (run vk ns).stopped = true := (inv_run vk ns).pinNamed.2.2

theorem EUF_010_outcome_after_readback (vk : Int) (ns : List Int) :
    (run vk ns).outcome ≠ 0 → (run vk ns).stage = 11 := (inv_run vk ns).outcomeOk.2.2

/-- EUF-001: one step advances exactly one stage (the guard `n = stage + 1`). -/
theorem EUF_001_order (s : St) (n : Int) (ok : Bool) (h : enabled s (.step n ok) = true) :
    n = s.stage + 1 := by
  simp [enabled] at h; exact h.2.1

end CallFlow
