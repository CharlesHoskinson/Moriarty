module ownedNodeValidation {
  // PUBLIC_SYNTHETIC; no WebSocket, SDK, wallet or transaction data.
  // Deadline means the source timer callback fired, not a proven hard wallclock bound.
  // closeEvent abstracts observed close OR readyState===CLOSED, never CLOSING.
  // Entry follows flags/serialization/lazy-session setup; no socket exists at factory creation.
  type Phase = Head | Context | Admission | Cleanup | Returned
  type Socket = Unused | Open | Closing | Closed
  type Answer = NoAnswer | RpcSuccess | RpcFailure
  type Fault = Healthy | HeadFailure | RuntimeFailure | MetadataFailure | AdmissionFailure
    | InvalidResult | UnknownResult | ContextMismatch | MutatedTransaction | CleanupFailure | ConstructorFailure
  type Request = {socket:Socket,answer:Answer,settled:bool,terminated:bool,head:int,owner:int}
  type Config = {fault:Fault,allowTimeout:bool,hashCacheHit:bool}
  type State = {cfg:Config,phase:Phase,requests:int->Request,selectedHead:int,active:bool,
    deadlineWon:bool,decisionSuccess:bool,returnedSuccess:bool,busy:bool,copyWiped:bool,
    factorySocket:bool,dispatches:int,freshStrictCertificate:bool,coinReserved:bool}
  pure val ids:Set[int]=0.to(3)
  pure val good:Config={fault:Healthy,allowTimeout:false,hashCacheHit:false}
  pure val unused:Request={socket:Unused,answer:NoAnswer,settled:false,terminated:false,head:0,owner:1}
  pure def opened(headArg:int):Request={...unused,socket:Open,head:headArg}
  pure def initial(c:Config):State={cfg:c,phase:(if(c.fault==ConstructorFailure) Cleanup else Head),
    requests:ids.mapBy(i=>if(i==0 and c.fault!=ConstructorFailure) opened(0) else unused),selectedHead:7,
    active:c.fault!=ConstructorFailure,deadlineWon:false,decisionSuccess:false,returnedSuccess:false,
    busy:true,copyWiped:false,factorySocket:false,dispatches:0,freshStrictCertificate:false,coinReserved:false}
  pure def badRpc(s:State,i:int):bool=(i==0 and s.cfg.fault==HeadFailure)
    or(i==1 and s.cfg.fault==RuntimeFailure) or(i==2 and s.cfg.fault==MetadataFailure)
    or(i==3 and s.cfg.fault==AdmissionFailure)
  pure def responded(s:State,i:int):State={
    val r=s.requests.get(i)
    {...s,requests:s.requests.put(i,{...r,socket:Closing,terminated:true,
      answer:(if(badRpc(s,i)) RpcFailure else RpcSuccess)})} }
  pure def errored(s:State,i:int):State={
    val r=s.requests.get(i)
    {...s,requests:s.requests.put(i,{...r,socket:Closing,answer:RpcFailure,terminated:true})} }
  pure def closed(s:State,i:int):State={
    val r=s.requests.get(i)
    {...s,requests:s.requests.put(i,{...r,socket:Closed,settled:true,
      answer:(if(r.answer==NoAnswer) RpcFailure else r.answer)})} }
  pure def finishRequests(s:State,success:bool):State={...s,phase:Cleanup,active:false,
    decisionSuccess:success,requests:ids.mapBy(i=>{
      val r=s.requests.get(i)
      if(r.socket==Open) {...r,socket:Closing,terminated:true,answer:RpcFailure}
      else r })}
  pure def settledOk(s:State,i:int):bool=s.requests.get(i).settled and s.requests.get(i).answer==RpcSuccess
  pure def settledFailed(s:State,i:int):bool=s.requests.get(i).settled and s.requests.get(i).answer==RpcFailure
  pure def allClosed(s:State):bool=ids.forall(i=>{
    val r=s.requests.get(i)
    r.socket==Unused or(r.socket==Closed and r.settled) })
  pure def canAdvance(s:State):bool=match s.phase {
    | Head=>s.requests.get(0).settled
    | Context=>Set(1,2).exists(i=>settledFailed(s,i)) or Set(1,2).forall(i=>settledOk(s,i))
    | Admission=>s.requests.get(3).settled
    | Cleanup=>allClosed(s)
    | _=>false }
  pure def advanced(s:State):State=match s.phase {
    | Head=>if(settledFailed(s,0)) finishRequests(s,false)
      else {...s,phase:Context,requests:s.requests.put(1,opened(s.selectedHead)).put(2,opened(s.selectedHead))}
    | Context=>if(Set(1,2).exists(i=>settledFailed(s,i)) or s.cfg.fault==ContextMismatch) finishRequests(s,false)
      else {...s,phase:Admission,requests:s.requests.put(3,opened(s.selectedHead))}
    | Admission=>finishRequests(s,settledOk(s,3) and not(Set(InvalidResult,UnknownResult,MutatedTransaction).contains(s.cfg.fault)))
    | Cleanup=>{...s,phase:Returned,busy:false,copyWiped:true,
      returnedSuccess:s.decisionSuccess and s.cfg.fault!=CleanupFailure}
    | _=>List().head() }
  pure def timedOut(s:State):State=finishRequests({...s,deadlineWon:true},false)
  var state:State
  action initWith(c:Config):bool=state'=initial(c)
  action init:bool={
    nondet f=Set(Healthy,HeadFailure,RuntimeFailure,MetadataFailure,AdmissionFailure,
      InvalidResult,UnknownResult,ContextMismatch,MutatedTransaction,CleanupFailure,ConstructorFailure).oneOf()
    nondet timeoutArg=Set(true,false).oneOf()
    nondet cacheArg=Set(true,false).oneOf()
    initWith({fault:f,allowTimeout:timeoutArg,hashCacheHit:cacheArg}) }
  action response(i:int):bool=all {state.requests.get(i).socket==Open,state'=responded(state,i)}
  action transportError(i:int):bool=all {Set(Open,Closing).contains(state.requests.get(i).socket),
    state.requests.get(i).answer!=RpcFailure,state'=errored(state,i)}
  action closeEvent(i:int):bool=all {Set(Open,Closing).contains(state.requests.get(i).socket),state'=closed(state,i)}
  action advance:bool=all {canAdvance(state),state'=advanced(state)}
  action expire:bool=all {state.active,state.cfg.allowTimeout,state'=timedOut(state)}
  action step:bool={nondet i=ids.oneOf();any {response(i),transportError(i),closeEvent(i),advance,expire}}
  val ownershipAndDeadline:bool=and {
    not(state.factorySocket),state.dispatches==0,not(state.freshStrictCertificate),not(state.coinReserved),
    ids.forall(i=>{
      val r=state.requests.get(i)
      r.owner==1 and(r.settled implies r.socket==Closed)
        and(i>0 and r.socket!=Unused implies r.head==state.selectedHead) }),
    state.phase==Returned implies allClosed(state) and not(state.busy) and state.copyWiped,
    state.returnedSuccess implies and {state.phase==Returned,state.decisionSuccess,not(state.deadlineWon),
      Set(0,1,2,3).forall(i=>settledOk(state,i)),not(Set(InvalidResult,UnknownResult,ContextMismatch,MutatedTransaction,CleanupFailure,ConstructorFailure).contains(state.cfg.fault))},
    state.deadlineWon implies not(state.decisionSuccess) and not(state.returnedSuccess) }
  val successfulClosedReturn:bool=state.phase==Returned and state.returnedSuccess
  val refusedClosedReturn:bool=state.phase==Returned and not(state.returnedSuccess)
  val responseAwaitingClose:bool=ids.exists(i=>state.requests.get(i).socket==Closing
    and state.requests.get(i).answer==RpcSuccess and not(state.requests.get(i).settled))
  val timeoutClosedReturn:bool=state.phase==Returned and state.deadlineWon and allClosed(state)
  val partialContextCleanup:bool=state.phase==Cleanup and Set(1,2).exists(i=>state.requests.get(i).socket==Closing)
  val cacheResultIsLimited:bool=state.returnedSuccess and state.cfg.hashCacheHit
    and not(state.freshStrictCertificate) and not(state.coinReserved)
}
