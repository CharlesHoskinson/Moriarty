import SherlockCodecB
import LeafProving.Spec

/-! Oracle executable of Leaf Proving: codec only (see `SherlockCodecB`). -/

open Lean SherlockCodecB LeafProving

def fieldNames : List String :=
  ["appAirs", "hasSha", "hasKeccak", "appCols", "nonCanonical", "shapeOk", "progD", "inputD",
   "tapeD", "stmtD", "fp1Outcome", "steps", "shardCap", "keyRoot", "manifestD", "admitted",
   "shardsProved", "appProofs", "leafProofs", "leafProgD", "foreignLeaves", "badLeaves",
   "records", "recordLeaves", "merged"]

def enc (s : St) : Json := Json.mkObj [
  ("appAirs", natJ s.appAirs), ("hasSha", toJson s.hasSha), ("hasKeccak", toJson s.hasKeccak),
  ("appCols", natJ s.appCols), ("nonCanonical", toJson s.nonCanonical),
  ("shapeOk", toJson s.shapeOk), ("progD", natJ s.progD), ("inputD", natJ s.inputD),
  ("tapeD", natJ s.tapeD), ("stmtD", natJ s.stmtD), ("fp1Outcome", natJ s.fp1Outcome),
  ("steps", natJ s.steps), ("shardCap", natJ s.shardCap), ("keyRoot", natJ s.keyRoot),
  ("manifestD", natJ s.manifestD), ("admitted", toJson s.admitted),
  ("shardsProved", natJ s.shardsProved), ("appProofs", natJ s.appProofs),
  ("leafProofs", natJ s.leafProofs), ("leafProgD", natJ s.leafProgD),
  ("foreignLeaves", natJ s.foreignLeaves), ("badLeaves", natJ s.badLeaves),
  ("records", natJ s.records), ("recordLeaves", natJ s.recordLeaves), ("merged", toJson s.merged)]

def dec (j : Json) : Except String St := do
  exactKeys j "state" fieldNames
  pure { appAirs := ← getNat j "appAirs", hasSha := ← getBool j "hasSha",
         hasKeccak := ← getBool j "hasKeccak", appCols := ← getNat j "appCols",
         nonCanonical := ← getBool j "nonCanonical", shapeOk := ← getBool j "shapeOk",
         progD := ← getNat j "progD", inputD := ← getNat j "inputD", tapeD := ← getNat j "tapeD",
         stmtD := ← getNat j "stmtD", fp1Outcome := ← getNat j "fp1Outcome",
         steps := ← getNat j "steps", shardCap := ← getNat j "shardCap",
         keyRoot := ← getNat j "keyRoot", manifestD := ← getNat j "manifestD",
         admitted := ← getBool j "admitted", shardsProved := ← getNat j "shardsProved",
         appProofs := ← getNat j "appProofs", leafProofs := ← getNat j "leafProofs",
         leafProgD := ← getNat j "leafProgD", foreignLeaves := ← getNat j "foreignLeaves",
         badLeaves := ← getNat j "badLeaves", records := ← getNat j "records",
         recordLeaves := ← getNat j "recordLeaves", merged := ← getBool j "merged" }

def ctx : Ctx St where
  fields := fieldNames
  decode := dec
  encode := enc
  stepRefusals s op := (stepRefusals s op).map Refusal.code
  step := step
  runRefusals s ops := (runRefusals s ops).map Refusal.code
  run := run

def main (args : List String) : IO UInt32 := mainWith ctx args
