// Source-order projection of factory-u draft_source.rs + owner_suspend.rs + owner_runtime.rs
// for U014-U016 await lowering, latent post-resume SI-20 controlOrigin / ScopeF semantics
// (not CLI-observable without a post-resume technical cut, per sherlock-eval d50d2adc4),
// spec-level sequential await site numbering (Design §3.2 only pins 1-based per-program sites),
// and /2 surface admission.
module awaitLowering {
  type Context = ContextReturnF | ContextRightF | ContextTopEntry
  type Phase = AtAwaitEval | AtWaiting | AtBodyEval | AtScopeReturn | AtContextReturn | AfterContext
  type EnvRoot = OuterEnv | CallerEnv | BinderEnv
  type Origin = SiteExprLet | SiteBody | ContextExpr
  type CutReply = CutNotAttempted | CutAccepted | CutRefusedSI20

  type LoweredState = {
    context: Context,
    phase: Phase,
    hasScopeF: bool,
    envRoot: EnvRoot,
    controlOrigin: Origin,
    cutReply: CutReply,
    chainedAwait: bool,
    firstSite: int,
    secondSite: int,
    binderCount: int,
    guardUsesLetLocal: bool,
    surfaceAdmitted: bool,
  }

  var state: LoweredState

  // owner_runtime.rs:4311-4396 (si20_return_ok):
  // When Control == Return and frames is non-empty, controlOrigin must be in the
  // If-branch closure of expectedChild(top).
  // - If ScopeF is on top (AtScopeReturn), expectedChild(ScopeF) = Let.body = SiteBody.
  // - If ReturnF or RightF is on top (AtContextReturn), expectedChild(top) = SiteExprLet.
  //   Because si20_return_ok only closes across ExpressionRowC::If branches (not Let.body),
  //   controlOrigin == SiteBody fails SI-20 when ScopeF was omitted.
  pure def si20ReturnOk(s: LoweredState): bool =
    match s.phase {
      | AtScopeReturn => s.controlOrigin == SiteBody
      | AtContextReturn =>
        match s.context {
          | ContextTopEntry => true
          | _ => s.controlOrigin == SiteExprLet
        }
      | _ => true
    }

  // draft_source.rs:823-854 (Parser::await_expression):
  // Source parses `let body = self.expression(0)?` BEFORE `self.awaits.push(...)`,
  // so in `await first ...; await second ...; E`, `second` gets site 1 and `first` gets site 2.
  pure def sourceFirstSite(chained: bool): int = if (chained) 2 else 1
  pure def sourceSecondSite(chained: bool): int = if (chained) 1 else 1

  // Contract (draft_source.rs:29,217-220; Design §3.2): 1-based sites in source order.
  pure def contractFirstSite(chained: bool): int = 1
  pure def contractSecondSite(chained: bool): int = if (chained) 2 else 1

  // draft_source.rs:681,809,1050,1145:
  // Source rejects yield / 0-binder await (binderCount == 0) and rejects guards referencing
  // ordinary `let` scalar locals (AwaitScopes::walk sets let_sort = None).
  pure def sourceSurfaceOk(binders: int, usesLetLocal: bool): bool =
    binders >= 1 and binders <= 4 and not(usesLetLocal)

  // Design §3.1, §3.3, §6.2: 0..=4 binders (`yield L;` = 0 binders) and in-scope scalar locals.
  pure def contractSurfaceOk(binders: int, usesLetLocal: bool): bool =
    binders >= 0 and binders <= 4

  // Context frame pop on ControlC::Return:
  // ReturnF restores saved_env (OuterEnv); RightF and TopEntry preserve current envRoot.
  pure def envAfterContextReturn(ctx: Context, currentEnv: EnvRoot): EnvRoot =
    match ctx {
      | ContextReturnF => OuterEnv
      | ContextRightF => currentEnv
      | ContextTopEntry => currentEnv
    }

  action initWith(
    ctx: Context,
    chained: bool,
    binders: int,
    usesLetLocal: bool
  ): bool = {
    state' = {
      context: ctx,
      phase: AtAwaitEval,
      hasScopeF: false,
      envRoot: CallerEnv,
      controlOrigin: SiteExprLet,
      cutReply: CutNotAttempted,
      chainedAwait: chained,
      firstSite: contractFirstSite(chained),
      secondSite: contractSecondSite(chained),
      binderCount: binders,
      guardUsesLetLocal: usesLetLocal,
      surfaceAdmitted: contractSurfaceOk(binders, usesLetLocal),
    }
  }

  action init: bool = {
    nondet ctx = Set(ContextReturnF, ContextRightF, ContextTopEntry).oneOf()
    nondet chained = Set(false, true).oneOf()
    nondet binders = Set(0, 1, 4).oneOf()
    nondet usesLetLocal = Set(false, true).oneOf()
    initWith(ctx, chained, binders, usesLetLocal)
  }

  // Source lowering + owner_suspend.rs:416 (await_enter):
  // Applies draft_source.rs post-order site numbering and surface filter, then intercepts
  // Eval(SiteExprLet) before KMachineEvalLet.
  action sourceAwaitEnter: bool = all {
    state.phase == AtAwaitEval,
    state' = {
      ...state,
      phase: AtWaiting,
      controlOrigin: SiteExprLet,
      firstSite: sourceFirstSite(state.chainedAwait),
      secondSite: sourceSecondSite(state.chainedAwait),
      surfaceAdmitted: sourceSurfaceOk(state.binderCount, state.guardUsesLetLocal),
    },
  }

  // Contract lowering + await_enter: preserves source-order site numbering and surface admission.
  action contractAwaitEnter: bool = all {
    state.phase == AtAwaitEval,
    state' = {
      ...state,
      phase: AtWaiting,
      controlOrigin: SiteExprLet,
      firstSite: contractFirstSite(state.chainedAwait),
      secondSite: contractSecondSite(state.chainedAwait),
      surfaceAdmitted: contractSurfaceOk(state.binderCount, state.guardUsesLetLocal),
    },
  }

  // owner_suspend.rs:444-491 (resume_accept):
  // Allocates binder cells in env_root and jumps directly to Eval(SiteBody) with
  // controlOrigin = SiteBody, without pushing ScopeF { saved_env: CallerEnv, origin: SiteExprLet }.
  action sourceResumeAccept: bool = all {
    state.phase == AtWaiting,
    state' = {
      ...state,
      phase: AtBodyEval,
      hasScopeF: false,
      envRoot: BinderEnv,
      controlOrigin: SiteBody,
    },
  }

  // Candidate remedy: push ScopeF { saved_env: CallerEnv, origin: SiteExprLet } on resume_accept.
  action scopedResumeAccept: bool = all {
    state.phase == AtWaiting,
    state' = {
      ...state,
      phase: AtBodyEval,
      hasScopeF: true,
      envRoot: BinderEnv,
      controlOrigin: SiteBody,
    },
  }

  // Body E finishes and produces ControlC::Return.
  action evalBodyToReturn: bool = all {
    state.phase == AtBodyEval,
    state' = {
      ...state,
      phase: if (state.hasScopeF) AtScopeReturn else AtContextReturn,
      controlOrigin: SiteBody,
    },
  }

  // KMachineReturnScope pops ScopeF, restoring envRoot = CallerEnv and controlOrigin = SiteExprLet.
  action popScopeF: bool = all {
    state.phase == AtScopeReturn,
    state.hasScopeF,
    state' = {
      ...state,
      phase: AtContextReturn,
      hasScopeF: false,
      envRoot: CallerEnv,
      controlOrigin: SiteExprLet,
    },
  }

  // U016 technical cut at AtContextReturn (e.g. InnerReturn before ReturnF pops):
  // resume_profile2_checkpoint / resume_technical_checkpoint runs owner_check_step_inv (including SI-20).
  action technicalCutAtContextReturn: bool = all {
    state.phase == AtContextReturn,
    state.cutReply == CutNotAttempted,
    state' = {
      ...state,
      cutReply: if (si20ReturnOk(state)) CutAccepted else CutRefusedSI20,
    },
  }

  // Pop enclosing context frame (ReturnF / RightF) or halt at TopEntry.
  action completeContextReturn: bool = all {
    state.phase == AtContextReturn,
    state' = {
      ...state,
      phase: AfterContext,
      envRoot: envAfterContextReturn(state.context, state.envRoot),
      controlOrigin: ContextExpr,
    },
  }

  action stutter: bool = all {
    state.phase == AfterContext,
    state' = state,
  }

  action sourceStep: bool = any {
    sourceAwaitEnter,
    sourceResumeAccept,
    evalBodyToReturn,
    technicalCutAtContextReturn,
    completeContextReturn,
    stutter,
  }

  action scopedStep: bool = any {
    contractAwaitEnter,
    scopedResumeAccept,
    evalBodyToReturn,
    popScopeF,
    technicalCutAtContextReturn,
    completeContextReturn,
    stutter,
  }

  action step: bool = sourceStep

  // Safety properties expected by the design / U014-U016 contract:
  val si20Coherent = si20ReturnOk(state)
  val technicalCutAcceptedAtReturn = state.cutReply != CutRefusedSI20
  val noBinderEnvLeak = state.phase == AfterContext implies state.envRoot != BinderEnv
  val si20AndCutSafety = si20Coherent and technicalCutAcceptedAtReturn and noBinderEnvLeak
  val monotoneSequentialSites = state.chainedAwait implies
    (state.firstSite == 1 and state.secondSite == 2)
  val surfaceMatchesDesign =
    state.surfaceAdmitted == contractSurfaceOk(state.binderCount, state.guardUsesLetLocal)

  val contractSafety = and {
    si20AndCutSafety,
    monotoneSequentialSites,
    surfaceMatchesDesign,
  }

  // Exact characterization of current factory-u source behavior:
  val sourceBehavior = and {
    (state.phase == AtContextReturn and not(state.hasScopeF)
      and state.context != ContextTopEntry and state.controlOrigin == SiteBody)
      implies not(si20ReturnOk(state)),
    state.cutReply == CutRefusedSI20 implies
      (state.phase == AtContextReturn or state.phase == AfterContext),
    (state.phase == AfterContext and state.context != ContextReturnF
      and state.controlOrigin == ContextExpr)
      implies (state.envRoot == BinderEnv or state.envRoot == CallerEnv),
  }

  // Witnesses for source disagreements and clean execution:
  val si20ViolatedAtInnerReturn =
    state.phase == AtContextReturn and state.context == ContextReturnF and not(si20ReturnOk(state))
  val technicalCutRefusedAtInnerReturn =
    state.context == ContextReturnF and state.cutReply == CutRefusedSI20
  val binderLeakedAfterReturn =
    state.phase == AfterContext and state.envRoot == BinderEnv
  val reversedSequentialSites =
    state.phase != AtAwaitEval and state.chainedAwait and state.firstSite == 2 and state.secondSite == 1
  val surfaceRejectedYieldOrLetLocal =
    state.phase != AtAwaitEval and contractSurfaceOk(state.binderCount, state.guardUsesLetLocal) and not(state.surfaceAdmitted)
  val returnFMasksEnvLeakAfterPop =
    state.phase == AfterContext and state.context == ContextReturnF and state.envRoot == OuterEnv
  val scopedCutAcceptedAtInnerReturn =
    state.context == ContextReturnF and state.cutReply == CutAccepted and state.envRoot == CallerEnv
  val scopedCleanAfterRightF =
    state.phase == AfterContext and state.context == ContextRightF and state.envRoot == CallerEnv
}
