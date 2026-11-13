// -*- mode: Bluespec; -*-
//
// Deterministic unit, witness, mutant, and exhaustive single-cell tamper tests
// for the X14 BLS12-381 G1 chain Y3 STARK AIR row kind (`y3.qnt`).
//
// Run with:
//   quint test qa/gemini-quint/x14-y3/y3_test.qnt --main=y3_test

module y3_test {
  import Y3Defs.* from "./y3"

  // 1. Completeness of `g1_add` on two active rows:
  //    Row 0: P1 = (0, 2) + P2 = (2, 5) -> lam = 8, x3 = 10, y3 = 9
  //    Row 1: P3 = (4, 4) + P4 = (7, 3) -> lam = 4, x3 = 5,  y3 = 5
  run completenessAddTest = {
    val st = honestTrace(0, 1, 0)
    all {
      assert(onCurve(0, 2)),
      assert(onCurve(2, 5)),
      assert(onCurve(10, 9)),
      assert(onCurve(4, 4)),
      assert(onCurve(7, 3)),
      assert(onCurve(5, 5)),
      assert(affineLambdaAdd(0, 2, 2, 5) == 8),
      assert(affineX3(8, 0, 2) == 10),
      assert(affineY3(8, 0, 10, 2) == 9),
      assert(affineLambdaAdd(4, 4, 7, 3) == 4),
      assert(affineX3(4, 4, 7) == 5),
      assert(affineY3(4, 4, 5, 4) == 5),
      assert(verifierAccepts(st, 0)),
      assert(invRowProperty(st))
    }
  }

  // 2. Completeness of `g1_double` on two active rows:
  //    Row 0: 2 * (2, 5) -> lam = 9, x3 = 12, y3 = 9
  //    Row 1: 2 * (4, 4) -> lam = 6, x3 = 2,  y3 = 8
  run completenessDblTest = {
    val st = honestTrace(1, 1, 0)
    all {
      assert(onCurve(2, 5)),
      assert(onCurve(12, 9)),
      assert(onCurve(4, 4)),
      assert(onCurve(2, 8)),
      assert(affineLambdaDbl(2, 5) == 9),
      assert(affineX3(9, 2, 2) == 12),
      assert(affineY3(9, 2, 12, 5) == 9),
      assert(affineLambdaDbl(4, 4) == 6),
      assert(affineX3(6, 4, 4) == 2),
      assert(affineY3(6, 4, 2, 4) == 8),
      assert(verifierAccepts(st, 0)),
      assert(invRowProperty(st))
    }
  }

  // 3. Completeness with an inactive padding row (`is_valid = 0` on row 1):
  //    Verifies that inactive padding rows are silenced on buses o+2..o+7 and accepted by sound Y3.
  run paddingRowInactiveCompletenessTest = {
    val stAddPad = honestTrace(0, 0, 0)
    val stDblPad = honestTrace(1, 0, 0)
    all {
      assert(stAddPad.y0.is_valid == 1),
      assert(stAddPad.y1_row.is_valid == 0),
      assert(verifierAccepts(stAddPad, 0)),
      assert(invRowProperty(stAddPad)),
      assert(stDblPad.y0.is_valid == 1),
      assert(stDblPad.y1_row.is_valid == 0),
      assert(verifierAccepts(stDblPad, 0)),
      assert(invRowProperty(stDblPad))
    }
  }

  // 4. Non-vacuity of the honest witness under mutants:
  //    - Mutants M1 (`m_no_recv_x3`), M3 (`m_no_recv_y1`), and M5 (`m_valid_free`, on active rows)
  //      accept the canonical honest witness and satisfy `invRowProperty`, proving those mutants are
  //      caught by real attacks rather than vacuous honest rejection.
  //    - Mutants M2 (`m_out_wrong_x`) and M4 (`m_sign_flip`) alter the honest emission/equation itself,
  //      so the canonical honest witness is rejected by M2 and M4 while the mutant-adapted layout
  //      violates `invRowProperty`.
  run honestUnderMutantsNonVacuityTest = {
    val honestSt = honestTrace(0, 1, 0)
    all {
      // M1 (`m_no_recv_x3`) accepts the canonical honest witness and refines
      assert(verifierAccepts(honestSt, 1)),
      assert(invRowProperty(honestSt)),
      // M3 (`m_no_recv_y1`) accepts the canonical honest witness and refines
      assert(verifierAccepts(honestSt, 3)),
      assert(invRowProperty(honestSt)),
      // M5 (`m_valid_free`) accepts the canonical 2-active-row honest witness and refines
      assert(verifierAccepts(honestSt, 5)),
      assert(invRowProperty(honestSt)),
      // M2 (`m_out_wrong_x`) and M4 (`m_sign_flip`) alter the honest bus/equation layout itself
      assert(not(verifierAccepts(honestSt, 2))),
      assert(not(verifierAccepts(honestSt, 4)))
    }
  }

  // 5. Mutant 1 (`m_no_recv_x3`): dropping Y3's bus o+6 receive allows forging `x3` in Y3.
  //    Here `forgedX3 = 0 != 10` yields `forgedY3 = 8 * (0 - 0) - 2 = 11 != 9 (mod 13)`.
  //    Accepted by `m_no_recv_x3` (`mId == 1`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  run mutantNoRecvX3CaughtTest = {
    val st = attackTraceNoRecvX3(0)
    all {
      assert(st.x0.x3 == 10),
      assert(st.y0.x3 == 0),
      assert(st.y0.y3 == 11),
      assert(verifierAccepts(st, 1)),
      assert(not(verifierAccepts(st, 0))),
      assert(not(bus6Balanced(st, 0))),
      assert(not(invRowProperty(st)))
    }
  }

  // 6. Mutant 2 (`m_out_wrong_x`): sending `(id, x1, y3)` instead of `(id, x3, y3)` on bus o+7
  //    causes the downstream sink to receive `out_x = x1 = 0` instead of `x3 = 10`.
  //    Accepted by `m_out_wrong_x` (`mId == 2`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  run mutantOutWrongXCaughtTest = {
    val st = attackTraceOutWrongX
    all {
      assert(st.y0.x1 == 0),
      assert(st.y0.x3 == 10),
      assert(st.s0.out_x == 0),
      assert(st.s0.out_x != st.x0.x3),
      assert(verifierAccepts(st, 2)),
      assert(not(verifierAccepts(st, 0))),
      assert(not(bus7Balanced(st, 0))),
      assert(not(invRowProperty(st)))
    }
  }

  // 7. Mutant 3 (`m_no_recv_y1`): dropping Y3's bus o+5 receive allows forging `y1` in Y3.
  //    Here `forgedY1 = 0 != 2` yields `forgedY3 = 8 * (0 - 10) - 0 = 11 != 9 (mod 13)`.
  //    Accepted by `m_no_recv_y1` (`mId == 3`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  run mutantNoRecvY1CaughtTest = {
    val st = attackTraceNoRecvY1(0)
    all {
      assert(st.l0.y1 == 2),
      assert(st.y0.y1 == 0),
      assert(st.y0.y3 == 11),
      assert(verifierAccepts(st, 3)),
      assert(not(verifierAccepts(st, 0))),
      assert(not(bus5Balanced(st, 0))),
      assert(not(invRowProperty(st)))
    }
  }

  // 8. Mutant 4 (`m_sign_flip`): replacing `x1 - x3` with `x3 - x1` in Y3's polynomial
  //    forces `y3 = 8 * (10 - 0) - 2 = 0 != 9 (mod 13)`, which is not even on E(F_13) (`0^2 != 10^3 + 4 mod 13`).
  //    Accepted by `m_sign_flip` (`mId == 4`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  run mutantSignFlipCaughtTest = {
    val st = attackTraceSignFlip
    all {
      assert(st.y0.y3 == 0),
      assert(affineY3(st.y0.lam, st.y0.x1, st.y0.x3, st.y0.y1) == 9),
      assert(not(onCurve(st.y0.x3, st.y0.y3))),
      assert(verifierAccepts(st, 4)),
      assert(not(verifierAccepts(st, 0))),
      assert(not(y3RowValid(st.y0, 0))),
      assert(not(invRowProperty(st)))
    }
  }

  // 9. Mutant 5 (`m_valid_free`): sending on bus o+7 with multiplicity `1` instead of `is_valid`
  //    allows an inactive padding row (`y1_row.is_valid == 0`, where the polynomial constraint is disabled)
  //    to inject an arbitrary forged point `(5, 0)` into an active downstream sink (`s1.is_valid == 1`).
  //    Accepted by `m_valid_free` (`mId == 5`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  run mutantValidFreeCaughtTest = {
    val st = attackTraceValidFree(0)
    all {
      assert(st.y1_row.is_valid == 0),
      assert(st.y1_row.y3 == 0),
      assert(st.s1.is_valid == 1),
      assert(st.s1.out_y == 0),
      assert(verifierAccepts(st, 5)),
      assert(not(verifierAccepts(st, 0))),
      assert(not(bus7Balanced(st, 0))),
      assert(not(invRowProperty(st)))
    }
  }

  // 10. Question (1) Attack A: Duplicate `id = 1` across two active rows allows swapping `y1` on bus o+5!
  //     Row 0 (P1=(0,2), P2=(2,5), honest out=(10,9)) and Row 1 (P3=(4,4), P4=(7,3), honest out=(5,5))
  //     both use `id = 1`. Swapping `y1` on bus o+5 produces forged outputs `(10, 7)` and `(5, 7)`
  //     (neither of which lies on E(F_13)), yet the sound verifier accepts the trace!
  run duplicateIdSwapAttackTest = {
    val st = duplicateIdSwapY1Trace
    all {
      assert(not(distinctActiveIds(st))),
      assert(verifierAccepts(st, 0)),
      assert(st.y0.y1 == 4),
      assert(st.l0.y1 == 2),
      assert(st.y0.y3 == 7),
      assert(affineY3(st.l0.lam, st.l0.x1, st.x0.x3, st.l0.y1) == 9),
      assert(st.y1_row.y1 == 2),
      assert(st.l1.y1 == 4),
      assert(st.y1_row.y3 == 7),
      assert(affineY3(st.l1.lam, st.l1.x1, st.x1_row.x3, st.l1.y1) == 5),
      assert(not(onCurve(st.s0.out_x, st.s0.out_y))),
      assert(not(onCurve(st.s1.out_x, st.s1.out_y)))
    }
  }

  // 11. Question (1) Attack B: Duplicate `id = 1` across two active rows also allows swapping `x3` on bus o+6!
  //     Swapping `x3` on bus o+6 produces forged outputs `(5, 10)` and `(10, 11)`, accepted by sound `y3`.
  run duplicateIdSwapX3AttackTest = {
    val st = duplicateIdSwapX3Trace
    all {
      assert(not(distinctActiveIds(st))),
      assert(verifierAccepts(st, 0)),
      assert(st.y0.x3 == 5),
      assert(st.x0.x3 == 10),
      assert(st.y0.y3 == 10),
      assert(st.y1_row.x3 == 10),
      assert(st.x1_row.x3 == 5),
      assert(st.y1_row.y3 == 11),
      assert(not(onCurve(st.s0.out_x, st.s0.out_y))),
      assert(not(onCurve(st.s1.out_x, st.s1.out_y)))
    }
  }

  // 12. Question (2) Attack A: Exceptional `g1_add` with P1 == P2 == (2, 5) (`x1 == x2` and `y1 == y2`).
  //     In `L_add`, `(x2 - x1) * lam - (y2 - y1) == 0 * lam - 0 == 0` holds for ALL 13 values of `lam` in F_13!
  //     Every `chosenLam` in F_13 is accepted by the sound verifier and yields a distinct forged output `(x3, y3)`.
  run exceptionalSamePointLambdaFreeTest = {
    val stLam0 = exceptionalSamePointAddTrace(0)
    val stLam1 = exceptionalSamePointAddTrace(1)
    val stLam9 = exceptionalSamePointAddTrace(9)
    all {
      assert(F13.forall(chosenLam => verifierAccepts(exceptionalSamePointAddTrace(chosenLam), 0))),
      // For chosenLam = 0: x3 = 9, y3 = 8 (not on curve: 8^2 = 12 != 9^3 + 4 = 5 mod 13)
      assert(stLam0.s0.out_x == 9),
      assert(stLam0.s0.out_y == 8),
      assert(not(onCurve(stLam0.s0.out_x, stLam0.s0.out_y))),
      // For chosenLam = 1: x3 = 10, y3 = 0 (not on curve: 0^2 = 0 != 10^3 + 4 = 3 mod 13)
      assert(stLam1.s0.out_x == 10),
      assert(stLam1.s0.out_y == 0),
      assert(not(onCurve(stLam1.s0.out_x, stLam1.s0.out_y))),
      // Only chosenLam = 9 matches true 2 * (2, 5) = (12, 9)
      assert(stLam9.s0.out_x == 12),
      assert(stLam9.s0.out_y == 9),
      assert(onCurve(stLam9.s0.out_x, stLam9.s0.out_y))
    }
  }

  // 13. Question (2) Case B: Exceptional `g1_add` with inverse points P1 = (2, 5), P2 = (2, 8) = -P1 (`x1 == x2`, `y1 != y2`).
  //     In `L_add`, `(x2 - x1) * lam - (y2 - y1) == 0 * lam - 3 == 10 != 0 (mod 13)` has NO solution `lam` in F_13
  //     when `is_valid == 1`.
  run exceptionalInversePointUnsatisfiableTest = {
    all {
      assert(onCurve(2, 5)),
      assert(onCurve(2, 8)),
      assert(fAdd(5, 8) == 0),
      assert(F13.forall(candidateLam => {
        val lr: LRow = { id: 1, is_valid: 1, x1: 2, y1: 5, x2: 2, y2: 8, lam: candidateLam }
        not(lRowValid(0, lr))
      }))
    }
  }

  // 14. Question (2) Case C: `g1_double` with `y1 == 0`.
  //     - For any `x1 != 0` in F_13, `3 * x1^2 != 0 (mod 13)`, so `(2 * 0) * lam == 3 * x1^2` has NO solution `lam`.
  //     - For the off-curve point `(0, 0)`, `0 * lam == 0` holds for ALL 13 values of `lam` in F_13!
  run exceptionalDoubleZeroPointLambdaFreeTest = {
    val nonZeroF13 = Set(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
    all {
      assert(not(onCurve(0, 0))),
      assert(nonZeroF13.forall(x =>
        F13.forall(candidateLam => {
          val lr: LRow = { id: 1, is_valid: 1, x1: x, y1: 0, x2: x, y2: 0, lam: candidateLam }
          not(lRowValid(1, lr))
        })
      )),
      assert(F13.forall(chosenLam => verifierAccepts(exceptionalDoubleZeroTrace(chosenLam), 0)))
    }
  }

  // 15. Exhaustive single-cell tamper check on sound Y3 (`allSingleCellTampersSound`):
  //     Across all 7 columns (`id`, `is_valid`, `lam`, `x1`, `x3`, `y1`, `y3`), all 13 values in F_13,
  //     and both recomputation modes (`recomputeY3` in {false, true}), every trace accepted by sound Y3
  //     satisfies `invRowProperty`.
  run exhaustiveSoundSingleCellTamperTest = {
    assert(allSingleCellTampersSound)
  }
}
