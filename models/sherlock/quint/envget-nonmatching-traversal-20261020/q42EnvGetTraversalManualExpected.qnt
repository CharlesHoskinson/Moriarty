module q42EnvGetTraversalManualExpected {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import q42EnvGetTraversalTypes.* from "./q42EnvGetTraversalTypes"
  import q42EnvGetTraversalInputs.* from "./q42EnvGetTraversalInputs"
  // SAME-AUTHOR manual data frozen before new producer; no producer function import.
  pure val q42OneAfter1:C6WholeHost = {...q42OneHopBefore,image:{...q42OneHopBefore.image,remaining_work:90,consumed_work:38,helper_work:12,helper_visits:10,helper:C6HSomeHelper(C6HEnvGet({common:{site:C6HSiteTagEvalVar,origin_reference_cursor:13,parent_resume:C6HNoIndex},binder:0,cursor:C6HSomeIndex(0),found:C6HNoRef,status:C6HTriStatusPending})),result:C6HNoResult}}
  pure val q42OneAfter2:C6WholeHost = {...q42OneHopBefore,image:{...q42OneHopBefore.image,remaining_work:89,consumed_work:39,helper_work:13,helper_visits:11,helper:C6HSomeHelper(C6HEnvGet({common:{site:C6HSiteTagEvalVar,origin_reference_cursor:13,parent_resume:C6HNoIndex},binder:0,cursor:C6HSomeIndex(0),found:C6HSomeRef(C6Const(0)),status:C6HTriStatusPositive})),result:C6HNoResult}}
  pure val q42OneAfter3:C6WholeHost = {...q42OneHopBefore,image:{...q42OneHopBefore.image,remaining_work:88,consumed_work:40,helper_work:14,helper_visits:11,helper:C6HNoHelper,result:C6HSomeResult(C6HResultEnvFound({value:C6Const(0)}))}}
  pure val q42OneExpected:Q42Trace = {hosts:List(q42OneHopBefore,q42OneAfter1,q42OneAfter2,q42OneAfter3),events:List({step_id:{run_id:q42OneHopBefore.image.run_id,ordinal:37},rule_id:"BHelperEnvGetCell",event_kind:"HelperVisit",
      site:Q41SomeText("EvalVar"),helper_kind:Q41SomeText("EnvGet"),disposition:"Applied",
      work_delta:1,helper_work_delta:1,helper_visits_delta:1,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:2,logical_depth_after:2,task_cursor_before:0,task_cursor_after:0,
      helper_phase_before:Q41SomePhase(Q41Tri(C6HTriStatusPending)),helper_phase_after:Q41SomePhase(Q41Tri(C6HTriStatusPending)),
      helper_cursor_before:Q41SomeInt(2),helper_cursor_after:Q41SomeInt(0),result_tag:Q41NoText,
      origin_reference_cursor:Q41SomeInt(13),source_expr_id:4,
      source_occurrence:Q41Occurrence({owner_is_function:false,function:0,ast_path:List(1,1)}),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"},{step_id:{run_id:q42OneHopBefore.image.run_id,ordinal:38},rule_id:"BHelperEnvGetCell",event_kind:"HelperVisit",
      site:Q41SomeText("EvalVar"),helper_kind:Q41SomeText("EnvGet"),disposition:"Applied",
      work_delta:1,helper_work_delta:1,helper_visits_delta:1,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:2,logical_depth_after:2,task_cursor_before:0,task_cursor_after:0,
      helper_phase_before:Q41SomePhase(Q41Tri(C6HTriStatusPending)),helper_phase_after:Q41SomePhase(Q41Tri(C6HTriStatusPositive)),
      helper_cursor_before:Q41SomeInt(0),helper_cursor_after:Q41SomeInt(0),result_tag:Q41NoText,
      origin_reference_cursor:Q41SomeInt(13),source_expr_id:4,
      source_occurrence:Q41Occurrence({owner_is_function:false,function:0,ast_path:List(1,1)}),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"},{step_id:{run_id:q42OneHopBefore.image.run_id,ordinal:39},rule_id:"BHelperEnvGetExit",event_kind:"HelperExit",
      site:Q41SomeText("EvalVar"),helper_kind:Q41SomeText("EnvGet"),disposition:"Applied",
      work_delta:1,helper_work_delta:1,helper_visits_delta:0,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:2,logical_depth_after:2,task_cursor_before:0,task_cursor_after:0,
      helper_phase_before:Q41SomePhase(Q41Tri(C6HTriStatusPositive)),helper_phase_after:Q41NoPhase,
      helper_cursor_before:Q41SomeInt(0),helper_cursor_after:Q41NoInt,result_tag:Q41SomeText("EnvFound"),
      origin_reference_cursor:Q41SomeInt(13),source_expr_id:4,
      source_occurrence:Q41Occurrence({owner_is_function:false,function:0,ast_path:List(1,1)}),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"}),finished:true,within_slice:true}
  pure val q42TwoAfter1:C6WholeHost = {...q42TwoHopBefore,image:{...q42TwoHopBefore.image,remaining_work:86,consumed_work:42,helper_work:18,helper_visits:15,helper:C6HSomeHelper(C6HEnvGet({common:{site:C6HSiteTagEvalVar,origin_reference_cursor:13,parent_resume:C6HNoIndex},binder:0,cursor:C6HSomeIndex(1),found:C6HNoRef,status:C6HTriStatusPending})),result:C6HNoResult}}
  pure val q42TwoAfter2:C6WholeHost = {...q42TwoHopBefore,image:{...q42TwoHopBefore.image,remaining_work:85,consumed_work:43,helper_work:19,helper_visits:16,helper:C6HSomeHelper(C6HEnvGet({common:{site:C6HSiteTagEvalVar,origin_reference_cursor:13,parent_resume:C6HNoIndex},binder:0,cursor:C6HSomeIndex(0),found:C6HNoRef,status:C6HTriStatusPending})),result:C6HNoResult}}
  pure val q42TwoAfter3:C6WholeHost = {...q42TwoHopBefore,image:{...q42TwoHopBefore.image,remaining_work:84,consumed_work:44,helper_work:20,helper_visits:17,helper:C6HSomeHelper(C6HEnvGet({common:{site:C6HSiteTagEvalVar,origin_reference_cursor:13,parent_resume:C6HNoIndex},binder:0,cursor:C6HSomeIndex(0),found:C6HSomeRef(C6Const(0)),status:C6HTriStatusPositive})),result:C6HNoResult}}
  pure val q42TwoAfter4:C6WholeHost = {...q42TwoHopBefore,image:{...q42TwoHopBefore.image,remaining_work:83,consumed_work:45,helper_work:21,helper_visits:17,helper:C6HNoHelper,result:C6HSomeResult(C6HResultEnvFound({value:C6Const(0)}))}}
  pure val q42TwoExpected:Q42Trace = {hosts:List(q42TwoHopBefore,q42TwoAfter1,q42TwoAfter2,q42TwoAfter3,q42TwoAfter4),events:List({step_id:{run_id:q42TwoHopBefore.image.run_id,ordinal:41},rule_id:"BHelperEnvGetCell",event_kind:"HelperVisit",
      site:Q41SomeText("EvalVar"),helper_kind:Q41SomeText("EnvGet"),disposition:"Applied",
      work_delta:1,helper_work_delta:1,helper_visits_delta:1,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:2,logical_depth_after:2,task_cursor_before:0,task_cursor_after:0,
      helper_phase_before:Q41SomePhase(Q41Tri(C6HTriStatusPending)),helper_phase_after:Q41SomePhase(Q41Tri(C6HTriStatusPending)),
      helper_cursor_before:Q41SomeInt(3),helper_cursor_after:Q41SomeInt(1),result_tag:Q41NoText,
      origin_reference_cursor:Q41SomeInt(13),source_expr_id:4,
      source_occurrence:Q41Occurrence({owner_is_function:false,function:0,ast_path:List(1,1)}),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"},{step_id:{run_id:q42TwoHopBefore.image.run_id,ordinal:42},rule_id:"BHelperEnvGetCell",event_kind:"HelperVisit",
      site:Q41SomeText("EvalVar"),helper_kind:Q41SomeText("EnvGet"),disposition:"Applied",
      work_delta:1,helper_work_delta:1,helper_visits_delta:1,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:2,logical_depth_after:2,task_cursor_before:0,task_cursor_after:0,
      helper_phase_before:Q41SomePhase(Q41Tri(C6HTriStatusPending)),helper_phase_after:Q41SomePhase(Q41Tri(C6HTriStatusPending)),
      helper_cursor_before:Q41SomeInt(1),helper_cursor_after:Q41SomeInt(0),result_tag:Q41NoText,
      origin_reference_cursor:Q41SomeInt(13),source_expr_id:4,
      source_occurrence:Q41Occurrence({owner_is_function:false,function:0,ast_path:List(1,1)}),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"},{step_id:{run_id:q42TwoHopBefore.image.run_id,ordinal:43},rule_id:"BHelperEnvGetCell",event_kind:"HelperVisit",
      site:Q41SomeText("EvalVar"),helper_kind:Q41SomeText("EnvGet"),disposition:"Applied",
      work_delta:1,helper_work_delta:1,helper_visits_delta:1,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:2,logical_depth_after:2,task_cursor_before:0,task_cursor_after:0,
      helper_phase_before:Q41SomePhase(Q41Tri(C6HTriStatusPending)),helper_phase_after:Q41SomePhase(Q41Tri(C6HTriStatusPositive)),
      helper_cursor_before:Q41SomeInt(0),helper_cursor_after:Q41SomeInt(0),result_tag:Q41NoText,
      origin_reference_cursor:Q41SomeInt(13),source_expr_id:4,
      source_occurrence:Q41Occurrence({owner_is_function:false,function:0,ast_path:List(1,1)}),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"},{step_id:{run_id:q42TwoHopBefore.image.run_id,ordinal:44},rule_id:"BHelperEnvGetExit",event_kind:"HelperExit",
      site:Q41SomeText("EvalVar"),helper_kind:Q41SomeText("EnvGet"),disposition:"Applied",
      work_delta:1,helper_work_delta:1,helper_visits_delta:0,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:2,logical_depth_after:2,task_cursor_before:0,task_cursor_after:0,
      helper_phase_before:Q41SomePhase(Q41Tri(C6HTriStatusPositive)),helper_phase_after:Q41NoPhase,
      helper_cursor_before:Q41SomeInt(0),helper_cursor_after:Q41NoInt,result_tag:Q41SomeText("EnvFound"),
      origin_reference_cursor:Q41SomeInt(13),source_expr_id:4,
      source_occurrence:Q41Occurrence({owner_is_function:false,function:0,ast_path:List(1,1)}),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"}),finished:true,within_slice:true}
}
