# Performance Audit 03 — Designer 2 Hollow Well, Part A

**Auditor:** Grok 4.7 xhigh Performance Auditor #3 (Split-Arc Rings, Stator & Control Core)
**File:** `src/designers/designer2_well.js`
**Scope read:** lines 1–1600, the rest of the ring loop and the control-core builder (1601–1942, the ring `for` does not close until 1678 and the 10 sector pads are built at 1755–1801), and `update()` (2417–3028) sections A–C. Chamber terraces and the 39-rule floor (section D from 2830 and section E from 2905) belong to Part B. One coupling is called out: `lastStepIndex` is written at line 2932.
**Constraint:** preserve the rendered rings, stator, and control core. No dropped segments, no dropped shadows, no atlas bleeding, no shared emissive across slots that currently differ.
**Date:** 2026-10-18

No source file was modified.

---

## 1. Verdict

Startup cost and the steady-state CPU path are both larger than the draw-call shape suggests. The 10 sector pads, the 24 bridge clasps, and the fuel segments are already instanced. The 12 half-arcs are not. Each of `R1.A` … `R6.B` builds its own chamfered extrude, dial ring, channel ring, light-pipe torus, and soffit ring, even though east is a rigid yaw of west by exactly −π. That is a pure duplicate.

Every frame, `update()` then:

- recomposes local matrices for ~100 static children (`matrixAutoUpdate` stays at the default `true`)
- rebuilds 24 clasp matrices and 10 sector matrices from Euler angles whose only changing term is a Y translation or a sector index
- calls `Color.setHex` on materials and instance colors that are piecewise constant, re-running the sRGB transfer
- builds plaque key strings and, on every step, sets `material.needsUpdate`, which bumps `Material.version` and forces the WebGPU render-object cache to recheck (and possibly rebuild) the pipeline

Module-scope scratch objects at 666–673 already prevent `new THREE.Color` / `new THREE.Vector3` inside `update()`. The remaining heap traffic is object literals, `?? []`, and strings. The remaining color cost is `setHex`, not `new`.

Apply in this order: share east/west geometry, freeze static matrices, stop recompiling plaque materials, dirty-check sections B and C, pre-linearize the finite hex set.

---

## 2. Already correct — do not regress

| What | Where | Why it stays |
| :--- | :--- | :--- |
| Scratch `Matrix4`, `Vector3`, `Quaternion`, `Euler`, two `Color`s | 666–673 | Hot path already reuses these |
| Chronological frame buffer | 2403 | 12-slot array, no per-frame `new Array` |
| Even / odd outer metals | 1440–1458 | One `MeshPhysicalMaterial` per parity, `brushedSilverTex` shared |
| Dark sapphire channel material | 1459–1465 | One material, 12 meshes |
| Clasp `InstancedMesh` | 1467–1482 | 24 clasps, 1 draw |
| Sector `InstancedMesh` | 1755–1801 | 10 pads, 1 extruded geometry, 1 draw |
| Upper/lower dial geometry within a slot | 1537–1555 | Two meshes, one `RingGeometry` |
| Upper/lower light pipe within a slot | 1577–1598 | Two meshes, one torus, one material |
| Collets, needle, stator dial | 1733–1753, 1814–1824, 1853–1870 | Top and bottom already share a geometry |
| Plaque pixel cache | `theme.js` 329–420 | `createPlaqueTexture` is an LRU. The well misses it by rebinding and by bumping `needsUpdate` |
| Fuel segments | 1383–1415 | 48 instances, 1 draw. Usage hint and the color loop are the leftover |

Do not scale one master ring onto the other five. Arc width is `0.092` for `r = 0..4` and `0.078` for `r = 5` (1490), while `outerR` lerps from `0.95` to `0.31` (596–597, 1489). Width is not proportional to radius, and the light-pipe tube radius is a constant `0.012` (1578). A uniform scale changes both the annulus and the tube.

Do not put slot A and slot B light pipes in one `InstancedMesh`. `instanceColor` multiplies diffuse only (`NodeMaterial.js` 845–850). Emissive stays `materialEmissive` (`NodeMaterial.js` 1113–1119). A and B currently have different emissive hexes (2596–2618). Instancing them onto one material collapses that. Upper and lower of the **same** slot already share a material and can be instanced (finding 6).

Do not cut `curveSegments: 36`, torus `10 × 36`, sector `curveSegments: 12`, or `castShadow` / `receiveShadow` on the outer arcs (1522–1523). Those change the silhouette or the lighting.

---

## 3. P0 — Share east/west geometry (pixel-identical where it matters)

### 3.1 The identity

```
WEST_THETA_START =  0.54π     (600)
EAST_THETA_START = -0.46π     (601)
EAST − WEST      = -π         exactly
```

Light-pipe bake (1580): west `rotateZ(0.56π)`, east `rotateZ(-0.44π)`. Difference is −π again.

`RingGeometry` UVs are a planar projection, not an arc-length UV (`RingGeometry.js` 93–94):

```
uv.x = (vertex.x / outerRadius + 1) / 2
uv.y = (vertex.y / outerRadius + 1) / 2
```

`createSplitArcDialTexture` draws ticks in that same plane (995–1008). A mesh yaw of −π after `rotation.x = ±π/2` (Euler order `XYZ`, so the matrix is `Ry · Rx`) is the same rigid motion as building the east shape. Glyph angle matches too: native east canvas rotation is `π/2 − shoulder`, and a west glyph carried by `Ry(−π)` picks up the same angle because `Ry(π) = Ry(−π)`. Shoulder offsets are `±0.28π` (1017) and the map is linear in theta, so both shoulders land on the east arc.

**Consequence for textures:** keep 12 dial textures (the slot code differs: `R1A` vs `R1B`). Author **both** of them in the west angular frame (`isEast = false`) and put the −π yaw on the east meshes. Feeding today’s east-framed texture to a shared west geometry mis-registers ticks and slot codes.

Brushed-metal UVs on the extrude are the one non-pixel item. Front-cap UVs follow the shape’s 2D points. A shared west extrude yawed onto the east shows the west grain rotated 180°. Streaks stay horizontal (`createBrushedMetalTexture` fills full-width rows). Silhouette, bevel, normals, roughness, metalness, and clearcoat match. If a later pass needs bit-identical grain phase, keep 12 outer extrudes and still share the five untextured geometries below. Quality does not drop either way.

Radii still differ per ring index. Share **within** a ring (A with B), not across `R1`…`R6`.

### 3.2 What the loop allocates today

Inside `for (r)` / `for (side)` at 1487–1677, each of the 12 slots does:

| Geometry | Lines | Count now | Count after |
| :--- | :--- | ---: | ---: |
| `createChamferedArcGeometry` extrude | 1510–1520 | 12 | 6 |
| Dial `RingGeometry` | 1537–1546 | 12 | 6 |
| Channel `RingGeometry` | 1559–1568 | 12 | 6 |
| Light-pipe `TorusGeometry` | 1577–1581 | 12 | 6 |
| Soffit channel `RingGeometry` | 1601–1610 | 12 | 6 |
| Plaque `PlaneGeometry(0.24, 0.060)` | 1626–1628 | 12 | 1 |

`trackGeo` (1339–1341) must see each shared geometry **once**. Pushing the same buffer twice makes `dispose()` (3030–3033) dispose it twice.

`createChamferedArcGeometry` (940–971) also builds a `THREE.Shape` and an `ExtrudeGeometry` with bevels. That cost is init-only and it dominates this section. Halving it is the startup win. Draw-call count does not change until finding 6.

### 3.3 Replacement — build once per ring, yaw the east meshes

Replace the body of the `r` loop from the `ringOuterMat` line (1495) through the end of the `side` loop (1677) with the structure below. Slot bookkeeping (`halfArcSlots.push`, `tagInspectable`, plaque azimuth) stays as it is today. Plaques stay on `halfArcGroup` with **no** extra yaw. The rewrite pulse remains `halfArcGroup.rotation.y` (2590–2593) and composes outside the east yaw, which is the current motion.

```javascript
const ringOuterMat = r % 2 === 0 ? halfArcOuterMatEven : halfArcOuterMatOdd;
const channelWidth = Math.max(0.024, arcWidth * 0.36);
const lowerChannelWidth = Math.max(0.036, arcWidth * 0.68);

// Canonical west frame. East is this geometry plus mesh yaw −π.
const outerArcGeo = trackGeo(createChamferedArcGeometry({
  innerR,
  outerR,
  thetaStart: WEST_THETA_START,
  thetaLength: HALF_ARC_THETA_LENGTH,
  thickness: 0.028,
  bevel: 0.004,
  curveSegments: 36,
}));
const dialGeo = trackGeo(new THREE.RingGeometry(
  innerR + 0.004,
  outerR - 0.004,
  36, 1,
  WEST_THETA_START,
  HALF_ARC_THETA_LENGTH,
));
const channelGeo = trackGeo(new THREE.RingGeometry(
  midR - channelWidth * 0.5,
  midR + channelWidth * 0.5,
  36, 1,
  WEST_THETA_START + 0.018,
  HALF_ARC_THETA_LENGTH - 0.036,
));
const lightPipeGeo = trackGeo(
  new THREE.TorusGeometry(midR, 0.012, 10, 36, LIGHT_PIPE_THETA_LENGTH),
);
lightPipeGeo.rotateZ(0.56 * Math.PI);
lightPipeGeo.rotateX(-Math.PI * 0.5);
const lowerChannelGeo = trackGeo(new THREE.RingGeometry(
  midR - lowerChannelWidth * 0.5,
  midR + lowerChannelWidth * 0.5,
  36, 1,
  WEST_THETA_START + 0.014,
  HALF_ARC_THETA_LENGTH - 0.028,
));

for (let side = 0; side < 2; side += 1) {
  const isEast = side === 1;
  const yaw = isEast ? -Math.PI : 0;
  // ... slotIdx, slotCode, halfArcGroup unchanged ...

  const outerArcMesh = new THREE.Mesh(outerArcGeo, ringOuterMat);
  outerArcMesh.rotation.y = yaw;
  outerArcMesh.castShadow = true;
  outerArcMesh.receiveShadow = true;
  halfArcGroup.add(outerArcMesh);

  // West angular frame for BOTH sides. Slot code still differs.
  // isEast is intentionally false — see §3.1.
  const dialTex = trackTex(createSplitArcDialTexture(slotCode, false));
  const dialMat = trackMat(new THREE.MeshBasicMaterial({
    map: dialTex,
    transparent: true,
    opacity: 0.78,
    depthWrite: false,
    side: THREE.DoubleSide,
  }));
  const dialMesh = new THREE.Mesh(dialGeo, dialMat);
  dialMesh.rotation.set(-Math.PI * 0.5, yaw, 0);
  dialMesh.position.y = 0.0045;
  halfArcGroup.add(dialMesh);

  const lowerDialMesh = new THREE.Mesh(dialGeo, dialMat);
  lowerDialMesh.rotation.set(Math.PI * 0.5, yaw, 0);
  lowerDialMesh.position.y = -0.029;
  halfArcGroup.add(lowerDialMesh);

  const channelMesh = new THREE.Mesh(channelGeo, darkSapphireChannelMat);
  channelMesh.rotation.set(-Math.PI * 0.5, yaw, 0);
  channelMesh.position.y = 0.0048;
  halfArcGroup.add(channelMesh);

  const lightPipeMat = trackMat(new THREE.MeshStandardMaterial({
    color: HEX_IDLE_SLATE,
    emissive: HEX_IDLE_SLATE,
    emissiveIntensity: 0.06,
    roughness: 0.18,
    metalness: 0.35,
  }));
  const lightPipeMesh = new THREE.Mesh(lightPipeGeo, lightPipeMat);
  lightPipeMesh.rotation.y = yaw;
  lightPipeMesh.position.y = 0.006;
  halfArcGroup.add(lightPipeMesh);

  const lowerLightPipeMesh = new THREE.Mesh(lightPipeGeo, lightPipeMat);
  lowerLightPipeMesh.rotation.y = yaw;
  lowerLightPipeMesh.position.y = -0.026;
  halfArcGroup.add(lowerLightPipeMesh);

  const lowerChannelMesh = new THREE.Mesh(lowerChannelGeo, lightPipeMat);
  lowerChannelMesh.rotation.set(Math.PI * 0.5, yaw, 0);
  lowerChannelMesh.position.y = -0.0294;
  halfArcGroup.add(lowerChannelMesh);

  // plaque mesh: unchanged azimuth math (1630–1640), shared plane from §3.4
}
```

`thetaStart` on the slot record (1668) must stay the **real** east or west start. Clasp positions (2652–2677) use `slot.thetaStart` and `slot.midR` in world space. Do not rewrite those to the canonical west angle.

### 3.4 One plaque plane

At 1626 a new `PlaneGeometry(0.24, 0.060)` is created per slot. Hoist next to `claspGeo` (1468):

```javascript
const halfArcPlaqueGeo = trackGeo(new THREE.PlaneGeometry(0.24, 0.060));
```

Use `halfArcPlaqueGeo` in every slot. Leave the control-core plane at 1905–1907 alone (`0.25 × 0.070`).

---

## 4. P0 — `matrixAutoUpdate = false` on static transforms

`Object3D.matrixAutoUpdate` defaults to `true`. `updateMatrixWorld` then runs `Matrix4.compose` on every object, every frame, even when position, quaternion, and scale are unchanged. Children of a moving parent still refresh `matrixWorld` from the cached local matrix when the parent sets `matrixWorldNeedsUpdate`. Freezing a static child does not freeze it in the world.

Add once next to the scratch block (after 673) or inside `createWellSubsystem`:

```javascript
function freezeStatic(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
  return obj;
}
```

Call `freezeStatic` **after** the final local position / rotation / scale is assigned.

### 4.1 Freeze these

| Object | Lines to freeze after | Moves later? |
| :--- | :--- | :--- |
| `fuelRingGroup`, `fuelTrackMesh`, `fuelInstancedMesh` | 1365, 1380, 1415 | No. Fuel update writes colors only (2523–2546) |
| `splitRingsGroup` | 1438 | No |
| `bridgeClaspsInstancedMesh` | 1482 | Object transform no. Instance matrices yes (§7.2) |
| Each half-arc **child** (outer, both dials, channel, both pipes, soffit, plaque) | end of §3.3 | No. The **group** moves |
| `halfArcGroup` | do **not** freeze with the default path | Yes: `position.y`, `rotation.y` (2587–2593). Set `matrixAutoUpdate = false` and call `updateMatrix()` only when dirty (§7.1) |
| `controlCoreGroup` | 1687 | No |
| Bridge tabs, stator housing, 4 collets, sector instanced mesh, both stator dials, sapphire lens, lens plaque | 1699–1927 | No |
| `transferNeedleGroup` | 1830 | `rotation.y` (2747). Same manual pattern as the half-arc group |
| Four needle meshes | 1854–1870 | `position.x` (2773–2776). Manual `updateMatrix()` when reach changes |
| `outputWaveMesh` | 1929 | Scale and visibility while `Done` (2814–2822). Manual while the wave runs |

Texture UV matrices are a separate flag. `CanvasTexture.matrixAutoUpdate` defaults to `true` (`Texture.js` 240) and nothing in this file animates `offset` / `repeat` / `rotation`. After each `new THREE.CanvasTexture` in `createSplitArcDialTexture` (1031), `createControlStatorDialTexture` (1118), and any private plaque texture:

```javascript
tex.matrixAutoUpdate = false;
```

Identity UV matrix, no visual change. `createPlaqueTexture` in `theme.js` should get the same line if Part A is allowed to touch it; otherwise set the flag on the returned texture before assigning `material.map`.

### 4.2 Half-arc group, manual matrix

Replace 2587–2593:

```javascript
const targetY = isOccupied
  ? slot.baseY + 0.012 + slot.rewritePulse * 0.011
  : slot.baseY - 0.016 + (slot.popPulse || 0) * 0.010;
const targetRotY = slot.rewritePulse * (slot.isEast ? 1 : -1) * 0.042
  * Math.sin(slot.rewritePulse * Math.PI);

slot.currentY = lerp(slot.currentY, targetY, alpha14);
slot.currentRotY = lerp(slot.currentRotY, targetRotY, alpha16);
if (Math.abs(targetY - slot.currentY) < 1e-4) slot.currentY = targetY;
if (Math.abs(targetRotY - slot.currentRotY) < 1e-4) slot.currentRotY = targetRotY;

if (
  Math.abs(slot.group.position.y - slot.currentY) > 1e-4 ||
  Math.abs(slot.group.rotation.y - slot.currentRotY) > 1e-4
) {
  slot.group.position.y = slot.currentY;
  slot.group.rotation.y = slot.currentRotY;
  slot.group.updateMatrix();
}
```

`1e-4` world units on a ring of radius ~1 is 0.1 mm. `1e-4` rad is ~0.006°. Both are invisible. Snapping onto the target is what lets the clasp upload stop (§7.2). The breath on `emissiveIntensity` (2598–2603) is a real per-frame signal and stays.

Initialize the group once, after it is parented (1507):

```javascript
halfArcGroup.matrixAutoUpdate = false;
halfArcGroup.updateMatrix();
```

---

## 5. P0 — Label textures: cache hits must not recompile the material

### 5.1 Plaques already cache pixels and then throw the cache away

`createPlaqueTexture` (`theme.js` 348–355) returns the same `CanvasTexture` for the same title, subtitle, badge, accent, size, and border. `update()` still does this on every key change (2633–2648 and 2780–2800):

```javascript
slot.plaqueMat.map = createPlaqueTexture({ ... });
slot.plaqueMat.needsUpdate = true;
```

`Material.needsUpdate = true` increments `version` (`Material.js` 1227–1230). The WebGPU renderer treats a version change as a render-object invalidation (`RenderObjects.js` 136–148) and a pipeline recheck (`WebGPUBackend.js` 2392–2409). A 384×96 label change does not change the shader. The texture is already uploaded inside `createPlaqueTexture` (`tex.needsUpdate = true` at `theme.js` 410).

`coreKey` (2779) includes `stepIndex`, so the lens plaque takes this path on **every step** even when the printed string is unchanged. `formatControlCoreLensCopy` then allocates a bindings array (871–874, `.slice().map()`).

Idle slots never get a map (the `if (frame)` guard at 2638). The first time a slot becomes occupied, `map` goes from `null` to a texture. The node graph branches on `material.map` at compile time (`NodeMaterial.js` setup of diffuse color). That first transition can need one version bump, before the mesh has been compiled, which is the call at the bottom of `createWellSubsystem` (3042) plus the first real step. Later swaps must not bump it.

### 5.2 Replacement — assign the cached texture, bump version once

Slot record gains `lastPlaqueKey` (already at 1674) and nothing else is required for the compare. Replace 2625–2649:

```javascript
if (isStepChanged || frame !== slot._plaqueFrame || isTopOfStack !== slot._plaqueTop) {
  slot._plaqueFrame = frame;
  slot._plaqueTop = isTopOfStack;
  const resolvedEnv = formatSlotEnvCompact(_chronoFramesBuffer, s, activeBindings);
  const envSummary = resolvedEnv !== 'root' ? resolvedEnv : (slotTel?.envText || resolvedEnv);
  const plaqueKey = frame
    ? `${slot.slotCode}|${frame.kind}|${frame.label}|${envSummary}|${isTopOfStack ? 1 : 0}`
    : `${slot.slotCode}|IDLE`;

  if (plaqueKey !== slot.lastPlaqueKey) {
    slot.lastPlaqueKey = plaqueKey;
    const copy = formatHalfArcPlaqueCopy(slot.slotCode, frame, envSummary);
    slot.group.userData.title = copy.title;
    slot.group.userData.subtitle = copy.subtitle;
    if (frame) {
      const tex = createPlaqueTexture({
        title: copy.title,
        subtitle: copy.subtitle,
        badge: copy.badge,
        accent: getFrameSemanticCss(frame),
        dark: true,
        width: 384,
        height: 96,
      });
      tex.matrixAutoUpdate = false;
      if (slot.plaqueMat.map !== tex) {
        const firstBind = slot.plaqueMat.map == null;
        slot.plaqueMat.map = tex;
        if (firstBind) slot.plaqueMat.needsUpdate = true;
      }
    }
  }
}
```

`isStepChanged` is computed at 2425. It stays true for the whole first call of a new index only because section E writes `lastStepIndex = stepIndex` at **2932**. A ring-only edit that removes that write will rebuild plaques every frame again. Leave line 2932 where it is.

Reference-compare `frame` plus `isTopOfStack` so a telemetry refresh that keeps `stepIndex` still rebuilds. `formatSlotEnvCompact` (791–816) allocates a string; it now runs on step changes, not at 60 Hz. Twelve slots × one template string per frame was the steady-state garbage.

Do **not** push the returned texture into `disposableTextures`. The LRU owns it (`theme.js` 412–419) and will not `dispose()` an entry that may still be bound. `dispose()` at 3037 would free a texture another subsystem, or this one on the next step, still samples.

Lens, replace 2779–2800:

```javascript
if (isStepChanged || isDoneState !== lastCoreDone || isErrorState !== lastCoreError) {
  const lensCopy = formatControlCoreLensCopy(
    step, activeSectorMeta, isDoneState, isErrorState, stepState.trace?.outcome,
  );
  const coreKey = `${lensCopy.title}|${lensCopy.subtitle}|${lensCopy.badge}|${activeSectorMeta.css}`;
  if (coreKey !== lastCorePlaqueKey) {
    lastCorePlaqueKey = coreKey;
    controlCoreGroup.userData.title = lensCopy.title;
    controlCoreGroup.userData.subtitle = `${lensCopy.badge} · ${lensCopy.subtitle}`;
    const tex = createPlaqueTexture({
      title: lensCopy.title,
      subtitle: lensCopy.subtitle,
      badge: lensCopy.badge,
      accent: activeSectorMeta.css || CSS_OP_GOLD,
      dark: true,
      width: 384,
      height: 108,
    });
    tex.matrixAutoUpdate = false;
    if (lensPlaqueMat.map !== tex) {
      const firstBind = lensPlaqueMat.map == null;
      lensPlaqueMat.map = tex;
      if (firstBind) lensPlaqueMat.needsUpdate = true;
    }
  }
}
```

Add `let lastCoreDone = false; let lastCoreError = false;` next to `lastCorePlaqueKey` (2412) and set them at the end of this block. Drop `stepIndex` from the key. Two steps that print the same lens reuse one texture and skip the uniform bind.

Give both plaque materials a 1×1 map at construction (1618 and 1897) if the first compiled shader must see a map even when the slot is idle:

```javascript
const BLANK_PLAQUE = trackTex((() => {
  const data = new Uint8Array([0, 0, 0, 0]);
  const tex = new THREE.DataTexture(data, 1, 1);
  tex.needsUpdate = true;
  tex.matrixAutoUpdate = false;
  return tex;
})());
```

Pass `map: BLANK_PLAQUE` into both `MeshBasicMaterial`s. First bind is then never `null`, and `needsUpdate` can be deleted entirely. Idle plaques are `visible = false` (1640) or `opacity = 0` (2621), so the blank is not seen.

### 5.3 Dial and stator canvases

`createSplitArcDialTexture` (976–1035) and `createControlStatorDialTexture` (1042–1122) have no cache. Each well rebuild (tests, HMR, a second subsystem) repaints 12 canvases of 512² plus one of 1024². Cache at module scope. Do not let `dispose()` free a cache hit.

```javascript
const _dialTexCache = new Map();
const _statorDialTexCache = { tex: null };

function createSplitArcDialTexture(slotCode, isEast) {
  const key = `${slotCode}|${isEast ? 1 : 0}`;
  const hit = _dialTexCache.get(key);
  if (hit) return hit;
  // ... existing body, including the document-undefined DataTexture path ...
  tex.matrixAutoUpdate = false;
  tex.userData.sharedDial = true;
  _dialTexCache.set(key, tex);
  return tex;
}

function createControlStatorDialTexture() {
  if (_statorDialTexCache.tex) return _statorDialTexCache.tex;
  // ... existing body ...
  tex.matrixAutoUpdate = false;
  tex.userData.sharedDial = true;
  _statorDialTexCache.tex = tex;
  return tex;
}
```

`trackTex` (1347–1349) must dedupe, and `dispose()` must skip shared entries:

```javascript
const trackedTextures = new Set();
const trackTex = (t) => {
  if (t && !t.userData?.sharedDial && !trackedTextures.has(t)) {
    trackedTextures.add(t);
    disposableTextures.push(t);
  }
  return t;
};
```

With §3.3, every dial is authored `isEast = false`, so the key collapses to `slotCode`. Keep the boolean in the key so a caller that still passes `true` cannot collide with a west-framed texture.

Optional init saving, still pixel-identical: paint the 28 ticks once per angular frame into an offscreen canvas and `drawImage` that canvas before stamping `slotCode`. Twelve unique composites remain (twelve slot codes). This does not change the GPU footprint. It removes 11 repeated tick passes. Do it only inside `createSplitArcDialTexture`, keyed by the angular frame, not by slot code.

---

## 6. P1 — Instanced pairs that already share a material

Geometry sharing (§3) does not remove draw calls. These do, and the materials already match so the pixels do not move.

**Outer arcs, per ring.** Both sides use `ringOuterMat` (1495, 1521). One `InstancedMesh(outerArcGeo, ringOuterMat, 2)` parented to `splitRingsGroup`, not to `halfArcGroup`. Instance 0: `T(0, currentY, 0) · Ry(currentRotY)`. Instance 1: `T(0, currentY, 0) · Ry(currentRotY − π)`. Translation is along Y and both rotations are around Y, so they commute and this equals today’s parent-group × child-yaw. `castShadow` / `receiveShadow` stay on the instanced mesh. Update an instance matrix only when that slot’s `moved` flag from §4.2 is set. Draw calls for the extrudes: 12 → 6.

**Channels, per ring.** All 12 already share `darkSapphireChannelMat`. Same pattern, 6 instanced meshes of count 2, static yaw baked into the instance matrix, Y and rewrite yaw patched when `moved`. Draw calls 12 → 6.

**Dials, per slot.** Top and bottom already share `dialGeo` and `dialMat`. One `InstancedMesh(dialGeo, dialMat, 2)` **child of `halfArcGroup`**. Instance matrices are local and static (`rotation.x = ∓π/2`, `rotation.y = yaw`, `position.y` as today). `freezeStatic` the instanced mesh and set `instanceMatrix.setUsage(THREE.StaticDrawUsage)`. The moving parent carries both. Draw calls 24 → 12. Opacity stays on the one material, which is how it works today (2604, 2613, 2620 write `slot.dialMat.opacity` once for both meshes).

**Light pipes, per slot.** Upper and lower already share `lightPipeGeo` and `lightPipeMat`. Same count-2 instanced mesh, child of the group, local Y only (`0.006` and `-0.026`) plus the east yaw. Do not merge A with B (§2). Draw calls 24 → 12.

Soffit channel and plaque stay as single meshes. They have no twin with the same geometry **and** the same material (the soffit uses `lightPipeMat` but a different geometry than the torus).

---

## 7. P1 — Dirty-check `update()` sections A–C

### 7.1 Half-arc materials

`setHex` does not allocate, but it runs the sRGB transfer (`Color.js` 204–214 → `ColorManagement.colorSpaceToWorking`) on every call. Occupied color is constant until `kind` changes. The per-frame signal is `emissiveIntensity` (the breath) and, during a pop, the lerp toward `HEX_ENV_CYAN`.

Seed a finite table once. Every hex this section writes is one of the constants at 625–635, `0x1e293b`, `0x334155`, `0x1b2536`, plus `PALETTE.*` values used by the fuel and needle paths:

```javascript
const _linearByHex = new Map();
function linearColor(hex) {
  let c = _linearByHex.get(hex);
  if (!c) {
    c = new THREE.Color(hex);
    _linearByHex.set(hex, c);
  }
  return c;
}
const _kindOnlyFrame = { kind: '' };
function semanticHexForKind(kind) {
  _kindOnlyFrame.kind = kind || '';
  return getFrameSemanticHex(_kindOnlyFrame);
}
```

Pre-seed `HEX_IDLE_SLATE`, `HEX_ENV_CYAN`, `HEX_OP_AMBER`, `HEX_OP_GOLD`, `HEX_CRIMSON`, `0x1e293b`, `0x334155`, and each `CONTROL_STATOR_SECTORS[i].color` so the first frame does not allocate.

Replace the synthetic object literals at 2596 and 2608:

```javascript
const semanticHex = frame
  ? getFrameSemanticHex(frame)
  : (slotTel?.occupied ? semanticHexForKind(slotTel.kind) : HEX_IDLE_SLATE);
```

```javascript
const popHex = semanticHexForKind(slot.poppedKind || 'Return');
```

`getFrameSemanticHex(frame)` on the real frame allocates nothing. `{ kind: slotTel.kind }` at 2596 and `{ kind: slot.poppedKind || 'Return' }` at 2608 allocate one object per occupied-telemetry slot and per popping slot, every frame.

Replace 2597–2623’s color writes:

```javascript
if (isOccupied) {
  if (slot.lastPipeHex !== semanticHex) {
    slot.lastPipeHex = semanticHex;
    const c = linearColor(semanticHex);
    slot.lightPipeMat.color.copy(c);
    slot.lightPipeMat.emissive.copy(c);
  }
  const breath = 0.5 + 0.5 * Math.sin(ambient * 4.2 + s * 0.6);
  const baseIntensity = isTopOfStack ? 1.45 : 0.85;
  slot.lightPipeMat.emissiveIntensity =
    baseIntensity + slot.rewritePulse * 0.85 + breath * 0.15;
  const dialOpacity = isTopOfStack ? 0.95 : 0.84;
  if (slot.dialMat.opacity !== dialOpacity) slot.dialMat.opacity = dialOpacity;
  const plaqueOpacity = isTopOfStack ? 1.0 : 0.92;
  if (slot.plaqueMat.opacity !== plaqueOpacity) slot.plaqueMat.opacity = plaqueOpacity;
  if (slot.lowerPlaqueMesh && !slot.lowerPlaqueMesh.visible) {
    slot.lowerPlaqueMesh.visible = true;
  }
} else if ((slot.popPulse || 0) > 0.02) {
  _scratchColor.copy(linearColor(popHex)).lerp(linearColor(HEX_ENV_CYAN), 0.45);
  slot.lightPipeMat.color.copy(_scratchColor);
  slot.lightPipeMat.emissive.copy(_scratchColor);
  slot.lightPipeMat.emissiveIntensity = 0.12 + slot.popPulse * 1.15;
  slot.dialMat.opacity = 0.45 + slot.popPulse * 0.35;
  slot.plaqueMat.opacity = 0.46 + slot.popPulse * 0.35;
  if (slot.lowerPlaqueMesh && !slot.lowerPlaqueMesh.visible) {
    slot.lowerPlaqueMesh.visible = true;
  }
} else if (slot.lastPipeHex !== HEX_IDLE_SLATE) {
  slot.lastPipeHex = HEX_IDLE_SLATE;
  const c = linearColor(HEX_IDLE_SLATE);
  slot.lightPipeMat.color.copy(c);
  slot.lightPipeMat.emissive.copy(c);
  slot.lightPipeMat.emissiveIntensity = 0.06;
  slot.dialMat.opacity = 0.45;
  slot.plaqueMat.opacity = 0.0;
  if (slot.lowerPlaqueMesh && slot.lowerPlaqueMesh.visible) {
    slot.lowerPlaqueMesh.visible = false;
  }
}
```

The pop branch still writes every frame while `popPulse` decays. That decay is the visual. The idle branch runs its writes once.

Initialize on the slot object (1656): `popPulse: 0`, `poppedKind: null`, `lastPipeHex: -1`, `lastClaspHex: -1`, `_plaqueFrame: undefined`, `_plaqueTop: false`. `popPulse` is currently added on first write (2574), which transitions the hidden class of every slot object on the first pop.

### 7.2 Clasps — bake rotation, patch Y, upload on change

2652–2691 recompute two full TRS matrices per slot per frame. Rotation (`endThetaA`, `endThetaB`) and XZ never change. Only `slot.currentY` changes, and `Matrix4.compose` stores translation Y at `elements[13]` (`Matrix4.js` 1049–1051). Scale stays 1.

After the slot loop, once, bake 24 matrices at the idle Y (`baseY - 0.016 - 0.002`) with the existing scratch compose (the block at 2658–2677 is the right bake; run it at init, not per frame). Then replace the per-frame clasp block:

```javascript
const claspArr = bridgeClaspsInstancedMesh.instanceMatrix.array;
const claspY = slot.currentY - 0.002;
const o0 = (s * 2) * 16;
const o1 = (s * 2 + 1) * 16;
if (claspArr[o0 + 13] !== claspY) {
  claspArr[o0 + 13] = claspY;
  claspArr[o1 + 13] = claspY;
  claspsMatrixDirty = true;
}
const claspHex = isOccupied ? semanticHex : 0x334155;
if (claspHex !== slot.lastClaspHex) {
  slot.lastClaspHex = claspHex;
  _scratchColor.copy(linearColor(claspHex));
  bridgeClaspsInstancedMesh.setColorAt(s * 2, _scratchColor);
  bridgeClaspsInstancedMesh.setColorAt(s * 2 + 1, _scratchColor);
  claspsColorDirty = true;
}
```

After the slot loop, replace the unconditional flags at 2688–2691:

```javascript
if (claspsMatrixDirty) bridgeClaspsInstancedMesh.instanceMatrix.needsUpdate = true;
if (claspsColorDirty && bridgeClaspsInstancedMesh.instanceColor) {
  bridgeClaspsInstancedMesh.instanceColor.needsUpdate = true;
}
```

`instanceMatrix.setUsage(THREE.DynamicDrawUsage)` at 1481 can stay. The win is not marking `needsUpdate` once the rings have snapped to their target (§4.2). Colors are piecewise constant (no clasp pulse), so a settled trace uploads nothing.

Set both dirty flags `false` at the start of section B.

### 7.3 Stator sectors — matrices are a step function, color pulses on one tooth

1784–1795 and 2705–2730 rebuild 10 matrices every frame. `theta` is constant. The only matrix change is `padY` `0.003 → 0.010` and scale Y `1 → 1.35` on the active index (2710–2715). The pulse at 2720 affects **color only**.

At init, next to the existing setup loop (1784), store two matrices per sector instead of writing the idle pose only:

```javascript
const sectorIdleMatrix = new Array(STATOR_SECTOR_COUNT);
const sectorActiveMatrix = new Array(STATOR_SECTOR_COUNT);
const SECTOR_LINEAR = CONTROL_STATOR_SECTORS.map((sec) => linearColor(sec.color));
const IDLE_SECTOR_LINEAR = linearColor(0x1e293b);

for (let i = 0; i < STATOR_SECTOR_COUNT; i += 1) {
  const theta = Math.PI * 0.5 - i * sectorStep;
  _scratchEuler.set(0, theta, 0);
  _scratchQuat.setFromEuler(_scratchEuler);
  _scratchPos.set(0, 0.003, 0);
  _scratchScale.set(1, 1, 1);
  sectorIdleMatrix[i] = new THREE.Matrix4().compose(_scratchPos, _scratchQuat, _scratchScale);
  _scratchPos.set(0, 0.010, 0);
  _scratchScale.set(1, 1.35, 1);
  sectorActiveMatrix[i] = new THREE.Matrix4().compose(_scratchPos, _scratchQuat, _scratchScale);
  statorSectorsInstancedMesh.setMatrixAt(i, sectorIdleMatrix[i]);
  statorSectorsInstancedMesh.setColorAt(i, IDLE_SECTOR_LINEAR);
}
```

`sectorStep` is already in the closure (1757). Replace the per-frame loop at 2705–2730:

```javascript
if (activeSectorIdx !== lastActiveSectorIdx) {
  if (lastActiveSectorIdx >= 0) {
    statorSectorsInstancedMesh.setMatrixAt(lastActiveSectorIdx, sectorIdleMatrix[lastActiveSectorIdx]);
    statorSectorsInstancedMesh.setColorAt(lastActiveSectorIdx, IDLE_SECTOR_LINEAR);
  }
  statorSectorsInstancedMesh.setMatrixAt(activeSectorIdx, sectorActiveMatrix[activeSectorIdx]);
  statorSectorsInstancedMesh.instanceMatrix.needsUpdate = true;
  lastActiveSectorIdx = activeSectorIdx;
}
const pulse = 1.05 + 0.25 * Math.sin(ambient * 6.0);
_scratchColor.copy(SECTOR_LINEAR[activeSectorIdx]).multiplyScalar(pulse);
statorSectorsInstancedMesh.setColorAt(activeSectorIdx, _scratchColor);
if (statorSectorsInstancedMesh.instanceColor) {
  statorSectorsInstancedMesh.instanceColor.needsUpdate = true;
}
```

`let lastActiveSectorIdx = -1` next to `pointerAngle` (2410). The active tooth still breathes every frame, which is the current look. The other nine colors and all ten matrices stay put across frames that do not change sector. `instanceMatrix` can use `StaticDrawUsage`; it is uploaded only on a sector change. Keep `instanceColor` on `DynamicDrawUsage`.

A partial `addUpdateRange` for the one pulsing color is optional and not worth it: the color buffer is 10 × 3 floats, and `addUpdateRange` allocates a `{start, count}` object (`BufferAttribute.js` 181–183) unless the range object is reused. Uploading 120 bytes is cheaper than a per-frame allocation. Skip the range API.

### 7.4 Needle, lens, output wave

Needle tip color and intensity are constant inside each branch (2752–2771). Only `targetReach` and the positions animate. `setHex` twice per frame is wasted.

```javascript
let needleMode = 0;
let needleHex = activeSectorMeta.color;
if (coreTel?.action === 'PUSH' || isPushRule) {
  needleMode = 1; needleHex = HEX_OP_AMBER;
  targetReach = 0.204 + 0.008 * pulseWave;
} else if (coreTel?.action === 'REWRITE' || isInPlaceRewriteRule) {
  needleMode = 2; needleHex = HEX_OP_GOLD;
  targetReach = 0.200 + 0.006 * Math.sin(ambient * 12);
} else if (coreTel?.action === 'POP' || isPopRule || rawControlKind === 'Return') {
  needleMode = 3; needleHex = HEX_ENV_CYAN;
  targetReach = 0.184 - 0.006 * pulseWave;
}
if (needleMode !== lastNeedleMode || needleHex !== lastNeedleHex) {
  lastNeedleMode = needleMode;
  lastNeedleHex = needleHex;
  const c = linearColor(needleHex);
  needleTipMat.color.copy(c);
  needleTipMat.emissive.copy(c);
  needleTipMat.emissiveIntensity =
    needleMode === 1 ? 1.5 : needleMode === 2 ? 1.85 : needleMode === 3 ? 1.35 : 1.1;
}
```

Reach still lerps every frame (2772). Write positions and `updateMatrix()` only when `pointerRadialReach` or `pointerAngle` actually changes, with the same `1e-4` snap as §4.2. `transferNeedleGroup.matrixAutoUpdate = false`.

Sapphire lens (2803–2827): `setHex` on enter of a state, write `emissiveIntensity` every frame only while `isErrorState` or `isDoneState` (those intensities use `Math.sin`). In the idle branch the color is `activeSectorMeta.color` and the intensity is the constant `0.22`. Gate that branch on `activeSectorIdx !== lastLensSector`.

Output wave: `outputWaveMat.color.setHex(HEX_ENV_CYAN)` (2821) is constant. `copy(linearColor(HEX_ENV_CYAN))` once when entering `isDoneState`. Keep the per-frame scale, opacity, and visibility writes while the wave is running. When leaving `Done` or `Error`, set `visible = false` and `opacity = 0` once, not every idle frame.

### 7.5 Fuel ring (lines 1368–1415 and update 2498–2546)

In the read range, and the same bug. Matrices are written at init (1394–1405) and never again. Colors are rewritten for all 48 segments every frame (2523–2542).

- `fuelInstancedMesh.instanceMatrix.setUsage(THREE.StaticDrawUsage)` at 1392. Leave `instanceColor` on `DynamicDrawUsage` (1412).
- `freezeStatic` the group, the track mesh, and the instanced mesh (§4.1).
- While `!isExhausted` and the lerp has snapped (`displayedFuelRatio === targetFuelRatio`) and `leadingEdgeIdx` / `isLowFuel` are unchanged, skip the 48 `setColorAt` calls and do not set `instanceColor.needsUpdate`.
- While draining or exhausted, keep the loop. Exhausted uses `Math.sin(ambient * 14 + s * 0.2)` (2528), so it must upload. Use `linearColor(PALETTE.errorRed | heatBright | amber | gold | 0x1e293b)` plus `_scratchColor.copy(...).multiplyScalar(...)` instead of `setHex` inside the loop.

`displayedFuelRatio` reaches the target only if snapped the same way as `currentY`. After the lerp at 2516:

```javascript
if (Math.abs(displayedFuelRatio - targetFuelRatio) < 1e-4) {
  displayedFuelRatio = targetFuelRatio;
}
```

### 7.6 One real per-frame allocation in the shared prologue

2435:

```javascript
const frames = step?.after?.frames ?? [];
```

When `step` is null (ambient / no trace) this allocates a fresh `[]` every call. `update()` then walks it for the chrono buffer (2488–2496). Hoist next to the scratch objects:

```javascript
const EMPTY_FRAMES = Object.freeze([]);
```

```javascript
const frames = step?.after?.frames ?? EMPTY_FRAMES;
```

The loop only reads. Do not mutate `EMPTY_FRAMES`.

`update(stepState = {})` (2417) allocates only when called with no argument. The self-call at 3042 passes an object. Low priority. Remove the default if a caller ever hits it at 60 Hz.

---

## 8. What not to change

- **Segment counts and shadow flags.** Cited in §2.
- **Instancing A with B on the light pipe, the soffit, or the plaque.** Different emissive, different maps.
- **A dial texture atlas.** `RingGeometry` UVs span the full square. Atlas neighbors bleed into the ticks at 512². Twelve separate textures are the quality-preserving choice. Cache them (§5.3); do not pack them.
- **Quantizing the sector pulse or the pipe breath** to skip uploads. Both are continuous and currently evaluated every frame (2603, 2720). Skipping writes when the hex is unchanged (§7.1, §7.3) does not quantize them.
- **Disposing LRU plaque textures** from this subsystem’s `dispose()`.
- **Section D / E** in this pass, except leaving line 2932 (`lastStepIndex = stepIndex`) intact so §5.2 and any `isStepChanged` gate stay true for exactly one call per step.

---

## 9. Apply order and how to tell it worked

1. §3 geometry share and §3.4 plaque plane. Confirm east slot codes still read `R1B`…`R6B` from above and from inside, and that a rewrite pulse still yaws the whole half-arc including the east metal.
2. §4 `freezeStatic` plus manual `updateMatrix` on the 12 groups, the needle group, and the needle meshes. A frozen child that was frozen before its `rotation.y = -π` assignment will render in the west. Freeze last.
3. §5 plaque `needsUpdate` removal and the lens key. Scrub a trace that repeats `EVAL` on the same rule and confirm the lens does not hitch on every step.
4. §7 dirty checks, linear colors, clasp Y patch, sector matrix cache, fuel static usage.
5. §6 instancing only after 1–4 match a capture. Instancing changes draw-call grouping and is the easiest place to drop a yaw sign.

Visual checks that this pass is not allowed to fail: 12 slot codes on the correct shoulders, east/west bridge gaps still 14.4°, light-pipe hue per frame kind (including pop afterglow toward cyan), sector tooth lift only on the active index, needle reach still inside the bore, underside plaques still hidden from above (`FrontSide`, `rotation.x = π/2`).

Expected cost change, orders of magnitude, not a benchmark: chamfered extrude builds 12 → 6; plaque `PlaneGeometry` 12 → 1; static `compose` calls drop by the child count in §4.1 (about 100 per frame); clasp and sector matrix rebuilds drop to “on change”; plaque shader-version bumps drop from once per step to once per distinct string. Draw calls drop only if §6 is applied (outer 12 → 6, channels 12 → 6, dials 24 → 12, pipes 24 → 12).
