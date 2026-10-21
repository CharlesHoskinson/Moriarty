module nominalPublicSourceRuntime {
  import nominalPublicSimulation.* from "./nominalPublicSimulation"
  import nominalPublicSourceInputs.* from "./nominalPublicSourceInputs"
  action init:bool = initProgram(q10MirrorProgram,256)
  val mirroredNominalPublished:bool = match machine.outcome {
    | Done(v) => rootType(v) == 40 and machine.constructorNodes == 11
      and machine.selectedArms == List(1,1,0,0,0)
    | _ => false }
}
