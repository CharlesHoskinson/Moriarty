import SherlockCodec.Wire
import SherlockExec.Spec

/-! Oracle executable of the Sherlock Execution context: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec SherlockExec

def fields : List (String × Kind) :=
  [("phase", .int),
   ("stop", .int),
   ("lastRefusal", .int),
   ("refusedField", .int),
   ("family", .int),
   ("machineVersion", .int),
   ("image", .int),
   ("plan", .int),
   ("keyRoot", .int),
   ("progD", .int),
   ("hasAppKey", .bool),
   ("appKeyCommitted", .bool),
   ("usesSha", .bool),
   ("usesKeccak", .bool),
   ("tapeChecked", .bool),
   ("tapeD", .int),
   ("outcome", .int),
   ("aluOverflow", .bool),
   ("nW", .int),
   ("microOps", .int),
   ("comm", .int),
   ("wD", .int),
   ("publicRows", .int),
   ("box", .int),
   ("stmtCount", .int),
   ("outcomeKind", .int),
   ("outcomePayload", .int),
   ("inputVal", .int),
   ("proofsStarted", .int),
   ("kitOk", .bool),
   ("proofAccepted", .bool),
   ("boundOutcome", .int),
   ("publicsOut", .int),
   ("privateLeaks", .int),
   ("ukey", .int)]

def enc (s : St) : List V :=
  [.i s.phase, .i s.stop, .i s.lastRefusal, .i s.refusedField, .i s.family, .i s.machineVersion, .i s.image, .i s.plan, .i s.keyRoot, .i s.progD, .b s.hasAppKey, .b s.appKeyCommitted, .b s.usesSha, .b s.usesKeccak, .b s.tapeChecked, .i s.tapeD, .i s.outcome, .b s.aluOverflow, .i s.nW, .i s.microOps, .i s.comm, .i s.wD, .i s.publicRows, .i s.box, .i s.stmtCount, .i s.outcomeKind, .i s.outcomePayload, .i s.inputVal, .i s.proofsStarted, .b s.kitOk, .b s.proofAccepted, .i s.boundOutcome, .i s.publicsOut, .i s.privateLeaks, .i s.ukey]

/-- Cell `n` as an integer or a boolean (a list pattern of 35 cells does not elaborate). -/
def gi (l : List V) (n : Nat) : Option Int := match l[n]? with | some (.i x) => some x | _ => none
def gb (l : List V) (n : Nat) : Option Bool := match l[n]? with | some (.b x) => some x | _ => none

def dec (l : List V) : Option St := do
  if l.length ≠ 35 then none
  let phase ← gi l 0
  let stop ← gi l 1
  let lastRefusal ← gi l 2
  let refusedField ← gi l 3
  let family ← gi l 4
  let machineVersion ← gi l 5
  let image ← gi l 6
  let plan ← gi l 7
  let keyRoot ← gi l 8
  let progD ← gi l 9
  let hasAppKey ← gb l 10
  let appKeyCommitted ← gb l 11
  let usesSha ← gb l 12
  let usesKeccak ← gb l 13
  let tapeChecked ← gb l 14
  let tapeD ← gi l 15
  let outcome ← gi l 16
  let aluOverflow ← gb l 17
  let nW ← gi l 18
  let microOps ← gi l 19
  let comm ← gi l 20
  let wD ← gi l 21
  let publicRows ← gi l 22
  let box ← gi l 23
  let stmtCount ← gi l 24
  let outcomeKind ← gi l 25
  let outcomePayload ← gi l 26
  let inputVal ← gi l 27
  let proofsStarted ← gi l 28
  let kitOk ← gb l 29
  let proofAccepted ← gb l 30
  let boundOutcome ← gi l 31
  let publicsOut ← gi l 32
  let privateLeaks ← gi l 33
  let ukey ← gi l 34
  some { phase, stop, lastRefusal, refusedField, family, machineVersion, image, plan, keyRoot, progD, hasAppKey, appKeyCommitted, usesSha, usesKeccak, tapeChecked, tapeD, outcome, aluOverflow, nW, microOps, comm, wD, publicRows, box, stmtCount, outcomeKind, outcomePayload, inputVal, proofsStarted, kitOk, proofAccepted, boundOutcome, publicsOut, privateLeaks, ukey }

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["ukey"],
    refusals := fun ps ops => match ps with
      | [u] => (refusals u ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [u] => run u ops
      | _ => init 0 }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args
