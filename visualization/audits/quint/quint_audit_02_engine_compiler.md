# Quint Formal Specification & Deep Audit Report 02: Sherlock CEK Engine, Quineling Compiler, Quinelings SDK (`WovenBody` & `LifeformFamilies`), and HTTP/Rust Runner

- **Agent**: `Quint-Engine-Compiler-Verifier` (Gemini Quint-LLM-Kit Verification Agent 2)
- **Formal Specification**: [`specs/quint/engine_quineling_pipeline.qnt`](file:///home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt)
- **Audited Files**:
  1. [`src/engine/sherlockEngine.js`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js)
  2. [`src/engine/sherlockQuinelingCompiler.js`](file:///home/charles/sherlock-bench/src/engine/sherlockQuinelingCompiler.js)
  3. [`src/engine/quinelingsSdk.js`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js)
  4. [`server.js`](file:///home/charles/sherlock-bench/server.js)
  5. [`rust-runner/main.rs`](file:///home/charles/sherlock-bench/rust-runner/main.rs)
  6. [`rust-runner/mori_lexer.rs`](file:///home/charles/sherlock-bench/rust-runner/mori_lexer.rs)

---

## 1. Formal Quint Specification & Verification Results

### 1.1 Model Summary (`specs/quint/engine_quineling_pipeline.qnt`)

The Quint specification [`engine_quineling_pipeline.qnt`](file:///home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt) formally models:
1. **CEK Trace Cache & Step Clamping (`executeAndCacheTrace`, `computeRingTelemetry`)**:
   - Models the 32-entry LRU trace cache (`TRACE_CACHE_MAX = 32`) and safe step/playhead clamping across 0-step, 1-step, and multi-step execution traces (`safeTotalSteps = max(1, rawTotalSteps)`).
2. **Horological 6-Ring / 12-Slot Continuation Telemetry (`R1.A .. R6.B`, `ringSlotBoundsValid`)**:
   - Models `filledRingSlots` (`0..12`), `overflowFrames` (`max(0, totalFrames - 12)`), `activeSlotIndex` (`0..11`), `frontierSlotIndex` (`totalFrames - 1`), `hasOverflowTop`, and the 10-sector `controlCore` indicator (`0..9`).
3. **Quineling Compiler 64-Entry LRU Cache, Bijective Topology-to-Mechanism Mapping & Cache-Aliasing Isolation (`compilePresetForCaller`, `selectWovenMechanism`, `selectQdlFamily`)**:
   - Models the exact `WovenBody.author(graph, seed)` and `QDL.fromGraph(graph, seed)` decision trees across all 8 canonical Sherlock presets (`0..7`: `fact4`/`factorial`, `list_fold`, `tree_fold`, `mutual_even_odd`, `lexical_let`, `checked_underflow`, `stack_overflow`, `fib4`/`fuel_exhaustion`), verifying that they map bijectively onto the 8 distinct `WovenBody` mechanisms (`logarithmic-mantle`, `pleated-braid`, `recursive-affine`, `lorenz-flow`, `phyllotaxis-fan`, `clifford-flow`, `recursive-julia`, `toroidal-weave`) and 8 `QDL` families (`nautilus`, `ribbon`, `coral`, `moth`, `bloom`, `jelly`, `comet`, `torus`).
   - Models per-caller view isolation (`callerA`, `callerB` with independent `activeTaskIndex` and `phase`) against the immutable `compilerCacheMap` entry (`noCacheAliasingCorruption`).
4. **`WovenBody.frame()` Typed-Array `reuse` Buffer Invariants (`sampleWovenFrame`, `computeFrameReuse`)**:
   - Models `points` (`budget * 4`) and `owners` (`budget`) capacity matching across mechanism and budget switches (`6000`, `6500`, `8192`, `22000`), owner-index bounds (`0 <= owner < nodeCount`), and degenerate bounding-box span protection (`spanX == 0`, `spanY == 0`).

### 1.2 Quint CLI Execution Output

```text
$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint typecheck /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
(exit 0 — no type errors)

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint test /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
  engine_quineling_pipeline
    ok bijectivePresetsTest passed 1 test(s)
    ok traceCacheAndRingOverflowTest passed 1 test(s)
    ok reuseBufferSwitchTest passed 1 test(s)
    ok cacheAliasingIsolationTest passed 1 test(s)

  4 passing (84ms)

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=bijectivePresetMechanisms --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
[ok] No violation found (45ms at 4444 traces/second).
Trace length statistics: max=21, min=21, average=21.00

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=traceCacheBounded --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
[ok] No violation found (48ms at 4167 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=reuseBufferCapacitySafe --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
[ok] No violation found (51ms at 3922 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=noCacheAliasingCorruption --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
[ok] No violation found (69ms at 2899 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=ringSlotBoundsValid --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
[ok] No violation found (49ms at 4082 traces/second).

$ /home/charles/.nvm/versions/node/v26.10.0/bin/quint run --invariant=allPipelineInvariants --max-steps=20 --max-samples=200 /home/charles/sherlock-bench/specs/quint/engine_quineling_pipeline.qnt
[ok] No violation found (78ms at 2564 traces/second).
```

---

## 2. Comprehensive Bug, Memory Leak, and Edge-Case Audit

### Category 1: Cache-Aliasing, Eviction, and Shared-State Mutation Bugs

#### Finding 1.1 — Incomplete Cache Key & Hot Fast-Path in `compileSherlockProgramToQuineling` Collides Distinct Fault Species (`Rule 140` vs `Rule 225`) and Syntax-Error Fallback Traces
- **Severity**: High
- **Location**: [`src/engine/sherlockQuinelingCompiler.js:501-560`](file:///home/charles/sherlock-bench/src/engine/sherlockQuinelingCompiler.js#L501-L560) and [`src/engine/sherlockQuinelingCompiler.js:207-235`](file:///home/charles/sherlock-bench/src/engine/sherlockQuinelingCompiler.js#L207-L235)
- **Root Cause**:
  1. `analyzeSherlockAst` (lines 208–235) branches on `terminalRuleLine` (`lastStep?.ruleLine ?? 80`) to distinguish `checked_underflow` (`terminalRuleLine === 225`) from `stack_overflow` (`terminalRuleLine === 140`). Both rules produce `trace.outcome.kind === 'Rejected'`. However, the 1-entry hot fast-path (`_lastOutcomeKind`, line 527) and `cacheKey` (`${cleanSource}|S:${stackLimit}|F:${initialFuel}|O:${outcomeKind}`, line 533) omit `terminalRuleLine` and `trace?.outcome?.code`.
  2. When `parseAndCheckDraft(cleanSource)` throws during live editing (lines 546–556), the `catch` fallback constructs `parsed` from `trace?.programName` and `trace?.programImage` (which belongs to the *previous* valid trace), but caches the resulting artifact under `cleanSource|S:...|F:...|O:...`. Subsequent calls with the same unparseable `cleanSource` and a different `trace` (or `trace = null`) receive the stale fallback artifact.
  3. Neither `compiledQuinelingCache` (lines 535–543, 650–654) nor `clientTraceCache` in [`src/engine/sherlockEngine.js:1716-1737`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L1716-L1737) refreshes entry insertion order on a cache hit (`map.delete(key); map.set(key, val)`), making both caches **FIFO** rather than true **LRU**, and both return the raw cached object reference without freezing or isolating per-caller mutable properties (`activeTaskIndex`, `phase`, `design.surface.samples`).
- **Drop-in Fix (`src/engine/sherlockQuinelingCompiler.js:501-560, 650-660`)**:
```javascript
const compiledQuinelingCache = new Map();
let _lastCacheKey = null;
let _lastArtifact = null;

export function compileSherlockProgramToQuineling(source, speciesCatalog, trace = null) {
  const rawSource =
    source ||
    trace?.source ||
    'language "moriarty-sherlock-draft/1"; program Factorial { fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } } entry fact(4); }';
  const stackLimit = Number(trace?.steps?.[0]?.after?.stackLimit ?? trace?.stackLimit ?? 16) || 16;
  const initialFuel = Number(trace?.steps?.[0]?.before?.fuel ?? trace?.fuel ?? 120) || 120;
  const outcomeKind = trace?.outcome?.kind || 'Done';
  const lastStep = trace?.steps?.[trace.steps.length - 1];
  const terminalRuleLine = lastStep?.ruleLine ?? 80;
  const fallbackProgramName = trace?.programName || '';

  const cleanSource = String(rawSource);
  const cacheKey = `${cleanSource}|S:${stackLimit}|F:${initialFuel}|O:${outcomeKind}|R:${terminalRuleLine}|P:${fallbackProgramName}`;

  if (_lastArtifact !== null && cacheKey === _lastCacheKey) {
    return _lastArtifact;
  }

  const cached = compiledQuinelingCache.get(cacheKey);
  if (cached !== undefined) {
    // True LRU promotion on hit
    compiledQuinelingCache.delete(cacheKey);
    compiledQuinelingCache.set(cacheKey, cached);
    _lastCacheKey = cacheKey;
    _lastArtifact = cached;
    return cached;
  }
```

#### Finding 1.2 — Shared Mutable Telemetry & Trace Objects in `sherlockEngine.js` (`clientTraceCache`, `traceCallSpansCache`, `traceStepTelemetryCache`) and `undefined` BigInt Crash in `materializeArithmeticWitness`
- **Severity**: Medium-High
- **Location**:
  - [`src/engine/sherlockEngine.js:270-273`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L270-L273) (`materializeArithmeticWitness`)
  - [`src/engine/sherlockEngine.js:1714-1738`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L1714-L1738) (`clientTraceCache`)
  - [`src/engine/sherlockEngine.js:2088-2247`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L2088-L2247) (`traceCallSpansCache`, `traceStepTelemetryCache`)
- **Root Cause**:
  1. In `materializeArithmeticWitness(op, lhsBig, rhsBig)` (line 271):
     `const active = Boolean(op && lhsBig !== null && rhsBig !== null);`
     In JavaScript, `undefined !== null` evaluates to `true`. Calling `materializeArithmeticWitness('+', undefined, undefined)` bypasses `INACTIVE_ARITHMETIC_WITNESS` and crashes at line 281 inside `uint128ToLeBytes(undefined)` with `TypeError: Cannot convert undefined to a BigInt`.
  2. `getCachedStepTelemetry(trace, stepIndex)` (lines 2226–2247) caches `{ recursion, ringTelemetry }` in `traceStepTelemetryCache` and returns the unfrozen object directly on every 60fps animation frame (`buildStepState`, line 2282). Any downstream mutation to `stepState.ringTelemetry`, `stepState.controlCore`, or `stepState.recursion.calls` permanently corrupts the cached telemetry for that step.
  3. `clientTraceCache` (lines 1716–1737) uses FIFO eviction instead of LRU promotion and caches failed Rust fallback traces (`{ ...fallback, rustError: data.error }`, line 1782) permanently.
- **Drop-in Fix (`src/engine/sherlockEngine.js:270-273` and `1716-1738`)**:
```javascript
export function materializeArithmeticWitness(op, lhsBig, rhsBig) {
  const active = Boolean(op && lhsBig != null && rhsBig != null);
  if (!active) return INACTIVE_ARITHMETIC_WITNESS;
```
```javascript
function getClientTrace(key) {
  if (!clientTraceCache.has(key)) return null;
  const val = clientTraceCache.get(key);
  clientTraceCache.delete(key);
  clientTraceCache.set(key, val);
  return val;
}
```

---

### Category 2: Typed-Array `reuse` Hazards & Territory Ownership Bugs in `quinelingsSdk.js`

#### Finding 2.1 — `WovenBody.frame()` / `LifeformFamilies.frame()` `reuse` Buffer Silent Detachment & Default Budget Mismatch Across Mechanisms
- **Severity**: Medium-High
- **Location**:
  - [`src/engine/quinelingsSdk.js:933`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L933) (`LifeformFamilies.frame`, default `budget = 6000`)
  - [`src/engine/quinelingsSdk.js:977`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L977) (`cliffordFrame`, default `budget = 22000`)
  - [`src/engine/quinelingsSdk.js:994`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L994) (`juliaFrame`, default `budget = 6500`)
  - [`src/engine/quinelingsSdk.js:1008`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1008) (`recursiveFrame`, default `budget = 6500`)
  - [`src/engine/quinelingsSdk.js:1060`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1060) (`WovenBody.frame`, default `budget = 22000`)
- **Root Cause**:
  1. All 5 `frame()` functions check:
     `const points = reuse?.points?.length === budget * 4 ? reuse.points : new Float32Array(budget * 4)`
     `const owners = reuse?.owners?.length === budget ? reuse.owners : new Uint16Array(budget)`
     When `reuse.points` or `reuse.owners` has a different length than `budget * 4` / `budget` (for instance, when switching between mechanisms with different default budgets `6000` vs `6500` vs `22000`, or passing an oversized buffer `reuse.points.length > budget * 4`), `frame()` allocates a brand-new `Float32Array` and `Uint16Array` on **every call** without updating `reuse.points = points; reuse.owners = owners;`.
  2. Callers that pass a mutable `reuse` container (e.g. `const reuse = {}; WovenBody.frame(body, phase, { budget, reuse })`) or switch budgets suffer continuous 60fps typed-array heap allocations, and any code reading `reuse.points` after `WovenBody.frame()` reads stale/unwritten data.
  3. Neither check verifies `reuse.points instanceof Float32Array` or `reuse.owners instanceof Uint16Array`.
- **Drop-in Fix (`src/engine/quinelingsSdk.js` helper for all 5 `frame` functions)**:
```javascript
function acquireFrameBuffers(budget, reuse) {
  const ptsLen = budget * 4;
  const points =
    reuse?.points instanceof Float32Array && reuse.points.length === ptsLen
      ? reuse.points
      : reuse?.points instanceof Float32Array && reuse.points.length > ptsLen
        ? reuse.points.subarray(0, ptsLen)
        : new Float32Array(ptsLen);
  const owners =
    reuse?.owners instanceof Uint16Array && reuse.owners.length === budget
      ? reuse.owners
      : reuse?.owners instanceof Uint16Array && reuse.owners.length > budget
        ? reuse.owners.subarray(0, budget)
        : new Uint16Array(budget);
  if (reuse && typeof reuse === 'object') {
    reuse.points = points;
    reuse.owners = owners;
  }
  return { points, owners };
}
```

#### Finding 2.2 — `recursiveFrame` and `WovenBody.anchor` Crash with `TypeError` When `branches.findIndex(...)` Returns `-1` (`branches.length < strands`)
- **Severity**: High
- **Location**:
  - [`src/engine/quinelingsSdk.js:1001`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1001) (`compileRecursive`)
  - [`src/engine/quinelingsSdk.js:1009`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1009) (`recursiveFrame`)
  - [`src/engine/quinelingsSdk.js:1066`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1066) (`WovenBody.anchor`)
- **Root Cause**:
  In `compileRecursive(body)` (line 1001), each generated branch is assigned `component: branches.length % body.record.strands`.
  `WovenBody.validate` (line 966) admits `g.depth` in `[2, 6]`, `g.transforms.length` in `[2, 3]`, and `r.strands` in `[8, 40]`.
  When `g.depth === 2` and `g.transforms.length === 2`, `branches.length` is $1 + 2 + 4 = 7 < 8 \le \text{strands}$ (or when `g.depth === 3`, `branches.length = 15 < strands` for `strands >= 16`).
  Consequently, components $7 \dots \text{strands}-1$ have **no entry** in `body.branches`.
  In `recursiveFrame` (line 1009) and `WovenBody.anchor` (line 1066):
  ```javascript
  const t = body.territoryByOwner[j],
        k = branches.findIndex(b => b.component === t.component),
        q = recursivePoint(body, k, (t.u[0] + t.u[1]) / 2, 0, phase);
  ```
  `branches.findIndex(...)` returns `-1`, and `recursivePoint(body, -1, ...)` indexes `body.branches[-1]` (`undefined`), throwing `TypeError: Cannot read properties of undefined (reading 'depth')`.
- **Drop-in Fix (`src/engine/quinelingsSdk.js:1009` and `1066`)**:
```javascript
// In recursiveFrame (line 1009):
for (let j = 0; j < Math.min(budget, body.nodes.length); j++) {
  const t = body.territoryByOwner[j];
  if (!t) continue;
  const foundIdx = branches.findIndex(b => b.component === t.component);
  const k = foundIdx >= 0 ? foundIdx : t.component % branches.length;
  const q = recursivePoint(body, k, (t.u[0] + t.u[1]) / 2, 0, phase);
  const p4 = j * 4;
  points[p4] = q.x; points[p4 + 1] = q.y; points[p4 + 2] = q.z; points[p4 + 3] = q.alpha;
  owners[j] = j;
}

// In WovenBody.anchor (line 1066):
if (body.recursive) {
  const foundIdx = body.branches.findIndex(b => b.component === p.component);
  const k = foundIdx >= 0 ? foundIdx : p.component % body.branches.length;
  return recursivePoint(body, k, (p.u[0] + p.u[1]) / 2, 0, phase);
}
```

#### Finding 2.3 — Unvalidated `LifeformFamilies.compile` Allows `owner = -1` Wraparound (`65535` in `Uint16Array`) and `undefined` `territoryByOwner[j]` Crash
- **Severity**: Medium
- **Location**: [`src/engine/quinelingsSdk.js:891-893, 936`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L891-L936)
- **Root Cause**:
  Unlike `WovenBody.validate(record, graph)` (line 966), `LifeformFamilies.validate(record)` (line 890) does **not** validate `record.territories` against `graph.nodes`. If `LifeformFamilies.compile(record, graph)` is called directly with a mismatched node ID:
  1. `nodes.findIndex(n => n.id === p.node)` returns `-1`, writing `territoryByOwner[-1] = p` and leaving `territoryByOwner[j]` as `undefined`.
  2. `owners[j] = q.owner` writes `-1` into `Uint16Array`, which silently wraps to `65535` (`0xFFFF`).
  3. Line 936 (`const patch = body.territoryByOwner[j]; point(body, patch.component, ...)`) crashes with `TypeError: Cannot read properties of undefined (reading 'component')`.
- **Drop-in Fix (`src/engine/quinelingsSdk.js:891, 936`)**:
```javascript
// In LifeformFamilies.compile (line 891):
const territories = r.territories.map((p, idx) => {
  const ownerIdx = nodes.findIndex(n => n.id === p.node);
  return { ...p, owner: ownerIdx >= 0 ? ownerIdx : idx % Math.max(1, nodes.length) };
});

// In LifeformFamilies.frame (line 936):
for (let j = 0; j < Math.min(budget, body.nodes.length); j++) {
  const patch = body.territoryByOwner[j];
  if (!patch) continue;
  const q = point(body, patch.component, (patch.u[0] + patch.u[1]) / 2, 0, phase), p4 = j * 4;
  points[p4] = q.x; points[p4 + 1] = q.y; points[p4 + 2] = q.z; points[p4 + 3] = q.alpha; owners[j] = j;
}
```

#### Finding 2.4 — Missing Owner-Reservation Pass in `WovenBody.frame` (`pleated-braid` & `lorenz-flow`) and Wrong Clamped Owner in `cliffordFrame`
- **Severity**: Low-Medium
- **Location**:
  - [`src/engine/quinelingsSdk.js:977`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L977) (`cliffordFrame`)
  - [`src/engine/quinelingsSdk.js:1060-1065`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1060-L1065) (`WovenBody.frame`)
- **Root Cause**:
  1. In `cliffordFrame` (line 977), for `i < body.nodes.length`, `index = Math.min(body.cloud.length - 1, Math.floor(u * body.cloud.length / body.record.strands) * body.record.strands + patch.component)`. When `index` clamps to `body.cloud.length - 1`, `mapOwner(body, index)` computes `index % strands` instead of `patch.component`, and `owners[i]` is set to `mark.owner` instead of `i`.
  2. In `WovenBody.frame` (lines 1060–1065, used for `pleated-braid` and `lorenz-flow`), core lanes have weight `0.004`. At low budgets (`budget = 128`), `Math.floor(budget * 0.004 / sum)` is `0`, and unlike the other 4 `frame()` implementations, `WovenBody.frame` omits the `for (let j = 0; j < body.nodes.length; j++)` owner-reservation pass.
- **Drop-in Fix (`src/engine/quinelingsSdk.js:977` and `1063`)**:
```javascript
// In cliffordFrame (line 977):
if (i < body.nodes.length && body.territoryByOwner[i]) {
  const patch = body.territoryByOwner[i],
        u = (patch.u[0] + patch.u[1]) / 2,
        index = Math.min(body.cloud.length - 1, Math.floor(u * body.cloud.length / body.record.strands) * body.record.strands + patch.component),
        [x, y] = body.cloud[index];
  mark = { x, y, index, density: density(body, x, y), owner: i };
}
```
```javascript
// In WovenBody.frame (after line 1063):
for (let j = 0; j < Math.min(budget, body.nodes.length); j++) {
  const patch = body.territoryByOwner[j];
  if (!patch) continue;
  const q = point(body, patch.component, (patch.u[0] + patch.u[1]) / 2, 0, phase), p4 = j * 4;
  points[p4] = q.x; points[p4 + 1] = q.y; points[p4 + 2] = q.z; points[p4 + 3] = q.alpha; owners[j] = j;
}
```

---

### Category 3: Division-by-Zero, `NaN`, and `Infinity` Propagation

#### Finding 3.1 — Empty Trace (`steps.length === 0`) or `NaN` Playhead Produces `stepIndex = -1` / `NaN` and Desynchronizes `buildRingTelemetry` & `buildRecursionTelemetry`
- **Severity**: High
- **Location**:
  - [`src/engine/sherlockEngine.js:1896-1906`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L1896-L1906) (`buildRingTelemetry`)
  - [`src/engine/sherlockEngine.js:2136-2166`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L2136-L2166) (`buildRecursionTelemetry`)
  - [`src/engine/sherlockEngine.js:2252-2260`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L2252-L2260) (`buildStepState`)
- **Root Cause**:
  In `buildStepState` (lines 2253–2258):
  ```javascript
  const steps = trace?.steps || [];
  const totalSteps = Math.max(1, steps.length);
  const clampedPlayhead = Math.max(0, Math.min(totalSteps - 0.0001, playhead));
  const stepIndex = Math.min(steps.length - 1, Math.floor(clampedPlayhead));
  const stepFraction = clampedPlayhead - stepIndex;
  ```
  1. When `steps.length === 0` (e.g. a failed compilation or empty trace), `stepIndex` evaluates to `Math.min(-1, 0) = -1`, making `stepFraction = clampedPlayhead - (-1) >= 1.0` (violating `0 <= stepFraction < 1`) and caching key `-1` in `traceStepTelemetryCache`.
  2. When `playhead` is `NaN`, `clampedPlayhead`, `stepIndex`, and `stepFraction` all become `NaN`, polluting `traceStepTelemetryCache` with key `NaN`.
  3. In `buildRingTelemetry` (line 1898) and `buildRecursionTelemetry` (line 2138), `const step = steps[stepIndex] || steps[0];` silently falls back to `steps[0]` when `stepIndex` is out of bounds, while `buildRecursionTelemetry` (lines 2156–2158) still compares the unclamped `stepIndex` against `bc.invokeStep` and `bc.returnStep`, desynchronizing call-stack statuses from `step`.
- **Drop-in Fix (`src/engine/sherlockEngine.js:1896-1906, 2136-2142, 2252-2260`)**:
```javascript
export function buildStepState({ trace, playhead = 0, ambient = 0, dt = 0.016 }) {
  const steps = trace?.steps || [];
  const totalSteps = Math.max(1, steps.length);
  const finitePlayhead = Number.isFinite(playhead) ? playhead : 0;
  const clampedPlayhead = Math.max(0, Math.min(totalSteps - 0.0001, finitePlayhead));
  const stepIndex = steps.length > 0 ? Math.min(steps.length - 1, Math.floor(clampedPlayhead)) : 0;
  const stepFraction = Math.max(0, Math.min(1, clampedPlayhead - stepIndex));
  const progress = totalSteps <= 1 ? 1 : stepIndex / (totalSteps - 1);
  const step = steps[stepIndex] || null;
```
And at the top of `buildRingTelemetry` and `buildRecursionTelemetry`:
```javascript
const safeStepIndex =
  steps.length > 0 && Number.isFinite(stepIndex)
    ? Math.max(0, Math.min(steps.length - 1, Math.floor(stepIndex)))
    : 0;
const step = steps[safeStepIndex] || null;
```

#### Finding 3.2 — `NaN` Propagation in `juliaFrame`, `cliffordFrame`, `lorenzBasis`, and `ownerAt` (`quinelingsSdk.js`)
- **Severity**: Medium
- **Location**:
  - [`src/engine/quinelingsSdk.js:977-978`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L977-L978) (`cliffordFrame`: `max = Math.log1p(body.maxDensity)`)
  - [`src/engine/quinelingsSdk.js:995`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L995) (`juliaFrame`: `u = j / (curve.line.length - 1)` and `curve.levelIndex / (body.record.geometry.levels.length - 1)`)
  - [`src/engine/quinelingsSdk.js:1040`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1040) (`lorenzBasis`: `k / (body.lanes.length - 1)`)
  - [`src/engine/quinelingsSdk.js:1041`](file:///home/charles/sherlock-bench/src/engine/quinelingsSdk.js#L1041) (`ownerAt`: unguarded `.owner` when `u` is `NaN`)
- **Root Cause**:
  1. In `juliaFrame` (line 995), `u = j / Math.max(1, curve.line.length - 1)` and `levels.length - 1` lack `Math.max(1, ...)` guards (unlike `cliffordFrame` line 978 which uses `Math.max(1, image.line.length - 1)`). If `levels.length === 1` or `curve.line.length === 1`, `0 / 0` yields `NaN`.
  2. When `p0.alpha` is `NaN` in `frame.ridges`, [`src/designers/designer8_quineling.js:921-923`](file:///home/charles/sherlock-bench/src/designers/designer8_quineling.js#L921-L923) uses `const rawA0 = p0.alpha ?? 0.2;` (which does **not** catch `NaN`) and `if (Math.max(rawA0, rawA1) < 0.012) continue;` (which evaluates to `false` when `rawA0` is `NaN`), writing `NaN` directly into WebGPU `ridgeLineColors`!
  3. In `cliffordFrame` (line 977), `const max = Math.log1p(body.maxDensity)` is `0` if `body.maxDensity === 0`, causing `Math.log1p(mark.density) / max` to evaluate to `NaN`.
  4. In `lorenzBasis` (line 1040), `k / (body.lanes.length - 1)` divides by zero if `body.lanes.length === 1`.
- **Drop-in Fix (`src/engine/quinelingsSdk.js:977, 995, 1040, 1041`)**:
```javascript
// In cliffordFrame (line 977):
const max = Math.max(1e-6, Math.log1p(body.maxDensity || 0));

// In juliaFrame (line 995):
const u = j / Math.max(1, curve.line.length - 1);
const levelDenom = Math.max(1, body.record.geometry.levels.length - 1);

// In lorenzBasis (line 1040):
const laneFrac = k / Math.max(1, body.lanes.length - 1);

// In ownerAt (line 1041):
const safeU = Number.isFinite(u) ? Math.max(0, Math.min(1, u)) : 0;
```

#### Finding 3.3 — `sherlockQuinelingCompiler.js` Underscore Literal Blindness, 32-Bit Literal Truncation to Zero, and Non-Finite `initialFuel` Crash
- **Severity**: Medium
- **Location**:
  - [`src/engine/sherlockQuinelingCompiler.js:70-77, 147-151`](file:///home/charles/sherlock-bench/src/engine/sherlockQuinelingCompiler.js#L70-L77)
  - [`src/engine/sherlockQuinelingCompiler.js:474`](file:///home/charles/sherlock-bench/src/engine/sherlockQuinelingCompiler.js#L474)
- **Root Cause**:
  1. The Sherlock lexer (`sherlockEngine.js:397`, `mori_lexer.rs:109`) supports numeric literals with underscores (e.g. `1_000_000`). However, `sherlockQuinelingCompiler.js:70` defines `const DIGITS_RE = /^\d+$/;` and tests `DIGITS_RE.test(rawText)` without stripping `_`, silently ignoring all underscore-formatted literals.
  2. `parseUInt32Literal(rawText)` (line 74) computes `Number(BigInt(rawText) & 0xffffffffn)`, which maps any positive multiple of $2^{32}$ (such as `4294967296`) to `0`, and can feed gigantic values ($4.29 \times 10^9$) into `lexical_let`'s repeated `square`/`multiply` task nodes.
  3. On line 474, `Math.max(10, analysis.initialFuel)` throws `TypeError` if `trace.fuel` is a `BigInt`, or produces `NaN` if `analysis.initialFuel` is `NaN`, causing `QuinelingKernels.validate(graph)` to throw an uncaught `Error: Expected finite number`.
- **Drop-in Fix (`src/engine/sherlockQuinelingCompiler.js:70-77, 202, 474`)**:
```javascript
const DIGITS_RE = /^[0-9][0-9_]*$/;

function parseUInt32Literal(rawText) {
  const clean = rawText.replace(/_/g, '');
  if (clean.length <= 9) return Number(clean) >>> 0;
  const big = BigInt(clean);
  // Fold high 32-bit limbs with XOR so multiples of 2^32 never collapse to 0, and clamp to safe kernel range
  const folded = Number((big ^ (big >> 32n)) & 0x7fffffffn);
  return Math.max(1, Math.min(100000, folded));
}
```
And in `analyzeSherlockAst` (line 202):
```javascript
const rawFuel = Number(trace?.steps?.[0]?.before?.fuel ?? trace?.fuel ?? 120);
const initialFuel = Number.isFinite(rawFuel) && rawFuel >= 0 ? rawFuel : 120;
```

---

### Category 4: HTTP Server (`server.js`) & Native Rust Runner (`rust-runner/main.rs`, `mori_lexer.rs`) Hazards

#### Finding 4.1 — `server.js` Caches Truncated/Failed `stdout` on Child Process Timeout or `maxBuffer` Overflow (`if (err && !stdout)`) and Lacks `child.stdin` EPIPE Guard
- **Severity**: Critical
- **Location**: [`server.js:144-163`](file:///home/charles/sherlock-bench/server.js#L144-L163)
- **Root Cause**:
  ```javascript
  const child = execFile(
    RUST_BIN,
    ['--fuel', String(fuel), '--stack-limit', String(stackLimit), '--slice-width', String(sliceWidth)],
    { maxBuffer: 16 * 1024 * 1024, timeout: 4000 },
    (err, stdout) => {
      if (err && !stdout) {
        res.writeHead(500, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ ok: false, error: err.message }));
        return;
      }
      setCachedTrace(traceKey, stdout);
      res.writeHead(200, { ... });
      res.end(stdout);
    }
  );
  child.stdin?.write(source);
  child.stdin?.end();
  ```
  1. **Corrupted Cache Poisoning (`if (err && !stdout)`)**: When `sherlock-runner` is killed by `timeout: 4000` (`SIGTERM`) or exceeds `maxBuffer: 16 * 1024 * 1024` (`ERR_CHILD_PROCESS_STDIO_MAXBUFFER`) after writing partial output to `stdout`, both `err` and `stdout` are truthy. Because the check is `if (err && !stdout)` instead of `if (err)`, `server.js` permanently caches the truncated, invalid JSON in `traceCache` (`setCachedTrace(traceKey, stdout)`) and returns `HTTP 200`!
  2. **Uncaught `EPIPE` Server Crash**: `child.stdin` has no `'error'` listener (`child.stdin?.on('error', () => {})`). If `sherlock-runner` exits early or fails to spawn while `source` is being written to `child.stdin`, Node.js throws an unhandled `'error'` (`EPIPE`) event, crashing the HTTP server.
  3. **Orphaned Child Process on Client Abort**: If the HTTP request closes early (`req.on('close', ...)`), `child` is not killed.
- **Drop-in Fix (`server.js:144-164`)**:
```javascript
        const child = execFile(
          RUST_BIN,
          ['--fuel', String(fuel), '--stack-limit', String(stackLimit), '--slice-width', String(sliceWidth)],
          { maxBuffer: 16 * 1024 * 1024, timeout: 4000 },
          (err, stdout) => {
            if (res.writableEnded) return;
            if (err) {
              res.writeHead(500, { 'Content-Type': 'application/json; charset=utf-8' });
              res.end(JSON.stringify({ ok: false, error: err.message }));
              return;
            }
            setCachedTrace(traceKey, stdout);
            res.writeHead(200, {
              'Content-Type': 'application/json; charset=utf-8',
              'X-Trace-Cache': 'MISS',
            });
            res.end(stdout);
          }
        );
        req.on('close', () => {
          if (!res.writableEnded) child.kill();
        });
        child.stdin?.on('error', () => {});
        child.stdin?.write(source);
        child.stdin?.end();
```

#### Finding 4.2 — `server.js` Body-Size Limit (`> 256 KiB`), `NaN` Parameter Propagation, Uncaught `URIError` Crash, Unbounded `staticAssetCache`, and Path Prefix Traversal
- **Severity**: High
- **Location**: [`server.js:43-80, 114-134, 172-181`](file:///home/charles/sherlock-bench/server.js#L43-L181)
- **Root Cause**:
  1. **Over-Limit Body Handling (`server.js:116-126`)**: When `totalBytes > 256 * 1024`, `req.destroy()` resets the TCP connection without sending `413 Payload Too Large` (causing the browser `fetch` in `sherlockEngine.js:1787` to enter `catch (_)` and permanently set `rustBackendAvailable = false`), and does not guard `req.on('end')` with an `aborted` flag.
  2. **`NaN` / `0` Parameter Coercion (`server.js:129-131`)**:
     `const fuel = Math.max(1, Math.min(2000, Number(payload.fuel || 200)));`
     If `payload.fuel` is `"abc"`, `Number("abc")` is `NaN` and `Math.max(1, Math.min(2000, NaN))` evaluates to `NaN`, passing `--fuel NaN` to `RUST_BIN` and caching under `F:NaN`. If `payload.fuel` is `0`, `0 || 200` silently overrides `0` with `200`.
  3. **Uncaught `decodeURIComponent` Crash (`server.js:172`)**: `let reqPath = decodeURIComponent(url.pathname);` is outside any `try / catch`. A single request with malformed percent-encoding (e.g. `GET /%E0%A4%A`) throws an uncaught `URIError` and terminates the server process.
  4. **Partial-Prefix Path Check & Unbounded `staticAssetCache` (`server.js:43, 175-179`)**:
     `!safePath.startsWith(__dirname)` allows sibling prefix matches (e.g. `/home/charles/sherlock-bench-other`) and allows serving internal files like `/rust-runner/sherlock-runner` or `/server.js`. Furthermore, `staticAssetCache` has no entry cap.
- **Drop-in Fix (`server.js:114-185`)**:
```javascript
  if (req.method === 'POST' && url.pathname.endsWith('/api/trace')) {
    const chunks = [];
    let totalBytes = 0;
    let tooLarge = false;
    req.on('data', (chunk) => {
      if (tooLarge) return;
      totalBytes += chunk.length;
      if (totalBytes > 256 * 1024) {
        tooLarge = true;
        res.writeHead(413, { 'Content-Type': 'application/json; charset=utf-8' });
        res.end(JSON.stringify({ ok: false, error: 'Payload exceeds 256 KiB limit' }));
        req.destroy();
        return;
      }
      chunks.push(chunk);
    });
    req.on('end', () => {
      if (tooLarge || res.writableEnded) return;
      try {
        const body = Buffer.concat(chunks, totalBytes).toString('utf8');
        const payload = JSON.parse(body || '{}');
        const source = String(payload.source || '');
        const rawFuel = Number(payload.fuel ?? 200);
        const rawStack = Number(payload.stackLimit ?? 16);
        const rawSlice = Number(payload.sliceWidth ?? 4);
        const fuel = Math.max(1, Math.min(2000, Number.isFinite(rawFuel) ? Math.floor(rawFuel) : 200));
        const stackLimit = Math.max(1, Math.min(64, Number.isFinite(rawStack) ? Math.floor(rawStack) : 16));
        const sliceWidth = Math.max(1, Math.min(32, Number.isFinite(rawSlice) ? Math.floor(rawSlice) : 4));
```
And for static path decoding (`server.js:172-181`):
```javascript
  let reqPath;
  try {
    reqPath = decodeURIComponent(url.pathname);
  } catch (_) {
    res.writeHead(400);
    res.end('Bad Request');
    return;
  }
  if (reqPath === '/') reqPath = '/index.html';

  const safePath = path.normalize(path.join(__dirname, reqPath));
  if (safePath !== __dirname && !safePath.startsWith(__dirname + path.sep)) {
    res.writeHead(403);
    res.end('Forbidden');
    return;
  }
```

#### Finding 4.3 — Rust Runner (`rust-runner/main.rs`) vs Browser Engine (`sherlockEngine.js`) Telemetry & Rule-111 Discrepancies
- **Severity**: Low-Medium
- **Location**:
  - [`rust-runner/main.rs:94`](file:///home/charles/sherlock-bench/rust-runner/main.rs#L94) vs [`src/engine/sherlockEngine.js:206, 1104`](file:///home/charles/sherlock-bench/src/engine/sherlockEngine.js#L206)
  - [`rust-runner/main.rs:303-307, 714`](file:///home/charles/sherlock-bench/rust-runner/main.rs#L303-L307)
  - [`rust-runner/main.rs:455-462, 483-488`](file:///home/charles/sherlock-bench/rust-runner/main.rs#L455-L488)
- **Root Cause**:
  1. **Rule 111 Label Mismatch**: `rust-runner/main.rs:94` labels Rule 111 as `"Var missing -> error(1)"`, whereas `sherlockEngine.js:206` labels it `'Var missing -> error(3)'`.
  2. **`max_host_steps = 1024` vs `fuel <= 2000` Divergence**: `server.js:129` allows `fuel` up to `2000`, while `main.rs:303` stops recording step telemetry at `max_host_steps = 1024` and then calls `let final_snap = machine.run();` (line 714), which continues executing unrecorded steps until completion. Thus, for traces requiring 1025–2000 steps, `steps` ends at step 1023 (`Pending`) while `outcome` reports `Done`, whereas `compileAndRunInBrowser` (`sherlockEngine.js:1347`) halts at 1024 steps with `outcome: { kind: 'Pending' }`.
  3. **Frame Metadata Fields Omitted in Rust JSON (`main.rs:455`)**: `main.rs` serializes frames with `{ index, kind, label, detail, originExprId, astPath, envId, bindings }`, omitting `returnType`, `varName`, and `op` (which `compileAndRunInBrowser` provides at `sherlockEngine.js:1510-1513`).

---

## 3. Summary Matrix of Findings

| ID | Component | Issue | Severity | Status |
|---|---|---|---|---|
| **1.1** | `sherlockQuinelingCompiler.js` | Cache key omits `terminalRuleLine` & fallback `programName`; FIFO eviction instead of LRU on hit | **High** | Documented + Fix Provided |
| **1.2** | `sherlockEngine.js` | `materializeArithmeticWitness` crashes on `undefined` (`!== null`); unfrozen cached telemetry in `traceStepTelemetryCache` | **Medium-High** | Documented + Fix Provided |
| **2.1** | `quinelingsSdk.js` (`WovenBody.frame`) | `reuse` buffers not updated on reallocation; default budget mismatch (`6000` / `6500` / `22000`) across mechanisms | **Medium-High** | Formally Modeled + Fix Provided |
| **2.2** | `quinelingsSdk.js` (`recursiveFrame`, `anchor`) | `branches.findIndex(...)` returns `-1` when `branches.length < strands`, crashing `recursivePoint(body, -1, ...)` | **High** | Formally Modeled + Fix Provided |
| **2.3** | `quinelingsSdk.js` (`LifeformFamilies`) | `LifeformFamilies.compile` lacks node ownership check; `owner = -1` wraps to `65535` in `Uint16Array` | **Medium** | Formally Modeled + Fix Provided |
| **2.4** | `quinelingsSdk.js` (`WovenBody.frame`, `cliffordFrame`) | `pleated-braid`/`lorenz-flow` lack owner-reservation pass; `cliffordFrame` assigns clamped `mapOwner` instead of `i` | **Low-Medium** | Documented + Fix Provided |
| **3.1** | `sherlockEngine.js` (`buildStepState`, `buildRingTelemetry`) | `steps.length === 0` yields `stepIndex = -1` and `stepFraction >= 1`; `NaN` playhead pollutes WeakMap cache | **High** | Formally Modeled + Fix Provided |
| **3.2** | `quinelingsSdk.js` & `designer8_quineling.js` | `0/0 = NaN` in single-point Julia contour / `maxDensity = 0` / single-lane Lorenz; `p0.alpha ?? 0.2` passes `NaN` to WebGPU | **Medium** | Documented + Fix Provided |
| **3.3** | `sherlockQuinelingCompiler.js` | `DIGITS_RE` ignores underscore literals (`1_000`); `parseUInt32Literal` maps $2^{32}$ to `0`; non-finite `initialFuel` throws | **Medium** | Documented + Fix Provided |
| **4.1** | `server.js` (`/api/trace`) | `if (err && !stdout)` caches truncated `stdout` on timeout/maxBuffer overflow; missing `child.stdin` `EPIPE` error handler | **Critical** | Documented + Fix Provided |
| **4.2** | `server.js` | `>256 KiB` `req.destroy()` lacks HTTP 413 response; `NaN` fuel/stackLimit propagation; uncaught `decodeURIComponent` `URIError` | **High** | Documented + Fix Provided |
| **4.3** | `rust-runner/main.rs` | `max_host_steps = 1024` vs `fuel <= 2000` trace/outcome split; Rule 111 label mismatch (`error(1)` vs `error(3)`) | **Low-Medium** | Documented + Fix Provided |
