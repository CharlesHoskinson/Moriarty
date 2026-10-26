module resumeEpoch_test {
  import resumeEpoch.* from "./resumeEpoch"
  run ordinaryAcceptTest = initWith(0, true, true).then(sourceAccept)
    .expect(state.header == {epoch: 1, reply: Accepted} and safeHeader)
  run lastRepresentableAcceptTest = initWith(MAX_EPOCH - 1, true, true).then(sourceAccept)
    .expect(state.header == {epoch: MAX_EPOCH, reply: Accepted} and safeHeader)
  run sourceCheckedOverflowPanicsTest = initWith(MAX_EPOCH, true, true).then(sourceAccept)
    .expect(panicked and sourceBehavior and not(safeHeader))
  run sourceUncheckedOverflowWrapsTest = initWith(MAX_EPOCH, true, false).then(sourceAccept)
    .expect(wrapped and sourceBehavior and not(safeHeader))
  run falseGuardPreservesMaximumEpochTest = initWith(MAX_EPOCH, false, true).then(sourceAccept)
    .expect(state.header == {epoch: MAX_EPOCH, reply: NotReady} and safeHeader)
  run candidateCheckedIncrementRefusesTest = initWith(MAX_EPOCH, true, false).then(checkedAccept)
    .expect(state.header == {epoch: MAX_EPOCH, reply: Refused} and safeHeader)
  run committedR0RefusesMaxEpochBeforeFalseGuardTest = initWith(MAX_EPOCH, false, true).then(checkedAccept)
    .expect(state.header == {epoch: MAX_EPOCH, reply: Refused} and safeHeader)
}
