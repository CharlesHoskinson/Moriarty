module interpreterChunks_test {
 import interpreterChunks.* from "./interpreterChunks"
 import aPure as A from "./aPure"
 import aInputs as P from "./aInputs"
 import aExpectations as E from "./aExpectations"
 import aPrefixExpectations as PE from "./aPrefixExpectations"
 import financeExpectations as FE from "./financeExpectations"
 // Existing independently authored whole Snapshot literals; no advance in expectations.
 run n01NamedTraceTest=initCase(E::n01State0).then(group(4,false))
  .expect(snapshots(s)==E::n01States and rules(s)==List(102,80) and s.work==1 and allInvariant)
 run n02NamedTraceTest=initCase(E::n02State0).then(group(4,false)).then(group(4,false))
  .expect(snapshots(s)==E::n02States and rules(s)==List(117,135,102,149,102,153,223,80) and s.work==7 and allInvariant)
 run n04NamedTraceTest=initCase(E::n04State0).then(5.reps(i=>group(4,false)))
  .expect(snapshots(s)==E::n04States and s.current==E::n04Published and s.work==17 and allInvariant)
 run n05NamedTraceTest=initCase(E::n05State0).then(7.reps(i=>group(4,false)))
  .expect(snapshots(s)==E::n05States and s.current==E::n05Published and s.work==27 and allInvariant)
 run twoProgramsTest=initCase({program:P::otherProgram,control:A::Eval(0),environment:Map(),frames:List(),stackLimit:8,remaining:1,initial:1,outcome:A::Pending}).then(group(4,false))
  .expect(s.current=={program:P::otherProgram,control:A::EmptyControl,environment:Map(),frames:List(),stackLimit:8,remaining:0,initial:1,outcome:A::Done(A::UInt(9))} and s.labels.program!=P::n01Program and rules(s)==List(102,80))
 run literalFuel0Test=initCase({program:P::n01Program,control:A::Eval(0),environment:Map(),frames:List(),stackLimit:8,remaining:0,initial:0,outcome:A::Pending}).then(group(4,false))
  .expect(s.current=={program:P::n01Program,control:A::EmptyControl,environment:Map(),frames:List(),stackLimit:8,remaining:0,initial:0,outcome:A::Exhausted} and rules(s)==List(84) and s.work==0)
 run literalFuel2Test=initCase({program:P::n01Program,control:A::Eval(0),environment:Map(),frames:List(),stackLimit:8,remaining:2,initial:2,outcome:A::Pending}).then(group(4,false))
  .expect(s.current=={program:P::n01Program,control:A::EmptyControl,environment:Map(),frames:List(),stackLimit:8,remaining:1,initial:2,outcome:A::Done(A::UInt(7))} and rules(s)==List(102,80) and s.work==1)
 run terminalReturnPrecedenceTest=initCase(E::returnZero).then(group(4,false))
  .expect(s.current==E::returnZeroDone and rules(s)==List(80) and s.work==0)
 run terminalErrorPrecedenceTest=initCase(E::errorZero).then(group(4,false))
  .expect(s.current==E::errorZeroRejected and rules(s)==List(82) and s.work==0)
 run framedReturnExhaustionTest=initCase(E::framedReturnZero).then(group(4,false))
  .expect(s.current==E::framedReturnExhausted and rules(s)==List(84) and s.current.frames==E::framedReturnZero.frames)
 run pushCapTest=initCase(E::pushCap).then(group(1,false))
  .expect(s.current==E::pushError and s.work==1 and rules(s)==List(140))
  .then(group(1,false)).expect(s.current==E::pushRejected and rules(s)==List(140,82) and s.work==1)
 run zeroPendingBudgetTest=initCase(E::n01State0).then(zeroBudget)
  .expect(s.current==E::n01State0 and s.observer.stop==BudgetStop and s.events.length()==0 and s.observer.count==0)
 run zeroTerminalBudgetTest=initCase(E::returnZeroDone).then(zeroBudget)
  .expect(s.current==E::returnZeroDone and s.observer.stop==BudgetStop and s.events.length()==0 and s.observer.count==0)
 run terminalObserverOrderTest=initCase(E::returnZero).then(group(1,true))
  .expect(s.current==E::returnZeroDone and s.observer.stop==TerminalStop and s.observer.count==1 and s.events.length()==1)
 run nonterminalObserverOrderTest=initCase(E::n01State0).then(observerBreak).then(observeEvent)
  .expect(s.current==E::n01State1 and s.observer.stop==ObserverStop and s.observer.count==1)
 run terminalNoNextTest=initCase(E::n01State0).then(group(4,false))
  .then(startCall(1,false)).then(probeNoNext)
  .expect(s.current==E::n01Published and s.events.length()==2 and s.work==1 and s.observer.stop==TerminalStop)
  .then(startCall(2,true)).then(probeNoNext)
  .expect(s.current==E::n01Published and s.events.length()==2 and s.work==1 and s.observer.stop==TerminalStop and s.observer.count==0 and allInvariant)
 run pendingPublicationCutTest=initCase(E::n01State0).then(group(1,false))
  .then(cut(1)).then(projectBoundary).then(resume)
  .expect(s.current==E::n01State1 and s.work==1 and s.current.remaining==0 and s.projection==ScalarProjected)
  .then(group(1,false)).expect(s.current==E::n01Published and rules(s)==List(102,80) and allInvariant)
 run callerEnvironmentCutTest=initCase(E::n05State0).then(group(4,false)).then(group(4,false))
  .expect(s.current==E::n05State8)
  .then(cut(1)).then(projectBoundary).then(resume)
  .expect(s.current==E::n05State8 and s.current.frames==E::n05State8.frames and s.current.environment==E::n05State8.environment)
  .then(5.reps(i=>group(4,false))).expect(s.current==E::n05Published and s.work==27 and allInvariant)
 run lexicalShadowCutTest=initCase(E::n04State0).then(group(4,false)).then(group(4,false)).then(group(2,false))
  .expect(s.current==E::n04State10)
  .then(cut(1)).then(projectBoundary).then(resume)
  .expect(s.current==E::n04State10)
  .then(group(4,false)).then(group(4,false)).expect(s.current==E::n04Published and allInvariant)
 run scalarProjectionSuccessTest=initCase(E::returnZeroDone).then(cut(1)).then(projectBoundary)
  .expect(s.projection==ScalarProjected and s.rawImage=={available:true,space:"Const",index:0,constants:List(7),data:List(),activeData:0} and s.canonicalImage=={available:true,space:"Data",index:0,constants:List(7),data:List(7),activeData:1} and s.current==E::returnZeroDone and s.events.length()==0 and s.work==0 and allInvariant)
 run scalarProjectionRefusalTest=initCase(E::returnZeroDone).then(cut(0)).then(projectBoundary)
  .expect(s.projection==ProjectionCapacityRefused and s.projectionLocation=="Data[0]" and s.current==E::returnZeroDone and s.current.outcome!=A::Exhausted and s.canonicalImage==noImage and s.work==0 and allInvariant)
 run groupedVsSingleTest=initCase(E::n02State0).then(8.reps(i=>group(1,false)))
  .expect(snapshots(s)==E::n02States and s.events==takeGroup(takeGroup(initialState(E::n02State0),4,false),4,false).events and s.work==7)
 run financialCutFailureTest=initCase(E::n05State0)
  .expect(s.finance==FE::financeInitial)
  .then(cut(1)).then(projectBoundary).then(resume).expect(s.finance==FE::financeInitial)
  .then(sourcePrefixObserved).expect(s.finance==FE::financePaid)
  .then(cut(1)).then(projectBoundary).then(resume).expect(s.finance==FE::financePaid)
  .then(tentativeObserved).expect(s.finance==FE::financeTentative)
  .then(cut(1)).then(projectBoundary).then(resume).expect(s.finance==FE::financeTentative)
  .then(localSegmentFailed).expect(s.finance==FE::financeFailed and s.current==E::n05State0 and s.events.length()==0 and s.work==0 and allInvariant)
 run wrongOutputSensitivityTest=initCase(E::n01State0).then(group(4,false))
  .expect(and{
    not(invariant({...s,work:2})),
    not(invariant({...s,events:s.events.append({before:s.current,after:s.current,rule:0,charge:0})})),
    not(invariant({...s,current:{...s.current,remaining:1}})),
    not(invariant({...s,events:s.events.replaceAt(1,{...s.events.nth(1),charge:1})})),
  })
 run savedEnvironmentSensitivityTest=initCase(E::n05State0).then(group(4,false)).then(group(4,false)).then(cut(1)).then(projectBoundary)
  .expect(not(invariant({...s,current:{...s.current,frames:List()}})))
 run financialWrongCutSensitivityTest=initCase(E::n05State0).then(sourcePrefixObserved).then(tentativeObserved).then(cut(1))
  .expect(and{
    not(invariant({...s,finance:{...s.finance,committed:{...s.finance.committed,reserve:62}}})),
    not(invariant({...s,finance:{...s.finance,committed:{...s.finance.committed,dutyPresent:false,residualDuty:0}}})),
    not(invariant({...s,finance:{...s.finance,tentative:{reserveDelta:0,liabilityDelta:0,effects:List()}}})),
  })

 run n01AllFuelPrefixesTest=29.reps(fuel=>{
  val expected=PE::expectedPrefix(E::n01States,fuel)
  initCase(expected.head()).then(8.reps(i=>group(4,false))).expect(snapshots(s)==expected and allInvariant)
 })

 run n02AllFuelPrefixesTest=29.reps(fuel=>{
  val expected=PE::expectedPrefix(E::n02States,fuel)
  initCase(expected.head()).then(8.reps(i=>group(4,false))).expect(snapshots(s)==expected and allInvariant)
 })

 run n04AllFuelPrefixesTest=29.reps(fuel=>{
  val expected=PE::expectedPrefix(E::n04States,fuel)
  initCase(expected.head()).then(8.reps(i=>group(4,false))).expect(snapshots(s)==expected and allInvariant)
 })

 run n05AllFuelPrefixesTest=29.reps(fuel=>{
  val expected=PE::expectedPrefix(E::n05States,fuel)
  initCase(expected.head()).then(8.reps(i=>group(4,false))).expect(snapshots(s)==expected and allInvariant)
 })
 run pendingProjectionRefusalTest=initCase(E::n01State0).then(group(1,false)).then(cut(0)).then(projectBoundary)
  .expect(s.projection==ProjectionCapacityRefused and s.projectionLocation=="Data[0]" and s.current==E::n01State1 and s.current.outcome==A::Pending and s.events.length()==1 and s.work==1 and allInvariant)
 run n02BudgetSevenBoundaryTest=initCase(E::n02State0).then(group(7,false))
  .expect(s.current==E::n02State7 and s.events.length()==7 and s.observer.stop==BudgetStop and s.work==7)
  .then(group(1,false)).expect(s.current==E::n02Published and s.observer.stop==TerminalStop and s.events.length()==8 and s.work==7 and allInvariant)
 run wrongSuccessorValueSensitivityTest=initCase(E::n01State0).then(group(4,false))
  .expect({
   val wrongReturn={...s.events.nth(0).after,control:A::Return(A::UInt(0))}
   val wrongDone={...s.current,outcome:A::Done(A::UInt(0))}
   val wrongEvents=List({...s.events.nth(0),after:wrongReturn},{...s.events.nth(1),before:wrongReturn,after:wrongDone})
   val corrupted={...s,current:wrongDone,events:wrongEvents}
   and{snapshots(s)==E::n01States,invariant(corrupted),snapshots(corrupted)!=E::n01States}
  })
}
