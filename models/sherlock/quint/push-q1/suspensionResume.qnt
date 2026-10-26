// U014-U016: one pure machine, no network. Frames are bottom-to-top.
// The initial prefix is 10 + inner(3), immediately before entering await.
// Suffix actions collapse owner microsteps; their unit charges are abstract.
// Only await (1) and accept (1 + five guard syntax nodes) have exact /2 costs.
module suspensionResume {
  type Phase = AtAwait | Awaiting | Body | InnerReturn | Addition | OuterReturn | Publication | Finished
  type Outcome = Running | Waiting | Halted | Fuel | DataNodes | EnvironmentCells
  type Environment = str -> int
  type Frame = ReturnOuter(Environment) | AddRight(int) | ReturnInner(Environment)
  type Guard = TrueGuard | FalseGuard | ErrorGuard
  type Report = NoReport | Accepted | NotReady | Refused
  type Event = Entered | AcceptedResume | RejectedResume | RefusedResume | Cut | BodyReturned | CalleeReturned | Added | CallerReturned | Published | Exhausted | CutBeforeAdd
  type Image = {
    phase: Phase, outcome: Outcome, frames: List[Frame], env: str -> int,
    data: List[int], value: int, capture: List[int], waitSite: int,
    initial: int, remaining: int, consumed: int, helperWork: int, cursor: int,
    dataCap: int, envCap: int, epoch: int, lastObservation: int,
    program: int, effects: List[int], duties: List[int], financialCommits: int,
  }
  type Observation = { checkpointMatches: bool, programMatches: bool,
    site: int, labelMatches: bool, layoutMatches: bool, value: int, guard: Guard }
  type State = { image: Image, uncut: Image, before: Image, report: Report,
    attempted: int, seen: Set[Event], cuts: int }
  pure val ACCEPT_CHARGE = 6
  pure val good: Observation = {checkpointMatches: true, programMatches: true,
    site: 1, labelMatches: true, layoutMatches: true, value: 4, guard: TrueGuard}
  pure def genesis(fuel: int, dc: int, ec: int): Image = {
    phase: AtAwait, outcome: Running,
    frames: List(ReturnOuter(Map()), AddRight(10), ReturnInner(Map())),
    env: Map("n" -> 3), data: List(3), value: 0, capture: List(), waitSite: 0,
    initial: fuel, remaining: fuel, consumed: 0, helperWork: 0, cursor: 0,
    dataCap: dc, envCap: ec, epoch: 0, lastObservation: 0,
    program: 1, effects: List(), duties: List(), financialCommits: 0,
  }
  pure def charge(i: Image, n: int): Image = {
    ...i, remaining: i.remaining - n, consumed: i.consumed + n, cursor: i.cursor + n,
  }
  pure def canEnter(i: Image): bool = i.phase == AtAwait and i.outcome == Running
  pure def enterImage(i: Image): Image = {
    if (i.remaining == 0) { ...i, phase: Finished, outcome: Fuel }
    else {
      val paid = charge(i, 1)
      if (paid.remaining < ACCEPT_CHARGE) { ...paid, phase: Finished, outcome: Fuel }
      else if (i.dataCap - i.data.length() < 1) { ...paid, phase: Finished, outcome: DataNodes }
      else if (i.envCap - i.env.keys().size() < 1) { ...paid, phase: Finished, outcome: EnvironmentCells }
      else { ...paid, phase: Awaiting, outcome: Waiting, waitSite: 1, capture: List(i.env.get("n")) }
    }
  }
  pure def validObservation(i: Image, o: Observation): bool = and {
    i.outcome == Waiting, i.phase == Awaiting, i.waitSite == 1,
    o.programMatches, o.checkpointMatches, o.site == i.waitSite,
    o.labelMatches, o.layoutMatches,
  }
  // Equality guard uses captured n. ErrorGuard abstracts any admitted arithmetic
  // guard error; the concrete n+1 guard cannot error on the small UInt domain.
  pure def guardAccepts(i: Image, o: Observation): bool =
    if (i.capture.length() == 1) o.guard == TrueGuard and o.value == i.capture.nth(0) + 1 else false
  pure def canAccept(i: Image, o: Observation): bool = and {
    validObservation(i, o), guardAccepts(i, o), i.remaining >= ACCEPT_CHARGE,
    i.dataCap - i.data.length() >= 1, i.envCap - i.env.keys().size() >= 1,
  }
  pure def acceptImage(i: Image, o: Observation): Image = {
    val paid = charge(i, ACCEPT_CHARGE)
    { ...paid, phase: Body, outcome: Running, capture: List(), waitSite: 0,
      env: i.env.put("v", o.value), data: i.data.append(o.value),
      epoch: i.epoch + 1, lastObservation: o.value }
  }
  // An explicit structural archive roundtrip, rather than a saved frame summary.
  // Real byte encoders and digest algorithms are deliberately outside this model.
  pure def restore(i: Image): Image = {
    phase: i.phase, outcome: i.outcome, frames: i.frames, env: i.env,
    data: i.data, value: i.value, capture: i.capture, waitSite: i.waitSite,
    initial: i.initial, remaining: i.remaining, consumed: i.consumed,
    helperWork: i.helperWork, cursor: i.cursor, dataCap: i.dataCap, envCap: i.envCap,
    epoch: i.epoch, lastObservation: i.lastObservation, program: i.program,
    effects: i.effects, duties: i.duties, financialCommits: i.financialCommits,
  }
  pure def savedEnvironment(f: Frame): Environment = match f {
    | ReturnOuter(saved) => saved | ReturnInner(saved) => saved | AddRight(_) => Map()
  }
  pure def leftOperand(f: Frame): int = match f {
    | AddRight(left) => left | _ => 0
  }
  pure def canAdvance(i: Image): bool = i.outcome == Running and Set(Body, InnerReturn, Addition, OuterReturn, Publication).contains(i.phase)
  pure def advanceImage(i: Image): Image = {
    if (i.phase == Publication) { ...i, phase: Finished, outcome: Halted }
    else if (i.remaining == 0) { ...i, phase: Finished, outcome: Fuel }
    else {
      val paid = charge(i, 1)
      match i.phase {
        | Body => { ...paid, phase: InnerReturn, value: i.env.get("v") }
        | InnerReturn => { ...paid, phase: Addition, frames: i.frames.slice(0, 2), env: savedEnvironment(i.frames.nth(2)) }
        | Addition => { ...paid, phase: OuterReturn, frames: i.frames.slice(0, 1), value: leftOperand(i.frames.nth(1)) + i.value }
        | OuterReturn => { ...paid, phase: Publication, frames: List(), env: savedEnvironment(i.frames.nth(0)) }
        | _ => i
      }
    }
  }
  pure def advanceEvent(i: Image): Event =
    if (i.phase != Publication and i.remaining == 0) Exhausted
    else match i.phase {
      | Body => BodyReturned | InnerReturn => CalleeReturned | Addition => Added
      | OuterReturn => CallerReturned | Publication => Published | _ => Exhausted
    }
  var state: State
  action initWith(fuel: int, dc: int, ec: int): bool = {
    val i = genesis(fuel, dc, ec)
    state' = {image: i, uncut: i, before: i, report: NoReport,
      attempted: 0, seen: Set(), cuts: 0}
  }
  action init: bool = {
    nondet fuel = Set(0, 1, 6, 7, 11, 20).oneOf()
    nondet dc = Set(1, 2).oneOf()
    nondet ec = Set(1, 2).oneOf()
    initWith(fuel, dc, ec)
  }
  action enter: bool = all {
    canEnter(state.image),
    state' = { ...state, before: state.image, image: enterImage(state.image),
      uncut: enterImage(state.uncut), report: NoReport, attempted: 0,
      seen: state.seen.union(Set(Entered)) },
  }
  action accept(o: Observation): bool = all {
    canAccept(state.image, o),
    state' = { ...state, before: state.image, image: acceptImage(state.image, o),
      uncut: acceptImage(state.uncut, o), report: Accepted, attempted: 0,
      seen: state.seen.union(Set(AcceptedResume)) },
  }
  // A failed attempt is observable in the host report only, never in the image.
  action notReady(o: Observation): bool = all {
    validObservation(state.image, o), not(guardAccepts(state.image, o)),
    state' = { ...state, before: state.image, report: NotReady,
      attempted: ACCEPT_CHARGE, seen: state.seen.union(Set(RejectedResume)) },
  }
  action refuse(o: Observation): bool = all {
    not(validObservation(state.image, o)),
    state' = { ...state, before: state.image, report: Refused,
      attempted: 0, seen: state.seen.union(Set(RefusedResume)) },
  }
  action cut: bool = all {
    state.image.outcome == Running, state.cuts < 3,
    state' = { ...state, before: state.image, image: restore(state.image),
      report: NoReport, attempted: 0, cuts: state.cuts + 1,
      seen: state.seen.union(if (state.image.phase == Addition) Set(Cut, CutBeforeAdd) else Set(Cut)) },
  }
  action advance: bool = all {
    canAdvance(state.image),
    state' = { ...state, before: state.image, image: advanceImage(state.image),
      uncut: advanceImage(state.uncut), report: NoReport, attempted: 0,
      seen: state.seen.union(Set(advanceEvent(state.image))) },
  }
  action step: bool = any {
    enter, accept(good), notReady({ ...good, value: 5 }),
    notReady({ ...good, guard: ErrorGuard }),
    refuse({ ...good, checkpointMatches: false }),
    cut, advance,
  }
  val checkpointTransparency = state.image == state.uncut
  val fuelConservation = and {
    state.image.initial == state.image.remaining + state.image.consumed,
    state.image.cursor == state.image.consumed - state.image.helperWork,
    state.image.remaining >= 0,
  }
  val waitingCanResume = state.image.outcome == Waiting implies and {
    state.image.phase == Awaiting, state.image.waitSite == 1,
    state.image.capture == List(3), state.image.remaining >= ACCEPT_CHARGE,
    state.image.dataCap - state.image.data.length() >= 1,
    state.image.envCap - state.image.env.keys().size() >= 1,
    state.image.frames == List(ReturnOuter(Map()), AddRight(10), ReturnInner(Map())),
    canAccept(state.image, good),
  }
  val rejectedImageUnchanged = Set(NotReady, Refused).contains(state.report) implies state.image == state.before
  val atomicAccept = state.report == Accepted implies and {
    state.image.frames == state.before.frames,
    state.image.initial == state.before.initial,
    state.image.remaining == state.before.remaining - ACCEPT_CHARGE,
    state.image.consumed == state.before.consumed + ACCEPT_CHARGE,
    state.image.cursor == state.before.cursor + ACCEPT_CHARGE,
    state.image.helperWork == state.before.helperWork,
    state.image.data == state.before.data.append(4),
    state.image.epoch == state.before.epoch + 1,
    state.image.capture == List(), state.image.waitSite == 0,
    state.image.outcome == Running, state.image.phase == Body,
  }
  val continuationResult = state.image.outcome == Halted implies and {
    state.image.value == 14, state.image.frames == List(), state.image.epoch == 1,
  }
  val noFinancialCommit = and {
    state.image.effects == List(), state.image.duties == List(), state.image.financialCommits == 0,
  }
  val safety = and {checkpointTransparency, fuelConservation, waitingCanResume,
    rejectedImageUnchanged, atomicAccept, continuationResult, noFinancialCommit}
  val entered = state.seen.contains(Entered)
  val accepted = state.seen.contains(AcceptedResume)
  val rejected = state.seen.contains(RejectedResume)
  val refused = state.seen.contains(RefusedResume)
  val cutReached = state.seen.contains(Cut)
  val bodyReturned = state.seen.contains(BodyReturned)
  val calleeReturned = state.seen.contains(CalleeReturned)
  val added = state.seen.contains(Added)
  val callerReturned = state.seen.contains(CallerReturned)
  val published = state.seen.contains(Published)
  val exhausted = state.image.outcome == Fuel or state.image.outcome == DataNodes or state.image.outcome == EnvironmentCells
  val cutBeforeAddition = state.seen.contains(CutBeforeAdd)
}
