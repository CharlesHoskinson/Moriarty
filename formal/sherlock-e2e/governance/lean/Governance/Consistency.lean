import Governance.Spec

/-!
# Requirements with no transition (SPEC-CHECKED) and measured constants

Each row below has no state machine in this oracle: it constrains a record (a freeze
record, a migration plan, a consumer's sealed configuration, the entry list of a
contract) or a review process. Each gets one Bool predicate `<ID>_ok` over a `Design`
record, and `consistent` shows that one design meets all of them at the spec's values.
-/

namespace Governance

/-- One governance parameter in the T15 freeze record. -/
structure Param where
  ruled : Bool        -- the owner ruled on it
  listed : Bool       -- the freeze record lists it as open
  recommended : Int   -- the recommended value written next to it (seconds or keys)
  deriving Repr, DecidableEq

/-- A consumer contract's configuration (CRR-001 to CRR-003, CRR-005). -/
structure Consumer where
  profileA : Bool            -- follows the era table
  sealsTable : Bool          -- the table address is a sealed field
  eraIsArg : Bool            -- the era of each claim is a call argument
  sealedLedgers : List Int   -- profile B: era ledgers sealed at deployment
  readLedgers : List Int     -- ledgers the consumer reads
  ledgerOfNamed : Bool       -- profile A: called ledger = ledgerOf(era_id)
  attestedLeaf : Bool        -- or: a leaf of a usable attested root (one ledger per program)
  nulFields : List Nat       -- nullifier hash inputs: 0 tag, 1 lineage, 2 input_pub, 3 outcome_pub, 4 era_id, 5 keyset_d, 6 C_P
  deriving Repr, DecidableEq

/-- A migration plan for a proof-version fork (EGV-010). -/
structure Migration where
  stopsVerify : Bool
  forkHeight : Int
  deployHeight : Int
  newTable : Bool
  newLedgers : Bool
  predecessorLink : Bool
  vkChangeOnOld : Bool
  deriving Repr, DecidableEq

/-- One read entry of the era table (ERT-008). -/
structure ReadEntry where
  writes : Nat
  keyId : Nat
  deriving Repr, DecidableEq

structure Design where
  tl : Param
  rtl : Param
  q : Param
  n : Param
  grid : Param
  skew : Param
  consumer : Consumer
  migration : Migration
  reads : List ReadEntry          -- active, era, ledgerOf, eraSafe, ledgerRootUsable
  verbs : List Nat                -- verb kinds of the era table (Op codes)
  maintenanceUpdate : Bool        -- the table needs a MaintenanceUpdate
  freezeVerb : Bool               -- a verb freezes, seizes or pauses
  reviewed : Bool                 -- LPC-012: the T22 change was reviewed
  carbonVerdict : Bool            -- a Carbon Quint lead verdict is filed
  deriving Repr, DecidableEq

def paramOk (p : Param) : Bool := p.ruled || (p.listed && 0 < p.recommended)

/-- EGV-008: every parameter without an owner ruling is listed with its recommended value;
the recommended values keep the derived bounds `RTL >= TL` (P2) and `Q <= n` (P4). -/
def EGV_008_ok (d : Design) : Bool :=
  paramOk d.tl && paramOk d.rtl && paramOk d.q && paramOk d.n && paramOk d.grid && paramOk d.skew
    && decide (d.tl.recommended ≤ d.rtl.recommended) && decide (d.q.recommended ≤ d.n.recommended)

/-- EGV-010: a fork that stops verification gets a new table and new ledgers with a
predecessor link, deployed before the fork height, and no key change on the old ones (EGV-007). -/
def EGV_010_ok (d : Design) : Bool :=
  let m := d.migration
  !m.stopsVerify || (m.newTable && m.newLedgers && m.predecessorLink && decide (m.deployHeight < m.forkHeight)
    && !m.vkChangeOnOld)

/-- LPC-012: a reviewed T22 change has a Carbon Quint lead verdict. -/
def LPC_012_ok (d : Design) : Bool := !d.reviewed || d.carbonVerdict

/-- CRR-001: profile A seals the table address and takes the era as a call argument. -/
def CRR_001_ok (d : Design) : Bool := !d.consumer.profileA || (d.consumer.sealsTable && d.consumer.eraIsArg)

/-- CRR-002: profile B reads only the era ledgers sealed at its deployment. -/
def CRR_002_ok (d : Design) : Bool :=
  d.consumer.profileA || d.consumer.readLedgers.all (fun l => d.consumer.sealedLedgers.contains l)

/-- CRR-003: profile A calls only a ledger named by `ledgerOf(era_id)` or by a usable attested leaf. -/
def CRR_003_ok (d : Design) : Bool :=
  !d.consumer.profileA || d.consumer.ledgerOfNamed || d.consumer.attestedLeaf

/-- CRR-005: the nullifier is keyed by (tag, lineage, input_pub, outcome_pub) and by no
era, key set or per-era program digest. -/
def CRR_005_ok (d : Design) : Bool := d.consumer.nulFields == [0, 1, 2, 3]

/-- ERT-008: the read entries write no state and each has its own key. -/
def ERT_008_ok (d : Design) : Bool :=
  d.reads.length == 5 && d.reads.all (fun r => r.writes == 0)
    && (d.reads.map (·.keyId)).eraseDups.length == d.reads.length

/-- GOV-009: every verb is an entry (the `Op` codes 2 to 8 of this oracle), no
`MaintenanceUpdate`, no verb that freezes, seizes or pauses. -/
def GOV_009_ok (d : Design) : Bool :=
  d.verbs == [2, 3, 4, 5, 6, 7, 8] && !d.maintenanceUpdate && !d.freezeVerb

def allOk (d : Design) : Bool :=
  EGV_008_ok d && EGV_010_ok d && LPC_012_ok d && CRR_001_ok d && CRR_002_ok d && CRR_003_ok d
    && CRR_005_ok d && ERT_008_ok d && GOV_009_ok d

/-- The witness: T22 recommended values (OPEN-QUESTIONS.md section 2: `TL` = 7 days,
`RTL` = `TL`, `Q` = 12 of `n` = 16, `GRID` = 3,600 s, `SKEW` = 60 s), none ruled; a
profile A consumer under N-11 (one ledger per program, attested leaf); the O11 migration;
LPC-012 not yet reviewed (status MISSING on 2026-11-15). -/
def wConsumer : Consumer where
  profileA := true
  sealsTable := true
  eraIsArg := true
  sealedLedgers := []
  readLedgers := []
  ledgerOfNamed := false
  attestedLeaf := true
  nulFields := [0, 1, 2, 3]

def wMigration : Migration where
  stopsVerify := true
  forkHeight := 1000
  deployHeight := 900
  newTable := true
  newLedgers := true
  predecessorLink := true
  vkChangeOnOld := false

def witness : Design where
  tl := ⟨false, true, 604800⟩
  rtl := ⟨false, true, 604800⟩
  q := ⟨false, true, 12⟩
  n := ⟨false, true, 16⟩
  grid := ⟨false, true, 3600⟩
  skew := ⟨false, true, 60⟩
  consumer := wConsumer
  migration := wMigration
  reads := [⟨0, 1⟩, ⟨0, 2⟩, ⟨0, 3⟩, ⟨0, 4⟩, ⟨0, 5⟩]
  verbs := [2, 3, 4, 5, 6, 7, 8]
  maintenanceUpdate := false
  freezeVerb := false
  reviewed := false
  carbonVerdict := false

theorem consistent : allOk witness = true := by decide

/-- A profile B consumer that reads a ledger it did not seal fails CRR-002. -/
theorem CRR_002_refuses_unsealed :
    CRR_002_ok { witness with
      consumer := { wConsumer with profileA := false, sealedLedgers := [1], readLedgers := [1, 2] } } = false := by
  decide
/-- A nullifier keyed by the era fails CRR-005. -/
theorem CRR_005_refuses_era_key :
    CRR_005_ok { witness with consumer := { wConsumer with nulFields := [0, 4, 2, 3] } } = false := by
  decide
/-- `RTL < TL` fails the derived bound of EGV-008 (P2). -/
theorem EGV_008_refuses_short_rtl :
    EGV_008_ok { witness with rtl := ⟨false, true, 3600⟩ } = false := by decide
/-- A migration deployed at the fork height fails EGV-010. -/
theorem EGV_010_refuses_late :
    EGV_010_ok { witness with migration := { wMigration with deployHeight := 1000 } } = false := by
  decide

/-! ## Measured constants -/

/-- ELD-006: the era ledger's wrap-keyed entries `acceptCollection` (91 lanes) and
`acceptRun` (85 lanes) fit `L_TX` = 1,024 lanes. MEASURED compile only (compactc 0.36.0
`--skip-zk`, no keys), `t22-governance@fe7207819:research/sherlock-t22-governance-20261114/REPORT.md:10,32`. -/
def lTx : Nat := 1024
def measuredAcceptCollection : Nat := 91
def measuredAcceptRun : Nat := 85

theorem measured_ELD_006 : measuredAcceptCollection ≤ lTx ∧ measuredAcceptRun ≤ lTx := by decide

end Governance
