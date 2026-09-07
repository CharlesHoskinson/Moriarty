module candidate_a_authority_adapter_test_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import consumption.* from "../consumption"
  import policies.* from "../policies"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter_f.* from "./candidate_a_authority_adapter_f"

  pure val emptyAdapterPlan: AResolvedPlan = {
    identity: InstallmentPlanA, operations: List()}
  pure val firstRequest: AAuthorityRequest = {
    before: {program: installmentProgram,
      state: {...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10),
        minimumTime: Time2}},
    input: PresentAInput(ChoiceInputA({id: FirstFillId, chooser: Bob, chosen: 1})),
    now: Time2}
  pure val parentKeyA = {domain: InstallmentDomain, principal: Alice, nonce: 0}
  run firstFillActualInputTest = {
    match adaptAuthorityA(firstRequest, OpFillSlot({parent: parentKeyA, slot: 1}),
      emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => all {
          obs.input == ChoiceLike({id: "fill1", chooser: Bob, chosen: 1}),
          obs.proposedSuccessor.state.continuation == N2,
          obs.proposedSuccessor.state.accounts.get(aliceA) == 5,
          obs.effects == List({source: Escrow(aliceA), destination: Wallet(Bob),
            asset: TokenA, quantity: 5}),
          authorityObservationMatchesA(OpFillSlot({parent: parentKeyA, slot: 1}), obs),
        }
      | _ => false
    }
  }
  run cancellationIdentityEvidenceTest = {
    match adaptAuthorityA({before: firstRequest.before, input: NoAInput, now: Time2},
      OpCancelParent(parentKeyA), emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => obs.proposedSuccessor == firstRequest.before and obs.effects == List()
          and obs.coreProjection == NoCoreProjection and obs.effectEvidence == EvidenceValid
      | _ => false
    }
  }
  pure def installmentRequest(state: AState, input: AInput, now: ATime): AAuthorityRequest =
    {before: {program: installmentProgram, state: state}, input: input, now: now}
  pure val refund10 = {source: Escrow(aliceA), destination: Wallet(Alice), asset: TokenA, quantity: 10}
  pure val refund5 = {source: Escrow(aliceA), destination: Wallet(Alice), asset: TokenA, quantity: 5}
  run agreementScenarioCoverageTest = {
    val deposit = {before: {program: canonicalSwap, state: emptyAState(N6)},
      input: PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 10})), now: Time2}
    val second = installmentRequest({...emptyAState(N2), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 5),
      choices: A_CHOICE_IDS.mapBy(_ => NoInt).put(FirstFillId, IntValue(1)), minimumTime: Time2},
      PresentAInput(ChoiceInputA({id: SecondFillId, chooser: Bob, chosen: 1})), Time2)
    val recovery10 = installmentRequest({...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10), minimumTime: Time2},
      PresentAInput(ChoiceInputA({id: RecoveryId, chooser: Alice, chosen: 1})), Time2)
    val recovery5 = installmentRequest({...emptyAState(N2), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 5), minimumTime: Time2},
      PresentAInput(ChoiceInputA({id: RecoveryId, chooser: Alice, chosen: 1})), Time2)
    val timeout = installmentRequest({...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10), minimumTime: Time2}, NoAInput, Time100)
    all {
      match adaptAuthorityA(deposit, OpFund, emptyAdapterPlan, PublicDisplay) { | AuthorityAdaptedA(o) => o.effects == List({source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 10}) and o.outcome == FundingAccepted | _ => false },
      match adaptAuthorityA(second, OpFillSlot({parent: parentKeyA, slot: 2}), emptyAdapterPlan, PublicDisplay) { | AuthorityAdaptedA(o) => o.proposedSuccessor.state.continuation == N0 and o.effects == List({source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 5}) and o.outcome == SecondInstallment | _ => false },
      match adaptAuthorityA(recovery10, OpRecover({parent: parentKeyA, recovery: {...parentKeyA, nonce: 1}}), emptyAdapterPlan, PublicDisplay) { | AuthorityAdaptedA(o) => o.effects == List(refund10) and o.outcome == Recovery | _ => false },
      match adaptAuthorityA(recovery5, OpRecover({parent: parentKeyA, recovery: {...parentKeyA, nonce: 1}}), emptyAdapterPlan, PublicDisplay) { | AuthorityAdaptedA(o) => o.effects == List(refund5) and o.outcome == Recovery | _ => false },
      match adaptAuthorityA(timeout, OpDeadlineRefund, emptyAdapterPlan, PublicDisplay) { | AuthorityAdaptedA(o) => o.effects == List(refund10) and o.input == NoInput | _ => false },
    }
  }
  run rejectionAndDiagnosticCoverageTest = {
    val deadlineInput = installmentRequest({...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10), minimumTime: Time2}, PresentAInput(ChoiceInputA({id: RecoveryId, chooser: Alice, chosen: 1})), Time100)
    val malformed: AAuthorityRequest = {...firstRequest, before: {...firstRequest.before, program: {root: N4, nodes: Map()}}}
    all {
      match adaptAuthorityA(deadlineInput, OpRecover({parent: parentKeyA, recovery: {...parentKeyA, nonce: 1}}), emptyAdapterPlan, PublicDisplay) { | AuthorityAdaptedA(o) => o.outcome == Rejected(CoreRejected(CoreErrorCode("contract_closed"))) and o.effects == List() | _ => false },
      adaptAuthorityA({...firstRequest, now: Time0}, OpFillSlot({parent: parentKeyA, slot: 1}), emptyAdapterPlan, PublicDisplay) == AuthorityInvalidRequestA,
      adaptAuthorityA(firstRequest, OpFillSlot({parent: {...parentKeyA, nonce: 1}, slot: 1}), emptyAdapterPlan, PublicDisplay) == AuthorityInvalidRequestA,
      match adaptAuthorityA(malformed, OpFund, emptyAdapterPlan, PublicDisplay) { | AuthorityComputationDiagnosticA(OutsideModelDomainA) => true | _ => false },
    }
  }
  run bindingMutationControlsTest = {
    match adaptAuthorityA(firstRequest, OpFillSlot({parent: parentKeyA, slot: 1}), emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => all {
          authorityObservationMatchesA(OpFillSlot({parent: parentKeyA, slot: 1}), obs),
          not(authorityObservationMatchesA(OpFillSlot({parent: parentKeyA, slot: 1}), {...obs, input: NoInput})),
          not(authorityObservationMatchesA(OpFillSlot({parent: parentKeyA, slot: 1}), {...obs, effects: List()})),
          not(authorityObservationMatchesA(OpFillSlot({parent: parentKeyA, slot: 1}), {...obs, outcome: Rejected(CoreRejected(NoCoreError))})),
          not(authorityObservationMatchesA(OpFillSlot({parent: parentKeyA, slot: 1}), {...obs, proposedSuccessor: obs.predecessor})),
        }
      | _ => false
    }
  }

  // Supplemental assertions against inherited behavior, not retroactive RED.
  pure val firstOperation = OpFillSlot({parent: parentKeyA, slot: 1})
  pure val recoverOperation = OpRecover({parent: parentKeyA, recovery: {...parentKeyA, nonce: 1}})
  pure val recoverInput = PresentAInput(ChoiceInputA({id: RecoveryId, chooser: Alice, chosen: 1}))
  pure val expectedInitialProjection: CoreStateObservation[AContinuation] = {
    accounts: firstRequest.before.state.accounts,
    choices: Map("settle" -> NoInt, "fill1" -> NoInt, "fill2" -> NoInt, "recover" -> NoInt, "other" -> NoInt),
    continuation: {program: installmentProgram, node: N4}, minimumTime: 2}

  run depositHasNoCorePaymentsTest = {
    val request: AAuthorityRequest = {before: {program: canonicalSwap, state: emptyAState(N6)},
      input: PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 10})), now: Time2}
    match adaptAuthorityA(request, OpFund, emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => obs.predecessor == request.before
          and obs.artifactAndCall == AgreementCallA(request)
          and obs.input == DepositLike({location: Escrow(aliceA), depositor: Alice, asset: TokenA, quantity: 10})
          and obs.proposedSuccessor.state.continuation == N5
          and obs.proposedSuccessor.state.accounts.get(aliceA) == 10
          and obs.effects == List({source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 10})
          and match obs.coreProjection {
            | CoreProjected(result) => result.accepted and result.payments == List()
                and result.warnings == List() and result.reductions == 0 and result.error == NoCoreError
            | _ => false
          }
      | _ => false
    }
  }
  run timeoutProjectionTwoReductionsTest = {
    val request: AAuthorityRequest = {...firstRequest, input: NoAInput, now: Time100}
    match adaptAuthorityA(request, OpDeadlineRefund, emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => obs.effects == List(refund10) and obs.outcome == DeadlineRefund
          and obs.transactionTime == 100 and obs.proposedSuccessor.state.minimumTime == Time100
          and match obs.coreProjection {
            | CoreProjected(result) => result.accepted and result.reductions == 2
                and result.state.continuation == {program: installmentProgram, node: N0}
                and result.payments == List({source: aliceA, recipient: Alice, asset: TokenA, quantity: 10})
                and result.warnings == List() and result.error == NoCoreError
            | _ => false
          }
      | _ => false
    }
  }
  run suppliedDeadlineRecoveryFullRollbackTest = {
    // Nontrivial stored values make absence/zero and complete choice rollback visible.
    val before: APredecessor = {...firstRequest.before, state: {...firstRequest.before.state,
      choices: firstRequest.before.state.choices.put(FirstFillId, IntValue(0)).put(OtherId, IntValue(-1))}}
    val request: AAuthorityRequest = {before: before, input: recoverInput, now: Time100}
    match adaptAuthorityA(request, recoverOperation, emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => obs.predecessor == before and obs.proposedSuccessor == before
          and obs.artifactAndCall == AgreementCallA(request) and obs.effects == List()
          and obs.input == ChoiceLike({id: "recover", chooser: Alice, chosen: 1})
          and obs.transactionTime == 100 and obs.effectEvidence == EvidenceValid
          and obs.outcome == Rejected(CoreRejected(CoreErrorCode("contract_closed")))
          and obs.coreProjection == CoreProjected({accepted: false, error: CoreErrorCode("contract_closed"),
            state: {...expectedInitialProjection, choices: expectedInitialProjection.choices
              .put("fill1", IntValue(0)).put("other", IntValue(-1))},
            payments: List(), warnings: List(), reductions: 0})
          and authorityObservationMatchesA(recoverOperation, obs)
      | _ => false
    }
  }
  run cancellationRejectsSuppliedInputTest =
    adaptAuthorityA(firstRequest, OpCancelParent(parentKeyA), emptyAdapterPlan, PublicDisplay) == AuthorityInvalidRequestA
      and adaptAuthorityA({...firstRequest, input: recoverInput}, OpCancelParent(parentKeyA),
        emptyAdapterPlan, PublicDisplay) == AuthorityInvalidRequestA

  run malformedStateAndInputDiagnosticsTest = {
    val missingAccounts: AState = {...firstRequest.before.state, accounts: Map()}
    val missingChoices: AState = {...firstRequest.before.state, choices: Map()}
    val negative: AState = {...firstRequest.before.state, accounts: firstRequest.before.state.accounts.put(aliceA, -1)}
    val invalidChoice: AState = {...firstRequest.before.state, choices: firstRequest.before.state.choices.put(OtherId, IntValue(3))}
    all {
      List(missingAccounts, missingChoices, negative, invalidChoice).foldl(true, (ok, state) => ok
        and adaptAuthorityA({...firstRequest, before: {...firstRequest.before, state: state}}, firstOperation,
          emptyAdapterPlan, PublicDisplay) == AuthorityComputationDiagnosticA(OutsideModelDomainA)),
      adaptAuthorityA({...firstRequest, input: PresentAInput(ChoiceInputA({id: FirstFillId, chooser: Bob, chosen: 3}))},
        firstOperation, emptyAdapterPlan, PublicDisplay) == AuthorityComputationDiagnosticA(OutsideModelDomainA),
      adaptAuthorityA({...firstRequest, input: PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 3}))},
        OpFund, emptyAdapterPlan, PublicDisplay) == AuthorityComputationDiagnosticA(OutsideModelDomainA),
      adaptAuthorityA({...firstRequest, input: NoAInput, before: {...firstRequest.before, state: missingAccounts}},
        OpCancelParent(parentKeyA), emptyAdapterPlan, PublicDisplay) == AuthorityInvalidRequestA,
    }
  }
  run predecessorArtifactAndTimeBindingTest = {
    val changedProgram: AProgram = {...installmentProgram, nodes: installmentProgram.nodes.put(N15,
      PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))}
    match adaptAuthorityA(firstRequest, firstOperation, emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => all {
          authorityObservationMatchesA(firstOperation, obs),
          not(authorityObservationMatchesA(firstOperation, {...obs, predecessor: {...obs.predecessor,
            state: {...obs.predecessor.state, minimumTime: Time1}}})),
          not(authorityObservationMatchesA(firstOperation, {...obs, artifactAndCall: AgreementCallA(
            {...firstRequest, before: {...firstRequest.before, program: changedProgram}})})),
          not(authorityObservationMatchesA(firstOperation, {...obs, artifactAndCall: AgreementCallA({...firstRequest, now: Time1})})),
          not(authorityObservationMatchesA(firstOperation, {...obs, transactionTime: 100})),
          not(authorityObservationMatchesA(firstOperation, {...obs, coreProjection: NoCoreProjection})),
          not(authorityObservationMatchesA(firstOperation, {...obs, effectEvidence: EvidenceUnavailable})),
        }
      | _ => false
    }
  }
  run completeCoreProjectionBindingTest = match adaptAuthorityA(firstRequest, firstOperation, emptyAdapterPlan, PublicDisplay) {
    | AuthorityAdaptedA(obs) => match obs.coreProjection {
        | CoreProjected(result) => List(
            {...result, reductions: 0},
            {...result, warnings: List({code: "partial_payment", requested: IntValue(10), paid: IntValue(5)})},
            {...result, error: CoreErrorCode("contract_closed")},
            {...result, accepted: false},
            {...result, payments: List()},
            {...result, state: {...result.state, minimumTime: 1}}).foldl(true, (ok, mutant) => ok
              and not(authorityObservationMatchesA(firstOperation, {...obs, coreProjection: CoreProjected(mutant)})))
        | _ => false
      }
    | _ => false
  }
  run actualTwoPaymentOrderBindingTest = {
    val request: AAuthorityRequest = {before: {program: canonicalSwap,
      state: {...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10).put(bobB, 20), minimumTime: Time2}},
      input: PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: 1})), now: Time2}
    match adaptAuthorityA(request, OpSettle, emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => match obs.coreProjection {
          | CoreProjected(result) => result.payments == List(
              {source: aliceA, recipient: Bob, asset: TokenA, quantity: 10},
              {source: bobB, recipient: Alice, asset: TokenB, quantity: 20})
              and authorityObservationMatchesA(OpSettle, obs)
              and not(authorityObservationMatchesA(OpSettle, {...obs, coreProjection: CoreProjected(
                {...result, payments: List(result.payments.nth(1), result.payments.nth(0))})}))
              and not(authorityObservationMatchesA(OpSettle, {...obs,
                effects: List(obs.effects.nth(1), obs.effects.nth(0))}))
          | _ => false
        }
      | _ => false
    }
  }
  run cancellationBindingAndInventedCoreControlsTest = {
    val request: AAuthorityRequest = {...firstRequest, input: NoAInput}
    match adaptAuthorityA(request, OpCancelParent(parentKeyA), emptyAdapterPlan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => all {
          obs.predecessor == request.before, obs.proposedSuccessor == request.before,
          obs.artifactAndCall == CancellationCallA({before: request.before, now: Time2}),
          obs.input == NoInput, obs.outcome == Cancellation, obs.effects == List(),
          obs.coreProjection == NoCoreProjection, obs.effectEvidence == EvidenceValid,
          authorityObservationMatchesA(OpCancelParent(parentKeyA), obs),
          not(authorityObservationMatchesA(OpCancelParent(parentKeyA), {...obs, effects: List(refund10)})),
          not(authorityObservationMatchesA(OpCancelParent(parentKeyA), {...obs, coreProjection: CoreProjected(
            {accepted: true, error: NoCoreError, state: expectedInitialProjection,
             payments: List(), warnings: List(), reductions: 0})})),
          not(authorityObservationMatchesA(OpCancelParent(parentKeyA), {...obs, artifactAndCall: AgreementCallA(request)})),
          not(authorityObservationMatchesA(firstOperation, obs)),
        }
      | _ => false
    }
  }
  run unvalidatedPlanAndDisplayCarriedVerbatimTest = {
    val suppliedPlan: AResolvedPlan = {identity: RecoveryPlanA, operations: List()}
    match adaptAuthorityA(firstRequest, firstOperation, suppliedPlan, AlternateDisplay) {
      | AuthorityAdaptedA(obs) => obs.resolvedPlan == suppliedPlan and obs.display == AlternateDisplay
          and authorityObservationMatchesA(firstOperation, obs)
      | _ => false
    }
  }
}
