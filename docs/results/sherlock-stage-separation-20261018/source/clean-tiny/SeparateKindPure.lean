module

@[expose] public section

namespace CleanDoReturnRepro

inductive Expr where
  | literal : Nat → Expr
  | variable : Nat → Expr
  deriving DecidableEq

structure ExprRow where
  occurrence : Nat
  expression : Expr
  deriving DecidableEq

def readNumber : Except String Nat := pure 4

/-- Separate declaration owns Expr result. Branches use ordinary pure, no early return. -/
def decodeKind (flag : Bool) : Except String Expr := do
  match flag with
  | true =>
    let number ← readNumber
    pure (Expr.literal number)
  | false => pure (Expr.variable 7)

def decodeRow (flag : Bool) : Except String ExprRow := do
  let expression ← decodeKind flag
  pure {
    occurrence := 19
    expression
  }

/-- Own declaration also handles String-tag matching and nested Except actions. -/
def decodeTaggedKind (tag : String) : Except String Expr := do
  match tag with
  | "Literal" =>
    let number ← readNumber
    pure (Expr.literal number)
  | "Variable" => pure (Expr.variable (← readNumber))
  | _ => throw "unsupported source expression tag"

def decodeTaggedRow (tag : String) : Except String ExprRow := do
  let expression ← decodeTaggedKind tag
  pure {
    occurrence := 23
    expression
  }

example : decodeRow true = .ok ⟨19, .literal 4⟩ := rfl
example : decodeRow false = .ok ⟨19, .variable 7⟩ := rfl
example : decodeTaggedRow "Literal" = .ok ⟨23, .literal 4⟩ := rfl
example : decodeTaggedRow "Variable" = .ok ⟨23, .variable 4⟩ := rfl
example : decodeTaggedRow "Unsupported" = .error "unsupported source expression tag" := rfl

end CleanDoReturnRepro
