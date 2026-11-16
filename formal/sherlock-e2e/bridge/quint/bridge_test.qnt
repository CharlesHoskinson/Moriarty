// Deterministic unit and scenario tests for Sherlock E2E Prototype — Bounded Context: Bridge
// Lane: e2e-bridge

module bridge_test {
  import bridge(
    L_MAX_COLS               = 2200,
    L_MAX_INTERACTIONS       = 100,
    L_FP3_COLS               = 2071,
    IFACE_PUBLIC_WORDS       = 12166,
    IFACE_LANE_PAIRS         = 5,
    ANCHOR_SLOT              = 15,
    K_MAX                    = 2,
    LFR_CONFIG_SLOT          = 15,
    WRAP_DECODER_CONFIG_SLOT = 15,
    SDK_LEDGER_CONFIG_SLOT   = 15,
    MUTANT                   = "none"
  ).* from "./bridge"

  // LTR-003, CLB-008, LW-022: Rekey slot-agreement check, L key column/interaction bounds, and WHIR shape check
  run br_LTR_003_CLB_008_LW_022_RekeyKeyBoundsAndWhirShapeTest =
    init
      .then(refuseRekeySlotMismatch)
      .then(refuseLKeyOverBounds(3))
      .then(refuseLKeyOverBounds(4))
      .then(refuseInvalidWhirShape)
      .then(startRekeyAndAdmitKeys(1))
      .expect(
        rekeyState.started
          and rekeyState.lfrSlotCfg == 15
          and rekeyState.wrapDecoderSlotCfg == 15
          and rekeyState.lKeyCols == 2071
          and rekeyState.lKeyMaxInteractions == 99
          and rekeyState.whirShapeChecked
          and rekeyState.whirLogRs == 5
          and rekeyState.whirFoldingK == 4
          and refusalLog.contains({ code: RefRekeySlotMismatch, targetId: 80 })
          and refusalLog.contains({ code: RefLKeyBounds, targetId: 3 })
          and refusalLog.contains({ code: RefLKeyBounds, targetId: 4 })
          and refusalLog.contains({ code: RefWhirShapeLogRsLtK, targetId: 2 })
          and inv_LTR_003
          and inv_CLB_008
          and inv_LW_022
          and inv
      )
      .then(startRekeyAndAdmitKeys(3).fail())

  // LTR-003: Over-bounds L key (2,201 columns > 2,200) is blocked from admitting even before rekey starts
  run br_LTR_003_OverBoundsKeyBlockedFromStartTest =
    init
      .expect(not(rekeyState.started) and inv)
      .then(startRekeyAndAdmitKeys(3).fail())

  // ANC-014, ANC-015: SherlockLedger stores C_P in slot 15 (keeping slot 0 user data); Mycroft refuses 16-field contract
  run br_ANC_014_015_ContractDeploySlot15AndCompilerRefusalTest =
    init
      .then(refuseCollidingSlot15Contract)
      .then(compileAndDeployContract(1))
      .expect(
        compiledContracts.keys().contains(1)
          and not(compiledContracts.keys().contains(2))
          and compiledContracts.get(1).ledgerAnchorSlot == 15
          and compiledContracts.get(1).slot15ValueCp == 3001
          and compiledContracts.get(1).slot0IsUserField
          and not(compiledContracts.get(1).usesSlot15)
          and refusalLog.contains({ code: RefCompilerSlot15Collision, targetId: 2 })
          and inv_ANC_014
          and inv_ANC_015
          and inv
      )
      .then(compileAndDeployContract(2).fail())

  // CLB-007: Chunk planner schedules 1- and 2-chunk calls and refuses 3-chunk call (> K_MAX = 2) before proving starts
  run br_CLB_007_ChunkPlannerKMax2AndRefusalTest =
    init
      .then(compileAndDeployContract(1))
      .then(planCallChunks(1))
      .then(planCallChunks(3))
      .then(refuseCallOverKMax(5))
      .expect(
        chunkPlans.keys().contains(1)
          and chunkPlans.keys().contains(3)
          and not(chunkPlans.keys().contains(5))
          and chunkPlans.get(1).chunkCount == 1
          and chunkPlans.get(3).chunkCount == 2
          and refusalLog.contains({ code: RefExceedsKMax, targetId: 5 })
          and inv_CLB_007
          and inv
      )
      .then(planCallChunks(5).fail())

  // LTR-001, LTR-002, LTR-004, LTR-005, LTR-006, LTR-008, CLB-001, CLB-004, LW-020, LW-021, ANC-013, ANC-016:
  // Full step-by-step ordinary 3-leaf call bridge from L through L_Fr and muninn transfer to Wrap
  run br_LTR_001_002_004_005_006_008_Ordinary3LeafFullChainTest =
    init
      .then(startRekeyAndAdmitKeys(1))
      .then(compileAndDeployContract(1))
      .then(planCallChunks(2))
      .then(proveLCallOrChunk(2))
      .expect(
        lProofs.get(2).baseField == FieldBabyBear
          and lProofs.get(2).hashConfig == HashMidnightPoseidonFr
          and lProofs.get(2).publicWordsCount == 12166
          and lProofs.get(2).laneMapPairs == 5
          and lProofs.get(2).stockVerified
          and inv_LTR_001
          and inv_CLB_001
      )
      .then(refuseTapeWithoutExpose)
      .then(refuseIllTypedTapeInMachineNew)
      .then(buildLfrMachine(2))
      .expect(
        lfrMachines.get(2).usedTapeExpose
          and lfrMachines.get(2).operandsWellTyped
          and lfrMachines.get(2).replayedChallengesD == lProofs.get(2).stockChallengesD
          and inv_LTR_004
          and inv_LW_021
      )
      .then(refuseTamperedTranscriptChallenge)
      .then(refuseMemBusTypeMismatchInCircuit)
      .then(proveAndVerifyLfr(2))
      .expect(
        lfrProofs.get(2).baseField == FieldBls12381Fr
          and lfrProofs.get(2).extDegree == 1
          and lfrProofs.get(2).memBusSixthElem == TyFr
          and lfrProofs.get(2).memBusBalanced
          and lfrProofs.get(2).anchorReadSlot == 15
          and not(lfrProofs.get(2).writesAnchor)
          and lfrProofs.get(2).slot0PreservedAsUserWord
          and lfrProofs.get(2).transcriptReplayedOutD == lfrProofs.get(2).stockTranscriptOutD
          and inv_LTR_002
          and inv_LTR_005
          and inv_CLB_004
          and inv_LW_020
      )
      .then(refuseForgedAnchorLfrNoRecord)
      .then(publishLfrReady(2))
      .expect(
        readyRecords.keys().contains(2)
          and not(readyRecords.keys().contains(89))
          and not(readyRecords.keys().contains(99))
          and readyRecords.get(2).proofSha256 == lfrProofs.get(2).proofSha256
          and readyRecords.get(2).lfrVkPrehash == UNIV_LFR_VK_PREHASH
          and readyRecords.get(2).publicWordsCount == 12166
          and readyRecords.get(2).claimPub == 7002
          and refusalLog.contains({ code: RefStockLfrForgedAnchor, targetId: 89 })
          and inv_LTR_008
      )
      .then(refuseCorruptedProofTransfer)
      .then(refuseWrapWrongSlotPin)
      .then(refuseMode0AnchorTemplateInWrap)
      .then(verifyTransferAndWrap(2))
      .expect(
        wrapAcceptances.keys().contains(2)
          and wrapAcceptances.get(2).lane6ReadKeyConst == 15
          and wrapAcceptances.get(2).lane15ChunkWriteKeyConst == 15
          and not(wrapAcceptances.get(2).carriesMode0AnchorWriteTemplate)
          and wrapAcceptances.get(2).transferSha256Matched
          and refusalLog.contains({ code: RefCorruptedProofTransfer, targetId: 90 })
          and refusalLog.contains({ code: RefWrapWrongSlotPin, targetId: 91 })
          and refusalLog.contains({ code: RefMode0AnchorTemplateWrap, targetId: 92 })
          and inv_LTR_006
          and inv_ANC_013
          and inv_ANC_016
          and inv
      )

  // LTR-007: 1-leaf Path C run (Call 1) and 3-leaf Full Tree run (Call 2) verify under the exact same L and L_Fr keys
  run br_LTR_007_PathCAndFullTreeSameUniversalKeysTest =
    init
      .then(bridgeCallEndToEnd(1))
      .then(bridgeCallEndToEnd(2))
      .expect(
        lProofs.get(1).treeForm == TreePathC
          and lProofs.get(1).leafCount == 1
          and lProofs.get(2).treeForm == TreeFull
          and lProofs.get(2).leafCount == 3
          and lProofs.get(1).lKeySha256 == lProofs.get(2).lKeySha256
          and lfrProofs.get(1).lfrKeySha256 == lfrProofs.get(2).lfrKeySha256
          and wrapAcceptances.keys().contains(1)
          and wrapAcceptances.keys().contains(2)
          and inv_LTR_007
          and inv
      )

  // CLB-002, CLB-003, CLB-006, ANC-016, ANC-018, ANC-019: L EffectAir, iface decoder, and Wrap refusals
  run br_CLB_002_003_006_ANC_016_018_019_LAndWrapRefusalsTest =
    init
      .then(refuseShiftedEffectLaneWrite)
      .then(refuseWrongChunkCountInL)
      .then(refuseMode0AnchorTemplateOnIface)
      .then(refuseMode0AnchorTemplateInWrap)
      .expect(
        refusalLog.contains({ code: RefShiftedEffectLane, targetId: 81 })
          and refusalLog.contains({ code: RefWrongChunkCount, targetId: 82 })
          and refusalLog.contains({ code: RefMode0AnchorTemplateIface, targetId: 83 })
          and refusalLog.contains({ code: RefMode0AnchorTemplateWrap, targetId: 92 })
          and inv_CLB_002
          and inv_CLB_003
          and inv_CLB_006
          and inv_ANC_016
          and inv_ANC_018
          and inv_ANC_019
          and inv
      )

  // CLB-005, ANC-017: 2-chunk Call 3 links Chunk 0 -> Chunk 1 via 7-operand lock; Call 4 sharing runTag = 444 produces distinct lock
  run br_CLB_005_ANC_017_TwoChunkLinkedAndEqualRunTagDistinctLocksTest =
    init
      .then(startRekeyAndAdmitKeys(1))
      .then(compileAndDeployContract(1))
      .then(planCallChunks(3))
      .then(planCallChunks(4))
      .then(proveLCallOrChunk(3))
      .then(buildLfrMachine(3))
      .then(proveAndVerifyLfr(3))
      .then(publishLfrReady(3))
      .then(verifyTransferAndWrap(3))
      .then(proveLCallOrChunk(4))
      .then(buildLfrMachine(4))
      .then(proveAndVerifyLfr(4))
      .then(publishLfrReady(4))
      .then(verifyTransferAndWrap(4))
      .then(proveLCallOrChunk(5))
      .then(buildLfrMachine(5))
      .then(proveAndVerifyLfr(5))
      .then(refuseSplicedOrAlteredChunkLock)
      .expect(
        not(lfrProofs.get(3).readsPriorChunkLock)
          and lfrProofs.get(3).writesNextChunkLock
          and lfrProofs.get(4).readsPriorChunkLock
          and not(lfrProofs.get(4).writesNextChunkLock)
          and lfrProofs.get(4).readLockOp == lfrProofs.get(3).writeLockOp
          and lfrProofs.get(3).writeLockOp != lfrProofs.get(5).writeLockOp
          and lfrProofs.get(3).writesAnchor
          and lfrProofs.get(3).anchorWriteSlot == 15
          and lfrProofs.get(4).writesAnchor
          and lfrProofs.get(4).anchorWriteSlot == 15
          and wrapAcceptances.keys().contains(3)
          and wrapAcceptances.keys().contains(4)
          and refusalLog.contains({ code: RefChunkLockMismatch, targetId: 88 })
          and inv_CLB_004
          and inv_CLB_005
          and inv_ANC_017
          and inv
      )

  // CLB-005, ANC-017: Chunk 1 (pSpecId = 4) cannot prove in L_Fr before Chunk 0 (pSpecId = 3) writes its lock
  run br_ChunkOrderDependencyBlockedTest =
    init
      .then(startRekeyAndAdmitKeys(1))
      .then(compileAndDeployContract(1))
      .then(planCallChunks(3))
      .then(proveLCallOrChunk(4))
      .then(buildLfrMachine(4))
      .expect(not(lfrProofs.keys().contains(3)) and inv)
      .then(proveAndVerifyLfr(4).fail())

  // Pure consistency checks witnessing findings Q-bridge-1 through Q-bridge-4
  run br_ConsistencyAndSpecContradictionsTest =
    init
      .expect(
        inv
          and not(consistency_CLB_001_post_rekey_nonzero_delta)
          and not(consistency_ANC_017_CLB_005_literal_lock_read_write)
          and not(consistency_mode0_slot15_outside_template_blocked)
          and not(consistency_CLB_008_catches_zero_and_sdk_drift)
      )
}

// Edge test 1: Smallest legal chunk cap K_MAX = 1 (1-slice Call 1 wraps; 2-slice Call 3 is refused at planner)
module bridge_edge_min_k1_test {
  import bridge(
    L_MAX_COLS               = 2200,
    L_MAX_INTERACTIONS       = 100,
    L_FP3_COLS               = 1995,
    IFACE_PUBLIC_WORDS       = 12166,
    IFACE_LANE_PAIRS         = 5,
    ANCHOR_SLOT              = 15,
    K_MAX                    = 1,
    LFR_CONFIG_SLOT          = 15,
    WRAP_DECODER_CONFIG_SLOT = 15,
    SDK_LEDGER_CONFIG_SLOT   = 15,
    MUTANT                   = "none"
  ).* from "./bridge"

  run br_EdgeMinK1_SingleSliceWrapsAndTwoSliceRefusedTest =
    init
      .then(bridgeCallEndToEnd(1))
      .then(refuseCallOverKMax(3))
      .expect(
        wrapAcceptances.keys().contains(1)
          and not(chunkPlans.keys().contains(3))
          and refusalLog.contains({ code: RefExceedsKMax, targetId: 3 })
          and inv_CLB_007
          and inv
      )
      .then(planCallChunks(3).fail())
}

// Edge test 2: L key width and interactions exactly at limits (2,200 columns, 100 interactions)
module bridge_edge_exact_bounds_test {
  import bridge(
    L_MAX_COLS               = 2200,
    L_MAX_INTERACTIONS       = 100,
    L_FP3_COLS               = 2200,
    IFACE_PUBLIC_WORDS       = 12166,
    IFACE_LANE_PAIRS         = 5,
    ANCHOR_SLOT              = 15,
    K_MAX                    = 2,
    LFR_CONFIG_SLOT          = 15,
    WRAP_DECODER_CONFIG_SLOT = 15,
    SDK_LEDGER_CONFIG_SLOT   = 15,
    MUTANT                   = "none"
  ).* from "./bridge"

  run br_EdgeExactBounds_AtLimitKeyAdmittedAndOverLimitRefusedTest =
    init
      .then(startRekeyAndAdmitKeys(1))
      .then(refuseLKeyOverBounds(3))
      .then(refuseLKeyOverBounds(4))
      .expect(
        rekeyState.started
          and rekeyState.lKeyCols == 2200
          and rekeyState.lKeyMaxInteractions == 100
          and refusalLog.contains({ code: RefLKeyBounds, targetId: 3 })
          and refusalLog.contains({ code: RefLKeyBounds, targetId: 4 })
          and inv_LTR_003
          and inv
      )
      .then(startRekeyAndAdmitKeys(3).fail())
}
