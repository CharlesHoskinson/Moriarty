module authorization_harness {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"

  type FixtureSigningState = SigningState[str,str,str,str]
  pure val emptyContext: AuthorityContext[str,str,str,str] = {candidate: "funded", ledger: fundedLedger,
    environment: environment, registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused),
    parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)}
  pure val initialSigning: FixtureSigningState = {context: emptyContext,
    signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)}
  var signingState: FixtureSigningState
  var selectedProfile: SigningProfile
  action initAfter = all {signingState' = initialSigning, selectedProfile' = SignAfterResolve}
  action initBefore = all {signingState' = initialSigning, selectedProfile' = SignBeforeResolve}
  action init = any {initAfter, initBefore}
  val activeAlice = if (selectedProfile == SignAfterResolve) aliceAfter else aliceBefore
  val activeBob = if (selectedProfile == SignAfterResolve) bobAfter else bobBefore
  action prepareAlice = all {
    canPrepareSigning(signingState, activeAlice, Alice),
    signingState' = applyPrepareSigning(signingState, activeAlice, Alice),
    selectedProfile' = selectedProfile,
  }
  action prepareBob = all {
    canPrepareSigning(signingState, activeBob, Bob),
    signingState' = applyPrepareSigning(signingState, activeBob, Bob),
    selectedProfile' = selectedProfile,
  }
  action signAlice = all {
    canSign(signingState, activeAlice, Alice, 0),
    signingState' = applySign(signingState, activeAlice, Alice, 0),
    selectedProfile' = selectedProfile,
  }
  action signBob = all {
    canSign(signingState, activeBob, Bob, 0),
    signingState' = applySign(signingState, activeBob, Bob, 0),
    selectedProfile' = selectedProfile,
  }
  action step = any {prepareAlice, prepareBob, signAlice, signBob}
  val aliceChecked = match signingState.signing.get(aliceKey) {
    | PreparedSigning(_) => true
    | _ => false
  }
  val bobChecked = match signingState.signing.get(bobKey) {
    | PreparedSigning(_) => true
    | _ => false
  }
  val aliceSigned = signingState.context.registry.get(aliceKey) == AuthorityRegistered({policy: activeAlice, signer: Alice, token: 0})
  val bobSigned = signingState.context.registry.get(bobKey) == AuthorityRegistered({policy: activeBob, signer: Bob, token: 0})
  // This is the end of the signing-only fixture, not financial terminality.
  val bothSignedAfter = selectedProfile == SignAfterResolve and aliceSigned and bobSigned
  val bothSignedBefore = selectedProfile == SignBeforeResolve and aliceSigned and bobSigned
  val signingFinished = aliceSigned and bobSigned
  val signingEnabled = canPrepareSigning(signingState, activeAlice, Alice)
    or canPrepareSigning(signingState, activeBob, Bob)
    or canSign(signingState, activeAlice, Alice, 0) or canSign(signingState, activeBob, Bob, 0)
  val noMoneyChanged = signingState.context.ledger == initialSigning.context.ledger
  val signingSafety = noMoneyChanged and validSigningState(signingState)
    and (signingFinished or signingEnabled) and (not(signingFinished) or not(signingEnabled))
}
