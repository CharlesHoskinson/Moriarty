module policies {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"

  // These are unsigned policy constraints. Context facts do not prove authority.
  type Environment = {physicalTime: int, anchor: int, implementationVersion: int, enforcementMechanism: int}
  type ParentFacts = ParentAbsent | ParentPresent({cancelled: bool, usedSlots: Set[int],
    paid: int, remainingAllowance: int, revision: int})
  type PolicyFacts = {ledger: Ledger, environment: Environment, parents: AuthorityKey -> ParentFacts}
  type Operation = OpFund | OpSettle | OpVoluntaryRefund | OpDeadlineRefund
    | OpFillSlot({parent: AuthorityKey, slot: int}) | OpCancelParent(AuthorityKey)
    | OpRecover({parent: AuthorityKey, recovery: AuthorityKey})
  type Capability = FundCapability | DisposeCapability | FirstFillCapability
    | SecondFillCapability | CancelCapability | RecoveryCapability
  type Condition = InputIs(NeutralInput) | BeforeTime(int) | AtOrAfterTime(int)
    | ParentMatches({key: AuthorityKey, expected: ParentFacts})
    | SourceBalanceIs({location: Location, asset: Asset, quantity: int})
  type EffectOrder = AnyOrder | ExactOrder
  type Clause = {operation: Operation, conditions: List[Condition],
    requiredEffects: List[Transfer], allowedEffects: List[Transfer], effectOrder: EffectOrder}
  type PolicyBody = {key: AuthorityKey, clauses: List[Clause], debitLocations: Set[Location],
    capabilities: Set[Capability], disclosures: Set[str], validFrom: int, validUntil: int,
    implementationVersion: int, enforcementMechanism: int}
  type SigningProfile = SignAfterResolve | SignBeforeResolve
  type PlannedOperation[s,c,a] = {predecessor: s, proposedSuccessor: s, artifactAndCall: a,
    input: NeutralInput, operation: Operation, transactionTime: int, effects: List[Transfer],
    coreProjection: CoreProjection[c], predecessorFacts: PolicyFacts}
  type ResolvedPlanView[s,c,a,p] = {identity: p, operations: List[PlannedOperation[s,c,a]]}
  type ArtifactConstraint[a] = AnyArtifactUnderMechanism | PinnedArtifact(a)
  type PlanBinding[s,c,a,p] = AfterResolution(ResolvedPlanView[s,c,a,p])
    | BeforeResolution(ArtifactConstraint[a])
  type Policy[s,c,a,p] = {body: PolicyBody, profile: SigningProfile, binding: PlanBinding[s,c,a,p]}

  pure val AUTHORITY_KEYS: Set[AuthorityKey] = Set(SwapDomain, InstallmentDomain).map(d =>
    PRINCIPALS.map(p => Set(0, 1).map(n => {domain: d, principal: p, nonce: n})).flatten()).flatten()

  // RED scaffolding: no operation is authorized before the constraints exist.
  pure def bodyAllows(body: PolicyBody, op: Operation, input: NeutralInput, time: int,
                      actual: List[Transfer], facts: PolicyFacts): bool = false
  pure def policyAllowsObservation(policy: Policy[s,c,a,p], op: Operation,
                                   obs: CandidateObservation[s,c,a,p], facts: PolicyFacts): bool = false
  pure def allRequiredPoliciesAllow(policies: AuthorityKey -> Policy[s,c,a,p], op: Operation,
                                    obs: CandidateObservation[s,c,a,p], facts: PolicyFacts): bool = false
}
