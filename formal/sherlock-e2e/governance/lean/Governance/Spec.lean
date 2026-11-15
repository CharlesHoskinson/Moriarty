/-!
# Governance oracle (bounded context Governance, slice C)

Executable reference semantics of `sherlock-e2e-governance` (EGV-001 to EGV-010) and
of the adopted governance change (`t22-governance@fe7207819`, change
`sherlock-governance`): GOV (verbs), ERT (era table), ELD (era ledger), CRR (consumer
read). Written from the EARS sentences only.

The era table holds at most 3 eras (slots 1, 2, 3; ids are given in order and never
reused, GOV-007). Each era slot has a status:

| st | meaning |
|---|---|
| 0 | unused id |
| 1 | announced, pending |
| 2 | current (activated) |
| 3 | superseded (its end was written by the next activation) |
| 4 | cancelled |
| 5 | soundness ended |

`start = 0` never occurs for a used era (`T >= now + TL >= 1`). `fin = 0` is an end
that is not set (an unbounded interval).

LEAD PICKs (recorded in ORACLE.md):
* LP-1 `end_eff(e) = min(end(e), start(P))` takes `P` as a pending era other than `e`.
* LP-2 the readback gate (EGV-002) requires threshold exactly 1 (ERT-002 says `>= 1`).
* LP-3 a cancelled era still holds its key set and ledger for ERT-003 (append-only, ERT-006).
* LP-4 soundness end clears `current` (no era is current until the next activation).
* LP-5 when several refusals of one verb apply, the state records the first in the
  order of `Refusal codes` below (the spec gives no order).
* LP-6 a premise result is written once per premise (EGV-009, one test run).
-/

namespace Governance

structure Era where
  st : Int
  start : Int
  fin : Int
  keyset : Int
  ledger : Int
  annAt : Int
  deriving Repr, DecidableEq

def Era.empty : Era := { st := 0, start := 0, fin := 0, keyset := 0, ledger := 0, annAt := 0 }

structure St where
  now : Int
  tl : Int
  rtl : Int
  q : Int
  n : Int
  grid : Int
  current : Int
  pending : Int
  nextId : Int
  e1 : Era
  e2 : Era
  e3 : Era
  roster : Int
  rosterPending : Int
  rosterAt : Int
  rosterGen : Int
  nonce : Int
  attestRoot : Int
  attestAt : Int
  attestUses : Int
  tableDeployed : Bool
  tableSealed : Bool
  l1 : Int
  l2 : Int
  l3 : Int
  sherlockUsable : Int
  unusable : Int
  pathEvidence : Int
  sealedReports : Int
  eraRow : Bool
  prem1 : Int
  prem2 : Int
  prem3 : Int
  rekeyStarted : Bool
  c1 : Int
  c2 : Int
  c3 : Int
  uses : Int
  lastRefusal : Int
  deriving Repr, DecidableEq

def init (tl rtl q n grid : Int) : St :=
  { now := 0, tl, rtl, q, n, grid, current := 0, pending := 0, nextId := 1,
    e1 := Era.empty, e2 := Era.empty, e3 := Era.empty, roster := 1, rosterPending := 0,
    rosterAt := 0, rosterGen := 0, nonce := 0, attestRoot := 0, attestAt := 0, attestUses := 0,
    tableDeployed := false, tableSealed := false, l1 := 0, l2 := 0, l3 := 0,
    sherlockUsable := 0, unusable := 0, pathEvidence := 0, sealedReports := 0, eraRow := false,
    prem1 := 0, prem2 := 0, prem3 := 0, rekeyStarted := false, c1 := 0, c2 := 0, c3 := 0,
    uses := 0, lastRefusal := 0 }

def opBase : Int := 1000000000

/-! ## Refusal codes recorded in `lastRefusal` (0 = the last verb was accepted) -/

def rNoQuorum : Int := 1          -- GOV-001
def rTableNotSealed : Int := 17   -- ERT-002 (the table is used only after its readback gate)
def rPending : Int := 2           -- GOV-002 (announce needs no pending era)
def rTooEarly : Int := 3          -- GOV-002, GOV-004 (T >= now + TL)
def rOffGrid : Int := 4           -- ERT-009
def rKeysetReused : Int := 5      -- ERT-003, EGV-004
def rLedgerReused : Int := 6      -- ERT-003
def rLedgerNotGated : Int := 7    -- ELD-002, EGV-002
def rNotStarted : Int := 8        -- GOV-003 (activate before start)
def rCancelLate : Int := 9        -- GOV-007 (cancel at or after start)
def rNoActiveEra : Int := 10      -- GOV-006
def rVkChange : Int := 11         -- EGV-007, GOV-009
def rRekeyNoEraCheck : Int := 12  -- EGV-005
def rSoundnessEnded : Int := 13   -- CRR-004
def rEraInactive : Int := 14      -- ELD-004, EGV-006, CRR-006
def rAttestNotUsable : Int := 15  -- GOV-008
def rNoRotation : Int := 16       -- GOV-011

/-! ## Reads (ERT-008: pure functions of the state, no write) -/

def era (s : St) (i : Int) : Era := if i = 1 then s.e1 else if i = 2 then s.e2 else s.e3

def setEra (s : St) (i : Int) (e : Era) : St :=
  if i = 1 then { s with e1 := e } else if i = 2 then { s with e2 := e } else { s with e3 := e }

/-- ERT-004: the effective end; 0 is unbounded. LP-1: the pending era other than `i`. -/
def endEff (s : St) (i : Int) : Int :=
  let e := era s i
  let p := if s.pending ≠ 0 ∧ s.pending ≠ i then (era s s.pending).start else 0
  if e.fin = 0 then p else if p = 0 then e.fin else min e.fin p

/-- ERT-004: `active(e)`. -/
def active (s : St) (i : Int) : Bool :=
  let e := era s i
  1 ≤ i ∧ i ≤ 3 ∧ (e.st = 1 ∨ e.st = 2 ∨ e.st = 3) ∧ e.start ≤ s.now
    ∧ (endEff s i = 0 ∨ s.now < endEff s i)

def ledgerState (s : St) (l : Int) : Int := if l = 1 then s.l1 else if l = 2 then s.l2 else s.l3

/-- The fallback of ELD-007 / CRR-006 / EGV-006: a recorded failure of premise A2 or A3. -/
def fallback (s : St) : Bool := s.prem2 = 2 ∨ s.prem3 = 2

def claims (s : St) (i : Int) : Int := if i = 1 then s.c1 else if i = 2 then s.c2 else s.c3

def setClaims (s : St) (i v : Int) : St :=
  if i = 1 then { s with c1 := v } else if i = 2 then { s with c2 := v } else { s with c3 := v }

/-! ## Operations -/

inductive Op where
  | tick (dt : Int)                                   -- 1
  | announce (sigs keyset ledger t : Int)             -- 2  GOV-002
  | activate (m : Int)                                -- 3  GOV-003
  | cancel (sigs m : Int)                             -- 4  GOV-007
  | soundnessEnd (sigs : Int)                         -- 5  GOV-006
  | rotateRoster (sigs r : Int)                       -- 6  GOV-005
  | cancelRotation (sigs : Int)                       -- 7  GOV-011
  | attest (sigs root : Int)                          -- 8  GOV-008
  | deploy (target committee threshold counter : Int) (vkEq : Bool) (ledger : Int)  -- 9
  | vkChange                                          -- 10 EGV-007
  | reportLedger (sealed : Bool)                      -- 11 EGV-003
  | premise (i : Int) (fail : Bool)                   -- 12 EGV-009
  | addEraRow                                         -- 13 EGV-005
  | rekeyStart                                        -- 14 EGV-005
  | acceptCall (e : Int)                              -- 15 ELD-003, ELD-004, ELD-007
  | useClaim (e : Int)                                -- 16 CRR-004, CRR-006, EGV-006, CRR-007
  | useAttested                                       -- 17 GOV-008
  deriving Repr, DecidableEq

def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then some (.tick a)
  else if c = 2 then some (.announce (a % 10) ((a / 10) % 100) ((a / 1000) % 100) (a / 100000))
  else if c = 3 then some (.activate a)
  else if c = 4 then some (.cancel (a % 10) (a / 10))
  else if c = 5 then some (.soundnessEnd a)
  else if c = 6 then some (.rotateRoster (a % 10) (a / 10))
  else if c = 7 then some (.cancelRotation a)
  else if c = 8 then some (.attest (a % 10) (a / 10))
  else if c = 9 then
    let v := (a / 10000) % 10
    if v = 0 ∨ v = 1 then
      some (.deploy (a % 10) ((a / 10) % 10) ((a / 100) % 10) ((a / 1000) % 10) (v = 1) (a / 100000))
    else none
  else if c = 10 then (if a = 0 then some .vkChange else none)
  else if c = 11 then (if a = 0 then some (.reportLedger false) else if a = 1 then some (.reportLedger true) else none)
  else if c = 12 then
    (if a % 10 = 1 then some (.premise (a / 10) false) else if a % 10 = 2 then some (.premise (a / 10) true) else none)
  else if c = 13 then (if a = 0 then some .addEraRow else none)
  else if c = 14 then (if a = 0 then some .rekeyStart else none)
  else if c = 15 then some (.acceptCall a)
  else if c = 16 then some (.useClaim a)
  else if c = 17 then (if a = 0 then some .useAttested else none)
  else none

def isSlot (i : Int) : Bool := 1 ≤ i ∧ i ≤ 3

def enabled (s : St) : Op → Bool
  | .tick dt => 0 ≤ dt
  | .announce _ k l _ => s.nextId ≤ 3 ∧ 1 ≤ k ∧ isSlot l
  | .activate m => isSlot m
  | .cancel _ m => isSlot m
  | .soundnessEnd _ => true
  | .rotateRoster _ r => 1 ≤ r
  | .cancelRotation _ => true
  | .attest _ root => 1 ≤ root
  | .deploy t _ _ _ _ l =>
      (t = 0 ∧ l = 0 ∧ s.tableDeployed = false) ∨ (t = 1 ∧ isSlot l ∧ ledgerState s l = 0)
        ∨ (t = 2 ∧ l = 0)
  | .vkChange => true
  | .reportLedger _ => true
  | .premise i _ => isSlot i ∧ (if i = 1 then s.prem1 else if i = 2 then s.prem2 else s.prem3) = 0
  | .addEraRow => s.eraRow = false
  | .rekeyStart => s.rekeyStarted = false
  | .acceptCall e => isSlot e ∧ (era s e).st ≠ 0
  | .useClaim e => isSlot e ∧ 0 < claims s e
  | .useAttested => true

def refuse (s : St) (code : Int) : St := { s with lastRefusal := code }

/-- GOV-003: activation of the pending era `m` (assumes the guard). -/
def doActivate (s : St) (m : Int) : St :=
  let s1 := if s.current ≠ 0 then
      setEra s s.current { era s s.current with fin := (era s m).start, st := 3 } else s
  let s2 := setEra s1 m { era s1 m with st := 2 }
  { s2 with current := m, pending := 0, lastRefusal := 0 }

def setPrem (s : St) (i v : Int) : St :=
  if i = 1 then { s with prem1 := v } else if i = 2 then { s with prem2 := v } else { s with prem3 := v }

def setLedger (s : St) (l v : Int) : St :=
  if l = 1 then { s with l1 := v } else if l = 2 then { s with l2 := v } else { s with l3 := v }

/-- ERT-003: does an earlier era hold this key set or ledger (LP-3: any used slot)? -/
def keysetUsed (s : St) (k : Int) : Bool :=
  (s.e1.st ≠ 0 ∧ s.e1.keyset = k) ∨ (s.e2.st ≠ 0 ∧ s.e2.keyset = k) ∨ (s.e3.st ≠ 0 ∧ s.e3.keyset = k)
def ledgerUsed (s : St) (l : Int) : Bool :=
  (s.e1.st ≠ 0 ∧ s.e1.ledger = l) ∨ (s.e2.st ≠ 0 ∧ s.e2.ledger = l) ∨ (s.e3.st ≠ 0 ∧ s.e3.ledger = l)

/-- One enabled operation. -/
def applyE (s : St) (op : Op) : St :=
  match op with
  | .tick dt =>
    let t := s.now + dt
    -- GOV-005: a rotation takes effect at `now + RTL`
    if s.rosterPending ≠ 0 ∧ s.rosterAt ≤ t then
      { s with now := t, roster := s.rosterPending, rosterPending := 0, rosterGen := s.rosterGen + 1 }
    else { s with now := t }
  | .announce sigs k l t =>
    -- GOV-001, GOV-002, GOV-004, ERT-002, ERT-003, ERT-009, ELD-002, EGV-004
    if sigs < s.q then refuse s rNoQuorum
    else if s.tableSealed = false then refuse s rTableNotSealed
    else if s.pending ≠ 0 then refuse s rPending
    else if t < s.now + s.tl then refuse s rTooEarly
    else if t % s.grid ≠ 0 then refuse s rOffGrid
    else if keysetUsed s k then refuse s rKeysetReused
    else if ledgerUsed s l then refuse s rLedgerReused
    else if ledgerState s l ≠ 1 then refuse s rLedgerNotGated
    else
      let m := s.nextId
      let s1 := setEra s m { st := 1, start := t, fin := 0, keyset := k, ledger := l, annAt := s.now }
      { s1 with pending := m, nextId := m + 1, nonce := s.nonce + 1, lastRefusal := 0 }
  | .activate m =>
    -- GOV-003: any party, at or after start(M), for the pending era M
    if s.pending ≠ m then refuse s rNotStarted
    else if s.now < (era s m).start then refuse s rNotStarted
    else doActivate s m
  | .cancel sigs m =>
    -- GOV-001, GOV-007: before start(M); the id is never used again
    if sigs < s.q then refuse s rNoQuorum
    else if s.pending ≠ m ∨ (era s m).start ≤ s.now then refuse s rCancelLate
    else
      let s1 := setEra s m { era s m with st := 4 }
      { s1 with pending := 0, nonce := s.nonce + 1, lastRefusal := 0 }
  | .soundnessEnd sigs =>
    -- GOV-001, GOV-006: first a due activation, then end(N) = now, cancel a pending era
    if sigs < s.q then refuse s rNoQuorum
    else
      let s1 := if s.pending ≠ 0 ∧ (era s s.pending).start ≤ s.now then doActivate s s.pending else s
      let nEra := s1.current
      if nEra = 0 ∨ !(active s1 nEra) then refuse s rNoActiveEra
      else
        let s2 := setEra s1 nEra { era s1 nEra with fin := s.now, st := 5 }
        let s3 := if s2.pending ≠ 0 then setEra s2 s2.pending { era s2 s2.pending with st := 4 } else s2
        { s3 with current := 0, pending := 0, nonce := s.nonce + 1, lastRefusal := 0 }
  | .rotateRoster sigs r =>
    -- GOV-001, GOV-005: in force at now + RTL; no era, ledger, claim or nullifier changes
    if sigs < s.q then refuse s rNoQuorum
    else { s with rosterPending := r, rosterAt := s.now + s.rtl, nonce := s.nonce + 1, lastRefusal := 0 }
  | .cancelRotation sigs =>
    -- GOV-001, GOV-011
    if sigs < s.q then refuse s rNoQuorum
    else if s.rosterPending = 0 then refuse s rNoRotation
    else { s with rosterPending := 0, rosterAt := 0, nonce := s.nonce + 1, lastRefusal := 0 }
  | .attest sigs root =>
    -- GOV-001, GOV-008
    if sigs < s.q then refuse s rNoQuorum
    else { s with attestRoot := root, attestAt := s.now, nonce := s.nonce + 1, lastRefusal := 0 }
  | .deploy t c th ctr vk l =>
    -- EGV-001, ERT-001, ELD-001: the runner deploys {[], 1, 0}; the arguments are the
    -- readback. EGV-002, ERT-002, ELD-002 (LP-2): a readback that differs marks it unusable.
    let ok := c = 0 ∧ th = 1 ∧ ctr = 0 ∧ vk = true
    if t = 0 then
      { s with tableDeployed := true, tableSealed := ok, unusable := if ok then s.unusable else s.unusable + 1 }
    else if t = 1 then
      { setLedger s l (if ok then 1 else 2) with unusable := if ok then s.unusable else s.unusable + 1 }
    else
      if ok then { s with sherlockUsable := s.sherlockUsable + 1 } else { s with unusable := s.unusable + 1 }
  | .vkChange =>
    -- EGV-007, GOV-009: no verifier-key change on a deployed contract
    refuse s rVkChange
  | .reportLedger sealed =>
    -- EGV-003: an unsealed ledger's receipts are transaction-path evidence
    if sealed then { s with sealedReports := s.sealedReports + 1 } else { s with pathEvidence := s.pathEvidence + 1 }
  | .premise i fail =>
    -- EGV-009 (LP-6): one result per premise A1, A2, A3
    setPrem s i (if fail then 2 else 1)
  | .addEraRow => { s with eraRow := true }
  | .rekeyStart =>
    -- EGV-005: no era row and no recorded failure of A2 or A3 -> refuse
    if s.eraRow = false ∧ fallback s = false then refuse s rRekeyNoEraCheck
    else { s with rekeyStarted := true, lastRefusal := 0 }
  | .acceptCall e =>
    -- ELD-003, ELD-004: primary path checks active(era_id) at inclusion;
    -- ELD-007: under the fallback the T20 entries are unchanged (no era check)
    if fallback s = false ∧ active s e = false then refuse s rEraInactive
    else { setClaims s e (claims s e + 1) with lastRefusal := 0 }
  | .useClaim e =>
    -- CRR-004: a soundness-ended era is unsafe; EGV-006, CRR-006: the fallback checks
    -- active at use; CRR-007: the consumer writes only its own nullifier (`uses`)
    if (era s e).st = 5 then refuse s rSoundnessEnded
    else if fallback s = true ∧ active s e = false then refuse s rEraInactive
    else { s with uses := s.uses + 1, lastRefusal := 0 }
  | .useAttested =>
    -- GOV-008: an attested root is usable only from TL after it was recorded
    if s.attestRoot = 0 ∨ s.now < s.attestAt + s.tl then refuse s rAttestNotUsable
    else { s with attestUses := s.attestUses + 1, lastRefusal := 0 }

/-- One operation. A disabled operation leaves the state unchanged (`--step` refuses it). -/
def apply (s : St) (op : Op) : St := if enabled s op then applyE s op else s

theorem apply_en {s : St} {op : Op} (h : enabled s op = true) : apply s op = applyE s op := by
  simp [apply, h]

theorem apply_dis {s : St} {op : Op} (h : enabled s op = false) : apply s op = s := by
  simp [apply, h]

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

def eraDom (e : Era) : Bool :=
  0 ≤ e.st && e.st ≤ 5 && 0 ≤ e.start && 0 ≤ e.fin && 0 ≤ e.keyset && 0 ≤ e.ledger && e.ledger ≤ 3
    && 0 ≤ e.annAt

def paramsOk (tl rtl q n grid : Int) : Bool :=
  1 ≤ tl && 0 ≤ rtl && 1 ≤ q && q ≤ n && n ≤ 9 && 1 ≤ grid

def validState (s : St) : Bool :=
  paramsOk s.tl s.rtl s.q s.n s.grid && 0 ≤ s.now && 0 ≤ s.current && s.current ≤ 3
    && 0 ≤ s.pending && s.pending ≤ 3 && 1 ≤ s.nextId && s.nextId ≤ 4
    && eraDom s.e1 && eraDom s.e2 && eraDom s.e3
    && 0 ≤ s.l1 && s.l1 ≤ 2 && 0 ≤ s.l2 && s.l2 ≤ 2 && 0 ≤ s.l3 && s.l3 ≤ 2
    && 0 ≤ s.prem1 && s.prem1 ≤ 2 && 0 ≤ s.prem2 && s.prem2 ≤ 2 && 0 ≤ s.prem3 && s.prem3 ≤ 2
    && 0 ≤ s.c1 && 0 ≤ s.c2 && 0 ≤ s.c3 && 0 ≤ s.uses && 0 ≤ s.nonce && 0 ≤ s.lastRefusal
    && s.lastRefusal ≤ 17

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

def refusals (tl rtl q n grid : Int) (ops : List Int) : List Refusal :=
  (if paramsOk tl rtl q n grid then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (tl rtl q n grid : Int) (ops : List Int) : St := runOps (init tl rtl q n grid) ops

end Governance
