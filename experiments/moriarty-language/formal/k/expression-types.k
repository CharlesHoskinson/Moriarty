requires "expression-wire.k"
module MORIARTY-EXPRESSION-TYPES
  imports MORIARTY-EXPRESSION-WIRE
  syntax EJSON ::= exType(String) [function]
                 | exType(String,EJSON) [function]
                 | exType(String,EJSON,EJSON) [function]
                 | exType(String,EJSON,EJSON,EJSON) [function]
  rule exType(K) => ejArray(ejCons(ejString(K),ejNil()))
  rule exType(K,A) => ejArray(ejCons(ejString(K),ejCons(A,ejNil())))
  rule exType(K,A,B) => ejArray(ejCons(ejString(K),ejCons(A,ejCons(B,ejNil()))))
  rule exType(K,A,B,C) => ejArray(ejCons(ejString(K),ejCons(A,ejCons(B,ejCons(C,ejNil())))))
  syntax Bool ::= exNumeric(EJSON) [function, total]
                | exFits(EJSON,EJSON) [function, total]
                | exTypeShape(EJSON) [function, total]
                | exUnitShape(EJSON) [function, total]
                | exUnitShapeList(EJList) [function, total]
  rule exNumeric(T) => ejMember(ejAt(T,0),ejCons(ejString("UInt64"),ejCons(ejString("UInt128"),ejCons(ejString("SInt128"),ejCons(ejString("Amount"),ejCons(ejString("Shares"),ejCons(ejString("Rate"),ejCons(ejString("Price"),ejCons(ejString("Quantity"),ejNil())))))))))
  rule exFits(T,V) => exDecimal(V) andBool exInt(V) >=Int (0 -Int (2 ^Int 127)) andBool exInt(V) <Int (2 ^Int 127)
    requires ejAt(T,0) ==K ejString("SInt128") orBool ejAt(T,0) ==K ejString("Rate") orBool ejAt(T,0) ==K ejString("Quantity")
  rule exFits(T,V) => exDecimal(V) andBool exInt(V) >=Int 0 andBool exInt(V) <Int (2 ^Int 64) requires ejAt(T,0) ==K ejString("UInt64")
  rule exFits(T,V) => exDecimal(V) andBool exInt(V) >=Int 0 andBool exInt(V) <Int (2 ^Int 128) [owise]
  rule exTypeShape(ejArray(ejCons(ejString(K),ejNil()))) => K ==String "UInt64" orBool K ==String "UInt128" orBool K ==String "SInt128" orBool K ==String "Bool" orBool K ==String "Text" orBool K ==String "Unit"
  rule exTypeShape(ejArray(ejCons(ejString(K),ejCons(N,ejNil())))) => exIdentifier(N) requires K ==String "Amount" orBool K ==String "Record" orBool K ==String "Enum" orBool K ==String "Operation"
  rule exTypeShape(ejArray(ejCons(ejString("Option"),ejCons(T,ejNil())))) => exTypeShape(T)
  rule exTypeShape(ejArray(ejCons(ejString("Rate"),ejCons(S,ejNil())))) => exDecimal(S)
  rule exTypeShape(ejArray(ejCons(ejString("Shares"),ejCons(V,ejCons(H,ejNil()))))) => exIdentifier(V) andBool exIdentifier(H)
  rule exTypeShape(ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(N,ejNil()))))) => exTypeShape(T) andBool exDecimal(N)
  rule exTypeShape(ejArray(ejCons(ejString("Quantity"),ejCons(U,ejCons(S,ejNil()))))) => exUnitShape(U) andBool exDecimal(S)
  rule exTypeShape(ejArray(ejCons(ejString("Price"),ejCons(A,ejCons(B,ejCons(S,ejNil())))))) => exIdentifier(A) andBool exIdentifier(B) andBool exDecimal(S)
  rule exTypeShape(_) => false [owise]
  rule exUnitShape(ejArray(L)) => exUnitShapeList(L)
  rule exUnitShape(_) => false [owise]
  rule exUnitShapeList(ejNil()) => true
  rule exUnitShapeList(ejCons(ejArray(ejCons(N,ejCons(P,ejNil()))),L)) => exIdentifier(N) andBool exDecimal(P) andBool exUnitShapeList(L)
  rule exUnitShapeList(_) => false [owise]

  // Empty string is success; all failures remain closed contract codes.
  syntax String ::= exFirst(String,String) [function, total]
                  | exCheck(Bool,String) [function, total]
                  | exResolve(EJSON,EJSON,String) [function, total]
                  | exResolveBase(EJSON,EJSON,String) [function, total]
                  | exUnits(EJSON,EJList,String,String) [function, total]
  rule exFirst("",B) => B
  rule exFirst(A,_) => A requires A =/=String ""
  rule exCheck(true,_) => ""
  rule exCheck(false,C) => C
  rule exResolve(S,ejArray(ejCons(ejString("Option"),ejCons(T,ejNil()))),C) => exResolve(S,T,C)
  rule exResolve(S,ejArray(ejCons(ejString("Collection"),ejCons(T,ejCons(N,ejNil())))),C) => exFirst(exResolve(S,T,C),exCheck(exInt(N) >=Int 0 andBool exInt(N) <=Int 128,"TYPE_COLLECTION_BOUND"))
  rule exResolve(S,T,C) => exResolveBase(S,T,C) [owise]
  rule exResolveBase(_,T,_) => "" requires T ==K exType("UInt64") orBool T ==K exType("UInt128") orBool T ==K exType("SInt128") orBool T ==K exType("Bool") orBool T ==K exType("Text")
  rule exResolveBase(S,ejArray(ejCons(ejString("Amount"),ejCons(A,ejNil()))),_) => exCheck(exIn(A,ejGet(S,"assets")),"TYPE_NAME")
  rule exResolveBase(S,ejArray(ejCons(ejString("Shares"),ejCons(V,ejCons(H,ejNil())))),_) => exCheck(exIn(V,ejGet(S,"vaults")) andBool exIn(H,ejGet(S,"parties")),"TYPE_NAME")
  rule exResolveBase(_,ejArray(ejCons(ejString("Rate"),ejCons(N,ejNil()))),C) => exCheck(exInt(N) >=Int 0 andBool exInt(N) <=Int 18,C)
  rule exResolveBase(S,ejArray(ejCons(ejString("Quantity"),ejCons(ejArray(U),ejCons(N,ejNil())))),C) => exFirst(exCheck(ejLength(U) <=Int 8,C),exFirst(exUnits(S,U,"",C),exCheck(exInt(N) >=Int 0 andBool exInt(N) <=Int 18,C)))
  rule exResolveBase(S,ejArray(ejCons(ejString("Price"),ejCons(A,ejCons(B,ejCons(N,ejNil()))))),C) => exFirst(exCheck(exIn(A,ejGet(S,"assets")) andBool exIn(B,ejGet(S,"assets")),"TYPE_NAME"),exCheck(A =/=K B andBool exInt(N) >=Int 0 andBool exInt(N) <=Int 18,C))
  rule exResolveBase(S,ejArray(ejCons(ejString("Record"),ejCons(ejString(N),ejNil()))),_) => exCheck(ejHas(ejGet(S,"recordTypes"),N),"TYPE_NAME")
  rule exResolveBase(S,ejArray(ejCons(ejString("Enum"),ejCons(ejString(N),ejNil()))),_) => exCheck(ejHas(ejGet(S,"enumTypes"),N),"TYPE_NAME")
  rule exResolveBase(S,ejArray(ejCons(ejString("Operation"),ejCons(ejString(N),ejNil()))),_) => exCheck(ejHas(ejGet(S,"operations"),N),"TYPE_NAME")
  rule exResolveBase(_,_,_) => "TYPE_NAME" [owise]
  rule exUnits(_,ejNil(),_,_) => ""
  rule exUnits(S,ejCons(ejArray(ejCons(ejString(N),ejCons(P,ejNil()))),L),Prev,C) => exFirst(exCheck(N >String Prev andBool exInt(P) =/=Int 0 andBool exInt(P) >=Int -128 andBool exInt(P) <=Int 127,C),exFirst(exCheck(exIn(ejString(N),ejGet(S,"units")),"TYPE_NAME"),exUnits(S,L,N,C)))
  rule exUnits(_,_,_,_) => "INPUT_SCHEMA" [owise]
  syntax Bool ::= exIn(EJSON,EJSON) [function, total]
  rule exIn(V,ejArray(L)) => ejMember(V,L)
  rule exIn(_,_) => false [owise]
endmodule
