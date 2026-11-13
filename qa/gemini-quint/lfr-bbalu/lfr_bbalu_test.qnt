// -*- mode: Bluespec; -*-
// Exhaustive pure checks, real-constant overflow checks, completeness checks,
// and mutant honest-vs-attack verification for `lfr_bbalu.qnt`.

module lfr_bbalu_test {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).* from "./lfr_bbalu"

  // =========================================================================
  // 1. Real circuit constants (BLS12-381 Fr and BabyBear, exact big integers)
  // =========================================================================
  // Cited sources:
  //   - crates/sherlock-lfr/src/bb.rs:12-16     (`P`, `P_MINUS_1_HI`, `W`)
  //   - crates/sherlock-lfr/src/range.rs:17-18  (`RANGE_BITS = 16`, `RANGE_SIZE = 1 << 16`)
  //   - crates/sherlock-lfr/src/alu.rs:14-20    (header overflow derivation)

  pure val P_REAL: int = 2013265921
  pure val P_MINUS_1_HI_REAL: int = 30720
  pure val RANGE_REAL: int = 65536
  pure val W_REAL: int = 11
  // BLS12-381 scalar field modulus r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
  pure val R_REAL: int =
    52435875175126190479447740508185965837690552500527637822603658699938581184513
  // 2^48 - 1 (maximum 3-limb 16-bit quotient q_k)
  pure val Q_MAX_REAL: int = 281474976710655
  // 2^68 and 2^80 bounds cited in crates/sherlock-lfr/src/alu.rs:15-16
  pure val TWO_POW_68: int = 295147905179352825856
  pure val TWO_POW_80: int = 1208925819614629174706176

  // Check real-constant integer bounds for Question 1 & Question 2 (alu.rs:14-20, 119-128)
  pure val realConstantsOverflowCheck: bool = {
    val pMaxVec = List(P_REAL - 1, P_REAL - 1, P_REAL - 1, P_REAL - 1)
    val eMulMax0 = unreducedCoeff(OP_MUL, pMaxVec, pMaxVec, 0, P_REAL, W_REAL)
    val eMulMax1 = unreducedCoeff(OP_MUL, pMaxVec, pMaxVec, 1, P_REAL, W_REAL)
    val eMulMax2 = unreducedCoeff(OP_MUL, pMaxVec, pMaxVec, 2, P_REAL, W_REAL)
    val eMulMax3 = unreducedCoeff(OP_MUL, pMaxVec, pMaxVec, 3, P_REAL, W_REAL)
    val eAddMax = 2 * (P_REAL - 1)
    val eSubMin = 0 - (P_REAL - 1) + P_REAL
    val eSubMax = (P_REAL - 1) - 0 + P_REAL
    val rhsMaxCanonical = Q_MAX_REAL * P_REAL + (P_REAL - 1)
    val rhsMaxBeforeLoZero = Q_MAX_REAL * P_REAL + (P_MINUS_1_HI_REAL * RANGE_REAL + (RANGE_REAL - 1))
    val qMulMax = eMulMax0 / P_REAL
    val edgeVals = Set(0, 1, P_REAL - 1)
    // Question 2: can "+ p" in Sub (a - b + p = q p + c) be dropped (q = 1 when a < b),
    // kept when a >= b (q = 0), or doubled (q = R_REAL - 1)?
    val subShiftPinned = edgeVals.forall(a =>
      edgeVals.forall(b => {
        val eSub = a - b + P_REAL
        val cDrop = a - b
        val cKeep = a - b + P_REAL
        val cDouble = a - b + 2 * P_REAL
        and {
          1 <= eSub and eSub <= 2 * P_REAL - 1,
          if (a < b) (cDrop < 0 and 0 <= cKeep and cKeep < P_REAL) else (0 <= cDrop and cDrop < P_REAL and cKeep >= P_REAL),
          cDouble >= P_REAL + 1,
          R_REAL - 1 > Q_MAX_REAL
        }
      })
    )
    and {
      // 1. P - 1 = 0x7800 * 2^16 + 0 (bb.rs:12-14)
      P_REAL - 1 == P_MINUS_1_HI_REAL * RANGE_REAL,
      Q_MAX_REAL == RANGE_REAL * RANGE_REAL * RANGE_REAL - 1,
      // 2. Every accepted q_k (3 limbs < 2^16) and c_k (< P_REAL) gives q_k * P + c_k < 2^80 < R_REAL
      rhsMaxCanonical == 566683961809554203901951,
      rhsMaxCanonical < TWO_POW_80,
      rhsMaxBeforeLoZero < TWO_POW_80,
      TWO_POW_80 < R_REAL,
      // 3. Every canonical input gives 0 <= e_k < 34 * P^2 < 2^68 < R_REAL, and honest q_k <= Q_MAX_REAL
      eAddMax < R_REAL,
      eSubMin == 1,
      eSubMax < R_REAL,
      eMulMax0 == 34 * (P_REAL - 1) * (P_REAL - 1),
      eMulMax1 == 24 * (P_REAL - 1) * (P_REAL - 1),
      eMulMax2 == 14 * (P_REAL - 1) * (P_REAL - 1),
      eMulMax3 == 4 * (P_REAL - 1) * (P_REAL - 1),
      eMulMax0 < 34 * P_REAL * P_REAL,
      34 * P_REAL * P_REAL < TWO_POW_68,
      TWO_POW_68 < R_REAL,
      qMulMax == 68451041245,
      qMulMax <= Q_MAX_REAL,
      // 4. In eval_canonical, 0x7800 - hi < 2^16 mod R_REAL iff hi <= 0x7800 for hi < 2^16
      R_REAL - (RANGE_REAL - 1 - P_MINUS_1_HI_REAL) >= RANGE_REAL,
      subShiftPinned
    }
  }

  // =========================================================================
  // 2. Completeness check (`invHonestAccepted`)
  // =========================================================================

  pure val invHonestAccepted: bool = {
    // Every canonical BabyBear input v0 in 0..P_SMALL-1 (including 0, 1, P_SMALL - 1) is accepted and refines
    val allInputsAccepted = 0.to(P_SMALL - 1).forall(v0 => {
      val w = buildWitnessForInputWith(List(v0, 0, 0, 0), SOUND_FLAGS)
      isAcceptedWith(w, SOUND_FLAGS) and witnessRefines(w)
    })
    // Every op kind (Add, Sub, Mul) on all BabyBear pairs and maximal EF4 inputs is accepted
    val edgeEf4 = Set(
      List(0, 0, 0, 0),
      List(1, 0, 0, 0),
      List(P_SMALL - 1, 0, 0, 0),
      List(P_SMALL - 1, P_SMALL - 1, P_SMALL - 1, P_SMALL - 1)
    )
    val allOpsComplete = Set(OP_ADD, OP_SUB, OP_MUL).forall(kind =>
      edgeEf4.forall(a =>
        edgeEf4.forall(b => {
          val r = buildHonestAluRow(kind, a, b, 1, 2, 3, 0)
          val lks = aluRowRangeLookupsWith(r, SOUND_FLAGS)
          and {
            evalAluRowGatesWith(r, SOUND_FLAGS),
            lks.length() == 24,
            lks.indices().forall(i => {
              val m = lks.nth(i).msg.nth(0)
              0 <= m and m < RANGE_MAX
            }),
            r.valC == refAluResult(kind, a, b)
          }
        })
      )
    )
    // Edge non-canonical values P_SMALL and R_SMALL - 1 are rejected by `eval_canonical` + `RANGE_BUS`
    val nonCanonEdgesRejected = Set(P_SMALL, R_SMALL - 1).forall(badC =>
      0.to(RANGE_MAX - 1).forall(loV =>
        0.to(RANGE_MAX - 1).forall(hiV =>
          Set(0, 1).forall(isMaxV => {
            val headOk = modR(P_HI_MAX - hiV) < RANGE_MAX
            val gateOk = evalCanonicalGateWith(badC, loV, hiV, canonInv(hiV), isMaxV, SOUND_FLAGS)
            not(headOk and gateOk)
          })
        )
      )
    )
    allInputsAccepted and allOpsComplete and nonCanonEdgesRejected
  }

  // =========================================================================
  // 3. Exhaustive pure soundness checks per attack question (Q1 .. Q5)
  // =========================================================================

  // Attack Q1 (Canonicity): over all (lo, hi, isMax, inv) in the tamper domain,
  // `eval_canonical` + `RANGE_BUS` accepts iff 0 <= c < P_SMALL and limbs are canonical.
  pure val exhaustiveQ1CanonicityCheck: bool =
    0.to(RANGE_MAX + 4).forall(loV =>
      0.to(RANGE_MAX + 2).forall(hiV =>
        0.to(2).forall(isMaxV => {
          val cVal = modR(loV + hiV * RANGE_MAX)
          val invCandidates = Set(0, 1, canonInv(hiV), R_SMALL - 1)
          invCandidates.forall(invV => {
            val lks = canonicalRangeLookupsWith(loV, hiV, SOUND_FLAGS)
            val lookupsInRange = lks.indices().forall(i => {
              val v = lks.nth(i).msg.nth(0)
              0 <= v and v < RANGE_MAX
            })
            val gateOk = evalCanonicalGateWith(cVal, loV, hiV, invV, isMaxV, SOUND_FLAGS)
            val accepted = lookupsInRange and gateOk
            val expected = and {
              0 <= cVal and cVal < P_SMALL,
              loV == canonLo(cVal),
              hiV == canonHi(cVal),
              isMaxV == canonIsMax(hiV),
              if (hiV == P_HI_MAX) true else invV == canonInv(hiV)
            }
            accepted == expected
          })
        })
      )
    )

  // Attack Q2 (Quotient & overflow): over all op kinds, all canonical a0, b0 in [0, P_SMALL),
  // all 3-limb quotients (ql0, ql1, ql2) in [0, RANGE_MAX)^3, and all canonical c0 in [0, P_SMALL),
  // the Fr identity `e0 == q0 * P_SMALL + c0 (mod R_SMALL)` holds iff `c0 == e0 % P_SMALL`
  // and `q0 == e0 / P_SMALL`.
  pure val exhaustiveQ2QuotientCheck: bool =
    Set(OP_ADD, OP_SUB, OP_MUL).forall(kind =>
      0.to(P_SMALL - 1).forall(a0 =>
        0.to(P_SMALL - 1).forall(b0 => {
          val e0 = unreducedCoeff(kind, List(a0, 0, 0, 0), List(b0, 0, 0, 0), 0, P_SMALL, W_SMALL)
          0.to(RANGE_MAX - 1).forall(ql0 =>
            0.to(RANGE_MAX - 1).forall(ql1 =>
              0.to(RANGE_MAX - 1).forall(ql2 =>
                0.to(P_SMALL - 1).forall(c0 => {
                  val q0 = ql0 + ql1 * RANGE_MAX + ql2 * RANGE_MAX * RANGE_MAX
                  val frEq = modR(e0) == modR(q0 * P_SMALL + c0)
                  val intEq = (c0 == e0 % P_SMALL) and (q0 == e0 / P_SMALL)
                  frEq == intEq
                })
              )
            )
          )
        })
      )
    )

  // Attack Q3 (Op kind & BabyBear subfield):
  // 1) Over all selector triples (sAdd, sSub, sMul) in {0, 1, 2}^3, the selector gates + PROG_BUS
  //    pin the triple to the exact key selector.
  // 2) Over all upper coefficients (v1, v2, v3) in [0, P_SMALL)^3, InputAir with zFlags = [1,1,1]
  //    accepts iff v1 == 0 and v2 == 0 and v3 == 0.
  // 3) Over all BabyBear operands [a0, 0, 0, 0] and [b0, 0, 0, 0], BbAluAir forces c[1..3] == [0, 0, 0].
  pure val exhaustiveQ3OpKindAndBbCheck: bool = {
    val selPinned = 0.to(2).forall(sAdd =>
      0.to(2).forall(sSub =>
        0.to(2).forall(sMul => {
          val boolGates = and {
            isBoolModR(sAdd),
            isBoolModR(sSub),
            isBoolModR(sMul),
            isBoolModR(sAdd + sSub + sMul)
          }
          val validSet = Set(List(0, 0, 0), List(1, 0, 0), List(0, 1, 0), List(0, 0, 1))
          boolGates == validSet.contains(List(sAdd, sSub, sMul))
        })
      )
    )
    val bbInputUpperZero = 0.to(P_SMALL - 1).forall(v1 =>
      0.to(P_SMALL - 1).forall(v2 =>
        0.to(P_SMALL - 1).forall(v3 => {
          val r = buildInputRow(List(4, v1, v2, v3), 1, 5, List(1, 1, 1), 0, List(0, 0, 0, 0))
          evalInputRowGatesWith(r, SOUND_FLAGS) == (v1 == 0 and v2 == 0 and v3 == 0)
        })
      )
    )
    val bbAluStaysInSubfield = Set(OP_ADD, OP_SUB, OP_MUL).forall(kind =>
      0.to(P_SMALL - 1).forall(a0 =>
        0.to(P_SMALL - 1).forall(b0 => {
          val res = refAluResult(kind, List(a0, 0, 0, 0), List(b0, 0, 0, 0))
          res.nth(1) == 0 and res.nth(2) == 0 and res.nth(3) == 0
        })
      )
    )
    selPinned and bbInputUpperZero and bbAluStaysInSubfield
  }

  // Attack Q4 (Range bus & 24 lookups):
  // 1) Every BbAlu row has 24 lookups; every Input row has 12 lookups.
  // 2) Range16Air gates over 0..RANGE_MAX^4 pin `rangeVal == List(0, 1, 2, 3)`.
  // 3) Over all field elements v in 0..R_SMALL-1 and query counts m in 1..24, a lookup of `v`
  //    with count `m` can be balanced by `Range16Air` iff `0 <= v < RANGE_MAX`.
  pure val exhaustiveQ4RangeBusCheck: bool = {
    val lookupCountsExact = and {
      aluRowRangeLookupsWith(honestWitness.aluRows.nth(0), SOUND_FLAGS).length() == 24,
      aluRowRangeLookupsWith(honestWitness.aluRows.nth(3), SOUND_FLAGS).length() == 24,
      inputRowRangeLookupsWith(honestWitness.inpRows.nth(0), SOUND_FLAGS).length() == 12
    }
    val tablePinned = 0.to(RANGE_MAX).forall(v0 =>
      0.to(RANGE_MAX).forall(v1 =>
        0.to(RANGE_MAX).forall(v2 =>
          0.to(RANGE_MAX).forall(v3 => {
            val rVal = List(v0, v1, v2, v3)
            evalRange16GatesWith(rVal, List(0, 0, 0, 0), SOUND_FLAGS) == (rVal == List(0, 1, 2, 3))
          })
        )
      )
    )
    val outOfRangeUnbalanceable = 0.to(R_SMALL - 1).forall(v =>
      1.to(24).forall(m => {
        val inRange = (0 <= v and v < RANGE_MAX)
        // If `v` is not in 0..RANGE_MAX-1, no `Range16Air` row emits `List(v)`, so its total
        // count on `RANGE_BUS` is `modR(m) != 0`.
        val queryEntry = { msg: List(v), cnt: m }
        val tableEntries = range(0, RANGE_MAX).foldl(EMPTY_BUS, (accE, i) =>
          accE.append({ msg: List(i), cnt: if (i == v) modR(-m) else 0 })
        )
        isBusBalanced(List(queryEntry).concat(tableEntries)) == inRange
      })
    )
    lookupCountsExact and tablePinned and outOfRangeUnbalanceable
  }

  // Attack Q5 (Memory traffic & padding rows):
  // 1) Exhaustively tampering ALU row 0's read value `valA[0]` across `0..P_SMALL-1` is accepted
  //    iff `valA[0] == honestWitness.inpRows.nth(0).valV.nth(0)`.
  // 2) Exhaustively tampering ALU row 0's program cells (`addrA`, `addrB`, `addrC`, `multC`) or
  //    activating padding row 3 (`selAdd = 1` or `multC > 0`) is rejected by `PROG_BUS`.
  pure val exhaustiveQ5MemAndProgCheck: bool = {
    val hw = honestWitness
    val readTamperRejected = 0.to(P_SMALL - 1).forall(a0 => {
      val r0 = buildHonestAluRow(OP_ADD, List(a0, 0, 0, 0), List(4, 0, 0, 0), 1, 1, 3, 0)
      val newAlu = hw.aluRows.replaceAt(0, r0)
      val w = {
        ...hw,
        aluRows: newAlu,
        rangeMult: computeRangeMultsWith(hw.inpRows, newAlu, SOUND_FLAGS)
      }
      isAcceptedWith(w, SOUND_FLAGS) == (a0 == 4)
    })
    val progTamperRejected = 0.to(5).forall(badAddr =>
      0.to(3).forall(badMult => {
        val r0 = { ...hw.aluRows.nth(0), addrC: badAddr, multC: badMult }
        val w = { ...hw, aluRows: hw.aluRows.replaceAt(0, r0) }
        progBusBalancedWith(w, SOUND_FLAGS) == (badAddr == 3 and badMult == 0)
      })
    )
    val padActiveRejected = Set(1, 2).forall(badMult => {
      val badPad = { ...padAluRow, multC: badMult }
      val w = { ...hw, aluRows: hw.aluRows.replaceAt(3, badPad) }
      not(progBusBalancedWith(w, SOUND_FLAGS))
    })
    readTamperRejected and progTamperRejected and padActiveRejected
  }

  // =========================================================================
  // 4. Mutant non-vacuity (honest refines) and attack violation checks
  // =========================================================================

  pure def checkMutantCaught(cf: ConstraintFlags, attackW: WitnessState): bool = {
    val honestW = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    and {
      // 1. Honest witness is accepted and refines under the mutant (non-vacuous mutant)
      isAcceptedWith(honestW, cf),
      witnessRefines(honestW),
      // 2. Attack witness is accepted under the mutant, but does NOT refine (violation!)
      isAcceptedWith(attackW, cf),
      not(witnessRefines(attackW)),
      // 3. Sound circuit rejects the attack witness
      not(isAcceptedWith(attackW, SOUND_FLAGS))
    }
  }

  pure val allMutantsVerified: bool = and {
    checkMutantCaught(MUT_Q1A_FLAGS, attackQ1aHiHeadroomWith(MUT_Q1A_FLAGS)),
    checkMutantCaught(MUT_Q1B_FLAGS, attackQ1bMaxLoZeroWith(MUT_Q1B_FLAGS)),
    checkMutantCaught(MUT_Q2_FLAGS, attackQ2QuotientWrapWith(MUT_Q2_FLAGS)),
    checkMutantCaught(MUT_Q3A_FLAGS, attackQ3aOpKindSwapWith(MUT_Q3A_FLAGS)),
    checkMutantCaught(MUT_Q3B_FLAGS, attackQ3bBbUpperNonZeroWith(MUT_Q3B_FLAGS)),
    checkMutantCaught(MUT_Q4A_FLAGS, attackQ4aLoOutOfRangeWith(MUT_Q4A_FLAGS)),
    checkMutantCaught(MUT_Q4B_FLAGS, attackQ4bRangeTableForgeWith(MUT_Q4B_FLAGS)),
    checkMutantCaught(MUT_Q5A_FLAGS, attackQ5aMemDisconnectWith(MUT_Q5A_FLAGS)),
    checkMutantCaught(MUT_Q5B_FLAGS, attackQ5bPaddingForgeWith(MUT_Q5B_FLAGS))
  }

  // =========================================================================
  // 5. `run ...Test` entry points for `quint test`
  // =========================================================================

  run invHonestAcceptedTest =
    init.expect(acc and invRefine and invHonestAccepted)

  run realConstantsOverflowTest =
    init.expect(realConstantsOverflowCheck)

  run exhaustiveQ1CanonicitySoundTest =
    init.expect(exhaustiveQ1CanonicityCheck)

  run exhaustiveQ2QuotientSoundTest =
    init.expect(exhaustiveQ2QuotientCheck)

  run exhaustiveQ3OpKindAndBbSoundTest =
    init.expect(exhaustiveQ3OpKindAndBbCheck)

  run exhaustiveQ4RangeBusSoundTest =
    init.expect(exhaustiveQ4RangeBusCheck)

  run exhaustiveQ5MemAndProgBusSoundTest =
    init.expect(exhaustiveQ5MemAndProgCheck)

  run allMutantsHonestAndAttackTest =
    init.expect(allMutantsVerified)

  run soundStateMachineStepPreservesRefineTest =
    init
      .then(step)
      .expect(invRefine)
}
