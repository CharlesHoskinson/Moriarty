// Finite consumer/evidence dependency projection, not a prover, wallet or factory.
module recursionFirstLoop {
  import doctorwatson_readiness.capability from "../doctorwatson_readiness"
  import doctorwatson_readiness.HandkeyVerify from "../doctorwatson_readiness"
  import doctorwatson_delivery_flow.Milestone from "../doctorwatson_delivery_flow"
  import doctorwatson_delivery_flow.RecursiveProof from "../doctorwatson_delivery_flow"
  import doctorwatson_delivery_flow.StagenetReadback from "../doctorwatson_delivery_flow"
  import doctorwatson_delivery_flow.DegreeAndBenchmark from "../doctorwatson_delivery_flow"
  type Node = ParentProof | ParentVerify | Build9 | MP2 | DPrepare | DProof | DVerify | TargetReadback | SyntheticImport | Benchmark
  type Status = Pending | Passed | Failed | Unknown
  type Profile = ExactIntent | FixedS0 | Native16
  type Original = OriginalUnknown | OriginalReconciled
  type ReadResult = Absent | ReadError | WrongIdentity | FullFinalizedExact
  type Verification = {actual:bool,independent:bool,routeOne:bool,routeTwo:bool,
    tamperRejected:bool,full28:bool,tupleMatches:bool,epoch:int,profile:Profile}
  type CaptureQuality = Provisional | CapturedUnchecked | CapturedCrossChecked
  type Evidence = {status:Status,epoch:int,profile:Profile}
  type TargetReceipt = {actual:bool,canonical:bool,finalized:bool,exactBytes:bool,
    identityMatched:bool,fullEffects:bool,feesMatched:bool,epoch:int,profile:Profile}
  type State = {epoch:int,profile:Profile,historyPresent:bool,sourceAccepted:bool,sourceEpoch:int,
    parentVerification:Verification,dVerification:Verification,captureQuality:CaptureQuality,compiledPIsEqual:bool,
    evidence:Node->Evidence,verifierDispatches:Set[Node],original:Original,originalReads:int,
    originalSends:int,newSends:int,targetPermit:bool,financialPremise:bool,
    milestones:Set[Milestone],milestoneEpoch:int,readbackObserved:bool}
  pure val NODES:Set[Node]=Set(ParentProof,ParentVerify,Build9,MP2,DPrepare,DProof,DVerify,TargetReadback,SyntheticImport,Benchmark)
  pure val LOCAL:Set[Node]=NODES.exclude(Set(TargetReadback))
  pure val VERIFIERS:Set[Node]=Set(ParentVerify,DVerify)
  pure def emptyVerification(p:Profile):Verification={actual:false,independent:false,routeOne:false,
    routeTwo:false,tamperRejected:false,full28:false,tupleMatches:false,epoch:-1,profile:p}
  pure def initial(permit:bool,p:Profile):State={epoch:0,profile:p,historyPresent:true,sourceAccepted:false,sourceEpoch:-1,
    parentVerification:emptyVerification(p),dVerification:emptyVerification(p),captureQuality:Provisional,compiledPIsEqual:false,
    evidence:NODES.mapBy(n=>{status:Pending,epoch:-1,profile:p}),verifierDispatches:Set(),
    original:OriginalUnknown,originalReads:0,originalSends:1,newSends:0,targetPermit:permit,
    financialPremise:true,milestones:Set(),milestoneEpoch:-1,readbackObserved:false}
  pure def current(s:State,n:Node):bool={val e=s.evidence.get(n)
    e.status==Passed and e.epoch==s.epoch and e.profile==s.profile}
  pure def dependencies(s:State,n:Node):bool=match n {
    | ParentProof=>s.sourceAccepted and s.sourceEpoch==s.epoch | Build9=>true | SyntheticImport=>true
    | ParentVerify=>current(s,ParentProof)
    | MP2=>current(s,Build9) and current(s,ParentVerify)
    | DPrepare=>current(s,ParentVerify) and current(s,MP2)
    | DProof=>current(s,DPrepare) and current(s,ParentVerify) and current(s,MP2)
    | DVerify=>current(s,DProof)
    | TargetReadback=>current(s,DVerify) and s.newSends==1
    | Benchmark=>current(s,TargetReadback) and current(s,DVerify) and current(s,SyntheticImport) and s.profile==ExactIntent }
  pure def factoryRunnable(n:Node):bool={val c=capability(HandkeyVerify)
    VERIFIERS.contains(n) and c.executorAvailable and c.collectorAvailable}
  pure def canObserveLocal(s:State,n:Node,epochArg:int,p:Profile,actual:bool,independent:bool,tupleMatches:bool):bool=and {
    LOCAL.exclude(VERIFIERS).exclude(Set(MP2)).contains(n),s.evidence.get(n).status==Pending,dependencies(s,n),actual,tupleMatches,
    epochArg==s.epoch,p==s.profile,
    not(VERIFIERS.contains(n)) or (independent and s.verifierDispatches.contains(n)),
    n!=Benchmark or p==ExactIntent }
  pure def verificationQualifies(s:State,n:Node,r:Verification):bool=and {
    VERIFIERS.contains(n),r.actual,r.independent,r.routeOne,r.tamperRejected,r.full28,r.tupleMatches,
    n!=ParentVerify or r.routeTwo,r.epoch==s.epoch,r.profile==s.profile}
  pure def goodVerification(s:State):Verification={actual:true,independent:true,routeOne:true,routeTwo:true,
    tamperRejected:true,full28:true,tupleMatches:true,epoch:s.epoch,profile:s.profile}
  pure def receiptQualifies(s:State,r:TargetReceipt):bool=and {r.actual,r.canonical,r.finalized,
    r.exactBytes,r.identityMatched,r.fullEffects,r.feesMatched,r.epoch==s.epoch,r.profile==s.profile}
  pure def fullReceipt(s:State):TargetReceipt={actual:true,canonical:true,finalized:true,exactBytes:true,
    identityMatched:true,fullEffects:true,feesMatched:true,epoch:s.epoch,profile:s.profile}
  pure def localRecorded(s:State,n:Node):State={...s,
    evidence:s.evidence.set(n,{status:Passed,epoch:s.epoch,profile:s.profile}),
    milestones:(if(n==ParentVerify) s.milestones.union(Set(RecursiveProof))
      else if(n==Benchmark) s.milestones.union(Set(DegreeAndBenchmark)) else s.milestones),
    milestoneEpoch:s.epoch}
  var state:State
  action initWith(permit:bool,p:Profile):bool=state'=initial(permit,p)
  // Separate checked scenario; default init intentionally remains unchanged.
  action initFinancialWith(permit:bool,p:Profile,financial:bool):bool=
    state'={...initial(permit,p),financialPremise:financial}
  action initFinancialFalse:bool=initFinancialWith(true,ExactIntent,false)
  action init:bool={nondet permit=Set(true,false).oneOf() nondet p=Set(ExactIntent,FixedS0,Native16).oneOf()
    initWith(permit,p)}
  action observeLocal(n:Node,epochArg:int,p:Profile,actual:bool,independent:bool,tupleMatches:bool):bool=all {
    canObserveLocal(state,n,epochArg,p,actual,independent,tupleMatches),state'=localRecorded(state,n)}
  // Actual external observations, not commands launched by this model.
  action passedLocal(n:Node):bool=observeLocal(n,state.epoch,state.profile,true,true,true)
  action dispatchVerifier(n:Node):bool=all {factoryRunnable(n),state.historyPresent,dependencies(state,n),
    state.evidence.get(n).status==Pending,not(state.verifierDispatches.contains(n)),
    state'={...state,verifierDispatches:state.verifierDispatches.union(Set(n))}}
  action observeVerifier(n:Node,r:Verification):bool=all {VERIFIERS.contains(n),
    state.evidence.get(n).status==Pending,dependencies(state,n),state.verifierDispatches.contains(n),
    verificationQualifies(state,n,r),state'={...localRecorded(state,n),
      parentVerification:(if(n==ParentVerify) r else state.parentVerification),
      dVerification:(if(n==DVerify) r else state.dVerification)}}
  action recordCapture(quality:CaptureQuality,pisEqual:bool,actual:bool,tupleMatches:bool):bool=all {
    state.evidence.get(MP2).status==Pending,dependencies(state,MP2),
    quality==CapturedCrossChecked,pisEqual,actual,tupleMatches,
    state'={...localRecorded(state,MP2),captureQuality:quality,compiledPIsEqual:pisEqual}}
  action recordFailure(n:Node,unknown:bool):bool=all {LOCAL.contains(n),dependencies(state,n),
    state.evidence.get(n).status==Pending,state'={...state,evidence:state.evidence.set(n,
      {status:(if(unknown) Unknown else Failed),epoch:state.epoch,profile:state.profile})}}
  action reconcileOriginal(result:ReadResult):bool=all {state.originalReads<2,
    state'={...state,originalReads:state.originalReads+1,
      original:(if(result==FullFinalizedExact) OriginalReconciled else state.original)}}
  pure def canSubmitTarget(s:State,identity:int):bool=and {identity==1,s.newSends==0,
    s.original==OriginalReconciled,current(s,DVerify),s.targetPermit,s.financialPremise}
  action submitTarget(identity:int):bool=all {canSubmitTarget(state,identity),
    state'={...state,newSends:1}}
  action observeTarget(r:TargetReceipt):bool=all {dependencies(state,TargetReadback),
    state.evidence.get(TargetReadback).status==Pending,
    state'={...state,readbackObserved:true,evidence:state.evidence.set(TargetReadback,
      {status:(if(receiptQualifies(state,r)) Passed else Unknown),epoch:state.epoch,profile:state.profile}),
      milestones:(if(receiptQualifies(state,r)) state.milestones.union(Set(StagenetReadback)) else state.milestones),
      milestoneEpoch:state.epoch}}
  action acceptSource:bool=all {not(state.sourceAccepted),state'={...state,sourceAccepted:true,sourceEpoch:state.epoch}}
  action editInputs:bool=all {state.epoch==0,state'={...state,epoch:1}}
  action removeHistory:bool=all {state.historyPresent,state'={...state,historyPresent:false}}
  action progress:bool=any {acceptSource,
    {nondet n=LOCAL.oneOf() passedLocal(n)},
    {nondet n=VERIFIERS.oneOf() dispatchVerifier(n)},
    {nondet n=VERIFIERS.oneOf() observeVerifier(n,goodVerification(state))},
    recordCapture(CapturedCrossChecked,true,true,true),
    reconcileOriginal(FullFinalizedExact),submitTarget(1),observeTarget(fullReceipt(state)) }
  action explore:bool=any {progress,acceptSource,editInputs,removeHistory,
    {nondet n=LOCAL.oneOf() nondet unknown=Set(true,false).oneOf() recordFailure(n,unknown)},
    {nondet result=Set(Absent,ReadError,WrongIdentity).oneOf() reconcileOriginal(result)},
    observeTarget({...fullReceipt(state),finalized:false}) }
  action step:bool=any {progress,explore}
  val firstLoopComplete:bool=and {current(state,ParentVerify),current(state,DVerify),
    current(state,TargetReadback),current(state,Benchmark),state.profile==ExactIntent,
    state.milestoneEpoch==state.epoch,
    Set(RecursiveProof,StagenetReadback,DegreeAndBenchmark).subseteq(state.milestones)}
  val loopInvariant:bool=and {state.originalSends==1,state.newSends<=1,
    current(state,ParentVerify) implies current(state,ParentProof) and verificationQualifies(state,ParentVerify,state.parentVerification),
    current(state,MP2) implies current(state,Build9) and current(state,ParentVerify)
      and state.captureQuality==CapturedCrossChecked and state.compiledPIsEqual,
    current(state,DPrepare) implies current(state,ParentVerify) and current(state,MP2),
    current(state,DProof) implies current(state,DPrepare),
    current(state,DVerify) implies current(state,DProof) and verificationQualifies(state,DVerify,state.dVerification),
    current(state,TargetReadback) implies current(state,DVerify) and state.newSends==1,
    current(state,Benchmark) implies current(state,TargetReadback) and current(state,SyntheticImport) and state.profile==ExactIntent,
    state.newSends==1 implies state.original==OriginalReconciled }
  val parentWitness:bool=current(state,ParentProof)
  val verifiedParentWitness:bool=current(state,ParentVerify)
  val buildWitness:bool=current(state,Build9)
  val importerWitness:bool=current(state,SyntheticImport)
  val sourceWitness:bool=state.sourceAccepted
  val mp2Witness:bool=current(state,MP2)
  val dWitness:bool=current(state,DVerify)
  val originalResolvedWitness:bool=state.original==OriginalReconciled
  val unresolvedReadWitness:bool=state.original==OriginalUnknown and state.originalReads>0
  val targetWitness:bool=current(state,TargetReadback)
  val benchmarkWitness:bool=current(state,Benchmark)
  val completeWitness:bool=firstLoopComplete
  val financialRefusalInvariant:bool=not(state.financialPremise) implies state.newSends==0
  val financialBlockedWhenOtherwiseReady:bool=not(state.financialPremise) and state.targetPermit
    and state.original==OriginalReconciled and current(state,DVerify) and state.newSends==0
  val retainedFailureWitness:bool=NODES.exists(n=>Set(Failed,Unknown).contains(state.evidence.get(n).status))
}
