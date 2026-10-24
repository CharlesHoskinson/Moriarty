module qpsPrimitivePredicates {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import qpsPrimitiveTypes.* from "./qpsPrimitiveTypes"
  // Independent source-first branch labels are supplied by typed input cases.
  // No producer/classifier/after-image imports occur in this property module.
  pure def qpsExpectedControl(pre:C6WholeHost,k:QPSKind):C6Control =
    if(k==QPSValue or k==QPSError) match pre.image.control {
      | C6PrimResult(p)=>match p.result {
        | C6PrimitiveValue(r)=>C6Return({value:r})
        | C6PrimitiveError(codeArg)=>C6Error({code:codeArg}) }
      | _=>List().head() }
    else pre.image.control
  pure def qpsExpectedOutcome(pre:C6WholeHost,k:QPSKind):C6Outcome =
    if(k==QPSPublishError) match pre.image.control {
      | C6Error(p)=>C6Rejected(p.code) | _=>List().head() }
    else if(k==QPSFuel) C6Exhausted(C6Fuel) else C6Running
  pure def qpsAfterPredicate(pre:C6WholeHost,post:C6WholeHost,k:QPSKind):bool = {
    val spent=if(k==QPSValue or k==QPSError) 1 else 0
    and {post.image.control==qpsExpectedControl(pre,k),post.image.outcome==qpsExpectedOutcome(pre,k),
      post.image.remaining_work+spent==pre.image.remaining_work,
      post.image.consumed_work==pre.image.consumed_work+spent,
      post.image.reference_cursor==pre.image.reference_cursor+spent,
      {...post,image:{...post.image,control:pre.image.control,outcome:pre.image.outcome,
        remaining_work:pre.image.remaining_work,consumed_work:pre.image.consumed_work,
        reference_cursor:pre.image.reference_cursor}}==pre}
  }
  pure def qpsExpectedEvent(pre:C6WholeHost,k:QPSKind):Q41StepEvent = {
    val charged=k==QPSValue or k==QPSError
    val located=pre.admission.view.occurrences.nth(pre.image.control_origin)
    {step_id:{run_id:pre.image.run_id,ordinal:pre.image.consumed_work},
      rule_id:(if(k==QPSValue) "KMachinePrimitiveValue" else if(k==QPSError) "KMachinePrimitiveError"
        else if(k==QPSPublishError) "KAdminPublishError" else "KAdminPublishExhausted"),
      event_kind:(if(charged) "LegacyComplete" else "Publish"),
      site:(if(charged) Q41SomeText("Prim") else Q41NoText),helper_kind:Q41NoText,
      disposition:(if(charged) "Applied" else if(k==QPSPublishError) "Published" else "ExhaustedFuelUncharged"),
      work_delta:(if(charged) 1 else 0),helper_work_delta:0,helper_visits_delta:0,
      reference_cursor_delta:(if(charged) 1 else 0),
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:pre.image.frames.length(),logical_depth_after:pre.image.frames.length(),
      task_cursor_before:pre.image.tasks.length(),task_cursor_after:pre.image.tasks.length(),
      helper_phase_before:Q41NoPhase,helper_phase_after:Q41NoPhase,
      helper_cursor_before:Q41NoInt,helper_cursor_after:Q41NoInt,
      result_tag:Q41NoText,origin_reference_cursor:Q41NoInt,source_expr_id:pre.image.control_origin,
      source_occurrence:Q41Occurrence({owner_is_function:located.owner_is_function,function:located.function,
        ast_path:pre.admission.view.occurrence_paths.slice(located.path.start,located.path.start+located.path.len)}),
      exhausted_kind:(if(k==QPSFuel) Q41SomeText("Fuel") else Q41NoText),
      outcome_tag:(if(charged) "Running" else if(k==QPSPublishError) "Rejected" else "Exhausted"),
      outcome_payload:qpsExpectedOutcome(pre,k),boundary:(if(charged) "None" else "Terminal")}
  }
}
