# Designer 8 Quineling — Performance Audit (visual quality held at 100%)

Auditor: Grok 4.7 xhigh Performance Auditor #7 (Designer 8 Quineling 3D Morphogenesis)
File: `src/designers/designer8_quineling.js` (1773 lines)
Scope: cache procedural geometries and buffers, stop per-frame geometry rebuilds and allocations, typed-array math for woven ribbons, dirty-checks inside `update()`. No source file was modified by this audit. Snippets below are the proposed replacements.

Call site: `createQuinelingSubsystem()` is constructed once in `src/stage/SherlockStage.js` and `sub.update(fullState)` runs every animation frame before `renderer.render` (`SherlockStage.js` around lines 204 and 353–363; `src/app.js` `animate`). Playback keeps the same `source` string and `trace` object across frames (`getOrBuildStepState` mutates `stepFraction`, `ambient`, and `dt` on one cached state).

House rule this file breaks: Designers 1, 2, 3, 5, and 6 document a zero-allocation `update()`. Designer 6 (`designer6_loom.js` lines 19–24 and 163–172) already shows the pattern to copy: module scratch objects plus a `Uint32Array` of last hex values so `setColorAt` runs only when the color changes.

Visual contract: every replacement below writes the same floats the current math writes. No radius, color hex, opacity, segment count, or SDK sample index changes. Where a rigid yaw replaces a vertex rebuild, the matrix identity is spelled out so the sign cannot drift.

---

## Executive summary

Steady-state `update()` (species held, step held) currently allocates hundreds of short-lived objects and rewrites three ribbon GPU buffers that often did not change. The fixes below remove that work and leave the picture the same.

| ID | Severity | Lines | What changes | Steady-state effect |
| --- | --- | --- | --- | --- |
| F1 | P0 | 1314, 243–252, 275–276, 1274–1276 | Cache the compiled artifact | Drops a full source-string copy every frame |
| F2 | P0 | 427–442, 1375–1456 | Flatten SDK ridges into typed samples; stop per-frame point objects | ~272 object allocs gone; ridge graph can be GC'd on the step boundary |
| F3 | P0 | 450–577, 1436–1444, 1694–1701 | `embedOnSpeciesSurface` writes into a caller buffer | ~320 `{x,y,z}` allocs gone per frame |
| F4 | P0 | 332–393, 1405–1460 | Typed `writeRibbonStrip`; color vs position dirty flags | Static species (tree, lotus, comet, tokamak) stop per-frame ribbon uploads |
| F5 | P0 | 1721–1759 | Bake tendrils once; yaw with `rotation.y` | 160-segment rebuild and ~11 KB × 3 uploads gone per frame |
| F6 | P0 | 1694–1717 | Spore colors dirty-checked; no `new THREE.Color`; direct instance matrix | Up to 64 `Color` allocs gone; color upload only on change |
| F7 | P0 | 1477–1496 | Cache nautilus septum pose | Up to 30 `Vector3` allocs and 20 `getPointAt`/`getTangentAt` calls gone per frame |
| F8 | P1 | 1504–1649 | Typed silk and jellyfish centerlines; color dirty flags | Same pixels, no `Vector3` traffic, color buffer quiet while `isDone` / `isFault` hold |
| F9 | P1 | 1320–1322, 1344–1370, 1559–1688 | Dirty-check visibility, rule strings, and material hex | Stops `Object.entries`, `padStart`, and redundant `setHex` |
| F10 | P2 | 744–776, 833–920, 1038–1054, 1207–1217 | Share identical procedural geometries | One GPU buffer per repeated primitive; scale carries the size |
| F11 | P2 | 395–408, 413–422, 626–684, 1127–1134 | `Uint16` indices, scratch cylinders, `DynamicDrawUsage`, static bounds | Init allocs down; streaming buffers declared before first render; no radius-0 cull |

Already in good shape (do not “optimize” these into a different picture):

- Index buffers are built once (`setIndex` at 633, 684, 815, 1134).
- Fore/hind wing extrudes are built once and shared left/right (984–995). `leftFore.scale.x = -1` is a mirror, not a second geometry.
- Coral branches share `cylUnitGeom` (875). Fuel cells share `cellArcGeom` (1234). Lotus petals share `petalConeGeom` (1062). Polyp petals share `polypPetalGeom` (906–907).
- Spore body is one `OctahedronGeometry` on an `InstancedMesh` (657–663). `instanceMatrix` is already `DynamicDrawUsage` (664).
- SDK `WovenBody.frame` is already gated by `lastStepRidgeKey` (1375–1389). It must stay off the per-frame path.
- Nautilus tube, Lorenz tube, and tokamak braid are `TubeGeometry` built once (737–741, 998–1025, 1254–1272). Their `Vector3` control points are retained by `CatmullRomCurve3`. Do not pool those vectors.
- Species animation branches are already mutually exclusive (`if / else if` from 1465). Hidden species do not rebuild their ribbons.
- Breathing `coreRoot.position.y` / `rotation.y` (1364–1365), nautilus yaw (1468), moth flap (1582–1587), lotus spin (1597–1607), comet-free ambient terms, and fuel braid yaw (1678) are the animation. Keep them.

---

## Per-frame cost today (species fixed, step fixed)

Counts are structural, from the source, not a profiler run.

Always:

1. `resolveCompiledQuineling` (1314) calls `compileSherlockProgramToQuineling`, which on a cache hit still does `String(source)` plus a template key of the entire program (`sherlockQuinelingCompiler.js` 596–607).
2. `Object.entries(speciesGroups)` (1320) allocates an array of 8 pairs.
3. `` `A${String(ruleLine).padStart(3, '0')}` `` (1345) allocates whenever `step.ruleId` is missing. The synthetic id is redundant with the `ruleLine` tests (see F9).
4. `` `${artifact.cacheKey}|step:${stepIndex}` `` (1375) allocates every frame just to compare.
5. `sampleSdkRidgePoint` (1409, 1438) returns a new object. 16 ribbons × (1 midpoint + 16 segments) = **272 objects**.
6. `embedOnSpeciesSurface` (1442, 1698) returns a new object. 256 ribbon samples + 64 spores = **320 objects**.
7. `writeRibbonStrip` on the SDK buffer (1445) and on tendrils (1744) rewrites positions, normals, and colors, then sets `needsUpdate` on all three attributes (1458–1460, 1757–1759).
8. Spore loop (1695–1714) calls `setMatrixAt` and `setColorAt` 64 times. On a fault step, line 1713 does `new THREE.Color(0xe11d48)` **per spore** (64 colors).

When `speciesId === 'factorial'` (1477–1496): each visible septum calls `getPointAt` and `getTangentAt` without a target. `CatmullRomCurve3.getPoint` defaults `optionalTarget = new Vector3()` (`node_modules/three/src/extras/curves/CatmullRomCurve3.js` line 175). Line 1491 also does `new THREE.Vector3(0, 1, 0)`. Up to **30 vectors per frame**. Pose does not depend on `ambient`.

When `list_fold`: silk rebuilds 3 × 28 segments (1505–1530) including colors that depend only on `isDone`.
When `checked_underflow`: tentacles rebuild 16 × 10 segments (1618–1649) including colors that depend only on `isFault`.

`WovenBody.frame` (1379) is not per frame, but each step change with `crests: true` builds a large ridge object graph (family adapter: on the order of strands × 15 ridges × 161 point objects; other mechanisms build caustic polylines). The renderer only reads 16 ridges × 16 quantized samples. `cachedSdkRidges` (1384) holds the whole graph until the next step. `wf.points` (budget 256) is never read.

GPU sizes if a buffer is reuploaded every frame:

| Buffer | Vertices | Position+normal+color |
| --- | --- | --- |
| SDK ribbons | 512 | 18 KB |
| Tendrils | 320 | 11.25 KB |
| Silk (list fold only) | 168 | 6 KB |
| Jelly tentacles (jelly only) | 320 | 11.25 KB |
| Spores | 64 instances | 4 KB matrices + 0.75 KB colors |

---

## F1. Cache the compiled artifact inside `update()`

**Lines:** 1314 (call), 243–252 (`resolveCompiledQuineling`), 275–276 and 1274–1276 (second resolve via telemetry).

`compileSherlockProgramToQuineling` is a `Map` hit during playback, but the hit still copies the Sherlock source into a new cache-key string every frame. `refreshInspectMetadata` calls `getQuinelingTelemetry`, which resolves again. UI telemetry (`app.js` 555) calls the same function once per step index, not per frame.

Invalidation that matches the compiler key (`cacheKey = source | stackLimit | initialFuel | outcomeKind`) without copying `source`:

- `stepState.source` and `stepState.trace` by reference (`===` on the same string is a pointer compare in V8).
- Cheap revision of `trace.steps[0]` fuel / stack limit and `trace.outcome.kind`, in case a trace object is mutated in place.

`getOrBuildStepState` keeps those references stable while `ambient` changes, so this returns the identical artifact object the compiler cache would return.

**Replacement**, inside `createQuinelingSubsystem`, next to the cache fields at 1294–1300:

```javascript
let cachedArtifact = null;
let cachedArtifactSource = undefined;
let cachedArtifactTrace = undefined;
let cachedArtifactRev = '';

function artifactRevision(stepState) {
  const trace = stepState.trace;
  const stackLimit = trace?.steps?.[0]?.after?.stackLimit ?? trace?.stackLimit ?? '';
  const initialFuel = trace?.steps?.[0]?.before?.fuel ?? trace?.fuel ?? '';
  const outcomeKind = trace?.outcome?.kind || '';
  return `${stackLimit}|${initialFuel}|${outcomeKind}`;
}

function currentArtifact(stepState) {
  const trace = stepState.trace || null;
  const source = stepState.source || trace?.source || '';
  const rev = artifactRevision(stepState);
  if (
    cachedArtifact &&
    source === cachedArtifactSource &&
    trace === cachedArtifactTrace &&
    rev === cachedArtifactRev
  ) {
    return cachedArtifact;
  }
  cachedArtifact = resolveCompiledQuineling(stepState);
  cachedArtifactSource = source;
  cachedArtifactTrace = trace;
  cachedArtifactRev = rev;
  return cachedArtifact;
}
```

**Replacement** at line 1314:

```javascript
const artifact = currentArtifact(stepState);
```

**Replacement** for `getQuinelingTelemetry` (275–276) so the inspector path can reuse an artifact it already holds. The one-argument call from `app.js` stays valid.

```javascript
export function getQuinelingTelemetry(stepState = {}, artifactOverride = null) {
  const artifact = artifactOverride || resolveCompiledQuineling(stepState);
  const species = artifact.speciesMeta;
  // ... rest of the function unchanged from line 278 ...
```

**Replacement** at 1276:

```javascript
const tel = getQuinelingTelemetry(stepState, artifact);
```

---

## F2. Flatten SDK ridges once per step; typed samples after that

**Lines:** 427–442 (`sampleSdkRidgePoint`), 1287–1296 (scratch), 1372–1456 (frame + ribbon loop).

`sampleSdkRidgePoint` quantizes with `Math.floor(u * (line.length - 1))`. That quantization is the visual result. Do not resample the continuous SDK curve at `u`, and do not shorten ridge lines. Copy those exact quantized `x, y, z, owner` values out once, then drop `wf.ridges` so the point-object graph can be collected on the step boundary.

`sdkWave` is the only per-frame use of the sample:

```javascript
clamp((rawPt.x + rawPt.y + rawPt.z) * 0.15 + Math.sin(ambient * 1.6 + u * TAU) * 0.04, -0.35, 0.35)
```

Store `rawSum = x + y + z` (or `NaN` when the sample is missing, which takes the sine-only branch at 1441).

`owner` is read only at `u = 0.5` (1409–1413). Store one owner per ribbon.

`WovenBody.frame` accepts `reuse` for the unused point buffer on these mechanisms: `logarithmic-mantle`, `toroidal-weave`, `phyllotaxis-fan` (family adapter), `pleated-braid`, `lorenz-flow`, `clifford-flow`. `recursiveFrame` (tree) and `juliaFrame` (comet) ignore `reuse`. Passing it is still free and cuts two typed allocations on the mechanisms that honor it. Crest generation itself stays; that is what produces the ridges.

Also cache `activeNodeIdx`. Line 1401–1402 runs `findIndex` every frame. Node ids are strings. Keep the truthy check: a truthy id that is missing must stay **-1**, which is what `findIndex` returns. Do not fall through to `stepIndex % length` in that case.

**Replacement** scratch (extend 1287–1300):

```javascript
const SDK_SAMPLE_COUNT = SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS;
const sdkSampleSum = new Float32Array(SDK_SAMPLE_COUNT); // NaN => missing sample
const sdkRibbonOwner = new Int16Array(SDK_RIBBON_COUNT);
const sdkFrameReuse = {
  points: new Float32Array(256 * 4),
  owners: new Uint16Array(256),
};
let lastRidgeArtifactKey = '';
let lastRidgeStep = -1;
let nodeIndexById = new Map();

function flattenSdkRidges(ridges) {
  const ridgeCount = ridges.length;
  for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
    const ridgeIndex = Math.floor((rIdx * Math.max(1, ridgeCount)) / SDK_RIBBON_COUNT);
    const ridge = ridges[ridgeIndex];
    const line = ridge?.line || ridge?.points;
    const lineOk = Array.isArray(line) && line.length > 0;
    const mid = lineOk ? line[Math.min(line.length - 1, Math.floor(0.5 * (line.length - 1)))] : null;
    let owner = -1;
    if (mid) {
      const rawOwner = Array.isArray(mid) ? 0 : (Number.isInteger(mid.owner) ? mid.owner : -1);
      owner = rawOwner;
    }
    sdkRibbonOwner[rIdx] = owner;
    for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
      const u = s / (SDK_RIBBON_SEGMENTS - 1);
      const slot = rIdx * SDK_RIBBON_SEGMENTS + s;
      if (!lineOk) {
        sdkSampleSum[slot] = NaN;
        continue;
      }
      const idx = Math.min(line.length - 1, Math.floor(u * (line.length - 1)));
      const pt = line[idx];
      if (!pt) {
        sdkSampleSum[slot] = NaN;
        continue;
      }
      const x = Array.isArray(pt) ? (pt[0] || 0) : (Number.isFinite(pt.x) ? pt.x : 0);
      const y = Array.isArray(pt) ? (pt[1] || 0) : (Number.isFinite(pt.y) ? pt.y : 0);
      const z = Array.isArray(pt) ? (pt[2] || 0) : (Number.isFinite(pt.z) ? pt.z : 0);
      sdkSampleSum[slot] = x + y + z;
    }
  }
}
```

This is the same indexing and the same `|| 0` / `Number.isFinite` rules as `sampleSdkRidgePoint` (427–441). Midpoint owner uses the same `u = 0.5` floor. Array points have `owner: 0` in the old helper, so a missing object owner on an array point stays 0, not “absent”. The old `ownerIdx` expression treats `0` as a valid owner (`midSample.owner >= 0`).

**Replacement** for the block at 1375–1403:

```javascript
if (artifact.cacheKey !== lastRidgeArtifactKey || stepIndex !== lastRidgeStep) {
  lastRidgeArtifactKey = artifact.cacheKey;
  lastRidgeStep = stepIndex;
  const stepPhaseRad = (stepIndex * 0.08) * TAU;
  const wf = WovenBody.frame(artifact.wovenBody, stepPhaseRad, {
    budget: 256,
    crests: true,
    reuse: sdkFrameReuse,
  });
  let ridges = wf.ridges;
  if (!ridges?.length) {
    const mf = Morphology.surfaceFrame(artifact.shape, stepPhaseRad, true);
    ridges = mf.ridges || [];
  }
  flattenSdkRidges(ridges);
  nodeIndexById = new Map(artifact.graph.nodes.map((n, i) => [n.id, i]));
}

const activeNodeIdx = activeNodeId
  ? (nodeIndexById.get(activeNodeId) ?? -1)
  : stepIndex % Math.max(1, artifact.graph.nodes.length);
```

Do not assign `cachedSdkRidges = wf.ridges`. After `flattenSdkRidges` returns, nothing in `update` should touch `wf`.

Per-frame wave, identical to 1439–1441, with no object:

```javascript
function sdkWaveAt(rIdx, s, ambient) {
  const uTau = (s / (SDK_RIBBON_SEGMENTS - 1)) * TAU;
  const sine = Math.sin(ambient * 1.6 + uTau) * 0.04;
  const sum = sdkSampleSum[rIdx * SDK_RIBBON_SEGMENTS + s];
  if (sum !== sum) return sine; // NaN => missing sample, old fallback
  const wave = sum * 0.15 + sine;
  return wave < -0.35 ? -0.35 : wave > 0.35 ? 0.35 : wave;
}
```

`u * TAU` equals the old `u * TAU` because `u = s / (SDK_RIBBON_SEGMENTS - 1)`.

---

## F3. `embedOnSpeciesSurface` writes into a typed slot

**Lines:** 450–577 (function), call sites 1442 and 1698.

The function returns a fresh object on every call and recomputes quantities that depend only on `(speciesId, segment, ribbon index)`. Split it:

- Once per species (or once per step when the ridge wave is baked as a coefficient): store the constants.
- Per frame: only the trig that multiplies `sdkDelta`.

`sdkDelta` is **ignored** for `tree_fold`, `lexical_let`, `stack_overflow`, and `fuel_exhaustion` (477–574). For those four, ribbon positions do not depend on the ridge or on `ambient`. They still depend on which ribbon is the wide “active” ribbon (`halfWidth` 0.0055 vs 0.0032), and that changes with owner + `activeNodeIdx`, i.e. on the step boundary, not every frame.

`sdkDelta` **is** used by:

| Species | Where | Per-frame term |
| --- | --- | --- |
| `factorial` | roll `+ sdkDelta * 0.4` (457) | all 16 ribbons |
| `list_fold` | angle `+ sdkDelta` (469) | all 16 ribbons |
| `mutual_even_odd` | `z + sdkDelta * 0.04` (515) | all 16 ribbons; x and y are constant |
| `checked_underflow` | tentacle twist `+ sdkDelta * 0.3` only when `rIdx >= 6` (545). Meridians `rIdx < 6` (532–540) ignore it | ribbons 6–15 only |

Spores call `embedOnSpeciesSurface(speciesId, u, v, s % 16, 0.03)` with a **constant** delta `0.03`, but `u` and `v` change with `ambient` (1696–1698). Spores still need a per-frame embed. They must not allocate.

**Replacement** signature. `out` is a `Float32Array`; `o` is the float index of `x` (`y` at `o+1`, `z` at `o+2`). Body is the current `switch`, with every `return { x, y, z }` changed to stores. Example for the factorial arm (452–463), which is the pattern for every arm:

```javascript
function embedOnSpeciesSurface(speciesId, u, v, rIdx, sdkDelta, out, o) {
  switch (speciesId) {
    case 'factorial': {
      const theta = u * 2.05 * TAU;
      const rad = 0.022 * Math.exp(0.215 * theta);
      const tubeR = Math.max(0.014, rad * 0.36);
      const roll = (v * 0.5 + 0.5) * TAU + sdkDelta * 0.4;
      const R = rad + (tubeR + 0.004) * Math.cos(roll);
      out[o] = Math.cos(theta) * R;
      out[o + 1] = Math.sin(theta) * R * 0.78 + 0.02;
      out[o + 2] = Math.sin(theta) * rad * 0.22 + (tubeR + 0.004) * Math.sin(roll);
      return;
    }
    // list_fold, tree_fold, mutual_even_odd, lexical_let,
    // checked_underflow, stack_overflow, fuel_exhaustion:
    // same expressions as lines 465–574, stored into out[o], out[o+1], out[o+2].
    default: {
      const theta = u * TAU + (rIdx / SDK_RIBBON_COUNT) * TAU;
      const phi = 3.0 * theta + v * Math.PI;
      const R = 0.32;
      const r = 0.074;
      const rc = r * Math.cos(phi);
      out[o] = (R + rc) * Math.cos(theta);
      out[o + 1] = (R + rc) * Math.sin(theta);
      out[o + 2] = r * Math.sin(phi);
    }
  }
}
```

Keep every constant (`2.05`, `0.215`, `0.78`, `0.36`, `0.004`, branch thresholds `0.45` / `0.4`, Lorenz `0.48`, lotus `PHI_GOLDEN_ANGLE`, jellyfish `0.56`, comet `2.2`, torus poloidal `3.0`) exactly as written. The `default` arm is `fuel_exhaustion`.

**Hot-path bake** for the four animated ribbon species. Fill once per `(speciesId, step)` because `vCoord` is `((rIdx % 8) / 7) * 2 - 1` and the ridge sum is fixed for the step. `sdkWave` still varies per frame, so store the coefficients the wave multiplies, not the final position.

`SDK_RIBBON_SEGMENTS` is 16, so `u = s / 15`.

```javascript
const SDK_BAKE = SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS;
// factorial: cosT, sinT, rad, tubeOff, rollBase
const factCosT = new Float32Array(SDK_BAKE);
const factSinT = new Float32Array(SDK_BAKE);
const factRad = new Float32Array(SDK_BAKE);
const factTubeOff = new Float32Array(SDK_BAKE);
const factRollBase = new Float32Array(SDK_BAKE);
// list_fold: x, waveY, rad, angleBase
const listX = new Float32Array(SDK_BAKE);
const listWaveY = new Float32Array(SDK_BAKE);
const listRad = new Float32Array(SDK_BAKE);
const listAngleBase = new Float32Array(SDK_BAKE);
// moth: x, y, zBase (z adds sdkWave * 0.04)
const mothX = new Float32Array(SDK_BAKE);
const mothY = new Float32Array(SDK_BAKE);
const mothZBase = new Float32Array(SDK_BAKE);
// jelly tentacle ribbons rIdx >= 6: rad, y, twistBase. rIdx < 6 stored as finished xyz.
const jellyRad = new Float32Array(SDK_BAKE);
const jellyY = new Float32Array(SDK_BAKE);
const jellyTwistBase = new Float32Array(SDK_BAKE);
const jellyStatic = new Float32Array(SDK_BAKE * 3); // meridians, fully baked

function bakeSdkEmbed(speciesId) {
  for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
    const v = ((rIdx % 8) / 7) * 2 - 1;
    for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
      const u = s / (SDK_RIBBON_SEGMENTS - 1);
      const i = rIdx * SDK_RIBBON_SEGMENTS + s;
      if (speciesId === 'factorial') {
        const theta = u * 2.05 * TAU;
        const rad = 0.022 * Math.exp(0.215 * theta);
        const tubeR = Math.max(0.014, rad * 0.36);
        factCosT[i] = Math.cos(theta);
        factSinT[i] = Math.sin(theta);
        factRad[i] = rad;
        factTubeOff[i] = tubeR + 0.004;
        factRollBase[i] = (v * 0.5 + 0.5) * TAU;
      } else if (speciesId === 'list_fold') {
        listX[i] = -0.4 + (0.4 - -0.4) * u;
        listWaveY[i] = Math.sin(u * TAU * 1.5) * 0.075;
        listRad[i] = 0.048 + 0.025 * Math.cos(u * TAU * 3.0);
        listAngleBase[i] = v * Math.PI + u * TAU * 2.0;
      } else if (speciesId === 'mutual_even_odd') {
        const side = rIdx % 2 === 0 ? -1 : 1;
        const ang = u * TAU;
        const rad = 0.13 + 0.06 * Math.abs(v);
        const tilt = 0.48 * side;
        const ct = Math.cos(tilt);
        const st = Math.sin(tilt);
        mothX[i] = side * 0.16 + Math.cos(ang) * rad;
        mothY[i] = Math.sin(ang) * rad * ct;
        mothZBase[i] = side * 0.04 + Math.sin(ang) * rad * st;
      } else if (speciesId === 'checked_underflow') {
        if (rIdx < 6) {
          const phi = (rIdx / 6) * TAU + v * 0.35;
          const theta = u * Math.PI * 0.56;
          const R = 0.312;
          const o = i * 3;
          jellyStatic[o] = R * Math.sin(theta) * Math.cos(phi);
          jellyStatic[o + 1] = 0.06 + R * Math.cos(theta);
          jellyStatic[o + 2] = R * Math.sin(theta) * Math.sin(phi);
        } else {
          const tIdx = rIdx % 16;
          const baseA = (tIdx / 16) * TAU;
          jellyRad[i] = (tIdx % 2 === 0 ? 0.24 : 0.16) * (1 - 0.18 * u);
          jellyY[i] = 0.02 + (-0.38 - 0.02) * u;
          jellyTwistBase[i] = baseA + u * 0.9;
        }
      }
    }
  }
}
```

`lerp(a, b, t)` is `a + (b - a) * t` (`theme.js` 115–116). The expanded forms above match it.

Per-frame centerline fill into a `Float32Array` of length `SDK_RIBBON_SEGMENTS * 3` (one ribbon) or into the strip scratch below. Factorial example, algebraically the same as lines 453–462:

```javascript
function fillFactorialRibbon(rIdx, ambient, dst) {
  for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
    const i = rIdx * SDK_RIBBON_SEGMENTS + s;
    const roll = factRollBase[i] + sdkWaveAt(rIdx, s, ambient) * 0.4;
    const c = Math.cos(roll);
    const sn = Math.sin(roll);
    const R = factRad[i] + factTubeOff[i] * c;
    const o = s * 3;
    dst[o] = factCosT[i] * R;
    dst[o + 1] = factSinT[i] * R * 0.78 + 0.02;
    dst[o + 2] = factSinT[i] * factRad[i] * 0.22 + factTubeOff[i] * sn;
  }
}
```

List fold:

```javascript
function fillListFoldRibbon(rIdx, ambient, dst) {
  for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
    const i = rIdx * SDK_RIBBON_SEGMENTS + s;
    const angle = listAngleBase[i] + sdkWaveAt(rIdx, s, ambient);
    const o = s * 3;
    dst[o] = listX[i];
    dst[o + 1] = listWaveY[i] + Math.sin(angle) * listRad[i];
    dst[o + 2] = Math.cos(angle) * listRad[i];
  }
}
```

Moth (x, y frozen; z adds the wave):

```javascript
function fillMothRibbon(rIdx, ambient, dst) {
  for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
    const i = rIdx * SDK_RIBBON_SEGMENTS + s;
    const o = s * 3;
    dst[o] = mothX[i];
    dst[o + 1] = mothY[i];
    dst[o + 2] = mothZBase[i] + sdkWaveAt(rIdx, s, ambient) * 0.04;
  }
}
```

Jelly:

```javascript
function fillJellyRibbon(rIdx, ambient, dst) {
  for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
    const i = rIdx * SDK_RIBBON_SEGMENTS + s;
    const o = s * 3;
    if (rIdx < 6) {
      const j = i * 3;
      dst[o] = jellyStatic[j];
      dst[o + 1] = jellyStatic[j + 1];
      dst[o + 2] = jellyStatic[j + 2];
    } else {
      const twist = jellyTwistBase[i] + sdkWaveAt(rIdx, s, ambient) * 0.3;
      const rad = jellyRad[i];
      dst[o] = Math.cos(twist) * rad;
      dst[o + 1] = jellyY[i];
      dst[o + 2] = Math.sin(twist) * rad;
    }
  }
}
```

Static species (`tree_fold`, `lexical_let`, `stack_overflow`, `fuel_exhaustion`): call `embedOnSpeciesSurface` once per step into a centerline cache of `SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS * 3` floats with `sdkDelta = 0`. The value is unused by those arms, so `0` matches any wave. Rebuild that cache only when `speciesId` changes. Width still changes per step (F4).

Spore call site (1696–1701), no object:

```javascript
const sporeScratch = new Float32Array(3);
// inside the loop:
const u = ((s / SPORE_COUNT) + ambient * 0.04 * (0.4 + liveFuelRatio)) % 1;
const v = Math.sin(s * PHI_GOLDEN_ANGLE + ambient * 0.8);
embedOnSpeciesSurface(speciesId, u, v, s % 16, 0.03, sporeScratch, 0);
const outScale = isExhausted ? 0.88 : 1.08;
const px = sporeScratch[0] * outScale;
const py = sporeScratch[1] * outScale;
const pz = sporeScratch[2] * outScale;
```

`% 1` on a negative sum is not reachable: `s / 64` is in `[0, 1)` and the ambient term is non-negative (`0.04 * (0.4 + liveFuelRatio) >= 0`). Same expression as line 1696.

---

## F4. Typed `writeRibbonStrip`, and do not rewrite static ribbons

**Lines:** 332–393, 1405–1460, plus the silk (1515), jelly (1634), and tendril (1744) callers.

`taper` and `glow` depend only on `s` and `segments`:

```javascript
const denom = Math.max(1, segments - 1);
const taper = Math.sin((s / denom) * Math.PI) * 0.65 + 0.35;
const glow = 0.72 + 0.28 * Math.sin((s / denom) * Math.PI);
```

Precompute them for the three segment counts in this file: 16 (SDK), 28 (silk), 10 (jelly and tendrils).

Normals do not use `halfWidth`. Positions do. Colors use `glow * rgb` and do not use `ambient`. So:

- Position + normal upload: only when the centerline or the per-ribbon width changed.
- Color upload: only when rgb or the active-node highlight changed.

Width rule (1452): `ownerIdx === activeNodeIdx ? 0.0055 : 0.0032`. `ownerIdx` comes from the flattened owner (F2) with the same fallback as 1410–1413.

```javascript
function ribbonOwnerIndex(rIdx, nodeColorCount) {
  const owner = sdkRibbonOwner[rIdx];
  if (owner !== null && owner !== undefined && owner >= 0) return owner;
  const n = Math.max(1, nodeColorCount);
  return rIdx % n;
}
```

`sdkRibbonOwner` stores `-1` for a missing midpoint. The old code used `midSample?.owner !== null && !== undefined && >= 0`. An array point’s owner is `0`, which is `>= 0`, so it is kept. Good.

**Replacement** for `writeRibbonStrip`. `points` is `Float32Array`, tightly packed xyz, length `segments * 3`.

```javascript
const TAPER_16 = new Float32Array(16);
const GLOW_16 = new Float32Array(16);
const TAPER_28 = new Float32Array(28);
const GLOW_28 = new Float32Array(28);
const TAPER_10 = new Float32Array(10);
const GLOW_10 = new Float32Array(10);

function fillTaperGlow(taper, glow) {
  const denom = Math.max(1, taper.length - 1);
  for (let s = 0; s < taper.length; s += 1) {
    const sn = Math.sin((s / denom) * Math.PI);
    taper[s] = sn * 0.65 + 0.35;
    glow[s] = 0.72 + 0.28 * sn;
  }
}
fillTaperGlow(TAPER_16, GLOW_16);
fillTaperGlow(TAPER_28, GLOW_28);
fillTaperGlow(TAPER_10, GLOW_10);

function writeRibbonStrip(positions, colors, normals, curveIndex, segments, points, halfWidth, r, g, b, taper, glow, writeColor) {
  const baseVertex = curveIndex * segments * 2;
  for (let s = 0; s < segments; s += 1) {
    const p = s * 3;
    const pNext = Math.min(segments - 1, s + 1) * 3;
    const pPrev = Math.max(0, s - 1) * 3;
    let tx = points[pNext] - points[pPrev];
    let ty = points[pNext + 1] - points[pPrev + 1];
    let tz = points[pNext + 2] - points[pPrev + 2];
    const tLen = Math.hypot(tx, ty, tz) || 1;
    tx /= tLen;
    ty /= tLen;
    tz /= tLen;

    let bx = -tz;
    let bz = tx;
    const bLen = Math.hypot(bx, 0, bz);
    if (bLen < 1e-4) {
      bx = 1;
      bz = 0;
    } else {
      bx /= bLen;
      bz /= bLen;
    }

    const nx = ty * bz;
    const ny = tz * bx - tx * bz;
    const nz = -ty * bx;
    const w = halfWidth * taper[s];
    const px = points[p];
    const py = points[p + 1];
    const pz = points[p + 2];

    const v0 = (baseVertex + s * 2) * 3;
    const v1 = (baseVertex + s * 2 + 1) * 3;
    positions[v0] = px - bx * w;
    positions[v0 + 1] = py;
    positions[v0 + 2] = pz - bz * w;
    positions[v1] = px + bx * w;
    positions[v1 + 1] = py;
    positions[v1 + 2] = pz + bz * w;

    normals[v0] = nx;
    normals[v0 + 1] = ny;
    normals[v0 + 2] = nz;
    normals[v1] = nx;
    normals[v1 + 1] = ny;
    normals[v1 + 2] = nz;

    if (writeColor) {
      const glowS = glow[s];
      const cr = r * glowS;
      const cg = g * glowS;
      const cb = b * glowS;
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

`by` is always 0 in the original (348–357), and the normal cross product simplifies to the three lines above. `positions[* + 1]` does not subtract `by * w`. Same bytes.

Ribbon centerline scratch: one ribbon at a time, `new Float32Array(SDK_RIBBON_SEGMENTS * 3)`.

**Replacement** for the ribbon loop 1405–1460:

```javascript
const ribbonCenter = new Float32Array(Math.max(SDK_RIBBON_SEGMENTS, SILK_SEGMENTS, JELLY_TENTACLE_SEGMENTS, TENDRIL_SEGMENTS) * 3);
const STATIC_RIBBON_SPECIES = {
  tree_fold: true,
  lexical_let: true,
  stack_overflow: true,
  fuel_exhaustion: true,
};

let lastRibbonKey = '';
let lastStaticSpecies = '';
const staticCenter = new Float32Array(SDK_RIBBON_COUNT * SDK_RIBBON_SEGMENTS * 3);

function ensureStaticCenter(speciesId) {
  if (speciesId === lastStaticSpecies) return;
  lastStaticSpecies = speciesId;
  const vTmp = new Float32Array(3); // init / species-change only, not per frame
  for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
    const v = ((rIdx % 8) / 7) * 2 - 1;
    for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
      const u = s / (SDK_RIBBON_SEGMENTS - 1);
      embedOnSpeciesSurface(speciesId, u, v, rIdx, 0, vTmp, 0);
      const o = (rIdx * SDK_RIBBON_SEGMENTS + s) * 3;
      staticCenter[o] = vTmp[0];
      staticCenter[o + 1] = vTmp[1];
      staticCenter[o + 2] = vTmp[2];
    }
  }
}

// inside update, after colors cr,cg,cb are known for this ribbon:
// The color block at 1415–1434 stays, but runs only when `ribbonKey` changes
// (see below). Hoist it out of the per-frame path.

const ribbonKey = `${speciesId}|${stepIndex}|${artifact.cacheKey}|${activeNodeIdx}|${isFault ? 1 : 0}|${isExhausted ? 1 : 0}`;
const colorsDirty = ribbonKey !== lastRibbonKey;
if (colorsDirty) lastRibbonKey = ribbonKey;

if (STATIC_RIBBON_SPECIES[speciesId]) {
  if (colorsDirty) {
    ensureStaticCenter(speciesId);
    const nodeCount = nodeColors.length;
    for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
      const ownerIdx = ribbonOwnerIndex(rIdx, nodeCount);
      // compute cr,cg,cb exactly as lines 1414–1434 from nodeColors[ownerIdx % n]
      const base = rIdx * SDK_RIBBON_SEGMENTS * 3;
      ribbonCenter.set(staticCenter.subarray(base, base + SDK_RIBBON_SEGMENTS * 3));
      writeRibbonStrip(
        sdkRibbonPositions, sdkRibbonColors, sdkRibbonNormals,
        rIdx, SDK_RIBBON_SEGMENTS, ribbonCenter,
        ownerIdx === activeNodeIdx ? 0.0055 : 0.0032,
        cr, cg, cb, TAPER_16, GLOW_16, true
      );
    }
    sdkRibbonGeom.attributes.position.needsUpdate = true;
    sdkRibbonGeom.attributes.normal.needsUpdate = true;
    sdkRibbonGeom.attributes.color.needsUpdate = true;
  }
} else {
  if (speciesId !== bakedWaveSpecies) {
    bakeSdkEmbed(speciesId);
    bakedWaveSpecies = speciesId;
  }
  const nodeCount = nodeColors.length;
  for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
    if (speciesId === 'factorial') fillFactorialRibbon(rIdx, ambient, ribbonCenter);
    else if (speciesId === 'list_fold') fillListFoldRibbon(rIdx, ambient, ribbonCenter);
    else if (speciesId === 'mutual_even_odd') fillMothRibbon(rIdx, ambient, ribbonCenter);
    else fillJellyRibbon(rIdx, ambient, ribbonCenter);
    const ownerIdx = ribbonOwnerIndex(rIdx, nodeCount);
    // cr,cg,cb from the same formula as 1414–1434; cache the 16 rgb triples
    // when colorsDirty, reuse them on later frames
    writeRibbonStrip(
      sdkRibbonPositions, sdkRibbonColors, sdkRibbonNormals,
      rIdx, SDK_RIBBON_SEGMENTS, ribbonCenter,
      ownerIdx === activeNodeIdx ? 0.0055 : 0.0032,
      cr, cg, cb, TAPER_16, GLOW_16, colorsDirty
    );
  }
  sdkRibbonGeom.attributes.position.needsUpdate = true;
  sdkRibbonGeom.attributes.normal.needsUpdate = true;
  if (colorsDirty) sdkRibbonGeom.attributes.color.needsUpdate = true;
}
```

`ribbonCenter.set(subarray)` copies 48 floats once per ribbon per step for static species. That runs only when `colorsDirty` is true (step / species / fault / active node). It does not run per frame.

Cache the 16 `(cr, cg, cb, halfWidth)` rows in a `Float32Array(SDK_RIBBON_COUNT * 4)` when `colorsDirty` flips, and read them on animated frames. Recomputing the lerps every frame is cheap next to the geometry upload, but it is still pure waste: lines 1415–1434 do not read `ambient` or `stepFraction`.

`staticCenter.subarray` allocates a small view object. On the step-change path that is acceptable. To keep the step boundary allocation-free too, `ribbonCenter.set` is optional: pass `staticCenter` plus a start offset into a variant of `writeRibbonStrip` that reads `points[base + s * 3]`. Prefer that if you want the step change to allocate nothing.

Do not call `computeBoundingSphere()` after these writes. F11 sets a padded sphere once. `needsUpdate` does not clear `boundingSphere` in r175, so a sphere set at init stays valid. A sphere computed from the zero-filled initial buffer would be radius 0 and frustum-cull the mesh forever if the first render won the race; an explicit sphere removes that.

---

## F5. Chamber tendrils: bake once, yaw the mesh

**Lines:** 1721–1759.

The only time-varying term is `ambient * 0.1` added to every curve’s base angle (1727):

```javascript
const angle = (idx / 8) * TAU + ambient * 0.1;
const twist = angle + t * (isFloorRoot ? -0.35 : 0.35);
```

Radius, Y, taper, and the per-curve twist offset do not depend on time. Colors (1743) depend on `isFault`, `isDone`, and the species hexes, not on `ambient`.

Bake at `ambient = 0`. Three.js `Matrix4.makeRotationY(θ)` (`node_modules/three/src/math/Matrix4.js` 894–905) maps a point to:

```text
x' =  cos θ · x + sin θ · z
z' = -sin θ · x + cos θ · z
```

If the baked point is `(cos twist0 · rad, y, sin twist0 · rad)`, that yaw produces `twist0 - θ`. The live code wants `twist0 + ambient * 0.1`. Therefore:

```javascript
chamberTendrilsMesh.rotation.y = -ambient * 0.1;
```

Parent `coreRoot.rotation.y` (1365) still applies on top, exactly as it does today, because today’s yaw is baked in the tendril mesh’s local space and the parent rotation is a second transform. Local mesh yaw then parent yaw is the same composition.

Normals rotate with the mesh (the normal matrix of a pure rotation). The original normals are the ribbon frame of those same local points. They match.

**Replacement:**

```javascript
let tendrilColorKey = '';

function tendrilRgb(isFault, isDone, isFloorRoot, species) {
  _color.setHex(isFault ? 0xe11d48 : isDone ? 0x2dd4bf : isFloorRoot ? species.primaryHex : species.secondaryHex);
  return { r: _color.r, g: _color.g, b: _color.b };
}

function rebuildTendrils(species, isFault, isDone) {
  for (let c = 0; c < TENDRIL_CURVE_COUNT; c += 1) {
    const isFloorRoot = c < 8;
    const idx = c % 8;
    const angle = (idx / 8) * TAU; // ambient term removed; mesh yaw supplies it
    const startR = QUINELING_MAX_RADIUS + 0.04;
    const endR = isFloorRoot ? 0.56 : 0.46;
    const startY = isFloorRoot ? -0.26 : 0.26;
    const endY = isFloorRoot ? (-3.38 - QUINELING_CORE_Y) : (-0.25 - QUINELING_CORE_Y);
    for (let s = 0; s < TENDRIL_SEGMENTS; s += 1) {
      const t = s / (TENDRIL_SEGMENTS - 1);
      const rad = startR + (endR - startR) * t + Math.sin(t * Math.PI) * 0.035;
      const twist = angle + t * (isFloorRoot ? -0.35 : 0.35);
      const o = s * 3;
      ribbonCenter[o] = Math.cos(twist) * rad;
      ribbonCenter[o + 1] = startY + (endY - startY) * t;
      ribbonCenter[o + 2] = Math.sin(twist) * rad;
    }
    const rgb = tendrilRgb(isFault, isDone, isFloorRoot, species);
    writeRibbonStrip(
      tendrilPositions, tendrilColors, tendrilNormals,
      c, TENDRIL_SEGMENTS, ribbonCenter,
      0.0036, rgb.r, rgb.g, rgb.b, TAPER_10, GLOW_10, true
    );
  }
  tendrilGeom.attributes.position.needsUpdate = true;
  tendrilGeom.attributes.normal.needsUpdate = true;
  tendrilGeom.attributes.color.needsUpdate = true;
}

// end of update(), replaces lines 1721–1759:
const tendrilKey = `${speciesId}|${isFault ? 1 : 0}|${isDone ? 1 : 0}`;
if (tendrilKey !== tendrilColorKey) {
  tendrilColorKey = tendrilKey;
  rebuildTendrils(species, isFault, isDone);
}
chamberTendrilsMesh.rotation.y = -ambient * 0.1;
```

`floorRelY` and `ringsRelY` (1722–1723) are constants. Inlining `-3.38 - QUINELING_CORE_Y` and `-0.25 - QUINELING_CORE_Y` matches the comments `-1.33` and `+1.80`. Opacity stays on `tendrilMat.opacity` (1342) and is independent of the vertex bake.

Build once at the end of `createQuinelingSubsystem` as well, before the first frame, so the first rendered frame is not an empty buffer. `rebuildTendrils` with the factorial defaults (`isFault` false, `isDone` false, factorial hexes) matches the first `update` only if the first step is that state. Safer: leave the first `update` to fill them (the key starts as `''` and will miss). Set the bounding sphere in F11 so a one-frame empty buffer cannot cull later frames.

---

## F6. Spores: no per-frame `Color`, dirty instance colors, direct matrices

**Lines:** 1692–1717, and the init at 657–671.

Line 1713 allocates a `THREE.Color` for every spore on every fault frame:

```javascript
if (isFault) _color.lerp(new THREE.Color(0xe11d48), 0.7);
```

`_color` is already reused (1288). The fault target must be too.

Instance color does not depend on `ambient`, `stepFraction`, `isDone`, or `isExhausted`. It depends on `artifact.rgbPixels`, `species.primaryHex`, and `isFault`. `isDone` / `isExhausted` change scale only (1703). Matrices must still update every frame because `u`, `v`, and rotation do.

`setColorAt` writes 3 floats (`InstancedMesh.js` 318–327). `instanceColor.needsUpdate = true` every frame reuploads 768 bytes even when those floats are unchanged. Designer 6 avoids that with `lastSegHex`.

`setMatrixAt` copies `matrix.elements` (`InstancedMesh.js` 339–342). `_dummy.rotation.set` + `updateMatrix` builds that matrix from Euler XYZ (the `Object3D` default) with z = 0. The same matrix, written straight into `instanceMatrix.array`:

Quaternion from Euler XYZ, z = 0 (`Quaternion.setFromEuler`, order `'XYZ'`):

```text
qx = sin(rx/2) cos(ry/2)
qy = cos(rx/2) sin(ry/2)
qz = sin(rx/2) sin(ry/2)
qw = cos(rx/2) cos(ry/2)
```

`Matrix4.compose` with uniform scale `sc` then matches `Object3D.updateMatrix` for this spore. Column-major layout, the order `toArray` writes:

```javascript
const _faultCrimson = new THREE.Color(0xe11d48);
let lastSporeColorKey = '';

function writeSporeMatrix(array, index, x, y, z, rotX, rotY, sc) {
  const hx = rotX * 0.5;
  const hy = rotY * 0.5;
  const qx = Math.sin(hx) * Math.cos(hy);
  const qy = Math.cos(hx) * Math.sin(hy);
  const qz = Math.sin(hx) * Math.sin(hy);
  const qw = Math.cos(hx) * Math.cos(hy);
  const x2 = qx + qx;
  const y2 = qy + qy;
  const z2 = qz + qz;
  const xx = qx * x2;
  const xy = qx * y2;
  const xz = qx * z2;
  const yy = qy * y2;
  const yz = qy * z2;
  const zz = qz * z2;
  const wx = qw * x2;
  const wy = qw * y2;
  const wz = qw * z2;
  const o = index * 16;
  array[o] = (1 - (yy + zz)) * sc;
  array[o + 1] = (xy + wz) * sc;
  array[o + 2] = (xz - wy) * sc;
  array[o + 3] = 0;
  array[o + 4] = (xy - wz) * sc;
  array[o + 5] = (1 - (xx + zz)) * sc;
  array[o + 6] = (yz + wx) * sc;
  array[o + 7] = 0;
  array[o + 8] = (xz + wy) * sc;
  array[o + 9] = (yz - wx) * sc;
  array[o + 10] = (1 - (xx + yy)) * sc;
  array[o + 11] = 0;
  array[o + 12] = x;
  array[o + 13] = y;
  array[o + 14] = z;
  array[o + 15] = 1;
}
```

**Replacement** for 1694–1717:

```javascript
const sporeColorKey = `${artifact.cacheKey}|${isFault ? 1 : 0}|${speciesId}`;
const sporeColorsDirty = sporeColorKey !== lastSporeColorKey;
if (sporeColorsDirty) lastSporeColorKey = sporeColorKey;

const sporeArray = sporeMesh.instanceMatrix.array;
const pixelCount = Math.max(1, rgbPixels.length);
const sc = isDone ? 1.15 : isExhausted ? 0.5 : 0.85;
for (let s = 0; s < SPORE_COUNT; s += 1) {
  const u = ((s / SPORE_COUNT) + ambient * 0.04 * (0.4 + liveFuelRatio)) % 1;
  const v = Math.sin(s * PHI_GOLDEN_ANGLE + ambient * 0.8);
  embedOnSpeciesSurface(speciesId, u, v, s % 16, 0.03, sporeScratch, 0);
  const outScale = isExhausted ? 0.88 : 1.08;
  writeSporeMatrix(
    sporeArray, s,
    sporeScratch[0] * outScale,
    sporeScratch[1] * outScale,
    sporeScratch[2] * outScale,
    ambient + s,
    ambient * 0.7 + s,
    sc
  );
  if (sporeColorsDirty) {
    const px = rgbPixels[s % pixelCount];
    if (px && px.length >= 3) _color.setRGB(px[0] / 255, px[1] / 255, px[2] / 255);
    else _color.setHex(species.primaryHex);
    if (isFault) _color.lerp(_faultCrimson, 0.7);
    sporeMesh.setColorAt(s, _color);
  }
}
sporeMesh.instanceMatrix.needsUpdate = true;
if (sporeColorsDirty && sporeMesh.instanceColor) sporeMesh.instanceColor.needsUpdate = true;
```

`_faultCrimson` must be the constant `0xe11d48` and must not be overwritten elsewhere. `Color.lerp` writes into `_color` only.

At init (after 671), set usage once, before the first render. r175 ignores a later `setUsage` change after the buffer has been uploaded (`BufferAttribute` docs: usage cannot be changed after first use):

```javascript
if (sporeMesh.instanceColor) {
  sporeMesh.instanceColor.setUsage(THREE.DynamicDrawUsage);
  sporeMesh.instanceColor.needsUpdate = true;
}
```

---

## F7. Nautilus septa: pose is a function of `chamberCount` only

**Lines:** 1465–1497, geometry alloc at 773.

`u`, `tubeR`, `getPointAt(u)`, and the quaternion from `(0, 1, 0)` toward the tangent do not read `ambient`. They change only when `chamberCount` changes. `chamberCount` is `clamp((entryMorph.entryN ?? 4) + 1, 2, 10)`, which is fixed for a compiled artifact.

Color and `emissiveIntensity` change with `callDepth`, `isDone`, `frameCount`, `ruleLine`, and `controlKindIsReturn(step)`. Track those. Do not call the spline unless the pose key changes.

`getPointAt(u, target)` and `getTangentAt(u, target)` exist and will not allocate if `target` is a live `Vector3`.

**Replacement** scratch:

```javascript
const _septumPoint = new THREE.Vector3();
const _septumTangent = new THREE.Vector3();
const _septumUp = new THREE.Vector3(0, 1, 0);
let lastChamberCount = -1;
let lastRibColorKey = '';
let lastSeptumColorKey = '';

function layoutNautilusChambers(chamberCount) {
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
    nautilusSpline.getPointAt(u, _septumPoint);
    nautilusSpline.getTangentAt(u, _septumTangent);
    ch.mesh.position.copy(_septumPoint);
    ch.mesh.quaternion.setFromUnitVectors(_septumUp, _septumTangent);
    ch.mesh.scale.set(tubeR, 1.0, tubeR);
  }
}
```

`setFromUnitVectors` does not allocate. It expects unit vectors. `getTangentAt` returns a unit tangent. `_septumUp` is already unit length. Do not normalize them again.

Inside the factorial branch, replace 1469–1496 with:

```javascript
const chamberCount = clamp((entryMorph.entryN ?? 4) + 1, 2, MAX_NAUTILUS_CHAMBERS);
if (chamberCount !== lastChamberCount) {
  lastChamberCount = chamberCount;
  layoutNautilusChambers(chamberCount);
}
const ribKey = `${callDepth}|${chamberCount}|${isDone ? 1 : 0}`;
const ribColorsDirty = ribKey !== lastRibColorKey;
if (ribColorsDirty) lastRibColorKey = ribKey;
const unlockU = clamp((callDepth + 1) / chamberCount, 0.35, 1.0);
for (let i = 0; i < nautilusRibs.length; i += 1) {
  const item = nautilusRibs[i];
  const unlocked = item.u <= unlockU || isDone;
  const pulse = unlocked ? 1.0 + 0.06 * Math.sin(ambient * 3.0 - item.u * 6.0) : 0.82;
  item.mesh.scale.setScalar(item.tubeR * pulse); // see F10: unit torus scaled by tubeR
  if (ribColorsDirty) {
    item.mat.color.setHex(isDone ? 0x34d399 : unlocked ? 0xfbbf24 : 0x78350f);
    item.mat.emissiveIntensity = unlocked ? 0.42 : 0.14;
  }
}
const septumKey = `${callDepth}|${frameCount}|${ruleLine}|${isDone ? 1 : 0}|${controlKindIsReturn(step) ? 1 : 0}`;
if (septumKey !== lastSeptumColorKey) {
  lastSeptumColorKey = septumKey;
  for (let c = 0; c < chamberCount; c += 1) {
    const ch = nautilusChambers[c];
    const activeChamber = c < callDepth || isDone;
    const lockedGold = c < frameCount && (ruleLine === 149 || controlKindIsReturn(step));
    ch.mat.color.setHex(isDone ? 0x2dd4bf : lockedGold ? 0xfde047 : activeChamber ? 0xf59e0b : 0x475569);
    ch.mat.emissiveIntensity = activeChamber ? 0.68 : 0.18;
  }
}
```

If F10 is not applied, keep `item.mesh.scale.setScalar(pulse)` and keep the per-rib `TorusGeometry(tubeR, tubeR * 0.22)`. The pose cache does not depend on sharing the geometry.

`nautilusGroup.rotation.y = Math.sin(ambient * 0.6) * 0.22` stays every frame.

Visibility of septa with `c >= chamberCount` is handled inside `layoutNautilusChambers`. When `chamberCount` is unchanged they stay hidden. No per-frame `visible` writes.

---

## F8. Silk and jellyfish tentacles: typed centerlines, quiet colors

**Lines:** 1498–1552 (silk and zooids), 1610–1649 (bell and tentacles).

Both already run only for the active species. Positions must move every frame. Colors must not.

Silk (1507–1512): `x = lerp(-0.4, 0.4, u)` is fixed per segment. `zOff = (st - 1) * 0.032` is fixed per strand. `y` and the cosine term depend on `ambient` and, for `y` only, on `podCount`.

```javascript
const silkX = new Float32Array(SILK_SEGMENTS);
const silkU = new Float32Array(SILK_SEGMENTS);
for (let s = 0; s < SILK_SEGMENTS; s += 1) {
  silkU[s] = s / (SILK_SEGMENTS - 1);
  silkX[s] = -0.4 + 0.8 * silkU[s];
}
let lastSilkColorDone = -1;

// replaces 1505–1530, still inside speciesId === 'list_fold':
const silkColorsDirty = (isDone ? 1 : 0) !== lastSilkColorDone;
if (silkColorsDirty) lastSilkColorDone = isDone ? 1 : 0;
for (let st = 0; st < SILK_STRANDS; st += 1) {
  const zOff = (st - 1) * 0.032;
  const yPhase = st * 0.4;
  for (let s = 0; s < SILK_SEGMENTS; s += 1) {
    const u = silkU[s];
    const o = s * 3;
    ribbonCenter[o] = silkX[s];
    ribbonCenter[o + 1] = Math.sin(u * Math.PI * podCount - ambient * 2.6 + yPhase) * 0.072;
    ribbonCenter[o + 2] = zOff + Math.cos(u * TAU * 1.5 - ambient * 2.0) * 0.045;
  }
  if (silkColorsDirty) _color.setHex(isDone ? 0x2dd4bf : st === 1 ? 0x22d3ee : 0xa855f7);
  writeRibbonStrip(
    silkPositions, silkColors, silkNormals,
    st, SILK_SEGMENTS, ribbonCenter,
    0.034, _color.r, _color.g, _color.b, TAPER_28, GLOW_28, silkColorsDirty
  );
}
silkGeom.attributes.position.needsUpdate = true;
silkGeom.attributes.normal.needsUpdate = true;
if (silkColorsDirty) silkGeom.attributes.color.needsUpdate = true;
```

When `silkColorsDirty` is false, `_color` still holds the previous strand’s color if you skip `setHex`. Either always `setHex` (it does not allocate; `_color` is reused) or cache three rgb triples. Always calling `setHex` and passing `writeColor = false` is the small, safe version: the color buffer is not rewritten and `needsUpdate` stays false. The snippet above must call `setHex` every strand even when colors are clean, **or** cache rgb, because `writeRibbonStrip` would otherwise receive a stale `_color` on a dirty frame’s later strand only if you skip `setHex`. Simplest correct form: call `_color.setHex` every strand every frame (no alloc) and pass `silkColorsDirty` as `writeColor`.

Zooid bells (1532–1551): position and pulse scale stay per frame. `bellMat.color.setHex` and `emissiveIntensity` depend on `isDone` and `isCurrent` (`zIdx === activePodIdx`), which are stable for a step. Gate them with a key `${isDone}|${activePodIdx}|${podCount}` plus the per-pod `bellR` (that depends on `listVals`, i.e. the artifact). `bellR` changes the scale, which is already applied every frame via `pulse`. Color gate is the only extra skip.

Jelly tentacles (1618–1646): same split. Precompute per tentacle `baseA`, `baseRad`. Per segment store nothing you cannot recompute cheaply; the loop is 160 iterations and the win is avoiding `Vector3` and skipping the color attribute.

```javascript
const tentacleBaseA = new Float32Array(JELLY_TENTACLE_COUNT);
const tentacleBaseRad = new Float32Array(JELLY_TENTACLE_COUNT);
for (let t = 0; t < JELLY_TENTACLE_COUNT; t += 1) {
  tentacleBaseA[t] = (t / 16) * TAU;
  tentacleBaseRad[t] = t % 2 === 0 ? 0.24 : 0.16;
}
let lastTentacleFault = -1;

// inside checked_underflow, replaces 1618–1649:
const tentacleColorsDirty = (isFault ? 1 : 0) !== lastTentacleFault;
if (tentacleColorsDirty) lastTentacleFault = isFault ? 1 : 0;
const swayAmp = isFault ? 0.045 : 0.022;
const flareK = isFault ? 0.14 : 0;
for (let t = 0; t < JELLY_TENTACLE_COUNT; t += 1) {
  const baseA = tentacleBaseA[t];
  const baseRad = tentacleBaseRad[t];
  for (let s = 0; s < JELLY_TENTACLE_SEGMENTS; s += 1) {
    const u = s / (JELLY_TENTACLE_SEGMENTS - 1);
    const sway = Math.sin(ambient * 2.2 + t * 0.7 + u * 4.0) * swayAmp * u;
    const rad = baseRad + sway + flareK * u;
    const twist = baseA + Math.cos(ambient * 1.6 + t + u * 3.0) * 0.12 * u;
    const o = s * 3;
    ribbonCenter[o] = Math.cos(twist) * rad;
    ribbonCenter[o + 1] = 0.02 + (-0.38 - 0.02) * u;
    ribbonCenter[o + 2] = Math.sin(twist) * rad;
  }
  _color.setHex(isFault ? 0xe11d48 : t % 2 === 0 ? 0xfb7185 : 0xfbbf24);
  writeRibbonStrip(
    jellyTentaclePos, jellyTentacleCol, jellyTentacleNorm,
    t, JELLY_TENTACLE_SEGMENTS, ribbonCenter,
    0.0075, _color.r, _color.g, _color.b, TAPER_10, GLOW_10, tentacleColorsDirty
  );
}
jellyTentacleGeom.attributes.position.needsUpdate = true;
jellyTentacleGeom.attributes.normal.needsUpdate = true;
if (tentacleColorsDirty) jellyTentacleGeom.attributes.color.needsUpdate = true;
```

`lerp(0.02, -0.38, u)` expanded is `0.02 + (-0.38 - 0.02) * u`. Same value.

Bell `scale`, `color.setHex`, and `emissiveIntensity` (1613–1616): scale must update (it uses `Math.sin(ambient * 3.2)`). Hex and intensity change only with `isFault`. Gate those two writes with `tentacleColorsDirty` (same boolean).

---

## F9. Dirty-check the cheap `update()` bookkeeping

These are smaller than F1–F8. They are still per-frame allocations or redundant material writes. None of them change a pixel when the gated inputs are unchanged.

### Visibility without `Object.entries` (1320–1322)

```javascript
const speciesKeys = Object.keys(speciesGroups); // once, after speciesGroups is filled
let lastVisibleSpecies = '';

// inside update:
if (speciesId !== lastVisibleSpecies) {
  for (let i = 0; i < speciesKeys.length; i += 1) {
    const key = speciesKeys[i];
    speciesGroups[key].visible = key === speciesId;
  }
  lastVisibleSpecies = speciesId;
}
```

`Object.keys` once at init, not per frame. Setting `visible` to the value it already has is a write the renderer does not need; skipping it is the same picture.

### Rule id string (1344–1359)

`ruleId` is only used for `isDone` and `isExhausted`. If `step.ruleId` is missing, the code synthesizes `A080` from `ruleLine === 80` and `A084` from `ruleLine === 84`, which the numeric tests already cover. If `step.ruleId` is present, it must still be honored.

```javascript
const ruleId = step?.ruleId || '';
const isDone = ruleLine === 80 || ruleId === 'A080';
const isFault = ruleLine === 82 || ruleLine === 140 || ruleLine === 225;
const isExhausted = ruleLine === 84 || ruleId === 'A084';
const isInPlaceRewrite = ruleLine === 149 || ruleLine === 145 || ruleLine === 177;
const isAluPrim = ruleLine === 223 || ruleLine === 225;
```

No `padStart`, no template string. Same booleans.

### Light color and intensity (1367–1370)

Neither expression reads `ambient`. Gate on `${speciesId}|${isFault}|${isDone}|${isExhausted}|${callDepth}|${species.primaryHex}`.

```javascript
let lastLightKey = '';
const lightKey = `${speciesId}|${isFault ? 1 : 0}|${isDone ? 1 : 0}|${isExhausted ? 1 : 0}|${callDepth}`;
if (lightKey !== lastLightKey) {
  lastLightKey = lightKey;
  corePointLight.color.setHex(
    isFault ? ROLES_1_HEX.faultCrimson : isDone ? ROLES_1_HEX.reportGreen : species.primaryHex
  );
  corePointLight.intensity = isDone ? 1.15 : isFault ? 1.25 : isExhausted ? 0.35 : 0.8 + callDepth * 0.08;
}
```

### Tree, moth, lotus, comet, tokamak material writes

| Block | Lines | Updates every frame (keep) | Stable for a step (gate) |
| --- | --- | --- | --- |
| Coral branches | 1559–1565 | nothing | `emissiveIntensity` from `progress`, `isDone` |
| Polyp crowns | 1566–1577 | `scale` (`sin(ambient * 3.2 + c)`) | `emissiveIntensity` |
| Moth wings | 1578–1590 | pivot rotation, Lorenz `rotation.z` | `emissiveIntensity` from `evenActive`, `isDone` |
| Nested cubes | 1595–1603 | `rotation.y`, `rotation.x` | `emissiveIntensity` |
| Lotus petals | 1604–1609 | `rotation.z` (has `sin(ambient * 2 + …)`) | `emissiveIntensity` |
| Comet | 1650–1674 | nothing. No `ambient` in this block | entire block: `ringCount`, `crushRatio`, `frameCount`, `isFault` |
| Tokamak braid | 1678–1680 | `rotation.z`, `emissiveIntensity` (`liveFuelRatio`) | `color.setHex` (`isExhausted` only) |
| Fuel cells | 1681–1688 | nothing continuous | `setHex` / opacity when `hasFuel` flips |

`litCells = Math.round(liveFuelRatio * 14)` can change mid-step because `liveFuelRatio` lerps with `stepFraction`. Track a `Uint8Array(14)` of the last `hasFuel` bit and write that cell’s material only when the bit flips. Cells that stay lit keep the same hex `0x38bdf8`, emissive `0x0284c7`, intensity `0.58`, opacity `0.92`.

Comet replacement for 1650–1674: compute a key `` `${ringCount}|${frameCount}|${isFault ? 1 : 0}|${artifact.cacheKey}` `` and skip the loop when it matches. `crushRatio` is `clamp(frameCount / ringCount, 0.15, 1.0)` and does not use `q`. Positions, scales, and hexes are constant for that key.

Damping (1336–1342) can snap once the error is below `1e-4` and then stop writing scale / opacity / y-offset:

```javascript
function dampToward(current, target, dampMix) {
  const next = current + (target - current) * dampMix;
  return Math.abs(next - target) < 1e-4 ? target : next;
}
```

Snapping to the exact target is the value the exponential was approaching. Below `1e-4` on a root whose radius is `0.44`, the leftover is under `5e-5` world units. After the snap, the value **is** the target, so settled frames match the intended scale exactly. Skip `coreRoot.scale.setScalar` when `dampedScale` did not change. `position.y` and `rotation.y` still update every frame because of the sine breath.

---

## F10. Share procedural geometries that differ only by scale

**Lines:** 744–757 (20 rib tori), 773 (10 septa cylinders), 848 (6 zooid spheres), 858 (6 zooid tori), 919 (4 polyp core spheres), 1041 (3 boxes), 1217 (10 comet rings).

These are init costs and GPU memory, not per-frame CPU. They do not change shading: `MeshPhysicalMaterial` is still per mesh, so per-object color and emissive stay unique. Only the vertex buffer is shared.

A `TorusGeometry(R, 0.22 * R)` scaled by `pulse` is the same mesh as `TorusGeometry(1, 0.22)` scaled by `R * pulse`. The current rib update **replaces** scale with `pulse` because `R` is baked into the geometry. After sharing, scale must be `tubeR * pulse` (F7 already writes that).

```javascript
const nautilusRibGeom = new THREE.TorusGeometry(1, 0.22, 10, 22);
const nautilusSeptumGeom = new THREE.CylinderGeometry(1, 0.9, 0.012, 20);
// in the rib loop, replace `new THREE.TorusGeometry(tubeR, tubeR * 0.22, 10, 22)`:
const rib = new THREE.Mesh(nautilusRibGeom, ribMat);
rib.scale.setScalar(tubeR); // first frame, before update()
// septum loop: `new THREE.Mesh(nautilusSeptumGeom, sepMat)`

const zooidBellGeom = new THREE.SphereGeometry(1, 24, 18);
const zooidRingGeom = new THREE.TorusGeometry(1.15, 0.11, 10, 28);
// bell = new THREE.Mesh(zooidBellGeom, bellMat)
// ring = new THREE.Mesh(zooidRingGeom, ringMat); ring.rotation.y = Math.PI * 0.5

const polypCoreGeom = new THREE.SphereGeometry(0.044, 16, 12);
// coreBulb = new THREE.Mesh(polypCoreGeom, cMat)

const unitCubeGeom = new THREE.BoxGeometry(1, 1, 1);
// cMesh = new THREE.Mesh(unitCubeGeom, cMat); cMesh.scale.setScalar(cubeSizes[c]);
// group scale animation at 1598 stays `nc.group.scale.setScalar(seedScale)`.
// World size = seedScale * cubeSizes[c], same as a BoxGeometry of that size
// under a group scale of seedScale.

const cometRingGeom = new THREE.TorusGeometry(0.22, 0.013, 10, 32);
// rMesh = new THREE.Mesh(cometRingGeom, rMat); rMesh.rotation.x = Math.PI * 0.5
```

Approximate vertex grids, `(radialSegments + 1) * (tubularSegments + 1)` for tori and `(widthSegments + 1) * (heightSegments + 1)` for spheres:

| Primitive | Copies now | Grid | Vertices kept |
| --- | --- | --- | --- |
| Nautilus rib torus | 20 | 11 × 23 = 253 | 253 instead of ~5060 |
| Septum cylinder | 10 | one lathed cylinder | 1 buffer instead of 10 |
| Zooid sphere | 6 | 25 × 19 = 475 | 475 instead of ~2850 |
| Zooid torus | 6 | 11 × 29 = 319 | 319 instead of ~1914 |
| Polyp core sphere | 4 | 17 × 13 = 221 | 221 instead of ~884 |
| Nested cube | 3 | 24 unique corners, different sizes | 1 box; size on `mesh.scale` |
| Comet shock torus | 10 | 11 × 33 = 363 | 363 instead of ~3630 |

Do not share geometries that are then **mutated** with `geometry.scale` or `geometry.translate` (that would alias every user). `polypPetalGeom.scale(...)` at 907 is already a one-time bake on the shared petal. Leave it. Do not call `scale` on `nautilusRibGeom`.

`createScallopedWingGeometry` (582–599) is already called twice and shared. Leave it.

---

## F11. Indices, cylinder scratch, buffer usage, static bounds

### `buildRibbonIndices` (395–408)

`setIndex` on a JS array walks it and copies into a `Uint16BufferAttribute` (`BufferGeometry.js` 230–234). All four meshes are under 65535 vertices. Build the typed array directly:

```javascript
function buildRibbonIndices(curveCount, segments) {
  const indexArray = new Uint16Array(curveCount * (segments - 1) * 6);
  let w = 0;
  for (let c = 0; c < curveCount; c += 1) {
    const base = c * segments * 2;
    for (let s = 0; s < segments - 1; s += 1) {
      const i0 = base + s * 2;
      indexArray[w++] = i0;
      indexArray[w++] = i0 + 2;
      indexArray[w++] = i0 + 1;
      indexArray[w++] = i0 + 1;
      indexArray[w++] = i0 + 2;
      indexArray[w++] = i0 + 3;
    }
  }
  return new THREE.Uint16BufferAttribute(indexArray, 1);
}
```

Triangle order is unchanged: `(i0, i2, i1, i1, i2, i3)` with `i1 = i0 + 1`, `i2 = i0 + 2`, `i3 = i0 + 3`.

### `alignCylinderBetween` (413–422)

Called only at init (894), six times, but each call does `new THREE.Vector3` twice, and line 755 does another `new THREE.Vector3(0, 0, 1)` per rib. Scratch is enough. The vectors are not retained.

```javascript
const _cylDir = new THREE.Vector3();
const _cylUp = new THREE.Vector3(0, 1, 0);
const _torusAxis = new THREE.Vector3(0, 0, 1);

function alignCylinderBetween(mesh, pA, pB, radius = 0.025) {
  const dx = pB.x - pA.x;
  const dy = pB.y - pA.y;
  const dz = pB.z - pA.z;
  const len = Math.max(0.001, Math.hypot(dx, dy, dz));
  mesh.position.set((pA.x + pB.x) * 0.5, (pA.y + pB.y) * 0.5, (pA.z + pB.z) * 0.5);
  mesh.scale.set(radius, len, radius);
  _cylDir.set(dx / len, dy / len, dz / len);
  mesh.quaternion.setFromUnitVectors(_cylUp, _cylDir);
}
```

Line 755 becomes `rib.quaternion.setFromUnitVectors(_torusAxis, tangent)`. `getTangentAt(u, _septumTangent)` can be used at init too so those 20 tangent vectors are not discarded.

### `DynamicDrawUsage` before first render

Set usage at `setAttribute` time. Attributes that stream every frame for at least one species: SDK position and normal, silk position and normal, jelly position and normal, spore `instanceColor`. Tendril position and normal are static after F5; leave them on the default `StaticDrawUsage`. Tendril color changes on species / fault / done; `DynamicDrawUsage` on that one attribute avoids a stall when the fault spine snaps.

```javascript
function streamAttr(array) {
  return new THREE.BufferAttribute(array, 3).setUsage(THREE.DynamicDrawUsage);
}
sdkRibbonGeom.setAttribute('position', streamAttr(sdkRibbonPositions));
sdkRibbonGeom.setAttribute('normal', streamAttr(sdkRibbonNormals));
sdkRibbonGeom.setAttribute('color', streamAttr(sdkRibbonColors));
// same for silkGeom and jellyTentacleGeom position + normal + color
// tendril color only:
tendrilGeom.setAttribute('color', streamAttr(tendrilColors));
```

Position and normal of the tendrils stay `new THREE.BufferAttribute(tendrilPositions, 3)` (static).

### Bounding spheres, set once

The renderer uses `geometry.boundingSphere` and computes it once if null (`three.webgpu.js` around the `computeBoundingSphere` call). The InstancedMesh path does **not** expand that sphere by instance matrices. A spore octahedron of radius `0.012` at the origin will cull every spore when the origin’s tiny sphere leaves the frustum while the shells are still on screen. Ribbons start as a zero-filled `Float32Array`; if bounds were computed before the first `update`, the sphere would be empty.

Set a padded sphere at creation. Loose spheres cost an occasional extra draw. Tight spheres cull real pixels. Prefer loose.

```javascript
sdkRibbonGeom.boundingSphere = new THREE.Sphere(new THREE.Vector3(0, 0, 0), 0.6);
silkGeom.boundingSphere = new THREE.Sphere(new THREE.Vector3(0, 0, 0), 0.6);
jellyTentacleGeom.boundingSphere = new THREE.Sphere(new THREE.Vector3(0, -0.18, 0), 0.6);
// Tendrils: x,z within ~0.60, y from -1.33 to +1.80. Center y = 0.235, radius ~1.70.
tendrilGeom.boundingSphere = new THREE.Sphere(new THREE.Vector3(0, 0.235, 0), 1.75);
// Spores sit on the same shells, outScale up to 1.15, plus the 0.012 octahedron.
sporeGeom.boundingSphere = new THREE.Sphere(new THREE.Vector3(0, 0, 0), 0.7);
```

Do not null these when attributes stream. Do not call `computeBoundingSphere()` per frame.

`addUpdateRange` (r175, float-element start/count) could upload only jelly SDK ribbons `rIdx >= 6`. A vec3 WebGPU buffer may be padded to vec4, and a wrong range corrupts vertices. The static-species path already uploads **nothing** on the steady frame. Leave partial ranges out unless a byte-level check confirms the write. Full uploads of the animated buffers are 6–12 KB and are the correct safe choice.

---

## What not to change

- Do not drop `crests: true` or lower the SDK ridge point counts. `sampleSdkRidgePoint` displays the quantized polyline. Fewer points move the ribbons.
- Do not skip the per-frame centerline for `factorial`, `list_fold`, `mutual_even_odd`, or jellyfish tentacles (`rIdx >= 6`). `sdkWave` is in the surface formula.
- Do not replace the ribbons with a shader displacement. The materials are `MeshPhysicalMaterial` with vertex colors and explicit normals. A shader that is “close” is a different picture. The typed CPU path writes the same normals.
- Do not recycle the `Vector3`s stored in `nautilusCurvePts`, `lorenzPts`, or `braidPts`. The curves keep those references.
- Do not turn off `DoubleSide`, transparency, or iridescence to save fill rate. That is a material change.
- Do not instance the eight species together. Only one species is visible, and their materials differ.
- `quinticStep` (55–58) is one call per frame. Leave it.
- Editing `quinelingsSdk.js` is out of this pass. `reuse` is a supported argument of `WovenBody.frame`. The ridge object graph is consumed here, so flattening in this file is the safe cut. `juliaFrame` and `recursiveFrame` will still allocate on a step change; that cost is once per step, not once per frame.

---

## Suggested order

1. F6 fault `Color` hoist and F7 septum target vectors. Small diffs, remove the worst allocs, easy to see that the nautilus and the fault spores did not move.
2. F1 artifact cache and F9 rule-id / `Object.entries` / light key. No geometry change.
3. F3 out-parameter embed, then F2 flatten. Keep `writeRibbonStrip` on `Vector3` until F2 is checked, by writing the typed centerline back into the existing `_sdkPts[s]` if you want a halfway step. The end state is F4’s typed strip.
4. F4 static-species skip and color dirty flag.
5. F5 tendril yaw. Check the sign on the first frame: `rotation.y = -ambient * 0.1`.
6. F8 silk and tentacles.
7. F10 shared geometries. Rib scale must become `tubeR * pulse` in the same change.
8. F11 usage flags and bounding spheres, in the same change as the streaming attributes, before the first render.

---

## Parity checks

No automated test covers this designer. After each step, compare against the current file on the same trace, paused and playing, for all eight presets.

- Factorial: septum discs stay on the tube; they do not crawl per frame. Ribs still pulse. Active chamber gold (`0xfde047`) still tracks `frameCount` and Return/Done.
- List fold: silk still travels along X with the sine; zooid bells stay on that wave. Color flips to `0x2dd4bf` only when the step is done, not as a per-frame flicker.
- Tree, lotus, comet, tokamak: SDK ribbon mesh is bit-stable across frames of one step (positions and normals). Colors change on the step, not on the clock. Comet rings do not drift while `frameCount` is held.
- Moth and nautilus and jellyfish: ribbons still crawl. Moth z-offset is `sdkWave * 0.04` on top of a stable x/y. Jelly meridians (`rIdx < 6`) do not crawl; tentacles do.
- Tendrils: one full turn of `ambient` by `2π / 0.1` seconds matches a full revolution of the old vertex bake. Floor roots still twist `-0.35` along their length and crown roots `+0.35`. Fault recolors to `0xe11d48`, done to `0x2dd4bf`, otherwise primary on the floor and secondary on the crown. They must not spin the wrong way (`+ambient * 0.1` on `rotation.y` is the wrong sign).
- Spores: fault tint matches `lerp(rgb, 0xe11d48, 0.7)`. Non-fault frames do not change spore color when the camera clock advances. Scale still drops to `0.5` on exhaust and rises to `1.15` on done.
- Switching presets shows only one species group. Returning to a preset restores the same ribbon.
- Interior floor / up camera presets still damp scale to `0.58` / `0.52` and tendril opacity to `0.08` / `0.06`, and `interior_up` still drops the root by `0.2`.

A practical diff: for a static species, log `sdkRibbonPositions` checksum at two `ambient` values on the same step. It must match. For tendrils, checksum of positions must match across `ambient`, while `rotation.y` changes. For factorial septa, `mesh.position` must match across `ambient` and change when `entryN` changes.
