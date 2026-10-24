module c6PositiveStateCodec {
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import c6PositiveEncoding.* from "./c6PositiveEncoding"
  import c6PositiveDecoding.* from "./c6PositiveDecoding"
  import c6PositiveHelperCodec.* from "./c6PositiveHelperCodec"
  // Normal positive code images and all frozen native/cap/length premises only.
  // No error/refusal/admission policy or new guard enforcement is introduced.
  pure def sEncodeDataRow(x:C6HDataRow):List[int] = match x {
    | C6HU128(p) => List(0,p.n,0,0,0)
    | C6HBool(p) => List(1,hBool(p.b),0,0,0)
    | C6HNil => List(2,0,0,0,0)
    | C6HCons(p) => List(3,0,0,p.fields.start,p.fields.len)
    | C6HLeaf(p) => List(4,0,0,p.fields.start,p.fields.len)
    | C6HBranch(p) => List(5,0,0,p.fields.start,p.fields.len)
    | C6HConstructor(p) => List(6,p.type_id,p.ctor_id,p.fields.start,p.fields.len)
  }
  pure def sDecodeDataRow(l:List[int]):C6HDataRow =
    if(l.nth(0) == 0) C6HU128({n:l.nth(1)})
    else if(l.nth(0) == 1) C6HBool({b:l.nth(1) == 1})
    else if(l.nth(0) == 2) C6HNil
    else if(l.nth(0) == 3) C6HCons({fields:c6DecodePositiveSlice(l,3)})
    else if(l.nth(0) == 4) C6HLeaf({fields:c6DecodePositiveSlice(l,3)})
    else if(l.nth(0) == 5) C6HBranch({fields:c6DecodePositiveSlice(l,3)})
    else C6HConstructor({type_id:l.nth(1),ctor_id:l.nth(2),fields:c6DecodePositiveSlice(l,3)})
  pure def sEncodeEnvCell(x:C6HEnvCell):List[int] =
    List(x.binder).concat(c6EncodeRef(x.value)).concat(c6EncodeEnv(x.previous))
  pure def sDecodeEnvCell(l:List[int]):C6HEnvCell =
    {binder:l.nth(0),value:c6DecodePositiveRef(l,1),previous:c6DecodePositiveEnv(l,3)}
  pure def sEncodeData(x:C6HDataTables):List[int] = {
    val a = x.data.foldl(List(C6_MAGIC,1,1145132097,x.data.length(),x.data_fields.length(),x.env.length(),x.value_slots.length()),
      (acc,rowArg) => acc.concat(sEncodeDataRow(rowArg)))
    val b = x.data_fields.foldl(a,(acc,rowArg) => acc.concat(c6EncodeRef(rowArg)))
    val c = x.env.foldl(b,(acc,rowArg) => acc.concat(sEncodeEnvCell(rowArg)))
    x.value_slots.foldl(c,(acc,rowArg) => acc.concat(c6EncodeRef(rowArg)))
  }
  pure def sDecodeData(l:List[int]):C6HDataTables = {
    val fieldsStart = 7+5*l.nth(3)
    val envStart = fieldsStart+2*l.nth(4)
    val slotsStart = envStart+5*l.nth(5)
    val dataArg:List[C6HDataRow] = range(0,l.nth(3)).foldl(List(),(acc,i) =>
      acc.append(sDecodeDataRow(l.slice(7+5*i,7+5*(i+1)))))
    val fieldsArg:List[C6Ref] = range(0,l.nth(4)).foldl(List(),(acc,i) =>
      acc.append(c6DecodePositiveRef(l,fieldsStart+2*i)))
    val envArg:List[C6HEnvCell] = range(0,l.nth(5)).foldl(List(),(acc,i) =>
      acc.append(sDecodeEnvCell(l.slice(envStart+5*i,envStart+5*(i+1)))))
    val slotsArg:List[C6Ref] = range(0,l.nth(6)).foldl(List(),(acc,i) =>
      acc.append(c6DecodePositiveRef(l,slotsStart+2*i)))
    {data:dataArg,data_fields:fieldsArg,env:envArg,value_slots:slotsArg}
  }
  pure def sCapacityDescriptor(c:C6HCaps):List[int] =
    List(c.max_frames,c.max_expressions,c.max_names,c.max_arms,c.max_children,c.max_value_slots,c.max_constants,c.max_data_nodes,c.max_env_cells,c.max_functions,c.max_arity,c.max_types,c.max_occurrences,c.max_occurrence_paths,c.max_parameters,c.max_patterns,c.max_pattern_children,c.max_constructors,c.max_ctor_fields,c.max_decl_templates,c.max_plan_nodes,c.max_plan_cases,c.max_field_paths,c.max_constant_fields,c.max_name_bytes,c.max_type_expr_nodes,c.max_template_bytes,c.max_data_field_slots,c.max_helper_tasks)
  pure def sImageInput(x:C6HState):C6Section =
    {control:x.root.control,control_origin:x.root.control_origin,env_root:x.root.env_root,stack_limit:x.root.stack_limit,outcome:x.root.outcome,initial_fuel:x.root.initial_fuel,remaining_work:x.root.remaining_work,consumed_work:x.root.consumed_work,helper_work:x.root.helper_work,helper_visits:x.root.helper_visits,reference_cursor:x.root.reference_cursor,alloc:x.root.alloc,active:x.root.active,seg_active:x.root.seg_active,seg_alloc:x.root.seg_alloc,seg_consumed_work:x.root.seg_consumed_work,frames:x.frames}
  pure def sRootFromSection(x:C6Section):C6HRoot =
    {control:x.control,control_origin:x.control_origin,env_root:x.env_root,stack_limit:x.stack_limit,outcome:x.outcome,initial_fuel:x.initial_fuel,remaining_work:x.remaining_work,consumed_work:x.consumed_work,helper_work:x.helper_work,helper_visits:x.helper_visits,reference_cursor:x.reference_cursor,alloc:x.alloc,active:x.active,seg_active:x.seg_active,seg_alloc:x.seg_alloc,seg_consumed_work:x.seg_consumed_work}
  pure def sEncodeState(x:C6HState,ctx:C6HContext):List[int] = {
    val imageArg = c6EncodeSection(sImageInput(x))
    val dataArg = sEncodeData(x.data)
    val helperArg = hEncodeTables(x.helper)
    List(C6_MAGIC,1,1398030676,3,1229799751,imageArg.length(),1145132097,dataArg.length(),
      1212501072,helperArg.length(),hBool(ctx.ext_present),0)
      .concat(sCapacityDescriptor(ctx.caps)).concat(imageArg).concat(dataArg).concat(helperArg)
  }
  pure def sDecodeState(l:List[int]):C6HState = {
    val imageEnd = 41+l.nth(5)
    val dataEnd = imageEnd+l.nth(7)
    val helperEnd = dataEnd+l.nth(9)
    val imageArg = c6DecodePositiveSection(l.slice(41,imageEnd))
    {root:sRootFromSection(imageArg),frames:imageArg.frames,
      data:sDecodeData(l.slice(imageEnd,dataEnd)),helper:hDecodeTables(l.slice(dataEnd,helperEnd))}
  }
  pure def sStateEnd(l:List[int]):int = 41+l.nth(5)+l.nth(7)+l.nth(9)
}
