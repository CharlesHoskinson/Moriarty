requires "../source/programview-state-event-source11.k"
// Author-visible synthetic source-first examples, not independent oracle data.
module SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11-LAWS
 imports SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11
 syntax NGProgram ::= nwHeldOutProgram() [function]
 rule nwHeldOutProgram() => ngProgram(ngNoDecls(),
   ngNode(70,ngCall(900,13,ngExprId(71,ngNoExprs())),
   ngNode(71,ngLiteral(901,ngUInt(19)),
   ngNode(72,ngVariable(902,50),ngNoNodes()))),
   ngFunctionRow(ngFunction(13,ngParam(50,0,ngNoParams()),0,72),ngNoFunctions()),70)
 syntax NIJSON ::= nwHeldOutSource() [function]
 rule nwHeldOutSource() => niObject(niField("absent",niNull(),
   niField("present_zero",niInt(0),niNoFields())))
 syntax NGState ::= nwS0(NGProgram) [function]
                  | nwS1(NGProgram) [function]
                  | nwS2(NGProgram) [function]
                  | nwS3(NGProgram) [function]
                  | nwS4(NGProgram) [function]
 rule nwS0(P) => ngState(P,ngEval(70),.Map,ngNoFrames(),ngNoEvents())
 rule nwS1(P) => ngState(P,ngEval(71),.Map,
   ngFrame(ngArguments(70,900,13,0,71,ngNoValues(),ngNoExprs(),.Map),ngNoFrames()),
   ngEvent(ngCallStarted(70,900,13,ngExprId(71,ngNoExprs()),.Map),ngNoEvents()))
 rule nwS2(P) => ngState(P,ngReturn(ngUInt(19)),.Map,
   ngFrame(ngArguments(70,900,13,0,71,ngNoValues(),ngNoExprs(),.Map),ngNoFrames()),
   ngEvent(ngCallStarted(70,900,13,ngExprId(71,ngNoExprs()),.Map),
   ngEvent(ngLiteralEvaluated(71,901,ngUInt(19)),ngNoEvents())))
 rule nwS3(P) => ngState(P,ngApply(70,900,13,ngValue(ngUInt(19),ngNoValues())),.Map,ngNoFrames(),
   ngEvent(ngCallStarted(70,900,13,ngExprId(71,ngNoExprs()),.Map),
   ngEvent(ngLiteralEvaluated(71,901,ngUInt(19)),
   ngEvent(ngArgumentReturned(70,900,0,71,ngUInt(19),ngValue(ngUInt(19),ngNoValues()),ngNoExprs(),.Map),ngNoEvents()))))
 rule nwS4(P) => ngState(P,ngEval(72),50 |-> ngUInt(19),
   ngFrame(ngFunctionReturn(70,900,13,0,.Map),ngNoFrames()),
   ngEvent(ngCallStarted(70,900,13,ngExprId(71,ngNoExprs()),.Map),
   ngEvent(ngLiteralEvaluated(71,901,ngUInt(19)),
   ngEvent(ngArgumentReturned(70,900,0,71,ngUInt(19),ngValue(ngUInt(19),ngNoValues()),ngNoExprs(),.Map),
   ngEvent(ngFunctionEntered(70,900,13,ngValue(ngUInt(19),ngNoValues()),50 |-> ngUInt(19),0),ngNoEvents())))))
 syntax NGTape ::= nwExpectedTape(NGProgram) [function]
 rule nwExpectedTape(P) => ngTape(nwS0(P),ngStateRow(nwS1(P),
   ngStateRow(nwS2(P),ngStateRow(nwS3(P),ngStateRow(nwS4(P),ngNoStates())))),nwS4(P))
 syntax Bool ::= nwLawCompleteFourSteps() [function]
               | nwLawZeroExactSource() [function]
               | nwLawChronologicalTwo() [function]
               | nwLawHaltedNoStutter() [function]
               | nwLawOutsideNoStutter() [function]
               | nwLawDistinctCallerAndTailMaps() [function]
 syntax Bool ::= nwCheckFour(NGProgram) [function]
               | nwCheckZero(NGProgram) [function]
               | nwCheckTwo(NGProgram) [function]
               | nwCheckHalted(NGProgram) [function]
               | nwCheckOutside(NGProgram) [function]
               | nwCheckCaller(NGProgram) [function]
 rule nwLawCompleteFourSteps() => nwCheckFour(nwHeldOutProgram())
 rule nwCheckFour(P) => niRunWindow(nwHeldOutSource(),P,4)
   ==K niWindowObserved(nwHeldOutSource(),nwExpectedTape(P))
 rule nwLawZeroExactSource() => nwCheckZero(nwHeldOutProgram())
 rule nwCheckZero(P) => niRunWindow(nwHeldOutSource(),P,0)
   ==K niWindowObserved(nwHeldOutSource(),ngTape(nwS0(P),ngNoStates(),nwS0(P)))
 rule nwLawChronologicalTwo() => nwCheckTwo(nwHeldOutProgram())
 rule nwCheckTwo(P) => niRunWindow(nwHeldOutSource(),P,2)
   ==K niWindowObserved(nwHeldOutSource(),ngTape(nwS0(P),
     ngStateRow(nwS1(P),ngStateRow(nwS2(P),ngNoStates())),nwS2(P)))
 rule nwLawHaltedNoStutter() => nwCheckHalted(nwHeldOutProgram())
 rule nwCheckHalted(P) => ngObserve(ngState(P,ngHalted(ngUInt(19)),.Map,ngNoFrames(),ngEvent(ngFinished(ngUInt(19)),ngNoEvents())),9)
   ==K ngTape(ngState(P,ngHalted(ngUInt(19)),.Map,ngNoFrames(),ngEvent(ngFinished(ngUInt(19)),ngNoEvents())),ngNoStates(),
              ngState(P,ngHalted(ngUInt(19)),.Map,ngNoFrames(),ngEvent(ngFinished(ngUInt(19)),ngNoEvents())))
 rule nwLawOutsideNoStutter() => nwCheckOutside(nwHeldOutProgram())
 rule nwCheckOutside(P) => ngObserve(ngState(P,ngOutsideModel("synthetic"),.Map,ngNoFrames(),ngEvent(ngOutsideEvent("synthetic"),ngNoEvents())),9)
   ==K ngTape(ngState(P,ngOutsideModel("synthetic"),.Map,ngNoFrames(),ngEvent(ngOutsideEvent("synthetic"),ngNoEvents())),ngNoStates(),
              ngState(P,ngOutsideModel("synthetic"),.Map,ngNoFrames(),ngEvent(ngOutsideEvent("synthetic"),ngNoEvents())))
 syntax NGState ::= nwCallerBefore(NGProgram) [function]
                  | nwCallerAfter(NGProgram) [function]
 rule nwCallerBefore(P) => ngState(P,ngApply(70,900,13,ngValue(ngUInt(19),ngNoValues())),9 |-> ngUInt(6),
   ngFrame(ngLetScope(44,440,8,8 |-> ngUInt(5)),ngNoFrames()),ngNoEvents())
 rule nwCallerAfter(P) => ngState(P,ngEval(72),50 |-> ngUInt(19),
   ngFrame(ngFunctionReturn(70,900,13,0,9 |-> ngUInt(6)),
   ngFrame(ngLetScope(44,440,8,8 |-> ngUInt(5)),ngNoFrames())),
   ngEvent(ngFunctionEntered(70,900,13,ngValue(ngUInt(19),ngNoValues()),50 |-> ngUInt(19),0),ngNoEvents()))
 rule nwLawDistinctCallerAndTailMaps() => nwCheckCaller(nwHeldOutProgram())
 rule nwCheckCaller(P) => ngObserve(nwCallerBefore(P),1)
   ==K ngTape(nwCallerBefore(P),ngStateRow(nwCallerAfter(P),ngNoStates()),nwCallerAfter(P))
endmodule
