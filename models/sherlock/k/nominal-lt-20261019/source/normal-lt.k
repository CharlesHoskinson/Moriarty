requires "nominal-product.k"
requires "normal-lt-syntax.k"

// Uncharged ordinary semantic counterpart only. Scalar Lt mode3/op4 is a
// correspondence label, never an identity with B helper/cost clocks.
module SHERLOCK-NOMINAL-LT
  imports SHERLOCK-NOMINAL-PRODUCT
  imports SHERLOCK-NOMINAL-LT-SYNTAX

  rule ngOccurrence(ngFoundExpr(ngLt(O, _, _))) => O
  rule ngEvaluate(ngFoundExpr(ngLt(O, L, R)), E, P, Rho, Fs, Es)
    => ngState(P, ngEval(L), Rho, ngFrame(ngLtLeft(E, O, R, Rho), Fs),
         ngAppendEvent(Es, ngLtStarted(E, O, L, R, Rho)))
  rule ngStep(ngState(P, ngReturn(V), _, ngFrame(ngLtLeft(E, O, R, Saved), Fs), Es))
    => ngState(P, ngEval(R), Saved, ngFrame(ngLtRight(E, O, V, Saved), Fs),
         ngAppendEvent(Es, ngLtRightRequested(E, O, V, R, Saved)))

  // Nominal values are admitted/canonical by the existing input premise.
  // This checks only the scalar UInt representation, not an admission policy.
  syntax Bool ::= ngLtMalformedUInt(NGValue) [function]
  rule ngLtMalformedUInt(ngUInt(N))
    => N <Int 0 orBool N >=Int 340282366920938463463374607431768211456
  rule ngLtMalformedUInt(ngBool(_)) => false
  rule ngLtMalformedUInt(ngNominal(_, _, _)) => false

  syntax NGState ::= ngCompleteLt(NGValue, NGValue, Int, Int, NGProgram, Map,
                                  NGFrames, NGEvents) [function]
  rule ngStep(ngState(P, ngReturn(R), _, ngFrame(ngLtRight(E, O, L, Saved), Fs), Es))
    => ngCompleteLt(L, R, E, O, P, Saved, Fs, Es)
  // Pop only the Lt frame. Every enclosing caller/field/scope frame survives.
  rule ngCompleteLt(ngUInt(L), ngUInt(R), E, O, P, Saved, Fs, Es)
    => ngState(P, ngLtPrimitiveValue(E, O, ngBool(L <Int R)), Saved, Fs,
         ngAppendEvent(Es, ngLtCompared(E, O, ngUInt(L), ngUInt(R),
                                       ngBool(L <Int R), Saved)))
    requires L >=Int 0 andBool R >=Int 0
      andBool L <Int 340282366920938463463374607431768211456
      andBool R <Int 340282366920938463463374607431768211456
  // Wrong canonical kind is source sort error11, never overflow10.
  rule ngCompleteLt(L, R, E, O, P, Saved, Fs, Es)
    => ngState(P, ngLtPrimitiveError(E, O, 11), Saved, Fs,
         ngAppendEvent(Es, ngLtSortFailed(E, O, L, R, 11, Saved)))
    requires notBool ngLtMalformedUInt(L) andBool notBool ngLtMalformedUInt(R)
      andBool notBool (ngIsUInt(L) andBool ngIsUInt(R))
  // Malformed raw scalar Int is outside the admitted UInt128 profile. Keep
  // the pending LtRight carrier rather than publish error11 or clamp values.
  rule ngCompleteLt(L, R, E, O, P, Saved, Fs, Es)
    => ngOutside(ngState(P, ngReturn(R), Saved,
         ngFrame(ngLtRight(E, O, L, Saved), Fs), Es),
         "lt_operand_outside_UInt128_admitted_domain")
    requires ngLtMalformedUInt(L) orBool ngLtMalformedUInt(R)

  // Separate primitive value/error steps retain complete current environments
  // and outer frames. Existing typed FunctionReturn/ConstructorFields rules
  // subsequently consume Bool success. Error does not unwind caller frames.
  rule ngStep(ngState(P, ngLtPrimitiveValue(E, O, ngBool(B)), Rho, Fs, Es))
    => ngState(P, ngReturn(ngBool(B)), Rho, Fs,
         ngAppendEvent(Es, ngLtPrimitiveReturned(E, O, ngBool(B), Rho)))
  rule ngStep(ngState(P, ngLtPrimitiveValue(E, O, V), Rho, Fs, Es))
    => ngOutside(ngState(P, ngLtPrimitiveValue(E, O, V), Rho, Fs, Es),
                  "lt_primitive_value_carrier_not_Bool")
    requires notBool ngIsBool(V)
  rule ngStep(ngState(P, ngLtPrimitiveError(E, O, 11), Rho, Fs, Es))
    => ngState(P, ngLtError(E, O, 11), Rho, Fs,
         ngAppendEvent(Es, ngLtPrimitiveFailed(E, O, 11, Rho)))
  rule ngStep(ngState(P, ngLtPrimitiveError(E, O, Code), Rho, Fs, Es))
    => ngOutside(ngState(P, ngLtPrimitiveError(E, O, Code), Rho, Fs, Es),
                  "lt_primitive_error_code_outside_profile") requires Code =/=Int 11

  rule ngTerminal(ngLtPrimitiveValue(_, _, _)) => false
  rule ngTerminal(ngLtPrimitiveError(_, _, _)) => false
  rule ngTerminal(ngLtError(_, _, 11)) => true
  rule ngTerminal(ngLtError(_, _, Code)) => false requires Code =/=Int 11
  rule ngStep(ngState(P, ngLtError(E, O, 11), Rho, Fs, Es))
    => ngState(P, ngLtError(E, O, 11), Rho, Fs, Es)
  rule ngAdvance(ngState(P, ngLtError(E, O, 11), Rho, Fs, Es), N)
    => ngState(P, ngLtError(E, O, 11), Rho, Fs, Es) requires N >Int 0
  rule ngStep(ngState(P, ngLtError(E, O, Code), Rho, Fs, Es))
    => ngOutside(ngState(P, ngLtError(E, O, Code), Rho, Fs, Es),
                  "lt_error_boundary_code_outside_profile") requires Code =/=Int 11
endmodule
