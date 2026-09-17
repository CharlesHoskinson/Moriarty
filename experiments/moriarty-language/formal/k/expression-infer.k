requires "expression-shape.k"
requires "expression-values.k"
module MORIARTY-EXPRESSION-INFER
  imports MORIARTY-EXPRESSION-SHAPE
  imports MORIARTY-EXPRESSION-VALUES
  syntax EXInference ::= exTyped(EJSON) | exTypedList(EJList) | exTypeError(String,EJSON,EJList)
  syntax EXInference ::= exInfer(EJSON,EJSON,EJSON,EJSON,Bool,Bool,EJList) [function, total]
                       | exInferStart(String,EJSON,EJSON,EJSON,EJSON,Bool,Bool,EJSON,EJList) [function, total]
                       | exInferChecked(String,String,EJSON,EJSON,EJSON,EJSON,Bool,EJSON,EJList) [function, total]
                       | exInferChildren(EJList,EJSON,EJSON,EJSON,Bool,EJList,Int) [function, total]
                       | exTypePrepend(EXInference,EXInference) [function, total]
                       | exInferFinish(EXInference,String,EJSON,EJSON,EJSON,Bool,EJSON,EJList) [function, total]
                       | exTypeSite(EXInference,EJSON,EJList) [function, total]
  syntax Bool ::= exStatement(String) [function, total]
  rule exStatement(K) => K ==String "Require" orBool K ==String "Let" orBool K ==String "NextWrite" orBool K ==String "Ensure" orBool K ==String "Emit"
  rule exInfer(N,S,L,W,E,A,P) => exInferName(ejGet(N,"constructor"),N,S,L,W,E,A,P)
  syntax EXInference ::= exInferName(EJSON,EJSON,EJSON,EJSON,EJSON,Bool,Bool,EJList) [function, total]
  rule exInferName(ejString(K),N,S,L,W,E,A,P) => exInferStart(K,ejGet(N,"operands"),S,L,W,E,A,ejGet(N,"span"),P)
  rule exInferName(_,N,_,_,_,_,_,P) => exTypeError("TYPE_CONSTRUCTOR",ejGet(N,"span"),P) [owise]
  rule exInferStart(K,O,S,L,W,E,A,Span,P) => exInferChecked(exFirst(exCheck(exStatement(K) ==Bool A,"TYPE_STATEMENT_PLACEMENT"),exPreType(K,O,S,L,W)),K,O,S,L,W,E,Span,P)
  rule exInferChecked("",K,O,S,L,W,E,Span,P) => exInferFinish(exInferChildren(exChildren(K,O),S,L,W,E,P,0),K,O,S,L,E,Span,P)
  rule exInferChecked(C,_,_,_,_,_,_,Span,P) => exTypeError(C,Span,P) requires C =/=String ""
  rule exInferChildren(ejNil(),_,_,_,_,_,_) => exTypedList(ejNil())
  rule exInferChildren(ejCons(N,R),S,L,W,E,P,I) => exTypePrepend(exInfer(N,S,L,W,E,false,ejAppend(P,ejCons(ejString(Int2String(I)),ejNil()))),exInferChildren(R,S,L,W,E,P,I +Int 1))
  rule exTypePrepend(exTyped(T),exTypedList(L)) => exTypedList(ejCons(T,L))
  rule exTypePrepend(exTypeError(C,S,P),_) => exTypeError(C,S,P)
  rule exTypePrepend(exTyped(_),exTypeError(C,S,P)) => exTypeError(C,S,P)
  rule exInferFinish(exTypeError(C,S,P),_,_,_,_,_,_,_) => exTypeError(C,S,P)
  rule exInferFinish(exTypedList(T),K,O,S,L,E,Span,P) => exTypeSite(exPrimitiveType(K,O,S,L,E,ejArray(T)),Span,P)
  rule exTypeSite(exTyped(T),_,_) => exTyped(T)
  rule exTypeSite(exTypeError(C,_,_),S,P) => exTypeError(C,S,P)
  syntax EJList ::= exChildren(String,EJSON) [function, total]
                  | exRoleChildren(EXRoles,EJSON) [function, total]
                  | exUnwrapArray(EJSON) [function, total]
                  | exFieldExpressions(EJList) [function, total]
  rule exChildren(K,O) => exRoleChildren(exRoles(K),O)
  rule exRoleChildren(exRolesNil(),_) => ejNil()
  rule exRoleChildren(exRolesCons(exRole(N,"expr"),R),O) => ejCons(ejGet(O,N),exRoleChildren(R,O))
  rule exRoleChildren(exRolesCons(exRole(N,"list"),R),O) => ejAppend(exUnwrapArray(ejGet(O,N)),exRoleChildren(R,O))
  rule exRoleChildren(exRolesCons(exRole(N,"fields"),R),O) => ejAppend(exFieldExpressions(exUnwrapArray(ejGet(O,N))),exRoleChildren(R,O))
  rule exRoleChildren(exRolesCons(exRole(_,K),R),O) => exRoleChildren(R,O) requires K =/=String "expr" andBool K =/=String "list" andBool K =/=String "fields"
  rule exUnwrapArray(ejArray(L)) => L
  rule exUnwrapArray(_) => ejNil() [owise]
  rule exFieldExpressions(ejNil()) => ejNil()
  rule exFieldExpressions(ejCons(F,L)) => ejCons(ejGet(F,"value"),exFieldExpressions(L))
  syntax String ::= exPreType(String,EJSON,EJSON,EJSON,EJSON) [function, total]
                  | exBinder(EJSON,EJSON,EJSON) [function, total]
                  | exBinderMaps(EJSON,EJSON,EJList) [function, total]
                  | exRecordNames(EJSON,EJList,EJList) [function, total]
                  | exRecordCoverage(EJSON,EJList) [function, total]
  rule exPreType("Let",O,S,L,_) => exBinder(ejGet(O,"name"),S,L)
  rule exPreType("NextWrite",O,S,_,W) => exWritePre(ejGet(O,"field"),S,W)
  syntax String ::= exWritePre(EJSON,EJSON,EJSON) [function, total]
  rule exWritePre(ejString(F),S,W) => exFirst(exCheck(ejHas(ejGet(S,"fields"),F),"TYPE_NAME"),exFirst(exCheck(ejGet(ejGet(ejGet(S,"fields"),F),"writeClass") ==K ejString("ordinary"),"TYPE_FINANCIAL_WRITE"),exCheck(notBool ejHas(W,F),"TYPE_DUPLICATE_WRITE")))
  rule exWritePre(_,_,_) => "INPUT_SCHEMA" [owise]
  rule exPreType("Emit",O,S,_,_) => exCheck(exHasName(ejGet(S,"operations"),ejGet(O,"operation")),"TYPE_NAME")
  rule exPreType("ConstructRecord",O,S,_,_) => exFirst(exCheck(exHasName(ejGet(S,"recordTypes"),ejGet(O,"recordType")),"TYPE_NAME"),exFirst(exRecordNames(O,exUnwrapArray(ejGet(O,"fields")),ejNil()),exRecordCoverage(exGetName(ejGet(S,"recordTypes"),ejGet(O,"recordType")),exUnwrapArray(ejGet(O,"fields")))))
  rule exPreType(K,O,S,_,_) => exResolve(S,ejGet(O,"elementType"),"TYPE_LITERAL") requires K ==String "ConstructSome" orBool K ==String "ConstructNone"
  rule exPreType("ConstructCollection",O,S,_,_) => exFirst(exResolve(S,ejGet(O,"elementType"),"TYPE_LITERAL"),exCheck(exInt(ejGet(O,"capacity")) >=Int 0 andBool exInt(ejGet(O,"capacity")) <=Int 128 andBool exArrayLength(ejGet(O,"items")) <=Int exInt(ejGet(O,"capacity")),"TYPE_COLLECTION_BOUND"))
  rule exPreType(_,_,_,_,_) => "" [owise]
  syntax Bool ::= exHasName(EJSON,EJSON) [function, total]
  syntax EJSON ::= exGetName(EJSON,EJSON) [function, total]
  rule exHasName(M,ejString(N)) => ejHas(M,N)
  rule exHasName(_,_) => false [owise]
  rule exGetName(M,ejString(N)) => ejGet(M,N)
  rule exGetName(_,_) => ejMissing() [owise]
  rule exBinder(ejString(N),S,L) => exFirst(exCheck(notBool ejHas(L,N) andBool N =/=String "pre" andBool N =/=String "post" andBool N =/=String "next","TYPE_DUPLICATE_BINDER"),exBinderMaps(ejString(N),S,exSchemaKeys()))
  rule exBinder(_,_,_) => "INPUT_SCHEMA" [owise]
  rule exBinderMaps(_,_,ejNil()) => ""
  rule exBinderMaps(N,S,ejCons(ejString(K),R)) => exFirst(exCheck(notBool exHasName(ejGet(S,K),N) andBool notBool exIn(N,ejGet(S,K)),"TYPE_DUPLICATE_BINDER"),exBinderMaps(N,S,R))
  rule exRecordNames(_,ejNil(),_) => ""
  rule exRecordNames(O,ejCons(F,R),Seen) => exFirst(exCheck(notBool ejMember(ejGet(F,"name"),Seen),"TYPE_DUPLICATE_FIELD"),exRecordNames(O,R,ejCons(ejGet(F,"name"),Seen)))
  rule exRecordCoverage(T,F) => exFirst(exCheck(exSize(T) ==Int ejLength(F),"TYPE_RECORD_FIELDS"),exRecordFieldNames(T,F))
  syntax String ::= exRecordFieldNames(EJSON,EJList) [function, total]
  rule exRecordFieldNames(_,ejNil()) => ""
  rule exRecordFieldNames(T,ejCons(F,R)) => exFirst(exCheck(exHasName(T,ejGet(F,"name")),"TYPE_RECORD_FIELDS"),exRecordFieldNames(T,R))
  syntax EXInference ::= exPrimitiveType(String,EJSON,EJSON,EJSON,Bool,EJSON) [function, total]
                       | exTypeIf(String,EJSON) [function, total]
                       | exLookupType(EJSON,EJSON) [function, total]
  rule exTypeIf("",T) => exTyped(T)
  rule exTypeIf(C,_) => exTypeError(C,exSynthetic(),ejNil()) requires C =/=String ""
  rule exLookupType(M,N) => exTypeIf(exCheck(exHasName(M,N),"TYPE_NAME"),exGetName(M,N))
  rule exPrimitiveType("ReadLocal",O,_,L,_,_) => exLookupType(L,ejGet(O,"name"))
  rule exPrimitiveType("ReadArg",O,S,_,_,_) => exLookupType(ejGet(S,"args"),ejGet(O,"name"))
  rule exPrimitiveType("ReadObs",O,S,_,_,_) => exLookupType(ejGet(S,"observations"),ejGet(O,"name"))
  rule exPrimitiveType("ReadPre",O,S,_,E,_) => exTypeIf(exFirst(exCheck(ejGet(O,"view") =/=K ejString("next"),"TYPE_NEXT_READ"),exFirst(exCheck(ejGet(O,"view") =/=K ejString("post") orBool E,"TYPE_POST_SCOPE"),exCheck(exHasName(ejGet(S,"fields"),ejGet(O,"field")),"TYPE_NAME"))),ejGet(exGetName(ejGet(S,"fields"),ejGet(O,"field")),"type"))
  rule exPrimitiveType(K,O,S,_,_,T) => exTypeIf(exFirst(exCheck(ejAt(ejAt(T,0),0) ==K ejString("Record"),"TYPE_MISMATCH"),exCheck(exHasName(exGetName(ejGet(S,"recordTypes"),ejAt(ejAt(T,0),1)),ejGet(O,"field")),"TYPE_NAME")),exGetName(exGetName(ejGet(S,"recordTypes"),ejAt(ejAt(T,0),1)),ejGet(O,"field"))) requires K ==String "ProjectField" orBool K ==String "AccessField"
  rule exPrimitiveType(K,_,_,_,_,T) => exTypeIf(exCheck(ejAt(ejAt(T,0),0) ==K ejString("Collection") andBool ejAt(T,1) ==K exType("UInt64"),"TYPE_MISMATCH"),ejAt(ejAt(T,0),1)) requires K ==String "ProjectIndex" orBool K ==String "AccessIndex"
  rule exPrimitiveType("ConstructEnum",O,S,_,_,_) => exTypeIf(exFirst(exCheck(exHasName(ejGet(S,"enumTypes"),ejGet(O,"enumType")),"TYPE_NAME"),exCheck(exIn(ejGet(O,"member"),exGetName(ejGet(S,"enumTypes"),ejGet(O,"enumType"))),"TYPE_ENUM_MEMBER")),exType("Enum",ejGet(O,"enumType")))
  rule exPrimitiveType("ConstructNone",O,_,_,_,_) => exTyped(exType("Option",ejGet(O,"elementType")))
  rule exPrimitiveType("ConstructSome",O,_,_,_,T) => exTypeIf(exCheck(ejAt(T,0) ==K ejGet(O,"elementType"),"TYPE_MISMATCH"),exType("Option",ejGet(O,"elementType")))
  rule exPrimitiveType("ConstructCollection",O,_,_,_,T) => exTypeIf(exAllTypes(exUnwrapArray(T),ejGet(O,"elementType")),exType("Collection",ejGet(O,"elementType"),ejGet(O,"capacity")))
  rule exPrimitiveType("ConstructRecord",O,S,_,_,T) => exTypeIf(exRecordTypeFields(exGetName(ejGet(S,"recordTypes"),ejGet(O,"recordType")),exUnwrapArray(ejGet(O,"fields")),exUnwrapArray(T)),exType("Record",ejGet(O,"recordType")))
  rule exPrimitiveType(K,_,_,_,_,T) => exTypeIf(exAllTypes(exUnwrapArray(T),exType("Bool")),exType("Bool")) requires K ==String "Not" orBool K ==String "And" orBool K ==String "Or"
  rule exPrimitiveType(K,_,_,_,_,T) => exTypeIf(exCheck(ejAt(T,0) ==K ejAt(T,1) andBool (K ==String "Eq" orBool exNumeric(ejAt(T,0))),"TYPE_MISMATCH"),exType("Bool")) requires K ==String "Eq" orBool K ==String "Lt" orBool K ==String "Lte" orBool K ==String "Gt" orBool K ==String "Gte"
  rule exPrimitiveType(K,_,_,_,_,T) => exTypeIf(exCheck(ejAt(T,0) ==K exType("Bool"),"TYPE_MISMATCH"),exType("Unit")) requires K ==String "Require" orBool K ==String "Ensure"
  rule exPrimitiveType("Let",_,_,_,_,_) => exTyped(exType("Unit"))
  rule exPrimitiveType("NextWrite",O,S,_,_,T) => exTypeIf(exCheck(ejAt(T,0) ==K ejGet(exGetName(ejGet(S,"fields"),ejGet(O,"field")),"type"),"TYPE_MISMATCH"),exType("Unit"))
  rule exPrimitiveType("Emit",O,S,_,_,T) => exTypeIf(exCheck(ejAt(T,0) ==K exType("Record",exGetName(ejGet(S,"operations"),ejGet(O,"operation"))),"TYPE_MISMATCH"),exType("Unit"))
  syntax String ::= exAllTypes(EJList,EJSON) [function, total]
                  | exRecordTypeFields(EJSON,EJList,EJList) [function, total]
  rule exAllTypes(ejNil(),_) => ""
  rule exAllTypes(ejCons(T,L),Expected) => exFirst(exCheck(T ==K Expected,"TYPE_MISMATCH"),exAllTypes(L,Expected))
  rule exRecordTypeFields(_,ejNil(),ejNil()) => ""
  rule exRecordTypeFields(S,ejCons(F,L),ejCons(T,R)) => exFirst(exCheck(T ==K exGetName(S,ejGet(F,"name")),"TYPE_MISMATCH"),exRecordTypeFields(S,L,R))
  rule exRecordTypeFields(_,_,_) => "TYPE_MISMATCH" [owise]
  syntax Bool ::= exLiteral(String) [function, total]
  rule exLiteral(K) => K ==String "LitUInt" orBool K ==String "LitSInt" orBool K ==String "LitBool" orBool K ==String "LitText" orBool K ==String "LitAmount" orBool K ==String "LitQuantity" orBool K ==String "LitShares" orBool K ==String "LitRate" orBool K ==String "LitPrice"
  syntax EJSON ::= exLiteralType(String,EJSON) [function, total]
                 | exLiteralValue(String,EJSON) [function, total]
  rule exLiteralType("LitUInt",O) => exType("UInt" +String exText(ejGet(O,"width")))
  rule exLiteralType("LitSInt",_) => exType("SInt128")
  rule exLiteralType("LitBool",_) => exType("Bool")
  rule exLiteralType("LitText",_) => exType("Text")
  rule exLiteralType("LitAmount",O) => exType("Amount",ejGet(O,"asset"))
  rule exLiteralType("LitQuantity",O) => exType("Quantity",ejGet(O,"units"),ejGet(O,"scale"))
  rule exLiteralType("LitShares",O) => exType("Shares",ejGet(O,"vault"),ejGet(O,"holder"))
  rule exLiteralType("LitRate",O) => exType("Rate",ejGet(O,"scale"))
  rule exLiteralType("LitPrice",O) => exType("Price",ejGet(O,"base"),ejGet(O,"quote"),ejGet(O,"scale"))
  rule exLiteralType(_,_) => ejMissing() [owise]
  rule exLiteralValue(K,O) => ejGet(O,"mantissa") requires K ==String "LitQuantity" orBool K ==String "LitRate" orBool K ==String "LitPrice"
  rule exLiteralValue(_,O) => ejGet(O,"value") [owise]
  syntax String ::= exText(EJSON) [function, total]
  rule exText(ejString(S)) => S
  rule exText(_) => "" [owise]
  rule exPrimitiveType(K,O,S,_,_,_) => exTypeIf(exFirst(exCheck(K =/=String "LitUInt" orBool ejGet(O,"width") ==K ejString("64") orBool ejGet(O,"width") ==K ejString("128"),"TYPE_LITERAL"),exFirst(exResolve(S,exLiteralType(K,O),"TYPE_LITERAL"),exFirst(exValueDomain(S,exLiteralType(K,O),exLiteralValue(K,O),"TYPE_LITERAL"),exValueBound(S,exLiteralType(K,O),exLiteralValue(K,O),"TYPE_LITERAL")))),exLiteralType(K,O)) requires exLiteral(K)
  rule exPrimitiveType(K,_,S,_,_,T) => exArithmeticType(K,ejAt(T,0),ejAt(T,1),S) requires K ==String "Add" orBool K ==String "Sub" orBool K ==String "Mul" orBool K ==String "FloorDiv" orBool K ==String "CeilDiv"
  syntax EXInference ::= exArithmeticType(String,EJSON,EJSON,EJSON) [function, total]
  rule exArithmeticType(K,L,R,S) => exQuantityType(K,L,R,S) requires (K ==String "Mul" orBool K ==String "FloorDiv" orBool K ==String "CeilDiv") andBool ejAt(L,0) ==K ejString("Quantity") andBool ejAt(R,0) ==K ejString("Quantity")
  rule exArithmeticType(K,L,R,_) => exTypeIf(exCheck(L ==K R andBool (L ==K exType("UInt64") orBool L ==K exType("UInt128") orBool L ==K exType("SInt128") orBool ((K ==String "Add" orBool K ==String "Sub") andBool (ejAt(L,0) ==K ejString("Amount") orBool ejAt(L,0) ==K ejString("Shares") orBool ejAt(L,0) ==K ejString("Rate") orBool ejAt(L,0) ==K ejString("Quantity")))),"TYPE_MISMATCH"),L) [owise]
  syntax EXInference ::= exQuantityType(String,EJSON,EJSON,EJSON) [function, total]
                       | exQuantityResult(EJSON,EJSON) [function, total]
  syntax Int ::= exDirection(String) [function, total]
  rule exDirection("Mul") => 1
  rule exDirection(_) => -1 [owise]
  rule exQuantityType(K,L,R,S) => exQuantityResult(S,exType("Quantity",ejArray(exMergeUnits(exUnwrapArray(ejAt(L,1)),exUnwrapArray(ejAt(R,1)),exDirection(K))),ejString(Int2String(exInt(ejAt(L,2)) +Int exDirection(K) *Int exInt(ejAt(R,2))))))
  rule exQuantityResult(S,T) => exTypeIf(exResolve(S,T,"TYPE_QUANTITY_DOMAIN"),T)
  syntax EJList ::= exMergeUnits(EJList,EJList,Int) [function, total]
                  | exSignedUnits(EJList,Int) [function, total]
                  | exUnitCons(String,Int,EJList) [function, total]
  rule exUnitCons(_,0,L) => L
  rule exUnitCons(N,P,L) => ejCons(ejArray(ejCons(ejString(N),ejCons(ejString(Int2String(P)),ejNil()))),L) requires P =/=Int 0
  rule exSignedUnits(ejNil(),_) => ejNil()
  rule exSignedUnits(ejCons(ejArray(ejCons(ejString(N),ejCons(P,ejNil()))),L),D) => exUnitCons(N,D *Int exInt(P),exSignedUnits(L,D))
  rule exMergeUnits(L,ejNil(),_) => L
  rule exMergeUnits(ejNil(),R,D) => exSignedUnits(R,D) requires R =/=K ejNil()
  rule exMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),ejCons(ejArray(ejCons(ejString(B),ejCons(Q,ejNil()))),R),D) => exUnitCons(A,exInt(P),exMergeUnits(L,ejCons(ejArray(ejCons(ejString(B),ejCons(Q,ejNil()))),R),D)) requires A <String B
  rule exMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),ejCons(ejArray(ejCons(ejString(B),ejCons(Q,ejNil()))),R),D) => exUnitCons(B,D *Int exInt(Q),exMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),R,D)) requires A >String B
  rule exMergeUnits(ejCons(ejArray(ejCons(ejString(A),ejCons(P,ejNil()))),L),ejCons(ejArray(ejCons(ejString(A),ejCons(Q,ejNil()))),R),D) => exUnitCons(A,exInt(P) +Int D *Int exInt(Q),exMergeUnits(L,R,D))
  rule exMergeUnits(_,_,_) => ejNil() [owise]
  rule exPrimitiveType(_,_,_,_,_,_) => exTypeError("TYPE_CONSTRUCTOR",exSynthetic(),ejNil()) [owise]
endmodule
