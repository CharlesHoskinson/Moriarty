module c6PositiveMetaTypes {
  // Declaration-only exact positive byte-projection carriers.
  // No encoder, reader, inputs, output data, policy or model implementation.
  type C6MBytePair = {first:List[int],second:List[int]}
  type C6MOptionPair = C6MNoPair | C6MSomePair(C6MBytePair)
  type C6MWidthEntry = {name:List[int],width:int}
  type C6MWidths = {
    index_widths:List[C6MWidthEntry],counter_widths:List[C6MWidthEntry],
    record_widths:List[C6MWidthEntry],
  }
  type C6MOptionWidths = C6MNoWidths | C6MSomeWidths(C6MWidths)
  type C6MGrammar = C6MDedicatedSorts | C6MGenericNominalExtensionReserved
  type C6MFinance = C6MFinanceAbsent | C6MTypedExtensionReserved
  type C6MDigest = C6MUnselected | C6MSelected({format:C6MBytePair,bytes:List[int]})
  type C6MRunInt = C6MNegative | C6MNat(int) | C6MNatAboveU128
  type C6MIdentity = {
    profile_id:int,grammar_domain:C6MGrammar,finance:C6MFinance,
    layout_version:C6MBytePair,layout_digest:C6MDigest,bounds_digest:C6MDigest,
    field_id:C6MOptionPair,program_membership_digest:C6MDigest,
  }
  type C6MBounds = {
    code_rows:int,function_rows:int,code_child_slots:int,parameter_slots:int,
    pattern_rows:int,match_plan_rows:int,match_arm_rows:int,type_rows:int,
    constructor_rows:int,constructor_field_slots:int,max_arity:int,
    constant_nodes:int,constant_field_slots:int,data_nodes:int,
    data_field_slots:int,environment_cells:int,frames:int,helper_tasks:int,
    serialization_bytes:int,max_reference_work:int,max_helper_work:int,step_cap:int,
  }
  type C6MLayoutCaps = {
    name_rows:int,name_bytes:int,occurrence_path_slots:int,value_slots:int,
  }
  type C6MLayout = {
    layout_version:C6MBytePair,field_id:C6MOptionPair,widths:C6MOptionWidths,
    scalar_layout_id:C6MOptionPair,bounds:C6MBounds,capacities:C6MLayoutCaps,
    boundary_capacities_present:bool,
  }
  type C6MRunId = {identity:C6MIdentity,fuel_input:C6MRunInt,stack_limit_input:C6MRunInt}
  type C6MMetadata = {identity:C6MIdentity,bounds:C6MBounds,layout:C6MLayout,run_id:C6MRunId}
  type C6MHostLimits = {
    max_total_lanes:int,max_state_lanes:int,max_metadata_bytes:int,
    max_string_bytes:int,max_width_entries:int,
  }
  type C6MInputCase = {case_id:str,metadata:C6MMetadata,limits:C6MHostLimits}
  type C6MPathStep = C6MField(str) | C6MIndex(int)
  type C6MBlobKind = C6MText | C6MOpaqueDigestBytes
  type C6MBlobSpan = {
    kind:C6MBlobKind,path:List[C6MPathStep],length_lane:int,
    bytes_start:int,bytes_end:int,bytes:List[int],
  }
  type C6MObservation = {
    encoded_lanes:List[int],decoded_metadata:C6MMetadata,blob_spans:List[C6MBlobSpan],
    final_blob_bytes:int,final_width_entries:int,final_cursor:int,
  }
  type C6MExpectedRoot = {
    schema:str,profile:str,case_ids:List[str],observations:List[C6MObservation],
  }
}
