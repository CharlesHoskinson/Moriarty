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

  // Fail-closed scaffolding: replaced only after the positive tests fail.
  pure def validFrozenCoreError(error: CoreError): bool = false
  pure def validFrozenCoreWarning(w: CoreWarning): bool = false
  pure def validCoreState(ids: Set[str], s: CoreStateObservation[c]): bool = false
  pure def paymentTransfer(p: CorePayment): Transfer = {
    source: Escrow(p.source), destination: Wallet(p.recipient), asset: p.asset, quantity: 0,
  }
  pure def validCorePayment(p: CorePayment): bool = false
  pure def rejectionPreserved(before: CoreStateObservation[c], r: CoreResultObservation[c],
                              transfers: List[Transfer]): bool = false
  pure def validCoreObservation(ids: Set[str], before: CoreStateObservation[c],
                               r: CoreResultObservation[c], transfers: List[Transfer]): bool = false
  pure def projectionSupportsCoreEvidence(projection: CoreProjection[c]): bool = false
}
