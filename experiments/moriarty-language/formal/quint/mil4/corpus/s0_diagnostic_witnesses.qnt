// Fixed full-decision witnesses. External admission premises are stipulated.
module s0_diagnostic_witnesses {
  import s0(
    signatureVerified = Set("t"),
    authenticatedSnapshots = Set(0, 1),
    nativeQualified = Set("t"),
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

  pure val missingFee: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(replayKey(t)),
    AdvanceHead({ before: 0, after: 1 })
  )

  action transferReady: bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))

  run stageDiagnosticTest = init
    .expect(all {
      transferObservation("missing", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("missing", t, 1)) == {
          accepted: false, judgment: Stage, code: "S0_STAGE_UNSUPPORTED",
          diagnosticWork: 1,
        },
      committedCount == 0 and currentHead == 0,
      workRemaining == 3 and workSpent == 0,
      consumed == Set() and lastEffects == List(),
    })
    .then(submitTransfer("missing", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("missing", t, 1)).fail())

  run effectDiagnosticTest = transferReady
    .expect(all {
      transferObservation("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)) == {
        accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH",
        diagnosticWork: 1,
      },
      balances.get("O") == 100 and balances.get("R") == 0 and
        balances.get("F") == 0,
      allowanceRemaining.get("O") == 100 and allowanceSpent.get("O") == 0,
      committedCount == 0 and currentHead == 0,
      workRemaining == 3 and workSpent == 0,
      consumed == Set() and lastEffects == List(),
    })
    .then(submitTransfer("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)).fail())

  run historyDiagnosticTest = transferReady
    .expect(transferObservation("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == {
        accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
      })
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(all {
      transferObservation("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == {
          accepted: false, judgment: History, code: "S0_HISTORY_STALE",
          diagnosticWork: 1,
        },
      balances.get("O") == 89 and balances.get("R") == 10 and
        balances.get("F") == 1,
      allowanceRemaining.get("O") == 89 and allowanceSpent.get("O") == 11,
      committedCount == 1 and currentHead == 1,
      workRemaining == 2 and workSpent == 1,
      consumed == Set(replayKey(t)) and lastEffects == transferLines(t, 10, 1, 1),
    })
    .then(submitTransfer("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())
}
