module candidate_a_authority_swap_fixtures_f {
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import authorization.* from "../authorization"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_core_f.* from "./candidate_a_core_f"
  import candidate_a_projection_f.* from "./candidate_a_projection_f"
  import candidate_a_authority_adapter_f.* from "./candidate_a_authority_adapter_f"
  import candidate_a_authority_boundary_f.* from "./candidate_a_authority_boundary_f"

  // Independently literal Task1 expectations; EvidenceValid is an external premise.
  type ModeS = SettleS | RefundS | TimeoutS(ATime) | RefuseS({now: ATime, chosen: int})
  type ScenarioS = {funded: int, mode: ModeS}
  pure val SCENARIOS_S: Set[ScenarioS] = Set(
    {funded: 2, mode: SettleS}, {funded: 2, mode: RefundS}).union(
    Set(0, 1, 2).map(n => Set(Time100, Time101).map(t => {funded: n, mode: TimeoutS(t)})).flatten()).union(
    Set(Time100, Time101).map(t => Set(0, 1).map(c => {funded: 2, mode: RefuseS({now: t, chosen: c})})).flatten())
  pure val PROFILES_S = Set(SignAfterResolve, SignBeforeResolve)
  pure def keyS(principal: Principal, nonce: int): AuthorityKey = {domain: SwapDomain, principal: principal, nonce: nonce}
  pure val programS: AProgram = {root: N6, nodes: Map(
    N0 -> CloseA,
    N1 -> PayA({account: bobB, payee: Alice, amount: ConstantA(20), continuation: N0}),
    N2 -> PayA({account: aliceA, payee: Bob, amount: ConstantA(10), continuation: N1}),
    N3 -> IfA({observation: ChoiceEqualsA({id: SettleId, expected: 1}), thenNode: N2, elseNode: N0}),
    N4 -> WhenA({cases: List({caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: 0, upper: 1}), continuation: N3}),
      timeout: Time100, timeoutNode: N0}),
    N5 -> WhenA({cases: List({caseAction: DepositA({account: bobB, depositor: Bob, amount: ConstantA(20)}), continuation: N4}),
      timeout: Time100, timeoutNode: N0}),
    N6 -> WhenA({cases: List({caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(10)}), continuation: N5}),
      timeout: Time100, timeoutNode: N0}),
    N7 -> CloseA, N8 -> CloseA, N9 -> CloseA, N10 -> CloseA, N11 -> CloseA, N12 -> CloseA,
    N13 -> CloseA, N14 -> CloseA, N15 -> CloseA)}
  pure def accountsS(a: int, b: int): CoreAccount -> int = Map(
    {owner: Alice, asset: TokenA} -> a, {owner: Alice, asset: TokenB} -> 0,
    {owner: Bob, asset: TokenA} -> 0, {owner: Bob, asset: TokenB} -> b,
    {owner: Mallory, asset: TokenA} -> 0, {owner: Mallory, asset: TokenB} -> 0)
  pure def choicesS(choice: OptionalInt): AChoiceId -> OptionalInt = Map(SettleId -> choice,
    FirstFillId -> NoInt, SecondFillId -> NoInt, RecoveryId -> NoInt, OtherId -> NoInt)
  pure def neutralChoicesS(choice: OptionalInt): str -> OptionalInt = Map("settle" -> choice,
    "fill1" -> NoInt, "fill2" -> NoInt, "recover" -> NoInt, "other" -> NoInt)
  pure def beforeS(n: int): APredecessor = {program: programS, state: {
    continuation: if (n == 0) N6 else if (n == 1) N5 else N4,
    accounts: accountsS(if (n >= 1) 10 else 0, if (n == 2) 20 else 0),
    choices: choicesS(NoInt), minimumTime: if (n == 0) Time0 else if (n == 1) Time1 else Time2}}
  pure def ledgerS(n: int): Ledger = LEDGER_KEYS.mapBy(_ => 0)
    .put((Wallet(Alice), TokenA), if (n == 0) 10 else 0)
    .put((Wallet(Bob), TokenB), if (n < 2) 20 else 0)
    .put((Escrow(aliceA), TokenA), if (n >= 1) 10 else 0)
    .put((Escrow(bobB), TokenB), if (n == 2) 20 else 0)
  pure def environmentS(t: int): Environment = {physicalTime: t, anchor: 0, implementationVersion: 0, enforcementMechanism: 0}
  pure def factsS(n: int, t: int): PolicyFacts = {
    ledger: ledgerS(n), environment: environmentS(t), parents: AUTHORITY_KEYS.mapBy(_ => ParentAbsent)}
  pure def timeS(mode: ModeS): ATime = match mode {
    | SettleS => Time2 | RefundS => Time2 | TimeoutS(t) => t | RefuseS(r) => r.now }
  pure def rejectedS(mode: ModeS): bool = match mode { | RefuseS(_) => true | _ => false }
  pure def chosenS(mode: ModeS): OptionalInt = match mode {
    | SettleS => IntValue(1) | RefundS => IntValue(0) | RefuseS(r) => IntValue(r.chosen) | _ => NoInt }
  pure def dispositionInputS(mode: ModeS): AInput = match chosenS(mode) {
    | NoInt => NoAInput | IntValue(c) => PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: c})) }
  pure def neutralInputS(mode: ModeS): NeutralInput = match chosenS(mode) {
    | NoInt => NoInput | IntValue(c) => ChoiceLike({id: "settle", chooser: Bob, chosen: c}) }
  pure def operationS(mode: ModeS): Operation = match mode {
    | SettleS => OpSettle | RefundS => OpVoluntaryRefund | TimeoutS(_) => OpDeadlineRefund
    | RefuseS(r) => if (r.chosen == 1) OpSettle else OpVoluntaryRefund }
  pure def paymentS(account: CoreAccount, recipient: Principal, q: int): CorePayment = {
    source: account, recipient: recipient, asset: account.asset, quantity: q}
  pure def transferS(account: CoreAccount, recipient: Principal, q: int): Transfer = {
    source: Escrow(account), destination: Wallet(recipient), asset: account.asset, quantity: q}
  pure def projectedS(before: APredecessor, accepted: bool, node: NodeId, a: int, b: int,
    choice: OptionalInt, t: int, payments: List[CorePayment], reductions: int): CoreProjection[AContinuation] =
    CoreProjected({accepted: accepted, error: if (accepted) NoCoreError else CoreErrorCode("contract_closed"),
      warnings: List(), payments: payments, reductions: reductions, state: {
        accounts: accountsS(a, b), choices: neutralChoicesS(choice), continuation: {program: programS, node: node}, minimumTime: t}})
  pure def fundingPlannedS(principal: Principal): AAuthorityPlanned = {
    val n = if (principal == Alice) 0 else 1
    val account = if (principal == Alice) aliceA else bobB
    val q = if (principal == Alice) 10 else 20
    val now = if (principal == Alice) Time1 else Time2
    val effect = {source: Wallet(principal), destination: Escrow(account), asset: account.asset, quantity: q}
    {predecessor: beforeS(n), proposedSuccessor: beforeS(n + 1),
      artifactAndCall: AgreementCallA({before: beforeS(n),
        input: PresentAInput(DepositInputA({account: account, depositor: principal, quantity: q})), now: now}),
      input: DepositLike({location: Escrow(account), depositor: principal, asset: account.asset, quantity: q}),
      operation: OpFund, transactionTime: timeValue(now), effects: List(effect), predecessorFacts: factsS(n, timeValue(now)),
      coreProjection: projectedS(beforeS(n), true, if (principal == Alice) N5 else N4, 10,
        if (principal == Alice) 0 else 20, NoInt, timeValue(now), List(), 0)}
  }
  pure def dispositionPlannedS(s: ScenarioS): AAuthorityPlanned = {
    val before = beforeS(s.funded)
    val rejected = rejectedS(s.mode)
    val settlement = s.mode == SettleS
    val now = timeS(s.mode)
    val payments = if (rejected or s.funded == 0) List() else if (s.funded == 1) List(paymentS(aliceA, Alice, 10))
      else if (settlement) List(paymentS(aliceA, Bob, 10), paymentS(bobB, Alice, 20))
      else List(paymentS(aliceA, Alice, 10), paymentS(bobB, Bob, 20))
    val effects = if (rejected or s.funded == 0) List() else if (s.funded == 1) List(transferS(aliceA, Alice, 10))
      else if (settlement) List(transferS(aliceA, Bob, 10), transferS(bobB, Alice, 20))
      else List(transferS(aliceA, Alice, 10), transferS(bobB, Bob, 20))
    val after = if (rejected) before else {program: programS, state: {continuation: N0,
      accounts: accountsS(0, 0), choices: choicesS(chosenS(s.mode)), minimumTime: now}}
    {predecessor: before, proposedSuccessor: after,
      artifactAndCall: AgreementCallA({before: before, input: dispositionInputS(s.mode), now: now}),
      input: neutralInputS(s.mode), operation: operationS(s.mode), transactionTime: timeValue(now),
      effects: effects, predecessorFacts: factsS(s.funded, timeValue(now)),
      coreProjection: if (rejected) projectedS(before, false, N4, 10, 20, NoInt, 2, List(), 0)
        else projectedS(before, true, N0, 0, 0, chosenS(s.mode), timeValue(now), payments,
          if (settlement or s.mode == RefundS) 3 else 1 + s.funded)}
  }
  pure def plannedS(id: AttemptId, s: ScenarioS): AAuthorityPlanned =
    if (id == FundingOneAttempt) fundingPlannedS(Alice)
    else if (id == FundingTwoAttempt) fundingPlannedS(Bob) else dispositionPlannedS(s)
  pure def planS(id: AttemptId, s: ScenarioS): AResolvedPlan = {identity: SwapPlanA, operations: List(plannedS(id, s))}
  pure def expectedS(id: AttemptId, s: ScenarioS): AAuthorityObservation = {
    val p = plannedS(id, s)
    {predecessor: p.predecessor, proposedSuccessor: p.proposedSuccessor, artifactAndCall: p.artifactAndCall,
      resolvedPlan: planS(id, s), input: p.input, transactionTime: p.transactionTime, effects: p.effects,
      coreProjection: p.coreProjection, effectEvidence: EvidenceValid, display: PublicDisplay,
      outcome: if (id != DispositionAttempt) FundingAccepted
        else if (rejectedS(s.mode)) Rejected(CoreRejected(CoreErrorCode("contract_closed")))
        else if (s.mode == SettleS) Settlement else if (s.mode == RefundS) VoluntaryRefund else DeadlineRefund}
  }
  pure def actualS(id: AttemptId, s: ScenarioS): AAuthorityAdaptation = {
    val p = plannedS(id, s)
    match p.artifactAndCall {
      | AgreementCallA(r) => adaptAuthorityA(r, p.operation, planS(id, s), PublicDisplay)
      | _ => AuthorityInvalidRequestA }
  }
  pure def policyS(id: AttemptId, s: ScenarioS, principal: Principal, profile: SigningProfile): AAuthorityPolicy = {
    val p = plannedS(id, s)
    val location = if (id != DispositionAttempt) Wallet(principal) else Escrow(if (principal == Alice) aliceA else bobB)
    val body: PolicyBody = {key: keyS(principal, if (id == DispositionAttempt) 1 else 0),
      debitLocations: Set(location), capabilities: Set(if (id == DispositionAttempt) DisposeCapability else FundCapability),
      disclosures: Set(), validFrom: p.transactionTime, validUntil: p.transactionTime,
      implementationVersion: 0, enforcementMechanism: 0,
      clauses: List({operation: p.operation, conditions: List(InputIs(p.input)),
        requiredEffects: p.effects, allowedEffects: p.effects, effectOrder: ExactOrder})}
    val plan = planS(id, s)
    {body: body, profile: profile, binding: if (profile == SignAfterResolve)
      AfterResolution({identity: plan, operations: plan.operations}) else BeforeResolution(AnyArtifactUnderMechanism)}
  }
  pure val unsignedS: AAuthorityExecution = {authority: {context: {candidate: beforeS(0),
    ledger: ledgerS(0), environment: environmentS(1), registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused),
    parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)}, signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)},
    attempts: ATTEMPT_IDS.mapBy(_ => NoAttempt)}
  pure def expectedKeysS(id: AttemptId, s: ScenarioS): Set[AuthorityKey] =
    if (id == FundingOneAttempt) Set(keyS(Alice, 0)) else if (id == FundingTwoAttempt) Set(keyS(Bob, 0))
    else if (rejectedS(s.mode) or s.funded == 0) Set()
    else if (s.funded == 1) Set(keyS(Alice, 1)) else Set(keyS(Alice, 1), keyS(Bob, 1))

}
