// Structural JSON terms. No checked/type/evaluation metadata is accepted from the codec.
module MORIARTY-EXPRESSION-JSON
  imports INT
  imports BOOL
  imports STRING
  syntax EJSON ::= ejNull() [symbol(ejNull)]
                 | ejBool(Bool) [symbol(ejBool)]
                 | ejNumber(Int) [symbol(ejNumber)]
                 | ejString(String) [symbol(ejString)]
                 | ejArray(EJList) [symbol(ejArray)]
                 | ejObject(EJList) [symbol(ejObject)]
                 | ejPair(String, EJSON) [symbol(ejPair)]
                 | ejMissing() [symbol(ejMissing)]
  syntax EJList ::= ejNil() [symbol(ejNil)]
                  | ejCons(EJSON, EJList) [symbol(ejCons)]
  syntax ERequest ::= expressionRequest(EJSON, EJSON) [symbol(expressionRequest)]
  syntax EJSON ::= ejGet(EJSON, String) [function, total]
                 | ejFind(EJList, String) [function, total]
                 | ejAt(EJSON, Int) [function, total]
                 | ejNth(EJList, Int) [function, total]
  rule ejGet(ejObject(L), K) => ejFind(L, K)
  rule ejGet(_, _) => ejMissing() [owise]
  rule ejFind(ejNil(), _) => ejMissing()
  rule ejFind(ejCons(ejPair(K,V),_), K) => V
  rule ejFind(ejCons(_,L), K) => ejFind(L,K) [owise]
  rule ejAt(ejArray(L), I) => ejNth(L,I)
  rule ejAt(_,_) => ejMissing() [owise]
  rule ejNth(ejCons(V,_),0) => V
  rule ejNth(ejCons(_,L),I) => ejNth(L,I -Int 1) requires I >Int 0
  rule ejNth(_,_) => ejMissing() [owise]
  syntax Int ::= ejLength(EJList) [function, total]
               | ejCount(EJSON) [function, total]
               | ejCountList(EJList) [function, total]
               | ejDepth(EJSON) [function, total]
               | ejDepthList(EJList) [function, total]
  rule ejLength(ejNil()) => 0
  rule ejLength(ejCons(_,L)) => 1 +Int ejLength(L)
  // Object keys are not extra JSON-tree value nodes.
  rule ejCount(ejArray(L)) => 1 +Int ejCountList(L)
  rule ejCount(ejObject(L)) => 1 +Int ejCountList(L)
  rule ejCount(ejPair(_,V)) => ejCount(V)
  rule ejCount(_) => 1 [owise]
  rule ejCountList(ejNil()) => 0
  rule ejCountList(ejCons(V,L)) => ejCount(V) +Int ejCountList(L)
  rule ejDepth(ejArray(L)) => 1 +Int ejDepthList(L)
  rule ejDepth(ejObject(L)) => 1 +Int ejDepthList(L)
  rule ejDepth(ejPair(_,V)) => ejDepth(V)
  rule ejDepth(_) => 1 [owise]
  rule ejDepthList(ejNil()) => 0
  rule ejDepthList(ejCons(V,L)) => maxInt(ejDepth(V),ejDepthList(L))
  syntax Bool ::= ejHas(EJSON,String) [function, total]
                | ejMember(EJSON,EJList) [function, total]
                | ejClosed(EJSON,EJList) [function, total]
                | ejKeysMatch(EJList,EJList) [function, total]
                | ejIsString(EJSON) [function, total]
                | ejIsObject(EJSON) [function, total]
                | ejIsArray(EJSON) [function, total]
  rule ejHas(V,K) => ejGet(V,K) =/=K ejMissing()
  rule ejMember(V,ejCons(V,_)) => true
  rule ejMember(V,ejCons(_,L)) => ejMember(V,L) [owise]
  rule ejMember(_,ejNil()) => false
  rule ejClosed(ejObject(L),Keys) => ejLength(L) ==Int ejLength(Keys) andBool ejKeysMatch(L,Keys)
  rule ejClosed(_,_) => false [owise]
  rule ejKeysMatch(ejNil(),_) => true
  rule ejKeysMatch(ejCons(ejPair(K,_),L),Keys) => ejMember(ejString(K),Keys) andBool ejKeysMatch(L,Keys)
  rule ejKeysMatch(_,_) => false [owise]
  rule ejIsString(ejString(_)) => true
  rule ejIsString(_) => false [owise]
  rule ejIsObject(ejObject(_)) => true
  rule ejIsObject(_) => false [owise]
  rule ejIsArray(ejArray(_)) => true
  rule ejIsArray(_) => false [owise]
  syntax EJList ::= ejAppend(EJList,EJList) [function, total]
  rule ejAppend(ejNil(),R) => R
  rule ejAppend(ejCons(V,L),R) => ejCons(V,ejAppend(L,R))
  syntax EJSON ::= ejPut(EJSON,String,EJSON) [function, total]
  syntax EJList ::= ejPutPair(EJList,String,EJSON) [function, total]
  rule ejPut(ejObject(L),K,V) => ejObject(ejPutPair(L,K,V))
  rule ejPut(_,_,_) => ejMissing() [owise]
  rule ejPutPair(ejNil(),K,V) => ejCons(ejPair(K,V),ejNil())
  rule ejPutPair(ejCons(ejPair(K,_),L),K,V) => ejCons(ejPair(K,V),L)
  rule ejPutPair(ejCons(ejPair(A,X),L),K,V) => ejCons(ejPair(K,V),ejCons(ejPair(A,X),L)) requires K <String A
  rule ejPutPair(ejCons(ejPair(A,X),L),K,V) => ejCons(ejPair(A,X),ejPutPair(L,K,V)) requires K >String A
  rule ejPutPair(ejCons(X,L),K,V) => ejCons(X,ejPutPair(L,K,V)) [owise]
endmodule
