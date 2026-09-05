module candidate_a_authority_adapter {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import execution.* from "./execution"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_core.* from "./candidate_a_core"
  import candidate_a_projection.* from "./candidate_a_projection"

  type AAuthorityRequest = {before: APredecessor, input: AInput, now: ATime}
  type ACall = AgreementCallA(AAuthorityRequest)
    | CancellationCallA({before: APredecessor, now: ATime})
  type APlanId = SwapPlanA | InstallmentPlanA | RecoveryPlanA
  type AResolvedPlan = {identity: APlanId,
    operations: List[PlannedOperation[APredecessor,AContinuation,ACall]]}
  type AAuthorityObservation = CandidateObservation[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityPolicy = Policy[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityExecution = ExecutionState[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityEvidence = EvidenceBundle[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityAdaptation = AuthorityAdaptedA(AAuthorityObservation)
    | AuthorityComputationDiagnosticA(ATransactionEvaluation)
    | AuthorityExtractionDiagnosticA(AEffectExtraction)
    | AuthorityInvalidRequestA

  pure def acceptedOutcomeA(op: Operation): SemanticOutcome = match op {
    | OpFund => FundingAccepted
    | OpSettle => Settlement
    | OpVoluntaryRefund => VoluntaryRefund
    | OpDeadlineRefund => DeadlineRefund
    | OpFillSlot(fill) => if (fill.slot == 1) FirstInstallment else SecondInstallment
    | OpRecover(_) => Recovery
    | OpCancelParent(_) => Cancellation
  }

  pure def adaptAuthorityA(request: AAuthorityRequest, op: Operation,
    plan: AResolvedPlan, display: DisplayProjection): AAuthorityAdaptation =
    if (not(validOperation(op)) or request.now == Time0) AuthorityInvalidRequestA
    else match op {
      | OpCancelParent(_) => if (not(validState(request.before.program, request.before.state))
          or request.input != NoAInput) AuthorityInvalidRequestA
        else AuthorityAdaptedA({predecessor: request.before, proposedSuccessor: request.before,
          artifactAndCall: CancellationCallA({before: request.before, now: request.now}), resolvedPlan: plan,
          transactionTime: timeValue(request.now), input: NoInput, effects: List(), outcome: Cancellation,
          coreProjection: NoCoreProjection, effectEvidence: EvidenceValid, display: display})
      | _ => match computeTransaction(request.before.program, request.before.state, request.input, request.now) {
          | TransactionComputedA(raw) => match extractCommittedEffects(request.before.program,
              request.before.state, request.input, request.now, raw) {
              | EffectsExtractedA(effects) => AuthorityAdaptedA({predecessor: request.before,
                  proposedSuccessor: {program: request.before.program, state: raw.state},
                  artifactAndCall: AgreementCallA(request), resolvedPlan: plan,
                  transactionTime: timeValue(request.now), input: projectInput(request.input), effects: effects,
                  outcome: if (raw.accepted) acceptedOutcomeA(op) else Rejected(CoreRejected(raw.error)),
                  coreProjection: CoreProjected(projectResult(request.before.program, raw)),
                  effectEvidence: EvidenceValid, display: display})
              | EffectResultMismatchA => AuthorityExtractionDiagnosticA(EffectResultMismatchA)
              | EffectOutsideDomainA => AuthorityExtractionDiagnosticA(EffectOutsideDomainA)
              | EffectReductionBoundFailureA => AuthorityExtractionDiagnosticA(EffectReductionBoundFailureA)
              | EffectInputPreconditionFailureA => AuthorityExtractionDiagnosticA(EffectInputPreconditionFailureA)
            }
          | OutsideModelDomainA => AuthorityComputationDiagnosticA(OutsideModelDomainA)
          | TransactionReductionBoundFailureA => AuthorityComputationDiagnosticA(TransactionReductionBoundFailureA)
          | TransactionInputPreconditionFailureA => AuthorityComputationDiagnosticA(TransactionInputPreconditionFailureA)
        }
    }
  pure def authorityObservationMatchesA(op: Operation, obs: AAuthorityObservation): bool =
    match obs.artifactAndCall {
      | AgreementCallA(request) => adaptAuthorityA(request, op, obs.resolvedPlan, obs.display)
          == AuthorityAdaptedA(obs)
      | CancellationCallA(call) => adaptAuthorityA({before: call.before, input: NoAInput,
          now: call.now}, op, obs.resolvedPlan, obs.display) == AuthorityAdaptedA(obs)
    }
}
