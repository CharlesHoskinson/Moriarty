// Source projection of b705bfacb sherlock/export_plans.rs::classify.
// Source-audited only: no Rust build/run claimed. The Rust classifier scans
// every pending frame, which can attribute an outer binary operation to an
// active scope-return step (K-A reference.rs rule 195).
module rustExportClassification {
  type Frame = Scope | ReturnF | LeftMul | RightMul | RightAdd
  type Op = ArenaRead | FrameRestore | CheckedMul | CheckedAdd
  type Input = {returning: bool, frames: List[Frame], rule: int}
  type Observation = {classified: bool, input: Input, operation: Op}
  var observation: Observation
  pure val pendingMul: Input = {returning: true, frames: List(Scope, RightMul), rule: 195}
  pure val activeMul: Input = {returning: true, frames: List(RightMul), rule: 153}
  pure val typedReturn: Input = {returning: true, frames: List(ReturnF, RightMul), rule: 185}
  pure def firstBinary(frames: List[Frame]): Op = frames.foldl(ArenaRead, (found, frame) =>
    if (found != ArenaRead) found
    else if (frame == LeftMul or frame == RightMul) CheckedMul
    else if (frame == RightAdd) CheckedAdd else ArenaRead)
  pure def sourceClassify(input: Input): Op =
    if (input.returning and input.frames.length() > 0 and input.frames.head() == ReturnF) FrameRestore
    else {
      val binary = firstBinary(input.frames)
      if (binary != ArenaRead) binary else if (input.returning) FrameRestore else ArenaRead
    }
  pure def referenceOperation(input: Input): Op =
    if (input.rule == 185 or input.rule == 195) FrameRestore
    else if (input.rule == 153 and input.frames.head() == RightMul) CheckedMul else ArenaRead
  // Source repair at 6a6774a0c: use the rule and top frame. ScopeRestore is
  // normalized to the FrameRestore category in this finite projection.
  pure def repairedClassify(input: Input): Op =
    if (input.returning and input.rule == 153 and input.frames.head() == RightMul) CheckedMul
    else if (input.returning and input.rule == 185 and input.frames.head() == ReturnF) FrameRestore
    else if (input.returning and input.rule == 195 and input.frames.head() == Scope) FrameRestore
    else ArenaRead
  action init = {
    nondet input = Set(pendingMul, activeMul, typedReturn).oneOf()
    observation' = {classified: false, input: input, operation: ArenaRead}
  }
  action classify = all {
    not(observation.classified),
    observation' = {...observation, classified: true, operation: sourceClassify(observation.input)},
  }
  action step = classify
  action classifyRepaired = all {
    not(observation.classified),
    observation' = {...observation, classified: true, operation: repairedClassify(observation.input)},
  }
  val belongsToActiveStep = not(observation.classified)
    or observation.operation == referenceOperation(observation.input)
  val sourceInv = not(observation.classified) or observation.operation == sourceClassify(observation.input)
  val misclassifiedPendingFrame = observation.classified and not(belongsToActiveStep)
  val repairedInv = belongsToActiveStep and (not(observation.classified)
    or observation.operation == repairedClassify(observation.input))
  val repairedPendingFrame = observation.classified and observation.input == pendingMul
    and observation.operation == FrameRestore
}
