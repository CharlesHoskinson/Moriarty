module q42EnvGetTraversalRelation {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import q41EnvGetOwnerRelation.* from "./q41EnvGetOwnerRelation"
  import q42EnvGetTraversalTypes.* from "./q42EnvGetTraversalTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  // Normal positive correspondence is conditional on the existing complete owner
  // read/validation/physical/arithmetic/resource premises. This is no admission
  // or refusal policy. OutsidePositiveSlice remains a model-domain marker only.
  pure def q42Previous(previousArg:C6Env):C6HIndex = match previousArg {
    | C6EnvNone => C6HNoIndex
    | C6EnvSome(indexArg) => C6HSomeIndex(indexArg)
  }
  pure def q42NormalStep(pre:C6WholeHost):Q41NormalStep =
    match pre.image.helper {
      | C6HNoHelper => Q41OutsidePositiveSlice
      | C6HSomeHelper(helperArg) => match helperArg {
        | C6HEnvGet(getArg) => {
          if(getArg.status == C6HTriStatusPending) {
            match getArg.cursor {
              | C6HNoIndex => Q41OutsidePositiveSlice
              | C6HSomeIndex(indexArg) => {
                val cellArg = pre.image.env.nth(indexArg)
                if(cellArg.binder != getArg.binder) {
                  val selectedArg = q41Selected("BHelperEnvGetCell","HelperVisit",getArg.common.site,1)
                  val chargedArg = q41Charge(pre,selectedArg.helper_visits)
                  // Core env_get_cell nonmatch writes only cursor=cell.previous.
                  // Even None leaves status Pending; Empty is a separate row.
                  val nextCursor = q42Previous(cellArg.previous)
                  val helperAfter = C6HSomeHelper(C6HEnvGet({...getArg,cursor:nextCursor}))
                  val afterArg = {...chargedArg,image:{...chargedArg.image,helper:helperAfter}}
                  val eventArg = q41Event(pre,afterArg,selectedArg,getArg.status,
                    Q41SomePhase(Q41Tri(getArg.status)),getArg.cursor,
                    q41Index(nextCursor),Q41NoText,getArg.common.origin_reference_cursor)
                  Q41Applied({after:afterArg,event:eventArg,selected:selectedArg})
                } else q41NormalStep(pre)
              }
            }
          } else q41NormalStep(pre)
        }
        | _ => Q41OutsidePositiveSlice
      }
    }
  pure def q42TraceStart(pre:C6WholeHost):Q42Trace =
    {hosts:List(pre),events:List(),finished:false,within_slice:true}
  pure def q42AppendOne(traceArg:Q42Trace):Q42Trace =
    if(traceArg.finished or not(traceArg.within_slice)) traceArg
    else match q42NormalStep(traceArg.hosts.nth(traceArg.hosts.length()-1)) {
      | Q41OutsidePositiveSlice => {...traceArg,within_slice:false}
      | Q41Applied(stepArg) => {...traceArg,
        hosts:traceArg.hosts.append(stepArg.after),events:traceArg.events.append(stepArg.event),
        finished:stepArg.selected.rule_id == "BHelperEnvGetExit"}
    }
  // rowBound is a finite observation bound, never an owner fuel/cap mutation.
  // Completed windows retain every host/event and acquire no padding stutters.
  pure def q42ComputeTrace(pre:C6WholeHost,rowBound:int):Q42Trace =
    range(0,rowBound).foldl(q42TraceStart(pre),(traceArg,ignoredIndex) => q42AppendOne(traceArg))
  type Q42Window = {before:C6WholeHost,after_miss:C6WholeHost,after_hit:C6WholeHost,
    after_exit:C6WholeHost,events:List[Q41StepEvent]}
  type Q42WindowProjection = Q42NoThreeEdgeWindow | Q42CompleteWindow(Q42Window)
  // Exact common one-miss window projection: no host/event omitted.
  pure def q42ObserveMissHitExit(pre:C6WholeHost):Q42WindowProjection = {
    val traceArg = q42ComputeTrace(pre,3)
    if(traceArg.finished and traceArg.within_slice and traceArg.hosts.length() == 4
      and traceArg.events.length() == 3)
      Q42CompleteWindow({before:traceArg.hosts.nth(0),after_miss:traceArg.hosts.nth(1),
        after_hit:traceArg.hosts.nth(2),after_exit:traceArg.hosts.nth(3),events:traceArg.events})
    else Q42NoThreeEdgeWindow
  }
  pure def q42PreservedOutsideWrites(pre:C6WholeHost,post:C6WholeHost):bool =
    {...post,image:{...post.image,helper:pre.image.helper,result:pre.image.result,
      remaining_work:pre.image.remaining_work,consumed_work:pre.image.consumed_work,
      helper_work:pre.image.helper_work,helper_visits:pre.image.helper_visits}} == pre
  pure def q42TraceOwnerLaws(traceArg:Q42Trace):bool = and {
    traceArg.hosts.length() == traceArg.events.length()+1,
    range(0,traceArg.events.length()).foldl(true,(okArg,indexArg) => {
      val pre = traceArg.hosts.nth(indexArg)
      val post = traceArg.hosts.nth(indexArg+1)
      val eventArg = traceArg.events.nth(indexArg)
      val isCell = eventArg.rule_id == "BHelperEnvGetCell"
      okArg and q42PreservedOutsideWrites(pre,post)
        and post.image.remaining_work == pre.image.remaining_work-1
        and post.image.consumed_work == pre.image.consumed_work+1
        and post.image.helper_work == pre.image.helper_work+1
        and post.image.helper_visits == pre.image.helper_visits+(if(isCell) 1 else 0)
        and post.image.reference_cursor == pre.image.reference_cursor
        and eventArg.step_id.ordinal == pre.image.consumed_work
        and eventArg.work_delta == 1 and eventArg.helper_work_delta == 1
        and eventArg.helper_visits_delta == (if(isCell) 1 else 0)
        and eventArg.reference_cursor_delta == 0
        and (if(isCell) post.image.result == pre.image.result else true)
    }),
  }
}
