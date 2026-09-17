requires "expression-types.k"
module MORIARTY-EXPRESSION-SHAPE
  imports MORIARTY-EXPRESSION-TYPES
  syntax EXRole ::= exRole(String,String)
  syntax EXRoles ::= exRolesNil() | exRolesCons(EXRole,EXRoles)
  syntax EXRoles ::= exRoles(String) [function, total]
  rule exRoles("LitUInt") => exRolesCons(exRole("width","int"),exRolesCons(exRole("value","int"),exRolesNil()))
  rule exRoles("LitSInt") => exRolesCons(exRole("value","int"),exRolesNil())
  rule exRoles("LitBool") => exRolesCons(exRole("value","bool"),exRolesNil())
  rule exRoles("LitText") => exRolesCons(exRole("value","text"),exRolesNil())
  rule exRoles("LitAmount") => exRolesCons(exRole("asset","id"),exRolesCons(exRole("value","int"),exRolesNil()))
  rule exRoles("LitQuantity") => exRolesCons(exRole("units","units"),exRolesCons(exRole("scale","int"),exRolesCons(exRole("mantissa","int"),exRolesNil())))
  rule exRoles("LitShares") => exRolesCons(exRole("vault","id"),exRolesCons(exRole("holder","id"),exRolesCons(exRole("value","int"),exRolesNil())))
  rule exRoles("LitRate") => exRolesCons(exRole("scale","int"),exRolesCons(exRole("mantissa","int"),exRolesNil()))
  rule exRoles("LitPrice") => exRolesCons(exRole("base","id"),exRolesCons(exRole("quote","id"),exRolesCons(exRole("scale","int"),exRolesCons(exRole("mantissa","int"),exRolesNil()))))
  rule exRoles("ReadLocal") => exRolesCons(exRole("name","id"),exRolesNil())
  rule exRoles("ReadArg") => exRolesCons(exRole("name","id"),exRolesNil())
  rule exRoles("ReadObs") => exRolesCons(exRole("name","id"),exRolesNil())
  rule exRoles("ReadPre") => exRolesCons(exRole("view","view"),exRolesCons(exRole("field","id"),exRolesNil()))
  rule exRoles("ProjectField") => exRolesCons(exRole("record","expr"),exRolesCons(exRole("field","id"),exRolesNil()))
  rule exRoles("AccessField") => exRolesCons(exRole("record","expr"),exRolesCons(exRole("field","id"),exRolesNil()))
  rule exRoles("ProjectIndex") => exRolesCons(exRole("collection","expr"),exRolesCons(exRole("index","expr"),exRolesNil()))
  rule exRoles("AccessIndex") => exRolesCons(exRole("collection","expr"),exRolesCons(exRole("index","expr"),exRolesNil()))
  rule exRoles("ConstructRecord") => exRolesCons(exRole("recordType","id"),exRolesCons(exRole("fields","fields"),exRolesNil()))
  rule exRoles("ConstructEnum") => exRolesCons(exRole("enumType","id"),exRolesCons(exRole("member","id"),exRolesNil()))
  rule exRoles("ConstructSome") => exRolesCons(exRole("elementType","type"),exRolesCons(exRole("value","expr"),exRolesNil()))
  rule exRoles("ConstructNone") => exRolesCons(exRole("elementType","type"),exRolesNil())
  rule exRoles("ConstructCollection") => exRolesCons(exRole("elementType","type"),exRolesCons(exRole("capacity","int"),exRolesCons(exRole("items","list"),exRolesNil())))
  rule exRoles("Add") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Sub") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Mul") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("FloorDiv") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("CeilDiv") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Eq") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Lt") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Lte") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Gt") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Gte") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Not") => exRolesCons(exRole("value","expr"),exRolesNil())
  rule exRoles("And") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Or") => exRolesCons(exRole("left","expr"),exRolesCons(exRole("right","expr"),exRolesNil()))
  rule exRoles("Require") => exRolesCons(exRole("condition","expr"),exRolesNil())
  rule exRoles("Ensure") => exRolesCons(exRole("condition","expr"),exRolesNil())
  rule exRoles("Let") => exRolesCons(exRole("name","id"),exRolesCons(exRole("value","expr"),exRolesNil()))
  rule exRoles("NextWrite") => exRolesCons(exRole("field","id"),exRolesCons(exRole("value","expr"),exRolesNil()))
  rule exRoles("Emit") => exRolesCons(exRole("operation","id"),exRolesCons(exRole("fields","expr"),exRolesNil()))
  rule exRoles(_) => exRolesNil() [owise]
  syntax Bool ::= exConstructor(String) [function, total]
  rule exConstructor("LitUInt") => true
  rule exConstructor("LitSInt") => true
  rule exConstructor("LitBool") => true
  rule exConstructor("LitText") => true
  rule exConstructor("LitAmount") => true
  rule exConstructor("LitQuantity") => true
  rule exConstructor("LitShares") => true
  rule exConstructor("LitRate") => true
  rule exConstructor("LitPrice") => true
  rule exConstructor("ReadLocal") => true
  rule exConstructor("ReadArg") => true
  rule exConstructor("ReadObs") => true
  rule exConstructor("ReadPre") => true
  rule exConstructor("ProjectField") => true
  rule exConstructor("AccessField") => true
  rule exConstructor("ProjectIndex") => true
  rule exConstructor("AccessIndex") => true
  rule exConstructor("ConstructRecord") => true
  rule exConstructor("ConstructEnum") => true
  rule exConstructor("ConstructSome") => true
  rule exConstructor("ConstructNone") => true
  rule exConstructor("ConstructCollection") => true
  rule exConstructor("Add") => true
  rule exConstructor("Sub") => true
  rule exConstructor("Mul") => true
  rule exConstructor("FloorDiv") => true
  rule exConstructor("CeilDiv") => true
  rule exConstructor("Eq") => true
  rule exConstructor("Lt") => true
  rule exConstructor("Lte") => true
  rule exConstructor("Gt") => true
  rule exConstructor("Gte") => true
  rule exConstructor("Not") => true
  rule exConstructor("And") => true
  rule exConstructor("Or") => true
  rule exConstructor("Require") => true
  rule exConstructor("Ensure") => true
  rule exConstructor("Let") => true
  rule exConstructor("NextWrite") => true
  rule exConstructor("Emit") => true
  rule exConstructor(_) => false [owise]
  syntax EJList ::= exRoleNames(EXRoles) [function, total]
  rule exRoleNames(exRolesNil()) => ejNil()
  rule exRoleNames(exRolesCons(exRole(N,_),R)) => ejCons(ejString(N),exRoleNames(R))
  syntax EXSite ::= exGood() | exBad(String,EJSON,EJList)
  syntax EXSite ::= exThen(EXSite,EXSite) [function, total]
                  | exAtCode(String,EJSON,EJList) [function, total]
                  | exLocate(EXSite,EJSON,EJList) [function, total]
                  | exCoreShape(EJSON,Int) [function, total]
                  | exShapeNode(EJSON,EJList,Int,Int,Int) [function, total]
                  | exShapeFields(EJSON,EJList,Int,Int,Int) [function, total]
                  | exShapeNamed(EJSON,EJSON,EJList,Int,Int,Int) [function, total]
                  | exShapeOperands(EXRoles,EJSON,EJList,Int,Int,Int,Int) [function, total]
                  | exShapeList(EJList,EJList,Int,Int,Int,Int,Bool) [function, total]
                  | exShapeAction(EJList,Int,Int,Int) [function, total]
  rule exThen(exGood(),R) => R
  rule exThen(exBad(C,S,P),_) => exBad(C,S,P)
  rule exAtCode("",_,_) => exGood()
  rule exAtCode(C,S,P) => exBad(C,S,P) requires C =/=String ""
  rule exLocate(exBad(C,_,P),S,P) => exBad(C,S,P)
  rule exLocate(R,_,_) => R [owise]
  rule exCoreShape(N,B) => exActionShape(N,B) requires ejHas(N,"statements")
  rule exCoreShape(N,B) => exShapeNode(N,ejNil(),1,B,0) requires notBool ejHas(N,"statements")
  syntax EXSite ::= exActionShape(EJSON,Int) [function, total]
  rule exActionShape(N,B) => exThen(exAtCode(exFirst(exCheck(ejClosed(N,ejCons(ejString("statements"),ejCons(ejString("span"),ejNil()))),"INPUT_SCHEMA"),exFirst(exSpanCode(ejGet(N,"span"),B),exFirst(exCheck(ejIsArray(ejGet(N,"statements")),"INPUT_SCHEMA"),exCheck(exArrayLength(ejGet(N,"statements")) <=Int 256,"INPUT_BOUND")))),exSafeSpan(ejGet(N,"span"),B),ejNil()),exShapeAction(exArrayItems(ejGet(N,"statements")),B,0,0))
  rule exShapeAction(ejNil(),_,_,_) => exGood()
  rule exShapeAction(ejCons(N,L),B,I,Seen) => exThen(exShapeNode(N,ejCons(ejString(Int2String(I)),ejNil()),1,B,Seen),exShapeAction(L,B,I +Int 1,Seen +Int exASTSize(N)))
  rule exShapeNode(N,P,D,B,Seen) => exBad("INPUT_BOUND",exSafeSpan(ejGet(N,"span"),B),P) requires D >Int 64 orBool Seen >=Int 4096
  rule exShapeNode(N,P,D,B,Seen) => exThen(exAtCode(exCheck(ejClosed(N,ejCons(ejString("constructor"),ejCons(ejString("operands"),ejCons(ejString("span"),ejNil())))),"INPUT_SCHEMA"),exSafeSpan(ejGet(N,"span"),B),P),exShapeFields(N,P,D,B,Seen)) requires D <=Int 64 andBool Seen <Int 4096
  rule exShapeFields(N,P,D,B,Seen) => exThen(exAtCode(exSpanCode(ejGet(N,"span"),B),exSafeSpan(ejGet(N,"span"),B),P),exShapeNamed(ejGet(N,"constructor"),N,P,D,B,Seen))
  rule exShapeNamed(ejString(K),N,P,D,B,Seen) => exLocate(exThen(exAtCode(exCheck(exConstructor(K),"TYPE_CONSTRUCTOR"),exSynthetic(),P),exThen(exAtCode(exCheck(ejClosed(ejGet(N,"operands"),exRoleNames(exRoles(K))),"INPUT_SCHEMA"),exSynthetic(),P),exShapeOperands(exRoles(K),ejGet(N,"operands"),P,D,B,0,Seen +Int 1))),exSafeSpan(ejGet(N,"span"),B),P)
  rule exShapeNamed(_,N,P,_,B,_) => exBad("TYPE_CONSTRUCTOR",exSafeSpan(ejGet(N,"span"),B),P) [owise]
  rule exShapeOperands(exRolesNil(),_,_,_,_,_,_) => exGood()
  rule exShapeOperands(exRolesCons(exRole(N,"expr"),R),O,P,D,B,I,Seen) => exThen(exShapeNode(ejGet(O,N),ejAppend(P,ejCons(ejString(Int2String(I)),ejNil())),D +Int 1,B,Seen),exShapeOperands(R,O,P,D,B,I +Int 1,Seen +Int exASTSize(ejGet(O,N))))
  rule exShapeOperands(exRolesCons(exRole(N,K),R),O,P,D,B,I,Seen) => exThen(exAtCode(exMetaShape(K,ejGet(O,N)),exSynthetic(),P),exShapeOperands(R,O,P,D,B,I,Seen)) requires K =/=String "expr" andBool K =/=String "list" andBool K =/=String "fields"
  rule exShapeOperands(exRolesCons(exRole(N,K),R),O,P,D,B,I,Seen) => exThen(exAtCode(exFirst(exCheck(ejIsArray(ejGet(O,N)),"INPUT_SCHEMA"),exCheck(exArrayLength(ejGet(O,N)) <=Int exListLimit(K),"INPUT_BOUND")),exSynthetic(),P),exThen(exShapeList(exArrayItems(ejGet(O,N)),P,D,B,I,Seen,K ==String "fields"),exShapeOperands(R,O,P,D,B,I +Int exArrayLength(ejGet(O,N)),Seen +Int exASTListSize(exArrayItems(ejGet(O,N)),K ==String "fields")))) requires K ==String "list" orBool K ==String "fields"
  rule exShapeList(ejNil(),_,_,_,_,_,_) => exGood()
  rule exShapeList(ejCons(N,L),P,D,B,I,Seen,false) => exThen(exShapeNode(N,ejAppend(P,ejCons(ejString(Int2String(I)),ejNil())),D +Int 1,B,Seen),exShapeList(L,P,D,B,I +Int 1,Seen +Int exASTSize(N),false))
  rule exShapeList(ejCons(N,L),P,D,B,I,Seen,true) => exThen(exAtCode(exCheck(ejClosed(N,ejCons(ejString("name"),ejCons(ejString("value"),ejNil()))) andBool exIdentifier(ejGet(N,"name")),"INPUT_SCHEMA"),exSynthetic(),P),exThen(exShapeNode(ejGet(N,"value"),ejAppend(P,ejCons(ejString(Int2String(I)),ejNil())),D +Int 1,B,Seen),exShapeList(L,P,D,B,I +Int 1,Seen +Int exASTSize(ejGet(N,"value")),true)))
  syntax Int ::= exArrayLength(EJSON) [function, total]
               | exListLimit(String) [function, total]
               | exASTSize(EJSON) [function, total]
               | exASTRoles(EXRoles,EJSON) [function, total]
               | exASTListSize(EJList,Bool) [function, total]
  syntax EJList ::= exArrayItems(EJSON) [function, total]
  rule exArrayLength(ejArray(L)) => ejLength(L)
  rule exArrayLength(_) => 0 [owise]
  rule exArrayItems(ejArray(L)) => L
  rule exArrayItems(_) => ejNil() [owise]
  rule exListLimit("fields") => 64
  rule exListLimit(_) => 128 [owise]
  rule exASTSize(N) => 1 +Int exASTName(ejGet(N,"constructor"),ejGet(N,"operands"))
  syntax Int ::= exASTName(EJSON,EJSON) [function, total]
  rule exASTName(ejString(K),O) => exASTRoles(exRoles(K),O)
  rule exASTName(_,_) => 0 [owise]
  rule exASTRoles(exRolesNil(),_) => 0
  rule exASTRoles(exRolesCons(exRole(N,"expr"),R),O) => exASTSize(ejGet(O,N)) +Int exASTRoles(R,O)
  rule exASTRoles(exRolesCons(exRole(N,K),R),O) => exASTListSize(exArrayItems(ejGet(O,N)),K ==String "fields") +Int exASTRoles(R,O) requires K ==String "list" orBool K ==String "fields"
  rule exASTRoles(exRolesCons(exRole(_,K),R),O) => exASTRoles(R,O) requires K =/=String "expr" andBool K =/=String "list" andBool K =/=String "fields"
  rule exASTListSize(ejNil(),_) => 0
  rule exASTListSize(ejCons(N,L),false) => exASTSize(N) +Int exASTListSize(L,false)
  rule exASTListSize(ejCons(N,L),true) => exASTSize(ejGet(N,"value")) +Int exASTListSize(L,true)
  syntax String ::= exMetaShape(String,EJSON) [function, total]
                  | exSpanCode(EJSON,Int) [function, total]
  rule exMetaShape("id",V) => exCheck(exIdentifier(V),"INPUT_SCHEMA")
  rule exMetaShape("int",V) => exCheck(exDecimal(V),"INPUT_SCHEMA")
  rule exMetaShape("text",V) => exCheck(ejIsString(V),"INPUT_SCHEMA")
  rule exMetaShape("bool",V) => exCheck(V ==K ejBool(true) orBool V ==K ejBool(false),"INPUT_SCHEMA")
  rule exMetaShape("view",V) => exCheck(V ==K ejString("pre") orBool V ==K ejString("post") orBool V ==K ejString("next"),"INPUT_SCHEMA")
  rule exMetaShape("type",V) => exCheck(exTypeShape(V),"INPUT_SCHEMA")
  rule exMetaShape("units",V) => exCheck(exUnitShape(V),"INPUT_SCHEMA")
  rule exMetaShape(_,_) => "INPUT_SCHEMA" [owise]
  rule exSpanCode(S,B) => exFirst(exCheck(ejClosed(S,ejCons(ejString("kind"),ejCons(ejString("start"),ejCons(ejString("end"),ejNil())))) andBool exDecimal(ejGet(S,"start")) andBool exDecimal(ejGet(S,"end")) andBool (ejGet(S,"kind") ==K ejString("source") orBool ejGet(S,"kind") ==K ejString("synthetic")),"INPUT_SCHEMA"),exCheck(exSpan(S,B),"INPUT_SPAN"))
endmodule
