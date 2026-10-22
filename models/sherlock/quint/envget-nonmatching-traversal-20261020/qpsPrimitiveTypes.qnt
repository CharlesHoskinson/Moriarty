module qpsPrimitiveTypes {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"

  type QPSKind = QPSValue | QPSError | QPSPublishError | QPSFuel | QPSOutside
  type QPSCase = {before:C6WholeHost,expected:QPSKind}
  type QPSStep = {after:C6WholeHost,event:Q41StepEvent,kind:QPSKind}
  type QPSTrace = {hosts:List[C6WholeHost],events:List[Q41StepEvent],finished:bool}
}
