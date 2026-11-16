// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Network Operations (lane e2e-network-operations).

module network_operations_test {
  import network_operations(
    PINNED_PREVIEW_GENESIS = 3096,
    LOCAL_DEVNET_GENESIS = 1001,
    PREVIEW_PROOF_SERVER_PORT = 6341,
    DEVNET_PROOF_SERVER_PORT = 6340,
    TAMPER_CONTROL_COUNT = 4,
    INBLOCK_TIMEOUT_S = 600,
    MAX_WALLET_DIR_MODE = 700,
    MAX_PHRASE_FILE_MODE = 600,
    EXIT_VERDICT_FAIL = 4,
    MAX_TTL_EXCLUSIVE_S = 1209600,
    MIN_REUSE_TTL_LEFT_S = 300,
    MAX_BALANCE_TRANSPORT_TRIES = 4,
    MAX_CONSECUTIVE_GATE_FAILS = 2,
    MUTANT_ID = 0
  ).* from "./network_operations"

  // 1. Full 7-step Preview flow: check -> balance-before -> deploy -> call+scan -> refusals -> balance-after -> receipts
  // Covers PVW-001..006, WAL-001..008, WAL-010, SCL-002, OBS-001..004, OBS-006, FRC-001, FRC-002, FRC-006.
  run no_previewHappyPathSevenStepsAndReceiptsTest =
    init
      .expect(inv and lastOp == "init")
      .then(doRunCheckStep(3600, 1, 1, false))
      .expect(
        inv and
        net.stepOkMarkers.contains("check") and
        net.manifestSha256Written and
        net.proofServerPort == 6341 and
        net.devnetPort6340Untouched
      )
      .then(doStartWalletAndBalanceBefore(3096, 700, 600, 77, false))
      .expect(
        inv and
        net.walletStarted and
        net.phraseReadInProcess and
        not(net.phraseExposedToShellOrEnv) and
        net.activeWalletSenders == 1 and
        net.stepOkMarkers.contains("balance-before")
      )
      .then(doDeployOrRecoverContract(3096, false))
      .expect(
        inv and
        net.deploySendCount == 1 and
        net.deployConfirmed and
        net.usedChainFeeParams and
        net.submittedFromWalletHost and
        net.stepOkMarkers.contains("deploy")
      )
      // Re-running deploy step recovers existing contract address without sending a second deploy (FRC-002)
      .then(doDeployOrRecoverContract(3096, false))
      .expect(inv and net.deploySendCount == 1)
      .then(doIncludeHonestCallAndScan(false))
      .expect(
        inv and
        net.honestCallIncluded and
        net.honestCallReadbackPresent and
        net.privacyScanPassed and
        not(net.forbiddenPrivateWordOnSurface) and
        net.controlWordsPresentOnSurface and
        net.stepOkMarkers.contains("call") and
        net.stepOkMarkers.contains("balance-after") and
        net.stepOkMarkers.contains("receipts")
      )
      .then(doRunTamperControlsAndVerdict(CtrlRefusedByNode, false))
      .expect(
        inv and
        net.refusalsStepStarted and
        net.submittedControlCount == 4 and
        net.verdictState == VerdictPassed and
        net.runnerExitCode == 0 and
        PREVIEW_STEPS.indices().forall(idx => net.stepOkMarkers.contains(PREVIEW_STEPS[idx]))
      )

  // 2. Pre-wallet stops, genesis/TTL/folder-VK/permission/wallet-pin guards, dry-run 0 network, and no-overwrite
  // Covers PVW-001, PVW-002, WAL-003, WAL-009, WAL-010, SCL-003, SCL-004, SCL-007, SCL-009.
  run no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest =
    init
      .expect(
        inv and
        // PVW-001: wrong RPC genesis blocked before wallet start
        not(canStartWallet(net, 8888, 700, 600, 77)) and
        // WAL-010: wide dir mode (755) or wide file mode (640) blocked
        not(canStartWallet(net, 3096, 755, 600, 77)) and
        not(canStartWallet(net, 3096, 700, 640, 77)) and
        // WAL-003: derived wallet address mismatch blocked
        not(canStartWallet(net, 3096, 700, 600, 99)) and
        // PVW-002: only undeployed1 (1001) and Preview (3096) are legal settle-runner target chains
        isLegalTargetChain(1001) and
        isLegalTargetChain(3096) and
        not(isLegalTargetChain(9999)) and
        // SCL-007: TTL must satisfy 0 < ttl < 1,209,600
        not(canAcceptTtl(0)) and
        canAcceptTtl(1) and
        canAcceptTtl(1209599) and
        not(canAcceptTtl(1209600)) and
        // SCL-004: state-folder VK must match CLI VK
        canAcceptStateFolder(1, 1) and
        not(canAcceptStateFolder(1, 2))
      )
      .then(doStopPreWalletOrConfig("SHERLOCK_SDK_ROOT_unset"))
      .expect(
        inv and
        not(net.walletStarted) and
        net.sdkRootCheckStoppedCleanly and
        net.verdictState == VerdictError and
        net.runnerExitCode == 1
      )
      .then(doAttemptWalletSetupOverwrite(false))
      .expect(
        inv and
        not(net.existingWalletOverwritten) and
        net.lastStopReason == "a wallet seed already exists"
      )
      .then(doRunSuiteDryRun(false))
      .expect(
        inv and
        net.dryRunExecuted and
        net.dryRunNetworkConnections == 0 and
        net.dryRunTxSent == 0
      )

  // 3. Tamper control outcomes: no-node-verdict (PVW-007), InBlock timeout (PVW-008), and included control (PVW-012)
  // Covers PVW-007, PVW-008, PVW-012, SCL-002, FRC-001, FRC-006.
  run no_tamperControlOutcomesAndExitCode4Test =
    init
      .then(doStartWalletAndBalanceBefore(3096, 700, 600, 77, false))
      .then(doDeployOrRecoverContract(3096, false))
      .then(doIncludeHonestCallAndScan(false))
      .then(doRunTamperControlsAndVerdict(CtrlNoNodeVerdict, false))
      .expect(
        inv and
        net.verdictState == VerdictFailed and
        net.runnerExitCode == 4 and
        not(net.stepOkMarkers.contains("refusals")) and
        net.failedStepsMarkedOk == Set()
      )
      .then(doRunTamperControlsAndVerdict(CtrlNotIncludedTimeout, false))
      .expect(
        inv and
        net.verdictState == VerdictFailed and
        net.runnerExitCode == 4 and
        not(net.stepOkMarkers.contains("refusals"))
      )
      .then(doRunTamperControlsAndVerdict(CtrlIncluded, false))
      .expect(
        inv and
        net.verdictState == VerdictFailed and
        net.runnerExitCode == 4 and
        not(net.stepOkMarkers.contains("refusals"))
      )

  // 4. T25 key gate (PVW-009, PVW-010, PVW-013, LPC-008), Validium chained batches (PVW-011, LPC-009), and SWF-009
  run no_t25GateDeployGpuBoxValidiumAndSwf009Test =
    init
      // Partial gate (T12 passes, RKY-005 & WRP-012 missing) is refused by PVW-013
      .then(doRecordT25KeyGate(false))
      .expect(
        inv and
        net.t25KeyGate.t12GatePass and
        not(canDeployFinalPrototype(net))
      )
      // Full 6-part gate passes -> final prototype deploys and proves on owned GPU box
      .then(doRecordT25KeyGate(true))
      .expect(inv and canDeployFinalPrototype(net))
      .then(doDeployAndProveFinalPrototype(false))
      .expect(
        inv and
        net.finalPrototypeDeployed and
        net.finalCallProvedOnOwnedGpuBox
      )
      // Validium batch 1 (root 10 -> 20) accepted; batch 2 (root 20 -> 30) accepted; stale root 10 refused
      .then(doSubmitValidiumBatch(1007, 10, 20, false))
      .expect(inv and net.validiumStoredRoot == 20 and net.validiumAcceptedBatches == 1)
      .then(doSubmitValidiumBatch(1007, 20, 30, false))
      .expect(inv and net.validiumStoredRoot == 30 and net.validiumAcceptedBatches == 2)
      .then(doSubmitValidiumBatch(1007, 10, 40, false))
      .expect(
        inv and
        net.validiumStoredRoot == 30 and
        net.validiumAcceptedBatches == 2 and
        not(net.validiumAcceptedBadBatch) and
        net.lastStopReason == "validium_stale_root_or_wrong_program_refused"
      )
      // SWF-009: unauthorized or auto-retried Mainnet T4 op refused; owner+checklist authorized with all 4 preconditions allowed
      .expect(
        not(canRunMainnetT4Op(false, true, false)) and
        not(canRunMainnetT4Op(true, false, false)) and
        not(canRunMainnetT4Op(true, true, true)) and
        canRunMainnetT4Op(true, true, false)
      )
      .then(doRunMainnetT4Operation(true, true, false))
      .expect(inv and net.mainnetOpRan and not(net.mainnetAutoRetried))

  // 5. Recovery: saved-call reuse guards, unknown inclusion hold, balance retries, double-gate stop, push holds, and spec gaps
  // Covers FRC-003..005, FRC-009, FRC-011, FRC-013, FRC-014, FRC-016, LPC-015, OBS-008.
  run no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest =
    init
      .expect(
        inv and
        consistency_constants and
        gap_q_netops_3_timeout_vs_ttl_expiry_reuse and
        gap_q_netops_4_pvw009_vs_pvw013 and
        // Initial saved call (ttlLeft=2400 > 300, not refused, sha256 valid) is reusable
        canReuseSavedCall(net.savedCall) and
        // Before FP-3 and T15, neither t10q nor sherlock-e2e-spec can be pushed
        not(canPushBranch(net, "t10q")) and
        not(canPushBranch(net, "sherlock-e2e-spec"))
      )
      .then(doReuseSavedCall)
      .expect(inv)
      // Boundary ttlLeft == 300 refused (FRC-003 requires STRICTLY > 300 s)
      .then(doSetSavedCallState(300, false, false, true))
      .expect(inv and not(canReuseSavedCall(net.savedCall)))
      // Marked refused saved call rejected (FRC-005)
      .then(doSetSavedCallState(2400, true, false, true))
      .expect(inv and not(canReuseSavedCall(net.savedCall)))
      // Corrupted sha256 saved call rejected (FRC-014)
      .then(doSetSavedCallState(2400, false, false, false))
      .expect(inv and not(canReuseSavedCall(net.savedCall)))
      // Inclusion-unknown call blocks reuse and resubmission (FRC-013)
      .then(doSetSavedCallState(2400, false, true, true))
      .expect(inv and not(canReuseSavedCall(net.savedCall)))
      // Transport error allows up to 4 resynced attempts (FRC-004); non-transport stops after 1 attempt (FRC-016)
      .then(doRecordWalletBalanceFailure(true, 4, false))
      .expect(
        inv and
        net.balanceAttemptCount == 4 and
        net.resyncedBeforeEachRetry and
        net.walletFailureChainRecorded
      )
      .then(doRecordWalletBalanceFailure(false, 1, false))
      .expect(
        inv and
        net.balanceAttemptCount == 1 and
        not(net.balanceIsTransportErr) and
        net.lastStopReason == "non_transport_balance_stopped"
      )
      // Two consecutive gate failures stop that row while other rows continue (FRC-009)
      .then(doRecordGateFailureOnRow)
      .expect(inv and net.rowConsecutiveGateFails == 1 and not(net.rowStoppedOnDoubleFail))
      .then(doRecordGateFailureOnRow)
      .expect(inv and net.rowConsecutiveGateFails == 2 and net.rowStoppedOnDoubleFail and net.otherRowsStillRunning)
      // FP-3 unlocks t10q push while holding sherlock-e2e-spec until T15 owner sign-off (LPC-015, FRC-011)
      .then(doAdvancePushGates(true, false))
      .expect(inv and canPushBranch(net, "t10q") and not(canPushBranch(net, "sherlock-e2e-spec")))
      .then(doPushBranch("t10q"))
      .expect(inv and net.pushedBranches == Set("t10q"))
      .then(doAdvancePushGates(false, true))
      .then(doPushBranch("sherlock-e2e-spec"))
      .expect(inv and net.pushedBranches == Set("t10q", "sherlock-e2e-spec"))
}
