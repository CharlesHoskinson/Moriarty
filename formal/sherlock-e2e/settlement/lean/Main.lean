import SherlockCodec.Wire
import Settlement.Spec

/-! Oracle executable of Settlement: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec Settlement

def fields : List (String × Kind) :=
  [("deployed", .bool),
   ("anchor", .int),
   ("anchorSlot", .int),
   ("anchorSealed", .bool),
   ("entries", .int),
   ("keyRun", .int),
   ("keyColl", .int),
   ("keyRoot", .int),
   ("keyRunRead", .int),
   ("deploys", .int),
   ("r1", .int),
   ("r2", .int),
   ("r3", .int),
   ("acceptRunTx", .int),
   ("argFiles", .int),
   ("argsReady", .bool),
   ("mismatches", .int),
   ("refusedNative", .int),
   ("routedJobs", .int),
   ("lastRefusal", .int),
   ("collForm", .bool),
   ("epochOpen", .bool),
   ("epochStart", .int),
   ("epochJobs", .int),
   ("now", .int),
   ("eraEnd", .int),
   ("collBudget", .int),
   ("inclTime", .int),
   ("epochsClosed", .int),
   ("jobsClosed", .int),
   ("collTx", .int),
   ("blockedColl", .int),
   ("refusedColl", .int),
   ("pathsGiven", .int),
   ("leafBytes", .int),
   ("cr1", .bool),
   ("cr2", .bool),
   ("cr3", .bool),
   ("regRoot", .int),
   ("intentTtl", .int),
   ("planned", .bool),
   ("planRefused", .bool),
   ("lastK", .int),
   ("runsStarted", .int),
   ("runsRefusedEra", .int),
   ("freezeWritten", .bool),
   ("openListed", .bool)]

def enc (s : St) : List V :=
  [.b s.deployed, .i s.anchor, .i s.anchorSlot, .b s.anchorSealed, .i s.entries, .i s.keyRun, .i s.keyColl, .i s.keyRoot, .i s.keyRunRead, .i s.deploys, .i s.r1, .i s.r2, .i s.r3, .i s.acceptRunTx, .i s.argFiles, .b s.argsReady, .i s.mismatches, .i s.refusedNative, .i s.routedJobs, .i s.lastRefusal, .b s.collForm, .b s.epochOpen, .i s.epochStart, .i s.epochJobs, .i s.now, .i s.eraEnd, .i s.collBudget, .i s.inclTime, .i s.epochsClosed, .i s.jobsClosed, .i s.collTx, .i s.blockedColl, .i s.refusedColl, .i s.pathsGiven, .i s.leafBytes, .b s.cr1, .b s.cr2, .b s.cr3, .i s.regRoot, .i s.intentTtl, .b s.planned, .b s.planRefused, .i s.lastK, .i s.runsStarted, .i s.runsRefusedEra, .b s.freezeWritten, .b s.openListed]

def dec : List V → Option St
  | (.b deployed) :: (.i anchor) :: (.i anchorSlot) :: (.b anchorSealed) :: (.i entries) :: (.i keyRun) :: (.i keyColl) :: (.i keyRoot) :: (.i keyRunRead) :: (.i deploys) :: (.i r1) :: (.i r2) :: (.i r3) :: (.i acceptRunTx) :: (.i argFiles) :: (.b argsReady) :: (.i mismatches) :: (.i refusedNative) :: (.i routedJobs) :: (.i lastRefusal) :: (.b collForm) :: (.b epochOpen) :: (.i epochStart) :: (.i epochJobs) :: (.i now) :: (.i eraEnd) :: (.i collBudget) :: (.i inclTime) :: (.i epochsClosed) :: (.i jobsClosed) :: (.i collTx) :: (.i blockedColl) :: (.i refusedColl) :: (.i pathsGiven) :: (.i leafBytes) :: (.b cr1) :: (.b cr2) :: (.b cr3) :: (.i regRoot) :: (.i intentTtl) :: (.b planned) :: (.b planRefused) :: (.i lastK) :: (.i runsStarted) :: (.i runsRefusedEra) :: (.b freezeWritten) :: (.b openListed) :: [] =>
    some { deployed, anchor, anchorSlot, anchorSealed, entries, keyRun, keyColl, keyRoot, keyRunRead, deploys, r1, r2, r3, acceptRunTx, argFiles, argsReady, mismatches, refusedNative, routedJobs, lastRefusal, collForm, epochOpen, epochStart, epochJobs, now, eraEnd, collBudget, inclTime, epochsClosed, jobsClosed, collTx, blockedColl, refusedColl, pathsGiven, leafBytes, cr1, cr2, cr3, regRoot, intentTtl, planned, planRefused, lastK, runsStarted, runsRefusedEra, freezeWritten, openListed }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["collBudget", "inclTime", "regRoot"],
    refusals := fun ps ops => match ps with
      | [cb, it, rr] => (refusals cb it rr ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [cb, it, rr] => run cb it rr ops
      | _ => init 0 0 0 }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args

