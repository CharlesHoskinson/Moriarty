# Grok 4.7 xhigh Performance Auditor #2 — Designer 1 Package & Lithography

Auditor: performance only. Visual output stays the same: same poses, same materials, same textures, same engraved strings, same lead pulse, same cutaway motion.

Scope read in full: `src/designers/designer1_package.js` (917 lines). Cross-checked against `src/stage/SherlockStage.js` (one shadow-casting key light, shadow ortho `[-3.6, 2.6] × [-2.8, 2.8]`, map `2048²`, `normalBias = 0.004`), `src/engine/sherlockEngine.js` `buildStepState` (phase set), `src/app.js` `getOrBuildStepState` (step object reused while the integer step holds), `src/designers/theme.js` `createBrushedMetalTexture` / plaque LRU, and the installed Three.js `Object3D.updateMatrixWorld`, `InstancedMesh.setColorAt`, `BufferAttribute.usage`, and `CurvePath.getPoints`.

No source file was modified. Shadow residuals below were computed from the constants in this file and in `SherlockStage.js`, not from a screenshot.

Scale used only for the shadow residual: package plan `±2.15`. The key light sits at `(6, 10.5, 5.4)` and aims at the origin, so the parallel shadow direction is `normalize(-6, -10.5, -5.4)`.

---

## Verdict

The steady frame is already careful about GPU texture uploads and about the 24 gold leads: one `InstancedMesh`, module-scope scratch colors, `instanceColor` marked `DynamicDrawUsage`, and a real `STILL` early-out. Those parts should stay.

The frame still pays for work that does not move a pixel:

| Cost every frame | Where | After the edits below |
| :--- | :--- | :--- |
| Local `matrix.compose` on 26 objects | default `matrixAutoUpdate === true` | 0 composes while the cutaway is settled; 2 while it is moving (`frontGateGroup`, `benchMesh`) |
| 4 torus draws + 4 groove draws | lines 644–667 | 1 instanced torus draw + 1 instanced groove draw + 1 south groove mesh |
| 1 instanced shadow draw of 24 leads | line 682 | 0; the ceramic tier already owns that shadow |
| Cutaway writes even when `cutawayMix` is unchanged | lines 820–822 | write + `updateMatrix()` only on change |

World-matrix multiplies still run. `Scene` and `SherlockStage.root` keep `matrixAutoUpdate` on, and `Object3D.updateMatrixWorld` forces every descendant when an ancestor recomputes (`Object3D.js` around 1176–1210). Freezing local matrices removes the quaternion compose. It does not remove the parent×local multiply. Do not set `matrixWorldAutoUpdate = false` on gate children. That flag would leave them behind when `frontGateGroup` drops.

`CurvePath.getPoints` feeds every `EllipseCurve` at `curveSegments * 2` (`CurvePath.js` 207–208). The tier bore arcs are 192 and 96 divisions. Leave that tessellation. A lower count is a different mesh, which fails a pixel-identical check at a bore-filling crop.

---

## What already holds the frame rate

- **Scratch math, lines 8–15.** `_tmpColorA`, `_tmpColorB`, `_tmpMatrix`, `_tmpPos`, `_tmpQuat`, `_tmpScale`, `_upAxis` live at module scope. `InstancedMesh.setColorAt` writes into the existing `Float32Array` (`InstancedMesh.js` 318–327). `Color.setHex` / `Color.lerp` mutate in place. The lead loop does not construct `THREE.*` objects.
- **Lead batch, lines 670–720.** 24 boxes, one geometry, one material, one `InstancedMesh`. Matrices are written once. `instanceMatrix` stays on the default `StaticDrawUsage` (`BufferAttribute.js` 111). `instanceColor.setUsage(DynamicDrawUsage)` is set at line 717, before the first render, which is the only moment `setUsage` is honored.
- **`STILL` lead upload, lines 857–860.** `buildStepState` uses `INGEST`, `DESCEND`, `RESOLVE`, `RETURN`, `EMBLEM`, `ERROR`, and `STILL`. Every phase except `STILL` advances a sine with `ambient`, so a per-frame color write is the look. `STILL` writes the idle gold once, then returns. `app.js` keeps the same `trace` object and the same step record until the integer step changes, so the plaque tests above the return stay quiet on a held step.
- **Plaque dirty checks, lines 824–855.** South title repaints only when `trace`, `trace.source`, or `trace.programName` changes and the extracted label actually differs. Front status repaints only when `stepIndex` or the trace reference changes, and then only when `text|color` changes. `paintRimEngraving` is not on the per-frame path.
- **Ceramic pair and silver roughness are singletons, lines 32–34, 56–59, 132–134.** The second call returns the same `CanvasTexture` objects. `createBrushedMetalTexture` is a second cache inside `theme.js` (`metalTextureCache`, key `` `${size}|${base}|${alpha}` ``).
- **Shared materials.** Tier U-body and gate share one material per tier. The four rings share `machinedSilverMat` and one `TorusGeometry`. The groove pair shares `grooveMat` and two boxes. Collars share `collarMat`.
- **Micro shadow flags that are already off.** Grooves (lines 644–655), fiducial rings (662), and the three engraving planes (729, 754, 779) never set `castShadow` / `receiveShadow`. Both default to `false` (`Object3D.js` 299–307). The bench receives and does not cast (433). The collars receive and do not cast (622, 630). Pin the micro flags in code so a later edit cannot turn a 2.5×0.24 text quad into a shadow caster. A shadow pass uses the quad, not the ink.
- **Gate hide, line 821.** `frontGateGroup.visible = cutawayMix < 0.96` drops the south bay’s draws and shadow casters once the stage has snapped the frames preset. Keep that threshold.

---

## Findings

### P0 — Freeze local matrices on every static node

`Object3D.DEFAULT_MATRIX_AUTO_UPDATE` is `true`. Each frame `updateMatrixWorld` calls `updateMatrix()`, which composes `position`, `quaternion`, and `scale` and sets `matrixWorldNeedsUpdate`. One auto-updating ancestor then forces the walk down the whole package.

Nothing in this file moves these nodes after the pose is set. `group` is parented under `SherlockStage.root` and this file never writes its transform. `benchGroup` is identity. `substrateGroup` and `lidGroup` are identity (they exist so `tagInspectable` has a root). Tier meshes, collars, grooves, rings, the lead `InstancedMesh`, and the three engraving planes keep the pose written at construction. Gate children move only because `frontGateGroup.position.y` changes. Their local matrices stay valid if they are baked once. The parent’s forced walk still refreshes `matrixWorld`.

Count: **26** `Object3D`s. **24** are static. The other two are `benchMesh` (line 430) and `frontGateGroup` (line 488), handled in the next finding.

Apply after the pose is written, never before. `updateMatrix()` first. With `matrixAutoUpdate` already false, a later `position.set` is invisible until `updateMatrix()`.

Add the helper next to `track` (after line 396):

```javascript
const freeze = (obj) => {
  obj.updateMatrix();
  obj.matrixAutoUpdate = false;
  return obj;
};

// Grooves, fiducials, engraving quads, lead instances.
// Shadow defaults are already false; pinning them keeps a 0.003-tall
// groove and a text quad out of the shadow pass.
const freezeMicro = (obj) => {
  obj.castShadow = false;
  obj.receiveShadow = false;
  return freeze(obj);
};
```

Bake each static object at the line that finishes its pose:

```javascript
// line 403, after group.add(benchGroup)
freeze(benchGroup);

// line 445, after group.add(substrateGroup)
freeze(substrateGroup);

// line 490, frontGateGroup is NOT frozen here — see P0 cutaway

// end of buildSplitCircularBoreTier, after each parent.add
freeze(uMesh);     // keeps castShadow / receiveShadow from lines 534–535
freeze(gateMesh);  // keeps lines 559–560; local pose only

// line 585, after group.add(lidGroup)
freeze(lidGroup);

// lines 623 and 631, after add
freeze(collarMesh);
freeze(gateCollarMesh);

// grooves — see the instancing snippet; freezeMicro the survivor mesh

// lines 743, 768, 793, after each engraving mesh is added
freezeMicro(northWordmarkMesh);
freezeMicro(southPlateMesh);
freezeMicro(frontValueMesh);

// line 683, after the lead InstancedMesh is added
freezeMicro(edgePadsInstanced);

// line 391, once every child exists and the root transform is still identity
freeze(group);
```

`freeze(group)` must run after the children are parented and after their local matrices are baked. `tagInspectable` (lines 795–809) does not write transforms, so it can stay where it is.

Tier meshes keep `castShadow = true` and `receiveShadow = true`. Tier 2 (`half = 1.98`) and tier 3 (`half = 1.82`) sit inside tier 1’s plan, but they cast the step shadow onto the deck below. That shadow is visible. The gate casters stay too: once the gate lowers, the south opening has to stop casting and the lowered plug has to cast from `y = -cutawayMix * 3.12`.

---

### P0 — Dirty-check `cutawayMix` before writing the two live matrices

Lines 819–822 run on every `update`, including a settled overview where `cutawayMix` is `0` and a settled frames view where the stage has snapped it to `1` (`SherlockStage.js` 295–299):

```javascript
frontGateGroup.position.y = -cutawayMix * 3.12;
frontGateGroup.visible = cutawayMix < 0.96;
benchMesh.position.y = -0.36 - cutawayMix * 3.08;
```

Assigning `position.y` does not allocate. With `matrixAutoUpdate` left on, the assign is also pointless: the compose runs whether or not the number changed. Bake both nodes, then recompute only on a real change.

Constructed pose is the `cutawayMix === 0` pose (`benchMesh.position.y = -0.36`, gate at `y = 0`). Record that before the first `update` (the call at line 909 passes no `cutawayMix`, so the `?? 0` path is the same pose).

Next to the other latches (after line 776):

```javascript
let lastCutawayMix = 0;
freeze(benchMesh);
freeze(frontGateGroup);
```

Replace lines 814–822 with:

```javascript
const _EMPTY_STEP = Object.freeze({});

function update(stepState) {
  const state = stepState == null ? _EMPTY_STEP : stepState;
  const ambient = state.ambient ?? 0;
  const phase = state.phase ?? 'STILL';
  const cutawayMix = state.cutawayMix ?? 0;

  // Strict inequality. The stage already snaps to 0 or 1 inside 0.005.
  // An epsilon here would stop the gate short of the authored pose.
  if (cutawayMix !== lastCutawayMix) {
    lastCutawayMix = cutawayMix;
    frontGateGroup.position.y = -cutawayMix * 3.12;
    frontGateGroup.updateMatrix();
    const gateVisible = cutawayMix < 0.96;
    if (frontGateGroup.visible !== gateVisible) {
      frontGateGroup.visible = gateVisible;
    }
    benchMesh.position.y = -0.36 - cutawayMix * 3.08;
    benchMesh.updateMatrix();
  }

  // plaque blocks (lines 824–855) stay here, above the lead early-out
```

`_EMPTY_STEP` replaces the default parameter `stepState = {}` at line 814. A default `{}` is evaluated on every call that omits the argument. `SherlockStage.render` always passes `fullState`, so this is not today’s per-frame allocation. It is the leak if `update()` is ever called bare. The init call at line 909 passes an object and does not use the default.

Keep the lead `return` (lines 858–860) **after** the cutaway block and the plaque block. A `STILL` frame in the frames preset still has to move the gate.

---

### P0 — Instance the four fiducial rings

Lines 658–667 build four `Mesh` objects on one `TorusGeometry` and one `machinedSilverMat`. The only differences are translation. Rotation is `x = π/2` on every ring. Draw calls go from 4 to 1. The physical material is shared with the silver tier, so the renderer keeps two pipelines (instanced and not). That is the same split Three.js already uses for the leads. The pixels match.

`_tmpScale` is already `(1, 1, 1)`. A `+π/2` rotation about X is the quaternion `(√2/2, 0, 0, √2/2)`. No new `Vector3`.

Replace lines 658–667:

```javascript
const cornerRingGeo = track(new THREE.TorusGeometry(0.044, 0.008, 8, 24));
const cornerRings = new THREE.InstancedMesh(cornerRingGeo, machinedSilverMat, 4);
cornerRings.name = 'CornerFiducialRings';
_tmpQuat.set(Math.SQRT1_2, 0, 0, Math.SQRT1_2);
const fiducialXZ = [-1.62, 1.62];
let fiducial = 0;
for (let sx = 0; sx < 2; sx++) {
  for (let sz = 0; sz < 2; sz++) {
    _tmpPos.set(fiducialXZ[sx], 0.108, fiducialXZ[sz]);
    _tmpMatrix.compose(_tmpPos, _tmpQuat, _tmpScale);
    cornerRings.setMatrixAt(fiducial++, _tmpMatrix);
  }
}
cornerRings.instanceMatrix.needsUpdate = true;
cornerRings.instanceMatrix.setUsage(THREE.StaticDrawUsage);
cornerRings.computeBoundingSphere();
lidGroup.add(cornerRings);
freezeMicro(cornerRings);
```

Loop order matches the current `sx` outer / `sz` inner order: `(-1.62, -1.62)`, `(-1.62, +1.62)`, `(+1.62, -1.62)`, `(+1.62, +1.62)`. `setUsage` has to run before the first render. Do not call it from `update`.

Tube radius is `0.008`. These rings sit on the lid and do not cast today. `freezeMicro` keeps it that way. Turning `castShadow` on would add a lid smudge about two shadow texels long and a fourth physical shadow draw. Leave it off.

---

### P0 — Instance the three lid grooves; leave the south groove on the gate

Lines 641–656 create four meshes. North and south share `grooveHGeo` (`BoxGeometry(grooveOffset * 2, 0.003, 0.010)`). The two verticals share `grooveVGeo` (`BoxGeometry(0.010, 0.003, grooveOffset * 2)`). Those boxes are the same shape with a `+π/2` yaw.

A yaw of `+π/2` about Y sends local `+X` to world `-Z` and local `+Z` to world `+X`. A long-in-X, thin-in-Z box becomes thin-in-X (`0.010`) and long-in-Z (`grooveOffset * 2`), which is `grooveVGeo`. Positions stay `(±grooveOffset, 0.106, 0)`.

The south groove is a child of `frontGateGroup` (line 650) so it drops with the cutaway. The other three stay on `lidGroup`. One `InstancedMesh` cannot have two parents. Putting the south instance in the lid batch and baking `gateY` into its matrix would also keep it visible after `frontGateGroup.visible` flips off, unless that instance is scaled to 0 and the bounding sphere is rebuilt. The south piece is one 12-triangle draw. Keep it as a mesh on the gate.

Replace lines 641–656:

```javascript
const grooveLen = grooveOffset * 2;
const grooveGeo = track(new THREE.BoxGeometry(grooveLen, 0.003, 0.010));
const grooveLid = new THREE.InstancedMesh(grooveGeo, grooveMat, 3);
grooveLid.name = 'LidPerimeterGrooves';

_tmpQuat.identity();
_tmpPos.set(0, 0.106, -grooveOffset);
_tmpMatrix.compose(_tmpPos, _tmpQuat, _tmpScale);
grooveLid.setMatrixAt(0, _tmpMatrix);

_tmpQuat.setFromAxisAngle(_upAxis, Math.PI / 2);
_tmpPos.set(-grooveOffset, 0.106, 0);
_tmpMatrix.compose(_tmpPos, _tmpQuat, _tmpScale);
grooveLid.setMatrixAt(1, _tmpMatrix);
_tmpPos.set(grooveOffset, 0.106, 0);
_tmpMatrix.compose(_tmpPos, _tmpQuat, _tmpScale);
grooveLid.setMatrixAt(2, _tmpMatrix);

grooveLid.instanceMatrix.needsUpdate = true;
grooveLid.instanceMatrix.setUsage(THREE.StaticDrawUsage);
grooveLid.computeBoundingSphere();
lidGroup.add(grooveLid);
freezeMicro(grooveLid);

const southGrooveMesh = new THREE.Mesh(grooveGeo, grooveMat);
southGrooveMesh.position.set(0, 0.106, grooveOffset);
frontGateGroup.add(southGrooveMesh);
freezeMicro(southGrooveMesh);
```

Height `0.003` matches one shadow texel of this rig (`6.2 / 2048 ≈ 0.0030`). They already do not cast. `freezeMicro` records that.

Draw calls for this subsystem, color pass:

| Mesh | Now | After |
| :--- | ---: | ---: |
| Bench, 3 U-bodies, 3 gates, 2 collars | 9 | 9 |
| Grooves | 4 | 2 |
| Fiducial rings | 4 | 1 |
| Leads | 1 | 1 |
| Engraving planes | 3 | 3 |
| **Total** | **21** | **16** |

The three engraving planes stay separate. Sizes differ (`2.15×0.28`, `2.25×0.26`, `2.50×0.24`), maps differ, and south/front are on the gate while north is on the lid. An atlas would be a different texture. Six triangles are not worth that.

---

### P0 — Stop casting shadows from the 24 leads

Line 682 sets `edgePadsInstanced.castShadow = true`. The key light is the only shadow caster in the stage. This flag adds one instanced shadow draw of 24 boxes every frame.

The lead does not extend the bench shadow past the ceramic, except for a residual smaller than the bias the stage already applies.

- Tier 1 top is `y = -0.14` at plan `±2.15` (lines 564–570: `yTop - depth = -0.35`, depth `0.21`).
- Lead center is `(CERAMIC_HALF - 0.01, -0.245)` in the side’s outward frame (line 701), box `0.06 × 0.14 × 0.11` (line 672). Outer face is at `2.17` (lip `0.020`). Top of the box is `y = -0.175` (drop `0.035` below the ceramic lip).
- Parallel projection onto the bench `y = -0.36` shifts a point by `t = (-0.36 - y) / Ly` along the light.
- East/west slope is `6 / 10.5 = 4/7`. The extra ray travel from the higher ceramic lip is `0.035 * 4/7 = 0.020`, the same as the proud lip. Ground `x` of the ceramic corner and of the lead’s outer top are both `-2.2757…`. Residual `0`.
- North/south slope is `5.4 / 10.5 = 18/35`. Extra ceramic travel is `0.035 * 18/35 = 0.018`. Residual `0.002` world units.
- Shadow ortho texel is `5.6 / 2048 ≈ 0.00273` on the short axis. On the bench the footprint stretches by `1 / |Ly| ≈ 1.26`, about `0.0034`. `normalBias` is `0.004`. The `0.002` lip is under both.

Tier 2 and tier 3 are inset (`1.98`, `1.82`), so they are not the ground silhouette. The leads on the lit sides throw their shadow back into the ceramic volume, not onto an open patch of bench.

Replace line 682:

```javascript
edgePadsInstanced.castShadow = false;
edgePadsInstanced.receiveShadow = false;
```

`freezeMicro(edgePadsInstanced)` from the first finding does the same two assignments. Do not also turn `receiveShadow` on. The leads are fully lit today. Receiving the step shadow would darken them.

Shadow draws go from 7 (3 U-bodies + 3 gates + the lead instances) to 6.

Also pin the static instance buffer next to line 715, before the first render:

```javascript
edgePadsInstanced.instanceMatrix.setUsage(THREE.StaticDrawUsage);
edgePadsInstanced.instanceMatrix.needsUpdate = true;
```

The default is already `StaticDrawUsage`. The explicit call is the contract. `update` never writes instance matrices. Leave `instanceColor` on `DynamicDrawUsage` (line 717).

---

### P1 — Cache the bench texture the same way as the ceramic pair

`getAlabasterCeramicTextures` and the silver roughness map return a module singleton. `createCleanBenchGroundTexture` (lines 177–279) does not. `createPackageSubsystem` calls it once (line 421) and `track()` puts the texture in `disposables`, so `dispose()` (lines 898–906) destroys it.

One call uploads a `2048²` RGBA image plus mips (about 16 MB of base pixels, about 21 MB with a full mip chain). A second `createPackageSubsystem()` in the same session paints that canvas again. Cache it, and do not dispose the shared texture when one stage shuts down. That matches the ceramic maps, which are already left out of `track()`.

```javascript
let _cachedBenchTex = null;

function createCleanBenchGroundTexture() {
  if (_cachedBenchTex) return _cachedBenchTex;
  // … existing raster, unchanged, lines 178–277 …
  const tex = new THREE.CanvasTexture(canvas);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = 8;
  tex.needsUpdate = true;
  _cachedBenchTex = tex;
  return tex;
}
```

At line 421, stop tracking the texture. Keep tracking the material:

```javascript
const benchTex = createCleanBenchGroundTexture();
const benchMat = track(new THREE.MeshStandardMaterial({
  map: benchTex,
  roughness: 0.96,
  metalness: 0.0,
}));
```

Disposing `benchMat` does not dispose `map`. The singleton survives a second stage. The first session still pays the 2048 raster once. That cost is the drawing, and shrinking the canvas would soften the 64 px drafting grid.

---

### P1 — Build the ceramic and roughness images in one typed-array pass

Both textures are already cached. The first call is still a main-thread hitch:

- Lines 86–101: 14,000 `fillStyle` template strings and 28,000 `fillRect`s.
- Lines 153–157: 512 more `fillStyle` strings, one per row of the roughness map.

The rects are integer-aligned (`Math.floor` coordinates, integer `r`) and opaque, so `fillRect` replaces pixels. The same seed, the same `nextRand` call order, and a clip to `[0, size)` produce the same bytes. Base fills stay `#fcfbfa` (`252, 251, 250`) and bump `128`, roughness base `212`.

Replace the ceramic loop (lines 86–101) after the two base `fillRect`s:

```javascript
const colorImg = cCtx.getImageData(0, 0, size, size);
const bumpImg = bCtx.getImageData(0, 0, size, size);
const cd = colorImg.data;
const bd = bumpImg.data;

for (let i = 0; i < 14000; i++) {
  const x = Math.floor(nextRand() * size);
  const y = Math.floor(nextRand() * size);
  const r = 1 + Math.floor(nextRand() * 2.2);
  const delta = (nextRand() - 0.45) * 12;

  const cr = Math.max(245, Math.min(255, Math.round(252 + delta * 0.5)));
  const cg = Math.max(243, Math.min(255, Math.round(251 + delta * 0.5)));
  const cb = Math.max(240, Math.min(255, Math.round(249 + delta * 0.5)));
  const bh = Math.max(0, Math.min(255, Math.round(128 + delta * 3.8)));

  const x1 = Math.min(size, x + r);
  const y1 = Math.min(size, y + r);
  for (let py = y; py < y1; py++) {
    let row = (py * size + x) * 4;
    for (let px = x; px < x1; px++) {
      cd[row] = cr;
      cd[row + 1] = cg;
      cd[row + 2] = cb;
      cd[row + 3] = 255;
      bd[row] = bh;
      bd[row + 1] = bh;
      bd[row + 2] = bh;
      bd[row + 3] = 255;
      row += 4;
    }
  }
}

cCtx.putImageData(colorImg, 0, 0);
bCtx.putImageData(bumpImg, 0, 0);
```

`r` is `1 + floor(u * 2.2)` with `u ∈ [0, 1)`, so `r ∈ {1, 2, 3}`, matching the current `fillRect` footprint. Keep the `CanvasTexture` setup at lines 103–121, including `repeat (1.5, 1.5)`, `SRGBColorSpace` on the color map, and `NoColorSpace` on the bump.

Roughness, replace lines 153–157:

```javascript
const roughImg = ctx.getImageData(0, 0, size, size);
const rd = roughImg.data;
for (let y = 0; y < size; y++) {
  const baseVal = 202 + Math.round(nextRand() * 22);
  let row = y * size * 4;
  for (let x = 0; x < size; x++) {
    rd[row] = baseVal;
    rd[row + 1] = baseVal;
    rd[row + 2] = baseVal;
    rd[row + 3] = 255;
    row += 4;
  }
}
ctx.putImageData(roughImg, 0, 0);
```

The base `fillRect` of `rgb(212, 212, 212)` is fully overwritten, row by row, as it is today. `getImageData` is only there so a missed pixel would still match. Either path is one upload into `_cachedSilverRoughnessTex`.

---

### P1 — Latch engraving paints on the plaque itself

`paintRimEngraving` (lines 310–323) always clears, sets a new font string, fills, and sets `tex.needsUpdate`. Callers at lines 837–839 and 851–853 already skip that when the label is unchanged. The helper itself will re-upload `1024×220` (about 900 KB) if a later caller forgets the latch. A repeat of the same string is the same image.

```javascript
function paintRimEngraving(plaque, text, color = '#e7e1d4', fontSize = 66) {
  if (!plaque.ctx || !plaque.canvas) return;
  const key = fontSize + '|' + color + '|' + text;
  if (plaque._paintKey === key) return;
  plaque._paintKey = key;

  const { ctx, canvas, tex } = plaque;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = color;
  ctx.font = `600 ${fontSize}px "JetBrains Mono", ui-monospace, monospace`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  if ('letterSpacing' in ctx) ctx.letterSpacing = '0.14em';
  ctx.fillText(text, canvas.width / 2, canvas.height / 2);
  tex.needsUpdate = true;
}
```

Forward playback still uploads once per distinct front label. The step counter is part of that string (`extractFrontValueStatus`, lines 378–380), so a cache cannot remove those uploads. It can only remove a repeat.

Scrubbing back to a label that is not the immediately previous one still re-rasters. A module LRU of `CanvasTexture`s keyed the same way, assigned with `mesh.material.map = cached`, avoids the second GPU upload. Cap it (24 entries is about 22 MB at `1024×220` RGBA). Do not `dispose()` an evicted texture that might still be `material.map`. Do not set `material.needsUpdate` on the swap: the material was already compiled with a map, and a material rebuild is the expensive path this cache is trying to avoid. On a cache miss, paint once into a new plaque and store `plaque.tex`. North’s `SHERLOCK` plate is static (line 727) and stays a single texture.

`theme.js` `createPlaqueTexture` is a different layout (filled rounded rect, badge, subtitle). Pointing the rim meshes at it would change the engraving. Keep `paintRimEngraving`.

---

### P2 — Change-path allocations that are already off the steady frame

Confirmed against `app.js` 910–928 and the conditions at lines 828–855: a held integer step does not call `extractProgramExpressionLabel` or `extractFrontValueStatus`. The steady lead loop allocates nothing. These are the holes on the change path, and the bare-call hole from P0.

1. **Hoist the program regexp, line 328.** A regexp literal inside a function allocates a `RegExp` every evaluation. It runs only when the trace identity or source changes.

```javascript
const ENTRY_EXPR_RE = /entry\s+([^;]+);/;
// inside extractProgramExpressionLabel
const match = source.match(ENTRY_EXPR_RE);
```

2. **Reuse the front-status record, lines 348–381.** The function returns a fresh object, then line 850 builds `` `${text}|${color}` ``. `fillText` needs a string, so the string stays. The object does not.

```javascript
const _frontStatus = { text: '', color: '' };

function extractFrontValueStatus(stepState) {
  // … same branches, but write _frontStatus.text / _frontStatus.color
  // and `return _frontStatus` from every branch
}
```

Every return path has to write both fields. A caller that stored the object and expected it to stay frozen would now see later steps. The only caller is `update`, which reads it immediately.

3. **Set the lead base color once.** Line 863 calls `_tmpColorB.setHex(CRYO_GOLD_HEX)` on every animated frame. `lerp` mutates `_tmpColorA` only. Next to line 15:

```javascript
_tmpColorB.setHex(CRYO_GOLD_HEX);
```

Delete line 863. Idle and pulse colors stay the same.

---

## Leave these alone

Each one changes pixels, the cutaway, or the zoomed bore.

| Idea | Why it stays |
| :--- | :--- |
| Lower `curveSegments` (96 / 48) or `bevelSegments` | `CurvePath` samples each `absarc` at `curveSegments * 2`. The tier bore at line 518 spans exactly `2π` (`start = -π/2 - π/4`, `end = 3π/2 - π/4`), so that arc is a 192-point ellipse. Fewer points are a different silhouette at a crop that fills the bore. |
| Retessellate the bench hole (`ShapeGeometry` 64, line 418) | Same constraint on the open well. The hole has to stay a real contour so the chamber is open. An alpha punch on a solid quad would write depth across the bore. |
| Drop `clearcoat` on the three body materials and the collar (lines 460, 473, 609) | Removes the second specular lobe. |
| `collarMat.side = FrontSide` (line 613) | The lathe is viewed from the deck and from inside the well. One side of the profile would disappear. |
| `transparent: false` on the engraving materials (lines 734, 759, 784) | `fillText` leaves partial-alpha edge pixels. `alphaTest: 0.08` plus blending is the current ink. Opaque alpha-test hardens the rim. |
| Lower `anisotropy` from 8, or shrink the 2048 bench | Softens the brush streaks and the 64 px drafting grid. |
| Quantize lead colors to 1/255 and skip `needsUpdate` | The render target is not documented as 8-bit. A float buffer would show the step. The upload is 24×3 floats, and `STILL` already skips it. |
| `matrixWorldAutoUpdate = false` on gate children | They would ignore `frontGateGroup`’s motion. Freeze local matrices only. |
| Merge south groove, south plate, and front value into the lid instances | They have to follow `frontGateGroup` and disappear with it at `cutawayMix >= 0.96`. |
| One material shared across ceramic, silver, and obsidian | Different PBR parameters are the finish. |
| Disable `castShadow` on the six tier meshes | Those are the contact shadow and the step shadows. The lead prune is safe because the lip is covered. The tiers are the thing covering it. |

---

## Suggested apply order

1. `freeze` / `freezeMicro` and the cutaway latch. Visual check: overview, frames preset in the middle of the drop, frames preset fully open. The south bay, the south engraving, and the bench all move as they do now.
2. Ring `InstancedMesh`, then groove `InstancedMesh` plus the south mesh. Check the four rings at `(±1.62, 0.108, ±1.62)` and the four grooves, including the south groove while the gate is moving.
3. `castShadow = false` on the leads. Compare the bench contact shadow on the west and north sides against the ceramic lip.
4. Bench singleton (and untrack it), then the ImageData ceramic and roughness passes. Diff the pixels before trusting the upload: same seed, same `nextRand` order.
5. Plaque latch, hoisted regexp, reused front-status object, one-time `_tmpColorB`.

Steady `STILL` frame after that: no lead color upload, no plaque upload, no cutaway matrix write, no local compose on the package tree, 16 color draws, 6 shadow draws. Animated phases still upload `instanceColor` once per frame. That upload is the pulse.
