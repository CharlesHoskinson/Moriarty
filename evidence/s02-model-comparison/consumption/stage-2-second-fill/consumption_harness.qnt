module consumption_harness {
  import effects.* from "./effects"
  import consumption.* from "./consumption"

  type FixturePolicy = {mechanismVersion: int, acceptedDisclosure: bool}
  pure val PARENT: Parent[FixturePolicy] = {
    key: {domain: InstallmentDomain, principal: Alice, nonce: 0},
    policy: {mechanismVersion: 1, acceptedDisclosure: false},
    source: Escrow({owner: Alice, asset: TokenA}), recipient: Bob,
    asset: TokenA, budget: 10, slots: Map(1 -> 5, 2 -> 5),
  }
  pure val INITIAL: Entry[FixturePolicy] = initialEntry(PARENT)

  var state: Entry[FixturePolicy]

  action init = state' = INITIAL
  action fillFirst = all {
    canConsumeSlot(PARENT, state, 1, state),
    state' = applyConsumeSlot(PARENT, state, 1),
  }
  action fillSecond = all {
    canConsumeSlot(PARENT, state, 2, state),
    state' = applyConsumeSlot(PARENT, state, 2),
  }
  action step = any { fillFirst, fillSecond }

  val firstFilled = state.usedSlots == Set(1) and not(state.cancelled)
  val fullyFilled = state.usedSlots == Set(1, 2) and state.paid == 10
  val terminal = state.cancelled or state.remainingAllowance == 0
  val safety = validEntry(PARENT, state) and state.paid + state.remainingAllowance == 10 and
    (not(state.cancelled) or not(canConsumeSlot(PARENT, state, 1, state)) and not(canConsumeSlot(PARENT, state, 2, state))) and
    (terminal or canConsumeSlot(PARENT, state, 1, state) or canConsumeSlot(PARENT, state, 2, state) or canCancelParent(PARENT, state, state))
}
