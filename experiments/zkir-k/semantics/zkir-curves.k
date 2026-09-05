// Affine elliptic-curve arithmetic for the four ZKIR curves.
//
//   Jubjub      twisted Edwards, a = -1, over r;   subgroup order rJ, cofactor 8
//   Curve25519  twisted Edwards, a = -1, over 2^255-19; subgroup order c25519L, cofactor 8
//   secp256k1   short Weierstrass y^2 = x^3 + 7 over k256P; prime order k256N
//   secp256r1   short Weierstrass y^2 = x^3 - 3x + b over p256P; prime order p256N
//
// A point is pt(X, Y) or inf(). Edwards curves never use inf(): their identity
// is pt(0, 1). Formulas are the complete unified Edwards addition (d is a
// non-square on both curves) and the textbook affine Weierstrass addition.
// Sources: midnight-curves 0.3.1 (jubjub/curve.rs, curve25519/), curve25519-dalek
// 4.1.3 for the Curve25519 base point, the SEC 2 and FIPS 186-4 generators.
requires "zkir-field.k"

module ZKIR-CURVES
  imports ZKIR-FIELD
  imports STRING

  syntax Point ::= pt(Int, Int) [symbol(pt)]
                 | inf()        [symbol(inf)]

  // --- Curve descriptors --------------------------------------------------------
  // edwards(p, d, order)   weierstrass(p, a, b, order)
  syntax Curve ::= edwards(Int, Int, Int) | weierstrass(Int, Int, Int, Int)

  syntax Curve ::= "#jubjub" [function, total]
                 | "#curve25519" [function, total]
                 | "#secp256k1" [function, total]
                 | "#secp256r1" [function, total]
  // Jubjub d = -(10240/10241) mod r (midnight-curves jubjub/curve.rs EDWARDS_D)
  rule #jubjub     => edwards(#r, fneg(fdiv(10240, 10241, #r), #r), #rJ)
  // Curve25519 d = -(121665/121666) mod p
  rule #curve25519 => edwards(#c25519P, fneg(fdiv(121665, 121666, #c25519P), #c25519P), #c25519L)
  rule #secp256k1  => weierstrass(#k256P, 0, 7, #k256N)
  rule #secp256r1  => weierstrass(#p256P, #p256P -Int 3,
                        41058363725152142129326129780047268409114441015993725554835256314039467401291, #p256N)

  syntax Int ::= curveField(Curve) [function, total]
  rule curveField(edwards(P, _, _)) => P
  rule curveField(weierstrass(P, _, _, _)) => P
  syntax Int ::= curveOrder(Curve) [function, total]
  rule curveOrder(edwards(_, _, N)) => N
  rule curveOrder(weierstrass(_, _, _, N)) => N

  // --- Generators ---------------------------------------------------------------
  // Jubjub: JubjubAffine::generator() (u, v) of midnight-curves; the subgroup
  // generator used by `JubjubSubgroup::generator()` is its cofactor clearing, 8*G.
  syntax Point ::= "#jubjubGenerator" [function, total]
  rule #jubjubGenerator => ecMul(#jubjub,
    pt(44746807950788659978687200207992930935149218647843500701850233404325651525118, 11), 8)
  // Curve25519: ED25519_BASEPOINT_POINT (already in the prime-order subgroup).
  syntax Point ::= "#curve25519Generator" [function, total]
  rule #curve25519Generator => pt(15112221349535400772501151409588531511454012693041857206046113283949847762202,
                                  46316835694926478169428394003475163141307993866256225615783033603165251855960)
  syntax Point ::= "#secp256k1Generator" [function, total]
  rule #secp256k1Generator => pt(55066263022277343669578718895168534326250603453777594175500187360389116729240,
                                 32670510020758816978083085130507043184471273380659243275938904335757337482424)
  syntax Point ::= "#secp256r1Generator" [function, total]
  rule #secp256r1Generator => pt(48439561293906451759052585252797914202762949526041747995844080717082404635286,
                                 36134250956749795798585127919587881956611106672985015071877198253568414405109)

  // --- Membership -----------------------------------------------------------------
  syntax Bool ::= onCurve(Curve, Point) [function, total]
  rule onCurve(_, inf()) => true
  // -x^2 + y^2 = 1 + d x^2 y^2
  rule onCurve(edwards(P, D, _), pt(X, Y))
    => X >=Int 0 andBool X <Int P andBool Y >=Int 0 andBool Y <Int P andBool
       fsub(fsq(Y, P), fsq(X, P), P) ==Int fadd(1, fmul(D, fmul(fsq(X, P), fsq(Y, P), P), P), P)
  rule onCurve(weierstrass(P, A, B, _), pt(X, Y))
    => X >=Int 0 andBool X <Int P andBool Y >=Int 0 andBool Y <Int P andBool
       fsq(Y, P) ==Int fadd(fadd(fmul(fsq(X, P), X, P), fmul(A, X, P), P), B, P)

  syntax Point ::= identity(Curve) [function, total]
  rule identity(edwards(_, _, _)) => pt(0, 1)
  rule identity(weierstrass(_, _, _, _)) => inf()

  syntax Bool ::= isIdentity(Curve, Point) [function, total]
  rule isIdentity(C, Q) => Q ==K identity(C)

  // In the prime-order subgroup: order * Q = identity (Weierstrass groups are
  // prime order, so any curve point qualifies).
  syntax Bool ::= inSubgroup(Curve, Point) [function, total]
  rule inSubgroup(edwards(P, D, N), Q) => isIdentity(edwards(P, D, N), ecMul(edwards(P, D, N), Q, N))
  rule inSubgroup(weierstrass(_, _, _, _), _) => true

  // --- Group law ------------------------------------------------------------------
  syntax Point ::= ecNeg(Curve, Point) [function, total]
  rule ecNeg(edwards(P, _, _), pt(X, Y)) => pt(fneg(X, P), Y)
  rule ecNeg(weierstrass(P, _, _, _), pt(X, Y)) => pt(X, fneg(Y, P))
  rule ecNeg(_, inf()) => inf()

  syntax Point ::= ecAdd(Curve, Point, Point) [function, total]
  rule ecAdd(edwards(P, D, _), pt(X1, Y1), pt(X2, Y2))
    => pt(fdiv(fadd(fmul(X1, Y2, P), fmul(Y1, X2, P), P),
              fadd(1, fmul(D, fmul(fmul(X1, X2, P), fmul(Y1, Y2, P), P), P), P), P),
          fdiv(fadd(fmul(Y1, Y2, P), fmul(X1, X2, P), P),
              fsub(1, fmul(D, fmul(fmul(X1, X2, P), fmul(Y1, Y2, P), P), P), P), P))
  rule ecAdd(edwards(_, _, _), _, inf()) => inf()   // never reached for valid values
  rule ecAdd(edwards(_, _, _), inf(), _) => inf()
  rule ecAdd(weierstrass(_, _, _, _), inf(), Q) => Q
  rule ecAdd(weierstrass(_, _, _, _), Q, inf()) => Q
  rule ecAdd(weierstrass(P, A, B, N), pt(X1, Y1), pt(X2, Y2))
    => ecDouble(weierstrass(P, A, B, N), pt(X1, Y1))
    requires X1 ==Int X2 andBool Y1 ==Int Y2
  rule ecAdd(weierstrass(P, _, _, _), pt(X1, Y1), pt(X2, Y2)) => inf()
    requires X1 ==Int X2 andBool Y1 =/=Int Y2
  rule ecAdd(weierstrass(P, _, _, _), pt(X1, Y1), pt(X2, Y2))
    => #wChord(P, X1, Y1, X2, fdiv(fsub(Y2, Y1, P), fsub(X2, X1, P), P))
    requires X1 =/=Int X2
  syntax Point ::= #wChord(Int, Int, Int, Int, Int) [function, total]   // (P, X1, Y1, X2, lambda)
  rule #wChord(P, X1, Y1, X2, L) => pt(fsub(fsub(fsq(L, P), X1, P), X2, P),
                                       fsub(fmul(L, fsub(X1, fsub(fsub(fsq(L, P), X1, P), X2, P), P), P), Y1, P))

  syntax Point ::= ecDouble(Curve, Point) [function, total]
  rule ecDouble(edwards(P, D, N), Q) => ecAdd(edwards(P, D, N), Q, Q)
  rule ecDouble(weierstrass(_, _, _, _), inf()) => inf()
  rule ecDouble(weierstrass(_, _, _, _), pt(_, 0)) => inf()
  rule ecDouble(weierstrass(P, A, _, _), pt(X, Y))
    => #wChord(P, X, Y, X, fdiv(fadd(fmul(3, fsq(X, P), P), A, P), fmul(2, Y, P), P))
    requires Y =/=Int 0

  // Double-and-add over the bits of K (K >= 0; callers pass canonical scalars).
  syntax Point ::= ecMul(Curve, Point, Int) [function, total]
  rule ecMul(C, _, K) => identity(C) requires K <=Int 0
  rule ecMul(C, Q, K) => #ecMulAcc(C, Q, K, identity(C)) requires K >Int 0
  syntax Point ::= #ecMulAcc(Curve, Point, Int, Point) [function, total]
  rule #ecMulAcc(_, _, 0, Acc) => Acc
  rule #ecMulAcc(C, Q, K, Acc) => #ecMulAcc(C, ecDouble(C, Q), K >>Int 1, ecAdd(C, Acc, Q))
    requires K >Int 0 andBool K modInt 2 ==Int 1
  rule #ecMulAcc(C, Q, K, Acc) => #ecMulAcc(C, ecDouble(C, Q), K >>Int 1, Acc)
    requires K >Int 0 andBool K modInt 2 ==Int 0
  rule #ecMulAcc(_, _, K, Acc) => Acc requires K <Int 0

  // --- Constructors from coordinates -------------------------------------------------
  syntax PointResult ::= ptOk(Point) | ptErr(String)

  // Exact membership test (secp256k1, secp256r1 via SEC1 decoding; Curve25519
  // via the curve equation in midnight-curves Curve25519Affine::from_xy).
  syntax PointResult ::= fromXY(Curve, Int, Int) [function, total]
  rule fromXY(C, X, Y) => ptOk(pt(X, Y)) requires onCurve(C, pt(X, Y))
  rule fromXY(C, X, Y) => ptErr("point is not on the curve") requires notBool onCurve(C, pt(X, Y))

  // Jubjub `from_xy` of midnight-circuits (ecc/curves.rs): compresses (y, parity
  // of x) and decompresses with JubjubAffine::from_bytes, so only the low bit
  // of x is used; the recovered x is the root of (y^2-1)/(1+d y^2) with that
  // parity. Fails when y is not the v-coordinate of any point, or when
  // (ZIP 216) u = 0 with the sign bit set.
  syntax PointResult ::= jubjubFromXY(Int, Int) [function, total]
  rule jubjubFromXY(X, Y) => ptErr("coordinate out of range") requires Y <Int 0 orBool Y >=Int #r orBool X <Int 0 orBool X >=Int #r
  rule jubjubFromXY(X, Y) => #jubjubDecompress(X modInt 2, Y, fsqrt(fdiv(fsub(fsq(Y, #r), 1, #r), fadd(1, fmul(#jubjubD, fsq(Y, #r), #r), #r), #r), #r))
    requires Y >=Int 0 andBool Y <Int #r andBool X >=Int 0 andBool X <Int #r
  syntax Int ::= "#jubjubD" [function, total]
  rule #jubjubD => fneg(fdiv(10240, 10241, #r), #r)
  syntax PointResult ::= #jubjubDecompress(Int, Int, SqrtResult) [function, total]
  rule #jubjubDecompress(_, _, noSqrt()) => ptErr("y is not on Jubjub")
  rule #jubjubDecompress(Sign, Y, sqrtOk(U)) => ptOk(pt(U, Y)) requires U modInt 2 ==Int Sign
  rule #jubjubDecompress(Sign, Y, sqrtOk(U)) => ptErr("invalid Jubjub encoding: -0")
    requires U modInt 2 =/=Int Sign andBool U ==Int 0
  rule #jubjubDecompress(Sign, Y, sqrtOk(U)) => ptOk(pt(fneg(U, #r), Y))
    requires U modInt 2 =/=Int Sign andBool U =/=Int 0
endmodule
