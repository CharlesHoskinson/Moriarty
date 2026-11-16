// Requirement-level `.expect(...)` scenario tests for Sherlock E2E Assurance
// Lane: e2e-assurance

module assurance_test {
  import assurance(
    EXIT_CODE_SEED_REFUSED = 4,
    SECRET_FILE_MODE = 600,
    MIN_D3_TERM_DECIBITS = 1000,
    MIN_D3_GRINDABLE_DECIBITS = 1010,
    MIN_WRAP_K = 19,
    MAX_WRAP_K = 21,
    G3_TRANSCRIPT_LEGS = 5,
    MAX_GATE_FAILURES_AFK = 2,
    NOT_ATTEMPTED_INIT_COUNT = 8,
    MUTANT_ID = 0
  ).* from "./assurance"

  // Constant & ZK-019 Lean axiom consistency check
  run as_ConsistencyAndZk019AxiomsTest =
    init.expect(consistency_constants and consistency_zk019 and inv)

  // ZKE-001, ZKE-005, ZKE-006, ZK-012, ZK-013 & SSG-007:
  // Constant `r_P` without `test-seed` exits 4; missing `deploy.secret` on odin refused;
  // bad `0644` mode refused; two prepared runs get fresh salts & distinct `claim_pub`, `0` log leaks,
  // and report `not qualified` before T12 passes.
  run as_Zke001And005And006AndZk012And013AndSsg007PrepareRunsTest =
    init
      .expect(not(canPrepareRun(assure, HostMuninn, false, true, 0)))
      .then(refuseConstantRpExit4)
      .expect(assure.lastExitCode == 4 and assure.lastRefusal == AssureRefusalConstRpExit4)
      .expect(not(canPrepareRun(assure, HostOdin, false, false, 0)))
      .then(refuseMissingDeploySecret(HostOdin))
      .expect(assure.lastRefusal == AssureRefusalMissingDeploySecret)
      .then(stageWorldReadableDeploySecret(HostHuginn))
      .expect(not(canPrepareRun(assure, HostHuginn, false, false, 0)))
      .then(refuseBadModeDeploySecret(HostHuginn))
      .expect(assure.lastRefusal == AssureRefusalBadDeploySecretMode(644))
      .then(prepareRunAndLog(HostMuninn, 10, false, false))
      .then(prepareRunAndLog(HostMuninn, 10, false, false))
      .expect(all {
        assure.preparedRuns.size() == 2,
        assure.debugLogs.size() == 2,
        inv_ZKE_001,
        inv_ZKE_005,
        inv_ZKE_006,
        inv_ZK_012,
        inv_ZK_013,
        inv_SSG_007,
      })

  // ZKE-004, ZK-018 & ZK-001: Secret/witness/STARK transfer to external helper refused;
  // transfer of `deploy.secret` to owned `odin` keeps mode `0600` and enables proving on `odin`;
  // Case B job on helper host refused.
  run as_Zke004AndZk018AndZk001OwnedRosterAndCaseBRefusalTest =
    init
      .then(refuseSecretTransferOutsideRoster(HostExternalHelper, FileRunSecret))
      .expect(assure.lastRefusal == AssureRefusalSecretTransferOutsideRoster((HostExternalHelper, FileRunSecret)))
      .expect(not(canPrepareRun(assure, HostExternalHelper, false, false, 0)))
      .then(refuseCaseBUnownedHost(HostExternalHelper))
      .expect(assure.lastRefusal == AssureRefusalCaseBUnownedProverHost(HostExternalHelper))
      .then(transferSecretFileToBox(HostMuninn, HostOdin, FileDeploySecret))
      .then(prepareRunAndLog(HostOdin, 10, false, false))
      .expect(all {
        assure.admittedSecretTransfers.size() == 1,
        assure.preparedRuns.exists(r => r.hostBox == HostOdin and r.deploySecretMode == 600),
        inv_ZKE_004,
        inv_ZK_018,
      })

  // ZKE-002, ZKE-003, ZKE-007, ZK-014, ZK-015, ZK-016, ZK-017 & ZK-002/004/007:
  // `GW_DET` in release exits 4; 2 CPU + 2 GPU proofs of statement 1 are 4 pairwise-distinct verified byte strings;
  // replayed slot refused; `test-seed` proof blocked from settling; production proof settles cleanly with 0 secret hits.
  run as_Zke002And003And007AndZk014To017WrapAndSettlePrivacyTest =
    init
      .expect(not(canMakeWrapProof(assure, HostMuninn, false, true, 1, 0)))
      .then(refuseEnvSeedInReleaseWrapExit4)
      .expect(assure.lastExitCode == 4 and assure.lastRefusal == AssureRefusalEnvSeedExit4)
      .then(makeWrapProof(HostMuninn, 1, WrapCpu, false, false, 1))
      .then(makeWrapProof(HostMuninn, 1, WrapCpu, false, false, 2))
      .then(makeWrapProof(HostHuginn, 1, WrapGpu, false, false, 3))
      .then(makeWrapProof(HostOdin, 1, WrapGpu, false, false, 4))
      .expect(all {
        assure.wrapProofs.size() == 4,
        assure.wrapProofs.map(w => w.proofBytesDigest).size() == 4,
        not(canMakeWrapProof(assure, HostMuninn, false, false, 1, 0)),
        inv_ZKE_007,
        inv_ZK_014,
        inv_ZK_015,
        inv_ZK_016,
        inv_ZK_017,
      })
      .then(refuseReplayedBlindingSlot(1))
      .expect(assure.lastRefusal == AssureRefusalReplayedBlindingSlot(1))
      .then(makeWrapProof(HostMuninn, 2, WrapCpu, true, true, 5))
      .then(refuseSettleTestSeedProof(5))
      .expect(assure.lastRefusal == AssureRefusalTestSeedSettleBlocked)
      .then(settleWrapProofAndScan(1))
      .expect(all {
        assure.settledCalls.size() == 1,
        inv_ZKE_002,
        inv_ZKE_003,
        inv_ADOPTED_ASSURANCE,
      })

  // SSG-001, SSG-002, DG-001, CPR-001 & ZK-004:
  // App key of `d04 withdraw` (16,346 cols) passes D3 >= 100/101 bits; <100/101 bits refused;
  // STARK crate PLONK dependency or wrap `k = 22` refused; `k = 21` clean STARK crates pass.
  run as_Ssg001And002AndCpr001AppD3AndWrapBoundsTest =
    init
      .then(refuseLowD3AppKey(995, 1005))
      .expect(assure.lastRefusal == AssureRefusalLowAppKeyD3((995, 1005)))
      .then(checkAppKeyD3(2, 16346, 1020, 1015))
      .expect(assure.checkedAppKeys.size() == 1 and inv_SSG_001)
      .then(refusePlonkInStarkOrWrapAboveK21(22, true))
      .expect(assure.lastRefusal == AssureRefusalPlonkDepOrWrapK(22))
      .then(refuseWrapKeyFixingLTranscript)
      .expect(assure.lastRefusal == AssureRefusalWrapKeyLeaksLTranscript)
      .then(auditCratesAndWrapBoundsPass(21, false))
      .expect(all {
        assure.t25CrateGatePassed,
        assure.wrapKeyLogK == 21,
        inv_SSG_002,
        inv_ADOPTED_ASSURANCE,
      })

  // SSG-003, SSG-004, SSG-006, SQG-014, SQG-016 & SQG-001:
  // Confirmed defect (`GP2-C2.1`), G3 leg mismatch, stale T10m G5 mapping, or UNVERIFIED step blocks gate;
  // once resolved, gate passes, lists all 8 `NOT ATTEMPTED` rows as open, and promotes manifest to `KeyQualified`.
  run as_Ssg003And004And006AndSqg014And016SecurityGateQualificationTest =
    init
      .expect(not(canPassSecurityGate(assure, 0)))
      .then(refuseSecurityGateEvaluation)
      .expect(assure.lastRefusal == AssureRefusalGateBlockedConfirmedCx(1))
      .then(runG3TranscriptCheck(4))
      .expect(not(assure.g3Passed) and assure.lastRefusal == AssureRefusalGateBlockedG3Mismatch(4))
      .then(completeT12ChecksAndFixes)
      .then(writePassingGateRecordAndQualify)
      .expect(all {
        assure.gatePassed,
        assure.manifestStatus == KeyQualified,
        assure.gateRecordOpenCxIds.size() == 8,
        inv_SSG_003,
        inv_SSG_004,
        inv_SSG_006,
        inv_SQG_014,
        inv_SQG_016,
      })

  // SQG-015: Two gate failures stop row T12 under the AFK rule and prevent qualification.
  run as_Sqg015DoubleGateFailureStopsT12RowTest =
    init
      .then(recordGateCheckFailure)
      .expect(not(assure.t12RowStopped) and assure.gateFailureStreak == 1)
      .then(recordGateCheckFailure)
      .expect(all {
        assure.t12RowStopped,
        assure.gateFailureStreak == 2,
        not(canPassSecurityGate(assure, 0)),
        inv_SQG_015,
      })

  // SSG-005, LPC-004 & LPC-016: Circuit change recorded in `CHANGES.md` with Carbon Quint verdict;
  // DONE row without verdict does not count DONE for T15 until verdict is filed.
  run as_Ssg005AndLpc004And016CircuitChangeAndVerdictGateTest =
    init
      .then(landAndAuditCircuitChange(true))
      .expect(assure.changesListAuditPassed and inv_SSG_005 and inv_LPC_004)
      .then(evaluateLoopRowForT15(false))
      .expect(assure.loopRows.forall(r => not(r.countedDoneForT15)) and inv_LPC_016)
      .then(evaluateLoopRowForT15(true))
      .expect(all {
        assure.loopRows.exists(r => r.countedDoneForT15 and r.hasLeadVerdict),
        inv_SSG_005,
        inv_LPC_004,
        inv_LPC_016,
        inv,
      })
}
