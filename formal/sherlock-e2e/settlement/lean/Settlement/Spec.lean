/-!
# Settlement oracle (bounded context Settlement, slice C)

Executable reference semantics of `sherlock-e2e-ledger-contract` (SLC-001 to SLC-012) and
`sherlock-e2e-batched-settlement` (BAT-001 to BAT-012), with the adopted requirements that
constrain them: LDG-001 to LDG-008 as amended on `t20-batched`, STL-001 to STL-006
(completion), ANC-012 (on `t20-batched`, capped by BAT-005), ANC-014, ANC-015, ERT-004 and
GOV-006 (the era end), CRR-007. Spec text only; no code was read to write it.

One state holds four parts:

* the SherlockLedger instance of one program: four entries, the sealed anchor in slot 15,
  keys per entry, `runs` over three claim keys `r1 .. r3` (0 = absent, else `outcome_pub`);
* the settle runner of one user's call: argument file, runner gates, node verdict;
* the epoch operator: one open epoch, the clock `now`, the claims roots `cr1 .. cr3`;
* the plan tool (`K_MAX`), the intent TTL, run starts against the era end, the T15 record.

Every refusal the spec names is a state outcome: `lastRefusal` is a sum of the bits below
(several bits when several refusals apply and the spec gives no order).
-/

namespace Settlement

set_option linter.unusedSimpArgs false

def opBase : Int := 1000000000
/-- BAT-001: epoch length, s. -/
def epochLen : Int := 3600
/-- BAT-001, LDG-005: the claims tree of depth 20 holds 1,048,576 claims. -/
def epochCap : Int := 1048576
/-- BAT-006, ANC-012: default intent TTL, s. -/
def ttlDefault : Int := 3600
/-- ANC-012: effect words of one call, `L_TX - A`. -/
def callWords : Int := 1015
/-- ANC-012: effect words of one chunk. -/
def chunkWords : Int := 990
/-- ANC-012: TTL per chunk, 10 x 175.5 s. -/
def ttlPerChunk : Int := 1755
/-- BAT-005: the chunk cap at every TTL. -/
def kMaxCap : Int := 2
/-- BAT-010 (FAL-SETTLEMENT-3, LEAD PICK): the one-block margin before `end_eff`, abstracted as one
time unit (any strict margin makes the landing fall inside `[start, end_eff)`). -/
def blockGap : Int := 1
/-- SLC-002, ANC-014: the anchor slot. -/
def anchorSlotConst : Int := 15
/-- SLC-003: key kind of the universal wrap key; SLC-004: of a compiled native key. -/
def keyWrap : Int := 1
def keyNative : Int := 2

/-! Refusal bits of `lastRefusal`. -/
def rNative : Int := 1       -- SLC-010: the native transcript refuses the call
def rUnbound : Int := 2      -- SLC-012: the leaf statement does not bind prog_d, input_d, outcome_d
def rOtherProgram : Int := 4 -- SLC-005: the node refuses (custom error 115), other program
def rReplay : Int := 8       -- LDG-002: claim_pub already in runs
def rMismatch : Int := 16    -- SLC-009: a recomputed public word differs, no argument file
def rNoForm : Int := 32      -- BAT-009: no L collection form in the key set
def rRegistry : Int := 64    -- LDG-008: reg_pub other than the current root
def rDupRoot : Int := 128    -- LDG-003: claims_root already in claim_roots
def rEra : Int := 256        -- BAT-011: the run cannot land before end_eff

structure St where
  deployed : Bool
  anchor : Int
  anchorSlot : Int
  anchorSealed : Bool
  entries : Int
  keyRun : Int
  keyColl : Int
  keyRoot : Int
  keyRunRead : Int
  deploys : Int
  r1 : Int
  r2 : Int
  r3 : Int
  acceptRunTx : Int
  argFiles : Int
  argsReady : Bool
  mismatches : Int
  refusedNative : Int
  routedJobs : Int
  lastRefusal : Int
  collForm : Bool
  epochOpen : Bool
  epochStart : Int
  epochJobs : Int
  now : Int
  eraEnd : Int
  collBudget : Int
  inclTime : Int
  epochsClosed : Int
  jobsClosed : Int
  collTx : Int
  blockedColl : Int
  refusedColl : Int
  pathsGiven : Int
  leafBytes : Int
  cr1 : Bool
  cr2 : Bool
  cr3 : Bool
  regRoot : Int
  intentTtl : Int
  planned : Bool
  planRefused : Bool
  lastK : Int
  runsStarted : Int
  runsRefusedEra : Int
  freezeWritten : Bool
  openListed : Bool
  deriving Repr, DecidableEq

/-- Parameters: the collection proof time and the inclusion time of BAT-010 and BAT-011
(D-077, LEAD PICK: parameters), and the current registry root. -/
def init (collBudget inclTime regRoot : Int) : St :=
  { deployed := false, anchor := 0, anchorSlot := 0, anchorSealed := false, entries := 0,
    keyRun := 0, keyColl := 0, keyRoot := 0, keyRunRead := 0, deploys := 0, r1 := 0, r2 := 0,
    r3 := 0, acceptRunTx := 0, argFiles := 0, argsReady := false, mismatches := 0,
    refusedNative := 0, routedJobs := 0, lastRefusal := 0, collForm := false, epochOpen := false,
    epochStart := 0, epochJobs := 0, now := 0, eraEnd := 0, collBudget, inclTime,
    epochsClosed := 0, jobsClosed := 0, collTx := 0, blockedColl := 0, refusedColl := 0,
    pathsGiven := 0, leafBytes := 0, cr1 := false, cr2 := false, cr3 := false, regRoot,
    intentTtl := 0, planned := false, planRefused := false, lastK := 0, runsStarted := 0,
    runsRefusedEra := 0, freezeWritten := false, openListed := false }

/-! ## Helpers -/

def runOf (s : St) (c : Int) : Int := if c = 1 then s.r1 else if c = 2 then s.r2 else s.r3
def setRun (s : St) (c v : Int) : St :=
  if c = 1 then { s with r1 := v } else if c = 2 then { s with r2 := v } else { s with r3 := v }
def rootIn (s : St) (r : Int) : Bool := if r = 1 then s.cr1 else if r = 2 then s.cr2 else s.cr3
def addRoot (s : St) (r : Int) : St :=
  if r = 1 then { s with cr1 := true } else if r = 2 then { s with cr2 := true } else { s with cr3 := true }
def b2i (b : Bool) : Int := if b then 1 else 0

/-- BAT-001, BAT-010: the close time of the open epoch: 3,600 s after its first job, or
earlier at `end_eff` minus the collection proof time, the inclusion time and one block. -/
def deadline (s : St) : Int :=
  if s.eraEnd = 0 then s.epochStart + epochLen
  else min (s.epochStart + epochLen) (s.eraEnd - s.collBudget - s.inclTime - blockGap)

/-- ANC-012 capped by BAT-005: chunks of an entry of `le` effect words. -/
def chunks (le : Int) : Int := if le ≤ callWords then 1 else (le + chunkWords - 1) / chunkWords
/-- ANC-012 formula `min(floor(TTL / 1,755 s), 2)` (owner rule K_MAX = 2, BAT-005; FAL-SETTLEMENT-2). -/
def kMax (ttl : Int) : Int := min (ttl / ttlPerChunk) kMaxCap
/-- BAT-005, ANC-012: refused when the entry needs chunks above the cap. -/
def planRefuses (le ttl : Int) : Bool := decide (callWords < le) && decide (kMax ttl < chunks le)

/-! ## Operations -/

inductive Op where
  | deploy (cp : Int)                                   -- 1
  | buildArgs (mismatch : Bool)                         -- 2
  | acceptRun (c o cp : Int) (operator native unbound differs : Bool) -- 3
  | tick (dt : Int)                                     -- 4
  | job                                                 -- 5
  | closeEpoch (root reg : Int)                         -- 6
  | announceEra (e : Int)                               -- 7
  | startRun (p : Int)                                  -- 8
  | intent (ttl : Int)                                  -- 9
  | plan (le : Int)                                     -- 10
  | addCollForm                                         -- 11
  | replayStart (eq : Bool)                             -- 12
  | freeze                                              -- 13
  deriving Repr, DecidableEq

def bit (a : Int) : Option Bool := if a = 0 then some false else if a = 1 then some true else none

/-- `code * 10^9 + arg`. acceptRun: arg = c + 10 o + 1000 cp + 10^5 operator + 10^6 native
+ 10^7 unbound + 10^8 differs (c 1..3, o 1..99, cp 0..99, flags 0/1); closeEpoch:
arg = root + 10 reg (root 1..3); the others take their one number (see ORACLE.md). -/
def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then (if 1 ≤ a ∧ a ≤ 99 then some (.deploy a) else none)
  else if c = 2 then (bit a).map .buildArgs
  else if c = 3 then
    let cl := a % 10
    let o := (a / 10) % 100
    let cp := (a / 1000) % 100
    match bit ((a / 100000) % 10), bit ((a / 1000000) % 10), bit ((a / 10000000) % 10), bit (a / 100000000) with
    | some f1, some f2, some f3, some f4 =>
      if 1 ≤ cl ∧ cl ≤ 3 ∧ 1 ≤ o then some (.acceptRun cl o cp f1 f2 f3 f4) else none
    | _, _, _, _ => none
  else if c = 4 then (if 1 ≤ a then some (.tick a) else none)
  else if c = 5 then (if a = 0 then some .job else none)
  else if c = 6 then (if 1 ≤ a % 10 ∧ a % 10 ≤ 3 then some (.closeEpoch (a % 10) (a / 10)) else none)
  else if c = 7 then (if 1 ≤ a then some (.announceEra a) else none)
  else if c = 8 then some (.startRun a)
  else if c = 9 then some (.intent a)
  else if c = 10 then (if 1 ≤ a then some (.plan a) else none)
  else if c = 11 then (if a = 0 then some .addCollForm else none)
  else if c = 12 then (bit a).map .replayStart
  else if c = 13 then (if a = 0 then some .freeze else none)
  else none

def enabled (s : St) : Op → Bool
  | .deploy cp => 1 ≤ cp
  | .buildArgs _ => s.deployed ∧ s.argsReady = false
  | .acceptRun c o _ _ _ _ _ => s.deployed ∧ s.argsReady ∧ 1 ≤ c ∧ c ≤ 3 ∧ 1 ≤ o
  | .tick dt => 1 ≤ dt ∧ (s.epochOpen = false ∨ s.now + dt ≤ deadline s)
  | .job => s.epochOpen = false ∨ (s.epochJobs < epochCap ∧ s.now < deadline s)
  | .closeEpoch r _ => s.epochOpen ∧ 1 ≤ r ∧ r ≤ 3 ∧ (deadline s ≤ s.now ∨ s.epochJobs = epochCap)
  | .announceEra e => 1 ≤ e
  | .startRun p => 0 ≤ p
  | .intent t => 0 ≤ t
  | .plan le => 1 ≤ le
  | .addCollForm => true
  | .replayStart _ => s.deployed
  | .freeze => true

/-- SLC-001 to SLC-005, STL-003, ANC-014, ANC-015: a new instance for program `cp`: four
entries, the sealed anchor `c_p` in slot 15, the universal wrap key on the two write entries,
the compiled native keys on the two read entries, empty `runs`. -/
def deployNew (s : St) (cp : Int) : St :=
  { s with deployed := true, anchor := cp, anchorSlot := anchorSlotConst, anchorSealed := true,
           entries := 4, keyRun := keyWrap, keyColl := keyWrap, keyRoot := keyNative,
           keyRunRead := keyNative, deploys := s.deploys + 1, r1 := 0, r2 := 0, r3 := 0,
           argsReady := false, lastRefusal := 0 }

/-- SLC-005, LDG-002: the node's refusal bits for a submitted `acceptRun`. -/
def nodeBad (s : St) (c cp : Int) : Int :=
  (if cp ≠ s.anchor then rOtherProgram else 0) + (if runOf s c ≠ 0 then rReplay else 0)

/-- STL-004, STL-005: a settled call sets `runs[claim_pub]` and counts a mismatch. -/
def settleRun (s : St) (c o : Int) (differs : Bool) : St :=
  let t := setRun s c o
  { t with acceptRunTx := s.acceptRunTx + 1, mismatches := s.mismatches + b2i differs, lastRefusal := 0 }

/-- SLC-005, LDG-002, STL-004, STL-005: the node's verdict on a submitted `acceptRun`. -/
def nodeAccept (s : St) (c o cp : Int) (differs : Bool) : St :=
  if nodeBad s c cp = 0 then settleRun s c o differs else { s with lastRefusal := nodeBad s c cp }

def acceptRunStep (s : St) (c o cp : Int) (operator native unbound differs : Bool) : St :=
  let s := { s with argsReady := false }
  -- BAT-002, LDG-002 (t20): an operator's job goes to the epoch collection, no acceptRun
  if operator then { s with routedJobs := s.routedJobs + 1, lastRefusal := 0 }
  -- SLC-010, SLC-012: the runner sends nothing for a natively refused or unbound call
  else if native ∨ unbound then
    { s with lastRefusal := (if native then rNative else 0) + (if unbound then rUnbound else 0),
             refusedNative := s.refusedNative + b2i native }
  else nodeAccept s c o cp differs

/-- BAT-001, BAT-007, BAT-008: close the epoch, give each submitter its slot and path,
publish the leaf list (32 bytes a claim). -/
def closeBase (s : St) : St :=
  { s with epochOpen := false, epochJobs := 0, epochsClosed := s.epochsClosed + 1,
           jobsClosed := s.jobsClosed + s.epochJobs, pathsGiven := s.pathsGiven + s.epochJobs,
           leafBytes := s.leafBytes + 32 * s.epochJobs }

/-- LDG-008, LDG-003: the node's refusal bits for a collection. -/
def collBad (s : St) (r reg : Int) : Int :=
  (if reg ≠ s.regRoot then rRegistry else 0) + (if rootIn s r then rDupRoot else 0)

/-- BAT-009, LDG-003, LDG-006, LDG-008: one collection submit for the closed epoch. -/
def closeStep (s : St) (r reg : Int) : St :=
  if s.collForm = false then { closeBase s with blockedColl := s.blockedColl + 1, lastRefusal := rNoForm }
  else if collBad s r reg = 0 then
    let t := addRoot (closeBase s) r
    { t with collTx := s.collTx + 1, lastRefusal := 0 }
  else { closeBase s with refusedColl := s.refusedColl + 1, lastRefusal := collBad s r reg }

def apply (s : St) (op : Op) : St :=
  if !enabled s op then s else
  match op with
  | .deploy cp => deployNew s cp
  | .buildArgs m =>
    -- SLC-006, SLC-009, STL-001: the builder writes the argument file unless a lane differs
    if m then { s with lastRefusal := rMismatch }
    else { s with argFiles := s.argFiles + 1, argsReady := true, lastRefusal := 0 }
  | .acceptRun c o cp f1 f2 f3 f4 => acceptRunStep s c o cp f1 f2 f3 f4
  | .tick dt => { s with now := s.now + dt }
  | .job =>
    -- BAT-001: the first job opens the epoch
    if s.epochOpen then { s with epochJobs := s.epochJobs + 1 }
    else { s with epochOpen := true, epochStart := s.now, epochJobs := 1 }
  | .closeEpoch r reg => closeStep s r reg
  | .announceEra e => { s with eraEnd := e }
  | .startRun p =>
    -- BAT-011, ERT-004, GOV-006: no run starts that cannot land before end_eff
    if s.eraEnd ≠ 0 ∧ s.eraEnd ≤ s.now + p + s.inclTime then
      { s with runsRefusedEra := s.runsRefusedEra + 1, lastRefusal := rEra }
    else { s with runsStarted := s.runsStarted + 1, lastRefusal := 0 }
  | .intent t =>
    -- BAT-006: no TTL given (0) sets 3,600 s
    { s with intentTtl := if t = 0 then ttlDefault else t }
  | .plan le =>
    -- BAT-005, ANC-012
    let ttl := if s.intentTtl = 0 then ttlDefault else s.intentTtl
    { s with planned := true, lastK := chunks le, planRefused := planRefuses le ttl }
  | .addCollForm => { s with collForm := true }
  | .replayStart eq =>
    -- SLC-011: a state that differs from the deploy-block state gets a new instance
    if eq then { s with lastRefusal := 0 } else deployNew s s.anchor
  | .freeze =>
    -- BAT-012: no collection receipt on the key set: the record lists collections as open
    { s with freezeWritten := true, openListed := !(s.collForm && decide (0 < s.collTx)) }

/-! ## Step domain -/

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
  0 ≤ s.anchor && 0 ≤ s.deploys && 0 ≤ s.r1 && 0 ≤ s.r2 && 0 ≤ s.r3 && 0 ≤ s.acceptRunTx
    && 0 ≤ s.argFiles && 0 ≤ s.epochJobs && s.epochJobs ≤ epochCap && 0 ≤ s.now && 0 ≤ s.eraEnd
    && 0 ≤ s.collBudget && 0 ≤ s.inclTime && 0 ≤ s.epochsClosed && 0 ≤ s.jobsClosed
    && 0 ≤ s.collTx && 0 ≤ s.intentTtl && 0 ≤ s.lastRefusal && s.lastRefusal < 512

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

def refusals (cb it rr : Int) (ops : List Int) : List Refusal :=
  (if 0 ≤ cb ∧ 0 ≤ it ∧ 0 ≤ rr then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (cb it rr : Int) (ops : List Int) : St := runOps (init cb it rr) ops

/-! ## Invariants -/

structure Inv (s : St) : Prop where
  /-- SLC-001, SLC-002, SLC-003, SLC-004, STL-003, ANC-014, ANC-015, LDG-001: a deployed
  instance has 4 entries, the sealed anchor `c_p` in slot 15, the wrap key on the write
  entries and native keys on the read entries. -/
  deployShape : s.deployed = true → s.entries = 4 ∧ s.anchorSlot = 15 ∧ s.anchorSealed = true ∧
    s.keyRun = keyWrap ∧ s.keyColl = keyWrap ∧ s.keyRoot = keyNative ∧ s.keyRunRead = keyNative ∧ 1 ≤ s.anchor
  /-- No argument file and no run before a deploy. -/
  undeployed : s.deployed = false → s.argsReady = false ∧ s.r1 = 0 ∧ s.r2 = 0 ∧ s.r3 = 0
  /-- STL-004: `runs` holds outcome words (0 = absent). -/
  runsNonneg : 0 ≤ s.r1 ∧ 0 ≤ s.r2 ∧ 0 ≤ s.r3
  /-- SLC-006, SLC-009: every `acceptRun` used one argument file written without a mismatch. -/
  argsUsed : s.acceptRunTx + b2i s.argsReady ≤ s.argFiles
  /-- STL-005: a mismatch is counted only for a settled call. -/
  mismatchSettled : 0 ≤ s.mismatches ∧ s.mismatches ≤ s.acceptRunTx
  /-- BAT-001, LDG-005: an open epoch holds 1 to 1,048,576 claims and ends by 3,600 s. -/
  epochBounds : s.epochOpen = true → 1 ≤ s.epochJobs ∧ s.epochJobs ≤ epochCap ∧ s.now ≤ s.epochStart + epochLen
  epochClosed : s.epochOpen = false → s.epochJobs = 0
  /-- LDG-006, BAT-001: at most one collection submit per closed epoch, and no empty epoch. -/
  onePerEpoch : s.collTx + s.blockedColl + s.refusedColl = s.epochsClosed ∧ s.epochsClosed ≤ s.jobsClosed
  /-- BAT-007: every closed claim got its slot and path; BAT-008: 32 bytes of leaf list each. -/
  membership : s.pathsGiven = s.jobsClosed ∧ s.leafBytes = 32 * s.jobsClosed
  /-- LDG-003: one accepted collection per distinct claims root. -/
  rootsDistinct : s.collTx = b2i s.cr1 + b2i s.cr2 + b2i s.cr3
  /-- BAT-009: no accepted collection without the `L` collection form. -/
  formFirst : 0 < s.collTx → s.collForm = true
  /-- BAT-005: an accepted plan has at most 2 chunks. -/
  planCap : s.planned = true → s.planRefused = false → s.lastK ≤ kMaxCap
  /-- BAT-012: a freeze record with no collection receipt lists collections as open. -/
  freezeOpen : s.freezeWritten = true → ¬(s.collForm = true ∧ 0 < s.collTx) → s.openListed = true
  counts : 0 ≤ s.acceptRunTx ∧ 0 ≤ s.epochsClosed ∧ 0 ≤ s.blockedColl ∧ 0 ≤ s.refusedColl ∧ 0 ≤ s.jobsClosed

theorem b2i_range (b : Bool) : 0 ≤ b2i b ∧ b2i b ≤ 1 := by cases b <;> decide
@[simp] theorem b2i_true : b2i true = 1 := rfl
@[simp] theorem b2i_false : b2i false = 0 := rfl

theorem inv_init (cb it rr : Int) : Inv (init cb it rr) := by
  constructor <;> simp [init, epochCap]

theorem chunks_le (le ttl : Int) (h : planRefuses le ttl = false) : chunks le ≤ kMaxCap := by
  simp [planRefuses] at h
  unfold chunks kMax at *
  by_cases h1 : le ≤ callWords
  · simp [h1, kMaxCap]
  · have := h (by unfold callWords at *; omega)
    simp [h1] at *; omega

theorem runOf_nonneg (s : St) (c : Int) (h0 : 0 ≤ s.r1 ∧ 0 ≤ s.r2 ∧ 0 ≤ s.r3) : 0 ≤ runOf s c := by
  unfold runOf; split; omega; split <;> omega

theorem collBad_root (s : St) (r reg : Int) (h : collBad s r reg = 0) : rootIn s r = false := by
  unfold collBad at h
  cases hq : rootIn s r
  · rfl
  · rw [hq] at h; simp [rDupRoot, rRegistry] at h; split at h <;> omega

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  have hb1 := b2i_range s.cr1; have hb2 := b2i_range s.cr2; have hb3 := b2i_range s.cr3
  have hba := b2i_range s.argsReady
  cases h
  constructor <;> (try dsimp only) <;>
    first
    | assumption
    | (simp_all [keyWrap, keyNative, anchorSlotConst, epochCap, epochLen, kMaxCap] <;> omega)
    | (simp_all [keyWrap, keyNative, anchorSlotConst, epochCap, epochLen, kMaxCap]; done)))

theorem inv_deployNew (cp : Int) (hc : 1 ≤ cp) : Inv (deployNew s cp) := by
  unfold deployNew; inv_tac

theorem inv_deploy (cp : Int) : Inv (apply s (.deploy cp)) := by
  by_cases he : 1 ≤ cp
  · simp only [apply, enabled, he, decide_true, Bool.not_true, Bool.false_eq_true, ite_false]
    exact inv_deployNew s h cp he
  · simp [apply, enabled, he]; exact h

theorem inv_buildArgs (m : Bool) : Inv (apply s (.buildArgs m)) := by
  by_cases he : s.deployed = true ∧ s.argsReady = false
  · have hen : enabled s (.buildArgs m) = true := by simp [enabled, he]
    unfold apply; rw [hen]; (try dsimp only)
    cases m <;> simp only [Bool.not_true, Bool.false_eq_true, ite_false, ite_true] <;> inv_tac
  · have : enabled s (.buildArgs m) = false := by unfold enabled; exact decide_eq_false he
    simp [apply, this]; exact h

theorem inv_settle (c o : Int) (d : Bool) (ha : s.argsReady = true) (_hc : 1 ≤ c ∧ c ≤ 3) (_ho : 1 ≤ o) :
    Inv (settleRun { s with argsReady := false } c o d) := by
  have hf4 := b2i_range d
  have hr := h.runsNonneg
  unfold settleRun setRun
  by_cases c1 : c = 1
  · simp only [c1, ite_true]; inv_tac
  · by_cases c2 : c = 2
    · simp only [c1, c2, ite_true, ite_false]; inv_tac
    · simp only [c1, c2, ite_false]; inv_tac

set_option maxHeartbeats 800000 in
theorem inv_acceptRun (c o cp : Int) (f1 f2 f3 f4 : Bool) : Inv (apply s (.acceptRun c o cp f1 f2 f3 f4)) := by
  by_cases he : s.deployed = true ∧ s.argsReady = true ∧ 1 ≤ c ∧ c ≤ 3 ∧ 1 ≤ o
  · have hen : enabled s (.acceptRun c o cp f1 f2 f3 f4) = true := by simp [enabled, he]
    unfold apply; rw [hen]; (try dsimp only); rw [ite_eq_right_of_eq_false _ _ (by simp)]
    unfold acceptRunStep; (try dsimp only)
    cases f1
    · cases f2 <;> cases f3
      · simp only [Bool.false_eq_true, ite_false, or_self]
        unfold nodeAccept
        by_cases hb : nodeBad { s with argsReady := false } c cp = 0
        · rw [ite_eq_left_of_eq_true _ _ (eq_true hb)]
          exact inv_settle s h c o f4 he.2.1 ⟨he.2.2.1, he.2.2.2.1⟩ he.2.2.2.2
        · rw [ite_eq_right_of_eq_false _ _ (eq_false hb)]; inv_tac
      all_goals (simp only [Bool.false_eq_true, ite_false, ite_true, or_true, true_or]; inv_tac)
    · simp only [ite_true]; inv_tac
  · have : enabled s (.acceptRun c o cp f1 f2 f3 f4) = false := by unfold enabled; exact decide_eq_false he
    simp [apply, this]; exact h

theorem inv_tick (dt : Int) : Inv (apply s (.tick dt)) := by
  by_cases he : 1 ≤ dt ∧ (s.epochOpen = false ∨ s.now + dt ≤ deadline s)
  · have hen : enabled s (.tick dt) = true := by simp only [enabled]; exact decide_eq_true he
    have hd : s.epochOpen = true → s.now + dt ≤ s.epochStart + epochLen := by
      intro ho
      have : s.now + dt ≤ deadline s := by cases he.2 <;> simp_all
      unfold deadline at this; split at this <;> omega
    unfold apply; rw [hen]; (try dsimp only); inv_tac
  · have : enabled s (.tick dt) = false := by unfold enabled; exact decide_eq_false he
    simp [apply, this]; exact h

theorem inv_job : Inv (apply s .job) := by
  by_cases he : s.epochOpen = false ∨ (s.epochJobs < epochCap ∧ s.now < deadline s)
  · have hen : enabled s .job = true := by simp only [enabled]; exact decide_eq_true he
    have hd : s.epochOpen = true → s.epochJobs < epochCap := by
      intro ho; cases he <;> simp_all
    unfold apply; rw [hen]; (try dsimp only)
    cases ho : s.epochOpen <;> simp only [ite_true, Bool.false_eq_true, ite_false] <;> inv_tac
  · have : enabled s .job = false := by unfold enabled; exact decide_eq_false he
    simp [apply, this]; exact h

theorem inv_close (r reg : Int) : Inv (apply s (.closeEpoch r reg)) := by
  by_cases he : s.epochOpen = true ∧ 1 ≤ r ∧ r ≤ 3 ∧ (deadline s ≤ s.now ∨ s.epochJobs = epochCap)
  · have hen : enabled s (.closeEpoch r reg) = true := by simp only [enabled]; exact decide_eq_true he
    have hj := h.epochBounds he.1
    unfold apply; rw [hen]; (try dsimp only); rw [ite_eq_right_of_eq_false _ _ (by simp)]
    unfold closeStep closeBase
    cases hf : s.collForm
    · simp only [Bool.false_eq_true, ite_true]; inv_tac
    · simp only [Bool.false_eq_true, ite_false]
      by_cases hb : collBad s r reg = 0
      · rw [ite_eq_left_of_eq_true _ _ (eq_true hb)]
        have hz : rootIn s r = false := collBad_root s r reg hb
        unfold addRoot rootIn at *
        by_cases r1 : r = 1
        · simp only [r1, ite_true] at hz ⊢; inv_tac
        · by_cases r2 : r = 2
          · simp only [r1, r2, ite_true, ite_false] at hz ⊢; inv_tac
          · simp only [r1, r2, ite_false] at hz ⊢; inv_tac
      · rw [ite_eq_right_of_eq_false _ _ (eq_false hb)]; inv_tac
  · have : enabled s (.closeEpoch r reg) = false := by unfold enabled; exact decide_eq_false he
    simp [apply, this]; exact h

theorem inv_announce (e : Int) : Inv (apply s (.announceEra e)) := by
  by_cases he : 1 ≤ e
  · simp [apply, enabled, he]; inv_tac
  · simp [apply, enabled, he]; exact h

theorem inv_startRun (p : Int) : Inv (apply s (.startRun p)) := by
  by_cases he : 0 ≤ p
  · have hen : enabled s (.startRun p) = true := by simp [enabled, he]
    unfold apply; rw [hen]; (try dsimp only)
    by_cases hc : s.eraEnd ≠ 0 ∧ s.eraEnd ≤ s.now + p + s.inclTime
    · rw [ite_eq_left_of_eq_true _ _ (eq_true hc)]; inv_tac
    · rw [ite_eq_right_of_eq_false _ _ (eq_false hc)]; inv_tac
  · simp [apply, enabled, he]; exact h

theorem inv_intent (t : Int) : Inv (apply s (.intent t)) := by
  by_cases he : 0 ≤ t
  · simp [apply, enabled, he]; inv_tac
  · simp [apply, enabled, he]; exact h

theorem inv_plan (le : Int) : Inv (apply s (.plan le)) := by
  by_cases he : 1 ≤ le
  · have hen : enabled s (.plan le) = true := by simp [enabled, he]
    unfold apply; rw [hen]; (try dsimp only)
    have hk := chunks_le le (if s.intentTtl = 0 then ttlDefault else s.intentTtl)
    cases h
    constructor <;> simp_all [kMaxCap]
  · simp [apply, enabled, he]; exact h

theorem inv_addCollForm : Inv (apply s .addCollForm) := by simp [apply, enabled]; inv_tac

theorem inv_replay (eq : Bool) : Inv (apply s (.replayStart eq)) := by
  by_cases he : s.deployed = true
  · have hen : enabled s (.replayStart eq) = true := by simp [enabled, he]
    unfold apply; rw [hen]; (try dsimp only)
    cases eq
    · simp only [Bool.false_eq_true, ite_false]
      exact inv_deployNew s h s.anchor (h.deployShape he).2.2.2.2.2.2.2
    · simp only [ite_true]; inv_tac
  · simp [apply, enabled, he]; exact h

theorem inv_freeze : Inv (apply s .freeze) := by
  simp only [apply, enabled, Bool.not_true, Bool.false_eq_true, ite_false]
  cases h
  constructor <;> (try dsimp only) <;> first
    | assumption
    | (intro _ hn; cases hc : s.collForm <;> simp_all)
end

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases op with
  | deploy cp => exact inv_deploy s h cp
  | buildArgs m => exact inv_buildArgs s h m
  | acceptRun c o cp f1 f2 f3 f4 => exact inv_acceptRun s h c o cp f1 f2 f3 f4
  | tick dt => exact inv_tick s h dt
  | job => exact inv_job s h
  | closeEpoch r reg => exact inv_close s h r reg
  | announceEra e => exact inv_announce s h e
  | startRun p => exact inv_startRun s h p
  | intent t => exact inv_intent s h t
  | plan le => exact inv_plan s h le
  | addCollForm => exact inv_addCollForm s h
  | replayStart eq => exact inv_replay s h eq
  | freeze => exact inv_freeze s h

theorem inv_stepOp (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp; split
  · exact h
  · exact inv_apply s _ h

theorem inv_runOps (s : St) (ns : List Int) (h : Inv s) : Inv (runOps s ns) := by
  induction ns generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_stepOp s n h)

theorem inv_run (cb it rr : Int) (ns : List Int) : Inv (run cb it rr ns) := inv_runOps _ _ (inv_init _ _ _)

/-! ## Requirement-level statements -/

/-- SLC-001, SLC-003, SLC-004, STL-003: shape of every deployed instance. -/
theorem SLC_001_004_deploy_shape (cb it rr : Int) (ns : List Int) (h : (run cb it rr ns).deployed = true) :
    (run cb it rr ns).entries = 4 ∧ (run cb it rr ns).keyRun = keyWrap ∧ (run cb it rr ns).keyColl = keyWrap ∧
    (run cb it rr ns).keyRoot = keyNative ∧ (run cb it rr ns).keyRunRead = keyNative := by
  have := (inv_run cb it rr ns).deployShape h; omega

/-- SLC-002, ANC-014, ANC-015: the anchor is sealed in slot 15. -/
theorem SLC_002_anchor_slot (cb it rr : Int) (ns : List Int) (h : (run cb it rr ns).deployed = true) :
    (run cb it rr ns).anchorSlot = 15 ∧ (run cb it rr ns).anchorSealed = true := by
  have := (inv_run cb it rr ns).deployShape h; exact ⟨this.2.1, this.2.2.1⟩

/-- SLC-002, ANC-015: only a deploy writes the anchor (no entry writes slot 15). -/
theorem SLC_002_anchor_constructor_only (s : St) (op : Op) (hd : ∀ cp, op ≠ .deploy cp)
    (hr : op ≠ .replayStart false) : (apply s op).anchor = s.anchor := by
  unfold apply; split
  · rfl
  · cases op with
    | deploy cp => exact absurd rfl (hd cp)
    | replayStart eq => cases eq; exact absurd rfl hr; simp
    | acceptRun c o cp f1 f2 f3 f4 =>
      simp only [acceptRunStep, nodeAccept, settleRun, setRun]; (repeat' split) <;> rfl
    | closeEpoch r reg => simp only [closeStep, closeBase, addRoot]; (repeat' split) <;> rfl
    | _ => simp only <;> (repeat' split) <;> rfl

/-- BAT-002, LDG-002 (t20): an operator's job never adds an `acceptRun`. -/
theorem BAT_002_operator_no_acceptRun (s : St) (c o cp : Int) (f2 f3 f4 : Bool) :
    (apply s (.acceptRun c o cp true f2 f3 f4)).acceptRunTx = s.acceptRunTx := by
  unfold apply; split
  · rfl
  · simp [acceptRunStep]

/-- SLC-010: a natively refused call sends no `acceptRun`. -/
theorem SLC_010_native_refused (s : St) (c o cp : Int) (f3 f4 : Bool) :
    (apply s (.acceptRun c o cp false true f3 f4)).acceptRunTx = s.acceptRunTx := by
  unfold apply; split
  · rfl
  · simp [acceptRunStep]

/-- SLC-012: no live settle of an unbound statement. -/
theorem SLC_012_unbound_refused (s : St) (c o cp : Int) (f2 f4 : Bool) :
    (apply s (.acceptRun c o cp false f2 true f4)).acceptRunTx = s.acceptRunTx := by
  unfold apply; split
  · rfl
  · cases f2 <;> simp [acceptRunStep]

/-- SLC-005: program B does not settle on the ledger of program A. -/
theorem SLC_005_other_program (s : St) (c o cp : Int) (f4 : Bool) (hcp : cp ≠ s.anchor) :
    (apply s (.acceptRun c o cp false false false f4)).acceptRunTx = s.acceptRunTx := by
  unfold apply; split
  · rfl
  · have hb : nodeBad { s with argsReady := false } c cp ≠ 0 := by
      unfold nodeBad; simp only [ne_eq, hcp, not_false_eq_true, ite_true, rOtherProgram, rReplay]; split <;> omega
    simp only [acceptRunStep, Bool.false_eq_true, ite_false, or_self, nodeAccept]
    rw [ite_eq_right_of_eq_false _ _ (eq_false hb)]

/-- LDG-002: a `claim_pub` already in `runs` is refused and keeps its outcome. -/
theorem LDG_002_at_most_once (s : St) (c o cp : Int) (f4 : Bool) (hr : runOf s c ≠ 0) :
    (apply s (.acceptRun c o cp false false false f4)).acceptRunTx = s.acceptRunTx ∧
    runOf (apply s (.acceptRun c o cp false false false f4)) c = runOf s c := by
  unfold apply; split
  · exact ⟨rfl, rfl⟩
  · have hr' : runOf { s with argsReady := false } c ≠ 0 := by simpa [runOf] using hr
    have hb : nodeBad { s with argsReady := false } c cp ≠ 0 := by
      unfold nodeBad; simp only [ne_eq, hr', not_false_eq_true, ite_true, rOtherProgram, rReplay]; split <;> omega
    simp only [acceptRunStep, Bool.false_eq_true, ite_false, or_self, nodeAccept]
    rw [ite_eq_right_of_eq_false _ _ (eq_false hb)]
    exact ⟨rfl, by simp [runOf]⟩

/-- STL-004: a settled call sets `runs[claim_pub]` to `outcome_pub`. -/
theorem STL_004_runs_set (s : St) (c o : Int) (f4 : Bool)
    (he : enabled s (.acceptRun c o s.anchor false false false f4) = true) (hr : runOf s c = 0) :
    runOf (apply s (.acceptRun c o s.anchor false false false f4)) c = o := by
  have hb : nodeBad { s with argsReady := false } c s.anchor = 0 := by
    have hr' : runOf { s with argsReady := false } c = 0 := by simpa [runOf] using hr
    unfold nodeBad; simp [hr']
  simp only [apply, he, Bool.not_true, Bool.false_eq_true, ite_false, acceptRunStep, or_self, nodeAccept]
  rw [ite_eq_left_of_eq_true _ _ (eq_true hb)]
  unfold settleRun setRun runOf
  by_cases c1 : c = 1
  · simp [c1]
  · by_cases c2 : c = 2
    · simp [c2]
    · simp [c1, c2]

/-- BAT-011: no run starts that cannot land before `end_eff`. -/
theorem BAT_011_era_refusal (s : St) (p : Int) (hp : 0 ≤ p) (he : s.eraEnd ≠ 0) (hl : s.eraEnd ≤ s.now + p + s.inclTime) :
    (apply s (.startRun p)).runsStarted = s.runsStarted ∧ (apply s (.startRun p)).lastRefusal = rEra := by
  simp [apply, enabled, hp, he, hl]

/-- BAT-006: no TTL given sets 3,600 s. -/
theorem BAT_006_default_ttl (s : St) : (apply s (.intent 0)).intentTtl = 3600 := by
  simp [apply, enabled, ttlDefault]

/-- BAT-010: while an era end is announced, the clock never passes `end_eff` minus the
collection proof time and the inclusion time with an epoch open. -/
theorem BAT_010_close_before_era (s : St) (dt : Int) (ho : s.epochOpen = true) (he : s.eraEnd ≠ 0)
    (hen : enabled s (.tick dt) = true) :
    (apply s (.tick dt)).now ≤ s.eraEnd - s.collBudget - s.inclTime - blockGap := by
  simp [enabled, ho] at hen
  simp [apply, enabled, ho, hen]
  have := hen.2; unfold deadline at this; simp [he] at this; omega

/-- LDG-006, BAT-001: at most one ledger write per closed epoch, none for an empty epoch. -/
theorem LDG_006_one_write_per_epoch (cb it rr : Int) (ns : List Int) :
    (run cb it rr ns).collTx ≤ (run cb it rr ns).epochsClosed ∧
    (run cb it rr ns).epochsClosed ≤ (run cb it rr ns).jobsClosed := by
  have := (inv_run cb it rr ns).onePerEpoch; have := (inv_run cb it rr ns).counts; omega

/-- BAT-005: an accepted plan never has more than 2 chunks, at every TTL. -/
theorem BAT_005_cap (le ttl : Int) : planRefuses le ttl = false → chunks le ≤ 2 := chunks_le le ttl

/-! ## Constants checked against MEASURED and DERIVED values -/

/-- ANC-012: `L_TX - A` = 1,024 - 9 = 1,015 effect words. -/
theorem anc012_call_words : (1024 : Int) - 9 = callWords := by decide
/-- ANC-012: largest checked call 474 words and largest static count 699 words fit one call
(`t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:116`). -/
theorem measured_largest_calls : (474 : Int) ≤ callWords ∧ (699 : Int) ≤ callWords ∧ chunks 474 = 1 ∧ chunks 699 = 1 := by
  decide
/-- ANC-012, BAT-005: `floor(3,600 / 1,755)` = 2 at the default TTL
(`t20-batched@5dc1be2ee:...RESULTS.md:122`); 1,755 s = 10 x 175.5 s (`RESULTS.md:13`). -/
theorem kmax_default_ttl : kMax 3600 = 2 ∧ (10 * 1755 : Int) = 17550 ∧ 3600 / ttlPerChunk = 2 := by decide
/-- BAT-005: at TTL 14,400 s the ANC-012 formula gives 8, the cap keeps 2 (FAL-SETTLEMENT-2). -/
theorem kmax_long_ttl : min (14400 / ttlPerChunk) 18 = 8 ∧ kMax 14400 = 2 := by decide
/-- ANC-012: below 1,755 s no chunking: `K_MAX` = 0. -/
theorem kmax_short_ttl : kMax 1754 = 0 ∧ kMax 1755 = 1 := by decide
/-- BAT-005: a 4-chunk entry (2,971 words) is refused at 14,400 s; a 2-chunk entry
(1,980 words) passes at 3,600 s; 1,981 words need 3 chunks. -/
theorem bat005_pins : planRefuses 2971 14400 = true ∧ planRefuses 1980 3600 = false ∧ chunks 1981 = 3 := by decide
/-- BAT-001, LDG-005: depth 20 gives 2^20 = 1,048,576 claims. -/
theorem epoch_cap_depth20 : epochCap = 2 ^ 20 := by decide
/-- BAT-003: one prover feeds 86,400 / 175.5 = 492 calls a day (`RESULTS.md:14`); a 3,600 s
epoch gives at most 24 transactions a day (`RESULTS.md:56`); 24 x 6,797 = 163,128 bytes
(about 163 KB, ESTIMATE 6,797); 492 x 6,783 = 3,337,236 bytes (3.34 MB, MEASURED 6,783,
`RESULTS.md:11`); 2^20 / 3,600 = 291 jobs a second (`RESULTS.md:54`). -/
theorem bat003_rates : (864000 : Int) / 1755 = 492 ∧ (86400 : Int) / epochLen = 24 ∧
    (24 * 6797 : Int) = 163128 ∧ (492 * 6783 : Int) = 3337236 ∧ (2 ^ 20 : Int) / 3600 = 291 := by decide
/-- BAT-004: 208 calls, 62 refused as native, 1 pure: 145 `acceptRun` transactions
(`/home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md:45`);
at most 106 deploys (one ledger per circuit, SLC-005). -/
theorem bat004_counts : (208 - 62 - 1 : Int) = 145 ∧ (106 : Int) ≤ 145 := by decide

end Settlement
