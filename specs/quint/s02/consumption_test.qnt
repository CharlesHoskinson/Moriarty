module consumption_test {
  import effects.* from "./effects"
  import consumption_harness.* from "./consumption_harness"
  import consumption.* from "./consumption"

  run firstFillTest = init.then(fillFirst).expect(
    state.usedSlots == Set(1) and state.paid == 5 and state.remainingAllowance == 5 and safety)
  run fullFillTest = init.then(fillFirst).then(fillSecond).expect(
    state.usedSlots == Set(1, 2) and state.paid == 10 and state.remainingAllowance == 0 and safety)
  run cancelBeforeFillTest = init.then(cancelParent).expect(
    state.cancelled and state.paid == 0 and state.remainingAllowance == 10 and
    not(canConsumeSlot(PARENT, state, 1, state)))
  run cancelAfterFillTest = init.then(fillFirst).then(cancelParent).expect(
    state.cancelled and state.paid == 5 and state.remainingAllowance == 5 and
    not(canConsumeSlot(PARENT, state, 2, state)))
  run staleCancelTest = init.then(fillFirst).expect(
    not(canCancelParent(PARENT, state, INITIAL)) and canCancelParent(PARENT, state, state))
  run staleFillTest = init.then(cancelParent).expect(
    not(canConsumeSlot(PARENT, state, 1, INITIAL)))
  run duplicateSlotTest = init.then(fillFirst).expect(
    not(canConsumeSlot(PARENT, state, 1, state)))
  run outOfOrderTest = init.expect(not(canConsumeSlot(PARENT, state, 2, state)))
  run substitutedParentTest = {
    val changed = {...PARENT, policy: {...PARENT.policy, mechanismVersion: 2}}
    init.then(fillFirst).expect(not(canConsumeSlot(changed, state, 2, state)))
  }
  run alteredTermsTest = {
    val changed = {...PARENT, recipient: Alice}
    init.then(fillFirst).expect(not(canConsumeSlot(changed, state, 2, state)))
  }
  run enlargedResidualTest = init.then(fillFirst).expect(
    not(validResidual(PARENT, INITIAL, 1, {...state, remainingAllowance: 10})))
  run exactResidualTest = init.then(fillFirst).expect(validResidual(PARENT, INITIAL, 1, state))
  run badRevisionTest = init.then(fillFirst).expect(not(validEntry(PARENT, {...state, revision: 0})))
  run badPaidTest = init.then(fillFirst).expect(not(validEntry(PARENT, {...state, paid: 0})))
  run duplicateCancelTest = init.then(cancelParent).expect(not(canCancelParent(PARENT, state, state)))
  run invalidSlotTest = init.expect(not(canConsumeSlot(PARENT, state, 3, state)))
}
