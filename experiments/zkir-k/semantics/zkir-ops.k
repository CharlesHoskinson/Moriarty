// Off-circuit value operations of ZKIR v3 (`ir_instructions/*.rs` at
// midnight-ledger 92e8bdd3), one function per Rust `*_offcircuit` function.
// Every function returns vOk(value) or vErr(message); the messages follow the
// Rust `anyhow!` texts closely enough to recognise them.
requires "zkir-values.k"
requires "zkir-hash.k"

module ZKIR-OPS
  imports ZKIR-VALUES
  imports ZKIR-HASH
  imports STRING

  syntax ValueResult ::= vOk(Value) | vErr(String)
  syntax PairResult  ::= pOk(Value, Value) | pErr(String)
  syntax CheckResult ::= cOk() | cErr(String)

  // --- helpers -------------------------------------------------------------------
  syntax String ::= tn(Value) [function, total]
  rule tn(V) => typeName(typeOf(V))

  syntax Bool ::= sameType(Value, Value) [function, total]
  rule sameType(A, B) => typeOf(A) ==K typeOf(B)

  // `TryFrom<IrValue> for Fr`
  syntax IntResult ::= iOk(Int) | iErr(String)
  syntax IntResult ::= asNative(Value) [function, total]
  rule asNative(native(X)) => iOk(X)
  rule asNative(V) => iErr("cannot convert " +String tn(V) +String " to Native") [owise]
  syntax BytesResult ::= bOk(Bytes) | bErr(String)
  syntax BytesResult ::= asBytes32(Value) [function, total]
  rule asBytes32(bytes32(B)) => bOk(B)
  rule asBytes32(V) => bErr("cannot convert " +String tn(V) +String " to Bytes32") [owise]

  // --- add ---------------------------------------------------------------------------
  syntax ValueResult ::= addV(Value, Value) [function, total]
  rule addV(native(A), native(B))                   => vOk(native(fadd(A, B, #r)))
  rule addV(jubjubPoint(P), jubjubPoint(Q))         => vOk(jubjubPoint(ecAdd(#jubjub, P, Q)))
  rule addV(secp256k1Point(P), secp256k1Point(Q))   => vOk(secp256k1Point(ecAdd(#secp256k1, P, Q)))
  rule addV(secp256k1Base(A), secp256k1Base(B))     => vOk(secp256k1Base(fadd(A, B, #k256P)))
  rule addV(secp256k1Scalar(A), secp256k1Scalar(B)) => vOk(secp256k1Scalar(fadd(A, B, #k256N)))
  rule addV(secp256r1Point(P), secp256r1Point(Q))   => vOk(secp256r1Point(ecAdd(#secp256r1, P, Q)))
  rule addV(secp256r1Base(A), secp256r1Base(B))     => vOk(secp256r1Base(fadd(A, B, #p256P)))
  rule addV(secp256r1Scalar(A), secp256r1Scalar(B)) => vOk(secp256r1Scalar(fadd(A, B, #p256N)))
  rule addV(curve25519Point(P), curve25519Point(Q)) => vOk(curve25519Point(ecAdd(#curve25519, P, Q)))
  rule addV(curve25519Base(A), curve25519Base(B))   => vOk(curve25519Base(fadd(A, B, #c25519P)))
  rule addV(curve25519Scalar(A), curve25519Scalar(B)) => vOk(curve25519Scalar(fadd(A, B, #c25519L)))
  rule addV(A, B) => vErr("Unsupported addition: " +String tn(A) +String " + " +String tn(B)) [owise]

  // --- mul ---------------------------------------------------------------------------
  syntax ValueResult ::= mulV(Value, Value) [function, total]
  rule mulV(native(A), native(B))                   => vOk(native(fmul(A, B, #r)))
  rule mulV(secp256k1Base(A), secp256k1Base(B))     => vOk(secp256k1Base(fmul(A, B, #k256P)))
  rule mulV(secp256k1Scalar(A), secp256k1Scalar(B)) => vOk(secp256k1Scalar(fmul(A, B, #k256N)))
  rule mulV(secp256r1Base(A), secp256r1Base(B))     => vOk(secp256r1Base(fmul(A, B, #p256P)))
  rule mulV(secp256r1Scalar(A), secp256r1Scalar(B)) => vOk(secp256r1Scalar(fmul(A, B, #p256N)))
  rule mulV(curve25519Base(A), curve25519Base(B))   => vOk(curve25519Base(fmul(A, B, #c25519P)))
  rule mulV(curve25519Scalar(A), curve25519Scalar(B)) => vOk(curve25519Scalar(fmul(A, B, #c25519L)))
  rule mulV(A, B) => vErr("Unsupported multiplication: " +String tn(A) +String " x " +String tn(B)) [owise]

  // --- neg ---------------------------------------------------------------------------
  syntax ValueResult ::= negV(Value) [function, total]
  rule negV(native(A))          => vOk(native(fneg(A, #r)))
  rule negV(jubjubPoint(P))     => vOk(jubjubPoint(ecNeg(#jubjub, P)))
  rule negV(secp256k1Point(P))  => vOk(secp256k1Point(ecNeg(#secp256k1, P)))
  rule negV(secp256k1Base(A))   => vOk(secp256k1Base(fneg(A, #k256P)))
  rule negV(secp256k1Scalar(A)) => vOk(secp256k1Scalar(fneg(A, #k256N)))
  rule negV(secp256r1Point(P))  => vOk(secp256r1Point(ecNeg(#secp256r1, P)))
  rule negV(secp256r1Base(A))   => vOk(secp256r1Base(fneg(A, #p256P)))
  rule negV(secp256r1Scalar(A)) => vOk(secp256r1Scalar(fneg(A, #p256N)))
  rule negV(curve25519Point(P)) => vOk(curve25519Point(ecNeg(#curve25519, P)))
  rule negV(curve25519Base(A))  => vOk(curve25519Base(fneg(A, #c25519P)))
  rule negV(curve25519Scalar(A))=> vOk(curve25519Scalar(fneg(A, #c25519L)))
  rule negV(A) => vErr("Unsupported negation of " +String tn(A)) [owise]

  // --- inv ---------------------------------------------------------------------------
  syntax ValueResult ::= invV(Value) [function, total]
  rule invV(native(0))          => vErr("cannot invert zero of type Native")
  rule invV(native(A))          => vOk(native(finv(A, #r))) requires A =/=Int 0
  rule invV(secp256k1Base(A))   => #invField(A, #k256P, secp256k1Base())
  rule invV(secp256k1Scalar(A)) => #invField(A, #k256N, secp256k1Scalar())
  rule invV(secp256r1Base(A))   => #invField(A, #p256P, secp256r1Base())
  rule invV(secp256r1Scalar(A)) => #invField(A, #p256N, secp256r1Scalar())
  rule invV(curve25519Base(A))  => #invField(A, #c25519P, curve25519Base())
  rule invV(curve25519Scalar(A))=> #invField(A, #c25519L, curve25519Scalar())
  rule invV(A) => vErr("Unsupported inversion of " +String tn(A)) [owise]
  syntax ValueResult ::= #invField(Int, Int, IrType) [function, total]
  rule #invField(0, _, T) => vErr("cannot invert zero of type " +String typeName(T))
  rule #invField(A, P, T) => vOk(#wrapField(finv(A, P), T)) requires A =/=Int 0
  syntax Value ::= #wrapField(Int, IrType) [function, total]
  rule #wrapField(X, native())          => native(X)
  rule #wrapField(X, secp256k1Base())   => secp256k1Base(X)
  rule #wrapField(X, secp256k1Scalar()) => secp256k1Scalar(X)
  rule #wrapField(X, secp256r1Base())   => secp256r1Base(X)
  rule #wrapField(X, secp256r1Scalar()) => secp256r1Scalar(X)
  rule #wrapField(X, curve25519Base())  => curve25519Base(X)
  rule #wrapField(X, curve25519Scalar())=> curve25519Scalar(X)
  rule #wrapField(X, jubjubScalar())    => jubjubScalar(X)
  rule #wrapField(X, _)                 => native(X) [owise]

  // --- test_eq / constrain_eq / cond_select --------------------------------------------
  syntax Bool ::= eqSupported(Value, Value) [function, total]
  rule eqSupported(A, B) => sameType(A, B)
  syntax ValueResult ::= testEqV(Value, Value) [function, total]
  rule testEqV(A, B) => vOk(native(bool2Word(A ==K B))) requires sameType(A, B)
  rule testEqV(A, B) => vErr("Unsupported test_eq: " +String tn(A) +String " == " +String tn(B)) requires notBool sameType(A, B)
  syntax Int ::= bool2Word(Bool) [function, total]
  rule bool2Word(true) => 1
  rule bool2Word(false) => 0

  syntax CheckResult ::= constrainEqV(Value, Value) [function, total]
  rule constrainEqV(A, B) => cErr("Unsupported constrain_eq: " +String tn(A) +String " == " +String tn(B)) requires notBool sameType(A, B)
  rule constrainEqV(A, B) => cOk() requires sameType(A, B) andBool A ==K B
  rule constrainEqV(A, B) => cErr("Equality constraint failed") requires sameType(A, B) andBool A =/=K B

  syntax ValueResult ::= selectV(Bool, Value, Value) [function, total]
  rule selectV(_, A, B) => vErr("Unsupported cond_select: " +String tn(A) +String " ? " +String tn(B)) requires notBool sameType(A, B)
  rule selectV(true, A, B) => vOk(A) requires sameType(A, B)
  rule selectV(false, A, B) => vOk(B) requires sameType(A, B)

  // --- curves ------------------------------------------------------------------------
  syntax ValueResult ::= ecMulV(Value, Value) [function, total]
  rule ecMulV(jubjubPoint(P), jubjubScalar(S))         => vOk(jubjubPoint(ecMul(#jubjub, P, S)))
  rule ecMulV(secp256k1Point(P), secp256k1Scalar(S))   => vOk(secp256k1Point(ecMul(#secp256k1, P, S)))
  rule ecMulV(secp256r1Point(P), secp256r1Scalar(S))   => vOk(secp256r1Point(ecMul(#secp256r1, P, S)))
  rule ecMulV(curve25519Point(P), curve25519Scalar(S)) => vOk(curve25519Point(ecMul(#curve25519, P, S)))
  rule ecMulV(A, B) => vErr("Unsupported EC multiplication: " +String tn(A) +String " x " +String tn(B)) [owise]

  syntax ValueResult ::= ecMulGeneratorV(Value) [function, total]
  rule ecMulGeneratorV(jubjubScalar(S))    => vOk(jubjubPoint(ecMul(#jubjub, #jubjubGenerator, S)))
  rule ecMulGeneratorV(secp256k1Scalar(S)) => vOk(secp256k1Point(ecMul(#secp256k1, #secp256k1Generator, S)))
  rule ecMulGeneratorV(A) => vErr("Unsupported EcMulGenerator for scalar of type " +String tn(A)) [owise]

  syntax PairResult ::= intoCoordinatesV(Value) [function, total]
  rule intoCoordinatesV(jubjubPoint(pt(X, Y)))     => pOk(native(X), native(Y))
  rule intoCoordinatesV(jubjubPoint(inf()))        => pOk(native(0), native(1))
  rule intoCoordinatesV(secp256k1Point(inf()))     => pErr("Cannot extract coordinates of the Secp256k1 identity")
  rule intoCoordinatesV(secp256k1Point(pt(X, Y)))  => pOk(secp256k1Base(X), secp256k1Base(Y))
  rule intoCoordinatesV(secp256r1Point(inf()))     => pErr("Cannot extract coordinates of the Secp256r1 identity")
  rule intoCoordinatesV(secp256r1Point(pt(X, Y)))  => pOk(secp256r1Base(X), secp256r1Base(Y))
  rule intoCoordinatesV(curve25519Point(pt(X, Y))) => pOk(curve25519Base(X), curve25519Base(Y))
  rule intoCoordinatesV(curve25519Point(inf()))    => pOk(curve25519Base(0), curve25519Base(1))
  rule intoCoordinatesV(A) => pErr("Unsupported coordinate extraction of " +String tn(A)) [owise]

  syntax ValueResult ::= fromCoordinatesV(Value, Value) [function, total]
  rule fromCoordinatesV(native(X), native(Y))                 => #ptToValue(jubjubFromXY(X, Y), #jubjub, jubjubPoint(), "Cannot build a Jubjub point")
  rule fromCoordinatesV(secp256k1Base(X), secp256k1Base(Y))   => #ptToValue(fromXY(#secp256k1, X, Y), #secp256k1, secp256k1Point(), "Cannot build a Secp256k1Point point")
  rule fromCoordinatesV(secp256r1Base(X), secp256r1Base(Y))   => #ptToValue(fromXY(#secp256r1, X, Y), #secp256r1, secp256r1Point(), "Cannot build a Secp256r1Point point")
  rule fromCoordinatesV(curve25519Base(X), curve25519Base(Y)) => #ptToValue(fromXY(#curve25519, X, Y), #curve25519, curve25519Point(), "Cannot build a Curve25519Point point")
  rule fromCoordinatesV(A, B) => vErr("Unsupported `from_coordinates` on (" +String tn(A) +String ", " +String tn(B) +String ")") [owise]
  syntax ValueResult ::= #ptToValue(PointResult, Curve, IrType, String) [function, total]
  rule #ptToValue(ptErr(_), _, _, Msg) => vErr(Msg)
  rule #ptToValue(ptOk(Q), C, T, _) => vOk(#wrapPoint(Q, T)) requires inSubgroup(C, Q)
  rule #ptToValue(ptOk(Q), C, _, Msg) => vErr(Msg) requires notBool inSubgroup(C, Q)

  // --- bytes -----------------------------------------------------------------------------
  syntax ValueResult ::= intoBytes32V(Value) [function, total]
  rule intoBytes32V(native(X))          => vOk(bytes32(intToBytes32(X)))
  rule intoBytes32V(secp256k1Base(X))   => vOk(bytes32(intToBytes32(X)))
  rule intoBytes32V(secp256k1Scalar(X)) => vOk(bytes32(intToBytes32(X)))
  rule intoBytes32V(secp256r1Base(X))   => vOk(bytes32(intToBytes32(X)))
  rule intoBytes32V(secp256r1Scalar(X)) => vOk(bytes32(intToBytes32(X)))
  rule intoBytes32V(curve25519Base(X))  => vOk(bytes32(intToBytes32(X)))
  rule intoBytes32V(curve25519Scalar(X))=> vOk(bytes32(intToBytes32(X)))
  rule intoBytes32V(A) => vErr("Unsupported into_bytes32 for " +String tn(A)) [owise]

  // from_bytes32: little-endian integer reduced modulo the target field
  syntax ValueResult ::= fromBytes32V(IrType, Bytes) [function, total]
  rule fromBytes32V(native(), B)          => vOk(native(bytes32ToInt(B) modInt #r))
  rule fromBytes32V(secp256k1Base(), B)   => vOk(secp256k1Base(bytes32ToInt(B) modInt #k256P))
  rule fromBytes32V(secp256k1Scalar(), B) => vOk(secp256k1Scalar(bytes32ToInt(B) modInt #k256N))
  rule fromBytes32V(secp256r1Base(), B)   => vOk(secp256r1Base(bytes32ToInt(B) modInt #p256P))
  rule fromBytes32V(secp256r1Scalar(), B) => vOk(secp256r1Scalar(bytes32ToInt(B) modInt #p256N))
  rule fromBytes32V(curve25519Base(), B)  => vOk(curve25519Base(bytes32ToInt(B) modInt #c25519P))
  rule fromBytes32V(curve25519Scalar(), B)=> vOk(curve25519Scalar(bytes32ToInt(B) modInt #c25519L))
  rule fromBytes32V(T, _) => vErr("Unsupported from_bytes32 for type " +String typeName(T)) [owise]

  // bytes32_into_low_high: low = first 31 bytes as a native element, high = byte 31
  syntax PairResult ::= bytes32IntoLowHighV(Bytes) [function, total]
  rule bytes32IntoLowHighV(B) => pOk(native(Bytes2Int(substrBytes(B, 0, 31), LE, Unsigned) modInt #r), native(B[31]))

  // bytes32_from_low_high: both operands must be native; low < 2^248, high < 256
  syntax ValueResult ::= bytes32FromLowHighV(Value, Value) [function, total]
  rule bytes32FromLowHighV(native(L), native(H)) => vOk(bytes32(replaceAtBytes(intToBytes32(L), 31, Int2Bytes(1, H, LE))))
    requires fitsBits(L, 248) andBool fitsBits(H, 8)
  rule bytes32FromLowHighV(native(L), native(H)) => vErr("Bytes32FromLowHigh: low operand must fit in 31 bytes (be less than 2^248) and high operand must fit in a single byte (be less than 256)")
    requires notBool (fitsBits(L, 248) andBool fitsBits(H, 8))
  // Rust: into_bytes32_offcircuit on each operand first, so non-field operands fail there
  rule bytes32FromLowHighV(A, B) => #lowHighErr(intoBytes32V(A), intoBytes32V(B)) [owise]
  syntax ValueResult ::= #lowHighErr(ValueResult, ValueResult) [function, total]
  rule #lowHighErr(vErr(S), _) => vErr(S)
  rule #lowHighErr(vOk(_), vErr(S)) => vErr(S)
  rule #lowHighErr(vOk(bytes32(BL)), vOk(bytes32(BH))) => vOk(bytes32(replaceAtBytes(BL, 31, substrBytes(BH, 0, 1))))
    requires BL[31] ==Int 0 andBool Bytes2Int(substrBytes(BH, 1, 32), LE, Unsigned) ==Int 0
  rule #lowHighErr(vOk(_), vOk(_)) => vErr("Bytes32FromLowHigh: low operand must fit in 31 bytes (be less than 2^248) and high operand must fit in a single byte (be less than 256)") [owise]

  // --- hashes --------------------------------------------------------------------------
  // All inputs of transient_hash / hash_to_curve must be Native.
  syntax IntsResult ::= isOk(List) | isErr(String)
  syntax IntsResult ::= asNatives(List) [function, total]
  rule asNatives(.List) => isOk(.List)
  rule asNatives(ListItem(V) Vs) => #consNative(asNative(V), asNatives(Vs))
  syntax IntsResult ::= #consNative(IntResult, IntsResult) [function, total]
  rule #consNative(iErr(S), _) => isErr(S)
  rule #consNative(iOk(_), isErr(S)) => isErr(S)
  rule #consNative(iOk(X), isOk(L)) => isOk(ListItem(X) L)

  // persistent_hash / keccak256 preimage: parse the field elements by the
  // alignment (transient-crypto AlignmentAtomExt::parse_field_repr) and lay
  // the atoms out as bytes (ValueAtomExt::binary_repr_unchecked). Only atom
  // segments are supported; `option` segments and `compress` atoms are
  // reported as unsupported.
  syntax BytesResult ::= alignedBytes(Alignment, List) [function, total]
  rule alignedBytes(alignment(Segs), Fs) => #alignedBytes(Segs, Fs, .Bytes)
  syntax BytesResult ::= #alignedBytes(Segments, List, Bytes) [function, total]
  rule #alignedBytes(.Segments, .List, Acc) => bOk(Acc)
  rule #alignedBytes(.Segments, Fs, _) => bErr("Inputs did not match alignment") requires size(Fs) >Int 0
  rule #alignedBytes((atom(fieldAtom()), Segs), ListItem(F) Fs, Acc) => #alignedBytes(Segs, Fs, Acc +Bytes Int2Bytes(32, F, LE))
  rule #alignedBytes((atom(fieldAtom()), _), .List, _) => bErr("Inputs did not match alignment")
  rule #alignedBytes((atom(bytesAtom(N)), Segs), Fs, Acc) => #alignedBytesAtom(N, Segs, Fs, Acc, bytesFromFieldRepr(Fs, N))
  rule #alignedBytes((atom(compressAtom()), _), _, _) => bErr("Inputs did not match alignment")   // Compress atoms cannot be parsed from field elements
  rule #alignedBytes((option(_), _), _, _) => bErr("unsupported: alignment option segment")
  syntax BytesResult ::= #alignedBytesAtom(Int, Segments, List, Bytes, BytesResult) [function, total]
  rule #alignedBytesAtom(_, _, _, _, bErr(S)) => bErr(S)
  rule #alignedBytesAtom(N, Segs, Fs, Acc, bOk(B)) => #alignedBytes(Segs, range(Fs, (N +Int 30) /Int 31, 0), Acc +Bytes B)

  // bytes_from_field_repr(repr, n): stray = n mod 31 bytes come first (from
  // repr[0]), then the 31-byte chunks in reverse order of the elements.
  syntax BytesResult ::= bytesFromFieldRepr(List, Int) [function, total]
  rule bytesFromFieldRepr(Fs, N) => bErr("Inputs did not match alignment") requires size(Fs) <Int (N +Int 30) /Int 31
  rule bytesFromFieldRepr(Fs, N) => #bffr(Fs, N, N modInt 31, N /Int 31) requires size(Fs) >=Int (N +Int 30) /Int 31
  syntax BytesResult ::= #bffr(List, Int, Int, Int) [function, total]   // (fields, n, stray, chunks)
  rule #bffr(Fs, N, 0, Chunks) => #bffrChunks(Fs, Chunks, 0, .Bytes)
  rule #bffr(ListItem(F) Fs, N, Stray, Chunks) => #bffrStray(#bffrChunks(Fs, Chunks, 0, .Bytes), F, Stray)
    requires Stray >Int 0 andBool fitsBits(F, 8 *Int Stray)
  rule #bffr(ListItem(F) _, _, Stray, _) => bErr("Inputs did not match alignment")
    requires Stray >Int 0 andBool notBool fitsBits(F, 8 *Int Stray)
  rule #bffr(.List, _, Stray, _) => bErr("Inputs did not match alignment") requires Stray >Int 0
  syntax BytesResult ::= #bffrStray(BytesResult, Int, Int) [function, total]
  rule #bffrStray(bErr(S), _, _) => bErr(S)
  rule #bffrStray(bOk(B), F, Stray) => bOk(B +Bytes Int2Bytes(Stray, F, LE))
  // chunk i takes fields[chunks - 1 - i]
  syntax BytesResult ::= #bffrChunks(List, Int, Int, Bytes) [function, total]
  rule #bffrChunks(_, Chunks, I, Acc) => bOk(Acc) requires I >=Int Chunks
  rule #bffrChunks(Fs, Chunks, I, Acc) => #bffrChunks(Fs, Chunks, I +Int 1, Acc +Bytes Int2Bytes(31, {Fs[Chunks -Int 1 -Int I]}:>Int, LE))
    requires I <Int Chunks andBool fitsBits({Fs[Chunks -Int 1 -Int I]}:>Int, 248)
  rule #bffrChunks(Fs, Chunks, I, _) => bErr("Inputs did not match alignment")
    requires I <Int Chunks andBool notBool fitsBits({Fs[Chunks -Int 1 -Int I]}:>Int, 248)

  // --- bits ---------------------------------------------------------------------------
  // resolve_operand_bits with a bound: value must be Native and fit in N bits;
  // N >= FR_BITS is "Excessive bit bound".
  syntax CheckResult ::= checkBits(Value, Int) [function, total]
  rule checkBits(_, N) => cErr("Excessive bit bound") requires N >=Int #frBits
  rule checkBits(native(V), N) => cOk() requires N <Int #frBits andBool fitsBits(V, N)
  rule checkBits(native(V), N) => cErr("Bit bound failed: " +String Int2String(V) +String " is not " +String Int2String(N) +String "-bit")
    requires N <Int #frBits andBool notBool fitsBits(V, N)
  rule checkBits(V, N) => cErr("cannot convert " +String tn(V) +String " to Native") requires N <Int #frBits andBool notBool isNative(V)
  syntax Bool ::= isNative(Value) [function, total]
  rule isNative(native(_)) => true
  rule isNative(_) => false [owise]

  // resolve_operand_bool: Native 0 or 1
  syntax BoolResult ::= boolOk(Bool) | boolErr(String)
  syntax BoolResult ::= asBool(Value) [function, total]
  rule asBool(native(0)) => boolOk(false)
  rule asBool(native(1)) => boolOk(true)
  rule asBool(native(V)) => boolErr("Expected boolean, found: " +String Int2String(V)) requires V =/=Int 0 andBool V =/=Int 1
  rule asBool(V) => boolErr("cannot convert " +String tn(V) +String " to Native") requires notBool isNative(V)
endmodule
