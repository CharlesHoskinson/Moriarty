module c6PositiveDataPadded {
  import c6PhysicalTypes.* from "./c6PhysicalTypes"
  import c6PositiveDataTypes.* from "./c6PositiveDataTypes"
  import c6PositiveEncoding.C6_MAGIC from "./c6PositiveEncoding"
  import c6PositiveEncoding.c6Pad from "./c6PositiveEncoding"
  import c6PositiveEncoding.c6EncodeRef from "./c6PositiveEncoding"
  import c6PositiveEncoding.c6EncodeEnv from "./c6PositiveEncoding"
  import c6PositiveDecoding.c6DecodePositiveRef from "./c6PositiveDecoding"
  import c6PositiveDecoding.c6DecodePositiveEnv from "./c6PositiveDecoding"
  import c6PositiveDecoding.c6DecodePositiveSlice from "./c6PositiveDecoding"

  // NORMAL POSITIVE DOMAIN ONLY, from the frozen owner contract:
  // gate1; fixed finite capacity geometry; counts at most caps; typed rows;
  // raw full-u128 payloads, canonical row padding and owner-valid checked caps.
  // Ref and previous indices use TABLE CAPS, not active-count membership.
  // These functions do not validate/reject untrusted lanes or implement gates.
  // Outside this positive image, interpretation is unspecified. No host-width,
  // admission, negative/refusal, SDK/field, namespace or native policy is given.
  pure val C6_DATA_SECTION_TAG: int = 1145132097

  pure def c6DataPositiveGeometry(capsArg:C6DataRelevantCaps):C6DataGeometry = {
    val fieldsBaseArg = 7 + 5*capsArg.max_data_nodes
    val envBaseArg = fieldsBaseArg + 2*capsArg.max_data_field_slots
    val valuesBaseArg = envBaseArg + 5*capsArg.max_env_cells
    {data_base:7,fields_base:fieldsBaseArg,env_base:envBaseArg,
      values_base:valuesBaseArg,total:valuesBaseArg + 2*capsArg.max_value_slots}
  }
  pure def c6DataPositiveCounts(tablesArg:C6DataTables):List[int] =
    List(tablesArg.data.length(),tablesArg.data_fields.length(),
      tablesArg.env.length(),tablesArg.value_slots.length())

  pure def c6EncodePositiveDataRow(rowArg:C6DataNode):List[int] = match rowArg {
    | C6DataU128(payloadArg) => List(0,payloadArg.n,0,0,0)
    | C6DataBool(payloadArg) => List(1,if(payloadArg.b) 1 else 0,0,0,0)
    | C6DataNil => List(2,0,0,0,0)
    | C6DataCons(payloadArg) => List(3,0,0,payloadArg.fields.start,payloadArg.fields.len)
    | C6DataLeaf(payloadArg) => List(4,0,0,payloadArg.fields.start,payloadArg.fields.len)
    | C6DataBranch(payloadArg) => List(5,0,0,payloadArg.fields.start,payloadArg.fields.len)
    | C6DataConstructor(payloadArg) => List(6,payloadArg.type_id,payloadArg.ctor_id,
        payloadArg.fields.start,payloadArg.fields.len)
  }
  pure def c6EncodePositiveDataEnvRow(cellArg:C6DataEnvCell):List[int] =
    List(cellArg.binder).concat(c6EncodeRef(cellArg.value)).concat(c6EncodeEnv(cellArg.previous))

  pure def c6PositivePaddedDataLanes(capsArg:C6DataRelevantCaps,tablesArg:C6DataTables):List[int] = {
    // Ordered flattening of the active prefixes. Each partition is padded
    // separately to its CAPACITY width; inactive zeros remain in whole output.
    val dataLanesArg = tablesArg.data.foldl(List(),(lanesArg,rowArg) =>
      lanesArg.concat(c6EncodePositiveDataRow(rowArg)))
    val fieldLanesArg = tablesArg.data_fields.foldl(List(),(lanesArg,refArg) =>
      lanesArg.concat(c6EncodeRef(refArg)))
    val envLanesArg = tablesArg.env.foldl(List(),(lanesArg,cellArg) =>
      lanesArg.concat(c6EncodePositiveDataEnvRow(cellArg)))
    val valueLanesArg = tablesArg.value_slots.foldl(List(),(lanesArg,refArg) =>
      lanesArg.concat(c6EncodeRef(refArg)))
    List(C6_MAGIC,1,C6_DATA_SECTION_TAG).concat(c6DataPositiveCounts(tablesArg))
      .concat(c6Pad(dataLanesArg,5*capsArg.max_data_nodes))
      .concat(c6Pad(fieldLanesArg,2*capsArg.max_data_field_slots))
      .concat(c6Pad(envLanesArg,5*capsArg.max_env_cells))
      .concat(c6Pad(valueLanesArg,2*capsArg.max_value_slots))
  }

  pure def c6DecodePositiveDataRow(rowArg:List[int]):C6DataNode = {
    val tagArg = rowArg.nth(0)
    if(tagArg == 0) C6DataU128({n:rowArg.nth(1)})
    else if(tagArg == 1) C6DataBool({b:rowArg.nth(1) == 1})
    else if(tagArg == 2) C6DataNil
    else if(tagArg == 3) C6DataCons({fields:c6DecodePositiveSlice(rowArg,3)})
    else if(tagArg == 4) C6DataLeaf({fields:c6DecodePositiveSlice(rowArg,3)})
    else if(tagArg == 5) C6DataBranch({fields:c6DecodePositiveSlice(rowArg,3)})
    // Remaining valid positive DATA tag is6. No malformed-tag policy.
    else C6DataConstructor({type_id:rowArg.nth(1),ctor_id:rowArg.nth(2),
      fields:c6DecodePositiveSlice(rowArg,3)})
  }
  pure def c6DecodePositiveDataEnvRow(rowArg:List[int]):C6DataEnvCell =
    {binder:rowArg.nth(0),value:c6DecodePositiveRef(rowArg,1),
      previous:c6DecodePositiveEnv(rowArg,3)}

  pure def c6PositivePaddedDataDecode(capsArg:C6DataRelevantCaps,lanesArg:List[int]):C6DataTables = {
    val geometryArg = c6DataPositiveGeometry(capsArg)
    // Header counts select active rows, but all four BASES use capacities.
    // Complete padded lanes are separately retained by the observation.
    val dataRowsArg = range(0,lanesArg.nth(3)).foldl(List(),(rowsArg,indexArg) =>
      rowsArg.append(c6DecodePositiveDataRow(lanesArg.slice(
        geometryArg.data_base+5*indexArg,geometryArg.data_base+5*(indexArg+1)))))
    val fieldRowsArg = range(0,lanesArg.nth(4)).foldl(List(),(rowsArg,indexArg) =>
      rowsArg.append(c6DecodePositiveRef(lanesArg,geometryArg.fields_base+2*indexArg)))
    val envRowsArg = range(0,lanesArg.nth(5)).foldl(List(),(rowsArg,indexArg) =>
      rowsArg.append(c6DecodePositiveDataEnvRow(lanesArg.slice(
        geometryArg.env_base+5*indexArg,geometryArg.env_base+5*(indexArg+1)))))
    val valueRowsArg = range(0,lanesArg.nth(6)).foldl(List(),(rowsArg,indexArg) =>
      rowsArg.append(c6DecodePositiveRef(lanesArg,geometryArg.values_base+2*indexArg)))
    {data:dataRowsArg,data_fields:fieldRowsArg,env:envRowsArg,value_slots:valueRowsArg}
  }

  pure def c6PositiveDataObservation(capsArg:C6DataRelevantCaps,tablesArg:C6DataTables):C6DataPositiveObservation = {
    val paddedArg = c6PositivePaddedDataLanes(capsArg,tablesArg)
    {geometry:c6DataPositiveGeometry(capsArg),counts:c6DataPositiveCounts(tablesArg),
      padded_lanes:paddedArg,decoded_tables:c6PositivePaddedDataDecode(capsArg,paddedArg)}
  }
  pure def positivePaddedDataExact(capsArg:C6DataRelevantCaps,tablesArg:C6DataTables,
      expectedArg:C6DataPositiveObservation):bool = {
    val observationArg = c6PositiveDataObservation(capsArg,tablesArg)
    // Independent complete observation prevents a mutually wrong roundtrip
    // from passing. Every geometry/count/raw lane/typed field is included.
    observationArg == expectedArg
      and observationArg.decoded_tables == tablesArg
      and c6PositivePaddedDataDecode(capsArg,expectedArg.padded_lanes) == tablesArg
  }
}
