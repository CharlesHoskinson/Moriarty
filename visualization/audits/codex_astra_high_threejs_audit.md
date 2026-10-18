The stainless interposer and turned collar should remain the visual anchors. The largest improvements will come from **camera clearance, deliberate cutaways, studio reflections, and less work between execution steps**.

I inspected all ten files, with GPT-6 Astra/high reviews of the package, well, frames, and arena. I examined all six supplied screenshots and captured all nine camera presets using a temporary WebGL fallback. The headless WebGPU capture initialized but produced blank images; therefore, this report makes no GPU timing claims. No project files were changed.

The proposed lighting values and camera settings below are implementation starting points. Geometry defects and allocation findings are verified; the redesigned presentation still needs visual validation.

**1. [SherlockStage.js](/home/charles/sherlock-bench/src/stage/SherlockStage.js:80) — establish a reflection environment before changing the stainless materials.**

The scene has a background and six lights, but **no `scene.environment`**. Direct lighting creates highlights, but the metal lacks a structured studio environment to reflect. That limits the finish as the camera moves.

Add a dedicated studio environment after `await renderer.init()`:

```js
// THREE imported from 'three/webgpu'.
// studioScene contains a neutral enclosure and bright softbox cards.
this._pmrem = new THREE.PMREMGenerator(this.renderer);
this._environmentRT = this._pmrem.fromScene(
  studioScene, 0, 0.1, 30, { size: 256 }
);
this.scene.environment = this._environmentRT.texture;
this.scene.environmentIntensity = 0.75;
```

Use the WebGPU PMREM implementation already exported by the installed build. Its local source explicitly requires renderer initialization before `fromScene()`. Keep the environment render target alive until stage disposal; generate it once, outside the render loop.

For the environment composition:

- Place one broad neutral white card above/front-left to draw a continuous highlight across the interposer.
- Add a narrower cool white strip opposite it to describe the collar’s curvature.
- Include dark regions between cards. Silver needs reflected contrast to reveal its machining.
- Keep the existing warm background independent of the environment. Avoid turning the entire reflection field cream or gold.

After introducing IBL, start with these reductions:

| Parameter | Current | Initial tuning value |
|---|---:|---:|
| Hemisphere intensity | 0.85 | 0.30 |
| Key intensity | 2.55 | 1.80 |
| Cool fill intensity | 1.05 | 0.45 |
| Champagne rim intensity | 0.85 | 0.40 |
| Exposure | 1.12 | 1.00 |

Retain ACES initially so the environment change can be evaluated independently. Do not increase exposure to compensate for obstructed interior views.

The existing interior point at `(0, -0.78, 0)` changes the chamber’s principal illumination to saturated semantic colors. Separate architectural illumination from semantic signaling:

| Light | Position | Starting configuration |
|---|---|---|
| Neutral upper bounce | `(0, -0.40, 0.15)` | White point; intensity `0.35`, distance `2.2`, decay `2` |
| Lower chamber fill | `(0, -1.65, 0.25)` | Slightly cool white point; intensity `0.18`, distance `1.5`, decay `2` |
| Existing semantic light | `(0, -0.78, 0)` | Reduce to roughly `0.15–0.35`; blend color over `120–180 ms` |

These are scene-scale tuning values, not photometric calibration. Keep these lights unshadowed initially and watch for illumination leaking through opaque surfaces. A rectangular light is an alternative for a broad interior highlight, but WebGPU requires the LTC texture setup, and rectangular lights do not cast shadows. [Three.js RectAreaLight documentation](https://threejs.org/docs/pages/RectAreaLight.html)

For shadows, the current configuration is **one 2048² directional shadow map, not cascades**. Its width of `9.2` units gives approximately `0.00449` world units per texel. The `normalBias = 0.018` is about four texels and large relative to several modeled details.

Recommended tuning:

- Fit the light-space shadow bounds to the package, visible chiplets, and relevant receivers, with approximately 10% padding.
- Start `bias` near `-0.00005` and `normalBias` near `0.003`; inspect contact separation and acne before increasing either.
- Tighten shadow near/far using projected bounds rather than retaining `1.5…26`.
- Use `light.shadow.autoUpdate = false` only after identifying all changing casters. Set `needsUpdate` when caster transforms, visibility, cutaways, or lighting change.
- Keep thin light pipes, labels, and decorative microgeometry out of the caster set.

I would not add cascades first: this is a compact product scene. If measured close-up shadow quality still requires them, use the WebGPU `CSMShadowNode`, with two cascades and a restricted useful distance—not the WebGL CSM implementation. Each cascade adds shadow rendering work. [Three.js CSMShadowNode documentation](https://threejs.org/docs/pages/CSMShadowNode.html)

Other stage corrections:

- `_fullState` avoids allocating a new wrapper each render, but `Object.assign()` can retain fields absent from later inputs. Define and overwrite the complete state contract.
- Hidden subsystems still update at lines 309–313. Separate semantic synchronization from visible animation so hidden layers can skip continuous work and resynchronize when shown.
- Use the animation loop’s timestamp and `dt`; the stage currently maintains a second clock.
- Report initialization failure rather than silently leaving `ready = false`.
- Precompile the relevant material variants before the first interior transition.

**2. [theme.js](/home/charles/sherlock-bench/src/designers/theme.js:129) — distinguish physical material properties from decorative texture detail.**

`createBrushedMetalTexture()` generates a color texture. Its `texture.anisotropy = 8` improves texture sampling; **it does not create anisotropic metal reflections**.

Introduce shared material recipes, then adopt them selectively:

| Surface | Suggested physical treatment |
|---|---|
| Existing stainless interposer | Preserve its current color and brushed maps; test material anisotropy around `0.35` |
| Turned collar | Preserve its smoother appearance; test anisotropy around `0.15–0.25` with circumferential direction |
| Obsidian shield | Roughness `0.38–0.46`; metalness `0` if intended as a dielectric coating, retaining `0.22` only if the metallic appearance is intentional |
| Alabaster ceramic | Metalness `0`; retain subtle grain, roughness around `0.38`, restrained clearcoat |
| Chamber walls | Neutral dark finish with sufficient roughness variation to reveal edges under white illumination |

For the interposer, current roughness is multiplied by its roughness map: `0.26 × roughly 0.79–0.88`, producing approximately `0.21–0.23` before other shading effects. Preserve that effective finish during the first IBL pass.

Material anisotropy needs coherent UV/tangent directions. A uniform rotation on poorly oriented UVs will not produce convincing turned machining. Color textures belong in sRGB; roughness, normal, bump, and anisotropy data belong in `NoColorSpace`. Three.js explicitly distinguishes these properties. [MeshPhysicalMaterial documentation](https://threejs.org/docs/pages/MeshPhysicalMaterial.html)

The plaque cache is not a bounded GPU-memory solution:

- It limits entries to 1,024.
- Eviction removes the lookup entry without disposing the texture.
- Active materials may still reference evicted textures, so unconditional eviction disposal would also be wrong.
- A `640 × 140` RGBA plaque contains about **350 KiB of base pixels**. A hypothetical 1,024 such textures would contain **350 MiB**, before mipmaps and canvas storage. This is an illustration, not measured occupancy.

Use either reusable per-slot canvases/textures or an atlas with explicit ownership. If retaining the cache, track active references and dispose unreferenced evictions. Define cache-level teardown for metal, ceramic, reticle, and plaque resources.

Finally, centralize semantic color selection using the **whole frame**, including Args evaluation progress. D2 and D3 currently disagree.

**3. [designer1_package.js](/home/charles/sherlock-bench/src/designers/designer1_package.js:451) — preserve the successful finish and reduce update overhead.**

The current stainless treatment is already intentional:

- Interposer: color `0xa7aaad`, roughness `0.26`, metalness `0.88`, brushed color/roughness maps.
- Collar: roughness `0.28`, metalness `0.90`, smoother surface without those maps.
- Both use restrained clearcoat.

Keep those differences. Add the reflection environment before changing their colors.

The exact collar geometry matters:

| Feature | Actual geometry |
|---|---|
| Nominal bore | Radius `1.06` |
| Collar outer edge | `(r=1.16, y=0.108)` |
| Flat lip inner edge | `(r=1.11, y=0.108)` |
| Bevel throat | `(r=1.06, y=0.082)` |
| Throat bottom | `(r=1.06, y=-0.12)` |

Thus, `1.06` is the throat radius, not the collar’s outer radius. The package tiers and bench have genuine holes; this is not a painted recess.

The top shield’s actual beveled bounds are approximately `y=-0.035…0.105`, with outer half-width `1.835`. These bounds explain the camera collision discussed below.

Performance findings:

- `extractFrontValueStatus()` creates an object every update; `frontKey` also formats text every frame before the canvas-change guard.
- Move these operations behind a trace/step/status revision check.
- Lead instance matrices are correctly initialized once. **Do not rewrite this as a matrix-upload problem.**
- Lead colors invalidate a 288-byte attribute every update, including constant states. Skip unchanged colors; retain animated writes when the lead wave is active.
- Freeze local matrices for genuinely static package components after construction.

The package has only 17 meshes and modest geometry. Removing its bevels would sacrifice a successful visual feature before addressing the larger costs elsewhere.

**4. [designer2_well.js](/home/charles/sherlock-bench/src/designers/designer2_well.js:2116) — simplify the chamber’s rendering structure and make depth legible.**

The six ring positions are more precise than their headline range suggests:

| Ring | Inner radius | Outer radius | Base Y |
|---|---:|---:|---:|
| R1 | 0.858 | 0.950 | 0.060 |
| R2 | 0.730 | 0.822 | 0.006 |
| R3 | 0.602 | 0.694 | -0.048 |
| R4 | 0.474 | 0.566 | -0.102 |
| R5 | 0.346 | 0.438 | -0.156 |
| R6 | 0.232 | 0.310 | -0.210 |

Idle ring groups settle `0.016` below these values; occupied groups settle `0.012` above them. Each body additionally extends locally from `-0.032` to `+0.004`.

The stator is at `y=-0.24`, but its real clear opening is smaller than the stated `r=0.135`:

- Sapphire annulus begins at `r=0.127`.
- During a pop, the needle stem can reach inward to approximately `r=0.120`.

Use those dimensions for camera clearance.

Verified update costs:

| Attribute group | Bytes invalidated per update |
|---|---:|
| Fuel colors | 576 |
| Clasp matrices and colors | 1,824 |
| Stator matrices and colors | 760 |
| Floor matrices and colors | 2,964 |
| **Total** | **6,124** |

That is approximately 367 KB/s at 60 Hz—modest bandwidth, but accompanied by **73 matrix writes and 121 color writes every update**. Optimize redundant computation and resource invalidation rather than presenting this as bandwidth saturation.

Recommended changes:

- Update stator matrices when sector selection changes.
- Update floor matrices when tile elevation/state changes.
- Update clasp colors only when semantic colors change.
- Keep animated attributes dynamic; use dirty ranges where worthwhile.
- Gate plaque/environment string formatting before building signatures.
- Replace temporary `{ kind: … }` objects in semantic classification with direct arguments.

The chamber contains 165 mesh objects. Its eight terrace assemblies account for **96 meshes**: walls, trim, and light loops. Merge static geometry by material while retaining lightweight picking proxies. This is a better first optimization than stripping the rings of detail.

There is also a concrete floor defect: the active tile rises to **`y=-2.1686`**, while the shared label plane remains at **`-2.171`**. The active tile therefore covers its own inscription. Keep tile tops below the label plane or lift the overlay above the maximum animated height.

For the interior architecture:

- Retain dark walls, but add restrained stainless edge reveals and neutral grazing illumination.
- Make only the active semantic terrace strongly colored.
- Add a narrow vertical datum/light spine to establish the chamber’s height.
- Keep the 39-rule floor visually subordinate until it is the focus.
- Use baked/local occlusion to anchor corners. Emissive strips alone do not illuminate neighboring geometry.

Replace ring interpolation such as `clamp(dt * 14, 0.08, 1)` with `1 - exp(-λdt)`. The minimum per-frame factor makes behavior refresh-rate dependent, especially at high refresh rates.

**5. [designer3_frames.js](/home/charles/sherlock-bench/src/designers/designer3_frames.js:549) — the wafers and plaques are the principal interior obstruction.**

Each wafer is approximately `1.24 × 1.16`, with a central opening of radius `0.16`. Settled centers occupy `y=-0.52…-1.92`.

The current interior eye is:

```text
position = (0.52, -0.52, 0.56)
target   = (-0.06, -1.22, -0.06)
FOV      = 48°
```

It sits at the first wafer’s height, beside its front corner. The large near-horizontal plaque surfaces consequently fill the view. That matches the supplied interior screenshot.

The “glass” is alpha-blended at opacity `0.80`, with double-sided rendering. As a simple compositing illustration, five fully overlapping layers retain only `0.2⁵ = 0.032%` of the background contribution. The label backgrounds are nearly opaque too. Raising the lights cannot expose what these surfaces cover.

Recommended presentation:

- Preserve stainless wafer bevels and the central collars.
- Use a controlled front cutaway in interior mode.
- Show detailed text on the selected wafer; use compact slot identifiers on others.
- Move full expressions and environment bindings to the inspector.
- Avoid making twelve overlapping transmissive sheets the default solution; that adds rendering cost without guaranteeing readability.

The hot path is not allocation-free. On an unchanged 12-wafer state, the probe recorded **12 `slice()`, 12 `map()`, and 12 `match()` calls**. `updateWaferPlaque()` formats the title before testing its signature, then sometimes formats it again for inspector metadata.

Perform label and metadata work once per frame revision. Keep only animation math in the continuous update.

Animation fixes:

- Wafer damping already uses the frame-rate-independent exponential form; retain it.
- Slot spacing changes with stack size, moving established wafers as the stack grows. Prefer stable depth slots or an explicitly choreographed compression event.
- Popped wafers disappear immediately. Retain the outgoing wafer during a short release transition.
- The orb uses `orbT = 0.45` at `stepFraction <= 0.005`, then jumps back near zero when the fraction advances. Remove this discontinuity.
- Give the orb an explicit departure/arrival timeline and deterministic scrubbing behavior.

**6. [designer4_arena.js](/home/charles/sherlock-bench/src/designers/designer4_arena.js:173) — fix call selection before polishing its motion.**

The subsystem evaluates every call but always displays the first six. A seven-level synthetic trace leaves the active seventh call invisible while all six displayed calls are waiting.

Keep the six-chiplet composition, but select a stable window containing the active call and relevant ancestors. Indicate overflow explicitly.

A separate verified failure occurs for rejected traces without function calls: fallback construction assigns a return step, then resolved styling takes precedence over error styling. The chiplet can display a successful green return for an error. Model `returned`, `rejected`, and `exhausted` independently.

Performance improvements:

- Refresh call status on integer-step changes.
- Refresh layout only when the visible call selection changes.
- Avoid creating a result object and signature strings every render.
- Retain the existing guarded canvas updates.
- Scope the trace cache to the subsystem or release it on disposal.

There is no continuous interpolation here today. A short status-color transition and restrained vertical emphasis would be sufficient; repeated ambient motion would distract from the call sequence.

The diagnostic arena capture crops the west chiplets. Frame their actual bounds, with the package as context, rather than keeping the well dominant.

**7. [designer5_circuit.js](/home/charles/sherlock-bench/src/designers/designer5_circuit.js:154) — retain its efficient update structure; improve observability.**

This is one of the stronger implementations:

- Instance matrices are initialized once.
- Colors update only when relevant state changes.
- Scratch objects and empty-array fallbacks are reused.
- No evident application-level allocation occurs in the normal steady update path.

The visual problem is scale. LEDs are only `0.018` units wide and `0.0012` high. In the diagnostic Aux/2 view, the broad shield and rings dominate while the intended subject is nearly invisible.

Recommended changes:

- Center Aux/2 on the real right-flank coordinate, `x=1.66`, rather than `x=1.15`.
- Use a dedicated macro view and a compact magnified readout.
- Preserve physical LED dimensions in overview; use a deliberate detail presentation instead of globally enlarging them.
- Check the `exprId % 16` mapping: it represents an address bucket, not necessarily an unambiguous expression row. Explain or change that representation.
- Replace the visibly bucketed ripple with continuous intensity only if the macro view warrants it.

**8. [designer6_loom.js](/home/charles/sherlock-bench/src/designers/designer6_loom.js:97) — fix progress normalization.**

Geometry and color updates are already guarded appropriately. However, `totalSteps` is clamped to 32 before clamping the actual `stepIndex`.

For the supplied 145-step trace, the display reaches its final segment by step 32 and then remains there.

Separate execution length from display capacity:

```js
const traceLength = /* actual total steps */;
const segmentCount = Math.min(32, traceLength);
const progress = traceLength <= 1 ? 1 : stepIndex / (traceLength - 1);
const activeSegment = Math.min(
  segmentCount - 1,
  Math.floor(progress * segmentCount)
);
```

Define how `ReferenceSlice` boundaries aggregate into those segments; they cannot remain literal step boundaries after compression.

The actual bar is at `z=-1.66`, while metadata still mentions `-1.92`. Correct that and frame the rear track explicitly. The current Loom capture gives much more space to the rings than to the progress bar.

**9. [designer7_cinema.js](/home/charles/sherlock-bench/src/designers/designer7_cinema.js:270) — replace the dive with a collision-aware sequence and implement a real section view.**

The immediate 70% advance in `setCameraPreset()` is the largest transition defect. It neither guarantees clearance nor provides a smooth beginning.

A 60 Hz simulation of Overview → Interior produces approximately:

```text
First update:       (1.375,  0.856, 1.649)
After six updates:  (0.837, -0.009, 0.954)
Final:              (0.520, -0.520, 0.560)
```

At the second position, horizontal radius is about `1.269`: outside the `1.06` bore and inside the shield’s solid footprint and vertical extent. The camera passes through the package.

The later radial clamp cannot prevent this. It activates only after the camera is below `y=-0.22`, and it checks neither wafers nor the actual aperture profile.

Use these initial destination settings:

| View | Position | Look-at | Vertical FOV | Near / far |
|---|---|---|---:|---|
| Interior architectural view | `(0, -0.39, 0.78)` | `(0, -1.13, -0.08)` | `64°` | `0.015 / 30` |
| Actual cross-section | `(3.0, 1.25, 4.0)` | `(0, -1.0, 0)` | `42°` | `0.03 / 40` |
| Top-down rings, with margin | `(0, 4.45, 0.28)` | `(0, -0.14, 0)` | `34°` | `0.03 / 40` |

These destinations require the visibility treatments below; changing coordinates alone is insufficient.

For the interior dive, use approximately 1.8 seconds:

1. **Approach above the aperture:** travel from the current eye to `(0, 1.10, 0)` on a curved path that remains above the package.
2. **Descend through the bore:** travel axially to `(0, -0.39, 0)`.
3. **Move onto the viewing position:** travel at `y=-0.39` to `(0, -0.39, 0.78)`, then settle the view toward the target.

At `y=-0.39`, the camera is below the ring pack and above the highest transient wafer plaque. The measured vertical clearance to that plaque is approximately `0.063`; the chamber wall is farther out at about `±0.986`.

Use a swept camera volume, including near-plane corners, rather than a point test. Validate all intermediate positions and transition interruptions. Coordinate temporary foreground/orb visibility during the bore passage.

For scripted travel, a constrained Bézier path with quintic timing works well:

```js
const s = t * t * t * (10 + t * (-15 + 6 * t));
```

For interactive orbit settling, the current exponential damping is reasonable. A critically damped spring becomes useful when retaining velocity across changing targets. Neither damping method substitutes for collision constraints.

Interpolate orientation with quaternions and preserve a stable azimuth during the axial descent; a straight-down `lookAt` with the usual up vector can become singular.

The **interior cutaway** should remove the front wafer portions beyond approximately `z=0.15`. That cut connects to the central openings and clears the central floor sightline in the tested stack. A cut at `z=0.18` still leaves a lower-wafer obstruction.

For the installed WebGPU renderer:

```js
const waferCut = new THREE.ClippingGroup();
waferCut.clippingPlanes = [
  new THREE.Plane(new THREE.Vector3(0, 0, -1), 0.15)
]; // retain z <= 0.15
waferCut.clipShadows = true;
```

Apply this to the relevant wafer geometry, with label handling separately. Preserve rear stainless rims, add intentional cut-edge treatment, and restore the full geometry on exit. `ClippingGroup` is the appropriate WebGPU scene-graph mechanism. [Three.js ClippingGroup documentation](https://threejs.org/docs/pages/ClippingGroup.html)

This exposes the central/front floor, not every far-side tile. A full-floor detail mode should temporarily suppress obstructing wafers and labels.

For **Cross-Section**, create an actual camera-facing section through the shell and chamber walls. With the proposed eye, a suitable horizontal outward direction is approximately `(0.6, 0, 0.8)`. Clip shell geometry on that side of a plane offset about `0.20` from the center. Include the ground/bench and upper package where they obstruct the sightline; cutting only the lower wall is insufficient.

Keep the retained collar and interposer portions opaque, and cap section boundaries. A whole-object transparency fade would weaken the stainless appearance and introduce overlapping transparency problems.

Picking and reticle corrections:

- `intersectObjects(scene.children, true)` allocates a results array and traverses unnecessary geometry. Use an explicit pick list/layer and a reusable result array; intersection records can still allocate.
- Filter hidden components before intersection work, rather than only afterward.
- Cache selection bounds instead of calling `Box3.setFromObject()` every frame for a static target.
- Update hover when the camera moves beneath a stationary pointer.
- Reset or conservatively define the reticle’s bounds after changing its vertices.
- Handle `pointercancel` and interrupted gestures.
- Snap FOV to its final value once within tolerance; it currently stops slightly short.

**10. [app.js](/home/charles/sherlock-bench/src/app.js:878) — remove trace reconstruction from the display loop.**

This is the highest-priority CPU change.

`animate()` calls `buildStepState()` every frame, including while paused. That calls `buildRecursionTelemetry()`, which scans **the complete trace**, constructs calls and stacks, and formats expressions. Ring telemetry also creates arrays and objects.

The stage’s reused wrapper does not eliminate these upstream allocations.

Split state preparation into three lifetimes:

| Lifetime | Work |
|---|---|
| New trace | Parse call relationships, immutable metadata, maximum depth |
| New integer step | Select frames, semantic state, labels, ring telemetry, inspector content |
| Display frame | Update `dt`, ambient time, transition fraction, camera and active animation |

Cache semantic results by trace identity/revision and step. Mutate a small reusable animation-state object between steps.

Also:

- Keep the existing step-change gate around DOM synchronization.
- Debounce slider-triggered execution and reject stale async results; overlapping requests can currently complete out of order.
- Render camera interactions even before a successful trace exists.
- Respect reduced-motion preferences and pause unnecessary rendering when hidden.
- Display the actual rendering backend/status. The “WebGPU” badge is currently assigned independently of renderer success.

The dependency mismatch also needs correction before implementation: `package.json` requests `^0.175.0`, while the lockfile and installed package are `0.186.1`; the browser’s vendored core reports revision `186`. Align the manifest, lockfile, vendor files, and addon imports.

The recommended implementation order is:

1. Fix the camera’s package collision, interior destination, and explicit cutaway.
2. Remove per-frame telemetry reconstruction and guard label formatting earlier.
3. Add studio IBL while preserving the existing stainless materials.
4. Correct the floor-label occlusion, orb discontinuity, progress normalization, and arena selection/error states.
5. Merge static chamber meshes, reduce unnecessary shadow updates, and optimize picking.

Acceptance should include screenshots at 0, 5, and 12 visible wafers; transition checks in both directions and during interruption; consistent motion at 30/60/120 Hz; and allocation profiling while paused. Measure WebGPU CPU/GPU timings on the target hardware—the diagnostic captures here establish composition and geometry issues, not production performance.