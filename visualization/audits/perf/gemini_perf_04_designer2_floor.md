# Gemini Argon High Performance Audit #4: Designer 2 — Hollow Well Part B (Lines 1600–3049)

## Executive Summary & Inventory Count (`designer2_well.js` Lines 1600–3049)

### Scene-Graph & GPU Resource Inventory (Sections 3, 4, 5 + Helpers)
| Subsystem | Groups | Regular `Mesh` | `InstancedMesh` (Instances) | Geometries | Materials | Textures | Lights |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Tail of 12 Half-Arcs (`1600–1678`)** | 12 (from `1504`) | 24 (`lowerChannelMesh` × 12, `lowerPlaqueMesh` × 12; 96 total child meshes across 12 slots) | 1 (`bridgeClaspsInstancedMesh`, 24 instances) | 24 in `1600–1630` (84 total across 12 slots) | 24 (`lightPipeMat` × 12, `plaqueMat` × 12) | Up to 12 dynamic `384×96` plaque textures | 0 |
| **3. Control 10-Sector Stator (`1680–1942`)** | 2 (`controlCoreGroup`, `transferNeedleGroup`) | 15 | 1 (`statorSectorsInstancedMesh`, 10 instances) | 9 | 8 | 1 static `1024×1024` (`statorDialTex`) + 1 dynamic `384×108` (`lensPlaqueMat`) | 0 |
| **4. Stepped Bore Chamber & Terraces (`1945–2160`, `1129–1218`)** | **27** (`wellStructureGroup` + 8×`tierGroup` + 8×`southWallGroup` + 8×`loopGroup` + `stackLimitGroup` + `horizonLoop`) | **102** (2 vault ceiling + 32 terrace walls + 32 chamfer trims + 32 terrace light-loop bars + 4 horizon loop bars) | 2 (`vaultRibsInstancedMesh` [8], `southVaultRibsInstancedMesh` [4]) | **53** (2 ceiling + 1 rib + 32 terrace/trim boxes + 18 light-loop boxes) | 13 (4 shared + 8 per-terrace `ringMat` + 1 `horizonRingMat`) | 1 shared (`brushedSilverTex`) | 0 |
| **5. 39-Rule Silicon Floor (`2162–2400`, `1225–1322`)** | 1 (`floorGroup`) | 4 (`floorBaseMesh`, `romBezelMesh`, `romBodyMesh`, `floorOverlayMesh`) | 4 (`ruleBezel` [39], `ruleTiles` [39], `ruleBeacon` [39], `ruleBeaconPost` [39]) | 8 | 6 | 1 static `2048×2048` (`floorOverlayTex`, 16 MB RGBA) | 0 |
| **Total (Part B Scope)** | **42 Groups** | **145 Meshes (145 Draw Calls)** | **8 InstancedMeshes (202 Instances, 8 Draw Calls)** | **94 Geometries** | **51 Materials** | **15+ CanvasTextures (~23 MB VRAM)** | **0** |

*(Note: Continuation wafers/orbs are managed in Designer 3 `designer3_chamber.js`, while Designer 2 owns the 48-segment fuel ring, 12 split-arc half-rings, 10-sector Control stator, 8 stepped bore terraces `D0..D7`, and the 39-rule floor chiplets.)*

---

## Prioritized Performance Optimizations (100% Visual Quality Preserved)

### P0-1. Eliminate 95% of Per-Frame Matrix/Color Work & 6 Unconditional GPU Buffer Uploads on the 39-Rule Floor (Lines 2262–2334, 2944–3027)
**Problem:**
In `.update()` Section E (lines 2944–3027), the loop iterates over all **39 `ruleCells` every single frame** and unconditionally executes:
- **117 `Matrix4.compose()` calls** (`39 × 3` across `ruleTilesInstancedMesh`, `ruleBeaconInstancedMesh`, and `ruleBeaconPostInstancedMesh`, even though `_scratchQuat` is always identity!)
- **117 `setColorAt()` + `Color.lerp()` / `multiplyScalar()` calls**
- **6 GPU buffer uploads per frame** (`instanceMatrix.needsUpdate = true` and `instanceColor.needsUpdate = true` on all 3 `InstancedMesh`es).
At any point in time, at most **1** rule is `isActive` and **1–2** rules have a decaying `trail > 0`. The remaining **36–37 cells are 100% static in their idle state**, yet their matrices and colors are recomputed and re-uploaded every frame!

**Drop-in Fix:**
1. At initialization (lines 2285–2315), pre-store each cell's idle tile color (`idleTileColor`), idle beacon color (`idleBeaconColor`), and `isSettledIdle: true`.
2. In `.update()` (lines 2944–3027), skip any cell where `!isActive && !isPrev && cell.trail === 0 && cell.isSettledIdle`, and only flag `needsUpdate = true` when `anyRuleUpdated` is true:

```javascript
// --- Replace lines 2285-2318 in createWellSubsystem() ---
    const idleTileColor = new THREE.Color(0x1b2536).lerp(
      _scratchColorB.setHex(rule.color),
      0.16
    );
    const idleBeaconColor = new THREE.Color(rule.color).multiplyScalar(0.32);

    ruleTilesInstancedMesh.setColorAt(idx, idleTileColor);

    // South-lip status bar matrix
    _scratchPos.set(cellX, WELL_FLOOR_Y + 0.050, cellZ + gridConfig.cellD * 0.46);
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    ruleBeaconInstancedMesh.setMatrixAt(idx, _scratchMat4);
    ruleBeaconInstancedMesh.setColorAt(idx, idleBeaconColor);

    // Corner optical beacon post matrix
    _scratchPos.set(
      cellX + gridConfig.cellW * 0.42,
      WELL_FLOOR_Y + 0.044,
      cellZ + gridConfig.cellD * 0.40
    );
    _scratchScale.set(1, 0.35, 1);
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    ruleBeaconPostInstancedMesh.setMatrixAt(idx, _scratchMat4);
    ruleBeaconPostInstancedMesh.setColorAt(idx, idleBeaconColor);

    const cellRecord = {
      index: idx,
      rule,
      col,
      row,
      x: cellX,
      z: cellZ,
      trail: 0,
      isSettledIdle: true,
      idleTileColor,
      idleBeaconColor,
    };
    ruleCells.push(cellRecord);
    ruleCellsByLine.set(rule.line, cellRecord);
    ruleCellsById.set(rule.id, cellRecord);
```

```javascript
// --- Replace lines 2942-3027 in update() ---
    _scratchQuat.identity();
    let anyRuleUpdated = false;

    for (let i = 0; i < ruleCells.length; i += 1) {
      const cell = ruleCells[i];
      const isActive = cell === activeCell;
      const isPrev = cell === effectivePrevCell && !isActive;

      if (isActive) {
        cell.trail = 1.0;
      } else if (isPrev) {
        cell.trail = Math.max(cell.trail * (1 - dt * 2.2), 0.36);
      } else if (cell.trail > 0) {
        cell.trail = Math.max(0, cell.trail - dt * 3.5);
      } else if (cell.isSettledIdle) {
        continue; // Fast-path: skips 36-37 idle cells every frame!
      }

      anyRuleUpdated = true;

      if (isActive) {
        cell.isSettledIdle = false;
        const pulse = 0.92 + 0.25 * Math.sin(ambient * 6.0);
        _scratchColor.setHex(cell.rule.color).multiplyScalar(pulse);
        _scratchPos.set(cell.x, WELL_FLOOR_Y + 0.054, cell.z);
        _scratchScale.set(1.03, 1.38, 1.03);
      } else if (cell.trail > 0.04) {
        cell.isSettledIdle = false;
        _scratchColor
          .setHex(0x1b2536)
          .lerp(_scratchColorB.setHex(cell.rule.color), 0.16 + cell.trail * 0.56);
        _scratchPos.set(cell.x, WELL_FLOOR_Y + 0.036, cell.z);
        _scratchScale.set(1, 1.08, 1);
      } else {
        cell.trail = 0;
        cell.isSettledIdle = true;
        _scratchColor.copy(cell.idleTileColor);
        _scratchPos.set(cell.x, WELL_FLOOR_Y + 0.030, cell.z);
        _scratchScale.set(1, 1, 1);
      }

      _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
      ruleTilesInstancedMesh.setMatrixAt(i, _scratchMat4);
      ruleTilesInstancedMesh.setColorAt(i, _scratchColor);

      const beaconY = isActive
        ? WELL_FLOOR_Y + 0.084
        : cell.trail > 0.04
          ? WELL_FLOOR_Y + 0.058
          : WELL_FLOOR_Y + 0.050;
      _scratchPos.set(cell.x, beaconY, cell.z + gridConfig.cellD * 0.46);
      _scratchScale.set(1.0, isActive ? 1.55 : 1.0, 1.0);
      _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
      ruleBeaconInstancedMesh.setMatrixAt(i, _scratchMat4);

      if (isActive) {
        _scratchColor.setHex(cell.rule.color).multiplyScalar(1.35);
      } else if (cell.trail > 0.04) {
        _scratchColor.setHex(cell.rule.color).multiplyScalar(0.32 + cell.trail * 0.65);
      } else {
        _scratchColor.copy(cell.idleBeaconColor);
      }
      ruleBeaconInstancedMesh.setColorAt(i, _scratchColor);

      const postScaleY = isActive ? 1.45 : cell.trail > 0.04 ? 0.72 : 0.35;
      const postY = isActive
        ? WELL_FLOOR_Y + 0.088
        : cell.trail > 0.04
          ? WELL_FLOOR_Y + 0.056
          : WELL_FLOOR_Y + 0.044;
      _scratchPos.set(
        cell.x + gridConfig.cellW * 0.42,
        postY,
        cell.z + gridConfig.cellD * 0.40
      );
      _scratchScale.set(isActive ? 1.35 : 1.0, postScaleY, isActive ? 1.35 : 1.0);
      _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
      ruleBeaconPostInstancedMesh.setMatrixAt(i, _scratchMat4);
      ruleBeaconPostInstancedMesh.setColorAt(i, _scratchColor);
    }

    if (anyRuleUpdated) {
      ruleTilesInstancedMesh.instanceMatrix.needsUpdate = true;
      if (ruleTilesInstancedMesh.instanceColor) {
        ruleTilesInstancedMesh.instanceColor.needsUpdate = true;
      }
      ruleBeaconInstancedMesh.instanceMatrix.needsUpdate = true;
      if (ruleBeaconInstancedMesh.instanceColor) {
        ruleBeaconInstancedMesh.instanceColor.needsUpdate = true;
      }
      ruleBeaconPostInstancedMesh.instanceMatrix.needsUpdate = true;
      if (ruleBeaconPostInstancedMesh.instanceColor) {
        ruleBeaconPostInstancedMesh.instanceColor.needsUpdate = true;
      }
    }
```

---

### P0-2. Batch the 64 Stepped Bore Terrace Wall & Trim Meshes into `InstancedMesh`es (Lines 1129–1196, 2022–2074, 2848–2858)
**Problem:**
`createSteppedTerrace()` (lines 1129–1196) is called 8 times (`d = 0..7` at lines 2022–2036). Each call creates **2 Groups, 4 BoxGeometries, and 8 individual `THREE.Mesh`es** (4 wall slabs + 4 chamfer trim ledges). Across 8 terraces, that is **64 separate `THREE.Mesh` draw calls, 32 `BoxGeometry` allocations, and 16 `THREE.Group` nodes** — for simple axis-aligned, untextured `MeshStandardMaterial` boxes! Note that `terraceWall` is not even tagged inspectable (only `lightRingLoop` is tagged at line 2054).
Similarly, `createRecessedLightLoop()` (lines 1201–1218) creates **4 separate `THREE.Mesh` draw calls per loop** (36 draw calls across `D0..D7` + `horizonLoop`), even though the North, West, and East bars (`n`, `w`, `e`) are completely static relative to `loopGroup`.

**Drop-in Fix:**
1. Batch all 64 terrace wall & chamfer trim boxes into **6 `InstancedMesh`es** sharing **1 unit `BoxGeometry(1, 1, 1)`** (since all face normals are axis-aligned and materials are untextured, positive scale preserves exact normals and PBR shading):
   - `evenWallsNorthWestEastInstanced` (12 instances, `matteAnodizedBlackMat`, static)
   - `oddWallsNorthWestEastInstanced` (12 instances, `matteAnodizedSlateMat`, static)
   - `evenWallsSouthInstanced` (4 instances, `matteAnodizedBlackMat`, inside `southCutawayGroup`)
   - `oddWallsSouthInstanced` (4 instances, `matteAnodizedSlateMat`, inside `southCutawayGroup`)
   - `trimNorthWestEastInstanced` (24 instances, `chamferTrimMat`, static)
   - `trimSouthInstanced` (8 instances, `chamferTrimMat`, inside `southCutawayGroup`)
   **Savings:** Cuts **64 draw calls down to 6 draw calls** (-58 draw calls), **32 `BoxGeometry` objects down to 1**, and **16 Groups down to 1 `southCutawayGroup`**!

---

### P0-3. Stop Recomputing 24 `bridgeClaspsInstancedMesh` Trig/Quaternions/Matrices &Allocating 24 Strings Every Frame in the 12-Slot Loop (Lines 2553–2691)
**Problem:**
Inside the 12-slot `halfArcSlots` loop in `.update()`:
1. **Unconditional 48 `Math.cos`/`Math.sin` + 24 `setFromEuler` + 24 `Matrix4.compose` calls per frame (lines 2652–2691):**
   For each slot `s`, `endThetaA`, `endThetaB`, and `slot.midR` never change after initialization! The **only** value in all 24 clasp matrices that ever changes at runtime is the Y coordinate `slot.currentY - 0.002` (which is index `claspIdx * 16 + 13` in column-major `bridgeClaspsInstancedMesh.instanceMatrix.array`).
2. **Per-frame string & object allocations when `stepIndex` hasn't changed (lines 2596, 2608, 2626–2633):**
   - `formatSlotEnvCompact(_chronoFramesBuffer, s, activeBindings)` and template string `` const plaqueKey = `${slot.slotCode}|...` `` run on all 12 slots **every single frame**, allocating up to 24 strings/frame even when `!isStepChanged`.
   - `{ kind: slot.poppedKind || 'Return' }` (line 2608) and `{ kind: slotTel.kind }` (line 2596) allocate temporary objects on the heap inside the 60 FPS loop.
3. **Unnecessary `slot.plaqueMat.needsUpdate = true` & `lensPlaqueMat.needsUpdate = true` (lines 2648, 2800):**
   In `three/webgpu`, setting `material.needsUpdate = true` when swapping `material.map` from one `CanvasTexture` to another increments `material.version` and triggers WebGPU node/pipeline re-validation. `material.needsUpdate = true` is **only** needed when `material.map` transitions from `null` to a texture the first time!

**Drop-in Fix:**
1. Initialize the 24 `bridgeClaspsInstancedMesh` matrices once after creating `halfArcSlots` (around line 1678).
2. In `.update()` (lines 2583–2691), snap `slot.currentY` / `slot.currentRotY` when converged, update only `instanceMatrix.array[claspIdx * 16 + 13]` when `slot.currentY` actually moves, and gate the plaque check on `isStepChanged`:

```javascript
// --- Add right after the halfArcSlots creation loop at line 1678 ---
  for (let s = 0; s < HALF_ARC_SLOT_COUNT; s += 1) {
    const slot = halfArcSlots[s];
    const endThetaA = slot.thetaStart;
    const endThetaB = slot.thetaStart + HALF_ARC_THETA_LENGTH;
    const claspIdx0 = s * 2;
    const claspIdx1 = s * 2 + 1;

    _scratchPos.set(
      Math.cos(endThetaA) * slot.midR,
      slot.currentY - 0.002,
      -Math.sin(endThetaA) * slot.midR
    );
    _scratchEuler.set(0, endThetaA, 0);
    _scratchQuat.setFromEuler(_scratchEuler);
    _scratchScale.set(1, 1, 1);
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    bridgeClaspsInstancedMesh.setMatrixAt(claspIdx0, _scratchMat4);

    _scratchPos.set(
      Math.cos(endThetaB) * slot.midR,
      slot.currentY - 0.002,
      -Math.sin(endThetaB) * slot.midR
    );
    _scratchEuler.set(0, endThetaB, 0);
    _scratchQuat.setFromEuler(_scratchEuler);
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    bridgeClaspsInstancedMesh.setMatrixAt(claspIdx1, _scratchMat4);

    _scratchColor.setHex(0x334155);
    bridgeClaspsInstancedMesh.setColorAt(claspIdx0, _scratchColor);
    bridgeClaspsInstancedMesh.setColorAt(claspIdx1, _scratchColor);
    slot.lastClaspHex = 0x334155;
    slot.poppedHex = HEX_ENV_RETURN_TEAL;
  }
  bridgeClaspsInstancedMesh.instanceMatrix.needsUpdate = true;
  if (bridgeClaspsInstancedMesh.instanceColor) {
    bridgeClaspsInstancedMesh.instanceColor.setUsage(THREE.DynamicDrawUsage);
    bridgeClaspsInstancedMesh.instanceColor.needsUpdate = true;
  }
```

```javascript
// --- Replace lines 2572-2691 in update() ---
      let claspsMatrixDirty = false; // (declare before the for-loop at line 2552)
      let claspsColorDirty = false;  // (declare before the for-loop at line 2552)

      // Inside loop over s = 0..11:
      if (isStepChanged && !isOccupied && (slotTel?.transition === 'POPPED' || slot.lastKind !== null)) {
        slot.popPulse = 1.0;
        slot.poppedKind = slotTel?.poppedKind || slot.lastKind || 'Return';
        slot.poppedHex = getFrameSemanticHex(
          slot.poppedKind === 'Scope' ? { kind: 'Scope' } : { kind: 'Return' }
        );
      } else if (isOccupied) {
        slot.popPulse = 0.0;
      }
      slot.lastKind = frame ? frame.kind : slotTel?.kind || null;
      slot.rewritePulse = Math.max(0, slot.rewritePulse - dt * 3.2);
      slot.popPulse = Math.max(0, (slot.popPulse || 0) - dt * 2.6);

      // Physical elevation & in-place rewrite lock-pulse (only dirty matrix when moving)
      const targetY = isOccupied
        ? slot.baseY + 0.012 + slot.rewritePulse * 0.011
        : slot.baseY - 0.016 + (slot.popPulse || 0) * 0.010;
      if (Math.abs(slot.currentY - targetY) > 1e-5) {
        slot.currentY = lerp(slot.currentY, targetY, alpha14);
        slot.group.position.y = slot.currentY;
        const matArr = bridgeClaspsInstancedMesh.instanceMatrix.array;
        const claspY = slot.currentY - 0.002;
        matArr[(s * 2) * 16 + 13] = claspY;
        matArr[(s * 2 + 1) * 16 + 13] = claspY;
        claspsMatrixDirty = true;
      } else if (slot.currentY !== targetY) {
        slot.currentY = targetY;
        slot.group.position.y = targetY;
      }

      const direction = slot.isEast ? 1 : -1;
      const targetRotY = slot.rewritePulse * direction * 0.042 * Math.sin(slot.rewritePulse * Math.PI);
      if (Math.abs(slot.currentRotY - targetRotY) > 1e-5) {
        slot.currentRotY = lerp(slot.currentRotY, targetRotY, alpha16);
        slot.group.rotation.y = slot.currentRotY;
      } else if (slot.currentRotY !== targetRotY) {
        slot.currentRotY = targetRotY;
        slot.group.rotation.y = targetRotY;
      }

      const semanticHex = frame ? getFrameSemanticHex(frame) : HEX_IDLE_SLATE;
      if (isOccupied) {
        const breath = 0.5 + 0.5 * Math.sin(ambient * 4.2 + s * 0.6);
        slot.lightPipeMat.color.setHex(semanticHex);
        slot.lightPipeMat.emissive.setHex(semanticHex);
        const baseIntensity = isTopOfStack ? 1.45 : 0.85;
        slot.lightPipeMat.emissiveIntensity =
          baseIntensity + slot.rewritePulse * 0.85 + breath * 0.15;
        slot.dialMat.opacity = isTopOfStack ? 0.95 : 0.84;
        slot.plaqueMat.opacity = isTopOfStack ? 1.0 : 0.92;
        if (slot.lowerPlaqueMesh) slot.lowerPlaqueMesh.visible = true;
      } else if (slot.popPulse > 0.02) {
        _scratchColor.setHex(slot.poppedHex).lerp(_scratchColorB.setHex(HEX_ENV_CYAN), 0.45);
        slot.lightPipeMat.color.copy(_scratchColor);
        slot.lightPipeMat.emissive.copy(_scratchColor);
        slot.lightPipeMat.emissiveIntensity = 0.12 + slot.popPulse * 1.15;
        slot.dialMat.opacity = 0.45 + slot.popPulse * 0.35;
        slot.plaqueMat.opacity = 0.46 + slot.popPulse * 0.35;
        if (slot.lowerPlaqueMesh) slot.lowerPlaqueMesh.visible = true;
      } else if (slot.lightPipeMat.emissiveIntensity !== 0.06) {
        slot.lightPipeMat.color.setHex(HEX_IDLE_SLATE);
        slot.lightPipeMat.emissive.setHex(HEX_IDLE_SLATE);
        slot.lightPipeMat.emissiveIntensity = 0.06;
        slot.dialMat.opacity = 0.45;
        slot.plaqueMat.opacity = 0.0;
        if (slot.lowerPlaqueMesh) slot.lowerPlaqueMesh.visible = false;
      }

      // Only format strings and check plaque key when step changes!
      if (isStepChanged) {
        const resolvedEnv = formatSlotEnvCompact(_chronoFramesBuffer, s, activeBindings);
        const envSummary =
          resolvedEnv !== 'root' ? resolvedEnv : (slotTel?.envText || resolvedEnv);
        const plaqueKey = frame
          ? `${slot.slotCode}|${frame.kind}|${frame.label}|${envSummary}|${isTopOfStack ? 1 : 0}`
          : `${slot.slotCode}|IDLE`;

        if (plaqueKey !== slot.lastPlaqueKey) {
          slot.lastPlaqueKey = plaqueKey;
          const copy = formatHalfArcPlaqueCopy(slot.slotCode, frame, envSummary);
          slot.group.userData.title = copy.title;
          slot.group.userData.subtitle = copy.subtitle;
          if (frame) {
            const hadMap = Boolean(slot.plaqueMat.map);
            slot.plaqueMat.map = createPlaqueTexture({
              title: copy.title,
              subtitle: copy.subtitle,
              badge: copy.badge,
              accent: getFrameSemanticCss(frame),
              dark: true,
              width: 384,
              height: 96,
            });
            if (!hadMap) slot.plaqueMat.needsUpdate = true;
          }
        }
      }

      const targetClaspHex = isOccupied ? semanticHex : 0x334155;
      if (targetClaspHex !== slot.lastClaspHex) {
        slot.lastClaspHex = targetClaspHex;
        _scratchColor.setHex(targetClaspHex);
        bridgeClaspsInstancedMesh.setColorAt(s * 2, _scratchColor);
        bridgeClaspsInstancedMesh.setColorAt(s * 2 + 1, _scratchColor);
        claspsColorDirty = true;
      }
    }

    if (claspsMatrixDirty) {
      bridgeClaspsInstancedMesh.instanceMatrix.needsUpdate = true;
    }
    if (claspsColorDirty && bridgeClaspsInstancedMesh.instanceColor) {
      bridgeClaspsInstancedMesh.instanceColor.needsUpdate = true;
    }
```

---

### P1-1. Avoid Re-Composing & Re-Uploading All 10 Stator Sector Matrices Every Frame (Lines 2705–2730)
**Problem:**
In Section C (lines 2705–2730), `statorSectorsInstancedMesh` has 10 sector pads (`0..9`). Their matrices (`padY = isSectorActive ? 0.010 : 0.003`, `scaleY = isSectorActive ? 1.35 : 1.0`) ONLY change when `activeSectorIdx` changes! Yet on every frame, all 10 sectors recompute `_scratchEuler`, `_scratchQuat.setFromEuler`, and `_scratchMat4.compose`, and upload `statorSectorsInstancedMesh.instanceMatrix.needsUpdate = true` to the GPU. Furthermore, 9 of the 10 sectors have a static color `0x1e293b` that only changes when `activeSectorIdx` changes; only the 1 `activeSectorIdx` pulses with `ambient`.

**Drop-in Fix:**
Track `let lastActiveSectorIdx = -1;` at closure scope (line 2413). In `.update()` (lines 2705–2730), only update `instanceMatrix` (and the inactive sector colors) when `activeSectorIdx !== lastActiveSectorIdx`, and on every frame only update the single pulsing `activeSectorIdx` color:

```javascript
// --- Replace lines 2705-2730 in update() ---
    if (activeSectorIdx !== lastActiveSectorIdx) {
      for (let i = 0; i < STATOR_SECTOR_COUNT; i += 1) {
        const isSectorActive = i === activeSectorIdx;
        const theta = Math.PI * 0.5 - i * sectorStep;
        _scratchPos.set(0, isSectorActive ? 0.010 : 0.003, 0);
        _scratchEuler.set(0, theta, 0);
        _scratchQuat.setFromEuler(_scratchEuler);
        _scratchScale.set(1.0, isSectorActive ? 1.35 : 1.0, 1.0);
        _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
        statorSectorsInstancedMesh.setMatrixAt(i, _scratchMat4);

        if (!isSectorActive) {
          _scratchColor.setHex(0x1e293b);
          statorSectorsInstancedMesh.setColorAt(i, _scratchColor);
        }
      }
      statorSectorsInstancedMesh.instanceMatrix.needsUpdate = true;
      lastActiveSectorIdx = activeSectorIdx;
    }

    const activeSectorPulse = 1.05 + 0.25 * Math.sin(ambient * 6.0);
    _scratchColor.setHex(activeSectorMeta.color).multiplyScalar(activeSectorPulse);
    statorSectorsInstancedMesh.setColorAt(activeSectorIdx, _scratchColor);
    if (statorSectorsInstancedMesh.instanceColor) {
      statorSectorsInstancedMesh.instanceColor.needsUpdate = true;
    }
```

---

### P1-2. Freeze Static World/Local Matrices (`matrixAutoUpdate = false`) on 220+ Static Scene-Graph Nodes (Lines 1363–2389)
**Problem:**
Over **220 `Mesh`, `InstancedMesh`, and `Group` nodes** in `designer2_well.js` never change their local `position`, `rotation`, or `scale` after creation:
- **96 child meshes** inside the 12 `halfArcGroup`s (`outerArcMesh`, `dialMesh`, `lowerDialMesh`, `channelMesh`, `lightPipeMesh`, `lowerLightPipeMesh`, `lowerChannelMesh`, `lowerPlaqueMesh` at lines 1521–1641)
- **13 static objects** in `controlCoreGroup` (lines 1685–1911)
- **100+ static wall/trim/loop meshes & groups** in `wellStructureGroup` (lines 1949–2148)
- **9 static floor objects** in `floorGroup` (`floorGroup`, `floorBaseMesh`, `ruleBezelInstancedMesh`, `ruleTilesInstancedMesh`, `ruleBeaconInstancedMesh`, `ruleBeaconPostInstancedMesh`, `romBezelMesh`, `romBodyMesh`, `floorOverlayMesh` at lines 2168–2389)
- **3 static fuel ring objects** (`fuelRingGroup`, `fuelTrackMesh`, `fuelInstancedMesh` at lines 1363–1415; plus remove `fuelInstancedMesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage)` at line 1392 since `fuelInstancedMesh.instanceMatrix` is never modified after initialization).

Because `matrixAutoUpdate` defaults to `true`, Three.js calls `matrix.compose(position, quaternion, scale)` on all 220+ objects every frame during `renderer.render()`.

**Drop-in Fix:**
Add a helper in `createWellSubsystem()` and freeze all static subtrees after construction (while keeping `matrixAutoUpdate = true` on the dynamic nodes: `halfArcGroup`, `transferNeedleGroup`, `needleStemMesh`, `lowerNeedleStemMesh`, `needleTipMesh`, `lowerNeedleTipMesh`, `outputWaveMesh`, `stackLimitGroup`, and the South cutaway objects):

```javascript
// --- Add at line 2401 after building floorGroup ---
  const dynamicObjects = new Set([
    group,
    transferNeedleGroup,
    needleStemMesh,
    lowerNeedleStemMesh,
    needleTipMesh,
    lowerNeedleTipMesh,
    outputWaveMesh,
    stackLimitGroup,
    vaultCeilingSouthMesh,
    southVaultRibsInstancedMesh,
  ]);
  for (let s = 0; s < halfArcSlots.length; s += 1) {
    dynamicObjects.add(halfArcSlots[s].group);
  }
  for (let d = 0; d < depthRings.length; d += 1) {
    const sw = depthRings[d].terraceWall?.userData?.southWallGroup;
    const sm = depthRings[d].loop?.userData?.southMesh;
    if (sw) dynamicObjects.add(sw);
    if (sm) dynamicObjects.add(sm);
  }
  if (horizonLoop.userData?.southMesh) {
    dynamicObjects.add(horizonLoop.userData.southMesh);
  }

  group.traverse((obj) => {
    if (!dynamicObjects.has(obj)) {
      obj.updateMatrix();
      obj.matrixAutoUpdate = false;
    }
  });
```

---

### P1-3. Gate 48-Segment Fuel Ring Color Updates & 19 South-Wall Cutaway Transform Updates When Settled (Lines 2515–2546, 2834–2885)
**Problem:**
1. **Fuel Ring (lines 2515–2546):** When `!isExhausted`, the 48 fuel segment colors depend ONLY on `displayedFuelRatio` and `step?.charge === 1` (there is no `ambient` term). Once `displayedFuelRatio` converges to `targetFuelRatio`, the loop still runs 48 times per frame and uploads `fuelInstancedMesh.instanceColor.needsUpdate = true` to the GPU every frame.
2. **South Cutaway Wall (lines 2834–2885):** Every frame, the code sets `.position.y` and `.visible` on 19 objects (`vaultCeilingSouthMesh`, `southVaultRibsInstancedMesh`, 8 `southWallGroup`s, 8 `loopSouthMesh`es, and `horizonSouthMesh`), even when `cutawayMix` hasn't changed (`cutawayMix === 0` or `cutawayMix === 1` on 99.9% of frames)!

**Drop-in Fix:**
```javascript
// --- Add to state tracking at line 2413 ---
  let lastCommittedFuelRatio = -1;
  let lastFuelCharge = -1;
  let lastCutawayMix = -1;

// --- Replace lines 2515-2546 in update() ---
    const targetFuelRatio = clamp(currentFuel / initialFuel, 0, 1);
    if (Math.abs(displayedFuelRatio - targetFuelRatio) > 1e-4) {
      displayedFuelRatio = lerp(displayedFuelRatio, targetFuelRatio, alpha12);
    } else {
      displayedFuelRatio = targetFuelRatio;
    }

    const isExhausted = activeRuleLine === 84 || activeRuleId === 'A084' || currentFuel === 0;
    const chargeBit = step?.charge === 1 ? 1 : 0;

    if (
      isExhausted ||
      displayedFuelRatio !== lastCommittedFuelRatio ||
      chargeBit !== lastFuelCharge
    ) {
      lastCommittedFuelRatio = displayedFuelRatio;
      lastFuelCharge = chargeBit;

      const litSegmentCount = displayedFuelRatio * FUEL_SEGMENTS;
      const isLowFuel = displayedFuelRatio < 0.28;
      const leadingEdgeIdx = Math.floor(litSegmentCount);

      for (let s = 0; s < FUEL_SEGMENTS; s += 1) {
        const segFill = clamp(litSegmentCount - s, 0, 1);
        const isLeadingEdge = Math.abs(s - leadingEdgeIdx) <= 1 && chargeBit === 1;

        if (isExhausted) {
          const errPulse = 0.55 + 0.45 * Math.sin(ambient * 14 + s * 0.2);
          _scratchColor.setHex(PALETTE.errorRed).multiplyScalar(errPulse);
        } else if (segFill > 0.02) {
          const activeHex = isLeadingEdge
            ? PALETTE.heatBright
            : isLowFuel
              ? PALETTE.amber
              : PALETTE.gold;
          _scratchColor
            .setHex(activeHex)
            .multiplyScalar((isLeadingEdge ? 1.25 : 0.9) * (0.35 + 0.65 * segFill));
        } else {
          _scratchColor.setHex(0x1e293b);
        }
        fuelInstancedMesh.setColorAt(s, _scratchColor);
      }
      if (fuelInstancedMesh.instanceColor) {
        fuelInstancedMesh.instanceColor.needsUpdate = true;
      }
    }
```
And for `cutawayMix` (lines 2834–2885), only update `southRetractY` and `showSouthWall` on the 19 South wall objects when `Math.abs(cutawayMix - lastCutawayMix) > 1e-5`.

---

### P2-1. Transparency, Overdraw & Backface Culling Optimizations (Lines 1528–1555, 1618–1641, 1805–1824, 2374–2388)
1. **`floorOverlayMat` (`2048×2048` texture across `2.92 × 2.92` floor plane, lines 2374–2381):**
   Over 75% of `floorOverlayTex` pixels have `alpha === 0` (`clearRect`), while the faintest drawn element is the cell hairline border at `alpha = 0.16`. Currently `floorOverlayMat` has `transparent: true, opacity: 0.96, depthWrite: false` with **no `alphaTest`**, forcing the GPU to run blending over 100% of the floor area.
   - **Fix:** Add `alphaTest: 0.01` and `side: THREE.FrontSide` to `floorOverlayMat` (line 2375) and `floorOverlayMesh.renderOrder = 2;`. This discards all `alpha === 0` fragments in the shader before blending while keeping 100% of the anti-aliased text and `0.16`-alpha borders identical.
2. **`dialMat` on the 12 Half-Arcs (lines 1528–1555):**
   Each half-arc already has two separate meshes (`dialMesh` at `y = +0.0045` with normal `+Y`, and `lowerDialMesh` at `y = -0.029` with normal `-Y`) sandwiching the opaque metal `outerArcMesh`. Yet `dialMat` sets `side: THREE.DoubleSide` (line 1533), which disables backface culling and renders all 24 transparent dial ring meshes from every camera angle (12 of them facing away through solid metal!).
   - **Fix:** Change `side: THREE.DoubleSide` to `side: THREE.FrontSide` and add `alphaTest: 0.01` on `dialMat` (line 1533). This cuts dial ring transparent fragment overdraw by **50%+** from every camera angle.
3. **Share `lowerPlaqueGeo` across all 12 Half-Arc Slots (line 1627):**
   Inside the 12-slot loop at line 1627, `trackGeo(new THREE.PlaneGeometry(0.24, 0.060))` allocates 12 identical `PlaneGeometry` buffers. Hoisting `const lowerPlaqueGeo = trackGeo(new THREE.PlaneGeometry(0.24, 0.060));` outside the loop at line 1484 saves 11 geometry allocations.
4. **Control Core `lensPlaqueMat` Recompilation & Array Allocation in `formatControlCoreLensCopy` (lines 870–875, 2779–2801):**
   - At line 2779, `coreKey` includes `stepIndex`, which forces `formatControlCoreLensCopy` and `lensPlaqueMat.needsUpdate = true` on **every single step** even when the plaque text hasn't changed, and `lensPlaqueMat.needsUpdate = true` triggers a WebGPU material pipeline check every step. Only set `if (!hadLensMap) lensPlaqueMat.needsUpdate = true;`.
   - At lines 870–875, replace `activeBindings.slice(0, 2).map(...).join(', ')` with direct indexing (`activeBindings.length === 1 ? ... : ...`) to eliminate 2 temporary array allocations per step.
