requires "c6-typed-root-syntax.k"

// Distinct positive CTX vocabulary. No admission/refusal equations.
module SHERLOCK-C6-POSITIVE-CONTEXT-SYNTAX
  imports SHERLOCK-C6-TYPED-ROOT-SYNTAX
  syntax CXExtFields ::= cxExt(Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) [klabel(cxExt)]
  syntax CXBudgets ::= cxBudgets(Int, Int) [klabel(cxBudgets)]
  syntax CXBounds ::= cxBounds(Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) [klabel(cxBounds)]
  syntax CXLayoutCaps ::= cxLayoutCaps(Int, Int, Int, Int) [klabel(cxLayoutCaps)]
  syntax CXHostLimits ::= cxHostLimits(Int, Int, Int, Int, Int) [klabel(cxHostLimits)]
  syntax CXPhysCaps ::= cxPhysCaps(Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) [klabel(cxPhysCaps)]
  syntax CXExt ::= cxAbsent() [klabel(cxAbsent)]
  syntax CXExt ::= cxPresent(CXExtFields) [klabel(cxPresent)]
  syntax CXPayload ::= cxPayload(CXExt, CXBudgets) [klabel(cxPayload)]
  syntax CXInput ::= cxInput(CXPayload, CXBounds, CXLayoutCaps, CXHostLimits) [klabel(cxInput)]
  syntax CXStateCaps ::= cxStateCaps(CXPhysCaps, Int, Int) [klabel(cxStateCaps)]
  syntax CXStateLimits ::= cxStateLimits(Int) [klabel(cxStateLimits)]
  syntax CXStateView ::= cxStateView(CXStateCaps, Bool, CXStateLimits) [klabel(cxStateView)]
  syntax CXObservation ::= cxObservation(CTLanes, CXPayload, CXExtFields, CXStateView, CTLanes) [klabel(cxObservation)]
  syntax CXCases ::= cxNoCases() [klabel(cxNoCases)]
  syntax CXCases ::= cxCase(String, CXInput, CXCases) [klabel(cxCase)]
  syntax CXObservations ::= cxNoObservations() [klabel(cxNoObservations)]
  syntax CXObservations ::= cxObservationCons(CXObservation, CXObservations) [klabel(cxObservationCons)]
endmodule
