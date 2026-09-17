// Isolated Core/4 extension of the expression definition; legacy files are unchanged.
requires "lifecycle-types.k"
module MORIARTY-LIFECYCLE-EXPRESSION-SCHEMA
  imports MORIARTY-LIFECYCLE-EXPRESSION-TYPES
  syntax EJList ::= lxSchemaKeys() [function]
  rule lxSchemaKeys() => ejCons(ejString("variantTypes"),ejCons(ejString("units"),ejCons(ejString("assets"),ejCons(ejString("vaults"),ejCons(ejString("parties"),ejCons(ejString("recordTypes"),ejCons(ejString("enumTypes"),ejCons(ejString("fields"),ejCons(ejString("args"),ejCons(ejString("observations"),ejCons(ejString("operations"),ejNil())))))))))))
  syntax String ::= lxSchemaShape(EJSON) [function, total]
                  | lxSchemaMaps(EJSON) [function, total]
                  | lxRosterShape(EJSON) [function, total]
                  | lxNames(EJList) [function, total]
                  | lxMapShape(EJSON,String) [function, total]
                  | lxMapItems(EJList,String) [function, total]
                  | lxMapItem(EJSON,String) [function, total]
  rule lxSchemaShape(S) => lxFirst(lxCheck(ejClosed(S,lxSchemaKeys()),"INPUT_SCHEMA"),lxFirst(lxCheck(lxJsonBytes(S) <=Int 65536 andBool ejCount(S) <=Int 4096 andBool ejDepth(S) <=Int 64,"INPUT_BOUND"),lxSchemaMapNames(S)))
  rule lxSchemaMaps(S) => lxFirst(lxRosterShape(ejGet(S,"units")),lxFirst(lxRosterShape(ejGet(S,"assets")),lxFirst(lxRosterShape(ejGet(S,"vaults")),lxFirst(lxRosterShape(ejGet(S,"parties")),lxFirst(lxMapShape(ejGet(S,"recordTypes"),"record"),lxFirst(lxMapShape(ejGet(S,"variantTypes"),"variant"),lxFirst(lxMapShape(ejGet(S,"enumTypes"),"enum"),lxFirst(lxMapShape(ejGet(S,"fields"),"field"),lxFirst(lxMapShape(ejGet(S,"args"),"type"),lxFirst(lxMapShape(ejGet(S,"observations"),"type"),lxMapShape(ejGet(S,"operations"),"name")))))))))))
  rule lxRosterShape(ejArray(L)) => lxNames(L)
  rule lxRosterShape(_) => "INPUT_SCHEMA" [owise]
  rule lxNames(ejNil()) => ""
  rule lxNames(ejCons(V,L)) => lxFirst(lxCheck(lxIdentifier(V),"INPUT_SCHEMA"),lxNames(L))
  rule lxMapShape(ejObject(L),K) => lxMapItems(L,K)
  rule lxMapShape(_,_) => "INPUT_SCHEMA" [owise]
  rule lxMapItems(ejNil(),_) => ""
  rule lxMapItems(ejCons(ejPair(N,V),L),K) => lxFirst(lxCheck(lxIdentifier(ejString(N)),"INPUT_SCHEMA"),lxFirst(lxMapItem(V,K),lxMapItems(L,K)))
  rule lxMapItems(_,_) => "INPUT_SCHEMA" [owise]
  rule lxMapItem(ejObject(L),"record") => lxFirst(lxMapNamesOnly(ejObject(L)),lxFirst(lxCheck(ejLength(L) <=Int 64,"INPUT_BOUND"),lxMapItems(L,"type")))
  rule lxMapItem(ejArray(L),"enum") => lxFirst(lxCheck(ejLength(L) <=Int 128,"INPUT_BOUND"),lxNames(L))
  rule lxMapItem(V,"field") => lxFirst(lxCheck(ejClosed(V,ejCons(ejString("type"),ejCons(ejString("writeClass"),ejNil()))),"INPUT_SCHEMA"),lxFirst(lxCheck(lxTypeShape(ejGet(V,"type")),"INPUT_SCHEMA"),lxCheck(ejGet(V,"writeClass") ==K ejString("ordinary") orBool ejGet(V,"writeClass") ==K ejString("financial"),"INPUT_SCHEMA")))
  rule lxMapItem(V,"type") => lxCheck(lxTypeShape(V),"INPUT_SCHEMA")
  rule lxMapItem(V,"name") => lxCheck(lxIdentifier(V),"INPUT_SCHEMA")
  rule lxMapItem(_,_) => "INPUT_SCHEMA" [owise]
  syntax Int ::= lxDeclarations(EJSON,EJList) [function, total]
               | lxSize(EJSON) [function, total]
  rule lxSize(ejObject(L)) => ejLength(L)
  rule lxSize(ejArray(L)) => ejLength(L)
  rule lxSize(_) => 0 [owise]
  rule lxDeclarations(_,ejNil()) => 0
  rule lxDeclarations(S,ejCons(ejString(K),L)) => lxSize(ejGet(S,K)) +Int lxDeclarations(S,L)
  syntax String ::= lxSorted(EJSON) [function, total]
                  | lxSortedList(EJList,String) [function, total]
                  | lxSortedEnums(EJSON) [function, total]
                  | lxSortedEnumList(EJList) [function, total]
                  | lxSchemaDomain(EJSON) [function, total]
  rule lxSorted(ejArray(L)) => lxSortedList(L,"")
  rule lxSorted(_) => "INPUT_SCHEMA" [owise]
  rule lxSortedList(ejNil(),_) => ""
  rule lxSortedList(ejCons(ejString(N),L),P) => lxFirst(lxCheck(N >String P,"INPUT_SCHEMA"),lxSortedList(L,N))
  rule lxSortedList(_,_) => "INPUT_SCHEMA" [owise]
  rule lxSortedEnums(ejObject(L)) => lxSortedEnumList(L)
  rule lxSortedEnums(_) => "INPUT_SCHEMA" [owise]
  rule lxSortedEnumList(ejNil()) => ""
  rule lxSortedEnumList(ejCons(ejPair(_,V),L)) => lxFirst(lxSorted(V),lxSortedEnumList(L))
  rule lxSortedEnumList(_) => "INPUT_SCHEMA" [owise]
  rule lxSchemaDomain(S) => lxFirst(lxCheck(lxSize(ejGet(S,"recordTypes")) +Int lxSize(ejGet(S,"enumTypes")) +Int lxSize(ejGet(S,"variantTypes")) <=Int 256,"INPUT_BOUND"),lxFirst(lxSorted(ejGet(S,"units")),lxFirst(lxSorted(ejGet(S,"assets")),lxFirst(lxSorted(ejGet(S,"vaults")),lxFirst(lxSorted(ejGet(S,"parties")),lxFirst(lxSortedEnums(ejGet(S,"enumTypes")),lxFirst(lxDefinitions(S,lxDefinitionList(S)),lxSchemaCycles(S))))))))
  // Definition resolution order is the lexical name order from contract S0;
  // record field keys are qualified as Record.field before sorting.
  syntax EJList ::= lxDefinitionList(EJSON) [function, total]
                  | lxDefinitionsFrom(EJSON,String,String) [function, total]
                  | lxDefinitionsItems(EJList,String,String) [function, total]
                  | lxSortPairs(EJList) [function, total]
                  | lxInsertPair(EJSON,EJList) [function, total]
  rule lxDefinitionList(S) => lxSortPairs(ejAppend(lxDefinitionsFrom(ejGet(S,"variantTypes"),"records",""),ejAppend(lxDefinitionsFrom(ejGet(S,"recordTypes"),"records",""),ejAppend(lxDefinitionsFrom(ejGet(S,"operations"),"operations",""),ejAppend(lxDefinitionsFrom(ejGet(S,"args"),"types",""),ejAppend(lxDefinitionsFrom(ejGet(S,"observations"),"types",""),lxDefinitionsFrom(ejGet(S,"fields"),"fields","")))))))
  rule lxDefinitionsFrom(ejObject(L),K,P) => lxDefinitionsItems(L,K,P)
  rule lxDefinitionsFrom(_,_,_) => ejNil() [owise]
  rule lxDefinitionsItems(ejNil(),_,_) => ejNil()
  rule lxDefinitionsItems(ejCons(ejPair(N,V),L),"records",_) => ejAppend(lxDefinitionsFrom(V,"types",N +String "."),lxDefinitionsItems(L,"records",""))
  rule lxDefinitionsItems(ejCons(ejPair(N,V),L),"types",P) => ejCons(ejPair(P +String N,V),lxDefinitionsItems(L,"types",P))
  rule lxDefinitionsItems(ejCons(ejPair(N,V),L),"operations",_) => ejCons(ejPair(N,lxType("Record",V)),lxDefinitionsItems(L,"operations",""))
  rule lxDefinitionsItems(ejCons(ejPair(N,V),L),"fields",_) => ejCons(ejPair(N,ejGet(V,"type")),lxDefinitionsItems(L,"fields",""))
  rule lxDefinitionsItems(_,_,_) => ejNil() [owise]
  rule lxSortPairs(ejNil()) => ejNil()
  rule lxSortPairs(ejCons(V,L)) => lxInsertPair(V,lxSortPairs(L))
  rule lxInsertPair(V,ejNil()) => ejCons(V,ejNil())
  rule lxInsertPair(ejPair(A,V),ejCons(ejPair(B,W),L)) => ejCons(ejPair(A,V),ejCons(ejPair(B,W),L)) requires A <=String B
  rule lxInsertPair(ejPair(A,V),ejCons(ejPair(B,W),L)) => ejCons(ejPair(B,W),lxInsertPair(ejPair(A,V),L)) requires A >String B
  rule lxInsertPair(_,L) => L [owise]
  syntax String ::= lxDefinitions(EJSON,EJList) [function, total]
                  | lxSchemaCycles(EJSON) [function, total]
                  | lxCycleDefinitions(EJSON,EJList) [function, total]
                  | lxCycleType(EJSON,EJSON,EJList) [function, total]
                  | lxCycleNamed(EJSON,String,EJList) [function, total]
                  | lxCycleFields(EJSON,EJSON,EJList) [function, total]
                  | lxCycleFieldList(EJSON,EJList,EJList) [function, total]
  rule lxDefinitions(_,ejNil()) => ""
  rule lxDefinitions(S,ejCons(ejPair(_,T),L)) => lxFirst(lxResolve(S,T,"TYPE_LITERAL"),lxDefinitions(S,L))
  // Every type is resolved first. Finite DFS with a path set detects Record and
  // Operation cycles through arbitrary Option/Collection wrappers.
  rule lxSchemaCycles(S) => lxCycleDefinitions(S,lxDefinitionList(S))
  rule lxCycleDefinitions(_,ejNil()) => ""
  rule lxCycleDefinitions(S,ejCons(ejPair(_,T),L)) => lxFirst(lxCycleType(S,T,ejNil()),lxCycleDefinitions(S,L))
  rule lxCycleType(S,ejArray(ejCons(ejString("Option"),ejCons(T,ejNil()))),P) => lxCycleType(S,T,P)
  rule lxCycleType(S,ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(_,ejNil())))),P) => lxCycleType(S,T,P)
  rule lxCycleType(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),P) => lxCycleNamed(S,"R:" +String N,P)
  rule lxCycleType(S,ejArray(ejCons(ejString("Operation"),ejCons(ejString(N),ejNil()))),P) => lxCycleNamed(S,"O:" +String N,P)
  rule lxCycleType(_,_,_) => "" [owise]
  rule lxCycleNamed(_,N,P) => "TYPE_SCHEMA_CYCLE" requires ejMember(ejString(N),P)
  rule lxCycleNamed(S,N,P) => lxCycleFields(S,ejGet(ejGet(S,"recordTypes"),lxSubstring(N,2,lengthString(N))),ejCons(ejString(N),P)) requires notBool ejMember(ejString(N),P) andBool lxSubstring(N,0,2) ==String "R:"
  rule lxCycleNamed(S,N,P) => lxCycleType(S,lxType("Record",ejGet(ejGet(S,"operations"),lxSubstring(N,2,lengthString(N)))),ejCons(ejString(N),P)) requires notBool ejMember(ejString(N),P) andBool lxSubstring(N,0,2) ==String "O:"
  rule lxCycleNamed(_,_,_) => "TYPE_NAME" [owise]
  rule lxCycleFields(S,ejObject(L),P) => lxCycleFieldList(S,L,P)
  rule lxCycleFields(_,_,_) => "TYPE_NAME" [owise]
  rule lxCycleFieldList(_,ejNil(),_) => ""
  rule lxCycleFieldList(S,ejCons(ejPair(_,T),L),P) => lxFirst(lxCycleType(S,T,P),lxCycleFieldList(S,L,P))
  syntax String ::= lxSchemaMapNames(EJSON) [function, total]
                  | lxMapNamesOnly(EJSON) [function, total]
                  | lxPairNamesOnly(EJList) [function, total]
  rule lxSchemaMapNames(S) => lxFirst(lxRosterShape(ejGet(S,"units")),lxFirst(lxRosterShape(ejGet(S,"assets")),lxFirst(lxRosterShape(ejGet(S,"vaults")),lxFirst(lxRosterShape(ejGet(S,"parties")),lxFirst(lxMapNamesOnly(ejGet(S,"recordTypes")),lxFirst(lxMapNamesOnly(ejGet(S,"enumTypes")),lxFirst(lxMapNamesOnly(ejGet(S,"variantTypes")),lxFirst(lxMapNamesOnly(ejGet(S,"fields")),lxFirst(lxMapNamesOnly(ejGet(S,"args")),lxFirst(lxMapNamesOnly(ejGet(S,"observations")),lxFirst(lxMapNamesOnly(ejGet(S,"operations")),lxSchemaMaps(S))))))))))))
  rule lxMapNamesOnly(ejObject(L)) => lxPairNamesOnly(L)
  rule lxMapNamesOnly(_) => "INPUT_SCHEMA" [owise]
  rule lxPairNamesOnly(ejNil()) => ""
  rule lxPairNamesOnly(ejCons(ejPair(N,_),L)) => lxFirst(lxCheck(lxIdentifier(ejString(N)),"INPUT_SCHEMA"),lxPairNamesOnly(L))
  rule lxPairNamesOnly(_) => "INPUT_SCHEMA" [owise]
  rule lxMapItem(ejObject(L),"variant") => lxFirst(lxMapNamesOnly(ejObject(L)),lxFirst(lxCheck(ejLength(L) >Int 0 andBool ejLength(L) <=Int 128,"INPUT_BOUND"),lxFirst(lxMapItems(L,"type"),lxCheck(lcDistinctTypes(L),"INPUT_SCHEMA"))))
  syntax Bool ::= lcDistinctTypes(EJList) [function, total] | lcAbsentType(EJSON,EJList) [function, total]
  rule lcDistinctTypes(ejNil()) => true
  rule lcDistinctTypes(ejCons(ejPair(_,T),L)) => lcAbsentType(T,L) andBool lcDistinctTypes(L)
  rule lcAbsentType(_,ejNil()) => true
  rule lcAbsentType(T,ejCons(ejPair(_,V),L)) => T =/=K V andBool lcAbsentType(T,L)
  rule lxCycleType(S,ejArray(ejCons(ejString("Variant"),ejCons(ejString(N),ejNil()))),P) => lxCycleNamed(S,"V:" +String N,P)
  rule lxCycleNamed(S,N,P) => lxCycleFields(S,ejGet(ejGet(S,"variantTypes"),lxSubstring(N,2,lengthString(N))),ejCons(ejString(N),P)) requires notBool ejMember(ejString(N),P) andBool lxSubstring(N,0,2) ==String "V:"

endmodule
