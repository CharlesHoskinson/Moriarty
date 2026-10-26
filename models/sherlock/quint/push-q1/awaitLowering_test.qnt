module awaitLowering_test {
  import awaitLowering.* from "./awaitLowering"

  // 1. Source resume_accept omits ScopeF; when inner(3) returns to ReturnF (InnerReturn),
  // controlOrigin == SiteBody != SiteExprLet violates SI-20 and refuses a U016 technical cut.
  run sourceInnerReturnViolatesSI20AndRefusesCutTest =
    initWith(ContextReturnF, false, 1, false)
      .then(sourceAwaitEnter)
      .then(sourceResumeAccept)
      .then(evalBodyToReturn)
      .expect(si20ViolatedAtInnerReturn and not(si20Coherent))
      .then(technicalCutAtContextReturn)
      .expect(technicalCutRefusedAtInnerReturn and not(technicalCutAcceptedAtReturn))

  // 2. When ReturnF pops without a technical cut at InnerReturn, ReturnF restores saved_env = OuterEnv,
  // masking the envRoot leak at AfterContext even though SI-20 failed on the Return step.
  run sourceReturnFPopRestoresOuterEnvAfterSI20ViolationTest =
    initWith(ContextReturnF, false, 1, false)
      .then(sourceAwaitEnter)
      .then(sourceResumeAccept)
      .then(evalBodyToReturn)
      .expect(not(si20Coherent))
      .then(completeContextReturn)
      .expect(returnFMasksEnvLeakAfterPop and noBinderEnvLeak)

  // 3. When the enclosing frame does not restore saved_env (e.g. RightF in `10 + (await ...)`),
  // omitting ScopeF both violates SI-20 on Return AND leaks BinderEnv into envRoot after RightF pops.
  run sourceRightFLeaksBinderEnvAndViolatesSI20Test =
    initWith(ContextRightF, false, 1, false)
      .then(sourceAwaitEnter)
      .then(sourceResumeAccept)
      .then(evalBodyToReturn)
      .expect(not(si20Coherent))
      .then(completeContextReturn)
      .expect(binderLeakedAfterReturn and not(noBinderEnvLeak))

  // 4. Candidate remedy (pushing ScopeF on resume_accept) satisfies SI-20 at both AtScopeReturn
  // and AtContextReturn, accepts a U016 technical cut at InnerReturn, and restores CallerEnv.
  run scopedRemedySatisfiesSI20AndAcceptsInnerReturnCutTest =
    initWith(ContextReturnF, false, 1, false)
      .then(contractAwaitEnter)
      .then(scopedResumeAccept)
      .then(evalBodyToReturn)
      .expect(state.phase == AtScopeReturn and si20Coherent)
      .then(popScopeF)
      .expect(state.phase == AtContextReturn and si20Coherent and state.envRoot == CallerEnv)
      .then(technicalCutAtContextReturn)
      .expect(scopedCutAcceptedAtInnerReturn and contractSafety)
      .then(completeContextReturn)
      .expect(contractSafety and state.envRoot == OuterEnv)

  // 5. Candidate remedy under RightF restores envRoot = CallerEnv before RightF pops.
  run scopedRemedyRestoresCallerEnvUnderRightFTest =
    initWith(ContextRightF, false, 1, false)
      .then(contractAwaitEnter)
      .then(scopedResumeAccept)
      .then(evalBodyToReturn)
      .then(popScopeF)
      .then(completeContextReturn)
      .expect(scopedCleanAfterRightF and contractSafety)

  // 6. Sequential await chain `await first ...; await second ...` gets reversed site numbers (2, 1)
  // in draft_source.rs because body is parsed before awaits.push.
  run sourceSequentialAwaitReversesSiteNumbersTest =
    initWith(ContextReturnF, true, 1, false)
      .then(sourceAwaitEnter)
      .expect(reversedSequentialSites and not(monotoneSequentialSites))

  // 7. Surface /2 parser rejects zero-binder await/yield and let-bound scalar locals in guards.
  run sourceSurfaceRejectsZeroBinderAndLetLocalGuardTest =
    initWith(ContextTopEntry, false, 0, false)
      .then(sourceAwaitEnter)
      .expect(surfaceRejectedYieldOrLetLocal and not(surfaceMatchesDesign)
        and not(sourceSurfaceOk(1, true)) and contractSurfaceOk(1, true))
}
