// Unit, adversarial-witness, reference-equivalence, and mutant test suite for `ivc06.qnt`
// (`crates/mori-recursion/src/ivc.rs` SHKIVC06 audit).
//
// Avoids Quint built-in names `fields`, `next`, and `fold`.

module ivc06_test {
  import ivc06.* from "./ivc06"

  // T1: Completeness on honest genesis (`steps = 2`, `steps = S_MAX = 16`) and honest step 1
  //     (`prevH = 2, steps = 3 -> h = 5`), matching `tests/ivc_node_mock.rs:230-256, 406-418`.
  run honestGenesisAndStep1CompletenessTest = {
    val wGen2 = mkHonestWitness(0, 2, 3, 12)
    val wGen16 = mkHonestWitness(0, S_MAX, 4, 19)
    val wStep1 = mkHonestWitness(2, 3, 2, 7)
    init
      .then(recordAttempt(wGen2, "genesis_steps_2", true))
      .expect(and {
        st.circuitOk,
        st.hostOk,
        st.deciderOk,
        st.refOk,
        st.foldMatchesNodeFold,
        st.heightBridgeOk,
        st.lastInst.h == 2,
        allInvariants,
      })
      .then(recordAttempt(wGen16, "genesis_steps_s_max", true))
      .expect(and {
        st.circuitOk,
        st.hostOk,
        st.deciderOk,
        st.refOk,
        st.foldMatchesNodeFold,
        st.heightBridgeOk,
        st.lastInst.h == S_MAX,
        allInvariants,
      })
      .then(recordAttempt(wStep1, "step1_honest", true))
      .expect(and {
        st.circuitOk,
        st.hostOk,
        st.deciderOk,
        st.refOk,
        st.foldMatchesNodeFold,
        st.heightBridgeOk,
        st.lastInst.h == 5,
        allInvariants,
      })
  }

  // T2: Lean `genesis_collapse_both` (`Aggregation.lean:1260-1278`) and `non_genesis_keeps_both`
  //     (`Aggregation.lean:1290-1310`) across varying uncollapsed `rawProofAcc` shapes.
  run leanGenesisCollapseBothAndNonGenesisKeepsBothTest =
    init.expect(
      Set(1, 2, 4).forall(nb =>
        Set(0, 7, 23).forall(pt =>
          Set(0, 5, 13).forall(ss =>
            Set(0, 9, 41).forall(df => {
              val rawProof: Acc = {
                numBases: nb,
                lhsBase: pt,
                lhsScalar: 1,
                rhsBase: fadd(pt, 3),
                rhsScalar: 1,
                selfScalar: ss,
                defect: df,
              }
              val carried: Acc = {
                numBases: 1,
                lhsBase: 4,
                lhsScalar: 1,
                rhsBase: 8,
                rhsScalar: 1,
                selfScalar: 2,
                defect: 0,
              }
              val leafAcc: Acc = {
                numBases: 2,
                lhsBase: 6,
                lhsScalar: 1,
                rhsBase: 11,
                rhsScalar: 1,
                selfScalar: 0,
                defect: 0,
              }
              and {
                // `collapse_scale_false` (`Aggregation.lean:1238`)
                collapseAcc(scaleByBit(0, rawProof)) == trivialAcc,
                collapseAcc(scaleByBit(0, carried)) == trivialAcc,
                // `genesis_collapse_both` (`Aggregation.lean:1260-1263`)
                leanNodeFold(0, rawProof, carried, leafAcc) ==
                  collapseAcc(accumulate3(trivialAcc, trivialAcc, leafAcc)),
                circuitFold(0, rawProof, trivialAcc, leafAcc) ==
                  leanNodeFold(0, rawProof, carried, leafAcc),
                // `non_genesis_keeps_both` (`Aggregation.lean:1290-1294`)
                leanNodeFold(1, rawProof, carried, leafAcc) ==
                  collapseAcc(accumulate3(collapseAcc(rawProof), collapseAcc(carried), leafAcc)),
                circuitFold(1, rawProof, carried, leafAcc) ==
                  leanNodeFold(1, rawProof, carried, leafAcc),
              }
            })
          )
        )
      )
    )

  // T3: Lean `src004_height_bridge` (`Aggregation.lean:1201-1206`) and `lin_refines_chain`
  //     (`Aggregation.lean:1454-1485`): every accepted step has `h = prevH + steps < HMAX`,
  //     `prevH < h`, `1 <= h - prevH <= S_MAX`, and `h > 0` (`LinCert.h_ne_zero`).
  run leanSrc004HeightBridgeAndLinRefinesChainTest =
    init.expect(
      Set(0, 1, 2, 15, HMAX - S_MAX - 1).forall(prevH =>
        Set(1, 2, 8, S_MAX).forall(steps => {
          val w = mkHonestWitness(prevH, steps, 2, 7)
          val inst = mkInstance(w)
          and {
            circuitSatisfied(inst, w),
            src004HeightBridgeHolds(w, inst),
            inst.h == prevH + steps,
            inst.h < HMAX,
            prevH < inst.h,
            inst.h > 0,
            // `lin_certified_prev_not_genesis` (`Aggregation.lean:1425-1427`):
            // output height `inst.h` is never 0, so a certified predecessor never triggers `is_genesis == 1`.
            isZeroGadget(inst.h) == 0,
          }
        })
      )
    )

  // T4: Adversarial Witness 1 — Height wrap (`tests/ivc_node_mock.rs:445-464`, `Aggregation.lean:1179, 1211`):
  //     (a) `prevH = P - 2, steps = 2` -> field `h = 0` (`step1 wrap h=0`),
  //     (b) `prevH = HMAX - 1, steps = 1` -> `h = HMAX` (`step1 h=2^64`),
  //     (c) `prevH = P, steps = 1` -> raw height `P == 0 mod P` at `truePrevH = P` (`CEX-1`).
  run adversarialHeightWrapRefusedTest = {
    val base = mkHonestWitness(2, 2, 2, 7)
    // (a) Field wrap to h = 0 (`prev_h = -2, steps = 2`, `tests/ivc_node_mock.rs:456-464`)
    val wWrapZero = {
      ...base,
      prevH: P - 2,
      truePrevH: P - 2,
      leafSteps: 2,
      trueSteps: 2,
    }
    // (b) Sum reaches HMAX (`prev_h = 2^64 - 1, steps = 1, h = 2^64`, `tests/ivc_node_mock.rs:445-454`)
    val wSumHmax = {
      ...base,
      prevH: HMAX - 1,
      truePrevH: HMAX - 1,
      leafSteps: 1,
      trueSteps: 1,
    }
    // (c) Raw height `prevH = P` reducing to `0 mod P` at `truePrevH = P` (`CEX-1`)
    val wRawModP = {
      ...mkHonestWitness(0, 1, 3, 12),
      prevH: P,
      truePrevH: P,
      prevChainValid: false,
      rawProofAcc: { ...base.rawProofAcc, defect: 44 },
    }
    init
      .then(recordAttempt(wWrapZero, "wrap_h_to_zero", false))
      .expect(and {
        st.lastInst.h == 0,
        not(st.circuitOk),
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
      .then(recordAttempt(wSumHmax, "sum_reaches_hmax", false))
      .expect(and {
        st.lastInst.h == HMAX,
        not(st.circuitOk),
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
      .then(recordAttempt(wRawModP, "raw_prev_h_equals_p", false))
      .expect(and {
        not(st.circuitOk),
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
  }

  // T5: Adversarial Witness 2 — Steps 0 and 17 (`S_MAX + 1`) (`tests/ivc_node_mock.rs:248-260`).
  //     Refused at both genesis (`prevH = 0`) and mid-chain (`prevH = 2`).
  run adversarialSteps0And17RefusedTest = {
    val wGen0 = { ...mkHonestWitness(0, 1, 3, 12), leafSteps: 0, trueSteps: 0 }
    val wGen17 = { ...mkHonestWitness(0, 1, 3, 12), leafSteps: S_MAX + 1, trueSteps: S_MAX + 1 }
    val wMid0 = { ...mkHonestWitness(2, 1, 2, 7), leafSteps: 0, trueSteps: 0 }
    val wMid17 = { ...mkHonestWitness(2, 1, 2, 7), leafSteps: S_MAX + 1, trueSteps: S_MAX + 1 }
    init
      .then(recordAttempt(wGen0, "genesis_steps_0", false))
      .expect(not(st.circuitOk) and not(st.deciderOk) and not(st.refOk) and allInvariants)
      .then(recordAttempt(wGen17, "genesis_steps_17", false))
      .expect(not(st.circuitOk) and not(st.deciderOk) and not(st.refOk) and allInvariants)
      .then(recordAttempt(wMid0, "step1_steps_0", false))
      .expect(not(st.circuitOk) and not(st.deciderOk) and not(st.refOk) and allInvariants)
      .then(recordAttempt(wMid17, "step1_steps_17", false))
      .expect(not(st.circuitOk) and not(st.deciderOk) and not(st.refOk) and allInvariants)
  }

  // T6: Adversarial Witness 3 — Non-neutral carried accumulator at genesis (`tests/ivc_node_mock.rs:262-290`).
  //     Tests both a well-formed collapsed carry with generator base + fixed scalar 5 and a
  //     defect-cancelling carry (`CEX-3`). Both are refused by `constrain_genesis_neutral` and `next_acc`.
  run adversarialNonNeutralGenesisCarryRefusedTest = {
    val base = mkHonestWitness(0, 2, 3, 12)
    val nonNeutralCarry: Acc = {
      numBases: 1,
      lhsBase: 1,
      lhsScalar: 1,
      rhsBase: 1,
      rhsScalar: 1,
      selfScalar: 5,
      defect: 33,
    }
    val wNonNeutral = {
      ...base,
      prevAcc: nonNeutralCarry,
    }
    val badLeaf: Acc = { ...base.rawLeafAcc, defect: 19 }
    val r = foldChallenge(trivialAcc, nonNeutralCarry, badLeaf)
    val cancellingCarry: Acc = {
      ...nonNeutralCarry,
      defect: fsub(0, fmul(r, badLeaf.defect)),
    }
    val wCancelling = {
      ...base,
      prevAcc: cancellingCarry,
      rawLeafAcc: badLeaf,
      leafTransitionValid: false,
    }
    init
      .then(recordAttempt(wNonNeutral, "genesis_non_neutral_carry", false))
      .expect(and {
        isCollapsedCarry(nonNeutralCarry),
        not(isNeutral(nonNeutralCarry)),
        not(st.circuitOk),
        not(st.hostOk),
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
      .then(recordAttempt(wCancelling, "genesis_cancelling_carry", false))
      .expect(and {
        not(st.circuitOk),
        not(st.hostOk),
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
  }

  // T7: Adversarial Witness 4 — Non-collapsed carried accumulator mid-chain (`tests/ivc_node_mock.rs:420-443`).
  //     Re-encodes `acc0` as `(B * HALF_MOD_P, scalar 2)`. Demonstrates that:
  //     1. `collapseAcc(reenc) == acc0` (same algebraic MSM value),
  //     2. `circuitFold(1, rawProofAcc, reenc, rawLeafAcc) != leanNodeFold(1, rawProofAcc, reenc, rawLeafAcc)`
  //        (without `constrain_collapsed_carry`, the circuit fold diverges from `nodeFold`!),
  //     3. `SHKIVC06` refuses `reenc` in-circuit (`not(st.circuitOk)`) and on the host (`not(st.hostOk)`).
  run adversarialNonCollapsedCarryMidChainRefusedTest = {
    val base = mkHonestWitness(2, 3, 2, 7)
    val reenc = reencodeCarryScalar2(base.prevAcc)
    val wNonCollapsed = {
      ...base,
      prevAcc: reenc,
    }
    init
      .then(recordAttempt(wNonCollapsed, "step1_non_collapsed_carry", false))
      .expect(and {
        // Same collapsed MSM value as `base.prevAcc`
        collapseAcc(reenc) == base.prevAcc,
        // Not in collapsed form (`lhsScalar == 2 != 1`, `rhsScalar == 2 != 1`)
        not(isCollapsedCarry(reenc)),
        // Without `constrain_collapsed_carry`, the raw circuit fold differs from Lean's `nodeFold`:
        circuitFold(1, base.rawProofAcc, reenc, base.rawLeafAcc) !=
          leanNodeFold(1, base.rawProofAcc, reenc, base.rawLeafAcc),
        // Refused by `constrain_collapsed_carry` in circuit and `is_collapsed_carry` on host:
        not(st.circuitOk),
        not(st.hostOk),
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
  }

  // T8: Adversarial Witness 5 — Invalid leaf (`tests/ivc_node_mock.rs:292-328, 358-395`).
  //     At both genesis (`prevH = 0`) and step 1 (`prevH = 2`), a syntactically prepared proof for
  //     an invalid leaf statement satisfies the node's deferred verifier (`circuitOk == true`),
  //     produces `inst.acc == leanNodeFold(...)` with `inst.acc.defect != 0`, and is refused by
  //     the final pairing check / decider (`deciderOk == false`).
  run adversarialInvalidLeafRefusedByPairingTest = {
    val baseGen = mkHonestWitness(0, 2, 3, 12)
    val wBadLeafGen = {
      ...baseGen,
      rawLeafAcc: { ...baseGen.rawLeafAcc, defect: 19 },
      leafTransitionValid: false,
    }
    val baseStep1 = mkHonestWitness(2, 3, 2, 7)
    val wBadLeafStep1 = {
      ...baseStep1,
      rawLeafAcc: { ...baseStep1.rawLeafAcc, defect: 23 },
      leafTransitionValid: false,
    }
    init
      .then(recordAttempt(wBadLeafGen, "genesis_invalid_leaf_deferred", false))
      .expect(and {
        st.circuitOk,
        st.hostOk,
        st.foldMatchesNodeFold,
        st.lastInst.acc.defect != 0,
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
      .then(recordAttempt(wBadLeafStep1, "step1_invalid_leaf_deferred", false))
      .expect(and {
        st.circuitOk,
        st.hostOk,
        st.foldMatchesNodeFold,
        st.lastInst.acc.defect != 0,
        not(st.deciderOk),
        not(st.refOk),
        allInvariants,
      })
  }

  // T9: Exhaustive check of whether `constrain_genesis_neutral` + `constrain_collapsed_carry`
  //     makes `circuitFold` equal Lean's `AccAlgebra.nodeFold`:
  //     For every 1-base-per-side accumulator `c`, `constrainGenesisNeutral(isGen, c)` and
  //     `constrainCollapsedCarry(c)` hold iff `c == collapseAcc(scaleByBit(1 - isGen, c))`,
  //     which makes `circuitFold(1 - isGen, p, c, l) == leanNodeFold(1 - isGen, p, c, l)`.
  run collapsedCarryMakesCircuitEqualNodeFoldTest = {
    val p: Acc = { numBases: 3, lhsBase: 7, lhsScalar: 1, rhsBase: 9, rhsScalar: 1, selfScalar: 4, defect: 0 }
    val l: Acc = { numBases: 2, lhsBase: 6, lhsScalar: 1, rhsBase: 11, rhsScalar: 1, selfScalar: 0, defect: 0 }
    init.expect(
      Set(0, 1).forall(isGen =>
        Set(0, 4).forall(lb =>
          Set(1, 2).forall(ls =>
            Set(0, 8).forall(rb =>
              Set(1, 2).forall(rs =>
                Set(0, 5).forall(ss => {
                  val c: Acc = {
                    numBases: 1,
                    lhsBase: lb,
                    lhsScalar: ls,
                    rhsBase: rb,
                    rhsScalar: rs,
                    selfScalar: ss,
                    defect: 0,
                  }
                  val notGen = 1 - isGen
                  val acceptedByCarryConstraints =
                    constrainGenesisNeutral(isGen, c) and constrainCollapsedCarry(c)
                  acceptedByCarryConstraints implies and {
                    c == collapseAcc(scaleByBit(notGen, c)),
                    circuitFold(notGen, p, c, l) == leanNodeFold(notGen, p, c, l),
                  }
                })
              )
            )
          )
        )
      )
    )
  }

  // T10: Seeded circuit mutants — verifies that removing any of the five SHKIVC06 constraints
  //      (`prev_h < 2^64`, `steps - 1 < S_MAX`, `h < 2^64`, `constrain_genesis_neutral`,
  //      `constrain_collapsed_carry`) admits an adversarial witness that SHKIVC06 refuses.
  run seededCircuitMutantsCaughtTest = {
    val baseGen = mkHonestWitness(0, 2, 3, 12)
    val baseStep1 = mkHonestWitness(2, 3, 2, 7)

    // Mutant M1: omit `prev_h < HMAX` -> accepts `prevH = P - 2, steps = 2` (`h = 0`)
    val wWrap = { ...baseStep1, prevH: P - 2, truePrevH: P - 2, leafSteps: 2, trueSteps: 2 }
    val m1Accepts = and {
      assertLowerThanFixed(fsub(wWrap.leafSteps, 1), S_MAX),
      assertLowerThanFixed(fadd(fmod(wWrap.prevH), fmod(wWrap.leafSteps)), HMAX),
    }

    // Mutant M2: omit `steps - 1 < S_MAX` (use only `steps != 0`) -> accepts `steps = 17`
    val wSteps17 = { ...baseGen, leafSteps: S_MAX + 1, trueSteps: S_MAX + 1 }
    val m2Accepts = and {
      assertLowerThanFixed(wSteps17.prevH, HMAX),
      fmod(wSteps17.leafSteps) != 0,
      assertLowerThanFixed(fadd(wSteps17.prevH, wSteps17.leafSteps), HMAX),
    }

    // Mutant M3: omit `h < HMAX` (`weaker_pair_insufficient`, `Aggregation.lean:1179`) -> accepts `h = HMAX`
    val wTop = { ...baseStep1, prevH: HMAX - 1, truePrevH: HMAX - 1, leafSteps: 1, trueSteps: 1 }
    val m3Accepts = and {
      assertLowerThanFixed(wTop.prevH, HMAX),
      assertLowerThanFixed(fsub(wTop.leafSteps, 1), S_MAX),
    }

    // Mutant M4: omit `constrain_genesis_neutral` -> accepts non-neutral genesis carry
    val nonNeutral: Acc = { numBases: 1, lhsBase: 1, lhsScalar: 1, rhsBase: 1, rhsScalar: 1, selfScalar: 5, defect: 0 }
    val wGenCarry = { ...baseGen, prevAcc: nonNeutral }
    val m4Accepts = and {
      constrainHeights(wGenCarry.prevH, wGenCarry.leafSteps).ok,
      constrainCollapsedCarry(wGenCarry.prevAcc),
    }

    // Mutant M5: omit `constrain_collapsed_carry` -> accepts mid-chain `(B/2, scalar 2)` carry
    val wReenc = { ...baseStep1, prevAcc: reencodeCarryScalar2(baseStep1.prevAcc) }
    val m5Accepts = and {
      constrainHeights(wReenc.prevH, wReenc.leafSteps).ok,
      constrainGenesisNeutral(0, wReenc.prevAcc),
    }

    init.expect(and {
      m1Accepts and not(circuitSatisfied(mkInstance(wWrap), wWrap)),
      m2Accepts and not(circuitSatisfied(mkInstance(wSteps17), wSteps17)),
      m3Accepts and not(circuitSatisfied(mkInstance(wTop), wTop)),
      m4Accepts and not(circuitSatisfied(mkInstance(wGenCarry), wGenCarry)),
      m5Accepts and not(circuitSatisfied(mkInstance(wReenc), wReenc)),
    })
  }
}
