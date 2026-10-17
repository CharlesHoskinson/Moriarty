// SOURCE ONLY. Positive ordinary assertions derived from normative row/schema.
// No independent O/expected tuple, circuit producer or runtime capture used.
module bScalarTagProjection_test {
  import bScalarTagProjection.* from "./bScalarTagProjection"

  run constZeroTest = initCase(zeroConst).then(tag).expect(and {
    state.image.helper.regA == 0, state.image.helper.phase == Second,
    state.image.counters == {remainingWork: 4, consumedWork: 13, helperWork: 10, helperVisits: 4},
    preservesNamedState, positiveRowRelation, fullWidthPayload,
  })

  run dataIndexedReadTest = initCase(smallData).then(tag).expect(and {
    state.image.helper.operandA == ref(DataSpace, 1),
    state.image.helper.regA == 7, state.program.constants == List({n: 23}, {n: 29}),
    state.image.global.data == List({n: 37}, {n: 7}),
    state.image.global.frames == List({op: Add, left: ref(DataSpace, 1), origin: 7}),
    state.image.helper.operandB == SomeRef(ref(ConstSpace, 0)),
    state.image.helper.staged == NoStaged, state.image.helper.expected == NoExpected,
    positiveRowRelation, preservesNamedState,
  })

  run constAbove64Test = initCase(wideConst).then(tag).expect(and {
    state.image.helper.regA == 18446744073709551623,
    state.image.helper.regA > 18446744073709551615,
    state.image.helper.operandA == ref(ConstSpace, 1),
    state.image.global.data == List({n: 23}, {n: 29}),
    positiveRowRelation, preservesNamedState, fullWidthPayload,
  })

  run dataAbove96Test = initCase(wideData).then(tag).expect(and {
    state.image.helper.regA == 79228162514264337593543950355,
    state.image.helper.operandA == ref(DataSpace, 0),
    positiveRowRelation, preservesNamedState, fullWidthPayload,
  })

  run fullUIntPayloadTest = initCase(fullUInt).then(tag).expect(and {
    state.image.helper.regA == 340282366920938463463374607431768211455,
    state.image.helper.phase == Second, positiveRowRelation, preservesNamedState, fullWidthPayload,
  })

  run wideCounterNormalTest = initCase(wideWork).then(tag).expect(and {
    state.image.counters.remainingWork == 18446744073709551624,
    state.image.counters.consumedWork == 18446744073709551631,
    state.image.counters.helperWork == 18446744073709551628,
    state.image.counters.helperVisits == 18446744073709551624,
    state.image.helper.regA == 18446744073709551623,
    state.image.global.referenceCursor == 3,
    positiveRowRelation, preservesNamedState, fullWidthPayload,
  })

  run rowObservationTest = initCase(wideConst).then(tag).expect(and {
    match state.observation {
      | NoObservation => false
      | Observed(ee) => and {
        ee.ruleId == "b.helper.Scalar.tag", ee.eventKind == "HelperVisit", ee.disposition == "Applied",
        ee.delta == List(-1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0),
        ee.helperPhaseBefore == First, ee.helperPhaseAfter == Second,
        ee.helperCursorBefore == 0, ee.helperCursorAfter == 1,
        ee.originReferenceCursor == 3, ee.logicalDepthBefore == 1, ee.logicalDepthAfter == 1,
        ee.taskCursorBefore == 0, ee.taskCursorAfter == 0,
      }
    }, positiveRowRelation, preservesNamedState,
  })

  run normalSampleRelationTest = {
    nondet ii = cases.oneOf()
    initCase(ii).then(step).expect(and {
      reachedSecond, positiveRowRelation, preservesNamedState, fullWidthPayload,
      state.image.helper.regA == ii.payload,
      state.image.counters.remainingWork + state.image.counters.consumedWork == state.pre.global.initialFuel,
      state.image.counters.consumedWork == state.image.counters.helperWork + state.image.global.referenceCursor,
    }).then(step).expect(positiveRowRelation and preservesNamedState)
  }
}
