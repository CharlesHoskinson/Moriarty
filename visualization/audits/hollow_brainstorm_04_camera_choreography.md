# Hollow Chip Brainstorm 04 — Camera Zoom Choreography, Swoop Trajectories & Framing Director

## 1. Diagnosis of Current Stage Camera Defects
1. **`Inside Hollow Chip` (`interior`)**: Placed the camera at `[0.52, -0.52, 0.56]` inside the plane of `SavedFrameWafer_0` (`y = -0.52`), staring steeply downward into a single clipped wafer texture (`clean_v1_interior.png`).
2. **`Cross-Section` (`frames`)**: Hovered at `[1.95, 2.15, 2.55]` outside the uncut 360° package wall (`clean_v1_frames.png`), hiding the entire subterranean chamber.
3. **`Top-Down Rings` (`well`)**: Used an off-axis `z = 0.52` offset (`7.4°` tilt) that introduced parallax skew (`clean_v1_well.png`).
4. **Controller Bugs in `designer7_cinema.js`**:
   - 70% single-frame snap in `setCameraPreset()`.
   - Linear interpolation across the deck plane (`y = 0.105`) dragging the camera through solid geometry.
   - Hard horizontal radius clamp (`horizR <= 0.86` when `y < -0.22`) crushing sub-deck camera presets.

## 2. Verified Stage Camera Presets & Arc-Lift Swoop Trajectory
- **`overview`**: `pos: [3.85, 4.85, 5.35], lookAt: [-0.38, -0.16, 0.05], fov: 32, cutaway: 0.0`
- **`interior` (`Inside Hollow Chip`)**: `pos: [0.0, -0.82, 1.38], lookAt: [0.0, -0.88, -0.18], fov: 48, cutaway: 1.0`
- **`well` (`Top-Down Rings`)**: `pos: [0.0, 3.85, 0.01], lookAt: [0.0, -0.12, 0.0], fov: 28, cutaway: 0.0`
- **`frames` (`Cross-Section`)**: `pos: [1.35, 0.28, 2.75], lookAt: [-0.08, -0.88, 0.0], fov: 34, cutaway: 1.0`
