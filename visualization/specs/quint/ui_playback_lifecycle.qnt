// =============================================================================
// Formal Quint Specification: Sherlock Workbench UI, Playback, Camera & Static Host
// File: specs/quint/ui_playback_lifecycle.qnt
//
// Models:
// 1. Async `executeCurrentSource` / `loadProgram` trace generation with monotonic
//    request sequence tokens (`latestRequestId` vs `appliedRequestId`) under rapid
//    concurrent preset clicks (`factorial`, `list_fold`, `stack_overflow`, etc.).
// 2. Playback transport & timeline scrubbing state machine across edge-case step
//    counts (`totalSteps == 0`, `totalSteps == 1`, `totalSteps == 45`), preventing
//    `stepIndex < 0` or division-by-zero (`totalSteps - 1 == 0`) in progress math.
// 3. Designer 7 (`designer7_cinema.js`) pointer drag / pan / capture state machine
//    across `pointerdown`, `pointermove`, `pointerup`, `pointercancel`,
//    `lostpointercapture`, and `window` `blur`.
// 4. Static GitHub Pages subpath (`/Moriarty/visualization/index.html`) vs local
//    native `sherlock-runner` server (`/api/trace`) trace resolution.
// =============================================================================

module ui_playback_lifecycle {

  type AsyncRequest = {
    reqId: int,
    presetId: int,
    stepCount: int,
    usedNetworkFetch: bool,
  }

  type UIState = {
    // Host environment: true on GitHub Pages / static subpath / preview server
    isStaticHost: bool,
    rustBackendAvailable: bool,
    networkFetchCount: int,

    // Active program & monotonic async request tracking
    activePresetId: int,
    latestRequestId: int,
    appliedRequestId: int,
    tracePresetId: int,

    // Playback & scrubber state
    totalSteps: int,
    stepIndex: int,
    playing: bool,
    progressPct: int,

    // DOM cache coherence tracking
    domRenderedPresetId: int,
    domRenderedStepIdx: int,
    qdlTelemetryStepIdx: int,

    // Designer 7 camera pointer / drag / raycast state machine
    isDragging: bool,
    isPanning: bool,
    pointerCaptured: bool,
    activePointerId: int,
    lastEndedByCancelOrBlur: bool,
  }

  var ui: UIState
  var inFlight: Set[AsyncRequest]

  pure val PRESET_IDS: Set[int] = Set(1, 2, 3, 4)

  // Step counts include edge cases: 0 steps, 1 step, 14 steps (fuel limit), 45 steps (fact(4))
  pure def presetStepCount(presetId: int): int =
    if (presetId == 1) 45
    else if (presetId == 2) 14
    else if (presetId == 3) 1
    else 0

  pure def clampStepIndex(idx: int, totalSteps: int): int =
    if (totalSteps <= 0) 0
    else if (idx < 0) 0
    else if (idx >= totalSteps) totalSteps - 1
    else idx

  // Safe progress calculation guarding against division by (totalSteps - 1) when totalSteps <= 1
  pure def computeProgressPct(stepIdx: int, totalSteps: int): int =
    if (totalSteps <= 1) 100
    else (clampStepIndex(stepIdx, totalSteps) * 100) / (totalSteps - 1)

  // Pure helper demonstrating the UNGUARDED bug in src/app.js:395-423 where an older
  // in-flight response overwrites a newer already-applied trace.
  pure def applyResolveUnguardedBuggy(s: UIState, req: AsyncRequest): UIState = {
    val nextTotal = req.stepCount
    val nextIdx = clampStepIndex(0, nextTotal)
    {
      ...s,
      appliedRequestId: req.reqId,
      tracePresetId: req.presetId,
      totalSteps: nextTotal,
      stepIndex: nextIdx,
      progressPct: computeProgressPct(nextIdx, nextTotal),
      domRenderedPresetId: req.presetId,
      domRenderedStepIdx: nextIdx,
      qdlTelemetryStepIdx: nextIdx,
    }
  }

  // Pure helper for the GUARDED resolution (ignores stale responses where req.reqId < latestRequestId)
  pure def applyResolveGuarded(s: UIState, req: AsyncRequest): UIState =
    if (req.reqId < s.latestRequestId)
      s
    else {
      val nextTotal = req.stepCount
      val nextIdx = clampStepIndex(0, nextTotal)
      {
        ...s,
        appliedRequestId: req.reqId,
        tracePresetId: req.presetId,
        totalSteps: nextTotal,
        stepIndex: nextIdx,
        progressPct: computeProgressPct(nextIdx, nextTotal),
        domRenderedPresetId: req.presetId,
        domRenderedStepIdx: nextIdx,
        qdlTelemetryStepIdx: nextIdx,
      }
    }

  action init: bool = all {
    ui' = {
      isStaticHost: true,
      rustBackendAvailable: false,
      networkFetchCount: 0,
      activePresetId: 1,
      latestRequestId: 1,
      appliedRequestId: 1,
      tracePresetId: 1,
      totalSteps: 45,
      stepIndex: 0,
      playing: true,
      progressPct: 0,
      domRenderedPresetId: 1,
      domRenderedStepIdx: 0,
      qdlTelemetryStepIdx: 0,
      isDragging: false,
      isPanning: false,
      pointerCaptured: false,
      activePointerId: -1,
      lastEndedByCancelOrBlur: false,
    },
    inFlight' = Set(),
  }

  // Configure host environment (Static GitHub Pages / subpath vs Local Rust Server)
  action configureHostMode(staticHost: bool): bool = all {
    inFlight.size() == 0,
    ui' = {
      ...ui,
      isStaticHost: staticHost,
      rustBackendAvailable: not(staticHost),
      networkFetchCount: 0,
    },
    inFlight' = inFlight,
  }

  // User clicks a preset button or triggers "Run program"
  action requestLoadPreset(presetId: int): bool = {
    val nextReqId = ui.latestRequestId + 1
    val stepCount = presetStepCount(presetId)
    val willFetch = not(ui.isStaticHost) and ui.rustBackendAvailable
    all {
      PRESET_IDS.contains(presetId),
      inFlight.size() < 4,
      ui' = {
        ...ui,
        activePresetId: presetId,
        latestRequestId: nextReqId,
        networkFetchCount: if (willFetch) ui.networkFetchCount + 1 else ui.networkFetchCount,
      },
      inFlight' = inFlight.union(Set({
        reqId: nextReqId,
        presetId: presetId,
        stepCount: stepCount,
        usedNetworkFetch: willFetch,
      })),
    }
  }

  // Async trace request resolves (may arrive out-of-order relative to concurrent requests)
  action resolveTraceRequest(req: AsyncRequest): bool = all {
    inFlight.contains(req),
    ui' = applyResolveGuarded(ui, req),
    inFlight' = inFlight.exclude(Set(req)),
  }

  // Server returns HTTP 404/405/501 on a non-Rust server -> disables further fetches and falls back to browser engine
  action fallbackOnServerHttpError(req: AsyncRequest): bool = all {
    inFlight.contains(req),
    req.usedNetworkFetch,
    ui' = {
      ...applyResolveGuarded(ui, req),
      rustBackendAvailable: false,
    },
    inFlight' = inFlight.exclude(Set(req)),
  }

  // Transport: Play / Pause toggle
  action togglePlayPause: bool = {
    val nextPlaying = not(ui.playing)
    val shouldWrap = nextPlaying and ui.totalSteps > 0 and ui.stepIndex >= ui.totalSteps - 1
    val nextIdx = if (shouldWrap) 0 else ui.stepIndex
    all {
      ui' = {
        ...ui,
        playing: nextPlaying,
        stepIndex: nextIdx,
        progressPct: computeProgressPct(nextIdx, ui.totalSteps),
        domRenderedStepIdx: nextIdx,
        qdlTelemetryStepIdx: nextIdx,
      },
      inFlight' = inFlight,
    }
  }

  // Transport: Step Previous (`stepPrevBtn`)
  action stepPrev: bool = {
    val nextIdx = clampStepIndex(ui.stepIndex - 1, ui.totalSteps)
    all {
      ui' = {
        ...ui,
        playing: false,
        stepIndex: nextIdx,
        progressPct: computeProgressPct(nextIdx, ui.totalSteps),
        domRenderedStepIdx: nextIdx,
        qdlTelemetryStepIdx: nextIdx,
      },
      inFlight' = inFlight,
    }
  }

  // Transport: Step Next (`stepNextBtn`) — guarded with clampStepIndex so totalSteps == 0 never yields -1
  action stepNext: bool = {
    val nextIdx = clampStepIndex(ui.stepIndex + 1, ui.totalSteps)
    all {
      ui' = {
        ...ui,
        playing: false,
        stepIndex: nextIdx,
        progressPct: computeProgressPct(nextIdx, ui.totalSteps),
        domRenderedStepIdx: nextIdx,
        qdlTelemetryStepIdx: nextIdx,
      },
      inFlight' = inFlight,
    }
  }

  // Timeline scrubber input (`stepScrubber`)
  action scrubToStep(targetIdx: int): bool = {
    val nextIdx = clampStepIndex(targetIdx, ui.totalSteps)
    all {
      targetIdx >= -1 and targetIdx <= 50,
      ui' = {
        ...ui,
        playing: false,
        stepIndex: nextIdx,
        progressPct: computeProgressPct(nextIdx, ui.totalSteps),
        domRenderedStepIdx: nextIdx,
        qdlTelemetryStepIdx: nextIdx,
      },
      inFlight' = inFlight,
    }
  }

  // Animation frame tick when playing
  action playbackTick: bool = {
    val nextRaw = if (ui.stepIndex + 1 >= ui.totalSteps) 0 else ui.stepIndex + 1
    val nextIdx = clampStepIndex(nextRaw, ui.totalSteps)
    all {
      ui.playing,
      ui.totalSteps > 0,
      ui' = {
        ...ui,
        stepIndex: nextIdx,
        progressPct: computeProgressPct(nextIdx, ui.totalSteps),
        domRenderedStepIdx: nextIdx,
        qdlTelemetryStepIdx: nextIdx,
      },
      inFlight' = inFlight,
    }
  }

  // Designer 7 Camera: pointerdown
  action pointerDown(pointerId: int, panMode: bool): bool = all {
    not(ui.isDragging),
    pointerId >= 1,
    ui' = {
      ...ui,
      isDragging: true,
      isPanning: panMode,
      pointerCaptured: true,
      activePointerId: pointerId,
      lastEndedByCancelOrBlur: false,
    },
    inFlight' = inFlight,
  }

  // Designer 7 Camera: pointermove with active buttons
  action pointerMoveActive(pointerId: int): bool = all {
    ui.isDragging,
    ui.activePointerId == pointerId,
    ui' = ui,
    inFlight' = inFlight,
  }

  // Designer 7 Camera: pointerup releases drag & capture cleanly
  action pointerUp(pointerId: int): bool = all {
    ui.isDragging,
    ui.activePointerId == pointerId,
    ui' = {
      ...ui,
      isDragging: false,
      isPanning: false,
      pointerCaptured: false,
      activePointerId: -1,
      lastEndedByCancelOrBlur: false,
    },
    inFlight' = inFlight,
  }

  // Designer 7 Camera: pointercancel / lostpointercapture / window blur must release drag & capture
  action pointerCancelOrBlur: bool = all {
    ui.isDragging,
    ui' = {
      ...ui,
      isDragging: false,
      isPanning: false,
      pointerCaptured: false,
      activePointerId: -1,
      lastEndedByCancelOrBlur: true,
    },
    inFlight' = inFlight,
  }

  action step: bool = {
    nondet preset = PRESET_IDS.oneOf()
    nondet scrubTarget = Set(0, 1, 13, 44).oneOf()
    nondet ptrId = Set(1, 2).oneOf()
    nondet pan = Set(true, false).oneOf()
    nondet staticMode = Set(true, false).oneOf()
    any {
      configureHostMode(staticMode),
      requestLoadPreset(preset),
      all {
        inFlight.size() > 0,
        nondet req = inFlight.oneOf()
        any {
          resolveTraceRequest(req),
          fallbackOnServerHttpError(req),
        }
      },
      togglePlayPause,
      stepPrev,
      stepNext,
      scrubToStep(scrubTarget),
      playbackTick,
      pointerDown(ptrId, pan),
      pointerMoveActive(ptrId),
      pointerUp(ptrId),
      pointerCancelOrBlur,
    }
  }

  // ===========================================================================
  // Formal Safety Invariants
  // ===========================================================================

  // 1. No older in-flight async trace response can ever overwrite a newer applied request,
  //    and once all in-flight requests drain, the rendered trace matches the active preset.
  val noStaleAsyncTraceOverwrite: bool = and {
    ui.appliedRequestId <= ui.latestRequestId,
    ui.domRenderedPresetId == ui.tracePresetId,
    ui.qdlTelemetryStepIdx == ui.stepIndex,
    (inFlight.size() == 0) implies (
      ui.appliedRequestId == ui.latestRequestId and
      ui.tracePresetId == ui.activePresetId
    ),
  }

  // 2. Step index and progress percentage are always in bounds across 0-step, 1-step, and N-step traces.
  val stepIndexAlwaysInBounds: bool = and {
    ui.stepIndex >= 0,
    (ui.totalSteps == 0) implies (ui.stepIndex == 0),
    (ui.totalSteps > 0) implies (ui.stepIndex < ui.totalSteps),
    ui.progressPct >= 0 and ui.progressPct <= 100,
    ui.domRenderedStepIdx == ui.stepIndex,
  }

  // 3. Pointer drag and pointer capture are never left stuck true after pointercancel / window blur.
  val pointerDragNeverStuckAfterCancel: bool = and {
    ui.lastEndedByCancelOrBlur implies (
      not(ui.isDragging) and
      not(ui.isPanning) and
      not(ui.pointerCaptured) and
      ui.activePointerId == -1
    ),
    ui.isDragging iff (ui.pointerCaptured and ui.activePointerId > 0),
  }

  // 4. Static hosts (GitHub Pages / subpath / static preview) perform zero network fetches to `/api/trace`.
  val staticHostZeroNetworkFetch: bool =
    ui.isStaticHost implies (
      not(ui.rustBackendAvailable) and
      ui.networkFetchCount == 0 and
      inFlight.forall(r => not(r.usedNetworkFetch))
    )

  // Combined safety invariant
  val allUILifecycleInvariants: bool = and {
    noStaleAsyncTraceOverwrite,
    stepIndexAlwaysInBounds,
    pointerDragNeverStuckAfterCancel,
    staticHostZeroNetworkFetch,
  }

  // ===========================================================================
  // Concrete Scenario & Counterexample Verification Tests (`quint test`)
  // ===========================================================================

  // Test 1: Out-of-order async preset responses (Preset 1 requested, then Preset 2 requested;
  // Preset 2 resolves first, then stale Preset 1 arrives late).
  // Also demonstrates that `applyResolveUnguardedBuggy` (current src/app.js) corrupts state.
  run outOfOrderAsyncPresetResolutionTest = {
    val req1 = { reqId: 2, presetId: 2, stepCount: 14, usedNetworkFetch: false }
    val req2 = { reqId: 3, presetId: 1, stepCount: 45, usedNetworkFetch: false }
    init
      .then(requestLoadPreset(2))
      .then(requestLoadPreset(1))
      .expect(ui.latestRequestId == 3 and inFlight.size() == 2)
      .then(resolveTraceRequest(req2))
      .expect(ui.appliedRequestId == 3 and ui.tracePresetId == 1 and ui.totalSteps == 45)
      // Verify that the unguarded implementation in src/app.js WOULD overwrite reqId 3 with stale reqId 2:
      .expect(applyResolveUnguardedBuggy(ui, req1).tracePresetId == 2)
      // Verify that the guarded state machine ignores stale req1 and preserves req2:
      .then(resolveTraceRequest(req1))
      .expect(all {
        ui.appliedRequestId == 3,
        ui.tracePresetId == 1,
        ui.activePresetId == 1,
        ui.totalSteps == 45,
        inFlight.size() == 0,
        noStaleAsyncTraceOverwrite,
      })
  }

  // Test 2: 0-step and 1-step traces never produce negative stepIndex (`-1`) or out-of-bounds progress
  run zeroAndSingleStepScrubBoundsTest = {
    val reqSingle = { reqId: 2, presetId: 3, stepCount: 1, usedNetworkFetch: false }
    val reqZero = { reqId: 3, presetId: 4, stepCount: 0, usedNetworkFetch: false }
    init
      .then(requestLoadPreset(3))
      .then(resolveTraceRequest(reqSingle))
      .expect(ui.totalSteps == 1 and ui.stepIndex == 0 and ui.progressPct == 100)
      .then(stepNext)
      .expect(ui.stepIndex == 0 and stepIndexAlwaysInBounds)
      .then(stepPrev)
      .expect(ui.stepIndex == 0 and stepIndexAlwaysInBounds)
      .then(requestLoadPreset(4))
      .then(resolveTraceRequest(reqZero))
      .expect(ui.totalSteps == 0 and ui.stepIndex == 0 and ui.progressPct == 100)
      .then(stepNext)
      .expect(ui.stepIndex == 0 and stepIndexAlwaysInBounds)
      .then(scrubToStep(10))
      .expect(ui.stepIndex == 0 and stepIndexAlwaysInBounds)
  }

  // Test 3: Pointer drag + pointercancel / window blur never leaves `isDragging` stuck true
  run pointerCancelAndWindowBlurReleaseDragTest =
    init
      .then(pointerDown(1, false))
      .expect(ui.isDragging and ui.pointerCaptured and ui.activePointerId == 1)
      .then(pointerMoveActive(1))
      .then(pointerCancelOrBlur)
      .expect(all {
        not(ui.isDragging),
        not(ui.isPanning),
        not(ui.pointerCaptured),
        ui.activePointerId == -1,
        ui.lastEndedByCancelOrBlur,
        pointerDragNeverStuckAfterCancel,
      })
      .then(pointerDown(2, true))
      .expect(ui.isDragging and ui.isPanning and not(ui.lastEndedByCancelOrBlur))
      .then(pointerUp(2))
      .expect(not(ui.isDragging) and pointerDragNeverStuckAfterCancel)

  // Test 4: Static GitHub Pages host makes zero network fetches while local server falls back on HTTP error
  run staticGitHubPagesSubpathZeroFetchTest = {
    val staticReq = { reqId: 2, presetId: 2, stepCount: 14, usedNetworkFetch: false }
    val localReq1 = { reqId: 3, presetId: 1, stepCount: 45, usedNetworkFetch: true }
    val localReq2 = { reqId: 4, presetId: 2, stepCount: 14, usedNetworkFetch: false }
    init
      .expect(ui.isStaticHost and ui.networkFetchCount == 0)
      .then(requestLoadPreset(2))
      .then(resolveTraceRequest(staticReq))
      .expect(staticHostZeroNetworkFetch and ui.networkFetchCount == 0)
      .then(configureHostMode(false))
      .expect(not(ui.isStaticHost) and ui.rustBackendAvailable)
      .then(requestLoadPreset(1))
      .expect(ui.networkFetchCount == 1)
      .then(fallbackOnServerHttpError(localReq1))
      .expect(not(ui.rustBackendAvailable) and ui.tracePresetId == 1)
      .then(requestLoadPreset(2))
      .then(resolveTraceRequest(localReq2))
      .expect(ui.networkFetchCount == 1 and ui.tracePresetId == 2)
  }
}
