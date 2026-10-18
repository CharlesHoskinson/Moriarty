# Grok 4.7 xhigh Agent 1 — Interior Camera Physics, Bounded Rotation, Micro-Observer

**Role:** Interior camera physics, bounded rotation, and micro-observer auditor.
**Controller:** `src/designers/designer7_cinema.js`
**Stage coupling:** `src/stage/SherlockStage.js`
**Live check:** `http://127.0.0.1:4377` at step 53, preset `interior`, plus the scratch captures named below.

The interior lens is a third-person orbit of radius **2.012** around a look-at parked at the mouth of the bore. Dragging rotates that sphere, so the eye translates. The hollow shaft is only `r ≤ 0.64` wide. The sphere is 2 units wide. Those two shapes meet only in a thin cap at the bottom of the sphere, and every gaze in that cap points **up**. A downward look exists on the same sphere only after the eye has already left the chip.

---

## 1. What the pictures show

| Capture | Gesture | What the frame actually contains |
| :--- | :--- | :--- |
| `scratch/clean_v3_interior.png` | Preset, step 53, full UI | Up the open bore. Six split-arc rings and the continuation annuli fill a 56° frame. The 39-rule floor is outside the frame. Caption still says “Drag to orbit”. |
| `scratch/grok_test_01_interior_default.png` | Same pose, canvas only | Same upward tunnel. Floor modules are not in view. |
| `scratch/grok_test_02_rotate_left.png` | Horizontal drag | Bore stays centered. The white radial bridge swings. Gaze is still locked on the same ceiling point. |
| `scratch/grok_test_03_drag_up_to_see_floor.png` | Drag up, meant to find the floor | A tighter, more axial version of the same upward tunnel. The floor never enters. |
| `scratch/grok_test_04_drag_down.png` | Drag down | Eye has left the shaft. Beige clear-color (`0xf4f0e8`) over a black deck edge and a sliver of rings. |
| `scratch/grok_test_05_wheel_zoom_out.png` | Wheel out | Outside the package. Cutaway gallery, continuation stack seen from the flank, four gold call-chiplet cubes. |

Agent 1 re-drove the same preset on the live canvas (`#sherlockCanvas`, step scrubber 53, playback paused) by dispatching `pointer*` and `wheel` events at the canvas center:

| Agent 1 frame | Input | Predicted eye (section 2) | Observed |
| :--- | :--- | :--- | :--- |
| `a1_01_default` | `interior` preset | `(0, -1.95, 0.52)`, gaze pitch **+73.5°** | Inside the bore, looking up through wafers at the stator. No floor. |
| `a1_02_rotate_left` | drag `dx = -200` | `Δθ = +1.36 rad` (78°) at constant φ | Still inside. Bridge and sector dial rotate around a fixed aim point. |
| `a1_03_drag_up` | drag `dy = -240` | φ hits the clamp after **28 px**. Eye drops to `y = -2.017`, `r ≈ 0.20`, gaze pitch rises to **+82.8°** and stays aimed at `y = -0.02` | Slightly deeper, more centered tunnel. Still no floor. Further drag is eaten by the clamp. |
| `a1_04_drag_down_90` | drag `dy = +90` | φ = 133.6°, eye `(0, -1.41, ~1.41)`, **outside** `innerHalf ≈ 1.22` | Wafer stack seen from the cutaway void. Beige background in the corners. |
| `a1_05_drag_down_260` | drag `dy = +260` | φ = 77.1°, eye `y = +0.43`, `r ≈ 1.91` | Above the deck, looking down at the ring tops. Studio gray surrounds the package. |
| `a1_06_wheel_out` | `deltaY = +700` | radius `2.012 → 4.35` | Black frame. The eye has left the lit shaft and is buried against solid or an unlit backface. |
| `a1_07_wheel_in` | `deltaY = -500` | radius shrinks; dive eases `lookAt.y` down | Same upward tunnel, slightly tighter. Scroll never aims the lens at the floor. |
| `a1_08_drag_down_right` | `dx = +140`, `dy = +180` | eye on the equator side of the sphere | A diagonal of beige void against a dark mass. The observer is outside. |

Horizontal orbit at the rest pose stays visually inside. Vertical drag and the wheel do not. “Look up” has about nine degrees of unused polar angle and then repeats the same ceiling. “Look down” is implemented as “walk the eye up the outside of a 2-unit sphere.”

---

## 2. Diagnosis — why `radius = 2.01` swings the eye through the walls

### 2.1 The map the controller actually evaluates

`CAMERA_PRESETS.interior` (`designer7_cinema.js` lines 23–29):

```text
pos    = (0.00, -1.95,  0.52)
lookAt = (0.00, -0.02, -0.05)
fov    = 56°
```

`setCameraPreset` converts `pos - lookAt` with `Spherical.setFromVector3` (lines 286–294). `update` writes the camera every frame (lines 619–644):

```text
offset = radius * (sin φ · sin θ,  cos φ,  sin φ · cos θ)
camera.position = target + offset
camera.position.y += arcLiftY
camera.lookAt(target)
```

That is `THREE.Vector3.setFromSphericalCoords(radius, phi, theta)`: φ is the polar angle from **+Y**, θ is `atan2(x, z)`.

Rest numbers, from those two vectors:

```text
offset        = (0, -1.93, 0.57)
radius        = √(1.93² + 0.57²) = √4.049800 = 2.012411
θ             = atan2(0, 0.57) = 0
φ             = acos(-1.93 / 2.012411) = acos(-0.959047) = 2.854420 rad = 163.546°
eye xz radius = hypot(0, 0.52) = 0.520
gaze pitch    = atan2(-cos φ, sin φ) = atan2(0.95905, 0.28335) = +73.55°
```

The aim point sits **1.93** above the eye, at the bore mouth. The eye is a point on the sphere of radius 2.012 centered there. `getMaxPhiForState` (lines 264–270) returns `π · 0.96 = 3.015929 rad = 172.800°` whenever the preset is `interior`, so the polar angle may travel almost pole to pole. The drag law (lines 459–462) is:

```text
desiredTheta -= dx * 0.0068
desiredPhi    = clamp(desiredPhi - dy * 0.0058,  0.002,  π·0.96)
```

Wheel (lines 535–539) multiplies radius:

```text
desiredRadius = clamp(desiredRadius * exp(deltaY * 0.0011),  0.45,  13.5)
```

`exp(0.11) = 1.116` per 100 px of `deltaY`. A 700 px wheel-out grows radius to **4.346**.

### 2.2 The volume the eye is supposed to stay in

Measured from the designers the camera is looking at:

| Volume | Bound | Source |
| :--- | :--- | :--- |
| Walkable shaft requested for the micro-observer | `hypot(x, z) ≤ 0.55`, `-3.05 ≤ y ≤ -0.45` | This audit’s contract |
| Continuation-wafer hole | `r < WAFER_INNER_RADIUS = 0.64` (collar starts at 0.64, glass from 0.68, outer bevel 0.96) | `designer3_frames.js` lines 50–59. The comment at line 261 still says `0.36..0.74`; the constants are the geometry. |
| Wafer stack | `y = -0.64 - 0.19·chrono`, clamped to `[-2.85, -0.64]` | `designer3_frames.js` lines 657–673 |
| 6 split-arc rings | `y` from `0.06` (R1, `outerR = 0.95`) down to `-0.21` (R6, `outerR = 0.31`) | `designer2_well.js` lines 592–597, 1487–1493 |
| Stator gate the upward sightline must thread | `y = -0.24`, open bore `r < 0.135`, metal `r ∈ [0.142, 0.216]` | lines 605–610 |
| Vault soffit | `y = -0.34`, hole `innerR = 0.955` | lines 613–614, 1929–1953 |
| Stepped chamber walls | `innerHalf` from `1.32` (D0) to `1.22` (D7), `y ∈ [-3.35, -0.35]` | `getTerraceInnerHalf`, lines 613–618, 686–688 |
| 39-rule floor | tile centers `y = -3.376`; grid `x ∈ {-0.925 … 0.925}` step `0.37`, `z ∈ {-0.96 … 0.96}` step `0.32`; cell `0.322 × 0.272` | lines 2077–2095, 2133–2140 |
| South wall while this preset is active | retracted by `cutawayMix → 1`, `southRetractY = -cutawayMix * 3.15` | `SherlockStage.js` lines 285–286; `designer2_well.js` line 2607 |

`SherlockStage` builds the camera at `near = 0.025` (line 85). Cinema only tightens `near` when it is **greater** than 0.04 (cinema lines 254–256), so the interior near plane stays **0.025**. Nothing in the stage stops a camera from translating through the open south cutaway. The only position clamp in cinema (lines 627–642) is explicitly skipped for `interior` and `frames`:

```text
if (preset !== 'interior' && preset !== 'frames' && target.y < -0.60 && eye.y < -0.22 && eye.z < 0.75)
    pull horiz radius back to 0.86 and keep y ≥ -3.22
```

Interior is the preset that is allowed to leave.

### 2.3 How many pixels until the eye is outside

At rest, `R sin φ = 0.570` and the look-at is 0.05 off the axis, so a pure θ sweep already moves the eye’s ground radius through:

```text
hypot(eye.x, eye.z) ∈ [0.520, 0.620]
```

The far side of a full turn (`θ = π`) is outside the 0.55 contract and still inside the 0.64 wafer hole. The pictures of a horizontal drag stay in the shaft for that reason.

Vertical drag decreases φ (`dy > 0`). Horizontal offset of the eye is `R sin φ` plus the 0.05 target bias. Exit distances from the rest φ:

| Threshold | φ at the wall | Eye `(x, y, z)` at θ = 0 | Drag down |
| :--- | ---: | :--- | ---: |
| Contract `r = 0.55` | 162.65° | `(0, -1.941, 0.550)` | **2.7 px** |
| Wafer bore `r = 0.64` | 159.95° | `(0, -1.910, 0.640)` | **10.8 px** |
| Bench well `r = 1.06` | 146.52° | `(0, -1.699, 1.060)` | 51 px |
| Chamber wall `r = 1.22` | 140.87° | `(0, -1.581, 1.220)` | 68 px |
| 90 px (live frame `a1_04`) | 133.64° | `y = -1.409`, `r = 1.406` | outside the vault |
| 260 px (live frame `a1_05`) | 77.14° | `y = +0.428`, `r = 1.912` | above the deck |

Three pixels of “look down” crosses `r = 0.55`. Eleven pixels crosses the sapphire hole and the eye is inside the wafer annulus. The south wall is retracted in this preset, so the eye does not bump into mesh; it flies into the cutaway void. That is `grok_test_04`, `a1_04`, and `a1_08`.

Drag up increases φ and dies at `172.800°`:

```text
headroom = 172.800° − 163.546° = 9.254° = 0.16151 rad = 27.8 px
eye at the clamp: y = -0.02 + 2.012411·cos(172.8°) = -2.017
                  horiz offset = 2.012411·sin(172.8°) = 0.252
                  gaze pitch = +82.8°, still aimed at y = -0.02
```

Past 28 px the picture barely changes (`a1_03`, `grok_test_03`). The lens was already pitched +73° at the rings. The extra nine degrees only sinks the eye and aims more steeply at the **same** point, so the outer rings crop away and nothing new appears. That is “when I look up I don’t get to see much.”

### 2.4 Why the floor is unreachable without leaving

Gaze pitch on this controller is not an input. It is a function of φ only:

```text
pitch(φ) = atan2(-cos φ, sin φ) = π/2 − φ     for φ ∈ (0, π)
```

- `pitch < 0` (any look toward the floor) ⇔ `φ < π/2`.
- At `φ = π/2` the offset is horizontal and has length `radius = 2.012`. The eye is at `y ≈ -0.02`, `r ≈ 2.01`, which is outside the package well.
- The sample points on the sphere that do satisfy `hypot(x, z) ≤ 0.55` and `y ∈ [-3.05, -0.45]` sit in the south cap, `φ ≳ 163°`, where `pitch(φ) ≳ +73°`.

So every pose that looks downward is outside the shaft, and every pose that is inside the shaft looks up at a point near `y = -0.02`. The 39-rule floor at `y = -3.38` is behind the lens for the entire legal cap. Tightening `maxPhi` would only freeze the user in that upward stare. The polar clamp is the wrong degree of freedom.

The wheel has the same shape. Zoom-out scales the sphere (`grok_test_05`, black frame at radius 4.35). Zoom-in, when `hypot(target.x, target.z) < 0.65` and radius drops under 2.1 (lines 543–547), slides `target.y` toward `-2.15`. That moves the center of the sphere; it does not pitch the lens down, which is why `a1_07` is still an upward tunnel.

`transitionArcBoost` (lines 302–305, 609–610, 625) adds up to `+0.28` on `camera.position.y` during a preset change. Harmless at the rest eye `y = -1.95`. Fatal if the eye is ever allowed up near `y = -0.45`: `-0.45 + 0.28 = -0.17`, which is above the soffit (`y = -0.34`) and inside the stator band (`y = -0.24`).

---

## 3. Specification — first-person micro-gimbal when `currentPresetName === 'interior'`

Replace the spherical write with an eye anchored in the shaft and a look direction driven by yaw and pitch. Other presets keep the existing orbit, including `frames`.

### 3.1 Contract

```text
INTERIOR_EYE_HOME     = (0.00, -1.85, 0.28)
INTERIOR_PITCH_MIN    = -82° = -1.431170 rad     look down at the 39-rule floor
INTERIOR_PITCH_MAX    = +85° = +1.483530 rad     look up at the 6 rings
INTERIOR_YAW          ∈ ℝ, continuous, period 2π  no clamp
INTERIOR_WHEEL_Y      ∈ [-2.90, -0.55]           scroll band
INTERIOR_HARD         hypot(x, z) ≤ 0.55
                      and y ∈ [-3.05, -0.45]      safety retraction
INTERIOR_LOOK_SENS    = 0.0075 rad/px
INTERIOR_WHEEL_GAIN   = 0.00190                   100 px ≈ one wafer spacing (0.19)
```

Seed the preset from the table so it stays the source of truth. Recommended row:

```javascript
interior: {
  name: 'interior',
  label: 'Inside Hollow Chip (Looking Up at Rings)',
  subtitle: 'Micro-observer inside the open bore. Drag looks around. Scroll rises and descends.',
  pos: [0.0, -1.85, 0.28],       // eye. Not an orbit point.
  lookAt: [0.0, -0.02, -0.05],   // used only to seed yaw and pitch
  fov: 56,
},
```

Direction from the home eye to that look-at:

```text
dir   = (-0.00, 1.83, -0.33)
yaw   = atan2(0, -0.33) = π
pitch = atan2(1.83, 0.33) = +79.78°
```

That seed reproduces today’s hero picture (up the bore at the rings) from an eye that already satisfies the contract: `hypot(0, 0.28) = 0.28 ≤ 0.55`, `y = -1.85 ∈ [-2.90, -0.55]`.

`+85°` from `z = 0.28` meets the stator plane `y = -0.24` at radial distance **0.139**, which is 0.004 outside the `r < 0.135` hole and on the inner lip. The useful “straight up the rings” pose is the seeded **+79.8°**, which aims at the existing look-at and threads the hole. The `+85°` stop is the safety rail before `lookAt` goes singular (`sin 85° = 0.9962`, `cos 85° = 0.0872`, so the camera basis stays well-conditioned). Leave the home eye at `z = 0.28`. If the stop itself must show bore rather than lip, set the home `z` to **0.241**: the same `+85°` ray then hits the stator plane at `r = 0.10`.

### 3.2 State and scratch (no per-frame allocation)

Next to the existing module scratch vectors:

```javascript
const _lookDir = new THREE.Vector3();
const _lookPoint = new THREE.Vector3();

const INTERIOR_PITCH_MIN = -82 * Math.PI / 180;
const INTERIOR_PITCH_MAX =  85 * Math.PI / 180;
const INTERIOR_HARD_RADIUS = 0.55;
const INTERIOR_HARD_Y_MIN = -3.05;
const INTERIOR_HARD_Y_MAX = -0.45;
const INTERIOR_WHEEL_Y_MIN = -2.90;
const INTERIOR_WHEEL_Y_MAX = -0.55;
const INTERIOR_LOOK_SENS = 0.0075;
const INTERIOR_WHEEL_GAIN = 0.0019;
```

Inside `createCinemaSubsystem`, beside the spherical channels:

```javascript
const interiorEye = new THREE.Vector3(0, -1.85, 0.28);
const desiredInteriorEye = new THREE.Vector3(0, -1.85, 0.28);
let interiorYaw = Math.PI;
let interiorPitch = 79.78 * Math.PI / 180;
let desiredInteriorYaw = interiorYaw;
let desiredInteriorPitch = interiorPitch;

function clampInteriorEye(v) {
  const r = Math.hypot(v.x, v.z);
  if (r > INTERIOR_HARD_RADIUS) {
    const s = INTERIOR_HARD_RADIUS / r;
    v.x *= s;
    v.z *= s;
  }
  v.y = clamp(v.y, INTERIOR_HARD_Y_MIN, INTERIOR_HARD_Y_MAX);
  return v;
}
```

The hard set is a closed convex cylinder (disk × interval). A lerp of two points in it stays in it, so the damped eye cannot tunnel out between the desired pose and the displayed pose.

### 3.3 Entering and leaving the mode

In `setCameraPreset`, branch **before** the spherical conversion when `key === 'interior'`:

```javascript
if (key === 'interior') {
  desiredFov = preset.fov || 56;
  desiredInteriorEye.set(preset.pos[0], preset.pos[1], preset.pos[2]);
  clampInteriorEye(desiredInteriorEye);
  if (prevPresetName !== 'interior') {
    // Arrive already inside. Do not lerp the eye in from the overview orbit;
    // that path crosses the roof. Yaw/pitch still damp.
    interiorEye.copy(desiredInteriorEye);
  }
  _lookDir.set(preset.lookAt[0], preset.lookAt[1], preset.lookAt[2]).sub(desiredInteriorEye);
  const horiz = Math.hypot(_lookDir.x, _lookDir.z) || 1e-6;
  desiredInteriorPitch = clamp(Math.atan2(_lookDir.y, horiz), INTERIOR_PITCH_MIN, INTERIOR_PITCH_MAX);
  const yaw = Math.atan2(_lookDir.x, _lookDir.z);
  let dYaw = yaw - interiorYaw;
  while (dYaw > Math.PI) dYaw -= Math.PI * 2;
  while (dYaw < -Math.PI) dYaw += Math.PI * 2;
  desiredInteriorYaw = interiorYaw + dYaw;
  transitionArcBoost = 0; // a +0.28 y lift exits the upper shaft
  return preset;
}
```

On the way **out** (`prevPresetName === 'interior' && key !== 'interior'`), re-seed the spherical channels from the gimbal eye so the exterior lerp starts where the observer actually is:

```javascript
_scratchOffset.copy(interiorEye).sub(desiredTarget);
_scratchSpherical.setFromVector3(_scratchOffset);
currentRadius = _scratchSpherical.radius;
currentPhi = _scratchSpherical.phi;
currentTheta = _scratchSpherical.theta;
```

Then run the existing desired-radius / desired-φ / desired-θ setup for the destination preset. `transitionArcBoost = 1` is fine on that exit; the destination is an exterior orbit.

### 3.4 Pointer and wheel

In the orbit half of `onPointerMove` (the `else` under `isPanning`, lines 459–463):

```javascript
if (currentPresetName === 'interior') {
  // dx > 0 (drag right) turns the observer's head to their right.
  // dy > 0 (drag down) pitches the gaze down toward the floor.
  desiredInteriorYaw -= dx * INTERIOR_LOOK_SENS;
  desiredInteriorPitch = clamp(
    desiredInteriorPitch - dy * INTERIOR_LOOK_SENS,
    INTERIOR_PITCH_MIN,
    INTERIOR_PITCH_MAX
  );
} else {
  const maxPhi = getMaxPhiForState(desiredRadius, desiredTarget);
  desiredTheta -= dx * 0.0068;
  desiredPhi = clamp(desiredPhi - dy * 0.0058, MIN_PHI, maxPhi);
}
```

Look basis, yaw `0` = world **+Z**, pitch `0` = horizontal, pitch positive = up:

```text
forward = (cos(pitch) · sin(yaw),  sin(pitch),  cos(pitch) · cos(yaw))
```

At the seeded `(yaw, pitch) = (π, +79.8°)`, `forward` points toward **-Z** and up, through the stator. Decreasing yaw from `π` turns the head toward world **+X**. With Three.js `lookAt` (camera local **−Z** aims along `forward`, local **+Y** toward world up), world **+X** is the observer’s right at that seed, so `desiredYaw -= dx * sens` is “drag right, look right.”

Full travel from the seed pitch `+79.8°` down to `-82°` is `2.824 rad / 0.0075 = 377 px`, one stroke on the ~610 px canvas. A full yaw turn is `2π / 0.0075 = 838 px`. Yaw is not clamped; wrap only when differencing for the damper, the same way θ is unwrapped today (lines 296–300).

Pan (`shift` / middle / right button) currently adds to `desiredTarget` (lines 453–458), which would slide the old sphere’s center. While interior, pan the **eye** in the horizontal camera basis and retract onto the cylinder:

```javascript
if (currentPresetName === 'interior') {
  const panScale = 0.0016;
  const cY = Math.cos(desiredInteriorYaw);
  const sY = Math.sin(desiredInteriorYaw);
  // right = (cos yaw, 0, -sin yaw), forward flattened = (sin yaw, 0, cos yaw)
  desiredInteriorEye.x += (-dx * cY + dy * sY) * panScale;
  desiredInteriorEye.z += ( dx * sY + dy * cY) * panScale;
  clampInteriorEye(desiredInteriorEye);
}
```

`onWheel`: do not touch `desiredRadius` or `desiredTarget.y` in this preset.

```javascript
if (currentPresetName === 'interior') {
  event.preventDefault();
  // deltaY > 0 (scroll down) descends toward the floor.
  desiredInteriorEye.y = clamp(
    desiredInteriorEye.y - event.deltaY * INTERIOR_WHEEL_GAIN,
    INTERIOR_WHEEL_Y_MIN,
    INTERIOR_WHEEL_Y_MAX
  );
  return;
}
```

One 100 px detent moves the eye by **0.190**, which is the continuation-wafer spacing. From the home eye, the stack is about 7 detents up to `y = -0.55` and 6 detents down to `y = -2.90`. The wheel band sits strictly inside the hard interval, 0.10 below the soffit side of the hard cap and 0.15 above the floor side.

### 3.5 Per-frame write

In `update`, skip the spherical block (lines 603–644) when the preset is interior. Do not add `arcLiftY`.

```javascript
if (currentPresetName === 'interior') {
  const damp = 1.0 - Math.exp(-11.0 * dt);
  interiorEye.lerp(desiredInteriorEye, damp);
  clampInteriorEye(interiorEye);

  let dYaw = desiredInteriorYaw - interiorYaw;
  while (dYaw > Math.PI) dYaw -= Math.PI * 2;
  while (dYaw < -Math.PI) dYaw += Math.PI * 2;
  interiorYaw += dYaw * damp;
  interiorPitch = lerp(interiorPitch, desiredInteriorPitch, damp);

  const cp = Math.cos(interiorPitch);
  const sp = Math.sin(interiorPitch);
  _lookDir.set(cp * Math.sin(interiorYaw), sp, cp * Math.cos(interiorYaw));
  _lookPoint.copy(interiorEye).add(_lookDir);

  camera.position.copy(interiorEye);
  camera.up.set(0, 1, 0);
  camera.lookAt(_lookPoint);

  if (typeof camera.fov === 'number' && Math.abs(camera.fov - desiredFov) > 0.05) {
    camera.fov = lerp(camera.fov, desiredFov, damp);
    camera.updateProjectionMatrix();
  }
} else {
  // existing radius / phi / theta / lookAt path, unchanged
}
```

`|forward × up| = cos(pitch)`. At the stops that length is `cos 82° = 0.1392` and `cos 85° = 0.0872`. Both are far from the singularity at ±90°, where `lookAt` drops its basis. Do not widen the pitch clamp to 90°.

Keep fov **56** and `near = 0.025`. At the hard eye, the nearest wafer collar is `0.64 − 0.55 = 0.09` away and the floor top (`y ≈ -3.366`) is `0.316` below `y = -3.05`. Neither meets the near plane.

### 3.6 What each pitch sees from the home eye `(0, -1.85, 0.28)`

Vertical fov is 56°, so a frame covers ±28° around the aim.

| Gaze | Pitch | Lands on |
| :--- | ---: | :--- |
| Seed, today’s hero | **+79.8°** at yaw `π` | Stator bore `(0, -0.24, 0)` and, through it, the six rings (`R1` at `y = 0.06`, `R6` at `y = -0.21`). |
| Ring R1 underside, outer sample `(0, 0.06, -0.90)` | **+58.3°** | Clears the soffit hole (`r = 0.65 < 0.955` at `y = -0.34`). Outer rings are a shallower look up, not a translation. |
| Across the shaft | **0°** | Inner collar of the wafers that share this height. Home `y = -1.85` sits between chrono 6 (`y = -1.78`) and chrono 7 (`y = -1.97`). Scroll one detent and the next annulus swings to eye level. |
| Nearest floor cells `(-0.185, -3.376, 0.32)` and `(0.185, -3.376, 0.32)` | **−82.9°** to the cell center | The −82° stop ray travels `1.522 · cot 82° = 0.214` horizontally and crosses those cells (center distance `0.189`, half-extent `0.161 × 0.136`). |
| Far corner cell `(0.925, -3.376, 0.96)` | **−53.0°** | Inside the same −82° frame’s upper edge (−82° + 28° = −54°). Scroll down to about `y = -2.40` and aim near −60° to hold the whole 6×7 grid with margin. |

The raycaster already shoots along `camera` (line 396) and ignores hits while `isDragging`. Once the look direction can pitch onto a floor tile or a wafer collar, the existing reticle and click-to-pin path start hitting those meshes. No raycaster change is required for that.

---

## 4. Why rotation can no longer leave the chip

While the preset is `interior`, a drag writes `desiredInteriorYaw` and `desiredInteriorPitch` and does not write `interiorEye`. The per-frame map is

```text
camera.position = e
forward         = (cos pitch · sin yaw,  sin pitch,  cos pitch · cos yaw)
camera.lookAt(e + forward)
```

with `pitch` clamped to `[-82°, +85°]` and `yaw` free on the circle. The partial derivatives of `e` with respect to yaw and with respect to pitch are the zero vector. A rotation is an element of SO(3) acting on `forward`. It is not a translation. If `e` satisfies

```text
C(e) = hypot(e.x, e.z) ≤ 0.55  ∧  -3.05 ≤ e.y ≤ -0.45
```

before the drag, it satisfies `C` after every drag, at every yaw, including a full turn, and at both pitch stops.

The only writers of `e` are the preset seed, the wheel, and the optional pan. Each one finishes through a retraction onto `C`:

- The seed is `(0, -1.85, 0.28)`, and `clampInteriorEye` is applied anyway.
- The wheel writes `y` inside `[-2.90, -0.55] ⊂ [-3.05, -0.45]` and does not change `x` or `z`.
- Pan adds a horizontal offset and then scales `(x, z)` back onto the disk of radius 0.55 and clamps `y` to the hard interval.

`C` is convex, and the displayed eye is `lerp(e, e_desired, damp)`. A convex set is closed under lerp, so the in-between frames of the damper stay in `C` too. `arcLiftY` is forced to 0 in this mode, so the old `+0.28` preset pop cannot lift the eye through the soffit.

Contrast with the sphere the code uses today: `e(φ, θ) = target + R · û(φ, θ)` has nonzero `∂e/∂φ` and `∂e/∂θ`. Every pixel of drag is a step along the tangent of a sphere of radius 2.012 centered at `y = -0.02`. That sphere intersects `C` only in the south cap, and on that cap the implied gaze pitch is greater than `+73°`. The gimbal deletes those partials.

Geometry will not save the old controller. `SherlockStage` sets `cutawayMix = 1` for `interior`, and Designer 2 drops the south wall by `3.15`. The shaft’s south side is intentionally open so the observer can **look** out across the continuation wafers. It must not be a door the eye can walk through. Keeping `e` inside `C` is what makes the open cutaway safe.

---

## 5. Experience recommendations, in order

1. **Ship the gimbal above.** It is the whole bug. A tighter `maxPhi`, and turning the existing `horizR ≤ 0.86` clamp on for `interior`, both leave the gaze locked on `y ≈ -0.02`. The 0.86 cylinder is also wider than the wafer hole (`0.64`), so the eye would still live inside the sapphire.

2. **Keep the arrival instantaneous and inside `C`.** Damping yaw and pitch is enough choreography. Lerping the eye from the overview point `(3.85, 4.85, 5.35)` toward `(0, -1.85, 0.28)` tracks a straight line through the roof. The parabolic arc-lift was built for exterior-to-exterior swoops; zero it on the way in.

3. **Three poses, one cylinder.** Pitch `+80°` is the six rings. Pitch `0°` plus the wheel is an elevator past the twelve continuation wafers, one 100 px detent per wafer. Pitch `−82°` reads the floor modules under the shaft; `y ≈ -2.40` and pitch `≈ -60°` frames the full 6×7 grid. The user should not have to leave the chip to get any of the three.

4. **Leave fov at 56° and near at 0.025.** At the pitch stop the vertical frame runs from about `−110°` (into the floor) to `−54°` (the far corner cells). That is the first interior frame in which the floor modules are readable.

5. **Change the caption while this preset is active.** The bar under the canvas says “Drag to orbit · Scroll or pinch to zoom” (`index.html` stage caption). For `interior` it should say “Drag to look · Scroll to rise and descend”. The orbit sentence is what teaches the gesture that ejects the eye.

6. **Do not move `desiredTarget` on the wheel during `interior`.** The dive at lines 543–551 is an overview gesture. Gating it avoids a stale target when the user later switches back to Overview.

7. **Stale radius comment.** `designer3_frames.js` line 261 still describes the wafers as `innerRadius = 0.36`. The mesh is `WAFER_INNER_RADIUS = 0.64`. The 0.55 eye cap is correct against **0.64** (0.09 of air). It would be needlessly tight against 0.36, and it would be illegal against a misreading that sent the eye out to the chamber wall at `innerHalf ≈ 1.22`.

---

## 6. Acceptance checks

After the patch, with the preset on `interior` and the playhead anywhere:

- At every yaw in `[0, 2π)` and both pitch stops, `hypot(camera.position.x, camera.position.z) ≤ 0.55` and `camera.position.y ∈ [-3.05, -0.45]`. A 400 px vertical drag and a 900 px horizontal drag must not change those inequalities.
- Pitch at the lower stop shows rule-floor tiles. Pitch at the seed (`≈ +80°`, yaw `π`) shows the six rings through the stator, matching `clean_v3_interior.png` without the orbit.
- Pitch near `0` shows a wafer inner collar. Ten wheel detents downward walk `y` from `-0.55` to `-2.90` and never change yaw, pitch, or `hypot(x, z)`.
- `deltaY = +700` must not turn the frame black and must not reveal the gold call chiplets. Those cubes are outside the shaft (`a1_06`, `grok_test_05`).
- Switching to Overview restores the spherical orbit. Switching back to Inside Hollow Chip snaps the eye to `(0, -1.85, 0.28)` and damps the gaze up the bore, with `transitionArcBoost === 0`.
- `frames`, `well`, `cinema`, and the other exterior presets still use `setFromSphericalCoords`.
