# Grok Quineling 03 — Compiler feed, live AST morphogenesis, manifold ribbons

Screenshot: `quineling_manip_03_live_edit_fact5.png` (copper nautilus, plaque `16 NODES`, white lattice sitting in the bore).
Verified by compiling every preset plus `fact(5)`, a 4-element `sum_list`, and `let seed = 3` through `compileSherlockProgramToQuineling`. All eight quines verify. No application source was edited.

## Verdict

`#sourceEditor` source is parsed and compiled once the user runs it. The task graph, FNV seed, and literal nodes do change with the edit. The 3D body does not: chamber, zooid, cube, ring, and fuel-cell counts are baked in `createQuinelingSubsystem`, and `#activeAstBadge` is painted from `PRESET_EDITORIAL_META`, so `fact(5)` still reads `Factorial · fact(4)`.

The white cage in the screenshot is the ribbon fallback, not a Quinelings manifold. `WovenBody.frame` returns `ridge.line[]` of `{x,y,z}` objects (210 ridges on factorial, 0 with `.points`). The reader tests `pt.length >= 2`, misses every sample, and draws a radius-0.18 helix. `Morphology.surfaceFrame` is never called. `wovenBounds` has `width/height/depth`, not `.scale`.

## 1. Preset and custom source do reach the compiler

Live path, after Compile, Ctrl/Cmd-Enter, a preset click, or a fuel/stack slider:

1. `app.js` `executeCurrentSource` copies `#sourceEditor` into `state.source` and runs `compileAndRunSherlock`.
2. `getOrBuildStepState` puts `source` and `presetId` on the step state.
3. `SherlockStage.render` forwards that object to `quineling.update`.
4. `resolveCompiledQuineling` calls `compileSherlockProgramToQuineling(source, QUINELING_SPECIES, trace)`.
5. The compiler calls `parseAndCheckDraft`. Cache key is `source|S:stack|F:fuel|O:outcome`, so an edit is a cache miss.

Typing alone does not compile. The `input` listener only renumbers lines. Species on the live path comes from `analyzeSherlockAst`, because `stepState.source` is always set. The preset-id shortcut in `resolveQuinelingSpecies` does not run in the frame loop.

| Preset | `speciesId` | Entry text (`programImage`, `astPath === "entry/"`) | `literals` | Graph nodes | Quine |
|---|---|---|---|---|---|
| `factorial` | factorial | `fact(4)` | `[4,0,1,1]` | 16 | yes |
| `list_fold` | list_fold | `sum_list(cons(5, cons(8, cons(3, nil))))` | `[5,8,3,0]` | 16 | yes |
| `tree_fold` | tree_fold | `sum_tree(branch(leaf(7), branch(leaf(4), leaf(9))))` | `[7,4,9]` | 16 | yes |
| `mutual_even_odd` | mutual_even_odd | `is_even(4)` | `[4,0,1,1,0,0,1]` | 20 | yes |
| `lexical_let` | lexical_let | `let seed = 12 in cube_plus(seed, 272)` | `[12,272]` | 20 | yes |
| `checked_underflow` | checked_underflow | `clamp_sub(3, 9)` | `[3,9]` | 10 | yes |
| `stack_overflow` | stack_overflow | `dive(6)` | `[6,0,0,1,1]` | 15 | yes |
| `fuel_exhaustion` | fuel_exhaustion | `count_down(5)` | `[5,0,100,1]` | 14 | yes |

Edits that were compiled the same way:

- `entry fact(5)` stays `factorial`. Literals become `[5,0,1,1]`, `lit_5` replaces `lit_4`, multiply `factor` becomes 5, seed changes. Node count stays 16. The designer still builds 5 septa and 28 ribs (`fact(4)..fact(0)`).
- `sum_list(cons(9, cons(2, cons(7, cons(1, nil)))))` stays `list_fold`. Literals `[9,2,7,1,0]`, nodes 16 → 17. Zooids stay the hardcoded `[5,8,3]`.
- `let seed = 3 in cube_plus(seed, 10)` stays `lexical_let`. Literals `[3,10]`, multiply factor 12 → 3. Cubes stay `[0.11, 0.21, 0.32]`.

Read parameters from `parsed.entry`, not from `analysis.literals` by index. For factorial the walk order is entry `4`, then `n == 0`, then base `1`, then `n - 1`, so `literals[1]` is `0`. `buildQuinelingTaskGraphFromAst` already stores that `0` on `if_base_*`. Chamber count must come from the entry call argument.

Suggested entry readers (compiler or designer, using the AST `parseAndCheckDraft` already returns):

```javascript
function uintLit(node) {
  return node?.kind === 'Literal' && node.litValue?.kind === 'UInt'
    ? Number(node.litValue.value)
    : null;
}
function consSpine(node, out = []) {
  let n = node;
  while (n?.kind === 'Binary' && n.op === 'cons') {
    const h = uintLit(n.left);
    if (h != null) out.push(h);
    n = n.right;
  }
  return out;
}
function treeLeaves(node, out = []) {
  if (!node) return out;
  if (node.kind === 'Unary' && node.op === 'leaf') {
    const v = uintLit(node.operand);
    if (v != null) out.push({ value: v });
  }
  for (const c of node.children || []) treeLeaves(c, out);
  return out;
}
```

Attach the result to the artifact as `analysis.entryMorph` so `update()` does not re-walk. Fuel and stack already arrive as `analysis.initialFuel` and `analysis.stackLimit` from the trace (slider values). `count_down(5)` does not set the fuel budget; the slider does. `dive(6)` does not set the frame horizon; `stackLimit` does.

## 2. How entry edits should change organs

Meshes are allocated once. Give each species a fixed pool and, when `artifact.cacheKey` changes, set `visible`, radius, and placement. Keep `update()` for motion only. Clamp every radius so the organism stays inside `QUINELING_MAX_RADIUS` (0.46).

**factorial.** `N = uintLit(entry.args[0])` (4 or 5). Visible septa = `clamp(N + 1, 1, 12)` so `fact(5)` shows six discs, `fact(0)` through `fact(5)`. Place septum `c` at the same log spiral the ribs already use:

```javascript
const u = count === 1 ? 0.7 : 0.24 + (c / (count - 1)) * 0.72;
const theta = u * 2.15 * Math.PI * 2 - Math.PI * 0.6;
const r = 0.042 * Math.exp(0.165 * (theta + Math.PI * 0.6));
```

Grow the whorl with `turns = clamp(1.15 + 0.22 * count, 1.4, 2.6)` and stop before `r > 0.42`. The unlock test `(callDepth + 1) / 5` must divide by `count`, not by 5. Rib count can stay 28; they are nacre, not call frames.

**list_fold.** Pods = `consSpine(entry.args[0])`, pool 8. Four elements → four bells. `x` from `lerp(-0.42, 0.42, i / (n - 1))`. Bell radius `clamp(0.05 + value * 0.006, 0.05, 0.12)` so `9` reads larger than `1`. Pleat fins stay 18; their wave period can follow `n`.

**tree_fold.** One crown per `treeLeaves(entry.args[0])`, pool 8. Keep the trunk and the Y split. Scale each bulb with `clamp(0.04 + value * 0.004, 0.04, 0.09)`. A deeper `branch` adds a pool limb (cylinder between parent joint and the new leaf) instead of another decorative twig.

**lexical_let.** Nested cubes = `analysis.letCount + 1` (seed, then each `let` in the body), pool 6. `seed = uintLit(entry.value)`. The body computes `sq = seed*seed`, `cb = sq*seed`; those products are not AST literals. Edge length:

```javascript
clamp(0.06 + 0.045 * Math.log10(1 + value), 0.08, 0.40)
```

`seed = 12` stays a wide nest (`12 / 144 / 1728`). `seed = 3` tightens to about `0.09 / 0.11 / 0.15`. Petal whorls = cube count, 8 petals per whorl, same golden angle already in the mesh. The offset literal tints the outer cube; it does not add a cube.

**checked_underflow.** Tentacle count stays 16 (Aux/2 limb width). `gap = Math.abs(b - a)` from the two call literals. Bell pulse amplitude `0.06 + clamp(gap, 0, 32) / 32 * 0.2`. A wider gap (for example `1, 100`) flares the bell and pushes tentacle emissive toward `#e11d48` earlier in the trace. Rule A225 still snaps them fully crimson.

**stack_overflow.** Visible shock rings = `clamp(analysis.stackLimit, 1, 12)`, not the constant 5. The ceiling disc stays at `y = 0.34`. Rings that index `< min(frameCount, entry N)` light; the rest stay slate until Rule A140. Moving the stack slider from 5 to 8 must show 8 rings on the next compile. `dive(3)` with limit 8 lights only three.

**fuel_exhaustion.** Visible cells = `clamp(analysis.initialFuel, 8, 24)`. The preset’s 14 cells match fuel 14. Fuel slider 8 shows 8 cells; fuel 20 shows 20. `count_down`’s literal does not retile the torus. Extinguish with `Math.round(liveFuelRatio * cellCount)`.

**mutual_even_odd.** Wing count stays two. Wing scale `clamp(0.85 + entryN * 0.04, 0.85, 1.35)` so `is_even(4)` and `is_even(8)` are distinguishable. Flap phase already follows the top call name.

Rebuild only when `cacheKey` changes. Pools that are over-allocated stay in the scene at `visible = false` so the frame path does not allocate geometries.

## 3. `#activeAstBadge` must follow the entry expression

`syncStepUI` writes the badge from `PRESET_EDITORIAL_META[state.activePresetId].sub`. That table is the canonical preset chip (`fact(4)`, `sum_list([5, 8, 3])`, …). It does not read the editor. Leave the preset-strip `<small>` labels on that table. Change only the badge.

The parsed call or let is already the right string: the entry row’s `text` is `fact(5)`, `sum_list(cons(9, …))`, or `let seed = 3 in cube_plus(seed, 10)`. Its `astPath` is `entry/`.

```javascript
function entryClause(source) {
  const m = String(source || '').match(/\bentry\s+([\s\S]*?)\s*;/);
  return m ? m[1].replace(/\s+/g, ' ').trim() : '';
}

function formatActiveAstBadge(trace, source, presetId) {
  const title =
    PRESET_EDITORIAL_META[presetId]?.title ||
    trace?.programName ||
    'Sherlock Program';
  const parsed = trace?.programImage?.expressions?.find((e) => e.astPath === 'entry/');
  const live = (parsed?.text || entryClause(source) || '').replace(/\s+/g, ' ').trim();
  const shown = live.length > 42 ? `${live.slice(0, 41)}…` : live;
  return shown ? `${title} · ${shown}` : title;
}
```

Call it from `syncStepUI` in place of the current badge write, and from the `#sourceEditor` `input` listener so the header moves while typing, before Compile. A failed compile must not put the static `fact(4)` label back. After a successful run, the parsed `entry/` text wins over the regex.

Expected: editing the factorial preset to `entry fact(5);` shows `Factorial · fact(5)`. A pasted list program while the factorial chip is still active should prefer `trace.programName` (`ListFold · sum_list(…)`) when `artifact.speciesId !== state.activePresetId`.

## 4. Map SDK samples onto the anatomy

### Why the cage is flat

In `designer8_quineling.js` `update`, around the `WovenBody.frame` loop:

- Ridges are `{ line, material }` or `{ line, primary }`. Points are `{ x, y, z, alpha, owner }`. `ridge.points` is absent, so every ribbon takes the fallback `cos/sin` helix at radius `0.18`, `y ∈ [-0.14, 0.14]`. That is the white lattice in the screenshot, centered in the bore, ignoring the copper shell.
- `boundsScale` reads `wovenBounds.scale`. The portrait is `{ cx, cy, cz, width, height, depth }` (factorial about `2.72 × 3.00 × 0.89`). The divisor is therefore stuck at `0.5`.
- `Morphology` is imported in the designer and unused. `surfaceFrame(shape, t)` is the call that actually samples the QDL membrane. `portraitFrame` only returns a 2D fit (`cx, cy, width, height`).
- Clifford (jelly) portrait depth is `0.20` on a width of `1.51`. Julia (comet) depth is `0.14` on a width of `1.38`. Plotting those raw points, even with the correct reader, stays a flat card. Lorenz `z` is the thin axis (`* 0.009` in the integrator). The logarithmic mantle and the toroidal weave are genuinely 3D, and they still will not meet the hand-built meshes or inherit `torusGroup`’s tilt, because the ribbon mesh is parented to `coreRoot`.
- `bioPhase` is in `[0, 1)`. Family adapters reduce phase with `phase % TAU`, so the mantle only advances one radian. Pass `bioPhase * Math.PI * 2`.

`wf.points` is a `Float32Array` of stride 4 (`x, y, z, alpha`), 22_000 samples. Draw crests, not the whole cloud.

### Reader

```javascript
function ridgePoint(ridge, u) {
  const line = ridge?.line;
  if (!line?.length) return null;
  const i = Math.min(line.length - 1, Math.floor(u * (line.length - 1)));
  const p = line[i];
  return p && Number.isFinite(p.x) ? p : null;
}
```

Family-adapter ridges are ordered `for chart k, for h in 0..14`, with `v = -1 + 2*h/14` and `u = j / 160`. Take 16 ridges spread across the list (`Math.floor(rIdx * ridges.length / 16)`), recover `v` as `(ridgeIndex % 15) / 14 * 2 - 1`, and recover `u` from the sample index. Use `p.owner` for the existing Chroma tint. Do not test `pt.length`.

Call `Morphology.surfaceFrame(artifact.shape, bioPhase * TAU, false)` once per frame (thumb `true` if the 6000-sample budget is too heavy). Its ridges use the same `{ line: [{x,y,z,owner}] }` shape, usually 4 crests × 301 points. Run them through the same embed. They are the QDL family membrane; WovenBody crests are the finer filaments.

Reparent `sdkManifoldRibbonMesh` onto the visible species group whenever `speciesId` changes so the tokamak tilt (`rotation.x = 0.42`, `rotation.z = 0.18`) and the comet lance (`rotation.z = -0.36`) apply to the ribbons. Write positions in that group’s local space.

### Per-species embed

`embed(speciesId, u, v)` returns the ribbon centerline. `u, v ∈ [0, 1] × [-1, 1]`. Add a wrinkle only for mechanisms whose SDK `z` is real: `pos + normal * (p.z - cz) * wrinkleGain`, with `wrinkleGain` small enough to stay inside the tube. For clifford and julia, ignore raw `x,y,z` and use only `u`, `v`, and `owner`.

**factorial — shell tube (same constants as the 28 ribs).**

```javascript
const theta = u * turns * TAU - Math.PI * 0.6;          // turns from §2
const radial = 0.042 * Math.exp(0.165 * (theta + Math.PI * 0.6));
const tube = Math.max(0.012, radial * 0.44);
const roll = (v * 0.5 + 0.5) * TAU;
const R = radial + tube * 0.55 * Math.cos(roll);
// x = cos(theta) * R,  y = sin(theta) * R,  z = tube * 0.55 * sin(roll)
```

Ribbons then lie on the nacre instead of in the bore. Septa stay the flat discs already placed on this curve.

**list_fold — horizontal helix through the zooids.**

```javascript
const x = lerp(-0.42, 0.42, u);
const y = Math.sin(u * Math.PI * podCount) * 0.075 + v * 0.02;
const z = Math.cos(u * TAU * 1.5) * 0.055 * (0.35 + Math.abs(v));
```

One ribbon per pod can be a circle in the bell’s local frame (`radius = bellRadius * 1.15`, the torus already on the pod).

**tree_fold — helical bark.** Assign ribbon `rIdx % limbCount` to a segment `pA → pB` with radii `rA, rB`. Build a normal/binormal from the segment axis. Center `lerp(pA, pB, u)`, azimuth `v * Math.PI + u * TAU * 1.5`, radius `lerp(rA, rB, u)`.

**mutual_even_odd — figure-8 on the two wing tori (radius 0.18, centers ±0.14).** Raw Lorenz `z` is too thin to read as a dipole.

```javascript
const side = (rIdx % 2 === 0) ? -1 : 1;
const ang = u * TAU;
const tilt = 0.52;
// x = side * 0.14 + cos(ang) * 0.18
// y = sin(ang) * 0.18 * cos(tilt)
// z = side * 0.06 + sin(ang) * 0.18 * sin(tilt)
```

`v` offsets along the wing normal so a few filaments lie on the forewing membrane.

**lexical_let — golden-angle petals.** Match the existing petal placement: `angle = k * PHI`, `ring = floor(k / 8)`, `r = 0.13 + ring * 0.075`, base `y = -0.14 + ring * 0.04`. Along the petal, `u` walks out of the cube by about `0.16` and `v` is thickness (`±0.015`). Inner ribbons can orbit each nested cube on a square-rounded loop of that cube’s edge length.

**checked_underflow — hemisphere plus helical tentacles.** Clifford’s depth `0.2` is the flat grid you get if you plot it raw.

Bell (first four ribbons), sphere radius `0.31`, center `y = 0.06`, polar cap `theta ∈ (0, 0.58π)`:

```javascript
const phi = u * TAU + rIdx;
const theta = (0.15 + ((v + 1) * 0.5) * 0.85) * Math.PI * 0.58;
const R = 0.31;
// x = R*sin(theta)*cos(phi), y = 0.06 + R*cos(theta), z = R*sin(theta)*sin(phi)
```

Remaining ribbons, one per tentacle `t` in `0..15`: `a = t/16 * TAU`, `rad = (t % 2 ? 0.25 : 0.17) * (1 - 0.25 * u)`, `y = lerp(0.02, -0.40, u)`, `x,z = (cos, sin)(a + u * TAU * 0.35) * rad`.

**stack_overflow — shock rings and a lance helix.** Julia depth `0.14` is a flat card. Ring `i` of `ringCount`: `y = lerp(-0.22, 0.32, i / (ringCount - 1))`, `rad = 0.14 + (i / (ringCount - 1)) * 0.19`, `ang = u * TAU`, `v` is a `±0.01` vertical ribbon width. Lance filaments go in `cometLanceSub` local space: cone radius `lerp(0.07, 0.14, 1 - u)` around the head at local `y = 0.22`, three turns, `ang = u * TAU * 3 + v`.

**fuel_exhaustion — (1, 3) torus on the visible doughnut.** The SDK weave is already toroidal; snap it to the mesh so it shares the open center (`R = 0.32`, minor `r = 0.068`) and the group tilt:

```javascript
const theta = u * TAU;
const phi = 3 * theta + (v + 1) * Math.PI;
const R = 0.32, r = 0.068;
// x = (R + r*cos(phi)) * cos(theta)
// y = (R + r*cos(phi)) * sin(theta)
// z = r * sin(phi)
```

Parent to `torusGroup`. A uniform fit of `wovenBounds` into the atrium is the wrong tool here: it preserves the SDK’s ~1.7 unit major radius and then misses the 14 cells.

Spores can use the same `embed` (Fibonacci index → `u`, `v`) so the 64 octahedra sit on the shell, the bell, or the torus. Today they orbit a fixed sphere of radius ~0.24 at the origin, which is why they halo the cage in the screenshot.

## 5. Acceptance

1. All eight presets still select the species in the table above, and `quineVerified` stays true.
2. After Compile, `entry fact(5)` shows six septa and the badge `Factorial · fact(5)`. The preset chip can keep saying `fact(4)`.
3. `cons(9, cons(2, cons(7, cons(1, nil))))` shows four bells whose radii follow 9, 2, 7, 1.
4. `let seed = 3` shrinks the three cubes versus `seed = 12`. Adding a `let` adds a cube and a petal whorl, up to the pool.
5. Stack slider `8` shows 8 shock rings. Fuel slider `20` shows 20 torus cells. Tentacles stay 16; a larger `|a-b|` only changes bell scale and crimson.
6. On the factorial view, no radius-0.18 lattice remains in the bore. Ribbon vertices lie on the log-spiral tube (`z` follows `sin(roll)`, not `0`). On the torus preset the ribbons are tilted with the doughnut and leave the hole empty. On the jelly preset the filaments cover the hemisphere and spiral down the tentacles.
7. `wovenBounds.scale` is not read. Phase passed into `WovenBody.frame` and `Morphology.surfaceFrame` is in radians across a full turn.

Files to change, when an implementer picks this up: `src/designers/designer8_quineling.js` (pools, embed, ribbon reader, reparent) and `src/app.js` (badge). The compiler already supplies `parsed.entry`, `literals`, `initialFuel`, and `stackLimit`; a small `entryMorph` on the artifact is optional and keeps the walker out of the frame loop.
