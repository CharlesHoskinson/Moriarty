# Grok 4.7 xhigh Audit #5 — Hollow Chamber Wafers, Call Tiles, Camera & Raycast

Auditor: Grok 4.7 xhigh Auditor #5 (Hollow 3D Chamber Wafers, Call-Tiles & Camera Choreography).
Lens: Apple visionOS spatial layout, materials, and eyes guidance, applied to a desktop pointer that stands in for gaze. Stage lens is `PerspectiveCamera` fov **34°** (vertical), near plane **0.04** after Designer 7 initializes it (`SherlockStage` starts at `0.1`).

Files inspected:

- `src/designers/designer3_frames.js`
- `src/designers/designer4_arena.js`
- `src/designers/designer7_cinema.js`
- `src/designers/ring_study_09_wafers_calls.md`
- Cross-checks: `designer2_well.js` (chamber, rings, stator), `designer1_package.js` (bench and lid), `theme.js` (`tagInspectable`, plaque canvas), `sherlockEngine.js` (frame order, rules 153 / 161 / 165 / 181 / 185), `ring_study_02_frames.md` (fact(4) slot table), `index.html` lens bar.

Angles below were computed from the preset vectors and a 34° vertical fov. A point is inside the frame only when its angle off the optical axis is under **17°**.

---

## Verdict

The chrono mapping, the family colors, and the bore clearance are sound. The spatial presentation is not.

**Inside Hollow Chip** parks the lens on the front lip of the wafers, 4 mm outside the metal and inside the near plane. Its own subtitle (rings above, floor below) is 34°–51° outside the frame. The dive into that preset is captured for about half a second by the anti-clip cylinder and scraped along the wafer corners.

The front-apron call tiles are real, correctly colored, and clear of the lid. No lens in the camera bar looks at them. Overview misses the row by 5°. The arena preset, which is not even on the bar, misses it by 47°.

The unwind orb is a 45 mm teal sphere that flies to `y = -0.24` on almost every pop, including `If` and `Args` pops, and its dock sits inside the sapphire lens and the north–south stator bridge. It never stops in a parent `[?]` hole.

Raycasting walks the whole scene, treats untagged geometry as absent, gives every line a 1-unit hit radius, and can keep a reticle on a wafer that has already popped.

---

## 1. Measured chamber (what the wafers actually fit into)

These numbers are the contract the next iteration has to respect.

| Volume | Measured bounds |
| :--- | :--- |
| Wafer outer bevel | `x ∈ [-0.623, 0.623]`, `z ∈ [-0.583, 0.583]`, local `y ∈ [-0.012, 0.012]` |
| Sapphire glass plate | `x ∈ [-0.570, 0.570]`, `z ∈ [-0.530, 0.530]`, local `y ∈ [-0.007, 0.007]`, hole radius `0.16` |
| Top plaque | `0.92 × 0.21` at local `y = 0.0115`, `z ∈ [0.245, 0.455]`, normal `+Y`, `FrontSide` |
| Square chamber inner half | `0.986` at terrace D0 (`y ≈ -0.46`) down to `0.916` at D7 (`y ≈ -2.04`) |
| Control stator | world `y = -0.24`, housing hole `r ≈ 0.145`, sapphire lens `r = 0.135`, thickness `0.006`, local `y = 0.001` |
| North–south bridge | `0.012 × 0.008 × ~0.46` through the origin, local `y = -0.004` (crosses the bore) |
| 39-rule floor | `y = -2.18` |
| Ceramic lid | `±2.15`. Tile back face is `z = 2.34`. Gap in front of the lid is `0.19` |
| Bench | `y = -0.34`, spans `±5.4` |

Gallery that is actually empty: in front of the wafers and inside the square wall, about `z ∈ [0.64, 0.90]` at `x = 0`. That band is only ~0.26 deep. A 34° lens standing there cannot see a shaft that runs from `y = -0.24` to `y = -2.18`.

Wafer stack for the settled 9-frame fact(4) picture (`spacing` hits the `0.165` cap):

| chrono | slot id the code stamps | `y` |
| ---: | :--- | ---: |
| 0 | `R1·A` | -0.480 |
| 1 | `R1·B` | -0.645 |
| 2 | `R2·A` | -0.810 |
| 3 | `R2·B` | -0.975 |
| 4 | `R3·A` | -1.140 |
| 5 | `R3·B` | -1.305 |
| 6 | `R4·A` | -1.470 |
| 7 | `R4·B` | -1.635 |
| 8 | `R5·A` | -1.800 |

Peak fact(4) is not this picture. `ring_study_02` step `S123` holds **11** frames: `R5·B` is `If(?)` and `R6·A` is `0 == [?]`. Eleven wafers at `spacing = 0.142` put the deepest plate at `y = -1.900`, still 0.28 above the floor. The code will show that stack. Study 09’s “peak is 9 frames, ending at `R5·A` RETURN fact(0)” is the settled instant around `S119`, one push before the base-case test.

Frame order is the right one. The engine `unshift`s, so `frames[0]` is the newest. Designer 3 reads `rawAfterFrames[length - 1 - chronoIdx]` (`designer3_frames.js` around the chrono loop). Designer 2 fills `_chronoFramesBuffer[s]` the same way. `R1·A` stays the oldest frame. Keep that.

---

## 2. Continuation wafers

What holds up:

- One shared glass geometry, one rhodium bevel, per-slot rim and collar materials colored by `ENV / OP_AMBER / OP_GOLD / GATE`. Colors match study 09 and `FRAME_KIND_META`.
- The hole is a real `Shape` hole of radius `0.16`. The orb (radius `0.045`) and the stator bore (`0.145`) both clear it.
- The footprint fits the square well at every terrace. Deepest of 12 plates stays at or above `y = -1.90` because `maxSpan = 1.42` and `targetY` is clamped to `[-1.92, -0.46]`.
- Plaques and inspect metadata refresh only when the signature changes. No `new THREE.Color` / `Vector3` / `Matrix4` inside `.update`.
- In-place `Left → Right` rewrites (rule 149) keep the same slot and only change the plaque and the rim color. That matches the rings.

What a person inside the chamber actually gets:

1. **Labels face the ceiling.** The plaque is a `PlaneGeometry` with `rotateX(-π/2)`, `MeshBasicMaterial` default `FrontSide` (`designer3_frames.js` plaque setup). From the cross-section and the top-down ring lens you can read them: the frames preset sees the top plaque 3.6° off axis, at about 51° from flat-on, which is foreshortened but legible. From Inside Hollow Chip the camera sits at `y = -0.62`, below wafer 0 (`y = -0.48`) and 13 mm above wafer 1’s top face, looking further down. The plaque normal points away from that camera. visionOS type sits on a window that faces the viewer. Research d3 asked for an upward-canted plaque plus a lexical capsule on the right rim. The capsule was never built. The cant is 0°.

2. **Glass is a dark stack, not spatial glass.** `sharedGlassMat` is `opacity: 0.80`, `transparent: true`, `depthWrite: false`, `side: DoubleSide`, roughness `0.18`, no `transmission`, no thickness, no IOR (`designer3_frames.js` glass material). One plate already throws away 80% of what is behind it. Two plates leave 4%. The hole is the only sightline, which is correct for a bore view and wrong for a gallery view, where the stack reads as a solid dark column. visionOS glass is a thin frosted sheet: low fill, sharp specular rim, text on an opaque plate in front of the glass. `DoubleSide` also doubles the raycast hits.

3. **A new wafer is born at `y = -0.45` at full footprint** (`currentY = -0.45`, `currentScale = 0.88` when `wasVisible` flips). A push at chrono 10 then lerps down through every plate above it. `lerp` factor is `clamp(dt * 11, 0.08, 0.45)`, which is frame-rate dependent. The camera uses `1 - exp(-8 dt)`. The stack should use that same damping. On pop, the slot’s `visible` flips false in one frame. The plate does not fold into its hole.

4. **The teal “dock flash” hits the deepest remaining plate, and the comment describes the opposite.** Around the flash block: “top-most waiting frame”, while `isUnwindTarget = isUnwinding && isDeepestActive`. During unwind the deepest survivor is usually the parent, so the color sometimes lands on the right plate by accident. Any step classified as unwinding also lerps that plate’s rim toward teal, including an `If` pop whose family is violet.

5. **Lexical fallback walks toward the child.** If a frame has no bindings, the loop from `chronoIdx + 1` borrows a deeper frame’s bindings, then `envRoots[0]`. Scope comes from the parent. A root `Return` with an empty `bindings` array will print the base-case `n`. Search lower chrono indices, then the root env.

6. **Slot letter is the chrono parity, which is what the rings do, and study 09 over-explains it.** `formatHalfRingSlotId` is `R(floor(i/2)+1)·A|B` and it is stored on the slot at construction. It is not “A means Return”. At `S009`, `R1·B` is an `If`. At `S123`, `R6·A` is `0 == [?]`. Color carries the family; the letter carries the ring slot. The plaque subtitle already includes both. The study’s peak table should be rewritten to the 11-frame `S123` row, and the call-tile inspector copy should stop saying every call owns a stable `A` and `B` pair.

There is no light-spine mesh in Designer 3. The stage layer still describes “Frosted-Glass `[?]` Wafers, Light-Spine & Rising Result Orb”. The hole has no thread.

---

## 3. Rising result orb

The mesh matches the study: sphere radius `0.045`, color and emissive `0x2dd4bf`, travel on `(x = 0, z = 0)`, target `CONTROL_CORE_Y = -0.24`.

The motion does not match a substitution unwind.

**Trigger is far wider than a returned value** (`designer3_frames.js`, `isUnwinding`):

- stack length dropped, or
- `controlKind === 'Return'` or `'PrimitiveResult'`, or
- `ruleName` contains `"Return"`, `"Binary primitive"`, or `"Restore scope"`.

`controlKind` is the *after* control. Rule 153 (binary primitive) ends as `PrimitiveResult` and pops the `Right` frame, so the orb fires. Rule 185 (`RetOk`) ends as `Return` and also fires. A single fact unwind level therefore launches the orb twice. Rule 161 (`IfTrue`), rule 165 (`IfFalse`), and rule 181 (`FinishArgs`) shrink the stack and launch it too. An `If` closing is not a value climbing the shaft.

**Start height is the survivor, not the plate that popped.** After the hide loop, `popIndex = max(numWafers, prevCount) - 1` and `clampedIdx` is forced down into the still-visible range. Going from 9 plates to 8 starts the orb at wafer 7, the parent, and the popped child’s last `y` is discarded. When the stack hits 0, `spacing` has already collapsed to the one-wafer value `0.14`, so the last launch starts at the wrong terrace.

**Dock intersects the core.** Orb center `y = -0.24`, radius `0.045`, so the sphere occupies `y ∈ [-0.285, -0.195]`.

- Sapphire lens bottom is about `y = -0.242` (`core -0.24`, local `+0.001`, half-thickness `0.003`). The center of the orb is inside the lens. The lens plaque (`0.225 × 0.072`, opacity `0.96`) sits on top of that disc, so the bore camera sees the plaque, not the arrival.
- The north–south bridge crosses the origin at about `y = -0.244` and is only 12 mm wide in `x`. The orb swallows the strut.

A clear dock is `y = -0.30` (top of a 45 mm sphere at `-0.255`, just under the bridge). Better: split that bridge into two struts with `|z| > 0.16` so the bore stays open, and pulse the lens emissive teal when the orb arrives. Do that in Designer 2; Designer 3 cannot dock cleanly while the strut occupies the axis.

**Pause loop.** If `stepFraction <= 0.005`, `orbT = (ambient * 1.6) % 1`. A paused unwind step makes the orb patrol the shaft forever. Hold `orbT` at `0` when the playhead is not advancing.

**Pedagogy the spec still asks for.** `DESIGNER_SPEC` and research d3 want the value to plug the parent hole, then the parent equation to close (`4 × [?] → 4 × 6`). Study 09 sends every pop to the stator instead. Both beats fit in one step:

- `stepFraction` 0.00–0.55: rise from the popped plate’s last `y` to the parent plate’s `y`. Color moves from the popped family color to teal. Parent plaque rewrites `[?]` to the returned atom.
- `stepFraction` 0.55–1.00: only on rule 185 when no parent `Right` remains, or on rule 80 `Done`, continue from the parent hole to `y = -0.30`.

A single reused cylinder, radius `0.004`, from the orb down to the plate it left, is the light-spine. Opacity follows `orbT`. No per-frame geometry.

The hit target is the visible sphere, 90 mm across. From the overview camera (~6 units away, 34° fov) that is a few pixels. Add a pick proxy of radius `0.12`, opacity `0`, `depthWrite: false`, `visible: true` (a `visible: false` mesh is rejected by the visibility walk in `findInspectableTarget`).

---

## 4. Front-apron call tiles

Placement is calm and physically clear. `APRON_Y = -0.31`, `APRON_Z = 2.52`. Body `0.64 × 0.044 × 0.36`. Back face at `z = 2.34`, lid at `z = 2.15`, gap `0.19`. Bottom of the plinth is essentially on the bench at `y = -0.34`. Porcelain `0xebe5da`, platinum plinth `0xd8dde6`, champagne bezel. State colors match the study: waiting `0xf59e0b`, active `0xfbbf24` (crimson via `PALETTE.errorRed` on trap), returned `0x2dd4bf`. For fact(4), `resolveCallRingNumber` yields R1..R5 in call order, because depth 0 uses `call.index` and depth > 0 uses `depth + 1`. Five tiles fit in `MAX_CALL_TILES`.

The row is invisible from every lens the product exposes.

Lens bar (`index.html`): Overview, Inside Hollow Chip, Top-Down Rings, Cross-Section, Orbit. `arena` is not on the bar.

Off-axis angle of the center tile `(0, -0.31, 2.52)`, 17° half-fov:

| Preset | Angle | In frame |
| :--- | ---: | :--- |
| `overview` `[0, 4.15, 4.25]` → `[0, -0.10, 0.20]` | 22.4° | no |
| `well` `[0, 3.65, 0.65]` → `[0, -0.25, 0]` | 34.7° | no |
| `frames` `[2.10, 1.85, 2.35]` → `[0, -0.65, 0]` | 39.2° | no |
| `interior` | 120° | behind the camera |
| `arena` `[-1.75, 1.85, 2.75]` → `[0, 0.42, 0]` | 46.9° | no |

`arena` still looks at `y = 0.42`, the old value stage. Designer 4’s header says that volume is deliberately empty. The lens and the tiles were never retargeted together.

At aspect ~1.45 the horizontal fov is only ~48°. Five tiles at spacing `0.72` run from `x = -1.76` to `1.76`. Even a camera placed on the apron and aimed at the row cannot hold the end tiles inside a 34° lens. The row has to shrink, or the apron lens has to widen while it is active.

Plaques are another ceiling decal: `PlaneGeometry` rotated `-π/2`, `FrontSide`, lying on the top face. The state that matters (`waiting`, `active in core`, `= 24`) lives in that texture. The bezel is 18 mm tall. Amber versus gold on that hairline will not read as two states from the hero camera. Body tint (`0xebe5da` / `0xf3ede0` / `0xe6f0ee`) is the right amount of restraint and too small to carry the state alone.

Inspector copy overclaims. `tagInspectable` and the per-frame details say the tile “corresponds directly” to wafers `Rk·A` and `Rk·B`. For `fact(0)`, `R5·B` is a transient `If`, never a gold multiply. Say: “Call depth k is ring Rk. Slot A/B show whatever frames currently occupy that ring.”

`userData.id` stays `call-tile-${slot}` while the title becomes `Call #${call.index}`. After the windowing path (root + latest 5 when there are more than 6 calls) the spatial slot and the call index diverge. Write the call index into `id` when the meta block updates.

`getOrBuildTraceCalls` and `plaqueCache` are module globals. `dispose()` clears the plaque cache for every instance. Status is written onto the cached call objects every frame. Fine for one stage. A second arena, or a trace replaced without changing `programName:stepCount`, will show the previous run’s calls.

Rule detection itself matches the engine: enter on rule line 200 or `controlKind === 'Enter'`, return on 185. That is the fact(4) path. The rule-135 synthesizer is only the fallback for programs with no `Enter`.

---

## 5. Camera presets, especially Inside Hollow Chip

Preset record (`designer7_cinema.js`):

```
interior  pos [0.58, -0.62, 0.62]  lookAt [0, -1.05, 0]
```

`research_d7.md` still says `[0.66, -0.28, 0.68]` looking at `[0, -1.15, 0]`. The code moved and the research note did not.

### 5.1 Where this camera actually is

- Horizontal radius `hypot(0.58, 0.62) = 0.849`.
- Bevel front face is `z = 0.583`. The lens is **37 mm in front of the lip**, and the near plane is **40 mm**. The lip of whichever plate shares this height falls inside the near plane.
- Wafer 1’s center is `y = -0.645`. The camera at `y = -0.62` clears that plate’s top (`-0.633`) by about 13 mm. It is in the gap vertically and in the lip horizontally.
- Optical axis offset length is `0.952`. `phi ≈ 63°`. `getMaxPhiForState` allows up to `0.62π ≈ 112°` here, so `setCameraPreset` does not flatten the shot. The bad pose is the pose that was authored.

### 5.2 What the 34° frame contains

From that pose:

| Target | Off axis | Distance |
| :--- | ---: | ---: |
| Look point (middle of the bore) | 0° | 0.95 |
| Mid-stack plaque `(0, -1.05, 0.35)` | 20.1° | 0.77 |
| Top plaque `(0, -0.48, 0.35)` | 44.6° | 0.65 |
| Control core `(0, -0.24, 0)` | 51.0° | 0.93 |
| Floor `(0, -2.18, 0)` | 34.6° | 1.78 |

The subtitle promises wafers, the six rings, the core, and the floor in one interior shot. The core and the floor are not in this frame. The top label is not in this frame. The mid label is just outside it. What remains is the rim and the hole of the middle plates, seen from a corner, with the front lip clipped by the near plane.

This is a room-scale problem, not a tuning problem. The empty gallery is ~0.26 deep. The shaft from core to floor is 1.94 tall. Covering that height from `z = 0.80` wants a half-angle near 50° (fov near 100°), which is a fisheye on this stage. visionOS does not solve a tall room by widening until the ceiling and the floor are both sharp. It places the viewer in front of one rank of windows, and it moves when the active rank changes.

### 5.3 Anti-clip has the inequality backwards

```javascript
if (camera.position.y < 0.16) {
  const horizR = Math.hypot(camera.position.x, camera.position.z);
  if (horizR > 0.86 && horizR < 2.25) {
    camera.position.x *= 0.86 / horizR;
    camera.position.z *= 0.86 / horizR;
  }
}
```

`0.86` is the wafer corner (`hypot(0.623, 0.583) = 0.853`), not the wall. The wall is out at `0.92–0.99`. The guard pulls any underwater camera in the band `0.86 < r < 2.25` onto the wafer corner, and it does the pull in one frame, without damping, and without writing back into `currentRadius`. The spherical solver then puts the camera back on the large radius next frame, and the guard yanks it again.

Simulated 60 fps dive from `overview` to `interior`: **33 frames** (about 0.55 s) take that snap. Around frame 20 the unconstrained pose is near `y = -0.40`, `r = 1.11`, and the guard collapses it onto `r = 0.86`, which is the stack. The transition into Inside Hollow Chip scrapes the wafers.

The same band catches a user who tries to step back from the lip into the readable gallery (`z ≈ 0.82`, `r ≈ 0.82` is still legal; `z ≈ 0.90` is not). Moving outward, the one direction that adds air, is what the guard forbids. Moving inward, into the glass, is allowed down to `MIN_ORBIT_DISTANCE = 0.55`. The hole is only `r = 0.16`, so that minimum still lives inside the plates.

`frames` and `well` themselves are the lenses that actually show the shaft. Keep them.

| Preset | Core | Floor | Top plaque | Center tile |
| :--- | :--- | :--- | :--- | :--- |
| `well` | in (0.0°) | in (3.1°) | in (5.3°) | out (34.7°) |
| `frames` | in (4.9°) | in (13.6°) | in (3.6°) | out (39.2°) |
| `overview` | in | in | in | out (22.4°) |

`well` is the bore. `frames` is the cutaway. `interior` should become the gallery and should stop claiming the other two jobs.

### 5.4 Cinema orbit

`cinema` adds `dt * 0.18` to theta and rewrites phi, radius, and `desiredTarget.y` whenever the pointer is up. It never enters the chamber. It also fights a vertical pan the moment the button is released. Leave it as the hero orbit. Do not use it to explain the stack.

---

## 6. Raycast, as a visionOS gaze proxy

`performRaycast` casts `scene.children` recursively at 16 Hz. `findInspectableTarget` walks hits in distance order and returns the first visible object with `userData.inspectable` or `inspectableParent`. The corner reticle is excluded, eases with `1 - exp(-20 dt)`, and swaps gold for `heatBright` when pinned. That reticle language is the right one: eight hairlines, no cage, no spinning beacon.

Failures, in the order they will be felt:

1. **Untagged geometry does not occlude.** A hit that is not inspectable is skipped, not treated as a wall. The lid is tagged, and the aperture is a real hole, so the hero shot behaves. Terrace walls are mostly untagged. A frosted plate that should block gaze at a wafer behind it will not, once its opacity drops, unless the rule is explicit: opaque untagged hits stop the search; hits with material opacity under ~0.35 or `userData.gazeThrough === true` do not. Glass should set `gazeThrough` on the plate and keep the collar, the plaque, and the pick proxy solid.

2. **The top plate owns the bore.** Glass, bevel, and rim are children of the inspectable slot. From `well`, the top wafer wins everywhere except its hole. Rays down the shared hole miss every plate (the holes are concentric) and hit the floor or the orb. Picking `R4` from the top-down lens only works on the annulus where perspective peeks through the top hole onto the next glass. Canted front plaques, each the pick root for its slot, give one target per terrace from the gallery. Point the reticle at the plaque box, with a minimum box height of `0.06`, so the brackets frame the window. `setFromObject` on the whole wafer produces a 24 mm-tall box and the vertical hairlines collapse to the 14 mm minimum.

3. **Line threshold is 1 world unit** (`Raycaster` default). Call-tile rims are `LineSegments` on a `0.72` pitch. The neighbor’s rim is a legal hit. Set `raycaster.params.Line.threshold = 0.015` once, and set `rimLines.raycast = () => {}`.

4. **Throttle has no trailing sample.** `onPointerMove` returns when `now - lastRaycastTime < 60`. A pointer that stops inside that window keeps the previous hover until the next move. Clicks recast, so selection is honest and hover is not. Store the latest NDC and cast from `update()` at 16 Hz. Gaze-style highlight needs the resting point.

5. **A pin survives the pop.** The reticle block requires `parent` and a non-empty box. It does not require `visible`. A pinned wafer that unwinds stays pinned at its last pose, and the step-change path re-emits that metadata. Clear `pinnedObject` when `!visible` or when `userData.id` changes, and emit the hover target.

6. **Whole-scene casts will get heavier if glass gains transmission.** Twelve beveled extrusions at `curveSegments: 24`, plus the well, on a recursive intersect at 16 Hz, is already the wrong list. Keep an explicit `pickList` (plaques, proxies, tile bodies, orb proxy) and cast that. `three` r183’s `Raycaster` in this repo has no `firstHitOnly`; the list is the cheap fix.

7. **Reticle `depthTest: false` draws the brackets through the stator and the lid.** visionOS focus is in the world and hides behind nearer opaque surfaces. Turn depth test on, `renderOrder` just above the plaque. The overlay mode is only useful when the box straddles the near plane, which today’s interior preset does on purpose and the next pose should not.

---

## 7. Study 09 against the code

| Study 09 claim | Code |
| :--- | :--- |
| Chrono 0 is oldest, `R1·A`, matching the rings | True. Same reversal as Designer 2 |
| Peak fact(4) is 9 frames ending at `R5·A` RETURN fact(0) | That is `S119`. Peak `S123` is 11 frames and uses `R5·B` and `R6·A` |
| `A` is the Return half, `B` is the Right half | Letter is `chronoIdx % 2`. Kind is only the rim color |
| Orb radius `0.045`, axis origin, target `y = -0.24`, teal | True, and the target volume is already occupied |
| Orb only on RETURN / RESOLVE | Also on any shorter stack: If, Args, FinishArgs |
| Tiles at `z = 2.52`, `y = -0.31`, porcelain / platinum / champagne, state colors | True, and clear of the lid |
| “Call k lives in ring k” for fact(4) | True for the five fact calls. Inspector text promises a stable A/B pair the base case never has |
| `y > 0.15` left clear | True. The `arena` lens still looks at `y = 0.42` |
| Zero per-frame `Color` / `Vector3` / `Matrix4` / `CanvasTexture` in `.update` | Holds in all three files. Designer 4’s trace cache and plaque cache are process-global |
| Both modules import `three/webgpu` | True. Designer 7 does too |

`research_d7.md` camera table does not match `CAMERA_PRESETS`. Update it in the same pass as the poses so the next study does not reintroduce `[0.66, -0.28, 0.68]`.

---

## 8. Next iteration

Do these in order. Each one is independently testable.

### A. Repose Inside Hollow Chip, and stop clipping the stack

In `CAMERA_PRESETS.interior`:

- `pos: [0.00, -1.05, 0.78]`
- `lookAt: [0.00, -1.05, 0.00]`
- `fov: 50` while this preset is current, restored to `34` on any other preset (`updateProjectionMatrix` inside the damp, so the fov eases).
- Subtitle: “Gallery in front of the active wafers.” Delete the rings-and-floor sentence. `well` already shows that.

Horizontal radius `0.78` is inside today’s anti-clip, 0.20 in front of the bevel (`z = 0.583`), 0.14 inside the shallowest wall (`0.92`). Near plane `0.04` clears the lip.

Replace the anti-clip with a keep-out that runs after the spherical placement and writes back into the spherical state so it cannot fight itself:

- If `y < -0.30` and the point is inside the bevel AABB expanded by `0.05` (`|x| < 0.67`, `|z| < 0.63`), push `z` toward the sign of `z` (or `+z` if `z` is ~0) until `|z| = 0.70`, keeping `x` inside `±0.40`.
- Clamp to the square wall with a margin: `|x|` and `|z|` ≤ `0.90` while `y` is between `-0.30` and `-2.00`.
- Do nothing when `hypot(x, z) > 1.15`. The cutaway and the hero orbit must be allowed to dip beside the die.
- Floor stay: `y = max(y, -2.00)`.

While `interior` is current and the user is not dragging, damp `desiredTarget.y` and the camera’s gallery `y` toward the active wafer: `waferSlots[numWafers - 1].currentY`, with `1 - exp(-4 dt)`. A 50° lens at `z = 0.78` sees about two plaques sharply; tracking the active terrace is how the rest of the stack is visited. Do not widen toward 100°.

Add an `apron` preset and a sixth lens-bar button:

- `pos: [0.00, 1.35, 4.55]`
- `lookAt: [0.00, -0.22, 2.15]`
- `fov: 42` while active
- Shrink the row: `spacing = min(0.62, 2.50 / max(1, count - 1))`, so five centers stay inside about `±1.24`. Combined with fov 42 and a ~1.45 aspect, the end tiles land inside the horizontal fov. Today’s `0.72` and the `3.85 / count` cap do not.

Retarget `arena` to that same pose, or delete it from `CAMERA_PRESETS` so nothing still aims at `y = 0.42`.

### B. Turn plaques into windows that face the gallery

Wafers, in the slot group, local transform:

- Plane `0.70 × 0.16` (shorter than today’s `0.92 × 0.21`; the canvas is 640×140 and the title is already sliced at 32 characters).
- `position.set(0, 0.045, 0.60)` so it sits just proud of the bevel’s `z = 0.583`.
- `rotation.x = -0.55` (~32° off vertical, tilted back). Normal has `+Y` and `+Z`, so the cross-section still sees the face and the gallery camera at `z = 0.78` sees it nearly straight on.
- `side: FrontSide`. Do not use `DoubleSide` to paper over a camera on the wrong side of the plate.
- Optional second plate, `0.28 × 0.08`, at local `(0.48, 0.03, 0.20)`, same tilt, text `n = 4` only. That is the lexical capsule from research d3. One binding, not the subtitle repeated.

Call tiles:

- Same tilt, plaque `rotation.x = -0.65`, raised to local `y = 0.06`, shifted to local `z = 0.10` so it addresses a camera on `+Z`.
- Keep the porcelain body horizontal. The window is the only thing that needs to face the lens.
- State stays in the subtitle (`waiting`, `active`, `= 24`) and in the bezel color. With the window facing the apron lens, the subtitle becomes the accessible name, not a ceiling detail.

### C. Glass

Per wafer, keep the shared geometry. Change the shared material, or clone it if a pop must fade one plate:

- `opacity: 1`, `transparent: false`, `transmission: 0.72`, `thickness: 0.02`, `roughness: 0.24`, `ior: 1.5`, `attenuationColor: 0x182232`, `attenuationDistance: 0.35`, `side: FrontSide`.
- If a transmission cost shows up on the 12 extrusions, fall back to `opacity: 0.22`, `depthWrite: true` on bevel and rim only, glass `depthWrite: false`, and `renderOrder = 20 + chronoIdx` on the glass mesh.
- Set `glassMesh.userData.gazeThrough = true` so the raycast in section E can pass through the plate to the plaque.

On first show, set `currentY` to `targetY + 0.06` and `currentScale` to `0.94`, not `y = -0.45` and `0.88`. The plate settles into its terrace. It does not descend through its parents. Damp with `1 - exp(-8 dt)`.

On hide, when the slot was visible and the playhead is inside an unwind step, scale toward `0.25` and lift `currentY` to the parent `y` for that step, then set `visible = false`. Instant `visible = false` is what makes the pop feel like a deletion.

### D. Orb, two beats, one mesh

Gate the orb with an explicit rule line, not `ruleName.includes('Return')` and not “stack got shorter”:

- Fly on **153** (value fills a `Right` / `Left` hole) and **185** (function value leaves a `Return`).
- Also fly on **80** when the outcome is `Done`, from the last plate to the dock.
- Do not fly on 161, 165, 173, 181.

Remember `lastY[chronoIdx]` before the hide loop. Start beat 1 at `lastY[popped]`. End beat 1 at the parent slot’s `currentY` (`popped - 1`), or at `DOCK_Y = -0.30` if there is no parent. Beat 2, only for 185 with no parent `Right` above, or for 80, runs from the parent `y` to `DOCK_Y`. Use `smoothstep` on `stepFraction` only. If `stepFraction <= 0.005`, hold at the start.

Write the returned atom into the parent plaque for the rest of the step (`4 × [?]` becomes `4 × 6`). The rim flash belongs on that parent, for beat 1 only, and it should not replace a violet or gold family for the whole step; add emissive on top, then release.

Reuse one thin cylinder as the spine between start and orb. Park the orb at `y = -0.30` so it kisses the lens from below. In Designer 2, break the north–south bridge into two boxes leaving `|z| < 0.18` empty, or the bore and the dock stay blocked no matter what Designer 3 does.

Pick proxy radius `0.12` as in section 3.

### E. Raycast

- Maintain `pickList`. Cast that, not `scene.children`.
- `params.Line.threshold = 0.015`. Disable raycast on tile rims and on the reticle.
- In `findInspectableTarget`, an untagged hit with opacity ≥ 0.4, `depthWrite !== false`, and no `gazeThrough` returns `null` immediately (occluder). Glass plates are gaze-through. Plaques, collars, tile bodies, and the orb proxy are not.
- Trailing cast from `update()` at 16 Hz using the latest NDC.
- Drop the pin when the target’s `visible` is false or its `id` changes.
- Reticle `depthTest: true`. Frame the plaque, minimum box `0.08 × 0.06 × 0.02`.

### F. Correct the study, do not retune the slots to the study’s shortcut

- Replace study 09 section 3.2’s 9-frame peak with the `S123` 11-frame row from `ring_study_02`. State that `A`/`B` is the ring slot, and the family color is the frame kind.
- Replace the call-tile sentence “wafers `Rk·A` & `Rk·B`” with “ring `Rk` holds this call’s frames while they are on the stack.”
- Put the new interior and apron poses into `research_d7.md` and study 09 so the next pass does not restore the lip camera.

---

## 9. What not to redo

- Chronological reversal. Oldest at the top, `R1·A` stable for the life of the root call.
- Family palette `0x2dd4bf / 0xf59e0b / 0xfbbf24 / 0x818cf8` on rims, collars, and tile bezels.
- Hole radius `0.16` and orb radius `0.045`. The clearance is right; the dock height is wrong.
- Preallocated slots, signature-gated plaque uploads, `three/webgpu` imports, and the absence of per-frame math allocations.
- The eight-corner hairline reticle and the click-versus-drag threshold of 5 px.
- `well` and `frames` as the shaft lenses. They already contain the core, the plates, and the floor at fov 34. Interior and apron are the missing viewpoints, not replacements for those two.
