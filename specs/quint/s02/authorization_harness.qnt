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
  action init = signingState' = initialSigning
  action prepareAlice = all {
    canPrepareSigning(signingState, aliceAfter, Alice),
    signingState' = applyPrepareSigning(signingState, aliceAfter, Alice),
  }
  action step = prepareAlice
  val aliceChecked = match signingState.signing.get(aliceKey) {
    | PreparedSigning(_) => true
    | _ => false
  }
  val noMoneyChanged = signingState.context.ledger == initialSigning.context.ledger
}
