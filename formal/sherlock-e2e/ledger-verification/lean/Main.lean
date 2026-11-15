import SherlockCodec.Wire
import LedgerVerification.Spec

/-! Oracle executable of LedgerVerification: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec LedgerVerification

def fields : List (String × Kind) :=
  [("cr0", .bool),
   ("cr1", .bool),
   ("cr2", .bool),
   ("cr3", .bool),
   ("gateOk", .bool),
   ("keyRead", .bool),
   ("lastG4", .bool),
   ("checks", .int),
   ("controls", .int),
   ("checkDone", .bool),
   ("exitNonzero", .bool),
   ("lastHonest", .bool),
   ("lastLanes", .int),
   ("lastFlips", .int),
   ("lastGate", .bool),
   ("callPhase", .int),
   ("nativeChecks", .int),
   ("nativeOk", .int),
   ("refusedCalls", .int),
   ("submits", .int),
   ("genesis", .int),
   ("nodeVersion", .int),
   ("connects", .int),
   ("chainStop", .bool),
   ("lastRefusal", .int)]

def enc (s : St) : List V :=
  [.b s.cr0, .b s.cr1, .b s.cr2, .b s.cr3, .b s.gateOk, .b s.keyRead, .b s.lastG4, .i s.checks, .i s.controls, .b s.checkDone, .b s.exitNonzero, .b s.lastHonest, .i s.lastLanes, .i s.lastFlips, .b s.lastGate, .i s.callPhase, .i s.nativeChecks, .i s.nativeOk, .i s.refusedCalls, .i s.submits, .i s.genesis, .i s.nodeVersion, .i s.connects, .b s.chainStop, .i s.lastRefusal]

def dec : List V → Option St
  | [.b cr0, .b cr1, .b cr2, .b cr3, .b gateOk, .b keyRead, .b lastG4, .i checks, .i controls, .b checkDone, .b exitNonzero, .b lastHonest, .i lastLanes, .i lastFlips, .b lastGate, .i callPhase, .i nativeChecks, .i nativeOk, .i refusedCalls, .i submits, .i genesis, .i nodeVersion, .i connects, .b chainStop, .i lastRefusal] =>
    some { cr0, cr1, cr2, cr3, gateOk, keyRead, lastG4, checks, controls, checkDone, exitNonzero, lastHonest, lastLanes, lastFlips, lastGate, callPhase, nativeChecks, nativeOk, refusedCalls, submits, genesis, nodeVersion, connects, chainStop, lastRefusal }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := [],
    refusals := fun ps ops => match ps with
      | [] => (refusals ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun _ ops => run ops }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args

