module ownedNodeValidationTest {
  import ownedNodeValidation.* from "./ownedNodeValidation"
  pure def replyAndClose(s:State,i:int):State=closed(responded(s,i),i)
  pure def contextReady(s:State):State=advanced(replyAndClose(s,0))
  pure def admissionReady(s:State):State=advanced(replyAndClose(replyAndClose(contextReady(s),1),2))
  pure def decisionReady(s:State):State=advanced(replyAndClose(admissionReady(s),3))
  run successfulOwnedLifecycleTest=initWith(good)
    .then(response(0)).then(closeEvent(0)).then(advance)
    .then(response(1)).then(response(2)).then(closeEvent(2)).then(closeEvent(1)).then(advance)
    .then(response(3)).then(closeEvent(3)).then(advance).then(advance)
    .expect(state.returnedSuccess and allClosed(state) and not(state.busy) and state.copyWiped and ownershipAndDeadline)
  run responseCannotSettleBeforeOwnCloseTest=initWith(good).then(response(0))
    .expect(state.requests.get(0).socket==Closing and not(state.requests.get(0).settled) and not(canAdvance(state)))
    .then(advance.fail())
  run parallelFailureClosesSiblingBeforeReturnTest=initWith(good).expect({
    val contextState=contextReady(initial({...good,fault:RuntimeFailure}))
    val cleaning=advanced(replyAndClose(contextState,1))
    val sibling=cleaning.requests.get(2)
    val returned=advanced(closed(cleaning,2))
    cleaning.phase==Cleanup and sibling.socket==Closing and sibling.terminated
      and not(sibling.settled) and not(canAdvance(cleaning))
      and returned.phase==Returned and not(returned.returnedSuccess) and allClosed(returned) })
  run timeoutCannotBeOverriddenByLateOkTest=initWith({...good,allowTimeout:true})
    .then(expire).expect(state.deadlineWon and state.requests.get(0).socket==Closing)
    .expect(not(canAdvance(state)) and state.requests.get(0).socket!=Open)
    .then(closeEvent(0)).then(advance)
    .expect(state.phase==Returned and not(state.returnedSuccess) and allClosed(state) and ownershipAndDeadline)
    .then(response(0).fail())
  run invalidUnknownMutationCleanupRefuseTest=initWith(good).expect(
    Set(InvalidResult,UnknownResult,MutatedTransaction,CleanupFailure).forall(f=>{
      val returned=advanced(decisionReady(initial({...good,fault:f})))
      returned.phase==Returned and not(returned.returnedSuccess) and allClosed(returned)
        and returned.copyWiped and not(returned.busy) }))
  run incompatibleContextDoesNotStartAdmissionTest=initWith(good).expect({
    val cleaning=admissionReady(initial({...good,fault:ContextMismatch}))
    val returned=advanced(cleaning)
    cleaning.phase==Cleanup and cleaning.requests.get(3).socket==Unused
      and returned.phase==Returned and not(returned.returnedSuccess) and allClosed(returned) })
  run bareCloseAndConstructorFailureRefuseTest=initWith(good).expect(and {
    not(advanced(advanced(closed(initial(good),0))).returnedSuccess),
    advanced(closed(errored(responded(initial(good),0),0),0)).phase==Cleanup,
    advanced(initial({...good,fault:ConstructorFailure})).phase==Returned,
    not(advanced(initial({...good,fault:ConstructorFailure})).returnedSuccess) })
  run sameHeadAndNoPersistentFactorySocketTest=initWith(good).expect({
    val admitted=admissionReady(initial(good))
    Set(1,2,3).forall(i=>admitted.requests.get(i).head==admitted.selectedHead)
      and not(admitted.factorySocket) and admitted.dispatches==0 })
  run cacheAcceptanceIsNotFreshStrictProofTest=initWith(good).expect({
    val returned=advanced(decisionReady(initial({...good,hashCacheHit:true})))
    returned.returnedSuccess and returned.cfg.hashCacheHit and not(returned.freshStrictCertificate)
      and not(returned.coinReserved) and returned.dispatches==0 })
  run deadlineAfterContextClosesAllBeforeReturnTest=initWith(good).expect({
    val timed=timedOut(contextReady(initial({...good,allowTimeout:true})))
    val oneClosed=closed(timed,1)
    val allAreClosed=closed(oneClosed,2)
    not(canAdvance(oneClosed)) and allClosed(allAreClosed)
      and not(advanced(allAreClosed).returnedSuccess) and advanced(allAreClosed).copyWiped })
}
