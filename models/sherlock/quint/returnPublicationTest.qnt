module returnPublicationTest {
  import returnPublication.* from "./returnPublication"

  pure val dataReturnState: StateImage = {
    ...defaultPreState,
    root: {
      ...defaultPreState.root,
      remainingFuel: List(9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255),
    },
    controlRows: List(returnControlRow0({ space: SPACE_DATA, index: 1 }), zeroControlRow),
  }

  // 1. Normal Constant-space return publication at zero remainingFuel, plus terminal blocking.
  run publishConstantZeroFuelTest =
    init
      .then(step)
      .expect(all {
        state == computePublishedPost(defaultPreState),
        state.root.outcomeTag == OUTCOME_DONE,
        state.root.outcomeValue == { space: SPACE_CONSTANT, index: 0 },
        state.root.remainingFuel == zeroBytes16,
        envelope == { ruleTag: 0, charge: 0 },
        isPublishedDoneState,
        returnPublicationSafetyInvariant,
      })
      .then(step.fail())

  // 2. Normal Data-space return publication (index 1 < activeData 2, while activeConstants == 1).
  run publishDataNonZeroFuelTest =
    initWith(defaultCaps, defaultProgram, dataReturnState)
      .then(step)
      .expect(all {
        state == computePublishedPost(dataReturnState),
        state.root.outcomeTag == OUTCOME_DONE,
        state.root.outcomeValue == { space: SPACE_DATA, index: 1 },
        state.root.remainingFuel == dataReturnState.root.remainingFuel,
        state.data == dataReturnState.data,
        state.dataFields == dataReturnState.dataFields,
        state.envCells == dataReturnState.envCells,
        state.valueSlots == dataReturnState.valueSlots,
        state.frameSlots == dataReturnState.frameSlots,
        isPublishedDoneState,
        returnPublicationSafetyInvariant,
      })
      .then(step.fail())

  // 3. Disabled enable (enable == 0) enforces unconditional scalar ranges but skips enabled guards.
  run disabledEnableUnconditionalRangeTest = {
    val mismatchedPost = {
      ...defaultPreState,
      root: { ...defaultPreState.root, outcomeTag: OUTCOME_REJECTED, outcomeError: 7 },
    }
    val validDisabled: ReturnPublicationStepInputs = {
      enable: 0,
      program: defaultProgram,
      pre: mismatchedPost,
      post: defaultPreState,
      envelope: { ruleTag: 38, charge: 255 },
    }
    val badEnable: ReturnPublicationStepInputs = { ...validDisabled, enable: 2 }
    val badRuleTag: ReturnPublicationStepInputs = {
      ...validDisabled, envelope: { ruleTag: 39, charge: 0 },
    }
    val badRootCount: ReturnPublicationStepInputs = {
      ...validDisabled,
      post: { ...defaultPreState, root: { ...defaultPreState.root, activeData: 3 } },
    }
    init.expect(all {
      returnPublicationStepConstraint(defaultCaps, validDisabled),
      not(returnPublicationStepConstraint(defaultCaps, badEnable)),
      not(returnPublicationStepConstraint(defaultCaps, badRuleTag)),
      not(returnPublicationStepConstraint(defaultCaps, badRootCount)),
    })
  }

  // 4. ValueRef active-prefix boundary refusals in both Constant and Data spaces.
  run activePrefixBoundaryRefusalTest = {
    val oobConstState = {
      ...defaultPreState,
      controlRows: List(returnControlRow0({ space: SPACE_CONSTANT, index: 1 }), zeroControlRow),
    }
    val oobDataState = {
      ...defaultPreState,
      root: { ...defaultPreState.root, activeData: 1 },
      controlRows: List(returnControlRow0({ space: SPACE_DATA, index: 1 }), zeroControlRow),
    }
    initWith(defaultCaps, defaultProgram, oobConstState)
      .then(step.fail())
      .then(initWith(defaultCaps, defaultProgram, oobDataState))
      .then(step.fail())
  }

  // 5. Stack depth, FrameSlots padding, and ControlRows spine/padding refusals.
  run nonEmptyStackAndControlSpineRefusalTest = {
    val depthOneState = {
      ...defaultPreState,
      root: { ...defaultPreState.root, depth: 1 },
    }
    val dirtyFrameState = {
      ...defaultPreState,
      frameSlots: List({ ...zeroFrameRow, tag: 8 }),
    }
    val dirtyControlColState = {
      ...defaultPreState,
      controlRows: List(
        { ...returnControlRow0({ space: SPACE_CONSTANT, index: 0 }), errorCode: 1 },
        zeroControlRow
      ),
    }
    val dirtyControlTailState = {
      ...defaultPreState,
      controlRows: List(
        returnControlRow0({ space: SPACE_CONSTANT, index: 0 }),
        { ...zeroControlRow, exprId: 0, primitiveIsOk: 1 }
      ),
    }
    initWith(defaultCaps, defaultProgram, depthOneState)
      .then(step.fail())
      .then(initWith(defaultCaps, defaultProgram, dirtyFrameState))
      .then(step.fail())
      .then(initWith(defaultCaps, defaultProgram, dirtyControlColState))
      .then(step.fail())
      .then(initWith(defaultCaps, defaultProgram, dirtyControlTailState))
      .then(step.fail())
  }

  // 6. Enabled post-state preservation/clearing and envelope tamper refusals.
  run enabledPostAndEnvelopeTamperRefusalTest = {
    val validPost = computePublishedPost(defaultPreState)
    val baseInput: ReturnPublicationStepInputs = {
      enable: 1,
      program: defaultProgram,
      pre: defaultPreState,
      post: validPost,
      envelope: { ruleTag: 0, charge: 0 },
    }
    val badCharge: ReturnPublicationStepInputs = {
      ...baseInput, envelope: { ruleTag: 0, charge: 1 },
    }
    val mutatedFuelPost: ReturnPublicationStepInputs = {
      ...baseInput,
      post: {
        ...validPost,
        root: { ...validPost.root, remainingFuel: List(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0) },
      },
    }
    val mutatedDataArenaPost: ReturnPublicationStepInputs = {
      ...baseInput,
      post: { ...validPost, data: List(zeroValueNode, zeroValueNode) },
    }
    val unclearedControlPost: ReturnPublicationStepInputs = {
      ...baseInput,
      post: { ...validPost, controlRows: defaultPreState.controlRows },
    }
    init.expect(all {
      returnPublicationStepConstraint(defaultCaps, baseInput),
      not(returnPublicationStepConstraint(defaultCaps, badCharge)),
      not(returnPublicationStepConstraint(defaultCaps, mutatedFuelPost)),
      not(returnPublicationStepConstraint(defaultCaps, mutatedDataArenaPost)),
      not(returnPublicationStepConstraint(defaultCaps, unclearedControlPost)),
    })
  }
}
