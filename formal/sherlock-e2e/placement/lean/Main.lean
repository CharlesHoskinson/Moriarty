import SherlockCodec.Wire
import Placement.Spec

/-! Oracle executable of the Placement context: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec Placement

def fields : List (String × Kind) :=
  [("phase", .int),
   ("jobCv", .int),
   ("boxOwned", .bool),
   ("resCv", .int),
   ("shaOk", .bool),
   ("resWords", .int),
   ("resPrivate", .int),
   ("verifyRan", .bool),
   ("verifyOk", .bool),
   ("dropCode", .int),
   ("kept", .int),
   ("submitted", .int),
   ("lastRefusal", .int),
   ("signed", .int),
   ("signedNoWallet", .int),
   ("slotsFree", .int),
   ("slotsUsed", .int),
   ("helperSlots", .int),
   ("wrapProofs", .int),
   ("sentOwned", .int),
   ("sentNotOwned", .int)]

def enc (s : St) : List V :=
  [.i s.phase, .i s.jobCv, .b s.boxOwned, .i s.resCv, .b s.shaOk, .i s.resWords, .i s.resPrivate, .b s.verifyRan, .b s.verifyOk, .i s.dropCode, .i s.kept, .i s.submitted, .i s.lastRefusal, .i s.signed, .i s.signedNoWallet, .i s.slotsFree, .i s.slotsUsed, .i s.helperSlots, .i s.wrapProofs, .i s.sentOwned, .i s.sentNotOwned]

def dec : List V → Option St
  | [.i phase, .i jobCv, .b boxOwned, .i resCv, .b shaOk, .i resWords, .i resPrivate, .b verifyRan, .b verifyOk, .i dropCode, .i kept, .i submitted, .i lastRefusal, .i signed, .i signedNoWallet, .i slotsFree, .i slotsUsed, .i helperSlots, .i wrapProofs, .i sentOwned, .i sentNotOwned] =>
    some { phase, jobCv, boxOwned, resCv, shaOk, resWords, resPrivate, verifyRan, verifyOk, dropCode, kept, submitted, lastRefusal, signed, signedNoWallet, slotsFree, slotsUsed, helperSlots, wrapProofs, sentOwned, sentNotOwned }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := [],
    refusals := fun _ ops => (refusals ops).map Refusal.code,
    run := fun _ ops => run ops }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args
