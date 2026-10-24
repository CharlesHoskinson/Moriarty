// Tracked finite Quint design projection for `literal_ui_valid.rs`.
// Models in-circuit recursive `uiValid` constraints over Fixed-A builtin type
// tables (`Types`, `Constructors`, `ConstructorFieldTypes`), `ConstantFields`,
// `Constants` (including 16-lane UInt8 payload for UInt128, Bool, List Nil/Cons,
// and Tree Leaf/Branch with strict child-before-parent DAG indices), and
// enabled root constant selection, paired with an independent bottom-up K
// semantic sort/validity evaluator from `models/sherlock/k/main.k`.

module literalUiValid {
  // Static finite capacity bounds (projection parameters only, never admission policy).
  const T_CAP: int
  const Q_CAP: int
  const QF_CAP: int
  const K_CAP: int
  const KF_CAP: int

  // Exact 2^128 upper bound from `models/sherlock/k/main.k` line 210.
  pure val POW2_128: int = 340282366920938463463374607431768211456
  pure val BYTE_INDICES: List[int] =
    List(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
  pure val BYTE_KEY_SET: Set[int] = 0.to(15)

  type TypeRow = {
    kind: int,
    ctorStart: int,
    ctorLength: int,
  }

  type CtorRow = {
    owningType: int,
    sourceOrder: int,
    fieldTypeStart: int,
    fieldTypeLen: int,
  }

  type CftRow = {
    typeId: int,
  }

  type ConstantRow = {
    kind: int,
    typeId: int,
    ctorId: int,
    uintBytes: int -> int,
    boolValue: int,
    fieldStart: int,
    fieldLength: int,
  }

  type ConstantFieldRow = {
    space: int,
    index: int,
  }

  type ProgramSlice = {
    activeTypes: int,
    activeConstructors: int,
    activeConstructorFieldTypes: int,
    activeConstants: int,
    activeConstantFields: int,
    types: int -> TypeRow,
    constructors: int -> CtorRow,
    constructorFieldTypes: int -> CftRow,
    constants: int -> ConstantRow,
    constantFields: int -> ConstantFieldRow,
  }

  type KSort =
    | KSortUInt
    | KSortBool
    | KSortList
    | KSortTree
    | KSortInvalid

  type StepTag =
    | StepInit
    | StepScalar
    | StepList
    | StepTree
    | StepDisabled

  type UiValidState = {
    enable: bool,
    constantIndex: int,
    program: ProgramSlice,
    selectedConstant: ConstantRow,
    accepted: bool,
    lastStep: StepTag,
  }

  // Safe index domain helper that avoids calling `0.to(-1)` when a capacity is 0.
  pure def indexSet(cap: int): Set[int] =
    if (cap <= 0) Set() else 0.to(cap - 1)

  pure def indexList(cap: int): List[int] =
    if (cap <= 0) List() else range(0, cap)

  // Explicit capacity precondition (not hidden as an unchecked `assume`).
  pure def capacitiesValid: bool = and {
    T_CAP >= 4,
    Q_CAP >= 4,
    QF_CAP >= 5,
    K_CAP >= 1,
    KF_CAP >= 0,
  }

  pure def zeroBytes: int -> int =
    BYTE_KEY_SET.mapBy(_ => 0)

  pure def maxBytes: int -> int =
    BYTE_KEY_SET.mapBy(_ => 255)

  pure def byteMapFromPair(b0: int, b15: int): int -> int =
    BYTE_KEY_SET.mapBy(idx => if (idx == 0) b0 else if (idx == 15) b15 else 0)

  pure def zeroConstantRow: ConstantRow = {
    kind: 0,
    typeId: 0,
    ctorId: 0,
    uintBytes: zeroBytes,
    boolValue: 0,
    fieldStart: 0,
    fieldLength: 0,
  }

  pure def zeroCfRow: ConstantFieldRow = {
    space: 0,
    index: 0,
  }

  pure def expectedTypeRow(r: int): TypeRow =
    if (r == 0) { kind: 0, ctorStart: 0, ctorLength: 0 }
    else if (r == 1) { kind: 1, ctorStart: 0, ctorLength: 0 }
    else if (r == 2) { kind: 2, ctorStart: 0, ctorLength: 2 }
    else if (r == 3) { kind: 3, ctorStart: 2, ctorLength: 2 }
    else { kind: 0, ctorStart: 0, ctorLength: 0 }

  pure def expectedCtorRow(r: int): CtorRow =
    if (r == 0) { owningType: 2, sourceOrder: 0, fieldTypeStart: 0, fieldTypeLen: 0 }
    else if (r == 1) { owningType: 2, sourceOrder: 1, fieldTypeStart: 0, fieldTypeLen: 2 }
    else if (r == 2) { owningType: 3, sourceOrder: 0, fieldTypeStart: 2, fieldTypeLen: 1 }
    else if (r == 3) { owningType: 3, sourceOrder: 1, fieldTypeStart: 3, fieldTypeLen: 2 }
    else { owningType: 0, sourceOrder: 0, fieldTypeStart: 0, fieldTypeLen: 0 }

  pure def expectedCftRow(r: int): CftRow =
    if (r == 0) { typeId: 0 }
    else if (r == 1) { typeId: 2 }
    else if (r == 2) { typeId: 0 }
    else if (r == 3) { typeId: 3 }
    else if (r == 4) { typeId: 3 }
    else { typeId: 0 }

  // Reconstruct mathematical UInt128 from the 16 little-endian u8 payload lanes.
  pure def reconstructUInt128(bytes: int -> int): int =
    BYTE_INDICES.foldl(
      { total: 0, factor: 1 },
      (acc, idx) => {
        total: acc.total + bytes.get(idx) * acc.factor,
        factor: acc.factor * 256,
      }
    ).total

  // Unconditional lane range constraints (`Emitter::image_ranges` in `literal_ui_valid.rs`).
  pure def uintBytesRangeOk(bytes: int -> int): bool = and {
    bytes.keys() == BYTE_KEY_SET,
    BYTE_KEY_SET.forall(b => 0 <= bytes.get(b) and bytes.get(b) <= 255),
  }

  pure def constantRowRangesOk(row: ConstantRow): bool = and {
    0 <= row.kind and row.kind <= 2,
    0 <= row.typeId,
    0 <= row.ctorId,
    uintBytesRangeOk(row.uintBytes),
    row.boolValue == 0 or row.boolValue == 1,
    0 <= row.fieldStart and row.fieldStart <= KF_CAP,
    0 <= row.fieldLength and row.fieldLength <= KF_CAP,
  }

  pure def cfRowRangesOk(row: ConstantFieldRow): bool = and {
    row.space == 0 or row.space == 1,
    0 <= row.index,
  }

  pure def programRangesOk(p: ProgramSlice): bool = and {
    capacitiesValid,
    0 <= p.activeTypes and p.activeTypes <= T_CAP,
    0 <= p.activeConstructors and p.activeConstructors <= Q_CAP,
    0 <= p.activeConstructorFieldTypes and p.activeConstructorFieldTypes <= QF_CAP,
    0 <= p.activeConstants and p.activeConstants <= K_CAP,
    0 <= p.activeConstantFields and p.activeConstantFields <= KF_CAP,
    p.types.keys() == indexSet(T_CAP),
    p.constructors.keys() == indexSet(Q_CAP),
    p.constructorFieldTypes.keys() == indexSet(QF_CAP),
    p.constants.keys() == indexSet(K_CAP),
    p.constantFields.keys() == indexSet(KF_CAP),
    indexSet(T_CAP).forall(r =>
      val tr = p.types.get(r)
      0 <= tr.kind and tr.kind <= 3 and 0 <= tr.ctorStart and tr.ctorStart <= Q_CAP
        and 0 <= tr.ctorLength and tr.ctorLength <= Q_CAP
    ),
    indexSet(Q_CAP).forall(r =>
      val cr = p.constructors.get(r)
      0 <= cr.owningType and 0 <= cr.sourceOrder and cr.sourceOrder <= Q_CAP
        and 0 <= cr.fieldTypeStart and cr.fieldTypeStart <= QF_CAP
        and 0 <= cr.fieldTypeLen and cr.fieldTypeLen <= QF_CAP
    ),
    indexSet(QF_CAP).forall(r => 0 <= p.constructorFieldTypes.get(r).typeId),
    indexSet(K_CAP).forall(r => constantRowRangesOk(p.constants.get(r))),
    indexSet(KF_CAP).forall(r => cfRowRangesOk(p.constantFields.get(r))),
  }

  // Section 1 of `literal_ui_valid.rs`: Fixed-A builtin header counts and tables.
  pure def fixedTablesOk(p: ProgramSlice): bool = and {
    p.activeTypes == 4,
    p.activeConstructors == 4,
    p.activeConstructorFieldTypes == 5,
    indexSet(T_CAP).forall(r => p.types.get(r) == expectedTypeRow(r)),
    indexSet(Q_CAP).forall(r => p.constructors.get(r) == expectedCtorRow(r)),
    indexSet(QF_CAP).forall(r => p.constructorFieldTypes.get(r) == expectedCftRow(r)),
  }

  // Section 2 of `literal_ui_valid.rs`: `ConstantFields` table constraints.
  pure def constantFieldsTableOk(p: ProgramSlice): bool =
    if (KF_CAP == 0) (
      p.activeConstantFields == 0
    ) else (
      indexSet(KF_CAP).forall(j =>
        val cf = p.constantFields.get(j)
        cf.space == 0 and (
          if (j < p.activeConstantFields)
            (0 <= cf.index and cf.index < p.activeConstants)
          else
            (cf.index == 0)
        )
      )
    )

  // Section 3 helper: Constructor field slot 0 (`Cons`, `Leaf`, `Branch`).
  pure def fieldSlot0Ok(p: ProgramSlice, i: int, row: ConstantRow, cRow: CtorRow): bool =
    if (0 <= cRow.fieldTypeStart and cRow.fieldTypeStart < p.activeConstructorFieldTypes
        and 0 <= row.fieldStart and row.fieldStart < p.activeConstantFields) {
      val declF0Type = p.constructorFieldTypes.get(cRow.fieldTypeStart).typeId
      val cf0 = p.constantFields.get(row.fieldStart)
      if (cf0.space == 0 and 0 <= cf0.index and cf0.index < i and cf0.index < p.activeConstants) {
        val child0Type = p.constants.get(cf0.index).typeId
        val expF0Type = if (row.ctorId == 3) 3 else 0
        child0Type == declF0Type and child0Type == expF0Type
      } else false
    } else false

  // Section 3 helper: Constructor field slot 1 (`Cons`, `Branch`).
  pure def fieldSlot1Ok(p: ProgramSlice, i: int, row: ConstantRow, cRow: CtorRow): bool =
    val cft1Idx = cRow.fieldTypeStart + 1
    val cf1Idx = row.fieldStart + 1
    if (0 <= cft1Idx and cft1Idx < p.activeConstructorFieldTypes
        and 0 <= cf1Idx and cf1Idx < p.activeConstantFields) {
      val declF1Type = p.constructorFieldTypes.get(cft1Idx).typeId
      val cf1 = p.constantFields.get(cf1Idx)
      if (cf1.space == 0 and 0 <= cf1.index and cf1.index < i and cf1.index < p.activeConstants) {
        val child1Type = p.constants.get(cf1.index).typeId
        val expF1Type = if (row.ctorId == 3) 3 else 2
        child1Type == declF1Type and child1Type == expF1Type
      } else false
    } else false

  // Section 3 of `literal_ui_valid.rs`: per-row `Constants` typing, zeroing, and DAG checks.
  pure def activeConstantRowOk(p: ProgramSlice, i: int, row: ConstantRow): bool =
    if (0 <= row.typeId and row.typeId < p.activeTypes) {
      val tRow = p.types.get(row.typeId)
      val baseTypeOk = (tRow.kind == row.typeId)
      if (row.kind == 0) {
        and {
          baseTypeOk,
          row.typeId == 0,
          row.ctorId == 0,
          row.boolValue == 0,
          row.fieldStart == 0,
          row.fieldLength == 0,
          tRow.ctorLength == 0,
        }
      } else if (row.kind == 1) {
        and {
          baseTypeOk,
          row.typeId == 1,
          row.ctorId == 0,
          row.fieldStart == 0,
          row.fieldLength == 0,
          tRow.ctorLength == 0,
          row.uintBytes == zeroBytes,
        }
      } else if (row.kind == 2) {
        if (0 <= row.ctorId and row.ctorId < p.activeConstructors) {
          val cRow = p.constructors.get(row.ctorId)
          val isNil = (row.ctorId == 0)
          val isCons = (row.ctorId == 1)
          val isLeaf = (row.ctorId == 2)
          val isBranch = (row.ctorId == 3)
          val hasF1 = (isCons or isBranch)
          val hasF0 = (hasF1 or isLeaf)
          and {
            baseTypeOk,
            row.uintBytes == zeroBytes,
            row.boolValue == 0,
            row.typeId == cRow.owningType,
            row.fieldLength == cRow.fieldTypeLen,
            cRow.sourceOrder < tRow.ctorLength,
            isNil or isCons or isLeaf or isBranch,
            isNil implies (row.fieldStart == 0 and row.fieldLength == 0),
            row.fieldStart + row.fieldLength <= p.activeConstantFields,
            if (KF_CAP == 0)
              (not(hasF0) and not(hasF1))
            else
              ((hasF0 implies fieldSlot0Ok(p, i, row, cRow))
                and (hasF1 implies fieldSlot1Ok(p, i, row, cRow))),
          }
        } else false
      } else false
    } else false

  pure def constantsTableOk(p: ProgramSlice): bool =
    indexSet(K_CAP).forall(i =>
      val row = p.constants.get(i)
      if (i < p.activeConstants)
        activeConstantRowOk(p, i, row)
      else
        row == zeroConstantRow
    )

  // Complete circuit constraint predicate emitted by `emit_literal_ui_valid_with_capacities`.
  pure def circuitUiValidGate(
    p: ProgramSlice,
    enable: bool,
    constantIndex: int,
    selectedConstant: ConstantRow
  ): bool = and {
    0 <= constantIndex and constantIndex < POW2_128,
    programRangesOk(p),
    constantRowRangesOk(selectedConstant),
    if (enable) and {
      fixedTablesOk(p),
      constantFieldsTableOk(p),
      constantsTableOk(p),
      0 <= constantIndex and constantIndex < p.activeConstants,
      selectedConstant == p.constants.get(constantIndex),
    } else (
      selectedConstant == zeroConstantRow
    ),
  }

  // Independent K-specification (`models/sherlock/k/main.k`) inductive sort & validity
  // evaluator. Computes each node's K sort bottom-up via `foldl` over `0..K_CAP-1`
  // without reading `Types`, `Constructors`, or `ConstructorFieldTypes` tables.
  pure def kSortMatchesTypeId(s: KSort, typeId: int): bool =
    if (s == KSortUInt) typeId == 0
    else if (s == KSortBool) typeId == 1
    else if (s == KSortList) typeId == 2
    else if (s == KSortTree) typeId == 3
    else false

  pure def kEvalNodeSort(p: ProgramSlice, i: int, earlierSorts: int -> KSort): KSort =
    val row = p.constants.get(i)
    if (row.kind == 0) {
      val n = reconstructUInt128(row.uintBytes)
      if (row.typeId == 0 and row.ctorId == 0 and row.boolValue == 0
          and row.fieldStart == 0 and row.fieldLength == 0
          and 0 <= n and n < POW2_128)
        KSortUInt
      else KSortInvalid
    } else if (row.kind == 1) {
      if (row.typeId == 1 and row.ctorId == 0 and row.uintBytes == zeroBytes
          and (row.boolValue == 0 or row.boolValue == 1)
          and row.fieldStart == 0 and row.fieldLength == 0)
        KSortBool
      else KSortInvalid
    } else if (row.kind == 2 and row.uintBytes == zeroBytes and row.boolValue == 0) {
      if (row.ctorId == 0 and row.typeId == 2 and row.fieldStart == 0 and row.fieldLength == 0) {
        KSortList
      } else if (row.ctorId == 1 and row.typeId == 2 and row.fieldLength == 2
          and 0 <= row.fieldStart and row.fieldStart + 2 <= p.activeConstantFields
          and row.fieldStart + 1 < KF_CAP) {
        val cf0 = p.constantFields.get(row.fieldStart)
        val cf1 = p.constantFields.get(row.fieldStart + 1)
        if (cf0.space == 0 and cf1.space == 0
            and 0 <= cf0.index and cf0.index < i
            and 0 <= cf1.index and cf1.index < i
            and earlierSorts.get(cf0.index) == KSortUInt
            and earlierSorts.get(cf1.index) == KSortList)
          KSortList
        else KSortInvalid
      } else if (row.ctorId == 2 and row.typeId == 3 and row.fieldLength == 1
          and 0 <= row.fieldStart and row.fieldStart + 1 <= p.activeConstantFields
          and row.fieldStart < KF_CAP) {
        val cf0 = p.constantFields.get(row.fieldStart)
        if (cf0.space == 0 and 0 <= cf0.index and cf0.index < i
            and earlierSorts.get(cf0.index) == KSortUInt)
          KSortTree
        else KSortInvalid
      } else if (row.ctorId == 3 and row.typeId == 3 and row.fieldLength == 2
          and 0 <= row.fieldStart and row.fieldStart + 2 <= p.activeConstantFields
          and row.fieldStart + 1 < KF_CAP) {
        val cf0 = p.constantFields.get(row.fieldStart)
        val cf1 = p.constantFields.get(row.fieldStart + 1)
        if (cf0.space == 0 and cf1.space == 0
            and 0 <= cf0.index and cf0.index < i
            and 0 <= cf1.index and cf1.index < i
            and earlierSorts.get(cf0.index) == KSortTree
            and earlierSorts.get(cf1.index) == KSortTree)
          KSortTree
        else KSortInvalid
      } else KSortInvalid
    } else KSortInvalid

  pure def kInductiveSorts(p: ProgramSlice): int -> KSort =
    val initSorts = indexSet(K_CAP).mapBy(_ => KSortInvalid)
    indexList(K_CAP).foldl(
      initSorts,
      (acc, i) =>
        if (i < p.activeConstants)
          acc.put(i, kEvalNodeSort(p, i, acc))
        else
          acc
    )

  pure def kSemanticRootValid(s: UiValidState): bool =
    val sorts = kInductiveSorts(s.program)
    and {
      0 <= s.constantIndex and s.constantIndex < s.program.activeConstants,
      indexSet(K_CAP).forall(i =>
        (i < s.program.activeConstants) implies (
          sorts.get(i) != KSortInvalid
            and kSortMatchesTypeId(sorts.get(i), s.program.constants.get(i).typeId)
        )
      ),
      sorts.get(s.constantIndex) != KSortInvalid,
      s.selectedConstant == s.program.constants.get(s.constantIndex),
    }

  // Canonical builders for Fixed-A tables and representative constant DAGs.
  pure def baseProgramSlice: ProgramSlice = {
    activeTypes: 4,
    activeConstructors: 4,
    activeConstructorFieldTypes: 5,
    activeConstants: 1,
    activeConstantFields: 0,
    types: indexSet(T_CAP).mapBy(r => expectedTypeRow(r)),
    constructors: indexSet(Q_CAP).mapBy(r => expectedCtorRow(r)),
    constructorFieldTypes: indexSet(QF_CAP).mapBy(r => expectedCftRow(r)),
    constants: indexSet(K_CAP).mapBy(_ => zeroConstantRow),
    constantFields: indexSet(KF_CAP).mapBy(_ => zeroCfRow),
  }

  pure def mkUIntRow(bytes: int -> int): ConstantRow = {
    ...zeroConstantRow,
    kind: 0,
    typeId: 0,
    uintBytes: bytes,
  }

  pure def mkBoolRow(bVal: int): ConstantRow = {
    ...zeroConstantRow,
    kind: 1,
    typeId: 1,
    boolValue: bVal,
  }

  pure def mkCtorRow(typeId: int, ctorId: int, fStart: int, fLen: int): ConstantRow = {
    ...zeroConstantRow,
    kind: 2,
    typeId: typeId,
    ctorId: ctorId,
    fieldStart: fStart,
    fieldLength: fLen,
  }

  pure def buildScalarProgram(isBool: bool, useMaxUInt: bool, bVal: int): ProgramSlice =
    val row0 =
      if (isBool) mkBoolRow(bVal)
      else if (useMaxUInt) mkUIntRow(maxBytes)
      else mkUIntRow(byteMapFromPair(42, 1))
    {
      ...baseProgramSlice,
      activeConstants: 1,
      activeConstantFields: 0,
      constants: indexSet(K_CAP).mapBy(i => if (i == 0) row0 else zeroConstantRow),
    }

  // List DAG: [0]=UInt(7), [1]=UInt(2^128-1), [2]=Nil, [3]=Cons(1,2), [4]=Cons(0,3)
  pure def buildListProgram: ProgramSlice = {
    ...baseProgramSlice,
    activeConstants: 5,
    activeConstantFields: 4,
    constants: indexSet(K_CAP).mapBy(i =>
      if (i == 0) mkUIntRow(byteMapFromPair(7, 0))
      else if (i == 1) mkUIntRow(maxBytes)
      else if (i == 2) mkCtorRow(2, 0, 0, 0)
      else if (i == 3) mkCtorRow(2, 1, 0, 2)
      else if (i == 4) mkCtorRow(2, 1, 2, 2)
      else zeroConstantRow
    ),
    constantFields: indexSet(KF_CAP).mapBy(j =>
      if (j == 0) { space: 0, index: 1 }
      else if (j == 1) { space: 0, index: 2 }
      else if (j == 2) { space: 0, index: 0 }
      else if (j == 3) { space: 0, index: 3 }
      else zeroCfRow
    ),
  }

  // Tree DAG with shared node: [0]=UInt(10), [1]=UInt(20), [2]=Leaf(0),
  // [3]=Leaf(1), [4]=Branch(2,3), [5]=Branch(4,4)
  pure def buildTreeProgram: ProgramSlice = {
    ...baseProgramSlice,
    activeConstants: 6,
    activeConstantFields: 6,
    constants: indexSet(K_CAP).mapBy(i =>
      if (i == 0) mkUIntRow(byteMapFromPair(10, 0))
      else if (i == 1) mkUIntRow(byteMapFromPair(20, 0))
      else if (i == 2) mkCtorRow(3, 2, 0, 1)
      else if (i == 3) mkCtorRow(3, 2, 1, 1)
      else if (i == 4) mkCtorRow(3, 3, 2, 2)
      else if (i == 5) mkCtorRow(3, 3, 4, 2)
      else zeroConstantRow
    ),
    constantFields: indexSet(KF_CAP).mapBy(j =>
      if (j == 0) { space: 0, index: 0 }
      else if (j == 1) { space: 0, index: 1 }
      else if (j == 2) { space: 0, index: 2 }
      else if (j == 3) { space: 0, index: 3 }
      else if (j == 4) { space: 0, index: 4 }
      else if (j == 5) { space: 0, index: 4 }
      else zeroCfRow
    ),
  }

  // Pure guard + state-update pairs (Step 3 of quint-modeling spine).
  pure def canTransition(
    p: ProgramSlice,
    enable: bool,
    rootIdx: int,
    selected: ConstantRow
  ): bool =
    circuitUiValidGate(p, enable, rootIdx, selected)

  pure def applyTransition(
    p: ProgramSlice,
    enable: bool,
    rootIdx: int,
    selected: ConstantRow,
    tag: StepTag
  ): UiValidState = {
    enable: enable,
    constantIndex: rootIdx,
    program: p,
    selectedConstant: selected,
    accepted: true,
    lastStep: tag,
  }

  // Single cohesive state variable.
  var state: UiValidState

  action init: bool = {
    val p = baseProgramSlice
    val sel = zeroConstantRow
    all {
      canTransition(p, true, 0, sel),
      state' = applyTransition(p, true, 0, sel, StepInit),
    }
  }

  action stepScalarAction(isBool: bool, useMaxUInt: bool, bVal: int): bool = {
    val p = buildScalarProgram(isBool, useMaxUInt, bVal)
    val sel = p.constants.get(0)
    all {
      canTransition(p, true, 0, sel),
      state' = applyTransition(p, true, 0, sel, StepScalar),
    }
  }

  action stepListAction(rootIdx: int): bool = {
    val p = buildListProgram
    val sel = if (0 <= rootIdx and rootIdx < K_CAP) p.constants.get(rootIdx) else zeroConstantRow
    all {
      K_CAP >= 5 and KF_CAP >= 4,
      canTransition(p, true, rootIdx, sel),
      state' = applyTransition(p, true, rootIdx, sel, StepList),
    }
  }

  action stepTreeAction(rootIdx: int): bool = {
    val p = buildTreeProgram
    val sel = if (0 <= rootIdx and rootIdx < K_CAP) p.constants.get(rootIdx) else zeroConstantRow
    all {
      K_CAP >= 6 and KF_CAP >= 6,
      canTransition(p, true, rootIdx, sel),
      state' = applyTransition(p, true, rootIdx, sel, StepTree),
    }
  }

  action stepDisabledAction(outOfBoundsIdx: int): bool = {
    val p = baseProgramSlice
    all {
      canTransition(p, false, outOfBoundsIdx, zeroConstantRow),
      state' = applyTransition(p, false, outOfBoundsIdx, zeroConstantRow, StepDisabled),
    }
  }

  action step: bool = {
    nondet isBool = Set(false, true).oneOf()
    nondet useMax = Set(false, true).oneOf()
    nondet bVal = Set(0, 1).oneOf()
    nondet listIdx = Set(2, 3, 4).oneOf()
    nondet treeIdx = Set(2, 4, 5).oneOf()
    nondet disIdx = Set(0, 10, 99).oneOf()
    any {
      stepScalarAction(isBool, useMax, bVal),
      stepListAction(listIdx),
      stepTreeAction(treeIdx),
      stepDisabledAction(disIdx),
    }
  }

  // Reachability witnesses (Step 5: one per major action / value family).
  val witnessUInt128MaxSelected: bool = and {
    state.enable,
    state.selectedConstant.kind == 0,
    reconstructUInt128(state.selectedConstant.uintBytes) == POW2_128 - 1,
  }

  val witnessBoolSelected: bool = and {
    state.enable,
    state.selectedConstant.kind == 1,
    state.selectedConstant.boolValue == 1,
  }

  val witnessConsListSelected: bool = and {
    state.enable,
    state.selectedConstant.kind == 2,
    state.selectedConstant.ctorId == 1,
  }

  val witnessBranchTreeSelected: bool = and {
    state.enable,
    state.selectedConstant.kind == 2,
    state.selectedConstant.ctorId == 3,
  }

  val witnessDisabledOutOfBoundsRoot: bool = and {
    not(state.enable),
    state.constantIndex >= state.program.activeConstants,
  }

  // Safety invariants (Step 5).
  // 1. Every circuit-accepted enabled state satisfies the independent K-spec
  //    bottom-up inductive sort and value validity judgment.
  val invCircuitImpliesKValid: bool =
    (state.accepted and state.enable) implies kSemanticRootValid(state)

  // 2. Strict child-before-parent DAG order (`f_index < i`), Constant space `0`,
  //    and zero-padding of inactive `Constants` and `ConstantFields` rows.
  val invStrictDagAndZeroPadding: bool =
    (state.accepted and state.enable) implies and {
      indexSet(K_CAP).forall(i =>
        val row = state.program.constants.get(i)
        if (i >= state.program.activeConstants)
          row == zeroConstantRow
        else if (row.kind == 2 and row.fieldLength > 0)
          indexSet(KF_CAP).forall(s =>
            (row.fieldStart <= s and s < row.fieldStart + row.fieldLength) implies (
              state.program.constantFields.get(s).space == 0
                and state.program.constantFields.get(s).index < i
            )
          )
        else true
      ),
      indexSet(KF_CAP).forall(j =>
        (j >= state.program.activeConstantFields) implies
          (state.program.constantFields.get(j) == zeroCfRow)
      ),
    }

  // 3. Enabled root constant selection soundness and disabled zero selection.
  val invSelectedRootFetchSound: bool =
    state.accepted implies (
      if (state.enable)
        (0 <= state.constantIndex
          and state.constantIndex < state.program.activeConstants
          and state.selectedConstant == state.program.constants.get(state.constantIndex))
      else
        (state.selectedConstant == zeroConstantRow)
    )

  // 4. Exact 16-lane UInt8 payload range (`0 <= N < 2^128`) and non-UInt zeroing.
  val invUInt128ByteLanesExact: bool =
    state.accepted implies indexSet(K_CAP).forall(i =>
      val row = state.program.constants.get(i)
      val n = reconstructUInt128(row.uintBytes)
      uintBytesRangeOk(row.uintBytes)
        and 0 <= n and n < POW2_128
        and ((state.enable and i < state.program.activeConstants and row.kind != 0)
              implies (row.uintBytes == zeroBytes))
    )
}

// Concrete default instance binding all `const` parameters.
module literalUiValidInstance {
  import literalUiValid(
    T_CAP = 5,
    Q_CAP = 5,
    QF_CAP = 6,
    K_CAP = 6,
    KF_CAP = 6
  ).*
}
