## ADDED Requirements

Status: **Consolidated 20-Agent Performance & 5-Agent Quineling OpenSpec Specification (`SVP-001`–`SVP-040`)**, 2026-10-18.
Every requirement below follows EARS (`WHEN / WHILE / IF / WHERE ... THE <subsystem> SHALL ...`) and is bound to the zero-quality-loss invariant: **100% of Apple industrial-design PBR finish, lighting, shadow softness, authentic `nursery.html` `WovenBody` morphogenesis, and Sherlock CEK trace semantics SHALL be preserved**.

---

### Requirement: SVP-001 Demand-gated directional shadow updates
WHILE the active designer and camera settle window has elapsed without scene or step mutations, the WebGPU/WebGL stage (`SherlockStage.js`) SHALL disable redundant directional shadow-map regeneration (`keyLight.shadow.needsUpdate = false`) and re-enable shadow updates (`markShadowDirty(frames)`) immediately upon any step change, designer switch, resize, or dynamic mesh motion.

#### Scenario: Static camera settle window
- **WHEN** `SherlockStage` renders frames after the shadow settle window (`_shadowDirtyFrames === 0`) on a static step.
- **THEN** The directional 2048×2048 shadow map is reused without re-rendering the shadow depth pass, and `markShadowDirty()` immediately restores shadow updates when the step or designer changes.

### Requirement: SVP-002 RAF-coalesced targeted raycasting
WHEN pointer movement occurs over the 3D viewport, `SherlockStage.js` SHALL coalesce pointer coordinates and execute `raycaster.intersectObjects` at most once per animation frame (`16.6ms`–`33ms`) against pre-registered interactive target arrays rather than traversing decorative static subtrees on every raw DOM `pointermove` event.

#### Scenario: High-frequency pointermove
- **WHEN** A 120Hz–1000Hz mouse or trackpad dispatches multiple `pointermove` events within a single `requestAnimationFrame` interval.
- **THEN** `SherlockStage` stores the latest normalized device coordinates and performs at most one targeted raycast pass during the frame tick.

### Requirement: SVP-003 Typed-array ImageData procedural textures
WHEN procedural ceramic and metallic micro-grain normal/roughness textures are synthesized in `theme.js`, the texture generator SHALL populate pixel buffers via a single `ctx.createImageData(size, size)` `Uint8ClampedArray` write and `ctx.putImageData(imgData, 0, 0)` instead of issuing `size * size` individual `fillStyle` + `fillRect` 2D canvas calls.

#### Scenario: Startup ceramic texture synthesis
- **WHEN** `getCeramicGrainTexture(256)` initializes at startup.
- **THEN** All 65,536 texels are written in a single typed-array loop and uploaded via one `putImageData` call with identical deterministic hash output.

### Requirement: SVP-004 Shared PBR material and glyph caches
WHEN multiple meshes across designers request identical PBR finishes or identical engraved label plaques (`createEngravedPlaque`, `createRegisterCapMaterial`, `createAnodizedAluminium`), `theme.js` SHALL memoize and return shared `CanvasTexture` and `MeshStandardMaterial`/`MeshPhysicalMaterial` instances keyed by their canonical parameter tuple unless unique per-instance mutation is requested.

#### Scenario: Repeated register/rule label lookup
- **WHEN** Designer 2 or Designer 7 requests plaques with identical `(text, subtext, width, height, bgHex, textHex, borderHex)` parameters.
- **THEN** The cached texture/material descriptor is returned without allocating a duplicate `<canvas>` or GPU texture.

### Requirement: SVP-005 Zero-allocation camera flight and orbit loop
WHILE `SherlockStage.js` updates camera presets, cinematic flights, or damped orbit controls at 60fps, the render loop SHALL compute all vector interpolations (`lerpVectors`, spherical offsets, target damping) using pre-allocated scratch `THREE.Vector3` and `THREE.Quaternion` instances with zero per-frame heap allocations.

#### Scenario: 60fps camera preset transition
- **WHEN** `flyToPreset()` interpolates camera position and look-at target over `0.85s`.
- **THEN** Zero `new THREE.Vector3` or `new THREE.Color` instances are allocated inside `_tick()`.

---

### Requirement: SVP-006 Designer 1 instanced pins, BGA, and capacitors
WHEN `designer1_package.js` constructs the Sherlock package substrate, it SHALL batch identical repeating LGA contact pads, BGA solder balls, and MLCC decoupling capacitors into `THREE.InstancedMesh` instances and lock static structural transforms (`matrixAutoUpdate = false`).

#### Scenario: Package substrate construction
- **WHEN** Designer 1 builds the perimeter pin array and decoupling capacitor ring.
- **THEN** Repeating pins/capacitors render via instanced draw calls and static substrate layers skip per-frame matrix recomputation.

### Requirement: SVP-007 Designer 1 step-dirty gate and zero-alloc pulse
WHILE `designer1_package.js` executes `.update(stepState, time)` on a frame where `stepState.stepIndex` is unchanged, it SHALL skip static rule/register plaque material re-bindings and update only time-varying trace emissive pulses using module-scoped scratch `THREE.Color` instances.

#### Scenario: Steady-step idle animation
- **WHEN** `.update(stepState, time)` runs across consecutive frames with the same `stepState.stepIndex`.
- **THEN** Only continuous bus/die pulse uniforms are updated and zero `THREE.Color` objects are allocated.

### Requirement: SVP-008 Designer 2 shared ring arc and stator geometries
WHEN `designer2_well.js` constructs the 12 split-arc ring registers (`R1.A..R6.B`), stator rings, and 10-sector Control Core, it SHALL share canonical `ExtrudeGeometry`/`CylinderGeometry`/`RingGeometry` instances across congruent ring slots and sectors and set `matrixAutoUpdate = false` on all stationary stator/well hardware.

#### Scenario: 12-slot R1.A..R6.B construction
- **WHEN** The 6 concentric split-arc register rings (`R1`–`R6`) are instantiated.
- **THEN** Congruent half-arcs share geometry buffers and stationary stator housing meshes lock their world matrices once at initialization.

### Requirement: SVP-009 Designer 2 ring and sector step-dirty memoization
WHILE `designer2_well.js` updates the 12 ring slots (`R1.A..R6.B`) and 10 Control Core sectors, it SHALL cache the previous step's slot values, provenance statuses, and active control tag so that canvas plaque redraws and material state changes occur only on step transitions, while continuous glow/breathing modulation uses pre-allocated scratch `THREE.Color` objects.

#### Scenario: Unchanged step index during orbit
- **WHEN** The user orbits the Hollow Well camera while paused on step `k`.
- **THEN** Ring slot plaques and Control Core sector labels are not redrawn or re-uploaded to the GPU.

### Requirement: SVP-010 Designer 2 instanced bore and 39-rule floor
WHEN `designer2_well.js` constructs the cylindrical bore wall and the 39-rule floor array, it SHALL share geometry buffers across all 39 rule modules, disable shadow casting (`castShadow = false`) on sub-millimeter internal LEDs/labels that contribute no visible shadow, and lock static floor module base matrices (`matrixAutoUpdate = false`).

#### Scenario: 39-rule floor and bore initialization
- **WHEN** All 39 rule floor cells and bore ribs are added to the Designer 2 group.
- **THEN** Shared rule cell geometries and locked static transforms eliminate redundant per-frame matrix traversals.

### Requirement: SVP-011 Designer 2 active-rule delta updates
WHEN `stepState.rule` transitions from rule `prevRule` to `nextRule` in `designer2_well.js`, the floor update routine SHALL update only the previously active rule module and the newly active rule module (plus active pulse waveguides) rather than re-evaluating material colors across all 39 rule modules every frame.

#### Scenario: Step transition from rule i to j
- **WHEN** The CEK machine advances from `stepIndex` `s` (`rule_a`) to `s + 1` (`rule_b`).
- **THEN** Only `rule_a` and `rule_b` floor modules mutate their base state; the remaining 37 inactive rule modules are untouched.

### Requirement: SVP-012 Designer 3 pooled continuation frame cards
WHEN `designer3_frames.js` renders the CEK continuation stack (`K`) across steps with varying stack depths, it SHALL maintain an object pool of frame card meshes and memoize frame card label textures by `(frameKind, label, envSummary)` rather than disposing and recreating meshes or `<canvas>` textures on every stack push/pop.

#### Scenario: Recursive call push/pop scrubbing
- **WHEN** The user scrubs back and forth across recursive `fact(4)` stack depths `0..4`.
- **THEN** Pooled frame card meshes are toggled via `.visible` and cached label textures are reused with zero GPU resource leaks.

### Requirement: SVP-013 Designer 4 pooled DAG wafer nodes and edges
WHEN `designer4_arena.js` renders inductive value DAG wafers (`Cons`, `Nil`, `Leaf`, `Branch`, scalar cells) and structural sharing links, it SHALL pool wafer node groups and connector tubes and cache wafer label plaques by canonical value signature.

#### Scenario: Tree/List constructor evaluation
- **WHEN** `TreeFold` or `ListFold` allocates and shares immutable inductive subtrees across steps.
- **THEN** Wafer meshes and connector curves are drawn from the pre-allocated pool without per-step geometry disposal churn.

### Requirement: SVP-014 Designers 3 and 4 scratch math transforms
WHILE `designer3_frames.js` and `designer4_arena.js` interpolate card elevations, spring animations, and wafer orbital positions at 60fps, all vector/euler/color calculations SHALL use module-scoped scratch objects (`_tmpVecA`, `_tmpVecB`, `_tmpColor`, `_tmpQuat`) with zero per-frame allocations.

#### Scenario: 60fps card and wafer interpolation
- **WHEN** `.update(stepState, time)` runs during playback in Designer 3 or Designer 4.
- **THEN** Zero temporary `THREE.Vector3`, `THREE.Euler`, or `THREE.Color` instances are allocated on the heap.

### Requirement: SVP-015 Designer 5 preallocated bus and gate buffers
WHEN `designer5_circuit.js` updates the bounded step-circuit witness buses, selector muxes, and arithmetic/equality gates, it SHALL reuse pre-allocated vertex/color attribute buffers and gate meshes and gate structural circuit reconfiguration on `stepState.stepIndex !== lastStepIndex`.

#### Scenario: Step-circuit witness animation
- **WHEN** Designer 5 animates signal propagation along active witness buses.
- **THEN** Gate states reconfigure only on step transitions and bus pulse animations mutate existing buffer attributes in place.

### Requirement: SVP-016 Designer 6 cached carry-borrow ribbon tubes
WHEN `designer6_loom.js` renders the 128-bit scalar arithmetic limb towers and carry/borrow propagation ribbons, it SHALL cache tube/ribbon geometries across steps when the arithmetic witness operands are unchanged and use the shared `INACTIVE_ARITHMETIC_WITNESS` fast path on non-arithmetic steps.

#### Scenario: Checked scalar arithmetic step
- **WHEN** The CEK trace steps through non-arithmetic control steps followed by a checked `sub` or `mul` step.
- **THEN** Non-arithmetic steps skip ribbon geometry rebuilds via the inactive witness fast path, and arithmetic steps reuse pre-allocated limb tower meshes.

### Requirement: SVP-017 Designer 7 visibility-gated subdesigner updates
WHILE `designer7_cinema.js` orchestrates the unified Cinema / Director view, it SHALL invoke `.update(stepState, time)` only on child sub-designers or narrative layers whose groups are currently visible (`group.visible === true` and opacity `> 0.001`).

#### Scenario: Director mode beat transitions
- **WHEN** Director mode focuses on an interior beat where exterior package or auxiliary layers are hidden (`visible === false`).
- **THEN** Hidden sub-designers are skipped during `.update()` and excluded from GPU scene traversal.

### Requirement: SVP-018 Designer 7 precollected cutaway material arrays
WHEN `designer7_cinema.js` initializes or transitions cutaway shell transparency, it SHALL iterate over a pre-collected flat array of fadeable materials cached at build time rather than calling `group.traverse()` every frame, and SHALL stop mutating `material.opacity` / `material.transparent` once `Math.abs(currentOpacity - targetOpacity) < 1e-4`.

#### Scenario: Cutaway shell transparency fade
- **WHEN** The exterior package shell finishes fading to its target cutaway opacity.
- **THEN** Per-frame `traverse()` calls are zero and settled materials cease triggering Three.js material state updates.

---

### Requirement: SVP-019 Authentic nursery.html WovenBody 3D point and ridge rendering
WHEN `designer8_quineling.js` renders a Sherlock program in Designer 8 (`Quineling`), it SHALL render the authentic `nursery.html` 3D `WovenBody` lifeform (`WovenBody_TissueCloud` `THREE.Points` with `TISSUE_BUDGET = 8192` + `WovenBody_CrestFilaments` `THREE.LineSegments` with `MAX_RIDGE_SEGMENTS = 28672` sampling the full length of every caustic/crest polyline) driven by `WovenBody.frame(wovenBody, phase)` and colored by `Chroma.colorFor(shape, ownerIdx)`.

#### Scenario: All 8 presets render WovenBody species
- **WHEN** Each of the 8 Sherlock presets (`factorial`, `list_fold`, `tree_fold`, `mutual_even_odd`, `lexical_let`, `checked_underflow`, `stack_overflow`, `fuel_exhaustion`) is selected in Designer 8.
- **THEN** The viewport displays the exact corresponding `nursery.html` `WovenBody` species (`logarithmic-mantle`, `pleated-braid`, `recursive-affine`, `lorenz-flow`, `phyllotaxis-fan`, `clifford-flow`, `recursive-julia`, `toroidal-weave`) with 8,192 tissue points and up to 28,672 caustic/crest filament segments, and never renders cartoon animal meshes.

### Requirement: SVP-020 Exponential density alpha and Y-up projection
WHEN `designer8_quineling.js` projects `WovenBody` 3D coordinates `(x, y, z)` and sample alpha into Three.js world space, it SHALL invert the 2D screen-space Y axis (`ny = -(y - cy) * scale`) to match `nursery.html` / `lifeform-webgl.js` orientation and apply exponential optical density mapping (`1 - Math.exp(-2.2 * rawAlpha)`) with premultiplied-alpha vertex shaders so translucent tissue clouds and bright caustic ridges exhibit authentic museum-specimen depth.

#### Scenario: WovenBody translucent tissue depth
- **WHEN** `WovenBody.frame()` returns raw sample points `(x, y, z, alpha)` and caustic ridges.
- **THEN** `designer8_quineling.js` maps `ny = -(y - cy) * scale` and computes premultiplied density alpha so overlapping folds and caustics glow without additive white-out or upside-down inversion.

### Requirement: SVP-021 Uncluttered specimen stage without chord overlays
WHEN `designer8_quineling.js` displays the 3D `WovenBody` specimen, non-`nursery.html` straight-line polygon chord overlays (`sdkManifoldRibbonMesh`, `depArcLines`, `tendrilMesh`) SHALL remain hidden (`visible = false`) so the organic `WovenBody` tissue cloud, caustic/crest filaments, and subtle semantic node anchors remain visually unobstructed.

#### Scenario: Hiding non-nursery ribbon/arc clutter
- **WHEN** Designer 8 renders any preset or custom `.mori` program.
- **THEN** `sdkManifoldRibbonMesh.visible`, `depArcLines.visible`, and `tendrilMesh.visible` are `false`, leaving the `WovenBody` tissue cloud and caustic filaments clean and unobstructed.

### Requirement: SVP-022 30Hz WovenBody sampling and zero-alloc typed buffers
WHILE `designer8_quineling.js` animates the `WovenBody` lifeform during live playback or idle breathing, it SHALL quantize `WovenBody.frame(wovenBody, phase, { budget: 8192, crests: true, reuse })` evaluations to a 30Hz phase cadence (`Math.floor(time * 30)`) and pass pre-allocated `Float32Array` and `Uint16Array` `reuse` buffers so zero typed arrays are allocated per frame.

#### Scenario: Live breathing animation at 60fps
- **WHEN** The render loop ticks at 60fps–120fps on a given CEK step.
- **THEN** `WovenBody.frame()` executes at most 30 times per second using pre-allocated `reuse.points` and `reuse.owners` buffers, while 60fps–120fps gentle specimen poise rotation continues smoothly every frame.

### Requirement: SVP-023 O(1) bounding-sphere point-cloud raycasting
WHEN the user moves the pointer over Designer 8, `WovenBody_TissueCloud` (`THREE.Points` with 8,192 vertices) SHALL override `raycast()` with an O(1) bounding-sphere intersection test (or defer to the semantic node anchor spheres) instead of testing all 8,192 individual points in JavaScript on every pointer move.

#### Scenario: Pointer hover over 8,192 tissue points
- **WHEN** `SherlockStage` raycasts against Designer 8 interactive targets on `pointermove`.
- **THEN** Raycasting completes in O(1) against the `WovenBody` bounding sphere and the semantic node anchor spheres without iterating 8,192 vertices.

### Requirement: SVP-024 Step-gated Quineling HUD and inspection badges
WHILE `designer8_quineling.js` updates the Quineling inspection HUD and Material 3 telemetry badges, it SHALL gate DOM text/style updates on changes to `(compiled.cacheKey, stepIndex, activeOpNodeId)` rather than rewriting DOM text nodes at 60fps.

#### Scenario: Scrubbing vs idle breathing frames
- **WHEN** `designer8_quineling.js` runs `.update(stepState, time)` across frames where `stepIndex` and `cacheKey` have not changed.
- **THEN** Zero DOM mutations occur on the Quineling HUD overlays.

### Requirement: SVP-025 Static matrix locking on pedestal and containment glass
WHEN `designer8_quineling.js` constructs the static museum pedestal, engraved brass/titanium trim rings, and floor shadow receiver, it SHALL call `updateMatrix()` and set `matrixAutoUpdate = false` on those static meshes.

#### Scenario: Museum vitrine and base rendering
- **WHEN** Designer 8 is mounted in the scene graph.
- **THEN** All stationary pedestal meshes have `matrixAutoUpdate === false`.

---

### Requirement: SVP-026 AST-to-QDL task-graph compilation for all 8 mechanisms
WHEN `compileSherlockProgramToQuineling(source, speciesCatalog, trace)` compiles a `.mori` program in `sherlockQuinelingCompiler.js`, it SHALL lower the real parsed Sherlock AST (`functions`, `exprs`, call graph, constructor/match topology, arithmetic operators, and resource bounds) into a valid Quinelings task graph whose topological invariants (`depth`, `fanout`, `cycles`, `reconvergent`, `cone`, `feedback`, `splits`, `leaves`, `merges`) deterministically author the corresponding `WovenBody` mechanism via `WovenBody.author(graph, seed)`.

#### Scenario: 8 Sherlock presets map to 8 mechanisms
- **WHEN** The 8 Sherlock presets are compiled through `compileSherlockProgramToQuineling`.
- **THEN** They deterministically author all 8 `WovenBody` mechanisms (`logarithmic-mantle`, `pleated-braid`, `recursive-affine`, `lorenz-flow`, `phyllotaxis-fan`, `clifford-flow`, `recursive-julia`, `toroidal-weave`) and pass `Quinelings.verifyQuine(quineProgram)`.

### Requirement: SVP-027 Program-derived WovenBody shape modulation
WHEN `WovenBody.author(graph, seed)` and `LifeformFamilies.author(graph, seed, mechanism)` author a lifeform in the upstream Quinelings SDK (`woven-body.js`, `lifeform-families.js`, bundled into `quinelingsSdk.js`), they SHALL derive continuous program-dependent shape modulation parameters (`chirality`, `radialModFreq`, `radialModAmp`, `axialTwist`, `lobeCount`, `filamentSpread`, `pleatSharpness`, `foldAsymmetry`) from a spectral digest of the program's opcode sequence, literal constants, node identifiers, and edge topology.

#### Scenario: Varying AST depth, branching and literals
- **WHEN** Two `.mori` programs with different AST structures, constants, or branching factors compile to the same `WovenBody` mechanism family.
- **THEN** Their `WovenBody` records exhibit distinct, deterministic geometry modulation (`chirality`, `lobeCount`, `axialTwist`, `filamentSpread`, `foldAsymmetry`) derived from the inputted program.

### Requirement: SVP-028 Program-derived temporal motion and harmonics
WHEN a `WovenBody` specimen is animated across time and CEK execution steps, its temporal phase cadence, harmonic oscillation frequencies (`temporalCadence`, `phaseHarmonic`, `breathDepth`, `waveVelocity`), and step-reactive excitation SHALL be modulated by the program's recursion/iteration structure and live CEK step telemetry (`fuel`, `stackDepth`, `activeRule`).

#### Scenario: Recursive vs iterative step cadence
- **WHEN** A deep recursive program or active CEK step executes in Designer 8.
- **THEN** The specimen's phase velocity, harmonic ripple, and active-territory luminescence respond deterministically to the program's motion signature and live CEK state.

### Requirement: SVP-029 Program-derived OKLCH chroma and iridescence
WHEN `Chroma.field(shape)` and `Chroma.colorFor(shape, index)` compute node and strand colors in the upstream Quinelings SDK (`chroma.js`, bundled into `quinelingsSdk.js`), they SHALL incorporate program-derived spectral digest parameters (`paletteRot`, `iridescenceSpan`, `luminanceFloor`, `chromaContrast`, `harmonicShift`) alongside topological role coordinates so each inputted program produces a rich, distinct OKLCH color palette.

#### Scenario: Distinct spectral signature per program
- **WHEN** `Chroma.colorFor(shape, idx)` evaluates colors across the 8 presets or user-edited `.mori` programs.
- **THEN** Each program exhibits a distinct, harmonious multi-stop OKLCH palette with program-dependent hue rotation, chroma contrast, and iridescence span.

### Requirement: SVP-030 Compilation LRU cache and lazy portrait frames
WHEN `compileSherlockProgramToQuineling(source, speciesCatalog, trace)` is invoked at 60Hz during rendering or across preset switches, it SHALL:
1. Return the cached artifact in O(1) via a 1-entry hot identity fast-path when `(rawSource, stackLimit, initialFuel, outcomeKind)` are reference-identical to the previous call, backed by a 64-entry LRU `Map`.
2. Lazily evaluate `Morphology.portraitFrame(shape)` via a getter (`get morphBounds()`) so unused 2D morphology bounds (211,680 trig calls) are never computed during 3D `WovenBody` compilation.

#### Scenario: Switching between presets in <1ms
- **WHEN** `resolveCompiledQuineling` is called every frame at 60Hz or when the user switches back to a previously visited preset.
- **THEN** The compiled Quineling artifact is returned in `<0.01ms` with zero string concatenation or re-compilation overhead.

### Requirement: SVP-031 WovenBody.frame typed-array reuse and basis cache
WHEN `WovenBody.frame(body, phase, { budget, crests, reuse })` executes in `quinelingsSdk.js` across any of the 8 mechanisms (`logarithmic-mantle`, `pleated-braid`, `recursive-affine`, `lorenz-flow`, `phyllotaxis-fan`, `clifford-flow`, `recursive-julia`, `toroidal-weave`), it SHALL:
1. Honor `reuse.points` (`Float32Array`) and `reuse.owners` (`Uint16Array`) across all 8 mechanisms (including `juliaFrame` and `recursiveFrame`) and write coordinates via direct indexed assignment (`points[p4] = ...`) rather than `points.set([x, y, z, a], p4)`.
2. Pre-index `territoriesByComponent`, `territoryByOwner`, `territoryByNode`, and `ownerMarks` at compile time for O(1) owner resolution.
3. Reuse per-strand `chartCache` / `_basisCache` `Map`s via `.clear()` and cache longitudinal neighbor bases (`ba.a`, `ba.b`) in `differential` / `caustics` to avoid redundant `Map.get()` and object allocations.

#### Scenario: Zero heap churn during 30Hz sampling
- **WHEN** `WovenBody.frame()` samples 8,192 points and caustic ridges at 30Hz.
- **THEN** `reuse.points` and `reuse.owners` are mutated in place, owner lookup runs in O(1), and `pleated-braid` / `lorenz-flow` caustic root-finding reuses row basis pairs without allocating temporary 4-element arrays.

---

### Requirement: SVP-032 WeakMap step telemetry and call-span memoization
WHEN `sherlockEngine.js` computes derived step telemetry (`enrichStepTelemetry`) or call-hierarchy spans (`computeCallSpans`) for a trace, it SHALL memoize results in `WeakMap` caches keyed by the `trace` and `step` object references so scrubbing or re-querying any step executes in O(1) after first visit.

#### Scenario: O(1) step lookup during timeline scrub
- **WHEN** The user scrubs the timeline slider across a 120-step trace multiple times.
- **THEN** `enrichStepTelemetry` and `computeCallSpans` return cached objects from `WeakMap` in O(1) without recomputing ring-slot provenance or rescanning the step array.

### Requirement: SVP-033 Singleton inactive arithmetic witness and 32-entry trace LRU
WHEN `sherlockEngine.js` evaluates a step that does not invoke a checked scalar arithmetic primitive, it SHALL reference a frozen `INACTIVE_ARITHMETIC_WITNESS` singleton rather than allocating a fresh 16-limb zeroed witness object, and SHALL cache up to 32 full program traces in an in-memory client LRU cache keyed by `(source, fuel, stackLimit, sliceWidth)`.

#### Scenario: Non-arithmetic step and preset reload
- **WHEN** A trace executes non-arithmetic steps (`Let`, `Var`, `If`, `Call`, `Match`, `Constructor`) or reloads a previously executed preset.
- **THEN** Non-arithmetic steps share `INACTIVE_ARITHMETIC_WITNESS` with zero limb-array allocations, and repeated preset traces resolve instantaneously from the 32-entry LRU cache.

### Requirement: SVP-034 Static-host detection and client-side trace fallback
WHEN the visualization runs on a static host (`*.github.io`, `file:`, or when `/api/trace` returns an error/404), `sherlockEngine.js` SHALL detect the static environment, skip futile `/api/trace` network round-trips once static mode is latched, and evaluate exact CEK traces directly in the browser via the client-side reference interpreter.

#### Scenario: GitHub Pages execution without backend
- **WHEN** The visualization is loaded from `https://charleshoskinson.github.io/Moriarty/visualization/` on GitHub Pages.
- **THEN** All 8 presets and custom `.mori` programs execute immediately in the browser without failed network delays or console errors.

### Requirement: SVP-035 Native Rust runner LTO and zero-copy lexer slices
WHEN the local native Rust trace runner (`rust-runner/`) is compiled and executed by `server.js`, it SHALL be compiled with `-C opt-level=3 -C lto=thin -C codegen-units=1 -C target-cpu=native` and tokenize `.mori` source via borrowed `&str` byte slices rather than allocating `Vec<char>` per lexing pass.

#### Scenario: Sub-millisecond native trace execution
- **WHEN** `rust-runner/sherlock-runner` executes a `.mori` program locally.
- **THEN** The optimized native binary lexes and evaluates the program in sub-millisecond time while emitting identical JSON trace records.

### Requirement: SVP-036 HTTP server Brotli/Gzip static cache and trace LRU
WHEN `server.js` serves static assets (`vendor/three.webgpu.js`, `src/**/*.js`, `index.html`) or responds to `POST /api/trace`, it SHALL serve pre-compressed Brotli/Gzip buffers from an in-memory static asset cache with `ETag` / `304 Not Modified` support and memoize `/api/trace` responses in a 32-entry in-memory LRU cache.

#### Scenario: Sub-millisecond asset and API serving
- **WHEN** A browser requests `vendor/three.webgpu.js` or posts a repeated `/api/trace` payload to `http://127.0.0.1:4377`.
- **THEN** Static assets return pre-compressed buffers (or `304 Not Modified`) from memory and `/api/trace` cache hits return in `<0.2ms` without spawning a child process.

---

### Requirement: SVP-037 In-place DOM node reuse for ring and rule HUD
WHEN `src/app.js` updates the 12-slot Ring Register HUD, 39-rule telemetry badges, continuation stack list, and timeline markers on step changes, it SHALL update existing cached DOM nodes in place (`textContent`, `classList.toggle`, `dataset`) instead of destroying and rebuilding subtrees via `.innerHTML = ...` on every step.

#### Scenario: 60fps timeline playback and scrubbing
- **WHEN** The user plays or scrubs a trace at 60fps.
- **THEN** `app.js` mutates only the changed text/class attributes of persistent DOM elements with zero DOM node destruction/creation churn.

### Requirement: SVP-038 CSS layout-style-paint containment and compositing
WHEN `src/styles.css` lays out the floating HUD panels, code editor drawer, ring register strip, and telemetry cards over the 3D WebGPU/WebGL `<canvas>`, independent HUD containers SHALL declare `contain: layout style paint` so text updates inside HUD badges never trigger full-document layout recalculations.

#### Scenario: Isolating HUD text mutations from canvas
- **WHEN** Step telemetry counters and register values update inside HUD cards.
- **THEN** Browser layout invalidation is strictly confined to the contained HUD panel subtree.

### Requirement: SVP-039 Relative importmap and modulepreload pipeline
WHEN `index.html` loads on either root (`/`) or a subpath (`/Moriarty/visualization/`), all stylesheet, script, and `<script type="importmap">` module specifiers (`three`, `three/webgpu`, `three/tsl`, `three/addons/`) SHALL use relative `./` paths and declare `<link rel="modulepreload">` for critical entry and vendor modules.

#### Scenario: Subpath deployment at /Moriarty/visualization/
- **WHEN** `index.html` is served from `/Moriarty/visualization/index.html`.
- **THEN** All ES modules and vendor bundles resolve relative to `./` without 404s, and critical modules preload in parallel.

### Requirement: SVP-040 Automated GitHub Pages static site integration
WHEN `npm --prefix site run build` (or `node site/scripts/build-docs.mjs`) executes in the `Moriarty` repository, the build script SHALL automatically copy the self-contained `visualization/` application (`index.html`, `src/`, `vendor/`) into `site/public/visualization/` so that `vite build` emits the complete interactive 3D visualization at `site/dist/visualization/` for GitHub Pages deployment.

#### Scenario: npm --prefix site run build produces dist/visualization/
- **WHEN** The GitHub Actions workflow `.github/workflows/site.yml` or a developer runs `npm --prefix site run build`.
- **THEN** `site/dist/visualization/index.html`, `site/dist/visualization/src/`, and `site/dist/visualization/vendor/` are generated and ready for static GitHub Pages hosting.

---

### Requirement: SVP-041 Quint-LLM-Kit GPU and heap memory lifecycle verification
WHILE the user switches presets, scrubs timelines, or toggles designers (`Designer 1`–`Designer 8`) arbitrarily many times, all GPU `BufferGeometry`, `Material`, `CanvasTexture`, and in-memory LRU caches SHALL satisfy a Quint-verified bounded-resource invariant (`specs/quint/visualization_memory_lifecycle.qnt`) with zero un-disposed GPU objects or unbounded Map growth.

#### Scenario: Bounded GPU textures, geometries and LRU caches across 1,000 scrubs
- **WHEN** `quint run` and `quint test` verify `visualization_memory_lifecycle.qnt` and the live application switches across all 8 presets and all 8 designers.
- **THEN** Active GPU geometries, textures, and LRU cache entries remain strictly bounded by their configured caps and no orphaned `BufferGeometry` or `CanvasTexture` leaks occur.

### Requirement: SVP-042 Quint-LLM-Kit CEK and Quineling cache aliasing verification
WHEN `sherlockEngine.js`, `sherlockQuinelingCompiler.js`, and `quinelingsSdk.js` serve cached traces, `WeakMap` step telemetry, or `WovenBody.frame()` `reuse` typed arrays, they SHALL satisfy a Quint-verified non-aliasing and determinism invariant (`specs/quint/engine_quineling_pipeline.qnt`) ensuring callers cannot corrupt cached traces or read stale typed-array slices across species switches.

#### Scenario: Cached trace and WovenBody artifacts never mutate or alias stale state
- **WHEN** `quint run` and `quint test` verify `engine_quineling_pipeline.qnt` and `WovenBody.frame()` switches between budgets or mechanisms.
- **THEN** All returned trace steps, telemetry records, and `points`/`owners` typed arrays match fresh un-cached evaluation bit-for-bit.

### Requirement: SVP-043 Quint-LLM-Kit async playback and static Pages state safety
WHEN rapid asynchronous preset loads, live editor compilations, timeline playback ticks, and Director mode transitions interleave on either local or static GitHub Pages hosts, `app.js` SHALL satisfy a Quint-verified request-epoch and state-consistency invariant (`specs/quint/ui_playback_lifecycle.qnt`) preventing out-of-order async responses from overwriting newer user selections.

#### Scenario: Rapid preset switching and static GitHub Pages fallback never desync UI
- **WHEN** `quint run` and `quint test` verify `ui_playback_lifecycle.qnt` and the user rapidly clicks through presets while playback is active.
- **THEN** Stale in-flight trace evaluations are discarded via monotonic request epoch counters and the HUD/3D stage remain strictly synchronized with the latest selected program.

