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

  pure def locationOwner(location: Location): Principal = match location {
    | Wallet(owner) => owner
    | Escrow(account) => account.owner
  }
  pure def validEnvironment(env: Environment): bool = Set(1, 2, 100, 101).contains(env.physicalTime)
    and Set(0, 1, 2).contains(env.anchor) and Set(0, 1).contains(env.implementationVersion)
    and Set(0, 1).contains(env.enforcementMechanism)
  pure def validPolicyFacts(facts: PolicyFacts): bool = validLedger(facts.ledger)
    and validEnvironment(facts.environment) and facts.parents.keys() == AUTHORITY_KEYS
  pure def validOperation(op: Operation): bool = match op {
    | OpFillSlot(fill) => AUTHORITY_KEYS.contains(fill.parent)
        and fill.parent.domain == InstallmentDomain and fill.parent.nonce == 0
        and Set(1, 2).contains(fill.slot)
    | OpCancelParent(parent) => AUTHORITY_KEYS.contains(parent)
        and parent.domain == InstallmentDomain and parent.nonce == 0
    | OpRecover(recovery) => AUTHORITY_KEYS.contains(recovery.parent)
        and recovery.parent.domain == InstallmentDomain and recovery.parent.nonce == 0
        and recovery.recovery == {...recovery.parent, nonce: 1}
    | _ => true
  }
  pure def requiredCapability(op: Operation): Capability = match op {
    | OpFund => FundCapability
    | OpFillSlot(fill) => if (fill.slot == 1) FirstFillCapability else SecondFillCapability
    | OpCancelParent(_) => CancelCapability
    | OpRecover(_) => RecoveryCapability
    | _ => DisposeCapability
  }
  pure def operationKeyAllowed(key: AuthorityKey, op: Operation): bool =
    AUTHORITY_KEYS.contains(key) and validOperation(op) and match op {
      | OpFund => key.domain == SwapDomain and key.nonce == 0
      | OpFillSlot(fill) => key == fill.parent
      | OpCancelParent(parent) => key == parent
      | OpRecover(recovery) => key == recovery.recovery
      | _ => key.domain == SwapDomain and key.nonce == 1
    }
  pure def conditionHolds(condition: Condition, input: NeutralInput, time: int, facts: PolicyFacts): bool =
    match condition {
      | InputIs(expected) => input == expected
      | BeforeTime(deadline) => time < deadline
      | AtOrAfterTime(deadline) => time >= deadline
      | ParentMatches(parent) => if (facts.parents.keys().contains(parent.key))
          facts.parents.get(parent.key) == parent.expected else false
      | SourceBalanceIs(source) => source.quantity >= 0
          and balance(facts.ledger, source.location, source.asset) == source.quantity
    }
  pure def validClause(clause: Clause): bool = validOperation(clause.operation)
    and validEffects(clause.requiredEffects) and validEffects(clause.allowedEffects)
    and submultiset(clause.requiredEffects, clause.allowedEffects)
  pure def validBody(body: PolicyBody): bool = AUTHORITY_KEYS.contains(body.key)
    and body.validFrom >= 0 and body.validFrom <= body.validUntil
    and Set(0, 1).contains(body.implementationVersion) and Set(0, 1).contains(body.enforcementMechanism)
    and body.clauses.length() > 0
    and body.debitLocations.forall(location => locationOwner(location) == body.key.principal)
    and body.clauses.foldl(true, (ok, clause) => ok and validClause(clause)
      and operationKeyAllowed(body.key, clause.operation)
      and body.capabilities.contains(requiredCapability(clause.operation)))
  pure def clauseAllows(clause: Clause, op: Operation, input: NeutralInput, time: int,
                       actual: List[Transfer], facts: PolicyFacts): bool = clause.operation == op
    and clause.conditions.foldl(true, (ok, condition) => ok and conditionHolds(condition, input, time, facts))
    and policyAllows(clause.requiredEffects, clause.allowedEffects, actual)
    and (clause.effectOrder == AnyOrder or actual == clause.allowedEffects)
  pure def bodyAllows(body: PolicyBody, op: Operation, input: NeutralInput, time: int,
                      actual: List[Transfer], facts: PolicyFacts): bool = validBody(body)
    and validPolicyFacts(facts) and operationKeyAllowed(body.key, op)
    and body.capabilities.contains(requiredCapability(op))
    and time == facts.environment.physicalTime and time >= body.validFrom and time <= body.validUntil
    and body.implementationVersion == facts.environment.implementationVersion
    and body.enforcementMechanism == facts.environment.enforcementMechanism
    and canApply(facts.ledger, actual)
    and actual.foldl(true, (ok, effect) => ok and (locationOwner(effect.source) != body.key.principal
      or body.debitLocations.contains(effect.source)))
    and body.clauses.foldl(false, (allowed, clause) => allowed or clauseAllows(clause, op, input, time, actual, facts))

  pure def validPolicy(policy: Policy[s,c,a,p]): bool = validBody(policy.body) and match policy.binding {
    | BeforeResolution(_) => policy.profile == SignBeforeResolve
    | AfterResolution(plan) => policy.profile == SignAfterResolve and plan.operations.length() > 0
        and plan.operations.foldl(true, (ok, planned) => ok and bodyAllows(policy.body, planned.operation,
          planned.input, planned.transactionTime, planned.effects, planned.predecessorFacts))
  }
  pure def plannedMatches(planned: PlannedOperation[s,c,a], op: Operation,
                          obs: CandidateObservation[s,c,a,p], facts: PolicyFacts): bool =
    planned == {predecessor: obs.predecessor, proposedSuccessor: obs.proposedSuccessor,
      artifactAndCall: obs.artifactAndCall, input: obs.input, operation: op,
      transactionTime: obs.transactionTime, effects: obs.effects,
      coreProjection: obs.coreProjection, predecessorFacts: facts}
  pure def bindingMatches(binding: PlanBinding[s,c,a,p], op: Operation,
                          obs: CandidateObservation[s,c,a,p], facts: PolicyFacts): bool = match binding {
    | BeforeResolution(constraint) => match constraint {
        | AnyArtifactUnderMechanism => true
        | PinnedArtifact(expected) => obs.artifactAndCall == expected
      }
    | AfterResolution(plan) => plan.identity == obs.resolvedPlan
        and plan.operations.foldl(false, (found, planned) => found or plannedMatches(planned, op, obs, facts))
  }
  pure def policyAllowsObservation(policy: Policy[s,c,a,p], op: Operation,
                                   obs: CandidateObservation[s,c,a,p], facts: PolicyFacts): bool =
    validPolicy(policy) and bodyAllows(policy.body, op, obs.input, obs.transactionTime, obs.effects, facts)
      and bindingMatches(policy.binding, op, obs, facts)

  pure def operationKeys(op: Operation): Set[AuthorityKey] = match op {
    | OpFillSlot(fill) => Set(fill.parent)
    | OpCancelParent(parent) => Set(parent)
    | OpRecover(recovery) => Set(recovery.recovery)
    | _ => Set()
  }
  pure def debitKey(op: Operation, principal: Principal): AuthorityKey = match op {
    | OpFund => {domain: SwapDomain, principal: principal, nonce: 0}
    | OpFillSlot(fill) => {...fill.parent, principal: principal}
    | OpCancelParent(parent) => {...parent, principal: principal}
    | OpRecover(recovery) => {...recovery.recovery, principal: principal}
    | _ => {domain: SwapDomain, principal: principal, nonce: 1}
  }
  pure def requiredKeys(op: Operation, actual: List[Transfer]): Set[AuthorityKey] =
    actual.foldl(operationKeys(op), (needed, effect) => needed.union(Set(debitKey(op, locationOwner(effect.source)))))
  // This checks policy contents only. A later boundary must also require exact
  // signed registrations and fresh verified evidence for every required key.
  pure def allRequiredPoliciesAllow(policies: AuthorityKey -> Policy[s,c,a,p], op: Operation,
                                    obs: CandidateObservation[s,c,a,p], facts: PolicyFacts): bool = {
    val needed = requiredKeys(op, obs.effects)
    validOperation(op) and needed.size() > 0 and needed.forall(key =>
      if (policies.keys().contains(key)) policies.get(key).body.key == key
        and policyAllowsObservation(policies.get(key), op, obs, facts) else false)
  }
}
