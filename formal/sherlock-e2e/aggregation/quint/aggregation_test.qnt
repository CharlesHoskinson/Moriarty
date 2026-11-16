// Deterministic requirement and scenario tests for Sherlock E2E Aggregation
// Lane: e2e-aggregation

module aggregation_test {
  import aggregation(
    MERGE_WORK_MAX_COLS     = 5400,
    MW_BASE_COLS            = 4988,
    MW_VALUE_DIGEST_DELTA   = 62,
    MW_STACKS_DELTA         = 71,
    KEYS_D_NODE_ROWS        = 7,
    KEYS_D_FOREIGN_ROWS     = 2,
    GP2_CASE_FLOOR          = 48,
    X7_CASE_FLOOR           = 22,
    CLAIMS_TREE_DEPTH       = 20,
    HAS_X8_COMPACT_VERIFIER = true,
    HAS_L_COLLECTION_KEY    = true,
    MUTANT                  = "none"
  ).* from "./aggregation"

  // NAG-001, NAG-002, NAG-003: N1 -> N2 -> N3 over BabyBear INTERNAL key, prog_d opening, and AIR-omission / wrong-key_root refusals
  run ag_NAG_001_002_003_NodeChainKeyRootAndOmitAirTest =
    init
      .then(proveN1(1))
      .then(proveN1(2))
      .then(proveN2Merge(1))
      .then(proveN3Root(1))
      .then(refuseMissingChildAir)
      .then(refuseN1WrongKeyRoot)
      .expect(
        inv
          and lastOp == "refuseN1WrongKeyRoot"
          and nodeProofs.keys().contains(11)
          and nodeProofs.keys().contains(12)
          and nodeProofs.keys().contains(21)
          and nodeProofs.keys().contains(31)
          and nodeProofs.get(11).starkField == FieldBabyBear
          and nodeProofs.get(11).paramSet == ParamInternal
          and nodeProofs.get(11).verifiedVk == UNIV_INTERNAL_REC_VK
          and nodeProofs.get(11).forwardedKeyRoot == 1001
          and nodeProofs.get(31).outcomeD == 501
          and refusalLog.contains({ code: RefMissingRequiredAir, targetId: 91 })
          and refusalLog.contains({ code: RefWrongProgDOpening, targetId: 92 })
      )

  // NAG-004: Leaves of one run using chip set 0 and chip set 3 under one key_root merge under N2 and N3
  run ag_NAG_004_MixedChipSetsMergedTest =
    init
      .then(proveN1(5))
      .then(proveN1(6))
      .then(proveN2Merge(3))
      .then(proveN3Root(3))
      .expect(
        inv
          and lastOp == "proveN3Root"
          and nodeProofs.get(23).chipSetsMerged == Set(0, 3)
          and nodeProofs.get(33).chipSetsMerged == Set(0, 3)
          and nodeProofs.get(23).forwardedKeyRoot == 1003
          and nodeProofs.get(33).forwardedKeyRoot == 1003
          and nodeProofs.get(33).stockVerified
      )

  // NAG-005, LPC-002: MERGE-WORK keys at 5,121 and 5,400 columns admitted; 5,401 and 5,417 refused and blocked
  run ag_NAG_005_LPC_002_MergeWorkColumnBoundTest =
    init
      .then(admitMergeWorkKey(1))
      .then(admitMergeWorkKey(2))
      .then(refuseMergeWorkKeyOverCols(3))
      .then(refuseMergeWorkKeyOverCols(4))
      .expect(
        inv
          and mwKeyRegistry.keys() == Set(1, 2)
          and mwKeyRegistry.get(1).colCount == 5121
          and mwKeyRegistry.get(2).colCount == 5400
          and refusalLog.contains({ code: RefMergeWorkCols, targetId: 3 })
          and refusalLog.contains({ code: RefMergeWorkCols, targetId: 4 })
      )
      .then(admitMergeWorkKey(3).fail())

  // NAG-006, NAG-007, NAG-011: Honest nonzero reg_root (AFK-F6) and linked 1-child MERGE-WORK (AFK-F9(b)) accepted; T10q and GP2-C2.1 refused
  run ag_NAG_006_007_011_T10qForgeriesGp2C21AndHonestInputsTest =
    init
      .then(admitMergeWorkKey(1))
      .then(proveN1(1))
      .then(proveOneChildMergeWork)
      .then(refuseT10qForgery(ForgeryMwF01FalseCarry))
      .then(refuseT10qForgery(ForgeryAfkF7ShapeCollision))
      .then(refuseT10qForgery(ForgeryGp2F42UnboundType))
      .then(refuseT10qForgery(ForgeryAfkF9bUnlinkedOneChild))
      .then(refuseGp2C21Forgery)
      .expect(
        inv
          and nodeProofs.get(11).regRoot == VALID_REG_ROOT
          and nodeProofs.get(70).oneChildMode
          and nodeProofs.get(70).oneChildLinked
          and refusalLog.contains({ code: RefT10qMwF01, targetId: 93 })
          and refusalLog.contains({ code: RefT10qAfkF7, targetId: 93 })
          and refusalLog.contains({ code: RefT10qGp2F42, targetId: 93 })
          and refusalLog.contains({ code: RefT10qAfkF9b, targetId: 93 })
          and refusalLog.contains({ code: RefGp2C21ConstRow, targetId: 94 })
      )

  // NAG-008, NAG-009, NAG-010: Child linking, L outcome binding, swapped-child & wrong-outcome refusals, and complete ProofBundle
  run ag_NAG_008_009_010_LOutcomeChildLinksAndProofBundleTest =
    init
      .then(proveRunNodeTree(1))
      .then(refuseDisjointOrSwappedChildren)
      .then(refuseLWrongOutcome)
      .then(proveLRun(1))
      .then(publishProofBundle(1))
      .expect(
        inv
          and lastOp == "publishProofBundle"
          and nodeProofs.get(21).hStart == 0
          and nodeProofs.get(21).hEnd == 12
          and nodeProofs.get(21).intervalsContiguous
          and nodeProofs.get(21).stateDigestsLinked
          and lRunProofs.get(1).lOutcomeD == 501
          and proofBundles.get(1).hasLeafEntries
          and proofBundles.get(1).hasLAndLFrEntries
          and proofBundles.get(1).nodeEntries.length() == 4
          and proofBundles.get(1).claimPub == lRunProofs.get(1).claimPub
          and proofBundles.get(1).keyManifestSha256 == UNIV_MANIFEST_SHA256
          and refusalLog.contains({ code: RefDisjointOrSwappedChildren, targetId: 21 })
          and refusalLog.contains({ code: RefWrongOutcomeAtL, targetId: 1 })
      )

  // NAG-010 negative check: publishing a ProofBundle before L proves the run is blocked
  run ag_NAG_010_PrematureBundleBlockedTest =
    init
      .then(proveN1(10))
      .expect(
        inv
          and nodeProofs.keys().contains(20)
          and not(nodeProofs.get(20).isTerminal)
      )
      .then(publishProofBundle(1).fail())

  // GPH-001: Interpreted VM and R_SL_* programs prove N1, N2, N3, L, and L_Fr under identical universal keys
  run ag_GPH_001_BothRuleFamiliesSameUniversalKeysTest =
    init
      .then(proveRunNodeTree(1))
      .then(proveLRun(1))
      .then(proveRunNodeTree(2))
      .then(proveLRun(2))
      .expect(
        inv
          and lRunProofs.get(1).family == FamInterpreted
          and lRunProofs.get(2).family == FamRsl
          and lRunProofs.get(1).verifiedLVk == UNIV_L_VK
          and lRunProofs.get(2).verifiedLVk == UNIV_L_VK
          and lRunProofs.get(1).verifiedLFrVk == UNIV_LFR_VK
          and lRunProofs.get(2).verifiedLFrVk == UNIV_LFR_VK
          and universalKeys == genesisUniversalKeys
      )

  // GPH-002, GP-002: Cross-run claim consumption via SEAL, SEAL-WITH-DEPS, DL-STEP/DESCEND; forged dep & mixed reg_root refused
  run ag_GPH_002_GP_002_CrossRunConsumeAndForgedDepRefusedTest =
    init
      .then(sealRunNodeTree(1))
      .then(sealRunNodeTree(5))
      .then(refuseGp2ForgedDep)
      .then(refuseMixedRegRootJoin)
      .then(proveGp2DischargeDeps(41))
      .expect(
        inv
          and nodeProofs.get(41).gp2Mode == ModeSeal
          and nodeProofs.get(45).gp2Mode == ModeSealWithDeps
          and nodeProofs.get(45).pendingDeps.size() == 1
          and nodeProofs.get(55).gp2Mode == ModeDlStepDescend
          and nodeProofs.get(55).pendingDeps.size() == 0
          and nodeProofs.get(55).claimSet.size() == 2
          and nodeProofs.get(55).depsVerifiedInSet
          and refusalLog.contains({ code: RefUnprovedOrUnsealedDep, targetId: 55 })
          and refusalLog.contains({ code: RefMixedRegRootJoin, targetId: 59 })
      )

  // GPH-002, GP-002 negative check: discharging pendingDeps against a SET that lacks the dependency is blocked
  run ag_GPH_002_UnprovedDepDischargeBlockedTest =
    init
      .then(sealRunNodeTree(2))
      .then(sealRunNodeTree(5))
      .expect(
        inv
          and nodeProofs.keys().contains(42)
          and nodeProofs.keys().contains(45)
      )
      .then(proveGp2DischargeDeps(42).fail())

  // GPH-003, GPH-004, GP-003: N3 accepts foreign units in keys_d, X8 adapts registered Compact proof, unlisted FV key refused & blocked
  run ag_GPH_003_004_GP_003_ForeignUnitAndCompactAdapterTest =
    init
      .then(proveN3ForeignUnit(1))
      .then(proveN3ForeignUnit(2))
      .then(adaptCompactForeignProof)
      .then(refuseUnlistedFvUnit)
      .then(refuseUnboundCompactAdapter)
      .expect(
        inv
          and nodeProofs.get(61).stmtKind == StmtFv
          and nodeProofs.get(61).tier == TierN3
          and nodeProofs.get(62).stmtKind == StmtFv
          and nodeProofs.get(65).adapterClass == AdapterCompactZkir3
          and nodeProofs.get(65).compactVkId == REGISTERED_COMPACT_VK
          and nodeProofs.get(65).adapterBoundInCircuit
          and refusalLog.contains({ code: RefUnlistedForeignKey, targetId: 99 })
          and refusalLog.contains({ code: RefUnboundCompactAdapter, targetId: 799 })
      )
      .then(proveN3ForeignUnit(3).fail())

  // GPH-005, GP-004: Balanced tree and chain over same MERGE-WORK units verify under universal keys and give equal claimD
  run ag_GPH_005_GP_004_TwoPcdTreeShapesSameClaimTest =
    init
      .then(admitMergeWorkKey(1))
      .then(provePcdWorkTree(1))
      .then(provePcdWorkTree(2))
      .expect(
        inv
          and nodeProofs.get(71).treeShape == ShapeBalanced
          and nodeProofs.get(72).treeShape == ShapeChain
          and nodeProofs.get(71).claimD == nodeProofs.get(72).claimD
          and nodeProofs.get(71).shapeD != nodeProofs.get(72).shapeD
          and not(nodeProofs.get(71).shapeUsesPhysState)
          and not(nodeProofs.get(72).shapeUsesPhysState)
      )

  // GPH-006: Suspended-and-resumed run across machines produces identical claim_pub to uninterrupted run
  run ag_GPH_006_SuspendResumeEqualClaimPubTest =
    init
      .then(proveRunNodeTree(1))
      .then(proveLRun(1))
      .then(proveRunNodeTree(4))
      .then(proveLRun(4))
      .expect(
        inv
          and not(lRunProofs.get(1).suspendedAndResumed)
          and lRunProofs.get(4).suspendedAndResumed
          and lRunProofs.get(1).claimPub == lRunProofs.get(4).claimPub
      )

  // GPH-007: Honest GP gate run passes when case map matches baseline and floors (48, 22) hold
  run ag_GPH_007_HonestGpGatePassTest =
    init
      .then(runGpGateHonest)
      .expect(
        inv
          and gateStatus.verdict == GatePassed
          and gateStatus.gp2Count == 48
          and gateStatus.x7Count == 22
          and gateStatus.caseMapMatchesBaseline
      )

  // GPH-007 Scenario: Refusal case replaced by copy of accept case (count stays 48) fails gate on case map
  run ag_GPH_007_TamperedCaseMapFailsGateTest =
    init
      .then(runGpGateTamperedCaseMap)
      .expect(
        inv
          and gateStatus.verdict == GateFailedCaseMap
          and gateStatus.gp2Count == 48
          and not(gateStatus.caseMapMatchesBaseline)
      )

  // DG-010, CPR-005: Failed derived gate leaves GP targets open and never narrows GP-001..GP-004 or GP-ZK
  run ag_DG_010_CPR_005_FailedDerivedGateKeepsGpOpenTest =
    init
      .then(runGpGateFailedDerived)
      .expect(
        inv
          and gateStatus.verdict == GateFailedDerived
          and not(gateStatus.gpTargetsNarrowed)
      )

  // GPH-008, LPC-003: Collection form of L over SET root of 3 claims of 3 programs + forged SET root refusal
  run ag_GPH_008_LPC_003_LCollectionThreeClaimsAndForgedSetRefusedTest =
    init
      .then(sealRunNodeTree(1))
      .then(sealRunNodeTree(2))
      .then(sealRunNodeTree(3))
      .then(proveGp2JoinSet(1))
      .then(proveGp2JoinSet(2))
      .then(proveLCollectionOverSet(52))
      .then(refuseLCollectionForgedSetRoot)
      .expect(
        inv
          and lCollectionProofs.keys().contains(52)
          and lCollectionProofs.get(52).memberClaims.size() == 3
          and lCollectionProofs.get(52).exposedClaimsRoot == nodeProofs.get(52).claimsRoot
          and lCollectionProofs.get(52).allMembershipsVerified
          and lCollectionProofs.get(52).verifiedLVk == UNIV_L_VK
          and refusalLog.contains({ code: RefForgedSetRootAtL, targetId: 88 })
      )

  // LPC-001 consistency check and documented spec contradictions Q-aggregation-1 and Q-aggregation-2
  run ag_ConsistencyAndSpecContradictionsTest =
    init
      .expect(
        inv
          and consistency_LPC_001
          and not(consistency_LPC_002_scenario_vs_NAG_005)
          and not(consistency_GPH_003_vs_GPH_004_consumption)
      )
}

module aggregation_edge_t25_test {
  import aggregation(
    MERGE_WORK_MAX_COLS     = 5121,
    MW_BASE_COLS            = 4988,
    MW_VALUE_DIGEST_DELTA   = 62,
    MW_STACKS_DELTA         = 71,
    KEYS_D_NODE_ROWS        = 7,
    KEYS_D_FOREIGN_ROWS     = 1,
    GP2_CASE_FLOOR          = 4,
    X7_CASE_FLOOR           = 2,
    CLAIMS_TREE_DEPTH       = 2,
    HAS_X8_COMPACT_VERIFIER = false,
    HAS_L_COLLECTION_KEY    = false,
    MUTANT                  = "none"
  ).* from "./aggregation"

  // Edge instance: 5,121 exact cap admitted, 5,122 refused, unbuilt X8 maps to FREEZE-PENDING and blocks adaptation
  run ag_EdgeT25_Exact5121AdmittedAndUnbuiltX8BlockedTest =
    init
      .then(admitMergeWorkKey(1))
      .then(refuseMergeWorkKeyOverCols(3))
      .expect(
        inv
          and mwKeyRegistry.get(1).colCount == 5121
          and refusalLog.contains({ code: RefMergeWorkCols, targetId: 3 })
          and consistency_LPC_001
          and hookRowStatus(HAS_X8_COMPACT_VERIFIER) == HookFreezePending
      )
      .then(adaptCompactForeignProof.fail())

  // Edge instance: unkeyed L collection form maps to FREEZE-PENDING and blocks proveLCollectionOverSet
  run ag_EdgeT25_LCollectionBlockedWhenUnkeyedTest =
    init
      .then(sealRunNodeTree(1))
      .expect(
        inv
          and nodeProofs.keys().contains(41)
          and hookRowStatus(HAS_L_COLLECTION_KEY) == HookFreezePending
      )
      .then(proveLCollectionOverSet(41).fail())
}
