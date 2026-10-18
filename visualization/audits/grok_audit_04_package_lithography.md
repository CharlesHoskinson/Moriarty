# Grok 4.7 xhigh Auditor #4 — Apple Silicon Package & Die Lithography

Auditor: Apple Hardware Design pass on the exterior package, ceramic/titanium proportions, chamfer tolerances, engraved typography, InstancedMesh batching, and visual restraint.

Scope read in full:

- `src/designers/designer1_package.js` (867 lines)
- `src/designers/designer5_circuit.js` (243 lines)
- `src/designers/designer6_loom.js` (173 lines)
- `src/designers/ring_study_07_package.md` (43 lines)

Cross-checked against `DESIGNER_SPEC.md` (Wave 2 envelope), `research_d1.md`, `research_d5.md`, `research_d6.md`, `ring_study_05_chip_materials.md`, `theme.js` texture helpers, and the installed Three.js `ExtrudeGeometry` / `InstancedMesh` / frustum path (`node_modules/three`).

Scale used for millimetre callouts: package plan is `±2.15`, read as a 43.0 mm module, so **1 world unit = 10 mm**. Chamfer targets below are in both units. This is an audit scale, chosen so a C0.30 chamfer is `0.030` world. It is not a claim about a specific M-series drawing.

Datum for the next iteration: **lid top face `y = 0.110`**. Every lithography mark, engraving plane, and chamfer is measured from that face. Today the lid top is `y = 0.105` and several "recessed" features sit above it.

---

## Verdict

The package is a seven-solid layer cake: plinth, lower alabaster, proud champagne-gold seam, upper alabaster, full-height gold well collar, silver-platinum lid, and a second chrome-gold aperture ring. An M-series module reads as two solids (matte substrate, satin lid) plus one tone-on-tone mark. Ring study 07 and the code agree with each other, and both leave the Apple proportion, finish, and die-floorplan behind.

The single most damaging material bug is on the lid. `roughness: 0.24` is multiplied by a roughness map whose texels sit near `62/255`. The brushed silver deck renders at about **0.06 roughness** — polished metal — so the brush grain cannot show. The ceramic has the matching albedo bug: `color: 0xebe5da` multiplies a map that is already `#ebe5da`.

The lithography on the lid is two full-length equalizer slots and a 32-segment ticker. Designer 5's four quadrants (ROM, L4 arenas, aux/1, aux/2) and designer 6's `ReferenceSlice` brackets are gone. What remains is bright, unlit, and proud of the metal.

Batching shape is the right idea (one `InstancedMesh` per array, module-scope scratch colors). The per-frame discipline is not: designer 1 rewrites 32 pad colors and reparses the source string every frame, designer 5 rewrites 32 LED colors every frame and allocates empty arrays on the idle path, and none of the three sets `instanceColor.setUsage(DynamicDrawUsage)`. Designer 2 already does that last part.

---

## What already matches the hardware bar

- Gull-wing leads, SMD forests, and truss gantries are gone. The plan is one rounded square, `±2.15`, with a single bore near `±1.08`.
- 32 edge pads are one `InstancedMesh`. Designer 6 dirty-checks `totalSteps`, `stepIndex`, and `phase` before rewriting instance colors. Designer 5 shares one geometry and one material across both flank arrays.
- Scratch `Matrix4` / `Vector3` / `Quaternion` / `Color` objects live at module scope. Steady-state pad coloring does not construct `THREE.*` objects.
- The bench is warm paper (`roughness: 0.95`) with one painted contact shadow and four hairline traces. That is the correct quiet stage.
- `three/webgpu` imports and `MeshStandardMaterial` / `MeshBasicMaterial` stay on the node-material path. Dispose paths exist.
- Designer 6's segment duty cycle (`pitch * 0.78`, floor `0.012`) is a believable watch index. The color and the placement are the problem, not the segmentation math.

---

## Findings

### P0 — Lid roughness is polished, not brushed

`designer1_package.js` lines 205–256 and 696–707. `ring_study_07_package.md` line 13 specifies `roughness: 0.24`, `metalness: 0.78`.

`getBrushedSilverPlatinumTextures()` paints a roughness canvas whose base is `rgb(62, 62, 62)` with streaks roughly in `28..110`. Three.js defines the sampled roughness as `material.roughness * roughnessMap.g`. With scalar `0.24` and texels near `0.24`, the deck lands near **0.06**, and the streak delta collapses to about `±0.02`. Specular from the four-light rig will blow the top face out. The albedo brush (`createBrushedMetalTexture`, streak alpha `0.08`) cannot compete with that highlight.

The same texture is assigned to `bumpMap`. Bump needs a zero-mean height centered at `128`. A map centered at `62` is a legal height only because bump uses derivatives, but the grooves are then locked to the roughness signal and scaled by `bumpScale: 0.0015`. After the roughness multiply, there is no anisotropic roughness left for the grooves to explain.

`material.color` is `0xd8dde6` while the albedo map is already filled with `#d8dde6`. Sampled albedo is `color * map`. The lid is a darker, mirror-finished silver, not the specified platinum.

`ExtrudeGeometry`'s `WorldUVGenerator` writes raw shape coordinates into the UVs (`ExtrudeGeometry.js` `generateTopUV`). The cap UVs span about `-2.15..2.15`, and the metal texture repeat stays at `(1, 1)`. That part is acceptable (a few brush tiles across the deck). The side-wall generator uses absolute X/Y against `1 - z` over a wall only `0.025` tall, so the brush on the lid edge is a smear. `normalizePlanarUVs` is applied to the bench only.

Next iteration, one material:

- Albedo map carries `#d8dde6`. Material `color` is `0xffffff`.
- `roughness: 1.0`. Roughness map is linear, mean pixel `~105` (`0.41`), streak range `80..140`, `NoColorSpace`.
- `metalness: 0.72`. No metalness map.
- `bumpMap` is a separate texture, mean `128`, `bumpScale: 0.0012`. Grooves align with the dark roughness streaks.
- `MeshPhysicalMaterial.anisotropy` about `0.55`, rotation `0`, so the highlight stretches along the brush (world X on the cap).
- Rebuild cap UVs into `0..1` across the lid plan, and give the sidewall a uniform world-unit scale (about 4 repeats per 10 mm on the ceramic, 8 per 10 mm on the metal).

Bead-blasted titanium in the Wave 2 note (`research_d1.md`: `0x6e6d68`, roughness `0.34`) is the closer Apple finish. If the alabaster direction from ring study 05 is the one that stays, the lid still has to be satin. Polished chrome is a jewelry finish.

### P0 — Ceramic albedo is double-applied, and the bump does not match the grain

`designer1_package.js` lines 96–198 and 504–513. Ring study 07 says `0xebe5da`, `roughness: 0.42`, `metalness: 0.08`, `bumpScale: 0.004`. Ring study 05 says the same hex at `roughness: 0.68`, `metalness: 0.06`.

`getAlabasterCeramicTextures()` calls `THEME.createCeramicGrainTexture({ size: 512, baseHex: '#ebe5da' })` and then ignores the result, because `_cachedCeramicBumpTex` is still null. It rasterizes a second 512² pebble map (18,000 rects plus 2,400 flecks), stores that unused color texture in `_cachedCeramicColorTex`, then calls `createCeramicGrainTexture` again and returns the theme map plus the local bump.

Those two maps disagree:

| Map | Source | Repeat | Character |
| :--- | :--- | :--- | :--- |
| Albedo actually bound | `theme.js` `createCeramicGrainTexture` | `(3, 3)` | `#ebe5da` wash, grain alpha `0.055` |
| Bump actually bound | local 18,000-rect raster | `(1.5, 1.5)` | hard 1–2 px pebbles, mean `128` |
| Albedo leaked | local raster, never assigned | `(1.5, 1.5)` | a third grain |

`material.color` is also `0xebe5da`, so the theme map is multiplied by itself and the body goes brown. `roughness: 0.42` and `metalness: 0.08` then put a glaze highlight on that brown. Sintered package ceramic is matte and dielectric.

Next iteration:

- One singleton pair from `theme.js`: albedo and a bump built in the same loop, same `repeat`, same seed. Delete the local 18,000-rect painter.
- Material `color: 0xffffff` when the map carries the porcelain.
- `roughness: 0.78`, `metalness: 0.02`, `bumpScale: 0.003`.
- Sidewall UVs in world millimetres, same density as the cap. Today's sidewall UVs span only the extrusion depth, so the pebble grain stretches into horizontal streaks down the flank.

### P0 — The "diamond chamfer" is a second gold part on an elliptical fillet

`buildHollowLayer` (`designer1_package.js` lines 537–567) sets `bevelThickness` and `bevelSize` equal and `bevelSegments: 2`. Three.js `ExtrudeGeometry` profiles that bevel as a quarter ellipse:

```
z  = bevelThickness * cos(t * π/2)
bs = bevelSize      * sin(t * π/2)
```

`bevelSegments` only tessellates the ellipse. Two segments is a faceted roundover. There is no 45° face in this geometry. Ring study 07's "diamond-cut polished chrome-gold inner aperture bezel" is a separate extrusion (`outerHalf: 1.124`, `yBottom: 0.082`, `height: 0.024`, bevel `0.002`) in `0xe5c86b`, roughness `0.16`, metalness `0.90`.

Stack as built, top face at `yBottom + height` (bevel is included in the height):

| Solid | Plan half | Y range | Corner radius | Bevel |
| :--- | ---: | :--- | ---: | ---: |
| Plinth `0xc8c0b0` | 2.05 | −0.340 .. −0.220 | 0.14 | 0.005 |
| Lower ceramic | 2.15 | −0.220 .. −0.067 | 0.16 | 0.007 |
| Gold seam, proud by 0.005 | 2.155 | −0.067 .. −0.053 | 0.16 | 0.002 |
| Upper ceramic | 2.15 | −0.053 .. 0.080 | 0.16 | 0.007 |
| Gold well collar | 1.104 | −0.220 .. 0.080 | 0.085 | 0.002 |
| Silver lid | 2.15 | 0.080 .. 0.105 | 0.16 | 0.0025 |
| Chrome-gold bezel | 1.124 | 0.082 .. 0.106 | 0.086 | 0.002 |

Six bevel sizes, four corner radii, two gold alloys. The bezel top is `0.001` above the lid and its hole is coplanar with the lid hole, so the gold reads as a decal ring, and the shared bore wall z-fights the titanium. The lid sidewall flares to about `2.1525` while the ceramic sidewall flares to about `2.157`, an accidental `0.0045` reveal (0.045 mm). A designed ceramic reveal is `0.060` (0.60 mm), constant, and it does not depend on `bevelSize`.

Hole paths use the same CCW `createRoundedSquarePath` as the shell. Three.js expects the opposite winding for holes. The top cap is emitted at bevel parameter `t = 0` (offset 0), and the hole was pre-inflated by `innerHalf + bevel`, so the visible top opening is `1.08 + bevel`, not `1.08`. Ceramic opens to about `1.087`, the gold seam and collar to about `1.082`. The seam therefore leaves a gold lip inside the bore. The bench hole is a sharp `1.08` with radius `0.08` and no bevel, so the paper, the collar, and the lid do not share a bore cylinder.

`stepState.aperture` is produced by the engine and never read. The four-leaf shutter in `research_d1.md` (`y = 0.094`) is absent. The bore is fully open in `STILL`, where the engine sets `aperture` near `0.18`.

Next iteration, two solids and one chamfer spec:

1. **Substrate.** One ceramic extrusion, `outerHalf: 2.15`, `cornerRadius: 0.10` (1.0 mm), `yBottom: -0.280`, `height: 0.320`, top at `y = 0.040`. Delete the plinth. The paper stays at `y = -0.340`, so the body sits `0.060` (0.6 mm) above the sheet and the painted contact shadow is the only shadow. `castShadow` on the body can stay; the plinth must not.
2. **Seam.** A groove, not a ring. Recess `0.004` deep and `0.012` tall at `y = -0.080`, `outerHalf` no greater than the ceramic sidewall. Color `PALETTE.ceramicEdge` (`0xd5cec0`). No second metal.
3. **Lid.** `outerHalf: 2.09` (reveal `0.060`), `cornerRadius: 0.080`, `yBottom: 0.040`, `height: 0.070`, top at `y = 0.110`. Same satin titanium as the material fix above. Inset is modeled in the plan, not as a side effect of bevel.
4. **Aperture chamfer.** One 45° face in the lid metal, from plan `1.140` at the top face down to bore `1.080` over `Δy = 0.020`. `bevelSegments` cannot make this. Build it as a dedicated ring (four trapezoids, or a lathed profile with one flat). Inner corner radius `0.040`. Delete the chrome-gold bezel and the full-height gold collar. The well liner, if the shaft needs a lip, is `PALETTE.wellSlate` `0x161d29`, roughness `0.72`, metalness `0.0`, flush with `x, z = ±1.080`.
5. **Bore contract.** Every opening that claims `±1.08` uses one shared `roundedRectHole(1.08, 0.04)` wound clockwise. No per-layer `+ bevel` inflation. Measure the resulting top-cap vertex radius in a unit test and reject anything outside `1.080 ± 0.002`.
6. **Iris.** Four graphite leaves at `y = 0.104`, thickness `0.004`, driven by `stepState.aperture` with the existing `easeInOutCubic`. Closed (`aperture = 0`) covers the bore; open parks the leaves under the lid land between `1.14` and `1.46`. If the iris is cut from the product, delete it from the research note and stop publishing `aperture` as a package input. A kinematic the mesh ignores is a broken instrument.

Chamfer tolerance for every remaining exterior edge: **C0.20** (`bevelSize = bevelThickness = 0.020`) on the substrate and lid outer silhouette, **C0.10** on the seam groove. One radius family. `bevelSegments: 4` only where a fillet is honestly the design; the aperture is the 45° ring.

### P0 — Edge pads are proud gold bars that cut the seam

`designer1_package.js` lines 627–668. `BoxGeometry(0.026, 0.092, 0.128)` centered at `(2.149, -0.02, offset)` before the yaw quaternion.

East-face extent is `x ∈ [2.136, 2.162]`. The ceramic top-face plan is `2.15` and the flared sidewall is about `2.157`, so each pad stands **~0.05 mm to 0.12 mm proud** and is `0.92 × 1.28` mm in section — a bus bar, not a flush land. Pad `y ∈ [-0.066, 0.026]` crosses the seam `y ∈ [-0.067, -0.053]`. All 32 pads intersect the proud gold ring.

Pitch `0.44` from `-1.54` to `1.54` does stay on the flat: outermost pad ends at `1.604`, corner curve starts at `2.15 - 0.16 = 1.99`. Keep that clearance.

`update` (lines 804–846) recolors every pad every frame. `STILL` still runs `sin(ambient * 2.0 + p * 0.2)`. `DESCEND` flips all four sides between `0x2dd4bf` and `0xfbbf24` at `ambient * 5.5`. Idle Apple hardware is still. Status light belongs to one edge, at one index, at a breath period near 3 s, and only while a phase is active.

Next iteration:

- Pocket the land. Pad `0.090 × 0.050 × 0.006`, face `0.002` below the ceramic sidewall. One `InstancedMesh`, matrices static.
- Place the row in the lower ceramic only, center `y = -0.120`, clear of the groove at `y = -0.080` by at least `0.030`.
- Idle color `0xd4af37` once. On `INGEST`, walk a single index along the west and north edges. On `RETURN` / `EMBLEM`, walk the east and south edges. On `ERROR`, hold all 32 at `PALETTE.errorRed` without a sine. On `STILL`, write colors once and return.
- Set `instanceColor.setUsage(THREE.DynamicDrawUsage)` when the color attribute is created. Leave `instanceMatrix` on static usage.
- Skip the write when `phase === lastPhase` and the phase is `STILL`.

### P0 — Engraving is an unlit monospace sticker

`designer1_package.js` lines 387–406 and 742–779. Ring study 07 calls this a laser etch with a specular bevel.

`paintRimEngraving` draws JetBrains Mono at weight `700`, `letterSpacing: 0.12em`, size `56` north and `58` south, in `#18202c`, with a white fill shifted `1.5` px as a fake bevel. The mesh is `MeshBasicMaterial`, `transparent`, `depthWrite: false`, `toneMapped: false`. Unlit text does not take the studio grade, and the white offset halos on `#d8dde6`. Package laser etch is tone-on-tone in the metal: a roughness change you see in raking light, in a humanist sans at regular weight, small caps or a light roman. It is not source-code bold.

Placement is asymmetric and crowded:

- North plane `1.92 × 0.32` at `(0, 0.1058, -1.56)` covers `z ∈ [-1.72, -1.40]`.
- South plane `1.90 × 0.34` at `(0, 0.1058, +1.58)` is `0.02` off the mirror.
- The progress channel sits at `z = -1.92` (`z ∈ [-1.934, -1.906]`). Gap to the wordmark is `0.186`. Both live in the north land, which is only about `1.03` deep (`1.124` to `2.15`). South has type and no scale. The package has a front and a back and then pretends to be a symmetric module.

`extractProgramExpressionLabel` runs every `update`: `match`, `trim`, and `toUpperCase` allocate on the hot path. The study's zero-allocation claim does not hold. An empty trace engraves the hardcoded `FACT(4)`. Canvas text is baked at construction; if JetBrains Mono finishes loading later, the plaque keeps the fallback until the label string changes.

Next iteration:

- One engraving material: `MeshStandardMaterial`, `color: 0x3a414c`, `roughness: 0.62`, `metalness: 0.35`, `transparent: true`, `opacity: 0.82`, tone mapping left on. No white under-draw. `polygonOffset` so the plane sits `0.0004` above `y = 0.110` without fighting the lid.
- Face: `"Inter", "Helvetica Neue", system-ui`. Weight `500`. Tracking `0.18em`. One size, cap height `0.050` world (0.50 mm) on both rims. North copy `SHERLOCK · K-A/1`. South copy is a mapped short mark (`FACT 4`, `SUM LIST`, `SUM TREE`), 12 characters maximum, measured with `measureText` before the ellipsis. Empty trace engraves nothing.
- Optical center of each land. With the lid at `outerHalf 2.09` and the chamfer starting at `1.14`, the band center is `z = ±1.615`. Plane `1.40 × 0.14`. If the progress hairline stays on the north lip, move the north wordmark to `z = -1.50` and keep `≥ 0.12` between mark and scale. Mirror the south number exactly.
- Repaint from `document.fonts.ready`, and only when `trace.source` or `trace.programName` changes identity. Cache the last source reference. Do not parse on other frames.
- Four status words (`FUEL`, `STACK`, `TYPE`, `PROOF`) from `research_d1.md` stay off the lid. The pads and the progress hairline already speak. Extra micro-dots would crowd the reveal.

### P0 — Designer 5 is two equalizer slots, not a die floorplan

`research_d5.md` specifies four flush pockets on `y = 0.106..0.116`, relief `≤ 0.012`:

- NW ROM, 16 micro-strips plus one gold scan slit
- NE six SRAM-style arena fills
- SW four aux/1 lanes, eight dots each
- SE a 16×3 limb / carry / fuel matrix

`DESIGNER_SPEC.md` gives those quadrants real rectangles around the bore. `designer5_circuit.js` replaces all of that with two channels at `x = ±1.62`, `z ∈ [-0.98, 0.98]`, each a `BoxGeometry(0.38, 0.0012, 2.08)` at `y = 0.1054` and 16 `BoxGeometry(0.32, 0.002, 0.038)` LEDs at `y = 0.106`.

Lid top is `0.105`. The channel occupies `y ∈ [0.1048, 0.1060]` and the LED occupies `y ∈ [0.105, 0.107]`. Ring study 07's "recessed" channel is **0.001 proud of the lid and intersecting it by 0.0002**. The LED bottoms are coplanar with the lid top. This will shimmer under camera motion.

Pitch is `(0.98 - -0.98) / 15 = 0.1307` against a mark depth of `0.038`, duty cycle **29%**. A long sapphire slot behind a dotted line spends the dark area of both flanks and still does not name a block. The slots also run through the middle of the lid (`z` crosses 0), so the eye meets two dark bars before it meets the bore.

Color is `MeshBasicMaterial`, `toneMapped: false`, at full `0x2dd4bf` and `0xfbbf24`. Those are UI accents. Research D5's ember `#e86446` and champagne `#f2c66d` at least belonged to a metal scene. Unlit full-chroma cyan next to satin metal is a graphic equalizer.

The left index is `exprId % 16`. Expression 0 and expression 16 share a pad. Every `i < exprCount` stays in teal (`0x1a9e8f` / `0x185a56`), so a program longer than 16 lights the whole strip. The null check after `??` is dead: `null` has already become `0`.

The right strip folds limbs, carries, borrows, and fuel bytes into one row. Any non-zero fuel byte paints champagne while arithmetic is idle, so the ALU picture is often a fuel thermometer. The ripple (`ambient * 3`) runs only in the empty-but-active branch, yet `setColorAt` and `needsUpdate` run for all 32 instances on every frame, including `STILL`.

`lhsBytes = arith?.lhsBytes || []` (and the same for rhs, result, carries, borrows, fuel) allocates a fresh array whenever aux/2 is absent. That is the common frame.

Next iteration — restore the four pockets, flush to the new lid datum `y = 0.110`:

| Pocket | Plan (inset 0.12 from the lid edge and 0.10 from the bore) | Content | Instancing |
| :--- | :--- | :--- | :--- |
| NW ROM | `x ∈ [-1.94, -1.24]`, `z ∈ [-1.94, -1.24]` | 16 strips, duty `≥ 0.85`, gap `0.008`. One scan slit, `0.0015` thick, lerped to `exprId` clamped into range. No modulo. | strips + slit |
| NE arenas | `x ∈ [1.24, 1.94]`, `z ∈ [-1.94, -1.24]` | 6 fills driven by `machineImage` counts. Unallocated length stays pocket color. | 6 fill instances, scale.x = used fraction |
| SW aux/1 | `x ∈ [-1.94, -1.24]`, `z ∈ [1.24, 1.94]` | 4 lanes × 8 dots. `activeKind === 'None'` leaves every dot at pocket color. | 32 dots |
| SE aux/2 | `x ∈ [1.24, 1.94]`, `z ∈ [1.24, 1.94]` | 16 × 3: row 0 result bytes, row 1 carry/borrow, row 2 fuel. Overflow uses `PALETTE.heatBright` on the offending cell only. | 48 cells |

Pocket floor `y = 0.104`, wall color `0x161d29`, roughness `0.72`. Mark top `y = 0.109`, so the mark sits `0.001` below the lid face. Marks use `MeshStandardMaterial` with `emissive` set to the semantic color and `emissiveIntensity` in `0.35..0.9`, tone mapping on, base color the pocket slate. Active ROM is champagne `0xd4af37`. Resolved arena fill is `0x45937a` at intensity `0.5`. Standby cells are not emissive.

One `InstancedMesh` for all marks that share the unit box (about 16 + 6 + 32 + 48), plus one mesh for the four pocket floors (or a second instanced quad). Set `instanceColor` to `DynamicDrawUsage`. Rewrite colors only when `exprId`, `machineImage`, `aux1`, `aux2`, or `phase` changes; drive the scan slit from a uniform or from `stepFraction`, not from a full recolor. Module-scope `const EMPTY_BYTES = []` replaces every `|| []`.

Labels `ROM`, `STATE`, `AUX/1`, `AUX/2` use the engraving material, cap height `0.040`, tracking `0.14em`, placed just outside each pocket. No cyan captions.

### P1 — Designer 6 is a ticker on the wordmark, and it ignores slices

`research_d6.md` places a floating `ReferenceSlice` ribbon at `y = 0.24`, `z = -1.48`, with brackets of width 1 / 2 / 4 / 8, a drop thread, and one proof prism. `designer6_loom.js` is a 32-segment bar at `y = 0.106`, `z = -1.92`. Ring study 07 documents the bar. The code matches the study and misses the data contract: `trace.slices`, `trace.sliceWidth`, and `stepFraction` are never read.

The channel uses the same proud `y = 0.1054` as designer 5. Completed steps are unlit `0x2dd4bf`, the active step unlit `0xfbbf24`, pending `0x243042`. A full cyan run of past steps is louder than the bore.

Hidden instances stay in the draw: `count` remains 32 and unused slots are scaled to `0` and dropped to `y = 0.056`. `InstancedMesh.computeBoundingSphere` (used by `Frustum.intersectsObject`, because the mesh owns a `boundingSphere`) runs once, when the sphere is still null. Later `totalSteps` changes do not recompute it. Today's span is fixed at `BAR_SPAN_X = 2.04`, so the stale sphere still covers the bar. The next time the span or `z` changes, the bar will cull wrong. Set `count` and call `computeBoundingSphere()` after every layout.

`instanceMatrix` correctly uses `DynamicDrawUsage`. `instanceColor` does not.

Next iteration — one hairline scale, slice-aware, sharing the engraving's quietness:

- Recessed track at the north lip: `z = -1.96`, `y` center `0.106` on the new lid (top of mark at `0.109`), depth `0.016`, length `1.84`. Pocket color `0x161d29`.
- Group segments by `trace.sliceWidth` (1, 2, 4, or 8). Intra-slice gap `0.006`. Inter-slice gap `0.028`. Past slices emissive `0x45937a` at intensity `0.45`. The active segment emissive `0xd4af37` at intensity `0.85`. Future segments are the pocket color with no emissive — etched pits, not a second LED color.
- `ERROR` lights only the active segment in `PALETTE.errorRed`.
- `progressInstanced.count = totalSteps`. After layout, `computeBoundingSphere()`. Color writes stay behind the existing dirty check.
- The proof prism and parabolic threads, if they return, live above the rim at `y ≥ 0.24` per research D6. They do not sit on the lid, and they do not share the wordmark band.

### P1 — Per-frame batching discipline

| Mesh | Draw count | Matrix usage | Color usage | When colors upload |
| :--- | ---: | :--- | :--- | :--- |
| `edgePadsInstanced` (32) | 1 | static default | static default, written dynamically | every frame, all phases |
| `leftExprInstanced` + `rightLimbInstanced` (16+16) | 2 | static | static default | every frame |
| flank channel pair | 2, shared geo/mat | static | n/a | never |
| `progressInstanced` (32, including scaled-off) | 1 | dynamic | static default | on step, phase, or layout change |

Designer 2 sets `instanceColor.setUsage(DynamicDrawUsage)` on its fuel, stator, and rule meshes. These three files should do the same the moment `setColorAt` has created the attribute.

Merging the two flank meshes is worth it only after the four-pocket rebuild. A single unit-box `InstancedMesh` for every lithography mark is the target draw, not two hard-coded strips.

`tagInspectable` on designer 5 is attached to the LED meshes, whose geometry bounding volume is a 0.32-wide box at the origin. Raycast uses `InstancedMesh.boundingSphere`, which does include instance matrices once computed, so picking can work after the first hover. Call `computeBoundingSphere()` at the end of setup anyway, so the first pick does not depend on a prior frustum test.

### P2 — Documents that will mislead the next pass

Ring study 07 should be rewritten with the code, or the code should be rewritten to a corrected study. As it stands the study states these falsehoods:

- Channels at `y = 0.1054` are recessed into a lid whose top is `y = 0.105`. They intersect the lid and finish above it.
- The aperture bezel is diamond-cut. It is a second extrusion on a two-segment elliptical fillet.
- Ceramic roughness `0.42` matches the porcelain study. Ring study 05 specifies `0.68`, and the bound roughness is `0.42` with a doubled albedo.
- Designer 5 and 6 are the lithography. They are a reduction that drops quadrants, slice brackets, and the proof prism.
- `.update` is zero-allocation. Designer 1 parses the source every frame. Designer 5 allocates up to six empty arrays per idle frame.

`research_d1.md` still specifies space-black `0x141518`, titanium `0x6e6d68`, a four-leaf iris, and four status dots. The implementation follows ring study 07's alabaster and gold instead, and implements neither the iris nor the dots. Pick one direction in the study before the next code pass. This audit's recommendation: keep the warm porcelain body from ring study 05, and apply Apple's two-solid proportion and satin lid. Do not return to a jewelry stack to get contrast.

---

## Next-iteration build sheet

Numbers an implementer can apply without re-deriving the audit. Lid datum `y = 0.110`. Bore `±1.080`. Outer ceramic `±2.150`.

| Feature | Value |
| :--- | :--- |
| Substrate | one solid, `y = -0.280..0.040`, corner `R0.10`, outer chamfer C0.20 |
| Lid | `outerHalf 2.09`, `y = 0.040..0.110`, corner `R0.080`, ceramic reveal `0.060` |
| Lid PBR | `color 0xffffff`, map `#d8dde6`, `roughness 1`, roughness-map mean `0.41`, `metalness 0.72`, anisotropy `0.55` |
| Ceramic PBR | `color 0xffffff`, map `#ebe5da`, `roughness 0.78`, `metalness 0.02`, matched bump repeat |
| Seam | recessed groove `0.004 × 0.012` at `y = -0.080`, ceramic-edge color, inside the sidewall |
| Aperture | 45° lid-metal chamfer, plan `1.140 → 1.080`, drop `0.020`, inner `R0.040` |
| Bore test | top-cap opening `1.080 ± 0.002` on substrate, lid, and bench |
| Pads | 32, one `InstancedMesh`, face recessed `0.002`, `y = -0.120`, static gold unless the phase walks one edge |
| Engraving | Inter 500, tracking `0.18em`, cap height `0.050`, centers `z = ±1.615`, tone-mapped standard material |
| NW/NE/SW/SE | four pockets, mark top `y = 0.109`, one instanced unit box, emissive on a slate base |
| North scale | `z = -1.96`, slice gaps, `count = totalSteps`, past emerald intensity `0.45`, future non-emissive |
| Iris | leaves at `y = 0.104` driven by `aperture`, or remove the input |
| Hot path | no `match` / `toUpperCase` / `\|\| []` per frame; `instanceColor` dynamic usage; `computeBoundingSphere()` after layout |

Order of work: material multiply fix, then delete the gold collar and gold bezel and cut the real chamfer, then recess pads and engrave tone-on-tone, then rebuild the four pockets and the slice hairline on `y = 0.110`. The silhouette has to be two solids before the lithography is worth tuning.

---

## Residual risk

This pass did not re-render the stage. The roughness product, the bevel ellipse, the bore inflation, and the `y = 0.1054` intersection are from the geometry and material code paths cited above. A screenshot would confirm the polished lid and the gold decal; it is not required to act on them. Frustum behavior is the current Three.js `Frustum.intersectsObject` path, which uses `InstancedMesh.boundingSphere` and computes it once. Designer 6 is safe only while `BAR_SPAN_X` stays `2.04`.
