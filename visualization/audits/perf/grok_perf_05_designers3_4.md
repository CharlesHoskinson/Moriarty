# Performance Audit #5 — Designer 3 Semantic Frames & Designer 4 Wafer Arena

Auditor: Grok 4.7 xhigh Performance Auditor #5
Scope: `src/designers/designer3_frames.js` (778 lines) and `src/designers/designer4_arena.js` (509 lines)
Engine: Three.js `0.175.0` via `three/webgpu` (`WebGPURenderer`, node materials)
Constraint: every change below keeps the same pixels, the same inspect targets, and the same shadow contact on the west-flank chiplets. No source file was modified by this audit.

Line numbers refer to the files as read on 2026-10-18.

---

## 1. Verdict

Paused playback still walks about **114 `Object3D` local matrices every frame** (88 in Designer 3, 26 in Designer 4) even though most of those transforms never change. Three.js r175 composes `position/quaternion/scale` into `matrix` whenever `matrixAutoUpdate` is true (`node_modules/three/src/core/Object3D.js` lines 1144–1162 and 1176–1213). `Scene.matrixAutoUpdate` stays true, so `force` still reaches every descendant and world matrices are still multiplied. Freezing **local** matrices is the safe, pixel-identical win. Leave `matrixWorldAutoUpdate` at its default `true` on anything parented under a group that moves (`slotGroup` Y/scale, `tileGroup` Z, the unwind orb).

Heap traffic on the steady frame is already in good shape. Designer 3 does not allocate inside `update()` while the step index and trace reference are stable. Designer 4 returns before doing any work in that same case. The remaining allocations fire on **step boundaries** (fast scrub and playback at ~3 steps/s) and at **startup** (12 plaque canvases that are never shown).

| ID | File:lines | What changes | When it pays | Pixels |
|---|---|---|---|---|
| D3-1 | `designer3_frames.js` 386–405, 427–497, 668–735 | `matrixAutoUpdate = false` + `updateMatrix()` only when a local transform changes | Every frame, including pause | Same |
| D3-2 | 668–735 | Snap exponential tails (1e-4 world units, 1e-3 opacity) so settled wafers stop writing matrices and colors | Pause, and any slot whose target is stable | Same (settled pose is the authored target) |
| D3-3 | 469–489, 530–557 | One shared placeholder plaque; value-compare before `formatWaferPlaqueTitle` / `createPlaqueTexture`; drop `plaqueMat.needsUpdate = true` | Startup, and every step whose badge text is unchanged | Same |
| D3-4 | 647–660, 708–718, 737–757 | Cache unwind classification per step; write rim/spine/orb uniforms only when the value changes | Every frame | Same |
| D3-5 | 441–459 | Merge the three co-transformed rim geometries that already share `rimMat` | Draw + raycast count, init once | Same |
| D4-1 | `designer4_arena.js` 311–375, 425–435 | Bake static locals; `updateMatrix()` on a tile group only when `position.z` changes | Every frame between steps (the common case; `update` already returns at 409–415) | Same |
| D4-2 | 243–284, 357–361, 464–468 | Keep the 6-canvas pool; add a signature → texture cache so a repeated face is not rasterized or reuploaded | Scrub back to a status already painted | Same |
| D4-3 | 78–86, 200–225, 437–468 | Step-change path: no trace-key string, no result object, no `` `R${n}` `` / signature strings, no redundant `setHex` | Every step index change | Same |

---

## 2. Already correct — do not regress

These are load-bearing. A later edit that “simplifies” them will either allocate per frame or change the picture.

### Designer 3

- Module-scope colors `_colorEnvReturnTeal` … `_colorBlended` (lines 68–75). `update()` blends with `.copy` / `.lerp` on `_colorBlended`. No `new THREE.Color()` on the hot path.
- One extruded geometry per ring band, shared by all 12 wafers: `waferOuterBevelGeo`, `waferGlassGeo`, `waferInnerCollarGeo`, `outerAccentLipGeo`, `lowerAccentLipGeo`, `waferPlaqueGeo` (lines 288–341). `curveSegments: 64` is shared, not per wafer.
- One `sharedGlassMat` and one `sharedOuterBevelMat` (lines 343–371). `createBrushedMetalTexture` (lines 359–363) is init-once and LRU-cached in `theme.js` lines 144–207. The `'#d8dde6'` request is remapped inside `theme.js` lines 149–152 to the high-key base so `color * map` does not square-darken. Leave that call alone.
- Plaque rebuilds are already gated twice:
  - `isStepChanged` (lines 596–598) wraps `updateWaferPlaque` (lines 720–728), so camera-preset fades and the unwind orb still run while the step is held.
  - `slot.lastSignature` (lines 538–540) skips `createPlaqueTexture` when the formatted badge is unchanged. `theme.js` `createPlaqueTexture` (lines 329–420) then returns the same `CanvasTexture` on a hit.
  - Lines 554–557 swap `plaqueMat.map` only when the texture object identity changes.
- `effectiveBindingsScratch` (line 582) and `EMPTY_ARRAY` (line 583) avoid per-frame arrays. `resolveActiveCameraPreset` (lines 251–255) does not allocate.
- `spineMesh.raycast = () => {}` (line 389) keeps the filament out of Designer 7’s raycaster.
- `dispose()` (lines 764–770) does not dispose plaque or rhodium textures. `theme.js` lines 412–417 also refuse to dispose LRU evictions because those textures may still be mounted. Keep both behaviors. Disposing a shared brush or plaque texture blanks every other designer that holds it.

### Designer 4

- Six canvases are created once in `createCallTileCanvasPlaque` (lines 228–240, called at 357) and repainted in place. `update()` does not call `document.createElement('canvas')`.
- Lines 409–415 return immediately when `stepIndex`, trace reference, and `phase` are unchanged. Tile faces do not depend on `stepFraction` or `dt`, so this early-out is visually complete. It does **not** stop Three.js from recomposing matrices; D4-1 does.
- `getOrBuildTraceCalls` (lines 82–167) rebuilds the call list once per trace. `_visibleCallsScratch` (lines 169, 178, 209–211) is reused.
- `paintCleanCallTile` runs only when `sig !== tile.lastSignature` (lines 464–468).
- Geometries are shared: `tileBodyGeo`, `tileEdgeGeo`, `tilePlaqueGeo` (lines 320–322).
- Only `bodyMesh` casts a shadow (line 339). Edge and plaque do not. Keep the body caster; the key light’s frustum in `SherlockStage.js` lines 156–159 is sized to include `WEST_FLANK_X = -2.88`.

### Stage facts these snippets rely on

- `SherlockStage.root` is never transformed (`src/stage/SherlockStage.js` lines 110–111, 220). Designer groups are parented there.
- `render()` calls `sub.update(fullState)` and then `renderer.render` (lines 353–363). A texture swapped inside `update()` is what the first frame shows. Wafers start at `slotGroup.visible = false` (line 430), so an init placeholder is never rasterized into a presented frame.
- `app.js` `getOrBuildStepState` (lines 910–928) reuses one step object while the index is held and only mutates `stepFraction`, `dt`, and progress. Designer 3’s `isStepChanged` test matches that.

---

## 3. Designer 3 — `src/designers/designer3_frames.js`

### D3-1. Freeze static local matrices

**Why.** `updateMatrixWorld` does not look at `visible`. A hidden wafer and the five rim/glass/bevel children under a moving slot still run `matrix.compose(position, quaternion, scale)` every frame while `matrixAutoUpdate` is true. Those children are at a constant local transform after init (glass, rhodium bevel, collar, both lips). The plaque’s `position` and `rotation.y` are constant; only `rotation.x` animates (line 701). The slot group’s only motion is `position.y` and uniform scale (lines 694–695). The spine is placed once (line 388). The root group and `wafersContainer` stay at identity.

**Invariant.** Call `updateMatrix()` after the last local write, then set `matrixAutoUpdate = false`. Every later local write calls `updateMatrix()` again. Do not set `matrixWorldAutoUpdate = false` on these nodes: a moving `slotGroup` must still concatenate into its children, and the scene root passes `force = true` every frame (`Object3D.js` 1180–1210). With the flag left `true`, children follow the parent. With the flag set `false`, they stay at the baked world matrix and the rings freeze in place.

**Add** next to `trackMat` (after line 281):

```javascript
function bakeStatic(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
  return obj;
}
```

**Spine** — replace lines 386–390:

```javascript
const spineMesh = new THREE.Mesh(spineGeo, spineMat);
spineMesh.name = 'OpenBoreLightFilament';
spineMesh.position.set(0, -1.80, 0);
spineMesh.raycast = () => {};
bakeStatic(spineMesh);
group.add(spineMesh);
```

**Orb** — replace lines 402–405. The orb moves, so it stays on a manual matrix (D3-4), not a one-shot bake:

```javascript
const orbMesh = new THREE.Mesh(orbGeo, orbMat);
orbMesh.name = 'UnwindResultOrb';
orbMesh.visible = false;
orbMesh.matrixAutoUpdate = false;
group.add(orbMesh);
```

**Per-slot meshes** — after each `slotGroup.add(...)` for the five static meshes (lines 434–459), bake them. One block:

```javascript
const glassMesh = new THREE.Mesh(waferGlassGeo, sharedGlassMat);
slotGroup.add(glassMesh);
bakeStatic(glassMesh);

const outerBevelMesh = new THREE.Mesh(waferOuterBevelGeo, sharedOuterBevelMat);
slotGroup.add(outerBevelMesh);
bakeStatic(outerBevelMesh);

const rimMat = trackMat(
  new THREE.MeshStandardMaterial({
    color: HEX_OP_LEFT_AMBER,
    emissive: HEX_OP_LEFT_AMBER,
    emissiveIntensity: 0.28,
    roughness: 0.24,
    metalness: 0.84,
    side: THREE.DoubleSide,
  })
);
const innerCollarMesh = new THREE.Mesh(waferInnerCollarGeo, rimMat);
slotGroup.add(innerCollarMesh);
bakeStatic(innerCollarMesh);

const outerLipMesh = new THREE.Mesh(outerAccentLipGeo, rimMat);
slotGroup.add(outerLipMesh);
bakeStatic(outerLipMesh);

const lowerLipMesh = new THREE.Mesh(lowerAccentLipGeo, rimMat);
slotGroup.add(lowerLipMesh);
bakeStatic(lowerLipMesh);
```

**Plaque** — replace lines 490–497. `rotation.x` is updated later, so the bake captures position and `rotation.y` only:

```javascript
const plaqueMesh = new THREE.Mesh(waferPlaqueGeo, plaqueMat);
plaqueMesh.position.set(
  Math.sin(badgeAzimuth) * BADGE_RADIUS,
  0.018,
  -Math.cos(badgeAzimuth) * BADGE_RADIUS
);
plaqueMesh.rotation.y = -badgeAzimuth * 0.65;
bakeStatic(plaqueMesh);
slotGroup.add(plaqueMesh);
```

**Slot group** — replace lines 428–431:

```javascript
const slotGroup = new THREE.Group();
slotGroup.name = `SavedFrameWafer_${i}`;
slotGroup.visible = false;
slotGroup.matrixAutoUpdate = false;
wafersContainer.add(slotGroup);
```

**Containers** — after the wafer loop, before `updateWaferPlaque` (after line 525):

```javascript
bakeStatic(wafersContainer);
bakeStatic(group);
```

`SherlockStage` only writes `group.visible` and `group.name` (`SherlockStage.js` 219–220 and 239–241). Those do not need a local matrix rebuild.

---

### D3-2 + D3-4. Settled motion and uniform writes inside `update()`

**Why.** Lines 688–718 run for every visible wafer on every animation frame:

- `position.set` / `scale.setScalar` dirty the local matrix even when the values have stopped moving in any visible way.
- `lerp` with `damp = 1 - exp(-10*dt)` never reaches the target, so the writes never stop.
- `_colorBlended` is copied onto `rimMat.color` and `rimMat.emissive` even when family and flash are unchanged (lines 708–718).
- `ruleName.includes(...)` (lines 649–660) is recomputed from step-invariant fields on every frame.
- The hidden branch (lines 729–734) stores the same four fields on every frame for wafers that are already hidden.
- `spineMesh.visible`, `spineMat.opacity`, and `orbMat.emissiveIntensity` are stored even when unchanged (lines 738–755).

Snapping to the authored target once the remaining error is below `1e-4` world units (0.1 mm) or `1e-3` opacity reaches the value the lerp was converging to. Below that threshold the exponential tail is not a visible motion. `showWaferBadges` still switches `emissiveIntensity` immediately between `0.34` and `0.14` (line 714). Keep that pop; it is the current look.

**Closure state** — extend lines 582–585:

```javascript
const effectiveBindingsScratch = new Array(MAX_WAFERS);
const EMPTY_ARRAY = [];
const TARGET_BADGE_PITCH = -0.06 * Math.PI;
const TOP_Y = -0.64;
const WAFER_SPACING = 0.19;
const SETTLE_POS = 1e-4;
const SETTLE_OPACITY = 1e-3;
let lastStepIndex = -1;
let lastTraceRef = null;
let cachedUnwinding = false;
```

**Slot record** — extend the object pushed at lines 508–524:

```javascript
waferSlots.push({
  slotIndex: i,
  slotId,
  group: slotGroup,
  rimMat,
  plaqueMat,
  plaqueMesh,
  badgeOpacity: 0.96,
  lastSignature: initSig,
  inspectSignature: '',
  currentY: initialY,
  targetY: initialY,
  lastActiveY: initialY,
  currentScale: 1.0,
  wasVisible: false,
  flashIntensity: 0,
  cacheSem: null,
  cacheKind: null,
  cacheLabel: null,
  cacheAst: null,
  cacheEnv: null,
  cacheBindName: null,
  cacheBindVal: null,
  appliedFamily: null,
  appliedFlash: -1,
  appliedEmissive: -1,
  detailsScratch: ['', '', '', '', ''],
});
```

**Replace lines 647–758** (unwind detection through the orb) with:

```javascript
if (isStepChanged) {
  const ruleLine = step?.ruleLine ?? 0;
  const ruleName = step?.ruleName || '';
  const controlKind = step?.controlKind || '';
  cachedUnwinding =
    controlKind === 'Return' ||
    controlKind === 'PrimitiveResult' ||
    ruleLine === 153 ||
    ruleLine === 185 ||
    ruleLine === 195 ||
    ruleLine === 80 ||
    ruleName.includes('Return') ||
    ruleName.includes('Binary primitive') ||
    ruleName.includes('Restore scope');
}
const isUnwinding = cachedUnwinding;

const damp = 1.0 - Math.exp(-10.0 * dt);

for (let chronoIdx = 0; chronoIdx < MAX_WAFERS; chronoIdx++) {
  const slot = waferSlots[chronoIdx];

  if (chronoIdx < numWafers) {
    const frame = rawAfterFrames[totalAfter - 1 - chronoIdx];
    const kind = ALL_FRAME_KINDS.includes(frame.kind) ? frame.kind : 'Return';
    const semFamily = getFrameSemanticFamily(kind);
    const depthNumber = chronoIdx + 1;
    const isDeepestActive = chronoIdx === numWafers - 1;
    const isUnwindTarget = isUnwinding && isDeepestActive;

    slot.targetY = clamp(TOP_Y - chronoIdx * WAFER_SPACING, -2.85, -0.64);

    if (!slot.wasVisible) {
      slot.currentY = clamp(slot.targetY + 0.08, -2.85, -0.58);
      slot.currentScale = 0.92;
      slot.wasVisible = true;
    }

    if (!slot.group.visible) slot.group.visible = true;

    let nextY = lerp(slot.currentY, slot.targetY, damp);
    if (Math.abs(slot.targetY - nextY) < SETTLE_POS) nextY = slot.targetY;
    let nextScale = lerp(slot.currentScale, isDeepestActive ? 1.0 : 0.97, damp);
    if (Math.abs((isDeepestActive ? 1.0 : 0.97) - nextScale) < SETTLE_POS) {
      nextScale = isDeepestActive ? 1.0 : 0.97;
    }

    let transformDirty = false;
    if (slot.currentY !== nextY) {
      slot.currentY = nextY;
      slot.lastActiveY = nextY;
      slot.group.position.y = nextY;
      transformDirty = true;
    } else {
      slot.lastActiveY = slot.currentY;
    }
    if (slot.currentScale !== nextScale) {
      slot.currentScale = nextScale;
      slot.group.scale.setScalar(nextScale);
      transformDirty = true;
    }
    if (transformDirty) slot.group.updateMatrix();

    let nextOpacity = lerp(slot.badgeOpacity, targetBadgeOpacity, damp * 1.6);
    if (Math.abs(targetBadgeOpacity - nextOpacity) < SETTLE_OPACITY) {
      nextOpacity = targetBadgeOpacity;
    }
    if (slot.badgeOpacity !== nextOpacity) {
      slot.badgeOpacity = nextOpacity;
      slot.plaqueMat.opacity = nextOpacity;
      const showBadge = nextOpacity > 0.02;
      if (slot.plaqueMesh.visible !== showBadge) slot.plaqueMesh.visible = showBadge;
    }

    let nextPitch = lerp(slot.plaqueMesh.rotation.x, TARGET_BADGE_PITCH, damp);
    if (Math.abs(TARGET_BADGE_PITCH - nextPitch) < SETTLE_POS) nextPitch = TARGET_BADGE_PITCH;
    if (slot.plaqueMesh.rotation.x !== nextPitch) {
      slot.plaqueMesh.rotation.x = nextPitch;
      slot.plaqueMesh.updateMatrix();
    }

    const dockFlash = isUnwindTarget ? smoothstep(0.15, 0.85, stepFraction) : 0;
    let nextFlash = lerp(slot.flashIntensity, dockFlash, damp);
    if (Math.abs(dockFlash - nextFlash) < SETTLE_OPACITY) nextFlash = dockFlash;
    slot.flashIntensity = nextFlash;

    const emissiveIntensity =
      (showWaferBadges ? 0.34 : 0.14) +
      (isDeepestActive ? 0.28 : 0.08) +
      slot.flashIntensity * 0.45;

    if (slot.appliedFamily !== semFamily || slot.appliedFlash !== slot.flashIntensity) {
      _colorBlended.copy(semFamily.colorObj);
      if (slot.flashIntensity > 0) {
        _colorBlended.lerp(_colorEnvBrightTeal, slot.flashIntensity * 0.65);
      }
      slot.rimMat.color.copy(_colorBlended);
      slot.rimMat.emissive.copy(_colorBlended);
      slot.appliedFamily = semFamily;
      slot.appliedFlash = slot.flashIntensity;
    }
    if (slot.appliedEmissive !== emissiveIntensity) {
      slot.rimMat.emissiveIntensity = emissiveIntensity;
      slot.appliedEmissive = emissiveIntensity;
    }

    if (isStepChanged) {
      updateWaferPlaque(
        slot,
        frame,
        depthNumber,
        effectiveBindingsScratch[chronoIdx],
        semFamily
      );
    }
  } else if (slot.group.visible || slot.wasVisible) {
    slot.group.visible = false;
    slot.wasVisible = false;
    slot.currentScale = 0.92;
    slot.flashIntensity = 0;
    slot.appliedFlash = -1;
  }
}

const spineVisible = numWafers > 0 || isUnwinding;
if (spineMesh.visible !== spineVisible) spineMesh.visible = spineVisible;
const spineOpacity = isUnwinding ? 0.28 : 0.14;
if (spineMat.opacity !== spineOpacity) spineMat.opacity = spineOpacity;

if (isUnwinding && (numWafers > 0 || prevCount > 0 || (step?.controlKind || '') === 'Return')) {
  if (!orbMesh.visible) orbMesh.visible = true;
  const popIndex = clamp(Math.max(numWafers, prevCount) - 1, 0, MAX_WAFERS - 1);
  const startY = clamp(
    waferSlots[popIndex]?.lastActiveY ?? (TOP_Y - popIndex * WAFER_SPACING),
    -2.85,
    -0.64
  );
  const orbT = smoothstep(0.0, 0.95, stepFraction);
  const orbY = lerp(startY, CONTROL_CORE_DOCK_Y, orbT);
  if (orbMesh.position.y !== orbY || orbMesh.position.x !== 0 || orbMesh.position.z !== 0) {
    orbMesh.position.set(0, orbY, 0);
    orbMesh.updateMatrix();
  }
  const orbGlow = 1.2 + 0.35 * Math.sin(orbT * Math.PI);
  if (orbMat.emissiveIntensity !== orbGlow) orbMat.emissiveIntensity = orbGlow;
} else if (orbMesh.visible) {
  orbMesh.visible = false;
}
```

`position.x` and `position.z` on a slot stay 0, which is the value `position.set(0, y, 0)` writes today (line 694). `lastActiveY` still tracks the displayed Y so the orb launch (old lines 744–748) stays continuous.

`String(step?.ruleName || '')` on line 649 is not a heap allocation when `ruleName` is already a string (`RULE_NAMES[...]` in the engine). The win is skipping six string scans per frame, not GC.

The `orbMesh.position.y !== orbY` test is exact for a given `stepFraction`. During an unwind, `orbY` changes every frame and `updateMatrix()` runs every frame. That is the animation. Outside an unwind the orb is hidden and the matrix is left alone.

---

### D3-3. Plaque textures: one placeholder, no rebuild when the text is unchanged, no shader recompile on swap

#### Startup canvases (lines 469–489)

The loop calls `createPlaqueTexture` twelve times with `Frame #${i+1}` / `${slotId} · Saved continuation ring`. Each call allocates a 512×128 canvas, uploads a `CanvasTexture`, and inserts an LRU entry (`theme.js` 365–420). Slots are created with `visible = false` (line 430). `SherlockStage.render` runs `update()` before `render()`. The first real step replaces the map before any presented frame. Those twelve textures are never shown, and `theme.js` will not dispose them when the LRU evicts them (lines 412–417), so they occupy cache slots and GPU memory for the session.

**Replace lines 468–489** with one shared texture created **once** above the loop (next to `sharedOuterBevelMat`, after line 371):

```javascript
const sharedBlankPlaque = createPlaqueTexture({
  badge: '',
  title: '',
  subtitle: '',
  accent: CSS_OP_LEFT_AMBER,
  bg: 'rgba(18, 26, 38, 0.94)',
  fg: '#f3efe6',
  muted: '#9e978b',
  width: 512,
  height: 128,
});
```

Inside the loop:

```javascript
const initSig = `init|${slotId}`;
const plaqueMat = trackMat(
  new THREE.MeshBasicMaterial({
    map: sharedBlankPlaque,
    transparent: true,
    opacity: 0.96,
    depthWrite: false,
    side: THREE.DoubleSide,
  })
);
```

Do not push `sharedBlankPlaque` into `disposableResources`. It lives in the theme LRU and may be reused by an equal key.

`initSig` stays unique so the first real `updateWaferPlaque` still paints the live badge. The placeholder is a `CanvasTexture` with the same 512×128 sRGB layout as every later plaque, so the material’s texture binding type does not change on the first swap.

#### Skip formatting when the source fields are unchanged (lines 530–557)

`lastSignature` already skips `createPlaqueTexture`, but it is built **after** `formatWaferPlaqueTitle` (lines 534–540). That function allocates the title, the binding string, and the `[hole]` / `*` rewrites (lines 208–245) on every visible wafer on every step boundary, including steps whose stack text did not change. The engine builds a fresh `label` string per step (`sherlockEngine.js` 1441–1502), so object identity never hits. String `===` in JavaScript compares contents, so a value compare of the inputs is enough and allocates nothing on the hit path.

`formatWaferPlaqueTitle` and the subtitle depend only on:

- `frame.kind`, `frame.label`, `frame.astPath`, `frame.envId`
- `effectiveBindings[0].name` and `valueSummary` (see `formatBindingsCompact`, lines 195–202)
- `semFamily` (accent, bg, muted, family name)
- `slot.slotId` and `depthNumber === slot.slotIndex + 1`, both constant for a slot

A raw mismatch that still formats to the same title (whitespace collapsed inside `formatBindingsCompact`) falls through to the existing signature check and the theme LRU. That path paints the same pixels. A raw match cannot format to a different title.

**Insert at the top of `updateWaferPlaque`**, before line 531’s `kind` / `formatWaferPlaqueTitle` work:

```javascript
function updateWaferPlaque(slot, frame, depthNumber, effectiveBindings, semFamily) {
  const b0 =
    Array.isArray(effectiveBindings) && effectiveBindings.length > 0
      ? effectiveBindings[0]
      : null;
  const bindName = b0?.name || '';
  const bindVal = b0?.valueSummary ?? '';
  const label = frame.label || '';
  const astPath = frame.astPath || '';
  const envId = frame.envId ?? 0;
  const rawKind = frame.kind || '';
  if (
    slot.cacheSem === semFamily &&
    slot.cacheKind === rawKind &&
    slot.cacheLabel === label &&
    slot.cacheAst === astPath &&
    slot.cacheEnv === envId &&
    slot.cacheBindName === bindName &&
    slot.cacheBindVal === bindVal
  ) {
    return;
  }
  slot.cacheSem = semFamily;
  slot.cacheKind = rawKind;
  slot.cacheLabel = label;
  slot.cacheAst = astPath;
  slot.cacheEnv = envId;
  slot.cacheBindName = bindName;
  slot.cacheBindVal = bindVal;

  const kind = ALL_FRAME_KINDS.includes(frame.kind) ? frame.kind : 'Return';
  // ...existing meta / formatWaferPlaqueTitle / signature / createPlaqueTexture...
```

Keep the `lastSignature` check (lines 538–540) as the second gate so a formatting collision still skips the canvas.

#### Do not set `plaqueMat.needsUpdate` (lines 554–557)

`Material.needsUpdate = true` only does `this.version++` (`node_modules/three/src/materials/Material.js` 1227–1230). That version bump is how the WebGPU renderer decides to refresh a compiled material. The map itself is a live reference: `MaterialNode.getTexture('map')` builds a `MaterialReferenceNode`, and `ReferenceNode.updateValue()` copies `material.map` onto the texture node on each object update (`ReferenceNode.js` 310–324). The initial map is never null, and every replacement is another 512×128 sRGB texture, so the shader graph does not gain or lose a `map` term.

`createPlaqueTexture` already sets `tex.needsUpdate = true` when the texture is first built (`theme.js` 410). A cache hit returns the uploaded texture and must not reupload.

**Replace lines 554–557:**

```javascript
if (slot.plaqueMat.map !== tex) {
  slot.plaqueMat.map = tex;
}
```

#### Reuse the inspect `details` array (lines 570–576)

This runs only when the signature misses, so it is not a per-frame allocation. It is still a new 5-element array and five template strings on every real badge change. The strings are the inspect text and have to be new when the text changes. The array does not.

**Replace lines 570–576:**

```javascript
const details = slot.detailsScratch;
details[0] = `Half-Ring Slot ID: ${slot.slotId} (chronological index ${slot.slotIndex})`;
details[1] = 'Annular Ring Bore: innerRadius = 1.02, outerRadius = 1.28';
details[2] = `Operation & Binding: ${title}`;
details[3] = `Semantic Ring Family: ${semFamily.roleDesc}`;
details[4] = `AST Origin: Expr #${frame.originExprId ?? 'root'} (${frame.astPath || 'entry'})`;
slot.group.userData.details = details;
```

`detailsScratch` is allocated in D3-2’s slot record. Designer 7 reads `userData.details` and does not mutate it (`designer7_cinema.js` `extractInspectMeta`).

---

### D3-5. One rim mesh per wafer

Lines 452–459 draw three meshes (`innerCollarMesh`, `outerLipMesh`, `lowerLipMesh`) with the **same** `rimMat` and identity local transforms. The geometries are already in slot-local space (`createAnnularRingGeometry` translates, and the lip geos are rotated and translated at lines 328–338). Merging them once at init removes 24 draw calls and 24 raycast targets at 12 wafers, and the hit still resolves through `inspectableParent` (`designer7_cinema.js` 507–508) because `tagInspectable(slotGroup, ...)` runs after the children exist (line 499).

The import map in `index.html` lines 12–19 exposes `three`, `three/webgpu`, and `three/tsl` only. Do not import `three/addons/utils/BufferGeometryUtils.js`; it will fail to resolve. Merge locally.

**Add** next to `createAnnularRingGeometry`:

```javascript
function mergeIndexedGeometries(geos) {
  let vertCount = 0;
  let indexCount = 0;
  for (let g = 0; g < geos.length; g++) {
    vertCount += geos[g].getAttribute('position').count;
    indexCount += geos[g].getIndex().count;
  }
  const positions = new Float32Array(vertCount * 3);
  const normals = new Float32Array(vertCount * 3);
  const uvs = new Float32Array(vertCount * 2);
  const indices = new Uint32Array(indexCount);
  let vBase = 0;
  let iBase = 0;
  for (let g = 0; g < geos.length; g++) {
    const geo = geos[g];
    const pos = geo.getAttribute('position');
    const nrm = geo.getAttribute('normal');
    const uv = geo.getAttribute('uv');
    const idx = geo.getIndex();
    positions.set(pos.array, vBase * 3);
    normals.set(nrm.array, vBase * 3);
    uvs.set(uv.array, vBase * 2);
    for (let i = 0; i < idx.count; i++) indices[iBase + i] = idx.getX(i) + vBase;
    vBase += pos.count;
    iBase += idx.count;
  }
  const merged = new THREE.BufferGeometry();
  merged.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  merged.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
  merged.setAttribute('uv', new THREE.BufferAttribute(uvs, 2));
  merged.setIndex(new THREE.BufferAttribute(indices, 1));
  merged.computeBoundingSphere();
  return merged;
}
```

**After** the three source geometries exist (after line 338):

```javascript
const waferRimGeo = trackGeo(
  mergeIndexedGeometries([waferInnerCollarGeo, outerAccentLipGeo, lowerAccentLipGeo])
);
```

**Inside the slot loop**, replace the three rim meshes with:

```javascript
const rimMesh = new THREE.Mesh(waferRimGeo, rimMat);
slotGroup.add(rimMesh);
bakeStatic(rimMesh);
```

Keep building `waferInnerCollarGeo`, `outerAccentLipGeo`, and `lowerAccentLipGeo`. They are inputs to the merge. They can stay in `disposableResources`. `rimMat` is unchanged, including `side: THREE.DoubleSide`, `roughness: 0.24`, `metalness: 0.84`, and the per-frame emissive writes from D3-4, so the collar and both lips keep the same PBR response.

`ExtrudeGeometry` and `RingGeometry` in r175 both provide indexed `position`, `normal`, and `uv`. If a future Three.js change drops `uv` on one of them, `mergeIndexedGeometries` should be updated before shipping; do not drop the attribute, or the rim shader’s UVs change.

---

## 4. Designer 4 — `src/designers/designer4_arena.js`

### D4-1. Static children, manual matrix on the tile group

**Why.** Lines 409–415 already skip the JavaScript update between steps. The scene graph does not. From the renderer’s `updateMatrixWorld`, these 26 objects recompose every frame:

| Object | Local motion after init |
|---|---|
| `group` (line 291) | none |
| `tileRowGroup` (lines 311–313) | `position` set once |
| `tileGroup` × 6 (line 327) | `position.z` on a step change (line 435) |
| `bodyMesh` × 6 | none |
| `edgeMesh` × 6 | `position.y = -0.024` once (line 353) |
| `plaqueMesh` × 6 | `rotation.x` and `position.y` once (lines 373–374) |

Same rule as D3-1: bake locals, leave `matrixWorldAutoUpdate` true, call `updateMatrix()` after a real `position.z` change. `bodyMesh.castShadow` (line 339) keeps working because the shadow pass reads `matrixWorld`, which still follows the parent.

**Add** inside `createArenaSubsystem`, next to `trackTex` (after line 309):

```javascript
function bakeStatic(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
  return obj;
}
```

**Replace lines 311–314:**

```javascript
const tileRowGroup = new THREE.Group();
tileRowGroup.name = 'WestFlankCallTileRow';
tileRowGroup.position.set(WEST_FLANK_X, GROUND_TILE_Y, 0);
bakeStatic(tileRowGroup);
group.add(tileRowGroup);
```

**Replace the mesh construction inside the loop (lines 327–375)** so each static mesh is baked after its one local write:

```javascript
const tileGroup = new THREE.Group();
tileGroup.name = `CallTile_${i}`;
tileGroup.matrixAutoUpdate = false;
tileRowGroup.add(tileGroup);

const bodyMat = trackMat(
  new THREE.MeshPhysicalMaterial({
    color: HEX_TILE_IDLE,
    roughness: 0.42,
    metalness: 0.22,
  })
);
const bodyMesh = new THREE.Mesh(tileBodyGeo, bodyMat);
bodyMesh.castShadow = true;
bodyMesh.receiveShadow = true;
bakeStatic(bodyMesh);
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
bakeStatic(edgeMesh);
tileGroup.add(edgeMesh);

const plaque = createCallTileCanvasPlaque();
trackTex(plaque.tex);

const plaqueMat = trackMat(
  new THREE.MeshBasicMaterial({
    map: plaque.tex,
    transparent: true,
    alphaTest: 0.08,
    depthWrite: false,
    toneMapped: false,
  })
);
const plaqueMesh = new THREE.Mesh(tilePlaqueGeo, plaqueMat);
plaqueMesh.rotation.x = -Math.PI / 2;
plaqueMesh.position.y = tileHeight * 0.5 + 0.002;
bakeStatic(plaqueMesh);
tileGroup.add(plaqueMesh);
```

The init `paintCleanCallTile` at line 361 is removed here on purpose. It is covered by D4-2: `update()` at line 501 runs before the group is parented and before the first presented frame, and with an empty trace it hides every tile (`count === 0`). The pixels from `fact(${4-i})` / `future` are never shown. Painting them uploads six 512×256 textures that the first real trace immediately overwrites.

**After the loop:**

```javascript
bakeStatic(group);
```

**Replace lines 432–435** (the per-tile position write):

```javascript
tile.group.visible = true;
const call = visibleCalls[i];
const targetZ = startZ + i * spacing;
if (tile.group.position.z !== targetZ || tile.group.position.x !== 0 || tile.group.position.y !== 0) {
  tile.group.position.set(0, 0, targetZ);
  tile.group.updateMatrix();
}
```

`spacing` and `startZ` are pure functions of `count` (lines 422–423). The same count yields the same `targetZ` bits, so `!==` does not miss an update and does not rewrite a stationary tile. Hidden tiles (lines 427–429) keep whatever matrix they had; they are not drawn. Guard the hide so a held step that somehow re-enters does not store `visible` again:

```javascript
if (i >= count) {
  if (tile.group.visible) tile.group.visible = false;
  continue;
}
```

---

### D4-2. Cache painted faces, keep one canvas per cached signature

**What is already a pool.** `createCallTileCanvasPlaque` (lines 228–240) builds one canvas per tile. `paintCleanCallTile` (lines 243–284) clears and repaints that same canvas, then sets `tex.needsUpdate = true`, which reuploads 512×256×4 bytes. The signature at line 464 stops this when the face is unchanged. Scrubbing back to a previous `(call, value, status, ring)` paints and uploads again because the previous pixels were destroyed by `clearRect`.

**Alias hazard.** A `CanvasTexture` displays whatever is currently on its canvas. Repainting a canvas that is still the `map` of any tile, or still stored under an older signature, changes that older face too. A cache entry owns its canvas for life. Recycle a canvas only when no tile’s `plaqueMat.map` points at its texture.

**Font strings.** Lines 260, 274, and 279 build a font template on every paint. Four constants remove that allocation from the raster path. The glyphs do not change.

**Add** above `paintCleanCallTile` (before line 243):

```javascript
const FONT_RING = '600 32px "JetBrains Mono", ui-monospace, monospace';
const FONT_LABEL_LG = '600 80px "JetBrains Mono", ui-monospace, monospace';
const FONT_LABEL_SM = '600 68px "JetBrains Mono", ui-monospace, monospace';
const FONT_VALUE = '500 58px "JetBrains Mono", ui-monospace, monospace';
const PLAQUE_CACHE_MAX = 32;
```

**Replace the four `ctx.font = ...` assignments inside `paintCleanCallTile`:**

```javascript
ctx.font = FONT_RING;          // was line 260
ctx.font = hasVal ? FONT_LABEL_SM : FONT_LABEL_LG; // was line 274
ctx.font = FONT_VALUE;         // was line 279
```

Leave the `fillStyle` branches (lines 255–272 and 277–281) exactly as they are. Those colors are the face.

**Inside `createArenaSubsystem`**, next to the tile array:

```javascript
const plaqueCache = new Map();
const plaquePool = [];

function plaqueEntryFromPool() {
  const reused = plaquePool.pop();
  if (reused) return reused;
  const created = createCallTileCanvasPlaque();
  trackTex(created.tex);
  return created;
}

function acquirePlaqueTexture(sig, label, value, status, ringTag, pinned) {
  const hit = plaqueCache.get(sig);
  if (hit) {
    plaqueCache.delete(sig);
    plaqueCache.set(sig, hit);
    return hit.tex;
  }
  let entry = null;
  if (plaqueCache.size >= PLAQUE_CACHE_MAX) {
    for (const [key, candidate] of plaqueCache) {
      if (pinned.has(candidate.tex)) continue;
      plaqueCache.delete(key);
      entry = candidate;
      break;
    }
  }
  if (!entry) entry = plaqueEntryFromPool();
  paintCleanCallTile(entry, label, value, status, ringTag);
  plaqueCache.set(sig, entry);
  return entry.tex;
}
```

`pinned` is the set of textures currently on a tile, built at the start of the tile loop so an eviction cannot repaint a face that is on screen:

```javascript
const pinnedPlaques = new Set();
for (let p = 0; p < MAX_CALL_TILES; p++) pinnedPlaques.add(callTiles[p].plaqueMat.map);
```

**Replace lines 464–468:**

```javascript
if (
  tile.lastCallExpr !== call.callExpr ||
  tile.lastRetVal !== retVal ||
  tile.lastStatus !== call.status ||
  tile.lastRingNum !== ringNum
) {
  tile.lastCallExpr = call.callExpr;
  tile.lastRetVal = retVal;
  tile.lastStatus = call.status;
  tile.lastRingNum = ringNum;
  const sig = `${call.callExpr}|${retVal}|${call.status}|R${ringNum}`;
  const tex = acquirePlaqueTexture(sig, call.callExpr, retVal, call.status, `R${ringNum}`, pinnedPlaques);
  if (tile.plaqueMat.map !== tex) tile.plaqueMat.map = tex;
  pinnedPlaques.add(tex);
}
```

The template string now runs only when the face is new, which is also when `paintCleanCallTile` must run on a cache miss. A cache hit does not build the string if the field compare hits first. Store `lastCallExpr`, `lastRetVal`, `lastStatus`, and `lastRingNum` on the tile record (lines 388–398) instead of `lastSignature`:

```javascript
lastCallExpr: '',
lastRetVal: null,
lastStatus: '',
lastRingNum: -1,
lastColorId: -1,
```

`lastRetVal: null` so the first resolved empty string still paints. `call.callExpr` and `call.status` are stable strings on the cached call node, and `===` also compares contents if the engine ever rebuilds them.

Do not set `plaqueMat.needsUpdate` on a map swap. Same reason as D3-3: the material is created with a map already set (the per-tile canvas or the first cached texture), and `ReferenceNode` tracks `material.map`.

`dispose()` already walks `disposableTextures` (lines 496–498). `plaqueEntryFromPool` pushes every new texture through `trackTex`, so a cache of 32 is released with the subsystem. Textures that were only popped to `plaquePool` and not yet re-tracked were tracked at creation. Do not dispose a texture twice; `trackTex` must ignore duplicates:

```javascript
const trackTex = (t) => {
  if (t && disposableTextures.indexOf(t) === -1) disposableTextures.push(t);
  return t;
};
```

Bound of 32 faces × 512×256×4 ≈ 16 MB if every entry is resident. A factorial or fib trace uses far fewer distinct `(expr, value, status, ring)` faces. The cap stops a long session from growing without a visual change, because eviction repaints the same function into a recycled canvas.

---

### D4-3. Zero allocations on the step-change path

`update()` returns early between steps, so these are not per-frame. They allocate on every step index change, which is every frame during a fast scrub.

#### Trace cache hit builds a string (lines 82–87)

```javascript
const traceKey = `${trace?.programName || ''}:${steps.length}`;
if (trace && trace === _cachedTraceRef && traceKey === _cachedTraceKey) {
  return _cachedAllCalls;
}
```

The key is allocated before the hit test. Length and program name are the whole key. Compare them as a number and a string the engine already holds.

**Replace lines 78–87** and the store at lines 163–166:

```javascript
let _cachedTraceRef = null;
let _cachedStepCount = -1;
let _cachedProgramName = '';
let _cachedAllCalls = [];

function getOrBuildTraceCalls(trace) {
  const steps = Array.isArray(trace?.steps) ? trace.steps : [];
  const programName = trace?.programName || '';
  if (
    trace &&
    trace === _cachedTraceRef &&
    steps.length === _cachedStepCount &&
    programName === _cachedProgramName
  ) {
    return _cachedAllCalls;
  }
  // ...existing rebuild...
  _cachedTraceRef = trace;
  _cachedStepCount = steps.length;
  _cachedProgramName = programName;
  _cachedAllCalls = allCalls;
  return allCalls;
}
```

Invalidation matches the old key: a new trace object, a different `steps.length`, or a different `programName`. In-place edits that keep all three are invisible to both versions.

While building each `callNode` (lines 109–122), store the ring so `update()` does not format `` `R${ringNum}` `` per tile per step. `resolveCallRingNumber` (lines 73–76) is a pure function of `depth` and `index`, both known here:

```javascript
const depth = openStack.length;
const index = allCalls.length + 1;
const rawRing = (depth > 0 ? depth + 1 : index) || 1;
const ringNum = clamp(rawRing, 1, 6);
const callNode = {
  index,
  callExpr: cleanCall,
  rawCall: String(rawCall).replace(/^Invoke\((.*)\)$/i, '$1'),
  invokeStep: invokeStepIdx,
  enterStep: i,
  returnStep: null,
  returnValue: null,
  depth,
  ringNum,
  ringTag: `R${ringNum}`,
  spawnedChildren: 0,
  isFaulted: false,
  isBaseCase: false,
  status: 'future',
};
```

Apply the same `ringNum` / `ringTag` fields on the empty-trace fallback object (lines 143–156). `update()` then uses `call.ringNum` and `call.ringTag` and does not call `resolveCallRingNumber`.

#### Result object (lines 222–225)

```javascript
return {
  visibleCalls: _visibleCallsScratch,
  isError,
};
```

This is a new object per step. Hoist it next to `_visibleCallsScratch` (line 169):

```javascript
const _visibleCallsScratch = [];
const _evalScratch = { visibleCalls: _visibleCallsScratch, isError: false };

function evaluateCallTilesAtStep(stepState) {
  // ...existing body through the isError computation...
  _evalScratch.isError = isError;
  return _evalScratch;
}
```

The caller already reads `visibleCalls` and `isError` synchronously (line 420) and does not retain the object. Reuse is invisible.

#### Color writes (lines 441–461)

`Color.setHex` does not allocate, but it runs for every visible tile on every step even when status and error bit are unchanged (a window slide that keeps the same status, or a step whose call states match the previous one). `instance` colors are not involved; each tile has its own materials. A numeric id avoids a status string and keeps the error red path (`PALETTE.errorRed`, line 447) intact.

**Add** next to the hex constants (after line 21):

```javascript
const _cTileIdle = new THREE.Color(HEX_TILE_IDLE);
const _cTileActive = new THREE.Color(HEX_TILE_ACTIVE);
const _cTileReturned = new THREE.Color(HEX_TILE_RETURNED);
const _cTileFault = new THREE.Color(HEX_TILE_FAULT);
const _cRimIdle = new THREE.Color(HEX_RIM_IDLE);
const _cRimWaiting = new THREE.Color(HEX_RIM_WAITING);
const _cRimActive = new THREE.Color(HEX_RIM_ACTIVE);
const _cRimReturned = new THREE.Color(HEX_RIM_RETURNED);
const _cRimError = new THREE.Color(PALETTE.errorRed);
const _cBlack = new THREE.Color(0x000000);

function applyTileColors(tile, colorId) {
  if (tile.lastColorId === colorId) return;
  tile.lastColorId = colorId;
  const body = tile.bodyMat.color;
  const edge = tile.edgeMat.color;
  const glow = tile.edgeMat.emissive;
  if (colorId === 1) {
    body.copy(_cTileReturned);
    edge.copy(_cRimReturned);
    glow.copy(_cRimReturned);
    tile.edgeMat.emissiveIntensity = 0.72;
  } else if (colorId === 2) {
    body.copy(_cTileFault);
    edge.copy(_cRimError);
    glow.copy(_cRimError);
    tile.edgeMat.emissiveIntensity = 0.90;
  } else if (colorId === 3) {
    body.copy(_cTileActive);
    edge.copy(_cRimActive);
    glow.copy(_cRimActive);
    tile.edgeMat.emissiveIntensity = 0.90;
  } else if (colorId === 4) {
    body.copy(_cTileIdle);
    edge.copy(_cRimWaiting);
    glow.copy(_cRimWaiting);
    tile.edgeMat.emissiveIntensity = 0.38;
  } else {
    body.copy(_cTileIdle);
    edge.copy(_cRimIdle);
    glow.copy(_cBlack);
    tile.edgeMat.emissiveIntensity = 0.0;
  }
}
```

**Replace lines 441–461:**

```javascript
const ringNum = call.ringNum;
const retVal = call.status === 'resolved' ? call.returnValue || 'ok' : '';
const colorId =
  call.status === 'resolved' ? 1 :
  call.status === 'active' ? (isError ? 2 : 3) :
  call.status === 'waiting' ? 4 : 5;
applyTileColors(tile, colorId);
```

Ids: `1` returned, `2` active+error, `3` active, `4` waiting, `5` future. Same hex values, same intensities as lines 441–461. `lastColorId` is on the tile record from D4-2. Resetting it is unnecessary when a tile is hidden; the next show compares against the last applied id and rewrites if the new call’s status differs.

Inspect strings at lines 480–484 stay as they are. They are already gated by `lastMetaCallIndex` / `lastMetaStatus` / `lastMetaReturn` (lines 470–478) and they are the text Designer 7 shows. Building them only when that triple changes is the right cutoff.

---

## 5. Preserve — changes that would move pixels or break a hit

Keep these as they are.

- **Annular `curveSegments: 64`** (lines 296, 308, 322, 329, 335) and the bevel thicknesses. Fewer segments change the bore silhouette. The cost is paid once because the geometry is shared.
- **Sapphire web `transparent: true`, `opacity: 0.36`, `depthWrite: false`, `side: DoubleSide`** (lines 344–355). The interior and cross-section cameras see the underside. An `InstancedMesh` of this material would draw instances in buffer order. Separate meshes are sorted by the renderer. Instancing the glass changes overlap against the rhodium ring, the orb, and the stator. Leave the 12 glass meshes as meshes.
- **Per-slot `rimMat` emissiveIntensity** (lines 714–718, preserved in D3-4). `InstancedMesh.setColorAt` multiplies diffuse (`MaterialNode` `USE_COLOR` path). It does not multiply emissive. One shared rim material plus `instanceColor` would flatten the active-wafer glow and the unwind flash. D3-5 merges geometries that already share a material; it does not merge across slots.
- **`matrixWorldAutoUpdate` left at `true`** on slot groups, tile groups, plaques, the orb, and every baked child. The local freeze is `matrixAutoUpdate` only.
- **Designer 3 `update()` still runs when the step index is unchanged.** `stepFraction` drives the orb and the flash (lines 703–705, 750–755). `activePreset` drives badge opacity (lines 604–609). Gating the whole function the way Designer 4 does would freeze the orb mid-flight and leave badges up in Overview.
- **Badge and tile texture resolution** (512×128 plaques, 512×256 chiplets). Dropping it softens glyphs under the interior camera and the top-down west flank. Cache and reuse; do not shrink.
- **`bodyMesh.castShadow` and `receiveShadow`** (lines 339–340) and `edgeMesh.receiveShadow` (line 354). Turning casters off removes the contact shadow on the studio floor.
- **`alphaTest: 0.08` and `toneMapped: false`** on the chiplet plaque (lines 367–368). Both are part of the current glyph edge and color.
- **Shared texture disposal.** `dispose()` in Designer 3 must keep ignoring theme LRU textures. Designer 4 may dispose the textures it uniquely created (the cache entries from D4-2) because nothing else holds them.

---

## 6. Apply order

1. D3-1 and D4-1 (`bakeStatic` + manual `updateMatrix`). Largest steady-frame CPU cut, no texture behavior change.
2. D3-2 snap thresholds, in the same `update()` edit as D3-1, or the lerp tail will call `updateMatrix()` forever and undo the freeze.
3. D3-4 uniform and unwind caching, same edit.
4. D3-3 placeholder and the raw-field gate. Confirm a factorial step change still swaps in `RET fact(4) · n=4` and that a second step with the same stack does not call `createPlaqueTexture` (the existing `lastSignature` path).
5. D4-3 allocation cleanup. Purely mechanical; colors go through `applyTileColors` with the same hex values.
6. D4-2 plaque cache. After this, scrubbing off a call and back should assign the cached texture (`plaqueMat.map` identity hit) and should not `clearRect` that canvas.
7. D3-5 rim merge last. It changes object identity (three meshes become one) while keeping the same triangles. Re-check a click on the collar, the lip, and the glass: all three should still resolve to the wafer’s inspect record.

Suggested visual check after an implementation pass, at 1× and while scrubbing:

- Overview and Top-Down: wafer badges at opacity 0, no text in the ring gaps (lines 601–609).
- Interior / Interior-Up / Frames: badges fade to 0.96 and pitch to `-0.06π`, then hold still when paused.
- A return step (rule lines 153, 185, 195, 80, or control kind `Return`): deepest collar flashes toward `0x2dd4bf`, orb rises from `lastActiveY` to `y = -0.285` and hides when the step is not an unwind.
- West flank for `fact` and a trace with more than 6 calls (window slide, lines 200–207): idle, waiting, active, returned, and error red (`PALETTE.errorRed`) rims, including the shadow under the active body.
- Pause for two seconds: GPU timeline should show the wafer and chiplet subtrees issuing no new canvas uploads and no per-frame `matrix.compose` on the baked meshes.

This audit did not apply the snippets and did not run the scene.
