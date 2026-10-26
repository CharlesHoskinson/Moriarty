module resumeAdmission_test {
  import resumeAdmission.* from "./resumeAdmission"
  pure val starved = {remaining: 0, dataSpare: 1, envSpare: 1, guardTrue: false}
  run falseGuardSkipsSI29SourceCounterexampleTest = init.expect(
    contractReply(starved) == R0 and sourceReply(starved) == NotReady)
  run trueGuardChecksSI29TooLateTest = init.expect(
    contractReply({...starved, guardTrue: true}) == R0
      and sourceReply({...starved, guardTrue: true}) == RunFailure)
  run machineProducedCheckpointAgreementTest = init.expect(
    Set(false, true).forall(g => {
      val c = {remaining: 6, dataSpare: 1, envSpare: 1, guardTrue: g}
      contractReply(c) == sourceReply(c)
    }))
  run contractRejectsAllStarvedInputsTest = init.expect(
    Set(0, 6).forall(f => Set(0, 1).forall(d => Set(0, 1).forall(e => {
      val c = {remaining: f, dataSpare: d, envSpare: e, guardTrue: false}
      resumable(c) or contractReply(c) == R0
    }))))
}
