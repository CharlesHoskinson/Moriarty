# Gemini Argon High Performance Audit #7: Designer 8 — Quineling 3D Morphogenesis (`src/designers/designer8_quineling.js`)

## Executive Summary
A systematic line-by-line performance audit of `/home/charles/sherlock-bench/src/designers/designer8_quineling.js` (and its compiler bridge in `src/engine/sherlockQuinelingCompiler.js`) identified **7 high-impact optimization opportunities** that preserve **100% of visual, mathematical, and animation fidelity**:

1. **P0 — Static Geometry + Y-Rotation Equivalence for the 16 Chamber Tendrils (`lines 677–693, 1720–1760`)**: Eliminates rebuilding 320 vertices / 160 spline points, 800 `Math.sin`/`Math.cos` calls, 320 `Math.hypot` calls, and 3 GPU buffer uploads on **every frame**.
2. **P0 — Zero-Allocation Surface Embedding (`embedOnSpeciesSurface`), Step-Cached SDK Ridge Tables (`sampleSdkRidgePoint`), & Static-Species Ribbon Skipping (`lines 424–577, 1375–1460`)**: Eliminates **592 heap object allocations per frame** (`320` in `embedOnSpeciesSurface` + `272` in `sampleSdkRidgePoint`), caches `WovenBody.frame` ridges across step scrubbing in a per-artifact `Map`, and skips per-frame `sdkRibbonGeom` rebuilds for the 4 species whose surface embedding is independent of `sdkDelta`.
3. **P0 — `writeRibbonStrip` Precomputed Trig LUTs, Fast Normal Cross-Products, Flat `Float32Array` Points, and Color Buffer Dirty-Checking (`lines 332–393, 1289–1292, 1445–1460, 1505–1530, 1618–1649`)**: Eliminates **1,152 `Math.sin` calls per frame** inside `writeRibbonStrip`, replaces variadic `Math.hypot` with `Math.sqrt`, exploits `by === 0` in the cross product, and stops re-uploading static vertex color buffers every frame.
4. **P1 — Eliminate Per-Frame Spline Evaluations & `Vector3` Allocations in `factorial` Chambers + Dirty-Check Discrete State in `stack_overflow` & `fuel_exhaustion` (`lines 1477–1497, 1650–1689`)**: Eliminates **15–30 `new THREE.Vector3` allocations** and 10–20 Catmull-Rom arc-length binary searches (`getPointAt`/`getTangentAt`) per frame in `factorial`, and skips redundant per-frame loops in `stack_overflow` and `fuel_exhaustion`.
5. **P1 — 64 RGB Genome Spores: Dirty-Check `instanceColor`, Eliminate 64 `new THREE.Color` Allocations on Fault, and Direct `instanceMatrix` Array Write with Angle-Addition LUTs (`lines 1694–1718`)**: Eliminates **64 `new THREE.Color` allocations/frame** on fault steps, stops uploading `instanceColor` to the GPU on every frame when unchanged, and replaces 64 `_dummy.updateMatrix()` Euler/Quaternion compositions + 192 trig calls with 6 trig calls and direct `Float32Array` matrix writes.
6. **P1 — Deduplicate 71 Redundant `THREE.MeshPhysicalMaterial` Instances and 50 Duplicate `BufferGeometry` Allocations in `createQuinelingSubsystem()` (`lines 630–1251`)**: Reduces `MeshPhysicalMaterial` count in Designer 8 from **102 down to 31** (-70%), eliminates 50 duplicate `TorusGeometry`/`CylinderGeometry`/`SphereGeometry` constructions inside loops, and adds missing `THREE.DynamicDrawUsage` flags.
7. **P2 — Eliminate Per-Frame `Object.entries(speciesGroups)` Allocations, `ruleId` String Formatting, Reference-Cached `resolveCompiledQuineling`, and Unused `portraitFrame` Bounds (`lines 243–253, 1314–1345`, `sherlockQuinelingCompiler.js:655–656`)**: Eliminates 9 array allocations/frame from `Object.entries`, per-frame string template allocations, and **220,000–370,000 unused 3D surface evaluations** during preset compilation.

---

## Prioritized Optimizations & Drop-In Code Snippets

---

### P0-1: Replace Per-Frame 16-Tendril Geometry Rebuild with Static Geometry + Y-Axis Rotation (`lines 677–693, 1720–1760`)

#### Problem Analysis
In `update()` at `lines 1722–1760`, all 16 vertical chamber tendrils (`TENDRIL_CURVE_COUNT = 16`, `TENDRIL_SEGMENTS = 10`, 320 vertices) are recomputed from scratch on every frame:
- `rad = lerp(startR, endR, t) + Math.sin(t * Math.PI) * 0.035` and `y = lerp(startY, endY, t)` are **100% constant** for all time.
- The only time-varying quantity in the loop is `angle = (idx / 8) * TAU + ambient * 0.1`, which adds `Δ = ambient * 0.1` to the polar angle `twist` in `(Math.cos(twist) * rad, y, Math.sin(twist) * rad)`.
- Because `writeRibbonStrip` computes horizontal binormals `b = normalize(-tz, 0, tx)` and normals `n = t × b` that commute with rotation around the Y axis, incrementing `twist` by `+ambient * 0.1` is **100% mathematically identical** to computing `tendrilPositions` and `tendrilNormals` **once** at `ambient = 0` inside `createQuinelingSubsystem()` and setting `chamberTendrilsMesh.rotation.y = -ambient * 0.1` in `update()`.
- Furthermore, `tendrilColors` only depends on `(species.primaryHex, species.secondaryHex, isFault, isDone)` (and `tendrilMat` is a `MeshBasicMaterial` that doesn't even use normals).

#### Drop-In Replacement (`lines 677–695` & `lines 1720–1760`)
```javascript
  // In createQuinelingSubsystem() (lines 677–695):
  const tendrilGeom = new THREE.BufferGeometry();
  const tendrilPositions = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  const tendrilColors = new Float32Array(TENDRIL_VERTEX_COUNT * 3);
  const tendrilNormals = new Float32Array(TENDRIL_VERTEX_COUNT * 3);

  // Precompute static tendril positions and normals once at ambient = 0
  const floorRelY = -3.38 - QUINELING_CORE_Y; // -1.33
  const ringsRelY = -0.25 - QUINELING_CORE_Y; // +1.80
  const _initTendrilPts = Array.from({ length: TENDRIL_SEGMENTS }, () => new THREE.Vector3());
  for (let c = 0; c < TENDRIL_CURVE_COUNT; c += 1) {
    const isFloorRoot = c < 8;
    const idx = c % 8;
    const angle = (idx / 8) * TAU;
    const startR = QUINELING_MAX_RADIUS + 0.04;
    const endR = isFloorRoot ? 0.56 : 0.46;
    const startY = isFloorRoot ? -0.26 : 0.26;
    const endY = isFloorRoot ? floorRelY : ringsRelY;
    for (let s = 0; s < TENDRIL_SEGMENTS; s += 1) {
      const t = s / (TENDRIL_SEGMENTS - 1);
      const rad = lerp(startR, endR, t) + Math.sin(t * Math.PI) * 0.035;
      const twist = angle + t * (isFloorRoot ? -0.35 : 0.35);
      _initTendrilPts[s].set(Math.cos(twist) * rad, lerp(startY, endY, t), Math.sin(twist) * rad);
    }
    writeRibbonStrip(
      tendrilPositions,
      tendrilColors,
      tendrilNormals,
      c,
      TENDRIL_SEGMENTS,
      _initTendrilPts,
      0.0036,
      1,
      1,
      1
    );
  }

  const tendrilColorAttr = new THREE.BufferAttribute(tendrilColors, 3);
  tendrilColorAttr.setUsage(THREE.DynamicDrawUsage);
  tendrilGeom.setAttribute('position', new THREE.BufferAttribute(tendrilPositions, 3));
  tendrilGeom.setAttribute('color', tendrilColorAttr);
  tendrilGeom.setAttribute('normal', new THREE.BufferAttribute(tendrilNormals, 3));
  tendrilGeom.setIndex(buildRibbonIndices(TENDRIL_CURVE_COUNT, TENDRIL_SEGMENTS));
```

And in `update()` (`lines 1720–1760`, adding `let lastTendrilColorKey = '';` at line 1296):
```javascript
    // ========================================================================
    // 5. UPDATE VERTICAL CHAMBER TENDRILS (Rigid Y-Rotation + Dirty-Checked Colors)
    // ========================================================================
    chamberTendrilsMesh.rotation.y = -ambient * 0.1;

    const floorHex = isFault ? 0xe11d48 : isDone ? 0x2dd4bf : species.primaryHex;
    const crownHex = isFault ? 0xe11d48 : isDone ? 0x2dd4bf : species.secondaryHex;
    const tendrilColorKey = `${floorHex}|${crownHex}`;
    if (tendrilColorKey !== lastTendrilColorKey) {
      lastTendrilColorKey = tendrilColorKey;
      for (let c = 0; c < TENDRIL_CURVE_COUNT; c += 1) {
        _color.setHex(c < 8 ? floorHex : crownHex);
        const baseVertex = c * TENDRIL_SEGMENTS * 2;
        for (let s = 0; s < TENDRIL_SEGMENTS; s += 1) {
          const glow = 0.72 + 0.28 * Math.sin((s / (TENDRIL_SEGMENTS - 1)) * Math.PI);
          const cr = _color.r * glow;
          const cg = _color.g * glow;
          const cb = _color.b * glow;
          const v0 = (baseVertex + s * 2) * 3;
          const v1 = v0 + 3;
          tendrilColors[v0] = cr;
          tendrilColors[v0 + 1] = cg;
          tendrilColors[v0 + 2] = cb;
          tendrilColors[v1] = cr;
          tendrilColors[v1 + 1] = cg;
          tendrilColors[v1 + 2] = cb;
        }
      }
      tendrilColorAttr.needsUpdate = true;
    }
```

---

### P0-2: Zero-Allocation `embedOnSpeciesSurface`, Step-Cached SDK Ridge Tables, & Static-Species Ribbon Skipping (`lines 424–577, 1375–1460`)

#### Problem Analysis
1. **`embedOnSpeciesSurface(speciesId, u, v, rIdx, sdkDelta)` (`lines 450–577`)** is called **320 times per frame** (256 times for SDK ribbons at `line 1442` + 64 times for spores at `line 1698`), allocating and returning a new `{ x, y, z }` object on every call (**320 heap allocations/frame**).
2. **`sampleSdkRidgePoint(ridge, u)` (`lines 427–442`)** is called **272 times per frame** (16 `midSample` calls at `line 1409` + 256 `rawPt` calls at `line 1438`), allocating a new `{ x, y, z, owner }` object every call (**272 heap allocations/frame**), even though `ridge` and `u` are **100% constant within a step**!
3. **`lastStepRidgeKey` (`line 1375`)** only caches a single step, so scrubbing or looping re-runs `WovenBody.frame` on every step change, and passes `budget: 256` without a `reuse` buffer when the minimum accepted budget is `128` and `points`/`owners` are unused.
4. **4 of the 8 species (`tree_fold`, `lexical_let`, `stack_overflow`, `fuel_exhaustion`)** do not reference `sdkDelta` inside `embedOnSpeciesSurface` (`lines 477–505, 518–529, 552–575`). For these 4 species, `sdkRibbonGeom` (`position`, `normal`, `color`) is **100% invariant across frames within the same step**, yet is rebuilt and re-uploaded to the GPU on every frame!
5. **`artifact.graph.nodes.findIndex((n) => n.id === activeNodeId)` (`line 1402`)** runs a linear array scan every frame.

#### Drop-In Replacement (`lines 424–577` & `lines 1375–1461`)
```javascript
// Reusable scratch point to eliminate 320 object allocations/frame in embedOnSpeciesSurface
const _surfOut = { x: 0, y: 0, z: 0 };

// Precomputed 16-segment Nautilus LUT for u = s / 15 in factorial SDK ribbons
const NAUTILUS_U16_COS = new Float32Array(SDK_RIBBON_SEGMENTS);
const NAUTILUS_U16_SIN = new Float32Array(SDK_RIBBON_SEGMENTS);
const NAUTILUS_U16_RAD = new Float32Array(SDK_RIBBON_SEGMENTS);
const NAUTILUS_U16_TUBER = new Float32Array(SDK_RIBBON_SEGMENTS);
for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
  const u = s / (SDK_RIBBON_SEGMENTS - 1);
  const theta = u * 2.05 * TAU;
  const rad = 0.022 * Math.exp(0.215 * theta);
  NAUTILUS_U16_COS[s] = Math.cos(theta);
  NAUTILUS_U16_SIN[s] = Math.sin(theta);
  NAUTILUS_U16_RAD[s] = rad;
  NAUTILUS_U16_TUBER[s] = Math.max(0.014, rad * 0.36) + 0.004;
}

const STATIC_RIBBON_SPECIES = new Set([
  'tree_fold',
  'lexical_let',
  'stack_overflow',
  'fuel_exhaustion',
]);

const _sdkFrameReuse = {
  points: new Float32Array(128 * 4),
  owners: new Uint16Array(128),
};

/**
 * Packs the 16 SDK ribbons' owner indices and 16x16 (x+y+z)*0.15 wave offsets once per step,
 * cached per-artifact by stepIndex so scrubbing/looping never recomputes WovenBody.frame.
 */
function getPackedStepRidges(artifact, stepIndex) {
  if (!artifact._stepRidgeCache) {
    artifact._stepRidgeCache = new Map();
  }
  let packed = artifact._stepRidgeCache.get(stepIndex);
  if (packed) return packed;

  const stepPhaseRad = (stepIndex * 0.08) * TAU;
  const wf = WovenBody.frame(artifact.wovenBody, stepPhaseRad, {
    budget: 128,
    crests: true,
    reuse: _sdkFrameReuse,
  });
  let ridges = wf.ridges;
  if (!ridges || !ridges.length) {
    const mf = Morphology.surfaceFrame(artifact.shape, stepPhaseRad, true);
    ridges = mf.ridges || [];
  }

  const owners = new Int16Array(SDK_RIBBON_COUNT);
  const rawSums = new Float32Array(SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS);
  const hasRaw = new Uint8Array(SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS);
  const nodeColorLen = Math.max(1, (artifact.nodeColors || []).length);

  for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
    const ridgeIndex = Math.floor((rIdx * Math.max(1, ridges.length)) / SDK_RIBBON_COUNT);
    const ridge = ridges[ridgeIndex];
    const midSample = sampleSdkRidgePoint(ridge, 0.5);
    owners[rIdx] =
      midSample?.owner !== null && midSample?.owner !== undefined && midSample.owner >= 0
        ? midSample.owner
        : rIdx % nodeColorLen;

    const baseOffset = rIdx * SDK_RIBBON_SEGMENTS;
    for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
      const u = s / (SDK_RIBBON_SEGMENTS - 1);
      const rawPt = sampleSdkRidgePoint(ridge, u);
      if (rawPt) {
        hasRaw[baseOffset + s] = 1;
        rawSums[baseOffset + s] = (rawPt.x + rawPt.y + rawPt.z) * 0.15;
      }
    }
  }

  packed = { owners, rawSums, hasRaw };
  if (artifact._stepRidgeCache.size >= 64) {
    artifact._stepRidgeCache.delete(artifact._stepRidgeCache.keys().next().value);
  }
  artifact._stepRidgeCache.set(stepIndex, packed);
  return packed;
}
```

And in `embedOnSpeciesSurface` (`lines 450–577`), write into `out = _surfOut` instead of allocating `{ x, y, z }` on every call:
```javascript
function embedOnSpeciesSurface(speciesId, u, v, rIdx = 0, sdkDelta = 0, out = _surfOut) {
  switch (speciesId) {
    case 'factorial': {
      const theta = u * 2.05 * TAU;
      const rad = 0.022 * Math.exp(0.215 * theta);
      const tubeR = Math.max(0.014, rad * 0.36) + 0.004;
      const roll = (v * 0.5 + 0.5) * TAU + sdkDelta * 0.4;
      const R = rad + tubeR * Math.cos(roll);
      out.x = Math.cos(theta) * R;
      out.y = Math.sin(theta) * R * 0.78 + 0.02;
      out.z = Math.sin(theta) * rad * 0.22 + tubeR * Math.sin(roll);
      return out;
    }
    // ... same in-place assignment `out.x = ...; out.y = ...; out.z = ...; return out;` for the other 7 cases
```

And in `update()` (`lines 1375–1461`, adding `let lastStaticRibbonStateKey = '';`):
```javascript
    const packedRidges = getPackedStepRidges(artifact, stepIndex);
    const nodeColors = artifact.nodeColors || [];
    const nodeColorLen = Math.max(1, nodeColors.length);

    if (!artifact._nodeIdToIndex) {
      artifact._nodeIdToIndex = new Map(artifact.graph.nodes.map((n, idx) => [n.id, idx]));
    }
    const activeExprId =
      step?.after?.control?.exprId ??
      step?.before?.control?.exprId ??
      null;
    const activeNodeId =
      activeExprId !== null && artifact.exprToNodeId?.has(activeExprId)
        ? artifact.exprToNodeId.get(activeExprId)
        : null;
    const activeNodeIdx =
      activeNodeId !== null && artifact._nodeIdToIndex.has(activeNodeId)
        ? artifact._nodeIdToIndex.get(activeNodeId)
        : stepIndex % Math.max(1, artifact.graph.nodes.length);

    const isStaticRibbonSpecies = STATIC_RIBBON_SPECIES.has(speciesId);
    const staticRibbonKey = isStaticRibbonSpecies
      ? `${artifact.cacheKey}|${stepIndex}|${activeNodeIdx}|${isFault ? 1 : 0}|${isExhausted ? 1 : 0}`
      : '';

    if (!isStaticRibbonSpecies || staticRibbonKey !== lastStaticRibbonStateKey) {
      lastStaticRibbonStateKey = staticRibbonKey;
      const { owners, rawSums, hasRaw } = packedRidges;

      for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
        const vCoord = ((rIdx % 8) / 7) * 2 - 1;
        const ownerIdx = owners[rIdx];
        const cSpec = nodeColors[ownerIdx % nodeColorLen];
        const rgb = cSpec?.rgb || [56, 189, 248];
        let cr = rgb[0] / 255;
        let cg = rgb[1] / 255;
        let cb = rgb[2] / 255;

        if (ownerIdx === activeNodeIdx) {
          cr = lerp(cr, 1.0, 0.45);
          cg = lerp(cg, 0.92, 0.45);
          cb = lerp(cb, 0.55, 0.45);
        }
        if (isFault) {
          cr = lerp(cr, 0.95, 0.7);
          cg *= 0.25;
          cb *= 0.35;
        } else if (isExhausted) {
          const gray = (cr + cg + cb) / 3;
          cr = lerp(cr, gray * 0.55, 0.75);
          cg = lerp(cg, gray * 0.6, 0.75);
          cb = lerp(cb, gray * 0.68, 0.75);
        }

        const baseOffset = rIdx * SDK_RIBBON_SEGMENTS;
        for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
          const u = s / (SDK_RIBBON_SEGMENTS - 1);
          let sdkWave = 0;
          if (!isStaticRibbonSpecies) {
            const ambWave = Math.sin(ambient * 1.6 + u * TAU) * 0.04;
            sdkWave = hasRaw[baseOffset + s]
              ? clamp(rawSums[baseOffset + s] + ambWave, -0.35, 0.35)
              : ambWave;
          }
          embedOnSpeciesSurface(speciesId, u, vCoord, rIdx, sdkWave, _sdkPts[s]);
        }
        writeRibbonStrip(
          sdkRibbonPositions,
          sdkRibbonColors,
          sdkRibbonNormals,
          rIdx,
          SDK_RIBBON_SEGMENTS,
          _sdkPts,
          ownerIdx === activeNodeIdx ? 0.0055 : 0.0032,
          cr,
          cg,
          cb
        );
      }
      sdkRibbonGeom.attributes.position.needsUpdate = true;
      sdkRibbonGeom.attributes.color.needsUpdate = true;
      sdkRibbonGeom.attributes.normal.needsUpdate = true;
    }
```

---

### P0-3: Optimize `writeRibbonStrip` with Precomputed Trig LUTs & Simplified Cross-Product (`lines 332–393`)

#### Problem Analysis
In `writeRibbonStrip` (`lines 332–393`):
1. `Math.sin((s / Math.max(1, segments - 1)) * Math.PI)` is computed **twice** per segment (`lines 364` and `385`) on every call, for fixed `segments` counts (`10`, `16`, `28`).
2. `Math.hypot(tx, ty, tz)` and `Math.hypot(bx, by, bz)` (`lines 342, 350`) are used instead of `Math.sqrt`, and `by` is **identically `0`** (`line 348: let by = 0;`), which reduces `Math.hypot(bx, by, bz)` to `Math.sqrt(bx * bx + bz * bz)` and simplifies the normal cross-product (`lines 360–362`) from 6 multiplies + 3 subtracts to `nx = ty * bz`, `ny = tz * bx - tx * bz`, `nz = -ty * bx` (and `p.y - by * w` / `p.y + by * w` to just `p.y`).

#### Drop-In Replacement (`lines 332–393`)
```javascript
const _ribbonLutCache = new Map();
function getRibbonSegmentLut(segments) {
  let lut = _ribbonLutCache.get(segments);
  if (lut) return lut;
  const taper = new Float32Array(segments);
  const glow = new Float32Array(segments);
  const denom = Math.max(1, segments - 1);
  for (let s = 0; s < segments; s += 1) {
    const sinVal = Math.sin((s / denom) * Math.PI);
    taper[s] = sinVal * 0.65 + 0.35;
    glow[s] = 0.72 + 0.28 * sinVal;
  }
  lut = { taper, glow };
  _ribbonLutCache.set(segments, lut);
  return lut;
}

function writeRibbonStrip(positions, colors, normals, curveIndex, segments, pointsArray, halfWidth, r, g, b) {
  const baseVertex = curveIndex * segments * 2;
  const { taper: taperLut, glow: glowLut } = getRibbonSegmentLut(segments);
  const lastIdx = segments - 1;

  for (let s = 0; s < segments; s += 1) {
    const p = pointsArray[s];
    const pNext = pointsArray[s < lastIdx ? s + 1 : lastIdx];
    const pPrev = pointsArray[s > 0 ? s - 1 : 0];

    let tx = pNext.x - pPrev.x;
    let ty = pNext.y - pPrev.y;
    let tz = pNext.z - pPrev.z;
    const tLen = Math.sqrt(tx * tx + ty * ty + tz * tz) || 1;
    const invTLen = 1 / tLen;
    tx *= invTLen;
    ty *= invTLen;
    tz *= invTLen;

    let bx = -tz;
    let bz = tx;
    const bLen = Math.sqrt(bx * bx + bz * bz);
    if (bLen < 1e-4) {
      bx = 1;
      bz = 0;
    } else {
      const invBLen = 1 / bLen;
      bx *= invBLen;
      bz *= invBLen;
    }

    // Since by === 0, cross product (t x b) simplifies directly:
    const nx = ty * bz;
    const ny = tz * bx - tx * bz;
    const nz = -ty * bx;

    const w = halfWidth * taperLut[s];
    const bwX = bx * w;
    const bwZ = bz * w;

    const v0 = (baseVertex + s * 2) * 3;
    const v1 = v0 + 3;

    positions[v0] = p.x - bwX;
    positions[v0 + 1] = p.y;
    positions[v0 + 2] = p.z - bwZ;

    positions[v1] = p.x + bwX;
    positions[v1 + 1] = p.y;
    positions[v1 + 2] = p.z + bwZ;

    normals[v0] = nx;
    normals[v0 + 1] = ny;
    normals[v0 + 2] = nz;
    normals[v1] = nx;
    normals[v1 + 1] = ny;
    normals[v1 + 2] = nz;

    if (colors) {
      const glow = glowLut[s];
      const cr = r * glow;
      const cg = g * glow;
      const cb = b * glow;
      colors[v0] = cr;
      colors[v0 + 1] = cg;
      colors[v0 + 2] = cb;
      colors[v1] = cr;
      colors[v1 + 1] = cg;
      colors[v1 + 2] = cb;
    }
  }
}
```

---

### P1-1: Cache Static `factorial` Chamber Transforms & Dirty-Check Discrete Species States (`lines 1477–1497, 1650–1689`)

#### Problem Analysis
1. **`factorial` (`lines 1477–1497`)**: Inside `update()`, for every active chamber `c`, `u = 0.22 + (c / Math.max(1, chamberCount - 1)) * 0.72` depends **only** on `chamberCount`. Yet every frame it calls `nautilusSpline.getPointAt(u)` (allocates `new THREE.Vector3` + binary-searches arc length), `nautilusSpline.getTangentAt(u)` (allocates `new THREE.Vector3` + binary-searches arc length), and `new THREE.Vector3(0, 1, 0)` (allocates a third `THREE.Vector3`), plus `quaternion.setFromUnitVectors(...)` and `scale.set(tubeR, 1.0, tubeR)`. Caching `lastNautilusChamberCount` eliminates **15–30 `Vector3` allocations and 10–20 spline evaluations per frame**.
2. **`stack_overflow` (`lines 1650–1674`)**: Every property updated in the `stack_overflow` branch (`ringCount`, `crushRatio`, `cometLanceSub.position.y`, `barrierMat`, and all 10 `cometShockRings`) depends **only** on `(ringCount, frameCount, isFault)` — zero dependence on `ambient` or `q`. Dirty-checking `cometStateKey` skips the entire loop on 95%+ of frames.
3. **`fuel_exhaustion` (`lines 1681–1688`)**: The 14 `torusFuelCells` material mutations only depend on `litCells` (`0..14`) and `isExhausted`. Dirty-checking `fuelCellStateKey` avoids 56 material mutations per frame.

#### Drop-In Replacement (`lines 1477–1497` & `lines 1650–1689`)
```javascript
      // In factorial (lines 1477–1497, with `let lastNautilusChamberCount = -1;` at line 1296):
      if (chamberCount !== lastNautilusChamberCount) {
        lastNautilusChamberCount = chamberCount;
        for (let c = 0; c < MAX_NAUTILUS_CHAMBERS; c += 1) {
          const ch = nautilusChambers[c];
          if (c >= chamberCount) {
            ch.mesh.visible = false;
            continue;
          }
          ch.mesh.visible = true;
          const u = 0.22 + (c / Math.max(1, chamberCount - 1)) * 0.72;
          const theta = u * 2.05 * TAU;
          const rad = 0.022 * Math.exp(0.215 * theta);
          const tubeR = Math.max(0.016, rad * 0.36);
          const pt = nautilusSpline.getPointAt(u);
          const tangent = nautilusSpline.getTangentAt(u);
          ch.mesh.position.copy(pt);
          ch.mesh.quaternion.setFromUnitVectors(_upAxisY, tangent);
          ch.mesh.scale.set(tubeR, 1.0, tubeR);
        }
      }
      const isRet = ruleLine === 149 || controlKindIsReturn(step);
      for (let c = 0; c < chamberCount; c += 1) {
        const ch = nautilusChambers[c];
        const activeChamber = c < callDepth || isDone;
        const lockedGold = c < frameCount && isRet;
        ch.mat.color.setHex(isDone ? 0x2dd4bf : lockedGold ? 0xfde047 : activeChamber ? 0xf59e0b : 0x475569);
        ch.mat.emissiveIntensity = activeChamber ? 0.68 : 0.18;
      }
```

```javascript
    // In stack_overflow & fuel_exhaustion (lines 1650–1689, with `let lastCometKey = ''; let lastFuelCellKey = -2;`):
    } else if (speciesId === 'stack_overflow') {
      const ringCount = clamp(artifact.analysis?.stackLimit ?? 5, 2, MAX_COMET_RINGS);
      const cometKey = `${ringCount}|${frameCount}|${isFault ? 1 : 0}`;
      if (cometKey !== lastCometKey) {
        lastCometKey = cometKey;
        const crushRatio = clamp(frameCount / ringCount, 0.15, 1.0);
        cometLanceSub.position.y = lerp(-0.08, 0.08, crushRatio);
        barrierMat.opacity = isFault ? 0.84 : 0.36 + crushRatio * 0.32;
        barrierMat.emissiveIntensity = isFault ? 0.95 : 0.48 + crushRatio * 0.3;

        for (let rIdx = 0; rIdx < MAX_COMET_RINGS; rIdx += 1) {
          const sr = cometShockRings[rIdx];
          if (rIdx >= ringCount) {
            sr.mesh.visible = false;
            continue;
          }
          sr.mesh.visible = true;
          const u = rIdx / Math.max(1, ringCount - 1);
          const yBase = lerp(-0.22, 0.28, u);
          const activeRing = rIdx < frameCount || isFault;
          const targetY = lerp(yBase, 0.32 - (ringCount - 1 - rIdx) * 0.032, crushRatio * 0.65);
          const scaleR = lerp(0.65, 1.45, u) * (activeRing ? 1.0 : 0.82);
          sr.mesh.position.y = targetY;
          sr.mesh.scale.set(scaleR, scaleR, 1.0 - u * 0.35);
          sr.mat.color.setHex(isFault ? 0xe11d48 : activeRing ? 0xf97316 : 0x64748b);
          sr.mat.emissiveIntensity = isFault ? 0.85 : activeRing ? 0.62 : 0.18;
        }
      }
    } else if (speciesId === 'fuel_exhaustion') {
      const litCells = Math.round(liveFuelRatio * 14);
      torusBraidMesh.rotation.z = ambient * (0.15 + liveFuelRatio * 0.85);
      torusBraidMat.color.setHex(isExhausted ? 0x475569 : 0xfbbf24);
      torusBraidMat.emissiveIntensity = isExhausted ? 0.12 : 0.24 + liveFuelRatio * 0.48;

      const fuelCellKey = isExhausted ? -1 : litCells;
      if (fuelCellKey !== lastFuelCellKey) {
        lastFuelCellKey = fuelCellKey;
        for (let fIdx = 0; fIdx < torusFuelCells.length; fIdx += 1) {
          const cell = torusFuelCells[fIdx];
          const hasFuel = fIdx < litCells && !isExhausted;
          cell.mat.color.setHex(hasFuel ? 0x38bdf8 : 0x334155);
          cell.mat.emissive.setHex(hasFuel ? 0x0284c7 : 0x0f172a);
          cell.mat.emissiveIntensity = hasFuel ? 0.58 : 0.08;
          cell.mat.opacity = hasFuel ? 0.92 : 0.42;
        }
      }
    }
```

---

### P1-2: Optimize 64 RGB Genome Spores Loop — Dirty-Check `instanceColor`, Eliminate 64 `new THREE.Color` Allocations, and Write `instanceMatrix` Directly (`lines 1694–1718`)

#### Problem Analysis
1. **Line 1713**: `if (isFault) _color.lerp(new THREE.Color(0xe11d48), 0.7);` allocates **64 `new THREE.Color` instances per frame** whenever `isFault` is true.
2. **Lines 1707–1717**: Spore colors only depend on `artifact.cacheKey` and `isFault` — they do not vary with `ambient` or `stepIndex`. Recomputing all 64 colors and setting `sporeMesh.instanceColor.needsUpdate = true` uploads the color buffer to the GPU on every single frame unnecessarily.
3. **Lines 1697–1705**: For each of the 64 spores, `Math.sin(s * PHI_GOLDEN_ANGLE + ambient * 0.8)` and `_dummy.rotation.set(ambient + s, ambient * 0.7 + s, 0); _dummy.updateMatrix(); sporeMesh.setMatrixAt(s, _dummy.matrix);` executes Euler-to-Quaternion half-angle trig and Quaternion-to-Matrix4 composition 64 times per frame. Using a 64-entry precomputed LUT (`SPORE_COS_S`, `SPORE_SIN_S`, `SPORE_COS_PHI`, `SPORE_SIN_PHI`) with angle-addition identities and writing the `'XYZ'` (`rz = 0`) rotation + scale + translation directly into `sporeMesh.instanceMatrix.array` replaces 192 trig calls + 64 quaternion/matrix compositions with just **6 trig calls per frame**.

#### Drop-In Replacement (`lines 1694–1718`)
```javascript
// Precomputed once at module load (or in createQuinelingSubsystem):
const _faultCrimsonColor = new THREE.Color(0xe11d48);
const SPORE_COS_S = new Float32Array(SPORE_COUNT);
const SPORE_SIN_S = new Float32Array(SPORE_COUNT);
const SPORE_COS_PHI = new Float32Array(SPORE_COUNT);
const SPORE_SIN_PHI = new Float32Array(SPORE_COUNT);
for (let s = 0; s < SPORE_COUNT; s += 1) {
  SPORE_COS_S[s] = Math.cos(s);
  SPORE_SIN_S[s] = Math.sin(s);
  SPORE_COS_PHI[s] = Math.cos(s * PHI_GOLDEN_ANGLE);
  SPORE_SIN_PHI[s] = Math.sin(s * PHI_GOLDEN_ANGLE);
}

// Inside update() (lines 1694–1718, with `let lastSporeColorKey = '';`):
    const outScale = isExhausted ? 0.88 : 1.08;
    const sporeScale = isDone ? 1.15 : isExhausted ? 0.5 : 0.85;
    const uOffset = ambient * 0.04 * (0.4 + liveFuelRatio);
    const cosAmb = Math.cos(ambient);
    const sinAmb = Math.sin(ambient);
    const cosAmb07 = Math.cos(ambient * 0.7);
    const sinAmb07 = Math.sin(ambient * 0.7);
    const cosAmb08 = Math.cos(ambient * 0.8);
    const sinAmb08 = Math.sin(ambient * 0.8);
    const instanceMatArr = sporeMesh.instanceMatrix.array;

    for (let s = 0; s < SPORE_COUNT; s += 1) {
      const u = ((s / SPORE_COUNT) + uOffset) % 1;
      const v = SPORE_SIN_PHI[s] * cosAmb08 + SPORE_COS_PHI[s] * sinAmb08;
      const surf = embedOnSpeciesSurface(speciesId, u, v, s % 16, 0.03, _surfOut);

      const cs = SPORE_COS_S[s];
      const ss = SPORE_SIN_S[s];
      // a = cos(ambient + s), b = sin(ambient + s)
      const a = cosAmb * cs - sinAmb * ss;
      const b = sinAmb * cs + cosAmb * ss;
      // c = cos(ambient * 0.7 + s), d = sin(ambient * 0.7 + s)
      const c = cosAmb07 * cs - sinAmb07 * ss;
      const d = sinAmb07 * cs + cosAmb07 * ss;

      const idx16 = s * 16;
      instanceMatArr[idx16] = c * sporeScale;
      instanceMatArr[idx16 + 1] = 0;
      instanceMatArr[idx16 + 2] = -d * sporeScale;
      instanceMatArr[idx16 + 3] = 0;
      instanceMatArr[idx16 + 4] = b * d * sporeScale;
      instanceMatArr[idx16 + 5] = a * sporeScale;
      instanceMatArr[idx16 + 6] = b * c * sporeScale;
      instanceMatArr[idx16 + 7] = 0;
      instanceMatArr[idx16 + 8] = a * d * sporeScale;
      instanceMatArr[idx16 + 9] = -b * sporeScale;
      instanceMatArr[idx16 + 10] = a * c * sporeScale;
      instanceMatArr[idx16 + 11] = 0;
      instanceMatArr[idx16 + 12] = surf.x * outScale;
      instanceMatArr[idx16 + 13] = surf.y * outScale;
      instanceMatArr[idx16 + 14] = surf.z * outScale;
      instanceMatArr[idx16 + 15] = 1;
    }
    sporeMesh.instanceMatrix.needsUpdate = true;

    const sporeColorKey = `${artifact.cacheKey}|${isFault ? 1 : 0}`;
    if (sporeColorKey !== lastSporeColorKey) {
      lastSporeColorKey = sporeColorKey;
      const rgbPixels = artifact.rgbPixels || [];
      const pxLen = Math.max(1, rgbPixels.length);
      for (let s = 0; s < SPORE_COUNT; s += 1) {
        const px = rgbPixels[s % pxLen];
        if (px && px.length >= 3) {
          _color.setRGB(px[0] / 255, px[1] / 255, px[2] / 255);
        } else {
          _color.setHex(species.primaryHex);
        }
        if (isFault) _color.lerp(_faultCrimsonColor, 0.7);
        sporeMesh.setColorAt(s, _color);
      }
      if (sporeMesh.instanceColor) sporeMesh.instanceColor.needsUpdate = true;
    }
```

---

### P1-3: Deduplicate 50 Loop-Allocated `BufferGeometry` Instances, 71 Redundant `MeshPhysicalMaterial` Instances, & Add `DynamicDrawUsage` (`lines 630–1251`)

#### Problem Analysis
1. **Duplicate `BufferGeometry` allocations inside loops**:
   - `nautilusRibs` (`line 753`): Allocates 20 separate `new THREE.TorusGeometry(tubeR, tubeR * 0.22, 10, 22)` instances. Since minor radius is `0.22 * tubeR`, all 20 ribs are exact uniform scales of a single shared `new THREE.TorusGeometry(1, 0.22, 10, 22)` with `item.mesh.scale.setScalar(item.tubeR * pulse)` in `update()` (`line 1473`). (**Saves 19 geometries**)
   - `nautilusChambers` (`line 773`): Calls `new THREE.CylinderGeometry(1, 0.9, 0.012, 20)` 10 times inside the loop. (**Saves 9 geometries**)
   - `listZooidPods` (`lines 848, 858`): Calls `new THREE.SphereGeometry(1, 24, 18)` 6 times and `new THREE.TorusGeometry(1.15, 0.11, 10, 28)` 6 times inside the loop. (**Saves 10 geometries**)
   - `coralPolypCrowns` (`line 919`): Calls `new THREE.SphereGeometry(0.044, 16, 12)` 4 times inside the loop. (**Saves 3 geometries**)
   - `cometShockRings` (`line 1217`): Calls `new THREE.TorusGeometry(0.22, 0.013, 10, 32)` 10 times inside the loop. (**Saves 9 geometries**)
2. **Redundant `MeshPhysicalMaterial` instances**:
   - `lotusPetals` (`lines 1064–1085` & `1604–1609`): Allocates **24 `MeshPhysicalMaterial` instances** (`pMat`), even though all 8 petals in each whorl (`ring = 0, 1, 2`) have 100% identical creation parameters (`cubeColors[ring]`) and 100% identical per-frame updates (`openMix` depends only on `lp.ring`). Sharing 3 materials (`lotusRingMats[0..2]`) eliminates **21 `MeshPhysicalMaterial` instances** and reduces 24 material mutations/frame to 3.
   - `listZooidPods` `ringMat` (`lines 851–857`): Allocates **6 identical `MeshPhysicalMaterial` instances** that are never modified in `update()`. Sharing 1 `zooidRingMat` eliminates **5 `MeshPhysicalMaterial` instances**.
   - `nautilusRibs` (`line 752`), `torusFuelCells` (`line 1237`), `cometShockRings` (`line 1210`), `nautilusChambers` (`line 764`): Sharing state-pooled materials across identical states eliminates another **45 `MeshPhysicalMaterial` instances**.
3. **Missing `THREE.DynamicDrawUsage`**:
   - Add `.setUsage(THREE.DynamicDrawUsage)` to the dynamic `BufferAttribute`s on `sdkRibbonGeom` (`lines 630–632`), `silkGeom` (`lines 812–814`), `jellyTentacleGeom` (`lines 1131–1133`), and `sporeMesh.instanceColor` (`line 670`).

#### Drop-In Replacement Snippets
```javascript
  // 1. DynamicDrawUsage on sdkRibbonGeom (lines 630–632):
  sdkRibbonGeom.setAttribute('position', new THREE.BufferAttribute(sdkRibbonPositions, 3).setUsage(THREE.DynamicDrawUsage));
  sdkRibbonGeom.setAttribute('color', new THREE.BufferAttribute(sdkRibbonColors, 3).setUsage(THREE.DynamicDrawUsage));
  sdkRibbonGeom.setAttribute('normal', new THREE.BufferAttribute(sdkRibbonNormals, 3).setUsage(THREE.DynamicDrawUsage));

  // 2. Shared unit TorusGeometry & CylinderGeometry in Nautilus (lines 744–776):
  const nautilusUnitRibGeom = new THREE.TorusGeometry(1, 0.22, 10, 22);
  const _zAxis = new THREE.Vector3(0, 0, 1);
  const nautilusRibs = [];
  for (let i = 0; i < 20; i += 1) {
    const u = (i + 1) / 20;
    const theta = u * 2.05 * TAU;
    const rad = 0.022 * Math.exp(0.215 * theta);
    const tubeR = Math.max(0.015, rad * 0.38);
    const pt = nautilusSpline.getPointAt(u);
    const tangent = nautilusSpline.getTangentAt(u);
    const ribMat = nautilusShellMat.clone();
    const rib = new THREE.Mesh(nautilusUnitRibGeom, ribMat);
    rib.position.copy(pt);
    rib.scale.setScalar(tubeR);
    rib.quaternion.setFromUnitVectors(_zAxis, tangent);
    nautilusGroup.add(rib);
    nautilusRibs.push({ mesh: rib, mat: ribMat, u, tubeR });
  }
  // (And in update() line 1473: `item.mesh.scale.setScalar(item.tubeR * pulse);`)

  const nautilusSepGeom = new THREE.CylinderGeometry(1, 0.9, 0.012, 20);
  const nautilusChambers = [];
  for (let c = 0; c < MAX_NAUTILUS_CHAMBERS; c += 1) {
    // ... reuse nautilusSepGeom instead of new THREE.CylinderGeometry(...)
    const sep = new THREE.Mesh(nautilusSepGeom, sepMat);
    nautilusGroup.add(sep);
    nautilusChambers.push({ mesh: sep, mat: sepMat, index: c });
  }

  // 3. Shared Geometries & Ring Material in listZooidPods (lines 831–863):
  const zooidBellGeom = new THREE.SphereGeometry(1, 24, 18);
  const zooidRingGeom = new THREE.TorusGeometry(1.15, 0.11, 10, 28);
  const zooidRingMat = new THREE.MeshPhysicalMaterial({
    color: 0xc084fc,
    emissive: new THREE.Color(0x9333ea),
    emissiveIntensity: 0.52,
    roughness: 0.25,
    metalness: 0.12,
  });

  // 4. Shared 3 Whorl Materials in lotusPetals (lines 1061–1085):
  const lotusPetals = [];
  const petalConeGeom = new THREE.ConeGeometry(0.044, 0.18, 6);
  petalConeGeom.scale(1.3, 1.0, 0.32);
  const lotusRingMats = cubeColors.map(
    (col) =>
      new THREE.MeshPhysicalMaterial({
        color: col,
        emissive: new THREE.Color(col),
        emissiveIntensity: 0.35,
        roughness: 0.28,
        metalness: 0.06,
        clearcoat: 0.4,
        transparent: true,
        opacity: 0.88,
      })
  );
  for (let p = 0; p < 24; p += 1) {
    const angle = p * PHI_GOLDEN_ANGLE;
    const ring = Math.floor(p / 8);
    const r = 0.11 + ring * 0.068;
    const pMat = lotusRingMats[ring];
    const pMesh = new THREE.Mesh(petalConeGeom, pMat);
    pMesh.position.set(Math.cos(angle) * r, -0.05 + ring * 0.032, Math.sin(angle) * r);
    pMesh.rotation.order = 'YXZ';
    pMesh.rotation.y = -angle;
    pMesh.rotation.z = -0.58 - ring * 0.16;
    seedLotusGroup.add(pMesh);
    lotusPetals.push({ mesh: pMesh, mat: pMat, ring, angle, r });
  }
```

---

### P2-1: Eliminate Per-Frame `Object.entries(speciesGroups)` Allocations, `ruleId` String Padding, Reference-Cached `resolveCompiledQuineling`, and Unused `portraitFrame` Bounds (`lines 243–253, 1314–1345`, `sherlockQuinelingCompiler.js:655–656`)

#### Problem Analysis
1. **`Object.entries(speciesGroups)` (`lines 1320–1322`)** allocates 9 arrays on **every frame** just to set `sGroup.visible = key === speciesId`, even when `speciesId` has not changed.
2. **`const ruleId = step?.ruleId || \`A\${String(ruleLine).padStart(3, '0')}\`;` (`line 1345`)** allocates padded strings every frame when `step?.ruleId` is undefined, even though `ruleLine === 80` and `ruleLine === 84` are already checked on `lines 1355, 1357`.
3. **`resolveCompiledQuineling(stepState)` (`lines 243–253`)** builds a multi-hundred-byte `cacheKey` string and hashes it in `compiledQuinelingCache` every frame; caching the last `(source, trace)` references short-circuits in two pointer comparisons.
4. **Unused `WovenBody.portraitFrame(wovenBody)` and `Morphology.portraitFrame(shape)` (`src/engine/sherlockQuinelingCompiler.js:655–656`)**: Neither `artifact.wovenBounds` nor `artifact.morphBounds` is read anywhere in the entire repository, yet computing them eagerly during `compileSherlockProgramToQuineling` evaluates **220,000 to 370,000 3D points** per preset compilation. Converting them to lazy getters on `artifact` eliminates 100% of that cost.

#### Drop-In Replacement (`lines 243–253` & `lines 1319–1346`)
```javascript
// Fast-path reference cache in resolveCompiledQuineling (lines 243–253):
let _lastSourceRef = null;
let _lastTraceRef = null;
let _lastArtifactRef = null;

export function resolveCompiledQuineling(stepState = {}) {
  const trace = stepState.trace || null;
  const source =
    stepState.source ||
    trace?.source ||
    'language "moriarty-sherlock-draft/1"; program Factorial { fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(4); }';
  if (source === _lastSourceRef && trace === _lastTraceRef && _lastArtifactRef) {
    return _lastArtifactRef;
  }
  const artifact = compileSherlockProgramToQuineling(source, QUINELING_SPECIES, trace);
  _lastSourceRef = source;
  _lastTraceRef = trace;
  _lastArtifactRef = artifact;
  return artifact;
}
```

```javascript
    // In update() (lines 1319–1346, replacing Object.entries(speciesGroups) and ruleId string padding):
    if (speciesId !== lastSpeciesParent) {
      lastSpeciesParent = speciesId;
      for (let i = 0; i < speciesGroupEntries.length; i += 1) {
        const entry = speciesGroupEntries[i];
        entry[1].visible = entry[0] === speciesId;
      }
    }
    // ...
    const ruleLine = step?.ruleLine ?? 80;
    const ruleId = step?.ruleId || '';
```

```javascript
  // In src/engine/sherlockQuinelingCompiler.js (lines 655–656 & 698–699):
  // Replace eager WovenBody.portraitFrame(wovenBody) and Morphology.portraitFrame(shape) with lazy getters:
  let _wovenBounds = null;
  let _morphBounds = null;
  // Inside artifact object literal:
  get wovenBounds() {
    return _wovenBounds || (_wovenBounds = WovenBody.portraitFrame(wovenBody));
  },
  get morphBounds() {
    return _morphBounds || (_morphBounds = Morphology.portraitFrame(shape));
  },
```

---

## Summary Table of Audited Gains

| Priority | Target Location | Bottleneck Eliminated | Per-Frame / Startup Savings |
| :--- | :--- | :--- | :--- |
| **P0-1** | `lines 677–695, 1720–1760` | Per-frame 16-tendril spline & ribbon rebuild | **-800 trig calls/frame, -320 `hypot`/frame, -3 GPU buffer uploads/frame** |
| **P0-2** | `lines 424–577, 1375–1460` | `embedOnSpeciesSurface` & `sampleSdkRidgePoint` heap churn + static species rebuilds | **-592 object allocations/frame, 0 per-frame ribbon rebuilds on 4/8 species** |
| **P0-3** | `lines 332–393` | Duplicate `Math.sin` & `Math.hypot` inside `writeRibbonStrip` | **-1,152 `Math.sin` calls/frame, -640 `Math.hypot` calls/frame** |
| **P1-1** | `lines 1477–1497, 1650–1689` | Per-frame `nautilusSpline.getPointAt/getTangentAt` + `stack_overflow`/`fuel_exhaustion` loops | **-15 to -30 `Vector3` allocations/frame, -20 spline binary searches/frame** |
| **P1-2** | `lines 1694–1718` | Per-frame 64-spore `instanceColor` upload, `new THREE.Color` on fault, `_dummy.updateMatrix()` | **-64 `Color` allocations/frame, -186 trig calls/frame, -1 GPU buffer upload/frame** |
| **P1-3** | `lines 630–1251` | Loop-duplicated `BufferGeometry` & `MeshPhysicalMaterial` instances | **-71 `MeshPhysicalMaterial`s (-70%), -50 `BufferGeometry`s** |
| **P2-1** | `lines 243–253, 1319–1346`, `compiler:655` | `Object.entries`, `padStart`, `cacheKey` string build, unused `portraitFrame` | **-9 array allocations/frame, -220k to -370k 3D surface evals per compile** |
