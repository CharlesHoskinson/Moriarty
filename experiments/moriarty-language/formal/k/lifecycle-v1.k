// Isolated Core/4 extension of the expression definition; legacy files are unchanged.
requires "lifecycle-binding.k"
module MORIARTY-LIFECYCLE-EXPRESSION-V1
  imports MORIARTY-LIFECYCLE-BINDING
  syntax LCResult ::= lcPending() [symbol(lcPending)]
                    | lcExpressionRejected(String,EJSON,EJList,Int) [symbol(lcExpressionRejected)]
                    | lcPrepared(EJSON,EJSON,EJSON,Int) [symbol(lcPrepared)]
                    | lcKernelRejected(String,LCIndex) [symbol(lcKernelRejected)]
                    | lcAdapterRejected(String) [symbol(lcAdapterRejected)]
  syntax LCIndex ::= Int | lcActionIndexNull() [symbol(lcActionIndexNull)]
  syntax LCParsed ::= lcParseFailure() [symbol(lcParseFailure)] | lcParsed(EJSON) [symbol(lcParsed)]
  syntax LCFinancial ::= lcMissingContext() [symbol(lcMissingContext)] | lcText(String,LCParsed) [symbol(lcText)]
  // JSON lexical extensions preserve JSON.parse numbers and escaped lone surrogates.
  // These cannot satisfy any admitted financial scalar field.
  syntax EJSON ::= lcJSONNumber(String) [symbol(lcJSONNumber)]
                 | lcJSONString(String) [symbol(lcJSONString)]
                 | lcJSONPair(String,EJSON) [symbol(lcJSONPair)]
  syntax ERequest ::= lifecycleRequest(EJSON,EJSON,LCFinancial) [symbol(lifecycleRequest)]
                    | lifecycleMissingContext(EJSON,EJSON) [symbol(lifecycleMissingContext)]
  configuration
    <k> $PGM:ERequest </k>
    <sigma> ejMissing() </sigma> <request> ejMissing() </request>
    <pre> ejMissing() </pre> <args> ejMissing() </args> <obs> ejMissing() </obs>
    <localTypes> ejObject(ejNil()) </localTypes> <locals> ejObject(ejNil()) </locals>
    <written> ejObject(ejNil()) </written> <writes> ejObject(ejNil()) </writes>
    <descriptors> ejNil() </descriptors> <sourceBytes> 0 </sourceBytes>
    <initialWork> 0 </initialWork> <work> 0 </work>
    <reducing> false </reducing> <out> lcPending() </out>
    <financial> lcMissingContext() </financial>
    <financialPre> ejMissing() </financialPre> <financialPost> ejMissing() </financialPost>
    <stepTransfers> ejArray(ejNil()) </stepTransfers> <effects> ejNil() </effects>
    <actions> ejNil() </actions> <suffix> ejNil() </suffix> <suffixIndex> 0 </suffixIndex>
    <suffixWork> 0 </suffixWork>
  syntax KItem ::= lxInput() | lxSchemaPhase() | lxShapePhase() | lxTypePhase()
                 | lxSnapshotPhase() | lxBegin()
                 | lxRequire(String,EJSON,EJList) | lxCheckSite(LXSite)
                 | lxStop(String,EJSON,EJList)
                 | lxStaticStatements(EJList,Int,Bool)
                 | lxStaticResult(LXInference,EJSON,EJList)
                 | lxStaticBind(LXInference,String)
                 | lxRunStatements(EJList,Int)
                 | lxEnter(EJSON,EJList) | lxReturned(EJSON,EJSON)
                 | lxEvaluate(EJSON,EJList) | lxApply(String,EJSON,EJSON,EJList,EJSON)
                 | lxCollect(EJSON,EJList,EJList,EJList,Int)
                 | lxCollectValue(EJSON,EJList,EJList,EJList,Int)
                 | lxBooleanLeft(String,EJSON,EJList)
                 | lxDiscard()
  rule <k> lifecycleRequest(S,R,F) => lxInput() </k> <sigma> _ => S </sigma> <request> _ => R </request> <financial> _ => F </financial>
  rule <k> lifecycleMissingContext(S,R) => lxInput() </k> <sigma> _ => S </sigma> <request> _ => R </request>
  // Admission phases are separate continuations. No recursive snapshot traversal
  // is evaluated until the schema/cycle phase succeeds.
  rule <k> lxRequire("",_,_) => .K ... </k>
  rule <k> lxRequire(C,S,P) => lxStop(C,S,P) ... </k> requires C =/=String ""
  rule <k> lxStop(C,S,P) ~> _ => .K </k> <out> lcPending() => lcExpressionRejected(C,S,P,0) </out> <writes> _ => ejObject(ejNil()) </writes> <descriptors> _ => ejNil() </descriptors> <locals> _ => ejObject(ejNil()) </locals> <reducing> false </reducing> <financialPost> _ => ejMissing() </financialPost> <effects> _ => ejNil() </effects> <stepTransfers> _ => ejArray(ejNil()) </stepTransfers>
  rule <k> lxStop(C,S,P) ~> _ => .K </k> <out> lcPending() => lcExpressionRejected(C,S,P,I -Int W) </out> <writes> _ => ejObject(ejNil()) </writes> <descriptors> _ => ejNil() </descriptors> <locals> _ => ejObject(ejNil()) </locals> <reducing> true </reducing> <initialWork> I </initialWork> <work> W </work> <financialPost> _ => ejMissing() </financialPost> <effects> _ => ejNil() </effects> <stepTransfers> _ => ejArray(ejNil()) </stepTransfers>
  rule <k> lxInput() => lxRequire(lxCheck(lxJValid(S),"INPUT_SCHEMA"),lxSynthetic(),ejNil()) ~> lxRequire(lxCheck(lxJsonBytes(S) <=Int 65536,"INPUT_BOUND"),lxSynthetic(),ejNil()) ~> lxRequestInput() ... </k> <sigma> S </sigma>
  syntax KItem ::= lxRequestInput() | lxRequestFields() | lxWorkInput()
  rule <k> lxRequestInput() => lxRequire(lxCheck(lxJValid(R),"INPUT_SCHEMA"),lxSynthetic(),ejNil()) ~> lxRequire(lxCheck(ejClosed(R,lxRequestKeys()),"INPUT_SCHEMA"),lxSynthetic(),ejNil()) ~> lxRequestFields() ... </k> <request> R </request>
  rule <k> lxRequestFields() => lxRequire(lxCheck(ejGet(R,"contract") ==K ejString("moriarty-financial-expression-contract/4") andBool ejIsString(ejGet(R,"source")),"INPUT_SCHEMA"),lxSynthetic(),ejNil()) ~> lxRequire(lxCheck(lxUtf8(lxText(ejGet(R,"source"))) <=Int 65536,"INPUT_BOUND"),lxSynthetic(),ejNil()) ~> lxRequire(lxCheck(lxDecimal(ejGet(R,"workInitial")),"INPUT_SCHEMA"),lxSynthetic(),ejNil()) ~> lxWorkInput() ... </k> <request> R </request> <sourceBytes> _ => lxUtf8(lxText(ejGet(R,"source"))) </sourceBytes>
  rule <k> lxWorkInput() => lxRequire(lxCheck(lxInt(ejGet(R,"workInitial")) >=Int 0 andBool lxInt(ejGet(R,"workInitial")) <=Int 65536,"INPUT_BOUND"),lxSynthetic(),ejNil()) ~> lxRequire(lxCheck(lxComponentsBound(S,R),"INPUT_BOUND"),lxSynthetic(),ejNil()) ~> lxSchemaPhase() ... </k> <sigma> S </sigma> <request> R </request> <initialWork> _ => lxInt(ejGet(R,"workInitial")) </initialWork> <work> _ => lxInt(ejGet(R,"workInitial")) </work>
  rule <k> lxSchemaPhase() => lxRequire(lxSchemaShape(S),lxSynthetic(),ejNil()) ~> lxShapePhase() ... </k> <sigma> S </sigma>
  rule <k> lxShapePhase() => lxCheckSite(lxCoreShape(ejGet(R,"core"),B)) ~> lxRequire(lxSchemaDomain(S),lxSynthetic(),ejNil()) ~> lxTypePhase() ... </k> <sigma> S </sigma> <request> R </request> <sourceBytes> B </sourceBytes>
  rule <k> lxCheckSite(lxGood()) => .K ... </k>
  rule <k> lxCheckSite(lxBad(C,S,P)) => lxStop(C,S,P) ... </k>
  rule <k> lxTypePhase() => lxStaticStatements(lxUnwrapArray(ejGet(ejGet(R,"core"),"statements")),0,false) ~> lcFinancialPhase() ... </k> <request> R </request> requires ejHas(ejGet(R,"core"),"statements")
  rule <k> lxTypePhase() => lxStaticResult(lxInfer(ejGet(R,"core"),S,L,W,false,false,ejNil()),ejGet(R,"core"),ejNil()) ~> lcFinancialPhase() ... </k> <request> R </request> <sigma> S </sigma> <localTypes> L </localTypes> <written> W </written> requires notBool ejHas(ejGet(R,"core"),"statements")
  rule <k> lxStaticStatements(ejNil(),_,_) => .K ... </k>
  rule <k> lxStaticStatements(ejCons(N,Rest),I,E) => lxRequire(lxCheck(notBool E orBool ejGet(N,"constructor") ==K ejString("Ensure"),"TYPE_STATEMENT_PLACEMENT"),ejGet(N,"span"),ejCons(ejString(Int2String(I)),ejNil())) ~> lxStaticResult(lxInfer(N,S,L,W,ejGet(N,"constructor") ==K ejString("Ensure"),true,ejCons(ejString(Int2String(I)),ejNil())),N,ejCons(ejString(Int2String(I)),ejNil())) ~> lxStaticStatements(Rest,I +Int 1,E orBool ejGet(N,"constructor") ==K ejString("Ensure")) ... </k> <sigma> S </sigma> <localTypes> L </localTypes> <written> W </written>
  rule <k> lxStaticResult(lxTypeError(C,S,P),_,_) => lxStop(C,S,P) ... </k>
  rule <k> lxStaticResult(lxTyped(_),N,P) => lxStaticBind(lxInfer(ejGet(ejGet(N,"operands"),"value"),S,L,W,false,false,ejAppend(P,ejCons(ejString("0"),ejNil()))),lxText(ejGet(ejGet(N,"operands"),"name"))) ... </k> <sigma> S </sigma> <localTypes> L </localTypes> <written> W </written> requires ejGet(N,"constructor") ==K ejString("Let")
  rule <k> lxStaticBind(lxTyped(T),N) => .K ... </k> <localTypes> L => ejPut(L,N,T) </localTypes>
  rule <k> lxStaticBind(lxTypeError(C,S,P),_) => lxStop(C,S,P) ... </k>
  rule <k> lxStaticResult(lxTyped(_),N,_) => .K ... </k> <written> W => ejPut(W,lxText(ejGet(ejGet(N,"operands"),"field")),ejBool(true)) </written> requires ejGet(N,"constructor") ==K ejString("NextWrite")
  rule <k> lxStaticResult(lxTyped(_),N,_) => .K ... </k> requires ejGet(N,"constructor") =/=K ejString("NextWrite") andBool ejGet(N,"constructor") =/=K ejString("Let")
  rule <k> lxSnapshotPhase() => lxRequire(lxSnapshot(S,lxFieldTypes(ejGet(S,"fields")),ejGet(R,"Pre")),lxSynthetic(),ejNil()) ~> lxRequire(lxSnapshot(S,ejGet(S,"args"),ejGet(R,"Args")),lxSynthetic(),ejNil()) ~> lxRequire(lxSnapshot(S,ejGet(S,"observations"),ejGet(R,"Obs")),lxSynthetic(),ejNil()) ~> lxBegin() ... </k> <sigma> S </sigma> <request> R </request>
  rule <k> lxBegin() => lxLaunchCore(ejGet(R,"core")) ... </k> <request> R </request> <pre> _ => ejGet(R,"Pre") </pre> <args> _ => ejGet(R,"Args") </args> <obs> _ => ejGet(R,"Obs") </obs> <localTypes> _ => ejObject(ejNil()) </localTypes> <written> _ => ejObject(ejNil()) </written> <reducing> false => true </reducing>
  syntax KItem ::= lxLaunchCore(EJSON)
  rule <k> lxLaunchCore(N) => lcPrefix(lxUnwrapArray(ejGet(N,"statements")),0) ... </k> requires ejHas(N,"statements")
  rule <k> lxEnter(N,P) => lxStop("WORK_EXHAUSTED",ejGet(N,"span"),P) ... </k> <work> 0 </work>
  rule <k> lxEnter(N,P) => lxEvaluate(N,P) ... </k> <work> W => W -Int 1 </work> requires W >Int 0
  rule <k> lxRunStatements(ejNil(),_) => .K ... </k>
  rule <k> lxRunStatements(ejCons(N,L),I) => lxEnter(N,ejCons(ejString(Int2String(I)),ejNil())) ~> lxDiscard() ~> lxRunStatements(L,I +Int 1) ... </k>
  rule <k> lxReturned(_,_) ~> lxDiscard() => .K ... </k>

  syntax Bool ::= lxJValid(EJSON) [function, total]
                | lxJListValid(EJList) [function, total]
  rule lxJValid(ejString(_)) => true
  rule lxJValid(ejBool(_)) => true
  rule lxJValid(ejArray(L)) => lxJListValid(L)
  rule lxJValid(ejObject(L)) => lxJListValid(L)
  rule lxJValid(ejPair(_,V)) => lxJValid(V)
  rule lxJValid(_) => false [owise]
  rule lxJListValid(ejNil()) => true
  rule lxJListValid(ejCons(V,L)) => lxJValid(V) andBool lxJListValid(L)
  syntax Bool ::= lxComponentsBound(EJSON,EJSON) [function, total]
  rule lxComponentsBound(S,R) => lxJsonBytes(S) <=Int 65536 andBool lxJsonBytes(ejGet(R,"core")) <=Int 65536 andBool lxJsonBytes(ejGet(R,"Pre")) <=Int 65536 andBool lxJsonBytes(ejGet(R,"Args")) <=Int 65536 andBool lxJsonBytes(ejGet(R,"Obs")) <=Int 65536
  syntax EJList ::= lxRequestKeys() [function]
  rule lxRequestKeys() => ejCons(ejString("contract"),ejCons(ejString("source"),ejCons(ejString("core"),ejCons(ejString("Pre"),ejCons(ejString("Args"),ejCons(ejString("Obs"),ejCons(ejString("workInitial"),ejNil())))))))
  syntax EJSON ::= lxFieldTypes(EJSON) [function, total]
                 | lxOverlay(EJSON,EJSON) [function, total]
  syntax EJList ::= lxFieldTypeList(EJList) [function, total]
  rule lxFieldTypes(ejObject(L)) => ejObject(lxFieldTypeList(L))
  rule lxFieldTypes(_) => ejObject(ejNil()) [owise]
  rule lxFieldTypeList(ejNil()) => ejNil()
  rule lxFieldTypeList(ejCons(ejPair(N,F),L)) => ejCons(ejPair(N,ejGet(F,"type")),lxFieldTypeList(L))
  rule lxOverlay(P,ejObject(ejNil())) => P
  rule lxOverlay(P,ejObject(ejCons(ejPair(N,V),L))) => lxOverlay(ejPut(P,N,V),ejObject(L))
  rule lxOverlay(P,_) => P [owise]
  // Entry is charged once. Child frames retain original paths and spans.
  rule <k> lxEvaluate(N,P) => lxEnter(ejGet(ejGet(N,"operands"),"left"),ejAppend(P,ejCons(ejString("0"),ejNil()))) ~> lxBooleanLeft(lxText(ejGet(N,"constructor")),N,P) ... </k> requires ejGet(N,"constructor") ==K ejString("And") orBool ejGet(N,"constructor") ==K ejString("Or")
  rule <k> lxReturned(_,ejBool(false)) ~> lxBooleanLeft("And",_,_) => lxReturned(lxType("Bool"),ejBool(false)) ... </k>
  rule <k> lxReturned(_,ejBool(true)) ~> lxBooleanLeft("Or",_,_) => lxReturned(lxType("Bool"),ejBool(true)) ... </k>
  rule <k> lxReturned(_,ejBool(true)) ~> lxBooleanLeft("And",N,P) => lxEnter(ejGet(ejGet(N,"operands"),"right"),ejAppend(P,ejCons(ejString("1"),ejNil()))) ... </k>
  rule <k> lxReturned(_,ejBool(false)) ~> lxBooleanLeft("Or",N,P) => lxEnter(ejGet(ejGet(N,"operands"),"right"),ejAppend(P,ejCons(ejString("1"),ejNil()))) ... </k>
  rule <k> lxEvaluate(N,P) => lxCollect(N,P,lxChildren(lxText(ejGet(N,"constructor")),ejGet(N,"operands")),ejNil(),0) ... </k> requires ejGet(N,"constructor") =/=K ejString("Select") andBool ejGet(N,"constructor") =/=K ejString("And") andBool ejGet(N,"constructor") =/=K ejString("Or")
  rule <k> lxCollect(N,P,ejCons(C,R),V,I) => lxEnter(C,ejAppend(P,ejCons(ejString(Int2String(I)),ejNil()))) ~> lxCollectValue(N,P,R,V,I) ... </k>
  rule <k> lxReturned(_,V) ~> lxCollectValue(N,P,R,Vs,I) => lxCollect(N,P,R,ejAppend(Vs,ejCons(V,ejNil())),I +Int 1) ... </k>
  rule <k> lxCollect(N,P,ejNil(),V,_) => lxApplyTyped(lxInfer(N,S,L,ejObject(ejNil()),true,lxStatement(lxText(ejGet(N,"constructor"))),P),N,P,ejArray(V)) ... </k> <sigma> S </sigma> <localTypes> L </localTypes>
  syntax KItem ::= lxApplyTyped(LXInference,EJSON,EJList,EJSON)
                 | lxReturnChecked(EJSON,EJSON,EJSON,EJList)
  rule <k> lxApplyTyped(lxTyped(T),N,P,V) => lxApply(lxText(ejGet(N,"constructor")),ejGet(N,"operands"),T,P,ejArray(ejCons(ejGet(N,"span"),ejCons(V,ejNil())))) ... </k>
  rule <k> lxApplyTyped(lxTypeError(C,Span,P),_,_,_) => lxStop(C,Span,P) ... </k>
  // Payload packages span and operand values; it carries no host-generated verdict.
  rule <k> lxApply(K,O,T,P,Payload) => lxReturnChecked(T,lxLiteralValue(K,O),ejAt(Payload,0),P) ... </k> requires lxLiteral(K)
  rule <k> lxApply("ReadLocal",O,T,P,X) => lxReturnChecked(T,lxGetName(L,ejGet(O,"name")),ejAt(X,0),P) ... </k> <locals> L </locals>
  rule <k> lxApply("ReadArg",O,T,P,X) => lxReturnChecked(T,lxGetName(A,ejGet(O,"name")),ejAt(X,0),P) ... </k> <args> A </args>
  rule <k> lxApply("ReadObs",O,T,P,X) => lxReturnChecked(T,lxGetName(A,ejGet(O,"name")),ejAt(X,0),P) ... </k> <obs> A </obs>
  rule <k> lxApply("ReadPre",O,T,P,X) => lxReturnChecked(T,lxGetName(Pre,ejGet(O,"field")),ejAt(X,0),P) ... </k> <pre> Pre </pre> requires ejGet(O,"view") ==K ejString("pre")
  rule <k> lxApply("ReadPre",O,T,P,X) => lxReturnChecked(T,lxGetName(lxOverlay(Pre,W),ejGet(O,"field")),ejAt(X,0),P) ... </k> <pre> Pre </pre> <writes> W </writes> requires ejGet(O,"view") ==K ejString("post")
  rule <k> lxApply(K,O,T,P,X) => lxReturnChecked(T,lxGetName(ejAt(ejAt(X,1),0),ejGet(O,"field")),ejAt(X,0),P) ... </k> requires K ==String "ProjectField" orBool K ==String "AccessField"
  rule <k> lxApply(K,_,T,P,X) => lxRequire(lxCheck(lxInt(ejAt(ejAt(X,1),1)) <Int lxArrayLength(ejAt(ejAt(X,1),0)),"INDEX_RANGE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejAt(ejAt(ejAt(X,1),0),lxInt(ejAt(ejAt(X,1),1))),ejAt(X,0),P) ... </k> requires K ==String "ProjectIndex" orBool K ==String "AccessIndex"
  rule <k> lxApply("ConstructRecord",O,T,P,X) => lxReturnChecked(T,ejObject(lxSortPairs(lxZipFields(lxUnwrapArray(ejGet(O,"fields")),lxUnwrapArray(ejAt(X,1))))),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ConstructEnum",O,T,P,X) => lxReturnChecked(T,ejGet(O,"member"),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ConstructNone",_,T,P,X) => lxReturnChecked(T,ejArray(ejNil()),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ConstructSome",_,T,P,X) => lxReturnChecked(T,ejArray(ejCons(ejAt(ejAt(X,1),0),ejNil())),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ConstructCollection",_,T,P,X) => lxReturnChecked(T,ejAt(X,1),ejAt(X,0),P) ... </k>
  rule <k> lxApply(K,_,T,P,X) => lxArithmetic(K,T,lxInt(ejAt(ejAt(X,1),0)),lxInt(ejAt(ejAt(X,1),1)),ejAt(X,0),P) ... </k> requires K ==String "Add" orBool K ==String "Sub" orBool K ==String "Mul" orBool K ==String "FloorDiv" orBool K ==String "CeilDiv"
  rule <k> lxApply("Eq",_,T,P,X) => lxReturnChecked(T,ejBool(ejAt(ejAt(X,1),0) ==K ejAt(ejAt(X,1),1)),ejAt(X,0),P) ... </k>
  rule <k> lxApply(K,_,T,P,X) => lxReturnChecked(T,ejBool(lxCompare(K,lxInt(ejAt(ejAt(X,1),0)),lxInt(ejAt(ejAt(X,1),1)))),ejAt(X,0),P) ... </k> requires K ==String "Lt" orBool K ==String "Lte" orBool K ==String "Gt" orBool K ==String "Gte"
  rule <k> lxApply("Not",_,T,P,X) => lxReturnChecked(T,ejBool(ejAt(ejAt(X,1),0) ==K ejBool(false)),ejAt(X,0),P) ... </k>
  rule <k> lxApply(K,_,T,P,X) => lxRequire(lxCheck(ejAt(ejAt(X,1),0) ==K ejBool(true),lxGuardCode(K)),ejAt(X,0),P) ~> lxReturned(T,ejMissing()) ... </k> requires K ==String "Require" orBool K ==String "Ensure"
  rule <k> lxApply("Let",O,T,_,X) => lxReturned(T,ejMissing()) ... </k> <locals> L => ejPut(L,lxText(ejGet(O,"name")),ejAt(ejAt(X,1),0)) </locals> <localTypes> LT => ejPut(LT,lxText(ejGet(O,"name")),lxInferredType(lxInfer(ejGet(O,"value"),S,LT,ejObject(ejNil()),false,false,ejNil()))) </localTypes> <sigma> S </sigma>
  rule <k> lxApply("NextWrite",O,T,_,X) => lxReturned(T,ejMissing()) ... </k> <writes> W => ejPut(W,lxText(ejGet(O,"field")),ejAt(ejAt(X,1),0)) </writes>
  rule <k> lxApply("Emit",O,T,P,X) => lxEmitCheck(T,ejObject(ejCons(ejPair("fields",ejAt(ejAt(X,1),0)),ejCons(ejPair("operation",ejGet(O,"operation")),ejNil()))),ejAt(X,0),P) ... </k>
  rule <k> lxReturnChecked(T,V,Span,P) => lxRequire(lxValueBound(S,T,V,"VALUE_BOUND"),Span,P) ~> lxReturned(T,V) ... </k> <sigma> S </sigma>
  syntax KItem ::= lxArithmetic(String,EJSON,Int,Int,EJSON,EJList)
                 | lxArithmeticResult(EJSON,Int,EJSON,EJList)
                 | lxDivide(String,EJSON,Int,Int,EJSON,EJList)
                 | lxEmitCheck(EJSON,EJSON,EJSON,EJList)
                 | lxEmitAppend(EJSON,EJSON,EJSON,EJList)
  rule <k> lxArithmetic("Add",T,A,B,S,P) => lxArithmeticResult(T,A +Int B,S,P) ... </k>
  rule <k> lxArithmetic("Sub",T,A,B,S,P) => lxArithmeticResult(T,A -Int B,S,P) ... </k>
  rule <k> lxArithmetic("Mul",T,A,B,S,P) => lxArithmeticResult(T,A *Int B,S,P) ... </k>
  rule <k> lxArithmetic(K,T,A,B,S,P) => lxRequire(lxCheck(B >Int 0,"ARITH_DENOMINATOR"),S,P) ~> lxDivide(K,T,A,B,S,P) ... </k> requires K ==String "FloorDiv" orBool K ==String "CeilDiv"
  rule <k> lxDivide("FloorDiv",T,A,B,S,P) => lxArithmeticResult(T,lxFloor(A,B),S,P) ... </k> requires B >Int 0
  rule <k> lxDivide("CeilDiv",T,A,B,S,P) => lxArithmeticResult(T,0 -Int lxFloor(0 -Int A,B),S,P) ... </k> requires B >Int 0
  rule <k> lxArithmeticResult(T,N,S,P) => lxRequire(lxCheck(lxFits(T,ejString(Int2String(N))),"ARITH_RANGE"),S,P) ~> lxReturnChecked(T,ejString(Int2String(N)),S,P) ... </k>
  syntax Int ::= lxFloor(Int,Int) [function]
  rule lxFloor(A,B) => A /Int B requires B >Int 0 andBool A >=Int 0
  rule lxFloor(A,B) => (0 -Int ((0 -Int A) /Int B)) -Int 1 requires B >Int 0 andBool A <Int 0 andBool (0 -Int A) %Int B =/=Int 0
  rule lxFloor(A,B) => 0 -Int ((0 -Int A) /Int B) requires B >Int 0 andBool A <Int 0 andBool (0 -Int A) %Int B ==Int 0
  syntax Bool ::= lxCompare(String,Int,Int) [function, total]
  rule lxCompare("Lt",A,B) => A <Int B
  rule lxCompare("Lte",A,B) => A <=Int B
  rule lxCompare("Gt",A,B) => A >Int B
  rule lxCompare("Gte",A,B) => A >=Int B
  rule lxCompare(_,_,_) => false [owise]
  syntax String ::= lxGuardCode(String) [function, total]
  rule lxGuardCode("Require") => "GUARD_FAILED"
  rule lxGuardCode(_) => "ENSURES_FAILED" [owise]
  syntax EJSON ::= lxInferredType(LXInference) [function, total]
  rule lxInferredType(lxTyped(T)) => T
  rule lxInferredType(_) => ejMissing() [owise]
  syntax EJList ::= lxZipFields(EJList,EJList) [function, total]
  rule lxZipFields(ejNil(),ejNil()) => ejNil()
  rule lxZipFields(ejCons(F,L),ejCons(V,R)) => ejCons(ejPair(lxText(ejGet(F,"name")),V),lxZipFields(L,R))
  rule lxZipFields(_,_) => ejNil() [owise]
  rule <k> lxEmitCheck(T,D,Span,P) => lxRequire(lxValueBound(S,lxType("Operation",ejGet(D,"operation")),D,"DESCRIPTOR_BOUND"),Span,P) ~> lxEmitAppend(T,D,Span,P) ... </k> <sigma> S </sigma>
  rule <k> lxEmitAppend(T,D,Span,P) => lxRequire(lxDescriptorBound(S,ejAppend(Ds,ejCons(D,ejNil()))),Span,P) ~> lxReturned(T,ejMissing()) ... </k> <sigma> S </sigma> <descriptors> Ds => ejAppend(Ds,ejCons(D,ejNil())) </descriptors>
  syntax String ::= lxDescriptorBound(EJSON,EJList) [function, total]
  syntax EJList ::= lxDescriptorChildren(EJList) [function, total]
  rule lxDescriptorChildren(ejNil()) => ejNil()
  rule lxDescriptorChildren(ejCons(D,L)) => ejCons(ejArray(ejCons(lxType("Operation",ejGet(D,"operation")),ejCons(D,ejNil()))),lxDescriptorChildren(L))
  rule lxDescriptorBound(S,L) => lxCheck(ejLength(L) <=Int 128 andBool lxJsonBytes(ejArray(L)) <=Int 65536 andBool 1 +Int lxChildrenNodes(S,lxDescriptorChildren(L)) <=Int 4096 andBool 1 +Int lxChildrenDepth(S,lxDescriptorChildren(L)) <=Int 64,"DESCRIPTOR_BOUND")

  syntax KItem ::= lcFinancialPhase() | lcFinancialContext() | lcAdmitted(EJSON)
                 | lcAdapterCheck(String) | lcKernelCheck(String,LCIndex)
                 | lcAdapterStop(String) | lcKernelStop(String,LCIndex)
                 | lcPrefix(EJList,Int) | lcPrefixDone() | lcBindKernel()
                 | lcMapAll(EJList) | lcCheckActions(EJList) | lcKernelStart()
                 | lcRunActions(EJList,Int) | lcApplyResult(LCApply,Int)
                 | lcKernelDone() | lcSuffixStart() | lcPublish()
                 | lcSelect(EJSON,EJList)
  rule <k> lcFinancialPhase() => lxRequire(lxCheck(ejHas(ejGet(R,"core"),"statements"),"TYPE_ACTION_REQUIRED"),lxSynthetic(),ejNil()) ~> lcFinancialContext() ... </k> <request> R </request>
  rule <k> lcFinancialContext() => lxStop("FINANCIAL_CONTEXT_REQUIRED",lxSynthetic(),ejNil()) ... </k> <financial> lcMissingContext() </financial>
  rule <k> lcFinancialContext() => lcAdapterStop("INPUT_BOUND") ... </k> <financial> lcText(T,_) </financial> requires lxUtf8(T) >Int 65536 orBool lcUtf16(T) >Int 65536
  rule <k> lcFinancialContext() => lcAdapterStop("INPUT_SCHEMA") ... </k> <financial> lcText(T,lcParseFailure()) </financial> requires lxUtf8(T) <=Int 65536 andBool lcUtf16(T) <=Int 65536
  rule <k> lcFinancialContext() => lcKernelCheck(lcState(F),lcActionIndexNull()) ~> lcAdmitted(F) ... </k> <financial> lcText(T,lcParsed(F)) </financial> requires lxUtf8(T) <=Int 65536 andBool lcUtf16(T) <=Int 65536
  rule <k> lcAdmitted(F) => lcAdapterCheck(lxCheck(ejGet(R,"workInitial") ==K ejGet(ejGet(F,"work"),"remaining"),"WORK_MISMATCH")) ~> lxSnapshotPhase() ... </k> <request> R </request> <financialPre> _ => F </financialPre> <financialPost> _ => F </financialPost>
  rule <k> lcAdapterCheck("") => .K ... </k>
  rule <k> lcAdapterCheck(C) => lcAdapterStop(C) ... </k> requires C =/=String ""
  rule <k> lcKernelCheck("",_) => .K ... </k>
  rule <k> lcKernelCheck(C,I) => lcKernelStop(C,I) ... </k> requires C =/=String ""
  rule <k> lcAdapterStop(C) ~> _ => .K </k> <out> lcPending() => lcAdapterRejected(C) </out> <financialPost> _ => ejMissing() </financialPost> <effects> _ => ejNil() </effects> <writes> _ => ejObject(ejNil()) </writes> <descriptors> _ => ejNil() </descriptors> <stepTransfers> _ => ejArray(ejNil()) </stepTransfers>
  rule <k> lcKernelStop(C,I) ~> _ => .K </k> <out> lcPending() => lcKernelRejected(C,I) </out> <financialPost> _ => ejMissing() </financialPost> <effects> _ => ejNil() </effects> <writes> _ => ejObject(ejNil()) </writes> <descriptors> _ => ejNil() </descriptors> <stepTransfers> _ => ejArray(ejNil()) </stepTransfers>
  rule <k> lcPrefix(ejNil(),I) => lcPrefixDone() ... </k> <suffixIndex> _ => I </suffixIndex>
  rule <k> lcPrefix(ejCons(N,L),I) => lcPrefixDone() ... </k> <suffix> _ => ejCons(N,L) </suffix> <suffixIndex> _ => I </suffixIndex> requires ejGet(N,"constructor") ==K ejString("Ensure")
  rule <k> lcPrefix(ejCons(N,L),I) => lxEnter(N,ejCons(ejString(Int2String(I)),ejNil())) ~> lxDiscard() ~> lcPrefix(L,I +Int 1) ... </k> requires ejGet(N,"constructor") =/=K ejString("Ensure")
  rule <k> lcPrefixDone() => lxRequire(lxSnapshotBounds(S,lxFieldTypes(ejGet(S,"fields")),lxOverlay(P,W),"VALUE_BOUND"),ejGet(ejGet(R,"core"),"span"),ejNil()) ~> lcBindKernel() ... </k> <sigma> S </sigma> <pre> P </pre> <writes> W </writes> <request> R </request>
  rule <k> lcBindKernel() => lcAdapterCheck(lxCheck(ejLength(D) >Int 0,"EMPTY_BATCH")) ~> lcAdapterCheck(lcNoFinancialFields(S)) ~> lcAdapterCheck(lcBinding(S)) ~> lcMapAll(D) ~> lcKernelStart() ... </k> <sigma> S </sigma> <descriptors> D </descriptors>
  rule <k> lcMapAll(ejNil()) => .K ... </k>
  rule <k> lcMapAll(ejCons(D,L)) => lcAdapterCheck(lcDescriptorCode(D,S)) ~> lcMapAll(L) ... </k> <sigma> S </sigma> <actions> A => ejAppend(A,ejCons(lcMapDescriptor(D),ejNil())) </actions>
  rule <k> lcCheckActions(ejNil()) => .K ... </k>
  rule <k> lcCheckActions(ejCons(A,L)) => lcKernelCheck(lcActionCode(A),lcActionIndexNull()) ~> lcCheckActions(L) ... </k>
  rule <k> lcKernelStart() => lcAdapterCheck(lcDebitCode(F,I -Int W)) ~> lcKernelCheck(lxCheck(lcJsonUnits(lcKernelInput(lcDebit(F,I -Int W),A)) <=Int 65536,"INPUT_UTF16_LENGTH"),lcActionIndexNull()) ~> lcKernelCheck(lxCheck(lxJsonBytes(lcKernelInput(lcDebit(F,I -Int W),A)) <=Int 65536,"INPUT_UTF8_LENGTH"),lcActionIndexNull()) ~> lcCheckActions(A) ~> lcKernelCheck(lxCheck(W >=Int ejLength(A),"INSUFFICIENT_WORK"),lcActionIndexNull()) ~> lcKernelCheck(lxCheck(lxInt(ejGet(ejGet(F,"work"),"spent")) +Int (I -Int W) +Int ejLength(A) <=Int lcU128(),"OVERFLOW"),lcActionIndexNull()) ~> lcRunActions(A,0) ~> lcKernelDone() ... </k> <actions> A </actions> <initialWork> I </initialWork> <work> W </work> <financialPost> F => lcDebit(F,I -Int W) </financialPost>
  rule <k> lcRunActions(ejNil(),_) => .K ... </k>
  rule <k> lcRunActions(ejCons(A,L),I) => lcApplyResult(lcApply(A,F,T),I) ~> lcRunActions(L,I +Int 1) ... </k> <financialPost> F </financialPost> <stepTransfers> T </stepTransfers>
  rule <k> lcApplyResult(lcFailed(C),I) => lcKernelStop(C,I) ... </k>
  rule <k> lcApplyResult(lcApplied(F,T,E),_) => .K ... </k> <financialPost> _ => F </financialPost> <stepTransfers> _ => T </stepTransfers> <effects> L => ejAppend(L,ejCons(E,ejNil())) </effects>
  rule <k> lcKernelDone() => lcKernelCheck(lxCheck(lxJsonBytes(lcDebit(F,ejLength(A))) <=Int 65536,"RESULT_BOUND"),lcActionIndexNull()) ~> lcKernelCheck(lcState(lcDebit(F,ejLength(A))),lcActionIndexNull()) ~> lcAdapterCheck(lcNominalUnits(A,F,lcUnit(S))) ~> lcSuffixStart() ... </k> <sigma> S </sigma> <actions> A </actions> <financialPost> F => lcDebit(F,ejLength(A)) </financialPost> <work> W => W -Int ejLength(A) </work>
  rule <k> lcSuffixStart() => lxRunStatements(L,I) ~> lcPublish() ... </k> <suffix> L </suffix> <suffixIndex> I </suffixIndex> <work> W </work> <suffixWork> _ => W </suffixWork>
  rule <k> lcPublish() => lcAdapterCheck(lcDebitCode(F,B -Int W)) ~> lcKernelCheck(lxCheck(lxJsonBytes(lcDebit(F,B -Int W)) <=Int 65536,"RESULT_BOUND"),lcActionIndexNull()) ~> lcKernelCheck(lcState(lcDebit(F,B -Int W)),lcActionIndexNull()) ~> lcCommit() ... </k> <financialPost> F => lcDebit(F,B -Int W) </financialPost> <suffixWork> B </suffixWork> <work> W </work>
  syntax KItem ::= lcCommit()
  rule <k> lcCommit() => .K </k> <out> lcPending() => lcPrepared(lxOverlay(P,Wr),F,ejArray(E),W) </out> <pre> P </pre> <writes> Wr </writes> <financialPost> F </financialPost> <effects> E </effects> <work> W </work>
  syntax EJSON ::= lcDebit(EJSON,Int) [function, total]
  rule lcDebit(F,N) => ejPut(F,"work",ejPut(ejPut(ejGet(F,"work"),"remaining",ejString(Int2String(lxInt(ejGet(ejGet(F,"work"),"remaining")) -Int N))),"spent",ejString(Int2String(lxInt(ejGet(ejGet(F,"work"),"spent")) +Int N))))
  syntax String ::= lcDebitCode(EJSON,Int) [function, total]
  rule lcDebitCode(F,N) => lxFirst(lxCheck(lxInt(ejGet(ejGet(F,"work"),"remaining")) >=Int N,"OVERFLOW"),lxCheck(lxInt(ejGet(ejGet(F,"work"),"spent")) +Int N <=Int lcU128(),"OVERFLOW"))
  syntax Int ::= lcUtf16(String) [function, total]
  rule lcUtf16("") => 0
  rule lcUtf16(S) => lcUtf16Unit(ordChar(lxSubstring(S,0,1))) +Int lcUtf16(lxSubstring(S,1,lengthString(S))) requires S =/=String ""
  syntax Int ::= lcUtf16Unit(Int) [function, total]
  // Count UTF-16 units from valid UTF-8 bytes: continuations add no units.
  rule lcUtf16Unit(N) => 0 requires N >=Int 128 andBool N <Int 192
  rule lcUtf16Unit(N) => 2 requires N >=Int 240
  rule lcUtf16Unit(N) => 1 requires N <Int 128 orBool (N >=Int 192 andBool N <Int 240)
  rule <k> lxEvaluate(N,P) => lxEnter(ejGet(ejGet(N,"operands"),"condition"),ejAppend(P,ejCons(ejString("0"),ejNil()))) ~> lcSelect(N,P) ... </k> requires ejGet(N,"constructor") ==K ejString("Select")
  rule <k> lxReturned(_,ejBool(true)) ~> lcSelect(N,P) => lxEnter(ejGet(ejGet(N,"operands"),"consequent"),ejAppend(P,ejCons(ejString("1"),ejNil()))) ... </k>
  rule <k> lxReturned(_,ejBool(false)) ~> lcSelect(N,P) => lxEnter(ejGet(ejGet(N,"operands"),"alternative"),ejAppend(P,ejCons(ejString("2"),ejNil()))) ... </k>
  rule <k> lxApply("ConstructAmount",_,T,P,X) => lxReturnChecked(T,ejAt(ejAt(X,1),0),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ConvertUInt",_,T,P,X) => lxRequire(lxCheck(lxFits(T,ejAt(ejAt(X,1),0)),"ARITH_RANGE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejAt(ejAt(X,1),0),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ProjectSome",_,T,P,X) => lxRequire(lxCheck(lxArrayLength(ejAt(ejAt(X,1),0)) ==Int 1,"OPTION_NONE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejAt(ejAt(ejAt(X,1),0),0),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ScalarValue",O,T,P,X) => lxReturnChecked(T,lcScalarValue(ejGet(O,"component"),ejAt(ejAt(X,1),0)),ejAt(X,0),P) ... </k>
  syntax EJSON ::= lcScalarValue(EJSON,EJSON) [function, total]
  rule lcScalarValue(ejString("negative"),V) => ejBool(lxInt(V) <Int 0)
  rule lcScalarValue(ejString("magnitude"),V) => ejString(Int2String(absInt(lxInt(V))))
  rule lcScalarValue(_,V) => V [owise]
  rule <k> lxApply("ReadOutstanding",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)) =/=K ejMissing(),"MISSING_OBLIGATION"),ejAt(X,0),P) ~> lxRequire(lxCheck(ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"denomination") ==K ejGet(O,"unit"),"NOMINAL_UNIT"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"outstanding"),ejAt(X,0),P) ... </k> <financialPre> F </financialPre>
  rule <k> lxApply("ReadPrincipal",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)) =/=K ejMissing(),"MISSING_OBLIGATION"),ejAt(X,0),P) ~> lxRequire(lxCheck(ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"denomination") ==K ejGet(O,"unit"),"NOMINAL_UNIT"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"principal"),ejAt(X,0),P) ... </k> <financialPre> F </financialPre>
  rule <k> lxApply("ReadAccrued",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)) =/=K ejMissing(),"MISSING_OBLIGATION"),ejAt(X,0),P) ~> lxRequire(lxCheck(ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"denomination") ==K ejGet(O,"unit"),"NOMINAL_UNIT"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"accrued"),ejAt(X,0),P) ... </k> <financialPre> F </financialPre>
  rule <k> lxApply("ReadBalance",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFindPair(ejGet(F,"balances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")) =/=K ejMissing(),"MISSING_BALANCE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFindPair(ejGet(F,"balances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")),"amount"),ejAt(X,0),P) ... </k> <financialPre> F </financialPre>
  rule <k> lxApply("ReadAllowanceRemaining",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")) =/=K ejMissing(),"MISSING_ALLOWANCE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")),"remaining"),ejAt(X,0),P) ... </k> <financialPre> F </financialPre>
  rule <k> lxApply("ReadAllowanceSpent",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")) =/=K ejMissing(),"MISSING_ALLOWANCE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")),"spent"),ejAt(X,0),P) ... </k> <financialPre> F </financialPre>
  rule <k> lxApply("ReadPostOutstanding",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)) =/=K ejMissing(),"MISSING_OBLIGATION"),ejAt(X,0),P) ~> lxRequire(lxCheck(ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"denomination") ==K ejGet(O,"unit"),"NOMINAL_UNIT"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"outstanding"),ejAt(X,0),P) ... </k> <financialPost> F </financialPost>
  rule <k> lxApply("ReadPostPrincipal",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)) =/=K ejMissing(),"MISSING_OBLIGATION"),ejAt(X,0),P) ~> lxRequire(lxCheck(ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"denomination") ==K ejGet(O,"unit"),"NOMINAL_UNIT"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"principal"),ejAt(X,0),P) ... </k> <financialPost> F </financialPost>
  rule <k> lxApply("ReadPostAccrued",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)) =/=K ejMissing(),"MISSING_OBLIGATION"),ejAt(X,0),P) ~> lxRequire(lxCheck(ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"denomination") ==K ejGet(O,"unit"),"NOMINAL_UNIT"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFind(ejGet(F,"obligations"),"id",ejAt(ejAt(X,1),0)),"accrued"),ejAt(X,0),P) ... </k> <financialPost> F </financialPost>
  rule <k> lxApply("ReadPostBalance",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFindPair(ejGet(F,"balances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")) =/=K ejMissing(),"MISSING_BALANCE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFindPair(ejGet(F,"balances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")),"amount"),ejAt(X,0),P) ... </k> <financialPost> F </financialPost>
  rule <k> lxApply("ReadPostAllowanceRemaining",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")) =/=K ejMissing(),"MISSING_ALLOWANCE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")),"remaining"),ejAt(X,0),P) ... </k> <financialPost> F </financialPost>
  rule <k> lxApply("ReadPostAllowanceSpent",O,T,P,X) => lxRequire(lxCheck(lcIdentifier(ejAt(ejAt(X,1),0)),"INVALID_IDENTIFIER"),ejAt(X,0),P) ~> lxRequire(lxCheck(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")) =/=K ejMissing(),"MISSING_ALLOWANCE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(lcFindPair(ejGet(F,"allowances"),ejAt(ejAt(X,1),0),ejGet(O,"asset")),"spent"),ejAt(X,0),P) ... </k> <financialPost> F </financialPost>

  rule <k> lxApply("ConstructShares",_,T,P,X) => lxReturnChecked(T,ejAt(ejAt(X,1),0),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ConstructVariant",O,T,P,X) => lxReturnChecked(T,ejObject(ejCons(ejPair("tag",ejGet(O,"tag")),ejCons(ejPair("value",ejAt(ejAt(X,1),0)),ejNil()))),ejAt(X,0),P) ... </k>
  rule <k> lxApply("ProjectVariant",O,T,P,X) => lxRequire(lxCheck(ejGet(ejAt(ejAt(X,1),0),"tag") ==K ejGet(O,"tag"),"VARIANT_CASE"),ejAt(X,0),P) ~> lxReturnChecked(T,ejGet(ejAt(ejAt(X,1),0),"value"),ejAt(X,0),P) ... </k>
  syntax EJSON ::= lcKernelInput(EJSON,EJList) [function, total]
  rule lcKernelInput(F,A) => ejObject(ejCons(ejPair("actions",ejArray(A)),ejCons(ejPair("schemaVersion",ejString("moriarty-financial-lifecycle/1")),ejCons(ejPair("state",F),ejNil()))))
  rule lcQuotedUnits("") => 2
  rule lcQuotedUnits(S) => lcEscapedUnits(ordChar(lxSubstring(S,0,1))) +Int lcQuotedUnits(lxSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule lcJsonUnits(ejString(S)) => lcQuotedUnits(S)
  rule lcJsonUnits(ejBool(true)) => 4
  rule lcJsonUnits(ejBool(false)) => 5
  rule lcJsonUnits(ejNull()) => 4
  rule lcJsonUnits(ejNumber(N)) => lengthString(Int2String(N))
  rule lcJsonUnits(ejArray(L)) => 2 +Int lcListUnits(L)
  rule lcJsonUnits(ejObject(L)) => 2 +Int lcListUnits(L)
  rule lcJsonUnits(ejPair(K,V)) => lcQuotedUnits(K) +Int 1 +Int lcJsonUnits(V)
  rule lcJsonUnits(ejMissing()) => 0
  rule lcListUnits(ejNil()) => 0
  rule lcListUnits(ejCons(V,ejNil())) => lcJsonUnits(V)
  rule lcListUnits(ejCons(V,ejCons(W,L))) => lcJsonUnits(V) +Int 1 +Int lcListUnits(ejCons(W,L))
  syntax Int ::= lcJsonUnits(EJSON) [function, total] | lcListUnits(EJList) [function, total] | lcQuotedUnits(String) [function, total] | lcEscapedUnits(Int) [function, total]
  rule lcEscapedUnits(N) => lxEscapedBytes(N) requires N <Int 128
  rule lcEscapedUnits(N) => lcUtf16Unit(N) requires N >=Int 128
endmodule
