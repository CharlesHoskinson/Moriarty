// U017-U020: statement/commit boundaries of stage/sherlock2.rs.
// Source correspondence, finite domain and known design disagreement: README.md.
// Premises: admitted, well-typed scalar statements and valid static cost plan;
// one asset, three distinct accounts, named present duty, opening 100/40.
// Host fuel is one per statement/commit/accept; logical expression costs and
// R0-R7, capture/hash identity, native phases and cryptography are abstracted.
// Read-only source cut: factory-u uncommitted sherlock2.rs SHA256 6054a66c...
module financialSegments {
  type Policy = AllOrNothing | RetainFees
  type Outcome = Running | Waiting | Halted | Rejected | Exhausted
  type Statement = Pay(int) | Fee(int) | Discharge(int) | Requires(bool)
  type Effect = Payment(int) | FeePaid(int) | Discharged(int)
  type Snapshot = {reserve: int, recipient: int, feePool: int, liability: int}
  type State = {
    policy: Policy, program: List[Statement], pc: int, awaitEnd: bool,
    outcome: Outcome, entry: Snapshot, committed: Snapshot, tentative: Snapshot,
    entryEffects: List[Effect], committedEffects: List[Effect],
    feeEffects: List[Effect], fallibleEffects: List[Effect],
    discarded: List[Effect], retained: List[Effect],
    feeSealed: bool, feeSnapshot: Snapshot, sealAt: int,
    initialFuel: int, remaining: int, consumed: int, committedEvents: int,
    cuts: int, notReady: int, failedSegment: bool,
  }
  var image: State
  pure val genesis: Snapshot = {reserve: 100, recipient: 0, feePool: 0, liability: 40}
  pure val firstPrefix = List(Pay(30), Fee(2), Discharge(30))
  pure val secondPrefix = List(Pay(10), Fee(1), Discharge(10))
  pure val retainedFailure = List(Fee(1), Pay(10), Discharge(10), Requires(false))
  pure def phaseEnd(program: List[Statement]): int = program.foldl(
    {cursor: 0, last: 0}, (a, stmt) => {
      cursor: a.cursor + 1,
      last: match stmt { | Fee(_) => a.cursor + 1 | _ => a.last },
    }).last
  pure def policyAdmits(program: List[Statement], policy: Policy): bool =
    policy == AllOrNothing or program.foldl({fallible: false, valid: true}, (a, stmt) =>
      match stmt {
        | Fee(_) => {...a, valid: a.valid and not(a.fallible)}
        | Requires(_) => a
        | _ => {...a, fallible: true}
      }).valid
  pure def initial(program: List[Statement], policy: Policy, fuel: int, awaitEnd: bool): State = {
    policy: policy, program: program, pc: 0, awaitEnd: awaitEnd, outcome: Running,
    entry: genesis, committed: genesis, tentative: genesis,
    entryEffects: List(), committedEffects: List(), feeEffects: List(),
    fallibleEffects: List(), discarded: List(), retained: List(),
    feeSealed: policy == AllOrNothing or phaseEnd(program) == 0,
    feeSnapshot: genesis, sealAt: if (policy == RetainFees) phaseEnd(program) else 0,
    initialFuel: fuel, remaining: fuel, consumed: 0, committedEvents: 0,
    cuts: 0, notReady: 0, failedSegment: false,
  }
  pure def seal(s: State): State =
    if (not(s.feeSealed) and s.pc == s.sealAt)
      {...s, feeSealed: true, feeSnapshot: s.tentative}
    else s
  pure def charged(s: State): State = {...s, remaining: s.remaining - 1, consumed: s.consumed + 1}
  pure def failed(s: State, outcome: Outcome): State =
    if (s.policy == RetainFees and s.feeSealed) {
      ...s, outcome: outcome, failedSegment: true, committed: s.feeSnapshot,
      committedEffects: s.entryEffects.concat(s.feeEffects),
      retained: s.feeEffects, discarded: s.fallibleEffects,
    } else {
      ...s, outcome: outcome, failedSegment: true, retained: List(),
      discarded: s.feeEffects.concat(s.fallibleEffects),
    }
  pure def executable(s: Snapshot, stmt: Statement): bool = match stmt {
    | Pay(n) => n >= 0 and n <= s.reserve
    | Fee(n) => n >= 0 and n <= s.reserve
    | Discharge(n) => n >= 0 and n <= s.liability
    | Requires(b) => b
  }
  pure def execute(s: Snapshot, stmt: Statement): Snapshot = match stmt {
    | Pay(n) => {...s, reserve: s.reserve - n, recipient: s.recipient + n}
    | Fee(n) => {...s, reserve: s.reserve - n, feePool: s.feePool + n}
    | Discharge(n) => {...s, liability: s.liability - n}
    | Requires(_) => s
  }
  pure def effects(stmt: Statement): List[Effect] = match stmt {
    | Pay(n) => List(Payment(n)) | Fee(n) => List(FeePaid(n))
    | Discharge(n) => List(Discharged(n)) | Requires(_) => List()
  }
  pure def statementNext(s: State): State = {
    val ready = seal(s)
    if (ready.remaining == 0) failed(ready, Exhausted) else {
      val c = charged(ready)
      val stmt = c.program.nth(c.pc)
      if (not(executable(c.tentative, stmt))) failed(c, Rejected)
      else {
        ...c, pc: c.pc + 1, tentative: execute(c.tentative, stmt),
        feeEffects: if (c.feeSealed) c.feeEffects else c.feeEffects.concat(effects(stmt)),
        fallibleEffects: if (c.feeSealed) c.fallibleEffects.concat(effects(stmt)) else c.fallibleEffects,
      }
    }
  }
  pure def commitNext(s: State): State = {
    val ready = seal(s)
    if (ready.remaining == 0) failed(ready, Exhausted) else {
      val c = charged(ready)
      {...c, committed: c.tentative,
        committedEffects: c.entryEffects.concat(c.feeEffects).concat(c.fallibleEffects),
        committedEvents: c.consumed, outcome: if (c.awaitEnd) Waiting else Halted}
    }
  }
  action initCase(program: List[Statement], policy: Policy, fuel: int, awaitEnd: bool): bool = all {
    policyAdmits(program, policy), fuel >= 0,
    image' = initial(program, policy, fuel, awaitEnd),
  }
  action init: bool = {
    nondet program = Set(firstPrefix, List(Fee(10), Fee(100)), retainedFailure,
      List(Fee(1), Discharge(41)), List(Fee(2)), List(Requires(false))).oneOf()
    nondet policy = Set(AllOrNothing, RetainFees).oneOf()
    nondet fuel = 0.to(10).oneOf()
    nondet awaitEnd = Set(false, true).oneOf()
    // Policy-invalid inputs are admission refusals, outside the image domain.
    image' = initial(program, if (policyAdmits(program, policy)) policy else AllOrNothing, fuel, awaitEnd)
  }
  action statementStep: bool = all {
    image.outcome == Running, image.pc < image.program.length(), image' = statementNext(image),
  }
  action commitStep: bool = all {
    image.outcome == Running, image.pc == image.program.length(), image' = commitNext(image),
  }
  action resumeWith(program: List[Statement], awaitEnd: bool): bool = all {
    image.outcome == Waiting, policyAdmits(program, image.policy),
    image' = if (image.remaining == 0) {
      ...image, outcome: Exhausted, entry: image.committed,
      entryEffects: image.committedEffects,
    } else {
      ...charged(image), outcome: Running, program: program, pc: 0, awaitEnd: awaitEnd,
      entry: image.committed, tentative: image.committed, entryEffects: image.committedEffects,
      feeEffects: List(), fallibleEffects: List(),
      feeSealed: image.policy == AllOrNothing or phaseEnd(program) == 0,
      sealAt: if (image.policy == RetainFees) phaseEnd(program) else 0,
      feeSnapshot: image.committed,
    },
  }
  action notReadyStep: bool = all {
    image.outcome == Waiting, image' = {...image, notReady: image.notReady + 1},
  }
  // Host execution has no persisted mid-segment images; this observer records
  // a possible technical cut, with financial state and work unchanged.
  action cutStep: bool = all {
    image.outcome == Running, image' = {...image, cuts: image.cuts + 1},
  }
  action terminalStep: bool = all {
    Set(Halted, Rejected, Exhausted).contains(image.outcome), image' = image,
  }
  action step: bool = any {
    statementStep, commitStep, resumeWith(secondPrefix, false),
    resumeWith(retainedFailure, false), notReadyStep, cutStep, terminalStep,
  }
  pure def replay(log: List[Effect]): Snapshot = log.foldl(genesis, (s, e) => match e {
    | Payment(n) => execute(s, Pay(n))
    | FeePaid(n) => execute(s, Fee(n))
    | Discharged(n) => execute(s, Discharge(n))
  })
  pure def validSnapshot(s: Snapshot): bool = and {
    s.reserve >= 0, s.recipient >= 0, s.feePool >= 0, s.liability >= 0,
    s.reserve + s.recipient + s.feePool == 100,
  }
  val conservationInv = validSnapshot(image.committed) and validSnapshot(image.tentative)
  val committedLogInv = image.committed == replay(image.committedEffects)
  val feeCountedOnceInv = 100 - image.committed.reserve ==
    image.committed.recipient + image.committed.feePool
  val workInv = image.remaining >= 0 and image.consumed >= image.committedEvents
    and image.remaining + image.consumed == image.initialFuel
  val stagingInv = (image.outcome == Running) implies image.committed == image.entry
  val rollbackInv = Set(Rejected, Exhausted).contains(image.outcome) implies
    (image.committed == image.entry or
      (image.policy == RetainFees and image.feeSealed and image.committed == image.feeSnapshot))
  val failureEffectsInv = image.failedSegment implies
    (if (image.policy == RetainFees and image.feeSealed)
      image.retained == image.feeEffects and image.discarded == image.fallibleEffects
        and image.committedEffects == image.entryEffects.concat(image.feeEffects)
    else image.retained == List() and image.discarded == image.feeEffects.concat(image.fallibleEffects)
      and image.committedEffects == image.entryEffects)
  val unsealedInv = not(image.feeSealed) implies image.fallibleEffects == List()
  val allInvariants = conservationInv and committedLogInv and feeCountedOnceInv
    and workInv and stagingInv and rollbackInv and failureEffectsInv and unsealedInv
  // Design §6.1/SI-29, intentionally separate from current-code invariants.
  // Even one resume-accept event needs positive remaining fuel.
  val waitingResumableInv = (image.outcome == Waiting) implies image.remaining > 0
  val paidPrefix = image.outcome == Waiting and image.committed.reserve == 68
    and image.committed.liability == 10
  val retainedFee = image.retained.length() > 0
  val rejected = image.outcome == Rejected
  val exhausted = image.outcome == Exhausted
  val completed = image.outcome == Halted
  val cutObserved = image.cuts > 0
  val notReadyObserved = image.notReady > 0
}
