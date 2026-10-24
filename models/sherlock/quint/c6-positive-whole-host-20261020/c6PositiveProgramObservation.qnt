module c6PositiveProgramObservation {
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveProgramEncoding.* from "./c6PositiveProgramEncoding"
  import c6PositiveProgramDecoding.* from "./c6PositiveProgramDecoding"

  // General source-only production. No expected argument/import here.
  pure def c6PObserve(programArg:C6PProgram):C6PObservation = {
    val lanesArg=c6PEncodeProgram(programArg)
    val decodedArg=c6PDecodeProgramPositive(lanesArg)
    {encoded_lanes:lanesArg,decoded_program:decodedArg.build.program,
      table_counts:decodedArg.table_counts,table_spans:decodedArg.table_spans,
      text_spans:decodedArg.build.reader.text_spans,final_cursor:decodedArg.build.reader.cursor}
  }

  // Independently prefrozen literal DATA enters only this final comparison.
  pure def c6PWholeObservationsExact(casesArg:List[C6PInputCase],expectedArg:List[C6PObservation]):bool =
    if(casesArg.length()==expectedArg.length()) {
      range(0,casesArg.length()).foldl(true,(priorArg,indexArg) => {
        val programArg=casesArg.nth(indexArg).program
        val actualArg=c6PObserve(programArg)
        priorArg and actualArg==expectedArg.nth(indexArg)
          and actualArg.decoded_program==programArg
          and actualArg.table_counts==c6PTableCounts(programArg)
          and actualArg.final_cursor==actualArg.encoded_lanes.length()
      })
    } else false

  // Diagnostic cursor laws, not B/helper microsteps or production charges.
  pure def c6PTableContinuity(observationArg:C6PObservation):bool =
    observationArg.table_spans.length()==22 and observationArg.table_counts.length()==22
      and range(0,22).foldl(true,(priorArg,indexArg) => {
        val tableArg=observationArg.table_spans.nth(indexArg)
        val previousEndArg=if(indexArg==0) 4 else observationArg.table_spans.nth(indexArg-1).rows_end
        val rowEndArg=if(tableArg.row_count==0) tableArg.rows_start
          else tableArg.row_spans.nth(tableArg.row_count-1).end
        priorArg and tableArg.table==c6PTableNames.nth(indexArg)
          and tableArg.count_lane==previousEndArg
          and observationArg.encoded_lanes.nth(tableArg.count_lane)==tableArg.row_count
          and tableArg.row_count==observationArg.table_counts.nth(indexArg)
          and tableArg.rows_start==tableArg.count_lane+1
          and tableArg.row_spans.length()==tableArg.row_count
          and tableArg.rows_end==rowEndArg
          and range(0,tableArg.row_count).foldl(true,(rowsOkArg,rowArg) => {
            val spanArg=tableArg.row_spans.nth(rowArg)
            val startArg=if(rowArg==0) tableArg.rows_start else tableArg.row_spans.nth(rowArg-1).end
            rowsOkArg and spanArg.row_index==rowArg and spanArg.start==startArg and spanArg.end>spanArg.start
          })
      })
      and observationArg.table_spans.nth(21).rows_end==observationArg.final_cursor
      and observationArg.final_cursor==observationArg.encoded_lanes.length()

  pure def c6PTextContinuity(observationArg:C6PObservation):bool =
    range(0,observationArg.text_spans.length()).foldl(true,(priorArg,indexArg) => {
      val spanArg=observationArg.text_spans.nth(indexArg)
      priorArg and spanArg.bytes_start==spanArg.length_lane+1
        and spanArg.bytes_end==spanArg.bytes_start+spanArg.utf8_bytes.length()
        and observationArg.encoded_lanes.nth(spanArg.length_lane)==spanArg.utf8_bytes.length()
        and range(0,spanArg.utf8_bytes.length()).foldl(true,(bytesOkArg,byteArg) =>
          bytesOkArg and observationArg.encoded_lanes.nth(spanArg.bytes_start+byteArg)==spanArg.utf8_bytes.nth(byteArg))
    })
  pure def c6PAllCursorLaws(casesArg:List[C6PInputCase]):bool =
    range(0,casesArg.length()).foldl(true,(priorArg,indexArg) => {
      val actualArg=c6PObserve(casesArg.nth(indexArg).program)
      priorArg and c6PTableContinuity(actualArg) and c6PTextContinuity(actualArg)
    })
}
