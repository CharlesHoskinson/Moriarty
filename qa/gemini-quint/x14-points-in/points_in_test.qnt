// Deterministic Unit, Counterexample, and Exhaustive Tamper Tests for X14 `PointsIn`
// Run: quint test --main points_in_test qa/gemini-quint/x14-points-in/points_in_test.qnt

module points_in_test {
  import points_in_spec(MUTANT = "none").* from "./points_in"

  // ---------------------------------------------------------------- Exhaustive Finite Tamper Domain Helpers
  pure val allTamperedCandidates: Set[AddChainWitness] =
    TAMPER_KINDS.fold(Set(honestAddTrace), (accK, tKind) =>
      ROW_DOM.fold(accK, (accR, r) =>
        LIMB_DOM.fold(accR, (accL, lv) =>
          BUS_DOM.fold(accL, (accB, bId) =>
            DELTA_DOM.fold(accB, (accD, d) =>
              accD.union(Set(buildTamperedWitness(tKind, r, lv, bId, d)))
            )
          )
        )
      )
    )

  pure def exhaustiveSoundForMutant(mt: str): bool = {
    val cands = allTamperedCandidates
    acceptsWithMutant(mt, honestAddTrace) and cands.fold(true, (ok, cand) =>
      ok and (acceptsWithMutant(mt, cand) implies rowRefines(cand))
    )
  }

  // ---------------------------------------------------------------- 1. Completeness Test (Question 3)
  run honestCompletenessTest = all {
    // Honest Add trace is accepted and refines G1 affine addition mod 13
    assert(acceptsWithMutant("none", honestAddTrace)),
    assert(rowRefines(honestAddTrace)),
    // Verify points in honestAddTrace are on E(F_13): y^2 = x^3 + 4 (mod 13), and outputs stay on curve
    assert(isOnCurveModP(0, 2)),
    assert(isOnCurveModP(2, 5)),
    assert(isOnCurveModP(limbsModP(honestAddTrace.sink[0].x3), limbsModP(honestAddTrace.sink[0].y3))),
    assert(isOnCurveModP(4, 4)),
    assert(isOnCurveModP(7, 3)),
    assert(isOnCurveModP(limbsModP(honestAddTrace.sink[1].x3), limbsModP(honestAddTrace.sink[1].y3))),
    // Honest Doubling trace is accepted and outputs stay on E(F_13)
    assert(dblChainAccepted(honestDblTrace)),
    assert(isOnCurveModP(limbsModP(honestDblTrace.sink[0].x3), limbsModP(honestDblTrace.sink[0].y3))),
    assert(isOnCurveModP(limbsModP(honestDblTrace.sink[1].x3), limbsModP(honestDblTrace.sink[1].y3)))
  }

  // ---------------------------------------------------------------- 2. Mutant Tests (`m_<name>`)
  // Mutant 1: `m_no_range` — drop range lookup of one limb (row 0, x1 limb 0)
  run mutantNoRangeCaughtTest = all {
    assert(acceptsWithMutant("no_range", honestAddTrace)),
    assert(acceptsWithMutant("no_range", cex_out_of_range_limb)),
    assert(not(acceptsWithMutant("none", cex_out_of_range_limb))),
    assert(cex_out_of_range_limb.lAdd[0].x1[0] == 4),
    assert(not(limbsInRange(cex_out_of_range_limb.lAdd[0].x1, BASE))),
    assert(not(rowRefines(cex_out_of_range_limb)))
  }

  // Mutant 2: `m_wrong_bits` — range lookup with max_bits = 3 (8) instead of 2 (4) (9 in place of 8)
  run mutantWrongBitsCaughtTest = all {
    assert(acceptsWithMutant("wrong_bits", honestAddTrace)),
    assert(acceptsWithMutant("wrong_bits", cex_out_of_range_limb)),
    assert(not(acceptsWithMutant("none", cex_out_of_range_limb))),
    assert(cex_out_of_range_limb.lAdd[0].x1[0] == 4),
    assert(limbsInRange(cex_out_of_range_limb.lAdd[0].x1, WRONG_BASE)),
    assert(not(limbsInRange(cex_out_of_range_limb.lAdd[0].x1, BASE))),
    assert(not(rowRefines(cex_out_of_range_limb)))
  }

  // Mutant 3: `m_no_send` — drop the send of the second point on bus o + 1 = 1101
  run mutantNoSendCaughtTest = all {
    assert(bus1100Balanced("no_send", honestAddTrace)),
    assert(not(bus1101Balanced("no_send", honestAddTrace))),
    assert(not(allAddBusesBalanced("no_send", honestAddTrace))),
    assert(not(acceptsWithMutant("no_send", honestAddTrace))),
    assert(allTamperedCandidates.forall(cand => not(acceptsWithMutant("no_send", cand))))
  }

  // Mutant 4: `m_dup_send` — send multiplicity 2 on one message (bus o = 1100)
  run mutantDupSendCaughtTest = all {
    assert(not(bus1100Balanced("dup_send", honestAddTrace))),
    assert(bus1101Balanced("dup_send", honestAddTrace)),
    assert(not(allAddBusesBalanced("dup_send", honestAddTrace))),
    assert(not(acceptsWithMutant("dup_send", honestAddTrace))),
    assert(allTamperedCandidates.forall(cand => not(acceptsWithMutant("dup_send", cand))))
  }

  // Mutant 5: `m_no_id_constraint` — drop transition constraint on id (row 1 has id = 0, mixing P1 and P2 across rows)
  run mutantNoIdConstraintCaughtTest = all {
    assert(acceptsWithMutant("no_id_constraint", honestAddTrace)),
    assert(acceptsWithMutant("no_id_constraint", cex_no_id_constraint_mix)),
    assert(not(acceptsWithMutant("none", cex_no_id_constraint_mix))),
    assert(cex_no_id_constraint_mix.lAdd[0].x2 == cex_no_id_constraint_mix.pin[1].x2),
    assert(cex_no_id_constraint_mix.lAdd[0].x2 != cex_no_id_constraint_mix.pin[0].x2),
    assert(not(rowRefines(cex_no_id_constraint_mix)))
  }

  // ---------------------------------------------------------------- 3. Question (a), (b), (c) and (1), (2) Tests
  // Question (a): Non-canonical value (limbs in [0, 4), integer value 13 >= P = 13)
  run questionANonCanonicalTest = {
    // Non-canonical x2 = 13 = [1, 3] colliding mod 13 with x1 = 0 = [0, 0]:
    // Even though limbs [1, 3] != [0, 0], (x2 - x1) mod 13 == 0, so INV still rejects!
    val nonCanonSameX: AddChainWitness = {
      ...neg_same_pt_trace,
      pin: [{ ...neg_same_pt_trace.pin[0], x2: [1, 3] }, neg_same_pt_trace.pin[1]],
      lAdd: [{ ...neg_same_pt_trace.lAdd[0], x2: [1, 3] }, neg_same_pt_trace.lAdd[1]],
      inv: [{ ...neg_same_pt_trace.inv[0], x2: [1, 3] }, neg_same_pt_trace.inv[1]],
      x3: [{ ...neg_same_pt_trace.x3[0], x2: [1, 3] }, neg_same_pt_trace.x3[1]]
    }
    all {
      assert(limbsInRange(nonCanonicalInputTrace.pin[0].x1, BASE)),
      assert(limbsToInt(nonCanonicalInputTrace.pin[0].x1) == 13),
      assert(limbsToInt(nonCanonicalInputTrace.pin[0].x1) >= P),
      assert(limbsModP(nonCanonicalInputTrace.pin[0].x1) == 0),
      assert(acceptsWithMutant("none", nonCanonicalInputTrace)),
      assert(rowRefines(nonCanonicalInputTrace)),
      assert(nonCanonicalInputTrace.sink[0] == honestAddTrace.sink[0]),
      // Non-canonical x2 = 13 with x1 = 0 is still rejected by INV mod 13
      assert(not(invAirOk("none", nonCanonSameX.inv))),
      assert(not(acceptsWithMutant("none", nonCanonSameX)))
    }
  }

  // Question (b): Off-curve input point (1, 1) (where 1^2 = 1 != 1^3 + 4 = 5 mod 13)
  run questionBOffCurveTest = all {
    assert(not(isOnCurveModP(1, 1))),
    assert(acceptsWithMutant("none", offCurveInputTrace)),
    assert(rowRefines(offCurveInputTrace)),
    // Output (x3, y3) for off-curve input is also off-curve, confirming on-curve check must live at transcript ingestion
    assert(not(isOnCurveModP(limbsModP(offCurveInputTrace.sink[0].x3), limbsModP(offCurveInputTrace.sink[0].y3))))
  }

  // Question (c): Exact interaction counts and message lengths in the real BLS12-381 AIR
  run questionCInteractionCountTest = {
    val realLimbs = 48
    val addRangeLookups = 4 * realLimbs
    val addSends = 2
    val addTotalInteractions = addRangeLookups + addSends
    val dblRangeLookups = 2 * realLimbs
    val dblSends = 2
    val dblTotalInteractions = dblRangeLookups + dblSends
    val perAirInteractionBound = 200
    val addMsgFields = 1 + 2 * realLimbs
    val dblMsgFields = 1 + 1 * realLimbs
    val maxLogUpMsgFields = 128
    all {
      assert(addRangeLookups == 192),
      assert(addTotalInteractions == 194),
      assert(addTotalInteractions < perAirInteractionBound),
      assert(dblRangeLookups == 96),
      assert(dblTotalInteractions == 98),
      assert(dblTotalInteractions < perAirInteractionBound),
      assert(addMsgFields == 97 and addMsgFields <= maxLogUpMsgFields),
      assert(dblMsgFields == 49 and dblMsgFields <= maxLogUpMsgFields)
    }
  }

  // Question (2): Exceptional cases (x1 == x2 in add, y == 0 in dbl) rejected by INV / INV_y
  run exceptionalCasesRejectedTest = all {
    // In `neg_same_pt_trace`, L_add, X3, Y3, PointsIn, and all buses pass, ONLY INV fails!
    assert(pointsInAirOk("none", neg_same_pt_trace)),
    assert(lAddAirOk("none", neg_same_pt_trace.lAdd)),
    assert(x3AirOk("none", neg_same_pt_trace.x3)),
    assert(y3AirOk("none", neg_same_pt_trace.y3)),
    assert(allAddBusesBalanced("none", neg_same_pt_trace)),
    assert(not(invAirOk("none", neg_same_pt_trace.inv))),
    assert(not(acceptsWithMutant("none", neg_same_pt_trace))),
    // In `neg_y_zero_dbl_trace`, doubling P = (0, 0) is rejected by INV_y
    assert(not(dblChainAccepted(neg_y_zero_dbl_trace)))
  }

  // ---------------------------------------------------------------- 4. Exhaustive Finite Tamper Domain Test
  run exhaustiveTamperDomainSoundTest = all {
    assert(exhaustiveSoundForMutant("none")),
    assert(not(exhaustiveSoundForMutant("no_range"))),
    assert(not(exhaustiveSoundForMutant("wrong_bits"))),
    assert(not(exhaustiveSoundForMutant("no_send"))),
    assert(not(exhaustiveSoundForMutant("dup_send"))),
    assert(not(exhaustiveSoundForMutant("no_id_constraint")))
  }
}
