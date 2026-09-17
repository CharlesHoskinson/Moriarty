// Isolated Core/4 extension of the expression definition; legacy files are unchanged.
requires "lifecycle-shape.k"
requires "lifecycle-values.k"
module MORIARTY-LIFECYCLE-EXPRESSION-INFER
  imports MORIARTY-LIFECYCLE-EXPRESSION-SHAPE
  imports MORIARTY-LIFECYCLE-EXPRESSION-VALUES
  syntax LXInference ::= lxTyped(EJSON) | lxTypedList(EJList) | lxTypeError(String,EJSON,EJList)
  syntax LXInference ::= lxInfer(EJSON,EJSON,EJSON,EJSON,Bool,Bool,EJList) [function, total]
                       | lxInferStart(String,EJSON,EJSON,EJSON,EJSON,Bool,Bool,EJSON,EJList) [function, total]
                       | lxInferChecked(String,String,EJSON,EJSON,EJSON,EJSON,Bool,EJSON,EJList) [function, total]
                       | lxInferChildren(EJList,EJSON,EJSON,EJSON,Bool,EJList,Int) [function, total]
                       | lxTypePrepend(LXInference,LXInference) [function, total]
                       | lxInferFinish(LXInference,String,EJSON,EJSON,EJSON,Bool,EJSON,EJList) [function, total]
                       | lxTypeSite(LXInference,EJSON,EJList) [function, total]
  syntax Bool ::= lxStatement(String) [function, total]
  rule lxStatement(K) => K ==String "Require" orBool K ==String "Let" orBool K ==String "NextWrite" orBool K ==String "Ensure" orBool K ==String "Emit"
  rule lxInfer(N,S,L,W,E,A,P) => lxInferName(ejGet(N,"constructor"),N,S,L,W,E,A,P)
  syntax LXInference ::= lxInferName(EJSON,EJSON,EJSON,EJSON,EJSON,Bool,Bool,EJList) [function, total]
  rule lxInferName(ejString(K),N,S,L,W,E,A,P) => lxInferStart(K,ejGet(N,"operands"),S,L,W,E,A,ejGet(N,"span"),P)
  rule lxInferName(_,N,_,_,_,_,_,P) => lxTypeError("TYPE_CONSTRUCTOR",ejGet(N,"span"),P) [owise]
  rule lxInferStart(K,O,S,L,W,E,A,Span,P) => lxInferChecked(lxFirst(lxCheck(lxStatement(K) ==Bool A,"TYPE_STATEMENT_PLACEMENT"),lxPreType(K,O,S,L,W)),K,O,S,L,W,E,Span,P)
  rule lxInferChecked("",K,O,S,L,W,E,Span,P) => lxInferFinish(lxInferChildren(lxChildren(K,O),S,L,W,E,P,0),K,O,S,L,E,Span,P)
  rule lxInferChecked(C,_,_,_,_,_,_,Span,P) => lxTypeError(C,Span,P) requires C =/=String ""
  rule lxInferChildren(ejNil(),_,_,_,_,_,_) => lxTypedList(ejNil())
  rule lxInferChildren(ejCons(N,R),S,L,W,E,P,I) => lxTypePrepend(lxInfer(N,S,L,W,E,false,ejAppend(P,ejCons(ejString(Int2String(I)),ejNil()))),lxInferChildren(R,S,L,W,E,P,I +Int 1))
  rule lxTypePrepend(lxTyped(T),lxTypedList(L)) => lxTypedList(ejCons(T,L))
  rule lxTypePrepend(lxTypeError(C,S,P),_) => lxTypeError(C,S,P)
  rule lxTypePrepend(lxTyped(_),lxTypeError(C,S,P)) => lxTypeError(C,S,P)
  rule lxInferFinish(lxTypeError(C,S,P),_,_,_,_,_,_,_) => lxTypeError(C,S,P)
  rule lxInferFinish(lxTypedList(T),K,O,S,L,E,Span,P) => lxTypeSite(lxPrimitiveType(K,O,S,L,E,ejArray(T)),Span,P)
  rule lxTypeSite(lxTyped(T),_,_) => lxTyped(T)
  rule lxTypeSite(lxTypeError(C,_,_),S,P) => lxTypeError(C,S,P)
  syntax EJList ::= lxChildren(String,EJSON) [function, total]
                  | lxRoleChildren(LXRoles,EJSON) [function, total]
                  | lxUnwrapArray(EJSON) [function, total]
                  | lxFieldExpressions(EJList) [function, total]
  rule lxChildren(K,O) => lxRoleChildren(lxRoles(K),O)
  rule lxRoleChildren(lxRolesNil(),_) => ejNil()
  rule lxRoleChildren(lxRolesCons(lxRole(N,"expr"),R),O) => ejCons(ejGet(O,N),lxRoleChildren(R,O))
  rule lxRoleChildren(lxRolesCons(lxRole(N,"list"),R),O) => ejAppend(lxUnwrapArray(ejGet(O,N)),lxRoleChildren(R,O))
  rule lxRoleChildren(lxRolesCons(lxRole(N,"fields"),R),O) => ejAppend(lxFieldExpressions(lxUnwrapArray(ejGet(O,N))),lxRoleChildren(R,O))
  rule lxRoleChildren(lxRolesCons(lxRole(_,K),R),O) => lxRoleChildren(R,O) requires K =/=String "expr" andBool K =/=String "list" andBool K =/=String "fields"
  rule lxUnwrapArray(ejArray(L)) => L
  rule lxUnwrapArray(_) => ejNil() [owise]
  rule lxFieldExpressions(ejNil()) => ejNil()
  rule lxFieldExpressions(ejCons(F,L)) => ejCons(ejGet(F,"value"),lxFieldExpressions(L))
  syntax String ::= lxPreType(String,EJSON,EJSON,EJSON,EJSON) [function, total]
                  | lxBinder(EJSON,EJSON,EJSON) [function, total]
                  | lxBinderMaps(EJSON,EJSON,EJList) [function, total]
                  | lxRecordNames(EJSON,EJList,EJList) [function, total]
                  | lxRecordCoverage(EJSON,EJList) [function, total]
  rule lxPreType("Let",O,S,L,_) => lxBinder(ejGet(O,"name"),S,L)
  rule lxPreType("NextWrite",O,S,_,W) => lxWritePre(ejGet(O,"field"),S,W)
  syntax String ::= lxWritePre(EJSON,EJSON,EJSON) [function, total]
  rule lxWritePre(ejString(F),S,W) => lxFirst(lxCheck(ejHas(ejGet(S,"fields"),F),"TYPE_NAME"),lxFirst(lxCheck(ejGet(ejGet(ejGet(S,"fields"),F),"writeClass") ==K ejString("ordinary"),"TYPE_FINANCIAL_WRITE"),lxCheck(notBool ejHas(W,F),"TYPE_DUPLICATE_WRITE")))
  rule lxWritePre(_,_,_) => "INPUT_SCHEMA" [owise]
  rule lxPreType("Emit",O,S,_,_) => lxCheck(lxHasName(ejGet(S,"operations"),ejGet(O,"operation")),"TYPE_NAME")
  rule lxPreType("ConstructRecord",O,S,_,_) => lxFirst(lxCheck(lxHasName(ejGet(S,"recordTypes"),ejGet(O,"recordType")),"TYPE_NAME"),lxFirst(lxRecordNames(O,lxUnwrapArray(ejGet(O,"fields")),ejNil()),lxRecordCoverage(lxGetName(ejGet(S,"recordTypes"),ejGet(O,"recordType")),lxUnwrapArray(ejGet(O,"fields")))))
  rule lxPreType(K,O,S,_,_) => lxResolve(S,ejGet(O,"elementType"),"TYPE_LITERAL") requires K ==String "ConstructSome" orBool K ==String "ConstructNone"
  rule lxPreType("ConstructCollection",O,S,_,_) => lxFirst(lxResolve(S,ejGet(O,"elementType"),"TYPE_LITERAL"),lxCheck(lxInt(ejGet(O,"capacity")) >=Int 0 andBool lxInt(ejGet(O,"capacity")) <=Int 128 andBool lxArrayLength(ejGet(O,"items")) <=Int lxInt(ejGet(O,"capacity")),"TYPE_COLLECTION_BOUND"))
  rule lxPreType(_,_,_,_,_) => "" [owise]
  rule lxBinder(ejString(N),S,L) => lxFirst(lxCheck(notBool ejHas(L,N) andBool N =/=String "pre" andBool N =/=String "post" andBool N =/=String "next","TYPE_DUPLICATE_BINDER"),lxBinderMaps(ejString(N),S,lxSchemaKeys()))
  rule lxBinder(_,_,_) => "INPUT_SCHEMA" [owise]
  rule lxBinderMaps(_,_,ejNil()) => ""
  rule lxBinderMaps(N,S,ejCons(ejString(K),R)) => lxFirst(lxCheck(notBool lxHasName(ejGet(S,K),N) andBool notBool lxIn(N,ejGet(S,K)),"TYPE_DUPLICATE_BINDER"),lxBinderMaps(N,S,R))
  rule lxRecordNames(_,ejNil(),_) => ""
  rule lxRecordNames(O,ejCons(F,R),Seen) => lxFirst(lxCheck(notBool ejMember(ejGet(F,"name"),Seen),"TYPE_DUPLICATE_FIELD"),lxRecordNames(O,R,ejCons(ejGet(F,"name"),Seen)))
  rule lxRecordCoverage(T,F) => lxFirst(lxCheck(lxSize(T) ==Int ejLength(F),"TYPE_RECORD_FIELDS"),lxRecordFieldNames(T,F))
  syntax String ::= lxRecordFieldNames(EJSON,EJList) [function, total]
  rule lxRecordFieldNames(_,ejNil()) => ""
  rule lxRecordFieldNames(T,ejCons(F,R)) => lxFirst(lxCheck(lxHasName(T,ejGet(F,"name")),"TYPE_RECORD_FIELDS"),lxRecordFieldNames(T,R))
  syntax LXInference ::= lxPrimitiveType(String,EJSON,EJSON,EJSON,Bool,EJSON) [function, total]
                       | lxTypeIf(String,EJSON) [function, total]
                       | lxLookupType(EJSON,EJSON) [function, total]
  rule lxTypeIf("",T) => lxTyped(T)
  rule lxTypeIf(C,_) => lxTypeError(C,lxSynthetic(),ejNil()) requires C =/=String ""
  rule lxLookupType(M,N) => lxTypeIf(lxCheck(lxHasName(M,N),"TYPE_NAME"),lxGetName(M,N))
  rule lxPrimitiveType("ReadLocal",O,_,L,_,_) => lxLookupType(L,ejGet(O,"name"))
  rule lxPrimitiveType("ReadArg",O,S,_,_,_) => lxLookupType(ejGet(S,"args"),ejGet(O,"name"))
  rule lxPrimitiveType("ReadObs",O,S,_,_,_) => lxLookupType(ejGet(S,"observations"),ejGet(O,"name"))
  rule lxPrimitiveType("ReadPre",O,S,_,E,_) => lxTypeIf(lxFirst(lxCheck(ejGet(O,"view") =/=K ejString("next"),"TYPE_NEXT_READ"),lxFirst(lxCheck(ejGet(O,"view") =/=K ejString("post") orBool E,"TYPE_POST_SCOPE"),lxCheck(lxHasName(ejGet(S,"fields"),ejGet(O,"field")),"TYPE_NAME"))),ejGet(lxGetName(ejGet(S,"fields"),ejGet(O,"field")),"type"))
  rule lxPrimitiveType(K,O,S,_,_,T) => lxTypeIf(lxFirst(lxCheck(ejAt(ejAt(T,0),0) ==K ejString("Record"),"TYPE_MISMATCH"),lxCheck(lxHasName(lxGetName(ejGet(S,"recordTypes"),ejAt(ejAt(T,0),1)),ejGet(O,"field")),"TYPE_NAME")),lxGetName(lxGetName(ejGet(S,"recordTypes"),ejAt(ejAt(T,0),1)),ejGet(O,"field"))) requires K ==String "ProjectField" orBool K ==String "AccessField"
  rule lxPrimitiveType(K,_,_,_,_,T) => lxTypeIf(lxCheck(ejAt(ejAt(T,0),0) ==K ejString("Collection") andBool ejAt(T,1) ==K lxType("UInt64"),"TYPE_MISMATCH"),ejAt(ejAt(T,0),1)) requires K ==String "ProjectIndex" orBool K ==String "AccessIndex"
  rule lxPrimitiveType("ConstructEnum",O,S,_,_,_) => lxTypeIf(lxFirst(lxCheck(lxHasName(ejGet(S,"enumTypes"),ejGet(O,"enumType")),"TYPE_NAME"),lxCheck(lxIn(ejGet(O,"member"),lxGetName(ejGet(S,"enumTypes"),ejGet(O,"enumType"))),"TYPE_ENUM_MEMBER")),lxType("Enum",ejGet(O,"enumType")))
  rule lxPrimitiveType("ConstructNone",O,_,_,_,_) => lxTyped(lxType("Option",ejGet(O,"elementType")))
  rule lxPrimitiveType("ConstructSome",O,_,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K ejGet(O,"elementType"),"TYPE_MISMATCH"),lxType("Option",ejGet(O,"elementType")))
  rule lxPrimitiveType("ConstructCollection",O,_,_,_,T) => lxTypeIf(lxAllTypes(lxUnwrapArray(T),ejGet(O,"elementType")),lxType("Collection",ejGet(O,"elementType"),ejGet(O,"capacity")))
  rule lxPrimitiveType("ConstructRecord",O,S,_,_,T) => lxTypeIf(lxRecordTypeFields(lxGetName(ejGet(S,"recordTypes"),ejGet(O,"recordType")),lxUnwrapArray(ejGet(O,"fields")),lxUnwrapArray(T)),lxType("Record",ejGet(O,"recordType")))
  rule lxPrimitiveType(K,_,_,_,_,T) => lxTypeIf(lxAllTypes(lxUnwrapArray(T),lxType("Bool")),lxType("Bool")) requires K ==String "Not" orBool K ==String "And" orBool K ==String "Or"
  rule lxPrimitiveType(K,_,_,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K ejAt(T,1) andBool (K ==String "Eq" orBool lxNumeric(ejAt(T,0))),"TYPE_MISMATCH"),lxType("Bool")) requires K ==String "Eq" orBool K ==String "Lt" orBool K ==String "Lte" orBool K ==String "Gt" orBool K ==String "Gte"
  rule lxPrimitiveType(K,_,_,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K lxType("Bool"),"TYPE_MISMATCH"),lxType("Unit")) requires K ==String "Require" orBool K ==String "Ensure"
  rule lxPrimitiveType("Let",_,_,_,_,_) => lxTyped(lxType("Unit"))
  rule lxPrimitiveType("NextWrite",O,S,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K ejGet(lxGetName(ejGet(S,"fields"),ejGet(O,"field")),"type"),"TYPE_MISMATCH"),lxType("Unit"))
  rule lxPrimitiveType("Emit",O,S,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K lxType("Record",lxGetName(ejGet(S,"operations"),ejGet(O,"operation"))),"TYPE_MISMATCH"),lxType("Unit"))
  syntax String ::= lxAllTypes(EJList,EJSON) [function, total]
                  | lxRecordTypeFields(EJSON,EJList,EJList) [function, total]
  rule lxAllTypes(ejNil(),_) => ""
  rule lxAllTypes(ejCons(T,L),Expected) => lxFirst(lxCheck(T ==K Expected,"TYPE_MISMATCH"),lxAllTypes(L,Expected))
  rule lxRecordTypeFields(_,ejNil(),ejNil()) => ""
  rule lxRecordTypeFields(S,ejCons(F,L),ejCons(T,R)) => lxFirst(lxCheck(T ==K lxGetName(S,ejGet(F,"name")),"TYPE_MISMATCH"),lxRecordTypeFields(S,L,R))
  rule lxRecordTypeFields(_,_,_) => "TYPE_MISMATCH" [owise]
  syntax Bool ::= lxLiteral(String) [function, total]
  rule lxLiteral(K) => K ==String "LitUInt" orBool K ==String "LitSInt" orBool K ==String "LitBool" orBool K ==String "LitText" orBool K ==String "LitAmount" orBool K ==String "LitQuantity" orBool K ==String "LitShares" orBool K ==String "LitRate" orBool K ==String "LitPrice"
  syntax EJSON ::= lxLiteralType(String,EJSON) [function, total]
                 | lxLiteralValue(String,EJSON) [function, total]
  rule lxLiteralType("LitUInt",O) => lxType("UInt" +String lxText(ejGet(O,"width")))
  rule lxLiteralType("LitSInt",_) => lxType("SInt128")
  rule lxLiteralType("LitBool",_) => lxType("Bool")
  rule lxLiteralType("LitText",_) => lxType("Text")
  rule lxLiteralType("LitAmount",O) => lxType("Amount",ejGet(O,"asset"))
  rule lxLiteralType("LitQuantity",O) => lxType("Quantity",ejGet(O,"units"),ejGet(O,"scale"))
  rule lxLiteralType("LitShares",O) => lxType("Shares",ejGet(O,"vault"),ejGet(O,"holder"))
  rule lxLiteralType("LitRate",O) => lxType("Rate",ejGet(O,"scale"))
  rule lxLiteralType("LitPrice",O) => lxType("Price",ejGet(O,"base"),ejGet(O,"quote"),ejGet(O,"scale"))
  rule lxLiteralType(_,_) => ejMissing() [owise]
  rule lxLiteralValue(K,O) => ejGet(O,"mantissa") requires K ==String "LitQuantity" orBool K ==String "LitRate" orBool K ==String "LitPrice"
  rule lxLiteralValue(_,O) => ejGet(O,"value") [owise]
  rule lxPrimitiveType(K,O,S,_,_,_) => lxTypeIf(lxFirst(lxCheck(K =/=String "LitUInt" orBool ejGet(O,"width") ==K ejString("256") orBool ejGet(O,"width") ==K ejString("64") orBool ejGet(O,"width") ==K ejString("128"),"TYPE_LITERAL"),lxFirst(lxResolve(S,lxLiteralType(K,O),"TYPE_LITERAL"),lxFirst(lxValueDomain(S,lxLiteralType(K,O),lxLiteralValue(K,O),"TYPE_LITERAL"),lxValueBound(S,lxLiteralType(K,O),lxLiteralValue(K,O),"TYPE_LITERAL")))),lxLiteralType(K,O)) requires lxLiteral(K)
  rule lxPrimitiveType(K,O,S,_,_,T) => lcArithmeticType(K,ejAt(T,0),ejAt(T,1),ejGet(O,"right"),S) requires K ==String "Add" orBool K ==String "Sub" orBool K ==String "Mul" orBool K ==String "FloorDiv" orBool K ==String "CeilDiv"
  syntax LXInference ::= lxArithmeticType(String,EJSON,EJSON,EJSON) [function, total]
  rule lxArithmeticType(K,L,R,S) => lxQuantityType(K,L,R,S) requires (K ==String "Mul" orBool K ==String "FloorDiv" orBool K ==String "CeilDiv") andBool ejAt(L,0) ==K ejString("Quantity") andBool ejAt(R,0) ==K ejString("Quantity")
  rule lxArithmeticType(K,L,R,_) => lxTypeIf(lxCheck(L ==K R andBool (L ==K lxType("UInt256") orBool L ==K lxType("UInt64") orBool L ==K lxType("UInt128") orBool L ==K lxType("SInt128") orBool ((K ==String "Add" orBool K ==String "Sub") andBool (ejAt(L,0) ==K ejString("Amount") orBool ejAt(L,0) ==K ejString("Shares") orBool ejAt(L,0) ==K ejString("Rate") orBool ejAt(L,0) ==K ejString("Quantity")))),"TYPE_MISMATCH"),L) [owise]
  syntax LXInference ::= lxQuantityType(String,EJSON,EJSON,EJSON) [function, total]
                       | lxQuantityResult(EJSON,EJSON) [function, total]
  syntax Int ::= lxDirection(String) [function, total]
  rule lxDirection("Mul") => 1
  rule lxDirection(_) => -1 [owise]
  rule lxQuantityType(K,L,R,S) => lxQuantityResult(S,lxType("Quantity",ejArray(lxMergeUnits(lxUnwrapArray(ejAt(L,1)),lxUnwrapArray(ejAt(R,1)),lxDirection(K))),ejString(Int2String(lxInt(ejAt(L,2)) +Int lxDirection(K) *Int lxInt(ejAt(R,2))))))
  rule lxQuantityResult(S,T) => lxTypeIf(lxResolve(S,T,"TYPE_QUANTITY_DOMAIN"),T)
  syntax EJList ::= lxMergeUnits(EJList,EJList,Int) [function, total]
                  | lxSignedUnits(EJList,Int) [function, total]
                  | lxUnitCons(String,Int,EJList) [function, total]
  rule lxUnitCons(_,0,L) => L
  rule lxUnitCons(N,P,L) => ejCons(ejArray(ejCons(ejString(N),ejCons(ejString(Int2String(P)),ejNil()))),L) requires P =/=Int 0
  rule lxSignedUnits(ejNil(),_) => ejNil()
  rule lxSignedUnits(ejCons(ejArray(ejCons(ejString(N),ejCons(P,ejNil()))),L),D) => lxUnitCons(N,D *Int lxInt(P),lxSignedUnits(L,D))
  rule lxMergeUnits(L,ejNil(),_) => L
  rule lxMergeUnits(ejNil(),R,D) => lxSignedUnits(R,D) requires R =/=K ejNil()
  rule lxMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),ejCons(ejArray(ejCons(ejString(B),ejCons(Q,ejNil()))),R),D) => lxUnitCons(A,lxInt(P),lxMergeUnits(L,ejCons(ejArray(ejCons(ejString(B),ejCons(Q,ejNil()))),R),D)) requires A <String B
  rule lxMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),ejCons(ejArray(ejCons(ejString(B),ejCons(Q,ejNil()))),R),D) => lxUnitCons(B,D *Int lxInt(Q),lxMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),R,D)) requires A >String B
  rule lxMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),ejCons(ejArray(ejCons(ejString(A),ejCons(Q,ejNil()))),R),D) => lxUnitCons(A,lxInt(P) +Int D *Int lxInt(Q),lxMergeUnits(L,R,D))
  rule lxMergeUnits(_,_,_) => ejNil() [owise]
  rule lxPrimitiveType(_,_,_,_,_,_) => lxTypeError("TYPE_CONSTRUCTOR",lxSynthetic(),ejNil()) [owise]
  rule lxPrimitiveType("ReadOutstanding",O,S,_,E,T) => lxTypeIf(lxFirst("",lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"unit"),ejGet(S,"units")),"TYPE_NAME"))),lcQuantity(ejGet(O,"unit")))
  rule lxPrimitiveType("ReadPrincipal",O,S,_,E,T) => lxTypeIf(lxFirst("",lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"unit"),ejGet(S,"units")),"TYPE_NAME"))),lcQuantity(ejGet(O,"unit")))
  rule lxPrimitiveType("ReadAccrued",O,S,_,E,T) => lxTypeIf(lxFirst("",lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"unit"),ejGet(S,"units")),"TYPE_NAME"))),lcQuantity(ejGet(O,"unit")))
  rule lxPrimitiveType("ReadBalance",O,S,_,E,T) => lxTypeIf(lxFirst("",lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"asset"),ejGet(S,"assets")),"TYPE_NAME"))),lxType("Amount",ejGet(O,"asset")))
  rule lxPrimitiveType("ReadAllowanceRemaining",O,S,_,E,T) => lxTypeIf(lxFirst("",lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"asset"),ejGet(S,"assets")),"TYPE_NAME"))),lxType("Amount",ejGet(O,"asset")))
  rule lxPrimitiveType("ReadAllowanceSpent",O,S,_,E,T) => lxTypeIf(lxFirst("",lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"asset"),ejGet(S,"assets")),"TYPE_NAME"))),lxType("Amount",ejGet(O,"asset")))
  rule lxPrimitiveType("ReadPostOutstanding",O,S,_,E,T) => lxTypeIf(lxFirst(lxCheck(E,"TYPE_POST_SCOPE"),lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"unit"),ejGet(S,"units")),"TYPE_NAME"))),lcQuantity(ejGet(O,"unit")))
  rule lxPrimitiveType("ReadPostPrincipal",O,S,_,E,T) => lxTypeIf(lxFirst(lxCheck(E,"TYPE_POST_SCOPE"),lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"unit"),ejGet(S,"units")),"TYPE_NAME"))),lcQuantity(ejGet(O,"unit")))
  rule lxPrimitiveType("ReadPostAccrued",O,S,_,E,T) => lxTypeIf(lxFirst(lxCheck(E,"TYPE_POST_SCOPE"),lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"unit"),ejGet(S,"units")),"TYPE_NAME"))),lcQuantity(ejGet(O,"unit")))
  rule lxPrimitiveType("ReadPostBalance",O,S,_,E,T) => lxTypeIf(lxFirst(lxCheck(E,"TYPE_POST_SCOPE"),lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"asset"),ejGet(S,"assets")),"TYPE_NAME"))),lxType("Amount",ejGet(O,"asset")))
  rule lxPrimitiveType("ReadPostAllowanceRemaining",O,S,_,E,T) => lxTypeIf(lxFirst(lxCheck(E,"TYPE_POST_SCOPE"),lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"asset"),ejGet(S,"assets")),"TYPE_NAME"))),lxType("Amount",ejGet(O,"asset")))
  rule lxPrimitiveType("ReadPostAllowanceSpent",O,S,_,E,T) => lxTypeIf(lxFirst(lxCheck(E,"TYPE_POST_SCOPE"),lxFirst(lxCheck(ejAt(T,0) ==K lxType("Text"),"TYPE_MISMATCH"),lxCheck(lxIn(ejGet(O,"asset"),ejGet(S,"assets")),"TYPE_NAME"))),lxType("Amount",ejGet(O,"asset")))
  syntax EJSON ::= lcQuantity(EJSON) [function]
  rule lcQuantity(U) => lxType("Quantity",ejArray(ejCons(ejArray(ejCons(U,ejCons(ejString("1"),ejNil()))),ejNil())),ejString("0"))
  rule lxPreType("ConstructAmount",O,S,_,_) => lxResolve(S,lxType("Amount",ejGet(O,"asset")),"TYPE_LITERAL")
  rule lxPreType("ConvertUInt",O,_,_,_) => lxCheck(ejGet(O,"width") ==K ejString("64") orBool ejGet(O,"width") ==K ejString("128") orBool ejGet(O,"width") ==K ejString("256"),"TYPE_LITERAL")
  rule lxPreType("ScalarValue",O,_,_,_) => lxCheck(ejGet(O,"component") ==K ejString("quanta") orBool ejGet(O,"component") ==K ejString("mantissa") orBool ejGet(O,"component") ==K ejString("negative") orBool ejGet(O,"component") ==K ejString("magnitude"),"TYPE_LITERAL")
  rule lxPrimitiveType("ConstructAmount",O,_,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K lxType("UInt128"),"TYPE_MISMATCH"),lxType("Amount",ejGet(O,"asset")))
  rule lxPrimitiveType("ConvertUInt",O,_,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K lxType("UInt64") orBool ejAt(T,0) ==K lxType("UInt128") orBool ejAt(T,0) ==K lxType("UInt256"),"TYPE_MISMATCH"),lxType("UInt" +String lxText(ejGet(O,"width"))))
  rule lxPrimitiveType("ProjectSome",_,_,_,_,T) => lxTypeIf(lxCheck(ejAt(ejAt(T,0),0) ==K ejString("Option"),"TYPE_MISMATCH"),ejAt(ejAt(T,0),1))
  rule lxPrimitiveType("Select",_,_,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K lxType("Bool") andBool ejAt(T,1) ==K ejAt(T,2) andBool ejAt(ejAt(T,1),0) =/=K ejString("Unit") andBool ejAt(ejAt(T,1),0) =/=K ejString("Operation"),"TYPE_MISMATCH"),ejAt(T,1))
  rule lxPrimitiveType("ScalarValue",O,_,_,_,T) => lcScalarType(lxText(ejGet(O,"component")),lxText(ejAt(ejAt(T,0),0)))
  syntax LXInference ::= lcScalarType(String,String) [function, total]
  rule lcScalarType("quanta",K) => lxTyped(lxType("UInt128")) requires K ==String "Amount" orBool K ==String "Shares"
  rule lcScalarType("mantissa","Price") => lxTyped(lxType("UInt128"))
  rule lcScalarType("mantissa",K) => lxTyped(lxType("SInt128")) requires K ==String "Rate" orBool K ==String "Quantity"
  rule lcScalarType("negative",K) => lxTyped(lxType("Bool")) requires K ==String "Rate" orBool K ==String "Quantity"
  rule lcScalarType("magnitude",K) => lxTyped(lxType("UInt128")) requires K ==String "Rate" orBool K ==String "Quantity"
  rule lcScalarType(_,_) => lxTypeError("TYPE_MISMATCH",lxSynthetic(),ejNil()) [owise]

  rule lxPreType("ConstructShares",O,S,_,_) => lxResolve(S,lxType("Shares",ejGet(O,"vault"),ejGet(O,"holder")),"TYPE_LITERAL")
  rule lxPreType("ConstructVariant",O,S,_,_) => lxFirst(lxCheck(lxHasName(ejGet(S,"variantTypes"),ejGet(O,"family")),"TYPE_NAME"),lxCheck(lxHasName(lxGetName(ejGet(S,"variantTypes"),ejGet(O,"family")),ejGet(O,"tag")),"TYPE_NAME"))
  rule lxPrimitiveType("ConstructShares",O,_,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K lxType("UInt128"),"TYPE_MISMATCH"),lxType("Shares",ejGet(O,"vault"),ejGet(O,"holder")))
  rule lxPrimitiveType("ConstructVariant",O,S,_,_,T) => lxTypeIf(lxCheck(ejAt(T,0) ==K lxGetName(lxGetName(ejGet(S,"variantTypes"),ejGet(O,"family")),ejGet(O,"tag")),"TYPE_MISMATCH"),lxType("Variant",ejGet(O,"family")))
  rule lxPrimitiveType("ProjectVariant",O,S,_,_,T) => lxTypeIf(lxFirst(lxCheck(ejAt(ejAt(T,0),0) ==K ejString("Variant"),"TYPE_MISMATCH"),lxCheck(lxHasName(lxGetName(ejGet(S,"variantTypes"),ejAt(ejAt(T,0),1)),ejGet(O,"tag")),"TYPE_NAME")),lxGetName(lxGetName(ejGet(S,"variantTypes"),ejAt(ejAt(T,0),1)),ejGet(O,"tag")))
  syntax LXInference ::= lcArithmeticType(String,EJSON,EJSON,EJSON,EJSON) [function, total]
  rule lcArithmeticType("Mul",L,R,_,_) => lxTyped(L) requires ejAt(L,0) ==K ejString("Amount") andBool R ==K lxType("UInt128")
  rule lcArithmeticType("Mul",L,R,_,_) => lxTyped(R) requires ejAt(R,0) ==K ejString("Amount") andBool L ==K lxType("UInt128")
  rule lcArithmeticType("Mul",L,R,_,_) => lxTyped(lcAmountProduct(ejAt(L,1),ejAt(R,1))) requires ejAt(L,0) ==K ejString("Amount") andBool ejAt(R,0) ==K ejString("Amount")
  rule lcArithmeticType("Mul",L,R,_,_) => lxTyped(lxType("ScaledAmount",ejAt(R,1),ejAt(R,3))) requires ejAt(L,0) ==K ejString("Amount") andBool ejAt(R,0) ==K ejString("Price") andBool ejAt(R,2) ==K ejAt(L,1)
  rule lcArithmeticType("Mul",L,R,_,_) => lxTyped(lxType("ScaledAmount",ejAt(L,1),ejAt(L,3))) requires ejAt(R,0) ==K ejString("Amount") andBool ejAt(L,0) ==K ejString("Price") andBool ejAt(L,2) ==K ejAt(R,1)
  rule lcArithmeticType("Mul",L,R,_,_) => lxTyped(lxType("SignedScaledAmount",ejAt(L,1),ejAt(R,1))) requires ejAt(L,0) ==K ejString("Amount") andBool ejAt(R,0) ==K ejString("Rate")
  rule lcArithmeticType("Mul",L,R,_,_) => lxTyped(lxType("SignedScaledAmount",ejAt(R,1),ejAt(L,1))) requires ejAt(R,0) ==K ejString("Amount") andBool ejAt(L,0) ==K ejString("Rate")
  rule lcArithmeticType(K,L,R,_,_) => lxTyped(lxType("Amount",lcOtherAsset(L,ejAt(R,1)))) requires (K ==String "FloorDiv" orBool K ==String "CeilDiv") andBool ejAt(L,0) ==K ejString("AmountProduct") andBool ejAt(R,0) ==K ejString("Amount") andBool (ejAt(R,1) ==K ejAt(L,1) orBool ejAt(R,1) ==K ejAt(L,2))
  rule lcArithmeticType(K,L,R,N,_) => lxTypeIf(lxCheck(R ==K lxType("UInt128") andBool ejGet(N,"constructor") ==K ejString("LitUInt") andBool ejGet(ejGet(N,"operands"),"width") ==K ejString("128") andBool lxInt(ejGet(ejGet(N,"operands"),"value")) ==Int (10 ^Int lxInt(ejAt(L,2))),"TYPE_SCALE_DIVISOR"),lxType(lcUnscaled(ejAt(L,0)),ejAt(L,1))) requires (K ==String "FloorDiv" orBool K ==String "CeilDiv") andBool (ejAt(L,0) ==K ejString("ScaledAmount") orBool ejAt(L,0) ==K ejString("SignedScaledAmount"))
  rule lcArithmeticType(K,L,R,_,S) => lxArithmeticType(K,L,R,S) [owise]
  syntax EJSON ::= lcAmountProduct(EJSON,EJSON) [function, total] | lcOtherAsset(EJSON,EJSON) [function, total]
  rule lcAmountProduct(A,B) => lxType("AmountProduct",A,B) requires lxText(A) <=String lxText(B)
  rule lcAmountProduct(A,B) => lxType("AmountProduct",B,A) requires lxText(A) >String lxText(B)
  rule lcOtherAsset(T,A) => ejAt(T,2) requires ejAt(T,1) ==K A
  rule lcOtherAsset(T,A) => ejAt(T,1) requires ejAt(T,1) =/=K A
  syntax String ::= lcUnscaled(EJSON) [function, total]
  rule lcUnscaled(ejString("ScaledAmount")) => "Amount"
  rule lcUnscaled(_) => "SignedAmount" [owise]
  rule lcScalarType("negative",K) => lxTyped(lxType("Bool")) requires K ==String "SignedAmount" orBool K ==String "NetAmount"
  rule lcScalarType("magnitude","NetAmount") => lxTyped(lxType("UInt128"))
  rule lcScalarType("magnitude","SignedAmount") => lxTyped(lxType("UInt256"))
endmodule
