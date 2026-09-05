// Prime-field arithmetic over K's arbitrary-precision Int (plan decision
// CLM-0709). Every function takes the modulus explicitly; results are kept in
// canonical range [0, p) with modInt (Euclidean, never negative).
//
// Moduli (midnight-curves 0.3.1):
//   r      BLS12-381 scalar field = ZKIR native field = Jubjub base field
//   rJ     Jubjub scalar field (prime-order subgroup order)
//   k256P  secp256k1 base field, k256N its group order
//   p256P  secp256r1 base field, p256N its group order
//   c25519P = 2^255 - 19, c25519L = 2^252 + 27742317777372353535851937790883648493
module ZKIR-FIELD
  imports INT
  imports BOOL
  imports LIST
  imports K-EQUAL

  syntax Int ::= "#r"        [function, total]
               | "#rJ"       [function, total]
               | "#k256P"    [function, total]
               | "#k256N"    [function, total]
               | "#p256P"    [function, total]
               | "#p256N"    [function, total]
               | "#c25519P"  [function, total]
               | "#c25519L"  [function, total]
  rule #r       => 52435875175126190479447740508185965837690552500527637822603658699938581184513
  rule #rJ      => 6554484396890773809930967563523245729705921265872317281365359162392183254199
  rule #k256P   => 115792089237316195423570985008687907853269984665640564039457584007908834671663
  rule #k256N   => 115792089237316195423570985008687907852837564279074904382605163141518161494337
  rule #p256P   => 115792089210356248762697446949407573530086143415290314195533631308867097853951
  rule #p256N   => 115792089210356248762697446949407573529996955224135760342422259061068512044369
  rule #c25519P => 57896044618658097711785492504343953926634992332820282019728792003956564819949
  rule #c25519L => 7237005577332262213973186563042994240857116359379907606001950938285454250989

  syntax Int ::= fmod(Int, Int)         [function, total]
               | fadd(Int, Int, Int)    [function, total]
               | fsub(Int, Int, Int)    [function, total]
               | fmul(Int, Int, Int)    [function, total]
               | fneg(Int, Int)         [function, total]
               | finv(Int, Int)         [function, total]   // inverse by Fermat; finv(0) = 0
               | fdiv(Int, Int, Int)    [function, total]
               | fpow(Int, Int, Int)    [function, total]
               | fsq(Int, Int)          [function, total]
  rule fmod(A, P)     => A modInt P
  rule fadd(A, B, P)  => (A +Int B) modInt P
  rule fsub(A, B, P)  => (A -Int B) modInt P
  rule fmul(A, B, P)  => (A *Int B) modInt P
  rule fneg(A, P)     => (0 -Int A) modInt P
  rule finv(A, P)     => 0 requires A modInt P ==Int 0
  rule finv(A, P)     => (A modInt P) ^%Int (P -Int 2) P requires A modInt P =/=Int 0
  rule fdiv(A, B, P)  => fmul(A, finv(B, P), P)
  rule fpow(A, E, P)  => (A modInt P) ^%Int E P requires E >=Int 0
  rule fpow(_, E, _)  => 0 requires E <Int 0
  rule fsq(A, P)      => fmul(A, A, P)

  // Legendre symbol as 0, 1 or P-1 (i.e. -1); 0 counts as a residue, as in
  // midnight-curves `ct_quadratic_non_residue` (legendre == -1).
  syntax Int ::= legendre(Int, Int) [function, total]
  rule legendre(A, P) => fpow(A, (P -Int 1) /Int 2, P)
  syntax Bool ::= isQuadraticNonResidue(Int, Int) [function, total]
  rule isQuadraticNonResidue(A, P) => legendre(A, P) ==Int P -Int 1

  syntax Bool ::= isOddInt(Int) [function, total]
  rule isOddInt(A) => A modInt 2 ==Int 1

  // --- Square roots: Tonelli-Shanks ---------------------------------------
  // fsqrt(A, P) is sqrtOk(X) with X*X = A mod P for some root, or noSqrt().
  // Callers that need a specific root fix the parity themselves, as the Rust
  // code does (Jubjub decompression, SVDW map).
  syntax SqrtResult ::= sqrtOk(Int) | noSqrt()
  syntax SqrtResult ::= fsqrt(Int, Int) [function, total]
  rule fsqrt(A, P) => sqrtOk(0) requires A modInt P ==Int 0
  rule fsqrt(A, P) => noSqrt() requires A modInt P =/=Int 0 andBool legendre(A, P) =/=Int 1
  rule fsqrt(A, P) => #ts(A modInt P, P, #twoAdicity(P -Int 1, 0), #oddPart(P -Int 1), #nonResidue(P, 2))
    requires A modInt P =/=Int 0 andBool legendre(A, P) ==Int 1

  syntax Int ::= #twoAdicity(Int, Int) [function, total]   // #twoAdicity(N, 0) = s with N = 2^s * odd
  rule #twoAdicity(N, S) => #twoAdicity(N /Int 2, S +Int 1) requires N >Int 0 andBool N modInt 2 ==Int 0
  rule #twoAdicity(N, S) => S requires N <=Int 0 orBool N modInt 2 ==Int 1
  syntax Int ::= #oddPart(Int) [function, total]
  rule #oddPart(N) => #oddPart(N /Int 2) requires N >Int 0 andBool N modInt 2 ==Int 0
  rule #oddPart(N) => N requires N <=Int 0 orBool N modInt 2 ==Int 1
  syntax Int ::= #nonResidue(Int, Int) [function, total]   // smallest quadratic non-residue >= Z
  rule #nonResidue(P, Z) => Z requires isQuadraticNonResidue(Z, P)
  rule #nonResidue(P, Z) => #nonResidue(P, Z +Int 1) requires notBool isQuadraticNonResidue(Z, P) andBool Z <Int P
  rule #nonResidue(P, Z) => 0 requires Z >=Int P

  // #ts(A, P, S, Q, Z): standard loop with M = S, c = Z^Q, t = A^Q, R = A^((Q+1)/2)
  syntax SqrtResult ::= #ts(Int, Int, Int, Int, Int) [function, total]
  rule #ts(A, P, S, Q, Z) => #tsLoop(P, S, fpow(Z, Q, P), fpow(A, Q, P), fpow(A, (Q +Int 1) /Int 2, P))
  syntax SqrtResult ::= #tsLoop(Int, Int, Int, Int, Int) [function, total]   // (P, M, c, t, R)
  rule #tsLoop(_, _, _, 0, _) => noSqrt()
  rule #tsLoop(_, _, _, 1, R) => sqrtOk(R)
  rule #tsLoop(P, M, C, T, R) => #tsStep(P, M, C, T, R, #tsOrder(P, T, 0, M))
    requires T =/=Int 0 andBool T =/=Int 1
  // least i with t^(2^i) = 1
  syntax Int ::= #tsOrder(Int, Int, Int, Int) [function, total]
  rule #tsOrder(_, 1, I, _) => I
  rule #tsOrder(P, T, I, M) => #tsOrder(P, fsq(T, P), I +Int 1, M) requires T =/=Int 1 andBool I <Int M
  rule #tsOrder(_, T, I, M) => I requires T =/=Int 1 andBool I >=Int M
  syntax SqrtResult ::= #tsStep(Int, Int, Int, Int, Int, Int) [function, total]
  rule #tsStep(P, M, C, T, R, I) => noSqrt() requires I >=Int M
  rule #tsStep(P, M, C, T, R, I) => #tsLoop(P, I, fsq(fpow(C, 2 ^Int (M -Int I -Int 1), P), P),
                                            fmul(T, fsq(fpow(C, 2 ^Int (M -Int I -Int 1), P), P), P),
                                            fmul(R, fpow(C, 2 ^Int (M -Int I -Int 1), P), P))
    requires I <Int M

  // --- Bit and byte helpers ------------------------------------------------
  // Little-endian bit I of A (0 or 1).
  syntax Int ::= bitAt(Int, Int) [function, total]
  rule bitAt(A, I) => (A >>Int I) &Int 1 requires I >=Int 0 andBool A >=Int 0
  rule bitAt(_, _) => 0 [owise]
  // Low N bits and the rest.
  syntax Int ::= lowBits(Int, Int) [function, total]
  rule lowBits(A, N) => A &Int ((1 <<Int N) -Int 1) requires N >=Int 0 andBool A >=Int 0
  rule lowBits(_, _) => 0 [owise]
  syntax Int ::= highBits(Int, Int) [function, total]   // A >> N
  rule highBits(A, N) => A >>Int N requires N >=Int 0 andBool A >=Int 0
  rule highBits(_, _) => 0 [owise]
  syntax Bool ::= fitsBits(Int, Int) [function, total]   // 0 <= A < 2^N
  rule fitsBits(A, N) => A >=Int 0 andBool A <Int (1 <<Int N) requires N >=Int 0
  rule fitsBits(_, _) => false [owise]
endmodule
