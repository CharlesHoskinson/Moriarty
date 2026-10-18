# Hollow Chip Brainstorm 05: Stainless Steel Studio Reflections, Hollow Vault Lighting & WebGPU Polish

**Architect Role:** Agent 5 of 5 — Stainless Steel Studio Reflections, Hollow Vault Lighting & WebGPU Polish Architect  
**Inspected Visual Captures:** `clean_v1_initial.png`, `clean_v1_interior.png`, `clean_v1_well.png`, `clean_v1_frames.png`  
**Inspected Code Modules:** `src/stage/SherlockStage.js`, `src/designers/theme.js`, `src/designers/designer1_package.js` .. `src/designers/designer7_cinema.js`

---

## 1. Procedural Apple Studio Softbox Environment Map (`scene.environment`)

In `SherlockStage.js`, `this.scene.environment` is currently `null`. For PBR surfaces with `metalness >= 0.85` (the machined silver-platinum interposer step, the lathe-turned beveled funnel collar, the 12 split-arc rings, and the wafer rhodium bevels), ~90% of integrated specular response comes from `scene.environment`.
- Add a cached singleton `createStudioEnvironmentMap()` in `src/designers/theme.js` (`THREE.EquirectangularReflectionMapping`) with an overhead warm studio softbox band, a cool sky fill band, a crisp platinum-graphite horizon band, and warm-gold/cyan-teal nadir vault bounce bands.
- Assign `this.scene.environment = createStudioEnvironmentMap()` and `this.scene.environmentIntensity = 0.65` in `SherlockStage._setupLights()`.
- Attach `map: brushedMetalMap, roughnessMap: brushedRoughnessMap` to `collarMat` in `designer1_package.js` so the lathe-turned funnel collar shares the anisotropic grain of the Tier 2 interposer step.

## 2. Interior Hollow Chamber Architectural Lighting & Fluted Stainless Vault Ribs

1. **12 Machined Stainless-Steel Vertical Vault Pillar Ribs (`vaultRibsInstancedMesh`)**:
   - Place 12 vertical fluted ribs around the chamber perimeter in `1 THREE.InstancedMesh` using `halfArcOuterMatEven` (`metalness: 0.90, roughness: 0.25, clearcoat: 0.35`).
2. **12 Vertical Optical Light-Guide Filaments (`vaultGuidesInstancedMesh`)**:
   - Inset along the inner chamfer of each stainless pillar rib (`1 THREE.InstancedMesh`), glowing with the active semantic transition color (`ENV` cyan-teal, `OP` gold, `GATE` violet).
3. **Interior Hollow Vault Floor Bounce Light (`vaultFloorLight` at `y = -1.95`)**:
   - Add `this.vaultFloorLight = new THREE.PointLight(0x2dd4bf, 1.65, 4.2, 1.8)` at `(0, -1.95, 0)` in `SherlockStage._setupLights()` to illuminate the lower wafers, the 12 stainless vault ribs, and the 39-rule floor.

## 3. WebGPU Performance & Smoothness Optimizations (Locked 60–120fps)

- **`designer2_well.js`**: Dirty-flag guard on `ruleTilesInstancedMesh` (skip stationary tiles), `statorSectorsInstancedMesh` (cache `lastActiveSectorIdx`), and `bridgeClaspsInstancedMesh` (only update when `anyClaspMoved`).
- **`designer1_package.js`**: Cache `lastLeadPhase` on `edgePadsInstanced` so `STILL` phase does not re-upload 24 colors every frame.
- **`SherlockStage.js`**: Tighten `keyLight.shadow.camera` frustum (`left = -3.5, right = 2.5, top = 2.6, bottom = -2.6, near = 4.5, far = 18.5`) for ~65% sharper shadow resolution on the split-arc rings and West-flank Call Chiplets.
- **`designer7_cinema.js`**: Remove the 1-frame `0.70` snap in `setCameraPreset()` and use a smooth arc-lift trajectory and cached inspectable bounding boxes.
