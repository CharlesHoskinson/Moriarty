// Finite readiness/dispatch refinement; opaque epochs are not filesystem proofs.
module doctorwatson_readiness {
  type Adapter = SdkBuild | HandkeyVerify | SettleReadback | RollupBenchmark
  type Attempt = NotStarted | Running | Succeeded | Failed | Unknown
  type Capability = {executorAvailable:bool,collectorAvailable:bool}
  type Launch = {task:int,fingerprint:int,history:bool,dependencyReady:bool,
    executor:bool,collector:bool,unknown:bool,debt:bool}
  type State = {historyPresent:bool,sharedEpoch:int,assessment:int,assessedEpoch:int,assessedHistory:bool,
    fingerprints:int->int,dependencies:int->bool,unknown:int->bool,debt:int->bool,
    attempts:int->Attempt,selected:int,selectedFingerprint:int,recheckAssessment:int,
    launches:Set[Launch],sourceAccepted:bool,proofAccepted:bool,targetAccepted:bool,
    explicitProofObservation:bool,explicitTargetObservation:bool}
  pure val TASKS:Set[int]=Set(1,2,3,4)
  pure def adapter(id:int):Adapter=
    if(id==1) SdkBuild else if(id==2) HandkeyVerify else if(id==3) SettleReadback else RollupBenchmark
  // Shared Python capability interface: build and new local verifier launchable;
  // settle collector only; remaining known adapters have neither capability.
  pure def capability(a:Adapter):Capability=match a {
    | SdkBuild=>{executorAvailable:true,collectorAvailable:true}
    | HandkeyVerify=>{executorAvailable:true,collectorAvailable:true}
    | SettleReadback=>{executorAvailable:false,collectorAvailable:true}
    | RollupBenchmark=>{executorAvailable:false,collectorAvailable:false} }
  pure def fingerprint(epoch:int,id:int):int=if(Set(1,2).contains(id)) epoch*10+id else id
  pure def initial(history:bool):State={historyPresent:history,sharedEpoch:0,assessment:0,
    assessedEpoch:-1,assessedHistory:false,fingerprints:TASKS.mapBy(id=>-1),dependencies:TASKS.mapBy(id=>true),
    unknown:TASKS.mapBy(id=>false),debt:TASKS.mapBy(id=>false),attempts:TASKS.mapBy(id=>NotStarted),
    selected:0,selectedFingerprint:-1,recheckAssessment:0,launches:Set(),sourceAccepted:false,
    proofAccepted:false,targetAccepted:false,explicitProofObservation:false,explicitTargetObservation:false}
  pure def assessmentCurrent(s:State):bool=s.assessment>0 and s.assessedEpoch==s.sharedEpoch and s.assessedHistory==s.historyPresent
  // Qualified compatibility candidate, not merely raw DAG edges and not execution authority.
  pure def dependencyReady(s:State,id:int):bool=and {assessmentCurrent(s),s.dependencies.get(id),
    not(s.unknown.get(id)),not(s.debt.get(id)),s.attempts.get(id)==NotStarted}
  pure def readyForDispatch(s:State,id:int):bool={
    val c=capability(adapter(id))
    dependencyReady(s,id) and c.executorAvailable and c.collectorAvailable }
  pure def canDispatch(s:State):bool=if(not(TASKS.contains(s.selected))) false else and {
    s.historyPresent,readyForDispatch(s,s.selected),s.recheckAssessment==s.assessment,
    s.selectedFingerprint==s.fingerprints.get(s.selected),
    s.selectedFingerprint==fingerprint(s.sharedEpoch,s.selected)}
  pure def assessed(s:State):State={...s,assessment:s.assessment+1,assessedEpoch:s.sharedEpoch,assessedHistory:s.historyPresent,
    fingerprints:TASKS.mapBy(id=>fingerprint(s.sharedEpoch,id)),recheckAssessment:0}
  pure def launchAt(s:State):Launch={task:s.selected,fingerprint:s.selectedFingerprint,
    history:s.historyPresent,dependencyReady:dependencyReady(s,s.selected),
    executor:capability(adapter(s.selected)).executorAvailable,
    collector:capability(adapter(s.selected)).collectorAvailable,
    unknown:s.unknown.get(s.selected),debt:s.debt.get(s.selected)}
  var state:State
  action initWith(history:bool):bool=state'=initial(history)
  action init:bool={nondet history=Set(true,false).oneOf() initWith(history)}
  action initCounterexample:bool=initWith(true)
  action assess:bool=all {state.assessment<5,state'=assessed(state)}
  // Legacy Root selection may identify dependency-ready tasks even lacking an executor.
  action selectTask(id:int):bool=all {TASKS.contains(id),dependencyReady(state,id),
    state'={...state,selected:id,selectedFingerprint:state.fingerprints.get(id),recheckAssessment:0}}
  // Independent use-time assessment, not reuse of an old assessment-local digest cache.
  action recheck:bool=all {TASKS.contains(state.selected),state.assessment<5,
    state'={...assessed(state),recheckAssessment:state.assessment+1}}
  action dispatch:bool=all {canDispatch(state),state'={...state,
    attempts:state.attempts.set(state.selected,Running),launches:state.launches.union(Set(launchAt(state)))}}
  action outcome(id:int,result:Attempt,cleanupDebt:bool):bool=all {
    TASKS.contains(id),state.attempts.get(id)==Running,Set(Succeeded,Failed,Unknown).contains(result),
    state'={...state,attempts:state.attempts.set(id,result),
      unknown:state.unknown.set(id,state.unknown.get(id) or result==Unknown),
      debt:state.debt.set(id,state.debt.get(id) or cleanupDebt)}}
  action provideHistory:bool=all {not(state.historyPresent),state'={...state,historyPresent:true}}
  action editSharedInput:bool=all {state.sharedEpoch==0,state'={...state,sharedEpoch:1}}
  action markUnknown(id:int):bool=all {TASKS.contains(id),not(state.unknown.get(id)),
    state'={...state,unknown:state.unknown.set(id,true)}}
  action markDebt(id:int):bool=all {TASKS.contains(id),not(state.debt.get(id)),
    state'={...state,debt:state.debt.set(id,true)}}
  action missingDependency(id:int):bool=all {TASKS.contains(id),state.dependencies.get(id),
    state'={...state,dependencies:state.dependencies.set(id,false)}}
  action acceptSource:bool=all {not(state.sourceAccepted),state'={...state,sourceAccepted:true}}
  // These observations are external premises, not inferred from source or local verifier wiring.
  action observeProof:bool=all {not(state.proofAccepted),state'={...state,
    proofAccepted:true,explicitProofObservation:true}}
  action observeTarget:bool=all {not(state.targetAccepted),state'={...state,
    targetAccepted:true,explicitTargetObservation:true}}
  action step:bool=any {assess,provideHistory,editSharedInput,acceptSource,observeProof,observeTarget,
    {nondet id=TASKS.oneOf() selectTask(id)},recheck,dispatch,
    {nondet id=TASKS.oneOf() markUnknown(id)},
    {nondet id=TASKS.oneOf() markDebt(id)},
    {nondet id=TASKS.oneOf() missingDependency(id)},
    {nondet id=TASKS.oneOf() nondet result=Set(Succeeded,Failed,Unknown).oneOf()
      nondet debtArg=Set(true,false).oneOf() outcome(id,result,debtArg)} }
  action counterexampleStep:bool=assess
  val readinessInvariant:bool=and {
    state.launches.forall(l=>and {l.history,l.dependencyReady,l.executor,l.collector,not(l.unknown),not(l.debt)}),
    TASKS.forall(id=>state.launches.filter(l=>l.task==id).size()<=1),
    state.proofAccepted implies state.explicitProofObservation,
    state.targetAccepted implies state.explicitTargetObservation,
    canDispatch(state) implies state.historyPresent and assessmentCurrent(state),
    TASKS.forall(id=>readyForDispatch(state,id) implies dependencyReady(state,id)) }
  val naiveDependenciesImplyRunnable:bool=TASKS.forall(id=>dependencyReady(state,id) implies readyForDispatch(state,id))
  val assessedWitness:bool=assessmentCurrent(state)
  val selectedWitness:bool=state.selected!=0
  val freshRecheckWitness:bool=state.recheckAssessment==state.assessment and state.recheckAssessment>0
  val buildLaunchWitness:bool=state.launches.exists(l=>l.task==1)
  val verifierLaunchWitness:bool=state.launches.exists(l=>l.task==2)
  val unavailableAdapterWitness:bool=dependencyReady(state,3) and not(readyForDispatch(state,3))
  val historyBlocksWitness:bool=not(state.historyPresent) and state.selected==2 and not(canDispatch(state))
  val retainedObligationWitness:bool=state.sharedEpoch==1 and (state.unknown.get(2) or state.debt.get(2))
  val staleSharedWitness:bool=state.sharedEpoch==1 and state.selected==2 and state.selectedFingerprint==2 and not(canDispatch(state))
  val terminalOutcomeWitness:bool=TASKS.exists(id=>Set(Succeeded,Failed,Unknown).contains(state.attempts.get(id)))
  val missingDependencyWitness:bool=TASKS.exists(id=>not(state.dependencies.get(id)))
  val proofObservationWitness:bool=state.explicitProofObservation
  val targetObservationWitness:bool=state.explicitTargetObservation
  val sourceOnlyWitness:bool=state.sourceAccepted and not(state.proofAccepted) and not(state.targetAccepted)
}
