# Quineling Agent 2 — Camera Manipulation, Floor/Ring Sightlines, Spatial Envelope

**Lens controller:** `src/designers/designer7_cinema.js` (`CAMERA_PRESETS`, `setCameraPreset`, interior micro-gimbal).
**Stage wiring:** `src/stage/SherlockStage.js` `render` writes `fullState.activePreset` and `fullState.dt` before every `sub.update(fullState)`.
**Organism:** `src/designers/designer8_quineling.js`. `update(stepState)` never reads `activePreset`. `QUINELING_MAX_RADIUS = 0.46` is never applied. The plaque, tendrils, and species meshes stay at hero size on every lens.

Buttons the user actually presses: **Inside Chip** (`interior`), **39 Floor** (`interior_floor`), **Look Up** (`interior_up`), **Cutaway** (`frames`).

---

## What the stills show

| Capture | Lens | What sits in the sightline |
| :--- | :--- | :--- |
| `quineling_interior_factorial.png` | `interior` | Hero framing. Nautilus fills the bore, six rings overhead, 39 modules along the bottom. The `NAUTILUS ARITHMETICUS` plaque lies across the near floor tiles. This lens is supposed to feature the organism. |
| `quineling_interior_floor.png` | `interior_floor` | **39 Floor** is the active button. The module grid (RetOk, RetErr, RestoreScop, LookupFn, A206–A214, ROM hub) is the subject. Orange floor-root tendrils drop through the top of the stage and cross the upper rows. The shell and plaque sit on the top lip of this downward view and are mostly cropped by the stage frame. |
| `quineling_interior_up.png` | `interior_up` | **Look Up** is active. The six split-arc rings and the stator fill the frame, so the body and the downward plaque fall below the +81° rest gaze. The eight crown tendrils still run through that bore toward `y = -0.25`. |
| `quineling_cross_section_factorial.png` | `frames` | **Cutaway**. Eye is outside the package. Nautilus is small and centered, rings above it, all 39 modules readable on the floor. Hint reads “Drag to orbit · Scroll or pinch to zoom”. |
| `scratch/quineling_manip_01_orbit.png` | interior gimbal, yaw | Same chip, after a horizontal drag. The nautilus occupies the left third. The plaque and the tendrils cover central floor titles (RightOp and its neighbors). |
| `scratch/quineling_manip_02_pitch_floor.png` | interior gimbal, pitch down | Plaque parked on the central tiles (RightOp, BinPrim, NextArg). Tendrils cross RetErr and the middle of the grid. Shell covers the left column. Module text under those meshes is unreadable. |

`activePreset` does **not** change during drag, strafe, or wheel. A fade keyed on `interior_floor` / `interior_up` stays in force for the whole manipulation.

---

## What the camera does

Interior lenses share one first-person gimbal (`isInteriorPresetName`). Exterior lenses, including `frames`, use a damped spherical orbit. `update` damps with `1 - exp(-12·dt)`.

### Rest poses

`clampInteriorEye` enforces `hypot(x, z) ≤ 0.94` and `y ∈ [-2.96, -0.52]`. Pitch clamp is `[-0.465π, +0.472π]` (−83.7° to +85°). Yaw is `atan2(dir.x, -dir.z)`.

| Preset | Eye after clamp | Pitch / yaw | FOV | `camera.near` |
| :--- | :--- | :--- | :--- | :--- |
| `interior` | authored `z = 1.12` is pulled in to **`(0, -1.85, 0.94)`**. Look-at `(0, -2.18, -0.28)`. | **−15.1° / 0** | 72° | `0.008` once inside |
| `interior_floor` | `(0, -2.15, 0.88)` (already inside the cylinder). Look-at `(0, -3.35, -0.18)`. | **−48.5° / 0** | 70° | `0.008` |
| `interior_up` | `(0, -1.32, 0.18)`. Look-at `(0, -0.10, -0.02)`. | **+80.7° / 0** | 70° | `0.008` |
| `frames` | `(1.85, -0.75, 5.15)`, look-at `(-0.05, -1.95, 0)`. Radius ≈ **5.62**. Not the gimbal. | spherical, `phi` allowed up to `0.96π` | 42° | restored to `0.025` |

Core anchor is `QUINELING_CORE_Y = -2.05`. On **39 Floor** the eye is 0.10 below that anchor and aimed down through the plaque. On **Look Up** the eye is 0.73 above the anchor, inside the crown-tendril cone (at that height the ribbons already have radius ≈ 0.33; the eye sits at radius 0.18).

### Switching lenses

- **Inside Chip ↔ 39 Floor ↔ Look Up.** Stays in the gimbal. `seedInteriorFromPreset` retargets eye, yaw, and pitch. Shortest-yaw unwrap. No arc-lift. FOV eases 72° ↔ 70°. The organism does not move, scale, or fade.
- **Any exterior → an interior lens.** Eye is planted at `(pos.x·0.5, -0.65, pos.z·0.5)`, then glides to the preset. `camera.near` drops to `0.008`.
- **Any interior lens → Cutaway.** `camera.near` returns to `0.025`. Desired spherical pose becomes the `frames` preset. `transitionArcBoost` adds a `sin`-shaped lift of 0.28 that decays at 2.4/s. `SherlockStage` eases `cutawayMix` to 1, which opens the south gate. While the gimbal is active, `currentRadius / phi / theta` are left frozen at the last exterior sphere (overview radius ≈ 8.4 if the user never orbited). The first Cutaway frame therefore rebuilds the camera from that stale sphere around the interior aim point, then damps toward the section. Expect a pop, then a glide.
- **Cutaway drag** orbits θ/φ. Shift / right-button pans the look-at. Wheel dollies radius inside `[0.45, 13.5]`.

### Mouse and wheel inside the chip

Hint under the stage: “Drag up/down to see Rings & 3D Floor · Scroll to rise/descend · Shift-drag to move”.

- Left drag: `yaw += dx·0.0052` (unbounded), `pitch -= dy·0.0048`, clamped. The eye does not translate. This is `quineling_manip_01` (yaw) and `quineling_manip_02` (pitch).
- Shift, right button, or middle button: horizontal strafe at `0.0022` units/px in the yaw frame, then the radial and height clamps.
- Wheel: elevator, `y -= deltaY·0.0022`, same clamps. The eye never dollies through the wall.
- A press that moves under 5 px still raycasts. `plaqueMesh` is `tagInspectable`, so the card steals clicks meant for the floor chiplets underneath it.

---

## Why the floor and the rings lose

These are all children of `coreRoot` and they ignore the lens.

**Plaque.** `PlaneGeometry(0.74, 0.185)` at local `(0, -0.49, 0.34)`, `rotation.x = -0.18`, opacity `0.95`. World position ≈ `(0, -2.54, 0.34)`, leaning toward the +Z observer. Width 0.74 covers the two central columns of the floor grid (`startX = -1.125`, `stepX = 0.45`, `cellW = 0.395` → columns centered at `x = -0.225` and `x = 0.225`). From the 39 Floor eye the card is at about −36° pitch, inside the 70° frustum. `coreRoot.rotation.y = ambient·0.22` swings that card across the grid even when the mouse is still.

**Tendrils.** 8 floor roots from local `y = -0.18` to `floorRelY = -1.33` (world `y = -3.38`, the chiplet plane), radius `0.22 → 0.52`. 8 crown tendrils from local `y = 0.18` to `ringsRelY = +1.80` (world `y = -0.25`), radius `0.22 → 0.42`. Opacity `0.65`, `depthWrite: false`. Floor roots land on the inner cells. Crown ribbons cross the open bore the Look Up lens is aimed through.

**Body.** Hero nautilus reaches roughly `r ≈ 0.39` plus a hood of radius `0.155`. Other species are wider than the 0.46 budget the constant names: moth forewings, jelly bell `r = 0.31` with 16 tentacles down to local `y ≈ -0.40`, comet barrier disc `r = 0.36` at local `y = +0.34`, lotus outer cube `0.32`, siphonophore fins across `x ∈ [-0.42, 0.42]`. Several of those materials are opaque (`transparent` left at the default), so writing `opacity` later will not show the floor through them. The nautilus shell is already `transparent` at `0.92` with `depthWrite` still true, so it still hides tile text.

`designer3_frames.js` already branches on this same field (`resolveActiveCameraPreset`, around the wafer-badge block). Designer 8 should follow that, and only that. No new channel.

---

## What Designer 8 should do

Handle this inside `createQuinelingSubsystem().update`. Keep the hero pose for `interior` and for `frames`. On the two study lenses, scale the organism down, fade it, and remove the plaque so the modules and the rings are fully readable with the Quineling still glowing in the chamber.

### Targets

Read once per frame:

```javascript
const preset = stepState.activePreset || stepState.cameraPreset || 'overview';
```

Damp three closure scalars with `1 - exp(-8·dt)` (a little slower than the camera’s 12/s, so the fade reads as a settle). Store each material’s original `opacity`, `transparent`, and `depthWrite` at init in one array built from the species materials, `sdkRibbonMat`, and `sporeMat`. No per-frame allocation.

| `activePreset` | `coreRoot.scale` | Opacity multiplier | `depthWrite` while faded | Plaque | Tendril opacity | Extra `coreRoot.position.y` |
| :--- | ---: | ---: | :--- | :--- | ---: | ---: |
| `interior` | 1.00 | 1 (authored values) | restore authored | visible, opacity 0.95 | 0.65 | 0 |
| `interior_floor` | **0.58** | **0.38** | `false` | **`visible = false`** | **0.08** | 0 |
| `interior_up` | **0.50** | **0.34** | `false` | **`visible = false`** | **0.06** | **−0.22** |
| `frames`, overview, well, cinema, and the other exterior lenses | 1.00 | 1 | restore authored | visible, opacity 0.95 | 0.65 | 0 |

Apply the scale only on `coreRoot`. Child `mesh.scale.setScalar(...)` calls in the species block are local and keep working. Scaling the root also retracts the tendrils, because their vertices are in `coreRoot` space:

- Floor study, scale 0.58: a 0.46-radius body fits in `r ≤ 0.27`, inside the central cells instead of across their labels. Floor-root tips stop near world `y ≈ -2.82`, about 0.56 above the chiplets at `y = -3.38`.
- Look-up study, scale 0.50 plus the −0.22 drop: core sits near `y = -2.27`. The tallest solid (comet barrier, local `y = +0.34`) lands near `y = -2.10`, 0.78 below the Look Up eye at `y = -1.32`. Crown tips land near `y = -1.37`, at eye height, and the ring plane at `y ≈ -0.25` stays clear.

Hide the plaque outright on both study lenses. A 0.58-scale card is still 0.43 wide, wider than one floor cell, and a translucent text card still destroys the labels under it. `findInspectableTarget` already skips `visible === false`, so floor-chiplet clicks work again. Restore `plaqueMesh.visible` when the damped opacity climbs back above 0.02.

For every cached material while the multiplier is below 1: set `transparent = true`, `depthWrite = false`, and `opacity = baseOpacity * multiplier`. `depthWrite: false` is the part that makes the tile text actually show through. Opacity alone still loses the depth test. When the multiplier returns to 1, write back the stored `transparent`, `depthWrite`, and `opacity` so the Inside Chip and Cutaway shots stay exact.

Leave `corePointLight` alone. Leave species morphogenesis alone. Leave `interior` and `frames` at full presence: the factorial interior still is the portrait, and the cutaway still is the architectural context shot where the organism is already small enough to read past.

### Check

1. **39 Floor** at rest: every chiplet title from A080 through A225, plus the ROM hub, is readable. Plaque is gone. Tendrils are faint and stop short of the tiles. The nautilus is still visible above the grid at about 40% opacity.
2. On **39 Floor**, yaw a full turn and pitch across the clamp. `activePreset` stays `interior_floor`, so the fade must hold the whole way. This is the `manip_01` / `manip_02` failure.
3. **Look Up** at rest: all 6 rings and the 10-sector stator are clear. Crown tendrils do not cross the ring metal.
4. On **Look Up**, wheel the elevator down to `y = -2.96`. The organism stays at the study scale and does not fill the bore.
5. **39 Floor → Inside Chip**: scale and opacity ease back to 1 over roughly half a second, plaque returns, `depthWrite` is restored on the shell.
6. **Look Up → Cutaway**: full organism, plaque on, south gate open, spherical orbit. No material left with a stale `depthWrite = false`.
7. Repeat 1 and 3 on the three silhouettes that exceed the radius budget: Luna moth, Medusa bell plus tentacles, and the comet barrier disc.
