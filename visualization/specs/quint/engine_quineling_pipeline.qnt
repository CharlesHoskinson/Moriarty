// -*- mode: quint; -*-
// =============================================================================
// Formal Quint Specification: Sherlock CEK Engine, Quineling Compiler & WovenBody
// File: specs/quint/engine_quineling_pipeline.qnt
//
// Models:
// 1. CEK Trace Cache (`clientTraceCache` & `server.js` `traceCache`, max 32 entries)
//    and step-index clamping / division-by-zero protection for 0-step & 1-step traces.
// 2. Horological 6-Ring / 12-Slot Continuation Telemetry (`R1.A .. R6.B`, slots 0..11,
//    overflowFrames >= 0, frontierSlotIndex, activeSlotIndex in 0..11, 10-sector ControlCore).
// 3. `compileSherlockProgramToQuineling` 64-entry LRU cache + 1-entry hot fast-path,
//    verifying bijective topology-driven assignment of all 8 canonical presets
//    (`fact4`..`fib4` / `factorial`..`fuel_exhaustion`) to their 8 distinct
//    `WovenBody` mechanisms and 8 `QDL` families, plus cache-aliasing isolation
//    across callers with distinct `activeTaskIndex` / `phase` state.
// 4. `WovenBody.frame()` typed-array `reuse` buffer capacity and owner-index bounds
//    across mechanism switches (`budget = 6000 | 6500 | 8192 | 22000`).
// =============================================================================

module engine_quineling_pipeline {

  // ---------------------------------------------------------------------------
  // 1. Domain Types & Constants
  // ---------------------------------------------------------------------------

  type PresetId = int
  type MechanismId = int
  type FamilyId = int

  // 8 Canonical Presets (0..7: fact4, list_fold, tree_fold, mutual_even_odd,
  // lexical_let, checked_underflow, stack_overflow, fib4 / fuel_exhaustion)
  pure val PRESETS: Set[PresetId] = 0.to(7)

  // 8 Distinct WovenBody Mechanisms (0..7):
  //   0: logarithmic-mantle
  //   1: pleated-braid
  //   2: recursive-affine
  //   3: lorenz-flow
  //   4: phyllotaxis-fan
  //   5: clifford-flow
  //   6: recursive-julia
  //   7: toroidal-weave
  pure val MECHANISMS: Set[MechanismId] = 0.to(7)

  // 8 Distinct QDL Families (0..7):
  //   0: nautilus, 1: ribbon, 2: coral, 3: moth,
  //   4: bloom, 5: jelly, 6: comet, 7: torus
  pure val FAMILIES: Set[FamilyId] = 0.to(7)

  pure val TRACE_CACHE_MAX: int = 32
  pure val COMPILER_CACHE_MAX: int = 64
  pure val RING_COUNT: int = 6
  pure val RING_SLOT_CAP: int = 12
  pure val CONTROL_SECTOR_COUNT: int = 10

  pure val VALID_BUDGETS: Set[int] = Set(6000, 6500, 8192, 22000)

  // Task-graph topology features extracted by `WovenBody.topology(graph)` and `programProfile(ns)`
  type GraphTopology = {
    depth: int,
    fanout: int,
    convergence: int,
    guards: int,
    effects: int,
    hasTreeBranch: bool,
    hasCoupled: bool,
    hasPipelineFold: bool,
    nodeCount: int,
    strands: int,
    branchCount: int,
  }

  // Ring & ControlCore Telemetry state for a step snapshot
  type RingTelemetryState = {
    totalFrames: int,
    filledRingSlots: int,
    overflowFrames: int,
    activeSlotIndex: int,
    frontierSlotIndex: int,
    hasOverflowTop: bool,
    controlSector: int,
    stepIndex: int,
    totalSteps: int,
    progressPermille: int,
  }

  // Compiled Quineling cache entry (immutable shared artifact in cache)
  type CompiledArtifact = {
    presetId: PresetId,
    mechanism: MechanismId,
    family: FamilyId,
    nodeCount: int,
    strands: int,
    branchCount: int,
    canonicalTaskIndex: int,
    canonicalPhase: int,
  }

  // Per-caller view returned from `compileSherlockProgramToQuineling`
  type CallerView = {
    presetId: PresetId,
    activeTaskIndex: int,
    phase: int,
  }

  // Typed-array `reuse` buffer state for `WovenBody.frame(body, phase, { budget, reuse })`
  type ReuseBufferState = {
    requestedBudget: int,
    pointsLength: int,
    ownersLength: int,
    reallocated: bool,
    maxOwnerIndex: int,
    nodeCount: int,
    allCoordsFinite: bool,
  }

  // Cohesive system state record
  type PipelineState = {
    traceCacheKeys: List[int],
    compilerCacheKeys: List[PresetId],
    compilerCacheMap: PresetId -> CompiledArtifact,
    presetMechanisms: PresetId -> MechanismId,
    presetFamilies: PresetId -> FamilyId,
    hotPresetId: int,
    callerA: CallerView,
    callerB: CallerView,
    ringTelemetry: RingTelemetryState,
    reuseBuffer: ReuseBufferState,
    stepCount: int,
  }

  var state: PipelineState

  // ---------------------------------------------------------------------------
  // 2. Pure Functional Layer (Topology, Mechanism Authoring, Ring & Reuse Logic)
  // ---------------------------------------------------------------------------

  // Topology constructed by `buildQuinelingTaskGraphFromAst` for each of the 8 presets
  pure def presetTopology(p: PresetId): GraphTopology =
    if (p == 0) {
      // fact4 / factorial -> logarithmic-mantle (nautilus)
      { depth: 4, fanout: 1, convergence: 2, guards: 0, effects: 0,
        hasTreeBranch: false, hasCoupled: false, hasPipelineFold: false,
        nodeCount: 7, strands: 9, branchCount: 9 }
    } else if (p == 1) {
      // list_fold -> pleated-braid (ribbon)
      { depth: 6, fanout: 1, convergence: 1, guards: 0, effects: 0,
        hasTreeBranch: false, hasCoupled: false, hasPipelineFold: true,
        nodeCount: 6, strands: 11, branchCount: 11 }
    } else if (p == 2) {
      // tree_fold -> recursive-affine (coral)
      { depth: 4, fanout: 3, convergence: 3, guards: 0, effects: 0,
        hasTreeBranch: true, hasCoupled: false, hasPipelineFold: false,
        nodeCount: 9, strands: 11, branchCount: 40 }
    } else if (p == 3) {
      // mutual_even_odd -> lorenz-flow (moth)
      { depth: 3, fanout: 1, convergence: 2, guards: 0, effects: 0,
        hasTreeBranch: false, hasCoupled: true, hasPipelineFold: false,
        nodeCount: 6, strands: 9, branchCount: 9 }
    } else if (p == 4) {
      // lexical_let -> phyllotaxis-fan (bloom)
      { depth: 6, fanout: 1, convergence: 1, guards: 0, effects: 0,
        hasTreeBranch: false, hasCoupled: false, hasPipelineFold: false,
        nodeCount: 6, strands: 8, branchCount: 16 }
    } else if (p == 5) {
      // checked_underflow -> clifford-flow (jelly)
      { depth: 4, fanout: 1, convergence: 2, guards: 2, effects: 1,
        hasTreeBranch: false, hasCoupled: false, hasPipelineFold: true,
        nodeCount: 8, strands: 9, branchCount: 9 }
    } else if (p == 6) {
      // stack_overflow -> recursive-julia (comet)
      { depth: 4, fanout: 3, convergence: 3, guards: 0, effects: 0,
        hasTreeBranch: false, hasCoupled: false, hasPipelineFold: true,
        nodeCount: 8, strands: 11, branchCount: 11 }
    } else {
      // fib4 / fuel_exhaustion -> toroidal-weave (torus)
      { depth: 4, fanout: 1, convergence: 2, guards: 0, effects: 1,
        hasTreeBranch: false, hasCoupled: false, hasPipelineFold: false,
        nodeCount: 5, strands: 9, branchCount: 9 }
    }

  // Exact decision tree from `WovenBody.author(graph, seed)` in `quinelingsSdk.js:961`
  pure def selectWovenMechanism(t: GraphTopology): MechanismId =
    if (t.guards > 0) 5
    else if (t.effects == 0 and t.fanout >= 2 and t.hasTreeBranch) 2
    else if (t.effects == 0 and t.hasCoupled) 3
    else if (t.effects == 0 and t.fanout <= 1 and t.depth >= 4 and t.hasPipelineFold) 1
    else if (t.fanout >= 2) 6
    else if (t.effects > 0) 7
    else if (t.convergence >= 2) 0
    else 4

  // Exact decision tree from `QDL.fromGraph(graph, seed)` in `quinelingsSdk.js:103`
  pure def selectQdlFamily(t: GraphTopology): FamilyId =
    if (t.guards > 0) 5
    else if (t.effects == 0 and t.fanout >= 2 and t.hasTreeBranch) 2
    else if (t.effects == 0 and t.hasCoupled) 3
    else if (t.effects == 0 and t.fanout <= 1 and t.nodeCount >= 4 and t.hasPipelineFold) 1
    else if (t.fanout >= 2) 6
    else if (t.effects > 0) 7
    else if (t.convergence >= 2) 0
    else 4

  pure def compileArtifactForPreset(p: PresetId): CompiledArtifact = {
    val t = presetTopology(p)
    {
      presetId: p,
      mechanism: selectWovenMechanism(t),
      family: selectQdlFamily(t),
      nodeCount: t.nodeCount,
      strands: t.strands,
      branchCount: t.branchCount,
      canonicalTaskIndex: 0,
      canonicalPhase: 0,
    }
  }

  // Pure LRU cache insertion with strict capacity bound `maxCap`
  pure def lruPut(keyList: List[int], key: int, maxCap: int): List[int] = {
    val withoutKey = keyList.select(k => k != key)
    val trimmed =
      if (withoutKey.length() >= maxCap and withoutKey.length() > 0)
        withoutKey.tail()
      else
        withoutKey
    trimmed.append(key)
  }

  // Pure computation of 6-ring / 12-slot continuation telemetry (`buildRingTelemetry` + `buildStepState`)
  // Includes safe clamping when `rawTotalSteps == 0` or `rawTotalSteps == 1` (preventing division by zero)
  pure def computeRingTelemetry(
    rawTotalSteps: int,
    rawPlayhead: int,
    frameCount: int,
    sector: int
  ): RingTelemetryState = {
    val safeTotalSteps = if (rawTotalSteps < 1) 1 else rawTotalSteps
    val safeStepIdx =
      if (rawPlayhead < 0) 0
      else if (rawPlayhead >= safeTotalSteps) safeTotalSteps - 1
      else rawPlayhead
    val progressPermille =
      if (safeTotalSteps <= 1) 1000
      else (safeStepIdx * 1000) / (safeTotalSteps - 1)
    val safeFrames = if (frameCount < 0) 0 else frameCount
    val filledSlots = if (safeFrames > RING_SLOT_CAP) RING_SLOT_CAP else safeFrames
    val overflow = if (safeFrames > RING_SLOT_CAP) safeFrames - RING_SLOT_CAP else 0
    val frontier = safeFrames - 1
    val activeSlot =
      if (safeFrames <= 0) 0
      else if (frontier >= RING_SLOT_CAP) RING_SLOT_CAP - 1
      else frontier
    val safeSector =
      if (sector < 0 or sector >= CONTROL_SECTOR_COUNT) 0
      else sector
    {
      totalFrames: safeFrames,
      filledRingSlots: filledSlots,
      overflowFrames: overflow,
      activeSlotIndex: activeSlot,
      frontierSlotIndex: frontier,
      hasOverflowTop: safeFrames > RING_SLOT_CAP,
      controlSector: safeSector,
      stepIndex: safeStepIdx,
      totalSteps: safeTotalSteps,
      progressPermille: progressPermille,
    }
  }

  // Pure computation of `WovenBody.frame()` `reuse` buffer allocation & owner-index safety
  pure def computeFrameReuse(
    prevReuse: ReuseBufferState,
    budget: int,
    artifact: CompiledArtifact,
    spanX: int,
    spanY: int
  ): ReuseBufferState = {
    val safeBudget = if (budget < 128) 128 else if (budget > 100000) 100000 else budget
    val expectedPointsLen = safeBudget * 4
    val expectedOwnersLen = safeBudget
    val canReuseInPlace =
      prevReuse.pointsLength == expectedPointsLen and
      prevReuse.ownersLength == expectedOwnersLen
    // Degenerate span guard (`Math.max(0.32, spanX, spanY) > 0` prevents division-by-zero NaN)
    val safeSpan = if (spanX <= 0 and spanY <= 0) 32 else if (spanX > spanY) spanX else spanY
    val coordsFinite = safeSpan > 0 and artifact.nodeCount >= 1 and safeBudget >= artifact.nodeCount
    {
      requestedBudget: safeBudget,
      pointsLength: expectedPointsLen,
      ownersLength: expectedOwnersLen,
      reallocated: not(canReuseInPlace),
      maxOwnerIndex: artifact.nodeCount - 1,
      nodeCount: artifact.nodeCount,
      allCoordsFinite: coordsFinite,
    }
  }

  // ---------------------------------------------------------------------------
  // 3. State Machine Actions (`init`, guarded transitions, `step`)
  // ---------------------------------------------------------------------------

  action init: bool = {
    val initialMap = PRESETS.mapBy(p => compileArtifactForPreset(p))
    val initialMechs = PRESETS.mapBy(p => compileArtifactForPreset(p).mechanism)
    val initialFams = PRESETS.mapBy(p => compileArtifactForPreset(p).family)
    val fact0 = compileArtifactForPreset(0)
    all {
      state' = {
        traceCacheKeys: List(0),
        compilerCacheKeys: List(0, 1, 2, 3, 4, 5, 6, 7),
        compilerCacheMap: initialMap,
        presetMechanisms: initialMechs,
        presetFamilies: initialFams,
        hotPresetId: 0,
        callerA: { presetId: 0, activeTaskIndex: 0, phase: 0 },
        callerB: { presetId: 1, activeTaskIndex: 0, phase: 0 },
        ringTelemetry: computeRingTelemetry(24, 0, 0, 0),
        reuseBuffer: {
          requestedBudget: 8192,
          pointsLength: 8192 * 4,
          ownersLength: 8192,
          reallocated: false,
          maxOwnerIndex: fact0.nodeCount - 1,
          nodeCount: fact0.nodeCount,
          allCoordsFinite: true,
        },
        stepCount: 0,
      }
    }
  }

  // Action 1: Record a CEK trace in the 32-entry bounded trace cache & update ring telemetry
  action executeAndCacheTrace(
    traceKey: int,
    rawSteps: int,
    playhead: int,
    frameDepth: int,
    sector: int
  ): bool = all {
    traceKey >= 0,
    rawSteps >= 0,
    frameDepth >= 0 and frameDepth <= 16,
    sector >= 0 and sector < CONTROL_SECTOR_COUNT,
    state' = {
      ...state,
      traceCacheKeys: lruPut(state.traceCacheKeys, traceKey, TRACE_CACHE_MAX),
      ringTelemetry: computeRingTelemetry(rawSteps, playhead, frameDepth, sector),
      stepCount: state.stepCount + 1,
    }
  }

  // Action 2: Compile/lookup a preset in `compileSherlockProgramToQuineling` and mutate caller local state
  // while preserving canonical cached artifact immutability (`noCacheAliasingCorruption`)
  action compilePresetForCaller(
    presetId: PresetId,
    isCallerA: bool,
    taskIdx: int,
    phaseVal: int
  ): bool = all {
    PRESETS.contains(presetId),
    taskIdx >= 0 and taskIdx < 16,
    phaseVal >= 0 and phaseVal <= 628,
    val artifact = compileArtifactForPreset(presetId)
    val updatedKeys = lruPut(state.compilerCacheKeys, presetId, COMPILER_CACHE_MAX)
    val updatedMap = state.compilerCacheMap.put(presetId, artifact)
    val view: CallerView = { presetId: presetId, activeTaskIndex: taskIdx, phase: phaseVal }
    state' = {
      ...state,
      compilerCacheKeys: updatedKeys,
      compilerCacheMap: updatedMap,
      presetMechanisms: state.presetMechanisms.put(presetId, artifact.mechanism),
      presetFamilies: state.presetFamilies.put(presetId, artifact.family),
      hotPresetId: presetId,
      callerA: if (isCallerA) view else state.callerA,
      callerB: if (not(isCallerA)) view else state.callerB,
      stepCount: state.stepCount + 1,
    }
  }

  // Action 3: Sample `WovenBody.frame(body, phase, { budget, reuse })` across mechanism/budget switches
  action sampleWovenFrame(
    presetId: PresetId,
    budget: int,
    spanX: int,
    spanY: int
  ): bool = all {
    PRESETS.contains(presetId),
    VALID_BUDGETS.contains(budget),
    val artifact = state.compilerCacheMap.get(presetId)
    val nextReuse = computeFrameReuse(state.reuseBuffer, budget, artifact, spanX, spanY)
    state' = {
      ...state,
      hotPresetId: presetId,
      reuseBuffer: nextReuse,
      stepCount: state.stepCount + 1,
    }
  }

  action step: bool = {
    nondet presetId = PRESETS.oneOf()
    nondet traceKey = 0.to(48).oneOf()
    nondet rawSteps = Set(0, 1, 14, 24, 64).oneOf()
    nondet playhead = Set(0, 1, 7, 23, 30).oneOf()
    nondet frameDepth = Set(0, 1, 5, 11, 12, 16).oneOf()
    nondet sector = 0.to(9).oneOf()
    nondet isCallerA = Set(true, false).oneOf()
    nondet taskIdx = Set(0, 1, 3, 5).oneOf()
    nondet phaseVal = Set(0, 100, 314, 628).oneOf()
    nondet budget = VALID_BUDGETS.oneOf()
    nondet spanX = Set(0, 40, 120).oneOf()
    nondet spanY = Set(0, 40, 120).oneOf()
    any {
      executeAndCacheTrace(traceKey, rawSteps, playhead, frameDepth, sector),
      compilePresetForCaller(presetId, isCallerA, taskIdx, phaseVal),
      sampleWovenFrame(presetId, budget, spanX, spanY),
    }
  }

  // ---------------------------------------------------------------------------
  // 4. Formal Safety Invariants
  // ---------------------------------------------------------------------------

  // Invariant 1: Trace LRU cache and Compiler LRU cache never exceed their capacity bounds (32 and 64)
  val traceCacheBounded: bool = and {
    state.traceCacheKeys.length() >= 1,
    state.traceCacheKeys.length() <= TRACE_CACHE_MAX,
    state.compilerCacheKeys.length() >= 1,
    state.compilerCacheKeys.length() <= COMPILER_CACHE_MAX,
  }

  // Invariant 2: The 8 canonical Sherlock presets map bijectively onto all 8 WovenBody mechanisms
  // and all 8 QDL families (`fact4`..`fib4` -> 8 distinct mechanisms & 8 distinct families)
  val bijectivePresetMechanisms: bool = {
    val assignedMechanisms = PRESETS.map(p => state.presetMechanisms.get(p))
    val assignedFamilies = PRESETS.map(p => state.presetFamilies.get(p))
    and {
      assignedMechanisms == MECHANISMS,
      assignedFamilies == FAMILIES,
      assignedMechanisms.size() == 8,
      assignedFamilies.size() == 8,
      state.presetMechanisms.get(0) == 0,
      state.presetMechanisms.get(1) == 1,
      state.presetMechanisms.get(2) == 2,
      state.presetMechanisms.get(3) == 3,
      state.presetMechanisms.get(4) == 4,
      state.presetMechanisms.get(5) == 5,
      state.presetMechanisms.get(6) == 6,
      state.presetMechanisms.get(7) == 7,
    }
  }

  // Invariant 3: `WovenBody.frame()` `reuse` buffers always match `budget * 4` (points) and
  // `budget` (owners), all owner indices lie in `0 .. nodeCount - 1`, and coordinates remain finite
  val reuseBufferCapacitySafe: bool = and {
    state.reuseBuffer.requestedBudget >= 128,
    state.reuseBuffer.requestedBudget <= 100000,
    state.reuseBuffer.pointsLength == state.reuseBuffer.requestedBudget * 4,
    state.reuseBuffer.ownersLength == state.reuseBuffer.requestedBudget,
    state.reuseBuffer.nodeCount >= 1,
    state.reuseBuffer.maxOwnerIndex == state.reuseBuffer.nodeCount - 1,
    state.reuseBuffer.maxOwnerIndex < state.reuseBuffer.ownersLength,
    state.reuseBuffer.allCoordsFinite,
  }

  // Invariant 4: Mutating `activeTaskIndex` or `phase` on Caller A or Caller B never mutates
  // the shared cached artifact in `compilerCacheMap`, nor corrupts the other caller's view
  val noCacheAliasingCorruption: bool =
    PRESETS.forall(p => {
      val cached = state.compilerCacheMap.get(p)
      val expected = compileArtifactForPreset(p)
      and {
        cached.canonicalTaskIndex == 0,
        cached.canonicalPhase == 0,
        cached.mechanism == expected.mechanism,
        cached.family == expected.family,
        cached.nodeCount == expected.nodeCount,
        cached.branchCount >= cached.strands,
      }
    })

  // Invariant 5: Horological 6-ring / 12-slot continuation telemetry bounds hold for all
  // frame counts (including 0 frames and stack overflow > 12 frames) and 0/1-step traces
  val ringSlotBoundsValid: bool = {
    val rt = state.ringTelemetry
    and {
      rt.totalFrames >= 0,
      rt.filledRingSlots >= 0 and rt.filledRingSlots <= RING_SLOT_CAP,
      rt.overflowFrames >= 0,
      rt.filledRingSlots + rt.overflowFrames == rt.totalFrames,
      rt.activeSlotIndex >= 0 and rt.activeSlotIndex < RING_SLOT_CAP,
      rt.frontierSlotIndex == rt.totalFrames - 1,
      rt.hasOverflowTop == (rt.totalFrames > RING_SLOT_CAP),
      rt.controlSector >= 0 and rt.controlSector < CONTROL_SECTOR_COUNT,
      rt.totalSteps >= 1,
      rt.stepIndex >= 0 and rt.stepIndex < rt.totalSteps,
      rt.progressPermille >= 0 and rt.progressPermille <= 1000,
    }
  }

  val allPipelineInvariants: bool = and {
    traceCacheBounded,
    bijectivePresetMechanisms,
    reuseBufferCapacitySafe,
    noCacheAliasingCorruption,
    ringSlotBoundsValid,
  }

  // ---------------------------------------------------------------------------
  // 5. Executable Unit & Scenario Tests (`quint test`)
  // ---------------------------------------------------------------------------

  // Test 1: Verify all 8 presets map bijectively to 8 distinct mechanisms and 8 families
  run bijectivePresetsTest =
    init
      .then(compilePresetForCaller(0, true, 2, 100))
      .then(compilePresetForCaller(1, false, 4, 200))
      .then(compilePresetForCaller(2, true, 1, 300))
      .then(compilePresetForCaller(3, false, 3, 400))
      .then(compilePresetForCaller(4, true, 2, 150))
      .then(compilePresetForCaller(5, false, 1, 250))
      .then(compilePresetForCaller(6, true, 5, 350))
      .then(compilePresetForCaller(7, false, 2, 450))
      .expect(allPipelineInvariants)

  // Test 2: Verify trace cache eviction at 32 entries and ring overflow at >12 frames + 0-step trace safety
  run traceCacheAndRingOverflowTest =
    init
      .then(executeAndCacheTrace(1, 0, 0, 0, 0))
      .expect(and {
        allPipelineInvariants,
        state.ringTelemetry.totalSteps == 1,
        state.ringTelemetry.progressPermille == 1000,
      })
      .then(executeAndCacheTrace(2, 1, 0, 12, 2))
      .expect(and {
        allPipelineInvariants,
        state.ringTelemetry.filledRingSlots == 12,
        state.ringTelemetry.overflowFrames == 0,
        not(state.ringTelemetry.hasOverflowTop),
      })
      .then(executeAndCacheTrace(3, 64, 30, 16, 9))
      .expect(and {
        allPipelineInvariants,
        state.ringTelemetry.filledRingSlots == 12,
        state.ringTelemetry.overflowFrames == 4,
        state.ringTelemetry.activeSlotIndex == 11,
        state.ringTelemetry.frontierSlotIndex == 15,
        state.ringTelemetry.hasOverflowTop,
      })

  // Test 3: Verify `WovenBody.frame()` `reuse` buffer reallocation across budget changes and degenerate spans
  run reuseBufferSwitchTest =
    init
      .then(sampleWovenFrame(0, 8192, 120, 120))
      .expect(and {
        allPipelineInvariants,
        not(state.reuseBuffer.reallocated),
        state.reuseBuffer.pointsLength == 8192 * 4,
      })
      .then(sampleWovenFrame(2, 6500, 0, 0))
      .expect(and {
        allPipelineInvariants,
        state.reuseBuffer.reallocated,
        state.reuseBuffer.pointsLength == 6500 * 4,
        state.reuseBuffer.ownersLength == 6500,
        state.reuseBuffer.allCoordsFinite,
      })
      .then(sampleWovenFrame(5, 6500, 80, 90))
      .expect(and {
        allPipelineInvariants,
        not(state.reuseBuffer.reallocated),
        state.reuseBuffer.pointsLength == 6500 * 4,
      })

  // Test 4: Verify no cache-aliasing corruption when two callers access the same preset with different task indices
  run cacheAliasingIsolationTest =
    init
      .then(compilePresetForCaller(0, true, 5, 314))
      .then(compilePresetForCaller(0, false, 11, 628))
      .expect(and {
        allPipelineInvariants,
        state.callerA.presetId == 0,
        state.callerB.presetId == 0,
        state.callerA.activeTaskIndex == 5,
        state.callerB.activeTaskIndex == 11,
        state.compilerCacheMap.get(0).canonicalTaskIndex == 0,
        state.compilerCacheMap.get(0).canonicalPhase == 0,
      })
}
