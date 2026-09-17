// Isolated Core/4 extension of the expression definition; legacy files are unchanged.
requires "lifecycle-wire.k"
module MORIARTY-LIFECYCLE-EXPRESSION-TYPES
  imports MORIARTY-LIFECYCLE-EXPRESSION-WIRE
  syntax EJSON ::= lxType(String) [function]
                 | lxType(String,EJSON) [function]
                 | lxType(String,EJSON,EJSON) [function]
                 | lxType(String,EJSON,EJSON,EJSON) [function]
  rule lxType(K) => ejArray(ejCons(ejString(K),ejNil()))
  rule lxType(K,A) => ejArray(ejCons(ejString(K),ejCons(A,ejNil())))
  rule lxType(K,A,B) => ejArray(ejCons(ejString(K),ejCons(A,ejCons(B,ejNil()))))
  rule lxType(K,A,B,C) => ejArray(ejCons(ejString(K),ejCons(A,ejCons(B,ejCons(C,ejNil())))))
  syntax Bool ::= lxNumeric(EJSON) [function, total]
                | lxFits(EJSON,EJSON) [function, total]
                | lxTypeShape(EJSON) [function, total]
                | lxUnitShape(EJSON) [function, total]
                | lxUnitShapeList(EJList) [function, total]
  rule lxNumeric(T) => lcWideNumeric(T) orBool T ==K lxType("UInt256") orBool ejMember(ejAt(T,0),ejCons(ejString("UInt64"),ejCons(ejString("UInt128"),ejCons(ejString("SInt128"),ejCons(ejString("Amount"),ejCons(ejString("Shares"),ejCons(ejString("Rate"),ejCons(ejString("Price"),ejCons(ejString("Quantity"),ejNil())))))))))
  rule lxFits(T,V) => lxDecimal(V) andBool lxInt(V) >=Int (0 -Int (2 ^Int 127)) andBool lxInt(V) <Int (2 ^Int 127)
    requires ejAt(T,0) ==K ejString("SInt128") orBool ejAt(T,0) ==K ejString("Rate") orBool ejAt(T,0) ==K ejString("Quantity")
  rule lxFits(T,V) => lxDecimal(V) andBool lxInt(V) >=Int 0 andBool lxInt(V) <Int (2 ^Int 64) requires ejAt(T,0) ==K ejString("UInt64")
  rule lxFits(T,V) => lxDecimal(V) andBool lxInt(V) >=Int 0 andBool lxInt(V) <Int (2 ^Int 128) [owise]
  rule lxTypeShape(ejArray(ejCons(ejString(K),ejNil()))) => K ==String "UInt256" orBool K ==String "UInt64" orBool K ==String "UInt128" orBool K ==String "SInt128" orBool K ==String "Bool" orBool K ==String "Text" orBool K ==String "Unit"
  rule lxTypeShape(ejArray(ejCons(ejString(K),ejCons(N,ejNil())))) => lxIdentifier(N) requires K ==String "SignedAmount" orBool K ==String "NetAmount" orBool K ==String "Variant" orBool K ==String "Amount" orBool K ==String "Record" orBool K ==String "Enum" orBool K ==String "Operation"
  rule lxTypeShape(ejArray(ejCons(ejString("Option"),ejCons(T,ejNil())))) => lxTypeShape(T)
  rule lxTypeShape(ejArray(ejCons(ejString("Rate"),ejCons(S,ejNil())))) => lxDecimal(S)
  rule lxTypeShape(ejArray(ejCons(ejString("Shares"),ejCons(V,ejCons(H,ejNil()))))) => lxIdentifier(V) andBool lxIdentifier(H)
  rule lxTypeShape(ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(N,ejNil()))))) => lxTypeShape(T) andBool lxDecimal(N)
  rule lxTypeShape(ejArray(ejCons(ejString("Quantity"),ejCons(U,ejCons(S,ejNil()))))) => lxUnitShape(U) andBool lxDecimal(S)
  rule lxTypeShape(ejArray(ejCons(ejString("Price"),ejCons(A,ejCons(B,ejCons(S,ejNil())))))) => lxIdentifier(A) andBool lxIdentifier(B) andBool lxDecimal(S)
  rule lxTypeShape(_) => false [owise]
  rule lxUnitShape(ejArray(L)) => lxUnitShapeList(L)
  rule lxUnitShape(_) => false [owise]
  rule lxUnitShapeList(ejNil()) => true
  rule lxUnitShapeList(ejCons(ejArray(ejCons(N,ejCons(P,ejNil()))),L)) => lxIdentifier(N) andBool lxDecimal(P) andBool lxUnitShapeList(L)
  rule lxUnitShapeList(_) => false [owise]

  // Empty string is success; all failures remain closed contract codes.
  syntax String ::= lxFirst(String,String) [function, total]
                  | lxCheck(Bool,String) [function, total]
                  | lxResolve(EJSON,EJSON,String) [function, total]
                  | lxResolveBase(EJSON,EJSON,String) [function, total]
                  | lxUnits(EJSON,EJList,String,String) [function, total]
  rule lxFirst("",B) => B
  rule lxFirst(A,_) => A requires A =/=String ""
  rule lxCheck(true,_) => ""
  rule lxCheck(false,C) => C
  rule lxResolve(S,ejArray(ejCons(ejString("Option"),ejCons(T,ejNil()))),C) => lxResolve(S,T,C)
  rule lxResolve(S,ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(N,ejNil())))),C) => lxFirst(lxResolve(S,T,C),lxCheck(lxInt(N) >=Int 0 andBool lxInt(N) <=Int 128,"TYPE_COLLECTION_BOUND"))
  rule lxResolve(S,T,C) => lxResolveBase(S,T,C) [owise]
  rule lxResolveBase(_,T,_) => "" requires T ==K lxType("UInt256") orBool T ==K lxType("UInt64") orBool T ==K lxType("UInt128") orBool T ==K lxType("SInt128") orBool T ==K lxType("Bool") orBool T ==K lxType("Text")
  rule lxResolveBase(S,ejArray(ejCons(ejString("Amount"),ejCons(A,ejNil()))),_) => lxCheck(lxIn(A,ejGet(S,"assets")),"TYPE_NAME")
  rule lxResolveBase(S,ejArray(ejCons(ejString("Shares"),ejCons(V,ejCons(H,ejNil())))),_) => lxCheck(lxIn(V,ejGet(S,"vaults")) andBool lxIn(H,ejGet(S,"parties")),"TYPE_NAME")
  rule lxResolveBase(_,ejArray(ejCons(ejString("Rate"),ejCons(N,ejNil()))),C) => lxCheck(lxInt(N) >=Int 0 andBool lxInt(N) <=Int 18,C)
  rule lxResolveBase(S,ejArray(ejCons(ejString("Quantity"),ejCons(ejArray(U),ejCons(N,ejNil())))),C) => lxFirst(lxCheck(ejLength(U) <=Int 8,C),lxFirst(lxUnits(S,U,"",C),lxCheck(lxInt(N) >=Int 0 andBool lxInt(N) <=Int 18,C)))
  rule lxResolveBase(S,ejArray(ejCons(ejString("Price"),ejCons(A,ejCons(B,ejCons(N,ejNil()))))),C) => lxFirst(lxCheck(lxIn(A,ejGet(S,"assets")) andBool lxIn(B,ejGet(S,"assets")),"TYPE_NAME"),lxCheck(A =/=K B andBool lxInt(N) >=Int 0 andBool lxInt(N) <=Int 18,C))
  rule lxResolveBase(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),_) => lxCheck(ejHas(ejGet(S,"recordTypes"),N),"TYPE_NAME")
  rule lxResolveBase(S,ejArray(ejCons(ejString("Enum"),ejCons(ejString(N),ejNil()))),_) => lxCheck(ejHas(ejGet(S,"enumTypes"),N),"TYPE_NAME")
  rule lxResolveBase(S,ejArray(ejCons(ejString("Operation"),ejCons(ejString(N),ejNil()))),_) => lxCheck(ejHas(ejGet(S,"operations"),N),"TYPE_NAME")
  rule lxResolveBase(_,_,_) => "TYPE_NAME" [owise]
  rule lxUnits(_,ejNil(),_,_) => ""
  rule lxUnits(S,ejCons(ejArray(ejCons(ejString(N),ejCons(P,ejNil()))),L),Prev,C) => lxFirst(lxCheck(N >String Prev andBool lxInt(P) =/=Int 0 andBool lxInt(P) >=Int -128 andBool lxInt(P) <=Int 127,C),lxFirst(lxCheck(lxIn(ejString(N),ejGet(S,"units")),"TYPE_NAME"),lxUnits(S,L,N,C)))
  rule lxUnits(_,_,_,_) => "INPUT_SCHEMA" [owise]
  syntax Bool ::= lxIn(EJSON,EJSON) [function, total]
  rule lxIn(V,ejArray(L)) => ejMember(V,L)
  rule lxIn(_,_) => false [owise]
  rule lxFits(T,V) => lxDecimal(V) andBool lxInt(V) >=Int 0 andBool lxInt(V) <Int (2 ^Int 256) requires T ==K lxType("UInt256")
  rule lxResolveBase(S,ejArray(ejCons(ejString("Variant"),ejCons(ejString(N),ejNil()))),_) => lxCheck(ejHas(ejGet(S,"variantTypes"),N),"TYPE_NAME")
  syntax Bool ::= lcWideNumeric(EJSON) [function, total]
  rule lcWideNumeric(T) => ejAt(T,0) ==K ejString("AmountProduct") orBool ejAt(T,0) ==K ejString("ScaledAmount") orBool ejAt(T,0) ==K ejString("SignedScaledAmount") orBool ejAt(T,0) ==K ejString("SignedAmount") orBool ejAt(T,0) ==K ejString("NetAmount")
  rule lxTypeShape(ejArray(ejCons(ejString("AmountProduct"),ejCons(A,ejCons(B,ejNil()))))) => lxIdentifier(A) andBool lxIdentifier(B)
  rule lxTypeShape(ejArray(ejCons(ejString(K),ejCons(A,ejCons(N,ejNil()))))) => lxIdentifier(A) andBool lxDecimal(N) requires K ==String "ScaledAmount" orBool K ==String "SignedScaledAmount"
  rule lxResolveBase(S,ejArray(ejCons(ejString(K),ejCons(A,ejNil()))),_) => lxCheck(lxIn(A,ejGet(S,"assets")),"TYPE_NAME") requires K ==String "SignedAmount" orBool K ==String "NetAmount"
  rule lxResolveBase(S,ejArray(ejCons(ejString("AmountProduct"),ejCons(A,ejCons(B,ejNil())))),C) => lxFirst(lxCheck(lxIn(A,ejGet(S,"assets")) andBool lxIn(B,ejGet(S,"assets")),"TYPE_NAME"),lxCheck(lxText(A) <=String lxText(B),C))
  rule lxResolveBase(S,ejArray(ejCons(ejString(K),ejCons(A,ejCons(N,ejNil())))),C) => lxFirst(lxCheck(lxIn(A,ejGet(S,"assets")),"TYPE_NAME"),lxCheck(lxInt(N) >=Int 0 andBool lxInt(N) <=Int 18,C)) requires K ==String "ScaledAmount" orBool K ==String "SignedScaledAmount"
  rule lxFits(T,V) => lxDecimal(V) andBool lxInt(V) >=Int 0 andBool lxInt(V) <Int (2 ^Int 256) requires ejAt(T,0) ==K ejString("AmountProduct") orBool ejAt(T,0) ==K ejString("ScaledAmount")
  rule lxFits(T,V) => lxDecimal(V) andBool lxInt(V) >=Int (0 -Int (2 ^Int 255)) andBool lxInt(V) <Int (2 ^Int 255) requires ejAt(T,0) ==K ejString("SignedScaledAmount") orBool ejAt(T,0) ==K ejString("SignedAmount")
  rule lxFits(T,V) => lxDecimal(V) andBool lxInt(V) >=Int (1 -Int (2 ^Int 128)) andBool lxInt(V) <Int (2 ^Int 128) requires ejAt(T,0) ==K ejString("NetAmount")
  syntax Bool ::= lxHasName(EJSON,EJSON) [function, total]
  syntax EJSON ::= lxGetName(EJSON,EJSON) [function, total]
  rule lxHasName(M,ejString(N)) => ejHas(M,N)
  rule lxHasName(_,_) => false [owise]
  rule lxGetName(M,ejString(N)) => ejGet(M,N)
  rule lxGetName(_,_) => ejMissing() [owise]
  syntax String ::= lxText(EJSON) [function, total]
  rule lxText(ejString(S)) => S
  rule lxText(_) => "" [owise]
endmodule
