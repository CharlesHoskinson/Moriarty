// Test suite for Sherlock X14 `L_add` soundness and completeness model.
// Run with: quint test qa/gemini-quint/x14-l-add/l_add_test.qnt --main=l_add_test

module l_add_test {
  import l_add_spec(MUTANT = "none").* from "./l_add"

  pure def honestOutputsMatch(s: ChainState): bool =
    onCurve(s.cons0.x3, s.cons0.y3) and
    onCurve(s.cons1.x3, s.cons1.y3) and
    (s.cons0.x3 == 10) and
    (s.cons0.y3 == 9) and
    (s.cons1.x3 == 5) and
    (s.cons1.y3 == 5)

  // (4) Completeness: every honest row on valid curve points (x1 != x2) is accepted
  // and computes the exact affine sum P1 + P2.
  run completenessHonestChainTest = init.expect(and {
    onCurve(honestProd0.x1, honestProd0.y1),
    onCurve(honestProd0.x2, honestProd0.y2),
    onCurve(honestProd1.x1, honestProd1.y1),
    onCurve(honestProd1.x2, honestProd1.y2),
    (honestProd0.x1 != honestProd0.x2),
    (honestProd1.x1 != honestProd1.x2),
    honestOutputsMatch(st),
    isChainAccepted(SOUND_CFG, st),
    checkInvRow(SOUND_CFG, st),
  })

  // Soundness of l_add across all prover tamper modes (1..9): every single-cell
  // or single-message attack is rejected by the sound AIR + bus balance.
  run soundRejectsProverAttacksTest = init.expect(and {
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 1, 0, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 2, 0, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 3, 0, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 4, 0, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 5, 1, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 6, 1, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 7, 1, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 8, 1, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 9, 0, 0))),
    not(isChainAccepted(SOUND_CFG, proverStepState(SOUND_CFG, 9, 1, 0))),
    checkInvRow(SOUND_CFG, proverStepState(SOUND_CFG, 1, 0, 0)),
    checkInvRow(SOUND_CFG, proverStepState(SOUND_CFG, 2, 0, 0)),
    checkInvRow(SOUND_CFG, proverStepState(SOUND_CFG, 3, 0, 0)),
    checkInvRow(SOUND_CFG, proverStepState(SOUND_CFG, 4, 0, 0)),
  })

  // Mutant 1 (m_no_recv_p2): dropping the receive of (id, x2, y2) on bus o+1
  // allows mutating y2 (and dropping the producer P2 send), violating invRow.
  run mutantNoRecvP2CaughtTest = init.expect(and {
    isChainAccepted(CFG_NO_RECV_P2, proverStepState(CFG_NO_RECV_P2, 1, 0, 0)),
    not(checkInvRow(CFG_NO_RECV_P2, proverStepState(CFG_NO_RECV_P2, 1, 0, 0))),
  })

  // Mutant 2 (m_send_wrong_x): sending (id, lambda, x2) instead of (id, lambda, x1)
  // for X3 on bus o+2 computes x3 = lambda^2 - 2*x2 != lambda^2 - x1 - x2, violating invRow.
  run mutantSendWrongXCaughtTest = init.expect(and {
    isChainAccepted(CFG_SEND_WRONG_X, buildHonestState(CFG_SEND_WRONG_X)),
    not(checkInvRow(CFG_SEND_WRONG_X, buildHonestState(CFG_SEND_WRONG_X))),
  })

  // Mutant 3 (m_no_id): dropping id on bus o+4 allows Y3 cross-row swap of (lambda, x1)
  // between Row 0 (id=0) and Row 1 (id=1), balancing all buses while violating invRow.
  run mutantNoIdCaughtTest = init.expect(and {
    isChainAccepted(CFG_NO_ID, proverStepState(CFG_NO_ID, 3, 0, 0)),
    not(checkInvRow(CFG_NO_ID, proverStepState(CFG_NO_ID, 3, 0, 0))),
  })

  // Mutant 4 (m_no_div_constraint): removing the FieldExpr division constraint
  // allows an arbitrary lambda in L_add, balancing all buses while violating invRow.
  run mutantNoDivConstraintCaughtTest = init.expect(and {
    isChainAccepted(CFG_NO_DIV_CONSTRAINT, proverStepState(CFG_NO_DIV_CONSTRAINT, 2, 0, 0)),
    not(checkInvRow(CFG_NO_DIV_CONSTRAINT, proverStepState(CFG_NO_DIV_CONSTRAINT, 2, 0, 0))),
  })

  // Mutant 5 (m_valid_free): using multiplicity 1 in place of is_valid on the sends
  // allows an inactive L_add row (is_valid = 0, which disables FieldExpr) sending
  // unconstrained messages for X3 and Y3, balancing all buses while violating invRow.
  run mutantValidFreeCaughtTest = init.expect(and {
    isChainAccepted(CFG_VALID_FREE, proverStepState(CFG_VALID_FREE, 4, 0, 0)),
    not(checkInvRow(CFG_VALID_FREE, proverStepState(CFG_VALID_FREE, 4, 0, 0))),
  })

  // Question (1): If two producer rows share the same id (both rowId = 0),
  // L_add can swap P2 between the two rows while balancing every bus and satisfying
  // every AIR constraint in SOUND_CFG. Thus id uniqueness must come from the producer.
  run sameIdSwapInputsAcceptedByBusTest = init.expect(
    isChainAccepted(
      SOUND_CFG,
      buildStateFromLRows(
        SOUND_CFG,
        { rowId: 0, x1: 0, y1: 2, x2: 2, y2: 5, isValid: 1, dropP2Send: false },
        { rowId: 0, x1: 4, y1: 4, x2: 7, y2: 3, isValid: 1, dropP2Send: false },
        {
          rowId: 0, x1: 0, y1: 2, x2: 7, y2: 3,
          lam: divFp(subFp(3, 2), subFp(7, 0)),
          isValid: 1
        },
        {
          rowId: 0, x1: 4, y1: 4, x2: 2, y2: 5,
          lam: divFp(subFp(5, 4), subFp(2, 4)),
          isValid: 1
        },
        false,
        1,
        NO_EXTRA_MSG
      )
    )
  )

  // Question (2a): Exceptional case x1 == x2 with y1 != y2 (inverse points P1 = (0,2), P2 = (0,11)):
  // lambda * (0 - 0) == 11 - 2 = 9 (mod 13) has no solution in F_13, so no valid row exists.
  run exceptionalInversePointsUnsatisfiableTest = init.expect(and {
    onCurve(0, 2),
    onCurve(0, 11),
    allFpHoldRowValid(SOUND_CFG, 0, 2, 0, 11, false),
  })

  // Question (2b): Exceptional case P1 == P2 (x1 == x2 and y1 == y2, e.g. (0,2) + (0,2)):
  // lambda * (0 - 0) == 2 - 2 = 0 (mod 13) holds for EVERY lambda in F_13!
  // Without an external x1 != x2 check, L_add accepts any forged slope lambda.
  run exceptionalDoublingLeavesLambdaFreeTest = init.expect(
    allFpHoldRowValid(SOUND_CFG, 0, 2, 0, 2, true)
  )

  // Question (3): Non-canonical / un-checked input limbs (e.g. x1 = 13 >= P)
  // are rejected when upstream limb bounds enforce canonical values, but would satisfy
  // raw modular congruence mulFp(lam, subFp(x2, 13)) == subFp(y2, y1) if unchecked.
  run nonCanonicalInputRejectedByBoundsPremiseTest = init.expect(and {
    (mulFp(8, subFp(2, 13)) == subFp(5, 2)),
    not(lAddRowValid(SOUND_CFG, { rowId: 0, x1: 13, y1: 2, x2: 2, y2: 5, lam: 8, isValid: 1 })),
  })
}
