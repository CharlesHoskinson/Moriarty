// Unit, Counterexample, and Exhaustive Pure Tests — Lane `packsel`: Packed Selectors
// Run: quint test --main packsel_test qa/gemini-quint/chipleaf-final/packsel_test.qnt

module packsel_test {
  import packsel(MUTANT = "none").* from "./packsel"

  // ---------------------------------------------------------------- Exhaustive Pure Layout Domain
  // Build the complete finite set of 93 distinct layout variants:
  //   - Arch::Narrow: s in 1..3 (3 variants; body degrees fixed to measured [2, 3, 2, 2, 1, 1, 2])
  //   - Arch::Wide: s in 1..3, wideDeg in 1..5 (15 variants)
  //   - Arch::Staggered: s in 1..3, stagDeg0 in 1..5, stagDeg1 in 1..5 (75 variants)
  pure val allLayoutConfigs: List[LayCfg] = {
    val emptyCfgs: List[LayCfg] = []
    val withNarrow = range(1, 4).foldl(emptyCfgs, (accS, sVal) =>
      accS.append({ arch: ARCH_NARROW, s: sVal, wideDeg: 3, stagDeg0: 2, stagDeg1: 3 })
    )
    val withWide = range(1, 4).foldl(withNarrow, (accS, sVal) =>
      range(1, 6).foldl(accS, (accD, wDeg) =>
        accD.append({ arch: ARCH_WIDE, s: sVal, wideDeg: wDeg, stagDeg0: 2, stagDeg1: 3 })
      )
    )
    range(1, 4).foldl(withWide, (accS, sVal) =>
      range(1, 6).foldl(accS, (accD0, d0) =>
        range(1, 6).foldl(accD0, (accD1, d1) =>
          accD1.append({ arch: ARCH_STAGGERED, s: sVal, wideDeg: 3, stagDeg0: d0, stagDeg1: d1 })
        )
      )
    )
  }

  // Exhaustively check that for every layout variant in `allLayoutConfigs`, `accepts implies refines`
  pure def exhaustiveLayoutsSound(mt: str): bool = {
    val cfgs = allLayoutConfigs
    cfgs.indices().forall(i => {
      val lay = buildLayout(mt, cfgs[i])
      acceptsWithMutant(mt, lay) implies refinesWith(mt, lay)
    })
  }

  // Check that there exists an accepted layout variant in `allLayoutConfigs` that violates `refinesWith`
  pure def exhaustiveMutantCaught(mt: str): bool = {
    val cfgs = allLayoutConfigs
    not(cfgs.indices().forall(i => {
      val lay = buildLayout(mt, cfgs[i])
      acceptsWithMutant(mt, lay) implies refinesWith(mt, lay)
    }))
  }

  // ---------------------------------------------------------------- 1. Replay of `vsel::pack` & Sound Baseline Tests
  // Verify exact replay of `pack` on Layout B (`Arch::Narrow`) with measured degrees [2, 3, 2, 2, 1, 1, 2] + cnt0 (1)
  run narrowPackReplayTest = all {
    assert(allLayoutConfigs.length() == 93),
    assert(acceptsWithMutant("none", hw_narrow_s1)),
    assert(refinesWith("none", hw_narrow_s1)),
    // 6 non-DEC core gates [3, 2, 2, 1, 1, 2] + rf count0 [1] pack into 2 code columns:
    assert(hw_narrow_s1.packedCoreCols == 2),
    // gi = 0 (slot row 1, bodyDeg 2) is dgi and uses Simple(COL_Q_DEC) of degree 1 (total degree 3):
    assert(hw_narrow_s1.gates[0].sel == { kind: SEL_SIMPLE, col: COL_Q_DEC, codeVal: 1, nCodes: 1, selDeg: 1 }),
    assert(hw_narrow_s1.gates[0].bodyDeg == 2),
    // Packed Column 0 (COL_CORE_BASE + 0, n = 3): gi = 1 (row 3, deg 3, code 1), gi = 2 (row 5, deg 2, code 2), gi = 3 (row 7, deg 2, code 3)
    assert(hw_narrow_s1.gates[1].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE + 0, codeVal: 1, nCodes: 3, selDeg: 3 }),
    assert(hw_narrow_s1.gates[2].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE + 0, codeVal: 2, nCodes: 3, selDeg: 3 }),
    assert(hw_narrow_s1.gates[3].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE + 0, codeVal: 3, nCodes: 3, selDeg: 3 }),
    // Packed Column 1 (COL_CORE_BASE + 1, n = 4): gi = 6 (row 12, deg 2, code 1), gi = 4 (row 9, deg 1, code 2),
    //                                              gi = 5 (row 11, deg 1, code 3), rf count0 (row 0, deg 1, code 4)
    assert(hw_narrow_s1.gates[6].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE + 1, codeVal: 1, nCodes: 4, selDeg: 4 }),
    assert(hw_narrow_s1.gates[4].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE + 1, codeVal: 2, nCodes: 4, selDeg: 4 }),
    assert(hw_narrow_s1.gates[5].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE + 1, codeVal: 3, nCodes: 4, selDeg: 4 }),
    assert(hw_narrow_s1.gates[7].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE + 1, codeVal: 4, nCodes: 4, selDeg: 4 }),
    // Max gate degree is 6 (gi = 1 has 3 + 3 = 6; gi = 6 has 4 + 2 = 6), max lookup degree is 6 (bus and dec are 6):
    assert(maxGateDegree(hw_narrow_s1) == 6),
    assert(maxLookupDegree(hw_narrow_s1) == 6),
    // Post-state slot s = 1 (rows 13..25) has no core gate enabled on any row:
    assert(range(13, 26).foldl(true, (ok, r) => ok and (activeGatesOnRow("none", hw_narrow_s1, r) == Set()))),
  }

  // Verify Wide, Staggered (shared column when stagDeg1 <= 4, split columns when stagDeg1 == 5), and Narrow (S = 3)
  run wideAndStaggeredLayoutsSoundTest = all {
    assert(acceptsWithMutant("none", hw_narrow_s3) and refinesWith("none", hw_narrow_s3)),
    assert(acceptsWithMutant("none", hw_wide_s1) and refinesWith("none", hw_wide_s1)),
    assert(hw_wide_s1.packedCoreCols == 1),
    // On Staggered with stagDeg1 = 3, gi = 1 (row 2) and rf count0 (row 1) share 1 column of n = 2 codes (deg 2 + 3 = 5 <= 6):
    assert(acceptsWithMutant("none", hw_stag_shared) and refinesWith("none", hw_stag_shared)),
    assert(hw_stag_shared.packedCoreCols == 1),
    assert(hw_stag_shared.gates[1].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE, codeVal: 1, nCodes: 2, selDeg: 2 }),
    assert(hw_stag_shared.gates[2].sel == { kind: SEL_LAGRANGE, col: COL_CORE_BASE, codeVal: 2, nCodes: 2, selDeg: 2 }),
    // On Staggered with stagDeg1 = 5, 2 + 5 = 7 > 6, so `pack` splits gi = 1 and rf count0 into 2 columns of n = 1 each:
    assert(acceptsWithMutant("none", hw_stag_split) and refinesWith("none", hw_stag_split)),
    assert(hw_stag_split.packedCoreCols == 2),
    assert(maxGateDegree(hw_stag_split) == 6),
  }

  // Verify Lagrange polynomial exact rational evaluation over 0..=n and outside 0..=n
  run lagrangeIndicatorPropertyTest = all {
    assert(
      range(1, 6).foldl(true, (okN, n) =>
        range(1, n + 1).foldl(okN, (okC, c) =>
          range(0, n + 1).foldl(okC, (okQ, q) => {
            val r = lagrangeRat("none", q, c, n)
            val wantOk = if (q == c) (r.num == r.den and r.den != 0) else (r.num == 0 and r.den != 0)
            okQ and wantOk
          })
        )
      )
    ),
    // Outside 0..=n (e.g. q = n + 1), L_c(q) is nonzero (confirming polynomial evaluation, not `q == c`):
    assert(lagrangeRat("none", 4, 1, 3).num != 0),
  }

  // ---------------------------------------------------------------- 2. Mutant Counterexample Tests (P1..P9)
  // P1 `same_code`: `pack` gives two members of a column (`gi = 1` at row 3 and `gi = 2` at row 5) the same code 1
  run mutantP1_sameCodeTest = all {
    assert(acceptsWithMutant("same_code", cex_P1_same_code)),
    assert(not(acceptsWithMutant("none", cex_P1_same_code))),
    assert(not(refinesWith("same_code", cex_P1_same_code))),
    // On row 3 and row 5, BOTH core gate 1 and core gate 2 are selected simultaneously:
    assert(activeGatesOnRow("same_code", cex_P1_same_code, 3) == Set(GATE_CORE_BASE + 1, GATE_CORE_BASE + 2)),
    assert(activeGatesOnRow("same_code", cex_P1_same_code, 5) == Set(GATE_CORE_BASE + 1, GATE_CORE_BASE + 2)),
  }

  // P2 `cnt_collides`: `rf count0` enabled on row 12 (`gi = 6`), overwriting code 1 with code 4
  run mutantP2_cntCollidesTest = all {
    assert(acceptsWithMutant("cnt_collides", cex_P2_cnt_collides)),
    assert(not(acceptsWithMutant("none", cex_P2_cnt_collides))),
    assert(not(refinesWith("cnt_collides", cex_P2_cnt_collides))),
    // Row 12 loses core gate `gi = 6` (`en prefix` and `count`), keeping only `GATE_RF_CNT0`:
    assert(activeGatesOnRow("cnt_collides", cex_P2_cnt_collides, 12) == Set(GATE_RF_CNT0)),
    assert(refGatesOnRow(cex_P2_cnt_collides, 12) == Set(GATE_CORE_BASE + 6)),
    // Honest advice witness under P2 is accepted and refines, while forged advice on row 12 (`gi = 6`) is accepted:
    assert(adviceAcceptedWith("cnt_collides", honestAdviceWitness("cnt_collides", DEFAULT_CFG))),
    assert(adviceRefinesWith("cnt_collides", honestAdviceWitness("cnt_collides", DEFAULT_CFG))),
    assert(adviceAcceptedWith("cnt_collides", forgedAdviceWitness("cnt_collides", DEFAULT_CFG, GATE_CORE_BASE + 6, 12))),
    assert(not(adviceAcceptedWith("none", forgedAdviceWitness("none", DEFAULT_CFG, GATE_CORE_BASE + 6, 12)))),
    assert(not(adviceRefinesWith("cnt_collides", forgedAdviceWitness("cnt_collides", DEFAULT_CFG, GATE_CORE_BASE + 6, 12)))),
  }

  // P3 `lagrange_skips_zero`: Lagrange product runs over 1..=n (0 not excluded), so unassigned `q = 0` rows are selected
  run mutantP3_lagrangeSkipsZeroTest = all {
    assert(acceptsWithMutant("lagrange_skips_zero", cex_P3_lagrange_skips_zero)),
    assert(not(refinesWith("lagrange_skips_zero", cex_P3_lagrange_skips_zero))),
    // On row 0, `COL_CORE_BASE + 0` has q = 0, yet all 3 gates of that column evaluate to nonzero:
    assert(lagrangeRat("lagrange_skips_zero", 0, 1, 3).num != 0),
    assert(activeGatesOnRow("lagrange_skips_zero", cex_P3_lagrange_skips_zero, 0).contains(GATE_CORE_BASE + 1)),
    assert(activeGatesOnRow("lagrange_skips_zero", cex_P3_lagrange_skips_zero, 0).contains(GATE_HASH_FIRST)),
  }

  // P4 `pack_bound_7`: `pack` admits `members + max degree <= 7`, packing 4 gates into col 0 with max body deg 3 (total deg 7)
  run mutantP4_packBound7Test = all {
    assert(acceptsWithMutant("pack_bound_7", cex_P4_pack_bound_7)),
    assert(not(acceptsWithMutant("none", cex_P4_pack_bound_7))),
    assert(not(refinesWith("pack_bound_7", cex_P4_pack_bound_7))),
    assert(activeGatesMatchRef("pack_bound_7", cex_P4_pack_bound_7)),
    assert(not(gateDegreesOk(cex_P4_pack_bound_7))),
    assert(maxGateDegree(cex_P4_pack_bound_7) == 7),
  }

  // P5 `chain_on_last`: `mem chain` uses `q * (q - 2)` (off for code 2 on rows 1..6, on for code 3 on row 7)
  run mutantP5_chainOnLastTest = {
    val ms = cex_P5_chain_on_last.memStart
    all {
      assert(acceptsWithMutant("chain_on_last", cex_P5_chain_on_last)),
      assert(not(refinesWith("chain_on_last", cex_P5_chain_on_last))),
      // Middle MEM rows 1..6 (q = 2) lose `GATE_MEM_CHAIN`:
      assert(not(activeGatesOnRow("chain_on_last", cex_P5_chain_on_last, ms + 1).contains(GATE_MEM_CHAIN))),
      // Last MEM row 7 (q = 3) wrongly enables `GATE_MEM_CHAIN` (reading the next region's row):
      assert(activeGatesOnRow("chain_on_last", cex_P5_chain_on_last, ms + 7).contains(GATE_MEM_CHAIN)),
    }
  }

  // P6 `walk_lagrange1`: `m3 walk` uses `L_1(q)` instead of raw `q`, dropping `m3 walk` on old walk levels 0 and 16
  run mutantP6_walkLagrange1Test = {
    val ws = cex_P6_walk_lagrange1.walkStart
    all {
      assert(acceptsWithMutant("walk_lagrange1", cex_P6_walk_lagrange1)),
      assert(not(acceptsWithMutant("none", cex_P6_walk_lagrange1))),
      assert(not(refinesWith("walk_lagrange1", cex_P6_walk_lagrange1))),
      // Old walk level 0 (ws + 0, q = 2) and level 16 (ws + 2, q = 3) lose `GATE_M3_WALK`:
      assert(activeGatesOnRow("walk_lagrange1", cex_P6_walk_lagrange1, ws + 0) == Set(GATE_M3_FIRST)),
      assert(activeGatesOnRow("walk_lagrange1", cex_P6_walk_lagrange1, ws + 2) == Set(GATE_M3_LAST)),
      // Honest advice witness under P6 is accepted and refines, while forged walk advice on ws + 0 is accepted:
      assert(adviceAcceptedWith("walk_lagrange1", honestAdviceWitness("walk_lagrange1", DEFAULT_CFG))),
      assert(adviceRefinesWith("walk_lagrange1", honestAdviceWitness("walk_lagrange1", DEFAULT_CFG))),
      assert(adviceAcceptedWith("walk_lagrange1", forgedAdviceWitness("walk_lagrange1", DEFAULT_CFG, GATE_M3_WALK, ws + 0))),
      assert(not(adviceAcceptedWith("none", forgedAdviceWitness("none", DEFAULT_CFG, GATE_M3_WALK, ws + 0)))),
      assert(not(adviceRefinesWith("walk_lagrange1", forgedAdviceWitness("walk_lagrange1", DEFAULT_CFG, GATE_M3_WALK, ws + 0)))),
    }
  }

  // P7 `walk_code_shift`: walker enables `wk.q[code]` with shifted code mapping (`0 = first, 1 = middle/other, 2 = last`)
  run mutantP7_walkCodeShiftTest = {
    val ws = cex_P7_walk_code_shift.walkStart
    all {
      assert(acceptsWithMutant("walk_code_shift", cex_P7_walk_code_shift)),
      assert(not(acceptsWithMutant("none", cex_P7_walk_code_shift))),
      assert(not(refinesWith("walk_code_shift", cex_P7_walk_code_shift))),
      // Old walk level 0 (ws + 0) gets code 1 instead of 2, losing `GATE_M3_FIRST`:
      assert(activeGatesOnRow("walk_code_shift", cex_P7_walk_code_shift, ws + 0) == Set(GATE_M3_WALK)),
      // New walk level 0 (ws + 3) and middle levels get code 2, wrongly enabling `GATE_M3_FIRST`:
      assert(activeGatesOnRow("walk_code_shift", cex_P7_walk_code_shift, ws + 1) == Set(GATE_M3_WALK, GATE_M3_FIRST)),
      assert(activeGatesOnRow("walk_code_shift", cex_P7_walk_code_shift, ws + 3) == Set(GATE_M3_WALK, GATE_M3_FIRST)),
    }
  }

  // P8 `dec_packed`: DEC-row core gate and DEC lookup use a packed code column (`n = 3` on Narrow -> lookup degree 8 > 6)
  run mutantP8_decPackedTest = {
    val wideP8 = buildLayout("dec_packed", WIDE_DEFAULT_CFG)
    all {
      assert(acceptsWithMutant("dec_packed", cex_P8_dec_packed)),
      assert(not(acceptsWithMutant("none", cex_P8_dec_packed))),
      assert(not(refinesWith("dec_packed", cex_P8_dec_packed))),
      // On Narrow, dgi = 0 is packed into a 3-code column (selDeg = 3), so DEC lookup input degree is 5 and required degree is 8:
      assert(cex_P8_dec_packed.lookups[3].lookupId == LK_DEC),
      assert(cex_P8_dec_packed.lookups[3].sel.selDeg == 3),
      assert(cex_P8_dec_packed.lookups[3].inputDeg == 5),
      assert(cex_P8_dec_packed.lookups[3].reqDeg == 8),
      assert(maxLookupDegree(cex_P8_dec_packed) == 8),
      // On Wide (wideDeg = 3), dgi = 0 and rf count0 share a 2-code column: lookup degree is 7 AND row 0 overwrites dgi = 0!
      assert(wideP8.lookups[3].reqDeg == 7),
      assert(activeGatesOnRow("dec_packed", wideP8, 0) == Set(GATE_RF_CNT0)),
    }
  }

  // P9 `abs_two_codes`: chain absorb row (`hashStart + 4`) gets two `enable` calls (`q_chain` then `q_first`)
  run mutantP9_absTwoCodesTest = {
    val hs = cex_P9_abs_two_codes.hashStart
    all {
      assert(acceptsWithMutant("abs_two_codes", cex_P9_abs_two_codes)),
      assert(not(acceptsWithMutant("none", cex_P9_abs_two_codes))),
      assert(not(refinesWith("abs_two_codes", cex_P9_abs_two_codes))),
      // Last-write-wins leaves code 1 (`q_first`) on `hs + 4`, erasing `GATE_HASH_CHAIN` and enabling `GATE_HASH_FIRST`:
      assert(cex_P9_abs_two_codes.fixedGrid[COL_ABS][hs + 4] == 1),
      assert(activeGatesOnRow("abs_two_codes", cex_P9_abs_two_codes, hs + 4) == Set(GATE_HASH_FIRST)),
      assert(refGatesOnRow(cex_P9_abs_two_codes, hs + 4) == Set(GATE_HASH_CHAIN)),
    }
  }

  // ---------------------------------------------------------------- 3. Honest-Refines & Unaffected-Variant Tests
  // For mutants P1, P2, P4 (on `WIDE_DEFAULT_CFG`) and P8 (on `WIDE_DEG5_CFG`), the unaffected layout variants
  // are still accepted and refine (`invRefine` holds on those variants, proving the violation is non-vacuous).
  run unaffectedLayoutVariantsRefineTest = all {
    assert(
      Set("same_code", "cnt_collides", "pack_bound_7").forall(mt => {
        val lay = buildLayout(mt, WIDE_DEFAULT_CFG)
        acceptsWithMutant(mt, lay) and refinesWith(mt, lay)
      })
    ),
    assert(acceptsWithMutant("dec_packed", buildLayout("dec_packed", WIDE_DEG5_CFG))),
    assert(refinesWith("dec_packed", buildLayout("dec_packed", WIDE_DEG5_CFG))),
  }

  // For mutants that shift gates from one row to another on `DEFAULT_CFG` (P5, P7, P9), an advice witness adapted
  // to the mutant's active gates is accepted by the mutant's gates and fails `adviceRefinesWith`, while the honest
  // reference advice witness is rejected on the wrongly-enabled row. For over-constraining mutants (P1, P3) that
  // strictly add extra gates on non-reference rows, the honest reference advice witness is rejected.
  run adaptedAdviceUnderGateShiftingMutantsTest = all {
    assert(
      Set("chain_on_last", "walk_code_shift", "abs_two_codes").forall(mt => {
        val aw: AdviceWitness = {
          lay: buildLayout(mt, DEFAULT_CFG),
          tamperedGateId: -1,
          tamperedRow: -1,
          adaptedToMutant: true
        }
        val honestAw = honestAdviceWitness(mt, DEFAULT_CFG)
        adviceAcceptedWith(mt, aw) and not(adviceRefinesWith(mt, aw)) and not(adviceAcceptedWith(mt, honestAw))
      })
    ),
    assert(
      Set("same_code", "lagrange_skips_zero").forall(mt => {
        val honestAw = honestAdviceWitness(mt, DEFAULT_CFG)
        not(adviceAcceptedWith(mt, honestAw))
      })
    ),
  }

  // ---------------------------------------------------------------- 4. Exhaustive Pure Domain Enumeration
  run exhaustiveSoundAll93LayoutsTest =
    assert(exhaustiveLayoutsSound("none"))

  run exhaustiveAllMutantsCaughtTest = all {
    assert(exhaustiveMutantCaught("same_code")),
    assert(exhaustiveMutantCaught("cnt_collides")),
    assert(exhaustiveMutantCaught("lagrange_skips_zero")),
    assert(exhaustiveMutantCaught("pack_bound_7")),
    assert(exhaustiveMutantCaught("chain_on_last")),
    assert(exhaustiveMutantCaught("walk_lagrange1")),
    assert(exhaustiveMutantCaught("walk_code_shift")),
    assert(exhaustiveMutantCaught("dec_packed")),
    assert(exhaustiveMutantCaught("abs_two_codes")),
  }
}
