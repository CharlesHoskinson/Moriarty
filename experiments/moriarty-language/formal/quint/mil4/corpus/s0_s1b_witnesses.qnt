// S1B finite local observations. Every tuple is stipulated, never authenticated here.
module s0_s1b_witnesses {
  import s0(
    signatureVerified = Set("digest-T-10-1", "digest-R-30"),
    authenticatedSnapshots = Set(0),
    nativeQualified = Set("digest-T-10-1", "digest-R-30"),
    ledgerAtomicReady = true, executingDomain = "D", settlementAsset = "A",
    initialWorkBudget = 1
  ).* from "../s0"

  pure val t: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A",
    digest: "digest-T-10-1", policyDigest: "p", evidenceChoice: "e",
    signer: "O", payer: "O", recipient: "R", feeRecipient: "F",
    obligationId: "", debtor: "", creditor: "", nonce: "T-10-1",
    preHead: 0, validFrom: 0, validThrough: 10,
    grossCap: 11, feeCap: 1, netFloor: 0, actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0, roundingNone: true,
    terminalOnly: true,
  }

  pure val pre: FinancialSnapshot = {
    balances: Map("O" -> 100, "R" -> 0, "F" -> 0),
    allowanceRemaining: Map("O" -> 11), allowanceSpent: Map("O" -> 0),
    obligations: Map(), head: 0, consumed: Set(),
    workRemaining: 1, workSpent: 0,
  }
  pure val terminal: RequestedOutcome = {
    phase: TerminalSuccess, retainedEffects: List(), retainedDuties: List(),
  }
  pure val tuple: StipulatedTuple = {
    available: true, intent: t, preState: pre, round: 0, expectedSuccessor: 1,
    requestedOutcome: terminal,
  }
  pure val request: ComparisonRequest = {
    submittedSuccessor: 1, outcome: terminal, premise: tuple,
  }
  pure def effects(before: int, after: int): List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    Credit({ account: "F", amount: 1 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(("D", "O", "T-10-1")),
    AdvanceHead({ before: before, after: after })
  )
  pure val expected: List[EffectLine] = effects(0, 1)

  action ready(i: SignedIntent): bool =
    init.then(seedAccount("O", 100, 11, 0))
      .then(seedBalanceOnly("R", 0)).then(seedBalanceOnly("F", 0))
      .then(sign(i))

  // Malformed-cell and consumed-replay fixtures bypass seed guards deliberately.
  action replacePre(p: FinancialSnapshot): bool = all {
    balances' = p.balances, allowanceRemaining' = p.allowanceRemaining,
    allowanceSpent' = p.allowanceSpent, obligations' = p.obligations,
    currentHead' = p.head, consumed' = p.consumed,
    workRemaining' = p.workRemaining, workSpent' = p.workSpent,
    signed' = signed, round' = round, lastEffects' = lastEffects,
    committedCount' = committedCount,
  }

  def rejected(i: SignedIntent, p: FinancialSnapshot, req: ComparisonRequest,
               lines: List[EffectLine], j: Judgment, c: str): bool = all {
    transferObservation("digest-T-10-1", i, i.preHead, 10, 1, lines, req) == {
      accepted: false, judgment: j, code: c, diagnosticWork: 1,
    },
    financialSnapshot == p,
    signed == Map("digest-T-10-1" -> i),
    round == 0 and committedCount == 0 and lastEffects == List(),
  }

  run H1Test = ready(t)
    .expect(all {
      financialSnapshot == pre,
      transferObservation("digest-T-10-1", t, 0, 10, 1, expected, request) == {
        accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
      },
    })
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, request))
    .expect(all {
      financialSnapshot == {
        balances: Map("O" -> 89, "R" -> 10, "F" -> 1),
        allowanceRemaining: Map("O" -> 0), allowanceSpent: Map("O" -> 11),
        obligations: Map(), head: 1, consumed: Set(("D", "O", "T-10-1")),
        workRemaining: 0, workSpent: 1,
      },
      signed == Map("digest-T-10-1" -> t),
      round == 0 and committedCount == 1 and lastEffects == expected,
    })

  pure val wrongHead = { ...request, submittedSuccessor: 9 }
  pure val selfHead = { ...request, submittedSuccessor: 0 }
  run H2Test = ready(t)
    .expect(rejected(t, pre, wrongHead, effects(0, 9), History, "S0_HISTORY_SUCCESSOR"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), wrongHead).fail())

  run H3Test = ready(t)
    .expect(rejected(t, pre, selfHead, effects(0, 0), History, "S0_HISTORY_SUCCESSOR"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 0), selfHead).fail())

  run H4Test = ready(t)
    .expect(rejected(t, pre, request, effects(0, 9), Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), request).fail())

  pure val stale = { ...t, preHead: 9 }
  pure val staleRequest = {
    ...wrongHead, premise: { ...tuple, intent: stale },
  }
  run H5Test = ready(stale)
    .expect(rejected(stale, pre, staleRequest, effects(9, 9), History, "S0_HISTORY_STALE"))
    .then(submitTransfer("digest-T-10-1", stale, stale.preHead, 10, 1,
      effects(9, 9), staleRequest).fail())

  pure val replayPre = { ...pre, consumed: Set(("D", "O", "T-10-1")) }
  pure val replayRequest = {
    ...wrongHead, premise: { ...tuple, preState: replayPre },
  }
  run H6Test = ready(t)
    .then(replacePre(replayPre))
    .expect(rejected(t, replayPre, replayRequest, effects(0, 9), History, "S0_HISTORY_REPLAY"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), replayRequest).fail())

  pure val failureOutcome = { ...terminal, phase: RequestedFailure }
  pure val retainedEffectOutcome = {
    ...terminal, retainedEffects: List(Credit({ account: "R", amount: 10 })),
  }
  pure val retainedDutyOutcome = {
    ...terminal, retainedDuties: List({ id: "d", owner: "O", amount: 1 }),
  }
  pure val failureRequest = {
    ...request, outcome: failureOutcome,
    premise: { ...tuple, requestedOutcome: failureOutcome },
  }
  pure val retainedEffectRequest = {
    ...request, outcome: retainedEffectOutcome,
    premise: { ...tuple, requestedOutcome: retainedEffectOutcome },
  }
  pure val retainedDutyRequest = {
    ...request, outcome: retainedDutyOutcome,
    premise: { ...tuple, requestedOutcome: retainedDutyOutcome },
  }
  run F1Test = ready(t)
    .expect(rejected(t, pre, failureRequest, expected, Failure, "S0_FAILURE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, failureRequest).fail())

  run F2Test = ready(t)
    .expect(rejected(t, pre, retainedEffectRequest, expected, Failure, "S0_FAILURE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, retainedEffectRequest).fail())

  run F3Test = ready(t)
    .expect(rejected(t, pre, retainedDutyRequest, expected, Failure, "S0_FAILURE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, retainedDutyRequest).fail())

  pure val unavailableRequest = {
    ...request, premise: { ...tuple, available: false },
  }
  pure val changedIntentRequest = {
    ...request, premise: { ...tuple, intent: { ...t, nonce: "different" } },
  }
  pure val changedPreRequest = {
    ...request, premise: { ...tuple, preState: { ...pre, balances: Map("O" -> 99, "R" -> 0, "F" -> 0) } },
  }
  pure val changedRoundRequest = {
    ...request, premise: { ...tuple, round: 1 },
  }
  pure val malformedPre = { ...pre, balances: Map("R" -> 0, "F" -> 0) }
  pure val malformedRequest = {
    ...unavailableRequest, premise: {
      ...unavailableRequest.premise, preState: malformedPre },
  }
  run P1Test = ready(t)
    .expect(rejected(t, pre, unavailableRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, unavailableRequest).fail())

  run P2IntentTest = ready(t)
    .expect(rejected(t, pre, changedIntentRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, changedIntentRequest).fail())

  run P2PreStateTest = ready(t)
    .expect(rejected(t, pre, changedPreRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, changedPreRequest).fail())

  run P2RoundTest = ready(t)
    .expect(rejected(t, pre, changedRoundRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, changedRoundRequest).fail())

  run P3Test = ready(t)
    .then(replacePre(malformedPre))
    .expect(rejected(t, malformedPre, malformedRequest, expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, malformedRequest).fail())

  pure val abstractNineRequest = {
    ...wrongHead, premise: { ...tuple, expectedSuccessor: 9 },
  }
  run nonArithmeticSuccessorTest = ready(t)
    .expect(transferObservation("digest-T-10-1", t, 0, 10, 1,
      effects(0, 9), abstractNineRequest) == {
        accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
      })
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      effects(0, 9), abstractNineRequest))
    .expect(financialSnapshot == {
      balances: Map("O" -> 89, "R" -> 10, "F" -> 1),
      allowanceRemaining: Map("O" -> 0), allowanceSpent: Map("O" -> 11),
      obligations: Map(), head: 9, consumed: Set(("D", "O", "T-10-1")),
      workRemaining: 0, workSpent: 1,
    } and lastEffects == effects(0, 9) and committedCount == 1 and
      signed == Map("digest-T-10-1" -> t) and round == 0)

  run failureAfterEffectTest = ready(t)
    .expect(rejected(t, pre, failureRequest, effects(0, 9), Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), failureRequest).fail())

  pure val r: SignedIntent = {
    ...t, program: RepayProgram, digest: "digest-R-30", nonce: "R-30",
    recipient: "", feeRecipient: "", obligationId: "L",
    debtor: "O", creditor: "C", grossCap: 30, feeCap: 0,
    actionAmount: 30, actionFee: 0,
  }
  pure val debt: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 1000,
    accrued: 10, outstanding: 1010, status: Outstanding,
  }
  pure val debtAfter: Obligation = { ...debt, principal: 980, accrued: 0, outstanding: 980 }
  pure val repayPre: FinancialSnapshot = {
    ...pre, balances: Map("O" -> 100, "C" -> 0),
    allowanceRemaining: Map("O" -> 100), obligations: Map("L" -> debt),
  }
  pure val repayExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }), Credit({ account: "C", amount: 30 }),
    SetObligation({ id: "L", value: debtAfter }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(("D", "O", "R-30")), AdvanceHead({ before: 0, after: 1 })
  )
  pure val recipientRepay = { ...r, recipient: "C" }
  pure val feeRecipientRepay = { ...r, feeRecipient: "F" }
  pure def repayRequest(i: SignedIntent): ComparisonRequest = {
    ...request, premise: { ...tuple, intent: i, preState: repayPre },
  }
  action repayReady(i: SignedIntent): bool =
    init.then(seedAccount("O", 100, 100, 0)).then(seedBalanceOnly("C", 0))
      .then(seedObligation("L", debt)).then(sign(i))
  def rejectedEndpoint(i: SignedIntent): bool = all {
    repayObservation("digest-R-30", i, 0, 30, repayExpected, repayRequest(i)) == {
      accepted: false, judgment: Intent, code: "S0_INTENT_SCOPE", diagnosticWork: 1,
    },
    financialSnapshot == repayPre,
    signed == Map("digest-R-30" -> i),
    round == 0 and lastEffects == List() and committedCount == 0,
  }
  run repayRecipientAbsentTest = repayReady(recipientRepay)
    .expect(rejectedEndpoint(recipientRepay))
    .then(submitRepay("digest-R-30", recipientRepay, 0, 30,
      repayExpected, repayRequest(recipientRepay)).fail())
  run repayFeeRecipientAbsentTest = repayReady(feeRecipientRepay)
    .expect(rejectedEndpoint(feeRecipientRepay))
    .then(submitRepay("digest-R-30", feeRecipientRepay, 0, 30,
      repayExpected, repayRequest(feeRecipientRepay)).fail())

  pure val changedOutcomeRequest = {
    ...request, premise: { ...tuple, requestedOutcome: failureOutcome },
  }
  run P2OutcomeTest = ready(t)
    .expect(rejected(t, pre, changedOutcomeRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      expected, changedOutcomeRequest).fail())

  pure val selfStipulatedRequest = {
    ...selfHead, premise: { ...tuple, expectedSuccessor: 0 },
  }
  run H3SelfPremiseTest = ready(t)
    .expect(rejected(t, pre, selfStipulatedRequest, effects(0, 0),
      History, "S0_HISTORY_SUCCESSOR"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      effects(0, 0), selfStipulatedRequest).fail())

  pure val staleReplayRequest = {
    ...request, premise: { ...tuple, intent: stale, preState: replayPre },
  }
  run staleAndReplayTest = ready(stale)
    .then(replacePre(replayPre))
    .expect(rejected(stale, replayPre, staleReplayRequest, effects(9, 1),
      History, "S0_HISTORY_STALE"))
    .then(submitTransfer("digest-T-10-1", stale, 9, 10, 1,
      effects(9, 1), staleReplayRequest).fail())

  pure val replayOnlyRequest = {
    ...request, premise: { ...tuple, preState: replayPre },
  }
  run effectAndReplayTest = ready(t)
    .then(replacePre(replayPre))
    .expect(rejected(t, replayPre, replayOnlyRequest, effects(0, 9),
      Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      effects(0, 9), replayOnlyRequest).fail())

  run effectAndWrongSuccessorTest = ready(t)
    .expect(rejected(t, pre, wrongHead, expected, Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      expected, wrongHead).fail())

  pure val post: FinancialSnapshot = {
    balances: Map("O" -> 89, "R" -> 10, "F" -> 1),
    allowanceRemaining: Map("O" -> 0), allowanceSpent: Map("O" -> 11),
    obligations: Map(), head: 1, consumed: Set(("D", "O", "T-10-1")),
    workRemaining: 0, workSpent: 1,
  }
  pure val postRequest = {
    ...request, premise: { ...tuple, preState: post },
  }
  // Only head 0 is stipulated as authenticated in this module. Local success
  // leaves head 1 unavailable, even though a commit exists and tuple matches.
  run committedHeadPremiseUnavailableTest = ready(t)
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, request))
    .expect(all {
      transferObservation("digest-T-10-1", t, 0, 10, 1,
        expected, postRequest) == {
          accepted: false, judgment: Stage, code: "S0_STAGE_PREMISE", diagnosticWork: 1,
        },
      financialSnapshot == post,
      signed == Map("digest-T-10-1" -> t),
      round == 0 and committedCount == 1 and lastEffects == expected,
      not(currentSnapshotAuthenticated),
    })
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      expected, postRequest).fail())

  // Postreview Stage controls: complete matching tuples isolate local shape.
  pure def stageRequest(i: SignedIntent, p: FinancialSnapshot): ComparisonRequest = {
    ...request, premise: { ...tuple, intent: i, preState: p },
  }
  def repayStageRejected(p: FinancialSnapshot, req: ComparisonRequest): bool = all {
    repayObservation("digest-R-30", r, 0, 30, repayExpected, req) == {
      accepted: false, judgment: Stage, code: "S0_STAGE_UNSUPPORTED", diagnosticWork: 1,
    },
    financialSnapshot == p,
    signed == Map("digest-R-30" -> r),
    round == 0 and committedCount == 0 and lastEffects == List(),
  }

  pure val transferExtraBalancePre: FinancialSnapshot = { ...pre, balances: pre.balances.put("X", 0) }
  run transferStageExtraBalanceTest = ready(t)
    .then(replacePre(transferExtraBalancePre))
    .expect(rejected(t, transferExtraBalancePre, stageRequest(t, transferExtraBalancePre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraBalancePre)).fail())

  pure val transferExtraRemainingPre: FinancialSnapshot = { ...pre, allowanceRemaining: pre.allowanceRemaining.put("X", 0) }
  run transferStageExtraRemainingTest = ready(t)
    .then(replacePre(transferExtraRemainingPre))
    .expect(rejected(t, transferExtraRemainingPre, stageRequest(t, transferExtraRemainingPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraRemainingPre)).fail())

  pure val transferExtraSpentPre: FinancialSnapshot = { ...pre, allowanceSpent: pre.allowanceSpent.put("X", 0) }
  run transferStageExtraSpentTest = ready(t)
    .then(replacePre(transferExtraSpentPre))
    .expect(rejected(t, transferExtraSpentPre, stageRequest(t, transferExtraSpentPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraSpentPre)).fail())

  pure val transferExtraObligationPre: FinancialSnapshot = { ...pre, obligations: pre.obligations.put("X", debt) }
  run transferStageExtraObligationTest = ready(t)
    .then(replacePre(transferExtraObligationPre))
    .expect(rejected(t, transferExtraObligationPre, stageRequest(t, transferExtraObligationPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraObligationPre)).fail())

  pure val transferNegativeWorkRemainingPre: FinancialSnapshot = { ...pre, workRemaining: -1 }
  run transferStageNegativeWorkRemainingTest = ready(t)
    .then(replacePre(transferNegativeWorkRemainingPre))
    .expect(rejected(t, transferNegativeWorkRemainingPre, stageRequest(t, transferNegativeWorkRemainingPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferNegativeWorkRemainingPre)).fail())

  pure val transferNegativeWorkSpentPre: FinancialSnapshot = { ...pre, workSpent: -1 }
  run transferStageNegativeWorkSpentTest = ready(t)
    .then(replacePre(transferNegativeWorkSpentPre))
    .expect(rejected(t, transferNegativeWorkSpentPre, stageRequest(t, transferNegativeWorkSpentPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferNegativeWorkSpentPre)).fail())

  pure val transferOverBoundWorkRemainingPre: FinancialSnapshot = { ...pre, workRemaining: UINT128_MAX + 1 }
  run transferStageOverBoundWorkRemainingTest = ready(t)
    .then(replacePre(transferOverBoundWorkRemainingPre))
    .expect(rejected(t, transferOverBoundWorkRemainingPre, stageRequest(t, transferOverBoundWorkRemainingPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferOverBoundWorkRemainingPre)).fail())

  pure val transferOverBoundWorkSpentPre: FinancialSnapshot = { ...pre, workSpent: UINT128_MAX + 1 }
  run transferStageOverBoundWorkSpentTest = ready(t)
    .then(replacePre(transferOverBoundWorkSpentPre))
    .expect(rejected(t, transferOverBoundWorkSpentPre, stageRequest(t, transferOverBoundWorkSpentPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferOverBoundWorkSpentPre)).fail())

  pure val transferWorkAggregateOverflowPre: FinancialSnapshot = { ...pre, workRemaining: 2, workSpent: UINT128_MAX - 1 }
  run transferStageWorkAggregateOverflowTest = ready(t)
    .then(replacePre(transferWorkAggregateOverflowPre))
    .expect(rejected(t, transferWorkAggregateOverflowPre, stageRequest(t, transferWorkAggregateOverflowPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferWorkAggregateOverflowPre)).fail())

  pure val transferAllowanceAggregateOverflowPre: FinancialSnapshot = { ...pre, allowanceRemaining: Map("O" -> 100), allowanceSpent: Map("O" -> UINT128_MAX - 1) }
  run transferStageAllowanceAggregateOverflowTest = ready(t)
    .then(replacePre(transferAllowanceAggregateOverflowPre))
    .expect(rejected(t, transferAllowanceAggregateOverflowPre, stageRequest(t, transferAllowanceAggregateOverflowPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferAllowanceAggregateOverflowPre)).fail())

  run transferStageWorkBeforePremiseTest = ready(t)
    .then(replacePre(transferNegativeWorkSpentPre))
    .expect(rejected(t, transferNegativeWorkSpentPre, { ...stageRequest(t, transferNegativeWorkSpentPre), premise: { ...tuple, available: false } }, expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, { ...stageRequest(t, transferNegativeWorkSpentPre), premise: { ...tuple, available: false } }).fail())

  pure val transferWorkBoundaryPre: FinancialSnapshot = { ...pre, workRemaining: 1, workSpent: UINT128_MAX - 1 }
  run transferStageWorkBoundaryTest = ready(t)
    .then(replacePre(transferWorkBoundaryPre))
    .expect(transferObservation("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferWorkBoundaryPre)) == ok)
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferWorkBoundaryPre)))
    .expect(workRemaining == 0 and workSpent == UINT128_MAX and committedCount == 1)

  pure val repayExtraBalancePre: FinancialSnapshot = { ...repayPre, balances: repayPre.balances.put("X", 0) }
  run repayStageExtraBalanceTest = repayReady(r)
    .then(replacePre(repayExtraBalancePre))
    .expect(repayStageRejected(repayExtraBalancePre, stageRequest(r, repayExtraBalancePre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraBalancePre)).fail())

  pure val repayExtraRemainingPre: FinancialSnapshot = { ...repayPre, allowanceRemaining: repayPre.allowanceRemaining.put("X", 0) }
  run repayStageExtraRemainingTest = repayReady(r)
    .then(replacePre(repayExtraRemainingPre))
    .expect(repayStageRejected(repayExtraRemainingPre, stageRequest(r, repayExtraRemainingPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraRemainingPre)).fail())

  pure val repayExtraSpentPre: FinancialSnapshot = { ...repayPre, allowanceSpent: repayPre.allowanceSpent.put("X", 0) }
  run repayStageExtraSpentTest = repayReady(r)
    .then(replacePre(repayExtraSpentPre))
    .expect(repayStageRejected(repayExtraSpentPre, stageRequest(r, repayExtraSpentPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraSpentPre)).fail())

  pure val repayExtraObligationPre: FinancialSnapshot = { ...repayPre, obligations: repayPre.obligations.put("X", debt) }
  run repayStageExtraObligationTest = repayReady(r)
    .then(replacePre(repayExtraObligationPre))
    .expect(repayStageRejected(repayExtraObligationPre, stageRequest(r, repayExtraObligationPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraObligationPre)).fail())

  pure val repayNegativeWorkRemainingPre: FinancialSnapshot = { ...repayPre, workRemaining: -1 }
  run repayStageNegativeWorkRemainingTest = repayReady(r)
    .then(replacePre(repayNegativeWorkRemainingPre))
    .expect(repayStageRejected(repayNegativeWorkRemainingPre, stageRequest(r, repayNegativeWorkRemainingPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayNegativeWorkRemainingPre)).fail())

  pure val repayNegativeWorkSpentPre: FinancialSnapshot = { ...repayPre, workSpent: -1 }
  run repayStageNegativeWorkSpentTest = repayReady(r)
    .then(replacePre(repayNegativeWorkSpentPre))
    .expect(repayStageRejected(repayNegativeWorkSpentPre, stageRequest(r, repayNegativeWorkSpentPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayNegativeWorkSpentPre)).fail())

  pure val repayOverBoundWorkRemainingPre: FinancialSnapshot = { ...repayPre, workRemaining: UINT128_MAX + 1 }
  run repayStageOverBoundWorkRemainingTest = repayReady(r)
    .then(replacePre(repayOverBoundWorkRemainingPre))
    .expect(repayStageRejected(repayOverBoundWorkRemainingPre, stageRequest(r, repayOverBoundWorkRemainingPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayOverBoundWorkRemainingPre)).fail())

  pure val repayOverBoundWorkSpentPre: FinancialSnapshot = { ...repayPre, workSpent: UINT128_MAX + 1 }
  run repayStageOverBoundWorkSpentTest = repayReady(r)
    .then(replacePre(repayOverBoundWorkSpentPre))
    .expect(repayStageRejected(repayOverBoundWorkSpentPre, stageRequest(r, repayOverBoundWorkSpentPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayOverBoundWorkSpentPre)).fail())

  pure val repayWorkAggregateOverflowPre: FinancialSnapshot = { ...repayPre, workRemaining: 2, workSpent: UINT128_MAX - 1 }
  run repayStageWorkAggregateOverflowTest = repayReady(r)
    .then(replacePre(repayWorkAggregateOverflowPre))
    .expect(repayStageRejected(repayWorkAggregateOverflowPre, stageRequest(r, repayWorkAggregateOverflowPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayWorkAggregateOverflowPre)).fail())

  pure val repayAllowanceAggregateOverflowPre: FinancialSnapshot = { ...repayPre, allowanceRemaining: Map("O" -> 100), allowanceSpent: Map("O" -> UINT128_MAX - 1) }
  run repayStageAllowanceAggregateOverflowTest = repayReady(r)
    .then(replacePre(repayAllowanceAggregateOverflowPre))
    .expect(repayStageRejected(repayAllowanceAggregateOverflowPre, stageRequest(r, repayAllowanceAggregateOverflowPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayAllowanceAggregateOverflowPre)).fail())

  run repayStageWorkBeforePremiseTest = repayReady(r)
    .then(replacePre(repayNegativeWorkSpentPre))
    .expect(repayStageRejected(repayNegativeWorkSpentPre, { ...stageRequest(r, repayNegativeWorkSpentPre), premise: { ...tuple, available: false } }))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, { ...stageRequest(r, repayNegativeWorkSpentPre), premise: { ...tuple, available: false } }).fail())

  pure val repayWorkBoundaryPre: FinancialSnapshot = { ...repayPre, workRemaining: 1, workSpent: UINT128_MAX - 1 }
  run repayStageWorkBoundaryTest = repayReady(r)
    .then(replacePre(repayWorkBoundaryPre))
    .expect(repayObservation("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayWorkBoundaryPre)) == ok)
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayWorkBoundaryPre)))
    .expect(workRemaining == 0 and workSpent == UINT128_MAX and committedCount == 1)
}
