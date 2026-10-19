# Performance audit #6 — Designer 5 Circuit, Designer 6 Loom, Designer 7 Cinema

**Scope:** `src/designers/designer5_circuit.js` (274 lines), `src/designers/designer6_loom.js` (197 lines), `src/designers/designer7_cinema.js` (933 lines). Caller context checked in `src/stage/SherlockStage.js` so the recommendations match how `update()` is actually invoked. No source files were modified.

**Rule for every item below:** the pixels stay the same. No resolution drop, no shadow-map shrink, no material swap, no animation slowdown, no change to which LED is lit, no change to which object the reticle picks. Each item removes work whose result is already on screen or is below a fraction of a pixel.

**Renderer:** `three@0.175` via `three/webgpu` (`WebGPURenderer`). `BufferAttribute.usage` (`StaticDrawUsage` / `DynamicDrawUsage`) is ignored for vertex and instance buffers: `WebGPUBackend` always creates them as `VERTEX | COPY_SRC | COPY_DST` (`node_modules/three/src/renderers/webgpu/WebGPUBackend.js` around the `createAttribute` calls). Changing `setUsage` in these three files does not move GPU memory. Recommendations below do not depend on that hint.

---

## Already in good shape

These paths are settled. Leave them.

| Site | What it already does |
| :--- | :--- |
| Designer 5, lines 171–194 | Left-flank `instanceColor` uploads only when `exprCount` or `activeRowIdx` changes, and `setColorAt` only for slots whose hex differs (`lastLeftHex`). |
| Designer 5, lines 241–251 | Right-flank upload is gated by `lastRightHex`. Unchanged limbs do not call `setColorAt`. |
| Designer 5, lines 26–31 and 99–128 | Scratch `Matrix4` / `Vector3` / `Color`. Instance matrices are written once at init. Bounding spheres are computed once. |
| Designer 6, lines 113–176 | Segment layout runs only when `segmentCount` or `sliceWidth` changes. Colors run only when `activeSegment` or `phase` changes. Per-slot hex cache. No per-frame string work on a steady step. |
| Designer 6, lines 19–24 | Scratch math at module scope. |
| Designer 7, lines 109–119, 419–433 | Scratch vectors, one `Raycaster`, pointer NDC stored on a single `Vector2`. |
| Designer 7, lines 631–648 | Picking is skipped when the pointer has not moved, and at most once per 60 ms. |
| Designer 7, lines 773–778 | `updateProjectionMatrix` runs only while `\|fov − desiredFov\| > 0.05`. |
| `SherlockStage.js` 353–356 | Hidden layers skip `update`, except `cinema` (the camera rig has to keep running). |
| `SherlockStage.js` 294–299 | `cutawayMix` snaps to 0 or 1 once it is within 0.005 of the target, so the frames-preset retract finishes instead of creeping forever. |

Circuit and loom appearance is a pure function of the latest `stepState` (plus `ambient` for the amber ripple). One call with the current state reproduces the exact LEDs. Skipping calls while a layer is hidden, then running once on the frame it is shown, matches the uninterrupted result.

---

## 1. Skip `update` while a designer group is hidden

### Designer 5 — `update`, line 154

`SherlockStage.setLayerVisible` (`SherlockStage.js` 236–241) writes `group.visible` and `layerVisibility` together, and the stage loop already skips a hidden key. `createCircuitSubsystem().update` itself never reads `group.visible`. Any direct caller (tests, a future loop that updates every subsystem) still walks both flanks. The ripple path below is the expensive one, and it is pure in `ambient`, so a skipped frame does not need to be replayed.

Insert at the top of `update`, immediately after the null check on line 155:

```js
function update(stepState) {
  if (!stepState) return;
  if (group.visible === false) return;
  const ambient = stepState.ambient ?? 0;
  // ... rest unchanged
}
```

### Designer 6 — `update`, line 97

Same gap. The bar is a pure function of `totalSteps`, `trace.steps.length`, `trace.sliceWidth`, `stepIndex`, and `phase`. A hidden frame produces the same meshes as a frame that ran.

```js
function update(stepState = {}) {
  if (group.visible === false) return;
  const rawSteps = stepState.trace?.steps;
  // ... rest unchanged
}
```

### Designer 7 — do not skip the whole `update`

`SUBSYSTEM_KEYS` forces `cinema` to run even when its layer is hidden (`SherlockStage.js` 355), because `update` owns the camera. The cinema group only holds the reticle (`designer7_cinema.js` 242–249). When that group is hidden, the camera block (lines 770–849) stays, and the reticle block should not run and should not flip the child back to `visible = true` (lines 891 and 899).

At line 851, before the pinned-object checks:

```js
if (group.visible === false) {
  if (reticle.group.visible) reticle.group.visible = false;
  return;
}
```

Hover and pin state are left intact, so showing the layer again restores the same reticle on the next frame. Camera motion for that frame has already been applied above the new return.

---

## 2. Replace the scene-wide pick walk with a cached mesh list

### What the file does today

There is no `scene.traverse()` call in `designer7_cinema.js`. Two library walks do the same job.

**Pick walk, every throttled pointer move.** Lines 536–539:

```536:539:src/designers/designer7_cinema.js
  function performRaycast() {
    if (!camera || !scene || !pointerInside || isDragging) return null;
    raycaster.setFromCamera(pointerNDC, camera);
    const hits = raycaster.intersectObjects(scene.children, true);
```

`Raycaster.intersectObjects(..., true)` recurses through every descendant (`node_modules/three/src/core/Raycaster.js` 218–258). `Mesh.raycast` does not consult `object.visible` (`Mesh.js` 238–259). Hidden layers, lights, groups, the retracted south wall, and the quineling meshes are all visited. `findInspectableTarget` (lines 477–511) then discards hits that are invisible or that have no `userData.inspectable`, `inspectableParent`, or `ruleRecords`. Decorative meshes never occlude an inspectable: the loop keeps going until the closest tagged hit. A flat list of those tagged meshes, tested with `recursive = false`, returns the same target.

Called from `onPointerMove` (lines 644–648, ≤ 16 Hz) and from a click in `onPointerUp` (line 668). It is not on the animation clock. While the pointer is moving it is the dominant CPU cost in this file, because the scene contains the package, the 12 half-arcs, the 39-rule floor, and the quineling organism.

**Bounds walk, on step change, and every frame if the box stays empty.** Lines 864–876:

```864:876:src/designers/designer7_cinema.js
      if (
        activeInspectTarget !== lastBoxTarget ||
        currentStepIdx !== lastBoxStepIndex ||
        rawBox.isEmpty()
      ) {
        if (activeInspectTarget.customBox) {
          rawBox.copy(activeInspectTarget.customBox);
        } else {
          rawBox.setFromObject(activeInspectTarget);
          if (!rawBox.isEmpty()) {
            rawBox.expandByScalar(0.018);
          }
        }
```

`Box3.setFromObject` → `expandByObject` walks every child and calls `updateWorldMatrix(false, false)` (`Box3.js` 303–384). The `rawBox.isEmpty()` clause retries that walk on every animation frame for a target that has no geometry. Floor-rule proxies are safe: they carry `customBox` (lines 454–457) and never enter `setFromObject`.

Build the pick list lazily. `createCinemaSubsystem` runs before `SherlockStage` parents the other groups (`SherlockStage.js` 196–221). A traverse inside the factory sees an empty scene. The first `performRaycast` is after the graph exists. The graph is not rebuilt at runtime (species changes toggle `visible`); the visibility filter covers that. Call `rebuildPickables()` if a future change adds or removes meshes.

### Replacement

Module-scope scratch next to the other cinema scratch (after line 119), plus the list inside `createCinemaSubsystem` next to the raycaster (after line 422):

```js
const _pickHits = [];
const _boundsScratch = new THREE.Box3();

// inside createCinemaSubsystem, after `const pointerNDC = new THREE.Vector2();`
const pickables = [];
let pickablesReady = false;

function rebuildPickables() {
  pickables.length = 0;
  if (!scene) {
    pickablesReady = true;
    return;
  }
  scene.traverse((obj) => {
    if (!obj.geometry) return;
    if (obj.userData?.isInspectorReticle) return;
    let cur = obj;
    let tagged = false;
    while (cur) {
      if (cur.userData?.inspectable || cur.userData?.ruleRecords) {
        tagged = true;
        break;
      }
      cur = cur.parent;
    }
    if (tagged || obj.userData?.inspectableParent) pickables.push(obj);
  });
  pickablesReady = true;
}

const visiblePickables = [];
function collectVisiblePickables() {
  if (!pickablesReady) rebuildPickables();
  visiblePickables.length = 0;
  for (let i = 0; i < pickables.length; i++) {
    const obj = pickables[i];
    if (isHierarchyVisible(obj)) visiblePickables.push(obj);
  }
  return visiblePickables;
}
```

`isHierarchyVisible` is defined later in the same function (lines 747–754). Hoist that function above `collectVisiblePickables`, or keep the helper below and only call it from `performRaycast` (the closure already allows that).

Replace the body of `performRaycast` (lines 536–555):

```js
function performRaycast() {
  if (!camera || !scene || !pointerInside || isDragging) return null;
  raycaster.setFromCamera(pointerNDC, camera);
  _pickHits.length = 0;
  const hits = raycaster.intersectObjects(collectVisiblePickables(), false, _pickHits);
  const target = findInspectableTarget(hits);

  if (target !== hoveredObject) {
    hoveredObject = target;
    if (!pinnedObject) {
      emitInspect(hoveredObject, false);
    }
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
```

`intersectObjects` appends. Clearing `_pickHits` first keeps click and hover from accumulating stale hits. `recursive = false` is required: the list is already the leaves. Instance ids still arrive on the hit, so `ruleRecords[hit.instanceId]` (lines 495–497) is unchanged. Meshes whose `raycast` is a no-op (`designer3_frames.js` spine) stay cheap if they remain in the list.

### Cached bounds for `setFromObject`

Same visual box as `expandByObject` with `precise = false`: geometry bounding box transformed by `matrixWorld`, united over descendants that have geometry. `customBox` stays the fast path and is not expanded (current code does not call `expandByScalar` on it).

```js
const boundsListCache = new WeakMap();

function boundsMeshesFor(target) {
  let meshes = boundsListCache.get(target);
  if (meshes) return meshes;
  meshes = [];
  target.traverse((child) => {
    if (child.geometry) meshes.push(child);
  });
  boundsListCache.set(target, meshes);
  return meshes;
}

function setBoundsFromTarget(target, box) {
  if (target.customBox) {
    box.copy(target.customBox);
    return;
  }
  const meshes = boundsMeshesFor(target);
  box.makeEmpty();
  for (let i = 0; i < meshes.length; i++) {
    const mesh = meshes[i];
    const geometry = mesh.geometry;
    if (geometry.boundingBox === null) geometry.computeBoundingBox();
    mesh.updateWorldMatrix(false, false);
    _boundsScratch.copy(geometry.boundingBox).applyMatrix4(mesh.matrixWorld);
    box.union(_boundsScratch);
  }
  if (!box.isEmpty()) box.expandByScalar(0.018);
}
```

Replace the refresh condition at lines 864–879 so an empty result is remembered and not recomputed every frame:

```js
const currentStepIdx = stepState.stepIndex ?? 0;
if (
  activeInspectTarget !== lastBoxTarget ||
  currentStepIdx !== lastBoxStepIndex
) {
  setBoundsFromTarget(activeInspectTarget, rawBox);
  lastBoxTarget = activeInspectTarget;
  lastBoxStepIndex = currentStepIdx;
}
```

Drop `|| rawBox.isEmpty()` from the condition. A target with no geometry traverses once, caches an empty mesh list, and then only hides the reticle (the branch at lines 898–900). A target whose matrices animate between steps still refreshes on `stepIndex` changes, which is the current policy, using the cached mesh list instead of a fresh walk. Child lists are stable for the life of an inspectable; the `WeakMap` drops them with the object.

`updateWorldMatrix(false, false)` matches `Box3.expandByObject`. It uses the parent `matrixWorld` already on the object. Cinema `update` runs before `renderer.render`, which is also when today’s `setFromObject` runs, so the box stays on the same frame of latency as the current reticle.

---

## 3. Settle the reticle material, and settle the camera when the damping has arrived

### 3a. Reticle opacity and color — lines 215–217

`updateGeometryFromBox` runs on every animation frame the reticle is up (line 892), because the hairline pulse is `sin(ambient * 4)` (line 171). That pulse is the drawn motion. Keep writing `cornerPositions` and keep `position.needsUpdate = true` while the reticle is visible. Freezing the buffer would stop the pulse.

The material write under it is not motion. Pinned and unpinned are two constants:

```215:217:src/designers/designer7_cinema.js
    cornerGeo.attributes.position.needsUpdate = true;
    cornerMat.color.setHex(pinned ? PALETTE.heatBright : PALETTE.goldBright);
    cornerMat.opacity = pinned ? 0.96 : 0.84;
```

`Color.setHex` (three `Color.js` 204–214) rewrites `r, g, b` and runs `colorSpaceToWorking` on every call, including when the hex is already installed. `cornerMat` is a transparent `LineBasicMaterial` (lines 143–148). WebGPU maps that class to a node material (`three.webgpu.js` registers `LineBasicNodeMaterial` as `LineBasicMaterial`). Writing `color` and `opacity` every frame dirties the material uniform even though the on-screen value is one of two constants.

There is no cutaway opacity lerp in designers 5, 6, or 7. The frames preset (lines 55–62) is what makes the stage drive `cutawayMix` (`SherlockStage.js` 295). That mix is position and visibility in designers 1 and 2, and the stage already snaps it (lines 297–299). The continuous material churn inside this audit’s files is the reticle pair above. Settle it the same way: write once per state, then leave the material alone.

```js
let lastReticlePinned = null;

function updateGeometryFromBox(min, max, ambient = 0, pinned = false) {
  // ... existing cornerPositions loop, unchanged ...
  cornerGeo.attributes.position.needsUpdate = true;
  const pinnedVisual = pinned ? 1 : 0;
  if (pinnedVisual !== lastReticlePinned) {
    lastReticlePinned = pinnedVisual;
    cornerMat.color.setHex(pinned ? PALETTE.heatBright : PALETTE.goldBright);
    cornerMat.opacity = pinned ? 0.96 : 0.84;
  }
}
```

Gate the visibility flag the same way at lines 891, 899, and 902 so a held hover does not assign `visible` every frame:

```js
if (!reticle.group.visible) reticle.group.visible = true;
// ...
if (reticle.group.visible) reticle.group.visible = false;
```

Hairline geometry, pulse amplitude (0.005 free, 0.0025 pinned), gold versus `heatBright`, and opacities 0.84 / 0.96 stay exactly as written.

While the smoothed box is still easing (lines 886–890), the vertex rewrite stays. Once `smoothMin` / `smoothMax` have reached `rawBox` and only the pulse remains, the upload is 48 vertices × 3 floats (576 bytes) per frame. That upload is the pulse. A node `positionNode` that adds `pulse * cornerSign` in the vertex shader would move the same sine off the CPU buffer; it is a behavior-preserving follow-up, and it needs a `LineBasicNodeMaterial` owned by this file so the classic material path cannot drop the node. The settle above is the change that removes uniform churn with the current material.

Also collapse the repeated parent walks at lines 852–862. `isHierarchyVisible` is called up to four times per frame on the same two objects:

```js
const pinVis = pinnedObject ? isHierarchyVisible(pinnedObject) : false;
const hoverVis = hoveredObject ? isHierarchyVisible(hoveredObject) : false;
if (pinnedObject && !pinVis) {
  pinnedObject = null;
  emitInspect(hoverVis ? hoveredObject : null, false);
}
if (hoveredObject && !hoverVis) hoveredObject = null;

const activeInspectTarget = pinnedObject || hoveredObject;
if (activeInspectTarget && (activeInspectTarget === pinnedObject ? pinVis : hoverVis)) {
  // existing reticle body
}
```

Re-read visibility after clearing a dead pin, because the `emitInspect` line uses the hover object that the next `if` may clear. The snippet above matches the current order: the emit sees the pre-clear hover visibility, then a dead hover is dropped, then the reticle uses whatever survived.

### 3b. Camera damping — lines 770–848

Exterior orbit lerps radius, phi, theta, and the look target every frame (lines 815–818) and then calls `camera.lookAt` (line 847). `Object3D.lookAt` rewrites the quaternion, which fires the quaternion `_onChange` hook and recomposes the Euler (`Object3D.js` 151–158). Interior mode does the same with a second `lookAt` (lines 784–799). The exponential lerp never reaches the target, so a rested preset still rewrites the camera forever.

`cinema` is a continuous shot (lines 805–813: theta advance, phi sine, radius cosine). That branch must keep writing every frame. Dragging, wheel zoom, and a non-zero `transitionArcBoost` (line 820) also keep writing.

Snap the other presets once the remaining error is far below a pixel, write that exact pose once, and skip `lookAt` until the desired pose moves.

Pixel scale, vertical, 800 px: world units per pixel ≈ `2 * tan(fov/2) * distance / 800`.

| View | Distance | fov | Units per pixel |
| :--- | :--- | :--- | :--- |
| Overview | ~7 | 32° | ~0.005 |
| Interior, subject 0.5 away | 0.5 | 72° | ~0.0009 |

An angular epsilon of `1e-4` rad at radius 7 is an arc of `7e-4` (about 0.14 px). A position epsilon of `1e-4` is about 0.1 px in the interior and about 0.02 px outside. Snapping inside those epsilons does not move a pixel. The snap also removes the ≤ 0.05° fov residue left by the gate on line 773, by assigning `desiredFov` once.

```js
const SETTLE_ANG = 1e-4;
const SETTLE_POS_SQ = 1e-8; // (1e-4)^2
let exteriorHold = false;
let interiorHold = false;

function writeExteriorPose() {
  transitionArcBoost = Math.max(0, transitionArcBoost - dt * 2.4);
  const arcLiftY = Math.sin(transitionArcBoost * Math.PI) * 0.28;
  _camOffset.setFromSphericalCoords(currentRadius, currentPhi, currentTheta);
  camera.position.copy(currentTarget).add(_camOffset);
  camera.position.y += arcLiftY;
  if (
    currentPresetName !== 'frames' &&
    currentTarget.y < -0.60 &&
    camera.position.y < -0.22 &&
    camera.position.z < 0.75
  ) {
    const horizR = Math.hypot(camera.position.x, camera.position.z);
    if (horizR > 0.94) {
      const clampScale = 0.94 / horizR;
      camera.position.x *= clampScale;
      camera.position.z *= clampScale;
    }
    camera.position.y = Math.max(camera.position.y, -3.22);
  }
  camera.lookAt(currentTarget);
}

function writeInteriorPose() {
  const cosPitch = Math.cos(interiorPitch);
  _interiorLookDir.set(
    Math.sin(interiorYaw) * cosPitch,
    Math.sin(interiorPitch),
    -Math.cos(interiorYaw) * cosPitch
  );
  camera.position.copy(interiorEye);
  _interiorLookTarget.copy(interiorEye).add(_interiorLookDir);
  camera.lookAt(_interiorLookTarget);
  currentTarget.copy(_interiorLookTarget);
  desiredTarget.copy(_interiorLookTarget);
}

// inside update(), replacing the `if (camera)` body from line 770
if (camera) {
  const damp = 1.0 - Math.exp(-12.0 * dt);

  if (typeof camera.fov === 'number') {
    const fovGap = Math.abs(camera.fov - desiredFov);
    if (fovGap > 0.05) {
      camera.fov = lerp(camera.fov, desiredFov, damp);
      camera.updateProjectionMatrix();
    } else if (camera.fov !== desiredFov) {
      camera.fov = desiredFov;
      camera.updateProjectionMatrix();
    }
  }

  if (isInteriorPresetName(currentPresetName)) {
    exteriorHold = false;
    clampInteriorEye(desiredInteriorEye);
    const eyeSq = interiorEye.distanceToSquared(desiredInteriorEye);
    const ang =
      Math.abs(interiorYaw - desiredInteriorYaw) +
      Math.abs(interiorPitch - desiredInteriorPitch);
    if (eyeSq < SETTLE_POS_SQ && ang < SETTLE_ANG) {
      if (!interiorHold) {
        interiorEye.copy(desiredInteriorEye);
        interiorYaw = desiredInteriorYaw;
        interiorPitch = desiredInteriorPitch;
        writeInteriorPose();
        interiorHold = true;
      }
    } else {
      interiorHold = false;
      interiorEye.lerp(desiredInteriorEye, damp);
      clampInteriorEye(interiorEye);
      interiorYaw = lerp(interiorYaw, desiredInteriorYaw, damp);
      interiorPitch = lerp(interiorPitch, desiredInteriorPitch, damp);
      writeInteriorPose();
    }
  } else {
    interiorHold = false;
    if (currentPresetName === 'cinema' && !isDragging) {
      exteriorHold = false;
      desiredTheta += dt * 0.18;
      desiredPhi = 0.82 + Math.sin(ambient * 0.24) * 0.06;
      desiredRadius = 7.45 + Math.cos(ambient * 0.20 + progress * Math.PI) * 0.38;
      const phaseMode = resolvePhaseMode(stepState.phase, step.controlKind);
      const phaseTargetY = phaseMode === 1 ? -0.26 : phaseMode === 2 ? -0.10 : -0.16;
      desiredTarget.y = lerp(desiredTarget.y, phaseTargetY, dt * 1.6);
    }

    const ang =
      Math.abs(currentRadius - desiredRadius) +
      Math.abs(currentPhi - desiredPhi) +
      Math.abs(currentTheta - desiredTheta);
    const targetSq = currentTarget.distanceToSquared(desiredTarget);
    const resting =
      currentPresetName !== 'cinema' &&
      !isDragging &&
      transitionArcBoost <= 0 &&
      ang < SETTLE_ANG &&
      targetSq < SETTLE_POS_SQ;

    if (resting) {
      if (!exteriorHold) {
        currentRadius = desiredRadius;
        currentPhi = desiredPhi;
        currentTheta = desiredTheta;
        currentTarget.copy(desiredTarget);
        writeExteriorPose();
        exteriorHold = true;
      }
    } else {
      exteriorHold = false;
      currentRadius = lerp(currentRadius, desiredRadius, damp);
      currentPhi = lerp(currentPhi, desiredPhi, damp);
      currentTheta = lerp(currentTheta, desiredTheta, damp);
      currentTarget.lerp(desiredTarget, damp);
      writeExteriorPose();
    }
  }
}
```

`resolvePhaseMode` moves inside the cinema branch. It is unused for every other preset (today it runs at line 768 on every frame). Drag, wheel, and `setCameraPreset` change `desired*` or `transitionArcBoost`, the error leaves the epsilon, and `exteriorHold` clears on the next frame with no extra invalidation flag. The well clamp (lines 832–845) is preserved inside `writeExteriorPose` and runs on the snap frame, then the held pose already includes it.

---

## 4. Circuit and loom hot paths

### 4a. Designer 6 — stop drawing the zero-scale tail — lines 128–142

`progressInstanced` is allocated with `MAX_STEPS` (32) instances (line 61). Inactive slots are moved to `y = BAR_Y - 0.05` and scaled to `(0, 0, 0)` (lines 135–137). They are invisible and still in the instanced draw, because `InstancedMesh.count` stays 32. `computeBoundingSphere` (line 142) also folds those collapsed points into the cull sphere.

Set `count` to the live segment count before the bounding-sphere update. Instances at `i >= count` are neither drawn nor raycast (`InstancedMesh` uses `count` for both). The live segments keep the matrices written in the same loop, so the bar geometry is unchanged. With the usual 16-step trace this drops 16 degenerate boxes from the draw.

Inside the layout branch, after the matrix loop and before `computeBoundingSphere` (between lines 141 and 142):

```js
progressInstanced.count = segmentCount;
progressInstanced.instanceMatrix.needsUpdate = true;
progressInstanced.computeBoundingSphere();
```

`count` must be updated whenever `segmentCount` changes, including the first update. The existing `lastSegmentCount` gate already limits this to layout changes. Colors for `i >= segmentCount` can stay as they are; they are not fetched once `count` excludes them. Restoring a longer trace writes those matrices again in this same branch before raising `count`.

### 4b. Designer 5 — the ripple bucket does not throttle — lines 200–208

```200:208:src/designers/designer5_circuit.js
    const arith = after?.aux2?.arithmetic;
    const fuel = after?.aux2?.fuel;
    const arithActive = Boolean(arith?.active);
    const rippleIdx = ((ambient * 3.0) % 1.0) * (LIMB_BYTE_COUNT - 1);
    const rippleBucket = arithActive ? Math.floor(rippleIdx * 2) : -1;

    if (step !== lastStepRef || rippleBucket !== lastRippleBucket) {
```

`rippleIdx` advances at `ambient * 45` LED-units per second (`3.0 * 15`). `SherlockStage` steps `ambient` by `dt` capped at 0.05 s (stage line 290). At 60 fps, `dt ≈ 0.0167`, so `rippleIdx` moves ≈ 0.75 per frame and `floor(rippleIdx * 2)` moves ≈ 1.5 buckets per frame. The bucket test is true on essentially every frame while `arith.active` is set, and the 16-limb loop runs every frame.

The color that loop produces is binary. Lines 233–234:

```js
const nearRipple = clamp(1 - Math.abs(c - rippleIdx) / 2.5, 0, 1);
nextHex = nearRipple > 0.5 ? AMBER_RIPPLE_HEX : STANDBY_SAPPHIRE_SLOT_HEX;
```

`nearRipple > 0.5` iff `|c - rippleIdx| < 1.25`. Limb `c` is amber exactly on the open interval `(rippleIdx - 1.25, rippleIdx + 1.25)`. Membership changes when `rippleIdx` crosses `c ± 1.25`, which are spaced 0.5 apart, and nowhere in between. `floor(rippleIdx * 2)` changes on the integers and the halves (0, 0.5, 1, …), which is a different set of edges, so the bucket is neither a correct change detector nor a working throttle. Because the bucket flips every frame, the continuous `rippleIdx` is sampled every frame and the on-screen result is that sampling. A gate on the actual window preserves those samples and skips the scan on frames that do not cross `c ± 1.25`.

At 60 fps the step (0.75) is larger than the gap (0.5), so almost every frame still crosses and the skip rarely fires. At 120 fps the step is ≈ 0.375 and roughly a third of the frames fall inside a stable window. The replacement is exact at both rates; it is the high-refresh win. The `lastRightHex` upload gate stays, so a crossing that does not change a stored hex still skips `setColorAt`.

Do not add `addUpdateRange` on this buffer. A full `instanceColor` upload is 16 × 3 × 4 = 192 bytes (loom is 384 bytes). `WebGPUAttributeUtils.updateAttribute` issues one `writeBuffer` per range and does not merge them. Several ranged writes of a 192-byte buffer cost more than one write of the whole buffer. The existing “upload only when `colorChanged`” path is the right GPU policy.

Replace lines 149 and 200–252:

```js
let lastExprCount = -1;
let lastActiveRowIdx = -1;
let lastStepRef = undefined;
let lastRippleFirst = 0x7fffffff;
let lastRippleLast = 0x7fffffff;

// inside update(), section 2
const arith = after?.aux2?.arithmetic;
const fuel = after?.aux2?.fuel;
const arithActive = Boolean(arith?.active);

let rippleIdx = 0;
let rippleFirst = 0;
let rippleLast = -1;
if (arithActive) {
  rippleIdx = ((ambient * 3.0) % 1.0) * (LIMB_BYTE_COUNT - 1);
  rippleFirst = Math.floor(rippleIdx - 1.25) + 1;
  rippleLast = Math.ceil(rippleIdx + 1.25) - 1;
}

const stepChanged = step !== lastStepRef;
const rippleChanged =
  arithActive && (rippleFirst !== lastRippleFirst || rippleLast !== lastRippleLast);

if (stepChanged || rippleChanged) {
  lastStepRef = step;
  lastRippleFirst = arithActive ? rippleFirst : 0x7fffffff;
  lastRippleLast = arithActive ? rippleLast : 0x7fffffff;

  const lhsBytes = arith?.lhsBytes || EMPTY_BYTES;
  const rhsBytes = arith?.rhsBytes || EMPTY_BYTES;
  const resBytes = arith?.resultBytes || EMPTY_BYTES;
  const carries = arith?.carries || EMPTY_BYTES;
  const borrows = arith?.borrows || EMPTY_BYTES;
  const fuelAfterBytes = fuel?.afterBytes || EMPTY_BYTES;

  let rightChanged = false;
  for (let c = 0; c < LIMB_BYTE_COUNT; c++) {
    const lb = lhsBytes[c] ?? 0;
    const rb = rhsBytes[c] ?? 0;
    const outB = resBytes[c] ?? 0;
    const cVal = carries[c] ?? 0;
    const boVal = borrows[c] ?? 0;
    const fb = fuelAfterBytes[c] ?? 0;

    const hasArithByte = outB > 0 || lb > 0 || rb > 0 || cVal > 0 || boVal > 0;
    let nextHex = STANDBY_SAPPHIRE_SLOT_HEX;

    if (arithActive && hasArithByte) {
      nextHex = arith?.overflow || boVal > 0 ? PALETTE.heatBright : AMBER_GOLD_HEX;
    } else if (arithActive) {
      nextHex =
        c > rippleIdx - 1.25 && c < rippleIdx + 1.25
          ? AMBER_RIPPLE_HEX
          : STANDBY_SAPPHIRE_SLOT_HEX;
    } else if (hasArithByte) {
      nextHex = AMBER_GOLD_HEX;
    } else if (fb > 0) {
      nextHex = CHAMPAGNE_GOLD_HEX;
    }

    if (lastRightHex[c] !== nextHex) {
      lastRightHex[c] = nextHex;
      _tmpColor.setHex(nextHex);
      rightLimbInstanced.setColorAt(c, _tmpColor);
      rightChanged = true;
    }
  }

  if (rightChanged && rightLimbInstanced.instanceColor) {
    rightLimbInstanced.instanceColor.needsUpdate = true;
  }
}
```

`c > rippleIdx - 1.25 && c < rippleIdx + 1.25` is the same predicate as `nearRipple > 0.5` (the clamp saturates outside that band, and inside the band the comparison reduces to that inequality). `step` is the object from `trace.steps[stepIndex]` (`sherlockEngine.js` `buildStepState`, around line 2158) and is stable for the whole time `getOrBuildStepState` keeps that index (`app.js` 910–928). Reference equality remains a valid step gate.

Left flank (lines 165–194) needs nothing beyond the visibility return in section 1. It already uploads only on an expression-count or active-row change.

### 4c. Designer 6 steady-state cost

After section 1 and 4a, a visible bar whose step has not changed executes the integer key math at lines 98–111 and two comparisons, then returns. That is the right steady state. Caching a single packed key would replace two comparisons with one and is not worth the extra code.

`instanceMatrix.setUsage(DynamicDrawUsage)` on line 62 does not affect this WebGPU backend (see the renderer note at the top). Leave it. The matrix upload already happens only inside the layout branch (`needsUpdate` on line 141), which is trace load / slice-width change, not per frame.

---

## 5. Smaller items, still pixel-identical

### Pointer rect cache — `updatePointerNDC`, lines 558–563

Every accepted raycast calls `getBoundingClientRect()`, which can force layout. The canvas size changes in `SherlockStage.resize` (stage lines 277–286), not on pointer move. Cache the rect on the cinema subsystem and refresh it from `resize` (or from a `ResizeObserver` if the stage does not forward the event). Hover coordinates stay in the same NDC space.

```js
let pointerRect = { left: 0, top: 0, width: 1, height: 1 };
function refreshPointerRect() {
  if (!domElement) return;
  const rect = domElement.getBoundingClientRect();
  pointerRect.left = rect.left;
  pointerRect.top = rect.top;
  pointerRect.width = rect.width;
  pointerRect.height = rect.height;
}
function updatePointerNDC(event) {
  if (!domElement) return;
  const width = pointerRect.width;
  const height = pointerRect.height;
  if (width <= 0 || height <= 0) return;
  pointerNDC.x = ((event.clientX - pointerRect.left) / width) * 2 - 1;
  pointerNDC.y = -((event.clientY - pointerRect.top) / height) * 2 + 1;
}
```

Call `refreshPointerRect()` once when the listeners are attached (after line 737) and from the stage `resize` path. A window scroll that moves the canvas without a resize would stale the cache; the workbench canvas is a full-viewport stage, so resize covers it. If the page ever scrolls the canvas, refresh on the next pointer down as well.

### Static matrices on objects that never move

Designer 5 channels and both instanced meshes are placed once (lines 56–64, 102–114) and never translated again. Designer 6’s channel and instanced mesh are the same (lines 47–63); segment motion is in `instanceMatrix`, not in the object matrix. Designer 7 writes the reticle in world space into the line buffer (lines 190–213) and leaves the reticle group at the origin.

The renderer still runs `updateMatrix()` for every `matrixAutoUpdate` object each time it walks the graph (`Object3D.updateMatrixWorld`). Freeze the static roots after the initial placement:

```js
function freezeStatic(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
}
```

Call it on `group`, `leftChannelMesh`, `rightChannelMesh`, `leftExprInstanced`, and `rightLimbInstanced` at the end of the designer 5 setup (after line 128). Call it on `group`, `channelMesh`, and `progressInstanced` at the end of designer 6 setup (after line 82). Call it on `reticleGroup` and `cornerLines` at the end of `createInspectorReticle` (after line 153). Do not freeze the camera. Do not freeze anything whose `position` or `quaternion` changes later (the cinema look target is the camera, which is not parented under this group).

Instance color and instance matrix updates do not go through `matrixAutoUpdate`. Freezing the `InstancedMesh` object matrix leaves `setColorAt` / `setMatrixAt` working.

This is a small saving next to sections 2 and 3. It is listed because it is free and exact.

---

## What was searched for and is not in these files

- **A literal per-frame `scene.traverse()` in `designer7_cinema.js`.** The equivalent walks are `intersectObjects(scene.children, true)` (line 539) and `Box3.setFromObject` (line 872). Section 2 replaces both with cached lists.
- **A cutaway opacity transition in designers 5, 6, or 7.** No opacity is lerped. Designer 7’s only repeating material write is the reticle (lines 216–217), settled in section 3a. Cutaway motion lives in `SherlockStage._cutawayMix` (snapped at stage lines 296–299), `designer1_package.js` 817–822 (position and visibility), and `designer2_well.js` 2833–2856 (position and visibility). Those writes become idempotent once the mix has snapped. Designer 2’s per-frame dial and fuel opacity (`designer2_well.js` 2523–2545 and 2597–2621) is outside this audit.
- **`addUpdateRange` on the 16- and 32-wide instance color buffers.** A single full-buffer `writeBuffer` is the cheaper WebGPU path at this size. Section 4b.
- **CPU frustum tests to skip circuit or loom `update`.** Both updates are a handful of arithmetic ops once their gates hold. A bounding-sphere-versus-frustum test costs as much as the update it would skip, and a late catch-up is unnecessary while the layer is visible. `frustumCulled` on the meshes already keeps off-screen draws out of the render pass. Section 1 covers the hidden-layer case, which is the one that drops real work (the active ripple).

---

## Application order

1. Designer 7 pick list and bounds cache (section 2). Largest CPU cut while the pointer moves, and it removes the empty-box per-frame walk.
2. Designer 7 camera hold and reticle material settle (section 3). Removes per-frame `lookAt` on rested presets and per-frame material uniform writes while the reticle is up. The pulse vertex upload stays.
3. Visibility returns in designers 5, 6, and the reticle half of 7 (section 1).
4. Loom `count` and the circuit ripple-window gate (section 4).
5. Pointer-rect cache and `matrixAutoUpdate = false` (section 5).

After each step, check three shots against the current build: overview at rest (camera must not drift), `frames` preset in and out (cutaway still opens and the mix still snaps), and a hovered floor chiplet plus a hovered flank LED (same inspect target, same reticle, same LED colors on an active `aux2.arithmetic` step and on a quiescent step). Cinema orbit should keep moving. Hidden circuit and loom layers should show the same LEDs on the frame they are turned back on.
