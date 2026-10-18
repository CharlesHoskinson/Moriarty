# Grok 4.7 xhigh Agent 5 of 5 — Interior Exploration UX, Closed-Vault Containment & Sub-View Presets

Auditor: Grok 4.7 (`xhigh`) Agent 5 of 5, Interior Exploration UX, Closed-Vault Containment, and Sub-View Presets.
Lens: a person who has been shrunk into the hollow chip and must stay inside it. They look around in place, ride a vertical shaft between the 39-rule floor and the six split-arc rings, and never orbit out through a missing wall.

Files inspected:

- `src/stage/SherlockStage.js` (`render` cutaway target, vault lights)
- `src/designers/designer7_cinema.js` (presets, orbit, wheel, pan, per-frame `lookAt`)
- `src/designers/designer1_package.js` (front-south gate, collar arc, bench drop)
- `src/designers/designer2_well.js` (chamber, ribs, soffit, 39-rule floor, south-wall retract)
- `index.html` and `src/app.js` (the “Drag to orbit” hint)
- Scratch captures under `/home/charles/.gemini/antigravity-cli/brain/67b84c75-1698-455f-a6d9-b18cc7cdde9b/scratch/`

This audit specifies the control change. It does not patch the stage.

---

## Verdict

`Inside Hollow Chip` is an orbit around a point on the ring plane, with the entire south side of the chip removed. The visitor is the size of the room. Turning the camera walks that orbit out of the shaft, and the missing south wall means the turn looks into the cream studio void.

`Cross-Section` is the view that should retract the south wall. `Inside Hollow Chip` should be a closed four-wall vault: obsidian terraces on all four sides, stainless ribs on all four sides, the 39-rule floor underfoot, and a full soffit with the six split-arc rings overhead.

Two code facts have to change together. Closing the wall without changing the camera drives the lens into the new wall. Changing the camera without closing the wall still shows the void when the gaze swings south.

---

## What the captures show

| Capture | What is on screen | Reading |
| :--- | :--- | :--- |
| `grok_test_01_interior_default.png` | Up the ring tunnel. Gold and teal soffits, stator, white wedge at the bottom of the bore. | Default `interior` pose. Floor tiles are behind the lens. The white wedge is the open south bay. |
| `grok_test_02_rotate_left.png` | Same tunnel, yawed. Wedge still at the south gap. | Yaw slides the view around the bore. It does not bring the floor into frame, and the south side is still open. |
| `grok_test_03_drag_up_to_see_floor.png` | Near plan view of the six rings and the stator, shot from above the die. | The drag that was meant to find the floor left the shaft and looked down from outside. |
| `grok_test_04_drag_down.png` | Black obsidian deck edge against a flat gray void. | Pitch orbit climbed to the deck plane. The chamber, the floor, and the rings are all out of frame. |
| `grok_test_05_wheel_zoom_out.png` | Exterior of the opened south bay: collar, four gold blocks, rings seen from outside, gray void above. | Wheel grew the orbit radius. The camera is outside the package, looking back in through the cutaway. |
| `clean_v1_interior.png` | Plaques of the upper wafers filling the frame at step 53. | An earlier interior pose sat in the wafer stack. The floor and the four walls are absent. |
| `clean_v2_interior.png` | Stepped atrium, rings, and a floor grid, with a wide pale opening on the right of the shaft. | The readable interior. The right-hand opening is the retracted south wall, so a turn exits the chip. |
| `clean_v3_interior.png` | Same up-the-bore framing as `grok_test_01`, white south wedge included. | Current preset reproduces the wedge. |
| `view_interior.png` | `Inside Hollow Chip` selected. Stage viewport is an empty black rectangle (step 106). | UI shell around the preset. The hint under the viewport still says “Drag to orbit · Scroll or pinch to zoom”. |
| `view_frames.png` / `clean_v1_frames.png` | `Cross-Section` selected. Exterior of the obsidian shield, `SHERLOCK` and `FACT(4)`, rings seen from above. | Cross-section is an outside architectural view. It is the preset that needs the south wall retracted. |

`view_frames.png` crops the stage to a corner of the shield, so it does not by itself prove the cutaway is open. The source does. `frames` and `interior` share one cutaway target.

---

## Measured vault (the containment box has to fit this)

Constants in `designer2_well.js`:

| Volume | Value |
| :--- | :--- |
| Chamber | `CHAMBER_TOP_Y = -0.35`, `CHAMBER_BOTTOM_Y = -3.35`, 8 terraces `D0..D7` |
| Terrace inner half | `lerp(1.32, 1.22, d / 7)`. D0 face at `±1.32`. D7 face at `±1.22`. Square, not round. |
| Terrace centers | `getTerraceY`: `-0.53, -0.91, -1.28, -1.66, -2.04, -2.42, -2.79, -3.17` |
| 39-rule floor | `WELL_FLOOR_Y = -3.38`. Tile centers `y = -3.376`. Label plane `y = -3.366`. |
| Floor grid | 6 × 7. Centers `x ∈ [-0.925, 0.925]` step `0.37`, `z ∈ [-0.96, 0.96]` step `0.32`. Cell `0.322 × 0.272`. Span `2.36`. |
| Floor slab | `WELL_BASE_Y + 0.01 = -3.39`, box height `0.024`. |
| Rings | `RING_TOP_Y = 0.06` (R1) down to `RING_BOTTOM_Y = -0.21` (R6). Group rest `baseY - 0.016`. Lower soffit pipe at `baseY - 0.041`. |
| Stator | `y = -0.24`. Open bore `r < 0.135`. Housing `r ≈ 0.138..0.220`. |
| Soffit ring | Mesh at `y = -0.34`, thickness `0.026`, `innerR = 0.955`, `outerR = 1.40`. |
| Package bore | Collar inner radius `WELL_RADIUS = 1.06`. Ceramic half `2.15`. |
| South package bay | `CUT_ANGLE = 0.25π` (±45°). Gate, south collar arc, south groove, south `FACT(4)` plate, and front value plate all live on `frontGateGroup`. |

The comment on the floor slab still says `y = -2.20..-2.18` (`designer2_well.js` around the slab mesh). The live constant is `-3.38`. The elevator in this audit is measured from `-3.38`. An elevator aimed at the stale comment would sink through the tiles.

At the bottom of the proposed shaft (`y = -2.85`) the inner face is about `±1.232`. At the top (`y = -0.55`) it is about `±1.32`. The soffit underside sits near `y = -0.37`. A lens at `y = -0.55` is about `0.18` under that soffit, which is the “right under the rings” station.

---

## 1. Closed four-wall vault in `Inside Hollow Chip`, cutaway only in `Cross-Section`

### The bug

`SherlockStage.render` sets one target for both presets:

```285:289:src/stage/SherlockStage.js
    const activePreset = this.getCameraPreset();
    const targetCutaway = activePreset === 'interior' || activePreset === 'frames' ? 1.0 : 0.0;
    this._cutawayMix = lerp(this._cutawayMix, targetCutaway, 1.0 - Math.exp(-12.0 * dt));
    if (Math.abs(this._cutawayMix - targetCutaway) < 0.005) {
      this._cutawayMix = targetCutaway;
```

`cutawayMix === 1` retracts every south surface that listens to it.

| Listener | At `cutawayMix = 1` |
| :--- | :--- |
| `designer1_package.js` `update` | `frontGateGroup.position.y = -1.86`, hidden once mix `≥ 0.96`. Bench drops from `-0.36` to `-2.21`. The south 90° of the ceramic tier, silver step, obsidian shield, and funnel collar leaves with the gate. |
| `designer2_well.js` terrace loop | `southRetractY = -3.15`. Every `southWallGroup` (south obsidian wall plus its chamfer ledge) is hidden once mix `≥ 0.85`. |
| Same loop | South segment of each recessed terrace light loop hidden and dropped. |
| Horizon ring | South segment of the `stackLimit` hairline dropped and hidden. |

From the interior lens at `(0, -1.95, 0.52)` the gaze is up and slightly north (`-Z`). South (`+Z`) falls at the bottom of that upward frame. With the south bay gone, the bottom of the bore is the scene background `0xf4f0e8` and the lowered bench. That is the white wedge in `grok_test_01`, `grok_test_02`, and `clean_v3_interior`.

`getCameraPreset()` returns the cinema preset name (`interior`, `frames`, `overview`, …). The fix belongs on that string.

### The fix

```javascript
const targetCutaway = activePreset === 'frames' ? 1.0 : 0.0;
```

`Inside Hollow Chip` (`activePreset === 'interior'`) keeps `targetCutaway = 0.0`. All four obsidian walls stay, the south light-loop segments stay, the horizon ring stays closed, `frontGateGroup` stays seated, and the bench stays at `y = -0.36`. The existing listeners already do this whenever mix is 0. They do not need a second flag.

`Cross-Section` (`activePreset === 'frames'`) keeps `targetCutaway = 1.0`. Its camera is outside the package at `(1.85, -0.18, 4.45)`, looking back at `(-0.06, -1.52, 0)`. That view is the architectural section. The south gate, the south chamber wall, and the dropped bench are what make the section readable. Leave that path alone.

Damping stays as written (`1 - exp(-12·dt)`). Switching from `interior` to `frames` still opens the bay. Switching from `frames` back to `interior` closes it. Switching from `overview` to `interior` no longer opens it.

### Two holes the mix does not close

`cutawayMix = 0` restores the south meshes. Two other openings are authored as always-open geometry, so a closed vault still has them until they are split the same way.

**Stainless soffit is 280.8°, and the gap faces east.** `createChamferedArcGeometry` builds a shape arc, then `rotateX(-π/2)`, so shape-theta `0` is world `+X` and shape-theta `-π/2` is world `+Z` (south). The soffit uses `thetaStart = 0.22π`, `thetaLength = 1.56π`. The missing `0.44π` (79.2°) is centered on `+X`. Looking up inside a “closed” vault still shows an eastern bite in the stainless ring. The south wedge in the screenshots is the wall cutaway, which is larger. The east bite is a second, smaller hole.

Build the soffit as a stationary 270° ring plus a south plug:

- Stationary arc: shape-theta start `-0.25π`, length `1.5π` (everything except the south 90°). This also fills the accidental east gap.
- South plug: shape-theta `-0.75π` to `-0.25π` (90° centered on `+Z`). Parent it to a group that copies `southRetractY` and `showSouthWall`.

At mix `0` the plug is seated and the soffit is a full ring. At mix `1` (`frames` only) the plug drops with the south wall, and the section still reads.

**Eight ribs leave the south face bare.** `VAULT_RIB_COORDS` places ribs at `z ∈ {-1.24, -1.22, -0.44, +0.44}` only. The comment in `designer2_well.js` says they leave the south cutaway window open. The south inner face near `z ≈ +1.22..+1.24` has obsidian and no flute. A 360° turn inside the closed vault shows ribs on the north, west, and east, and a flat south wall.

Add a second instanced set, `southRibGroup`, four ribs, same `0.036 × vaultRibHeight × 0.028` box and the same stainless material:

| x | z | rotY | Seat |
| ---: | ---: | :--- | :--- |
| `-1.22` | `+1.22` | `-0.25π` | Southwest corner |
| `-0.44` | `+1.24` | `0` | South wall |
| `+0.44` | `+1.24` | `0` | South wall |
| `+1.22` | `+1.22` | `+0.25π` | Southeast corner |

`rotY = 0` keeps the `0.028` thickness on `Z`, flush to the south face, matching the north ribs. In the well `update`, drive this group with the same `southRetractY` and the same `cutawayMix < 0.85` visibility as `southWallGroup`. Interior (mix 0) shows twelve flutes, three per wall plus the corners. Cross-section hides the four south flutes with the south wall.

Keep the south ribs on their own mesh. The existing eight are baked into one `InstancedMesh` and should stay put during the frames retract.

### Vault lights must not follow the cutaway anymore

```331:336:src/stage/SherlockStage.js
    // Boost architectural interior vault lights when viewing inside the hollow chip or cross-section
    if (this.vaultUpperLight && this.vaultFloorLight) {
      const vaultBoost = 1.0 + this._cutawayMix * 0.75;
      this.vaultUpperLight.intensity = 1.15 * vaultBoost;
      this.vaultFloorLight.intensity = 1.35 * vaultBoost;
    }
```

Today both presets force mix `1`, so the boost is `1.75` (upper `2.01`, floor `2.36`). After the split, `interior` mix is `0` and this formula would drop the closed vault back to the exterior intensities (`1.15` and `1.35`) just as the visitor needs the soffit and the floor lit from inside.

Drive the boost from the preset, and keep the numbers the interior already has:

```javascript
const inHollow = activePreset === 'interior' || activePreset === 'frames';
const vaultBoost = inHollow ? 1.75 : 1.0;
```

`frames` stays at the current boosted values. `overview`, `well`, `arena`, `circuit`, `loom`, and `cinema` stay at `1.0`. The three point lights stay where they are: upper at `(0, -0.65, 0.28)`, semantic well light at `(0, -1.55, 0.10)`, floor bounce at `(0, -2.85, 0.25)`. The floor light already sits on the lower elevator station.

Fog does not need a change. `FogExp2` density `0.016` over a `2.4` interior path is an optical depth of about `0.04`. The cream void is visible because the wall is absent, not because the fog is thin.

---

## 2. Why a drag or a wheel tick leaves the chip

`CAMERA_PRESETS.interior` is:

```javascript
pos:    [0.0, -1.95, 0.52]
lookAt: [0.0, -0.02, -0.05]
fov:    56
```

Every frame the cinema rig does this (`designer7_cinema.js` `update`):

1. Damp `currentRadius`, `currentPhi`, `currentTheta`, and `currentTarget`.
2. Add a transition arc of up to `+0.28` on `y` (`transitionArcBoost` starts at `1` on every preset change).
3. Place the camera at `currentTarget + spherical(radius, phi, theta)`.
4. Call `camera.lookAt(currentTarget)`.

The interior offset from look-at to camera is `(0, -1.93, 0.57)`.

| Spherical quantity | Value |
| :--- | :--- |
| Radius | `2.012` |
| Phi (from `+Y`) | `163.5°` (`2.854` rad) |
| Theta | `0` |
| Horizontal arm | `0.570` around the look-at point `(0, -0.05)` |

The look-at sits on the ring plane (`y = -0.02`), a little north of the bore. The camera hangs `1.93` below that point. The room’s inner half is `1.22..1.32`. The orbit radius `2.012` is larger than the room. The visitor is not small. The visitor is a crane whose pivot is on the ceiling.

Drag (`onPointerMove`, left button) does not rotate the lens in place:

```javascript
desiredTheta -= dx * 0.0068;
desiredPhi = clamp(desiredPhi - dy * 0.0058, MIN_PHI, maxPhi);
```

`getMaxPhiForState` allows `interior` to reach `0.96π` (`172.8°`). That is still “camera underneath the look-at”. It cannot aim the lens down at `y = -3.38` while the body stays at `y = -1.95`. Pointer-down decreases phi. Phi `90°` puts the camera on a circle of radius `2.012` at `y = -0.02`, which is the deck plane, `0.79` outside the inner face. That circle is the gray void and the black deck edge in `grok_test_04_drag_down.png`.

Yaw at the default phi only swings the `0.570` arm, so a small yaw can stay inside the square. The moment phi opens toward the horizon, the same yaw is a `2.012` arm and the camera is outside. Facing south while the bay is retracted looks straight out of the chip even when the arm is still short. `grok_test_02_rotate_left.png` is that south-facing gap.

The wheel edits radius, not height:

```javascript
desiredRadius = clamp(desiredRadius * exp(deltaY * 0.0011), 0.45, 13.5);
```

The dive helper only runs when the look-at is inside `hypot(x, z) < 0.65`, radius is already `< 2.1`, and `deltaY < 0`. A zoom-out (`deltaY > 0`) on `interior` grows the radius toward `13.5` and never pulls it back. `grok_test_05_wheel_zoom_out.png` is that radius, outside the package, looking into the opened bay.

The only wall clamp in `update` is explicitly skipped for these two presets:

```javascript
if (
  currentPresetName !== 'interior' &&
  currentPresetName !== 'frames' &&
  currentTarget.y < -0.60 &&
  camera.position.y < -0.22 &&
  camera.position.z < 0.75
) { /* scale xz into r = 0.86, clamp y to -3.22 */ }
```

`interior` and `frames` are the presets allowed to leave. Pan has the same shape: it slides `desiredTarget` by a scale of `max(0.9, radius) * 0.0014`, and only clamps target `y` to `[-3.35, 1.6]`. There is no clamp on target `x` or `z`. `lookAt` then keeps the lens tied to that escaped target.

The arc lift makes the entry worse. `setCameraPreset` sets `transitionArcBoost = 1` whenever the name changes. Peak lift is `+0.28`. Added to a lens already at `y = -1.95`, the entry hop is harmless. Added to the upper station `y = -0.55`, the hop reaches `y = -0.27`, which is inside the stator (`y = -0.24`) and above the soffit underside. The interior entry has to zero that boost.

`camera.lookAt` at the bottom of `update` overwrites any Euler written earlier in the same function. An in-place pitch only survives if the interior branch sets the Euler and does not call `lookAt`.

---

## 3. Exact interior rig: in-place micro-gimbal and scroll elevator

Shrink the visitor by giving them no orbit arm. While `currentPresetName === 'interior'`, the camera is a point inside the chamber. Drag changes yaw and pitch. The wheel changes `y`. Shift-drag or the middle button strafes `x` and `z`. Every frame clamps the point into the box below. `frames`, `overview`, `well`, `arena`, `circuit`, `loom`, and `cinema` keep the spherical orbit they have now.

### Pose to write into `CAMERA_PRESETS.interior`

```javascript
interior: {
  name: 'interior',
  label: 'Inside Hollow Chip (Looking Up at Rings)',
  subtitle: 'Closed four-wall vault. Drag looks around in place. Scroll rides the shaft from the 39-rule floor up to the six rings.',
  pos: [0.0, -1.70, 0.0],
  lookAt: [0.0, -1.28, -0.55], // seed orientation only; not an orbit pivot
  fov: 64,
}
```

`lookAt - pos = (0, 0.42, -0.55)` faces north (`-Z`) at `37°` above the horizon. Entry shows the upper obsidian course and the start of the soffit. Pitch from there reaches the rings. The elevator reaches the floor. Fov `64°` (half-angle `32°`) is wide enough to read a cluster of floor cells from `y = -2.85` without turning the shaft into a fisheye. The old fov `56` can stay if the rig lands; `64` is the one that matches “made smaller and able to see the modules”.

On entering `interior`, `setCameraPreset` copies `pos` into the desired eye, derives yaw and pitch from `pos → lookAt`, sets `transitionArcBoost = 0`, and does not write `desiredRadius / desiredPhi / desiredTheta` for this preset. On leaving `interior`, the destination preset fills the spherical state exactly as it does today.

### Constants

```javascript
const INTERIOR_X_MIN = -0.96;
const INTERIOR_X_MAX =  0.96;
const INTERIOR_Y_MIN = -2.85; // 0.53 above the tiles at y = -3.38
const INTERIOR_Y_MAX = -0.55; // 0.18 under the soffit underside
const INTERIOR_Z_MIN = -0.96;
const INTERIOR_Z_MAX =  0.96;

const INTERIOR_PITCH_MIN = -1.25; // -71.6°, up into the six soffits and the stator bore
const INTERIOR_PITCH_MAX =  1.20; // +68.8°, down onto the 39-rule tiles

const INTERIOR_YAW_SPEED   = 0.0052; // rad per pixel, ~121 px for a quarter turn
const INTERIOR_PITCH_SPEED = 0.0042; // rad per pixel
const INTERIOR_STRAFE_SPEED = 0.0024; // world units per pixel, not scaled by any radius
const INTERIOR_ELEVATOR_SPEED = 0.0028; // world units per wheel pixel
```

`±0.96` sits `0.27` inside the tightest inner face on this shaft (`≈ 1.232` at `y = -2.85`) and `0.36` inside the D0 face. The corner of the box is still inside the square room. Floor-cell centers run to `x = ±0.925` and `z = ±0.96`, so the eye can stand over any cell and look down. Near plane stays `0.025`. The lowest eye is `0.53` above the label plane, so the near plane does not cut the tiles.

Yaw is unbounded. Wrap the stored value into `(-π, π]` after each drag so the damp does not spin the long way.

### Pointer, while `interior`

Rotation order is `'YXZ'`. Yaw `0` and pitch `0` look down world `-Z` (north). Positive yaw turns the lens toward `-X` only if applied as Three.js `rotation.y`; the sign below is chosen so the picture moves the way a person expects.

Left button, no shift (look, in place, eye fixed):

```javascript
interiorYaw   -= dx * INTERIOR_YAW_SPEED;   // pointer right turns the view right
interiorPitch += dy * INTERIOR_PITCH_SPEED; // pointer down looks down, pointer up looks up
interiorPitch  = clamp(interiorPitch, INTERIOR_PITCH_MIN, INTERIOR_PITCH_MAX);
```

Do not edit `desiredTheta`, `desiredPhi`, `desiredRadius`, or `desiredTarget` on this path.

Shift, or button 1, or button 2 (horizontal strafe). Forward and right come from yaw only, so strafe cannot climb the shaft:

```javascript
const fwdX = -Math.sin(interiorYaw);
const fwdZ = -Math.cos(interiorYaw);
const rightX =  Math.cos(interiorYaw);
const rightZ = -Math.sin(interiorYaw);
interiorEyeX += (rightX * dx - fwdX * dy) * INTERIOR_STRAFE_SPEED;
interiorEyeZ += (rightZ * dx - fwdZ * dy) * INTERIOR_STRAFE_SPEED;
```

Pointer-right moves to the camera’s right. Pointer-up moves horizontally toward the wall the lens is facing, which is how a corner module comes under the crosshair. `y` does not change.

### Wheel, while `interior`

Ignore radius. A notch of `deltaY = 100` moves `0.28` world units. The shaft from `-0.55` to `-2.85` is `2.30`, about eight notches.

```javascript
const pixels =
  event.deltaMode === 1 ? event.deltaY * 16 :
  event.deltaMode === 2 ? event.deltaY * 120 :
  event.deltaY;
interiorEyeY = clamp(interiorEyeY - pixels * INTERIOR_ELEVATOR_SPEED, INTERIOR_Y_MIN, INTERIOR_Y_MAX);
```

Positive `deltaY` (wheel rolled toward the user) descends toward the floor modules at `y = -2.85`. Negative `deltaY` ascends toward the rings at `y = -0.55`. `preventDefault` stays. The overview dive helper and the `exp(deltaY * 0.0011)` radius term do not run in this preset.

### Per-frame apply, while `interior`

Use the same damp as the orbit, `damp = 1 - exp(-11·dt)`, on eye, yaw, and pitch. Then clamp the damped eye. Then write the camera and skip `lookAt`:

```javascript
camera.position.set(eyeX, eyeY, eyeZ);
camera.rotation.order = 'YXZ';
camera.rotation.set(pitch, yaw, 0);
camera.fov = desiredFov; // 64, damped like the other presets
```

`arcLiftY` is `0` for the whole time the preset is `interior`. The spherical block (`setFromSphericalCoords`, `position.copy(target).add(offset)`, `lookAt`) runs only for the other presets.

After the clamp, a frame is inside the vault when all three hold:

- `x ∈ [-0.96, 0.96]`
- `y ∈ [-2.85, -0.55]`
- `z ∈ [-0.96, 0.96]`

No drag, pan, wheel, or preset transition may publish a camera outside that box while the active preset is `interior`.

### What each axis is for

| Input | Motion | What becomes visible |
| :--- | :--- | :--- |
| Drag | Yaw `360°`, pitch `-71.6°..+68.8°`, eye fixed | North, east, south, and west obsidian, the rib on that wall, then the rings overhead or the tiles underfoot. |
| Wheel | `y` from `-2.85` to `-0.55` | Down: 39-rule cells fill the frame (from `y = -2.85` a `+69°` pitch lands the axis near the outer cells at `~1.1` from center). Up: the six soffit pipes (`y ≈ 0.02` down to `y ≈ -0.25`) and the stator bore at `y = -0.24`. |
| Shift-drag | `x, z` inside `±0.96`, `y` fixed | A chosen floor cell moves under the lens. Standing over `x = ±0.92` is how the end columns stay readable. |

From the mid station `y = -1.70`, a level gaze hits the walls. The soffit lip (`r = 0.955`, `y ≈ -0.35`) is about `55°` above the horizon, inside the pitch limit, so looking up from mid-shaft does show the rings. The old pose could not do this: pitch was an orbit around `y = -0.02`, and the floor was behind a camera that only knew how to look up.

### Hint copy

`index.html` and the cleared-inspector string in `src/app.js` both say `Drag to orbit · Scroll or pinch to zoom`. While `interior` is active, that line should read `Drag to look · Scroll to rise and descend`. Restore the orbit sentence on every other preset. The current sentence is the instruction that walks people out of the chip.

---

## 4. Five sub-view stations inside the closed vault

These are stations of the `interior` rig. They set eye, yaw, and pitch. They do not change `activePreset`, and they do not change `targetCutaway`. `frames` remains the only cutaway. Damp toward the station with the same `1 - exp(-11·dt)` used for the eye, and run the clamp afterward so a station can never settle outside the box.

Yaw `0` faces north (`-Z`). Pitch is Three.js `rotation.x` in `'YXZ'` (positive looks down).

| # | Station id | Eye `(x, y, z)` | Yaw | Pitch | What the systematic pass confirms |
| :--- | :--- | :--- | :--- | ---: | :--- |
| 1 | `interior-shaft` | `(0.00, -1.70, 0.00)` | `0` | `-0.42` | Default entry. Upper walls and the near soffit. All four walls in a yaw sweep. South wall is obsidian and ribs, with the cream void gone. |
| 2 | `interior-floor` | `(0.00, -2.85, 0.08)` | `0` | `+1.05` | South-central 39-rule cells, labels on the overlay plane at `y = -3.366`, gunmetal slab, lower ribs. |
| 3 | `interior-floor-west` | `(-0.62, -2.85, -0.18)` | `-0.55` | `+1.10` | West column of the 6×7 grid (`x ≈ -0.925`). Proves strafe-plus-pitch can read a module the center station only sees edge-on. |
| 4 | `interior-rings` | `(0.00, -0.55, 0.06)` | `0` | `-1.15` | Underside of R1..R6, both soffit light pipes, the open bore `r < 0.135`, the 10-sector stator at `y = -0.24`. Full soffit ring, east gap closed. |
| 5 | `interior-south-wall` | `(0.00, -1.70, 0.00)` | `π` | `+0.08` | Straight at the south inner face (`+Z`): obsidian terrace, south chamfer, two south ribs and two south corners, south light-loop segment, closed horizon ring. This is the shot that fails today. |

Station 5 is the containment test. If that frame shows `0xf4f0e8`, the bench interior, the gold leads, or the outside of the collar, the south bay is still open or the eye has crossed `z = 0.96`.

A yaw lap at station 1, stopped at `0`, `π/2`, `π`, and `-π/2`, is the four-wall pass. The same lap at station 2 is the floor pass. Station 4 held through a full yaw is the ceiling pass. Those three laps plus station 3 are the systematic walk.

Suggested bindings, if a later change adds them: keys `1..5` while the pointer is over the stage and `interior` is active. They are optional. The drag, wheel, and clamp are the feature.

---

## 5. Acceptance checks

1. Select `Inside Hollow Chip`. `cutawayMix` settles at `0`. `frontGateGroup` is at `y = 0` and visible. Every `southWallGroup` is at local `y = 0` and visible. The bench is at `y = -0.36`.
2. Select `Cross-Section`. `cutawayMix` settles at `1`. The south gate, south walls, south ribs, south soffit plug, and south horizon segment retract as they do today. The frames camera stays at its exterior preset.
3. From station 1, drag a full circle. Camera `x, z` stay at the station (look does not translate). `y` stays `-1.70`. The south stop shows obsidian and stainless, and the clear color `0xf4f0e8` is absent.
4. From station 1, roll the wheel toward the user until it stops. Eye `y` is `-2.85`, and `x, z` are unchanged. Pitch to `+1.05`. Several rule ids from the 6×7 overlay are readable. Roll the other way until it stops. Eye `y` is `-0.55`. Pitch to `-1.15`. All six ring soffits are in frame.
5. Shift-drag in every direction until the eye stops. The stop is the box `±0.96` in `x` and `z`, with `y` unchanged. Release and confirm the next frame did not ease past the box.
6. Switch `overview → interior` and `frames → interior`. The lens never publishes a position outside the box, including during the damp, and `transitionArcBoost` contributes `0`.
7. Vault intensities while `interior` or `frames` are active: upper `1.15 × 1.75`, floor `1.35 × 1.75`. On `overview` they return to `1.15` and `1.35`.
8. The stage hint reads `Drag to look · Scroll to rise and descend` only while `interior` is the active preset.

---

## Recommendations, in the order they should land

1. Change the cutaway line so only `frames` targets `1.0`. `interior` targets `0.0`. The package gate, the bench, the south terraces, the south light loops, and the south horizon segment follow that one number already.
2. Replace the interior spherical orbit with the micro-gimbal and the elevator in section 3. Clamp `[x, y, z]` to `[-0.96, 0.96] × [-2.85, -0.55] × [-0.96, 0.96]` after damping, and skip `lookAt` on that branch.
3. Decouple the vault-light boost from `cutawayMix` so the closed vault stays at boost `1.75`.
4. Close the soffit into a full ring, with a south 90° plug that retracts only when `frames` opens the bay. Add the four south ribs on a group that uses the same retract.
5. Reseed the `interior` preset to eye `(0, -1.70, 0)`, fov `64`, and point the cleared-inspector hint at look and elevator.
6. Add the five stations in section 4 as damped poses inside `interior`, and shoot station 5 plus the four-yaw lap before calling the vault closed.
