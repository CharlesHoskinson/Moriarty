requires "programview-identity-source10.k"
requires "normal-trace-observations.k"

// Complete source plus the existing chronological, non-stuttering full-state tape.
// Nonnegative finite Cuts and the existing positive NG program domain are premises.
// No transition rule, frame/environment behavior, value semantics or clock changes.
module SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11
 imports SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10
 imports SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS
 syntax NIWindowObservation ::= niWindowObserved(NIJSON,NGTape)
                            | niRunWindow(NIJSON,NGProgram,Int) [function]
 rule niRunWindow(Source,P,Cuts)
   => niWindowObserved(Source,ngObserve(ngInitial(P,.Map),Cuts))
endmodule
