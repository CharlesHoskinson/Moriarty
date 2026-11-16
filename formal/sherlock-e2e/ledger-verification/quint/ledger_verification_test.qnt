// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Ledger Verification (lane e2e-ledger-verification).

module ledger_verification_test {
  import ledger_verification(
    PINNED_LEDGER_CRATE_COUNT = 4,
    CHANGED_LANE_CONTROL_COUNT = 7,
    FLIPPED_BYTE_CONTROL_COUNT = 3,
    CALL_VECTOR_LEN = 1026,
    PINNED_LEDGER_REV_ID = 9101,
    PINNED_PREVIEW_NODE_BUILD_ID = 21101,
    NODE_INVALID_PROOF_ERR_CODE = 115,
    MUTANT_ID = 0
  ).* from "./ledger_verification"

  // LVR-001, LVR-002:
  // Local path override or wrong git rev fails the manifest gate; both direct git rev a2b30128
  // and [patch.crates-io] selecting rev a2b30128 pass all 4 midnight-ledger crate checks.
  run lv_manifestGateAndPatchCratesIoTest =
    init
      .expect(
        inv and
        lastOp == "init" and
        not(canPassManifestGate(mkOverriddenCrateDeps(true), true)) and
        not(canPassManifestGate(mkOverriddenCrateDeps(false), true))
      )
      .then(doRefuseManifestOverride(true))
      .expect(
        inv and
        lvr.manifestGateChecked and
        not(lvr.manifestGatePassed) and
        lvr.lastManifestError == "local_path_override_refused"
      )
      .then(doRefuseManifestOverride(false))
      .expect(
        inv and
        not(lvr.manifestGatePassed) and
        lvr.lastManifestError == "wrong_git_rev_refused"
      )
      .then(doCheckManifestPass(true, false))
      .expect(
        inv and
        lvr.manifestGatePassed and
        allFourLedgerCratesPinned(lvr.crateDeps)
      )

  // LVR-001, LVR-003, LVR-004, LVR-005:
  // Honest MidnightVK (ZkStdLib) bundle verifies under stock 9.1.0.1 VerifierKey::verify,
  // refuses all 7 named changed-lane vectors (0, 10, 20, 34, 39, 513, 1025) and all 3 flipped-byte
  // proofs, and exits 0 (verdictOk == true).
  run lv_honestCheckAndTenTamperControlsRefusedTest =
    init
      .then(doCheckManifestPass(false, false))
      .then(doStageWrapBundle(1, MidnightVkZkStdLib, true, false))
      .then(doRunLedgerCheckTool(1, false))
      .expect(
        inv and
        lastOp == "runLedgerCheckTool" and
        lvr.checkRuns.exists(r =>
          r.bundleId == 1 and
          r.usedStockVerifierVerify and
          r.verifierCrateRevId == 9101 and
          r.usedMidnightVkDeserialize and
          r.keyReadAccepted and
          r.honestProofAccepted and
          r.changedLanesChecked == Set(0, 10, 20, 34, 39, 513, 1025) and
          r.changedLaneRefusedCount == 7 and
          r.flippedByteCheckedCount == 3 and
          r.flippedByteRefusedCount == 3 and
          not(r.anyTamperControlAccepted) and
          r.exitCode == 0 and
          r.verdictOk
        )
      )

  // LVR-003, LW-017:
  // A G4 lane-gate key or malformed framing is refused by VerifierKey::deserialize (MidnightVK),
  // exits nonzero, and reports no G4 ledger-accepted figure.
  run lv_g4KeyRefusedByReaderTest =
    init
      .then(doCheckManifestPass(false, false))
      .then(doStageWrapBundle(2, G4LaneGateKey, true, false))
      .then(doRunLedgerCheckTool(2, false))
      .expect(
        inv and
        lvr.checkRuns.exists(r =>
          r.bundleId == 2 and
          r.vkFraming == G4LaneGateKey and
          not(r.keyReadAccepted) and
          not(r.g4ReportedAccepted) and
          r.exitCode != 0 and
          not(r.verdictOk)
        )
      )

  // LVR-005:
  // Both an accepted flipped-byte tamper control AND a refused honest proof cause nonzero exit status.
  run lv_acceptedControlOrRefusedHonestExitsNonzeroTest =
    init
      .then(doCheckManifestPass(false, false))
      .then(doStageWrapBundle(3, MidnightVkZkStdLib, true, true))
      .then(doRunLedgerCheckTool(3, false))
      .expect(
        inv and
        lvr.checkRuns.exists(r =>
          r.bundleId == 3 and
          r.anyTamperControlAccepted and
          r.flippedByteRefusedCount == 2 and
          r.exitCode != 0 and
          not(r.verdictOk)
        )
      )
      .then(doStageWrapBundle(4, MidnightVkZkStdLib, false, false))
      .then(doRunLedgerCheckTool(4, false))
      .expect(
        inv and
        lvr.checkRuns.exists(r =>
          r.bundleId == 4 and
          not(r.honestProofAccepted) and
          r.exitCode != 0 and
          not(r.verdictOk)
        )
      )

  // LVR-006, LVR-007:
  // Settle runner cannot submit before running the native 9.1.0.1 validator;
  // tampered call R1 gets "failed to verify proof: Invalid proof" (equivalent to custom error 115);
  // natively refused honest call is marked refused in state, needs fresh prove on rerun, and is not submitted;
  // natively accepted honest call submits cleanly.
  run lv_nativeValidatorBeforeSubmitAndRefusalNoSubmitTest =
    init
      .then(doConnectSettleNode(1, 100, 21101))
      .then(doBuildLedgerCall(1, 1, true, true))
      .expect(not(canSubmitCall(lvr, 1)))
      .then(doRunNativeValidator(1))
      .expect(canSubmitCall(lvr, 1))
      .then(doSubmitValidatedCall(1))
      .expect(
        inv and
        lvr.calls.get(1).nativeValidatorRan and
        lvr.calls.get(1).nativeValidatorAccepted and
        lvr.calls.get(1).submittedToNode
      )
      .then(doBuildLedgerCall(2, 1, false, false))
      .then(doRunNativeValidator(2))
      .expect(
        inv and
        not(canSubmitCall(lvr, 2)) and
        lvr.calls.get(2).nativeRefusalMsg == "failed to verify proof: Invalid proof" and
        lvr.calls.get(2).equivalentNodeCustomError == 115 and
        not(lvr.calls.get(2).submittedToNode)
      )
      .then(doBuildLedgerCall(3, 1, true, false))
      .then(doRunNativeValidator(3))
      .expect(
        inv and
        not(canSubmitCall(lvr, 3)) and
        lvr.calls.get(3).markedRefusedInState and
        lvr.calls.get(3).needsFreshProveOnRerun and
        not(lvr.calls.get(3).submittedToNode)
      )

  // LVR-008, MID-006, and pure spec-gap witnesses:
  // First connect records genesis hash and node version; connecting the same state folder to a second
  // genesis hash stops with "same local chain as the deploy"; Preview qualification requires build 2.1.101-bbd616b1.
  run lv_chainIdentitySecondGenesisRefusalAndPreviewPinTest =
    init
      .expect(
        consistency_constants and
        gap_q_lvr_2_stdlib_patch_passes_4crate_check and
        gap_q_lvr_3_short_vector_lane_collision and
        not(canWritePreviewQualification(21199, 21101))
      )
      .then(doConnectSettleNode(1, 100, 21101))
      .expect(
        inv and
        lvr.folders.get(1).connectedOnce and
        lvr.folders.get(1).recordedGenesisHash == 100 and
        lvr.folders.get(1).recordedNodeBuildId == 21101 and
        not(canConnectFolder(lvr, 1, 200, 21101))
      )
      .then(doRefuseSecondGenesisForFolder(1, 200))
      .expect(
        inv and
        lvr.folders.get(1).chainMismatchStopped and
        lvr.folders.get(1).lastConnectStopReason == "same local chain as the deploy" and
        lvr.folders.get(1).recordedGenesisHash == 100
      )
      .then(doRefusePreviewPinMismatch(21199))
      .expect(inv and lvr.lastRunnerError == "preview_node_build_pin_mismatch" and not(lvr.previewQualWritten))
      .then(doRecordPreviewQualification(21101, 21101))
      .expect(
        inv and
        lvr.previewQualWritten and
        lvr.previewReportedNodeBuildId == 21101 and
        lvr.previewPinnedNodeBuildId == 21101
      )
}
