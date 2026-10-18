# Grok 4.7 xhigh Agent 4 — Miniature Observer Scale, FOV & Interior Proportions

Auditor: Grok 4.7 (`xhigh`) Agent 4 of 5 (Miniature Observer Scale, FOV & Interior Proportions).
Stage evidence: `clean_v3_interior.png` (Inside Hollow Chip, step 53 / 145, `Push(frame)`).
Code: `src/designers/designer7_cinema.js` (interior lens and orbit), `src/designers/designer3_frames.js` (continuation rings), `src/designers/designer2_well.js` (vault, terraces, 39-rule floor), cross-checked with `designer1_package.js` (`CERAMIC_HALF = 2.15`) and `src/stage/SherlockStage.js` (camera near plane).

---

## Verdict

The observer feels too big because the body of the camera rig is larger than the room it is standing in. The interior preset is a **56°** lens parked inside the continuation bore (`r = 0.64`), looking **+74°** up the shaft. That frame is a narrow patch of ceiling rings. The 39-rule floor is behind and below the eye, and it never enters the picture. Dragging the view orbits a **2.01**-radius crane around a target up at the stator, so a turn toward the horizon carries the eye out through the chamber wall.

Make the observer a point inside a room, and make the room use the ceramic body it already has. Widen the hollow from `innerHalf = 1.22` to **`innerHalf = 1.56`** (`OUTER_HALF = 1.68`, `floorSpan = 2.96`) so the interior is a **`3.12 × 3.12 × 3.40`** silicon-and-stainless atrium. Stand a **72°** micro-observer (`camera.near = 0.008`) in the lower-middle gallery at **`eye = [0.0, -2.05, 0.92]`**, with the Default Horizon/Diagonal Atrium Gaze **`lookAt = [0.0, -1.35, -0.45]`**. Look around in place, yaw free, pitch clamped to **+85°** (ceiling rings) and **−82°** (floor modules). The eye stays in the atrium for the whole turn.

---

## 1. Why the observer currently feels too big

### The lens is standing in the bore

`CAMERA_PRESETS.interior` in `designer7_cinema.js`:

| | Current interior preset |
| :--- | :--- |
| Eye | `[0.0, -1.95, 0.52]` |
| Look-at | `[0.0, -0.02, -0.05]` |
| Vertical FOV | **56°** |
| Gaze pitch above the horizon | **+73.6°** |
| Orbit radius around the look-at | **2.012** |
| Horizontal distance from the chip axis | **0.52** |

Designer 3 builds every continuation ring with `WAFER_INNER_RADIUS = 0.64` and `WAFER_OUTER_RADIUS = 0.96` (`designer3_frames.js`). The open bore is a tube **1.28** across. The eye sits at horizontal radius **0.52**, so the inner collar is **0.12** from the lens. The outer rhodium lip is only **0.44** away.

The eye height `y = -1.95` falls in the stack itself. Decks are `y = -0.64 - i * 0.19`, so wafer 7 is at **−1.97** and wafer 6 is at **−1.78**. The observer is between two annuli, inside their hole, not in a gallery outside them.

A 56° cone aimed up that tube fills with the nearest metal. A collar only 0.12 away subtends a band on the order of 18° across — a third of the whole frame for a lip that is 0.04 thick. `clean_v3_interior.png` is that picture: gold, teal, and amber arcs occupy every edge, the stator is a small disk at the end of the tunnel, and no wall and no floor module is in frame. With `fov = 56°` and the continuation rings encroaching inward to `r = 0.64`, the camera fills the bore and can only see a narrow patch of the ceiling or wall at one time.

### The room is a shaft cut inside a much larger package

The ceramic body is already `half = 2.15` (`CERAMIC_HALF` in `designer1_package.js`). The hollow does not use it.

| Volume | Measured |
| :--- | :--- |
| Narrowest terrace inner half | **`innerHalf = 1.22`** at D7 (`getTerraceInnerHalf` lerps `1.32 → 1.22`) |
| Inner room at the floor | **2.44 × 2.44** |
| `OUTER_HALF` | **1.44** (outer span 2.88) |
| Clear height, deck `y = 0` down to `WELL_BASE_Y` | **3.40** (`WELL_BASE_Y = -3.40`) |
| Wall at the bottom terrace | **0.22** thick, and the plan still tapers inward as you descend |
| Perimeter gallery outside the rings (`0.96` to `1.22`) | **0.26** wide |

The orbit radius (**2.01**) is already larger than the inner half (**1.22**) and larger than `OUTER_HALF` (**1.44**). The rig does not fit in the room. A person that size, pressed against a 0.64 bore, reads as a giant in a pipe.

### The floor modules exist, and this lens never looks at them

The 39-rule matrix is real geometry, not a missing model (`designer2_well.js`, `FlushInstanced39RuleFloor`):

- 39 instanced tiles, `0.322 × 0.008 × 0.272`, on a 6×7 grid.
- `floorSpan = 2.36`, tile origins from about `x ∈ [-0.93, 0.93]`, `z ∈ [-0.96, 0.96]`.
- Tile tops at `WELL_FLOOR_Y + 0.004` with `WELL_FLOOR_Y = -3.38`.

From the current eye `(0, -1.95, 0.52)` the floor center `(0, -3.38, 0)` lies at **−70°** pitch, **144°** off an optical axis that is aimed **+74°**. Half of a 56° lens is 28°. The modules are five half-angles outside the frame. The nearest (south) row is still steeper than **−70°**. Looking up, which is the only pose this preset offers, shows the canopy tunnel and leaves the floor behind the observer's feet. That is the report: the modules are on the floor, and these angles do not show them.

The plaque title still says `y = -2.18`. The mesh is at **−3.38**. Any camera note that aims at −2.18 is aiming at empty air a full unit above the tiles.

### One turn leaves the chip

Designer 7 always places the camera on a sphere around `desiredTarget`:

```text
camera.position = lookAt + spherical(radius, phi, theta)
camera.lookAt(lookAt)
```

Pitch and yaw move the **body** on that sphere. They do not turn the head. Interior look-at is `[0.0, -0.02, -0.05]`, up at the stator, and the radius is **2.012**.

| Gesture | Where the eye goes |
| :--- | :--- |
| Yaw at the default pitch (phi ≈ 164°) | Horizontal radius stays **≈ 0.57**, still inside the bore. The view stays a ring tunnel. |
| Pitch toward the horizon (phi → 90°) | Horizontal radius grows to the full **2.01**. That is outside `innerHalf` 1.22, outside `OUTER_HALF` 1.44, and against the ceramic at 2.15. The eye has left the hollow. |
| Pitch down to see the floor | The orbit lifts the eye **above** the stator target and out through the open top of the package. |

`getMaxPhiForState` already allows this swing: interior max phi is `π * 0.96` (about **+83°** of upward gaze) and `MIN_PHI = 0.002` (about **−90°**). Those limits describe the direction of the crane, and they do not keep the eye indoors.

The anti-clip block in the same update does the opposite of a room constraint, and it is switched off for this preset. It runs only when `currentPresetName !== 'interior' && !== 'frames'`, and then it crushes horizontal radius to **0.86** — back into the bore. Inside Hollow Chip is the one view with no containment at all.

`MIN_ORBIT_DISTANCE = 0.45` also refuses a miniature stance. Zooming walks the eye along the 2.01 crane toward the stator. It never shrinks the observer, and it never steps sideways into a gallery.

`clean_v3_interior.png` matches the preset exactly: Camera View is Inside Hollow Chip, the frame is the upward bore, the floor modules are absent, and the hint says drag to orbit. Orbit is the control that exits the package.

---

## 2. Make the observer tiny inside a 3.12 × 3.12 × 3.40 atrium

The ceramic half of **2.15** stays. The hollow inside it grows until the observer is a point and the architecture is the subject.

| | Now | Atrium |
| :--- | :--- | :--- |
| Package half | 2.15 | 2.15 (unchanged) |
| `OUTER_HALF` | 1.44 | **1.68** |
| Narrowest `innerHalf` | **1.22** | **1.56** on every terrace |
| Inner span | 2.44 × 2.44 | **3.12 × 3.12** |
| Vault drop, `y = 0` to `WELL_BASE_Y = -3.40` | 3.40 | **3.40** |
| Interior room | a tapering shaft | **`3.12 × 3.12 × 3.40`** |
| Shell thickness (`OUTER_HALF − innerHalf`) | 0.22 at the floor, 0.12 at D0 | **0.12**, constant |
| Ceramic margin outside the shell (`2.15 − 1.68`) | 0.71 | **0.47** |
| Gallery outside the rings (`r = 0.96` to the wall) | 0.26 | **0.60** |
| `floorSpan` | 2.36 | **2.96** |

`innerHalf = 1.56` is the whole change of feeling. The continuation rings stay a sculpture in the middle of the room (`r = 0.64..0.96`) instead of forming the walls of the room. A 0.60 gallery is wide enough to stand in, look along, and still see the opposite wall 3.12 away.

`getTerraceInnerHalf` should stop tapering. A well that narrows toward the floor is what makes a descending observer feel larger at every step. Eight terraces can keep their horizontal joints and their recessed light loops; the inner face stays at **±1.56** from D0 through D7.

Pieces that are bolted to the old wall and have to move with it:

- Vault ribs in `VAULT_RIB_COORDS` are planted at **|x|, |z| ≈ 1.22..1.24**. On a 1.56 wall they would hang in mid-gallery. Seat them on the new inner face, about **±1.54**, still leaving the south cutaway open.
- The `stackLimit` horizon loop is `createRecessedLightLoop(1.21, ...)`. It belongs on the new base perimeter, about **r = 1.54**.
- The stainless soffit is an annulus `innerR = 0.955`, `outerR = 1.40` at `y = -0.35`. Extend the outer edge to the new wall (**1.56**) so the ceiling meets the shell. Keep the inner edge near **0.98** so the bore up to the six split-arc rings stays open.
- The 6×7 floor constants (`startX = -0.925`, `stepX = 0.37`, `stepZ = 0.32`, `cellW = 0.322`, `cellD = 0.272`) tile a rug about **2.2** across on a plane of **2.36**. Scale that grid with `floorSpan` (**× 2.96 / 2.36 ≈ 1.25**) so the modules occupy the **2.96** span and leave a thin baseboard inside the 3.12 walls. A 2.36 rug in a 3.12 room would read as a mat, and the observer would still be looking at empty gunmetal.

The tiles are only **0.008** tall. From a gallery eye they read as a printed floor. Keep them a single instanced mesh, and give each module a small architectural thickness (a few centimetres of plinth, still flush with the overlay labels) so a downward look reads them as 3D modules on the floor of the chip.

---

## 3. The micro-observer lens

Replace the interior preset with a crisp wide-angle micro-observer lens. This is the whole of "made smaller so we can properly see everything": the eye is a point, the near plane is 8 mm, and the lens is wide enough to take in a room instead of a pipe.

| | Specification |
| :--- | :--- |
| Mode | In-place look from a fixed gallery eye. Yaw and pitch rotate the gaze. They do not orbit the body. |
| Eye | **`[0.0, -2.05, 0.92]`** — lower-middle gallery |
| Default gaze | **`lookAt = [0.0, -1.35, -0.45]`** — Horizon/Diagonal Atrium Gaze |
| Gaze pitch | **+27.1°** above the horizon, aimed across the atrium toward −Z |
| Vertical FOV | **`fov = 72°`** (half-angle 36°) |
| Near plane | **`camera.near = 0.008`** |
| Pitch clamp | **+85°** up to the ceiling rings, **−82°** down to the floor modules |
| Yaw | Full **360°**, eye held at the gallery point |

Why this eye is inside the room and clear of the metal:

- Horizontal radius **0.92** is inside the new wall at **1.56** (clearance **0.64**) and inside the old wall too.
- Wafer 7 is at `y = -1.97`, wafer 8 at `y = -2.16`. The eye at `y = -2.05` sits in the air gap between those decks, **0.08** under the deck above and **0.11** over the deck below. Both distances are well outside a **0.008** near plane, so the lips render as nearby architecture instead of clipping through the camera.
- Radial **0.92** is just under the outer rhodium band (`0.91..0.96`). That is a perch in the lower-middle gallery, between continuation decks, looking across the open bore. Dolly and pan must not climb the 0.08 into wafer 7.

Why 72° and this look-at belong together. Measured from `eye = [0.0, -2.05, 0.92]` with the optical axis on `lookAt = [0.0, -1.35, -0.45]`:

| Subject | Elevation | Angle off the axis | In the 72° frame? |
| :--- | ---: | ---: | :--- |
| Control stator center `(0, -0.24, 0)` | +63.1° | **+36.0°** | On the top edge |
| Far split-arc outer rim `(0, -0.21, -0.95)` | +44.5° | +17.5° | Upper third |
| Top continuation ring, far inner collar | +42.1° | +15.0° | Upper third |
| Vault soffit, far inner edge | +42.2° | +15.1° | Upper third |
| Unwind orb column at gaze height `(0, -1.35, 0)` | +37.3° | +10.2° | Center |
| North wall at eye height `(0, -2.05, -1.56)` | 0° | −27.1° | Lower third |
| North floor row `(0, -3.38, -0.96)` | −35.2° | −62.3° | One pitch downward |
| Floor center `(0, -3.38, 0)` | −55.3° | −82.4° | At the **−82°** pitch stop, centered |

The establishing gaze puts the **6 concentric split-arc rings and the Control stator** on the top of the frame, the **continuation frame gallery** across the middle, and the **rising Unwind Orb** in the center of the bore. The north wall sits in the lower third, so the shot is a room with a ceiling, not a tunnel. The **3D 39-rule floor modules** are the downward look from the same standing point: the north row comes in as pitch falls past the horizon, and the **−82°** stop centers the floor (the floor center is 82° below this optical axis). Pitch **+85°** rolls the canopy rings into the middle of the view, overhead, with the stator in the upper canopy.

The angular span from the north floor row up to the stator center is **98°**. A single 72° frame covers the canopy, the gallery, and the orb together, and the floor is the head-tilt from that same eye. Stretching the lens past ~90° to force the floor and the stator into one picture would fisheye the stainless shell. 72° is the wide architectural lens; **+85° / −82°** is how one miniature observer sees the rest without walking into a different room.

`SherlockStage` constructs the camera at near **0.025**. Designer 7 only writes `camera.near = 0.04` when the current near is already larger than 0.04, so the live near plane stays **0.025**. Set **`camera.near = 0.008`** for real on entering the interior preset, and restore **0.025** on leaving it. At 8 mm the lip of a continuation deck 80 mm overhead stays visible, and a module the observer pitches down toward does not disappear into the near plane.

Horizontal FOV at 72° vertical on a 16:9 stage is about **105°**. From this eye that is enough lateral coverage to hold the left and right continuation arcs and both side walls in the same glance, which is the 360° gallery the current 56° bore shot never offers.

---

## 4. Look-around that stays inside the chip

Keep Designer 7's spherical orbit for Overview, Top-Down, Cross-Section, and Cinema. Inside Hollow Chip needs a different controller: a head turn on a body that does not move.

If the new pose is fed through the existing orbit (`radius = 1.538` around `[0.0, -1.35, -0.45]`), the default frame is right and the room is still abandoned as soon as the user looks at the floor:

| Pitch of the orbit | Eye Y | Horizontal radius | Result |
| ---: | ---: | ---: | :--- |
| +85° | −2.88 | 0.13 | Still indoors, hovering above the floor |
| +27° (the preset) | −2.05 | 1.37 | The gallery perch. Inside 1.56 |
| 0° (horizon) | −1.35 | **1.538** | Just inside a 1.56 wall, clipping the shell |
| −30° | −0.58 | 1.33 | Up against the soffit |
| **−82°** | **+0.17** | 0.21 | Above the deck, **outside the chip**, which is the bug the user hit |

An in-place look at a frozen eye does not do this. Yaw 360° leaves the eye at radius 0.92. Pitch from **−82°** to **+85°** leaves it there. The floor modules and the ceiling rings are directions, not destinations.

Interior controller, concrete:

1. On entering `interior`, set `eye = [0.0, -2.05, 0.92]`, aim at `[0.0, -1.35, -0.45]`, `fov = 72`, `camera.near = 0.008`. Skip the parabolic arc-lift (`transitionArcBoost`) for this preset so the dive does not punch the eye up through a continuation deck on the way in.
2. Drag left/right changes yaw with no limit. Drag up/down changes pitch, clamped to **[−82°, +85°]**. Write `lookAt = eye + direction(yaw, pitch)`. Do not call `setFromSphericalCoords` to place the camera while this preset is active.
3. Pan (shift-drag or middle button) translates the eye in the view plane. Dolly (wheel) translates the eye along the gaze. After either one, clamp into the atrium box:
   - `|x| ≤ 1.40`, `|z| ≤ 1.40` (0.16 inside the 1.56 wall, clear of the ribs).
   - `y ∈ [-3.15, -0.48]` (above the tile plinths, below the soffit at −0.35).
   - Reject the continuation annulus: if `0.62 < sqrt(x²+z²) < 0.98` and the eye is within **0.03** of any deck `y = -0.64 - i * 0.19`, push it back to the previous valid point. The home perch is legal because it sits in the gap between wafer 7 and wafer 8.
4. Leave the `horizR > 0.86` crush unused for this preset. That clamp is what pins a visitor inside the bore.
5. FOV already damps toward `desiredFov` in the update loop. Setting the preset to 72 is enough for the lens. The near plane needs an explicit assignment; the existing `if (camera.near > 0.04)` branch will not lower it to 0.008.

The home perch is the composition. Walking the gallery is optional and stays indoors. Rotating the camera never is a way out of the chip.

---

## 5. Recommendations, in the order they change the feeling

1. **Widen the vault** inside the `half = 2.15` body. `getTerraceInnerHalf` returns **1.56** at every depth. `OUTER_HALF = 1.68`. The interior room is a **`3.12 × 3.12 × 3.40`** atrium. Move ribs, the stackLimit horizon, and the soffit outer edge out to that wall.
2. **Spread the 39-rule floor** to **`floorSpan = 2.96`**, scale the 6×7 grid with it, and give the tiles a few centimetres of plinth so they read as 3D modules from above. Point labels and inspect copy at `WELL_FLOOR_Y = -3.38`, where the mesh actually is.
3. **Replace the interior preset** with the micro-observer: `fov = 72`, `camera.near = 0.008`, `eye = [0.0, -2.05, 0.92]`, `lookAt = [0.0, -1.35, -0.45]`. Keep the continuation rings at `r = 0.64..0.96` so they become the sculpture in the middle of the gallery rather than the walls of a tunnel.
4. **Look around in place** for this preset only. Yaw 360°. Pitch **+85°** to the ceiling rings and the Control stator, **−82°** down to the floor modules. Contain pan and dolly in the atrium box above, and keep the eye out of the ring annuli.
5. **Leave the other presets on the orbit rig.** Overview, Top-Down, Cross-Section, and Cinema are exterior shots. They should keep their current radii. The miniature observer is a property of being inside the chip.

Checked against `clean_v3_interior.png`: the published Inside Hollow Chip frame is the 56° upward bore, step 53, with no floor and no wall. The five changes above are what turns that tunnel into one standing place from which the floor modules, the continuation gallery, the unwind orb, and the upper canopy are all directions in the same room.
