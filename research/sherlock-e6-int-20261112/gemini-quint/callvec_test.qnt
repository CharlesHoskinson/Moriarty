// -*- mode: quint; -*-
// Test suite for the E6 Call-Vector Soundness & Completeness Model (callvec.qnt).
//
// Commands (from repository root):
//   quint typecheck qa/gemini-quint/e6-callvec/callvec_test.qnt
//   quint test --main=callvec_test qa/gemini-quint/e6-callvec/callvec_test.qnt
//   quint run --main=callvec_test --invariant=allSoundAndCompleteInv --max-samples=1000 --max-steps=10 --seed=0x424c533132333831 qa/gemini-quint/e6-callvec/callvec_test.qnt

module callvec_test {
  import LaneSpec(
    LIMBS = 3,
    LIMB_BASE = 8,
    CHUNK_BASE = 16,
    R_MOD = 353,
    P_BB = 13,
    PACK_N = 2
  ).* from "./callvec"

  // 1. Edge cases unit test (Questions 2 & 3):
  //    - choice 0: y = 0 ([0, 0, 0]) -> accepted, lane = 0, borrows = [0, 0]
  //    - choice 1: y = r - 1 = 352 ([0, 4, 5]) -> accepted, lane = 352, borrows = [0, 0]
  //    - choice 2: y = r = 353 ([1, 4, 5]) -> rejected for all 4 borrow combinations
  //    - choice 3: y = B - 1 = 7 ([7, 0, 0]) -> accepted, borrow b_1 = 1
  //    - choice 4: y = 39 ([7, 4, 0]) -> accepted, borrows b_1 = 1, b_2 = 1
  //    - choice 5: limb out of range ([8, 0, 0]) -> rejected by range check
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

  // 2. Exhaustive 3-limb check across all 8^3 = 512 3-bit limb triples and all 4 borrow combinations
  run exhaustiveThreeLimbSoundnessAndCompletenessTest =
    init
      .then(stepEdge(0))
      .expect(exhaustiveThreeLimbCheck)

  // 3. Adversarial borrow-forgery / field wrap-around test (Question 2):
  //    Prover supplies non-canonical y = r = 353 (limbs [1, 4, 5]) with all 4 possible
  //    borrow combinations [b_1, b_2] in {0, 1}^2; every combination must fail circuitLaneConstraints.
  run adversarialFieldWrapRejectedTest =
    init
      .then(stepArbitrary(1, 4, 5, 0, 0))
      .expect(allSoundAndCompleteInv and not(circuitLaneConstraints(state.limbs, state.proverBorrows)))
      .then(stepArbitrary(1, 4, 5, 1, 0))
      .expect(allSoundAndCompleteInv and not(circuitLaneConstraints(state.limbs, state.proverBorrows)))
      .then(stepArbitrary(1, 4, 5, 0, 1))
      .expect(allSoundAndCompleteInv and not(circuitLaneConstraints(state.limbs, state.proverBorrows)))
      .then(stepArbitrary(1, 4, 5, 1, 1))
      .expect(allSoundAndCompleteInv and not(circuitLaneConstraints(state.limbs, state.proverBorrows)))

  // 4. Unpack soundness and BabyBear comparison test (Question 1):
  //    - Packed word x = 30 = 14 + 16 * 1 has low chunk u_0 = 14 in [P_BB = 13, CHUNK_BASE = 16).
  //      Circuit `chunks` accepts (u_0 = 14, u_1 = 1), native `e3_iface::unpack` rejects with errNonCanonBb,
  //      and the subsequent limb range check (< LIMB_BASE = 8) rejects u_0 = 14.
  //    - Packed word x = 21 = 5 + 16 * 1 has chunks (5, 1) both < P_BB = 13: both accept and agree.
  run unpackDifferenceOnBabyBearTest =
    init
      .then(stepUnpack(30, 14, 1))
      .expect(and {
        circuitChunksConstraints(state.packedWord, state.proverChunks),
        nativeUnpackWord(state.packedWord).errNonCanonBb,
        allSoundAndCompleteInv,
      })
      .then(stepUnpack(21, 5, 1))
      .expect(and {
        circuitChunksConstraints(state.packedWord, state.proverChunks),
        nativeUnpackWord(state.packedWord).isOk,
        state.proverChunks == nativeUnpackWord(state.packedWord).words,
        allSoundAndCompleteInv,
      })

  // 5. Nondeterministic canonical completeness test (Question 3)
  run nondetCanonicalCompletenessTest = {
    nondet y = 0.to(R_MOD - 1).oneOf()
    init
      .then(stepCanonical(y))
      .expect(and {
        allSoundAndCompleteInv,
        circuitLaneConstraints(state.limbs, state.proverBorrows),
        nativeDec(state.limbs).isSome,
        nativeDec(state.limbs).decoded == y,
        circuitLaneOutput(state.limbs) == y,
      })
  }
}
