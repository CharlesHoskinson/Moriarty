// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Conformance (lane e2e-conformance).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-compact-suite/spec.md (ECS-001..014 + ECS-015)
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md (Conformance rows in sections 1-4, 6-8)

module conformance {
  // =========================================================================
  // Constants (suite manifest counts, circuit/call denominators, and classes)
  // =========================================================================

  // ECS-001: total programs listed in MANIFEST.json (36: 33 compile + 3 rejected by stock compactc).
  const MANIFEST_TOTAL_PROGRAMS: int
  // ECS-001, ECS-011: compiled suite programs (33).
  const COMPILED_SUITE_PROGRAMS: int
  // ECS-011: total circuits across the 33 compiled programs, including pure h02 nativeMethods (106).
  const SUITE_TOTAL_CIRCUITS: int
  // ECS-004 Scenario, ECS-011: count of pure circuits with no ZKIR, proof, or transaction (1).
  const SUITE_PURE_CIRCUITS: int
  // ECS-002, ECS-004, ECS-011: total recorded suite calls (208).
  const SUITE_TOTAL_CALLS: int
  // ECS-004 Status, ECS-012 Status, ECS-014: eligible proof calls that pass natively (145).
  const SUITE_ELIGIBLE_PROOF_CALLS: int
  // ECS-002 Status, ECS-004 Status, ECS-009: calls rejected by the native circuit (62).
  const SUITE_NATIVE_REJECTED_CALLS: int
  // ECS-004: number of mutually exclusive call result classes (6).
  const CALL_CLASSES_COUNT: int
  // Configurable active batch size varied across concrete instances (1..6).
  const ACTIVE_BATCH_SIZE: int
  // Mutant selector: 0 = honest specification; 1..15 = one guard mutant per ECS-001..015.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_ECS001_CALLS_FOR_UNCOMPILED: int = 1
  pure val MUT_ECS002_TRANSCRIPT_ON_REJECTED_REF: int = 2
  pure val MUT_ECS003_PROVE_AFTER_COMPILE_MISMATCH: int = 3
  pure val MUT_ECS004_MULTI_OR_BAD_PURE_CLASS: int = 4
  pure val MUT_ECS005_BLOCKED_EMPTY_REASON: int = 5
  pure val MUT_ECS006_BLOCK_ENABLED_KIT: int = 6
  pure val MUT_ECS007_DISPATCH_WITHOUT_SWITCH: int = 7
  pure val MUT_ECS008_PREVIEW_WITHOUT_SWITCH: int = 8
  pure val MUT_ECS009_UNEXECUTED_REFUSAL_MEASURED: int = 9
  pure val MUT_ECS010_DONE_ON_LIVE_MISMATCH: int = 10
  pure val MUT_ECS011_INCONSISTENT_SUITE_COUNTS: int = 11
  pure val MUT_ECS012_OMIT_MISMATCH_DENOMINATOR: int = 12
  pure val MUT_ECS013_DUPLICATE_OR_UNMAPPED_ID: int = 13
  pure val MUT_ECS014_COMPLETE_MISSING_EVIDENCE: int = 14
  pure val MUT_ECS015_PASS_ON_DRY_SETTLE: int = 15

  // =========================================================================
  // Domain types
  // =========================================================================

  type CallResultClass =
    | ClsPass
    | ClsRefusedAsNative
    | ClsMismatch
    | ClsBlocked
    | ClsProveFailed
    | ClsNa

  type EvidenceLabel =
    | EvDerived
    | EvMeasured
    | EvNotApplicable

  type SuiteCallScenario =
    | ScenEligibleLivePass
    | ScenEligibleCompileMismatch
    | ScenEligibleProveFailed
    | ScenEligibleNoNativePreviewRef
    | ScenEligibleKitCall
    | ScenNativeRejectedDerived
    | ScenNativeRejectedExecuted
    | ScenPureH02

  type ManifestRefState = {
    evaluated: bool,
    manifestSchema: str,
    manifestProgramCount: int,
    compiledProgramCount: int,
    callsForUncompiledPrograms: int,
    nativeRefSchema: str,
    rejectedRefHasTranscriptWords: bool,
    duplicateIdentityCount: int,
    unmappedExecutedVectorCount: int
  }

  type CallRunState = {
    evaluated: bool,
    scenario: SuiteCallScenario,
    allowDispatch: bool,
    allowPreview: bool,
    machineHoldsKit: bool,
    compileRan: bool,
    proveRan: bool,
    settleRan: bool,
    compareRan: bool,
    stageOrderValid: bool,
    compileMatchedNative: bool,
    dispatchedToProverBox: bool,
    previewTxSubmitted: bool,
    walletOpened: bool,
    settledWithoutPreviewTx: bool,
    assignedClassCount: int,
    resultClass: CallResultClass,
    blockedReason: str,
    blockedForEnabledKit: bool,
    hasExecutedRslRejection: bool,
    evidenceKind: EvidenceLabel
  }

  type SuiteSummaryState = {
    evaluated: bool,
    reportedPrograms: int,
    reportedCircuits: int,
    reportedCalls: int,
    tableRowSum: int,
    provedComparedCount: int,
    mismatchCount: int,
    denominatorReportedWithMismatch: bool,
    eligibleMissingEvidenceCount: int,
    emptyVectorComparedEqual: bool,
    liveTableFieldsPresent: bool,
    liveRunMarkedComplete: bool
  }

  type ConformanceState = {
    manifest: ManifestRefState,
    callRun: CallRunState,
    summary: SuiteSummaryState,
    // Monotone reachability flags across random walks
    livePassSeen: bool,
    dryPreviewBlockedSeen: bool,
    compileMismatchEarlyStopSeen: bool,
    derivedAndMeasuredRefusalsSeen: bool,
    liveRunCompleteSeen: bool
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var conf: ConformanceState
  var lastOp: str

  // =========================================================================
  // Pure consistency & spec-gap witness predicates
  // =========================================================================

  val consistency_constants: bool =
    MANIFEST_TOTAL_PROGRAMS == 36 and
    COMPILED_SUITE_PROGRAMS == 33 and
    COMPILED_SUITE_PROGRAMS < MANIFEST_TOTAL_PROGRAMS and
    SUITE_TOTAL_CIRCUITS == 106 and
    SUITE_PURE_CIRCUITS == 1 and
    SUITE_TOTAL_CALLS == 208 and
    SUITE_ELIGIBLE_PROOF_CALLS == 145 and
    SUITE_NATIVE_REJECTED_CALLS == 62 and
    SUITE_ELIGIBLE_PROOF_CALLS + SUITE_NATIVE_REJECTED_CALLS + SUITE_PURE_CIRCUITS == SUITE_TOTAL_CALLS and
    CALL_CLASSES_COUNT == 6 and
    ACTIVE_BATCH_SIZE >= 1 and
    ACTIVE_BATCH_SIZE <= 6

  // Finding Q-conformance-1 witness:
  // When 1 call mismatches at stage 1 (compile W != native_words) and stops before prove (ECS-003 Scenario)
  // while 0 calls reach compare after a proof, ECS-012's line "Mismatch: n of m compared after a proof"
  // reports n = 1 > m = 0 (numerator strictly exceeds its own stated denominator).
  pure val gap_q_conf_1_compile_mismatch_exceeds_post_proof_denominator: bool = {
    val compileStageMismatches = 1
    val comparedAfterProofM = 0
    val totalMismatchesN = compileStageMismatches
    totalMismatchesN > comparedAfterProofM
  }

  // Finding Q-conformance-2 witness:
  // Literal ECS-015 ("If the T23 runner settled a call without a Preview transaction, then the T23 runner
  // SHALL class the call as blocked with the reason `preview off`") has an unconditional antecedent that
  // overwrites a post-proof transcript `mismatch` as `blocked ("preview off")` whenever Preview is off.
  pure val gap_q_conf_2_ecs015_unconditional_masks_dry_mismatch: bool = {
    val settledWithoutPreviewTx = true
    val postProofTranscriptDiffers = true
    val literalEcs015Class = ClsBlocked
    settledWithoutPreviewTx and postProofTranscriptDiffers and (literalEcs015Class != ClsMismatch)
  }

  // Finding Q-conformance-3 witness:
  // ECS-002 requires storing the preimage for every suite call (only transcript words are omitted on native
  // rejection), whereas ECS-009 Status states all 62 rejected rows lack preimages/tapes.
  pure val gap_q_conf_3_ecs002_preimage_required_vs_ecs009_missing_62_tapes: bool = {
    val ecs002RequiresPreimagePerCall = true
    val ecs009StoredRejectedPreimages = 0
    ecs002RequiresPreimagePerCall and (ecs009StoredRejectedPreimages < SUITE_NATIVE_REJECTED_CALLS)
  }

  // Finding Q-conformance-4 witness:
  // ECS-010 demands per-call Preview tx hash/block and prove/wrap times across the 208-row suite table,
  // but 63 of the 208 calls (62 refused-as-native + 1 pure h02) never prove, wrap, or settle on Preview.
  val gap_q_conf_4_ecs010_per_call_tx_vs_63_non_settling_calls: bool = {
    val nonSettlingSuiteCalls = SUITE_NATIVE_REJECTED_CALLS + SUITE_PURE_CIRCUITS
    nonSettlingSuiteCalls == 63 and nonSettlingSuiteCalls > 0 and nonSettlingSuiteCalls < SUITE_TOTAL_CALLS
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    conf' = {
      manifest: {
        evaluated: false,
        manifestSchema: "t18-compact-suite-manifest/1",
        manifestProgramCount: MANIFEST_TOTAL_PROGRAMS,
        compiledProgramCount: COMPILED_SUITE_PROGRAMS,
        callsForUncompiledPrograms: 0,
        nativeRefSchema: "mycroft-native-ref/1",
        rejectedRefHasTranscriptWords: false,
        duplicateIdentityCount: 0,
        unmappedExecutedVectorCount: 0
      },
      callRun: {
        evaluated: false,
        scenario: ScenEligibleLivePass,
        allowDispatch: false,
        allowPreview: false,
        machineHoldsKit: true,
        compileRan: false,
        proveRan: false,
        settleRan: false,
        compareRan: false,
        stageOrderValid: true,
        compileMatchedNative: true,
        dispatchedToProverBox: false,
        previewTxSubmitted: false,
        walletOpened: false,
        settledWithoutPreviewTx: false,
        assignedClassCount: 1,
        resultClass: ClsBlocked,
        blockedReason: "dispatch off",
        blockedForEnabledKit: false,
        hasExecutedRslRejection: false,
        evidenceKind: EvNotApplicable
      },
      summary: {
        evaluated: false,
        reportedPrograms: COMPILED_SUITE_PROGRAMS,
        reportedCircuits: SUITE_TOTAL_CIRCUITS,
        reportedCalls: SUITE_TOTAL_CALLS,
        tableRowSum: SUITE_TOTAL_CALLS,
        provedComparedCount: 0,
        mismatchCount: 0,
        denominatorReportedWithMismatch: true,
        eligibleMissingEvidenceCount: SUITE_ELIGIBLE_PROOF_CALLS,
        emptyVectorComparedEqual: false,
        liveTableFieldsPresent: true,
        liveRunMarkedComplete: false
      },
      livePassSeen: false,
      dryPreviewBlockedSeen: false,
      compileMismatchEarlyStopSeen: false,
      derivedAndMeasuredRefusalsSeen: false,
      liveRunCompleteSeen: false
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Implements ECS-001, ECS-002, ECS-013:
  // Builds `t18-compact-suite-manifest/1`, generates `mycroft-native-ref/1` records, and checks unique call identities.
  action doBuildManifestAndNativeRefs(mutTrig: bool): bool = {
    val mutUncompiledCalls = (MUTANT_ID == MUT_ECS001_CALLS_FOR_UNCOMPILED and mutTrig)
    val mutRejWords = (MUTANT_ID == MUT_ECS002_TRANSCRIPT_ON_REJECTED_REF and mutTrig)
    val mutDupIds = (MUTANT_ID == MUT_ECS013_DUPLICATE_OR_UNMAPPED_ID and mutTrig)
    all {
      conf' = {
        ...conf,
        manifest: {
          evaluated: true,
          manifestSchema: if (mutUncompiledCalls) "wrong-manifest/0" else "t18-compact-suite-manifest/1",
          manifestProgramCount: MANIFEST_TOTAL_PROGRAMS,
          compiledProgramCount: COMPILED_SUITE_PROGRAMS,
          callsForUncompiledPrograms: if (mutUncompiledCalls) 2 else 0,
          nativeRefSchema: if (mutRejWords) "wrong-ref/0" else "mycroft-native-ref/1",
          rejectedRefHasTranscriptWords: mutRejWords,
          duplicateIdentityCount: if (mutDupIds) 1 else 0,
          unmappedExecutedVectorCount: if (mutDupIds) 3 else 0
        }
      },
      lastOp' = "buildManifestAndNativeRefs"
    }
  }

  // Implements ECS-003, ECS-004, ECS-005, ECS-006, ECS-007, ECS-008, ECS-009, ECS-015:
  // Executes one suite call through compile -> prove -> settle -> compare under dispatch/preview switches.
  action doEvaluateSuiteCall(
    scen: SuiteCallScenario,
    dispatchSw: bool,
    previewSw: bool,
    kitInMachine: bool,
    mutTrig: bool
  ): bool = {
    val mutProveAfterMis = (MUTANT_ID == MUT_ECS003_PROVE_AFTER_COMPILE_MISMATCH and scen == ScenEligibleCompileMismatch and mutTrig)
    val mutBadClass = (MUTANT_ID == MUT_ECS004_MULTI_OR_BAD_PURE_CLASS and mutTrig)
    val mutEmptyReason = (MUTANT_ID == MUT_ECS005_BLOCKED_EMPTY_REASON and mutTrig)
    val mutBlockKit = (MUTANT_ID == MUT_ECS006_BLOCK_ENABLED_KIT and scen == ScenEligibleKitCall and kitInMachine and mutTrig)
    val mutBadDispatch = (MUTANT_ID == MUT_ECS007_DISPATCH_WITHOUT_SWITCH and not(dispatchSw) and mutTrig)
    val mutBadPreview = (MUTANT_ID == MUT_ECS008_PREVIEW_WITHOUT_SWITCH and not(previewSw) and mutTrig)
    val mutMeasDerived = (MUTANT_ID == MUT_ECS009_UNEXECUTED_REFUSAL_MEASURED and scen == ScenNativeRejectedDerived and mutTrig)
    val mutPassDry = (MUTANT_ID == MUT_ECS015_PASS_ON_DRY_SETTLE and dispatchSw and not(previewSw) and mutTrig)

    val isPure = (scen == ScenPureH02)
    val isNatRej = (scen == ScenNativeRejectedDerived or scen == ScenNativeRejectedExecuted)
    val isCompileMis = (scen == ScenEligibleCompileMismatch)
    val kitBlocked = (scen == ScenEligibleKitCall and (not(kitInMachine) or mutBlockKit))
    val noNatPreviewRef = (scen == ScenEligibleNoNativePreviewRef)
    val canDispatch = (dispatchSw or mutBadDispatch)
    val canPreview = (previewSw or mutBadPreview)

    val compRan = not(isPure) and not(scen == ScenNativeRejectedDerived)
    val prvRan =
      if (isPure or isNatRej or kitBlocked) false
      else if (isCompileMis) mutProveAfterMis
      else canDispatch
    val proveSucceeded = prvRan and (scen != ScenEligibleProveFailed) and not(isCompileMis)
    val stlRan = proveSucceeded and not(noNatPreviewRef)
    val settledLive = stlRan and canPreview
    val settledDry = stlRan and not(canPreview)
    val cmpRan = stlRan

    val cls: CallResultClass =
      if (isPure) (if (mutBadClass) ClsPass else ClsNa)
      else if (isNatRej) ClsRefusedAsNative
      else if (isCompileMis) ClsMismatch
      else if (kitBlocked) ClsBlocked
      else if (not(canDispatch)) ClsBlocked
      else if (scen == ScenEligibleProveFailed) ClsProveFailed
      else if (noNatPreviewRef) ClsBlocked
      else if (settledDry) (if (mutPassDry) ClsPass else ClsBlocked)
      else ClsPass

    val reasonTxt =
      if (cls != ClsBlocked) "none"
      else if (mutEmptyReason) ""
      else if (kitBlocked) "kit missing from machine version"
      else if (not(canDispatch)) "dispatch off"
      else if (noNatPreviewRef) "no native Preview transcript"
      else "preview off"

    val execRej = (scen == ScenNativeRejectedExecuted)
    val evLbl: EvidenceLabel =
      if (not(isNatRej)) EvNotApplicable
      else if (execRej or mutMeasDerived) EvMeasured
      else EvDerived

    val nextCall: CallRunState = {
      evaluated: true,
      scenario: scen,
      allowDispatch: dispatchSw,
      allowPreview: previewSw,
      machineHoldsKit: kitInMachine,
      compileRan: compRan,
      proveRan: prvRan,
      settleRan: stlRan,
      compareRan: cmpRan,
      stageOrderValid: not(mutProveAfterMis),
      compileMatchedNative: not(isCompileMis),
      dispatchedToProverBox: prvRan,
      previewTxSubmitted: settledLive,
      walletOpened: settledLive,
      settledWithoutPreviewTx: settledDry,
      assignedClassCount: if (mutBadClass) 2 else 1,
      resultClass: cls,
      blockedReason: reasonTxt,
      blockedForEnabledKit: mutBlockKit,
      hasExecutedRslRejection: execRej,
      evidenceKind: evLbl
    }
    all {
      conf' = {
        ...conf,
        callRun: nextCall,
        livePassSeen: conf.livePassSeen or (cls == ClsPass and settledLive),
        dryPreviewBlockedSeen:
          conf.dryPreviewBlockedSeen or (settledDry and cls == ClsBlocked and reasonTxt == "preview off"),
        compileMismatchEarlyStopSeen:
          conf.compileMismatchEarlyStopSeen or (isCompileMis and cls == ClsMismatch and not(prvRan)),
        derivedAndMeasuredRefusalsSeen:
          conf.derivedAndMeasuredRefusalsSeen or (scen == ScenNativeRejectedExecuted and evLbl == EvMeasured)
      },
      lastOp' = "evaluateSuiteCall"
    }
  }

  // Implements ECS-010, ECS-011, ECS-012, ECS-014:
  // Aggregates suite table counts, mismatch denominator, and live-run completion gate over the 145 eligible calls.
  action doWriteSuiteSummaryAndCheckCompletion(
    provedCompCnt: int,
    mismatches: int,
    missingEvCnt: int,
    emptyVecBug: bool,
    mutTrig: bool
  ): bool = {
    val mutDoneOnMis = (MUTANT_ID == MUT_ECS010_DONE_ON_LIVE_MISMATCH and mismatches >= 1 and mutTrig)
    val mutBadCounts = (MUTANT_ID == MUT_ECS011_INCONSISTENT_SUITE_COUNTS and mutTrig)
    val mutNoDenom = (MUTANT_ID == MUT_ECS012_OMIT_MISMATCH_DENOMINATOR and mutTrig)
    val mutCompleteMissing =
      (MUTANT_ID == MUT_ECS014_COMPLETE_MISSING_EVIDENCE and (missingEvCnt >= 1 or emptyVecBug) and mutTrig)

    val trulyComplete =
      (provedCompCnt == SUITE_ELIGIBLE_PROOF_CALLS) and
      (mismatches == 0) and
      (missingEvCnt == 0) and
      not(emptyVecBug)
    val markComplete = trulyComplete or mutDoneOnMis or mutCompleteMissing

    val nextSum: SuiteSummaryState = {
      evaluated: true,
      reportedPrograms: COMPILED_SUITE_PROGRAMS,
      reportedCircuits: if (mutBadCounts) (SUITE_TOTAL_CIRCUITS - 1) else SUITE_TOTAL_CIRCUITS,
      reportedCalls: SUITE_TOTAL_CALLS,
      tableRowSum: if (mutBadCounts) (SUITE_TOTAL_CALLS - 1) else SUITE_TOTAL_CALLS,
      provedComparedCount: provedCompCnt,
      mismatchCount: mismatches,
      denominatorReportedWithMismatch: not(mutNoDenom),
      eligibleMissingEvidenceCount: missingEvCnt,
      emptyVectorComparedEqual: (emptyVecBug and mutCompleteMissing),
      liveTableFieldsPresent: not(mutDoneOnMis),
      liveRunMarkedComplete: markComplete
    }
    all {
      conf' = {
        ...conf,
        summary: nextSum,
        liveRunCompleteSeen: conf.liveRunCompleteSeen or (trulyComplete and markComplete)
      },
      lastOp' = "writeSuiteSummaryAndCheckCompletion"
    }
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    {
      nondet mutTrig = Set(true, false).oneOf()
      doBuildManifestAndNativeRefs(mutTrig)
    },
    {
      nondet scen = Set(
        ScenEligibleLivePass,
        ScenEligibleCompileMismatch,
        ScenEligibleProveFailed,
        ScenEligibleNoNativePreviewRef,
        ScenEligibleKitCall,
        ScenNativeRejectedDerived,
        ScenNativeRejectedExecuted,
        ScenPureH02
      ).oneOf()
      nondet dSw = Set(false, true).oneOf()
      nondet pSw = Set(false, true).oneOf()
      nondet kHas = Set(true, false).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doEvaluateSuiteCall(scen, dSw, pSw, kHas, mutTrig)
    },
    {
      nondet pCmp = Set(0, SUITE_ELIGIBLE_PROOF_CALLS).oneOf()
      nondet misC = Set(0, 1).oneOf()
      nondet missE = Set(0, ACTIVE_BATCH_SIZE).oneOf()
      nondet empBug = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doWriteSuiteSummaryAndCheckCompletion(pCmp, misC, missE, empBug, mutTrig)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per ECS-001..015)
  // =========================================================================

  val inv_ECS_001: bool =
    conf.manifest.manifestSchema == "t18-compact-suite-manifest/1" and
    conf.manifest.manifestProgramCount == MANIFEST_TOTAL_PROGRAMS and
    conf.manifest.compiledProgramCount == COMPILED_SUITE_PROGRAMS and
    conf.manifest.callsForUncompiledPrograms == 0

  val inv_ECS_002: bool =
    conf.manifest.nativeRefSchema == "mycroft-native-ref/1" and
    not(conf.manifest.rejectedRefHasTranscriptWords)

  val inv_ECS_003: bool =
    conf.callRun.evaluated implies (
      conf.callRun.stageOrderValid and
      (conf.callRun.proveRan implies conf.callRun.compileRan) and
      (conf.callRun.settleRan implies conf.callRun.proveRan) and
      (conf.callRun.compareRan implies conf.callRun.settleRan) and
      (not(conf.callRun.compileMatchedNative) implies
        (conf.callRun.resultClass == ClsMismatch and not(conf.callRun.proveRan)))
    )

  val inv_ECS_004: bool =
    conf.callRun.assignedClassCount == 1 and
    ((conf.callRun.evaluated and conf.callRun.scenario == ScenPureH02) implies
      (conf.callRun.resultClass == ClsNa))

  val inv_ECS_005: bool =
    (conf.callRun.evaluated and conf.callRun.resultClass == ClsBlocked) implies
      (conf.callRun.blockedReason != "" and conf.callRun.blockedReason != "none")

  val inv_ECS_006: bool =
    (conf.callRun.evaluated and conf.callRun.machineHoldsKit) implies
      not(conf.callRun.blockedForEnabledKit)

  val inv_ECS_007: bool =
    (conf.callRun.evaluated and not(conf.callRun.allowDispatch)) implies
      not(conf.callRun.dispatchedToProverBox)

  val inv_ECS_008: bool =
    (conf.callRun.evaluated and not(conf.callRun.allowPreview)) implies
      (not(conf.callRun.previewTxSubmitted) and not(conf.callRun.walletOpened))

  val inv_ECS_009: bool =
    (conf.callRun.evaluated and
      conf.callRun.resultClass == ClsRefusedAsNative and
      not(conf.callRun.hasExecutedRslRejection)) implies
      (conf.callRun.evidenceKind == EvDerived)

  val inv_ECS_010: bool =
    conf.summary.liveRunMarkedComplete implies (
      conf.summary.liveTableFieldsPresent and
      conf.summary.mismatchCount == 0
    )

  val inv_ECS_011: bool =
    conf.summary.reportedPrograms == COMPILED_SUITE_PROGRAMS and
    conf.summary.reportedCircuits == SUITE_TOTAL_CIRCUITS and
    conf.summary.reportedCalls == SUITE_TOTAL_CALLS and
    conf.summary.tableRowSum == SUITE_TOTAL_CALLS

  val inv_ECS_012: bool =
    conf.summary.denominatorReportedWithMismatch

  val inv_ECS_013: bool =
    conf.manifest.duplicateIdentityCount == 0 and
    conf.manifest.unmappedExecutedVectorCount == 0

  val inv_ECS_014: bool =
    not(conf.summary.emptyVectorComparedEqual) and
    (conf.summary.liveRunMarkedComplete implies (
      conf.summary.provedComparedCount == SUITE_ELIGIBLE_PROOF_CALLS and
      conf.summary.eligibleMissingEvidenceCount == 0 and
      conf.summary.mismatchCount == 0
    ))

  val inv_ECS_015: bool =
    (conf.callRun.evaluated and conf.callRun.settledWithoutPreviewTx) implies (
      conf.callRun.resultClass == ClsBlocked and
      conf.callRun.blockedReason == "preview off"
    )

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_ECS_001 and
    inv_ECS_002 and
    inv_ECS_003 and
    inv_ECS_004 and
    inv_ECS_005 and
    inv_ECS_006 and
    inv_ECS_007 and
    inv_ECS_008 and
    inv_ECS_009 and
    inv_ECS_010 and
    inv_ECS_011 and
    inv_ECS_012 and
    inv_ECS_013 and
    inv_ECS_014 and
    inv_ECS_015

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under conformance_main)
  // =========================================================================

  // Witness 1: Eligible call proves, settles on Preview (with switches enabled), compares equal -> ClsPass.
  val reach_live_eligible_call_passes: bool = not(
    conf.livePassSeen
  )

  // Witness 2: Dispatch ON + Preview OFF settles dry and classifies as blocked ("preview off") (ECS-015).
  val reach_dispatch_on_preview_off_blocked: bool = not(
    conf.dryPreviewBlockedSeen
  )

  // Witness 3: Stage-1 compile mismatch stops before prove and executed native refusal records MEASURED (ECS-003, ECS-009).
  val reach_compile_mismatch_stop_and_measured_refusal: bool = not(
    conf.compileMismatchEarlyStopSeen and conf.derivedAndMeasuredRefusalsSeen
  )

  // Witness 4: Full 145-call live run completes with 0 missing evidence items and 0 mismatches (ECS-010, ECS-014).
  val reach_full_145_live_run_complete: bool = not(
    conf.liveRunCompleteSeen and conf.manifest.evaluated
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module conformance_main {
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
  ).*
}

// Minimum batch bound (ACTIVE_BATCH_SIZE = 1 missing evidence item sufficient to block ECS-014 completion).
module conformance_edge_min {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36,
    COMPILED_SUITE_PROGRAMS = 33,
    SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1,
    SUITE_TOTAL_CALLS = 208,
    SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62,
    CALL_CLASSES_COUNT = 6,
    ACTIVE_BATCH_SIZE = 1,
    MUTANT_ID = 0
  ).*
}

// Upper batch bound (ACTIVE_BATCH_SIZE = 6 == CALL_CLASSES_COUNT).
module conformance_edge_at_limit {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36,
    COMPILED_SUITE_PROGRAMS = 33,
    SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1,
    SUITE_TOTAL_CALLS = 208,
    SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62,
    CALL_CLASSES_COUNT = 6,
    ACTIVE_BATCH_SIZE = 6,
    MUTANT_ID = 0
  ).*
}

// Guard mutants (15 mutants — one 1-to-1 mutant per ECS-001..015)

module conformance_m_ecs001_calls_for_uncompiled {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 1
  ).*
}

module conformance_m_ecs002_transcript_on_rejected_ref {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 2
  ).*
}

module conformance_m_ecs003_prove_after_compile_mismatch {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 3
  ).*
}

module conformance_m_ecs004_multi_or_bad_pure_class {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 4
  ).*
}

module conformance_m_ecs005_blocked_empty_reason {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 5
  ).*
}

module conformance_m_ecs006_block_enabled_kit {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 6
  ).*
}

module conformance_m_ecs007_dispatch_without_switch {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 7
  ).*
}

module conformance_m_ecs008_preview_without_switch {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 8
  ).*
}

module conformance_m_ecs009_unexecuted_refusal_measured {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 9
  ).*
}

module conformance_m_ecs010_done_on_live_mismatch {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 10
  ).*
}

module conformance_m_ecs011_inconsistent_suite_counts {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 11
  ).*
}

module conformance_m_ecs012_omit_mismatch_denominator {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 12
  ).*
}

module conformance_m_ecs013_duplicate_or_unmapped_id {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 13
  ).*
}

module conformance_m_ecs014_complete_missing_evidence {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 14
  ).*
}

module conformance_m_ecs015_pass_on_dry_settle {
  import conformance(
    MANIFEST_TOTAL_PROGRAMS = 36, COMPILED_SUITE_PROGRAMS = 33, SUITE_TOTAL_CIRCUITS = 106,
    SUITE_PURE_CIRCUITS = 1, SUITE_TOTAL_CALLS = 208, SUITE_ELIGIBLE_PROOF_CALLS = 145,
    SUITE_NATIVE_REJECTED_CALLS = 62, CALL_CLASSES_COUNT = 6, ACTIVE_BATCH_SIZE = 4, MUTANT_ID = 15
  ).*
}
