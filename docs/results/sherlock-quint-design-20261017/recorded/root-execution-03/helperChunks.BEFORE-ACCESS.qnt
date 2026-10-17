module helperChunks {
 import bPure as B from "./bPure"
 import aPure as A from "./aPure"
 import financeTypes.* from "./financeTypes"
 import financeLogic as F from "./financeLogic"
 type BRecord={before:B::State,after:B::State,event:B::Event}
 type Saved={current:B::State,records:List[BRecord],finance:Finance}
 type Harness={initial:B::State,current:B::State,records:List[BRecord],saved:Saved,finance:Finance,phase:str,cuts:int,probes:int,noNextResponse:B::State,lastAction:str}
 var h:Harness
 pure val input:B::Input={caseId:"lookup-deeper",control:B::EvalVariable("x"),envRoot:B::At(1),cells:List({binder:"x",value:0,previous:B::Absent},{binder:"y",value:2,previous:B::At(0)}),params:List(),args:List()}
 pure def limits(fuel:int):B::Limits={initialFuel:fuel,maxHelperWork:8,environmentCells:2,helperTasks:1}
 pure def initialHarness(fuel:int):Harness={
  val q=B::initial(input,limits(fuel))
  {initial:q,current:q,records:List(),saved:{current:q,records:List(),finance:F::initialFinance},finance:F::initialFinance,phase:"active",cuts:0,probes:0,noNextResponse:q,lastAction:"initCase"}
 }
 pure def canEvent(q:Harness):bool=q.phase=="active" and q.current.machine.outcome==B::Running and not(B::isCutpoint(q.current.machine))
 pure def applyEvent(q:Harness):Harness={
  val post=B::advance(q.current)
  match post.lastEvent{
   | B::StepEvent(event)=>{...q,current:post,records:q.records.append({before:q.current,after:post,event:event}),lastAction:"helperEvent"}
   | B::NoEvent=>q // guarded component excludes terminal/cutpoint; no record invented.
  }
 }
 pure def getOrigin(mm:B::Machine):int=match mm.helper{
  | B::GetHelper(gg)=>gg.originReferenceCursor
  | _=>mm.referenceCursor
 }
 pure def rules(q:Harness):List[str]=q.records.foldl(List(),(acc,e)=>acc.append(e.event.ruleId))
 // Completed-interval projection only; B exhaustion stays visible in current.
 pure val mappedOrigin:A::Snapshot={program:{expressions:Map(0 -> A::Variable(0)),functions:List(),entry:0},control:A::Eval(0),environment:Map(0 -> A::UInt(0),1 -> A::BoolValue(false)),frames:List(),stackLimit:1,remaining:1,initial:1,outcome:A::Pending}
 pure def erased(q:Harness):A::Snapshot=if(q.current.machine.referenceCursor==0)mappedOrigin else if(q.current.machine.outcome==B::Halted(0)){...mappedOrigin,control:A::EmptyControl,remaining:0,outcome:A::Done(A::UInt(0))} else {...mappedOrigin,control:A::Return(A::UInt(0)),remaining:0}
 pure def invariant(q:Harness):bool=and{
  q.current.input==q.initial.input,q.current.limits==q.initial.limits,
  q.current.machine.remainingWork+q.current.machine.consumedWork==q.initial.limits.initialFuel,
  q.current.machine.helperWork<=q.current.machine.consumedWork,
  q.current.machine.helperWork<=8,q.current.machine.helperVisits<=q.current.machine.helperWork,
  q.current.machine.allocEnvironmentCells==2,q.current.machine.cells==input.cells,
  q.current.machine.tasks==List(),q.cuts<=3,q.records.length()<=7,
  if(q.probes==1)and{q.noNextResponse.input==q.current.input,q.noNextResponse.limits==q.current.limits,q.noNextResponse.machine==q.current.machine,q.noNextResponse.lastEvent==B::NoEvent} else true,
  q.current.machine.consumedWork==q.records.foldl(0,(n,e)=>n+e.event.workDelta),
  if(q.records.length()==0)q.current==q.initial else and{q.records.head().before==q.initial,q.records.nth(q.records.length()-1).after==q.current},
  q.records.indices().forall(i=>if(i==0)true else q.records.nth(i-1).after==q.records.nth(i).before),
  q.current.machine.referenceCursor==0 or q.current.machine.referenceCursor==1,
  if(q.current.machine.referenceCursor==0)getOrigin(q.current.machine)==0 else and{q.current.machine.consumedWork==5,q.current.machine.consumedWork==q.current.machine.referenceCursor+q.current.machine.helperWork,q.current.machine.helperVisits==2},
  if(q.phase=="cut")and{q.current==q.saved.current,q.records==q.saved.records,q.finance==q.saved.finance} else true,
  F::financeInvariant(q.finance),
 }
 action initCase(fuel:int):bool=all{0.to(6).contains(fuel),h'=initialHarness(fuel)}
 action init={nondet fuel=0.to(6).oneOf() initCase(fuel)}
 action helperEvent:bool=all{canEvent(h),h'=applyEvent(h)}
 action cut:bool=all{h.phase=="active",h.cuts<3,h'={...h,saved:{current:h.current,records:h.records,finance:h.finance},phase:"cut",cuts:h.cuts+1,lastAction:"cut"}}
 action resume:bool=all{h.phase=="cut",h.saved.current.machine.outcome==B::Running,h'={...h,current:h.saved.current,records:h.saved.records,finance:h.saved.finance,phase:"active",lastAction:"resume"}}
 action probeNoNext:bool=all{h.phase=="active",h.current.machine.outcome!=B::Running,h.probes==0,h'={...h,probes:1,noNextResponse:B::advance(h.current),lastAction:"probeNoNext"}}
 action step=any{helperEvent,cut,resume,probeNoNext}
 val allInvariant=invariant(h)
 val entered=h.records.length()>0
 val helperVisited=h.current.machine.helperVisits>0
 val exited=h.current.machine.result==B::EnvFound(0)
 val completed=h.current.machine.referenceCursor==1
 val published=h.current.machine.outcome==B::Halted(0)
 val exhausted=h.current.machine.outcome==B::FuelExhausted
 val cutReached=h.lastAction=="cut"
 val resumed=h.lastAction=="resume"
 val probed=h.probes==1
}
