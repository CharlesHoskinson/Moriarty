// Authored source QA suite for `literalUiValid.qnt`.
// Exercises positive boundary constructors (UInt128 0 and 2^128-1 via 16 u8
// lanes, Bool 0/1, List Nil/Cons, Tree Leaf/Branch with shared DAG child,
// zero-capacity `KF_CAP == 0`, and disabled `enable == false` gating) and
// negative/rejection cases (self/forward DAG cycles, wrong child types, wrong
// ValueRef space, out-of-bounds slices, non-zero unused/inactive lanes,
// corrupted Fixed-A tables, and out-of-range u8 lanes).

module literalUiValidTest {
  import literalUiValid(
    T_CAP = 5,
    Q_CAP = 5,
    QF_CAP = 6,
    K_CAP = 6,
    KF_CAP = 6
  ).* from "./literalUiValid"

  import literalUiValid(
    T_CAP = 4,
    Q_CAP = 4,
    QF_CAP = 5,
    K_CAP = 3,
    KF_CAP = 0
  ) as zeroKf from "./literalUiValid"

  // 1. Verify explicit capacity bounds on both concrete instances.
  run capacityBoundsTest: bool = all {
    capacitiesValid,
    zeroKf::capacitiesValid,
  }

  // 2. Positive boundary test: UInt128 0, mixed bytes, and 2^128 - 1 (all 16 bytes = 255).
  run uint128BoundaryTest: bool = {
    val pZero = buildScalarProgram(false, false, 0)
    val pMax = buildScalarProgram(false, true, 0)
    val selMax = pMax.constants.get(0)
    all {
      reconstructUInt128(zeroBytes) == 0,
      reconstructUInt128(maxBytes) == POW2_128 - 1,
      circuitUiValidGate(pZero, true, 0, pZero.constants.get(0)),
      circuitUiValidGate(pMax, true, 0, selMax),
      init
        .then(stepScalarAction(false, true, 0))
        .expect(and {
          invCircuitImpliesKValid,
          invUInt128ByteLanesExact,
          witnessUInt128MaxSelected,
        }),
    }
  }

  // 3. Positive boundary test: Bool(0) and Bool(1).
  run boolBoundaryTest: bool = {
    val pFalse = buildScalarProgram(true, false, 0)
    val pTrue = buildScalarProgram(true, false, 1)
    all {
      circuitUiValidGate(pFalse, true, 0, pFalse.constants.get(0)),
      circuitUiValidGate(pTrue, true, 0, pTrue.constants.get(0)),
      init
        .then(stepScalarAction(true, false, 1))
        .expect(and {
          invCircuitImpliesKValid,
          invSelectedRootFetchSound,
          witnessBoolSelected,
        }),
    }
  }

  // 4. Positive recursive list test: Nil and nested Cons(UInt, Cons(UInt, Nil)).
  run listNilAndNestedConsTest: bool = {
    val pList = buildListProgram
    all {
      circuitUiValidGate(pList, true, 2, pList.constants.get(2)),
      circuitUiValidGate(pList, true, 3, pList.constants.get(3)),
      circuitUiValidGate(pList, true, 4, pList.constants.get(4)),
      init
        .then(stepListAction(4))
        .expect(and {
          invCircuitImpliesKValid,
          invStrictDagAndZeroPadding,
          invSelectedRootFetchSound,
          witnessConsListSelected,
        }),
    }
  }

  // 5. Positive recursive tree test: Leaf, Branch(Leaf, Leaf), and shared DAG Branch(b, b).
  run treeLeafAndSharedBranchDagTest: bool = {
    val pTree = buildTreeProgram
    all {
      circuitUiValidGate(pTree, true, 2, pTree.constants.get(2)),
      circuitUiValidGate(pTree, true, 4, pTree.constants.get(4)),
      circuitUiValidGate(pTree, true, 5, pTree.constants.get(5)),
      init
        .then(stepTreeAction(5))
        .expect(and {
          invCircuitImpliesKValid,
          invStrictDagAndZeroPadding,
          invSelectedRootFetchSound,
          witnessBranchTreeSelected,
        }),
    }
  }

  // 6. Positive and negative boundary test for legal zero-capacity `KF_CAP == 0`.
  run zeroKfCapacityBoundaryTest: bool = {
    val pZeroKfNil = {
      ...zeroKf::baseProgramSlice,
      activeConstants: 3,
      activeConstantFields: 0,
      constants: zeroKf::indexSet(3).mapBy(i =>
        if (i == 0) zeroKf::mkUIntRow(zeroKf::maxBytes)
        else if (i == 1) zeroKf::mkBoolRow(1)
        else zeroKf::mkCtorRow(2, 0, 0, 0)
      ),
    }
    val pZeroKfLeafBad = {
      ...pZeroKfNil,
      constants: pZeroKfNil.constants.put(2, zeroKf::mkCtorRow(3, 2, 0, 0)),
    }
    all {
      zeroKf::circuitUiValidGate(pZeroKfNil, true, 0, pZeroKfNil.constants.get(0)),
      zeroKf::circuitUiValidGate(pZeroKfNil, true, 1, pZeroKfNil.constants.get(1)),
      zeroKf::circuitUiValidGate(pZeroKfNil, true, 2, pZeroKfNil.constants.get(2)),
      not(zeroKf::circuitUiValidGate(pZeroKfLeafBad, true, 2, pZeroKfLeafBad.constants.get(2))),
    }
  }

  // 7. Disabled gate test (`enable == false`): out-of-bounds root index allowed when range-valid.
  run disabledEnableGateTest: bool =
    init
      .then(stepDisabledAction(99))
      .expect(and {
        invCircuitImpliesKValid,
        invSelectedRootFetchSound,
        witnessDisabledOutOfBoundsRoot,
      })

  // 8. Negative test: self-reference (`f_index == i`) and forward-reference (`f_index > i`) in DAG.
  run negSelfAndForwardReferenceDagTest: bool = {
    val pList = buildListProgram
    // Self-reference: Cons at row 3 has tail (cf slot 1) pointing to row 3 instead of row 2.
    val pSelfRef = {
      ...pList,
      constantFields: pList.constantFields.put(1, { space: 0, index: 3 }),
    }
    // Forward-reference: Cons at row 3 has tail pointing to later Cons at row 4 (`4 > 3`).
    val pForwardRef = {
      ...pList,
      constantFields: pList.constantFields.put(1, { space: 0, index: 4 }),
    }
    all {
      not(circuitUiValidGate(pSelfRef, true, 3, pSelfRef.constants.get(3))),
      not(circuitUiValidGate(pForwardRef, true, 3, pForwardRef.constants.get(3))),
    }
  }

  // 9. Negative test: wrong child types for Cons, Leaf, and Branch.
  run negWrongChildTypeTest: bool = {
    val pList = buildListProgram
    // Replace row 0 (UInt head of Cons at row 4) with Bool(true) -> wrong head type for Cons.
    val pConsBadHead = {
      ...pList,
      constants: pList.constants.put(0, mkBoolRow(1)),
    }
    // Cons at row 3 with tail pointing to UInt at row 0 instead of List at row 2.
    val pConsBadTail = {
      ...pList,
      constantFields: pList.constantFields.put(1, { space: 0, index: 0 }),
    }
    val pTree = buildTreeProgram
    // Leaf at row 2 pointing to Bool at row 0 instead of UInt.
    val pLeafBadChild = {
      ...pTree,
      constants: pTree.constants.put(0, mkBoolRow(1)),
    }
    // Branch at row 4 with left child pointing to UInt at row 0 instead of Tree at row 2.
    val pBranchBadChild = {
      ...pTree,
      constantFields: pTree.constantFields.put(2, { space: 0, index: 0 }),
    }
    all {
      not(circuitUiValidGate(pConsBadHead, true, 4, pConsBadHead.constants.get(4))),
      not(circuitUiValidGate(pConsBadTail, true, 3, pConsBadTail.constants.get(3))),
      not(circuitUiValidGate(pLeafBadChild, true, 2, pLeafBadChild.constants.get(2))),
      not(circuitUiValidGate(pBranchBadChild, true, 4, pBranchBadChild.constants.get(4))),
    }
  }

  // 10. Negative test: wrong ValueRef space (`Data = 1`), Nil with fields, and slice overflow.
  run negWrongSpaceAndFieldSliceTest: bool = {
    val pList = buildListProgram
    val pDataSpace = {
      ...pList,
      constantFields: pList.constantFields.put(0, { space: 1, index: 1 }),
    }
    val pNilNonZeroStart = {
      ...pList,
      constants: pList.constants.put(2, mkCtorRow(2, 0, 1, 0)),
    }
    val pSliceExceedsActiveCf = {
      ...pList,
      activeConstantFields: 3,
      constantFields: pList.constantFields.put(3, zeroCfRow),
    }
    val pWrongOwningType = {
      ...pList,
      constants: pList.constants.put(2, mkCtorRow(3, 0, 0, 0)),
    }
    all {
      not(circuitUiValidGate(pDataSpace, true, 3, pDataSpace.constants.get(3))),
      not(circuitUiValidGate(pNilNonZeroStart, true, 2, pNilNonZeroStart.constants.get(2))),
      not(circuitUiValidGate(pSliceExceedsActiveCf, true, 4, pSliceExceedsActiveCf.constants.get(4))),
      not(circuitUiValidGate(pWrongOwningType, true, 2, pWrongOwningType.constants.get(2))),
    }
  }

  // 11. Negative test: non-zero unused scalar lanes, inactive row padding, and Fixed-A table corruption.
  run negNonZeroUnusedLanesAndInactivePaddingTest: bool = {
    val pBase = baseProgramSlice
    val pUIntWithBoolBit = {
      ...pBase,
      constants: pBase.constants.put(0, { ...zeroConstantRow, boolValue: 1 }),
    }
    val pBoolWithUIntByte = {
      ...pBase,
      constants: pBase.constants.put(0, {
        ...mkBoolRow(1),
        uintBytes: byteMapFromPair(1, 0),
      }),
    }
    val pDirtyInactiveConst = {
      ...pBase,
      constants: pBase.constants.put(1, mkBoolRow(1)),
    }
    val pDirtyInactiveCf = {
      ...pBase,
      constantFields: pBase.constantFields.put(0, { space: 0, index: 1 }),
    }
    val pCorruptFixedTypes = {
      ...pBase,
      types: pBase.types.put(2, { kind: 2, ctorStart: 0, ctorLength: 1 }),
    }
    all {
      not(circuitUiValidGate(pUIntWithBoolBit, true, 0, pUIntWithBoolBit.constants.get(0))),
      not(circuitUiValidGate(pBoolWithUIntByte, true, 0, pBoolWithUIntByte.constants.get(0))),
      not(circuitUiValidGate(pDirtyInactiveConst, true, 0, pDirtyInactiveConst.constants.get(0))),
      not(circuitUiValidGate(pDirtyInactiveCf, true, 0, pDirtyInactiveCf.constants.get(0))),
      not(circuitUiValidGate(pCorruptFixedTypes, true, 0, pCorruptFixedTypes.constants.get(0))),
    }
  }

  // 12. Negative test: out-of-range u8 payload lane (>255) and enabled out-of-bounds root index.
  run negOutOfBoundsByteAndRootIndexTest: bool = {
    val pBase = baseProgramSlice
    val badRow = mkUIntRow(byteMapFromPair(256, 0))
    val pBadByte = {
      ...pBase,
      constants: pBase.constants.put(0, badRow),
    }
    all {
      // Out-of-range byte (>255) fails unconditional `image_ranges` even when `enable == false`.
      not(circuitUiValidGate(pBadByte, false, 0, zeroConstantRow)),
      not(circuitUiValidGate(pBadByte, true, 0, badRow)),
      // Enabled root selection with `constantIndex >= activeConstants` fails root fetch.
      not(circuitUiValidGate(pBase, true, 1, zeroConstantRow)),
    }
  }
}
