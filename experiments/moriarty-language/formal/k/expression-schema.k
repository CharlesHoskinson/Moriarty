requires "expression-types.k"
module MORIARTY-EXPRESSION-SCHEMA
  imports MORIARTY-EXPRESSION-TYPES
  syntax EJList ::= exSchemaKeys() [function]
  rule exSchemaKeys() => ejCons(ejString("units"),ejCons(ejString("assets"),ejCons(ejString("vaults"),ejCons(ejString("parties"),ejCons(ejString("recordTypes"),ejCons(ejString("enumTypes"),ejCons(ejString("fields"),ejCons(ejString("args"),ejCons(ejString("observations"),ejCons(ejString("operations"),ejNil()))))))))))
  syntax String ::= exSchemaShape(EJSON) [function, total]
                  | exSchemaMaps(EJSON) [function, total]
                  | exRosterShape(EJSON) [function, total]
                  | exNames(EJList) [function, total]
                  | exMapShape(EJSON,String) [function, total]
                  | exMapItems(EJList,String) [function, total]
                  | exMapItem(EJSON,String) [function, total]
  rule exSchemaShape(S) => exFirst(exCheck(ejClosed(S,exSchemaKeys()),"INPUT_SCHEMA"),exFirst(exCheck(exJsonBytes(S) <=Int 65536 andBool ejCount(S) <=Int 4096 andBool ejDepth(S) <=Int 64,"INPUT_BOUND"),exSchemaMapNames(S)))
  rule exSchemaMaps(S) => exFirst(exRosterShape(ejGet(S,"units")),exFirst(exRosterShape(ejGet(S,"assets")),exFirst(exRosterShape(ejGet(S,"vaults")),exFirst(exRosterShape(ejGet(S,"parties")),exFirst(exMapShape(ejGet(S,"recordTypes"),"record"),exFirst(exMapShape(ejGet(S,"enumTypes"),"enum"),exFirst(exMapShape(ejGet(S,"fields"),"field"),exFirst(exMapShape(ejGet(S,"args"),"type"),exFirst(exMapShape(ejGet(S,"observations"),"type"),exMapShape(ejGet(S,"operations"),"name"))))))))))
  rule exRosterShape(ejArray(L)) => exNames(L)
  rule exRosterShape(_) => "INPUT_SCHEMA" [owise]
  rule exNames(ejNil()) => ""
  rule exNames(ejCons(V,L)) => exFirst(exCheck(exIdentifier(V),"INPUT_SCHEMA"),exNames(L))
  rule exMapShape(ejObject(L),K) => exMapItems(L,K)
  rule exMapShape(_,_) => "INPUT_SCHEMA" [owise]
  rule exMapItems(ejNil(),_) => ""
  rule exMapItems(ejCons(ejPair(N,V),L),K) => exFirst(exCheck(exIdentifier(ejString(N)),"INPUT_SCHEMA"),exFirst(exMapItem(V,K),exMapItems(L,K)))
  rule exMapItems(_,_) => "INPUT_SCHEMA" [owise]
  rule exMapItem(ejObject(L),"record") => exFirst(exMapNamesOnly(ejObject(L)),exFirst(exCheck(ejLength(L) <=Int 64,"INPUT_BOUND"),exMapItems(L,"type")))
  rule exMapItem(ejArray(L),"enum") => exFirst(exCheck(ejLength(L) <=Int 128,"INPUT_BOUND"),exNames(L))
  rule exMapItem(V,"field") => exFirst(exCheck(ejClosed(V,ejCons(ejString("type"),ejCons(ejString("writeClass"),ejNil()))),"INPUT_SCHEMA"),exFirst(exCheck(exTypeShape(ejGet(V,"type")),"INPUT_SCHEMA"),exCheck(ejGet(V,"writeClass") ==K ejString("ordinary") orBool ejGet(V,"writeClass") ==K ejString("financial"),"INPUT_SCHEMA")))
  rule exMapItem(V,"type") => exCheck(exTypeShape(V),"INPUT_SCHEMA")
  rule exMapItem(V,"name") => exCheck(exIdentifier(V),"INPUT_SCHEMA")
  rule exMapItem(_,_) => "INPUT_SCHEMA" [owise]
  syntax Int ::= exDeclarations(EJSON,EJList) [function, total]
               | exSize(EJSON) [function, total]
  rule exSize(ejObject(L)) => ejLength(L)
  rule exSize(ejArray(L)) => ejLength(L)
  rule exSize(_) => 0 [owise]
  rule exDeclarations(_,ejNil()) => 0
  rule exDeclarations(S,ejCons(ejString(K),L)) => exSize(ejGet(S,K)) +Int exDeclarations(S,L)
  syntax String ::= exSorted(EJSON) [function, total]
                  | exSortedList(EJList,String) [function, total]
                  | exSortedEnums(EJSON) [function, total]
                  | exSortedEnumList(EJList) [function, total]
                  | exSchemaDomain(EJSON) [function, total]
  rule exSorted(ejArray(L)) => exSortedList(L,"")
  rule exSorted(_) => "INPUT_SCHEMA" [owise]
  rule exSortedList(ejNil(),_) => ""
  rule exSortedList(ejCons(ejString(N),L),P) => exFirst(exCheck(N >String P,"INPUT_SCHEMA"),exSortedList(L,N))
  rule exSortedList(_,_) => "INPUT_SCHEMA" [owise]
  rule exSortedEnums(ejObject(L)) => exSortedEnumList(L)
  rule exSortedEnums(_) => "INPUT_SCHEMA" [owise]
  rule exSortedEnumList(ejNil()) => ""
  rule exSortedEnumList(ejCons(ejPair(_,V),L)) => exFirst(exSorted(V),exSortedEnumList(L))
  rule exSortedEnumList(_) => "INPUT_SCHEMA" [owise]
  rule exSchemaDomain(S) => exFirst(exCheck(exDeclarations(S,exSchemaKeys()) <=Int 256,"INPUT_BOUND"),exFirst(exSorted(ejGet(S,"units")),exFirst(exSorted(ejGet(S,"assets")),exFirst(exSorted(ejGet(S,"vaults")),exFirst(exSorted(ejGet(S,"parties")),exFirst(exSortedEnums(ejGet(S,"enumTypes")),exFirst(exDefinitions(S,exDefinitionList(S)),exSchemaCycles(S))))))))
  // Definition resolution order is the lexical name order from contract S0;
  // record field keys are qualified as Record.field before sorting.
  syntax EJList ::= exDefinitionList(EJSON) [function, total]
                  | exDefinitionsFrom(EJSON,String,String) [function, total]
                  | exDefinitionsItems(EJList,String,String) [function, total]
                  | exSortPairs(EJList) [function, total]
                  | exInsertPair(EJSON,EJList) [function, total]
  rule exDefinitionList(S) => exSortPairs(ejAppend(exDefinitionsFrom(ejGet(S,"recordTypes"),"records",""),ejAppend(exDefinitionsFrom(ejGet(S,"operations"),"operations",""),ejAppend(exDefinitionsFrom(ejGet(S,"args"),"types",""),ejAppend(exDefinitionsFrom(ejGet(S,"observations"),"types",""),exDefinitionsFrom(ejGet(S,"fields"),"fields",""))))))
  rule exDefinitionsFrom(ejObject(L),K,P) => exDefinitionsItems(L,K,P)
  rule exDefinitionsFrom(_,_,_) => ejNil() [owise]
  rule exDefinitionsItems(ejNil(),_,_) => ejNil()
  rule exDefinitionsItems(ejCons(ejPair(N,V),L),"records",_) => ejAppend(exDefinitionsFrom(V,"types",N +String "."),exDefinitionsItems(L,"records",""))
  rule exDefinitionsItems(ejCons(ejPair(N,V),L),"types",P) => ejCons(ejPair(P +String N,V),exDefinitionsItems(L,"types",P))
  rule exDefinitionsItems(ejCons(ejPair(N,V),L),"operations",_) => ejCons(ejPair(N,exType("Record",V)),exDefinitionsItems(L,"operations",""))
  rule exDefinitionsItems(ejCons(ejPair(N,V),L),"fields",_) => ejCons(ejPair(N,ejGet(V,"type")),exDefinitionsItems(L,"fields",""))
  rule exDefinitionsItems(_,_,_) => ejNil() [owise]
  rule exSortPairs(ejNil()) => ejNil()
  rule exSortPairs(ejCons(V,L)) => exInsertPair(V,exSortPairs(L))
  rule exInsertPair(V,ejNil()) => ejCons(V,ejNil())
  rule exInsertPair(ejPair(A,V),ejCons(ejPair(B,W),L)) => ejCons(ejPair(A,V),ejCons(ejPair(B,W),L)) requires A <=String B
  rule exInsertPair(ejPair(A,V),ejCons(ejPair(B,W),L)) => ejCons(ejPair(B,W),exInsertPair(ejPair(A,V),L)) requires A >String B
  rule exInsertPair(_,L) => L [owise]
  syntax String ::= exDefinitions(EJSON,EJList) [function, total]
                  | exSchemaCycles(EJSON) [function, total]
                  | exCycleDefinitions(EJSON,EJList) [function, total]
                  | exCycleType(EJSON,EJSON,EJList) [function, total]
                  | exCycleNamed(EJSON,String,EJList) [function, total]
                  | exCycleFields(EJSON,EJSON,EJList) [function, total]
                  | exCycleFieldList(EJSON,EJList,EJList) [function, total]
  rule exDefinitions(_,ejNil()) => ""
  rule exDefinitions(S,ejCons(ejPair(_,T),L)) => exFirst(exResolve(S,T,"TYPE_LITERAL"),exDefinitions(S,L))
  // Every type is resolved first. Finite DFS with a path set detects Record and
  // Operation cycles through arbitrary Option/Collection wrappers.
  rule exSchemaCycles(S) => exCycleDefinitions(S,exDefinitionList(S))
  rule exCycleDefinitions(_,ejNil()) => ""
  rule exCycleDefinitions(S,ejCons(ejPair(_,T),L)) => exFirst(exCycleType(S,T,ejNil()),exCycleDefinitions(S,L))
  rule exCycleType(S,ejArray(ejCons(ejString("Option"),ejCons(T,ejNil()))),P) => exCycleType(S,T,P)
  rule exCycleType(S,ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(_,ejNil())))),P) => exCycleType(S,T,P)
  rule exCycleType(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),P) => exCycleNamed(S,"R:" +String N,P)
  rule exCycleType(S,ejArray(ejCons(ejString("Operation"),ejCons(ejString(N),ejNil()))),P) => exCycleNamed(S,"O:" +String N,P)
  rule exCycleType(_,_,_) => "" [owise]
  rule exCycleNamed(_,N,P) => "TYPE_SCHEMA_CYCLE" requires ejMember(ejString(N),P)
  rule exCycleNamed(S,N,P) => exCycleFields(S,ejGet(ejGet(S,"recordTypes"),exSubstring(N,2,lengthString(N))),ejCons(ejString(N),P)) requires notBool ejMember(ejString(N),P) andBool exSubstring(N,0,2) ==String "R:"
  rule exCycleNamed(S,N,P) => exCycleType(S,exType("Record",ejGet(ejGet(S,"operations"),exSubstring(N,2,lengthString(N)))),ejCons(ejString(N),P)) requires notBool ejMember(ejString(N),P) andBool exSubstring(N,0,2) ==String "O:"
  rule exCycleNamed(_,_,_) => "TYPE_NAME" [owise]
  rule exCycleFields(S,ejObject(L),P) => exCycleFieldList(S,L,P)
  rule exCycleFields(_,_,_) => "TYPE_NAME" [owise]
  rule exCycleFieldList(_,ejNil(),_) => ""
  rule exCycleFieldList(S,ejCons(ejPair(_,T),L),P) => exFirst(exCycleType(S,T,P),exCycleFieldList(S,L,P))
  syntax String ::= exSchemaMapNames(EJSON) [function, total]
                  | exMapNamesOnly(EJSON) [function, total]
                  | exPairNamesOnly(EJList) [function, total]
  rule exSchemaMapNames(S) => exFirst(exRosterShape(ejGet(S,"units")),exFirst(exRosterShape(ejGet(S,"assets")),exFirst(exRosterShape(ejGet(S,"vaults")),exFirst(exRosterShape(ejGet(S,"parties")),exFirst(exMapNamesOnly(ejGet(S,"recordTypes")),exFirst(exMapNamesOnly(ejGet(S,"enumTypes")),exFirst(exMapNamesOnly(ejGet(S,"fields")),exFirst(exMapNamesOnly(ejGet(S,"args")),exFirst(exMapNamesOnly(ejGet(S,"observations")),exFirst(exMapNamesOnly(ejGet(S,"operations")),exSchemaMaps(S)))))))))))
  rule exMapNamesOnly(ejObject(L)) => exPairNamesOnly(L)
  rule exMapNamesOnly(_) => "INPUT_SCHEMA" [owise]
  rule exPairNamesOnly(ejNil()) => ""
  rule exPairNamesOnly(ejCons(ejPair(N,_),L)) => exFirst(exCheck(exIdentifier(ejString(N)),"INPUT_SCHEMA"),exPairNamesOnly(L))
  rule exPairNamesOnly(_) => "INPUT_SCHEMA" [owise]
endmodule
