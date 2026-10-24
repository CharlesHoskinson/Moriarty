module interpreterChunks {
 import aPure as A from "./aPure"
 import aInputs as P from "./aInputs"
 import financeTypes.* from "./financeTypes"
 import financeLogic as F from "./financeLogic"
 type StopReason=RunningStop|BudgetStop|TerminalStop|ObserverStop
 type Phase=Observing|Stopped|CutPhase|ProjectedPhase
 type Projection=NotProjected|SemanticIdentity|ScalarProjected|ProjectionCapacityRefused
 type ScalarImage={available:bool,space:str,index:int,constants:List[int],data:List[int],activeData:int}
 type Event={before:A::Snapshot,after:A::Snapshot,rule:int,charge:int}
 type Observer={budget:int,count:int,breakRequested:bool,stop:StopReason}
 type Labels={program:A::Program,profile:str,layout:str,fuelBound:int,frameBound:int,eventBound:int,cutBound:int}
 type Boundary={snapshot:A::Snapshot,finance:Finance,events:List[Event],work:int,observer:Observer,phase:Phase,labels:Labels,calls:int}
 type State={initial:A::Snapshot,current:A::Snapshot,events:List[Event],work:int,labels:Labels,observer:Observer,phase:Phase,cuts:int,saved:Boundary,rawImage:ScalarImage,canonicalImage:ScalarImage,projection:Projection,projectionCapacity:int,projectionLocation:str,finance:Finance,calls:int,probes:int,lastAction:str}
 var s:State
 pure val noImage:ScalarImage={available:false,space:"absent",index:0,constants:List(),data:List(),activeData:0}
 pure def terminal(q:A::Snapshot):bool=q.outcome!=A::Pending
 pure def observedDomain(q:A::Snapshot):bool=and{q.initial>=0,q.initial<=28,q.remaining>=0,q.remaining<=q.initial,q.frames.length()<=8,Set(0,1,2,8).contains(q.stackLimit),not(q.outcome==A::Pending and q.control==A::EmptyControl)}
 pure def declaredEntry(q:A::Snapshot):bool={
  val knownProgram=Set(P::n01Program,P::n02Program,P::n04Program,P::n05Program,P::otherProgram).contains(q.program)
  val genesis=and{q.control==A::Eval(q.program.entry),q.environment==Map(),q.frames==List(),q.remaining==q.initial,q.outcome==A::Pending}
  val terminalFixture=and{q.program==P::n01Program,q.control==A::EmptyControl,q.outcome==A::Done(A::UInt(7)),q.environment==Map(),q.frames==List(),q.remaining==0,q.initial==0,q.stackLimit==8}
  val returnErrorFixture=and{q.program==P::n01Program,q.environment==Map(),q.frames==List(),q.remaining==0,q.initial==0,q.stackLimit==8,q.outcome==A::Pending,q.control==A::Return(A::UInt(7)) or q.control==A::Error(3)}
  val framedFixture=and{q.program==P::n01Program,q.control==A::Return(A::UInt(7)),q.environment==Map(5 -> A::UInt(2)),q.frames==List(A::ScopeFrame(Map(5 -> A::UInt(1)))),q.stackLimit==1,q.remaining==0,q.initial==0,q.outcome==A::Pending}
  val pushFixture=and{q.program==P::n01Program,q.control==A::Push({frame:A::ScopeFrame(Map(5 -> A::UInt(1))),expression:0,environment:Map(5 -> A::UInt(2))}),q.environment==Map(),q.frames==List(),q.stackLimit==0,q.remaining==1,q.initial==1,q.outcome==A::Pending}
  knownProgram and (genesis or terminalFixture or returnErrorFixture or framedFixture or pushFixture)
 }
 pure def labelsFor(q:A::Snapshot):Labels={program:q.program,profile:"sherlock-k-a/1",layout:"semantic8+scalar-projection-fixture/1",fuelBound:28,frameBound:8,eventBound:29,cutBound:3}
 pure def boundaryOf(q:State):Boundary={snapshot:q.current,finance:q.finance,events:q.events,work:q.work,observer:q.observer,phase:q.phase,labels:q.labels,calls:q.calls}
 pure def initialState(q:A::Snapshot):State={
  val observer:Observer={budget:0,count:0,breakRequested:false,stop:BudgetStop}
  val labels=labelsFor(q)
  val boundary:Boundary={snapshot:q,finance:F::initialFinance,events:List(),work:0,observer:observer,phase:Stopped,labels:labels,calls:0}
  {initial:q,current:q,events:List(),work:0,labels:labels,observer:observer,phase:Stopped,cuts:0,saved:boundary,rawImage:noImage,canonicalImage:noImage,projection:NotProjected,projectionCapacity:1,projectionLocation:"none",finance:F::initialFinance,calls:0,probes:0,lastAction:"initCase"}
 }
 pure def openObservation(q:State,budget:int,requestBreak:bool):State={...q,observer:{budget:budget,count:0,breakRequested:requestBreak,stop:if(budget==0)BudgetStop else RunningStop},phase:if(budget==0)Stopped else Observing,calls:q.calls+1,probes:0,lastAction:if(budget==0)"zeroBudget" else "startCall"}
 pure def canObserve(q:State):bool=and{q.phase==Observing,q.observer.count<q.observer.budget,not(terminal(q.current)),q.current.control!=A::EmptyControl,q.events.length()<29}
 pure def appendObserved(q:State):State={
  val tr=A::advance(q.current)
  val nextCount=q.observer.count+1
  val stop=if(terminal(tr.state))TerminalStop else if(q.observer.breakRequested)ObserverStop else if(nextCount==q.observer.budget)BudgetStop else RunningStop
  {...q,current:tr.state,events:q.events.append({before:q.current,after:tr.state,rule:tr.rule,charge:tr.charge}),work:q.work+tr.charge,observer:{...q.observer,count:nextCount,stop:stop},phase:if(stop==RunningStop)Observing else Stopped,lastAction:"observeEvent"}
 }
 pure def takeGroup(q:State,budget:int,requestBreak:bool):State={
  val opened=openObservation(q,budget,requestBreak)
  range(0,budget).foldl(opened,(acc,i)=>if(canObserve(acc))appendObserved(acc) else if(acc.phase==Observing and terminal(acc.current)){...acc,phase:Stopped,observer:{...acc.observer,stop:TerminalStop},lastAction:"probeNoNext"} else acc)
 }
 pure def semanticBoundaryPreserved(a:Boundary,b:Boundary):bool=and{a.snapshot==b.snapshot,a.finance==b.finance,a.events==b.events,a.work==b.work,a.observer==b.observer,a.labels==b.labels,a.calls==b.calls}
 pure def isScalarCarrier(q:A::Snapshot):bool=q.outcome==A::Done(A::UInt(7)) or (q.outcome==A::Pending and q.control==A::Return(A::UInt(7)) and q.frames.length()==0)
 pure def imageFor(q:A::Snapshot):ScalarImage=if(isScalarCarrier(q)){available:true,space:"Const",index:0,constants:List(7),data:List(),activeData:0} else noImage
 pure def imageMeaning(i:ScalarImage):int=if(i.space=="Const")i.constants.nth(i.index) else i.data.nth(i.index)
 pure def doCut(q:State,capacity:int):State={...q,saved:boundaryOf(q),phase:CutPhase,cuts:q.cuts+1,rawImage:imageFor(q.current),canonicalImage:noImage,projection:NotProjected,projectionCapacity:capacity,projectionLocation:"none",lastAction:"cut"}
 pure def doProject(q:State):State=if(q.rawImage.available and q.projectionCapacity==0){...q,projection:ProjectionCapacityRefused,projectionLocation:"Data[0]",lastAction:"projectBoundary"} else if(q.rawImage.available){...q,canonicalImage:{available:true,space:"Data",index:0,constants:List(7),data:List(7),activeData:1},projection:ScalarProjected,phase:ProjectedPhase,lastAction:"projectBoundary"} else {...q,projection:SemanticIdentity,phase:ProjectedPhase,lastAction:"projectBoundary"}
 pure def doResume(q:State):State={...q,current:q.saved.snapshot,finance:q.saved.finance,events:q.saved.events,work:q.saved.work,observer:q.saved.observer,labels:q.saved.labels,calls:q.saved.calls,phase:q.saved.phase,lastAction:"resume"}
 pure def snapshots(q:State):List[A::Snapshot]=List(q.initial).concat(q.events.foldl(List(),(acc,e)=>acc.append(e.after)))
 pure def rules(q:State):List[int]=q.events.foldl(List(),(acc,e)=>acc.append(e.rule))
 pure def chain(q:State):bool=and{
  if(q.events.length()==0)q.current==q.initial else and{q.events.head().before==q.initial,q.events.nth(q.events.length()-1).after==q.current},
  q.events.indices().forall(i=>if(i==0)true else q.events.nth(i-1).after==q.events.nth(i).before),
  q.work==q.events.foldl(0,(acc,e)=>acc+e.charge),
  q.events.indices().forall(i=>{val e=q.events.nth(i) and{e.rule!=0,e.charge==0 or e.charge==1,e.before.program==e.after.program,e.before.initial==e.after.initial,e.before.remaining-e.after.remaining==e.charge}}),
 }
 pure def invariant(q:State):bool=and{observedDomain(q.current),chain(q),q.labels==labelsFor(q.initial),q.current.program==q.labels.program,q.current.remaining+q.work==q.initial.remaining,q.cuts<=3,q.calls<=16,q.events.length()<=29,q.observer.count<=q.observer.budget,F::financeInvariant(q.finance),if(q.phase==CutPhase or q.phase==ProjectedPhase)semanticBoundaryPreserved(boundaryOf(q),q.saved) else true,if(q.projection==ScalarProjected)and{q.rawImage.available,q.canonicalImage.available,imageMeaning(q.rawImage)==imageMeaning(q.canonicalImage)} else true}
 action initCase(q:A::Snapshot):bool=all{observedDomain(q),declaredEntry(q),s'=initialState(q)}
 action init={
  nondet program=Set(P::n01Program,P::n02Program,P::n04Program,P::n05Program,P::otherProgram).oneOf()
  nondet fuel=0.to(28).oneOf()
  nondet limit=Set(0,1,2,8).oneOf()
  initCase(A::initialSnapshot(program,fuel,limit))
 }
 action startCall(budget:int,requestBreak:bool):bool=all{s.phase==Stopped,s.calls<16,Set(0,1,2,4,7).contains(budget),s'=openObservation(s,budget,requestBreak)}
 action zeroBudget:bool=startCall(0,false)
 action observerBreak:bool=startCall(4,true)
 action observeEvent:bool=all{canObserve(s),s'=appendObserved(s)}
 action advancePending:bool=observeEvent
 action group(budget:int,requestBreak:bool):bool=all{s.phase==Stopped,s.calls<16,Set(0,1,2,4,7).contains(budget),s'=takeGroup(s,budget,requestBreak)}
 action cut(capacity:int):bool=all{s.phase!=CutPhase,s.phase!=ProjectedPhase,s.cuts<3,Set(0,1).contains(capacity),s'=doCut(s,capacity)}
 action projectBoundary:bool=all{s.phase==CutPhase,s.projection==NotProjected,s'=doProject(s)}
 action resume:bool=all{s.phase==ProjectedPhase,not(terminal(s.saved.snapshot)),s'=doResume(s)}
 action probeNoNext:bool=all{terminal(s.current),s.phase==Observing,s.observer.budget>0,s.observer.count==0,s.probes==0,A::advance(s.current)=={state:s.current,rule:0,charge:0},s'= {...s,probes:s.probes+1,phase:Stopped,observer:{...s.observer,stop:TerminalStop},lastAction:"probeNoNext"}}
 action sourcePrefixObserved:bool=all{s.phase==Stopped,s.finance.phase=="initial",s'={...s,finance:F::paidPrefix(s.finance),lastAction:"sourcePrefixObserved"}}
 action tentativeObserved:bool=all{s.phase==Stopped,s.finance.phase=="committed",s'={...s,finance:F::nextTentative(s.finance),lastAction:"tentativeObserved"}}
 action localSegmentFailed:bool=all{s.phase==Stopped,s.finance.phase=="tentative",s'={...s,finance:F::failTentative(s.finance),lastAction:"localSegmentFailed"}}
 action step={
  nondet budget=Set(0,1,2,4,7).oneOf()
  nondet capacity=Set(0,1).oneOf()
  any{startCall(budget,false),observerBreak,observeEvent,cut(capacity),projectBoundary,resume,probeNoNext,sourcePrefixObserved,tentativeObserved,localSegmentFailed}
 }
 val allInvariant=invariant(s)
 val observed=s.events.length()>0
 val freePublication=s.events.foldl(false,(found,e)=>found or (e.rule==80 and e.charge==0))
 val terminalObserved=s.observer.stop==TerminalStop
 val callLimitReached=s.calls==16
 val zeroObserved=s.lastAction=="zeroBudget"
 val observerStopped=s.observer.stop==ObserverStop
 val cutReached=s.lastAction=="cut"
 val scalarProjected=s.projection==ScalarProjected
 val semanticProjected=s.projection==SemanticIdentity
 val projected=s.lastAction=="projectBoundary" and s.projection!=ProjectionCapacityRefused
 val projectionRefused=s.projection==ProjectionCapacityRefused
 val resumed=s.lastAction=="resume"
 val probed=s.probes>0
 val paid=s.finance.phase=="committed"
 val tentative=s.finance.phase=="tentative"
 val localFailed=s.finance.phase=="failed"
}
