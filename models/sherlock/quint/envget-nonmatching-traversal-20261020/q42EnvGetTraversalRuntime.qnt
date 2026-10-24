module q42EnvGetTraversalRuntime {
  import q42EnvGetTraversalTypes.* from "./q42EnvGetTraversalTypes"
  import q42EnvGetTraversalRelation.* from "./q42EnvGetTraversalRelation"
  import q42EnvGetTraversalInputs.* from "./q42EnvGetTraversalInputs"
  // Runtime depends only on before-inputs; manual expected DATA is test-only.
  var machine:Q42Trace
  action init:bool = machine' = q42TraceStart(q42OneHopBefore)
  action step:bool = all {
    not(machine.finished),machine.within_slice,
    machine' = q42AppendOne(machine),
  }
  val completeHostInputRetained:bool = machine.hosts.nth(0) == q42OneHopBefore
  val normalOwnerWorkPreserved:bool = q42TraceOwnerLaws(machine)
  val completeTraversalWitness:bool = machine.finished and machine.within_slice
    and machine.hosts.length() == 4 and machine.events.length() == 3
}
