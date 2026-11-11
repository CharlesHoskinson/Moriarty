// Unit, Counterexample, and Exhaustive Free-Advice Tests — Running-Sum RANGE and ALU
// Run: quint test --main range_test qa/gemini-quint/chipleaf-final/range_test.qnt

module range_test {
  import rangechip(P = 97, MUTANT = "none").* from "./range"

  // ---------------------------------------------------------------- Finite Exhaustive Domains
  pure val EXH_TARGET_OPS: Set[int] = Set(0, 1)
  pure val EXH_OTHER_VALS: Set[int] = Set(0, 1)
  pure val EXH_PADS: Set[int] = Set(0, 1, 2, 4, P - 1)
  pure val EXH_L1S: Set[int] = Set(0, 1, 2, 3, 4, P - 1)
  pure val EXH_L2S: Set[int] = Set(0, 2, 4)
  pure val EXH_L3S: Set[int] = Set(0, 1, 3, 4)
  pure val EXH_ACC_MODES: Set[int] = Set(0, 1, 2, 3)
  pure val EXH_FREE_ACCS: Set[int] = Set(0, 1, 2, 32, P - 1)

  // Raw independent 8-cell sub-domains for Op 0 (no recurrence helper)
  pure val RAW_PADS: Set[int] = Set(0, 2, 4)
  pure val RAW_L1S: Set[int] = Set(0, 1, 2)
  pure val RAW_L2S: Set[int] = Set(0, 2)
  pure val RAW_L3S: Set[int] = Set(0, 1)
  pure val RAW_A0S: Set[int] = Set(0, 1, 32)
  pure val RAW_A1S: Set[int] = Set(0, 1, 2, 4)
  pure val RAW_A2S: Set[int] = Set(0, 6, 8, 16)
  pure val RAW_A3S: Set[int] = Set(0, 1, 24, 32, 64)

  // Q2c ALU finite check domains
  pure val ALU_OPS: Set[int] = Set(OP_ADD, OP_LT)
  pure val ALU_XY_DOM: Set[int] = Set(0, 1, 15, 16, 30, 31)
  pure val ALU_FLAGS: Set[int] = Set(0, 1)
  pure val ALU_Z_DOM: Set[int] = Set(0, 1, 15, 16, 30, 31, 32, 62, P - 1)

  // 1. Exhaustive check over free limb, pad, and accumulator choices across both ops
  pure def exhaustiveFreeDomainSound(mt: str): bool = {
    val attacksOk = ATTACK_IDS.fold(true, (okA, aid) => {
      val cand = applyProverAttack(mt, aid)
      okA and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
    })
    val limbsAndPadOk = EXH_TARGET_OPS.fold(true, (okOp, tOp) =>
      EXH_OTHER_VALS.fold(okOp, (okOv, oVal) =>
        EXH_PADS.fold(okOv, (okPad, pVal) =>
          EXH_L1S.fold(okPad, (okL1, l1) =>
            EXH_L2S.fold(okL1, (okL2, l2) =>
              EXH_L3S.fold(okL2, (okL3, l3) => {
                val cand = buildFreeOpWitness(mt, tOp, oVal, pVal, l1, l2, l3, 0, 0)
                okL3 and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
              })
            )
          )
        )
      )
    )
    val freeAccOk = EXH_PADS.fold(true, (okPad, pVal) =>
      Set(0, 1, 2, P - 1).fold(okPad, (okL1, l1) =>
        Set(0, 1).fold(okL1, (okL3, l3) =>
          EXH_ACC_MODES.fold(okL3, (okMode, aMode) =>
            EXH_FREE_ACCS.fold(okMode, (okAcc, fAcc) => {
              val cand = buildFreeOpWitness(mt, 0, 21, pVal, l1, 0, l3, aMode, fAcc)
              okAcc and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
            })
          )
        )
      )
    )
    attacksOk and limbsAndPadOk and freeAccOk
  }

  // 2. Exhaustive check over raw independent 8-cell choices of Op 0 (no recurrence helper)
  pure def exhaustiveRawIndependentCellsSound(mt: str): bool =
    RAW_PADS.fold(true, (okP, pVal) =>
      RAW_L1S.fold(okP, (ok1, l1) =>
        RAW_L2S.fold(ok1, (ok2, l2) =>
          RAW_L3S.fold(ok2, (ok3, l3) =>
            RAW_A0S.fold(ok3, (okA0, a0) =>
              RAW_A1S.fold(okA0, (okA1, a1) =>
                RAW_A2S.fold(okA1, (okA2, a2) =>
                  RAW_A3S.fold(okA2, (okA3, a3) => {
                    val cand = buildRawIndependentOp0Witness(mt, pVal, l1, l2, l3, a0, a1, a2, a3)
                    okA3 and (acceptsWithMutant(mt, cand) implies refinesWith(mt, cand))
                  })
                )
              )
            )
          )
        )
      )
    )

  pure def exhaustiveMutantCaught(mt: str): bool =
    not(exhaustiveFreeDomainSound(mt))

  // ---------------------------------------------------------------- 1. Honest Witness & Complete Range 0..31 Tests
  run honestWitnessAcceptedAndRefinesTest = all {
    assert(acceptsWithMutant("none", hw_honest)),
    assert(refinesWith("none", hw_honest)),
    assert(hw_honest.opW[0] == 0 and hw_honest.opW[1] == 21),
    assert(acceptsWithMutant("none", hw_honest_max)),
    assert(refinesWith("none", hw_honest_max)),
    assert(hw_honest_max.opW[0] == 19 and hw_honest_max.opW[1] == 31),
    assert(acceptsWithMutant("none", hw_honest_zero_both)),
    assert(refinesWith("none", hw_honest_zero_both)),
    // Every integer v in 0..31 (0..RANGE_BOUND - 1) is accepted and refines in running-sum mode:
    assert(
      0.to(RANGE_BOUND - 1).forall(v => {
        val wPair = buildHonestWitnessForPair("none", v, RANGE_BOUND - 1 - v)
        acceptsWithMutant("none", wPair) and refinesWith("none", wPair)
          and wPair.opW[0] == v and wPair.opW[1] == RANGE_BOUND - 1 - v
      })
    )
  }

  // ---------------------------------------------------------------- 2. Mutant Counterexample Tests (R1..R8)
  // R1 `no_first`: row 1 carries q_step instead of q_first; free acc_0 = 1 on row 2 shifts to w0 = 64 >= 32
  run mutantR1_noFirstTest = all {
    assert(acceptsWithMutant("no_first", cex_R1_no_first)),
    assert(not(acceptsWithMutant("none", cex_R1_no_first))),
    assert(cex_R1_no_first.opW[0] == 64),
    assert(cex_R1_no_first.opW[0] >= RANGE_BOUND),
    assert(not(refinesWith("no_first", cex_R1_no_first)))
  }

  // R2 `first_dropped`: row 1 carries no selector; free acc_1 = 2 on row 3 shifts to w0 = 32 >= 32
  run mutantR2_firstDroppedTest = all {
    assert(acceptsWithMutant("first_dropped", cex_R2_first_dropped)),
    assert(not(acceptsWithMutant("none", cex_R2_first_dropped))),
    assert(cex_R2_first_dropped.opW[0] == 32),
    assert(cex_R2_first_dropped.opW[0] >= RANGE_BOUND),
    assert(not(refinesWith("first_dropped", cex_R2_first_dropped)))
  }

  // R3 `pad_unchecked`: pad = 2 * limb_top dropped; limb_top = 2 (in table 0..3) gives w0 = 32 >= 32
  run mutantR3_padUncheckedTest = all {
    assert(acceptsWithMutant("pad_unchecked", cex_R3_pad_unchecked)),
    assert(not(acceptsWithMutant("none", cex_R3_pad_unchecked))),
    assert(cex_R3_pad_unchecked.opW[0] == 32),
    assert(cex_R3_pad_unchecked.opW[0] >= RANGE_BOUND),
    assert(not(refinesWith("pad_unchecked", cex_R3_pad_unchecked)))
  }

  // R4 `pad_row_no_lookup`: q_rng off on row 0; pad = 4 (outside table 0..3) with limb_top = 2 gives w0 = 32 >= 32
  run mutantR4_padRowNoLookupTest = all {
    assert(acceptsWithMutant("pad_row_no_lookup", cex_R4_pad_row_no_lookup)),
    assert(not(acceptsWithMutant("none", cex_R4_pad_row_no_lookup))),
    assert(cex_R4_pad_row_no_lookup.opW[0] == 32),
    assert(cex_R4_pad_row_no_lookup.opW[0] >= RANGE_BOUND),
    assert(not(refinesWith("pad_row_no_lookup", cex_R4_pad_row_no_lookup)))
  }

  // R5 `extra_step`: one extra q_step row (4 limbs); top limb 1 with pad = 2 gives w0 = 64 >= 32
  run mutantR5_extraStepTest = all {
    assert(acceptsWithMutant("extra_step", cex_R5_extra_step)),
    assert(cex_R5_extra_step.opW[0] == 64),
    assert(cex_R5_extra_step.opW[0] >= RANGE_BOUND),
    assert(not(refinesWith("extra_step", cex_R5_extra_step)))
  }

  // R6 `gate_off_centre`:
  //   - Reading row 0 (`gate_off_centre`): free acc_0 = 32 on row 2 gives out-of-range w0 = 32 >= 32
  //   - Reading row 10 / L-1 (`gate_row10`): limbs [0, 0, 1] have base-4 value 1, but gate on row L-1 sees acc_2 = 0 != 1
  run mutantR6_gateOffCentreTest = all {
    assert(acceptsWithMutant("gate_off_centre", cex_R6_gate_off_centre)),
    assert(not(acceptsWithMutant("none", cex_R6_gate_off_centre))),
    assert(cex_R6_gate_off_centre.opW[0] == 32),
    assert(cex_R6_gate_off_centre.opW[0] >= RANGE_BOUND),
    assert(not(refinesWith("gate_off_centre", cex_R6_gate_off_centre))),
    assert(acceptsWithMutant("gate_row10", cex_R6_row10)),
    assert(not(acceptsWithMutant("none", cex_R6_row10))),
    assert(cex_R6_row10.opW[0] == 0),
    assert(not(refinesWith("gate_row10", cex_R6_row10)))
  }

  // R7 `shared_sum`: Op 1's row 1 uses q_step and reads Op 0's last row (no pad row); Op 0 = 1 carries into Op 1 -> w1 = 64 >= 32
  run mutantR7_sharedSumTest = all {
    assert(acceptsWithMutant("shared_sum", cex_R7_shared_sum)),
    assert(cex_R7_shared_sum.opW[0] == 1),
    assert(cex_R7_shared_sum.opW[1] == 64),
    assert(cex_R7_shared_sum.opW[1] >= RANGE_BOUND),
    assert(not(refinesWith("shared_sum", cex_R7_shared_sum)))
  }

  // R8 `step_on_pad_row`: q_step also enabled on row 0 while row 1 keeps q_first.
  // Soundness is preserved because row 1's q_first enforces acc_1 = limb_1 without reading acc_0!
  // However, literal `assign_running` (which writes acc_0 = 0 on row 0) is rejected whenever BASE * prevAcc + pad != 0 (mod P).
  run mutantR8_stepOnPadRowHarmlessTest = all {
    val hwR8 = buildHonestWitness("step_on_pad_row")
    assert(acceptsWithMutant("step_on_pad_row", hwR8)),
    assert(refinesWith("step_on_pad_row", hwR8)),
    // Literal `assign_running` writes acc_0 = 0 on row 2, which fails row 2's q_step because prevAcc on row 1 is 7 != 0:
    assert(not(acceptsWithMutant("step_on_pad_row", hw_honest)))
  }

  // ---------------------------------------------------------------- 3. Honest Witness Under Every Mutant Tests
  run honestUnderMutantsRefinesTest = all {
    assert(
      Set(
        "no_first",
        "first_dropped",
        "pad_unchecked",
        "pad_row_no_lookup",
        "extra_step",
        "gate_off_centre",
        "shared_sum",
        "step_on_pad_row"
      ).forall(mt => {
        val hw = buildHonestWitness(mt)
        acceptsWithMutant(mt, hw) and refinesWith(mt, hw)
      })
    ),
    // Under `gate_row10` (reading row L-1), an honest witness with limb0 == 0 (e.g. (0, 0)) is accepted and refines,
    // whereas an honest witness with limb0 != 0 (e.g. (0, 21)) has acc_{L-1} = 5 != 21:
    val hwRow10Zero = buildHonestWitnessForPair("gate_row10", 0, 0)
    val hwRow10NonZero = buildHonestWitnessForPair("gate_row10", 0, 21)
    assert(acceptsWithMutant("gate_row10", hwRow10Zero) and refinesWith("gate_row10", hwRow10Zero)),
    assert(acceptsWithMutant("gate_row10", hwRow10NonZero) and not(refinesWith("gate_row10", hwRow10NonZero)))
  }

  // ---------------------------------------------------------------- 4. Small-Prime Modular Wrap-Around Tests (Q2a & Q2c)
  // (a) Running-sum wrap-around at P = 23 < RANGE_BOUND (32):
  //     Limbs [1, 2, 0] (pad = 2) have integer base-4 value 24, which wraps mod 23 to w0 = 1 != 24.
  // (b) Pad-multiplication wrap-around at P = 5 <= PAD_SHIFT * (BASE - 1) = 6:
  //     Top limb l1 = 3 >= TOP_BOUND (2) has 2 * 3 = 6 == 1 (mod 5), so pad = 1 (in TABLE 0..3) passes!
  run smallPrimeWrapAroundTest = {
    val wSumWrapP23 = buildFreeOpWitnessAtPrime(23, "none", 0, 21, 2, 1, 2, 0, 0, 0)
    val wPadWrapP5 = buildFreeOpWitnessAtPrime(5, "none", 0, 0, 1, 3, 0, 0, 0, 0)
    all {
      assert(acceptsAtPrime(23, "none", wSumWrapP23)),
      assert(wSumWrapP23.opW[0] == 1),
      assert(not(refinesWith("none", wSumWrapP23))),
      assert(acceptsAtPrime(5, "none", wPadWrapP5)),
      assert(wPadWrapP5.rows[3].limb == 3),
      assert(not(refinesWith("none", wPadWrapP5)))
    }
  }

  // ---------------------------------------------------------------- 5. Q2c: ALU Relation Soundness & Wrap Tests
  run aluRelationQ2cSoundAndWrapTest = {
    // 1. Exhaustive check at P = 97 >= 2 * RANGE_BOUND (64):
    //    For all op in {OP_ADD, OP_LT}, x, y in ALU_XY_DOM (< 32), flag in {0, 1}, z in ALU_Z_DOM,
    //    if aluGateOk(t) and 0 <= t.w < RANGE_BOUND hold, then t == aluRefEval(op, x, y).
    val aluSoundAtP97 =
      ALU_OPS.fold(true, (okOp, op) =>
        ALU_XY_DOM.fold(okOp, (okX, x) =>
          ALU_XY_DOM.fold(okX, (okY, y) =>
            ALU_FLAGS.fold(okY, (okF, fl) =>
              ALU_Z_DOM.fold(okF, (okZ, z) => {
                val is_add = 2 - op
                val is_lt = op - 1
                val wMod = modP(is_add * z + is_lt * (z * (y - x - 1) + (1 - z) * (x - y)))
                val cand: AluTupleW = {
                  chip: CHIP_ALU,
                  key: modP(op + FLAG_SHIFT * fl),
                  x: x,
                  y: y,
                  z: z,
                  op: op,
                  flag: fl,
                  w: wMod
                }
                val refT = aluRefEval(op, x, y)
                okZ and ((aluGateOk(cand) and cand.w < RANGE_BOUND) implies (cand == refT))
              })
            )
          )
        )
      )
    // 2. If RANGE allows w = RANGE_BOUND (32), ADD(16, 16) accepts z = 32, flag = 0 (forging carry = 0 instead of 1):
    val forgedAddCarry: AluTupleW = {
      chip: CHIP_ALU,
      key: OP_ADD,
      x: 16,
      y: 16,
      z: 32,
      op: OP_ADD,
      flag: 0,
      w: 32
    }
    // 3. At P = 47 (where RANGE_BOUND = 32 < P < 2 * RANGE_BOUND = 64), LT(20, 1) with forged z = 1 (claiming 20 < 1)
    //    has w = (1 - 20 - 1) mod 47 = 27 < 32, which passes both RANGE and the ALU gate by modular wrap-around!
    val forgedLtWrapP47: AluTupleW = {
      chip: CHIP_ALU,
      key: OP_LT,
      x: 20,
      y: 1,
      z: 1,
      op: OP_LT,
      flag: 0,
      w: 27
    }
    all {
      assert(aluSoundAtP97),
      assert(aluGateOk(forgedAddCarry)),
      assert(forgedAddCarry != aluRefEval(OP_ADD, 16, 16)),
      assert(aluGateOkAtPrime(47, forgedLtWrapP47)),
      assert(forgedLtWrapP47.w < RANGE_BOUND),
      assert(forgedLtWrapP47 != aluRefEval(OP_LT, 20, 1))
    }
  }

  // ---------------------------------------------------------------- 6. Exhaustive Pure Domain Checks
  run exhaustiveSoundTest = all {
    assert(exhaustiveFreeDomainSound("none")),
    assert(exhaustiveFreeDomainSound("step_on_pad_row"))
  }

  run exhaustiveRawIndependentSoundTest = all {
    assert(exhaustiveRawIndependentCellsSound("none")),
    assert(exhaustiveRawIndependentCellsSound("step_on_pad_row")),
    assert(not(exhaustiveRawIndependentCellsSound("no_first"))),
    assert(not(exhaustiveRawIndependentCellsSound("first_dropped"))),
    assert(not(exhaustiveRawIndependentCellsSound("pad_unchecked"))),
    assert(not(exhaustiveRawIndependentCellsSound("pad_row_no_lookup"))),
    assert(not(exhaustiveRawIndependentCellsSound("gate_off_centre"))),
    assert(not(exhaustiveRawIndependentCellsSound("gate_row10")))
  }

  run exhaustiveMutantsCaughtTest = all {
    assert(exhaustiveMutantCaught("no_first")),
    assert(exhaustiveMutantCaught("first_dropped")),
    assert(exhaustiveMutantCaught("pad_unchecked")),
    assert(exhaustiveMutantCaught("pad_row_no_lookup")),
    assert(exhaustiveMutantCaught("extra_step")),
    assert(exhaustiveMutantCaught("gate_off_centre")),
    assert(exhaustiveMutantCaught("gate_row10")),
    assert(exhaustiveMutantCaught("shared_sum"))
  }
}
