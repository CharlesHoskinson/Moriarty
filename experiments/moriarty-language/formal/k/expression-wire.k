requires "expression-json.k"
module MORIARTY-EXPRESSION-WIRE
  imports MORIARTY-EXPRESSION-JSON
  // Pinned domains.md:1751-1758 specifies substr only for start<end<=length.
  // All callers use this explicit total helper, including empty one-character tails.
  syntax String ::= exSubstring(String,Int,Int) [function, total]
  rule exSubstring(S,A,B) => substrString(S,A,B) requires A >=Int 0 andBool A <Int B andBool B <=Int lengthString(S)
  rule exSubstring(_,A,B) => "" requires A >=Int B
  rule exSubstring(S,A,B) => "" requires A <Int 0 orBool B >Int lengthString(S)
  syntax Bool ::= exAlpha(String) [function, total]
                | exDigit(String) [function, total]
                | exIdentifier(EJSON) [function, total]
                | exIdentifierTail(String) [function, total]
                | exDecimal(EJSON) [function, total]
                | exDigits(String) [function, total]
                | exNatural(EJSON) [function, total]
  rule exAlpha(C) => (C >=String "A" andBool C <=String "Z") orBool (C >=String "a" andBool C <=String "z") requires lengthString(C) ==Int 1
  rule exAlpha(_) => false [owise]
  rule exDigit(C) => C >=String "0" andBool C <=String "9" requires lengthString(C) ==Int 1
  rule exDigit(_) => false [owise]
  rule exIdentifier(ejString(S)) => lengthString(S) >=Int 1 andBool lengthString(S) <=Int 64 andBool exAlpha(exSubstring(S,0,1)) andBool exIdentifierTail(exSubstring(S,1,lengthString(S)))
  rule exIdentifier(_) => false [owise]
  rule exIdentifierTail("") => true
  rule exIdentifierTail(S) => (exAlpha(exSubstring(S,0,1)) orBool exDigit(exSubstring(S,0,1)) orBool exSubstring(S,0,1) ==String "_") andBool exIdentifierTail(exSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule exDigits("") => true
  rule exDigits(S) => exDigit(exSubstring(S,0,1)) andBool exDigits(exSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule exNatural(ejString("0")) => true
  rule exNatural(ejString(S)) => lengthString(S) >Int 0 andBool exSubstring(S,0,1) >=String "1" andBool exSubstring(S,0,1) <=String "9" andBool exDigits(exSubstring(S,1,lengthString(S))) requires S =/=String "0"
  rule exNatural(_) => false [owise]
  rule exDecimal(ejString(S)) => exNatural(ejString(S)) orBool (lengthString(S) >Int 1 andBool exSubstring(S,0,1) ==String "-" andBool exSubstring(S,1,2) =/=String "0" andBool exNatural(ejString(exSubstring(S,1,lengthString(S)))))
  rule exDecimal(_) => false [owise]
  syntax Int ::= exInt(EJSON) [function, total]
               | exUtf8(String) [function, total]
               | exCharBytes(Int) [function, total]
               | exQuotedBytes(String) [function, total]
               | exEscapedBytes(Int) [function, total]
               | exJsonBytes(EJSON) [function, total]
               | exListBytes(EJList) [function, total]
  rule exInt(ejString(S)) => String2Int(S) requires exDecimal(ejString(S))
  rule exInt(_) => 0 [owise]
  rule exCharBytes(N) => 1 requires N <Int 128
  rule exCharBytes(N) => 2 requires N >=Int 128 andBool N <Int 2048
  rule exCharBytes(N) => 3 requires N >=Int 2048 andBool N <Int 65536
  rule exCharBytes(N) => 4 requires N >=Int 65536
  rule exUtf8("") => 0
  rule exUtf8(S) => exCharBytes(ordChar(exSubstring(S,0,1))) +Int exUtf8(exSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule exEscapedBytes(N) => 2 requires N ==Int 34 orBool N ==Int 92 orBool N ==Int 8 orBool N ==Int 9 orBool N ==Int 10 orBool N ==Int 12 orBool N ==Int 13
  rule exEscapedBytes(N) => 6 requires N <Int 32 andBool N =/=Int 8 andBool N =/=Int 9 andBool N =/=Int 10 andBool N =/=Int 12 andBool N =/=Int 13
  rule exEscapedBytes(N) => exCharBytes(N) requires N >=Int 32 andBool N =/=Int 34 andBool N =/=Int 92
  rule exQuotedBytes("") => 2
  rule exQuotedBytes(S) => exEscapedBytes(ordChar(exSubstring(S,0,1))) +Int exQuotedBytes(exSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule exJsonBytes(ejString(S)) => exQuotedBytes(S)
  rule exJsonBytes(ejBool(true)) => 4
  rule exJsonBytes(ejBool(false)) => 5
  rule exJsonBytes(ejNull()) => 4
  rule exJsonBytes(ejNumber(N)) => lengthString(Int2String(N))
  rule exJsonBytes(ejArray(L)) => 2 +Int exListBytes(L)
  rule exJsonBytes(ejObject(L)) => 2 +Int exListBytes(L)
  rule exJsonBytes(ejPair(K,V)) => exQuotedBytes(K) +Int 1 +Int exJsonBytes(V)
  rule exJsonBytes(ejMissing()) => 0
  rule exListBytes(ejNil()) => 0
  rule exListBytes(ejCons(V,ejNil())) => exJsonBytes(V)
  rule exListBytes(ejCons(V,ejCons(W,L))) => exJsonBytes(V) +Int 1 +Int exListBytes(ejCons(W,L))
  syntax EJSON ::= exSynthetic() [function]
  rule exSynthetic() => ejObject(ejCons(ejPair("end",ejString("0")),ejCons(ejPair("kind",ejString("synthetic")),ejCons(ejPair("start",ejString("0")),ejNil()))))
  syntax Bool ::= exSpan(EJSON,Int) [function, total]
  rule exSpan(S,N) => ejClosed(S,ejCons(ejString("end"),ejCons(ejString("kind"),ejCons(ejString("start"),ejNil())))) andBool exNatural(ejGet(S,"start")) andBool exNatural(ejGet(S,"end")) andBool ((ejGet(S,"kind") ==K ejString("synthetic") andBool exInt(ejGet(S,"start")) ==Int 0 andBool exInt(ejGet(S,"end")) ==Int 0) orBool (ejGet(S,"kind") ==K ejString("source") andBool exInt(ejGet(S,"start")) <=Int exInt(ejGet(S,"end")) andBool exInt(ejGet(S,"end")) <=Int N))
  syntax EJSON ::= exSafeSpan(EJSON,Int) [function, total]
  rule exSafeSpan(S,N) => S requires exSpan(S,N)
  rule exSafeSpan(_,_) => exSynthetic() [owise]
endmodule
