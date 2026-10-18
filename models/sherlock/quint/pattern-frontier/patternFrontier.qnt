module patternFrontier {
  import nominalConstruction.* from "./nominalConstruction"
  // LOCAL operational reference bridge. None of these events/counts is a B rule,
  // helperWork charge, runtime capacity policy, or production admission rule.
  type FrontierSlice = {start: int, len: int}
  type FrontierValueRow = {token: Token, children: FrontierSlice}
  type FrontierPatternRow = {token: PatternToken, children: FrontierSlice}
  type FrontierArm = {sourceArmId: int, bodyExprId: int, patternRoot: int}
  type FrontierBinderOrigin = {name: int, path: List[int]}
  type FrontierInput = {
    sourceMatchId: int, arms: List[FrontierArm], valueRoot: int,
    valueRows: int -> FrontierValueRow, valueFields: List[int],
    patternRows: int -> FrontierPatternRow, patternFields: List[int],
    decodedValues: int -> Value, encodedPatterns: int -> List[PatternToken],
    sourcePaths: List[List[List[int]]], binderOrigins: List[List[FrontierBinderOrigin]],
    savedEnvironment: Environment, layouts: (int, int) -> Layout,
    analysisBounds: {advances: int, maxTasks: int, maxPath: int},
  }
  type FrontierTask = InspectTask({patternRow: int, valueRow: int, path: List[int]})
    | FieldTask({parentPattern: int, parentValue: int, patterns: FrontierSlice,
        values: FrontierSlice, cursor: int, path: List[int]})
  type FrontierBinding = {name: int, valueRow: int, path: List[int]}
  type FrontierVisit = {armIndex: int, sourceArmId: int, path: List[int],
    patternRow: int, valueRow: int, token: PatternToken}
  type FrontierPhase = BeginArm | Walking | Selected | NoArm
  type FrontierEvent = FrontierStart | ArmBegin | WildcardDone | BinderDone
    | HeaderMatched | ArmMiss | FieldScheduled | FieldFinished | ArmSelected | ArmsExhausted
  type FrontierDiagnostics = {transitions: int, patternRows: int, valueHeaders: int,
    fieldPairs: int, fieldFinishes: int, bindings: int, maxTasks: int}
  type FrontierCursor = {input: FrontierInput, phase: FrontierPhase, armIndex: int,
    tasks: List[FrontierTask], partial: List[FrontierBinding], visits: List[FrontierVisit],
    diagnostics: FrontierDiagnostics, event: FrontierEvent}
  type FrontierResult = {found: bool, index: int, bindings: Environment}
  pure val FRONTIER_PROFILE = "sherlock-quint-pattern-frontier-correspondence/1"

  pure def frontierInitial(input: FrontierInput): FrontierCursor = {
    input: input, phase: BeginArm, armIndex: 0, tasks: List(), partial: List(), visits: List(),
    diagnostics: {transitions: 0, patternRows: 0, valueHeaders: 0,
      fieldPairs: 0, fieldFinishes: 0, bindings: 0, maxTasks: 0}, event: FrontierStart,
  }
  pure def frontierTerminal(cursorState: FrontierCursor): bool =
    cursorState.phase == Selected or cursorState.phase == NoArm
  pure def frontierHeaderEquals(actualToken: Token, expectedToken: PatternToken): bool =
    match expectedToken {
      | PatternConstructor(wanted) => match actualToken {
          | ConstructorToken(actual) => actual == wanted
          | _ => false
        }
      | _ => false
    }
  // Reads only one indexed pattern row and, for Constructor, one value row.
  // It NEVER uses decodedValues, encodedPatterns, fieldsOf, or nestedPatternMatch.
  pure def frontierInspect(cursorState: FrontierCursor, task: {patternRow: int, valueRow: int, path: List[int]}): FrontierCursor = {
    val input = cursorState.input
    val patRow = input.patternRows.get(task.patternRow)
    val recorded = {...cursorState, visits: cursorState.visits.append({
      armIndex: cursorState.armIndex, sourceArmId: input.arms.nth(cursorState.armIndex).sourceArmId,
      path: task.path, patternRow: task.patternRow, valueRow: task.valueRow, token: patRow.token}),
      diagnostics: {...cursorState.diagnostics, patternRows: cursorState.diagnostics.patternRows + 1}}
    match patRow.token {
      | PatternWildcard => {...recorded, tasks: cursorState.tasks.tail(), event: WildcardDone}
      | PatternBind(name) => {...recorded, tasks: cursorState.tasks.tail(), event: BinderDone,
          partial: cursorState.partial.append({name: name, valueRow: task.valueRow, path: task.path}),
          diagnostics: {...recorded.diagnostics, bindings: recorded.diagnostics.bindings + 1}}
      | PatternConstructor(_) => {
          val valRow = input.valueRows.get(task.valueRow)
          val inspected = {...recorded, diagnostics: {...recorded.diagnostics,
            valueHeaders: recorded.diagnostics.valueHeaders + 1}}
          if (frontierHeaderEquals(valRow.token, patRow.token)) {...inspected, event: HeaderMatched,
            tasks: List(FieldTask({parentPattern: task.patternRow, parentValue: task.valueRow,
              patterns: patRow.children, values: valRow.children, cursor: 0, path: task.path})).concat(cursorState.tasks.tail())}
          else {...inspected, phase: BeginArm, armIndex: cursorState.armIndex + 1,
            tasks: List(), partial: List(), event: ArmMiss}
        }
    }
  }
  // Exactly two indexed field-slot reads. No traversal/materialization of a child.
  pure def frontierField(cursorState: FrontierCursor, task: {parentPattern: int, parentValue: int,
      patterns: FrontierSlice, values: FrontierSlice, cursor: int, path: List[int]}): FrontierCursor =
    if (task.cursor == task.patterns.len) {...cursorState, tasks: cursorState.tasks.tail(),
      event: FieldFinished, diagnostics: {...cursorState.diagnostics,
        fieldFinishes: cursorState.diagnostics.fieldFinishes + 1}}
    else {
      val input = cursorState.input
      val childTask = InspectTask({patternRow: input.patternFields.nth(task.patterns.start + task.cursor),
        valueRow: input.valueFields.nth(task.values.start + task.cursor), path: task.path.append(task.cursor)})
      {...cursorState, event: FieldScheduled,
        tasks: List(childTask, FieldTask({...task, cursor: task.cursor + 1})).concat(cursorState.tasks.tail()),
        diagnostics: {...cursorState.diagnostics, fieldPairs: cursorState.diagnostics.fieldPairs + 1}}
    }
  pure def frontierAdvance(cursorState: FrontierCursor): FrontierCursor =
    if (frontierTerminal(cursorState)) cursorState
    else {
      val moved = if (cursorState.phase == BeginArm) {
        if (cursorState.armIndex == cursorState.input.arms.length())
          {...cursorState, phase: NoArm, event: ArmsExhausted}
        else {...cursorState, phase: Walking, event: ArmBegin, partial: List(),
          tasks: List(InspectTask({patternRow: cursorState.input.arms.nth(cursorState.armIndex).patternRoot,
            valueRow: cursorState.input.valueRoot, path: List()}))}
      } else if (cursorState.tasks.length() == 0) {...cursorState, phase: Selected, event: ArmSelected}
      else match cursorState.tasks.head() {
        | InspectTask(task) => frontierInspect(cursorState, task)
        | FieldTask(task) => frontierField(cursorState, task)
      }
      {...moved, diagnostics: {...moved.diagnostics,
        transitions: cursorState.diagnostics.transitions + 1,
        maxTasks: if (moved.tasks.length() > cursorState.diagnostics.maxTasks)
          moved.tasks.length() else cursorState.diagnostics.maxTasks}}
    }

  // Everything below is PROPERTY/reference evaluation, outside local advance.
  pure def frontierRepeat(cursorState: FrontierCursor, count: int): FrontierCursor =
    range(0, count).foldl(cursorState, (acc, ignored) => frontierAdvance(acc))
  pure def frontierTrace(input: FrontierInput): List[FrontierCursor] =
    range(0, input.analysisBounds.advances).foldl(List(frontierInitial(input)), (acc, ignored) =>
      acc.append(frontierAdvance(acc.nth(acc.length() - 1))))
  pure def frontierProjection(cursorState: FrontierCursor): FrontierResult = {
    found: cursorState.phase == Selected, index: cursorState.armIndex,
    bindings: if (cursorState.phase == Selected)
      cursorState.partial.foldl(cursorState.input.savedEnvironment, (env, binding) =>
        env.put(binding.name, cursorState.input.decodedValues.get(binding.valueRow)))
      else cursorState.input.savedEnvironment,
  }
  pure def frontierReference(input: FrontierInput): FrontierResult =
    range(0, input.arms.length()).foldl({found: false, index: input.arms.length(), bindings: input.savedEnvironment},
      (acc, armIndex) => if (acc.found) acc else {
        val attempted = nestedPatternMatch(input.encodedPatterns.get(input.arms.nth(armIndex).patternRoot),
          input.decodedValues.get(input.valueRoot), input.savedEnvironment)
        if (attempted.matched) {found: true, index: armIndex, bindings: attempted.bindings} else acc
      })
  pure def frontierSliceValid(segment: FrontierSlice, count: int): bool =
    segment.start >= 0 and segment.len >= 0 and segment.start + segment.len <= count
  pure def frontierProgram(input: FrontierInput): Program =
    {expressions: Map(), functions: Map(), layouts: input.layouts, entry: 0}
  pure def frontierIsWildcard(patToken: PatternToken): bool = match patToken {
    | PatternWildcard => true
    | _ => false
  }
  pure def frontierPatternArity(patToken: PatternToken): int = match patToken {
    | PatternConstructor(c) => c.arity
    | _ => 0
  }
  pure def frontierValueEncodingValid(input: FrontierInput): bool =
    if (input.valueRows.keys() != input.decodedValues.keys() or not(input.valueRows.keys().contains(input.valueRoot))) false
    else input.valueRows.keys().forall(rowId => {
      val rowData = input.valueRows.get(rowId)
      val segment = rowData.children
      if (not(frontierSliceValid(segment, input.valueFields.length())) or segment.len != tokenArity(rowData.token)) false
      else {
        val childIds = input.valueFields.slice(segment.start, segment.start + segment.len)
        if (not(childIds.foldl(true, (ok, childId) => ok and input.valueRows.keys().contains(childId)))) false
        else and {
          input.decodedValues.get(rowId) == childIds.foldl(List(rowData.token),
            (acc, childId) => acc.concat(input.decodedValues.get(childId))),
          validValue(frontierProgram(input), input.decodedValues.get(rowId)),
        }
      }
    })
  pure def frontierPatternTyped(input: FrontierInput, nestedTokens: List[PatternToken]): bool = {
    val scan = nestedTokens.foldl({expected: List(rootType(input.decodedValues.get(input.valueRoot))),
      names: List(), ok: true}, (acc, patToken) => if (acc.expected.length() == 0) {...acc, ok: false}
      else match patToken {
        | PatternWildcard => {...acc, expected: acc.expected.tail()}
        | PatternBind(name) => {...acc, expected: acc.expected.tail(), names: acc.names.append(name)}
        | PatternConstructor(c) => {
            val key = (c.typeId, c.ctorId)
            if (not(input.layouts.keys().contains(key))) {...acc, ok: false}
            else {...acc, ok: acc.ok and acc.expected.head() == c.typeId and c.arity == input.layouts.get(key).fields.length(),
              expected: input.layouts.get(key).fields.concat(acc.expected.tail())}
          }
      })
    scan.ok and scan.expected.length() == 0 and distinct(scan.names)
  }
  pure def frontierPatternEncodingValid(input: FrontierInput): bool =
    if (input.patternRows.keys() != input.encodedPatterns.keys()) false
    else input.patternRows.keys().forall(rowId => {
      val rowData = input.patternRows.get(rowId)
      val segment = rowData.children
      if (not(frontierSliceValid(segment, input.patternFields.length())) or segment.len != frontierPatternArity(rowData.token)) false
      else {
        val childIds = input.patternFields.slice(segment.start, segment.start + segment.len)
        if (not(childIds.foldl(true, (ok, childId) => ok and input.patternRows.keys().contains(childId)))) false
        else input.encodedPatterns.get(rowId) == childIds.foldl(List(rowData.token),
          (acc, childId) => acc.concat(input.encodedPatterns.get(childId)))
      }
    })
  pure def frontierValueAtPath(input: FrontierInput, path: List[int]): int =
    path.foldl(input.valueRoot, (nodeId, position) =>
      if (not(input.valueRows.keys().contains(nodeId))) -1 else {
        val segment = input.valueRows.get(nodeId).children
        if (position < 0 or position >= segment.len) -1 else input.valueFields.nth(segment.start + position)
      })
  pure def frontierPatternAtPath(input: FrontierInput, armIndex: int, path: List[int]): int =
    path.foldl(input.arms.nth(armIndex).patternRoot, (nodeId, position) =>
      if (not(input.patternRows.keys().contains(nodeId))) -1 else {
        val segment = input.patternRows.get(nodeId).children
        if (position < 0 or position >= segment.len) -1 else input.patternFields.nth(segment.start + position)
      })
  pure def frontierSourceMetadataValid(input: FrontierInput, armIndex: int): bool = {
    val nestedTokens = input.encodedPatterns.get(input.arms.nth(armIndex).patternRoot)
    val sourceScan = nestedTokens.foldl({pending: List(List()), paths: List(), binders: List(), ok: true},
      (acc, patToken) => if (acc.pending.length() == 0) {...acc, ok: false} else {
        val path = acc.pending.head()
        val children = range(0, frontierPatternArity(patToken)).foldl(List(),
          (paths, index) => paths.append(path.append(index)))
        {...acc, pending: children.concat(acc.pending.tail()), paths: acc.paths.append(path),
          binders: match patToken {
            | PatternBind(name) => acc.binders.append({name: name, path: path})
            | _ => acc.binders
          }}
      })
    and { sourceScan.ok, sourceScan.pending.length() == 0,
      input.sourcePaths.nth(armIndex) == sourceScan.paths,
      input.binderOrigins.nth(armIndex) == sourceScan.binders,
    }
  }
  pure def frontierInputsValid(input: FrontierInput): bool = and {
    frontierValueEncodingValid(input), frontierPatternEncodingValid(input),
    input.arms.length() == input.sourcePaths.length(), input.arms.length() == input.binderOrigins.length(),
    range(0, input.arms.length()).foldl(true, (ok, armIndex) => ok and {
      val armData = input.arms.nth(armIndex)
      if (not(input.patternRows.keys().contains(armData.patternRoot))) false else
        frontierPatternTyped(input, input.encodedPatterns.get(armData.patternRoot)) and
          frontierSourceMetadataValid(input, armIndex)
    }),
  }
  pure def frontierVisitsOrdered(cursorState: FrontierCursor): bool =
    range(0, cursorState.input.arms.length()).foldl(true, (ok, armIndex) => {
      val armVisits = cursorState.visits.select(visit => visit.armIndex == armIndex)
      val sourceOrder = cursorState.input.sourcePaths.nth(armIndex)
      ok and armVisits.length() <= sourceOrder.length() and
        range(0, armVisits.length()).foldl(true, (prior, index) => prior and {
          val visit = armVisits.nth(index)
          and { visit.path == sourceOrder.nth(index),
            visit.sourceArmId == cursorState.input.arms.nth(armIndex).sourceArmId,
            visit.patternRow == frontierPatternAtPath(cursorState.input, armIndex, visit.path),
            visit.valueRow == frontierValueAtPath(cursorState.input, visit.path),
            visit.token == cursorState.input.patternRows.get(visit.patternRow).token,
          }
        }) and (if (cursorState.phase == Selected and cursorState.armIndex == armIndex)
          armVisits.length() == sourceOrder.length() else true)
    })
  pure def frontierPendingValid(cursorState: FrontierCursor): bool = {
    val input = cursorState.input
    val isWalking = cursorState.phase == Walking
    and {
      cursorState.tasks.length() <= input.analysisBounds.maxTasks,
      cursorState.diagnostics.maxTasks <= input.analysisBounds.maxTasks,
      cursorState.tasks.foldl(true, (ok, pendingTask) => ok and match pendingTask {
        | InspectTask(task) => and {
            isWalking, task.path.length() <= input.analysisBounds.maxPath,
            task.patternRow == frontierPatternAtPath(input, cursorState.armIndex, task.path),
            task.valueRow == frontierValueAtPath(input, task.path),
          }
        | FieldTask(task) => and {
            isWalking, task.path.length() <= input.analysisBounds.maxPath,
            task.parentPattern == frontierPatternAtPath(input, cursorState.armIndex, task.path),
            task.parentValue == frontierValueAtPath(input, task.path),
            task.patterns == input.patternRows.get(task.parentPattern).children,
            task.values == input.valueRows.get(task.parentValue).children,
            task.patterns.len == task.values.len, task.cursor >= 0, task.cursor <= task.patterns.len,
          }
      }),
      cursorState.partial == cursorState.visits.foldl(List(), (acc, visit) =>
        if (visit.armIndex != cursorState.armIndex) acc else match visit.token {
          | PatternBind(name) => acc.append({name: name, valueRow: visit.valueRow, path: visit.path})
          | _ => acc
        }),
      cursorState.partial.foldl(true, (ok, binding) => ok and
        binding.valueRow == frontierValueAtPath(input, binding.path) and
        input.binderOrigins.nth(cursorState.armIndex).foldl(false, (found, origin) =>
          found or (origin.name == binding.name and origin.path == binding.path))),
      if (cursorState.phase == BeginArm or cursorState.phase == NoArm)
        cursorState.tasks.length() == 0 and cursorState.partial.length() == 0 else true,
    }
  }
  pure def frontierLocalReadBound(before: FrontierCursor, after: FrontierCursor): bool = {
    val oldCounts = before.diagnostics
    val newCounts = after.diagnostics
    if (frontierTerminal(before)) after == before
    else and {
      after.input == before.input, newCounts.transitions == oldCounts.transitions + 1,
      newCounts.patternRows - oldCounts.patternRows >= 0, newCounts.patternRows - oldCounts.patternRows <= 1,
      newCounts.valueHeaders - oldCounts.valueHeaders >= 0, newCounts.valueHeaders - oldCounts.valueHeaders <= 1,
      newCounts.fieldPairs - oldCounts.fieldPairs >= 0, newCounts.fieldPairs - oldCounts.fieldPairs <= 1,
      newCounts.patternRows - oldCounts.patternRows + newCounts.fieldPairs - oldCounts.fieldPairs <= 1,
      if (after.event == WildcardDone or after.event == BinderDone)
        newCounts.valueHeaders == oldCounts.valueHeaders else true,
      if (after.event == ArmMiss) after.partial.length() == 0 and after.tasks.length() == 0 else true,
    }
  }
  pure def frontierCaseCorresponds(input: FrontierInput): bool = {
    val prefixes = frontierTrace(input)
    val completed = prefixes.nth(prefixes.length() - 1)
    and {
      frontierInputsValid(input), frontierTerminal(completed),
      frontierProjection(completed) == frontierReference(input),
      prefixes.foldl(true, (ok, prefix) => ok and frontierVisitsOrdered(prefix) and frontierPendingValid(prefix)),
      range(0, prefixes.length() - 1).foldl(true, (ok, index) =>
        ok and frontierLocalReadBound(prefixes.nth(index), prefixes.nth(index + 1))),
    }
  }
  // Finite deterministic cut/replay only: no production codec/checkpoint theorem.
  pure def frontierAllCuts(input: FrontierInput): bool = {
    val prefixes = frontierTrace(input)
    val completed = prefixes.nth(prefixes.length() - 1)
    range(0, prefixes.length()).foldl(true, (ok, cut) => ok and
      frontierRepeat(prefixes.nth(cut), input.analysisBounds.advances - cut) == completed)
  }
}
