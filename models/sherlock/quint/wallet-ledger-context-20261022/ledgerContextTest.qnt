module ledgerContextTest {
  import ledgerContext.* from "./ledgerContext"
  run fundedWalletBlankCounterexampleTest=initFundedBlank.then(step)
    .expect(state.cfg.chainFunded and state.available==0 and state.result==ProjectedFeeRefusal and not(naiveFundingClaim))
  run authenticEligibleProjectedSuccessTest=initWith(eligible).then(4.reps(i=>step))
    .expect(state.available==100 and state.result==ProjectedPass and state.readyForReview
      and state.checks==2 and state.spends==0 and not(state.registered) and not(state.finalized))
  run missingExactInputMetadataRefusesTest=initWith(eligible).expect(
    Set({...eligible,metadataPresent:false},{...eligible,metadataKey:6}).forall(c=>{
      val checked=checkSnapshot(initial(c),FirstChecked)
      checked.available==0 and checked.result==ProjectedFeeRefusal }))
  run eligibilityAndAgeCannotBeInventedTest=initWith(eligible).expect(
    Set({...eligible,matchingOwner:false},{...eligible,isNight:false},{...eligible,generationless:false},
      {...eligible,created:10},{...eligible,created:11}).forall(c=>{
      val checked=checkSnapshot(initial(c),FirstChecked)
      checked.available==0 and checked.result==ProjectedFeeRefusal }))
  run expiredClockOrInvalidSignatureRefusesTest=initWith(eligible).expect(and {
    projectedCheck(initial({...eligible,now:21}))==ProjectedClockRefusal,
    projectedCheck(initial({...eligible,now:111,ttl:200}))==ProjectedClockRefusal,
    projectedCheck(initial({...eligible,now:9}))==ProjectedClockRefusal,
    projectedCheck(initial({...eligible,signatureValid:false}))==ProjectedSignatureRefusal })
  run zeroAllowanceCannotEstablishRegistrationTest=initWith({...eligible,context:Blank,allowance:0})
    .then(4.reps(i=>step)).expect(state.result==ProjectedPass and not(state.readyForReview)
      and state.spends==0 and not(state.registered) and not(state.finalized))
  run fabricateDisableOrStaleClockIsNotFixTest=initWith(eligible).expect(
    Set({...eligible,context:FabricatedCopy},{...eligible,flags:{...strict,balancing:false},context:Blank},
      {...eligible,flags:{...strict,signatures:false},signatureValid:false},
      {...eligible,staleClock:true,now:21}).forall(c=>{
      val checked=checkSnapshot(initial(c),Rechecked)
      checked.result==ProjectedPass and not(furtherReviewCandidate(checked)) }))
  run stateChangeInvalidatesAndRevalidatesTest=initWith({...eligible,change:InputSpent})
    .then(step).expect(state.result==ProjectedPass and state.checks==1)
    .then(step).expect(state.certEpoch!=state.epoch and not(furtherReviewCandidate(state)))
    .then(step).expect(state.result==ProjectedFeeRefusal and state.checks==2)
    .then(step).expect(not(state.readyForReview) and state.spends==0)
  run timeAndEpochChangeNeedFreshCertificateTest=initWith(eligible).expect(and {
    not(furtherReviewCandidate(changeSnapshot(checkSnapshot(initial({...eligible,change:EpochAdvanced}),FirstChecked)))),
    checkSnapshot(changeSnapshot(checkSnapshot(initial({...eligible,change:ClockAdvanced}),FirstChecked)),Rechecked).result==ProjectedClockRefusal })
  run financialGuardAndU128CapRemainRequiredTest=initWith(eligible).expect(and {
    not(furtherReviewCandidate(checkSnapshot(initial({...eligible,financialGuard:false}),Rechecked))),
    availability(initial({...eligible,value:u128max,ratio:2,decay:2,ctime:2}))==u128max,
    availability(initial({...eligible,allowance:101}))==100,
    projectedCheck(initial({...eligible,allowance:101}))==ProjectedFeeRefusal })
}
