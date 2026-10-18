// SECOND INCREMENT, UNEXECUTED: one resume invocation transition.
// Finite normal events and frozen normal financial examples only.
module continuation_lifecycle {
  // Import pure declarations only: no prefix state variable, init or action.
  import continuation_prefix.Specimen from "./continuation_prefix"
  import continuation_prefix.Escrow from "./continuation_prefix"
  import continuation_prefix.Reserve from "./continuation_prefix"
  import continuation_prefix.FinancialFrame from "./continuation_prefix"
  import continuation_prefix.initialFrame from "./continuation_prefix"
  import continuation_prefix.applyStart from "./continuation_prefix"

  type NormalEvent = Delivery(bool) | Approval(int) | Settlement(bool)
  type InvocationStatus = NotInvoked | Waiting | Complete | BusinessRejected
  type ResumeEffect =
    | StateValue({ name: str, before: int, after: int })
    | TransferAmount(int)
    | DutyValue({ before: int, after: int })
    | FeeAmount(int)
    | EmitQuote(int)

  // Abstract boundary/control record c, not the serialized Rust Continuation.
  // Invocation attempts/effects do not live in c, so a false observation can
  // preserve this entire representation by assigning the same record.
  type PendingState = {
    specimen: Specimen,
    balances: str -> int,
    programState: str -> int,
    dutyPresent: bool,
    dutyAmount: int,
    site: int,
    captures: List[int],
    committed: int,
  }
  type Invocation = {
    status: InvocationStatus,
    attempted: int,
    advanced: bool,
    hasSegment: bool,
    newContinuation: bool,
    effects: List[ResumeEffect],
    sourceSteps: List[int],
    errorSteps: List[int],
  }
  type Observation = {
    c: PendingState,
    last: Invocation,
    attemptedTotal: int,
  }

  var observation: Observation

  pure def prefixProjection(f: FinancialFrame): PendingState = {
    specimen: f.specimen,
    balances: f.balances,
    programState: f.programState,
    dutyPresent: f.dutyPresent,
    dutyAmount: f.dutyAmount,
    site: f.pendingSite,
    captures: f.captureValues,
    committed: f.logicalCommitted,
  }
  pure def firstContinuation(specimen: Specimen): PendingState =
    prefixProjection(applyStart(initialFrame(specimen)))

  pure val emptyInvocation: Invocation = {
    status: NotInvoked, attempted: 0, advanced: false,
    hasSegment: false, newContinuation: false,
    effects: List(), sourceSteps: List(), errorSteps: List(),
  }

  // Finite normal event values only. No invalid-event or hostile domain.
  pure def normalEvents(c: PendingState): Set[NormalEvent] =
    if (c.specimen == Escrow) Set(Delivery(false), Delivery(true))
    else if (c.site == 1) Set(Approval(41), Approval(43), Approval(51))
    else Set(Settlement(false), Settlement(true))

  pure def canResume(c: PendingState): bool = c.site > 0

  pure def observe(o: Observation, c: PendingState, result: Invocation): Observation = {
    c: c,
    last: result,
    attemptedTotal: o.attemptedTotal + result.attempted,
  }
  pure def waitingObservation(o: Observation, attempted: int): Observation =
    observe(o, o.c, {
      status: Waiting, attempted: attempted, advanced: false,
      hasSegment: false, newContinuation: false,
      effects: List(), sourceSteps: List(), errorSteps: List(),
    })

  pure def applyInvocation(o: Observation, event: NormalEvent): Observation = {
    val c = o.c
    match event {
      | Delivery(ready) =>
        if (not(ready)) waitingObservation(o, 2)
        else {
          val amount = c.captures.nth(0)
          val post = {
            ...c,
            balances: c.balances
              .set("escrow", c.balances.get("escrow") - amount)
              .set("bob", c.balances.get("bob") + amount),
            dutyAmount: c.dutyAmount - amount,
            site: 0, captures: List(), committed: c.committed + 6,
          }
          observe(o, post, {
            status: Complete, attempted: 6, advanced: true,
            hasSegment: true, newContinuation: false,
            effects: List(TransferAmount(amount), DutyValue({ before: c.dutyAmount, after: c.dutyAmount - amount })),
            sourceSteps: List(2, 3, 4), errorSteps: List(),
          })
        }
      | Approval(quote) => {
          val held = c.captures.nth(1)
          if (quote < held) waitingObservation(o, 4)
          else {
            val post = {
              ...c,
              balances: c.balances
                .set("reserve", c.balances.get("reserve") - held)
                .set("target", c.balances.get("target") + held),
              programState: c.programState
                .set("allocated", held)
                .set("basis", c.programState.get("basis") + quote),
              dutyAmount: c.dutyAmount - held,
              site: 2, captures: c.captures.append(quote),
              committed: c.committed + 15,
            }
            observe(o, post, {
              status: Waiting, attempted: 15, advanced: true,
              hasSegment: true, newContinuation: true,
              effects: List(
                StateValue({ name: "allocated", before: c.programState.get("allocated"), after: held }),
                TransferAmount(held),
                DutyValue({ before: c.dutyAmount, after: c.dutyAmount - held }),
                StateValue({ name: "basis", before: c.programState.get("basis"), after: c.programState.get("basis") + quote })
              ),
              sourceSteps: List(3, 4, 5, 6, 7), errorSteps: List(),
            })
          }
        }
      | Settlement(ready) =>
        if (not(ready)) waitingObservation(o, 2)
        else {
          val quote = c.captures.nth(2)
          if (quote > 50)
            // An ordinary commercial predicate refuses the suffix. No segment
            // is published, while invocation work is still observed separately.
            observe(o, c, {
              status: BusinessRejected, attempted: 6, advanced: false,
              hasSegment: false, newContinuation: false,
              effects: List(), sourceSteps: List(), errorSteps: List(9),
            })
          else {
            val extra = c.captures.nth(0)
            val post = {
              ...c,
              balances: c.balances
                .set("reserve", c.balances.get("reserve") - extra)
                .set("treasury", c.balances.get("treasury") + extra),
              site: 0, captures: List(), committed: c.committed + 10,
            }
            observe(o, post, {
              status: Complete, attempted: 10, advanced: true,
              hasSegment: true, newContinuation: false,
              effects: List(FeeAmount(extra), EmitQuote(quote)),
              sourceSteps: List(8, 9, 10, 11), errorSteps: List(),
            })
          }
        }
    }
  }

  action initCase(specimen: Specimen): bool = {
    val c = firstContinuation(specimen)
    observation' = { c: c, last: emptyInvocation, attemptedTotal: c.committed }
  }
  action init: bool = {
    nondet specimen = Set(Escrow, Reserve).oneOf()
    initCase(specimen)
  }

  // The only new business transition. A false event is an observable invocation
  // updating last/attemptedTotal, not a blanket unchanged-state fallback.
  action resume(event: NormalEvent): bool = all {
    canResume(observation.c),
    normalEvents(observation.c).contains(event),
    observation' = applyInvocation(observation, event),
  }
  action step: bool = {
    nondet event = normalEvents(observation.c).oneOf()
    resume(event)
  }

  // Positive targets only; no security invariant or adversarial test is added.
  val resumeObserved: bool = observation.last.status != NotInvoked
  val falseObservationReached: bool =
    observation.last.status == Waiting and not(observation.last.hasSegment)
  val completionReached: bool = observation.last.status == Complete
  val businessRejectionReached: bool = observation.last.status == BusinessRejected
}
