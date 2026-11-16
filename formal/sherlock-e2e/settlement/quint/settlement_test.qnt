// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Settlement (lane e2e-settlement).

module settlement_test {
  import settlement(
    CONTRACT_ENTRY_COUNT = 4,
    ANCHOR_SLOT = 15,
    STATEMENT_PUBLIC_WORDS = 4,
    RUN_SALT_WORDS = 8,
    EPOCH_WINDOW_S = 3600,
    MAX_CLAIMS_PER_EPOCH = 1048576,
    K_MAX_CHUNKS = 2,
    DEFAULT_INTENT_TTL_S = 3600,
    CLAIMS_TREE_DEPTH = 20,
    CLAIM_LEAF_BYTES = 32,
    COLLECTION_PROOF_TIME_S = 176,
    INCLUSION_TIME_S = 60,
    BLOCK_INTERVAL_S = 6,
    MUTANT_ID = 0
  ).* from "./settlement"

  // SLC-001..005, STL-003:
  // Prototype ledger exports 4 entries (no openClaim, no descend), seals c_p in slot 15,
  // installs universal wrap VK on write entries and native keys on read entries;
  // Preview T25 redeploy waits on rekey + T12 and stops on readback VK mismatch.
  run st_deployEntriesSlot15AndT25ReadbackTest =
    init
      .expect(
        inv and
        lastOp == "init" and
        not(canDeployLedger(stl, 1, true, true))
      )
      .then(doPassRekeyAndT12)
      .expect(not(canDeployLedger(stl, 1, true, false)))
      .then(doRefuseInstalledVkReadbackMismatch)
      .expect(inv and stl.lastRefusal == "installed_vk_readback_mismatch")
      .then(doDeployProgramLedger(1, true, true, false))
      .expect(
        inv and
        stl.ledgers.get(1).exportedEntries == Set("acceptRun", "acceptCollection", "rootAccepted", "runAccepted") and
        stl.ledgers.get(1).anchorSlot == 15 and
        stl.ledgers.get(1).anchorSealedInConstructor and
        stl.ledgers.get(1).writeEntriesVkId == stl.universalWrapVkId and
        stl.ledgers.get(1).readEntriesKeepNativeVk and
        stl.ledgers.get(1).isT25PreviewDeploy
      )

  // SLC-006..009, STL-001, ZK-006, ZK-011:
  // Argument builder produces 4-word StatementPublics with fuel 0, stack_limit 0, and 8-word salt;
  // changing an effect word changes outcome_pub & claim_pub while leaving c_p & input_pub fixed (SLC-007);
  // two different private tapes produce distinct input_pub words (SLC-008);
  // call-vector lane 10/20/34/39 mismatch raises Mismatch and writes no file (SLC-009).
  run st_argumentBuilderEffectDeltaWitnessSaltAndLaneCheckTest =
    init
      .expect(not(canBuildAcceptArgs(false, 8, 0, 0, false)))
      .then(doRefuseLaneMismatch)
      .expect(inv and stl.lastRefusal == "Mismatch" and stl.knownArgIds == Set())
      .then(doBuildAcceptArgs(1, 1, 11, 50, 7, 100, true, false, true, true, false))
      .then(doBuildAcceptArgs(2, 1, 11, 50, 7, 101, true, false, true, true, false))
      .then(doBuildAcceptArgs(3, 1, 12, 51, 7, 100, true, false, true, true, false))
      .expect(
        inv and
        stl.builtArgs.get(1).cP == stl.builtArgs.get(2).cP and
        stl.builtArgs.get(1).inputPub == stl.builtArgs.get(2).inputPub and
        stl.builtArgs.get(1).outcomePub != stl.builtArgs.get(2).outcomePub and
        stl.builtArgs.get(1).claimPub != stl.builtArgs.get(2).claimPub and
        stl.builtArgs.get(1).inputPub != stl.builtArgs.get(3).inputPub and
        stl.builtArgs.get(1).outcomePub != stl.builtArgs.get(3).outcomePub
      )

  // SLC-005, SLC-010, SLC-012, STL-004, LDG-002:
  // Program B cannot settle on program A's ledger; natively refused or unbound-statement calls
  // are refused; honest call settles once (runs[claim_pub] = outcome_pub) and refuses replay.
  run st_acceptRunGuardsAndSingleSettleReplayRefusalTest =
    init
      .then(doDeployProgramLedger(1, false, true, false))
      .then(doBuildAcceptArgs(1, 1, 11, 50, 7, 9, true, false, true, true, false))
      .then(doBuildAcceptArgs(2, 2, 12, 50, 7, 9, true, false, true, true, false))
      .then(doBuildAcceptArgs(3, 1, 13, 50, 7, 9, true, true, true, true, false))
      .then(doBuildAcceptArgs(4, 1, 14, 50, 7, 9, true, false, false, true, false))
      .expect(
        not(canAcceptRun(stl, 1, 2)) and
        not(canAcceptRun(stl, 1, 3)) and
        not(canAcceptRun(stl, 1, 4)) and
        canAcceptRun(stl, 1, 1)
      )
      .then(doAcceptRun(1, 1))
      .expect(
        inv and
        not(canAcceptRun(stl, 1, 1)) and
        stl.settledRuns.get(stl.builtArgs.get(1).claimPub).outcomePub == stl.builtArgs.get(1).outcomePub and
        stl.settledRuns.get(stl.builtArgs.get(1).claimPub).acceptCount == 1
      )

  // BAT-001, BAT-002, BAT-007, BAT-008, BAT-009, BAT-012, LDG-001, LDG-003, LDG-004, LDG-005:
  // Operator jobs cannot use acceptRun; T15 freeze lists collection open before collection receipt;
  // acceptCollection is refused until L collection key is enabled; once enabled, epoch closes at
  // t + 3,600 s with 20-sibling paths and 32B leaf list, settles 1 root, and consumer uses claim once across sources.
  run st_epochBatchCollectionAndConsumerSingleUseAcrossSourcesTest =
    init
      .then(doDeployProgramLedger(1, false, true, false))
      .then(doBuildAcceptArgs(1, 1, 11, 50, 7, 9, true, false, true, false, false))
      .expect(not(canAcceptRun(stl, 1, 1)))
      .then(doWriteT15FreezeRecord(false))
      .expect(inv and stl.t15FreezeWritten and stl.t15ListsCollectionOpen)
      .then(doBuildAcceptArgs(2, 1, 15, 60, 7, 9, true, false, true, true, false))
      .then(doAcceptRun(1, 2))
      .then(doArriveOperatorJob(stl.builtArgs.get(2).claimPub))
      .then(doArriveOperatorJob(702))
      .then(doCloseEpoch(true, false))
      .expect(
        inv and
        stl.closedEpochs.exists(e =>
          e.claimCount == 2 and
          e.closeTimeS == e.firstJobTimeS + 3600 and
          e.deliveredSiblingCount == 20 and
          e.leafListPublishedOffChain and
          e.leafBytesPerClaim == 32
        ) and
        not(canAcceptCollection(stl, 10002, true))
      )
      .then(doRefuseCollectionWithoutLKey)
      .expect(inv and stl.lastRefusal == "missing_l_collection_form_key")
      .then(doEnableLCollectionKey)
      .expect(not(canAcceptCollection(stl, 10002, false)))
      .then(doAcceptCollection(10002, true))
      .expect(
        inv and
        stl.acceptedRoots.contains(10002) and
        not(canAcceptCollection(stl, 10002, true)) and
        canConsumerUseClaim(stl, stl.builtArgs.get(2).claimPub, true)
      )
      .then(doConsumerUseClaim(stl.builtArgs.get(2).claimPub, true))
      .expect(
        inv and
        not(canConsumerUseClaim(stl, stl.builtArgs.get(2).claimPub, false)) and
        not(canConsumerUseClaim(stl, stl.builtArgs.get(2).claimPub, true)) and
        not(canConsumerUseClaim(stl, 999999, true))
      )

  // BAT-005, BAT-006, SLC-011:
  // Entry needing 3 chunks is refused even at TTL = 14,400 s; unset Preview intent TTL defaults to 3,600 s;
  // suite replay deploys fresh instance on dirty state and reuses instance on deploy-block state.
  run st_chunkCapDefaultIntentTtlAndInstanceReuseTest =
    init
      .expect(not(canPlanEntryChunks(3, 14400)) and canPlanEntryChunks(2, 14400))
      .then(doRefuseOverChunkedEntry(1, 3, 14400))
      .expect(inv and stl.lastRefusal == "chunk_cap_K_MAX_2_exceeded")
      .then(doPlanEntryChunks(2, 2, 14400))
      .then(doBuildPreviewIntent(1, false, 0, false))
      .then(doBuildPreviewIntent(2, true, 7200, false))
      .then(doStartSuiteReplay(1, 1, true, false))
      .then(doStartSuiteReplay(2, 1, false, false))
      .expect(
        inv and
        stl.previewIntents.exists(pi => pi.intentId == 1 and pi.assignedTtlS == 3600) and
        stl.previewIntents.exists(pi => pi.intentId == 2 and pi.assignedTtlS == 7200) and
        stl.suiteReplays.exists(sr => sr.replayId == 1 and not(sr.deployedFreshInstance)) and
        stl.suiteReplays.exists(sr => sr.replayId == 2 and sr.deployedFreshInstance)
      )

  // BAT-010, BAT-011, STL-005, BAT-003, BAT-004, and pure spec-gap witnesses:
  // Announced era end closes open epoch early at end_eff - 242 s (< t0 + 3,600 s) and refuses
  // runs landing at or after end_eff (500 s remaining vs 757 s prove time); suite classification
  // records mismatch and refused-as-native accurately.
  run st_eraEndBoundariesSuiteClassificationAndSpecGapsTest =
    init
      .expect(
        consistency_constants and
        consistency_bat003_daily_batch_rates and
        consistency_bat004_t23_counts and
        gap_q_stl_1_ldg001_single_vs_slc005_multi_ledger and
        gap_q_stl_2_chunk2_lane10_lock_vs_cp
      )
      .then(doArriveOperatorJob(701))
      .then(doAnnounceEraEnd(942))
      .then(doCloseEpoch(true, false))
      .expect(
        inv and
        stl.nowS == 700 and
        stl.closedEpochs.exists(e => e.closeTimeS == 700 and e.closeTimeS < e.firstJobTimeS + 3600) and
        not(canStartEraRun(stl, 757)) and
        not(canStartEraRun(stl, 182)) and
        canStartEraRun(stl, 181)
      )
      .then(doRefuseLateEraRun(757))
      .expect(inv and stl.lastRefusal == "run_outlives_era_end_eff")
      .then(doStartEraRun(1, 181))
      .then(doClassifySuiteCall(1, true, false, false))
      .then(doClassifySuiteCall(2, false, true, false))
      .then(doClassifySuiteCall(3, false, false, false))
      .expect(
        inv and
        stl.suiteClassifications.exists(sc => sc.suiteCallId == 1 and sc.assignedClass == ClassRefusedAsNative) and
        stl.suiteClassifications.exists(sc => sc.suiteCallId == 2 and sc.assignedClass == ClassMismatch) and
        stl.suiteClassifications.exists(sc => sc.suiteCallId == 3 and sc.assignedClass == ClassPass)
      )
}
