module c6PositiveMetaEncoding {
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"

  // Normal positive representation only; no native/refusal/authority policy.
  pure val C6_META_MAGIC:int = 18918541878711089
  pure val C6_META_VERSION:int = 1
  pure val C6_META_TAG:int = 1296389185

  pure def c6MEncodeBlob(bytesArg:List[int]):List[int] =
    List(bytesArg.length()).concat(bytesArg)
  pure def c6MEncodePair(pairArg:C6MBytePair):List[int] =
    c6MEncodeBlob(pairArg.first).concat(c6MEncodeBlob(pairArg.second))
  pure def c6MEncodeOptionPair(optionArg:C6MOptionPair):List[int] =
    match optionArg {
      | C6MNoPair => List(0)
      | C6MSomePair(pairArg) => List(1).concat(c6MEncodePair(pairArg))
    }
  pure def c6MEncodeDigest(digestArg:C6MDigest):List[int] =
    match digestArg {
      | C6MUnselected => List(0)
      | C6MSelected(selectedArg) => List(1)
        .concat(c6MEncodePair(selectedArg.format))
        .concat(c6MEncodeBlob(selectedArg.bytes))
    }
  pure def c6MGrammarTag(grammarArg:C6MGrammar):int =
    match grammarArg {
      | C6MDedicatedSorts => 0
      | C6MGenericNominalExtensionReserved => 1
    }
  pure def c6MFinanceTag(financeArg:C6MFinance):int =
    match financeArg {
      | C6MFinanceAbsent => 0
      | C6MTypedExtensionReserved => 1
    }
  pure def c6MEncodeIdentity(identityArg:C6MIdentity):List[int] =
    List(identityArg.profile_id,c6MGrammarTag(identityArg.grammar_domain),
      c6MFinanceTag(identityArg.finance))
      .concat(c6MEncodePair(identityArg.layout_version))
      .concat(c6MEncodeDigest(identityArg.layout_digest))
      .concat(c6MEncodeDigest(identityArg.bounds_digest))
      .concat(c6MEncodeOptionPair(identityArg.field_id))
      .concat(c6MEncodeDigest(identityArg.program_membership_digest))

  pure def c6MEncodeBounds(boundsArg:C6MBounds):List[int] = List(
    boundsArg.code_rows,boundsArg.function_rows,boundsArg.code_child_slots,
    boundsArg.parameter_slots,boundsArg.pattern_rows,boundsArg.match_plan_rows,
    boundsArg.match_arm_rows,boundsArg.type_rows,boundsArg.constructor_rows,
    boundsArg.constructor_field_slots,boundsArg.max_arity,boundsArg.constant_nodes,
    boundsArg.constant_field_slots,boundsArg.data_nodes,boundsArg.data_field_slots,
    boundsArg.environment_cells,boundsArg.frames,boundsArg.helper_tasks,
    boundsArg.serialization_bytes,boundsArg.max_reference_work,
    boundsArg.max_helper_work,boundsArg.step_cap
  )
  pure def c6MEncodeWidthList(entriesArg:List[C6MWidthEntry]):List[int] =
    entriesArg.foldl(List(entriesArg.length()),(priorArg,entryArg) =>
      priorArg.concat(c6MEncodeBlob(entryArg.name)).append(entryArg.width))
  pure def c6MEncodeWidths(optionArg:C6MOptionWidths):List[int] =
    match optionArg {
      | C6MNoWidths => List(0)
      | C6MSomeWidths(widthsArg) => List(1)
        .concat(c6MEncodeWidthList(widthsArg.index_widths))
        .concat(c6MEncodeWidthList(widthsArg.counter_widths))
        .concat(c6MEncodeWidthList(widthsArg.record_widths))
    }
  pure def c6MEncodeRunInt(runArg:C6MRunInt):List[int] =
    match runArg {
      | C6MNegative => List(0,0)
      | C6MNat(valueArg) => List(1,valueArg)
      | C6MNatAboveU128 => List(2,0)
    }
  pure def c6MEncodeLayout(layoutArg:C6MLayout):List[int] =
    c6MEncodePair(layoutArg.layout_version)
      .concat(c6MEncodeOptionPair(layoutArg.field_id))
      .concat(c6MEncodeWidths(layoutArg.widths))
      .concat(c6MEncodeOptionPair(layoutArg.scalar_layout_id))
      .concat(c6MEncodeBounds(layoutArg.bounds))
      .concat(List(layoutArg.capacities.name_rows,layoutArg.capacities.name_bytes,
        layoutArg.capacities.occurrence_path_slots,layoutArg.capacities.value_slots,
        if(layoutArg.boundary_capacities_present) 1 else 0))

  pure def c6MEncodeMetadata(metadataArg:C6MMetadata):List[int] =
    List(C6_META_MAGIC,C6_META_VERSION,C6_META_TAG)
      .concat(c6MEncodeIdentity(metadataArg.identity))
      .concat(c6MEncodeBounds(metadataArg.bounds))
      .concat(c6MEncodeLayout(metadataArg.layout))
      .concat(c6MEncodeIdentity(metadataArg.run_id.identity))
      .concat(c6MEncodeRunInt(metadataArg.run_id.fuel_input))
      .concat(c6MEncodeRunInt(metadataArg.run_id.stack_limit_input))
}
