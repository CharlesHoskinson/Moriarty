import Conformance.Spec

/-!
# SPEC-CHECKED rows of Conformance

ECS-001, ECS-010 and ECS-011 fix the form of the suite records; MYC-ACC-008 and
MYC-LOW-018 fix how a conformance result is labelled and kept. None has a transition of
its own. Each is one Bool predicate over a `Report` record; `consistent` shows one report
satisfies all of them together with the dry-run counts.
-/

namespace Conformance.Consistency

structure Report where
  manifestSchemaOk : Bool    -- ECS-001: schema t18-compact-suite-manifest/1
  programs : Nat             -- ECS-001: programs in the manifest
  compiled : Nat             -- ECS-001: programs that compile natively
  callsForUncompiled : Nat   -- ECS-001 scenario: calls made for a program that does not compile
  rowHasTx : Bool            -- ECS-010: per call the transaction hash and block
  rowHasCompare : Bool       -- ECS-010: the native comparison
  rowHasTimes : Bool         -- ECS-010: the prove and wrap time per box
  circuits : Nat             -- ECS-011
  calls : Nat                -- ECS-011
  labelsSeparate : Bool      -- MYC-ACC-008: model proofs, primitives, native, ledger apart
  numbersLabelled : Bool     -- MYC-ACC-008: MEASURED / DERIVED / ESTIMATE on every number
  statusesSeparate : Bool    -- MYC-LOW-018: native compile, ZKIR emission, execution, proof
  pinsKept : Bool            -- MYC-LOW-018: hashes, pins, exact command outcome
  deriving Repr, DecidableEq

def ECS_001_ok (r : Report) : Bool := r.manifestSchemaOk && r.compiled ≤ r.programs && r.callsForUncompiled = 0
def ECS_010_ok (r : Report) : Bool := r.rowHasTx && r.rowHasCompare && r.rowHasTimes
def ECS_011_ok (r : Report) : Bool := r.circuits = 106 && r.calls = 208
def MYC_ACC_008_ok (r : Report) : Bool := r.labelsSeparate && r.numbersLabelled
def MYC_LOW_018_ok (r : Report) : Bool := r.statusesSeparate && r.pinsKept

def allOk (r : Report) : Bool :=
  ECS_001_ok r && ECS_010_ok r && ECS_011_ok r && MYC_ACC_008_ok r && MYC_LOW_018_ok r

/-- The suite as measured: 36 programs, 33 compile (MANIFEST.json, PREVIEW.md:32),
106 circuits and 208 calls (REPORT.md:8-9). -/
def witness : Report :=
  { manifestSchemaOk := true, programs := 36, compiled := 33, callsForUncompiled := 0,
    rowHasTx := true, rowHasCompare := true, rowHasTimes := true, circuits := 106, calls := 208,
    labelsSeparate := true, numbersLabelled := true, statusesSeparate := true, pinsKept := true }

theorem consistent : allOk witness = true := by decide

/-- ECS-011 and ECS-004 agree: the dry-run classes (T23-DRYRUN.md:13-19) sum to the
ECS-011 call count. -/
theorem ecs011_ecs004 : (0 : Nat) + 62 + 0 + 145 + 0 + 1 = witness.calls := by decide

end Conformance.Consistency
