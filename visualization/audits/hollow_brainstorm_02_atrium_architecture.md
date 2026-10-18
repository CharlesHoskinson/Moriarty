# Hollow Chip Brainstorm 02 — Architectural Cutaway & Subterranean Rotunda Atrium Experience

**Role**: Agent 2 of 5 — *"Inside Hollow Chip" Architectural Cutaway & Atrium Experience Designer*  
**Inspected Artifacts**:
- `clean_v1_interior.png` (`Inside Hollow Chip` preset at Step 53/145)
- `clean_v1_frames.png` (`Cross-Section` preset at Step 53/145)
- `clean_v1_overview_step52.png` (`Overview` preset at Step 53/145)
- `src/designers/designer1_package.js`, `src/designers/designer2_well.js`, `src/designers/designer3_frames.js`, `src/designers/designer7_cinema.js`, `src/stage/SherlockStage.js`

---

## 1. Visual Diagnosis of the Current Screenshots (Why `Inside Hollow Chip` & `Cross-Section` Fail Today)

### A. `clean_v1_interior.png` ("Inside Hollow Chip") — Camera Buried Inside the Top Wafer Edge
- **What the screenshot shows**: An extreme, disorienting Dutch-angle macro crop of the edge of two continuation wafers (`RETURN fact(4) · n = 4` and `D3 · env#3`), with zero spatial context. The viewer cannot see that they are inside a 3D room, cannot see the 6 concentric rings or 10-sector stator ceiling above, cannot see the full stack of 5 active wafers, and cannot see the 39-rule floor below.
- **Root geometric cause**:
  1. In `designer7_cinema.js`, `CAMERA_PRESETS.interior` places the camera at `pos: [0.52, -0.52, 0.56]` looking at `[-0.06, -1.22, -0.06]` (`fov: 48`).
  2. In `designer3_frames.js`, the top Saved Frame Wafer (`chronoIdx = 0`) sits at `y = -0.52` and spans `WAFER_OUTER_W = 1.24` (`x ∈ [-0.62, +0.62]`) by `WAFER_OUTER_D = 1.16` (`z ∈ [-0.58, +0.58]`), with corner radius `r_corner = √(0.62² + 0.58²) ≈ 0.85`.
  3. Thus `pos = [0.52, -0.52, 0.56]` is **physically inside the bounding volume of Wafer #1 (`x = 0.52 < 0.62`, `y = -0.52`, `z = 0.56 < 0.58`)**!
  4. Why couldn't the camera simply move further back (e.g., `z = +1.35..+1.85, y = -0.85`)? Because in `designer2_well.js`, `createSteppedTerrace()` builds **4 solid, opaque rectangular walls** at `innerHalf = 0.986..0.916` (`OUTER_HALF = 1.06`), leaving only `0.95 - 0.58 = 0.37` units of clearance between the front of the wafers and the solid South wall! Furthermore, in `designer1_package.js`, the opaque paper bench (`benchMesh` at `y = -0.36`) has a circular bore of only `WELL_RADIUS = 1.06`, so any camera outside `r = 1.06` below `y = -0.36` has its line of sight sliced in half by the bench plane and blocked by the solid South chamber wall.

### B. `clean_v1_frames.png` ("Cross-Section") — Not a Cross-Section at All
- **What the screenshot shows**: A slightly closer top-down oblique exterior view (`pos: [1.95, 2.15, 2.55]`, `lookAt: [-0.18, -0.48, 0.04]`) looking into the top `r = 1.06` collar.
- **Root geometric cause**: Because the Three-Tier Package (`CERAMIC_HALF = 2.15`), the bench ground (`y = -0.36` outside `r = 1.06`), and the 4-sided chamber walls (`y = -0.35..-2.15`) are 100% opaque all the way around `360°`, the subterranean chamber (`y = -0.35..-2.18`) is completely hidden from any oblique angle. Not a single continuation wafer or floor tile is visible in `Cross-Section`!

---

## 2. Pillar 1: Inverted Vault Geometry (`r_collar = 1.06` Flaring into a Subterranean Stainless & Obsidian Rotunda `r_vault = 1.45`)

Instead of extruding a narrow `±0.95` box shaft straight down from the `r = 1.06` top aperture, we use the true volume of the monolithic chip (`CERAMIC_HALF = 2.15`, `SILVER_STEP_HALF = 1.98`, `DARK_SHIELD_HALF = 1.82`) to create an **Inverted Rotunda Vault** inside the chip:

| Architectural Zone | Current (`v1`) | Proposed Inverted Vault (`v2`) | Spatial & Visual Impact |
| :--- | :--- | :--- | :--- |
| **Chip Exterior Footprint** | `CERAMIC_HALF = 2.15` | `CERAMIC_HALF = 2.15` (unchanged) | Preserves 100% of the clean `Overview` exterior silhouette. |
| **Upper Well Collar (`y = +0.108..-0.22`)** | `WELL_RADIUS = 1.06` | `WELL_RADIUS = 1.06` (unchanged) | Preserves the exact 6 Split-Arc Rings & Fuel Ring proportions. |
| **Inverted Stainless Vault Flare (`y = -0.22..-0.46`)** | None (flat step at `y = -0.35`) | `LatheGeometry` flaring smoothly from `r = 1.06` (`y = -0.22`) to `r = 1.45` (`y = -0.46`) | Creates a cathedral-like **Brushed Stainless-Platinum Domed Ceiling** when looking up/across from inside the chip. |
| **Subterranean Chamber Inner Radius (`y = -0.46..-2.15`)** | Square box `innerHalf = 0.916..0.986` | Cylindrical/Octagonal Rotunda `r_inner = 1.42` (`r_outer = 1.56`) | **+55% wider radial room** (`+120%` floor area), creating a `0.66`-unit-wide annular gallery outside the wafers! |
| **Continuation Wafer Footprint (`y = -0.52..-1.92`)** | `1.24 × 1.16` (`r_corner = 0.85`) | Annular ring `r_inner = 0.36, r_outer = 0.74` | Wafers sit cleanly inside `r ≤ 0.74` with an open central shaft, leaving a generous **`r ∈ [0.74, 1.42]` camera & light-pillar atrium** all around them. |
| **4 Vertical Optical Light-Pillars** | None | `4` slim laser-etched optical rails at `r = 0.82` (`θ = ±35°, ±145°`, `y = -0.24..-2.18`) | Physically anchors the floating wafers inside the 3D atrium and carries vertical semantic light pulses from the rings to the 39-rule floor. |
| **39-Rule Decoder Floor (`y = -2.18`)** | `floorSpan = 1.84` crammed into `±0.92` | `floorSpan = 2.04` set inside a **Machined Stainless Floor Bezel Ring (`r = 1.38..1.45`)** | Every rule tile (`A080..A225`) is framed like a luxury watch movement plate at the base of the stainless vault. |

---

## 3. Pillar 2: Dynamic Architectural Cutaway for `Inside Hollow Chip` (`interior`) & `Cross-Section` (`frames`)

We introduce a **Smooth Damped Architectural Cutaway (`cutawayMix ∈ [0, 1]`)** driven by `activePreset === 'interior' || activePreset === 'frames'`:
- **`Overview` (`cutawayMix = 0`)**: 100% solid monolithic three-tier package.
- **`Inside Hollow Chip` (`interior`, `cutawayMix = 1`)**: Camera sits at `pos: [0.38, -0.98, 1.32]`, `lookAt: [-0.04, -1.08, -0.18]`, `fov: 44`, viewing the stainless vaulted ceiling, the 6 split-arc rings & 10-sector stator overhead, the stacked open-bore continuation wafers in the center, and the 39-rule floor below.
- **`Cross-Section` (`frames`, `cutawayMix = 1`)**: Camera sits at `pos: [1.65, 0.62, 2.55]`, `lookAt: [-0.10, -0.92, -0.05]`, `fov: 34`, revealing the full vertical cutaway section of the chip from the exterior top shield down to the subterranean rule floor.
