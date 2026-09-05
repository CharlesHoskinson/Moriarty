// Hash primitives used by ZKIR v3, computed concretely.
//
//   poseidonHash(List)      transient_hash: Poseidon sponge over the BLS12-381
//                           scalar field, t = 3, rate 2, R_F = 8, R_P = 60, with
//                           the fixed-length domain (capacity element = input
//                           length). midnight-circuits 7.2.4 hash/poseidon/*.
//   hashToCurve(List)       transient_crypto::hash::hash_to_curve: variable-length
//                           Poseidon sponge (capacity 2^64, length padding),
//                           two squeezes, Shallue-van de Woestijne map to the
//                           Weierstrass form of Jubjub, conversion to Edwards,
//                           cofactor clearing, sum of the two points.
//   sha256(Bytes)           FIPS 180-4.
//   keccak256(Bytes)        Keccak-f[1600], rate 1088, pad 0x01 .. 0x80 (pre-NIST).
//
// The Rust Poseidon CPU code folds partial rounds in pairs ("round skips"); that
// is an algebraic rewrite of the plain round function implemented here, and the
// differential tests against the crate confirm the outputs agree.
requires "zkir-field.k"
requires "zkir-curves.k"
requires "zkir-constants.k"

module ZKIR-HASH
  imports ZKIR-FIELD
  imports ZKIR-CURVES
  imports ZKIR-CONSTANTS
  imports BYTES
  imports LIST

  // ---------------------------------------------------------------------------
  // Poseidon permutation on a 3-element state (s0, s1, s2), all mod r.
  syntax PState ::= ps(Int, Int, Int)

  syntax Int ::= sbox(Int) [function, total]
  rule sbox(X) => fmul(fsq(fsq(X, #r), #r), X, #r)

  // new[i] = rc[i] + sum_j MDS[i][j] * s[j]
  syntax PState ::= linear(PState, Int) [function, total]   // (state, round index for RC[round+1]; -1 = zero constants)
  rule linear(ps(S0, S1, S2), R) => ps(
      fadd(#rc(R, 0), fadd(fadd(fmul(poseidonMDS(0, 0), S0, #r), fmul(poseidonMDS(0, 1), S1, #r), #r), fmul(poseidonMDS(0, 2), S2, #r), #r), #r),
      fadd(#rc(R, 1), fadd(fadd(fmul(poseidonMDS(1, 0), S0, #r), fmul(poseidonMDS(1, 1), S1, #r), #r), fmul(poseidonMDS(1, 2), S2, #r), #r), #r),
      fadd(#rc(R, 2), fadd(fadd(fmul(poseidonMDS(2, 0), S0, #r), fmul(poseidonMDS(2, 1), S1, #r), #r), fmul(poseidonMDS(2, 2), S2, #r), #r), #r))
  syntax Int ::= #rc(Int, Int) [function, total]
  rule #rc(R, J) => poseidonRC(R +Int 1, J) requires R >=Int 0 andBool R <Int 67
  rule #rc(_, _) => 0 [owise]

  syntax PState ::= fullRound(PState, Int) [function, total]
  rule fullRound(ps(S0, S1, S2), R) => linear(ps(sbox(S0), sbox(S1), sbox(S2)), R)
  syntax PState ::= partialRound(PState, Int) [function, total]
  rule partialRound(ps(S0, S1, S2), R) => linear(ps(S0, S1, sbox(S2)), R)

  syntax PState ::= permute(PState) [function, total]
  rule permute(ps(S0, S1, S2)) => #rounds(ps(fadd(S0, poseidonRC(0, 0), #r), fadd(S1, poseidonRC(0, 1), #r), fadd(S2, poseidonRC(0, 2), #r)), 0)
  syntax PState ::= #rounds(PState, Int) [function, total]
  rule #rounds(S, R) => #rounds(fullRound(S, R), R +Int 1) requires R <Int 4
  rule #rounds(S, R) => #rounds(partialRound(S, R), R +Int 1) requires R >=Int 4 andBool R <Int 64
  rule #rounds(S, R) => #rounds(fullRound(S, R), R +Int 1) requires R >=Int 64 andBool R <Int 68
  rule #rounds(S, R) => S requires R >=Int 68

  // Absorb a queue of elements in chunks of two, permuting after each chunk.
  syntax PState ::= absorbAll(PState, List) [function, total]
  rule absorbAll(S, .List) => S
  rule absorbAll(ps(S0, S1, S2), ListItem(A) ListItem(B) Rest) => absorbAll(permute(ps(fadd(S0, A, #r), fadd(S1, B, #r), S2)), Rest)
  rule absorbAll(ps(S0, S1, S2), ListItem(A)) => permute(ps(fadd(S0, A, #r), S1, S2))

  // Fixed-length Poseidon hash (transient_hash).
  syntax Int ::= poseidonHash(List) [function, total]
  rule poseidonHash(Xs) => #ps0(absorbAll(ps(0, 0, size(Xs) modInt #r), Xs))
  syntax Int ::= #ps0(PState) [function, total]
  rule #ps0(ps(S0, _, _)) => S0
  syntax Int ::= #ps1(PState) [function, total]
  rule #ps1(ps(_, S1, _)) => S1

  // transient_commit(values, opening) = poseidonHash([opening] ++ values)
  syntax Int ::= transientCommit(List, Int) [function, total]
  rule transientCommit(Vs, Rand) => poseidonHash(ListItem(Rand) Vs)

  // Variable-length sponge used by hash_to_curve: capacity 2^64, the input
  // length appended as padding, then two squeezes (register 0 and 1).
  syntax PState ::= varLenSponge(List) [function, total]
  rule varLenSponge(Xs) => absorbAll(ps(0, 0, 18446744073709551616), Xs ListItem(size(Xs)))

  // ---------------------------------------------------------------------------
  // Hash to curve (Jubjub subgroup).
  syntax Point ::= hashToCurve(List) [function, total]
  rule hashToCurve(Xs) => #htc(varLenSponge(Xs))
  syntax Point ::= #htc(PState) [function, total]
  rule #htc(ps(X1, X2, _)) => ecAdd(#jubjub, mapToCurve(X1), mapToCurve(X2))

  // SVDW constants (mtc_params.rs), all mod r
  syntax Int ::= "#svdwC1" [function, total]   // g(Z)
  rule #svdwC1 => #g(#svdwZ)
  syntax Int ::= #g(Int) [function, total]
  rule #g(X) => fadd(fadd(fmul(fmul(X, X, #r), X, #r), fmul(#svdwA, X, #r), #r), #svdwB, #r)
  syntax Int ::= "#svdwC2" [function, total]   // -Z / 2
  rule #svdwC2 => fneg(fmul(#svdwZ, finv(2, #r), #r), #r)
  syntax Int ::= "#svdwC3" [function, total]   // sqrt(-g(Z) * (3 Z^2 + 4 A)), even root
  rule #svdwC3 => #evenRoot(fsqrt(fneg(fmul(#svdwC1, fadd(fmul(3, fsq(#svdwZ, #r), #r), fmul(4, #svdwA, #r), #r), #r), #r), #r))
  syntax Int ::= #evenRoot(SqrtResult) [function, total]
  rule #evenRoot(sqrtOk(S)) => S requires S modInt 2 ==Int 0
  rule #evenRoot(sqrtOk(S)) => fneg(S, #r) requires S modInt 2 ==Int 1
  rule #evenRoot(noSqrt()) => 0
  syntax Int ::= "#svdwC4" [function, total]   // -4 g(Z) / (3 Z^2 + 4 A)
  rule #svdwC4 => fneg(fmul(fmul(4, #svdwC1, #r), finv(fadd(fmul(3, fsq(#svdwZ, #r), #r), fmul(4, #svdwA, #r), #r), #r), #r), #r)

  syntax Point ::= mapToCurve(Int) [function, total]
  rule mapToCurve(U) => ecMul(#jubjub, #edFromMont(#wToMont(#svdw(U))), 8)

  // svdw_map_to_curve: returns pt(x, y) on the Weierstrass form
  syntax Point ::= #svdw(Int) [function, total]
  rule #svdw(U) => #svdw1(U, fmul(fsq(U, #r), #svdwC1, #r))
  syntax Point ::= #svdw1(Int, Int) [function, total]   // (u, tv1 = u^2 c1)
  rule #svdw1(U, T) => #svdw2(U, fadd(1, T, #r), fsub(1, T, #r))
  syntax Point ::= #svdw2(Int, Int, Int) [function, total]   // (u, tv2 = 1 + t, tv1 = 1 - t)
  rule #svdw2(U, TV2, TV1) => #svdw3(U, TV2, TV1, finv(fmul(TV1, TV2, #r), #r))
  syntax Point ::= #svdw3(Int, Int, Int, Int) [function, total]   // (u, tv2, tv1, tv3 = inv(tv1 tv2))
  rule #svdw3(U, TV2, TV1, TV3) => #svdw4(U, TV2, TV3, fmul(fmul(fmul(U, TV1, #r), TV3, #r), #svdwC3, #r))
  syntax Point ::= #svdw4(Int, Int, Int, Int) [function, total]   // (u, tv2, tv3, tv4)
  rule #svdw4(U, TV2, TV3, TV4) => #svdwPick(U,
      fsub(#svdwC2, TV4, #r), fadd(#svdwC2, TV4, #r),
      fadd(fmul(fsq(fmul(fsq(TV2, #r), TV3, #r), #r), #svdwC4, #r), #svdwZ, #r))
  // e1 = g(x1) residue; e2 = g(x2) residue and not e1; x = e1 ? x1 : e2 ? x2 : x3
  syntax Point ::= #svdwPick(Int, Int, Int, Int) [function, total]
  rule #svdwPick(U, X1, X2, X3) => #svdwY(U, X1) requires notBool isQuadraticNonResidue(#g(X1), #r)
  rule #svdwPick(U, X1, X2, X3) => #svdwY(U, X2) requires isQuadraticNonResidue(#g(X1), #r) andBool notBool isQuadraticNonResidue(#g(X2), #r)
  rule #svdwPick(U, X1, X2, X3) => #svdwY(U, X3) requires isQuadraticNonResidue(#g(X1), #r) andBool isQuadraticNonResidue(#g(X2), #r)
  syntax Point ::= #svdwY(Int, Int) [function, total]
  rule #svdwY(U, X) => #svdwSign(U, X, fsqrt(#g(X), #r))
  syntax Point ::= #svdwSign(Int, Int, SqrtResult) [function, total]
  rule #svdwSign(U, X, sqrtOk(Y)) => pt(X, Y) requires (U modInt 2) ==Int (Y modInt 2)
  rule #svdwSign(U, X, sqrtOk(Y)) => pt(X, fneg(Y, #r)) requires (U modInt 2) =/=Int (Y modInt 2)
  rule #svdwSign(_, X, noSqrt()) => pt(X, 0)   // unreachable: g(x) is a residue by construction

  // weierstrass_to_montgomery: x' = x K - J/3, y' = y K
  syntax Point ::= #wToMont(Point) [function, total]
  rule #wToMont(pt(X, Y)) => pt(fsub(fmul(X, #montK, #r), fmul(#montJ, finv(3, #r), #r), #r), fmul(Y, #montK, #r))
  rule #wToMont(inf()) => inf()
  // montgomery_to_edwards: tv1 = x + 1; tv2 = inv(tv1 y); x' = tv2 tv1 x; y' = tv2 y (x - 1); y' = 1 if tv2 = 0
  syntax Point ::= #edFromMont(Point) [function, total]
  rule #edFromMont(pt(X, Y)) => #edFromMont2(X, Y, fadd(X, 1, #r), finv(fmul(fadd(X, 1, #r), Y, #r), #r))
  rule #edFromMont(inf()) => pt(0, 1)
  syntax Point ::= #edFromMont2(Int, Int, Int, Int) [function, total]
  rule #edFromMont2(X, Y, TV1, TV2) => pt(fmul(fmul(TV2, TV1, #r), X, #r), fmul(fmul(TV2, Y, #r), fsub(X, 1, #r), #r)) requires TV2 =/=Int 0
  rule #edFromMont2(X, Y, TV1, TV2) => pt(fmul(fmul(TV2, TV1, #r), X, #r), 1) requires TV2 ==Int 0

  // ---------------------------------------------------------------------------
  // SHA-256
  syntax Int ::= "#m32" [function, total]
  rule #m32 => 4294967295
  syntax Int ::= rotr32(Int, Int) [function, total]
  rule rotr32(X, N) => ((X >>Int N) |Int (X <<Int (32 -Int N))) &Int #m32
  syntax Int ::= add32(Int, Int) [function, total]
  rule add32(A, B) => (A +Int B) &Int #m32

  syntax List ::= "#shaKs" [function, total]
  rule #shaKs => ListItem(1116352408) ListItem(1899447441) ListItem(3049323471) ListItem(3921009573) ListItem(961987163) ListItem(1508970993) ListItem(2453635748) ListItem(2870763221) ListItem(3624381080) ListItem(310598401) ListItem(607225278) ListItem(1426881987) ListItem(1925078388) ListItem(2162078206) ListItem(2614888103) ListItem(3248222580) ListItem(3835390401) ListItem(4022224774) ListItem(264347078) ListItem(604807628) ListItem(770255983) ListItem(1249150122) ListItem(1555081692) ListItem(1996064986) ListItem(2554220882) ListItem(2821834349) ListItem(2952996808) ListItem(3210313671) ListItem(3336571891) ListItem(3584528711) ListItem(113926993) ListItem(338241895) ListItem(666307205) ListItem(773529912) ListItem(1294757372) ListItem(1396182291) ListItem(1695183700) ListItem(1986661051) ListItem(2177026350) ListItem(2456956037) ListItem(2730485921) ListItem(2820302411) ListItem(3259730800) ListItem(3345764771) ListItem(3516065817) ListItem(3600352804) ListItem(4094571909) ListItem(275423344) ListItem(430227734) ListItem(506948616) ListItem(659060556) ListItem(883997877) ListItem(958139571) ListItem(1322822218) ListItem(1537002063) ListItem(1747873779) ListItem(1955562222) ListItem(2024104815) ListItem(2227730452) ListItem(2361852424) ListItem(2428436474) ListItem(2756734187) ListItem(3204031479) ListItem(3329325298)
  syntax Int ::= shaK(Int) [function, total]
  rule shaK(I) => {#shaKs[I]}:>Int

  syntax Bytes ::= sha256Bytes(Bytes) [function, total]
  rule sha256Bytes(M) => #shaBlocks(#shaPad(M), 0,
      ListItem(1779033703) ListItem(3144134277) ListItem(1013904242) ListItem(2773480762)
      ListItem(1359893119) ListItem(2600822924) ListItem(528734635) ListItem(1541459225))
  syntax Bytes ::= #shaPad(Bytes) [function, total]
  rule #shaPad(M) => padRightBytes(M +Bytes Int2Bytes(1, 128, BE), ((lengthBytes(M) +Int 9 +Int 63) /Int 64) *Int 64 -Int 8, 0) +Bytes Int2Bytes(8, lengthBytes(M) *Int 8, BE)
  syntax Bytes ::= #shaBlocks(Bytes, Int, List) [function, total]   // (padded, offset, H)
  rule #shaBlocks(P, Off, H) => #shaOut(H) requires Off >=Int lengthBytes(P)
  rule #shaBlocks(P, Off, H) => #shaBlocks(P, Off +Int 64, #shaCompress(#shaSchedule(substrBytes(P, Off, Off +Int 64), 0, .List), H))
    requires Off <Int lengthBytes(P)
  syntax Bytes ::= #shaOut(List) [function, total]
  rule #shaOut(.List) => .Bytes
  rule #shaOut(ListItem(W) Ws) => Int2Bytes(4, W, BE) +Bytes #shaOut(Ws)

  // message schedule W[0..63]
  syntax List ::= #shaSchedule(Bytes, Int, List) [function, total]
  rule #shaSchedule(B, T, W) => #shaSchedule(B, T +Int 1, W ListItem(Bytes2Int(substrBytes(B, 4 *Int T, 4 *Int T +Int 4), BE, Unsigned)))
    requires T <Int 16
  rule #shaSchedule(B, T, W) => #shaSchedule(B, T +Int 1, W ListItem(add32(add32(#s1({W[T -Int 2]}:>Int), {W[T -Int 7]}:>Int), add32(#s0({W[T -Int 15]}:>Int), {W[T -Int 16]}:>Int))))
    requires T >=Int 16 andBool T <Int 64
  rule #shaSchedule(_, T, W) => W requires T >=Int 64
  syntax Int ::= #s0(Int) [function, total]
  rule #s0(X) => rotr32(X, 7) xorInt rotr32(X, 18) xorInt (X >>Int 3)
  syntax Int ::= #s1(Int) [function, total]
  rule #s1(X) => rotr32(X, 17) xorInt rotr32(X, 19) xorInt (X >>Int 10)
  syntax Int ::= #S0(Int) [function, total]
  rule #S0(X) => rotr32(X, 2) xorInt rotr32(X, 13) xorInt rotr32(X, 22)
  syntax Int ::= #S1(Int) [function, total]
  rule #S1(X) => rotr32(X, 6) xorInt rotr32(X, 11) xorInt rotr32(X, 25)

  syntax List ::= #shaCompress(List, List) [function, total]   // (W, H) -> H'
  rule #shaCompress(W, H) => #shaAddH(#shaRound(W, 0, H), H)
  // working variables carried as a List [a,b,c,d,e,f,g,h]
  syntax List ::= #shaRound(List, Int, List) [function, total]
  rule #shaRound(_, T, V) => V requires T >=Int 64
  rule #shaRound(W, T, ListItem(A) ListItem(B) ListItem(C) ListItem(D) ListItem(E) ListItem(F) ListItem(G) ListItem(HH))
    => #shaRound(W, T +Int 1, #shaNext(A, B, C, D, E, F, G,
         add32(add32(add32(add32(HH, #S1(E)), (E &Int F) xorInt ((E xorInt #m32) &Int G)), shaK(T)), {W[T]}:>Int),
         add32(#S0(A), (A &Int B) xorInt (A &Int C) xorInt (B &Int C))))
    requires T <Int 64
  syntax List ::= #shaNext(Int, Int, Int, Int, Int, Int, Int, Int, Int) [function, total]   // (a..g, T1, T2)
  rule #shaNext(A, B, C, D, E, F, G, T1, T2)
    => ListItem(add32(T1, T2)) ListItem(A) ListItem(B) ListItem(C) ListItem(add32(D, T1)) ListItem(E) ListItem(F) ListItem(G)
  syntax List ::= #shaAddH(List, List) [function, total]
  rule #shaAddH(.List, .List) => .List
  rule #shaAddH(ListItem(X) Xs, ListItem(Y) Ys) => ListItem(add32(X, Y)) #shaAddH(Xs, Ys)
  rule #shaAddH(_, _) => .List [owise]

  // ---------------------------------------------------------------------------
  // Keccak-256. State: List of 25 lanes, index x + 5*y, 64-bit little-endian lanes.
  syntax Int ::= "#m64" [function, total]
  rule #m64 => 18446744073709551615
  syntax Int ::= rotl64(Int, Int) [function, total]
  rule rotl64(X, 0) => X
  rule rotl64(X, N) => ((X <<Int N) |Int (X >>Int (64 -Int N))) &Int #m64 requires N >Int 0

  syntax List ::= "#keccakRCs" [function, total]
  rule #keccakRCs => ListItem(1) ListItem(32898) ListItem(9223372036854808714) ListItem(9223372039002292224) ListItem(32907) ListItem(2147483649) ListItem(9223372039002292353) ListItem(9223372036854808585) ListItem(138) ListItem(136) ListItem(2147516425) ListItem(2147483658) ListItem(2147516555) ListItem(9223372036854775947) ListItem(9223372036854808713) ListItem(9223372036854808579) ListItem(9223372036854808578) ListItem(9223372036854775936) ListItem(32778) ListItem(9223372039002259466) ListItem(9223372039002292353) ListItem(9223372036854808704) ListItem(2147483649) ListItem(9223372039002292232)
  syntax Int ::= keccakRC(Int) [function, total]
  rule keccakRC(I) => {#keccakRCs[I]}:>Int
  syntax List ::= "#keccakRots" [function, total]
  rule #keccakRots => ListItem(0) ListItem(1) ListItem(62) ListItem(28) ListItem(27) ListItem(36) ListItem(44) ListItem(6) ListItem(55) ListItem(20) ListItem(3) ListItem(10) ListItem(43) ListItem(25) ListItem(39) ListItem(41) ListItem(45) ListItem(15) ListItem(21) ListItem(8) ListItem(18) ListItem(2) ListItem(61) ListItem(56) ListItem(14)
  syntax Int ::= keccakRot(Int) [function, total]   // rotation offset for lane index x + 5y
  rule keccakRot(I) => {#keccakRots[I]}:>Int

  syntax Int ::= lane(List, Int, Int) [function, total]   // (state, x, y)
  rule lane(S, X, Y) => {S[(X modInt 5) +Int 5 *Int (Y modInt 5)]}:>Int

  syntax Bytes ::= keccak256Bytes(Bytes) [function, total]
  rule keccak256Bytes(M) => substrBytes(#kOut(#kAbsorb(#kPad(M), 0, #kZero(25)), 0, .Bytes), 0, 32)
  syntax Bytes ::= #kPad(Bytes) [function, total]
  rule #kPad(M) => #kPadEnd(padRightBytes(M +Bytes Int2Bytes(1, 1, BE), ((lengthBytes(M) +Int 1 +Int 135) /Int 136) *Int 136, 0))
  syntax Bytes ::= #kPadEnd(Bytes) [function, total]
  rule #kPadEnd(P) => replaceAtBytes(P, lengthBytes(P) -Int 1, Int2Bytes(1, P[lengthBytes(P) -Int 1] |Int 128, BE))
  syntax List ::= #kZero(Int) [function, total]
  rule #kZero(0) => .List
  rule #kZero(N) => ListItem(0) #kZero(N -Int 1) requires N >Int 0
  rule #kZero(N) => .List requires N <Int 0

  syntax List ::= #kAbsorb(Bytes, Int, List) [function, total]
  rule #kAbsorb(P, Off, S) => S requires Off >=Int lengthBytes(P)
  rule #kAbsorb(P, Off, S) => #kAbsorb(P, Off +Int 136, keccakF(#kXorBlock(S, substrBytes(P, Off, Off +Int 136), 0)))
    requires Off <Int lengthBytes(P)
  syntax List ::= #kXorBlock(List, Bytes, Int) [function, total]
  rule #kXorBlock(S, B, I) => S requires I >=Int 17
  rule #kXorBlock(S, B, I) => #kXorBlock(S[I <- ({S[I]}:>Int xorInt Bytes2Int(substrBytes(B, 8 *Int I, 8 *Int I +Int 8), LE, Unsigned))], B, I +Int 1)
    requires I <Int 17
  syntax Bytes ::= #kOut(List, Int, Bytes) [function, total]
  rule #kOut(S, I, Acc) => Acc requires I >=Int 4
  rule #kOut(S, I, Acc) => #kOut(S, I +Int 1, Acc +Bytes Int2Bytes(8, {S[I]}:>Int, LE)) requires I <Int 4

  syntax List ::= keccakF(List) [function, total]
  rule keccakF(S) => #kRounds(S, 0)
  syntax List ::= #kRounds(List, Int) [function, total]
  rule #kRounds(S, R) => S requires R >=Int 24
  rule #kRounds(S, R) => #kRounds(#kIota(#kChi(#kRhoPi(#kTheta(S))), R), R +Int 1) requires R <Int 24

  // theta
  syntax List ::= #kTheta(List) [function, total]
  rule #kTheta(S) => #kThetaApply(S, #kColumns(S, 0), 0)
  syntax List ::= #kColumns(List, Int) [function, total]   // C[x] = xor of column x
  rule #kColumns(_, X) => .List requires X >=Int 5
  rule #kColumns(S, X) => ListItem(lane(S, X, 0) xorInt lane(S, X, 1) xorInt lane(S, X, 2) xorInt lane(S, X, 3) xorInt lane(S, X, 4)) #kColumns(S, X +Int 1)
    requires X <Int 5
  syntax List ::= #kThetaApply(List, List, Int) [function, total]
  rule #kThetaApply(S, _, I) => S requires I >=Int 25
  rule #kThetaApply(S, C, I) => #kThetaApply(S[I <- ({S[I]}:>Int xorInt ({C[(I +Int 4) modInt 5]}:>Int xorInt rotl64({C[(I +Int 1) modInt 5]}:>Int, 1)))], C, I +Int 1)
    requires I <Int 25

  // rho + pi: B[y, 2x+3y] = rotl(A[x, y], r[x, y])
  syntax List ::= #kRhoPi(List) [function, total]
  rule #kRhoPi(S) => #kRhoPiApply(S, #kZero(25), 0)
  syntax List ::= #kRhoPiApply(List, List, Int) [function, total]
  rule #kRhoPiApply(_, B, I) => B requires I >=Int 25
  rule #kRhoPiApply(S, B, I) => #kRhoPiApply(S, B[((I modInt 5) *Int 0 +Int (I /Int 5)) +Int 5 *Int ((2 *Int (I modInt 5) +Int 3 *Int (I /Int 5)) modInt 5) <- rotl64({S[I]}:>Int, keccakRot(I))], I +Int 1)
    requires I <Int 25

  // chi
  syntax List ::= #kChi(List) [function, total]
  rule #kChi(S) => #kChiApply(S, 0)
  syntax List ::= #kChiApply(List, Int) [function, total]
  rule #kChiApply(_, I) => .List requires I >=Int 25
  rule #kChiApply(S, I) => ListItem(lane(S, I modInt 5, I /Int 5) xorInt ((lane(S, I modInt 5 +Int 1, I /Int 5) xorInt #m64) &Int lane(S, I modInt 5 +Int 2, I /Int 5))) #kChiApply(S, I +Int 1)
    requires I <Int 25

  // iota
  syntax List ::= #kIota(List, Int) [function, total]
  rule #kIota(S, R) => S[0 <- ({S[0]}:>Int xorInt keccakRC(R))]
endmodule
