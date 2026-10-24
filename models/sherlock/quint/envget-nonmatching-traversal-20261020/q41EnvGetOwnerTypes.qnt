module q41EnvGetOwnerTypes {
  // DATA-only declaration interface. No producer/selection/step/charges/values.
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  type Q41OptText = Q41NoText | Q41SomeText(str)
  type Q41OptInt = Q41NoInt | Q41SomeInt(int)
  type Q41Phase = Q41Scalar(C6HScalarPhase) | Q41Tri(C6HTriStatus)
    | Q41Bind(C6HBindPhase) | Q41Append(C6HAppendPhase) | Q41Match(C6HMatchPhase)
    | Q41Build(C6HBuildPhase) | Q41Plan(C6HPlanPhase)
  type Q41OptPhase = Q41NoPhase | Q41SomePhase(Q41Phase)
  type Q41Locator = Q41Occurrence({owner_is_function:bool,function:int,ast_path:List[int]})
    | Q41Table({table:str,row:int}) | Q41Field({name:str})
  type Q41ExpandedStepId = {run_id:C6MRunId,ordinal:int}
  type Q41AllocDeltas = {data_nodes:int,data_fields:int,environment_cells:int}
  type Q41StepEvent = {
    step_id:Q41ExpandedStepId,rule_id:str,event_kind:str,site:Q41OptText,
    helper_kind:Q41OptText,disposition:str,work_delta:int,helper_work_delta:int,
    helper_visits_delta:int,reference_cursor_delta:int,alloc_deltas:Q41AllocDeltas,
    value_slot_writes:int,logical_depth_before:int,logical_depth_after:int,
    task_cursor_before:int,task_cursor_after:int,helper_phase_before:Q41OptPhase,
    helper_phase_after:Q41OptPhase,helper_cursor_before:Q41OptInt,
    helper_cursor_after:Q41OptInt,result_tag:Q41OptText,origin_reference_cursor:Q41OptInt,
    source_expr_id:int,source_occurrence:Q41Locator,exhausted_kind:Q41OptText,
    outcome_tag:str,outcome_payload:C6Outcome,boundary:str,
  }
  type Q41Selected = {rule_id:str,event_kind:str,site:Q41OptText,
    helper_kind:Q41OptText,helper_eligible:bool,helper_visits:int}
  type Q41Window = {before:C6WholeHost,after_cell:C6WholeHost,after_exit:C6WholeHost,
    events:List[Q41StepEvent]}
  type Q41ExpectedRoot = {schema:str,profile:str,case_id:str,window:Q41Window}
}
