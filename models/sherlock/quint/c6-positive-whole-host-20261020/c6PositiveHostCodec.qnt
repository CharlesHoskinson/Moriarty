module c6PositiveHostCodec {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import c6PositiveMetaEncoding.* from "./c6PositiveMetaEncoding"
  import c6PositiveMetaDecoding.* from "./c6PositiveMetaDecoding"
  import c6PositiveProgramEncoding.* from "./c6PositiveProgramEncoding"
  import c6PositiveProgramDecoding.* from "./c6PositiveProgramDecoding"
  import c6PositiveContext.* from "./c6PositiveContext"
  import c6PositiveStateCodec.* from "./c6PositiveStateCodec"
  // General positive canonical code-image projection only. Successful native child
  // codec/cap/host-fit/UTF8/tree/config premises are external, not enforced here.
  // Off-domain interpretation is unspecified. No admission/refusal policy.
  type C6HostDecoded = {host:C6WholeHost,config:C6WholeHostConfig,decoded_context:C6CtxPayload}
  pure def c6HostMetadata(h:C6WholeHost):C6MMetadata =
    {identity:h.admission.identity,bounds:h.admission.bounds,
      layout:h.admission.layout,run_id:h.image.run_id}
  pure def c6HostContextInput(cfg:C6WholeHostConfig,lim:C6MHostLimits):C6CtxInput =
    {context:{ext:cfg.ext,budgets:cfg.budgets},bounds:cfg.layout.bounds,
      capacities:cfg.layout.capacities,limits:lim}
  pure def c6HostFlatContext(viewArg:C6CtxStateView):C6HContext =
    {caps:{max_frames:viewArg.caps.phys.max_frames,max_expressions:viewArg.caps.phys.max_expressions,max_names:viewArg.caps.phys.max_names,max_arms:viewArg.caps.phys.max_arms,max_children:viewArg.caps.phys.max_children,max_value_slots:viewArg.caps.phys.max_value_slots,max_constants:viewArg.caps.phys.max_constants,max_data_nodes:viewArg.caps.phys.max_data_nodes,max_env_cells:viewArg.caps.phys.max_env_cells,max_functions:viewArg.caps.phys.max_functions,max_arity:viewArg.caps.phys.max_arity,max_types:viewArg.caps.phys.max_types,max_occurrences:viewArg.caps.phys.max_occurrences,max_occurrence_paths:viewArg.caps.phys.max_occurrence_paths,max_parameters:viewArg.caps.phys.max_parameters,max_patterns:viewArg.caps.phys.max_patterns,max_pattern_children:viewArg.caps.phys.max_pattern_children,max_constructors:viewArg.caps.phys.max_constructors,max_ctor_fields:viewArg.caps.phys.max_ctor_fields,max_decl_templates:viewArg.caps.phys.max_decl_templates,max_plan_nodes:viewArg.caps.phys.max_plan_nodes,max_plan_cases:viewArg.caps.phys.max_plan_cases,max_field_paths:viewArg.caps.phys.max_field_paths,max_constant_fields:viewArg.caps.phys.max_constant_fields,max_name_bytes:viewArg.caps.phys.max_name_bytes,max_type_expr_nodes:viewArg.caps.phys.max_type_expr_nodes,max_template_bytes:viewArg.caps.phys.max_template_bytes,max_data_field_slots:viewArg.caps.max_data_field_slots,max_helper_tasks:viewArg.caps.max_helper_tasks},ext_present:viewArg.ext_present,
      limits:{max_total_lanes:viewArg.limits.max_total_lanes}}
  pure def c6HostStateContext(cfg:C6WholeHostConfig,lim:C6MHostLimits):C6HContext =
    c6HostFlatContext(c6CtxDeriveState(c6HostContextInput(cfg,lim)))
  pure def c6HostOwnedState(img:C6HostImage):C6HState =
    {root:{control:img.control,control_origin:img.control_origin,env_root:img.env_root,stack_limit:img.stack_limit,outcome:img.outcome,initial_fuel:img.initial_fuel,remaining_work:img.remaining_work,consumed_work:img.consumed_work,helper_work:img.helper_work,helper_visits:img.helper_visits,reference_cursor:img.reference_cursor,alloc:img.alloc,active:img.active,seg_active:img.seg_base.active,seg_alloc:img.seg_base.alloc,seg_consumed_work:img.seg_base.consumed_work},frames:img.frames,
      data:{data:img.data,data_fields:img.data_fields,env:img.env,value_slots:img.value_slots},
      helper:{helper:img.helper,result:img.result,tasks:img.tasks}}
  pure def c6HostImageFromParts(runArg:C6MRunId,st:C6HState):C6HostImage =
    {run_id:runArg,control:st.root.control,control_origin:st.root.control_origin,env_root:st.root.env_root,stack_limit:st.root.stack_limit,outcome:st.root.outcome,initial_fuel:st.root.initial_fuel,remaining_work:st.root.remaining_work,consumed_work:st.root.consumed_work,helper_work:st.root.helper_work,helper_visits:st.root.helper_visits,reference_cursor:st.root.reference_cursor,alloc:st.root.alloc,active:st.root.active,
      helper:st.helper.helper,result:st.helper.result,
      seg_base:{active:st.root.seg_active,alloc:st.root.seg_alloc,consumed_work:st.root.seg_consumed_work},
      data:st.data.data,data_fields:st.data.data_fields,env:st.data.env,frames:st.frames,
      tasks:st.helper.tasks,value_slots:st.data.value_slots}
  pure def c6HostAssemble(metaArg:List[int],progArg:List[int],ctxArg:List[int],stateArg:List[int]):List[int] =
    List(18918541878711089,1,1213158228,1,4,
      1296389185,metaArg.length(),1347571527,progArg.length(),1129601108,ctxArg.length(),
      1398030676,stateArg.length(),0).concat(metaArg).concat(progArg).concat(ctxArg).concat(stateArg)
  pure def c6HostEncode(h:C6WholeHost,cfg:C6WholeHostConfig,lim:C6MHostLimits):List[int] =
    c6HostAssemble(c6MEncodeMetadata(c6HostMetadata(h)),c6PEncodeProgram(h.admission.view),
      c6CtxEncode({ext:cfg.ext,budgets:cfg.budgets}),
      sEncodeState(c6HostOwnedState(h.image),c6HostStateContext(cfg,lim)))
  // Offsets come from complete actual directory lengths, never capacity bounds.
  pure def c6HostOffsets(l:List[int]):List[int] =
    range(0,4).foldl(List(14),(acc,i) => acc.append(acc.nth(i)+l.nth(6+2*i)))
  pure def c6HostDecodePositive(l:List[int],cfg:C6WholeHostConfig):C6HostDecoded = {
    val at = c6HostOffsets(l)
    val metaArg = c6MDecodeMetadataPositive(l.slice(at.nth(0),at.nth(1))).value
    val progArg = c6PDecodeProgramPositive(l.slice(at.nth(1),at.nth(2))).build.program
    val contextArg = c6CtxDecodePositive(l.slice(at.nth(2),at.nth(3)))
    val stateArg = sDecodeState(l.slice(at.nth(3),at.nth(4)))
    // Rust verifies decoded META/CTX equal caller config before reconstructing.
    // This positive projection retains that explicit external premise, not a guard.
    {host:{admission:{identity:metaArg.identity,bounds:metaArg.bounds,
      layout:metaArg.layout,view:progArg},image:c6HostImageFromParts(metaArg.run_id,stateArg)},config:cfg,decoded_context:contextArg}
  }
}
