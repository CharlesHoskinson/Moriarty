# Performance audit 01 — Stage, WebGPU renderer, lighting, shadows, raycasting

Auditor: Grok 4.7 xhigh, stage / WebGPU / lighting / shadows / raycasting.
Scope read: `src/stage/SherlockStage.js` (380 lines) and `src/designers/theme.js` (558 lines).
Raycasting is implemented in `src/designers/designer7_cinema.js` (constructed at `SherlockStage.js` 205–212) and registered through `tagInspectable` in `theme.js` 436–450. Those call sites are cited because the stage file does not contain a `Raycaster`.
Installed Three.js is **0.186.1** (`node_modules/three/package.json`), imported as `three/webgpu`. `package.json` allows `^0.175.0`.
No source files were modified.

Constraint used throughout: a change qualifies only when a visible frame keeps the same shaded pixels, the same shadow edges, the same IBL, and the same hover target. Startup work may move earlier. Frames the user cannot see (hidden tab) may skip the GPU submit.

## What already holds the image still

These are in place. Replacing them with a coarser shadow map, fewer lights, or a lower pixel ratio would change the picture.

| Site | What it already does |
|---|---|
| `SherlockStage.js` 85–87 | One `THREE.Color` and one `_fullState` object, reused every frame. |
| `SherlockStage.js` 97–108 | `WebGPURenderer`, `powerPreference: 'high-performance'`, device pixel ratio capped at 2, ACES Filmic at exposure `1.06`, `PCFShadowMap`. |
| `SherlockStage.js` 126–137, 362–364 | `await renderer.init()` before any `renderer.render`. Headless failure leaves `ready === false` and does not throw into the rAF loop. Subsystem `update` still runs, so CPU state stays deterministic. |
| `SherlockStage.js` 140–193 | One shadow-casting directional light. Fill, rim, hemisphere, and the four interior points do not cast. Interior semantic color is written through `_scratchColor` (312–339), not `new THREE.Color()`. |
| `SherlockStage.js` 353–360 | Hidden designer groups skip `update`, except `cinema`, which owns the camera. |
| `theme.js` 104–109 | Module-scope scratch math. Nothing in `src/` currently aliases these into a material, so they are safe. |
| `theme.js` 131–137, 144–207, 215–278, 284–322, 329–420 | Metal, ceramic, reticle, and plaque textures are keyed and reused. Plaque eviction (412–418) drops the `Map` entry and does **not** call `dispose()` on a texture a mesh may still display. |
| `theme.js` 453–557 | Studio equirect is a process singleton. `scene.environment` is assigned once (`SherlockStage.js` 142–143). |

`app.js` 946–969 calls `stage.render` on every animation frame while a trace exists, including while playback is paused. That is correct for the current image: Designer 2 rewrites light-pipe emissive with `sin(ambient)` every update, and Designer 8 moves spore and tendril vertices with `ambient` every update. Those motions are not shadow casters. They do force the beauty pass to stay live while the tab is visible.

## Measured shadow coverage (do not retune the window)

Configured window at `SherlockStage.js` 153–159:

- map `2048×2048`
- orthographic `left = -3.6`, `right = 2.6`, `top = 2.8`, `bottom = -2.8` (6.20 × 5.60)
- `near = 4.5`, `far = 19`
- `bias = -0.00012`, `normalBias = 0.004`
- light at `(6.0, 10.5, 5.4)`, target at the origin

Three.js applies that projection when the shadow node is first built (`node_modules/three/src/nodes/lighting/ShadowNode.js` 378–379, `shadow.camera.updateProjectionMatrix()`). It does **not** rebuild the 2048² target when the view camera orbits. With `shadow.autoUpdate === true` (the `LightShadow` default, `LightShadow.js` 148), `ShadowNode.updateBefore` (`ShadowNode.js` 800–816) re-renders the shadow scene on every beauty frame anyway. `renderShadow` (`ShadowNode.js` 641–654) calls `renderer.render(scene, shadow.camera)`.

Projecting receiver corners through that orthographic camera (Three r186 `OrthographicCamera` + `LightShadow.updateMatrices`, after `updateProjectionMatrix`):

| Point | World | NDC | Inside |
|---|---|---|---|
| Arena west extent | `(-3.24, -0.29, ±1.86)` | x `-0.984` / y `0.971` | yes, near the edge |
| Floor NW / SE / SW tile tops | y ≈ `-3.29` | \|y\| ≤ `0.74` | yes |
| Floor NE tile top | `(1.323, -3.290, 1.338)` | y **`-1.248`** | no |
| Ceramic carrier corner | `(2.15, -0.35, -2.15)` | x **`1.141`** | no |
| Package / gate / rings / cutaway bench at y `-3.44` | — | inside | yes |
| Bench at \|x\| or \|z\| = 6 | y `-0.36` | outside | no (28-unit ground, half-span 14) |

Texel size on the configured window is about `6.20 / 2048 ≈ 0.00303` world units. `normalBias = 0.004` is about 1.3 texels. Leave map size, bias, normalBias, radius (default `1`), and `PCFShadowMap` alone. A tighter window clips the NE floor and the +X/−Z ceramic corner further. A wider window changes every shadow edge. `VSMShadowMap` adds blur passes (`ShadowNode.js` 701–704) and a different filter. `BasicShadowMap` hardens the edges.

Caster set (`castShadow = true` under `src/`):

- package U-bodies and the front gate (`designer1_package.js` 534, 559) — gate moves only while `cutawayMix` moves
- 24 gold leads (`designer1_package.js` 682) — instance **colors** pulse; instance **matrices** stay put
- 12 split-arc bodies (`designer2_well.js` 1522) — Y / yaw lerp settles at rate 14–16 (`designer2_well.js` 2429–2430, 2587–2592)
- 39 floor tiles (`designer2_well.js` 2234) — active tile pose is constant; trail decay finishes in about `0.96 / 3.5 ≈ 0.27 s` (`designer2_well.js` 2952–2971)
- six west chiplet bodies (`designer4_arena.js` 339) — move on step change, then stop

Quineling meshes, light pipes, stator pads, and the bench do not cast. Continuous emissive breath does not belong in the shadow map.

Depth of the covered content sits around view-Z `-11.3 … -17.1`, inside `near = 4.5` / `far = 19`. Shrinking that range does not reduce the 2048² fill; it only requantizes depth and can move acne relative to the tuned bias. Leave near and far alone.

## Finding 1 — Freeze the 2048² shadow pass when casters are still

**Impact:** one extra scene traversal and a depth render of every caster, every frame, including while the user only orbits. Orbit does not move this directional light or a settled caster. The beauty pass keeps running, so breath, spores, and the reticle stay live. The shadow image stays the one from the last settling frame.

**Why 0.50 s is enough:** cutaway snaps by `0.005` at rate 12 (`SherlockStage.js` 295–298), which is `t = -ln(0.005) / 12 ≈ 0.44 s`. Arc elevation residuals fall below a shadow texel well before `0.40 s` at rate 14. Floor trail pose stops changing once `trail ≤ 0.04`. The window below is restarted on the last frame `cutawayMix` actually changes, so the `0.50 s` is margin after the last motion, not a cap that cuts the lerp off.

`ShadowNode` only clears `shadow.needsUpdate` when the depth-texture version did not change (`ShadowNode.js` 818–821). Assign the flag explicitly every frame so a sticky `true` cannot keep the pass alive.

### Replace `SherlockStage.js` 149–162

```javascript
    // 2. Warm Daylight Key Light. 2048² PCF, one cascade, fixed frustum.
    //    autoUpdate is off: ShadowNode re-renders this map only when needsUpdate is set.
    const keyLight = new THREE.DirectionalLight(0xfff7ea, 2.05);
    keyLight.position.set(6.0, 10.5, 5.4);
    keyLight.castShadow = true;
    keyLight.shadow.mapSize.set(2048, 2048);
    keyLight.shadow.camera.near = 4.5;
    keyLight.shadow.camera.far = 19.0;
    keyLight.shadow.camera.left = -3.6;
    keyLight.shadow.camera.right = 2.6;
    keyLight.shadow.camera.top = 2.8;
    keyLight.shadow.camera.bottom = -2.8;
    keyLight.shadow.bias = -0.00012;
    keyLight.shadow.normalBias = 0.004;
    keyLight.shadow.autoUpdate = false;
    keyLight.shadow.needsUpdate = true;
    keyLight.shadow.camera.updateProjectionMatrix();
    this.keyLight = keyLight;
    this.scene.add(keyLight);
    freezeStaticObject3D(keyLight);
    freezeStaticObject3D(keyLight.target);
```

Add the helper next to `SUBSYSTEM_KEYS` (`SherlockStage.js` 71):

```javascript
const SUBSYSTEM_KEYS = ['package', 'well', 'frames', 'arena', 'circuit', 'loom', 'quineling', 'cinema'];
const GATE_RULE_LINES = new Set([114, 123, 126, 161, 165, 173, 214, 220]);
const SHADOW_SETTLE_SEC = 0.5;

function freezeStaticObject3D(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
  obj.updateMatrixWorld(true);
}
```

Call `freezeStaticObject3D` on the hemisphere, fill, and rim after they are positioned (`SherlockStage.js` 146–172), and on `vaultUpperLight`, `wellPointLight`, `vaultFloorLight`, and `coreSpecLight` after their positions are set (`SherlockStage.js` 177–193). Their positions and colors are the only things that change later; color writes do not need `matrixAutoUpdate`. Parent `this.root` does not move. A frozen world matrix matches the matrix Three.js would recompute.

### Replace `setLayerVisible` (`SherlockStage.js` 236–242)

```javascript
  setLayerVisible(id, visible) {
    if (!(id in this.layerVisibility)) return;
    const next = Boolean(visible);
    if (this.layerVisibility[id] === next) return;
    this.layerVisibility[id] = next;
    if (this.subsystems[id]?.group) {
      this.subsystems[id].group.visible = next;
    }
    this._shadowForce = true;
  }
```

The early return also stops a same-value toggle from dirtying the shadow map.

### Shadow flag inside `render`, immediately before `renderer.render`

Constructor additions beside `_cutawayMix` (`SherlockStage.js` 83):

```javascript
    this._shadowForce = true;
    this._shadowSettleUntil = 0;
    this._shadowStep = Number.NaN;
    this._shadowVis = -1;
    this._shadowCutaway = Number.NaN;
    this._lightMode = -1;
    this._vaultMode = -1;
```

```javascript
    const stepIndex = fullState.stepIndex ?? -1;
    let vis = 0;
    for (let i = 0; i < SUBSYSTEM_KEYS.length; i++) {
      if (this.layerVisibility[SUBSYSTEM_KEYS[i]] !== false) vis |= 1 << i;
    }
    if (
      this._shadowForce ||
      stepIndex !== this._shadowStep ||
      vis !== this._shadowVis ||
      this._cutawayMix !== this._shadowCutaway
    ) {
      this._shadowSettleUntil = this.ambient + SHADOW_SETTLE_SEC;
      this._shadowStep = stepIndex;
      this._shadowVis = vis;
      this._shadowCutaway = this._cutawayMix;
      this._shadowForce = false;
    }
    // Explicit assignment every frame. See ShadowNode.js 818–821.
    this.keyLight.shadow.needsUpdate = this.ambient < this._shadowSettleUntil;
```

While a trace is playing, `stepIndex` changes about every `1 / 3.2 s`, the window stays open, and shadows follow the step. While paused and orbiting, the pass stops half a second after the last step, layer toggle, or cutaway sample. Leads may still pulse color. Their silhouettes do not.

## Finding 2 — Skip the GPU submit while the document is hidden

**Site:** `render` (`SherlockStage.js` 288–364), driven by `app.js` 946–969.

Chrome throttles a hidden tab to about 1 Hz and still runs the beauty pass and the shadow pass. Neither image is on a display. Keep the CPU `update` calls so lerp state tracks the playhead the same way the throttled loop does today. Skip only `renderer.render`. On the next visible frame, force one shadow redraw because casters may have moved with no depth pass in between.

```javascript
  render(stepState) {
    const now = performance.now();
    const dt = Math.min(0.05, Math.max(0.001, (now - this.lastTime) / 1000));
    this.lastTime = now;

    const hidden =
      typeof document !== 'undefined' && document.visibilityState === 'hidden';

    this.ambient += dt;
    // ... existing cutaway, _fullState, lights, subsystem updates ...

    if (hidden) {
      this._shadowForce = true;
      return;
    }

    // shadow needsUpdate assignment from Finding 1
    if (this.ready === true) {
      this.renderer.render(this.scene, this.camera);
    }
  }
```

Do not skip `renderer.render` while `visibilityState === 'visible'`. Designer 2 and Designer 8 change shaded pixels every frame from `ambient`. A settled-camera dirty bit on the beauty pass would freeze that motion.

`resize` (`SherlockStage.js` 120–123, 277–286) schedules `setSize` on every window `resize` event. Coalesce to one buffer realloc per frame. The size that lands is the size of the latest event before that frame, which is the size the browser paints.

```javascript
    this._resizeScheduled = false;
    this._onResize = () => {
      if (this._resizeScheduled || this._disposed) return;
      this._resizeScheduled = true;
      requestAnimationFrame(() => {
        this._resizeScheduled = false;
        if (!this._disposed) this.resize();
      });
    };
```

Leave the DPR cap at 2 (`SherlockStage.js` 102–103) and `setSize(..., false)` (`SherlockStage.js` 285). The `false` flag keeps the canvas CSS box under the page layout.

## Finding 3 — Bake PMREM once, after `renderer.init()`

**Site:** `createStudioEnvironmentMap` (`theme.js` 461–557) and `_setupLights` / `_initRenderer` (`SherlockStage.js` 126–143).

The equirect is already a singleton, `512×256`, `EquirectangularReflectionMapping`, sRGB. That part is right. WebGPU does not sample that image directly for rough metals. `PMREMNode.updateBefore` (`node_modules/three/src/nodes/pmrem/PMREMNode.js` 290–316) builds a PMREM on the first material that needs it, then caches it in a per-renderer `WeakMap` (`PMREMNode.js` 43–114). Each such node also constructs its own `PMREMGenerator` (`PMREMNode.js` 321–325). The first generator keeps a ping-pong render target for the life of the node (`PMREMGenerator.js` 367–372).

`fromEquirectangular` sizes the cube from `texture.image.width / 4` (`PMREMGenerator.js` 346–355). A 512-wide canvas becomes cube size 128 and a CubeUV target of `384 × 512` half-float. That is the image the lazy path already produces. Generating it inside the first interactive frame is what stalls. Raising the canvas to `1024×512` would raise the cube to 256 and change roughness response. Leave the canvas size alone.

`fromEquirectangular` throws if it runs before `renderer.init()` (`PMREMGenerator.js` 215–218). Keep the singleton assignment in `_setupLights` so materials have an environment if init fails. Replace it with the PMREM texture **before** `this.ready = true`, so the first compiled pipelines see the final texture.

### Replace `_initRenderer` (`SherlockStage.js` 126–138)

```javascript
  async _initRenderer() {
    try {
      await this.renderer.init();
      if (this._disposed) {
        this.renderer.dispose();
        return;
      }
      this._bakeStudioPMREM();
      this.ready = true;
      this.resize();
    } catch (err) {
      this.ready = false;
    }
  }

  _bakeStudioPMREM() {
    if (!this._envSource || this._envTarget) return;
    const pmrem = new THREE.PMREMGenerator(this.renderer);
    const rt = pmrem.fromEquirectangular(this._envSource);
    pmrem.dispose();
    this._envTarget = rt;
    this.scene.environment = rt.texture;
    this.scene.environmentIntensity = 0.68;
  }
```

### Store the source (`SherlockStage.js` 141–143)

```javascript
    this._envSource = createStudioEnvironmentMap();
    this.scene.environment = this._envSource;
    this.scene.environmentIntensity = 0.68;
```

`PMREMNode` treats `isPMREMTexture` / `CubeUVReflectionMapping` as already filtered (`PMREMNode.js` 299–301) and applies the render-target Y flip when `isRenderTargetTexture` is set (`PMREMNode.js` 343–348). `fromEquirectangular` sets both (`PMREMGenerator.js` 851–853). Intensity stays `0.68`. Background stays the solid `0xf4f0e8` (`SherlockStage.js` 90–91), so the PMREM is reflections only.

`pmrem.dispose()` releases the ping-pong target and the blur materials. It does not release `rt`. `dispose()` on the stage must:

```javascript
    if (this._envTarget) {
      this._envTarget.dispose();
      this._envTarget = null;
    }
    if (this._envSource) {
      this._envSource.dispose();
      this._envSource = null;
    }
```

Add that next to `this.renderer.dispose()` (`SherlockStage.js` 376–378).

`createStudioEnvironmentMap` returns the first texture forever and ignores a later `width` / `height` (`theme.js` 462–464). The stage calls it once with the defaults. Keep that singleton. A second size would be a second GPU upload and a different reflection.

## Finding 4 — Raycast the inspectable roots, with a trailing 60 ms edge

**Sites:** `performRaycast` (`designer7_cinema.js` 536–555), the throttle (`designer7_cinema.js` 631–648), `tagInspectable` (`theme.js` 436–450).

The 60 ms gate and the same-pixel early-out are already there (`RAYCAST_THROTTLE_MS = 60`, lines 430–433 and 631–648). Clicks ignore the gate and raycast once (`designer7_cinema.js` 663–668). What still dominates is the target set:

```javascript
raycaster.setFromCamera(pointerNDC, camera);
const hits = raycaster.intersectObjects(scene.children, true);
```

`intersectObjects(..., true)` walks every descendant (`Raycaster.js` 218–228 and 240–262). Layer tests do not prune children: a failed `layers.test` still recurses (`Raycaster.js` 244–261). The walk includes the 28×28 bench `ShapeGeometry` (`designer1_package.js` 399–433, half-span 14, not passed to `tagInspectable`), eight untagged terrace walls (`designer2_well.js` 2022–2036), lights, and any mesh whose hit is then discarded.

`findInspectableTarget` (`designer7_cinema.js` 477–511) already ignores non-inspectable hits, the reticle, and invisible ancestors, then accepts `userData.ruleRecords[instanceId]` or the nearest `userData.inspectable` parent. Restricting the raycast to the tagged roots produces the same winner: occluders that are not inspectable never won. There are 17 `tagInspectable` call sites; the terrace loop adds 8 light rings and the frame loop adds one root per wafer. Recursive tests still cover the children of those roots, which is what picking the package, a half-arc, or a floor instance requires.

`intersectObjects` allocates a new array when the third argument is omitted, then sorts it (`Raycaster.js` 218–226). Reuse one array. Intersection records inside Three.js are still allocated per hit; fewer roots means fewer of those.

The throttle drops the pointer sample that falls inside the 60 ms window if the pointer then stops. The hover target stays on the previous sample. A single trailing timer fires one raycast at the rested coordinates and preserves the 16 Hz ceiling while the pointer is moving. Copy `clientX` / `clientY` into numbers. Pooled `PointerEvent` objects are not safe to retain. Clear the timer in `dispose` (`designer7_cinema.js` 912–921).

### `theme.js` — replace `tagInspectable` (436–450)

```javascript
const inspectableRoots = [];

export function getInspectableRoots() {
  return inspectableRoots;
}

export function clearInspectableRoots() {
  inspectableRoots.length = 0;
}

export function tagInspectable(object3d, meta) {
  object3d.userData = {
    ...object3d.userData,
    inspectable: true,
    ...meta,
  };
  if (object3d.userData.__inspectListed !== true) {
    object3d.userData.__inspectListed = true;
    inspectableRoots.push(object3d);
  }
  object3d.traverse((child) => {
    if (child !== object3d) {
      child.userData = {
        ...child.userData,
        inspectableParent: object3d,
      };
    }
  });
  return object3d;
}
```

Call `clearInspectableRoots()` from `SherlockStage.dispose` (`SherlockStage.js` 367). One stage is constructed (`app.js` 106). The registry is module-scope, so a disposed stage must not leave roots in the list.

### `designer7_cinema.js` — replace the raycast block (536–555 and 631–648)

```javascript
  import { getInspectableRoots } from './theme.js';

  const _rayHits = [];
  const _rayRoots = [];
  let raycastTimer = 0;
  let pendingClientX = 0;
  let pendingClientY = 0;

  function hierarchyVisible(obj) {
    let cur = obj;
    while (cur) {
      if (cur.visible === false) return false;
      cur = cur.parent;
    }
    return obj != null;
  }

  function performRaycast() {
    if (!camera || !pointerInside || isDragging) return null;
    raycaster.setFromCamera(pointerNDC, camera);

    const roots = getInspectableRoots();
    _rayRoots.length = 0;
    const source = roots.length > 0 ? roots : (scene ? scene.children : _rayRoots);
    for (let i = 0; i < source.length; i++) {
      if (hierarchyVisible(source[i])) _rayRoots.push(source[i]);
    }

    _rayHits.length = 0;
    raycaster.intersectObjects(_rayRoots, true, _rayHits);
    const target = findInspectableTarget(_rayHits);

    if (target !== hoveredObject) {
      hoveredObject = target;
      if (!pinnedObject) emitInspect(hoveredObject, false);
    }
    if (domElement) {
      domElement.style.cursor = isDragging
        ? 'grabbing'
        : hoveredObject
          ? 'pointer'
          : 'grab';
    }
    return target;
  }

  function samplePointer(clientX, clientY) {
    if (!domElement) return;
    const rect = domElement.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) return;
    pointerNDC.x = ((clientX - rect.left) / rect.width) * 2 - 1;
    pointerNDC.y = -((clientY - rect.top) / rect.height) * 2 + 1;
  }

  function runThrottledRaycast(clientX, clientY) {
    if (clientX === lastRaycastClientX && clientY === lastRaycastClientY) return;
    const now = performance.now();
    const elapsed = now - lastRaycastTime;
    if (elapsed < RAYCAST_THROTTLE_MS) {
      pendingClientX = clientX;
      pendingClientY = clientY;
      if (raycastTimer !== 0) return;
      raycastTimer = setTimeout(() => {
        raycastTimer = 0;
        if (!pointerInside || isDragging) return;
        lastRaycastTime = performance.now();
        lastRaycastClientX = pendingClientX;
        lastRaycastClientY = pendingClientY;
        samplePointer(pendingClientX, pendingClientY);
        performRaycast();
      }, RAYCAST_THROTTLE_MS - elapsed);
      return;
    }
    lastRaycastTime = now;
    lastRaycastClientX = clientX;
    lastRaycastClientY = clientY;
    samplePointer(clientX, clientY);
    performRaycast();
  }
```

`onPointerMove` (the non-drag path, lines 631–648) becomes `runThrottledRaycast(event.clientX, event.clientY)`. `onPointerUp` keeps its immediate `performRaycast` for clicks. `dispose` adds `if (raycastTimer) clearTimeout(raycastTimer)`.

Leave `raycaster.params.Line.threshold` at `0.015` (`designer7_cinema.js` 420). Leave `first-hit` behavior sorted by distance. Three r186 `Raycaster` has no `firstHitOnly` flag; the sort in `intersectObjects` is what makes the nearest inspectable win.

Do not raycast from `cinema.update` (`designer7_cinema.js` 763). A stationary pointer during an orbit can go stale; if that is fixed later, it has to share `lastRaycastTime` and the same root list so the budget stays 16 Hz plus one trailing call.

## Finding 5 — Zero-allocation frame body, and the allocations that are already gone

`render` does not call `new`. The remaining per-frame garbage and redundant writes are smaller than the shadow pass and still real.

### 5a. `Object.assign` retains stale keys and walks every field

`SherlockStage.js` 301–310:

```javascript
    if (stepState && typeof stepState === 'object') {
      fullState.semanticTransition = stepState.semanticTransition ?? null;
      fullState.ruleLine = stepState.ruleLine ?? null;
      Object.assign(fullState, stepState);
    }
```

`buildStepState` (`sherlockEngine.js` 2185–2199) returns a fixed set: `ambient`, `dt`, `stepIndex`, `stepFraction`, `totalSteps`, `progress`, `phase`, `aperture`, `recursion`, `ringTelemetry`, `controlCore`, `trace`, `step`. `app.js` 937–938 also sets `presetId` and `source` on that object. `Object.assign` copies those references and **leaves any key that a previous object had and this one does not**. The two lines above the assign are overwritten when the property exists, and left in place when it does not. Copy the contract. The stage then overwrites `ambient`, `dt`, `activePreset`, and `cutawayMix` as it already does.

```javascript
    const fullState = this._fullState;
    if (stepState && typeof stepState === 'object') {
      fullState.stepIndex = stepState.stepIndex ?? 0;
      fullState.stepFraction = stepState.stepFraction ?? 0;
      fullState.totalSteps = stepState.totalSteps ?? 0;
      fullState.progress = stepState.progress ?? 0;
      fullState.phase = stepState.phase ?? 'STILL';
      fullState.aperture = stepState.aperture ?? 0;
      fullState.recursion = stepState.recursion ?? null;
      fullState.ringTelemetry = stepState.ringTelemetry ?? null;
      fullState.controlCore = stepState.controlCore ?? null;
      fullState.trace = stepState.trace ?? null;
      fullState.step = stepState.step ?? null;
      fullState.presetId = stepState.presetId ?? null;
      fullState.source = stepState.source ?? null;
      fullState.semanticTransition = stepState.semanticTransition ?? null;
      fullState.ruleLine = stepState.ruleLine ?? null;
    }
    fullState.ambient = this.ambient;
    fullState.dt = dt;
    fullState.activePreset = activePreset;
    fullState.cutawayMix = this._cutawayMix;
```

No object is allocated. `_fullState`'s shape stays stable after the first frame.

Prefer `stepState.dt` when it is a finite number, and only sample `performance.now()` when it is not. `app.js` 948 already computed `dt` and stored it on the step state; `render` currently ignores it and reads the clock again (`SherlockStage.js` 289–291), then overwrites `fullState.dt`. One clock keeps the stage and the playhead on the same delta. The local clamp stays for callers that pass no state:

```javascript
    const incomingDt = stepState && typeof stepState.dt === 'number' ? stepState.dt : NaN;
    const now = performance.now();
    const dt = Number.isFinite(incomingDt)
      ? Math.min(0.05, Math.max(0.001, incomingDt))
      : Math.min(0.05, Math.max(0.001, (now - this.lastTime) / 1000));
    this.lastTime = now;
```

### 5b. Light uniforms are stepwise, but they are written every frame

`SherlockStage.js` 321–351 sets `wellPointLight` color and intensity, and the two vault intensities, on every frame. The color is a function of `phase` and `GATE_RULE_LINES` only. Vault intensity is a function of the preset name only. `setHex` + `color.copy` of the same value still runs. The GPU light uniform is refreshed by Three.js on render either way; this removes the CPU writes and keeps `_scratchColor` free for subsystems in the same turn.

```javascript
    const phase = fullState.semanticTransition || fullState.phase || 'DESCEND';
    const rawRule =
      fullState.ruleLine ??
      fullState.step?.rule?.line ??
      fullState.step?.ruleLine ??
      fullState.step?.after?.ruleLine ??
      0;
    const ruleNum = typeof rawRule === 'number' ? rawRule : Number(rawRule);

    let lightMode = 0;
    let lightHex = 0xfbbf24;
    let lightIntensity = 1.45;
    if (phase === 'ERROR') {
      lightMode = 1;
      lightHex = 0xe11d48;
      lightIntensity = 1.65;
    } else if (phase === 'GATE' || GATE_RULE_LINES.has(ruleNum)) {
      lightMode = 2;
      lightHex = 0x818cf8;
      lightIntensity = 1.5;
    } else if (
      phase === 'ENV' ||
      phase === 'RETURN' ||
      phase === 'RESOLVE' ||
      phase === 'EMBLEM'
    ) {
      lightMode = 3;
      lightHex = 0x2dd4bf;
      lightIntensity = 1.5;
    }
    if (lightMode !== this._lightMode) {
      this._lightMode = lightMode;
      this._scratchColor.setHex(lightHex);
      this.wellPointLight.color.copy(this._scratchColor);
      this.wellPointLight.intensity = lightIntensity;
    }

    const inHollow =
      activePreset === 'interior' ||
      activePreset === 'interior_up' ||
      activePreset === 'interior_floor' ||
      activePreset === 'frames';
    const vaultMode = inHollow ? 1 : 0;
    if (vaultMode !== this._vaultMode && this.vaultUpperLight && this.vaultFloorLight) {
      this._vaultMode = vaultMode;
      const vaultBoost = inHollow ? 1.95 : 1.0;
      this.vaultUpperLight.intensity = 1.25 * vaultBoost;
      this.vaultFloorLight.intensity = 1.55 * vaultBoost;
    }
```

Hex values, intensities, and the hollow preset list match lines 321–351. `GATE_RULE_LINES` is unchanged.

### 5c. Texture helpers: the GPU upload is already cached; keep it that way

`createBrushedMetalTexture`, `createCeramicGrainTexture`, and `createReticleTexture` pay the canvas cost once per key (`theme.js` 153–156, 222–230, 288–291). Call sites use a handful of keys (`designer1_package.js` 126–130 size 512 / alpha 0.07; `designer2_well.js` 1353–1357 size 256 / alpha 0.08; `designer3_frames.js` 359–363 size 256 / alpha 0.04). `#d8dde6` is rewritten to `#f4f6fa` (`theme.js` 149–151) so a legacy base shares the high-key map. That rewrite is a quality fix and should stay.

On a hit, the metal and reticle paths call `Map.has` and then `Map.get`. Use one `get`. Do not add an LRU `dispose()` to these maps: the returned texture is mounted on a material for the life of the stage, and the key space is finite.

```javascript
  const cached = metalTextureCache.get(key);
  if (cached) return cached;
```

Same pattern at `theme.js` 229 and 289.

Plaque hits already skip the canvas (`theme.js` 350–356). They still build the key string and reshuffle the `Map` for LRU order. That is cheap next to a canvas upload. Callers must keep calling it only when the label changes (Designer 2 already does, at `designer2_well.js` 2633). A second fast path avoids the reshuffle when the same key arrives twice in a row:

```javascript
let lastPlaqueKey = '';
let lastPlaqueTex = null;

// inside createPlaqueTexture, after cacheKey is built:
  if (cacheKey === lastPlaqueKey && lastPlaqueTex) return lastPlaqueTex;
```

Set `lastPlaqueKey` / `lastPlaqueTex` on both the hit path and the miss path. Do not `dispose()` inside the `MAX_PLAQUE_CACHE_SIZE` branch (`theme.js` 412–418). An evicted entry can still be the `map` on a live material. Disposing it clears a plaque that is on screen. The current comment at 415 is the right policy.

`anisotropy` stays 8 on metal and ceramic (`theme.js` 203, 271) and 4 on plaques (`theme.js` 408). Those values select mip taps; lowering them softens grazing text and brush streaks. `CanvasTexture` already uses mipmaps (`LinearMipmapLinearFilter` is the `Texture` default). Leave `generateMipmaps` on. The env canvas is consumed by PMREM, which allocates its own filtered chain; do not set the equirect's anisotropy in an attempt to sharpen IBL.

`SCRATCH_COLOR` through `SCRATCH_OBJ3D` (`theme.js` 104–109) are the right pattern. Do not pass them by reference into `material.color` or `object.position`. `SherlockStage._scratchColor` is a separate instance, which keeps the light write from aliasing a subsystem scratch. Designers currently keep private scratches and do not import these. Leave the exports.

### 5d. `updatePlaqueTexture` for slots that repaint

When a designer does change a plaque, `createPlaqueTexture` allocates a new `CanvasTexture` per unique string until the LRU drops the key. Prefer repainting one texture per slot when the caller already holds one. Same draw commands as `createPlaqueTexture` (lines 365–410). New texture only if `existing` is empty.

```javascript
export function updatePlaqueTexture(existing, options) {
  const cacheKey = plaqueCacheKey(options); // the key expression at theme.js 344–348
  if (existing && existing.userData && existing.userData.plaqueKey === cacheKey) {
    return existing;
  }
  const fresh = createPlaqueTexture(options);
  if (existing && existing.image && fresh.image && existing !== fresh) {
    const src = fresh.image;
    const ctx = existing.image.getContext('2d');
    if (ctx && src.width === existing.image.width && src.height === existing.image.height) {
      ctx.clearRect(0, 0, src.width, src.height);
      ctx.drawImage(src, 0, 0);
      existing.userData.plaqueKey = cacheKey;
      existing.needsUpdate = true;
      return existing;
    }
  }
  if (fresh) fresh.userData.plaqueKey = cacheKey;
  return fresh;
}
```

Factor the key at 348 into `plaqueCacheKey` so the two functions cannot drift. `needsUpdate = true` is a one-shot version bump (`Texture.js` 758), not a per-frame upload. This preserves the glyphs (same canvas drawing) and uploads only the slot texture the mesh already references.

## Finding 6 — Static light list stays at eight

`_setupLights` (`SherlockStage.js` 140–193`) is one hemisphere, three directionals (only the key casts), and four points:

| Light | Intensity | Distance / decay | Role |
|---|---|---|---|
| Hemisphere `0xfffbf2` / `0xc9c2b4` | 0.52 | — | studio fill under IBL |
| Key `0xfff7ea` | 2.05 | shadow frustum above | daylight and the only shadow |
| Fill `0xd5e3f8` | 0.72 | — | cool specular on stainless |
| Rim `0xffeed8` | 0.62 | — | champagne graze |
| `vaultUpperLight` | 1.25, ×1.95 in the bore | 5.2 / 1.6 | underside of the rings |
| `wellPointLight` | 1.45–1.65 | 6.0 / 1.6 | semantic phase color |
| `vaultFloorLight` | 1.55, ×1.95 in the bore | 5.5 / 1.6 | floor bounce |
| `coreSpecLight` | 0.95 | 3.2 / 2.0 | core specular |

Removing any of these changes the bore, the stainless, or the floor. Finding 5b already stops the redundant intensity writes. Distances already limit the points. No further light-loop change keeps the current shading.

## Apply order

1. Finding 1 shadow flag and `freezeStaticObject3D`. Highest GPU cost, no pixel change on settled casters.
2. Finding 2 hidden-tab skip and resize coalesce.
3. Finding 3 PMREM bake before `ready = true`, plus `dispose` of `_envTarget`.
4. Finding 4 inspectable-root raycast and trailing timer.
5. Finding 5 field copy, light mode cache, plaque repaint helper.

## Verification that does not require a visual diff to be trusted

- Pause playback, orbit for two seconds, and confirm `keyLight.shadow.needsUpdate` is `false` on subsequent frames while `renderer.render` still runs. Resume playback and confirm it stays `true`.
- Toggle the frames preset and confirm `needsUpdate` stays `true` until `0.5 s` after `_cutawayMix` has snapped to `0` or `1` (`SherlockStage.js` 297–298).
- Hover a floor tile, a half-arc, and empty bench. The bench must not become a hit. The floor instance id must still resolve through `ruleRecords` (`designer7_cinema.js` 495–497).
- After init, `scene.environment.mapping` is `CubeUVReflectionMapping` and `scene.environment.isPMREMTexture === true`. `environmentIntensity` is still `0.68`. Background is still `0xf4f0e8`.
- Hide the tab during playback, return, and confirm the playhead pose matches a throttled background update, with one fresh shadow frame (`_shadowForce`).
