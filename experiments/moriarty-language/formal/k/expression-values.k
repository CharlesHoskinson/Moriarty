requires "expression-schema.k"
module MORIARTY-EXPRESSION-VALUES
  imports MORIARTY-EXPRESSION-SCHEMA
  syntax String ::= exValueShape(EJSON,EJSON,EJSON) [function, total]
                  | exValueDomain(EJSON,EJSON,EJSON,String) [function, total]
                  | exValueBound(EJSON,EJSON,EJSON,String) [function, total]
                  | exChildShapes(EJSON,EJList) [function, total]
                  | exChildDomains(EJSON,EJList,String) [function, total]
  syntax EJList ::= exValueChildren(EJSON,EJSON,EJSON) [function, total]
                  | exRecordChildren(EJSON,EJSON) [function, total]
                  | exRecordChildList(EJList,EJSON) [function, total]
                  | exElementChildren(EJSON,EJList) [function, total]
  rule exValueChildren(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),V) => exRecordChildren(ejGet(ejGet(S,"recordTypes"),N),V)
  rule exValueChildren(_,ejArray(ejCons(ejString("Option"),ejCons(T,ejNil()))),ejArray(L)) => exElementChildren(T,L)
  rule exValueChildren(_,ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(_,ejNil())))),ejArray(L)) => exElementChildren(T,L)
  rule exValueChildren(S,ejArray(ejCons(ejString("Operation"),ejCons(ejString(N),ejNil()))),V) => ejCons(ejArray(ejCons(exType("Record",ejGet(ejGet(S,"operations"),N)),ejCons(ejGet(V,"fields"),ejNil()))),ejNil())
  rule exValueChildren(_,_,_) => ejNil() [owise]
  rule exRecordChildren(ejObject(L),V) => exRecordChildList(L,V)
  rule exRecordChildren(_,_) => ejNil() [owise]
  rule exRecordChildList(ejNil(),_) => ejNil()
  rule exRecordChildList(ejCons(ejPair(K,T),L),V) => ejCons(ejArray(ejCons(T,ejCons(ejGet(V,K),ejNil()))),exRecordChildList(L,V))
  rule exElementChildren(_,ejNil()) => ejNil()
  rule exElementChildren(T,ejCons(V,L)) => ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),exElementChildren(T,L))
  syntax EJList ::= exKeys(EJSON) [function, total]
                  | exKeyList(EJList) [function, total]
  rule exKeys(ejObject(L)) => exKeyList(L)
  rule exKeys(_) => ejNil() [owise]
  rule exKeyList(ejNil()) => ejNil()
  rule exKeyList(ejCons(ejPair(K,_),L)) => ejCons(ejString(K),exKeyList(L))
  rule exValueShape(S,T,V) => exFirst(exValueHeadShape(S,T,V),exChildShapes(S,exValueChildren(S,T,V)))
  syntax String ::= exValueHeadShape(EJSON,EJSON,EJSON) [function, total]
  rule exValueHeadShape(_,T,V) => exCheck(exDecimal(V),"INPUT_SCHEMA") requires exNumeric(T)
  rule exValueHeadShape(_,T,V) => exCheck(V ==K ejBool(true) orBool V ==K ejBool(false),"INPUT_SCHEMA") requires T ==K exType("Bool")
  rule exValueHeadShape(_,T,V) => exCheck(ejIsString(V),"INPUT_SCHEMA") requires ejAt(T,0) ==K ejString("Text") orBool ejAt(T,0) ==K ejString("Enum")
  rule exValueHeadShape(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),V) => exCheck(ejClosed(V,exKeys(ejGet(ejGet(S,"recordTypes"),N))),"INPUT_SCHEMA")
  rule exValueHeadShape(_,ejArray(ejCons(ejString("Option"),ejCons(_,ejNil()))),V) => exCheck(ejIsArray(V) andBool exSize(V) <=Int 1,"INPUT_SCHEMA")
  rule exValueHeadShape(_,ejArray(ejCons(ejString("Collection"),ejCons(_,ejCons(N,ejNil())))),V) => exCheck(ejIsArray(V) andBool exSize(V) <=Int exInt(N),"INPUT_SCHEMA")
  rule exValueHeadShape(_,ejArray(ejCons(ejString("Operation"),ejCons(N,ejNil()))),V) => exCheck(ejClosed(V,ejCons(ejString("operation"),ejCons(ejString("fields"),ejNil()))) andBool ejGet(V,"operation") ==K N,"INPUT_SCHEMA")
  rule exValueHeadShape(_,_,_) => "INPUT_SCHEMA" [owise]
  rule exChildShapes(_,ejNil()) => ""
  rule exChildShapes(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L)) => exFirst(exValueShape(S,T,V),exChildShapes(S,L))
  rule exValueDomain(S,T,V,C) => exFirst(exValueHeadDomain(S,T,V,C),exChildDomains(S,exValueChildren(S,T,V),C))
  syntax String ::= exValueHeadDomain(EJSON,EJSON,EJSON,String) [function, total]
  rule exValueHeadDomain(_,T,V,C) => exCheck(exFits(T,V),C) requires exNumeric(T)
  rule exValueHeadDomain(_,T,ejString(V),C) => exCheck(exUtf8(V) <=Int 1024,C) requires T ==K exType("Text")
  rule exValueHeadDomain(S,ejArray(ejCons(ejString("Enum"),ejCons(ejString(N),ejNil()))),V,C) => exCheck(exIn(V,ejGet(ejGet(S,"enumTypes"),N)),C)
  rule exValueHeadDomain(_,_,_,_) => "" [owise]
  rule exChildDomains(_,ejNil(),_) => ""
  rule exChildDomains(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L),C) => exFirst(exValueDomain(S,T,V,C),exChildDomains(S,L,C))
  syntax Int ::= exValueNodes(EJSON,EJSON,EJSON) [function, total]
               | exValueDepth(EJSON,EJSON,EJSON) [function, total]
               | exChildrenNodes(EJSON,EJList) [function, total]
               | exChildrenDepth(EJSON,EJList) [function, total]
  rule exValueNodes(S,T,V) => 1 +Int exChildrenNodes(S,exValueChildren(S,T,V))
  rule exValueDepth(S,T,V) => 1 +Int exChildrenDepth(S,exValueChildren(S,T,V))
  rule exChildrenNodes(_,ejNil()) => 0
  rule exChildrenNodes(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L)) => exValueNodes(S,T,V) +Int exChildrenNodes(S,L)
  rule exChildrenDepth(_,ejNil()) => 0
  rule exChildrenDepth(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L)) => maxInt(exValueDepth(S,T,V),exChildrenDepth(S,L))
  rule exValueBound(S,T,V,C) => exCheck(exValueNodes(S,T,V) <=Int 4096 andBool exValueDepth(S,T,V) <=Int 64 andBool exJsonBytes(ejObject(ejCons(ejPair("type",T),ejCons(ejPair("value",V),ejNil())))) <=Int 65536,C)
  syntax String ::= exSnapshot(EJSON,EJSON,EJSON) [function, total]
                  | exSnapshotBounds(EJSON,EJSON,EJSON,String) [function, total]
                  | exIndividualBounds(EJSON,EJList,String) [function, total]
  rule exSnapshot(S,Types,V) => exFirst(exCheck(ejClosed(V,exKeys(Types)),"INPUT_SCHEMA"),exFirst(exChildShapes(S,exRecordChildren(Types,V)),exFirst(exSnapshotBounds(S,Types,V,"INPUT_BOUND"),exChildDomains(S,exRecordChildren(Types,V),"INPUT_VALUE"))))
  rule exSnapshotBounds(S,T,V,C) => exFirst(exIndividualBounds(S,exRecordChildren(T,V),C),exCheck(1 +Int exChildrenNodes(S,exRecordChildren(T,V)) <=Int 4096 andBool 1 +Int exChildrenDepth(S,exRecordChildren(T,V)) <=Int 64 andBool exJsonBytes(V) <=Int 65536,C))
  rule exIndividualBounds(_,ejNil(),_) => ""
  rule exIndividualBounds(S,ejCons(ejArray(ejCons(T,ejCons(V,ejNil()))),L),C) => exFirst(exValueBound(S,T,V,C),exIndividualBounds(S,L,C))
endmodule
