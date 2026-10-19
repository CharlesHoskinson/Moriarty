# Perf Audit 04 — Designer 2 Hollow Well, Part B

**Auditor:** Grok 4.7 xhigh Performance Auditor #4 (39-Rule Floor Modules, Bore Architecture & Update Hot Path)
**File:** `src/designers/designer2_well.js` lines 1600–3049 (`createWellSubsystem` from the lower light-pipe channel through `dispose`)
**Renderer that actually runs:** `THREE.WebGPURenderer` (`src/stage/SherlockStage.js` lines 97–108), installed `three@0.186.1` (`package.json` allows `^0.175.0`). One PCF directional shadow, 2048², `shadowMap.enabled = true`.
**Constraint:** every change below keeps the current pixels. Material models, clearcoat, instance colors, elevations, pulse formulas, and the open bore are unchanged. Items that would move a pixel are listed under Rejected.

No source file was modified by this audit.

---

## 1. Verdict

The floor is already instanced, and the module-scope scratch (`_scratchMat4`, `_scratchPos`, `_scratchQuat`, `_scratchScale`, `_scratchEuler`, `_scratchColor`, `_scratchColorB` at lines 667–673) plus `_chronoFramesBuffer` (line 2403) are the right shape. The hot path then throws that away. `update()` (lines 2417–3028) recomposes and re-uploads buffers that do not change, builds strings to decide that a plaque did not change, and walks about 200 static meshes through `Matrix4.compose` because nothing is frozen.

The comment above `createStatic39RuleFloorOverlayTexture` (lines 1221–1223) says the floor is one `InstancedMesh` plus one overlay. The mesh that was built is eight main-pass draws plus a shadow draw. Collapsing those eight into one draw would merge a clearcoat physical material, a gold standard material, and an unlit basic material. That changes the picture. The draw-call wins that keep the picture are elsewhere in this range: the stator hardware, the needle pair, and the chamber walls.

What costs frames, in order:

| Rank | Where | What happens every frame today | Pixel-identical fix |
| ---: | :--- | :--- | :--- |
| 1 | Floor loop, lines 2944–3027 | 39 cells × 3 `Matrix4.compose` + 6 full `writeBuffer`s, including 37 idle cells whose pose is constant | Pose buckets. Upload a matrix only when idle / trail / active changes. `addUpdateRange` for the one pulsing color |
| 2 | Clasps, lines 2652–2691 | 24 Euler→quat composes and a full 24-matrix upload. Rotation is constant; only Y tracks the arc | Write matrix element 13 when Y changes |
| 3 | Stator teeth, lines 2705–2730 | 10 Euler composes and a full matrix+color upload. Pose is two discrete states; only the active tooth’s color pulses | Cache 10 idle and 10 active matrices at init. Pulse one color |
| 4 | Scene-graph, whole bore and floor | ~200 static meshes `compose` a local matrix every frame, and `Scene` then forces a `matrixWorld` multiply | `freezeWorld` on leaves with no moving ancestor; `freezeLocal` on children of a moving group |
| 5 | Plaque + lens keys, lines 2626–2650 and 2779–2800 | A fresh string per slot per frame (`plaqueKey`) plus `formatSlotEnvCompact` templates, then a compare against last frame’s string | Compare the fields the string was built from. Call the formatter only when those fields change |
| 6 | Chamber construction, lines 2022–2148 | 64 wall/ledge draws + 32 light-loop draws + 4 horizon draws, all untextured boxes | Instance by shared material. Keep south segments as their own objects so cutaway still retracts only the south wall |
| 7 | Transparency writes, lines 2595–2623, 2803–2828, 2859–2903 | `setHex` / `opacity` / `emissiveIntensity` assigned even when the value is the one already on the material | Write on mode change. Keep writing every frame only where a `sin` is actually visible |

`StaticDrawUsage` vs `DynamicDrawUsage` is not a lever on this renderer. `WebGPUBackend.createAttribute` (installed `three/src/renderers/webgpu/WebGPUBackend.js` line 2886) always allocates `GPUBufferUsage.VERTEX | COPY_SRC | COPY_DST`. The usage hint never reaches the device. The upload that matters is `needsUpdate` with an empty `updateRanges` array, which `WebGPUAttributeUtils.updateAttribute` turns into a full `queue.writeBuffer` from byte 0.

---

## 2. Draw calls in this range

### 2.1 Floor (lines 2184–2388) — 8 main draws + 1 shadow draw

| Line | Object | Material | Main draws | Shadow draw |
| ---: | :--- | :--- | ---: | ---: |
| 2185 | `floorBaseMesh` | `MeshStandardMaterial`, `receiveShadow` | 1 | 0 |
| 2210 | `ruleBezelInstancedMesh` × 39 | gold `MeshStandardMaterial`, `receiveShadow` | 1 | 0 |
| 2228 | `ruleTilesInstancedMesh` × 39 | `MeshPhysicalMaterial` clearcoat 0.35, `castShadow` + `receiveShadow` | 1 | 1 |
| 2246 | `ruleBeaconInstancedMesh` × 39 | shared `beaconMat` (`MeshBasicMaterial`) | 1 | 0 |
| 2255 | `ruleBeaconPostInstancedMesh` × 39 | same `beaconMat` | 1 | 0 |
| 2347 | `romBezelMesh` | same `bezelMat`, **no** `receiveShadow` | 1 | 0 |
| 2354 | `romBodyMesh` | its own `MeshPhysicalMaterial` | 1 | 0 |
| 2382 | `floorOverlayMesh` | transparent `MeshBasicMaterial`, `depthWrite: false` | 1 | 0 |

`castShadow` on the tiles is what adds the shadow draw. `ShadowBaseNode` renders any object with `castShadow === true` into the directional map (`three/src/nodes/lighting/ShadowBaseNode.js` lines 90–100). The raised chiplet (scale Y 1.38, center `WELL_FLOOR_Y + 0.054`) does cast a contact shadow under the key light at `(6.0, 10.5, 5.4)`. Removing `castShadow` deletes that shadow. Leave the flag.

Do not merge the ROM bezel into `ruleBezelInstancedMesh`. The bezel mesh has `receiveShadow = true` (line 2215). `romBezelMesh` does not. One instanced material has one shadow-receive flag (`RenderObject.getDynamicCacheKey` hashes `object.receiveShadow`). The ROM rim would start sampling the shadow map.

Do not merge the two beacon meshes into `BatchedMesh`. `BatchedMesh.setMatrixAt` sets `_matricesTexture.needsUpdate` on every call and uploads the whole texture (`BatchedMesh.js` lines 1075–1084). `BatchedMesh._initColorsTexture` (lines 367–376) marks that texture with `ColorManagement.workingColorSpace`. `InstancedMesh.setColorAt` is a linear `InstancedBufferAttribute` multiply. Those are not the same color path. Two instanced draws with partial `writeBuffer`s are the pixel-exact choice. After section 5, both draws are idle most frames; one extra static draw of 39 boxes is cheaper than a full texture upload on every pulse.

The overlay plane stays. It is the one texture that replaced 39 text meshes. Do not shrink the 2048 canvas or the anisotropy 4 set at line 1319. Do not add `alphaTest`; it clips the antialiased label edges.

### 2.2 Bore hardware (lines 1690–1930) — 17 draws, 6 of them the same gold material

These six share `bridgeStrutMat` (lines 1691–1697) and never move:

| Lines | Meshes | Geometry |
| ---: | :--- | :--- |
| 1699–1705 | north tab, south tab | one `BoxGeometry(0.016, 0.008, 0.032)` |
| 1733–1742 | upper + lower outer collet | one torus, `rotation.x = π/2`, Y = `0.002` and `-0.022` |
| 1745–1753 | upper + lower inner collet | one torus, same rotation, Y = `0.002` and `-0.022` |

Replacement, local to `controlCoreGroup` (so the group’s `y = CONTROL_STATOR_Y` still applies). Init only; nothing here runs in `update()`.

```javascript
import { mergeGeometries } from 'three/addons/utils/BufferGeometryUtils.js';

const _bakePos = new THREE.Vector3();
const _bakeQuat = new THREE.Quaternion();
const _bakeScale = new THREE.Vector3(1, 1, 1);
const _bakeEuler = new THREE.Euler();
const _bakeMat = new THREE.Matrix4();

function bakedGeometry(geometry, x, y, z, rotationX) {
  const g = geometry.clone();
  _bakePos.set(x, y, z);
  _bakeEuler.set(rotationX, 0, 0);
  _bakeQuat.setFromEuler(_bakeEuler);
  _bakeMat.compose(_bakePos, _bakeQuat, _bakeScale);
  g.applyMatrix4(_bakeMat);
  return g;
}

const statorHardwareGeo = trackGeo(mergeGeometries([
  bakedGeometry(bridgeTabGeo, 0, -0.004, -0.224, 0),
  bakedGeometry(bridgeTabGeo, 0, -0.004, 0.224, 0),
  bakedGeometry(outerColletGeo, 0, 0.002, 0, Math.PI * 0.5),
  bakedGeometry(outerColletGeo, 0, -0.022, 0, Math.PI * 0.5),
  bakedGeometry(innerColletGeo, 0, 0.002, 0, Math.PI * 0.5),
  bakedGeometry(innerColletGeo, 0, -0.022, 0, Math.PI * 0.5),
]));
const statorHardwareMesh = new THREE.Mesh(statorHardwareGeo, bridgeStrutMat);
controlCoreGroup.add(statorHardwareMesh);
```

Delete the six `controlCoreGroup.add(...)` calls at lines 1701, 1705, 1737, 1742, 1748, and 1753. Six draws become one. Same triangles, same material, same local positions. Dispose `bridgeTabGeo`, `outerColletGeo`, and `innerColletGeo` after the merge if nothing else references them (`trackGeo` already retained the clones inside the merged geometry; do not dispose the merged result).

The needle pairs move, but they move by the same local X. Lines 2772–2776 write the same `pointerRadialReach` into both tips and the same `pointerRadialReach - 0.034` into both stems. The only difference is Y = `0` vs Y = `-0.044`.

```javascript
const needleStemPairGeo = trackGeo(mergeGeometries([
  bakedGeometry(needleStemGeo, 0, 0, 0, 0),
  bakedGeometry(needleStemGeo, 0, -0.044, 0, 0),
]));
const needleStemMesh = new THREE.Mesh(needleStemPairGeo, needleArmMat);
needleStemMesh.position.set(0.162, 0, 0);
transferNeedleGroup.add(needleStemMesh);

const needleTipPairGeo = trackGeo(mergeGeometries([
  bakedGeometry(needleTipGeo, 0, 0, 0, 0),
  bakedGeometry(needleTipGeo, 0, -0.044, 0, 0),
]));
const needleTipMesh = new THREE.Mesh(needleTipPairGeo, needleTipMat);
needleTipMesh.position.set(0.196, 0, 0);
transferNeedleGroup.add(needleTipMesh);
```

Drop `lowerNeedleStemMesh` and `lowerNeedleTipMesh` (lines 1858–1860 and 1868–1870). In `update()`, replace lines 2773–2776 with:

```javascript
needleTipMesh.position.x = pointerRadialReach;
needleStemMesh.position.x = pointerRadialReach - 0.034;
```

Both tips already share `needleTipMat`, so the `setHex` at lines 2754–2770 still paints both. Two draws become one for the stems, two become one for the tips.

Upper and lower stator dials (lines 1815–1824) share `statorDialGeo` and `statorDialMat`. Bake both transforms (including `lowerStatorDialMesh.scale` of `(1, -1, 1)`) into one geometry, keep `side: THREE.DoubleSide` and `depthWrite: false`. Do not switch the merged mesh to `FrontSide`: the negative Y scale flips winding, and `FrontSide` would cull the soffit copy. One transparent sort entry instead of two. Same fragments from above and from inside the bore.

### 2.3 Half-arc soffit added in this range (lines 1600–1641)

Per slot, `lightPipeMat` is already on `lightPipeMesh` (line 1592) and `lowerLightPipeMesh` (line 1596). Line 1611 adds a third mesh, `lowerChannelMesh`, with that same material. Three draws, one material, transforms that never change relative to `halfArcGroup`.

After line 1614:

```javascript
const _pipeBake = new THREE.Matrix4();
const _pipePos = new THREE.Vector3();

function meshBake(geometry, y) {
  const g = geometry.clone();
  _pipePos.set(0, y, 0);
  _pipeBake.makeTranslation(_pipePos.x, _pipePos.y, _pipePos.z);
  g.applyMatrix4(_pipeBake);
  return g;
}

const lightPipeMergedGeo = trackGeo(mergeGeometries([
  meshBake(lightPipeGeo, 0.006),
  meshBake(lightPipeGeo, -0.026),
  meshBake(lowerChannelGeo, -0.0294),
]));
const lightPipeMesh = new THREE.Mesh(lightPipeMergedGeo, lightPipeMat);
halfArcGroup.add(lightPipeMesh);
```

Do not also `add` the three source meshes. `lightPipeMat.emissive` in `update()` (lines 2599–2620) still drives the whole soffit, which is what the three meshes do today because they share one material. Twelve slots × two removed draws = 24 draws removed. `lowerPlaqueMesh` stays its own mesh: it has its own map and it toggles `visible`.

### 2.4 Chamber (lines 1975–2148) — 104 draws down to 29

`createSteppedTerrace` (lines 1129–1195, called at 2027) builds 4 walls + 4 ledges per terrace. `createRecessedLightLoop` (lines 1201–1217, called at 2049 and 2147) builds 4 boxes per call. None of these materials have maps, so a unit box plus an instance scale matches the UVs and, after the normal matrix, the shading. `BoxGeometry` of any size still uses a 0–1 UV per face.

Counts today, for `TERRACE_COUNT = 8`:

| Builder | Draws |
| :--- | ---: |
| Terrace walls | 32 |
| Terrace ledges | 32 |
| Depth light loops | 32 |
| Horizon loop | 4 |
| Vault ceiling (two arcs, different geometry, south one moves) | 2 |
| Vault ribs (already instanced, lines 2098 and 2115) | 2 |
| **Total** | **104** |

South segments are the constraint. `update()` lines 2837–2857 and 2880–2883 move only the south wall group, the south light-loop segment, the south horizon segment, and `vaultCeilingSouthMesh`. A single instanced mesh whose object transform moves would drag the north wall with it. Split south out so the object matrix carries `southRetractY` and the local bounding sphere stays valid. `Frustum.intersectsObject` (`three/src/math/Frustum.js` lines 146–164) transforms `InstancedMesh.boundingSphere` by `matrixWorld`. If the retract lives in the object matrix, the sphere follows. If the retract is baked into one instance of a shared mesh, that instance leaves the sphere computed at rest and the south wall pops out of frame during cutaway.

Wall materials alternate (`matteAnodizedBlackMat` / `matteAnodizedSlateMat`, lines 1953–1965). Roughness is 0.68 vs 0.62 and metalness is 0.38 vs 0.42. Instance color cannot express that. Two materials, not one.

Ledges all share `chamferTrimMat`. Light-loop materials are one per terrace on purpose: `ring.material.emissive` is animated per depth (lines 2859–2877).

Replacement for the loop at lines 2022–2073 and the horizon block at lines 2132–2148. `unitBox` is `new THREE.BoxGeometry(1, 1, 1)`.

```javascript
function freezeWorld(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
  obj.matrixWorldAutoUpdate = false;
  obj.updateWorldMatrix(true, false);
}

function addBoxInstances(mesh, records) {
  for (let i = 0; i < records.length; i += 1) {
    const r = records[i];
    _scratchPos.set(r.x, r.y, r.z);
    _scratchQuat.identity();
    _scratchScale.set(r.sx, r.sy, r.sz);
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    mesh.setMatrixAt(i, _scratchMat4);
  }
  mesh.instanceMatrix.needsUpdate = true;
  mesh.computeBoundingSphere();
  mesh.receiveShadow = true;
  freezeWorld(mesh);
}

const evenNorth = [];
const oddNorth = [];
const evenSouth = [];
const oddSouth = [];
const evenWest = [];
const oddWest = [];
const evenEast = [];
const oddEast = [];
const ledgeNorth = [];
const ledgeSouth = [];
const ledgeWest = [];
const ledgeEast = [];
const southLightMeshes = [];
const southLedgeMeshHolder = { mesh: null };
const southWallMeshHolder = { even: null, odd: null };

for (let d = 0; d < TERRACE_COUNT; d += 1) {
  const ringY = getTerraceY(d);
  const innerHalf = getTerraceInnerHalf(d);
  const thickness = OUTER_HALF - innerHalf;
  const wallMid = (OUTER_HALF + innerHalf) * 0.5;
  const fullSpan = OUTER_HALF * 2;
  const innerSpan = innerHalf * 2;
  const bucket = d % 2 === 0;
  const northList = bucket ? evenNorth : oddNorth;
  const southList = bucket ? evenSouth : oddSouth;
  const westList = bucket ? evenWest : oddWest;
  const eastList = bucket ? evenEast : oddEast;

  northList.push({ x: 0, y: ringY, z: -wallMid, sx: fullSpan, sy: terraceHeight, sz: thickness });
  southList.push({ x: 0, y: ringY, z: wallMid, sx: fullSpan, sy: terraceHeight, sz: thickness });
  westList.push({ x: -wallMid, y: ringY, z: 0, sx: thickness, sy: terraceHeight, sz: innerSpan });
  eastList.push({ x: wallMid, y: ringY, z: 0, sx: thickness, sy: terraceHeight, sz: innerSpan });

  const ledgeThick = 0.008;
  const ledgeH = 0.004;
  const ledgeMid = innerHalf + ledgeThick * 0.5;
  const ledgeY = ringY + terraceHeight * 0.5;
  const ledgeSpanX = innerHalf * 2 + ledgeThick * 2;
  ledgeNorth.push({ x: 0, y: ledgeY, z: -ledgeMid, sx: ledgeSpanX, sy: ledgeH, sz: ledgeThick });
  ledgeSouth.push({ x: 0, y: ledgeY, z: ledgeMid, sx: ledgeSpanX, sy: ledgeH, sz: ledgeThick });
  ledgeWest.push({ x: -ledgeMid, y: ledgeY, z: 0, sx: ledgeThick, sy: ledgeH, sz: innerHalf * 2 });
  ledgeEast.push({ x: ledgeMid, y: ledgeY, z: 0, sx: ledgeThick, sy: ledgeH, sz: innerHalf * 2 });

  const ringMat = trackMat(new THREE.MeshStandardMaterial({
    color: 0x1e293b,
    emissive: PALETTE.gold,
    emissiveIntensity: 0.03,
    roughness: 0.32,
    metalness: 0.65,
  }));
  const ringHalf = innerHalf - 0.003;
  const loopY = ringY + terraceHeight * 0.44;
  const loopGroup = new THREE.Group();
  loopGroup.position.y = loopY;

  const nwe = new THREE.InstancedMesh(trackGeo(unitBox), ringMat, 3);
  const nweRecords = [
    { x: 0, y: 0, z: -ringHalf, sx: ringHalf * 2, sy: 0.006, sz: 0.007 },
    { x: -ringHalf, y: 0, z: 0, sx: 0.007, sy: 0.006, sz: ringHalf * 2 - 0.014 },
    { x: ringHalf, y: 0, z: 0, sx: 0.007, sy: 0.006, sz: ringHalf * 2 - 0.014 },
  ];
  addBoxInstances(nwe, nweRecords);
  const southLoop = new THREE.Mesh(
    trackGeo(new THREE.BoxGeometry(ringHalf * 2, 0.006, 0.007)),
    ringMat
  );
  southLoop.position.set(0, 0, ringHalf);
  loopGroup.add(nwe, southLoop);
  wellStructureGroup.add(loopGroup);
  southLightMeshes.push(southLoop);

  depthRings.push({
    index: d,
    code: `D${d}`,
    y: ringY,
    innerHalf,
    material: ringMat,
    southLoop,
  });
}

function makeWallMesh(list, material) {
  const mesh = new THREE.InstancedMesh(trackGeo(unitBox), material, list.length);
  addBoxInstances(mesh, list);
  wellStructureGroup.add(mesh);
  return mesh;
}

makeWallMesh(evenNorth, matteAnodizedBlackMat);
makeWallMesh(oddNorth, matteAnodizedSlateMat);
southWallMeshHolder.even = makeWallMesh(evenSouth, matteAnodizedBlackMat);
southWallMeshHolder.odd = makeWallMesh(oddSouth, matteAnodizedSlateMat);
makeWallMesh(evenWest, matteAnodizedBlackMat);
makeWallMesh(oddWest, matteAnodizedSlateMat);
makeWallMesh(evenEast, matteAnodizedBlackMat);
makeWallMesh(oddEast, matteAnodizedSlateMat);

const ledgeMat = chamferTrimMat;
makeWallMesh(ledgeNorth, ledgeMat);
southLedgeMeshHolder.mesh = makeWallMesh(ledgeSouth, ledgeMat);
makeWallMesh(ledgeWest, ledgeMat);
makeWallMesh(ledgeEast, ledgeMat);

const horizonGroup = new THREE.Group();
horizonGroup.position.y = getTerraceY(7);
const horizonNwe = new THREE.InstancedMesh(trackGeo(unitBox), horizonRingMat, 3);
addBoxInstances(horizonNwe, [
  { x: 0, y: 0, z: -1.5, sx: 3.0, sy: 0.010, sz: 0.012 },
  { x: -1.5, y: 0, z: 0, sx: 0.012, sy: 0.010, sz: 3.0 - 0.024 },
  { x: 1.5, y: 0, z: 0, sx: 0.012, sy: 0.010, sz: 3.0 - 0.024 },
]);
const horizonSouthMesh = new THREE.Mesh(
  trackGeo(new THREE.BoxGeometry(3.0, 0.010, 0.012)),
  horizonRingMat
);
horizonSouthMesh.position.set(0, 0, 1.5);
horizonGroup.add(horizonNwe, horizonSouthMesh);
group.add(horizonGroup);
```

`addBoxInstances` calls `freezeWorld`, which is correct for north/east/west and for the ledge meshes that do not move. Do **not** call `freezeWorld` on `southWallMeshHolder.even`, `southWallMeshHolder.odd`, or `southLedgeMeshHolder.mesh`. Those three get `position.y = southRetractY` in `update()`. Leave their `matrixAutoUpdate` and `matrixWorldAutoUpdate` at the default, or section 4’s cutaway gate will call `updateMatrix()` itself.

`makeWallMesh` as written freezes the south meshes too. Split it:

```javascript
function makeWallMesh(list, material, moving) {
  const mesh = new THREE.InstancedMesh(trackGeo(unitBox), material, list.length);
  addBoxInstances(mesh, list);
  if (moving) {
    mesh.matrixAutoUpdate = true;
    mesh.matrixWorldAutoUpdate = true;
  }
  wellStructureGroup.add(mesh);
  return mesh;
}
```

Pass `moving = true` for the two south wall meshes and the south ledge mesh.

Draws after the split:

| Group | Draws |
| :--- | ---: |
| Walls N/E/W × 2 materials, plus south × 2 materials | 8 |
| Ledges N/E/W/S | 4 |
| Light loops: one 3-instance mesh + one south mesh, per terrace | 16 |
| Horizon: one 3-instance mesh + one south mesh | 2 |
| Vault ceiling (leave lines 2009–2017; geometries differ and the south arc moves) | 2 |
| Vault ribs (already 2) | 2 |
| **Total** | **34** |

16 light-loop draws is the bound-safe number. Merging each terrace’s south segment back into the 3-instance mesh would cut 8 draws and would cull that segment once `southRetractY` reaches `-3.15`, because the sphere was computed at Y = 0. Do not do that.

`tagInspectable` today hangs off `lightRingLoop` (lines 2054–2063). Hang the same payload off `loopGroup` so inspection still resolves a depth ring.

Vault ribs (lines 2098–2130) are already one matrix fill at init. They are missing the freeze and the sphere:

```javascript
vaultRibsInstancedMesh.computeBoundingSphere();
freezeWorld(vaultRibsInstancedMesh);
southVaultRibsInstancedMesh.computeBoundingSphere();
```

Leave `southVaultRibsInstancedMesh` on the default auto-update. Line 2839 sets its `position.y`.

---

## 3. Static matrix freeze

`Object3D.updateMatrix` (installed `three/src/core/Object3D.js` lines 1144–1162) always `compose`s and always sets `matrixWorldNeedsUpdate`. `updateMatrixWorld` then sets `force = true` for every descendant. `Scene.matrixAutoUpdate` is true, so the scene dirties the whole graph every frame even when the scene did not move.

`matrixAutoUpdate = false` by itself only skips `compose`. The scene’s `force` still multiplies `matrixWorld`. `matrixWorldAutoUpdate = false` skips that multiply too (`Object3D.js` lines 1180–1198). A frozen leaf whose parent later moves keeps a stale `matrixWorld`. Freeze world only when no ancestor in this subsystem ever changes transform. The well group and `this.root` are both left at identity (`SherlockStage.js` lines 110 and 218–220).

```javascript
function freezeLocal(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
}

function freezeWorld(obj) {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
  obj.matrixWorldAutoUpdate = false;
  obj.updateWorldMatrix(true, false);
}
```

Call `freezeWorld` once, after the object is parented and its local TRS is final.

| Object | Lines | Freeze | Why |
| :--- | ---: | :--- | :--- |
| `controlCoreGroup` and every static child except the needle group and `outputWaveMesh` | 1685–1930 | `freezeWorld` | Group Y is set once to `CONTROL_STATOR_Y` |
| `statorSectorsInstancedMesh` object (not its instance buffer) | 1777 | `freezeWorld` | Tooth motion is in `instanceMatrix` |
| `statorHousingMesh`, both dials, `sapphireLensMesh`, `lowerCorePlaqueMesh` | 1728–1911 | `freezeWorld` | Never written in `update()` |
| `transferNeedleGroup` | 1828 | neither | `rotation.y` every settled pointer frame, lines 2747 |
| `needleStemMesh`, `needleTipMesh` | 1854–1866 | `freezeLocal` | Local X changes; world must follow the group |
| `outputWaveMesh` | 1923 | neither | Scale animates while `Done` |
| `vaultCeilingMesh` | 2009 | `freezeWorld` | Static |
| `vaultCeilingSouthMesh` | 2014 | neither | `position.y` tracks cutaway, line 2837 |
| `wellStructureGroup`, `floorGroup` | 1949, 2168 | `freezeWorld` | Identity for the life of the subsystem |
| `floorBaseMesh`, `ruleBezelInstancedMesh`, both ROM meshes, `floorOverlayMesh` | 2185–2388 | `freezeWorld` | Bezel instance buffer is filled once at 2278 and never touched again |
| `ruleTilesInstancedMesh`, both beacon meshes | 2228–2260 | `freezeWorld` on the object | Motion is the instance buffer, not the object |
| `halfArcGroup` | 1504, slot pushed at 1656 | neither | `position.y` and `rotation.y` in `update()` |
| Children of `halfArcGroup` (channel, pipes, dials, plaque) | 1514–1641 | `freezeLocal` | Local TRS is constant; the group is the thing that lifts |
| `stackLimitGroup` | 2133 | neither | Y lerps, line 2888 |

`halfArcGroup` children are created before and inside this range. Freeze them at the bottom of the slot loop, after line 1641 and before `halfArcSlots.push`:

```javascript
halfArcGroup.traverse((obj) => {
  if (obj !== halfArcGroup) freezeLocal(obj);
});
```

`lowerPlaqueMesh.visible` does not need a live local matrix. `freezeLocal` leaves `matrixWorldAutoUpdate` true, so when the arc group moves, the scene’s force (and the group’s own dirty flag) still rebuilds child world matrices from the baked local matrix.

In `update()`, a frozen moving object must opt back in on the frames it actually moves:

```javascript
if (slot.group.position.y !== slot.currentY || slot.group.rotation.y !== slot.currentRotY) {
  slot.group.position.y = slot.currentY;
  slot.group.rotation.y = slot.currentRotY;
  slot.group.updateMatrix();
}
```

`updateMatrix` sets `matrixWorldNeedsUpdate`. Children with `freezeLocal` still have `matrixWorldAutoUpdate === true`, so they follow.

Same pattern for the needle, only when the number changed:

```javascript
if (needleTipMesh.position.x !== pointerRadialReach) {
  needleTipMesh.position.x = pointerRadialReach;
  needleTipMesh.updateMatrix();
}
if (needleStemMesh.position.x !== pointerRadialReach - 0.034) {
  needleStemMesh.position.x = pointerRadialReach - 0.034;
  needleStemMesh.updateMatrix();
}
```

`NodeMaterialObserver.equals` (`three/src/materials/nodes/manager/NodeMaterialObserver.js` lines 490–497) returns false when `matrixWorld` differs, which refreshes that render object. A frozen static leaf keeps a bitwise-identical `matrixWorld`, so the observer takes the hit path and does not rebind. That is the GPU-side half of the freeze. The CPU half is skipping `compose`.

Do not `freezeWorld` the well `group` created at line 1332 if any code outside this function starts writing `group.position`. Nothing does today. Freezing it is safe and is what stops the well root from re-dirtied its own matrix. It does not stop the `Scene` from passing `force`. Leaf `matrixWorldAutoUpdate = false` is what stops the multiply.

---

## 4. Shadow and transparency state

### 4.1 What is already stable

`receiveShadow` is a pipeline bit, not a per-frame toggle (`RenderObject.js` lines 856 and 945–948). The floor base, bezels, tiles, vault ceiling, stator housing, and terrace walls set it once. Leaving those flags alone keeps the contact shadows and avoids a pipeline rebuild.

`castShadow = true` on `ruleTilesInstancedMesh` (line 2234) is one extra draw of 39 instances into the 2048 map, every frame the directional light re-renders its shadow (the default). That draw stays. What this file can stop is feeding it a new instance buffer every frame. Section 5 does that. The shadow pass then reads the buffer left by the last pose change, which is the pose on screen.

`outerArcMesh.castShadow` (line 1522) is outside this line window. Do not clear it from here; those arcs are the large casters the key light is framed for (`SherlockStage.js` lines 149–161).

### 4.2 Transparent objects introduced or retinted in this range

| Lines | Material | Flags that define the pipeline |
| ---: | :--- | :--- |
| 1618–1625 | per-slot `plaqueMat` | transparent, `depthWrite: false`, `FrontSide` |
| 1805–1812 | `statorDialMat` | transparent, `depthWrite: false`, `DoubleSide` |
| 1873–1885 | `sapphireLensMat` | transparent, opacity 0.88, `DoubleSide`, **`depthWrite` left at the default `true`**, clearcoat 1 |
| 1897–1903 | `lensPlaqueMat` | transparent, `depthWrite: false`, `FrontSide` |
| 1914–1921 | `outputWaveMat` | transparent, `depthWrite: false`, `DoubleSide` |
| 2374–2380 | `floorOverlayMat` | transparent, `depthWrite: false` |

`RenderList` puts `material.transparent === true` on the sorted transparent list (`RenderList.js` lines 324–330) and sorts that list every frame. Each distinct `(shader, depthWrite, side)` pair is a pipeline switch inside that pass. The sapphire annulus is the only transparent bore surface that writes depth, so it is its own depth-stencil state sitting between the `depthWrite: false` dials, plaques, wave, and floor overlay.

Do not set `sapphireLensMat.depthWrite = false` to “match the others.” With the current flag, the annulus blends over the opaque stator (opaques run first) and then writes depth. Flipping it changes which later transparent fragments survive when the painter sort and the geometric order disagree. That is a pixel change. Do not toggle `transparent` when plaque opacity hits 1.0 either (line 2605). `transparent` is a shader-compile bit; flipping it on the top-of-stack transition rebuilds the pipeline for one plaque.

The churn this file does cause is writing those uniforms when they did not change.

`update()` lines 2595–2623 assign `color`, `emissive`, `emissiveIntensity`, and both opacities on every slot, every frame. Occupied intensity really does change (`breath` and `rewritePulse`). Occupied **color** does not, except when `semanticHex` changes. Pop color does not change during the pulse: the lerp toward `HEX_ENV_CYAN` is a constant `0.45` (line 2609), and only `emissiveIntensity` and the opacities track `popPulse`. Idle values are constants (`HEX_IDLE_SLATE`, intensity `0.06`, dial opacity `0.45`, plaque opacity `0`).

Replace the body of the color block (lines 2595–2623) with a mode latch. `0` idle, `1` occupied, `2` pop.

```javascript
const showPlaque = isOccupied || (slot.popPulse || 0) > 0.02;
if (slot.lowerPlaqueMesh.visible !== showPlaque) {
  slot.lowerPlaqueMesh.visible = showPlaque;
}

if (isOccupied) {
  if (slot.pipeMode !== 1 || slot.pipeHex !== semanticHex || slot.pipeTop !== (isTopOfStack ? 1 : 0)) {
    slot.lightPipeMat.color.setHex(semanticHex);
    slot.lightPipeMat.emissive.setHex(semanticHex);
    slot.dialMat.opacity = isTopOfStack ? 0.95 : 0.84;
    slot.plaqueMat.opacity = isTopOfStack ? 1.0 : 0.92;
    slot.pipeMode = 1;
    slot.pipeHex = semanticHex;
    slot.pipeTop = isTopOfStack ? 1 : 0;
  }
  const breath = 0.5 + 0.5 * Math.sin(ambient * 4.2 + s * 0.6);
  const baseIntensity = isTopOfStack ? 1.45 : 0.85;
  slot.lightPipeMat.emissiveIntensity =
    baseIntensity + slot.rewritePulse * 0.85 + breath * 0.15;
} else if ((slot.popPulse || 0) > 0.02) {
  if (slot.pipeMode !== 2 || slot.pipeHex !== (slot.poppedKind || 'Return')) {
    const popHex = getFrameSemanticHex({ kind: slot.poppedKind || 'Return' });
    _scratchColor.setHex(popHex).lerp(_scratchColorB.setHex(HEX_ENV_CYAN), 0.45);
    slot.lightPipeMat.color.copy(_scratchColor);
    slot.lightPipeMat.emissive.copy(_scratchColor);
    slot.pipeMode = 2;
    slot.pipeHex = slot.poppedKind || 'Return';
  }
  slot.lightPipeMat.emissiveIntensity = 0.12 + slot.popPulse * 1.15;
  slot.dialMat.opacity = 0.45 + slot.popPulse * 0.35;
  slot.plaqueMat.opacity = 0.46 + slot.popPulse * 0.35;
} else if (slot.pipeMode !== 0) {
  slot.lightPipeMat.color.setHex(HEX_IDLE_SLATE);
  slot.lightPipeMat.emissive.setHex(HEX_IDLE_SLATE);
  slot.lightPipeMat.emissiveIntensity = 0.06;
  slot.dialMat.opacity = 0.45;
  slot.plaqueMat.opacity = 0.0;
  slot.pipeMode = 0;
  slot.pipeHex = 0;
}
```

`NodeMaterialObserver` clones a `Color` when `equals` fails (lines 547–551). Skipping `setHex` on the idle and steady occupied slots keeps `materialData._version` from bumping and keeps the uniform buffer quiet. The breath write still bumps intensity, which is the animation on screen.

Initialize `pipeMode: -1` on the slot record at line 1656 so the first frame takes the latch.

### 4.3 Output wave and the lens (lines 2803–2828)

The `else` branch (lines 2823–2827) and the error branch’s `outputWaveMesh.visible = false` run even when the wave is already hidden and the lens emissive is already the sector color. Gate them:

```javascript
let lensMode = -1;
let lensHex = -1;

// inside update(), replace lines 2803–2828
if (isErrorState) {
  const faultPulse = 0.5 + 0.5 * Math.sin(ambient * 12);
  if (lensMode !== 0) {
    sapphireLensMat.emissive.setHex(HEX_CRIMSON);
    lensMode = 0;
    if (outputWaveMesh.visible) outputWaveMesh.visible = false;
    if (outputWaveMat.opacity !== 0) outputWaveMat.opacity = 0;
  }
  sapphireLensMat.emissiveIntensity = 0.45 + faultPulse * 0.4;
} else if (isDoneState) {
  const doneBreath = 0.5 + 0.5 * Math.sin(ambient * 4.0);
  if (lensMode !== 1) {
    sapphireLensMat.emissive.setHex(HEX_ENV_CYAN);
    outputWaveMat.color.setHex(HEX_ENV_CYAN);
    lensMode = 1;
  }
  sapphireLensMat.emissiveIntensity = 0.42 + doneBreath * 0.35;
  const waveT = stepFraction > 0.001 && stepFraction < 0.999
    ? stepFraction
    : (ambient * 0.75) % 1.0;
  const waveScale = lerp(1.0, 4.2, waveT);
  if (!outputWaveMesh.visible) outputWaveMesh.visible = true;
  outputWaveMesh.scale.set(waveScale, waveScale, 1);
  outputWaveMat.opacity = (1.0 - waveT) * 0.65;
} else if (lensMode !== 2 || lensHex !== activeSectorMeta.color) {
  sapphireLensMat.emissive.setHex(activeSectorMeta.color);
  sapphireLensMat.emissiveIntensity = 0.22;
  lensMode = 2;
  lensHex = activeSectorMeta.color;
  if (outputWaveMesh.visible) outputWaveMesh.visible = false;
  if (outputWaveMat.opacity !== 0) outputWaveMat.opacity = 0;
}
```

The `Done` scale and the error/done sines stay per-frame. They are the animation.

### 4.4 Cutaway writes (lines 2833–2857 and 2880–2883)

`cutawayMix` is constant on every frame the cross-section is not moving. The code still writes `position.y` on the south ceiling, the south ribs, both south walls of every terrace, every south light segment, and the south horizon segment.

```javascript
let appliedCutaway = Number.NaN;

// replace the writes at 2834–2857 and 2880–2883
if (Math.abs(cutawayMix - appliedCutaway) > 1e-6) {
  appliedCutaway = cutawayMix;
  const southRetractY = -cutawayMix * 3.15;
  const showSouthWall = cutawayMix < 0.85;

  vaultCeilingSouthMesh.position.y = CHAMBER_TOP_Y + 0.01 + southRetractY;
  vaultCeilingSouthMesh.visible = showSouthWall;
  vaultCeilingSouthMesh.updateMatrix();

  southVaultRibsInstancedMesh.position.y = southRetractY;
  southVaultRibsInstancedMesh.visible = showSouthWall;
  southVaultRibsInstancedMesh.updateMatrix();

  southWallMeshHolder.even.position.y = southRetractY;
  southWallMeshHolder.odd.position.y = southRetractY;
  southWallMeshHolder.even.visible = showSouthWall;
  southWallMeshHolder.odd.visible = showSouthWall;
  southWallMeshHolder.even.updateMatrix();
  southWallMeshHolder.odd.updateMatrix();
  southLedgeMeshHolder.mesh.position.y = southRetractY;
  southLedgeMeshHolder.mesh.visible = showSouthWall;
  southLedgeMeshHolder.mesh.updateMatrix();

  for (let d = 0; d < southLightMeshes.length; d += 1) {
    southLightMeshes[d].position.y = southRetractY;
    southLightMeshes[d].visible = showSouthWall;
    southLightMeshes[d].updateMatrix();
  }
  horizonSouthMesh.position.y = southRetractY;
  horizonSouthMesh.visible = showSouthWall;
  horizonSouthMesh.updateMatrix();
}
```

While the cross-section animates, `cutawayMix` changes every frame and this block still runs. When it stops, the writes stop. `1e-6` is far below a shadow-map texel at this frustum (`shadow.camera` spans about 6.2 units across 2048 texels).

---

## 5. Step-index dirty checking in `update()`

`isStepChanged` is computed at line 2425 and then used for rewrite/pop detection and the floor title. It is not used to skip the work that is a pure function of the step. `lastStepIndex = stepIndex` must stay where it is (line 2932), inside the step-change block, so the flag remains true for the whole function. Do not return early from `update()`. Fuel lerp, pulses, pointer angle, output-wave scale, and the breath sines are not functions of the step index alone.

### 5.1 Floor poses (lines 2905–3027)

This is the largest per-frame cost in the function. For each of 39 cells the code always:

- decays `trail`
- `setHex` + `lerp` or `multiplyScalar`
- `Matrix4.compose` three times
- `setMatrixAt` / `setColorAt` three times

then sets `needsUpdate` on six attributes (lines 3016–3027), which uploads:

| Buffer | Bytes every frame |
| :--- | ---: |
| tile matrix | 39 × 64 = 2496 |
| tile color | 39 × 12 = 468 |
| beacon matrix | 2496 |
| beacon color | 468 |
| post matrix | 2496 |
| post color | 468 |
| **Total** | **8892** |

The pose is a step function of `trail > 0.04` and `isActive`. Inside a pose the matrix does not change.

| Pose | Tile Y offset | Tile scale | Beacon Y offset | Beacon scale Y | Post Y offset | Post scale |
| ---: | ---: | :--- | ---: | ---: | ---: | :--- |
| active | +0.054 | (1.03, 1.38, 1.03) | +0.084 | 1.55 | +0.088 | (1.35, 1.45, 1.35) |
| trail (`trail > 0.04`) | +0.036 | (1, 1.08, 1) | +0.058 | 1 | +0.056 | (1, 0.72, 1) |
| idle | +0.030 | (1, 1, 1) | +0.050 | 1 | +0.044 | (1, 0.35, 1) |

Those are the literals at lines 2957–3011. Active and trail **colors** change every frame (the `sin` at 2958, the trail lerp at 2963–2965 and 2992). Idle color is `lerp(0x1b2536, rule.color, 0.16)` and idle beacon color is `rule.color * 0.32`, both already written at init (lines 2285–2304).

Once `trail <= 0.04` and the cell is not the previous cell, further decay does not change a pixel. The next time that cell becomes `prev`, line 2952 floors trail at `0.36`, so the leftover below `0.04` is discarded. Zeroing it is the same animation.

Add `pose: 0` to the cell record at line 2306 (`0` matches the init matrices). Then replace lines 2942–3027:

```javascript
_scratchQuat.identity();

let tileMatrixDirty = false;
let tileColorDirty = false;
let beaconMatrixDirty = false;
let beaconColorDirty = false;
let postMatrixDirty = false;
let postColorDirty = false;

const tileMatrix = ruleTilesInstancedMesh.instanceMatrix;
const tileColor = ruleTilesInstancedMesh.instanceColor;
const beaconMatrix = ruleBeaconInstancedMesh.instanceMatrix;
const beaconColor = ruleBeaconInstancedMesh.instanceColor;
const postMatrix = ruleBeaconPostInstancedMesh.instanceMatrix;
const postColor = ruleBeaconPostInstancedMesh.instanceColor;

for (let i = 0; i < ruleCells.length; i += 1) {
  const cell = ruleCells[i];
  const isActive = cell === activeCell;
  const isPrev = cell === effectivePrevCell && !isActive;

  let nextTrail = cell.trail;
  if (isActive) nextTrail = 1.0;
  else if (isPrev) nextTrail = Math.max(cell.trail * (1 - dt * 2.2), 0.36);
  else if (cell.trail > 0.04) nextTrail = Math.max(0, cell.trail - dt * 3.5);
  else if (cell.pose === 0) continue;

  if (!isActive && !isPrev && nextTrail <= 0.04) nextTrail = 0;
  cell.trail = nextTrail;

  const pose = isActive ? 2 : nextTrail > 0.04 ? 1 : 0;
  const poseChanged = pose !== cell.pose;
  cell.pose = pose;

  if (poseChanged) {
    if (pose === 2) {
      _scratchPos.set(cell.x, WELL_FLOOR_Y + 0.054, cell.z);
      _scratchScale.set(1.03, 1.38, 1.03);
    } else if (pose === 1) {
      _scratchPos.set(cell.x, WELL_FLOOR_Y + 0.036, cell.z);
      _scratchScale.set(1, 1.08, 1);
    } else {
      _scratchPos.set(cell.x, WELL_FLOOR_Y + 0.030, cell.z);
      _scratchScale.set(1, 1, 1);
    }
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    ruleTilesInstancedMesh.setMatrixAt(i, _scratchMat4);
    tileMatrix.addUpdateRange(i * 16, 16);
    tileMatrixDirty = true;

    const beaconY = pose === 2
      ? WELL_FLOOR_Y + 0.084
      : pose === 1
        ? WELL_FLOOR_Y + 0.058
        : WELL_FLOOR_Y + 0.050;
    _scratchPos.set(cell.x, beaconY, cell.z + gridConfig.cellD * 0.46);
    _scratchScale.set(1, pose === 2 ? 1.55 : 1, 1);
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    ruleBeaconInstancedMesh.setMatrixAt(i, _scratchMat4);
    beaconMatrix.addUpdateRange(i * 16, 16);
    beaconMatrixDirty = true;

    const postScaleY = pose === 2 ? 1.45 : pose === 1 ? 0.72 : 0.35;
    const postY = pose === 2
      ? WELL_FLOOR_Y + 0.088
      : pose === 1
        ? WELL_FLOOR_Y + 0.056
        : WELL_FLOOR_Y + 0.044;
    const postXZ = pose === 2 ? 1.35 : 1;
    _scratchPos.set(cell.x + gridConfig.cellW * 0.42, postY, cell.z + gridConfig.cellD * 0.40);
    _scratchScale.set(postXZ, postScaleY, postXZ);
    _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
    ruleBeaconPostInstancedMesh.setMatrixAt(i, _scratchMat4);
    postMatrix.addUpdateRange(i * 16, 16);
    postMatrixDirty = true;
  }

  if (pose === 2 || pose === 1 || poseChanged) {
    if (pose === 2) {
      const pulse = 0.92 + 0.25 * Math.sin(ambient * 6.0);
      _scratchColor.setHex(cell.rule.color).multiplyScalar(pulse);
    } else if (pose === 1) {
      _scratchColor
        .setHex(0x1b2536)
        .lerp(_scratchColorB.setHex(cell.rule.color), 0.16 + cell.trail * 0.56);
    } else {
      _scratchColor.setHex(0x1b2536).lerp(_scratchColorB.setHex(cell.rule.color), 0.16);
    }
    ruleTilesInstancedMesh.setColorAt(i, _scratchColor);
    tileColor.addUpdateRange(i * 3, 3);
    tileColorDirty = true;

    if (pose === 2) _scratchColor.setHex(cell.rule.color).multiplyScalar(1.35);
    else if (pose === 1) _scratchColor.setHex(cell.rule.color).multiplyScalar(0.32 + cell.trail * 0.65);
    else _scratchColor.setHex(cell.rule.color).multiplyScalar(0.32);
    ruleBeaconInstancedMesh.setColorAt(i, _scratchColor);
    beaconColor.addUpdateRange(i * 3, 3);
    beaconColorDirty = true;
    ruleBeaconPostInstancedMesh.setColorAt(i, _scratchColor);
    postColor.addUpdateRange(i * 3, 3);
    postColorDirty = true;
  }
}

if (tileMatrixDirty) tileMatrix.needsUpdate = true;
if (tileColorDirty) tileColor.needsUpdate = true;
if (beaconMatrixDirty) beaconMatrix.needsUpdate = true;
if (beaconColorDirty) beaconColor.needsUpdate = true;
if (postMatrixDirty) postMatrix.needsUpdate = true;
if (postColorDirty) postColor.needsUpdate = true;
```

`continue` on a settled idle cell skips the decay, the compose, and the upload. An idle frame with no active rule uploads nothing. A steady step uploads 12 bytes × 3 color buffers for the active cell (36 bytes) instead of 8892. A pose change adds one 64-byte matrix range per mesh.

`addUpdateRange(start, count)` counts array elements (`BufferAttribute.js` lines 181–184). `WebGPUAttributeUtils.updateAttribute` turns that into `writeBuffer` and then `clearUpdateRanges` (lines 223–272). Instance color `itemSize` is 3 and `3 * 4` is a multiple of 4, so r186 does not pad it (the pad path at lines 117–124 is for strides that are not 4-byte aligned). Offset `index * 12` is 4-byte aligned. Matrix ranges are `index * 16` floats = `index * 64` bytes, also aligned.

Do not call `addUpdateRange` and also expect a full-buffer upload in the same frame. An empty `updateRanges` means “upload everything”; a non-empty list means “upload only these.” The snippet never mixes them.

On a settled step the active tile’s matrix is not in the upload, and the shadow pass draws that same matrix. The contact shadow stays where the chiplet is.

`instanceMatrix.setUsage(THREE.DynamicDrawUsage)` at lines 2233, 2251, and 2260 can stay or go. WebGPU ignores it (section 1). Deleting it does not change the picture and does not change the device buffer. The `needsUpdate` gate is the change that matters.

Idle bounding sphere: call `computeBoundingSphere()` once after the init loop (after line 2333) on tiles, beacons, and posts. Do not call it from `update()`. The active tile top sits about 0.034 above the idle top (`(0.052 * 1.38) / 2 + 0.054` versus `(0.052 / 2) + 0.030`, relative to `WELL_FLOOR_Y`). The floor span is 2.92 (line 2181). The union sphere is dominated by the corner instances. The 0.034 rise stays inside a sphere computed at the idle pose, which is also the pose written at lines 2280–2304. Recomputing the sphere every frame would walk all 39 matrices inside `InstancedMesh.computeBoundingSphere` (lines 151–178) for no cull change.

### 5.2 Stator teeth (lines 2705–2730)

`padY` is `0.010` or `0.003`. Scale Y is `1.35` or `1.0`. Rotation is `θ = π/2 - i * sectorStep`, the same value written at init (line 1785). Nothing in that matrix depends on `ambient`. The active color does: `sector.color * (1.05 + 0.25 * sin(ambient * 6))` (lines 2719–2721). Inactive color is the constant `0x1e293b`.

The init loop writes Y = `0.004` (line 1786). `update()` is invoked once before return (line 3042) with no step, so `rawControlKind` falls through to `'Eval'` (line 2698) and sector 0 is active before the first paint. The visible pose is the `update()` pose. Cache that, not the `0.004` pose.

After line 1800:

```javascript
const statorIdleMatrices = new Array(STATOR_SECTOR_COUNT);
const statorActiveMatrices = new Array(STATOR_SECTOR_COUNT);
for (let i = 0; i < STATOR_SECTOR_COUNT; i += 1) {
  const theta = Math.PI * 0.5 - i * sectorStep;
  _scratchEuler.set(0, theta, 0);
  _scratchQuat.setFromEuler(_scratchEuler);
  _scratchPos.set(0, 0.003, 0);
  _scratchScale.set(1, 1, 1);
  statorIdleMatrices[i] = new THREE.Matrix4().compose(_scratchPos, _scratchQuat, _scratchScale);
  _scratchPos.set(0, 0.010, 0);
  _scratchScale.set(1, 1.35, 1);
  statorActiveMatrices[i] = new THREE.Matrix4().compose(_scratchPos, _scratchQuat, _scratchScale);
}
let lastStatorSector = -1;
```

Replace lines 2705–2730:

```javascript
if (activeSectorIdx !== lastStatorSector) {
  if (lastStatorSector >= 0) {
    statorSectorsInstancedMesh.setMatrixAt(lastStatorSector, statorIdleMatrices[lastStatorSector]);
    statorSectorsInstancedMesh.instanceMatrix.addUpdateRange(lastStatorSector * 16, 16);
    _scratchColor.setHex(0x1e293b);
    statorSectorsInstancedMesh.setColorAt(lastStatorSector, _scratchColor);
    statorSectorsInstancedMesh.instanceColor.addUpdateRange(lastStatorSector * 3, 3);
  } else {
    for (let i = 0; i < STATOR_SECTOR_COUNT; i += 1) {
      const idle = i === activeSectorIdx ? statorActiveMatrices[i] : statorIdleMatrices[i];
      statorSectorsInstancedMesh.setMatrixAt(i, idle);
      if (i !== activeSectorIdx) {
        _scratchColor.setHex(0x1e293b);
        statorSectorsInstancedMesh.setColorAt(i, _scratchColor);
      }
    }
    statorSectorsInstancedMesh.instanceMatrix.needsUpdate = true;
    statorSectorsInstancedMesh.instanceColor.needsUpdate = true;
  }
  if (lastStatorSector >= 0) {
    statorSectorsInstancedMesh.setMatrixAt(activeSectorIdx, statorActiveMatrices[activeSectorIdx]);
    statorSectorsInstancedMesh.instanceMatrix.addUpdateRange(activeSectorIdx * 16, 16);
    statorSectorsInstancedMesh.instanceMatrix.needsUpdate = true;
    statorSectorsInstancedMesh.instanceColor.needsUpdate = true;
  }
  lastStatorSector = activeSectorIdx;
}

const sector = CONTROL_STATOR_SECTORS[activeSectorIdx];
const pulse = 1.05 + 0.25 * Math.sin(ambient * 6.0);
_scratchColor.setHex(sector.color).multiplyScalar(pulse);
statorSectorsInstancedMesh.setColorAt(activeSectorIdx, _scratchColor);
statorSectorsInstancedMesh.instanceColor.addUpdateRange(activeSectorIdx * 3, 3);
statorSectorsInstancedMesh.instanceColor.needsUpdate = true;
```

On the first call `lastStatorSector === -1`, so the full buffer is written once (no `addUpdateRange` on that path). After that, a sector change uploads two matrices and two colors, and every other frame uploads 12 bytes for the pulsing tooth. `setUsage(THREE.DynamicDrawUsage)` at line 1782 is the same no-op as on the floor. The line can be deleted; behavior on this WebGPU device does not change.

`activeSectorMeta` is already resolved at line 2703. Use it instead of indexing `CONTROL_STATOR_SECTORS` again inside the loop. The replacement above reads `CONTROL_STATOR_SECTORS[activeSectorIdx]` once per frame for the pulse. That is the same object `activeSectorMeta` already points at.

### 5.3 Clasps (lines 2652–2691)

`endThetaA`, `endThetaB`, and `midR` do not change. The XZ translation and the Y rotation are constant. Only `slot.currentY - 0.002` tracks the arc. A composed matrix with scale 1 stores translation in elements 12, 13, 14. Writing element 13 is the same matrix the `compose` at lines 2666 and 2676 produces, provided the rotation was composed once.

```javascript
let claspsMatrixDirty = false;
let claspsColorDirty = false;
const claspMatrixAttr = bridgeClaspsInstancedMesh.instanceMatrix;
const claspColorAttr = bridgeClaspsInstancedMesh.instanceColor;
const claspArray = claspMatrixAttr.array;

// inside the slot loop, replace lines 2652–2686
const claspY = slot.currentY - 0.002;
const claspIdx0 = s * 2;
const claspIdx1 = s * 2 + 1;
if (slot.claspInit !== 1) {
  const endThetaA = slot.thetaStart;
  const endThetaB = slot.thetaStart + HALF_ARC_THETA_LENGTH;
  _scratchPos.set(Math.cos(endThetaA) * slot.midR, claspY, -Math.sin(endThetaA) * slot.midR);
  _scratchEuler.set(0, endThetaA, 0);
  _scratchQuat.setFromEuler(_scratchEuler);
  _scratchScale.set(1, 1, 1);
  _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
  bridgeClaspsInstancedMesh.setMatrixAt(claspIdx0, _scratchMat4);
  _scratchPos.set(Math.cos(endThetaB) * slot.midR, claspY, -Math.sin(endThetaB) * slot.midR);
  _scratchEuler.set(0, endThetaB, 0);
  _scratchQuat.setFromEuler(_scratchEuler);
  _scratchMat4.compose(_scratchPos, _scratchQuat, _scratchScale);
  bridgeClaspsInstancedMesh.setMatrixAt(claspIdx1, _scratchMat4);
  claspMatrixAttr.addUpdateRange(claspIdx0 * 16, 32);
  slot.claspInit = 1;
  slot.claspY = claspY;
  claspsMatrixDirty = true;
} else if (slot.claspY !== claspY) {
  claspArray[claspIdx0 * 16 + 13] = claspY;
  claspArray[claspIdx1 * 16 + 13] = claspY;
  claspMatrixAttr.addUpdateRange(claspIdx0 * 16 + 13, 1);
  claspMatrixAttr.addUpdateRange(claspIdx1 * 16 + 13, 1);
  slot.claspY = claspY;
  claspsMatrixDirty = true;
}

const claspHex = isOccupied ? semanticHex : 0x334155;
if (slot.claspHex !== claspHex) {
  _scratchColor.setHex(claspHex);
  bridgeClaspsInstancedMesh.setColorAt(claspIdx0, _scratchColor);
  bridgeClaspsInstancedMesh.setColorAt(claspIdx1, _scratchColor);
  claspColorAttr.addUpdateRange(claspIdx0 * 3, 3);
  claspColorAttr.addUpdateRange(claspIdx1 * 3, 3);
  slot.claspHex = claspHex;
  claspsColorDirty = true;
}
```

After the slot loop, replace lines 2688–2691:

```javascript
if (claspsMatrixDirty) claspMatrixAttr.needsUpdate = true;
if (claspsColorDirty) claspColorAttr.needsUpdate = true;
```

`addUpdateRange(claspIdx0 * 16, 32)` covers both 16-float matrices because the two clasps of a slot are adjacent (`s * 2` and `s * 2 + 1`). A settled arc writes nothing. A moving arc writes 4 bytes × 2.

Put `claspInit: 0`, `claspY: 0`, `claspHex: -1` on the slot record at line 1656.

### 5.4 Fuel colors (lines 2518–2546)

Fuel **matrices** are not written in `update()`. Only colors are. The matrix `DynamicDrawUsage` at line 1392 never pays for itself on this backend, and the color buffer is uploaded in full even after `displayedFuelRatio` has visually arrived.

The non-exhausted color is a function of `displayedFuelRatio` and `step.charge`. The exhausted color is a `sin` and must stay per-frame. Snap the exponential tail so the steady state is exact and the branch can go quiet:

```javascript
displayedFuelRatio = lerp(displayedFuelRatio, targetFuelRatio, alpha12);
if (Math.abs(displayedFuelRatio - targetFuelRatio) < 1e-5) {
  displayedFuelRatio = targetFuelRatio;
}

const fuelSettled = displayedFuelRatio === targetFuelRatio && !isExhausted && !isStepChanged;
if (!fuelSettled) {
  const litSegmentCount = displayedFuelRatio * FUEL_SEGMENTS;
  const isLowFuel = displayedFuelRatio < 0.28;
  const leadingEdgeIdx = Math.floor(litSegmentCount);
  for (let s = 0; s < FUEL_SEGMENTS; s += 1) {
    // body of lines 2524–2542, unchanged
  }
  if (fuelInstancedMesh.instanceColor) {
    fuelInstancedMesh.instanceColor.needsUpdate = true;
  }
}
```

`1e-5` on a color multiplier changes an 8-bit channel by `1e-5 * 255`, far below one code value. The `sin` path for `isExhausted` still runs every frame. `isStepChanged` covers a `step.charge` edge that does not move the fuel ratio.

### 5.5 Terrace and horizon materials (lines 2859–2903)

Non-deepest occupied rings and idle rings assign a constant hex every frame. Only the deepest occupied ring breathes, and only the fault ring pulses.

```javascript
for (let d = 0; d < depthRings.length; d += 1) {
  const ring = depthRings[d];
  const isOccupied = d < frameCount;
  const isDeepestOccupied = d === clampedDepthIdx && frameCount > 0;
  let mode = 0;
  let hex = 0x1e293b;
  let emissiveHex = 0x161d29;
  let intensity = 0.02;
  if (isFrameLimitError && d === stackLimit - 1) {
    mode = 3;
    hex = PALETTE.errorRed;
    emissiveHex = PALETTE.errorRed;
    intensity = 1.1 + (0.5 + 0.5 * Math.sin(ambient * 16)) * 1.2;
  } else if (isDeepestOccupied) {
    mode = 2;
    hex = getFrameSemanticHex(_chronoFramesBuffer[d] ?? frames[d] ?? null);
    emissiveHex = hex;
    intensity = 0.85 + (0.5 + 0.5 * Math.sin(ambient * 4.5)) * 0.35;
  } else if (isOccupied) {
    mode = 1;
    hex = getFrameSemanticHex(_chronoFramesBuffer[d] ?? frames[d] ?? null);
    emissiveHex = hex;
    intensity = 0.36;
  }
  const animates = mode === 2 || mode === 3;
  if (animates || mode !== ring.mode || hex !== ring.hex) {
    ring.material.color.setHex(hex);
    ring.material.emissive.setHex(emissiveHex);
    ring.material.emissiveIntensity = intensity;
    ring.mode = mode;
    ring.hex = hex;
  }
}
```

Idle emissive stays `0x161d29` with intensity `0.02` (lines 2874–2876). Do not copy `hex` into emissive on the idle path. Store `mode: -1` and `hex: -1` on each `depthRings` entry.

Horizon, replace lines 2894–2903:

```javascript
if (isFrameLimitError) {
  const crimsonPulse = 0.5 + 0.5 * Math.sin(ambient * 18);
  if (horizonMode !== 1) {
    horizonRingMat.color.setHex(PALETTE.errorRed);
    horizonRingMat.emissive.setHex(PALETTE.errorRed);
    horizonMode = 1;
  }
  horizonRingMat.emissiveIntensity = 1.5 + crimsonPulse * 1.5;
} else if (horizonMode !== 0) {
  horizonRingMat.color.setHex(PALETTE.gold);
  horizonRingMat.emissive.setHex(PALETTE.gold);
  horizonRingMat.emissiveIntensity = 0.45;
  horizonMode = 0;
}
```

`stackLimitGroup.position.y` still lerps (lines 2886–2892). Snap and skip the write when the group is already on the target:

```javascript
const limitTerraceIdx = clamp(stackLimit - 1, 0, TERRACE_COUNT - 1);
const targetLimitY = getTerraceY(limitTerraceIdx) - terraceHeight * 0.46;
if (Math.abs(stackLimitGroup.position.y - targetLimitY) < 1e-5) {
  if (stackLimitGroup.position.y !== targetLimitY) {
    stackLimitGroup.position.y = targetLimitY;
    stackLimitGroup.updateMatrix();
  }
} else {
  stackLimitGroup.position.y = lerp(stackLimitGroup.position.y, targetLimitY, alpha10);
  stackLimitGroup.updateMatrix();
}
```

### 5.6 Pointer material (lines 2749–2776)

`emissiveIntensity` in each branch is a constant (`1.5`, `1.85`, `1.35`, `1.1`). The `sin` in the rewrite branch is inside `targetReach`, not inside the color. `setHex` every frame does not change the tooth color while the branch and `activeSectorMeta.color` stay put.

```javascript
let needleMode = -1;
let needleHex = -1;

function paintNeedle(mode, hex, intensity) {
  if (needleMode === mode && needleHex === hex) return;
  needleTipMat.color.setHex(hex);
  needleTipMat.emissive.setHex(hex);
  needleTipMat.emissiveIntensity = intensity;
  needleMode = mode;
  needleHex = hex;
}
```

Call `paintNeedle(0, HEX_OP_AMBER, 1.5)`, `paintNeedle(1, HEX_OP_GOLD, 1.85)`, `paintNeedle(2, HEX_ENV_CYAN, 1.35)`, or `paintNeedle(3, activeSectorMeta.color, 1.1)` from the existing branches. Leave the `targetReach` formulas and the angle integration as they are.

Snap the angle and the reach so a resting hand does not dirty `transferNeedleGroup` forever:

```javascript
if (Math.abs(angleDiff) < 1e-5) {
  if (pointerAngle !== targetPointerAngle) {
    pointerAngle = targetPointerAngle;
    transferNeedleGroup.rotation.y = pointerAngle;
    transferNeedleGroup.updateMatrix();
  }
} else {
  pointerAngle += angleDiff * alpha14;
  transferNeedleGroup.rotation.y = pointerAngle;
  transferNeedleGroup.updateMatrix();
}

pointerRadialReach = lerp(pointerRadialReach, targetReach, alpha16);
if (Math.abs(pointerRadialReach - targetReach) < 1e-5) pointerRadialReach = targetReach;
```

The rewrite reach uses `Math.sin(ambient * 12)` (line 2758), so it correctly keeps updating for as long as that branch is active. Push/pop reach follows `stepFraction` and goes quiet when `stepFraction` sits on 0 or 1.

---

## 6. Zero-allocation animation updates

The scratch math types are already allocated once. The per-frame garbage is strings and one fresh array.

### 6.1 `step?.after?.frames ?? []` (line 2434)

When `step` is null or `frames` is missing, `?? []` allocates a new empty array every call. The initial `update({ ambient: 0, dt: 0.016, stepFraction: 0 })` at line 3042 does this, and so does every idle frame.

Next to `_chronoFramesBuffer` (line 2403):

```javascript
const EMPTY_FRAMES = [];
```

Line 2434 becomes:

```javascript
const frames = step?.after?.frames ?? EMPTY_FRAMES;
```

Do not push into `EMPTY_FRAMES`. The rest of `update()` only reads it.

### 6.2 Plaque key (lines 2626–2650)

`formatSlotEnvCompact` (lines 791–816) returns a new template string whenever a slot has bindings. The plaque block then builds another template (lines 2629–2631) on every slot, including idle slots (`${slot.slotCode}|IDLE'`), and compares it to `lastPlaqueKey`. That is at least 12 strings per frame, more when frames are occupied. The canvas rebuild is already behind the compare, and `createPlaqueTexture` (`theme.js` lines 329–420) already caches by content. The waste is the key, not the canvas.

`createPlaqueTexture` stores the texture in an LRU and the comment at `theme.js` lines 412–417 says not to `dispose()` an evicted texture that may still be on a mesh. Do not add a `dispose()` on the old `plaqueMat.map`.

The visible plaque depends on `frame` identity, `frame.kind`, `frame.label`, `envSummary`, `isTopOfStack`, and `slotTel.envText`. Within one `stepIndex` the trace step object is stable, so `frame` and `isTopOfStack` are stable. Compare those fields. Build the env string only when they differ. String `!==` in JavaScript compares text, so a caller that passes a new `envText` with the same characters does not rebuild.

Replace lines 2626–2650:

```javascript
const telEnv = slotTel?.envText || '';
const topBit = isTopOfStack ? 1 : 0;
const frameLabel = frame ? frame.label : '';
const frameKind = frame ? frame.kind : '';
if (
  frame !== slot.plaqueFrame ||
  frameKind !== slot.plaqueKind ||
  frameLabel !== slot.plaqueLabel ||
  telEnv !== slot.plaqueTelEnv ||
  topBit !== slot.plaqueTop ||
  isStepChanged
) {
  const resolvedEnv = formatSlotEnvCompact(_chronoFramesBuffer, s, activeBindings);
  const envSummary = resolvedEnv !== 'root' ? resolvedEnv : (telEnv || resolvedEnv);
  if (envSummary !== slot.plaqueEnv || frame !== slot.plaqueFrame || topBit !== slot.plaqueTop || frameKind !== slot.plaqueKind || frameLabel !== slot.plaqueLabel) {
    const copy = formatHalfArcPlaqueCopy(slot.slotCode, frame, envSummary);
    slot.group.userData.title = copy.title;
    slot.group.userData.subtitle = copy.subtitle;
    if (frame) {
      slot.plaqueMat.map = createPlaqueTexture({
        title: copy.title,
        subtitle: copy.subtitle,
        badge: copy.badge,
        accent: getFrameSemanticCss(frame),
        dark: true,
        width: 384,
        height: 96,
      });
      slot.plaqueMat.needsUpdate = true;
    }
    slot.plaqueEnv = envSummary;
  }
  slot.plaqueFrame = frame;
  slot.plaqueKind = frameKind;
  slot.plaqueLabel = frameLabel;
  slot.plaqueTelEnv = telEnv;
  slot.plaqueTop = topBit;
}
```

`isStepChanged` is in the outer condition so an in-place mutation of the same frame object (bindings written into the existing object without replacing it) still refreshes. The inner compare stops a second canvas upload when the step changed but the engraved string did not. `formatHalfArcPlaqueCopy` still allocates its return object, and only on that inner change, which is the same rate the current `plaqueKey` check already allows.

Initialize `plaqueFrame: null`, `plaqueKind: ''`, `plaqueLabel: ''`, `plaqueTelEnv: ''`, `plaqueTop: -1`, `plaqueEnv: ''` on the slot at line 1656. `lastPlaqueKey` can go away.

Also initialize `popPulse: 0` and `poppedKind: null` there. Today `popPulse` is added the first time line 2574 runs, which transitions the slot’s hidden class in the middle of the first trace. One-time, but the slot loop is monomorphic after that only if every slot grows the same properties in the same order. Setting them in the literal keeps one shape from the start.

### 6.3 Core lens key (lines 2779–2801)

```javascript
const coreKey = `${stepIndex}|${rawControlKind}|${activeRuleId}|${isDoneState ? 1 : 0}|${isErrorState ? 1 : 0}`;
```

allocates every frame, then compares to `lastCorePlaqueKey`. The texture rebuild is already guarded. Replace the string with the five fields:

```javascript
let lastCoreStep = -2;
let lastCoreKind = '';
let lastCoreRule = undefined;
let lastCoreDone = false;
let lastCoreErr = false;

if (
  stepIndex !== lastCoreStep ||
  rawControlKind !== lastCoreKind ||
  activeRuleId !== lastCoreRule ||
  isDoneState !== lastCoreDone ||
  isErrorState !== lastCoreErr
) {
  lastCoreStep = stepIndex;
  lastCoreKind = rawControlKind;
  lastCoreRule = activeRuleId;
  lastCoreDone = isDoneState;
  lastCoreErr = isErrorState;
  const lensCopy = formatControlCoreLensCopy(
    step,
    activeSectorMeta,
    isDoneState,
    isErrorState,
    stepState.trace?.outcome
  );
  controlCoreGroup.userData.title = lensCopy.title;
  controlCoreGroup.userData.subtitle = `${lensCopy.badge} · ${lensCopy.subtitle}`;
  lensPlaqueMat.map = createPlaqueTexture({
    title: lensCopy.title,
    subtitle: lensCopy.subtitle,
    badge: lensCopy.badge,
    accent: activeSectorMeta.css || CSS_OP_GOLD,
    dark: true,
    width: 384,
    height: 108,
  });
  lensPlaqueMat.needsUpdate = true;
}
```

`formatControlCoreLensCopy` still allocates on a real content change (`slice` / `map` / `join` at lines 868–875). That runs once per step, not once per frame. Same pixels as the string key: the key’s components are exactly these five fields.

The floor title strings at lines 2934–2935 are already inside `if (isStepChanged)`. Leave them.

### 6.4 Half-arc motion without a trailing compose

Lines 2584–2593 lerp Y and yaw and assign them even after the exponential has flatlined. Combine with the freeze in section 3:

```javascript
slot.rewritePulse = slot.rewritePulse > 0 ? Math.max(0, slot.rewritePulse - dt * 3.2) : 0;
slot.popPulse = slot.popPulse > 0 ? Math.max(0, slot.popPulse - dt * 2.6) : 0;

const targetY = isOccupied
  ? slot.baseY + 0.012 + slot.rewritePulse * 0.011
  : slot.baseY - 0.016 + slot.popPulse * 0.010;
if (Math.abs(slot.currentY - targetY) < 1e-5) slot.currentY = targetY;
else slot.currentY = lerp(slot.currentY, targetY, alpha14);

const direction = slot.isEast ? 1 : -1;
const targetRotY = slot.rewritePulse * direction * 0.042 * Math.sin(slot.rewritePulse * Math.PI);
if (Math.abs(slot.currentRotY - targetRotY) < 1e-5) slot.currentRotY = targetRotY;
else slot.currentRotY = lerp(slot.currentRotY, targetRotY, alpha16);

if (slot.group.position.y !== slot.currentY || slot.group.rotation.y !== slot.currentRotY) {
  slot.group.position.y = slot.currentY;
  slot.group.rotation.y = slot.currentRotY;
  slot.group.updateMatrix();
}
```

`1e-5` world units on an arc whose radius is between 0.31 and 0.95 is not a visible step. Cutting the infinite tail is what lets the clasp Y-write and the group matrix go idle. While `rewritePulse` or `popPulse` is above 0, `targetY` still moves and the lerp still runs.

---

## 7. Rejected

These would show up in a screenshot or under the shadow map. They are not in the patch list.

| Idea | Why it changes the picture |
| :--- | :--- |
| One draw for bezels + tiles + beacons | Three BRDFs: gold standard, clearcoat physical, unlit basic. A shared material drops clearcoat or lights the beacons |
| `BatchedMesh` for the two beacon meshes | Full texture upload on every `setMatrixAt`, and instance color goes through a working-color-space `DataTexture` instead of a linear instanced attribute |
| Fold `romBezelMesh` into `ruleBezelInstancedMesh` | The instanced mesh receives shadows (line 2215). The ROM rim does not |
| `castShadow = false` on the tiles, or only while idle | Deletes the contact shadow of the raised chiplet. Idle tiles are 0.052 tall and still sit in the key-light frustum (`far = 19`, floor at Y = −3.38, light at Y = 10.5) |
| `receiveShadow = false` on the floor, bezels, vault, or stator | Removes shadow receiving from those surfaces |
| `MeshPhysicalMaterial` → `MeshStandardMaterial` on the tiles (line 2219), the sapphire annulus (line 1874), the stator housing (line 1720), or the ROM body (line 2357) | Drops clearcoat |
| `sapphireLensMat.depthWrite = false` | Changes which transparent fragments survive against the annulus |
| Toggling `material.transparent` when opacity is 0 or 1 | Pipeline rebuild, and a one-frame sort-bucket change |
| `alphaTest` on `floorOverlayMat`, or a smaller overlay texture | Clips label AA, or blurs the engraving |
| Re-sorting transparents by material to reduce pipeline switches | Painter order is what makes the wave, the dials, and the plaques composite. Sorting by material changes overlap |
| Per-instance frustum culling of the 39 tiles | Three.js tests one sphere for the whole `InstancedMesh`. A manual per-instance test would drop a tile whose sphere was computed before a pose change if the test were tight. The floor is one compact grid; one draw is the right cull |
| Recomputing `computeBoundingSphere` inside `update()` | No pixel change, but it walks every instance matrix every frame and erases the upload win |

---

## 8. Order to apply

1. Floor pose buckets and `addUpdateRange` (section 5.1). Largest upload. No geometry change. Easy to diff: idle frame should show zero `writeBuffer`s on the three floor instance attributes; a held step should show three 12-byte color writes.
2. Clasp element-13 update and stator matrix cache (sections 5.3 and 5.2). Removes the per-frame Euler traffic.
3. Plaque and lens field compares, `EMPTY_FRAMES`, half-arc snap (section 6). Allocation profile of `update()` should go flat between steps.
4. Material latches and the cutaway gate (sections 4.2–4.4).
5. `freezeWorld` / `freezeLocal` after parenting (section 3).
6. Merged stator hardware, merged needle, merged light pipe (section 2.2 and 2.3).
7. Instanced chamber (section 2.4). Do this last. It is the largest structural edit and the easiest place to rig the south wall to the wrong parent.

After 1–3, a steady step at 60 fps should keep the active tooth pulsing, the active chiplet pulsing, and the occupied light pipes breathing, and should not upload a floor matrix, a clasp matrix, or a stator matrix. A step change should upload two stator matrices, the previous and next floor poses, and the clasp colors whose hex changed. Cutaway motion should move only south objects. The open bore (`r < 0.135`), the underside plaques, and the raised-chiplet shadow stay.
