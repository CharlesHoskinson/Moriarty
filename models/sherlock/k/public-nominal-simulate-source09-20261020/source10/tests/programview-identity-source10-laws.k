requires "../source/programview-identity-source10.k"
module SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10-LAWS
 imports SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10
 syntax Bool ::= niLawRawPresence() [function]
 rule niLawRawPresence() =>
   niSource(niObserved(niObject(niField("decl",niString("0"),niField("other",niNull(),niNoFields()))),
     npObserved(ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngHalted(ngBool(true)),
       .Map,ngNoFrames(),ngNoEvents()),npBool(true))))
   ==K niObject(niField("decl",niString("0"),niField("other",niNull(),niNoFields())))
 syntax Bool ::= niLawSavedStateRetained() [function]
 rule niLawSavedStateRetained() =>
   niCompleteState(niObserved(niNull(),npObserved(
     ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngHalted(ngUInt(9)),
       (99 |-> ngBool(true)),ngFrame(ngArmScope(3,7,11,(99 |-> ngUInt(8))),ngNoFrames()),
       ngEvent(ngLiteralEvaluated(4,14,ngUInt(9)),ngNoEvents())),npUInt(9))))
   ==K ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngHalted(ngUInt(9)),
       (99 |-> ngBool(true)),ngFrame(ngArmScope(3,7,11,(99 |-> ngUInt(8))),ngNoFrames()),
       ngEvent(ngLiteralEvaluated(4,14,ngUInt(9)),ngNoEvents()))
 syntax Bool ::= niLawEventOrderRetained() [function]
 rule niLawEventOrderRetained() =>
   niOrderedEvents(niObserved(niNull(),npObserved(
     ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngHalted(ngUInt(9)),.Map,ngNoFrames(),
       ngEvent(ngLiteralEvaluated(4,14,ngUInt(9)),ngEvent(ngFinished(ngUInt(9)),ngNoEvents()))),npUInt(9))))
   ==K ngEvent(ngLiteralEvaluated(4,14,ngUInt(9)),ngEvent(ngFinished(ngUInt(9)),ngNoEvents()))
endmodule
