module qpsPrimitiveRelation {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import qpsPrimitiveTypes.* from "./qpsPrimitiveTypes"

  // Positive, idle helper/result projection only. Classification is NOT a
  // replacement for AB/SI/PS/SC/physical validation or a new refusal policy.
  pure def qpsClassify(pre:C6WholeHost):QPSKind =
    if(pre.image.outcome!=C6Running or pre.image.helper!=C6HNoHelper or pre.image.result!=C6HNoResult)
      QPSOutside
    else match pre.image.control {
      // P1 occurs before fuel-zero P2 and makes no charged-row arithmetic.
      | C6Error(_)=>QPSPublishError
      | _=>if(pre.image.remaining_work==0) match pre.image.control {
          // This is the separate P1 PublishValue row, outside this slice.
          | C6Return(_)=>if(pre.image.frames.length()==0) QPSOutside else QPSFuel
          | _=>QPSFuel }
        else if(pre.image.remaining_work>0) match pre.image.control {
          | C6PrimResult(p)=>match p.result {
            | C6PrimitiveValue(_)=>QPSValue | C6PrimitiveError(_)=>QPSError }
          | _=>QPSOutside }
        else QPSOutside
    }
  pure def qpsCanStep(pre:C6WholeHost):bool=qpsClassify(pre)!=QPSOutside
  pure def qpsApply(pre:C6WholeHost):QPSStep = {
    val kindArg=qpsClassify(pre)
    val applied=kindArg==QPSValue or kindArg==QPSError
    val nextHost:C6WholeHost=if(applied) {
      val nextControl=match pre.image.control {
        | C6PrimResult(p)=>match p.result {
          | C6PrimitiveValue(r)=>C6Return({value:r})
          | C6PrimitiveError(codeArg)=>C6Error({code:codeArg}) }
        | _=>List().head() }
      {...pre,image:{...pre.image,control:nextControl,remaining_work:pre.image.remaining_work-1,
        consumed_work:pre.image.consumed_work+1,reference_cursor:pre.image.reference_cursor+1}}
    } else if(kindArg==QPSPublishError) match pre.image.control {
      | C6Error(p)=>{...pre,image:{...pre.image,outcome:C6Rejected(p.code)}}
      | _=>List().head() }
    else if(kindArg==QPSFuel) {...pre,image:{...pre.image,outcome:C6Exhausted(C6Fuel)}}
    else List().head()
    val located=pre.admission.view.occurrences.nth(pre.image.control_origin)
    val terminalTag=match nextHost.image.outcome {
      | C6Running=>"Running" | C6Rejected(_)=>"Rejected" | C6Exhausted(_)=>"Exhausted"
      | _=>List().head() }
    val rowEvent:Q41StepEvent={
      step_id:{run_id:nextHost.image.run_id,ordinal:pre.image.consumed_work},
      rule_id:(if(kindArg==QPSValue) "KMachinePrimitiveValue" else if(kindArg==QPSError) "KMachinePrimitiveError"
        else if(kindArg==QPSPublishError) "KAdminPublishError" else "KAdminPublishExhausted"),
      event_kind:(if(applied) "LegacyComplete" else "Publish"),
      site:(if(applied) Q41SomeText("Prim") else Q41NoText),helper_kind:Q41NoText,
      disposition:(if(applied) "Applied" else if(kindArg==QPSPublishError) "Published" else "ExhaustedFuelUncharged"),
      work_delta:(if(applied) 1 else 0),helper_work_delta:0,helper_visits_delta:0,
      reference_cursor_delta:(if(applied) 1 else 0),
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:pre.image.frames.length(),logical_depth_after:nextHost.image.frames.length(),
      task_cursor_before:pre.image.tasks.length(),task_cursor_after:nextHost.image.tasks.length(),
      helper_phase_before:Q41NoPhase,helper_phase_after:Q41NoPhase,
      helper_cursor_before:Q41NoInt,helper_cursor_after:Q41NoInt,
      result_tag:Q41NoText,origin_reference_cursor:Q41NoInt,
      source_expr_id:pre.image.control_origin,
      source_occurrence:Q41Occurrence({owner_is_function:located.owner_is_function,function:located.function,
        ast_path:pre.admission.view.occurrence_paths.slice(located.path.start,located.path.start+located.path.len)}),
      exhausted_kind:(if(kindArg==QPSFuel) Q41SomeText("Fuel") else Q41NoText),
      outcome_tag:terminalTag,outcome_payload:nextHost.image.outcome,boundary:(if(applied) "None" else "Terminal")}
    {after:nextHost,event:rowEvent,kind:kindArg}
  }
}
