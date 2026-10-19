# High-Performance Audit Report #6: `designer5_circuit.js`, `designer6_loom.js` & `designer7_cinema.js`

## Executive Summary & Specific Audit Answers

1. **Child Designer Instantiation & Invisible Layer Handling (`designer7_cinema.js` & `SherlockStage.js`)**:
   - `designer7_cinema.js` does **not** instantiate child designers; `SherlockStage.js` (`lines 196–234`) instantiates all 8 subsystems (`package`, `well`, `frames`, `arena`, `circuit`, `loom`, `quineling`, `cinema`) and keeps all 8 groups mounted in `this.root`. While `SherlockStage.render()` (`line 355`) skips `.update()` when `this.layerVisibility[key] === false`, **`designer7_cinema.js` still raycasts against the entire `scene.children` hierarchy** (`line 539`), testing lights, the reticle, and hundreds of non-inspectable decorative meshes before checking visibility in `findInspectableTarget()` (`lines 482–492`).
2. **Scene Graph Traversal (`traverse()`) in `designer7_cinema.js`**:
   - Although `.traverse()` is not called directly by name in `designer7_cinema.js`, **two expensive recursive scene-graph traversals** happen dynamically:
     1. **Inside `.update()` (`line 872`)**: `rawBox.setFromObject(activeInspectTarget)` internally executes `activeInspectTarget.updateWorldMatrix(false, true)` + `activeInspectTarget.traverse(...)` across every descendant mesh and instance matrix whenever `activeInspectTarget` or `stepIndex` changes—even for 100% static targets (`Designer1`, `Designer5`, `Designer6`) and including invisible child meshes (`child.visible === false`).
     2. **Inside `performRaycast()` (`line 539`)**: `raycaster.intersectObjects(scene.children, true)` recursively traverses every object in the scene (including non-inspectable meshes) and allocates a new `hits` array on every raycast.
3. **Cutaway / Opacity & Settled-Transition State Churn (`designer7_cinema.js`)**:
   - In `createInspectorReticle().updateGeometryFromBox()` (`lines 216–217`), `cornerMat.color.setHex(...)` and `cornerMat.opacity = ...` are written on **every single frame** while an object is hovered or pinned, even when `pinned` has not changed.
   - In `update()` (`lines 770–849`), camera damping (`lerp`, `setFromSphericalCoords`, `camera.lookAt`, `clampInteriorEye`) runs unconditionally on **every single frame** even after the camera transition has completely settled (`Math.abs(current - target) < 1e-4`), continuously dirtying the camera's world/view matrices.
   - In `update()` (`lines 887–890`), `smoothMin.lerp(rawBox.min, boxDamp)` and `smoothMax.lerp(rawBox.max, boxDamp)` continue lerping every frame even after `smoothMin`/`smoothMax` have converged (`< 1e-4`).
4. **`designer5_circuit.js` & `designer6_loom.js` Mesh Creation, Geometry Updates & Allocations**:
   - Both files already use `InstancedMesh` + `BoxGeometry` and module-scope scratch objects (no curve/tube geometries or per-frame heap allocations).
   - However, **`designer5_circuit.js`** (`lines 200–252`) re-evaluates all 16 limbs and performs 96 byte-array lookups ~60 times/sec whenever `arithActive` is true (because `rippleBucket` changes continuously with `ambient`), even when `step === lastStepRef`. It also uses 2 separate `THREE.Mesh` draw calls for identical static flank channels (`lines 56–64`) with `matrixAutoUpdate = true`.
   - **`designer6_loom.js`** (`lines 128–176`) keeps `progressInstanced.count = 32` (`MAX_STEPS`) even when `segmentCount` is smaller (e.g., 16), forcing the GPU to run vertex shaders on degenerate zero-scale instances (`_tmpScale.set(0, 0, 0)`), and calls `progressInstanced.computeBoundingSphere()` inside `update()` (`line 142`) instead of using a static precomputed bounding volume.

---

## Prioritized Optimizations & Drop-In Code Snippets

### Priority 1 (High Impact): Replace Full-Scene Raycast Traversal & Per-Step `Box3.setFromObject()` Traversal in `src/designers/designer7_cinema.js`

**File**: `/home/charles/sherlock-bench/src/designers/designer7_cinema.js`
**Lines**: `419–556` and `860–905`

#### Why this matters:
1. `raycaster.intersectObjects(scene.children, true)` (`line 539`) traverses all lights, decorative non-inspectable meshes, and the reticle itself, and allocates a fresh `hits` array every call. Caching a pre-collected array of inspectable root objects (lazily built once when `scene.children` is populated) and passing a reusable `_raycastHits` array eliminates non-inspectable mesh raycasting and per-raycast array allocations.
2. `rawBox.setFromObject(activeInspectTarget)` (`line 872`) traverses the target's subtree and recomputes world bounds on every step change (`currentStepIdx !== lastBoxStepIndex`), even for static targets (`Designer5` flanks, `Designer6` progress bar, `Designer1` package, etc.). Caching bounding boxes for static targets and snapping `smoothMin`/`smoothMax` once settled (`< 1e-8` squared distance) avoids per-step subtree traversals.

#### Drop-in Replacement (`src/designers/designer7_cinema.js`, lines `435–565`):
```javascript
  let hoveredObject = null;
  let pinnedObject = null;
  let lastCursor = 'grab';

  const _raycastHits = [];
  let cachedInspectableRoots = null;
  let cachedSceneChildCount = -1;
  let cachedSubtreeCount = -1;

  function getInspectableRoots() {
    if (!scene) return [];
    const rootGroup = scene.children.find((c) => c.isGroup) || scene;
    const subCount = rootGroup.children ? rootGroup.children.length : 0;
    if (
      cachedInspectableRoots &&
      scene.children.length === cachedSceneChildCount &&
      subCount === cachedSubtreeCount
    ) {
      return cachedInspectableRoots;
    }
    cachedSceneChildCount = scene.children.length;
    cachedSubtreeCount = subCount;
    const roots = [];
    scene.traverse((obj) => {
      if (obj.userData?.isInspectorReticle) return;
      if (obj.userData?.inspectable || Array.isArray(obj.userData?.ruleRecords)) {
        roots.push(obj);
      }
    });
    cachedInspectableRoots = roots;
    return roots;
  }

  // Cached DOM rect to avoid synchronous layout reflow on every pointermove
  let cachedRect = null;
  let cachedRectTime = 0;

  // Smoothed world-space bounding box for the inspector reticle
  const rawBox = new THREE.Box3();
  const smoothMin = new THREE.Vector3();
  const smoothMax = new THREE.Vector3();
  let reticleInitialized = false;
  let reticleBoxSettled = false;
```

And in `performRaycast()` & `updatePointerNDC()` (`src/designers/designer7_cinema.js`, lines `536–564`):
```javascript
  function setCursor(nextCursor) {
    if (domElement && lastCursor !== nextCursor) {
      lastCursor = nextCursor;
      domElement.style.cursor = nextCursor;
    }
  }

  function performRaycast() {
    if (!camera || !scene || !pointerInside || isDragging) return null;
    raycaster.setFromCamera(pointerNDC, camera);
    const inspectableRoots = getInspectableRoots();
    _raycastHits.length = 0;
    for (let i = 0; i < inspectableRoots.length; i++) {
      const rootObj = inspectableRoots[i];
      if (isHierarchyVisible(rootObj)) {
        raycaster.intersectObject(rootObj, true, _raycastHits);
      }
    }
    if (_raycastHits.length > 1) {
      _raycastHits.sort((a, b) => a.distance - b.distance);
    }
    const target = findInspectableTarget(_raycastHits);

    if (target !== hoveredObject) {
      hoveredObject = target;
      if (!pinnedObject) {
        emitInspect(hoveredObject, false);
      }
    }
    setCursor(isDragging ? 'grabbing' : hoveredObject ? 'pointer' : 'grab');
    return target;
  }

  function updatePointerNDC(event) {
    if (!domElement) return;
    const now = performance.now();
    if (!cachedRect || now - cachedRectTime > 250) {
      cachedRect = domElement.getBoundingClientRect();
      cachedRectTime = now;
    }
    if (cachedRect.width <= 0 || cachedRect.height <= 0) return;
    pointerNDC.x = ((event.clientX - cachedRect.left) / cachedRect.width) * 2 - 1;
    pointerNDC.y = -((event.clientY - cachedRect.top) / cachedRect.height) * 2 + 1;
  }
```

---

### Priority 2 (High Impact): Gate Camera Damping, `lookAt()`, and Reticle Material State When Settled in `src/designers/designer7_cinema.js`

**File**: `/home/charles/sherlock-bench/src/designers/designer7_cinema.js`
**Lines**: `166–218` and `770–905`

#### Why this matters:
1. Inside `createInspectorReticle()` (`lines 216–217`), `cornerMat.color.setHex(...)` and `cornerMat.opacity = ...` are reassigned every frame even when `pinned` hasn't changed. Tracking `lastPinned` avoids per-frame material uniform churn.
2. Inside `update()` (`lines 770–849`), when the camera is stationary (`Math.abs(current - desired) < 1e-4` and `currentPresetName !== 'cinema'`), `lerp`, `setFromSphericalCoords`, and `camera.lookAt()` still execute every frame. Adding a settled check snaps the camera once and skips trig/matrix calculations on stationary frames.

#### Drop-in Replacement (`src/designers/designer7_cinema.js`, lines `165–220`):
```javascript
  let lastPinned = null;

  function updateGeometryFromBox(min, max, ambient = 0, pinned = false) {
    const dx = Math.max(0.035, max.x - min.x);
    const dy = Math.max(0.025, max.y - min.y);
    const dz = Math.max(0.035, max.z - min.z);

    const pulse = (pinned ? 0.0025 : 0.005) * Math.sin(ambient * 4.0);
    const x0 = min.x - pulse;
    const y0 = min.y - pulse;
    const z0 = min.z - pulse;
    const x1 = max.x + pulse;
    const y1 = max.y + pulse;
    const z1 = max.z + pulse;

    const bx = clamp(dx * 0.22, 0.018, 0.11);
    const by = clamp(dy * 0.22, 0.014, 0.09);
    const bz = clamp(dz * 0.22, 0.018, 0.11);

    let idx = 0;
    for (let i = 0; i < 8; i++) {
      const [sx, sy, sz] = cornerSigns[i];
      const cx = sx > 0 ? x0 : x1;
      const cy = sy > 0 ? y0 : y1;
      const cz = sz > 0 ? z0 : z1;

      // X hairline
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;
      cornerPositions[idx++] = cx + sx * bx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;

      // Y hairline
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy + sy * by;
      cornerPositions[idx++] = cz;

      // Z hairline
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz + sz * bz;
    }

    cornerGeo.attributes.position.needsUpdate = true;
    if (pinned !== lastPinned) {
      lastPinned = pinned;
      cornerMat.color.setHex(pinned ? PALETTE.heatBright : PALETTE.goldBright);
      cornerMat.opacity = pinned ? 0.96 : 0.84;
    }
  }
```

#### Drop-in Replacement for Camera & Reticle Update (`src/designers/designer7_cinema.js`, lines `770–905`):
```javascript
    if (camera) {
      const damp = 1.0 - Math.exp(-12.0 * dt);

      if (typeof camera.fov === 'number' && camera.fov !== desiredFov) {
        if (Math.abs(camera.fov - desiredFov) > 1e-3) {
          camera.fov = lerp(camera.fov, desiredFov, damp);
        } else {
          camera.fov = desiredFov;
        }
        if (typeof camera.updateProjectionMatrix === 'function') {
          camera.updateProjectionMatrix();
        }
      }

      if (isInteriorPresetName(currentPresetName)) {
        const eyeDistSq = interiorEye.distanceToSquared(desiredInteriorEye);
        const yawDiff = Math.abs(interiorYaw - desiredInteriorYaw);
        const pitchDiff = Math.abs(interiorPitch - desiredInteriorPitch);

        if (eyeDistSq > 1e-8 || yawDiff > 1e-5 || pitchDiff > 1e-5) {
          clampInteriorEye(desiredInteriorEye);
          if (eyeDistSq > 1e-8) {
            interiorEye.lerp(desiredInteriorEye, damp);
            clampInteriorEye(interiorEye);
          } else {
            interiorEye.copy(desiredInteriorEye);
          }

          interiorYaw = yawDiff > 1e-5 ? lerp(interiorYaw, desiredInteriorYaw, damp) : desiredInteriorYaw;
          interiorPitch = pitchDiff > 1e-5 ? lerp(interiorPitch, desiredInteriorPitch, damp) : desiredInteriorPitch;

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
      } else {
        if (currentPresetName === 'cinema' && !isDragging) {
          desiredTheta += dt * 0.18;
          desiredPhi = 0.82 + Math.sin(ambient * 0.24) * 0.06;
          desiredRadius = 7.45 + Math.cos(ambient * 0.20 + progress * Math.PI) * 0.38;

          const phaseTargetY =
            phaseMode === 1 ? -0.26 : phaseMode === 2 ? -0.10 : -0.16;
          desiredTarget.y = lerp(desiredTarget.y, phaseTargetY, dt * 1.6);
        }

        const radDiff = Math.abs(currentRadius - desiredRadius);
        const phiDiff = Math.abs(currentPhi - desiredPhi);
        const thetaDiff = Math.abs(currentTheta - desiredTheta);
        const targetDistSq = currentTarget.distanceToSquared(desiredTarget);

        if (
          radDiff > 1e-5 ||
          phiDiff > 1e-5 ||
          thetaDiff > 1e-5 ||
          targetDistSq > 1e-8 ||
          transitionArcBoost > 0
        ) {
          currentRadius = radDiff > 1e-5 ? lerp(currentRadius, desiredRadius, damp) : desiredRadius;
          currentPhi = phiDiff > 1e-5 ? lerp(currentPhi, desiredPhi, damp) : desiredPhi;
          currentTheta = thetaDiff > 1e-5 ? lerp(currentTheta, desiredTheta, damp) : desiredTheta;
          if (targetDistSq > 1e-8) {
            currentTarget.lerp(desiredTarget, damp);
          } else {
            currentTarget.copy(desiredTarget);
          }

          transitionArcBoost = Math.max(0, transitionArcBoost - dt * 2.4);
          const arcLiftY = transitionArcBoost > 0 ? Math.sin(transitionArcBoost * Math.PI) * 0.28 : 0;

          _camOffset.setFromSphericalCoords(
            currentRadius,
            currentPhi,
            currentTheta
          );
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
      }
    }

    // Drop pinned or hovered object if it was popped/hidden
    if (pinnedObject && !isHierarchyVisible(pinnedObject)) {
      pinnedObject = null;
      emitInspect(hoveredObject && isHierarchyVisible(hoveredObject) ? hoveredObject : null, false);
    }
    if (hoveredObject && !isHierarchyVisible(hoveredObject)) {
      hoveredObject = null;
    }

    // Update Minimalist 8-Corner Hairline Reticle
    const activeInspectTarget = pinnedObject || hoveredObject;
    if (activeInspectTarget && isHierarchyVisible(activeInspectTarget)) {
      const currentStepIdx = stepState.stepIndex ?? 0;
      const isStaticTarget = Boolean(activeInspectTarget.userData?.staticWorldBox);
      if (
        activeInspectTarget !== lastBoxTarget ||
        (!isStaticTarget && currentStepIdx !== lastBoxStepIndex) ||
        rawBox.isEmpty()
      ) {
        if (activeInspectTarget.customBox) {
          rawBox.copy(activeInspectTarget.customBox);
        } else if (activeInspectTarget.userData?.cachedWorldBox && isStaticTarget) {
          rawBox.copy(activeInspectTarget.userData.cachedWorldBox);
        } else {
          rawBox.setFromObject(activeInspectTarget);
          if (!rawBox.isEmpty()) {
            rawBox.expandByScalar(0.018);
            if (isStaticTarget) {
              activeInspectTarget.userData.cachedWorldBox = rawBox.clone();
            }
          }
        }
        lastBoxTarget = activeInspectTarget;
        lastBoxStepIndex = currentStepIdx;
        reticleBoxSettled = false;
      }

      if (!rawBox.isEmpty()) {
        if (!reticleInitialized) {
          smoothMin.copy(rawBox.min);
          smoothMax.copy(rawBox.max);
          reticleInitialized = true;
          reticleBoxSettled = true;
        } else if (!reticleBoxSettled) {
          const boxDamp = 1.0 - Math.exp(-20.0 * dt);
          smoothMin.lerp(rawBox.min, boxDamp);
          smoothMax.lerp(rawBox.max, boxDamp);
          if (
            smoothMin.distanceToSquared(rawBox.min) < 1e-8 &&
            smoothMax.distanceToSquared(rawBox.max) < 1e-8
          ) {
            smoothMin.copy(rawBox.min);
            smoothMax.copy(rawBox.max);
            reticleBoxSettled = true;
          }
        }
        reticle.group.visible = true;
        reticle.updateGeometryFromBox(smoothMin, smoothMax, ambient, Boolean(pinnedObject));

        if (stepState.stepIndex !== undefined && stepState.stepIndex !== lastStepIndex) {
          lastStepIndex = stepState.stepIndex;
          emitInspect(activeInspectTarget, Boolean(pinnedObject));
        }
      } else {
        reticle.group.visible = false;
      }
    } else {
      reticle.group.visible = false;
      reticleInitialized = false;
      reticleBoxSettled = false;
      lastBoxTarget = null;
    }
```

---

### Priority 3 (Medium-High Impact): Decouple Step-Level Byte Extraction from 60Hz Ripple Animation & Merge Static Channel Meshes in `src/designers/designer5_circuit.js`

**File**: `/home/charles/sherlock-bench/src/designers/designer5_circuit.js`
**Lines**: `47–149` and `196–253`

#### Why this matters:
1. **Per-Frame Byte-Array Lookup Churn (`lines 200–252`)**: When `arithActive` is `true`, `rippleIdx = ((ambient * 3.0) % 1.0) * 15` advances continuously with `ambient`, causing `rippleBucket = Math.floor(rippleIdx * 2)` to change **~60 times per second**. Because `step !== lastStepRef` and `rippleBucket !== lastRippleBucket` are combined into a single block, every `rippleBucket` tick re-reads 6 byte arrays (`lhsBytes`, `rhsBytes`, `resBytes`, `carries`, `borrows`, `fuelAfterBytes`) across all 16 limbs (96 array lookups/frame) and recomputes `hasArithByte` even though `step` hasn't changed! Precomputing a 16-entry `stepBaseHex` array (`0` = eligible for ripple pulse, non-zero = fixed limb color for this step) only when `step !== lastStepRef` reduces the 60Hz ripple loop to a simple `Math.abs(c - rippleIdx) < 1.25` check on idle limbs.
2. **Draw Call & Static Matrix Optimization (`lines 56–128`)**: `leftChannelMesh` and `rightChannelMesh` are static `THREE.Mesh` instances with identical geometry and material; freezing `matrixAutoUpdate = false` on all static meshes (`leftChannelMesh`, `rightChannelMesh`, `leftExprInstanced`, `rightLimbInstanced`, `group`) and marking `staticWorldBox: true` in `tagInspectable` prevents per-frame matrix updates and per-step bounding box recomputations.

#### Drop-in Replacement (`src/designers/designer5_circuit.js`, lines `56–150` & `196–253`):
```javascript
  const leftChannelMesh = new THREE.Mesh(channelGeo, channelMat);
  leftChannelMesh.position.set(LEFT_FLANK_X, CHANNEL_Y, 0);
  leftChannelMesh.receiveShadow = true;
  leftChannelMesh.matrixAutoUpdate = false;
  leftChannelMesh.updateMatrix();
  group.add(leftChannelMesh);

  const rightChannelMesh = new THREE.Mesh(channelGeo, channelMat);
  rightChannelMesh.position.set(RIGHT_FLANK_X, CHANNEL_Y, 0);
  rightChannelMesh.receiveShadow = true;
  rightChannelMesh.matrixAutoUpdate = false;
  rightChannelMesh.updateMatrix();
  group.add(rightChannelMesh);
```
And at lines `127–150`:
```javascript
  leftExprInstanced.matrixAutoUpdate = false;
  leftExprInstanced.updateMatrix();
  rightLimbInstanced.matrixAutoUpdate = false;
  rightLimbInstanced.updateMatrix();
  group.matrixAutoUpdate = false;
  group.updateMatrix();
  leftExprInstanced.computeBoundingSphere();
  leftExprInstanced.computeBoundingBox();
  rightLimbInstanced.computeBoundingSphere();
  rightLimbInstanced.computeBoundingBox();

  tagInspectable(leftExprInstanced, {
    type: 'circuit-left-flank',
    id: 'left-flank-expression-array',
    title: 'Left Flank AST Expression Photolithography Channel',
    subtitle: 'y = 0.1052 // Dark Sapphire-Slate Channel · Cyan-Teal (0x2dd4bf) Active AST Row',
    details: 'Recessed dark sapphire-slate photolithography channel flush on the left silver-platinum flank (y = 0.1052), glowing with Cyan-Teal (0x2dd4bf) on the active AST expression row.',
    staticWorldBox: true,
  });

  tagInspectable(rightLimbInstanced, {
    type: 'circuit-right-flank',
    id: 'right-flank-uint128-array',
    title: 'Right Flank 16-Byte UInt128 Photolithography Channel',
    subtitle: 'y = 0.1052 // Dark Sapphire-Slate Channel · Amber-Gold (0xfbbf24) UInt128 Limbs',
    details: 'Recessed dark sapphire-slate photolithography channel flush on the right silver-platinum flank (y = 0.1052), glowing with Amber-Gold (0xfbbf24) for 16-byte UInt128 arithmetic limbs.',
    staticWorldBox: true,
  });

  let lastExprCount = -1;
  let lastActiveRowIdx = -1;
  let lastStepRef = undefined;
  let lastRippleBucket = -1;
  // 0 indicates the limb is idle during active arithmetic and should respond to the ripple wave
  const stepBaseHex = new Uint32Array(LIMB_BYTE_COUNT);
```

And in `update(stepState)` (`src/designers/designer5_circuit.js`, lines `196–253`):
```javascript
    // -----------------------------------------------------------------------
    // 2. Right Flank: 16-Byte UInt128 Arithmetic Limbs (Amber-Gold 0xfbbf24)
    //    Decode byte arrays ONLY on step change; ripple pass only touches idle limbs
    // -----------------------------------------------------------------------
    const arith = after?.aux2?.arithmetic;
    const arithActive = Boolean(arith?.active);
    const rippleIdx = ((ambient * 3.0) % 1.0) * (LIMB_BYTE_COUNT - 1);
    const rippleBucket = arithActive ? Math.floor(rippleIdx * 2) : -1;

    const stepChanged = step !== lastStepRef;
    if (stepChanged) {
      lastStepRef = step;
      const fuel = after?.aux2?.fuel;
      const lhsBytes = arith?.lhsBytes || EMPTY_BYTES;
      const rhsBytes = arith?.rhsBytes || EMPTY_BYTES;
      const resBytes = arith?.resultBytes || EMPTY_BYTES;
      const carries = arith?.carries || EMPTY_BYTES;
      const borrows = arith?.borrows || EMPTY_BYTES;
      const fuelAfterBytes = fuel?.afterBytes || EMPTY_BYTES;
      const overflow = Boolean(arith?.overflow);

      for (let c = 0; c < LIMB_BYTE_COUNT; c++) {
        const lb = lhsBytes[c] ?? 0;
        const rb = rhsBytes[c] ?? 0;
        const outB = resBytes[c] ?? 0;
        const cVal = carries[c] ?? 0;
        const boVal = borrows[c] ?? 0;
        const fb = fuelAfterBytes[c] ?? 0;
        const hasArithByte = outB > 0 || lb > 0 || rb > 0 || cVal > 0 || boVal > 0;

        if (arithActive && hasArithByte) {
          stepBaseHex[c] = overflow || boVal > 0 ? PALETTE.heatBright : AMBER_GOLD_HEX;
        } else if (arithActive) {
          stepBaseHex[c] = 0; // Eligible for ambient ripple
        } else if (hasArithByte) {
          stepBaseHex[c] = AMBER_GOLD_HEX;
        } else if (fb > 0) {
          stepBaseHex[c] = CHAMPAGNE_GOLD_HEX;
        } else {
          stepBaseHex[c] = STANDBY_SAPPHIRE_SLOT_HEX;
        }
      }
    }

    if (stepChanged || rippleBucket !== lastRippleBucket) {
      lastRippleBucket = rippleBucket;
      let rightChanged = false;

      for (let c = 0; c < LIMB_BYTE_COUNT; c++) {
        const baseHex = stepBaseHex[c];
        const nextHex =
          baseHex !== 0
            ? baseHex
            : Math.abs(c - rippleIdx) < 1.25
              ? AMBER_RIPPLE_HEX
              : STANDBY_SAPPHIRE_SLOT_HEX;

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

---

### Priority 4 (Medium Impact): Set `progressInstanced.count = segmentCount` & Eliminate Dynamic `computeBoundingSphere()` in `src/designers/designer6_loom.js`

**File**: `/home/charles/sherlock-bench/src/designers/designer6_loom.js`
**Lines**: `47–91` and `113–177`

#### Why this matters:
1. **GPU Vertex Shader & CPU Loop Reduction (`lines 128–170`)**: When `segmentCount < MAX_STEPS` (e.g., 16 segments vs `MAX_STEPS = 32`), `designer6_loom.js` currently sets instances `16..31` to zero scale `_tmpScale.set(0, 0, 0)` and leaves `progressInstanced.count = 32`. By setting `progressInstanced.count = segmentCount`, WebGPU only dispatches `segmentCount` instances in the draw call (cutting vertex shader work by 50% on 16-step traces) and both the matrix composition loop and color update loop only iterate `0 .. segmentCount - 1`.
2. **Eliminate Dynamic `computeBoundingSphere()` (`lines 82 & 142`)**: `progressInstanced.computeBoundingSphere()` iterates all 32 instance matrices and unions transformed spheres. Because the progress bar always occupies the fixed span `[-BAR_SPAN_X / 2, +BAR_SPAN_X / 2]` at `(0, BAR_Y, BAR_Z)`, setting a static `boundingSphere` and `boundingBox` once at initialization eliminates `computeBoundingSphere()` during `update()`.

#### Drop-in Replacement (`src/designers/designer6_loom.js`, lines `47–91` & `113–177`):
```javascript
  const channelMesh = new THREE.Mesh(channelGeo, channelMat);
  channelMesh.position.set(0, CHANNEL_Y, BAR_Z);
  channelMesh.receiveShadow = true;
  channelMesh.matrixAutoUpdate = false;
  channelMesh.updateMatrix();
  group.add(channelMesh);

  // Single InstancedMesh for all 32 flush execution progress segments (1 draw call total)
  const segGeo = track(new THREE.BoxGeometry(1, 0.0012, 0.012));
  const segMat = track(
    new THREE.MeshBasicMaterial({
      color: 0xffffff,
      toneMapped: false,
    })
  );

  const progressInstanced = new THREE.InstancedMesh(segGeo, segMat, MAX_STEPS);
  progressInstanced.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
  progressInstanced.matrixAutoUpdate = false;
  progressInstanced.updateMatrix();
  group.add(progressInstanced);
  group.matrixAutoUpdate = false;
  group.updateMatrix();

  const lastSegHex = new Uint32Array(MAX_STEPS);

  // Pre-initialize all 32 matrices & colors so GPU buffers are allocated upfront
  for (let i = 0; i < MAX_STEPS; i++) {
    _tmpPos.set(0, BAR_Y, BAR_Z);
    _tmpScale.set(0.04, 1, 1);
    _tmpMatrix.compose(_tmpPos, _tmpQuat, _tmpScale);
    progressInstanced.setMatrixAt(i, _tmpMatrix);
    _tmpColor.setHex(PENDING_SAPPHIRE_SLOT_HEX);
    progressInstanced.setColorAt(i, _tmpColor);
    lastSegHex[i] = PENDING_SAPPHIRE_SLOT_HEX;
  }
  progressInstanced.instanceMatrix.needsUpdate = true;
  if (progressInstanced.instanceColor) {
    progressInstanced.instanceColor.setUsage(THREE.DynamicDrawUsage);
    progressInstanced.instanceColor.needsUpdate = true;
  }
  progressInstanced.boundingSphere = new THREE.Sphere(
    new THREE.Vector3(0, BAR_Y, BAR_Z),
    BAR_SPAN_X * 0.55
  );
  progressInstanced.boundingBox = new THREE.Box3(
    new THREE.Vector3(-BAR_SPAN_X * 0.55, BAR_Y - 0.01, BAR_Z - 0.02),
    new THREE.Vector3(BAR_SPAN_X * 0.55, BAR_Y + 0.01, BAR_Z + 0.02)
  );

  tagInspectable(group, {
    type: 'execution-progress',
    id: 'flush-rear-progress-filament',
    title: 'Flush Execution Progress Filament',
    subtitle: 'y = 0.1052, z = -1.92 // Sapphire-Slate Track · Slice-Grouped Execution Progress',
    details: '32-segment execution progress bar grouped by ReferenceSlice width and inset flush into a dark sapphire-slate track on the rear brushed silver-platinum deck (y = 0.1052).',
    staticWorldBox: true,
  });
```

And in `update(stepState = {})` (`src/designers/designer6_loom.js`, lines `113–177`):
```javascript
    // Recompute segment widths & slice-grouped positions only when segmentCount or sliceWidth changes
    if (segmentCount !== lastSegmentCount || sliceWidth !== lastSliceWidth) {
      lastSegmentCount = segmentCount;
      lastSliceWidth = sliceWidth;

      const nomPitch = BAR_SPAN_X / segmentCount;
      const numBreaks = segmentCount > 1 ? Math.floor((segmentCount - 1) / sliceWidth) : 0;
      const numIntra = Math.max(0, segmentCount - 1 - numBreaks);
      const intraGap = Math.min(0.010, nomPitch * 0.16);
      const interGap = Math.min(0.032, nomPitch * 0.48);
      const totalGap = numIntra * intraGap + numBreaks * interGap;
      const segWidth = Math.max(0.010, (BAR_SPAN_X - totalGap) / segmentCount);
      const actualSpan = segmentCount * segWidth + totalGap;
      let cursorX = -actualSpan * 0.5 + segWidth * 0.5;

      for (let i = 0; i < segmentCount; i++) {
        _tmpPos.set(cursorX, BAR_Y, BAR_Z);
        _tmpScale.set(segWidth, 1, 1);
        const isSliceBreak = (i + 1) % sliceWidth === 0;
        cursorX += segWidth + (isSliceBreak ? interGap : intraGap);
        _tmpMatrix.compose(_tmpPos, _tmpQuat, _tmpScale);
        progressInstanced.setMatrixAt(i, _tmpMatrix);
      }
      progressInstanced.count = segmentCount;
      progressInstanced.instanceMatrix.needsUpdate = true;
      lastActiveSeg = -1;
    }

    if (activeSegment !== lastActiveSeg || phase !== lastPhase) {
      lastActiveSeg = activeSegment;
      lastPhase = phase;

      const activeHex = phase === 'ERROR' ? PALETTE.errorRed : LUMINOUS_GOLD_HEX;
      let colorChanged = false;

      for (let i = 0; i < segmentCount; i++) {
        let nextHex = PENDING_SAPPHIRE_SLOT_HEX;
        if (i < activeSegment) {
          nextHex = CYAN_TEAL_HEX;
        } else if (i === activeSegment) {
          nextHex = activeHex;
        }

        if (lastSegHex[i] !== nextHex) {
          lastSegHex[i] = nextHex;
          _tmpColor.setHex(nextHex);
          progressInstanced.setColorAt(i, _tmpColor);
          colorChanged = true;
        }
      }

      if (colorChanged && progressInstanced.instanceColor) {
        progressInstanced.instanceColor.needsUpdate = true;
      }

      group.userData.subtitle = `Step ${rawStepIndex + 1} / ${rawTotalSteps} (Slice ×${sliceWidth}) · ${phase}`;
    }
```
