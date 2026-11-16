// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Call Flow (lane e2e-call-flow).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-user-flow/spec.md (EUF-001, 002, 005..014)
//   - openspec/changes/sherlock-completion-20261115/specs/sherlock-loop-completion/spec.md (LPC-014, LPC-017)
//   - openspec/changes/sherlock-e2e-prototype-20261115/design.md (Stage dependency table bound by EUF-002)
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md (Call Flow rows in sections 1, 2, 5, 8)

module call_flow {
  // =========================================================================
  // Constants (stage counts, version pins, eligible calls, and fleet/loop sizes)
  // =========================================================================

  // EUF-001: count of ordered stages named in EUF-001 (11).
  const EUF001_STAGE_COUNT: int
  // EUF-002 Scenario: count of per-call stages from `tape` to `read-back` on a second call (7).
  const SECOND_CALL_STAGE_COUNT: int
  // EUF-012: count of version/identity pins checked per run (`compactc`, `ZKIR`, `machine_version`, `keys_d`, `ledger` => 5).
  const PIN_DIMENSIONS_COUNT: int
  // EUF-013: eligible suite calls that are neither refused-as-native nor n/a (145).
  const ELIGIBLE_SUITE_CALLS: int
  // EUF-011: maintainer/factory tools permitted on a clean user box (0).
  const MAINTAINER_TOOLS_ALLOWED: int
  //LOOP row count varied across concrete instances (1..25).
  const LOOP_ROW_COUNT: int
  // Mutant selector: 0 = honest specification; 1..14 = one 1-to-1 mutant per requirement.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_EUF001_OUT_OF_ORDER_STAGE: int = 1
  pure val MUT_EUF002_MISSING_INPUT_OR_REDEPLOY: int = 2
  pure val MUT_EUF005_SUBMIT_AFTER_FAILED_STAGE: int = 3
  pure val MUT_EUF006_PROVE_REJECTED_COMPACT_CALL: int = 4
  pure val MUT_EUF007_PROVE_WITHOUT_BOX_PROFILE: int = 5
  pure val MUT_EUF008_BOX_DEPENDENT_CLAIM_PUB: int = 6
  pure val MUT_EUF009_SECOND_PROG_DIFFERENT_VK: int = 7
  pure val MUT_EUF010_ACCEPT_MISMATCHED_READBACK: int = 8
  pure val MUT_EUF011_REQUIRE_MAINTAINER_TOOL: int = 9
  pure val MUT_EUF012_CONTINUE_ON_PIN_MISMATCH: int = 10
  pure val MUT_EUF013_COMPLETE_PARTIAL_OR_SPLIT_KEYS: int = 11
  pure val MUT_EUF014_RESUBMIT_UNREAD_CALL: int = 12
  pure val MUT_LPC014_FREEZE_WITHOUT_VERDICT: int = 13
  pure val MUT_LPC017_KEEP_STALE_DOM_LOCK_DOCS: int = 14

  // =========================================================================
  // Canonical stage lists & domain types
  // =========================================================================

  pure val EUF001_STAGES: List[str] = List(
    "compile", "setup", "plan", "deploy", "tape",
    "execute", "prove", "wrap", "ledger_check", "submit", "read_back"
  )

  pure val DESIGN_DEP_TABLE_STAGES: List[str] = List(
    "compile", "plan_and_admit", "deploy", "setup", "tape",
    "execute", "prove", "wrap", "ledger_check", "build_arguments",
    "submit", "accept_and_read_back", "compare"
  )

  type FlowOutcomeCase =
    | FlowFirstCallHappy
    | FlowSecondCallFromTape
    | FlowMissingWrapArtifactStop
    | FlowLedgerCheckRefusedStop
    | FlowCompactExecuteRejected
    | FlowReadbackMismatch

  type PipelineRunState = {
    evaluated: bool,
    flowCase: FlowOutcomeCase,
    isSecondCallOfDeployed: bool,
    stagesInCanonicalOrder: bool,
    stageInputsSatisfied: bool,
    missingArtifactStoppedCleanly: bool,
    deploymentsSentThisCall: int,
    executeRejectedCompact: bool,
    preSubmitFailed: bool,
    proveStageStarted: bool,
    submitStageStarted: bool,
    txSentCount: int,
    recordedStopStageAndError: bool,
    readbackArrived: bool,
    readbackEqualsOutcomePub: bool,
    outcomeEqualsNativeWords: bool,
    readbackVerifiedOk: bool,
    maintainerToolsUsed: int
  }

  type PinsAndBoxesState = {
    evaluated: bool,
    installedKeySetId: int,
    boxProfileKeySetId: int,
    boxHasMatchingProfile: bool,
    proveAllowedOnBox: bool,
    differingPinName: str,
    stoppedAndNamedPin: bool,
    continuedPastPinMismatch: bool,
    claimPubBoxA: int,
    claimPubBoxB: int,
    firstProgWrapVk: int,
    secondProgWrapVk: int,
    progAnchorsDiffer: bool
  }

  type ReconcileState = {
    evaluated: bool,
    submittedHasNoReadback: bool,
    reconciledByTxHash: bool,
    txCountForClaimPub: int
  }

  type FreezeAndDocsState = {
    evaluated: bool,
    joinedEligibleCallsCount: int,
    universalKeySetsInEvidence: int,
    prototypeReportedComplete: bool,
    totalRows: int,
    readyOrOwnerWaitingRows: int,
    freezePresented: bool,
    ownerRulingChangedLockForm: bool,
    domLockHitsInDesignDocs: int
  }

  type CallFlowState = {
    pipeline: PipelineRunState,
    pinsBoxes: PinsAndBoxesState,
    reconcile: ReconcileState,
    completion: FreezeAndDocsState,
    // Monotone reachability flags across random walks
    firstCallSettledSeen: bool,
    secondCallFromTapeSeen: bool,
    preSubmitAndExecRejectStopsSeen: bool,
    reconciledUnreadSubmitSeen: bool,
    prototypeAndFreezeCompleteSeen: bool
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var flow: CallFlowState
  var lastOp: str

  // =========================================================================
  // Pure consistency & spec-gap witness predicates
  // =========================================================================

  val consistency_constants: bool =
    EUF001_STAGE_COUNT == 11 and
    EUF001_STAGES.length() == EUF001_STAGE_COUNT and
    SECOND_CALL_STAGE_COUNT == 7 and
    SECOND_CALL_STAGE_COUNT < EUF001_STAGE_COUNT and
    PIN_DIMENSIONS_COUNT == 5 and
    ELIGIBLE_SUITE_CALLS == 145 and
    MAINTAINER_TOOLS_ALLOWED == 0 and
    LOOP_ROW_COUNT >= 1 and
    LOOP_ROW_COUNT <= 25

  // Finding Q-call-flow-1 witness:
  // EUF-001 lists 11 stages (`compile, setup, plan, deploy, ...`), whereas `design.md`'s Stage Dependency Table
  // (normatively bound by EUF-002) lists 13 stages (`compile, plan_and_admit, deploy, setup, ..., build_arguments,
  // submit, accept_and_read_back, compare`):
  //   (a) stage index 1 differs (`setup` in EUF-001 vs `plan_and_admit` in `design.md`);
  //   (b) `EUF-001` omits `build_arguments` (the sole producer of `StatementPublics` required by `submit`);
  //   (c) `EUF-001` omits `compare` (the sole reader of `NativeCallReference` required by `EUF-010`).
  pure val gap_q_cf_1_euf001_vs_design_dep_table_mismatch: bool = {
    val eufSet = Set(
      "compile", "setup", "plan", "deploy", "tape",
      "execute", "prove", "wrap", "ledger_check", "submit", "read_back"
    )
    EUF001_STAGES.length() != DESIGN_DEP_TABLE_STAGES.length() and
      EUF001_STAGES[1] != DESIGN_DEP_TABLE_STAGES[1] and
      not(eufSet.contains("build_arguments")) and
      not(eufSet.contains("compare"))
  }

  // Finding Q-call-flow-2 witness:
  // Literal EUF-001 requires all 11 stages (including `deploy`) on "one end-to-end call", whereas EUF-002 Scenario
  // ("Second call") starts at `tape` (7 stages) and sends 0 deployments.
  val gap_q_cf_2_first_call_11_stages_vs_second_call_7_stages: bool =
    EUF001_STAGE_COUNT == 11 and SECOND_CALL_STAGE_COUNT == 7 and (EUF001_STAGE_COUNT != SECOND_CALL_STAGE_COUNT)

  // Finding Q-call-flow-3 witness:
  // On a clean external developer machine (EUF-011) or an interpreted-family Sherlock program, no native Compact
  // reference transcript exists (`hasNativeTranscript == false`), making unconditional EUF-010 unexecutable.
  pure val gap_q_cf_3_euf010_requires_absent_native_transcript_on_user_call: bool = {
    val cleanUserCallOutsideSuite = true
    val hasNativeTranscriptRef = false
    cleanUserCallOutsideSuite and not(hasNativeTranscriptRef)
  }

  // Finding Q-call-flow-4 witness:
  // When a submitted tx is refused by the node or expires its TTL, `hasReadback == false`; literal EUF-014's
  // permanent ban on sending a second tx for the same `claim_pub` prevents retrying the deterministic `claim_pub`
  // of EUF-008 even after reconciliation confirms permanent non-inclusion.
  pure val gap_q_cf_4_euf014_blocks_retry_after_resolved_non_inclusion: bool = {
    val hasReadback = false
    val reconciledStatusIsExpiredOrNodeRefused = true
    val claimPubDeterministicByEuf008 = true
    not(hasReadback) and reconciledStatusIsExpiredOrNodeRefused and claimPubDeterministicByEuf008
  }

  // Finding Q-call-flow-5 witness:
  // Under LPC-014 ("every row is DONE with a lead verdict or waits on the owner"), a critical-path row (e.g. T23)
  // in `waitsOnOwner` satisfies LPC-014 (`canPresentT15ByLpc014 == true`) while `joinedCalls < 145`, violating
  // EUF-013 Scenario's requirement that T15 present completed suite/gate/Preview receipts.
  val gap_q_cf_5_lpc014_owner_wait_vs_euf013_freeze_evidence: bool = {
    val t23WaitsOnOwner = true
    val allOtherRowsDoneWithVerdict = true
    val joinedSuiteCallsCompleted = 0
    val canPresentT15ByLpc014 = (t23WaitsOnOwner and allOtherRowsDoneWithVerdict)
    canPresentT15ByLpc014 and (joinedSuiteCallsCompleted < ELIGIBLE_SUITE_CALLS)
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    flow' = {
      pipeline: {
        evaluated: false,
        flowCase: FlowFirstCallHappy,
        isSecondCallOfDeployed: false,
        stagesInCanonicalOrder: true,
        stageInputsSatisfied: true,
        missingArtifactStoppedCleanly: true,
        deploymentsSentThisCall: 0,
        executeRejectedCompact: false,
        preSubmitFailed: false,
        proveStageStarted: false,
        submitStageStarted: false,
        txSentCount: 0,
        recordedStopStageAndError: true,
        readbackArrived: false,
        readbackEqualsOutcomePub: true,
        outcomeEqualsNativeWords: true,
        readbackVerifiedOk: false,
        maintainerToolsUsed: 0
      },
      pinsBoxes: {
        evaluated: false,
        installedKeySetId: 25,
        boxProfileKeySetId: 25,
        boxHasMatchingProfile: true,
        proveAllowedOnBox: true,
        differingPinName: "none",
        stoppedAndNamedPin: true,
        continuedPastPinMismatch: false,
        claimPubBoxA: 701,
        claimPubBoxB: 701,
        firstProgWrapVk: 901,
        secondProgWrapVk: 901,
        progAnchorsDiffer: true
      },
      reconcile: {
        evaluated: false,
        submittedHasNoReadback: false,
        reconciledByTxHash: true,
        txCountForClaimPub: 1
      },
      completion: {
        evaluated: false,
        joinedEligibleCallsCount: 0,
        universalKeySetsInEvidence: 1,
        prototypeReportedComplete: false,
        totalRows: LOOP_ROW_COUNT,
        readyOrOwnerWaitingRows: 0,
        freezePresented: false,
        ownerRulingChangedLockForm: true,
        domLockHitsInDesignDocs: 0
      },
      firstCallSettledSeen: false,
      secondCallFromTapeSeen: false,
      preSubmitAndExecRejectStopsSeen: false,
      reconciledUnreadSubmitSeen: false,
      prototypeAndFreezeCompleteSeen: false
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Implements EUF-001, EUF-002, EUF-005, EUF-006, EUF-010, EUF-011:
  // Executes one end-to-end call flow (first call, second call from tape, missing wrap input, muninn wrap refusal,
  // execute-stage Compact rejection, or read-back comparison).
  action doRunEndToEndCallPipeline(fCase: FlowOutcomeCase, mutTrig: bool): bool = {
    val mutOutOrder = (MUTANT_ID == MUT_EUF001_OUT_OF_ORDER_STAGE and mutTrig)
    val mutBadInputOrRedeploy = (MUTANT_ID == MUT_EUF002_MISSING_INPUT_OR_REDEPLOY and mutTrig)
    val mutSubmitOnFail = (MUTANT_ID == MUT_EUF005_SUBMIT_AFTER_FAILED_STAGE and mutTrig)
    val mutProveRejected = (MUTANT_ID == MUT_EUF006_PROVE_REJECTED_COMPACT_CALL and fCase == FlowCompactExecuteRejected and mutTrig)
    val mutAcceptBadReadback = (MUTANT_ID == MUT_EUF010_ACCEPT_MISMATCHED_READBACK and fCase == FlowReadbackMismatch and mutTrig)
    val mutMaintainerTool = (MUTANT_ID == MUT_EUF011_REQUIRE_MAINTAINER_TOOL and mutTrig)

    val isSecond = (fCase == FlowSecondCallFromTape)
    val isMissingWrapIn = (fCase == FlowMissingWrapArtifactStop)
    val isLedgerRefused = (fCase == FlowLedgerCheckRefusedStop)
    val isExecRej = (fCase == FlowCompactExecuteRejected)
    val isRbMismatch = (fCase == FlowReadbackMismatch)
    val preSubFail = (isMissingWrapIn or isLedgerRefused or isExecRej)

    val deploysSent =
      if (isSecond) (if (mutBadInputOrRedeploy) 1 else 0)
      else 1

    val prvStarted =
      if (isExecRej) mutProveRejected
      else true

    val subStarted =
      if (preSubFail) (mutSubmitOnFail or mutProveRejected)
      else true

    val txCnt = if (subStarted) 1 else 0
    val rbArrived = (fCase == FlowFirstCallHappy or fCase == FlowSecondCallFromTape or fCase == FlowReadbackMismatch)
    val rbMatch = not(isRbMismatch)
    val rbOk = if (not(rbArrived)) false else (rbMatch or mutAcceptBadReadback)

    val nextPipe: PipelineRunState = {
      evaluated: true,
      flowCase: fCase,
      isSecondCallOfDeployed: isSecond,
      stagesInCanonicalOrder: not(mutOutOrder),
      stageInputsSatisfied: not(isMissingWrapIn and mutBadInputOrRedeploy),
      missingArtifactStoppedCleanly: if (isMissingWrapIn) not(mutBadInputOrRedeploy) else true,
      deploymentsSentThisCall: deploysSent,
      executeRejectedCompact: isExecRej,
      preSubmitFailed: preSubFail,
      proveStageStarted: prvStarted,
      submitStageStarted: subStarted,
      txSentCount: txCnt,
      recordedStopStageAndError: if (preSubFail) not(mutSubmitOnFail) else true,
      readbackArrived: rbArrived,
      readbackEqualsOutcomePub: rbMatch,
      outcomeEqualsNativeWords: rbMatch,
      readbackVerifiedOk: rbOk,
      maintainerToolsUsed: if (mutMaintainerTool) 1 else 0
    }
    all {
      flow' = {
        ...flow,
        pipeline: nextPipe,
        firstCallSettledSeen:
          flow.firstCallSettledSeen or (fCase == FlowFirstCallHappy and rbOk and not(mutOutOrder)),
        secondCallFromTapeSeen:
          flow.secondCallFromTapeSeen or (isSecond and deploysSent == 0 and rbOk),
        preSubmitAndExecRejectStopsSeen:
          flow.preSubmitAndExecRejectStopsSeen or (preSubFail and txCnt == 0)
      },
      lastOp' = "runEndToEndCallPipeline"
    }
  }

  // Implements EUF-007, EUF-008, EUF-009, EUF-012:
  // Checks box profile key-set binding, 5-dimension version pins, cross-box claim_pub equality,
  // and byte-equal wrap VK across multi-program deployments.
  action doCheckProfilePinsClaimAndMultiDeploy(
    profileMatchesKeys: bool,
    pinMismatchKind: str,
    mutTrig: bool
  ): bool = {
    val mutNoProfile = (MUTANT_ID == MUT_EUF007_PROVE_WITHOUT_BOX_PROFILE and not(profileMatchesKeys) and mutTrig)
    val mutClaimDiff = (MUTANT_ID == MUT_EUF008_BOX_DEPENDENT_CLAIM_PUB and mutTrig)
    val mutVkDiff = (MUTANT_ID == MUT_EUF009_SECOND_PROG_DIFFERENT_VK and mutTrig)
    val mutPinCont = (MUTANT_ID == MUT_EUF012_CONTINUE_ON_PIN_MISMATCH and pinMismatchKind != "none" and mutTrig)

    val nextPB: PinsAndBoxesState = {
      evaluated: true,
      installedKeySetId: 25,
      boxProfileKeySetId: if (profileMatchesKeys) 25 else 10,
      boxHasMatchingProfile: profileMatchesKeys,
      proveAllowedOnBox: (profileMatchesKeys or mutNoProfile),
      differingPinName: pinMismatchKind,
      stoppedAndNamedPin: if (pinMismatchKind != "none") not(mutPinCont) else true,
      continuedPastPinMismatch: mutPinCont,
      claimPubBoxA: 701,
      claimPubBoxB: if (mutClaimDiff) 799 else 701,
      firstProgWrapVk: 901,
      secondProgWrapVk: if (mutVkDiff) 902 else 901,
      progAnchorsDiffer: true
    }
    all {
      flow' = {
        ...flow,
        pinsBoxes: nextPB
      },
      lastOp' = "checkProfilePinsClaimAndMultiDeploy"
    }
  }

  // Implements EUF-014:
  // Reconciles a submitted call lacking read-back by its transaction hash without sending a second transaction.
  action doReconcileSubmittedCall(missingReadback: bool, mutTrig: bool): bool = {
    val mutResubmit = (MUTANT_ID == MUT_EUF014_RESUBMIT_UNREAD_CALL and missingReadback and mutTrig)
    val nextRec: ReconcileState = {
      evaluated: true,
      submittedHasNoReadback: missingReadback,
      reconciledByTxHash: not(mutResubmit),
      txCountForClaimPub: if (mutResubmit) 2 else 1
    }
    all {
      flow' = {
        ...flow,
        reconcile: nextRec,
        reconciledUnreadSubmitSeen:
          flow.reconciledUnreadSubmitSeen or (missingReadback and not(mutResubmit))
      },
      lastOp' = "reconcileSubmittedCall"
    }
  }

  // Implements EUF-013, LPC-014, LPC-017:
  // Evaluates full 145-call prototype completion under 1 universal key set, T15 freeze presentation,
  // and design-document alignment with owner formula rulings (`DOM_LOCK` removal).
  action doEvaluatePrototypeFreezeAndDocs(
    all145Joined: bool,
    singleKeySet: bool,
    allRowsReady: bool,
    mutTrig: bool
  ): bool = {
    val mutBadProto =
      (MUTANT_ID == MUT_EUF013_COMPLETE_PARTIAL_OR_SPLIT_KEYS and (not(all145Joined) or not(singleKeySet)) and mutTrig)
    val mutBadFreeze = (MUTANT_ID == MUT_LPC014_FREEZE_WITHOUT_VERDICT and not(allRowsReady) and mutTrig)
    val mutStaleDocs = (MUTANT_ID == MUT_LPC017_KEEP_STALE_DOM_LOCK_DOCS and mutTrig)

    val joinedCnt = if (all145Joined) ELIGIBLE_SUITE_CALLS else 60
    val keySetCnt = if (singleKeySet) 1 else 2
    val readyCnt = if (allRowsReady) LOOP_ROW_COUNT else (LOOP_ROW_COUNT - 1)
    val protoDone = (all145Joined and singleKeySet) or mutBadProto
    val freezePres = allRowsReady or mutBadFreeze

    val nextComp: FreezeAndDocsState = {
      evaluated: true,
      joinedEligibleCallsCount: joinedCnt,
      universalKeySetsInEvidence: keySetCnt,
      prototypeReportedComplete: protoDone,
      totalRows: LOOP_ROW_COUNT,
      readyOrOwnerWaitingRows: readyCnt,
      freezePresented: freezePres,
      ownerRulingChangedLockForm: true,
      domLockHitsInDesignDocs: if (mutStaleDocs) 2 else 0
    }
    all {
      flow' = {
        ...flow,
        completion: nextComp,
        prototypeAndFreezeCompleteSeen:
          flow.prototypeAndFreezeCompleteSeen or
            (all145Joined and singleKeySet and allRowsReady and not(mutStaleDocs))
      },
      lastOp' = "evaluatePrototypeFreezeAndDocs"
    }
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    {
      nondet fCase = Set(
        FlowFirstCallHappy,
        FlowSecondCallFromTape,
        FlowMissingWrapArtifactStop,
        FlowLedgerCheckRefusedStop,
        FlowCompactExecuteRejected,
        FlowReadbackMismatch
      ).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRunEndToEndCallPipeline(fCase, mutTrig)
    },
    {
      nondet profOk = Set(true, false).oneOf()
      nondet pinKind = Set("none", "zkir", "keys_d", "ledger").oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doCheckProfilePinsClaimAndMultiDeploy(profOk, pinKind, mutTrig)
    },
    {
      nondet missRb = Set(true, false).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doReconcileSubmittedCall(missRb, mutTrig)
    },
    {
      nondet j145 = Set(true, false).oneOf()
      nondet oneK = Set(true, false).oneOf()
      nondet rowsOk = Set(true, false).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doEvaluatePrototypeFreezeAndDocs(j145, oneK, rowsOk, mutTrig)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per own + completion ID)
  // =========================================================================

  val inv_EUF_001: bool =
    flow.pipeline.stagesInCanonicalOrder

  val inv_EUF_002: bool =
    flow.pipeline.stageInputsSatisfied and
    flow.pipeline.missingArtifactStoppedCleanly and
    ((flow.pipeline.evaluated and flow.pipeline.isSecondCallOfDeployed) implies
      (flow.pipeline.deploymentsSentThisCall == 0))

  val inv_EUF_005: bool =
    (flow.pipeline.evaluated and flow.pipeline.preSubmitFailed) implies (
      not(flow.pipeline.submitStageStarted) and
      flow.pipeline.txSentCount == 0 and
      flow.pipeline.recordedStopStageAndError
    )

  val inv_EUF_006: bool =
    (flow.pipeline.evaluated and flow.pipeline.executeRejectedCompact) implies (
      not(flow.pipeline.proveStageStarted) and
      not(flow.pipeline.submitStageStarted) and
      flow.pipeline.txSentCount == 0
    )

  val inv_EUF_007: bool =
    (flow.pinsBoxes.evaluated and not(flow.pinsBoxes.boxHasMatchingProfile)) implies
      not(flow.pinsBoxes.proveAllowedOnBox)

  val inv_EUF_008: bool =
    flow.pinsBoxes.claimPubBoxA == flow.pinsBoxes.claimPubBoxB

  val inv_EUF_009: bool =
    flow.pinsBoxes.firstProgWrapVk == flow.pinsBoxes.secondProgWrapVk and
    flow.pinsBoxes.progAnchorsDiffer

  val inv_EUF_010: bool =
    flow.pipeline.readbackVerifiedOk implies (
      flow.pipeline.readbackArrived and
      flow.pipeline.readbackEqualsOutcomePub and
      flow.pipeline.outcomeEqualsNativeWords
    )

  val inv_EUF_011: bool =
    flow.pipeline.maintainerToolsUsed <= MAINTAINER_TOOLS_ALLOWED

  val inv_EUF_012: bool =
    (flow.pinsBoxes.evaluated and flow.pinsBoxes.differingPinName != "none") implies (
      flow.pinsBoxes.stoppedAndNamedPin and
      not(flow.pinsBoxes.continuedPastPinMismatch)
    )

  val inv_EUF_013: bool =
    flow.completion.prototypeReportedComplete implies (
      flow.completion.joinedEligibleCallsCount == ELIGIBLE_SUITE_CALLS and
      flow.completion.universalKeySetsInEvidence == 1
    )

  val inv_EUF_014: bool =
    (flow.reconcile.evaluated and flow.reconcile.submittedHasNoReadback) implies (
      flow.reconcile.reconciledByTxHash and
      flow.reconcile.txCountForClaimPub == 1
    )

  val inv_LPC_014: bool =
    flow.completion.freezePresented implies
      (flow.completion.readyOrOwnerWaitingRows == flow.completion.totalRows)

  val inv_LPC_017: bool =
    flow.completion.ownerRulingChangedLockForm implies
      (flow.completion.domLockHitsInDesignDocs == 0)

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_EUF_001 and
    inv_EUF_002 and
    inv_EUF_005 and
    inv_EUF_006 and
    inv_EUF_007 and
    inv_EUF_008 and
    inv_EUF_009 and
    inv_EUF_010 and
    inv_EUF_011 and
    inv_EUF_012 and
    inv_EUF_013 and
    inv_EUF_014 and
    inv_LPC_014 and
    inv_LPC_017

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under call_flow_main)
  // =========================================================================

  // Witness 1: Full first-call flow settles on Preview and second call resumes from `tape` with 0 deploys.
  val reach_first_and_second_call_settle: bool = not(
    flow.firstCallSettledSeen and flow.secondCallFromTapeSeen
  )

  // Witness 2: Pre-submit failure and execute-stage Compact rejection both halt before transaction send.
  val reach_pre_submit_and_execute_reject_stops: bool = not(
    flow.preSubmitAndExecRejectStopsSeen
  )

  // Witness 3: Missing-readback submitted call reconciles by tx hash with 1 tx for claim_pub (EUF-014).
  val reach_reconciled_unread_submission: bool = not(
    flow.reconciledUnreadSubmitSeen
  )

  // Witness 4: All 145 eligible suite calls complete under 1 universal key set and T15 freeze presents cleanly.
  val reach_prototype_complete_and_freeze_presented: bool = not(
    flow.prototypeAndFreezeCompleteSeen and flow.pinsBoxes.evaluated
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module call_flow_main {
  import call_flow(
    EUF001_STAGE_COUNT = 11,
    SECOND_CALL_STAGE_COUNT = 7,
    PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145,
    MAINTAINER_TOOLS_ALLOWED = 0,
    LOOP_ROW_COUNT = 5,
    MUTANT_ID = 0
  ).*
}

// Minimum loop-row boundary (LOOP_ROW_COUNT = 1 single unverified row blocks LPC-014 freeze).
module call_flow_edge_min {
  import call_flow(
    EUF001_STAGE_COUNT = 11,
    SECOND_CALL_STAGE_COUNT = 7,
    PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145,
    MAINTAINER_TOOLS_ALLOWED = 0,
    LOOP_ROW_COUNT = 1,
    MUTANT_ID = 0
  ).*
}

// Full 25-row LOOP table boundary (T1..T25 => LOOP_ROW_COUNT = 25).
module call_flow_edge_at_limit {
  import call_flow(
    EUF001_STAGE_COUNT = 11,
    SECOND_CALL_STAGE_COUNT = 7,
    PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145,
    MAINTAINER_TOOLS_ALLOWED = 0,
    LOOP_ROW_COUNT = 25,
    MUTANT_ID = 0
  ).*
}

// Guard mutants (14 mutants — one 1-to-1 mutant per own + completion ID)

module call_flow_m_euf001_out_of_order_stage {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 1
  ).*
}

module call_flow_m_euf002_missing_input_or_redeploy {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 2
  ).*
}

module call_flow_m_euf005_submit_after_failed_stage {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 3
  ).*
}

module call_flow_m_euf006_prove_rejected_compact_call {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 4
  ).*
}

module call_flow_m_euf007_prove_without_box_profile {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 5
  ).*
}

module call_flow_m_euf008_box_dependent_claim_pub {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 6
  ).*
}

module call_flow_m_euf009_second_prog_different_vk {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 7
  ).*
}

module call_flow_m_euf010_accept_mismatched_readback {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 8
  ).*
}

module call_flow_m_euf011_require_maintainer_tool {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 9
  ).*
}

module call_flow_m_euf012_continue_on_pin_mismatch {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 10
  ).*
}

module call_flow_m_euf013_complete_partial_or_split_keys {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 11
  ).*
}

module call_flow_m_euf014_resubmit_unread_call {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 12
  ).*
}

module call_flow_m_lpc014_freeze_without_verdict {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 13
  ).*
}

module call_flow_m_lpc017_keep_stale_dom_lock_docs {
  import call_flow(
    EUF001_STAGE_COUNT = 11, SECOND_CALL_STAGE_COUNT = 7, PIN_DIMENSIONS_COUNT = 5,
    ELIGIBLE_SUITE_CALLS = 145, MAINTAINER_TOOLS_ALLOWED = 0, LOOP_ROW_COUNT = 5, MUTANT_ID = 14
  ).*
}
