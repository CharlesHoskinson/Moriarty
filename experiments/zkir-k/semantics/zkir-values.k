// ZKIR v3 runtime values (ir_types.rs `IrValue`) and their encoding as raw
// native field elements (ir_instructions/encode.rs, midnight-circuits 7.2.4
// `Instantiable::as_public_input` / `from_public_input`).
//
//   Native            [x]
//   Bytes32           [int_le(bytes[0..31]), bytes[31]]
//   JubjubPoint       [x, y]  (affine twisted-Edwards coordinates)
//   JubjubScalar      [s]     (252-bit scalar, one element)
//   Foreign field K   limbs of (x - 1 mod p) in base 2^LOG2_BASE, NB_LIMBS limbs,
//                     grouped 254/LOG2_BASE limbs per element:
//                       secp256k1/secp256r1 base and scalar, Curve25519 base:
//                         LOG2_BASE 64, 4 limbs, 3 per element  -> 2 elements
//                       Curve25519 scalar: LOG2_BASE 51, 5 limbs, 4 per element -> 2 elements
//   Weierstrass point enc(x) ++ enc(y) ++ [is_identity]; identity encodes (0,0),
//                     i.e. enc(p-1) twice, then 1
//   Curve25519 point  enc(x) ++ enc(y)
requires "zkir-syntax.k"
requires "zkir-curves.k"

module ZKIR-VALUES
  imports ZKIR-SYNTAX
  imports ZKIR-CURVES
  imports BYTES
  imports LIST
  imports STRING

  syntax Value ::= native(Int)            [symbol(nativeV)]
                 | bytes32(Bytes)         [symbol(bytes32V)]
                 | jubjubPoint(Point)     [symbol(jubjubPointV)]
                 | jubjubScalar(Int)      [symbol(jubjubScalarV)]
                 | secp256k1Point(Point)  [symbol(secp256k1PointV)]
                 | secp256k1Base(Int)     [symbol(secp256k1BaseV)]
                 | secp256k1Scalar(Int)   [symbol(secp256k1ScalarV)]
                 | secp256r1Point(Point)  [symbol(secp256r1PointV)]
                 | secp256r1Base(Int)     [symbol(secp256r1BaseV)]
                 | secp256r1Scalar(Int)   [symbol(secp256r1ScalarV)]
                 | curve25519Point(Point) [symbol(curve25519PointV)]
                 | curve25519Base(Int)    [symbol(curve25519BaseV)]
                 | curve25519Scalar(Int)  [symbol(curve25519ScalarV)]

  syntax IrType ::= typeOf(Value) [function, total]
  rule typeOf(native(_))          => native()
  rule typeOf(bytes32(_))         => bytes32()
  rule typeOf(jubjubPoint(_))     => jubjubPoint()
  rule typeOf(jubjubScalar(_))    => jubjubScalar()
  rule typeOf(secp256k1Point(_))  => secp256k1Point()
  rule typeOf(secp256k1Base(_))   => secp256k1Base()
  rule typeOf(secp256k1Scalar(_)) => secp256k1Scalar()
  rule typeOf(secp256r1Point(_))  => secp256r1Point()
  rule typeOf(secp256r1Base(_))   => secp256r1Base()
  rule typeOf(secp256r1Scalar(_)) => secp256r1Scalar()
  rule typeOf(curve25519Point(_)) => curve25519Point()
  rule typeOf(curve25519Base(_))  => curve25519Base()
  rule typeOf(curve25519Scalar(_))=> curve25519Scalar()

  // `IrValue::default(t)`: zero / identity of each type.
  syntax Value ::= defaultValue(IrType) [function, total]
  rule defaultValue(native())          => native(0)
  rule defaultValue(bytes32())         => bytes32(padRightBytes(.Bytes, 32, 0))
  rule defaultValue(jubjubPoint())     => jubjubPoint(pt(0, 1))
  rule defaultValue(jubjubScalar())    => jubjubScalar(0)
  rule defaultValue(secp256k1Point())  => secp256k1Point(inf())
  rule defaultValue(secp256k1Base())   => secp256k1Base(0)
  rule defaultValue(secp256k1Scalar()) => secp256k1Scalar(0)
  rule defaultValue(secp256r1Point())  => secp256r1Point(inf())
  rule defaultValue(secp256r1Base())   => secp256r1Base(0)
  rule defaultValue(secp256r1Scalar()) => secp256r1Scalar(0)
  rule defaultValue(curve25519Point()) => curve25519Point(pt(0, 1))
  rule defaultValue(curve25519Base())  => curve25519Base(0)
  rule defaultValue(curve25519Scalar())=> curve25519Scalar(0)

  // Debug-style names matching the Rust variant names (used by the oracle).
  syntax String ::= typeName(IrType) [function, total]
  rule typeName(native())          => "Native"
  rule typeName(bytes32())         => "Bytes32"
  rule typeName(jubjubPoint())     => "JubjubPoint"
  rule typeName(jubjubScalar())    => "JubjubScalar"
  rule typeName(secp256k1Point())  => "Secp256k1Point"
  rule typeName(secp256k1Base())   => "Secp256k1Base"
  rule typeName(secp256k1Scalar()) => "Secp256k1Scalar"
  rule typeName(secp256r1Point())  => "Secp256r1Point"
  rule typeName(secp256r1Base())   => "Secp256r1Base"
  rule typeName(secp256r1Scalar()) => "Secp256r1Scalar"
  rule typeName(curve25519Point()) => "Curve25519Point"
  rule typeName(curve25519Base())  => "Curve25519Base"
  rule typeName(curve25519Scalar())=> "Curve25519Scalar"

  // --- Foreign-field limb encoding -------------------------------------------------
  // encForeign(x, p, log2Base, nbLimbs): list of native elements.
  syntax List ::= encForeign(Int, Int, Int, Int) [function, total]
  rule encForeign(X, P, L, NB) => #groupLimbs(fsub(X, 1, P), L, NB, 254 /Int L) requires L >Int 0 andBool P >Int 0
  rule encForeign(_, _, _, _) => .List [owise]
  // #groupLimbs(e, log2Base, limbsLeft, perBatch): successive batches of perBatch limbs
  syntax List ::= #groupLimbs(Int, Int, Int, Int) [function, total]
  rule #groupLimbs(_, _, NB, _) => .List requires NB <=Int 0
  rule #groupLimbs(E, L, NB, B) => ListItem(lowBits(E, L *Int minInt(NB, B))) #groupLimbs(highBits(E, L *Int minInt(NB, B)), L, NB -Int minInt(NB, B), B)
    requires NB >Int 0

  // decForeign(fields, p, log2Base, nbLimbs): the element, or -1 when the
  // encoding is invalid (wrong count, batch too large, non-zero tail limbs).
  syntax Int ::= decForeign(List, Int, Int, Int) [function, total]
  rule decForeign(_, _, L, _) => -1 requires L <=Int 0 orBool L >Int 254
  rule decForeign(Fs, P, L, NB) => -1
    requires L >Int 0 andBool L <=Int 254 andBool size(Fs) =/=Int (NB +Int (254 /Int L) -Int 1) /Int (254 /Int L)
  rule decForeign(Fs, P, L, NB) => #decForeign(Fs, P, L, NB, 254 /Int L, 0, 0)
    requires L >Int 0 andBool L <=Int 254 andBool size(Fs) ==Int (NB +Int (254 /Int L) -Int 1) /Int (254 /Int L)
  // #decForeign(fields, p, log2, nb, perBatch, accumulated, limbsSeen)
  syntax Int ::= #decForeign(List, Int, Int, Int, Int, Int, Int) [function, total]
  rule #decForeign(.List, P, L, NB, _, Acc, _) => fadd(Acc, 1, P)
    requires fitsBits(Acc, L *Int NB)
  rule #decForeign(.List, _, L, NB, _, Acc, _) => -1
    requires notBool fitsBits(Acc, L *Int NB)
  rule #decForeign(ListItem(F) Fs, P, L, NB, B, Acc, Seen) => -1
    requires notBool fitsBits(F, L *Int B)
  rule #decForeign(ListItem(F) Fs, P, L, NB, B, Acc, Seen)
    => #decForeign(Fs, P, L, NB, B, Acc +Int (F <<Int (L *Int Seen)), Seen +Int B)
    requires fitsBits(F, L *Int B)

  // Curve-specific parameters (LOG2_BASE, NB_LIMBS) over the BLS12-381 scalar field.
  syntax List ::= encField64(Int, Int) [function, total]    // 64-bit limbs, 4 limbs
  rule encField64(X, P) => encForeign(X, P, 64, 4)
  syntax Int ::= decField64(List, Int) [function, total]
  rule decField64(Fs, P) => decForeign(Fs, P, 64, 4)

  // --- Points ----------------------------------------------------------------------
  syntax List ::= encWPoint(Point, Int) [function, total]
  rule encWPoint(inf(), P) => encField64(0, P) encField64(0, P) ListItem(1)
  rule encWPoint(pt(X, Y), P) => encField64(X, P) encField64(Y, P) ListItem(0)
  syntax List ::= encEPointForeign(Point, Int) [function, total]
  rule encEPointForeign(pt(X, Y), P) => encField64(X, P) encField64(Y, P)
  rule encEPointForeign(inf(), P) => encField64(0, P) encField64(1, P)

  // --- Value encoding (encode_offcircuit) ---------------------------------------------
  syntax List ::= encodeValue(Value) [function, total]
  rule encodeValue(native(X))            => ListItem(X)
  rule encodeValue(bytes32(B))           => ListItem(Bytes2Int(substrBytes(B, 0, 31), LE, Unsigned)) ListItem(B[31]) requires lengthBytes(B) ==Int 32
  rule encodeValue(bytes32(B))           => .List requires lengthBytes(B) =/=Int 32   // not a value the VM constructs
  rule encodeValue(jubjubPoint(pt(X, Y))) => ListItem(X) ListItem(Y)
  rule encodeValue(jubjubPoint(inf()))   => ListItem(0) ListItem(1)   // Edwards values never carry inf(); total-function completeness only
  rule encodeValue(jubjubScalar(S))      => ListItem(S)
  rule encodeValue(secp256k1Point(Q))    => encWPoint(Q, #k256P)
  rule encodeValue(secp256k1Base(X))     => encField64(X, #k256P)
  rule encodeValue(secp256k1Scalar(X))   => encField64(X, #k256N)
  rule encodeValue(secp256r1Point(Q))    => encWPoint(Q, #p256P)
  rule encodeValue(secp256r1Base(X))     => encField64(X, #p256P)
  rule encodeValue(secp256r1Scalar(X))   => encField64(X, #p256N)
  rule encodeValue(curve25519Point(Q))   => encEPointForeign(Q, #c25519P)
  rule encodeValue(curve25519Base(X))    => encField64(X, #c25519P)
  rule encodeValue(curve25519Scalar(X))  => encForeign(X, #c25519L, 51, 5)

  // --- Value decoding (decode_offcircuit) -------------------------------------------------
  syntax DecodeResult ::= decOk(Value) | decErr(String) | decPanic(String)   // decPanic: a Rust assert! fires

  syntax DecodeResult ::= decodeValue(List, IrType) [function, total]
  // a raw element is an Fr, so it must already lie in [0, r)
  rule decodeValue(ListItem(X), native()) => decOk(native(X)) requires X >=Int 0 andBool X <Int #blsScalarModulus
  rule decodeValue(ListItem(X), native()) => decErr("is not a canonical field element") requires X <Int 0 orBool X >=Int #blsScalarModulus
  rule decodeValue(Fs, native()) => decErr("Failed to decode as Native") requires size(Fs) =/=Int 1

  // Bytes32: the Rust decoder asserts (panics) when the low element uses byte
  // 31 or the high element exceeds one byte (encode.rs assert_eq!)
  rule decodeValue(ListItem(Lo) ListItem(Hi), bytes32())
    => decOk(bytes32(replaceAtBytes(Int2Bytes(32, Lo, LE), 31, Int2Bytes(1, Hi, LE))))
    requires fitsBits(Lo, 248) andBool fitsBits(Hi, 8)
  rule decodeValue(ListItem(Lo) ListItem(Hi), bytes32()) => decPanic("assertion failed: Bytes32 low element uses byte 31 or high element exceeds a byte")
    requires notBool (fitsBits(Lo, 248) andBool fitsBits(Hi, 8))
  rule decodeValue(Fs, bytes32()) => decErr("Failed to decode as Bytes32") requires size(Fs) =/=Int 2

  rule decodeValue(ListItem(X) ListItem(Y), jubjubPoint()) => #decJubjub(jubjubFromXY(X, Y))
  rule decodeValue(Fs, jubjubPoint()) => decErr("Failed to decode as JubjubPoint") requires size(Fs) =/=Int 2
  syntax DecodeResult ::= #decJubjub(PointResult) [function, total]
  rule #decJubjub(ptErr(_)) => decErr("Failed to decode as JubjubPoint")
  rule #decJubjub(ptOk(Q)) => decOk(jubjubPoint(Q)) requires inSubgroup(#jubjub, Q)
  rule #decJubjub(ptOk(Q)) => decErr("Failed to decode as JubjubPoint") requires notBool inSubgroup(#jubjub, Q)

  // JubjubScalar: one element below 2^254; its low 252 bits must be a canonical
  // scalar (AssignedScalarOfNativeCurve::from_public_input).
  rule decodeValue(ListItem(F), jubjubScalar()) => decOk(jubjubScalar(lowBits(F, 252)))
    requires fitsBits(F, 254) andBool lowBits(F, 252) <Int #rJ
  rule decodeValue(ListItem(F), jubjubScalar()) => decErr("Failed to decode as JubjubScalar")
    requires notBool (fitsBits(F, 254) andBool lowBits(F, 252) <Int #rJ)
  rule decodeValue(Fs, jubjubScalar()) => decErr("Failed to decode as JubjubScalar") requires size(Fs) =/=Int 1

  rule decodeValue(Fs, secp256k1Base())   => #decField(decField64(Fs, #k256P), secp256k1Base())
  rule decodeValue(Fs, secp256k1Scalar()) => #decField(decField64(Fs, #k256N), secp256k1Scalar())
  rule decodeValue(Fs, secp256r1Base())   => #decField(decField64(Fs, #p256P), secp256r1Base())
  rule decodeValue(Fs, secp256r1Scalar()) => #decField(decField64(Fs, #p256N), secp256r1Scalar())
  rule decodeValue(Fs, curve25519Base())  => #decField(decField64(Fs, #c25519P), curve25519Base())
  rule decodeValue(Fs, curve25519Scalar())=> #decField(decForeign(Fs, #c25519L, 51, 5), curve25519Scalar())
  syntax DecodeResult ::= #decField(Int, IrType) [function, total]
  rule #decField(-1, T) => decErr("Failed to decode as " +String typeName(T))
  rule #decField(X, secp256k1Base())    => decOk(secp256k1Base(X))    requires X >=Int 0
  rule #decField(X, secp256k1Scalar())  => decOk(secp256k1Scalar(X))  requires X >=Int 0
  rule #decField(X, secp256r1Base())    => decOk(secp256r1Base(X))    requires X >=Int 0
  rule #decField(X, secp256r1Scalar())  => decOk(secp256r1Scalar(X))  requires X >=Int 0
  rule #decField(X, curve25519Base())   => decOk(curve25519Base(X))   requires X >=Int 0
  rule #decField(X, curve25519Scalar()) => decOk(curve25519Scalar(X)) requires X >=Int 0
  rule #decField(_, T) => decErr("Failed to decode as " +String typeName(T)) [owise]

  // Weierstrass points: 5 elements; last == 1 means identity.
  rule decodeValue(Fs, secp256k1Point()) => #decWPoint(Fs, #secp256k1, secp256k1Point())
  rule decodeValue(Fs, secp256r1Point()) => #decWPoint(Fs, #secp256r1, secp256r1Point())
  syntax DecodeResult ::= #decWPoint(List, Curve, IrType) [function, total]
  rule #decWPoint(Fs, C, T) => decErr("Failed to decode as " +String typeName(T)) requires size(Fs) =/=Int 5
  rule #decWPoint(Fs, C, T) => decOk(#wrapPoint(inf(), T)) requires size(Fs) ==Int 5 andBool Fs[4] ==K 1
  rule #decWPoint(Fs, C, T) => #decWPointXY(decField64(range(Fs, 0, 3), curveField(C)), decField64(range(Fs, 2, 1), curveField(C)), C, T)
    requires size(Fs) ==Int 5 andBool Fs[4] =/=K 1
  syntax DecodeResult ::= #decWPointXY(Int, Int, Curve, IrType) [function, total]
  rule #decWPointXY(X, Y, C, T) => decErr("Failed to decode as " +String typeName(T)) requires X <Int 0 orBool Y <Int 0
  rule #decWPointXY(X, Y, C, T) => #decPointResult(fromXY(C, X, Y), C, T) requires X >=Int 0 andBool Y >=Int 0
  syntax DecodeResult ::= #decPointResult(PointResult, Curve, IrType) [function, total]
  rule #decPointResult(ptErr(_), _, T) => decErr("Failed to decode as " +String typeName(T))
  rule #decPointResult(ptOk(Q), C, T) => decOk(#wrapPoint(Q, T)) requires inSubgroup(C, Q)
  rule #decPointResult(ptOk(Q), C, T) => decErr("Failed to decode as " +String typeName(T)) requires notBool inSubgroup(C, Q)

  // Curve25519 points: 4 elements, exact curve check then subgroup check.
  rule decodeValue(Fs, curve25519Point()) => decErr("Failed to decode as Curve25519Point") requires size(Fs) =/=Int 4
  rule decodeValue(Fs, curve25519Point())
    => #decWPointXY(decField64(range(Fs, 0, 2), #c25519P), decField64(range(Fs, 2, 0), #c25519P), #curve25519, curve25519Point())
    requires size(Fs) ==Int 4

  syntax Value ::= #wrapPoint(Point, IrType) [function, total]
  rule #wrapPoint(Q, secp256k1Point())  => secp256k1Point(Q)
  rule #wrapPoint(Q, secp256r1Point())  => secp256r1Point(Q)
  rule #wrapPoint(Q, curve25519Point()) => curve25519Point(Q)
  rule #wrapPoint(Q, jubjubPoint())     => jubjubPoint(Q)
  rule #wrapPoint(_, _)                 => native(0) [owise]

  // Strict decoding re-encodes the value and demands the original elements
  // (midnight-zkir 2ffe2d1 `decode_offcircuit`); 92e8bdd3 does not check.
  syntax DecodeResult ::= decodeStrict(List, IrType, Bool) [function, total]
  rule decodeStrict(L, T, false) => decodeValue(L, T)
  rule decodeStrict(L, T, true) => #canonical(decodeValue(L, T), L, T)
  syntax DecodeResult ::= #canonical(DecodeResult, List, IrType) [function, total]
  rule #canonical(decErr(S), _, _) => decErr(S)
  rule #canonical(decPanic(S), _, _) => decPanic(S)
  rule #canonical(decOk(V), L, _) => decOk(V) requires encodeValue(V) ==K L
  rule #canonical(decOk(V), L, T) => decErr("The encoded value of type " +String typeName(T) +String " is not in canonical form") requires encodeValue(V) =/=K L


  // --- Bytes32 helpers ------------------------------------------------------------------
  // Canonical little-endian 32-byte form of a field element (into_bytes32).
  syntax Bytes ::= intToBytes32(Int) [function, total]
  rule intToBytes32(X) => Int2Bytes(32, X, LE) requires X >=Int 0
  rule intToBytes32(_) => padRightBytes(.Bytes, 32, 0) [owise]
  syntax Int ::= bytes32ToInt(Bytes) [function, total]
  rule bytes32ToInt(B) => Bytes2Int(B, LE, Unsigned)
endmodule
