// Fixed ordering regressions for the repaired provisional S0 Quint candidate.
module s0_divergence_witnesses {
  import s0(
    signatureVerified = Set("overflow", "alias"),
    authenticatedSnapshots = Set(0),
    nativeQualified = Set("overflow", "alias"),
    ledgerAtomicReady = true,
    executingDomain = "D",
    settlementAsset = "A",
    initialWorkBudget = 1
  ).* from "../s0"

  // This helper stipulates comparison input only. It establishes no premise.
  def fixtureRequest(id: str, supplied: SignedIntent, successor: int): ComparisonRequest = {
    submittedSuccessor: successor, outcome: terminalOutcome,
    premise: { available: true,
      intent: if (signed.keys().contains(id)) signed.get(id) else supplied,
      preState: financialSnapshot, round: round, expectedSuccessor: successor, requestedOutcome: terminalOutcome },
  }

  pure val transfer: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A",
    digest: "overflow", policyDigest: "p", evidenceChoice: "e",
    signer: "O", payer: "O", recipient: "R", feeRecipient: "F",
    obligationId: "", debtor: "", creditor: "", nonce: "n1",
    preHead: 0, validFrom: 0, validThrough: 1,
    grossCap: 11, feeCap: 1, netFloor: 10,
    actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0,
    roundingNone: true, terminalOnly: true,
  }
  pure val missingFee: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(replayKey(transfer)),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val repayAlias: SignedIntent = {
    ...transfer, program: RepayProgram, digest: "alias",
    obligationId: "loan", debtor: "O", creditor: "O",
    recipient: "O", actionAmount: 30, actionFee: 0,
    grossCap: 30, feeCap: 0, netFloor: 0,
  }
  pure val selfDebt: Obligation = {
    debtor: "O", creditor: "O", asset: "A",
    principal: 1000, accrued: 10, outstanding: 1010,
    status: Outstanding,
  }

  run overflowAndMissingFeeTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", UINT128_MAX))
      .then(seedBalanceOnly("F", 0))
      .then(sign(transfer))
      .expect(all {
        transferObservation("overflow", transfer, 0, 10, 1, missingFee, fixtureRequest("overflow", transfer, 1)).judgment == Effect,
        transferObservation("overflow", transfer, 0, 10, 1, missingFee, fixtureRequest("overflow", transfer, 1)).code == "S0_EFFECT_RANGE",
        committedCount == 0,
        lastEffects == List(),
      })
      .then(submitTransfer("overflow", transfer, 0, 10, 1, missingFee, fixtureRequest("overflow", transfer, 1)).fail())

  run repaymentAliasTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedObligation("loan", selfDebt))
      .then(sign(repayAlias))
      .expect(all {
        repayObservation("alias", repayAlias, 0, 30,
          repayLines(repayAlias, selfDebt, 30, 1), fixtureRequest("alias", repayAlias, 1)).judgment == Stage,
        repayObservation("alias", repayAlias, 0, 30,
          repayLines(repayAlias, selfDebt, 30, 1), fixtureRequest("alias", repayAlias, 1)).code == "S0_STAGE_UNSUPPORTED",
        committedCount == 0,
        obligations.get("loan") == selfDebt,
      })
      .then(submitRepay("alias", repayAlias, 0, 30,
        repayLines(repayAlias, selfDebt, 30, 1), fixtureRequest("alias", repayAlias, 1)).fail())
  pure val debt: Obligation = { ...selfDebt, creditor: "C" }
  pure val repay: SignedIntent = {
    ...repayAlias, creditor: "C", recipient: "", feeRecipient: "",
  }
  pure val badDebtor: Obligation = { ...debt, debtor: "X" }
  pure val signerMismatch: SignedIntent = { ...repay, signer: "X" }
  pure val payerMismatch: SignedIntent = { ...repay, payer: "X" }
  pure val signedDebtorMismatch: SignedIntent = { ...repay, debtor: "X" }
  pure val signedCreditorMismatch: SignedIntent = { ...repay, creditor: "X" }
  pure val nonidentity: SignedIntent = { ...repay, conversionMantissa: 2 }
  pure val scaled: SignedIntent = { ...repay, conversionScale: 1 }
  pure val rounded: SignedIntent = { ...repay, roundingNone: false }
  pure val roundAlias: SignedIntent = {
    ...transfer, recipient: "O", validFrom: 1,
  }
  pure val capAlias: SignedIntent = {
    ...transfer, recipient: "O", grossCap: 10,
  }
  pure val overpay: SignedIntent = {
    ...repay, actionAmount: 1011, grossCap: 1011,
  }
  pure val missingCreditor: List[EffectLine] = List(
    Debit({ account: "O", amount: 1011 }),
    SetObligation({ id: "loan", value: repaid(debt, 1011) }),
    UseAllowance({ owner: "O", amount: 1011 }),
    UseReplay(replayKey(overpay)),
    AdvanceHead({ before: 0, after: 1 })
  )

  action repayReady(i: SignedIntent, o: Obligation): bool =
    init.then(seedAccount("O", 2000, 2000, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("loan", o))
      .then(sign(i))

  def rejectedRepay(i: SignedIntent, o: Obligation, j: Judgment,
                    c: str, lines: List[EffectLine]): bool = all {
    not(repayObservation("alias", i, 0, i.actionAmount, lines, fixtureRequest("alias", i, 1)).accepted),
    repayObservation("alias", i, 0, i.actionAmount, lines, fixtureRequest("alias", i, 1)).judgment == j,
    repayObservation("alias", i, 0, i.actionAmount, lines, fixtureRequest("alias", i, 1)).code == c,
    committedCount == 0 and currentHead == 0,
    balances.get("O") == 2000 and balances.get("C") == 0,
    allowanceRemaining.get("O") == 2000 and allowanceSpent.get("O") == 0,
    workRemaining == 1 and workSpent == 0,
    consumed == Set() and lastEffects == List(),
    obligations.get("loan") == o,
  }

  run debtorMismatchTest = repayReady(repay, badDebtor)
    .expect(rejectedRepay(repay, badDebtor, Stage, "S0_STAGE_UNSUPPORTED",
      repayLines(repay, badDebtor, 30, 1)))
    .then(submitRepay("alias", repay, 0, 30,
      repayLines(repay, badDebtor, 30, 1), fixtureRequest("alias", repay, 1)).fail())

  run debtorSignerMismatchTest = repayReady(signerMismatch, debt)
    .expect(rejectedRepay(signerMismatch, debt, Stage, "S0_STAGE_UNSUPPORTED",
      repayLines(signerMismatch, debt, 30, 1)))
    .then(submitRepay("alias", signerMismatch, 0, 30,
      repayLines(signerMismatch, debt, 30, 1), fixtureRequest("alias", signerMismatch, 1)).fail())

  run debtorPayerMismatchTest = repayReady(payerMismatch, debt)
    .expect(rejectedRepay(payerMismatch, debt, Stage, "S0_STAGE_UNSUPPORTED",
      repayLines(payerMismatch, debt, 30, 1)))
    .then(submitRepay("alias", payerMismatch, 0, 30,
      repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).fail())

  run signedDebtorScopeTest = repayReady(signedDebtorMismatch, debt)
    .expect(rejectedRepay(signedDebtorMismatch, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(signedDebtorMismatch, debt, 30, 1)))
    .then(submitRepay("alias", signedDebtorMismatch, 0, 30,
      repayLines(signedDebtorMismatch, debt, 30, 1), fixtureRequest("alias", signedDebtorMismatch, 1)).fail())

  run signedCreditorScopeTest = repayReady(signedCreditorMismatch, debt)
    .expect(rejectedRepay(signedCreditorMismatch, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(signedCreditorMismatch, debt, 30, 1)))
    .then(submitRepay("alias", signedCreditorMismatch, 0, 30,
      repayLines(signedCreditorMismatch, debt, 30, 1), fixtureRequest("alias", signedCreditorMismatch, 1)).fail())

  // A substituted supplied payer must not change the bound Stage classification.
  run suppliedPayerScopeTest = repayReady(repay, debt)
    .expect(all {
      not(repayObservation("alias", payerMismatch, 0, 30,
        repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).accepted),
      repayObservation("alias", payerMismatch, 0, 30,
        repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).judgment == Intent,
      repayObservation("alias", payerMismatch, 0, 30,
        repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).code == "S0_INTENT_SCOPE",
      obligations.get("loan") == debt and committedCount == 0,
      consumed == Set() and lastEffects == List(),
    })
    .then(submitRepay("alias", payerMismatch, 0, 30,
      repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).fail())

  run nonidentityConversionTest = repayReady(nonidentity, debt)
    .expect(rejectedRepay(nonidentity, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(nonidentity, debt, 30, 1)))
    .then(submitRepay("alias", nonidentity, 0, 30,
      repayLines(nonidentity, debt, 30, 1), fixtureRequest("alias", nonidentity, 1)).fail())

  run conversionScaleTest = repayReady(scaled, debt)
    .expect(rejectedRepay(scaled, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(scaled, debt, 30, 1)))
    .then(submitRepay("alias", scaled, 0, 30,
      repayLines(scaled, debt, 30, 1), fixtureRequest("alias", scaled, 1)).fail())

  run conversionRoundingTest = repayReady(rounded, debt)
    .expect(rejectedRepay(rounded, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(rounded, debt, 30, 1)))
    .then(submitRepay("alias", rounded, 0, 30,
      repayLines(rounded, debt, 30, 1), fixtureRequest("alias", rounded, 1)).fail())

  action aliasReady(i: SignedIntent): bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("F", 0)).then(sign(i))

  def rejectedAlias(i: SignedIntent): bool = all {
    not(transferObservation("overflow", i, 0, 10, 1,
      transferLines(i, 10, 1, 1), fixtureRequest("overflow", i, 1)).accepted),
    transferObservation("overflow", i, 0, 10, 1,
      transferLines(i, 10, 1, 1), fixtureRequest("overflow", i, 1)).judgment == Intent,
    transferObservation("overflow", i, 0, 10, 1,
      transferLines(i, 10, 1, 1), fixtureRequest("overflow", i, 1)).code == "S0_INTENT_SCOPE",
    committedCount == 0 and currentHead == 0,
    balances.get("O") == 100 and balances.get("F") == 0,
    allowanceRemaining.get("O") == 100 and allowanceSpent.get("O") == 0,
    workRemaining == 1 and workSpent == 0,
    consumed == Set() and lastEffects == List(),
  }

  run invalidRoundAndAliasTest = aliasReady(roundAlias)
    .expect(rejectedAlias(roundAlias))
    .then(submitTransfer("overflow", roundAlias, 0, 10, 1,
      transferLines(roundAlias, 10, 1, 1), fixtureRequest("overflow", roundAlias, 1)).fail())

  run capAndAliasTest = aliasReady(capAlias)
    .expect(rejectedAlias(capAlias))
    .then(submitTransfer("overflow", capAlias, 0, 10, 1,
      transferLines(capAlias, 10, 1, 1), fixtureRequest("overflow", capAlias, 1)).fail())

  run overpayAndMissingCreditorTest = repayReady(overpay, debt)
    .expect(rejectedRepay(overpay, debt, Effect, "S0_EFFECT_RANGE",
      missingCreditor))
    .then(submitRepay("alias", overpay, 0, 1011, missingCreditor, fixtureRequest("alias", overpay, 1)).fail())
}

