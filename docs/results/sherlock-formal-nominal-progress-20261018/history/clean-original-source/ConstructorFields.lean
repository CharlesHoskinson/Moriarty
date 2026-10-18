module

public import Clean.Sherlock.CanonicalTypeId

@[expose] public section

/-!
# Generic constructor-field continuation semantics

The callable `beginFields`/`returnField` slice implements the reference continuation from the
adopted interface: canonical type and constructor identity, source occurrence, saved caller
environment, evaluated value prefix and remaining expression suffix. A returned value is appended
once, and the next expression is scheduled in the saved environment. The final state is a Build
request, before value allocation or validation. No source expression is pre-evaluated here.

`Expr`, `Value` and `Env` are explicit semantic carriers, so generic nominal values and recursive
field expressions do not have to be flattened to the older UInt-list/tree source types. The
conditional field-evaluation relation states the child evaluations independently. It supplies no
termination, fuel, allocation, capacity, constructor-layout validation or B-helper charge claim.

These are semantic definitions and order/preservation proofs. Production Rust still refuses the
reserved ConstructorFields frame in the inspected source. A production adapter and the bounded
Clean circuit/constraint correspondence remain separate unimplemented obligations.
-/

namespace Clean.Sherlock.ConstructorFields

open Clean.Sherlock.CanonicalTypeId

variable {Expr Value Env : Type}

structure Frame (Expr Value Env : Type) where
  typeId : TypeId
  ctorId : CtorId
  origin : Nat
  completedValues : List Value
  remainingExprs : List Expr
  savedEnvironment : Env

/-- Input to the later Build helper, retaining identity, complete fields and caller context. -/
structure BuildInput (Value Env : Type) where
  typeId : TypeId
  ctorId : CtorId
  origin : Nat
  fields : List Value
  savedEnvironment : Env

inductive Cursor (Expr Value Env : Type) where
  | eval (expression : Expr) (environment : Env) (frame : Frame Expr Value Env)
  | build (input : BuildInput Value Env)

/-- Schedule the next source expression, or request Build when no fields remain. -/
def schedule (frame : Frame Expr Value Env) : Cursor Expr Value Env :=
  match frame.remainingExprs with
  | [] => .build
      { typeId := frame.typeId, ctorId := frame.ctorId, origin := frame.origin,
        fields := frame.completedValues, savedEnvironment := frame.savedEnvironment }
  | expression :: rest => .eval expression frame.savedEnvironment
      { frame with remainingExprs := rest }

/-- Start at an explicit evaluated prefix. Technical cuts retain this complete prefix. -/
def beginFrom (typeId : TypeId) (ctorId : CtorId) (origin : Nat)
    (prefix : List Value) (expressions : List Expr) (saved : Env) : Cursor Expr Value Env :=
  schedule { typeId, ctorId, origin, completedValues := prefix,
    remainingExprs := expressions, savedEnvironment := saved }

def beginFields (typeId : TypeId) (ctorId : CtorId) (origin : Nat)
    (expressions : List Expr) (saved : Env) : Cursor Expr Value Env :=
  beginFrom typeId ctorId origin [] expressions saved

/-- Consume one completed child evaluation. Build cannot consume another field return. -/
def returnField (cursor : Cursor Expr Value Env) (value : Value) :
    Option (Cursor Expr Value Env) :=
  match cursor with
  | .eval _ _ frame => some (schedule
      { frame with completedValues := frame.completedValues ++ [value] })
  | .build _ => none

/-- Replay only completed field returns; this never evaluates any child expression. -/
def runReturned (values : List Value) (cursor : Cursor Expr Value Env) :
    Option (Cursor Expr Value Env) :=
  match values with
  | [] => some cursor
  | value :: rest => do
    let next ← returnField cursor value
    runReturned rest next

/-- Identity, source origin and the complete saved environment of a live or Build cursor. -/
def context (cursor : Cursor Expr Value Env) : TypeId × CtorId × Nat × Env :=
  match cursor with
  | .eval _ _ frame => (frame.typeId, frame.ctorId, frame.origin, frame.savedEnvironment)
  | .build input => (input.typeId, input.ctorId, input.origin, input.savedEnvironment)

theorem context_schedule (frame : Frame Expr Value Env) :
    context (schedule frame) =
      (frame.typeId, frame.ctorId, frame.origin, frame.savedEnvironment) := by
  cases h : frame.remainingExprs <;> simp [schedule, h, context]

/-- A completed field cannot replace any constructor identity or saved caller context. -/
theorem returnField_preserves_context (cursor next : Cursor Expr Value Env) (value : Value)
    (h : returnField cursor value = some next) : context next = context cursor := by
  cases cursor with
  | build input => cases h
  | eval expression environment frame =>
    change some (schedule { frame with completedValues := frame.completedValues ++ [value] }) =
      some next at h
    cases h
    exact context_schedule _

/-- With the full field count, returned values accumulate exactly in source order. -/
theorem runReturned_complete (typeId : TypeId) (ctorId : CtorId) (origin : Nat)
    (expressions : List Expr) (values : List Value) (saved : Env) (prefix : List Value)
    (hLength : expressions.length = values.length) :
    runReturned values (beginFrom typeId ctorId origin prefix expressions saved) =
      some (.build { typeId, ctorId, origin, fields := prefix ++ values,
        savedEnvironment := saved }) := by
  induction expressions generalizing values prefix with
  | nil =>
    cases values with
    | nil => rfl
    | cons value rest => simp at hLength
  | cons expression expressions ih =>
    cases values with
    | nil => simp at hLength
    | cons value values =>
      have hTail : expressions.length = values.length := Nat.succ.inj hLength
      change runReturned values
        (beginFrom typeId ctorId origin (prefix ++ [value]) expressions saved) = _
      rw [ih values (prefix ++ [value]) hTail]
      simp only [List.append_assoc, List.singleton_append]

/-- Each child starts in the saved environment, with field expressions and results in order. -/
inductive FieldsEvaluate (evaluates : Env → Expr → Value → Prop) (saved : Env) :
    List Expr → List Value → Prop where
  | nil : FieldsEvaluate evaluates saved [] []
  | cons {expression : Expr} {value : Value} {expressions : List Expr} {values : List Value}
      (head : evaluates saved expression value)
      (tail : FieldsEvaluate evaluates saved expressions values) :
      FieldsEvaluate evaluates saved (expression :: expressions) (value :: values)

theorem FieldsEvaluate.length_eq {evaluates : Env → Expr → Value → Prop} {saved : Env}
    {expressions : List Expr} {values : List Value}
    (h : FieldsEvaluate evaluates saved expressions values) :
    expressions.length = values.length := by
  induction h with
  | nil => rfl
  | cons head tail ih => exact congrArg Nat.succ ih

/-- Conditional semantic theorem: actual child meanings produce exactly the Build field order. -/
theorem completed_fields_meaning (evaluates : Env → Expr → Value → Prop)
    (typeId : TypeId) (ctorId : CtorId) (origin : Nat) (saved : Env)
    (expressions : List Expr) (values : List Value)
    (h : FieldsEvaluate evaluates saved expressions values) :
    runReturned values (beginFields typeId ctorId origin expressions saved) =
      some (.build { typeId, ctorId, origin, fields := values, savedEnvironment := saved }) := by
  exact runReturned_complete typeId ctorId origin expressions values saved [] h.length_eq

/-- A technical split between returned fields composes on the exact cursor, including pending
expression, saved environment, prefix and suffix; it does not construct a financial boundary. -/
theorem runReturned_append (left right : List Value) (cursor : Cursor Expr Value Env) :
    runReturned (left ++ right) cursor =
      (runReturned left cursor).bind (runReturned right) := by
  induction left generalizing cursor with
  | nil => rfl
  | cons value rest ih =>
    cases h : returnField cursor value with
    | none => simp [runReturned, h]
    | some next => simp [runReturned, h, ih]

end Clean.Sherlock.ConstructorFields
