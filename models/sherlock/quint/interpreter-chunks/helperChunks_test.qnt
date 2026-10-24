module helperChunks_test {
 import helperChunks.* from "./helperChunks"
 import bPure as B from "./bPure"
 import aPure as A from "./aPure"
 import bExpectations as E from "./bExpectations"
 import bEventExpectations as EV from "./bEventExpectations"
 run envGetCompleteTest=initCase(5)
  .expect(h.current.machine==E::initialMachine5)
  .then(helperEvent).expect(h.current.machine==E::afterEntry5)
  .then(helperEvent).expect(h.current.machine==E::afterFirstCell5)
  .then(helperEvent).expect(h.current.machine==E::afterFound5)
  .then(helperEvent).expect(h.current.machine==E::afterExit5)
  .then(helperEvent).expect(h.current.machine==E::afterComplete5)
  .then(helperEvent).expect(h.current.machine==E::afterPublish5 and rules(h)==E::rowSequence and List(h.initial).concat(h.records.foldl(List(),(acc,r)=>acc.append(r.after)))==EV::states5 and allInvariant)
 run helperCutTest=initCase(5).then(helperEvent).then(helperEvent)
  .then(cut).expect(h.saved.current.machine==E::afterFirstCell5)
  .then(resume).expect(h.current.machine==E::afterFirstCell5 and h.records.length()==2 and allInvariant)
  .then(4.reps(i=>helperEvent)).expect(h.current.machine==E::afterPublish5)
 run pendingResultCutTest=initCase(5).then(4.reps(i=>helperEvent))
  .expect(h.current.machine==E::afterExit5)
  .then(cut).then(resume).expect(h.current.machine.result==B::EnvFound(0) and h.current.machine.referenceCursor==0 and erased(h)==mappedOrigin)
  .then(helperEvent).expect(h.current.machine==E::afterComplete5 and erased(h)=={...mappedOrigin,control:A::Return(A::UInt(0)),remaining:0})
 run fuelFourVsFiveTest=initCase(4).then(4.reps(i=>helperEvent))
  .expect(h.current.machine.result==B::EnvFound(0) and h.current.machine.referenceCursor==0 and h.current.machine.consumedWork==4)
  .then(helperEvent).expect(h.current.machine.outcome==B::FuelExhausted and h.current.machine.result==B::EnvFound(0) and h.current.machine.helperWork==4 and h.current.machine.referenceCursor==0 and allInvariant)
 run equalFuelDifferentClockTest=initCase(1).then(helperEvent)
  .expect(h.current.machine.helper==B::GetHelper({binder:"x",cursor:B::At(1),found:B::Absent,status:B::Pending,originReferenceCursor:0}) and h.current.machine.referenceCursor==0 and erased(h)==mappedOrigin and A::advance(mappedOrigin).state=={...mappedOrigin,control:A::Return(A::UInt(0)),remaining:0})
  .then(helperEvent).expect(h.current.machine.outcome==B::FuelExhausted and h.current.machine.helperWork==1 and h.current.machine.helperVisits==0)
 run terminalNoNextTest=initCase(5).then(6.reps(i=>helperEvent)).then(probeNoNext)
  .expect(h.current.machine==E::afterPublish5 and h.records.length()==6 and h.probes==1 and h.noNextResponse.lastEvent==B::NoEvent and allInvariant)
 run allFuelDomainTest=7.reps(fuel=>{
  initCase(fuel).then(7.reps(i=>if(canEvent(h))helperEvent else all{h'=h}))
   .expect(allInvariant and h.current.machine.outcome!=B::Running)
 })
 run skipExitSensitivityTest=initCase(5).then(4.reps(i=>helperEvent))
  .expect(and{
   h.current.machine==E::afterExit5,
   h.current.machine!={...E::afterExit5,helper:E::afterFound5.helper},
   rules(h)!=List("b.helper.EnvGet.entry","b.helper.EnvGet.cell","b.helper.EnvGet.cell","k.machine.eval.variable.present"),
   not(invariant({...h,current:{...h.current,machine:{...h.current.machine,referenceCursor:1}}})),
  })
}
