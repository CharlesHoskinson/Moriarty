module observations {
  import effects.* from "./effects"

  // Neutral carriers, not a Core interpreter or correspondence result.
  type OptionalInt = NoInt | IntValue(int)
  type CoreChoiceId = str
  type CoreError = NoCoreError | CoreErrorCode(str)
  type CoreWarning = {code: str, requested: OptionalInt, paid: OptionalInt}
  type CoreAccount = {owner: Principal, asset: Asset}
  type CorePayment = {source: CoreAccount, recipient: Principal, asset: Asset, quantity: int}
  type CoreStateObservation[c] = {
    accounts: CoreAccount -> int, choices: CoreChoiceId -> OptionalInt,
    continuation: c, minimumTime: int,
  }
  type CoreResultObservation[c] = {
    accepted: bool, error: CoreError, warnings: List[CoreWarning],
    payments: List[CorePayment], state: CoreStateObservation[c], reductions: int,
  }
  type CoreProjection[c] = NoCoreProjection | CoreProjected(CoreResultObservation[c])
  type RejectionReason = CoreRejected(CoreError) | UnauthorizedEffect | StaleBindings
    | EvidenceMissing | ConsumptionConflict
  type SemanticOutcome = FundingAccepted | Settlement | VoluntaryRefund | DeadlineRefund
    | FirstInstallment | SecondInstallment | Cancellation | Recovery | Rejected(RejectionReason)
  type EvidenceDisposition = EvidenceValid | EvidenceUnavailable | EvidenceInvalid
  type NeutralInput = NoInput
    | DepositLike({location: Location, depositor: Principal, asset: Asset, quantity: int})
    | ChoiceLike({id: str, chooser: Principal, chosen: int})
  type DisplayProjection = PublicDisplay | AlternateDisplay
  type CandidateObservation[s,c,a,p] = {
    predecessor: s, proposedSuccessor: s, artifactAndCall: a, resolvedPlan: p,
    transactionTime: int, input: NeutralInput, effects: List[Transfer],
    outcome: SemanticOutcome, coreProjection: CoreProjection[c],
    effectEvidence: EvidenceDisposition, display: DisplayProjection,
  }
  pure val CORE_ACCOUNTS: Set[CoreAccount] =
    PRINCIPALS.map(p => ASSETS.map(a => {owner: p, asset: a})).flatten()

  pure def validFrozenCoreError(error: CoreError): bool = match error {
    | NoCoreError => true
    | CoreErrorCode(code) => Set("time_before_state", "contract_closed", "input_required",
        "no_matching_input", "choice_out_of_bounds", "non_positive_deposit").contains(code)
  }
  pure def validFrozenCoreWarning(w: CoreWarning): bool = match w.requested {
    | NoInt => false
    | IntValue(requested) => match w.paid {
        | NoInt => false
        | IntValue(paid) => (w.code == "non_positive_payment" and requested <= 0 and paid == 0)
            or (w.code == "partial_payment" and requested > 0 and paid >= 0 and paid < requested)
      }
  }
  pure def validCoreState(ids: Set[str], s: CoreStateObservation[c]): bool =
    s.accounts.keys() == CORE_ACCOUNTS and s.accounts.keys().forall(k => s.accounts.get(k) >= 0)
      and s.choices.keys() == ids
  pure def paymentTransfer(p: CorePayment): Transfer = {
    source: Escrow(p.source), destination: Wallet(p.recipient), asset: p.asset, quantity: p.quantity,
  }
  pure def validCorePayment(p: CorePayment): bool = p.asset == p.source.asset and p.quantity > 0
  pure def isEscrowTransfer(t: Transfer): bool = match t.source {
    | Wallet(_) => false
    | Escrow(_) => true
  }
  pure def isDepositTransfer(t: Transfer): bool = match t.source {
    | Escrow(_) => false
    | Wallet(_) => match t.destination {
        | Escrow(_) => true
        | Wallet(_) => false
      }
  }
  pure def rejectionPreserved(before: CoreStateObservation[c], r: CoreResultObservation[c],
                              transfers: List[Transfer]): bool =
    not(r.accepted) and r.state == before and r.warnings == List() and r.payments == List()
      and transfers == List() and r.reductions == 0
  // Structural/emission consistency only. Candidate execution and the independent
  // checker must establish the actual balances, continuation, and reduction count.
  pure def validCoreObservation(ids: Set[str], before: CoreStateObservation[c],
                               r: CoreResultObservation[c], transfers: List[Transfer]): bool =
    validCoreState(ids, before) and validCoreState(ids, r.state) and validFrozenCoreError(r.error)
      and (r.accepted == (r.error == NoCoreError)) and r.reductions >= 0
      and r.warnings.foldl(true, (ok, w) => ok and validFrozenCoreWarning(w))
      and r.payments.foldl(true, (ok, p) => ok and validCorePayment(p))
      and validEffects(transfers)
      and transfers.foldl(true, (ok, t) => ok and (isEscrowTransfer(t) or isDepositTransfer(t)))
      and transfers.select(t => isDepositTransfer(t)).length() <= 1
      and (if (r.accepted)
        r.state.minimumTime >= before.minimumTime
          and r.payments.foldl(List(), (acc, p) => acc.append(paymentTransfer(p)))
            == transfers.select(t => isEscrowTransfer(t))
        else rejectionPreserved(before, r, transfers))
  // Necessary carrier applicability, never sufficient evidence of correspondence.
  pure def projectionSupportsCoreEvidence(projection: CoreProjection[c]): bool = match projection {
    | NoCoreProjection => false
    | CoreProjected(_) => true
  }
}
