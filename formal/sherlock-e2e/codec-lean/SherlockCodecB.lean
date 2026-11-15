import Lean.Data.Json

/-!
# Shared strict codec of the slice B oracles

Copied from the software-factory worked example `examples/token-bucket/lean/Main.lean`
(strict decoding) and extended with the field helpers and the two modes that every
slice B oracle uses. It holds no spec logic: each context passes its own decoders,
its refusal function, its transition and its encoder.

Modes:
* default: one `kink-implementation/1` request on stdin. Inputs are the fields of the
  context's initial state plus `ops`, a list of operation codes. Answer
  `{"end":"final","cells":St}` or `{"end":"could-not-apply","reason":CODES}`.
* `--step`: JSON Lines `{"state":St,"op":N}` -> `{"state":St}` | `{"refused":[CODE..]}`
  | `{"error":TEXT}`.

An operation is one integer `op = code * 16 + arg` with `arg < 16`, so the Quint
replayer (`scripts/quint_replay.py --op-var lastOp`) can pass it unchanged.
-/

open Lean

namespace SherlockCodecB

def maxLine : Nat := 65536

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

/-! Lean 4.35 stores objects in a TreeMap and overwrites duplicate keys.
Check decoded keys before insertion. Reuse Lean's scalar and string decoders. -/
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
  !t.isEmpty && t.all Char.isDigit && (t == "0" || !t.startsWith "0")

/-- A natural-number value: a JSON integer >= 0 or a canonical decimal string. -/
def natOf (k : String) : Json → Except String Nat
  | .str t =>
    if !canonicalDecimal t then .error s!"{k}: not a canonical decimal natural: {t}"
    else match t.toNat? with
      | some n => .ok n
      | none => .error s!"{k}: not a decimal natural: {t}"
  | .num n =>
    if n.exponent != 0 then .error s!"{k}: not an integer"
    else if n.mantissa < 0 then .error s!"{k}: negative"
    else .ok n.mantissa.toNat
  | v => .error s!"{k}: not an integer: {v.compress}"

def getNat (j : Json) (k : String) : Except String Nat := do natOf k (← j.getObjVal? k)

def getBool (j : Json) (k : String) : Except String Bool := do
  match ← j.getObjVal? k with
  | .bool b => pure b
  | v => throw s!"{k}: not a boolean: {v.compress}"

def getNats (j : Json) (k : String) : Except String (List Nat) := do
  match ← j.getObjVal? k with
  | .arr a => a.toList.mapM (natOf k)
  | v => throw s!"{k}: not an array: {v.compress}"

/-- The object has exactly these keys. -/
def exactKeys (j : Json) (what : String) (keys : List String) : Except String Unit := do
  let obj ← j.getObj?
  let got := (obj.toArray.map (·.1)).toList
  if got.length != keys.length || !(got.all keys.contains) then
    throw s!"{what}: keys must be exactly {keys}, got {got}"

def natJ (n : Nat) : Json := toJson n
def natsJ (l : List Nat) : Json := Json.arr (l.map natJ).toArray

/-- What one context gives the codec. -/
structure Ctx (σ : Type) where
  fields : List String
  decode : Json → Except String σ
  encode : σ → Json
  /-- refusal codes of one operation in one state (state domain included) -/
  stepRefusals : σ → Nat → List String
  step : σ → Nat → σ
  /-- refusal codes of a whole run (init domain); `[]` means the run is applied -/
  runRefusals : σ → List Nat → List String
  run : σ → List Nat → σ

def reason (rs : List String) : String := "|".intercalate rs

def answer {σ} (c : Ctx σ) (req : Json) : Except String Json := do
  let proto ← req.getObjValAs? String "protocol"
  if proto != "kink-implementation/1" then throw s!"unsupported protocol {proto}"
  let inputs ← req.getObjVal? "inputs"
  exactKeys inputs "inputs" (c.fields ++ ["ops"])
  let ops ← getNats inputs "ops"
  -- decode the state part: rebuild an object without `ops`
  let obj ← inputs.getObj?
  let s ← c.decode (Json.obj (obj.erase "ops"))
  match c.runRefusals s ops with
  | [] => return Json.mkObj [("end", "final"), ("cells", c.encode (c.run s ops))]
  | rs => return Json.mkObj [("end", "could-not-apply"), ("reason", reason rs)]

def stepAnswer {σ} (c : Ctx σ) (line : String) : Json :=
  match (do
      let j ← parseLine line
      exactKeys j "request" ["state", "op"]
      let st ← j.getObjVal? "state"
      exactKeys st "state" c.fields
      let s ← c.decode st
      let op ← getNat j "op"
      pure (s, op)) with
  | .error e => Json.mkObj [("error", toJson e)]
  | .ok (s, op) =>
    match c.stepRefusals s op with
    | [] => Json.mkObj [("state", c.encode (c.step s op))]
    | rs => Json.mkObj [("refused", toJson rs)]

partial def stepLoop {σ} (c : Ctx σ) (stdin : IO.FS.Stream) (stdout : IO.FS.Stream) : IO Unit := do
  let line ← stdin.getLine
  if line.isEmpty then return
  if line.trimAscii.isEmpty then return (← stepLoop c stdin stdout)
  stdout.putStrLn (stepAnswer c line).compress
  stdout.flush
  stepLoop c stdin stdout

def mainWith {σ} (c : Ctx σ) (args : List String) : IO UInt32 := do
  let stdin ← IO.getStdin
  let stdout ← IO.getStdout
  if args == ["--step"] then
    stepLoop c stdin stdout
    return 0
  let input ← stdin.readToEnd
  match parseLine input >>= answer c with
  | .ok out => stdout.putStrLn out.compress; return 0
  | .error e => IO.eprintln s!"oracle: {e}"; return 2

end SherlockCodecB
