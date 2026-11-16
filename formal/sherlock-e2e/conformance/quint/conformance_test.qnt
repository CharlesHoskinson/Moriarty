// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Conformance (lane e2e-conformance).

module conformance_test {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36,
    COMPILED_SUITE_PROGRAMS = 33,
    SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1,
    SUITE_TOTAL_CALLS = 208,
    SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62,
    CALL_CLASSES_COUNT = 6,
    ACTIVE_BATCH_SIZE = 4,
    MUTANT_ID = 0
  ).* from "./conformance"

  // 1. Manifest schema `t18-compact-suite-manifest/1`, native reference `mycroft-native-ref/1`, and call identity uniqueness
  // Covers ECS-001, ECS-002, ECS-011, ECS-013.
  run cf_manifestNativeReferencesAndUniqueCallIdentityTest =
    init
      .expect(inv and lastOp == "init")
      .then(doBuildManifestAndNativeRefs(false))
      .expect(
        inv and
        conf.manifest.evaluated and
        conf.manifest.manifestSchema == "t18-compact-suite-manifest/1" and
        conf.manifest.manifestProgramCount == 36 and
        conf.manifest.compiledProgramCount == 33 and
        conf.manifest.callsForUncompiledPrograms == 0 and
        conf.manifest.nativeRefSchema == "mycroft-native-ref/1" and
        not(conf.manifest.rejectedRefHasTranscriptWords) and
        conf.manifest.duplicateIdentityCount == 0 and
        conf.manifest.unmappedExecutedVectorCount == 0
      )

  // 2. All 6 call classes (`pass`, `refused-as-native`, `mismatch`, `blocked`, `prove-failed`, `n/a`) & stage order
  // Covers ECS-003, ECS-004, ECS-005, ECS-009.
  run cf_sixCallClassesStageOrderingAndRefusalEvidenceLabelsTest =
    init
      // Pure circuit h02 -> n/a with 0 stages run
      .then(doEvaluateSuiteCall(ScenPureH02, true, true, true, false))
      .expect(
        inv and
        conf.callRun.resultClass == ClsNa and
        not(conf.callRun.compileRan) and
        not(conf.callRun.proveRan)
      )
      // Compile mismatch -> ClsMismatch and prove stage does NOT run (ECS-003 Scenario)
      .then(doEvaluateSuiteCall(ScenEligibleCompileMismatch, true, true, true, false))
      .expect(
        inv and
        conf.callRun.resultClass == ClsMismatch and
        conf.callRun.compileRan and
        not(conf.callRun.proveRan) and
        conf.compileMismatchEarlyStopSeen
      )
      // Prove failure -> ClsProveFailed
      .then(doEvaluateSuiteCall(ScenEligibleProveFailed, true, true, true, false))
      .expect(inv and conf.callRun.resultClass == ClsProveFailed)
      // Proof with no passing native Preview transcript -> ClsBlocked ("no native Preview transcript") (ECS-005)
      .then(doEvaluateSuiteCall(ScenEligibleNoNativePreviewRef, true, true, true, false))
      .expect(
        inv and
        conf.callRun.resultClass == ClsBlocked and
        conf.callRun.blockedReason == "no native Preview transcript"
      )
      // Refused-as-native without R_SL execution -> DERIVED; with R_SL execution -> MEASURED (ECS-009)
      .then(doEvaluateSuiteCall(ScenNativeRejectedDerived, true, true, true, false))
      .expect(
        inv and
        conf.callRun.resultClass == ClsRefusedAsNative and
        conf.callRun.evidenceKind == EvDerived
      )
      .then(doEvaluateSuiteCall(ScenNativeRejectedExecuted, true, true, true, false))
      .expect(
        inv and
        conf.callRun.resultClass == ClsRefusedAsNative and
        conf.callRun.evidenceKind == EvMeasured
      )

  // 3. Dispatch switch, Preview switch, machine-version kit unblocking, and dry-settle blocked ("preview off")
  // Covers ECS-006, ECS-007, ECS-008, ECS-015.
  run cf_dispatchPreviewSwitchesKitReasonsAndDrySettleBlockedTest =
    init
      // Dispatch OFF -> no prover dispatch, blocked ("dispatch off") (ECS-007)
      .then(doEvaluateSuiteCall(ScenEligibleLivePass, false, false, true, false))
      .expect(
        inv and
        not(conf.callRun.dispatchedToProverBox) and
        not(conf.callRun.previewTxSubmitted) and
        not(conf.callRun.walletOpened) and
        conf.callRun.resultClass == ClsBlocked and
        conf.callRun.blockedReason == "dispatch off"
      )
      // Dispatch ON, Preview OFF -> proves & dry-settles without opening wallet -> blocked ("preview off") (ECS-008, ECS-015)
      .then(doEvaluateSuiteCall(ScenEligibleLivePass, true, false, true, false))
      .expect(
        inv and
        conf.callRun.dispatchedToProverBox and
        not(conf.callRun.previewTxSubmitted) and
        not(conf.callRun.walletOpened) and
        conf.callRun.settledWithoutPreviewTx and
        conf.callRun.resultClass == ClsBlocked and
        conf.callRun.blockedReason == "preview off" and
        conf.dryPreviewBlockedSeen
      )
      // Kit missing -> blocked; kit present in machine version + both switches ON -> ClsPass (ECS-006)
      .then(doEvaluateSuiteCall(ScenEligibleKitCall, true, true, false, false))
      .expect(
        inv and
        conf.callRun.resultClass == ClsBlocked and
        conf.callRun.blockedReason == "kit missing from machine version"
      )
      .then(doEvaluateSuiteCall(ScenEligibleKitCall, true, true, true, false))
      .expect(
        inv and
        not(conf.callRun.blockedForEnabledKit) and
        conf.callRun.previewTxSubmitted and
        conf.callRun.resultClass == ClsPass and
        conf.livePassSeen
      )

  // 4. Suite table counts, mismatch denominator, full-evidence live completion gate, and spec-gap witnesses
  // Covers ECS-010, ECS-011, ECS-012, ECS-014, Q-conformance-1..4.
  run cf_suiteSummaryMismatchDenominatorLiveCompletionAndSpecGapsTest =
    init
      .expect(
        inv and
        consistency_constants and
        gap_q_conf_1_compile_mismatch_exceeds_post_proof_denominator and
        gap_q_conf_2_ecs015_unconditional_masks_dry_mismatch and
        gap_q_conf_3_ecs002_preimage_required_vs_ecs009_missing_62_tapes and
        gap_q_conf_4_ecs010_per_call_tx_vs_63_non_settling_calls
      )
      // Dry run: 0 proved comparisons -> mismatch 0 of 0 compared, not complete (ECS-012, ECS-014)
      .then(doWriteSuiteSummaryAndCheckCompletion(0, 0, 145, false, false))
      .expect(
        inv and
        conf.summary.provedComparedCount == 0 and
        conf.summary.mismatchCount == 0 and
        conf.summary.denominatorReportedWithMismatch and
        not(conf.summary.liveRunMarkedComplete)
      )
      // Missing receipt on 1 call or 1 mismatch blocks live completion (ECS-010, ECS-014)
      .then(doWriteSuiteSummaryAndCheckCompletion(145, 0, 1, false, false))
      .expect(inv and not(conf.summary.liveRunMarkedComplete))
      .then(doWriteSuiteSummaryAndCheckCompletion(145, 1, 0, false, false))
      .expect(inv and not(conf.summary.liveRunMarkedComplete))
      // All 145 eligible calls proved, settled, read back, and matched native with 0 missing items -> complete!
      .then(doWriteSuiteSummaryAndCheckCompletion(145, 0, 0, false, false))
      .expect(
        inv and
        conf.summary.reportedPrograms == 33 and
        conf.summary.reportedCircuits == 106 and
        conf.summary.reportedCalls == 208 and
        conf.summary.tableRowSum == 208 and
        conf.summary.liveRunMarkedComplete and
        conf.liveRunCompleteSeen
      )
}
