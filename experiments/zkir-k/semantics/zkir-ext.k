// ZKIR-EXT: the surface of the standalone midnightntwrk/midnight-zkir crate at
// 2ffe2d1 (branch zkir-v3, midnight-zkir 3.0.0) on top of the 92e8bdd3
// definition (plan decision CLM-0707). Differences from 92e8bdd3:
//   types      Bool, Byte, Bytes<n> for any 1 <= n <= 2^24 (Bytes<32> is the old Bytes32;
//              the value is kept as bytes32(_) so every base rule still applies)
//   added      and, or, xor (Bool gates), concat, slice, nth, reverse (byte strings),
//              load_constant (typed constant from its raw encoding), sha512
//   removed    reverse_bytes (subsumed by reverse); tools/zkir_kast.py rejects it in --ext mode
// Encodings (zkir/src/ir_instructions/encode.rs at 2ffe2d1): Bool and Byte are one
// element each; Bytes<n> is ceil(n/31) elements, 31-byte little-endian chunks.
requires "zkir-vm.k"
requires "zkir-ops.k"
requires "zkir-sha512-constants.k"

module ZKIR-EXT-SYNTAX
  imports ZKIR-SYNTAX

  syntax IrType ::= boolT()       [symbol(Bool)]
                  | byteT()       [symbol(Byte)]
                  | bytesT(Int)   [symbol(BytesN)]     // Bytes<n> with n =/= 32
  rule encodedLen(boolT())  => 1
  rule encodedLen(byteT())  => 1
  rule encodedLen(bytesT(N)) => (N +Int 30) /Int 31

  syntax Instr ::= andI(Operands, String)                    [symbol(and)]
                 | orI(Operands, String)                     [symbol(or)]
                 | xorI(Operands, String)                    [symbol(xor)]
                 | concat(Operands, String)                  [symbol(concat)]
                 | slice(Operand, Int, Int, String)          [symbol(slice)]          // bytes, start, len, output
                 | nth(Operand, Int, String)                 [symbol(nth)]            // bytes, index, output
                 | reverse(Operand, String)                  [symbol(reverse)]
                 | loadConstant(IrType, Operands, String)    [symbol(load_constant)]  // type, encoding (immediates), output
                 | sha512(Alignment, Operands, String)       [symbol(sha512)]

  rule reads(andI(Is, _))            => Is
  rule reads(orI(Is, _))             => Is
  rule reads(xorI(Is, _))            => Is
  rule reads(concat(Is, _))          => Is
  rule reads(slice(B, _, _, _))      => B, .Operands
  rule reads(nth(B, _, _))           => B, .Operands
  rule reads(reverse(B, _))          => B, .Operands
  rule reads(loadConstant(_, Es, _)) => Es
  rule reads(sha512(_, Is, _))       => Is
  rule writes(andI(_, O))            => O, .Ids
  rule writes(orI(_, O))             => O, .Ids
  rule writes(xorI(_, O))            => O, .Ids
  rule writes(concat(_, O))          => O, .Ids
  rule writes(slice(_, _, _, O))     => O, .Ids
  rule writes(nth(_, _, O))          => O, .Ids
  rule writes(reverse(_, O))         => O, .Ids
  rule writes(loadConstant(_, _, O)) => O, .Ids
  rule writes(sha512(_, _, O))       => O, .Ids
endmodule

module ZKIR-EXT-VALUES
  imports ZKIR-EXT-SYNTAX
  imports ZKIR-VALUES
  imports ZKIR-OPS

  syntax Value ::= boolV(Bool)   [symbol(boolV)]
                 | byteV(Int)    [symbol(byteV)]
                 | bytesV(Bytes) [symbol(bytesV)]     // length =/= 32
  rule typeOf(boolV(_))   => boolT()
  rule typeOf(byteV(_))   => byteT()
  rule typeOf(bytesV(B))  => bytesT(lengthBytes(B))
  rule defaultValue(boolT())   => boolV(false)
  rule defaultValue(byteT())   => byteV(0)
  rule defaultValue(bytesT(N)) => bytesV(padRightBytes(.Bytes, N, 0))
  rule typeName(boolT())   => "Bool"
  rule typeName(byteT())   => "Byte"
  rule typeName(bytesT(_)) => "Bytes"

  // any byte string value, of either representation
  syntax BytesResult ::= asBytesAny(Value) [function, total]
  rule asBytesAny(bytes32(B)) => bOk(B)
  rule asBytesAny(bytesV(B))  => bOk(B)
  rule asBytesAny(V) => bErr("cannot convert " +String tn(V) +String " to Bytes") [owise]
  syntax Value ::= mkBytes(Bytes) [function, total]
  rule mkBytes(B) => bytes32(B) requires lengthBytes(B) ==Int 32
  rule mkBytes(B) => bytesV(B)  requires lengthBytes(B) =/=Int 32

  rule encodeValue(boolV(true))  => ListItem(1)
  rule encodeValue(boolV(false)) => ListItem(0)
  rule encodeValue(byteV(X))     => ListItem(X)
  rule encodeValue(bytesV(B))    => #encChunks(B)
  syntax List ::= #encChunks(Bytes) [function, total]
  rule #encChunks(B) => .List requires lengthBytes(B) ==Int 0
  rule #encChunks(B) => ListItem(Bytes2Int(substrBytes(B, 0, minInt(31, lengthBytes(B))), LE, Unsigned)) #encChunks(substrBytes(B, minInt(31, lengthBytes(B)), lengthBytes(B)))
    requires lengthBytes(B) >Int 0

  rule decodeValue(ListItem(0), boolT()) => decOk(boolV(false))
  rule decodeValue(ListItem(1), boolT()) => decOk(boolV(true))
  rule decodeValue(Fs, boolT()) => decErr("Failed to decode as Bool") [owise]
  rule decodeValue(ListItem(X), byteT()) => decOk(byteV(X)) requires fitsBits(X, 8)
  rule decodeValue(Fs, byteT()) => decErr("Failed to decode as Byte") [owise]
  rule decodeValue(Fs, bytesT(N)) => decErr("Failed to decode as Bytes") requires size(Fs) =/=Int (N +Int 30) /Int 31
  rule decodeValue(Fs, bytesT(N)) => #decChunks(Fs, N, .Bytes) requires size(Fs) ==Int (N +Int 30) /Int 31
  syntax DecodeResult ::= #decChunks(List, Int, Bytes) [function, total]
  rule #decChunks(.List, 0, Acc) => decOk(mkBytes(Acc))
  rule #decChunks(.List, N, _) => decErr("Failed to decode as Bytes") requires N =/=Int 0
  rule #decChunks(ListItem(F) Fs, N, Acc) => #decChunks(Fs, N -Int minInt(31, N), Acc +Bytes Int2Bytes(minInt(31, N), F, LE))
    requires N >Int 0 andBool fitsBits(F, 8 *Int minInt(31, N))
  rule #decChunks(ListItem(F) _, N, _) => decErr("Failed to decode as Bytes")
    requires N <=Int 0 orBool notBool fitsBits(F, 8 *Int minInt(31, N))
endmodule

// SHA-512 (FIPS 180-4), 64-bit words, 80 rounds, 128-byte blocks.
module ZKIR-SHA512
  imports ZKIR-HASH
  imports ZKIR-SHA512-CONSTANTS

  syntax Int ::= rotr64(Int, Int) [function, total]
  rule rotr64(X, N) => ((X >>Int N) |Int (X <<Int (64 -Int N))) &Int #m64
  syntax Int ::= add64(Int, Int) [function, total]
  rule add64(A, B) => (A +Int B) &Int #m64

  syntax Bytes ::= sha512Bytes(Bytes) [function, total]
  rule sha512Bytes(M) => #sha512Blocks(#sha512Pad(M), 0, #sha512H0)
  syntax Bytes ::= #sha512Pad(Bytes) [function, total]
  rule #sha512Pad(M) => padRightBytes(M +Bytes Int2Bytes(1, 128, BE), ((lengthBytes(M) +Int 17 +Int 127) /Int 128) *Int 128 -Int 16, 0) +Bytes Int2Bytes(16, lengthBytes(M) *Int 8, BE)
  syntax Bytes ::= #sha512Blocks(Bytes, Int, List) [function, total]
  rule #sha512Blocks(P, Off, H) => #sha512Out(H) requires Off >=Int lengthBytes(P)
  rule #sha512Blocks(P, Off, H) => #sha512Blocks(P, Off +Int 128, #sha512Compress(#sha512Schedule(substrBytes(P, Off, Off +Int 128), 0, .List), H))
    requires Off <Int lengthBytes(P)
  syntax Bytes ::= #sha512Out(List) [function, total]
  rule #sha512Out(.List) => .Bytes
  rule #sha512Out(ListItem(W) Ws) => Int2Bytes(8, W, BE) +Bytes #sha512Out(Ws)
  syntax List ::= #sha512Schedule(Bytes, Int, List) [function, total]
  rule #sha512Schedule(B, T, W) => #sha512Schedule(B, T +Int 1, W ListItem(Bytes2Int(substrBytes(B, 8 *Int T, 8 *Int T +Int 8), BE, Unsigned)))
    requires T <Int 16
  rule #sha512Schedule(B, T, W) => #sha512Schedule(B, T +Int 1, W ListItem(add64(add64(#t1({W[T -Int 2]}:>Int), {W[T -Int 7]}:>Int), add64(#t0({W[T -Int 15]}:>Int), {W[T -Int 16]}:>Int))))
    requires T >=Int 16 andBool T <Int 80
  rule #sha512Schedule(_, T, W) => W requires T >=Int 80
  syntax Int ::= #t0(Int) [function, total]
  rule #t0(X) => rotr64(X, 1) xorInt rotr64(X, 8) xorInt (X >>Int 7)
  syntax Int ::= #t1(Int) [function, total]
  rule #t1(X) => rotr64(X, 19) xorInt rotr64(X, 61) xorInt (X >>Int 6)
  syntax Int ::= #T0(Int) [function, total]
  rule #T0(X) => rotr64(X, 28) xorInt rotr64(X, 34) xorInt rotr64(X, 39)
  syntax Int ::= #T1(Int) [function, total]
  rule #T1(X) => rotr64(X, 14) xorInt rotr64(X, 18) xorInt rotr64(X, 41)
  syntax List ::= #sha512Compress(List, List) [function, total]
  rule #sha512Compress(W, H) => #sha512AddH(#sha512Round(W, 0, H), H)
  syntax List ::= #sha512Round(List, Int, List) [function, total]
  rule #sha512Round(_, T, V) => V requires T >=Int 80
  rule #sha512Round(W, T, ListItem(A) ListItem(B) ListItem(C) ListItem(D) ListItem(E) ListItem(F) ListItem(G) ListItem(HH))
    => #sha512Round(W, T +Int 1, #sha512Next(A, B, C, D, E, F, G,
         add64(add64(add64(add64(HH, #T1(E)), (E &Int F) xorInt ((E xorInt #m64) &Int G)), {#sha512Ks[T]}:>Int), {W[T]}:>Int),
         add64(#T0(A), (A &Int B) xorInt (A &Int C) xorInt (B &Int C))))
    requires T <Int 80
  syntax List ::= #sha512Next(Int, Int, Int, Int, Int, Int, Int, Int, Int) [function, total]
  rule #sha512Next(A, B, C, D, E, F, G, T1, T2)
    => ListItem(add64(T1, T2)) ListItem(A) ListItem(B) ListItem(C) ListItem(add64(D, T1)) ListItem(E) ListItem(F) ListItem(G)
  syntax List ::= #sha512AddH(List, List) [function, total]
  rule #sha512AddH(.List, .List) => .List
  rule #sha512AddH(ListItem(X) Xs, ListItem(Y) Ys) => ListItem(add64(X, Y)) #sha512AddH(Xs, Ys)
  rule #sha512AddH(_, _) => .List [owise]
endmodule

module ZKIR-EXT
  imports ZKIR-VM
  imports ZKIR-EXT-VALUES
  imports ZKIR-SHA512

  // The 2ffe2d1 decoder re-encodes every decoded value and rejects
  // non-canonical encodings; switch the base VM to strict decoding before the
  // first job step (higher priority than the base job rule).
  rule <k> job(P, Pre) => job(P, Pre) ... </k> <strictDecode> false => true </strictDecode> [priority(30)]
  rule <k> genJob(P, Pre) => genJob(P, Pre) ... </k> <strictDecode> false => true </strictDecode> [priority(30)]

  // neg on a Bool is logical negation at 2ffe2d1 (neg.rs)
  rule negV(boolV(B)) => vOk(boolV(notBool B))

  // --- boolean gates ---------------------------------------------------------------------
  rule <k> #exec(andI(Is, O)) => #put(O, #boolGate(resolveAll(Is, M), "and")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(orI(Is, O))  => #put(O, #boolGate(resolveAll(Is, M), "or"))  ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(xorI(Is, O)) => #put(O, #boolGate(resolveAll(Is, M), "xor")) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #boolGate(ValuesResult, String) [function, total]
  rule #boolGate(vsErr(S), _) => vErr(S)
  rule #boolGate(vsOk(.List), _) => vErr("Boolean gate requires at least one input")
  rule #boolGate(vsOk(Vs), Op) => #boolFold(Vs, Op, #boolInit(Op)) requires size(Vs) >Int 0
  syntax Bool ::= #boolInit(String) [function, total]
  rule #boolInit("and") => true
  rule #boolInit(_) => false [owise]
  syntax ValueResult ::= #boolFold(List, String, Bool) [function, total]
  rule #boolFold(.List, _, Acc) => vOk(boolV(Acc))
  rule #boolFold(ListItem(boolV(B)) Vs, "and", Acc) => #boolFold(Vs, "and", Acc andBool B)
  rule #boolFold(ListItem(boolV(B)) Vs, "or", Acc)  => #boolFold(Vs, "or", Acc orBool B)
  rule #boolFold(ListItem(boolV(B)) Vs, "xor", Acc) => #boolFold(Vs, "xor", Acc xorBool B)
  rule #boolFold(ListItem(V:Value) _, _, _) => vErr("Boolean gate expects Bool inputs, found " +String tn(V)) [owise]
  rule #boolFold(ListItem(_) _, _, _) => vErr("Boolean gate expects Bool inputs") [owise]

  // --- byte strings ------------------------------------------------------------------------
  rule <k> #exec(concat(Is, O)) => #put(O, #concat(resolveAll(Is, M), .Bytes)) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #concat(ValuesResult, Bytes) [function, total]
  rule #concat(vsErr(S), _) => vErr(S)
  rule #concat(vsOk(.List), Acc) => vErr("Concat requires at least one byte of input") requires lengthBytes(Acc) ==Int 0
  rule #concat(vsOk(.List), Acc) => vOk(mkBytes(Acc)) requires lengthBytes(Acc) >Int 0
  rule #concat(vsOk(ListItem(byteV(X)) Vs), Acc) => #concat(vsOk(Vs), Acc +Bytes Int2Bytes(1, X, LE))
  rule #concat(vsOk(ListItem(bytesV(B)) Vs), Acc) => #concat(vsOk(Vs), Acc +Bytes B)
  rule #concat(vsOk(ListItem(bytes32(B)) Vs), Acc) => #concat(vsOk(Vs), Acc +Bytes B)
  rule #concat(vsOk(ListItem(V:Value) _), _) => vErr("Concat expects Byte or Bytes inputs, found " +String tn(V)) [owise]
  rule #concat(vsOk(ListItem(_) _), _) => vErr("Concat expects Byte or Bytes inputs") [owise]

  rule <k> #exec(slice(B, S, L, O)) => #put(O, #slice(resolve(B, M), S, L)) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #slice(ValueResult, Int, Int) [function, total]
  rule #slice(vErr(S), _, _) => vErr(S)
  rule #slice(vOk(V), S, L) => #slice2(asBytesAny(V), S, L)
  syntax ValueResult ::= #slice2(BytesResult, Int, Int) [function, total]
  rule #slice2(bErr(S), _, _) => vErr(S)
  rule #slice2(bOk(_), _, 0) => vErr("slice length must be at least 1")
  rule #slice2(bOk(B), S, L) => vErr("slice out of bounds") requires L >Int 0 andBool S +Int L >Int lengthBytes(B)
  rule #slice2(bOk(B), S, L) => vOk(mkBytes(substrBytes(B, S, S +Int L))) requires L >Int 0 andBool S +Int L <=Int lengthBytes(B)

  rule <k> #exec(nth(B, I, O)) => #put(O, #nth(resolve(B, M), I)) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #nth(ValueResult, Int) [function, total]
  rule #nth(vErr(S), _) => vErr(S)
  rule #nth(vOk(V), I) => #nth2(asBytesAny(V), I)
  syntax ValueResult ::= #nth2(BytesResult, Int) [function, total]
  rule #nth2(bErr(S), _) => vErr(S)
  rule #nth2(bOk(B), I) => vErr("nth out of bounds") requires I >=Int lengthBytes(B)
  rule #nth2(bOk(B), I) => vOk(byteV(B[I])) requires I <Int lengthBytes(B)

  rule <k> #exec(reverse(B, O)) => #put(O, #rev(resolve(B, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #rev(ValueResult) [function, total]
  rule #rev(vErr(S)) => vErr(S)
  rule #rev(vOk(V)) => #rev2(asBytesAny(V))
  syntax ValueResult ::= #rev2(BytesResult) [function, total]
  rule #rev2(bErr(S)) => vErr(S)
  rule #rev2(bOk(B)) => vOk(mkBytes(reverseBytes(B)))

  // --- constants and sha512 ------------------------------------------------------------------
  rule <k> #exec(loadConstant(T, Es, O)) => #put(O, #toValue(decodeStrict(#immInts(Es), T, true))) ... </k> <status> ok() </status>
  syntax List ::= #immInts(Operands) [function, total]
  rule #immInts(.Operands) => .List
  rule #immInts((imm(I), Es)) => ListItem(I) #immInts(Es)
  rule #immInts((var(_), Es)) => #immInts(Es)

  rule <k> #exec(sha512(Al, Xs, O)) => #put(O, #sha512V(resolveAll(Xs, M), Al)) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #sha512V(ValuesResult, Alignment) [function, total]
  rule #sha512V(vsErr(S), _) => vErr(S)
  rule #sha512V(vsOk(Vs), Al) => #sha512V2(asNatives(Vs), Al)
  syntax ValueResult ::= #sha512V2(IntsResult, Alignment) [function, total]
  rule #sha512V2(isErr(S), _) => vErr(S)
  rule #sha512V2(isOk(L), Al) => #sha512V3(alignedBytes(Al, L))
  syntax ValueResult ::= #sha512V3(BytesResult) [function, total]
  rule #sha512V3(bErr(S)) => vErr(S)
  rule #sha512V3(bOk(B)) => vOk(bytesV(sha512Bytes(B)))

  // --- in-circuit relations of the extension gates ------------------------------------------------
  // Boolean gates and sha512 are functional relations over their inputs;
  // concat / slice / nth / reverse select wires and impose no constraints beyond
  // the output being the selected bytes; load_constant fixes its output.
  rule eval(gate(andI(Is, O)), M, _, _)   => #matches(#boolGate(resolveAll(Is, M), "and"), rdId(O, M), "and output")
  rule eval(gate(orI(Is, O)), M, _, _)    => #matches(#boolGate(resolveAll(Is, M), "or"), rdId(O, M), "or output")
  rule eval(gate(xorI(Is, O)), M, _, _)   => #matches(#boolGate(resolveAll(Is, M), "xor"), rdId(O, M), "xor output")
  rule eval(gate(concat(Is, O)), M, _, _) => #matches(#concat(resolveAll(Is, M), .Bytes), rdId(O, M), "concat output")
  rule eval(gate(slice(B, S, L, O)), M, _, _) => #matches(#slice(rd(B, M), S, L), rdId(O, M), "slice output")
  rule eval(gate(nth(B, I, O)), M, _, _)  => #matches(#nth(rd(B, M), I), rdId(O, M), "nth output")
  rule eval(gate(reverse(B, O)), M, _, _) => #matches(#rev(rd(B, M)), rdId(O, M), "reverse output")
  rule eval(gate(loadConstant(T, Es, O)), M, _, _) => #matches(#toValue(decodeStrict(#immInts(Es), T, true)), rdId(O, M), "load_constant output")
  rule eval(gate(sha512(Al, Xs, O)), M, _, Chips) => #and(#needAll(Xs, M), #and(#chipNamed("sha2_512", Chips), #matches(#sha512V(resolveAll(Xs, M), Al), rdId(O, M), "sha512 output")))
  rule #chipsOfInstrs((sha512(_, _, _) ; Is)) => SetItem("sha2_512") |Set #chipsOfInstrs(Is)
endmodule
