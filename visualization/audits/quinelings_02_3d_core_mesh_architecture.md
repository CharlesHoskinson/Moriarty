# Quineling 02 — 3D Procedural Quineling Core Mesh & WebGPU Shader Architecture

> **Target Document Path:** `/home/charles/sherlock-bench/audits/quinelings_02_3d_core_mesh_architecture.md`
> **Subsystem Role:** 3D Procedural Quineling Core Organism (`createQuinelingSubsystem`) suspended at `(x = 0, y = -1.65, z = 0)` inside the Sherlock hollow atrium (`r <= 0.56`, `y ∈ [-2.25, -1.05]`).

---

## 1. Spatial Envelope, Sightline Guarantees & Camera Geometry

The Sherlock processor’s interior vault (`designer2_well.js` & `designer3_frames.js`) has strict architectural envelopes:
- **6 Split-Arc Semantic Rings (`R1..R6`):** `y = +0.06` down to `-0.21`, radial span `r = 0.232..0.950`.
- **10-Sector Control Optical Stator:** `y = -0.24`, `r = 0.142..0.216`, with a **True Open Central Bore (`r < 0.135`)**.
- **12 Annular Continuation Wafers (`designer3_frames.js`):** `y = -0.64` down to `-2.75`, perimeter gallery `r = 1.02..1.28`, leaving the central `r < 1.00` cylinder completely open.
- **39-Rule 3D Silicon Floor (`designer2_well.js`):** `y = -3.38` (active elevated chiplets peak at `y = -3.286`, static label overlay at `y = -3.288`).
- **Interior First-Person Camera (`designer7_cinema.js`):**
  - `interior`: `pos = [0.0, -1.88, 0.94]`, `lookAt = [0.0, -2.42, -0.42]`, `fov = 72°`
  - `interior_up`: `pos = [0.0, -2.35, 0.32]`, `lookAt = [0.0, -0.10, -0.02]`, `fov = 70°`
  - `interior_floor`: `pos = [0.0, -2.15, 0.88]`, `lookAt = [0.0, -3.35, -0.18]`, `fov = 70°`
  - `frames` (Cross-Section): `pos = [1.95, -0.55, 4.95]`, `lookAt = [-0.05, -1.72, 0.0]`, `fov = 40°`
  - `well` (Top-Down Rings): `pos = [0.0, 3.65, 0.01]`, `lookAt = [0.0, -0.12, 0.0]`, `fov = 28°`

### 1.1 Five-Layer Procedural & Instanced 3D Anatomy
1. **Layer 1: Central Nucleus, Double Quote Membrane & Apical Bore Crown (`nucleusMesh` + `membraneMesh` + `crownHaloMesh`)**
2. **Layer 2: Articulated Vertebrae / Organ Nodes (`vertebraeInstancedMesh`, Count = `16`)**
3. **Layer 3: Radial Harmonic Ribs / Folded Petals (`ribsInstancedMesh`, Count = `48`)**
4. **Layer 4: Endpoint-Pinned Recursive Filaments & Root/Crown Tendrils (`filamentsLineMesh`, Dynamic `LineSegments`)**
5. **Layer 5: Orbiting Chroma Spores — Lossless Source Byte Strand (`sporesInstancedMesh`, Count = `64`)**
