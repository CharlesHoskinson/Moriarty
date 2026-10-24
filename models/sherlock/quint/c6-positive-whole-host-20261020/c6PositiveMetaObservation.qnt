module c6PositiveMetaObservation {
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveMetaEncoding.* from "./c6PositiveMetaEncoding"
  import c6PositiveMetaDecoding.* from "./c6PositiveMetaDecoding"

  // Full six-field normal result; private expected data never enters here.
  pure def c6MObserve(metadataArg:C6MMetadata):C6MObservation = {
    val lanesArg = c6MEncodeMetadata(metadataArg)
    val decodedArg = c6MDecodeMetadataPositive(lanesArg)
    {encoded_lanes:lanesArg,decoded_metadata:decodedArg.value,
      blob_spans:decodedArg.reader.blob_spans,
      final_blob_bytes:decodedArg.reader.blob_bytes,
      final_width_entries:decodedArg.reader.width_entries,
      final_cursor:decodedArg.reader.cursor}
  }
  pure def c6MWidthEntries(optionArg:C6MOptionWidths):int =
    match optionArg {
      | C6MNoWidths => 0
      | C6MSomeWidths(widthsArg) => widthsArg.index_widths.length()
        +widthsArg.counter_widths.length()+widthsArg.record_widths.length()
    }
  pure def c6MBlobSpanLaws(observationArg:C6MObservation):bool = {
    val sumArg = observationArg.blob_spans.foldl(0,(priorArg,spanArg) =>
      priorArg+spanArg.bytes.length())
    sumArg==observationArg.final_blob_bytes
      and range(0,observationArg.blob_spans.length()).foldl(true,(priorArg,indexArg) => {
        val spanArg = observationArg.blob_spans.nth(indexArg)
        val previousEndArg = if(indexArg==0) 3
          else observationArg.blob_spans.nth(indexArg-1).bytes_end
        priorArg and spanArg.length_lane>=previousEndArg
          and spanArg.bytes_start==spanArg.length_lane+1
          and spanArg.bytes_end==spanArg.bytes_start+spanArg.bytes.length()
          and spanArg.bytes_end<=observationArg.final_cursor
          and observationArg.encoded_lanes.nth(spanArg.length_lane)==spanArg.bytes.length()
          and range(0,spanArg.bytes.length()).foldl(true,(bytesOkArg,byteArg) =>
            bytesOkArg and observationArg.encoded_lanes.nth(spanArg.bytes_start+byteArg)
              ==spanArg.bytes.nth(byteArg))
      })
  }
  // General laws on positive canonical input, not malformed-stream guards.
  pure def c6MPositiveRoundtripLaws(metadataArg:C6MMetadata):bool = {
    val observationArg = c6MObserve(metadataArg)
    observationArg.decoded_metadata==metadataArg
      and c6MEncodeMetadata(observationArg.decoded_metadata)==observationArg.encoded_lanes
      and observationArg.final_cursor==observationArg.encoded_lanes.length()
      and observationArg.final_width_entries==c6MWidthEntries(metadataArg.layout.widths)
      and c6MBlobSpanLaws(observationArg)
  }
  // Compare EVERY original expected-root field. Static wrapper labels are
  // declared interface literals; no expected field feeds the writer/reader.
  pure def c6MWholeExpectedRootExact(inputsArg:List[C6MInputCase],
      expectedArg:C6MExpectedRoot):bool = {
    val idsArg:List[str] = inputsArg.foldl(List(),(priorArg,inputArg) =>
      priorArg.append(inputArg.case_id))
    val observationsArg:List[C6MObservation] = inputsArg.foldl(List(),(priorArg,inputArg) =>
      priorArg.append(c6MObserve(inputArg.metadata)))
    expectedArg.schema=="independent-c6-complete-META-positive/1"
      and expectedArg.profile=="sherlock-c6-complete-META-positive/1"
      and expectedArg.case_ids==idsArg
      and expectedArg.observations==observationsArg
      and inputsArg.foldl(true,(priorArg,inputArg) =>
        priorArg and c6MPositiveRoundtripLaws(inputArg.metadata))
  }
}
