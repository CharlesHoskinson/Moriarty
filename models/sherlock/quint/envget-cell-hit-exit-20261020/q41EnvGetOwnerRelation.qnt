module q41EnvGetOwnerRelation {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  // Genuine normal owner two-row projection, NOT a codec or admission engine.
  // External: real bound/reachable preimage, Running, no P1/P2/P3/P6 refusal,
  // enough normal work/budget, successful reads/validation/arithmetic/physical
  // checks, and active lengths equal full owned Vec lengths. No new guard policy.
  // OutsidePositiveSlice means this model has no row; it is NOT a Core outcome,
  // rejection/refusal, terminal step, admission result or unchanged host result.
  type Q41NormalStep = Q41OutsidePositiveSlice
    | Q41Applied({after:C6WholeHost,event:Q41StepEvent,selected:Q41Selected})
  type Q41NormalWindow = Q41NoWindow | Q41ComputedWindow(Q41Window)
  pure def q41Index(x:C6HIndex):Q41OptInt = match x {
    | C6HNoIndex => Q41NoInt | C6HSomeIndex(i) => Q41SomeInt(i)
  }
  pure def q41SiteText(x:C6HSiteTag):str = match x {
    | C6HSiteTagEvalLiteral => "EvalLiteral"
    | C6HSiteTagEvalVar => "EvalVar"
    | C6HSiteTagEvalLet => "EvalLet"
    | C6HSiteTagEvalBinary => "EvalBinary"
    | C6HSiteTagEvalUnary => "EvalUnary"
    | C6HSiteTagEvalIf => "EvalIf"
    | C6HSiteTagEvalMatch => "EvalMatch"
    | C6HSiteTagEvalCall => "EvalCall"
    | C6HSiteTagPush => "Push"
    | C6HSiteTagRetLet => "RetLet"
    | C6HSiteTagRetLeft => "RetLeft"
    | C6HSiteTagRetRight => "RetRight"
    | C6HSiteTagRetUnary => "RetUnary"
    | C6HSiteTagRetIf => "RetIf"
    | C6HSiteTagRetMatch => "RetMatch"
    | C6HSiteTagRetArgs => "RetArgs"
    | C6HSiteTagRetCall => "RetCall"
    | C6HSiteTagRetScope => "RetScope"
    | C6HSiteTagInvoke => "Invoke"
    | C6HSiteTagEnter => "Enter"
    | C6HSiteTagBound => "Bound"
    | C6HSiteTagCases => "Cases"
    | C6HSiteTagCaseResult => "CaseResult"
    | C6HSiteTagPrim => "Prim"
    | C6HSiteTagEvalConstruct => "EvalConstruct"
    | C6HSiteTagRetCtorFields => "RetCtorFields"
    | C6HSiteTagMaterialize => "Materialize"
  }
  pure def q41Selected(ruleArg:str,kindArg:str,siteArg:C6HSiteTag,visitsArg:int):Q41Selected =
    {rule_id:ruleArg,event_kind:kindArg,site:Q41SomeText(q41SiteText(siteArg)),
      helper_kind:Q41SomeText("EnvGet"),helper_eligible:true,helper_visits:visitsArg}
  pure def q41Occurrence(pre:C6WholeHost):Q41Locator = {
    val occurrenceArg = pre.admission.view.occurrences.nth(pre.image.control_origin)
    val pathArg = pre.admission.view.occurrence_paths.slice(occurrenceArg.path.start,
      occurrenceArg.path.start+occurrenceArg.path.len)
    Q41Occurrence({owner_is_function:occurrenceArg.owner_is_function,
      function:occurrenceArg.function,ast_path:pathArg})
  }
  // P4 charge + P7 visit count. Ref cursor stays because neither row is LegacyComplete.
  // All other image/admission/Program/layout fields are retained by record spread.
  pure def q41Charge(pre:C6WholeHost,visitsArg:int):C6WholeHost =
    {...pre,image:{...pre.image,remaining_work:pre.image.remaining_work-1,
      consumed_work:pre.image.consumed_work+1,helper_work:pre.image.helper_work+1,
      helper_visits:pre.image.helper_visits+visitsArg}}
  pure def q41Event(pre:C6WholeHost,post:C6WholeHost,sel:Q41Selected,
      phaseBefore:C6HTriStatus,phaseAfter:Q41OptPhase,cursorBefore:C6HIndex,
      cursorAfter:Q41OptInt,resultTag:Q41OptText,originArg:int):Q41StepEvent =
    {step_id:{run_id:post.image.run_id,ordinal:pre.image.consumed_work},
      rule_id:sel.rule_id,event_kind:sel.event_kind,site:sel.site,helper_kind:sel.helper_kind,
      disposition:"Applied",work_delta:1,helper_work_delta:1,
      helper_visits_delta:sel.helper_visits,reference_cursor_delta:0,
      alloc_deltas:{data_nodes:0,data_fields:0,environment_cells:0},value_slot_writes:0,
      logical_depth_before:pre.image.frames.length(),logical_depth_after:post.image.frames.length(),
      task_cursor_before:pre.image.tasks.length(),task_cursor_after:post.image.tasks.length(),
      helper_phase_before:Q41SomePhase(Q41Tri(phaseBefore)),helper_phase_after:phaseAfter,
      helper_cursor_before:q41Index(cursorBefore),helper_cursor_after:cursorAfter,
      result_tag:resultTag,origin_reference_cursor:Q41SomeInt(originArg),
      source_expr_id:pre.image.control_origin,source_occurrence:q41Occurrence(pre),
      exhausted_kind:Q41NoText,outcome_tag:"Running",outcome_payload:C6Running,boundary:"None"}
  pure def q41NormalStep(pre:C6WholeHost):Q41NormalStep =
    match pre.image.helper {
      | C6HNoHelper => Q41OutsidePositiveSlice
      | C6HSomeHelper(helperArg) => match helperArg {
        | C6HEnvGet(getArg) => {
          if(getArg.status == C6HTriStatusPending) {
            match getArg.cursor {
              | C6HNoIndex => Q41OutsidePositiveSlice
              | C6HSomeIndex(indexArg) => {
                val cellArg = pre.image.env.nth(indexArg)
                if(cellArg.binder == getArg.binder) {
                  val selectedArg = q41Selected("BHelperEnvGetCell","HelperVisit",getArg.common.site,1)
                  val chargedArg = q41Charge(pre,selectedArg.helper_visits)
                  val helperAfter = C6HSomeHelper(C6HEnvGet({...getArg,
                    found:C6HSomeRef(cellArg.value),status:C6HTriStatusPositive}))
                  val afterArg = {...chargedArg,image:{...chargedArg.image,helper:helperAfter}}
                  val eventArg = q41Event(pre,afterArg,selectedArg,getArg.status,
                    Q41SomePhase(Q41Tri(C6HTriStatusPositive)),getArg.cursor,
                    q41Index(getArg.cursor),Q41NoText,getArg.common.origin_reference_cursor)
                  Q41Applied({after:afterArg,event:eventArg,selected:selectedArg})
                } else Q41OutsidePositiveSlice
              }
            }
          } else if(getArg.status == C6HTriStatusPositive) {
            match getArg.found {
              | C6HNoRef => Q41OutsidePositiveSlice
              | C6HSomeRef(valueArg) => {
                val selectedArg = q41Selected("BHelperEnvGetExit","HelperExit",getArg.common.site,0)
                val chargedArg = q41Charge(pre,selectedArg.helper_visits)
                val afterArg = {...chargedArg,image:{...chargedArg.image,helper:C6HNoHelper,
                  result:C6HSomeResult(C6HResultEnvFound({value:valueArg}))}}
                val eventArg = q41Event(pre,afterArg,selectedArg,getArg.status,Q41NoPhase,
                  getArg.cursor,Q41NoInt,Q41SomeText("EnvFound"),getArg.common.origin_reference_cursor)
                Q41Applied({after:afterArg,event:eventArg,selected:selectedArg})
              }
            }
          } else Q41OutsidePositiveSlice
        }
        | _ => Q41OutsidePositiveSlice
      }
    }
  pure def q41ComputeWindow(pre:C6WholeHost):Q41NormalWindow =
    match q41NormalStep(pre) {
      | Q41OutsidePositiveSlice => Q41NoWindow
      | Q41Applied(firstArg) =>
        if(firstArg.selected.rule_id == "BHelperEnvGetCell") {
          match q41NormalStep(firstArg.after) {
            | Q41OutsidePositiveSlice => Q41NoWindow
            | Q41Applied(secondArg) =>
              if(secondArg.selected.rule_id == "BHelperEnvGetExit")
                Q41ComputedWindow({before:pre,after_cell:firstArg.after,
                  after_exit:secondArg.after,events:List(firstArg.event,secondArg.event)})
              else Q41NoWindow
          }
        } else Q41NoWindow
    }
}
