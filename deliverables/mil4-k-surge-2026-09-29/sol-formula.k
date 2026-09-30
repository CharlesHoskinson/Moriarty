requires "sol-static.k"

// Concrete Phi0 evaluation over stage-bound snapshots. No configuration.
module M4F-SYNTAX
  imports M4S-SYNTAX
  syntax M4FValues ::= m4fNoValues()
                     | m4fCell(String, M4SType, Int, String, M4FValues)
  syntax M4FFrame ::= m4fFrame(String, String, M4FValues, M4FValues, M4FValues)
  syntax M4FValue ::= m4fValue(M4SType, Int) | m4fValueReject(String)
  syntax M4FTruth ::= m4fTrue() | m4fFalse() | m4fReject(String)
  syntax M4FCount ::= m4fCount(Int) | m4fCountReject(String)
endmodule

module M4F-FORMULA
  imports M4S-STATIC
  imports M4F-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  imports K-EQUAL

  // The stage supplies a pre-head, a post-head and finite typed values. A
  // mismatched head, duplicate key or absent value is a rejection. The stage
  // must separately prove the snapshot's authentication at those heads.
  syntax Bool ::= m4fFits(M4SType, Int) [function, total]
                | m4fHas(M4FValues, String) [function, total]
                | m4fListValid(M4SEnv, M4FValues, M4STypes, String) [function, total]
                | m4fFrameValid(M4SEnv, M4FFrame) [function, total]
  rule m4fFits(m4sDelta(_),V) => V >=Int (0 -Int (2 ^Int 127))
                                   andBool V <Int (2 ^Int 127)
  rule m4fFits(m4sProp(),_) => false
  rule m4fFits(_,V) => m4sUnsigned(V,128) [owise]
  rule m4fHas(m4fNoValues(),_) => false
  rule m4fHas(m4fCell(ID,_,_,_,_),ID) => true
  rule m4fHas(m4fCell(ID,_,_,_,REST),WANT) => m4fHas(REST,WANT)
    requires ID =/=String WANT
  rule m4fListValid(_,m4fNoValues(),_,_) => true
  rule m4fListValid(E,m4fCell(ID,T,V,H,REST),DECL,EXPECTED)
    => H ==String EXPECTED andBool m4sLookup(DECL,ID) ==K m4sTyped(T)
       andBool m4sTypeValid(m4fAssets(E),T) andBool m4fFits(T,V)
       andBool notBool m4fHas(REST,ID)
       andBool m4fListValid(E,REST,DECL,EXPECTED)
  syntax M4SAssets ::= m4fAssets(M4SEnv) [function, total]
  rule m4fAssets(m4sEnv(A,_,_)) => A
  rule m4fFrameValid(m4sEnv(A,C,H),m4fFrame(PH,QH,PRE,POST,HOLES))
    => m4sAssetTableValid(A) andBool PH =/=String "" andBool QH =/=String ""
       andBool m4fListValid(m4sEnv(A,C,H),PRE,C,PH)
       andBool m4fListValid(m4sEnv(A,C,H),POST,C,QH)
       andBool m4fListValid(m4sEnv(A,C,H),HOLES,H,PH)

  syntax M4FValue ::= m4fFind(M4FValues, String, String) [function, total]
                     | m4fTerm(M4SEnv, M4STerm, M4SPhase, M4FFrame, Int) [function, total]
                     | m4fBinary(M4FValue, M4FValue, String) [function, total]
                     | m4fScale(M4FValue, Int) [function, total]
                     | m4fChecked(M4SType, Int, String) [function, total]
  rule m4fFind(m4fNoValues(),_,_) => m4fValueReject("MISSING_AUTHENTICATED_READ")
  rule m4fFind(m4fCell(ID,T,V,H,_),ID,H) => m4fValue(T,V)
  rule m4fFind(m4fCell(ID,_,_,H,_),ID,EXPECTED)
    => m4fValueReject("STALE_AUTHENTICATED_HEAD")
    requires H =/=String EXPECTED
  rule m4fFind(m4fCell(ID,_,_,_,REST),WANT,EXPECTED)
    => m4fFind(REST,WANT,EXPECTED) requires ID =/=String WANT
  rule m4fTerm(_,_,_,_,N) => m4fValueReject("FORMULA_DEPTH") requires N <=Int 0
  rule m4fTerm(_,m4sNat(V),_,_,N) => m4fChecked(m4sUInt(),V,"FORMULA_LITERAL")
    requires N >Int 0
  rule m4fTerm(_,m4sQtyLit(A,V),_,_,N) => m4fChecked(m4sQty(A),V,"FORMULA_LITERAL")
    requires N >Int 0
  rule m4fTerm(_,m4sPriceLit(B,Q,S,V),_,_,N)
    => m4fChecked(m4sPrice(B,Q,S),V,"FORMULA_LITERAL") requires N >Int 0
  rule m4fTerm(_,m4sPre(ID),_,m4fFrame(PH,_,PRE,_,_),N)
    => m4fFind(PRE,ID,PH) requires N >Int 0
  rule m4fTerm(_,m4sPost(_),m4sGuard(),_,N)
    => m4fValueReject("TYPE_POST_GUARD") requires N >Int 0
  rule m4fTerm(_,m4sPost(ID),m4sOutcome(),m4fFrame(_,QH,_,POST,_),N)
    => m4fFind(POST,ID,QH) requires N >Int 0
  rule m4fTerm(_,m4sHole(ID),_,m4fFrame(PH,_,_,_,HOLES),N)
    => m4fFind(HOLES,ID,PH) requires N >Int 0
  rule m4fTerm(E,m4sAdd(L,R),P,F,N)
    => m4fBinary(m4fTerm(E,L,P,F,N -Int 1),m4fTerm(E,R,P,F,N -Int 1),"ADD")
    requires N >Int 0
  rule m4fTerm(E,m4sSub(L,R),P,F,N)
    => m4fBinary(m4fTerm(E,L,P,F,N -Int 1),m4fTerm(E,R,P,F,N -Int 1),"SUB")
    requires N >Int 0
  rule m4fTerm(E,m4sMin(L,R),P,F,N)
    => m4fBinary(m4fTerm(E,L,P,F,N -Int 1),m4fTerm(E,R,P,F,N -Int 1),"MIN")
    requires N >Int 0
  rule m4fTerm(E,m4sMax(L,R),P,F,N)
    => m4fBinary(m4fTerm(E,L,P,F,N -Int 1),m4fTerm(E,R,P,F,N -Int 1),"MAX")
    requires N >Int 0
  rule m4fTerm(E,m4sTimesLiteral(T,K),P,F,N)
    => m4fScale(m4fTerm(E,T,P,F,N -Int 1),K) requires N >Int 0
  rule m4fTerm(_,m4sOracle(_),_,_,N) => m4fValueReject("OPEN_ORACLE_PREMISE")
    requires N >Int 0
  rule m4fTerm(_,m4sOmega(_),_,_,N) => m4fValueReject("OPEN_OMEGA_CERTIFICATION")
    requires N >Int 0
  rule m4fTerm(_,m4sVariableProduct(_,_),_,_,N)
    => m4fValueReject("UNSUPPORTED_PHI1") requires N >Int 0
  rule m4fChecked(T,V,_) => m4fValue(T,V) requires m4fFits(T,V)
  rule m4fChecked(_,_,C) => m4fValueReject(C) [owise]
  rule m4fBinary(m4fValueReject(C),_,_) => m4fValueReject(C)
  rule m4fBinary(m4fValue(_,_),m4fValueReject(C),_) => m4fValueReject(C)
  rule m4fBinary(m4fValue(m4sQty(ID),A),m4fValue(m4sDelta(ID),B),"ADD")
    => m4fChecked(m4sQty(ID),A +Int B,"FORMULA_OVERFLOW_OR_RANGE")
  rule m4fBinary(m4fValue(m4sDelta(ID),A),m4fValue(m4sQty(ID),B),"ADD")
    => m4fChecked(m4sQty(ID),A +Int B,"FORMULA_OVERFLOW_OR_RANGE")
  rule m4fBinary(m4fValue(m4sQty(ID),A),m4fValue(m4sDelta(ID),B),"SUB")
    => m4fChecked(m4sQty(ID),A -Int B,"FORMULA_OVERFLOW_OR_RANGE")
  rule m4fBinary(m4fValue(m4sInstant(C),A),m4fValue(m4sDuration(C),B),"ADD")
    => m4fChecked(m4sInstant(C),A +Int B,"FORMULA_TIME_RANGE")
  rule m4fBinary(m4fValue(m4sInstant(C),A),m4fValue(m4sDuration(C),B),"SUB")
    => m4fChecked(m4sInstant(C),A -Int B,"FORMULA_TIME_RANGE")
  rule m4fBinary(m4fValue(m4sInstant(C),A),m4fValue(m4sInstant(C),B),"SUB")
    => m4fChecked(m4sDuration(C),A -Int B,"FORMULA_TIME_RANGE")
  rule m4fBinary(m4fValue(m4sDuration(C),A),m4fValue(m4sDuration(C),B),"ADD")
    => m4fChecked(m4sDuration(C),A +Int B,"FORMULA_TIME_RANGE")
  rule m4fBinary(m4fValue(m4sDuration(C),A),m4fValue(m4sDuration(C),B),"SUB")
    => m4fChecked(m4sDuration(C),A -Int B,"FORMULA_TIME_RANGE")
  rule m4fBinary(m4fValue(T,A),m4fValue(T,B),"ADD")
    => m4fChecked(T,A +Int B,"FORMULA_OVERFLOW")
    requires m4sArithmetic(T,"ADD")
  rule m4fBinary(m4fValue(T,A),m4fValue(T,B),"SUB")
    => m4fChecked(T,A -Int B,"FORMULA_UNDERFLOW_OR_RANGE")
    requires m4sArithmetic(T,"SUB")
  rule m4fBinary(m4fValue(T,A),m4fValue(T,B),"MIN")
    => m4fValue(T,minInt(A,B)) requires m4sArithmetic(T,"MIN")
  rule m4fBinary(m4fValue(T,A),m4fValue(T,B),"MAX")
    => m4fValue(T,maxInt(A,B)) requires m4sArithmetic(T,"MAX")
  rule m4fBinary(_,_,_) => m4fValueReject("TYPE_MISMATCH") [owise]
  rule m4fScale(m4fValueReject(C),_) => m4fValueReject(C)
  rule m4fScale(m4fValue(T,V),K) => m4fChecked(T,V *Int K,"FORMULA_OVERFLOW")
    requires m4sArithmetic(T,"MUL_LITERAL") andBool m4sUnsigned(K,128)
  rule m4fScale(_,_) => m4fValueReject("TYPE_COEFFICIENT") [owise]

  syntax M4FTruth ::= m4fCompare(M4FValue, M4FValue, String) [function, total]
                     | m4fNot(M4FTruth) [function, total]
                     | m4fAndTruth(M4FTruth, M4FTruth) [function, total]
                     | m4fOrTruth(M4FTruth, M4FTruth) [function, total]
                     | m4fEval(M4SEnv, M4SFormula, M4SPhase, M4FFrame, Int) [function, total]
                     | m4fThresholdResult(Int, Int, M4FCount) [function, total]
                     | m4fEvaluate(M4SEnv, M4SFormula, M4SPhase, M4FFrame, Int) [function, total]
                     | m4fAfterStatic(M4SCheck, M4SEnv, M4SFormula, M4SPhase, M4FFrame, Int) [function, total]
  syntax M4FCount ::= m4fEvalList(M4SEnv, M4SFormulas, M4SPhase, M4FFrame, Int) [function, total]
                     | m4fCountStep(M4FTruth, M4FCount) [function, total]
  rule m4fCompare(m4fValueReject(C),_,_) => m4fReject(C)
  rule m4fCompare(m4fValue(_,_),m4fValueReject(C),_) => m4fReject(C)
  rule m4fCompare(m4fValue(T,A),m4fValue(T,B),"EQ") => m4fTrue()
    requires m4sComparable(T) andBool A ==Int B
  rule m4fCompare(m4fValue(T,A),m4fValue(T,B),"EQ") => m4fFalse()
    requires m4sComparable(T) andBool A =/=Int B
  rule m4fCompare(m4fValue(T,A),m4fValue(T,B),"LT") => m4fTrue()
    requires m4sComparable(T) andBool A <Int B
  rule m4fCompare(m4fValue(T,A),m4fValue(T,B),"LT") => m4fFalse()
    requires m4sComparable(T) andBool A >=Int B
  rule m4fCompare(m4fValue(T,A),m4fValue(T,B),"LE") => m4fTrue()
    requires m4sComparable(T) andBool A <=Int B
  rule m4fCompare(m4fValue(T,A),m4fValue(T,B),"LE") => m4fFalse()
    requires m4sComparable(T) andBool A >Int B
  rule m4fCompare(_,_,_) => m4fReject("TYPE_MISMATCH") [owise]
  rule m4fNot(m4fTrue()) => m4fFalse()
  rule m4fNot(m4fFalse()) => m4fTrue()
  rule m4fNot(m4fReject(C)) => m4fReject(C)
  // Eager evaluation propagates a rejection from either arm, including an
  // oracle term in an arm whose Boolean value would otherwise be irrelevant.
  rule m4fAndTruth(m4fReject(C),_) => m4fReject(C)
  rule m4fAndTruth(m4fTrue(),m4fReject(C)) => m4fReject(C)
  rule m4fAndTruth(m4fFalse(),m4fReject(C)) => m4fReject(C)
  rule m4fAndTruth(m4fTrue(),m4fTrue()) => m4fTrue()
  rule m4fAndTruth(m4fTrue(),m4fFalse()) => m4fFalse()
  rule m4fAndTruth(m4fFalse(),m4fTrue()) => m4fFalse()
  rule m4fAndTruth(m4fFalse(),m4fFalse()) => m4fFalse()
  rule m4fOrTruth(m4fReject(C),_) => m4fReject(C)
  rule m4fOrTruth(m4fTrue(),m4fReject(C)) => m4fReject(C)
  rule m4fOrTruth(m4fFalse(),m4fReject(C)) => m4fReject(C)
  rule m4fOrTruth(m4fTrue(),m4fTrue()) => m4fTrue()
  rule m4fOrTruth(m4fTrue(),m4fFalse()) => m4fTrue()
  rule m4fOrTruth(m4fFalse(),m4fTrue()) => m4fTrue()
  rule m4fOrTruth(m4fFalse(),m4fFalse()) => m4fFalse()
  rule m4fEval(_,_,_,_,N) => m4fReject("FORMULA_DEPTH") requires N <=Int 0
  rule m4fEval(E,m4sEqual(L,R),P,F,N)
    => m4fCompare(m4fTerm(E,L,P,F,N -Int 1),m4fTerm(E,R,P,F,N -Int 1),"EQ")
    requires N >Int 0
  rule m4fEval(E,m4sLess(L,R),P,F,N)
    => m4fCompare(m4fTerm(E,L,P,F,N -Int 1),m4fTerm(E,R,P,F,N -Int 1),"LT")
    requires N >Int 0
  rule m4fEval(E,m4sAtMost(L,R),P,F,N)
    => m4fCompare(m4fTerm(E,L,P,F,N -Int 1),m4fTerm(E,R,P,F,N -Int 1),"LE")
    requires N >Int 0
  rule m4fEval(E,m4sNotAtom(A),P,F,N)
    => m4fNot(m4fEval(E,A,P,F,N -Int 1))
    requires N >Int 0 andBool m4sAtomic(A)
  rule m4fEval(_,m4sNotAtom(A),_,_,N) => m4fReject("TYPE_NONATOMIC_NEGATION")
    requires N >Int 0 andBool notBool m4sAtomic(A)
  rule m4fEval(E,m4sAnd(L,R),P,F,N)
    => m4fAndTruth(m4fEval(E,L,P,F,N -Int 1),m4fEval(E,R,P,F,N -Int 1))
    requires N >Int 0
  rule m4fEval(E,m4sOr(L,R),P,F,N)
    => m4fOrTruth(m4fEval(E,L,P,F,N -Int 1),m4fEval(E,R,P,F,N -Int 1))
    requires N >Int 0
  rule m4fEval(E,m4sThreshold(K,FS),P,F,N)
    => m4fThresholdResult(K,m4sFormulaCount(FS),m4fEvalList(E,FS,P,F,N -Int 1))
    requires N >Int 0
  rule m4fEval(_,m4sUnsupportedFormula(_),_,_,N)
    => m4fReject("OPEN_FORMULA_PREMISE") requires N >Int 0
  rule m4fEvalList(_,m4sNoFormulas(),_,_,N) => m4fCount(0)
    requires N >Int 0
  rule m4fEvalList(_,m4sNoFormulas(),_,_,N)
    => m4fCountReject("FORMULA_DEPTH") requires N <=Int 0
  rule m4fEvalList(_,m4sFormulaCons(_,_),_,_,N)
    => m4fCountReject("FORMULA_DEPTH") requires N <=Int 0
  rule m4fEvalList(E,m4sFormulaCons(A,REST),P,F,N)
    => m4fCountStep(m4fEval(E,A,P,F,N -Int 1),m4fEvalList(E,REST,P,F,N -Int 1))
    requires N >Int 0
  rule m4fCountStep(m4fReject(C),_) => m4fCountReject(C)
  rule m4fCountStep(m4fTrue(),m4fCountReject(C)) => m4fCountReject(C)
  rule m4fCountStep(m4fFalse(),m4fCountReject(C)) => m4fCountReject(C)
  rule m4fCountStep(m4fTrue(),m4fCount(N)) => m4fCount(N +Int 1)
  rule m4fCountStep(m4fFalse(),m4fCount(N)) => m4fCount(N)
  rule m4fThresholdResult(_,_,m4fCountReject(C)) => m4fReject(C)
  rule m4fThresholdResult(K,SIZE,m4fCount(C)) => m4fReject("TYPE_THRESHOLD_BOUND")
    requires K <Int 1 orBool K >Int SIZE orBool SIZE >Int 128
  rule m4fThresholdResult(K,SIZE,m4fCount(C)) => m4fTrue()
    requires K >=Int 1 andBool K <=Int SIZE andBool SIZE <=Int 128 andBool C >=Int K
  rule m4fThresholdResult(K,SIZE,m4fCount(C)) => m4fFalse()
    requires K >=Int 1 andBool K <=Int SIZE andBool SIZE <=Int 128 andBool C <Int K
  rule m4fEvaluate(E,F,P,FRAME,N)
    => m4fAfterStatic(m4sStatic(E,F,P,N),E,F,P,FRAME,N)
  rule m4fAfterStatic(m4sRejected(C),_,_,_,_,_) => m4fReject(C)
  rule m4fAfterStatic(m4sTyped(m4sProp()),E,F,P,FRAME,N)
    => m4fEval(E,F,P,FRAME,N) requires m4fFrameValid(E,FRAME)
  rule m4fAfterStatic(m4sTyped(m4sProp()),E,_,_,FRAME,_)
    => m4fReject("INVALID_AUTHENTICATED_FRAME")
    requires notBool m4fFrameValid(E,FRAME)
  rule m4fAfterStatic(_,_,_,_,_,_) => m4fReject("TYPE_FORMULA") [owise]
endmodule
