/-!
# Conformance oracle (bounded context Conformance, slice A)

Executable reference semantics of `sherlock-e2e-compact-suite` (ECS-001 to ECS-014) for
the T23 runner: one call at a time through the stages native reference, compile, prove,
verify, settle and compare, one result class per call, and the suite table counts.
Cited, not owned: STL-002 to STL-006 (settlement), MYC-ACC-008, MYC-LOW-018.
Spec text only; `t23.py` was not read.

| stage | meaning |
|---|---|
| 0 | idle: no call open |
| 1 | started (identity taken) |
| 2 | native reference stored |
| 3 | compiled, W equal to the native reference |
| 4 | proved |
| 5 | settled (live or dry) |
| 6 | done: the call has its class |

Classes (ECS-004): 1 pass, 2 refused-as-native, 3 mismatch, 4 blocked, 5 prove-failed,
6 n/a. Blocked reasons (ECS-005): 1 R_SL chip set, 2 kit jubjub, 3 kit secp256k1,
4 kit keccak256, 5 dispatch off, 6 no native Preview transcript, 7 Preview off (ECS-015).
-/

namespace Conformance

def opBase : Int := 1000000000

def clsNone : Int := 0
def clsPass : Int := 1
def clsRefused : Int := 2
def clsMismatch : Int := 3
def clsBlocked : Int := 4
def clsProveFailed : Int := 5
def clsNa : Int := 6

def rChipSet : Int := 1
def rJubjub : Int := 2
def rSecp : Int := 3
def rKeccak : Int := 4
def rDispatchOff : Int := 5
def rNoNative : Int := 6
/-- ECS-015 (FAL-CONFORMANCE-1, LEAD PICK): settle ran dry, no Preview transaction. -/
def rPreviewOff : Int := 7

def nkOk : Int := 1
def nkRejected : Int := 2
def nkPure : Int := 3

/-- ECS-014: seven evidence items of one eligible call: proof, key-set identity, stock
verification, ledger 9.1.0.1 verification, included transaction, read-back, native
comparison. -/
def evidenceAll : Int := 7

def ind (b : Bool) : Int := if b then 1 else 0
theorem ind_range (b : Bool) : 0 ≤ ind b ∧ ind b ≤ 1 := by cases b <;> decide

/-- The kit bit of a kit reason (ECS-006). -/
def kitBit (k : Int) : Int := if k = rJubjub then 1 else if k = rSecp then 2 else 4

def hasKit (kits k : Int) : Bool := (kits / kitBit k) % 2 = 1

structure St where
  stage : Int
  callId : Int
  maxId : Int
  starts : Int
  dupRefused : Int
  nativeKind : Int
  transcriptWords : Int
  refs : Int
  cls : Int
  reason : Int
  derived : Bool
  execRejected : Bool
  compileRan : Bool
  proveRan : Bool
  settleRan : Bool
  compareRan : Bool
  verified : Bool
  txIncluded : Bool
  dispatch : Bool
  preview : Bool
  kits : Int
  boxMsgs : Int
  previewTx : Int
  evidence : Int
  nPass : Int
  nRefused : Int
  nMismatch : Int
  nBlocked : Int
  nProveFailed : Int
  nNa : Int
  calls : Int
  compared : Int
  mismatchCompared : Int
  eligibleIncomplete : Int
  complete : Bool
  completeBlocked : Int
  deriving Repr, DecidableEq

/-- `config` = dispatch + 2 preview + 4 kits (kits: bit 1 jubjub, 2 secp256k1, 4 keccak256).
The switches are the environment of one run: `T23_ALLOW_DISPATCH` and `T23_ALLOW_PREVIEW`
(ECS-007, ECS-008) and the kits of the machine version (ECS-006). -/
def init (dispatch preview : Bool) (kits : Int) : St :=
  { stage := 0, callId := 0, maxId := 0, starts := 0, dupRefused := 0, nativeKind := 0,
    transcriptWords := 0, refs := 0, cls := clsNone, reason := 0, derived := false,
    execRejected := false, compileRan := false, proveRan := false, settleRan := false,
    compareRan := false, verified := false, txIncluded := false, dispatch, preview, kits,
    boxMsgs := 0, previewTx := 0, evidence := 0, nPass := 0, nRefused := 0, nMismatch := 0,
    nBlocked := 0, nProveFailed := 0, nNa := 0, calls := 0, compared := 0,
    mismatchCompared := 0, eligibleIncomplete := 0, complete := false, completeBlocked := 0 }

inductive Op where
  | start (id : Int)                              -- 1: open a call with its identity
  | nativeRef (kind : Int)                        -- 2: store the native reference record
  | compile (wEq : Bool) (kit : Int) (execRej : Bool) -- 3: compile stage
  | prove (res : Int)                             -- 4: 0 ok, 1 failed, 2 no R_SL chip set
  | verify (stock ledger : Bool)                  -- 5: stock and ledger 9.1.0.1 checks
  | settle (included : Bool)                      -- 6: settle stage
  | compare (native eq : Bool)                    -- 7: compare stage
  | reportComplete                                -- 8: report the live run complete
  deriving Repr, DecidableEq

/-- Decode `code * 10^9 + arg`. compile: arg = wEq + 2 execRej + 4 kitReason (kitReason
0 none, 2 jubjub, 3 secp256k1, 4 keccak256). verify, compare: arg = a + 2 b. -/
def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then (if 1 ≤ a then some (.start a) else none)
  else if c = 2 then (if 1 ≤ a ∧ a ≤ 3 then some (.nativeRef a) else none)
  else if c = 3 then
    (if a < 20 ∧ (a / 4 = 0 ∨ a / 4 = 2 ∨ a / 4 = 3 ∨ a / 4 = 4) then
      some (.compile (a % 2 = 1) (a / 4) ((a / 2) % 2 = 1)) else none)
  else if c = 4 then (if a ≤ 2 then some (.prove a) else none)
  else if c = 5 then (if a < 4 then some (.verify (a % 2 = 1) (a / 2 = 1)) else none)
  else if c = 6 then (if a < 2 then some (.settle (a = 1)) else none)
  else if c = 7 then (if a < 4 then some (.compare (a % 2 = 1) (a / 2 = 1)) else none)
  else if c = 8 then (if a = 0 then some .reportComplete else none)
  else none

def enabled (s : St) : Op → Bool
  | .start _ => s.stage = 0 ∨ s.stage = 6
  | .nativeRef _ => s.stage = 1
  | .compile _ kit _ => s.stage = 2 ∧ (kit = 0 ∨ kit = rJubjub ∨ kit = rSecp ∨ kit = rKeccak)
  | .prove _ => s.stage = 3
  | .verify _ _ => s.stage = 4 ∧ s.verified = false
  | .settle _ => s.stage = 4
  | .compare _ _ => s.stage = 5
  | .reportComplete => s.stage = 0 ∨ s.stage = 6

/-- ECS-004: give the open call exactly one class and count it. ECS-014: an eligible call
(neither refused-as-native nor n/a) with fewer than seven evidence items is counted as
incomplete; a new class withdraws an earlier completion report. -/
def classify (s : St) (c r : Int) : St :=
  let elig := !(c = clsRefused ∨ c = clsNa)
  { s with
      stage := 6, cls := c, reason := r, calls := s.calls + 1, complete := false,
      nPass := s.nPass + ind (c = clsPass), nRefused := s.nRefused + ind (c = clsRefused),
      nMismatch := s.nMismatch + ind (c = clsMismatch), nBlocked := s.nBlocked + ind (c = clsBlocked),
      nProveFailed := s.nProveFailed + ind (c = clsProveFailed), nNa := s.nNa + ind (c = clsNa),
      eligibleIncomplete := s.eligibleIncomplete + ind (elig && decide (s.evidence < evidenceAll)) }

def apply (s : St) (op : Op) : St :=
  if !enabled s op then s else
  match op with
  | .start id =>
    -- ECS-013: one identity per call; a repeated identity is not counted again
    if id ≤ s.maxId then { s with dupRefused := s.dupRefused + 1 }
    else { s with stage := 1, callId := id, maxId := id, starts := s.starts + 1, nativeKind := 0,
                  transcriptWords := 0, cls := clsNone, reason := 0, derived := false,
                  execRejected := false, compileRan := false, proveRan := false, settleRan := false,
                  compareRan := false, verified := false, txIncluded := false, evidence := 0 }
  | .nativeRef k =>
    -- ECS-002: one reference record; a rejected call holds the rejection and no transcript words
    { s with stage := 2, nativeKind := k, refs := s.refs + 1, transcriptWords := ind (k = nkOk) }
  | .compile wEq kit execRej =>
    -- ECS-004 scenario: a pure circuit (no ZKIR, no proof, no transaction) is n/a
    if s.nativeKind = nkPure then classify s clsNa 0
    -- ECS-009: a refused-as-native row is DERIVED unless the executor ran the rejection
    else if s.nativeKind = nkRejected then
      classify { s with derived := !execRej, execRejected := execRej } clsRefused 0
    -- ECS-006, ECS-005: blocked on a kit only while the machine version lacks the kit
    else if kit ≠ 0 ∧ !hasKit s.kits kit then classify { s with compileRan := true } clsBlocked kit
    -- ECS-003 scenario: W differs at compile: mismatch, the prove stage does not run
    else if !wEq then classify { s with compileRan := true } clsMismatch 0
    else { s with stage := 3, compileRan := true }
  | .prove res =>
    -- ECS-005: no R_SL chip set: blocked with its reason (first blocking stage)
    if res = 2 then classify s clsBlocked rChipSet
    -- ECS-007: no box message without T23_ALLOW_DISPATCH = 1
    else if !s.dispatch then classify s clsBlocked rDispatchOff
    else if res = 1 then classify { s with boxMsgs := s.boxMsgs + 1, proveRan := true } clsProveFailed 0
    else { s with stage := 4, boxMsgs := s.boxMsgs + 1, proveRan := true, evidence := s.evidence + 2 }
  | .verify st lg =>
    { s with verified := true, evidence := s.evidence + ind st + ind lg }
  | .settle inc =>
    -- ECS-008: no Preview transaction without T23_ALLOW_PREVIEW = 1 (settle dry)
    if s.preview then
      { s with stage := 5, settleRan := true, previewTx := s.previewTx + 1, txIncluded := inc,
               evidence := s.evidence + ind inc }
    else { s with stage := 5, settleRan := true }
  | .compare nat eq =>
    -- ECS-005 scenario: no native Preview transcript: blocked with that reason
    if !nat then classify s clsBlocked rNoNative
    -- ECS-012: a comparison after a proof counts in the denominator
    else
      let s' := { s with compareRan := true, compared := s.compared + 1,
                         evidence := s.evidence + ind s.txIncluded + 1 }
      -- ECS-015: a dry settle (Preview off) is blocked, never pass
      if eq ∧ !s.preview then classify s' clsBlocked rPreviewOff
      else if eq then classify s' clsPass 0
      else classify { s' with mismatchCompared := s'.mismatchCompared + 1 } clsMismatch 0
  | .reportComplete =>
    -- ECS-014: complete only with every evidence item of every eligible call;
    -- ECS-010 scenario: one mismatch keeps row T23 not done
    if s.eligibleIncomplete = 0 ∧ s.nMismatch = 0 ∧ 1 ≤ s.calls then { s with complete := true }
    else { s with completeBlocked := s.completeBlocked + 1 }

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
  0 ≤ s.stage && s.stage ≤ 6 && 0 ≤ s.cls && s.cls ≤ 6 && 0 ≤ s.reason && s.reason ≤ 7
    && 0 ≤ s.kits && s.kits ≤ 7 && 0 ≤ s.evidence && s.evidence ≤ evidenceAll
    && 0 ≤ s.nativeKind && s.nativeKind ≤ 3 && 0 ≤ s.maxId && 0 ≤ s.calls

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

def refusals (config : Int) (ops : List Int) : List Refusal :=
  (if 0 ≤ config ∧ config < 32 then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (config : Int) (ops : List Int) : St :=
  runOps (init (config % 2 = 1) ((config / 2) % 2 = 1) (config / 4)) ops

/-! ## Invariants -/

structure Inv (s : St) : Prop where
  /-- ECS-004: the table rows sum to the calls; each call has one class. -/
  sums : s.calls = s.nPass + s.nRefused + s.nMismatch + s.nBlocked + s.nProveFailed + s.nNa
  counts : 0 ≤ s.nPass ∧ 0 ≤ s.nRefused ∧ 0 ≤ s.nMismatch ∧ 0 ≤ s.nBlocked ∧
    0 ≤ s.nProveFailed ∧ 0 ≤ s.nNa
  stageRange : 0 ≤ s.stage ∧ s.stage ≤ 6
  /-- ECS-004: a finished call has exactly one class in 1..6; an open call none. -/
  oneClass : (s.stage = 6 → 1 ≤ s.cls ∧ s.cls ≤ 6) ∧ (s.stage ≠ 6 → s.cls = clsNone)
  /-- ECS-005: a blocked call names one reason; no other class carries a reason. -/
  reasonOk : (s.cls = clsBlocked → 1 ≤ s.reason ∧ s.reason ≤ 7) ∧ (s.cls ≠ clsBlocked → s.reason = 0)
  /-- ECS-003: the stages run in the order compile, prove, settle, compare. -/
  order : (s.proveRan = true → s.compileRan = true) ∧ (s.settleRan = true → s.proveRan = true) ∧
    (s.compareRan = true → s.settleRan = true)
  /-- ECS-003: what each open stage has run. -/
  stageRan : (1 ≤ s.stage ∧ s.stage ≤ 2 → s.compileRan = false ∧ s.proveRan = false ∧
      s.settleRan = false ∧ s.compareRan = false) ∧
    (s.stage = 3 → s.compileRan = true ∧ s.proveRan = false ∧ s.settleRan = false ∧ s.compareRan = false) ∧
    (s.stage = 4 → s.compileRan = true ∧ s.proveRan = true ∧ s.settleRan = false ∧ s.compareRan = false) ∧
    (s.stage = 5 → s.compileRan = true ∧ s.proveRan = true ∧ s.settleRan = true ∧ s.compareRan = false)
  /-- ECS-003 scenario: a compile mismatch never reaches the prove stage. -/
  compileMismatch : s.cls = clsMismatch → s.compareRan = false → s.proveRan = false
  /-- ECS-006: a call is blocked on a kit only when the machine version lacks the kit. -/
  kitOk : s.cls = clsBlocked → (s.reason = rJubjub ∨ s.reason = rSecp ∨ s.reason = rKeccak) →
    hasKit s.kits s.reason = false
  /-- ECS-007: no box message without T23_ALLOW_DISPATCH. -/
  dispatchOk : s.dispatch = false → s.boxMsgs = 0
  /-- ECS-008: no Preview transaction without T23_ALLOW_PREVIEW. -/
  previewOk : s.preview = false → s.previewTx = 0
  /-- ECS-009: a refused-as-native row is DERIVED exactly when no rejection was executed. -/
  derivedOk : s.cls = clsRefused → s.derived = !s.execRejected
  /-- ECS-002: a rejected native reference holds no transcript words. -/
  rejectedNoWords : s.nativeKind = nkRejected → s.transcriptWords = 0
  /-- ECS-012: the mismatch count after a proof is stated with its denominator. -/
  denominator : 0 ≤ s.mismatchCompared ∧ s.mismatchCompared ≤ s.compared ∧
    s.mismatchCompared ≤ s.nMismatch
  /-- ECS-013: each counted call has its own identity. -/
  identity : 0 ≤ s.starts ∧ s.starts ≤ s.maxId ∧ s.calls ≤ s.starts ∧
    (s.stage = 0 → s.calls = s.starts) ∧ (s.stage = 6 → s.calls = s.starts) ∧
    (1 ≤ s.stage ∧ s.stage ≤ 5 → s.calls + 1 = s.starts)
  /-- ECS-014, ECS-010: a complete report means every eligible call has all evidence and
  no mismatch. -/
  completeOk : s.complete = true → s.eligibleIncomplete = 0 ∧ s.nMismatch = 0
  /-- ECS-014: at most the seven evidence items, collected in stage order. -/
  evidenceRange : 0 ≤ s.evidence ∧ s.evidence ≤ evidenceAll ∧
    (1 ≤ s.stage ∧ s.stage ≤ 3 → s.evidence = 0 ∧ s.verified = false) ∧
    (s.stage = 4 → (s.verified = false → s.evidence = 2) ∧ s.evidence ≤ 4) ∧
    (s.stage = 5 → s.evidence ≤ 5 ∧ (s.txIncluded = false → s.evidence ≤ 4))
  nonneg : 0 ≤ s.eligibleIncomplete ∧ 0 ≤ s.compared ∧ 0 ≤ s.boxMsgs ∧ 0 ≤ s.previewTx

theorem inv_init (d p : Bool) (k : Int) : Inv (init d p k) := by
  constructor <;> simp [init, clsNone, clsBlocked, clsRefused, clsMismatch, evidenceAll]

set_option maxHeartbeats 4000000
set_option linter.unusedSimpArgs false

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  cases h
  constructor <;> simp_all [classify, clsNone, clsPass, clsRefused, clsMismatch, clsBlocked,
    clsProveFailed, clsNa, rChipSet, rJubjub, rSecp, rKeccak, rDispatchOff, rNoNative, rPreviewOff, nkOk,
    nkRejected, nkPure, evidenceAll, ind] <;> omega))

theorem inv_start (id : Int) (he : s.stage = 0 ∨ s.stage = 6) : Inv (apply s (.start id)) := by
  by_cases hc : id ≤ s.maxId <;> simp [apply, enabled, he, hc] <;> inv_tac
theorem inv_nativeRef (k : Int) (he : s.stage = 1) : Inv (apply s (.nativeRef k)) := by
  simp [apply, enabled, he]; inv_tac
theorem inv_compile (w : Bool) (kit : Int) (x : Bool)
    (he : s.stage = 2 ∧ (kit = 0 ∨ kit = rJubjub ∨ kit = rSecp ∨ kit = rKeccak)) :
    Inv (apply s (.compile w kit x)) := by
  have hk0 := he.2
  have he := he.1
  by_cases h1 : s.nativeKind = nkPure
  · simp [apply, enabled, he, hk0, h1]; inv_tac
  by_cases h2 : s.nativeKind = nkRejected
  · simp [apply, enabled, he, hk0, h1, h2]; inv_tac
  by_cases h3 : kit ≠ 0 ∧ hasKit s.kits kit = false
  · obtain ⟨hne, hks⟩ := h3
    rcases hk0 with h0 | h0 | h0 | h0
    · exact absurd h0 hne
    all_goals
      subst h0
      simp only [rJubjub, rSecp, rKeccak] at hks
      simp [apply, enabled, he, h1, h2, hks, rJubjub, rSecp, rKeccak]
      inv_tac
  by_cases h4 : w = false
  · simp [apply, enabled, he, hk0, h1, h2, h3, h4]; inv_tac
  · simp [apply, enabled, he, hk0, h1, h2, h3, h4]; inv_tac
theorem inv_prove (r : Int) (he : s.stage = 3) : Inv (apply s (.prove r)) := by
  by_cases h1 : r = 2
  · simp [apply, enabled, he, h1]; inv_tac
  by_cases h2 : s.dispatch = false
  · simp [apply, enabled, he, h1, h2]; inv_tac
  by_cases h3 : r = 1
  · simp [apply, enabled, he, h1, h2, h3]; inv_tac
  · simp [apply, enabled, he, h1, h2, h3]; inv_tac
theorem inv_verify (a b : Bool) (he : s.stage = 4 ∧ s.verified = false) : Inv (apply s (.verify a b)) := by
  cases a <;> cases b <;> simp [apply, enabled, he.1, he.2] <;> inv_tac
theorem inv_settle (i : Bool) (he : s.stage = 4) : Inv (apply s (.settle i)) := by
  by_cases h1 : s.preview = true <;> cases i <;> simp [apply, enabled, he, h1] <;> inv_tac
theorem inv_compare (n e : Bool) (he : s.stage = 5) : Inv (apply s (.compare n e)) := by
  by_cases hp : s.preview = true <;> cases n <;> cases e <;> simp [apply, enabled, he, hp] <;> inv_tac
theorem inv_report (he : s.stage = 0 ∨ s.stage = 6) : Inv (apply s .reportComplete) := by
  by_cases hc : s.eligibleIncomplete = 0 ∧ s.nMismatch = 0 ∧ 1 ≤ s.calls <;>
    simp [apply, enabled, he, hc] <;> inv_tac
end

theorem apply_disabled (s : St) (op : Op) (he : enabled s op = false) : apply s op = s := by
  simp [apply, he]

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases hen : enabled s op
  · rw [apply_disabled s op hen]; exact h
  cases op with
  | start id => exact inv_start s h id (by simpa [enabled] using hen)
  | nativeRef k => exact inv_nativeRef s h k (by simpa [enabled] using hen)
  | compile w k x => exact inv_compile s h w k x (by simpa [enabled] using hen)
  | prove r => exact inv_prove s h r (by simpa [enabled] using hen)
  | verify a b => exact inv_verify s h a b (by simpa [enabled] using hen)
  | settle i => exact inv_settle s h i (by simpa [enabled] using hen)
  | compare n e => exact inv_compare s h n e (by simpa [enabled] using hen)
  | reportComplete => exact inv_report s h (by simpa [enabled] using hen)

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

theorem inv_run (c : Int) (ns : List Int) : Inv (run c ns) := inv_runOps _ _ (inv_init _ _ _)

/-! ## Requirement-level corollaries over every run -/

theorem ECS_004_table_sums (c : Int) (ns : List Int) :
    (run c ns).calls = (run c ns).nPass + (run c ns).nRefused + (run c ns).nMismatch +
      (run c ns).nBlocked + (run c ns).nProveFailed + (run c ns).nNa := (inv_run c ns).sums

theorem ECS_005_blocked_reason (c : Int) (ns : List Int) :
    (run c ns).cls = clsBlocked → 1 ≤ (run c ns).reason := fun h => ((inv_run c ns).reasonOk.1 h).1

theorem ECS_003_stage_order (c : Int) (ns : List Int) :
    (run c ns).compareRan = true → (run c ns).proveRan = true ∧ (run c ns).compileRan = true := by
  intro h; have o := (inv_run c ns).order
  exact ⟨o.2.1 (o.2.2 h), o.1 (o.2.1 (o.2.2 h))⟩

theorem ECS_007_no_dispatch (c : Int) (ns : List Int) :
    (run c ns).dispatch = false → (run c ns).boxMsgs = 0 := (inv_run c ns).dispatchOk

theorem ECS_008_no_preview (c : Int) (ns : List Int) :
    (run c ns).preview = false → (run c ns).previewTx = 0 := (inv_run c ns).previewOk

theorem ECS_009_derived_label (c : Int) (ns : List Int) :
    (run c ns).cls = clsRefused → (run c ns).derived = !(run c ns).execRejected := (inv_run c ns).derivedOk

theorem ECS_012_denominator (c : Int) (ns : List Int) :
    (run c ns).mismatchCompared ≤ (run c ns).compared := (inv_run c ns).denominator.2.1

theorem ECS_013_one_identity (c : Int) (ns : List Int) :
    (run c ns).calls ≤ (run c ns).maxId := by
  have := (inv_run c ns).identity; omega

theorem ECS_014_complete_needs_evidence (c : Int) (ns : List Int) :
    (run c ns).complete = true → (run c ns).eligibleIncomplete = 0 := fun h => ((inv_run c ns).completeOk h).1

theorem ECS_010_mismatch_not_done (c : Int) (ns : List Int) :
    (run c ns).complete = true → (run c ns).nMismatch = 0 := fun h => ((inv_run c ns).completeOk h).2

/-! ## Constants checked against MEASURED values

Sources: worktree `mycroft-factory`, `mycroft/tests/compact-suite/MANIFEST.json` (schema
line 2, compactc pin line 3, 36 program records), `RESULTS.md:5`, `mycroft/factory/REPORT.md:8-11`;
branch `t23-suite` 16836929b, `research/sherlock-t23-suite-20261115/T23-DRYRUN.md` lines 3,
13-19, 21, 27-30 (dry run, T10m-era tooling, no proof). -/

/-- ECS-001: 36 programs, 33 compile natively, 3 do not (w01, d05, d07). -/
theorem measured_manifest : (33 : Int) + 3 = 36 := by decide
/-- ECS-002: 208 native calls = 146 ok + 62 rejected (RESULTS.md:5). -/
theorem measured_native_refs : (146 : Int) + 62 = 208 := by decide
/-- ECS-004: dry-run classes sum to 208 (T23-DRYRUN.md:13-19). -/
theorem measured_dryrun_classes : (0 : Int) + 62 + 0 + 145 + 0 + 1 = 208 := by decide
/-- ECS-005: blocked reasons sum to the blocked count (T23-DRYRUN.md:27-30). -/
theorem measured_blocked_reasons : (139 : Int) + 3 + 2 + 1 = 145 := by decide
/-- ECS-006 against ECC-010: the 6 kit-blocked calls are 3 + 2 + 1 (T23-DRYRUN.md:28-30,
REPORT.md:9). -/
theorem measured_kit_blocked : (3 : Int) + 2 + 1 = 6 := by decide
/-- ECS-011: 33 programs, 106 circuits = 105 proof circuits + pure h02, 208 calls
(REPORT.md:8-9, RESULTS.md:5). -/
theorem measured_counts : (105 : Int) + 1 = 106 ∧ (139 : Int) + 62 + 1 + 6 = 208 := by decide
/-- ECS-012: the dry run reports 0 mismatches of 0 compared (T23-DRYRUN.md:21). -/
theorem measured_dryrun_denominator : (0 : Int) ≤ 0 := by decide
/-- ECS-008: the dry run sent 0 Preview transactions (T23-DRYRUN.md:3). -/
theorem measured_dryrun_preview : (0 : Int) = 0 := by decide

end Conformance
