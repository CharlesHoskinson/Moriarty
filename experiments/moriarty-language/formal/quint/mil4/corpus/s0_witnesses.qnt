// Finite MIL/4 S0 semantic witnesses. External premises are stipulated here.
module s0_witnesses {
  import s0(
    signatureVerified = Set("t", "r", "t2"),
    authenticatedSnapshots = Set(0, 1),
    nativeQualified = Set("t", "r", "t2"),
    ledgerAtomicReady = true,
    executingDomain = "D",
    settlementAsset = "A",
    initialWorkBudget = 3
  ).* from "../s0"

  // This helper stipulates comparison input only. It establishes no premise.
  def fixtureRequest(id: str, supplied: SignedIntent, successor: int): ComparisonRequest = {
    submittedSuccessor: successor, outcome: terminalOutcome,
    premise: { available: true,
      intent: if (signed.keys().contains(id)) signed.get(id) else supplied,
      preState: financialSnapshot, round: round, expectedSuccessor: successor, requestedOutcome: terminalOutcome },
  }

  pure val t: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A", digest: "t",
    policyDigest: "p", evidenceChoice: "e", signer: "O", payer: "O",
    recipient: "R", feeRecipient: "F", obligationId: "",
    debtor: "", creditor: "", nonce: "nt", preHead: 0,
    validFrom: 0, validThrough: 2, grossCap: 11, feeCap: 1,
    netFloor: 10, actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0, roundingNone: true,
    terminalOnly: true,
  }
  pure val r: SignedIntent = {
    ...t, program: RepayProgram, digest: "r", nonce: "nr",
    recipient: "", feeRecipient: "", obligationId: "loan",
    debtor: "O", creditor: "C", grossCap: 30, feeCap: 0,
    netFloor: 0, actionAmount: 30, actionFee: 0,
  }
  pure val o: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 1000,
    accrued: 10, outstanding: 1010, status: Outstanding,
  }
  pure val u = UINT128_MAX
  pure val s = NOMINAL_MAX
  pure val rNear: SignedIntent = {
    ...r, actionAmount: 1, grossCap: 1,
  }
  pure val oNear: Obligation = {
    ...o, principal: s - 1, accrued: 1, outstanding: s,
  }
  pure val t2: SignedIntent = {
    ...t, digest: "t2", preHead: 1,
  }
  pure val rOverpay: SignedIntent = {
    ...r, actionAmount: 1011, grossCap: 1011,
  }
  pure val tFeeCap: SignedIntent = {
    ...t, actionFee: 2, grossCap: 12,
  }
  pure val tGrossCap: SignedIntent = {
    ...t, grossCap: 10,
  }
  pure val missingFee: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(replayKey(t)),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val missingCreditor: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }),
    SetObligation({ id: "loan", value: repaid(o, 30) }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(replayKey(r)),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val wrongCreditor: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }),
    Credit({ account: "X", amount: 30 }),
    SetObligation({ id: "loan", value: repaid(o, 30) }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(replayKey(r)),
    AdvanceHead({ before: 0, after: 1 })
  )

  action transferReady: bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))

  action repayReady: bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("loan", o))
      .then(sign(r))

  run transferTenOneTest = transferReady
    .expect(transferObservation("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(all {
      balances.get("O") == 89,
      balances.get("R") == 10,
      balances.get("F") == 1,
      allowanceRemaining.get("O") == 89,
      allowanceSpent.get("O") == 11,
      workRemaining == 2 and workSpent == 1,
      consumed == Set(replayKey(t)),
      currentHead == 1 and committedCount == 1,
      lastEffects == transferLines(t, 10, 1, 1),
    })

  run repayThirtyTest = repayReady
    .expect(repayObservation("r", r, 0, 30, repayLines(r, o, 30, 1), fixtureRequest("r", r, 1)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
    .then(submitRepay("r", r, 0, 30, repayLines(r, o, 30, 1), fixtureRequest("r", r, 1)))
    .expect(all {
      balances.get("O") == 70 and balances.get("C") == 30,
      obligations.get("loan") == repaid(o, 30),
      obligations.get("loan").principal == 980,
      obligations.get("loan").accrued == 0,
      obligations.get("loan").outstanding == 980,
      obligations.get("loan").status == Outstanding,
      allowanceRemaining.get("O") == 70,
      allowanceSpent.get("O") == 30,
      consumed == Set(replayKey(r)),
      currentHead == 1 and committedCount == 1,
      lastEffects == repayLines(r, o, 30, 1),
    })

  run repayNearBoundTest =
    init.then(seedAccount("O", 1, 1, u - 1))
      .then(seedBalanceOnly("C", u - 1))
      .then(seedObligation("loan", oNear))
      .then(sign(rNear))
      .expect(repayObservation("r", rNear, 0, 1, repayLines(rNear, oNear, 1, 1), fixtureRequest("r", rNear, 1)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
      .then(submitRepay("r", rNear, 0, 1, repayLines(rNear, oNear, 1, 1), fixtureRequest("r", rNear, 1)))
      .expect(all {
        balances.get("C") == u,
        allowanceSpent.get("O") == u,
        obligations.get("loan").principal == s - 1,
        obligations.get("loan").accrued == 0,
        obligations.get("loan").outstanding == s - 1,
        committedCount == 1,
      })

  run recipientSubstitutionTest = transferReady
    .expect(all {
      not(transferObservation("t", {...t, recipient: "X"}, 0, 10, 1,
        transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).accepted),
      transferObservation("t", {...t, recipient: "X"}, 0, 10, 1,
        transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).judgment == Intent,
      transferObservation("t", {...t, recipient: "X"}, 0, 10, 1,
        transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).code == "S0_INTENT_SCOPE",
      transferObservation("t", {...t, recipient: "X"}, 0, 10, 1, transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).diagnosticWork == 1,
    })
    .expect(committedCount == 0 and lastEffects == List())
    .then(submitTransfer("t", {...t, recipient: "X"}, 0, 10, 1,
      transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).fail())

  run feeCapTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(tFeeCap))
    .expect(transferObservation("t", tFeeCap, 0, 10, 2,
      transferLines(tFeeCap, 10, 2, 1), fixtureRequest("t", tFeeCap, 1)) == { accepted: false, judgment: Intent, code: "S0_INTENT_SCOPE", diagnosticWork: 1 })
    .expect(committedCount == 0 and lastEffects == List())
    .then(submitTransfer("t", tFeeCap, 0, 10, 2,
      transferLines(tFeeCap, 10, 2, 1), fixtureRequest("t", tFeeCap, 1)).fail())

  run grossCapTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(tGrossCap))
      .expect(transferObservation("t", tGrossCap, 0, 10, 1,
        transferLines(tGrossCap, 10, 1, 1), fixtureRequest("t", tGrossCap, 1)) == { accepted: false, judgment: Intent, code: "S0_INTENT_SCOPE", diagnosticWork: 1 })
      .then(submitTransfer("t", tGrossCap, 0, 10, 1,
        transferLines(tGrossCap, 10, 1, 1), fixtureRequest("t", tGrossCap, 1)).fail())

  run missingFeeTest = transferReady
    .expect(transferObservation("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 0 and lastEffects == List())
    .then(submitTransfer("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)).fail())

  run missingCreditorTest = repayReady
    .expect(repayObservation("r", r, 0, 30, missingCreditor, fixtureRequest("r", r, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 0 and obligations.get("loan") == o and
      lastEffects == List())
    .then(submitRepay("r", r, 0, 30, missingCreditor, fixtureRequest("r", r, 1)).fail())

  run wrongCreditorTest = repayReady
    .expect(repayObservation("r", r, 0, 30, wrongCreditor, fixtureRequest("r", r, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 0 and obligations.get("loan") == o and
      lastEffects == List())
    .then(submitRepay("r", r, 0, 30, wrongCreditor, fixtureRequest("r", r, 1)).fail())

  run allowanceTest =
    init.then(seedAccount("O", 100, 10, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))
      .expect(transferObservation("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: false, judgment: Authority, code: "S0_AUTH_SCOPE", diagnosticWork: 1 })
      .expect(committedCount == 0 and lastEffects == List())
      .then(submitTransfer("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())

  run staleHeadTest = transferReady
    .then(sign({...t2, nonce: "fresh"}))
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(transferObservation("t2", {...t2, nonce: "fresh"}, 1, 10, 1,
      transferLines({...t2, nonce: "fresh"}, 10, 1, 2), fixtureRequest("t2", {...t2, nonce: "fresh"}, 2)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
    .expect(transferObservation("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: false, judgment: History, code: "S0_HISTORY_STALE", diagnosticWork: 1 })
    .expect(committedCount == 1 and currentHead == 1)
    .then(submitTransfer("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())

  run replayTest = transferReady
    .then(sign(t2))
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(transferObservation("t2", t2, 1, 10, 1,
      transferLines(t2, 10, 1, 2), fixtureRequest("t2", t2, 2)) == { accepted: false, judgment: History, code: "S0_HISTORY_REPLAY", diagnosticWork: 1 })
    .expect(committedCount == 1 and currentHead == 1)
    .then(submitTransfer("t2", t2, 1, 10, 1,
      transferLines(t2, 10, 1, 2), fixtureRequest("t2", t2, 2)).fail())

  run overflowTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", u))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))
      .expect(transferObservation("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_RANGE", diagnosticWork: 1 })
      .expect(committedCount == 0 and lastEffects == List())
      .then(submitTransfer("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())

  run overpayTest =
    init.then(seedAccount("O", 1011, 1011, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("loan", o))
      .then(sign(rOverpay))
      .expect(repayObservation("r", rOverpay, 0, 1011,
        repayLines(rOverpay, o, 1011, 1), fixtureRequest("r", rOverpay, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_RANGE", diagnosticWork: 1 })
      .expect(committedCount == 0 and obligations.get("loan") == o)
      .then(submitRepay("r", rOverpay, 0, 1011,
        repayLines(rOverpay, o, 1011, 1), fixtureRequest("r", rOverpay, 1)).fail())

  run missingFeeAndStaleTest = transferReady
    .then(sign({...t2, nonce: "fresh"}))
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(transferObservation("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 1 and currentHead == 1)
    .then(submitTransfer("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)).fail())
}
