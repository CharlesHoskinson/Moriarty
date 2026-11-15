import SherlockCodec.Wire
import CompactComp.Spec

/-! Oracle executable of the compact-compilation context: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec CompactComp

def fields : List (String × Kind) :=
  [("phase", .int),
   ("deployProfile", .bool),
   ("anchorSlot", .int),
   ("requested", .int),
   ("accepted", .int),
   ("refused", .int),
   ("lastRefusal", .int),
   ("images", .int),
   ("abis", .int),
   ("exitCode", .int),
   ("keccakChip", .int),
   ("tier3", .int),
   ("jubPoints", .int),
   ("jubScalars", .int),
   ("tapeWords", .int),
   ("curveChecks", .int),
   ("secpKit", .int),
   ("img1Src", .int),
   ("img1Hash", .int),
   ("img2Src", .int),
   ("img2Hash", .int),
   ("gateBits", .int),
   ("gateFailed", .bool),
   ("failedGate", .int),
   ("settleUses", .int),
   ("useBits", .int),
   ("settleBlocked", .int)]

def enc (s : St) : List V :=
  [.i s.phase, .b s.deployProfile, .i s.anchorSlot, .i s.requested, .i s.accepted, .i s.refused, .i s.lastRefusal, .i s.images, .i s.abis, .i s.exitCode, .i s.keccakChip, .i s.tier3, .i s.jubPoints, .i s.jubScalars, .i s.tapeWords, .i s.curveChecks, .i s.secpKit, .i s.img1Src, .i s.img1Hash, .i s.img2Src, .i s.img2Hash, .i s.gateBits, .b s.gateFailed, .i s.failedGate, .i s.settleUses, .i s.useBits, .i s.settleBlocked]

def dec : List V → Option St
  | [.i phase, .b deployProfile, .i anchorSlot, .i requested, .i accepted, .i refused, .i lastRefusal, .i images, .i abis, .i exitCode, .i keccakChip, .i tier3, .i jubPoints, .i jubScalars, .i tapeWords, .i curveChecks, .i secpKit, .i img1Src, .i img1Hash, .i img2Src, .i img2Hash, .i gateBits, .b gateFailed, .i failedGate, .i settleUses, .i useBits, .i settleBlocked] =>
    some { phase, deployProfile, anchorSlot, requested, accepted, refused, lastRefusal, images, abis, exitCode, keccakChip, tier3, jubPoints, jubScalars, tapeWords, curveChecks, secpKit, img1Src, img1Hash, img2Src, img2Hash, gateBits, gateFailed, failedGate, settleUses, useBits, settleBlocked }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["deploy"],
    refusals := fun ps ops => match ps with
      | [t] => (refusals t ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [t] => run t ops
      | _ => run 0 [] }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args
