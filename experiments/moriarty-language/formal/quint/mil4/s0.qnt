// Provisional MIL/4 S0 transition model. See README.md for its evidence limits.
module s0 {
  type ReplayKey = (str, str, str)
  type Program = TransferProgram | RepayProgram
  type DebtStatus = Outstanding | Settled
  type Judgment = Accepted | Stage | Intent | Effect | Authority | History | Failure
  type Decision = { accepted: bool, judgment: Judgment, code: str,
                    diagnosticWork: int }

  type SignedIntent = {
    sourceVersion: str,
    coreVersion: str,
    profile: str,
    program: Program,
    domain: str,
    asset: str,
    digest: str,
    policyDigest: str,
    evidenceChoice: str,
    signer: str,
    payer: str,
    recipient: str,
    feeRecipient: str,
    obligationId: str,
    debtor: str,
    creditor: str,
    nonce: str,
    preHead: int,
    validFrom: int,
    validThrough: int,
    grossCap: int,
    feeCap: int,
    netFloor: int,
    actionAmount: int,
    actionFee: int,
    conversionMantissa: int,
    conversionScale: int,
    roundingNone: bool,
    terminalOnly: bool,
  }

  type Obligation = {
    debtor: str,
    creditor: str,
    asset: str,
    principal: int,
    accrued: int,
    outstanding: int,
    status: DebtStatus,
  }

  type EffectLine =
    | Debit({ account: str, amount: int })
    | Credit({ account: str, amount: int })
    | SetObligation({ id: str, value: Obligation })
    | UseAllowance({ owner: str, amount: int })
    | UseReplay(ReplayKey)
    | AdvanceHead({ before: int, after: int })

  type RequestedPhase = TerminalSuccess | RequestedFailure | RequestedPending
  type RetainedDuty = { id: str, owner: str, amount: int }
  type RequestedOutcome = { phase: RequestedPhase,
    retainedEffects: List[EffectLine], retainedDuties: List[RetainedDuty] }
  type FinancialSnapshot = {
    balances: str -> int, allowanceRemaining: str -> int,
    allowanceSpent: str -> int, workRemaining: int, workSpent: int,
    obligations: str -> Obligation, head: int, consumed: Set[ReplayKey],
  }
  type StipulatedTuple = { available: bool, intent: SignedIntent,
    preState: FinancialSnapshot, round: int, expectedSuccessor: int,
    requestedOutcome: RequestedOutcome }
  type ComparisonRequest = { submittedSuccessor: int,
    outcome: RequestedOutcome, premise: StipulatedTuple }
  pure val terminalOutcome: RequestedOutcome = {
    phase: TerminalSuccess, retainedEffects: List(), retainedDuties: List(),
  }

  // These are environmental premises, not checks implemented by Quint. The
  // selected external interfaces must establish their truth before admission.
  const signatureVerified: Set[str]
  const authenticatedSnapshots: Set[int]
  const nativeQualified: Set[str]
  const ledgerAtomicReady: bool
  const executingDomain: str
  const settlementAsset: str
  const initialWorkBudget: int

  pure val UINT128_MAX = 340282366920938463463374607431768211455
  pure val NOMINAL_MAX = 170141183460469231731687303715884105727

  var signed: str -> SignedIntent
  var balances: str -> int
  var allowanceRemaining: str -> int
  var allowanceSpent: str -> int
  var workRemaining: int
  var workSpent: int
  var obligations: str -> Obligation
  var currentHead: int
  var round: int
  var consumed: Set[ReplayKey]
  var lastEffects: List[EffectLine]
  var committedCount: int

  val financialSnapshot: FinancialSnapshot = {
    balances: balances, allowanceRemaining: allowanceRemaining,
    allowanceSpent: allowanceSpent, workRemaining: workRemaining,
    workSpent: workSpent, obligations: obligations, head: currentHead,
    consumed: consumed,
  }
  // Snapshot authentication remains an explicit external premise at every head.
  // A local commit never adds a head to this stipulated set.
  val currentSnapshotAuthenticated = authenticatedSnapshots.contains(currentHead)

  pure def replayKey(i: SignedIntent): ReplayKey = (i.domain, i.signer, i.nonce)
  pure def ok: Decision = { accepted: true, judgment: Accepted, code: "",
                            diagnosticWork: 0 }
  // Rejection reports one abstract diagnostic unit; it does not debit work.
  pure def reject(j: Judgment, c: str): Decision =
    { accepted: false, judgment: j, code: c, diagnosticWork: 1 }
  pure def withinUInt(n: int): bool = n >= 0 and n <= UINT128_MAX
  pure def withinNominal(n: int): bool = n >= 0 and n <= NOMINAL_MAX
  pure def minInt(a: int, b: int): int = if (a < b) a else b

  // Stage validates the complete work pair before premise or Authority checks.
  val stageWorkWellFormed = withinUInt(workRemaining) and withinUInt(workSpent)
    and workRemaining + workSpent <= UINT128_MAX

  pure def transferLines(i: SignedIntent, v: int, f: int, successor: int): List[EffectLine] = {
    val gross = v + f
    val money = if (f == 0)
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }))
    else
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }),
           Credit({ account: i.feeRecipient, amount: f }))
    money.concat(List(
      UseAllowance({ owner: i.payer, amount: gross }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: successor })
    ))
  }

  pure def repaid(o: Obligation, n: int): Obligation = {
    val da = minInt(n, o.accrued)
    val dp = n - da
    val p = o.principal - dp
    val a = o.accrued - da
    { ...o, principal: p, accrued: a, outstanding: p + a,
      status: if (p + a == 0) Settled else Outstanding }
  }

  pure def repayLines(i: SignedIntent, o: Obligation, n: int, successor: int): List[EffectLine] =
    List(
      Debit({ account: i.payer, amount: n }),
      Credit({ account: o.creditor, amount: n }),
      SetObligation({ id: i.obligationId, value: repaid(o, n) }),
      UseAllowance({ owner: i.payer, amount: n }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: successor })
    )

  // A rejected proposal is inspected through this value; it is never a stage.
  // Code spelling and within-judgment precedence are provisional W-D3 leaves.
  // Signed scope and caps precede alias diagnosis. Numeric effect guards in
  // each observation precede submitted-vector comparison.
  def beforeEffect(id: str, supplied: SignedIntent,
                     claimedHead: int, amount: int, fee: int,
                     recipient: str, debtor: str, isRepay: bool, request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)) or
        supplied.sourceVersion != "Source/6" or
        supplied.coreVersion != "Core/5" or
        supplied.profile != "S0" or
        supplied.domain != executingDomain or
        supplied.asset != settlementAsset)
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else if (not(signatureVerified.contains(signed.get(id).digest)) or
             not(currentSnapshotAuthenticated) or
             not(nativeQualified.contains(signed.get(id).digest)) or
             not(ledgerAtomicReady) or
             not(request.premise.available) or
             request.premise.intent != signed.get(id) or
             request.premise.preState != financialSnapshot or
             request.premise.round != round or
             request.premise.requestedOutcome != request.outcome)
      reject(Stage, "S0_STAGE_PREMISE")
    else if (signed.get(id) != supplied or
             amount != supplied.actionAmount or
             fee != supplied.actionFee or
             amount <= 0 or
             not(withinNominal(supplied.actionAmount)) or
             not(withinNominal(supplied.actionFee)) or
             not(withinNominal(supplied.grossCap)) or
             not(withinNominal(supplied.feeCap)) or
             not(withinNominal(supplied.netFloor)) or
             not(withinUInt(supplied.validFrom)) or
             not(withinUInt(supplied.validThrough)) or
             round < supplied.validFrom or round > supplied.validThrough or
             supplied.validFrom > supplied.validThrough or
             claimedHead != supplied.preHead or
             (not(isRepay) and
              (fee > supplied.feeCap or
               amount + fee > supplied.grossCap or
               amount < supplied.netFloor)) or
             (isRepay and (amount > supplied.grossCap or
                           supplied.feeCap != 0 or
                           supplied.netFloor != 0 or
                           supplied.recipient != "" or
                           supplied.feeRecipient != "" or
                           supplied.debtor != debtor or
                           supplied.creditor != recipient or
                           supplied.conversionMantissa != 1 or
                           supplied.conversionScale != 0 or
                           not(supplied.roundingNone))))
      reject(Intent, "S0_INTENT_SCOPE")
    else if (supplied.payer == recipient or
             (not(isRepay) and
              (supplied.payer == supplied.feeRecipient or
               supplied.recipient == supplied.feeRecipient)))
      reject(Intent, "S0_INTENT_ALIAS")
    else if (not(withinNominal(amount)) or
             not(withinNominal(amount + fee)) or fee < 0)
      reject(Effect, "S0_EFFECT_RANGE")
    else ok
  }

  def afterEffect(supplied: SignedIntent, claimedHead: int,
                  amount: int, fee: int, request: ComparisonRequest): Decision = {
    if (supplied.signer != supplied.payer or
             allowanceRemaining.get(supplied.payer) < amount + fee or
             allowanceSpent.get(supplied.payer) + amount + fee > UINT128_MAX or
             workRemaining < 1 or workSpent + 1 > UINT128_MAX)
      reject(Authority, "S0_AUTH_SCOPE")
    else if (claimedHead != currentHead)
      reject(History, "S0_HISTORY_STALE")
    else if (consumed.contains(replayKey(supplied)))
      reject(History, "S0_HISTORY_REPLAY")
    else if (request.submittedSuccessor == claimedHead or
             request.submittedSuccessor != request.premise.expectedSuccessor)
      reject(History, "S0_HISTORY_SUCCESSOR")
    else if (not(supplied.terminalOnly) or
             request.outcome.phase != TerminalSuccess or
             request.outcome.retainedEffects != List() or
             request.outcome.retainedDuties != List())
      reject(Failure, "S0_FAILURE_UNSUPPORTED")
    else ok
  }

  def transferObservation(id: str, supplied: SignedIntent,
                          claimedHead: int, v: int, f: int,
                          lines: List[EffectLine], request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != TransferProgram or
          not(stageWorkWellFormed) or
          obligations.keys() != Set() or
          allowanceRemaining.keys() != Set(bound.payer) or
          allowanceSpent.keys() != Set(bound.payer) or
          // Preserve Intent diagnosis for signed endpoint aliases. Distinct
          // endpoints select the exact three-balance S0 footprint.
          (bound.payer != bound.recipient and bound.payer != bound.feeRecipient and
           bound.recipient != bound.feeRecipient and
           balances.keys() != Set(bound.payer, bound.recipient, bound.feeRecipient)) or
          not(balances.keys().contains(bound.payer)) or
          not(balances.keys().contains(bound.recipient)) or
          not(balances.keys().contains(bound.feeRecipient)) or
          not(withinUInt(balances.get(bound.recipient))) or
          not(withinUInt(balances.get(bound.feeRecipient))) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)) or
          not(withinUInt(balances.get(bound.payer))) or
          not(withinUInt(allowanceRemaining.get(bound.payer))) or
          not(withinUInt(allowanceSpent.get(bound.payer))) or
          allowanceRemaining.get(bound.payer) + allowanceSpent.get(bound.payer) > UINT128_MAX)
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val base = beforeEffect(id, supplied, claimedHead, v, f,
                                supplied.recipient, "", false, request)
      if (not(base.accepted)) base
      else if (balances.get(supplied.payer) < v + f or
               balances.get(supplied.recipient) + v > UINT128_MAX or
               balances.get(supplied.feeRecipient) + f > UINT128_MAX or
               not(withinUInt(balances.get(supplied.payer))) or
               not(withinUInt(balances.get(supplied.recipient))) or
               not(withinUInt(balances.get(supplied.feeRecipient))))
        reject(Effect, "S0_EFFECT_RANGE")
      else if (lines != transferLines(supplied, v, f, request.submittedSuccessor))
        reject(Effect, "S0_EFFECT_MISMATCH")
      else afterEffect(supplied, claimedHead, v, f, request)
      }
    }
  }

  def repayObservation(id: str, supplied: SignedIntent,
                       claimedHead: int, n: int,
                       lines: List[EffectLine], request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != RepayProgram or
          not(stageWorkWellFormed) or
          obligations.keys() != Set(bound.obligationId) or
          allowanceRemaining.keys() != Set(bound.payer) or
          allowanceSpent.keys() != Set(bound.payer) or
          not(obligations.keys().contains(bound.obligationId)) or
          not(balances.keys().contains(bound.payer)) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)) or
          not(withinUInt(balances.get(bound.payer))) or
          not(withinUInt(allowanceRemaining.get(bound.payer))) or
          not(withinUInt(allowanceSpent.get(bound.payer))) or
          allowanceRemaining.get(bound.payer) + allowanceSpent.get(bound.payer) > UINT128_MAX)
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val o = obligations.get(bound.obligationId)
        // Use the immutable bound intent here: a supplied payer substitution
        // belongs to Intent after the existing typed state has passed Stage.
        if (balances.keys() != Set(bound.payer, o.creditor) or
            not(balances.keys().contains(o.creditor)) or
            not(withinUInt(balances.get(o.creditor))) or
            not(withinNominal(o.principal)) or
            not(withinNominal(o.accrued)) or
            not(withinNominal(o.outstanding)) or
            o.outstanding != o.principal + o.accrued or
            o.status != Outstanding or o.asset != settlementAsset or
            o.debtor != bound.signer or o.debtor != bound.payer or
            o.creditor == bound.payer)
          reject(Stage, "S0_STAGE_UNSUPPORTED")
        else {
        val base = beforeEffect(id, supplied, claimedHead, n, 0,
                                  o.creditor, o.debtor, true, request)
        if (not(base.accepted)) base
        else if (n > o.outstanding or
                 balances.get(supplied.payer) < n or
                 balances.get(o.creditor) + n > UINT128_MAX or
                 not(withinUInt(balances.get(supplied.payer))) or
                 not(withinUInt(balances.get(o.creditor))) or
                 not(withinUInt(repaid(o, n).outstanding)))
          reject(Effect, "S0_EFFECT_RANGE")
        else if (lines != repayLines(supplied, o, n, request.submittedSuccessor))
          reject(Effect, "S0_EFFECT_MISMATCH")
        else afterEffect(supplied, claimedHead, n, 0, request)
        }
      }
    }
  }

  action init: bool = all {
    withinUInt(initialWorkBudget),
    signed' = Map(),
    balances' = Map(),
    allowanceRemaining' = Map(),
    allowanceSpent' = Map(),
    workRemaining' = initialWorkBudget,
    workSpent' = 0,
    obligations' = Map(),
    currentHead' = 0,
    round' = 0,
    consumed' = Set(),
    lastEffects' = List(),
    committedCount' = 0,
  }

  // Environment setup represents authenticated cells; it is not a ledger proof.
  action seedAccount(account: str, balance: int,
                     remaining: int, spent: int): bool = all {
    committedCount == 0,
    not(balances.keys().contains(account)),
    withinUInt(balance) and withinUInt(remaining) and withinUInt(spent),
    remaining + spent <= UINT128_MAX,
    balances' = balances.put(account, balance),
    allowanceRemaining' = allowanceRemaining.put(account, remaining),
    allowanceSpent' = allowanceSpent.put(account, spent),
    workRemaining' = workRemaining, workSpent' = workSpent,
    signed' = signed, obligations' = obligations,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  // A receiving balance does not imply authority to spend from that account.
  action seedBalanceOnly(account: str, balance: int): bool = all {
    committedCount == 0,
    not(balances.keys().contains(account)),
    withinUInt(balance),
    balances' = balances.put(account, balance),
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    signed' = signed, obligations' = obligations,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action seedObligation(id: str, o: Obligation): bool = all {
    committedCount == 0,
    not(obligations.keys().contains(id)),
    o.asset == settlementAsset,
    withinUInt(o.principal) and withinUInt(o.accrued) and
      withinUInt(o.outstanding),
    o.outstanding == o.principal + o.accrued,
    obligations' = obligations.put(id, o),
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action advanceRound: bool = all {
    round' = round + 1,
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    obligations' = obligations, currentHead' = currentHead,
    consumed' = consumed, lastEffects' = lastEffects,
    committedCount' = committedCount,
  }

  action sign(i: SignedIntent): bool = all {
    not(signed.keys().contains(i.digest)),
    i.domain == executingDomain and i.asset == settlementAsset,
    i.sourceVersion == "Source/6" and i.coreVersion == "Core/5",
    i.profile == "S0",
    signed' = signed.put(i.digest, i),
    balances' = balances, allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent, obligations' = obligations,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action submitTransfer(id: str, supplied: SignedIntent, claimedHead: int,
                        v: int, f: int, lines: List[EffectLine], request: ComparisonRequest): bool = all {
    transferObservation(id, supplied, claimedHead, v, f, lines, request).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - v - f)
      .put(supplied.recipient, balances.get(supplied.recipient) + v)
      .put(supplied.feeRecipient, balances.get(supplied.feeRecipient) + f),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - v - f),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + v + f),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = request.submittedSuccessor,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, obligations' = obligations, round' = round,
  }

  action submitRepay(id: str, supplied: SignedIntent, claimedHead: int,
                     n: int, lines: List[EffectLine], request: ComparisonRequest): bool = all {
    repayObservation(id, supplied, claimedHead, n, lines, request).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - n)
      .put(obligations.get(supplied.obligationId).creditor,
           balances.get(obligations.get(supplied.obligationId).creditor) + n),
    obligations' = obligations.put(supplied.obligationId,
      repaid(obligations.get(supplied.obligationId), n)),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - n),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + n),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = request.submittedSuccessor,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, round' = round,
  }
}
