module c6PositiveMetaDecoding {
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"

  // Positive canonical-image cursor only. Tag/length/UTF8/host-fit/native
  // checked-success premises are external; no malformed decoder or guard.
  type C6MRState = {
    lanes:List[int],cursor:int,blob_bytes:int,width_entries:int,
    blob_spans:List[C6MBlobSpan],
  }
  type C6MRBlob = {reader:C6MRState,value:List[int]}
  type C6MRPair = {reader:C6MRState,value:C6MBytePair}
  type C6MROptionPair = {reader:C6MRState,value:C6MOptionPair}
  type C6MRDigest = {reader:C6MRState,value:C6MDigest}
  type C6MRIdentity = {reader:C6MRState,value:C6MIdentity}
  type C6MRBounds = {reader:C6MRState,value:C6MBounds}
  type C6MRWidthList = {reader:C6MRState,value:List[C6MWidthEntry]}
  type C6MRWidths = {reader:C6MRState,value:C6MOptionWidths}
  type C6MRLayout = {reader:C6MRState,value:C6MLayout}
  type C6MRRunInt = {reader:C6MRState,value:C6MRunInt}
  type C6MRMetadata = {reader:C6MRState,value:C6MMetadata}

  pure def c6MReadBlob(readerArg:C6MRState,kindArg:C6MBlobKind,
      pathArg:List[C6MPathStep]):C6MRBlob = {
    val lengthArg = readerArg.lanes.nth(readerArg.cursor)
    val startArg = readerArg.cursor+1
    val bytesArg:List[int] = range(0,lengthArg).foldl(List(),(priorArg,indexArg) =>
      priorArg.append(readerArg.lanes.nth(startArg+indexArg)))
    val spanArg:C6MBlobSpan = {kind:kindArg,path:pathArg,length_lane:readerArg.cursor,
      bytes_start:startArg,bytes_end:startArg+lengthArg,bytes:bytesArg}
    {reader:{...readerArg,cursor:startArg+lengthArg,
      blob_bytes:readerArg.blob_bytes+lengthArg,
      blob_spans:readerArg.blob_spans.append(spanArg)},value:bytesArg}
  }
  pure def c6MReadPair(readerArg:C6MRState,pathArg:List[C6MPathStep]):C6MRPair = {
    val firstArg = c6MReadBlob(readerArg,C6MText,pathArg.append(C6MField("first")))
    val secondArg = c6MReadBlob(firstArg.reader,C6MText,pathArg.append(C6MField("second")))
    {reader:secondArg.reader,value:{first:firstArg.value,second:secondArg.value}}
  }
  pure def c6MReadOptionPair(readerArg:C6MRState,
      pathArg:List[C6MPathStep]):C6MROptionPair = {
    val presentArg = readerArg.lanes.nth(readerArg.cursor)
    val nextArg = {...readerArg,cursor:readerArg.cursor+1}
    if(presentArg==0) {reader:nextArg,value:C6MNoPair}
    else {
      val pairArg = c6MReadPair(nextArg,pathArg)
      {reader:pairArg.reader,value:C6MSomePair(pairArg.value)}
    }
  }
  pure def c6MReadDigest(readerArg:C6MRState,pathArg:List[C6MPathStep]):C6MRDigest = {
    val selectedArg = readerArg.lanes.nth(readerArg.cursor)
    val nextArg = {...readerArg,cursor:readerArg.cursor+1}
    if(selectedArg==0) {reader:nextArg,value:C6MUnselected}
    else {
      val formatArg = c6MReadPair(nextArg,pathArg.append(C6MField("format")))
      val bytesArg = c6MReadBlob(formatArg.reader,C6MOpaqueDigestBytes,
        pathArg.append(C6MField("bytes")))
      {reader:bytesArg.reader,value:C6MSelected({format:formatArg.value,bytes:bytesArg.value})}
    }
  }
  pure def c6MReadIdentity(readerArg:C6MRState,
      pathArg:List[C6MPathStep]):C6MRIdentity = {
    val atArg = readerArg.cursor
    val versionArg = c6MReadPair({...readerArg,cursor:atArg+3},
      pathArg.append(C6MField("layout_version")))
    val layoutArg = c6MReadDigest(versionArg.reader,pathArg.append(C6MField("layout_digest")))
    val boundsArg = c6MReadDigest(layoutArg.reader,pathArg.append(C6MField("bounds_digest")))
    val fieldArg = c6MReadOptionPair(boundsArg.reader,pathArg.append(C6MField("field_id")))
    val membershipArg = c6MReadDigest(fieldArg.reader,
      pathArg.append(C6MField("program_membership_digest")))
    {reader:membershipArg.reader,value:{
      profile_id:readerArg.lanes.nth(atArg),
      grammar_domain:if(readerArg.lanes.nth(atArg+1)==0) C6MDedicatedSorts
        else C6MGenericNominalExtensionReserved,
      finance:if(readerArg.lanes.nth(atArg+2)==0) C6MFinanceAbsent else C6MTypedExtensionReserved,
      layout_version:versionArg.value,layout_digest:layoutArg.value,
      bounds_digest:boundsArg.value,field_id:fieldArg.value,
      program_membership_digest:membershipArg.value,
    }}
  }
  pure def c6MReadBounds(readerArg:C6MRState):C6MRBounds = {
    val atArg = readerArg.cursor
    {reader:{...readerArg,cursor:atArg+22},value:{
      code_rows:readerArg.lanes.nth(atArg),
      function_rows:readerArg.lanes.nth(atArg+1),
      code_child_slots:readerArg.lanes.nth(atArg+2),
      parameter_slots:readerArg.lanes.nth(atArg+3),
      pattern_rows:readerArg.lanes.nth(atArg+4),
      match_plan_rows:readerArg.lanes.nth(atArg+5),
      match_arm_rows:readerArg.lanes.nth(atArg+6),
      type_rows:readerArg.lanes.nth(atArg+7),
      constructor_rows:readerArg.lanes.nth(atArg+8),
      constructor_field_slots:readerArg.lanes.nth(atArg+9),
      max_arity:readerArg.lanes.nth(atArg+10),
      constant_nodes:readerArg.lanes.nth(atArg+11),
      constant_field_slots:readerArg.lanes.nth(atArg+12),
      data_nodes:readerArg.lanes.nth(atArg+13),
      data_field_slots:readerArg.lanes.nth(atArg+14),
      environment_cells:readerArg.lanes.nth(atArg+15),
      frames:readerArg.lanes.nth(atArg+16),
      helper_tasks:readerArg.lanes.nth(atArg+17),
      serialization_bytes:readerArg.lanes.nth(atArg+18),
      max_reference_work:readerArg.lanes.nth(atArg+19),
      max_helper_work:readerArg.lanes.nth(atArg+20),
      step_cap:readerArg.lanes.nth(atArg+21),
    }}
  }
  pure def c6MReadWidthList(readerArg:C6MRState,
      pathArg:List[C6MPathStep]):C6MRWidthList = {
    val lengthArg = readerArg.lanes.nth(readerArg.cursor)
    val initialArg:C6MRWidthList = {reader:{...readerArg,cursor:readerArg.cursor+1,
      width_entries:readerArg.width_entries+lengthArg},value:List()}
    range(0,lengthArg).foldl(initialArg,(priorArg,indexArg) => {
      val nameArg = c6MReadBlob(priorArg.reader,C6MText,
        pathArg.append(C6MIndex(indexArg)).append(C6MField("name")))
      val widthArg = nameArg.reader.lanes.nth(nameArg.reader.cursor)
      {reader:{...nameArg.reader,cursor:nameArg.reader.cursor+1},
        value:priorArg.value.append({name:nameArg.value,width:widthArg})}
    })
  }
  pure def c6MReadWidths(readerArg:C6MRState,pathArg:List[C6MPathStep]):C6MRWidths = {
    val presentArg = readerArg.lanes.nth(readerArg.cursor)
    val nextArg = {...readerArg,cursor:readerArg.cursor+1}
    if(presentArg==0) {reader:nextArg,value:C6MNoWidths}
    else {
      val indexArg = c6MReadWidthList(nextArg,pathArg.append(C6MField("index_widths")))
      val counterArg = c6MReadWidthList(indexArg.reader,pathArg.append(C6MField("counter_widths")))
      val recordArg = c6MReadWidthList(counterArg.reader,pathArg.append(C6MField("record_widths")))
      {reader:recordArg.reader,value:C6MSomeWidths({index_widths:indexArg.value,
        counter_widths:counterArg.value,record_widths:recordArg.value})}
    }
  }
  pure def c6MReadLayout(readerArg:C6MRState,pathArg:List[C6MPathStep]):C6MRLayout = {
    val versionArg = c6MReadPair(readerArg,pathArg.append(C6MField("layout_version")))
    val fieldArg = c6MReadOptionPair(versionArg.reader,pathArg.append(C6MField("field_id")))
    val widthsArg = c6MReadWidths(fieldArg.reader,pathArg.append(C6MField("widths")))
    val scalarArg = c6MReadOptionPair(widthsArg.reader,pathArg.append(C6MField("scalar_layout_id")))
    val boundsArg = c6MReadBounds(scalarArg.reader)
    val atArg = boundsArg.reader.cursor
    {reader:{...boundsArg.reader,cursor:atArg+5},value:{
      layout_version:versionArg.value,field_id:fieldArg.value,widths:widthsArg.value,
      scalar_layout_id:scalarArg.value,bounds:boundsArg.value,
      capacities:{name_rows:boundsArg.reader.lanes.nth(atArg),
        name_bytes:boundsArg.reader.lanes.nth(atArg+1),
        occurrence_path_slots:boundsArg.reader.lanes.nth(atArg+2),
        value_slots:boundsArg.reader.lanes.nth(atArg+3)},
      boundary_capacities_present:boundsArg.reader.lanes.nth(atArg+4)==1,
    }}
  }
  pure def c6MReadRunInt(readerArg:C6MRState):C6MRRunInt = {
    val atArg = readerArg.cursor
    val tagArg = readerArg.lanes.nth(atArg)
    {reader:{...readerArg,cursor:atArg+2},value:
      if(tagArg==0) C6MNegative
      else if(tagArg==1) C6MNat(readerArg.lanes.nth(atArg+1))
      else C6MNatAboveU128}
  }
  pure def c6MDecodeMetadataPositive(lanesArg:List[int]):C6MRMetadata = {
    val initialArg:C6MRState = {lanes:lanesArg,cursor:3,blob_bytes:0,
      width_entries:0,blob_spans:List()}
    val identityArg = c6MReadIdentity(initialArg,List(C6MField("identity")))
    val boundsArg = c6MReadBounds(identityArg.reader)
    val layoutArg = c6MReadLayout(boundsArg.reader,List(C6MField("layout")))
    val runIdentityArg = c6MReadIdentity(layoutArg.reader,
      List(C6MField("run_id"),C6MField("identity")))
    val fuelArg = c6MReadRunInt(runIdentityArg.reader)
    val stackArg = c6MReadRunInt(fuelArg.reader)
    {reader:stackArg.reader,value:{identity:identityArg.value,bounds:boundsArg.value,
      layout:layoutArg.value,run_id:{identity:runIdentityArg.value,
        fuel_input:fuelArg.value,stack_limit_input:stackArg.value}}}
  }
}
