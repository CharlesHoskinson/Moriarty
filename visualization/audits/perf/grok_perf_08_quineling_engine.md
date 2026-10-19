# Performance audit 08 — Quineling SDK and Sherlock-to-Quineling compiler

Scope: `src/engine/quinelingsSdk.js` (1031 lines) and `src/engine/sherlockQuinelingCompiler.js` (708 lines). No source file was modified.

Invariant for every change below: same numbers, same canonical strings, same genome bytes, same ridge samples, same bounds. Sample counts stay put (`Morphology.portraitFrame` 16×49×3, `LifeformFamilies.portraitFrame` 32×81×5, pleated `WovenBody.portraitFrame` 128×257, crest rows/cols, Lorenz/Clifford/Julia budgets). `fnv1a32` keeps `& 0xff`. That mask is the seed; changing it moves every woven coordinate.

## Where the time actually goes

`compileSherlockProgramToQuineling` already memoizes the whole artifact (`compiledQuinelingCache`, line 588). After the first miss, `resolveCompiledQuineling` / `getQuinelingTelemetry` are a `Map` lookup. Scrubbing does **not** recompile while source, stack, fuel, and outcome stay fixed.

The expensive calls are:

1. **Cache miss (program change).** Universal work is `Quinelings.execute` + two extra `canon` passes (`encode`, `encodeColors`) and `Morphology.portraitFrame` (no anatomy on `QDL.create`, so every species takes the 16-phase ribbon fit). Species work stacks on top: pleated portrait, family-adapter portrait, `integrateLorenz`, `compileClifford`, `compileJulia`, or `recursiveBounds`.
2. **Step-index change.** `designer8_quineling.js` lines 1375–1388 call `WovenBody.frame(body, stepIndex * 0.08 * TAU, { budget: 256, crests: true })` and keep **only** `wf.ridges`. The 256-point cloud is discarded. Crest construction is the scrub cost. Animation frames that stay on the same step do not call `frame`.

| Species | Mechanism | Compile spike | Per new step |
|---|---|---|---|
| factorial | logarithmic-mantle | `Families.portraitFrame` + morphology fit | `compressionGrid` + 15×161 ridge points per chart |
| list_fold | pleated-braid | pleated portrait, phase loop innermost | `caustics`: 144×40 `differential` per strand |
| tree_fold | recursive-affine | `recursiveBounds` 64×branches×5×3 | `recursiveCaustics` + scroll rows |
| mutual_even_odd | lorenz-flow | `integrateLorenz` 512+3200 RK4 steps | Lorenz `caustics`: 480×20 `differential` per strand |
| lexical_let | phyllotaxis-fan | portrait × `strands * whorls` | same, chart count is `strands * whorls` |
| checked_underflow | clifford-flow | burn 320 + 8192 iterates, grid 96, 6 critical images | `cliffordFrame` warp of retained marks |
| stack_overflow | recursive-julia | 128² × up to 48 iterates | contour warp |
| fuel_exhaustion | toroidal-weave | family portrait, no material grid | ridges, no `compressionGrid` |

`budget: 256` is noise next to those crest loops. An opt-in that skips the point cloud is listed last and does not change the default return value.

## P0 — Memo key is short, so a hit can return a different quineling

`compileSherlockProgramToQuineling` lines 601–608:

```javascript
const stackLimit = trace?.steps?.[0]?.after?.stackLimit ?? trace?.stackLimit ?? 16;
const initialFuel = trace?.steps?.[0]?.before?.fuel ?? trace?.fuel ?? 120;
const outcomeKind = trace?.outcome?.kind || 'Done';
const cacheKey = `${cleanSource}|S:${stackLimit}|F:${initialFuel}|O:${outcomeKind}`;
```

The artifact also depends on:

- `stepsUsedVal` (line 429): `Math.max(1, trace?.steps?.length || 12)`. That literal is inside the task graph, so it changes `canon`, both genomes, and `rgbPixels` (the spores `designer8` draws).
- `terminalRuleLine` (lines 124–125): last step `ruleLine`, default 80. It selects `speciesId` in the chain at lines 128–155 (rule 225, 140, 84).
- `speciesCatalog` (line 624). The key ignores it. A second catalog object for the same source returns the first catalog’s family and mechanism.
- The `|` delimiter. A source that itself contains `|S:` / `|F:` / `|O:` can share a key with a different triple.

`Map.has` + `Map.get` (lines 606–608) is two lookups. A hit does not refresh order, so eviction at lines 702–705 drops the oldest insertion, including a program the user is still scrubbing.

Replace the key block and the hit path:

```javascript
const CATALOG_STAMPS = new WeakMap();
let nextCatalogStamp = 1;

function catalogStamp(catalog) {
  if (!catalog || typeof catalog !== 'object') return 0;
  let id = CATALOG_STAMPS.get(catalog);
  if (id === undefined) {
    id = nextCatalogStamp;
    nextCatalogStamp += 1;
    CATALOG_STAMPS.set(catalog, id);
  }
  return id;
}

function quinelingCacheKey(cleanSource, speciesCatalog, trace) {
  const stackLimit = trace?.steps?.[0]?.after?.stackLimit ?? trace?.stackLimit ?? 16;
  const initialFuel = trace?.steps?.[0]?.before?.fuel ?? trace?.fuel ?? 120;
  const outcomeKind = trace?.outcome?.kind || 'Done';
  const stepsUsedVal = Math.max(1, trace?.steps?.length || 12);
  const terminalRuleLine = trace?.steps?.[trace.steps.length - 1]?.ruleLine ?? 80;
  // JSON.stringify is injective on this array of strings and finite numbers.
  return JSON.stringify([
    cleanSource,
    catalogStamp(speciesCatalog),
    stackLimit,
    initialFuel,
    outcomeKind,
    terminalRuleLine,
    stepsUsedVal,
  ]);
}

export function compileSherlockProgramToQuineling(source, speciesCatalog, trace = null) {
  const cleanSource = String(
    source ||
      trace?.source ||
      'language "moriarty-sherlock-draft/1"; program Factorial { fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(4); }'
  );
  const cacheKey = quinelingCacheKey(cleanSource, speciesCatalog, trace);
  const cached = compiledQuinelingCache.get(cacheKey);
  if (cached !== undefined) {
    compiledQuinelingCache.delete(cacheKey);
    compiledQuinelingCache.set(cacheKey, cached);
    return cached;
  }
  const stackLimit = trace?.steps?.[0]?.after?.stackLimit ?? trace?.stackLimit ?? 16;
  const initialFuel = trace?.steps?.[0]?.before?.fuel ?? trace?.fuel ?? 120;
  const outcomeKind = trace?.outcome?.kind || 'Done';
  // ... rest of the function unchanged, including stepsUsedVal at the fuel witness ...
```

Reuse the same `stepsUsedVal` / `terminalRuleLine` locals inside `analyzeSherlockAst` / `buildQuinelingTaskGraphFromAst` so the key and the graph cannot drift. `catalogStamp` assumes a catalog object is not mutated after the first compile (`QUINELING_SPECIES` is frozen).

On insert, keep the size cap (lines 702–705). Refresh-on-read makes that cap an LRU of artifacts, which is what scrubbing wants.

## P0 — One canonical string, one genome

Lines 640–650 and 687:

```javascript
const quineCanonical = Quinelings.canon(quineProgram);
const quineProof = Quinelings.execute(quineProgram, { constructionOnly: true });
// ...
const harmonicGenome = Quinelings.encode(quineProgram);      // canon() again
const colorGenome = Quinelings.encodeColors(quineProgram);   // canon() a third time
// ...
quineBytes: new TextEncoder().encode(quineCanonical).length,
```

`encode` (SDK line 778) and `encodeColors` (SDK line 794) each call `canon` + `new TextEncoder()`. `encodeColors` is the byte-to-RGB map of the harmonic bands and nothing else:

```javascript
pixels: g.bands.map(a => a.map(c => {
  if (c === 0) return null;
  const b = c - 1;
  return [b, 255 - b, (73 * b + 19) % 256];
}))
```

`execute` still has to `canon` the reconstructed term (SDK line 769). That object is a different tree, so an identity cache would not hit it. The original program is canonicalized three times.

In `quinelingsSdk.js`, hoist one encoder next to `const module = undefined;` (line 2) and split `encode` (lines 777–794):

```javascript
const TEXT_ENCODER = new TextEncoder();

function genomeFromCanonical(text) {
  const data = TEXT_ENCODER.encode(text);
  if (data.length > 65536) throw Error('Genome exceeds 64 KiB');
  const bytes = new Uint8Array(12 + data.length);
  bytes.set([81, 76, 78, 71]);
  const view = new DataView(bytes.buffer);
  view.setUint32(4, data.length);
  bytes.set(data, 8);
  view.setUint32(8 + data.length, checksum(data));
  const bands = [];
  for (let i = 0; i < bytes.length; i += 32) {
    const a = new Array(32);
    const end = Math.min(i + 32, bytes.length);
    let j = 0;
    for (; i + j < end; j += 1) a[j] = bytes[i + j] + 1;
    for (; j < 32; j += 1) a[j] = 0;
    bands.push(a);
  }
  return { format: 'quineling-harmonics-1', bands, byteLength: data.length };
}

function colorGenomeFromHarmonic(g) {
  return {
    format: 'quineling-chroma-1',
    pixels: g.bands.map((row) => row.map((c) => {
      if (c === 0) return null;
      const b = c - 1;
      return [b, 255 - b, (73 * b + 19) % 256];
    })),
  };
}

function encode(program) {
  const packed = genomeFromCanonical(canon(program));
  return { format: packed.format, bands: packed.bands };
}

function encodeColors(program) {
  return colorGenomeFromHarmonic(encode(program));
}
```

`bytes.slice(i, i + 32)` (line 781) copies a subarray per band and then `forEach` writes it. Indexing `bytes` fills the same 32 coefficients, including the zero pad. `encode` / `encodeColors` return shapes stay `{format, bands}` and `{format, pixels}`.

Export `genomeFromCanonical` and `colorGenomeFromHarmonic` on the `Quinelings` api (line 833). Compiler replacement for lines 648–649 and 687:

```javascript
const packedGenome = Quinelings.genomeFromCanonical(quineCanonical);
const harmonicGenome = { format: packedGenome.format, bands: packedGenome.bands };
const colorGenome = Quinelings.colorGenomeFromHarmonic(harmonicGenome);
// quineBytes: packedGenome.byteLength
```

`TextEncoder.encode(quineCanonical).length` equals `packedGenome.byteLength` because both encode that same string.

## P0 — Pleated portrait rebuilds the basis cache on every sample

`WovenBody.portraitFrame` (line 1026), generic branch (list_fold / pleated-braid only; Lorenz, Clifford, Julia, recursive, and adapters return earlier):

```javascript
for (let k = 0; k < body.lanes.length; k++)
  for (let j = 0; j <= 256; j++)
    for (let h = 0; h < 128; h++) {
      const q = point(body, k, j / 256, 0, TAU * h / 128, true),
            c = [q.cx, q.cy, q.cz],
            e = [q.ex, q.ey, q.ez];
      // ...
    }
```

That is `lanes × 257 × 128` samples. Phase `h` is the inner index. `basis` (line 1004) does:

```javascript
if (!cache || cache.phase !== phase)
  body.chartCache = cache = { phase, lanes: body.lanes.map(() => new Map()) };
```

Every sample changes phase, so every sample allocates a cache object and one `Map` per lane, then throws it away. `point(..., true)` also spreads the basis (`{...b, ex, ey, ez}` around line 1008) and the loop allocates `c` and `e`. The bounds only need six numbers. `Math.min` / `Math.max` over the same finite set do not depend on visit order.

Replace the generic loop with phase as the outer index, and read `basis` directly:

```javascript
const lo = [Infinity, Infinity, Infinity];
const hi = [-Infinity, -Infinity, -Infinity];
for (let h = 0; h < 128; h += 1) {
  const phase = TAU * h / 128;
  for (let k = 0; k < body.lanes.length; k += 1) {
    for (let j = 0; j <= 256; j += 1) {
      const u = j / 256;
      const b = basis(body, k, u, phase);
      let ex;
      let ey;
      let ez;
      if (b.family === 'lorenz') {
        ex = ey = ez = 1.18 * b.width;
      } else {
        ex = (1 - b.roll) * 1.32 * b.width * Math.abs(b.cc)
          + b.roll * b.width * Math.sqrt(b.cc * b.cc + 0.7 * 0.7 * b.ss * b.ss);
        ey = (1 - b.roll) * b.width * (1.32 * Math.abs(b.ss) + 0.025)
          + b.roll * b.width * Math.sqrt(b.ss * b.ss + 0.7 * 0.7 * b.cc * b.cc);
        ez = b.width * 0.7;
      }
      lo[0] = Math.min(lo[0], b.cx - ex);
      lo[1] = Math.min(lo[1], b.cy - ey);
      lo[2] = Math.min(lo[2], b.cz - ez);
      hi[0] = Math.max(hi[0], b.cx + ex);
      hi[1] = Math.max(hi[1], b.cy + ey);
      hi[2] = Math.max(hi[2], b.cz + ez);
    }
  }
}
```

The `ex` / `ey` / `ez` formulas are the `bounds === true` branch of `point` (line 1008), copied so the spread disappears. Each `(k, u, phase)` is still evaluated once. Within one phase the existing `chartCache` now survives all 257 longitudinal samples instead of being discarded after each one.

## P0 — Crest and grid loops allocate a result object per discarded sample

### Family adapter (`LifeformFamilies`)

`point` (lines 867–878) always returns a fresh `{x,y,z,alpha,owner}`. Callers that only need coordinates:

- `compressionGrid` (lines 883–886) reads `q.x` and `q.y`, then drops the object. `rows * columns` is 48×15 per chart, and `materialGrid` is keyed by phase (line 883), so a new step phase rebuilds it. Factorial and lexical_let pay this; toroidal-weave deletes `material` (compiler lines 513–515) and skips it.
- `portraitFrame` (line 898) reads `q.x/y/z` and allocates `[-1,-.5,0,.5,1]` on every `(h,k,j)` because the array literal sits inside the `j` loop.
- `frame` (lines 892–896) does `points.set([q.x,q.y,q.z,q.alpha], j*4)`, which allocates a 4-array per sample. Ridge vertices (`line.push(q)`, line 896) must stay fresh objects. Those are the retained geometry.

`owner` (line 850) already buckets territories, then allocates a closure on every `.find`.

Hoist the portrait ordinates once:

```javascript
const PORTRAIT_VS = [-1, -0.5, 0, 0.5, 1];
const _xy = { x: 0, y: 0, z: 0, alpha: 0, owner: -1 };
```

Add `pointInto(body, k, u, v, phase, out, skipOwner)` containing the current `point` formulas, writing `out.x/y/z/alpha/owner` and returning `out`. `point` becomes `return pointInto(..., { x:0, y:0, z:0, alpha:0, owner:-1 }, skipOwner)` so existing callers still receive a new object.

`compressionGrid` and `portraitFrame` reuse `_xy` and read the fields before the next call. For the grid, skip `alpha` and `owner` (`skipOwner` is already `true`); `x` and `y` do not depend on them. Keep the Jacobian arithmetic on the typed `x` / `y` buffers exactly as written.

`owner` without a closure:

```javascript
function owner(body, k, u) {
  let components = componentCaches.get(body);
  if (!components) {
    components = Array.from({ length: body.record.strands }, () => []);
    for (const p of body.territories) components[p.component].push(p);
    componentCaches.set(body, components);
  }
  const group = components[k % body.record.strands];
  for (let i = 0; i < group.length; i += 1) {
    const p = group[i];
    if (u >= p.u[0] && (u < p.u[1] || (u === 1 && p.u[1] === 1))) return p.owner;
  }
}
```

Patches on one component do not overlap (`validate` enforces that), so the first match is the only match. Same predicate as line 850, including `&&` / `||` precedence.

`frame` sample store (and the node-patch `points.set` just below it):

```javascript
const o = j * 4;
points[o] = q.x;
points[o + 1] = q.y;
points[o + 2] = q.z;
points[o + 3] = q.alpha;
owners[j] = q.owner;
```

Same four-index store in `cliffordFrame` (line 935), `juliaFrame` (line 951), `recursiveFrame` (line 965), and the pleated `frame` (line 1018).

### Pleated / Lorenz `differential` (line 1007) and `caustics` (line 1012)

`caustics` calls `differential(...).nz` once per column (144×40 per strand, or 480×20 for Lorenz), then `point` calls `differential` again for each retained root. `differential` allocates `du`, the `transverseDerivative` array, `chartAt` objects, and the result record. `dv` does not escape. `q` is only read for its numbers.

Use module scratches (`_chartQ`, `_chartA`, `_chartB`, `_dv = [0,0,0]`, `_diff`) and write `chartAt` / `transverseDerivative` into them. Copy `a.x/y/z` into locals before reusing a scratch. Keep the three divisions `(b.x - ax) / (ub - ua)` as divisions. A multiplied reciprocal is not bit-identical. `point` copies `q.x/y/z`, normals, `compression`, and `owner` into the fresh ridge object before the next `chartAt`. `caustics` reads `_diff.nz` the same way it reads `differential().nz`.

`transverseDerivative` and `chartAt` are not re-entrant, and `caustics` does not nest `differential` inside `point`. A scratch is safe. Do not scratch `recursiveScrollRow`: `recursiveJacobianRow` (line 976) returns a closure that retains the row.

### `ownerAt` (line 998)

Every pleated/Lorenz sample scans all territories. Bucket once per body. Disjoint patches make scan order irrelevant:

```javascript
const ownerBuckets = new WeakMap();
function ownerAt(body, k, u) {
  let buckets = ownerBuckets.get(body);
  if (!buckets) {
    buckets = Array.from({ length: body.record.strands }, () => []);
    for (const p of body.territories) buckets[p.component].push(p);
    ownerBuckets.set(body, buckets);
  }
  const group = buckets[k];
  for (let i = 0; i < group.length; i += 1) {
    const p = group[i];
    if (u >= p.u[0] && (u < p.u[1] || (u === 1 && p.u[1] === 1))) return p.owner;
  }
}
```

## P1 — Lorenz RK4 and Clifford iterate allocate a vector per substep

`integrateLorenz` (line 994), used for mutual_even_odd (`burn: 512`, `steps: 3200` in the compiler record, lines 534–545):

```javascript
const f = ([x, y, z]) => [d.sigma * (y - x), x * (d.rho - z) - y, x * y - d.beta * z];
const add = (a, b, m) => a.map((x, i) => x + m * b[i]);
// each step: 4×f, 3×add, 1×state.map, plus orbit.push
```

3712 steps × 8 short arrays. Scalar RK4 with the same operation order is bit-identical. `orbit.push([x*0.036, (25-z)*0.03, y*0.009])` stays, because `orbitPoint` indexes `p[i][k]`.

```javascript
function integrateLorenz(d) {
  const dt = d.dt;
  let x = d.initial[0];
  let y = d.initial[1];
  let z = d.initial[2];
  const orbit = [];
  for (let i = 0; i < d.burn + d.steps; i += 1) {
    const a0 = d.sigma * (y - x);
    const a1 = x * (d.rho - z) - y;
    const a2 = x * y - d.beta * z;
    const bx = x + (dt / 2) * a0, by = y + (dt / 2) * a1, bz = z + (dt / 2) * a2;
    const b0 = d.sigma * (by - bx);
    const b1 = bx * (d.rho - bz) - by;
    const b2 = bx * by - d.beta * bz;
    const cx = x + (dt / 2) * b0, cy = y + (dt / 2) * b1, cz = z + (dt / 2) * b2;
    const c0 = d.sigma * (cy - cx);
    const c1 = cx * (d.rho - cz) - cy;
    const c2 = cx * cy - d.beta * cz;
    const dx = x + dt * c0, dy = y + dt * c1, dz = z + dt * c2;
    const e0 = d.sigma * (dy - dx);
    const e1 = dx * (d.rho - dz) - dy;
    const e2 = dx * dy - d.beta * dz;
    x = x + dt * (a0 + 2 * b0 + 2 * c0 + e0) / 6;
    y = y + dt * (a1 + 2 * b1 + 2 * c1 + e1) / 6;
    z = z + dt * (a2 + 2 * b2 + 2 * c2 + e2) / 6;
    check(
      Number.isFinite(x) && Number.isFinite(y) && Number.isFinite(z)
        && Math.abs(x) < 100 && Math.abs(y) < 100 && Math.abs(z) < 100,
      'integration escaped finite admitted enclosure'
    );
    if (i >= d.burn) orbit.push([x * 0.036, (25 - z) * 0.03, y * 0.009]);
  }
  return orbit;
}
```

`d.dt / 2` matches `add(state, k, d.dt / 2)`. The initial values are copied once, which is what `d.initial.slice()` did.

`lorenzBasis` (line 997) calls `orbitPoint` and `orbitTangent` (lines 995–996). Each builds an array with `[0,1,2].map`, then `t.map`, plus `N` and `B`. `Math.hypot(...t)` spreads that array. `basis` caches the returned object, so `N` and `B` are retained once per `(k, u, phase)`. The temporaries are not.

Compute `i` and `t` once per index and write scalars. `t*t` once is the same value as three evaluations of `t*t`. `t2 * t` matches `t*t*t`.

```javascript
function orbitSample(body, index) {
  const p = body.orbit;
  const n = p.length;
  const i = Math.max(1, Math.min(n - 3, Math.floor(index)));
  const t = Math.max(0, Math.min(1, index - i));
  const t2 = t * t;
  const t3 = t2 * t;
  const q = [0, 0, 0];
  const tv = [0, 0, 0];
  for (let k = 0; k < 3; k += 1) {
    const a = p[i - 1][k], b = p[i][k], c = p[i + 1][k], d = p[i + 2][k];
    q[k] = 0.5 * ((2 * b) + (-a + c) * t + (2 * a - 5 * b + 4 * c - d) * t2 + (-a + 3 * b - 3 * c + d) * t3);
    tv[k] = 0.5 * ((-a + c) + 2 * (2 * a - 5 * b + 4 * c - d) * t + 3 * (-a + 3 * b - 3 * c + d) * t2);
  }
  return { q, tv };
}
```

Keep `orbitPoint` / `orbitTangent` as wrappers that return those arrays if anything else calls them, or inline `orbitSample` into `lorenzBasis` and set `cx/cy/cz` from `q[k]` and `N`/`B` from `tv` normalized with `Math.hypot(tv[0], tv[1], tv[2])`.

`compileClifford` (line 927) assigns `[x, y] = mapStep(d, x, y)` once per iterate (burn 320 + 8192). `mapStep` (line 926) returns a new pair. The retained store is `cloud.push([x, y])`, which is already a fresh pair. Inline the two scalar assignments in the burn loop:

```javascript
const nx = Math.sin(a * y) + c * Math.cos(a * x);
const ny = Math.sin(b * x) + e * Math.cos(b * y);
x = nx;
y = ny;
```

Leave `mapStep` in place for `forward()` inside `criticalCurves` (line 930). Those returned pairs are stored on the curves.

Line 928:

```javascript
body.maxDensity = Math.max(...cells.map((c) => c.count));
```

`densityGrid` is 96 (compiler line 555), so this spreads 9216 numbers and allocates the mapped array. Counts are nonnegative and `cells` is non-empty:

```javascript
let maxDensity = 0;
for (let i = 0; i < cells.length; i += 1) {
  if (cells[i].count > maxDensity) maxDensity = cells[i].count;
}
body.maxDensity = maxDensity;
```

## P1 — Morphology fit allocates on every ribbon sample

`Morphology.portraitFrame` (lines 647–661) runs for every Sherlock compile. `QDL.create` does not set `design.anatomy`, so the anatomy early-return does not fire. The fit is `16` phases × `ribbonCount` (default ribbons 28, line 619) × `49` × `3` calls to `surfacePoint`. Each `surfacePoint` (lines 620–646) calls `strandPoint` three times, `project` three times, and `pose` twice. `strandPoint` (line 553) returns `livingPose(...)`, which allocates `{x,y}` (line 546). `project` allocates `{x,y}` (line 551). The `v` loop is `for (const v of [-1,0,1])` inside `(t,k,i)`, so the 3-array is reallocated per sample.

`fitCache` (line 647) makes the second call free. The first call is the cost. `motionState` is already passed in as `m`, so the per-sample path does not re-enter it.

Give `livingPose`, `project`, and `pose` an optional `out` object. When `out` is omitted, allocate as they do now (callers of `anchor` / `surfaceFrame` keep a fresh point). Inside `surfacePoint` and `portraitFrame`, reuse two scratches and copy `x,y` into locals before the next write.

`pose` (line 618) when called with `m.projection` must not rebuild the cosine record. It already receives `projection`. Write:

```javascript
function pose(p, z, d, projection, out) {
  const c = projection || {
    cy: Math.cos(d.composition.yaw),
    sy: Math.sin(d.composition.yaw),
    cp: Math.cos(d.composition.pitch),
    sp: Math.sin(d.composition.pitch),
    lean: d.composition.lean,
  };
  const o = out || {};
  const x = p.x * c.cy + z * c.sy;
  const depth = z * c.cy - p.x * c.sy;
  const y = p.y * c.cp - depth * c.sp;
  o.x = x + c.lean * y;
  o.y = y;
  o.z = depth;
  return o;
}
```

Portrait’s second loop (line 655) allocates two corner objects per node per phase. `r` is nonnegative. Replace with:

```javascript
minX = Math.min(minX, p.x - r);
maxX = Math.max(maxX, p.x + r);
minY = Math.min(minY, p.y - r);
maxY = Math.max(maxY, p.y + r);
```

Hoist `const PORTRAIT_V = [-1, 0, 1]` and the phase table. `Array.from({length:16}, ...)` (line 652) once per fit is fine.

`motionState` (lines 532–534) allocates `Object.keys(DEFAULT_RHYTHM)` on every call, including cache hits:

```javascript
const RHYTHM_KEYS = Object.keys(DEFAULT_RHYTHM);
// in the hit test, replace Object.keys(DEFAULT_RHYTHM).every(...) with:
RHYTHM_KEYS.every((key) => cached.rhythm[key] === authored[key])
```

Unroll `yaw` / `pitch` / `lean` the same way. Hit results stay the cached state object.

`anchor` (lines 601–602) filters peers by level on every call:

```javascript
const peers = s.nodes.filter((x) => !x.parent && x.level === n.level);
const index = Math.max(0, peers.findIndex((x) => x.id === n.id));
```

Portrait calls this `16 × nodeCount` times. A `WeakMap` from shape to `Map<level, nodes>` filled in node order matches `filter` order:

```javascript
const peersByShape = new WeakMap();
function peersAtLevel(s, level) {
  let pack = peersByShape.get(s);
  if (!pack) {
    pack = new Map();
    for (const x of s.nodes) {
      if (x.parent) continue;
      let row = pack.get(x.level);
      if (!row) {
        row = [];
        pack.set(x.level, row);
      }
      row.push(x);
    }
    peersByShape.set(s, pack);
  }
  return pack.get(level) || EMPTY_PEERS;
}
```

Do not hang this list on `s` as an enumerable field. `Chroma.compile` stringifies selected fields only today; an extra enumerable key would still be a trap.

## P1 — `clone` is `JSON.parse(JSON.stringify)` on every quine and kernel step

Three copies of the same helper:

- Orbit `clone`, line 7. `QuinelingKernels` and `Quinelings` use this one (`const clone = O.clone`, lines 77 and 702).
- Anatomy `copy`, line 154.
- QDL `clone`, line 327.

`makeTaskProgram` (lines 734–736) clones the graph and the design. `execute` clones every `quote` / `makeQuote` / `makeApply` (lines 762, 770–772). `run` (line 136) clones every input record and every value, and constructs `new TextEncoder()` **inside the step loop**.

On the admitted domain (plain objects, plain arrays, finite numbers, strings, booleans, `null`) a structural clone matches JSON. `-0` must become `+0`, because `JSON.stringify(-0)` is `"0"`. Non-finite numbers become `null`. `undefined`, functions, and symbols are omitted on objects and become `null` in arrays. Anything with `toJSON`, a custom prototype, or a cycle falls back to `JSON.parse(JSON.stringify(x))`, which preserves today’s throw.

```javascript
function clone(x) {
  if (x === null) return null;
  const t = typeof x;
  if (t === 'number') {
    if (!Number.isFinite(x)) return null;
    return x === 0 ? 0 : x;
  }
  if (t === 'string' || t === 'boolean') return x;
  if (t !== 'object') return JSON.parse(JSON.stringify(x));
  if (typeof x.toJSON === 'function') return JSON.parse(JSON.stringify(x));
  const proto = Object.getPrototypeOf(x);
  if (Array.isArray(x)) {
    if (proto !== Array.prototype) return JSON.parse(JSON.stringify(x));
    const out = new Array(x.length);
    for (let i = 0; i < x.length; i += 1) {
      const v = x[i];
      out[i] = (v === undefined || typeof v === 'function' || typeof v === 'symbol') ? null : clone(v);
    }
    return out;
  }
  if (proto !== Object.prototype && proto !== null) return JSON.parse(JSON.stringify(x));
  const out = {};
  const keys = Object.keys(x);
  for (let i = 0; i < keys.length; i += 1) {
    const k = keys[i];
    const v = x[k];
    if (v === undefined || typeof v === 'function' || typeof v === 'symbol') continue;
    out[k] = clone(v);
  }
  return out;
}
```

Use this body for all three helpers. Key order follows `Object.keys`, which is the order `JSON.stringify` uses for ordinary records. Do not add a `WeakMap` canon cache on top: `canon` walks whatever the object is now, and a mutated graph must not replay an old string.

`boundedJSON` (lines 90–97) copies `seen` at every node (`new Set(seen).add(value)`). A single set with add/delete on the way back sees the same ancestors and does not see siblings, which is what the copy achieved:

```javascript
function boundedJSON(value, depth = 0, seen = new Set()) {
  requireThat(depth <= 24, 'JSON depth exceeds 24');
  if (value === null || typeof value === 'boolean') return;
  if (typeof value === 'string') {
    requireThat(value.length <= 16384, 'String too long');
    return;
  }
  if (typeof value === 'number') {
    number(value);
    return;
  }
  requireThat(value && typeof value === 'object', 'Expected finite JSON');
  requireThat(!seen.has(value), 'Cyclic value');
  seen.add(value);
  if (Array.isArray(value)) {
    array(value);
    for (let i = 0; i < value.length; i += 1) boundedJSON(value[i], depth + 1, seen);
  } else {
    const keys = Object.keys(value);
    requireThat(keys.length <= 512, 'Record too large');
    for (let i = 0; i < keys.length; i += 1) {
      key(keys[i]);
      boundedJSON(value[keys[i]], depth + 1, seen);
    }
  }
  seen.delete(value);
}
```

`forEach` order and `Object.keys` order are unchanged.

## P1 — Kernel `run` copies the pending set every step

`run` lines 133–137:

```javascript
while (pending.size) {
  const n = [...pending].find((n) => n.inputs.every((id) => values.has(id)));
  // ...
  requireThat(new TextEncoder().encode(canon(value)).length <= 65536, 'Task value exceeds 64 KiB');
}
```

`new Set(g.nodes)` iterates in node order. Spreading it and taking the first ready node is “lowest index whose inputs are done”. A `Uint8Array` scan from index 0 picks that same node. `inputs.map` order stays the input order. `TEXT_ENCODER` is the hoisted encoder from the genome section (kernels live in a later IIFE; close over the module-level encoder, or hoist one encoder inside the kernels IIFE).

```javascript
function run(graph, overrides = {}) {
  validate(graph);
  record(overrides);
  const g = clone(graph);
  for (const [id, value] of Object.entries(overrides)) {
    const n = g.nodes.find((node) => node.id === id);
    requireThat(n && n.op === 'literal', 'Overrides may change literals only');
    boundedJSON(value);
    n.params.value = clone(value);
  }
  validate(g);
  const values = new Map();
  const trace = [];
  const effects = [];
  const nodes = g.nodes;
  const done = new Uint8Array(nodes.length);
  let remaining = nodes.length;
  let steps = 0;
  while (remaining) {
    let n = null;
    let nIndex = -1;
    for (let i = 0; i < nodes.length; i += 1) {
      if (done[i]) continue;
      const candidate = nodes[i];
      let ready = true;
      for (let k = 0; k < candidate.inputs.length; k += 1) {
        if (!values.has(candidate.inputs[k])) {
          ready = false;
          break;
        }
      }
      if (ready) {
        n = candidate;
        nIndex = i;
        break;
      }
    }
    requireThat(n && ++steps <= 64, 'Task stuck or out of fuel');
    const inputs = new Array(n.inputs.length);
    for (let k = 0; k < n.inputs.length; k += 1) inputs[k] = values.get(n.inputs[k]);
    const value = calculate(n.op, inputs, n.params);
    boundedJSON(value);
    requireThat(TEXT_ENCODER.encode(canon(value)).length <= 65536, 'Task value exceeds 64 KiB');
    values.set(n.id, value);
    done[nIndex] = 1;
    remaining -= 1;
    trace.push({ edge: n.id, rule: n.op, inputs: clone(inputs), value: clone(value) });
    if (n.op === 'action' && value.status === 'simulated') effects.push(clone(value));
  }
  return { output: g.outputs.map((id) => clone(values.get(id))), trace, effects, graph: g };
}
```

`validate` (line 100) also does `new TextEncoder().encode(canon(graph))`. Use `TEXT_ENCODER` there too. The canonical bytes do not change.

`calculate` line 110, `Math[op](...v)` for `min` / `max`, spreads up to 512 numbers. A loop of `Math.min` / `Math.max` on a non-empty finite array matches:

```javascript
case 'min':
case 'max': {
  const v = numbers(a[0]);
  requireThat(v.length > 0, 'Extremum requires values');
  let m = v[0];
  for (let i = 1; i < v.length; i += 1) m = op === 'min' ? Math.min(m, v[i]) : Math.max(m, v[i]);
  return m;
}
```

`compare` (line 89) runs `canon` for `eq` / `ne`. For finite numbers, strings, booleans, and `null`, `a === b` matches `canon` equality (`-0` and `0` both stringify as `0` and `===` each other; `JSON.stringify` is injective on strings). Keep `canon` for arrays and records.

Sherlock task graphs are under 30 nodes, so this is smaller than a portrait. It still runs on every compile (`QuinelingKernels.run(graph)`, compiler line 650), and `validate` runs again inside `run` after the clone (line 134) and inside `makeTaskProgram` (line 735).

## P1 — `describe` walks every link once per node

`describe` (lines 812–826) is called once per compile on the quine. `isV1Program` (lines 709–722) walks that same tree first, then `execute` has already walked it. Degree and depth then do:

```javascript
function depth(id, path = new Set()) {
  // ...
  const next = new Set(path).add(id);
  const inputs = links.filter((e) => e.to === id);
  const d = inputs.length ? 1 + Math.max(...inputs.map((e) => depth(e.from, next))) : 0;
  // ...
}
nodes.forEach((n, i) => {
  n.outdegree = links.filter((e) => e.from === n.id).length;
  n.indegree = links.filter((e) => e.to === n.id).length;
  // ...
});
```

Build adjacency once, in link order, so each node’s input list matches `filter`:

```javascript
const incoming = new Map();
const outgoing = new Map();
for (const n of nodes) {
  incoming.set(n.id, []);
  outgoing.set(n.id, []);
}
for (const e of links) {
  incoming.get(e.to).push(e);
  outgoing.get(e.from).push(e);
}
const stack = new Set();
function depth(id) {
  if (depths.has(id)) return depths.get(id);
  if (stack.has(id)) throw Error('Unbounded cycle');
  stack.add(id);
  const inputs = incoming.get(id);
  let d = 0;
  for (let i = 0; i < inputs.length; i += 1) d = Math.max(d, 1 + depth(inputs[i].from));
  stack.delete(id);
  depths.set(id, d);
  return d;
}
```

Empty inputs leave `d` at 0. Non-empty inputs yield `1 + max(child depths)`, the same value as line 824’s `inputs.length ? 1 + Math.max(...) : 0`. The add/delete stack matches the copied `path` set: a node is on the stack only while its ancestors are being resolved, and a memoized node is not re-entered.

Then:

```javascript
n.outdegree = outgoing.get(n.id).length;
n.indegree = incoming.get(n.id).length;
```

`n.frequency`, `n.level`, `n.u`, and `n.side` stay as they are (lines 826).

`isV1Program` on a tree that this file just built is pure overhead the second time (`describe` after `execute`). A `WeakMap` from object to the boolean is the same answer while the object is not mutated. `makeTaskProgram` returns a fresh tree and the compiler does not mutate it before `describe` / `encode`. If a later caller mutates a program and asks again, the cached boolean would be stale. Prefer the `WeakMap` only if programs stay immutable after construction; otherwise leave the walk. Do not stamp the program with a non-enumerable flag: `isV1Program` rejects hidden keys via `Reflect.ownKeys` (line 718).

## P2 — Compiler AST walk allocates a regex and a BigInt per expression

`analyzeSherlockAst` (lines 45–211) walks `programImage.expressions` once, then again for entry literals (lines 158–168). `buildQuinelingTaskGraphFromAst` walks them a third time (line 297) to emit nodes. The third walk does different work and should stay. The entry-literal walk is the same `Literal` nodes.

Hoist the patterns next to `fnv1a32`. The global sanitize pattern is only used with `replace`, which resets `lastIndex`.

```javascript
const FN_OWNER_RE = /^fn\[\d+\]:([^/]+)/;
const DIGITS_RE = /^\d+$/;
const LIST_MATCH_RE = /\bnil\b|\bcons\s*\(/;
const TREE_MATCH_RE = /\bleaf\s*\(|\bbranch\s*\(/;
const SANITIZE_NON_ID_RE = /[^A-Za-z0-9_-]+/g;
const SANITIZE_LEADING_RE = /^[^A-Za-z]+/;

function uint32Literal(rawText) {
  // 15 digits is always <= 999999999999999 < 2^53, so Number is exact.
  // `>>> 0` is the same low 32 bits as BigInt(raw) & 0xffffffffn.
  if (rawText.length < 16) return Number(rawText) >>> 0;
  return Number(BigInt(rawText) & 0xffffffffn);
}

function ownerFnName(astPath) {
  if (typeof astPath !== 'string') return null;
  const m = FN_OWNER_RE.exec(astPath);
  return m ? m[1] : null;
}
```

Use `ownerFnName(ex.astPath)` at line 65, `DIGITS_RE` / `uint32Literal` at lines 70–71 and 164–165, and the match patterns at lines 85–86. Point `sanitizeNodeId` (lines 33–38) at the hoisted sanitize patterns.

Declare `entryLits` and `entryExpr` before the main loop and fill them in the `Literal` case and on the first `astPath === 'entry/'`. Delete the second loop (lines 161–168) and the `exprs.find` (line 158):

```javascript
let entryExpr = null;
const entryLits = [];
// inside case 'Literal':
const rawText = String(ex.text || '').trim();
if (DIGITS_RE.test(rawText)) {
  const n = uint32Literal(rawText);
  literals.push(n);
  if (typeof ex.astPath === 'string' && ex.astPath.startsWith('entry/')) entryLits.push(n);
} else if (rawText === 'true' || rawText === 'false') {
  literals.push(rawText === 'true' ? 1 : 0);
}
// once per expression, before the switch:
if (entryExpr === null && ex.astPath === 'entry/') entryExpr = ex;
// after the loop:
entryExpr = entryExpr || exprs[0] || null;
```

Push order follows `exprs`, which is the order of the deleted loop. `entryN`, `listValues`, `treeLeaves`, `seedValue`, and `subGap` (lines 169–176) then read the same `entryLits`.

In `buildQuinelingTaskGraphFromAst`:

- `allocId` (lines 225–231) calls `sanitizeNodeId` twice on a collision. Sanitize once into `baseId` and append `_suffix`.
- `reportInputs.includes` / `reportLabels.includes` (lines 286–291) are length ≤ 8. A parallel `Set` for membership preserves array order, which is the report order. Small.
- `exprs.filter` for literals (line 256) only needs `literalExprs[0]`. Take the first `Literal` while scanning, instead of allocating the filtered array. `lits.slice(0, 6)` (line 258) can be a bounded push into a 6-slot array.

`authorSpeciesWovenRecord` (line 479) calls `WovenBody.author(graph, seed)` and keeps `topology`, `strands`, and `territories` only. Every Sherlock graph has an `action` whose first input is a `compare`, so `topology(...).guards > 0` (SDK line 917) and `author` builds a Clifford dynamics record (SDK line 923) that the species record then replaces (compiler lines 496–582). `author` does not integrate; the waste is the discarded record plus a second graph walk (`nodes` then `topology`).

`WovenBody.topology` is exported (line 1027). The strand and territory loops in `author` (lines 919–922) do not use the RNG or the mechanism:

```javascript
function authorSpeciesWovenRecord(graph, speciesMeta, seed) {
  const t = WovenBody.topology(graph);
  const strands = Math.max(8, Math.min(14, 7 + t.fanout + Math.floor(t.convergence / 2)));
  const ns = graph.nodes;
  const territories = [];
  for (let k = 0; k < strands; k += 1) {
    const assigned = ns.filter((n, i) => i % strands === k);
    if (!assigned.length) assigned.push(ns[k % ns.length]);
    assigned.forEach((n, i) => {
      territories.push({
        node: n.id,
        component: k,
        u: [i / assigned.length, (i + 1) / assigned.length],
      });
    });
  }
  // record construction continues exactly as lines 485–582, using t, strands, territories
```

`LifeformFamilies.authorParams` has its own `rng(seed)` (SDK line 845). Skipping `author` does not change that sequence.

`WovenBody.validate(record, graph)` at compiler line 584 runs again as the first line of `WovenBody.compile` (SDK line 925). The validate implementation sorts local group arrays, not `record.territories`. Drop line 584. A bad record still throws from `compile`, with the same `WovenBody:` message.

## P2 — `frame` is pure in `(body, phase, budget, crests)` and the UI only keeps ridges

`juliaFrame` (line 951) and `recursiveFrame` (line 965) ignore `reuse`. `Families.frame`, `cliffordFrame`, and pleated `frame` honor it. Thread the same `reuse.points` / `reuse.owners` length check through Julia and recursive so a caller can recycle the typed arrays. Default `reuse` stays undefined, so today’s arrays are still freshly allocated and filled with the same values.

`WovenBody.frame` (line 1015) has no phase memo. Designer8 caches one step key, so stepping backward recomputes a phase that was built a moment ago. Crest objects are not mutated by the sampler (`sampleSdkRidgePoint` copies `x,y,z,owner`). Later `frame` calls write `body.chartCache` and `body.materialGrid`; they do not write into previously returned ridge objects.

```javascript
const frameMemo = new WeakMap();

function frame(body, phase, options = {}) {
  const budget = options.budget ?? 22000;
  const crests = options.crests ?? true;
  if (!options.reuse && body) {
    let memo = frameMemo.get(body);
    if (!memo) {
      memo = new Map();
      frameMemo.set(body, memo);
    }
    const key = (crests ? '1:' : '0:') + budget + ':' + (Object.is(phase, -0) ? '-0' : String(phase));
    const hit = memo.get(key);
    if (hit) return hit;
    const built = frameUncached(body, phase, { budget, crests });
    memo.set(key, built);
    if (memo.size > 12) memo.delete(memo.keys().next().value);
    return built;
  }
  return frameUncached(body, phase, options);
}
```

`frameUncached` is the current `frame` body. A hit skips both the 256-point fill and the crest rebuild. Treat the returned `points`, `owners`, and `ridges` as immutable. `options.reuse` bypasses the memo so the caller’s buffer is still written.

Optional, default off, so the mathematical return value of `frame` stays intact: `options.ridgesOnly === true` skips the budget point loop and returns empty `points` / `owners` of length 0 while building ridges with the same formulas. Designer8 would pass it. Without that flag, keep filling the 256 samples exactly as now. Do not change the default.

## P2 — Recursive bounds and scroll rows

`recursiveBounds` (line 991):

```javascript
for (let h = 0; h < 64; h++) {
  const phase = TAU * h / 64;
  for (let k = 0; k < body.branches.length; k++)
    for (const u of [0, 0.25, 0.5, 0.75, 1])
      for (const v of [-1, 0, 1]) {
        const p = recursivePoint(body, k, u, v, phase);
        // min/max p.x, p.y, p.z
      }
}
```

Phase is already outer, and `recursivePose` caches that phase (line 959). The two array literals are inside the branch loop: `64 × branches` allocations. Hoist `const REC_U = [0, 0.25, 0.5, 0.75, 1]` and `const REC_V = [-1, 0, 1]`. Read `p.x/y/z` from a scratch filled by `recursivePoint` only in this bounds loop; ridge `recursivePoint` calls must still return a new object.

`recursiveScrollRow` (line 961) is pure in `(body, k, u, phase)`. `recursiveCaustics` calls it once per `u` via `recursiveJacobianRow`, then `recursivePoint` calls it again for each root and each crest sample. Cache the last row on a `WeakMap` keyed by body plus `(k, u, phase)` using `Object.is` for `u` and `phase`. Callers only read the row. Returning the same row object is the same coefficients.

## What not to change

- `fnv1a32` (compiler lines 22–28), including `& 0xff`. The seed at line 209 is `(fnv1a32(source) % 2147483646) + 1`.
- Portrait and crest sample counts, Lorenz `dt` / `burn` / `steps`, Clifford `iterations` / `densityGrid`, Julia `grid` / `iterations`. Fewer samples change bounds or the drawn ridges.
- `basis` float keys and `cache.phase !== phase` versus `Object.is` (line 1004 vs lifeform line 851). `-0` handling differs on purpose between the two caches.
- Quine `execute` still evaluates the term and `canon`s the emission. That string is the proof at compiler lines 641–643.
- Do not freeze the cached artifact. Callers hold `exprToNodeId`, `graph`, and `wovenBody` and a freeze would throw on a write that works today. The cache returns the same object; that is the point of the memo.

## Suggested order

1. Fix the compile cache key (correctness of the memo) and derive the color genome from one canonical pack.
2. Reorder pleated `portraitFrame` and stop spreading basis objects. This is the list_fold compile spike.
3. Scratch `point` / `differential` / `chartAt`, bucket `ownerAt`, and replace `points.set([…])`. This is every step change.
4. Scalar Lorenz RK4 and Clifford `mapStep` in the burn loop. One species each, once per compile.
5. Morphology `out` parameters for `portraitFrame`. Every species, once per compile.
6. Structural `clone`, hoisted `TextEncoder`, kernel scan, `describe` adjacency, AST regex/BigInt fusion.
7. `frame` LRU and `reuse` on Julia/recursive frames.
