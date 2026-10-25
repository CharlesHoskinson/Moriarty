module suspensionResume_test {
  import suspensionResume.* from "./suspensionResume"

  run guardSyntaxChargeTest = initWith(20, 2, 2).expect(ACCEPT_CHARGE == 1 + 5)
  run pendingCalleeSnapshotTest = initWith(20, 2, 2).then(enter).expect(and {
    state.image.frames == List(ReturnOuter(Map()), AddRight(10), ReturnInner(Map())),
    state.image.env == Map("n" -> 3), state.image.data == List(3),
    state.image.capture == List(3), state.image.remaining == 19,
    state.image.consumed == 1, state.image.waitSite == 1, safety,
  })
  run acceptedResumeRetainsCallerTest = initWith(20, 2, 2).then(enter).then(accept(good)).expect(and {
    state.image.phase == Body, state.image.env.get("n") == 3,
    state.image.env.get("v") == 4, state.image.data == List(3, 4),
    state.image.consumed == 7, state.image.epoch == 1,
    state.image.lastObservation == 4, atomicAccept, safety,
  })
  run innerReturnsFourOuterReturnsFourteenTest = initWith(20, 2, 2)
    .then(enter).then(accept(good)).then(advance).then(advance)
    .expect(state.image.phase == Addition and state.image.value == 4
      and state.image.frames == List(ReturnOuter(Map()), AddRight(10)) and state.image.env == Map())
    .then(advance).then(advance).then(advance)
    .expect(state.image.outcome == Halted and state.image.value == 14 and safety)
    .then(advance.fail())
  run falseObservationByteIdentityTest = initWith(20, 2, 2).then(enter)
    .then(notReady({...good, value: 5})).expect(and {
      state.report == NotReady, state.image == state.before,
      state.attempted == 6, state.image.consumed == 1,
      state.image.epoch == 0, state.image.lastObservation == 0, safety,
    }).then(accept(good)).expect(safety)
  run guardErrorByteIdentityTest = initWith(20, 2, 2).then(enter)
    .then(notReady({...good, guard: ErrorGuard}))
    .expect(state.report == NotReady and state.image == state.before and safety)
  run repeatedNotReadyCannotConsumeResourcesTest = initWith(7, 2, 2).then(enter)
    .then(10.reps(_ => notReady({...good, value: 5})))
    .expect(state.image.remaining == 6 and state.image.consumed == 1
      and state.image.env.keys().size() == 1 and safety)
    .then(accept(good)).expect(state.image.remaining == 0 and safety)
  run localReplayIsDeterministicTest = {
    val c = enterImage(genesis(20, 2, 2))
    initWith(20, 2, 2).expect(canAccept(c, good) and acceptImage(c, good) == acceptImage(restore(c), good))
  }
  run wrongCheckpointRefusedTest = initWith(20, 2, 2).then(enter)
    .then(refuse({...good, checkpointMatches: false}))
    .expect(state.report == Refused and state.image == state.before and safety)
  run wrongProgramRefusedTest = initWith(20, 2, 2).then(enter)
    .then(refuse({...good, programMatches: false})).expect(safety)
  run wrongSiteRefusedTest = initWith(20, 2, 2).then(enter)
    .then(refuse({...good, site: 2})).expect(safety)
  run wrongLabelRefusedTest = initWith(20, 2, 2).then(enter)
    .then(refuse({...good, labelMatches: false})).expect(safety)
  run wrongLayoutRefusedTest = initWith(20, 2, 2).then(enter)
    .then(refuse({...good, layoutMatches: false})).expect(safety)
  run runningCheckpointIsRefusedTest = initWith(20, 2, 2).then(refuse(good))
    .expect(state.report == Refused and safety)
  run cannotAcceptWrongCheckpointTest = initWith(20, 2, 2).then(enter)
    .then(accept({...good, checkpointMatches: false}).fail())
  run zeroFuelAwaitEntryUnchargedTest = initWith(0, 2, 2).then(enter)
    .expect(state.image.outcome == Fuel and state.image.consumed == 0 and safety)
  run insufficientAcceptFuelNeverWaitsTest = initWith(6, 2, 2).then(enter)
    .expect(state.image.outcome == Fuel and state.image.remaining == 5
      and state.image.consumed == 1 and safety)
  run exactlyAffordableAcceptTest = initWith(7, 2, 2).then(enter)
    .expect(waitingCanResume).then(accept(good))
    .expect(state.image.remaining == 0 and state.image.phase == Body and safety)
  run dataCapacityRefusalTest = initWith(20, 1, 2).then(enter)
    .expect(state.image.outcome == DataNodes and state.image.consumed == 1 and safety)
  run environmentCapacityRefusalTest = initWith(20, 2, 1).then(enter)
    .expect(state.image.outcome == EnvironmentCells and state.image.consumed == 1 and safety)
  run resourceFailurePrecedenceTest = initWith(6, 1, 1).then(enter)
    .expect(state.image.outcome == Fuel).then(initWith(20, 1, 1)).then(enter)
    .expect(state.image.outcome == DataNodes)
  run waitingCannotAdvanceTest = initWith(20, 2, 2).then(enter).then(advance.fail())
  run cutBetweenCalleeReturnAndAdditionTest = initWith(20, 2, 2)
    .then(enter).then(accept(good)).then(advance).then(advance).then(cut)
    .expect(state.image == state.before and state.image.value == 4
      and state.image.frames == List(ReturnOuter(Map()), AddRight(10)) and cutBeforeAddition and safety)
    .then(advance).then(advance).then(advance)
    .expect(state.image.outcome == Halted and state.image.value == 14 and safety)
  run cutsDoNotInventFinancialCommitTest = initWith(20, 2, 2).then(cut)
    .then(enter).then(accept(good)).then(cut).then(advance).then(advance)
    .then(cut).then(advance).then(advance).then(advance)
    .expect(state.cuts == 3 and state.image == state.uncut and noFinancialCommit and safety)
  run zeroFuelFinalPublicationTest = initWith(11, 2, 2).then(enter).then(accept(good))
    .then(4.reps(_ => advance)).expect(state.image.remaining == 0 and state.image.phase == Publication)
    .then(advance).expect(state.image.outcome == Halted and state.image.value == 14 and safety)
  run savedCallerLexicalContextsRestoredTest = {
    val c = acceptImage(enterImage(genesis(20, 2, 2)), good)
    val withSaved = {...c, frames: List(ReturnOuter(Map("entry" -> 9)), AddRight(10), ReturnInner(Map("caller" -> 8)))}
    val returned = advanceImage(advanceImage(withSaved))
    val published = advanceImage(advanceImage(returned))
    initWith(20, 2, 2).expect(returned.env == Map("caller" -> 8) and published.env == Map("entry" -> 9))
  }
  run pendingAdditionConsumesSavedLeftOperandTest = {
    val c = advanceImage(advanceImage(acceptImage(enterImage(genesis(20, 2, 2)), good)))
    val changed = {...c, frames: List(ReturnOuter(Map()), AddRight(99))}
    initWith(20, 2, 2).expect(advanceImage(changed).value == 103)
  }
  // Mutation checks: independent expectations reject lossy checkpoints and wrong work.
  run droppedCallerFrameDetectedTest = {
    val c = enterImage(genesis(20, 2, 2))
    initWith(20, 2, 2).expect(restore(c) == c and {...restore(c), frames: List(ReturnInner(Map()))} != c)
  }
  run lostLexicalContextDetectedTest = {
    val c = enterImage(genesis(20, 2, 2))
    initWith(20, 2, 2).expect({...restore(c), env: Map()} != c)
  }
  run replenishedFuelDetectedTest = {
    val c = enterImage(genesis(20, 2, 2))
    val bad = {...c, remaining: c.remaining + 1}
    initWith(20, 2, 2).expect(bad.initial != bad.remaining + bad.consumed)
  }
}
