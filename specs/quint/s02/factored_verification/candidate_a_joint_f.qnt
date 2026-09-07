module candidate_a_joint_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core.* from "../candidate_a_core"
  import candidate_a_projection.* from "../candidate_a_projection"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"

  type JointF = {evaluation: ATransactionEvaluation, extraction: AEffectExtraction}
  pure def diagnosticF(e: ATransactionEvaluation): JointF = {
    evaluation: e, extraction: match e {
      | OutsideModelDomainA => EffectOutsideDomainA
      | TransactionReductionBoundFailureA => EffectReductionBoundFailureA
      | TransactionInputPreconditionFailureA => EffectInputPreconditionFailureA
      | TransactionComputedA(_) => EffectsExtractedA(List())
    }}
  pure def rejectF(before: AState, error: CoreError): JointF =
    diagnosticF(rolledBackTransaction(before, error))
  pure def acceptF(input: AInput, pre: AReduction, finished: AReduction): JointF = {
    val actual = acceptedTransaction(finished)
    val prefix = pre.payments.foldl(List(), (items, payment) => items.append(paymentTransfer(payment)))
    val withDeposit = match input {
      | NoAInput => prefix
      | PresentAInput(supplied) => match supplied {
          | ChoiceInputA(_) => prefix
          | DepositInputA(deposit) => prefix.append({source: Wallet(deposit.depositor),
              destination: Escrow(deposit.account), asset: deposit.account.asset, quantity: deposit.quantity})
        }
    }
    val suffix = finished.payments.slice(pre.payments.length(), finished.payments.length())
    {evaluation: actual, extraction: EffectsExtractedA(
      suffix.foldl(withDeposit, (items, payment) => items.append(paymentTransfer(payment))))}
  }
  pure def computeJointF(program: AProgram, before: AState, supplied: AInput, now: ATime): JointF = {
    val prog = program
    val original = before
    val input = supplied
    val clock = now
    if (not(validState(prog, original)) or not(validInput(input))) diagnosticF(OutsideModelDomainA)
    else if (timeValue(clock) < timeValue(original.minimumTime))
      rejectF(original, CoreErrorCode("time_before_state"))
    else {
      val timed = {...original, minimumTime: clock}
      val preliminary = reduceToQuiescence(prog, timed)
      match preliminary {
        | QuiescenceOutsideDomainA => diagnosticF(OutsideModelDomainA)
        | ReductionBoundFailureA => diagnosticF(TransactionReductionBoundFailureA)
        | ReductionCompleteA(pre) => {
            val currentNode = prog.nodes.get(pre.state.continuation)
            match input {
              | NoAInput => match currentNode {
                  | CloseA => if (pre.reductions == 0 and pre.payments == List() and pre.warnings == List())
                      rejectF(original, CoreErrorCode("contract_closed"))
                    else acceptF(input, pre, pre)
                  | WhenA(_) => rejectF(original, CoreErrorCode("input_required"))
                  | _ => acceptF(input, pre, pre)
                }
              | PresentAInput(present) => match currentNode {
                  | WhenA(_) => {
                      val application = applyInput(prog, pre.state, present)
                      match application {
                        | InputOutsideDomainA => diagnosticF(OutsideModelDomainA)
                        | InputPreconditionFailureA => diagnosticF(TransactionInputPreconditionFailureA)
                        | InputAppliedA(applied) => if (applied.error != NoCoreError)
                            rejectF(original, applied.error)
                          else {
                            val following = reduceToQuiescence(prog, applied.state)
                            match following {
                              | QuiescenceOutsideDomainA => diagnosticF(OutsideModelDomainA)
                              | ReductionBoundFailureA => diagnosticF(TransactionReductionBoundFailureA)
                              | ReductionCompleteA(post) => {
                                  val finished = {state: post.state,
                                    payments: post.payments.foldl(pre.payments, (items, payment) => items.append(payment)),
                                    warnings: post.warnings.foldl(pre.warnings, (items, warning) => items.append(warning)),
                                    reductions: pre.reductions + post.reductions}
                                  acceptF(input, pre, finished)
                                }
                            }
                          }
                      }
                    }
                  | CloseA => rejectF(original, CoreErrorCode("contract_closed"))
                  | _ => rejectF(original, CoreErrorCode("no_matching_input"))
                }
            }
          }
      }
    }
  }
  pure def computeTransactionF(program: AProgram, before: AState, input: AInput, now: ATime): ATransactionEvaluation = {
    val joint = computeJointF(program, before, input, now)
    joint.evaluation
  }
  pure def extractCommittedEffectsF(program: AProgram, before: AState, input: AInput,
    now: ATime, supplied: ATransactionResult): AEffectExtraction = {
    val joint = computeJointF(program, before, input, now)
    match joint.evaluation {
      | TransactionComputedA(actual) => if (supplied != actual) EffectResultMismatchA else joint.extraction
      | _ => joint.extraction
    }
  }
  pure def adaptAuthorityF(request: AAuthorityRequest, op: Operation,
    plan: AResolvedPlan, display: DisplayProjection): AAuthorityAdaptation = {
    val req = request
    val operation = op
    val resolved = plan
    val shown = display
    if (not(validOperation(operation)) or req.now == Time0) AuthorityInvalidRequestA
    else match operation {
      | OpCancelParent(_) => if (not(validState(req.before.program, req.before.state))
          or req.input != NoAInput) AuthorityInvalidRequestA
        else AuthorityAdaptedA({predecessor: req.before, proposedSuccessor: req.before,
          artifactAndCall: CancellationCallA({before: req.before, now: req.now}), resolvedPlan: resolved,
          transactionTime: timeValue(req.now), input: NoInput, effects: List(), outcome: Cancellation,
          coreProjection: NoCoreProjection, effectEvidence: EvidenceValid, display: shown})
      | _ => {
          val joint = computeJointF(req.before.program, req.before.state, req.input, req.now)
          match joint.evaluation {
            | TransactionComputedA(raw) => match joint.extraction {
                | EffectsExtractedA(effects) => AuthorityAdaptedA({predecessor: req.before,
                    proposedSuccessor: {program: req.before.program, state: raw.state},
                    artifactAndCall: AgreementCallA(req), resolvedPlan: resolved,
                    transactionTime: timeValue(req.now), input: projectInput(req.input), effects: effects,
                    outcome: if (raw.accepted) acceptedOutcomeA(operation) else Rejected(CoreRejected(raw.error)),
                    coreProjection: CoreProjected(projectResult(req.before.program, raw)),
                    effectEvidence: EvidenceValid, display: shown})
                | _ => AuthorityExtractionDiagnosticA(joint.extraction)
              }
            | _ => AuthorityComputationDiagnosticA(joint.evaluation)
          }
        }
    }
  }
}
