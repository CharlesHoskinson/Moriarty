import SherlockCodec.Wire
import Conformance.Spec

/-! Oracle executable of the conformance context: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec Conformance

def fields : List (String × Kind) :=
  [("stage", .int),
   ("callId", .int),
   ("maxId", .int),
   ("starts", .int),
   ("dupRefused", .int),
   ("nativeKind", .int),
   ("transcriptWords", .int),
   ("refs", .int),
   ("cls", .int),
   ("reason", .int),
   ("derived", .bool),
   ("execRejected", .bool),
   ("compileRan", .bool),
   ("proveRan", .bool),
   ("settleRan", .bool),
   ("compareRan", .bool),
   ("verified", .bool),
   ("txIncluded", .bool),
   ("dispatch", .bool),
   ("preview", .bool),
   ("kits", .int),
   ("boxMsgs", .int),
   ("previewTx", .int),
   ("evidence", .int),
   ("nPass", .int),
   ("nRefused", .int),
   ("nMismatch", .int),
   ("nBlocked", .int),
   ("nProveFailed", .int),
   ("nNa", .int),
   ("calls", .int),
   ("compared", .int),
   ("mismatchCompared", .int),
   ("eligibleIncomplete", .int),
   ("complete", .bool),
   ("completeBlocked", .int)]

def enc (s : St) : List V :=
  [.i s.stage, .i s.callId, .i s.maxId, .i s.starts, .i s.dupRefused, .i s.nativeKind, .i s.transcriptWords, .i s.refs, .i s.cls, .i s.reason, .b s.derived, .b s.execRejected, .b s.compileRan, .b s.proveRan, .b s.settleRan, .b s.compareRan, .b s.verified, .b s.txIncluded, .b s.dispatch, .b s.preview, .i s.kits, .i s.boxMsgs, .i s.previewTx, .i s.evidence, .i s.nPass, .i s.nRefused, .i s.nMismatch, .i s.nBlocked, .i s.nProveFailed, .i s.nNa, .i s.calls, .i s.compared, .i s.mismatchCompared, .i s.eligibleIncomplete, .b s.complete, .i s.completeBlocked]

def dec : List V → Option St
  | (.i stage) :: (.i callId) :: (.i maxId) :: (.i starts) :: (.i dupRefused) :: (.i nativeKind) :: (.i transcriptWords) :: (.i refs) :: (.i cls) :: (.i reason) :: (.b derived) :: (.b execRejected) :: (.b compileRan) :: (.b proveRan) :: (.b settleRan) :: (.b compareRan) :: (.b verified) :: (.b txIncluded) :: (.b dispatch) :: (.b preview) :: (.i kits) :: (.i boxMsgs) :: (.i previewTx) :: (.i evidence) :: (.i nPass) :: (.i nRefused) :: (.i nMismatch) :: (.i nBlocked) :: (.i nProveFailed) :: (.i nNa) :: (.i calls) :: (.i compared) :: (.i mismatchCompared) :: (.i eligibleIncomplete) :: (.b complete) :: (.i completeBlocked) :: [] =>
    some { stage, callId, maxId, starts, dupRefused, nativeKind, transcriptWords, refs, cls, reason, derived, execRejected, compileRan, proveRan, settleRan, compareRan, verified, txIncluded, dispatch, preview, kits, boxMsgs, previewTx, evidence, nPass, nRefused, nMismatch, nBlocked, nProveFailed, nNa, calls, compared, mismatchCompared, eligibleIncomplete, complete, completeBlocked }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["config"],
    refusals := fun ps ops => match ps with
      | [t] => (refusals t ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [t] => run t ops
      | _ => run 0 [] }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args
