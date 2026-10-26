// U033 provenance contract at abe5ee146: each record belongs to its producing
// block, and consecutive blocks follow same/successor/generic-exit flow.
// Metadata validation only. Actual StepRecords must separately match the
// reference stream; passing this model is not proof of a causal IR executor.
module residualFlow {
  type BlockRef = Missing | Named(int)
  type Record = {block: BlockRef, rule: int}
  type Block = {generic: bool, rules: Set[int], successors: Set[int]}
  type Validation = {accepted: List[Record], attempts: int, rejected: int}
  var validation: Validation
  pure val ids = Set(0, 1, 2, 3)
  pure val blocks: int -> Block = Map(
    0 -> {generic: false, rules: Set(102), successors: Set(1, 3)},
    1 -> {generic: false, rules: Set(80), successors: Set()},
    2 -> {generic: false, rules: Set(84), successors: Set()},
    3 -> {generic: true, rules: Set(), successors: Set()})
  pure def belongs(record: Record): bool = match record.block {
    | Missing => false
    | Named(id) => if (ids.contains(id)) blocks.get(id).generic or blocks.get(id).rules.contains(record.rule) else false
  }
  pure def follows(before: Record, after: Record): bool = match before.block {
    | Missing => false
    | Named(left) => match after.block {
      | Missing => false
      | Named(right) => if (ids.contains(left) and ids.contains(right))
          left == right or blocks.get(left).successors.contains(right) or blocks.get(left).generic
        else false
    }
  }
  pure def canPublish(trace: List[Record], record: Record): bool =
    belongs(record) and (if (trace.length() == 0) true else follows(trace.nth(trace.length() - 1), record))
  pure def valid(trace: List[Record]): bool = trace.indices().forall(ix =>
    belongs(trace.nth(ix)) and (if (ix == 0) true else follows(trace.nth(ix - 1), trace.nth(ix))))
  action init = validation' = {accepted: List(), attempts: 0, rejected: 0}
  action publish(record: Record): bool = all {
    canPublish(validation.accepted, record),
    validation' = {...validation, accepted: validation.accepted.append(record), attempts: validation.attempts + 1},
  }
  action refuse(record: Record): bool = all {
    not(canPublish(validation.accepted, record)),
    validation' = {...validation, attempts: validation.attempts + 1, rejected: validation.rejected + 1},
  }
  action step = {
    nondet record = Set({block: Missing, rule: 102}, {block: Named(0), rule: 102},
      {block: Named(1), rule: 80}, {block: Named(2), rule: 84}, {block: Named(3), rule: 153},
      {block: Named(0), rule: 80}, {block: Named(99), rule: 102}).oneOf()
    any {publish(record), refuse(record)}
  }
  val modelInv = valid(validation.accepted)
    and validation.accepted.length() + validation.rejected == validation.attempts
  val acceptedSome = validation.accepted.length() > 0
  val refusedSome = validation.rejected > 0
  val usedSuccessor = validation.accepted.indices().exists(ix => if (ix == 0) false
    else validation.accepted.nth(ix - 1).block == Named(0) and validation.accepted.nth(ix).block == Named(1))
  val exitedGeneric = validation.accepted.indices().exists(ix => if (ix == 0) false
    else validation.accepted.nth(ix - 1).block == Named(3) and validation.accepted.nth(ix).block != Named(3))
}
