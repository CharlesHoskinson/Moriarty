module c6PositiveHostObservation {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveHostCodec.* from "./c6PositiveHostCodec"
  import c6PositiveMetaObservation.* from "./c6PositiveMetaObservation"
  import c6PositiveProgramObservation.* from "./c6PositiveProgramObservation"
  import c6PositiveContext.* from "./c6PositiveContext"
  import c6PositiveStateObservation.* from "./c6PositiveStateObservation"
  pure def c6HostSections(l:List[int]):List[C6HChildSection] = {
    val at = c6HostOffsets(l)
    val namesArg = List("metadata","program","context","state")
    range(0,4).foldl(List(),(acc,i) => acc.append({name:namesArg.nth(i),tag:l.nth(5+2*i),
      start:at.nth(i),length:l.nth(6+2*i),end:at.nth(i+1),lanes:l.slice(at.nth(i),at.nth(i+1))}))
  }
  pure def c6HostStateSections(parts:List[C6HChildSection],baseArg:int):List[C6HostStateSection] =
    parts.foldl(List(),(acc,p) => acc.append({name:p.name,tag:p.tag,relative_start:p.start,
      relative_end:p.end,absolute_start:baseArg+p.start,absolute_end:baseArg+p.end,
      length:p.length,lanes:p.lanes}))
  pure def c6HostObserve(c:C6WholeHostInputCase):C6WholeHostObservation = {
    val ctxInput = c6HostContextInput(c.config,c.limits)
    val contextArg = c6CtxObserve(ctxInput)
    val stateContext = c6HostFlatContext(contextArg.state_context)
    val metaArg = c6MObserve(c6HostMetadata(c.host))
    val progArg = c6PObserve(c.host.admission.view)
    val stateArg = sObserveState(c6HostOwnedState(c.host.image),stateContext)
    val lanesArg = c6HostAssemble(metaArg.encoded_lanes,progArg.encoded_lanes,
      contextArg.encoded_lanes,stateArg.encoded_lanes)
    val decodedArg = c6HostDecodePositive(lanesArg,c.config)
    val at = c6HostOffsets(lanesArg)
    {encoded_lanes:lanesArg,decoded_host:decodedArg.host,decoded_config:decodedArg.config,
      retained_limits:c.limits,state_context:stateContext,host_sections:c6HostSections(lanesArg),
      state_sections:c6HostStateSections(stateArg.child_sections,at.nth(3)),
      components:{metadata:metaArg,program:progArg,context:contextArg,state:stateArg},final_cursor:at.nth(4)}
  }
  pure def c6HostObserveRoot(casesArg:List[C6WholeHostInputCase]):C6WholeHostExpectedRoot =
    {schema:"independent-c6-whole-host-positive/1",profile:"sherlock-c6-whole-host-positive/1",
      case_ids:casesArg.foldl(List(),(acc,c) => acc.append(c.case_id)),
      observations:casesArg.foldl(List(),(acc,c) => acc.append(c6HostObserve(c)))}
  pure def c6HostCompositionLaws(c:C6WholeHostInputCase):bool = {
    val obs = c6HostObserve(c)
    val l = obs.encoded_lanes
    val at = c6HostOffsets(l)
    and {
      obs.decoded_host == c.host,obs.decoded_config == c.config,
      c6HostDecodePositive(l,c.config).decoded_context == {ext:c.config.ext,budgets:c.config.budgets},
      c6HostEncode(obs.decoded_host,obs.decoded_config,c.limits) == l,
      c6HostEncode(c.host,c.config,c.limits) == l,
      obs.retained_limits == c.limits,obs.final_cursor == l.length(),
      obs.state_context == c6HostStateContext(c.config,c.limits),
      obs.components.context.decoded_context == {ext:c.config.ext,budgets:c.config.budgets},
      obs.components.metadata.decoded_metadata == c6HostMetadata(c.host),
      obs.components.program.decoded_program == c.host.admission.view,
      obs.components.state.decoded_state == c6HostOwnedState(c.host.image),
      obs.components.state.retained_context == obs.state_context,
      obs.host_sections.length() == 4,obs.state_sections.length() == 3,
      at.nth(0) == 14,at.nth(4) == l.length(),
      range(0,4).foldl(true,(okArg,i) => {
        val p = obs.host_sections.nth(i)
        okArg and p.start == at.nth(i) and p.end == at.nth(i+1)
          and p.length == p.lanes.length() and p.end == p.start+p.length
          and p.lanes == l.slice(p.start,p.end)
      }),
      obs.host_sections.nth(0).lanes == obs.components.metadata.encoded_lanes,
      obs.host_sections.nth(1).lanes == obs.components.program.encoded_lanes,
      obs.host_sections.nth(2).lanes == obs.components.context.encoded_lanes,
      obs.host_sections.nth(3).lanes == obs.components.state.encoded_lanes,
      obs.components.metadata.final_cursor == obs.components.metadata.encoded_lanes.length(),
      obs.components.program.final_cursor == obs.components.program.encoded_lanes.length(),
      obs.components.state.final_cursor == obs.components.state.encoded_lanes.length(),
      obs.state_sections.nth(0).relative_start == 41,
      obs.state_sections.nth(2).absolute_end == l.length(),
      range(0,3).foldl(true,(okArg,i) => {
        val p = obs.state_sections.nth(i)
        val rel = obs.components.state.child_sections.nth(i)
        okArg and p.name == rel.name and p.tag == rel.tag and p.length == rel.length
          and p.relative_start == rel.start and p.relative_end == rel.end
          and p.absolute_start == at.nth(3)+rel.start and p.absolute_end == at.nth(3)+rel.end
          and p.lanes == rel.lanes and p.lanes == l.slice(p.absolute_start,p.absolute_end)
          and p.length == p.lanes.length()
      }),
    }
  }
  // Parameterized law for the publicly declared same-host/absent-vs-zero pair.
  // The pair premise is asserted by the directed test, not hidden in a fixture-ID branch.
  pure def c6HostPresenceLaws(a:C6WholeHostInputCase,b:C6WholeHostInputCase):bool = {
    val x = c6HostObserve(a)
    val y = c6HostObserve(b)
    and {
      a.host == b.host,a.config.layout == b.config.layout,a.config.budgets == b.config.budgets,
      a.limits == b.limits,a.config.ext == C6CtxAbsent,
      b.config.ext == C6CtxPresent(c6CtxZeroExt),
      x.state_context.caps == y.state_context.caps,
      not(x.state_context.ext_present),y.state_context.ext_present,
      x.components.context.effective_ext == y.components.context.effective_ext,
      x.components.context.encoded_lanes != y.components.context.encoded_lanes,
      x.components.state.encoded_lanes != y.components.state.encoded_lanes,
      x.components.metadata == y.components.metadata,x.components.program == y.components.program,
      x.decoded_host == y.decoded_host,x.decoded_config != y.decoded_config,
      x.encoded_lanes != y.encoded_lanes,
    }
  }
}
