module c6PositiveStateObservation {
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import c6PositiveHelperCodec.* from "./c6PositiveHelperCodec"
  import c6PositiveStateCodec.* from "./c6PositiveStateCodec"
  pure def hObserveHelper(x:C6HOptHelper):C6HHelperRecordObservation = {
    val l = hEncodeHelper(x)
    {encoded_lanes:l,decoded_helper:hDecodeHelper(l)}
  }
  pure def hObserveResult(x:C6HOptResult):C6HResultRecordObservation = {
    val l = hEncodeResult(x)
    {encoded_lanes:l,decoded_result:hDecodeResult(l)}
  }
  pure def hObserveTask(x:C6HTask):C6HTaskRecordObservation = {
    val l = hEncodeTaskValue(x)
    {encoded_lanes:l,decoded_task:hDecodeTask(l)}
  }
  pure def hObserveTables(x:C6HHelperTables):C6HHelperSectionObservation = {
    val l = hEncodeTables(x)
    val spansArg:List[C6HTaskSpan] = range(0,l.nth(3)).foldl(List(),(acc,i) =>
      acc.append({index:i,start:31+7*i,length:7,end:31+7*(i+1),lanes:l.slice(31+7*i,31+7*(i+1))}))
    {encoded_lanes:l,decoded_tables:hDecodeTables(l),task_spans:spansArg,final_cursor:31+7*l.nth(3)}
  }
  type StateSpans = {cursor:int,spans:List[C6HChildSection]}
  pure def sObserveState(x:C6HState,ctx:C6HContext):C6HStateObservation = {
    val l = sEncodeState(x,ctx)
    val namesArg = List("image","data","helper")
    val initial:StateSpans = {cursor:41,spans:List()}
    val folded = range(0,3).foldl(initial,(acc,i) => {
      val sizeArg = l.nth(5+2*i)
      {cursor:acc.cursor+sizeArg,spans:acc.spans.append({name:namesArg.nth(i),tag:l.nth(4+2*i),
        start:acc.cursor,length:sizeArg,end:acc.cursor+sizeArg,lanes:l.slice(acc.cursor,acc.cursor+sizeArg)})}
    })
    {encoded_lanes:l,decoded_state:sDecodeState(l),retained_context:ctx,capacity_descriptor:l.slice(12,41),
      child_sections:folded.spans,final_cursor:folded.cursor}
  }
  pure def hObserveRoot(x:C6HInputs):C6HExpectedRoot = {
    val hi:List[str] = x.helper_record_cases.foldl(List(),(a,c) => a.append(c.case_id))
    val ri:List[str] = x.result_record_cases.foldl(List(),(a,c) => a.append(c.case_id))
    val ti:List[str] = x.task_record_cases.foldl(List(),(a,c) => a.append(c.case_id))
    val hsi:List[str] = x.helper_section_cases.foldl(List(),(a,c) => a.append(c.case_id))
    val si:List[str] = x.state_cases.foldl(List(),(a,c) => a.append(c.case_id))
    val ho:List[C6HHelperRecordObservation] = x.helper_record_cases.foldl(List(),(a,c) => a.append(hObserveHelper(c.helper)))
    val ro:List[C6HResultRecordObservation] = x.result_record_cases.foldl(List(),(a,c) => a.append(hObserveResult(c.result)))
    val tobs:List[C6HTaskRecordObservation] = x.task_record_cases.foldl(List(),(a,c) => a.append(hObserveTask(c.task)))
    val hso:List[C6HHelperSectionObservation] = x.helper_section_cases.foldl(List(),(a,c) => a.append(hObserveTables(c.tables)))
    val so:List[C6HStateObservation] = x.state_cases.foldl(List(),(a,c) => a.append(sObserveState(c.state,c.context)))
    {schema:"independent-c6-positive-helper-task-state/1",profile:x.profile,
      case_ids:{helper_record_cases:hi,result_record_cases:ri,task_record_cases:ti,helper_section_cases:hsi,state_cases:si},
      helper_record_observations:ho,result_record_observations:ro,task_record_observations:tobs,
      helper_section_observations:hso,state_observations:so}
  }
}
