module candidate_a_integrated_lowering {
import effects.* from "./effects"
import consumption.* from "./consumption"
import observations.* from "./observations"
import policies.* from "./policies"
import authorization.* from "./authorization"
import execution.* from "./execution"
import candidate_a_types.* from "./candidate_a_types"
import candidate_a_programs.* from "./candidate_a_programs"
import candidate_a_core.* from "./candidate_a_core"
import candidate_a_projection.* from "./candidate_a_projection"
import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"
import candidate_a_authority_installment_fixtures.* from "./candidate_a_authority_installment_fixtures"
import candidate_a_authority_installment.* from "./candidate_a_authority_installment"
import candidate_a_authority_swap_fixtures.* from "./candidate_a_authority_swap_fixtures"
import candidate_a_authority_swap.* from "./candidate_a_authority_swap"
import candidate_a_integrated_observer.* from "./candidate_a_integrated_observer"

type A4EvidenceMode = NormalEvidenceA4 | ParentEvidenceA4 | WrongSignerEvidenceA4 | WrongNonceEvidenceA4
type A4Instruction = LifecycleIA4(CommandI) | LifecycleSA4(CommandS)
  | DirectLifecycleIA4(CommandI) | DirectLifecycleSA4(CommandS)
  | ReplayProposalA4(AttemptId) | ReplayCommitA4(AttemptId)
  | ParentCancellationA4 | ParentSlotA4(int) | RecoveryFinancialA4 | DuplicateCancellationA4
  | VerifyEvidenceA4({id: AttemptId, mode: A4EvidenceMode})
  | RejectEvidenceA4({id: AttemptId, mode: A4EvidenceMode})
  | RejectConstructedA4(AttemptId) | MutateA4(A4DerivationStage)
  | StaleSignA4 | BadPlanA4 | BadPrepareA4 | BadProposeA4 | WrongCoreA4
type A4Step = {kind: str, instruction: A4Instruction}
type A4Case = {descriptor: A4CaseDescriptor, scenario: A4Scenario,
  profile: SigningProfile, steps: List[A4Step]}
pure def diagnosticA4(message: str): A4Arguments = directA4(DiagnosticA4(ObserverIssueA4(message)))
pure def proposeAdaptedA4(id: AttemptId, op: Operation, actor: Principal,
  call: ACall, adaptation: AAuthorityAdaptation): A4Command = match adaptation {
    | AuthorityAdaptedA(obs) => ProposeA4({id: id, operation: op, observation: obs, actor: actor})
    | _ => DiagnosticA4(AdapterIssueA4({call: call, adaptation: adaptation}))
  }
pure def caseEvidenceA4(c: A4Case, a: AAuthorityAttempt, mode: A4EvidenceMode): AAuthorityEvidence = {
  val normal = match c.scenario {
    | InstallmentScenarioA4(s) => evidenceI(a, if (mode == ParentEvidenceA4)
        parentPolicyI(c.profile) else selectedPolicyI(a.id, s, c.profile))
    | SwapScenarioA4(s) => evidenceS(a, s, c.profile) }
  val key = keyS(Alice, 1)
  if (mode == WrongSignerEvidenceA4) {...normal, signatures: normal.signatures.put(key,
    {...normal.signatures.get(key), signed: {...normal.signatures.get(key).signed, signer: Mallory}})}
  else if (mode == WrongNonceEvidenceA4) {...normal, signatures:
    normal.signatures.keys().exclude(Set(key)).mapBy(k => normal.signatures.get(k)).put(keyS(Alice, 0),
      {...normal.signatures.get(key), signed: {policy: policyS(FundingOneAttempt, staleScenarioS, Alice, c.profile),
        signer: Alice, token: 0}})}
  else normal
}
pure def lowerIA4(state: AAuthorityExecution, c: A4Case, s: ScenarioI, cmd: CommandI): A4Command = match cmd {
  | PrepareParentI => PrepareA4({policy: parentPolicyI(c.profile), signer: Alice})
  | SignParentI => SignA4({policy: parentPolicyI(c.profile), signer: Alice, token: 0})
  | PrepareRecoveryI => PrepareA4({policy: recoveryPolicyI(s, c.profile), signer: Alice})
  | SignRecoveryI => SignA4({policy: recoveryPolicyI(s, c.profile), signer: Alice, token: 0})
  | ProposeI(id) => proposeAdaptedA4(id, plannedForI(id, s).operation,
      if (Set(FirstFillAttempt, SecondFillAttempt).contains(id)) Bob else Alice, plannedForI(id, s).artifactAndCall, actualI(id, s))
  | VerifyI(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => VerifyA4({id: id, evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing verify attempt")) }
  | CommitI(id) => CommitA4({id: id})
  | RejectStaleI(id) => RejectVerifiedA4({id: id})
  | AdvanceI => AdvanceA4({environment: environmentI(timeValue(timeI(modeI(s))))})
  | RejectRecoveryI => match attemptA4(state, RecoveryAttempt) {
      | PresentAttemptA4(a) => RejectProposedA4({id: RecoveryAttempt,
          evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing recovery attempt")) }
}
pure def lowerSA4(state: AAuthorityExecution, c: A4Case, s: ScenarioS, cmd: CommandS): A4Command = match cmd {
  | PrepareS(x) => PrepareA4({policy: policyS(x.id, s, x.principal, c.profile), signer: x.principal})
  | SignS(x) => SignA4({policy: policyS(x.id, s, x.principal, c.profile), signer: x.principal, token: 0})
  | ProposeS(id) => proposeAdaptedA4(id, plannedS(id, s).operation,
      if (id == FundingOneAttempt) Alice else Bob, plannedS(id, s).artifactAndCall, actualS(id, s))
  | VerifyS(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => VerifyA4({id: id, evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing verify attempt")) }
  | CommitS(id) => CommitA4({id: id})
  | AdvanceS(t) => AdvanceA4({environment: environmentS(timeValue(t))})
  | RejectProposedS(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => RejectProposedA4({id: id, evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing rejected attempt")) }
  | RejectVerifiedS(id) => RejectVerifiedA4({id: id})
}
pure val badPlanA4: AResolvedPlan = {
  val original = fundingPlannedS(Alice)
  {identity: SwapPlanA, operations: List(original, {...original, proposedSuccessor: beforeS(0)})}
}
pure def badPolicyA4(p: SigningProfile): AAuthorityPolicy = {
  val original = policyS(FundingOneAttempt, staleScenarioS, Alice, p)
  {...original, binding: if (p == SignAfterResolve)
    AfterResolution({identity: badPlanA4, operations: badPlanA4.operations})
    else BeforeResolution(AnyArtifactUnderMechanism)}
}
pure def mutatedA4(a: AAuthorityAttempt, mutation: str): AAuthorityAttempt = {
  val obs = a.observation
  val changed = if (mutation == "unused-successor") {...obs, proposedSuccessor: {...obs.proposedSuccessor,
    program: {...obs.proposedSuccessor.program, nodes: obs.proposedSuccessor.program.nodes.put(N15,
      PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))}}}
    else if (mutation == "reversed-effects") {...obs, effects: List(obs.effects.nth(1), obs.effects.nth(0))}
    else if (mutation == "reductions") {...obs, coreProjection: match obs.coreProjection {
      | CoreProjected(r) => CoreProjected({...r, reductions: r.reductions + 1})
      | NoCoreProjection => NoCoreProjection }}
    else if (mutation == "neutral-chooser") {...obs, input: ChoiceLike({id: "settle", chooser: Alice, chosen: 1})}
    else {...obs, resolvedPlan: {...obs.resolvedPlan, operations: List({
      ...obs.resolvedPlan.operations.nth(0), predecessorFacts: {
        ...obs.resolvedPlan.operations.nth(0).predecessorFacts, environment: environmentS(100)}})}}
  {...a, observation: changed}
}

pure def evidenceCommandA4(state: AAuthorityExecution, c: A4Case, id: AttemptId,
  mode: A4EvidenceMode, rejecting: bool): A4Command = match attemptA4(state, id) {
    | PresentAttemptA4(a) => if (rejecting) RejectProposedA4({id: id, evidence: caseEvidenceA4(c, a, mode)})
        else VerifyA4({id: id, evidence: caseEvidenceA4(c, a, mode)})
    | _ => DiagnosticA4(ObserverIssueA4("missing evidence attempt"))
  }
pure def buildA4(state: AAuthorityExecution, c: A4Case, instruction: A4Instruction): A4Arguments = match instruction {
  | LifecycleIA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => {guard: InstallmentCommandGuardA4(cmd), command: lowerIA4(state, c, s, cmd)}
      | _ => diagnosticA4("foreign installment command") }
  | LifecycleSA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => {guard: SwapCommandGuardA4(cmd), command: lowerSA4(state, c, s, cmd)}
      | _ => diagnosticA4("foreign swap command") }
  | DirectLifecycleIA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => directA4(lowerIA4(state, c, s, cmd))
      | _ => diagnosticA4("foreign direct installment command") }
  | DirectLifecycleSA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => directA4(lowerSA4(state, c, s, cmd))
      | _ => diagnosticA4("foreign direct swap command") }
  | ReplayProposalA4(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => directA4(ProposeA4({id: id, operation: a.operation, observation: a.observation, actor: a.actor}))
      | _ => diagnosticA4("missing replay attempt") }
  | ReplayCommitA4(id) => directA4(CommitA4({id: id}))
  | ParentCancellationA4 => match state.authority.context.parents.get(parentKeyI) {
      | ParentLive(live) => directA4(CancelParentProbeA4({parent: live.parent, entry: live.entry, prepared: live.entry}))
      | _ => diagnosticA4("missing cancellation parent") }
  | ParentSlotA4(slot) => match state.authority.context.parents.get(parentKeyI) {
      | ParentLive(live) => directA4(ConsumeSlotProbeA4({parent: live.parent, entry: live.entry, slot: slot, prepared: live.entry}))
      | _ => diagnosticA4("missing slot parent") }
  | RecoveryFinancialA4 => {
      val adaptation = actualI(RecoveryAttempt, RecoverI({residual: false, mode: Choice2I}))
      match adaptation {
        | AuthorityAdaptedA(obs) => directA4(FinancialProbeA4({operation: recoverI, effects: obs.effects}))
        | _ => directA4(DiagnosticA4(AdapterIssueA4({call: plannedForI(RecoveryAttempt,
            RecoverI({residual: false, mode: Choice2I})).artifactAndCall, adaptation: adaptation}))) }
    }
  | DuplicateCancellationA4 => directA4(proposeAdaptedA4(FreshCancelAttempt, cancelI, Alice, cancel0PlannedI.artifactAndCall,
      actualI(CancelAttempt, RecoverI({residual: false, mode: Choice2I}))))
  | VerifyEvidenceA4(x) => directA4(evidenceCommandA4(state, c, x.id, x.mode, false))
  | RejectEvidenceA4(x) => directA4(evidenceCommandA4(state, c, x.id, x.mode, true))
  | RejectConstructedA4(id) => directA4(RejectVerifiedA4({id: id}))
  | MutateA4(stage) => {
      val id = if (c.descriptor.lifecycle == "installment") FirstFillAttempt else DispositionAttempt
      match attemptA4(state, id) {
        | PresentAttemptA4(a) => {
            val changed = if (stage == ProposedDerivationA4) mutatedA4(a, c.descriptor.control) else a
            directA4(DeriveA4({mutationId: c.descriptor.control, baseCaseId: c.descriptor.caseId,
              baseSequence: if (id == FirstFillAttempt) 3 else 16, attempt: changed,
              evidence: caseEvidenceA4(c, changed, NormalEvidenceA4), stage: stage}))
          }
        | _ => diagnosticA4("missing mutation base") }
    }
  | StaleSignA4 => directA4(SignA4({policy: policyS(DispositionAttempt, staleScenarioS, Alice, c.profile), signer: Alice, token: 0}))
  | BadPlanA4 => directA4(PlanMatchesProbeA4({plan: badPlanA4}))
  | BadPrepareA4 => directA4(PrepareA4({policy: badPolicyA4(c.profile), signer: Alice}))
  | BadProposeA4 => {
      val adaptation = actualS(FundingOneAttempt, staleScenarioS)
      match adaptation {
        | AuthorityAdaptedA(obs) => directA4(ProposeA4({id: FundingOneAttempt, operation: OpFund,
            observation: {...obs, resolvedPlan: badPlanA4}, actor: Alice}))
        | _ => directA4(DiagnosticA4(AdapterIssueA4({call: fundingPlannedS(Alice).artifactAndCall, adaptation: adaptation}))) }
    }
  | WrongCoreA4 => directA4(CoreAcceptedProbeA4({request: {before: beforeS(2),
      input: PresentAInput(ChoiceInputA({id: SettleId, chooser: Alice, chosen: 1})), now: Time2}}))
}
pure def guardA4(state: AAuthorityExecution, c: A4Case, args: A4Arguments): bool = match args.guard {
  | InstallmentCommandGuardA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => args.command == lowerIA4(state, c, s, cmd) and canCommandI(state, s, c.profile, cmd)
      | _ => false }
  | SwapCommandGuardA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => args.command == lowerSA4(state, c, s, cmd) and canCommandS(state, s, c.profile, cmd)
      | _ => false }
  | _ => args.guard == directTagA4(args.command) and directGuardA4(state, args.command)
}
pure def applyA4(state: AAuthorityExecution, c: A4Case, args: A4Arguments): AAuthorityExecution = match args.guard {
  | InstallmentCommandGuardA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => applyCommandI(state, s, c.profile, cmd) | _ => state }
  | SwapCommandGuardA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => applyCommandS(state, s, c.profile, cmd) | _ => state }
  | _ => directApplyA4(state, args.command)
}
pure def initialA4(c: A4Case): AAuthorityExecution = match c.scenario {
  | InstallmentScenarioA4(_) => unsignedI | SwapScenarioA4(_) => unsignedS }
pure def observeA4(state: AAuthorityExecution, c: A4Case, sequence: int, step: A4Step): A4Event = {
  val args = buildA4(state, c, step.instruction)
  val actualGuard = guardA4(state, c, args)
  val records = computationsA4(state, args.command)
  val diagnostic = match args.command { | DiagnosticA4(_) => true | _ => false }
  val expectedGuard = step.kind != "denied-probe"
  {caseId: c.descriptor.caseId, profile: c.descriptor.profile, sequence: sequence,
    kind: if (diagnostic or actualGuard != expectedGuard or not(records.foldl(true, (ok, x) => ok and computedA4(x))))
      "diagnostic" else step.kind, arguments: args, observedGuard: actualGuard, computations: records}
}
pure def startA4(c: A4Case): A4Event = {caseId: c.descriptor.caseId, profile: c.descriptor.profile,
  sequence: 0, kind: "case-start", arguments: directA4(CaseStartA4(c.descriptor)), observedGuard: true, computations: List()}
pure def financiallyDoneA4(state: AAuthorityExecution, c: A4Case): bool = match c.scenario {
  | InstallmentScenarioA4(_) => financialTerminalI(state)
  | SwapScenarioA4(_) => financialTerminalS(state) }
pure def retainedRefusalA4(state: AAuthorityExecution, c: A4Case): bool = {
  val id = if (c.descriptor.lifecycle == "installment") RecoveryAttempt else DispositionAttempt
  val noPending = if (c.descriptor.lifecycle == "installment") noPendingI(state) else noPendingS(state)
  noPending and match state.attempts.get(id) {
    | RejectedOperation(r) => r.stage == VerificationBoundary and
        (match r.reason { | CoreRejected(_) => true | UnauthorizedEffect => c.descriptor.lifecycle == "swap" | _ => false })
    | _ => false }
}
pure def negativeDoneA4(state: AAuthorityExecution, previous: A4Event): bool =
  (previous.kind == "denied-probe" and not(previous.observedGuard))
    or (previous.kind == "transition" and previous.observedGuard and match previous.arguments.command {
      | RejectProposedA4(c) => match state.attempts.get(c.id) { | RejectedOperation(_) => true | _ => false }
      | RejectVerifiedA4(c) => match state.attempts.get(c.id) { | RejectedOperation(_) => true | _ => false }
      | _ => false })
pure def endA4(state: AAuthorityExecution, c: A4Case, previous: A4Event): A4Event = {
  val ordinary = c.descriptor.control == "ordinary"
  val financial = financiallyDoneA4(state, c)
  val ready = if (ordinary) financial or retainedRefusalA4(state, c) else negativeDoneA4(state, previous)
  {caseId: c.descriptor.caseId, profile: c.descriptor.profile, sequence: c.steps.length() + 1,
    kind: if (ready) "case-end" else "diagnostic", observedGuard: ready, computations: List(),
    arguments: directA4(CaseEndA4({status: if (not(ordinary)) "negative-complete"
      else if (financial) "financial-terminal" else "refusal-terminal"}))}
}
}
