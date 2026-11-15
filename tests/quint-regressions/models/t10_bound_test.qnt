// -*- mode: quint; -*-
// Deterministic witness tests, mutant non-vacuity + exploit checks, and exhaustive
// finite-domain soundness proofs for QA lane t10-bound.

module t10_bound_test {
  import t10_bound_defs.* from "./t10_bound"
  // LEAD REPAIR: instance imports are not re-exported, so the test imports the sound instance directly.
  import t10_bound_base(
    C_OPS_SKIP = -1, C_OPEN_FREE = false, C_ANCHOR_NOT_CP = false, C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false, C_HHI_VALID_FREE = false, C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false, C_TIE_DROPPED = false, C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false, C_LEDGER_NO_REPLAY = false, C_LEDGER_TWO_MAPS = false
  ).* from "./t10_bound"

  // ===========================================================================
  // 1. Pure Exhaustive Finite-Domain Soundness Predicates
  // ===========================================================================

  // Q1a: Exhaustive check over BabyBear (BB_P = 17) that gen_r_check's is_zero gadget
  // (lverify.rs:1659-1673, 1745-1750) forces openSel in {0, 1} and openSel == [param4 == SHK_SET].
  pure val exhaustiveIsZeroGadgetOk: bool =
    0.to(BB_P - 1).forall(p4 =>
      0.to(BB_P - 1).forall(invW =>
        0.to(BB_P - 1).forall(openWord => {
          val diff4 = modPos(p4 - TAG_SHK_SET, BB_P)
          val eq1 = (modPos(diff4 * invW, BB_P) == modPos(1 - openWord, BB_P))
          val eq2 = (modPos(diff4 * openWord, BB_P) == 0)
          (and { eq1, eq2 }) implies and {
            isBit(openWord),
            (openWord == 1) == (p4 == TAG_SHK_SET),
          }
        })
      )
    )

  // Q1b: Exhaustive check over (progId, entryKind, modeBit, param4Word, openSel, a4, a5)
  pure val exhaustiveQ1AnchorAndSelectorsOk: bool = {
    val basePrev = honestInitForCfg(CFG_SOUND)
    val withRoot = mkHonestAcceptCollection(CFG_SOUND, basePrev)
    Set(PROG_A, PROG_B).forall(progId =>
      Set(ENTRY_ACCEPT_RUN, ENTRY_OPEN_CLAIM).forall(ek =>
        0.to(2).forall(mBit =>
          Set(TAG_SHK_PROGC, TAG_SHK_LOCK, TAG_SHK_SET).forall(p4 =>
            Set(0, 1, 2, 10).forall(oSel =>
              Set(sealOp0(PROG_A), sealOp0(PROG_B), SET_ROOT_PRE).forall(a4v =>
                Set(0, sealOp0(PROG_A), 12, sealOp0(PROG_B)).forall(a5v => {
                  val raw0 = withPreLedgerFrom(withRoot, mkHonestRunBase(progId, IN_1, OUT_OK, PROG_A))
                  val cand = finalizeTransition(CFG_SOUND, refreshOnion({
                    ...raw0,
                    isTampered: true,
                    entryKind: ek,
                    modeBit: mBit,
                    chunkJ: if (mBit == 1) 1 else 0,
                    chunkK: if (mBit == 1) 1 else 0,
                    param4Word: p4,
                    openInvW: invDiffTagSet(p4),
                    openSel: oSel,
                    a4Limbs: limbsOfFr(a4v),
                    a5Limbs: limbsOfFr(a5v),
                    callSetPub: if (ek == ENTRY_OPEN_CLAIM) digestPub(TAG_SHK_SET, a4v) else SET_PUB_HONEST,
                  }))
                  isAcceptedCfg(CFG_SOUND, cand) implies refinesWitness(cand)
                })
              )
            )
          )
        )
      )
    )
  }

  // Q1c: Exhaustive check over all limb pairs (including >= OP_LIMB_BOUND and non-canonical >= FR_P)
  // across all 3 R operands and claimed outcomes.
  pure val exhaustiveQ1OperandLimbsAndCanonOk: bool = {
    val basePrev = honestInitForCfg(CFG_SOUND)
    0.to(2).forall(opIdx =>
      0.to(OP_LIMB_BOUND).forall(lo =>
        0.to(OP_LIMB_BOUND).forall(hi =>
          Set(OUT_OK, OUT_ALT).forall(claimedOut => {
            val testFl: FrLimbs = { loLimb: lo, hiLimb: hi }
            val raw0 = withPreLedgerFrom(basePrev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
            val packedVal = packLimbsModFr(testFl)
            val tweaked =
              if (opIdx == 0) {
                val cCp = digestPub(TAG_SHK_PROGC, packedVal)
                val cOut = outcomePubOf(PROG_A, IN_1, claimedOut)
                {
                  ...raw0,
                  isTampered: true,
                  rOp0Limbs: testFl,
                  callCp: cCp,
                  callOutcomePub: cOut,
                  callClaimPub: claimHashPub(cCp, raw0.callInputPub, cOut),
                }
              } else if (opIdx == 1) {
                val cIn = digestPub(TAG_SHK_INPUT, packedVal)
                val cOut = outcomePubOf(PROG_A, IN_1, claimedOut)
                {
                  ...raw0,
                  isTampered: true,
                  rOp1Limbs: testFl,
                  callInputPub: cIn,
                  callOutcomePub: cOut,
                  callClaimPub: claimHashPub(raw0.callCp, cIn, cOut),
                }
              } else {
                val cOut = digestPub(TAG_SHK_OUTCOME, packedVal)
                {
                  ...raw0,
                  isTampered: true,
                  rOp2Limbs: testFl,
                  callOutcomePub: cOut,
                  callClaimPub: claimHashPub(raw0.callCp, raw0.callInputPub, cOut),
                }
              }
            val cand = finalizeTransition(CFG_SOUND, refreshOnion(tweaked))
            isAcceptedCfg(CFG_SOUND, cand) implies and {
              limbsCanon(testFl),
              refinesWitness(cand),
            }
          })
        )
      )
    )
  }

  // Q2a: Exhaustive per-limb Euclidean uniqueness over BB_P (scales BabyBear + 16-bit bound):
  // whenever 0 <= hPreLimb < HHI_BOUND and 0 <= cin <= HHI_BOUND (so hPreLimb + cin < 2*HHI_BOUND < BB_P),
  // any ranged limb 0 <= hhiLimb < HHI_BOUND and boolean carry cryBit in {0, 1} satisfying
  // hhiLimb + HHI_BOUND * cryBit == hPreLimb + cin (mod BB_P) equals integer Euclidean division.
  pure val exhaustiveQ2SingleLimbEuclideanOk: bool =
    0.to(HHI_BOUND - 1).forall(hPreLimb =>
      0.to(HHI_BOUND).forall(cin =>
        0.to(HHI_BOUND - 1).forall(hhiLimb =>
          0.to(1).forall(cryBit => {
            val modEq = (modPos(hhiLimb + HHI_BOUND * cryBit, BB_P) == modPos(hPreLimb + cin, BB_P))
            modEq implies and {
              hhiLimb == (hPreLimb + cin) % HHI_BOUND,
              cryBit == (hPreLimb + cin) / HHI_BOUND,
              hhiLimb + HHI_BOUND * cryBit == hPreLimb + cin,
            }
          })
        )
      )
    )

  // Q2b: Exhaustive 4-limb integer uniqueness below H64_BOUND (256, scaling 2^64):
  // across boundary values of h_pre in 0..255 and steps in 1..HHI_BOUND, checks all 4^4 * 2^3 = 2048
  // ranged (hhi, cry) tuples with cry[3] == 0: unique solution when sum < 256, zero solutions on overflow.
  pure val exhaustiveQ2FourLimbUniquenessOk: bool =
    Set(0, 1, 3, 6, 15, 63, 191, 252, 254, 255).forall(preVal =>
      1.to(HHI_BOUND).forall(st => {
        val hPre = encodeH64(preVal)
        val expectedSum = preVal + st
        val expectedLimbs = encodeH64(expectedSum % H64_BOUND)
        val expectedCarries = carriesOfAdd(hPre, st)
        0.to(HHI_BOUND - 1).forall(l0 =>
          0.to(HHI_BOUND - 1).forall(l1 =>
            0.to(HHI_BOUND - 1).forall(l2 =>
              0.to(HHI_BOUND - 1).forall(l3 =>
                0.to(1).forall(c0 =>
                  0.to(1).forall(c1 =>
                    0.to(1).forall(c2 => {
                      val candHhi = List(l0, l1, l2, l3)
                      val candCry = List(c0, c1, c2, 0)
                      val eqHolds = limbAddConstraintsOk(hPre, st, candHhi, candCry)
                      if (expectedSum < H64_BOUND)
                        eqHolds == and {
                          candHhi == expectedLimbs,
                          candCry == expectedCarries,
                          decodeH64(candHhi) == expectedSum,
                        }
                      else
                        not(eqHolds)
                    })
                  )
                )
              )
            )
          )
        )
      })
    )

  // Q2c: Exhaustive tamper check on N1 limbs (0..BB_P-1 across all 4 limb slots), multiplicities,
  // carries, and Path C steps (0..BB_P-1).
  pure val exhaustiveQ2AllPathsTamperOk: bool = {
    val basePrev = honestInitForCfg(CFG_SOUND)
    val n1Check =
      0.to(3).forall(limbIdx =>
        0.to(BB_P - 1).forall(limbVal =>
          0.to(1).forall(cryVal =>
            0.to(2).forall(multVal => {
              val raw0 = withPreLedgerFrom(basePrev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
              val adjIdx = if (limbIdx < 3) (limbIdx + 1) else 3
              val hhiBase = raw0.n1Hhi0.replaceAt(limbIdx, limbVal)
              val bumpedHhi =
                if (and { cryVal == 1, limbIdx < 3 })
                  hhiBase.replaceAt(adjIdx, modPos(hhiBase[adjIdx] + 1, BB_P))
                else
                  hhiBase
              val bumpedCry = raw0.n1Cry0.replaceAt(limbIdx, cryVal)
              val cand = finalizeTransition(CFG_SOUND, refreshOnion({
                ...raw0,
                isTampered: true,
                n1Hhi0: bumpedHhi,
                n1Cry0: bumpedCry,
                n1Mult0: multVal,
                rootHhi: bumpedHhi,
              }))
              isAcceptedCfg(CFG_SOUND, cand) implies refinesWitness(cand)
            })
          )
        )
      )
    val pathCCheck =
      0.to(BB_P - 1).forall(pcSteps =>
        0.to(HHI_BOUND - 1).forall(pre0 => {
          val raw0 = withPreLedgerFrom(basePrev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
          val cand = finalizeTransition(CFG_SOUND, refreshOnion({
            ...raw0,
            isTampered: true,
            pcBit: 1,
            leaf0Hpre: List(pre0, 0, 0, 0),
            leaf0Steps: pcSteps,
            rootHlo: List(0, 0, 0, 0),
            rootHhi: List(pcSteps, 0, 0, 0),
          }))
          isAcceptedCfg(CFG_SOUND, cand) implies refinesWitness(cand)
        })
      )
    and { n1Check, pathCCheck }
  }

  // Q3: Exhaustive tamper check across pcBit, child log, NOP/PUB entries, ENT_BUS, VAL_BUS, LOG_BUS, TIE_BUS
  pure val exhaustiveQ3LogTieAndOnionOk: bool = {
    val basePrev = honestInitForCfg(CFG_SOUND)
    val badChildD = stepOnionBb(D_EMPTY, entDigestBb({ opCode: OP_PUB, opParam: 22 }, 9))
    0.to(1).forall(pcVal =>
      Set(D_EMPTY, badChildD).forall(cLogD =>
        0.to(1).forall(cLogN =>
          Set(OP_NOP, OP_PUB).forall(extOp =>
            Set(0, 5, 6).forall(tblVal =>
              Set(false, true).forall(incExtra =>
                Set(false, true).forall(incNop =>
                  Set(0, 5, 6).forall(onVal =>
                    Set(0, 1).forall(unusedW =>
                      Set(false, true).forall(forgeTie => {
                        val raw0 =
                          if (pcVal == 0)
                            withPreLedgerFrom(basePrev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
                          else
                            mkHonestPathC(CFG_SOUND, basePrev)
                        val extParam = if (extOp == OP_PUB) 21 else 0
                        val draft = {
                          ...raw0,
                          isTampered: true,
                          preRuns: basePrev.postRuns,
                          preOpenRuns: basePrev.postOpenRuns,
                          preRoots: basePrev.postRoots,
                          preHistory: basePrev.postHistory,
                          childLogD: cLogD,
                          childLogN: cLogN,
                          extraEntOp: extOp,
                          extraEntParam: extParam,
                          tableExtraVal: tblVal,
                          valUsedCnt: if (extOp == OP_PUB) 6 else 5,
                          unusedValWord: unusedW,
                          onionIncExtra: incExtra,
                          onionIncNop: incNop,
                          onionExtraVal: onVal,
                        }
                        val onOut = evalOnion(buildOnionEntries(draft), buildOnionValWords(draft))
                        val expD = if (forgeTie) D_EMPTY else onOut.onD
                        val expN = if (forgeTie) 0 else onOut.onN
                        val cand = finalizeTransition(CFG_SOUND, {
                          ...draft,
                          exposedLogD: expD,
                          exposedLogN: expN,
                        })
                        isAcceptedCfg(CFG_SOUND, cand) implies refinesWitness(cand)
                      })
                    )
                  )
                )
              )
            )
          )
        )
      )
    )
  }

  // Q4: Exhaustive tamper check across SherlockLedger entry points, R1..R4, zero anchor,
  // unaccepted collection root, cross-entry replay, and privacy leak flag.
  pure val exhaustiveQ4LedgerProtocolOk: bool = {
    val step0 = honestInitForCfg(CFG_SOUND)
    val stepWithRoot = mkHonestAcceptCollection(CFG_SOUND, step0)
    Set(false, true).forall(hasRoot =>
      Set(cpPubOf(PROG_A), ZERO_PUB).forall(lAnc =>
        Set(PROG_A, PROG_B).forall(progId =>
          Set(IN_0, IN_1).forall(inId =>
            Set(OUT_OK, OUT_ALT).forall(claimedOut =>
              Set(true, false).forall(pfOk =>
                Set(false, true).forall(leakPriv =>
                  Set(ENTRY_ACCEPT_RUN, ENTRY_OPEN_CLAIM).forall(ek => {
                    val prevSt = if (hasRoot) stepWithRoot else step0
                    val rawBase =
                      if (ek == ENTRY_ACCEPT_RUN)
                        withPreLedgerFrom(prevSt, mkHonestRunBase(progId, inId, OUT_OK, PROG_A))
                      else {
                        val openRaw = withPreLedgerFrom(prevSt, mkHonestRunBase(progId, inId, OUT_OK, PROG_A))
                        refreshOnion({
                          ...openRaw,
                          entryKind: ENTRY_OPEN_CLAIM,
                          param4Word: TAG_SHK_SET,
                          openInvW: 0,
                          openSel: 1,
                          a4Limbs: limbsOfFr(SET_ROOT_PRE),
                          a5Limbs: limbsOfFr(sealOp0(PROG_A)),
                          callSetPub: SET_PUB_HONEST,
                        })
                      }
                    val cOutPub = outcomePubOf(progId, inId, claimedOut)
                    val cClaim = claimHashPub(rawBase.callCp, rawBase.callInputPub, cOutPub)
                    val cand = finalizeTransition(CFG_SOUND, {
                      ...rawBase,
                      isTampered: true,
                      ledgerAnchor: lAnc,
                      callOutcomePub: cOutPub,
                      callClaimPub: cClaim,
                      proofValidBit: pfOk,
                      leaksPrivate: leakPriv,
                    })
                    isAcceptedCfg(CFG_SOUND, cand) implies refinesWitness(cand)
                  })
                )
              )
            )
          )
        )
      )
    )
  }

  // ===========================================================================
  // 2. Group A: Sound Honest Execution & Multi-Path Reachability Tests
  // ===========================================================================

  run honestAcceptRunTest =
    init
      .expect(and {
        acc,
        invRefine,
        not(ws.isTampered),
        ws.runProg == PROG_A,
        ws.entryKind == ENTRY_ACCEPT_RUN,
        ws.postHistory.length() == 1,
      })

  run honestBinaryN1Test =
    init
      .then(applyCandidate(mkHonestBinaryN1(CFG_SOUND, ws)))
      .expect(and {
        acc,
        invRefine,
        ws.pcBit == 0,
        ws.n1V0 == 1,
        ws.n1V1 == 1,
        decodeH64(ws.rootHhi) == decodeH64(ws.rootHlo) + 4,
        ws.postHistory.length() == 2,
      })

  run honestPathCTest =
    init
      .then(applyCandidate(mkHonestPathC(CFG_SOUND, ws)))
      .expect(and {
        acc,
        invRefine,
        ws.pcBit == 1,
        ws.rootHlo == List(0, 0, 0, 0),
        ws.rootHhi == List(3, 0, 0, 0),
        ws.postHistory.length() == 2,
      })

  run honestAcceptCollAndOpenClaimTest =
    init
      .then(applyCandidate(mkHonestAcceptCollection(CFG_SOUND, ws)))
      .expect(and {
        acc,
        invRefine,
        ws.entryKind == ENTRY_ACCEPT_COLL,
        ws.postRoots.contains(SET_PUB_HONEST),
      })
      .then(applyCandidate(mkHonestOpenClaim(CFG_SOUND, ws, IN_1)))
      .expect(and {
        acc,
        invRefine,
        ws.entryKind == ENTRY_OPEN_CLAIM,
        ws.param4Word == TAG_SHK_SET,
        ws.openSel == 1,
        ws.postHistory.length() == 2,
      })

  // ===========================================================================
  // 3. Group B: Design Contract Refusal Tests (R1..R4, Zero Anchor, Privacy)
  // ===========================================================================

  run refuseR1ChangedOutcomeTest =
    init
      .then(applyCandidate(mkRefusalR1(CFG_SOUND, ws)))
      .expect(and {
        not(acc),
        invRefine,
        ws.attemptTag == ATTEMPT_R1,
        ws.postHistory.length() == 1,
      })

  run refuseR2FlippedProofTest =
    init
      .then(applyCandidate(mkRefusalR2(CFG_SOUND, ws)))
      .expect(and {
        not(acc),
        invRefine,
        ws.attemptTag == ATTEMPT_R2,
        ws.postHistory.length() == 1,
      })

  run refuseR3SameEntryReplayTest =
    init
      .then(applyCandidate(mkReplayAcceptRun(CFG_SOUND, ws)))
      .expect(and {
        not(acc),
        invRefine,
        ws.attemptTag == ATTEMPT_R3,
        ws.postHistory.length() == 1,
      })

  run refuseR3CrossEntryRunThenOpenTest =
    init
      .then(applyCandidate(mkHonestAcceptCollection(CFG_SOUND, ws)))
      .expect(acc)
      .then(applyCandidate(mkReplayOpenClaim(CFG_SOUND, ws)))
      .expect(and {
        not(acc),
        invRefine,
        ws.attemptTag == ATTEMPT_R3,
        ws.postHistory.length() == 1,
      })

  run refuseR3CrossEntryOpenThenRunTest =
    init
      .then(applyCandidate(mkHonestAcceptCollection(CFG_SOUND, ws)))
      .expect(acc)
      .then(applyCandidate(mkHonestOpenClaim(CFG_SOUND, ws, IN_1)))
      .expect(and { acc, ws.postHistory.length() == 2 })
      .then(applyCandidate(
        finalizeTransition(CFG_SOUND, withPreLedgerFrom(ws, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A)))
      ))
      .expect(and {
        not(acc),
        invRefine,
        ws.postHistory.length() == 2,
      })

  run refuseR4WrongProgramTest =
    init
      .then(applyCandidate(mkAttackR4WrongProg(CFG_SOUND, ws)))
      .expect(and {
        not(acc),
        invRefine,
        ws.attemptTag == ATTEMPT_R4,
        ws.postHistory.length() == 1,
      })

  run refuseZeroAnchorTest =
    init
      .then(applyCandidate(mkZeroAnchorAttempt(CFG_SOUND, ws)))
      .expect(and {
        not(acc),
        invRefine,
        ws.attemptTag == ATTEMPT_ZERO_ANCHOR,
        ws.postHistory.length() == 1,
      })

  run refusePrivacyLeakTest =
    init
      .expect({
        val h0 = honestInitForCfg(CFG_SOUND)
        val leakRun = finalizeTransition(CFG_SOUND, {
          ...withPreLedgerFrom(h0, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A)),
          leaksPrivate: true,
        })
        val leakColl = finalizeTransition(CFG_SOUND, {
          ...mkHonestAcceptCollection(CFG_SOUND, h0),
          leaksPrivate: true,
        })
        and {
          acc,
          not(isAcceptedCfg(CFG_SOUND, leakRun)),
          not(isAcceptedCfg(CFG_SOUND, leakColl)),
        }
      })

  // ===========================================================================
  // 4. Group C: Mutant Non-Vacuity (Honest Witness) + Real Attack Exploit Tests
  // ===========================================================================

  run mutantOpsCheckSkip0Test =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_OPS_SKIP_0)
        val atkM = mkAttackSkipOp(CFG_M_OPS_SKIP_0, honestM, 0)
        val atkSound = mkAttackSkipOp(CFG_SOUND, ws, 0)
        and {
          isAcceptedCfg(CFG_M_OPS_SKIP_0, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_OPS_SKIP_0, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantOpsCheckSkip1Test =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_OPS_SKIP_1)
        val atkM = mkAttackSkipOp(CFG_M_OPS_SKIP_1, honestM, 1)
        val atkSound = mkAttackSkipOp(CFG_SOUND, ws, 1)
        and {
          isAcceptedCfg(CFG_M_OPS_SKIP_1, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_OPS_SKIP_1, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantOpsCheckSkip2Test =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_OPS_SKIP_2)
        val atkM = mkAttackSkipOp(CFG_M_OPS_SKIP_2, honestM, 2)
        val atkSound = mkAttackSkipOp(CFG_SOUND, ws, 2)
        and {
          isAcceptedCfg(CFG_M_OPS_SKIP_2, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_OPS_SKIP_2, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantOpenFreeTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_OPEN_FREE)
        val atkM = mkAttackOpenFree(CFG_M_OPEN_FREE, honestM, 10)
        val atkSound = mkAttackOpenFree(CFG_SOUND, ws, 10)
        and {
          isAcceptedCfg(CFG_M_OPEN_FREE, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_OPEN_FREE, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantAnchorNotCpTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_ANCHOR_NOT_CP)
        val atkM = mkAttackR4WrongProg(CFG_M_ANCHOR_NOT_CP, honestM)
        val atkSound = mkAttackR4WrongProg(CFG_SOUND, ws)
        and {
          isAcceptedCfg(CFG_M_ANCHOR_NOT_CP, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_ANCHOR_NOT_CP, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantHhiOneLimbTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_HHI_ONE_LIMB)
        val atkM = mkAttackHhiOneLimb(CFG_M_HHI_ONE_LIMB, honestM)
        val atkSound = mkAttackHhiOneLimb(CFG_SOUND, ws)
        and {
          isAcceptedCfg(CFG_M_HHI_ONE_LIMB, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_HHI_ONE_LIMB, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantHhiValidFreeTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_HHI_VALID_FREE)
        val atkM = mkAttackHhiValidFree(CFG_M_HHI_VALID_FREE, honestM, 0)
        val atkSound = mkAttackHhiValidFree(CFG_SOUND, ws, 0)
        and {
          isAcceptedCfg(CFG_M_HHI_VALID_FREE, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_HHI_VALID_FREE, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  // Literal +valid on TIE_BUS (stmt.rs:1364) against OnionAir +O_FIN (+1):
  // net multiplicity is 1 + 1 = 2 != 0 (mod BB_P), causing completeness collapse
  // (neither honest nor forged witnesses are accepted). Paired with mutantTieDroppedTest below.
  run mutantTieMultPlusCompletenessCollapseTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_TIE_MULT_PLUS)
        val atkM = mkAttackForgedExposedLog(CFG_M_TIE_MULT_PLUS, honestM)
        and {
          not(isAcceptedCfg(CFG_M_TIE_MULT_PLUS, honestM)),
          refinesWitness(honestM),
          not(isAcceptedCfg(CFG_M_TIE_MULT_PLUS, atkM)),
        }
      })

  run mutantTieDroppedTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_TIE_DROPPED)
        val atkM = mkAttackForgedExposedLog(CFG_M_TIE_DROPPED, honestM)
        val atkSound = mkAttackForgedExposedLog(CFG_SOUND, ws)
        and {
          isAcceptedCfg(CFG_M_TIE_DROPPED, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_TIE_DROPPED, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantOnionSkipValueTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_ONION_SKIP_VAL)
        val atkM = mkAttackOnionSkipValue(CFG_M_ONION_SKIP_VAL, honestM, 6)
        val atkSound = mkAttackOnionSkipValue(CFG_SOUND, ws, 6)
        and {
          isAcceptedCfg(CFG_M_ONION_SKIP_VAL, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_ONION_SKIP_VAL, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantChildLogFreePc1Test =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_CHILD_LOG_FREE_PC1)
        val atkM = mkAttackChildLogPc1(CFG_M_CHILD_LOG_FREE_PC1, honestM)
        val atkSound = mkAttackChildLogPc1(CFG_SOUND, ws)
        and {
          isAcceptedCfg(CFG_M_CHILD_LOG_FREE_PC1, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_CHILD_LOG_FREE_PC1, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantLedgerNoReplayTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_LEDGER_NO_REPLAY)
        val atkM = mkReplayAcceptRun(CFG_M_LEDGER_NO_REPLAY, honestM)
        val atkSound = mkReplayAcceptRun(CFG_SOUND, ws)
        and {
          isAcceptedCfg(CFG_M_LEDGER_NO_REPLAY, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_LEDGER_NO_REPLAY, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  run mutantLedgerTwoMapsTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_LEDGER_TWO_MAPS)
        val collM = mkHonestAcceptCollection(CFG_M_LEDGER_TWO_MAPS, honestM)
        val atkM = mkReplayOpenClaim(CFG_M_LEDGER_TWO_MAPS, collM)
        val collSound = mkHonestAcceptCollection(CFG_SOUND, ws)
        val atkSound = mkReplayOpenClaim(CFG_SOUND, collSound)
        and {
          isAcceptedCfg(CFG_M_LEDGER_TWO_MAPS, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_LEDGER_TWO_MAPS, collM),
          isAcceptedCfg(CFG_M_LEDGER_TWO_MAPS, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  // Diagnostic mutant for real code defect in Kind::Work (stmt.rs:1079-1086)
  run mutantWorkHhiUnrangedTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_WORK_HHI_UNRANGED)
        val atkM = mkAttackWorkHhiUnranged(CFG_M_WORK_HHI_UNRANGED, honestM)
        and {
          isAcceptedCfg(CFG_M_WORK_HHI_UNRANGED, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_WORK_HHI_UNRANGED, atkM),
          not(refinesWitness(atkM)),
        }
      })

  // Diagnostic mutant for chunked j=1 anchor bypass in gen_ops_check (lverify.rs:1876)
  run mutantChunkJ1AnchorTest =
    init
      .expect({
        val honestM = honestInitForCfg(CFG_M_CHUNK_J1_ANCHOR)
        val atkM = mkAttackChunkJ1Anchor(CFG_M_CHUNK_J1_ANCHOR, honestM)
        val atkSound = mkAttackChunkJ1Anchor(CFG_SOUND, ws)
        and {
          isAcceptedCfg(CFG_M_CHUNK_J1_ANCHOR, honestM),
          refinesWitness(honestM),
          isAcceptedCfg(CFG_M_CHUNK_J1_ANCHOR, atkM),
          not(refinesWitness(atkM)),
          not(isAcceptedCfg(CFG_SOUND, atkSound)),
        }
      })

  // ===========================================================================
  // 5. Group D: Exhaustive Finite-Domain Soundness Tests
  // ===========================================================================

  run exhaustiveQ1IsZeroGadgetTest =
    init.expect(exhaustiveIsZeroGadgetOk)

  run exhaustiveQ1SelectorsAndAnchorTest =
    init.expect(exhaustiveQ1AnchorAndSelectorsOk)

  run exhaustiveQ1OperandLimbsAndCanonTest =
    init.expect(exhaustiveQ1OperandLimbsAndCanonOk)

  run exhaustiveQ2SingleLimbEuclideanTest =
    init.expect(exhaustiveQ2SingleLimbEuclideanOk)

  run exhaustiveQ2FourLimbUniquenessTest =
    init.expect(exhaustiveQ2FourLimbUniquenessOk)

  run exhaustiveQ2AllPathsTamperTest =
    init.expect(exhaustiveQ2AllPathsTamperOk)

  run exhaustiveQ3LogTieAndOnionTest =
    init.expect(exhaustiveQ3LogTieAndOnionOk)

  run exhaustiveQ4LedgerProtocolTest =
    init.expect(exhaustiveQ4LedgerProtocolOk)
}
