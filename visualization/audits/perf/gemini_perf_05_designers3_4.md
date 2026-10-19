# Gemini Argon High Performance Audit #5: Designer 3 (Semantic Frames) & Designer 4 (Wafer Arena)

**Audited Files:**
- `/home/charles/sherlock-bench/src/designers/designer3_frames.js` (Lines 1–778)
- `/home/charles/sherlock-bench/src/designers/designer4_arena.js` (Lines 1–509)

All optimizations below preserve **100% of visual quality, animations, and raycaster inspection metadata** while eliminating redundant Canvas 2D rasterizations, GPU texture uploads, per-frame string/regex/Euler math, and scene-graph matrix recomputations.

---

## Priority 1 (Critical): Canvas 2D Texture Generation, Caching & Startup VRAM Waste

### 1.1 `designer4_arena.js` — Replace Per-Slot Mutable Canvas Re-Rasterization with Signature-Keyed LRU Texture Cache + Fix Startup Double-Paint
- **Location:** `/home/charles/sherlock-bench/src/designers/designer4_arena.js`, lines `228–284`, `357–399`, `464–468`, and `501`
- **Problem:**
  1. **Window-Sliding & Scrubbing Re-Uploads:** Each of the 6 `callTiles` owns a single mutable `512×256` `<canvas>` and `THREE.CanvasTexture`. Whenever a recursive trace with `> 6` calls (e.g. `fib(4)` with 9 calls) slides its 6-tile window (`windowStart` at line 201), **all 6 tiles shift by 1 index**, causing all 6 `512×256` canvases to be cleared, re-rendered with 2D text, and re-uploaded to GPU VRAM (`tex.needsUpdate = true`, **3 MB of synchronous texture uploads in a single step**), even though 5 of those 6 exact textures were already rendered on adjacent tiles 16ms earlier. Scrubbing backward/forward or looping playback likewise re-rasterizes and re-uploads every status change.
  2. **Startup Double-Paint:** Lines `360–361` paint `fact(4)..fact(0)` onto all 6 canvases during `createArenaSubsystem()`, but line `394` initializes `lastSignature: ''` and line `501` runs a dummy `update({ ambient: 0, dt: 0.016, stepFraction: 0 })` without a trace. On the very first real frame, all active tiles miss `lastSignature` and re-paint the exact same textures.
- **Drop-In Code Snippet (`src/designers/designer4_arena.js`, replacing lines `228–284` and updating lines `357–399`, `464–468`):**
```javascript
// Replace lines 228-284 in src/designers/designer4_arena.js:
const MAX_CALL_TILE_CACHE_SIZE = 256;
const _callTileTextureCache = new Map();

function getOrCreateCallTileTexture(label, value, status, ringTag) {
  const sig = `${label}|${value}|${status}|${ringTag}`;
  if (_callTileTextureCache.has(sig)) {
    const cached = _callTileTextureCache.get(sig);
    _callTileTextureCache.delete(sig);
    _callTileTextureCache.set(sig, cached);
    return cached;
  }

  const canvas = makeCanvas(512, 256);
  if (!canvas) {
    const data = new Uint8Array([0, 0, 0, 0]);
    const fallbackTex = new THREE.DataTexture(data, 1, 1, THREE.RGBAFormat);
    fallbackTex.needsUpdate = true;
    _callTileTextureCache.set(sig, fallbackTex);
    return fallbackTex;
  }

  const ctx = canvas.getContext('2d');
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  const isReturned = status === 'resolved';
  const isActive = status === 'active';
  const isWaiting = status === 'waiting';

  // Subtle top-right Ring indicator (R1..R6)
  ctx.textAlign = 'right';
  ctx.textBaseline = 'top';
  ctx.fillStyle = isReturned
    ? 'rgba(167, 243, 208, 0.72)'
    : isActive || isWaiting
      ? 'rgba(253, 230, 138, 0.72)'
      : 'rgba(155, 150, 140, 0.45)';
  ctx.font = '600 32px "JetBrains Mono", ui-monospace, monospace';
  ctx.fillText(ringTag, canvas.width - 24, 18);

  // Main call expression (matching clean reference `paintTile`)
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillStyle = isReturned
    ? '#dfe9df'
    : isActive
      ? '#f4d9c8'
      : isWaiting
        ? '#e7e1d4'
        : '#9b968c';
  const hasVal = isReturned && Boolean(value);
  ctx.font = `600 ${hasVal ? 68 : 80}px "JetBrains Mono", ui-monospace, monospace`;
  ctx.fillText(label, canvas.width / 2, hasVal ? canvas.height * 0.38 : canvas.height * 0.54);

  if (hasVal) {
    ctx.fillStyle = '#cfe3d4';
    ctx.font = '500 58px "JetBrains Mono", ui-monospace, monospace';
    ctx.fillText(`→ ${value}`, canvas.width / 2, canvas.height * 0.76);
  }

  const tex = new THREE.CanvasTexture(canvas);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = 8;
  tex.needsUpdate = true;

  if (_callTileTextureCache.size >= MAX_CALL_TILE_CACHE_SIZE) {
    const oldestKey = _callTileTextureCache.keys().next().value;
    if (oldestKey !== undefined) {
      _callTileTextureCache.delete(oldestKey);
    }
  }
  _callTileTextureCache.set(sig, tex);
  return tex;
}
```
And in `createArenaSubsystem()` (lines `357–399` and `464–468`):
```javascript
    // Replace lines 357-368:
    const ringNum = i + 1;
    const initLabel = `fact(${Math.max(0, 4 - i)})`;
    const initRingTag = `R${ringNum}`;
    const initSig = `${initLabel}||future|${initRingTag}`;
    const initTex = getOrCreateCallTileTexture(initLabel, '', 'future', initRingTag);

    const plaqueMat = trackMat(
      new THREE.MeshBasicMaterial({
        map: initTex,
        transparent: true,
        alphaTest: 0.08,
        depthWrite: false,
        toneMapped: false,
      })
    );
```
```javascript
    // In callTiles.push(...) at lines 388-398:
    callTiles.push({
      index: i,
      group: tileGroup,
      bodyMat,
      edgeMat,
      plaqueMat,
      lastSignature: initSig,
      lastZ: NaN,
      lastStyleKey: '',
      lastMetaCallIndex: -1,
      lastMetaStatus: '',
      lastMetaReturn: '',
    });
```
```javascript
    // Replace lines 464-468 in update():
    const sig = `${call.callExpr}|${retVal}|${call.status}|${ringTag}`;
    if (sig !== tile.lastSignature) {
      tile.lastSignature = sig;
      const nextTex = getOrCreateCallTileTexture(call.callExpr, retVal, call.status, ringTag);
      if (tile.plaqueMat.map !== nextTex) {
        tile.plaqueMat.map = nextTex;
        tile.plaqueMat.needsUpdate = true;
      }
    }
```

---

### 1.2 `designer3_frames.js` — Eliminate 12 Unused Startup Canvas Textures & Defer Hidden Plaque Rasterization
- **Location:** `/home/charles/sherlock-bench/src/designers/designer3_frames.js`, lines `468–490`, `530–577`, and `720–728`
- **Problem:**
  1. **12 Unused Startup Textures (3.1 MB VRAM):** In `createFramesSubsystem()` (lines `469–479`), the loop creates 12 unique `512×128` `<canvas>` textures (`title: 'Frame #1'..'Frame #12'`, `subtitle: 'R1·A · Saved continuation ring'`) for slots that start with `slotGroup.visible = false` and whose `plaqueMat.map` is overwritten the first time the slot becomes active.
  2. **Invisible Badge Rasterization in Exterior Presets:** In 7 of the 11 camera presets (including default `'overview'`, `'well'`, `'arena'`, `'circuit'`, `'aux2'`, `'loom'`, `'cinema'`), `showWaferBadges` is `false` and `slot.plaqueMesh.visible` is `false`. Yet `updateWaferPlaque` unconditionally calls `createPlaqueTexture(...)` on every step change, creating `<canvas>` elements and `THREE.CanvasTexture` objects for meshes with `visible === false`.
  3. **Redundant Regex Formatting on Unchanged Outer Frames:** When `isStepChanged` is true, outer continuation frames (`chronoIdx = 0, 1, 2...`) remain unchanged on the stack across dozens of steps. Currently `updateWaferPlaque` runs `formatWaferPlaqueTitle` (4 regex passes: `/\[hole\]/gi`, `/\s*\*\s*/g`, `/fn\[\d+\]:([A-Za-z0-9_]+)/`, `/\s+/g`) for all active slots on every step change just to recompute `signature` and allocate a new 5-element `details` array.
- **Drop-In Code Snippet (`src/designers/designer3_frames.js`, lines `468–577` and `720–728`):**
```javascript
    // Replace lines 468-489 in createFramesSubsystem() with a single shared lazy material setup:
    const plaqueMat = trackMat(
      new THREE.MeshBasicMaterial({
        map: null,
        transparent: true,
        opacity: 0.96,
        depthWrite: false,
        side: THREE.DoubleSide,
      })
    );
```
```javascript
    // In tagInspectable & waferSlots.push (lines 499-524), pre-allocate the 5-item details array:
    const detailsArr = [
      `Half-Ring Slot ID: ${slotId} (chronological index ${i})`,
      'Annular Ring Bore: innerRadius = 1.02, outerRadius = 1.28',
      '',
      '',
      '',
    ];
    tagInspectable(slotGroup, {
      type: 'frame',
      id: `saved-frame-wafer-${i}`,
      title: `[${slotId}] Annular Continuation Ring #${i + 1}`,
      subtitle: `Open-Bore Continuation Ring (r = 1.02..1.28) · Slot ${slotId}`,
      details: detailsArr,
    });

    const initialY = -0.64 - i * 0.19;
    waferSlots.push({
      slotIndex: i,
      slotId,
      group: slotGroup,
      rimMat,
      plaqueMat,
      plaqueMesh,
      badgeOpacity: 0.0,
      lastSignature: '',
      renderedTexSignature: '',
      pendingTitle: '',
      pendingSubtitle: '',
      pendingSemFamily: SEMANTIC_FAMILY_DESC.OP_LEFT,
      lastRawKind: '',
      lastRawLabel: '',
      lastRawAstPath: '',
      lastRawEnvId: -1,
      lastRawOriginExprId: null,
      lastBindName: '',
      lastBindVal: '',
      kind: 'Return',
      semFamily: SEMANTIC_FAMILY_DESC.ENV_RETURN,
      lastColorHex: -1,
      lastEmissiveIntensity: -1,
      currentY: initialY,
      targetY: initialY,
      lastActiveY: initialY,
      currentScale: 1.0,
      targetScale: 0.97,
      isDeepestActive: false,
      isUnwindTarget: false,
      wasVisible: false,
      flashIntensity: 0,
    });
```
```javascript
  // Replace updateWaferPlaque (lines 530-577) with fast pre-key guard + deferred texture binding:
  const VALID_FRAME_KINDS = new Set(ALL_FRAME_KINDS);

  function syncWaferPlaqueTexture(slot) {
    if (slot.renderedTexSignature === slot.lastSignature || !slot.lastSignature) return;
    slot.renderedTexSignature = slot.lastSignature;
    const semFamily = slot.pendingSemFamily;
    const tex = createPlaqueTexture({
      badge: slot.slotId,
      title: slot.pendingTitle,
      subtitle: slot.pendingSubtitle,
      accent: semFamily.css,
      bg: semFamily.bg,
      fg: '#f3efe6',
      muted: semFamily.muted,
      width: 512,
      height: 128,
    });
    if (slot.plaqueMat.map !== tex) {
      slot.plaqueMat.map = tex;
      slot.plaqueMat.needsUpdate = true;
    }
  }

  function updateWaferPlaqueMeta(slot, frame, depthNumber, effectiveBindings, semFamily, kind) {
    const b0 =
      Array.isArray(effectiveBindings) && effectiveBindings.length > 0
        ? effectiveBindings[0]
        : null;
    const bindName = b0 ? String(b0.name ?? '') : '';
    const bindVal = b0 ? String(b0.valueSummary ?? '') : '';
    const rawLabel = frame.label || kind;
    const rawAstPath = frame.astPath || 'root';
    const rawEnvId = frame.envId ?? 0;
    const rawOriginExprId = frame.originExprId ?? null;

    // Fast O(1) guard: skip all regex formatting & string building if frame + binding are unchanged
    if (
      slot.lastRawKind === kind &&
      slot.lastRawLabel === rawLabel &&
      slot.lastRawAstPath === rawAstPath &&
      slot.lastRawEnvId === rawEnvId &&
      slot.lastRawOriginExprId === rawOriginExprId &&
      slot.lastBindName === bindName &&
      slot.lastBindVal === bindVal
    ) {
      return;
    }

    slot.lastRawKind = kind;
    slot.lastRawLabel = rawLabel;
    slot.lastRawAstPath = rawAstPath;
    slot.lastRawEnvId = rawEnvId;
    slot.lastRawOriginExprId = rawOriginExprId;
    slot.lastBindName = bindName;
    slot.lastBindVal = bindVal;

    const meta = FRAME_KIND_META[kind] || FRAME_KIND_META.Return;
    const badge = slot.slotId;
    const title = formatWaferPlaqueTitle(frame, effectiveBindings);
    const subtitle = `${semFamily.family} (${meta.badge}) · D${depthNumber} · env#${rawEnvId}`;
    const accentCss = semFamily.css;

    slot.pendingTitle = title;
    slot.pendingSubtitle = subtitle;
    slot.pendingSemFamily = semFamily;
    slot.lastSignature = `${badge}|${title}|${subtitle}|${accentCss}`;

    const ud = slot.group.userData;
    ud.inspectable = true;
    ud.type = 'frame';
    ud.id = `frame-wafer-${depthNumber}`;
    ud.slotId = badge;
    ud.kind = kind;
    ud.semanticFamily = semFamily.family;
    ud.originExprId = rawOriginExprId;
    ud.astPath = rawAstPath;
    ud.envId = rawEnvId;
    ud.title = `[${badge}] ${title}`;
    ud.subtitle = `Slot ${badge} (Depth ${depthNumber}) · ${semFamily.roleDesc}`;
    const details = ud.details;
    details[2] = `Operation & Binding: ${title}`;
    details[3] = `Semantic Ring Family: ${semFamily.roleDesc}`;
    details[4] = `AST Origin: Expr #${rawOriginExprId ?? 'root'} (${ frame.astPath || 'entry' })`;
  }
```

---

## Priority 2 (High): Per-Frame `.update()` vs Step-Change Separation & Settled-State Dirty Gating

### 2.1 `designer3_frames.js` — Hoist Step-Invariant Logic Out of 60fps Loop, Eliminate Per-Frame `Quaternion.setFromEuler`, and Gate Settled Transforms
- **Location:** `/home/charles/sherlock-bench/src/designers/designer3_frames.js`, lines `491–498` and `584–759`
- **Problem:**
  1. **Per-Frame `String()` & Substring Scans (Lines `648–660`):** `String(step?.ruleName || '')`, `String(step?.controlKind || '')`, and 3 `.includes()` substring checks execute on every 60–120fps render frame even though `step` only changes when `isStepChanged` is true.
  2. **Per-Frame `ALL_FRAME_KINDS.includes()` & `slot.targetY` Recomputation (Lines `671–678`):** `slot.targetY = clamp(topY - chronoIdx * spacing, -2.85, -0.64)` is a constant (`-0.64 - chronoIdx * 0.19`) already set at slot initialization, and `kind` / `semFamily` / `isDeepestActive` / `targetScale` only change when `isStepChanged` is true.
  3. **72 Redundant Trigonometric Calls Per Frame on Constant `targetBadgePitch` (Lines `610` & `701`):** `targetBadgePitch` is a constant `-0.06 * Math.PI`. Yet line `701` executes `slot.plaqueMesh.rotation.x = lerp(slot.plaqueMesh.rotation.x, targetBadgePitch, damp);` for every visible wafer on every frame, triggering Three.js `Euler._onChangeCallback -> Quaternion.setFromEuler` (6 `sin`/`cos` calls per wafer) and marking `plaqueMesh.matrixWorld` dirty forever. Setting `plaqueMesh.rotation.x = -0.06 * Math.PI` once at initialization eliminates this completely.
  4. **Unbounded Asymptotic Lerp Drift & Idle Loop Execution:** Without an epsilon snap (`1e-4`), `slot.currentY`, `slot.currentScale`, `slot.badgeOpacity`, and `slot.flashIntensity` drift in IEEE-754 floating point forever, dirtying `slot.group.position` / `scale` and `slot.rimMat.color` / `emissive` on every frame even when playback is paused. And when `numWafers === 0 && !isUnwinding`, lines `729–734` still loop over all 12 hidden slots every frame.
- **Drop-In Code Snippet (`src/designers/designer3_frames.js`, lines `491–498` and `584–759`):**
```javascript
    // At lines 491-498 in createFramesSubsystem(), set constant pitch once at init:
    plaqueMesh.position.set(
      Math.sin(badgeAzimuth) * BADGE_RADIUS,
      0.018,
      -Math.cos(badgeAzimuth) * BADGE_RADIUS
    );
    plaqueMesh.rotation.set(-0.06 * Math.PI, -badgeAzimuth * 0.65, 0);
    plaqueMesh.matrixAutoUpdate = false;
    plaqueMesh.updateMatrix();
    slotGroup.add(plaqueMesh);
```
```javascript
  // Replace lines 584-759 in src/designers/designer3_frames.js:
  let lastStepIndex = -1;
  let lastTraceRef = null;
  let lastStepRef = null;
  let cachedNumWafers = 0;
  let cachedPrevCount = 0;
  let cachedIsUnwinding = false;
  let cachedControlKind = '';

  const topY = -0.64;
  const spacing = 0.19;

  function update(stepState) {
    if (!stepState) return;

    const dt = clamp(stepState.dt ?? 0.016, 0.001, 0.1);
    const stepFraction = clamp(stepState.stepFraction ?? 0, 0, 1);
    const step = stepState.step;
    const stepIdx = stepState.stepIndex ?? step?.stepIndex ?? 0;
    const isStepChanged =
      stepIdx !== lastStepIndex ||
      stepState.trace !== lastTraceRef ||
      step !== lastStepRef;

    if (isStepChanged) {
      lastStepIndex = stepIdx;
      lastTraceRef = stepState.trace;
      lastStepRef = step;

      const after = step?.after;
      const before = step?.before;
      const rawAfterFrames = Array.isArray(after?.frames) ? after.frames : EMPTY_ARRAY;
      const rawBeforeFrames = Array.isArray(before?.frames) ? before.frames : EMPTY_ARRAY;

      const totalAfter = rawAfterFrames.length;
      cachedNumWafers = Math.min(totalAfter, MAX_WAFERS);
      cachedPrevCount = Math.min(rawBeforeFrames.length, MAX_WAFERS);

      const ruleLine = step?.ruleLine ?? 0;
      const ruleName = step?.ruleName ? String(step.ruleName) : '';
      cachedControlKind = step?.controlKind ? String(step.controlKind) : '';
      cachedIsUnwinding =
        cachedControlKind === 'Return' ||
        cachedControlKind === 'PrimitiveResult' ||
        ruleLine === 153 ||
        ruleLine === 185 ||
        ruleLine === 195 ||
        ruleLine === 80 ||
        ruleName.includes('Return') ||
        ruleName.includes('Binary primitive') ||
        ruleName.includes('Restore scope');

      const rootEnvBindings =
        Array.isArray(after?.envRoots) && after.envRoots[0]?.bindings
          ? after.envRoots[0].bindings
          : EMPTY_ARRAY;

      for (let chronoIdx = 0; chronoIdx < cachedNumWafers; chronoIdx++) {
        const frame = rawAfterFrames[totalAfter - 1 - chronoIdx];
        let resolved =
          Array.isArray(frame.bindings) && frame.bindings.length > 0
            ? frame.bindings
            : null;
        if (!resolved) {
          for (let j = chronoIdx - 1; j >= 0; j--) {
            const parentFrame = rawAfterFrames[totalAfter - 1 - j];
            if (
              Array.isArray(parentFrame?.bindings) &&
              parentFrame.bindings.length > 0
            ) {
              resolved = parentFrame.bindings;
              break;
            }
          }
        }
        const effectiveBindings = resolved || rootEnvBindings;
        effectiveBindingsScratch[chronoIdx] = effectiveBindings;

        const slot = waferSlots[chronoIdx];
        const kind = VALID_FRAME_KINDS.has(frame.kind) ? frame.kind : 'Return';
        const semFamily = getFrameSemanticFamily(kind);
        const isDeepestActive = chronoIdx === cachedNumWafers - 1;

        slot.kind = kind;
        slot.semFamily = semFamily;
        slot.isDeepestActive = isDeepestActive;
        slot.isUnwindTarget = cachedIsUnwinding && isDeepestActive;
        slot.targetScale = isDeepestActive ? 1.0 : 0.97;

        updateWaferPlaqueMeta(
          slot,
          frame,
          chronoIdx + 1,
          effectiveBindings,
          semFamily,
          kind
        );
      }

      for (let chronoIdx = cachedNumWafers; chronoIdx < MAX_WAFERS; chronoIdx++) {
        const slot = waferSlots[chronoIdx];
        if (slot.wasVisible || slot.group.visible) {
          slot.group.visible = false;
          slot.wasVisible = false;
          slot.currentScale = 0.92;
          slot.flashIntensity = 0;
        }
      }
    }

    const numWafers = cachedNumWafers;
    const isUnwinding = cachedIsUnwinding;

    const activePreset = resolveActiveCameraPreset(stepState);
    const showWaferBadges =
      activePreset === 'interior' ||
      activePreset === 'interior_up' ||
      activePreset === 'interior_floor' ||
      activePreset === 'frames';
    const targetBadgeOpacity = showWaferBadges ? 0.96 : 0.0;
    const damp = 1.0 - Math.exp(-10.0 * dt);
    const badgeDamp = Math.min(1.0, damp * 1.6);

    for (let chronoIdx = 0; chronoIdx < numWafers; chronoIdx++) {
      const slot = waferSlots[chronoIdx];
      const semFamily = slot.semFamily;

      let transformDirty = false;
      if (!slot.wasVisible) {
        slot.currentY = clamp(slot.targetY + 0.08, -2.85, -0.58);
        slot.currentScale = 0.92;
        slot.wasVisible = true;
        slot.group.visible = true;
        transformDirty = true;
      }

      if (Math.abs(slot.currentY - slot.targetY) > 1e-4) {
        slot.currentY = lerp(slot.currentY, slot.targetY, damp);
        transformDirty = true;
      } else if (slot.currentY !== slot.targetY) {
        slot.currentY = slot.targetY;
        transformDirty = true;
      }
      slot.lastActiveY = slot.currentY;

      if (Math.abs(slot.currentScale - slot.targetScale) > 1e-4) {
        slot.currentScale = lerp(slot.currentScale, slot.targetScale, damp);
        transformDirty = true;
      } else if (slot.currentScale !== slot.targetScale) {
        slot.currentScale = slot.targetScale;
        transformDirty = true;
      }

      if (transformDirty) {
        slot.group.position.set(0, slot.currentY, 0);
        slot.group.scale.setScalar(slot.currentScale);
        slot.group.updateMatrix();
      }

      if (Math.abs(slot.badgeOpacity - targetBadgeOpacity) > 1e-3) {
        slot.badgeOpacity = lerp(slot.badgeOpacity, targetBadgeOpacity, badgeDamp);
      } else {
        slot.badgeOpacity = targetBadgeOpacity;
      }
      const badgeVisible = slot.badgeOpacity > 0.02;
      slot.plaqueMesh.visible = badgeVisible;
      if (badgeVisible) {
        syncWaferPlaqueTexture(slot);
        slot.plaqueMat.opacity = slot.badgeOpacity;
      }

      const dockFlash = slot.isUnwindTarget
        ? smoothstep(0.15, 0.85, stepFraction)
        : 0;
      if (Math.abs(slot.flashIntensity - dockFlash) > 1e-4) {
        slot.flashIntensity = lerp(slot.flashIntensity, dockFlash, damp);
      } else {
        slot.flashIntensity = dockFlash;
      }

      if (slot.flashIntensity > 1e-4) {
        _colorBlended
          .copy(semFamily.colorObj)
          .lerp(_colorEnvBrightTeal, slot.flashIntensity * 0.65);
        slot.rimMat.color.copy(_colorBlended);
        slot.rimMat.emissive.copy(_colorBlended);
        slot.lastColorHex = -1;
      } else if (slot.lastColorHex !== semFamily.hex) {
        slot.rimMat.color.copy(semFamily.colorObj);
        slot.rimMat.emissive.copy(semFamily.colorObj);
        slot.lastColorHex = semFamily.hex;
      }

      const baseEmissive = showWaferBadges ? 0.34 : 0.14;
      const nextEmissiveIntensity =
        baseEmissive +
        (slot.isDeepestActive ? 0.28 : 0.08) +
        slot.flashIntensity * 0.45;
      if (Math.abs(slot.lastEmissiveIntensity - nextEmissiveIntensity) > 1e-4) {
        slot.rimMat.emissiveIntensity = nextEmissiveIntensity;
        slot.lastEmissiveIntensity = nextEmissiveIntensity;
      }
    }

    // Update Open-Bore Light Filament & Clean Unwind Result Orb (`orbMesh`, radius = 0.046)
    spineMesh.visible = numWafers > 0 || isUnwinding;
    spineMat.opacity = isUnwinding ? 0.28 : 0.14;

    if (
      isUnwinding &&
      (numWafers > 0 || cachedPrevCount > 0 || cachedControlKind === 'Return')
    ) {
      orbMesh.visible = true;
      const popIndex = clamp(
        Math.max(numWafers, cachedPrevCount) - 1,
        0,
        MAX_WAFERS - 1
      );
      const startY = clamp(
        waferSlots[popIndex]?.lastActiveY ?? (topY - popIndex * spacing),
        -2.85,
        -0.64
      );

      const orbT = smoothstep(0.0, 0.95, stepFraction);
      const orbY = lerp(startY, CONTROL_CORE_DOCK_Y, orbT);

      orbMesh.position.set(0, orbY, 0);
      orbMesh.updateMatrix();
      orbMat.emissiveIntensity = 1.2 + 0.35 * Math.sin(orbT * Math.PI);
    } else {
      orbMesh.visible = false;
    }
  }
```

---

### 2.2 `designer4_arena.js` — Dirty-Check Tile Transforms (`lastZ`) and Material States (`lastStyleKey`)
- **Location:** `/home/charles/sherlock-bench/src/designers/designer4_arena.js`, lines `425–486`
- **Problem:**
  On every step change, `update()` unconditionally sets `tile.group.position.set(0, 0, targetZ)` (dirtying transform matrices even when `count` is constant across the whole trace) and overwrites `bodyMat.color`, `edgeMat.color`, `edgeMat.emissive`, and `edgeMat.emissiveIntensity` on all visible tiles even when 4–5 of the 6 tiles did not change status on that step.
- **Drop-In Code Snippet (`src/designers/designer4_arena.js`, replacing lines `425–463`):**
```javascript
    for (let i = 0; i < MAX_CALL_TILES; i += 1) {
      const tile = callTiles[i];
      if (i >= count) {
        tile.group.visible = false;
        continue;
      }

      tile.group.visible = true;
      const call = visibleCalls[i];
      const targetZ = startZ + i * spacing;
      if (tile.lastZ !== targetZ) {
        tile.lastZ = targetZ;
        tile.group.position.set(0, 0, targetZ);
        tile.group.updateMatrix();
      }

      const ringNum = resolveCallRingNumber(call);
      const ringTag = `R${ringNum}`;
      const retVal = call.status === 'resolved' ? call.returnValue || 'ok' : '';

      const styleKey =
        call.status === 'active' && isError ? 'fault' : call.status;
      if (tile.lastStyleKey !== styleKey) {
        tile.lastStyleKey = styleKey;
        if (styleKey === 'resolved') {
          tile.bodyMat.color.setHex(HEX_TILE_RETURNED);
          tile.edgeMat.color.setHex(HEX_RIM_RETURNED);
          tile.edgeMat.emissive.setHex(HEX_RIM_RETURNED);
          tile.edgeMat.emissiveIntensity = 0.72;
        } else if (styleKey === 'active') {
          tile.bodyMat.color.setHex(HEX_TILE_ACTIVE);
          tile.edgeMat.color.setHex(HEX_RIM_ACTIVE);
          tile.edgeMat.emissive.setHex(HEX_RIM_ACTIVE);
          tile.edgeMat.emissiveIntensity = 0.90;
        } else if (styleKey === 'fault') {
          tile.bodyMat.color.setHex(HEX_TILE_FAULT);
          tile.edgeMat.color.setHex(PALETTE.errorRed);
          tile.edgeMat.emissive.setHex(PALETTE.errorRed);
          tile.edgeMat.emissiveIntensity = 0.90;
        } else if (styleKey === 'waiting') {
          tile.bodyMat.color.setHex(HEX_TILE_IDLE);
          tile.edgeMat.color.setHex(HEX_RIM_WAITING);
          tile.edgeMat.emissive.setHex(HEX_RIM_WAITING);
          tile.edgeMat.emissiveIntensity = 0.38;
        } else {
          tile.bodyMat.color.setHex(HEX_TILE_IDLE);
          tile.edgeMat.color.setHex(HEX_RIM_IDLE);
          tile.edgeMat.emissive.setHex(0x000000);
          tile.edgeMat.emissiveIntensity = 0.0;
        }
      }
```

---

## Priority 3 (Medium): Zero-Allocation Discipline & Pre-Compiled Regexes

### 3.1 `designer4_arena.js` — Eliminate Per-Step Wrapper Object & String Key Allocations, and Pre-Compile 15 Regexes
- **Location:** `/home/charles/sherlock-bench/src/designers/designer4_arena.js`, lines `33–87` and `169–226`
- **Problem:**
  1. `evaluateCallTilesAtStep(stepState)` (lines `222–225`) allocates a new `{ visibleCalls: _visibleCallsScratch, isError }` object on every step transition.
  2. `getOrBuildTraceCalls(trace)` (line `84`) allocates a template string ``const traceKey = `${trace?.programName || ''}:${steps.length}`;`` on every step even when `trace === _cachedTraceRef` and `steps.length === _cachedStepsLen`.
  3. `compactCallLabel` (lines `33–58`) and `compactValueAtom` (lines `60–71`) execute 15 inline regex replacements unconditionally—including 6 complex nested `branch(...)` / `leaf(...)` / `cons(...)` regexes even on simple calls like `fact(4)` or `fib(3)` that contain none of those constructors.
- **Drop-In Code Snippet (`src/designers/designer4_arena.js`, replacing lines `33–87` and `169–226`):**
```javascript
// Replace lines 33-87 in src/designers/designer4_arena.js:
const RE_INVOKE_WRAP = /^Invoke\((.*)\)$/i;
const RE_ENTER_WRAP = /^Enter\((.*)\)$/i;
const RE_RETURN_WRAP = /^Return\((.*)\)$/i;
const RE_PRIMOK_WRAP = /^PrimOk\((.*)\)$/i;
const RE_DONE_WRAP = /^Done\((.*)\)$/i;
const RE_ADT_HINT = /branch|leaf|cons/i;
const RE_BRANCH_3 =
  /branch\s*\(\s*leaf\((\d+)\)\s*,\s*branch\s*\(\s*leaf\((\d+)\)\s*,\s*leaf\((\d+)\)\s*\)\s*\)/gi;
const RE_BRANCH_2 = /branch\s*\(\s*leaf\((\d+)\)\s*,\s*leaf\((\d+)\)\s*\)/gi;
const RE_LEAF = /leaf\s*\(\s*(\d+)\s*\)/gi;
const RE_CONS_3 =
  /cons\s*\(\s*(\d+)\s*,\s*cons\s*\(\s*(\d+)\s*,\s*cons\s*\(\s*(\d+)\s*,\s*nil\s*\)\s*\)\s*\)/gi;
const RE_CONS_2 = /cons\s*\(\s*(\d+)\s*,\s*cons\s*\(\s*(\d+)\s*,\s*nil\s*\)\s*\)/gi;
const RE_CONS_1 = /cons\s*\(\s*(\d+)\s*,\s*nil\s*\)/gi;
const RE_WHITESPACE = /\s+/g;

function compactCallLabel(rawLabel) {
  if (!rawLabel) return 'call()';
  let s = String(rawLabel)
    .replace(RE_INVOKE_WRAP, '$1')
    .replace(RE_ENTER_WRAP, '$1')
    .replace(RE_RETURN_WRAP, '$1')
    .trim();

  if (RE_ADT_HINT.test(s)) {
    s = s
      .replace(RE_BRANCH_3, 'B($1,B($2,$3))')
      .replace(RE_BRANCH_2, 'B($1,$2)')
      .replace(RE_LEAF, 'L($1)')
      .replace(RE_CONS_3, '[$1,$2,$3]')
      .replace(RE_CONS_2, '[$1,$2]')
      .replace(RE_CONS_1, '[$1]');
  }
  s = s.replace(RE_WHITESPACE, '');
  if (s.length > 16) {
    s = `${s.slice(0, 14)}..`;
  }
  return s;
}

function compactValueAtom(raw) {
  if (raw === null || raw === undefined) return '';
  let s = String(raw)
    .replace(RE_RETURN_WRAP, '$1')
    .replace(RE_PRIMOK_WRAP, '$1')
    .replace(RE_DONE_WRAP, '$1')
    .trim();
  if (RE_ADT_HINT.test(s)) {
    s = s.replace(RE_BRANCH_2, 'B($1,$2)').replace(RE_LEAF, 'L($1)');
  }
  if (s.length > 12) s = `${s.slice(0, 10)}..`;
  return s;
}

function resolveCallRingNumber(call) {
  const rawRing = (call.depth > 0 ? call.depth + 1 : call.index) || 1;
  return clamp(rawRing, 1, 6);
}

let _cachedTraceRef = null;
let _cachedProgramName = '';
let _cachedStepsLen = -1;
let _cachedAllCalls = [];

function getOrBuildTraceCalls(trace) {
  const steps = Array.isArray(trace?.steps) ? trace.steps : [];
  const progName = trace?.programName || '';
  if (
    trace &&
    trace === _cachedTraceRef &&
    progName === _cachedProgramName &&
    steps.length === _cachedStepsLen
  ) {
    return _cachedAllCalls;
  }
```
And at lines `163–167` & `169–226`:
```javascript
  _cachedTraceRef = trace;
  _cachedProgramName = progName;
  _cachedStepsLen = steps.length;
  _cachedAllCalls = allCalls;
  return allCalls;
}

const _visibleCallsScratch = [];
const _evalResultScratch = {
  visibleCalls: _visibleCallsScratch,
  isError: false,
};

function evaluateCallTilesAtStep(stepState) {
  // ... (lines 172-220 unchanged) ...
  _evalResultScratch.visibleCalls = _visibleCallsScratch;
  _evalResultScratch.isError = isError;
  return _evalResultScratch;
}
```

---

## Priority 4 (Medium): Static Transform Freezing (`matrixAutoUpdate = false`), Raycast Pruning & Shadow Flags

### 4.1 `designer3_frames.js` — Freeze All 89 Scene-Graph Node Matrices (`matrixAutoUpdate = false`) & Prune Redundant Child Raycasts
- **Location:** `/home/charles/sherlock-bench/src/designers/designer3_frames.js`, lines `268–270`, `386–405`, and `421–498`
- **Problem:**
  1. `group`, `wafersContainer`, `spineMesh`, and all `12 × 6 = 72` child meshes inside `SavedFrameWafer_0..11` have static local transforms, yet `matrixAutoUpdate` is left `true` on all of them.
  2. When Designer 7's raycaster calls `raycaster.intersectObjects(scene.children, true)`, it tests every ray against `outerLipMesh` and `lowerLipMesh` (256 triangles per wafer) even though they are paper-thin (`0.012` wide) decorative accent strips flush on the surface of `glassMesh` / `outerBevelMesh` on the same `slotGroup`.
  3. Shared geometries (`waferOuterBevelGeo`, `waferGlassGeo`, `waferInnerCollarGeo`, `waferPlaqueGeo`) do not pre-compute bounding spheres/boxes at initialization, deferring lazy computation to the first raycast/reticle frame.
- **Drop-In Code Snippet (`src/designers/designer3_frames.js`, lines `268–276`, `386–405`, `421–460`):**
```javascript
  const NOOP_RAYCAST = () => {};

  const group = new THREE.Group();
  group.name = 'Designer3_ContinuationWafers';
  group.matrixAutoUpdate = false;

  const disposableResources = [];

  function trackGeo(geo) {
    geo.computeBoundingBox();
    geo.computeBoundingSphere();
    disposableResources.push(geo);
    return geo;
  }
```
```javascript
  // Lines 386-405 (spineMesh, orbMesh, wafersContainer):
  const spineMesh = new THREE.Mesh(spineGeo, spineMat);
  spineMesh.name = 'OpenBoreLightFilament';
  spineMesh.position.set(0, -1.80, 0);
  spineMesh.matrixAutoUpdate = false;
  spineMesh.updateMatrix();
  spineMesh.raycast = NOOP_RAYCAST;
  group.add(spineMesh);

  const orbMesh = new THREE.Mesh(orbGeo, orbMat);
  orbMesh.name = 'UnwindResultOrb';
  orbMesh.visible = false;
  orbMesh.matrixAutoUpdate = false;
  group.add(orbMesh);
  // ...
  const wafersContainer = new THREE.Group();
  wafersContainer.name = 'SavedFrameWafersStack';
  wafersContainer.matrixAutoUpdate = false;
  group.add(wafersContainer);
```
```javascript
  // Inside the for (let i = 0; i < MAX_WAFERS; i++) loop (lines 428-460):
    const slotGroup = new THREE.Group();
    slotGroup.name = `SavedFrameWafer_${i}`;
    slotGroup.visible = false;
    slotGroup.matrixAutoUpdate = false;
    wafersContainer.add(slotGroup);

    const glassMesh = new THREE.Mesh(waferGlassGeo, sharedGlassMat);
    glassMesh.matrixAutoUpdate = false;
    slotGroup.add(glassMesh);

    const outerBevelMesh = new THREE.Mesh(waferOuterBevelGeo, sharedOuterBevelMat);
    outerBevelMesh.matrixAutoUpdate = false;
    slotGroup.add(outerBevelMesh);

    // ... rimMat ...
    const innerCollarMesh = new THREE.Mesh(waferInnerCollarGeo, rimMat);
    innerCollarMesh.matrixAutoUpdate = false;
    slotGroup.add(innerCollarMesh);

    const outerLipMesh = new THREE.Mesh(outerAccentLipGeo, rimMat);
    outerLipMesh.matrixAutoUpdate = false;
    outerLipMesh.raycast = NOOP_RAYCAST;
    slotGroup.add(outerLipMesh);

    const lowerLipMesh = new THREE.Mesh(lowerAccentLipGeo, rimMat);
    lowerLipMesh.matrixAutoUpdate = false;
    lowerLipMesh.raycast = NOOP_RAYCAST;
    slotGroup.add(lowerLipMesh);
```

---

### 4.2 `designer4_arena.js` — Freeze All 26 Scene-Graph Node Matrices (`matrixAutoUpdate = false`), Prune Decal Raycast, and Downgrade Unused `MeshPhysicalMaterial` to `MeshStandardMaterial`
- **Location:** `/home/charles/sherlock-bench/src/designers/designer4_arena.js`, lines `290–376`
- **Problem:**
  1. `group`, `tileRowGroup`, all 6 `tileGroup`s, and all `6 × 3 = 18` child meshes (`bodyMesh`, `edgeMesh`, `plaqueMesh`) have `matrixAutoUpdate = true` even though their local transforms are static (and `tileGroup` only moves on `targetZ !== tile.lastZ`).
  2. `bodyMat` (line `332`) instantiates `THREE.MeshPhysicalMaterial({ color: HEX_TILE_IDLE, roughness: 0.42, metalness: 0.22 })` without using any physical features (`clearcoat`, `transmission`, `sheen`, `iridescence`). Using `THREE.MeshStandardMaterial` produces 100% identical metallic-roughness PBR shading while avoiding physical-shader uniform/pipeline overhead.
  3. `plaqueMesh` sits `0.002` above `bodyMesh` (`0.72 × 0.44` footprint) on the same `tileGroup`; setting `plaqueMesh.raycast = NOOP_RAYCAST` skips 6 decal plane raycast checks per mouse move while `bodyMesh` and `edgeMesh` handle inspection hits identically.
- **Drop-In Code Snippet (`src/designers/designer4_arena.js`, lines `290–376`):**
```javascript
  const NOOP_RAYCAST = () => {};

  const group = new THREE.Group();
  group.name = 'Designer4_WestFlankCallTiles';
  group.matrixAutoUpdate = false;
  // ...
  const trackGeo = (g) => {
    g.computeBoundingBox();
    g.computeBoundingSphere();
    disposableGeometries.push(g);
    return g;
  };
  // ...
  const tileRowGroup = new THREE.Group();
  tileRowGroup.name = 'WestFlankCallTileRow';
  tileRowGroup.position.set(WEST_FLANK_X, GROUND_TILE_Y, 0);
  tileRowGroup.matrixAutoUpdate = false;
  tileRowGroup.updateMatrix();
  group.add(tileRowGroup);
  // ...
  for (let i = 0; i < MAX_CALL_TILES; i += 1) {
    const tileGroup = new THREE.Group();
    tileGroup.name = `CallTile_${i}`;
    tileGroup.matrixAutoUpdate = false;
    tileRowGroup.add(tileGroup);

    const bodyMat = trackMat(
      new THREE.MeshStandardMaterial({
        color: HEX_TILE_IDLE,
        roughness: 0.42,
        metalness: 0.22,
      })
    );
    const bodyMesh = new THREE.Mesh(tileBodyGeo, bodyMat);
    bodyMesh.castShadow = true;
    bodyMesh.receiveShadow = true;
    bodyMesh.matrixAutoUpdate = false;
    tileGroup.add(bodyMesh);

    const edgeMat = trackMat(
      new THREE.MeshStandardMaterial({
        color: HEX_RIM_IDLE,
        emissive: 0x000000,
        emissiveIntensity: 0,
        roughness: 0.38,
        metalness: 0.35,
      })
    );
    const edgeMesh = new THREE.Mesh(tileEdgeGeo, edgeMat);
    edgeMesh.position.y = -0.024;
    edgeMesh.receiveShadow = true;
    edgeMesh.matrixAutoUpdate = false;
    edgeMesh.updateMatrix();
    tileGroup.add(edgeMesh);
    // ...
    const plaqueMesh = new THREE.Mesh(tilePlaqueGeo, plaqueMat);
    plaqueMesh.rotation.x = -Math.PI / 2;
    plaqueMesh.position.y = tileHeight * 0.5 + 0.002;
    plaqueMesh.matrixAutoUpdate = false;
    plaqueMesh.updateMatrix();
    plaqueMesh.raycast = NOOP_RAYCAST;
    tileGroup.add(plaqueMesh);
```
