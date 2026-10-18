module

public import Std

@[expose] public section

/-!
# Sherlock pure first-order reference machine

Generic source programs are data supplied to one evaluator. The machine follows the staged
control/fuel conventions of the reviewed draft K fragment. This module defines executable reference
semantics, not a circuit relation or correspondence proof. Financial effects and await are absent.
-/

namespace Clean.Sherlock.Semantics

abbrev Name := Int

inductive UIntList where
  | nil
  | cons (head : Int) (tail : UIntList)
  deriving Repr, BEq

inductive UIntTree where
  | leaf (value : Int)
  | branch (left right : UIntTree)
  deriving Repr, BEq

inductive Value where
  | uint (value : Int)
  | bool (value : Bool)
  | list (value : UIntList)
  | tree (value : UIntTree)
  deriving Repr, BEq

inductive ValueType where
  | uint | bool | list | tree
  deriving Repr, BEq

inductive BinaryOp where
  | add | sub | mul | eq | lt | cons | branch
  deriving Repr, BEq

inductive UnaryOp where
  | leaf | not
  deriving Repr, BEq

inductive Pattern where
  | bind (name : Name)
  | nil
  | cons (head tail : Name)
  | leaf (value : Name)
  | branch (left right : Name)
  deriving Repr, BEq

inductive Expr where
  | literal (value : Value)
  | var (name : Name)
  | letIn (name : Name) (bound body : Expr)
  | binary (op : BinaryOp) (left right : Expr)
  | unary (op : UnaryOp) (argument : Expr)
  | ite (condition yes no : Expr)
  | call (name : Name) (arguments : List Expr)
  | matchValue (scrutinee : Expr) (cases : List (Pattern × Expr))
  deriving Repr

structure Function where
  name : Name
  parameters : List (Name × ValueType)
  resultType : ValueType
  body : Expr
  deriving Repr

abbrev Program := List Function
abbrev Env := List (Name × Value)

inductive Error where
  | invalidBudget | invalidLiteral | unboundVariable | stackLimit | nonBoolCondition
  | resultType | missingFunction | argumentBinding | noMatch | arithmeticRange | operandType
  deriving Repr, BEq

/-- Draft-fragment diagnostic codes; these are not the public Sherlock diagnostic ABI. -/
def Error.code : Error → Nat
  | .invalidBudget => 1
  | .invalidLiteral => 2
  | .unboundVariable => 3
  | .stackLimit => 4
  | .nonBoolCondition => 5
  | .resultType => 6
  | .missingFunction => 7
  | .argumentBinding => 8
  | .noMatch => 9
  | .arithmeticRange => 10
  | .operandType => 11

inductive Frame where
  | letBound (name : Name) (body : Expr) (saved : Env)
  | binaryLeft (op : BinaryOp) (right : Expr) (saved : Env)
  | binaryRight (op : BinaryOp) (left : Value)
  | unary (op : UnaryOp)
  | condition (yes no : Expr) (saved : Env)
  | matching (cases : List (Pattern × Expr)) (saved : Env)
  | arguments (name : Name) (done : List Value) (todo : List Expr) (saved : Env)
  | functionReturn (expected : ValueType) (saved : Env)
  | scope (saved : Env)
  deriving Repr

inductive Control where
  | eval (expression : Expr)
  | ret (value : Value)
  | push (frame : Frame) (next : Control) (environment : Env)
  | invoke (name : Name) (arguments : List Value)
  | enter (function : Option Function) (arguments : List Value)
  | bound (bindings : Option Env) (expected : ValueType) (body : Expr)
  | cases (value : Value) (remaining : List (Pattern × Expr)) (saved : Env)
  | caseResult (bindings : Option Env) (value : Value) (body : Expr)
      (remaining : List (Pattern × Expr)) (saved : Env)
  | primitive (result : Except Error Value)
  | error (reason : Error)
  deriving Repr

inductive Outcome where
  | pending
  | done (value : Value)
  | rejected (reason : Error)
  | exhausted
  deriving Repr, BEq

inductive Status where
  | running | halted | rejected | exhausted
  deriving Repr, BEq

structure Machine where
  program : Program
  control : Control
  environment : Env := []
  stack : List Frame := []
  fuel : Nat
  initialFuel : Nat
  stackLimit : Nat
  outcome : Outcome := .pending
  deriving Repr

def Machine.status (machine : Machine) : Status :=
  match machine.outcome with
  | .pending => .running
  | .done _ => .halted
  | .rejected _ => .rejected
  | .exhausted => .exhausted

def Machine.consumed (machine : Machine) : Nat := machine.initialFuel - machine.fuel

def uintLimit : Int := 340282366920938463463374607431768211456

def validUInt (value : Int) : Bool := decide (0 ≤ value ∧ value < uintLimit)

def validList : UIntList → Bool
  | .nil => true
  | .cons head tail => validUInt head && validList tail

def validTree : UIntTree → Bool
  | .leaf value => validUInt value
  | .branch left right => validTree left && validTree right

def validValue : Value → Bool
  | .uint value => validUInt value
  | .bool _ => true
  | .list value => validList value
  | .tree value => validTree value

def hasType : Value → ValueType → Bool
  | .uint _, .uint => true
  | .bool _, .bool => true
  | .list _, .list => true
  | .tree _, .tree => true
  | _, _ => false

def lookup (name : Name) : Env → Option Value
  | [] => none
  | (key, value) :: rest => if name == key then some value else lookup name rest

def bind (environment : Env) (name : Name) (value : Value) : Env :=
  (name, value) :: environment.filter (fun entry ↦ entry.1 != name)

def findFunction (name : Name) : Program → Option Function
  | [] => none
  | fn :: rest => if fn.name == name then some fn else findFunction name rest

/-- Parameter binding starts with an empty lexical environment and rejects duplicate names. -/
def bindParameters : List (Name × ValueType) → List Value → Option Env
  | [], [] => some []
  | (name, expected) :: parameters, value :: values =>
    match bindParameters parameters values with
    | none => none
    | some environment =>
      if hasType value expected && (lookup name environment).isNone then
        some (bind environment name value)
      else none
  | _, _ => none

def matchPattern (pattern : Pattern) (value : Value) (environment : Env) : Option Env :=
  match pattern, value with
  | .bind name, value => some (bind environment name value)
  | .nil, .list .nil => some environment
  | .cons head tail, .list (.cons first rest) =>
    if head == tail then none
    else some (bind (bind environment head (.uint first)) tail (.list rest))
  | .leaf name, .tree (.leaf value) => some (bind environment name (.uint value))
  | .branch left right, .tree (.branch first second) =>
    if left == right then none
    else some (bind (bind environment left (.tree first)) right (.tree second))
  | _, _ => none

def checkedUInt (value : Int) : Except Error Value :=
  if validUInt value then .ok (.uint value) else .error .arithmeticRange

def binaryResult : BinaryOp → Value → Value → Except Error Value
  | .eq, left, right => .ok (.bool (left == right))
  | .add, .uint left, .uint right => checkedUInt (left + right)
  | .sub, .uint left, .uint right => checkedUInt (left - right)
  | .mul, .uint left, .uint right => checkedUInt (left * right)
  | .lt, .uint left, .uint right => .ok (.bool (decide (left < right)))
  | .cons, .uint head, .list tail => .ok (.list (.cons head tail))
  | .branch, .tree left, .tree right => .ok (.tree (.branch left right))
  | _, _, _ => .error .operandType

def unaryResult : UnaryOp → Value → Except Error Value
  | .leaf, .uint value => .ok (.tree (.leaf value))
  | .not, .bool value => .ok (.bool (!value))
  | _, _ => .error .operandType

def evalExpression (machine : Machine) (expression : Expr) : Machine :=
  let saved := machine.environment
  let next : Control := match expression with
    | .literal value =>
      if validValue value then .ret value else .error .invalidLiteral
    | .var name => match lookup name saved with
      | some value => .ret value
      | none => .error .unboundVariable
    | .letIn name bound body => .push (.letBound name body saved) (.eval bound) saved
    | .binary op left right => .push (.binaryLeft op right saved) (.eval left) saved
    | .unary op argument => .push (.unary op) (.eval argument) saved
    | .ite condition yes no => .push (.condition yes no saved) (.eval condition) saved
    | .matchValue value cases => .push (.matching cases saved) (.eval value) saved
    | .call name [] => .invoke name []
    | .call name (first :: rest) =>
      .push (.arguments name [] rest saved) (.eval first) saved
  { machine with control := next }

def returnValue (machine : Machine) (value : Value) : Machine :=
  match machine.stack with
  | [] => machine
  | frame :: rest => match frame with
    | .letBound name body saved =>
      { machine with
        control := .eval body
        stack := .scope saved :: rest
        environment := bind saved name value }
    | .binaryLeft op right saved =>
      { machine with
        control := .eval right
        stack := .binaryRight op value :: rest
        environment := saved }
    | .binaryRight op left =>
      { machine with control := .primitive (binaryResult op left value), stack := rest }
    | .unary op =>
      { machine with control := .primitive (unaryResult op value), stack := rest }
    | .condition yes no saved => match value with
      | .bool condition =>
        { machine with
          control := .eval (if condition then yes else no)
          stack := rest
          environment := saved }
      | _ => { machine with control := .error .nonBoolCondition }
    | .matching cases saved =>
      { machine with control := .cases value cases saved, stack := rest, environment := saved }
    | .arguments name done todo saved => match todo with
      | [] =>
        { machine with
          control := .invoke name (done ++ [value])
          stack := rest
          environment := saved }
      | first :: remaining =>
        { machine with
          control := .eval first
          stack := .arguments name (done ++ [value]) remaining saved :: rest
          environment := saved }
    | .functionReturn expected saved =>
      if hasType value expected then
        { machine with control := .ret value, stack := rest, environment := saved }
      else { machine with control := .error .resultType }
    | .scope saved =>
      { machine with control := .ret value, stack := rest, environment := saved }

/-- One already-charged K-style machine stage. Helpers themselves do not consume semantic fuel. -/
def advance (machine : Machine) : Machine :=
  match machine.control with
  | .eval expression => evalExpression machine expression
  | .ret value => returnValue machine value
  | .push frame next environment =>
    if machine.stack.length < machine.stackLimit then
      { machine with control := next, environment := environment, stack := frame :: machine.stack }
    else { machine with control := .error .stackLimit }
  | .invoke name arguments =>
    { machine with control := .enter (findFunction name machine.program) arguments }
  | .enter none _ => { machine with control := .error .missingFunction }
  | .enter (some fn) arguments =>
    { machine with
      control := .bound (bindParameters fn.parameters arguments) fn.resultType fn.body }
  | .bound none _ _ => { machine with control := .error .argumentBinding }
  | .bound (some environment) expected body =>
    { machine with
      control := .push (.functionReturn expected machine.environment) (.eval body) environment }
  | .cases _ [] _ => { machine with control := .error .noMatch }
  | .cases value ((pattern, body) :: rest) saved =>
    { machine with control := .caseResult (matchPattern pattern value saved) value body rest saved }
  | .caseResult none value _ rest saved =>
    { machine with control := .cases value rest saved }
  | .caseResult (some bindings) _ body _ saved =>
    { machine with control := .push (.scope saved) (.eval body) bindings }
  | .primitive (.ok value) => { machine with control := .ret value }
  | .primitive (.error reason) => { machine with control := .error reason }
  | .error _ => machine

/-- Terminal publication costs zero; every other live transition consumes one positive fuel unit. -/
def step (machine : Machine) : Machine :=
  match machine.outcome with
  | .pending => match machine.control, machine.stack with
    | .ret value, [] => { machine with outcome := .done value }
    | .error reason, _ => { machine with outcome := .rejected reason }
    | _, _ => match machine.fuel with
      | 0 => { machine with outcome := .exhausted }
      | remaining + 1 => advance { machine with fuel := remaining }
  | _ => machine

/-- Total bounded driver. The extra step permits zero-cost final publication at fuel zero. -/
def drive : Nat → Machine → Machine
  | 0, machine => machine
  | steps + 1, machine =>
    match machine.outcome with
    | .pending => drive steps (step machine)
    | _ => machine

def run (machine : Machine) : Machine := drive (machine.fuel + 1) machine

/-- Startup uses signed inputs to represent the K fragment's invalid-negative-budget result. -/
def start (program : Program) (expression : Expr) (fuel stackLimit : Int) : Machine :=
  if fuel < 0 ∨ stackLimit < 0 then
    { program := [], control := .error .invalidBudget, fuel := 0,
        initialFuel := 0, stackLimit := 0 }
  else
    { program := program, control := .eval expression, fuel := fuel.toNat,
        initialFuel := fuel.toNat, stackLimit := stackLimit.toNat }

def execute (program : Program) (expression : Expr) (fuel stackLimit : Int) : Machine :=
  run (start program expression fuel stackLimit)

end Clean.Sherlock.Semantics
