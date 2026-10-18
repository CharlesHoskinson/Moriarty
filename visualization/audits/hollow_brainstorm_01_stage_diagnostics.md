# Hollow Chip Brainstorm 01: Stage-by-Stage Screenshot Diagnostic & Spatial Frustum Blueprint

## 1. Visual Diagnosis of the 4 Camera Stages (Step 2 & Step 53 Screenshots)

| Stage Button | Screenshot(s) | Current `CAMERA_PRESETS` (`designer7_cinema.js`) | Spherical / Frustum State | Visual Diagnosis |
| :--- | :--- | :--- | :--- | :--- |
| **1. `Overview`** | `clean_v1_initial.png` (Step 2)<br>`clean_v1_overview_step52.png` (Step 53) | `pos: [3.85, 4.85, 5.35]`<br>`lookAt: [-0.38, -0.16, 0.05]`<br>`fov: 32°` | `R = 8.211`<br>`φ = 52.4°` (from `+Y`)<br>`θ = 40.3°` | **Exterior Strong, Interior Pinched & Bleeding:** The 3-tier porcelain/silver/obsidian package (`±2.15`), West Call Chiplets (`fact(4)..fact(0)`), and 6 split-arc rings (`R1..R6`) frame cleanly. However: (a) the central stator bore (`r = 0.128` at `y = -0.24`) foreshortens at `φ = 52.4°` into a tiny `0.156`-unit slit (`< 18px` tall), hiding the hollow chamber; and (b) cropped fragments of the top wafer plaque (`"B1"`, `"D1"`) bleed through the South `14.4°` split-arc bridge gap (`z ∈ [0.22, 0.95]`). |
| **2. `Inside Hollow Chip`** | `clean_v1_interior.png` (Step 53) | `pos: [0.52, -0.52, 0.56]`<br>`lookAt: [-0.06, -1.22, -0.06]`<br>`fov: 48°` | `R = 1.100`<br>`φ = 50.5°`<br>`θ = 43.1°` | **Catastrophic Wafer-Stack Collision:** The viewport is 100% occluded at point-blank range (`d = 0.00..0.28` units from the lens) by the solid top surfaces and front plaques of Wafer 0 (`R1·A`: `"RETURN fact(4) · n = 4"`, `"env#4"`) and Wafer 1 (`R1·B`: `"D3 · env#3"`), sheared at a `~43°` diagonal roll across the screen. The 3D atrium walls (`D0..D7`), vertical wafer stack separation, central Unwind Orb channel (`x = 0, z = 0`), and 39-rule floor (`y = -2.18`) are completely invisible. |
| **3. `Top-Down Rings`** | `clean_v1_well.png` (Step 53) | `pos: [0.0, 3.85, 0.52]`<br>`lookAt: [0.0, -0.14, 0.0]`<br>`fov: 32°` | `R = 4.024`<br>`φ = 7.4°` (off-zenith)<br>`θ = 0.0°` | **Severe Multi-Layer Typography Confetti Bleed:** Looking down at `φ = 7.4°`, every `0.036` radial inter-ring gap (`R1..R6`), both `14.4°` North/South bridge gaps, and the `r = 0.128` central stator hole expose sliced, overlapping text strips from the 5 wafer plaques (`"R1·A"`, `"RN entry"`, `"n"`, `"env (RET"`, `"1 · en"`) and the 39-rule floor (`"BindLet"` dead-center inside the stator ring). |
| **4. `Cross-Section`** | `clean_v1_frames.png` (Step 53) | `pos: [1.95, 2.15, 2.55]`<br>`lookAt: [-0.18, -0.48, 0.04]`<br>`fov: 34°` | `R = 4.212`<br>`φ = 51.4°` (`38.6°` elev)<br>`θ = 40.3°` | **Zero Cross-Section / 100% Side-Wall Occlusion:** No geometry is cut away. Because the 3-tier outer package (`y = -0.35..+0.108`), silver well collar (`r = 1.06`), 6 concentric rings (`y = -0.21..+0.06`), and 4-sided chamber walls (`y = -2.15..-0.35`) are completely solid on the camera-facing `+X / +Z` quadrant, the vertical stack of 5 continuation wafers (`y = -0.52..-1.18`) is 100% hidden behind the rim. |

---

## 2. Exact Coordinate Collision Math: Why `clean_v1_interior.png` (`Inside Hollow Chip`) Fails

At Step 53 (`5` saved frames: `R1·A`, `R1·B`, `R2·A`, `R2·B`, `R3·A`), `designer3_frames.js` constructs solid horizontal wafers inside `y ∈ [-0.52, -1.92]`:
- **Horizontal Footprint:** Each wafer spans `WAFER_OUTER_W = 1.24` (`x ∈ [-0.62, +0.62]`), `WAFER_OUTER_D = 1.16` (`z ∈ [-0.58, +0.58]`), diagonal bounding radius `r = 0.85..0.88`, with a tiny central hole of only `SOCKET_RADIUS = 0.16`.
- **Upward-Facing Front Plaque:** Each wafer mounts a `0.92 × 0.21` plaque (`waferPlaqueGeo`, rotated `-0.46π = -82.8°` around X and translated to `(0, +0.016, +0.35)`).
- **Camera Position (`pos = [0.52, -0.52, 0.56]`):**
  - $y_{\text{cam}} = -0.520$ is strictly coplanar with Wafer 0 ($y_0 = -0.520$) at $(x, z) = (0.52, 0.56) \in [-0.62, 0.62] \times [-0.58, 0.58]$. The lens is embedded directly inside Wafer 0's outer rhodium bevel at $d = 0.00$, hits Wafer 1 ($y = -0.685$) at $d = 0.260$, and hits Wafer 2 ($y = -0.850$) at $d = 0.519$.

---

## 3. Exact Spatial Coordinates, Geometry & Frustum Math to Fix All 4 Stages

1. **Wafer & Plaque Geometry Discipline (`designer3_frames.js`)**:
   - Open the central wafer bore into an **Annular Ring Wafer (`innerRadius = 0.34, outerRadius = 0.72`)** so the central vertical light column and Unwind Result Orb are 100% unobstructed.
   - Replace the giant `0.92 × 0.21` upward-facing text slab with a compact front-angled rim badge (`0.54 × 0.125`) and hide/fade wafer plaques when viewing from `Top-Down Rings` (`well`).
2. **Optical Baffle / Stage-Aware Visibility Gating (`designer2_well.js` & `designer3_frames.js`)**:
   - Eliminate `Top-Down Rings` (`well`) bleed-through by fading wafer plaques when polar angle $\phi < 16^\circ$ or `cameraPreset === 'well'`.
3. **Architectural Front-South Cutaway (`designer1_package.js` & `designer2_well.js`)**:
   - In `interior` (`Inside Hollow Chip`) and `frames` (`Cross-Section`) modes, smoothly open the front-south cutaway bay of the subterranean chamber and outer package skirt so the camera has a completely unobstructed view of the entire vertical stack inside the hollow chip.
