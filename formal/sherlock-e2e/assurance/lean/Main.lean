import SherlockCodec
import Assurance.Spec

/-! Oracle executable of Assurance: codec only (see `SherlockCodec`). -/

open Lean SherlockCodec Assurance

def fieldNames : List String :=
  ["testSeedBuild", "seedEnv", "constRpPath", "rpDraws", "deploySecret", "saltDraws", "salt", "blinderDraws", "blinders", "devices", "submitted", "roster", "transfers", "usedSlots", "gateBits", "unverified", "confirmedNoFix", "notAttempted", "g5Current", "gateRecord", "recordManifest", "recordOpen", "manifestD", "qualified", "runRecords", "lastRunQualified", "changes", "changeRows", "verdicts", "doneRows"]

def enc (s : St) : Json := Json.mkObj [
  ("testSeedBuild", toJson s.testSeedBuild),
  ("seedEnv", toJson s.seedEnv),
  ("constRpPath", toJson s.constRpPath),
  ("rpDraws", natJ s.rpDraws),
  ("deploySecret", toJson s.deploySecret),
  ("saltDraws", natJ s.saltDraws),
  ("salt", natJ s.salt),
  ("blinderDraws", natJ s.blinderDraws),
  ("blinders", natsJ s.blinders),
  ("devices", natsJ s.devices),
  ("submitted", natJ s.submitted),
  ("roster", natJ s.roster),
  ("transfers", natJ s.transfers),
  ("usedSlots", natJ s.usedSlots),
  ("gateBits", natJ s.gateBits),
  ("unverified", natJ s.unverified),
  ("confirmedNoFix", natJ s.confirmedNoFix),
  ("notAttempted", natJ s.notAttempted),
  ("g5Current", toJson s.g5Current),
  ("gateRecord", toJson s.gateRecord),
  ("recordManifest", natJ s.recordManifest),
  ("recordOpen", natJ s.recordOpen),
  ("manifestD", natJ s.manifestD),
  ("qualified", toJson s.qualified),
  ("runRecords", natJ s.runRecords),
  ("lastRunQualified", toJson s.lastRunQualified),
  ("changes", natJ s.changes),
  ("changeRows", natJ s.changeRows),
  ("verdicts", natJ s.verdicts),
  ("doneRows", natJ s.doneRows)]

def dec (j : Json) : Except String St := do
  exactKeys j "state" fieldNames
  pure { testSeedBuild := ← getBool j "testSeedBuild",
         seedEnv := ← getBool j "seedEnv",
         constRpPath := ← getBool j "constRpPath",
         rpDraws := ← getNat j "rpDraws",
         deploySecret := ← getBool j "deploySecret",
         saltDraws := ← getNat j "saltDraws",
         salt := ← getNat j "salt",
         blinderDraws := ← getNat j "blinderDraws",
         blinders := ← getNats j "blinders",
         devices := ← getNats j "devices",
         submitted := ← getNat j "submitted",
         roster := ← getNat j "roster",
         transfers := ← getNat j "transfers",
         usedSlots := ← getNat j "usedSlots",
         gateBits := ← getNat j "gateBits",
         unverified := ← getNat j "unverified",
         confirmedNoFix := ← getNat j "confirmedNoFix",
         notAttempted := ← getNat j "notAttempted",
         g5Current := ← getBool j "g5Current",
         gateRecord := ← getBool j "gateRecord",
         recordManifest := ← getNat j "recordManifest",
         recordOpen := ← getNat j "recordOpen",
         manifestD := ← getNat j "manifestD",
         qualified := ← getBool j "qualified",
         runRecords := ← getNat j "runRecords",
         lastRunQualified := ← getBool j "lastRunQualified",
         changes := ← getNat j "changes",
         changeRows := ← getNat j "changeRows",
         verdicts := ← getNat j "verdicts",
         doneRows := ← getNat j "doneRows" }

def ctx : Ctx St where
  fields := fieldNames
  decode := dec
  encode := enc
  stepRefusals s op := (stepRefusals s op).map Refusal.code
  step := step
  runRefusals s ops := (runRefusals s ops).map Refusal.code
  run := run

def main (args : List String) : IO UInt32 := mainWith ctx args
