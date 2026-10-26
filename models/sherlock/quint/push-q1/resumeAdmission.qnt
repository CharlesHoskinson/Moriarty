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
  // Committed factory-u 815a22d87 (suspension2.rs:674-676, owner_suspend.rs:423-433)
  // calls suspend::check_resumable before admit_observation and s.guard.run, matching contractReply.
  pure def contractReply(c: Checkpoint): Reply =
    if (not(resumable(c))) R0 else if (c.guardTrue) Accepted else NotReady
  // Financial route (XFin) SI-29 counterpart in stage/sherlock2.rs:319-325,460-469 (815a22d87):
  // - At await entry (after the 1-unit commit-point charge), remaining_work == 0 exhausts Fuel
  //   rather than publishing Waiting with 0 fuel.
  // - At fin2::resume (460-469), remaining_work == 0 is refused at R0 before guard evaluation.
  type FinAwaitEntry = FinFuelExhausted | FinWaiting(int)
  pure def finAwaitEntryAfterCommitCharge(remainingAfterCommit: int): FinAwaitEntry =
    if (remainingAfterCommit <= 0) FinFuelExhausted else FinWaiting(remainingAfterCommit)
  pure def finResumeReply(remainingWork: int, guardTrue: bool): Reply =
    if (remainingWork <= 0) R0 else if (guardTrue) Accepted else NotReady
  // Pre-815a22d87 source projection (retained for regression/counterexample coverage):
  // resume_profile2_checkpoint checked owner_runtime SI01-SI24/SI27, and the first
  // SI29 resource check was resume_accept, AFTER the false-guard return.
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
