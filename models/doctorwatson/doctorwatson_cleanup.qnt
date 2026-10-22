// One cycle finalizer. Opaque epochs do not prove filesystem, Git or inventory truth.
module doctorwatson_cleanup {
  type GraphStatus = NoRefresh | PlannedRefresh | FailedRefresh | VerifiedRefresh
  type Disposition = Pending | Retained | WorktreeRemoved | Deleted | CleanupFailed | CleanupUnknown
  type Reason = NoReason | ActiveTask | OpenPR | UniqueWork | UserData | InactivityUnknown | OwnerUnknown
  type DeleteResult = DeleteOK | DeleteFailure | DeleteUnknown
  type Entry = {cycle:int,source:int,graph:int,payload:int}
  type Candidate = {merged:bool,active:bool,openPR:bool,userData:bool,inactiveKnown:bool,ownerKnown:bool,
    requestedTip:int,tip:int,disposition:Disposition,reason:Reason,retentionRecord:bool}
  type Retirement = {candidate:int,merged:bool,tipMatched:bool,rootMatched:bool,unprotected:bool,
    graph:bool,wiki:bool,inventory:bool}
  type State = {applyMode:bool,requestedSource:int,source:int,graphArtifact:int,
    graphStatus:GraphStatus,graphSource:int,graphPin:int,graphHistory:Set[Entry],
    payload:int,entries:List[Entry],wikiConflict:bool,root:int,requestedRoot:int,
    inventoryKnown:bool,inventoryFresh:bool,candidates:int->Candidate,retirements:Set[Retirement],
    interrupted:bool,finished:bool,reportSaved:bool,reportFailed:bool,unrelatedProgress:int}
  pure val IDS:Set[int]=Set(1,2)
  pure val prefix:Entry={cycle:0,source:-1,graph:-1,payload:7}
  pure val clean:Candidate={merged:true,active:false,openPR:false,userData:false,inactiveKnown:true,ownerKnown:true,
    requestedTip:0,tip:0,disposition:Pending,reason:NoReason,retentionRecord:false}
  pure def initial(applyArg:bool):State={applyMode:applyArg,requestedSource:0,source:0,graphArtifact:0,
    graphStatus:NoRefresh,graphSource:-1,graphPin:-1,graphHistory:Set(),payload:0,entries:List(prefix),
    wikiConflict:false,root:0,requestedRoot:0,inventoryKnown:true,inventoryFresh:true,
    candidates:IDS.mapBy(id=>clean),retirements:Set(),interrupted:false,finished:false,reportSaved:false,reportFailed:false,
    unrelatedProgress:0}
  pure def currentGraph(s:State):bool=and {s.graphStatus==VerifiedRefresh,
    s.graphSource==s.requestedSource,s.graphSource==s.source,s.graphPin==s.graphArtifact}
  pure def wantedEntry(s:State):Entry={cycle:1,source:s.requestedSource,graph:s.graphPin,payload:s.payload}
  pure def cycleEntries(s:State):List[Entry]=s.entries.select(e=>e.cycle==1)
  pure def currentWiki(s:State):bool=currentGraph(s) and not(s.wikiConflict)
    and cycleEntries(s)==List(wantedEntry(s))
  pure def reasonFor(c:Candidate):Reason=if(c.active) ActiveTask else if(c.openPR) OpenPR
    else if(c.userData) UserData else if(not(c.inactiveKnown)) InactivityUnknown else if(not(c.ownerKnown)) OwnerUnknown
    else if(not(c.merged)) UniqueWork else NoReason
  pure def identityCurrent(s:State,id:int):bool=s.root==s.requestedRoot
    and s.candidates.get(id).tip==s.candidates.get(id).requestedTip
  pure def preCleanup(s:State):bool=and {s.applyMode,not(s.interrupted),not(s.finished),
    currentGraph(s),currentWiki(s),s.inventoryKnown,s.inventoryFresh}
  pure def eligible(s:State,id:int):bool=and {preCleanup(s),identityCurrent(s,id),
    reasonFor(s.candidates.get(id))==NoReason,s.candidates.get(id).merged}
  pure def canFinish(s:State):bool=preCleanup(s) and s.root==s.requestedRoot and not(s.reportFailed) and IDS.forall(id=>{
    val c=s.candidates.get(id)
    (c.disposition==Deleted and identityCurrent(s,id)) or (c.disposition==Retained and c.reason!=NoReason and c.retentionRecord) })
  var state:State
  action initWith(applyArg:bool):bool=state'=initial(applyArg)
  action init:bool={nondet applyArg=Set(true,false).oneOf() initWith(applyArg)}
  action refresh(statusArg:GraphStatus):bool=all {not(state.finished),not(state.interrupted),
    Set(PlannedRefresh,FailedRefresh,VerifiedRefresh).contains(statusArg),
    state'={...state,graphStatus:statusArg,graphSource:state.source,graphPin:state.graphArtifact,
      graphHistory:state.graphHistory.union(Set({cycle:1,source:state.source,graph:state.graphArtifact,payload:state.payload}))}}
  action appendWiki:bool=all {state.applyMode,not(state.finished),not(state.interrupted),currentGraph(state),
    state'=(if(cycleEntries(state).length()==0) {...state,entries:state.entries.append(wantedEntry(state))}
      else if(cycleEntries(state)==List(wantedEntry(state))) state else {...state,wikiConflict:true})}
  action retain(id:int,recordComplete:bool):bool=all {preCleanup(state),IDS.contains(id),
    state.candidates.get(id).disposition==Pending,reasonFor(state.candidates.get(id))!=NoReason,
    state'={...state,candidates:state.candidates.setBy(id,c=>{...c,disposition:(if(c.inactiveKnown and c.ownerKnown) Retained else CleanupFailed),
      reason:reasonFor(c),retentionRecord:recordComplete})}}
  action removeWorktree(id:int):bool=all {IDS.contains(id),eligible(state,id),
    state.candidates.get(id).disposition==Pending,
    state'={...state,candidates:state.candidates.setBy(id,c=>{...c,disposition:WorktreeRemoved})}}
  action deleteRef(id:int,result:DeleteResult):bool=all {IDS.contains(id),preCleanup(state),
    state.candidates.get(id).disposition==WorktreeRemoved,
    state'={...state,retirements:(if(eligible(state,id) and result==DeleteOK)
      state.retirements.union(Set({candidate:id,merged:state.candidates.get(id).merged,
        tipMatched:state.candidates.get(id).tip==state.candidates.get(id).requestedTip,
        rootMatched:state.root==state.requestedRoot,unprotected:reasonFor(state.candidates.get(id))==NoReason,
        graph:currentGraph(state),wiki:currentWiki(state),inventory:state.inventoryKnown and state.inventoryFresh}))
      else state.retirements),candidates:state.candidates.setBy(id,c=>{...c,
      disposition:(if(not(eligible(state,id)) or result==DeleteFailure) CleanupFailed
        else if(result==DeleteUnknown) CleanupUnknown else Deleted)})}}
  action finishCycle(reportDurable:bool):bool=all {canFinish(state),state'={...state,
    finished:reportDurable,reportSaved:reportDurable,reportFailed:not(reportDurable)}}
  action changeSource:bool=all {state.source==0,state'={...state,source:1,finished:false,reportSaved:false}}
  action changeArtifact:bool=all {state.graphArtifact==0,state'={...state,graphArtifact:1,finished:false,reportSaved:false}}
  action changePayload:bool=all {state.payload==0,state'={...state,payload:1,finished:false,reportSaved:false}}
  action moveTip(id:int):bool=all {IDS.contains(id),state.candidates.get(id).tip==0,
    state'={...state,finished:false,reportSaved:false,candidates:state.candidates.setBy(id,c=>{...c,tip:1})}}
  action moveRoot:bool=all {state.root==0,state'={...state,root:1,finished:false,reportSaved:false}}
  action protect(id:int,why:Reason):bool=all {IDS.contains(id),not(Set(Deleted,Retained).contains(state.candidates.get(id).disposition)),
    why!=NoReason,state'={...state,candidates:state.candidates.setBy(id,c=>{...c,
      active:why==ActiveTask,openPR:why==OpenPR,userData:why==UserData,
      inactiveKnown:why!=InactivityUnknown,ownerKnown:why!=OwnerUnknown,merged:why!=UniqueWork})}}
  action unknownInventory:bool=all {state.inventoryKnown,state'={...state,inventoryKnown:false}}
  action staleInventory:bool=all {state.inventoryFresh,state'={...state,inventoryFresh:false}}
  action interrupt:bool=all {not(state.interrupted),not(state.finished),state'={...state,interrupted:true,
    candidates:IDS.mapBy(id=>{val c=state.candidates.get(id)
      if(c.disposition==WorktreeRemoved) {...c,disposition:CleanupUnknown} else c})}}
  action unrelatedWork:bool=all {state.unrelatedProgress==0,state'={...state,unrelatedProgress:1}}
  action exploratoryStep:bool=any {
    {nondet result=Set(PlannedRefresh,FailedRefresh,VerifiedRefresh).oneOf() refresh(result)},appendWiki,
    {nondet id=IDS.oneOf() nondet complete=Set(true,false).oneOf() retain(id,complete)},
    {nondet id=IDS.oneOf() removeWorktree(id)},
    {nondet id=IDS.oneOf() nondet result=Set(DeleteOK,DeleteFailure,DeleteUnknown).oneOf() deleteRef(id,result)},
    {nondet durable=Set(true,false).oneOf() finishCycle(durable)},changeSource,changeArtifact,changePayload,
    {nondet id=IDS.oneOf() moveTip(id)},moveRoot,
    {nondet id=IDS.oneOf() nondet why=Set(ActiveTask,OpenPR,UniqueWork,UserData,InactivityUnknown,OwnerUnknown).oneOf() protect(id,why)},
    unknownInventory,staleInventory,interrupt,unrelatedWork }
  // Same actions, grouped to exercise ordinary progress as well as fault interleavings.
  action normalStep:bool=any {
    all {state.graphStatus==NoRefresh,refresh(VerifiedRefresh)},
    all {not(currentWiki(state)),appendWiki},
    {nondet id=IDS.oneOf() removeWorktree(id)},
    {nondet id=IDS.oneOf() retain(id,true)},
    {nondet id=IDS.oneOf() deleteRef(id,DeleteOK)},finishCycle(true) }
  action step:bool=any {normalStep,exploratoryStep}
  val cleanupInvariant:bool=and {
    state.entries.length()>=1,state.entries.nth(0)==prefix,cycleEntries(state).length()<=1,
    state.retirements.forall(r=>and {r.merged,r.tipMatched,r.rootMatched,r.unprotected,r.graph,r.wiki,r.inventory}),
    state.finished implies and {state.applyMode,state.reportSaved,currentGraph(state),currentWiki(state),
      not(state.interrupted),IDS.forall(id=>{
        val c=state.candidates.get(id)
        (c.disposition==Deleted and identityCurrent(state,id)) or (c.disposition==Retained and c.reason!=NoReason and c.retentionRecord) })},
    IDS.forall(id=>state.candidates.get(id).disposition==Deleted implies state.applyMode) }
  val graphWitness:bool=currentGraph(state)
  val wikiWitness:bool=currentWiki(state)
  val retainedWitness:bool=IDS.exists(id=>state.candidates.get(id).disposition==Retained)
  val removedWitness:bool=IDS.exists(id=>state.candidates.get(id).disposition==WorktreeRemoved)
  val deletedWitness:bool=IDS.exists(id=>state.candidates.get(id).disposition==Deleted)
  val partialWitness:bool=IDS.exists(id=>state.candidates.get(id).disposition==CleanupFailed)
  val unknownWitness:bool=IDS.exists(id=>state.candidates.get(id).disposition==CleanupUnknown)
  val conflictWitness:bool=state.wikiConflict
  val finishedWitness:bool=state.finished
  val interruptedWitness:bool=state.interrupted
  val unrelatedWitness:bool=state.unrelatedProgress==1
}
