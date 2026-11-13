//Executable Quint test suite for the X14 L_add G1 affine addition slope AIR model.
//Run with:
//  quint test --main=l_add_test qa/gemini-quint/x14-l-add/l_add_test.qnt

module l_add_test {
  import l_add_core.* from "./l_add"

  // 1. Completeness: honest prover trace on distinct valid curve points
  //    Row 0: (1, 7) + (2, 10) = (11, 2), slope lambda = 3
  //    Row 1: (1, 7) + (7, 5)  = (10, 6), slope lambda = 4
  //    Satisfies all AIR constraints, bus balance, and refinement invariant.
  run honestCompletenessTest =
    val s = buildHonestState(SOUND_CFG)
    assert(
      onCurve(PT_A) and
      onCurve(PT_B) and
      onCurve(PT_C) and
      s.accepted and
      s.l0.lambda == 3 and
      s.x0.x3 == 11 and
      s.y0.y3 == 2 and
      s.l1.lambda == 4 and
      s.x1.x3 == 10 and
      s.y1.y3 == 6 and
      refinementHolds(s)
    )

  // 2. Padding completeness: inactive rows (is_valid = 0) are accepted and satisfy refinement.
  run inactivePaddingCompletenessTest =
    val honest = buildHonestState(SOUND_CFG)
    val s = buildState(
      SOUND_CFG,
      { ...honest.prod0, isValid: 0 },
      honest.prod1,
      { ...honest.l0, isValid: 0, lambda: 12 },
      honest.l1,
      0,
      1,
      NO_EXTRA_MSG
    )
    assert(s.accepted and refinementHolds(s))

  // 3. Soundness against forged slope: wrong lambda on an active row is rejected by the sound AIR.
  run soundRejectsForgedSlopeTest =
    val s = buildStateForMode(SOUND_CFG, 4)
    assert(not(s.accepted) and refinementHolds(s))

  // 4. Mutant 1 (m_no_recv_p2): dropping the second input receive (bus 1101) allows substituting P2
  //    and violates the refinement invariant.
  run mutantNoRecvP2CaughtTest =
    val s = buildStateForMode(CFG_NO_RECV_P2, 1)
    assert(s.accepted and not(refinementHolds(s)))

  // 5. Mutant 2 (m_send_wrong_x): sending x2 instead of x1 on bus 1102 to X3 produces the wrong x3
  //    and violates the refinement invariant.
  run mutantSendWrongXCaughtTest =
    val s = buildStateForMode(CFG_SEND_WRONG_X, 0)
    assert(s.accepted and not(refinementHolds(s)))

  // 6. Mutant 3 (m_no_id): omitting `id` from buses 1102..1105 allows swapping (lambda, x1, x2, y1)
  //    between Row 0 and Row 1, violating the refinement invariant.
  run mutantNoIdCaughtTest =
    val s = buildStateForMode(CFG_NO_ID, 3)
    assert(s.accepted and not(refinementHolds(s)))

  // 7. Mutant 4 (m_no_div_constraint): dropping the (x2 - x1) * lambda = y2 - y1 constraint allows
  //    an arbitrary slope lambda, violating the refinement invariant.
  run mutantNoDivConstraintCaughtTest =
    val s = buildStateForMode(CFG_NO_DIV_CONSTRAINT, 4)
    assert(s.accepted and not(refinementHolds(s)))

  // 8. Mutant 5 (m_valid_free): omitting the boolean constraint is_valid * (is_valid - 1) = 0 allows
  //    is_valid = 2 with wrong slope on a single row, violating the refinement invariant.
  run mutantValidFreeCaughtTest =
    val s = buildStateForMode(CFG_VALID_FREE, 5)
    assert(s.accepted and not(refinementHolds(s)))

  // 9. Finding 1 (degenerate P1 == P2, i.e., x1 == x2 and y1 == y2):
  //    (x2 - x1) * lambda == y2 - y1 collapses to 0 * lambda == 0, leaving lambda completely
  //    unconstrained in [0, p)! Here P1 = P2 = (1, 7); the doubling tangent slope is
  //    3 * 1^2 / (2 * 7) = 3 / 1 = 3 (mod 13), giving 2 * P1 = (7, 5),
  //    yet the sound AIR accepts forged lambda = 5 and outputs (10, 4) != (7, 5).
  run degenerateEqualPointsUnconstrainedLambdaTest =
    val honest = buildHonestState(SOUND_CFG)
    val prodEq: ProducerRow = { isValid: 1, rowId: 10, x1: PT_A.x, y1: PT_A.y, x2: PT_A.x, y2: PT_A.y }
    val lForged: LAddRow = {
      isValid: 1,
      rowId: 10,
      x1: PT_A.x,
      y1: PT_A.y,
      x2: PT_A.x,
      y2: PT_A.y,
      lambda: 5
    }
    val s = buildState(
      SOUND_CFG,
      prodEq,
      honest.prod1,
      lForged,
      honest.l1,
      1,
      1,
      NO_EXTRA_MSG
    )
    assert(
      s.accepted and
      lAddConstraintHolds(SOUND_CFG, lForged) and
      Fp(s.x0.x3, s.y0.y3) != g1AddSpec(PT_A, PT_A)
    )

  // 10. Finding 2 (inverse points P1 == -P2, i.e., x1 == x2 and y1 != y2):
  //     (x2 - x1) * lambda == y2 - y1 becomes 0 * lambda == y2 - y1 != 0, which has no solution
  //     in Fp. Every candidate lambda in Fp is rejected when is_valid == 1.
  run inversePointsUnsatisfiableTest =
    val pNegA: Point = Fp(1, 6) // -PT_A on y^2 = x^3 + 4 (mod 13)
    assert(
      onCurve(pNegA) and
      Fp.forall(candLambda => {
        val row: LAddRow = {
          isValid: 1,
          rowId: 10,
          x1: PT_A.x,
          y1: PT_A.y,
          x2: pNegA.x,
          y2: pNegA.y,
          lambda: candLambda
        }
        not(lAddConstraintHolds(SOUND_CFG, row))
      })
    )

  // 11. Finding 3 (duplicate `id` across two active rows):
  //     If the upstream producer assigns the same rowId to two active rows (rowId = 10),
  //     L_add can swap P2 between Row 0 and Row 1 while balancing every bus (1100..1105),
  //     computing P1_0 + P2_1 and P1_1 + P2_0 instead of P1_0 + P2_0 and P1_1 + P2_1.
  run sameIdSwapInputsAcceptedByBusTest =
    val prod0SameId: ProducerRow = { isValid: 1, rowId: 10, x1: PT_A.x, y1: PT_A.y, x2: PT_B.x, y2: PT_B.y }
    val prod1SameId: ProducerRow = { isValid: 1, rowId: 10, x1: PT_A.x, y1: PT_A.y, x2: PT_C.x, y2: PT_C.y }
    // L_add swaps P2 between Row 0 and Row 1:
    val l0Swapped: LAddRow = {
      isValid: 1,
      rowId: 10,
      x1: PT_A.x,
      y1: PT_A.y,
      x2: PT_C.x,
      y2: PT_C.y,
      lambda: fpDiv(fpSub(PT_C.y, PT_A.y), fpSub(PT_C.x, PT_A.x))
    }
    val l1Swapped: LAddRow = {
      isValid: 1,
      rowId: 10,
      x1: PT_A.x,
      y1: PT_A.y,
      x2: PT_B.x,
      y2: PT_B.y,
      lambda: fpDiv(fpSub(PT_B.y, PT_A.y), fpSub(PT_B.x, PT_A.x))
    }
    val s = buildState(
      SOUND_CFG,
      prod0SameId,
      prod1SameId,
      l0Swapped,
      l1Swapped,
      1,
      1,
      NO_EXTRA_MSG
    )
    assert(
      s.accepted and
      Fp(s.x0.x3, s.y0.y3) == g1AddSpec(PT_A, PT_C) and
      Fp(s.x0.x3, s.y0.y3) != g1AddSpec(PT_A, PT_B)
    )
}
