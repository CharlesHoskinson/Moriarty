// Isolated Core/4 extension of the expression definition; legacy files are unchanged.
requires "expression-json.k"
module MORIARTY-LIFECYCLE-EXPRESSION-WIRE
  imports MORIARTY-EXPRESSION-JSON
  // Pinned domains.md:1751-1758 specifies substr only for start<end<=length.
  // All callers use this explicit total helper, including empty one-character tails.
  syntax String ::= lxSubstring(String,Int,Int) [function, total]
  rule lxSubstring(S,A,B) => substrString(S,A,B) requires A >=Int 0 andBool A <Int B andBool B <=Int lengthString(S)
  rule lxSubstring(_,A,B) => "" requires A >=Int B
  rule lxSubstring(S,A,B) => "" requires A <Int 0 orBool B >Int lengthString(S)
  syntax Bool ::= lxAlpha(String) [function, total]
                | lxDigit(String) [function, total]
                | lxIdentifier(EJSON) [function, total]
                | lxIdentifierTail(String) [function, total]
                | lxDecimal(EJSON) [function, total]
                | lxDigits(String) [function, total]
                | lxNatural(EJSON) [function, total]
  rule lxAlpha(C) => (C >=String "A" andBool C <=String "Z") orBool (C >=String "a" andBool C <=String "z") requires lengthString(C) ==Int 1
  rule lxAlpha(_) => false [owise]
  rule lxDigit(C) => C >=String "0" andBool C <=String "9" requires lengthString(C) ==Int 1
  rule lxDigit(_) => false [owise]
  rule lxIdentifier(ejString(S)) => lengthString(S) >=Int 1 andBool lengthString(S) <=Int 64 andBool lxAlpha(lxSubstring(S,0,1)) andBool lxIdentifierTail(lxSubstring(S,1,lengthString(S)))
  rule lxIdentifier(_) => false [owise]
  rule lxIdentifierTail("") => true
  rule lxIdentifierTail(S) => (lxAlpha(lxSubstring(S,0,1)) orBool lxDigit(lxSubstring(S,0,1)) orBool lxSubstring(S,0,1) ==String "_") andBool lxIdentifierTail(lxSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule lxDigits("") => true
  rule lxDigits(S) => lxDigit(lxSubstring(S,0,1)) andBool lxDigits(lxSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule lxNatural(ejString("0")) => true
  rule lxNatural(ejString(S)) => lengthString(S) >Int 0 andBool lxSubstring(S,0,1) >=String "1" andBool lxSubstring(S,0,1) <=String "9" andBool lxDigits(lxSubstring(S,1,lengthString(S))) requires S =/=String "0"
  rule lxNatural(_) => false [owise]
  rule lxDecimal(ejString(S)) => lxNatural(ejString(S)) orBool (lengthString(S) >Int 1 andBool lxSubstring(S,0,1) ==String "-" andBool lxSubstring(S,1,2) =/=String "0" andBool lxNatural(ejString(lxSubstring(S,1,lengthString(S)))))
  rule lxDecimal(_) => false [owise]
  syntax Int ::= lxInt(EJSON) [function, total]
               | lxUtf8(String) [function, total]
               | lxQuotedBytes(String) [function, total]
               | lxEscapedBytes(Int) [function, total]
               | lxJsonBytes(EJSON) [function, total]
               | lxListBytes(EJList) [function, total]
  rule lxInt(ejString(S)) => String2Int(S) requires lxDecimal(ejString(S))
  rule lxInt(_) => 0 [owise]
  // LLVM String hooks index UTF-8 bytes, not Unicode code points.
  rule lxUtf8(S) => lengthString(S)
  rule lxEscapedBytes(N) => 2 requires N ==Int 34 orBool N ==Int 92 orBool N ==Int 8 orBool N ==Int 9 orBool N ==Int 10 orBool N ==Int 12 orBool N ==Int 13
  rule lxEscapedBytes(N) => 6 requires N <Int 32 andBool N =/=Int 8 andBool N =/=Int 9 andBool N =/=Int 10 andBool N =/=Int 12 andBool N =/=Int 13
  rule lxEscapedBytes(N) => 1 requires N >=Int 32 andBool N =/=Int 34 andBool N =/=Int 92
  rule lxQuotedBytes("") => 2
  rule lxQuotedBytes(S) => lxEscapedBytes(ordChar(lxSubstring(S,0,1))) +Int lxQuotedBytes(lxSubstring(S,1,lengthString(S))) requires S =/=String ""
  rule lxJsonBytes(ejString(S)) => lxQuotedBytes(S)
  rule lxJsonBytes(ejBool(true)) => 4
  rule lxJsonBytes(ejBool(false)) => 5
  rule lxJsonBytes(ejNull()) => 4
  rule lxJsonBytes(ejNumber(N)) => lengthString(Int2String(N))
  rule lxJsonBytes(ejArray(L)) => 2 +Int lxListBytes(L)
  rule lxJsonBytes(ejObject(L)) => 2 +Int lxListBytes(L)
  rule lxJsonBytes(ejPair(K,V)) => lxQuotedBytes(K) +Int 1 +Int lxJsonBytes(V)
  rule lxJsonBytes(ejMissing()) => 0
  rule lxListBytes(ejNil()) => 0
  rule lxListBytes(ejCons(V,ejNil())) => lxJsonBytes(V)
  rule lxListBytes(ejCons(V,ejCons(W,L))) => lxJsonBytes(V) +Int 1 +Int lxListBytes(ejCons(W,L))
  syntax EJSON ::= lxSynthetic() [function]
  rule lxSynthetic() => ejObject(ejCons(ejPair("end",ejString("0")),ejCons(ejPair("kind",ejString("synthetic")),ejCons(ejPair("start",ejString("0")),ejNil()))))
  syntax Bool ::= lxSpan(EJSON,Int) [function, total]
  rule lxSpan(S,N) => ejClosed(S,ejCons(ejString("end"),ejCons(ejString("kind"),ejCons(ejString("start"),ejNil())))) andBool lxNatural(ejGet(S,"start")) andBool lxNatural(ejGet(S,"end")) andBool ((ejGet(S,"kind") ==K ejString("synthetic") andBool lxInt(ejGet(S,"start")) ==Int 0 andBool lxInt(ejGet(S,"end")) ==Int 0) orBool (ejGet(S,"kind") ==K ejString("source") andBool lxInt(ejGet(S,"start")) <=Int lxInt(ejGet(S,"end")) andBool lxInt(ejGet(S,"end")) <=Int N))
  syntax EJSON ::= lxSafeSpan(EJSON,Int) [function, total]
  rule lxSafeSpan(S,N) => S requires lxSpan(S,N)
  rule lxSafeSpan(_,_) => lxSynthetic() [owise]
endmodule
