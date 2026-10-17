module returnPublication {
  // Finite Quint source projection of the local A80 (k.admin.publish.value)
  // instruction fragment in return_publication_step.rs and its L4 schema.
  // Scope: single-step local physical relation; unexecuted source only.

  pure val RETURN_PUBLICATION_RULE_ID = "k.admin.publish.value"
  pure val RETURN_PUBLICATION_RULE_START_LINE = 80
  pure val RETURN_PUBLICATION_RULE_TAG = 0
  pure val RETURN_PUBLICATION_RULE_CHARGE = 0

  pure val SPACE_CONSTANT = 0
  pure val SPACE_DATA = 1

  pure val OUTCOME_PENDING = 0
  pure val OUTCOME_DONE = 1
  pure val OUTCOME_REJECTED = 2
  pure val OUTCOME_EXHAUSTED = 3

  pure val CONTROL_EVAL = 0
  pure val CONTROL_RETURN = 1
  pure val CONTROL_PUSH = 2
  pure val CONTROL_ERROR = 9

  type ProgramCapacities = {
    n: int, nb: int, o: int, op: int, x: int, xc: int, f: int, p: int,
    r: int, a: int, t: int, q: int, qf: int, k: int, kf: int,
  }

  type StateCapacities = {
    d: int, df: int, e: int, v: int, s: int, c: int, stackLimit: int,
  }

  type LiteralCapacities = {
    program: ProgramCapacities,
    state: StateCapacities,
  }

  type ValueRef = { space: int, index: int }
  type EnvRoot = { present: int, index: int }
  type Slice = { start: int, len: int }

  type NameRow = { byteStart: int, byteLength: int }
  type NameByteRow = { byte: int }
  type OccurrenceRow = { ownerTag: int, functionIndex: int, pathStart: int, pathLength: int }
  type OccurrencePathRow = { childPosition: int }
  type ExpressionRow = {
    tag: int, occurrenceId: int, valueRef: ValueRef, nameId: int,
    binaryOp: int, unaryOp: int, childSlice: Slice, armSlice: Slice,
  }
  type ExprChildRow = { expression: int }
  type FunctionRow = {
    sourceTableIndex: int, nameId: int, parameterSlice: Slice, returnType: int, bodyExprId: int,
  }
  type ParameterRow = { nameId: int, typeId: int }
  type PatternRow = { tag: int, name0: int, name1: int }
  type ArmRow = { patternId: int, bodyExprId: int }
  type TypeRow = { kind: int, ctorStart: int, ctorLength: int }
  type ConstructorRow = { owningType: int, sourceOrder: int, fieldTypeSlice: Slice }
  type ConstructorFieldTypeRow = { typeId: int }

  type ValueNode = {
    kind: int, typeId: int, ctorId: int, uintBytes: List[int],
    boolValue: int, fieldStart: int, fieldLength: int,
  }

  type ProgramHeader = {
    entryExprId: int, activeNames: int, activeNameBytes: int,
    activeOccurrences: int, activeOccurrencePaths: int, activeExpressions: int,
    activeExprChildren: int, activeFunctions: int, activeParameters: int,
    activePatterns: int, activeArms: int, activeTypes: int,
    activeConstructors: int, activeConstructorFieldTypes: int,
    activeConstants: int, activeConstantFields: int,
  }

  type ProgramImage = {
    root: ProgramHeader,
    names: List[NameRow],
    nameBytes: List[NameByteRow],
    occurrences: List[OccurrenceRow],
    occurrencePaths: List[OccurrencePathRow],
    expressions: List[ExpressionRow],
    exprChildren: List[ExprChildRow],
    functions: List[FunctionRow],
    parameters: List[ParameterRow],
    patterns: List[PatternRow],
    arms: List[ArmRow],
    types: List[TypeRow],
    constructors: List[ConstructorRow],
    constructorFieldTypes: List[ConstructorFieldTypeRow],
    constants: List[ValueNode],
    constantFields: List[ValueRef],
  }

  type EnvCell = { nameId: int, valueRef: ValueRef, parent: EnvRoot }

  type FrameRow = {
    tag: int, nameId: int, binaryOp: int, unaryOp: int, typeId: int,
    expr0: int, expr1: int, savedEnv: EnvRoot, valueRef: ValueRef,
    completedSlice: Slice, remainingExprSlice: Slice, armSlice: Slice,
  }

  type ControlRow = {
    tag: int, exprId: int, valueRef: ValueRef, pendingFrame: FrameRow,
    environment: EnvRoot, nameId: int, valueSlice: Slice,
    functionPresent: int, functionId: int, bindingsPresent: int,
    bindings: EnvRoot, typeId: int, armSlice: Slice,
    primitiveIsOk: int, errorCode: int,
  }

  type StateRoot = {
    activeData: int, activeDataFields: int, activeEnvCells: int,
    activeValueSlots: int, depth: int, controlLength: int,
    environment: EnvRoot, stackLimit: int,
    remainingFuel: List[int], initialFuel: List[int],
    outcomeTag: int, outcomeValue: ValueRef, outcomeError: int,
  }

  type StateImage = {
    root: StateRoot,
    data: List[ValueNode],
    dataFields: List[ValueRef],
    envCells: List[EnvCell],
    valueSlots: List[ValueRef],
    frameSlots: List[FrameRow],
    controlRows: List[ControlRow],
  }

  type LiteralEnvelope = { ruleTag: int, charge: int }

  type ReturnPublicationStepInputs = {
    enable: int,
    program: ProgramImage,
    pre: StateImage,
    post: StateImage,
    envelope: LiteralEnvelope,
  }

  pure def maxInt(a: int, b: int): int = if (a >= b) a else b

  // Smallest power of two >= 2 and > capacity - 1, matching literal_host::id bit width.
  pure def idBitLimit(capacity: int): int =
    range(0, 128).foldl(2, (bound, _) => if (bound <= capacity - 1) bound * 2 else bound)

  pure def inBit(v: int): bool = (v == 0 or v == 1)
  pure def inU8(v: int): bool = (0 <= v and v < 256)
  pure def inCount(v: int, cap: int): bool = (0 <= v and v <= cap)
  pure def inTag(v: int, variants: int): bool = (0 <= v and v < variants)
  pure def inId(v: int, cap: int): bool = (0 <= v and v < idBitLimit(cap))

  pure def inBytes16(bs: List[int]): bool =
    bs.length() == 16 and bs.foldl(true, (ok, b) => ok and inU8(b))

  pure def inSlice(s: Slice, cap: int): bool =
    inCount(s.start, cap) and inCount(s.len, cap)

  pure def inValueRef(vr: ValueRef, valCap: int): bool =
    inBit(vr.space) and inId(vr.index, valCap)

  pure def inEnvRoot(er: EnvRoot, envCap: int): bool =
    inBit(er.present) and inId(er.index, envCap)

  pure def inValueNode(node: ValueNode, tCap: int, qCap: int, fieldCap: int): bool = and {
    inTag(node.kind, 3),
    inId(node.typeId, tCap),
    inId(node.ctorId, qCap),
    inBytes16(node.uintBytes),
    inBit(node.boolValue),
    inCount(node.fieldStart, fieldCap),
    inCount(node.fieldLength, fieldCap),
  }

  pure def inFrameRow(fr: FrameRow, c: LiteralCapacities, valCap: int): bool = and {
    inTag(fr.tag, 9),
    inId(fr.nameId, c.program.n),
    inTag(fr.binaryOp, 7),
    inTag(fr.unaryOp, 2),
    inId(fr.typeId, c.program.t),
    inId(fr.expr0, c.program.x),
    inId(fr.expr1, c.program.x),
    inEnvRoot(fr.savedEnv, c.state.e),
    inValueRef(fr.valueRef, valCap),
    inSlice(fr.completedSlice, c.state.v),
    inSlice(fr.remainingExprSlice, c.program.xc),
    inSlice(fr.armSlice, c.program.a),
  }

  pure def inControlRow(cr: ControlRow, c: LiteralCapacities, valCap: int): bool = and {
    inTag(cr.tag, 10),
    inId(cr.exprId, c.program.x),
    inValueRef(cr.valueRef, valCap),
    inFrameRow(cr.pendingFrame, c, valCap),
    inEnvRoot(cr.environment, c.state.e),
    inId(cr.nameId, c.program.n),
    inSlice(cr.valueSlice, c.state.v),
    inBit(cr.functionPresent),
    inId(cr.functionId, c.program.f),
    inBit(cr.bindingsPresent),
    inEnvRoot(cr.bindings, c.state.e),
    inId(cr.typeId, c.program.t),
    inSlice(cr.armSlice, c.program.a),
    inBit(cr.primitiveIsOk),
    inU8(cr.errorCode),
  }

  pure def validMinimumShape(c: LiteralCapacities): bool = and {
    c.program.n >= 0, c.program.nb >= 0, c.program.o >= 0, c.program.op >= 0,
    c.program.x >= 1, c.program.xc >= 0, c.program.f >= 0, c.program.p >= 0,
    c.program.r >= 0, c.program.a >= 0, c.program.t >= 4, c.program.q >= 4,
    c.program.qf >= 5, c.program.k >= 1, c.program.kf >= 0,
    c.state.d >= 0, c.state.df >= 0, c.state.e >= 0, c.state.v >= 0,
    c.state.s >= 0, c.state.c >= 1, c.state.stackLimit >= 0,
  }

  pure def validProgramShape(c: LiteralCapacities, p: ProgramImage): bool = and {
    p.names.length() == c.program.n,
    p.nameBytes.length() == c.program.nb,
    p.occurrences.length() == c.program.o,
    p.occurrencePaths.length() == c.program.op,
    p.expressions.length() == c.program.x,
    p.exprChildren.length() == c.program.xc,
    p.functions.length() == c.program.f,
    p.parameters.length() == c.program.p,
    p.patterns.length() == c.program.r,
    p.arms.length() == c.program.a,
    p.types.length() == c.program.t,
    p.constructors.length() == c.program.q,
    p.constructorFieldTypes.length() == c.program.qf,
    p.constants.length() == c.program.k,
    p.constants.foldl(true, (ok, row) => ok and row.uintBytes.length() == 16),
    p.constantFields.length() == c.program.kf,
  }

  pure def validStateShape(c: LiteralCapacities, s: StateImage): bool = and {
    s.root.remainingFuel.length() == 16,
    s.root.initialFuel.length() == 16,
    s.data.length() == c.state.d,
    s.data.foldl(true, (ok, row) => ok and row.uintBytes.length() == 16),
    s.dataFields.length() == c.state.df,
    s.envCells.length() == c.state.e,
    s.valueSlots.length() == c.state.v,
    s.frameSlots.length() == c.state.s,
    s.controlRows.length() == c.state.c,
  }

  pure def validShape(c: LiteralCapacities, input: ReturnPublicationStepInputs): bool = and {
    validMinimumShape(c),
    validProgramShape(c, input.program),
    validStateShape(c, input.pre),
    validStateShape(c, input.post),
  }

  pure def validProgramRanges(c: LiteralCapacities, p: ProgramImage): bool = {
    val valCap = maxInt(c.program.k, c.state.d)
    val pathPos = maxInt(c.program.xc, c.program.a + 1)
    and {
      inId(p.root.entryExprId, c.program.x),
      inCount(p.root.activeNames, c.program.n),
      inCount(p.root.activeNameBytes, c.program.nb),
      inCount(p.root.activeOccurrences, c.program.o),
      inCount(p.root.activeOccurrencePaths, c.program.op),
      inCount(p.root.activeExpressions, c.program.x),
      inCount(p.root.activeExprChildren, c.program.xc),
      inCount(p.root.activeFunctions, c.program.f),
      inCount(p.root.activeParameters, c.program.p),
      inCount(p.root.activePatterns, c.program.r),
      inCount(p.root.activeArms, c.program.a),
      inCount(p.root.activeTypes, c.program.t),
      inCount(p.root.activeConstructors, c.program.q),
      inCount(p.root.activeConstructorFieldTypes, c.program.qf),
      inCount(p.root.activeConstants, c.program.k),
      inCount(p.root.activeConstantFields, c.program.kf),
      p.names.foldl(true, (ok, r) => ok and inCount(r.byteStart, c.program.nb) and inCount(r.byteLength, c.program.nb)),
      p.nameBytes.foldl(true, (ok, r) => ok and inU8(r.byte)),
      p.occurrences.foldl(true, (ok, r) => ok and inTag(r.ownerTag, 2) and inId(r.functionIndex, c.program.f) and inCount(r.pathStart, c.program.op) and inCount(r.pathLength, c.program.op)),
      p.occurrencePaths.foldl(true, (ok, r) => ok and inCount(r.childPosition, pathPos)),
      p.expressions.foldl(true, (ok, r) => ok and inTag(r.tag, 8) and inId(r.occurrenceId, c.program.o) and inValueRef(r.valueRef, valCap) and inId(r.nameId, c.program.n) and inTag(r.binaryOp, 7) and inTag(r.unaryOp, 2) and inSlice(r.childSlice, c.program.xc) and inSlice(r.armSlice, c.program.a)),
      p.exprChildren.foldl(true, (ok, r) => ok and inId(r.expression, c.program.x)),
      p.functions.foldl(true, (ok, r) => ok and inId(r.sourceTableIndex, c.program.f) and inId(r.nameId, c.program.n) and inSlice(r.parameterSlice, c.program.p) and inId(r.returnType, c.program.t) and inId(r.bodyExprId, c.program.x)),
      p.parameters.foldl(true, (ok, r) => ok and inId(r.nameId, c.program.n) and inId(r.typeId, c.program.t)),
      p.patterns.foldl(true, (ok, r) => ok and inTag(r.tag, 5) and inId(r.name0, c.program.n) and inId(r.name1, c.program.n)),
      p.arms.foldl(true, (ok, r) => ok and inId(r.patternId, c.program.r) and inId(r.bodyExprId, c.program.x)),
      p.types.foldl(true, (ok, r) => ok and inTag(r.kind, 4) and inCount(r.ctorStart, c.program.q) and inCount(r.ctorLength, c.program.q)),
      p.constructors.foldl(true, (ok, r) => ok and inId(r.owningType, c.program.t) and inCount(r.sourceOrder, c.program.q) and inSlice(r.fieldTypeSlice, c.program.qf)),
      p.constructorFieldTypes.foldl(true, (ok, r) => ok and inId(r.typeId, c.program.t)),
      p.constants.foldl(true, (ok, r) => ok and inValueNode(r, c.program.t, c.program.q, c.program.kf)),
      p.constantFields.foldl(true, (ok, r) => ok and inValueRef(r, valCap)),
    }
  }

  pure def validStateRanges(c: LiteralCapacities, s: StateImage): bool = {
    val valCap = maxInt(c.program.k, c.state.d)
    and {
      inCount(s.root.activeData, c.state.d),
      inCount(s.root.activeDataFields, c.state.df),
      inCount(s.root.activeEnvCells, c.state.e),
      inCount(s.root.activeValueSlots, c.state.v),
      inCount(s.root.depth, c.state.s),
      inCount(s.root.controlLength, c.state.c),
      inEnvRoot(s.root.environment, c.state.e),
      inCount(s.root.stackLimit, c.state.stackLimit),
      inBytes16(s.root.remainingFuel),
      inBytes16(s.root.initialFuel),
      inTag(s.root.outcomeTag, 4),
      inValueRef(s.root.outcomeValue, valCap),
      inU8(s.root.outcomeError),
      s.data.foldl(true, (ok, r) => ok and inValueNode(r, c.program.t, c.program.q, c.state.df)),
      s.dataFields.foldl(true, (ok, r) => ok and inValueRef(r, valCap)),
      s.envCells.foldl(true, (ok, r) => ok and inId(r.nameId, c.program.n) and inValueRef(r.valueRef, valCap) and inEnvRoot(r.parent, c.state.e)),
      s.valueSlots.foldl(true, (ok, r) => ok and inValueRef(r, valCap)),
      s.frameSlots.foldl(true, (ok, r) => ok and inFrameRow(r, c, valCap)),
      s.controlRows.foldl(true, (ok, r) => ok and inControlRow(r, c, valCap)),
    }
  }

  pure def validEnvelopeRanges(env: LiteralEnvelope): bool =
    inTag(env.ruleTag, 39) and inU8(env.charge)

  // Unconditional scalar range framing emitted before gated equations.
  pure def unconditionalFraming(c: LiteralCapacities, input: ReturnPublicationStepInputs): bool = and {
    inBit(input.enable),
    validProgramRanges(c, input.program),
    validStateRanges(c, input.pre),
    validStateRanges(c, input.post),
    validEnvelopeRanges(input.envelope),
  }

  pure val zeroValueRef: ValueRef = { space: 0, index: 0 }
  pure val zeroEnvRoot: EnvRoot = { present: 0, index: 0 }
  pure val zeroSlice: Slice = { start: 0, len: 0 }
  pure val zeroBytes16: List[int] = List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)

  pure val zeroValueNode: ValueNode = {
    kind: 0, typeId: 0, ctorId: 0, uintBytes: zeroBytes16,
    boolValue: 0, fieldStart: 0, fieldLength: 0,
  }

  pure val zeroFrameRow: FrameRow = {
    tag: 0, nameId: 0, binaryOp: 0, unaryOp: 0, typeId: 0,
    expr0: 0, expr1: 0, savedEnv: zeroEnvRoot, valueRef: zeroValueRef,
    completedSlice: zeroSlice, remainingExprSlice: zeroSlice, armSlice: zeroSlice,
  }

  pure val zeroControlRow: ControlRow = {
    tag: 0, exprId: 0, valueRef: zeroValueRef, pendingFrame: zeroFrameRow,
    environment: zeroEnvRoot, nameId: 0, valueSlice: zeroSlice,
    functionPresent: 0, functionId: 0, bindingsPresent: 0,
    bindings: zeroEnvRoot, typeId: 0, armSlice: zeroSlice,
    primitiveIsOk: 0, errorCode: 0,
  }

  pure def returnControlRow0(vr: ValueRef): ControlRow =
    { ...zeroControlRow, tag: CONTROL_RETURN, valueRef: vr }

  pure def enabledPreGuards(input: ReturnPublicationStepInputs): bool = {
    val row0 = input.pre.controlRows.nth(0)
    val retRef = row0.valueRef
    val selectedActive =
      if (retRef.space == SPACE_DATA) input.pre.root.activeData
      else input.program.root.activeConstants
    and {
      input.envelope.ruleTag == RETURN_PUBLICATION_RULE_TAG,
      input.envelope.charge == RETURN_PUBLICATION_RULE_CHARGE,
      input.pre.root.outcomeTag == OUTCOME_PENDING,
      input.pre.root.outcomeValue == zeroValueRef,
      input.pre.root.outcomeError == 0,
      input.pre.root.depth == 0,
      input.pre.frameSlots.foldl(true, (ok, fr) => ok and (fr == zeroFrameRow)),
      input.pre.root.controlLength == 1,
      row0 == returnControlRow0(retRef),
      range(1, input.pre.controlRows.length()).foldl(true, (ok, idx) =>
        ok and (input.pre.controlRows.nth(idx) == zeroControlRow)),
      0 <= retRef.index and retRef.index < selectedActive,
    }
  }

  pure def computePublishedPost(pre: StateImage): StateImage = {
    val retRef = pre.controlRows.nth(0).valueRef
    val clearedControls = pre.controlRows.foldl(List(), (acc, _) => acc.append(zeroControlRow))
    {
      ...pre,
      root: {
        ...pre.root,
        controlLength: 0,
        outcomeTag: OUTCOME_DONE,
        outcomeValue: retRef,
        outcomeError: 0,
      },
      controlRows: clearedControls,
    }
  }

  pure def enabledPostConstraints(input: ReturnPublicationStepInputs): bool =
    input.post == computePublishedPost(input.pre)

  // Complete local instruction-fragment relation for emit_return_publication_step.
  pure def returnPublicationStepConstraint(c: LiteralCapacities, input: ReturnPublicationStepInputs): bool = and {
    validShape(c, input),
    unconditionalFraming(c, input),
    (input.enable == 1) implies and {
      enabledPreGuards(input),
      enabledPostConstraints(input),
    },
  }

  // Default bounded capacities and fixture state for concrete module entry point.
  pure val defaultCaps: LiteralCapacities = {
    program: {
      n: 0, nb: 0, o: 1, op: 0, x: 1, xc: 0, f: 0, p: 0,
      r: 0, a: 0, t: 4, q: 4, qf: 5, k: 2, kf: 0,
    },
    state: { d: 2, df: 1, e: 1, v: 1, s: 1, c: 2, stackLimit: 4 },
  }

  pure val emptyNames: List[NameRow] = List()
  pure val emptyNameBytes: List[NameByteRow] = List()
  pure val emptyOccPaths: List[OccurrencePathRow] = List()
  pure val emptyExprChildren: List[ExprChildRow] = List()
  pure val emptyFunctions: List[FunctionRow] = List()
  pure val emptyParameters: List[ParameterRow] = List()
  pure val emptyPatterns: List[PatternRow] = List()
  pure val emptyArms: List[ArmRow] = List()
  pure val emptyConstFields: List[ValueRef] = List()

  pure val defaultProgram: ProgramImage = {
    root: {
      entryExprId: 0, activeNames: 0, activeNameBytes: 0,
      activeOccurrences: 1, activeOccurrencePaths: 0, activeExpressions: 1,
      activeExprChildren: 0, activeFunctions: 0, activeParameters: 0,
      activePatterns: 0, activeArms: 0, activeTypes: 4,
      activeConstructors: 4, activeConstructorFieldTypes: 5,
      activeConstants: 1, activeConstantFields: 0,
    },
    names: emptyNames,
    nameBytes: emptyNameBytes,
    occurrences: List({ ownerTag: 0, functionIndex: 0, pathStart: 0, pathLength: 0 }),
    occurrencePaths: emptyOccPaths,
    expressions: List({
      tag: 0, occurrenceId: 0, valueRef: { space: SPACE_CONSTANT, index: 0 },
      nameId: 0, binaryOp: 0, unaryOp: 0, childSlice: zeroSlice, armSlice: zeroSlice,
    }),
    exprChildren: emptyExprChildren,
    functions: emptyFunctions,
    parameters: emptyParameters,
    patterns: emptyPatterns,
    arms: emptyArms,
    types: List(
      { kind: 0, ctorStart: 0, ctorLength: 0 },
      { kind: 1, ctorStart: 0, ctorLength: 0 },
      { kind: 2, ctorStart: 0, ctorLength: 2 },
      { kind: 3, ctorStart: 2, ctorLength: 2 }
    ),
    constructors: List(
      { owningType: 2, sourceOrder: 0, fieldTypeSlice: { start: 0, len: 0 } },
      { owningType: 2, sourceOrder: 1, fieldTypeSlice: { start: 0, len: 2 } },
      { owningType: 3, sourceOrder: 0, fieldTypeSlice: { start: 2, len: 1 } },
      { owningType: 3, sourceOrder: 1, fieldTypeSlice: { start: 3, len: 2 } }
    ),
    constructorFieldTypes: List(
      { typeId: 0 }, { typeId: 2 }, { typeId: 0 }, { typeId: 3 }, { typeId: 3 }
    ),
    constants: List(
      { ...zeroValueNode, uintBytes: List(7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0) },
      zeroValueNode
    ),
    constantFields: emptyConstFields,
  }

  pure val defaultPreState: StateImage = {
    root: {
      activeData: 2, activeDataFields: 1, activeEnvCells: 1,
      activeValueSlots: 1, depth: 0, controlLength: 1,
      environment: { present: 1, index: 0 }, stackLimit: 4,
      remainingFuel: zeroBytes16,
      initialFuel: List(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
      outcomeTag: OUTCOME_PENDING, outcomeValue: zeroValueRef, outcomeError: 0,
    },
    data: List(
      { ...zeroValueNode, uintBytes: List(42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0) },
      { ...zeroValueNode, kind: 1, typeId: 1, boolValue: 1 }
    ),
    dataFields: List({ space: SPACE_DATA, index: 0 }),
    envCells: List({ nameId: 0, valueRef: { space: SPACE_CONSTANT, index: 0 }, parent: zeroEnvRoot }),
    valueSlots: List({ space: SPACE_DATA, index: 1 }),
    frameSlots: List(zeroFrameRow),
    controlRows: List(returnControlRow0({ space: SPACE_CONSTANT, index: 0 }), zeroControlRow),
  }

  var caps: LiteralCapacities
  var program: ProgramImage
  var state: StateImage
  var envelope: LiteralEnvelope
  var published: bool

  action initWith(c: LiteralCapacities, prog: ProgramImage, st: StateImage): bool = all {
    validMinimumShape(c),
    validProgramShape(c, prog),
    validProgramRanges(c, prog),
    validStateShape(c, st),
    validStateRanges(c, st),
    caps' = c,
    program' = prog,
    state' = st,
    envelope' = { ruleTag: RETURN_PUBLICATION_RULE_TAG, charge: RETURN_PUBLICATION_RULE_CHARGE },
    published' = false,
  }

  action init: bool = initWith(defaultCaps, defaultProgram, defaultPreState)

  // Single enabled guarded action; blocks rather than stuttering when row80 guards fail.
  action step: bool = {
    val postState = computePublishedPost(state)
    val env = { ruleTag: RETURN_PUBLICATION_RULE_TAG, charge: RETURN_PUBLICATION_RULE_CHARGE }
    val candidate: ReturnPublicationStepInputs = {
      enable: 1, program: program, pre: state, post: postState, envelope: env,
    }
    all {
      returnPublicationStepConstraint(caps, candidate),
      caps' = caps,
      program' = program,
      state' = postState,
      envelope' = env,
      published' = true,
    }
  }

  val isPublishedDoneState: bool = and {
    published,
    state.root.outcomeTag == OUTCOME_DONE,
    state.root.controlLength == 0,
    state.root.outcomeError == 0,
  }

  // Witness predicate negated for reachability checking (falsified on publication).
  val reachabilityWitnessNotPublished: bool = not(isPublishedDoneState)

  // Genuine state invariant over the physical projection.
  val returnPublicationSafetyInvariant: bool = and {
    validMinimumShape(caps),
    validProgramShape(caps, program),
    validProgramRanges(caps, program),
    validStateShape(caps, state),
    validStateRanges(caps, state),
    validEnvelopeRanges(envelope),
    published implies and {
      state.root.outcomeTag == OUTCOME_DONE,
      state.root.outcomeError == 0,
      state.root.controlLength == 0,
      state.root.depth == 0,
      state.root.outcomeValue.index < (
        if (state.root.outcomeValue.space == SPACE_DATA) state.root.activeData
        else program.root.activeConstants
      ),
      state.frameSlots.foldl(true, (ok, fr) => ok and (fr == zeroFrameRow)),
      state.controlRows.foldl(true, (ok, cr) => ok and (cr == zeroControlRow)),
      envelope == { ruleTag: RETURN_PUBLICATION_RULE_TAG, charge: RETURN_PUBLICATION_RULE_CHARGE },
    },
  }
}
