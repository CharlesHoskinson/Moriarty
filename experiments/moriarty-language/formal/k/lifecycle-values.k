// Isolated Core/4 extension of the expression definition; legacy files are unchanged.
requires "lifecycle-schema.k"
module MORIARTY-LIFECYCLE-EXPRESSION-VALUES
  imports MORIARTY-LIFECYCLE-EXPRESSION-SCHEMA
  syntax String ::= lxValueShape(EJSON,EJSON,EJSON) [function, total]
                  | lxValueDomain(EJSON,EJSON,EJSON,String) [function, total]
                  | lxValueBound(EJSON,EJSON,EJSON,String) [function, total]
                  | lxChildShapes(EJSON,EJList) [function, total]
                  | lxChildDomains(EJSON,EJList,String) [function, total]
  syntax EJList ::= lxValueChildren(EJSON,EJSON,EJSON) [function, total]
                  | lxRecordChildren(EJSON,EJSON) [function, total]
                  | lxRecordChildList(EJList,EJSON) [function, total]
                  | lxElementChildren(EJSON,EJList) [function, total]
  rule lxValueChildren(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),V) => lxRecordChildren(ejGet(ejGet(S,"recordTypes"),N),V)
  rule lxValueChildren(_,ejArray(ejCons(ejString("Option"),ejCons(T,ejNil()))),ejArray(L)) => lxElementChildren(T,L)
  rule lxValueChildren(_,ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(_,ejNil())))),ejArray(L)) => lxElementChildren(T,L)
  rule lxValueChildren(S,ejArray(ejCons(ejString("Operation"),ejCons(ejString(N),ejNil()))),V) => ejCons(ejArray(ejCons(lxType("Record",ejGet(ejGet(S,"operations"),N)),ejCons(ejGet(V,"fields"),ejNil()))),ejNil())
  rule lxValueChildren(_,_,_) => ejNil() [owise]
  rule lxRecordChildren(ejObject(L),V) => lxRecordChildList(L,V)
  rule lxRecordChildren(_,_) => ejNil() [owise]
  rule lxRecordChildList(ejNil(),_) => ejNil()
  rule lxRecordChildList(ejCons(ejPair(K,T),L),V) => ejCons(ejArray(ejCons(T,ejCons(ejGet(V,K),ejNil()))),lxRecordChildList(L,V))
  rule lxElementChildren(_,ejNil()) => ejNil()
  rule lxElementChildren(T,ejCons(V,L)) => ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),lxElementChildren(T,L))
  syntax EJList ::= lxKeys(EJSON) [function, total]
                  | lxKeyList(EJList) [function, total]
  rule lxKeys(ejObject(L)) => lxKeyList(L)
  rule lxKeys(_) => ejNil() [owise]
  rule lxKeyList(ejNil()) => ejNil()
  rule lxKeyList(ejCons(ejPair(K,_),L)) => ejCons(ejString(K),lxKeyList(L))
  rule lxValueShape(S,T,V) => lxFirst(lxValueHeadShape(S,T,V),lxChildShapes(S,lxValueChildren(S,T,V)))
  syntax String ::= lxValueHeadShape(EJSON,EJSON,EJSON) [function, total]
  rule lxValueHeadShape(_,T,V) => lxCheck(lxDecimal(V),"INPUT_SCHEMA") requires lxNumeric(T)
  rule lxValueHeadShape(_,T,V) => lxCheck(V ==K ejBool(true) orBool V ==K ejBool(false),"INPUT_SCHEMA") requires T ==K lxType("Bool")
  rule lxValueHeadShape(_,T,V) => lxCheck(ejIsString(V),"INPUT_SCHEMA") requires ejAt(T,0) ==K ejString("Text") orBool ejAt(T,0) ==K ejString("Enum")
  rule lxValueHeadShape(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),V) => lxCheck(ejClosed(V,lxKeys(ejGet(ejGet(S,"recordTypes"),N))),"INPUT_SCHEMA")
  rule lxValueHeadShape(_,ejArray(ejCons(ejString("Option"),ejCons(_,ejNil()))),V) => lxCheck(ejIsArray(V) andBool lxSize(V) <=Int 1,"INPUT_SCHEMA")
  rule lxValueHeadShape(_,ejArray(ejCons(ejString("Collection"),ejCons(_,ejCons(N,ejNil())))),V) => lxCheck(ejIsArray(V) andBool lxSize(V) <=Int lxInt(N),"INPUT_SCHEMA")
  rule lxValueHeadShape(_,ejArray(ejCons(ejString("Operation"),ejCons(N,ejNil()))),V) => lxCheck(ejClosed(V,ejCons(ejString("operation"),ejCons(ejString("fields"),ejNil()))) andBool ejGet(V,"operation") ==K N,"INPUT_SCHEMA")
  rule lxValueHeadShape(_,_,_) => "INPUT_SCHEMA" [owise]
  rule lxChildShapes(_,ejNil()) => ""
  rule lxChildShapes(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L)) => lxFirst(lxValueShape(S,T,V),lxChildShapes(S,L))
  rule lxValueDomain(S,T,V,C) => lxFirst(lxValueHeadDomain(S,T,V,C),lxChildDomains(S,lxValueChildren(S,T,V),C))
  syntax String ::= lxValueHeadDomain(EJSON,EJSON,EJSON,String) [function, total]
  rule lxValueHeadDomain(_,T,V,C) => lxCheck(lxFits(T,V),C) requires lxNumeric(T)
  rule lxValueHeadDomain(_,T,ejString(V),C) => lxCheck(lxUtf8(V) <=Int 1024,C) requires T ==K lxType("Text")
  rule lxValueHeadDomain(S,ejArray(ejCons(ejString("Enum"),ejCons(ejString(N),ejNil()))),V,C) => lxCheck(lxIn(V,ejGet(ejGet(S,"enumTypes"),N)),C)
  rule lxValueHeadDomain(_,_,_,_) => "" [owise]
  rule lxChildDomains(_,ejNil(),_) => ""
  rule lxChildDomains(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L),C) => lxFirst(lxValueDomain(S,T,V,C),lxChildDomains(S,L,C))
  syntax Int ::= lxValueNodes(EJSON,EJSON,EJSON) [function, total]
               | lxValueDepth(EJSON,EJSON,EJSON) [function, total]
               | lxChildrenNodes(EJSON,EJList) [function, total]
               | lxChildrenDepth(EJSON,EJList) [function, total]
  rule lxValueNodes(S,T,V) => 1 +Int lxChildrenNodes(S,lxValueChildren(S,T,V))
  rule lxValueDepth(S,T,V) => 1 +Int lxChildrenDepth(S,lxValueChildren(S,T,V))
  rule lxChildrenNodes(_,ejNil()) => 0
  rule lxChildrenNodes(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L)) => lxValueNodes(S,T,V) +Int lxChildrenNodes(S,L)
  rule lxChildrenDepth(_,ejNil()) => 0
  rule lxChildrenDepth(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L)) => maxInt(lxValueDepth(S,T,V),lxChildrenDepth(S,L))
  rule lxValueBound(S,T,V,C) => lxCheck(lxValueNodes(S,T,V) <=Int 4096 andBool lxValueDepth(S,T,V) <=Int 64 andBool lxJsonBytes(ejObject(ejCons(ejPair("type",T),ejCons(ejPair("value",V),ejNil())))) <=Int 65536,C)
  syntax String ::= lxSnapshot(EJSON,EJSON,EJSON) [function, total]
                  | lxSnapshotBounds(EJSON,EJSON,EJSON,String) [function, total]
                  | lxIndividualBounds(EJSON,EJList,String) [function, total]
  rule lxSnapshot(S,Types,V) => lxFirst(lxCheck(ejClosed(V,lxKeys(Types)),"INPUT_SCHEMA"),lxFirst(lxChildShapes(S,lxRecordChildren(Types,V)),lxFirst(lxSnapshotBounds(S,Types,V,"INPUT_BOUND"),lxChildDomains(S,lxRecordChildren(Types,V),"INPUT_VALUE"))))
  rule lxSnapshotBounds(S,T,V,C) => lxFirst(lxIndividualBounds(S,lxRecordChildren(T,V),C),lxCheck(1 +Int lxChildrenNodes(S,lxRecordChildren(T,V)) <=Int 4096 andBool 1 +Int lxChildrenDepth(S,lxRecordChildren(T,V)) <=Int 64 andBool lxJsonBytes(V) <=Int 65536,C))
  rule lxIndividualBounds(_,ejNil(),_) => ""
  rule lxIndividualBounds(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L),C) => lxFirst(lxValueBound(S,T,V,C),lxIndividualBounds(S,L,C))
  rule lxValueChildren(S,ejArray(ejCons(ejString("Variant"),ejCons(N,ejNil()))),V) => ejCons(ejArray(ejCons(lxGetName(lxGetName(ejGet(S,"variantTypes"),N),ejGet(V,"tag")),ejCons(ejGet(V,"value"),ejNil()))),ejNil())
  rule lxValueHeadShape(S,ejArray(ejCons(ejString("Variant"),ejCons(N,ejNil()))),V) => lxCheck(ejClosed(V,ejCons(ejString("tag"),ejCons(ejString("value"),ejNil()))) andBool ejIsString(ejGet(V,"tag")) andBool lxHasName(lxGetName(ejGet(S,"variantTypes"),N),ejGet(V,"tag")),"INPUT_SCHEMA")
endmodule
