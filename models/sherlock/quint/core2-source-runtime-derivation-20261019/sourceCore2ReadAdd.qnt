module sourceCore2ReadAdd {
  import financialBoundary.* from "./financialBoundary"

  // Positive ordinary Core/2 fragment, not production admission/error policy.
  type SourceExpression = SourceRead(str) | SourceUIntAdd({leftId: int, rightId: int})
    | SourceLiteral(Scalar)
  type SourceProgram = {semantics: str, actionName: str,
    expressions: List[SourceExpression], statements: List[Statement]}
  type SourceRequest = {identity: Identity, plan: ActionPlan, guardCosts: int -> int,
    input: InvocationInput, startBoundary: Snapshot}
  type ExpressionTask = NeedExpression(int)
    | FinishSourceAdd({nodeId: int, leftId: int, rightId: int})
  type ExpressionScan = {supported: bool, todo: List[ExpressionTask],
    memo: int -> Scalar, visited: Set[int], attempted: int}
  type ExpressionObservation = {value: OptionalScalar, visited: Set[int], attempted: int}
  type SourceRuntime = SourceDerived(RuntimeEvidence) | OutsideSourceFragment
  type SourceQueue = SourceInvocations(List[Invocation]) | OutsideSourceQueue
  type CaptureRead = {supported: bool, rows: List[NamedValue]}
  type SiteRead = NoSourceSite | SourceSite(AwaitSite)
  type SegmentScan = {supported: bool, stopped: bool, notAdvanced: bool,
    startOrdinal: int, resumeOrdinal: int, ordinalCursor: int, env: str -> Scalar, snapshot: Snapshot,
    steps: List[Step], effects: List[Effect], attempted: int,
    exitSite: int, capturesOut: List[NamedValue]}
  pure val CORE_UINT_MAX = 340282366920938463463374607431768211455

  // Explicit projection: old/supplied runtime evidence is not an input field.
  pure def sourceRequest(invocation: Invocation): SourceRequest = {
    identity: invocation.identity, plan: invocation.plan,
    guardCosts: invocation.guardCosts, input: invocation.input,
    startBoundary: invocation.startBoundary,
  }
  pure def sourceBoundary(request: SourceRequest): Snapshot = match request.input {
    | StartInput(_) => request.startBoundary
    | ResumeInput(input) => input.continuation.boundary
  }
  pure def overlayNamed(env: str -> Scalar, names: List[NamedValue]): str -> Scalar =
    names.foldl(env, (acc, named) => acc.put(named.name, named.value))
  pure def sourceEnvironment(request: SourceRequest): str -> Scalar = match request.input {
    | StartInput(input) => input.scenario.inputs.keys().fold(sourceBoundary(request).state,
        (acc, name) => acc.put(name, input.scenario.inputs.get(name)))
    | ResumeInput(input) => overlayNamed(overlayNamed(sourceBoundary(request).state,
        input.continuation.captures), input.event.values)
  }
  pure def scalarInFragment(value: Scalar): bool = match value {
    | UIntValue(number) => number >= 0 and number <= CORE_UINT_MAX
    | BoolValue(_) => true
  }
  pure def addSourceValues(scan: ExpressionScan, task: {nodeId: int, leftId: int, rightId: int},
      rest: List[ExpressionTask]): ExpressionScan = {
    if (not(scan.memo.keys().contains(task.leftId)) or
        not(scan.memo.keys().contains(task.rightId))) {...scan, supported: false} else {
      match scan.memo.get(task.leftId) {
        | UIntValue(leftNumber) => match scan.memo.get(task.rightId) {
            | UIntValue(rightNumber) => {
                val total = leftNumber + rightNumber
                if (leftNumber >= 0 and rightNumber >= 0 and total <= CORE_UINT_MAX)
                  {...scan, todo: rest, memo: scan.memo.put(task.nodeId, UIntValue(total))}
                else {...scan, supported: false}
              }
            | BoolValue(_) => {...scan, supported: false}
          }
        | BoolValue(_) => {...scan, supported: false}
      }
    }
  }
  pure def expressionTick(program: SourceProgram, env: str -> Scalar,
      scan: ExpressionScan): ExpressionScan = {
    if (not(scan.supported) or scan.todo.length() == 0) scan else {
      val rest = scan.todo.tail()
      match scan.todo.head() {
        | FinishSourceAdd(task) => addSourceValues(scan, task, rest)
        | NeedExpression(nodeId) => {
            if (scan.memo.keys().contains(nodeId)) {...scan, todo: rest}
            else if (nodeId < 0 or nodeId >= program.expressions.length())
              {...scan, supported: false}
            else {
              // A node attempt is charged before its children; cache hits above cost zero.
              val charged = {...scan, attempted: scan.attempted + 1,
                visited: scan.visited.union(Set(nodeId))}
              match program.expressions.nth(nodeId) {
                | SourceRead(name) => if (env.keys().contains(name)) {
                    val value = env.get(name)
                    if (scalarInFragment(value))
                      {...charged, todo: rest, memo: charged.memo.put(nodeId, value)}
                    else {...charged, supported: false}
                  } else {...charged, supported: false}
                | SourceLiteral(value) => if (scalarInFragment(value))
                    {...charged, todo: rest, memo: charged.memo.put(nodeId, value)}
                  else {...charged, supported: false}
                | SourceUIntAdd(children) => {
                    if (children.leftId >= 0 and children.leftId < nodeId and
                        children.rightId >= 0 and children.rightId < nodeId)
                      {...charged, todo: List(NeedExpression(children.leftId),
                        NeedExpression(children.rightId), FinishSourceAdd({nodeId: nodeId,
                          leftId: children.leftId, rightId: children.rightId})).concat(rest)}
                    else {...charged, supported: false}
                  }
              }
            }
          }
      }
    }
  }
  pure def deriveExpression(program: SourceProgram, env: str -> Scalar,
      rootId: int): ExpressionObservation = {
    val emptyMemo: int -> Scalar = Map()
    val initialScan: ExpressionScan = {supported: true, todo: List(NeedExpression(rootId)),
      memo: emptyMemo, visited: Set(), attempted: 0}
    // At most one first visit per node, two child requests and one return per Add.
    // 1+3*N worklist ticks is a finite observation bound, not a logical/helper fee.
    val finished = program.expressions.foldl(expressionTick(program, env, initialScan),
      (acc, unusedNode) => expressionTick(program, env,
        expressionTick(program, env, expressionTick(program, env, acc))))
    {value: if (finished.supported and finished.todo.length() == 0 and
        finished.memo.keys().contains(rootId)) SomeScalar(finished.memo.get(rootId)) else NoScalar,
      visited: finished.visited, attempted: finished.attempted}
  }
  pure def sourceSite(plan: ActionPlan, ordinal: int): SiteRead =
    plan.sites.foldl(NoSourceSite, (acc, siteRow) =>
      if (siteRow.ordinal == ordinal) SourceSite(siteRow) else acc)
  pure def readCaptures(env: str -> Scalar, slots: List[CaptureSlot]): CaptureRead = {
    val emptyRead: CaptureRead = {supported: true, rows: List()}
    slots.foldl(emptyRead, (acc, slot) => if (not(acc.supported)) acc
      else if (env.keys().contains(slot.name))
        {...acc, rows: acc.rows.append({name: slot.name, value: env.get(slot.name)})}
      else {...acc, supported: false})
  }
  pure def awaitEntry(request: SourceRequest, scan: SegmentScan, ordinal: int): SegmentScan =
    match sourceSite(request.plan, ordinal) {
      | NoSourceSite => {...scan, supported: false, stopped: true}
      | SourceSite(siteRow) => {
          val captured = readCaptures(scan.env, siteRow.captures)
          {...scan, supported: captured.supported, stopped: true,
            attempted: scan.attempted + 1, exitSite: siteRow.site, capturesOut: captured.rows}
        }
    }
  pure def resumeGuard(program: SourceProgram, scan: SegmentScan, ordinal: int,
      statement: Statement, rootId: int): SegmentScan = {
    val evaluated = deriveExpression(program, scan.env, rootId)
    val charged = {...scan, attempted: scan.attempted + 1 + evaluated.attempted}
    match evaluated.value {
      | NoScalar => {...charged, supported: false, stopped: true}
      | SomeScalar(value) => match value {
          | UIntValue(_) => {...charged, supported: false, stopped: true}
          | BoolValue(guardHolds) => if (guardHolds) {...charged,
              steps: scan.steps.append({index: ordinal, statement: statement,
                value: SomeScalar(value), snapshot: scan.snapshot, effects: List()})}
            else {...charged, stopped: true, notAdvanced: true}
        }
    }
  }
  pure def valueStatement(program: SourceProgram, scan: SegmentScan, ordinal: int,
      statement: Statement, name: str, rootId: int, writesState: bool): SegmentScan = {
    val evaluated = deriveExpression(program, scan.env, rootId)
    val charged = {...scan, attempted: scan.attempted + 1 + evaluated.attempted}
    match evaluated.value {
      | NoScalar => {...charged, supported: false, stopped: true}
      | SomeScalar(value) => {
          if (writesState and not(scan.snapshot.state.keys().contains(name)))
            {...charged, supported: false, stopped: true}
          else {
            val nextSnapshot = if (writesState)
              {...scan.snapshot, state: scan.snapshot.state.put(name, value)} else scan.snapshot
            val effects: List[Effect] = if (writesState)
              List(StateEffect({name: name, before: scan.snapshot.state.get(name), after: value}))
              else List()
            {...charged, env: scan.env.put(name, value), snapshot: nextSnapshot,
              steps: scan.steps.append({index: ordinal, statement: statement,
                value: SomeScalar(value), snapshot: nextSnapshot, effects: effects}),
              effects: scan.effects.concat(effects)}
          }
        }
    }
  }
  pure def statementTick(program: SourceProgram, request: SourceRequest,
      scan: SegmentScan, ordinal: int): SegmentScan = {
    if (not(scan.supported) or scan.stopped or ordinal < scan.startOrdinal) scan else {
      val statement = program.statements.nth(ordinal)
      match statement {
        | AwaitStatement(awaited) => if (ordinal == scan.resumeOrdinal)
            resumeGuard(program, scan, ordinal, statement, awaited.guard)
          else awaitEntry(request, scan, ordinal)
        | LetStatement(bound) => if (ordinal == scan.resumeOrdinal)
            {...scan, supported: false, stopped: true}
          else valueStatement(program, scan, ordinal, statement, bound.name, bound.value, false)
        | NextStatement(updated) => if (ordinal == scan.resumeOrdinal)
            {...scan, supported: false, stopped: true}
          else valueStatement(program, scan, ordinal, statement, updated.state, updated.value, true)
        | _ => {...scan, supported: false, stopped: true}
      }
    }
  }
  pure def deriveRuntime(program: SourceProgram, request: SourceRequest): SourceRuntime = {
    val entryOrdinal = match request.input {
      | StartInput(_) => 0
      | ResumeInput(input) => if (input.continuation.site > 0 and
          input.continuation.site <= request.plan.sites.length())
          request.plan.sites.nth(input.continuation.site - 1).ordinal else -1
    }
    val isResume = match request.input { | StartInput(_) => false | ResumeInput(_) => true }
    val initialScan: SegmentScan = {supported: entryOrdinal >= 0 and
        (entryOrdinal < program.statements.length() or not(isResume)) and
        program.actionName == request.identity.actionName and
        program.semantics == request.identity.semantics and
        program.statements.length() == request.plan.statementCount,
      stopped: false, notAdvanced: false, startOrdinal: entryOrdinal, ordinalCursor: 0,
      resumeOrdinal: match request.input { | StartInput(_) => -1 | ResumeInput(_) => entryOrdinal },
      env: sourceEnvironment(request), snapshot: sourceBoundary(request), steps: List(),
      effects: List(), attempted: 0, exitSite: 0, capturesOut: List()}
    val finished = program.statements.foldl(initialScan, (acc, unusedStatement) => {
      val steppedScan = statementTick(program, request, acc, acc.ordinalCursor)
      {...steppedScan, ordinalCursor: acc.ordinalCursor + 1}
    })
    if (not(finished.supported)) OutsideSourceFragment
    else if (finished.notAdvanced) SourceDerived({runtimeResult: NotAdvanced,
      attempted: finished.attempted})
    else SourceDerived({runtimeResult: Ran({execution: {semantics: program.semantics,
        actionName: program.actionName, pre: sourceBoundary(request), post: finished.snapshot,
        steps: finished.steps, effects: finished.effects}, exitSite: finished.exitSite,
        capturesOut: finished.capturesOut}), attempted: finished.attempted})
  }
  pure def deriveSourceQueue(program: SourceProgram, requests: List[SourceRequest]): SourceQueue = {
    val emptyQueue: List[Invocation] = List()
    val scanned = requests.foldl({supported: true, rows: emptyQueue}, (acc, request) =>
      if (not(acc.supported)) acc else match deriveRuntime(program, request) {
        | OutsideSourceFragment => {...acc, supported: false}
        | SourceDerived(evidence) => {...acc, rows: acc.rows.append({identity: request.identity,
            plan: request.plan, guardCosts: request.guardCosts, input: request.input,
            startBoundary: request.startBoundary, runtime: evidence})}
      })
    if (scanned.supported) SourceInvocations(scanned.rows) else OutsideSourceQueue
  }
}
