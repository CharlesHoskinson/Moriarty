requires "expression-infer.k"
module MORIARTY-EXPRESSION-V1
  imports MORIARTY-EXPRESSION-INFER
  syntax EXResult ::= exPending() [symbol(exPending)] | exRejected(String,EJSON,EJList,Int) [symbol(exRejected)]
                    | exValue(EJSON,EJSON,Int) [symbol(exValue)] | exPrepared(EJSON,EJList,Int) [symbol(exPrepared)]
  configuration
    <k> $PGM:ERequest </k>
    <sigma> ejMissing() </sigma> <request> ejMissing() </request>
    <pre> ejMissing() </pre> <args> ejMissing() </args> <obs> ejMissing() </obs>
    <localTypes> ejObject(ejNil()) </localTypes> <locals> ejObject(ejNil()) </locals>
    <written> ejObject(ejNil()) </written> <writes> ejObject(ejNil()) </writes>
    <descriptors> ejNil() </descriptors> <sourceBytes> 0 </sourceBytes>
    <initialWork> 0 </initialWork> <work> 0 </work>
    <reducing> false </reducing> <out> exPending() </out>
  syntax KItem ::= exInput() | exSchemaPhase() | exShapePhase() | exTypePhase()
                 | exSnapshotPhase() | exBegin() | exFinish()
                 | exRequire(String,EJSON,EJList) | exCheckSite(EXSite)
                 | exStop(String,EJSON,EJList)
                 | exStaticStatements(EJList,Int,Bool)
                 | exStaticResult(EXInference,EJSON,EJList)
                 | exStaticBind(EXInference,String)
                 | exRunStatements(EJList,Int)
                 | exEnter(EJSON,EJList) | exReturned(EJSON,EJSON)
                 | exEvaluate(EJSON,EJList) | exApply(String,EJSON,EJSON,EJList,EJSON)
                 | exCollect(EJSON,EJList,EJList,EJList,Int)
                 | exCollectValue(EJSON,EJList,EJList,EJList,Int)
                 | exBooleanLeft(String,EJSON,EJList)
                 | exPublishExpression() | exDiscard()
  rule <k> expressionRequest(S,R) => exInput() </k> <sigma> _ => S </sigma> <request> _ => R </request>
  // Admission phases are separate continuations. No recursive snapshot traversal
  // is evaluated until the schema/cycle phase succeeds.
  rule <k> exRequire("",_,_) => .K ... </k>
  rule <k> exRequire(C,S,P) => exStop(C,S,P) ... </k> requires C =/=String ""
  rule <k> exStop(C,S,P) ~> _ => .K </k> <out> exPending() => exRejected(C,S,P,0) </out> <writes> _ => ejObject(ejNil()) </writes> <descriptors> _ => ejNil() </descriptors> <locals> _ => ejObject(ejNil()) </locals> <reducing> false </reducing>
  rule <k> exStop(C,S,P) ~> _ => .K </k> <out> exPending() => exRejected(C,S,P,I -Int W) </out> <writes> _ => ejObject(ejNil()) </writes> <descriptors> _ => ejNil() </descriptors> <locals> _ => ejObject(ejNil()) </locals> <reducing> true </reducing> <initialWork> I </initialWork> <work> W </work>
  rule <k> exInput() => exRequire(exCheck(exJValid(S),"INPUT_SCHEMA"),exSynthetic(),ejNil()) ~> exRequire(exCheck(exJsonBytes(S) <=Int 65536,"INPUT_BOUND"),exSynthetic(),ejNil()) ~> exRequestInput() ... </k> <sigma> S </sigma>
  syntax KItem ::= exRequestInput() | exRequestFields() | exWorkInput()
  rule <k> exRequestInput() => exRequire(exCheck(exJValid(R),"INPUT_SCHEMA"),exSynthetic(),ejNil()) ~> exRequire(exCheck(ejClosed(R,exRequestKeys()),"INPUT_SCHEMA"),exSynthetic(),ejNil()) ~> exRequestFields() ... </k> <request> R </request>
  rule <k> exRequestFields() => exRequire(exCheck(ejGet(R,"contract") ==K ejString("moriarty-expression-contract/1") andBool ejIsString(ejGet(R,"source")),"INPUT_SCHEMA"),exSynthetic(),ejNil()) ~> exRequire(exCheck(exUtf8(exText(ejGet(R,"source"))) <=Int 65536,"INPUT_BOUND"),exSynthetic(),ejNil()) ~> exRequire(exCheck(exDecimal(ejGet(R,"workInitial")),"INPUT_SCHEMA"),exSynthetic(),ejNil()) ~> exWorkInput() ... </k> <request> R </request> <sourceBytes> _ => exUtf8(exText(ejGet(R,"source"))) </sourceBytes>
  rule <k> exWorkInput() => exRequire(exCheck(exInt(ejGet(R,"workInitial")) >=Int 0 andBool exInt(ejGet(R,"workInitial")) <=Int 65536,"INPUT_BOUND"),exSynthetic(),ejNil()) ~> exRequire(exCheck(exComponentsBound(S,R),"INPUT_BOUND"),exSynthetic(),ejNil()) ~> exSchemaPhase() ... </k> <sigma> S </sigma> <request> R </request> <initialWork> _ => exInt(ejGet(R,"workInitial")) </initialWork> <work> _ => exInt(ejGet(R,"workInitial")) </work>
  rule <k> exSchemaPhase() => exRequire(exSchemaShape(S),exSynthetic(),ejNil()) ~> exShapePhase() ... </k> <sigma> S </sigma>
  rule <k> exShapePhase() => exCheckSite(exCoreShape(ejGet(R,"core"),B)) ~> exRequire(exSchemaDomain(S),exSynthetic(),ejNil()) ~> exTypePhase() ... </k> <sigma> S </sigma> <request> R </request> <sourceBytes> B </sourceBytes>
  rule <k> exCheckSite(exGood()) => .K ... </k>
  rule <k> exCheckSite(exBad(C,S,P)) => exStop(C,S,P) ... </k>
  rule <k> exTypePhase() => exStaticStatements(exUnwrapArray(ejGet(ejGet(R,"core"),"statements")),0,false) ~> exSnapshotPhase() ... </k> <request> R </request> requires ejHas(ejGet(R,"core"),"statements")
  rule <k> exTypePhase() => exStaticResult(exInfer(ejGet(R,"core"),S,L,W,false,false,ejNil()),ejGet(R,"core"),ejNil()) ~> exSnapshotPhase() ... </k> <request> R </request> <sigma> S </sigma> <localTypes> L </localTypes> <written> W </written> requires notBool ejHas(ejGet(R,"core"),"statements")
  rule <k> exStaticStatements(ejNil(),_,_) => .K ... </k>
  rule <k> exStaticStatements(ejCons(N,Rest),I,E) => exRequire(exCheck(notBool E orBool ejGet(N,"constructor") ==K ejString("Ensure"),"TYPE_STATEMENT_PLACEMENT"),ejGet(N,"span"),ejCons(ejString(Int2String(I)),ejNil())) ~> exStaticResult(exInfer(N,S,L,W,ejGet(N,"constructor") ==K ejString("Ensure"),true,ejCons(ejString(Int2String(I)),ejNil())),N,ejCons(ejString(Int2String(I)),ejNil())) ~> exStaticStatements(Rest,I +Int 1,E orBool ejGet(N,"constructor") ==K ejString("Ensure")) ... </k> <sigma> S </sigma> <localTypes> L </localTypes> <written> W </written>
  rule <k> exStaticResult(exTypeError(C,S,P),_,_) => exStop(C,S,P) ... </k>
  rule <k> exStaticResult(exTyped(_),N,P) => exStaticBind(exInfer(ejGet(ejGet(N,"operands"),"value"),S,L,W,false,false,ejAppend(P,ejCons(ejString("0"),ejNil()))),exText(ejGet(ejGet(N,"operands"),"name"))) ... </k> <sigma> S </sigma> <localTypes> L </localTypes> <written> W </written> requires ejGet(N,"constructor") ==K ejString("Let")
  rule <k> exStaticBind(exTyped(T),N) => .K ... </k> <localTypes> L => ejPut(L,N,T) </localTypes>
  rule <k> exStaticBind(exTypeError(C,S,P),_) => exStop(C,S,P) ... </k>
  rule <k> exStaticResult(exTyped(_),N,_) => .K ... </k> <written> W => ejPut(W,exText(ejGet(ejGet(N,"operands"),"field")),ejBool(true)) </written> requires ejGet(N,"constructor") ==K ejString("NextWrite")
  rule <k> exStaticResult(exTyped(_),N,_) => .K ... </k> requires ejGet(N,"constructor") =/=K ejString("NextWrite") andBool ejGet(N,"constructor") =/=K ejString("Let")
  rule <k> exSnapshotPhase() => exRequire(exSnapshot(S,exFieldTypes(ejGet(S,"fields")),ejGet(R,"Pre")),exSynthetic(),ejNil()) ~> exRequire(exSnapshot(S,ejGet(S,"args"),ejGet(R,"Args")),exSynthetic(),ejNil()) ~> exRequire(exSnapshot(S,ejGet(S,"observations"),ejGet(R,"Obs")),exSynthetic(),ejNil()) ~> exBegin() ... </k> <sigma> S </sigma> <request> R </request>
  rule <k> exBegin() => exLaunchCore(ejGet(R,"core")) ... </k> <request> R </request> <pre> _ => ejGet(R,"Pre") </pre> <args> _ => ejGet(R,"Args") </args> <obs> _ => ejGet(R,"Obs") </obs> <localTypes> _ => ejObject(ejNil()) </localTypes> <written> _ => ejObject(ejNil()) </written> <reducing> false => true </reducing>
  syntax KItem ::= exLaunchCore(EJSON)
  rule <k> exLaunchCore(N) => exRunStatements(exUnwrapArray(ejGet(N,"statements")),0) ~> exFinish() ... </k> requires ejHas(N,"statements")
  rule <k> exLaunchCore(N) => exEnter(N,ejNil()) ~> exPublishExpression() ... </k> requires notBool ejHas(N,"statements")
  rule <k> exEnter(N,P) => exStop("WORK_EXHAUSTED",ejGet(N,"span"),P) ... </k> <work> 0 </work>
  rule <k> exEnter(N,P) => exEvaluate(N,P) ... </k> <work> W => W -Int 1 </work> requires W >Int 0
  rule <k> exRunStatements(ejNil(),_) => .K ... </k>
  rule <k> exRunStatements(ejCons(N,L),I) => exEnter(N,ejCons(ejString(Int2String(I)),ejNil())) ~> exDiscard() ~> exRunStatements(L,I +Int 1) ... </k>
  rule <k> exReturned(_,_) ~> exDiscard() => .K ... </k>
  rule <k> exReturned(T,V) ~> exPublishExpression() => .K </k> <work> W </work> <out> exPending() => exValue(T,V,W) </out>
  rule <k> exFinish() => exRequire(exSnapshotBounds(S,exFieldTypes(ejGet(S,"fields")),exOverlay(Pre,Writes),"VALUE_BOUND"),ejGet(ejGet(R,"core"),"span"),ejNil()) ~> exCommitCandidate() ... </k> <sigma> S </sigma> <request> R </request> <pre> Pre </pre> <writes> Writes </writes>
  syntax KItem ::= exCommitCandidate()
  rule <k> exCommitCandidate() => .K </k> <pre> Pre </pre> <writes> Writes </writes> <descriptors> D </descriptors> <work> W </work> <out> exPending() => exPrepared(exOverlay(Pre,Writes),D,W) </out>

  syntax Bool ::= exJValid(EJSON) [function, total]
                | exJListValid(EJList) [function, total]
  rule exJValid(ejString(_)) => true
  rule exJValid(ejBool(_)) => true
  rule exJValid(ejArray(L)) => exJListValid(L)
  rule exJValid(ejObject(L)) => exJListValid(L)
  rule exJValid(ejPair(_,V)) => exJValid(V)
  rule exJValid(_) => false [owise]
  rule exJListValid(ejNil()) => true
  rule exJListValid(ejCons(V,L)) => exJValid(V) andBool exJListValid(L)
  syntax Bool ::= exComponentsBound(EJSON,EJSON) [function, total]
  rule exComponentsBound(S,R) => exJsonBytes(S) <=Int 65536 andBool exJsonBytes(ejGet(R,"core")) <=Int 65536 andBool exJsonBytes(ejGet(R,"Pre")) <=Int 65536 andBool exJsonBytes(ejGet(R,"Args")) <=Int 65536 andBool exJsonBytes(ejGet(R,"Obs")) <=Int 65536
  syntax EJList ::= exRequestKeys() [function]
  rule exRequestKeys() => ejCons(ejString("contract"),ejCons(ejString("source"),ejCons(ejString("core"),ejCons(ejString("Pre"),ejCons(ejString("Args"),ejCons(ejString("Obs"),ejCons(ejString("workInitial"),ejNil())))))))
  syntax EJSON ::= exFieldTypes(EJSON) [function, total]
                 | exOverlay(EJSON,EJSON) [function, total]
  syntax EJList ::= exFieldTypeList(EJList) [function, total]
  rule exFieldTypes(ejObject(L)) => ejObject(exFieldTypeList(L))
  rule exFieldTypes(_) => ejObject(ejNil()) [owise]
  rule exFieldTypeList(ejNil()) => ejNil()
  rule exFieldTypeList(ejCons(ejPair(N,F),L)) => ejCons(ejPair(N,ejGet(F,"type")),exFieldTypeList(L))
  rule exOverlay(P,ejObject(ejNil())) => P
  rule exOverlay(P,ejObject(ejCons(ejPair(N,V),L))) => exOverlay(ejPut(P,N,V),ejObject(L))
  rule exOverlay(P,_) => P [owise]
  // Entry is charged once. Child frames retain original paths and spans.
  rule <k> exEvaluate(N,P) => exEnter(ejGet(ejGet(N,"operands"),"left"),ejAppend(P,ejCons(ejString("0"),ejNil()))) ~> exBooleanLeft(exText(ejGet(N,"constructor")),N,P) ... </k> requires ejGet(N,"constructor") ==K ejString("And") orBool ejGet(N,"constructor") ==K ejString("Or")
  rule <k> exReturned(_,ejBool(false)) ~> exBooleanLeft("And",_,_) => exReturned(exType("Bool"),ejBool(false)) ... </k>
  rule <k> exReturned(_,ejBool(true)) ~> exBooleanLeft("Or",_,_) => exReturned(exType("Bool"),ejBool(true)) ... </k>
  rule <k> exReturned(_,ejBool(true)) ~> exBooleanLeft("And",N,P) => exEnter(ejGet(ejGet(N,"operands"),"right"),ejAppend(P,ejCons(ejString("1"),ejNil()))) ... </k>
  rule <k> exReturned(_,ejBool(false)) ~> exBooleanLeft("Or",N,P) => exEnter(ejGet(ejGet(N,"operands"),"right"),ejAppend(P,ejCons(ejString("1"),ejNil()))) ... </k>
  rule <k> exEvaluate(N,P) => exCollect(N,P,exChildren(exText(ejGet(N,"constructor")),ejGet(N,"operands")),ejNil(),0) ... </k> requires ejGet(N,"constructor") =/=K ejString("And") andBool ejGet(N,"constructor") =/=K ejString("Or")
  rule <k> exCollect(N,P,ejCons(C,R),V,I) => exEnter(C,ejAppend(P,ejCons(ejString(Int2String(I)),ejNil()))) ~> exCollectValue(N,P,R,V,I) ... </k>
  rule <k> exReturned(_,V) ~> exCollectValue(N,P,R,Vs,I) => exCollect(N,P,R,ejAppend(Vs,ejCons(V,ejNil())),I +Int 1) ... </k>
  rule <k> exCollect(N,P,ejNil(),V,_) => exApplyTyped(exInfer(N,S,L,ejObject(ejNil()),true,exStatement(exText(ejGet(N,"constructor"))),P),N,P,ejArray(V)) ... </k> <sigma> S </sigma> <localTypes> L </localTypes>
  syntax KItem ::= exApplyTyped(EXInference,EJSON,EJList,EJSON)
                 | exReturnChecked(EJSON,EJSON,EJSON,EJList)
  rule <k> exApplyTyped(exTyped(T),N,P,V) => exApply(exText(ejGet(N,"constructor")),ejGet(N,"operands"),T,P,ejArray(ejCons(ejGet(N,"span"),ejCons(V,ejNil())))) ... </k>
  rule <k> exApplyTyped(exTypeError(C,Span,P),_,_,_) => exStop(C,Span,P) ... </k>
  // Payload packages span and operand values; it carries no host-generated verdict.
  rule <k> exApply(K,O,T,P,Payload) => exReturnChecked(T,exLiteralValue(K,O),ejAt(Payload,0),P) ... </k> requires exLiteral(K)
  rule <k> exApply("ReadLocal",O,T,P,X) => exReturnChecked(T,exGetName(L,ejGet(O,"name")),ejAt(X,0),P) ... </k> <locals> L </locals>
  rule <k> exApply("ReadArg",O,T,P,X) => exReturnChecked(T,exGetName(A,ejGet(O,"name")),ejAt(X,0),P) ... </k> <args> A </args>
  rule <k> exApply("ReadObs",O,T,P,X) => exReturnChecked(T,exGetName(A,ejGet(O,"name")),ejAt(X,0),P) ... </k> <obs> A </obs>
  rule <k> exApply("ReadPre",O,T,P,X) => exReturnChecked(T,exGetName(Pre,ejGet(O,"field")),ejAt(X,0),P) ... </k> <pre> Pre </pre> requires ejGet(O,"view") ==K ejString("pre")
  rule <k> exApply("ReadPre",O,T,P,X) => exReturnChecked(T,exGetName(exOverlay(Pre,W),ejGet(O,"field")),ejAt(X,0),P) ... </k> <pre> Pre </pre> <writes> W </writes> requires ejGet(O,"view") ==K ejString("post")
  rule <k> exApply(K,O,T,P,X) => exReturnChecked(T,exGetName(ejAt(ejAt(X,1),0),ejGet(O,"field")),ejAt(X,0),P) ... </k> requires K ==String "ProjectField" orBool K ==String "AccessField"
  rule <k> exApply(K,_,T,P,X) => exRequire(exCheck(exInt(ejAt(ejAt(X,1),1)) <Int exArrayLength(ejAt(ejAt(X,1),0)),"INDEX_RANGE"),ejAt(X,0),P) ~> exReturnChecked(T,ejAt(ejAt(ejAt(X,1),0),exInt(ejAt(ejAt(X,1),1))),ejAt(X,0),P) ... </k> requires K ==String "ProjectIndex" orBool K ==String "AccessIndex"
  rule <k> exApply("ConstructRecord",O,T,P,X) => exReturnChecked(T,ejObject(exSortPairs(exZipFields(exUnwrapArray(ejGet(O,"fields")),exUnwrapArray(ejAt(X,1))))),ejAt(X,0),P) ... </k>
  rule <k> exApply("ConstructEnum",O,T,P,X) => exReturnChecked(T,ejGet(O,"member"),ejAt(X,0),P) ... </k>
  rule <k> exApply("ConstructNone",_,T,P,X) => exReturnChecked(T,ejArray(ejNil()),ejAt(X,0),P) ... </k>
  rule <k> exApply("ConstructSome",_,T,P,X) => exReturnChecked(T,ejArray(ejCons(ejAt(ejAt(X,1),0),ejNil())),ejAt(X,0),P) ... </k>
  rule <k> exApply("ConstructCollection",_,T,P,X) => exReturnChecked(T,ejAt(X,1),ejAt(X,0),P) ... </k>
  rule <k> exApply(K,_,T,P,X) => exArithmetic(K,T,exInt(ejAt(ejAt(X,1),0)),exInt(ejAt(ejAt(X,1),1)),ejAt(X,0),P) ... </k> requires K ==String "Add" orBool K ==String "Sub" orBool K ==String "Mul" orBool K ==String "FloorDiv" orBool K ==String "CeilDiv"
  rule <k> exApply("Eq",_,T,P,X) => exReturnChecked(T,ejBool(ejAt(ejAt(X,1),0) ==K ejAt(ejAt(X,1),1)),ejAt(X,0),P) ... </k>
  rule <k> exApply(K,_,T,P,X) => exReturnChecked(T,ejBool(exCompare(K,exInt(ejAt(ejAt(X,1),0)),exInt(ejAt(ejAt(X,1),1)))),ejAt(X,0),P) ... </k> requires K ==String "Lt" orBool K ==String "Lte" orBool K ==String "Gt" orBool K ==String "Gte"
  rule <k> exApply("Not",_,T,P,X) => exReturnChecked(T,ejBool(ejAt(ejAt(X,1),0) ==K ejBool(false)),ejAt(X,0),P) ... </k>
  rule <k> exApply(K,_,T,P,X) => exRequire(exCheck(ejAt(ejAt(X,1),0) ==K ejBool(true),exGuardCode(K)),ejAt(X,0),P) ~> exReturned(T,ejMissing()) ... </k> requires K ==String "Require" orBool K ==String "Ensure"
  rule <k> exApply("Let",O,T,_,X) => exReturned(T,ejMissing()) ... </k> <locals> L => ejPut(L,exText(ejGet(O,"name")),ejAt(ejAt(X,1),0)) </locals> <localTypes> LT => ejPut(LT,exText(ejGet(O,"name")),exInferredType(exInfer(ejGet(O,"value"),S,LT,ejObject(ejNil()),false,false,ejNil()))) </localTypes> <sigma> S </sigma>
  rule <k> exApply("NextWrite",O,T,_,X) => exReturned(T,ejMissing()) ... </k> <writes> W => ejPut(W,exText(ejGet(O,"field")),ejAt(ejAt(X,1),0)) </writes>
  rule <k> exApply("Emit",O,T,P,X) => exEmitCheck(T,ejObject(ejCons(ejPair("fields",ejAt(ejAt(X,1),0)),ejCons(ejPair("operation",ejGet(O,"operation")),ejNil()))),ejAt(X,0),P) ... </k>
  rule <k> exReturnChecked(T,V,Span,P) => exRequire(exValueBound(S,T,V,"VALUE_BOUND"),Span,P) ~> exReturned(T,V) ... </k> <sigma> S </sigma>
  syntax KItem ::= exArithmetic(String,EJSON,Int,Int,EJSON,EJList)
                 | exArithmeticResult(EJSON,Int,EJSON,EJList)
                 | exDivide(String,EJSON,Int,Int,EJSON,EJList)
                 | exEmitCheck(EJSON,EJSON,EJSON,EJList)
                 | exEmitAppend(EJSON,EJSON,EJSON,EJList)
  rule <k> exArithmetic("Add",T,A,B,S,P) => exArithmeticResult(T,A +Int B,S,P) ... </k>
  rule <k> exArithmetic("Sub",T,A,B,S,P) => exArithmeticResult(T,A -Int B,S,P) ... </k>
  rule <k> exArithmetic("Mul",T,A,B,S,P) => exArithmeticResult(T,A *Int B,S,P) ... </k>
  rule <k> exArithmetic(K,T,A,B,S,P) => exRequire(exCheck(B >Int 0,"ARITH_DENOMINATOR"),S,P) ~> exDivide(K,T,A,B,S,P) ... </k> requires K ==String "FloorDiv" orBool K ==String "CeilDiv"
  rule <k> exDivide("FloorDiv",T,A,B,S,P) => exArithmeticResult(T,exFloor(A,B),S,P) ... </k> requires B >Int 0
  rule <k> exDivide("CeilDiv",T,A,B,S,P) => exArithmeticResult(T,0 -Int exFloor(0 -Int A,B),S,P) ... </k> requires B >Int 0
  rule <k> exArithmeticResult(T,N,S,P) => exRequire(exCheck(exFits(T,ejString(Int2String(N))),"ARITH_RANGE"),S,P) ~> exReturnChecked(T,ejString(Int2String(N)),S,P) ... </k>
  syntax Int ::= exFloor(Int,Int) [function]
  rule exFloor(A,B) => A /Int B requires B >Int 0 andBool A >=Int 0
  rule exFloor(A,B) => (0 -Int ((0 -Int A) /Int B)) -Int 1 requires B >Int 0 andBool A <Int 0 andBool (0 -Int A) %Int B =/=Int 0
  rule exFloor(A,B) => 0 -Int ((0 -Int A) /Int B) requires B >Int 0 andBool A <Int 0 andBool (0 -Int A) %Int B ==Int 0
  syntax Bool ::= exCompare(String,Int,Int) [function, total]
  rule exCompare("Lt",A,B) => A <Int B
  rule exCompare("Lte",A,B) => A <=Int B
  rule exCompare("Gt",A,B) => A >Int B
  rule exCompare("Gte",A,B) => A >=Int B
  rule exCompare(_,_,_) => false [owise]
  syntax String ::= exGuardCode(String) [function, total]
  rule exGuardCode("Require") => "GUARD_FAILED"
  rule exGuardCode(_) => "ENSURES_FAILED" [owise]
  syntax EJSON ::= exInferredType(EXInference) [function, total]
  rule exInferredType(exTyped(T)) => T
  rule exInferredType(_) => ejMissing() [owise]
  syntax EJList ::= exZipFields(EJList,EJList) [function, total]
  rule exZipFields(ejNil(),ejNil()) => ejNil()
  rule exZipFields(ejCons(F,L),ejCons(V,R)) => ejCons(ejPair(exText(ejGet(F,"name")),V),exZipFields(L,R))
  rule exZipFields(_,_) => ejNil() [owise]
  rule <k> exEmitCheck(T,D,Span,P) => exRequire(exValueBound(S,exType("Operation",ejGet(D,"operation")),D,"DESCRIPTOR_BOUND"),Span,P) ~> exEmitAppend(T,D,Span,P) ... </k> <sigma> S </sigma>
  rule <k> exEmitAppend(T,D,Span,P) => exRequire(exDescriptorBound(S,ejAppend(Ds,ejCons(D,ejNil()))),Span,P) ~> exReturned(T,ejMissing()) ... </k> <sigma> S </sigma> <descriptors> Ds => ejAppend(Ds,ejCons(D,ejNil())) </descriptors>
  syntax String ::= exDescriptorBound(EJSON,EJList) [function, total]
  syntax EJList ::= exDescriptorChildren(EJList) [function, total]
  rule exDescriptorChildren(ejNil()) => ejNil()
  rule exDescriptorChildren(ejCons(D,L)) => ejCons(ejArray(ejCons(exType("Operation",ejGet(D,"operation")),ejCons(D,ejNil()))),exDescriptorChildren(L))
  rule exDescriptorBound(S,L) => exCheck(ejLength(L) <=Int 128 andBool exJsonBytes(ejArray(L)) <=Int 65536 andBool 1 +Int exChildrenNodes(S,exDescriptorChildren(L)) <=Int 4096 andBool 1 +Int exChildrenDepth(S,exDescriptorChildren(L)) <=Int 64,"DESCRIPTOR_BOUND")
endmodule
