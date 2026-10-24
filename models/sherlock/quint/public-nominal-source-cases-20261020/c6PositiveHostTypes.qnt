module c6PositiveHostTypes {
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  type C6HostSegBase = {active:C6Active,alloc:C6Alloc,consumed_work:int}
  type C6HostAdmission = {identity:C6MIdentity,bounds:C6MBounds,view:C6PProgram,layout:C6MLayout}
  type C6HostImage = {run_id:C6MRunId,control:C6Control,control_origin:int,env_root:C6Env,stack_limit:int,
    outcome:C6Outcome,initial_fuel:int,remaining_work:int,consumed_work:int,helper_work:int,
    helper_visits:int,reference_cursor:int,alloc:C6Alloc,active:C6Active,helper:C6HOptHelper,
    result:C6HOptResult,seg_base:C6HostSegBase,data:List[C6HDataRow],data_fields:List[C6Ref],
    env:List[C6HEnvCell],frames:List[C6Frame],tasks:List[C6HTask],value_slots:List[C6Ref]}
  type C6WholeHost = {admission:C6HostAdmission,image:C6HostImage}
  type C6WholeHostConfig = {layout:C6MLayout,ext:C6CtxExt,budgets:C6CtxBudgets}
  type C6WholeHostInputCase = {case_id:str,host:C6WholeHost,config:C6WholeHostConfig,limits:C6MHostLimits}
  type C6HostStateSection = {name:str,tag:int,relative_start:int,relative_end:int,
    absolute_start:int,absolute_end:int,length:int,lanes:List[int]}
  type C6HostComponents = {metadata:C6MObservation,program:C6PObservation,
    context:C6CtxObservation,state:C6HStateObservation}
  type C6WholeHostObservation = {encoded_lanes:List[int],decoded_host:C6WholeHost,
    decoded_config:C6WholeHostConfig,retained_limits:C6MHostLimits,state_context:C6HContext,
    host_sections:List[C6HChildSection],state_sections:List[C6HostStateSection],
    components:C6HostComponents,final_cursor:int}
  type C6WholeHostExpectedRoot = {schema:str,profile:str,case_ids:List[str],observations:List[C6WholeHostObservation]}
}
