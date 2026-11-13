// -*- mode: Bluespec; -*-
// Deterministic unit and property tests for x14_identity and its five mutants.
// Every test is named <something>Test so `quint test` runs all of them by default.

module x14_identity_test {
  import x14_defs.* from "./x14_identity"

  pure val CFG_NO_ZERO_LIMBS: MutantCfg = { ...SOUND_CFG, no_zero_limbs: true }
  pure val CFG_E_IGNORES_FLAG: MutantCfg = { ...SOUND_CFG, e_ignores_flag: true }
  pure val CFG_SEL_WRONG: MutantCfg = { ...SOUND_CFG, sel_wrong: true }
  pure val CFG_OUT_FLAG_WRONG: MutantCfg = { ...SOUND_CFG, out_flag_wrong: true }
  pure val CFG_OUT_MULT_V: MutantCfg = { ...SOUND_CFG, out_mult_v: true }

  var st: TraceState

  action init: bool = {
    st' = traceHonestAdd(SOUND_CFG)
  }

  // 1. Curve arithmetic and group law with identity over E(F_13): y^2 = x^3 + 4 (mod 13)
  run curveGroupLawTest = {
    init.then(all {
      assert(not(onCurve(0, 0))),
      assert(onCurve(PT_G1.x, PT_G1.y)),
      assert(onCurve(PT_G2.x, PT_G2.y)),
      assert(onCurve(PT_G3.x, PT_G3.y)),
      assert(onCurve(PT_G4.x, PT_G4.y)),
      assert(g1Add(PT_INF, PT_G1) == PT_G1),
      assert(g1Add(PT_G1, PT_INF) == PT_G1),
      assert(g1Add(PT_INF, PT_INF) == PT_INF),
      assert(g1Dbl(PT_INF) == PT_INF),
      assert(g1Dbl(PT_G1) == PT_G2),
      assert(g1Add(PT_G1, PT_G2) == PT_G3),
      assert(g1Dbl(PT_G2) == PT_G4),
      st' = st,
    })
  }

  // 2. Honest add trace (O + P, P + O, O + O, P1 + P2) verifies and refines G1 group law
  run honestAddVerifyTest = {
    val tr = traceHonestAdd(SOUND_CFG)
    init.then(all {
      assert(producerValid(tr)),
      assert(pointsInValid(SOUND_CFG, tr)),
      assert(chainValid(tr)),
      assert(busBalanced(allBusEntries(SOUND_CFG, tr))),
      assert(isTraceAccepted(SOUND_CFG, tr)),
      assert(allRowsRefined(tr)),
      st' = tr,
    })
  }

  // 3. Honest dbl trace (2 * O, 2 * G1, 2 * G2, padding) verifies and refines G1 doubling
  run honestDblVerifyTest = {
    val tr = traceHonestDbl(SOUND_CFG)
    init.then(all {
      assert(producerValid(tr)),
      assert(pointsInValid(SOUND_CFG, tr)),
      assert(chainValid(tr)),
      assert(busBalanced(allBusEntries(SOUND_CFG, tr))),
      assert(isTraceAccepted(SOUND_CFG, tr)),
      assert(allRowsRefined(tr)),
      st' = tr,
    })
  }

  // 4. Measured negative test (neg == "inf_lie"):
  // A flag on a real point (nonzero limbs) fails ONLY PointsIn; chain constraints and LogUp balance hold.
  run negFlagOnRealPointFailsOnlyPointsInTest = {
    val tr = traceAttackFlagRealPoint(SOUND_CFG)
    init.then(all {
      assert(producerValid(tr)),
      assert(chainValid(tr)),
      assert(busBalanced(allBusEntries(SOUND_CFG, tr))),
      assert(not(pointsInValid(SOUND_CFG, tr))),
      assert(not(isTraceAccepted(SOUND_CFG, tr))),
      st' = tr,
    })
  }

  // 5. Measured negative test (neg == "inf_out"):
  // Consumer taking the chain sum in place of P2 for O + P2 satisfies all AIR constraints but fails LogUp balance.
  run negConsumerTakesChainSumFailsLogUpTest = {
    val tr = traceNegSinkTakesChainSum(SOUND_CFG)
    init.then(all {
      assert(producerValid(tr)),
      assert(pointsInValid(SOUND_CFG, tr)),
      assert(chainValid(tr)),
      assert(not(busBalanced(allBusEntries(SOUND_CFG, tr)))),
      assert(not(isTraceAccepted(SOUND_CFG, tr))),
      st' = tr,
    })
  }

  // 6. Mutant m_no_zero_limbs: accepts a flagged real point and outputs P2 instead of P1 + P2
  run mutantNoZeroLimbsCaughtTest = {
    val tr = traceAttackFlagRealPoint(CFG_NO_ZERO_LIMBS)
    init.then(all {
      assert(isTraceAccepted(CFG_NO_ZERO_LIMBS, tr)),
      assert(not(allRowsRefined(tr))),
      assert(not(isTraceAccepted(SOUND_CFG, tr))),
      st' = tr,
    })
  }

  // 7. Mutant m_e_ignores_flag:
  // (a) On O + P and P + O (with x != 0), the chain runs on (0, 0) and P, accepting the wrong secant point (1, 4, 0).
  // (b) On O + O and 2 * O, the denominator (x2 - x1 = 0 or y1 = 0) is not invertible, so INV / INV_y rejects.
  run mutantEIgnoresFlagCaughtTest = {
    val tr_accept_wrong = traceAttackRunChainOnZero(CFG_E_IGNORES_FLAG)
    val tr_oo_reject = traceHonestAdd(CFG_E_IGNORES_FLAG)
    val tr_2o_reject = traceHonestDbl(CFG_E_IGNORES_FLAG)
    init.then(all {
      assert(isTraceAccepted(CFG_E_IGNORES_FLAG, tr_accept_wrong)),
      assert(tr_accept_wrong.sink_rows[0].x == 1 and tr_accept_wrong.sink_rows[0].y == 4),
      assert(not(allRowsRefined(tr_accept_wrong))),
      assert(not(isTraceAccepted(SOUND_CFG, tr_accept_wrong))),
      assert(not(chainValid(tr_oo_reject))),
      assert(not(chainValid(tr_2o_reject))),
      st' = tr_accept_wrong,
    })
  }

  // 8. Mutant m_sel_wrong: sel = P1 always outputs (0, 0, 0) on O + P2 instead of P2
  run mutantSelWrongCaughtTest = {
    val tr = traceAttackSelP1(CFG_SEL_WRONG)
    init.then(all {
      assert(isTraceAccepted(CFG_SEL_WRONG, tr)),
      assert(not(allRowsRefined(tr))),
      assert(not(isTraceAccepted(SOUND_CFG, tr))),
      st' = tr,
    })
  }

  // 9. Mutant m_out_flag_wrong: out_flag = f0 + f1 outputs flag 1 on O + P2 and flag 2 on O + O
  run mutantOutFlagWrongCaughtTest = {
    val tr = traceAttackOutFlagSum(CFG_OUT_FLAG_WRONG)
    init.then(all {
      assert(isTraceAccepted(CFG_OUT_FLAG_WRONG, tr)),
      assert(tr.sink_rows[0].f == 1),
      assert(tr.sink_rows[2].f == 2),
      assert(not(allRowsRefined(tr))),
      assert(not(isTraceAccepted(SOUND_CFG, tr))),
      st' = tr,
    })
  }

  // 10. Mutant m_out_mult_v: multiplicity v instead of v - e emits two outputs for one valid id
  run mutantOutMultVCaughtTest = {
    val tr = traceAttackDoubleOutput(CFG_OUT_MULT_V)
    init.then(all {
      assert(isTraceAccepted(CFG_OUT_MULT_V, tr)),
      assert(not(allRowsRefined(tr))),
      assert(not(isTraceAccepted(SOUND_CFG, tr))),
      st' = tr,
    })
  }

  // 11. Dropping or duplicating one message on the OUT bus fails LogUp multiset balance
  run dropOrDupMessageFailsLogUpTest = {
    val tr_base = traceHonestAdd(SOUND_CFG)
    val tr_drop: TraceState = { ...tr_base, fault_kind: 1 }
    val tr_dup: TraceState = { ...tr_base, fault_kind: 2 }
    init.then(all {
      assert(not(busBalanced(allBusEntries(SOUND_CFG, tr_drop)))),
      assert(not(busBalanced(allBusEntries(SOUND_CFG, tr_dup)))),
      st' = tr_base,
    })
  }
}
