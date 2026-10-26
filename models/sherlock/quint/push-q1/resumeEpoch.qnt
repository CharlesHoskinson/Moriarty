// Imported-header arithmetic projection, not an executed Rust test.
// Header::get reads u64 without a reachability check; both resume routes use +1.
// Model both Rust overflow modes explicitly, without relying on Quint overflow.
module resumeEpoch {
  type Reply = Pending | Accepted | NotReady | Refused | Panic
  type Header = { epoch: int, reply: Reply }
  type State = { before: int, header: Header, guardTrue: bool, overflowChecks: bool }
  pure val MAX_EPOCH = 18446744073709551615
  pure def sourceIncrement(epoch: int, overflowChecks: bool): Header =
    if (epoch < MAX_EPOCH) {epoch: epoch + 1, reply: Accepted}
    else if (overflowChecks) {epoch: epoch, reply: Panic}
    else {epoch: 0, reply: Accepted}
  // Candidate remedy leaves the input checkpoint intact and refuses overflow.
  // Refusal stage is deliberately unspecified: the current design does not pin it.
  pure def checkedIncrement(epoch: int): Header =
    if (epoch < MAX_EPOCH) {epoch: epoch + 1, reply: Accepted}
    else {epoch: epoch, reply: Refused}
  pure def sourceResult(s: State): Header =
    if (s.guardTrue) sourceIncrement(s.before, s.overflowChecks)
    else {epoch: s.before, reply: NotReady}
  pure def checkedResult(s: State): Header =
    if (s.guardTrue) checkedIncrement(s.before)
    else {epoch: s.before, reply: NotReady}
  var state: State
  action initWith(epoch: int, ready: bool, checks: bool): bool =
    state' = {before: epoch, header: {epoch: epoch, reply: Pending},
      guardTrue: ready, overflowChecks: checks}
  action init: bool = {
    nondet epoch = Set(0, MAX_EPOCH - 1, MAX_EPOCH).oneOf()
    nondet ready = Set(false, true).oneOf()
    nondet checks = Set(false, true).oneOf()
    initWith(epoch, ready, checks)
  }
  action sourceAccept: bool = all {
    state.header.reply == Pending,
    state' = {...state, header: sourceResult(state)},
  }
  action checkedAccept: bool = all {
    state.header.reply == Pending,
    state' = {...state, header: checkedResult(state)},
  }
  action step: bool = sourceAccept
  val acceptedEpochAdvanced = state.header.reply == Accepted implies and {
    state.header.epoch == state.before + 1,
    state.header.epoch <= MAX_EPOCH,
  }
  val noPanic = state.header.reply != Panic
  val notReadyIdentity = state.header.reply == NotReady implies state.header.epoch == state.before
  val safeHeader = acceptedEpochAdvanced and noPanic and notReadyIdentity
  val sourceBehavior = and {
    state.header.epoch >= 0, state.header.epoch <= MAX_EPOCH, notReadyIdentity,
    state.header.reply == Accepted implies
      (state.header.epoch == state.before + 1 or
       (state.before == MAX_EPOCH and not(state.overflowChecks) and state.header.epoch == 0)),
    state.header.reply == Panic implies state.before == MAX_EPOCH and state.overflowChecks,
  }
  val wrapped = state.header.reply == Accepted and state.before == MAX_EPOCH and state.header.epoch == 0
  val panicked = state.header.reply == Panic
  val acceptedWithoutOverflow = state.header.reply == Accepted and state.header.epoch == state.before + 1
  val notReady = state.header.reply == NotReady
  val refusedOverflow = state.header.reply == Refused
}
