import SherlockCodec.Wire
import CallFlow.Spec

/-! Oracle executable of the call-flow context: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec CallFlow

def fields : List (String × Kind) :=
  [("stage", .int),
   ("stopped", .bool),
   ("failedStage", .int),
   ("failedPin", .int),
   ("nativeRejected", .bool),
   ("universalVk", .int),
   ("programs", .int),
   ("deploys", .int),
   ("lastDeployVk", .int),
   ("vkRefused", .int),
   ("installedKeys", .int),
   ("profileKeys", .int),
   ("provedOk", .bool),
   ("claim", .int),
   ("claimA", .int),
   ("claimB", .int),
   ("txs", .int),
   ("txTotal", .int),
   ("unknown", .bool),
   ("resendRefused", .int),
   ("outcome", .int),
   ("eligible", .int),
   ("joined", .int),
   ("firstKeys", .int),
   ("mixedKeys", .bool),
   ("complete", .bool),
   ("completeBlocked", .int)]

def enc (s : St) : List V :=
  [.i s.stage, .b s.stopped, .i s.failedStage, .i s.failedPin, .b s.nativeRejected, .i s.universalVk, .i s.programs, .i s.deploys, .i s.lastDeployVk, .i s.vkRefused, .i s.installedKeys, .i s.profileKeys, .b s.provedOk, .i s.claim, .i s.claimA, .i s.claimB, .i s.txs, .i s.txTotal, .b s.unknown, .i s.resendRefused, .i s.outcome, .i s.eligible, .i s.joined, .i s.firstKeys, .b s.mixedKeys, .b s.complete, .i s.completeBlocked]

def dec : List V → Option St
  | [.i stage, .b stopped, .i failedStage, .i failedPin, .b nativeRejected, .i universalVk, .i programs, .i deploys, .i lastDeployVk, .i vkRefused, .i installedKeys, .i profileKeys, .b provedOk, .i claim, .i claimA, .i claimB, .i txs, .i txTotal, .b unknown, .i resendRefused, .i outcome, .i eligible, .i joined, .i firstKeys, .b mixedKeys, .b complete, .i completeBlocked] =>
    some { stage, stopped, failedStage, failedPin, nativeRejected, universalVk, programs, deploys, lastDeployVk, vkRefused, installedKeys, profileKeys, provedOk, claim, claimA, claimB, txs, txTotal, unknown, resendRefused, outcome, eligible, joined, firstKeys, mixedKeys, complete, completeBlocked }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["vk"],
    refusals := fun ps ops => match ps with
      | [t] => (refusals t ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [t] => run t ops
      | _ => run 0 [] }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args
