// MIL/4 static proposal. Specified only: no source/Core/native correspondence claim.
// All identifiers in this file use the M4S prefix to avoid imports from legacy K.
module M4S-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX

  syntax M4SAssets ::= m4sNoAssets() | m4sAsset(String, String, Int, M4SAssets)
  syntax M4STypes ::= m4sNoBindings() | m4sBinding(String, M4SType, M4STypes)
  syntax M4SEnv ::= m4sEnv(M4SAssets, M4STypes, M4STypes)
  syntax M4SType ::= m4sQty(String)
                   | m4sDelta(String)
                   | m4sPrice(String, String, Int)
                   | m4sShares(String, String)
                   | m4sInstant(String)
                   | m4sDuration(String)
                   | m4sUInt()
                   | m4sProp()
  syntax M4STerm ::= m4sNat(Int)
                   | m4sQtyLit(String, Int)
                   | m4sPriceLit(String, String, Int, Int)
                   | m4sPre(String)
                   | m4sPost(String)
                   | m4sHole(String)
                   | m4sAdd(M4STerm, M4STerm)
                   | m4sSub(M4STerm, M4STerm)
                   | m4sMin(M4STerm, M4STerm)
                   | m4sMax(M4STerm, M4STerm)
                   | m4sTimesLiteral(M4STerm, Int)
                   | m4sOracle(String)
                   | m4sOmega(String)
                   | m4sVariableProduct(M4STerm, M4STerm)
  syntax M4SFormula ::= m4sEqual(M4STerm, M4STerm)
                      | m4sLess(M4STerm, M4STerm)
                      | m4sAtMost(M4STerm, M4STerm)
                      | m4sNotAtom(M4SFormula)
                      | m4sAnd(M4SFormula, M4SFormula)
                      | m4sOr(M4SFormula, M4SFormula)
                      | m4sThreshold(Int, M4SFormulas)
                      | m4sUnsupportedFormula(String)
  syntax M4SFormulas ::= m4sNoFormulas() | m4sFormulaCons(M4SFormula, M4SFormulas)
  syntax M4SPhase ::= m4sGuard() | m4sOutcome()
  syntax M4SCheck ::= m4sTyped(M4SType) | m4sRejected(String)
  syntax M4SNumber ::= m4sNumber(Int) | m4sNumericReject(String)
  syntax M4SRounding ::= m4sFloor() | m4sCeil() | m4sExact()
  syntax M4SResidue ::= m4sWholeUnit(String, String, Int)
                      | m4sProductRemainder(Int)
endmodule

module M4S-STATIC
  imports M4S-SYNTAX
  imports INT
  imports BOOL
  imports STRING

  // Asset IDs are nominal. Domain and decimals are checked metadata, never an
  // equality substitute for the ID. Environments are finite linked lists.
  syntax Bool ::= m4sUnsigned(Int, Int) [function, total]
                | m4sKnownAsset(M4SAssets, String) [function, total]
                | m4sAssetTableValid(M4SAssets) [function, total]
                | m4sTypeValid(M4SAssets, M4SType) [function, total]
                | m4sBindingsValid(M4SAssets, M4STypes) [function, total]
                | m4sBoundName(M4STypes, String) [function, total]
                | m4sEnvValid(M4SEnv) [function, total]
                | m4sComparable(M4SType) [function, total]
                | m4sArithmetic(M4SType, String) [function, total]
  rule m4sUnsigned(N,W) => N >=Int 0 andBool N <Int (2 ^Int W)
    requires W >Int 0
  rule m4sUnsigned(_,_) => false [owise]
  rule m4sKnownAsset(m4sNoAssets(),_) => false
  rule m4sKnownAsset(m4sAsset(ID,_,_,_),ID) => true
  rule m4sKnownAsset(m4sAsset(ID,_,_,REST),WANT) => m4sKnownAsset(REST,WANT)
    requires ID =/=String WANT
  rule m4sAssetTableValid(m4sNoAssets()) => true
  rule m4sAssetTableValid(m4sAsset(ID,DOMAIN,D,REST))
    => ID =/=String "" andBool DOMAIN =/=String "" andBool D >=Int 0
       andBool D <=Int 18 andBool notBool m4sKnownAsset(REST,ID)
       andBool m4sAssetTableValid(REST)
  rule m4sTypeValid(A,m4sQty(ID)) => m4sKnownAsset(A,ID)
  rule m4sTypeValid(A,m4sDelta(ID)) => m4sKnownAsset(A,ID)
  rule m4sTypeValid(A,m4sPrice(BASE,QUOTE,S))
    => m4sKnownAsset(A,BASE) andBool m4sKnownAsset(A,QUOTE)
       andBool BASE =/=String QUOTE andBool S >=Int 0 andBool S <=Int 18
  // A share needs a registered share asset and issue authority; that registry
  // is outside this static slice, so an arbitrary pair is not admitted.
  rule m4sTypeValid(_,m4sShares(_,_)) => false
  rule m4sTypeValid(_,m4sInstant(C)) => C =/=String ""
  rule m4sTypeValid(_,m4sDuration(C)) => C =/=String ""
  rule m4sTypeValid(_,m4sUInt()) => true
  rule m4sTypeValid(_,m4sProp()) => false
  rule m4sBoundName(m4sNoBindings(),_) => false
  rule m4sBoundName(m4sBinding(N,_,_),N) => true
  rule m4sBoundName(m4sBinding(N,_,REST),WANT) => m4sBoundName(REST,WANT)
    requires N =/=String WANT
  rule m4sBindingsValid(_,m4sNoBindings()) => true
  rule m4sBindingsValid(A,m4sBinding(N,T,REST))
    => N =/=String "" andBool notBool m4sBoundName(REST,N)
       andBool m4sTypeValid(A,T) andBool m4sBindingsValid(A,REST)
  rule m4sEnvValid(m4sEnv(A,C,H))
    => m4sAssetTableValid(A) andBool m4sBindingsValid(A,C)
       andBool m4sBindingsValid(A,H)
  rule m4sComparable(m4sQty(_)) => true
  rule m4sComparable(m4sDelta(_)) => true
  rule m4sComparable(m4sPrice(_,_,_)) => true
  rule m4sComparable(m4sShares(_,_)) => false
  rule m4sComparable(m4sInstant(_)) => true
  rule m4sComparable(m4sDuration(_)) => true
  rule m4sComparable(m4sUInt()) => true
  rule m4sComparable(m4sProp()) => false
  rule m4sArithmetic(m4sQty(_),OP) => m4sNumericOp(OP)
  rule m4sArithmetic(m4sDelta(_),OP) => m4sNumericOp(OP)
  rule m4sArithmetic(m4sUInt(),OP) => m4sNumericOp(OP)
  rule m4sArithmetic(T,OP) => m4sComparable(T) andBool
    (OP ==String "MIN" orBool OP ==String "MAX") [owise]
  syntax Bool ::= m4sNumericOp(String) [function, total]
  rule m4sNumericOp("ADD") => true
  rule m4sNumericOp("SUB") => true
  rule m4sNumericOp("MIN") => true
  rule m4sNumericOp("MAX") => true
  rule m4sNumericOp("MUL_LITERAL") => true
  rule m4sNumericOp(_) => false [owise]

  syntax M4SCheck ::= m4sLookup(M4STypes, String) [function, total]
                    | m4sInfer(M4SEnv, M4STerm, M4SPhase, Int) [function, total]
                    | m4sCombine(M4SCheck, M4SCheck, String) [function, total]
                    | m4sScaleType(M4SCheck, Int) [function, total]
                    | m4sCompare(M4SCheck, M4SCheck) [function, total]
                    | m4sForm(M4SEnv, M4SFormula, M4SPhase, Int) [function, total]
                    | m4sFormList(M4SEnv, M4SFormulas, M4SPhase, Int) [function, total]
                    | m4sBoth(M4SCheck, M4SCheck) [function, total]
                    | m4sThresholdCheck(Int, M4SFormulas, M4SCheck) [function, total]
                    | m4sStatic(M4SEnv, M4SFormula, M4SPhase, Int) [function, total]
  syntax Int ::= m4sFormulaCount(M4SFormulas) [function, total]
  rule m4sLookup(m4sNoBindings(),_) => m4sRejected("UNKNOWN_NAME")
  rule m4sLookup(m4sBinding(N,T,_),N) => m4sTyped(T)
  rule m4sLookup(m4sBinding(N,_,REST),WANT) => m4sLookup(REST,WANT)
    requires N =/=String WANT

  // Fuel is a public finite formula/term depth bound. Exhaustion rejects.
  rule m4sInfer(_,_,_,N) => m4sRejected("STATIC_DEPTH") requires N <=Int 0
  rule m4sInfer(_,m4sNat(V),_,N) => m4sTyped(m4sUInt())
    requires N >Int 0 andBool m4sUnsigned(V,128)
  rule m4sInfer(_,m4sNat(V),_,N) => m4sRejected("STATIC_LITERAL_RANGE")
    requires N >Int 0 andBool notBool m4sUnsigned(V,128)
  rule m4sInfer(m4sEnv(A,_,_),m4sQtyLit(ID,V),_,N) => m4sTyped(m4sQty(ID))
    requires N >Int 0 andBool m4sKnownAsset(A,ID) andBool m4sUnsigned(V,128)
  rule m4sInfer(m4sEnv(A,_,_),m4sQtyLit(ID,V),_,N) => m4sRejected("STATIC_ASSET_OR_RANGE")
    requires N >Int 0 andBool notBool (m4sKnownAsset(A,ID) andBool m4sUnsigned(V,128))
  rule m4sInfer(m4sEnv(A,_,_),m4sPriceLit(B,Q,S,V),_,N)
    => m4sTyped(m4sPrice(B,Q,S))
    requires N >Int 0 andBool m4sTypeValid(A,m4sPrice(B,Q,S)) andBool m4sUnsigned(V,128)
  rule m4sInfer(m4sEnv(A,_,_),m4sPriceLit(B,Q,S,V),_,N)
    => m4sRejected("STATIC_PRICE_OR_RANGE")
    requires N >Int 0 andBool notBool (m4sTypeValid(A,m4sPrice(B,Q,S)) andBool m4sUnsigned(V,128))
  rule m4sInfer(m4sEnv(_,C,_),m4sPre(ID),_,N) => m4sLookup(C,ID)
    requires N >Int 0
  rule m4sInfer(_,m4sPost(_),m4sGuard(),N) => m4sRejected("TYPE_POST_GUARD")
    requires N >Int 0
  rule m4sInfer(m4sEnv(_,C,_),m4sPost(ID),m4sOutcome(),N) => m4sLookup(C,ID)
    requires N >Int 0
  rule m4sInfer(m4sEnv(_,_,H),m4sHole(ID),_,N) => m4sLookup(H,ID)
    requires N >Int 0
  rule m4sInfer(E,m4sAdd(L,R),P,N)
    => m4sCombine(m4sInfer(E,L,P,N -Int 1),m4sInfer(E,R,P,N -Int 1),"ADD")
    requires N >Int 0
  rule m4sInfer(E,m4sSub(L,R),P,N)
    => m4sCombine(m4sInfer(E,L,P,N -Int 1),m4sInfer(E,R,P,N -Int 1),"SUB")
    requires N >Int 0
  rule m4sInfer(E,m4sMin(L,R),P,N)
    => m4sCombine(m4sInfer(E,L,P,N -Int 1),m4sInfer(E,R,P,N -Int 1),"MIN")
    requires N >Int 0
  rule m4sInfer(E,m4sMax(L,R),P,N)
    => m4sCombine(m4sInfer(E,L,P,N -Int 1),m4sInfer(E,R,P,N -Int 1),"MAX")
    requires N >Int 0
  rule m4sInfer(E,m4sTimesLiteral(T,K),P,N)
    => m4sScaleType(m4sInfer(E,T,P,N -Int 1),K)
    requires N >Int 0
  rule m4sInfer(_,m4sOracle(_),_,N) => m4sRejected("OPEN_ORACLE_PREMISE")
    requires N >Int 0
  rule m4sInfer(_,m4sOmega(_),_,N) => m4sRejected("OPEN_OMEGA_CERTIFICATION")
    requires N >Int 0
  rule m4sInfer(_,m4sVariableProduct(_,_),_,N) => m4sRejected("UNSUPPORTED_PHI1")
    requires N >Int 0

  rule m4sCombine(m4sRejected(C),_,_) => m4sRejected(C)
  rule m4sCombine(m4sTyped(_),m4sRejected(C),_) => m4sRejected(C)
  rule m4sCombine(m4sTyped(m4sQty(A)),m4sTyped(m4sDelta(A)),"ADD")
    => m4sTyped(m4sQty(A))
  rule m4sCombine(m4sTyped(m4sDelta(A)),m4sTyped(m4sQty(A)),"ADD")
    => m4sTyped(m4sQty(A))
  rule m4sCombine(m4sTyped(m4sQty(A)),m4sTyped(m4sDelta(A)),"SUB")
    => m4sTyped(m4sQty(A))
  rule m4sCombine(m4sTyped(m4sInstant(C)),m4sTyped(m4sDuration(C)),"ADD")
    => m4sTyped(m4sInstant(C))
  rule m4sCombine(m4sTyped(m4sInstant(C)),m4sTyped(m4sDuration(C)),"SUB")
    => m4sTyped(m4sInstant(C))
  rule m4sCombine(m4sTyped(m4sInstant(C)),m4sTyped(m4sInstant(C)),"SUB")
    => m4sTyped(m4sDuration(C))
  rule m4sCombine(m4sTyped(m4sDuration(C)),m4sTyped(m4sDuration(C)),"ADD")
    => m4sTyped(m4sDuration(C))
  rule m4sCombine(m4sTyped(m4sDuration(C)),m4sTyped(m4sDuration(C)),"SUB")
    => m4sTyped(m4sDuration(C))
  rule m4sCombine(m4sTyped(T),m4sTyped(T),OP) => m4sTyped(T)
    requires m4sArithmetic(T,OP)
  rule m4sCombine(m4sTyped(_),m4sTyped(_),_) => m4sRejected("TYPE_MISMATCH") [owise]
  rule m4sScaleType(m4sRejected(C),_) => m4sRejected(C)
  rule m4sScaleType(m4sTyped(T),K) => m4sTyped(T)
    requires m4sArithmetic(T,"MUL_LITERAL") andBool m4sUnsigned(K,128)
  rule m4sScaleType(m4sTyped(_),_) => m4sRejected("TYPE_COEFFICIENT") [owise]
  rule m4sCompare(m4sRejected(C),_) => m4sRejected(C)
  rule m4sCompare(m4sTyped(_),m4sRejected(C)) => m4sRejected(C)
  rule m4sCompare(m4sTyped(T),m4sTyped(T)) => m4sTyped(m4sProp())
    requires m4sComparable(T)
  rule m4sCompare(m4sTyped(_),m4sTyped(_)) => m4sRejected("TYPE_MISMATCH") [owise]

  rule m4sForm(_,_,_,N) => m4sRejected("STATIC_DEPTH") requires N <=Int 0
  rule m4sForm(E,m4sEqual(L,R),P,N)
    => m4sCompare(m4sInfer(E,L,P,N -Int 1),m4sInfer(E,R,P,N -Int 1))
    requires N >Int 0
  rule m4sForm(E,m4sLess(L,R),P,N)
    => m4sCompare(m4sInfer(E,L,P,N -Int 1),m4sInfer(E,R,P,N -Int 1))
    requires N >Int 0
  rule m4sForm(E,m4sAtMost(L,R),P,N)
    => m4sCompare(m4sInfer(E,L,P,N -Int 1),m4sInfer(E,R,P,N -Int 1))
    requires N >Int 0
  rule m4sForm(E,m4sNotAtom(A),P,N) => m4sForm(E,A,P,N -Int 1)
    requires N >Int 0 andBool m4sAtomic(A)
  rule m4sForm(_,m4sNotAtom(A),_,N) => m4sRejected("TYPE_NONATOMIC_NEGATION")
    requires N >Int 0 andBool notBool m4sAtomic(A)
  rule m4sForm(E,m4sAnd(L,R),P,N)
    => m4sBoth(m4sForm(E,L,P,N -Int 1),m4sForm(E,R,P,N -Int 1))
    requires N >Int 0
  rule m4sForm(E,m4sOr(L,R),P,N)
    => m4sBoth(m4sForm(E,L,P,N -Int 1),m4sForm(E,R,P,N -Int 1))
    requires N >Int 0
  rule m4sForm(E,m4sThreshold(K,FS),P,N)
    => m4sThresholdCheck(K,FS,m4sFormList(E,FS,P,N -Int 1))
    requires N >Int 0
  rule m4sForm(_,m4sUnsupportedFormula(_),_,N)
    => m4sRejected("OPEN_FORMULA_PREMISE") requires N >Int 0
  syntax Bool ::= m4sAtomic(M4SFormula) [function, total]
  rule m4sAtomic(m4sEqual(_,_)) => true
  rule m4sAtomic(m4sLess(_,_)) => true
  rule m4sAtomic(m4sAtMost(_,_)) => true
  rule m4sAtomic(_) => false [owise]
  rule m4sBoth(m4sRejected(C),_) => m4sRejected(C)
  rule m4sBoth(m4sTyped(m4sProp()),m4sRejected(C)) => m4sRejected(C)
  rule m4sBoth(m4sTyped(m4sProp()),m4sTyped(m4sProp())) => m4sTyped(m4sProp())
  rule m4sBoth(_,_) => m4sRejected("TYPE_FORMULA") [owise]
  rule m4sFormulaCount(m4sNoFormulas()) => 0
  rule m4sFormulaCount(m4sFormulaCons(_,REST)) => 1 +Int m4sFormulaCount(REST)
  rule m4sFormList(_,m4sNoFormulas(),_,N) => m4sTyped(m4sProp())
    requires N >Int 0
  rule m4sFormList(_,m4sNoFormulas(),_,N) => m4sRejected("STATIC_DEPTH")
    requires N <=Int 0
  rule m4sFormList(_,m4sFormulaCons(_,_),_,N) => m4sRejected("STATIC_DEPTH")
    requires N <=Int 0
  rule m4sFormList(E,m4sFormulaCons(F,REST),P,N)
    => m4sBoth(m4sForm(E,F,P,N -Int 1),m4sFormList(E,REST,P,N -Int 1))
    requires N >Int 0
  rule m4sThresholdCheck(_,_,m4sRejected(C)) => m4sRejected(C)
  rule m4sThresholdCheck(K,FS,m4sTyped(m4sProp())) => m4sTyped(m4sProp())
    requires K >=Int 1 andBool K <=Int m4sFormulaCount(FS)
             andBool m4sFormulaCount(FS) <=Int 128
  rule m4sThresholdCheck(_,_,m4sTyped(m4sProp()))
    => m4sRejected("TYPE_THRESHOLD_BOUND") [owise]
  rule m4sThresholdCheck(_,_,_) => m4sRejected("TYPE_FORMULA") [owise]
  rule m4sStatic(E,F,P,N) => m4sForm(E,F,P,N)
    requires m4sEnvValid(E) andBool N >Int 0 andBool N <=Int 256
  rule m4sStatic(_,_,_,_) => m4sRejected("STATIC_ENV_OR_BOUND") [owise]

  // Mathematical integers model checked finite widths; no field reduction.
  // Result is a value or a named rejection, never a Boolean success flag.
  syntax M4SNumber ::= m4sCheckedAdd(Int, Int) [function, total]
                     | m4sCheckedSub(Int, Int) [function, total]
                     | m4sCheckedMul(Int, Int, Int) [function, total]
                     | m4sCheckedDivMod(Int, Int, Int, Int, Int, M4SRounding) [function, total]
                     | m4sRounded(Int, Int, M4SRounding) [function, total]
                     | m4sReserveUnit(M4SResidue, Int, Int) [function, total]
  rule m4sCheckedAdd(A,B) => m4sNumber(A +Int B)
    requires m4sUnsigned(A,128) andBool m4sUnsigned(B,128)
             andBool m4sUnsigned(A +Int B,128)
  rule m4sCheckedAdd(_,_) => m4sNumericReject("NUMERIC_OVERFLOW") [owise]
  rule m4sCheckedSub(A,B) => m4sNumber(A -Int B)
    requires m4sUnsigned(A,128) andBool m4sUnsigned(B,128) andBool A >=Int B
  rule m4sCheckedSub(_,_) => m4sNumericReject("NUMERIC_UNDERFLOW_OR_RANGE") [owise]
  rule m4sCheckedMul(A,B,W) => m4sNumber(A *Int B)
    requires (W ==Int 128 orBool W ==Int 256) andBool m4sUnsigned(A,128)
             andBool m4sUnsigned(B,128) andBool m4sUnsigned(A *Int B,W)
  rule m4sCheckedMul(_,_,_) => m4sNumericReject("NUMERIC_PRODUCT_WIDTH") [owise]
  rule m4sCheckedDivMod(_,D,_,_,_,_) => m4sNumericReject("NUMERIC_ZERO_DIVISOR")
    requires D ==Int 0
  rule m4sCheckedDivMod(N,D,Q,R,W,_) => m4sNumericReject("NUMERIC_RANGE")
    requires D =/=Int 0 andBool notBool ((W ==Int 128 orBool W ==Int 256)
      andBool m4sUnsigned(N,W) andBool m4sUnsigned(D,W)
      andBool m4sUnsigned(Q,128) andBool R >=Int 0 andBool R <Int D)
  rule m4sCheckedDivMod(N,D,Q,R,W,_) => m4sNumericReject("NUMERIC_FORGED_QUOTIENT")
    requires D >Int 0 andBool (W ==Int 128 orBool W ==Int 256)
      andBool m4sUnsigned(N,W) andBool m4sUnsigned(D,W)
      andBool m4sUnsigned(Q,128) andBool R >=Int 0 andBool R <Int D
      andBool N =/=Int Q *Int D +Int R
  rule m4sCheckedDivMod(N,D,Q,R,W,MODE) => m4sRounded(Q,R,MODE)
    requires D >Int 0 andBool (W ==Int 128 orBool W ==Int 256)
      andBool m4sUnsigned(N,W) andBool m4sUnsigned(D,W)
      andBool m4sUnsigned(Q,128) andBool R >=Int 0 andBool R <Int D
      andBool N ==Int Q *Int D +Int R
  rule m4sRounded(Q,R,m4sFloor()) => m4sNumber(Q)
    requires m4sUnsigned(Q,128) andBool R >=Int 0
  rule m4sRounded(Q,0,m4sCeil()) => m4sNumber(Q)
    requires m4sUnsigned(Q,128)
  rule m4sRounded(Q,R,m4sCeil()) => m4sNumber(Q +Int 1)
    requires R >Int 0 andBool m4sUnsigned(Q,128)
      andBool m4sUnsigned(Q +Int 1,128)
  rule m4sRounded(Q,R,m4sCeil()) => m4sNumericReject("NUMERIC_ROUND_OVERFLOW")
    requires R >Int 0 andBool m4sUnsigned(Q,128)
      andBool notBool m4sUnsigned(Q +Int 1,128)
  rule m4sRounded(Q,0,m4sExact()) => m4sNumber(Q)
    requires m4sUnsigned(Q,128)
  rule m4sRounded(_,R,m4sExact()) => m4sNumericReject("NUMERIC_INEXACT")
    requires R >Int 0
  rule m4sRounded(_,_,_) => m4sNumericReject("NUMERIC_ROUND_INPUT") [owise]

  // A fractional divmod remainder is not an asset unit. Only an integer
  // allocation residue can credit the protocol reserve, with conservation.
  rule m4sReserveUnit(m4sProductRemainder(_),_,_)
    => m4sNumericReject("NUMERIC_PRODUCT_RESIDUE")
  rule m4sReserveUnit(m4sWholeUnit(ASSET,"protocol-reserve",RESERVE),TOTAL,ALLOCATED)
    => m4sNumber(RESERVE)
    requires ASSET =/=String "" andBool m4sUnsigned(TOTAL,128)
      andBool m4sUnsigned(ALLOCATED,128) andBool m4sUnsigned(RESERVE,128)
      andBool TOTAL ==Int ALLOCATED +Int RESERVE
  rule m4sReserveUnit(_,_,_) => m4sNumericReject("NUMERIC_RESERVE_CONSERVATION") [owise]
endmodule
