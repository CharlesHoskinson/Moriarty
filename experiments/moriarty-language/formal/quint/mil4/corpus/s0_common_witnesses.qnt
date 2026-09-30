// Three common-state positive witnesses with independent literal expectations.
// K h0/h1 map to 0/1 and repayment account P maps to O in this instance.
module s0_common_witnesses {
  import s0(
    signatureVerified = Set("digest-T-10-1", "digest-R-30", "digest-R-near-bound"),
    authenticatedSnapshots = Set(0),
    nativeQualified = Set("digest-T-10-1", "digest-R-30", "digest-R-near-bound"),
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
  pure val r: SignedIntent = {
    ...t, program: RepayProgram, digest: "digest-R-30", nonce: "R-30",
    recipient: "", feeRecipient: "", obligationId: "L",
    debtor: "O", creditor: "C", grossCap: 30, feeCap: 0,
    actionAmount: 30, actionFee: 0,
  }
  pure val near: SignedIntent = {
    ...r, digest: "digest-R-near-bound", nonce: "R-near-bound",
    grossCap: 1, actionAmount: 1,
  }
  pure val debt: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 1000,
    accrued: 10, outstanding: 1010, status: Outstanding,
  }
  pure val debtAfterThirty: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 980,
    accrued: 0, outstanding: 980, status: Outstanding,
  }
  pure val debtNear: Obligation = {
    debtor: "O", creditor: "C", asset: "A",
    principal: 170141183460469231731687303715884105726,
    accrued: 1, outstanding: 170141183460469231731687303715884105727,
    status: Outstanding,
  }
  pure val debtAfterOne: Obligation = {
    debtor: "O", creditor: "C", asset: "A",
    principal: 170141183460469231731687303715884105726,
    accrued: 0, outstanding: 170141183460469231731687303715884105726,
    status: Outstanding,
  }

  // Expected lines and debt values are literal; no model preparation helper
  // computes the oracle supplied to submit or checked in the successor.
  pure val transferExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    Credit({ account: "F", amount: 1 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(("D", "O", "T-10-1")),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val thirtyExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }),
    Credit({ account: "C", amount: 30 }),
    SetObligation({ id: "L", value: debtAfterThirty }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(("D", "O", "R-30")),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val oneExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 1 }),
    Credit({ account: "C", amount: 1 }),
    SetObligation({ id: "L", value: debtAfterOne }),
    UseAllowance({ owner: "O", amount: 1 }),
    UseReplay(("D", "O", "R-near-bound")),
    AdvanceHead({ before: 0, after: 1 })
  )

  run transferCommonTest =
    init.then(seedAccount("O", 100, 11, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))
      .expect(all {
        signed == Map("digest-T-10-1" -> t),
        balances == Map("O" -> 100, "R" -> 0, "F" -> 0),
        allowanceRemaining == Map("O" -> 11),
        allowanceSpent == Map("O" -> 0),
        obligations == Map(),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        transferObservation("digest-T-10-1", t, 0, 10, 1,
          transferExpected, fixtureRequest("digest-T-10-1", t, 1)) == {
            accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
          },
      })
      .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, transferExpected, fixtureRequest("digest-T-10-1", t, 1)))
      .expect(all {
        signed == Map("digest-T-10-1" -> t),
        balances == Map("O" -> 89, "R" -> 10, "F" -> 1),
        allowanceRemaining == Map("O" -> 0),
        allowanceSpent == Map("O" -> 11),
        not(allowanceRemaining.keys().contains("R")) and
          not(allowanceRemaining.keys().contains("F")),
        not(allowanceSpent.keys().contains("R")) and
          not(allowanceSpent.keys().contains("F")),
        obligations == Map(),
        consumed == Set(("D", "O", "T-10-1")),
        lastEffects == transferExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })

  run repayThirtyCommonTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("L", debt)).then(sign(r))
      .expect(all {
        signed == Map("digest-R-30" -> r),
        balances == Map("O" -> 100, "C" -> 0),
        allowanceRemaining == Map("O" -> 100),
        allowanceSpent == Map("O" -> 0),
        obligations == Map("L" -> debt),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        repayObservation("digest-R-30", r, 0, 30, thirtyExpected, fixtureRequest("digest-R-30", r, 1)) == {
          accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
        },
      })
      .then(submitRepay("digest-R-30", r, 0, 30, thirtyExpected, fixtureRequest("digest-R-30", r, 1)))
      .expect(all {
        signed == Map("digest-R-30" -> r),
        balances == Map("O" -> 70, "C" -> 30),
        allowanceRemaining == Map("O" -> 70),
        allowanceSpent == Map("O" -> 30),
        not(allowanceRemaining.keys().contains("C")) and
          not(allowanceSpent.keys().contains("C")),
        obligations == Map("L" -> debtAfterThirty),
        consumed == Set(("D", "O", "R-30")),
        lastEffects == thirtyExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })

  run repayNearBoundCommonTest =
    init.then(seedAccount("O", 1, 1, 340282366920938463463374607431768211454))
      .then(seedBalanceOnly("C", 340282366920938463463374607431768211454))
      .then(seedObligation("L", debtNear)).then(sign(near))
      .expect(all {
        signed == Map("digest-R-near-bound" -> near),
        balances == Map("O" -> 1,
          "C" -> 340282366920938463463374607431768211454),
        allowanceRemaining == Map("O" -> 1),
        allowanceSpent == Map("O" -> 340282366920938463463374607431768211454),
        obligations == Map("L" -> debtNear),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        repayObservation("digest-R-near-bound", near, 0, 1, oneExpected, fixtureRequest("digest-R-near-bound", near, 1)) == {
          accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
        },
      })
      .then(submitRepay("digest-R-near-bound", near, 0, 1, oneExpected, fixtureRequest("digest-R-near-bound", near, 1)))
      .expect(all {
        signed == Map("digest-R-near-bound" -> near),
        balances == Map("O" -> 0,
          "C" -> 340282366920938463463374607431768211455),
        allowanceRemaining == Map("O" -> 0),
        allowanceSpent == Map("O" -> 340282366920938463463374607431768211455),
        not(allowanceRemaining.keys().contains("C")) and
          not(allowanceSpent.keys().contains("C")),
        obligations == Map("L" -> debtAfterOne),
        consumed == Set(("D", "O", "R-near-bound")),
        lastEffects == oneExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })
}
