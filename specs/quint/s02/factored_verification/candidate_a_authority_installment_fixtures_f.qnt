module candidate_a_authority_installment_fixtures_f {
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

type RecoveryModeI = Choice2I | Timeout100I | Timeout101I | Refuse100I | Refuse101I
type ScenarioI = TwoFillsI | RecoverI({residual: bool, mode: RecoveryModeI})
pure val MODES_I = Set(Choice2I, Timeout100I, Timeout101I, Refuse100I, Refuse101I)
pure val SCENARIOS_I = Set(TwoFillsI).union(Set(false, true).map(residual =>
  MODES_I.map(mode => RecoverI({residual: residual, mode: mode}))).flatten())
pure val PROFILES_I = Set(SignAfterResolve, SignBeforeResolve)
pure val parentKeyI: AuthorityKey = {domain: InstallmentDomain, principal: Alice, nonce: 0}
pure val recoveryKeyI: AuthorityKey = {domain: InstallmentDomain, principal: Alice, nonce: 1}
pure val fill1I: Operation = OpFillSlot({parent: parentKeyI, slot: 1})
pure val fill2I: Operation = OpFillSlot({parent: parentKeyI, slot: 2})
pure val cancelI: Operation = OpCancelParent(parentKeyI)
pure val recoverI: Operation = OpRecover({parent: parentKeyI, recovery: recoveryKeyI})
pure val programI: AProgram = {root: N4, nodes: Map(
  N0 -> CloseA,
  N1 -> PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}),
  N2 -> WhenA({cases: List(
    {caseAction: ChoiceA({id: SecondFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N1},
    {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
    timeout: Time100, timeoutNode: N0}),
  N3 -> PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N2}),
  N4 -> WhenA({cases: List(
    {caseAction: ChoiceA({id: FirstFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N3},
    {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
    timeout: Time100, timeoutNode: N0}),
  N5 -> CloseA, N6 -> CloseA, N7 -> CloseA, N8 -> CloseA, N9 -> CloseA,
  N10 -> CloseA, N11 -> CloseA, N12 -> CloseA, N13 -> CloseA, N14 -> CloseA, N15 -> CloseA)}
pure def accountsI(q: int): CoreAccount -> int = Map(
  {owner: Alice, asset: TokenA} -> q, {owner: Alice, asset: TokenB} -> 0,
  {owner: Bob, asset: TokenA} -> 0, {owner: Bob, asset: TokenB} -> 0,
  {owner: Mallory, asset: TokenA} -> 0, {owner: Mallory, asset: TokenB} -> 0)
pure def choicesI(first: OptionalInt, second: OptionalInt, recovery: OptionalInt): AChoiceId -> OptionalInt =
  Map(SettleId -> NoInt, FirstFillId -> first, SecondFillId -> second, RecoveryId -> recovery, OtherId -> NoInt)
pure def neutralChoicesI(first: OptionalInt, second: OptionalInt, recovery: OptionalInt): str -> OptionalInt =
  Map("settle" -> NoInt, "fill1" -> first, "fill2" -> second, "recover" -> recovery, "other" -> NoInt)
pure val initialBeforeI: APredecessor = {program: programI, state: {
  continuation: N4, accounts: accountsI(10), choices: choicesI(NoInt, NoInt, NoInt), minimumTime: Time2}}
pure val residualBeforeI: APredecessor = {program: programI, state: {
  continuation: N2, accounts: accountsI(5), choices: choicesI(IntValue(1), NoInt, NoInt), minimumTime: Time2}}
pure val filledBeforeI: APredecessor = {program: programI, state: {
  continuation: N0, accounts: accountsI(0), choices: choicesI(IntValue(1), IntValue(1), NoInt), minimumTime: Time2}}
pure def ledgerI(escrow: int, alice: int, bob: int): Ledger = LEDGER_KEYS.mapBy(_ => 0)
  .put((Escrow(aliceA), TokenA), escrow).put((Wallet(Alice), TokenA), alice).put((Wallet(Bob), TokenA), bob)
pure def environmentI(t: int): Environment = {physicalTime: t, anchor: 0, implementationVersion: 0, enforcementMechanism: 0}
pure def parentFactsI(residual: bool, cancelled: bool): ParentFacts = ParentPresent({
  cancelled: cancelled, usedSlots: if (residual) Set(1) else Set(), paid: if (residual) 5 else 0,
  remainingAllowance: if (residual) 5 else 10, revision: (if (residual) 1 else 0) + (if (cancelled) 1 else 0)})
pure def factsI(residual: bool, cancelled: bool, t: int): PolicyFacts = {
  ledger: if (residual) ledgerI(5, 0, 5) else ledgerI(10, 0, 0), environment: environmentI(t),
  parents: AUTHORITY_KEYS.mapBy(_ => ParentAbsent).put(parentKeyI, parentFactsI(residual, cancelled))}
pure def inputI(id: AChoiceId, chooser: Principal): AInput = PresentAInput(ChoiceInputA({id: id, chooser: chooser, chosen: 1}))
pure def transferI(recipient: Principal, q: int): Transfer =
  {source: Escrow(aliceA), destination: Wallet(recipient), asset: TokenA, quantity: q}
pure def projectedI(node: NodeId, q: int, first: OptionalInt, second: OptionalInt, recovery: OptionalInt,
  t: int, accepted: bool, error: CoreError, payments: List[CorePayment], reductions: int): CoreProjection[AContinuation] =
  CoreProjected({accepted: accepted, error: error, warnings: List(), payments: payments, reductions: reductions,
    state: {accounts: accountsI(q), choices: neutralChoicesI(first, second, recovery),
      continuation: {program: programI, node: node}, minimumTime: t}})
pure val fill1PlannedI: AAuthorityPlanned = {predecessor: initialBeforeI, proposedSuccessor: residualBeforeI,
  artifactAndCall: AgreementCallA({before: initialBeforeI, input: inputI(FirstFillId, Bob), now: Time2}),
  input: ChoiceLike({id: "fill1", chooser: Bob, chosen: 1}), operation: fill1I, transactionTime: 2,
  effects: List(transferI(Bob, 5)), predecessorFacts: factsI(false, false, 2),
  coreProjection: projectedI(N2, 5, IntValue(1), NoInt, NoInt, 2, true, NoCoreError,
    List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), 1)}
pure val fill2PlannedI: AAuthorityPlanned = {predecessor: residualBeforeI, proposedSuccessor: filledBeforeI,
  artifactAndCall: AgreementCallA({before: residualBeforeI, input: inputI(SecondFillId, Bob), now: Time2}),
  input: ChoiceLike({id: "fill2", chooser: Bob, chosen: 1}), operation: fill2I, transactionTime: 2,
  effects: List(transferI(Bob, 5)), predecessorFacts: factsI(true, false, 2),
  coreProjection: projectedI(N0, 0, IntValue(1), IntValue(1), NoInt, 2, true, NoCoreError,
    List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), 1)}
pure def cancelPlannedI(residual: bool): AAuthorityPlanned = {
  val before = if (residual) residualBeforeI else initialBeforeI
  {predecessor: before, proposedSuccessor: before, artifactAndCall: CancellationCallA({before: before, now: Time2}),
    input: NoInput, operation: cancelI, transactionTime: 2, effects: List(), coreProjection: NoCoreProjection,
    predecessorFacts: factsI(residual, false, 2)}
}
pure val cancel0PlannedI = cancelPlannedI(false)
pure val cancel1PlannedI = cancelPlannedI(true)
pure val parentPlanI: AResolvedPlan = {identity: InstallmentPlanA,
  operations: List(fill1PlannedI, fill2PlannedI, cancel0PlannedI, cancel1PlannedI)}
pure def residualI(scenario: ScenarioI): bool = match scenario { | TwoFillsI => true | RecoverI(r) => r.residual }
pure def modeI(scenario: ScenarioI): RecoveryModeI = match scenario { | TwoFillsI => Choice2I | RecoverI(r) => r.mode }
pure def timeI(mode: RecoveryModeI): ATime = match mode {
  | Choice2I => Time2 | Timeout100I => Time100 | Timeout101I => Time101 | Refuse100I => Time100 | Refuse101I => Time101 }
pure def refusedI(mode: RecoveryModeI): bool = Set(Refuse100I, Refuse101I).contains(mode)
pure def suppliedI(mode: RecoveryModeI): bool = mode == Choice2I or refusedI(mode)
pure def recoveryPlannedI(residual: bool, mode: RecoveryModeI): AAuthorityPlanned = {
  val before = if (residual) residualBeforeI else initialBeforeI
  val q = if (residual) 5 else 10
  val first = if (residual) IntValue(1) else NoInt
  val now = timeI(mode)
  val rejected = refusedI(mode)
  val after = if (rejected) before else {program: programI, state: {continuation: N0,
    accounts: accountsI(0), choices: choicesI(first, NoInt, if (suppliedI(mode)) IntValue(1) else NoInt), minimumTime: now}}
  {predecessor: before, proposedSuccessor: after,
    artifactAndCall: AgreementCallA({before: before, input: if (suppliedI(mode)) inputI(RecoveryId, Alice) else NoAInput, now: now}),
    input: if (suppliedI(mode)) ChoiceLike({id: "recover", chooser: Alice, chosen: 1}) else NoInput,
    operation: recoverI, transactionTime: timeValue(now), effects: if (rejected) List() else List(transferI(Alice, q)),
    predecessorFacts: factsI(residual, true, timeValue(now)),
    coreProjection: if (rejected) projectedI(if (residual) N2 else N4, q, first, NoInt, NoInt, 2,
      false, CoreErrorCode("contract_closed"), List(), 0)
    else projectedI(N0, 0, first, NoInt, if (suppliedI(mode)) IntValue(1) else NoInt, timeValue(now), true, NoCoreError,
      List({source: aliceA, recipient: Alice, asset: TokenA, quantity: q}), if (suppliedI(mode)) 1 else 2)}
}
pure def recoveryPlanI(scenario: ScenarioI): AResolvedPlan = {identity: RecoveryPlanA,
  operations: List(recoveryPlannedI(residualI(scenario), modeI(scenario)))}
pure def clauseI(planned: AAuthorityPlanned): Clause = {operation: planned.operation,
  conditions: List(InputIs(planned.input), ParentMatches({key: parentKeyI,
    expected: planned.predecessorFacts.parents.get(parentKeyI)}), SourceBalanceIs({
    location: Escrow(aliceA), asset: TokenA, quantity: planned.predecessorFacts.ledger.get((Escrow(aliceA), TokenA))})),
  requiredEffects: planned.effects, allowedEffects: planned.effects, effectOrder: ExactOrder}
pure val parentBodyI: PolicyBody = {key: parentKeyI, debitLocations: Set(Escrow(aliceA)),
  capabilities: Set(FirstFillCapability, SecondFillCapability, CancelCapability), disclosures: Set(),
  validFrom: 2, validUntil: 99, implementationVersion: 0, enforcementMechanism: 0,
  clauses: List(clauseI(fill1PlannedI), clauseI(fill2PlannedI), clauseI(cancel0PlannedI), clauseI(cancel1PlannedI))}
pure def policyI(body: PolicyBody, plan: AResolvedPlan, profile: SigningProfile): AAuthorityPolicy = {
  body: body, profile: profile, binding: if (profile == SignAfterResolve)
    AfterResolution({identity: plan, operations: plan.operations}) else BeforeResolution(AnyArtifactUnderMechanism)}
pure def parentPolicyI(profile: SigningProfile): AAuthorityPolicy = policyI(parentBodyI, parentPlanI, profile)
pure def recoveryPolicyI(scenario: ScenarioI, profile: SigningProfile): AAuthorityPolicy = {
  val planned = recoveryPlannedI(residualI(scenario), modeI(scenario))
  policyI({key: recoveryKeyI, debitLocations: Set(Escrow(aliceA)), capabilities: Set(RecoveryCapability), disclosures: Set(),
    validFrom: planned.transactionTime, validUntil: planned.transactionTime, implementationVersion: 0, enforcementMechanism: 0,
    clauses: List(clauseI(planned))}, recoveryPlanI(scenario), profile)
}
pure val unsignedI: AAuthorityExecution = {authority: {context: {candidate: initialBeforeI,
  ledger: ledgerI(10, 0, 0), environment: environmentI(2), registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused),
  parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)}, signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)},
  attempts: ATTEMPT_IDS.mapBy(_ => NoAttempt)}
pure def plannedForI(id: AttemptId, scenario: ScenarioI): AAuthorityPlanned = match id {
  | FirstFillAttempt => fill1PlannedI | SecondFillAttempt => fill2PlannedI
  | CancelAttempt => cancel0PlannedI | FreshCancelAttempt => cancel1PlannedI
  | _ => recoveryPlannedI(residualI(scenario), modeI(scenario)) }
pure def planForI(id: AttemptId, scenario: ScenarioI): AResolvedPlan =
  if (id == RecoveryAttempt) recoveryPlanI(scenario) else parentPlanI
pure def expectedObservationI(id: AttemptId, scenario: ScenarioI): AAuthorityObservation = {
  val p = plannedForI(id, scenario)
  {predecessor: p.predecessor, proposedSuccessor: p.proposedSuccessor, artifactAndCall: p.artifactAndCall,
    resolvedPlan: planForI(id, scenario), input: p.input, transactionTime: p.transactionTime,
    effects: p.effects, coreProjection: p.coreProjection, effectEvidence: EvidenceValid, display: PublicDisplay,
    outcome: if (id == FirstFillAttempt) FirstInstallment else if (id == SecondFillAttempt) SecondInstallment
      else if (id == RecoveryAttempt) (if (refusedI(modeI(scenario))) Rejected(CoreRejected(CoreErrorCode("contract_closed"))) else Recovery)
      else Cancellation}
}
pure def actualI(id: AttemptId, scenario: ScenarioI): AAuthorityAdaptation = {
  val p = plannedForI(id, scenario)
  val request = match p.artifactAndCall {
    | AgreementCallA(r) => r | CancellationCallA(r) => {before: r.before, input: NoAInput, now: r.now} }
  adaptAuthorityA(request, p.operation, planForI(id, scenario), PublicDisplay)
}
}
