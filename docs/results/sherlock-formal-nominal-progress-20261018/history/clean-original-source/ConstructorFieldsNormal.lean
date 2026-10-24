module

public import Clean.Sherlock.ConstructorFields
public import Clean.Sherlock.Semantics

@[expose] public section

/-!
Normal source-only checks using the existing reference Expr/Value/Env carriers. These expected
cursors are literal independent observations. No compiler, evaluator or candidate result was used
to compute the expected expressions, values or environments. Kernel checking is not yet run.
-/

namespace Clean.Sherlock.ConstructorFieldsNormal

open Clean.Sherlock.CanonicalTypeId
open Clean.Sherlock.ConstructorFields
open Clean.Sherlock.Semantics

def saved : Env := [(10, .uint 7), (20, .bool true)]

def firstExpression : Expr := .var 10
def secondExpression : Expr := .literal (.bool false)

def initial : Cursor Expr Value Env :=
  beginFields ⟨9⟩ ⟨12⟩ 37 [firstExpression, secondExpression] saved

/-- A cut after one child retains the first value, second expression and complete caller env. -/
theorem first_return_retains_pending_second :
    returnField initial (.uint 7) = some (.eval secondExpression saved
      { typeId := ⟨9⟩, ctorId := ⟨12⟩, origin := 37, completedValues := [.uint 7],
        remainingExprs := [], savedEnvironment := saved }) := rfl

/-- Final Build receives UInt followed by Bool, with nominal identities above builtin rows. -/
theorem two_fields_build_in_source_order :
    runReturned [.uint 7, .bool false] initial = some (.build
      { typeId := ⟨9⟩, ctorId := ⟨12⟩, origin := 37, fields := [.uint 7, .bool false],
        savedEnvironment := saved }) := rfl

/-- Nullary constructors schedule Build without evaluating a fabricated scalar expression. -/
theorem nullary_build :
    beginFields (Expr := Expr) (Value := Value) ⟨4⟩ ⟨6⟩ 38 [] saved = .build
      { typeId := ⟨4⟩, ctorId := ⟨6⟩, origin := 38, fields := [],
        savedEnvironment := saved } := rfl

/-- An empty technical observation returns the exact pending continuation. -/
theorem zero_returns_preserve_cursor : runReturned [] initial = some initial := rfl

end Clean.Sherlock.ConstructorFieldsNormal
