/-!
# Ledger Verification oracle (bounded context Ledger Verification, slice C)

Executable reference semantics of `sherlock-e2e-ledger-verifier` (LVR-001 to LVR-008)
and of the adopted MID requirements that constrain the check tool (MID-001, MID-003).
Spec text only; no code was read to write it.

Three parts share one state:

* the ledger check tool: the manifest gate over the 4 ledger crates (LVR-002), the key
  reader (LVR-003, MID-001), the check with its 10 tamper controls and exit status
  (LVR-001, LVR-004, LVR-005);
* the settle runner's native check of one proven call before the submit (LVR-006, LVR-007),
  `callPhase`: 0 no call, 1 built, 2 native accepted, 3 submitted, 4 natively refused;
* the chain identity of one state folder (LVR-008).
-/

namespace LedgerVerification

set_option linter.unusedSimpArgs false

/-- LVR-004: 7 changed-lane vectors and 3 flipped-byte proofs. -/
def lanesCtl : Int := 7
def flipsCtl : Int := 3
def opBase : Int := 1000000000

/-! `lastRefusal` codes (state outcomes, never `--step` refusals). -/
def refNone : Int := 0
def refG4 : Int := 1        -- LVR-003: the ledger reader refuses the key
def refNative : Int := 2    -- LVR-007: the native validator refused the honest call
def refChain : Int := 3     -- LVR-008: "same local chain as the deploy"

structure St where
  cr0 : Bool
  cr1 : Bool
  cr2 : Bool
  cr3 : Bool
  gateOk : Bool
  keyRead : Bool
  lastG4 : Bool
  checks : Int
  controls : Int
  checkDone : Bool
  exitNonzero : Bool
  lastHonest : Bool
  lastLanes : Int
  lastFlips : Int
  lastGate : Bool
  callPhase : Int
  nativeChecks : Int
  nativeOk : Int
  refusedCalls : Int
  submits : Int
  genesis : Int
  nodeVersion : Int
  connects : Int
  chainStop : Bool
  lastRefusal : Int
  deriving Repr, DecidableEq

def init : St :=
  { cr0 := false, cr1 := false, cr2 := false, cr3 := false, gateOk := false, keyRead := false,
    lastG4 := false, checks := 0, controls := 0, checkDone := false, exitNonzero := false,
    lastHonest := false, lastLanes := 0, lastFlips := 0, lastGate := false, callPhase := 0,
    nativeChecks := 0, nativeOk := 0, refusedCalls := 0, submits := 0, genesis := 0,
    nodeVersion := 0, connects := 0, chainStop := false, lastRefusal := refNone }

inductive Op where
  | manifest (i : Int) (atRev : Bool)          -- 1: crate i (0..3) resolves to rev a2b30128 or not
  | gate                                       -- 2: the manifest gate runs
  | loadKey (g4 : Bool)                        -- 3: the ledger reader loads wrap.vk
  | check (honest : Bool) (lanes flips : Int)  -- 4: one check run and its controls' results
  | buildCall                                  -- 5: a ledger call is built (new proof)
  | native (ok : Bool)                         -- 6: the native 9.1.0.1 validator answers
  | submit                                     -- 7: the settle runner submits the call
  | connect (genesis version : Int)            -- 8: the runner connects to a node
  deriving Repr, DecidableEq

def bit (a : Int) : Option Bool := if a = 0 then some false else if a = 1 then some true else none

/-- `code * 10^9 + arg`. manifest: arg = 10 i + b; loadKey, native: arg = b;
check: arg = h + 10 lanes + 100 flips (lanes 0..7, flips 0..3); connect: arg = 1000 g + v
(g >= 1, v 0..999); gate, buildCall, submit: arg = 0. -/
def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then (if a / 10 ≤ 3 then (bit (a % 10)).map (.manifest (a / 10)) else none)
  else if c = 2 then (if a = 0 then some .gate else none)
  else if c = 3 then (bit a).map .loadKey
  else if c = 4 then
    (if (a / 10) % 10 ≤ 7 ∧ a / 100 ≤ 3 then (bit (a % 10)).map (fun h => .check h ((a / 10) % 10) (a / 100))
     else none)
  else if c = 5 then (if a = 0 then some .buildCall else none)
  else if c = 6 then (bit a).map .native
  else if c = 7 then (if a = 0 then some .submit else none)
  else if c = 8 then (if 1 ≤ a / 1000 then some (.connect (a / 1000) (a % 1000)) else none)
  else none

def enabled (s : St) : Op → Bool
  | .manifest i _ => 0 ≤ i ∧ i ≤ 3
  | .gate => true
  | .loadKey _ => true
  | .check _ l f => s.keyRead ∧ 0 ≤ l ∧ l ≤ 7 ∧ 0 ≤ f ∧ f ≤ 3
  | .buildCall => s.callPhase = 0 ∨ s.callPhase = 3 ∨ s.callPhase = 4
  | .native _ => s.callPhase = 1
  | .submit => s.callPhase = 2
  | .connect g _ => 1 ≤ g ∧ s.chainStop = false

def apply (s : St) (op : Op) : St :=
  if !enabled s op then s else
  match op with
  | .manifest i b =>
    -- LVR-002: a manifest change invalidates the gate until it runs again
    let s' := { s with gateOk := false }
    if i = 0 then { s' with cr0 := b } else if i = 1 then { s' with cr1 := b }
    else if i = 2 then { s' with cr2 := b } else { s' with cr3 := b }
  | .gate =>
    -- LVR-002: the gate passes only when all 4 crates resolve to rev a2b30128
    { s with gateOk := s.cr0 && s.cr1 && s.cr2 && s.cr3 }
  | .loadKey g4 =>
    -- LVR-003, MID-001: the ledger's own reader refuses a G4 key
    if g4 then { s with keyRead := false, lastG4 := true, lastRefusal := refG4 }
    else { s with keyRead := true, lastG4 := false, lastRefusal := refNone }
  | .check h l f =>
    -- LVR-004: every check runs 7 + 3 controls; LVR-005: a passed control or a refused
    -- honest proof gives a nonzero exit; LVR-001 (LEAD PICK): a check whose ledger crates
    -- did not pass the manifest gate is not a check by the stock verifier, so it also fails
    { s with checks := s.checks + 1, controls := s.controls + (lanesCtl + flipsCtl),
             checkDone := true, lastHonest := h, lastLanes := l, lastFlips := f,
             lastGate := s.gateOk,
             exitNonzero := if h = true ∧ l = lanesCtl ∧ f = flipsCtl ∧ s.gateOk = true then false else true }
  | .buildCall => { s with callPhase := 1 }
  | .native ok =>
    -- LVR-006: the native check runs on the built call; LVR-007: a refusal is recorded and
    -- nothing is submitted (the next run proves a new call)
    if ok then { s with callPhase := 2, nativeChecks := s.nativeChecks + 1, nativeOk := s.nativeOk + 1 }
    else { s with callPhase := 4, nativeChecks := s.nativeChecks + 1,
                  refusedCalls := s.refusedCalls + 1, lastRefusal := refNative }
  | .submit => { s with callPhase := 3, submits := s.submits + 1 }
  | .connect g v =>
    -- LVR-008: the first connect records genesis and node version; another genesis stops
    if s.genesis = 0 then { s with genesis := g, nodeVersion := v, connects := s.connects + 1 }
    else if g = s.genesis then { s with connects := s.connects + 1 }
    else { s with chainStop := true, lastRefusal := refChain }

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
  0 ≤ s.checks && 0 ≤ s.controls && 0 ≤ s.lastLanes && s.lastLanes ≤ 7 && 0 ≤ s.lastFlips
    && s.lastFlips ≤ 3 && 0 ≤ s.callPhase && s.callPhase ≤ 4 && 0 ≤ s.nativeChecks
    && 0 ≤ s.nativeOk && 0 ≤ s.refusedCalls && 0 ≤ s.submits && 0 ≤ s.genesis
    && 0 ≤ s.nodeVersion && 0 ≤ s.connects && 0 ≤ s.lastRefusal && s.lastRefusal ≤ 3

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
  /-- LVR-002: the gate passes only with all 4 crates at the stock revision. -/
  gateCrates : s.gateOk = true → s.cr0 = true ∧ s.cr1 = true ∧ s.cr2 = true ∧ s.cr3 = true
  /-- LVR-003, MID-001: a key that the reader holds is not a G4 key. -/
  keyNotG4 : s.keyRead = true → s.lastG4 = false
  /-- LVR-004: every check ran exactly 10 tamper controls. -/
  controlsPerCheck : s.controls = 10 * s.checks
  /-- LVR-005, LVR-004, LVR-001: exit 0 only with the honest proof accepted, all 7 lanes and
  all 3 flipped bytes refused, and the manifest gate passed. -/
  exitZero : s.checkDone = true → s.exitNonzero = false →
    s.lastHonest = true ∧ s.lastLanes = lanesCtl ∧ s.lastFlips = flipsCtl ∧ s.lastGate = true
  /-- LVR-006: every native check ended either accepted or refused. -/
  nativeSplit : s.nativeChecks = s.nativeOk + s.refusedCalls
  /-- LVR-006, LVR-007: every submit follows a native acceptance; a refused call is not
  submitted. -/
  submitAfterNative : s.submits + (if s.callPhase = 2 then 1 else 0) = s.nativeOk
  phaseRange : 0 ≤ s.callPhase ∧ s.callPhase ≤ 4
  counts : 0 ≤ s.submits ∧ 0 ≤ s.refusedCalls ∧ 0 ≤ s.nativeOk ∧ 0 ≤ s.checks
  /-- LVR-008: a folder holds a genesis once it has connected. -/
  genesisSet : (s.connects = 0 ↔ s.genesis = 0) ∧ 0 ≤ s.genesis ∧ 0 ≤ s.connects

theorem inv_init : Inv init := by constructor <;> simp [init] <;> decide

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  cases h
  constructor <;> simp_all [lanesCtl, flipsCtl, refG4, refNative, refChain, refNone] <;> omega))

theorem inv_manifest (i : Int) (b : Bool) : Inv (apply s (.manifest i b)) := by
  by_cases he : 0 ≤ i ∧ i ≤ 3
  · by_cases h0 : i = 0
    · simp [apply, enabled, he, h0]; inv_tac
    · by_cases h1 : i = 1
      · simp [apply, enabled, he, h0, h1]; inv_tac
      · by_cases h2 : i = 2
        · simp [apply, enabled, he, h0, h1, h2]; inv_tac
        · simp [apply, enabled, he, h0, h1, h2]; inv_tac
  · simp [apply, enabled, he]; exact h
theorem inv_gate : Inv (apply s .gate) := by simp [apply, enabled]; inv_tac
theorem inv_loadKey (g : Bool) : Inv (apply s (.loadKey g)) := by
  cases g <;> simp [apply, enabled] <;> inv_tac
theorem inv_check (x : Bool) (l f : Int) : Inv (apply s (.check x l f)) := by
  by_cases he : s.keyRead = true ∧ 0 ≤ l ∧ l ≤ 7 ∧ 0 ≤ f ∧ f ≤ 3
  · by_cases hx : x = true ∧ l = lanesCtl ∧ f = flipsCtl ∧ s.gateOk = true
    · simp only [apply, enabled, he, hx, decide_true, Bool.not_true, Bool.false_eq_true, ite_false, ite_true, and_self]
      inv_tac
    · simp only [apply, enabled, he, hx, decide_true, Bool.not_true, Bool.false_eq_true, ite_false, and_self]
      inv_tac
  · have : enabled s (.check x l f) = false := by unfold enabled; exact decide_eq_false he
    simp [apply, this]; exact h
theorem inv_buildCall : Inv (apply s .buildCall) := by
  by_cases he : s.callPhase = 0 ∨ s.callPhase = 3 ∨ s.callPhase = 4
  · simp [apply, enabled, he]; inv_tac
  · simp [apply, enabled, he]; exact h
theorem inv_native (x : Bool) : Inv (apply s (.native x)) := by
  by_cases he : s.callPhase = 1
  · cases x <;> simp [apply, enabled, he] <;> inv_tac
  · simp [apply, enabled, he]; exact h
theorem inv_submit : Inv (apply s .submit) := by
  by_cases he : s.callPhase = 2
  · simp [apply, enabled, he]; inv_tac
  · simp [apply, enabled, he]; exact h
theorem inv_connect (g v : Int) : Inv (apply s (.connect g v)) := by
  by_cases he : 1 ≤ g ∧ s.chainStop = false
  · by_cases h0 : s.genesis = 0
    · simp [apply, enabled, he, h0]; inv_tac
    · have hen : (!enabled s (.connect g v)) = false := by simp [enabled, he]
      unfold apply; rw [hen]; dsimp only
      rw [ite_eq_right_of_eq_false _ _ (by simp), ite_eq_right_of_eq_false _ _ (eq_false h0)]
      by_cases h1 : g = s.genesis
      · rw [ite_eq_left_of_eq_true _ _ (eq_true h1)]; inv_tac
      · rw [ite_eq_right_of_eq_false _ _ (eq_false h1)]; inv_tac
  · have : enabled s (.connect g v) = false := by unfold enabled; exact decide_eq_false he
    simp [apply, this]; exact h
end

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases op with
  | manifest i b => exact inv_manifest s h i b
  | gate => exact inv_gate s h
  | loadKey g => exact inv_loadKey s h g
  | check x l f => exact inv_check s h x l f
  | buildCall => exact inv_buildCall s h
  | native x => exact inv_native s h x
  | submit => exact inv_submit s h
  | connect g v => exact inv_connect s h g v

theorem inv_stepOp (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp; split
  · exact h
  · exact inv_apply s _ h

theorem inv_runOps (s : St) (ns : List Int) (h : Inv s) : Inv (runOps s ns) := by
  induction ns generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_stepOp s n h)

theorem inv_run (ns : List Int) : Inv (run ns) := inv_runOps _ _ inv_init

/-! ## Requirement-level corollaries -/

theorem LVR_002_gate (ns : List Int) : (run ns).gateOk = true →
    (run ns).cr0 = true ∧ (run ns).cr1 = true ∧ (run ns).cr2 = true ∧ (run ns).cr3 = true :=
  (inv_run ns).gateCrates

theorem LVR_003_no_g4_key (ns : List Int) : (run ns).keyRead = true → (run ns).lastG4 = false :=
  (inv_run ns).keyNotG4

theorem LVR_004_ten_controls (ns : List Int) : (run ns).controls = 10 * (run ns).checks :=
  (inv_run ns).controlsPerCheck

theorem LVR_005_exit_status (ns : List Int) : (run ns).checkDone = true → (run ns).exitNonzero = false →
    (run ns).lastHonest = true ∧ (run ns).lastLanes = 7 ∧ (run ns).lastFlips = 3 ∧ (run ns).lastGate = true :=
  (inv_run ns).exitZero

theorem LVR_006_native_before_submit (ns : List Int) : (run ns).submits ≤ (run ns).nativeOk := by
  have := (inv_run ns).submitAfterNative; split at this <;> omega

theorem LVR_007_refused_not_submitted (ns : List Int) :
    (run ns).submits + (run ns).refusedCalls ≤ (run ns).nativeChecks := by
  have h1 := (inv_run ns).submitAfterNative; have h2 := (inv_run ns).nativeSplit
  split at h1 <;> omega

/-- LVR-007: a natively refused call is never submitted; only a new built call can be. -/
theorem LVR_007_step (s : St) (h : s.callPhase = 4) : apply s .submit = s := by
  simp [apply, enabled, h]

/-- LVR-008: once a genesis is recorded, no operation changes it. -/
theorem LVR_008_genesis_stable (s : St) (op : Op) (h : s.genesis ≠ 0) : (apply s op).genesis = s.genesis := by
  unfold apply
  split
  · rfl
  · cases op with
    | manifest i b => simp only <;> (repeat' split) <;> rfl
    | connect g v => simp only [h, ite_false]; (repeat' split) <;> (try simp_all)
    | _ => simp only <;> (repeat' split) <;> (try rfl)

/-! ## Constants checked against MEASURED values -/

/-- MID-003: VK at most 50,000 bytes; T17 (a) installed wrap VK 1,325 bytes on the T10m
keys (`t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:17`). -/
theorem measured_vk_bytes : (1325 : Int) ≤ 50000 := by decide
/-- MID-003: 1,026 public inputs = 2 + L_TX, at most 65,536. -/
theorem mid003_public_inputs : (2 + 1024 : Int) = 1026 ∧ (1026 : Int) ≤ 65536 := by decide
/-- MID-003: the transaction bound 1,048,576 bytes; Preview limit MEASURED 1,048,576
(`t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:15`); `acceptRun`
6,780 bytes on Preview (`t17-preview@9e1e7f398:...RESULTS.md:19`) and 6,783 bytes on devnet
block 22,728 (`t20-batched@5dc1be2ee:...RESULTS.md:11`). -/
theorem measured_tx_bytes : (6780 : Int) ≤ 1048576 ∧ (6783 : Int) ≤ 1048576 ∧ (1048576 : Int) = 2 ^ 20 := by decide
/-- LVR-004: the 7 named lanes 0, 10, 20, 34, 39, the middle lane 513 and the last lane 1,025
of the 1,026-lane vector are distinct and in range; 7 + 3 = 10 controls. -/
theorem lvr004_lanes : [0, 10, 20, 34, 39, 513, 1025].Nodup ∧ [0, 10, 20, 34, 39, 513, 1025].all (· < 1026)
    ∧ [0, 10, 20, 34, 39, 513, 1025].length = 7 ∧ lanesCtl + flipsCtl = 10 := by decide

end LedgerVerification
