module qpsPrimitiveRuntime {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import qpsPrimitiveTypes.* from "./qpsPrimitiveTypes"
  import qpsPrimitiveInputs.* from "./qpsPrimitiveInputs"
  import qpsPrimitivePredicates.* from "./qpsPrimitivePredicates"
  import qpsPrimitiveRelation.* from "./qpsPrimitiveRelation"
  import qpsPrimitiveTest.* from "./qpsPrimitiveTest"
  var machine:QPSTrace
  action init:bool=all {
    machine'={hosts:List(qpsInput(C6PrimResult({result:C6PrimitiveError(10)}),1,true)),
      events:List(),finished:false},
  }
  action step:bool=all {
    not(machine.finished),qpsCanStep(machine.hosts.nth(machine.hosts.length()-1)),
    {val computed=qpsApply(machine.hosts.nth(machine.hosts.length()-1))
     machine'={hosts:machine.hosts.append(computed.after),events:machine.events.append(computed.event),
       finished:computed.after.image.outcome!=C6Running}},
  }
  val completeErrorPublicationHistory:bool =
    if(machine.events.length()==0) machine.hosts.length()==1 and not(machine.finished)
    else and {machine.hosts.length()==machine.events.length()+1,
      machine.events.length()<=2,
      qpsAfterPredicate(machine.hosts.nth(0),machine.hosts.nth(1),QPSError),
      machine.events.nth(0)==qpsExpectedEvent(machine.hosts.nth(0),QPSError),
      if(machine.events.length()==2) and {
        qpsAfterPredicate(machine.hosts.nth(1),machine.hosts.nth(2),QPSPublishError),
        machine.events.nth(1)==qpsExpectedEvent(machine.hosts.nth(1),QPSPublishError),machine.finished,
      } else not(machine.finished)}
  val primitiveErrorThenFreePublication:bool = machine.finished and machine.hosts.length()==3
    and machine.events.length()==2 and machine.hosts.nth(2).image.outcome==C6Rejected(10)
}
