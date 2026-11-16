// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Call Flow (lane e2e-call-flow).

module call_flow_test {
  import call_flow(
    EUF001_STAGE_COUNT = 11,
    SECOND_CALL_STAGE_COUNT = 7,
    PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145,
    MAINTAINER_TOOLS_ALLOWED = 0,
    LOOP_ROW_COUNT = 5,
    MUTANT_ID = 0
  ).* from "./call_flow"

  // 1. First joined run, second call starting at `tape` with 0 deploys, clean user box (0 maintainer tools), and read-back
  // Covers EUF-001, EUF-002, EUF-010, EUF-011.
  run cfl_firstCallSecondCallFromTapeAndReadbackTest =
    init
      .expect(inv and lastOp == "init")
      .then(doRunEndToEndCallPipeline(FlowFirstCallHappy, false))
      .expect(
        inv and
        flow.pipeline.stagesInCanonicalOrder and
        flow.pipeline.stageInputsSatisfied and
        flow.pipeline.deploymentsSentThisCall == 1 and
        flow.pipeline.txSentCount == 1 and
        flow.pipeline.readbackVerifiedOk and
        flow.pipeline.maintainerToolsUsed == 0 and
        flow.firstCallSettledSeen
      )
      .then(doRunEndToEndCallPipeline(FlowSecondCallFromTape, false))
      .expect(
        inv and
        flow.pipeline.isSecondCallOfDeployed and
        flow.pipeline.deploymentsSentThisCall == 0 and
        flow.pipeline.readbackVerifiedOk and
        flow.secondCallFromTapeSeen
      )
      // Mismatched read-back fails verification (EUF-010)
      .then(doRunEndToEndCallPipeline(FlowReadbackMismatch, false))
      .expect(inv and not(flow.pipeline.readbackVerifiedOk))

  // 2. Pre-submit stage stops (missing L_Fr artifact in wrap, muninn wrap refusal) and execute-stage Compact rejection
  // Covers EUF-002, EUF-005, EUF-006.
  run cfl_preSubmitStageStopsAndNativeCompactRejectionTest =
    init
      .then(doRunEndToEndCallPipeline(FlowMissingWrapArtifactStop, false))
      .expect(
        inv and
        flow.pipeline.missingArtifactStoppedCleanly and
        not(flow.pipeline.submitStageStarted) and
        flow.pipeline.txSentCount == 0 and
        flow.pipeline.recordedStopStageAndError
      )
      .then(doRunEndToEndCallPipeline(FlowLedgerCheckRefusedStop, false))
      .expect(
        inv and
        not(flow.pipeline.submitStageStarted) and
        flow.pipeline.txSentCount == 0 and
        flow.pipeline.recordedStopStageAndError
      )
      .then(doRunEndToEndCallPipeline(FlowCompactExecuteRejected, false))
      .expect(
        inv and
        flow.pipeline.executeRejectedCompact and
        not(flow.pipeline.proveStageStarted) and
        not(flow.pipeline.submitStageStarted) and
        flow.pipeline.txSentCount == 0
      )

  // 3. Box profile key-set check, 5-pin mismatch stops, cross-box claim_pub equality, and shared wrap VK across deploys
  // Covers EUF-007, EUF-008, EUF-009, EUF-012.
  run cfl_boxProfileVersionPinsCrossBoxClaimAndSharedWrapVkTest =
    init
      // Box without matching profile for T25 keys cannot prove (EUF-007); keys_d pin mismatch stops call (EUF-012)
      .then(doCheckProfilePinsClaimAndMultiDeploy(false, "keys_d", false))
      .expect(
        inv and
        not(flow.pinsBoxes.proveAllowedOnBox) and
        flow.pinsBoxes.stoppedAndNamedPin and
        not(flow.pinsBoxes.continuedPastPinMismatch)
      )
      // Matching profile + matching pins: prove allowed, claim_pub equal across boxes, wrap VK equal across deploys
      .then(doCheckProfilePinsClaimAndMultiDeploy(true, "none", false))
      .expect(
        inv and
        flow.pinsBoxes.proveAllowedOnBox and
        flow.pinsBoxes.claimPubBoxA == flow.pinsBoxes.claimPubBoxB and
        flow.pinsBoxes.firstProgWrapVk == flow.pinsBoxes.secondProgWrapVk and
        flow.pinsBoxes.progAnchorsDiffer
      )

  // 4. Post-submit reconciliation without resubmission, 145-call prototype completion, T15 freeze, DOM_LOCK docs, and spec gaps
  // Covers EUF-013, EUF-014, LPC-014, LPC-017, Q-call-flow-1..5.
  run cfl_reconcileUnreadSubmitPrototypeFreezeDocsAndSpecGapsTest =
    init
      .expect(
        inv and
        consistency_constants and
        gap_q_cf_1_euf001_vs_design_dep_table_mismatch and
        gap_q_cf_2_first_call_11_stages_vs_second_call_7_stages and
        gap_q_cf_3_euf010_requires_absent_native_transcript_on_user_call and
        gap_q_cf_4_euf014_blocks_retry_after_resolved_non_inclusion and
        gap_q_cf_5_lpc014_owner_wait_vs_euf013_freeze_evidence
      )
      // Submitted call with lost node response reconciles by tx hash and sends no second tx (EUF-014)
      .then(doReconcileSubmittedCall(true, false))
      .expect(
        inv and
        flow.reconcile.reconciledByTxHash and
        flow.reconcile.txCountForClaimPub == 1 and
        flow.reconciledUnreadSubmitSeen
      )
      // Partial suite (60 < 145) or unverified row blocks completion and T15 freeze (EUF-013, LPC-014)
      .then(doEvaluatePrototypeFreezeAndDocs(false, true, false, false))
      .expect(
        inv and
        not(flow.completion.prototypeReportedComplete) and
        not(flow.completion.freezePresented) and
        flow.completion.domLockHitsInDesignDocs == 0
      )
      // All 145 eligible calls on 1 universal key set + all rows ready -> prototype complete & T15 freeze ready
      .then(doEvaluatePrototypeFreezeAndDocs(true, true, true, false))
      .expect(
        inv and
        flow.completion.prototypeReportedComplete and
        flow.completion.freezePresented and
        flow.completion.domLockHitsInDesignDocs == 0 and
        flow.prototypeAndFreezeCompleteSeen
      )
}
