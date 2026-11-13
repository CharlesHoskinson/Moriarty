// -*- mode: quint; -*-
// Test suite for the E6 Call-Vector Soundness & Completeness Model (`module e6_callvec_test`).

module e6_callvec_test {
  import LaneSpec(
    LIMBS = 3,
    LIMB_BASE = 8,
    CHUNK_BASE = 16,
    R_MOD = 353,
    P_BB = 13,
    PACK_N = 2
  ).* from "./callvec"

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
