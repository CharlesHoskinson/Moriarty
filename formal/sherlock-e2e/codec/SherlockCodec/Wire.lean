import Lean.Data.Json

/-!
# Shared codec of the Sherlock e2e oracles

Every oracle of `formal/sherlock-e2e/<context>/lean` has a flat observable state:
a record of integer and boolean cells. This module holds the strict JSON decoder
(copied from the software-factory token-bucket example), the cell codec and the
two modes of the executable. It holds no spec logic: each oracle passes its
`Oracle` record, whose functions come from its `Spec.lean`.

Wire forms:

* default mode: one `kink-implementation/1` request on stdin,
  `{"protocol": "kink-implementation/1", "inputs": {<params>, "ops": [op, ...]}}`.
  The answer is `{"end": "final", "cells": {...}}` or
  `{"end": "could-not-apply", "reason": CODES}` (codes sorted, joined by `|`).
* `--step`: JSON Lines `{"state": {...}, "op": INT}` -> `{"state": {...}}`,
  `{"refused": [CODE, ...]}` (state or operation outside the step domain), or
  `{"error": TEXT}` (the line does not decode).

An operation is one integer `code * 1000000000 + arg` with `0 <= arg < 10^9`
(see each ORACLE.md for the codes and the meaning of `arg`).
-/

namespace SherlockCodec
open Lean

inductive Kind where
  | int
  | bool
  deriving DecidableEq

/-- One cell value. -/
inductive V where
  | i (x : Int)
  | b (x : Bool)
  deriving DecidableEq

/-- What a context's `Main.lean` passes to the shared modes. -/
structure Oracle (St : Type) where
  fields : List (String × Kind)
  enc : St → List V
  dec : List V → Option St
  /-- refusal codes of one `--step` line (operation or state outside the domain) -/
  stepRefusals : St → Int → List String
  stepOp : St → Int → St
  /-- the integer parameters of the default mode (besides `ops`) -/
  params : List String
  /-- refusal codes of a default-mode request -/
  refusals : List Int → List Int → List String
  run : List Int → List Int → St

def maxLine : Nat := 1048576

/-- Refuse a fraction or an exponent in any number outside a string. -/
def integerNumbersOnly (t : String) : Except String Unit := Id.run do
  let mut inStr := false
  let mut esc := false
  let mut inNum := false
  for c in t.toList do
    if inStr then
      if esc then esc := false
      else if c == '\\' then esc := true
      else if c == '"' then inStr := false
    else if c == '"' then inStr := true; inNum := false
    else if c.isDigit || c == '-' then inNum := true
    else if inNum && (c == '.' || c == 'e' || c == 'E' || c == '+') then
      return .error "numbers must be integers (no fraction or exponent)"
    else inNum := false
  return .ok ()

namespace StrictJson
open Std.Internal.Parsec Std.Internal.Parsec.String

mutual
  partial def value : Parser Json := do
    let c ← peek!
    if c == '{' then
      skip; ws
      if (← peek!) == '}' then
        skip; ws
        return Json.obj ∅
      return Json.obj (← object ∅)
    else if c == '[' then
      skip; ws
      if (← peek!) == ']' then
        skip; ws
        return Json.arr #[]
      return Json.arr (← array #[])
    else
      Json.Parser.anyCore

  partial def object (fields : Std.TreeMap.Raw String Json) :
      Parser (Std.TreeMap.Raw String Json) := do
    Json.Parser.lookahead (· == '"') "\""; skip
    let key ← Json.Parser.str
    if fields.contains key then fail s!"duplicate JSON key: {key}"
    ws; Json.Parser.lookahead (· == ':') ":"; skip; ws
    let item ← value
    let separator ← any
    if separator == '}' then
      ws
      return fields.insert key item
    else if separator == ',' then
      ws
      object (fields.insert key item)
    else
      fail "unexpected character in object"

  partial def array (items : Array Json) : Parser (Array Json) := do
    let item ← value
    let separator ← any
    if separator == ']' then
      ws
      return items.push item
    else if separator == ',' then
      ws
      array (items.push item)
    else
      fail "unexpected character in array"
end

def parse (text : String) : Except String Json :=
  Parser.run (do ws; let result ← value; eof; return result) text
end StrictJson

def parseLine (t : String) : Except String Json := do
  if t.utf8ByteSize > maxLine then throw s!"request longer than {maxLine} bytes"
  integerNumbersOnly t
  StrictJson.parse t

def canonicalDecimal (t : String) : Bool :=
  let d := if t.startsWith "-" then t.drop 1 else t
  !d.isEmpty && d.all Char.isDigit && (d == "0" || !d.startsWith "0") && !(t.startsWith "-" && d == "0")

def intOf (k : String) : Json → Except String Int
  | .str t =>
    if !canonicalDecimal t then .error s!"{k}: not a canonical decimal integer: {t}"
    else match t.toInt? with
      | some i => .ok i
      | none => .error s!"{k}: not a decimal integer: {t}"
  | .num n => if n.exponent != 0 then .error s!"{k}: not an integer" else .ok n.mantissa
  | v => .error s!"{k}: not an integer: {v.compress}"

def getInt (j : Json) (k : String) : Except String Int := do intOf k (← j.getObjVal? k)

/-- The object has exactly these keys. -/
def exactKeys (j : Json) (what : String) (keys : List String) : Except String Unit := do
  let obj ← j.getObj?
  let got := (obj.toArray.map (·.1)).toList
  if got.length != keys.length || !(got.all keys.contains) || !(keys.all got.contains) then
    throw s!"{what}: keys must be exactly {keys}, got {got}"

def cellOf (j : Json) : String × Kind → Except String V
  | (k, .int) => do return .i (← getInt j k)
  | (k, .bool) => do
    match ← j.getObjVal? k with
    | .bool b => return .b b
    | v => throw s!"{k}: not a boolean: {v.compress}"

def cellJson : V → Json
  | .i x => toJson x
  | .b x => toJson x

variable {St : Type}

def stJson (o : Oracle St) (s : St) : Json :=
  Json.mkObj ((o.fields.zip (o.enc s)).map fun (f, v) => (f.1, cellJson v))

def stOfJson (o : Oracle St) (j : Json) : Except String St := do
  exactKeys j "state" (o.fields.map (·.1))
  let vs ← o.fields.mapM (cellOf j)
  match o.dec vs with
  | some s => return s
  | none => throw "state: cells do not decode"

def reason (rs : List String) : String := "|".intercalate ((rs.toArray.qsort (· < ·)).toList)

/-- Answer one kink-implementation/1 request. -/
def answer (o : Oracle St) (req : Json) : Except String Json := do
  exactKeys req "request" ["protocol", "inputs"]
  let proto ← req.getObjValAs? String "protocol"
  if proto != "kink-implementation/1" then throw s!"unsupported protocol {proto}"
  let inputs ← req.getObjVal? "inputs"
  exactKeys inputs "inputs" (o.params ++ ["ops"])
  let ps ← o.params.mapM (getInt inputs)
  let opsJ ← (← inputs.getObjVal? "ops").getArr?
  let ops ← opsJ.toList.mapM (intOf "ops")
  match o.refusals ps ops with
  | [] => return Json.mkObj [("end", "final"), ("cells", stJson o (o.run ps ops))]
  | rs => return Json.mkObj [("end", "could-not-apply"), ("reason", reason rs)]

/-- One `--step` line: a decode error, a refusal, or the next state. -/
def stepAnswer (o : Oracle St) (line : String) : Json :=
  match (do
      let j ← parseLine line
      exactKeys j "request" ["state", "op"]
      let s ← stOfJson o (← j.getObjVal? "state")
      let op ← getInt j "op"
      pure (s, op)) with
  | .error e => Json.mkObj [("error", toJson e)]
  | .ok (s, op) =>
    match o.stepRefusals s op with
    | [] => Json.mkObj [("state", stJson o (o.stepOp s op))]
    | rs => Json.mkObj [("refused", toJson rs)]

partial def stepLoop (o : Oracle St) (stdin : IO.FS.Stream) (stdout : IO.FS.Stream) : IO Unit := do
  let line ← stdin.getLine
  if line.isEmpty then return
  if line.trimAscii.isEmpty then return (← stepLoop o stdin stdout)
  stdout.putStrLn (stepAnswer o line).compress
  stdout.flush
  stepLoop o stdin stdout

def main (o : Oracle St) (args : List String) : IO UInt32 := do
  let stdin ← IO.getStdin
  let stdout ← IO.getStdout
  if args == ["--step"] then
    stepLoop o stdin stdout
    return 0
  if args == ["--fields"] then
    stdout.putStrLn (Json.arr (o.fields.map fun (f, k) =>
      Json.mkObj [("name", toJson f), ("kind", toJson (if k == .int then "int" else "bool"))]).toArray).compress
    return 0
  let input ← stdin.readToEnd
  match parseLine input >>= answer o with
  | .ok out => stdout.putStrLn out.compress; return 0
  | .error e => IO.eprintln s!"oracle: {e}"; return 2

end SherlockCodec
