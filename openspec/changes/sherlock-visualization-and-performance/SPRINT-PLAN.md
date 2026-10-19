# Sherlock Visualization & Performance Sprints (`tasks.md` / `SPRINT-PLAN.md`)

Status: **Active Execution Loop Armed (`SVP-SP01`–`SVP-SP07`)**, 2026-10-18.
This schedule organizes all 43 EARS requirements (`SVP-001`–`SVP-043`) derived from the 20 performance reviews (10 Grok 4.7 `xhigh` + 10 Gemini Argon High), 5 Quineling reviews, and 3 Gemini `Quint-llm-kit` verification audits into seven executable sprints.

---

## Sprint 1 (`SVP-SP01`): Sherlock CEK Engine, Native Rust Runner & HTTP Server
**Requirements**: `SVP-032`, `SVP-033`, `SVP-034`, `SVP-035`, `SVP-036`
**Target Files**: `visualization/src/engine/sherlockEngine.js`, `visualization/rust-runner/*.rs`, `visualization/server.js`

- [x] **SVP-SP01.1 (`SVP-032`)**: Memoize per-step telemetry (`enrichStepTelemetry`) and call-hierarchy spans (`computeCallSpans`) using `WeakMap` caches keyed by trace and step object references in `src/engine/sherlockEngine.js`.
- [x] **SVP-SP01.2 (`SVP-033`)**: Replace per-step inactive arithmetic witness allocations with a frozen `INACTIVE_ARITHMETIC_WITNESS` singleton and add a 32-entry client trace LRU cache in `src/engine/sherlockEngine.js`.
- [x] **SVP-SP01.3 (`SVP-034`)**: Implement static-host detection (`*.github.io`, `file:`, or latched network fallback) in `src/engine/sherlockEngine.js` so GitHub Pages evaluates CEK traces client-side with zero failed network stalls.
- [x] **SVP-SP01.4 (`SVP-035`)**: Optimize `rust-runner/mori_lexer.rs` with zero-copy `&str` slice scanning and compile `rust-runner/sherlock-runner` with `-C opt-level=3 -C lto=thin -C codegen-units=1 -C target-cpu=native`.
- [x] **SVP-SP01.5 (`SVP-036`)**: Add in-memory Brotli/Gzip static asset caching, `ETag` / `304 Not Modified` handling, and a 32-entry `/api/trace` response LRU cache in `server.js`.

---

## Sprint 2 (`SVP-SP02`): WebGPU/WebGL Stage, Lighting, Theme & App DOM/CSS Containment
**Requirements**: `SVP-001`, `SVP-002`, `SVP-003`, `SVP-004`, `SVP-005`, `SVP-037`, `SVP-038`
**Target Files**: `visualization/src/stage/SherlockStage.js`, `visualization/src/designers/theme.js`, `visualization/src/app.js`, `visualization/src/styles.css`

- [x] **SVP-SP02.1 (`SVP-001`)**: Gate directional shadow map regeneration (`keyLight.shadow.needsUpdate`) in `SherlockStage.js` via `_shadowDirtyFrames` and `markShadowDirty(frames)`.
- [x] **SVP-SP02.2 (`SVP-002`)**: Coalesce pointer coordinates on `pointermove` and execute targeted raycasting at most once per `requestAnimationFrame` tick in `SherlockStage.js`.
- [x] **SVP-SP02.3 (`SVP-003`)**: Replace 65,536 `fillRect` calls in `theme.js` `getCeramicGrainTexture()` with a single `ImageData` `Uint8ClampedArray` write and `putImageData()`.
- [x] **SVP-SP02.4 (`SVP-004`)**: Memoize engraved plaques (`createEngravedPlaque`) and shared register materials in `theme.js` with bounded LRU eviction (`MAX_PLAQUE_CACHE = 512`).
- [x] **SVP-SP02.5 (`SVP-005`)**: Eliminate per-frame `THREE.Vector3` / `THREE.Color` allocations in `SherlockStage.js` camera flight and orbit damping loops, and add complete `dispose()` cleanup.
- [x] **SVP-SP02.6 (`SVP-037`)**: Refactor `src/app.js` HUD updates (12-slot ring strip, 39-rule grid, stack frames, preset buttons with `data-preset-id`) to mutate cached DOM nodes in place instead of rebuilding `.innerHTML`.
- [x] **SVP-SP02.7 (`SVP-038`)**: Add `contain: layout style paint` to HUD overlays and drawers in `src/styles.css`.

---

## Sprint 3 (`SVP-SP03`): Upstream Quinelings SDK/QDS Program-Authored Morphogenesis & Hot-Path Optimization
**Requirements**: `SVP-026`, `SVP-027`, `SVP-028`, `SVP-029`, `SVP-030`, `SVP-031`
**Target Files**: `/home/charles/repos/quinelings/src/*.js`, `visualization/src/engine/quinelingsSdk.js`, `visualization/src/engine/sherlockQuinelingCompiler.js`

- [x] **SVP-SP03.1 (`SVP-026`)**: Compile real `.mori` ASTs in `sherlockQuinelingCompiler.js` into executable Quinelings task graphs whose topological invariants author all 8 `WovenBody` mechanisms (`logarithmic-mantle`, `pleated-braid`, `recursive-affine`, `lorenz-flow`, `phyllotaxis-fan`, `clifford-flow`, `recursive-julia`, `toroidal-weave`).
- [x] **SVP-SP03.2 (`SVP-027`, `SVP-028`, `SVP-029`)**: Enhance upstream Quinelings SDK (`woven-body.js`, `lifeform-families.js`, `chroma.js`, `qdl.js`, `visual-capsule.js`, `creation.js`, `nursery.js`) with deterministic `programDigest` modulation for shape, temporal motion, and OKLCH chroma/iridescence.
- [x] **SVP-SP03.3 (`SVP-030`)**: Add a 1-entry 60Hz reference fast-path + 64-entry LRU cache, precompiled RegExps, single-pass AST analysis, and a lazy `morphBounds` getter in `sherlockQuinelingCompiler.js`.
- [x] **SVP-SP03.4 (`SVP-031`)**: Optimize `quinelingsSdk.js` (`WovenBody` & `LifeformFamilies`) with `reuse.points` / `reuse.owners` typed-array reuse across all 8 mechanisms (including `juliaFrame` and `recursiveFrame`), direct `points[p4]` writes, O(1) `territoriesByComponent` / `territoryByOwner` / `ownerMarks` lookups, neighbor-cached `basisWithNeighbors` + `differentialNzFromBasis` in `caustics`, and reused `compressionGrid` buffers.

---

## Sprint 4 (`SVP-SP04`): Authentic `nursery.html` 3D `WovenBody` Rendering in Designer 8
**Requirements**: `SVP-019`, `SVP-020`, `SVP-021`, `SVP-022`, `SVP-023`, `SVP-024`, `SVP-025`
**Target Files**: `visualization/src/designers/designer8_quineling.js`

- [x] **SVP-SP04.1 (`SVP-019`, `SVP-020`)**: Render `WovenBody_TissueCloud` (`TISSUE_BUDGET = 8192`) and full-strand `WovenBody_CrestFilaments` (`MAX_RIDGE_SEGMENTS = 28672`) with `nursery.html` Y-up projection (`ny = -(y - cy) * scale`) and exponential optical density alpha (`1 - Math.exp(-2.2 * rawPtAlpha)`).
- [x] **SVP-SP04.2 (`SVP-021`)**: Hide non-`nursery.html` polygon-chord clutter overlays (`sdkManifoldRibbonMesh`, `depArcLines`, `tendrilMesh`).
- [x] **SVP-SP04.3 (`SVP-022`)**: Quantize live `WovenBody.frame()` sampling to 30Hz and pass reusable `Float32Array` / `Uint16Array` buffers.
- [x] **SVP-SP04.4 (`SVP-023`, `SVP-024`, `SVP-025`)**: Implement O(1) bounding-sphere raycasting on `WovenBody_TissueCloud`, step-gate Quineling HUD badge updates, and lock static pedestal matrices (`matrixAutoUpdate = false`).

---

## Sprint 5 (`SVP-SP05`): Designers 1–7 Zero-Quality-Loss Instancing, Dirty-Gating & Memory Hygiene
**Requirements**: `SVP-006`–`SVP-018`
**Target Files**: `visualization/src/designers/designer1_package.js` .. `visualization/src/designers/designer7_cinema.js`

- [x] **SVP-SP05.1 (`SVP-006`, `SVP-007`)**: Lock static substrate/pin/capacitor matrices (`matrixAutoUpdate = false`), step-gate state updates, and use scratch `THREE.Color` instances in `designer1_package.js`.
- [x] **SVP-SP05.2 (`SVP-008`, `SVP-009`, `SVP-010`, `SVP-011`)**: Lock static well/stator/floor matrices, step-gate ring & Control Core plaque updates, and apply O(1) active-rule floor delta transitions in `designer2_well.js`.
- [x] **SVP-SP05.3 (`SVP-012`, `SVP-013`, `SVP-014`)**: Ensure zero-allocation 60fps `.update()` math and proper geometry/material disposal on trace rebuild in `designer3_frames.js` and `designer4_arena.js`.
- [x] **SVP-SP05.4 (`SVP-015`, `SVP-016`)**: Step-gate circuit/loom updates, use `INACTIVE_ARITHMETIC_WITNESS` fast path, and dispose dynamic geometries on rebuild in `designer5_circuit.js` and `designer6_loom.js`.
- [x] **SVP-SP05.5 (`SVP-017`, `SVP-018`)**: Pre-collect cutaway fadeable material arrays, settle-gate opacity updates, and skip `.update()` on hidden sub-designers in `designer7_cinema.js`.

---

## Sprint 6 (`SVP-SP06`): Static GitHub Pages Integration in `Moriarty` (`sherlock` branch)
**Requirements**: `SVP-039`, `SVP-040`
**Target Files**: `visualization/index.html`, `site/scripts/build-docs.mjs`, `site/src/App.tsx`, `site/.gitignore`

- [x] **SVP-SP06.1 (`SVP-039`)**: Configure relative `./` asset and `<script type="importmap">` paths plus `<link rel="modulepreload">` hints in `visualization/index.html`.
- [x] **SVP-SP06.2 (`SVP-040`)**: Sync `/home/charles/sherlock-bench` to `/home/charles/repos/Moriarty/visualization` and update `site/scripts/build-docs.mjs` so `npm --prefix site run build` copies `visualization/` into `site/public/visualization/` and outputs `site/dist/visualization/index.html`.
- [x] **SVP-SP06.3 (`SVP-040`)**: Verify static GitHub Pages build (`npm --prefix site run verify`) and headless Chromium Playwright execution of `site/dist/visualization/index.html` with zero backend API.

---

## Sprint 7 (`SVP-SP07`): 3-Agent Gemini + `Quint-llm-kit` Formal Verification & Memory-Leak Audit
**Requirements**: `SVP-041`, `SVP-042`, `SVP-043`
**Target Files**: `visualization/specs/quint/*.qnt`, `visualization/audits/quint/*.md`, all `visualization/src/**/*.js`

- [x] **SVP-SP07.1 (`SVP-041`)**: Run Gemini Quint-LLM-Kit Agent #1 (`Quint-Memory-Lifecycle-Verifier`) to author and verify `visualization/specs/quint/visualization_memory_lifecycle.qnt` (`quint typecheck`, `quint test`, `quint run`) and audit `SherlockStage.js`, `theme.js`, and `designer1`–`designer8` for GPU/heap memory leaks.
- [x] **SVP-SP07.2 (`SVP-042`)**: Run Gemini Quint-LLM-Kit Agent #2 (`Quint-Engine-Compiler-Verifier`) to author and verify `visualization/specs/quint/engine_quineling_pipeline.qnt` (`quint typecheck`, `quint test`, `quint run`) and audit `sherlockEngine.js`, `sherlockQuinelingCompiler.js`, `quinelingsSdk.js`, and `server.js` for cache-aliasing, typed-array reuse, and numeric edge-case bugs.
- [x] **SVP-SP07.3 (`SVP-043`)**: Run Gemini Quint-LLM-Kit Agent #3 (`Quint-UI-Playback-GitHubPages-Verifier`) to author and verify `visualization/specs/quint/ui_playback_lifecycle.qnt` (`quint typecheck`, `quint test`, `quint run`) and audit `app.js`, `index.html`, `styles.css`, and `site/scripts/build-docs.mjs` for async race conditions, DOM state desyncs, and static GitHub Pages bugs.
- [x] **SVP-SP07.4**: Remediate all memory leaks, bugs, and issues identified by the 3 Gemini Quint-LLM-Kit agents, re-verify Playwright + Quint checks, and commit to `Moriarty` (`sherlock` branch).
