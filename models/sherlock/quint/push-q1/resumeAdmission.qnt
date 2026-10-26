// Source-order projection of factory-u suspension2::resume_pure, not a Rust FFI test.
// SI01-SI24, SI27, source/CK/layout matching and capture matching are assumed true.
// Input is a re-encoded Waiting image, not necessarily machine-produced. A public
// checksum cannot enforce reachability. Fuel/work/cursor can stay SI03/SI04-valid
// when the remaining budget is less than ACCEPT_CHARGE.
module resumeAdmission {
  type Reply = Unchecked | R0 | NotReady | RunFailure | Accepted
  type Checkpoint = {remaining: int, dataSpare: int, envSpare: int, guardTrue: bool}
  var checkpoint: Checkpoint
  var reply: Reply
  pure def resumable(c: Checkpoint): bool =
    c.remaining >= 6 and c.dataSpare >= 1 and c.envSpare >= 1
  // The design's R0 includes SI29, before evaluating the observation guard.
  pure def contractReply(c: Checkpoint): Reply =
    if (not(resumable(c))) R0 else if (c.guardTrue) Accepted else NotReady
  // Source: resume_profile2_checkpoint checks owner_runtime SI01-SI24/SI27;
  // the first SI29 resource check is resume_accept, AFTER the false-guard return.
  pure def sourceReply(c: Checkpoint): Reply =
    if (not(c.guardTrue)) NotReady else if (not(resumable(c))) RunFailure else Accepted
  action init: bool = {
    nondet remaining = Set(0, 6).oneOf()
    nondet dataSpare = Set(0, 1).oneOf()
    nondet envSpare = Set(0, 1).oneOf()
    nondet guardTrue = Set(false, true).oneOf()
    all { checkpoint' = {remaining: remaining, dataSpare: dataSpare, envSpare: envSpare, guardTrue: guardTrue}, reply' = Unchecked }
  }
  action sourceCheck: bool = all {
    reply == Unchecked, checkpoint' = checkpoint, reply' = sourceReply(checkpoint),
  }
  action contractCheck: bool = all {
    reply == Unchecked, checkpoint' = checkpoint, reply' = contractReply(checkpoint),
  }
  action step: bool = sourceCheck
  val matchesContract = reply == Unchecked or reply == contractReply(checkpoint)
  val acceptedIsAffordable = reply == Accepted implies resumable(checkpoint)
  val notReadyOnUnresumable = reply == NotReady and not(resumable(checkpoint))
  val lateResourceFailure = reply == RunFailure
}
