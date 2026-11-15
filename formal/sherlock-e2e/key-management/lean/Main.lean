import SherlockCodecB
import KeyManagement.Spec

/-! Oracle executable of Key Management: codec only (see `SherlockCodecB`). -/

open Lean SherlockCodecB KeyManagement

def fieldNames : List String :=
  ["listRows", "rowsCommitted", "treeHoldsAll", "rslKeyIdx", "keydagIdx", "listD", "manifestD",
   "manifestListD", "keygens", "phase", "gatesPass", "d3MinBits", "d3MinGrindBits", "qualified",
   "box0", "box1", "box2", "proofs", "setupRuns", "keyRoot", "programs"]

def enc (s : St) : Json := Json.mkObj [
  ("listRows", natJ s.listRows), ("rowsCommitted", toJson s.rowsCommitted),
  ("treeHoldsAll", toJson s.treeHoldsAll), ("rslKeyIdx", natJ s.rslKeyIdx),
  ("keydagIdx", natJ s.keydagIdx), ("listD", natJ s.listD), ("manifestD", natJ s.manifestD),
  ("manifestListD", natJ s.manifestListD), ("keygens", natJ s.keygens), ("phase", natJ s.phase),
  ("gatesPass", natJ s.gatesPass), ("d3MinBits", natJ s.d3MinBits),
  ("d3MinGrindBits", natJ s.d3MinGrindBits), ("qualified", toJson s.qualified),
  ("box0", natJ s.box0), ("box1", natJ s.box1), ("box2", natJ s.box2),
  ("proofs", natJ s.proofs), ("setupRuns", natJ s.setupRuns), ("keyRoot", natJ s.keyRoot),
  ("programs", natJ s.programs)]

def dec (j : Json) : Except String St := do
  exactKeys j "state" fieldNames
  pure { listRows := ← getNat j "listRows", rowsCommitted := ← getBool j "rowsCommitted",
         treeHoldsAll := ← getBool j "treeHoldsAll", rslKeyIdx := ← getNat j "rslKeyIdx",
         keydagIdx := ← getNat j "keydagIdx", listD := ← getNat j "listD",
         manifestD := ← getNat j "manifestD", manifestListD := ← getNat j "manifestListD",
         keygens := ← getNat j "keygens", phase := ← getNat j "phase",
         gatesPass := ← getNat j "gatesPass", d3MinBits := ← getNat j "d3MinBits",
         d3MinGrindBits := ← getNat j "d3MinGrindBits", qualified := ← getBool j "qualified",
         box0 := ← getNat j "box0", box1 := ← getNat j "box1", box2 := ← getNat j "box2",
         proofs := ← getNat j "proofs", setupRuns := ← getNat j "setupRuns",
         keyRoot := ← getNat j "keyRoot", programs := ← getNat j "programs" }

def ctx : Ctx St where
  fields := fieldNames
  decode := dec
  encode := enc
  stepRefusals s op := (stepRefusals s op).map Refusal.code
  step := step
  runRefusals s ops := (runRefusals s ops).map Refusal.code
  run := run

def main (args : List String) : IO UInt32 := mainWith ctx args
