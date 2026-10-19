module literalJoinRefusal {
  import literalCompleteImages.* from "./literalCompleteImages"
  import literalCompleteInputs.* from "./literalCompleteInputs"
  // NEW bounded model-only seams. B20 actions/ITF and all production APIs stay unchanged.
  // Domain: the existing UInt7 Program, opaque context[17,29], fixed physical shape,
  // literal Init with positive byte-valid fuel. This is not general admission/uiValid.
  pure val JOIN_REFUSAL_MODEL_PROFILE = "sherlock-a102-a80-join-refusal-model/1"
  type ImageStep = {program: Image, before: Image, after: Image, envelope: Envelope}

  pure def literalStartAllowed(programImage: Image, initialImage: Image,
      contextLimbs: Row): bool = programImage == uintProgram
    and contextLimbs == contextInput and initPremise(programImage, initialImage)

  pure def stateFuelBytesInRange(image: Image): bool =
    if (smallFixtureStateShape(image))
      image.root.slice(9, 41).foldl(true,
        (ok, byteValue) => ok and 0 <= byteValue and byteValue < 256)
    else false

  // Explicit two-child join; neither child's Program nor right.before is discarded.
  // The Rust composer aliases one intermediate, rather than exposing this new API.
  pure def completeJoinAllowed(boundProgram: Image, leftImageStep: ImageStep,
      rightImageStep: ImageStep, contextLimbs: Row): bool =
    if (smallFixtureProgramShape(boundProgram)
        and smallFixtureStateShape(leftImageStep.before)
        and smallFixtureStateShape(leftImageStep.after)
        and smallFixtureStateShape(rightImageStep.before)
        and smallFixtureStateShape(rightImageStep.after)) {
      literalStartAllowed(boundProgram, leftImageStep.before, contextLimbs)
        and leftImageStep.program == boundProgram
        and rightImageStep.program == boundProgram
        and leftImageStep.after == rightImageStep.before
        and stateFuelBytesInRange(leftImageStep.before)
        and stateFuelBytesInRange(leftImageStep.after)
        and stateFuelBytesInRange(rightImageStep.before)
        and stateFuelBytesInRange(rightImageStep.after)
        and relation({program: boundProgram, origin: leftImageStep.before,
          intermediate: leftImageStep.after, done: rightImageStep.after,
          literalEnvelope: leftImageStep.envelope, returnEnvelope: rightImageStep.envelope})
    } else false

  action initChecked(programImage: Image, initialImage: Image,
      contextLimbs: Row): bool = all {
    literalStartAllowed(programImage, initialImage, contextLimbs),
    initWith(programImage, initialImage, contextLimbs),
  }
  val checkedLiteralReady = if (execution.states.length() == 1
      and execution.envelopes.length() == 0)
    literalStartAllowed(execution.program, execution.states.nth(0), execution.context)
    else false
  action literalStepChecked = all {checkedLiteralReady, literalStep}

  // A80 remains free at remaining fuel0. This guard does not require middle fuel>0.
  val checkedPublicationReady = if (execution.states.length() == 2
      and execution.envelopes == List(literalEnvelope)) {
    literalStartAllowed(execution.program, execution.states.nth(0), execution.context)
      and stateFuelBytesInRange(execution.states.nth(1))
      and execution.states.nth(1) == literalAfter(execution.program, execution.states.nth(0))
  } else false
  action publicationStepChecked = all {checkedPublicationReady, publicationStep}
}
