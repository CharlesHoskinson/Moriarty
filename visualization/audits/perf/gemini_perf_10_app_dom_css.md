# Gemini Argon High Performance Audit #10: App Controller, DOM Updates, Layout & CSS Compositing (`src/app.js`, `index.html`, `src/styles.css`)

Below is a prioritized, line-referenced audit of every performance optimization opportunity across `/home/charles/sherlock-bench/src/app.js`, `/home/charles/sherlock-bench/src/styles.css`, and `/home/charles/sherlock-bench/index.html` that preserves 100% of visual quality, Material Design 3 styling, and UI behavior.

---

## Priority 1 (Critical Hot Path): Eliminate Per-Step `innerHTML` Rebuilds in `syncStepUI()` (`src/app.js`)

During playback (`3.2 * speed` steps/sec) and timeline scrubbing (up to 60–120 Hz), `syncStepUI()` is invoked on every step transition (`src/app.js:455–773`). Currently, it performs **8 separate `.innerHTML = ...` assignments** that destroy, re-parse, allocate, and re-layout up to **110+ DOM nodes per step**.

### 1.1 Pre-build and Update `#sixRingsGauge` (6 Split-Arc Rings + 10-Sector Control Stator) In-Place
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:595–689`
- **Bottleneck**: Even though `state.lastRingsHtml` checks string equality, `ringsHtml` changes on **almost every single step** because `activeSectorKey` (`Eval` → `Push` → `Invoke` → `Enter` → `Bound` → `Return` → `PrimitiveResult`) or `chronoFrames` changes on each transition. Setting `els.sixRingsGauge.innerHTML = ringsHtml` destroys and recreates **6 `.ring-pill` grids, 12 `.ring-slot-row` rows, 24 nested spans, and 10 `.control-sector-chip` badges (~75 DOM nodes)** on every step. Yet the DOM topology of `#sixRingsGauge` is **100% static** (always 6 rings × 2 slots + 1 needle readout + 10 control sector chips).
- **Fix**: Build the 6 ring rows and 10 sector chips once at startup, cache their element references, and update only `.className`, `.textContent`, `.title`, `aria-current`, and `.hidden` when a slot or sector actually changes.

```javascript
// Replace lines 595–689 in /home/charles/sherlock-bench/src/app.js with a persistent DOM gauge:
function initSixRingsGaugeDOM() {
  if (!els.sixRingsGauge) return null;
  els.sixRingsGauge.innerHTML = '';
  const rings = [];

  for (let r = 0; r < 6; r++) {
    const idxA = r * 2;
    const idxB = r * 2 + 1;
    const slotLo = String(idxA + 1).padStart(2, '0');
    const slotHi = String(idxB + 1).padStart(2, '0');

    const pill = document.createElement('div');
    pill.className = 'ring-pill ring-idle';

    const top = document.createElement('div');
    top.className = 'ring-pill-top';
    const ringId = document.createElement('span');
    ringId.className = 'ring-id';
    ringId.textContent = `R${r + 1} (${slotHi})`;
    const ringIdx = document.createElement('span');
    ringIdx.className = 'ring-slots-idx';
    ringIdx.textContent = `${slotLo}–${slotHi}`;
    top.append(ringId, ringIdx);

    const rowA = document.createElement('div');
    rowA.className = 'ring-slot-row slot-idle';
    const mainA = document.createElement('span');
    mainA.className = 'slot-main';
    const keyA = document.createElement('span');
    keyA.className = 'slot-key';
    keyA.textContent = 'West A:';
    const valA = document.createElement('span');
    valA.className = 'slot-val';
    valA.textContent = '—';
    mainA.append(keyA, valA);
    rowA.appendChild(mainA);

    const rowB = document.createElement('div');
    rowB.className = 'ring-slot-row slot-idle';
    const mainB = document.createElement('span');
    mainB.className = 'slot-main';
    const keyB = document.createElement('span');
    keyB.className = 'slot-key';
    keyB.textContent = 'East B:';
    const valB = document.createElement('span');
    valB.className = 'slot-val';
    valB.textContent = '—';
    mainB.append(keyB, valB);
    rowB.appendChild(mainB);

    let overflowBadge = null;
    if (r === 5) {
      overflowBadge = document.createElement('span');
      overflowBadge.className = 'overflow-badge hidden';
      rowB.appendChild(overflowBadge);
    }

    pill.append(top, rowA, rowB);
    els.sixRingsGauge.appendChild(pill);

    rings.push({
      slotLo,
      slotHi,
      pill,
      rowA,
      valA,
      rowB,
      valB,
      overflowBadge,
      lastPillCls: '',
      lastPillTitle: '',
      lastRowACls: '',
      lastRowATitle: '',
      lastValA: '',
      lastFrontierA: false,
      lastRowBCls: '',
      lastRowBTitle: '',
      lastValB: '',
      lastFrontierB: false,
      lastOverflow: -1,
    });
  }

  const strip = document.createElement('div');
  strip.className = 'control-sector-strip';
  strip.setAttribute('aria-label', '10-Sector Control Stator');

  const stripTop = document.createElement('div');
  stripTop.className = 'control-strip-top';
  const stripLabel = document.createElement('span');
  stripLabel.textContent = '10-Sector Control Stator';
  const needleStrong = document.createElement('strong');
  stripTop.append(stripLabel, needleStrong);

  const grid = document.createElement('div');
  grid.className = 'control-sector-grid';
  const sectorChips = CONTROL_STRIP_SECTORS.map((sec) => {
    const chip = document.createElement('span');
    chip.className = 'control-sector-chip';
    chip.title = sec.title;
    chip.textContent = sec.badge;
    grid.appendChild(chip);
    return { keyLower: sec.key.toLowerCase(), key: sec.key, el: chip, lastCls: 'control-sector-chip', lastActive: false };
  });

  strip.append(stripTop, grid);
  els.sixRingsGauge.appendChild(strip);

  return { rings, needleStrong, lastNeedleText: '', sectorChips };
}
```
And inside `syncStepUI()` (`src/app.js:596–689`), update `ringsGaugeDOM` in-place:
```javascript
  if (ringsGaugeDOM && els.ringsSummaryBadge) {
    const visibleSlots = Math.min(12, chronoFrames.length);
    const summaryText =
      overflowFrames > 0
        ? `${visibleSlots}/12 (+${overflowFrames}) · FUEL ${fuelPct}% · CTRL: ${ctrlKindUpper}`
        : `${visibleSlots}/12 SLOTS · FUEL ${fuelPct}% · CTRL: ${ctrlKindUpper}`;
    if (summaryText !== state.lastRingsSummaryText) {
      state.lastRingsSummaryText = summaryText;
      els.ringsSummaryBadge.textContent = summaryText;
    }

    const frontierChronoIdx = chronoFrames.length > 0 ? Math.min(11, chronoFrames.length - 1) : -1;
    const activeRingIdx = frontierChronoIdx >= 0 ? Math.floor(frontierChronoIdx / 2) : -1;

    for (let r = 0; r < 6; r++) {
      const dom = ringsGaugeDOM.rings[r];
      const idxA = r * 2;
      const idxB = r * 2 + 1;
      const fA = chronoFrames[idxA];
      const fB = chronoFrames[idxB];
      const fill = (fA ? 1 : 0) + (fB ? 1 : 0);
      const hintA = resolveChronoEnvHint(chronoFrames, idxA, activeBindings);
      const hintB = resolveChronoEnvHint(chronoFrames, idxB, activeBindings);
      const metaA = getFrameSlotMeta(fA, hintA);
      const metaB = getFrameSlotMeta(fB, hintB);
      const isFocusRing = r === activeRingIdx;
      const isFrontierA = idxA === frontierChronoIdx;
      const isFrontierB = idxB === frontierChronoIdx;
      const isSlotBOverflow = r === 5 && overflowFrames > 0;

      const pillCls = `ring-pill ${fill === 0 ? 'ring-idle' : isFocusRing ? 'ring-active' : 'ring-occupied'}`;
      if (pillCls !== dom.lastPillCls) {
        dom.lastPillCls = pillCls;
        dom.pill.className = pillCls;
      }
      const pillTitle = `Ring R${r + 1} (Chrono Slots ${dom.slotLo}–${dom.slotHi}) · West A: ${metaA.readout} | East B: ${metaB.readout}`;
      if (pillTitle !== dom.lastPillTitle) {
        dom.lastPillTitle = pillTitle;
        dom.pill.title = pillTitle;
      }

      const rowACls = `ring-slot-row ${metaA.cls}${isFrontierA ? ' slot-frontier' : ''}`;
      if (rowACls !== dom.lastRowACls) {
        dom.lastRowACls = rowACls;
        dom.rowA.className = rowACls;
      }
      if (metaA.readout !== dom.lastValA) {
        dom.lastValA = metaA.readout;
        dom.valA.textContent = metaA.readout;
      }
      const rowATitle = `West Slot A (#${dom.slotLo}): ${metaA.tooltip}`;
      if (rowATitle !== dom.lastRowATitle) {
        dom.lastRowATitle = rowATitle;
        dom.rowA.title = rowATitle;
      }
      if (isFrontierA !== dom.lastFrontierA) {
        dom.lastFrontierA = isFrontierA;
        if (isFrontierA) dom.rowA.setAttribute('aria-current', 'true');
        else dom.rowA.removeAttribute('aria-current');
      }

      const rowBCls = `ring-slot-row ${metaB.cls}${isFrontierB ? ' slot-frontier' : ''}${isSlotBOverflow ? ' slot-overflow' : ''}`;
      if (rowBCls !== dom.lastRowBCls) {
        dom.lastRowBCls = rowBCls;
        dom.rowB.className = rowBCls;
      }
      if (metaB.readout !== dom.lastValB) {
        dom.lastValB = metaB.readout;
        dom.valB.textContent = metaB.readout;
      }
      const rowBTitle = `East Slot B (#${dom.slotHi}): ${metaB.tooltip}${isSlotBOverflow ? ` (+${overflowFrames} overflow beyond 12-slot window)` : ''}`;
      if (rowBTitle !== dom.lastRowBTitle) {
        dom.lastRowBTitle = rowBTitle;
        dom.rowB.title = rowBTitle;
      }
      if (isFrontierB !== dom.lastFrontierB) {
        dom.lastFrontierB = isFrontierB;
        if (isFrontierB) dom.rowB.setAttribute('aria-current', 'true');
        else dom.rowB.removeAttribute('aria-current');
      }
      if (dom.overflowBadge) {
        const nextOverflow = isSlotBOverflow ? overflowFrames : 0;
        if (nextOverflow !== dom.lastOverflow) {
          dom.lastOverflow = nextOverflow;
          if (nextOverflow > 0) {
            dom.overflowBadge.classList.remove('hidden');
            dom.overflowBadge.textContent = `+${nextOverflow} overflow`;
            dom.overflowBadge.title = `${nextOverflow} additional frame(s) beyond the 12-slot window`;
          } else {
            dom.overflowBadge.classList.add('hidden');
          }
        }
      }
    }

    const activeSectorKey = (
      statusInfo.label === 'Done'
        ? 'Return'
        : statusInfo.label === 'Rejected' || statusInfo.label === 'Exhausted'
          ? 'Error'
          : rawCtrlKind
    ).toLowerCase();

    const needleHemisphere =
      frontierChronoIdx < 0
        ? 'Center (0 frames)'
        : frontierChronoIdx % 2 === 0
          ? `West Slot A (R${activeRingIdx + 1}·A)`
          : `East Slot B (R${activeRingIdx + 1}·B)`;
    const needleText = `Needle → ${needleHemisphere}`;
    if (needleText !== ringsGaugeDOM.lastNeedleText) {
      ringsGaugeDOM.lastNeedleText = needleText;
      ringsGaugeDOM.needleStrong.textContent = needleText;
    }

    for (let i = 0; i < ringsGaugeDOM.sectorChips.length; i++) {
      const chip = ringsGaugeDOM.sectorChips[i];
      const isSecActive = chip.keyLower === activeSectorKey;
      const toneCls = isSecActive
        ? statusInfo.label === 'Done'
          ? ' is-done'
          : statusInfo.label === 'Rejected' || statusInfo.label === 'Exhausted' || chip.key === 'Error'
            ? ' is-fault'
            : ''
        : '';
      const nextCls = `control-sector-chip${isSecActive ? ' active' : ''}${toneCls}`;
      if (nextCls !== chip.lastCls) {
        chip.lastCls = nextCls;
        chip.el.className = nextCls;
      }
      if (isSecActive !== chip.lastActive) {
        chip.lastActive = isSecActive;
        if (isSecActive) chip.el.setAttribute('aria-current', 'step');
        else chip.el.removeAttribute('aria-current');
      }
    }
  }
```

---

### 1.2 Toggle `.active-line` in `updateEditorLineNumbers()` Instead of Rebuilding `innerHTML` on Every Step
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:346–362` (called on every step at line `517`)
- **Bottleneck**: Whenever `origin.line` changes from step to step (which is nearly every step), `updateEditorLineNumbers(origin.line)` loops `1..lines` and sets `els.editorLineNumbers.innerHTML = html`, destroying and recreating all line number nodes even when the total line count (`lines === state.lastLineCount`) has not changed.
- **Fix**: Rebuild `els.editorLineNumbers` DOM children **only** when `lines !== state.lastLineCount`, caching each line's `<span>` element in an array. When `activeLine` changes on a step, simply remove `.active-line` from the previous line's `<span>` and add `.active-line` to the new line's `<span>` (2 `classList` operations instead of an `innerHTML` parse).

```javascript
// Replace lines 346–362 in /home/charles/sherlock-bench/src/app.js:
let editorLineSpans = [];

function updateEditorLineNumbers(activeLine = 1) {
  if (!els.editorLineNumbers) return;
  const sourceText = els.sourceEditor.value || '';
  let lines = 1;
  for (let i = 0; i < sourceText.length; i++) {
    if (sourceText.charCodeAt(i) === 10) lines++;
  }

  if (lines !== state.lastLineCount) {
    state.lastLineCount = lines;
    state.lastActiveLine = -1;
    editorLineSpans = new Array(lines);
    const frag = document.createDocumentFragment();
    for (let i = 1; i <= lines; i++) {
      const span = document.createElement('span');
      span.textContent = String(i);
      editorLineSpans[i - 1] = span;
      frag.appendChild(span);
      frag.appendChild(document.createTextNode('\n'));
    }
    els.editorLineNumbers.replaceChildren(frag);
  }

  if (activeLine !== state.lastActiveLine) {
    const prevSpan = editorLineSpans[state.lastActiveLine - 1];
    if (prevSpan) prevSpan.classList.remove('active-line');
    const nextSpan = editorLineSpans[activeLine - 1];
    if (nextSpan) nextSpan.classList.add('active-line');
    state.lastActiveLine = activeLine;
  }
}
```
*(Note: Counting `\n` via `charCodeAt(10)` also avoids allocating an intermediate `.split('\n')` string array on every step!)*

---

### 1.3 Replace 5 Telemetry `<dd>` `.innerHTML` Rebuilds with Cached Text/`<small>`/`<strong>` Nodes
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:543–593`
- **Bottleneck**: `#telemetryTransition`, `#telemetryQdlMorph`, `#telemetryFrames`, `#telemetryCalls`, and `#telemetryFuel` each use `.innerHTML = ...` with `escapeHtml(...)` template strings on every step (`#telemetryTransition` and `#telemetryFuel` change on **100% of steps**).
- **Fix**: Pre-create `{ mainNode, smallNode }` child nodes once for each of those 5 `<dd>` elements and update `.nodeValue` / `.textContent` directly.

```javascript
// Initialize once after `els` declaration in /home/charles/sherlock-bench/src/app.js:
function initTelemetryPair(ddEl, useStrong = false) {
  if (!ddEl) return null;
  ddEl.textContent = '';
  const main = useStrong ? document.createElement('strong') : document.createTextNode('');
  const space = document.createTextNode(' ');
  const small = document.createElement('small');
  ddEl.append(main, space, small);
  return { main, small, lastMain: '', lastSmall: '' };
}

function setTelemetryPair(pair, mainText, smallText) {
  if (!pair) return;
  if (mainText !== pair.lastMain) {
    pair.lastMain = mainText;
    if (pair.main.nodeType === 3) pair.main.nodeValue = mainText;
    else pair.main.textContent = mainText;
  }
  if (smallText !== pair.lastSmall) {
    pair.lastSmall = smallText;
    pair.small.textContent = smallText;
  }
}

const telemetryDOM = {
  transition: initTelemetryPair(els.telemetryTransition, false),
  qdlMorph: initTelemetryPair(els.telemetryQdlMorph, true),
  frames: initTelemetryPair(els.telemetryFrames, false),
  calls: initTelemetryPair(els.telemetryCalls, false),
  fuel: initTelemetryPair(els.telemetryFuel, false),
};
```
And inside `syncStepUI()` (`src/app.js:543–593`):
```javascript
  setTelemetryPair(
    telemetryDOM.transition,
    `${clampedIdx + 1} / ${totalSteps}`,
    `Rule ${step.ruleId} · ${step.ruleName}`
  );
  setTelemetryPair(
    telemetryDOM.qdlMorph,
    qdlTel.speciesName,
    `· ${qdlTel.family} (${qdlTel.mechanism}) · ${qdlTel.morphGesture}`
  );
  const ringNote =
    overflowFrames > 0
      ? `· ${Math.min(12, chronoFrames.length)}/12 slots (${litRings}/6 rings, +${overflowFrames} overflow)`
      : `· ${chronoFrames.length}/12 slots (${litRings}/6 rings)`;
  setTelemetryPair(telemetryDOM.frames, String(frames.length), ringNote);
  setTelemetryPair(
    telemetryDOM.calls,
    String(activeCallCount),
    `· ${rec ? rec.calls.length : 0} total`
  );
  setTelemetryPair(
    telemetryDOM.fuel,
    `${step.after.fuel} / ${initialFuel}`,
    `· charge ${step.charge}`
  );
```

---

### 1.4 Pool DOM Nodes for `#frameStackList` and `#recursionCallsTrack` Instead of `.innerHTML` Replacement
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:691–772`
- **Bottleneck**:
  1. `els.frameStackList.innerHTML = framesHtml` (`src/app.js:729`) destroys and recreates all `.frame-item` cards whenever a frame is pushed, popped, or rewritten in-place.
  2. `els.recursionCallsTrack.innerHTML = callsHtml` (`src/app.js:770`) destroys and recreates all `<button class="rec-call-node">` elements on every call or return step. In addition to GC and layout churn, **destroying `<button>` nodes during playback breaks user clicks** (if `mousedown` occurs before a step tick and `mouseup` occurs after `innerHTML` replaces the button, the browser drops the `click` event!).
- **Fix**: Maintain pooled DOM element arrays (`frameItemPool` and `callNodePool`) and update existing DOM nodes in-place, toggling `.hidden` on unused pool slots.

```javascript
// Replace lines 691–772 in /home/charles/sherlock-bench/src/app.js with pooled DOM updaters:
const frameListPool = { emptyEl: null, items: [] };
const callsTrackPool = { emptyEl: null, items: [] };

function syncFramesListDOM(frames, stackLimit, activeBindings) {
  if (!els.frameStackList) return;
  const countText = `${frames.length} of ${stackLimit} limit`;
  if (countText !== state.lastFrameCountText && els.frameStackCount) {
    state.lastFrameCountText = countText;
    els.frameStackCount.textContent = countText;
  }

  if (!frameListPool.emptyEl) {
    els.frameStackList.textContent = '';
    const p = document.createElement('p');
    p.className = 'empty-note hidden';
    els.frameStackList.appendChild(p);
    frameListPool.emptyEl = p;
  }

  if (frames.length === 0) {
    const rootEnvStr =
      activeBindings.length > 0
        ? `Active env: ${activeBindings.map((b) => `${b.name} = ${b.valueSummary}`).join(', ')}`
        : 'No saved frames.';
    frameListPool.emptyEl.textContent = rootEnvStr;
    frameListPool.emptyEl.classList.remove('hidden');
    for (let i = 0; i < frameListPool.items.length; i++) {
      frameListPool.items[i].root.classList.add('hidden');
    }
    return;
  }

  frameListPool.emptyEl.classList.add('hidden');

  while (frameListPool.items.length < frames.length) {
    const root = document.createElement('div');
    root.className = 'frame-item';
    const top = document.createElement('div');
    top.className = 'frame-item-top';
    const op = document.createElement('span');
    op.className = 'frame-op';
    const tag = document.createElement('span');
    tag.className = 'frame-kind-tag';
    top.append(op, tag);
    const env = document.createElement('div');
    env.className = 'frame-env-row';
    root.append(top, env);
    els.frameStackList.appendChild(root);
    frameListPool.items.push({ root, op, tag, env, lastCls: '', lastOp: '', lastTag: '', lastEnv: '' });
  }

  for (let idx = 0; idx < frameListPool.items.length; idx++) {
    const item = frameListPool.items[idx];
    if (idx >= frames.length) {
      if (item.lastCls !== 'frame-item hidden') {
        item.lastCls = 'frame-item hidden';
        item.root.className = 'frame-item hidden';
      }
      continue;
    }
    const fr = frames[idx];
    const slotMeta = getFrameSlotMeta(fr);
    const chronoIdx = frames.length - 1 - idx;
    const ringNum = Math.floor(chronoIdx / 2) + 1;
    const slotChar = chronoIdx % 2 === 0 ? 'A' : 'B';
    const cls = `frame-item ${slotMeta.itemCls}${idx === 0 ? ' is-top' : ''}`;
    const opLabel = formatAlgebraicText(fr.label);
    const envText = resolveFrameEnvText(frames, idx, activeBindings);
    const borrowedNote = fr.kind === 'Right' || fr.kind === 'Unary' ? ' (enclosing)' : '';
    const kindTag =
      ringNum <= 6
        ? `R${ringNum}·${slotChar} · ${slotMeta.family} · ${fr.kind}`
        : `+${chronoIdx - 11} overflow · ${slotMeta.family} · ${fr.kind}`;
    const fullEnv = `Env${borrowedNote}: ${envText}`;

    if (cls !== item.lastCls) {
      item.lastCls = cls;
      item.root.className = cls;
    }
    if (opLabel !== item.lastOp) {
      item.lastOp = opLabel;
      item.op.textContent = opLabel;
    }
    if (kindTag !== item.lastTag) {
      item.lastTag = kindTag;
      item.tag.textContent = kindTag;
    }
    if (fullEnv !== item.lastEnv) {
      item.lastEnv = fullEnv;
      item.env.textContent = fullEnv;
    }
  }
}

function syncRecursionCallsDOM(rec, controlText) {
  if (!rec || !els.recursionModePill || !els.recursionCallsTrack) return;
  const modeLabel =
    rec.mode === 'RESOLVED'
      ? 'Resolved'
      : rec.mode === 'UNWINDING'
        ? 'Unwinding'
        : rec.mode === 'BASE CASE'
          ? 'Base case'
          : rec.mode === 'FAULT'
            ? 'Stopped'
            : 'Unfolding';
  if (modeLabel !== state.lastRecModeLabel) {
    state.lastRecModeLabel = modeLabel;
    els.recursionModePill.textContent = modeLabel;
  }
  const modeCls = `recursion-mode-pill ${
    rec.mode === 'UNWINDING' || rec.mode === 'RESOLVED'
      ? 'unwinding'
      : rec.mode === 'BASE CASE'
        ? 'base'
        : rec.mode === 'FAULT'
          ? 'fault'
          : ''
  }`;
  if (modeCls !== state.lastRecModeCls) {
    state.lastRecModeCls = modeCls;
    els.recursionModePill.className = modeCls;
  }

  if (!callsTrackPool.emptyEl) {
    els.recursionCallsTrack.textContent = '';
    const emptySpan = document.createElement('span');
    emptySpan.className = 'empty-note hidden';
    els.recursionCallsTrack.appendChild(emptySpan);
    callsTrackPool.emptyEl = emptySpan;
  }

  const calls = rec.calls;
  if (calls.length === 0) {
    callsTrackPool.emptyEl.textContent = `Entry expression → ${controlText}`;
    callsTrackPool.emptyEl.classList.remove('hidden');
    for (let i = 0; i < callsTrackPool.items.length; i++) {
      callsTrackPool.items[i].btn.classList.add('hidden');
      callsTrackPool.items[i].arrow.classList.add('hidden');
    }
    return;
  }

  callsTrackPool.emptyEl.classList.add('hidden');

  while (callsTrackPool.items.length < calls.length) {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'rec-call-node';
    const arrow = document.createElement('span');
    arrow.className = 'rec-call-arrow';
    arrow.textContent = '→';
    els.recursionCallsTrack.append(btn, arrow);
    callsTrackPool.items.push({ btn, arrow, lastCls: '', lastText: '', lastStep: -1, lastTitle: '' });
  }

  for (let idx = 0; idx < callsTrackPool.items.length; idx++) {
    const item = callsTrackPool.items[idx];
    if (idx >= calls.length) {
      item.btn.classList.add('hidden');
      item.arrow.classList.add('hidden');
      item.lastCls = 'hidden';
      continue;
    }
    const c = calls[idx];
    const retSuffix = c.status === 'returned' && c.returnValue ? ` = ${c.returnValue}` : '';
    const text = `${c.label}${retSuffix}`;
    const stepTarget = c.status === 'returned' && c.returnStep !== null ? c.returnStep : c.invokeStep;
    const cls = `rec-call-node ${c.status}`;
    const title = `Jump to ${c.label}`;

    if (cls !== item.lastCls) {
      item.lastCls = cls;
      item.btn.className = cls;
    }
    if (text !== item.lastText) {
      item.lastText = text;
      item.btn.textContent = text;
    }
    if (stepTarget !== item.lastStep) {
      item.lastStep = stepTarget;
      item.btn.dataset.step = String(stepTarget);
    }
    if (title !== item.lastTitle) {
      item.lastTitle = title;
      item.btn.title = title;
    }
    item.arrow.classList.toggle('hidden', idx >= calls.length - 1);
  }
}
```

---

## Priority 2: Eliminate Per-Step Algorithmic Redundancy & Uncached Queries (`src/app.js`)

### 2.1 Cache `buildStepState()` Per Step Across the Entire Trace Instead of Only a Single Step
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:910–943`
- **Bottleneck**: `getOrBuildStepState(stepIdx, stepFraction, dt)` only caches a single step (`state.cachedStepIdx`). Every time `playhead` advances to the next integer step, loops back to `0`, or is scrubbed across the timeline, `buildStepState()` runs from scratch:
  - `buildRecursionTelemetry(trace, stepIndex)` (`src/engine/sherlockEngine.js:2037–2146`) loops from `i = 0` to `steps.length` on **every step** ($O(N^2)$ across playback).
  - `buildRingTelemetry(trace, stepIndex)` (`src/engine/sherlockEngine.js:1841–2030`) clones and reverses frame arrays (`[...frames].reverse()`) and formats all 12 slot labels on every step.
  - Furthermore, inside `syncStepUI()` (`src/app.js:521`), `const chronoFrames = [...frames].reverse();` clones and reverses the frames array a third time!
- **Fix**: Because `trace.steps[clampedIdx]` is immutable for a given `state.trace`, cache built step states in a per-trace array `state.stepStateCache[clampedIdx]` (reset when `state.trace` changes in `executeCurrentSource`), and attach `chronoFrames` and `qdlTelemetry` to the cached step entry so both `animate()` and `syncStepUI()` reuse them in $O(1)$ time with zero re-computation when playback loops or scrubs.

```javascript
// Replace getOrBuildStepState (lines 910–943 in /home/charles/sherlock-bench/src/app.js):
function getOrBuildStepState(stepIdx, stepFraction = 0, dt = 0.016) {
  const totalSteps = state.trace?.steps?.length || 0;
  const clampedIdx = Math.max(0, Math.min(totalSteps - 1, stepIdx));

  if (state.cachedTraceRef !== state.trace || !Array.isArray(state.stepStateCache)) {
    state.cachedTraceRef = state.trace;
    state.stepStateCache = new Array(totalSteps);
  }

  let cached = state.stepStateCache[clampedIdx];
  if (!cached) {
    cached = buildStepState({
      trace: state.trace,
      playhead: clampedIdx,
      ambient: stage.ambient,
      dt,
    });
    const frames = cached.step?.after?.frames || [];
    cached.chronoFrames = [...frames].reverse();
    cached.qdlTelemetry = getQuinelingTelemetry({
      presetId: state.activePresetId,
      source: state.source,
      trace: state.trace,
      step: cached.step,
    });
    state.stepStateCache[clampedIdx] = cached;
  }

  // Update dynamic per-frame scalars in-place:
  cached.stepFraction = stepFraction;
  cached.progress =
    totalSteps > 1
      ? Math.min(1, Math.max(0, (clampedIdx + stepFraction) / (totalSteps - 1)))
      : 1;
  cached.ambient = stage.ambient;
  cached.dt = dt;
  cached.presetId = state.activePresetId;
  cached.source = state.source;

  if (clampedIdx === 0) {
    const step = cached.step;
    if (!step) {
      cached.phase = 'STILL';
      cached.aperture = 0.18;
    } else if (stepFraction < 0.45) {
      cached.phase = 'INGEST';
      cached.aperture = 0.45 + 0.5 * stepFraction;
    } else if (step.ruleLine === 82 || step.ruleLine === 84 || step.controlKind === 'Error') {
      cached.phase = 'ERROR';
      cached.aperture = 0.85;
    } else if (step.ruleLine === 80) {
      cached.phase = 'EMBLEM';
      cached.aperture = 0.68;
    } else if (step.controlKind === 'Return' || step.controlKind === 'PrimitiveResult' || step.ruleLine === 185) {
      cached.phase = step.after.frames.length === 0 ? 'RETURN' : 'RESOLVE';
      cached.aperture = 0.92;
    } else {
      cached.phase = 'DESCEND';
      cached.aperture = 0.92;
    }
  }

  return cached;
}
```

---

### 2.2 Move `activeAstBadge` Regex & Expression Lookup Out of `syncStepUI()` into `executeCurrentSource()`
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:502–513`
- **Bottleneck**: On **every step**, `syncStepUI()` searches `state.trace.programImage?.expressions?.find((e) => e.astPath === 'entry/')` and executes regex `String(state.source || '').match(/\bentry\s+([\s\S]*?)\s*;/)` to compute `els.activeAstBadge.textContent`, even though that badge text depends **only** on the compiled program and never changes between steps!
- **Fix**: Compute and assign `els.activeAstBadge.textContent` once in `executeCurrentSource()` right after `state.trace = res;` (`src/app.js:380`).

---

### 2.3 Cache Camera Lens Buttons & Preset Buttons Instead of Repeated `querySelectorAll` and `innerHTML = ''`
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:316–344` and `871–877`
- **Bottleneck**:
  - `els.cameraLensBar.querySelectorAll('button')` is queried at line `871` and re-queried on every camera preset switch in `updateCameraLensButtons()` at line `336`.
  - `renderPresets()` (`src/app.js:316–333`) sets `els.presetStrip.innerHTML = ''` and recreates all 8 preset `<button>` elements every time `selectPreset()` is called, destroying keyboard focus on the clicked preset button.
- **Fix**: Cache `const cameraLensBtns = els.cameraLensBar ? Array.from(els.cameraLensBar.querySelectorAll('button')) : [];` once at module init, and build the 8 preset buttons once in `renderPresets()`, toggling `.active` and `aria-pressed` in-place when `selectPreset()` runs.

```javascript
// Replace renderPresets & updateCameraLensButtons (lines 316–344 in /home/charles/sherlock-bench/src/app.js):
const cameraLensBtns = els.cameraLensBar
  ? Array.from(els.cameraLensBar.querySelectorAll('button'))
  : [];
let presetBtnEntries = [];

function renderPresets() {
  if (!els.presetStrip) return;
  if (presetBtnEntries.length === 0) {
    const frag = document.createDocumentFragment();
    for (const p of PRESET_PROGRAMS) {
      const meta = PRESET_EDITORIAL_META[p.id] || { title: p.title, sub: p.badge };
      const btn = document.createElement('button');
      btn.type = 'button';
      btn.className = 'preset-btn';
      const titleSpan = document.createElement('span');
      titleSpan.textContent = meta.title;
      const subSmall = document.createElement('small');
      subSmall.textContent = meta.sub;
      btn.append(titleSpan, subSmall);
      btn.addEventListener('click', () => selectPreset(p));
      frag.appendChild(btn);
      presetBtnEntries.push({ id: p.id, preset: p, btn });
    }
    els.presetStrip.replaceChildren(frag);
  }

  for (let i = 0; i < presetBtnEntries.length; i++) {
    const entry = presetBtnEntries[i];
    const isActive = entry.id === state.activePresetId;
    entry.btn.classList.toggle('active', isActive);
    entry.btn.setAttribute('aria-pressed', String(isActive));
    if (isActive && els.presetDescription) {
      els.presetDescription.textContent = entry.preset.description;
    }
  }
}

function updateCameraLensButtons(activePreset) {
  for (let i = 0; i < cameraLensBtns.length; i++) {
    const btn = cameraLensBtns[i];
    const isActive = btn.dataset.preset === activePreset;
    btn.classList.toggle('active', isActive);
    btn.setAttribute('aria-pressed', String(isActive));
  }
  if (els.stageInspectText) {
    const nextHint = getDefaultStageHint(activePreset);
    if (nextHint !== state.lastInspectText) {
      state.lastInspectText = nextHint;
      els.stageInspectText.textContent = nextHint;
    }
  }
}
```

---

### 2.4 Debounce / Coalesce `executeCurrentSource()` on Slider/Editor `input` Events & Guard Against Stale Async Traces
- **File & Lines**: `/home/charles/sherlock-bench/src/app.js:364–392` and `800–820`
- **Bottleneck**:
  - `els.fuelSlider`, `els.stackSlider`, and `els.sourceEditor` call `executeCurrentSource(false)` synchronously on every `input` event (`src/app.js:802, 813, 819`).
  - `executeCurrentSource` calls `await compileAndRunSherlock(...)` (`src/app.js:366`), which issues a `POST /api/trace` network request that spawns the native `sherlock-runner` child process on the server (`server.js:39`). Dragging a slider fires dozens of concurrent HTTP requests and child processes, and because `executeCurrentSource` has no sequence counter, out-of-order async responses can overwrite `state.trace` with stale results.
- **Fix**: Add a monotonic sequence counter `state.compileSeq` in `executeCurrentSource()` to discard stale async completions, and schedule slider/editor live recompilation with a short 60ms debounce while keeping the numeric readout labels (`#fuelValLabel`, `#stackValLabel`, line numbers) instantaneous.

```javascript
// Replace executeCurrentSource (lines 364–392) and input listeners (lines 800–820) in /home/charles/sherlock-bench/src/app.js:
let compileDebounceTimer = 0;
let compileSeq = 0;

function scheduleCompile(resetPlayhead = false, delayMs = 60) {
  if (compileDebounceTimer) clearTimeout(compileDebounceTimer);
  compileDebounceTimer = window.setTimeout(() => {
    compileDebounceTimer = 0;
    executeCurrentSource(resetPlayhead);
  }, delayMs);
}

async function executeCurrentSource(resetPlayhead = true) {
  if (compileDebounceTimer) {
    clearTimeout(compileDebounceTimer);
    compileDebounceTimer = 0;
  }
  const reqId = ++compileSeq;
  state.source = els.sourceEditor.value;
  const res = await compileAndRunSherlock(state.source, {
    fuel: state.fuel,
    stackLimit: state.stackLimit,
    sliceWidth: 4,
    preferRust: true,
  });

  if (reqId !== compileSeq) return; // Discard superseded async trace response

  if (!res.ok) {
    els.compileErrorBanner.textContent = res.error || 'Compilation error';
    els.compileErrorBanner.classList.remove('hidden');
    return;
  }

  els.compileErrorBanner.classList.add('hidden');
  state.trace = res;
  state.stepStateCache = null;

  if (els.activeAstBadge) {
    const activeMeta = PRESET_EDITORIAL_META[state.activePresetId];
    const title = activeMeta?.title || state.trace.programName || 'Sherlock Program';
    const entryExpr = state.trace.programImage?.expressions?.find((e) => e.astPath === 'entry/');
    const entryMatch = String(state.source || '').match(/\bentry\s+([\s\S]*?)\s*;/);
    const liveEntry = (entryExpr?.text || (entryMatch ? entryMatch[1] : '') || activeMeta?.sub || '')
      .replace(/\s+/g, ' ')
      .trim();
    const shortEntry = liveEntry.length > 36 ? `${liveEntry.slice(0, 35)}…` : liveEntry;
    els.activeAstBadge.textContent = shortEntry ? `${title} · ${shortEntry}` : title;
  }

  const maxStep = Math.max(0, res.steps.length - 1);
  els.stepScrubber.max = String(maxStep);
  els.stepScrubber.step = '1';
  if (resetPlayhead) {
    state.playhead = 0;
  } else {
    state.playhead = Math.min(state.playhead, maxStep);
  }
  state.lastRenderedStepIdx = -1;
  syncStepUI(Math.floor(state.playhead));
}
```

---

## Priority 3: CSS Layout Containment, Paint Isolation & Content Visibility (`src/styles.css`)

### 3.1 Isolate Dynamic Inspector & Stage Containers with `contain` to Stop Full-Page Reflows on Step Ticks
- **File & Lines**: `/home/charles/sherlock-bench/src/styles.css:461–469, 616–637, 815–863, 1136–1147, 1274–1293, 1524–1530, 1633–1640`
- **Bottleneck**:
  1. Every step tick mutates text inside `.stage-caption` (`#hudControlSummary`, `#recursionEquationText`), `.telemetry-dl > div`, `.six-rings-bar`, `.frames-list`, `.recursion-calls-track`, and `.line-numbers`.
  2. None of these containers have CSS `contain` rules (`contain: layout style paint` or `contain: content`). Consequently, whenever `#frameStackList` or `#recursionCallsTrack` adds/removes items or `#telemetryControl` wraps a long expression, the browser invalidates layout up through `.workbench-grid` and `document.body` (including the huge below-the-fold documentation deck).
  3. `.stage-viewport-card` (`src/styles.css:815–824`) has explicit dimensions (`height: min(68vh, 640px); min-height: 464px; overflow: hidden;`) and hosts the 60fps WebGPU `<canvas>`, making it an ideal candidate for `contain: strict;` so DOM updates outside the viewport card never trigger layout/paint checks inside the canvas container and vice versa.
  4. `.ring-pill` (`src/styles.css:1292`) has `transition: border-color 0.16s ease, background-color 0.16s ease, box-shadow 0.16s ease;`. Animating `box-shadow` across step transitions forces main-thread software shadow repaints on every frame of the 160ms transition.
- **Fix**: Add targeted CSS containment, compositor layer hints, and remove `box-shadow` from `.ring-pill`'s transition list.

```css
/* Drop-in additions / updates for /home/charles/sherlock-bench/src/styles.css */

/* 1. Strict containment & GPU layer isolation for the 3D WebGPU stage card (lines 815–836) */
.stage-viewport-card {
  contain: strict;
}

#sherlockCanvas {
  will-change: contents;
  transform: translateZ(0);
}

/* 2. Layout & style containment on the 3 main workbench columns (lines 461, 809) */
.panel,
.stage-column {
  contain: layout style;
}

/* 3. Paint/layout containment on high-frequency step-updated sub-containers (lines 616, 624, 843, 1142, 1274, 1464, 1524, 1633) */
.code-editor-wrap,
.line-numbers,
.stage-caption,
.telemetry-dl > div,
.six-rings-bar,
.control-sector-strip,
.frames-list,
.recursion-calls-track {
  contain: layout style paint;
}

/* 4. Avoid main-thread box-shadow repaint animation on every step in .ring-pill (line 1292) */
.ring-pill {
  contain: layout style paint;
  transition: border-color 0.16s ease, background-color 0.16s ease;
}

/* 5. Fix missing --error-bg fallback in .compile-error (line 692) */
.compile-error {
  background: var(--error-bg, rgba(196, 48, 43, 0.08));
}
```

---

### 3.2 Defer Below-the-Fold Documentation Rendering with `content-visibility: auto`
- **File & Lines**: `/home/charles/sherlock-bench/src/styles.css:1703–1709, 1732–1737, 1826–1833, 1962–1968`
- **Bottleneck**: The below-the-fold `.prose-deck` (`index.html:253–561`) contains 4 editorial cards, the 39-rule decoder floor guide (6 category cards + 39 rule chips), and 3 dense architectural comparison tables. This static section accounts for **>60% of the total DOM nodes on the page** and participates in initial style/layout/paint calculation as well as root layout invalidations.
- **Fix**: Apply `content-visibility: auto; contain-intrinsic-size: auto 1400px;` to `.prose-deck` and `contain: layout style paint;` to individual below-the-fold cards/tables.

```css
/* Drop-in update at /home/charles/sherlock-bench/src/styles.css:1703 */
.prose-deck {
  padding-top: 24px;
  border-top: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  flex-direction: column;
  gap: 20px;
  content-visibility: auto;
  contain-intrinsic-size: auto 1400px;
}

.primer-card,
.prose-card,
.floor-guide-card,
.cek-table-wrap {
  contain: layout style paint;
}
```

---

## Priority 4: Flatten the 4-Level ES Module Waterfall with `<link rel="modulepreload">` (`index.html`)

- **File & Lines**: `/home/charles/sherlock-bench/index.html:12–21`
- **Bottleneck**: Currently, `index.html` loads `<script type="module" src="/src/app.js"></script>` at line 564 with no `<link rel="modulepreload">` hints. This creates a **4-round-trip sequential discovery waterfall** for 3.95 MB of critical JS modules:
  1. **Depth 1**: `/src/app.js` (35 KB)
  2. **Depth 2**: `/src/engine/sherlockEngine.js` (72 KB), `/src/stage/SherlockStage.js` (13 KB), `/src/designers/designer8_quineling.js` (71 KB)
  3. **Depth 3**: `/vendor/three.webgpu.js` (**2.28 MB**), `/vendor/three.tsl.js` (37 KB), `/src/engine/sherlockQuinelingCompiler.js` (22 KB), `/src/designers/theme.js` + `designer1..7.js`
  4. **Depth 4**: `/vendor/three.core.js` (**1.40 MB**, imported inside `three.webgpu.js:6`) and `/src/engine/quinelingsSdk.js` (**153 KB**, imported inside `sherlockQuinelingCompiler.js:16`)
  Because `/vendor/three.core.js` (1.40 MB) sits at Depth 4, the browser cannot even begin fetching or parsing it until `/src/app.js`, `/src/stage/SherlockStage.js`, and the entire 2.28 MB `/vendor/three.webgpu.js` bundle have sequentially downloaded and been parsed!
- **Fix**: Add `<link rel="modulepreload">` hints immediately after the `<script type="importmap">` block in `/home/charles/sherlock-bench/index.html:20` so the browser fetches and compiles the Three.js WebGPU/Core/TSL bundles and core engine modules in parallel from the start of `<head>`.

```html
<!-- Insert right after </script> (line 20) in /home/charles/sherlock-bench/index.html -->
  <link rel="modulepreload" href="/src/app.js" />
  <link rel="modulepreload" href="/vendor/three.webgpu.js" />
  <link rel="modulepreload" href="/vendor/three.core.js" />
  <link rel="modulepreload" href="/vendor/three.tsl.js" />
  <link rel="modulepreload" href="/src/engine/sherlockEngine.js" />
  <link rel="modulepreload" href="/src/stage/SherlockStage.js" />
  <link rel="modulepreload" href="/src/designers/designer8_quineling.js" />
  <link rel="modulepreload" href="/src/engine/sherlockQuinelingCompiler.js" />
  <link rel="modulepreload" href="/src/engine/quinelingsSdk.js" />
```
