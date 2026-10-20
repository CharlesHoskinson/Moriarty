module q42EnvGetTraversalTypes {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  type Q42Trace = {hosts:List[C6WholeHost],events:List[Q41StepEvent],finished:bool,within_slice:bool}
}
