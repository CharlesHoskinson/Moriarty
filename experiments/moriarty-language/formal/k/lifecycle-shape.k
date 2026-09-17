// Isolated Core/4 extension of the expression definition; legacy files are unchanged.
requires "lifecycle-types.k"
module MORIARTY-LIFECYCLE-EXPRESSION-SHAPE
  imports MORIARTY-LIFECYCLE-EXPRESSION-TYPES
  syntax LXRole ::= lxRole(String,String)
  syntax LXRoles ::= lxRolesNil() | lxRolesCons(LXRole,LXRoles)
  syntax LXRoles ::= lxRoles(String) [function, total]
  rule lxRoles("LitUInt") => lxRolesCons(lxRole("width","int"),lxRolesCons(lxRole("value","int"),lxRolesNil()))
  rule lxRoles("LitSInt") => lxRolesCons(lxRole("value","int"),lxRolesNil())
  rule lxRoles("LitBool") => lxRolesCons(lxRole("value","bool"),lxRolesNil())
  rule lxRoles("LitText") => lxRolesCons(lxRole("value","text"),lxRolesNil())
  rule lxRoles("LitAmount") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("value","int"),lxRolesNil()))
  rule lxRoles("LitQuantity") => lxRolesCons(lxRole("units","units"),lxRolesCons(lxRole("scale","int"),lxRolesCons(lxRole("mantissa","int"),lxRolesNil())))
  rule lxRoles("LitShares") => lxRolesCons(lxRole("vault","id"),lxRolesCons(lxRole("holder","id"),lxRolesCons(lxRole("value","int"),lxRolesNil())))
  rule lxRoles("LitRate") => lxRolesCons(lxRole("scale","int"),lxRolesCons(lxRole("mantissa","int"),lxRolesNil()))
  rule lxRoles("LitPrice") => lxRolesCons(lxRole("base","id"),lxRolesCons(lxRole("quote","id"),lxRolesCons(lxRole("scale","int"),lxRolesCons(lxRole("mantissa","int"),lxRolesNil()))))
  rule lxRoles("ReadLocal") => lxRolesCons(lxRole("name","id"),lxRolesNil())
  rule lxRoles("ReadArg") => lxRolesCons(lxRole("name","id"),lxRolesNil())
  rule lxRoles("ReadObs") => lxRolesCons(lxRole("name","id"),lxRolesNil())
  rule lxRoles("ReadPre") => lxRolesCons(lxRole("view","view"),lxRolesCons(lxRole("field","id"),lxRolesNil()))
  rule lxRoles("ProjectField") => lxRolesCons(lxRole("record","expr"),lxRolesCons(lxRole("field","id"),lxRolesNil()))
  rule lxRoles("AccessField") => lxRolesCons(lxRole("record","expr"),lxRolesCons(lxRole("field","id"),lxRolesNil()))
  rule lxRoles("ProjectIndex") => lxRolesCons(lxRole("collection","expr"),lxRolesCons(lxRole("index","expr"),lxRolesNil()))
  rule lxRoles("AccessIndex") => lxRolesCons(lxRole("collection","expr"),lxRolesCons(lxRole("index","expr"),lxRolesNil()))
  rule lxRoles("ConstructRecord") => lxRolesCons(lxRole("recordType","id"),lxRolesCons(lxRole("fields","fields"),lxRolesNil()))
  rule lxRoles("ConstructEnum") => lxRolesCons(lxRole("enumType","id"),lxRolesCons(lxRole("member","id"),lxRolesNil()))
  rule lxRoles("ConstructSome") => lxRolesCons(lxRole("elementType","type"),lxRolesCons(lxRole("value","expr"),lxRolesNil()))
  rule lxRoles("ConstructNone") => lxRolesCons(lxRole("elementType","type"),lxRolesNil())
  rule lxRoles("ConstructCollection") => lxRolesCons(lxRole("elementType","type"),lxRolesCons(lxRole("capacity","int"),lxRolesCons(lxRole("items","list"),lxRolesNil())))
  rule lxRoles("Add") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Sub") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Mul") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("FloorDiv") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("CeilDiv") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Eq") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Lt") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Lte") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Gt") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Gte") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Not") => lxRolesCons(lxRole("value","expr"),lxRolesNil())
  rule lxRoles("And") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Or") => lxRolesCons(lxRole("left","expr"),lxRolesCons(lxRole("right","expr"),lxRolesNil()))
  rule lxRoles("Require") => lxRolesCons(lxRole("condition","expr"),lxRolesNil())
  rule lxRoles("Ensure") => lxRolesCons(lxRole("condition","expr"),lxRolesNil())
  rule lxRoles("Let") => lxRolesCons(lxRole("name","id"),lxRolesCons(lxRole("value","expr"),lxRolesNil()))
  rule lxRoles("NextWrite") => lxRolesCons(lxRole("field","id"),lxRolesCons(lxRole("value","expr"),lxRolesNil()))
  rule lxRoles("Emit") => lxRolesCons(lxRole("operation","id"),lxRolesCons(lxRole("fields","expr"),lxRolesNil()))
  rule lxRoles(_) => lxRolesNil() [owise]
  syntax Bool ::= lxConstructor(String) [function, total]
  rule lxConstructor("LitUInt") => true
  rule lxConstructor("LitSInt") => true
  rule lxConstructor("LitBool") => true
  rule lxConstructor("LitText") => true
  rule lxConstructor("LitAmount") => true
  rule lxConstructor("LitQuantity") => true
  rule lxConstructor("LitShares") => true
  rule lxConstructor("LitRate") => true
  rule lxConstructor("LitPrice") => true
  rule lxConstructor("ReadLocal") => true
  rule lxConstructor("ReadArg") => true
  rule lxConstructor("ReadObs") => true
  rule lxConstructor("ReadPre") => true
  rule lxConstructor("ProjectField") => true
  rule lxConstructor("AccessField") => true
  rule lxConstructor("ProjectIndex") => true
  rule lxConstructor("AccessIndex") => true
  rule lxConstructor("ConstructRecord") => true
  rule lxConstructor("ConstructEnum") => true
  rule lxConstructor("ConstructSome") => true
  rule lxConstructor("ConstructNone") => true
  rule lxConstructor("ConstructCollection") => true
  rule lxConstructor("Add") => true
  rule lxConstructor("Sub") => true
  rule lxConstructor("Mul") => true
  rule lxConstructor("FloorDiv") => true
  rule lxConstructor("CeilDiv") => true
  rule lxConstructor("Eq") => true
  rule lxConstructor("Lt") => true
  rule lxConstructor("Lte") => true
  rule lxConstructor("Gt") => true
  rule lxConstructor("Gte") => true
  rule lxConstructor("Not") => true
  rule lxConstructor("And") => true
  rule lxConstructor("Or") => true
  rule lxConstructor("Require") => true
  rule lxConstructor("Ensure") => true
  rule lxConstructor("Let") => true
  rule lxConstructor("NextWrite") => true
  rule lxConstructor("Emit") => true
  rule lxConstructor(_) => false [owise]
  syntax EJList ::= lxRoleNames(LXRoles) [function, total]
  rule lxRoleNames(lxRolesNil()) => ejNil()
  rule lxRoleNames(lxRolesCons(lxRole(N,_),R)) => ejCons(ejString(N),lxRoleNames(R))
  syntax LXSite ::= lxGood() | lxBad(String,EJSON,EJList)
  syntax LXSite ::= lxThen(LXSite,LXSite) [function, total]
                  | lxAtCode(String,EJSON,EJList) [function, total]
                  | lxLocate(LXSite,EJSON,EJList) [function, total]
                  | lxCoreShape(EJSON,Int) [function, total]
                  | lxShapeNode(EJSON,EJList,Int,Int,Int) [function, total]
                  | lxShapeFields(EJSON,EJList,Int,Int,Int) [function, total]
                  | lxShapeNamed(EJSON,EJSON,EJList,Int,Int,Int) [function, total]
                  | lxShapeOperands(LXRoles,EJSON,EJList,Int,Int,Int,Int) [function, total]
                  | lxShapeList(EJList,EJList,Int,Int,Int,Int,Bool) [function, total]
                  | lxShapeAction(EJList,Int,Int,Int) [function, total]
  rule lxThen(lxGood(),R) => R
  rule lxThen(lxBad(C,S,P),_) => lxBad(C,S,P)
  rule lxAtCode("",_,_) => lxGood()
  rule lxAtCode(C,S,P) => lxBad(C,S,P) requires C =/=String ""
  rule lxLocate(lxBad(C,_,P),S,P) => lxBad(C,S,P)
  rule lxLocate(R,_,_) => R [owise]
  rule lxCoreShape(N,B) => lxActionShape(N,B) requires ejHas(N,"statements")
  rule lxCoreShape(N,B) => lxShapeNode(N,ejNil(),1,B,0) requires notBool ejHas(N,"statements")
  syntax LXSite ::= lxActionShape(EJSON,Int) [function, total]
  rule lxActionShape(N,B) => lxThen(lxAtCode(lxFirst(lxCheck(ejClosed(N,ejCons(ejString("statements"),ejCons(ejString("span"),ejNil()))),"INPUT_SCHEMA"),lxFirst(lxSpanCode(ejGet(N,"span"),B),lxFirst(lxCheck(ejIsArray(ejGet(N,"statements")),"INPUT_SCHEMA"),lxCheck(lxArrayLength(ejGet(N,"statements")) <=Int 256,"INPUT_BOUND")))),lxSafeSpan(ejGet(N,"span"),B),ejNil()),lxShapeAction(lxArrayItems(ejGet(N,"statements")),B,0,0))
  rule lxShapeAction(ejNil(),_,_,_) => lxGood()
  rule lxShapeAction(ejCons(N,L),B,I,Seen) => lxThen(lxShapeNode(N,ejCons(ejString(Int2String(I)),ejNil()),1,B,Seen),lxShapeAction(L,B,I +Int 1,Seen +Int lxASTSize(N)))
  rule lxShapeNode(N,P,D,B,Seen) => lxBad("INPUT_BOUND",lxSafeSpan(ejGet(N,"span"),B),P) requires D >Int 64 orBool Seen >=Int 4096
  rule lxShapeNode(N,P,D,B,Seen) => lxThen(lxAtCode(lxCheck(ejClosed(N,ejCons(ejString("constructor"),ejCons(ejString("operands"),ejCons(ejString("span"),ejNil())))),"INPUT_SCHEMA"),lxSafeSpan(ejGet(N,"span"),B),P),lxShapeFields(N,P,D,B,Seen)) requires D <=Int 64 andBool Seen <Int 4096
  rule lxShapeFields(N,P,D,B,Seen) => lxThen(lxAtCode(lxSpanCode(ejGet(N,"span"),B),lxSafeSpan(ejGet(N,"span"),B),P),lxShapeNamed(ejGet(N,"constructor"),N,P,D,B,Seen))
  rule lxShapeNamed(ejString(K),N,P,D,B,Seen) => lxLocate(lxThen(lxAtCode(lxCheck(lxConstructor(K),"TYPE_CONSTRUCTOR"),lxSynthetic(),P),lxThen(lxAtCode(lxCheck(ejClosed(ejGet(N,"operands"),lxRoleNames(lxRoles(K))),"INPUT_SCHEMA"),lxSynthetic(),P),lxShapeOperands(lxRoles(K),ejGet(N,"operands"),P,D,B,0,Seen +Int 1))),lxSafeSpan(ejGet(N,"span"),B),P)
  rule lxShapeNamed(_,N,P,_,B,_) => lxBad("TYPE_CONSTRUCTOR",lxSafeSpan(ejGet(N,"span"),B),P) [owise]
  rule lxShapeOperands(lxRolesNil(),_,_,_,_,_,_) => lxGood()
  rule lxShapeOperands(lxRolesCons(lxRole(N,"expr"),R),O,P,D,B,I,Seen) => lxThen(lxShapeNode(ejGet(O,N),ejAppend(P,ejCons(ejString(Int2String(I)),ejNil())),D +Int 1,B,Seen),lxShapeOperands(R,O,P,D,B,I +Int 1,Seen +Int lxASTSize(ejGet(O,N))))
  rule lxShapeOperands(lxRolesCons(lxRole(N,K),R),O,P,D,B,I,Seen) => lxThen(lxAtCode(lxMetaShape(K,ejGet(O,N)),lxSynthetic(),P),lxShapeOperands(R,O,P,D,B,I,Seen)) requires K =/=String "expr" andBool K =/=String "list" andBool K =/=String "fields"
  rule lxShapeOperands(lxRolesCons(lxRole(N,K),R),O,P,D,B,I,Seen) => lxThen(lxAtCode(lxFirst(lxCheck(ejIsArray(ejGet(O,N)),"INPUT_SCHEMA"),lxCheck(lxArrayLength(ejGet(O,N)) <=Int lxListLimit(K),"INPUT_BOUND")),lxSynthetic(),P),lxThen(lxShapeList(lxArrayItems(ejGet(O,N)),P,D,B,I,Seen,K ==String "fields"),lxShapeOperands(R,O,P,D,B,I +Int lxArrayLength(ejGet(O,N)),Seen +Int lxASTListSize(lxArrayItems(ejGet(O,N)),K ==String "fields")))) requires K ==String "list" orBool K ==String "fields"
  rule lxShapeList(ejNil(),_,_,_,_,_,_) => lxGood()
  rule lxShapeList(ejCons(N,L),P,D,B,I,Seen,false) => lxThen(lxShapeNode(N,ejAppend(P,ejCons(ejString(Int2String(I)),ejNil())),D +Int 1,B,Seen),lxShapeList(L,P,D,B,I +Int 1,Seen +Int lxASTSize(N),false))
  rule lxShapeList(ejCons(N,L),P,D,B,I,Seen,true) => lxThen(lxAtCode(lxCheck(ejClosed(N,ejCons(ejString("name"),ejCons(ejString("value"),ejNil()))) andBool lxIdentifier(ejGet(N,"name")),"INPUT_SCHEMA"),lxSynthetic(),P),lxThen(lxShapeNode(ejGet(N,"value"),ejAppend(P,ejCons(ejString(Int2String(I)),ejNil())),D +Int 1,B,Seen),lxShapeList(L,P,D,B,I +Int 1,Seen +Int lxASTSize(ejGet(N,"value")),true)))
  syntax Int ::= lxArrayLength(EJSON) [function, total]
               | lxListLimit(String) [function, total]
               | lxASTSize(EJSON) [function, total]
               | lxASTRoles(LXRoles,EJSON) [function, total]
               | lxASTListSize(EJList,Bool) [function, total]
  syntax EJList ::= lxArrayItems(EJSON) [function, total]
  rule lxArrayLength(ejArray(L)) => ejLength(L)
  rule lxArrayLength(_) => 0 [owise]
  rule lxArrayItems(ejArray(L)) => L
  rule lxArrayItems(_) => ejNil() [owise]
  rule lxListLimit("fields") => 64
  rule lxListLimit(_) => 128 [owise]
  rule lxASTSize(N) => 1 +Int lxASTName(ejGet(N,"constructor"),ejGet(N,"operands"))
  syntax Int ::= lxASTName(EJSON,EJSON) [function, total]
  rule lxASTName(ejString(K),O) => lxASTRoles(lxRoles(K),O)
  rule lxASTName(_,_) => 0 [owise]
  rule lxASTRoles(lxRolesNil(),_) => 0
  rule lxASTRoles(lxRolesCons(lxRole(N,"expr"),R),O) => lxASTSize(ejGet(O,N)) +Int lxASTRoles(R,O)
  rule lxASTRoles(lxRolesCons(lxRole(N,K),R),O) => lxASTListSize(lxArrayItems(ejGet(O,N)),K ==String "fields") +Int lxASTRoles(R,O) requires K ==String "list" orBool K ==String "fields"
  rule lxASTRoles(lxRolesCons(lxRole(_,K),R),O) => lxASTRoles(R,O) requires K =/=String "expr" andBool K =/=String "list" andBool K =/=String "fields"
  rule lxASTListSize(ejNil(),_) => 0
  rule lxASTListSize(ejCons(N,L),false) => lxASTSize(N) +Int lxASTListSize(L,false)
  rule lxASTListSize(ejCons(N,L),true) => lxASTSize(ejGet(N,"value")) +Int lxASTListSize(L,true)
  syntax String ::= lxMetaShape(String,EJSON) [function, total]
                  | lxSpanCode(EJSON,Int) [function, total]
  rule lxMetaShape("id",V) => lxCheck(lxIdentifier(V),"INPUT_SCHEMA")
  rule lxMetaShape("int",V) => lxCheck(lxDecimal(V),"INPUT_SCHEMA")
  rule lxMetaShape("text",V) => lxCheck(ejIsString(V),"INPUT_SCHEMA")
  rule lxMetaShape("bool",V) => lxCheck(V ==K ejBool(true) orBool V ==K ejBool(false),"INPUT_SCHEMA")
  rule lxMetaShape("view",V) => lxCheck(V ==K ejString("pre") orBool V ==K ejString("post") orBool V ==K ejString("next"),"INPUT_SCHEMA")
  rule lxMetaShape("type",V) => lxCheck(lxTypeShape(V),"INPUT_SCHEMA")
  rule lxMetaShape("units",V) => lxCheck(lxUnitShape(V),"INPUT_SCHEMA")
  rule lxMetaShape(_,_) => "INPUT_SCHEMA" [owise]
  rule lxSpanCode(S,B) => lxFirst(lxCheck(ejClosed(S,ejCons(ejString("kind"),ejCons(ejString("start"),ejCons(ejString("end"),ejNil())))) andBool lxDecimal(ejGet(S,"start")) andBool lxDecimal(ejGet(S,"end")) andBool (ejGet(S,"kind") ==K ejString("source") orBool ejGet(S,"kind") ==K ejString("synthetic")),"INPUT_SCHEMA"),lxCheck(lxSpan(S,B),"INPUT_SPAN"))
  rule lxRoles("ConstructAmount") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("value","expr"),lxRolesNil()))
  rule lxConstructor("ConstructAmount") => true
  rule lxRoles("ScalarValue") => lxRolesCons(lxRole("component","id"),lxRolesCons(lxRole("value","expr"),lxRolesNil()))
  rule lxConstructor("ScalarValue") => true
  rule lxRoles("ProjectSome") => lxRolesCons(lxRole("value","expr"),lxRolesNil())
  rule lxConstructor("ProjectSome") => true
  rule lxRoles("ConvertUInt") => lxRolesCons(lxRole("width","int"),lxRolesCons(lxRole("value","expr"),lxRolesNil()))
  rule lxConstructor("ConvertUInt") => true
  rule lxRoles("Select") => lxRolesCons(lxRole("condition","expr"),lxRolesCons(lxRole("consequent","expr"),lxRolesCons(lxRole("alternative","expr"),lxRolesNil())))
  rule lxConstructor("Select") => true
  rule lxRoles("ReadOutstanding") => lxRolesCons(lxRole("unit","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadOutstanding") => true
  rule lxRoles("ReadPrincipal") => lxRolesCons(lxRole("unit","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadPrincipal") => true
  rule lxRoles("ReadAccrued") => lxRolesCons(lxRole("unit","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadAccrued") => true
  rule lxRoles("ReadBalance") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadBalance") => true
  rule lxRoles("ReadAllowanceRemaining") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadAllowanceRemaining") => true
  rule lxRoles("ReadAllowanceSpent") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadAllowanceSpent") => true
  rule lxRoles("ReadPostOutstanding") => lxRolesCons(lxRole("unit","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadPostOutstanding") => true
  rule lxRoles("ReadPostPrincipal") => lxRolesCons(lxRole("unit","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadPostPrincipal") => true
  rule lxRoles("ReadPostAccrued") => lxRolesCons(lxRole("unit","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadPostAccrued") => true
  rule lxRoles("ReadPostBalance") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadPostBalance") => true
  rule lxRoles("ReadPostAllowanceRemaining") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadPostAllowanceRemaining") => true
  rule lxRoles("ReadPostAllowanceSpent") => lxRolesCons(lxRole("asset","id"),lxRolesCons(lxRole("identity","expr"),lxRolesNil()))
  rule lxConstructor("ReadPostAllowanceSpent") => true
  rule lxConstructor("ConstructShares") => true
  rule lxConstructor("ConstructVariant") => true
  rule lxConstructor("ProjectVariant") => true
  rule lxRoles("ConstructShares") => lxRolesCons(lxRole("vault","id"),lxRolesCons(lxRole("holder","id"),lxRolesCons(lxRole("value","expr"),lxRolesNil())))
  rule lxRoles("ConstructVariant") => lxRolesCons(lxRole("family","id"),lxRolesCons(lxRole("tag","id"),lxRolesCons(lxRole("value","expr"),lxRolesNil())))
  rule lxRoles("ProjectVariant") => lxRolesCons(lxRole("tag","id"),lxRolesCons(lxRole("value","expr"),lxRolesNil()))
endmodule
