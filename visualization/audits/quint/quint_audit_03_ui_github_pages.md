# Quint-LLM-Kit Formal Verification & Audit Report 03: App UI Controller, Playback State Machine, Camera Choreography & Static GitHub Pages Pipeline

- **Agent**: `Quint-UI-Playback-GitHubPages-Verifier` (Verification Agent 3)
- **Formal Specification**: `/home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt`
- **Audited Targets**:
  - `/home/charles/sherlock-bench/src/app.js`
  - `/home/charles/sherlock-bench/index.html` (and `/home/charles/repos/Moriarty/visualization/index.html`)
  - `/home/charles/sherlock-bench/src/styles.css`
  - `/home/charles/sherlock-bench/src/designers/designer7_cinema.js`
  - `/home/charles/sherlock-bench/src/designers/designer8_quineling.js`
  - `/home/charles/sherlock-bench/src/engine/sherlockEngine.js`
  - `/home/charles/repos/Moriarty/site/scripts/build-docs.mjs`
  - `/home/charles/repos/Moriarty/site/package.json`
  - `/home/charles/repos/Moriarty/.github/workflows/site.yml`

---

## 1. Executive Summary of Findings

Using formal state-machine modeling in Quint (`specs/quint/ui_playback_lifecycle.qnt`) and systematic code inspection across the UI controller, 3D camera/raycast subsystem, and static GitHub Pages build pipeline, we identified **11 concrete bugs, race conditions, state desyncs, and deployment hazards**:

| ID | Severity | Component | File & Lines | Summary |
|---|---|---|---|---|
| **BUG-UI-01** | **CRITICAL** | Async Trace Race Condition | `src/app.js:395-423` | `executeCurrentSource()` has no monotonic request sequence token (`requestSeq`). Rapid preset clicks or edits while `compileAndRunSherlock()` awaits `POST ./api/trace` allow an older in-flight response to resolve late and overwrite `state.trace`, `state.playhead`, and DOM UI with stale program data while `state.activePresetId` and `els.sourceEditor.value` belong to the newer preset. |
| **BUG-UI-02** | **HIGH** | Premature `state.source` Mutation & Mid-Keystroke QDL Recompiles | `src/app.js:396-411`, `src/engine/sherlockQuinelingCompiler.js:522-560` | `state.source = els.sourceEditor.value` is mutated *before* `await compileAndRunSherlock()` and *before* checking `if (!res.ok)`. On any intermediate syntax error while typing, `state.source` holds broken code while `state.trace` holds the old program trace, causing 60fps `animate()` to trigger full `WovenBody.author` + `WovenBody.compile` (6,000 samples) on fallback ASTs on every keystroke. |
| **BUG-UI-03** | **HIGH** | Stale Step-0 Cache Lock in `#telemetryQdlMorph` | `src/app.js:586-591`, `src/designers/designer8_quineling.js:244-249` | `syncStepUI()` calls `getQuinelingTelemetry({ presetId, source, trace, step })` without passing `stepIndex: clampedIdx`. Inside `getQuinelingTelemetry()`, `const stepIndex = stepState.stepIndex ?? 0` always evaluates to `0`, hitting `_lastTelemetryKey === "${artifact.cacheKey}\|step:0"` on every step `1..N` and freezing `#telemetryQdlMorph` on step 0's gesture for the entire trace. |
| **BUG-UI-04** | **MEDIUM** | Negative `playhead` & Out-of-Bounds `stepIndex` on `0`-Step Traces | `src/app.js:872`, `src/engine/sherlockEngine.js:2254-2258` | In `stepNextBtn` (`src/app.js:872`), `maxStep = state.trace.steps.length - 1` yields `-1` when `steps.length === 0`, setting `state.playhead = -1`. In `buildStepState()` (`src/engine/sherlockEngine.js:2256`), `Math.min(steps.length - 1, ...)` also yields `stepIndex = -1` and `stepFraction = +1.0` when `steps.length === 0`. |
| **BUG-UI-05** | **MEDIUM** | Cache-Hit vs Cache-Miss `progress` Formula Discrepancy | `src/app.js:951-954`, `src/engine/sherlockEngine.js:2258` | On cache miss, `buildStepState()` computes `progress = stepIndex / (totalSteps - 1)` (ignoring `stepFraction`), whereas on cache hit, `getOrBuildStepState()` mutates `cached.progress = (clampedIdx + stepFraction) / (totalSteps - 1)`, causing discontinuous `progress` jumps at step boundaries. |
| **BUG-UI-06** | **MEDIUM** | Keyboard Shortcuts Blocked After Clicking Any Button & Missing Modifier Guard | `src/app.js:910-939` | `window.addEventListener('keydown')` returns early whenever `document.activeElement.tagName === 'BUTTON'`. Clicking Play/Pause, Step, Reset, a Preset card, or a Camera Lens button leaves focus on that `<button>`, breaking `ArrowLeft`, `ArrowRight`, and `1`–`8` shortcuts. Also lacks `if (ev.ctrlKey \|\| ev.metaKey \|\| ev.altKey) return;` before `ev.key >= '1' && ev.key <= '8'`, hijacking `Cmd/Ctrl+1..8` browser tab switching. |
| **BUG-UI-07** | **LOW** | Missing `window.__sherlockApp` Lifecycle & Uncleaned `requestAnimationFrame` / Listeners | `src/app.js:118-120, 910, 977-1008` | `animate()` schedules `requestAnimationFrame(animate)` unconditionally without storing `rafId` or checking `stage._disposed`, and `window` `keydown` listener has no cleanup hook on `window.__sherlockApp`. |
| **BUG-CAM-01** | **HIGH** | Permanent Stuck `isDragging = true` on `pointercancel`, `lostpointercapture`, or `window` `blur` | `src/designers/designer7_cinema.js:569-748, 915-925` | `designer7_cinema.js` only listens for `pointerup` and `pointerleave`. If the OS/browser fires `pointercancel` or `lostpointercapture`, or the window loses focus (`blur`) mid-drag, `isDragging` remains `true` forever: moving the cursor orbits the camera without holding a button, 3D raycast hover/reticle inspection (`if (isDragging) return null`) is permanently disabled, and Cinema Orbit auto-rotation freezes. Multi-pointer interleaving is also unguarded (`activePointerId` not tracked). |
| **BUG-GHP-01** | **CRITICAL** | Import Map Rejected due to Preceding `<link rel="modulepreload">` + 2.28 MB Unused Preload + Missing Favicon | `index.html:12-23`, `Moriarty/visualization/index.html:11-20` | In `sherlock-bench/index.html:12-14`, `<link rel="modulepreload">` is placed *before* `<script type="importmap">` (`lines 15-23`), which per WHATWG HTML Standard § 4.12.1.1.3 sets *acquiring import maps* to `false` and invalidates the import map! Line 14 also preloads `./vendor/three.webgpu.nodes.js` (2.28 MB), which is never imported anywhere. Also lacks `<link rel="icon" href="data:," />`, causing a 404 `/favicon.ico` request. Meanwhile, `Moriarty/visualization/index.html` still uses root-absolute `/src/...` and `/vendor/...` paths that 404 under `/Moriarty/visualization/`. |
| **BUG-GHP-02** | **HIGH** | `IS_STATIC_HOST` False Negative on Subpath / CI `http.server` / Custom Domains + HTTP 501 Loop | `src/engine/sherlockEngine.js:1708-1748` | `IS_STATIC_HOST` only checks `/\.github\.io$/i` and `file:`. When served under `.github/workflows/site.yml` (`python3 -m http.server 8892` at `http://localhost:8892/Moriarty/visualization/`), `vite preview`, or a custom domain, `IS_STATIC_HOST` is `false`, firing `POST ./api/trace` which fails with HTTP `501` (`http.server`) or `404`. Worse, line 1746 only checks `status === 404 \|\| status === 405`, so on HTTP `501` `rustBackendAvailable` stays `true` forever! |
| **BUG-GHP-03** | **HIGH** | `Moriarty/site/scripts/build-docs.mjs` Does Not Stage `../visualization` into `public/visualization` | `Moriarty/site/scripts/build-docs.mjs:1-33`, `Moriarty/site/.gitignore` | `npm run build` in `Moriarty/site` runs `node scripts/build-docs.mjs && tsc -b && vite build`, which only builds `public/docs` and never copies the Sherlock visualization (`index.html`, `src/`, `vendor/`) into `site/public/visualization`, so `site/dist/visualization/index.html` is never produced. |

---

## 2. Formal Quint Specification & Verification Results

We authored `/home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt` to formally model:
1. Concurrent async `requestLoadPreset` calls and out-of-order `resolveTraceRequest` completions, contrasting `applyResolveUnguardedBuggy` (current `src/app.js:395-423`) against `applyResolveGuarded` (monotonic `latestRequestId` vs `appliedRequestId`).
2. Transport & timeline scrubber transitions (`togglePlayPause`, `stepPrev`, `stepNext`, `scrubToStep`, `playbackTick`) across `0`-step, `1`-step, `14`-step, and `45`-step traces.
3. Camera pointer lifecycle (`pointerDown`, `pointerMoveActive`, `pointerUp`, `pointerCancelOrBlur`) with `activePointerId` and capture tracking.
4. Static host (`isStaticHost`) vs local Rust server (`rustBackendAvailable`) trace resolution.

### Verified Invariants
- `val noStaleAsyncTraceOverwrite`: Guarantees `ui.appliedRequestId <= ui.latestRequestId`, `ui.domRenderedPresetId == ui.tracePresetId`, `ui.qdlTelemetryStepIdx == ui.stepIndex`, and once `inFlight` drains, `ui.tracePresetId == ui.activePresetId`.
- `val stepIndexAlwaysInBounds`: Guarantees `ui.stepIndex >= 0`, `(ui.totalSteps == 0) implies (ui.stepIndex == 0)`, `(ui.totalSteps > 0) implies (ui.stepIndex < ui.totalSteps)`, and `0 <= ui.progressPct <= 100`.
- `val pointerDragNeverStuckAfterCancel`: Guarantees that after `pointerCancelOrBlur`, `not(ui.isDragging) and not(ui.isPanning) and not(ui.pointerCaptured) and ui.activePointerId == -1`.
- `val staticHostZeroNetworkFetch`: Guarantees `ui.isStaticHost implies (not(ui.rustBackendAvailable) and ui.networkFetchCount == 0 and inFlight.forall(r => not(r.usedNetworkFetch)))`.

### Quint CLI Verification Output

```text
$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint typecheck /home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt
[ok]

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint test /home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt
  ui_playback_lifecycle
    ok outOfOrderAsyncPresetResolutionTest passed 1 test(s)
    ok zeroAndSingleStepScrubBoundsTest passed 1 test(s)
    ok pointerCancelAndWindowBlurReleaseDragTest passed 1 test(s)
    ok staticGitHubPagesSubpathZeroFetchTest passed 1 test(s)

  4 passing (65ms)

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --verbosity=1 --invariant=noStaleAsyncTraceOverwrite --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt
[ok] No violation found (32ms at 6250 traces/second).
Use --seed=0x218642a940e93b84 --backend=rust to reproduce.

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --verbosity=1 --invariant=stepIndexAlwaysInBounds --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt
[ok] No violation found (36ms at 5556 traces/second).
Use --seed=0x6c1d022b3a69b927 --backend=rust to reproduce.

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --verbosity=1 --invariant=pointerDragNeverStuckAfterCancel --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt
[ok] No violation found (27ms at 7407 traces/second).
Use --seed=0xf11396566dcf752c --backend=rust to reproduce.

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --verbosity=1 --invariant=staticHostZeroNetworkFetch --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/ui_playback_lifecycle.qnt
[ok] No violation found (32ms at 6250 traces/second).
Use --seed=0x4c858c82caa3c5c9 --backend=rust to reproduce.
```

---

## 3. Detailed Bug Analysis & Drop-In Fixes

### 3.1. `BUG-UI-01` & `BUG-UI-02`: Async Trace Overwrite Race & Premature `state.source` Mutation (`src/app.js:395-423`)

#### Root Cause
In `/home/charles/sherlock-bench/src/app.js` (lines 395–423):
```javascript
async function executeCurrentSource(resetPlayhead = true) {
  state.source = els.sourceEditor.value;
  const res = await compileAndRunSherlock(state.source, {
    fuel: state.fuel,
    stackLimit: state.stackLimit,
    sliceWidth: 4,
    preferRust: true,
  });

  if (!res.ok) {
    els.compileErrorBanner.textContent = res.error || 'Compilation error';
    els.compileErrorBanner.classList.remove('hidden');
    return;
  }

  els.compileErrorBanner.classList.add('hidden');
  state.trace = res;
```
1. **Unsequenced Async Resolution (`BUG-UI-01`)**: When the user rapidly clicks Preset A (which triggers an uncached `POST ./api/trace` child-process run taking ~20–50ms) and then clicks Preset B (which hits `clientTraceCache` and resolves on the next microtask), Preset B's response is applied first, and then Preset A's response resolves later and overwrites `state.trace`, `els.stepScrubber.max`, `state.playhead`, and `syncStepUI()`. The UI enters a torn state where `state.activePresetId` and `els.sourceEditor.value` show Preset B, while `state.trace`, the 3D stage, and the Inspector show Preset A.
2. **Premature `state.source` Assignment (`BUG-UI-02`)**: Setting `state.source = els.sourceEditor.value` before `await compileAndRunSherlock` and before `if (!res.ok)` means that during live typing in `#sourceEditor`, every intermediate syntax-error keystroke mutates `state.source`. On the next 60fps `animate()` frame, `getOrBuildStepState()` passes the broken `state.source` into `stage.render(stepState)`, causing `designer8_quineling.js` (`compileSherlockProgramToQuineling`) to miss its cache and execute a full 6,000-sample `WovenBody` + Quine compilation on the fallback AST on every keystroke.
3. **Stale Preset Title After Custom Source Edit**: In `els.sourceEditor.addEventListener('input')` (`src/app.js:831-834`), editing the source code does not clear or check `state.activePresetId`, so `syncStepUI()` (`line 535`) continues displaying the old preset's title (`PRESET_EDITORIAL_META[state.activePresetId].title`) even when the user writes a custom program.

#### Drop-In Fix for `src/app.js`
Add `requestSeq: 0` and `appliedRequestSeq: 0` to `state` (`src/app.js:20-41`), and update `executeCurrentSource` (`src/app.js:395-423`):

```javascript
// In state object (src/app.js:20-41):
  requestSeq: 0,
  appliedRequestSeq: 0,
  rafId: 0,
```

```javascript
// Replace executeCurrentSource (src/app.js:395-423):
async function executeCurrentSource(resetPlayhead = true) {
  const reqId = ++state.requestSeq;
  const draftSource = els.sourceEditor.value;
  const reqFuel = state.fuel;
  const reqStackLimit = state.stackLimit;

  const matchingPreset = PRESET_PROGRAMS.find((p) => p.source === draftSource);
  const nextPresetId = matchingPreset ? matchingPreset.id : null;
  if (state.activePresetId !== nextPresetId) {
    state.activePresetId = nextPresetId;
    renderPresets();
  }

  const res = await compileAndRunSherlock(draftSource, {
    fuel: reqFuel,
    stackLimit: reqStackLimit,
    sliceWidth: 4,
  });

  // Guard against out-of-order async resolution
  if (reqId < state.appliedRequestSeq || reqId !== state.requestSeq) {
    return;
  }
  state.appliedRequestSeq = reqId;

  if (!res.ok) {
    els.compileErrorBanner.textContent = res.error || 'Compilation error';
    els.compileErrorBanner.classList.remove('hidden');
    return;
  }

  els.compileErrorBanner.classList.add('hidden');
  state.source = draftSource;
  state.trace = res;

  const totalSteps = Array.isArray(res.steps) ? res.steps.length : 0;
  const maxStep = Math.max(0, totalSteps - 1);
  els.stepScrubber.max = String(maxStep);
  els.stepScrubber.step = '1';
  if (resetPlayhead || totalSteps <= 1) {
    state.playhead = 0;
  } else {
    state.playhead = Math.max(0, Math.min(state.playhead, maxStep));
  }
  state.cachedStepState = null;
  state.cachedTraceRef = null;
  state.cachedStepIdx = -1;
  state.lastRenderedStepIdx = -1;
  syncStepUI(Math.floor(state.playhead));
}
```

---

### 3.2. `BUG-UI-03`: Stale Step-0 Cache Lock in `#telemetryQdlMorph` (`src/app.js:586-591`, `src/designers/designer8_quineling.js:244-249`)

#### Root Cause
In `/home/charles/sherlock-bench/src/app.js` (lines 586–591):
```javascript
  const qdlTel = getQuinelingTelemetry({
    presetId: state.activePresetId,
    source: state.source,
    trace: state.trace,
    step,
  });
```
And in `/home/charles/sherlock-bench/src/designers/designer8_quineling.js` (lines 243–249):
```javascript
export function getQuinelingTelemetry(stepState = {}) {
  const artifact = resolveCompiledQuineling(stepState);
  const stepIndex = stepState.stepIndex ?? 0;
  const telemetryKey = `${artifact.cacheKey}|step:${stepIndex}`;
  if (telemetryKey === _lastTelemetryKey && _lastTelemetryResult) {
    return _lastTelemetryResult;
  }
```
Because `syncStepUI()` omits `stepIndex: clampedIdx` when calling `getQuinelingTelemetry(...)`, `stepState.stepIndex ?? 0` always evaluates to `0`. Consequently, `telemetryKey` is `${artifact.cacheKey}|step:0` on every step `0..N-1`, and `getQuinelingTelemetry()` returns the cached step-0 result forever without updating `morphGesture` (`UNFURLING PATCH`, `CHIRAL PHASE LOCK`, `16-LIMB ALU CONVERGENCE`, `CROWN BLOOM · ... DONE`) as the trace steps forward.

#### Drop-In Fix
1. In `src/app.js:586-591`, pass `stepIndex: clampedIdx`:
```javascript
  const qdlTel = getQuinelingTelemetry({
    presetId: state.activePresetId,
    source: state.source,
    trace: state.trace,
    step,
    stepIndex: clampedIdx,
  });
```
2. In `src/designers/designer8_quineling.js:245`, fall back to `stepState.step?.index`:
```javascript
  const stepIndex = stepState.stepIndex ?? stepState.step?.index ?? 0;
```

---

### 3.3. `BUG-UI-04` & `BUG-UI-05`: `0`-Step / `1`-Step Scrubbing Bounds & `progress` Formula Discrepancy (`src/app.js:872, 951-954`, `src/engine/sherlockEngine.js:2252-2260`)

#### Root Cause
1. In `src/app.js:872` (`els.stepNextBtn` click handler):
   ```javascript
   const maxStep = state.trace ? state.trace.steps.length - 1 : 0;
   state.playhead = Math.min(maxStep, Math.floor(state.playhead) + 1);
   ```
   When `state.trace.steps.length === 0`, `maxStep` is `-1`, setting `state.playhead = -1` and `els.stepScrubber.value = "-1"`.
2. In `src/engine/sherlockEngine.js:2252-2260` (`buildStepState`):
   ```javascript
   export function buildStepState({ trace, playhead = 0, ambient = 0, dt = 0.016 }) {
     const steps = trace?.steps || [];
     const totalSteps = Math.max(1, steps.length);
     const clampedPlayhead = Math.max(0, Math.min(totalSteps - 0.0001, playhead));
     const stepIndex = Math.min(steps.length - 1, Math.floor(clampedPlayhead));
     const stepFraction = clampedPlayhead - stepIndex;
     const progress = totalSteps <= 1 ? 1 : stepIndex / (totalSteps - 1);
   ```
   - When `steps.length === 0`, `steps.length - 1` is `-1`, so `stepIndex` becomes `-1` and `stepFraction` becomes `clampedPlayhead - (-1) = +1.0`.
   - Furthermore, on a cache miss in `getOrBuildStepState()` (`src/app.js:962`), `buildStepState()` sets `progress = stepIndex / (totalSteps - 1)` (without `stepFraction`), whereas on a cache hit (`src/app.js:951-954`), `getOrBuildStepState()` sets `cached.progress = (clampedIdx + stepFraction) / (totalSteps - 1)` (with `stepFraction`).

#### Drop-In Fix
1. In `src/app.js:872`:
```javascript
  const maxStep = state.trace ? Math.max(0, state.trace.steps.length - 1) : 0;
  state.playhead = Math.min(maxStep, Math.floor(state.playhead) + 1);
```
2. In `src/engine/sherlockEngine.js:2252-2260`:
```javascript
export function buildStepState({ trace, playhead = 0, ambient = 0, dt = 0.016 }) {
  const steps = trace?.steps || [];
  const totalSteps = Math.max(1, steps.length);
  const maxStepIdx = Math.max(0, steps.length - 1);
  const clampedPlayhead = Math.max(0, Math.min(maxStepIdx, Number.isFinite(playhead) ? playhead : 0));
  const stepIndex = Math.max(0, Math.min(maxStepIdx, Math.floor(clampedPlayhead)));
  const stepFraction = Math.max(0, Math.min(1, clampedPlayhead - stepIndex));
  const progress =
    totalSteps <= 1
      ? 1
      : Math.min(1, Math.max(0, (stepIndex + stepFraction) / (totalSteps - 1)));
  const step = steps[stepIndex] || steps[0] || null;
```

---

### 3.4. `BUG-UI-06` & `BUG-UI-07`: Keyboard Shortcut Focus Trap, Modifier Hijack & App Disposal (`src/app.js:910-939, 977-1009`)

#### Root Cause
In `/home/charles/sherlock-bench/src/app.js` (lines 910–939):
```javascript
window.addEventListener('keydown', (ev) => {
  const active = document.activeElement;
  const tag = active?.tagName;
  if (
    tag === 'TEXTAREA' ||
    tag === 'INPUT' ||
    tag === 'SELECT' ||
    tag === 'BUTTON' ||
    tag === 'SUMMARY' ||
    active?.getAttribute?.('role') === 'button'
  ) {
    return;
  }
```
- Because `tag === 'BUTTON'` is blocked for all keys, clicking any button on the workbench (`#playPauseBtn`, `#stepPrevBtn`, `#stepNextBtn`, `#resetStepBtn`, `.preset-btn`, `#cameraLensBar button`, `.rec-call-node`) leaves focus on that `<button>`, rendering `ArrowLeft`, `ArrowRight`, and `1`–`8` shortcuts completely unresponsive until the user clicks on non-interactive background space. (Note: `Space` on a focused `<button>` already triggers its native click in the browser, so only `Space` needs to avoid double-triggering when a `<button>` is focused.)
- Similarly, when `#stepScrubber` (`<input type="range">`) is focused, pressing `Space` or `1`–`8` is blocked.
- Furthermore, `else if (ev.key >= '1' && ev.key <= '8')` does not check `if (ev.ctrlKey || ev.metaKey || ev.altKey) return;`, intercepting `Cmd+1..8` / `Ctrl+1..8` browser tab-switching shortcuts.
- Finally, `window.addEventListener('keydown', ...)` and `requestAnimationFrame(animate)` have no cleanup method and `window.__sherlockApp` is not exposed for lifecycle management.

#### Drop-In Fix for `src/app.js:910-1009`
```javascript
function onGlobalKeyDown(ev) {
  if (ev.ctrlKey || ev.metaKey || ev.altKey) return;
  const active = document.activeElement;
  const tag = active?.tagName;
  if (tag === 'TEXTAREA' || tag === 'SELECT' || active?.isContentEditable) {
    return;
  }
  const isButtonFocus =
    tag === 'BUTTON' || tag === 'SUMMARY' || active?.getAttribute?.('role') === 'button';
  const isRangeInput = tag === 'INPUT' && active?.type === 'range';
  if (tag === 'INPUT' && !isRangeInput) {
    return;
  }

  if (ev.code === 'Space') {
    // If a button is focused, let the browser's native Space activation handle it unless it's not playPauseBtn
    if (isButtonFocus && active === els.playPauseBtn) return;
    ev.preventDefault();
    els.playPauseBtn.click();
  } else if (ev.code === 'ArrowLeft') {
    if (isRangeInput) return;
    ev.preventDefault();
    if (!els.stepPrevBtn.disabled) els.stepPrevBtn.click();
  } else if (ev.code === 'ArrowRight') {
    if (isRangeInput) return;
    ev.preventDefault();
    if (!els.stepNextBtn.disabled) els.stepNextBtn.click();
  } else if (ev.key >= '1' && ev.key <= '8') {
    const idx = Number(ev.key) - 1;
    if (PRESET_PROGRAMS[idx]) {
      ev.preventDefault();
      selectPreset(PRESET_PROGRAMS[idx]);
    }
  }
}

window.addEventListener('keydown', onGlobalKeyDown);
```
And in `animate(now)` (`src/app.js:977-1009`):
```javascript
function animate(now) {
  if (stage._disposed) return;
  state.rafId = requestAnimationFrame(animate);
  const dt = Math.min(0.05, Math.max(0.001, (now - state.lastTick) / 1000));
  state.lastTick = now;
  ...
}

function disposeApp() {
  if (state.rafId) {
    cancelAnimationFrame(state.rafId);
    state.rafId = 0;
  }
  window.removeEventListener('keydown', onGlobalKeyDown);
  stage.dispose();
}

if (typeof window !== 'undefined') {
  window.__sherlockApp = { state, stage, selectPreset, executeCurrentSource, dispose: disposeApp };
}
```

---

### 3.5. `BUG-CAM-01`: Pointer Drag / Capture Stuck State in `src/designers/designer7_cinema.js` (`lines 569-748, 915-925`)

#### Root Cause
In `/home/charles/sherlock-bench/src/designers/designer7_cinema.js`:
1. `domElement` registers listeners for `pointerdown`, `pointermove`, `pointerup`, `pointerleave`, `wheel`, and `contextmenu` (`lines 740-748`), but **never** registers `pointercancel`, `lostpointercapture`, or `window` `blur`.
2. When `pointercancel` fires (e.g., palm rejection, OS gesture takeover, touch scroll interrupt, Escape key) or the browser window blurs (`Alt+Tab`) while `isDragging === true`, `isDragging` stays `true` permanently:
   - `onPointerMove` (`line 591: if (isDragging)`) continues orbiting or panning the camera on every mouse movement even with zero buttons pressed (`event.buttons === 0`).
   - `performRaycast` (`line 539: if (!camera || !scene || !pointerInside || isDragging) return null;`) permanently aborts all 3D raycast hover and reticle updates.
   - `update` (`line 808: if (currentPresetName === 'cinema' && !isDragging)`) permanently freezes Cinema Orbit rotation.
   - `domElement.style.cursor` remains stuck on `'grabbing'`.
3. `activePointerId` is not tracked, so a secondary touch pointer down/up during a multi-touch gesture corrupts `lastPointerX`/`lastPointerY` deltas and prematurely terminates the primary drag.

#### Drop-In Fix for `src/designers/designer7_cinema.js`
Replace the pointer handlers and `dispose()` in `src/designers/designer7_cinema.js` (`lines 424-429, 569-748, 915-925`):

```javascript
  let isDragging = false;
  let isPanning = false;
  let activePointerId = null;
  let dragMovedDistance = 0;
  let lastPointerX = 0;
  let lastPointerY = 0;
```

```javascript
  function releaseDragState(pointerId = activePointerId) {
    if (
      domElement &&
      domElement.releasePointerCapture &&
      pointerId !== null &&
      pointerId !== undefined
    ) {
      try {
        if (!domElement.hasPointerCapture || domElement.hasPointerCapture(pointerId)) {
          domElement.releasePointerCapture(pointerId);
        }
      } catch (_) {
        // Ignore release errors
      }
    }
    isDragging = false;
    isPanning = false;
    activePointerId = null;
    if (domElement) {
      domElement.style.cursor = hoveredObject ? 'pointer' : 'grab';
    }
  }

  function onPointerDown(event) {
    if (!domElement) return;
    if (isDragging && activePointerId !== null && event.pointerId !== activePointerId) {
      return;
    }
    isDragging = true;
    activePointerId = event.pointerId ?? null;
    isPanning = event.button === 2 || event.button === 1 || event.shiftKey;
    dragMovedDistance = 0;
    lastPointerX = event.clientX;
    lastPointerY = event.clientY;
    pointerInside = true;
    updatePointerNDC(event);
    if (domElement.setPointerCapture && event.pointerId !== undefined) {
      try {
        domElement.setPointerCapture(event.pointerId);
      } catch (_) {
        // Ignore pointer capture errors on synthetic events
      }
    }
    domElement.style.cursor = 'grabbing';
  }

  function onPointerMove(event) {
    pointerInside = true;

    if (isDragging) {
      if (activePointerId !== null && event.pointerId !== undefined && event.pointerId !== activePointerId) {
        return;
      }
      // Self-healing guard if pointerup was missed outside window without pointer capture
      if (event.buttons !== undefined && event.buttons === 0 && event.pointerType === 'mouse') {
        releaseDragState(event.pointerId);
        return;
      }
      const dx = event.clientX - lastPointerX;
      const dy = event.clientY - lastPointerY;
      lastPointerX = event.clientX;
      lastPointerY = event.clientY;
      dragMovedDistance += Math.hypot(dx, dy);
      ...
```

```javascript
  function onPointerCancel(event) {
    if (activePointerId !== null && event?.pointerId !== undefined && event.pointerId !== activePointerId) {
      return;
    }
    releaseDragState(event?.pointerId);
  }

  function onWindowBlur() {
    if (isDragging) {
      releaseDragState();
    }
  }

  if (domElement && domElement.addEventListener) {
    domElement.style.cursor = 'grab';
    domElement.addEventListener('pointerdown', onPointerDown);
    domElement.addEventListener('pointermove', onPointerMove);
    domElement.addEventListener('pointerup', onPointerUp);
    domElement.addEventListener('pointercancel', onPointerCancel);
    domElement.addEventListener('lostpointercapture', onPointerCancel);
    domElement.addEventListener('pointerleave', onPointerLeave);
    domElement.addEventListener('wheel', onWheel, { passive: false });
    domElement.addEventListener('contextmenu', onContextMenu);
  }
  if (typeof window !== 'undefined' && window.addEventListener) {
    window.addEventListener('blur', onWindowBlur);
  }
```
And in `dispose()` (`src/designers/designer7_cinema.js:915-925`):
```javascript
  function dispose() {
    releaseDragState();
    if (domElement && domElement.removeEventListener) {
      domElement.removeEventListener('pointerdown', onPointerDown);
      domElement.removeEventListener('pointermove', onPointerMove);
      domElement.removeEventListener('pointerup', onPointerUp);
      domElement.removeEventListener('pointercancel', onPointerCancel);
      domElement.removeEventListener('lostpointercapture', onPointerCancel);
      domElement.removeEventListener('pointerleave', onPointerLeave);
      domElement.removeEventListener('wheel', onWheel);
      domElement.removeEventListener('contextmenu', onContextMenu);
    }
    if (typeof window !== 'undefined' && window.removeEventListener) {
      window.removeEventListener('blur', onWindowBlur);
    }
    reticle.dispose();
  }
```

---

### 3.6. `BUG-GHP-01`, `BUG-GHP-02` & `BUG-GHP-03`: Static GitHub Pages Subpath Compatibility & `Moriarty/site` Build Pipeline

#### 1. `index.html` Import Map Ordering, Unused 2.28 MB Preload & Favicon (`index.html:6-24`)
- **Problem**:
  1. In `/home/charles/sherlock-bench/index.html` (lines 12–23), `<link rel="modulepreload" href="./vendor/three.webgpu.js" />` is placed *before* `<script type="importmap">`. Per the WHATWG HTML Standard (§ 4.12.1.1.3), fetching any module or `modulepreload` link sets the document's *acquiring import maps* flag to `false`, causing subsequent `<script type="importmap">` tags to fail with a console error!
  2. Line 14 preloads `./vendor/three.webgpu.nodes.js` (2,276,209 bytes / 2.28 MB), which is never imported by `three.webgpu.js` or any file in `src/`.
  3. Missing `<link rel="icon" href="data:," />` causes browsers to request `/favicon.ico` (404 Not Found), which fails Playwright CI suites (`page.on("response", ... r.status >= 400)` in `Moriarty/site/test/`).
  4. `/home/charles/repos/Moriarty/visualization/index.html` still has root-absolute `/src/styles.css`, `/vendor/three.webgpu.js`, and `/src/app.js` paths, which 404 when served under `/Moriarty/visualization/index.html`.
- **Drop-In Fix for `index.html:6-24`**:
```html
  <title>Sherlock — The execution engine at the center of Moriarty</title>
  <meta name="description" content="Sherlock is the execution engine at the center of Moriarty: a small computer whose internal workings are intentionally visible so every step can be accounted for and proved." />
  <link rel="icon" href="data:," />
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,400;0,500;0,600;0,700;1,400&family=Newsreader:ital,opsz,wght@0,6..72,400;0,6..72,500;0,6..72,600;1,6..72,400&family=Plus+Jakarta+Sans:wght@400;500;600;700&display=swap" rel="stylesheet" />
  <link rel="stylesheet" href="./src/styles.css" />
  <script type="importmap">
    {
      "imports": {
        "three": "./vendor/three.webgpu.js",
        "three/webgpu": "./vendor/three.webgpu.js",
        "three/tsl": "./vendor/three.tsl.js"
      }
    }
  </script>
  <link rel="modulepreload" href="./vendor/three.webgpu.js" />
  <link rel="modulepreload" href="./vendor/three.core.js" />
```

#### 2. `IS_STATIC_HOST` & HTTP Status Fallback in `src/engine/sherlockEngine.js` (`lines 1708-1750`)
- **Problem**:
  - `IS_STATIC_HOST` only checks `/\.github\.io$/i.test(window.location.hostname || '') || window.location.protocol === 'file:'`.
  - When the static site is served under `.github/workflows/site.yml` (`python3 -m http.server 8892` at `http://localhost:8892/Moriarty/visualization/index.html`), `vite preview`, or a custom GitHub Pages domain (`CNAME`), `IS_STATIC_HOST` evaluates to `false`.
  - Furthermore, `src/app.js:401` explicitly passes `preferRust: true` instead of relying on the default `preferRust = !IS_STATIC_HOST`.
  - Even worse, when `python3 -m http.server` receives `POST ./api/trace`, it responds with **HTTP `501 Not Implemented`**, which does not match `if (resp.status === 404 || resp.status === 405)` (`line 1746`), leaving `rustBackendAvailable = true` forever!
- **Drop-In Fix for `src/engine/sherlockEngine.js:1708-1750`**:
```javascript
const IS_STATIC_HOST =
  typeof window !== 'undefined' &&
  (/\.github\.io$/i.test(window.location.hostname || '') ||
    window.location.protocol === 'file:' ||
    /\/visualization(?:\/|$)/i.test(window.location.pathname || '') ||
    (window.location.hostname !== '127.0.0.1' && window.location.hostname !== 'localhost') ||
    (window.location.port !== '4377' && !window.__SHERLOCK_ENABLE_RUST_API__));

let rustBackendAvailable = !IS_STATIC_HOST;
```
And inside `compileAndRunSherlock` (`src/engine/sherlockEngine.js:1746-1750`):
```javascript
      if (!resp.ok) {
        if (resp.status === 404 || resp.status === 405 || resp.status === 501) {
          rustBackendAvailable = false;
        }
      } else {
```

#### 3. Staging `../visualization` in `Moriarty/site/scripts/build-docs.mjs` & `Moriarty/site/.gitignore`
- **Problem**:
  - `Moriarty/site/package.json` runs `"build": "npm run docs && tsc -b && vite build"`, where `"docs": "node scripts/build-docs.mjs"`.
  - Vite copies everything inside `site/public/` to `site/dist/`.
  - Currently `build-docs.mjs` only populates `site/public/docs/` and does not copy the Sherlock visualization (`../visualization`) into `site/public/visualization/`.
  - Note that `../visualization` also contains non-web artifacts (`rust-runner/`, `audits/`, `server.js`, `package.json`, `public/graphify/`, and unused `vendor/three.webgpu.nodes.js` / `vendor/three.module.js`) that should **not** bloat the static GitHub Pages deployment artifact.
- **Drop-In Addition for `/home/charles/repos/Moriarty/site/scripts/build-docs.mjs` (append after line 32)**:
```javascript
import { cp, rm } from 'node:fs/promises';

// Stage static Sherlock visualization into public/visualization for Vite build & dev
const vizSource = path.resolve(root, '../visualization');
const vizOutput = path.join(root, 'public/visualization');
await rm(vizOutput, { recursive: true, force: true });
await mkdir(path.join(vizOutput, 'vendor'), { recursive: true });

await cp(path.join(vizSource, 'index.html'), path.join(vizOutput, 'index.html'));
await cp(path.join(vizSource, 'src'), path.join(vizOutput, 'src'), {
  recursive: true,
  filter: (srcPath) => !srcPath.endsWith('.md'),
});
for (const vendorFile of ['three.core.js', 'three.webgpu.js', 'three.tsl.js']) {
  await cp(
    path.join(vizSource, 'vendor', vendorFile),
    path.join(vizOutput, 'vendor', vendorFile)
  );
}
console.log('Staged Sherlock visualization into public/visualization.');
```
- **Drop-In Addition for `/home/charles/repos/Moriarty/site/.gitignore`**:
```gitignore
node_modules/
dist/
public/docs/
public/visualization/
*.tsbuildinfo
```
