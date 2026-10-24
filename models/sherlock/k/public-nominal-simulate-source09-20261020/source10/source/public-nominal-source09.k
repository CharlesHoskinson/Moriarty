requires "nominal-product.k"
// Positive source semantic extension and lossless typed return observer only.
module SHERLOCK-PUBLIC-NOMINAL-SOURCE09
 imports SHERLOCK-NOMINAL-PRODUCT
 syntax NGExpr ::= ngNot(Int, Int)
 syntax NGFrame ::= ngNotArgument(Int, Int, Map)
 syntax NGEvent ::= ngNotStarted(Int, Int, Int, Map)
                  | ngNotReturned(Int, Int, NGValue, NGValue, Map)
 rule ngOccurrence(ngFoundExpr(ngNot(O,_))) => O
 rule ngEvaluate(ngFoundExpr(ngNot(O,X)),E,P,Rho,Fs,Es)
  => ngState(P,ngEval(X),Rho,ngFrame(ngNotArgument(E,O,Rho),Fs),
       ngAppendEvent(Es,ngNotStarted(E,O,X,Rho)))
 rule ngStep(ngState(P,ngReturn(ngBool(B)),_,ngFrame(ngNotArgument(E,O,Saved),Fs),Es))
  => ngState(P,ngReturn(ngBool(notBool B)),Saved,Fs,
       ngAppendEvent(Es,ngNotReturned(E,O,ngBool(B),ngBool(notBool B),Saved)))
 rule ngStep(ngState(P,ngReturn(V),Rho,ngFrame(ngNotArgument(E,O,Saved),Fs),Es))
  => ngOutside(ngState(P,ngReturn(V),Rho,ngFrame(ngNotArgument(E,O,Saved),Fs),Es),
       "not_operand_type_outside_model") requires notBool ngIsBool(V)

 syntax NPType ::= npUIntType() | npBoolType() | npNominalType(String,NPTypes)
 syntax NPTypes ::= npNoTypes() | npType(NPType,NPTypes)
 syntax NPTypeRows ::= npNoTypeRows() | npTypeRow(Int,NPType,NPTypeRows)
 syntax NPFieldNames ::= npNoFieldNames() | npFieldName(String,NPFieldNames)
 syntax NPCtorRows ::= npNoCtorRows() | npCtorRow(Int,Int,String,NPFieldNames,NPCtorRows)
 syntax NPCtorSchema ::= npCtorSchema(String,NPFieldNames)
 syntax NPValue ::= npUInt(Int) | npBool(Bool) | npNominal(NPType,String,NPFields)
 syntax NPFields ::= npNoFields() | npField(String,NPValue,NPFields)
 syntax NPObservation ::= npObserved(NGState,NPValue)
 syntax NPType ::= npFindType(Int,NPTypeRows) [function]
 rule npFindType(T,npTypeRow(T,X,_)) => X
 rule npFindType(T,npTypeRow(U,_,Rs)) => npFindType(T,Rs) requires T =/=Int U
 syntax NPCtorSchema ::= npFindCtor(Int,Int,NPCtorRows) [function]
 rule npFindCtor(T,C,npCtorRow(T,C,N,Fs,_)) => npCtorSchema(N,Fs)
 rule npFindCtor(T,C,npCtorRow(U,D,_,_,Rs)) => npFindCtor(T,C,Rs)
  requires T =/=Int U orBool C =/=Int D
 syntax NPValue ::= npProjectPositive(NGValue,NPTypeRows,NPCtorRows) [function]
                  | npProjectNominal(NPType,NPCtorSchema,NGValues,NPTypeRows,NPCtorRows) [function]
 rule npProjectPositive(ngUInt(I),_,_) => npUInt(I)
 rule npProjectPositive(ngBool(B),_,_) => npBool(B)
 rule npProjectPositive(ngNominal(T,C,Vs),Ts,Cs)
  => npProjectNominal(npFindType(T,Ts),npFindCtor(T,C,Cs),Vs,Ts,Cs)
 rule npProjectNominal(T,npCtorSchema(C,Fs),Vs,Ts,Cs)
  => npNominal(T,C,npProjectFields(Fs,Vs,Ts,Cs))
 syntax NPFields ::= npProjectFields(NPFieldNames,NGValues,NPTypeRows,NPCtorRows) [function]
 rule npProjectFields(npNoFieldNames(),ngNoValues(),_,_) => npNoFields()
 rule npProjectFields(npFieldName(N,Ns),ngValue(V,Vs),Ts,Cs)
  => npField(N,npProjectPositive(V,Ts,Cs),npProjectFields(Ns,Vs,Ts,Cs))
 // Complete positive metadata/arity and a halted result are premises; no refusal policy.
 syntax NPObservation ::= npObserveHalt(NGState,NPTypeRows,NPCtorRows) [function]
                       | npRunReference(NGProgram,NPTypeRows,NPCtorRows,Int) [function]
 rule npObserveHalt(ngState(P,ngHalted(V),Rho,Fs,Es),Ts,Cs)
  => npObserved(ngState(P,ngHalted(V),Rho,Fs,Es),npProjectPositive(V,Ts,Cs))
 rule npRunReference(P,Ts,Cs,Cuts)
  => npObserveHalt(ngAdvance(ngInitial(P,.Map),Cuts),Ts,Cs)
 syntax NPValue ::= npProjectedValue(NPObservation) [function]
 rule npProjectedValue(npObserved(_,V)) => V
endmodule
