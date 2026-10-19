# Quint Formal Verification Audit Report 01: Three.js WebGPU Stage, Theme Caches & 8 Designers Memory Lifecycle

- **Verifier Agent**: `Quint-Memory-Lifecycle-Verifier` (Agent 1)
- **Formal Specification**: [`specs/quint/visualization_memory_lifecycle.qnt`](file:///home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt)
- **Audited Codebase Scope**:
  - [`src/stage/SherlockStage.js`](file:///home/charles/sherlock-bench/src/stage/SherlockStage.js)
  - [`src/designers/theme.js`](file:///home/charles/sherlock-bench/src/designers/theme.js)
  - [`src/designers/designer1_package.js`](file:///home/charles/sherlock-bench/src/designers/designer1_package.js)
  - [`src/designers/designer2_well.js`](file:///home/charles/sherlock-bench/src/designers/designer2_well.js)
  - [`src/designers/designer3_frames.js`](file:///home/charles/sherlock-bench/src/designers/designer3_frames.js)
  - [`src/designers/designer4_arena.js`](file:///home/charles/sherlock-bench/src/designers/designer4_arena.js)
  - [`src/designers/designer5_circuit.js`](file:///home/charles/sherlock-bench/src/designers/designer5_circuit.js)
  - [`src/designers/designer6_loom.js`](file:///home/charles/sherlock-bench/src/designers/designer6_loom.js)
  - [`src/designers/designer7_cinema.js`](file:///home/charles/sherlock-bench/src/designers/designer7_cinema.js)
  - [`src/designers/designer8_quineling.js`](file:///home/charles/sherlock-bench/src/designers/designer8_quineling.js)

---

## 1. Executive Summary

Using the `Quint-llm-kit` formal modeling and simulation toolchain, we modeled the complete GPU/heap resource lifecycle of `SherlockStage`, `theme.js` LRU caches, and all 8 Designer subsystems (`D1`–`D8`) in [`visualization_memory_lifecycle.qnt`](file:///home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt) and audited every line of the implementation against five formal safety invariants:

1. `noLeakedResourcesAfterDispose`
2. `boundedPlaqueCacheInvariant`
3. `boundedMaterialCacheInvariant`
4. `shadowMapDemandBoundedInvariant`
5. `noDanglingSharedResourceDispose`

### Summary of Findings

| ID | Severity | Subsystem / File | Lines | Category | Summary |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **MEM-01** | **CRITICAL** | `designer8_quineling.js` | `499–714`, `1111–1122` | GPU Memory Leak (`noLeakedResourcesAfterDispose`) | `createQuinelingSubsystem()` omits `dispose()` entirely, leaking **6 `BufferGeometry`s** (including 57,344-vertex ridge buffers & 8,192-point tissue buffers), **6 `Material`s**, and **2 `InstancedMesh`s** on every stage teardown. |
| **MEM-02** | **CRITICAL** | `designer2_well.js` | `1391`, `1476`, `1777`, `2098`, `2115`, `2210`, `2228`, `2246`, `2255`, `3044–3054` | GPU Memory Leak (`noLeakedResourcesAfterDispose`) | `createWellSubsystem()` creates **9 `THREE.InstancedMesh` instances** and never calls `.dispose()` on any of them in `dispose()`, leaking all 9 GPU `instanceMatrix` and `instanceColor` attribute buffers. |
| **MEM-03** | **HIGH** | `theme.js` | `132–137`, `423–429` | GPU Texture Cache Leak (`boundedPlaqueCacheInvariant` / `boundedMaterialCacheInvariant`) | `plaqueTextureCache` deletes evicted entries without disposing unmounted `CanvasTexture`s (permanently leaking GPU VRAM in `renderer.textures`); `metalTextureCache`, `reticleTextureCache`, and `ceramicTextureCache` are unbounded and lack a disposal API. |
| **MEM-04** | **HIGH** | `SherlockStage.js` & `designer8_quineling.js` | `SherlockStage.js:126–138, 150–165, 293, 308–316, 364, 385–397`; `designer8_quineling.js:738, 824` | Lifecycle Bug & Stage Disposal Leak | (a) `SherlockStage.render()` calls `sub.update(fullState)` with 1 arg, but `designer8_quineling.js` reads `ambient` & `dt` from args 2 & 3 (`ambient = 0`), freezing time-driven 30Hz `WovenBody.frame` sampling; (b) `keyLight.shadow.map` (2048×2048 `RenderTarget`) is never disposed; (c) `render()` lacks a `if (this._disposed) return;` guard; (d) `Object.assign(this._fullState, stepState)` retains stale references across partial updates. |
| **MEM-05** | **MEDIUM** | `designer8_quineling.js` | `536–541`, `565–570`, `602–607`, `629–634`, `677–682` | Heap / Inspector Metadata Corruption | `tagInspectable(mesh, titleStr, detailsStr, subtitleStr)` passes strings instead of a `meta` object; `theme.js` spreads `...meta`, creating ~40 character-index keys (`0: 'W', 1: 'o', ...`) on `userData` and leaving `title`/`subtitle` `undefined`. |
| **MEM-06** | **MEDIUM** | `designer7_cinema.js` | `445`, `536`, `740–748`, `915–925` | Event Listener & Scene Graph Retention | Missing `'pointercancel'` listener (can lock `isDragging = true`); `removeEventListener('wheel', onWheel)` omits matching options; `dispose()` fails to clear `floorRuleProxyCache`, `_raycastHits`, `hoveredObject`, `pinnedObject`, `lastBoxTarget`, and `inspectCallback`. |
| **MEM-07** | **MEDIUM** | `designer4_arena.js` & `designer1_package.js` | `designer4_arena.js:78–80, 164–198`; `designer1_package.js:32–34, 56–168` | Module-Scope Heap & Singleton Texture Retention | `designer4_arena.js` retains `_cachedTraceRef` and `_cachedAllCalls` at module scope after `dispose()` and mutates `.status` in-place; `designer1_package.js` caches 3 module-scope 512×512 `CanvasTexture`s with no disposal hook. |
| **MEM-08** | **LOW** | `designer8_quineling.js` & `sherlockQuinelingCompiler.js` | `designer8_quineling.js:237–238, 309, 736, 831`; `sherlockQuinelingCompiler.js:501–506` | Per-Sample GC Churn & Module Cache Retention | `WovenBody.frame(..., { crests: true, reuse: _frameReuse })` reuses `points`/`owners` typed arrays but allocates new `ridges` arrays/point objects at 30Hz; module-scope caches (`sharedTissueSpriteTexture`, `_lastTelemetryResult`, `compiledQuinelingCache`) have no cleanup hook. |

---

## 2. Formal Quint Verification Results

### 2.1 Executed Commands & Outputs

All required Quint verification checks were executed using `/home/charles/.nvm/versions/node/v26.10.0/bin/quint` against [`/home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt`](file:///home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt):

```text
$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint typecheck /home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt
[ok] (0 errors)

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint test /home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt
  visualization_memory_lifecycle
    ok fullStageLifecycleAndDisposeTest passed 1 test(s)
    ok lruCacheEvictionSafetyTest passed 1 test(s)
    ok earlyDisposeBeforeInitAndRemountTest passed 1 test(s)
    ok auditProofUnpatchedDesignerLeaksTest passed 1 test(s)
    ok auditProofUnpatchedPlaqueVramLeakTest passed 1 test(s)

  5 passing (86ms)

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=boundedPlaqueCacheInvariant --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt
[ok] No violation found (42ms at 4762 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=boundedMaterialCacheInvariant --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt
[ok] No violation found (41ms at 4878 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=noLeakedResourcesAfterDispose --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt
[ok] No violation found (37ms at 5405 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=shadowMapDemandBoundedInvariant --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt
[ok] No violation found (35ms at 5714 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=noDanglingSharedResourceDispose --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/visualization_memory_lifecycle.qnt
[ok] No violation found (37ms at 5405 traces/second).
```

### 2.2 Subsystem Audit Matrix

| Subsystem | Owned Geometries Disposed? | Owned Materials Disposed? | Owned Textures Disposed? | `InstancedMesh` Disposed? | Shared `theme.js` Resources Protected? | Event Listeners Cleaned Up? |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **`SherlockStage.js`** | N/A | N/A | `scene.environment` singleton | N/A | Yes | `window.resize` removed; **missing `keyLight.shadow` dispose & `_disposed` guard in `render()`** |
| **`theme.js`** | N/A | N/A | **NO** — Evicted `plaqueTextureCache` textures never disposed; 3 caches unbounded | N/A | Yes (`plaqueTextureCache` avoids disposing mounted textures, but leaks unmounted ones) | N/A |
| **`D1: designer1_package.js`** | Yes (`track()`) | Yes (`track()`) | Partial — `benchTex` & 3 rim plaques disposed; **3 module-scope 512×512 textures never disposed** | Yes (`edgePadsInstanced.dispose()`) | Yes (`brushedMetalMap` not disposed by D1) | N/A |
| **`D2: designer2_well.js`** | Yes (`trackGeo()`) | Yes (`trackMat()`) | Yes (`trackTex()` on 12 split-arc dials + stator dial + floor overlay) | **NO — 9 `InstancedMesh` instances leaked!** | Yes (`brushedSilverTex` and `createPlaqueTexture` results not in `disposableTextures`) | N/A |
| **`D3: designer3_frames.js`** | Yes (`trackGeo()`) | Yes (`trackMat()`) | N/A (uses `theme.js` plaques) | N/A | Yes (`rhodiumBrushTex` & `createPlaqueTexture` not in `disposableResources`) | N/A |
| **`D4: designer4_arena.js`** | Yes (`trackGeo()`) | Yes (`trackMat()`) | Yes (`trackTex()` on 6 call tile plaques) | N/A | Yes | N/A (**leaks `_cachedTraceRef` at module scope**) |
| **`D5: designer5_circuit.js`** | Yes (`track()`) | Yes (`track()`) | N/A | Yes (`leftExprInstanced.dispose()`, `rightLimbInstanced.dispose()`) | Yes | N/A |
| **`D6: designer6_loom.js`** | Yes (`track()`) | Yes (`track()`) | N/A | Yes (`progressInstanced.dispose()`) | Yes | N/A |
| **`D7: designer7_cinema.js`** | Yes (`reticle.dispose()`) | Yes (`reticle.dispose()`) | N/A | N/A | Yes | Partial (**missing `pointercancel`, retains `floorRuleProxyCache` & `_raycastHits`**) |
| **`D8: designer8_quineling.js`** | **NO — 6 Geometries leaked!** | **NO — 6 Materials leaked!** | **NO — `sharedTissueSpriteTexture` leaked!** | **NO — 2 `InstancedMesh` instances leaked!** | N/A | N/A (**`dispose()` method completely missing!**) |

---

## 3. Detailed Findings & Drop-In Code Fixes

### Finding MEM-01 (CRITICAL): `designer8_quineling.js` Completely Omits `dispose()`, Leaking 14 GPU Buffers/Materials/InstancedMeshes

- **File**: [`/home/charles/sherlock-bench/src/designers/designer8_quineling.js`](file:///home/charles/sherlock-bench/src/designers/designer8_quineling.js#L499-L1122)
- **Lines**: `499–714` (allocations), `1111–1122` (return object missing `dispose`)
- **Formal Invariant Violated**: `noLeakedResourcesAfterDispose` (`auditProofUnpatchedDesignerLeaksTest`)

#### Root Cause
`createQuinelingSubsystem()` allocates the largest dynamic WebGPU vertex buffers in the entire application:
1. `tissueGeom` (`THREE.BufferGeometry`, line 501 — `8,192 × 3` float32 position + color attributes) and `tissueMat` (`THREE.PointsMaterial`, line 507)
2. `ridgeLineGeom` (`THREE.BufferGeometry`, line 547 — `57,344 × 3` float32 position + color attributes = **1.37 MB** per instance) and `ridgeLineMat` (`THREE.LineBasicMaterial`, line 554)
3. `sdkRibbonGeom` (`THREE.BufferGeometry`, line 576 — `1,152 × 3` float32 position + color + normal + index buffers) and `sdkRibbonMat` (`THREE.MeshPhysicalMaterial`, line 585)
4. `anchorNodeGeom` (`THREE.SphereGeometry`, line 613), `anchorNodeMat` (`THREE.MeshStandardMaterial`, line 614), and `anchorNodeMesh` (`THREE.InstancedMesh`, line 623)
5. `depArcGeom` (`THREE.BufferGeometry`, line 637) and `depArcMat` (`THREE.LineBasicMaterial`, line 644)
6. `sporeGeom` (`THREE.OctahedronGeometry`, line 661), `sporeMat` (`THREE.MeshStandardMaterial`, line 662), and `sporeMesh` (`THREE.InstancedMesh`, line 671)
7. `tendrilGeom` (`THREE.BufferGeometry`, line 688) and `tendrilMat` (`THREE.MeshPhysicalMaterial`, line 697)

However, `createQuinelingSubsystem()` returns `{ group, coreRoot, speciesGroups, tissuePointsMesh, ridgeLineSegments, sdkManifoldRibbonMesh, anchorNodeMesh, sporeMesh, update }` at lines `1111–1122` **without a `dispose()` method**. When `SherlockStage.dispose()` iterates `SUBSYSTEM_KEYS`, `sub.dispose` is `undefined` for `quineling`, leaking 100% of Designer 8's GPU memory.

#### Drop-In Code Fix (`src/designers/designer8_quineling.js`, lines `1110–1122`)
```javascript
  function dispose() {
    cachedWovenFrame = null;
    anchorNodeMesh.dispose();
    sporeMesh.dispose();

    tissueGeom.dispose();
    tissueMat.dispose();
    ridgeLineGeom.dispose();
    ridgeLineMat.dispose();
    sdkRibbonGeom.dispose();
    sdkRibbonMat.dispose();
    anchorNodeGeom.dispose();
    anchorNodeMat.dispose();
    depArcGeom.dispose();
    depArcMat.dispose();
    sporeGeom.dispose();
    sporeMat.dispose();
    tendrilGeom.dispose();
    tendrilMat.dispose();
  }

  return {
    group,
    coreRoot,
    speciesGroups,
    tissuePointsMesh,
    ridgeLineSegments,
    sdkManifoldRibbonMesh,
    anchorNodeMesh,
    sporeMesh,
    update,
    dispose,
  };
```

---

### Finding MEM-02 (CRITICAL): `designer2_well.js` Leaks All 9 `THREE.InstancedMesh` GPU Attribute Buffers in `dispose()`

- **File**: [`/home/charles/sherlock-bench/src/designers/designer2_well.js`](file:///home/charles/sherlock-bench/src/designers/designer2_well.js#L1391-L3054)
- **Lines**:
  - `1391`: `fuelInstancedMesh` (48 instances, dynamic `instanceMatrix` + `instanceColor`)
  - `1476`: `bridgeClaspsInstancedMesh` (24 instances, dynamic `instanceMatrix` + `instanceColor`)
  - `1777`: `statorSectorsInstancedMesh` (10 instances, dynamic `instanceMatrix` + `instanceColor`)
  - `2098`: `vaultRibsInstancedMesh` (8 instances, `instanceMatrix`)
  - `2115`: `southVaultRibsInstancedMesh` (4 instances, `instanceMatrix`)
  - `2210`: `ruleBezelInstancedMesh` (39 instances, `instanceMatrix`)
  - `2228`: `ruleTilesInstancedMesh` (39 instances, dynamic `instanceMatrix` + `instanceColor`)
  - `2246`: `ruleBeaconInstancedMesh` (39 instances, dynamic `instanceMatrix` + `instanceColor`)
  - `2255`: `ruleBeaconPostInstancedMesh` (39 instances, dynamic `instanceMatrix` + `instanceColor`)
  - `3044–3054`: `dispose()`
- **Formal Invariant Violated**: `noLeakedResourcesAfterDispose` (`auditProofUnpatchedDesignerLeaksTest`)

#### Root Cause
In Three.js (`three/webgpu`), `THREE.InstancedMesh` allocates internal `InstancedBufferAttribute` GPU buffers (`instanceMatrix` and `instanceColor`) that are only freed when `instancedMesh.dispose()` is called (which disposes the `InstancedMesh` object itself). While `designer1_package.js` (`line 899`), `designer5_circuit.js` (`lines 256–257`), and `designer6_loom.js` (`line 182`) explicitly call `.dispose()` on their `InstancedMesh` instances, `designer2_well.js` only tracks geometries, materials, and textures, leaving all **9 `InstancedMesh` instances** undisposed.

#### Drop-In Code Fix (`src/designers/designer2_well.js`, lines `3044–3054`)
```javascript
  function dispose() {
    fuelInstancedMesh.dispose();
    bridgeClaspsInstancedMesh.dispose();
    statorSectorsInstancedMesh.dispose();
    vaultRibsInstancedMesh.dispose();
    southVaultRibsInstancedMesh.dispose();
    ruleBezelInstancedMesh.dispose();
    ruleTilesInstancedMesh.dispose();
    ruleBeaconInstancedMesh.dispose();
    ruleBeaconPostInstancedMesh.dispose();

    for (let i = 0; i < disposableGeometries.length; i += 1) {
      disposableGeometries[i].dispose();
    }
    for (let i = 0; i < disposableMaterials.length; i += 1) {
      disposableMaterials[i].dispose();
    }
    for (let i = 0; i < disposableTextures.length; i += 1) {
      disposableTextures[i].dispose();
    }
    disposableGeometries.length = 0;
    disposableMaterials.length = 0;
    disposableTextures.length = 0;
  }
```

---

### Finding MEM-03 (HIGH): `theme.js` `plaqueTextureCache` Eviction GPU Texture Leak & Unbounded Auxiliary Caches

- **File**: [`/home/charles/sherlock-bench/src/designers/theme.js`](file:///home/charles/sherlock-bench/src/designers/theme.js#L132-L432)
- **Lines**: `132–137`, `144–208`, `215–290`, `295–334`, `340–432`
- **Formal Invariants Addressed**: `boundedPlaqueCacheInvariant`, `boundedMaterialCacheInvariant`, `noDanglingSharedResourceDispose` (`auditProofUnpatchedPlaqueVramLeakTest`)

#### Root Cause
1. **Evicted Plaque VRAM Leak (`lines 423–429`)**:
   ```javascript
   if (plaqueTextureCache.size >= MAX_PLAQUE_CACHE_SIZE) {
     const oldestKey = plaqueTextureCache.keys().next().value;
     if (oldestKey !== undefined) {
       // Do NOT call .dispose() on evicted textures that may still be mounted on active meshes
       plaqueTextureCache.delete(oldestKey);
     }
   }
   ```
   In Three.js WebGPU/WebGL, removing a `THREE.CanvasTexture` from a JS `Map` does **not** release its GPU texture (`GPUTexture`) in the renderer backend; GPU memory is only freed when `tex.dispose()` is called. Because at most 25 plaque meshes exist across Designer 2 (12 half-arcs + 1 stator lens) and Designer 3 (12 continuation wafers), **>97.5% of LRU evictions from the 1024-entry cache are not mounted on any mesh**, yet they are never `.dispose()`d!
2. **Unbounded Auxiliary Texture Caches (`lines 134–137`)**:
   `metalTextureCache`, `reticleTextureCache`, and `ceramicTextureCache` have no `MAX_MATERIAL_CACHE_SIZE` bound and no LRU eviction.
3. **Missing Cache Teardown Helper**:
   There is no `disposeThemeCaches()` export to release cached GPU textures when the application unmounts `SherlockStage`.

#### Drop-In Code Fix (`src/designers/theme.js`, lines `130–138` and `420–433`)
```javascript
const MAX_PLAQUE_CACHE_SIZE = 1024;
const MAX_MATERIAL_CACHE_SIZE = 64;
const plaqueTextureCache = new Map();
const metalTextureCache = new Map();
const reticleTextureCache = new Map();
let ceramicGrainSingleton = null;
const ceramicTextureCache = new Map();

function setBoundedCacheEntry(cacheMap, key, tex, maxSize = MAX_MATERIAL_CACHE_SIZE) {
  if (cacheMap.has(key)) {
    cacheMap.delete(key);
  } else if (cacheMap.size >= maxSize) {
    const oldestKey = cacheMap.keys().next().value;
    if (oldestKey !== undefined) {
      const evicted = cacheMap.get(oldestKey);
      cacheMap.delete(oldestKey);
      if (evicted && (!evicted.userData?.refCount || evicted.userData.refCount <= 0)) {
        evicted.dispose();
      }
    }
  }
  cacheMap.set(key, tex);
}

/**
 * Helper for designers to swap `material.map` with reference-counted plaque ownership
 * so LRU-evicted textures are disposed if and only if no mesh mounts them.
 */
export function assignSharedPlaqueTexture(material, nextTex) {
  const prevTex = material.map;
  if (prevTex === nextTex) return;
  if (prevTex && prevTex.userData?.sharedPlaque) {
    prevTex.userData.refCount = Math.max(0, (prevTex.userData.refCount || 1) - 1);
    if (prevTex.userData.refCount === 0 && prevTex.userData.evictedFromCache) {
      prevTex.dispose();
    }
  }
  if (nextTex && nextTex.userData?.sharedPlaque) {
    nextTex.userData.refCount = (nextTex.userData.refCount || 0) + 1;
  }
  material.map = nextTex;
  material.needsUpdate = true;
}

export function disposeThemeCaches() {
  for (const tex of plaqueTextureCache.values()) tex?.dispose?.();
  plaqueTextureCache.clear();
  for (const tex of metalTextureCache.values()) tex?.dispose?.();
  metalTextureCache.clear();
  for (const tex of reticleTextureCache.values()) tex?.dispose?.();
  reticleTextureCache.clear();
  for (const tex of ceramicTextureCache.values()) tex?.dispose?.();
  ceramicTextureCache.clear();
  if (ceramicGrainSingleton) {
    ceramicGrainSingleton.dispose();
    ceramicGrainSingleton = null;
  }
  if (studioEnvMapSingleton) {
    studioEnvMapSingleton.dispose();
    studioEnvMapSingleton = null;
  }
}
```
And in `createPlaqueTexture` (`lines 420–431`):
```javascript
  tex.userData = { title, subtitle, badge, width, height, sharedPlaque: true, refCount: 0, evictedFromCache: false };
  tex.needsUpdate = true;

  if (plaqueTextureCache.size >= MAX_PLAQUE_CACHE_SIZE) {
    const oldestKey = plaqueTextureCache.keys().next().value;
    if (oldestKey !== undefined) {
      const evictedTex = plaqueTextureCache.get(oldestKey);
      plaqueTextureCache.delete(oldestKey);
      if (evictedTex) {
        evictedTex.userData.evictedFromCache = true;
        if (!evictedTex.userData.refCount || evictedTex.userData.refCount <= 0) {
          evictedTex.dispose();
        }
      }
    }
  }
  plaqueTextureCache.set(cacheKey, tex);
  return tex;
```

---

### Finding MEM-04 (HIGH): `SherlockStage.js` & `designer8_quineling.js` Signature Mismatch, Un-disposed Shadow Map `RenderTarget`, and Post-Dispose Render Guard

- **Files**:
  - [`/home/charles/sherlock-bench/src/stage/SherlockStage.js`](file:///home/charles/sherlock-bench/src/stage/SherlockStage.js#L126-L398) (`lines 126–138, 150–165, 293, 308–316, 364, 385–397`)
  - [`/home/charles/sherlock-bench/src/designers/designer8_quineling.js`](file:///home/charles/sherlock-bench/src/designers/designer8_quineling.js#L738-L826) (`lines 738, 772, 780, 824`)
- **Formal Invariants Addressed**: `noLeakedResourcesAfterDispose`, `shadowMapDemandBoundedInvariant`

#### Root Cause
1. **Broken `ambient` / `dt` Parameter Forwarding to Designer 8 (`SherlockStage.js:364` vs `designer8_quineling.js:738`)**:
   In `SherlockStage.js` line 364:
   ```javascript
   sub.update(fullState);
   ```
   Every other designer (`D1`–`D7`) reads `stepState.ambient` and `stepState.dt` from the `stepState` object. However, `designer8_quineling.js` line 738 declares:
   ```javascript
   function update(stepState = {}, ambient = 0, dt = 0.016)
   ```
   and never reads `stepState.ambient` or `stepState.dt`! Consequently, whenever `SherlockStage.render()` runs, `ambient` inside Designer 8 is permanently `0`, so `Math.abs(ambient - lastFrameSampleTime) >= 0.032` (`line 824`) is permanently `false` after the first sample, freezing continuous 30Hz `WovenBody` animation!
2. **Un-disposed `keyLight.shadow.map` (`SherlockStage.js:150–165, 385–397`)**:
   `this.keyLight` allocates a 2048×2048 shadow map `RenderTarget`, but `this.keyLight.shadow.dispose()` / `this.keyLight.dispose()` is never called in `SherlockStage.dispose()`.
3. **Missing `if (this._disposed) return;` Guard in `render()` (`SherlockStage.js:293`)**:
   Calling `render()` after `dispose()` updates disposed subsystems and invokes `this.renderer.render()` on a disposed renderer.
4. **Stale Reference Retention in `this._fullState` (`SherlockStage.js:308–316`)**:
   `Object.assign(fullState, stepState)` mutates `this._fullState` without clearing keys from prior calls when `stepState` omits properties, and `this._fullState` is not cleared on `dispose()`.

#### Drop-In Code Fix 1 (`src/designers/designer8_quineling.js`, line `738`)
```javascript
  function update(stepState = {}, ambientArg = 0, dtArg = 0.016) {
    const ambient = stepState?.ambient ?? ambientArg;
    const dt = stepState?.dt ?? dtArg;
```

#### Drop-In Code Fix 2 (`src/stage/SherlockStage.js`, lines `293–300` and `385–398`)
```javascript
  render(stepState) {
    if (this._disposed) return;
    const now = performance.now();
```
```javascript
  dispose() {
    if (this._disposed) return;
    this._disposed = true;
    if (typeof window !== 'undefined' && window.removeEventListener) {
      window.removeEventListener('resize', this._onResize);
    }
    for (let i = 0; i < SUBSYSTEM_KEYS.length; i++) {
      const sub = this.subsystems[SUBSYSTEM_KEYS[i]];
      if (sub && typeof sub.dispose === 'function') sub.dispose();
    }
    if (this.keyLight?.shadow?.map) {
      this.keyLight.shadow.map.dispose();
    }
    if (this.keyLight?.dispose) {
      this.keyLight.dispose();
    }
    if (this.ready && this.renderer) {
      this.renderer.dispose();
    }
    this._fullState = { ambient: 0, dt: 0.016, activePreset: 'overview', cutawayMix: 0 };
  }
```

---

### Finding MEM-05 (MEDIUM): `designer8_quineling.js` `tagInspectable` Call Signature Mismatch Corrupts `userData`

- **File**: [`/home/charles/sherlock-bench/src/designers/designer8_quineling.js`](file:///home/charles/sherlock-bench/src/designers/designer8_quineling.js#L536-L683)
- **Lines**: `536–541`, `565–570`, `602–607`, `629–634`, `677–682`

#### Root Cause
In `theme.js` (`lines 447–453`), `tagInspectable(object3d, meta)` is defined as:
```javascript
export function tagInspectable(object3d, meta) {
  object3d.userData = {
    ...object3d.userData,
    inspectable: true,
    ...meta,
  };
```
In `designer8_quineling.js`, all 5 calls to `tagInspectable` pass **3 strings** instead of a `meta` object:
```javascript
tagInspectable(
  tissuePointsMesh,
  'WovenBody Mathematical Tissue Field',
  '8,192 real-time points sampled from WovenBody.frame(wovenBody, phase) and colored by Chroma.colorFor(shape, owner).',
  'nursery.html // WovenBody.frame'
);
```
Because JavaScript's object spread `{ ...'WovenBody Mathematical Tissue Field' }` spreads string indices, `tissuePointsMesh.userData` is polluted with `0: 'W', 1: 'o', 2: 'v', ..., 34: 'd'` while `userData.title`, `userData.subtitle`, and `userData.details` remain `undefined`!

#### Drop-In Code Fix (`src/designers/designer8_quineling.js`, lines `536–541`, `565–570`, `602–607`, `629–634`, `677–682`)
```javascript
  tagInspectable(tissuePointsMesh, {
    type: 'quineling-tissue',
    id: 'woven-body-tissue-cloud',
    title: 'WovenBody Mathematical Tissue Field',
    subtitle: 'nursery.html // WovenBody.frame',
    details: [
      '8,192 real-time points sampled from WovenBody.frame(wovenBody, phase) and colored by Chroma.colorFor(shape, owner).',
    ],
  });
```
*(Apply the same `{ type, id, title, subtitle, details: [...] }` object structure to `ridgeLineSegments`, `sdkManifoldRibbonMesh`, `anchorNodeMesh`, and `sporeMesh` at lines `565–570`, `602–607`, `629–634`, and `677–682`.)*

---

### Finding MEM-06 (MEDIUM): `designer7_cinema.js` Missing `pointercancel` Listener & Retained Scene References on `dispose()`

- **File**: [`/home/charles/sherlock-bench/src/designers/designer7_cinema.js`](file:///home/charles/sherlock-bench/src/designers/designer7_cinema.js#L445-L926)
- **Lines**: `445`, `536`, `740–748`, `915–925`

#### Root Cause
1. `domElement` registers `pointerdown`, `pointermove`, `pointerup`, and `pointerleave`, but omits `pointercancel`. If the browser cancels a pointer gesture (e.g., palm rejection, OS gesture, or window blur during touch/drag), `isDragging` and `isPanning` remain `true` indefinitely, blocking `performRaycast()`.
2. `dispose()` does not clear `floorRuleProxyCache` (which holds up to 39 proxy objects with `parent: instancedMesh.parent`), `_raycastHits` (which retains Three.js intersection hit objects referencing scene meshes), `hoveredObject`, `pinnedObject`, `lastBoxTarget`, or `inspectCallback`.

#### Drop-In Code Fix (`src/designers/designer7_cinema.js`, lines `740–748` and `915–925`)
```javascript
  if (domElement && domElement.addEventListener) {
    domElement.style.cursor = 'grab';
    domElement.addEventListener('pointerdown', onPointerDown);
    domElement.addEventListener('pointermove', onPointerMove);
    domElement.addEventListener('pointerup', onPointerUp);
    domElement.addEventListener('pointercancel', onPointerUp);
    domElement.addEventListener('pointerleave', onPointerLeave);
    domElement.addEventListener('wheel', onWheel, { passive: false });
    domElement.addEventListener('contextmenu', onContextMenu);
  }
```
```javascript
  function dispose() {
    if (domElement && domElement.removeEventListener) {
      domElement.removeEventListener('pointerdown', onPointerDown);
      domElement.removeEventListener('pointermove', onPointerMove);
      domElement.removeEventListener('pointerup', onPointerUp);
      domElement.removeEventListener('pointercancel', onPointerUp);
      domElement.removeEventListener('pointerleave', onPointerLeave);
      domElement.removeEventListener('wheel', onWheel, { passive: false });
      domElement.removeEventListener('contextmenu', onContextMenu);
    }
    floorRuleProxyCache.clear();
    _raycastHits.length = 0;
    hoveredObject = null;
    pinnedObject = null;
    lastBoxTarget = null;
    inspectCallback = null;
    reticle.dispose();
  }
```

---

### Finding MEM-07 (MEDIUM): Module-Scope Trace & Texture Retention in `designer4_arena.js` and `designer1_package.js`

- **Files**:
  - [`/home/charles/sherlock-bench/src/designers/designer4_arena.js`](file:///home/charles/sherlock-bench/src/designers/designer4_arena.js#L78-L85) (`lines 78–85`, `164–198`, `489–499`)
  - [`/home/charles/sherlock-bench/src/designers/designer1_package.js`](file:///home/charles/sherlock-bench/src/designers/designer1_package.js#L32-L168) (`lines 32–34`, `56–168`, `898–907`)

#### Root Cause
1. In `designer4_arena.js`, `_cachedTraceRef` and `_cachedAllCalls` are declared at module scope (`lines 78–80`). When `dispose()` is called (`lines 489–499`), `_cachedTraceRef` is never cleared, keeping the entire execution `trace` object (and all its step snapshots) alive in the JS heap. Moreover, `evaluateCallTilesAtStep` mutates `c.status` in-place on `_cachedAllCalls` elements (`lines 186–198`), which causes cross-instance state interference if multiple `createArenaSubsystem()` instances are mounted.
2. In `designer1_package.js`, `_cachedCeramicColorTex`, `_cachedCeramicBumpTex`, and `_cachedSilverRoughnessTex` (`lines 32–34`) are 512×512 `CanvasTexture` instances cached at module scope without being tracked or disposed.

#### Drop-In Code Fix (`src/designers/designer4_arena.js`)
Move `_cachedTraceRef`, `_cachedTraceKey`, and `_cachedAllCalls` inside `createArenaSubsystem()` instance scope and clear them in `dispose()`:
```javascript
  function dispose() {
    _cachedTraceRef = null;
    _cachedTraceKey = '';
    _cachedAllCalls = [];
    _visibleCallsScratch.length = 0;
    for (let i = 0; i < disposableGeometries.length; i += 1) {
      disposableGeometries[i].dispose();
    }
    for (let i = 0; i < disposableMaterials.length; i += 1) {
      disposableMaterials[i].dispose();
    }
    for (let i = 0; i < disposableTextures.length; i += 1) {
      disposableTextures[i].dispose();
    }
  }
```
And in `src/designers/designer1_package.js`, either scope the ceramic/silver textures to `createPackageSubsystem()` via `track(...)` or dispose and null out `_cachedCeramicColorTex`, `_cachedCeramicBumpTex`, and `_cachedSilverRoughnessTex` in `dispose()`.

---

### Finding MEM-08 (LOW): 30Hz Ridge Array Allocation Churn & Module-Scope Cache Retention in `designer8_quineling.js`

- **Files**:
  - [`/home/charles/sherlock-bench/src/designers/designer8_quineling.js`](file:///home/charles/sherlock-bench/src/designers/designer8_quineling.js#L237-L327) (`lines 237–238, 309–327, 831–835`)
  - [`/home/charles/sherlock-bench/src/engine/sherlockQuinelingCompiler.js`](file:///home/charles/sherlock-bench/src/engine/sherlockQuinelingCompiler.js#L501-L506) (`lines 501–506`)

#### Root Cause
1. In `designer8_quineling.js` (`line 831`), `WovenBody.frame(artifact.wovenBody, wovenPhase, { budget: TISSUE_BUDGET, crests: true, reuse: _frameReuse })` reuses the `points` (`Float32Array`) and `owners` (`Uint16Array`) buffers in `_frameReuse`, but `WovenBody.frame` still allocates new `ridges` arrays of `{ x, y, z, alpha, owner }` objects at 30Hz whenever `ambient` advances by `0.032s`. When playback is paused (`stepState.playing === false` and `stepIndex` / `cacheKey` are unchanged), sampling `WovenBody.frame` can be throttled or skipped if the camera/user is idle, or `ridges` arrays can be pooled inside `_frameReuse`.
2. `sharedTissueSpriteTexture` (`designer8_quineling.js:309`) and `_lastTelemetryResult` (`line 238`), as well as `_lastArtifact` in `sherlockQuinelingCompiler.js:506`, are held at module scope without a teardown reset.

#### Drop-In Code Fix (`src/designers/designer8_quineling.js` & `src/engine/sherlockQuinelingCompiler.js`)
In `designer8_quineling.js` `dispose()`:
```javascript
  if (sharedTissueSpriteTexture) {
    sharedTissueSpriteTexture.dispose();
    sharedTissueSpriteTexture = null;
  }
  _lastTelemetryKey = '';
  _lastTelemetryResult = null;
```
And in `src/engine/sherlockQuinelingCompiler.js`, export a cache cleanup helper:
```javascript
export function clearCompiledQuinelingCache() {
  compiledQuinelingCache.clear();
  _lastRawSource = null;
  _lastStackLimit = -1;
  _lastInitialFuel = -1;
  _lastOutcomeKind = '';
  _lastArtifact = null;
}
```
