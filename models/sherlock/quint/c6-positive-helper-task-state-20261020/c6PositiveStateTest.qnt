module c6PositiveStateTest {
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import c6PositiveStateInputs.* from "./c6PositiveStateInputs"
  import c6PositiveHelperCodec.* from "./c6PositiveHelperCodec"
  import c6PositiveStateCodec.* from "./c6PositiveStateCodec"
  import c6PositiveStateObservation.* from "./c6PositiveStateObservation"
  import independentC6PositiveStateExpected.* from "./independentC6PositiveStateExpected"
  // The independent DATA import is used ONLY by this full-root predicate.
  run c6PositiveStateCompleteObservationsTest =
    assert(hObserveRoot(suppliedInputs) == expectedRoot)
  run c6PositiveStateRoundtripAndFramingLawsTest = assert(and {
    suppliedInputs.helper_record_cases.foldl(true,(okArg,c) => okArg and
      hDecodeHelper(hEncodeHelper(c.helper)) == c.helper and hEncodeHelper(c.helper).length() == 23),
    suppliedInputs.result_record_cases.foldl(true,(okArg,c) => okArg and
      hDecodeResult(hEncodeResult(c.result)) == c.result and hEncodeResult(c.result).length() == 4),
    suppliedInputs.task_record_cases.foldl(true,(okArg,c) => okArg and
      hDecodeTask(hEncodeTaskValue(c.task)) == c.task and hEncodeTaskValue(c.task).length() == 7),
    suppliedInputs.helper_section_cases.foldl(true,(okArg,c) => {
      val l = hEncodeTables(c.tables)
      okArg and hDecodeTables(l) == c.tables and hEncodeTables(hDecodeTables(l)) == l
        and l.length() == 31+7*c.tables.tasks.length()
        and hObserveTables(c.tables).final_cursor == l.length()
    }),
    suppliedInputs.state_cases.foldl(true,(okArg,c) => {
      val l = sEncodeState(c.state,c.context)
      val obs = sObserveState(c.state,c.context)
      okArg and sDecodeState(l) == c.state and sEncodeState(sDecodeState(l),c.context) == l
        and sStateEnd(l) == l.length() and obs.final_cursor == l.length()
        and obs.capacity_descriptor == sCapacityDescriptor(c.context.caps)
        and obs.retained_context == c.context
        and obs.child_sections.foldl(true,(allArg,partArg) => allArg and
          partArg.end == partArg.start+partArg.length and
          partArg.lanes == l.slice(partArg.start,partArg.end))
    }),
  })
}
