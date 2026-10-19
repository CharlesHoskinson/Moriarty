# Sherlock 3D Visualization, Authentic Quinelings & Zero-Quality-Loss Performance Program

Status: **Active Implementation / OpenSpec & EARS Consolidated**, 2026-10-18. This package consolidates the findings of **20 independent performance audits** (10 Grok 4.7 `xhigh` + 10 Gemini Argon High) and **5 specialized Quineling morphogenesis & UI audits** (Grok 4.7 `xhigh`) into normative EARS requirements (`SVP-001`–`SVP-040`), OpenSpec scenarios, and a six-sprint implementation schedule (`SVP-SP01`–`SVP-SP06`).

## 1. Problem and Result

The Sherlock Universal Interpreter 3D Visualization (`visualization/`, developed at `http://127.0.0.1:4377` / `/home/charles/sherlock-bench`) renders the 12-slot split-arc ring register (`R1.A..R6.B`), 10-sector Control Core, 39-rule CEK floor array, immutable inductive wafer DAG, arithmetic carry/borrow loom, and program-authored Quineling 3D lifeforms across 8 designer views (`Designer 1`–`Designer 8`).

Three mission-critical objectives govern this OpenSpec change:
1. **Zero-Quality-Loss Performance Optimization (20-Agent Audit Consolidation)**:
   - Eliminate CPU/GPU bottlenecks across the WebGPU/WebGL stage, 8 Three.js designers, CEK trace engine, native Rust runner, HTTP server, and DOM/CSS controller while preserving **100% of visual, physical-material (PBR), lighting, shadow, and semantic trace fidelity**.
2. **Authentic `nursery.html` Quineling Shapes Driven by Real Programs (Upstream SDK/QDS + Designer 8)**:
   - Replace cartoon animal/template surrogates in Designer 8 with the authentic 8 `nursery.html` mathematical `WovenBody` mechanisms (`logarithmic-mantle`, `pleated-braid`, `recursive-affine`, `lorenz-flow`, `phyllotaxis-fan`, `clifford-flow`, `recursive-julia`, `toroidal-weave`).
   - Enhance the upstream Quinelings SDK and QDS (`/home/charles/repos/quinelings`) so that real inputted programs author both the `WovenBody` mechanism selection and continuous program-dependent variation in shape (`chirality`, `radialModFreq`, `axialTwist`, `lobeCount`, `filamentSpread`), motion (`temporalCadence`, `phaseHarmonic`, `breathDepth`), and color (`paletteRot`, `iridescenceSpan`, `luminanceFloor`, `chromaContrast`).
3. **Static GitHub Pages Deployment in `Moriarty` (`sherlock` branch)**:
   - Host the complete interactive 3D visualization under `visualization/` on the `sherlock` branch of `Moriarty` and integrate it into `site/scripts/build-docs.mjs` so that GitHub Pages publishes the standalone application at `/Moriarty/visualization/` with automatic client-side CEK trace evaluation when `/api/trace` is absent.

## 2. Audit Corpus Consolidated in This Package

All 25 independent audit reports are retained in `visualization/audits/` and mapped in [traceability.md](traceability.md):

- **10 Grok 4.7 `xhigh` Performance Reviews (`visualization/audits/perf/grok_perf_01..10.md`, 9,271 lines)**:
  - `grok_perf_01_stage_theme.md` — `SherlockStage.js` & `theme.js`
  - `grok_perf_02_designer1.md` — `designer1_package.js`
  - `grok_perf_03_designer2_rings.md` — `designer2_well.js` (Part A: Rings, Stator & Control Core)
  - `grok_perf_04_designer2_floor.md` — `designer2_well.js` (Part B: 39-Rule Floor, Wafers & `.update()`)
  - `grok_perf_05_designers3_4.md` — `designer3_frames.js` & `designer4_arena.js`
  - `grok_perf_06_designers5_6_7.md` — `designer5_circuit.js`, `designer6_loom.js` & `designer7_cinema.js`
  - `grok_perf_07_designer8.md` — `designer8_quineling.js`
  - `grok_perf_08_quineling_engine.md` — `sherlockQuinelingCompiler.js` & `quinelingsSdk.js`
  - `grok_perf_09_cek_rust_server.md` — `sherlockEngine.js`, `rust-runner/*.rs` & `server.js`
  - `grok_perf_10_app_dom_css.md` — `app.js`, `index.html` & `styles.css`
- **10 Gemini Argon High Performance Reviews (`Gemini-Perf-01..10`)**:
  - Parallel 10-subsystem deep architectural and line-by-line audits matching the exact 10 domains above.
- **5 Grok 4.7 `xhigh` Quineling Morphogenesis & UI Reviews (`visualization/audits/grok_quineling_01..05.md`, 1,111 lines)**:
  - `grok_quineling_01_visual_species.md` — Authentic `nursery.html` `WovenBody` visual species vs cartoon animals
  - `grok_quineling_02_manipulation_camera.md` — Direct 3D manipulation, orbit damping & bounding-sphere raycasting
  - `grok_quineling_03_compiler_morphogenesis.md` — AST-to-QDL task-graph lowering & semantic morphogenesis
  - `grok_quineling_04_m3_ui_and_slop_audit.md` — Material 3 Expressive HUD polish & removal of geometric clutter overlays
  - `grok_quineling_05_upstream_sdk_patch.md` — Upstream SDK/QDS program-authored shape, motion & color modulation

## 3. Capabilities and Structure

- [requirements.md](requirements.md): Master EARS index (`SVP-001`–`SVP-040`).
- [specs/sherlock-visualization-and-performance/spec.md](specs/sherlock-visualization-and-performance/spec.md): Normative EARS clauses and `WHEN`/`THEN` verification scenarios.
- [design.md](design.md): Subsystem architecture, mathematical invariants, and zero-quality-loss optimization rules.
- [traceability.md](traceability.md): Crosswalk from all 25 reviews to `SVP-001`–`SVP-040` and target source files.
- [tasks.md](tasks.md) & [SPRINT-PLAN.md](SPRINT-PLAN.md): Six-sprint execution plan (`SVP-SP01`–`SVP-SP06`).
