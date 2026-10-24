module c6PositiveContextTest {
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveContext.* from "./c6PositiveContext"
  import c6PositiveContextInputs.* from "./c6PositiveContextInputs"
  import independentC6ContextExpected.* from "./independentC6ContextExpected"

  run c6PositiveContextCompleteObservationsTest = all {
    c6CtxCompleteObservationsExact(c6CtxInputCases, expectedObservations),
  }

  run c6PositiveContextPresenceAndLifetimeTest = all {
    c6CtxInputCases.nth(0).input.context.ext == C6CtxAbsent,
    c6CtxInputCases.nth(1).input.context.ext == C6CtxPresent(c6CtxZeroExt),
    c6CtxPresenceSeparated(c6CtxInputCases.nth(0).input, c6CtxInputCases.nth(1).input),
    c6CtxEffectiveExt(c6CtxInputCases.nth(2).input.context.ext).max_lifetime_work > c6CtxHostUsizeMax,
    c6CtxEffectiveExt(c6CtxInputCases.nth(3).input.context.ext).max_lifetime_work > c6CtxHostUsizeMax,
    c6CtxLifetimeTransported(c6CtxInputCases.nth(2).input.context),
    c6CtxLifetimeTransported(c6CtxInputCases.nth(3).input.context),
  }
}
