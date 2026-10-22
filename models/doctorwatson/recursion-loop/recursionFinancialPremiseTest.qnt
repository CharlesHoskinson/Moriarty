module recursionFinancialPremiseTest {
  import recursionFirstLoop.* from "./recursionFirstLoop"
  action verifiedParent:bool=passedLocal(ParentProof).then(dispatchVerifier(ParentVerify))
    .then(observeVerifier(ParentVerify,goodVerification(state)))
  action localChain:bool=acceptSource.then(verifiedParent).then(passedLocal(Build9))
    .then(recordCapture(CapturedCrossChecked,true,true,true)).then(passedLocal(DPrepare)).then(passedLocal(DProof))
    .then(dispatchVerifier(DVerify)).then(observeVerifier(DVerify,goodVerification(state)))
  run missingFinancialPremiseRefusesOtherwiseReadyTargetTest=initFinancialWith(true,ExactIntent,false)
    .then(localChain).then(reconcileOriginal(FullFinalizedExact))
    .expect(state.targetPermit and current(state,DVerify) and state.original==OriginalReconciled
      and state.newSends==0 and not(state.financialPremise) and not(canSubmitTarget(state,1))
      and financialBlockedWhenOtherwiseReady and loopInvariant and financialRefusalInvariant)
    .then(submitTarget(1).fail())
  run presentFinancialPremiseAllowsPairedTargetTest=initFinancialWith(true,ExactIntent,true)
    .then(localChain).then(reconcileOriginal(FullFinalizedExact))
    .expect(state.targetPermit and current(state,DVerify) and state.original==OriginalReconciled
      and state.newSends==0 and state.financialPremise and canSubmitTarget(state,1))
    .then(submitTarget(1)).expect(state.newSends==1 and state.originalSends==1 and loopInvariant)
}
