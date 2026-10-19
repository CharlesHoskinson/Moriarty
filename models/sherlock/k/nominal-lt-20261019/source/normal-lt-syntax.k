requires "nominal-product-syntax.k"

// Additive observation profile, not a change to the retained positive model.
module SHERLOCK-NOMINAL-LT-SYNTAX
  imports SHERLOCK-NOMINAL-PRODUCT-SYNTAX
  syntax NGExpr ::= ngLt(Int, Int, Int)
  syntax NGFrame ::= ngLtLeft(Int, Int, Int, Map)
                   | ngLtRight(Int, Int, NGValue, Map)
  // These source-bearing controls expose completion/primitive boundaries.
  // ngLtError(11) is an error-control observation boundary, not P1 Rejected.
  syntax NGControl ::= ngLtPrimitiveValue(Int, Int, NGValue)
                     | ngLtPrimitiveError(Int, Int, Int)
                     | ngLtError(Int, Int, Int)
  syntax NGEvent ::= ngLtStarted(Int, Int, Int, Int, Map)
                   | ngLtRightRequested(Int, Int, NGValue, Int, Map)
                   | ngLtCompared(Int, Int, NGValue, NGValue, NGValue, Map)
                   | ngLtSortFailed(Int, Int, NGValue, NGValue, Int, Map)
                   | ngLtPrimitiveReturned(Int, Int, NGValue, Map)
                   | ngLtPrimitiveFailed(Int, Int, Int, Map)
endmodule
