# Upstream Quinelings SDK patch — browser export and 3D frames

Auditor: Grok 4.7 (`xhigh`), agent 5 of 5. Repository: `/home/charles/repos/quinelings`. No source files were edited. Findings below were read from source and checked with Node against `lifeform-families.js`, `woven-body.js`, `morphology.js`, and `programs/generated/tideglass.json`.

Apply these in `/home/charles/repos/quinelings`, then rebuild the browser bundle. Keep every change additive. `tests/verify-woven-body.cjs` and `verify-morphology.cjs` compare frames with `deepEqual` and `strictEqual` on the cached portrait object. New fields are safe. Changing existing `points` values, ridge `line` objects, or the default `WovenBody.author(graph, seed)` record is not.

## 1. Export `Morphology` and `ThoughtCompiler` from the browser entry

Severity: high. Browser and WebGPU callers cannot import the two modules they need.

`packages/agent-sdk/src/browser-entry.ts` is the entry `scripts/build-ranch-browser.mjs` bundles to `assets/ranch/quinelings-runtime.js`. Its named export list is `Runtime`, `QuinelingError`, `Anatomy`, `QDL`, `Quinelings`, `QuinelingKernels`, `QuinelingOffspring`, `QuinelingWorld`, `V1Session`, `QDLV1`, `QDLV1Library`, `Chroma`, `migrateLegacy`, `V1Ranch`, `WovenBody`, `VisualCapsule`, `LifeformFamilies`.

The built file ends at the same list (`assets/ranch/quinelings-runtime.js` around lines 27781–27799). `Morphology` and `ThoughtCompiler` are absent.

Both modules are already evaluated inside that bundle. `morphology.js` assigns `root.Morphology` (bundle near line 1923) with `surfaceFrame` and `portraitFrame`. `thought.js` assigns `root.ThoughtCompiler` (bundle near line 4365) with `parse` and `compile`. `core.js` requires morphology. `index.ts` imports thought as a private binding `T` and never re-exports it. A static `import { Morphology, ThoughtCompiler } from './assets/ranch/quinelings-runtime.js'` has no binding. `globalThis.Morphology` only appears as a side effect after evaluation.

`packages/agent-sdk/src/index.ts` does not export them either. Its public surface is `Runtime`, `QuinelingError`, `capabilities`, `examples`, and the type re-exports. The Node package root is the session runtime. The browser entry is the module bag 3D pages import. Patch the browser entry. A matching re-export on `index.ts` is optional and cheap: thought is already imported there, and `core.js` already pulls morphology into the Node bundle.

Exact patch for `packages/agent-sdk/src/browser-entry.ts`:

```ts
// @ts-expect-error shared JavaScript has no declarations
import Morphology from '../../../morphology.js';
// @ts-expect-error shared JavaScript has no declarations
import ThoughtCompiler from '../../../thought.js';

export {
  Runtime, QuinelingError, Anatomy, QDL, Quinelings, QuinelingKernels,
  QuinelingOffspring, QuinelingWorld, Morphology, ThoughtCompiler
};
```

Leave the existing `Chroma`, `WovenBody`, `VisualCapsule`, and `LifeformFamilies` exports in place.

Optional Node parity, at the bottom of `packages/agent-sdk/src/index.ts`:

```ts
export { default as Morphology } from '../../../morphology.js';
export { default as ThoughtCompiler } from '../../../thought.js';
```

`ThoughtCompiler` is the module `index.ts` already imports as `T`. Re-export that binding if a second import is unwelcome: `export const ThoughtCompiler = T`.

Then run `node scripts/build-ranch-browser.mjs` so `assets/ranch/quinelings-runtime.js` gains `Morphology` and `ThoughtCompiler` in the final `export { ... }`. Add one assertion to a browser import check:

```js
const sdk = await import('./assets/ranch/quinelings-runtime.js');
assert.equal(typeof sdk.Morphology.surfaceFrame, 'function');
assert.equal(typeof sdk.Morphology.portraitFrame, 'function');
assert.equal(typeof sdk.ThoughtCompiler.parse, 'function');
assert.equal(typeof sdk.ThoughtCompiler.compile, 'function');
```

`surfaceFrame` and `portraitFrame` are the calls gallery.js and a 3D fit both need. `ThoughtCompiler.parse` / `compile` are the browser equivalents of `Runtime.parse` / `Runtime.compile`.

## 2. Clamp `LifeformFamilies.authorParams` into the domain `validate` already enforces

Severity: medium. The author function can return a geometry record that `validate` rejects.

`lifeform-families.js` line 5 builds parameters with raw arithmetic. `validate` (line 6) then requires this exact domain:

| field | domain | integer |
| --- | --- | --- |
| growth | [0.12, 0.35] | no |
| spread | [0.5, 1.2] | no |
| curl | [0.4, 1.6] | no |
| asymmetry | [0, 0.4] | no |
| offset | [0, 2π] | no |
| frequency | [2, 8] | yes |
| lobes | [3, 9] | yes |
| twist | [1, 3] | no |
| whorls | [2, 4] | yes |

`frequency` is `2 + min(6, t.depth)` and `lobes` is `3 + min(6, t.convergence + t.effects)`. Neither is rounded. `whorls` is the only structural field passed through `Math.floor`. Checked:

- `authorParams('logarithmic-mantle', {depth: 4.9, convergence: 1.2, effects: 0.2}, 7)` returns `frequency: 6.9`, `lobes: 4.4`. `validate` throws `LifeformFamilies: integer structure required`.
- `authorParams('phyllotaxis-fan', {depth: -1, convergence: 0, effects: 0}, 1)` returns `frequency: 1`, `whorls: 1`. `validate` throws `LifeformFamilies: bounded parameters required`.

`WovenBody.author` calls `authorParams` with integer topology, and those outputs already sit inside the table, so `tests/verify-woven-body.cjs` stays green if the clamp is a no-op on in-range integers. Callers that pass a fractional topology, or that spread species overrides into the same bag, currently have to copy the table. Sherlock does that in `authorSpeciesWovenRecord` (`clampVal` plus `Math.round`). One table in the SDK removes that fork.

Patch `lifeform-families.js`. Hoist the limit table out of `validate` and clamp before return. Do not loosen `validate`. Admission stays fail-closed. Only authoring clamps.

```js
const LIMITS = {
  growth: [.12, .35], spread: [.5, 1.2], curl: [.4, 1.6], asymmetry: [0, .4],
  offset: [0, TAU], frequency: [2, 8], lobes: [3, 9], twist: [1, 3], whorls: [2, 4]
};
const INTEGERS = new Set(['frequency', 'lobes', 'whorls']);
const clampNum = (v, lo, hi) => Math.max(lo, Math.min(hi, Number.isFinite(v) ? v : lo));
function clampGeometry(g) {
  const out = {};
  for (const [k, [lo, hi]] of Object.entries(LIMITS)) {
    const n = clampNum(g[k], lo, hi);
    out[k] = INTEGERS.has(k) ? Math.round(n) : n;
  }
  if (g.material && g.material.model === 'projected-compression') out.material = g.material;
  return out;
}
function authorParams(mechanism, t, seed) {
  const r = rng(seed >>> 0);
  const depth = Number.isFinite(t?.depth) ? t.depth : 1;
  const conv = Number.isFinite(t?.convergence) ? t.convergence : 0;
  const effects = Number.isFinite(t?.effects) ? t.effects : 0;
  return clampGeometry({
    growth: .16 + .12 * r(),
    spread: .7 + .3 * r(),
    curl: .7 + .8 * r(),
    asymmetry: .12 + .22 * r(),
    offset: TAU * r(),
    frequency: 2 + Math.min(6, depth),
    lobes: 3 + Math.min(6, conv + effects),
    twist: 1 + Math.min(3, Math.max(0, depth)) / 2,
    whorls: 2 + Math.min(2, Math.floor(depth / 3)),
    ...(mechanism === 'toroidal-weave' ? {} : { material: { model: 'projected-compression', rows: 48, columns: 15, focus: 2, quiet: .12, gain: .65 } })
  });
}
```

Export `LIMITS` and `clampGeometry` on the public `api` object next to `authorParams`. Point `validate` at `LIMITS` so the two cannot drift. `toroidal-weave` must still omit `material`. `clampGeometry` should drop `material` unless the caller kept it, and `authorParams` should keep the current ternary.

In-range integer topology must return the same numbers as today. Add a test that fractional and negative feature bags validate after `authorParams`, and that `authorParams(mechanism, W.topology(graph), 42)` is deep-equal to the pre-clamp formula for the three adapter mechanisms.

## 3. Let `WovenBody.author` take a mechanism without changing the default

Severity: medium. `lorenz-flow`, `recursive-affine`, and `pleated-braid` validate, compile, and frame, and `author` never emits them.

`woven-body.js` `author` (line 16) picks a mechanism with one cascade: guards → `clifford-flow`, fanout ≥ 2 → `recursive-julia`, effects → `toroidal-weave`, convergence ≥ 2 → `logarithmic-mantle`, else `phyllotaxis-fan`. Geometry for clifford and Julia is inlined there. Lorenz and affine exist only as hand-built records (see `tests/verify-woven-body.cjs`). Every 3D integrator recopies `sigma === 10`, `rho === 28`, `beta === 8/3`, grid bounds, and scroll-chart limits, and a typo fails `validate` with `out of bounds` or `classical Lorenz RK4 required`.

Patch signature: `author(graph, seed = 1, options = {})`. When `options.mechanism` is omitted, keep the cascade and the current record bit-for-bit (`assert.deepEqual(W.author(graph, 42), r)` must keep passing). When it is set, require it to be one of `pleated-braid`, `lorenz-flow`, `clifford-flow`, `recursive-affine`, `recursive-julia`, `logarithmic-mantle`, `toroidal-weave`, `phyllotaxis-fan`. Dispatch:

- Adapter mechanisms: `geometry = Families.authorParams(mechanism, topology, seed)` after the clamp in section 2. Delete `dynamics` and `embedding`.
- `clifford-flow` and `recursive-julia`: use the object literals already inside `author`.
- `pleated-braid`: topology, strands, folds, turns, territories only.
- `lorenz-flow`: the classical preset already admitted by `validate` (`sigma: 10`, `rho: 28`, `beta: 8/3`, `method: 'rk4'`, `dt` in `[0.002, 0.01]`, `window <= floor((steps - 5) / 2)`). Use the finite preset from the woven-body test (`dt: 0.006`, `burn: 512`, `steps: 4096`, `window: 512`, initial inside `[-2, 2]` and off the origin) so the new path validates on the first call.
- `recursive-affine`: `model: 'contractive-branch-charts'`, `chart: 'rolled-scroll'`, `rollTurns` in `[2, 4.2]`, `depth` in `[2, 6]`, two or three transforms with `scale` in `[0.35, 0.67]`.

Do not accept a free-form geometry override that skips `validate`. Call `validate` at the end of `author` so a new mechanism cannot ship an illegal record.

## 4. Export object-space normals and honor `reuse` on every `frame`

Severity: medium. WebGPU lighting and buffer upload do not match what the samplers already compute.

Checked on tideglass, seed 42:

- Family adapter (`logarithmic-mantle`): `WovenBody.frame(body, phase, {budget: 512, reuse})` returns keys `points`, `owners`, `ridges` only. `reuse.points` is ignored. Two calls allocate two `Float32Array`s. `LifeformFamilies.frame` (line 52) destructures `reuse` and never reads it. `WovenBody.frame` forwards `reuse` into that call (line 111).
- `pleated-braid`: the same `reuse` buffer is kept (`f1.points === reuse.points === f2.points`). There is still no `normals` array. Crest vertices do carry `nx`, `ny`, `nz` because `point()` (line 105) returns them and `caustics` pushes that object onto `ridge.line`. The packed cloud does `points.set([q.x, q.y, q.z, q.alpha])` and drops the normal.
- `juliaFrame`, `recursiveFrame`, and `cliffordFrame` follow the same packed layout. Only clifford and the pleated path consult `reuse`.

`Anatomy.frame` already returns `normals` as a stride-3 `Float32Array` beside stride-4 `points`. `Runtime.frame` copies both. `ranch-renderer.js` uploads position, normal, and owner. The experimental body should use that layout.

Patch, without changing existing sample coordinates:

1. In `lifeform-families.js` `frame`, allocate like the pleated path:

```js
const points = reuse?.points?.length === budget * 4 ? reuse.points : new Float32Array(budget * 4);
const owners = reuse?.owners?.length === budget ? reuse.owners : new Uint16Array(budget);
const normals = reuse?.normals?.length === budget * 3 ? reuse.normals : new Float32Array(budget * 3);
```

2. Fill `normals` with a unit cross product of `point()` at `(u ± ε, v)` and `(u, v ± ε)`. Keep the current `points` and `owners` writes, including the prefix overwrite that forces every operation onto samples `[0, nodes.length)`. That overwrite is what `tests/verify-lifeform-families.cjs` relies on for owner coverage. Leave it. Also return `anchors`: one `{x,y,z,owner}` per node, using the same midpoint sample the prefix writes. A WebGPU grid can ignore indices `0 .. nodes.length-1` and draw organs from `anchors`. Document that those prefix slots are organ reservations, not chart vertices.

3. In `woven-body.js` `frame`, `juliaFrame`, `recursiveFrame`, and `cliffordFrame`, write the same `normals` buffer. The pleated path can copy `q.nx, q.ny, q.nz` it already computes. Julia, clifford, and recursive need the same finite-difference normal, or a finite analytic derivative where one exists (`recursiveScrollJacobian` is a scalar area derivative, not a 3-vector; do not store it as a normal).

4. Return shape stays `{points, owners, ridges}` plus `normals`. Stride: `points.length === owners.length * 4`, `normals.length === owners.length * 3`, `points` is `[x, y, z, alpha]`.

5. On each ridge, add `positions: Float32Array` of tightly packed `[x, y, z]` in the same order as `line`. Keep `line` as the array of `{x, y, z, alpha, owner, ...}`. `lifeform-renderer.js` and `verify-woven-body.cjs` read `ridge.line`. Renaming `line` to `points` would break them.

6. Update `packages/agent-sdk/src/experimental.ts` `Frame` and `MathematicalLifeforms.frame` so `normals` is `Float32Array` and `crests` stays boolean for this experimental sampler. Anatomy's integer crest count is a different API. Do not overload it.

Object axes, for the WebGPU camera: X and Y are the chart plane, Y is the long axis (mantle spiral and pleat spine run in XY), Z is thickness. The frame is right-handed and Y-up. Phase is radians. One cycle is `2π`. `verify-woven-body.cjs` expects substantial travel between phase `0` and phase `2`. Do not rescale phase to the unit interval.

`Families.portraitFrame` and `WovenBody.portraitFrame` already return `{cx, cy, cz, width, height, depth}` in that same space. Fit the camera to that box. Do not rebuild bounds from a handful of ribbon samples.

## 5. Give `Morphology.portraitFrame` a Z extent, and add an object-space sampler

Severity: medium. The 2D morphology frame is what a 3D fit will call, and it has no depth.

`morphology.js` `portraitFrame` (lines 130–142):

- With `design.anatomy`, it returns `Anatomy.portraitFrame` and multiplies `width` and `height` by `1.5`. `depth` is left at the raw anatomy extent. A uniform box fit then clips Z or over-pads XY.
- Without anatomy, `include()` tracks only x and y. Checked with `QDL.create('nautilus')` plus `Quinelings.describe`: the result is `{cx, cy, width, height}`. There is no `cz` and no `depth`. `verify-morphology.cjs` only asserts `width`, `height`, and identity of the cached object (`assert.strictEqual(frame, M.portraitFrame(s))`).

`surfaceFrame` (line 145) runs every sample through `pose()` (yaw, pitch, lean) before writing `points`. Those coordinates are the gallery camera, not the body. A second WebGPU view matrix applies the camera twice. The function also returns no normals. Gallery and `verify-morphology.cjs` depend on this projected frame. Leave `surfaceFrame` as the 2D view.

Patch:

1. In the non-anatomy branch, track `minZ` / `maxZ` from `surfacePoint(...).z` and from anchor `z`, with the same padding already applied to x and y. Add `cz` and `depth` on the cached object. Same object identity, so the `strictEqual` cache check holds. Existing `cx`, `cy`, `width`, `height` stay the 2D fit.

2. On the anatomy branch, set `depth: f.depth * 1.5` beside the existing width and height scale. `gallery.js` uses only `width` and `height`, so the 2D occupancy fit is unchanged. State in the comment that the `1.5` is portrait padding, and that object-space bounds are `Anatomy.portraitFrame`.

3. Add `objectFrame(s, t, thumb)` that samples the same `(u, v, ribbon)` grid as `surfaceFrame` but writes `surfacePoint` before `pose`. Return `{points, normals, owners, ridges, bounds}` with the strides in section 4. `bounds` is the unpadded `{cx, cy, cz, width, height, depth}` for anatomy, and the new z-aware box for family membranes. Export it on the `api` object next to `surfaceFrame`.

4. Export `chroma.js` `channels(hex)` on the public `api`. It already parses `#rrggbb` to three bytes and is currently private. `colorFor` keeps returning a hex string. WebGPU uploads `channels(colorFor(shape, i)).map(c => c / 255)`. Do not change `colorFor` to an `{rgb}` object. Default designs from `QDL.create` have no `chroma` block, so `colorFor` returns `ink.neutral` (`#f0f1eb`). Role color is `Chroma.ROLES[Chroma.role(op)].color`, which is already public. Document that. Do not switch the default or gallery palettes will change.

## 6. What to leave alone

- `validate` in both files stays strict. Clamping belongs in `authorParams` and in the new `author` mechanism path.
- Ridge property name stays `line`. A 3D consumer that reads `ridge.points` or `ridge.owner` is reading fields the SDK does not set. Each vertex is `ridge.line[i]` with `{x, y, z, alpha, owner}`. Packed `[x, y, z]` belongs in the new `ridge.positions`.
- The first `nodes.length` cloud samples in `LifeformFamilies.frame` stay reserved for operation patches.
- Frozen QDL 1 source, `QDL.create`, and `Runtime.frame` anatomy strides stay as they are. Experimental geometry remains outside the frozen design record (`D.validate` rejects `design.woven`).
- Phase units stay radians.

## Verification

From `/home/charles/repos/quinelings`:

```sh
node tests/verify-lifeform-families.cjs
node tests/verify-woven-body.cjs
node verify-morphology.cjs
node scripts/build-ranch-browser.mjs
node --input-type=module -e "import('./assets/ranch/quinelings-runtime.js').then(s=>{if(typeof s.Morphology.surfaceFrame!=='function'||typeof s.ThoughtCompiler.parse!=='function')process.exit(1)})"
```

Add the fractional `authorParams` case and a `reuse.points` identity assertion for a `logarithmic-mantle` body to `tests/verify-lifeform-families.cjs`. Add `normals.length === owners.length * 3` for one pleated frame and one family frame to `tests/verify-woven-body.cjs`.

## Suggested apply order

1. Section 1 (export) and rebuild the ranch bundle. Unblocks `import { Morphology, ThoughtCompiler }`.
2. Section 2 (clamp + exported `LIMITS`). Unblocks species authoring without a forked limit table.
3. Section 3 (mechanism argument). Unblocks Lorenz and affine without hand-copied dynamics.
4. Sections 4 and 5 (normals, `reuse`, Z bounds, `objectFrame`, `channels`). Unblocks a WebGPU vertex layout that matches `Anatomy.frame` and the ranch renderer.
