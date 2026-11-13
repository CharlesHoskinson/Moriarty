// -*- mode: Bluespec; -*-
// Deterministic unit and negative tests for X14 PointsIn padding (is_valid / v column).

module x14_pad_test {
  import x14_pad_defs.* from "./x14_pad"

  // 1. Honest full unit (4 of 4 rows active) verifies and satisfies invRow
  run honestFullUnitTest = {
    val t = honestTrace(4)
    verifyTrace(t, true, true, false, false, false) and
    holdsInvRow(t) and
    countValidPin(t) == 4
  }

  // 2. Measured case "partial": partly filled unit (2 of 4 rows active, 2 padding)
  // verifies and satisfies invRow
  run honestPartialUnitTest = {
    val t = honestTrace(2)
    verifyTrace(t, true, true, false, false, false) and
    holdsInvRow(t) and
    countValidPin(t) == 2 and
    t.pin[2].v == 0 and t.pin[2].e == 0 and
    t.l_add[2].is_valid == 0 and t.y3_air[2].is_valid == 0 and t.sink[2].v == 0
  }

  // 3. Honest all-padding unit (0 of 4 rows active) verifies and satisfies invRow
  run honestEmptyUnitTest = {
    val t = honestTrace(0)
    verifyTrace(t, true, true, false, false, false) and
    holdsInvRow(t) and
    countValidPin(t) == 0
  }

  // 4. Measured negative case "pad_hole": a hole (row 1 padding, row 2 valid)
  // passes all chain/sink/range constraints and LogUp bus balance, failing ONLY
  // the PointsIn prefix transition constraint next.v * (1 - v) = 0
  run measuredPadHoleTest = {
    val t = holeTrace
    checkLAdd(t.l_add, true) and
    checkInv(t.inv_air, true) and
    checkX3(t.x3_air, true) and
    checkY3(t.y3_air, true) and
    checkSink(t.sink, true) and
    checkRangeBus(t, false) and
    checkPermutationBuses(t, false, false) and
    not(checkPointsIn(t.pin, true, true)) and
    checkPointsIn(t.pin, false, true) and
    not(verifyTrace(t, true, true, false, false, false))
  }

  // 5. Measured negative case "pad_forge": a padding row of PointsIn (row 2, v = 0)
  // whose chain rows and sink row are marked valid (is_valid = 1) passes all per-AIR
  // polynomial constraints and fails ONLY the LogUp bus balance on BUS_P1 / BUS_P2
  run measuredPadForgeTest = {
    val t = padForgeTrace
    checkPointsIn(t.pin, true, true) and
    checkLAdd(t.l_add, true) and
    checkInv(t.inv_air, true) and
    checkX3(t.x3_air, true) and
    checkY3(t.y3_air, true) and
    checkSink(t.sink, true) and
    checkRangeBus(t, false) and
    not(checkPermutationBuses(t, false, false)) and
    not(verifyTrace(t, true, true, false, false, false))
  }

  // 6. Mutant m_no_prefix accepts holeTrace, violating invRow while preserving per-id soundness
  run mutantNoPrefixCaughtTest = {
    val t = holeTrace
    verifyTrace(t, false, true, false, false, false) and
    not(holdsInvRow(t)) and
    holdsPerIdSoundness(t)
  }

  // 7. Mutant m_v_not_bool accepts nonBoolVTrace (v[0] = 2), violating invRow
  run mutantVNotBoolCaughtTest = {
    val t = nonBoolVTrace
    verifyTrace(t, true, false, false, false, false) and
    not(verifyTrace(t, true, true, false, false, false)) and
    not(holdsInvRow(t))
  }

  // 8. Mutant m_send_mult_one accepts sendMultOnePadZeroTrace (chain active on zero-point
  // padding rows), violating invRow, and rejects honest padded trace honestTrace(2)
  run mutantSendMultOneCaughtTest = {
    val t = sendMultOnePadZeroTrace
    verifyTrace(t, true, true, true, false, false) and
    not(verifyTrace(t, true, true, false, false, false)) and
    not(holdsInvRow(t)) and
    not(verifyTrace(honestTrace(2), true, true, true, false, false))
  }

  // 9. Mutant m_chain_valid_free accepts forgeY3ViaFreeValidTrace (forged y3 on row 0 with
  // Y3.is_valid[0] = 0), violating invRow
  run mutantChainValidFreeCaughtTest = {
    val t = forgeY3ViaFreeValidTrace
    verifyTrace(t, true, true, false, true, false) and
    not(verifyTrace(t, true, true, false, false, false)) and
    not(holdsInvRow(t))
  }

  // 10. Mutant m_range_mult_v accepts unCheckedPadLimbTrace (out-of-range limb P on padding row 2),
  // violating invRow (holdsPinLimbsInRange) while still satisfying operational holdsRefinement
  run mutantRangeMultVCaughtTest = {
    val t = unCheckedPadLimbTrace
    verifyTrace(t, true, true, false, false, true) and
    not(verifyTrace(t, true, true, false, false, false)) and
    not(holdsInvRow(t)) and
    holdsRefinement(t)
  }

  // 11. Sound verifier rejects dropping or duplicating a single bus message
  run soundRejectsSingleBusPerturbTest = {
    val base = honestTrace(2)
    val droppedP1 = {
      pin: base.pin, l_add: base.l_add, inv_air: base.inv_air,
      x3_air: base.x3_air, y3_air: base.y3_air, sink: base.sink,
      perturb: { active: true, bus: BUS_P1, row_id: 0, delta: -1 }
    }
    val dupedOut = {
      pin: base.pin, l_add: base.l_add, inv_air: base.inv_air,
      x3_air: base.x3_air, y3_air: base.y3_air, sink: base.sink,
      perturb: { active: true, bus: BUS_OUT, row_id: 1, delta: 1 }
    }
    not(verifyTrace(droppedP1, true, true, false, false, false)) and
    not(verifyTrace(dupedOut, true, true, false, false, false))
  }
}
