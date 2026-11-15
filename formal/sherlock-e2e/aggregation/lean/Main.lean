import SherlockCodec
import Aggregation.Spec

/-! Oracle executable of Aggregation: codec only (see `SherlockCodec`). -/

open Lean SherlockCodec Aggregation

def fieldNames : List String :=
  ["runProg", "keyRoot", "regRoot", "sMax", "mwColumns", "nodeArity", "segOn", "segPre", "segPost", "segLo", "segHi", "chipSets", "leaves", "works", "depsD", "sealed", "claimSteps", "outcome", "lAccepted", "mwKeyOk", "foreignUnits", "suspended", "ckPost", "ckHi", "cKind", "cProg", "cKeyRoot", "cPre", "cPost", "cHPre", "cSteps", "cAirs", "cRegRoot", "cChipSet", "cForged", "cKeyRow", "cStatus", "cOutcome", "cChain", "cManifestOk"]

def enc (s : St) : Json := Json.mkObj [
  ("runProg", natJ s.runProg),
  ("keyRoot", natJ s.keyRoot),
  ("regRoot", natJ s.regRoot),
  ("sMax", natJ s.sMax),
  ("mwColumns", natJ s.mwColumns),
  ("nodeArity", natJ s.nodeArity),
  ("segOn", toJson s.segOn),
  ("segPre", natJ s.segPre),
  ("segPost", natJ s.segPost),
  ("segLo", natJ s.segLo),
  ("segHi", natJ s.segHi),
  ("chipSets", natJ s.chipSets),
  ("leaves", natJ s.leaves),
  ("works", natJ s.works),
  ("depsD", natJ s.depsD),
  ("sealed", toJson s.sealed),
  ("claimSteps", natJ s.claimSteps),
  ("outcome", natJ s.outcome),
  ("lAccepted", toJson s.lAccepted),
  ("mwKeyOk", toJson s.mwKeyOk),
  ("foreignUnits", natJ s.foreignUnits),
  ("suspended", toJson s.suspended),
  ("ckPost", natJ s.ckPost),
  ("ckHi", natJ s.ckHi),
  ("cKind", natJ s.cKind),
  ("cProg", natJ s.cProg),
  ("cKeyRoot", natJ s.cKeyRoot),
  ("cPre", natJ s.cPre),
  ("cPost", natJ s.cPost),
  ("cHPre", natJ s.cHPre),
  ("cSteps", natJ s.cSteps),
  ("cAirs", natJ s.cAirs),
  ("cRegRoot", natJ s.cRegRoot),
  ("cChipSet", natJ s.cChipSet),
  ("cForged", natJ s.cForged),
  ("cKeyRow", natJ s.cKeyRow),
  ("cStatus", natJ s.cStatus),
  ("cOutcome", natJ s.cOutcome),
  ("cChain", natJ s.cChain),
  ("cManifestOk", toJson s.cManifestOk)]

def dec (j : Json) : Except String St := do
  exactKeys j "state" fieldNames
  pure { runProg := ← getNat j "runProg",
         keyRoot := ← getNat j "keyRoot",
         regRoot := ← getNat j "regRoot",
         sMax := ← getNat j "sMax",
         mwColumns := ← getNat j "mwColumns",
         nodeArity := ← getNat j "nodeArity",
         segOn := ← getBool j "segOn",
         segPre := ← getNat j "segPre",
         segPost := ← getNat j "segPost",
         segLo := ← getNat j "segLo",
         segHi := ← getNat j "segHi",
         chipSets := ← getNat j "chipSets",
         leaves := ← getNat j "leaves",
         works := ← getNat j "works",
         depsD := ← getNat j "depsD",
         sealed := ← getBool j "sealed",
         claimSteps := ← getNat j "claimSteps",
         outcome := ← getNat j "outcome",
         lAccepted := ← getBool j "lAccepted",
         mwKeyOk := ← getBool j "mwKeyOk",
         foreignUnits := ← getNat j "foreignUnits",
         suspended := ← getBool j "suspended",
         ckPost := ← getNat j "ckPost",
         ckHi := ← getNat j "ckHi",
         cKind := ← getNat j "cKind",
         cProg := ← getNat j "cProg",
         cKeyRoot := ← getNat j "cKeyRoot",
         cPre := ← getNat j "cPre",
         cPost := ← getNat j "cPost",
         cHPre := ← getNat j "cHPre",
         cSteps := ← getNat j "cSteps",
         cAirs := ← getNat j "cAirs",
         cRegRoot := ← getNat j "cRegRoot",
         cChipSet := ← getNat j "cChipSet",
         cForged := ← getNat j "cForged",
         cKeyRow := ← getNat j "cKeyRow",
         cStatus := ← getNat j "cStatus",
         cOutcome := ← getNat j "cOutcome",
         cChain := ← getNat j "cChain",
         cManifestOk := ← getBool j "cManifestOk" }

def ctx : Ctx St where
  fields := fieldNames
  decode := dec
  encode := enc
  stepRefusals s op := (stepRefusals s op).map Refusal.code
  step := step
  runRefusals s ops := (runRefusals s ops).map Refusal.code
  run := run

def main (args : List String) : IO UInt32 := mainWith ctx args
