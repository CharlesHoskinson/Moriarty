module authorization_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import authorization_harness.* from "./authorization_harness"

  run prepareAfterResolveTest = initAfter.then(prepareAlice).expect(aliceChecked)
  run prepareBeforeResolveTest = assert(canPrepareSigning(initialSigning, aliceBefore, Alice))
  run wrongSignerCannotCheckTest = assert(not(canPrepareSigning(initialSigning, aliceAfter, Mallory)))
  run cannotSignWithoutCheckTest = assert(not(canSign(initialSigning, aliceAfter, Alice, 0)))
  run afterResolveSigningTest = initAfter.then(prepareAlice).then(signAlice).expect(aliceSigned)
  run beforeResolveSigningTest = initBefore.then(prepareAlice).then(signAlice).expect(aliceSigned)
  run independentConcurrentChecksTest = initAfter.then(prepareAlice).then(prepareBob).then(signAlice).then(signBob).expect(bothSignedAfter)
  run reverseConcurrentChecksTest = initBefore.then(prepareBob).then(prepareAlice).then(signBob).then(signAlice).expect(bothSignedBefore)

  pure val alicePrepared = applyPrepareSigning(initialSigning, aliceAfter, Alice)
  pure val aliceRegistered = applySign(alicePrepared, aliceAfter, Alice, 0)
  pure val parentSigningInitial: FixtureSigningState = {...initialSigning,
    context: {...emptyContext, candidate: "unfilled", ledger: installmentFacts.ledger}}
  pure val parentPrepared = applyPrepareSigning(parentSigningInitial, parentBefore, Alice)
  pure val parentRegistered = applySign(parentPrepared, parentBefore, Alice, 0)
  pure val parentSigned = {policy: parentBefore, signer: Alice, token: 0}
  pure val liveParent = fixtureParent(parentSigned)
  pure val cancelledEntry = applyCancelParent(liveParent, initialEntry(liveParent))
  pure val cancelledContext = {...parentRegistered.context,
    registry: parentRegistered.context.registry.put(parentKey, AuthorityConsumed({signed: parentSigned, revision: 1})),
    parents: parentRegistered.context.parents.put(parentKey, ParentLive({parent: liveParent, entry: cancelledEntry}))}

  run wrongSignerCannotSignTest = assert(not(canSign(alicePrepared, aliceAfter, Mallory, 0)))
  run changedBodyCannotSignTest = assert(not(canSign(alicePrepared,
    {...aliceAfter, body: {...aliceBody, disclosures: Set("changed")}}, Alice, 0)))
  run changedPlanCannotSignTest = assert(not(canSign(alicePrepared,
    {...aliceAfter, binding: AfterResolution({identity: "changed-plan", operations: List(settleOperation)})}, Alice, 0)))
  run changedProfileCannotSignTest = assert(not(canSign(alicePrepared, aliceBefore, Alice, 0)))
  run duplicateSignatureRejectedTest = assert(not(canSign(aliceRegistered, aliceAfter, Alice, 0)))
  run registeredPolicyCannotBeReplacedTest = assert(not(canPrepareSigning(aliceRegistered, aliceBefore, Alice)))
  run invalidSignatureTokenRejectedTest = assert(not(canSign(alicePrepared, aliceAfter, Alice, 2)))
  run candidateChangeInvalidatesCheckTest = assert(not(canSign({...alicePrepared,
    context: {...emptyContext, candidate: "different-state"}}, aliceAfter, Alice, 0)))
  run ledgerChangeInvalidatesCheckTest = assert(not(canSign({...alicePrepared,
    context: {...emptyContext, ledger: fundedLedger.put((Wallet(Mallory), TokenA), 1)}}, aliceAfter, Alice, 0)))
  run clockChangeInvalidatesCheckTest = assert(not(canSign({...alicePrepared,
    context: {...emptyContext, environment: {...environment, physicalTime: 100}}}, aliceAfter, Alice, 0)))
  run anchorChangeInvalidatesCheckTest = assert(not(canSign({...alicePrepared,
    context: {...emptyContext, environment: {...environment, anchor: 1}}}, aliceAfter, Alice, 0)))
  run versionChangeInvalidatesCheckTest = assert(not(canSign({...alicePrepared,
    context: {...emptyContext, environment: {...environment, implementationVersion: 1}}}, aliceAfter, Alice, 0)))
  run mechanismChangeInvalidatesCheckTest = assert(not(canSign({...alicePrepared,
    context: {...emptyContext, environment: {...environment, enforcementMechanism: 1}}}, aliceAfter, Alice, 0)))
  run noRepeatedIdenticalCheckActionTest = assert(not(canPrepareSigning(alicePrepared, aliceAfter, Alice)))
  run staleCheckCanBeRefreshedTest = assert(canPrepareSigning({...alicePrepared,
    context: {...emptyContext, environment: {...environment, anchor: 1}}}, aliceAfter, Alice))
  run mismatchedMechanismCannotPrepareTest = assert(not(canPrepareSigning(initialSigning,
    {...aliceBefore, body: {...aliceBody, enforcementMechanism: 1}}, Alice)))
  run missingRegistryKeyRejectedTest = assert(not(canPrepareSigning({...initialSigning,
    context: {...emptyContext, registry: Map()}}, aliceBefore, Alice)))
  run missingSigningKeyRejectedTest = assert(not(canSign({...alicePrepared, signing: Map()}, aliceAfter, Alice, 0)))
  run foreignDependencyKeyRejectedTest = assert(not(canPrepareSigning(initialSigning,
    {...aliceBefore, body: {...aliceBody, clauses: List({...settleClause,
      conditions: List(ParentMatches({key: {...parentKey, nonce: 99}, expected: ParentAbsent}))})}}, Alice)))
  run parentPreparationTest = assert(canPrepareSigning(parentSigningInitial, parentBefore, Alice))
  run parentSigningTest = assert(canSign(parentPrepared, parentBefore, Alice, 0))
  run parentRegistrationInitializesExactTermsTest = assert(validSigningState(parentRegistered)
    and parentRegistered.context.parents.get(parentKey) == ParentLive({parent: liveParent, entry: initialEntry(liveParent)})
    and parentRegistered.context.ledger == parentSigningInitial.context.ledger)
  run parentRegistrationDoesNotConsumeNonceTest = assert(parentRegistered.context.registry.get(parentKey) == AuthorityRegistered(parentSigned))
  run authenticPolicyFactsProjectionTest = assert(contextPolicyFacts(parentRegistered.context) == installmentFacts)
  run cancellationAccountingContextValidTest = assert(validAuthorityContext(cancelledContext))
  run recoveryCanBeCheckedAfterCancelTest = assert(canPrepareSigning({...parentRegistered, context: cancelledContext}, recoveryBefore, Alice))
  run recoveryCannotSignStaleParentCheckTest = {
    val checked = applyPrepareSigning(parentRegistered, recoveryBefore, Alice)
    assert(not(canSign({...checked, context: cancelledContext}, recoveryBefore, Alice, 0)))
  }
  run recoveryCanSignFreshParentCheckTest = {
    val checked = applyPrepareSigning({...parentRegistered, context: cancelledContext}, recoveryBefore, Alice)
    assert(canSign(checked, recoveryBefore, Alice, 0))
  }
  run parentSignatureSubstitutionInvalidatesCheckTest = {
    val checked = applyPrepareSigning(parentRegistered, recoveryBefore, Alice)
    val differentSigned = {...parentSigned, token: 1}
    val differentParent = fixtureParent(differentSigned)
    val changed = {...parentRegistered.context,
      registry: parentRegistered.context.registry.put(parentKey, AuthorityRegistered(differentSigned)),
      parents: parentRegistered.context.parents.put(parentKey,
        ParentLive({parent: differentParent, entry: initialEntry(differentParent)}))}
    assert(validAuthorityContext(changed) and not(canSign({...checked, context: changed}, recoveryBefore, Alice, 0)))
  }
  run parentEntryRevisionMustMatchAuthorityTest = assert(not(validAuthorityContext({...cancelledContext,
    registry: cancelledContext.registry.put(parentKey, AuthorityConsumed({signed: parentSigned, revision: 2}))})))
  run parentEntryCannotSurviveUnusedAuthorityTest = assert(not(validAuthorityContext({...parentRegistered.context,
    registry: parentRegistered.context.registry.put(parentKey, AuthorityUnused)})))
  run parentAuthorityCannotLoseEntryTest = assert(not(validAuthorityContext({...parentRegistered.context,
    parents: parentRegistered.context.parents.put(parentKey, ParentVacant)})))
  run unsupportedParentPreparationTest = assert(Set(Bob, Mallory).forall(principal => {
    val key = {...parentKey, principal: principal}
    val unsupported: TestPolicy = {...parentBefore, body: {...parentBody, key: key, debitLocations: Set(),
      capabilities: Set(CancelCapability), clauses: List({...cancelInitialClause,
        operation: OpCancelParent(key), conditions: List(InputIs(NoInput))})}}
    validPolicy(unsupported) and not(canPrepareSigning(parentSigningInitial, unsupported, principal))
  }))
}
