module observations_harness {
  import effects.* from "./effects"
  import observations.* from "./observations"

  // Declared structural examples only. This does not execute a candidate or Core.
  pure val CHOICE_IDS: Set[str] = Set("settle")
  pure val before: CoreStateObservation[str] = {
    accounts: CORE_ACCOUNTS.mapBy(_ => 0), choices: Map("settle" -> NoInt),
    continuation: "waiting", minimumTime: 0,
  }
  pure val deposit: Transfer = {
    source: Wallet(Alice), destination: Escrow({owner: Alice, asset: TokenA}),
    asset: TokenA, quantity: 10,
  }
  pure val acceptedDeposit: CoreResultObservation[str] = {
    accepted: true, error: NoCoreError, warnings: List(), payments: List(),
    state: {...before, accounts: before.accounts.put({owner: Alice, asset: TokenA}, 10), minimumTime: 1},
    reductions: 0,
  }
  pure val rejected: CoreResultObservation[str] = {
    accepted: false, error: CoreErrorCode("no_matching_input"),
    warnings: List(), payments: List(), state: before, reductions: 0,
  }
  var state: {phase: int, valid: bool}
  action init = state' = {phase: 0, valid: true}
  action inspectDeposit = all {
    state.phase == 0,
    state' = {phase: 1, valid: validCoreObservation(CHOICE_IDS, before, acceptedDeposit, List(deposit))},
  }
  action inspectRejection = all {
    state.phase == 1,
    state' = {phase: 2, valid: state.valid and validCoreObservation(CHOICE_IDS, before, rejected, List())},
  }
  action step = any {inspectDeposit, inspectRejection}
  val checkedDeposit = state.phase == 1
  val checkedRejection = state.phase == 2
  val safety = state.valid
}
