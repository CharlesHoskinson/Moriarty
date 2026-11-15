/-!
# Sherlock Execution oracle (bounded context Sherlock Execution, slice A)

Executable reference semantics of `sherlock-e2e-universal-interpreter` (EUI),
`sherlock-e2e-rsl-kits` (EKT) and `sherlock-e2e-rsl-statement` (ERS), and of the
adopted requirements that constrain one call of one program: the call record,
admission (`prog_d`), the tape check, execution under one rule family, the
statement, the app proof, the entry into recursion and the private/public split.
Spec text only; no code was read to write it.

Phases of one call:

| phase | meaning |
|---|---|
| 0 | idle |
| 1 | loaded: the call record decoded (ERS-001, ERS-014) |
| 2 | admitted: rule family and `prog_d` fixed (EUI-001, ERS-006, ERS-015, EKT-004) |
| 3 | tape checked (EUI-007) |
| 4 | executed: one outcome (EUI-004, EUI-005) |
| 5 | statement built (ERS-007, ERS-008, ERS-010) |
| 6 | app proof accepted (EKT-007) |
| 7 | stopped by a refusal (`stop` names it) |
| 8 | closed with no statement: an R_SL call rejected (ERS-009) |
| 9 | in recursion: the outcome commitment is bound (ERS-012) |

Digests are abstract injective encodings over small domains (`progDigest`,
`nativeComm`, `nativeW`, `valDigest`, `staticOps`); ORACLE.md gives the formulas
so a replay can compare them or leave them out.
-/

namespace SherlockExec

def opBase : Int := 1000000000

/-! ## Abstract digests -/

/-- ERS-006, MYC-CLM-023, MYC-KEY-003, INIT-007:
`prog_d = H(DOM_PROG, image_d, plan_d, key_root, machine_version)`. -/
def progDigest (image plan keyRoot mv : Int) : Int := image + 1000 * plan + 10000 * keyRoot + 100000 * mv
/-- The native reference `comm` of a call (MYC-LOW-009, MYC-TRN-014), abstract. -/
def nativeComm (image tapeD : Int) : Int := 2 * image + 3 * tapeD + 1
/-- The native reference effect words W of a call (CF-003, MYC-LOW-010), abstract digest. -/
def nativeW (image tapeD : Int) : Int := 5 * image + 7 * tapeD + 2
/-- ERS-008: the value digest of the list `comm` followed by W. -/
def valDigest (comm w : Int) : Int := 1000003 * comm + w
/-- EUI-008, CF-004, MYC-LOW-012: the static micro-op count of an entry (a function of
the image only, never of the tape). -/
def staticOps (image : Int) : Int := image + 10

/-! ## Codes recorded in the state -/

def stopNone : Int := 0
def stopMalformed : Int := 1   -- ERS-014 `Refuse::Unsupported` with the field name
def stopMixedHash : Int := 2   -- EKT-004 "persistentHash and keccak256 in one program"
def stopBadTape : Int := 3     -- EUI-007, MYC-REF-036, MYC-REF-038 status `refused`
def stopVersion : Int := 4     -- EUI-001 (LEAD PICK): a machine version naming no family

def refNone : Int := 0
def refKit : Int := 1          -- EKT-007 the app verifier refuses the proof
def refUnbound : Int := 2      -- ERS-012 the outcome commitment is not the statement's
def refPrivate : Int := 3      -- ERS-002 a private record leaves the owned boxes

/-- Outcome kinds: 0 none, 1 Halted, 2 Rejected, 3 Exhausted (EUI-004, EUI-005, GP-001). -/
def outHalted : Int := 1
def outRejected : Int := 2
def outExhausted : Int := 3

/-- Family codes: 1 interpreted (120 step rules), 2 straight-line `R_SL` (EUI-001). -/
def famInterp : Int := 1
def famRsl : Int := 2

structure St where
  phase : Int
  stop : Int
  lastRefusal : Int
  refusedField : Int
  family : Int
  machineVersion : Int
  image : Int
  plan : Int
  keyRoot : Int
  progD : Int
  hasAppKey : Bool
  appKeyCommitted : Bool
  usesSha : Bool
  usesKeccak : Bool
  tapeChecked : Bool
  tapeD : Int
  outcome : Int
  aluOverflow : Bool
  nW : Int
  microOps : Int
  comm : Int
  wD : Int
  publicRows : Int
  box : Int
  stmtCount : Int
  outcomeKind : Int
  outcomePayload : Int
  inputVal : Int
  proofsStarted : Int
  kitOk : Bool
  proofAccepted : Bool
  boundOutcome : Int
  publicsOut : Int
  privateLeaks : Int
  ukey : Int
  deriving Repr, DecidableEq

/-- The start state; `ukey` names the one universal key set (EUI-002, CPR-003). -/
def init (ukey : Int) : St :=
  { phase := 0, stop := 0, lastRefusal := 0, refusedField := 0, family := 0, machineVersion := 0,
    image := 0, plan := 0, keyRoot := 0, progD := 0, hasAppKey := false, appKeyCommitted := false,
    usesSha := false, usesKeccak := false, tapeChecked := false, tapeD := 0, outcome := 0,
    aluOverflow := false, nW := 0, microOps := 0, comm := 0, wD := 0, publicRows := 0, box := 0,
    stmtCount := 0, outcomeKind := 0, outcomePayload := 0, inputVal := 0, proofsStarted := 0,
    kitOk := false, proofAccepted := false, boundOutcome := 0, publicsOut := 0, privateLeaks := 0,
    ukey }

/-! ## Operations -/

inductive Op where
  /-- 1: load a call record; `kind` 0 well formed, 1 field `idx` missing, 2 field `idx` of a wrong type -/
  | load (kind idx : Int)
  /-- 2: the planner admits the program -/
  | admit (image plan keyRoot mv : Int) (appKey sha keccak : Bool)
  /-- 3: the prover wrapper checks the tape; `flags` bit 0 wrong selector, 1 non-canonical
  word, 2 missing word, 3 leftover word -/
  | tape (flags tapeD : Int)
  /-- 4: the executor runs the call: `kind` 1 Halted, 2 Rejected, 3 Exhausted; `alu` an exact
  ALU result at or above its modulus; `nW` effect words; `box` the executing box -/
  | execute (kind : Int) (alu : Bool) (nW box : Int)
  /-- 5: the statement builder -/
  | build
  /-- 6: prove the app; `kitOk` false is a trace that violates a kit relation -/
  | prove (kitOk : Bool)
  /-- 7: the statement adapter offers this outcome commitment to recursion -/
  | recurse (offered : Int)
  /-- 8: a record leaves to a box: `pub` false the call record or `RslStatement`, true the
  `StatementPublics`; `owned` the target box is owned by the user -/
  | send (pub owned : Bool)
  /-- 9: start the next call -/
  | newCall
  deriving Repr, DecidableEq

def bit (n : Int) : Option Bool := if n = 0 then some false else if n = 1 then some true else none

def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then some (.load (a / 100) (a % 100))
  else if c = 2 then
    match bit ((a / 1000000) % 10), bit ((a / 10000000) % 10), bit (a / 100000000) with
    | some k, some sh, some kc =>
      some (.admit (a % 1000) ((a / 1000) % 10) ((a / 10000) % 10) ((a / 100000) % 10) k sh kc)
    | _, _, _ => none
  else if c = 3 then some (.tape (a % 16) (a / 16))
  else if c = 4 then
    match bit ((a / 10) % 10) with
    | some alu => some (.execute (a % 10) alu ((a / 100) % 10000) (a / 1000000))
    | none => none
  else if c = 5 then (if a = 0 then some .build else none)
  else if c = 6 then (bit a).map Op.prove
  else if c = 7 then some (.recurse a)
  else if c = 8 then
    match bit (a / 10), bit (a % 10) with
    | some p, some o => some (.send p o)
    | _, _ => none
  else if c = 9 then (if a = 0 then some .newCall else none)
  else none

/-- When each operation is enabled. -/
def enabled (s : St) : Op → Bool
  | .load kind idx => s.phase = 0 ∧ ((kind = 0 ∧ idx = 0) ∨ ((kind = 1 ∨ kind = 2) ∧ 1 ≤ idx ∧ idx ≤ 14))
  | .admit .. => s.phase = 1
  | .tape .. => s.phase = 2
  | .execute kind alu _ _ => s.phase = 3 ∧ (kind = outHalted ∨ kind = outRejected ∨
      (kind = outExhausted ∧ s.family = famInterp)) ∧ (alu = false ∨ s.family = famRsl)
  | .build => s.phase = 4
  | .prove _ => s.phase = 5
  | .recurse _ => s.phase = 6
  | .send p _ => 1 ≤ s.phase ∧ (p = false ∨ s.stmtCount = 1)
  | .newCall => s.phase = 7 ∨ s.phase = 8 ∨ s.phase = 9

def stopped (s : St) (code : Int) : St := { s with phase := 7, stop := code }

/-- The response of an enabled operation. -/
def applyCore (s : St) : Op → St
  | .load kind idx =>
    -- ERS-014: a missing field or a field of a wrong type is `Refuse::Unsupported` naming it
    if kind = 0 then { s with phase := 1 } else stopped { s with refusedField := idx } stopMalformed
  | .admit image plan keyRoot mv appKey sha keccak =>
    let s' := { s with image, plan, keyRoot, machineVersion := mv, hasAppKey := appKey,
                       usesSha := sha, usesKeccak := keccak }
    -- EUI-001 (LEAD PICK): the machine version names exactly one family
    if ¬(mv = famInterp ∨ mv = famRsl) then stopped s' stopVersion
    -- EKT-004: an R_SL program that calls both hashes is refused by name
    else if mv = famRsl ∧ sha = true ∧ keccak = true then stopped s' stopMixedHash
    -- ERS-006, MYC-CLM-023: prog_d over image, plan, key_root, machine version;
    -- ERS-015, MYC-KEY-006: a per-program app key is committed under key_root
    else { s' with phase := 2, family := mv, progD := progDigest image plan keyRoot mv,
                   appKeyCommitted := appKey }
  | .tape flags d =>
    -- EUI-007, MYC-REF-036, MYC-REF-038, MYC-TAPE-006, MYC-TAPE-007: a bad tape starts no proof
    if flags ≠ 0 then stopped s stopBadTape
    else { s with phase := 3, tapeChecked := true, tapeD := d }
  | .execute kind alu nW box =>
    -- EKT-011, MYC-REF-037, MYC-LOW-005: a failed check rejects the call
    let o := if alu then outRejected else kind
    -- EUI-008, CF-004: the micro-op count is a function of the entry, not of the tape
    let s' := { s with phase := 4, outcome := o, aluOverflow := alu, microOps := staticOps s.image, box }
    -- ERS-005, EUI-006, CF-003, MYC-LOW-010: comm and W equal the native reference;
    -- ERS-003: one PUBLIC row for comm, then one per effect word
    if o = outHalted then
      { s' with comm := nativeComm s.image s.tapeD, wD := nativeW s.image s.tapeD, nW,
                publicRows := 1 + nW }
    else s'
  | .build =>
    -- ERS-009, MYC-LOW-007: a rejected R_SL (Compact) call has no statement
    if s.family = famRsl ∧ s.outcome = outRejected then { s with phase := 8 }
    -- ERS-010: one statement; ERS-007: input_d binds the tape's value digest;
    -- ERS-008: a halted call has outcome Halted with payload vd(comm ++ W)
    else { s with phase := 5, stmtCount := 1, outcomeKind := s.outcome, inputVal := s.tapeD,
                  outcomePayload := if s.outcome = outHalted then valDigest s.comm s.wD else s.outcome }
  | .prove ok =>
    -- EKT-007: a trace that violates a kit relation is refused by the app verifier
    let s' := { s with proofsStarted := s.proofsStarted + 1, kitOk := ok }
    if ok then { s' with phase := 6, proofAccepted := true, lastRefusal := refNone }
    else { s' with lastRefusal := refKit }
  | .recurse offered =>
    -- ERS-012: recursion consumes exactly the statement's outcome commitment
    if offered = s.outcomePayload then { s with phase := 9, boundOutcome := offered, lastRefusal := refNone }
    else { s with lastRefusal := refUnbound }
  | .send p owned =>
    -- ERS-002, MYC-CLM-009: a private record stays on owned boxes; only StatementPublics leave
    if p then { s with publicsOut := s.publicsOut + 1, lastRefusal := refNone }
    else if owned then { s with lastRefusal := refNone }
    else { s with lastRefusal := refPrivate }
  | .newCall => init s.ukey |> fun t => { t with privateLeaks := s.privateLeaks }

/-- One operation; a disabled operation leaves the state unchanged (`--step` refuses it). -/
def apply (s : St) (op : Op) : St := if enabled s op then applyCore s op else s

theorem apply_of_enabled {s : St} {op : Op} (he : enabled s op = true) : apply s op = applyCore s op := by
  simp [apply, he]

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
  0 ≤ s.phase && s.phase ≤ 9 && 0 ≤ s.stop && s.stop ≤ 4 && 0 ≤ s.lastRefusal && s.lastRefusal ≤ 3
    && 0 ≤ s.family && s.family ≤ 2 && 0 ≤ s.outcome && s.outcome ≤ 3 && 0 ≤ s.stmtCount
    && s.stmtCount ≤ 1 && 0 ≤ s.proofsStarted && 0 ≤ s.publicsOut && 0 ≤ s.privateLeaks && 0 ≤ s.nW

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

def refusals (ukey : Int) (ops : List Int) : List Refusal :=
  (if 0 ≤ ukey then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (ukey : Int) (ops : List Int) : St := runOps (init ukey) ops

/-! ## Invariants -/

structure Inv (s : St) : Prop where
  phaseRange : 0 ≤ s.phase ∧ s.phase ≤ 9
  stopIff : s.phase = 7 ↔ s.stop ≠ 0
  stopRange : 0 ≤ s.stop ∧ s.stop ≤ 4
  /-- EUI-001, ERS-006, ERS-015, EKT-004, MYC-CLM-023, MYC-KEY-003, MYC-KEY-006, INIT-007:
  an admitted program runs under the one family its machine version names, has the
  `prog_d` of the identity formula, commits its app key under `key_root`, and is not an
  R_SL program with both hashes. -/
  admitted : s.family = 0 ∨ (s.family = s.machineVersion ∧ (s.family = famInterp ∨ s.family = famRsl)
    ∧ s.progD = progDigest s.image s.plan s.keyRoot s.machineVersion
    ∧ (s.hasAppKey = true → s.appKeyCommitted = true)
    ∧ ¬(s.family = famRsl ∧ s.usesSha = true ∧ s.usesKeccak = true))
  pFamily0 : s.phase ≤ 1 → s.family = 0
  pFamily : 2 ≤ s.phase ∧ s.phase ≠ 7 → s.family ≠ 0
  pTape : (3 ≤ s.phase ∧ s.phase ≠ 7) → s.tapeChecked = true
  pNoOutcome : (s.phase ≤ 3 ∨ s.phase = 7) → s.outcome = 0 ∧ s.stmtCount = 0 ∧ s.proofsStarted = 0
    ∧ s.proofAccepted = false ∧ s.publicsOut = 0
  pOutcome : (4 ≤ s.phase ∧ s.phase ≠ 7) → s.outcome ≠ 0
  pNoStmt : s.phase = 4 → s.stmtCount = 0 ∧ s.proofsStarted = 0 ∧ s.proofAccepted = false ∧ s.publicsOut = 0
  pStmt : (s.phase = 5 ∨ s.phase = 6 ∨ s.phase = 9) → s.stmtCount = 1
  pAccepted : (s.phase = 6 ∨ s.phase = 9) → s.proofAccepted = true
  pNotAccepted : s.phase = 5 → s.proofAccepted = false
  /-- EUI-004, EUI-005, GP-001: one outcome kind; an R_SL call never ends Exhausted. -/
  outcomeRange : 0 ≤ s.outcome ∧ s.outcome ≤ 3 ∧ (s.outcome = outExhausted → s.family = famInterp)
  /-- EKT-011: an ALU result at or above its modulus rejects the call. -/
  aluRejects : s.aluOverflow = true → s.outcome = outRejected
  /-- ERS-005, EUI-006, CF-003, MYC-LOW-009, MYC-LOW-010, ERS-003: a halted call gives the
  native `comm` and W and one PUBLIC row per statement word. -/
  native : s.outcome = outHalted → s.comm = nativeComm s.image s.tapeD ∧ s.wD = nativeW s.image s.tapeD
    ∧ s.publicRows = 1 + s.nW
  /-- EUI-008, CF-004, MYC-LOW-012: the step count is static. -/
  static : s.outcome ≠ 0 → s.microOps = staticOps s.image
  /-- ERS-009, MYC-LOW-007: a rejected R_SL call has no statement and no proof. -/
  rejectedNoStmt : s.phase = 8 → s.family = famRsl ∧ s.outcome = outRejected ∧ s.stmtCount = 0
    ∧ s.proofsStarted = 0 ∧ s.publicsOut = 0
  stmtNotRejectedRsl : s.stmtCount = 1 → ¬(s.family = famRsl ∧ s.outcome = outRejected)
  /-- ERS-007, ERS-008, ERS-010: at most one statement; it binds the tape digest and, for a
  halted call, the outcome Halted with payload vd(comm ++ W). -/
  stmt : 0 ≤ s.stmtCount ∧ s.stmtCount ≤ 1 ∧ (s.stmtCount = 1 → s.outcome ≠ 0
    ∧ s.outcomeKind = s.outcome ∧ s.inputVal = s.tapeD
    ∧ (s.outcome = outHalted → s.outcomePayload = valDigest s.comm s.wD))
  /-- EUI-007: no proof starts without a checked tape and a statement. -/
  proofNeedsTape : 0 < s.proofsStarted → s.tapeChecked = true ∧ s.stmtCount = 1
  proofsNonneg : 0 ≤ s.proofsStarted
  /-- EKT-007: an accepted app proof had no kit-relation violation. -/
  kitAccepted : s.proofAccepted = true → s.kitOk = true
  /-- ERS-012: recursion holds exactly the statement's outcome commitment. -/
  bound : s.phase = 9 → s.boundOutcome = s.outcomePayload
  /-- ERS-002, MYC-CLM-009: no private record ever left the owned boxes. -/
  noLeak : s.privateLeaks = 0
  /-- ERS-002: only the StatementPublics of a built statement leave. -/
  publicsNeedStmt : 0 < s.publicsOut → s.stmtCount = 1
  publicsNonneg : 0 ≤ s.publicsOut

theorem inv_init (u : Int) : Inv (init u) := by
  constructor <;> simp [init, famInterp, famRsl, outExhausted, outRejected, outHalted]

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  cases h
  constructor <;> simp_all [stopNone, stopMalformed, stopMixedHash, stopBadTape, stopVersion,
      refNone, refKit, refUnbound, refPrivate, outHalted, outRejected, outExhausted,
      famInterp, famRsl, init] <;> omega))

set_option linter.unusedSimpArgs false

theorem inv_load (k i : Int) (he : enabled s (.load k i) = true) : Inv (apply s (.load k i)) := by
  have hp : s.phase = 0 := by simp [enabled] at he; exact he.1
  rw [apply_of_enabled he]; clear he
  by_cases hc : k = 0
  · simp only [applyCore, hc, ite_true]; inv_tac
  · simp only [applyCore, hc, ite_false, stopped]; inv_tac

theorem inv_admit (i p k m : Int) (a sh kc : Bool) (hp : s.phase = 1) :
    Inv (apply s (.admit i p k m a sh kc)) := by
  rw [apply_of_enabled (by simp [enabled, hp])]
  by_cases h1 : ¬(m = famInterp ∨ m = famRsl)
  · simp only [applyCore, h1, ite_true, stopped]; inv_tac
  · by_cases h2 : m = famRsl ∧ sh = true ∧ kc = true
    · simp only [applyCore, h1, h2, ite_false, ite_true, stopped, and_self]; inv_tac
    · simp only [applyCore, h1, h2, ite_false]; inv_tac

theorem inv_tape (f d : Int) (hp : s.phase = 2) : Inv (apply s (.tape f d)) := by
  rw [apply_of_enabled (by simp [enabled, hp])]
  by_cases hc : f ≠ 0
  · simp only [applyCore, hc, ite_true, stopped]; inv_tac
  · simp only [applyCore, hc, ite_false]; inv_tac

theorem inv_execute (k : Int) (a : Bool) (n b : Int) (he : enabled s (.execute k a n b) = true) :
    Inv (apply s (.execute k a n b)) := by
  cases a
  · have ⟨hp, hk⟩ : s.phase = 3 ∧ (k = 1 ∨ k = 2 ∨ k = 3 ∧ s.family = 1) := by
      simpa [enabled, outHalted, outRejected, outExhausted, famInterp] using he
    rw [apply_of_enabled he]; clear he
    by_cases hc : k = outHalted
    · simp only [applyCore, hc, ite_true, Bool.false_eq_true, ite_false]; inv_tac
    · simp only [applyCore, hc, ite_false, Bool.false_eq_true]; inv_tac
  · have ⟨hp, hk, hf⟩ : s.phase = 3 ∧ (k = 1 ∨ k = 2 ∨ k = 3 ∧ s.family = 1) ∧ s.family = 2 := by
      simpa [enabled, outHalted, outRejected, outExhausted, famInterp, famRsl] using he
    rw [apply_of_enabled he]; clear he
    simp [applyCore, outRejected, outHalted]; inv_tac

theorem inv_build (hp : s.phase = 4) : Inv (apply s .build) := by
  rw [apply_of_enabled (by simp [enabled, hp])]
  by_cases hc : s.family = famRsl ∧ s.outcome = outRejected
  · simp only [applyCore, hc, and_self, ite_true]; inv_tac
  · by_cases ho : s.outcome = outHalted
    · simp only [applyCore, hc, ho, ite_false, ite_true]; inv_tac
    · simp only [applyCore, hc, ho, ite_false]; inv_tac

theorem inv_prove (ok : Bool) (hp : s.phase = 5) : Inv (apply s (.prove ok)) := by
  rw [apply_of_enabled (by simp [enabled, hp])]
  cases ok
  · simp only [applyCore, ite_false, Bool.false_eq_true]; inv_tac
  · simp only [applyCore, ite_true]; inv_tac

theorem inv_recurse (x : Int) (hp : s.phase = 6) : Inv (apply s (.recurse x)) := by
  rw [apply_of_enabled (by simp [enabled, hp])]
  by_cases hc : x = s.outcomePayload
  · simp only [applyCore, hc, ite_true]; inv_tac
  · simp only [applyCore, hc, ite_false]; inv_tac

set_option maxHeartbeats 2000000 in
theorem inv_send (p o : Bool) (he : enabled s (.send p o) = true) : Inv (apply s (.send p o)) := by
  have he' := he
  simp only [enabled, decide_eq_true_eq] at he'
  obtain ⟨hp, hs⟩ := he'
  rw [apply_of_enabled he]; clear he
  cases p <;> cases o <;> simp only [applyCore, ite_false, ite_true, Bool.false_eq_true] <;> inv_tac

theorem inv_newCall (he : enabled s .newCall = true) : Inv (apply s .newCall) := by
  rw [apply_of_enabled he]; clear he
  simp only [applyCore]; inv_tac
end

theorem apply_disabled (s : St) (op : Op) (he : enabled s op = false) : apply s op = s := by
  simp [apply, he]

theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases hen : enabled s op
  · rw [apply_disabled s op hen]; exact h
  cases op with
  | load k i => exact inv_load s h k i hen
  | admit i p k m a sh kc => exact inv_admit s h i p k m a sh kc (by simpa [enabled] using hen)
  | tape f d => exact inv_tape s h f d (by simpa [enabled] using hen)
  | execute k a n b => exact inv_execute s h k a n b hen
  | build => exact inv_build s h (by simpa [enabled] using hen)
  | prove ok => exact inv_prove s h ok (by simpa [enabled] using hen)
  | recurse x => exact inv_recurse s h x (by simpa [enabled] using hen)
  | send p o => exact inv_send s h p o hen
  | newCall => exact inv_newCall s h hen

theorem inv_stepOp (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp; split
  · exact h
  · exact inv_apply s _ h

theorem inv_runOps (s : St) (ns : List Int) (h : Inv s) : Inv (runOps s ns) := by
  induction ns generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_stepOp s n h)

/-- Every state the oracle reaches satisfies every invariant. -/
theorem inv_run (u : Int) (ns : List Int) : Inv (run u ns) := inv_runOps _ _ (inv_init u)

/-! ## The universal key set never changes (EUI-002, EUI-003, CPR-003, MYC-KEY-001, GP-001) -/

theorem apply_ukey (s : St) (op : Op) : (apply s op).ukey = s.ukey := by
  unfold apply; split
  · cases op <;> simp only [applyCore, stopped, init] <;> (repeat' split) <;> rfl
  · rfl

theorem runOps_ukey (s : St) (ns : List Int) : (runOps s ns).ukey = s.ukey := by
  induction ns generalizing s with
  | nil => rfl
  | cons n ns ih =>
    simp only [runOps]; rw [ih]; unfold stepOp; split
    · rfl
    · exact apply_ukey _ _

/-- EUI-003, CPR-003, MYC-KEY-001: no admitted program, input, outcome or run length
changes the universal key set. -/
theorem EUI_003_program_is_data (u : Int) (ns : List Int) : (run u ns).ukey = u := runOps_ukey _ _

/-! ## Requirement-level corollaries over every run -/

theorem EUI_001_one_family (u : Int) (ns : List Int) :
    (run u ns).family = 0 ∨ ((run u ns).family = (run u ns).machineVersion
      ∧ ((run u ns).family = 1 ∨ (run u ns).family = 2)) := by
  rcases (inv_run u ns).admitted with h | ⟨a, b, _⟩
  · exact Or.inl h
  · exact Or.inr ⟨a, b⟩

theorem ERS_006_prog_d (u : Int) (ns : List Int) (h : (run u ns).family ≠ 0) :
    (run u ns).progD = progDigest (run u ns).image (run u ns).plan (run u ns).keyRoot
      (run u ns).machineVersion := by
  rcases (inv_run u ns).admitted with h0 | ⟨_, _, c, _⟩
  · exact absurd h0 h
  · exact c

theorem ERS_015_app_key_under_root (u : Int) (ns : List Int) (h : (run u ns).family ≠ 0) :
    (run u ns).hasAppKey = true → (run u ns).appKeyCommitted = true := by
  rcases (inv_run u ns).admitted with h0 | ⟨_, _, _, d, _⟩
  · exact absurd h0 h
  · exact d

theorem EKT_004_no_mixed_hash (u : Int) (ns : List Int) :
    ¬((run u ns).family = 2 ∧ (run u ns).usesSha = true ∧ (run u ns).usesKeccak = true) := by
  rcases (inv_run u ns).admitted with h0 | ⟨_, _, _, _, e⟩
  · intro ⟨h1, _⟩; rw [h0] at h1; exact absurd h1 (by decide)
  · exact e

theorem EUI_004_rsl_two_outcomes (u : Int) (ns : List Int) :
    (run u ns).family = 2 → (run u ns).outcome ≠ 3 := by
  have := (inv_run u ns).outcomeRange; intro hf ho
  have := this.2.2 ho; simp [famInterp] at this; omega

theorem EUI_005_one_outcome_kind (u : Int) (ns : List Int) :
    0 ≤ (run u ns).outcome ∧ (run u ns).outcome ≤ 3 :=
  ⟨(inv_run u ns).outcomeRange.1, (inv_run u ns).outcomeRange.2.1⟩

theorem EKT_011_alu_canonical (u : Int) (ns : List Int) :
    (run u ns).aluOverflow = true → (run u ns).outcome = 2 := (inv_run u ns).aluRejects

theorem ERS_005_native_comm_and_W (u : Int) (ns : List Int) : (run u ns).outcome = 1 →
    (run u ns).comm = nativeComm (run u ns).image (run u ns).tapeD
      ∧ (run u ns).wD = nativeW (run u ns).image (run u ns).tapeD :=
  fun h => let ⟨a, b, _⟩ := (inv_run u ns).native h; ⟨a, b⟩

theorem ERS_003_public_rows (u : Int) (ns : List Int) :
    (run u ns).outcome = 1 → (run u ns).publicRows = 1 + (run u ns).nW :=
  fun h => ((inv_run u ns).native h).2.2

theorem EUI_008_static_steps (u : Int) (ns : List Int) :
    (run u ns).outcome ≠ 0 → (run u ns).microOps = staticOps (run u ns).image := (inv_run u ns).static

theorem ERS_009_rejected_rsl_no_statement (u : Int) (ns : List Int) :
    (run u ns).stmtCount = 1 → ¬((run u ns).family = 2 ∧ (run u ns).outcome = 2) :=
  (inv_run u ns).stmtNotRejectedRsl

theorem ERS_010_at_most_one_statement (u : Int) (ns : List Int) : (run u ns).stmtCount ≤ 1 :=
  (inv_run u ns).stmt.2.1

theorem ERS_007_input_binds_tape (u : Int) (ns : List Int) :
    (run u ns).stmtCount = 1 → (run u ns).inputVal = (run u ns).tapeD :=
  fun h => ((inv_run u ns).stmt.2.2 h).2.2.1

theorem ERS_008_halted_payload (u : Int) (ns : List Int) :
    (run u ns).stmtCount = 1 → (run u ns).outcome = 1 →
      (run u ns).outcomeKind = 1 ∧ (run u ns).outcomePayload = valDigest (run u ns).comm (run u ns).wD :=
  fun h ho => let ⟨_, k, _, p⟩ := (inv_run u ns).stmt.2.2 h; ⟨by rw [k, ho], p ho⟩

theorem EUI_007_no_proof_without_tape (u : Int) (ns : List Int) :
    0 < (run u ns).proofsStarted → (run u ns).tapeChecked = true :=
  fun h => ((inv_run u ns).proofNeedsTape h).1

theorem EKT_007_kit_relation (u : Int) (ns : List Int) :
    (run u ns).proofAccepted = true → (run u ns).kitOk = true := (inv_run u ns).kitAccepted

theorem ERS_012_bound_into_recursion (u : Int) (ns : List Int) :
    (run u ns).phase = 9 → (run u ns).boundOutcome = (run u ns).outcomePayload := (inv_run u ns).bound

theorem ERS_002_private_stays (u : Int) (ns : List Int) : (run u ns).privateLeaks = 0 :=
  (inv_run u ns).noLeak

/-- ERS-011: the public words of an execution do not depend on the box. -/
def publics (s : St) : Int × Int × Int × Int × Int := (s.outcome, s.comm, s.wD, s.publicRows, s.microOps)

theorem ERS_011_box_independent (s : St) (k : Int) (a : Bool) (n b1 b2 : Int) :
    publics (apply s (.execute k a n b1)) = publics (apply s (.execute k a n b2)) := by
  have : enabled s (.execute k a n b1) = enabled s (.execute k a n b2) := by simp [enabled]
  unfold apply; rw [this]
  split
  · simp only [applyCore, publics]; (repeat' split) <;> rfl
  · rfl

/-- MYC-KEY-004: two programs with different images have different `prog_d` (on the
domain of the abstract digest). -/
theorem MYC_KEY_004_prog_d_injective (i1 p1 k1 m1 i2 p2 k2 m2 : Int)
    (h1 : 0 ≤ i1 ∧ i1 < 1000 ∧ 0 ≤ p1 ∧ p1 < 10 ∧ 0 ≤ k1 ∧ k1 < 10)
    (h2 : 0 ≤ i2 ∧ i2 < 1000 ∧ 0 ≤ p2 ∧ p2 < 10 ∧ 0 ≤ k2 ∧ k2 < 10)
    (h : progDigest i1 p1 k1 m1 = progDigest i2 p2 k2 m2) : i1 = i2 ∧ p1 = p2 ∧ k1 = k2 ∧ m1 = m2 := by
  unfold progDigest at h; omega

/-- MYC-KEY-003 against MYC-KEY-008 (finding FAL-EXEC-1): admission computes `prog_d`
from image, plan, key_root and machine version only. Two states that differ only in the
universal key set get the same `prog_d` from the same admission. -/
theorem MYC_KEY_003_prog_d_ignores_ukey (s : St) (u : Int) (i p k m : Int) (a sh kc : Bool) :
    (apply { s with ukey := u } (.admit i p k m a sh kc)).progD = (apply s (.admit i p k m a sh kc)).progD := by
  simp only [apply, enabled]
  by_cases hp : s.phase = 1
  · simp only [hp, decide_true, ite_true, applyCore]
    (repeat' split) <;> rfl
  · simp [hp]

end SherlockExec
