// SOURCE ONLY: normal model tests, all UNRUN. Expected local transitions are
// hand-derived directly from normative rows, not host execution or frozen O.
module bValidateProjection_test {
  import bValidateProjection.* from "./bValidateProjection"

  run scalarEntryTest = initCase(uintInput, normalLimits)
    .then(step).expect(and {
      state.machine.control == Eval(0), state.machine.result == NoResult,
      state.machine.tasks == List({ref: Const(0), visit: Unvisited, cursor: 0}),
      state.machine.helper == ValidateHelper({site: "EvalLiteral", originReferenceCursor: 0,
        parentResume: Absent, root: Const(0), status: Pending}),
      state.machine.consumedWork == 1, state.machine.helperWork == 1,
      state.machine.helperVisits == 0, state.machine.referenceCursor == 0, allInv,
      match state.lastEvent { | NoEvent => false | StepEvent(ee) => and {
        ee.ruleId == "b.helper.Validate.entry", ee.helperKind == ValidateKind,
        ee.originReferenceCursor == Present(0), ee.helperCursorBefore == Absent,
        ee.helperCursorAfter == Present(1), ee.helperPhaseBefore == NoPhase,
        ee.helperPhaseAfter == Phase(Pending), ee.resultTag == NoResultTag,
      } },
    })

  run scalarOccurrenceEmptyExitTest = initCase(uintInput, normalLimits)
    .then(step).then(step).expect(and {
      state.machine.tasks == List(), state.machine.helperVisits == 1,
      state.machine.referenceCursor == 0, allInv,
    })
    .then(step).expect(state.machine.helper == ValidateHelper({site: "EvalLiteral",
      originReferenceCursor: 0, parentResume: Absent, root: Const(0), status: Positive}))
    .then(step).expect(and {
      state.machine.helper == NoHelper, state.machine.result == Validated(true),
      state.machine.control == Eval(0), state.machine.helperWork == 4,
      state.machine.helperVisits == 1, state.machine.referenceCursor == 0,
      match state.lastEvent { | NoEvent => false | StepEvent(ee) => and {
        ee.ruleId == "b.helper.Validate.exit", ee.resultTag == ValidatedTag,
        ee.helperKind == ValidateKind, ee.originReferenceCursor == Present(0),
        ee.helperCursorBefore == Present(0), ee.helperCursorAfter == Absent,
        ee.helperPhaseBefore == Phase(Positive), ee.helperPhaseAfter == NoPhase,
      } }, allInv,
    })

  run literalCompletionPublishTest = initCase(uintInput, normalLimits)
    .then(4.reps(_ => step)).then(step).expect(and {
      state.machine.control == Return(Const(0)), state.machine.result == NoResult,
      state.machine.outcome == Running, state.machine.referenceCursor == 1,
      state.machine.consumedWork == 5, state.machine.helperWork == 4,
      match state.lastEvent { | NoEvent => false | StepEvent(ee) => and {
        ee.ruleId == "k.machine.eval.literal.valid", ee.workDelta == 1,
        ee.helperWorkDelta == 0, ee.referenceCursorDelta == 1,
        ee.helperKind == ValidateKind, ee.originReferenceCursor == Present(0),
        ee.resultTag == ValidatedTag, ee.helperCursorBefore == Absent,
        ee.helperCursorAfter == Absent, ee.helperPhaseBefore == NoPhase,
        ee.stepId.ordinal == 4, ee.sourceExprId == 0,
      } }, allInv,
    })
    .then(step).expect(and {
      state.machine.outcome == Halted(Const(0)), state.machine.consumedWork == 5,
      match state.lastEvent { | NoEvent => false | StepEvent(ee) => and {
        ee.ruleId == "k.admin.publish.value", ee.eventKind == Publish,
        ee.disposition == Published, ee.workDelta == 0, ee.helperWorkDelta == 0,
        ee.helperVisitsDelta == 0, ee.referenceCursorDelta == 0,
        ee.helperKind == NoHelperKind, ee.originReferenceCursor == Absent,
        ee.resultTag == NoResultTag, ee.stepId.ordinal == 5, ee.boundary == Terminal,
      } }, allInv,
    }).then(step).expect(state.lastEvent == NoEvent and state.machine.outcome == Halted(Const(0)))

  run leafTailReplacementTest = initCase(leafInput, normalLimits)
    .then(step).then(step).expect(state.machine.tasks == List({ref: Const(1), visit: Fields, cursor: 0}))
    .then(step).expect(and {
      state.machine.tasks == List({ref: Const(0), visit: Unvisited, cursor: 0}),
      state.machine.helperVisits == 2, allInv,
    })

  run consOrderedNonTailTest = initCase(listInput, normalLimits)
    .then(step).then(step).then(step).expect(and {
      state.machine.tasks == List({ref: Const(2), visit: Fields, cursor: 1},
        {ref: Const(0), visit: Unvisited, cursor: 0}),
      state.machine.helperVisits == 2, allInv,
    })
    .then(step).then(step).expect(and {
      state.machine.tasks == List({ref: Const(1), visit: Unvisited, cursor: 0}),
      state.machine.helperVisits == 4, allInv,
    })

  run repeatedOccurrenceTest = initCase(repeatedInput, normalLimits)
    .then(10.reps(_ => step)).expect(and {
      state.machine.tasks == List(), state.machine.helperVisits == 9,
      state.machine.helperWork == 10, state.machine.referenceCursor == 0, allInv,
    })
    .then(step).then(step).then(step).then(step).expect(and {
      state.machine.outcome == Halted(Const(2)), state.machine.consumedWork == 13,
      state.machine.helperWork == 12, state.machine.helperVisits == 9,
      state.machine.referenceCursor == 1, allInv,
    })

  run normalProgramPublicationTest = {
    nondet ii = cases.oneOf()
    initCase(ii, normalLimits).then(20.reps(_ => step))
      .expect(state.machine.outcome == Halted(root(ii)) and allInv)
  }
}
