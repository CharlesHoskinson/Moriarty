// Test suite for X3 AIR soundness, completeness, mutants, id collision, and exceptional cases.
// Run: quint test qa/gemini-quint/x14-x3/x3_test.qnt --main=x3_test --max-samples=100 --seed=0x14

module x3_test {
  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ) as sound from "./x3"

  import x3_base(
    NO_RECV_X2 = true,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ) as t_no_recv_x2 from "./x3"

  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = true,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ) as t_sign_flip from "./x3"

  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = true,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ) as t_no_id_send from "./x3"

  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = true,
    VALID_FREE = false
  ) as t_recv_swapped from "./x3"

  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = true
  ) as t_valid_free from "./x3"

  // ---- Completeness tests on the sound circuit.
  // 1. Honest addition chain (o = 1100, x1 != x2) is accepted and outputs P1 + P2.
  run honestAddChainAcceptedTest =
    sound::init
      .then(sound::proveHonestAdd(false))
      .expect(and {
        sound::st.accepted,
        sound::invRow,
        not(sound::invNoAcceptedHonest),
        not(sound::invNoAcceptedAdd),
        sound::st.srcRows[0].x3 == 8,
        sound::st.srcRows[0].y3 == 3,
        sound::st.srcRows[1].x3 == 0,
        sound::st.srcRows[1].y3 == 2,
      })

  // 2. Honest doubling chain (o = 1110, x2 = x1) is accepted and outputs [2]P1.
  run honestDblChainAcceptedTest =
    sound::init
      .then(sound::proveHonestDbl(false))
      .expect(and {
        sound::st.accepted,
        sound::invRow,
        not(sound::invNoAcceptedHonest),
        not(sound::invNoAcceptedDbl),
        sound::st.srcRows[0].x3 == 12,
        sound::st.srcRows[0].y3 == 9,
        sound::st.srcRows[1].x3 == 6,
        sound::st.srcRows[1].y3 == 5,
      })

  // 3. Permuting X3 rows physically while preserving rowId is accepted by the LogUp multiset bus.
  run permutedX3RowOrderAcceptedTest =
    sound::init
      .then(sound::proveHonestAdd(true))
      .expect(and {
        sound::st.accepted,
        sound::invRow,
      })

  // ---- Soundness refusal tests on the sound circuit.
  // 4. Tampering any single cell of X3 (lam, x1, x2, x3, rowId, isValid) is refused.
  run tamperX3InputCellRefusedTest =
    sound::init
      .then(sound::proveTamperSingleCell(0, 2, 1, false))
      .expect(and {
        not(sound::st.accepted),
        sound::invRow,
      })

  // 5. Dropping one bus message (multDelta = -1) is refused by busBalanced.
  run droppedBusMessageRefusedTest =
    sound::init
      .then(sound::proveTamperBusMessage(0, 1, -1, false))
      .expect(and {
        not(sound::st.accepted),
        sound::invRow,
      })

  // 6. Duplicating one bus message (multDelta = +1) is refused by busBalanced.
  run duplicatedBusMessageRefusedTest =
    sound::init
      .then(sound::proveTamperBusMessage(0, 2, 1, false))
      .expect(and {
        not(sound::st.accepted),
        sound::invRow,
      })

  // 7. Random steps of the nondeterministic prover preserve invRow in sound module.
  run randomStepsSoundTest =
    sound::init
      .then(25.reps(_ => sound::step.expect(sound::invRow)))

  // ---- Mutant detection tests (each mutant must accept the attack and violate invRow).
  // 8. m_no_recv_x2: drop receive on bus o+3 (x2 free).
  run mutantNoRecvX2CaughtTest =
    t_no_recv_x2::init
      .then(t_no_recv_x2::proveAttackNoRecvX2(0, 1, false))
      .expect(and {
        t_no_recv_x2::st.accepted,
        not(t_no_recv_x2::invRow),
      })

  // 9. m_sign_flip: constraint x3 = lambda^2 - x1 + x2.
  run mutantSignFlipCaughtTest =
    t_sign_flip::init
      .then(t_sign_flip::proveAttackSignFlip(false))
      .expect(and {
        t_sign_flip::st.accepted,
        not(t_sign_flip::invRow),
      })

  // 10. m_no_id_send: drop id from send on bus o+6.
  run mutantNoIdSendCaughtTest =
    t_no_id_send::init
      .then(t_no_id_send::proveAttackNoIdSend(false))
      .expect(and {
        t_no_id_send::st.accepted,
        not(t_no_id_send::invRow),
      })

  // 11. m_recv_swapped: receive (id, x1, lambda) in place of (id, lambda, x1).
  run mutantRecvSwappedCaughtTest =
    t_recv_swapped::init
      .then(t_recv_swapped::proveAttackRecvSwapped(false))
      .expect(and {
        t_recv_swapped::st.accepted,
        not(t_recv_swapped::invRow),
      })

  // 12. m_valid_free: multiplicity 1 in place of is_valid on the send.
  run mutantValidFreeCaughtTest =
    t_valid_free::init
      .then(t_valid_free::proveAttackValidFree(0, 1, false))
      .expect(and {
        t_valid_free::st.accepted,
        not(t_valid_free::invRow),
      })

  // ---- Question (1) & Question (2) structural findings tests.
  // 13. Question (1): Two rows sharing the same id (rowId = 1) CAN swap x2 across bus o+3.
  run question1SameIdInputSwapAcceptedTest =
    sound::init
      .then(sound::proveSameIdSwapX2)
      .expect(and {
        sound::st.accepted,
        not(sound::chainOutputsAffineSum(sound::st.srcRows, false)),
      })

  // 14. Question (2a): P1 == P2 (x1 == x2, y1 == y2) in g1_add leaves lambda free (0 * lam == 0).
  run question2ExceptionalAddEqualPointsFreeLambdaTest =
    sound::init
      .then(sound::proveExceptionalAddEqualPoints(7))
      .expect(and {
        sound::st.accepted,
        sound::st.srcRows[0].x3 != sound::PT1.x,
      })

  // 15. Question (2b): P2 == -P1 (x1 == x2, y1 != y2) in g1_add is unsatisfiable and refused.
  run question2ExceptionalAddInversePointsRefusedTest =
    sound::init
      .then(sound::proveExceptionalAddInversePoints(0))
      .expect(not(sound::st.accepted))

  // 16. Question (2c): (0, 0) in g1_double leaves lambda free (0 * lam == 0) and forges (lam^2, -lam^3).
  run question2ExceptionalDblOriginFreeLambdaTest =
    sound::init
      .then(sound::proveExceptionalDblOrigin(5))
      .expect(and {
        sound::st.accepted,
        sound::st.srcRows[0].x3 == 12,
        sound::st.srcRows[0].y3 == 5,
      })
}
