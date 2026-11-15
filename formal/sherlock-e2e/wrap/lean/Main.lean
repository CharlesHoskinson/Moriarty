import SherlockCodec.Wire
import Wrap.Spec

/-! Oracle executable of the Wrap context: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec Wrap

def fields : List (String × Kind) :=
  [("phase", .int), ("k", .int), ("rows", .int), ("vkBytes", .int), ("vkId", .int),
   ("keygenRan", .bool), ("vkFileWritten", .bool), ("files", .int), ("proofs", .int),
   ("stop", .int), ("lastRefusal", .int), ("anchorRead", .int), ("chunkWrite", .int),
   ("mode", .int), ("template", .bool), ("testSeed", .bool), ("draws", .int),
   ("lastBlinder", .int), ("prevBlinder", .int), ("qual", .int), ("eraVk", .int),
   ("keysDeployed", .int), ("deployed", .int), ("gpuVk", .int)]

def enc (s : St) : List V :=
  [.i s.phase, .i s.k, .i s.rows, .i s.vkBytes, .i s.vkId, .b s.keygenRan, .b s.vkFileWritten,
   .i s.files, .i s.proofs, .i s.stop, .i s.lastRefusal, .i s.anchorRead, .i s.chunkWrite,
   .i s.mode, .b s.template, .b s.testSeed, .i s.draws, .i s.lastBlinder, .i s.prevBlinder,
   .i s.qual, .i s.eraVk, .i s.keysDeployed, .i s.deployed, .i s.gpuVk]

def dec : List V → Option St
  | [.i phase, .i k, .i rows, .i vkBytes, .i vkId, .b keygenRan, .b vkFileWritten, .i files,
     .i proofs, .i stop, .i lastRefusal, .i anchorRead, .i chunkWrite, .i mode, .b template,
     .b testSeed, .i draws, .i lastBlinder, .i prevBlinder, .i qual, .i eraVk, .i keysDeployed,
     .i deployed, .i gpuVk] =>
    some { phase, k, rows, vkBytes, vkId, keygenRan, vkFileWritten, files, proofs, stop,
           lastRefusal, anchorRead, chunkWrite, mode, template, testSeed, draws, lastBlinder,
           prevBlinder, qual, eraVk, keysDeployed, deployed, gpuVk }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["testSeed"],
    refusals := fun ps ops => match ps with
      | [t] => (refusals t ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [t] => run t ops
      | _ => init false }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args
