# Grok 4.7 xhigh Agent 3 — Upward Ring Visibility & Continuation Stack Clearance

Auditor: Grok 4.7 (`xhigh`) Agent 3 of 5, Upward Ring Visibility & Continuation Stack Clearance.
Lens: `Inside Hollow Chip`, camera preset `interior` in `designer7_cinema.js` (`pos [0, -1.95, 0.52]`, `lookAt [0, -0.02, -0.05]`, fov 56°). The eye sits on the bore axis-offset at `r = 0.52`, `y = -1.95`, looking up at the split-arc crown.

Evidence:

- Screenshot `clean_v3_interior.png` (Inside Hollow Chip, step 53 / 145, `Push(frame) · 4 × (3 × [ Push(frame) ])`).
- `src/designers/designer3_frames.js` — annular continuation stack.
- `src/designers/designer2_well.js` — 12 Split-Arc Half-Rings `R1.A..R6.B` and the 10-sector Control stator.

---

## Verdict

Looking up fails for two stacked reasons, and both are in the geometry, not the camera preset.

1. The continuation wafers are a solid tunnel on `r = 0.64..0.96`. That band covers every radial millimeter of Rings `R1` and `R2`, and the outer 59% of `R3`, including `R3`'s light-pipe midline. From `y = -1.95` the nearest plates are a few centimeters above the lens, so the 56° frame is filled with opaque gold and teal metal. The open bore is a porthole, not a view of the crown.
2. What still shows through that porthole is the underside of the half-arcs. The body is non-emissive machined metal. The only underside light is a torus whose tube diameter is `0.024`. The live expressions (`RET:UInt`, `4 × [?]`, …) are not drawn on any downward face. The stator's `EVAL` / `PUSH` / `RET` words sit on a dial the size of the bore plug, behind a metal housing whose pads are biased to the top face.

Move the wafers out to the perimeter gallery (`inner 1.02`, `outer 1.28`) so the cylinder `r < 1.00` under all six rings is empty. Put a wide luminous soffit channel and a downward plaque on each of the 12 half-arcs and on the 10 stator sectors.

---

## 1. What `clean_v3_interior.png` actually shows

The frame is the interior preset: header `Push(frame) · 4 × (3 × [ Push(frame) ])`, footer camera chip `Inside Hollow Chip`, step 53 / 145.

Foreground, wrapping the left, right, and bottom of the picture, is a stack of thick smooth bands. They alternate luminous gold and emerald-teal. They are large because they are close. They are the continuation annuli, not the split-arc rings. Edge badges ride on those near bands (`R2.A RET fact(4) · n=4`, `R1.B 4 × [?]`, depth tags such as `D4 · env#2`). Those strings are continuation-stack copy. They are not the ring soffit.

Through the hole in the middle, six concentric arcs recede toward a small stator. The arcs read as dark grey and dull gold metal with hairline colored edges. One amber pad is lit on the stator. The sector words are not readable at this distance. The white disc in the center is the open bore / lens, not a label.

The rings the preset subtitle promises ("looking UP through the open bore at the 6 glowing Split-Arc Rings") are the small mechanism at the end of the tunnel. The tunnel is the thing you see.

---

## 2. Measured radii

Split-arc radii from `designer2_well.js`. `outerR = lerp(RING_OUTER_R_TOP, RING_OUTER_R_BOTTOM, r / 5)` with `0.95 → 0.31`. Arc width is `0.092`, except `R6` at `0.078`.

| Ring | `outerR` | `innerR` | light-pipe `midR` | world `baseY` |
| :--- | ---: | ---: | ---: | ---: |
| R1 | 0.950 | 0.858 | 0.904 | 0.060 |
| R2 | 0.822 | 0.730 | 0.776 | 0.006 |
| R3 | 0.694 | 0.602 | 0.648 | -0.048 |
| R4 | 0.566 | 0.474 | 0.520 | -0.102 |
| R5 | 0.438 | 0.346 | 0.392 | -0.156 |
| R6 | 0.310 | 0.232 | 0.271 | -0.210 |

Full crown span is `r = 0.232..0.950`. Control stator housing is `r = 0.138..0.220` at `y = -0.24`, open bore `r < 0.135`.

Continuation wafers from `designer3_frames.js` lines 56–59:

```text
WAFER_INNER_RADIUS    = 0.64   // anodized collar inner
WAFER_COLLAR_OUTER_R  = 0.68   // collar outer / glass inner
WAFER_RHODIUM_INNER_R = 0.91   // glass outer / rhodium inner
WAFER_OUTER_RADIUS    = 0.96   // rhodium outer
```

Collar and rhodium bevel are opaque `MeshStandardMaterial`. The sapphire web (`r = 0.68..0.91`) is `opacity: 0.36`, `depthWrite: false`. Seven plates sit above the interior eye (`topY = -0.64`, `spacing = 0.19`, eye at `y = -1.95`). Two glass hits transmit `0.64² ≈ 0.41`. Seven transmit `0.64⁷ ≈ 0.04`. The collar and the rhodium ring transmit nothing.

The file header still says the `0.64` bore "leaves a wide-open central upward sightline into the 6 Concentric Split-Arc Rings". The bore only clears `r < 0.64`. That is the stator, `R6`, `R5`, `R4`, and the inner lip of `R3`. It does not clear `R1` or `R2`.

Vault soffit (`designer2_well.js`, `innerR = 0.955`, `outerR = 1.40`, `y ≈ -0.34`) is the real ceiling aperture. Chamber walls step from `innerHalf = 1.32` at D0 to `1.22` at D7. The gallery the wafers should occupy is the ring between the crown (`r ≤ 0.95`) and those walls.

---

## 3. Cause A — the continuation tunnel occludes R1, R2, and R3

Radial overlap of the opaque/glass wafer band `r = 0.64..0.96` with each ring:

| Ring | Ring span | Covered by `0.64..0.96` | Light pipe |
| :--- | :--- | :--- | :--- |
| R1 | 0.858..0.950 | 100% | `midR 0.904` buried in the glass web |
| R2 | 0.730..0.822 | 100% | `midR 0.776` buried in the glass web |
| R3 | 0.602..0.694 | outer 0.054 of 0.092 = 59% | `midR 0.648` sits on the inner collar |
| R4 | 0.474..0.566 | none | visible only down the porthole |
| R5 | 0.346..0.438 | none | same |
| R6 | 0.232..0.310 | none | same |
| Stator | 0.138..0.220 | none | a small disc at the end of the tunnel |

`R1`–`R3` together span `r = 0.602..0.950`. The screenshot's solid yellow/teal foreground is that overlap, plus the bevel. The envelope people describe as `r ≈ 0.63..0.99` is this band seen with the collar thickness and the vault lip at `0.955`. Nothing in the crown actually reaches `0.99`. `R1` ends at `0.950`. The fuel ring is a separate loop at `r = 1.03`, `y = 0.095`.

Why the near bands eat the frame: the eye is at `r = 0.52`, only `0.12` inside the bore wall. The first wafer above the lens is ~`0.17` up (`y ≈ -1.78`). A 56° cone aimed mostly upward therefore intersects the annulus immediately. Rays toward the south (near) rim of `R1` cross the wafer planes at `r ≈ 0.77`, inside the metal. Rays toward the north rim can thread the hole, which is why a crescent of rings survives in the middle of the picture and why it looks small.

Clearing `r < 0.64` was the right move relative to the old solid slabs. It is the wrong radius for a crown whose outer three rings live at `r ≥ 0.60`.

---

## 4. Cause B — the underside is dark metal, a 0.024 wire, and no expression

Each half-arc (`SplitArc_R1A` … `SplitArc_R6B`) is built in `designer2_well.js` around lines 1504–1641.

The body is `createChamferedArcGeometry` with `thickness: 0.028`, rotated into XZ and translated so the top cap is near local `y = 0` and the bottom cap is near local `y = -0.028`. Material is `halfArcOuterMatEven` (`color 0xb4b8be`, metalness `0.90`, no emissive) or `halfArcOuterMatOdd` (`color 0xc8a45d`, metalness `0.90`, no emissive). From below, that cap is grey silver or dull champagne. In the dark chamber it reads as the dark metal in the screenshot. (`ringMat` at line 1976 is a different mesh — the terrace light loops, `color 0x1e293b`, `emissiveIntensity 0.03`. Same visual family: dark metal, almost no glow.)

Underside light, the entire glow you get when you look up:

- `TorusGeometry(midR, 0.012, 10, 36, 0.88π)` shared by the upper pipe (`y = +0.006`) and `lowerLightPipeMesh` (`y = -0.025`). Tube radius `0.012` means the luminous strip is `0.024` wide on an arc that is `0.092` wide (`0.078` on `R6`). About a quarter of the ring lights up, and only as a wire, not as a face.
- The dark-sapphire channel that could have been that face is `channelWidth = max(0.024, arcWidth * 0.36)` ≈ `0.033`, material `0x0f172a` with no emissive, and it is placed only at `y = +0.0048`. There is no lower channel mesh.
- `lowerDialMesh` at `y = -0.029` reuses the tick texture. The paint is `rgba(20, 23, 31, …)` — dark ink. It does not carry the expression, and it disappears on the metal.

Expressions are not on the soffit at all. `formatHalfArcPlaqueCopy` still knows how to say `R1A · RET:UInt (root)` and `R1B · 4 × [?] (n=4)` (comment at line 820; title built at line 849). The update loop writes that string into `userData.title` only. The mesh that used to show it is gone:

```1599:1606:src/designers/designer2_well.js
      // Keep plaqueMat stub (no floating rectangular sticker mesh on the rings!)
      const plaqueMat = trackMat(
        new THREE.MeshBasicMaterial({
          transparent: true,
          opacity: 0.0,
          depthWrite: false,
        })
      );
```

`plaqueMat` has no map and no `Mesh`. `plaqueMat.opacity` is animated (lines 2403–2417) and nothing draws it. A top-face plaque at local `y = +0.0265` facing `+Y` is the historical slot this stub replaced: `+0.0265` is just above the top cap, and `rotation.x = -π/2` is how every other upward plaque in this scene is aimed. That face points at the sky. The interior camera never sees it. Deleting the mesh made the upward view worse, not better. Top-down bleed was a real bug. The fix is a downward face, not silence.

The stator has the same bias. Sector pads are extruded `thickness: 0.028` and then parked at local `y = 0.003` (idle) or `0.010` (active), so the color sits on the crown. The housing is metal, `thickness: 0.022`. `lowerStatorDialMesh` at `y = -0.0255` does carry `EVAL`, `PUSH`, `RETURN`, `INVOKE`, `ENTER`, `BOUND`, `CASES`, `MATCH`, `PRIM`, `ERROR` in `createControlStatorDialTexture`, `DoubleSide`. It is a ring of radial width `0.074` (`0.142..0.216`) seen through the tunnel, at a world diameter of about `0.43`. At the interior eye that disc is ~14° across. The words do not survive. There is no per-sector downward plaque, and `lensPlaqueMat` is the same empty stub as the ring plaques.

---

## 5. Fix in `designer3_frames.js` — park the stack on the gallery wall

Open the whole cylinder under the crown. Set:

```text
WAFER_INNER_RADIUS    = 1.02
WAFER_COLLAR_OUTER_R  = 1.06   // collar stays 0.04 wide
WAFER_RHODIUM_INNER_R = 1.23   // glass web 1.06..1.23
WAFER_OUTER_RADIUS    = 1.28
BADGE_RADIUS          = 1.145  // mid-glass, was 0.79
```

`r = 0.00..1.00` is then empty. Measured clearance to the crown:

- `R1` outer rim `0.950` to wafer inner collar `1.02` is `0.070` of air.
- `R2`..`R6` and the stator sit further inside that cylinder. Nothing in `0.232..0.950` can be hidden by a wafer again.
- Interior eye at `r = 0.52` gains `0.50` of radial air to the nearest metal (`1.02 - 0.52`) instead of `0.12`. The 56° upward cone hits open volume, then the ring soffits, then the stator. The gold/teal bands move out to the peripheral vision, where a gallery belongs.

Keep the existing three-band construction (emissive collar, sapphire web, rhodium bevel), the `0.19` vertical spacing, the `y = -0.64` top, and the West/East badge stagger. Only the radii change. Update the inspect strings and the header comment that still advertise `innerRadius = 0.64` / `0.36`. Both are wrong after this move, and the `0.64` comment is what hid this bug.

Wall fit, which the constants have to respect:

- D0 `innerHalf = 1.32` clears outer `1.28` by `0.04` on the cardinal faces.
- `innerHalf` falls by `0.10 / 7` per terrace. It drops through `1.28` at D3 (`innerHalf ≈ 1.277`). D7 is `1.22`.
- The eight vault ribs sit near `r ≈ 1.32` and still clear `1.28`. The square walls do not, on the lower terraces.
- Soffit is at `y ≈ -0.34` with a hole of `r = 0.955`. Wafers start at `y = -0.64`, so they hang in the gallery below the soffit rather than inside the hole. That is what we want.

Companion, not a different design: clamp each slot's outer radius to the terrace it occupies, `outerR = min(1.28, innerHalfAt(slot.y) - 0.04)`, and keep `innerR` pinned at `1.02`. Deep slots get a narrower web. They never cross `r = 1.02`, and they never enter the wall. If a constant outer radius is preferred for shared geometry, use `1.20` (clears D7's `1.22` by `0.02`) and keep `WAFER_INNER_RADIUS = 1.02`. Shared geometry at a hard `1.28` will interpenetrate D3–D7.

Also retarget the orb copy. It currently says the value rises "through the `r = 0.64` open-bore". After the move the shaft is `r < 1.02`. The orb path itself (`x = 0`, `z = 0`, up to `CONTROL_CORE_DOCK_Y = -0.285`) does not need a new radius.

---

## 6. Fix in `designer2_well.js` — soffit color and downward plaques

### 6.1 Wider underside sapphire channel on all 12 half-arcs

Add a lower channel next to the existing top channel (do not steal the top-down look).

- Radial width: `undersideChannelWidth = arcWidth * 0.72` → `0.066` on `R1..R5`, `0.056` on `R6`. That is most of the arc, with a metal bevel left on both rims. Replace the `0.024` wire as the thing you see from below.
- Geometry: `RingGeometry` arc, same `thetaStart + 0.018` and `thetaLength - 0.036` as the top channel, at local `y = -0.030` (just under the bottom cap).
- Material: one `MeshStandardMaterial` per half-arc, `emissive` driven by the same `semanticHex` already written into `lightPipeMat` (occupied / pop afterglow / idle slate). `roughness ≈ 0.18`, `metalness ≈ 0.15`, `side: FrontSide` is enough if the ring faces `-Y`. Bump occupied `emissiveIntensity` to the same band as the pipe (`1.45` top of stack, `0.85` otherwise) so the soffit reads as a colored band, not a filament.
- Keep `lowerLightPipeMesh` as a bright core inside that band, or drop the lower torus once the band exists. Two glows on top of each other will blow out the crown. One face-on band is the readable object.
- Do not put this emissive on `halfArcOuterMatEven/Odd`. Those materials are shared across six half-arcs. Per-slot color has to live on a per-slot material, which `lightPipeMat` already is. Point the new channel at `lightPipeMat`, or clone that pattern.

### 6.2 `lowerPlaqueMesh` on `R1.A..R6.B`

Build one downward plaque per half-arc, in the same loop that creates `SplitArc_${slotCode}`.

- Geometry: `PlaneGeometry(0.34, 0.078)`. Large enough to read `R1.A  RET:UInt` from `y = -1.95`, small enough to sit on the arc (`arcWidth 0.092` is the radial budget; the plaque extends along the tangent, so `0.34` of arc length at `midR ≈ 0.90` is about 22°).
- Placement: center of the half-arc. East slot `θ = 0`, west slot `θ = π`. Position `(cos θ · midR, -0.040, -sin θ · midR)` in the half-arc group, using the same angle convention as the clasp placement (`cos` on X, `-sin` on Z). Local `y = -0.040` clears the tube, which bottoms near `-0.037`.
- Aim the normal at `-Y`: `rotation.x = +π/2`, then `rotation.y = θ` so the text's horizontal axis follows the tangent and the top of the text points outward. `MeshBasicMaterial`, `transparent: true`, `depthWrite: false`, `side: FrontSide`. FrontSide plus a `-Y` normal means the top-down preset does not see these plaques through the bridge gaps. That is the bleed the stub was trying to kill.
- Texture: `createPlaqueTexture`, the same helper the wafers use. Copy from `formatHalfArcPlaqueCopy`, with the slot id written `R1.A` / `R1.B` (insert the dot). Title line is the expression: `RET:UInt`, `4 × [?]`, `IF [?]`, `SCOPE`. Subtitle line is family and env: `ENV · root`, `OP · n=4`. Accent is `getFrameSemanticCss(frame)`. Idle slots get `R1.A · IDLE` at low opacity.
- Hook it where `plaqueKey` already changes (line 2428). Today that branch only sets `userData`. Also assign the texture to `lowerPlaqueMat.map` and set `needsUpdate`. Drive `lowerPlaqueMat.opacity` from the values already computed for `plaqueMat` (`1.0` / `0.92` occupied, pop fade, `0.46` idle). Hide at opacity `< 0.02`.
- Leave the top face free of a second sticker. Top-down already has the dial ticks and the light pipe. The new information is exclusively for the upward view.

Expected readable set, looking up from inside, for a typical `fact` stack: `R1.A RET:UInt`, `R1.B 4 × [?]`, and the matching `R2.A`..`R6.B` lines in teal, gold, amber, or violet. Family color comes from the channel. The words come from the plaque.

### 6.3 Same treatment on the 10-sector Control stator

The stator is what hangs directly above the eye once the tunnel is gone. It needs the same two signals: full-width color, and the word.

- Underside luminous band: a second `InstancedMesh` of the 10 sector pads, or a `RingGeometry` sector matching `sectorPadSpan`, at local `y = -0.028` (below the housing bottom at `-0.022` and the lower collet at `-0.022`). Radial span `STATOR_INNER_R..STATOR_OUTER_R` (`0.142..0.216`), not a `0.008` collet. Reuse `sectorPadMat` / `setColorAt` so the active sector (`EVAL`…`ERROR`) pulses on the soffit with the same hex already applied at line 2504. Idle sectors stay `0x1e293b` but the band is still a surface, not a buried extrusion.
- Ten downward plaques, one per `CONTROL_STATOR_SECTORS` entry. `PlaneGeometry(0.055, 0.022)` at `STATOR_MID_R` (`0.179`), local `y = -0.034`, `rotation.x = +π/2`, yawed to the sector center `θ = π/2 - i · sectorStep`. Text is the short badge already in the table: `EVAL`, `PUSH`, `RET`, `INVOKE`, `ENTER`, `BOUND`, `CASES`, `MATCH`, `PRIM`, `ERR`. Active plaque opacity `1`, idle `0.72`, accent from `sector.css`. These are static strings; build the textures once, do not allocate them in `update`.
- Keep `lowerStatorDialMesh` if the tick ring still helps. The plaques are what make the words survive from `y = -1.95`. The dial texture alone does not, because the whole stator is only `r ≤ 0.216`.

---

## 7. Acceptance

A later Inside Hollow Chip frame, same step family as `clean_v3_interior.png`, should show:

- No gold or teal annulus crossing `r < 1.00`. The continuation colors, if visible, sit out by the gunmetal walls.
- All six rings visible at once from below, each mostly filled with its semantic color (teal, cyan, amber, gold, indigo, violet) rather than a grey cap and a hairline.
- Downward plaques reading `R1.A RET:UInt`, `R1.B 4 × [?]`, and the other ten slots, right-side up when looking up.
- Stator words `EVAL`, `PUSH`, `RET`, `INVOKE`, `ENTER`, `BOUND`, `CASES`, `MATCH`, `PRIM`, `ERR` on the underside, with the active sector brighter.
- Top-down Rings unchanged: no new stickers showing through the north/south bridge gaps.
- Deepest wafers not stuck through the D3–D7 walls.
