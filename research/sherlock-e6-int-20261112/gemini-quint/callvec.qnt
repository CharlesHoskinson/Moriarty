// -*- mode: quint; -*-
// E6 Call-Vector Soundness & Completeness Model (lane-src/callvec.rs vs lane-src/e3_iface.rs)
//
// Correspondence Map:
//   LaneSpec::circuitChunksConstraints <-> lane-src/callvec.rs `unpack` (`Alg::chunks`)
//   LaneSpec::nativeUnpackWord         <-> lane-src/e3_iface.rs `unpack`
//   LaneSpec::nativeDec                <-> lane-src/e3_iface.rs `dec`
//   LaneSpec::nativeBorrowChain        <-> lane-src/callvec.rs lines 79-104 (`out` borrow witness)
//   LaneSpec::circuitLaneConstraints   <-> lane-src/callvec.rs lines 80-104 (`range`, `wbit`, `lin`)
//   LaneSpec::circuitLaneOutput        <-> lane-src/callvec.rs lines 105-107 (`lanes.push(a.lin(...))`)
//
// Structure:
// 1. `LaneSpec`: Abstract parametric specification of the E6 unpack, limb range checks,
//    field-arithmetic borrow chain against r - 1, native borrow witness generation,
//    and statement lane reconstruction, for arbitrary (LIMBS, LIMB_BASE, CHUNK_BASE, R_MOD, P_BB, PACK_N).
// 2. `callvec`: Concrete runnable module instantiating `LaneSpec` at reduced width:
//    3 limbs of 3 bits (LIMBS = 3, LIMB_BASE = 8), 4-bit unpacked chunks (CHUNK_BASE = 16),
//    small prime R_MOD = 353 (r - 1 = 352, limbs [0, 4, 5]), BabyBear analog P_BB = 13, PACK_N = 2.

module LaneSpec {
  // 1. Types
  type Phase = PhaseInit | PhaseCanonical | PhaseArbitrary | PhaseEdge | PhaseUnpack

  type DecOutcome = {
    isSome: bool,
    decoded: int,
  }

  type UnpackOutcome = {
    isOk: bool,
    words: List[int],
    errHighBits: bool,
    errNonCanonBb: bool,
  }

  type LaneState = {
    phase: Phase,
    limbs: List[int],
    proverBorrows: List[int],
    packedWord: int,
    proverChunks: List[int],
  }

  // 2. Module Parameters (Constants)
  const LIMBS: int
  const LIMB_BASE: int
  const CHUNK_BASE: int
  const R_MOD: int
  const P_BB: int
  const PACK_N: int

  // 3. Pure Functions
  // Integer power base^exp for non-negative exponents up to 9.
  pure def powInt(base: int, exp: int): int =
    if (exp <= 0) 1
    else if (exp == 1) base
    else if (exp == 2) base * base
    else if (exp == 3) base * base * base
    else if (exp == 4) base * base * base * base
    else List(0, 1, 2, 3, 4, 5, 6, 7, 8).foldl(1, (acc, idx) =>
      if (idx < exp) acc * base else acc
    )

  // Canonical representative in [0, R_MOD) of an integer x modulo R_MOD.
  pure def modFr(x: int): int =
    ((x % R_MOD) + R_MOD) % R_MOD

  // k-th limb of R_MOD - 1 in base LIMB_BASE (`r_minus_1_limbs()` in `callvec.rs`).
  pure def rMinus1Limb(k: int): int =
    ((R_MOD - 1) / powInt(LIMB_BASE, k)) % LIMB_BASE

  // Reconstruct the integer sum_{k=0..LIMBS-1} c_k * LIMB_BASE^k.
  pure def limbsToInt(cs: List[int]): int =
    List(0, 1, 2, 3, 4, 5, 6, 7, 8).foldl(0, (acc, k) =>
      if (k < cs.length()) acc + cs.nth(k) * powInt(LIMB_BASE, k) else acc
    )

  // Decompose a non-negative integer x into LIMBS limbs in base LIMB_BASE.
  pure def intToLimbs(x: int): List[int] = {
    val emptyLimbs: List[int] = List()
    List(0, 1, 2, 3, 4, 5, 6, 7, 8).foldl(emptyLimbs, (acc, k) =>
      if (k < LIMBS)
        acc.append((x / powInt(LIMB_BASE, k)) % LIMB_BASE)
      else
        acc
    )
  }

  // Native reference `e3_iface::dec(c)`:
  // Succeeds iff every limb is in [0, LIMB_BASE) and the reconstructed integer is < R_MOD.
  pure def nativeDec(cs: List[int]): DecOutcome = {
    val allInRange = cs.indices().forall(k => cs.nth(k) >= 0 and cs.nth(k) < LIMB_BASE)
    val total = limbsToInt(cs)
    if (cs.length() == LIMBS and allInRange and total >= 0 and total < R_MOD)
      { isSome: true, decoded: total }
    else
      { isSome: false, decoded: 0 }
  }

  // Native borrow witness generator from `callvec.rs`:
  // b_0 = 0; for k in 0..LIMBS-2, b_{k+1} = 1 if (R_k - c_k - b_k < 0) else 0; b_{LIMBS} = 0.
  pure def nativeBorrowChain(cs: List[int]): List[int] = {
    val startBorrows: List[int] = List(0)
    val middle = List(0, 1, 2, 3, 4, 5, 6, 7, 8).foldl(startBorrows, (bs, k) =>
      if (k + 1 < LIMBS) {
        val bk = bs.nth(k)
        val rk = rMinus1Limb(k)
        val ck = cs.nth(k)
        val nextB = if (rk - ck - bk < 0) 1 else 0
        bs.append(nextB)
      } else {
        bs
      }
    )
    middle.append(0)
  }

  // Extract the LIMBS - 1 interior witness borrow bits [b_1, ..., b_{LIMBS-1}].
  pure def nativeInnerBorrows(cs: List[int]): List[int] = {
    val fullBs = nativeBorrowChain(cs)
    val emptyBs: List[int] = List()
    List(1, 2, 3, 4, 5, 6, 7, 8).foldl(emptyBs, (acc, k) =>
      if (k < LIMBS) acc.append(fullBs.nth(k)) else acc
    )
  }

  // Expand interior borrow bits [b_1, ..., b_{LIMBS-1}] with boundary zeros b_0 = 0, b_{LIMBS} = 0.
  pure def fullBorrowsFromInner(innerBorrows: List[int]): List[int] =
    List(0).concat(innerBorrows).append(0)

  // Integer difference hat_d_k = R_k - c_k - b_k + LIMB_BASE * b_{k+1} (before mod R_MOD).
  pure def intDiffAt(cs: List[int], fullBorrows: List[int], k: int): int = {
    val rk = rMinus1Limb(k)
    val ck = cs.nth(k)
    val bk = fullBorrows.nth(k)
    val bk1 = fullBorrows.nth(k + 1)
    rk - ck - bk + LIMB_BASE * bk1
  }

  // Field element d_k in [0, R_MOD) computed by `a.lin(&terms, F::from(rl[k]))`.
  pure def fieldDiffAt(cs: List[int], fullBorrows: List[int], k: int): int =
    modFr(intDiffAt(cs, fullBorrows, k))

  // Circuit constraints for a single lane given limb cells `cs` and interior borrow bits
  // `innerBorrows` = [b_1, ..., b_{LIMBS-1}]:
  // 1. Each limb satisfies 0 <= c_k < LIMB_BASE (`a.range(&c[k], LIMB_BITS)`).
  // 2. Each interior borrow bit b_k is boolean in {0, 1} (`a.wbit`).
  // 3. Each field linear combination d_k satisfies 0 <= d_k < LIMB_BASE (`a.range(&d, LIMB_BITS)`).
  pure def circuitLaneConstraints(cs: List[int], innerBorrows: List[int]): bool = {
    val fullBs = fullBorrowsFromInner(innerBorrows)
    and {
      cs.length() == LIMBS,
      innerBorrows.length() == LIMBS - 1,
      cs.indices().forall(k => cs.nth(k) >= 0 and cs.nth(k) < LIMB_BASE),
      innerBorrows.indices().forall(j => innerBorrows.nth(j) == 0 or innerBorrows.nth(j) == 1),
      cs.indices().forall(k => {
        val dk = fieldDiffAt(cs, fullBs, k)
        dk >= 0 and dk < LIMB_BASE
      }),
    }
  }

  // Output cell of the lane: sum_{k=0..LIMBS-1} c_k * LIMB_BASE^k mod R_MOD.
  pure def circuitLaneOutput(cs: List[int]): int =
    modFr(limbsToInt(cs))

  // Unpack model (Question 1):
  // Circuit `a.chunks(x, CHUNK_BITS, PACK_N)` constrains:
  // - 0 <= c_i < CHUNK_BASE for i in 0..PACK_N-1
  // - (sum_{i=0..PACK_N-1} c_i * CHUNK_BASE^i) mod R_MOD == x
  pure def chunksToInt(chunks: List[int]): int =
    List(0, 1, 2, 3, 4, 5, 6, 7).foldl(0, (acc, i) =>
      if (i < chunks.length()) acc + chunks.nth(i) * powInt(CHUNK_BASE, i) else acc
    )

  pure def extractChunks(x: int): List[int] = {
    val emptyChunks: List[int] = List()
    List(0, 1, 2, 3, 4, 5, 6, 7).foldl(emptyChunks, (acc, i) =>
      if (i < PACK_N)
        acc.append((x / powInt(CHUNK_BASE, i)) % CHUNK_BASE)
      else
        acc
    )
  }

  pure def circuitChunksConstraints(x: int, chunks: List[int]): bool = and {
    x >= 0 and x < R_MOD,
    chunks.length() == PACK_N,
    chunks.indices().forall(i => chunks.nth(i) >= 0 and chunks.nth(i) < CHUNK_BASE),
    modFr(chunksToInt(chunks)) == x,
  }

  // Native `e3_iface::unpack` on a single packed word x into PACK_N chunks:
  pure def nativeUnpackWord(x: int): UnpackOutcome = {
    val extracted = extractChunks(x)
    val highBits = x / powInt(CHUNK_BASE, PACK_N)
    val allBelowP = extracted.indices().forall(i => extracted.nth(i) < P_BB)
    if (not(allBelowP))
      { isOk: false, words: extracted, errHighBits: false, errNonCanonBb: true }
    else if (highBits != 0)
      { isOk: false, words: extracted, errHighBits: true, errNonCanonBb: false }
    else
      { isOk: true, words: extracted, errHighBits: false, errNonCanonBb: false }
  }

  // All 2-bit interior borrow combinations [b_1, b_2] for the 3-limb instance
  pure val ALL_BORROWS_3: Set[List[int]] = Set(
    List(0, 0),
    List(0, 1),
    List(1, 0),
    List(1, 1)
  )

  // Existential satisfiability of the circuit for a limb vector `cs`
  pure def circuitSatisfiable(cs: List[int]): bool =
    if (LIMBS == 3)
      ALL_BORROWS_3.exists(bs => circuitLaneConstraints(cs, bs))
    else
      circuitLaneConstraints(cs, nativeInnerBorrows(cs))

  // Exhaustive check across all 8^3 = 512 3-bit limb vectors and all 4 borrow vectors
  pure val exhaustiveThreeLimbCheck: bool = {
    val limbVals = List(0, 1, 2, 3, 4, 5, 6, 7)
    limbVals.indices().forall(i0 =>
      limbVals.indices().forall(i1 =>
        limbVals.indices().forall(i2 => {
          val cs = List(i0, i1, i2)
          val nativeBs = nativeInnerBorrows(cs)
          val decRes = nativeDec(cs)
          if (decRes.isSome) (
            circuitLaneConstraints(cs, nativeBs)
              and (circuitLaneOutput(cs) == decRes.decoded)
              and ALL_BORROWS_3.forall(bs =>
                circuitLaneConstraints(cs, bs) iff (bs == nativeBs)
              )
          ) else (
            ALL_BORROWS_3.forall(bs => not(circuitLaneConstraints(cs, bs)))
          )
        })
      )
    )
  }

  // Guards and next-state transition functions
  pure def canEvalCanonical(y: int): bool =
    y >= 0 and y < R_MOD

  pure def applyEvalCanonical(s: LaneState, y: int): LaneState = {
    val cs = intToLimbs(y)
    val bs = nativeInnerBorrows(cs)
    {
      ...s,
      phase: PhaseCanonical,
      limbs: cs,
      proverBorrows: bs,
    }
  }

  pure def canEvalArbitrary(c0: int, c1: int, c2: int, b1: int, b2: int): bool = and {
    LIMBS == 3,
    c0 >= 0 and c0 < CHUNK_BASE,
    c1 >= 0 and c1 < CHUNK_BASE,
    c2 >= 0 and c2 < CHUNK_BASE,
    b1 == 0 or b1 == 1,
    b2 == 0 or b2 == 1,
  }

  pure def applyEvalArbitrary(s: LaneState, c0: int, c1: int, c2: int, b1: int, b2: int): LaneState = {
    val cs = List(c0, c1, c2)
    val bs = List(b1, b2)
    {
      ...s,
      phase: PhaseArbitrary,
      limbs: cs,
      proverBorrows: bs,
    }
  }

  pure def canEvalEdge(choice: int): bool =
    choice >= 0 and choice <= 5

  pure def edgeLimbsForChoice(choice: int): List[int] =
    if (choice == 0) intToLimbs(0)
    else if (choice == 1) intToLimbs(R_MOD - 1)
    else if (choice == 2) intToLimbs(R_MOD)
    else if (choice == 3) intToLimbs(LIMB_BASE - 1)
    else if (choice == 4) intToLimbs((LIMB_BASE - 1) + rMinus1Limb(1) * LIMB_BASE)
    else intToLimbs(0).replaceAt(0, LIMB_BASE)

  pure def applyEvalEdge(s: LaneState, choice: int): LaneState = {
    val cs = edgeLimbsForChoice(choice)
    val bs = nativeInnerBorrows(cs)
    {
      ...s,
      phase: PhaseEdge,
      limbs: cs,
      proverBorrows: bs,
    }
  }

  pure def canEvalUnpack(x: int, u0: int, u1: int): bool = and {
    PACK_N == 2,
    x >= 0 and x < R_MOD,
    u0 >= 0 and u0 < CHUNK_BASE,
    u1 >= 0 and u1 < CHUNK_BASE,
  }

  pure def applyEvalUnpack(s: LaneState, x: int, u0: int, u1: int): LaneState = {
    {
      ...s,
      phase: PhaseUnpack,
      packedWord: x,
      proverChunks: List(u0, u1),
    }
  }

  // 4. State Variables
  var state: LaneState

  // 5. Invariants & Witnesses (Questions 1 - 5)

  // Invariant 1 — Unpack Soundness (Question 1):
  val unpackSoundnessInv: bool = {
    val x = state.packedWord
    val cs = state.proverChunks
    val circuitOk = circuitChunksConstraints(x, cs)
    val uniqueDigits = extractChunks(x)
    val nativeRes = nativeUnpackWord(x)
    if (circuitOk) (
      (x < powInt(CHUNK_BASE, PACK_N))
        and (cs == uniqueDigits)
        and (
          if (nativeRes.isOk)
            (cs == nativeRes.words)
          else (
            nativeRes.errNonCanonBb
              and not(nativeRes.errHighBits)
              and cs.indices().exists(i => cs.nth(i) >= P_BB and cs.nth(i) >= LIMB_BASE)
          )
        )
    ) else (
      nativeRes.isOk implies circuitChunksConstraints(x, nativeRes.words)
    )
  }

  // Invariant 2 — No Field Wrap-Around in `lin` (Question 2):
  val noFieldWrapInv: bool = {
    val cs = state.limbs
    val fullBs = fullBorrowsFromInner(state.proverBorrows)
    val limbsInRange = cs.indices().forall(k => cs.nth(k) >= 0 and cs.nth(k) < LIMB_BASE)
    limbsInRange implies cs.indices().forall(k => {
      val dkField = fieldDiffAt(cs, fullBs, k)
      val dkInt = intDiffAt(cs, fullBs, k)
      (dkField >= 0 and dkField < LIMB_BASE) iff (dkInt >= 0 and dkInt < LIMB_BASE and dkField == dkInt)
    })
  }

  // Invariant 3 — Lane Soundness & Witness Uniqueness (Question 2 & Question 5):
  val soundness: bool = {
    val cs = state.limbs
    val bs = state.proverBorrows
    val decRes = nativeDec(cs)
    circuitLaneConstraints(cs, bs) implies and {
      decRes.isSome,
      circuitLaneOutput(cs) == decRes.decoded,
      bs == nativeInnerBorrows(cs),
    }
  }

  val soundnessInv: bool = soundness

  // Invariant 4 — Lane Completeness & Exact IFF Equivalence (Question 2 & Question 3):
  val completeness: bool = {
    val cs = state.limbs
    val nativeBs = nativeInnerBorrows(cs)
    val decRes = nativeDec(cs)
    if (decRes.isSome) (
      circuitLaneConstraints(cs, nativeBs)
        and circuitSatisfiable(cs)
        and (circuitLaneOutput(cs) == decRes.decoded)
    ) else (
      not(circuitSatisfiable(cs))
    )
  }

  val completenessInv: bool = completeness
  val completenessAndIffInv: bool = completeness

  // Invariant 5 — Parameter Consistency & Packing Layout Indices (Questions 1 - 4):
  val layoutAndParamsInv: bool = {
    val wordsTotal = 12164
    val packedTotal = 1 + (wordsTotal + 7) / 8
    val laneMapLoJ = 106 / 8
    val laneMapHiJ = (114 + 7) / 8
    val callAtLoJ = 2930 / 8
    val callAtHiJ = (wordsTotal + 7) / 8
    val lastPackedChunks = wordsTotal - 8 * 1520
    and {
      state.limbs.length() == LIMBS,
      limbsToInt(intToLimbs(R_MOD - 1)) == R_MOD - 1,
      packedTotal == 1522,
      laneMapLoJ == 13 and laneMapHiJ == 15,
      callAtLoJ == 366 and callAtHiJ == 1521,
      lastPackedChunks == 4,
      wordsTotal - 2930 == 1026 * 9,
    }
  }

  // Combined top-level safety invariant
  val allSoundAndCompleteInv: bool = and {
    unpackSoundnessInv,
    noFieldWrapInv,
    soundness,
    completeness,
    layoutAndParamsInv,
  }

  val inv: bool = allSoundAndCompleteInv

  // Witnesses (Reachability checks for `quint run --witnesses`)
  val witnessAcceptedCanonical: bool =
    state.phase != PhaseInit and circuitLaneConstraints(state.limbs, state.proverBorrows)

  val witnessRejectedNonCanon: bool = and {
    state.limbs.indices().forall(k => state.limbs.nth(k) < LIMB_BASE),
    limbsToInt(state.limbs) >= R_MOD,
    not(circuitSatisfiable(state.limbs)),
  }

  val witnessRejectedLimbRange: bool = and {
    state.limbs.indices().exists(k => state.limbs.nth(k) >= LIMB_BASE),
    not(circuitSatisfiable(state.limbs)),
  }

  val witnessActiveBorrowAccepted: bool = and {
    circuitLaneConstraints(state.limbs, state.proverBorrows),
    state.proverBorrows.indices().exists(j => state.proverBorrows.nth(j) == 1),
  }

  val witnessUnpackDiffersOnBB: bool = and {
    circuitChunksConstraints(state.packedWord, state.proverChunks),
    nativeUnpackWord(state.packedWord).errNonCanonBb,
  }

  // 6. Actions (Guarded, Thin)
  action stepCanonical(y: int): bool = all {
    canEvalCanonical(y),
    state' = applyEvalCanonical(state, y),
  }

  action stepArbitrary(c0: int, c1: int, c2: int, b1: int, b2: int): bool = all {
    canEvalArbitrary(c0, c1, c2, b1, b2),
    state' = applyEvalArbitrary(state, c0, c1, c2, b1, b2),
  }

  action stepEdge(choice: int): bool = all {
    canEvalEdge(choice),
    state' = applyEvalEdge(state, choice),
  }

  action stepUnpack(x: int, u0: int, u1: int): bool = all {
    canEvalUnpack(x, u0, u1),
    state' = applyEvalUnpack(state, x, u0, u1),
  }

  // 7. Initialization
  action init: bool = all {
    state' = {
      phase: PhaseInit,
      limbs: intToLimbs(0),
      proverBorrows: nativeInnerBorrows(intToLimbs(0)),
      packedWord: 0,
      proverChunks: extractChunks(0),
    },
  }

  // 8. Step Action
  action step: bool = {
    nondet y = 0.to(R_MOD - 1).oneOf()
    nondet c0 = 0.to(CHUNK_BASE - 1).oneOf()
    nondet c1 = 0.to(CHUNK_BASE - 1).oneOf()
    nondet c2 = 0.to(CHUNK_BASE - 1).oneOf()
    nondet b1 = Set(0, 1).oneOf()
    nondet b2 = Set(0, 1).oneOf()
    nondet edgeChoice = 0.to(5).oneOf()
    nondet x = 0.to(R_MOD - 1).oneOf()
    nondet u0 = 0.to(CHUNK_BASE - 1).oneOf()
    nondet u1 = 0.to(CHUNK_BASE - 1).oneOf()
    any {
      stepCanonical(y),
      stepArbitrary(c0, c1, c2, b1, b2),
      stepEdge(edgeChoice),
      stepUnpack(x, x % CHUNK_BASE, (x / CHUNK_BASE) % CHUNK_BASE),
      stepUnpack(x, u0, u1),
    }
  }
}

// Concrete reduced-width module (default entry point for `callvec.qnt`)
module callvec {
  import LaneSpec(
    LIMBS = 3,
    LIMB_BASE = 8,
    CHUNK_BASE = 16,
    R_MOD = 353,
    P_BB = 13,
    PACK_N = 2
  ).*

  // Deterministic Unit Tests (`quint test`)
  run edgeCasesTest =
    init
      .then(stepEdge(0))
      .expect(allSoundAndCompleteInv and circuitLaneOutput(state.limbs) == 0)
      .then(stepEdge(1))
      .expect(allSoundAndCompleteInv and circuitLaneOutput(state.limbs) == R_MOD - 1)
      .then(stepEdge(2))
      .expect(allSoundAndCompleteInv and not(circuitSatisfiable(state.limbs)))
      .then(stepEdge(3))
      .expect(allSoundAndCompleteInv and state.proverBorrows.nth(0) == 1)
      .then(stepEdge(4))
      .expect(allSoundAndCompleteInv and state.proverBorrows.nth(0) == 1 and state.proverBorrows.nth(1) == 1)
      .then(stepEdge(5))
      .expect(allSoundAndCompleteInv and not(circuitSatisfiable(state.limbs)))

  run exhaustiveThreeLimbSoundnessAndCompletenessTest =
    init
      .then(stepEdge(0))
      .expect(exhaustiveThreeLimbCheck)

  run unpackDifferenceOnBabyBearTest =
    init
      .then(stepUnpack(30, 14, 1))
      .expect(and {
        circuitChunksConstraints(state.packedWord, state.proverChunks),
        nativeUnpackWord(state.packedWord).errNonCanonBb,
        allSoundAndCompleteInv,
      })
}
