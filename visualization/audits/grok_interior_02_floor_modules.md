# Grok 4.7 Agent 2 — Interior Floor Modules & 39-Rule Decoder

Auditor: Interior Chip Floor Modules & 39-Rule Decoder.
Scope: why the chamber floor at `WELL_FLOOR_Y = -3.38` cannot be seen, and how to replace the flush decoder decal in `src/designers/designer2_well.js` with 39 physical micro-silicon rule modules.

Evidence opened:

- `scratch/clean_v3_interior.png` — Inside Hollow Chip, step 53, looking up the bore.
- `scratch/grok_test_01_interior_default.png` — same preset, closer crop, still the bore.
- `scratch/grok_test_02_rotate_left.png` — yaw inside the preset. Still the bore. Floor still absent.
- `scratch/grok_test_03_drag_up_to_see_floor.png` — after the gesture meant to reveal the floor, the camera is outside, above the package, looking down at the rings.
- `scratch/grok_test_04_drag_down.png` — camera has left the chamber and is facing the outer package wall and the gray studio void.
- `scratch/grok_test_05_wheel_zoom_out.png` — cutaway from outside. The deep floor is not in frame.
- `scratch/view_frames.png` — older Cross-Section crop of the package skin and the side panels. No chamber floor.

`scratch/grok_test_06_cross_section.png` is not in that directory. The Cross-Section diagnosis below uses the live `frames` preset in `designer7_cinema.js` and the two exterior shots that do exist (`grok_test_05`, `view_frames.png`).

Code cited:

- `src/designers/designer7_cinema.js` — `CAMERA_PRESETS.interior`, spherical drag, keep-out exemption.
- `src/designers/designer2_well.js` — `WELL_FLOOR_Y`, `createStatic39RuleFloorOverlayTexture`, `FlushInstanced39RuleFloor`, the per-step tile update.
- `src/stage/SherlockStage.js` — cutaway coupling and `vaultFloorLight`.

---

## 1. What the shots actually show

`clean_v3_interior.png` and `grok_test_01_interior_default.png` are the same pose. The eye is inside the vault, pitched steeply up. The frame is the six split-arc rings, the 10-sector stator, the open sapphire bore, and the ring plaques (`R2·B`, `fact([?])`, `RET entry`). The bottom of the frame is the near lip of the lowest ring and the south bridge gap. There is no floor, no rule id, no `A080`…`A225` label, and no chiplet.

`grok_test_02_rotate_left.png` only changes yaw. The horizon of the shot is still the ring stack. Yaw at this polar angle stays inside the vault, which is why this one shot does not fall out of the chip. It also cannot discover the floor. The floor is behind the camera.

`grok_test_03_drag_up_to_see_floor.png` is the failure the user described. The chip interior is gone. The camera is a high exterior angle on the closed ring stack: fuel ring, gold and teal half-arcs, stator, one amber sector. The 39-rule floor is on the far side of the package and is not in the frustum.

`grok_test_04_drag_down.png` is the same orbit at the equator. The frame is a black package sidewall, a sliver of rings over the top edge, and empty gray void. The user has left the chip.

`grok_test_05_wheel_zoom_out.png` confirms the cross-section problem from outside the south cutaway. The eye is at lid height looking across the ring stack. The floor, three units below, is under the lower edge of the frame.

---

## 2. Diagnosis — the default interior gaze hides `y = -3.38`

`CAMERA_PRESETS.interior` in `designer7_cinema.js`:

```js
pos: [0.0, -1.95, 0.52],
lookAt: [0.0, -0.02, -0.05],
fov: 56,
```

The label on that preset is `Inside Hollow Chip (Looking Up at Rings)`. The offset from look-at to camera is `(0, -1.93, 0.57)`.

| Quantity | Value |
| --- | --- |
| Orbit radius | `2.012` |
| Polar angle `phi` (from +Y) | `2.854 rad` (`163.6°`) |
| Azimuth `theta` | `0` (camera sits on +Z, the open south side) |
| Gaze elevation | `+73.5°` above horizontal |
| Eye height | `y = -1.95` |
| Floor | `y = -3.38` (`1.43` below the eye) |
| Angle between the gaze axis and the floor center `(0, -3.38, 0)` | `143.6°` |
| Half-fov at 56° | `28°` |

The floor center is about `116°` outside the view cone. The bottom of the frustum is still `45°` above the horizon, so the entire chamber below the eye, including every terrace under the camera and the whole decoder, is behind the lens. “When I look up I don’t get to see much” is this pose doing what it was written to do: it spends the whole frame on the underside of the rings. The modules are not small in that shot. They are outside it.

The preset was deepened on purpose (`designer2_well.js` comment above `WELL_FLOOR_Y`: “deepened so the interior camera clears the rings and looks up at them”). The camera was then parked at `y = -1.95` specifically so the rings would clear the near plane. Nothing in that preset aims the lens at the floor it was deepened to expose.

`frames` does not rescue it. Preset `pos [1.85, -0.18, 4.45]`, `lookAt [-0.06, -1.52, 0]`, fov `38`. The floor center is `18.1°` off that look axis and `5.78` units away, against a `19°` half-angle, so it sits on the bottom pixel row of the cross-section. A tile `0.008` tall at that distance subtends about `0.08°`. The cross-section subtitle promises the 39-rule floor. The lens does not resolve it.

---

## 3. Diagnosis — drag-to-look-down orbits out through the roof

Pointer motion is a third-person spherical orbit around `desiredTarget`, not a tilt of the gaze.

```js
desiredTheta -= dx * 0.0068;
desiredPhi = clamp(desiredPhi - dy * 0.0058, MIN_PHI, maxPhi);
```

`dy` is the change in `clientY`. Pointer moving down makes `dy > 0`, which decreases `phi`. In Three.js, `phi ≈ 0` puts the camera above the target (looking down) and `phi ≈ π` puts it below the target (looking up). The interior pose starts at `phi = 2.854`, already under a target that sits on the ring plane (`lookAt.y = -0.02`), on a sphere of radius `2.012`.

Decreasing `phi` walks the eye up that sphere:

| `phi` | Camera `y` | Horizontal radius | Where the eye is |
| --- | --- | --- | --- |
| `163.6°` (preset) | `-1.95` | `0.57` | Inside the vault, looking up |
| `142.7°` | `-1.62` | `1.22` | Crosses the terrace inner half. South wall is fully retracted |
| `99.4°` | `-0.35` | `1.99` | Crosses the vault soffit (`y = -0.35`, hole radius `0.955`) while already outside `OUTER_HALF = 1.44` |
| `90°` | `-0.02` | `2.01` | Outside, beside the lid. This is the station in `grok_test_04_drag_down.png` |
| `~26°` | `+1.79` | `0.88` | Above the package, looking down at the rings. This is `grok_test_03_drag_up_to_see_floor.png` |

The gesture that tries to bring the floor into view is the gesture that raises the camera. From the preset, `phi` only has to fall by `0.364 rad` to cross the inner wall. At `0.0058 rad/px` that is **63 pixels**. `getMaxPhiForState` then allows the rest of the trip: for `interior` it returns `π * 0.96`, and the lower clamp is `MIN_PHI = 0.002`, so the orbit is free to finish above the chip.

Two further locks keep the user out:

1. The keep-in at `designer7_cinema.js` (the block that scales `horizR` back to `0.86` and clamps `camera.position.y` to `-3.22`) is skipped whenever `currentPresetName` is `interior` or `frames`. Those are the only two presets that enter the chamber.
2. That keep-in writes `camera.position` after `setFromSphericalCoords` and does not write `currentRadius` or `currentPhi`. The next frame rebuilds the pose from the sphere, so a position patch cannot stick.

Yaw is the safe axis. At `phi = 163.6°`, `sin(phi) * radius ≈ 0.57`, which is inside the vault, so `grok_test_02_rotate_left.png` stays indoors. Pitch is the axis that leaves. The user requirement “I should not leave the chip when I rotate the camera” is unmet for any vertical drag.

Wheel-zoom is a radius change on the same sphere (`onWheel`), with a dive helper that only runs when `hypot(target.x, target.z) < 0.65` and the user scrolls in. Scrolling out, which is `grok_test_05`, multiplies radius upward from `2.012` and climbs out of the south cutaway. `SherlockStage` forces `cutawayMix = 1` for both `interior` and `frames`, so the south wall is gone for the whole visit and the sphere’s +Z side opens straight onto the gray void.

---

## 4. Diagnosis — the 39 rules are a decal over flush boxes

`WELL_FLOOR_Y = -3.38`, `WELL_BASE_Y = -3.40`. Section 5 of `createWellSubsystem` builds `FlushInstanced39RuleFloor`. There are two draws, and the one the camera would read is flat.

**Ceramic slab.** `BoxGeometry(2.88, 0.024, 2.88)` at `y = -3.39`. A plinth, not a module.

**Tile instances.** One `InstancedMesh` of `BoxGeometry(0.322, 0.008, 0.272)`, 39 copies, material roughness `0.42`, metalness `0.35`, idle color `0x161d29`. Each instance is placed at `y = WELL_FLOOR_Y + 0.004` (`-3.376`). The box is 8 mm tall on a cell `322 × 272 mm`. No bevel, no frame, no status bar, no emissive.

The per-step update (`ruleTilesInstancedMesh` loop) does not change that read:

- Active: `scale (1.02, 1.35, 1.02)`, center lifted to `WELL_FLOOR_Y + 0.006`. Raised height is `0.0108`. Top of the active tile is `y = -3.3686`.
- Trail: center at `+0.0045`, scale `1`.
- Idle: center at `+0.004`, color `0x161d29`.

`setColorAt` tints diffuse color. The material emissive is left at black, so an “active” tile is a slightly brighter pancake, not a lamp. `instanceColor` on this WebGPU material does not drive emissive.

**Label decal.** `floorOverlayMesh` is a single `PlaneGeometry(2.36, 2.36)` with `createStatic39RuleFloorOverlayTexture` (2048² canvas), `MeshBasicMaterial`, `transparent: true`, `opacity: 0.94`, `depthWrite: false`, `rotation.x = -π/2`, placed at `WELL_FLOOR_Y + 0.014` (`-3.366`).

That plane is `2.6 mm` above the top of the raised active tile, which is what the comment asks for (“elevated active tiles never cover their labels”). The rule id, the `0F`/`1F` mark, the short name, the line number, the hairline bezel, and the `SHERLOCK · K-A/1 · 39-RULE ROM` bar exist only as pixels on this unlit sheet. `MeshBasicMaterial` ignores `vaultFloorLight` (`SherlockStage.js`, point light at `(0, -2.85, 0.25)`). From a grazing angle the sheet has no silhouette. `depthWrite: false` lets it ghost through anything that does rise.

The inspect record is one object for the whole floor (`type: 'k-decoder-bus-hub'`), and its title still says `y = -2.18`. The overlay comment still says the plane is at `-2.166`. Both numbers are the pre-deepening floor. `findInspectableTarget` walks up to `floorGroup` and drops `intersection.instanceId`, so a click cannot name `A135` even after the modules become solid.

Grid, which the new modules should keep (source order is the ROM order; do not reflow):

```js
cols: 6, rows: 7,
startX: -0.925, startZ: -0.96,
stepX: 0.37, stepZ: 0.32,
cellW: 0.322, cellD: 0.272,
floorSpan: 2.36
```

Index `i` is column `i % 6`, row `floor(i / 6)`, `x = -0.925 + col * 0.37`, `z = -0.96 + row * 0.32`. Indices `0..38` are `SHERLOCK_K_RULES`. Row 6 columns 3–5 have no mesh. The ROM signature is painted across that hole by the decal, centered on column 4 of row 6.

Category spans, for the engraved aisle ticks only (do not reorder the instances):

| Indices | Count | Category | Ids |
| --- | --- | --- | --- |
| 0–2 | 3 | terminal, charge 0 | A080 A082 A084 |
| 3–12 | 10 | dispatch | A102–A131 |
| 13–14 | 2 | stack | A135 A140 |
| 15–24 | 10 | continuation | A145–A181 |
| 25–32 | 8 | function | A185–A208 |
| 33–38 | 7 | pattern / primitives | A212–A225 |

---

## 5. Specification — 39 physical micro-silicon rule modules

Build these in `createWellSubsystem`, in place of `ruleTilesInstancedMesh` + `floorOverlayMesh`. Keep `floorBaseMesh`, `ruleCells`, `ruleCellsByLine`, `ruleCellsById`, and the zero-allocation scratch objects (`_scratchMat4`, `_scratchPos`, `_scratchQuat`, `_scratchScale`, `_scratchColor`, `_scratchColorB`). The renderer is `WebGPURenderer` (`three@0.175`, import from `three/webgpu`). `onBeforeCompile` will not run. Per-instance label UVs have to go through a node material.

Four instanced draws plus one plaque. That replaces the current two draws.

### 5.1 Shared chiplet body

Add `createRuleChipletGeometry()` next to `createChamferedArcGeometry`. Same trick that function already uses: `ExtrudeGeometry`, then `rotateX(-π/2)` so the lid faces +Y.

- Rounded-rect `Shape` centered on the origin, `0.286 × 0.236`, corner radius `0.012`.
- Extrude `depth: 0.040`, `bevelEnabled: true`, `bevelThickness: 0.004`, `bevelSize: 0.005`, `bevelSegments: 2`, `curveSegments: 4`.
- After the rotate, translate so the underside sits at local `y = 0`. Instance position is then the contact point on the slab.

`MeshStandardMaterial` (this alias is `MeshStandardNodeMaterial` under the WebGPU build): color `0x1a2333`, roughness `0.62`, metalness `0.22`, `polygonOffset` on, `polygonOffsetFactor: 1`. Idle `setColorAt` stays `0x1a2333`. Active `setColorAt` lerps that silicon toward `rule.color` by `0.55`, multiplied by the existing pulse `0.85 + 0.3 * sin(ambient * 6)`.

Instance translation: `(cell.x, WELL_FLOOR_Y + 0.010, cell.z)`. Lid top at about `-3.326` when idle.

### 5.2 Gold bezel, static in the socket

Second `InstancedMesh`, count 39. `Shape` outer `0.308 × 0.258` with a hole `0.272 × 0.222`, corner radius `0.014`. Extrude depth `0.010`, bevel `0.002`. Underside at local `y = 0`.

Material: `MeshPhysicalMaterial` matching the stator collets — color `PALETTE.gold`, roughness `0.22`, metalness `0.92`, clearcoat `0.32`. Reuse `brushedSilverTex` only if it stays subtle at this size; a flat gold physical material is enough.

Place at `(cell.x, WELL_FLOOR_Y + 0.008, cell.z)`. Do not scale it in the update loop. The bezel is the socket. The silicon rises out of it. That gap is what reads as a module from across the floor.

### 5.3 Active-rule beacon

Third `InstancedMesh`, count 39. `BoxGeometry(0.20, 0.008, 0.012)`. Unlit on purpose: `MeshBasicMaterial` with `color: 0xffffff`, `toneMapped: false`. `setColorAt` is then the lamp itself.

- Idle and cold trail: color `0x141820`. The bar is a dark slit in the south lip of the lid.
- Trail (`cell.trail > 0.04`): color = silicon lerped toward `rule.color` by `cell.trail * 0.65`.
- Active: color = `rule.color` at full strength, times the pulse. No multiply that pulls it back toward gray.

Seat the bar on the south lip, local offset `(0, 0.046, 0.092)` added in the instance matrix, so it sits on the lid and faces the south gallery camera in section 6. Parent the offset into the composed matrix. Do not add a child mesh per instance.

Fourth `InstancedMesh`, the beacon post, count 39. `BoxGeometry(0.012, 0.070, 0.012)`, same unlit material, same color rules. Idle scale `y = 0.02` (a 1.4 mm nub, visible as a contact but not a column). Active scale `y = 1` and a `0.016` lift with the body. A 70 mm post on a 40 mm chiplet is readable from the far row and from the cross-section without becoming the old vertical beam the flush floor was written to avoid. Hide nothing with `visible = false` per instance (InstancedMesh cannot). Scale is the gate.

Active body motion, in the same loop that already writes `ruleCells`:

- Active: body `y = WELL_FLOOR_Y + 0.026` (lifts `16 mm` out of a bezel that stays at `+0.008`). Scale stays `(1, 1, 1)`. The current `y` scale of `1.35` on an 8 mm box is a 3 mm change and should go away.
- Trail: body `y = WELL_FLOOR_Y + 0.014`.
- Idle: body `y = WELL_FLOOR_Y + 0.010`.

Write body, bar, and post matrices in that one loop. No new `Vector3` or `Matrix4` allocations. The bezel matrix is written once at build and left alone.

### 5.4 Crisp labels on the lid

Stop drawing the full-floor plane. Keep `createStatic39RuleFloorOverlayTexture` as the atlas, with two changes:

- Paint an opaque silicon rectangle behind each cell (`#121820`) so the label quad does not need a transparent full-floor sheet.
- Move the ROM signature out of this atlas. It becomes the plaque in section 5.5. Leaving it in the atlas paints three empty cells with text that would then have no module under it.

Fifth draw: `ruleLabelMesh`, `InstancedMesh` of `PlaneGeometry(0.262, 0.168)`, count 39. Put an `InstancedBufferAttribute` `uvOffset` (`vec2`) and `uvScale` (`vec2`) on that geometry.

For cell `(cellX, cellZ)`, using the same span math as `toPxX` / `toPxY` (`halfSpan = 1.18`, `span = 2.36`):

```js
uvScale = (0.262 / span, 0.168 / span)
uvOffset = ((cellX - 0.131 + halfSpan) / span,
            (cellZ - 0.084 + halfSpan) / span)
```

Material, WebGPU node path (import `attribute`, `uv`, `texture` from `three/tsl`):

```js
const labelMat = new THREE.MeshStandardMaterial({
  roughness: 0.72,
  metalness: 0.04,
});
labelMat.colorNode = texture(
  floorOverlayTex,
  uv().mul(attribute('uvOffset', 'vec2')).add(attribute('uvScale', 'vec2'))
);
```

Confirm the attribute order against a single cell before wiring all 39. `PlaneGeometry` rotated `rotation.x = -π/2` maps local +Y to world −Z, and `CanvasTexture` flips Y. If `A080` (column 0, row 0, the north-west cell) comes out mirrored, replace the V offset with `1 - (v0 + vScale)` and keep U as written. Do not flip the whole atlas and also swap V.

Each label instance uses the body’s quaternion and the body’s Y, plus `0.002`, so the text lifts with the active chiplet and clears the lid. Fixed tilt, not a billboard: pitch the plane `-0.42 rad` about X (top edge toward −Z / north, face toward the south gallery). A billboard would read as a sticker and would hide the gold bezel.

`shortName.slice(0, 11)` already truncates `PushFrame`, `RestoreScope`, `FinishArgs`. On a `0.262` lid viewed from `1.2` away at fov 50 that truncation is the right budget. Do not add a second line of prose to the lid. The full name belongs in the inspect card (section 5.6).

Set `depthWrite: true` on the label material. The current `depthWrite: false` is what lets the decal swim over the tiles.

### 5.5 ROM plaque

One mesh, not an instance, for the hole at row 6, columns 3–5. Center is the point the atlas already uses: `hubX = startX + 4 * stepX` (`0.555`), `hubZ = startZ + 6 * stepZ` (`0.96`). Size `0.70 × 0.018 × 0.22`, gold bezel of the same physical material, lid texture a small canvas that says `SHERLOCK · K-A/1 · 39-RULE ROM` in `#d4af37` on `#121820`. Tilt it by the same `-0.42 rad` so it belongs to the module field. `tagInspectable` on this mesh alone, with the hub copy that is currently on `floorGroup`.

### 5.6 One module, one inspect hit

`findInspectableTarget` returns the first ancestor with `userData.inspectable`, which today is the whole floor. When `intersection.instanceId` is an integer and the hit object is one of the four rule meshes, resolve `SHERLOCK_K_RULES[instanceId]` before walking up.

Store the lookup on the mesh at build time (`userData.ruleRecords = ruleCells`) so `designer7_cinema.js` does not import the rule table. Return a small object whose `userData` is:

- `type: 'k-rule-module'`
- `id: rule.id` (`A135`)
- `title: '${rule.id} · L${rule.line} ${rule.shortName}'`
- `subtitle: rule.fullName`
- `details: [rule.categoryLabel, 'Charge ${rule.charge}F', rule.description]`

The floor-group inspect record can stay as the fallback when the ray hits the ceramic slab.

### 5.7 What this looks like from inside

From the south gallery pose in section 6, each module is a dark silicon rounded block standing proud of a gold socket, with `A135` / `PushFrame` / `L135` on a tilted lid and a dark slit on the near edge. The active rule is the block that has risen out of its socket, the slit at full `rule.color`, and a 70 mm post of that same color. The previous rule keeps a dimmer slit and a half-lift for the trail window that already exists (`cell.trail`). Error rules (`A082`, `A111`, `A140`, `A169`, `A190`, `A202`, `A206`, `A212`, `A225`) use the red they already carry in `rule.color`. No new palette.

`vaultFloorLight` at `y = -2.85` will catch the bevels and the gold sockets because those materials are physical. The unlit beacon stays legible when that light is tinted teal or crimson by the phase logic. Do not put the labels on `MeshBasicMaterial`; the silicon lid should take the vault light, and the beacon is the only unlit element.

---

## 6. Camera changes required for those modules to be visible

Building the modules without changing the interior orbit leaves them at `143°` outside the frustum, and the first vertical drag still exits through the roof. The pose and the drag model are part of the floor work.

### 6.1 Replace the interior preset with a south-gallery stance

```js
interior: {
  name: 'interior',
  label: 'Inside Hollow Chip',
  subtitle: 'Standing in the south gallery of the vault. Pitch down across the 39 rule modules, pitch up through the bore to the split-arc rings.',
  pos: [0.0, -2.68, 1.12],
  lookAt: [0.0, -3.22, 0.20],
  fov: 50,
}
```

Eye is `0.70` above the floor and at horizontal radius `1.12`, inside the terrace inner half (`1.22`) and under the soffit rather than in the bore. Gaze is `29°` below the horizon, across the grid from the south row (`z ≈ 0.96`) toward the north row (`z = -0.96`). Near modules subtend roughly `15°`; the far row still subtends roughly `8°`, enough for an `A080` id on the atlas above. Pitching up toward the stator `(0, -0.24, 0)` is a `+65°` elevation, which stays inside the chamber and looks through the soffit hole (`innerR = 0.955`) instead of requiring the camera to relocate.

Drop the words `Looking Up at Rings` from the button label. The rings remain one pitch away. They are no longer the only thing the preset can show.

### 6.2 First-person pitch and yaw while `interior` is current

In `onPointerMove`, when `currentPresetName === 'interior'` and the drag is not a pan:

- Change yaw by `-dx * 0.005`.
- Change pitch by `-dy * 0.004`, then clamp pitch to `[-1.15, 1.25]` radians (about `-66°` to `+72°`). Pointer down looks down at the floor. Pointer up looks up the bore.
- Do not modify `desiredPhi`, `desiredTheta`, or `desiredRadius` on this path.

In `update()`, while the preset is `interior`, skip `setFromSphericalCoords`. Write `camera.position` from the gallery position and `camera.lookAt` from `position + forward(yaw, pitch)`. When the preset changes away from `interior`, seed `currentRadius`, `currentPhi`, `currentTheta`, and `desired*` from the camera and the current target once, so the exterior orbit does not snap back along the old radius-`2.012` sphere.

Wheel while interior dollies along the horizontal forward vector, then clamps into the keep-in box. It does not multiply a spherical radius.

### 6.3 Keep-in box, written back every frame

While `interior` is current, clamp the eye after drag, dolly, and pan:

```js
x in [-0.98, 0.98]
y in [-3.20, -0.95]
z in [-0.98, 1.18]
```

`y = -3.20` stays `0.18` above the floor so the near plane (`0.04`, and the cinema code also forces `camera.near = 0.04`) does not enter the lids. `y = -0.95` stays below the soffit (`-0.35`) so a pitch-up cannot push the eye through the ceiling. The horizontal limits sit inside the terrace inner half, so a yaw cannot reach the wall.

Apply the same box to `frames` only on the inside of the cut, and remove the `currentPresetName !== 'interior'` exemption. Any clamp that still runs on the spherical path has to assign the clamped offset back into `currentRadius`, `currentPhi`, and `currentTheta` in the same frame. A position-only patch is overwritten by `setFromSphericalCoords` on the next line.

### 6.4 Close the south wall on the interior preset

`SherlockStage.render` sets `targetCutaway = 1` for both `interior` and `frames`. That retracts `southWallGroup` by `3.15` and opens the vault onto the gray void, which is the background of `grok_test_04` and `grok_test_05`. Drive cutaway from `frames` only. Interior stays a closed room. The cross-section keeps the cut.

### 6.5 Aim the cross-section at the module field

`frames.lookAt` is `[-0.06, -1.52, 0]`, mid-stack. Move the look-at down to the module tops so the new bevels and the active post are in the lower half of that shot, not on the last row of pixels:

```js
pos: [1.55, -1.35, 3.35],
lookAt: [0.0, -3.05, 0.1],
fov: 38,
```

The eye stays outside the south cut, which is the point of Cross-Section, and the look-at sits on the decoder instead of on the ring plane.

---

## 7. Acceptance

1. Load Inside Hollow Chip. The first frame shows the south rows of the decoder (gold sockets, lids, ids) with the bore and the soffit ring in the upper part of the frame. It does not open on the underside of the rings alone.
2. Drag pointer down about 80 px. The eye does not move. The gaze tilts down. Near-row lids get larger. The camera’s horizontal radius stays under `1.18`. The gray studio background stays hidden.
3. Drag pointer up until the pitch clamp. The split-arc rings and the stator come into view through the soffit opening. The eye is still at `y ≤ -0.95`.
4. Yaw a full turn. The south, west, north, and east walls stay outside the near plane. The shot never matches `grok_test_04_drag_down.png`.
5. Step the trace onto a known line (A135 PushFrame, then A145 BindLet). The A135 block sits visibly higher than its gold bezel, its south bar and 70 mm post are the rule color, and A145 takes over on the next step while A135 dims through `cell.trail`. From the cross-section pose, that post is distinguishable without reading the lid text.
6. Click one lid. The inspect card shows that rule’s id, line, full name, and description, not `k-a-1-decoder-bus-hub` and not `y = -2.18`.
7. Draw count for the floor stays at the ceramic slab, four instanced meshes (body, bezel, bar, post), the label instanced mesh, and the one ROM plaque. No per-rule `Mesh` in the update loop, and no per-frame allocation beyond the scratch objects already in the file.
