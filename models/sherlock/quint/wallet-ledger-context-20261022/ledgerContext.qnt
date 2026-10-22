module ledgerContext {
  // PUBLIC_SYNTHETIC diagnostic. No wallet/SDK/ledger/RPC is called or authorized.
  type Context = Blank | AuthenticSnapshot | FabricatedCopy
  type Change = Stable | InputSpent | ClockAdvanced | EpochAdvanced
  type Phase = Prepare | FirstChecked | ContextChanged | Rechecked | Finished
  type Result = Unchecked | ProjectedPass | ProjectedFeeRefusal | ProjectedClockRefusal | ProjectedSignatureRefusal
  type Flags = {balancing:bool,signatures:bool,limits:bool}
  type Input = {context:Context,metadataPresent:bool,metadataKey:int,inputKey:int,
    value:int,ratio:int,decay:int,created:int,ctime:int,grace:int,ttl:int,now:int,
    allowance:int,matchingOwner:bool,isNight:bool,generationless:bool,chainFunded:bool,
    signatureValid:bool,financialGuard:bool,flags:Flags,staleClock:bool,change:Change}
  type State = {cfg:Input,phase:Phase,inputPresent:bool,epoch:int,now:int,result:Result,
    available:int,certEpoch:int,certTime:int,checks:int,readyForReview:bool,
    spends:int,registered:bool,finalized:bool}
  pure val strict:Flags={balancing:true,signatures:true,limits:true}
  pure val eligible:Input={context:AuthenticSnapshot,metadataPresent:true,metadataKey:5,inputKey:5,
    value:5,ratio:100,decay:2,created:0,ctime:10,grace:100,ttl:20,now:10,allowance:4,
    matchingOwner:true,isNight:true,generationless:true,chainFunded:true,signatureValid:true,
    financialGuard:true,flags:strict,staleClock:false,change:Stable}
  pure val u128max:int=340282366920938463463374607431768211455
  pure def sat(x:int):int=if(x>u128max) u128max else x
  pure def smaller(a:int,b:int):int=if(a<b) a else b
  pure def initial(c:Input):State={cfg:c,phase:Prepare,inputPresent:c.metadataPresent,
    epoch:0,now:c.now,result:Unchecked,available:0,certEpoch:-1,certTime:-1,checks:0,
    readyForReview:false,spends:0,registered:false,finalized:false}
  pure def availability(s:State):int={
    val c=s.cfg
    if(c.context==Blank or not(s.inputPresent) or c.metadataKey!=c.inputKey
      or not(c.matchingOwner and c.isNight and c.generationless)) 0
    else {
      val age=if(c.ctime<c.created) 0 else c.ctime-c.created
      smaller(sat(c.value*c.ratio),sat(age*sat(c.value*c.decay))) } }
  pure def validationClock(s:State):int=if(s.cfg.staleClock) s.cfg.ctime else s.now
  // Projected checks only. Other transaction checks/order remain external.
  pure def projectedCheck(s:State):Result={
    val c=s.cfg
    val clockNow=validationClock(s)
    if(c.flags.signatures and not(c.signatureValid)) ProjectedSignatureRefusal
    else if(clockNow>c.ttl or clockNow<c.ctime or clockNow>c.ctime+c.grace) ProjectedClockRefusal
    else if(c.flags.balancing and c.allowance>0 and availability(s)<c.allowance) ProjectedFeeRefusal
    else ProjectedPass }
  pure def checkSnapshot(s:State,nextPhase:Phase):State={...s,phase:nextPhase,
    result:projectedCheck(s),available:availability(s),certEpoch:s.epoch,
    certTime:validationClock(s),checks:s.checks+1,readyForReview:false}
  pure def changeSnapshot(s:State):State=match s.cfg.change {
    | Stable=>{...s,phase:ContextChanged,readyForReview:false}
    | InputSpent=>{...s,phase:ContextChanged,epoch:s.epoch+1,inputPresent:false,readyForReview:false}
    | ClockAdvanced=>{...s,phase:ContextChanged,epoch:s.epoch+1,now:s.cfg.ttl+1,readyForReview:false}
    | EpochAdvanced=>{...s,phase:ContextChanged,epoch:s.epoch+1,readyForReview:false} }
  pure def furtherReviewCandidate(s:State):bool=and {
    s.result==ProjectedPass,s.cfg.context==AuthenticSnapshot,s.cfg.flags==strict,
    s.cfg.allowance>0,s.cfg.financialGuard,s.cfg.signatureValid,
    s.inputPresent,s.cfg.metadataKey==s.cfg.inputKey,s.cfg.matchingOwner,
    s.cfg.isNight,s.cfg.generationless,s.certEpoch==s.epoch,s.certTime==s.now,
    s.available>=s.cfg.allowance,s.now<=s.cfg.ttl,s.now>=s.cfg.ctime,s.now<=s.cfg.ctime+s.cfg.grace }
  pure def nextState(s:State):State=match s.phase {
    | Prepare=>checkSnapshot(s,FirstChecked)
    | FirstChecked=>changeSnapshot(s)
    | ContextChanged=>checkSnapshot(s,Rechecked)
    | Rechecked=>{...s,phase:Finished,readyForReview:furtherReviewCandidate(s)}
    | _=>List().head() }
  var state:State
  action initWith(c:Input):bool=state'=initial(c)
  action initFundedBlank:bool=initWith({...eligible,context:Blank})
  action init:bool={
    nondet contextArg=Set(Blank,AuthenticSnapshot,FabricatedCopy).oneOf()
    nondet presentArg=Set(false,true).oneOf()
    nondet allowanceArg=Set(0,4,101).oneOf()
    nondet createdArg=Set(0,10,11).oneOf()
    nondet sigArg=Set(false,true).oneOf()
    nondet guardArg=Set(false,true).oneOf()
    nondet changeArg=Set(Stable,InputSpent,ClockAdvanced,EpochAdvanced).oneOf()
    initWith({...eligible,context:contextArg,metadataPresent:presentArg,allowance:allowanceArg,
      created:createdArg,signatureValid:sigArg,financialGuard:guardArg,change:changeArg}) }
  action step:bool=all {state.phase!=Finished,state'=nextState(state)}
  val noFalseAuthority:bool=and {
    state.spends==0,not(state.registered),not(state.finalized),state.cfg.flags==strict,
    state.readyForReview implies and {state.phase==Finished,state.checks==2,furtherReviewCandidate(state)},
    (state.cfg.context==Blank and state.cfg.allowance>0 and state.result==ProjectedPass) implies false,
    (state.cfg.context!=AuthenticSnapshot or state.certEpoch!=state.epoch or state.certTime!=state.now)
      implies not(state.readyForReview) }
  val fundedBlankRefusal:bool=state.phase==FirstChecked and state.cfg.chainFunded and state.cfg.context==Blank
    and state.cfg.allowance>0 and state.result==ProjectedFeeRefusal
  val authenticEligible:bool=state.phase==Finished and state.readyForReview
  val missingInputRefusal:bool=state.phase==FirstChecked and state.cfg.context==AuthenticSnapshot
    and not(state.inputPresent) and state.result==ProjectedFeeRefusal
  val revalidationAfterSpend:bool=state.phase==Finished and state.cfg.change==InputSpent
    and state.checks==2 and state.result==ProjectedFeeRefusal
  val zeroAllowanceNotRegistration:bool=state.phase==Finished and state.cfg.allowance==0
    and state.result==ProjectedPass and not(state.readyForReview) and not(state.registered)
  val naiveFundingClaim:bool=state.cfg.chainFunded and state.cfg.allowance>0 and state.checks>0
    implies state.result==ProjectedPass
}
