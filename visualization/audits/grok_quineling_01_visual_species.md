# Grok Quineling 01 — 8-Species Visual Distinctness, Geometry & Shader Audit

**Captures:** `/home/charles/.gemini/antigravity-cli/brain/67b84c75-1698-455f-a6d9-b18cc7cdde9b/scratch/quineling_species_0{1..8}_*.png`
**Source:** `src/designers/designer8_quineling.js` (read only; no application files were edited)
**Camera in these frames:** interior preset `pos [0, -1.85, 1.12]`, `lookAt [0, -2.18, -0.28]`, `fov 72°` (`designer7_cinema.js`). Core sits at `QUINELING_CORE_Y = -2.05`. Floor modules sit at `y = -3.38`.

## Verdict

The eight silhouettes are distinguishable: coiled torus-cluster, horizontal card-ribbon, Y-coral, twin discs, hanging crystal burst, medusa, ringed comet, tilted tokamak. They are not museum-grade organisms. Every frame is the same plastic kit plus two shared defects that dominate the read:

1. `plaqueMesh` is a child of the spinning `coreRoot`, `side: DoubleSide`, `depthWrite: false`. Past `coreRoot.rotation.y = π/2` the camera sees the back of the canvas and the binomial is mirrored (`ERIRUAHXE`, `ISSYBA ATEMOC`, `ROTARTBUS ASUDEM`). The same card is mounted at local `(0, -0.49, 0.34)` with `rotation.x = -0.18`, so it occupies the bottom of the 72° frame and paints over the floor tiles.
2. `sdkManifoldRibbonMesh` is one white/pastel horizontally striped cylinder in the middle of all eight bodies. `WovenBody.frame` is sampled with the wrong ridge schema, and the live fallback is a 16-strand helix of radius `0.18` and height `0.28`. The array branch that would have run for `[x, y]` samples only adds `sin * 0.08` on Z. Neither path wraps the chart onto the shell, torus, bell, or helix.

Secondary shared artifacts: identical Fibonacci spore sphere, `depthWrite: false` chamber rays that stab through the body, and `MeshPhysicalMaterial` tuned like hard candy (`metalness` 0.1–0.45, `clearcoat` 1, `transmission` 0.45–0.55) under a single point light.

---

## Per-species evaluation

| # | Species | Silhouette readable? | Distinct from the other seven? | Artifacts that break the organism |
| --- | --- | --- | --- | --- |
| 01 | `Nautilus Arithmeticus` | Weak. Reads as a scribble of copper rings plus a cream sphere. | Yes, by color and ring mass. | Planar log-spiral of 28 separate tori; outer radius ≈ 0.60 > `QUINELING_MAX_RADIUS` 0.46; white cage in the aperture; plaque `NAUTILUS ARITHMETICUS` cut off on the bottom edge of the frame (text still forward). |
| 02 | `Taenia Cons-Catenata` | Partial. Three bells read; the “silk” is a row of flat cards. | Yes, horizontal. | 18 `BoxGeometry` fins, not a membrane; white cage behind the bells; plaque clipped at the right frame edge. |
| 03 | `Corallium Arboris` | Strong Y. Best macro read of the set. | Yes. | White cage crosses the trunk; branch joints are scaled cylinders that do not blend; crowns are 6 cones on a dodecahedron; almost no Z (a flat cutout). |
| 04 | `Lepidoptera Alternans` | Recognizable moth. | Yes. | Wings are scaled `CircleGeometry` discs with no veins; “Lorenz” is two `TorusGeometry` rings, not an orbit; white cage through the thorax; plaque lies across the cyan wing and the floor. Wing tip extent ≈ 0.50. |
| 05 | `PhylloCube Umbra` | Crystal burst, not a lotus. | Yes. | 24 cones hang from `y = -0.14` with `rotation.x` 0.55–0.99, so the bloom points down; wireframe + `transmission` on the outer cubes; white cage through the center; plaque angled into the floor band. |
| 06 | `Medusa Subtractor` | Strongest organism in the set. | Yes. | White cage visible inside the bell; 16 tentacles are rigid cylinders (laser rods); plaque reads `ROTARTBUS ASUDEM` (mirrored `MEDUSA SUBTRACTOR`) and covers tentacles and floor chips. |
| 07 | `Cometa Abyssi` | Strong comet + five rings. | Yes. | Open `ConeGeometry` shows a hollow sheet; white cage wrapped on the tail; plaque reads `ISSYBA ATEMOC` (mirrored `COMETA ABYSSI`) and sits on the floor tiles. |
| 08 | `Cyclopora Exhaurire` | Torus hole reads; the braid does not. | Yes. | `(1,3)` `TorusKnotGeometry(0.32, …)` swings to XY radius `0.48` and, with `rotation.x = 0.42`, projects into the floor; white cage fills the hole; plaque reads `ERIRUAHXE AROPOLCYC` (full mirror of `CYCLOPORA EXHAURIRE`) in the lower-left tile band. Dark cells match the fuel-out state and are fine. |

A viewer can name the species from the silhouette. The cage, the mirrored plaque, and the primitive surfaces keep every one in the same toy-plastic family.

---

## A. Plaque — mirrored text and bottom-frame clip

Construction (`designer8_quineling.js`):

```1089:1099:src/designers/designer8_quineling.js
  const plaqueMat = new THREE.MeshBasicMaterial({
    map: initialPlaqueTex,
    transparent: true,
    opacity: 0.95,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  const plaqueMesh = new THREE.Mesh(new THREE.PlaneGeometry(0.74, 0.185), plaqueMat);
  plaqueMesh.position.set(0, -0.49, 0.34);
  plaqueMesh.rotation.x = -0.18;
  coreRoot.add(plaqueMesh);
```

Yaw driver:

```1188:1189:src/designers/designer8_quineling.js
    coreRoot.position.y = QUINELING_CORE_Y + Math.sin(breathPhase) * 0.025;
    coreRoot.rotation.y = ambient * 0.22 + (isInPlaceRewrite ? Math.sin(q * Math.PI) * 0.18 : 0);
```

`PlaneGeometry` faces `+Z`. The canvas glyphs are authored left-to-right. `DoubleSide` draws the back face with the same UVs, so the string is a character-wise mirror. `ambient * 0.22` is unbounded, so world yaw crosses `π/2` after about 7 seconds and stays reversed until it crosses `3π/2`.

Confirmed mirrors in the captures:

| Capture | On-screen text | Source string |
| --- | --- | --- |
| 08 `fuel_exhaustion` | `ERIRUAHXE AROPOLCYC` | `CYCLOPORA EXHAURIRE` |
| 07 `stack_overflow` | `ISSYBA ATEMOC` | `COMETA ABYSSI` |
| 06 `checked_underflow` | `ROTARTBUS ASUDEM` | `MEDUSA SUBTRACTOR` |

01 and 04 are still on the front face (`NAUTILUS ARITHMETICUS`, `LEPIDOPTERA…`) and already collide with the frame. The mount is the cause. World center is about `(0, -2.54, 0.34)` while yaw is near 0: 0.49 below a core at `-2.05`, pushed 0.34 toward a camera at `z = 1.12`, then tilted by `-0.18` rad. The card is `0.74 × 0.185`. At 72° it fills the lower third. `depthWrite: false` composites it over the floor chips. After yaw passes `π/2`, local `+Z = 0.34` orbits behind the body (`z' = 0.34·cos(yaw)`); at `yaw = π` the card sits at world `z = -0.34`, still drawn on top, reversed, and projected onto the tile band. That is the clip in frames 07 and 08.

Setting `plaqueMesh.rotation.y = -coreRoot.rotation.y` does not undo this. Default Euler order `XYZ` plus the existing `rotation.x` does not commute with the parent yaw.

Extra defect in the refresh path (lines 1168–1180): `plaqueMat.map.dispose()` runs on textures returned from the `createPlaqueTexture` LRU. `theme.js` documents that cached plaques must not be disposed. A species round-trip can remount a disposed canvas (blank card).

### Fix

Reparent the plaque to `group` (`Designer8_QuinelingCore`), not `coreRoot`. Face the interior camera. Use the front face only. Keep it in a band that stays inside the frame for this lens: world `y ∈ [-2.72, -2.48]`, `|x| ≤ 0.36`, `z ∈ [0.02, 0.12]`.

```javascript
const plaqueAnchor = new THREE.Group();
plaqueAnchor.name = 'QuinelingPlaqueAnchor';
plaqueAnchor.position.set(0, QUINELING_CORE_Y - 0.52, 0.06);
group.add(plaqueAnchor);

const plaqueMat = new THREE.MeshBasicMaterial({
  map: initialPlaqueTex,
  transparent: true,
  opacity: 0.94,
  side: THREE.FrontSide,
  depthWrite: true,
  depthTest: true,
});
const plaqueMesh = new THREE.Mesh(new THREE.PlaneGeometry(0.62, 0.155), plaqueMat);
plaqueMesh.rotation.x = -0.06;
plaqueAnchor.add(plaqueMesh);

// inside update(), after coreRoot.rotation.y is set — do not copy that yaw:
plaqueAnchor.position.y = QUINELING_CORE_Y - 0.52 + Math.sin(breathPhase) * 0.012;
```

On refresh, drop the previous map only when `userData.sharedPlaque` is absent. The cache in `theme.js` already dedupes by title string. Shrink the plane from `0.74` to `0.62` so a 72° lens no longer shears the descenders into the tile row.

Acceptance: spin `ambient` through `[0, 40]`. The binomial stays left-to-right. The card never covers a floor chip label and never leaves the viewport.

---

## B. Striped cylinder — `sdkManifoldRibbonMesh`

The mesh is one `BufferGeometry` of `16 × 14` ribbon segments, material `color: 0xffffff`, `vertexColors`, `opacity: 0.86`, `depthWrite: false`, `side: DoubleSide` (lines 448–471). It is added to `coreRoot` for every species. The plotter:

```1217:1256:src/designers/designer8_quineling.js
    for (let rIdx = 0; rIdx < SDK_RIBBON_COUNT; rIdx += 1) {
      const ridge = ridges[rIdx % Math.max(1, ridges.length)];
      // ...
      const pts = ridge?.points || [];
      for (let s = 0; s < SDK_RIBBON_SEGMENTS; s += 1) {
        const u = s / (SDK_RIBBON_SEGMENTS - 1);
        const pt = pts[Math.min(pts.length - 1, Math.floor(u * Math.max(1, pts.length - 1)))];
        if (pt && pt.length >= 2) {
          const px = pt[0] * boundsScale;
          const py = pt[1] * boundsScale;
          const pz =
            (pt[2] !== undefined ? pt[2] * boundsScale : 0) +
            Math.sin(u * Math.PI * 2 + rIdx * 0.5 + ambient * 1.4) * 0.08;
          _sdkPts[s].set(px, py, pz);
        } else {
          const a = u * TAU + rIdx * 0.4;
          _sdkPts[s].set(Math.cos(a) * 0.18, (u - 0.5) * 0.28, Math.sin(a) * 0.18);
        }
      }
```

`WovenBody.frame` (`quinelingsSdk.js`) returns `{ points: Float32Array (xyzα), owners, ridges }`. Every mechanism stores a ridge as `{ line: [{ x, y, z, alpha, owner }, …], material }`:

- `LifeformFamilies.frame` — `parameter-filament`, real 3D `x, y, z` on the log mantle, torus weave, or phyllotaxis fan
- `caustics` / Lorenz — `{ x, y, z }`
- `recursiveFrame` — `{ x, y, z }`
- `cliffordFrame` / `juliaFrame` — objects whose `z` is only `0.07·sin(x+y+phase)` or `0.06·sin(x−y+phase)`

There is no `ridge.points`, and a sample is not an array, so `pt.length >= 2` is never true. All 16 ribbons take the `else` branch: one full turn, radius `0.18`, `y` from `-0.14` to `+0.14`, phase step `0.4`. `writeRibbonStrip` builds its binormal as `(−tz, 0, tx)`, so each helix becomes a flat horizontal strip. White base color plus vertex tint is the pastel barrel in all eight screenshots. `wf.points` is never read.

The `if` branch is the bug the captures are pointing at, and it is dead only because of the schema mismatch. If it were fed `[x, y]` it would still draw a vertical slab: `boundsScale` is `0.28 / max(0.25, wovenBounds.scale || 0.5)`. `portraitFrame` returns `{ cx, cy, cz, width, height, depth }`, never `.scale`, so the scale is the constant `0.56`, and missing `z` is replaced by `sin * 0.08`. That is a billboard, not a surface. Clifford and Julia need a second step even after the schema fix, because their SDK `z` is already that tiny sine.

`depthWrite: false` lets the barrel paint over bell, trunk, tail, and torus hole regardless of which side it is on.

### Fix

Sample `ridge.line` as objects. Fit the portrait frame. Where the chart is flatter than a quarter of its width, embed `(u, v)` on the active species surface. Delete the helix fallback.

```javascript
function sampleRidge(ridge, u) {
  const line = ridge?.line || ridge?.points || [];
  if (!line.length) return null;
  const i = Math.min(line.length - 1, Math.floor(u * (line.length - 1)));
  const p = line[i];
  if (Array.isArray(p)) return { x: p[0] || 0, y: p[1] || 0, z: p[2] || 0 };
  if (p && Number.isFinite(p.x) && Number.isFinite(p.y)) {
    return { x: p.x, y: p.y, z: Number.isFinite(p.z) ? p.z : 0 };
  }
  return null;
}

function fitOf(bounds) {
  const w = Math.max(bounds?.width || 1, 1e-4);
  const h = Math.max(bounds?.height || 1, 1e-4);
  const d = Math.max(bounds?.depth || 1e-4, 1e-4);
  const span = Math.max(w, h, d);
  return {
    s: (QUINELING_MAX_RADIUS * 1.55) / span,
    cx: bounds?.cx || 0,
    cy: bounds?.cy || 0,
    cz: bounds?.cz || 0,
    flat: d < 0.25 * Math.max(w, h),
  };
}

/** Drop-in for the px/py/pz + helix block. `nx, ny, nz` are frame-normalized. */
function embedOnSpecies(speciesId, nx, ny, nz, flat) {
  if (!flat) return { x: nx, y: ny, z: nz };
  const theta = Math.atan2(ny, nx);
  const rad = Math.min(1, Math.hypot(nx, ny) / QUINELING_MAX_RADIUS);
  const u = (theta + Math.PI) / TAU;
  switch (speciesId) {
    case 'factorial': {
      const th = u * 2.15 * TAU;
      const rr = 0.028 * Math.exp(0.145 * th);
      return { x: rr * Math.cos(th), y: rr * Math.sin(th) * 0.92, z: (rad - 0.5) * rr * 0.62 };
    }
    case 'list_fold': {
      const x = (u - 0.5) * 0.78;
      const y = Math.sin(u * TAU * 1.5) * 0.07 + (rad - 0.5) * 0.04;
      return { x, y, z: Math.sin(u * TAU * 3) * 0.05 };
    }
    case 'tree_fold': {
      const side = Math.cos(theta) >= 0 ? 1 : -1;
      const y = -0.30 + rad * 0.56;
      return { x: side * rad * 0.28, y, z: Math.sin(theta * 2) * 0.08 * rad };
    }
    case 'mutual_even_odd': {
      const wing = nx >= 0 ? 1 : -1;
      return { x: wing * (0.08 + rad * 0.28), y: ny * 0.85, z: 0.02 * wing + nz * 0.15 };
    }
    case 'lexical_let': {
      const a = u * TAU;
      const r = 0.10 + rad * 0.16;
      return { x: Math.cos(a) * r, y: -0.02 + rad * 0.16, z: Math.sin(a) * r };
    }
    case 'checked_underflow': {
      const a = u * TAU;
      const phi = 0.15 + rad * Math.PI * 0.55;
      const R = 0.30;
      return {
        x: R * Math.sin(phi) * Math.cos(a),
        y: 0.08 + R * Math.cos(phi),
        z: R * Math.sin(phi) * Math.sin(a),
      };
    }
    case 'stack_overflow': {
      const a = u * TAU;
      const y = -0.16 + rad * 0.48;
      const R = 0.10 + rad * 0.22;
      return { x: R * Math.cos(a), y, z: R * Math.sin(a) };
    }
    case 'fuel_exhaustion': {
      const a = u * TAU;
      const b = rad * TAU;
      const R = 0.30;
      const r = 0.062;
      return {
        x: (R + r * Math.cos(b)) * Math.cos(a),
        y: (R + r * Math.cos(b)) * Math.sin(a) * 0.55,
        z: r * Math.sin(b),
      };
    }
    default:
      return { x: nx, y: ny, z: nz };
  }
}
```

Inside the segment loop, replace both branches with:

```javascript
const fit = fitOf(artifact.wovenBounds);
const raw = sampleRidge(ridge, u);
let pos = { x: 0, y: 0, z: 0 };
if (raw) {
  const nx = (raw.x - fit.cx) * fit.s;
  const ny = (raw.y - fit.cy) * fit.s;
  const nz = (raw.z - fit.cz) * fit.s;
  pos = embedOnSpecies(speciesId, nx, ny, nz, fit.flat);
}
_sdkPts[s].set(pos.x, pos.y, pos.z);
```

Material, same mesh:

```javascript
const sdkRibbonMat = new THREE.MeshPhysicalMaterial({
  color: 0xffffff,
  vertexColors: true,
  emissive: new THREE.Color(0x041018),
  emissiveIntensity: 0.85,
  roughness: 0.42,
  metalness: 0.02,
  transparent: true,
  opacity: 0.42,
  side: THREE.DoubleSide,
  depthWrite: true,
  polygonOffset: true,
  polygonOffsetFactor: -1,
  polygonOffsetUnits: -1,
});
```

Drop half-width from `0.0065` to `0.0035` so the filaments read as caustics on the body, not as a second organism. Hide the mesh when `ridges.length === 0` rather than drawing a placeholder helix.

Acceptance: the barrel is gone on all eight presets. Factorial filaments follow the spire. Fuel filaments lie on the torus. The medusa cage is a set of curves on the bell. Swapping species changes the filament shape.

---

## C. Species geometry and shader — exact edits in `designer8_quineling.js`

Stay on `MeshPhysicalMaterial` from `three/webgpu` `^0.175`. One organism material helper beats eight candy presets. Tissue: `roughness` 0.38–0.55, `metalness` ≤ 0.06, `clearcoat` 0.25, `clearcoatRoughness` 0.4, `transmission` only on the bell and the nautilus hood, `thickness` 0.4, `attenuationDistance` 0.6, `ior` 1.35. Nacre and the tokamak are the only high-iridescence / higher-metal surfaces. Emissive intensity stays a pulse of the species accent, not a flat 0.5 wash. The core point light (`intensity` 1.35, `distance` 2.5) plus `clearcoat: 1` is why every capture looks injection-molded; cap that light at `0.7` and add a second `PointLight` at local `(0.2, 0.15, 0.25)`, intensity `0.35`, color `0x9ad7ff`, as the rim.

### 01 Nautilus — one conispiral tube

Delete the 28-torus loop (lines 544–558). Build one `CatmullRomCurve3` and a `TubeGeometry`.

```javascript
const nautilusPts = [];
for (let i = 0; i <= 64; i += 1) {
  const u = i / 64;
  const theta = u * 2.05 * TAU;
  const rad = 0.02 * Math.exp(0.42 * theta); // outer ≈ 0.42, under the 0.46 budget
  nautilusPts.push(new THREE.Vector3(
    Math.cos(theta) * rad,
    Math.sin(theta) * rad * 0.55 + 0.02,
    Math.sin(theta) * rad * 0.34,
  ));
}
const nautilusShell = new THREE.Mesh(
  new THREE.TubeGeometry(new THREE.CatmullRomCurve3(nautilusPts), 96, 0.02, 12, false),
  nautilusShellMat,
);
```

Place the five septa on that curve, `quaternion` aligned to the tangent, radius = local tube radius. Move the hood to the last point. Scale the tube radius along the curve with a second, thinner tube if a flare is required. `z = sin(u·π)·0.02` (line 554) is what flattened the shell into a ring scribble.

### 02 Siphonophore — one ribbon, three bells

Replace the 18 boxes (lines 646–663) with a single strip from `writeRibbonStrip` along `x = lerp(-0.40, 0.40, u)`, `y = sin(u·TAU·1.5)·0.07`, `z = sin(u·TAU)·0.05`, half-width `0.045`. Keep the three bells on that curve. `BoxGeometry(0.016, 0.15, 0.22)` is the card stack in the capture.

### 03 Coral — blended limbs, corrected radius

`alignCylinderBetween` does `mesh.scale.set(radiusA, len, radiusB)` on `CylinderGeometry(0.72, 1.0, 1, 14)`. World radius becomes `0.72·rA` on X and `1.0·rB` on Z, so every limb is an ellipse and the joints gap. Use a unit cylinder `CylinderGeometry(1, 1, 1, 16)` and scale `(rA, len, rB)` only if `rA === rB`; otherwise generate a `LatheGeometry` or a tube through the existing `pA/pB` polyline (`treeBranchDefs`) with radius interpolated along the path. Give each limb a Z sag of at least `0.06` so the Y is not a paper cutout. Swap the six `ConeGeometry` petals for short `SphereGeometry` scaled `(1.4, 0.45, 0.7)` sitting on the tip normal.

### 04 Moth — winged membrane and a real Lorenz tube

Replace `CircleGeometry` fore/hind wings with a `Shape` (leading ellipse + a scalloped trailing edge) extruded `0.004` via `ExtrudeGeometry`, `side: DoubleSide`. Add 5 `Line` veins in the wing pivot. Delete `lorenzLeftLoop` / `lorenzRightLoop`. The SDK already integrates Lorenz (`integrateLorenz` inside `WovenBody` for `lorenz-flow`). Tube the compiled orbit:

```javascript
const orbit = artifact.wovenBody.orbit; // present for mechanism 'lorenz-flow'
if (orbit?.length > 8) {
  const curve = new THREE.CatmullRomCurve3(
    orbit.map(([x, y, z]) => new THREE.Vector3(x, y, z).multiplyScalar(0.9)),
  );
  // rebuild TubeGeometry(curve, 180, 0.004, 6, false) when the species becomes active
}
```

The two tori at `(±0.14, 0, ±0.06)` are why the capture shows glass rings instead of a figure-8.

### 05 Lotus — bloom upward

`ConeGeometry` points `+Y`. Lines 884–886 park petals at `y = -0.14` and tilt them with positive `rotation.x`, which is the downward shrapnel in the capture. Set:

```javascript
pMesh.position.set(Math.cos(angle) * r, 0.02 + ring * 0.035, Math.sin(angle) * r);
pMesh.rotation.order = 'YXZ';
pMesh.rotation.y = -angle;
pMesh.rotation.z = -0.7 - ring * 0.18; // tip up and out
```

Turn off `wireframe` on the middle and outer cubes. Use `opacity` 0.22 / 0.12 and `transmission` 0.65 with `depthWrite: true` on the inner cube only. Wireframe plus transmission is the scribbled cage around the seed.

### 06 Medusa — spline tentacles

Keep the bell. It is the one mesh that already reads as tissue; only the material needs `thickness: 0.45`, `attenuationColor` set from `0xf43f5e`, `roughness: 0.22`, `metalness: 0`. Replace `CylinderGeometry(0.011, 0.003, 0.44)` with a 6-point `CatmullRomCurve3` per tentacle, rebuilt each frame:

```javascript
const hang = -0.06 - s * 0.07;
const sway = Math.sin(ambient * 1.6 + jt.index + s) * (isFault ? 0.02 : 0.012) * s;
pts[s].set(
  Math.cos(jt.angle) * (jt.rad + sway),
  hang,
  Math.sin(jt.angle) * (jt.rad + sway),
);
```

Tube radius `0.008 → 0.002`. Stop at local `y = -0.42` so the tips stay above the plaque band.

### 07 Comet — closed tapered tail

`ConeGeometry(0.14, 0.54, 20, 1, true)` is open, which is the hollow orange sheet. Pass `openEnded = false`, or use a tube whose radius runs `0.07 → 0.01` along the lance. Parent the five shock rings to the lance axis and scale their tube radius with `1 - rIdx/5` so they thin as they pancake, instead of five identical `TorusGeometry` hoops of tube `0.015`.

### 08 Tokamak — braid on the tube, not a free knot

`TorusKnotGeometry(0.32, 0.014, 96, 12, 1, 3)` uses Three’s curve `(radius·(2+cos(qφ))/2)`, so the XY radius reaches `1.5·0.32 = 0.48` and leaves the fuel tube. Build the `(1,3)` winding on the torus of major `0.32`, minor `0.07`:

```javascript
const braidPts = [];
for (let i = 0; i <= 180; i += 1) {
  const a = (i / 180) * TAU;
  const b = 3 * a;
  const R = 0.32 + 0.07 * Math.cos(b);
  braidPts.push(new THREE.Vector3(R * Math.cos(a), R * Math.sin(a), 0.07 * Math.sin(b)));
}
const torusBraidMesh = new THREE.Mesh(
  new THREE.TubeGeometry(new THREE.CatmullRomCurve3(braidPts, true), 240, 0.006, 8, true),
  torusBraidMat,
);
```

Drop `torusGroup.rotation.x` from `0.42` to `0.28` so the front of the ring stays in the upper half of the interior frame. Keep the 14 arc segments; their on/off fuel coloring is already correct.

### Spores and tendrils

`sporeMesh` (lines 1417–1424) is the same golden-angle ball on every species, which is the diamond confetti in every capture. Position each instance with `embedOnSpecies(speciesId, sx, sy, sz, true)` and a radial offset of `0.02` along the species normal so the motes skin the body.

Chamber tendrils (lines 1449–1480) start at local `y = ±0.18`, `r = 0.22`, which is inside the bell, the shell, and the torus. Start them at `r = QUINELING_MAX_RADIUS + 0.04` and fade the first two segment alphas to `0`. Set tendril `depthWrite: true` and `opacity: 0.35`. The current rays are the colored lines that cross the floor labels.

---

## Done when

1. Frames 06, 07, and 08 show `MEDUSA SUBTRACTOR`, `COMETA ABYSSI`, and `CYCLOPORA EXHAURIRE` left-to-right at the same `ambient` that now produces `ROTARTBUS`, `ISSYBA`, and `ERIRUAHXE`.
2. The plaque’s screen rect stays above the floor-chip row for the interior camera at yaw `0`, `π/2`, and `π`.
3. No species shows a vertical barrel of horizontal white stripes. Filament shape changes with `speciesId`.
4. Nautilus outer radius ≤ `0.46`. Fuel braid lies on the tube. Lotus petals point up. Moth wings are one membrane each. Coral limb cross-sections are round and meet.
5. Bell and hood are the only meshes with `transmission > 0`. Nothing organic uses `metalness > 0.08` except the tokamak segments and the nautilus nacre.
