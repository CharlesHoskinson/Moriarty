# Audit 02 — Lighting, Materiality & WebGPU Pipeline

Auditor: Grok 4.7 xhigh, Apple Industrial Design lighting, materiality, and WebGPU pipeline.
Scope: `src/stage/SherlockStage.js`, `src/designers/theme.js`, `src/designers/ring_study_05_chip_materials.md`, `src/designers/ring_study_06_lighting.md`, checked against the package, well, frame, and call-tile materials that consume them.
Runtime: `three@0.186.1` (`three/webgpu`). Three r186 has no legacy-light switch. Direct diffuse is Lambert `albedo / π`. `PointLight` intensity is candela with Frostbite windowed decay. `MeshPhysicalMaterial.anisotropy` and `RectAreaLight` are available. `PCFSoftShadowMap` is removed. `NeutralToneMapping` and `AgXToneMapping` exist.

Verdict: the stage builds the six lights, the exposure, the fog, and the palette tokens the studies name. The silver-platinum deck still cannot read as brushed titanium, the alabaster body is the wrong grain at the wrong value, and the plaque cache can hand a live material a texture another subsystem has already disposed. Fix the two texture multiplies and the shared-cache ownership before retuning the rig. Then give the metals an environment and a real anisotropic lobe.

---

## What already holds

- One shadow caster. `keyLight.castShadow = true` in `SherlockStage._setupLights`. Fill, rim, and both practicals stay analytic. That is the right WebGPU budget.
- `PCFShadowMap` is the type this renderer still implements. `Renderer` warns and downgrades `PCFSoftShadowMap`.
- Output is `SRGBColorSpace` with `ACESFilmicToneMapping`. The canvas path is a real HDR chain (`outputBufferType` defaults to half-float).
- Pixel ratio is capped at 2. `powerPreference: 'high-performance'` is set.
- Studio lights live on `scene`. The well practical and the core practical live on `this.root`, so they travel with the package. That split is correct.
- Phase tint uses `this._scratchColor` allocated in the constructor. No per-frame `Color` allocation on that path.
- Roughness and bump canvases in `designer1_package.js` use `NoColorSpace`. Albedo canvases use `SRGBColorSpace`. That part of the color pipeline is right.
- Plaque redraws in the well are gated on a content key (`designer2_well.js`), so a steady frame does not rebuild a canvas.
- Hemisphere ground `0xd8d0c2` under a white sky is the right idea: overhead diffuser, warm table bounce.

---

## 1. WebGPU renderer

`SherlockStage` constructor, lines 82–97 and 115–123.

| Setting | Current | Assessment |
| :--- | :--- | :--- |
| Backend | `WebGPURenderer`, WebGL2 fallback inside three | Correct entry point. |
| MSAA | `antialias: true` → 4 samples (`WebGPURenderer` options) | Expensive at DPR 2 on a full-viewport half-float target. |
| Depth | default, `reversedDepthBuffer` off, camera `near = 0.1`, `far = 100`, fov 34° | Too coarse for a 0.006–0.007 bevel once a preset dollies in. |
| Alpha | default `alpha: true` | Opaque paper backplate does not need a transparent framebuffer. |
| Init failure | `catch` sets `ready = false` and drops the error | A dead GPU looks like a frozen stage. |
| Dispose vs init | `dispose()` skips `renderer.dispose()` when `ready` is still false | `init()` can resolve after dispose, set `ready = true`, and leak the device. |

`render()` only draws when `this.ready === true`. Frames before `init()` resolve are dropped. That is fine. The race is not.

`scene.fog` is `FogExp2(0xf4f0e8, 0.024)`. At the overview camera (~7.3 units) the extinction is about 3%. The studies are right that this keeps the package sharp. It does not separate the package from the backdrop, because the fog color, the clear color, and the bench paint are the same family (`0xf4f0e8`, bench gradient `rgb(243, 239, 230)`).

Camera fov is 34° (about a 40 mm full-frame vertical angle) and no preset in `designer7_cinema.js` changes it. Horological macro wants a longer lens, about 18–22° vertical, so the six rings do not bow at the edges. Leave 34° for the overview if the wide read is intentional. Tighten it on `well`, `frames`, and `interior`.

### Next iteration — renderer

```js
this._disposed = false;

this.camera = new THREE.PerspectiveCamera(34, 1, 0.02, 80);

this.renderer = new THREE.WebGPURenderer({
  canvas: this.canvas,
  antialias: true,
  samples: 2,
  alpha: false,
  reversedDepthBuffer: true,
  powerPreference: 'high-performance',
});
this.renderer.outputColorSpace = THREE.SRGBColorSpace;
this.renderer.toneMapping = THREE.NeutralToneMapping;
this.renderer.toneMappingExposure = 1.02;

async _initRenderer() {
  try {
    await this.renderer.init();
    if (this._disposed) {
      this.renderer.dispose();
      return;
    }
    this.ready = true;
    this.resize();
  } catch (err) {
    this.ready = false;
    console.error('WebGPURenderer.init failed', err);
  }
}

dispose() {
  this._disposed = true;
  // ...existing listener and subsystem teardown...
  this.renderer?.dispose();
}
```

Measure `samples: 2` against `samples: 4` on the target GPU before locking it. Reversed-Z is the higher-value switch: it protects the chamfer and the wafer stack when `near` drops to 0.02. Neutral tone mapping keeps silver neutral. ACES Filmic (the study’s `1.18`) compresses highlights toward a warm shoulder and dulls the cool fill that is supposed to define the platinum. If the grade must stay ACES, drop exposure to about `1.00` after the hemisphere is lowered (section 2). Do not stack ACES `1.18` on top of an environment map.

`Object.assign(this._fullState, stepState)` never clears a previous `semanticTransition`. A later step that only sends `phase` leaves the well practical on the old tint. Set `fullState.semanticTransition = stepState.semanticTransition ?? null` each frame before resolving the phase.

---

## 2. The studio rig

`_setupLights` matches ring study 06 number for number.

| Role | Study & code | Shader fact in r186 |
| :--- | :--- | :--- |
| Hemisphere | `(0xffffff, 0xd8d0c2, 1.55)` | Added as diffuse irradiance only. No specular. Up-facing albedo sees the full sky term, then `/ π`. |
| Key | `(0xfffcf5, 2.55)` at `(4.8, 8.6, 5.2)`, shadow 1024, ortho `±4.2`, `near 1`, `far 20`, `bias -0.0004` | Direct term is `NdotL * intensity`, then Lambert `/ π`. |
| Fill | `(0xdbeafe, 1.35)` at `(-5.2, 5.8, 3.6)` | Same direct path. Ratio to key is 0.53 before the hemisphere. |
| Rim | `(0xfbbf24, 1.25)` at `(0, 4.2, -5.6)` | A directional has no cone. Every normal that faces `-Z` receives saturated gold, including ceramic walls and the bench. |
| Well practical | `PointLight(0x2dd4bf, 2.2, 5.5, 1.4)` at `(0, -0.85, 0)` | 4th argument is `decay`. r186’s physical default is `2`. `1.4` is a custom power. |
| Core practical | `PointLight(0xffffff, 1.35, 3.5, 1.5)` at `(0, 0.85, 0.25)` | Same. `decay` 1.5. |

On an upward white Lambert surface the key at `NdotL ≈ 0.75` contributes about `2.55 * 0.75 / π ≈ 0.61`, and the hemisphere contributes about `1.55 / π ≈ 0.49`. The wash is ~80% of the key. Chamfers stay open and the package goes flat. That fights both Rams instrumentation and Apple product stills, which keep the fill closer to 1:4 and let the contact shadow draw the footprint.

The key elevation is `atan2(8.6, hypot(4.8, 5.2)) ≈ 50°`, and its azimuth sits near the camera (`camera` at `(0.15, 5.15, 5.1)`). The top deck is lit almost full-face. A product key wants to come from the side, around 35–40° elevation, so the deck’s grain and the well lip both have a readable terminator.

Shadow map: ortho width `8.4` over `1024` texels is `0.0082` units per texel. Package bevels are `0.006–0.007` (`designer1_package.js`). The shadow of the gold seam is smaller than a texel. `LightShadow.radius` defaults to `1` and `normalBias` defaults to `0`, so the PCF Vogel disk (`ShadowFilterNode.js`) is one texel wide and the bias is a constant depth hack. `normalBias` is the control that should hold a curved chamfer out of acne.

The bench extends to `±5.4`. The shadow camera covers `±4.2`. Outer contact is whatever the ground canvas painted. That paint is a centered radial blob (`rgba(24, 32, 44, 0.12)` under the body), not a shadow offset opposite the key. The eye reads two light directions at once.

There is no `scene.environment`. Hemisphere light does not feed indirect specular (`HemisphereLightNode` only adds irradiance). With no PMREM, metal highlights are only the five analytic sources. Brushed titanium in this reference world is a long reflection of a large softbox. A directional cannot make that lobe. `RectAreaLight` / `RectAreaLightNode` is in this three build. `RoomEnvironment` is at `three/addons/environments/RoomEnvironment.js`.

Phase tint (`render`, lines 269–287): `ERROR → 0xe11d48` at intensity `2.4`, `ENV | RETURN | RESOLVE | EMBLEM → 0x2dd4bf` at `2.2`, everything else `0xfbbf24` at `2.1`. `GATE` falls through to gold. Ring study 05 makes violet-indigo a primary family. The practical inside the well should agree. The intensity steps (`2.4 / 2.2 / 2.1`) are smaller than what ACES will show. Color has to carry the state. On `ERROR`, also drop the hemisphere for that frame so the crimson is not swimming in 1.55 of white sky.

Study 05 still says “4-light WebGPU studio rig”. Study 06 says “5-point” and then specifies six lights. `DESIGNER_LAYERS` in `SherlockStage.js` still describes “Chamfered Space-Black Ceramic”. Those strings will send the next pass back to the dark palette.

### Next iteration — rig

Keep six sources. Change their jobs. Numbers below are for three r186’s shader (directional intensity is a linear irradiance multiplier, point intensity is candela, decay must be 2). Re-tune exposure only after this, on a gray-ceramic sphere and a metal sphere side by side.

```js
const hemi = new THREE.HemisphereLight(0xf3efe6, 0xc9c0b2, 0.45);

const keyLight = new THREE.DirectionalLight(0xfff4e6, 2.15);
keyLight.position.set(6.4, 4.8, 3.2); // ~37°, camera-right
keyLight.castShadow = true;
keyLight.shadow.mapSize.set(2048, 2048);
keyLight.shadow.camera.near = 0.5;
keyLight.shadow.camera.far = 24;
const d = 3.2; // package + immediate bench, not the whole world
keyLight.shadow.camera.left = -d;
keyLight.shadow.camera.right = d;
keyLight.shadow.camera.top = d;
keyLight.shadow.camera.bottom = -d;
keyLight.shadow.bias = -0.00008;
keyLight.shadow.normalBias = 0.015;
keyLight.shadow.radius = 3.5;
keyLight.target.position.set(0, -0.05, 0);
this.scene.add(keyLight.target);

const fillLight = new THREE.DirectionalLight(0xe6eef8, 0.52);
fillLight.position.set(-4.6, 3.0, 3.8);

const rimLight = new THREE.DirectionalLight(0xfff1df, 0.72);
rimLight.position.set(0.3, 2.2, -6.4);
```

Gold leaves the rim. `0xfbbf24` on a directional paints the rear ceramic orange and cancels the cool fill on the titanium. Champagne stays a material (`PALETTE.gold` on the seam, pads, and clasps). If a golden edge kiss is still required, add a `SpotLight` with `angle ≈ 0.32`, `penumbra ≈ 0.85`, color `0xf0d7a2`, aimed at the rear lip only.

```js
this.wellPointLight = new THREE.PointLight(0x2dd4bf, 16, 4.2, 2);
this.wellPointLight.position.set(0, -0.85, 0);

this.coreSpecLight = new THREE.PointLight(0xfff8f2, 10, 2.0, 2);
this.coreSpecLight.position.set(0, 0.42, 0.12);
```

At decay 2 the old intensities (`2.2` and `1.35`) collapse inside a well whose walls sit ~1 unit away. The raised candela values put the practical back in the same visual range, with a real inverse-square falloff so the ruby and the inner rings pick up a jewel glint and the outer ceramic does not.

Phase map:

| Phase | Well color | Well intensity | Note |
| :--- | :--- | :--- | :--- |
| `ERROR` | `0xe11d48` | `22` | Also set hemi intensity to `0.22` for that frame. |
| `ENV`, `RETURN`, `RESOLVE`, `EMBLEM` | `0x2dd4bf` | `16` | |
| `GATE` | `0x818cf8` | `16` | New. Do not fall through to gold. |
| `OP`, `DESCEND`, default | `0xfbbf24` | `14` | |

Environment, once:

```js
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js';

const pmrem = new THREE.PMREMGenerator(this.renderer);
this.scene.environment = pmrem.fromScene(new RoomEnvironment(), 0.04).texture;
pmrem.dispose();
```

Apply `envMapIntensity` per material: `0.85` on titanium and gold, `0.35` on porcelain, `0.55` on the gunmetal well. A room probe is a stand-in. The better target is a three-softbox HDR (large warm key, smaller cool fill, thin warm rim) so the anisotropic highlight has a rectangle to stretch. A `RectAreaLight` matching the key azimuth (`width ≈ 3.5`, `height ≈ 2.2`, color `0xfff4e6`) supplies the wrapped specular the directional cannot. Keep the directional as the shadow caster only.

Painted bench shadow: move the radial gradient’s center about `0.22` units toward `(-0.55, -0.48)` in world XZ (opposite the new key) and keep the 3D shadow frustum tight. One shadow direction.

Update ring study 06 to these numbers in the same change. Leave the study on `decay 1.4` and the code will be “fixed” back.

---

## 3. Material palette

### 3.1 Two multiplies that erase the spec

`MaterialNode` in three r186 builds:

- diffuse = `material.color * map`
- roughness = `material.roughness * roughnessMap.g`

The deck in `designer1_package.js` (lines 696–706) sets `color: 0xd8dde6` and `map` from `createBrushedMetalTexture({ baseHex: '#d8dde6' })`. The same hex is applied twice. In linear light that squares the albedo. `#d8dde6` (linear roughly `0.69, 0.76, 0.82`) becomes roughly `0.47, 0.58, 0.67`, a darker, cooler gray. The ceramic body does the same with `0xebe5da` × a canvas already filled `#ebe5da` (lines 504–512), so the porcelain lands near `#d9ceb8` and picks up a dirty tan the study does not specify.

The roughness map is worse. `getBrushedSilverPlatinumTextures` fills `rgb(62, 62, 62)` and is tagged `NoColorSpace`, so the sample is `62/255 ≈ 0.243`. The material then multiplies by `roughness: 0.24`:

- mean roughness ≈ `0.24 × 0.243 ≈ 0.058`
- streak range in that canvas is about `28…110`, so effective roughness ≈ `0.026…0.104`

The spec’s satin `0.24` never reaches the shader. The deck is polished metal. Combined with `metalness: 0.78` and no environment, the body goes dark and the only bright events are tiny specular pins from the directionals. The brush canvas cannot save it: streak alpha is ~`0.04–0.10`, so the albedo grain is a faint tint, and nothing stretches the highlight along the grain.

The same texture is the bump map at `bumpScale: 0.0015`. Bump wants mean height `0.5`. Mean `0.24` is a global dent, and the deviation is too small to read as machining.

Rings repeat the color double multiply: `halfArcOuterMatEven` / `Odd` use `color` `0xe2e7f0` / `0xd8dde6` times `map: brushedSilverTex` (`designer2_well.js`). Wafers and call-tile plinths do it again (`designer3_frames.js`, `designer4_arena.js`).

### 3.2 Brushed titanium is an albedo hack

Apple and Lange satin metal is an anisotropic GGX lobe:

- `metalness: 1`
- roughness about `0.34–0.42` for a fine brush, `0.18–0.22` only on a polished chamfer
- `MeshPhysicalMaterial.anisotropy` about `0.7–0.85`
- grain direction locked to the tangent

`MeshStandardMaterial` has no anisotropy. Nothing in `src/` sets `anisotropy`, `anisotropyRotation`, or `computeTangents()`. Texture `anisotropy` (sampler) is a different feature and is also missing on the theme canvases (section 4).

`ExtrudeGeometry` on the deck does not generate tangents. Anisotropy without tangents is undefined. Top-cap UVs of the extrude run with shape X → U. The theme brush is horizontal (constant V, lines along U), so after `rotation.x = -π/2` the grain runs along world +X. That is a coherent top-face direction. Side and bevel UVs of an extrude run along the perimeter, so the same map turns the grain up the 0.025 mm wall. A unibody splits those faces: brushed top, polished edge.

Clearcoat on the split arcs (`0.30–0.35`, roughness `0.22–0.26`) adds a second isotropic lobe on top of a non-anisotropic base. It reads as lacquer, and the lacquer ignores the brush.

`metalness: 0.78` leaves a 22% dielectric diffuse. Satin titanium is a conductor. Use `1.0` and let roughness and anisotropy do the softening. “Anodized” in the study name fights this: anodized aluminum is an oxide dielectric, bare titanium is metal. Pick the metal. The color token `0xd8dde6` can stay as the white-balance of the metal. It should live on `material.color` only.

Gold `0xd4af37` at `metalness: 0.88`, `roughness: 0.20` is web brass. Champagne contact alloy for this palette is closer to `0xe4d2a8`, `metalness: 1`, `roughness: 0.22`. Keep `0xd4af37` for small emblems and the fuel segments if those need to read as yellow gold at a glance.

### 3.3 Porcelain is two grains, and the spec grain loses

Study 05: ceramic `0xebe5da`, `roughness: 0.68`, `metalness: 0.06`, singleton `createCeramicGrainTexture()`.

`designer1_package.js` `getAlabasterCeramicTextures`:

1. Calls `createCeramicGrainTexture({ size: 512, baseHex: '#ebe5da' })`.
2. The singleton guard in `theme.js` is `size === 256 && baseHex === '#ebe5da' && grainAlpha === 0.055`. Size `512` misses it. The package does not use the singleton the study describes.
3. On the first call `_cachedCeramicBumpTex` is null, so the function falls through and paints 18,000 local pebbles plus a real bump centered at 128.
4. It then returns the theme canvas as `colorMap` and the local bump as `bumpMap`. The local color canvas is stored in `_cachedCeramicColorTex` and never shown.

The visible albedo is the theme grain: 2×2 blocks, alpha about `0.03–0.07`, colors `(250,246,238)` and `(198,190,176)`. Study 05 names `#f7f4ee` and `#d5cec0`. The dark pebble is not `ceramicEdge`. Theme code also does `repeat.set(3, 3)` on the shared texture. The bump uses `repeat (1.5, 1.5)` and a different noise. Color grain and height grain crawl against each other.

The material is `roughness: 0.42`, `metalness: 0.08`, not `0.68 / 0.06`. `0.42` is polished ceramic (Watch Edition). `0.68` is bisque porcelain (the study). Either is defensible. They should be the same number in the study and the shader. Porcelain is a dielectric: `metalness: 0`. A `0.06–0.08` metalness term adds a plastic edge.

Figure-ground: paper `0xf4f0e8` = `(244, 240, 232)`, ceramic `0xebe5da` = `(235, 229, 218)`, delta about `(9, 11, 14)`. `paperWarm` is the same hex as `ceramicAlabaster`. Under the current hemisphere the ACES shoulder closes that gap further. The package dissolves into the sweep. Apple stills that put a light object on a light ground earn the edge with a contact shadow and a bench at least ~20 sRGB units darker, not with a second copy of the object color.

Well implementation is close to the study and a little plastic: `wellSlate` roughness `0.68` / metalness `0.38` versus the study’s `0.72 / 0.35` (`designer2_well.js`). Gunmetal wants `metalness: 1`, roughness `0.58–0.66`, and the environment from section 2. Without the probe, raising metalness only makes a blacker plastic. Do both in one change.

Rim engravings use `MeshBasicMaterial` with `toneMapped: false`. They ignore the studio and sit on a different curve from the deck. Laser etch is darker albedo and higher roughness on the same lighting, with `polygonOffset` to avoid z-fight.

### Next iteration — materials

One owner for each surface. `material.color` tints. Maps carry only what the spec layer needs.

Brushed top deck (world +X grain):

```js
deckGeo.computeTangents();

const brushedSilverTitaniumMat = new THREE.MeshPhysicalMaterial({
  color: 0xd8dde6,
  metalness: 1,
  roughness: 0.38,
  roughnessMap: brushedRoughnessMap, // linear, mean sample ≈ 1.0, streaks 0.75–1.0
  anisotropy: 0.78,
  anisotropyRotation: 0,
  envMapIntensity: 0.85,
});
```

Paint the roughness canvas around `rgb(230, 230, 230)` with darker grooves near `rgb(180, 180, 180)`, `NoColorSpace`. Leave `material.roughness` at `0.38` so the multiply lands near `0.27–0.38`. If the canvas stores absolute roughness instead, set `material.roughness = 1`.

Polished chamfer and aperture lip, separate material, no brush map:

```js
new THREE.MeshPhysicalMaterial({
  color: 0xe7e9ee,
  metalness: 1,
  roughness: 0.16,
  anisotropy: 0.15,
  envMapIntensity: 0.9,
});
```

Champagne seam and pads:

```js
new THREE.MeshPhysicalMaterial({
  color: 0xe4d2a8,
  metalness: 1,
  roughness: 0.22,
  envMapIntensity: 0.8,
});
```

Porcelain body:

```js
new THREE.MeshPhysicalMaterial({
  color: 0xffffff,          // albedo lives in the map
  map: ceramicColorMap,     // the 18,000-pebble canvas, not the theme 2×2 grain
  normalMap: ceramicNormalMap,
  normalScale: new THREE.Vector2(0.35, 0.35),
  roughness: 0.64,
  metalness: 0,
  envMapIntensity: 0.35,
});
```

Generate the normal from the same height field as the pebbles. Match `repeat` on color, normal, and roughness (suggest `(2, 2)` across the 4.3-unit package). Delete the theme-grain return path in `getAlabasterCeramicTextures` so the local pebble color is the one on the mesh. A second material for the outer bevel at `roughness: 0.30` gives the polished ceramic edge the study’s gold chamfer can sit against.

Well:

```js
new THREE.MeshPhysicalMaterial({
  color: 0x161d29,
  metalness: 1,
  roughness: 0.62,
  envMapIntensity: 0.55,
});
```

Engraved nomenclature: `MeshStandardMaterial`, map from the existing plaque canvas, `color: 0xffffff`, `roughness: 0.72`, `metalness: 0.15`, `polygonOffset: true`, `polygonOffsetFactor: -1`. Drop `toneMapped: false`.

Split-arc metal: same physical material as the deck at `anisotropy: 0.6`, `clearcoat: 0`. The semantic color belongs on the light-pipe emissive, not in a gold rim and not multiplied into the brush albedo.

Bench immediate surface: paint `≈ #d4cfc4` under the package so the alabaster `(235, 229, 218)` has a real edge, fading to `0xf4f0e8` at the sweep.

---

## 4. Texture caches

### 4.1 Shared plaques are disposed by clients

`theme.js` `createPlaqueTexture` is a process-wide `Map` capped at 512. Eviction deletes the key and does not call `dispose()`. The GPU allocation stays until the page dies. The headless branch (`document === undefined`) inserts a fallback and skips the cap.

Two more caches wrap that function and do dispose:

- `designer3_frames.js` `getOrCreateCachedPlaqueTexture` (cap 160) calls `oldestTex.dispose()` on eviction, and `dispose()` disposes every entry.
- `designer4_arena.js` `getPlaqueTexture` (cap 128) does the same, including on subsystem dispose.

Those textures are the objects stored in `theme.js`. After dispose, `createPlaqueTexture` still hits its `Map` and returns the dead texture. The next plaque with that key is blank for the rest of the session. A long trace that exceeds 160 unique frame signatures hits this without a stage rebuild. `designer2_well.js` calls `createPlaqueTexture` directly, so a colliding key (same title, colors, and size) goes blank there too.

`theme.js` metal, ceramic, and reticle maps have no cap and no `dispose` path. They are safe from the double-free only because clients happen not to dispose them. `designer1` `dispose()` does dispose anything it `track()`s. The shared brush and ceramic maps are not tracked today. The next person who `track()`s a theme texture will reintroduce the bug.

`createCeramicGrainTexture` writes `repeat.set(3, 3)` onto the cached object. Every consumer shares that repeat. A second caller cannot ask for `(1, 1)` without moving the package grain.

`createBrushedMetalTexture` / `createCeramicGrainTexture` cache the empty `new THREE.Texture()` fallback. The first call in a headless test poisons every later browser call for that key.

Canvas plaques sample JetBrains Mono (`index.html` loads it from Google Fonts, async). The cache key has no font-ready bit. The first plaques rasterize in the fallback monospace and stay wrong after the font arrives.

Plaque `CanvasTexture` keeps default mipmaps. At overview distance the mip chain blurs 28 px type. UI plaques want `generateMipmaps = false`, `minFilter = magFilter = LinearFilter`.

Brush and ceramic theme textures leave `anisotropy` at `1`. Oblique macro views pick a higher mip and the 1 px streaks vanish. `designer1` sets `anisotropy = 8` on its roughness map only. The albedo the eye sees is the theme texture.

### 4.2 Brush and ceramic filters

Set sampler anisotropy where the texture is created, once:

```js
tex.anisotropy = 8;
tex.wrapS = tex.wrapT = THREE.RepeatWrapping;
```

For the metal albedo, do not also tint `material.color` (section 3). Prefer a roughness-only grain and a flat metal color so the cache stores one linear map.

Ceramic: stop exporting the 2×2 alpha dither as the package albedo. If `createCeramicGrainTexture` remains, it should return `{ color, normal, roughness }` from one hash, with repeat left at `(1, 1)` for the caller to set on a `.clone()` if two repeats are required. Cloning a `CanvasTexture` shares the image and allows a different `repeat`.

### Next iteration — cache contract

One module owns GPU lifetime.

```js
export function retainTexture(tex) {
  tex.userData.refs = (tex.userData.refs || 0) + 1;
  return tex;
}

export function releaseTexture(tex) {
  if (!tex) return;
  tex.userData.refs = Math.max(0, (tex.userData.refs || 1) - 1);
  if (tex.userData.refs === 0 && tex.userData.evicted) tex.dispose();
}
```

- `createPlaqueTexture` is the only plaque cache. Delete the `Map`s in `designer3_frames.js` and `designer4_arena.js`.
- Evict by dropping the key and setting `userData.evicted = true`. Call `dispose()` only when `refs === 0`.
- Subsystem `dispose()` calls `releaseTexture` on maps it retained. It never calls `dispose()` on a theme texture.
- Do not insert headless fallbacks into the cache.
- Await `document.fonts.load('600 28px "JetBrains Mono"')` once before the first plaque, and bump a `fontEpoch` in the key if that load finishes late.
- Plaques: no mipmaps, linear filter. Brush and ceramic: mipmaps on, `anisotropy = 8`.
- Add `disposeThemeTextures()` and call it from `SherlockStage.dispose()` after subsystems release. That is the full teardown, including metal, ceramic, and reticle.
- Key floats with a fixed precision (`streakAlpha.toFixed(3)`) so `0.08` and `0.0800000001` share an entry.

`MAX_PLAQUE_CACHE_SIZE = 512` can stay. The bug is ownership, not the number.

---

## 5. Order of work

1. Stop clients from disposing theme textures. One cache, refcount, no headless entries, font epoch, plaque filtering. This is a correctness bug already visible on long traces.
2. Remove the double color multiply and the double roughness multiply on the deck, the arcs, the wafers, and the plinths. Put satin roughness near `0.38` in the actual shader. Confirm with a one-frame readback or a debug roughness view before any lighting retune.
3. Use the pebble height field as color + normal, one repeat, `metalness: 0`, roughness `0.64`. Drop the theme 2×2 grain from the package body.
4. Switch titanium and gold to `MeshPhysicalMaterial`, `metalness: 1`, `computeTangents()`, anisotropy on the top cap only. Split the polished chamfer.
5. Add the room probe (then a real softbox HDR) and `envMapIntensity` per tier. Then apply the rig in section 2: lower hemisphere, side key, neutral rim, decay 2 practicals, `GATE` tint, tighter 2048 shadow, offset bench contact.
6. Switch the renderer to reversed-Z, `near: 0.02`, Neutral tone map, dispose-safe init. Retune exposure on the two spheres. Update ring studies 05 and 06 and the `DESIGNER_LAYERS` blurb in the same change so the next pass does not restore `0xfbbf24`, decay `1.4`, or Space-Black ceramic.

Do not retune exposure, fog, or light intensity while the deck roughness is still ~0.06 and the ceramic map is squared. Those knobs will be fitting the bug.
