// Deterministic tests, non-vacuity traces, mutant attack/honest-use pairs, and exhaustive pure
// checks over finite attack domains for Sherlock row T20 (batched Midnight writes).
//
// Every `run` definition ends in `Test` so `quint test --main <module>` executes all of them
// without silently skipping any test.

// ============================================================================
// 1. Sound Design: Exhaustive Pure Checks + Scenario & Refusal Tests
// ============================================================================

module t20_batched_test {
  import t20_batched_base(MUT = "none", REG_MODE = "strict").* from "./t20_batched"

  // --------------------------------------------------------------------------
  // 1A. Exhaustive Pure Helper Predicates over Finite Attack Domains
  // --------------------------------------------------------------------------

  pure val ALL_BIT_PAIRS: Set[List[bool]] = Set(
    List(false, false),
    List(true, false),
    List(false, true),
    List(true, true)
  )

  pure val REP_TREES: Set[List[int]] = Set(
    EMPTY_INT_LIST,
    List(runClaimPub(RUN_A)),
    List(runClaimPub(RUN_A), fgnLeafOf("none", FGN_1.fpub)),
    List(runClaimPub(RUN_A), runClaimPub(RUN_A), fgnLeafOf("none", FGN_1.fpub)),
    List(runClaimPub(RUN_A), runClaimPub(RUN_B), fgnLeafOf("none", FGN_1.fpub), fgnLeafOf("none", FGN_2.fpub))
  )

  // Exhaustive check 1: H2 / H3 zero-preimage resistance, cross-arity domain separation, and injectivity
  pure val checkExhaustiveHashSeparation: bool = {
    val allRuns = PROVED_RUNS.union(UNPROVED_RUNS)
    and {
      // No 2-input hash or 3-input hash collides with ZERO_FIELD = 0
      0.to(12).forall(a => 0.to(12).forall(b => hash2(a, b) != ZERO_FIELD)),
      allRuns.forall(r => runClaimPub(r) != ZERO_FIELD),
      0.to(30).forall(fp => fgnLeafOf("none", fp) != ZERO_FIELD),
      // Cross-arity separation: transientHash<2> (cap(0,2)) never equals transientHash<3> (cap(0,3))
      0.to(12).forall(a => 0.to(12).forall(b =>
        allRuns.forall(r => hash2(a, b) != runClaimPub(r))
      )),
      // Injectivity of hash2 on small domain and runClaimPub on all modelled RunSpecs
      0.to(6).forall(a1 => 0.to(6).forall(b1 =>
        0.to(6).forall(a2 => 0.to(6).forall(b2 =>
          (hash2(a1, b1) == hash2(a2, b2)) implies (a1 == a2 and b1 == b2)
        ))
      )),
      allRuns.forall(r1 => allRuns.forall(r2 =>
        (runClaimPub(r1) == runClaimPub(r2)) implies (r1 == r2)
      )),
    }
  }

  // Build an accepted-tree test state for exhaustive Q2 consumer checks
  pure def stateWithOneAcceptedTree(leaves: List[int]): SysState = {
    val rv = treeRoot4(leaves)
    val hRuns = PROVED_RUNS.filter(r => leaves.indices().exists(i => leaves.nth(i) == runClaimPub(r)))
    val hFgns = VALID_FGNS.filter(f => leaves.indices().exists(i => leaves.nth(i) == fgnLeafOf("none", f.fpub)))
    {
      ...initState,
      collEpoch: 1,
      claimRoots: Set(rv),
      rootLeaves: Map(rv -> leaves),
      rootEpochMap: Map(rv -> 0),
      honestRootRuns: Map(rv -> hRuns),
      honestRootFgns: Map(rv -> hFgns),
      rootMaxDepth: Map(rv -> 1),
      daPublishedRoots: Set(rv),
      ledgerWrites: 1,
      collEpochsWithRoot: Set(0),
    }
  }

  // Exhaustive check 2: Q2(a), Q2(c), Q2(d), empty slot 0, and internal nodes over all 5 representative trees
  pure val checkExhaustiveMerkleAndConsumerQ2: bool =
    REP_TREES.forall(leaves => {
      val padded = padLeaves4(leaves)
      val rv = treeRoot4(leaves)
      val intNodes = treeInternalNodes4(leaves)
      val sibPool = Set(ZERO_FIELD, hash2(ZERO_FIELD, ZERO_FIELD), rv)
        .union(intNodes)
        .union(ALL_SLOTS.map(idx => padded.nth(idx)))
      val stAcc = stateWithOneAcceptedTree(leaves)
      val candRuns = PROVED_RUNS.union(UNPROVED_RUNS)
      val candFpubs = Set(
        ZERO_FIELD,
        FGN_1.fpub,
        FGN_2.fpub,
        FORGED_FPUB,
        runClaimPub(RUN_A),
        runClaimPub(RUN_B),
        fgnLeafOf("none", FGN_1.fpub)
      ).union(intNodes)
      and {
        // 1. Empty slot 0 IS a valid depth-D_TREE Merkle leaf at every unfilled slotIdx >= leaves.length(),
        //    proving that a consumer without leaf hashing would accept 0 on non-full trees:
        ALL_SLOTS.forall(slotIdx =>
          (slotIdx >= leaves.length()) implies
            (padded.nth(slotIdx) == ZERO_FIELD and rootFromPath(ZERO_FIELD, treePath4(leaves, slotIdx)) == rv)
        ),
        // 2. Internal nodes and the root itself NEVER verify as depth-D_TREE leaves for ANY sibling/bit path:
        intNodes.union(Set(rv)).forall(iNode =>
          sibPool.forall(s0 => sibPool.forall(s1 => ALL_BIT_PAIRS.forall(bits =>
            rootFromPath(iNode, { sibs: List(s0, s1), bits: bits }) != rv
          )))
        ),
        // 3. Exhaustive useRunClaim check over all candidate RunSpecs and all (s0, s1, bits) witness paths:
        //    accepted IFF the run leaf actually sits at bitsToSlot(bits) of the accepted tree!
        candRuns.forall(r =>
          sibPool.forall(s0 => sibPool.forall(s1 => ALL_BIT_PAIRS.forall(bits => {
            val candPath = { sibs: List(s0, s1), bits: bits }
            val slotIdx = bitsToSlot(bits)
            val expectedOk = (
              slotIdx < leaves.length() and
              leaves.nth(slotIdx) == runClaimPub(r) and
              candPath == treePath4(leaves, slotIdx)
            )
            canUseRunClaim("none", stAcc, 1, r, candPath) == expectedOk
          })))
        ),
        // 4. Exhaustive useForeignClaim check over all candidate fpub Field inputs (including 0, runClaimPub,
        //    already-tagged foreign leaves, and internal nodes) and all (s0, s1, bits) witness paths:
        candFpubs.forall(fp =>
          sibPool.forall(s0 => sibPool.forall(s1 => ALL_BIT_PAIRS.forall(bits => {
            val candPath = { sibs: List(s0, s1), bits: bits }
            val slotIdx = bitsToSlot(bits)
            val expectedOk = (
              slotIdx < leaves.length() and
              leaves.nth(slotIdx) == fgnLeafOf("none", fp) and
              candPath == treePath4(leaves, slotIdx)
            )
            canUseForeignClaim("none", stAcc, 1, fp, candPath) == expectedOk
          })))
        ),
      }
    })

  // Helper to apply one of 4 valid consumption paths for RUN_A in exhaustive Q2(b) check:
  //   0 = root1 slot 0, 1 = root1 slot 1 (duplicate leaf in same root), 2 = root2 slot 1, 3 = runsMap
  pure def canUseRunAWay(s: SysState, cid: int, way: int, leaves1: List[int], leaves2: List[int]): bool =
    if (way == 0) canUseRunClaim("none", s, cid, RUN_A, treePath4(leaves1, 0))
    else if (way == 1) canUseRunClaim("none", s, cid, RUN_A, treePath4(leaves1, 1))
    else if (way == 2) canUseRunClaim("none", s, cid, RUN_A, treePath4(leaves2, 1))
    else canUseSettledRun(s, cid, RUN_A)

  pure def applyUseRunAWay(s: SysState, cid: int, way: int, leaves1: List[int], leaves2: List[int]): SysState =
    if (way == 0) applyUseRunClaim("none", s, cid, RUN_A, treePath4(leaves1, 0))
    else if (way == 1) applyUseRunClaim("none", s, cid, RUN_A, treePath4(leaves1, 1))
    else if (way == 2) applyUseRunClaim("none", s, cid, RUN_A, treePath4(leaves2, 1))
    else applyUseSettledRun(s, cid, RUN_A)

  // Exhaustive check 3: Q2(b) single-use across two roots, root + settled runsMap, AND two leaf slots of one root
  pure val checkExhaustiveDoubleUseQ2b: bool = {
    val leaves1 = List(runClaimPub(RUN_A), runClaimPub(RUN_A), fgnLeafOf("none", FGN_1.fpub))
    val leaves2 = List(fgnLeafOf("none", FGN_1.fpub), runClaimPub(RUN_A))
    val rv1 = treeRoot4(leaves1)
    val rv2 = treeRoot4(leaves2)
    val baseSt: SysState = {
      ...initState,
      collEpoch: 2,
      runsMap: Map(runClaimPub(RUN_A) -> RUN_A.outp),
      claimRoots: Set(rv1, rv2),
      rootLeaves: Map(rv1 -> leaves1, rv2 -> leaves2),
      rootEpochMap: Map(rv1 -> 0, rv2 -> 1),
      honestRootRuns: Map(rv1 -> Set(RUN_A), rv2 -> Set(RUN_A)),
      honestRootFgns: Map(rv1 -> Set(FGN_1), rv2 -> Set(FGN_1)),
      rootMaxDepth: Map(rv1 -> 1, rv2 -> 1),
      daPublishedRoots: Set(rv1, rv2),
      ledgerWrites: 3,
      collEpochsWithRoot: Set(0, 1),
      userPaidRuns: 1,
    }
    and {
      // All 4 x 4 ordered pairs of RUN_A sources: consumer 1 succeeds once, is blocked on 2nd use,
      // while consumer 2 remains completely unblocked on its own `used` set.
      0.to(3).forall(w1 => 0.to(3).forall(w2 => {
        val ok1 = canUseRunAWay(baseSt, 1, w1, leaves1, leaves2)
        val after1 = applyUseRunAWay(baseSt, 1, w1, leaves1, leaves2)
        val blockedRepeat = not(canUseRunAWay(after1, 1, w2, leaves1, leaves2))
        val okOtherConsumer = canUseRunAWay(after1, 2, w2, leaves1, leaves2)
        val after2 = applyUseRunAWay(after1, 2, w2, leaves1, leaves2)
        ok1 and checkUseOnce(after1) and blockedRepeat and okOtherConsumer and checkUseOnce(after2)
      })),
      // All 2 x 2 ordered pairs of FGN_1 sources (root1 slot 2 vs root2 slot 0):
      Set(0, 1).forall(fw1 => Set(0, 1).forall(fw2 => {
        val pFirst = if (fw1 == 0) treePath4(leaves1, 2) else treePath4(leaves2, 0)
        val pSecond = if (fw2 == 0) treePath4(leaves1, 2) else treePath4(leaves2, 0)
        val ok1 = canUseForeignClaim("none", baseSt, 1, FGN_1.fpub, pFirst)
        val after1 = applyUseForeignClaim("none", baseSt, 1, FGN_1.fpub, pFirst)
        val blockedRepeat = not(canUseForeignClaim("none", after1, 1, FGN_1.fpub, pSecond))
        val okOther = canUseForeignClaim("none", after1, 2, FGN_1.fpub, pSecond)
        ok1 and checkUseOnce(after1) and blockedRepeat and okOther
      })),
    }
  }

  // Deterministic multi-parameter run builder for Q1 exhaustive write-accounting verification
  pure def runWriteAccountingScenario(
    ep0Jobs: int,
    ep1Jobs: int,
    uRuns: int,
    cUses: int,
    regKeyOps: int,
    dagD: int
  ): SysState = {
    val runList = List(RUN_A, RUN_B, RUN_C, RUN_D)
    // 1. Stage regKeyOps (0..2) in registry epoch 0 and close registry epoch 0
    val sReg1 = if (regKeyOps >= 1) applyStageKeyChange("none", initState, 2, true) else initState
    val sReg2 = if (regKeyOps >= 2) applyStageKeyChange("none", sReg1, 3, true) else sReg1
    val sAfterReg = applyCloseRegEpoch("none", sReg2)
    // 2. Collection epoch 0: prove ep0Jobs (0..3) and close
    val sEp0Proved = range(0, ep0Jobs).foldl(sAfterReg, (acc, idx) =>
      applyProveRunJob("none", acc, runList.nth(idx), dagD, true)
    )
    val sAfterEp0 =
      if (ep0Jobs == 0) applyCloseEmptyEpoch(sEp0Proved)
      else applyAcceptCollection("none", sEp0Proved, sEp0Proved.stagedRegRoot, true)
    // 3. Collection epoch 1: prove ep1Jobs (0..3, shifted by RUN_E on slot 0 to keep distinct root) and close
    val ep1List = List(RUN_E, RUN_B, RUN_C, RUN_D)
    val sEp1Proved = range(0, ep1Jobs).foldl(sAfterEp0, (acc, idx) =>
      applyProveRunJob("none", acc, ep1List.nth(idx), dagD, true)
    )
    val sAfterEp1 =
      if (ep1Jobs == 0) applyCloseEmptyEpoch(sEp1Proved)
      else applyAcceptCollection("none", sEp1Proved, sEp1Proved.stagedRegRoot, true)
    // 4. User-submitted runs (0..3)
    val sAfterUsers = range(0, uRuns).foldl(sAfterEp1, (acc, idx) =>
      applyUserAcceptRun(acc, runList.nth(idx))
    )
    // 5. Consumer uses (0..2 consumers using available epoch-0 or settled runs)
    if (ep0Jobs > 0) {
      val leaves0 = pendingLeaves("none", sEp0Proved.pendingJobs)
      val p0 = treePath4(leaves0, 0)
      val sUse1 = if (cUses >= 1) applyUseRunClaim("none", sAfterUsers, 1, RUN_A, p0) else sAfterUsers
      if (cUses >= 2) applyUseRunClaim("none", sUse1, 2, RUN_A, p0) else sUse1
    } else {
      sAfterUsers
    }
  }

  // Exhaustive check 4: Q1 exact SherlockLedger write count independence over 1,296 parameter combinations
  pure val checkExhaustiveWriteAccountingQ1: bool =
    0.to(3).forall(ep0Jobs =>
      0.to(3).forall(ep1Jobs =>
        0.to(2).forall(uRuns =>
          0.to(2).forall(cUses =>
            0.to(2).forall(regKeyOps =>
              DAG_DEPTHS.forall(dagD => {
                val endSt = runWriteAccountingScenario(ep0Jobs, ep1Jobs, uRuns, cUses, regKeyOps, dagD)
                val expectedWrites =
                  (if (ep0Jobs > 0) 1 else 0) +
                  (if (ep1Jobs > 0) 1 else 0) +
                  uRuns +
                  (if (regKeyOps > 0) 1 else 0)
                and {
                  endSt.ledgerWrites == expectedWrites,
                  checkWritesBounded(endSt),
                  checkRegWritesPerEpoch(endSt),
                  checkNoDescendCalls(endSt),
                }
              })
            )
          )
        )
      )
    )

  // Exhaustive check 5: Q4 chunk cap arithmetic, measured entries, TTL bounds, and GP-1..GP-4 invariance
  pure val checkExhaustiveChunkCapQ4: bool = {
    val allTestSizes = MEASURED_SIZES.union(
      Set(1, 3, STATIC_MAX_WORDS, 1014, 1015, 1016, 1979, 1980, 1981, 2970, 5000, 17820, 17821)
    )
    val validTtls = Set(1755, 3600, 7200, 35000, 100000)
    and {
      ONE_CALL_MAX == 1015,
      DEFAULT_TTL_SEC / TEN_X_PROVE_SEC == 2,
      BLOCK_COMPUTE_CAP == 18,
      FULL_CHUNK_BYTES == 39453,
      TX_BYTE_CAP == 26,
      derivedKMax(1754) == 0,
      derivedKMax(1755) == 1,
      derivedKMax(3600) == 2,
      derivedKMax(35000) == 18,
      derivedKMax(100000) == 18,
      // Below 10x single-wrap proving time (TTL < 1,755 s), Compact planner refuses cleanly:
      allTestSizes.forall(w => {
        val pShort = planCompactCall("none", w, 1754)
        pShort.refused and pShort.chunks == 0 and pShort.coveredWords == 0
      }),
      // Across all valid TTLs and sizes, chunk cap and no-truncation hold without exception:
      validTtls.forall(ttl => allTestSizes.forall(w => {
        val stPlan = applySubmitCompactPlan("none", initState, w, ttl)
        checkChunkCap(stPlan) and checkNoTruncation(stPlan)
      })),
      // At default 3,600 s TTL: <= 1,015 is 1 call (0 anchor writes); 1,016..1,980 is 2 chunks; >= 1,981 refused:
      allTestSizes.forall(w => {
        val p = planCompactCall("none", w, DEFAULT_TTL_SEC)
        if (w <= 1015) (p.chunks == 1 and p.anchorWrites == 0 and not(p.refused))
        else if (w <= 1980) (p.chunks == 2 and p.anchorWrites == 2 and not(p.refused))
        else (p.chunks == 0 and p.anchorWrites == 0 and p.refused)
      }),
      // General Sherlock runs (GP-1..GP-4) always write the fixed 3-word run statement in 1 call:
      validTtls.forall(ttl => {
        val gp = planGeneralRun(ttl)
        gp.effectWords == 3 and gp.chunks == 1 and gp.anchorWrites == 0 and not(gp.refused)
      }),
    }
  }

  // Exhaustive check 6: Q5 strict vs one-epoch grace registry check and circuit key-membership enforcement
  pure val checkExhaustiveRegistryQ5: bool =
    and {
      Set(0, 1, 2, 3).forall(curR =>
        Set(0, 1, 2).forall(prevR =>
          Set(0, 1, 2, 3, 99).forall(candR => and {
            regRootAccepted("none", "strict", curR, prevR, candR) == (candR == curR),
            regRootAccepted("none", "grace_one", curR, prevR, candR) == (candR == curR or candR == prevR),
          })
        )
      ),
      ALL_KEYS.powerset().forall(admittedKeys =>
        VALID_FGNS.forall(f => {
          val stCustom = { ...initState, regTable: Map(0 -> admittedKeys) }
          canProveFgnJob(stCustom, f, 0) == admittedKeys.contains(f.keyId)
        })
      ),
    }

  run exhaustiveHashSeparationTest =
    init.expect(checkExhaustiveHashSeparation)

  run exhaustiveMerkleAndConsumerQ2Test =
    init.expect(checkExhaustiveMerkleAndConsumerQ2)

  run exhaustiveDoubleUseQ2bTest =
    init.expect(checkExhaustiveDoubleUseQ2b)

  run exhaustiveWriteAccountingQ1Test =
    init.expect(checkExhaustiveWriteAccountingQ1)

  run exhaustiveChunkCapQ4Test =
    init.expect(checkExhaustiveChunkCapQ4)

  run exhaustiveRegistryQ5Test =
    init.expect(checkExhaustiveRegistryQ5)

  // --------------------------------------------------------------------------
  // 1B. Deterministic Positive, Negative, DA/Fallback, and Starvation Traces
  // --------------------------------------------------------------------------

  // Full batched epoch: 3 proved jobs (2 run + 1 foreign, depth 3) settle in 1 SherlockLedger write;
  // two consumers use RUN_A and FGN_1 with 0 extra SherlockLedger writes.
  run honestBatchAndTwoConsumersTest = {
    val batchLeaves = List(runClaimPub(RUN_A), runClaimPub(RUN_B), fgnLeafOf("none", FGN_1.fpub))
    init
      .then(proveRunJob(RUN_A, 3, true))
      .then(proveRunJob(RUN_B, 1, true))
      .then(proveFgnJob(FGN_1, 3, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(batchLeaves, 0)))
      .then(consumerUseRunFromRoot(2, RUN_A, treePath4(batchLeaves, 0)))
      .then(consumerUseForeignFromRoot(1, FGN_1.fpub, treePath4(batchLeaves, 2)))
      .expect(and {
        invAll,
        st.ledgerWrites == 1,
        seenUsedRunClaim,
        seenUsedForeignClaim,
        seenDeepDagClaimUsed,
      })
  }

  // Empty collection epochs and unchanged registry epochs write nothing to SherlockLedger
  run honestEmptyEpochsZeroWritesTest =
    init
      .then(closeEmptyCollEpoch)
      .then(closeEmptyCollEpoch)
      .then(closeRegEpoch)
      .then(stageKeyChange(2, true))
      .then(stageKeyChange(2, false))
      .then(closeRegEpoch)
      .expect(invAll and st.ledgerWrites == 0 and st.claimRoots.size() == 0 and st.currentRegRoot == 0)

  // Q2(b)(iii): diamond-shared claim appearing at BOTH slot 0 and slot 1 of ONE claims_root is used at most once
  run refuseDuplicateSlotsInSameRootTest = {
    val dupLeaves = List(runClaimPub(RUN_A), runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 1, true))
      .then(proveRunJob(RUN_A, 1, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(dupLeaves, 0)))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(dupLeaves, 1)).fail())
      .expect(invAll and st.useLog.length() == 1)
  }

  // Q2(a, c, d): non-member, cross-kind, empty-slot-0, and pre-close uses are all refused
  run refuseForgedCrossKindEmptyAndUnclosedTest = {
    val oneLeaf = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      // Q2(d): epoch 0 not yet closed -> root not accepted
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(oneLeaf, 0)).fail())
      .then(acceptCollectionEpoch(0, true))
      // Q2(a): forged run claim not in accepted tree
      .then(consumerUseRunFromRoot(1, FORGED_RUN_A, treePath4(oneLeaf, 0)).fail())
      // Q2(c): valid run leaf submitted to useForeignClaim
      .then(consumerUseForeignFromRoot(1, runClaimPub(RUN_A), treePath4(oneLeaf, 0)).fail())
      // Q2 empty slot 0: slot 1 holds 0 in tree, but useForeignClaim(0) hashes H2(TAG_FGN, 0) != 0
      .then(consumerUseForeignFromRoot(1, ZERO_FIELD, treePath4(oneLeaf, 1)).fail())
      .expect(invAll and st.useLog.length() == 0)
  }

  // Q3: Withheld claim list delays third-party path discovery (slotPathAvailable == false without receipt),
  // receipt holder can still consume immediately, and user's acceptRun + useSettledRun fallback is sound
  // and blocks a second consumption after DA is later published (and in reverse order too).
  run daWithheldReceiptAndFallbackSoundnessTest = {
    val twoLeaves = List(runClaimPub(RUN_A), runClaimPub(RUN_B))
    val rv = treeRoot4(twoLeaves)
    init
      // RUN_A has no receipt; RUN_B has operator receipt
      .then(proveRunJob(RUN_A, 0, false))
      .then(proveRunJob(RUN_B, 0, true))
      // Epoch closes with DA withheld (publishDa = false)
      .then(acceptCollectionEpoch(0, false))
      .expect(not(slotPathAvailable(st, rv, 0)) and slotPathAvailable(st, rv, 1))
      // Submitter with receipt for slot 1 (RUN_B) can consume immediately despite withheld list
      .then(consumerUseRunFromRoot(1, RUN_B, treePath4(twoLeaves, 1)))
      // User of RUN_A falls back to self-paid acceptRun + useSettledRun while DA is withheld
      .then(userAcceptRun(RUN_A))
      .then(consumerUseSettledRun(1, RUN_A))
      .expect(invAll and seenFallbackSettledRunUsed and st.ledgerWrites == 2)
      // Later DA publication reveals path for slot 0, but consumer 1's nullifier blocks a 2nd use!
      .then(publishDa(rv))
      .expect(slotPathAvailable(st, rv, 0))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(twoLeaves, 0)).fail())
      // Reverse order check on RUN_B: already used via root by consumer 1 -> settled-run fallback also blocked!
      .then(userAcceptRun(RUN_B))
      .then(consumerUseSettledRun(1, RUN_B).fail())
      // Consumer 2 (fresh nullifier set) can still use RUN_A once from the newly published root
      .then(consumerUseRunFromRoot(2, RUN_A, treePath4(twoLeaves, 0)))
      .expect(invAll and st.ledgerWrites == 3 and st.useLog.length() == 3)
  }

  // Q4: Measured entries (474, 699, 1015) take 1 call (0 anchor writes); 1016..1980 take 2 chunks;
  // >= 1981 refused at plan time; general Sherlock runs take 1 call.
  run chunkCapBoundariesTest =
    init
      .then(submitCompactPlan(474, DEFAULT_TTL_SEC))
      .then(submitCompactPlan(699, DEFAULT_TTL_SEC))
      .then(submitCompactPlan(1015, DEFAULT_TTL_SEC))
      .expect(invAll and st.userContractAnchorWrites == 0)
      .then(submitCompactPlan(1016, DEFAULT_TTL_SEC))
      .then(submitCompactPlan(1980, DEFAULT_TTL_SEC))
      .then(submitCompactPlan(1981, DEFAULT_TTL_SEC))
      .then(submitGeneralRunPlan(DEFAULT_TTL_SEC))
      .expect(and {
        invAll,
        seenChunkedCall,
        seenRefusedPlan,
        st.userContractAnchorWrites == 4,
      })

  // Q5 (batched registry update + new key admission): 2 key changes in 1 registry epoch -> 1 write
  run registryBatchedTwoKeysAndNewKeyCollectionTest = {
    val fgn2Leaves = List(fgnLeafOf("none", FGN_2.fpub))
    init
      // Key 2 not yet registered under genesis root 0 -> foreign job with FGN_2 refused
      .then(proveFgnJob(FGN_2, 0, true).fail())
      // Stage two net key changes in registry epoch 0 (add key 2, add key 3) -> 1 reg_root write at close
      .then(stageKeyChange(2, true))
      .then(stageKeyChange(3, true))
      .then(closeRegEpoch)
      .expect(invAll and seenRegTwoKeyChanges and st.currentRegRoot == 1 and st.ledgerWrites == 1)
      // Now prove FGN_2 under reg_root 1 and accept collection under reg_root 1
      .then(proveFgnJob(FGN_2, 0, true))
      .then(acceptCollectionEpoch(1, true))
      .then(consumerUseForeignFromRoot(1, FGN_2.fpub, treePath4(fgn2Leaves, 0)))
      .expect(invAll and seenCollectionUnderNewReg and st.ledgerWrites == 2)
  }

  // Q5 (strict liveness starvation): if registry changes every epoch before operator submits acceptCollection,
  // strict `reg_root == reg_pub` refuses the collection repeatedly until the operator re-proves under currentRegRoot.
  run strictRegistryChurnStarvationTest =
    init
      .then(proveFgnJob(FGN_1, 0, true))
      // Registry epoch 0 updates before collection settles
      .then(stageKeyChange(2, true))
      .then(closeRegEpoch)
      .then(acceptCollectionEpoch(0, true).fail())
      .then(recordStaleRefusal)
      // Operator re-proves under reg 1, but registry epoch 1 updates again before inclusion!
      .then(reprovePendingUnderCurrentReg)
      .then(stageKeyChange(3, true))
      .then(closeRegEpoch)
      .then(acceptCollectionEpoch(1, true).fail())
      .then(recordStaleRefusal)
      // Operator re-proves under reg 2, and registry epoch 2 updates a 3rd time!
      .then(reprovePendingUnderCurrentReg)
      .then(stageKeyChange(2, false))
      .then(closeRegEpoch)
      .then(acceptCollectionEpoch(2, true).fail())
      .then(recordStaleRefusal)
      .expect(invAll and st.collRefusals == 3 and st.acceptedColls.length() == 0)
}

// ============================================================================
// 2. Open N-8 Option: One-Epoch Registry Grace Window (`REG_MODE = "grace_one"`)
// ============================================================================

module t20_batched_grace_test {
  import t20_batched_base(MUT = "none", REG_MODE = "grace_one").* from "./t20_batched"

  // Under one-epoch grace, a collection proved under reg 0 still lands after 1 registry update (current = 1, prev = 0)
  run graceAcceptsOneEpochOldRootTest =
    init
      .then(proveFgnJob(FGN_1, 0, true))
      .then(stageKeyChange(2, true))
      .then(closeRegEpoch)
      .expect(st.currentRegRoot == 1 and st.prevRegRoot == 0)
      .then(acceptCollectionEpoch(0, true))
      .expect(invAll and invGraceRegRefused and st.acceptedColls.length() == 1)

  // Under one-epoch grace, a root 2 epochs old (reg 0 when current = 2, prev = 1) is still refused
  run graceRefusesTwoEpochsOldRootTest =
    init
      .then(proveFgnJob(FGN_1, 0, true))
      .then(stageKeyChange(2, true))
      .then(closeRegEpoch)
      .then(stageKeyChange(3, true))
      .then(closeRegEpoch)
      .expect(st.currentRegRoot == 2 and st.prevRegRoot == 1)
      .then(acceptCollectionEpoch(0, true).fail())
      .then(recordStaleRefusal)
      .expect(invAll and st.collRefusals == 1 and st.acceptedColls.length() == 0)

  // Security/liveness tradeoff of naive one-epoch grace (why N-8 remains open):
  // If key 1 is REVOKED in reg 0 -> reg 1, naive grace still accepts a collection proved under prev = 0!
  run graceRevokedKeyTradeoffTest =
    init
      .then(proveFgnJob(FGN_1, 0, true))
      .then(stageKeyChange(1, false))
      .then(stageKeyChange(2, true))
      .then(closeRegEpoch)
      .expect(not(getIntSetOr(st.regTable, st.currentRegRoot).contains(1)))
      .then(acceptCollectionEpoch(0, true))
      .expect(invAll and invGraceRegRefused and not(invStaleRegRefused))
}

// ============================================================================
// 3. Mutant Attack + Honest-Behaviour Test Pairs (12 Mutants)
// ============================================================================

module t20_batched_mut_per_job_accept_run_test {
  import t20_batched_base(MUT = "per_job_accept_run", REG_MODE = "strict").* from "./t20_batched"

  // Real attack: 2 operator jobs in epoch 0 cause 2 SherlockLedger writes (> 1 write per busy epoch)
  run attackPerJobAcceptRunTest =
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(proveRunJob(RUN_B, 0, true))
      .expect(not(invAll) and not(invWritesBounded) and st.ledgerWrites == 2)

  // Honest use: single job + user run + consumer settlement consumption still execute cleanly
  run honestPerJobAcceptRunTest =
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(consumerUseSettledRun(1, RUN_A))
      .then(userAcceptRun(RUN_B))
      .then(consumerUseSettledRun(1, RUN_B))
      .expect(invAll and st.useLog.length() == 2)
}

module t20_batched_mut_per_claim_open_test {
  import t20_batched_base(MUT = "per_claim_open", REG_MODE = "strict").* from "./t20_batched"

  // Real attack: consuming a collection claim writes openClaim to SherlockLedger (2 writes for 1 epoch, 0 user runs)
  run attackPerClaimOpenTest = {
    val leaves = List(runClaimPub(RUN_A), runClaimPub(RUN_B))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(proveRunJob(RUN_B, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves, 0)))
      .expect(not(invAll) and not(invWritesBounded) and st.ledgerWrites == 2)
  }

  // Honest use: epoch settlement and user-settled run consumption satisfy invAll; collection consumption is accepted
  run honestPerClaimOpenTest =
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(userAcceptRun(RUN_B))
      .then(consumerUseSettledRun(1, RUN_B))
      .expect(invAll and st.ledgerWrites == 2 and st.useLog.length() == 1)
}

module t20_batched_mut_nullifier_keyed_by_root_test {
  import t20_batched_base(MUT = "nullifier_keyed_by_root", REG_MODE = "strict").* from "./t20_batched"

  // Real attack 1: same claim RUN_A included in two epochs' roots is consumed twice by consumer 1
  run attackNullifierKeyedByRootTwoRootsTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    val leaves1 = List(runClaimPub(RUN_A), runClaimPub(RUN_B))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(proveRunJob(RUN_A, 0, true))
      .then(proveRunJob(RUN_B, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves1, 0)))
      .expect(not(invAll) and not(invUseOnce))
  }

  // Real attack 2: same claim RUN_A in an accepted root and in runsMap is consumed twice by consumer 1
  run attackNullifierKeyedByRootAndRunsTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(userAcceptRun(RUN_A))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .then(consumerUseSettledRun(1, RUN_A))
      .expect(not(invAll) and not(invUseOnce))
  }

  // Honest use: single-root consumption by two consumers works, and same-root replay is still refused
  run honestNullifierKeyedByRootTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)).fail())
      .then(consumerUseRunFromRoot(2, RUN_A, treePath4(leaves0, 0)))
      .expect(invAll and st.useLog.length() == 2)
  }
}

module t20_batched_mut_no_root_read_test {
  import t20_batched_base(MUT = "no_root_read", REG_MODE = "strict").* from "./t20_batched"

  // Real attack 1 (Q2a): attacker consumes a forged claim with an unaccepted Merkle root
  run attackNoRootReadForgedTest =
    init
      .then(consumerUseRunFromRoot(1, FORGED_RUN_A, { sibs: List(0, 0), bits: List(false, false) }))
      .expect(not(invAll) and not(invUsedOnlyMembers) and not(invKindSeparated) and not(invNoUseBeforeClose))

  // Real attack 2 (Q2d): consumer uses a proved run from an open, unaccepted epoch before epoch close
  run attackNoRootReadBeforeCloseTest = {
    val provisional = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(provisional, 0)))
      .expect(not(invAll) and not(invNoUseBeforeClose) and not(invUsedOnlyMembers))
  }

  // Honest use: consuming an accepted root's claim after epoch close works and preserves invAll
  run honestNoRootReadTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)).fail())
      .expect(invAll and st.useLog.length() == 1)
  }
}

module t20_batched_mut_no_kind_tag_test {
  import t20_batched_base(MUT = "no_kind_tag", REG_MODE = "strict").* from "./t20_batched"

  // Real attack 1 (Q2c): run leaf `runClaimPub(RUN_A)` in an accepted tree is consumed as a foreign claim
  run attackNoKindTagRunAsForeignTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseForeignFromRoot(1, runClaimPub(RUN_A), treePath4(leaves0, 0)))
      .expect(not(invAll) and not(invUsedOnlyMembers) and not(invKindSeparated))
  }

  // Real attack 2 (Q2 empty slot 0): zero-padded empty slot 1 (`ZERO_FIELD = 0`) is consumed via useForeignClaim(0)!
  run attackNoKindTagEmptySlotZeroTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseForeignFromRoot(1, ZERO_FIELD, treePath4(leaves0, 1)))
      .expect(not(invAll) and not(invUsedOnlyMembers) and not(invKindSeparated))
  }

  // Honest use: honest run claims and honest foreign claims are both accepted and preserve invAll
  run honestNoKindTagTest = {
    val leaves0 = List(runClaimPub(RUN_A), fgnLeafOf("no_kind_tag", FGN_1.fpub))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(proveFgnJob(FGN_1, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .then(consumerUseForeignFromRoot(1, FGN_1.fpub, treePath4(leaves0, 1)))
      .expect(invAll and st.useLog.length() == 2)
  }
}

module t20_batched_mut_chunk_no_cap_test {
  import t20_batched_base(MUT = "chunk_no_cap", REG_MODE = "strict").* from "./t20_batched"

  // Real attack: oversized call (2,970 words -> 3 chunks) exceeds K_MAX = 2 and breaks the 10x TTL rule
  run attackChunkNoCapTest =
    init
      .then(submitCompactPlan(2970, DEFAULT_TTL_SEC))
      .expect(not(invAll) and not(invChunkCap))

  // Honest use: measured calls (<= 1,015) and 2-chunk fallback calls (1,016..1,980) work identically
  run honestChunkNoCapTest =
    init
      .then(submitCompactPlan(474, DEFAULT_TTL_SEC))
      .then(submitCompactPlan(1980, DEFAULT_TTL_SEC))
      .then(submitGeneralRunPlan(DEFAULT_TTL_SEC))
      .expect(invAll and seenChunkedCall)
}

module t20_batched_mut_chunk_truncate_test {
  import t20_batched_base(MUT = "chunk_truncate", REG_MODE = "strict").* from "./t20_batched"

  // Real attack: 2,970-word call silently truncates to 1,980 words instead of refusing at plan time
  run attackChunkTruncateTest =
    init
      .then(submitCompactPlan(2970, DEFAULT_TTL_SEC))
      .expect(not(invAll) and not(invNoTruncation))

  // Honest use: calls within cap (<= 1,980 words) are accepted without truncation
  run honestChunkTruncateTest =
    init
      .then(submitCompactPlan(474, DEFAULT_TTL_SEC))
      .then(submitCompactPlan(1980, DEFAULT_TTL_SEC))
      .expect(invAll and seenChunkedCall)
}

module t20_batched_mut_per_key_reg_write_test {
  import t20_batched_base(MUT = "per_key_reg_write", REG_MODE = "strict").* from "./t20_batched"

  // Real attack: 2 key additions in registry epoch 0 emit 2 on-chain reg_root writes in one epoch
  run attackPerKeyRegWriteTest =
    init
      .then(stageKeyChange(2, true))
      .then(stageKeyChange(3, true))
      .expect(not(invAll) and not(invRegWritesPerEpoch) and not(invWritesBounded))

  // Honest use: 1 key addition in registry epoch 0 + foreign collection under new root works cleanly
  run honestPerKeyRegWriteTest =
    init
      .then(stageKeyChange(2, true))
      .then(closeRegEpoch)
      .then(proveFgnJob(FGN_2, 0, true))
      .then(acceptCollectionEpoch(1, true))
      .expect(invAll and seenCollectionUnderNewReg)
}

module t20_batched_mut_no_reg_check_test {
  import t20_batched_base(MUT = "no_reg_check", REG_MODE = "strict").* from "./t20_batched"

  // Real attack: after registry advances to root 1, acceptCollection accepts stale root 0 or foreign root 99
  run attackNoRegCheckTest =
    init
      .then(stageKeyChange(2, true))
      .then(closeRegEpoch)
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(99, true))
      .expect(not(invAll) and not(invStaleRegRefused))

  // Honest use: accepting a collection under the current registry root 0 works and preserves invAll
  run honestNoRegCheckTest =
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(proveFgnJob(FGN_1, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .expect(invAll and st.acceptedColls.length() == 1)
}

module t20_batched_mut_descend_k_open_test {
  import t20_batched_base(MUT = "descend_k_open", REG_MODE = "strict").* from "./t20_batched"

  // Real attack: opening a depth-3 DAG claim spends ceil(3 / 2) = 2 extra descend writes on SherlockLedger
  run attackDescendKOpenTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 3, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .expect(not(invAll) and not(invNoDescendCalls) and not(invWritesBounded) and st.descendWrites == 2)
  }

  // Honest use: depth-0 leaf claims require 0 descend steps and preserve invAll
  run honestDescendKOpenTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .expect(invAll and st.useLog.length() == 1 and st.descendWrites == 0)
  }
}

module t20_batched_mut_nullifier_keyed_by_pos_test {
  import t20_batched_base(MUT = "nullifier_keyed_by_pos", REG_MODE = "strict").* from "./t20_batched"

  // Real attack (Q2b-iii): diamond-duplicate claim at slots 0 and 1 of ONE root is used twice by consumer 1
  run attackNullifierKeyedByPosTest = {
    val dupLeaves = List(runClaimPub(RUN_A), runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 1, true))
      .then(proveRunJob(RUN_A, 1, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(dupLeaves, 0)))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(dupLeaves, 1)))
      .expect(not(invAll) and not(invUseOnce))
  }

  // Honest use: distinct claims at slots 0 and 1 work, and replay on the same slot 0 is still refused
  run honestNullifierKeyedByPosTest = {
    val distinctLeaves = List(runClaimPub(RUN_A), runClaimPub(RUN_B))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(proveRunJob(RUN_B, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(distinctLeaves, 0)))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(distinctLeaves, 0)).fail())
      .then(consumerUseRunFromRoot(1, RUN_B, treePath4(distinctLeaves, 1)))
      .expect(invAll and st.useLog.length() == 2)
  }
}

module t20_batched_mut_unbound_claims_root_test {
  import t20_batched_base(MUT = "unbound_claims_root", REG_MODE = "strict").* from "./t20_batched"

  // Real attack (T8b / W-2 circuit delta): unconstrained L emits an ADM seal premise into claims_root,
  // allowing a Compact consumer to discharge a non-run premise as if it were a proved run!
  run attackUnboundClaimsRootTest = {
    val badLeaves = List(runClaimPub(ADM_PREMISE_SPEC))
    init
      .then(injectUnboundLeaf(ADM_PREMISE_SPEC))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, ADM_PREMISE_SPEC, treePath4(badLeaves, 0)))
      .expect(not(invAll) and not(invUsedOnlyMembers) and not(invKindSeparated))
  }

  // Honest use: when L only emits proved run/foreign claims, honest consumption succeeds and preserves invAll
  run honestUnboundClaimsRootTest = {
    val leaves0 = List(runClaimPub(RUN_A))
    init
      .then(proveRunJob(RUN_A, 0, true))
      .then(acceptCollectionEpoch(0, true))
      .then(consumerUseRunFromRoot(1, RUN_A, treePath4(leaves0, 0)))
      .expect(invAll and st.useLog.length() == 1)
  }
}
