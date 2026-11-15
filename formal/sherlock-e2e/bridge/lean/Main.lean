import SherlockCodecB
import Bridge.Spec

/-! Oracle executable of Bridge: codec only (see `SherlockCodecB`). -/

open Lean SherlockCodecB Bridge

def fieldNames : List String :=
  ["lCols", "lInter", "slotIface", "slotWrap", "lKeyId", "keygens", "mode", "k", "slices", "laneOk", "tplWrite", "lockOk", "tapeExposed", "tapeTyped", "transcriptOk", "stockAccepts", "planned", "phase", "keyPathC", "keyFull", "verifierRefused", "proofSha", "recordSha", "movedSha", "records", "wraps"]

def enc (s : St) : Json := Json.mkObj [
  ("lCols", natJ s.lCols),
  ("lInter", natJ s.lInter),
  ("slotIface", natJ s.slotIface),
  ("slotWrap", natJ s.slotWrap),
  ("lKeyId", natJ s.lKeyId),
  ("keygens", natJ s.keygens),
  ("mode", natJ s.mode),
  ("k", natJ s.k),
  ("slices", natJ s.slices),
  ("laneOk", toJson s.laneOk),
  ("tplWrite", toJson s.tplWrite),
  ("lockOk", toJson s.lockOk),
  ("tapeExposed", toJson s.tapeExposed),
  ("tapeTyped", toJson s.tapeTyped),
  ("transcriptOk", toJson s.transcriptOk),
  ("stockAccepts", toJson s.stockAccepts),
  ("planned", toJson s.planned),
  ("phase", natJ s.phase),
  ("keyPathC", natJ s.keyPathC),
  ("keyFull", natJ s.keyFull),
  ("verifierRefused", toJson s.verifierRefused),
  ("proofSha", natJ s.proofSha),
  ("recordSha", natJ s.recordSha),
  ("movedSha", natJ s.movedSha),
  ("records", natJ s.records),
  ("wraps", natJ s.wraps)]

def dec (j : Json) : Except String St := do
  exactKeys j "state" fieldNames
  pure { lCols := ← getNat j "lCols",
         lInter := ← getNat j "lInter",
         slotIface := ← getNat j "slotIface",
         slotWrap := ← getNat j "slotWrap",
         lKeyId := ← getNat j "lKeyId",
         keygens := ← getNat j "keygens",
         mode := ← getNat j "mode",
         k := ← getNat j "k",
         slices := ← getNat j "slices",
         laneOk := ← getBool j "laneOk",
         tplWrite := ← getBool j "tplWrite",
         lockOk := ← getBool j "lockOk",
         tapeExposed := ← getBool j "tapeExposed",
         tapeTyped := ← getBool j "tapeTyped",
         transcriptOk := ← getBool j "transcriptOk",
         stockAccepts := ← getBool j "stockAccepts",
         planned := ← getBool j "planned",
         phase := ← getNat j "phase",
         keyPathC := ← getNat j "keyPathC",
         keyFull := ← getNat j "keyFull",
         verifierRefused := ← getBool j "verifierRefused",
         proofSha := ← getNat j "proofSha",
         recordSha := ← getNat j "recordSha",
         movedSha := ← getNat j "movedSha",
         records := ← getNat j "records",
         wraps := ← getNat j "wraps" }

def ctx : Ctx St where
  fields := fieldNames
  decode := dec
  encode := enc
  stepRefusals s op := (stepRefusals s op).map Refusal.code
  step := step
  runRefusals s ops := (runRefusals s ops).map Refusal.code
  run := run

def main (args : List String) : IO UInt32 := mainWith ctx args
