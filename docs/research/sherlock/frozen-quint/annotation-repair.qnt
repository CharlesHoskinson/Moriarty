module universalMachine {
  // Finite algorithm/control abstraction. This is not the K expression evaluator.
  type Phase = Running | Waiting | Halted | Rejected | Exhausted
  type Program = Factorial | ListSum | TreeSum
  type Scenario = FactorialCase | ListCase | TreeCase | YieldCase | ExhaustCase | BoundCase
  type Control = Evaluate | ReturnValue | Finished
  type Node = Nil | Cons({head: int, tail: int}) | Leaf(int) | Branch({left: int, right: int})
  type Frame = Multiply(int) | AddHead(int) | VisitRight(int) | AddLeft(int)
  type Pending = NoPending | PendingReturn(int)
  type Result = NoResult | ResultValue(int)
  type Reason = NoReason | ConstructorMismatch | ScalarBound | StackBound | MissingReference
  type Snapshot = {acc: int, frames: List[Frame], arg: int, cursor: int, consumed: int}
  type Machine = {
    phase: Phase, program: Program, control: Control,
    arg: int, cursor: int, acc: int, frames: List[Frame], heap: int -> Node,
    total: int, remaining: int, consumed: int, attempted: int,
    yieldEnabled: bool, yielded: bool, pending: Pending,
    observations: int, acceptedEvents: int, boundary: int, checkpoint: Snapshot,
    result: Result, reason: Reason, maxDepth: int, visited: Set[int],
  }

  pure val MAX_SCALAR = 15
  pure val MAX_FRAMES = 8
  pure val HEAP_IDS = 0.to(7)
  pure val SCENARIOS = Set(FactorialCase, ListCase, TreeCase, YieldCase, ExhaustCase, BoundCase)
  var machine: Machine

  pure val initialHeap: int -> Node = Map(
    0 -> Nil,
    1 -> Cons({head: 3, tail: 0}),
    2 -> Cons({head: 2, tail: 1}),
    3 -> Cons({head: 1, tail: 2}),
    4 -> Leaf(4),
    5 -> Leaf(5),
    6 -> Nil,
    7 -> Branch({left: 4, right: 5})
  )

  pure def initial(sc: Scenario): Machine = {
    val budget = if (sc == ExhaustCase) 2 else 32
    {
      phase: Running,
      program: if (sc == ListCase) ListSum else if (sc == TreeCase) TreeSum else Factorial,
      control: Evaluate,
      arg: if (sc == BoundCase) 4 else 3,
      cursor: if (sc == TreeCase) 7 else 3,
      acc: 0, frames: List(), heap: initialHeap,
      total: budget, remaining: budget, consumed: 0, attempted: 0,
      yieldEnabled: sc == YieldCase, yielded: false, pending: NoPending,
      observations: 0, acceptedEvents: 0, boundary: 0,
      checkpoint: {acc: 0, frames: List(), arg: 0, cursor: 0, consumed: 0},
      result: NoResult, reason: NoReason, maxDepth: 0, visited: Set(),
    }
  }

  pure def isTerminal(s: Machine): bool =
    Set(Halted, Rejected, Exhausted).contains(s.phase)

  pure def inScalar(n: int): bool = n >= 0 and n <= MAX_SCALAR

  pure def charged(s: Machine): Machine = {
    ...s, remaining: s.remaining - 1,
    consumed: s.consumed + 1, attempted: s.attempted + 1,
  }

  pure def refuse(s: Machine, r: Reason): Machine =
    {...s, phase: Rejected, control: Finished, result: NoResult, reason: r}

  pure def push(s: Machine, f: Frame): Machine = {
    val depth = s.frames.length() + 1
    if (depth > MAX_FRAMES) refuse(s, StackBound)
    else {
      ...s, frames: List(f).concat(s.frames),
      maxDepth: if (depth > s.maxDepth) depth else s.maxDepth,
    }
  }

  pure def evaluateFactorial(s: Machine): Machine =
    if (s.arg == 0) {...s, acc: 1, control: ReturnValue}
    else {
      val pushed = push(s, Multiply(s.arg))
      {...pushed, arg: s.arg - 1}
    }

  pure def evaluateListNode(s: Machine, node: Node): Machine =
    match node {
      | Nil => {...s, acc: 0, control: ReturnValue}
      | Cons(c) => {
        val pushed = push(s, AddHead(c.head))
        {...pushed, cursor: c.tail}
      }
      | _ => refuse(s, ConstructorMismatch)
    }

  pure def evaluateTreeNode(s: Machine, node: Node): Machine =
    match node {
      | Leaf(v) => {...s, acc: v, control: ReturnValue}
      | Branch(b) => {
        val pushed = push(s, VisitRight(b.right))
        {...pushed, cursor: b.left}
      }
      | _ => refuse(s, ConstructorMismatch)
    }

  pure def evaluateData(s: Machine): Machine =
    if (not(s.heap.keys().contains(s.cursor))) refuse(s, MissingReference)
    else {
      val marked = {...s, visited: s.visited.union(Set(s.cursor))}
      val node = s.heap.get(s.cursor)
      if (s.program == ListSum) evaluateListNode(marked, node)
      else evaluateTreeNode(marked, node)
    }

  pure def evaluate(s: Machine): Machine =
    if (s.program == Factorial) evaluateFactorial(s) else evaluateData(s)

  pure def checkedAccumulator(s: Machine, value: int): Machine =
    if (inScalar(value)) {...s, acc: value}
    else refuse(s, ScalarBound)

  pure def applyFrame(s: Machine, frame: Frame): Machine =
    match frame {
      | Multiply(n) => checkedAccumulator(s, n * s.acc)
      | AddHead(n) => checkedAccumulator(s, n + s.acc)
      | VisitRight(ref) => {
        val pushed = push(s, AddLeft(s.acc))
        {...pushed, cursor: ref, control: Evaluate}
      }
      | AddLeft(n) => checkedAccumulator(s, n + s.acc)
    }

  pure def returnOne(s: Machine): Machine =
    if (s.frames.length() == 0)
      {...s, phase: Halted, control: Finished, result: ResultValue(s.acc)}
    else {
      val frame = s.frames.head()
      val popped = {...s, frames: s.frames.tail()}
      applyFrame(popped, frame)
    }

  pure def suspendReturn(s: Machine): Machine = {
    ...s, phase: Waiting, yielded: true, pending: PendingReturn(s.acc),
    boundary: s.boundary + 1,
    checkpoint: {
      acc: s.acc, frames: s.frames, arg: s.arg,
      cursor: s.cursor, consumed: s.consumed,
    },
  }

  pure def canTick(s: Machine): bool = s.phase == Running

  pure def applyTick(s: Machine): Machine =
    if (s.remaining == 0)
      {...s, phase: Exhausted, control: Finished, attempted: s.attempted + 1}
    else {
      val next = charged(s)
      if (s.control == Evaluate) evaluate(next)
      else if (s.control == ReturnValue) {
        if (s.yieldEnabled and not(s.yielded) and s.frames.length() > 0)
          suspendReturn(next)
        else returnOne(next)
      }
      else refuse(next, ConstructorMismatch)
    }

  // Concrete demonstration bound: at most one not-ready observation before a ready event.
  pure def canObserve(s: Machine, ready: bool): bool =
    s.phase == Waiting and (ready or s.observations == 0)

  pure def resumePending(s: Machine): Machine =
    match s.pending {
      | PendingReturn(value) => {
        ...s, phase: Running, acc: value, pending: NoPending,
        acceptedEvents: s.acceptedEvents + 1,
      }
      | NoPending => s
    }

  // ready is only the modeled event condition, not authentication or proof validity.
  pure def applyObservation(s: Machine, ready: bool): Machine = {
    val observed = {...s, observations: s.observations + 1}
    if (ready) resumePending(observed) else observed
  }

  action initCase(sc: Scenario): bool = machine' = initial(sc)
  action init = {
    nondet sc = SCENARIOS.oneOf()
    machine' = initial(sc)
  }
  action tick = all {canTick(machine), machine' = applyTick(machine)}
  action observe(ready: bool): bool = all {
    canObserve(machine, ready), machine' = applyObservation(machine, ready),
  }
  action terminalStutter = all {isTerminal(machine), machine' = machine}
  action step = any {tick, observe(false), observe(true), terminalStutter}

  pure def frameValid(frame: Frame): bool = match frame {
    | Multiply(n) => n >= 1 and n <= 4
    | AddHead(n) => inScalar(n)
    | VisitRight(ref) => HEAP_IDS.contains(ref)
    | AddLeft(n) => inScalar(n)
  }

  pure def nodeValid(id: int, node: Node): bool = match node {
    | Nil => true
    | Cons(c) => inScalar(c.head) and HEAP_IDS.contains(c.tail) and c.tail < id
    | Leaf(v) => inScalar(v)
    | Branch(b) => and {
      HEAP_IDS.contains(b.left), HEAP_IDS.contains(b.right),
      b.left < id, b.right < id,
    }
  }

  val finiteStateInv = and {
    inScalar(machine.acc), machine.arg >= 0, machine.arg <= 4,
    HEAP_IDS.contains(machine.cursor), machine.frames.length() <= MAX_FRAMES,
    machine.maxDepth >= machine.frames.length(), machine.maxDepth <= MAX_FRAMES,
    machine.observations >= 0, machine.observations <= 2,
    machine.acceptedEvents >= 0, machine.acceptedEvents <= 1,
    machine.boundary >= 0, machine.boundary <= 1,
    machine.visited.forall(id => HEAP_IDS.contains(id)),
  }
  val heapAndFramesInv = and {
    machine.heap == initialHeap, machine.heap.keys() == HEAP_IDS,
    HEAP_IDS.forall(id => nodeValid(id, machine.heap.get(id))),
    machine.frames.foldl(true, (ok, frame) => ok and frameValid(frame)),
  }
  val workAccountingInv = and {
    Set(2, 32).contains(machine.total),
    machine.remaining >= 0, machine.remaining <= machine.total,
    machine.consumed + machine.remaining == machine.total,
    machine.attempted == machine.consumed + (if (machine.phase == Exhausted) 1 else 0),
  }
  val waitingInv = if (machine.phase == Waiting) and {
    machine.control == ReturnValue, machine.frames.length() > 0,
    machine.pending == PendingReturn(machine.acc),
    machine.checkpoint.frames == machine.frames,
    machine.checkpoint.acc == machine.acc,
    machine.checkpoint.consumed == machine.consumed,
    machine.boundary == 1, machine.acceptedEvents == 0,
  } else machine.pending == NoPending
  val terminalResultInv = and {
    (machine.phase == Halted) implies and {
      machine.control == Finished, machine.frames.length() == 0,
      machine.result == ResultValue(machine.acc), machine.reason == NoReason,
    },
    (machine.phase == Rejected) implies machine.reason != NoReason,
    (machine.phase != Halted) implies machine.result == NoResult,
    (machine.phase == Exhausted) implies machine.remaining == 0,
  }
  val checkpointInv = and {
    machine.checkpoint.consumed >= 0,
    machine.checkpoint.consumed <= machine.consumed,
    (machine.boundary == 0) implies machine.checkpoint.consumed == 0,
    (machine.boundary == 1) implies machine.yielded,
  }
  val allInv = and {
    finiteStateInv, heapAndFramesInv, workAccountingInv,
    waitingInv, terminalResultInv, checkpointInv,
  }
  // Named non-initial reachability objectives; tests also force each concrete trace.
  val recursiveWitness = machine.phase == Halted and machine.program == Factorial and machine.maxDepth >= 3
  val listWitness = machine.phase == Halted and machine.program == ListSum and machine.visited.size() == 4
  val treeWitness = machine.phase == Halted and machine.program == TreeSum and machine.visited == Set(4, 5, 7)
  val waitingWitness = machine.phase == Waiting and machine.frames.length() >= 3
  val resumedWitness = machine.phase == Halted and machine.acceptedEvents == 1 and machine.observations == 2
  val exhaustionWitness = machine.phase == Exhausted and machine.consumed > 0
  val rejectionWitness = machine.phase == Rejected and machine.reason == ScalarBound
}
