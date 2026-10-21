requires "public-nominal-source09.k"
// Source-wire DATA retention plus the existing complete reference observation.
// No JSON parser, admission predicate, Core clock, or replacement evaluator.
module SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10
 imports SHERLOCK-PUBLIC-NOMINAL-SOURCE09
 syntax NIJSON ::= niNull() | niBool(Bool) | niInt(Int) | niString(String)
                 | niArray(NIJItems) | niObject(NIJFields)
 syntax NIJItems ::= niNoItems() | niItem(NIJSON,NIJItems)
 syntax NIJFields ::= niNoFields() | niField(String,NIJSON,NIJFields)
 syntax NIObservation ::= niObserved(NIJSON,NPObservation)
                       | niRun(NIJSON,NGProgram,NPTypeRows,NPCtorRows,Int) [function]
 rule niRun(Source,P,Ts,Cs,Cuts) => niObserved(Source,npRunReference(P,Ts,Cs,Cuts))
 syntax NIJSON ::= niSource(NIObservation) [function]
 rule niSource(niObserved(Source,_)) => Source
 syntax NPObservation ::= niReferenceObservation(NIObservation) [function]
 rule niReferenceObservation(niObserved(_,Observation)) => Observation
 syntax NGState ::= niCompleteState(NIObservation) [function]
 rule niCompleteState(niObserved(_,npObserved(State,_))) => State
 syntax NGEvents ::= niOrderedEvents(NIObservation) [function]
 rule niOrderedEvents(niObserved(_,npObserved(ngState(_,_,_,_,Events),_))) => Events
endmodule
