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

/-- Exact failed C02 shape: Expr annotation does not change inherited return continuation. -/
def decodeRow (flag : Bool) : Except String ExprRow := do
  let expression : Expr ← do
    match flag with
    | true =>
      let number ← readNumber
      return Expr.literal number
    | false => return Expr.variable 7
  return {
    occurrence := 19
    expression
  }

end CleanDoReturnRepro
