module independentCore2SequencePort {
  import financialBoundary.* from "./financialBoundary"
  import independentCore2SequenceSeed as S from "./independentCore2SequenceSeed"
  // A general finite admitted Invocation list; no fixture IDs or source evaluator.
  pure val suppliedQueue = S::suppliedInvocations

  pure def sameSourceContext(leftInvocation: Invocation, rightInvocation: Invocation): bool = and {
    leftInvocation.identity == rightInvocation.identity,
    leftInvocation.plan == rightInvocation.plan,
    leftInvocation.guardCosts == rightInvocation.guardCosts,
  }
  pure def startsAtIdle(invocation: Invocation): bool = match invocation.input {
    | StartInput(_) => true
    | ResumeInput(_) => false
  }
  pure def resumeFollows(previousResult: StageResult, following: Invocation): bool = match previousResult {
    | Waiting(waiting) => match following.input {
        | ResumeInput(input) => input.continuation == waiting.continuation
        | StartInput(_) => false
      }
    | _ => false
  }
  pure val queueLinked = suppliedQueue.indices().forall(i => i == 0 or
    match publish(suppliedQueue.nth(i - 1)) {
      | Result(previousResult) => resumeFollows(previousResult, suppliedQueue.nth(i))
      | NoResult => false
    })
  pure val queueDomain = if (suppliedQueue.length() == 0) false else and {
    startsAtIdle(suppliedQueue.head()),
    suppliedQueue.foldl(true, (ok, invocation) => ok and normalDomain(invocation)
      and sameSourceContext(suppliedQueue.head(), invocation)),
    queueLinked,
  }
  pure def isQueuePrefix(rows: List[Record]): bool = {
    if (rows.length() > suppliedQueue.length()) false else {
      val scanned = rows.foldl({ok: true, cursor: 0}, (acc, row) => {
        ok: acc.ok and row.invocation == suppliedQueue.nth(acc.cursor),
        cursor: acc.cursor + 1,
      })
      scanned.ok
    }
  }
  pure def linkedPrefix(rows: List[Record]): bool = {
    val scanned = rows.foldl({ok: true, previous: NoResult}, (acc, row) => match acc.previous {
      | NoResult => {ok: acc.ok, previous: Result(row.result)}
      | Result(previousResult) => {ok: acc.ok and resumeFollows(previousResult, row.invocation),
          previous: Result(row.result)}
    })
    scanned.ok
  }
  val queuePositionInv = if (machine.phase == Ready) {
    if (machine.records.length() < suppliedQueue.length())
      machine.current == suppliedQueue.nth(machine.records.length()) else false
  } else {
    if (machine.records.length() > 0 and machine.records.length() <= suppliedQueue.length())
      machine.current == suppliedQueue.nth(machine.records.length() - 1) else false
  }
  val sequenceInv = and {
    queueDomain, allInv, normalDomain(machine.current),
    isQueuePrefix(machine.records), linkedPrefix(machine.records), queuePositionInv,
  }
  action init: bool = if (suppliedQueue.length() == 0) false
    else initInvocation(suppliedQueue.head())
  action advanceInput: bool = {
    if (machine.phase != Published or machine.records.length() == 0
        or machine.records.length() >= suppliedQueue.length()) false else {
      val suppliedNext = suppliedQueue.nth(machine.records.length())
      val previousResult = machine.records.nth(machine.records.length() - 1).result
      all {
        resumeFollows(previousResult, suppliedNext),
        sameSourceContext(machine.current, suppliedNext),
        beginInvocation(suppliedNext),
      }
    }
  }
  action finalStutter: bool = all {
    machine.phase == Published, machine.records.length() == suppliedQueue.length(),
    terminalStutter,
  }
  action sequenceStep: bool = any {publishStep, advanceInput, finalStutter}
  val allInputsPublishedWitness = machine.phase == Published
    and machine.records.length() == suppliedQueue.length()
}
