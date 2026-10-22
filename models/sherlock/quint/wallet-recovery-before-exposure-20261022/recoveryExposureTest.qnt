module recoveryExposureTest {
  import recoveryExposure.* from "./recoveryExposure"
  run oldOrderExposesBeforeRecoveryTest=initSource28.then(2.reps(i=>advance))
    .expect(state.financialPassed and state.signedBytesExposurePossible and state.thirdPartyRelayPossible
      and state.modeledExplicitAttempts==0 and not(state.ownedUncertainty) and not(exposureSafety))
  run correctedExposureHasDurableBoundRecoveryTest=initWith(corrected).then(5.reps(i=>advance))
    .expect(state.signedBytesExposurePossible and state.modeledExplicitAttempts==0 and exposureSafety and correctedSafety)
    .then(2.reps(i=>advance)).expect(state.phase==Finished and state.modeledExplicitAttempts==1 and not(state.finalized))
  run fullFinancialGuardBlocksBothOrdersTest=initWith(corrected).expect(
    Set(Source28ExposureFirst,RequiredRecoveryFirst).forall(r=>{
      val stopped=nextState(initial({...corrected,route:r,fault:FinancialReject}))
      stopped.phase==RefusedBefore and not(stopped.recordsWritten) and not(stopped.signedBytesExposurePossible) }))
  run failedWriteOrFsyncCannotExposeCorrectedTest=initWith(corrected).expect(
    Set(RecordWriteFailure,FsyncFailure).forall(f=>{
      val written=nextState(nextState(initial({...corrected,fault:f})))
      val stopped=if(written.phase==RefusedBefore) written else nextState(written)
      stopped.phase==RefusedBefore and not(stopped.recoveryDurable)
        and not(stopped.signedBytesExposurePossible) and not(stopped.ownedUncertainty)
        and not(stopped.ownSubmitMayHaveStarted) and stopped.modeledExplicitAttempts==0 }))
  run nodeRefusalAndTimeoutKeepProtectedRecoveryTest=initWith(corrected).expect(
    Set(NodeInvalid,NodeUnknown,NodeTimeout).forall(f=>{
      val start=initial({...corrected,fault:f})
      val a=nextState(nextState(nextState(start)))
      val stopped=nextState(nextState(nextState(a)))
      stopped.phase==RefusedAfter and stopped.recoveryDurable and stopped.boundPreparedIdentity
        and stopped.ownedUncertainty and stopped.thirdPartyRelayPossible and stopped.modeledExplicitAttempts==0 }))
  run localRevertCannotRevokeExposedAuthorityTest=initWith({...corrected,fault:NodeTimeout})
    .then(6.reps(i=>advance)).then(revertBookkeeping)
    .expect(state.localBookkeepingReverted and state.recoveryDurable and state.boundPreparedIdentity
      and state.ownedUncertainty and state.thirdPartyRelayPossible and state.modeledExplicitAttempts==0)
  run crashAfterExposureLeavesDurableRecoveryTest=initWith({...corrected,allowCrash:true})
    .then(5.reps(i=>advance)).then(crash)
    .expect(state.phase==Crashed and state.recoveryDurable and state.boundPreparedIdentity
      and state.ownedUncertainty and state.thirdPartyRelayPossible and state.modeledExplicitAttempts==0)
    .then(advance.fail())
  run ambiguousOwnSubmitDoesNotRetryTest=initWith({...corrected,fault:OwnDispatchAmbiguous})
    .then(7.reps(i=>advance)).expect(state.phase==Ambiguous and state.modeledExplicitAttempts==1
      and state.ownsExplicitSubmit and exposureSafety).then(advance.fail())
  run possibleOwnCallIsNotActualOwnCallCountTest=initWith({...corrected,fault:PostValidationFailure})
    .then(6.reps(i=>advance)).expect(state.ownSubmitMayHaveStarted and state.modeledExplicitAttempts==0)
    .then(advance).expect(state.phase==RefusedAfter and state.ownSubmitMayHaveStarted and state.modeledExplicitAttempts==0)
  run expiryDoesNotEraseExposureHistoryTest=initWith({...corrected,fault:NodeUnknown})
    .then(6.reps(i=>advance)).then(authorityExpires)
    .expect(state.signedBytesExposurePossible and not(state.thirdPartyRelayPossible) and state.recoveryDurable
      and state.boundPreparedIdentity and state.ownedUncertainty and not(state.registered))
}
